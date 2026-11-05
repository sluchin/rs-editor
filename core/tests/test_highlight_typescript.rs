//! TypeScript の色付け (`core/src/highlight/typescript.rs`) のテスト.
//! 期待値は、emacs-typescript/typescript-mode の font-lock の結果 (`tests/ts-highlight/expected/`).
//! 同じ入力 (`tests/ts-highlight/cases/`) を、`tests/ts-highlight/run.sh` で、Emacs と直に比べられる.

use rseditor_core::highlight::{tokenize, Language};

/// 範囲を、Emacs の出力 (`dump.el`) と同じ形 (始まりと終わりは 1 から数える. タブ区切り) にする.
fn dump(text: &str) -> String {
    tokenize(Language::TypeScript, text)
        .iter()
        .map(|s| format!("{}\t{}\t{}\n", s.start + 1, s.end + 1, s.kind.name()))
        .collect()
}

/// 本文の色付けを、(文字列, 種類の名前) の並びにする.
fn kinds(text: &str) -> Vec<(String, &'static str)> {
    let chars: Vec<char> = text.chars().collect();
    tokenize(Language::TypeScript, text)
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

/// 試験の入力と、Emacs の出力を、同じ名前で比べる.
macro_rules! golden {
    ($($test:ident: $name:literal,)*) => {$(
        #[test]
        fn $test() {
            let text = include_str!(concat!("../../tests/ts-highlight/cases/", $name, ".ts"));
            let expected = include_str!(concat!("../../tests/ts-highlight/expected/", $name, ".txt"));
            assert_eq!(dump(text), expected);
        }
    )*};
}

golden! {
    test_basics: "01-basics",
    test_generics: "02-generics",
    test_declarations: "03-declarations",
    test_misc: "04-misc",
    test_arrows: "05-arrows",
    test_realistic: "06-realistic",
    test_edges: "07-edges",
    test_jsdoc: "08-jsdoc",
    test_corners: "09-corners",
    test_more_corners: "10-more-corners",
    test_bof_arrow: "11-bof-arrow",
}

#[test]
fn test_mode_name() {
    assert_eq!(
        Language::from_mode_name("typescript-mode"),
        Some(Language::TypeScript)
    );
    assert!(Language::TypeScript.spec().is_none());
    assert_eq!(kinds(""), expect(&[]));
}

#[test]
fn test_unbalanced_parentheses_before_an_arrow_are_skipped() {
    // Emacs は、この入力で `scan-error` になり、色付けを止める. ここでは、引数を探すのをやめるだけ
    assert_eq!(kinds("a) => { };"), expect(&[("=>", "keyword")]));
}
