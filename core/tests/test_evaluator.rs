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
        "Symbol's function definition is void: unknown-op"
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

#[test]
fn condition_case_binds_the_emacs_data_of_each_kind() {
    let eval = Evaluator::new();
    // (cdr e) は、種類ごとの Emacs のデータ. error-message-string は、それから文を作る
    for (code, data, text) in [
        ("(error \"boom %d\" 3)", "(\"boom 3\")", "boom 3"),
        ("(error \"End of buffer\")", "()", "End of buffer"),
        (
            "(error \"Beginning of buffer\")",
            "()",
            "Beginning of buffer",
        ),
        ("(/ 1 0)", "()", "Arithmetic error"),
        ("(error \"Quit\")", "()", "Quit"),
        (
            "(error \"Search failed: \\\"zz\\\"\")",
            "(\"zz\")",
            "Search failed: \"zz\"",
        ),
        (
            "(error \"Search failed: zz\")",
            "(\"zz\")",
            "Search failed: \"zz\"",
        ),
        ("(error \"Search failed\")", "(\"\")", "Search failed: \"\""),
        (
            "undefined-var",
            "(undefined-var)",
            "Symbol's value as variable is void: undefined-var",
        ),
        (
            "(undefined-fn 1)",
            "(undefined-fn)",
            "Symbol's function definition is void: undefined-fn",
        ),
        (
            "(error \"Opening input file: No such file or directory, /x/y\")",
            "(\"Opening input file\" \"No such file or directory\" \"/x/y\")",
            "Opening input file: No such file or directory, /x/y",
        ),
        (
            "(error \"Cannot open load file: zz\")",
            "(\"Cannot open load file\" \"No such file or directory\" \"zz\")",
            "Cannot open load file: No such file or directory, zz",
        ),
        (
            "(error \"Opening input file\")",
            "(\"Opening input file\")",
            "Opening input file",
        ),
        (
            "(+ 1 \"a\")",
            "(number-or-marker-p \"a\")",
            "Wrong type argument: number-or-marker-p, \"a\"",
        ),
        ("(abs 'a)", "(numberp a)", "Wrong type argument: numberp, a"),
        ("(car 1)", "(listp 1)", "Wrong type argument: listp, 1"),
        (
            "(length 1)",
            "(sequencep 1)",
            "Wrong type argument: sequencep, 1",
        ),
        ("(member 1 2)", "(listp 2)", "Wrong type argument: listp, 2"),
        (
            "(string-length 1)",
            "(stringp 1)",
            "Wrong type argument: stringp, 1",
        ),
        (
            "(list-ref '(1) \"a\")",
            "(integerp \"a\")",
            "Wrong type argument: integerp, \"a\"",
        ),
        (
            "(error \"Wrong type argument: odd\")",
            "(\"Wrong type argument: odd\")",
            "Wrong type argument: odd",
        ),
        (
            "(error \"Wrong type argument: p, )\")",
            "(\"Wrong type argument: p, )\")",
            "Wrong type argument: p, )",
        ),
        (
            "(error \"Unbound variable\")",
            "(\"Unbound variable\")",
            "Unbound variable",
        ),
        (
            "(error \"Symbol's value as variable is void: v\")",
            "(v)",
            "Symbol's value as variable is void: v",
        ),
    ] {
        let code_data = format!("(condition-case e {code} ((error quit) (cdr e)))");
        assert_eq!(eval.eval(&code_data).unwrap(), data, "{code}");
        let code_text =
            format!("(condition-case e {code} ((error quit) (error-message-string e)))");
        assert_eq!(
            eval.eval(&code_text).unwrap(),
            format!("{text:?}").replace("\\'", "'"),
            "{code}"
        );
    }
    // メッセージを持たない変数 (種類だけ) でも、文になる
    assert_eq!(
        eval.eval("(error-message-string '(error))").unwrap(),
        "\"error\""
    );
}
