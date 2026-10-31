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
fn delimiter(c: char) -> bool {
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
pub struct LispParser {
    /// Scheme か.
    scheme: bool,
    /// タブの幅.
    tab_width: usize,
    /// 開いているリスト.
    stack: Vec<Frame>,
    /// 文字列の中か.
    in_string: bool,
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
            comment_string: false,
            block_comment: 0,
            datum: None,
            first_sexp_col: Vec::new(),
            comment: None,
        }
    }

    /// 開いているリストの数.
    pub fn depth(&self) -> usize {
        self.stack.len()
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
                // 行が文字列の中で始まると, Emacs は閉じる `"` を, 式の始めと見る
                if ch == '"' {
                    if !self.comment_string {
                        self.set_first(line, col);
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
    let (Some(first), Some(last)) = (elements.first(), elements.last()) else {
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
    /// 字下げを続ける間 (`indent-sexp`): カーソルの行 (1 から) と, 行の中の位置.
    pub while_deeper: Option<(usize, usize)>,
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
    let mut parser = config.lisp().then(|| LispParser::new(config.scheme(), tab));
    let mut previous: Option<Vec<char>> =
        (options.from > 1).then(|| lines[options.from - 2].clone());
    if let Some(p) = parser.as_mut() {
        for (n, text) in lines.iter().enumerate().take(options.from - 1) {
            p.scan_line(text, n, 0, text.len());
        }
    }
    // indent-sexp: カーソルの行の, カーソルの所の深さより深い間だけ
    let mut depth = 0;
    if let (Some((line, index)), Some(p)) = (options.while_deeper, parser.as_mut()) {
        let text = &lines[line - 1];
        p.scan_line(text, line - 1, 0, index);
        depth = p.depth();
        p.scan_line(text, line - 1, index, text.len());
    }
    for n in options.from..=options.to.min(lines.len()) {
        let mut text = lines[n - 1].clone();
        if options.while_deeper.is_some() && parser.as_ref().is_some_and(|p| p.depth() <= depth) {
            break;
        }
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

/// 行の範囲を字下げします (Emacs の `indent-region`). 終わりが行の始めなら, その行は含めない.
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
    let end = if to == starts[last - 1] && to > from {
        last - 1
    } else {
        last
    };
    let options = IndentLines {
        from: line_of(from),
        to: end,
        ..IndentLines::default()
    };
    let result = indent_lines(lines, options, config);
    let edits = edits_of(lines, &result);
    Plan {
        cursor: Some(map_pos(&edits, point, true)),
        edits,
    }
}

/// カーソルの後ろの式 (終わりは `end`) の, 次の行からを字下げします (Emacs の `indent-sexp`. Lisp だけ).
pub fn indent_sexp(lines: &[Vec<char>], pos: usize, end: usize, config: &Config) -> Plan {
    if !config.lisp() {
        return Plan::default();
    }
    let starts = line_starts(lines);
    let n = starts.partition_point(|&s| s <= pos);
    // 行の始めが, 式の終わりより前の行だけ
    let last = starts.partition_point(|&s| s < end);
    let options = IndentLines {
        from: n + 1,
        to: last,
        skip_blank: true,
        ..IndentLines::default()
    };
    let result = indent_lines(lines, options, config);
    Plan {
        edits: edits_of(lines, &result),
        cursor: None,
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
        line_start + insert.len()
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
