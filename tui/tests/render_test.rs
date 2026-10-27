//! 画面の組み立て (`render`) と、ホストの細かい操作のテスト.

mod common;
use common::Fixture;

use std::collections::BTreeMap;

use rseditor_core::editor::Editor;
use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::value::Value;
use rseditor_core::window::Direction;
use rseditor_tui::render::{render, render_with, Display, Minibuffer};

/// 本文のあるエディタを描く.
fn draw(text: &str, rows: usize, cols: usize) -> (Vec<String>, (usize, usize)) {
    let mut editor = Editor::new();
    editor.current_buffer_mut().insert(text);
    let mut tops = BTreeMap::new();
    let frame = render(&editor, &mut tops, rows, cols, "", None);
    (frame.lines(), frame.cursor)
}

#[test]
fn test_wrapping_tabs_controls_and_wide_characters() {
    // 折り返す: カーソルは、折り返した行の中
    let (lines, cursor) = draw("abcdefghij", 6, 4);
    assert_eq!(&lines[..3], ["abcd", "efgh", "ij"]);
    assert_eq!(cursor, (2, 2));
    // タブは桁に合わせて空白に, 制御文字は ^X に, DEL は ^?
    let (lines, _) = draw("a\tb\n\x01\x7f", 6, 20);
    assert_eq!(lines[0], "a       b");
    assert_eq!(lines[1], "^A^?");
    // 幅の広い文字は 2 マス. 端で割らずに、次の行へ
    let (lines, cursor) = draw("あいう", 6, 5);
    assert_eq!(lines[0], "あい");
    assert_eq!(lines[1], "う");
    assert_eq!(cursor, (1, 2));
    // 幅 1 の窓でも壊れない
    let (lines, _) = draw("あb", 6, 1);
    assert_eq!(lines.len(), 6);
}

#[test]
fn test_scrolling_keeps_the_cursor_visible() {
    let text: String = (0..20).map(|i| format!("line{i}\n")).collect();
    // 末尾にカーソルがあれば、末尾が見える
    let (lines, cursor) = draw(&text, 6, 10);
    assert_eq!(lines[3], "");
    assert!(lines[..4].contains(&"line19".to_string()));
    assert_eq!(cursor.1, 0);
    // 前に戻ると、一番上の行も戻る
    let mut editor = Editor::new();
    editor.current_buffer_mut().insert(&text);
    let mut tops = BTreeMap::new();
    render(&editor, &mut tops, 6, 10, "", None);
    assert!(tops[&1] > 0);
    editor.current_buffer_mut().set_point(0);
    let frame = render(&editor, &mut tops, 6, 10, "", None);
    assert_eq!(tops[&1], 0);
    assert_eq!(frame.line(0), "line0");
    // 折り返した行が多くても、カーソルの行が見える
    let long = format!("{}\nx", "a".repeat(40));
    let (lines, _) = draw(&long, 4, 5);
    assert_eq!(lines[0], "x");
}

#[test]
fn test_mode_line_and_echo_area() {
    let mut fx = Fixture::new();
    fx.press("a");
    fx.app.draw();
    let screen = fx.screen();
    assert!(screen[10].starts_with("-:**-  *scratch*   L1"));
    assert_eq!(screen[10].chars().count(), 30);
    let frames = fx.frames.lock().unwrap();
    let mode_row = &frames.last().unwrap().cells[10];
    assert!(mode_row.iter().all(|c| c.reverse));
    drop(frames);
    // メッセージは、最後の行. 幅を越えたら切る
    fx.press("C-x C-z");
    fx.app.draw();
    assert_eq!(fx.screen()[11], "C-x C-z is undefined");
    fx.app
        .eval("(message \"あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへほ\")")
        .unwrap();
    fx.app.draw();
    assert!(fx.screen()[11].chars().count() <= 15);
}

#[test]
fn test_mode_names_and_the_file_dialects() {
    let dir = tempfile::tempdir().unwrap();
    let el = dir.path().join("a.el");
    let scm = dir.path().join("b.scm");
    std::fs::write(&el, "(a)").unwrap();
    std::fs::write(&scm, "(b)").unwrap();
    let files = [
        el.to_str().unwrap().to_string(),
        scm.to_str().unwrap().to_string(),
    ];
    let fx = Fixture::sized(12, 50, &files).unwrap();
    fx.app.draw();
    assert!(fx.screen()[10].contains("(Scheme)"));
    fx.app.host().editor().set_buffer("a.el").unwrap();
    let w = fx.app.host().editor().selected_window();
    fx.app.host().editor().set_window_buffer(w, "a.el").unwrap();
    fx.app.draw();
    assert!(fx.screen()[10].contains("(ELisp)"));
}

#[test]
fn test_split_windows_are_drawn_side_by_side_and_stacked() {
    let mut editor = Editor::new();
    editor.current_buffer_mut().insert("one");
    editor.split_window(Direction::Right);
    let mut tops = BTreeMap::new();
    let frame = render(&editor, &mut tops, 6, 20, "", None);
    let lines = frame.lines();
    assert_eq!(lines[0], "one       one");
    assert_eq!(lines[4], "-:**-  *sc-:**-  *sc");
    // 上下に分ける. 小さすぎる窓は描かない
    let mut editor = Editor::new();
    editor.split_window(Direction::Below);
    let frame = render(&editor, &mut BTreeMap::new(), 4, 10, "", None);
    assert_eq!(
        frame.lines().iter().filter(|l| l.starts_with("-:")).count(),
        1
    );
    // 行が 0 や 1 でも壊れない
    for rows in [0, 1, 2] {
        let frame = render(&Editor::new(), &mut BTreeMap::new(), rows, 10, "", None);
        assert_eq!(frame.cells.len(), rows);
    }
}

#[test]
fn test_minibuffer_cursor() {
    let editor = Editor::new();
    let minibuffer = Minibuffer {
        prompt: "名前: ",
        input: "abc",
        cursor: 2,
    };
    let frame = render(&editor, &mut BTreeMap::new(), 6, 20, "", Some(&minibuffer));
    assert_eq!(frame.line(5), "名前: abc");
    assert_eq!(frame.cursor, (5, 8));
}

#[test]
fn test_host_operations_of_the_terminal() {
    let mut fx = Fixture::new();
    let host = fx.app.host();
    let list =
        |names: &[&str]| Value::List(names.iter().map(|n| Value::String(n.to_string())).collect());
    // キーの割り当て
    let keys = host
        .call("where-is-internal", vec![Value::String("find-file".into())])
        .unwrap();
    assert_eq!(keys.to_string(), "(\"C-x C-f\")");
    assert_eq!(
        host.call("editor-command-names", vec![])
            .unwrap()
            .to_string(),
        "()"
    );
    // 端末では何もしないもの
    for op in [
        "highlight-regexp",
        "unhighlight-regexp",
        "outline-flag-region",
        "local-set-key",
        "set-background-color",
        "set-foreground-color",
        "set-font",
    ] {
        assert!(host.call(op, vec![]).is_ok(), "{op}");
    }
    assert_eq!(
        host.call("outline-invisible-p", vec![])
            .unwrap()
            .to_string(),
        "#f"
    );
    // 受けないもの, 動かせないコマンド
    assert_eq!(
        host.call("no-such-op", vec![]).unwrap_err(),
        "The terminal does not support no-such-op"
    );
    assert_eq!(
        fx.app
            .eval("(call-interactively 'no-such-command)")
            .unwrap_err(),
        "Undefined command: no-such-command"
    );
    // 画面の高さ: 窓が 1 つなら, 行 - 2. 分けると半分
    assert_eq!(fx.app.eval("(window-body-height)").unwrap(), "10");
    fx.press("C-x 2");
    assert_eq!(fx.app.eval("(window-body-height)").unwrap(), "5");
    let _ = list;
    // 中央に置く
    fx.press("C-x 1 C-l");
    assert_eq!(fx.app.message(), "");
}

#[test]
fn test_a_long_line_is_cut_at_the_bottom_of_the_window() {
    let (lines, _) = draw(&"a".repeat(40), 4, 5);
    assert_eq!(lines[0], "aaaaa");
    assert_eq!(lines[1], "aaaaa");
    assert!(lines[2].starts_with("-:"));
}

#[test]
fn test_missing_arguments_and_odd_candidates() {
    let fx = Fixture::new();
    let host = fx.app.host();
    // 文字列でない引数は、空の文字列として扱う
    assert!(host.call("message", vec![]).is_ok());
    assert_eq!(fx.app.message(), "");
    fx.feed("RET");
    let list = Value::List(
        vec![Value::Number(1.0), Value::String("a".into())]
            .into_iter()
            .collect(),
    );
    let answer = fx
        .app
        .host()
        .call(
            "completing-read",
            vec![Value::String("P: ".into()), list, Value::String("x".into())],
        )
        .unwrap();
    assert_eq!(answer.to_string(), "\"x\"");
    fx.feed("RET");
    let answer = fx
        .app
        .host()
        .call(
            "completing-read",
            vec![Value::String("P: ".into()), Value::Number(1.0)],
        )
        .unwrap();
    assert_eq!(answer.to_string(), "\"\"");
}

/// 出し方の設定を使って, 本文のあるエディタを描く.
fn draw_with(
    text: &str,
    rows: usize,
    cols: usize,
    display: Display,
) -> (Vec<String>, (usize, usize)) {
    let mut editor = Editor::new();
    editor.current_buffer_mut().insert(text);
    let frame = render_with(&editor, &mut BTreeMap::new(), rows, cols, "", None, display);
    (frame.lines(), frame.cursor)
}

#[test]
fn test_truncate_lines_and_selective_display() {
    // 切る: 1 行を 1 行で出し, カーソルは窓の端
    let truncate = Display {
        truncate_lines: true,
        ..Display::default()
    };
    let (lines, cursor) = draw_with("abcdefghij\nxy", 6, 4, truncate);
    assert_eq!(&lines[..2], ["abcd", "xy"]);
    assert_eq!(cursor, (1, 2));
    let (_, cursor) = draw_with("abcdefghij", 6, 4, truncate);
    assert_eq!(cursor, (0, 3));
    // 字下げした行を出さない (空の行は出す)
    let selective = Display {
        selective_display: Some(2),
        ..Display::default()
    };
    let (lines, _) = draw_with("a\n  b\n\tc\n\nd", 8, 10, selective);
    assert_eq!(&lines[..3], ["a", "", "d"]);
}
