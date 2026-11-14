//! org-lint (`lisp/org-lint.scm`) のテスト. 偽のエディタ (バッファを持つ) の上で, 報告, 一覧のバッファ, 行へ移ることを確かめる.
//! 個々のチェッカーは, Emacs の org-lint の結果と比べる (`tests/org-lint/run.sh`).

use std::path::Path;

use rseditor_core::scheme::evaluator::Evaluator;

mod common;
use common::FakeEditor;

/// org と org-lint を読み込んだ評価器と, 偽のエディタを作る.
///
/// # Returns
///
/// 評価器と, 偽のエディタ.
fn setup() -> (Evaluator, FakeEditor) {
    let lisp = Path::new(env!("CARGO_MANIFEST_DIR")).join("../lisp");
    let ev = Evaluator::new();
    let editor = FakeEditor::new();
    ev.eval_with(
        &format!(
            "(load {:?}) (load {:?})",
            lisp.join("org.scm").to_string_lossy(),
            lisp.join("org-lint.scm").to_string_lossy()
        ),
        &editor,
    )
    .unwrap();
    (ev, editor)
}

/// 出しているバッファの本文を置く.
///
/// # Arguments
///
/// * `editor` - 偽のエディタ.
/// * `text` - 本文.
fn set_text(editor: &FakeEditor, text: &str) {
    editor.with_buffer(|b| {
        b.text = text.chars().collect();
        b.point = 1;
    });
}

/// 評価する (失敗したら panic する).
///
/// # Arguments
///
/// * `ev` - 評価器.
/// * `editor` - 偽のエディタ.
/// * `code` - コード.
///
/// # Returns
///
/// 結果.
fn run(ev: &Evaluator, editor: &FakeEditor, code: &str) -> String {
    ev.eval_with(code, editor)
        .unwrap_or_else(|e| panic!("{}: {}", code, e))
}

/// 評価して失敗の内容を返す.
///
/// # Arguments
///
/// * `ev` - 評価器.
/// * `editor` - 偽のエディタ.
/// * `code` - コード.
///
/// # Returns
///
/// 失敗の内容 (成功したら panic する).
fn fail(ev: &Evaluator, editor: &FakeEditor, code: &str) -> String {
    ev.eval_with(code, editor)
        .expect_err(&format!("{} did not fail", code))
        .to_string()
}

const SAMPLE: &str = "#+CATEGORY: a\n#+CATEGORY: b\n* QUOTE x :a::b:\n#+BEGIN_SRC\n#+END_SRC\n";

#[test]
fn test_org_lint_reports() {
    let (ev, editor) = setup();
    set_text(&editor, SAMPLE);
    assert_eq!(
        run(
            &ev,
            &editor,
            "(map (lambda (r) (list (car (cadr r)) (list-ref (cadr r) 3))) (org-lint-reports))"
        ),
        "((\"2\" deprecated-category-setup) (\"3\" spurious-colons) (\"3\" quote-section) (\"4\" missing-language-in-src-block))"
    );
    // チェッカーを選ぶ
    assert_eq!(
        run(
            &ev,
            &editor,
            "(map (lambda (r) (list-ref (cadr r) 3)) (org-lint '(spurious-colons)))"
        ),
        "(spurious-colons)"
    );
    assert!(fail(&ev, &editor, "(org-lint 5)").contains("Invalid argument `5' for `org-lint'"));
}

#[test]
fn test_org_lint_display_and_goto() {
    let (ev, editor) = setup();
    set_text(&editor, SAMPLE);
    let source = editor.name();
    run(&ev, &editor, "(org-lint)");
    assert_eq!(editor.name(), "*Org Lint*");
    let text = editor.text();
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines[0], "Line  Trust  Warning  Checker");
    assert_eq!(
        lines[1],
        "2     nil    Spurious CATEGORY keyword.  Set :CATEGORY: property instead  deprecated-category-setup"
    );
    assert_eq!(lines.len(), 5);
    // RET: その行へ移る
    run(
        &ev,
        &editor,
        "(goto-char (org-line-start (org-lines) 5)) (org-lint-goto)",
    );
    assert_eq!(editor.name(), source);
    assert_eq!(editor.current_line(), "#+BEGIN_SRC");
    // 見出しの行では, 移れない
    run(&ev, &editor, "(org-lint)");
    run(&ev, &editor, "(goto-char 1)");
    assert!(fail(&ev, &editor, "(org-lint-goto)").contains("No report on this line"));
}

#[test]
fn test_org_lint_clean_buffer() {
    let (ev, editor) = setup();
    set_text(&editor, "* Heading\ntext\n");
    assert_eq!(run(&ev, &editor, "(org-lint-reports)"), "()");
    run(&ev, &editor, "(org-lint)");
    assert_eq!(editor.text(), "Line  Trust  Warning  Checker\n");
}
