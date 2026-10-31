//! 字下げ (`core/src/indent.rs`) のテスト. 期待値は, `emacs -Q --batch` で `indent-region` を動かした結果
//! (`indent-tabs-mode` は `t`). 本文の全ての位置からの比べは `core/tests/lisp.json` (`test_lisp.rs`) で行うので,
//! ここでは, その表に無い形を試す.

use rseditor_core::indent::{indent_region, indent_sexp, Config, Plan};
use rseditor_core::syntax::Dialect;

/// 設定.
fn config(dialect: Dialect) -> Config {
    Config {
        dialect,
        indent_tabs: true,
        tab_width: 8,
    }
}

/// 本文を行に分ける.
fn split(text: &str) -> Vec<Vec<char>> {
    text.split('\n').map(|l| l.chars().collect()).collect()
}

/// 変更の計画を本文に行う.
fn apply(text: &str, plan: &Plan) -> String {
    let mut out: Vec<char> = text.chars().collect();
    for e in plan.edits.iter().rev() {
        out.splice(e.from..e.to, e.insert.chars());
    }
    out.into_iter().collect()
}

/// 本文の全体を字下げする.
fn region(text: &str, dialect: Dialect) -> String {
    let size = text.chars().count();
    apply(
        text,
        &indent_region(&split(text), 0, size, 0, &config(dialect)),
    )
}

#[test]
fn test_scheme_comments_and_characters() {
    // 入れ子の `#| |#` と, `#;` でコメントにした式 (印, 文字列, 文字) は, 引数にしない
    assert_eq!(
        region("(foo #| #| |# |#\nbar)", Dialect::Scheme),
        "(foo #| #| |# |#\n bar)"
    );
    assert_eq!(
        region("(foo #;'a\nbar)", Dialect::Scheme),
        "(foo #;'a\n bar)"
    );
    assert_eq!(
        region("(foo #;\"s\"\nbar)", Dialect::Scheme),
        "(foo #;\"s\"\n bar)"
    );
    assert_eq!(
        region("(foo #;#\\a\nbar)", Dialect::Scheme),
        "(foo #;#\\a\n bar)"
    );
    // 文字 (`#\a`) は, 引数
    assert_eq!(
        region("(foo #\\a\nbar)", Dialect::Scheme),
        "(foo #\\a\n     bar)"
    );
}

#[test]
fn test_third_special_argument_uses_the_normal_indent() {
    // 特別な引数が 3 つのフォーム: 3 つ目の特別な引数は, ふつうの字下げ. 本体は, 本体の字下げ
    assert_eq!(
        region("(dynamic-wind a b\nc\nd)", Dialect::Scheme),
        "(dynamic-wind a b\n\t      c\n  d)"
    );
    assert_eq!(
        region("(with-temp-buffer-window a b\nc\nd)", Dialect::Elisp),
        "(with-temp-buffer-window a b\n\t\t\t c\n  d)"
    );
}

#[test]
fn test_indent_sexp_is_only_for_lisp() {
    // Lisp でないバッファでは, 何もしない (このエディタの決まり. コマンドの説明のとおり)
    let plan = indent_sexp(&split("(a\nb)"), 0, 5, &config(Dialect::Fundamental));
    assert!(plan.edits.is_empty() && plan.cursor.is_none());
}
