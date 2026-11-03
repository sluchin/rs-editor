//! `scheme::evaluator` のテスト.

use rseditor_core::scheme::builtins;
use rseditor_core::scheme::evaluator::*;
use rseditor_core::scheme::host::NoHost;
#[test]
fn evaluates_addition() {
    let eval = Evaluator::new();
    assert_eq!(eval.eval("(+ 1 2 3)").unwrap(), "6");
}

#[test]
fn evaluates_subtraction() {
    let eval = Evaluator::new();
    assert_eq!(eval.eval("(- 10 3)").unwrap(), "7");
}

#[test]
fn evaluates_unary_minus_as_negation() {
    let eval = Evaluator::new();
    assert_eq!(eval.eval("(- 5)").unwrap(), "-5");
}

#[test]
fn evaluates_multiplication() {
    let eval = Evaluator::new();
    assert_eq!(eval.eval("(* 2 3 4)").unwrap(), "24");
}

#[test]
fn evaluates_division() {
    let eval = Evaluator::new();
    assert_eq!(eval.eval("(/ 20 4)").unwrap(), "5");
}

#[test]
fn division_by_zero_is_an_error() {
    let eval = Evaluator::new();
    assert!(eval.eval("(/ 1 0)").is_err());
}

#[test]
fn evaluates_nested_expressions() {
    let eval = Evaluator::new();
    assert_eq!(eval.eval("(+ 1 (* 2 3))").unwrap(), "7");
}

#[test]
fn evaluates_quote() {
    let eval = Evaluator::new();
    assert_eq!(eval.eval("(quote (a b c))").unwrap(), "(a b c)");
    assert_eq!(eval.eval("'(a b c)").unwrap(), "(a b c)");
}

#[test]
fn evaluates_list() {
    let eval = Evaluator::new();
    assert_eq!(eval.eval("(list 1 2 3)").unwrap(), "(1 2 3)");
}

#[test]
fn evaluates_multiple_top_level_forms_line_by_line() {
    let eval = Evaluator::new();
    assert_eq!(eval.eval("(+ 1 1)\n(+ 2 2)").unwrap(), "2\n4");
}

#[test]
fn empty_input_evaluates_to_empty_string() {
    let eval = Evaluator::new();
    assert_eq!(eval.eval("").unwrap(), "");
}

#[test]
fn unknown_operator_is_an_error() {
    let eval = Evaluator::new();
    assert_eq!(
        eval.eval("(unknown-op 1 2)").unwrap_err(),
        "Unbound variable: unknown-op"
    );
}

#[test]
fn wrong_argument_type_is_an_error() {
    let eval = Evaluator::new();
    assert!(eval.eval(r#"(+ 1 "two")"#).is_err());
}

#[test]
fn unknown_builtin_is_an_error() {
    // 組み込みの手続きの値は, 名前の一覧からしか作らないので, 評価では起きない
    let evaluator = Evaluator::new();
    let mut machine = Machine::new(&evaluator, &NoHost);
    assert_eq!(
        builtins::call(&mut machine, "none", vec![]),
        Err("Unknown procedure: none".to_string())
    );
}

#[test]
fn remembers_definitions_between_evaluations() {
    let eval = Evaluator::new();
    assert_eq!(eval.eval("(define x 10)").unwrap(), "x");
    assert_eq!(eval.eval("(* x 2)").unwrap(), "20");
}

#[test]
fn condition_case_tells_the_kind_of_the_error() {
    let eval = Evaluator::new();
    // (car e) は、エラーの種類. 知らないメッセージは error
    for (code, kind) in [
        ("(/ 1 0)", "arith-error"),
        ("(modulo 1 0)", "arith-error"),
        ("zzz", "void-variable"),
        ("(error \"End of buffer\")", "end-of-buffer"),
        ("(error \"Search failed: \\\"x\\\"\")", "search-failed"),
        ("(error \"boom\")", "error"),
    ] {
        let code = format!("(condition-case e {code} (error (car e)))");
        assert_eq!(eval.eval(&code).unwrap(), kind, "{code}");
    }
    // 種類の名前の節と、変数が nil の節
    assert_eq!(
        eval.eval("(condition-case e (/ 1 0) (end-of-buffer 1) (arith-error 2))")
            .unwrap(),
        "2"
    );
    assert_eq!(
        eval.eval("(condition-case nil (/ 1 0) (error 3))").unwrap(),
        "3"
    );
    // 合わない節は通り抜ける
    assert!(eval
        .eval("(condition-case nil (/ 1 0) (end-of-buffer 1))")
        .is_err());
}
