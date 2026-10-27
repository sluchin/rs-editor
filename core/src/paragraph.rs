//! 段落と文の移動 (Emacs 30 の paragraphs.el. `M-{`, `M-}`, `M-a`, `M-e`, `M-k`).
//!
//! 段落を分ける行は, モードの `paragraph-start` と `paragraph-separate` で決めます. Fundamental モードと
//! emacs-lisp-mode では, 空白だけの行で, scheme-mode では, 空の行だけです (空白だけの行は, 段落を分けない).

use crate::sexp::MotionError;
use crate::syntax::Dialect;

/// 本文の見え方: `limit` から先は無いものとする (Emacs の検索の範囲. 範囲の端が行の終わりでなければ, 行の終わりとしない).
struct View<'a> {
    /// 本文.
    text: &'a [char],
    /// 見える範囲の終わり.
    limit: usize,
}

impl View<'_> {
    /// 位置の文字. 範囲の終わりが行の途中なら, どれにも合わない文字 (`\0`).
    fn at(&self, p: usize) -> Option<char> {
        if p < self.limit {
            Some(self.text[p])
        } else if p == self.limit && self.limit < self.text.len() && self.text[p] != '\n' {
            Some('\0')
        } else {
            None
        }
    }

    /// 行の終わり (正規表現の `$`).
    fn eol(&self, p: usize) -> bool {
        matches!(self.at(p), None | Some('\n'))
    }

    /// 行の始め (正規表現の `^`).
    fn bol(&self, p: usize) -> bool {
        p == 0 || self.text[p - 1] == '\n'
    }

    /// 位置から, `set` の文字を飛ばした位置.
    fn skip(&self, mut p: usize, set: &str) -> usize {
        while self.at(p).is_some_and(|c| set.contains(c)) {
            p += 1;
        }
        p
    }
}

/// 段落の始めか, 段落を分ける行か (Emacs の `paragraph-start`. 位置で合えば `true`).
fn is_start(v: &View, p: usize, dialect: Dialect) -> bool {
    if dialect == Dialect::Scheme {
        v.eol(p) || (v.bol(p) && v.at(p) == Some('\x0c'))
    } else {
        v.at(p) == Some('\x0c') || v.eol(v.skip(p, " \t"))
    }
}

/// 段落を分ける行か (Emacs の `paragraph-separate`).
fn is_separate(v: &View, p: usize, dialect: Dialect) -> bool {
    if dialect == Dialect::Scheme {
        is_start(v, p, dialect)
    } else {
        v.eol(v.skip(p, " \t\x0c"))
    }
}

/// 位置で, `^[ \t]*(?:paragraph-start|paragraph-separate)` が合えば, その終わり (Emacs の `forward-paragraph` の探す正規表現).
fn search_match(v: &View, p: usize, dialect: Dialect) -> Option<usize> {
    if !v.bol(p) {
        return None;
    }
    let k = v.skip(p, " \t");
    if dialect == Dialect::Scheme {
        if v.eol(k) {
            return Some(k);
        }
        return (k == p && v.at(k) == Some('\x0c')).then_some(k + 1);
    }
    if v.at(k) == Some('\x0c') {
        return Some(k + 1);
    }
    let k2 = v.skip(k, " \t\x0c");
    v.eol(k2).then_some(k2)
}

/// 文の終わりの後ろに付く文字 (`[  \t\n]*`) を飛ばします.
fn skip_after_sentence(v: &View, p: usize) -> usize {
    v.skip(p, " \u{a0}\t\n")
}

/// 位置で, 文の終わり (Emacs の `(sentence-end)`. `sentence-end-double-space` が `t`) が合えば, その終わり.
fn sentence_end_match(v: &View, p: usize) -> Option<usize> {
    let c = v.at(p)?;
    if ".?!…‽".contains(c) {
        let j = v.skip(p + 1, "]\"'”’)}»›");
        let tail = if v.eol(j) {
            Some(j)
        } else if (matches!(v.at(j), Some(' ' | '\u{a0}')) && v.eol(j + 1)) || v.at(j) == Some('\t')
        {
            Some(j + 1)
        } else if matches!(v.at(j), Some(' ' | '\u{a0}'))
            && matches!(v.at(j + 1), Some(' ' | '\u{a0}'))
        {
            Some(j + 2)
        } else {
            None
        };
        return tail.map(|t| skip_after_sentence(v, t));
    }
    if "。．？！".contains(c) {
        return Some(skip_after_sentence(v, v.skip(p, "。．？！")));
    }
    None
}

/// 位置から前へ, 合う所を探します (Emacs の `re-search-backward`. 合う文字は, 位置の前で終わる).
///
/// # Returns
///
/// 合った所の始めと終わり.
fn search_backward(
    text: &[char],
    pos: usize,
    bound: usize,
    matcher: &dyn Fn(&View, usize) -> Option<usize>,
) -> Option<(usize, usize)> {
    let v = View { text, limit: pos };
    (bound..=pos).rev().find_map(|start| {
        matcher(&v, start)
            .filter(|&end| end <= pos)
            .map(|end| (start, end))
    })
}

/// 行の始め.
fn bol(text: &[char], p: usize) -> usize {
    let mut p = p.min(text.len());
    while p > 0 && text[p - 1] != '\n' {
        p -= 1;
    }
    p
}

/// 次の行の始め (最後の行なら, 本文の終わり).
fn next_line(text: &[char], p: usize) -> usize {
    (p..text.len())
        .find(|&i| text[i] == '\n')
        .map_or(text.len(), |i| i + 1)
}

/// 段落を越えます (Emacs の `forward-paragraph`. `fill-prefix` は無い).
///
/// 後ろへは, 段落の後ろの, 段落を分ける行の始めへ, 前へは, 段落の前の, 段落を分ける行の始めへ動きます.
///
/// # Arguments
///
/// * `text` - 本文.
/// * `pos` - 位置.
/// * `count` - 越える数 (負なら, 前へ).
/// * `dialect` - Lisp の方言.
///
/// # Returns
///
/// 越えた後の位置.
pub fn forward_paragraph(text: &[char], pos: usize, count: i64, dialect: Dialect) -> usize {
    let max = text.len();
    let whole = View { text, limit: max };
    let sep = |p: usize| is_separate(&whole, p, dialect);
    let start = |p: usize| is_start(&whole, p, dialect);
    let mut arg = count;
    let mut p = pos;
    while arg < 0 && p > 0 {
        // 前が空の行なら, そこへ
        if !sep(p) && text[p - 1] == '\n' && bol(text, p - 1) == p - 1 && sep(p - 1) {
            p -= 1;
            arg += 1;
            continue;
        }
        // 前の, 段落を分ける行を越える
        p = bol(text, p - 1);
        while p > 0 && sep(p) {
            p = bol(text, p - 1);
        }
        if p == 0 {
            break;
        }
        arg += 1;
        p = (p..max).find(|&i| text[i] == '\n').unwrap_or(max);
        // 前の, 段落の始めか, 段落を分ける行を探す
        let mut found = false;
        loop {
            let Some((at, _)) = search_backward(text, p, 0, &|v, i| search_match(v, i, dialect))
            else {
                p = 0;
                break;
            };
            p = at;
            if sep(p) || start(p) {
                found = true;
                break;
            }
        }
        if !found {
            p = 0;
            continue;
        }
        // 段落を分ける行を越える
        while p < max && sep(p) {
            p = next_line(text, p);
        }
        // 前の行が空なら, その行へ. 空でなければ, 段落の始めへ
        let line_start = bol(text, p);
        let previous_end = line_start.saturating_sub(1);
        p = if previous_end > bol(text, previous_end) {
            previous_end + 1
        } else {
            previous_end
        };
    }
    while arg > 0 && p < max {
        // 段落を分ける行を越える
        p = bol(text, p);
        while p < max && sep(p) {
            p = next_line(text, p);
        }
        if p < max {
            arg -= 1;
        }
        p = next_line(text, p);
        // 次の, 段落の始めか, 段落を分ける行を探す
        let mut found = p;
        loop {
            let Some(i) = (p..=max).find(|&i| search_match(&whole, i, dialect).is_some()) else {
                p = max;
                break;
            };
            found = i;
            p = i;
            if p >= max || sep(p) || start(p) {
                break;
            }
            p += 1;
        }
        if p < max {
            p = found;
        }
    }
    p
}

/// 段落の本文の始めへ動きます (Emacs の `start-of-paragraph-text`).
///
/// # Returns
///
/// 段落の, 始めの空白でない文字.
fn start_of_paragraph_text(text: &[char], pos: usize, dialect: Dialect) -> usize {
    let start = forward_paragraph(text, pos, -1, dialect);
    let mut p = start;
    while p < text.len() && " \t\n".contains(text[p]) {
        p += 1;
    }
    // 越えた空の行が, 始めの位置を含むなら, その前から
    if p >= pos && start < pos {
        return if start > 0 {
            start_of_paragraph_text(text, start, dialect)
        } else {
            start
        };
    }
    p
}

/// 段落の本文の終わりへ動きます (Emacs の `end-of-paragraph-text`).
///
/// # Errors
///
/// 本文の終わりから動こうとしたとき (`End of buffer`).
fn end_of_paragraph_text(text: &[char], pos: usize, dialect: Dialect) -> Result<usize, String> {
    let mut p = forward_paragraph(text, pos, 1, dialect);
    if p > 0 && text[p - 1] == '\n' {
        p -= 1;
    }
    if p > pos {
        return Ok(p);
    }
    if p >= text.len() {
        return Err("End of buffer".to_string());
    }
    p += 1;
    if p < text.len() {
        end_of_paragraph_text(text, p, dialect)
    } else {
        Ok(p)
    }
}

/// 文を越えます (Emacs の `forward-sentence`). 段落の終わりも, 文の終わりです.
///
/// 後ろへは, 文の終わり (後ろの空白の前) へ, 前へは, 文の始めへ動きます.
///
/// # Arguments
///
/// * `text` - 本文.
/// * `pos` - 位置.
/// * `count` - 越える数 (負なら, 前へ).
/// * `dialect` - Lisp の方言.
///
/// # Returns
///
/// 越えた後の位置.
///
/// # Errors
///
/// 本文の終わりから後ろへ動こうとしたとき (`End of buffer`).
pub fn forward_sentence(
    text: &[char],
    pos: usize,
    count: i64,
    dialect: Dialect,
) -> Result<usize, MotionError> {
    let mut p = pos;
    for _ in count..0 {
        let text_start = start_of_paragraph_text(text, p, dialect);
        let line_start = bol(text, text_start);
        // 探す範囲の始めが, 位置の後ろ (Emacs の `Invalid search bound`)
        if line_start > p {
            return Err(MotionError {
                point: p,
                message: "Invalid search bound (wrong side of point)".to_string(),
            });
        }
        let mut m = search_backward(text, p, line_start, &sentence_end_match);
        // 前の文の終わりが, 位置で終われば (位置が, 文の始め), もう 1 つ前
        if let Some((start, end)) = m {
            if end >= p {
                m = search_backward(text, start, line_start, &sentence_end_match);
            }
        }
        p = m.map_or(text_start, |(_, end)| end);
    }
    for _ in 0..count {
        let end = end_of_paragraph_text(text, p, dialect)
            .map_err(|message| MotionError { point: p, message })?;
        let v = View { text, limit: end };
        match (p..end).find_map(|i| sentence_end_match(&v, i)) {
            Some(found) => {
                p = found;
                while p > 0 && " \t\n".contains(text[p - 1]) {
                    p -= 1;
                }
            }
            None => p = end,
        }
    }
    Ok(p)
}
