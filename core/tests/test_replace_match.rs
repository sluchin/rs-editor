//! 置き換える文字列の組み立て (`core/src/replace.rs`) のテスト. 期待値は, `emacs -Q --batch` の `replace-match` の結果.

use rseditor_core::replace::{apply_case, case_action, expand, CaseAction};

#[test]
fn test_caseless_initial_counts_as_a_lowercase_initial() {
    // 大文字小文字の無い文字 (`1`, `-`) が語の頭: 後ろが大文字だけなら, 全て大文字
    assert_eq!(case_action("1AB"), CaseAction::AllCaps);
    assert_eq!(apply_case("xy", case_action("1AB")), "XY");
    assert_eq!(case_action("-AB"), CaseAction::AllCaps);
    assert_eq!(case_action("1ab"), CaseAction::NoChange);
}

#[test]
fn test_expand_backslashes() {
    // `\\` は `\` 1 つ. 大文字と小文字は, 展開した後に変える
    let text = expand("a\\\\b", &[]).unwrap();
    assert_eq!(text, "a\\b");
    assert_eq!(apply_case(&text, case_action("AB")), "A\\B");
    // `\` の後ろが `&` 数字 `\` のどれでもなければ, エラー
    assert_eq!(
        expand("a\\qb", &[]),
        Err("Invalid use of ‘\\’ in replacement text".to_string())
    );
}
