//! 段落を詰める (Emacs 30 の `fill-paragraph`. `M-q`). 行を `fill-column` までに詰め直します.
//!
//! - Lisp では, カーソルの行のコメントの段落を詰めます (コードと文字列は, 変えません).
//!   続きの行には, コメントの `;` の並びを付けます.
//! - Lisp でなければ, 空の行で区切った段落を詰めます (Emacs の text-mode).
//!   続きの行の始め (fill-prefix) は, 2 行目の始めの空白や `>` などから取ります (`adaptive-fill-mode`).
//! - 語の間の空白は 1 つにし, 文の終わり (`.` `?` `!`) の後に 2 つあった空白は, 2 つのままにします
//!   (`sentence-end-double-space`). 行をつないだ所が, 文の終わりなら, 空白 2 つにします.
//! - 日本語 (漢字, かな) は, 空白の無い所でも折り, 行をつなぐときは空白を入れません.
//!   行の始めに置けない文字 (`。` `」` など) と, 行の終わりに置けない文字 (`「` など) は, 禁則で避けます.

use crate::indent::{column_at, whitespace, Config, Edit, LispParser, Plan};
use crate::scheme::builtins::char_width;

/// 禁則で, 行を越えてよい桁の数 (Emacs の `kinsoku-limit`).
const KINSOKU_LIMIT: usize = 4;

/// 行の始めに置けない文字.
const NO_LINE_START: &str =
    "、。，．・：；？！゛゜ヽヾゝゞ々ー’”）〕］｝〉》」』】ぁぃぅぇぉっゃゅょゎァィゥェォッャュョヮヵヶ";

/// 行の終わりに置けない文字.
const NO_LINE_END: &str = "‘“（〔［｛〈《「『【";

/// 行の始めの, fill-prefix になる文字 (Emacs の `adaptive-fill-regexp`).
const ADAPTIVE_PREFIX: &str = "-–!|#%;>*·•‣⁃◦ \t";

/// 文の終わりの後ろに付く, 閉じ括弧と引用符.
const CLOSERS: &str = "])}\"'”’";

/// 日本語と中国語の文字か (空白の無い所でも折り, 行をつなぐときは空白を入れない).
fn is_cjk(c: char) -> bool {
    matches!(c as u32,
        0x2E80..=0x2FDF | 0x3000..=0x303F | 0x3041..=0x30FF | 0x31F0..=0x31FF
        | 0x3400..=0x4DBF | 0x4E00..=0x9FFF | 0xF900..=0xFAFF | 0xFF00..=0xFFEF
        | 0x20000..=0x2FA1F)
}

/// 空白 (スペースとタブ) だけの行か.
fn is_blank(text: &[char]) -> bool {
    text.iter().all(|c| matches!(c, ' ' | '\t'))
}

/// 詰める段落.
struct Paragraph {
    /// 始めの行 (1 から).
    first: usize,
    /// 終わりの行.
    last: usize,
    /// 始めの行の, 本文の前 (そのまま残す).
    head: Vec<char>,
    /// 続きの行の始めに付けるもの.
    prefix: Vec<char>,
    /// 行ごとの, 本文の始めの位置 (行の始めから).
    starts: Vec<usize>,
}

/// 1 行だけの段落の, 続きの行の始めを返します. 空白だけならそのまま, ほかは同じ幅の空白にします.
fn prefix_from_head(head: &[char], tab_width: usize) -> Vec<char> {
    if is_blank(head) {
        head.to_vec()
    } else {
        vec![' '; column_at(head, head.len(), tab_width)]
    }
}

/// 行の始めの, fill-prefix になる文字の長さ.
fn adaptive_len(text: &[char]) -> usize {
    text.iter()
        .take_while(|c| ADAPTIVE_PREFIX.contains(**c))
        .count()
}

/// Lisp の行のコメント.
#[derive(Clone)]
struct Comment {
    /// 本文の始め (行の始めから).
    body: usize,
    /// `;` の桁.
    col: usize,
    /// `;` の並び.
    semicolons: Vec<char>,
    /// `;` の後ろの空白.
    spaces: Vec<char>,
    /// コメントの前にコードがあるか.
    code: bool,
}

/// 行ごとの, Lisp のコメント (文字列の中の `;` を除く). 本文が無ければ `None`.
fn comments(lines: &[Vec<char>], config: &Config) -> Vec<Option<Comment>> {
    let mut parser = LispParser::new(
        config.dialect == crate::syntax::Dialect::Scheme,
        config.tab_width,
    );
    lines
        .iter()
        .enumerate()
        .map(|(n, text)| {
            parser.scan_line(text, n, 0, text.len());
            let start = parser.comment?;
            let rest = &text[start..];
            let semis = rest.iter().take_while(|c| **c == ';').count();
            let spaces = rest[semis..]
                .iter()
                .take_while(|c| matches!(c, ' ' | '\t'))
                .count();
            let body = start + semis + spaces;
            if body == text.len() {
                return None;
            }
            Some(Comment {
                body,
                col: column_at(text, start, config.tab_width),
                semicolons: rest[..semis].to_vec(),
                spaces: rest[semis..semis + spaces].to_vec(),
                code: !is_blank(&text[..start]),
            })
        })
        .collect()
}

/// Lisp のコメントの段落を探します.
fn comment_paragraph(lines: &[Vec<char>], line: usize, config: &Config) -> Option<Paragraph> {
    let all = comments(lines, config);
    let at = |n: usize| -> Option<&Comment> {
        if n < 1 {
            None
        } else {
            all.get(n - 1).and_then(Option::as_ref)
        }
    };
    let current = at(line)?;
    // 同じ桁で, 同じ数の `;` のコメントなら, 同じ段落 (コードの後ろのコメントは, 段落の始めの行だけ)
    let same = |c: Option<&Comment>, code: bool| {
        c.is_some_and(|c| {
            c.code == code && c.col == current.col && c.semicolons == current.semicolons
        })
    };
    let mut first = line;
    if !current.code {
        while same(at(first - 1), false) {
            first -= 1;
        }
        if same(at(first - 1), true) {
            first -= 1;
        }
    }
    let mut last = line;
    while same(at(last + 1), false) {
        last += 1;
    }
    let starts: Vec<usize> = (first..=last)
        .map(|n| at(n).map_or(0, |c| c.body))
        .collect();
    let head = lines[first - 1][..starts[0]].to_vec();
    let prefix = if last > first {
        lines[first][..starts[1]].to_vec()
    } else {
        let mut p: Vec<char> = whitespace(0, current.col, config).chars().collect();
        p.extend(&current.semicolons);
        p.extend(&current.spaces);
        p
    };
    Some(Paragraph {
        first,
        last,
        head,
        prefix,
        starts,
    })
}

/// 空の行で区切った段落を探します (Lisp でないとき). 空の行なら, 次の段落. `fill_prefix` があれば, 続きの行の始めにする.
fn text_paragraph(
    lines: &[Vec<char>],
    line: usize,
    config: &Config,
    fill_prefix: Option<&str>,
) -> Option<Paragraph> {
    let blank = |n: usize| is_blank(&lines[n - 1]);
    let mut first = line;
    while first <= lines.len() && blank(first) {
        first += 1;
    }
    if first > lines.len() {
        return None;
    }
    while first > 1 && !blank(first - 1) {
        first -= 1;
    }
    let mut last = first;
    while last < lines.len() && !blank(last + 1) {
        last += 1;
    }
    let first_text = &lines[first - 1];
    let head = first_text[..adaptive_len(first_text)].to_vec();
    let prefix: Vec<char> = if let Some(p) = fill_prefix {
        p.chars().collect()
    } else if last > first {
        let next = &lines[first];
        next[..adaptive_len(next)].to_vec()
    } else {
        prefix_from_head(&head, config.tab_width)
    };
    let mut starts = vec![head.len()];
    for text in &lines[first..last] {
        // fill-prefix があれば除き, 残りの始めの空白も除く
        let rest = if text.starts_with(&prefix) {
            prefix.len()
        } else {
            0
        };
        starts.push(
            rest + text[rest..]
                .iter()
                .take_while(|c| matches!(c, ' ' | '\t'))
                .count(),
        );
    }
    Some(Paragraph {
        first,
        last,
        head,
        prefix,
        starts,
    })
}

/// 段落の本文の 1 文字.
struct Char {
    /// 文字.
    ch: char,
    /// 元の位置 (文書の始めから).
    pos: usize,
}

/// 本文の, 語の間の空白.
struct Gap {
    /// 空白の後の文字の番号 (本文の中).
    index: usize,
    /// 空白. 空なら, 空白の無い所で折れる.
    text: &'static str,
}

/// `text` が文の終わり (`.` `?` `!` と, 後ろの閉じ括弧と引用符) で終わるか.
fn sentence_end(text: &[char]) -> bool {
    let n = text
        .iter()
        .rev()
        .take_while(|c| CLOSERS.contains(**c))
        .count();
    text.len() > n && matches!(text[text.len() - n - 1], '.' | '?' | '!')
}

/// 段落の本文を, 文字と, 語の間の空白に分けます. 空白は, 1 つ (文の終わりの後は 2 つ) にします.
fn split_words(lines: &[Vec<char>], paragraph: &Paragraph) -> (Vec<Char>, Vec<Gap>) {
    let mut offset = 0;
    let mut line_from = Vec::with_capacity(lines.len());
    for l in lines {
        line_from.push(offset);
        offset += l.len() + 1;
    }
    let mut chars: Vec<Char> = Vec::new();
    // 次の文字の前の空白 (`None` は, 空白でない所)
    let mut spaces: Vec<Option<String>> = Vec::new();
    let mut pending: Option<String> = None;
    for n in paragraph.first..=paragraph.last {
        let line = &lines[n - 1];
        let begin = paragraph.starts[n - paragraph.first];
        let mut end = line.len();
        while end > begin && matches!(line[end - 1], ' ' | '\t') {
            end -= 1;
        }
        if n > paragraph.first {
            pending = Some("\n".to_string());
        }
        for (i, &ch) in line[begin..end].iter().enumerate() {
            if ch == ' ' || ch == '\t' {
                pending.get_or_insert_with(String::new).push(ch);
                continue;
            }
            spaces.push(pending.take());
            chars.push(Char {
                ch,
                pos: line_from[n - 1] + begin + i,
            });
        }
    }
    let mut gaps = Vec::new();
    for index in 1..chars.len() {
        let before = chars[index - 1].ch;
        let after = chars[index].ch;
        let cjk = is_cjk(before) || is_cjk(after);
        let Some(space) = &spaces[index] else {
            // 日本語の前後は, 空白が無くても折れる (禁則の文字の前後を除く)
            if cjk && !NO_LINE_END.contains(before) && !NO_LINE_START.contains(after) {
                gaps.push(Gap { index, text: "" });
            }
            continue;
        };
        let near: Vec<char> = chars[index.saturating_sub(4)..index]
            .iter()
            .map(|c| c.ch)
            .collect();
        let text = if space == "\n" && cjk {
            ""
        } else if sentence_end(&near) && (space.contains('\n') || space.len() >= 2) {
            "  "
        } else {
            " "
        };
        gaps.push(Gap { index, text });
    }
    (chars, gaps)
}

/// 語の中の 1 つの部分.
enum Part {
    /// 本文の文字 (本文の中の番号).
    Char(usize),
    /// 語の中の空白.
    Space(&'static str),
}

/// 行の終わりで折れない所を含む, 語.
struct Word {
    /// 文字と, 語の中の空白.
    parts: Vec<Part>,
    /// 幅.
    width: usize,
    /// 後ろの, 行の始めに置けない文字の幅 (禁則で, 行を越えて置ける).
    tail: usize,
}

impl Word {
    /// 空の語.
    fn new() -> Self {
        Word {
            parts: Vec::new(),
            width: 0,
            tail: 0,
        }
    }
}

/// 本文を, 折れる所で, 語に分けます. 文の終わりでない `.` の後の空白 1 つでは, 折りません (`Mr. Smith`).
/// 語の前の空白 (始めの語は空) も返します.
fn to_words(chars: &[Char], gaps: &[Gap]) -> (Vec<Word>, Vec<&'static str>) {
    let mut words = Vec::new();
    let mut spaces = vec![""];
    let mut word = Word::new();
    let mut g = 0;
    for (index, c) in chars.iter().enumerate() {
        if g < gaps.len() && gaps[g].index == index {
            let text = gaps[g].text;
            g += 1;
            if text == " " && chars[index - 1].ch == '.' {
                word.parts.push(Part::Space(text));
                word.width += 1;
                word.tail = 0;
            } else {
                words.push(std::mem::replace(&mut word, Word::new()));
                spaces.push(text);
            }
        }
        let width = char_width(c.ch);
        word.parts.push(Part::Char(index));
        word.width += width;
        word.tail = if NO_LINE_START.contains(c.ch) {
            word.tail + width
        } else {
            0
        };
    }
    words.push(word);
    (words, spaces)
}

/// 段落を詰めて, 行に分けます. 新しい本文と, 本文の文字ごとの新しい位置 (段落の始めから) を返します.
fn break_lines(
    chars: &[Char],
    gaps: &[Gap],
    paragraph: &Paragraph,
    config: &Config,
    fill_column: usize,
) -> (String, Vec<usize>) {
    let (words, spaces) = to_words(chars, gaps);
    let tab = config.tab_width;
    let mut out: String = paragraph.head.iter().collect();
    let mut out_len = paragraph.head.len();
    let mut positions = Vec::with_capacity(chars.len());
    let mut col = column_at(&paragraph.head, paragraph.head.len(), tab);
    let prefix_text: String = paragraph.prefix.iter().collect();
    for (i, word) in words.iter().enumerate() {
        let space = spaces[i];
        let end = col + space.len() + word.width;
        // 行の始めに置けない文字は, 少しなら, 行を越えて置く (禁則)
        let fits = end <= fill_column
            || (word.tail > 0 && word.tail <= KINSOKU_LIMIT && end - word.tail <= fill_column);
        if i > 0 && !fits {
            out.push('\n');
            out.push_str(&prefix_text);
            out_len += 1 + paragraph.prefix.len();
            col = column_at(&paragraph.prefix, paragraph.prefix.len(), tab);
        } else {
            out.push_str(space);
            out_len += space.len();
            col += space.len();
        }
        for part in &word.parts {
            match part {
                Part::Char(k) => {
                    positions.push(out_len);
                    out.push(chars[*k].ch);
                    out_len += 1;
                    col += char_width(chars[*k].ch);
                }
                Part::Space(text) => {
                    out.push_str(text);
                    out_len += text.len();
                    col += text.len();
                }
            }
        }
    }
    (out, positions)
}

/// カーソルの段落を詰めます (Emacs の `fill-paragraph`). `pos` はカーソルの位置 (0 から). 詰める段落が無ければ `None`.
pub fn fill_paragraph(
    lines: &[Vec<char>],
    pos: usize,
    config: &Config,
    fill_column: usize,
    fill_prefix: Option<&str>,
) -> Option<Plan> {
    let mut starts_of_lines = Vec::with_capacity(lines.len());
    let mut offset = 0;
    for l in lines {
        starts_of_lines.push(offset);
        offset += l.len() + 1;
    }
    let line = starts_of_lines.partition_point(|&s| s <= pos);
    let paragraph = if config.dialect == crate::syntax::Dialect::Fundamental {
        text_paragraph(lines, line, config, fill_prefix)?
    } else {
        comment_paragraph(lines, line, config)?
    };
    let (chars, gaps) = split_words(lines, &paragraph);
    let (text, positions) = break_lines(&chars, &gaps, &paragraph, config, fill_column);
    let from = starts_of_lines[paragraph.first - 1];
    let to = starts_of_lines[paragraph.last - 1] + lines[paragraph.last - 1].len();
    // カーソルは, 同じ文字の前に置く (段落の始めの行の, 本文の前なら, そのまま)
    let next = chars.iter().position(|c| c.pos >= pos);
    // 前の文字と同じ行の, 語の間の空白の中なら, 前の文字からの距離を保つ (空白が変わるなら, 前の文字の後ろに置く)
    let line_of = |p: usize| starts_of_lines.partition_point(|&s| s <= p);
    let in_gap = next.filter(|&k| k > 0).filter(|&k| {
        chars[k].pos > pos && chars[k - 1].pos < pos && line_of(chars[k - 1].pos) == line_of(pos)
    });
    let cursor = if pos < from + paragraph.head.len() {
        pos
    } else if let Some(k) = in_gap {
        let (before, after) = (&chars[k - 1], &chars[k]);
        let unchanged = positions[k] - positions[k - 1] == after.pos - before.pos
            && line_of(before.pos) == line_of(after.pos);
        let offset = if unchanged { pos - before.pos } else { 1 };
        from + positions[k - 1] + offset
    } else {
        from + next.map_or(text.chars().count(), |k| positions[k])
    };
    let original: String = lines[paragraph.first - 1..paragraph.last]
        .iter()
        .map(|l| l.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");
    let edits = if original == text {
        Vec::new()
    } else {
        vec![Edit {
            from,
            to,
            insert: text,
        }]
    };
    // 変わらなければ, カーソルも動かさない
    let cursor = if edits.is_empty() { pos } else { cursor };
    Some(Plan {
        edits,
        cursor: Some(cursor),
    })
}
