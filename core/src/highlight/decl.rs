//! 宣言の名前 (関数、変数、型) を決めます. Emacs の cc-mode の宣言の推測 (`c-font-lock-declarations`) の、よく使う形だけの版です.
//!
//! 字句解析器が覚えた、コードの語と記号の並び (コメントとプリプロセッサの行を除く) を、文の始まりから読みます.
//! 型の並びと宣言子 (名前、`*`、`[]`、`()`、`:`、`=`) の形に合えば宣言として、合わなければ式として読み飛ばします.
//!
//! 扱う形: 変数 (複数、配列、初期化、ビットフィールド)、関数の宣言と定義、関数ポインタ、引数、`typedef`、
//! `struct` `union` `enum` (`class`) の名前と本体、ラベルと `goto`.
//!
//! C++ では、さらに、修飾した名前 (`std::string`。`::` の前は定数の色)、`namespace` の名前、`enum class`、クラスの継承、
//! コンストラクタとデストラクタ、メソッドの定義 (`void A::f()`)、`using 名前 = 型;`、テンプレートの引数と仮引数を読みます.
//! 演算子の関数 (`operator+`)、`typename T::x`、配列へのポインタと参照 `int (&a)[4]`、`if` `while` `switch` の条件の中の宣言も読みます.

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
        cpp: spec.is_keyword("namespace"),
        chars,
        toks,
        words,
        out: Vec::new(),
    };
    parser.scan(0, false, false);
    parser.out
}

/// 宣言を読む状態.
struct Parser<'a> {
    /// 言語の表.
    spec: &'a LanguageSpec,
    /// C++ か (修飾した名前、テンプレート、クラスの本体を読む).
    cpp: bool,
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

    /// 位置 `j` の名前から続く、修飾した名前 (`a::b::c`) の、最後の名前の位置 (修飾が無ければ `j`).
    fn chain_end(&self, mut j: usize) -> usize {
        while self.cpp
            && self.punct(j + 1) == ':'
            && self.punct(j + 2) == ':'
            && self.is_name(j + 3)
        {
            j += 3;
        }
        j
    }

    /// 修飾した名前 (`first` から `last`) に色を付けます. `::` の前は定数の色、最後は `kind`.
    fn name_chain(&mut self, first: usize, last: usize, kind: TokenKind) {
        let mut k = first;
        while k < last {
            self.name(k, TokenKind::Constant);
            k += 3;
        }
        self.name(last, kind);
    }

    /// 位置 `open` の `<` から、対応する `>` の次までの位置 (文の終わりや括弧の外れに先に着けば `None`).
    fn angle_end(&self, open: usize) -> Option<usize> {
        let mut depth = 0usize;
        for k in open..self.toks.len() {
            match self.punct(k) {
                '<' => depth += 1,
                '>' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(k + 1);
                    }
                }
                ';' | '{' | '}' | ')' => return None,
                _ => {}
            }
        }
        None
    }

    /// テンプレートの引数 (`<` の位置から `close` の前まで) の名前に、型の色を付けます.
    fn angle_args(&mut self, open: usize, close: usize) {
        let mut k = open + 1;
        while k < close {
            if self.is_name(k) {
                // Emacs は、`::` の前の名前と、`<` が続く名前だけに色を付ける (`Foo<Bar>` の `Bar` には付けない)
                let last = self.chain_end(k);
                let mut q = k;
                while q < last {
                    self.name(q, TokenKind::Constant);
                    q += 3;
                }
                if self.punct(last + 1) == '<' {
                    self.name(last, TokenKind::Type);
                }
                k = last;
            }
            k += 1;
        }
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
    fn scan(&mut self, mut i: usize, stop: bool, in_class: bool) -> usize {
        let mut start = true;
        let mut depth = 0usize;
        let mut case = false;
        while i < self.toks.len() {
            if start {
                if let Some(next) = self.try_decl(i) {
                    i = next;
                    continue;
                }
                if depth == 0 {
                    if let Some(next) = self.constructor(i, in_class) {
                        i = next;
                        continue;
                    }
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
                    "namespace" if self.cpp && self.is_name(i + 1) => {
                        self.name(i + 1, TokenKind::Constant);
                    }
                    "using" if self.cpp && self.is_name(i + 1) && self.punct(i + 2) == '=' => {
                        self.name(i + 1, TokenKind::Type);
                        // `using Vec = std::vector<int>;` の、右辺の型
                        let mark = self.out.len();
                        if self.type_specifier(i + 3, true).is_none() {
                            self.out.truncate(mark);
                        }
                    }
                    "template" if self.cpp && self.punct(i + 1) == '<' => {
                        if let Some(close) = self.angle_end(i + 1) {
                            self.template_params(i + 1, close);
                            i = close - 1;
                            start = true;
                        }
                    }
                    "if" | "while" | "switch" if self.cpp && self.punct(i + 1) == '(' => {
                        if let Some(end) = self.condition(i + 1) {
                            i = end;
                            start = self.punct(end) == ';';
                            continue;
                        }
                    }
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

    /// `template <typename T, class U>` の仮引数の名前 (`typename` と `class` の次) に、型の色を付けます.
    fn template_params(&mut self, open: usize, close: usize) {
        for k in open + 2..close {
            if !self.is_name(k) {
                continue;
            }
            if matches!(self.words[k - 1].as_str(), "typename" | "class") {
                self.name(k, TokenKind::Type);
            } else if self.spec.is_type(&self.words[k - 1])
                && matches!(self.punct(k + 1), ',' | '>' | '=')
            {
                // 型でない仮引数 `int N`
                self.name(k, TokenKind::VariableName);
            }
        }
    }

    /// 型の無い関数の宣言と定義 (C++ のコンストラクタとデストラクタ) を読みます. クラスの本体の中か、修飾した名前
    /// (`A::A(int x) : v(x) {}`) の、`(` が続くときだけです.
    ///
    /// # Returns
    ///
    /// 宣言の次の位置 (定義なら、本体の `{` の位置). 宣言でなければ `None`.
    fn constructor(&mut self, i: usize, in_class: bool) -> Option<usize> {
        if !self.cpp {
            return None;
        }
        let mut j = i;
        while self.is_specifier(j) {
            j += 1;
        }
        j += usize::from(self.punct(j) == '~');
        if !self.is_name(j) {
            return None;
        }
        let mut last = self.chain_end(j);
        // 修飾したデストラクタ `A::~A()`
        let tilde = self.punct(last + 1) == ':'
            && self.punct(last + 2) == ':'
            && self.punct(last + 3) == '~'
            && self.is_name(last + 4);
        let name = if tilde { last + 4 } else { last };
        if self.punct(name + 1) != '(' || !(in_class || name > j) {
            return None;
        }
        let mark = self.out.len();
        if tilde {
            self.name_chain(j, last, TokenKind::Constant);
            last = name;
            self.name(last, TokenKind::FunctionName);
        } else {
            self.name_chain(j, last, TokenKind::FunctionName);
        }
        let end = self.function_tail(last + 1);
        if end.is_none() {
            self.out.truncate(mark);
        }
        end
    }

    /// 関数の引数の並び (`(` の位置から) と、後ろの `const` など、初期化子の並び (`: v(x)`)、`= default` を読みます.
    ///
    /// # Returns
    ///
    /// `;` の次の位置 (定義なら、本体の `{` の位置).
    fn function_tail(&mut self, open: usize) -> Option<usize> {
        let mut j = self.params(open)?;
        j = self.after_params(j);
        if self.punct(j) == '=' {
            j = self.skip_expr(j + 1, &[';']);
        }
        match self.punct(j) {
            ';' => Some(j + 1),
            '{' => Some(j),
            _ => None,
        }
    }

    /// 引数の並びの後ろの、`const` や `__attribute__((...))`、コンストラクタの初期化子の並び (`: v(x)`) を、読み飛ばします.
    fn after_params(&self, mut j: usize) -> usize {
        while self.is_keyword(j) {
            j += 1;
            if self.punct(j) == '(' {
                j = self.skip_balanced(j);
            }
        }
        if self.cpp && self.punct(j) == ':' && self.punct(j + 1) != ':' {
            j = self.skip_expr(j + 1, &['{', ';']);
        }
        j
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
                // C++ の `for (auto x : v)`
                ')' if self.cpp => return Some(j),
                _ => return None,
            }
        }
    }

    /// 継承の並び (`:` の次から `{` か `;` の前まで) の名前に、型の色を付けます.
    ///
    /// # Returns
    ///
    /// `{` か `;` の位置 (本文の終わりでもよい).
    fn bases(&mut self, mut j: usize) -> usize {
        while j < self.toks.len() && !matches!(self.punct(j), '{' | ';') {
            if self.is_name(j) {
                let last = self.chain_end(j);
                self.name_chain(j, last, TokenKind::Type);
                j = last;
            }
            j += 1;
        }
        j
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
            } else if self.cpp && self.is_word_in(j, &["typename"]) && self.is_name(j + 1) {
                // `typename T::value_type x` の `typename`
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
                // C++ の `enum class E`
                if self.cpp && tag == "enum" && matches!(self.words[j].as_str(), "class" | "struct")
                {
                    j += 1;
                }
                let named = self.is_name(j);
                if named {
                    self.name(j, TokenKind::Type);
                    j += 1;
                }
                if self.cpp && named && self.punct(j) == ':' {
                    j = self.bases(j + 1);
                }
                if self.punct(j) == '{' {
                    j = if tag == "enum" {
                        self.enum_body(j + 1)
                    } else {
                        self.scan(j + 1, true, self.cpp)
                    };
                } else if !named {
                    return None;
                }
                seen = true;
            } else if !seen && self.is_name(j) {
                let last = self.chain_end(j);
                // C++ のテンプレートの引数 `vector<int>`
                let angle = if self.cpp && self.punct(last + 1) == '<' {
                    self.angle_end(last + 1)
                } else {
                    None
                };
                let after = angle.unwrap_or(last + 1);
                if !self.type_follows(after, bare) {
                    break;
                }
                self.name_chain(j, last, TokenKind::Type);
                if let Some(close) = angle {
                    self.angle_args(last + 1, close);
                }
                seen = true;
                j = after;
            } else {
                break;
            }
        }
        seen.then_some((j, typedef))
    }

    /// 位置 `j` が、型の名前の次に来られる字句か (`bare` は、引数と `using` の右辺のように、型だけで終われる場合).
    fn type_follows(&self, j: usize, bare: bool) -> bool {
        self.is_name(j)
            || self.is_specifier(j)
            || (self.cpp && self.is_word_in(j, &["operator"]))
            || matches!(self.punct(j), '*' | '&')
            || (bare && matches!(self.punct(j), ',' | ')' | ';'))
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
        let op = if self.cpp && self.is_word_in(j, &["operator"]) {
            self.operator_symbols(j)
                .filter(|&end| self.punct(end) == '(')
        } else {
            None
        };
        let open = j;
        if self.punct(j) == '(' && matches!(self.punct(j + 1), '*' | '&') {
            // 関数ポインタ `(*名前)(引数)` と、配列へのポインタと参照 `(*名前)[4]`
            let mut k = j + 1;
            while matches!(self.punct(k), '*' | '&') {
                k += 1;
            }
            if !self.is_name(k) || self.punct(k + 1) != ')' {
                return None;
            }
            j = k + 2;
            let call = self.punct(j) == '(';
            // Emacs は、`typedef int (&A)[4];` の `A` に色を付けない
            if !(typedef && !call && self.punct(open + 1) == '&') {
                self.name(
                    k,
                    if typedef {
                        TokenKind::Type
                    } else if call {
                        TokenKind::FunctionName
                    } else {
                        TokenKind::VariableName
                    },
                );
            }
            if call {
                j = self.params(j)?;
            } else {
                while self.punct(j) == '[' {
                    j = self.skip_balanced(j);
                }
            }
        } else if let Some(end) = op {
            // 演算子の関数 `operator+(...)`. 色が付くのは、演算子の記号
            function = true;
            for k in j + 1..end {
                self.name(k, TokenKind::FunctionName);
            }
            j = self.params(end)?;
            j = self.after_params(j);
        } else if self.is_name(j) {
            let first = j;
            let name = self.chain_end(j);
            j = name + 1;
            let qualified = if self.cpp
                && self.punct(j) == ':'
                && self.punct(j + 1) == ':'
                && self.is_word_in(j + 2, &["operator"])
            {
                self.operator_symbols(j + 2)
                    .filter(|&end| self.punct(end) == '(')
            } else {
                None
            };
            if let Some(end) = qualified {
                // `A::operator+(...)`. `::` の前は定数の色で、演算子の記号には色が付かない
                function = true;
                self.name_chain(first, name, TokenKind::Constant);
                j = self.params(end)?;
                j = self.after_params(j);
            } else if self.punct(j) == '(' {
                function = true;
                self.name_chain(
                    first,
                    name,
                    if typedef {
                        TokenKind::Type
                    } else {
                        TokenKind::FunctionName
                    },
                );
                j = self.params(j)?;
                j = self.after_params(j);
            } else {
                self.name_chain(
                    first,
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

    /// 位置 `op` の `operator` の次から続く、演算子の記号 (`+` `==` `()` `[]` `<<`) の次の位置 (記号が無ければ `None`).
    fn operator_symbols(&self, op: usize) -> Option<usize> {
        let first = op + 1;
        if matches!(
            (self.punct(first), self.punct(first + 1)),
            ('(', ')') | ('[', ']')
        ) {
            return Some(first + 2);
        }
        let mut k = first;
        while !matches!(self.punct(k), '\0' | '(' | ')' | ';' | '{' | '}') {
            k += 1;
        }
        (k > first).then_some(k)
    }

    /// `if` `while` `switch` の `(` の位置 `open` から、条件の中の宣言 (`if (int x = f())`、`if (auto it = m.find(k); ...)`) を読みます.
    /// 変数に初期値が付く形 (`=` が 1 つ) だけです.
    ///
    /// # Returns
    ///
    /// 宣言の次の位置 (`)` か `;`). 宣言でなければ `None`.
    fn condition(&mut self, open: usize) -> Option<usize> {
        let mark = self.out.len();
        let end = self.condition_declaration(open + 1);
        if end.is_none() {
            self.out.truncate(mark);
        }
        end
    }

    /// `condition` の本体 (宣言が読めなければ、色を捨てるのは呼び出し側).
    fn condition_declaration(&mut self, start: usize) -> Option<usize> {
        let (mut j, _) = self.type_specifier(start, false)?;
        while matches!(self.punct(j), '*' | '&') || self.is_specifier(j) {
            j += 1;
        }
        if !self.is_name(j) || self.punct(j + 1) != '=' || self.punct(j + 2) == '=' {
            return None;
        }
        let (end, _) = self.declarator(j, false)?;
        matches!(self.punct(end), ')' | ';').then_some(end)
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
