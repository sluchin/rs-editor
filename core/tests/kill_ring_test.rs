//! kill-ring (`kill_ring`) のテスト.

use rseditor_core::kill_ring::{KillRing, KILL_RING_MAX};

#[test]
fn test_kill_new_and_current_kill() {
    let mut ring = KillRing::new();
    assert!(ring.is_empty());
    assert_eq!(
        ring.current_kill(0, false).unwrap_err(),
        "Kill ring is empty"
    );
    ring.kill_new("a", false);
    ring.kill_new("b", false);
    ring.kill_new("c", false);
    assert_eq!(ring.len(), 3);
    // 新しいものが先. 進めると, 古い方へ一周する
    assert_eq!(ring.current_kill(0, false).unwrap(), "c");
    assert_eq!(ring.current_kill(1, false).unwrap(), "b");
    assert_eq!(ring.current_kill(1, false).unwrap(), "a");
    assert_eq!(ring.current_kill(1, false).unwrap(), "c");
    assert_eq!(ring.current_kill(-1, false).unwrap(), "a");
    // do_not_move なら, 次に貼るものを動かさない
    assert_eq!(ring.current_kill(1, true).unwrap(), "c");
    assert_eq!(ring.current_kill(0, true).unwrap(), "a");
    // 先頭は, 次に貼るものを動かしても変わらない
    assert_eq!(ring.head(), Some("c"));
    assert_eq!(KillRing::new().head(), None);
    // 新しく入れたら, 次に貼るものは先頭に戻る
    ring.kill_new("d", false);
    assert_eq!(ring.current_kill(0, false).unwrap(), "d");
}

#[test]
fn test_kill_new_replace() {
    let mut ring = KillRing::new();
    // 空なら, 置き換えずに入れる
    ring.kill_new("a", true);
    assert_eq!(ring.len(), 1);
    ring.kill_new("b", true);
    assert_eq!(ring.len(), 1);
    assert_eq!(ring.current_kill(0, true).unwrap(), "b");
}

#[test]
fn test_kill_append() {
    let mut ring = KillRing::new();
    // 空なら, 新しく入れる
    ring.kill_append("b", false);
    assert_eq!(ring.current_kill(0, true).unwrap(), "b");
    ring.kill_append("c", false);
    ring.kill_append("a", true);
    assert_eq!(ring.len(), 1);
    assert_eq!(ring.current_kill(0, true).unwrap(), "abc");
}

#[test]
fn test_kill_ring_max() {
    let mut ring = KillRing::new();
    for i in 0..(KILL_RING_MAX + 10) {
        ring.kill_new(&i.to_string(), false);
    }
    assert_eq!(ring.len(), KILL_RING_MAX);
    assert_eq!(
        ring.current_kill(0, true).unwrap(),
        (KILL_RING_MAX + 9).to_string()
    );
}
