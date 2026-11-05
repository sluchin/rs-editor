//! ロックの補助 (`core/src/sync.rs`) のテスト.

use std::sync::{Arc, Mutex};

use rseditor_core::sync::LockExt;

#[test]
fn test_lock_recover_works_after_a_thread_panicked_holding_the_lock() {
    let shared = Arc::new(Mutex::new(vec![1]));
    let thread = {
        let shared = shared.clone();
        std::thread::spawn(move || {
            let mut guard = shared.lock().unwrap();
            guard.push(2);
            panic!("crash while holding the lock");
        })
    };
    assert!(thread.join().is_err());
    // 普通のロックは、以後ずっと失敗する
    assert!(shared.lock().is_err());
    // こちらは、続けられる (途中までの更新は残る)
    assert_eq!(*shared.lock_recover(), vec![1, 2]);
    shared.lock_recover().push(3);
    assert_eq!(*shared.lock_recover(), vec![1, 2, 3]);
}

#[test]
fn test_lock_recover_on_a_healthy_lock() {
    let value = Mutex::new(5);
    *value.lock_recover() += 1;
    assert_eq!(*value.lock_recover(), 6);
}
