//! シンタックスハイライト: 本文を字句に分けて、範囲と種類を返します.
//!
//! 核は、色を知りません. 「どこからどこまでが、どの種類か」だけを返し、色への割り当ては、画面 (Tauri) と
//! ターミナル (ncurses) が、それぞれ行います. 種類は、Emacs の font-lock の face に当たります.
//! 手本は、Emacs 30 の `c-mode` です (`tests/c-highlight/` で、Emacs の結果と比べる).

mod lang;

pub use lang::{Language, LanguageSpec};

/// 字句の種類 (括弧内は、Emacs の face).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    /// コメントの本文 (`font-lock-comment-face`).
    Comment,
    /// コメントの区切り (`font-lock-comment-delimiter-face`). `/*` と `//` の後の空白、`*/` の前の空白を含む.
    CommentDelimiter,
    /// ドキュメントのコメント `/** ... */` の全体 (`font-lock-doc-face`).
    DocComment,
    /// 文字列と文字定数 (`font-lock-string-face`). `#include <...>` と `#error` の本文も.
    String,
    /// キーワード (`font-lock-keyword-face`).
    Keyword,
    /// 組み込みの型 (`font-lock-type-face`).
    Type,
    /// 組み込みの定数 (`font-lock-constant-face`).
    Constant,
    /// プリプロセッサの指示 (`font-lock-preprocessor-face`).
    Preprocessor,
    /// 数値 (Emacs の `c-mode` は、色を付けない).
    Number,
}

impl TokenKind {
    /// 全ての種類 (番号を付けて使うため).
    pub const ALL: [TokenKind; 9] = [
        TokenKind::Comment,
        TokenKind::CommentDelimiter,
        TokenKind::DocComment,
        TokenKind::String,
        TokenKind::Keyword,
        TokenKind::Type,
        TokenKind::Constant,
        TokenKind::Preprocessor,
        TokenKind::Number,
    ];

    /// 画面に渡す名前 (CSS のクラス名と、Lisp の記号に使う).
    pub fn name(self) -> &'static str {
        match self {
            TokenKind::Comment => "comment",
            TokenKind::CommentDelimiter => "comment-delimiter",
            TokenKind::DocComment => "doc",
            TokenKind::String => "string",
            TokenKind::Keyword => "keyword",
            TokenKind::Type => "type",
            TokenKind::Constant => "constant",
            TokenKind::Preprocessor => "preprocessor",
            TokenKind::Number => "number",
        }
    }
}

/// 本文の範囲 (文字の位置. `start` から `end` の前まで) と、その種類.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    /// 始まり.
    pub start: usize,
    /// 終わり (この位置の文字は含まない).
    pub end: usize,
    /// 種類.
    pub kind: TokenKind,
}

/// 字句に分けます.
///
/// # Arguments
///
/// * `language` - 言語.
/// * `text` - 本文.
///
/// # Returns
///
/// 範囲の並び (始まりの順. 重ならない). 色の付かない文字は、含まない.
pub fn tokenize(language: Language, text: &str) -> Vec<Span> {
    let chars: Vec<char> = text.chars().collect();
    let mut lexer = Lexer {
        spec: language.spec(),
        chars: &chars,
        pos: 0,
        spans: Vec::new(),
    };
    lexer.run();
    lexer.spans
}

/// 字句解析器の状態.
struct Lexer<'a> {
    /// 言語の表.
    spec: &'static LanguageSpec,
    /// 本文.
    chars: &'a [char],
    /// 次に読む位置.
    pos: usize,
    /// ここまでの結果.
    spans: Vec<Span>,
}

/// 語の文字か (Emacs の `c-symbol`: 英数字、`_`、`$`).
fn is_word(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '$'
}

/// 空白か (行の中の空白. 改行を含まない).
fn is_blank(c: char) -> bool {
    c == ' ' || c == '\t'
}

impl Lexer<'_> {
    /// 位置 `i` の文字 (本文の外は `'\0'`).
    fn at(&self, i: usize) -> char {
        self.chars.get(i).copied().unwrap_or('\0')
    }

    /// 範囲を足します (空の範囲は足さない).
    fn push(&mut self, start: usize, end: usize, kind: TokenKind) {
        if start < end {
            self.spans.push(Span { start, end, kind });
        }
    }

    /// `from` から、空白を読み飛ばした位置.
    fn skip_blanks(&self, from: usize) -> usize {
        let mut i = from;
        while is_blank(self.at(i)) {
            i += 1;
        }
        i
    }

    /// 本文の終わりまで読みます.
    fn run(&mut self) {
        // 行の始まりか (空白だけを読んだあとも).
        let mut line_start = true;
        // 今の行の始まりの位置 (指示の色は、行頭の空白から).
        let mut line_begin = 0;
        // プリプロセッサの指示の中か (行の終わりまで. `\` で続く).
        let mut directive: Option<Directive> = None;
        while self.pos < self.chars.len() {
            let c = self.at(self.pos);
            let next = self.at(self.pos + 1);
            if c == '\n' {
                self.pos += 1;
                directive = None;
                line_start = true;
                line_begin = self.pos;
                continue;
            }
            if c == '\\' && next == '\n' {
                // 行の継続: 次の行は、指示の続き (指示の始まりではない)
                self.pos += 2;
                line_start = false;
                continue;
            }
            if is_blank(c) || c == '\r' {
                self.pos += 1;
                continue;
            }
            let at_line_start = line_start;
            line_start = false;
            if c == '/' && next == '*' {
                self.block_comment();
            } else if c == '/' && next == '/' {
                self.line_comment();
            } else if c == '"' || c == '\'' {
                self.quoted(c);
            } else if c == '#' && at_line_start {
                directive = self.directive(line_begin);
            } else if c == '<' && directive == Some(Directive::Include) {
                self.header_name();
            } else if c.is_ascii_digit() || (c == '.' && next.is_ascii_digit()) {
                self.number();
            } else if is_word(c) {
                self.word(directive);
            } else {
                self.pos += 1;
            }
            if let Some(Directive::Message) = directive {
                // #error と #warning の、行の終わりまでの本文
                self.message_text();
                directive = None;
            }
        }
    }

    /// `/* ... */` を読みます. `/**` で始まれば、全体がドキュメントのコメント.
    ///
    /// 区切りは、Emacs に合わせます. 始まりは、`/*` と、続く `*` (最後の `*` が `*/` の一部なら除く) と、続く空白.
    /// 終わりは、`*/` と、その前の空白. 本文が空なら、全体が区切り.
    fn block_comment(&mut self) {
        let start = self.pos;
        let doc = self.at(start + 2) == '*' && !matches!(self.at(start + 3), '/' | '*');
        let mut open_end = start + 2;
        while self.at(open_end) == '*' && self.at(open_end + 1) != '/' {
            open_end += 1;
        }
        open_end = self.skip_blanks(open_end);
        let mut close = open_end;
        while close < self.chars.len() && !(self.at(close) == '*' && self.at(close + 1) == '/') {
            close += 1;
        }
        let closed = close < self.chars.len();
        let end = if closed { close + 2 } else { close };
        self.pos = end;
        if doc {
            self.push(start, end, TokenKind::DocComment);
            return;
        }
        let mut body_end = close;
        while body_end > open_end && is_blank(self.at(body_end - 1)) {
            body_end -= 1;
        }
        if body_end <= open_end {
            // 本文が空 (空白だけ): 全体が区切り
            self.push(start, end, TokenKind::CommentDelimiter);
            return;
        }
        self.push(start, open_end, TokenKind::CommentDelimiter);
        self.push(open_end, body_end, TokenKind::Comment);
        self.push(body_end, end, TokenKind::CommentDelimiter);
    }

    /// `//` から、行の終わりまで (`\` で続く行を含む) 読みます.
    fn line_comment(&mut self) {
        let start = self.pos;
        let mut i = start;
        while self.at(i) == '/' {
            i += 1;
        }
        let body_start = self.skip_blanks(i);
        let mut end = body_start;
        while end < self.chars.len() {
            match self.at(end) {
                '\n' => break,
                '\\' if self.at(end + 1) == '\n' => end += 2,
                _ => end += 1,
            }
        }
        self.pos = end;
        self.push(start, body_start, TokenKind::CommentDelimiter);
        self.push(body_start, end, TokenKind::Comment);
    }

    /// 文字列と文字定数 (`quote` で閉じる) を読みます. `\` の次の文字は、読み飛ばす. 閉じないまま行が終われば、そこまで
    /// (その引用符は、含めない).
    fn quoted(&mut self, quote: char) {
        let start = self.pos;
        let mut i = start + 1;
        let mut closed = false;
        while i < self.chars.len() {
            match self.at(i) {
                '\\' => i += 2,
                '\n' => break,
                c if c == quote => {
                    i += 1;
                    closed = true;
                    break;
                }
                _ => i += 1,
            }
        }
        self.pos = i.min(self.chars.len());
        // 閉じない文字列の、始まりの引用符は、Emacs では警告の色 (文字列の色ではない)
        self.push(
            if closed { start } else { start + 1 },
            self.pos,
            TokenKind::String,
        );
    }

    /// `#` から、指示の名前までを読みます.
    ///
    /// # Arguments
    ///
    /// * `line_begin` - 行の始まりの位置 (色は、行頭の空白から付ける).
    ///
    /// # Returns
    ///
    /// 指示の種類 (`#` の次に名前が無ければ `None`).
    fn directive(&mut self, line_begin: usize) -> Option<Directive> {
        let start = self.pos;
        let name_start = self.skip_blanks(start + 1);
        let mut end = name_start;
        while is_word(self.at(end)) {
            end += 1;
        }
        if end == name_start {
            self.pos = start + 1;
            return None;
        }
        self.pos = end;
        self.push(line_begin, end, TokenKind::Preprocessor);
        let name: String = self.chars[name_start..end].iter().collect();
        Some(match name.as_str() {
            "include" | "include_next" | "import" => Directive::Include,
            "if" | "elif" => Directive::Condition,
            "error" | "warning" => Directive::Message,
            _ => Directive::Other,
        })
    }

    /// `#include <...>` の `<...>` を、文字列として読みます (行の中で閉じなければ、`<` だけ読み飛ばす).
    fn header_name(&mut self) {
        let start = self.pos;
        let mut i = start + 1;
        while i < self.chars.len() && !matches!(self.at(i), '>' | '\n') {
            i += 1;
        }
        if self.at(i) == '>' {
            self.pos = i + 1;
            self.push(start, self.pos, TokenKind::String);
        } else {
            self.pos = start + 1;
        }
    }

    /// `#error` と `#warning` の、行の終わりまでの本文を、文字列として読みます.
    fn message_text(&mut self) {
        let start = self.skip_blanks(self.pos);
        let mut end = start;
        while end < self.chars.len() && self.at(end) != '\n' {
            end += 1;
        }
        while end > start && matches!(self.at(end - 1), ' ' | '\t' | '\r') {
            end -= 1;
        }
        self.push(start, end, TokenKind::String);
        self.pos = end;
    }

    /// 数値 (`0x1F`、`1.5e-3`、`10UL`) を読みます.
    fn number(&mut self) {
        let start = self.pos;
        let mut i = start;
        while i < self.chars.len() {
            let c = self.at(i);
            let exponent_sign = matches!(c, '+' | '-')
                && i > start
                && matches!(self.at(i - 1), 'e' | 'E' | 'p' | 'P');
            if is_word(c) || c == '.' || exponent_sign {
                i += 1;
            } else {
                break;
            }
        }
        self.pos = i;
        self.push(start, i, TokenKind::Number);
    }

    /// 語を読んで、キーワード、型、定数なら、その種類にします.
    ///
    /// `#if` と `#elif` の行の `defined` は、指示の色です. Emacs は、`_` で区切った部分にも色を付けるので (`__off64_t_defined` の
    /// 終わりの `defined`)、同じにします.
    fn word(&mut self, directive: Option<Directive>) {
        let start = self.pos;
        let mut end = start;
        while is_word(self.at(end)) {
            end += 1;
        }
        self.pos = end;
        let word: String = self.chars[start..end].iter().collect();
        if directive == Some(Directive::Condition) {
            let mut at = start;
            for part in word.split('_') {
                if part == "defined" {
                    self.push(at, at + part.chars().count(), TokenKind::Preprocessor);
                }
                at += part.chars().count() + 1;
            }
        }
        let kind = if self.spec.is_constant(&word) {
            Some(TokenKind::Constant)
        } else if self.spec.is_type(&word) {
            Some(TokenKind::Type)
        } else if self.spec.is_keyword(&word) {
            Some(TokenKind::Keyword)
        } else {
            None
        };
        if let Some(kind) = kind {
            self.push(start, end, kind);
        }
    }
}

/// プリプロセッサの指示の種類.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Directive {
    /// `#include` など (`<...>` が文字列).
    Include,
    /// `#if` と `#elif` (`defined` が指示の色).
    Condition,
    /// `#error` と `#warning` (行の終わりまでが文字列).
    Message,
    /// それ以外.
    Other,
}
