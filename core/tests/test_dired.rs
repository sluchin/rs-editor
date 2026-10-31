//! dired (`lisp/dired.scm`) と, 組み込みの `directory-listing` のテスト.
//! 本文の形 (見出し, 字下げ, `total` の行が無いこと, 最初のファイルの位置) は, `emacs -Q --batch` の `dired` の結果.

use std::path::Path;

use rseditor_core::editor::Editor;
use rseditor_core::scheme::editor_host::EditorHost;
use rseditor_core::scheme::evaluator::Evaluator;
use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::system::directory_listing;
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
    ev.eval_with(include_str!("../../lisp/dired.scm"), &host)
        .unwrap();
    (ev, host)
}

fn temp_dir(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("rseditor-dired-{name}"));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("sub")).unwrap();
    std::fs::write(dir.join("a.txt"), "hi\n").unwrap();
    dir
}

#[test]
fn test_directory_listing_errors() {
    let dir = temp_dir("listing");
    let text = directory_listing("ls", &dir).unwrap();
    assert!(text.starts_with("total "));
    assert!(text.contains(" a.txt\n"));
    assert!(directory_listing("ls", &dir.join("none"))
        .unwrap_err()
        .contains("rseditor-dired-listing/none"));
    // `ls` が失敗したとき (標準エラー出力が理由), 見つからないとき
    assert_eq!(directory_listing("false", &dir), Err(String::new()));
    assert!(directory_listing("no-such-ls-program", &dir)
        .unwrap_err()
        .starts_with("no-such-ls-program: "));
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn test_dired_text_and_point_match_emacs() {
    let (ev, host) = setup();
    let dir = temp_dir("text");
    let path = dir.display().to_string();
    ev.eval_with(&format!("(dired {path:?})"), &host).unwrap();
    let editor = host.editor();
    let buffer = editor.current_buffer();
    let text = buffer.text();
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines[0], format!("  {path}:"));
    assert!(lines[1].ends_with(" ."), "{}", lines[1]);
    assert!(lines[2].ends_with(" .."), "{}", lines[2]);
    assert!(lines[3].starts_with("  -rw-") && lines[3].ends_with(" a.txt"));
    assert!(lines[4].starts_with("  drwx") && lines[4].ends_with(" sub"));
    assert!(!text.contains("total "));
    // カーソルは, 最初のファイルの名前に置く
    assert!(text[buffer.point()..].starts_with("a.txt\n"));
    assert!(!buffer.modified());
    assert_eq!(editor.current_buffer_name(), "rseditor-dired-text");
    drop(editor);
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn test_dired_of_an_empty_directory_puts_point_at_the_end() {
    let (ev, host) = setup();
    let dir = std::env::temp_dir().join("rseditor-dired-empty");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    ev.eval_with(&format!("(dired {:?})", dir.display().to_string()), &host)
        .unwrap();
    let editor = host.editor();
    let buffer = editor.current_buffer();
    assert_eq!(buffer.point(), buffer.text().len());
    drop(editor);
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn test_dired_reuses_the_buffer_and_reverts() {
    let (ev, host) = setup();
    let dir = temp_dir("reuse");
    let path = format!("{}/", dir.display());
    ev.eval_with(&format!("(dired {path:?})"), &host).unwrap();
    ev.eval_with("(switch-to-buffer \"*scratch*\")", &host)
        .unwrap();
    // 同じディレクトリは, 同じバッファ (末尾の / は区別しない)
    ev.eval_with(&format!("(dired {:?})", dir.display().to_string()), &host)
        .unwrap();
    assert_eq!(host.editor().current_buffer_name(), "rseditor-dired-reuse");
    assert_eq!(host.editor().buffer_list().len(), 2);
    // ファイルを足して作り直すと, 一覧に出る. 元に戻せる前のカーソルを残す
    std::fs::write(dir.join("b.txt"), "").unwrap();
    assert!(!host.editor().current_buffer().text().contains("b.txt"));
    ev.eval_with("(revert-buffer)", &host).unwrap();
    assert!(host.editor().current_buffer().text().contains("b.txt"));
    // dired でないバッファは, 作り直さない
    ev.eval_with("(switch-to-buffer \"*scratch*\")", &host)
        .unwrap();
    assert_eq!(ev.eval_with("(files--revert-dired)", &host).unwrap(), "#f");
    // 消したバッファは忘れて, 作り直す
    ev.eval_with("(kill-buffer \"rseditor-dired-reuse\")", &host)
        .unwrap();
    ev.eval_with(&format!("(dired {path:?})"), &host).unwrap();
    assert_eq!(host.editor().current_buffer_name(), "rseditor-dired-reuse");
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn test_dired_names_are_made_unique_and_root_keeps_its_name() {
    let (ev, host) = setup();
    let a = temp_dir("uniq");
    let b = std::env::temp_dir()
        .join("other")
        .join("rseditor-dired-uniq");
    std::fs::create_dir_all(&b).unwrap();
    ev.eval_with(&format!("(dired {:?})", a.display().to_string()), &host)
        .unwrap();
    ev.eval_with(&format!("(dired {:?})", b.display().to_string()), &host)
        .unwrap();
    assert_eq!(
        host.editor().current_buffer_name(),
        "rseditor-dired-uniq<2>"
    );
    ev.eval_with("(dired \"/\")", &host).unwrap();
    assert_eq!(host.editor().current_buffer_name(), "/");
    std::fs::remove_dir_all(a).unwrap();
    std::fs::remove_dir_all(Path::new(&b).parent().unwrap()).unwrap();
}

#[test]
fn test_dired_other_window_and_the_default_directory() {
    let (ev, host) = setup();
    let dir = temp_dir("other");
    ev.eval_with(
        &format!("(dired-other-window {:?})", dir.display().to_string()),
        &host,
    )
    .unwrap();
    assert_eq!(host.editor().window_list().len(), 2);
    assert_eq!(host.editor().current_buffer_name(), "rseditor-dired-other");
    // 聞くときの始めの入力: dired のバッファはそのディレクトリ, ファイルのバッファはそのディレクトリ, ほかはホーム
    let default = |code: &str| ev.eval_with(code, &host).unwrap();
    assert_eq!(
        default("(dired--default-directory)"),
        format!("{:?}", format!("{}/", dir.display()))
    );
    ev.eval_with("(switch-to-buffer \"*scratch*\")", &host)
        .unwrap();
    assert_eq!(default("(dired--default-directory)"), "\"~/\"");
    let file = dir.join("a.txt");
    ev.eval_with(
        &format!("(find-file-noselect {:?})", file.display().to_string()),
        &host,
    )
    .unwrap();
    ev.eval_with("(switch-to-buffer \"a.txt\")", &host).unwrap();
    assert_eq!(
        default("(dired--default-directory)"),
        format!("{:?}", format!("{}/", dir.display()))
    );
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn test_dired_jump_goes_to_the_line_of_the_file_and_up_a_level() {
    let (ev, host) = setup();
    let dir = temp_dir("jump");
    let file = dir.join("a.txt");
    ev.eval_with(
        &format!("(switch-to-buffer (find-file-noselect {:?}))", file.display().to_string()),
        &host,
    )
    .unwrap();
    ev.eval_with("(dired-jump)", &host).unwrap();
    {
        let editor = host.editor();
        let buffer = editor.current_buffer();
        assert_eq!(editor.current_buffer_name(), "rseditor-dired-jump");
        assert!(buffer.text()[buffer.point()..].starts_with("a.txt\n"));
    }
    // dired のバッファからは, 一つ上のディレクトリの, このディレクトリの行へ
    ev.eval_with("(dired-jump)", &host).unwrap();
    {
        let editor = host.editor();
        let buffer = editor.current_buffer();
        assert!(buffer.text()[buffer.point()..].starts_with("rseditor-dired-jump\n"));
    }
    // 無いファイルは, 一覧を作り直しても見つからない. 先頭のまま
    ev.eval_with(
        &format!("(dired-jump #f {:?})", dir.join("none").display().to_string()),
        &host,
    )
    .unwrap();
    // 別のウィンドウ, ファイルのないバッファ (ホーム), 根
    ev.eval_with("(delete-other-windows)", &host).unwrap();
    ev.eval_with(
        &format!("(dired-jump-other-window {:?})", file.display().to_string()),
        &host,
    )
    .unwrap();
    assert_eq!(host.editor().window_list().len(), 2);
    ev.eval_with("(switch-to-buffer \"*scratch*\")", &host)
        .unwrap();
    ev.eval_with("(dired-jump)", &host).unwrap();
    ev.eval_with("(dired-jump #f \"/\")", &host).unwrap();
    assert_eq!(host.editor().current_buffer_name(), "/");
    std::fs::remove_dir_all(dir).unwrap();
}
