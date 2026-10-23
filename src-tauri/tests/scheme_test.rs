//! Scheme の評価器 (`scheme`) のテスト. エラーの経路と, 値の表示を確かめる.

use rseditor::scheme::evaluator::Evaluator;
use rseditor::scheme::parser::Parser;
use rseditor::scheme::value::Value;

/// 評価して, 失敗の内容を返す.
fn eval_err(code: &str) -> String {
    Evaluator::new().eval(code).unwrap_err()
}

#[test]
fn test_evaluator_reports_the_errors() {
    assert_eq!(eval_err("(1 2)"), "First element must be a symbol");
    assert_eq!(eval_err("(-)"), "- expects at least one argument");
    assert_eq!(eval_err("(- \"a\")"), "- expects numbers");
    assert_eq!(eval_err("(- 1 \"a\")"), "- expects numbers");
    assert_eq!(eval_err("(* 2 #t)"), "* expects numbers");
    assert_eq!(eval_err("(/ 1)"), "/ expects at least two arguments");
    assert_eq!(eval_err("(/ #f 1)"), "/ expects numbers");
    assert_eq!(eval_err("(quote 1 2)"), "quote expects 1 argument");
}

#[test]
fn test_parser_reads_escapes_and_trailing_spaces() {
    let values = Parser::new("\"a\\tb\\\"c\\\\\" ").parse().unwrap();
    assert_eq!(values, vec![Value::String("a\tb\"c\\".to_string())]);
    // マイナスだけは, 記号
    assert_eq!(
        Parser::new("-").parse().unwrap(),
        vec![Value::Symbol("-".to_string())]
    );
}

#[test]
fn test_parser_reports_the_errors() {
    let err = |code: &str| Parser::new(code).parse().unwrap_err();
    assert_eq!(err("@"), "Unexpected character: @");
    assert_eq!(err("\"a\\x\""), "Invalid escape sequence");
    assert_eq!(err("\"abc"), "Unclosed string");
    assert_eq!(err("1.2.3"), "Invalid number: 1.2.3");
}

#[test]
fn test_value_display_and_equality_of_nil_and_functions() {
    assert_eq!(Value::Nil.to_string(), "nil");
    assert_eq!(
        Value::Function("car".to_string()).to_string(),
        "#<procedure:car>"
    );
    assert_eq!(Value::Nil, Value::Nil);
    assert_ne!(Value::Nil, Value::Boolean(false));
}
