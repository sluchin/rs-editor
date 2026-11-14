//! TypeScript のシンタックスハイライト. emacs-typescript/typescript-mode (`typescript-mode.el` 0.4 以降) の font-lock と同じ所に, 同じ色を付けます.
//!
//! typescript-mode は, 構文表 (コメントと文字列) と, `typescript-syntax-propertize` (正規表現のリテラル) と,
//! `typescript--font-lock-keywords-4` (正規表現と, 括弧を読む関数の並び) で色を付けます. ここでも同じ形です.
//! 構文を先に読み (`syntax`), 次に規則を上から順に当てて, 文字ごとの種類を決めます (`paint.rs`. 先に付いた色を上書きしない決まりも, Emacs と同じ).
//!
//! 色が付く所 (Emacs の face): キーワードと, `this.`, 修飾子 (`private` など), 組み込みの型 `number` など, `=>`, `${` と `}` (keyword),
//! `true` `null` `undefined` など (constant), `class` `interface` `type` `enum` `extends` `new` の次の名前と, `: Foo` の型 (type),
//! `function` の名前と, `name(` の呼び出しと, `@decorator` (function-name), `const` の名前と, `function` と method の引数 (variable-name),
//! `// tslint:...` (preprocessor).
//!
//! 移していないもの (今は, 色が付かない): JSDoc の `@param` などの色 (typescript-mode は独自の face), アロー関数の引数,
//! `class` の framework (Dojo, ExtJS など) の宣言, `implements` と型引数の細かい形.

use std::sync::OnceLock;

use regex::{Captures, Regex};

/// JSDoc のタグの前に許す文字 (`typescript-jsdoc-before-tag-regexp`).
const JSDOC_BEFORE: &str = r"(?:^[ \t\x0c]*\*+|/\*\*)[ \t\x0c]*";

/// 引数と型の付くタグ.
const JSDOC_PARAM_TAGS: &[&str] = &["argument", "arg", "param", "property", "prop", "typedef"];

/// 型の付くタグ.
const JSDOC_TYPED_TAGS: &[&str] = &[
    "enum",
    "extends",
    "field",
    "id",
    "implements",
    "lends",
    "mods",
    "requires",
    "returns",
    "return",
    "throws",
    "throw",
    "type",
    "yields",
    "yield",
];

/// 値が 1 つ付くタグ.
const JSDOC_ARG_TAGS: &[&str] = &[
    "access",
    "alias",
    "augments",
    "base",
    "borrows",
    "bug",
    "callback",
    "config",
    "default",
    "define",
    "emits",
    "exception",
    "extends",
    "external",
    "fires",
    "function",
    "func",
    "host",
    "kind",
    "listens",
    "memberof",
    "member",
    "method",
    "mixes",
    "module",
    "namespace",
    "name",
    "requires",
    "since",
    "suppress",
    "this",
    "throws",
    "variation",
    "version",
    "var",
];

/// 値の付かないタグ.
const JSDOC_EMPTY_TAGS: &[&str] = &[
    "abstract",
    "addon",
    "async",
    "author",
    "classdesc",
    "class",
    "constants",
    "constant",
    "constructor",
    "constructs",
    "const",
    "copyright",
    "defaultvalue",
    "default",
    "deprecated",
    "description",
    "desc",
    "event",
    "example",
    "exec",
    "exports",
    "export",
    "fileoverview",
    "file",
    "final",
    "function",
    "func",
    "generator",
    "global",
    "hideconstructor",
    "hidden",
    "ignore",
    "implicitcast",
    "inheritdoc",
    "inner",
    "instance",
    "interface",
    "license",
    "method",
    "mixin",
    "noalias",
    "noshadow",
    "notypecheck",
    "override",
    "overview",
    "owner",
    "package",
    "preservetry",
    "preserve",
    "private",
    "protected",
    "public",
    "readonly",
    "static",
    "summary",
    "supported",
    "todo",
    "tutorial",
    "virtual",
];

use super::paint::{Fill, Painter};
use super::{Span, TokenKind};

/// キーワード (typescript-mode の `typescript--keyword-re`).
const KEYWORDS: &[&str] = &[
    "abstract",
    "any",
    "as",
    "async",
    "await",
    "boolean",
    "bigint",
    "break",
    "case",
    "catch",
    "class",
    "const",
    "constructor",
    "continue",
    "debugger",
    "declare",
    "default",
    "delete",
    "do",
    "else",
    "enum",
    "export",
    "extends",
    "extern",
    "false",
    "finally",
    "for",
    "function",
    "from",
    "get",
    "goto",
    "if",
    "implements",
    "import",
    "in",
    "infer",
    "instanceof",
    "interface",
    "keyof",
    "let",
    "module",
    "namespace",
    "never",
    "new",
    "null",
    "number",
    "object",
    "of",
    "override",
    "private",
    "protected",
    "public",
    "readonly",
    "return",
    "satisfies",
    "set",
    "static",
    "string",
    "super",
    "switch",
    "this",
    "throw",
    "true",
    "try",
    "type",
    "typeof",
    "unknown",
    "var",
    "void",
    "while",
];

/// 組み込みの型 (`typescript--basic-type-re`).
const BASIC_TYPES: &[&str] = &[
    "any", "bool", "boolean", "bigint", "never", "number", "string", "unknown", "void",
];

/// 修飾子 (`typescript--access-modifier-re`).
const ACCESS_MODIFIERS: &[&str] = &[
    "private",
    "protected",
    "public",
    "readonly",
    "static",
    "extends",
    "implements",
];

/// 定数 (`typescript--constant-re`).
const CONSTANTS: &[&str] = &[
    "false",
    "null",
    "undefined",
    "Infinity",
    "NaN",
    "true",
    "arguments",
    "this",
];

/// 識別子 (`typescript--name-re`).
const NAME: &str = r"[a-zA-Z_$][\w$]*";

/// 型の名前 (`typescript--type-name-re`. 大文字で始まり, 2 文字以上).
const TYPE_NAME: &str = r"(?:[A-Z][A-Za-z0-9]+\.)*(?:[A-Z][A-Za-z0-9]+)";

/// 空白 (構文表の空白. 改行は含まない).
const SPACE: &str = r"[ \t\x0c]";

/// `\b(a|b|...)\b`.
fn words(list: &[&str]) -> String {
    format!(r"\b({})\b", list.join("|"))
}

/// 正規表現を作ります.
fn re(pattern: &str) -> Regex {
    Regex::new(pattern).expect("正規表現が正しい")
}

/// 使う正規表現.
struct Res {
    this: Regex,
    access: Regex,
    basic: Regex,
    generic: Regex,
    generic_arg: Regex,
    annotation: Regex,
    cast: Regex,
    import: Regex,
    heading1: Regex,
    heading2: Regex,
    constant: Regex,
    keyword: Regex,
    for_each: Regex,
    each: Regex,
    yield_: Regex,
    brace_key: Regex,
    tslint: Regex,
    prototype: Regex,
    class: Regex,
    extends: Regex,
    implements: Regex,
    implements_word: Regex,
    interface: Regex,
    type_: Regex,
    enum_: Regex,
    decl: Regex,
    new: Regex,
    instanceof: Regex,
    function: Regex,
    arrow_block: Regex,
    call: Regex,
    dot_call: Regex,
    decorator: Regex,
    console: Regex,
    arrow: Regex,
    subst: Regex,
    argument: Regex,
    argument_type: Regex,
    dotted_before: Regex,
    control_call: Regex,
    number_before: Regex,
    jsdoc_param: Regex,
    jsdoc_typed: Regex,
    jsdoc_arg: Regex,
    jsdoc_empty: Regex,
    typedoc_link: Regex,
    typedoc_literal: fancy_regex::Regex,
}

/// 正規表現の表.
fn res() -> &'static Res {
    static RES: OnceLock<Res> = OnceLock::new();
    RES.get_or_init(|| {
        let name_group = r"([\w$]+)";
        let decl_name = |keyword: &str| re(&format!(r"\b{keyword}[ \t]+{name_group}"));
        let call = r"([\w$]+)(<.+>)?[ ]*\(".to_string();
        Res {
            this: re(r"(this)\."),
            access: re(&words(ACCESS_MODIFIERS)),
            basic: re(&words(BASIC_TYPES)),
            generic: re(&format!(r"{NAME}{SPACE}*<{SPACE}*[a-zA-Z_$]")),
            generic_arg: re(&format!(r"({NAME})(?:{SPACE}*>[^<]*)?")),
            annotation: re(&format!(
                r"(?::|=>){SPACE}(?:{SPACE}*({NAME}){SPACE}*(is){SPACE}*)?({TYPE_NAME})(?:<{TYPE_NAME}>)?(?:\[\])?(?:[,;])?{SPACE}*\{{?"
            )),
            cast: re(&format!("<({TYPE_NAME})>")),
            import: re(r"\bimport\b"),
            heading1: re(&format!(
                r"(?m)^{SPACE}*(?:declare|(?:export(?:{SPACE}+default)?))?{SPACE}*function{SPACE}+({NAME})"
            )),
            heading2: re(&format!(r"(?m)^{SPACE}*({NAME}){SPACE}*:{SPACE}*function\b")),
            constant: re(&words(CONSTANTS)),
            keyword: re(&words(KEYWORDS)),
            for_each: re(r"\bfor\b"),
            each: re(&format!(r"{SPACE}+(each)\b")),
            yield_: re(r"\byield(?:\*|\b)"),
            brace_key: re(&format!(r"\b({})\b:", KEYWORDS.join("|"))),
            tslint: re(r"(?m)(?://|/\*)[ \t\x0c]*(tslint:.*?)(?:\*/|$)"),
            prototype: re(r"\.(prototype)\b"),
            class: decl_name("class"),
            extends: decl_name("extends"),
            implements: re(r"\bimplements[ \t]+"),
            implements_word: re(r"\b[\p{L}\p{N}]+"),
            interface: decl_name("interface"),
            type_: decl_name("type"),
            enum_: decl_name("enum"),
            decl: re(&format!(r"\b(?:const|var|let)\b|{}", words(BASIC_TYPES))),
            new: re(&format!(r"\bnew{SPACE}+({NAME}(?:\.{NAME})*)")),
            instanceof: re(&format!(r"\binstanceof{SPACE}+({NAME}(?:\.{NAME})*)")),
            function: re(&format!(
                r"(?m)\bfunction\b(?:{SPACE}+{NAME})?{SPACE}*(?:<.*>)?{SPACE}*\({SPACE}*(?:$|[a-zA-Z_$])"
            )),
            arrow_block: re(r"(=>)[ \t\x0c]*\{"),
            call: re(&call),
            dot_call: re(&format!(r"\.{SPACE}*{call}")),
            decorator: re(&format!("(@{NAME})")),
            console: re(r"\b(console)\b"),
            arrow: re(r"(=>)"),
            subst: re(r"(?m)(?:^|[^\\])(\$\{)(.+?)(\})"),
            argument: re(&format!(
                r"(?m)({NAME}\??){SPACE}*(,|:|\)|$|{SPACE}*(?://|/\*).*$)"
            )),
            dotted_before: re(&format!(r"{NAME}(?:\.{NAME})*\z")),
            control_call: re(r"^(?:switch|if|while|until|for)\b[ \t\x0c\n]*\("),
            number_before: re(
                r"(?:NaN|-?(?:0[Bb][01]+|0[Oo][0-7]+|0[Xx][0-9a-fA-F]+|Infinity|(?:\d*\.\d+|\d+)(?:[Ee][+-]?\d+)?))\z",
            ),
            jsdoc_param: re(&format!(
                r"(?m){JSDOC_BEFORE}(@(?:{})){SPACE}*(\{{[^}}]+\}})?{SPACE}*\[?([\p{{L}}\p{{N}}_$.]+)?\]?\b",
                JSDOC_PARAM_TAGS.join("|")
            )),
            jsdoc_typed: re(&format!(
                r"(?m){JSDOC_BEFORE}(@(?:{})){SPACE}*(\{{[^}}]+\}})?",
                JSDOC_TYPED_TAGS.join("|")
            )),
            jsdoc_arg: re(&format!(
                r"(?m){JSDOC_BEFORE}(@(?:{})){SPACE}+([^ \t]+)",
                JSDOC_ARG_TAGS.join("|")
            )),
            jsdoc_empty: re(&format!(
                r"(?m){JSDOC_BEFORE}(@(?:{})){SPACE}*",
                JSDOC_EMPTY_TAGS.join("|")
            )),
            typedoc_link: re(r"\[\[.*?\]\]"),
            typedoc_literal: fancy_regex::Regex::new(r"(`+).*?\1").expect("正規表現が正しい"),
            argument_type: re(&format!(
                r"(?m),$|\(|\)(?:{SPACE}*=>{SPACE}*)?|,{SPACE}*{NAME}\??(?::|\))"
            )),
        }
    })
}

/// 文字の構文上の種類.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Class {
    /// コード.
    Code,
    /// コメント.
    Comment,
    /// `'` と `"` の文字列.
    Str,
    /// `` ` `` の文字列 (テンプレート).
    Template,
    /// 正規表現のリテラル.
    Regex,
}

/// 正規表現の括弧 1 つの色 (括弧の番号, 種類 (`None` は, 色を消す), 色の付け方).
type Group = (usize, Option<TokenKind>, Fill);

/// 本文と, 文字ごとの種類と, 構文.
struct Ts<'a> {
    /// 本文.
    text: &'a str,
    /// 文字ごとの種類.
    p: Painter,
    /// 文字ごとの構文上の種類.
    class: Vec<Class>,
    /// 文字を含む, コメントか文字列の終わり (コード, は, その文字の次).
    run_end: Vec<usize>,
}

/// 語の文字か (構文表の語. `_` と `$` は, 記号).
fn is_word(c: char) -> bool {
    c.is_alphanumeric()
}

/// 名前の文字か (語と記号).
fn is_name_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '$'
}

impl Ts<'_> {
    /// 文字数.
    fn len(&self) -> usize {
        self.p.chars.len()
    }

    /// `pos` を含む行の終わり (改行の位置).
    fn line_end(&self, pos: usize) -> usize {
        (pos..self.len())
            .find(|&i| self.p.chars[i] == '\n')
            .unwrap_or(self.len())
    }

    /// 構文を読んで, コメントと文字列と正規表現のリテラルに, 種類を付けます.
    fn syntax(&mut self) {
        let n = self.len();
        let mut i = 0;
        if self.p.at(0) == '#' && self.p.at(1) == '!' {
            i = self.mark(0, self.line_end(0), Class::Comment);
        }
        while i < n {
            let c = self.p.chars[i];
            i = if c == '/' && self.p.at(i + 1) == '/' {
                self.mark(i, self.line_end(i), Class::Comment)
            } else if c == '/' && self.p.at(i + 1) == '*' {
                let end = (i + 2..n)
                    .find(|&k| self.p.chars[k] == '*' && self.p.at(k + 1) == '/')
                    .map_or(n, |k| k + 2);
                self.mark(i, end, Class::Comment)
            } else if matches!(c, '\'' | '"' | '`') {
                let mut k = i + 1;
                while k < n {
                    match self.p.chars[k] {
                        '\\' => k += 2,
                        d if d == c => {
                            k += 1;
                            break;
                        }
                        _ => k += 1,
                    }
                }
                let class = if c == '`' {
                    Class::Template
                } else {
                    Class::Str
                };
                self.mark(i, k.min(n), class)
            } else if c == '/' && self.regex_starts(i) {
                let end = self.regex_end(i);
                self.mark(i, end, Class::Regex)
            } else {
                i + 1
            };
        }
    }

    /// `from` から `to` の前までを, 種類 `class` にします.
    ///
    /// # Returns
    ///
    /// `to` (次に読む位置).
    fn mark(&mut self, from: usize, to: usize, class: Class) -> usize {
        self.class[from..to].fill(class);
        self.run_end[from..to].fill(to);
        let kind = match class {
            Class::Comment => TokenKind::Comment,
            _ => TokenKind::String,
        };
        self.p.kinds[from..to].fill(Some(kind));
        to
    }

    /// `i` の `/` が, 正規表現のリテラルの始まりか (`typescript-syntax-propagate` の規則).
    fn regex_starts(&self, i: usize) -> bool {
        let next = self.p.at(i + 1);
        if i + 1 >= self.len() || next == '/' || next == '*' {
            return false;
        }
        let chars = &self.p.chars;
        let mut k = i;
        while k > 0 && matches!(chars[k - 1], ' ' | '\t') {
            k -= 1;
        }
        let at_line_start = k == 0 || chars[k - 1] == '\n';
        if !at_line_start && "=([{,:;|&!".contains(chars[k - 1]) {
            return true;
        }
        if at_line_start {
            // 行の始め (空白があれば, 前のコードの終わりが, 式の始まりのときだけ)
            if k == i {
                return true;
            }
            return self.after_operator(k);
        }
        // `return` の後
        let word_end = k;
        let mut start = word_end;
        while start > 0 && is_name_char(chars[start - 1]) {
            start -= 1;
        }
        chars[start..word_end].iter().collect::<String>() == "return"
    }

    /// `pos` の前の, 空白とコメントを除いた文字が, `=({[,:;` か, 本文の始め.
    fn after_operator(&self, pos: usize) -> bool {
        let mut k = pos;
        while k > 0 {
            let c = self.p.chars[k - 1];
            if c.is_whitespace() {
                k -= 1;
            } else if self.class[k - 1] == Class::Comment {
                while k > 0 && self.class[k - 1] == Class::Comment {
                    k -= 1;
                }
            } else {
                return "=({[,:;".contains(c);
            }
        }
        true
    }

    /// 正規表現のリテラル (始まりの `/` が `i`) の終わり.
    fn regex_end(&self, i: usize) -> usize {
        let n = self.len();
        let chars = &self.p.chars;
        let mut j = i + 1;
        while j < n {
            let c = chars[j];
            if c == '[' {
                let mut k = j + 1;
                let mut closed = false;
                while k < n {
                    match chars[k] {
                        ']' => {
                            closed = true;
                            break;
                        }
                        '\\' if k + 1 < n && chars[k + 1] != '\n' => k += 2,
                        '\\' => break,
                        _ => k += 1,
                    }
                }
                if !closed {
                    break;
                }
                j = k + 1;
            } else if c == '\\' {
                if j + 1 < n && chars[j + 1] != '\n' {
                    j += 2;
                } else {
                    break;
                }
            } else if c == '/' {
                return j + 1;
            } else {
                j += 1;
            }
        }
        // 閉じない (次の, 正規表現の始まりの `/` までが, 文字列. 他の `/` には, 文字列の終わりの印が付かない)
        (j..n)
            .find(|&k| chars[k] == '/' && self.regex_starts(k))
            .map_or(n, |k| k + 1)
    }

    /// 正規表現で見つけた範囲 (バイト) を, 文字の範囲にします.
    fn span_of(&self, m: regex::Match<'_>) -> (usize, usize) {
        (self.p.char_of[m.start()], self.p.char_of[m.end()])
    }

    /// 規則 1 つを, 本文の全体に当てます.
    ///
    /// # Arguments
    ///
    /// * `groups` - (括弧の番号, 種類 (`None` は, 色を消す), 色の付け方).
    fn apply(&mut self, re: &Regex, groups: &[Group]) {
        for caps in re.captures_iter(self.text) {
            self.paint_groups(&caps, groups);
        }
    }

    /// 1 回の一致の, 括弧ごとの色を付けます.
    fn paint_groups(&mut self, caps: &Captures<'_>, groups: &[Group]) {
        for &(group, kind, fill) in groups {
            if let Some(m) = caps.get(group) {
                let (from, to) = self.span_of(m);
                self.p.paint(from, to, kind, fill);
            }
        }
    }

    /// 空白とコメントを, `limit` まで読み飛ばします (Emacs の `forward-comment`).
    fn forward_comment(&self, mut pos: usize, limit: usize) -> usize {
        while pos < limit
            && (self.p.chars[pos].is_whitespace() || self.class[pos] == Class::Comment)
        {
            pos = if self.class[pos] == Class::Comment {
                self.run_end[pos].min(limit)
            } else {
                pos + 1
            };
        }
        pos
    }

    /// 次の式の終わり (Emacs の `forward-sexp`). 閉じ括弧に当たるか, 括弧が閉じなければ `None`.
    /// 式が無ければ, `limit`.
    fn forward_sexp(&self, mut pos: usize, limit: usize) -> Option<usize> {
        while pos < limit {
            let c = self.p.chars[pos];
            match self.class[pos] {
                Class::Comment => pos = self.run_end[pos],
                Class::Code if is_name_char(c) => {
                    while pos < limit
                        && self.class[pos] == Class::Code
                        && is_name_char(self.p.chars[pos])
                    {
                        pos += 1;
                    }
                    return Some(pos);
                }
                Class::Code if matches!(c, '(' | '[' | '{') => return self.group_end(pos, limit),
                Class::Code if matches!(c, ')' | ']' | '}') => return None,
                Class::Code => pos += 1,
                _ => return Some(self.run_end[pos].min(limit)),
            }
        }
        Some(limit)
    }

    /// 開き括弧 `open` に対応する閉じ括弧の次.
    fn group_end(&self, open: usize, limit: usize) -> Option<usize> {
        let mut stack = Vec::new();
        let mut pos = open;
        while pos < limit {
            if self.class[pos] == Class::Code {
                match self.p.chars[pos] {
                    '(' => stack.push(')'),
                    '[' => stack.push(']'),
                    '{' => stack.push('}'),
                    c @ (')' | ']' | '}') => {
                        if stack.pop() != Some(c) {
                            return None;
                        }
                        if stack.is_empty() {
                            return Some(pos + 1);
                        }
                    }
                    _ => {}
                }
                pos += 1;
            } else {
                pos = self.run_end[pos];
            }
        }
        None
    }

    /// 括弧の中の `pos` から, それを囲む閉じ括弧の次 (Emacs の `up-list`). 無ければ `None`.
    fn up_list(&self, mut pos: usize) -> Option<usize> {
        let mut depth = 0usize;
        while pos < self.len() {
            if self.class[pos] == Class::Code {
                match self.p.chars[pos] {
                    '(' | '[' | '{' => depth += 1,
                    ')' | ']' | '}' => {
                        if depth == 0 {
                            return Some(pos + 1);
                        }
                        depth -= 1;
                    }
                    _ => {}
                }
                pos += 1;
            } else {
                pos = self.run_end[pos];
            }
        }
        None
    }

    /// `pos` から `limit` の手前までに, 名前があればその終わり.
    fn name_at(&self, pos: usize, limit: usize) -> Option<usize> {
        let c = *self.p.chars.get(pos).filter(|_| pos < limit)?;
        if !(c.is_ascii_alphabetic() || c == '_' || c == '$') {
            return None;
        }
        let mut end = pos + 1;
        while end < limit && is_name_char(self.p.chars[end]) {
            end += 1;
        }
        Some(end)
    }

    /// 分割代入 `[a, b]` と `{a: b}` を読み飛ばします (`typescript--forward-destructuring-spec`).
    ///
    /// # Arguments
    ///
    /// * `paint` - 名前に, 変数の色を付けるか.
    ///
    /// # Returns
    ///
    /// 閉じる括弧まで読めたか. 読んだ所まで, `pos` を進める.
    fn destructuring(&mut self, pos: &mut usize, limit: usize, paint: bool) -> bool {
        match self.p.at(*pos) {
            '[' if *pos < limit => {
                *pos += 1;
                loop {
                    *pos = self.forward_comment(*pos, limit);
                    let c = if *pos < limit { self.p.at(*pos) } else { '\0' };
                    if matches!(c, '[' | '{') {
                        if !self.destructuring(pos, limit, paint) {
                            break;
                        }
                    } else if c == ',' {
                        *pos += 1;
                    } else if let Some(end) = self.name_at(*pos, limit) {
                        self.paint_variable(*pos, end, paint);
                        *pos = end;
                    } else {
                        break;
                    }
                }
                if *pos < limit && self.p.at(*pos) == ']' {
                    *pos += 1;
                    return true;
                }
                false
            }
            '{' if *pos < limit => {
                *pos += 1;
                *pos = self.forward_comment(*pos, limit);
                while let Some(end) = self
                    .name_at(*pos, limit)
                    .filter(|&end| end < limit && self.p.at(end) == ':')
                {
                    *pos = self.forward_comment(end + 1, limit);
                    let c = if *pos < limit { self.p.at(*pos) } else { '\0' };
                    let ok = if matches!(c, '[' | '{') {
                        self.destructuring(pos, limit, paint)
                    } else if let Some(end) = self.name_at(*pos, limit) {
                        self.paint_variable(*pos, end, paint);
                        *pos = end;
                        true
                    } else {
                        false
                    };
                    if !ok {
                        break;
                    }
                    *pos = self.forward_comment(*pos, limit);
                    if *pos < limit && self.p.at(*pos) == ',' {
                        *pos = self.forward_comment(*pos + 1, limit);
                    } else {
                        break;
                    }
                }
                if *pos < limit && self.p.at(*pos) == '}' {
                    *pos += 1;
                    return true;
                }
                false
            }
            _ => false,
        }
    }

    /// 変数の名前に, 色を付けます (`paint` が真のとき).
    fn paint_variable(&mut self, from: usize, to: usize, paint: bool) {
        if paint {
            self.p
                .paint(from, to, Some(TokenKind::VariableName), Fill::Whole);
        }
    }

    /// 式 1 つを, 行の終わりか, `, ; ]) }` まで読み飛ばします (`typescript--forward-expression`. 行をまたぐ式は読まない).
    ///
    /// # Returns
    ///
    /// 読めたか (括弧が閉じなければ偽).
    fn forward_expression(&self, pos: &mut usize, limit: usize) -> bool {
        *pos = self.forward_comment(*pos, limit);
        loop {
            if *pos >= limit || self.p.chars[*pos] == '\n' {
                return true;
            }
            *pos = self.forward_comment(*pos, limit);
            if *pos < limit && ",;])}".contains(self.p.chars[*pos]) {
                return true;
            }
            match self.forward_sexp(*pos, limit) {
                Some(end) => *pos = end,
                None => return false,
            }
        }
    }

    /// `const` `let` `var` の次の, 変数の名前 (`typescript--variable-decl-matcher`. 行の終わりまでを読む).
    fn variable_declaration(&mut self, start: usize) {
        let limit = self.line_end(start);
        let mut pos = self.forward_comment(start, limit);
        let mut first = true;
        loop {
            if !first {
                if pos < limit && self.p.at(pos) == ',' {
                    pos = self.forward_comment(pos + 1, limit);
                } else {
                    return;
                }
            }
            if let Some(end) = self.name_at(pos, limit) {
                self.paint_variable(pos, end, true);
                pos = end;
            } else {
                let mut probe = pos;
                if !self.destructuring(&mut probe, limit, false) {
                    return;
                }
                self.destructuring(&mut pos, limit, true);
            }
            pos = self.forward_comment(pos, limit);
            if pos < limit && self.p.at(pos) == '=' {
                pos += 1;
                if !self.forward_expression(&mut pos, limit) {
                    return;
                }
                pos = self.forward_comment(pos, limit);
            }
            first = false;
        }
    }

    /// 引数の並びの変数の名前 (`typescript--function-argument-matcher`). `start` から `limit` まで.
    fn function_arguments(&mut self, start: usize, limit: usize) {
        let res = res();
        let hay = &self.text[..self.p.byte_of[limit]];
        let mut at = self.p.byte_of[start];
        while let Some(caps) = res.argument.captures_at(hay, at) {
            let (name, ender) = (caps.get(1).unwrap(), caps.get(2).unwrap());
            let (from, to) = self.span_of(name);
            self.p
                .paint(from, to, Some(TokenKind::VariableName), Fill::Over);
            at = caps.get(0).unwrap().end();
            if ender.as_str() != ":" {
                continue;
            }
            // 型の指定を, 次の引数か, 引数の終わりまで読み飛ばす
            while let Some(m) = res.argument_type.find_at(hay, at) {
                if m.as_str().contains("=>") {
                    at = m.end();
                } else {
                    at = m.start();
                    break;
                }
            }
        }
    }

    /// `name(` の `(` の直後 `after` が, メソッドの定義の引数か (`typescript--font-lock-keywords-3` の最後の規則).
    ///
    /// # Returns
    ///
    /// 引数の並びの終わり `close` (`)` の次) が, メソッドの定義なら真.
    fn method_definition(&self, after: usize, close: usize) -> bool {
        let n = self.len();
        let mut k = close;
        while k < n && self.p.chars[k].is_whitespace() {
            k += 1;
        }
        let brace = self.p.at(k) == '{';
        let colon = self.p.at(k) == ':' && self.not_ternary(after - 1);
        if !(brace || colon) {
            return false;
        }
        // 名前の語がキーワードなら, 定義ではない (`if (x) {` など)
        let mut end = after - 1;
        while end > 0 && !is_word(self.p.chars[end - 1]) {
            end -= 1;
        }
        let mut word = end;
        while word > 0 && is_word(self.p.chars[word - 1]) {
            word -= 1;
        }
        end == 0 || self.p.kinds[word] != Some(TokenKind::Keyword)
    }

    /// `(` の位置 `open` の前の式の, さらに前の文字が, `?` でないか (三項演算子ではないか).
    fn not_ternary(&self, open: usize) -> bool {
        // 名前 (語と記号の並び) の始めまで戻る
        let mut k = open;
        while k > 0 && self.p.chars[k - 1].is_whitespace() {
            k -= 1;
        }
        while k > 0 && is_name_char(self.p.chars[k - 1]) {
            k -= 1;
        }
        while k > 0 && matches!(self.p.chars[k - 1], ' ' | '\t' | '\n' | '\x0c') {
            k -= 1;
        }
        !(k > 0 && self.p.chars[k - 1] == '?')
    }

    /// 型の引数の規則 (`NAME<T` の後ろの, 行の終わりまでの名前を, 型にする).
    fn generics(&mut self) {
        let res = res();
        let mut at = 0;
        while let Some(m) = res.generic.find_at(self.text, at) {
            let end = self.p.char_of[m.end()];
            let eol = self.line_end(end);
            let hay = &self.text[..self.p.byte_of[eol]];
            let mut b = self.p.byte_of[end - 1];
            while let Some(caps) = res.generic_arg.captures_at(hay, b) {
                let whole = caps.get(0).unwrap();
                let (from, to) = self.span_of(caps.get(1).unwrap());
                self.p.paint(from, to, Some(TokenKind::Type), Fill::Whole);
                b = whole.end();
            }
            at = self.p.byte_of[eol];
        }
    }

    /// `implements A, B` の名前を, 型にする.
    fn implements(&mut self) {
        let res = res();
        for m in res.implements.find_iter(self.text) {
            let eol = self.line_end(self.p.char_of[m.end()]);
            let hay = &self.text[..self.p.byte_of[eol]];
            let mut b = m.end();
            while let Some(w) = res.implements_word.find_at(hay, b) {
                let (from, to) = self.span_of(w);
                self.p.paint(from, to, Some(TokenKind::Type), Fill::Whole);
                b = w.end();
            }
        }
    }

    /// `{ class: 1 }` のように, キーになるキーワードの色を消す.
    fn brace_keys(&mut self) {
        let res = res();
        for (open, c) in self.p.chars.clone().into_iter().enumerate() {
            if c != '{' {
                continue;
            }
            let limit = self
                .up_list(open + 1)
                .unwrap_or_else(|| self.line_end(open + 1));
            let hay = &self.text[..self.p.byte_of[limit]];
            let mut b = self.p.byte_of[open + 1];
            while let Some(caps) = res.brace_key.captures_at(hay, b) {
                let (from, to) = self.span_of(caps.get(1).unwrap());
                self.p.paint(from, to, None, Fill::Over);
                b = caps.get(0).unwrap().end();
            }
        }
    }

    /// `pos` の前の, 空白とコメントを読み飛ばした位置 (`typescript--backward-syntactic-ws`).
    fn back_ws(&self, mut pos: usize) -> usize {
        while pos > 0 {
            if self.p.chars[pos - 1].is_whitespace() {
                pos -= 1;
            } else if self.class[pos - 1] == Class::Comment {
                while pos > 0 && self.class[pos - 1] == Class::Comment {
                    pos -= 1;
                }
            } else {
                break;
            }
        }
        pos
    }

    /// `pos` の前の式の始め (Emacs の `backward-sexp`). 括弧が合わなければ `None`.
    fn backward_sexp(&self, mut pos: usize) -> Option<usize> {
        while pos > 0 {
            let c = self.p.chars[pos - 1];
            match self.class[pos - 1] {
                Class::Comment => {
                    while pos > 0 && self.class[pos - 1] == Class::Comment {
                        pos -= 1;
                    }
                }
                Class::Code if is_name_char(c) => {
                    while pos > 0
                        && self.class[pos - 1] == Class::Code
                        && is_name_char(self.p.chars[pos - 1])
                    {
                        pos -= 1;
                    }
                    return Some(pos);
                }
                Class::Code if matches!(c, ')' | ']' | '}') => {
                    let mut depth = 0usize;
                    let mut k = pos;
                    while k > 0 {
                        k -= 1;
                        if self.class[k] != Class::Code {
                            continue;
                        }
                        match self.p.chars[k] {
                            ')' | ']' | '}' => depth += 1,
                            '(' | '[' | '{' => {
                                depth -= 1;
                                if depth == 0 {
                                    return Some(k);
                                }
                            }
                            _ => {}
                        }
                    }
                    return None;
                }
                Class::Code if matches!(c, '(' | '[' | '{') => return None,
                Class::Code => pos -= 1,
                _ => {
                    let mut k = pos - 1;
                    while k > 0
                        && self.class[k - 1] == self.class[k]
                        && self.run_end[k - 1] == self.run_end[k]
                    {
                        k -= 1;
                    }
                    return Some(k);
                }
            }
        }
        Some(0)
    }

    /// ジェネリクスの `>` の位置 `close` から戻って, 対応する `<` の位置 (`typescript--backward-over-generic-parameter-list`).
    fn back_over_generic(&self, close: usize) -> Option<usize> {
        let mut depth = 1usize;
        let mut k = close;
        while depth > 0 {
            k = (0..k)
                .rev()
                .find(|&i| matches!(self.p.chars[i], '<' | '>'))?;
            if self.p.chars[k] == '>' {
                if k == 0 || self.p.chars[k - 1] != '=' {
                    depth += 1;
                }
            } else {
                depth -= 1;
            }
        }
        Some(k)
    }

    /// 関数の本体の `{` (位置 `brace`) から戻って, 引数の並びの終わり (`)` の次) を探す (`typescript--backward-to-parameter-list`).
    fn back_to_parameters(&self, brace: usize) -> Option<usize> {
        let res = res();
        // 戻り値の型の指定がある形
        let mut pos = brace;
        let found = loop {
            pos = self.back_ws(pos);
            if pos > 0 && self.p.chars[pos - 1] == ':' {
                pos -= 1;
                while pos > 0 && matches!(self.p.chars[pos - 1], ' ' | '\t' | '\x0c') {
                    pos -= 1;
                }
                if pos > 0 && self.p.chars[pos - 1] == ')' {
                    break Some(pos);
                }
            }
            let before = if pos > 0 { self.p.chars[pos - 1] } else { '\0' };
            let two: String = self.p.chars[pos.saturating_sub(2)..pos].iter().collect();
            if pos >= 2 && (two == "=>" || two == "is") {
                pos -= 2;
            } else if before == '>' {
                pos -= 1;
                if let Some(open) = self.back_over_generic(pos) {
                    pos = open;
                }
            } else if before == '|' || before == '.' {
                pos -= 1;
            } else if matches!(before, ')' | '}' | '"' | ']')
                || (res
                    .dotted_before
                    .is_match(&self.text[..self.p.byte_of[pos]])
                    && !self.control_before(pos))
            {
                match self.backward_sexp(pos) {
                    Some(start) => pos = start,
                    None => break None,
                }
            } else {
                // 数値の指定 (直前の空白まで戻って探す)
                let from = (0..pos)
                    .rev()
                    .find(|&i| self.p.chars[i].is_whitespace())
                    .map_or(0, |i| i + 1);
                let slice = &self.text[self.p.byte_of[from]..self.p.byte_of[pos]];
                match res.number_before.find(slice) {
                    Some(m) => pos = self.p.char_of[self.p.byte_of[from] + m.start()],
                    None => break None,
                }
            }
        };
        found.or_else(|| {
            // 戻り値の型の指定が無い形
            let pos = self.back_ws(brace);
            (pos > 0 && self.p.chars[pos - 1] == ')').then_some(pos)
        })
    }

    /// `pos` の前の語が, `if (` などの制御構文か.
    fn control_before(&self, pos: usize) -> bool {
        let mut end = pos;
        while end > 0 && !is_word(self.p.chars[end - 1]) {
            end -= 1;
        }
        let mut word = end;
        while word > 0 && is_word(self.p.chars[word - 1]) {
            word -= 1;
        }
        // `backward-word` は, 語の始めへ. そこから前向きに照合する
        let hay = &self.text[self.p.byte_of[word]..];
        res().control_call.is_match(hay)
    }

    /// アロー関数の引数 (`=> {` の前の `(...)`).
    fn arrow_arguments(&mut self) {
        let res = res();
        for caps in res.arrow_block.captures_iter(self.text) {
            let brace = self.p.char_of[caps.get(0).unwrap().end()] - 1;
            let end = self.back_to_parameters(brace).unwrap_or(brace);
            if let Some(start) = self.backward_sexp(end) {
                if end > start {
                    self.function_arguments(start, end);
                }
            }
        }
    }

    /// `function` の引数と, メソッドの定義の引数.
    fn arguments(&mut self) {
        let res = res();
        for m in res.function.find_iter(self.text) {
            let end = self.p.char_of[m.end()];
            let limit = self.up_list(end).unwrap_or(end);
            self.function_arguments(end.saturating_sub(1), limit);
        }
        // アロー関数 (`=> {`) の `=>` は, キーワード. 引数は, そこから戻って探す
        self.apply(
            &res.arrow_block,
            &[(1, Some(TokenKind::Keyword), Fill::Whole)],
        );
        self.arrow_arguments();
        for caps in res.call.captures_iter(self.text) {
            let end = self.p.char_of[caps.get(0).unwrap().end()];
            if self.class[end - 1] != Class::Code {
                continue;
            }
            // メソッドの定義なら引数の並びを読む. そうでなければ, `)` の後ろから行の終わりまでを読む (Emacs は, 探す範囲の限界を行の終わりにする)
            let close = self.up_list(end);
            match close.filter(|&close| self.method_definition(end, close)) {
                Some(close) => self.function_arguments(end, close),
                None => {
                    let start = close.unwrap_or(end);
                    self.function_arguments(start, self.line_end(start));
                }
            }
        }
    }
}

/// 本文を字句に分けます.
///
/// # Returns
///
/// 範囲の並び (始まりの順. 重ならない. 行をまたがない. 続く同じ種類の範囲は, 1 つにする).
pub(super) fn tokenize(text: &str) -> Vec<Span> {
    let p = Painter::new(text);
    let n = p.chars.len();
    let mut ts = Ts {
        text,
        p,
        class: vec![Class::Code; n],
        run_end: (1..=n).collect(),
    };
    ts.syntax();
    ts.rules();
    ts.p.spans()
}

impl Ts<'_> {
    /// 規則を, `typescript--font-lock-keywords-4` の並びで当てます.
    fn rules(&mut self) {
        use Fill::{Over, Whole};
        use TokenKind::{Constant, FunctionName, Keyword, Type, VariableName};
        let res = res();
        // 上書きしない規則 (`nil`)
        let kw = [(1, Some(Keyword), Whole)];
        let ty = [(1, Some(Type), Whole)];
        self.apply(&res.this, &kw);
        self.apply(&res.access, &kw);
        self.apply(&res.basic, &kw);
        self.generics();
        self.apply(
            &res.annotation,
            &[
                (1, Some(VariableName), Whole),
                (2, Some(Keyword), Whole),
                (3, Some(Type), Whole),
            ],
        );
        self.apply(&res.cast, &ty);
        // レベル 1 と 2
        self.apply(&res.import, &[(0, Some(Keyword), Whole)]);
        self.apply(&res.heading1, &[(1, Some(FunctionName), Whole)]);
        self.apply(&res.heading2, &[(1, Some(FunctionName), Whole)]);
        self.apply(&res.constant, &[(0, Some(Constant), Whole)]);
        self.apply(&res.basic, &[(0, Some(Type), Whole)]);
        self.apply(&res.keyword, &kw);
        self.for_each();
        self.apply(&res.yield_, &[(0, Some(Keyword), Whole)]);
        // レベル 3
        self.brace_keys();
        self.jsdoc();
        self.tslint();
        self.apply(&res.prototype, &[(1, Some(Constant), Whole)]);
        self.apply(&res.class, &ty);
        self.apply(&res.extends, &ty);
        self.implements();
        self.apply(&res.interface, &ty);
        self.apply(&res.type_, &ty);
        self.apply(&res.enum_, &ty);
        self.declarations();
        self.apply(&res.new, &ty);
        self.apply(&res.instanceof, &ty);
        self.arguments();
        // レベル 4
        self.apply(&res.decorator, &[(1, Some(FunctionName), Whole)]);
        self.apply(&res.call, &[(1, Some(FunctionName), Whole)]);
        self.apply(&res.dot_call, &[(1, Some(FunctionName), Over)]);
        self.apply(&res.console, &ty);
        self.apply(&res.arrow, &kw);
        self.substitutions();
    }

    /// `for each` の `each`.
    fn for_each(&mut self) {
        let res = res();
        for m in res.for_each.find_iter(self.text) {
            let eol = self.line_end(self.p.char_of[m.end()]);
            let hay = &self.text[..self.p.byte_of[eol]];
            if let Some(caps) = res.each.captures_at(hay, m.end()) {
                let (from, to) = self.span_of(caps.get(1).unwrap());
                self.p
                    .paint(from, to, Some(TokenKind::Keyword), Fill::Whole);
            }
        }
    }

    /// 位置 `pos` (文字の間) が, `/**` で始まるコメントの中か.
    fn in_documentation(&self, pos: usize) -> bool {
        if pos == 0 || self.class[pos - 1] != Class::Comment {
            return false;
        }
        let end = self.run_end[pos - 1];
        let mut start = pos - 1;
        while start > 0 && self.class[start - 1] == Class::Comment && self.run_end[start - 1] == end
        {
            start -= 1;
        }
        let closed = end >= start + 4 && self.p.at(end - 2) == '*' && self.p.at(end - 1) == '/';
        (pos < end || !closed)
            && self.p.at(start) == '/'
            && self.p.at(start + 1) == '*'
            && self.p.at(start + 2) == '*'
    }

    /// JSDoc と TypeDoc の印 (`/**` のコメントの中だけ).
    fn jsdoc(&mut self) {
        use TokenKind::{DocTag, DocType, DocValue};
        let res = res();
        let list: [(&Regex, &[Group]); 4] = [
            (
                &res.jsdoc_param,
                &[
                    (1, Some(DocTag), Fill::Over),
                    (2, Some(DocType), Fill::Over),
                    (3, Some(DocValue), Fill::Over),
                ],
            ),
            (
                &res.jsdoc_typed,
                &[
                    (1, Some(DocTag), Fill::Over),
                    (2, Some(DocType), Fill::Over),
                ],
            ),
            (
                &res.jsdoc_arg,
                &[
                    (1, Some(DocTag), Fill::Over),
                    (2, Some(DocValue), Fill::Over),
                ],
            ),
            (&res.jsdoc_empty, &[(1, Some(DocTag), Fill::Over)]),
        ];
        for (re, groups) in list {
            for caps in re.captures_iter(self.text) {
                if self.in_documentation(self.p.char_of[caps.get(0).unwrap().end()]) {
                    self.paint_groups(&caps, groups);
                }
            }
        }
        for m in res.typedoc_link.find_iter(self.text) {
            if self.in_documentation(self.p.char_of[m.end()]) {
                let (from, to) = self.span_of(m);
                self.p.paint(from, to, Some(DocValue), Fill::Over);
            }
        }
        for m in res.typedoc_literal.find_iter(self.text).flatten() {
            let (from, to) = (self.p.char_of[m.start()], self.p.char_of[m.end()]);
            if self.in_documentation(to) {
                self.p.paint(from, to, Some(DocValue), Fill::Over);
            }
        }
    }

    /// コメントの中の `tslint:` の指示.
    fn tslint(&mut self) {
        let res = res();
        for caps in res.tslint.captures_iter(self.text) {
            let m = caps.get(1).unwrap();
            let (from, to) = self.span_of(m);
            if self.class[from] == Class::Comment {
                self.p
                    .paint(from, to, Some(TokenKind::Preprocessor), Fill::Over);
            }
        }
    }

    /// `const` `let` `var` の変数の名前.
    fn declarations(&mut self) {
        let res = res();
        let ends: Vec<usize> = res
            .decl
            .find_iter(self.text)
            .filter(|m| {
                let text = m.as_str();
                matches!(text, "const" | "var" | "let")
            })
            .map(|m| self.p.char_of[m.end()])
            .collect();
        for end in ends {
            self.variable_declaration(end);
        }
    }

    /// テンプレートの `${...}`.
    fn substitutions(&mut self) {
        use TokenKind::Keyword;
        let res = res();
        for caps in res.subst.captures_iter(self.text) {
            let whole = caps.get(0).unwrap();
            let end = self.p.char_of[whole.end()];
            if self.class[end - 1] != Class::Template {
                continue;
            }
            self.paint_groups(
                &caps,
                &[
                    (1, Some(Keyword), Fill::Over),
                    (2, None, Fill::Over),
                    (3, Some(Keyword), Fill::Over),
                ],
            );
        }
    }
}
