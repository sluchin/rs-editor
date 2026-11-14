//! ファイルの読み書きの組み込み (`core/src/files.rs`) と, バックアップと自動保存 (`lisp/files.scm`) のテスト.

use std::fs;

use rseditor_core::buffer::Buffer;
use rseditor_core::editor::Editor;
use rseditor_core::files::{
    auto_save, auto_save_file_name, insert_file_contents, note_key, AUTO_SAVE_INTERVAL,
};
use rseditor_core::scheme::editor_host::EditorHost;
use rseditor_core::scheme::evaluator::Evaluator;
use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::value::Value;

/// 画面の操作に答える偽物 (メッセージを受け, 問いには y と答える).
struct Ui;

impl Host for Ui {
    fn call(&self, op: &str, _args: Vec<Value>) -> Result<Value, rseditor_core::error::Error> {
        match op {
            "message" => Ok(Value::Nil),
            "read-key" => Ok(Value::String("y".to_string())),
            _ => Err(format!("ui: {op}").into()),
        }
    }
}

fn setup() -> (Evaluator, EditorHost<Ui>) {
    (Evaluator::new(), EditorHost::new(Editor::new(), Ui))
}

#[test]
fn test_auto_save_file_name() {
    assert_eq!(auto_save_file_name("/tmp/a.txt"), "/tmp/#a.txt#");
    assert_eq!(auto_save_file_name("a.txt"), "#a.txt#");
}

#[test]
fn test_insert_file_contents() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("a.txt");
    let path = file.to_str().unwrap();
    fs::write(&file, "new\n").unwrap();
    // 訪れなければ, カーソルの後ろに入れ, 書き換えたことになる
    let mut buffer = Buffer::new("b");
    buffer.insert("ab");
    buffer.set_point(1);
    assert_eq!(insert_file_contents(&mut buffer, path, false), Ok(4));
    assert_eq!(buffer.text(), "anew\nb");
    assert_eq!(buffer.point(), 1);
    assert!(buffer.modified());
    // 空でないバッファに訪れるときも, 入れる (書き換えていないことになる)
    assert_eq!(insert_file_contents(&mut buffer, path, true), Ok(4));
    assert_eq!(buffer.text(), "anew\nnew\nb");
    assert!(!buffer.modified());
    assert_eq!(buffer.file(), Some(path));
    // 無いファイル, 読めないファイル
    let missing = dir.path().join("none");
    assert_eq!(
        insert_file_contents(&mut buffer, missing.to_str().unwrap(), true)
            .map_err(|e| e.to_string()),
        Err(format!(
            "Opening input file: No such file or directory, {}",
            missing.display()
        ))
    );
    let error = insert_file_contents(&mut buffer, dir.path().to_str().unwrap(), false).unwrap_err();
    assert_eq!(error.kind(), "file-error");
    assert!(error.ends_with(dir.path().to_str().unwrap()));
}

#[test]
fn test_backup_once_and_delete_the_auto_save_file() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("a.txt");
    let path = file.to_str().unwrap().to_string();
    let backup = format!("{path}~");
    fs::write(&file, "old\n").unwrap();
    let (ev, host) = setup();
    let eval = |code: &str| ev.eval_with(code, &host).unwrap();
    eval(&format!("(switch-to-buffer (find-file-noselect {path:?}))"));
    eval("(insert \"new\\n\")");
    // 自動保存してから保存すると, 自動保存のファイルを消す
    assert_eq!(auto_save(&mut host.editor()), 1);
    assert_eq!(
        fs::read_to_string(auto_save_file_name(&path)).unwrap(),
        "new\nold\n"
    );
    // 変えていなければ, 自動保存しない
    assert_eq!(auto_save(&mut host.editor()), 0);
    eval("(save-buffer)");
    assert_eq!(fs::read_to_string(&file).unwrap(), "new\nold\n");
    assert_eq!(fs::read_to_string(&backup).unwrap(), "old\n");
    assert!(!std::path::Path::new(&auto_save_file_name(&path)).exists());
    assert_eq!(eval("(buffer-modified-p)"), "#f");
    // 2 度目の保存は, バックアップを作り直さない. 保存した後は, 自動保存しない
    eval("(insert \"more\\n\") (save-buffer)");
    assert_eq!(fs::read_to_string(&backup).unwrap(), "old\n");
    assert_eq!(auto_save(&mut host.editor()), 0);
    // 別のファイルに書くと (write-file), バッファの名前が変わり, そのファイルのバックアップを作る
    let other = dir.path().join("b.txt");
    fs::write(&other, "b\n").unwrap();
    eval(&format!("(write-file {:?})", other.to_str().unwrap()));
    assert_eq!(eval("(buffer-name)"), "\"b.txt\"");
    assert_eq!(
        fs::read_to_string(format!("{}~", other.display())).unwrap(),
        "b\n"
    );
    // 新しいファイルは, 初めの保存ではバックアップを作らず, 次の保存で作る (Emacs と同じ)
    let fresh = dir.path().join("c.txt");
    let fresh_backup = format!("{}~", fresh.display());
    eval(&format!(
        "(switch-to-buffer (find-file-noselect {:?})) (insert \"1\") (save-buffer)",
        fresh.to_str().unwrap()
    ));
    assert!(!std::path::Path::new(&fresh_backup).exists());
    eval("(insert \"2\") (save-buffer)");
    assert_eq!(fs::read_to_string(&fresh_backup).unwrap(), "1\n");
    // do-auto-save は, Lisp からも呼べる
    eval("(insert \"3\") (do-auto-save) (do-auto-save)");
    assert_eq!(
        fs::read_to_string(auto_save_file_name(fresh.to_str().unwrap())).unwrap(),
        "123\n"
    );
    // バッファの訪れるファイルを外す
    eval("(files--set-buffer-file-name #f)");
    assert_eq!(eval("(buffer-file-name)"), "#f");
}

#[test]
fn test_auto_save_skips_the_files_it_cannot_write() {
    let dir = tempfile::tempdir().unwrap();
    let mut editor = Editor::new();
    let missing = dir.path().join("none").join("a.txt");
    let buffer = editor.get_buffer_create("a.txt");
    buffer.set_file(Some(missing.to_str().unwrap().to_string()));
    buffer.insert("x");
    assert_eq!(auto_save(&mut editor), 0);
}

/// ミニバッファの問いに, 順に答える偽物.
struct Answers(std::sync::Mutex<Vec<&'static str>>);

impl Host for Answers {
    fn call(&self, op: &str, _args: Vec<Value>) -> Result<Value, rseditor_core::error::Error> {
        match op {
            "message" => Ok(Value::Nil),
            "read-string" | "read-file-name" => {
                Ok(Value::String(self.0.lock().unwrap().remove(0).to_string()))
            }
            _ => Err(format!("ui: {op}").into()),
        }
    }
}

#[test]
fn test_the_commands_of_the_builtins() {
    let dir = tempfile::tempdir().unwrap();
    let path = |name: &str| dir.path().join(name).to_str().unwrap().to_string();
    let (a, b) = (path("a.txt"), path("b.txt"));
    fs::write(&a, "a\n").unwrap();
    let ev = Evaluator::new();
    let run = |answers: Vec<&'static str>, code: &str, command: &str| {
        let host = EditorHost::new(Editor::new(), Answers(std::sync::Mutex::new(answers)));
        ev.eval_with(code, &host).unwrap();
        let result = ev.run_command(command, &host);
        (result, host)
    };
    // M-x write-region: リージョンを書く. 既にあるファイルは, 聞いてから書き換える
    let leak = |s: String| -> &'static str { Box::leak(s.into_boxed_str()) };
    let region = "(insert \"hello\") (set-mark 2) (goto-char 4)";
    let (result, _) = run(vec![leak(b.clone())], region, "write-region");
    assert!(result.is_ok());
    assert_eq!(fs::read_to_string(&b).unwrap(), "el");
    let (result, _) = run(vec![leak(b.clone()), "no"], region, "write-region");
    assert_eq!(
        result.map_err(|e| e.to_string()),
        Err(format!("File already exists: {b}"))
    );
    let (result, _) = run(
        vec![leak(b.clone()), "yes"],
        "(insert \"xyz\") (set-mark 1)",
        "write-region",
    );
    assert!(result.is_ok());
    assert_eq!(fs::read_to_string(&b).unwrap(), "xyz");
    // M-x copy-file: ディレクトリに写すと, 同じ名前のファイルにする
    let sub = path("sub");
    fs::create_dir(&sub).unwrap();
    let (result, _) = run(vec![leak(a.clone()), leak(sub.clone())], "", "copy-file");
    assert!(result.is_ok());
    assert_eq!(fs::read_to_string(format!("{sub}/a.txt")).unwrap(), "a\n");
    let (result, _) = run(
        vec![leak(a.clone()), leak(b.clone()), "no"],
        "",
        "copy-file",
    );
    assert_eq!(
        result.map_err(|e| e.to_string()),
        Err(format!("File already exists: {b}"))
    );
    let (result, _) = run(
        vec![leak(a.clone()), leak(b.clone()), "yes"],
        "",
        "copy-file",
    );
    assert!(result.is_ok());
    assert_eq!(fs::read_to_string(&b).unwrap(), "a\n");
    // M-x set-visited-file-name: バッファの名前も変わり, 書き換えたことになる
    let open = format!("(switch-to-buffer (find-file-noselect {a:?}))");
    let (result, host) = run(vec![leak(path("c.txt"))], &open, "set-visited-file-name");
    assert!(result.is_ok());
    let eval = |code: &str| ev.eval_with(code, &host).unwrap();
    assert_eq!(eval("(buffer-name)"), "\"c.txt\"");
    assert_eq!(eval("(buffer-modified-p)"), "#t");
    // M-x basic-save-buffer, M-x do-auto-save
    assert!(ev.run_command("basic-save-buffer", &host).is_ok());
    assert_eq!(fs::read_to_string(path("c.txt")).unwrap(), "a\n");
    eval("(insert \"1\")");
    assert!(ev.run_command("do-auto-save", &host).is_ok());
    assert_eq!(fs::read_to_string(path("#c.txt#")).unwrap(), "1a\n");
    // 2 度読んでも, 組み込みを呼ぶ
    eval(include_str!("../../lisp/files.scm"));
    eval(&format!("(write-region \"x\" #f {b:?})"));
    assert_eq!(fs::read_to_string(&b).unwrap(), "x");
    eval(&format!("(copy-file {a:?} {b:?} #t) (do-auto-save)"));
    assert_eq!(fs::read_to_string(&b).unwrap(), "a\n");
}

#[test]
fn test_copy_file() {
    let dir = tempfile::tempdir().unwrap();
    let a = dir.path().join("a");
    let b = dir.path().join("b");
    fs::write(&a, "a").unwrap();
    let ev = Evaluator::new();
    let (a, b) = (a.to_str().unwrap(), b.to_str().unwrap());
    ev.eval(&format!("(copy-file {a:?} {b:?})")).unwrap();
    assert_eq!(fs::read_to_string(b).unwrap(), "a");
    // 既にあれば, 3 つ目の引数が真のときだけ置き換える
    assert_eq!(
        ev.eval(&format!("(copy-file {a:?} {b:?})"))
            .map_err(|e| e.to_string()),
        Err(format!("File already exists: {b}"))
    );
    ev.eval(&format!("(copy-file {a:?} {b:?} #t)")).unwrap();
    assert!(ev
        .eval(&format!("(copy-file \"{a}-none\" {b:?} #t)"))
        .unwrap_err()
        .starts_with(&format!("{a}-none: ")));
    assert_eq!(
        ev.eval("(copy-file \"a\")").map_err(|e| e.to_string()),
        Err("copy-file expects 2 or 3 arguments".to_string())
    );
}

#[test]
fn test_note_key_asks_for_an_auto_save_every_interval() {
    let mut editor = Editor::new();
    for _ in 1..AUTO_SAVE_INTERVAL {
        assert!(!note_key(&mut editor));
    }
    assert!(note_key(&mut editor));
    // 自動保存すると, 数え直す
    auto_save(&mut editor);
    assert_eq!(editor.keys_since_auto_save, 0);
    assert!(!note_key(&mut editor));
}
