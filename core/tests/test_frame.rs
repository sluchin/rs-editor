//! 描く内容 (`frame`) と, 本文の変更の記録のテスト.

use rseditor_core::buffer::{Buffer, Changes, TextChange};
use rseditor_core::editor::Editor;
use rseditor_core::frame::FrameBuilder;
use rseditor_core::window::Direction;

fn change(from: usize, to: usize, insert: &str) -> TextChange {
    TextChange {
        from,
        to,
        insert: insert.to_string(),
    }
}

#[test]
fn test_changes_are_recorded_in_utf16() {
    let mut b = Buffer::new("a");
    // 作ったばかりのバッファは, 全体を読み直したことにする
    assert_eq!(b.take_changes(), Changes::Reload);
    assert_eq!(b.version(), 0);
    b.insert("😀a");
    b.insert("b");
    b.delete_region(2, 3).unwrap();
    assert_eq!(b.version(), 3);
    assert_eq!(
        b.take_changes(),
        Changes::Edits(vec![
            change(0, 0, "😀a"),
            change(3, 3, "b"),
            change(3, 4, "")
        ])
    );
    assert_eq!(b.take_changes(), Changes::Edits(vec![]));
    // 読み込むと, 全体
    b.load("new");
    assert_eq!(b.take_changes(), Changes::Reload);
    assert_eq!(b.utf16_position(1), 1);
    b.load("😀x");
    assert_eq!(b.utf16_position(1), 2);
    assert_eq!(b.utf16_position(99), 3);
    assert_eq!(b.char_position(2), 1);
    assert_eq!(b.char_position(99), 2);
}

#[test]
fn test_too_many_changes_become_a_reload() {
    let mut b = Buffer::new("a");
    b.take_changes();
    for _ in 0..1001 {
        b.insert("x");
    }
    assert_eq!(b.take_changes(), Changes::Reload);
    // 大きな文字も
    b.insert(&"y".repeat((1 << 20) + 1));
    assert_eq!(b.take_changes(), Changes::Reload);
    // 読み直したことにした後の変更は, 覚えない
    b.insert(&"z".repeat((1 << 20) + 1));
    b.insert("w");
    assert_eq!(b.take_changes(), Changes::Reload);
}

#[test]
fn test_frame_sends_the_text_once_and_then_the_changes() {
    let mut editor = Editor::new();
    let mut frames = FrameBuilder::new();
    editor.current_buffer_mut().insert("hello");
    let frame = frames.build(&mut editor, "hi");
    assert_eq!(frame.message, "hi");
    assert_eq!(frame.selected, 1);
    assert_eq!(frame.windows.len(), 1);
    assert_eq!(frame.windows[0].buffer, "*scratch*");
    assert_eq!(frame.windows[0].point, 5);
    assert_eq!(frame.windows[0].mark, None);
    assert_eq!(frame.windows[0].width, 100.0);
    let scratch = &frame.buffers[0];
    assert_eq!(scratch.text.as_deref(), Some("hello"));
    assert!(scratch.changes.is_empty());
    assert!(scratch.modified);
    // 次は, 変更だけ
    editor.current_buffer_mut().insert("!");
    let frame = frames.build(&mut editor, "");
    assert_eq!(frame.buffers[0].text, None);
    assert_eq!(frame.buffers[0].changes, vec![change(5, 5, "!")]);
    // 変えなければ, 何も送らない
    let frame = frames.build(&mut editor, "");
    assert_eq!(frame.buffers[0].text, None);
    assert!(frame.buffers[0].changes.is_empty());
    // 送り直す
    frames.reset();
    let frame = frames.build(&mut editor, "");
    assert_eq!(frame.buffers[0].text.as_deref(), Some("hello!"));
}

#[test]
fn test_frame_of_windows_marks_and_hidden_buffers() {
    let mut editor = Editor::new();
    let mut frames = FrameBuilder::new();
    let b = editor.get_buffer_create("other");
    b.insert("ab");
    b.set_file(Some("/tmp/other.txt".to_string()));
    b.set_read_only(true);
    b.set_directory(Some("/tmp".to_string()));
    b.add_highlight(rseditor_core::buffer::Highlight {
        regexp: "a+".to_string(),
        face: "hi-yellow".to_string(),
        group: 0,
        fold: true,
        js: false,
    });
    // 出していないバッファは, 本文を送らない
    let frame = frames.build(&mut editor, "");
    let other = &frame.buffers[1];
    assert_eq!(other.name, "other");
    assert_eq!(other.text, None);
    assert_eq!(other.file.as_deref(), Some("/tmp/other.txt"));
    assert_eq!(other.directory.as_deref(), Some("/tmp"));
    assert!(other.read_only);
    assert_eq!(other.highlights[0].regexp, "a+");
    assert_eq!(other.highlights[0].face, "hi-yellow");
    assert_eq!(other.highlights[0].group, 0);
    // 窓を分けて出すと, 全体を送る. マークを使っていれば, リージョンの端も
    let w = editor.split_window(Direction::Right);
    editor.set_window_buffer(w, "other").unwrap();
    let b = editor.get_buffer_mut("other").unwrap();
    b.set_point(2);
    b.push_mark(0, true);
    let frame = frames.build(&mut editor, "");
    assert_eq!(frame.windows.len(), 2);
    assert_eq!(frame.windows[1].left, 50.0);
    assert_eq!(frame.windows[1].mark, Some(0));
    assert_eq!(frame.buffers[1].text.as_deref(), Some("ab"));
    // 消したバッファは忘れ, 同じ名前で作り直すと, 全体を送る
    editor.kill_buffer("other").unwrap();
    frames.build(&mut editor, "");
    editor.get_buffer_create("other").insert("new");
    let w = editor.selected_window();
    editor.set_window_buffer(w, "other").unwrap();
    let frame = frames.build(&mut editor, "");
    assert_eq!(frame.buffers[1].text.as_deref(), Some("new"));
}

#[test]
fn test_frame_sends_syntax_tokens_for_shown_buffers_in_utf16() {
    let mut editor = Editor::new();
    let mut frames = FrameBuilder::new();
    // 出していないバッファには、字句を送らない
    let b = editor.get_buffer_create("hidden");
    b.load("int x;");
    b.set_mode_name("c-mode");
    let frame = frames.build(&mut editor, "");
    assert!(frame.buffers[1].tokens.is_empty());
    // 出しているバッファには、UTF-16 の位置で送る (😀 は 2 つ)
    let b = editor.current_buffer_mut();
    b.load("/* 😀 */ int x;");
    b.set_mode_name("c-mode");
    let frame = frames.build(&mut editor, "");
    let kinds: Vec<(usize, usize, &str)> = frame.buffers[0]
        .tokens
        .iter()
        .map(|t| (t.from, t.to, t.kind))
        .collect();
    assert_eq!(
        kinds,
        vec![
            (0, 3, "comment-delimiter"),
            (3, 5, "comment"),
            (5, 8, "comment-delimiter"),
            (9, 12, "type"),
            (13, 14, "variable-name")
        ]
    );
}

#[test]
fn test_frame_carries_the_tab_bar() {
    let mut editor = Editor::new();
    let mut frames = FrameBuilder::new();
    let frame = frames.build(&mut editor, "");
    assert!(frame.tabs.is_empty());
    assert_eq!(frame.tab, 0);
    editor.set_tab_bar(vec!["a".to_string(), "b".to_string()], 1);
    assert_eq!(
        editor.tab_bar(),
        (&["a".to_string(), "b".to_string()][..], 1)
    );
    let frame = frames.build(&mut editor, "");
    assert_eq!(frame.tabs, vec!["a", "b"]);
    assert_eq!(frame.tab, 1);
}
