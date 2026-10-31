//! `scheme::parser` のテスト.

use rseditor_core::scheme::evaluator::MAX_DEPTH;
use rseditor_core::scheme::parser::*;
use rseditor_core::scheme::value::Value;
fn parse_one(input: &str) -> Value {
    Parser::new(input)
        .parse()
        .unwrap()
        .into_iter()
        .next()
        .unwrap()
}

#[test]
fn parses_integers() {
    assert_eq!(parse_one("42"), Value::Number(42.0));
}

#[test]
fn parses_negative_numbers() {
    assert_eq!(parse_one("-3.5"), Value::Number(-3.5));
}

#[test]
fn parses_symbols() {
    assert_eq!(parse_one("foo-bar?"), Value::Symbol("foo-bar?".to_string()));
}

#[test]
fn parses_booleans() {
    assert_eq!(parse_one("#t"), Value::Boolean(true));
    assert_eq!(parse_one("#f"), Value::Boolean(false));
}

#[test]
fn parses_strings_with_escapes() {
    assert_eq!(
        parse_one(r#""hello\nworld""#),
        Value::String("hello\nworld".to_string())
    );
}

#[test]
fn parses_nested_lists() {
    let value = parse_one("(+ 1 (* 2 3))");
    match value {
        Value::List(items) => {
            assert_eq!(items.len(), 3);
            assert_eq!(items[0], Value::Symbol("+".to_string()));
            assert_eq!(items[1], Value::Number(1.0));
            assert_eq!(
                items[2],
                Value::List(
                    vec![
                        Value::Symbol("*".to_string()),
                        Value::Number(2.0),
                        Value::Number(3.0),
                    ]
                    .into()
                )
            );
        }
        _ => panic!("expected a list"),
    }
}

#[test]
fn parses_empty_list() {
    assert_eq!(parse_one("()"), Value::List(vec![].into()));
}

#[test]
fn skips_comments() {
    let values = Parser::new("; a comment\n(+ 1 1)").parse().unwrap();
    assert_eq!(values.len(), 1);
}

#[test]
fn parses_multiple_top_level_forms() {
    let values = Parser::new("1 2 3").parse().unwrap();
    assert_eq!(
        values,
        vec![Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)]
    );
}

#[test]
fn parses_quote_and_dots() {
    assert_eq!(
        parse_one("'(a . b)"),
        Value::List(
            vec![
                Value::Symbol("quote".to_string()),
                Value::List(
                    vec![
                        Value::Symbol("a".to_string()),
                        Value::Symbol(".".to_string()),
                        Value::Symbol("b".to_string()),
                    ]
                    .into()
                ),
            ]
            .into()
        )
    );
    assert_eq!(parse_one(".5"), Value::Number(0.5));
    assert_eq!(parse_one("a.b:c"), Value::Symbol("a.b:c".to_string()));
}

#[test]
fn errors_on_unclosed_list() {
    assert!(Parser::new("(+ 1 2").parse().is_err());
}

#[test]
fn errors_on_invalid_boolean() {
    assert!(Parser::new("#x").parse().is_err());
    assert!(Parser::new("#tx").parse().is_err());
}

#[test]
fn parses_long_booleans() {
    assert_eq!(parse_one("#true"), Value::Boolean(true));
    assert_eq!(parse_one("#false"), Value::Boolean(false));
}

#[test]
fn parses_atoms_like_emacs() {
    // 数として読めない, 数字で始まる並びは, 記号
    assert_eq!(parse_one("1+"), Value::Symbol("1+".to_string()));
    assert_eq!(parse_one("1.2.3"), Value::Symbol("1.2.3".to_string()));
    assert_eq!(parse_one("+5"), Value::Number(5.0));
    assert_eq!(parse_one("-.5"), Value::Number(-0.5));
    assert_eq!(parse_one("1e3"), Value::Number(1000.0));
    // 数の形でないものは, 数として読めても記号
    assert_eq!(parse_one("inf"), Value::Symbol("inf".to_string()));
    assert_eq!(parse_one("-"), Value::Symbol("-".to_string()));
    // Unicode の数字の文字は, 記号
    assert_eq!(parse_one("①"), Value::Symbol("①".to_string()));
    assert_eq!(parse_one("x①"), Value::Symbol("x①".to_string()));
}

#[test]
fn errors_on_too_deep_nesting() {
    // 上限まで潜るので, 評価と同じく, 大きなスタックで読む
    std::thread::Builder::new()
        .stack_size(256 * 1024 * 1024)
        .spawn(|| {
            let deep = format!("{}{}", "(".repeat(MAX_DEPTH + 1), ")".repeat(MAX_DEPTH + 1));
            assert_eq!(Parser::new(&deep).parse().unwrap_err(), "Too deeply nested");
            let quoted = format!("{}x", "'".repeat(MAX_DEPTH + 1));
            assert_eq!(
                Parser::new(&quoted).parse().unwrap_err(),
                "Too deeply nested"
            );
            // 上限までは, 読める
            let ok = format!("{}{}", "(".repeat(MAX_DEPTH), ")".repeat(MAX_DEPTH));
            assert!(Parser::new(&ok).parse().is_ok());
        })
        .unwrap()
        .join()
        .unwrap();
}
