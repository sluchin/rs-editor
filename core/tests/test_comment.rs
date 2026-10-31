//! `comment-dwim` (`M-;`) のテスト. 期待値は, `emacs -Q --batch` の scheme-mode で, `comment-dwim` を動かした結果.
//! 本文の全ての位置からの比べは `core/tests/lisp.json` (`test_lisp.rs`) で行うので, ここでは, リージョンと, 空白だけの行を試す.

use rseditor_core::comment::{comment_dwim, comment_region};
use rseditor_core::indent::{Config, Plan};
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

/// 変更の計画を本文に行い, 本文とカーソルを返す.
fn apply(text: &str, plan: &Plan) -> (String, usize) {
    let mut out: Vec<char> = text.chars().collect();
    for e in plan.edits.iter().rev() {
        out.splice(e.from..e.to, e.insert.chars());
    }
    (out.into_iter().collect(), plan.cursor.unwrap())
}

/// マークと点 (0 から) から, `comment-dwim` を動かす.
fn dwim(text: &str, mark: Option<usize>, pos: usize) -> (String, usize) {
    let plan = comment_dwim(&split(text), pos, mark, 40, &config(Dialect::Scheme)).unwrap();
    apply(text, &plan)
}

#[test]
fn test_no_comment_syntax() {
    let lines = split("a");
    let result = comment_dwim(&lines, 0, None, 40, &config(Dialect::Fundamental));
    assert_eq!(result.unwrap_err(), "No comment syntax is defined");
}

#[test]
fn test_region_comments_and_uncomments_lines() {
    assert_eq!(
        dwim("(a)\n  (b)\n", Some(0), 10),
        (";; (a)\n;;   (b)\n".to_string(), 16)
    );
    assert_eq!(
        dwim("  (a)\n\n    (b)", Some(0), 14),
        ("  ;; (a)\n\n  ;;   (b)".to_string(), 20)
    );
    assert_eq!(
        dwim(";; (a)\n  ;;(b)", Some(0), 14),
        ("(a)\n  (b)".to_string(), 9)
    );
    assert_eq!(
        dwim(";; (a)\n(b)", Some(0), 10),
        (";; ;; (a)\n;; (b)".to_string(), 16)
    );
    // 終わりが行の始めなら, その行は含めない
    assert_eq!(
        dwim("(a)\n(b)\n", Some(0), 8),
        (";; (a)\n;; (b)\n".to_string(), 14)
    );
    assert_eq!(
        dwim(";; (a b)\n  ;; (c d)\n", Some(0), 6),
        ("(a b)\n  ;; (c d)\n".to_string(), 3)
    );
    // 空白だけの行しか無ければ, 何もしない
    assert_eq!(dwim("\n  \n", Some(0), 4), ("\n  \n".to_string(), 4));
}

#[test]
fn test_region_is_aligned_to_the_shallowest_indentation() {
    let plan = comment_region(
        &split("    (a)\n  (b)"),
        0,
        13,
        13,
        &config(Dialect::Scheme),
    );
    assert_eq!(apply("    (a)\n  (b)", &plan).0, "  ;;   (a)\n  ;; (b)");
}

#[test]
fn test_blank_line_gets_a_comment_at_the_indentation() {
    assert_eq!(
        dwim("(define (f)\n\n  1)", None, 12),
        ("(define (f)\n  ;; \n  1)".to_string(), 17)
    );
    assert_eq!(dwim("   ", None, 3), (";; ".to_string(), 3));
    // 字下げが合っていれば, 空白をそのまま使う
    assert_eq!(
        dwim("(a\n  \n b)", None, 5),
        ("(a\n ;; \n b)".to_string(), 7)
    );
}
