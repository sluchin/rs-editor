//! Rust のシンタックスハイライト. rust-lang/rust-mode (`rust-mode.el` 1.0.6) の font-lock と同じ所に、同じ色を付けます.
//!
//! rust-mode は、構文表 (コメントと文字列) と、正規表現の並び (`rust-font-lock-keywords`) で色を付けます. ここでも同じ形です.
//! まず、コメント、文字列、文字、生の文字列を読み (構文表と `rust-syntax-propertize` に当たる)、次に、正規表現を上から順に当てて、
//! 文字ごとの種類を決めます. 後の規則は、Emacs の font-lock と同じく、先の規則が付けた色を上書きしません
//! (`keep` の規則だけは、色の付いていない文字にだけ付けます. 付けない規則は、範囲に 1 文字でも色が付いていれば、全体を付けません).
//!
//! 色の付く所 (Emacs の face): キーワードと、`&raw const` の `raw` (keyword)、組み込みの型 `u8` など (type)、`unsafe` (warning)、
//! 属性 `#[...]` と、マクロ `name!` (preprocessor)、`println!` など (builtin)、フィールド名 `name:` と `let` の名前とライフタイム (variable-name)、
//! 大文字で始まる名前と `Foo::` (type)、`foo::` (constant)、`?` (builtin)、数値の型の接尾辞 `1u8` (type)、
//! `struct` `enum` `union` `type` の次の名前 (type)、`mod` `use` の次の名前 (constant)、`fn` の次の名前 (function-name).

use std::sync::OnceLock;

use regex::Regex;

use super::paint::{Fill, Painter};
use super::{Span, TokenKind};

/// キーワード (rust-mode の `rust-keywords`).
const KEYWORDS: &[&str] = &[
    "as",
    "async",
    "await",
    "box",
    "break",
    "const",
    "continue",
    "crate",
    "do",
    "dyn",
    "else",
    "enum",
    "extern",
    "existential",
    "false",
    "fn",
    "for",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "priv",
    "pub",
    "ref",
    "return",
    "self",
    "static",
    "struct",
    "super",
    "true",
    "trait",
    "type",
    "try",
    "use",
    "virtual",
    "where",
    "while",
    "yield",
];

/// 組み込みの型 (rust-mode の `rust-special-types`).
const SPECIAL_TYPES: &[&str] = &[
    "u8", "i8", "u16", "i16", "u32", "i32", "u64", "i64", "u128", "i128", "f16", "f32", "f64",
    "f128", "isize", "usize", "bool", "str", "char",
];

/// 色を付ける書式のマクロ (rust-mode の `rust-builtin-formatting-macros`. `write!` と `writeln!` は別).
const FORMATTING_MACROS: &[&str] = &["eprint", "eprintln", "format", "print", "println"];

/// 識別子 (rust-mode の `rust-re-ident`. Emacs の標準の構文表では、`$` も語の文字).
const IDENT: &str = r"[\w$\u{80}-\u{10FFFF}]+";

/// 小文字で始まる識別子 (`rust-re-lc-ident`).
const LC_IDENT: &str = r"[\p{Ll}_\u{80}-\u{10FFFF}][\w$\u{80}-\u{10FFFF}]*";

/// 大文字で始まる識別子 (`rust-re-uc-ident`).
const UC_IDENT: &str = r"\p{Lu}[\w$\u{80}-\u{10FFFF}]*";

/// 空白 (Emacs の `[[:space:]]`. rust-mode の構文表では、改行は空白ではない).
const SPACE: &str = r"[ \t\x0c]";

/// 正規表現の規則 1 つ.
struct Rule {
    /// 正規表現.
    re: Regex,
    /// 色を付ける部分 (番号付きの括弧).
    group: usize,
    /// 色.
    kind: TokenKind,
    /// 色の付け方.
    fill: Fill,
}

/// `\b(a|b|...)\b` の形の正規表現.
fn words(list: &[&str]) -> String {
    format!(r"\b(?:{})\b", list.join("|"))
}

/// 規則を作ります.
fn rule(pattern: &str, group: usize, kind: TokenKind, fill: Fill) -> Rule {
    Rule {
        re: Regex::new(pattern).expect("正規表現が正しい"),
        group,
        kind,
        fill,
    }
}

/// 規則の並び (rust-mode の `rust-font-lock-keywords` と同じ順).
///
/// `Foo::` と `foo::` の規則は、後ろに `<` が続けば当てない決まりなので、`paths` で別に扱う.
fn rules() -> &'static Vec<Rule> {
    static RULES: OnceLock<Vec<Rule>> = OnceLock::new();
    RULES.get_or_init(|| {
        let item = |keyword: &str, kind: TokenKind| {
            let pattern = format!(
                r"\b{keyword}\b(?:<{SPACE}*'{IDENT}{SPACE}*>)?{SPACE}+({IDENT})"
            );
            rule(&pattern, 1, kind, Fill::Whole)
        };
        let formatting = words(FORMATTING_MACROS);
        vec![
            // キーワード
            rule(&words(KEYWORDS), 0, TokenKind::Keyword, Fill::Whole),
            // 文脈で決まるキーワード
            rule(
                &format!(r"\b(default){SPACE}+fn\b"),
                1,
                TokenKind::Keyword,
                Fill::Whole,
            ),
            rule(
                &format!(r"(?m)(?:\s|^)(\bunion\b){SPACE}+{IDENT}"),
                1,
                TokenKind::Keyword,
                Fill::Whole,
            ),
            rule(
                &format!(r"&{SPACE}*(raw){SPACE}+(?:const|mut)"),
                1,
                TokenKind::Keyword,
                Fill::Whole,
            ),
            // 組み込みの型
            rule(&words(SPECIAL_TYPES), 0, TokenKind::Type, Fill::Whole),
            // `unsafe`
            rule(r"\bunsafe\b", 0, TokenKind::Warning, Fill::Whole),
            // 属性 `#[bar(baz)]` と `#![bar(baz)]`
            rule(
                &format!(r"(#!?\[{IDENT}[^\]]*\])"),
                1,
                TokenKind::Preprocessor,
                Fill::Gaps,
            ),
            // 書式のマクロ
            rule(
                &format!(r#"({formatting}!)[ \t\x0c\n]*[({{\[][ \t\x0c\n]*(?:r#*)?"?"#),
                1,
                TokenKind::Builtin,
                Fill::Whole,
            ),
            rule(
                &format!(r#"(\bwrite(?:ln)?\b!)[ \t\x0c\n]*[({{\[][ \t\x0c\n]*{SPACE}*[^"]+,{SPACE}*(?:r#*)?""#),
                1,
                TokenKind::Builtin,
                Fill::Whole,
            ),
            // マクロの呼び出し `foo!` (`!` も含む)
            rule(
                &format!(r"({IDENT}!)[({{\[ \t\x0c]"),
                1,
                TokenKind::Preprocessor,
                Fill::Whole,
            ),
            // フィールド名 `foo:` (`:` は含まない)
            rule(
                &format!(r"({IDENT}){SPACE}*:[^:]"),
                1,
                TokenKind::VariableName,
                Fill::Whole,
            ),
            // 大文字で始まる名前は、型かコンストラクタ
            rule(&format!(r"\b({UC_IDENT})\b"), 1, TokenKind::Type, Fill::Whole),
            // 型を推論する束縛
            rule(
                &format!(
                    r"\b(?:let{SPACE}+ref|let|ref|for){SPACE}+(?:mut{SPACE}+)?({IDENT})\b"
                ),
                1,
                TokenKind::VariableName,
                Fill::Whole,
            ),
            // `foo::` と `Foo::` は、`paths` で当てる
            // ライフタイム `'foo`
            rule(
                &format!(r"'({IDENT})[^']"),
                1,
                TokenKind::VariableName,
                Fill::Whole,
            ),
            // `?` の演算子
            rule(r"\?", 0, TokenKind::Builtin, Fill::Whole),
            // 接尾辞のある数値
            rule(
                &format!(
                    r"\b(?:0[box]?|[1-9])[\da-fA-F_.]*(?:[eE][+-]?[\d_])?({})\b",
                    [
                        "u8", "i8", "u16", "i16", "u32", "i32", "u64", "i64", "u128", "i128",
                        "usize", "isize", "f32", "f64"
                    ]
                    .join("|")
                ),
                1,
                TokenKind::Type,
                Fill::Whole,
            ),
            // `struct Foo` などの名前
            item("enum", TokenKind::Type),
            item("struct", TokenKind::Type),
            item("union", TokenKind::Type),
            item("type", TokenKind::Type),
            item("mod", TokenKind::Constant),
            item("use", TokenKind::Constant),
            item("fn", TokenKind::FunctionName),
        ]
    })
}

/// `Foo::` と `foo::` の規則で使う正規表現 (大文字で始まる名前と、小文字で始まる名前).
fn path_regexes() -> &'static (Regex, Regex) {
    static PATHS: OnceLock<(Regex, Regex)> = OnceLock::new();
    PATHS.get_or_init(|| {
        (
            Regex::new(&format!("({UC_IDENT})::")).expect("正規表現が正しい"),
            Regex::new(&format!("({LC_IDENT})::")).expect("正規表現が正しい"),
        )
    })
}

/// Rust の構文を読む (コメント、文字列、文字、生の文字列).
impl Painter {
    /// コメントと文字列と文字を読んで、種類を決めます (構文表と、`rust-syntax-propertize` に当たる).
    fn syntax(&mut self) {
        let mut i = 0;
        while i < self.chars.len() {
            let c = self.chars[i];
            i = if c == '/' && self.at(i + 1) == '/' {
                self.line_comment(i)
            } else if c == '/' && self.at(i + 1) == '*' {
                self.block_comment(i)
            } else if c == '"' {
                self.string(i)
            } else if c == '\'' {
                self.char_literal(i)
            } else if c == 'r' {
                self.raw_string(i)
            } else {
                i + 1
            };
        }
    }

    /// コメントの始まり `i` が、ドキュメントのコメントか (rust-mode の `/\([*][*!][^*!]\|/[/!][^/!]\)`).
    fn is_doc(&self, i: usize) -> bool {
        let (a, b, c) = (self.at(i + 1), self.at(i + 2), self.at(i + 3));
        let more = i + 3 < self.chars.len();
        more && match a {
            '*' => matches!(b, '*' | '!') && !matches!(c, '*' | '!'),
            _ => matches!(b, '/' | '!') && !matches!(c, '/' | '!'),
        }
    }

    /// コメントの範囲に、種類を付けます.
    fn comment(&mut self, from: usize, to: usize) {
        let kind = if self.is_doc(from) {
            TokenKind::DocComment
        } else {
            TokenKind::Comment
        };
        self.kinds[from..to].fill(Some(kind));
    }

    /// `//` のコメントを、行の終わり (含まない) まで読みます.
    ///
    /// # Returns
    ///
    /// 次に読む位置.
    fn line_comment(&mut self, from: usize) -> usize {
        let end = (from..self.chars.len())
            .find(|&i| self.chars[i] == '\n')
            .unwrap_or(self.chars.len());
        self.comment(from, end);
        end
    }

    /// `/* */` のコメント (入れ子にできる) を読みます. 閉じなければ、本文の終わりまで.
    fn block_comment(&mut self, from: usize) -> usize {
        let mut depth = 0;
        let mut i = from;
        while i < self.chars.len() {
            if self.chars[i] == '/' && self.at(i + 1) == '*' {
                depth += 1;
                i += 2;
            } else if self.chars[i] == '*' && self.at(i + 1) == '/' {
                depth -= 1;
                i += 2;
                if depth == 0 {
                    break;
                }
            } else {
                i += 1;
            }
        }
        self.comment(from, i);
        i
    }

    /// 文字列 `"..."` (`\` で、次の文字を飛ばす) を読みます. 閉じなければ、本文の終わりまで.
    fn string(&mut self, from: usize) -> usize {
        let mut i = from + 1;
        while i < self.chars.len() {
            match self.chars[i] {
                '\\' => i += 2,
                '"' => {
                    i += 1;
                    break;
                }
                _ => i += 1,
            }
        }
        let end = i.min(self.chars.len());
        self.kinds[from..end].fill(Some(TokenKind::String));
        end
    }

    /// 文字 `'a'` `'\n'` `'\x41'` `'\u{1F600}'` を読みます (rust-mode の `rust--char-literal-rx`). 文字でなければ (ライフタイム)、`'` だけ飛ばす.
    fn char_literal(&mut self, from: usize) -> usize {
        let hex = |i: usize| self.at(i).is_ascii_hexdigit();
        let body_end = if self.at(from + 1) == '\\' {
            match self.at(from + 2) {
                'u' if self.at(from + 3) == '{' => {
                    let digits = (from + 4..from + 10).take_while(|&i| hex(i)).count();
                    (digits >= 1 && self.at(from + 4 + digits) == '}').then_some(from + 5 + digits)
                }
                'x' => (hex(from + 3) && hex(from + 4)).then_some(from + 5),
                '\'' | 'n' | 'r' | 't' | '0' | '"' | '\\' => Some(from + 3),
                _ => None,
            }
        } else if from + 1 < self.chars.len() && self.at(from + 1) != '\'' {
            Some(from + 2)
        } else {
            None
        };
        match body_end {
            Some(end) if self.at(end) == '\'' => {
                self.kinds[from..=end].fill(Some(TokenKind::String));
                end + 1
            }
            _ => from + 1,
        }
    }

    /// 生の文字列 `r#"..."#` を読みます (`r` から、閉じる `"` と同じ数の `#` まで). 閉じなければ、本文の終わりまで.
    /// `r` の後が `#*"` でなければ、何もしない.
    fn raw_string(&mut self, from: usize) -> usize {
        let mut quote = from + 1;
        while self.at(quote) == '#' {
            quote += 1;
        }
        if self.at(quote) != '"' {
            return from + 1;
        }
        let hashes = quote - from - 1;
        let end = (quote + 1..self.chars.len())
            .find(|&i| self.chars[i] == '"' && (1..=hashes).all(|k| self.at(i + k) == '#'))
            .map_or(self.chars.len(), |i| i + 1 + hashes);
        self.kinds[from..end].fill(Some(TokenKind::String));
        end
    }

    /// `Foo::` と `foo::` (後ろに `<` が続かないもの) の名前に、`kind` を付けます.
    fn paths(&mut self, re: &Regex, kind: TokenKind, offsets: &[usize], text: &str) {
        for caps in re.captures_iter(text) {
            let (m, whole) = (caps.get(1).unwrap(), caps.get(0).unwrap());
            // `foo::<T>` の形 (型の指定) は、当てない
            let rest = &text[whole.end()..];
            let after = rest.trim_start_matches([' ', '\t', '\x0c']);
            if !after.starts_with('<') {
                self.paint(
                    offsets[m.start()],
                    offsets[m.end()],
                    Some(kind),
                    Fill::Whole,
                );
            }
        }
    }
}

/// 本文を字句に分けます.
///
/// # Returns
///
/// 範囲の並び (始まりの順. 重ならない. 行をまたがない. 続く同じ種類の範囲は、1 つにする).
pub(super) fn tokenize(text: &str) -> Vec<Span> {
    // 正規表現の位置 (バイト) から、文字の位置へ
    let mut offsets = vec![0; text.len() + 1];
    let mut painter = Painter::new(text);
    for (n, (byte, c)) in text.char_indices().enumerate() {
        offsets[byte..byte + c.len_utf8()].fill(n);
    }
    offsets[text.len()] = painter.chars.len();
    painter.syntax();
    let (upper, lower) = path_regexes();
    for (index, rule) in rules().iter().enumerate() {
        for caps in rule.re.captures_iter(text) {
            if let Some(m) = caps.get(rule.group) {
                painter.paint(
                    offsets[m.start()],
                    offsets[m.end()],
                    Some(rule.kind),
                    rule.fill,
                );
            }
        }
        // `Foo::` と `foo::` は、`'foo` (ライフタイム) の規則の前に当てる
        if index == LIFETIME_BEFORE {
            painter.paths(upper, TokenKind::Type, &offsets, text);
            painter.paths(lower, TokenKind::Constant, &offsets, text);
        }
    }
    painter.spans()
}

/// `Foo::` と `foo::` の規則を当てる位置 (`rules` の中の、型を推論する束縛の規則の番号).
const LIFETIME_BEFORE: usize = 12;
