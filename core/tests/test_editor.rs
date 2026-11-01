//! エディタの状態 (`editor`) のテスト.

use rseditor_core::editor::{prefix_numeric_value, Editor, PrefixArg, SCRATCH};
use rseditor_core::window::Direction;

#[test]
fn test_prefix_numeric_value() {
    // 値は `emacs -Q --batch` の `prefix-numeric-value`
    assert_eq!(prefix_numeric_value(None), 1);
    assert_eq!(prefix_numeric_value(Some(PrefixArg::Minus)), -1);
    assert_eq!(prefix_numeric_value(Some(PrefixArg::Universal(4))), 4);
    assert_eq!(prefix_numeric_value(Some(PrefixArg::Universal(16))), 16);
    assert_eq!(prefix_numeric_value(Some(PrefixArg::Number(-7))), -7);
    let mut editor = Editor::default();
    assert_eq!(editor.prefix_arg, None);
    editor.prefix_arg = Some(PrefixArg::Number(5));
    assert_eq!(prefix_numeric_value(editor.prefix_arg), 5);
}

#[test]
fn test_buffers() {
    let mut editor = Editor::new();
    assert_eq!(editor.buffer_list(), vec![SCRATCH]);
    assert_eq!(editor.current_buffer_name(), SCRATCH);
    assert!(editor.get_buffer("a").is_none() && editor.get_buffer_mut("a").is_none());
    // 無ければ作り, あればそのバッファを返す
    editor.get_buffer_create("a").insert("x");
    assert_eq!(editor.get_buffer_create("a").text(), "x");
    // 名前が重なれば番号を付ける
    assert_eq!(editor.generate_new_buffer("a"), "a<2>");
    assert_eq!(editor.generate_new_buffer("a"), "a<3>");
    assert_eq!(editor.generate_new_buffer("b"), "b");
    assert_eq!(
        editor.buffer_list(),
        vec![SCRATCH, "a", "a<2>", "a<3>", "b"]
    );
    // 今のバッファ
    editor.set_buffer("a").unwrap();
    assert_eq!(editor.current_buffer().text(), "x");
    editor.current_buffer_mut().insert("y");
    assert_eq!(editor.get_buffer("a").unwrap().text(), "xy");
    assert_eq!(editor.set_buffer("z").unwrap_err(), "No such buffer: z");
}

#[test]
fn test_with_buffer() {
    let mut editor = Editor::new();
    editor.generate_new_buffer("a");
    let name = editor
        .with_buffer("a", |e| e.current_buffer_name().to_string())
        .unwrap();
    assert_eq!(name, "a");
    // 動かした後は, 元の今のバッファに戻る
    assert_eq!(editor.current_buffer_name(), SCRATCH);
    assert_eq!(
        editor.with_buffer("z", |_| ()).unwrap_err(),
        "No such buffer: z"
    );
    // 動かしている間に, 元のバッファを消したら, 戻さない
    editor
        .with_buffer("a", |e| e.kill_buffer(SCRATCH).unwrap())
        .unwrap();
    assert_eq!(editor.current_buffer_name(), "a");
}

#[test]
fn test_kill_buffer() {
    let mut editor = Editor::new();
    editor.generate_new_buffer("a");
    editor.generate_new_buffer("b");
    let second = editor.split_window(Direction::Below);
    editor.set_window_buffer(second, "a").unwrap();
    editor.set_buffer("a").unwrap();
    // 消したバッファを出していたウィンドウと今のバッファには, 次のバッファを出す
    editor.kill_buffer("a").unwrap();
    assert_eq!(editor.buffer_list(), vec![SCRATCH, "b"]);
    assert_eq!(editor.window_buffer(second), Some("b"));
    assert_eq!(editor.current_buffer_name(), "b");
    assert_eq!(
        editor.window_buffer(editor.selected_window()),
        Some(SCRATCH)
    );
    // 最後のバッファ (末尾) を消すと, 一つ前を出す
    editor.kill_buffer("b").unwrap();
    assert_eq!(editor.window_buffer(second), Some(SCRATCH));
    // 無いバッファは失敗にする. 最後の 1 つを消すと, *scratch* を作り直す
    assert_eq!(editor.kill_buffer("z").unwrap_err(), "No such buffer: z");
    editor.generate_new_buffer("c");
    editor.kill_buffer(SCRATCH).unwrap();
    editor.kill_buffer("c").unwrap();
    assert_eq!(editor.buffer_list(), vec![SCRATCH]);
    assert_eq!(editor.current_buffer_name(), SCRATCH);
}

#[test]
fn test_windows() {
    let mut editor = Editor::new();
    assert_eq!(editor.window_list(), vec![1]);
    assert_eq!(editor.selected_window(), 1);
    editor.generate_new_buffer("a");
    // 分けると, 新しいウィンドウは同じバッファを出し, 選んだウィンドウは変わらない
    let two = editor.split_window(Direction::Below);
    let three = editor.split_window(Direction::Right);
    assert_eq!((two, three), (2, 3));
    assert_eq!(editor.window_list(), vec![1, 3, 2]);
    assert_eq!(editor.selected_window(), 1);
    assert_eq!(editor.window_buffer(3), Some(SCRATCH));
    assert_eq!(editor.window_buffer(9), None);
    // set-window-buffer は, 今のバッファを変えない. select-window は変える
    editor.set_window_buffer(2, "a").unwrap();
    assert_eq!(editor.current_buffer_name(), SCRATCH);
    editor.select_window(2).unwrap();
    assert_eq!(editor.current_buffer_name(), "a");
    assert_eq!(editor.selected_window(), 2);
    assert_eq!(editor.select_window(9).unwrap_err(), "No such window: 9");
    assert_eq!(
        editor.set_window_buffer(9, "a").unwrap_err(),
        "No such window: 9"
    );
    assert_eq!(
        editor.set_window_buffer(1, "z").unwrap_err(),
        "No such buffer: z"
    );
    // 次のウィンドウへ (一周する. 負なら前へ)
    editor.other_window(1);
    assert_eq!(editor.selected_window(), 1);
    editor.other_window(-1);
    assert_eq!(editor.selected_window(), 2);
    editor.other_window(1);
    editor.other_window(1);
    assert_eq!(editor.selected_window(), 3);
    assert_eq!(editor.layout().window_ids(), vec![1, 3, 2]);
}

#[test]
fn test_delete_windows() {
    let mut editor = Editor::new();
    editor.split_window(Direction::Below);
    editor.split_window(Direction::Right);
    // 選んでいないウィンドウを消す
    editor.delete_window(3).unwrap();
    assert_eq!(editor.window_list(), vec![1, 2]);
    assert_eq!(editor.selected_window(), 1);
    // 選んでいるウィンドウを消すと, 前 (無ければ次) のウィンドウを選ぶ
    editor.delete_window(1).unwrap();
    assert_eq!(editor.selected_window(), 2);
    editor.split_window(Direction::Below);
    editor.select_window(4).unwrap();
    editor.delete_window(4).unwrap();
    assert_eq!(editor.selected_window(), 2);
    // 1 つだけのウィンドウは消せない. 無いウィンドウも失敗にする
    assert_eq!(
        editor.delete_window(2).unwrap_err(),
        "Attempt to delete minibuffer or sole ordinary window"
    );
    assert_eq!(editor.window_list(), vec![2]);
    assert_eq!(editor.delete_window(9).unwrap_err(), "No such window: 9");
    // 選んでいるウィンドウだけにする
    editor.split_window(Direction::Right);
    editor.split_window(Direction::Right);
    editor.delete_other_windows();
    assert_eq!(editor.window_list(), vec![2]);
    assert_eq!(editor.window_buffer(5), None);
}

#[test]
fn test_window_configuration() {
    let mut editor = Editor::new();
    editor.get_buffer_create("a");
    editor.get_buffer_create("b");
    let second = editor.split_window(Direction::Below);
    editor.set_window_buffer(second, "a").unwrap();
    let saved = editor.save_window_configuration();
    // 設定を覚えた後に, ウィンドウを分け直し, バッファを変え, 選び直す
    editor.delete_other_windows();
    editor
        .set_window_buffer(editor.selected_window(), "b")
        .unwrap();
    editor.select_window(editor.selected_window()).unwrap();
    assert_eq!(editor.window_list().len(), 1);
    editor.restore_window_configuration(saved).unwrap();
    assert_eq!(editor.window_list(), vec![1, second]);
    assert_eq!(editor.window_buffer(1), Some(SCRATCH));
    assert_eq!(editor.window_buffer(second), Some("a"));
    assert_eq!(editor.selected_window(), 1);
    assert_eq!(editor.current_buffer_name(), SCRATCH);
    // 覚えた後に消したバッファを出していたウィンドウは, 今のバッファを出す
    editor.kill_buffer("a").unwrap();
    editor.restore_window_configuration(saved).unwrap();
    assert_eq!(editor.window_buffer(second), Some(SCRATCH));
    // 新しいウィンドウの番号は, 戻した設定のものと重ならない
    assert_eq!(editor.split_window(Direction::Right), second + 1);
    assert!(editor.restore_window_configuration(99).is_err());
}

#[test]
fn test_generate_new_buffer_name_and_rename() {
    let mut editor = Editor::new();
    // 使われていない名前はそのまま. 使われていれば <2>, <3>
    assert_eq!(editor.generate_new_buffer_name("a", None), "a");
    editor.get_buffer_create("a");
    editor.get_buffer_create("a<2>");
    assert_eq!(editor.generate_new_buffer_name("a", None), "a<3>");
    // ignore した名前は, 使われていないものとして扱う
    assert_eq!(editor.generate_new_buffer_name("a", Some("a")), "a");
    // 名前を変えると, ウィンドウと, 覚えた設定と, global-mark-ring の名前も変わる
    editor.set_buffer("a").unwrap();
    let second = editor.split_window(Direction::Below);
    editor.set_window_buffer(second, "a").unwrap();
    let saved = editor.save_window_configuration();
    editor.global_mark_ring.push(("a".to_string(), 0));
    editor.global_mark_ring.push(("a<2>".to_string(), 0));
    assert_eq!(editor.rename_buffer("b", false).unwrap(), "b");
    assert_eq!(editor.current_buffer_name(), "b");
    assert_eq!(editor.current_buffer().name(), "b");
    assert!(editor.get_buffer("a").is_none());
    assert_eq!(editor.window_buffer(second), Some("b"));
    assert_eq!(
        editor.global_mark_ring,
        vec![("b".to_string(), 0), ("a<2>".to_string(), 0)]
    );
    editor.restore_window_configuration(saved).unwrap();
    assert_eq!(editor.window_buffer(second), Some("b"));
    editor.set_buffer("b").unwrap();
    // 同じ名前への変更は何もしない. 使われている名前は失敗するが, unique なら <N> を付ける
    assert_eq!(editor.rename_buffer("b", false).unwrap(), "b");
    assert_eq!(
        editor.rename_buffer("a<2>", false).unwrap_err(),
        "Buffer name ‘a<2>’ is in use"
    );
    assert_eq!(editor.rename_buffer("a<2>", true).unwrap(), "a<2><2>");
    assert_eq!(
        editor.rename_buffer("", false).unwrap_err(),
        "Empty string is invalid as a buffer name"
    );
}
