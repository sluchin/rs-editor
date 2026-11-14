//! 構文表と, 式の走査 (Emacs 30 の syntax.c の `scan-lists`, `parse-partial-sexp`, `forward-comment`).
//!
//! 構文表は, Emacs 30 の Fundamental モード (`standard-syntax-table`), emacs-lisp-mode, scheme-mode のものです.
//! Lisp のモードでは, `;` から行の終わりまでがコメントで, Scheme では `#| |#` もコメントです (入れ子にできる).
//! Scheme の `#;` の後ろの式も, コメントです (Emacs の `scheme-syntax-propertize` と同じく, 文字の構文を上書きする).
//! 位置は 0 から数える文字の位置です. 期待値は, `emacs -Q --batch` の結果 (`tools/syntax/run.sh` で作る) と比べます.

use crate::scheme::value::Value;
use std::collections::HashMap;
use std::fmt;
use std::sync::OnceLock;

use regex::Regex;

use crate::buffer::Buffer;

/// Lisp の方言.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

/// コメントの型 a (`;` から行の終わりまで).
const STYLE_A: u8 = 0;
/// コメントの型 b (Scheme の `#| |#`).
const STYLE_B: u8 = 1;
/// コメントの型 c (Scheme の `#;` の式).
const STYLE_C: u8 = 2;

/// 文字の構文.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Syntax {
    /// 種類.
    pub class: Class,
    /// 2 文字のコメントの印 (Scheme の `#|` と `|#`).
    flags: u8,
    /// コメントの型 (Emacs の flag の `b` と `c`).
    style: u8,
    /// 入れ子にできるコメントの印なら `true` (Emacs の flag の `n`).
    nested: bool,
}

impl Syntax {
    /// 印の無い構文.
    fn plain(class: Class) -> Self {
        Syntax {
            class,
            flags: 0,
            style: STYLE_A,
            nested: false,
        }
    }

    /// `#;` の式のコメントの印 (Emacs の `< cn` と `> cn`).
    fn datum_comment(class: Class) -> Self {
        Syntax {
            class,
            flags: 0,
            style: STYLE_C,
            nested: true,
        }
    }
}

/// コメントの種類: 型と, 入れ子にできるか.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CommentKind {
    /// 型.
    style: u8,
    /// 入れ子にできるなら `true`.
    nested: bool,
}

impl CommentKind {
    /// 1 文字の印のコメントの種類.
    fn of(syntax: Syntax) -> Self {
        CommentKind {
            style: syntax.style,
            nested: syntax.nested,
        }
    }

    /// 2 文字の印 (Scheme の `#|` と `|#`) のコメントの種類: 型 b の, 入れ子にできるコメント
    /// (Emacs の `SYNTAX_FLAGS_COMMENT_STYLE`. 2 文字の印は, ほかに無い).
    const BLOCK: CommentKind = CommentKind {
        style: STYLE_B,
        nested: true,
    };
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
            // `#` は `' 14`, `|` は `" 23bn` (`#| |#` は, 型 b の入れ子にできるコメント)
            let pipe = c == '|';
            let flags = if pipe {
                FLAG_START2 | FLAG_END1
            } else {
                FLAG_START1 | FLAG_END2
            };
            return Syntax {
                class: scheme(c),
                flags,
                style: if pipe { STYLE_B } else { STYLE_A },
                nested: pipe,
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

impl From<ScanError> for crate::error::Error {
    /// Emacs の `scan-error` (データは, 理由と, 最後に読めた所と, 失敗した所. 位置は 1 から数える).
    fn from(e: ScanError) -> Self {
        crate::error::Error::new(
            "scan-error",
            vec![
                Value::String(e.message.to_string()),
                Value::Number((e.from + 1) as f64),
                Value::Number((e.to + 1) as f64),
            ],
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
    /// コメントが, 入れ子にできるコメント (Scheme の `#| |#` と `#;`) なら `true`.
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
    /// 位置ごとに上書きした構文 (Emacs の `syntax-table` のテキスト属性. Scheme の `#;` の式のコメントの印).
    props: HashMap<usize, Syntax>,
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
        let mut source = Source {
            chars,
            dialect,
            min: 0,
            max,
            props: HashMap::new(),
        };
        if dialect == Dialect::Scheme {
            source.propertize_datum_comments();
        }
        source
    }

    /// `#;` を探します (`from` から, `limit` までに終わるもの). 見つかれば, `#` の位置.
    fn find_datum_comment(&self, from: usize, limit: usize) -> Option<usize> {
        (from..limit.saturating_sub(1))
            .find(|&at| self.chars[at] == '#' && self.chars[at + 1] == ';')
    }

    /// `#;` の `#` をコメントの始めにし, 後ろの式の最後の文字をコメントの終わりにします
    /// (Emacs の `scheme-syntax-propertize`. 本文全体を, 前から 1 回だけ読む).
    fn propertize_datum_comments(&mut self) {
        let end = self.chars.len();
        let mut point = 0;
        while let Some(at) = self.find_datum_comment(point, end) {
            self.props
                .insert(at, Syntax::datum_comment(Class::CommentStart));
            point = self.sexp_comment(at + 2, end);
        }
    }

    /// `#;` の後ろの点から, 式のコメントの終わりに印を付けます (Emacs の `scheme-syntax-propertize-sexp-comment`).
    /// `#;` が, 文字列や別のコメントの中なら, 何もしない.
    ///
    /// # Returns
    ///
    /// 次に `#;` を探し始める位置.
    fn sexp_comment(&mut self, point: usize, end: usize) -> usize {
        let (state, comment) = self.parse(self.min, point);
        match (comment, state.start) {
            (Some((kind, _)), Some(start)) if kind.style == STYLE_C => {
                let mut checked = point;
                self.sexp_comment_from(start + 2, &mut checked, end)
            }
            _ => point,
        }
    }

    /// `startpos` から式を読み, その最後の文字に, コメントの終わりの印を付けます. 読み飛ばした所に `#;` があれば,
    /// 先にその式に印を付けてから, 読み直す (Emacs の `scheme-syntax-propertize-sexp-comment` の `loop`).
    ///
    /// # Returns
    ///
    /// 式の終わり (読めなければ `end`).
    fn sexp_comment_from(&mut self, startpos: usize, checked: &mut usize, end: usize) -> usize {
        loop {
            // forward-sexp: 式が無ければ終わりまで, 読めなければ (scan-error) 失敗
            let found = match self.scan_lists(startpos, 1, 0, true) {
                Ok(next) => Some(next.unwrap_or(end)),
                Err(_) => None,
            };
            let limit = found.unwrap_or(end).min(end);
            if *checked < limit {
                // 読み飛ばした所の `#;` (文字列とコメントの中のものは除く) を, 入れ子の式のコメントにする
                let mut point = *checked;
                let mut nested = None;
                while let Some(at) = self.find_datum_comment(point, limit) {
                    point = at + 2;
                    if self.parse(startpos, at).0.start.is_none() {
                        nested = Some(at);
                        break;
                    }
                }
                if nested.is_none() {
                    point = limit;
                }
                *checked = point;
                if let Some(at) = nested.filter(|_| point < limit) {
                    self.props
                        .insert(at, Syntax::datum_comment(Class::CommentStart));
                    self.sexp_comment_from(point, checked, end);
                    continue;
                }
            }
            if let Some(found) = found {
                self.props
                    .insert(found - 1, Syntax::datum_comment(Class::CommentEnd));
            }
            return found.unwrap_or(end);
        }
    }

    /// バッファの本文から作ります (絞り込んでいれば見える範囲だけを読む).
    pub fn from_buffer(buffer: &Buffer, dialect: Dialect) -> Self {
        Source::new(&buffer.text(), dialect).narrow(buffer.begv(), buffer.zv())
    }

    /// 読む範囲を決めます (範囲は本文の中に収める).
    pub fn narrow(mut self, min: usize, max: usize) -> Self {
        self.max = max.min(self.chars.len());
        self.min = min.min(self.max);
        self
    }

    /// 位置の文字の構文. 本文の外は単語とする.
    fn syntax_at(&self, pos: usize) -> Syntax {
        if let Some(syntax) = self.props.get(&pos) {
            return *syntax;
        }
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
    pub fn char_at(&self, pos: usize) -> Option<char> {
        self.chars.get(pos).copied()
    }

    /// 位置の文字の構文の種類 (前後の文字で決まる特別な場合も含む. 本文の外は単語).
    pub fn class_of(&self, pos: usize) -> Class {
        self.class_at(pos)
    }

    /// 読む範囲 (始めと終わり).
    pub fn range(&self) -> (usize, usize) {
        (self.min, self.max)
    }

    /// 位置から, 構文の種類が `classes` の文字 (`negate` なら以外の文字) を飛ばします (Emacs の `skip-syntax-forward` と `skip-syntax-backward`).
    ///
    /// # Returns
    ///
    /// 飛ばした後の位置 (`limit` を越えない).
    pub fn skip_syntax(
        &self,
        from: usize,
        classes: &[Class],
        negate: bool,
        forward: bool,
        limit: usize,
    ) -> usize {
        let matches = |p: usize| classes.contains(&self.class_at(p)) != negate;
        let mut p = from;
        if forward {
            while p < limit.min(self.chars.len()) && matches(p) {
                p += 1;
            }
        } else {
            while p > limit && matches(p - 1) {
                p -= 1;
            }
        }
        p
    }

    /// 本文の長さ (文字の数).
    pub fn size(&self) -> usize {
        self.chars.len()
    }

    /// 位置の 2 文字が, 構文の印 `first` と `second` を持てば `true` (`#|` など).
    fn pair(&self, pos: usize, first: u8, second: u8, max: usize) -> bool {
        pos + 1 < max
            && self.syntax_at(pos).flags & first != 0
            && self.syntax_at(pos + 1).flags & second != 0
    }

    /// 範囲の始めから位置までを読んで, 構文の状態を返します (Emacs の `syntax-ppss`).
    pub fn parse_state(&self, pos: usize) -> ParseState {
        self.parse(self.min, pos).0
    }

    /// `from` から `pos` までを読んで, 構文の状態と, 中にいるコメントの種類と入れ子の深さを返します
    /// (Emacs の `parse-partial-sexp`).
    fn parse(&self, from: usize, pos: usize) -> (ParseState, Option<(CommentKind, usize)>) {
        let mut state = ParseState::default();
        // 文字列の終わりの文字, 中にいるコメントの種類と入れ子の深さ
        let mut term = None;
        let mut comment: Option<CommentKind> = None;
        let mut nesting = 0;
        let mut i = from;
        while i < pos {
            let class = self.class_at(i);
            let ch = self.char_at(i);
            let mut next = i + 1;
            if state.in_string {
                // 閉じるのは, 文字列の構文の, 同じ文字だけ (`#;` の式の終わりの `"` は, コメントの終わり)
                if ch == term && class == Class::Str {
                    state.in_string = false;
                } else if class == Class::Escape {
                    next += 1;
                }
            } else if let Some(kind) = comment {
                match self.comment_step(i, kind, &mut nesting, pos) {
                    Ok(end) => {
                        comment = None;
                        state.in_comment = false;
                        next = end;
                    }
                    Err(at) => next = at,
                }
            } else if let Some((len, kind)) = self.comment_start(i, pos) {
                comment = Some(kind);
                state.in_comment = true;
                state.nested = kind.nested;
                state.start = Some(i);
                nesting = usize::from(kind.nested);
                next = i + len;
            } else if class == Class::Str {
                state.in_string = true;
                state.start = Some(i);
                term = ch;
            } else if class == Class::Escape {
                next += 1;
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
            i = next;
        }
        (state, comment.map(|kind| (kind, nesting)))
    }

    /// 位置から始まるコメントの始めの印 (`max` より前に収まるもの): 印の長さと, コメントの種類.
    fn comment_start(&self, pos: usize, max: usize) -> Option<(usize, CommentKind)> {
        if self.pair(pos, FLAG_START1, FLAG_START2, max) {
            let kind = CommentKind::BLOCK;
            return Some((2, kind));
        }
        let syntax = self.syntax_at(pos);
        (pos < max && syntax.class == Class::CommentStart).then(|| (1, CommentKind::of(syntax)))
    }

    /// コメントの中の位置 `i` を読みます (Emacs の `forw_comment` の 1 回り). 同じ型の入れ子の印で深さを変える.
    ///
    /// # Returns
    ///
    /// コメントが終われば, 終わりの印の後ろ (`Ok`). 続けば, 次に読む位置 (`Err`).
    fn comment_step(
        &self,
        i: usize,
        kind: CommentKind,
        nesting: &mut usize,
        max: usize,
    ) -> Result<usize, usize> {
        let syntax = self.syntax_at(i);
        if syntax.class == Class::CommentEnd && syntax.style == kind.style {
            if !syntax.nested && !kind.nested {
                return Ok(i + 1);
            }
            if syntax.nested && kind.nested {
                *nesting -= 1;
                if *nesting == 0 {
                    return Ok(i + 1);
                }
            }
        }
        if kind.nested && syntax.class == Class::CommentStart && CommentKind::of(syntax) == kind {
            *nesting += 1;
        }
        if self.pair(i, FLAG_END1, FLAG_END2, max) && CommentKind::BLOCK == kind {
            *nesting -= 1;
            return if *nesting == 0 { Ok(i + 2) } else { Err(i + 2) };
        }
        if kind.nested && self.pair(i, FLAG_START1, FLAG_START2, max) && CommentKind::BLOCK == kind
        {
            *nesting += 1;
            return Err(i + 2);
        }
        Err(i + 1)
    }

    /// コメントの終わりまで読みます (Emacs の `forw_comment`).
    ///
    /// # Arguments
    ///
    /// * `from` - コメントの始めの印の後ろ.
    /// * `kind` - コメントの種類.
    /// * `max` - 範囲の終わり.
    ///
    /// # Returns
    ///
    /// コメントの後ろ. 終わらなければ `None`.
    fn forward_comment_body(&self, from: usize, kind: CommentKind, max: usize) -> Option<usize> {
        let mut nesting = usize::from(kind.nested);
        let mut i = from;
        while i < max {
            match self.comment_step(i, kind, &mut nesting, max) {
                Ok(end) => return Some(end),
                Err(next) => i = next,
            }
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

    /// 終わりの印 (`;` のコメントの改行, `|#`, `#;` の式の最後の文字. `from` はその 1 文字目) から, 前へ, コメントの
    /// 始めを探します (Emacs の `back_comment`). 前へ読みながら, 同じ種類のコメントの始めの印を探し, 文字列の印や
    /// 別の種類のコメントが混じって分からなければ, 前から読み直す. `kind` のコメントの終わりでなければ `None`.
    fn back_comment(&self, from: usize, kind: CommentKind) -> Option<usize> {
        let stop = self.min;
        let comment_end = from;
        let mut from = from;
        // 文字列の印の数の偶奇 (開いている文字列の印), 分からなくなったか
        let mut string_style: Option<char> = None;
        let mut string_lossage = false;
        let mut comment_lossage = false;
        let mut comstart_pos = None;
        let mut defun_start = None;
        let mut nesting = 1;
        let mut lossage = false;
        // 前に読んだ (後ろの) 文字の構文. 始めは, 印の無い空白 (Emacs の 0)
        let mut syntax = Syntax::plain(Class::Space);
        while from > stop {
            from -= 1;
            let prev_syntax = syntax;
            syntax = self.syntax_at(from);
            let c = self.char_at(from);
            let mut code = syntax.class;
            // 2 文字の印
            let com2start = syntax.flags & FLAG_START1 != 0
                && prev_syntax.flags & FLAG_START2 != 0
                && CommentKind::BLOCK == kind;
            let mut com2end = syntax.flags & FLAG_END1 != 0 && prev_syntax.flags & FLAG_END2 != 0;
            let comstart = com2start || code == Class::CommentStart;
            // 2 文字の印が重なっていれば (`|#|` など), 前から読み直す
            if from > stop && (com2end || comstart) {
                let next = self.syntax_at(from - 1);
                if ((comstart || kind.nested)
                    && syntax.flags & FLAG_END2 != 0
                    && next.flags & FLAG_END1 != 0)
                    || ((com2end || kind.nested)
                        && syntax.flags & FLAG_START2 != 0
                        && kind.style == STYLE_B
                        && next.flags & FLAG_START1 != 0)
                {
                    lossage = true;
                    break;
                }
            }
            if com2start && comstart_pos.is_none() {
                com2end = false;
            }
            if com2end {
                code = Class::CommentEnd;
            } else if com2start {
                code = Class::CommentStart;
            } else if code == Class::CommentStart && CommentKind::of(syntax) != kind {
                // 別の種類のコメントの始めは, 見ない
                continue;
            }
            if code != Class::CommentEnd && self.char_quoted(from) {
                continue;
            }
            match code {
                Class::Str => match string_style {
                    None => string_style = c,
                    Some(open) if Some(open) == c => string_style = None,
                    Some(_) => string_lossage = true,
                },
                Class::CommentStart => {
                    if string_style.is_some() || comment_lossage || string_lossage {
                        lossage = true;
                        break;
                    }
                    if !kind.nested {
                        // 一番前の始めの印まで探す
                        comstart_pos = Some(from);
                    } else {
                        nesting -= 1;
                        if nesting == 0 {
                            return Some(from);
                        }
                    }
                }
                Class::CommentEnd => {
                    let nested = (com2end && prev_syntax.nested) || syntax.nested;
                    if syntax.style == kind.style && nested == kind.nested {
                        // 同じ種類の終わりの印: それより前は, その印のコメント
                        if !kind.nested {
                            break;
                        }
                        nesting += 1;
                    } else if comstart_pos.is_some() || c != Some('\n') {
                        comment_lossage = true;
                    }
                }
                // 行の始めの開く括弧は, 文字列とコメントの外とする (Emacs の `open-paren-in-column-0-is-defun-start`)
                Class::Open if from == stop || self.char_at(from - 1) == Some('\n') => {
                    defun_start = Some(from);
                    break;
                }
                _ => {}
            }
        }
        if !lossage {
            return comstart_pos;
        }
        // 前から読み直す. 始めは, 行の始めの開く括弧か, 終わりの印を含む文字列かコメントの始め (`syntax-ppss`)
        let mut start = defun_start.unwrap_or_else(|| {
            self.parse(self.min, comment_end)
                .0
                .start
                .unwrap_or(comment_end)
        });
        let mut found = None;
        while start < comment_end {
            let (state, comment) = self.parse(start, comment_end);
            start = comment_end;
            match comment {
                Some((k, depth))
                    if k.style == kind.style
                        && k.nested == kind.nested
                        && (!kind.nested || depth == 1) =>
                {
                    found = state.start;
                }
                // 別のコメントの中なら, 入れ子かもしれないので, そのコメントの中から読み直す
                Some(_) => {
                    found = None;
                    start = state.start.map_or(comment_end, |s| s + 2);
                }
                None => found = None,
            }
        }
        found
    }

    /// 括弧の深さを数えながら, 式を読みます (Emacs の `scan-lists` と `scan-sexps`).
    ///
    /// # Arguments
    ///
    /// * `from` - 読み始める位置.
    /// * `count` - 読む数 (負なら前へ).
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
                // コメントの始め (`;`, `#|`, `#;` の `#`)
                let mut comment = CommentKind::of(self.syntax_at(from));
                if ignore_comments && self.pair(from, FLAG_START1, FLAG_START2, max) {
                    class = Class::CommentStart;
                    comment = CommentKind::BLOCK;
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
                        let end = self.forward_comment_body(from, comment, max);
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
                            if self.char_at(from) == term && self.class_at(from) == Class::Str {
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
                // コメントの終わり (改行, `|#`, `#;` の式の最後の文字)
                let mut comment = CommentKind::of(self.syntax_at(from));
                if ignore_comments && from > min && self.pair(from - 1, FLAG_END1, FLAG_END2, max) {
                    class = Class::CommentEnd;
                    from -= 1;
                    comment = CommentKind::BLOCK;
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
                        if let Some(start) = self.back_comment(from, comment) {
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
                            if !self.char_quoted(from)
                                && self.char_at(from) == term
                                && self.class_at(from) == Class::Str
                            {
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

    /// コメントの外で, 空白と見る文字か (Emacs の `forward-comment`: 空白と, コメントの終わりの改行).
    fn blank_between_comments(&self, pos: usize) -> bool {
        match self.class_at(pos) {
            Class::Space => true,
            Class::CommentEnd => self.char_at(pos) == Some('\n'),
            _ => false,
        }
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
            while pos < max && self.blank_between_comments(pos) {
                pos += 1;
            }
            let Some((len, kind)) = self.comment_start(pos, max) else {
                return pos;
            };
            pos = self
                .forward_comment_body(pos + len, kind, max)
                .unwrap_or(max);
        }
        pos
    }

    /// コメントと空白を `count` 個飛ばします (Emacs の `forward-comment`. `count` が負なら前へ).
    ///
    /// # Returns
    ///
    /// 飛ばした後の位置と, `count` 個のコメントを飛ばせたか (空白だけを飛ばして止まったら `false`).
    pub fn skip_comments(&self, from: usize, count: i64) -> (usize, bool) {
        let (min, max) = (self.min, self.max);
        let mut p = from;
        for _ in 0..count.unsigned_abs() {
            if count > 0 {
                let mut q = p;
                while q < max && self.blank_between_comments(q) {
                    q += 1;
                }
                if self.comment_start(q, max).is_none() {
                    return (q, false);
                }
                p = self.forward_comment(q, 1);
                continue;
            }
            loop {
                if p <= min {
                    return (p, false);
                }
                if p >= min + 2 && self.pair(p - 2, FLAG_END1, FLAG_END2, max) {
                    let kind = CommentKind::BLOCK;
                    match self.back_comment(p - 2, kind) {
                        Some(start) => {
                            p = start;
                            break;
                        }
                        None => return (p, false),
                    }
                }
                let syntax = self.syntax_at(p - 1);
                match syntax.class {
                    Class::CommentEnd => match self.back_comment(p - 1, CommentKind::of(syntax)) {
                        Some(start) => {
                            p = start;
                            break;
                        }
                        // コメントの終わりでない改行は, 空白. ほかの終わりの印は, そこで止まる
                        None if self.char_at(p - 1) == Some('\n') => p -= 1,
                        None => return (p, false),
                    },
                    Class::Space => p -= 1,
                    _ => return (p, false),
                }
            }
        }
        (p, true)
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

/// 構文の指定の文字 (`\s-` の `-` など) から構文の種類.
pub fn class_of_designator(designator: char) -> Option<Class> {
    Some(match designator {
        ' ' | '-' => Class::Space,
        '.' => Class::Punct,
        'w' => Class::Word,
        '_' => Class::Symbol,
        '(' => Class::Open,
        ')' => Class::Close,
        '"' => Class::Str,
        '\\' => Class::Escape,
        '\'' => Class::Prefix,
        '<' => Class::CommentStart,
        '>' => Class::CommentEnd,
        _ => return None,
    })
}

/// 構文の種類の文字を, 正規表現の文字クラスの中身にします (`[` と `]` の間に置く). ASCII の文字は構文表から,
/// ASCII でない文字は, 単語, 空白, 括弧だけ (句読点と記号は, 含めない).
pub fn class_chars(dialect: Dialect, class: Class) -> String {
    let mut set = String::new();
    for code in 0u8..128 {
        let c = code as char;
        if syntax_of(c, dialect).class == class {
            set.push_str(&regex::escape(&c.to_string()));
        }
    }
    match class {
        Class::Word => set.push_str(r"\p{L}\p{N}"),
        Class::Space => set.push_str("\u{00a0}\u{3000}"),
        Class::Open | Class::Close => {
            let want = usize::from(class == Class::Close);
            for (i, c) in WIDE_PARENS.chars().enumerate() {
                if i % 2 == want {
                    set.push(c);
                }
            }
        }
        _ => {}
    }
    set
}
