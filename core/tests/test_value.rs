//! `scheme::value` のテスト.

use std::sync::Arc;

use rseditor_core::scheme::env::Env;
use rseditor_core::scheme::value::*;

/// 数のリストを作る.
fn numbers(items: &[f64]) -> List {
    items.iter().map(|n| Value::Number(*n)).collect()
}

#[test]
fn debug_shows_the_list_elements() {
    assert_eq!(
        format!("{:?}", Value::List(numbers(&[1.0, 2.0]).tail())),
        "List([Number(2.0)])"
    );
}

#[test]
fn values_are_send_and_sync() {
    fn assert<T: Send + Sync>() {}
    assert::<Value>();
}

#[test]
fn cons_shares_the_block_only_once() {
    let a = numbers(&[1.0, 2.0]);
    // 空きが無ければ写す. 写した配列は, 前に空きを持つ
    let b = a.cons(Value::Number(0.0));
    assert!(!a.shares_block_with(&b));
    // 前へ足していくと, 同じ配列の空きを使う
    let c = b.cons(Value::Number(-1.0));
    assert!(b.shares_block_with(&c));
    // 同じリストに, もう一度足すと (空きは取られている), 写す
    let d = b.cons(Value::Number(9.0));
    assert!(!b.shares_block_with(&d));
    // cdr で作ったリストの前は, 取られているので, 写す
    let e = c.tail().cons(Value::Number(7.0));
    assert!(!c.shares_block_with(&e));
    assert_eq!(*a, *numbers(&[1.0, 2.0]));
    assert_eq!(*b, *numbers(&[0.0, 1.0, 2.0]));
    assert_eq!(*c, *numbers(&[-1.0, 0.0, 1.0, 2.0]));
    assert_eq!(*d, *numbers(&[9.0, 0.0, 1.0, 2.0]));
    assert_eq!(*e, *numbers(&[7.0, 0.0, 1.0, 2.0]));
    // 空きを使い切ったら, また写す
    let mut list = List::from(vec![]);
    for i in 0..100 {
        list = list.cons(Value::Number(i as f64));
    }
    assert_eq!(list.len(), 100);
    assert_eq!(list[0], Value::Number(99.0));
    assert_eq!(list[99], Value::Number(0.0));
    assert_eq!(list.drop_front(200).len(), 0);
}

#[test]
fn cons_from_threads_claims_the_slot_once() {
    let base = numbers(&[1.0]).cons(Value::Number(0.0));
    let results: Vec<List> = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..8)
            .map(|i| {
                let base = base.clone();
                scope.spawn(move || base.cons(Value::Number(10.0 + i as f64)))
            })
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });
    for (i, list) in results.iter().enumerate() {
        assert_eq!(**list, *numbers(&[10.0 + i as f64, 0.0, 1.0]));
    }
    // 同じ配列の空きを取れたのは, 1 つだけ
    let shared = results
        .iter()
        .filter(|l| l.shares_block_with(&base))
        .count();
    assert_eq!(shared, 1);
}

#[test]
fn lists_drop_each_element_once() {
    let lambda = Arc::new(Lambda {
        name: None,
        params: vec![],
        rest: None,
        body: vec![],
        env: Env::new(),
    });
    {
        let mut list = List::from(vec![Value::Lambda(lambda.clone())]);
        for _ in 0..10 {
            list = list.cons(Value::Lambda(lambda.clone()));
        }
        let forked = list.tail().cons(Value::Lambda(lambda.clone()));
        assert_eq!(Arc::strong_count(&lambda), 1 + 11 + 11);
        drop(forked);
        assert_eq!(Arc::strong_count(&lambda), 1 + 11);
    }
    assert_eq!(Arc::strong_count(&lambda), 1);
}

#[test]
fn displays_integer_numbers_without_decimal_point() {
    assert_eq!(Value::Number(42.0).to_string(), "42");
}

#[test]
fn displays_large_numbers_without_overflow() {
    // i64 に収まる整数は, 整数で書く (-0 は 0)
    assert_eq!(
        Value::Number(-9.223372036854775e18).to_string(),
        "-9223372036854774784"
    );
    assert_eq!(Value::Number(-0.0).to_string(), "0");
    // 収まらない数は, i64 にしない (飽和して値が変わらないように)
    assert_eq!(Value::Number(1e19).to_string(), "10000000000000000000");
    assert_eq!(Value::Number(-1e19).to_string(), "-10000000000000000000");
    assert_eq!(Value::Number(f64::INFINITY).to_string(), "inf");
    assert_eq!(Value::Number(f64::NAN).to_string(), "NaN");
}

#[test]
fn displays_fractional_numbers() {
    assert_eq!(Value::Number(3.5).to_string(), "3.5");
}

#[test]
fn displays_booleans() {
    assert_eq!(Value::Boolean(true).to_string(), "#t");
    assert_eq!(Value::Boolean(false).to_string(), "#f");
}

#[test]
fn displays_strings_with_quotes() {
    assert_eq!(Value::String("hi".to_string()).to_string(), "\"hi\"");
    assert_eq!(
        Value::String("a\"b\n".to_string()).to_string(),
        "\"a\\\"b\\n\""
    );
    assert_eq!(Value::String("hi".to_string()).to_display(), "hi");
    assert_eq!(Value::Number(1.0).to_display(), "1");
}

#[test]
fn displays_lists_with_spaces() {
    let list = Value::List(vec![Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)].into());
    assert_eq!(list.to_string(), "(1 2 3)");
}

#[test]
fn displays_nested_lists() {
    let list = Value::List(
        vec![
            Value::Symbol("a".to_string()),
            Value::List(vec![Value::Number(1.0), Value::Number(2.0)].into()),
        ]
        .into(),
    );
    assert_eq!(list.to_string(), "(a (1 2))");
}

#[test]
fn displays_lambdas_with_and_without_names() {
    let lambda = |name: Option<&str>| {
        Value::Lambda(Arc::new(Lambda {
            name: name.map(str::to_string),
            params: vec![],
            rest: None,
            body: vec![],
            env: Env::new(),
        }))
    };
    assert_eq!(lambda(Some("f")).to_string(), "#<procedure:f>");
    assert_eq!(lambda(None).to_string(), "#<procedure>");
    assert_eq!(
        format!("{:?}", lambda(Some("f"))),
        "Lambda(Lambda(Some(\"f\")))"
    );
}

#[test]
fn equality_holds_across_variants() {
    assert_eq!(Value::Number(1.0), Value::Number(1.0));
    assert_ne!(Value::Number(1.0), Value::Boolean(true));
    assert_eq!(
        Value::List(vec![Value::Number(1.0)].into()),
        Value::List(vec![Value::Number(1.0)].into())
    );
    assert_eq!(
        Value::Function("car".to_string()),
        Value::Function("car".to_string())
    );
}

#[test]
fn only_false_is_false() {
    assert!(!Value::Boolean(false).is_true());
    assert!(Value::Nil.is_true());
    assert!(Value::List(vec![].into()).is_true());
}
