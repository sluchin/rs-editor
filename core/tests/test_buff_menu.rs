//! バッファの一覧 (`lisp/buff-menu.scm`) のテスト. 期待値 (`buffers.txt`) は, `emacs -Q --batch -l tools/syntax/buffers.el` の結果.

use rseditor_core::editor::Editor;
use rseditor_core::scheme::editor_host::EditorHost;
use rseditor_core::scheme::evaluator::Evaluator;
use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::value::Value;

/// 画面の要る操作を受けない偽物.
struct NoUi;

impl Host for NoUi {
    fn call(&self, op: &str, _args: Vec<Value>) -> Result<Value, String> {
        Err(format!("ui: {op}"))
    }
}

fn setup() -> (Evaluator, EditorHost<NoUi>) {
    let ev = Evaluator::new();
    let host = EditorHost::new(Editor::new(), NoUi);
    ev.eval_with(include_str!("../../lisp/simple.scm"), &host)
        .unwrap();
    ev.eval_with(include_str!("../../lisp/buff-menu.scm"), &host)
        .unwrap();
    (ev, host)
}

fn list(ev: &Evaluator, host: &EditorHost<NoUi>, code: &str) -> String {
    ev.eval_with(code, host).unwrap();
    let editor = host.editor();
    editor.get_buffer("*Buffer List*").unwrap().text()
}

#[test]
fn test_list_buffers_matches_emacs() {
    let (ev, host) = setup();
    {
        let mut editor = host.editor();
        editor.get_buffer_create("big").load(&"x".repeat(1_234_567));
    }
    ev.eval_with(
        r#"(for-each (lambda (n) (with-current-buffer (get-buffer-create n) (insert "xxxxxxxxxxxx")))
                     '("short" "a-very-long-buffer-name-over-19" "名前あいうえおかきくけこさしすせそたち" " hidden"))"#,
        &host,
    )
    .unwrap();
    // `big` を最後に並べる (Emacs と同じ順)
    ev.eval_with("(kill-buffer \"big\")", &host).unwrap();
    host.editor()
        .get_buffer_create("big")
        .load(&"x".repeat(1_234_567));
    let text = list(&ev, &host, "(list-buffers-noselect)");
    assert_eq!(text, include_str!("buffers.txt"));
}

#[test]
fn test_list_buffers_files_only_and_the_file_column() {
    let (ev, host) = setup();
    let path = std::env::temp_dir().join("rseditor-test-buff-menu.txt");
    std::fs::write(&path, "hello").unwrap();
    ev.eval_with(
        &format!("(find-file-noselect {:?})", path.to_str().unwrap()),
        &host,
    )
    .unwrap();
    ev.eval_with(
        "(with-current-buffer (get-buffer-create \"other\") (insert \"ab\"))",
        &host,
    )
    .unwrap();
    let all = list(&ev, &host, "(list-buffers-noselect)");
    assert!(all.contains("other"), "{all}");
    let files = list(&ev, &host, "(list-buffers-noselect 1)");
    assert!(!files.contains("other"), "{files}");
    assert!(files.contains("rseditor-test-buff-menu.txt"), "{files}");
    assert!(files.ends_with(&format!("{}\n", path.display())), "{files}");
    // 負の引数は, 全部
    let negative = list(&ev, &host, "(list-buffers-noselect -1)");
    assert!(negative.contains("other"));
    std::fs::remove_file(path).unwrap();
}

#[test]
fn test_list_buffers_abbreviates_the_home_directory_and_shows_the_dialect() {
    let (ev, host) = setup();
    ev.eval_with(
        "(set-syntax-dialect 'scheme) (set-buffer (get-buffer-create \"s\")) (set-syntax-dialect 'elisp)",
        &host,
    )
    .unwrap();
    let text = list(&ev, &host, "(list-buffers-noselect)");
    assert!(text.contains("ELisp"), "{text}");
    assert!(text.contains(" Scheme "), "{text}");
    let home = std::env::var("HOME").unwrap();
    let dir = std::path::Path::new(&home).join(".rseditor-test-buff-menu");
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("a.txt");
    std::fs::write(&file, "").unwrap();
    ev.eval_with(
        &format!("(find-file-noselect {:?})", file.to_str().unwrap()),
        &host,
    )
    .unwrap();
    let text = list(&ev, &host, "(list-buffers-noselect)");
    assert!(text.contains("~/.rseditor-test-buff-menu/a.txt"), "{text}");
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn test_list_buffers_displays_the_list_in_another_window() {
    let (ev, host) = setup();
    ev.eval_with("(list-buffers)", &host).unwrap();
    let editor = host.editor();
    assert_eq!(editor.window_list().len(), 2);
    assert_eq!(editor.current_buffer().name(), "*scratch*");
}
