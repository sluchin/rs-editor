//! Lisp (Scheme と Emacs Lisp) のシンタックスハイライト. Emacs 30 の scheme-mode と emacs-lisp-mode の font-lock と同じ所に,
//! 同じ色を付けます (Emacs の `scheme-font-lock-keywords-2` と `lisp-el-font-lock-keywords-2` の決まりを, 字句ごとに読み替えたもの).
//!
//! 色を付けるのは, コメント, 文字列, `(` の直後の特別な形 (`define`, `defun`, `if` など), 特別な形で定義する名前,
//! `:key` などの印です (`car` などの関数や, 数には付けない). Emacs Lisp では, 文字列の中の正規表現の括弧, 要らない `\`,
//! 閉じた括弧の後ろに続く引数 (見落としやすい) にも色を付けます.
//!
//! 解析は, 行ごとに, 字句を 1 つずつ読みます (状態は, 行をまたいで覚える). 字句は, 行をまたがない.

use std::collections::VecDeque;

use super::elisp_keywords::ELISP_KEYWORDS;
use super::{Span, TokenKind};

/// Lisp の方言.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Dialect {
    /// scheme-mode.
    Scheme,
    /// emacs-lisp-mode.
    Elisp,
}

/// 特別な形の後で色を付ける名前を待つ.
#[derive(Debug, Clone, Copy)]
struct Pending {
    /// 名前の色 (`None` なら色を付けない).
    kind: Option<TokenKind>,
    /// `(` の後の名前の色.
    in_paren: Option<TokenKind>,
    /// 名前の前に, 飛ばせる `(` の数.
    parens: usize,
}

/// `(` の直後の名前の色と, その後で待つ名前.
#[derive(Debug, Clone, Copy)]
struct Head {
    /// 名前の色 (特別な形か, 警告).
    face: TokenKind,
    /// その後で, 色を付ける名前を待つなら, その決まり.
    next: Option<Pending>,
}

/// 開いているリスト.
#[derive(Debug, Clone)]
struct Frame {
    /// リストの頭の名前 (無ければ `None`).
    head: Option<String>,
    /// 要素の数 (頭を含む).
    count: usize,
    /// 関数を呼ぶリストなら真 (Emacs Lisp).
    funcall: bool,
    /// 束縛のリスト (`let` の 1 つ目の引数) なら真 (Emacs Lisp).
    bindings: bool,
}

/// 行をまたいで覚える状態.
#[derive(Debug, Default)]
struct State {
    /// 文字列の途中なら真.
    in_string: bool,
    /// `;` のコメントの途中なら真 (`` `name' `` で分けたとき).
    line_comment: bool,
    /// 続けて読む字句 (文字列やコメントの中を分けたとき). 長さと種類.
    queue: VecDeque<(usize, TokenKind)>,
    /// `#| |#` のコメントの深さ (入れ子にできる).
    comment_depth: usize,
    /// `#;` でコメントにした式の, 括弧の深さ (コメントでなければ `None`).
    datum: Option<i64>,
    /// 直前の字句が `(` なら真.
    after_paren: bool,
    /// 直前の字句が `'` なら真.
    after_quote: bool,
    /// 特別な形の後で, 色を付ける名前を待っているなら, その決まり.
    pending: Option<Pending>,
    /// 開いているリスト (外から順に).
    stack: Vec<Frame>,
    /// 行の始めからの, 括弧の深さ.
    line_depth: i64,
    /// 閉じた括弧の後ろに続く引数の警告 (Emacs Lisp). 0: 無し, 1: 次の空白でない字句から, 2: 警告している. 行ごとに戻す.
    hidden_arg: u8,
}

/// 1 行を読む位置.
struct Stream<'a> {
    /// 行の文字.
    line: &'a [char],
    /// 次に読む位置.
    pos: usize,
    /// 読んでいる字句の始まり.
    start: usize,
}

impl Stream<'_> {
    /// 行の終わりか.
    fn eol(&self) -> bool {
        self.pos >= self.line.len()
    }

    /// 位置 `i` の文字 (行の外は `'\0'`).
    fn at(&self, i: usize) -> char {
        self.line.get(i).copied().unwrap_or('\0')
    }

    /// 1 文字読みます.
    fn next(&mut self) -> Option<char> {
        let c = self.line.get(self.pos).copied();
        self.pos += usize::from(c.is_some());
        c
    }

    /// 空白を読み飛ばします. 読んだら真.
    fn eat_space(&mut self) -> bool {
        let from = self.pos;
        while !self.eol() && is_space(self.at(self.pos)) {
            self.pos += 1;
        }
        self.pos > from
    }

    /// 次の文字が `c` なら読みます.
    fn eat(&mut self, c: char) -> bool {
        let found = !self.eol() && self.at(self.pos) == c;
        self.pos += usize::from(found);
        found
    }

    /// 次が `s` なら読みます.
    fn eat_str(&mut self, s: &str) -> bool {
        let found = s
            .chars()
            .enumerate()
            .all(|(i, c)| self.at(self.pos + i) == c)
            && !s.is_empty();
        self.pos += if found { s.chars().count() } else { 0 };
        found
    }

    /// 名前の文字を読み飛ばします.
    fn eat_word(&mut self) {
        while !self.eol() && is_word(self.at(self.pos)) {
            self.pos += 1;
        }
    }

    /// `c` の手前まで進みます. 無ければ進まずに偽.
    fn skip_to(&mut self, c: char) -> bool {
        match self.line[self.pos..].iter().position(|x| *x == c) {
            Some(offset) => {
                self.pos += offset;
                true
            }
            None => false,
        }
    }

    /// 読んでいる字句が, 空白だけか.
    fn current_is_space(&self) -> bool {
        self.line[self.start..self.pos]
            .iter()
            .all(|c| c.is_whitespace())
    }

    /// 空白を飛ばして, 行の終わりか `;` か閉じ括弧が続くか (進めない).
    fn rest_is_closing(&self) -> bool {
        let mut i = self.pos;
        while i < self.line.len() && self.line[i].is_whitespace() {
            i += 1;
        }
        i >= self.line.len() || matches!(self.line[i], ';' | ')' | ']')
    }
}

/// 空白か (JavaScript の `\s` と ` `).
fn is_space(c: char) -> bool {
    c.is_whitespace() || c == '\u{a0}'
}

/// 名前 (シンボル, 数, `#t` など) の文字.
fn is_word(c: char) -> bool {
    !c.is_whitespace() && !matches!(c, '(' | ')' | '[' | ']' | '"' | ';' | '\'' | '`' | ',')
}

/// Scheme で, `(` の直後でキーワードの色にする名前 (Emacs の `scheme-font-lock-keywords-2`).
const SCHEME_KEYWORDS: &[&str] = &[
    "and",
    "begin",
    "call-with-current-continuation",
    "call-with-input-file",
    "call-with-output-file",
    "call-with-port",
    "call/cc",
    "case",
    "case-lambda",
    "cond",
    "cond-expand",
    "define-library",
    "define-record-type",
    "define-values",
    "delay",
    "delay-force",
    "do",
    "else",
    "except",
    "export",
    "for-each",
    "force",
    "guard",
    "if",
    "import",
    "include",
    "include-ci",
    "include-library-declarations",
    "lambda",
    "let",
    "let*",
    "let*-values",
    "let-syntax",
    "let-values",
    "letrec",
    "letrec*",
    "letrec-syntax",
    "map",
    "only",
    "or",
    "parameterize",
    "prefix",
    "receive",
    "rename",
    "syntax",
    "syntax-error",
    "syntax-rules",
    "unless",
    "when",
    "λ",
];

/// Emacs Lisp の定義の形と, 定義する名前の色 (Emacs の `lisp-define-type`).
const ELISP_DEFINITIONS: &[(&str, TokenKind)] = &[
    ("cl-defgeneric", TokenKind::FunctionName),
    ("cl-defmacro", TokenKind::FunctionName),
    ("cl-defmethod", TokenKind::FunctionName),
    ("cl-defstruct", TokenKind::Type),
    ("cl-defsubst", TokenKind::FunctionName),
    ("cl-deftype", TokenKind::Type),
    ("cl-defun", TokenKind::FunctionName),
    ("defadvice", TokenKind::FunctionName),
    ("defalias", TokenKind::FunctionName),
    ("defclass", TokenKind::Type),
    ("defconst", TokenKind::VariableName),
    ("defcustom", TokenKind::VariableName),
    ("defface", TokenKind::VariableName),
    ("defgeneric", TokenKind::FunctionName),
    ("defgroup", TokenKind::Type),
    ("define-advice", TokenKind::FunctionName),
    ("define-derived-mode", TokenKind::FunctionName),
    ("define-error", TokenKind::VariableName),
    ("define-generic-mode", TokenKind::FunctionName),
    ("define-global-minor-mode", TokenKind::FunctionName),
    ("define-globalized-minor-mode", TokenKind::FunctionName),
    ("define-inline", TokenKind::FunctionName),
    ("define-minor-mode", TokenKind::FunctionName),
    ("define-skeleton", TokenKind::FunctionName),
    ("define-widget", TokenKind::FunctionName),
    ("defmacro", TokenKind::FunctionName),
    ("defmethod", TokenKind::FunctionName),
    ("defsubst", TokenKind::FunctionName),
    ("deftheme", TokenKind::Type),
    ("defun", TokenKind::FunctionName),
    ("defvar", TokenKind::VariableName),
    ("defvar-local", TokenKind::VariableName),
    ("defvaralias", TokenKind::VariableName),
    ("ert-deftest", TokenKind::FunctionName),
];

/// Emacs Lisp で, 後ろの名前を定数の色にする形.
const ELISP_FEATURES: &[&str] = &["catch", "featurep", "provide", "require", "throw"];

/// Emacs Lisp で警告の色にする形.
const ELISP_WARNINGS: &[&str] = &[
    "cl-assert",
    "cl-check-type",
    "error",
    "signal",
    "user-error",
    "warn",
];

/// Emacs Lisp で, 1 つ目の引数が束縛のリストの形.
const ELISP_LETS: &[&str] = &[
    "let",
    "let*",
    "letrec",
    "dlet",
    "if-let",
    "if-let*",
    "when-let",
    "when-let*",
    "and-let*",
    "while-let",
];

/// 名前を待ちます.
fn pending(kind: Option<TokenKind>, in_paren: Option<TokenKind>, parens: usize) -> Pending {
    Pending {
        kind,
        in_paren,
        parens,
    }
}

/// Scheme の `define` の仲間の, 定義する名前の色 (`define` と `define-public` などは関数, `define-syntax` と `define-macro`
/// は変数, `define-class` などは型).
fn scheme_define(word: &str) -> Option<TokenKind> {
    let rest = word.strip_prefix("define")?;
    let rest = rest.strip_prefix('*').unwrap_or(rest);
    match rest {
        "" | "-public" | "-method" | "-generic" | "-generic-procedure" => {
            Some(TokenKind::FunctionName)
        }
        "-syntax" | "-macro" => Some(TokenKind::VariableName),
        "-class" | "-module" => Some(TokenKind::Type),
        _ => None,
    }
}

impl Dialect {
    /// `(` の直後の名前を見ます.
    ///
    /// # Arguments
    ///
    /// * `word` - 名前.
    /// * `funcall` - 関数を呼ぶ所 (引用したリストや, 引数のリストでない) なら真.
    fn head(self, word: &str, funcall: bool) -> Option<Head> {
        let keyword = |next| {
            Some(Head {
                face: TokenKind::Keyword,
                next,
            })
        };
        match self {
            Dialect::Scheme => {
                if let Some(kind) = scheme_define(word) {
                    // `(define ((f a) b)` のように, 前の `(` は幾つでも飛ばす
                    return keyword(Some(pending(Some(kind), Some(kind), usize::MAX)));
                }
                match word {
                    // 名前付きの let の名前は, 関数の色
                    "let" => keyword(Some(pending(Some(TokenKind::FunctionName), None, 0))),
                    "library" => keyword(Some(pending(
                        Some(TokenKind::Type),
                        Some(TokenKind::Type),
                        1,
                    ))),
                    _ => SCHEME_KEYWORDS
                        .contains(&word)
                        .then(|| keyword(None))
                        .flatten(),
                }
            }
            Dialect::Elisp => {
                if let Some((_, kind)) = ELISP_DEFINITIONS.iter().find(|(name, _)| *name == word) {
                    // 関数の名前は, `(` の後なら色を付けない (変数と型は付ける)
                    let in_paren = (*kind != TokenKind::FunctionName).then_some(*kind);
                    return keyword(Some(pending(Some(*kind), in_paren, 1)));
                }
                if ELISP_FEATURES.contains(&word) {
                    return keyword(Some(pending(Some(TokenKind::Constant), None, 0)));
                }
                if ELISP_WARNINGS.contains(&word) {
                    return Some(Head {
                        face: TokenKind::Warning,
                        next: None,
                    });
                }
                // マクロとスペシャルフォームは, 関数を呼ぶ所だけ
                (funcall && ELISP_KEYWORDS.binary_search(&word).is_ok())
                    .then(|| keyword(None))
                    .flatten()
            }
        }
    }

    /// どこにあっても色を付ける名前 (`:key` など) を見ます.
    fn word(self, word: &str) -> Option<TokenKind> {
        let chars: Vec<char> = word.chars().collect();
        match self {
            Dialect::Scheme => {
                if chars.len() >= 3 && chars[0] == '<' && chars[chars.len() - 1] == '>' {
                    return Some(TokenKind::Type);
                }
                let rest = word.strip_prefix('#').unwrap_or(word);
                (rest.starts_with(':') && rest.chars().count() >= 2).then_some(TokenKind::Builtin)
            }
            Dialect::Elisp => match chars.as_slice() {
                [':', _, ..] => Some(TokenKind::Builtin),
                ['&', _, ..] => Some(TokenKind::Type),
                _ => None,
            },
        }
    }
}

/// 文字列とコメントの中の, シンボルの名前の長さ (`\` で始まる 2 文字も含む). 後ろに `look` のどれかが続く, 最も長いもの.
fn symbol_before(line: &[char], from: usize, look: &[char]) -> Option<usize> {
    let mut ends = Vec::new();
    let mut i = from;
    while i < line.len() {
        let c = line[i];
        if c == '\\' && i + 1 < line.len() {
            i += 2;
        } else if !c.is_whitespace()
            && !matches!(
                c,
                '(' | ')' | '[' | ']' | '"' | '\'' | '`' | '‘' | '’' | ',' | ';' | '\\'
            )
        {
            i += 1;
        } else {
            break;
        }
        ends.push(i);
    }
    ends.iter()
        .rev()
        .find(|end| line.get(**end).is_some_and(|c| look.contains(c)))
        .map(|end| end - from)
}

/// 文字列とコメントの中で, 色を分ける所の, 部分ごとの長さと種類 (`None` は, 文字列かコメントのまま).
type Groups = Vec<(usize, Option<TokenKind>)>;

/// 文字列の中の, 色を分ける所の決まりの数. 決まりは, 上から順に試す (`span_groups`).
const STRING_RULES: usize = 7;

/// コメントの中で, 色を分ける決まり (`` `name' ``) の番号.
const COMMENT_RULE: usize = 5;

/// 位置 `pos` から, 色を分ける決まり `rule` に合うか見ます.
///
/// 決まり: 0 `\\\\` (正規表現の `\` そのもの), 1 正規表現のグループ `\\(` `\\(?:` `\\(?1:` `\\|` `\\)`,
/// 2 コマンドのキー `\\[command]`, 3 キーマップ `\\<map>` `\\{map}`, 4 `\\=`, 5 `` `name' `` と `‘name’`,
/// 6 要らない `\` (`\.` などは, `.` と同じ. Emacs 30 で, ASCII の文字ごとに確かめたもの).
fn span_groups(line: &[char], pos: usize, rule: usize) -> Option<Groups> {
    let at = |i: usize| line.get(pos + i).copied().unwrap_or('\0');
    match rule {
        0 => (0..4).all(|i| at(i) == '\\').then(|| vec![(4, None)]),
        1 => {
            if at(0) != '\\' || at(1) != '\\' {
                return None;
            }
            let construct = match at(2) {
                '(' => {
                    // `(?:` と `(?1:`
                    let mut i = 3;
                    let mut len = 1;
                    if at(i) == '?' {
                        i += 1;
                        while at(i).is_ascii_digit() {
                            i += 1;
                        }
                        if at(i) == ':' {
                            len = i + 1 - 2;
                        }
                    }
                    len
                }
                '|' | ')' => 1,
                _ => return None,
            };
            Some(vec![
                (2, Some(TokenKind::RegexpGroup)),
                (construct, Some(TokenKind::RegexpGroup)),
            ])
        }
        2 | 3 => {
            let (open, close, kind): (&[char], &[char], TokenKind) = if rule == 2 {
                (&['['], &[']'], TokenKind::Constant)
            } else {
                (&['<', '{'], &['>', '}'], TokenKind::VariableName)
            };
            if at(0) != '\\' || at(1) != '\\' || !open.contains(&at(2)) {
                return None;
            }
            let len = symbol_before(line, pos + 3, close)?;
            Some(vec![(3, None), (len, Some(kind))])
        }
        4 => (at(0) == '\\' && at(1) == '\\' && at(2) == '=')
            .then(|| vec![(3, Some(TokenKind::Builtin))]),
        5 => {
            if !matches!(at(0), '`' | '‘' | '\'') {
                return None;
            }
            let len = symbol_before(line, pos + 1, &['\'', '’'])?;
            Some(vec![(1, None), (len, Some(TokenKind::Constant))])
        }
        _ => (at(0) == '\\'
            && "!#$%&'()*+,-./89:;<=>?@BDEFGIJKLOPQRTVWXYZ[]_`cghijklmopqwyz{|}~".contains(at(1))
            && at(1) != '\0')
            .then(|| vec![(1, Some(TokenKind::Warning))]),
    }
}

/// 続けて読む字句を, 1 つ読みます.
fn read_queued(stream: &mut Stream, state: &mut State) -> Option<TokenKind> {
    let (len, kind) = state.queue.pop_front()?;
    stream.pos += len;
    Some(kind)
}

/// 文字列かコメントの中で, 色を分ける所ならそこで字句を分けます. 字句の始まりなら, 分けた字句を続けて読む字句にします.
///
/// # Arguments
///
/// * `rules` - 試す決まりの番号.
/// * `default` - 文字列かコメントの, 字句の種類.
///
/// # Returns
///
/// 分けるなら真.
fn at_span(
    stream: &Stream,
    state: &mut State,
    rules: std::ops::Range<usize>,
    default: TokenKind,
) -> bool {
    // 行の始めの `\(` は, Emacs が括弧を見誤らないための `\` (要るもの)
    let rules = if stream.pos == 0 {
        rules.start..rules.end - 1
    } else {
        rules
    };
    for rule in rules {
        let Some(groups) = span_groups(stream.line, stream.pos, rule) else {
            continue;
        };
        if stream.pos > stream.start {
            return true;
        }
        state.queue = groups
            .into_iter()
            .map(|(len, kind)| (len, kind.unwrap_or(default)))
            .collect();
        return true;
    }
    false
}

/// 文字列を, 閉じる `"` か, 行の終わりまで読みます.
fn read_string(stream: &mut Stream, state: &mut State, dialect: Dialect) -> TokenKind {
    while !stream.eol() {
        if dialect == Dialect::Elisp && at_span(stream, state, 0..STRING_RULES, TokenKind::String) {
            return read_queued(stream, state).unwrap_or(TokenKind::String);
        }
        let ch = stream.next();
        if ch == Some('"') {
            state.in_string = false;
            break;
        }
        if ch == Some('\\') {
            stream.next();
        }
    }
    TokenKind::String
}

/// `;` のコメントを, 行の終わりまで読みます.
fn read_line_comment(stream: &mut Stream, state: &mut State, dialect: Dialect) -> TokenKind {
    while !stream.eol() {
        if dialect == Dialect::Elisp
            && at_span(
                stream,
                state,
                COMMENT_RULE..COMMENT_RULE + 1,
                TokenKind::Comment,
            )
        {
            return read_queued(stream, state).unwrap_or(TokenKind::Comment);
        }
        stream.next();
    }
    state.line_comment = false;
    TokenKind::Comment
}

/// `#| |#` のコメントを, 閉じるところか, 行の終わりまで読みます.
fn read_block_comment(stream: &mut Stream, state: &mut State) -> TokenKind {
    while !stream.eol() {
        if stream.eat_str("|#") {
            state.comment_depth -= 1;
            if state.comment_depth == 0 {
                break;
            }
        } else if stream.eat_str("#|") {
            state.comment_depth += 1;
        } else {
            stream.next();
        }
    }
    TokenKind::Comment
}

/// `#;` でコメントにした式を, 1 つの字句ずつ読みます. 式が終わったらコメントを終えます.
fn read_datum_comment(stream: &mut Stream, state: &mut State) -> TokenKind {
    if stream.eat_space() {
        return TokenKind::Comment;
    }
    let depth = state.datum.unwrap_or(0);
    let ch = stream.next().unwrap_or('\0');
    if ch == '(' || ch == '[' {
        state.datum = Some(depth + 1);
    } else if ch == ')' || ch == ']' {
        state.datum = Some(depth - 1);
    } else if ch == '"' {
        // 閉じる `"` まで (同じ行に無ければ行の終わりまで)
        if !stream.skip_to('"') {
            stream.pos = stream.line.len();
        }
        stream.next();
    } else if matches!(ch, '\'' | '`' | ',') {
        // 引用の印は, 後ろの式と合わせて 1 つ
        return TokenKind::Comment;
    } else {
        stream.eat_word();
    }
    if state.datum.unwrap_or(0) <= 0 {
        state.datum = None;
    }
    TokenKind::Comment
}

/// 開いているリストに, 要素を 1 つ足します.
fn add_element(state: &mut State, word: Option<&str>) {
    if let Some(top) = state.stack.last_mut() {
        if top.count == 0 {
            top.head = word.map(str::to_string);
        }
        top.count += 1;
    }
}

/// `(` を開きます. Emacs Lisp では, 関数を呼ぶリストかを決めます (Emacs の `lisp--el-funcall-position-p`.
/// `'(` と, `let` の束縛は, 関数を呼ばない).
fn open_list(state: &mut State, quoted: bool) {
    let parent = state.stack.last();
    let funcall = !quoted && !parent.is_some_and(|p| p.bindings);
    let bindings = parent.is_some_and(|p| {
        p.count == 1
            && p.head
                .as_deref()
                .is_some_and(|head| ELISP_LETS.contains(&head))
    });
    add_element(state, None);
    state.stack.push(Frame {
        head: None,
        count: 0,
        funcall,
        bindings,
    });
}

/// 字句を 1 つ読みます.
///
/// # Returns
///
/// 字句の種類 (色を付けなければ `None`. 括弧も).
fn read_token(stream: &mut Stream, state: &mut State, dialect: Dialect) -> Option<TokenKind> {
    if let Some(queued) = read_queued(stream, state) {
        return Some(queued);
    }
    if state.in_string {
        return Some(read_string(stream, state, dialect));
    }
    if state.line_comment {
        return Some(read_line_comment(stream, state, dialect));
    }
    if state.comment_depth > 0 {
        return Some(read_block_comment(stream, state));
    }
    if state.datum.is_some() {
        return Some(read_datum_comment(stream, state));
    }
    if stream.eat_space() {
        return None;
    }
    let after_paren = state.after_paren;
    let after_quote = state.after_quote;
    let waiting = state.pending.take();
    state.after_paren = false;
    state.after_quote = false;
    let scheme = dialect == Dialect::Scheme;
    let ch = stream.next().unwrap_or('\0');
    if ch == ';' {
        state.line_comment = true;
        return Some(read_line_comment(stream, state, dialect));
    }
    if ch == '"' {
        add_element(state, None);
        state.in_string = true;
        return Some(read_string(stream, state, dialect));
    }
    if scheme && ch == '#' && stream.eat('|') {
        state.comment_depth = 1;
        return Some(read_block_comment(stream, state));
    }
    if scheme && ch == '#' && stream.eat(';') {
        state.datum = Some(0);
        return Some(TokenKind::Comment);
    }
    if ch == '(' || ch == '[' {
        state.after_paren = ch == '(';
        state.line_depth += 1;
        open_list(state, after_quote);
        if let Some(w) = waiting.filter(|w| ch == '(' && w.parens > 0) {
            state.pending = Some(pending(w.in_paren, w.in_paren, w.parens - 1));
        }
        return None;
    }
    if ch == ')' || ch == ']' {
        state.stack.pop();
        state.line_depth -= 1;
        // 前の行で開いた括弧を閉じて, 後ろに引数が続くなら, 警告する (`;` のコメントと, 閉じる括弧は除く)
        if !scheme && state.line_depth < 0 && state.hidden_arg == 0 && !stream.rest_is_closing() {
            state.hidden_arg = 1;
        }
        return None;
    }
    if ch == '\'' {
        state.after_quote = true;
        // Emacs Lisp の `(defalias 'f` は, `'` の後の名前を待つ
        if !scheme {
            state.pending = waiting;
        }
        return None;
    }
    // `#'` は, 後ろの式と合わせて 1 つ (`#'(lambda` は, 関数を呼ぶリスト)
    if !scheme && ch == '#' && stream.eat('\'') {
        return None;
    }
    if !is_word(ch) {
        return None;
    }
    // 文字 (Scheme の `#\(`, Emacs Lisp の `?(` と `?\(`) は, 括弧も名前の文字にする
    if scheme && ch == '#' && stream.eat('\\') {
        stream.next();
    }
    if !scheme && ch == '?' {
        stream.eat('\\');
        stream.next();
    }
    stream.eat_word();
    let word: String = stream.line[stream.start..stream.pos].iter().collect();
    add_element(state, Some(&word));
    if let Some(w) = waiting {
        return w.kind;
    }
    let funcall = state.stack.last().is_some_and(|top| top.funcall);
    if after_paren {
        if let Some(head) = dialect.head(&word, funcall) {
            state.pending = head.next;
            return Some(head.face);
        }
    }
    dialect.word(&word)
}

/// 字句を 1 つ読みます. 閉じた括弧の後ろに続く引数は, 警告の色にします.
fn token(stream: &mut Stream, state: &mut State, dialect: Dialect) -> Option<TokenKind> {
    if stream.start == 0 {
        // 特別な形の後の名前は, 同じ行にあるときだけ
        state.pending = None;
        state.line_depth = 0;
        state.hidden_arg = 0;
    }
    let hidden = state.hidden_arg;
    let kind = read_token(stream, state, dialect);
    // 警告は, 括弧の後ろの, 空白でない字句から, 行の終わりまで
    if hidden == 0 || (hidden == 1 && stream.current_is_space()) {
        return kind;
    }
    state.hidden_arg = 2;
    Some(TokenKind::Warning)
}

/// 本文を字句に分けます.
///
/// # Returns
///
/// 範囲の並び (始まりの順. 重ならない. 行をまたがない. 続く同じ種類の範囲は, 1 つにする).
pub(super) fn tokenize(dialect: Dialect, text: &str) -> Vec<Span> {
    let chars: Vec<char> = text.chars().collect();
    let mut state = State::default();
    let mut spans: Vec<Span> = Vec::new();
    let mut offset = 0;
    for line in chars.split(|c| *c == '\n') {
        let mut stream = Stream {
            line,
            pos: 0,
            start: 0,
        };
        while !stream.eol() {
            stream.start = stream.pos;
            let kind = token(&mut stream, &mut state, dialect);
            let (start, end) = (offset + stream.start, offset + stream.pos);
            match (kind, spans.last_mut()) {
                (Some(kind), Some(last)) if last.end == start && last.kind == kind => {
                    last.end = end;
                }
                (Some(kind), _) => spans.push(Span { start, end, kind }),
                (None, _) => {}
            }
        }
        offset += line.len() + 1;
    }
    spans
}
