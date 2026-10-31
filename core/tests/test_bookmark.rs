//! bookmark (`lisp/bookmark.scm`) のテスト.

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

/// ブックマークの保存先を `name` の下に変えた, 評価器とエディタ.
fn setup(name: &str) -> (Evaluator, EditorHost<NoUi>, std::path::PathBuf) {
    let dir = std::env::temp_dir().join(format!("rseditor-bookmark-{name}"));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let ev = Evaluator::new();
    let host = EditorHost::new(Editor::new(), NoUi);
    ev.eval_with(include_str!("../../lisp/simple.scm"), &host)
        .unwrap();
    ev.eval_with(include_str!("../../lisp/bookmark.scm"), &host)
        .unwrap();
    let saved = dir.join("sub").join("bookmarks");
    ev.eval_with(
        &format!("(set! bookmark-default-file {:?})", saved.display().to_string()),
        &host,
    )
    .unwrap();
    (ev, host, dir)
}

fn visit(ev: &Evaluator, host: &EditorHost<NoUi>, path: &std::path::Path) {
    ev.eval_with(&format!("(find-file {:?})", path.display().to_string()), host)
        .unwrap();
}

/// カーソルの位置 (Lisp と同じく 1 から数える).
fn point(host: &EditorHost<NoUi>) -> usize {
    host.editor().current_buffer().point() + 1
}

#[test]
fn test_set_saves_and_jump_returns() {
    let (ev, host, dir) = setup("jump");
    let file = dir.join("a.txt");
    std::fs::write(&file, "one\ntwo \"quoted\"\nthree\n").unwrap();
    visit(&ev, &host, &file);
    ev.eval_with("(goto-char 6)", &host).unwrap();
    ev.eval_with("(bookmark-set \"here\")", &host).unwrap();
    let saved = std::fs::read_to_string(dir.join("sub").join("bookmarks")).unwrap();
    assert!(saved.starts_with("((\"here\" (filename "), "{saved}");
    ev.eval_with("(goto-char 1)", &host).unwrap();
    ev.eval_with("(bookmark-jump \"here\")", &host).unwrap();
    assert_eq!(point(&host), 6);
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn test_jump_follows_the_edited_text_and_a_new_session_reads_the_file() {
    let (ev, host, dir) = setup("edit");
    let file = dir.join("b.txt");
    std::fs::write(&file, "aaa\nbbb target ccc\n").unwrap();
    visit(&ev, &host, &file);
    ev.eval_with("(goto-char 10)", &host).unwrap();
    ev.eval_with("(bookmark-set \"t\")", &host).unwrap();
    // 前に 3 文字足すと, 覚えた位置の前後の文字で直す
    ev.eval_with("(goto-char 1)", &host).unwrap();
    ev.eval_with("(insert \"xyz\")", &host).unwrap();
    ev.eval_with("(bookmark-jump \"t\")", &host).unwrap();
    assert_eq!(point(&host), 13);
    // 新しい評価器は, 保存したファイルから読む
    let (ev2, host2, _) = setup("edit-2");
    let saved = dir.join("sub").join("bookmarks");
    ev2.eval_with(
        &format!("(set! bookmark-default-file {:?})", saved.display().to_string()),
        &host2,
    )
    .unwrap();
    ev2.eval_with("(bookmark-jump \"t\")", &host2).unwrap();
    assert_eq!(host2.editor().current_buffer_name(), "b.txt");
    std::fs::remove_dir_all(dir).unwrap();
    let _ = std::fs::remove_dir_all(std::env::temp_dir().join("rseditor-bookmark-edit-2"));
}

#[test]
fn test_set_replaces_a_bookmark_of_the_same_name_in_place() {
    let (ev, host, dir) = setup("replace");
    let file = dir.join("c.txt");
    std::fs::write(&file, "0123456789\n").unwrap();
    visit(&ev, &host, &file);
    ev.eval_with("(goto-char 2)", &host).unwrap();
    ev.eval_with("(bookmark-set \"a\")", &host).unwrap();
    ev.eval_with("(bookmark-set \"b\")", &host).unwrap();
    ev.eval_with("(goto-char 8)", &host).unwrap();
    ev.eval_with("(bookmark-set \"a\")", &host).unwrap();
    let names = ev
        .eval_with("(map car bookmark-alist)", &host)
        .unwrap()
        .to_string();
    assert_eq!(names, "(\"b\" \"a\")");
    ev.eval_with("(goto-char 1)", &host).unwrap();
    ev.eval_with("(bookmark-jump \"a\")", &host).unwrap();
    assert_eq!(point(&host), 8);
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn test_errors() {
    let (ev, host, dir) = setup("errors");
    assert!(ev
        .eval_with("(bookmark-set \"x\")", &host)
        .unwrap_err()
        .contains("not visiting a file"));
    assert!(ev
        .eval_with("(bookmark-jump \"x\")", &host)
        .unwrap_err()
        .contains("No bookmarks"));
    let file = dir.join("d.txt");
    std::fs::write(&file, "text\n").unwrap();
    visit(&ev, &host, &file);
    ev.eval_with("(bookmark-set \"d\")", &host).unwrap();
    assert!(ev
        .eval_with("(bookmark-jump \"nope\")", &host)
        .unwrap_err()
        .contains("Invalid bookmark nope"));
    std::fs::remove_file(&file).unwrap();
    assert!(ev
        .eval_with("(bookmark-jump \"d\")", &host)
        .unwrap_err()
        .contains("file not found"));
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn test_set_no_overwrite_signals_an_error_or_pushes() {
    let (ev, host, dir) = setup("no-overwrite");
    let file = dir.join("e.txt");
    std::fs::write(&file, "0123456789\n").unwrap();
    visit(&ev, &host, &file);
    ev.eval_with("(bookmark-set-no-overwrite \"a\")", &host)
        .unwrap();
    assert!(ev
        .eval_with("(bookmark-set-no-overwrite \"a\")", &host)
        .unwrap_err()
        .contains("A bookmark named \"a\" already exists"));
    ev.eval_with("(goto-char 8)", &host).unwrap();
    ev.eval_with("(bookmark-set-no-overwrite \"a\" #t)", &host)
        .unwrap();
    let count = ev
        .eval_with("(length bookmark-alist)", &host)
        .unwrap()
        .to_string();
    assert_eq!(count, "2");
    ev.eval_with("(goto-char 1)", &host).unwrap();
    ev.eval_with("(bookmark-jump \"a\")", &host).unwrap();
    assert_eq!(point(&host), 8);
    std::fs::remove_dir_all(dir).unwrap();
}
