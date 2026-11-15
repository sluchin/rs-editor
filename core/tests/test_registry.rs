//! 組み込みの手続きの登録表 (`registry.rs`) のテスト.

use rseditor_core::error::Error;
use rseditor_core::register;
use rseditor_core::scheme::evaluator::{Evaluator, Machine};
use rseditor_core::scheme::registry::{registry, Registry};
use rseditor_core::scheme::value::Value;

/// 何もしない手続き.
fn nothing(_machine: &mut Machine, _name: &str, _args: Vec<Value>) -> Result<Value, Error> {
    Ok(Value::Nil)
}

#[test]
fn test_register_and_get() {
    let mut registry = Registry::new();
    register!(registry, "a" => nothing, "b" => nothing);

    assert!(registry.get("a").is_some());
    assert!(registry.get("c").is_none());
    let mut names: Vec<&str> = registry.names().collect();
    names.sort();
    assert_eq!(names, ["a", "b"]);
}

#[test]
#[should_panic(expected = "duplicate builtin: a")]
fn test_register_duplicate_name_panics() {
    let mut registry = Registry::new();
    registry.register("a", nothing);
    registry.register("a", nothing);
}

#[test]
fn test_registry_has_the_migrated_procedures_and_defines_them() {
    let names: Vec<&str> = registry().names().collect();
    for name in [
        "add-hook",
        "remove-hook",
        "run-hooks",
        "run-hook-with-args",
        "ert-run-tests-batch",
        "ert-run-tests-batch-and-exit",
        "ert-test-boundp",
        "ert-delete-test",
        "ert-delete-all-tests",
        "ert-fail",
        "ert-skip",
    ] {
        assert!(names.contains(&name), "{name} is not registered");
    }
    // 評価器は, 表の手続きを定義し, 呼べる
    let evaluator = Evaluator::new();
    let output = evaluator
        .eval(
            "(define n 0) (define h '()) (define inc (lambda () (set! n (+ n 1)))) \
                 (add-hook 'h 'inc) (run-hooks 'h) n",
        )
        .unwrap();
    assert_eq!(output.lines().last(), Some("1"));
}
