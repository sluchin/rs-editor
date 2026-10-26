//! Scheme の評価器, パーサ, 値の表示の, 境界の場合のテスト.

use rseditor::scheme::evaluator::Evaluator;
use rseditor::scheme::parser::Parser;
use rseditor::scheme::value::Value;

/// 評価して, 結果を返す.
///
/// # Arguments
///
/// * `code` - コード.
///
/// # Returns
///
/// 結果 (失敗したら panic する).
fn eval(code: &str) -> String {
    Evaluator::new().eval(code).unwrap()
}

/// 評価して, 失敗の内容を返す.
///
/// # Arguments
///
/// * `code` - コード.
///
/// # Returns
///
/// 失敗の内容 (成功したら panic する).
fn eval_err(code: &str) -> String {
    Evaluator::new().eval(code).unwrap_err()
}

/// 読み込んで, 式を返す.
///
/// # Arguments
///
/// * `code` - コード.
///
/// # Returns
///
/// 式 (失敗したら panic する).
fn parse(code: &str) -> Vec<Value> {
    Parser::new(code).parse().unwrap()
}

/// 読み込んで, 失敗の内容を返す.
///
/// # Arguments
///
/// * `code` - コード.
///
/// # Returns
///
/// 失敗の内容 (成功したら panic する).
fn parse_err(code: &str) -> String {
    Parser::new(code).parse().unwrap_err()
}

/// 数のリストの値を作る.
///
/// # Arguments
///
/// * `numbers` - 数.
///
/// # Returns
///
/// リスト.
fn number_list(numbers: &[f64]) -> Value {
    Value::List(numbers.iter().map(|n| Value::Number(*n)).collect())
}

/// Scheme 評価器のエッジケーステスト.
mod evaluator_edge_cases {
    use super::*;

    #[test]
    fn test_arithmetic_with_large_numbers() {
        // 大きな数でも, 整数として表示する.
        assert_eq!(eval("(+ 10000000000 10000000000)"), "20000000000");
    }

    #[test]
    fn test_arithmetic_with_negative_numbers() {
        assert_eq!(eval("(+ -5 3)"), "-2");
        assert_eq!(eval("(- -5 3)"), "-8");
        assert_eq!(eval("(* -5 3)"), "-15");
        assert_eq!(eval("(/ -20 4)"), "-5");
    }

    #[test]
    fn test_arithmetic_with_floats() {
        // 浮動小数点数の誤差があるので, 差で比べる.
        assert_eq!(eval("(< (abs (- (+ 0.1 0.2) 0.3)) 0.0000000001)"), "#t");
        assert_eq!(eval("(* 0.5 3)"), "1.5");
    }

    #[test]
    fn test_zero_operations() {
        assert_eq!(eval("(+ 0 5)"), "5");
        assert_eq!(eval("(* 0 100)"), "0");
        assert_eq!(eval("(- 5 0)"), "5");
    }

    #[test]
    fn test_multiple_operations_left_to_right() {
        // (- 10 2 3) は (- (- 10 2) 3).
        assert_eq!(eval("(- 10 2 3)"), "5");
        assert_eq!(eval("(/ 100 5 2)"), "10");
    }

    #[test]
    fn test_division_precision() {
        assert!(eval("(/ 10 3)").starts_with("3.333"));
    }

    #[test]
    fn test_multiplication_order_independence() {
        assert_eq!(eval("(= (* 3 4) (* 4 3))"), "#t");
        assert_eq!(eval("(= (* 3 4 5) (* 5 4 3))"), "#t");
    }

    #[test]
    fn test_addition_order_independence() {
        assert_eq!(eval("(= (+ 1 2) (+ 2 1))"), "#t");
        assert_eq!(eval("(= (+ 1 2 3) (+ 3 2 1))"), "#t");
    }

    #[test]
    fn test_subtraction_order_dependent() {
        assert_eq!(eval("(- 5 3)"), "2");
        assert_eq!(eval("(- 3 5)"), "-2");
    }

    #[test]
    fn test_division_order_dependent() {
        assert_eq!(eval("(/ 20 4)"), "5");
        assert_eq!(eval("(/ 4 20)"), "0.2");
    }

    #[test]
    fn test_operators_without_arguments() {
        // 加算と乗算は, 単位元を返す (Scheme と同じ). 減算と除算は, 引数が要る.
        assert_eq!(eval("(+)"), "0");
        assert_eq!(eval("(*)"), "1");
        assert_eq!(eval_err("(-)"), "- expects at least one argument");
        assert_eq!(eval_err("(/)"), "/ expects at least two arguments");
    }

    #[test]
    fn test_operator_with_wrong_type_errors() {
        assert_eq!(eval_err("(+ \"hello\" 1)"), "+ expects numbers");
    }

    #[test]
    fn test_quote_preserves_structure() {
        // 評価せずに, そのまま返す.
        assert_eq!(eval("(quote (1 2 3))"), "(1 2 3)");
        assert_eq!(eval("'(+ 1 2)"), "(+ 1 2)");
    }

    #[test]
    fn test_list_operation_with_evaluated_elements() {
        assert_eq!(eval("(list (+ 1 1) (* 2 3))"), "(2 6)");
    }

    #[test]
    fn test_empty_list() {
        assert_eq!(eval("()"), "()");
        assert_eq!(eval("(null? '())"), "#t");
    }

    #[test]
    fn test_nested_list_operations() {
        assert_eq!(eval("(list (list 1 2) (list 3 4))"), "((1 2) (3 4))");
    }

    #[test]
    fn test_mixed_numeric_types() {
        // 整数と小数を混ぜても, 計算できる.
        assert_eq!(eval("(+ 1 2.5 3)"), "6.5");
    }
}

/// Scheme パーサーのエッジケーステスト.
mod parser_edge_cases {
    use super::*;

    #[test]
    fn test_parse_empty_list() {
        assert_eq!(parse("()"), vec![Value::List(vec![].into())]);
    }

    #[test]
    fn test_parse_nested_lists() {
        assert_eq!(
            parse("((1 2) (3 4))"),
            vec![Value::List(
                vec![number_list(&[1.0, 2.0]), number_list(&[3.0, 4.0]),].into()
            )]
        );
    }

    #[test]
    fn test_parse_symbol_with_special_chars() {
        let symbols = ["foo-bar", "foo_bar", "foo?", "foo!"];
        assert_eq!(
            parse(&symbols.join(" ")),
            symbols
                .iter()
                .map(|s| Value::Symbol(s.to_string()))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_parse_string_with_escapes() {
        assert_eq!(
            parse(r#""hello\nworld\t\"q\" \\""#),
            vec![Value::String("hello\nworld\t\"q\" \\".to_string())]
        );
    }

    #[test]
    fn test_parse_number_formats() {
        assert_eq!(
            parse("42 -42 3.5 -3.5"),
            vec![
                Value::Number(42.0),
                Value::Number(-42.0),
                Value::Number(3.5),
                Value::Number(-3.5),
            ]
        );
    }

    #[test]
    fn test_parse_boolean_formats() {
        assert_eq!(
            parse("#t #f"),
            vec![Value::Boolean(true), Value::Boolean(false)]
        );
    }

    #[test]
    fn test_parse_comments() {
        // `;` から行末までは, 読み飛ばす (最後の行のコメントも).
        assert_eq!(
            parse("; first\n1 ; second\n2 ; last"),
            vec![Value::Number(1.0), Value::Number(2.0)]
        );
    }

    #[test]
    fn test_parse_whitespace_variations() {
        assert_eq!(
            parse(" 1\t2\n3\r\n4 "),
            vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
                Value::Number(4.0),
            ]
        );
    }

    #[test]
    fn test_parse_multiple_expressions() {
        assert_eq!(parse("1 2 3").len(), 3);
        assert_eq!(parse("(+ 1 2) (* 3 4)").len(), 2);
    }

    #[test]
    fn test_parse_unclosed_list_error() {
        assert_eq!(parse_err("("), "Unclosed list");
        assert_eq!(parse_err("(1 (2)"), "Unclosed list");
    }

    #[test]
    fn test_parse_unclosed_string_error() {
        assert_eq!(parse_err("\"hello"), "Unclosed string");
    }

    #[test]
    fn test_parse_invalid_escape_sequence() {
        assert_eq!(parse_err(r#""hello\x""#), "Invalid escape sequence");
    }
}

/// Value 型の表示形式のテスト.
mod value_display_tests {
    use super::*;

    #[test]
    fn test_nil_display() {
        assert_eq!(Value::Nil.to_string(), "nil");
    }

    #[test]
    fn test_symbol_display() {
        assert_eq!(Value::Symbol("foo-bar".to_string()).to_string(), "foo-bar");
    }

    #[test]
    fn test_string_display_with_special_chars() {
        // 文字列は, 引用符を付けて, 特殊な文字をエスケープして表す. `display` では, そのまま.
        let value = Value::String("hello\nworld".to_string());
        assert_eq!(value.to_string(), r#""hello\nworld""#);
        assert_eq!(value.to_display(), "hello\nworld");
    }

    #[test]
    fn test_list_display_with_many_items() {
        assert_eq!(
            number_list(&[1.0, 2.0, 3.0, 4.0, 5.0]).to_string(),
            "(1 2 3 4 5)"
        );
    }

    #[test]
    fn test_deeply_nested_list_display() {
        assert_eq!(eval("'(((1 2) 3) 4)"), "(((1 2) 3) 4)");
    }
}
