//! 字下げ (Emacs 30 の `lisp-indent-line` と, Fundamental モードの `indent-relative`).
//!
//! Lisp の字下げは, Emacs の `calculate-lisp-indent`, `lisp-indent-function` (`scheme-indent-function`),
//! `lisp-indent-specform`, `lisp-indent-defform` を, 開いているリストの要素の位置から計算し直したものです.
//! 桁は, Emacs と同じく, タブを `tab-width` の倍数まで, 全角の文字を 2 桁と数えます.
//!
//! 本文は, 行ごとの文字の並び (`&[Vec<char>]`) で受け, 変更の計画 ([`Plan`]) を返します.

use std::collections::{HashMap, HashSet};

use crate::indent_specs::{lookup, Spec, ELISP, SCHEME};
use crate::scheme::builtins::char_width;
use crate::syntax::Dialect;

/// `;` 1 つのコメントの行の桁 (Emacs の `comment-column`).
pub const COMMENT_COLUMN: usize = 40;

/// 本体の字下げの幅 (Emacs の `lisp-body-indent`).
const BODY_INDENT: usize = 2;

/// 字下げの設定.
#[derive(Debug, Clone, Copy)]
pub struct Config {
    /// Lisp の方言 (`Fundamental` なら, Fundamental モードの字下げ).
    pub dialect: Dialect,
    /// 字下げにタブを使うなら `true` (Emacs の `indent-tabs-mode`).
    pub indent_tabs: bool,
    /// タブの幅 (Emacs の `tab-width`).
    pub tab_width: usize,
}

impl Config {
    /// Lisp の字下げか.
    fn lisp(&self) -> bool {
        self.dialect != Dialect::Fundamental
    }

    /// Scheme か.
    fn scheme(&self) -> bool {
        self.dialect == Dialect::Scheme
    }
}

/// 本文の変更: 範囲 (全体の文字の位置) を, 文字列に替える.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edit {
    /// 始め.
    pub from: usize,
    /// 終わり.
    pub to: usize,
    /// 入れる文字列.
    pub insert: String,
}

/// 変更の計画: 変更 (元の本文の位置. 前から順で, 重ならない) と, 変えた後のカーソル (あれば).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Plan {
    /// 変更.
    pub edits: Vec<Edit>,
    /// 変えた後のカーソル (変えた後の本文の位置). 決まらなければ `None` (変更から写す).
    pub cursor: Option<usize>,
}

/// 行の, 位置の桁を返します.
pub fn column_at(text: &[char], index: usize, tab_width: usize) -> usize {
    let mut col = 0;
    for &c in &text[..index.min(text.len())] {
        col = if c == '\t' {
            (col / tab_width + 1) * tab_width
        } else {
            col + char_width(c)
        };
    }
    col
}

/// 桁から桁までを埋める空白を返します (Emacs の `indent-to`).
pub fn whitespace(from: usize, to: usize, config: &Config) -> String {
    let tab = config.tab_width;
    let tabs = if config.indent_tabs {
        (to / tab).saturating_sub(from / tab)
    } else {
        0
    };
    if tabs == 0 {
        return " ".repeat(to.saturating_sub(from));
    }
    "\t".repeat(tabs) + &" ".repeat(to - (to / tab) * tab)
}

/// 字下げの空白を, 桁まで変えます (Emacs の `indent-line-to`. 今の空白は, できるだけ残す).
fn indent_to(old: &[char], col: usize, config: &Config) -> String {
    let tab = config.tab_width;
    let current = column_at(old, old.len(), tab);
    if current > col {
        // 桁までを残す (タブの途中なら, タブを空白にする)
        let mut i = 0;
        while column_at(old, i + 1, tab) <= col {
            i += 1;
        }
        let kept: String = old[..i].iter().collect();
        return kept + &" ".repeat(col - column_at(old, i, tab));
    }
    // タブを入れられるなら, 後ろの空白を消してから入れる
    let can_tab = col - (current / tab) * tab >= tab;
    let mut base: Vec<char> = old.to_vec();
    if can_tab {
        while base.last() == Some(&' ') {
            base.pop();
        }
    }
    let base_col = column_at(&base, base.len(), tab);
    base.iter().collect::<String>() + &whitespace(base_col, col, config)
}

/// 行の始めの空白の長さ.
pub fn indent_length(text: &[char]) -> usize {
    text.iter().take_while(|c| matches!(c, ' ' | '\t')).count()
}

/// 行が空白 (スペースとタブ) だけか.
fn blank(text: &[char]) -> bool {
    text.iter().all(|c| matches!(c, ' ' | '\t'))
}

/// 文字列が `text` の位置から始まるか.
fn starts_with(text: &[char], i: usize, pattern: &str) -> bool {
    pattern
        .chars()
        .enumerate()
        .all(|(k, c)| text.get(i + k) == Some(&c))
}

/// 名前の終わりの文字 (空白, 括弧, `"`, `;`, 式の前の印).
pub(crate) fn delimiter(c: char) -> bool {
    c.is_whitespace() || "()[]\";'`,".contains(c)
}

/// リストの要素の種類.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    /// リスト.
    List,
    /// シンボル.
    Symbol,
    /// それ以外.
    Other,
}

/// リストの要素.
#[derive(Debug, Clone)]
struct Element {
    /// 桁 (前の `'` などを含む).
    col: usize,
    /// 行 (0 から).
    line: usize,
    /// 種類.
    kind: Kind,
    /// 名前 (シンボルなら).
    text: String,
    /// 最後の式と見ないなら `true` (`indent-sexp` を始めた点を含む文字列).
    hidden: bool,
}

/// 開いているリスト.
#[derive(Debug, Clone)]
struct Frame {
    /// `(` の桁.
    col: usize,
    /// `(` の行.
    line: usize,
    /// これまでに読んだ, リストの要素 (前から順).
    elements: Vec<Element>,
}

/// Lisp の本文を, 行ごとに読みます (Emacs の `parse-partial-sexp`). 開いているリストと, 文字列の中かを覚えます.
#[derive(Clone)]
pub struct LispParser {
    /// Scheme か.
    scheme: bool,
    /// タブの幅.
    tab_width: usize,
    /// 開いているリスト.
    stack: Vec<Frame>,
    /// 文字列の中か.
    in_string: bool,
    /// 開いている文字列の, `"` の桁.
    string_col: usize,
    /// 文字列が閉じる行の, 式の始めを, 行頭の最初のトークンと見るなら `true` (行ごとに新しく読む Emacs の経路).
    first_token: bool,
    /// 文字列が, `#;` でコメントにした式なら `true` (Emacs は, コメントの中と見る).
    comment_string: bool,
    /// `#| |#` のコメントの深さ (Scheme).
    block_comment: usize,
    /// `#;` でコメントにした式の, 括弧の深さ (Scheme. コメントでなければ `None`).
    datum: Option<i64>,
    /// 行ごとの, 始めの式の桁.
    first_sexp_col: Vec<Option<usize>>,
    /// 最後に読んだ行の, コメントの `;` の位置 (行の始めから. コメントが無ければ `None`).
    pub comment: Option<usize>,
}

impl LispParser {
    /// 方言とタブの幅から作ります.
    pub fn new(scheme: bool, tab_width: usize) -> Self {
        LispParser {
            scheme,
            tab_width,
            stack: Vec::new(),
            in_string: false,
            string_col: 0,
            first_token: false,
            comment_string: false,
            block_comment: 0,
            datum: None,
            first_sexp_col: Vec::new(),
            comment: None,
        }
    }

    /// 行ごとに新しく読む (Emacs の `indent-for-tab-command` や `newline` の経路) ように, 式の始めを決めます.
    pub fn fresh(mut self, fresh: bool) -> Self {
        self.first_token = fresh;
        self
    }

    /// 行の始めの式の桁を覚えます (まだ決まっていなければ).
    fn set_first(&mut self, line: usize, col: usize) {
        if self.first_sexp_col.len() <= line {
            self.first_sexp_col.resize(line + 1, None);
        }
        self.first_sexp_col[line].get_or_insert(col);
    }

    /// 式を, 開いているリストの要素にします.
    fn element(
        &mut self,
        kind: Kind,
        at_col: usize,
        prefix: &mut Option<usize>,
        line: usize,
        name: String,
    ) {
        let col = prefix.take().unwrap_or(at_col);
        self.set_first(line, col);
        if let Some(frame) = self.stack.last_mut() {
            frame.elements.push(Element {
                col,
                line,
                kind,
                text: name,
                hidden: false,
            });
        }
    }

    /// 1 行の `start` から `end` までを読みます.
    pub fn scan_line(&mut self, text: &[char], line: usize, start: usize, end: usize) {
        let mut i = start;
        let mut prefix: Option<usize> = None;
        let tab = self.tab_width;
        let end = end.min(text.len());
        self.comment = None;
        while i < end {
            let ch = text[i];
            let col = column_at(text, i, tab);
            if self.in_string {
                // 文字列が閉じると, 文字列の式の始め (開いた `"` の桁) が, この行の始めの式になる
                if ch == '"' {
                    if !self.comment_string {
                        // Scheme は行ごとに新しく読むので, 行の最初の式 (行頭から) を見る. Emacs Lisp は, 文字列の式の始めを見る
                        let first = if self.first_token {
                            column_at(text, indent_length(text), tab)
                        } else {
                            self.string_col
                        };
                        self.set_first(line, first);
                    }
                    self.in_string = false;
                    self.comment_string = false;
                }
                i += if ch == '\\' { 2 } else { 1 };
            } else if self.block_comment > 0 {
                let close = starts_with(text, i, "|#");
                let open = starts_with(text, i, "#|");
                if close {
                    self.block_comment -= 1;
                } else if open {
                    self.block_comment += 1;
                }
                i += if close || open { 2 } else { 1 };
            } else if ch == ' ' || ch == '\t' {
                i += 1;
            } else if ch == ';' {
                self.comment = Some(i);
                break;
            } else if self.datum.is_some() {
                i = self.skip_datum(text, i);
            } else if self.scheme && (starts_with(text, i, "#|") || starts_with(text, i, "#;")) {
                // `#;` の後の式は, コメント (Emacs の scheme-mode)
                if text[i + 1] == '|' {
                    self.block_comment = 1;
                } else {
                    self.datum = Some(0);
                }
                i += 2;
            } else if ch == '"' {
                self.element(Kind::Other, col, &mut prefix, line, String::new());
                self.in_string = true;
                self.string_col = col;
                i += 1;
            } else if ch == '(' || ch == '[' {
                self.element(Kind::List, col, &mut prefix, line, String::new());
                self.stack.push(Frame {
                    col,
                    line,
                    elements: Vec::new(),
                });
                i += 1;
            } else if ch == ')' || ch == ']' {
                self.stack.pop();
                prefix = None;
                i += 1;
            } else if matches!(ch, '\'' | '`' | ',')
                || (ch == '#' && !(self.scheme && text.get(i + 1) == Some(&'\\')))
            {
                // 式の前の印 (`'`, `` ` ``, `,`, `,@`, `#'`, `#(`. Emacs の構文表では, `#` はいつも前の印)
                prefix.get_or_insert(col);
                let two = (ch == ',' && text.get(i + 1) == Some(&'@'))
                    || (ch == '#' && text.get(i + 1) == Some(&'\''));
                i += if two { 2 } else { 1 };
            } else {
                let from = i;
                // Scheme の文字 (`#\(`) は, 括弧も名前の文字. Emacs Lisp の `?\(` は `\` で括弧を名前の文字にする
                if self.scheme && starts_with(text, i, "#\\") {
                    i += 3;
                }
                while i < text.len() && !delimiter(text[i]) {
                    i += if text[i] == '\\' { 2 } else { 1 };
                }
                if i == from {
                    // 空白の文字 (全角の空白など) は, 読み飛ばす
                    i += 1;
                    continue;
                }
                let name: String = text[from..i.min(text.len())].iter().collect();
                self.element(Kind::Symbol, col, &mut prefix, line, name);
            }
        }
    }

    /// `#;` でコメントにした式の, 字句を 1 つ読み飛ばします. 式が終わったら, コメントを終えます.
    fn skip_datum(&mut self, text: &[char], i: usize) -> usize {
        let ch = text[i];
        // 式の前の印は, 後ろの式と合わせて 1 つ
        if "'`,".contains(ch) || (ch == '#' && text.get(i + 1) != Some(&'\\')) {
            return i + 1;
        }
        let depth = self.datum.unwrap_or(0);
        let mut next = i + 1;
        if ch == '(' || ch == '[' {
            self.datum = Some(depth + 1);
        } else if ch == ')' || ch == ']' {
            self.datum = Some(depth - 1);
        } else if ch == '"' {
            self.in_string = true;
            self.comment_string = true;
        } else {
            if starts_with(text, i, "#\\") {
                next = i + 3;
            }
            while next < text.len() && !delimiter(text[next]) {
                next += 1;
            }
        }
        if self.datum.is_some_and(|d| d <= 0) {
            self.datum = None;
        }
        next
    }
}

/// Emacs の `lisp-indent-specform` (特別な引数が `count` 個のフォーム).
fn specform(count: usize, frame: &Frame, normal: usize) -> usize {
    // 行の前にある引数の数 (フォームの名前を除く)
    let args = frame.elements.len() as i64 - 1;
    let left = count as i64 - args;
    let body = frame.col + BODY_INDENT;
    // 特別な引数の行は, 始めの 2 つまでは, 本体の倍の字下げ
    if left > 0 {
        return if args <= 1 {
            frame.col + 2 * BODY_INDENT
        } else {
            normal
        };
    }
    if (count == 0 && left == 0) || (left == 0 && body <= normal) {
        return body;
    }
    normal
}

/// 行の, Lisp の字下げの桁を計算します (Emacs の `calculate-lisp-indent`). 変えないなら `None`.
fn lisp_indent(parser: &LispParser, text: &[char], scheme: bool) -> Option<usize> {
    let rest = &text[indent_length(text)..];
    // 文字列の中で始まる行と, `;;;` のコメントは, 変えない. `;` 1 つのコメントは, コメントの桁
    if (parser.in_string && !parser.comment_string) || starts_with(rest, 0, ";;;") {
        return None;
    }
    if rest.first() == Some(&';') && rest.get(1) != Some(&';') {
        return Some(COMMENT_COLUMN);
    }
    let Some(frame) = parser.stack.last() else {
        return Some(0);
    };
    let elements = &frame.elements;
    let (Some(first), Some(last)) = (elements.first(), elements.iter().rfind(|e| !e.hidden)) else {
        return Some(frame.col + 1);
    };
    // 最後の式が始まる行の, 始めの式の桁
    let last_line_col = parser.first_sexp_col.get(last.line).copied().flatten()?;
    // フォームの名前がシンボルでなければ, 名前か, 最後の式の行の始めの式にそろえる
    if first.kind != Kind::Symbol {
        return Some(if last.line == first.line {
            first.col
        } else {
            last_line_col
        });
    }
    // ふつうの字下げ: 名前と同じ行に最後の式があれば, 1 つ目の引数 (無ければ名前) にそろえる
    let mut normal = last_line_col;
    if last.line == first.line {
        normal = if elements.len() == 1 {
            first.col
        } else {
            elements[1].col
        };
    }
    let name = first.text.as_str();
    let spec = lookup(if scheme { SCHEME } else { ELISP }, name);
    // 名前が def で始まるフォームを定義の形にするのは, Scheme だけ (Emacs 30 の Emacs Lisp は, 指定だけを見る)
    let def_name = scheme && spec.is_none() && name.chars().count() > 3 && name.starts_with("def");
    if spec == Some(Spec::Defun) || def_name {
        // 定義の形 (Emacs の `lisp-indent-defform`): `(` の行に最後の式があれば, 本体の字下げ
        return Some(if last.line == frame.line {
            frame.col + BODY_INDENT
        } else {
            normal
        });
    }
    match spec {
        Some(Spec::Let) => {
            // 名前付きの let (名前が, let と同じ行にある) は, 特別な引数が 2 つ
            let named =
                elements.get(1).is_some_and(|second| {
                    second.line == first.line
                        && second.kind == Kind::Symbol
                        && second.text.chars().next().is_some_and(|c| {
                            c.is_ascii_alphanumeric() || "-+*/?!@$%^&_:~".contains(c)
                        })
                });
            Some(specform(if named { 2 } else { 1 }, frame, normal))
        }
        Some(Spec::Args(n)) => Some(specform(n, frame, normal)),
        _ => Some(normal),
    }
}

/// 行の, Fundamental モードの字下げの桁: 前の行が空白だけなら 0, そうでなければ前の行の字下げ.
fn fundamental_indent(previous: Option<&[char]>, tab_width: usize) -> usize {
    match previous {
        Some(line) if !blank(line) => column_at(line, indent_length(line), tab_width),
        _ => 0,
    }
}

/// 字下げを変える行.
#[derive(Debug, Clone, Copy, Default)]
pub struct IndentLines {
    /// 始めの行 (1 から).
    pub from: usize,
    /// 終わりの行 (1 から).
    pub to: usize,
    /// 空白だけの行を変えないなら `true` (Emacs の `indent-sexp`).
    pub skip_blank: bool,
    /// 空の行も字下げするなら `true` (`RET` の新しい行).
    pub empty_lines: bool,
    /// 行を順に読む (Emacs Lisp の `lisp-indent-region` と `indent-sexp`) なら `true`. 行ごとに新しく読むなら `false`.
    pub incremental: bool,
}

/// 行の字下げを変えた結果.
#[derive(Debug, Default)]
pub struct IndentResult {
    /// 変えた行 (1 から) と, 行の始めの空白を替える文字列.
    pub changes: Vec<(usize, String)>,
    /// 行ごとの, 字下げを変えた後の本文.
    pub lines: HashMap<usize, Vec<char>>,
    /// 字下げを計算した行.
    pub computed: HashSet<usize>,
}

/// 行の字下げを, 上から順に変えます (変えた行の後の行は, 変えた後の行を見て計算する).
pub fn indent_lines(lines: &[Vec<char>], options: IndentLines, config: &Config) -> IndentResult {
    let mut result = IndentResult::default();
    let tab = config.tab_width;
    let mut parser = config
        .lisp()
        .then(|| LispParser::new(config.scheme(), tab).fresh(!options.incremental));
    let mut previous: Option<Vec<char>> =
        (options.from > 1).then(|| lines[options.from - 2].clone());
    if let Some(p) = parser.as_mut() {
        for (n, text) in lines.iter().enumerate().take(options.from - 1) {
            p.scan_line(text, n, 0, text.len());
        }
    }
    for n in options.from..=options.to.min(lines.len()) {
        let mut text = lines[n - 1].clone();
        if (!text.is_empty() || options.empty_lines) && !(blank(&text) && options.skip_blank) {
            let col = match parser.as_ref() {
                Some(p) => lisp_indent(p, &text, config.scheme()),
                None => Some(fundamental_indent(previous.as_deref(), tab)),
            };
            let length = indent_length(&text);
            if let Some(col) = col {
                result.computed.insert(n);
                if column_at(&text, length, tab) != col {
                    // `;` 1 つのコメントは, 空白を入れ直す (Emacs の `comment-indent`)
                    let comment =
                        text.get(length) == Some(&';') && text.get(length + 1) != Some(&';');
                    let insert = if comment {
                        whitespace(0, col, config)
                    } else {
                        indent_to(&text[..length], col, config)
                    };
                    let mut changed: Vec<char> = insert.chars().collect();
                    changed.extend_from_slice(&text[length..]);
                    text = changed;
                    result.changes.push((n, insert));
                    result.lines.insert(n, text.clone());
                }
            }
        }
        if let Some(p) = parser.as_mut() {
            p.scan_line(&text, n - 1, 0, text.len());
        }
        previous = Some(text);
    }
    result
}

/// 行の始めの位置 (全体の文字の位置) を返します.
pub(crate) fn line_starts(lines: &[Vec<char>]) -> Vec<usize> {
    let mut starts = Vec::with_capacity(lines.len());
    let mut at = 0;
    for line in lines {
        starts.push(at);
        at += line.len() + 1;
    }
    starts
}

/// 字下げの結果を, 全体の位置の変更にします.
pub(crate) fn edits_of(lines: &[Vec<char>], result: &IndentResult) -> Vec<Edit> {
    let starts = line_starts(lines);
    result
        .changes
        .iter()
        .map(|(n, insert)| Edit {
            from: starts[n - 1],
            to: starts[n - 1] + indent_length(&lines[n - 1]),
            insert: insert.clone(),
        })
        .collect()
}

/// 変更の後の位置を返します (CodeMirror の `mapPos`): 消した範囲の始めは前に残り, 中と終わりは入れた文字の後ろ.
/// 入れるだけの変更の所は, `after` なら入れた文字の後ろ.
pub fn map_pos(edits: &[Edit], pos: usize, after: bool) -> usize {
    let mut shift: i64 = 0;
    for e in edits {
        let len = e.insert.chars().count() as i64;
        let deleted = e.to - e.from;
        if pos < e.from {
            break;
        }
        if deleted > 0 {
            if pos == e.from {
                return (e.from as i64 + shift) as usize;
            }
            if pos <= e.to {
                return (e.from as i64 + shift + len) as usize;
            }
        } else if pos == e.from {
            return (e.from as i64 + shift + if after { len } else { 0 }) as usize;
        }
        shift += len - deleted as i64;
    }
    (pos as i64 + shift) as usize
}

/// 字下げを変えた後の, カーソルの位置を返します (字下げの中にあれば, 字下げの後ろへ動かす).
fn cursor_after_indent(lines: &[Vec<char>], pos: usize, result: &IndentResult) -> usize {
    let starts = line_starts(lines);
    let n = starts.partition_point(|&s| s <= pos);
    let line = &lines[n - 1];
    let edits = edits_of(lines, result);
    if !result.computed.contains(&n) {
        return map_pos(&edits, pos, true);
    }
    let offset = pos - starts[n - 1];
    let old_length = indent_length(line);
    let new_length = indent_length(result.lines.get(&n).unwrap_or(line));
    let new_from = map_pos(&edits, starts[n - 1], false);
    // `;` 1 つと空白のコメント: カーソルが `;` の前か後ろなら, `;` の後ろへ (Emacs の `comment-indent`)
    if line.get(old_length) == Some(&';')
        && matches!(line.get(old_length + 1), Some(' ' | '\t'))
        && offset <= old_length + 1
    {
        return new_from + new_length + 1;
    }
    if offset <= old_length {
        new_from + new_length
    } else {
        new_from + offset - old_length + new_length
    }
}

/// 前の空でない行の, 桁より後ろの, 次の字下げの位置 (空白の後の, 文字の始め) を返します (Emacs の `indent-relative`).
fn relative_indent(lines: &[Vec<char>], line: usize, col: usize, tab: usize) -> Option<usize> {
    let mut n = line - 1;
    while n >= 1 && lines[n - 1].is_empty() {
        n -= 1;
    }
    if n < 1 {
        return None;
    }
    let text = &lines[n - 1];
    // 桁の所へ (タブの途中なら, タブの始め)
    let mut i = 0;
    while i < text.len() && column_at(text, i + 1, tab) <= col {
        i += 1;
    }
    if i < text.len() && column_at(text, i, tab) < col {
        i += 1;
    }
    if column_at(text, i, tab) > col {
        i -= 1;
    }
    let space = |c: Option<&char>| matches!(c, Some(' ' | '\t'));
    // 文字の途中なら, その語を飛ばし, 空白を飛ばす
    if !space(text.get(i)) {
        while i < text.len() && !space(text.get(i)) {
            i += 1;
        }
    }
    while i < text.len() && space(text.get(i)) {
        i += 1;
    }
    (i < text.len()).then(|| column_at(text, i, tab))
}

/// `TAB` (Emacs の `indent-for-tab-command`). Lisp では行の字下げを直し, Fundamental モードでは,
/// 前の行の次の字下げの位置まで, カーソルの所に空白を入れます (`indent-relative`).
pub fn indent_for_tab(lines: &[Vec<char>], pos: usize, config: &Config) -> Plan {
    let starts = line_starts(lines);
    let n = starts.partition_point(|&s| s <= pos);
    let line = &lines[n - 1];
    if config.lisp() {
        let options = IndentLines {
            from: n,
            to: n,
            ..IndentLines::default()
        };
        let result = indent_lines(lines, options, config);
        return Plan {
            edits: edits_of(lines, &result),
            cursor: Some(cursor_after_indent(lines, pos, &result)),
        };
    }
    let tab = config.tab_width;
    let offset = pos - starts[n - 1];
    let col = column_at(line, offset, tab);
    match relative_indent(lines, n, col, tab) {
        // 次の字下げの位置が無ければ, 次のタブの位置へ (Emacs の `tab-to-tab-stop`. 前の空白は消す)
        None => {
            let next = (col / tab + 1) * tab;
            let before = line[..offset]
                .iter()
                .rev()
                .take_while(|c| matches!(c, ' ' | '\t'))
                .count();
            let from = offset - before;
            let insert = whitespace(column_at(line, from, tab), next, config);
            Plan {
                cursor: Some(starts[n - 1] + from + insert.chars().count()),
                edits: vec![Edit {
                    from: starts[n - 1] + from,
                    to: pos,
                    insert,
                }],
            }
        }
        Some(target) => {
            let insert = whitespace(col, target, config);
            Plan {
                cursor: Some(pos + insert.chars().count()),
                edits: vec![Edit {
                    from: pos,
                    to: pos,
                    insert,
                }],
            }
        }
    }
}

/// 行の範囲を字下げします (Emacs の `indent-region`). 行の始めが終わり以上の行は含めない (空のリージョンは何もしない).
pub fn indent_region(
    lines: &[Vec<char>],
    from: usize,
    to: usize,
    point: usize,
    config: &Config,
) -> Plan {
    let starts = line_starts(lines);
    let line_of = |pos: usize| starts.partition_point(|&s| s <= pos);
    let last = line_of(to);
    let end = if to == starts[last - 1] {
        last - 1
    } else {
        last
    };
    // Emacs Lisp は, 始めの行を, 終わりに関係なく字下げする (`lisp-indent-region`)
    let to = if config.dialect == Dialect::Elisp {
        end.max(line_of(from))
    } else {
        end
    };
    let options = IndentLines {
        from: line_of(from),
        to,
        incremental: !config.scheme(),
        ..IndentLines::default()
    };
    let result = indent_lines(lines, options, config);
    let edits = edits_of(lines, &result);
    let cursor = point_after_edits(lines, &edits, point, config);
    Plan {
        cursor: Some(cursor),
        edits,
    }
}

/// 点の位置を, 字下げの変更の後ろへ直します (Emacs の `indent-line-to` と `save-excursion` の印と同じ).
/// 点が, 行頭の空白を変えた所の中にあれば: 減らすときは文字の始め, 大きく増やすときは空白の始め (空白を入れ直す),
/// 残して足すだけなら, 空白の中の同じ位置. 行頭でない変更 (コメント) の中なら, 変更の始め.
fn point_after_edits(lines: &[Vec<char>], edits: &[Edit], point: usize, config: &Config) -> usize {
    let starts = line_starts(lines);
    let tab = config.tab_width;
    match edits
        .iter()
        .find(|e| e.from < e.to && e.from <= point && point <= e.to)
    {
        Some(e) => {
            let k = starts.partition_point(|&s| s <= e.from) - 1;
            let from = map_pos(edits, e.from, false);
            // 行の始めは, 変えても動かない (Emacs の `indent-line-to` は, 空白の前の点を残す)
            if starts[k] != e.from || point == e.from {
                return from;
            }
            let line = &lines[k];
            let cur = column_at(line, indent_length(line), tab);
            let insert: Vec<char> = e.insert.chars().collect();
            let column = column_at(&insert, insert.len(), tab);
            if cur > column {
                from + insert.len()
            } else if column - (cur / tab) * tab >= tab {
                from
            } else {
                from + (point - e.from)
            }
        }
        None => map_pos(edits, point, false),
    }
}

/// カーソルの後ろの式 (終わりは `end`) の, 次の行からを字下げします (Emacs の `indent-sexp`. Lisp だけ).
/// 点の行の後ろの `;` のコメントも, `indent-for-comment` と同じにそろえます. 空白だけの行と, `;` で始まる行は変えません.
pub fn indent_sexp(lines: &[Vec<char>], pos: usize, end: usize, config: &Config) -> Plan {
    if !config.lisp() {
        return Plan::default();
    }
    let starts = line_starts(lines);
    let tab = config.tab_width;
    let scheme = config.scheme();
    let first = starts.partition_point(|&s| s <= pos);
    let mut parser = LispParser::new(scheme, tab);
    for (n, text) in lines.iter().enumerate().take(first - 1) {
        parser.scan_line(text, n, 0, text.len());
    }
    let mut edits: Vec<Edit> = Vec::new();
    // 字下げ (行の始めの空白) を変えた編集の始め. 点の位置は, これだけ `point_after_edits` で直す
    let mut indents: Vec<usize> = Vec::new();
    let mut line = first;
    let mut text = lines[first - 1].clone();
    // 今の行の, 字下げの桁 (Emacs の `calculate-lisp-indent`. コメントの行の `;;` で使う)
    let mut pending = lisp_indent(&parser, &text, scheme);
    // 変えた行の, 始めの空白の長さの差 (元の行の桁に直すのに使う)
    let mut lead_shift: i64 = 0;
    // 点が文字列の中なら, Emacs は, 点から読み始めるので, その文字列を最後の式と見ない (深さと, 要素の番号).
    // `lisp-indent-specform` は, 位置から数えるので, 引数には数える
    let mut probe = parser.clone();
    probe.scan_line(&text, first - 1, 0, pos - starts[first - 1]);
    let mut open_string = probe
        .in_string
        .then(|| {
            probe
                .stack
                .last()
                .map(|f| (probe.stack.len(), f.elements.len() - 1))
        })
        .flatten();
    loop {
        parser.scan_line(&text, line - 1, 0, text.len());
        if let Some((depth, index)) = open_string.take() {
            if let Some(frame) = parser.stack.get_mut(depth - 1) {
                frame.elements[index].hidden = true;
            }
        }
        // 行の後ろの `;` のコメント (Emacs の `indent-for-comment`). 終わりより前のものだけ
        if let Some(i) = parser.comment {
            let comment = starts[line - 1] + (i as i64 - lead_shift) as usize;
            let double = text.get(i + 1) == Some(&';');
            let triple = text.get(i + 2) == Some(&';');
            let code_end = text[..i]
                .iter()
                .rposition(|c| !matches!(c, ' ' | '\t'))
                .map_or(0, |p| p + 1);
            let orig = |k: usize| (k as i64 - lead_shift) as usize;
            let change = if code_end == 0 && double && !triple {
                // `;;` だけの行は, 字下げの桁にそろえる (Emacs の `lisp-comment-indent`)
                pending.map(|col| (starts[line - 1], whitespace(0, col, config)))
            } else if code_end == 0 && !double {
                Some((starts[line - 1], whitespace(0, COMMENT_COLUMN, config)))
            } else if code_end > 0 && !double {
                let col = column_at(&text, code_end, tab);
                let target = COMMENT_COLUMN.max(col + 1);
                Some((
                    starts[line - 1] + orig(code_end),
                    whitespace(col, target, config),
                ))
            } else {
                None
            };
            if let Some((from, insert)) = change {
                let old: String = text[code_end..i].iter().collect();
                if comment <= end && insert != old {
                    if code_end == 0 {
                        indents.push(from);
                    }
                    edits.push(Edit {
                        from,
                        to: starts[line - 1] + orig(i),
                        insert,
                    });
                }
            }
        }
        // 次の行. 始めが終わりより後ろなら, 止める
        let next = line + 1;
        if next > lines.len() || starts[next - 1] >= end {
            break;
        }
        let original = &lines[next - 1];
        let length = indent_length(original);
        let code = original.get(length).copied();
        let indent = lisp_indent(&parser, original, scheme);
        let mut next_text = original.clone();
        // 空白だけの行と, `;` で始まる行は, 字下げを変えない (コメントの行は, 上の `indent-for-comment` で)
        if code.is_some() && code != Some(';') {
            if let Some(col) = indent {
                if column_at(original, length, tab) != col {
                    let insert = indent_to(&original[..length], col, config);
                    indents.push(starts[next - 1]);
                    edits.push(Edit {
                        from: starts[next - 1],
                        to: starts[next - 1] + length,
                        insert: insert.clone(),
                    });
                    let mut changed: Vec<char> = insert.chars().collect();
                    changed.extend_from_slice(&original[length..]);
                    next_text = changed;
                }
            }
        }
        lead_shift = next_text.len() as i64 - original.len() as i64;
        pending = indent;
        text = next_text;
        line = next;
    }
    // 点が, 字下げを変えた行の空白の中なら, その規則で. コメントの中なら, 変更の始め
    let cursor = match edits
        .iter()
        .find(|e| e.from < e.to && e.from <= pos && pos <= e.to)
    {
        Some(e) if indents.contains(&e.from) => point_after_edits(lines, &edits, pos, config),
        Some(e) => map_pos(&edits, e.from, false),
        None => map_pos(&edits, pos, false),
    };
    Plan {
        cursor: Some(cursor),
        edits,
    }
}

/// `RET` (Emacs の `newline` と `electric-indent-mode`). 改行を入れ, 前の行の字下げを直して (Lisp だけ),
/// 前の行の終わりの空白を消し, 新しい行を字下げします.
pub fn newline_and_indent(lines: &[Vec<char>], pos: usize, config: &Config) -> Plan {
    let starts = line_starts(lines);
    let n = starts.partition_point(|&s| s <= pos);
    let line = &lines[n - 1];
    let offset = pos - starts[n - 1];
    let after: Vec<char> = line[offset..]
        .iter()
        .take_while(|c| matches!(c, ' ' | '\t'))
        .copied()
        .collect();
    // 前の行は, カーソルまで. 新しい行は, カーソルの後ろ
    let head: Vec<char> = line[..offset].to_vec();
    let tail: Vec<char> = line[offset + after.len()..].to_vec();
    let mut new_next = after.clone();
    new_next.extend_from_slice(&tail);
    let with = |previous: &[char], next: &[char]| -> Vec<Vec<char>> {
        let mut doc = lines.to_vec();
        doc[n - 1] = previous.to_vec();
        doc.insert(n, next.to_vec());
        doc
    };
    // 前の行の字下げを直す (Fundamental モードは, 直さない)
    let mut prev = head.clone();
    if config.lisp() {
        let simulated = with(&head, &new_next);
        let options = IndentLines {
            from: n,
            to: n,
            ..IndentLines::default()
        };
        let result = indent_lines(&simulated, options, config);
        prev = result.lines.get(&n).cloned().unwrap_or(head);
    }
    // 前の行の終わりの空白を消す
    while matches!(prev.last(), Some(' ' | '\t')) {
        prev.pop();
    }
    let with_prev = with(&prev, &new_next);
    let options = IndentLines {
        from: n + 1,
        to: n + 1,
        empty_lines: true,
        ..IndentLines::default()
    };
    let next = indent_lines(&with_prev, options, config);
    let new_line: Vec<char> = match next.lines.get(&(n + 1)) {
        Some(indented) => indented[..indent_length(indented)].to_vec(),
        None => after.clone(),
    };
    let mut insert: Vec<char> = prev.clone();
    insert.push('\n');
    insert.extend(new_line);
    // 字下げを計算しなかった (文字列の中) なら, カーソルは新しい行の始め
    let line_start = starts[n - 1];
    let cursor = if next.computed.contains(&(n + 1)) {
        // 続きが `;` と空白のコメントなら, カーソルは `;` の後ろ (`indent_for_tab` と同じ)
        let comment =
            config.lisp() && tail.first() == Some(&';') && matches!(tail.get(1), Some(' ' | '\t'));
        line_start + insert.len() + usize::from(comment)
    } else {
        line_start + prev.len() + 1
    };
    Plan {
        edits: vec![Edit {
            from: line_start,
            to: pos + after.len(),
            insert: insert.iter().collect(),
        }],
        cursor: Some(cursor),
    }
}
