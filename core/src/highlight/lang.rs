//! 言語ごとの字句の表. 字句解析器 (`mod.rs`) は、言語によらず 1 つで、違いはここのデータだけです.
//!
//! 語は、Emacs 30 の cc-mode (`c-keywords-regexp`、`c-primitive-type-key`、`c-constant-key`) から取りました.
//! 表は、二分探索のために、辞書順に並べます.

/// 言語.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    /// C (`c-mode`).
    C,
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
};

impl Language {
    /// 言語の表.
    pub fn spec(self) -> &'static LanguageSpec {
        match self {
            Language::C => &C,
        }
    }

    /// `major-mode` の名前から、言語を決めます (言語が無ければ `None`).
    pub fn from_mode_name(name: &str) -> Option<Language> {
        match name {
            "c-mode" => Some(Language::C),
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
