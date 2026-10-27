//! 構文表と, 式の走査 (Emacs 30 の syntax.c の `scan-lists`, `parse-partial-sexp`, `forward-comment`).
//!
//! 構文表は, Emacs 30 の Fundamental モード (`standard-syntax-table`), emacs-lisp-mode, scheme-mode のものです.
//! Lisp のモードでは, `;` から行の終わりまでがコメントで, Scheme では `#| |#` もコメントです (入れ子にできる).
//! 位置は 0 から数える文字の位置です. 期待値は, `emacs -Q --batch` の結果 (`tools/syntax/run.sh` で作る) と比べます.

use std::fmt;
use std::sync::OnceLock;

use regex::Regex;

use crate::buffer::Buffer;

/// Lisp の方言.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dialect {
    /// Lisp でない Fundamental モード.
    Fundamental,
    /// emacs-lisp-mode.
    Elisp,
    /// scheme-mode.
    Scheme,
}

impl Dialect {
    /// Lisp か (コメントを読み飛ばすか).
    fn is_lisp(self) -> bool {
        self != Dialect::Fundamental
    }
}

/// 構文の種類 (Emacs の syntax class).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Class {
    /// 空白 (` `).
    Space,
    /// 句読点 (`.`).
    Punct,
    /// 単語 (`w`).
    Word,
    /// 記号 (`_`).
    Symbol,
    /// 開く括弧 (`(`).
    Open,
    /// 閉じる括弧 (`)`).
    Close,
    /// 文字列 (`"`).
    Str,
    /// エスケープ (`\`).
    Escape,
    /// 式の前の印 (`'`).
    Prefix,
    /// コメントの始め (`<`).
    CommentStart,
    /// コメントの終わり (`>`).
    CommentEnd,
}

/// 2 文字のコメントの印 (Emacs の flag): 始めの 1 文字目.
const FLAG_START1: u8 = 1;
/// 始めの 2 文字目.
const FLAG_START2: u8 = 2;
/// 終わりの 1 文字目.
const FLAG_END1: u8 = 4;
/// 終わりの 2 文字目.
const FLAG_END2: u8 = 8;

/// 文字の構文.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Syntax {
    /// 種類.
    pub class: Class,
    /// 2 文字のコメントの印 (Scheme の `#|` と `|#`).
    flags: u8,
}

impl Syntax {
    /// 印の無い構文.
    fn plain(class: Class) -> Self {
        Syntax { class, flags: 0 }
    }
}

/// ASCII でない括弧 (開く括弧と閉じる括弧を, 交互に並べる).
const WIDE_PARENS: &str = "「」『』（）【】〔〕〈〉《》［］｛｝〖〗〘〙〚〛｟｠⦅⦆⟨⟩";

/// 空白の, ASCII でない文字 (Emacs の `standard-syntax-table`).
const WIDE_SPACES: &str = "\u{00a0}\u{3000}";

/// Fundamental モードの, ASCII の文字の構文 (書いていない文字は句読点).
fn fundamental(c: char) -> Class {
    match c {
        '\t' | '\n' | '\x0c' | '\r' | ' ' => Class::Space,
        '$' | '%' => Class::Word,
        '&' | '*' | '+' | '-' | '/' | '<' | '=' | '>' | '_' | '|' => Class::Symbol,
        '"' => Class::Str,
        '\\' => Class::Escape,
        '(' | '[' | '{' => Class::Open,
        ')' | ']' | '}' => Class::Close,
        _ => Class::Punct,
    }
}

/// emacs-lisp-mode の, ASCII の文字の構文 (書いていない文字は記号).
fn elisp(c: char) -> Class {
    match c {
        '\t' | '\x0c' | ' ' => Class::Space,
        '#' | '\'' | ',' | '`' => Class::Prefix,
        '\n' => Class::CommentEnd,
        ';' => Class::CommentStart,
        '"' => Class::Str,
        '\\' => Class::Escape,
        '(' | '[' => Class::Open,
        ')' | ']' => Class::Close,
        _ => Class::Symbol,
    }
}

/// scheme-mode の, ASCII の文字の構文 (書いていない文字は記号).
fn scheme(c: char) -> Class {
    match c {
        '\t' | '\x0c' | '\r' | ' ' => Class::Space,
        '#' | '\'' | ',' | '@' | '`' => Class::Prefix,
        '\n' => Class::CommentEnd,
        ';' => Class::CommentStart,
        '"' | '|' => Class::Str,
        '\\' => Class::Escape,
        '(' | '[' | '{' => Class::Open,
        ')' | ']' | '}' => Class::Close,
        _ => Class::Symbol,
    }
}

/// ASCII でない文字の分類 (句読点, 記号, 絵文字) に使う正規表現.
fn unicode_classes() -> &'static (Regex, Regex, Regex) {
    static CLASSES: OnceLock<(Regex, Regex, Regex)> = OnceLock::new();
    CLASSES.get_or_init(|| {
        (
            Regex::new(r"^\p{P}$").unwrap(),
            Regex::new(r"^\p{S}$").unwrap(),
            Regex::new(r"^\p{Emoji_Presentation}$").unwrap(),
        )
    })
}

/// 文字の構文を返します.
///
/// # Arguments
///
/// * `c` - 文字.
/// * `dialect` - Lisp の方言.
pub fn syntax_of(c: char, dialect: Dialect) -> Syntax {
    if c.is_ascii() {
        if dialect == Dialect::Scheme && (c == '#' || c == '|') {
            let flags = if c == '#' {
                FLAG_START1 | FLAG_END2
            } else {
                FLAG_START2 | FLAG_END1
            };
            return Syntax {
                class: scheme(c),
                flags,
            };
        }
        if c.is_ascii_alphanumeric() {
            return Syntax::plain(Class::Word);
        }
        return Syntax::plain(match dialect {
            Dialect::Scheme => scheme(c),
            Dialect::Elisp => elisp(c),
            Dialect::Fundamental => fundamental(c),
        });
    }
    if let Some(index) = WIDE_PARENS.chars().position(|p| p == c) {
        return Syntax::plain(if index % 2 == 0 {
            Class::Open
        } else {
            Class::Close
        });
    }
    if WIDE_SPACES.contains(c) {
        return Syntax::plain(Class::Space);
    }
    let mut buf = [0u8; 4];
    let s: &str = c.encode_utf8(&mut buf);
    let (punct, symbol, emoji) = unicode_classes();
    if punct.is_match(s) {
        return Syntax::plain(Class::Punct);
    }
    if symbol.is_match(s) && !emoji.is_match(s) {
        return Syntax::plain(Class::Symbol);
    }
    Syntax::plain(Class::Word)
}

/// 式の走査の失敗 (Emacs の `scan-error`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanError {
    /// 理由 (`Unbalanced parentheses` など).
    pub message: &'static str,
    /// 最後に読めた所.
    pub from: usize,
    /// 失敗した所.
    pub to: usize,
}

impl fmt::Display for ScanError {
    /// Emacs の表示 (`Scan error: "Unbalanced parentheses", 4, 11`. 位置は 1 から数える).
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Scan error: \"{}\", {}, {}",
            self.message,
            self.from + 1,
            self.to + 1
        )
    }
}

/// 位置までの構文の状態 (Emacs の `parse-partial-sexp` の結果).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ParseState {
    /// 括弧の深さ.
    pub depth: i64,
    /// 開いている括弧の位置 (外から順).
    pub opens: Vec<usize>,
    /// 文字列の中なら `true`.
    pub in_string: bool,
    /// コメントの中なら `true`.
    pub in_comment: bool,
    /// コメントが, 入れ子にできるコメント (Scheme の `#| |#`) なら `true`.
    pub nested: bool,
    /// 文字列かコメントの始め (どちらの中でもなければ `None`).
    pub start: Option<usize>,
}

/// 本文と, 読む範囲 (Emacs のナローイング), 方言.
#[derive(Debug, Clone)]
pub struct Source {
    /// 本文 (1 文字ずつ).
    chars: Vec<char>,
    /// Lisp の方言.
    dialect: Dialect,
    /// 範囲の始め.
    min: usize,
    /// 範囲の終わり.
    max: usize,
}

impl Source {
    /// 本文全体を読む範囲にします.
    ///
    /// # Arguments
    ///
    /// * `text` - 本文.
    /// * `dialect` - Lisp の方言.
    pub fn new(text: &str, dialect: Dialect) -> Self {
        let chars: Vec<char> = text.chars().collect();
        let max = chars.len();
        Source {
            chars,
            dialect,
            min: 0,
            max,
        }
    }

    /// バッファの本文から作ります.
    pub fn from_buffer(buffer: &Buffer, dialect: Dialect) -> Self {
        Source::new(&buffer.text(), dialect)
    }

    /// 読む範囲を決めます (範囲は本文の中に収める).
    pub fn narrow(mut self, min: usize, max: usize) -> Self {
        self.max = max.min(self.chars.len());
        self.min = min.min(self.max);
        self
    }

    /// 位置の文字の構文. 本文の外は単語とする.
    fn syntax_at(&self, pos: usize) -> Syntax {
        let at = |p: usize| self.chars.get(p).copied();
        if self.dialect == Dialect::Elisp {
            // `,@` の `@` は, 後ろの記号に付けない. `##` は, 空の名前の記号 (Emacs の `elisp-mode-syntax-propertize`)
            if at(pos) == Some('@') && pos > 0 && at(pos - 1) == Some(',') {
                return Syntax::plain(Class::Prefix);
            }
            if at(pos) == Some('#')
                && (at(pos + 1) == Some('#') || (pos > 0 && at(pos - 1) == Some('#')))
            {
                return Syntax::plain(Class::Symbol);
            }
        }
        at(pos).map_or(Syntax::plain(Class::Word), |c| syntax_of(c, self.dialect))
    }

    /// 位置の文字の種類.
    fn class_at(&self, pos: usize) -> Class {
        self.syntax_at(pos).class
    }

    /// 位置の文字.
    fn char_at(&self, pos: usize) -> Option<char> {
        self.chars.get(pos).copied()
    }

    /// 位置の 2 文字が, 構文の印 `first` と `second` を持てば `true` (`#|` など).
    fn pair(&self, pos: usize, first: u8, second: u8, max: usize) -> bool {
        pos + 1 < max
            && self.syntax_at(pos).flags & first != 0
            && self.syntax_at(pos + 1).flags & second != 0
    }

    /// 範囲の始めから位置までを読んで, 構文の状態を返します (Emacs の `syntax-ppss`).
    pub fn parse_state(&self, pos: usize) -> ParseState {
        let mut state = ParseState::default();
        // 文字列の終わりの文字, コメントの入れ子の深さ (`;` のコメントは 0)
        let mut term = None;
        let mut nesting = 0;
        let mut i = self.min;
        while i < pos {
            let class = self.class_at(i);
            let ch = self.char_at(i);
            if state.in_string {
                if ch == term {
                    state.in_string = false;
                } else if class == Class::Escape {
                    i += 1;
                }
            } else if state.in_comment {
                if nesting == 0 {
                    if class == Class::CommentEnd {
                        state.in_comment = false;
                    }
                } else if self.pair(i, FLAG_END1, FLAG_END2, pos) {
                    i += 1;
                    nesting -= 1;
                    if nesting == 0 {
                        state.in_comment = false;
                    }
                } else if self.pair(i, FLAG_START1, FLAG_START2, pos) {
                    i += 1;
                    nesting += 1;
                }
            } else if self.pair(i, FLAG_START1, FLAG_START2, pos) {
                state.in_comment = true;
                state.nested = true;
                state.start = Some(i);
                nesting = 1;
                i += 1;
            } else if class == Class::CommentStart {
                state.in_comment = true;
                state.nested = false;
                state.start = Some(i);
                nesting = 0;
            } else if class == Class::Str {
                state.in_string = true;
                state.start = Some(i);
                term = ch;
            } else if class == Class::Escape {
                i += 1;
            } else if class == Class::Open {
                state.opens.push(i);
                state.depth += 1;
            } else if class == Class::Close {
                state.opens.pop();
                state.depth -= 1;
            }
            if !state.in_string && !state.in_comment {
                state.start = None;
                state.nested = false;
            }
            i += 1;
        }
        state
    }

    /// コメントの終わりまで読みます (Emacs の `forw_comment`).
    ///
    /// # Arguments
    ///
    /// * `from` - コメントの始めの印の後ろ.
    /// * `nesting` - 入れ子の深さ (`;` のコメントは 0, `#|` のコメントは 1).
    /// * `max` - 範囲の終わり.
    ///
    /// # Returns
    ///
    /// コメントの後ろ. 終わらなければ `None`.
    fn forward_comment_body(&self, from: usize, mut nesting: usize, max: usize) -> Option<usize> {
        let mut i = from;
        while i < max {
            if nesting == 0 {
                if self.class_at(i) == Class::CommentEnd {
                    return Some(i + 1);
                }
            } else if self.pair(i, FLAG_END1, FLAG_END2, max) {
                i += 1;
                nesting -= 1;
                if nesting == 0 {
                    return Some(i + 1);
                }
            } else if self.pair(i, FLAG_START1, FLAG_START2, max) {
                i += 1;
                nesting += 1;
            }
            i += 1;
        }
        None
    }

    /// 式の走査で使う, 位置の文字の種類. emacs-lisp-mode では `scan-sexps` は, ASCII でない文字を全て記号とする
    /// (Emacs の `multibyte-syntax-as-symbol`).
    fn scan_class(&self, pos: usize, sexp: bool) -> Class {
        let multibyte = self.char_at(pos).is_some_and(|c| !c.is_ascii());
        if sexp && self.dialect == Dialect::Elisp && multibyte {
            Class::Symbol
        } else {
            self.class_at(pos)
        }
    }

    /// 位置の文字が, エスケープ (`\`) の後なら `true` (Emacs の `char_quoted`).
    fn char_quoted(&self, pos: usize) -> bool {
        let mut count = 0;
        while pos > count + self.min && self.class_at(pos - 1 - count) == Class::Escape {
            count += 1;
        }
        count % 2 == 1
    }

    /// 終わりの印 (`;` のコメントの改行, `|#`) から, コメントの始めを探します (Emacs の `back_comment`).
    /// その種類のコメントの終わりでなければ (`#| |#` の中の改行など) `None`.
    fn back_comment(&self, from: usize, nested: bool) -> Option<usize> {
        let state = self.parse_state(from);
        if state.in_comment && state.nested == nested {
            state.start
        } else {
            None
        }
    }

    /// 括弧の深さを数えながら, 式を読みます (Emacs の `scan-lists` と `scan-sexps`).
    ///
    /// # Arguments
    ///
    /// * `from` - 読み始める位置.
    /// * `count` - 読む数 (負なら, 前へ).
    /// * `depth` - 始めの深さ (`1` で外のリストの終わりまで, `-1` で中のリストの始めまで).
    /// * `sexp` - 記号や文字列も, 式として数えるなら `true` (`scan-sexps`).
    ///
    /// # Returns
    ///
    /// 読んだ後の位置. 範囲の端まで式が無ければ `None`.
    ///
    /// # Errors
    ///
    /// 括弧が合わないとき.
    pub fn scan_lists(
        &self,
        from: usize,
        mut count: i64,
        mut depth: i64,
        sexp: bool,
    ) -> Result<Option<usize>, ScanError> {
        let (min, max) = (self.min, self.max);
        let ignore_comments = self.dialect.is_lisp();
        let min_depth = depth.min(0);
        let mut from = from;
        let mut last_good = from;
        // 記号や単語の続きの文字か
        let symbol_char =
            |class: Class| matches!(class, Class::Word | Class::Symbol | Class::Prefix);

        while count > 0 {
            let mut done = false;
            while from < max && !done {
                let mut class = self.scan_class(from, sexp);
                if depth == min_depth {
                    last_good = from;
                }
                // `#|` のコメント
                let mut nesting = 0;
                if ignore_comments && self.pair(from, FLAG_START1, FLAG_START2, max) {
                    class = Class::CommentStart;
                    nesting = 1;
                    from += 1;
                }
                from += 1;
                let lose = |from: usize| ScanError {
                    message: "Unbalanced parentheses",
                    from: last_good,
                    to: from,
                };
                match class {
                    Class::Escape | Class::Word | Class::Symbol => {
                        if class == Class::Escape {
                            if from == max {
                                return Err(lose(from));
                            }
                            from += 1;
                        }
                        if depth != 0 || !sexp {
                            continue;
                        }
                        while from < max {
                            let next = self.scan_class(from, sexp);
                            if next == Class::Escape {
                                from += 1;
                                if from == max {
                                    return Err(lose(from));
                                }
                            } else if !symbol_char(next) {
                                break;
                            }
                            from += 1;
                        }
                        done = true;
                    }
                    Class::CommentStart => {
                        let end = self.forward_comment_body(from, nesting, max);
                        from = end.unwrap_or(max);
                        if end.is_none() {
                            if depth == 0 {
                                done = true;
                            } else {
                                return Err(lose(from));
                            }
                        }
                    }
                    Class::Open => {
                        depth += 1;
                        if depth == 0 {
                            done = true;
                        }
                    }
                    Class::Close => {
                        depth -= 1;
                        if depth == 0 {
                            done = true;
                        } else if depth < min_depth {
                            return Err(ScanError {
                                message: "Containing expression ends prematurely",
                                from: last_good,
                                to: from,
                            });
                        }
                    }
                    Class::Str => {
                        let term = self.char_at(from - 1);
                        loop {
                            if from >= max {
                                return Err(lose(from));
                            }
                            if self.char_at(from) == term {
                                break;
                            }
                            if self.scan_class(from, sexp) == Class::Escape {
                                from += 1;
                            }
                            from += 1;
                        }
                        from += 1;
                        if depth == 0 && sexp {
                            done = true;
                        }
                    }
                    _ => {}
                }
            }
            if !done {
                if depth != 0 {
                    return Err(ScanError {
                        message: "Unbalanced parentheses",
                        from: last_good,
                        to: from,
                    });
                }
                return Ok(None);
            }
            count -= 1;
        }

        while count < 0 {
            let mut done = false;
            while from > min && !done {
                from -= 1;
                let mut class = self.scan_class(from, sexp);
                if depth == min_depth {
                    last_good = from;
                }
                // `|#` のコメントの終わり
                let nested =
                    ignore_comments && from > min && self.pair(from - 1, FLAG_END1, FLAG_END2, max);
                if nested {
                    class = Class::CommentEnd;
                    from -= 1;
                }
                if class != Class::CommentEnd && self.char_quoted(from) {
                    from -= 1;
                    class = Class::Word;
                }
                match class {
                    Class::Word | Class::Symbol | Class::Escape => {
                        if depth != 0 || !sexp {
                            continue;
                        }
                        while from > min {
                            let prev = self.scan_class(from - 1, sexp);
                            if prev == Class::CommentEnd {
                                break;
                            }
                            if self.char_quoted(from - 1) {
                                from -= 1;
                            } else if !symbol_char(prev) {
                                break;
                            }
                            from -= 1;
                        }
                        done = true;
                    }
                    // 深さが -1 から始まるとき (`backward-down-list`) は, 閉じる括弧で 0 になって止まる
                    Class::Close => {
                        depth += 1;
                        if depth == 0 {
                            done = true;
                        }
                    }
                    Class::Open => {
                        depth -= 1;
                        if depth == 0 {
                            done = true;
                        } else if depth < min_depth {
                            return Err(ScanError {
                                message: "Containing expression ends prematurely",
                                from: last_good,
                                to: from,
                            });
                        }
                    }
                    Class::CommentEnd => {
                        if let Some(start) = self.back_comment(from, nested) {
                            from = start;
                        }
                    }
                    Class::Str => {
                        let term = self.char_at(from);
                        loop {
                            if from == min {
                                return Err(ScanError {
                                    message: "Unbalanced parentheses",
                                    from: last_good,
                                    to: from,
                                });
                            }
                            from -= 1;
                            if !self.char_quoted(from) && self.char_at(from) == term {
                                break;
                            }
                        }
                        if depth == 0 && sexp {
                            done = true;
                        }
                    }
                    _ => {}
                }
            }
            if !done {
                if depth != 0 {
                    return Err(ScanError {
                        message: "Unbalanced parentheses",
                        from: last_good,
                        to: from,
                    });
                }
                return Ok(None);
            }
            count += 1;
        }
        Ok(Some(from))
    }

    /// 空白とコメントを飛ばします (Emacs の `forward-comment` の前向き. 改行も空白とする).
    ///
    /// # Arguments
    ///
    /// * `from` - 位置.
    /// * `count` - 飛ばすコメントの数.
    ///
    /// # Returns
    ///
    /// 飛ばした後の位置.
    pub fn forward_comment(&self, from: usize, count: usize) -> usize {
        let max = self.max;
        let mut pos = from;
        for _ in 0..count {
            while pos < max && matches!(self.class_at(pos), Class::Space | Class::CommentEnd) {
                pos += 1;
            }
            let nesting = usize::from(self.pair(pos, FLAG_START1, FLAG_START2, max));
            if nesting == 0 && !self.is_comment_start(pos) {
                return pos;
            }
            pos = self
                .forward_comment_body(pos + 1 + nesting, nesting, max)
                .unwrap_or(max);
        }
        pos
    }

    /// 前の, 式の前の印 (`'` など) を飛ばします (Emacs の `backward-prefix-chars`).
    pub fn backward_prefix_chars(&self, mut pos: usize) -> usize {
        while pos > self.min
            && self.class_at(pos - 1) == Class::Prefix
            && !self.char_quoted(pos - 1)
        {
            pos -= 1;
        }
        pos
    }

    /// 位置の文字が, 空白の構文か.
    pub fn is_whitespace(&self, pos: usize) -> bool {
        pos < self.chars.len() && self.class_at(pos) == Class::Space
    }

    /// 位置の文字が, 開く括弧の構文か.
    pub fn is_open_paren(&self, pos: usize) -> bool {
        pos < self.chars.len() && self.class_at(pos) == Class::Open
    }

    /// 位置の文字が, コメントの始めの構文か (Emacs の `\s<`).
    pub fn is_comment_start(&self, pos: usize) -> bool {
        pos < self.chars.len() && self.class_at(pos) == Class::CommentStart
    }
}
