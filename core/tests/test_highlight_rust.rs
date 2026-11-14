//! Rust の色付け (`core/src/highlight/rust.rs`) のテスト.
//! 期待値は, rust-lang/rust-mode の font-lock の結果 (`tests/rust-highlight/cases/` で, 同じ入力を Emacs と比べる).

use rseditor_core::highlight::{tokenize, Language};

/// 本文の色付けを, (文字列, 種類の名前) の並びにする.
fn kinds(text: &str) -> Vec<(String, &'static str)> {
    let chars: Vec<char> = text.chars().collect();
    tokenize(Language::Rust, text)
        .iter()
        .map(|s| {
            (
                chars[s.start..s.end].iter().collect::<String>(),
                s.kind.name(),
            )
        })
        .collect()
}

/// `(文字列, 種類)` の並びを作る.
fn expect(items: &[(&str, &'static str)]) -> Vec<(String, &'static str)> {
    items.iter().map(|(t, k)| (t.to_string(), *k)).collect()
}

#[test]
fn test_comments() {
    assert_eq!(
        kinds("// a\n/// d\n//! e\n//// f\n/* b /* n */ c */ /** g */ /**/ x"),
        expect(&[
            ("// a", "comment"),
            ("/// d", "doc"),
            ("//! e", "doc"),
            ("//// f", "comment"),
            ("/* b /* n */ c */", "comment"),
            ("/** g */", "doc"),
            ("/**/", "doc"),
        ])
    );
    // 閉じないコメントは, 本文の終わりまで
    assert_eq!(kinds("/* a"), expect(&[("/* a", "comment")]));
    // 3 文字に満たないものは, ドキュメントではない
    assert_eq!(kinds("//"), expect(&[("//", "comment")]));
    assert_eq!(kinds("/// "), expect(&[("/// ", "doc")]));
    assert_eq!(kinds("/**"), expect(&[("/**", "comment")]));
}

#[test]
fn test_strings_and_chars() {
    assert_eq!(
        kinds(r#""a \" b" 'x' '\n' '\x41' '\u{1F600}' '\'' 'é'"#),
        expect(&[
            (r#""a \" b""#, "string"),
            ("'x'", "string"),
            ("'\\n'", "string"),
            ("'\\x41'", "string"),
            ("'\\u{1F600}'", "string"),
            ("'\\''", "string"),
            ("'é'", "string"),
        ])
    );
    // 閉じない文字列は, 本文の終わりまで. 行をまたぐ文字列は, 行ごとに分ける
    assert_eq!(kinds("\"a"), expect(&[("\"a", "string")]));
    assert_eq!(
        kinds("\"a\nb\""),
        expect(&[("\"a", "string"), ("b\"", "string")])
    );
    // 文字として閉じないもの (ライフタイム, 壊れた逃れ) は, 文字ではない
    assert_eq!(kinds("'a "), expect(&[("a", "variable-name")]));
    assert_eq!(kinds("'\\q'"), expect(&[]));
    assert_eq!(kinds("'\\u{zz}'"), expect(&[]));
    assert_eq!(kinds("'\\x4'"), expect(&[]));
    assert_eq!(kinds("'\\u{41"), expect(&[]));
    assert_eq!(kinds("''"), expect(&[]));
    assert_eq!(kinds("'"), expect(&[]));
}

#[test]
fn test_raw_strings() {
    assert_eq!(
        kinds("r\"a\" r#\"b \"# r##\"d\"##"),
        expect(&[
            ("r\"a\"", "string"),
            ("r#\"b \"#", "string"),
            ("r##\"d\"##", "string"),
        ])
    );
    // 閉じない生の文字列は, 本文の終わりまで. `r` の後が引用符でなければ, ただの名前
    assert_eq!(kinds("r#\"a"), expect(&[("r#\"a", "string")]));
    assert_eq!(kinds("r#x"), expect(&[]));
}

#[test]
fn test_keywords_and_types() {
    assert_eq!(
        kinds("pub fn f(x: u8) { let mut y = true; }"),
        expect(&[
            ("pub", "keyword"),
            ("fn", "keyword"),
            ("f", "function-name"),
            ("x", "variable-name"),
            ("u8", "type"),
            ("let", "keyword"),
            ("mut", "keyword"),
            ("y", "variable-name"),
            ("true", "keyword"),
        ])
    );
    assert_eq!(kinds("unsafe { }"), expect(&[("unsafe", "warning")]));
    assert_eq!(
        kinds("default fn a() {}\nunion U { a: u8 }\nlet p = &raw const x;"),
        expect(&[
            ("default", "keyword"),
            ("fn", "keyword"),
            ("a", "function-name"),
            ("union", "keyword"),
            ("U", "type"),
            ("a", "variable-name"),
            ("u8", "type"),
            ("let", "keyword"),
            ("p", "variable-name"),
            ("raw", "keyword"),
            ("const", "keyword"),
        ])
    );
}

#[test]
fn test_attributes_and_macros() {
    assert_eq!(
        kinds("#[derive(Debug)]\n#![allow(x)]"),
        expect(&[
            ("#[derive(Debug)]", "preprocessor"),
            ("#![allow(x)]", "preprocessor"),
        ])
    );
    assert_eq!(
        kinds("vec![1]; println!(\"a\"); write!(f, \"b\"); writeln!(f, r#\"c\"#);"),
        expect(&[
            ("vec!", "preprocessor"),
            ("println!", "builtin"),
            ("\"a\"", "string"),
            ("write!", "builtin"),
            ("\"b\"", "string"),
            ("writeln!", "builtin"),
            ("r#\"c\"#", "string"),
        ])
    );
}

#[test]
fn test_names_and_paths() {
    assert_eq!(
        kinds("Foo::bar(); foo::<u8>(); a::B; for i in x { r? }"),
        expect(&[
            ("Foo", "type"),
            ("u8", "type"),
            ("a", "constant"),
            ("B", "type"),
            ("for", "keyword"),
            ("i", "variable-name"),
            ("in", "keyword"),
            ("?", "builtin"),
        ])
    );
    assert_eq!(
        kinds("fn lt<'a>(x: &'a str) {}"),
        expect(&[
            ("fn", "keyword"),
            ("lt", "function-name"),
            ("a", "variable-name"),
            ("x", "variable-name"),
            ("a", "variable-name"),
            ("str", "type"),
        ])
    );
    assert_eq!(
        kinds("let n = 0xffu8 + 2.5e1f64 + 7;\nmod m { use std::io; }\nenum E {}\ntype T = u8;"),
        expect(&[
            ("let", "keyword"),
            ("n", "variable-name"),
            ("u8", "type"),
            ("f64", "type"),
            ("mod", "keyword"),
            ("m", "constant"),
            ("use", "keyword"),
            ("std", "constant"),
            ("enum", "keyword"),
            ("E", "type"),
            ("type", "keyword"),
            ("T", "type"),
            ("u8", "type"),
        ])
    );
}

#[test]
fn test_unicode_positions() {
    // 位置は文字単位 (バイトではない)
    let spans = tokenize(Language::Rust, "// あ\nfn い() {}");
    assert_eq!((spans[0].start, spans[0].end), (0, 4));
    assert_eq!((spans[1].start, spans[1].end), (5, 7));
    assert_eq!((spans[2].start, spans[2].end), (8, 9));
}

#[test]
fn test_mode_name() {
    assert_eq!(Language::from_mode_name("rust-mode"), Some(Language::Rust));
    assert!(Language::Rust.spec().is_none());
}
