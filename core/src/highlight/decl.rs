//! 宣言の名前 (関数、変数、型) を決めます. Emacs の cc-mode の宣言の推測 (`c-font-lock-declarations`) の、よく使う形だけの版です.
//!
//! 字句解析器が覚えた、コードの語と記号の並び (コメントとプリプロセッサの行を除く) を、文の始まりから読みます.
//! 型の並びと宣言子 (名前、`*`、`[]`、`()`、`:`、`=`) の形に合えば宣言として、合わなければ式として読み飛ばします.
//!
//! 扱う形: 変数 (複数、配列、初期化、ビットフィールド)、関数の宣言と定義、関数ポインタ、引数、`typedef`、
//! `struct` `union` `enum` (`class`) の名前と本体、ラベルと `goto`. 扱わない形: C++ のテンプレート、名前空間 (`::`)、
//! クラスの継承とメソッド、ファイルの先頭の、型の無い関数の呼び出し (`MODULE_LICENSE("GPL");`).

use super::{Class, LanguageSpec, Span, Tok, TokenKind};

/// 宣言の前に付く語 (型ではない).
const SPECIFIERS: &[&str] = &[
    "_Noreturn",
    "_Thread_local",
    "auto",
    "const",
    "constexpr",
    "explicit",
    "extern",
    "friend",
    "inline",
    "mutable",
    "register",
    "restrict",
    "static",
    "thread_local",
    "typedef",
    "virtual",
    "volatile",
];

/// 後ろに括弧が付く、宣言の前の語 (`__attribute__((unused))`).
const ATTRIBUTES: &[&str] = &["__attribute__", "__declspec"];

/// 後ろの括弧の式が型になる語 (`typeof(x) y;`).
const TYPE_OF: &[&str] = &["__typeof__", "decltype", "typeof"];

/// 名前と本体を持つ型の語.
const AGGREGATES: &[&str] = &["class", "enum", "struct", "union"];

/// 宣言の名前を決めます.
///
/// # Arguments
///
/// * `spec` - 言語の表.
/// * `chars` - 本文.
/// * `toks` - コードの字句 (字句解析器が読んだもの).
///
/// # Returns
///
/// 名前の範囲 (型の名前、関数の名前、変数の名前、ラベル). 並びの順は決まっていない.
pub(super) fn declarations(spec: &LanguageSpec, chars: &[char], toks: &[Tok]) -> Vec<Span> {
    let words = toks
        .iter()
        .map(|t| {
            if t.class == Class::Word {
                chars[t.start..t.end].iter().collect()
            } else {
                String::new()
            }
        })
        .collect();
    let mut parser = Parser {
        spec,
        chars,
        toks,
        words,
        out: Vec::new(),
    };
    parser.scan(0, false);
    parser.out
}

/// 宣言を読む状態.
struct Parser<'a> {
    /// 言語の表.
    spec: &'a LanguageSpec,
    /// 本文.
    chars: &'a [char],
    /// コードの字句.
    toks: &'a [Tok],
    /// 字句ごとの語 (語でなければ空).
    words: Vec<String>,
    /// ここまでに見つけた名前.
    out: Vec<Span>,
}

impl Parser<'_> {
    /// 位置 `j` の記号 (記号でなければ、字句の外も、`'\0'`).
    fn punct(&self, j: usize) -> char {
        match self.toks.get(j) {
            Some(t) if t.class == Class::Punct => self.chars[t.start],
            _ => '\0',
        }
    }

    /// 位置 `j` が、キーワードでない語 (識別子) か.
    fn is_name(&self, j: usize) -> bool {
        self.words
            .get(j)
            .is_some_and(|w| !w.is_empty() && !self.spec.is_keyword(w))
    }

    /// 位置 `j` が、`list` の語で、この言語のキーワードか.
    fn is_word_in(&self, j: usize, list: &[&str]) -> bool {
        self.words
            .get(j)
            .is_some_and(|w| list.contains(&w.as_str()) && self.spec.is_keyword(w))
    }

    /// 位置 `j` が、宣言の前に付く語か.
    fn is_specifier(&self, j: usize) -> bool {
        self.words
            .get(j)
            .is_some_and(|w| SPECIFIERS.contains(&w.as_str()) && self.spec.is_keyword(w))
    }

    /// 位置 `j` が、`struct` など名前と本体を持つ型の語か.
    fn is_aggregate(&self, j: usize) -> bool {
        self.words
            .get(j)
            .is_some_and(|w| AGGREGATES.contains(&w.as_str()) && self.spec.is_keyword(w))
    }

    /// 位置 `j` が、キーワードの語か.
    fn is_keyword(&self, j: usize) -> bool {
        self.words.get(j).is_some_and(|w| self.spec.is_keyword(w))
    }

    /// 位置 `j` の字句を、名前として足します.
    fn name(&mut self, j: usize, kind: TokenKind) {
        let tok = self.toks[j];
        self.out.push(Span {
            start: tok.start,
            end: tok.end,
            kind,
        });
    }

    /// 位置 `j` の開き括弧から、対応する閉じ括弧の次までの位置 (閉じなければ、字句の終わり).
    fn skip_balanced(&self, j: usize) -> usize {
        let mut depth = 0usize;
        for k in j..self.toks.len() {
            match self.punct(k) {
                '(' | '[' | '{' => depth += 1,
                ')' | ']' | '}' => {
                    depth -= 1;
                    if depth == 0 {
                        return k + 1;
                    }
                }
                _ => {}
            }
        }
        self.toks.len()
    }

    /// 位置 `j` から、`stops` のどれかの記号 (括弧の外) か、対応しない閉じ括弧の位置まで進みます.
    fn skip_expr(&self, mut j: usize, stops: &[char]) -> usize {
        while j < self.toks.len() {
            let c = self.punct(j);
            if stops.contains(&c) || matches!(c, ')' | ']' | '}') {
                break;
            }
            j = if matches!(c, '(' | '[' | '{') {
                self.skip_balanced(j)
            } else {
                j + 1
            };
        }
        j
    }

    /// 文の始まりから、本文の終わり (`stop` なら、対応する `}` の次) まで読みます.
    ///
    /// # Returns
    ///
    /// 読み終えた位置.
    fn scan(&mut self, mut i: usize, stop: bool) -> usize {
        let mut start = true;
        let mut depth = 0usize;
        let mut case = false;
        while i < self.toks.len() {
            if start {
                if let Some(next) = self.try_decl(i) {
                    i = next;
                    continue;
                }
                if self.is_name(i) && self.punct(i + 1) == ':' && self.punct(i + 2) != ':' {
                    self.name(i, TokenKind::Constant);
                    i += 2;
                    continue;
                }
            }
            start = false;
            match self.punct(i) {
                ';' => start = true,
                '{' => {
                    depth += 1;
                    start = true;
                }
                '}' => {
                    if depth == 0 && stop {
                        return i + 1;
                    }
                    depth = depth.saturating_sub(1);
                    start = true;
                }
                ':' => {
                    start = case || (i > 0 && self.is_keyword(i - 1));
                    case = false;
                }
                _ => match self.words[i].as_str() {
                    "goto" if self.is_name(i + 1) => self.name(i + 1, TokenKind::Constant),
                    "else" | "do" => start = true,
                    "case" => case = true,
                    "for" if self.punct(i + 1) == '(' => {
                        // `for (int i = 0; ...)` の、初めの式
                        i += 1;
                        start = true;
                    }
                    _ => {}
                },
            }
            i += 1;
        }
        i
    }

    /// 位置 `i` から宣言を読みます. 宣言でなければ、見つけた名前を捨てます.
    ///
    /// # Returns
    ///
    /// 宣言の次の位置 (関数の定義なら、本体の `{` の位置). 宣言でなければ `None`.
    fn try_decl(&mut self, i: usize) -> Option<usize> {
        let mark = self.out.len();
        let end = self.declaration(i);
        if end.is_none() {
            self.out.truncate(mark);
        }
        end
    }

    /// 型と、`,` で区切った宣言子を読みます.
    fn declaration(&mut self, i: usize) -> Option<usize> {
        let (mut j, typedef) = self.type_specifier(i, false)?;
        if self.punct(j) == ';' {
            // `struct node;` や `enum { A } ;` (宣言子の無い宣言)
            return Some(j + 1);
        }
        if j >= self.toks.len() {
            // 閉じない本体で、本文が終わった
            return Some(j);
        }
        loop {
            let (next, function) = self.declarator(j, typedef)?;
            j = next;
            match self.punct(j) {
                ',' => j += 1,
                ';' => return Some(j + 1),
                // 本文の終わりで、`;` が無い
                '\0' if j >= self.toks.len() => return Some(j),
                '{' if function => return Some(j),
                _ => return None,
            }
        }
    }

    /// 型 (前に付く語、組み込みの型、`struct 名前 { ... }`、型の名前) を読みます.
    /// 型の名前は、後ろが名前か `*` `&` (`bare` なら `,` `)` も) のときだけ、型として色を付けます.
    ///
    /// # Returns
    ///
    /// 型の次の位置と、`typedef` が付いていたか (型が無ければ `None`).
    fn type_specifier(&mut self, mut j: usize, bare: bool) -> Option<(usize, bool)> {
        let mut typedef = false;
        let mut seen = false;
        loop {
            if self.is_specifier(j) {
                typedef |= self.words[j] == "typedef";
                // C++ の `auto x = ...;` は、`auto` が型の代わり
                seen |= self.words[j] == "auto";
                j += 1;
            } else if self.is_word_in(j, ATTRIBUTES) && self.punct(j + 1) == '(' {
                j = self.skip_balanced(j + 1);
            } else if self.is_word_in(j, TYPE_OF) && self.punct(j + 1) == '(' {
                j = self.skip_balanced(j + 1);
                seen = true;
            } else if self.words.get(j).is_some_and(|w| self.spec.is_type(w)) {
                // Emacs は、`_Complex double z;` の `z` に色を付けない
                if matches!(self.words[j].as_str(), "_Complex" | "_Imaginary") {
                    return None;
                }
                seen = true;
                j += 1;
            } else if self.is_aggregate(j) {
                let tag = self.words[j].clone();
                j += 1;
                let named = self.is_name(j);
                if named {
                    self.name(j, TokenKind::Type);
                    j += 1;
                }
                if self.punct(j) == '{' {
                    j = if tag == "enum" {
                        self.enum_body(j + 1)
                    } else {
                        self.scan(j + 1, true)
                    };
                } else if !named {
                    return None;
                }
                seen = true;
            } else if !seen && self.is_name(j) && self.type_follows(j + 1, bare) {
                self.name(j, TokenKind::Type);
                seen = true;
                j += 1;
            } else {
                break;
            }
        }
        seen.then_some((j, typedef))
    }

    /// 位置 `j` が、型の名前の次に来られる字句か.
    fn type_follows(&self, j: usize, bare: bool) -> bool {
        self.is_name(j)
            || self.is_specifier(j)
            || matches!(self.punct(j), '*' | '&')
            || (bare && matches!(self.punct(j), ',' | ')'))
    }

    /// 宣言子 (`*` と、名前、`[]`、`()`、`:`、`= 初期値`) を読みます.
    ///
    /// # Returns
    ///
    /// 宣言子の次の位置と、関数の宣言子だったか (宣言子として読めなければ `None`).
    fn declarator(&mut self, mut j: usize, typedef: bool) -> Option<(usize, bool)> {
        while matches!(self.punct(j), '*' | '&') || self.is_specifier(j) {
            j += 1;
        }
        let mut function = false;
        if self.punct(j) == '(' && self.punct(j + 1) == '*' {
            // 関数ポインタ `(*名前)(引数)`
            let mut k = j + 1;
            while self.punct(k) == '*' {
                k += 1;
            }
            if !self.is_name(k) || self.punct(k + 1) != ')' {
                return None;
            }
            self.name(
                k,
                if typedef {
                    TokenKind::Type
                } else {
                    TokenKind::FunctionName
                },
            );
            j = k + 2;
            if self.punct(j) == '(' {
                j = self.params(j)?;
            }
        } else if self.is_name(j) {
            let name = j;
            j += 1;
            if self.punct(j) == '(' {
                function = true;
                self.name(
                    name,
                    if typedef {
                        TokenKind::Type
                    } else {
                        TokenKind::FunctionName
                    },
                );
                j = self.params(j)?;
                // `const` や `__attribute__((...))` など
                while self.is_keyword(j) {
                    j += 1;
                    if self.punct(j) == '(' {
                        j = self.skip_balanced(j);
                    }
                }
            } else {
                self.name(
                    name,
                    if typedef {
                        TokenKind::Type
                    } else {
                        TokenKind::VariableName
                    },
                );
                while self.punct(j) == '[' {
                    j = self.skip_balanced(j);
                }
                if self.punct(j) == ':' {
                    j = self.skip_expr(j + 1, &[',', ';']);
                }
            }
        } else {
            return None;
        }
        if self.punct(j) == '=' {
            j = self.skip_expr(j + 1, &[',', ';']);
        }
        Some((j, function))
    }

    /// 引数の並び (`(` の位置から) を読みます.
    ///
    /// # Returns
    ///
    /// `)` の次の位置 (閉じていなければ `None`).
    fn params(&mut self, open: usize) -> Option<usize> {
        let close = self.skip_balanced(open);
        if self.punct(close - 1) != ')' {
            return None;
        }
        let mut j = open + 1;
        while j < close - 1 {
            j = self.param(j, close - 1);
            j += 1;
        }
        Some(close)
    }

    /// 引数 1 つ (`start` から、`,` か `end` の前まで) を読みます. 型と名前に色を付けられなければ、読み飛ばす.
    ///
    /// # Returns
    ///
    /// 引数の終わり (`,` か `end`) の位置.
    fn param(&mut self, start: usize, end: usize) -> usize {
        let mark = self.out.len();
        let mut j = start;
        if let Some((next, _)) = self.type_specifier(start, true) {
            j = next;
            if !matches!(self.punct(j), ',' | ')') {
                match self.declarator(j, false) {
                    Some((next, _)) => j = next,
                    None => self.out.truncate(mark),
                }
            }
        }
        self.skip_expr(j, &[',']).min(end)
    }

    /// 列挙子の並び (`{` の次から) を読みます.
    ///
    /// # Returns
    ///
    /// `}` の次の位置.
    fn enum_body(&mut self, mut j: usize) -> usize {
        while j < self.toks.len() && self.punct(j) != '}' {
            if self.is_name(j) {
                self.name(j, TokenKind::VariableName);
                j += 1;
                if self.punct(j) == '=' {
                    j = self.skip_expr(j + 1, &[',', '}']);
                }
            } else {
                j += 1;
            }
        }
        j + 1
    }
}
