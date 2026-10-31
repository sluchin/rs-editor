//! `scheme::host` のテスト.

use std::time::Duration;

use rseditor_core::scheme::host::*;
use rseditor_core::scheme::value::Value;

use serde_json::json;
use std::sync::Arc;

#[test]
fn no_host_fails() {
    assert_eq!(
        NoHost.call("point", vec![]),
        Err("No editor is attached (point)".to_string())
    );
}

#[test]
fn converts_values_to_json_and_back() {
    let value = Value::List(
        vec![
            Value::Nil,
            Value::Boolean(true),
            Value::Number(1.5),
            Value::String("a".to_string()),
            Value::Symbol("b".to_string()),
            Value::Function("car".to_string()),
        ]
        .into(),
    );
    let json = to_json(&value);
    assert_eq!(json, json!([null, true, 1.5, "a", "b", "#<procedure:car>"]));
    assert_eq!(
        from_json(json),
        Value::List(
            vec![
                Value::Nil,
                Value::Boolean(true),
                Value::Number(1.5),
                Value::String("a".to_string()),
                Value::String("b".to_string()),
                Value::String("#<procedure:car>".to_string()),
            ]
            .into()
        )
    );
    assert_eq!(
        from_json(json!({"a": 1})),
        Value::String("{\"a\":1}".to_string())
    );
}

#[test]
fn bridge_passes_the_reply_to_the_request() {
    let bridge = Arc::new(Bridge::new(Duration::from_secs(5)));
    let answerer = Arc::clone(&bridge);
    let host = BridgeHost::new(&bridge, move |request: EditorRequest| {
        assert_eq!(request.op, "insert");
        assert_eq!(request.args, vec![json!("x")]);
        let answerer = Arc::clone(&answerer);
        std::thread::spawn(move || {
            // 待っている頼みに, 返事が届くまで送り直す
            while !answerer.reply(request.id, json!(3), None) {
                std::thread::yield_now();
            }
        });
        Ok(())
    });
    assert_eq!(
        host.call("insert", vec![Value::String("x".to_string())]),
        Ok(Value::Number(3.0))
    );
}

#[test]
fn bridge_passes_the_error_of_the_reply() {
    let bridge = Bridge::new(Duration::from_secs(5));
    let result = std::thread::scope(|scope| {
        scope.spawn(|| {
            while !bridge.reply(1, json!(null), Some("bad".to_string())) {
                std::thread::yield_now();
            }
        });
        bridge.request("point", vec![], &|_| Ok(()))
    });
    assert_eq!(result, Err("bad".to_string()));
}

#[test]
fn bridge_waits_longer_for_the_input() {
    // 返事を待つ長さが 0 でも, 入力 (read-string) は待つ
    let bridge = Bridge::new(Duration::ZERO);
    let result = std::thread::scope(|scope| {
        scope.spawn(|| {
            std::thread::sleep(Duration::from_millis(20));
            while !bridge.reply(1, json!("abc"), None) {
                std::thread::yield_now();
            }
        });
        bridge.request("read-string", vec![], &|_| Ok(()))
    });
    assert_eq!(result, Ok(Value::String("abc".to_string())));
}

#[test]
fn bridge_fails_when_sending_fails_or_nobody_answers() {
    let bridge = Bridge::new(Duration::from_millis(10));
    assert_eq!(
        bridge.request("point", vec![], &|_| Err("closed".to_string())),
        Err("closed".to_string())
    );
    assert_eq!(
        bridge.request("point", vec![], &|_| Ok(())),
        Err("The editor did not answer (point)".to_string())
    );
    // 時間切れの後の返事は, 捨てる
    assert!(!bridge.reply(2, json!(1), None));
}
