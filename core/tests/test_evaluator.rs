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
        builtins::call(&mut machine, "none", vec![]).map_err(|e| e.to_string()),
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
    // (car e) は, エラーの種類. error はメッセージだけで, 文の書き出しで種類は決まらない
    for (code, kind) in [
        ("(/ 1 0)", "arith-error"),
        ("(modulo 1 0)", "arith-error"),
        ("zzz", "void-variable"),
        ("(signal 'end-of-buffer '())", "end-of-buffer"),
        ("(signal 'search-failed '(\"x\"))", "search-failed"),
        ("(signal 'quit '())", "quit"),
        ("(user-error \"no\")", "user-error"),
        ("(error \"End of buffer\")", "error"),
        ("(error \"Quit\")", "error"),
        ("(error \"Search failed: x\")", "error"),
        ("(error \"boom\")", "error"),
    ] {
        let code = format!("(condition-case e {code} ((error quit) (car e)))");
        assert_eq!(eval.eval(&code).unwrap(), kind, "{code}");
    }
    // 種類の名前の節と, 変数が nil の節
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
fn condition_case_matches_the_error_conditions() {
    let eval = Evaluator::new();
    // 親の種類でも受ける (file-missing は file-error で, user-error は error で)
    for (code, handler) in [
        ("(signal 'file-missing '(\"a\" \"b\" \"c\"))", "file-error"),
        ("(signal 'file-already-exists '(\"a\" \"c\"))", "file-error"),
        ("(signal 'text-read-only '())", "buffer-read-only"),
        ("(user-error \"x\")", "error"),
        ("(signal 'args-out-of-range '(1))", "error"),
        ("(signal 'quit '())", "quit"),
        ("(signal 'quit '())", "t"),
        ("(signal 'quit '())", "(error quit)"),
        ("(signal 'whatever '(1))", "t"),
    ] {
        let code = format!("(condition-case e {code} ({handler} 'caught))");
        assert_eq!(eval.eval(&code).unwrap(), "caught", "{code}");
    }
    // 受けないもの: quit は error で, 知らない種類も error では受けない
    for (code, handler) in [
        ("(signal 'quit '())", "error"),
        ("(signal 'whatever '(1))", "error"),
        ("(signal 'whatever '(1))", "whatever"),
        ("(error \"x\")", "user-error"),
        ("(signal 'file-error '(\"a\"))", "file-missing"),
    ] {
        let code = format!("(condition-case e {code} ({handler} 'caught))");
        assert!(eval.eval(&code).is_err(), "{code}");
    }
}

#[test]
fn signal_and_define_error() {
    let eval = Evaluator::new();
    // signal の種類とデータは, そのまま condition-case の変数になる
    assert_eq!(
        eval.eval("(condition-case e (signal 'my-error '(1 \"two\" three)) (t e))")
            .unwrap(),
        "(my-error 1 \"two\" three)"
    );
    // 種類が偽 (nil) で (種類 . データ) を渡すと, 投げ直せる
    assert_eq!(
        eval.eval("(condition-case e (condition-case e2 (/ 1 0) (error (signal #f e2))) (arith-error 'again))")
            .unwrap(),
        "again"
    );
    assert_eq!(
        eval.eval("(signal 1 '())").unwrap_err(),
        "Wrong type argument: symbolp, 1"
    );
    // define-error: 文と, 親の種類
    eval.eval("(define-error 'my-problem \"My problem\")")
        .unwrap();
    eval.eval("(define-error 'my-sub \"My sub\" 'my-problem)")
        .unwrap();
    eval.eval("(define-error 'my-multi \"My multi\" '(my-problem file-error))")
        .unwrap();
    for (code, expected) in [
        (
            "(condition-case e (signal 'my-problem '(1 \"x\")) (error (error-message-string e)))",
            "\"My problem: 1, \\\"x\\\"\"",
        ),
        (
            "(condition-case e (signal 'my-sub '()) (my-problem 'parent))",
            "parent",
        ),
        (
            "(condition-case e (signal 'my-sub '()) (error 'any))",
            "any",
        ),
        (
            "(condition-case e (signal 'my-multi '(\"f\")) (file-error 'file))",
            "file",
        ),
        (
            "(condition-case e (signal 'my-problem '()) (my-sub 'sub))",
            "x",
        ),
    ] {
        let result = eval.eval(code);
        if expected == "x" {
            assert!(result.is_err(), "{code}");
        } else {
            assert_eq!(result.unwrap(), expected, "{code}");
        }
    }
    assert_eq!(
        eval.eval("(define-error 1 2)").unwrap_err(),
        "define-error expects a name and a message"
    );
    // error-message-string の種類の無いものは, そのまま
    assert_eq!(
        eval.eval("(error-message-string \"plain\")").unwrap(),
        "\"plain\""
    );
    assert_eq!(
        eval.eval("(error-message-string '(error))").unwrap(),
        "\"peculiar error\""
    );
}

#[test]
fn condition_case_binds_the_emacs_data_of_each_kind() {
    let eval = Evaluator::new();
    // (cdr e) は, 種類ごとの Emacs のデータ. error-message-string は, それから文を作る
    for (code, data, text) in [
        ("(error \"boom %d\" 3)", "(\"boom 3\")", "boom 3"),
        ("(signal 'end-of-buffer '())", "()", "End of buffer"),
        (
            "(signal 'beginning-of-buffer '())",
            "()",
            "Beginning of buffer",
        ),
        ("(/ 1 0)", "()", "Arithmetic error"),
        ("(signal 'quit '())", "()", "Quit"),
        (
            "(signal 'search-failed '(\"zz\"))",
            "(\"zz\")",
            "Search failed: \"zz\"",
        ),
        ("(signal 'search-failed '())", "()", "Search failed"),
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
            "(signal 'file-missing '(\"Opening input file\" \"No such file or directory\" \"/x/y\"))",
            "(\"Opening input file\" \"No such file or directory\" \"/x/y\")",
            "Opening input file: No such file or directory, /x/y",
        ),
        (
            "(load \"zz-missing\")",
            "(\"Cannot open load file\" \"No such file or directory\" \"zz-missing\")",
            "Cannot open load file: No such file or directory, zz-missing",
        ),
        (
            "(signal 'file-missing '(\"Opening input file\"))",
            "(\"Opening input file\")",
            "Opening input file",
        ),
        (
            "(signal 'file-already-exists '(\"File exists\" \"/x\"))",
            "(\"File exists\" \"/x\")",
            "File exists: /x",
        ),
        (
            "(signal 'scan-error '(\"Unbalanced parentheses\" 1 5))",
            "(\"Unbalanced parentheses\" 1 5)",
            "Scan error: \"Unbalanced parentheses\", 1, 5",
        ),
        (
            "(signal 'invalid-regexp '(\"Unmatched [\"))",
            "(\"Unmatched [\")",
            "Invalid regexp: \"Unmatched [\"",
        ),
        (
            "(signal 'end-of-file '(\"Error reading from stdin\"))",
            "(\"Error reading from stdin\")",
            "End of file during parsing: Error reading from stdin",
        ),
        ("(user-error \"a\" )", "(\"a\")", "a"),
        ("(signal 'user-error '(\"a\" \"b\"))", "(\"a\" \"b\")", "a, b"),
        (
            "(signal 'buffer-read-only '(b))",
            "(b)",
            "Buffer is read-only: b",
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
            "(signal 'wrong-type-argument '(odd))",
            "(odd)",
            "Wrong type argument: odd",
        ),
        (
            "(signal 'args-out-of-range (list (string->symbol \"#<buffer b>\") 1 99))",
            "(#<buffer b> 1 99)",
            "Args out of range: #<buffer b>, 1, 99",
        ),
        (
            "(substring \"a, \\\"b\\\"\" 9)",
            "(\"a, \\\"b\\\"\" 9 nil)",
            "Args out of range: \"a, \\\"b\\\"\", 9, nil",
        ),
        (
            "(signal 'args-out-of-range '(\"x\"))",
            "(\"x\")",
            "Args out of range: \"x\"",
        ),
        ("(signal 'args-out-of-range '())", "()", "Args out of range"),
        (
            "(signal 'void-variable '(v))",
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
}

#[test]
fn length_counts_the_characters_of_a_string() {
    let eval = Evaluator::new();
    assert_eq!(eval.eval("(length \"aあb\")").unwrap(), "3");
    assert_eq!(eval.eval("(length '(1 2))").unwrap(), "2");
}
