//! ミニバッファの入力の履歴 (`M-p`, `M-n`, `M-r`, `M-s`).

use rseditor_tui::history::{add_to_history, HistoryArgs, Navigator, HISTORY_LENGTH};

fn items(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| s.to_string()).collect()
}

#[test]
fn test_add_to_history_skips_empty_and_repeated_items() {
    let mut history = items(&["b", "a"]);
    add_to_history(&mut history, "");
    add_to_history(&mut history, "b");
    assert_eq!(history, items(&["b", "a"]));
    add_to_history(&mut history, "c");
    assert_eq!(history, items(&["c", "b", "a"]));
    for i in 0..HISTORY_LENGTH + 5 {
        add_to_history(&mut history, &i.to_string());
    }
    assert_eq!(history.len(), HISTORY_LENGTH);
}

#[test]
fn test_previous_and_next_walk_the_history_and_keep_the_typed_text() {
    let history = items(&["c", "b", "a"]);
    let mut nav = Navigator::new();
    assert_eq!(nav.previous("typed", &history, &[]), Ok("c".to_string()));
    assert_eq!(nav.previous("c", &history, &[]), Ok("b".to_string()));
    assert_eq!(nav.previous("b", &history, &[]), Ok("a".to_string()));
    assert_eq!(
        nav.previous("a", &history, &[]),
        Err("Beginning of history; no preceding item".to_string())
    );
    assert_eq!(nav.next("a", &history, &[]), Ok("b".to_string()));
    assert_eq!(nav.next("b", &history, &[]), Ok("c".to_string()));
    // 位置 0 へ戻ると, 打っていた文字
    assert_eq!(nav.next("c", &history, &[]), Ok("typed".to_string()));
    assert_eq!(
        nav.next("typed", &history, &[]),
        Err("End of history; no default available".to_string())
    );
}

#[test]
fn test_next_past_the_end_of_the_history_gives_the_defaults() {
    let history = items(&["a"]);
    let defaults = items(&["d1", "d2"]);
    let mut nav = Navigator::new();
    assert_eq!(nav.next("t", &history, &defaults), Ok("d1".to_string()));
    assert_eq!(nav.next("d1", &history, &defaults), Ok("d2".to_string()));
    assert_eq!(
        nav.next("d2", &history, &defaults),
        Err("End of defaults; no next item".to_string())
    );
    // 戻ると, 打っていた文字, さらに履歴
    assert_eq!(
        nav.previous("d2", &history, &defaults),
        Ok("d1".to_string())
    );
    assert_eq!(nav.previous("d1", &history, &defaults), Ok("t".to_string()));
    assert_eq!(nav.previous("t", &history, &defaults), Ok("a".to_string()));
}

#[test]
fn test_search_finds_the_matching_items_in_both_directions() {
    let history = items(&["foo3", "bar2", "foo1", "baz0"]);
    let mut nav = Navigator::new();
    assert_eq!(
        nav.search("foo", true, "t", &history, &[]),
        Ok("foo3".to_string())
    );
    assert_eq!(
        nav.search("foo", true, "foo3", &history, &[]),
        Ok("foo1".to_string())
    );
    assert_eq!(
        nav.search("foo", true, "foo1", &history, &[]),
        Err("No earlier matching history item".to_string())
    );
    assert_eq!(
        nav.search("o3", false, "foo1", &history, &[]),
        Ok("foo3".to_string())
    );
    assert_eq!(
        nav.search("o3", false, "foo3", &history, &[]),
        Err("No later matching history item".to_string())
    );
    assert!(nav.search("\\(", true, "x", &history, &[]).is_err());
}

#[test]
fn test_history_args_from_the_request_of_the_core() {
    use rseditor_core::scheme::value::Value;
    let s = |t: &str| Value::String(t.to_string());
    let list = |v: Vec<Value>| Value::List(v.into());
    // 名前, 既定の値, 中身, 始めの位置 (文字列でない要素は除く)
    let args = vec![
        s("P: "),
        s(""),
        s("isearch-ring"),
        list(vec![s("d")]),
        list(vec![s("b"), Value::Number(1.0), s("a")]),
        Value::Number(1.0),
    ];
    assert_eq!(
        HistoryArgs::from_args(&args, 2),
        HistoryArgs {
            name: "isearch-ring".to_string(),
            defaults: items(&["d"]),
            items: Some(items(&["b", "a"])),
            start: 1,
        }
    );
    // 省くと, 名前の無い, 画面の履歴
    assert_eq!(HistoryArgs::from_args(&[], 2), HistoryArgs::default());
    assert_eq!(
        HistoryArgs::named("minibuffer", items(&["d"])),
        HistoryArgs {
            name: "minibuffer".to_string(),
            defaults: items(&["d"]),
            items: None,
            start: 0,
        }
    );
}

#[test]
fn test_navigator_starting_at_a_position() {
    let history = items(&["c", "b", "a"]);
    // 位置 1 から始めると, M-p は 2 つ目. 位置 0 に戻ると, 始めの文字
    let mut nav = Navigator::starting_at(1, "init", &history);
    assert_eq!(nav.previous("init", &history, &[]), Ok("b".to_string()));
    assert_eq!(nav.next("b", &history, &[]), Ok("c".to_string()));
    assert_eq!(nav.next("c", &history, &[]), Ok("init".to_string()));
    // 履歴の数より先は, 履歴の終わり
    let mut nav = Navigator::starting_at(9, "", &history);
    assert_eq!(
        nav.previous("", &history, &[]),
        Err("Beginning of history; no preceding item".to_string())
    );
}
