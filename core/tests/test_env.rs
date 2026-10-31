//! `scheme::env` のテスト.

use std::sync::Arc;

use rseditor_core::scheme::env::*;
use rseditor_core::scheme::value::{Lambda, Value};
#[test]
fn looks_up_the_outer_environments() {
    let global = Env::new();
    global.define("a", Value::Number(1.0));
    let inner = global.child();
    inner.define("b", Value::Number(2.0));
    assert_eq!(inner.get("a"), Some(Value::Number(1.0)));
    assert_eq!(inner.get("b"), Some(Value::Number(2.0)));
    assert_eq!(global.get("b"), None);
}

#[test]
fn set_changes_the_nearest_variable() {
    let global = Env::new();
    global.define("a", Value::Number(1.0));
    let inner = global.child();
    assert!(inner.set("a", Value::Number(3.0)));
    assert_eq!(global.get("a"), Some(Value::Number(3.0)));
    assert!(!inner.set("none", Value::Nil));
}

#[test]
fn collect_breaks_the_cycles_that_cannot_be_reached() {
    let global = Env::new();
    let lambda = |env: &Env| {
        Value::Lambda(Arc::new(Lambda {
            name: None,
            params: Vec::new(),
            rest: None,
            body: Vec::new(),
            env: env.clone(),
        }))
    };
    // 環境と手続きの輪 (一番外側から, たどれない)
    let lost = global.child();
    lost.define("f", lambda(&lost));
    let lost_alive = lost.alive_probe();
    drop(lost);
    // 一番外側から, リストの中の手続きでたどれる輪
    let kept = global.child();
    kept.define("g", lambda(&kept));
    global.define("h", Value::List(vec![lambda(&kept), Value::Nil].into()));
    drop(kept);
    assert_eq!(global.live_frames(), 2);

    global.collect();
    assert!(!lost_alive());
    assert_eq!(global.live_frames(), 1);
    global.define("h", Value::Nil);
    global.collect();
    assert_eq!(global.live_frames(), 0);
}

#[test]
fn child_forgets_the_released_frames() {
    let global = Env::new();
    for _ in 0..PRUNE_AT * 3 {
        global.child();
    }
    assert!(global.registered_frames() <= PRUNE_AT + 1);
    assert_eq!(global.live_frames(), 0);
}
