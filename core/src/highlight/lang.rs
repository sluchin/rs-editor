//! 言語ごとの字句の表. 字句解析器 (`mod.rs`) は、言語によらず 1 つで、違いはここのデータだけです.
//!
//! 語は、Emacs 30 の cc-mode (`c-keywords-regexp`、`c-primitive-type-key`、`c-constant-key`) から取りました.
//! 表は、二分探索のために、辞書順に並べます.

/// 言語.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    /// C (`c-mode`).
    C,
    /// C++ (`c++-mode`).
    Cpp,
    /// Rust (`rust-mode`. 解析は `rust.rs`).
    Rust,
    /// TypeScript (`typescript-mode`. 解析は `typescript.rs`).
    TypeScript,
    /// Scheme (`scheme-mode`. 解析は `lisp.rs`).
    Scheme,
    /// Emacs Lisp (`emacs-lisp-mode`. 解析は `lisp.rs`).
    Elisp,
}

/// 言語の字句の表.
#[derive(Debug)]
pub struct LanguageSpec {
    /// キーワード (型と定数を含む. 型と定数は、それぞれの表を先に引く).
    pub keywords: &'static [&'static str],
    /// 組み込みの型の名前.
    pub types: &'static [&'static str],
    /// 組み込みの定数の名前.
    pub constants: &'static [&'static str],
    /// 生の文字列 `R"x(...)x"` があるか.
    pub raw_strings: bool,
    /// 数値の区切り `1'000` があるか.
    pub digit_separators: bool,
}

/// C の表 (`c-mode`).
static C: LanguageSpec = LanguageSpec {
    keywords: &[
        "NULL",
        "_Bool",
        "_Complex",
        "_Generic",
        "_Imaginary",
        "__asm__",
        "__attribute__",
        "__declspec",
        "asm",
        "auto",
        "break",
        "case",
        "char",
        "const",
        "continue",
        "default",
        "do",
        "double",
        "else",
        "enum",
        "extern",
        "false",
        "float",
        "for",
        "goto",
        "if",
        "inline",
        "int",
        "long",
        "register",
        "restrict",
        "return",
        "short",
        "signed",
        "sizeof",
        "static",
        "struct",
        "switch",
        "true",
        "typedef",
        "typeof",
        "union",
        "unsigned",
        "void",
        "volatile",
        "while",
    ],
    types: &[
        "_Bool",
        "_Complex",
        "_Imaginary",
        "char",
        "double",
        "float",
        "int",
        "long",
        "short",
        "signed",
        "unsigned",
        "void",
    ],
    constants: &["NULL", "false", "true"],
    raw_strings: false,
    digit_separators: false,
};

/// C++ の表 (`c++-mode`).
static CPP: LanguageSpec = LanguageSpec {
    keywords: &[
        "NULL",
        "__asm__",
        "__attribute__",
        "__declspec",
        "alignas",
        "alignof",
        "and",
        "and_eq",
        "asm",
        "auto",
        "bitand",
        "bitor",
        "bool",
        "break",
        "case",
        "catch",
        "char",
        "char16_t",
        "char32_t",
        "char8_t",
        "class",
        "co_await",
        "co_return",
        "co_yield",
        "compl",
        "concept",
        "const",
        "const_cast",
        "consteval",
        "constexpr",
        "constinit",
        "continue",
        "decltype",
        "default",
        "delete",
        "do",
        "double",
        "dynamic_cast",
        "else",
        "enum",
        "explicit",
        "export",
        "extern",
        "false",
        "final",
        "float",
        "for",
        "friend",
        "goto",
        "if",
        "import",
        "inline",
        "int",
        "long",
        "module",
        "mutable",
        "namespace",
        "new",
        "noexcept",
        "not",
        "not_eq",
        "nullptr",
        "operator",
        "or",
        "or_eq",
        "override",
        "private",
        "protected",
        "public",
        "register",
        "reinterpret_cast",
        "requires",
        "return",
        "short",
        "signed",
        "sizeof",
        "static",
        "static_cast",
        "struct",
        "switch",
        "template",
        "this",
        "thread_local",
        "throw",
        "true",
        "try",
        "typedef",
        "typeid",
        "typename",
        "typeof",
        "union",
        "unsigned",
        "using",
        "virtual",
        "void",
        "volatile",
        "wchar_t",
        "while",
        "xor",
        "xor_eq",
    ],
    types: &[
        "bool", "char", "char16_t", "char32_t", "char8_t", "double", "float", "int", "long",
        "short", "signed", "unsigned", "void", "wchar_t",
    ],
    constants: &["NULL", "false", "nullptr", "true"],
    raw_strings: true,
    digit_separators: true,
};

impl Language {
    /// 言語の字句の表 (Lisp は、表を使わないので `None`).
    pub fn spec(self) -> Option<&'static LanguageSpec> {
        match self {
            Language::C => Some(&C),
            Language::Cpp => Some(&CPP),
            Language::Rust | Language::TypeScript | Language::Scheme | Language::Elisp => None,
        }
    }

    /// `major-mode` の名前から、言語を決めます (C、C++、Rust、TypeScript、`lisp-interaction-mode` の Scheme. それ以外は `None`. Lisp の他のモードは、構文の方言で決まる).
    pub fn from_mode_name(name: &str) -> Option<Language> {
        match name {
            "c-mode" => Some(Language::C),
            "c++-mode" => Some(Language::Cpp),
            "rust-mode" => Some(Language::Rust),
            "typescript-mode" => Some(Language::TypeScript),
            // `*scratch*` は、Scheme を評価する
            "lisp-interaction-mode" => Some(Language::Scheme),
            _ => None,
        }
    }
}

impl LanguageSpec {
    /// 組み込みの型か.
    pub fn is_type(&self, word: &str) -> bool {
        self.types.binary_search(&word).is_ok()
    }

    /// 組み込みの定数か.
    pub fn is_constant(&self, word: &str) -> bool {
        self.constants.binary_search(&word).is_ok()
    }

    /// キーワードか.
    pub fn is_keyword(&self, word: &str) -> bool {
        self.keywords.binary_search(&word).is_ok()
    }
}
