//! 起動のときに読む Lisp のうち, ファイルとバッファのコマンド (`lisp/files.scm`, `lisp/window.scm`) と,
//! 問いの手続き (`lisp/subr.scm`) のテスト.

use std::fs;

use rseditor_core::scheme::evaluator::Evaluator;
use tempfile::tempdir;

mod common;
use common::{FakeBuffer, FakeEditor};

/// 偽のエディタに, 答えを並べる.
///
/// # Arguments
///
/// * `editor` - 偽のエディタ.
/// * `answers` - 答え (前から使う).
fn answer(editor: &FakeEditor, answers: &[&str]) {
    *editor.answers.lock().unwrap() = answers.iter().map(|a| a.to_string()).collect();
    editor.prompts.lock().unwrap().clear();
}

/// 偽のエディタで出した問いを返す.
///
/// # Arguments
///
/// * `editor` - 偽のエディタ.
///
/// # Returns
///
/// 問い.
fn prompts(editor: &FakeEditor) -> Vec<String> {
    editor.prompts.lock().unwrap().clone()
}

/// ファイルのバッファを作って, 出す.
///
/// # Arguments
///
/// * `editor` - 偽のエディタ.
/// * `path` - ファイルのパス (バッファの名前にもする).
/// * `text` - 本文.
/// * `modified` - 書き換えたか.
fn visit(editor: &FakeEditor, path: &str, text: &str, modified: bool) {
    editor.buffers.lock().unwrap().insert(
        path.to_string(),
        FakeBuffer {
            text: text.chars().collect(),
            point: 1,
            file: Some(path.to_string()),
            modified,
            ..Default::default()
        },
    );
    *editor.current.lock().unwrap() = path.to_string();
}

#[test]
fn test_y_or_n_p_and_yes_or_no_p() {
    let ev = Evaluator::new();
    let editor = FakeEditor::new();
    // y と SPC は真, n と DEL は偽. ほかのキーは, 聞き直す
    for (keys, result) in [
        (vec!["y"], "#t"),
        (vec![" "], "#t"),
        (vec!["x", "n"], "#f"),
        (vec!["Backspace"], "#f"),
        (vec!["Delete"], "#f"),
    ] {
        answer(&editor, &keys);
        assert_eq!(
            ev.eval_with("(y-or-n-p \"Go? \")", &editor).unwrap(),
            result
        );
    }
    assert_eq!(prompts(&editor), vec!["Go? (y or n) "], "the last question");
    answer(&editor, &["x", "n"]);
    ev.eval_with("(y-or-n-p \"Go? \")", &editor).unwrap();
    assert_eq!(
        prompts(&editor),
        vec!["Go? (y or n) ", "Please answer y or n.  Go? (y or n) "]
    );
    // yes と no は, 全て書く (前後の空白は除く). ほかの答えは, 聞き直す
    answer(&editor, &["y", " yes "]);
    assert_eq!(
        ev.eval_with("(yes-or-no-p \"Go? \")", &editor).unwrap(),
        "#t"
    );
    assert_eq!(
        prompts(&editor),
        vec![
            "Go? (yes or no) ",
            "Please answer yes or no.  Go? (yes or no) "
        ]
    );
    answer(&editor, &["no"]);
    assert_eq!(
        ev.eval_with("(yes-or-no-p \"Go? \")", &editor).unwrap(),
        "#f"
    );
}

#[test]
fn test_read_key_and_write_buffer_arguments() {
    let ev = Evaluator::new();
    let editor = FakeEditor::new();
    answer(&editor, &["q"]);
    assert_eq!(ev.eval_with("(read-key)", &editor).unwrap(), "\"q\"");
    assert_eq!(prompts(&editor), vec![""]);
    assert_eq!(
        ev.eval_with("(read-key \"a\" \"b\" \"c\")", &editor).unwrap_err(),
        "read-key expects 0 to 2 arguments"
    );
    for code in [
        "(files--write-buffer)",
        "(files--write-buffer \"a\" \"b\" \"c\")",
    ] {
        assert_eq!(
            ev.eval_with(code, &editor).unwrap_err(),
            "files--write-buffer expects 1 or 2 arguments"
        );
    }
}

#[test]
fn test_file_name_directory() {
    let ev = Evaluator::new();
    assert_eq!(
        ev.eval("(list (file-name-directory \"/a/b.txt\") (file-name-directory \"/a/\") (file-name-directory \"b.txt\"))")
            .unwrap(),
        "(\"/a/\" \"/a/\" #f)"
    );
}

#[test]
fn test_switch_to_buffer() {
    let ev = Evaluator::new();
    let editor = FakeEditor::new();
    // バッファが 1 つなら, 既定は無い. 空の入力は, 何もしない
    answer(&editor, &[""]);
    ev.run_command("switch-to-buffer", &editor).unwrap();
    assert_eq!(prompts(&editor), vec!["Switch to buffer: "]);
    assert_eq!(editor.name(), "*scratch*");
    // 無い名前は, 作る
    answer(&editor, &["new"]);
    ev.run_command("switch-to-buffer", &editor).unwrap();
    assert_eq!(editor.name(), "new");
    // 空の入力は, ほかのバッファ. 候補は, 全てのバッファ
    answer(&editor, &[""]);
    ev.run_command("switch-to-buffer", &editor).unwrap();
    assert_eq!(
        prompts(&editor),
        vec!["Switch to buffer (default *scratch*): "]
    );
    assert_eq!(
        *editor.completions.lock().unwrap(),
        vec!["*scratch*", "new"]
    );
    assert_eq!(editor.name(), "*scratch*");
    // 引数で渡す
    ev.eval_with("(switch-to-buffer \"new\")", &editor).unwrap();
    assert_eq!(editor.name(), "new");
}

#[test]
fn test_save_buffer() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("a.txt").to_string_lossy().to_string();
    let ev = Evaluator::new();
    let editor = FakeEditor::new();
    // 書き換えていなければ, 書かない
    visit(&editor, &path, "old", false);
    ev.run_command("save-buffer", &editor).unwrap();
    assert_eq!(editor.message(), "(No changes need to be saved)");
    assert!(!dir.path().join("a.txt").exists());
    // 書き換えていれば, 書く
    visit(&editor, &path, "new", true);
    ev.run_command("save-buffer", &editor).unwrap();
    assert_eq!(fs::read_to_string(&path).unwrap(), "new");
    assert_eq!(editor.message(), format!("Wrote {}", path));
    assert!(!editor.buffer().modified);

    // ファイルの無いバッファは, ファイルの名前を聞く. ほかのファイルを置き換えるときは, 聞く
    *editor.current.lock().unwrap() = "*scratch*".to_string();
    editor.with_buffer(|b| b.text = "scratch".chars().collect());
    answer(&editor, &[&path, "n"]);
    ev.run_command("save-buffer", &editor).unwrap();
    assert_eq!(
        prompts(&editor),
        vec![
            "Save file: ".to_string(),
            format!("File ‘{}’ exists; overwrite? (y or n) ", path)
        ]
    );
    assert_eq!(editor.message(), "Canceled");
    assert_eq!(fs::read_to_string(&path).unwrap(), "new");
    answer(&editor, &[&path, "y"]);
    ev.run_command("save-buffer", &editor).unwrap();
    assert_eq!(fs::read_to_string(&path).unwrap(), "scratch");
    assert_eq!(editor.buffer().file, Some(path.clone()));
}

#[test]
fn test_write_file() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("a.txt").to_string_lossy().to_string();
    let other = dir.path().join("b.txt").to_string_lossy().to_string();
    let ev = Evaluator::new();
    let editor = FakeEditor::new();
    visit(&editor, &path, "text", false);
    // 新しいファイルと, 今のファイルは, 聞かずに書く
    for target in [&other, &path] {
        answer(&editor, &[target]);
        ev.run_command("write-file", &editor).unwrap();
        assert_eq!(prompts(&editor), vec!["Write file: "]);
        assert_eq!(fs::read_to_string(target).unwrap(), "text");
    }
    // 引数で渡す
    editor.with_buffer(|b| b.text = "more".chars().collect());
    answer(&editor, &["y"]);
    ev.eval_with(&format!("(write-file {:?})", other), &editor)
        .unwrap();
    assert_eq!(fs::read_to_string(&other).unwrap(), "more");
}

#[test]
fn test_insert_file() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("in.txt");
    fs::write(&path, "inserted").unwrap();
    let ev = Evaluator::new();
    let editor = FakeEditor::new();
    editor.with_buffer(|b| {
        b.text = "ab".chars().collect();
        b.point = 2;
    });
    // カーソルは, 入れた文字の前に残す
    answer(&editor, &[&path.to_string_lossy()]);
    ev.run_command("insert-file", &editor).unwrap();
    assert_eq!(prompts(&editor), vec!["Insert file: "]);
    assert_eq!(editor.text(), "ainsertedb");
    assert_eq!(editor.buffer().point, 2);
    ev.eval_with(
        &format!("(insert-file {:?})", path.to_string_lossy()),
        &editor,
    )
    .unwrap();
    assert_eq!(editor.text(), "ainsertedinsertedb");
}

#[test]
fn test_find_alternate_file() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("a.txt").to_string_lossy().to_string();
    let other = dir.path().join("b.txt").to_string_lossy().to_string();
    fs::write(&other, "other").unwrap();
    let ev = Evaluator::new();
    let editor = FakeEditor::new();
    // 書き換えたファイルのバッファは, 捨ててよいか聞く. 入力の始めは, 今のファイル
    visit(&editor, &path, "edited", true);
    answer(&editor, &[&other, "no"]);
    ev.run_command("find-alternate-file", &editor).unwrap();
    assert_eq!(
        prompts(&editor),
        vec![
            "Find alternate file: ".to_string(),
            "Kill and replace buffer ‘".to_string() + &path + "’ without saving it? (yes or no) "
        ]
    );
    assert_eq!(editor.message(), "Aborted");
    assert_eq!(editor.name(), path);
    answer(&editor, &[&other, "yes"]);
    ev.run_command("find-alternate-file", &editor).unwrap();
    assert_eq!(editor.name(), other);
    assert_eq!(editor.text(), "other");
    assert!(!editor.buffers.lock().unwrap().contains_key(&path));
    // 書き換えていなければ, 聞かない. 引数で渡す
    ev.eval_with(&format!("(find-alternate-file {:?})", path), &editor)
        .unwrap();
    assert_eq!(editor.name(), path);
    assert!(!editor.buffers.lock().unwrap().contains_key(&other));
}

#[test]
fn test_save_some_buffers() {
    let dir = tempdir().unwrap();
    let paths: Vec<String> = ["a", "b", "c", "d"]
        .iter()
        .map(|n| dir.path().join(n).to_string_lossy().to_string())
        .collect();
    let ev = Evaluator::new();
    let editor = FakeEditor::new();
    // 書き換えたファイルのバッファが無ければ, 知らせる
    ev.run_command("save-some-buffers", &editor).unwrap();
    assert_eq!(editor.message(), "(No files need saving)");

    let reset = || {
        for path in &paths {
            let _ = fs::remove_file(path);
            visit(&editor, path, "x", true);
        }
    };
    let saved = || {
        paths
            .iter()
            .map(|p| fs::metadata(p).is_ok())
            .collect::<Vec<_>>()
    };
    // y と SPC で書き, n と DEL で飛ばす. ほかのキーは, 聞き直す
    reset();
    answer(&editor, &["y", "n", "x", " ", "Backspace"]);
    ev.run_command("save-some-buffers", &editor).unwrap();
    assert_eq!(saved(), vec![true, false, true, false]);
    assert_eq!(
        prompts(&editor)[..2],
        [
            format!("Save file {}? (y, n, !, ., q) ", paths[0]),
            format!("Save file {}? (y, n, !, ., q) ", paths[1])
        ]
    );
    // ! で, 残りを全て書く
    reset();
    answer(&editor, &["Delete", "!"]);
    ev.run_command("save-some-buffers", &editor).unwrap();
    assert_eq!(saved(), vec![false, true, true, true]);
    // . で, 書いて終える
    reset();
    answer(&editor, &["n", "."]);
    ev.run_command("save-some-buffers", &editor).unwrap();
    assert_eq!(saved(), vec![false, true, false, false]);
    // q と RET で, 終える (書かなければ, メッセージを消す)
    for key in ["q", "Enter"] {
        reset();
        answer(&editor, &[key]);
        ev.run_command("save-some-buffers", &editor).unwrap();
        assert_eq!(saved(), vec![false, false, false, false]);
        assert_eq!(editor.message(), "");
    }
    // 全て飛ばしても, メッセージを消す
    reset();
    answer(&editor, &["n", "n", "n", "n"]);
    ev.run_command("save-some-buffers", &editor).unwrap();
    assert_eq!(editor.message(), "");
}

#[test]
fn test_file_name_nondirectory_and_auto_save_file_name() {
    let ev = Evaluator::new();
    let editor = FakeEditor::new();
    assert_eq!(
        ev.eval("(list (file-name-nondirectory \"/a/b.txt\") (file-name-nondirectory \"/a/\") (file-name-nondirectory \"b.txt\"))")
            .unwrap(),
        "(\"b.txt\" \"\" \"b.txt\")"
    );
    // ファイルのバッファは, 同じディレクトリの #名前#. ファイルのないバッファは #f
    assert_eq!(
        ev.eval_with("(make-auto-save-file-name)", &editor).unwrap(),
        "#f"
    );
    visit(&editor, "/tmp/a.txt", "abc", false);
    assert_eq!(
        ev.eval_with("(make-auto-save-file-name)", &editor).unwrap(),
        "\"/tmp/#a.txt#\""
    );
    visit(&editor, "a.txt", "abc", false);
    assert_eq!(
        ev.eval_with("(make-auto-save-file-name)", &editor).unwrap(),
        "\"#a.txt#\""
    );
}

#[test]
fn test_file_newer_than_file_p() {
    let ev = Evaluator::new();
    let dir = tempdir().unwrap();
    let old = dir.path().join("old").to_str().unwrap().to_string();
    let new = dir.path().join("new").to_str().unwrap().to_string();
    let none = dir.path().join("none").to_str().unwrap().to_string();
    fs::write(&old, "").unwrap();
    std::thread::sleep(std::time::Duration::from_millis(50));
    fs::write(&new, "").unwrap();
    for (a, b, result) in [
        (&new, &old, "#t"),
        (&old, &new, "#f"),
        (&old, &none, "#t"),
        (&none, &old, "#f"),
        (&none, &none, "#f"),
    ] {
        assert_eq!(
            ev.eval(&format!("(file-newer-than-file-p \"{}\" \"{}\")", a, b))
                .unwrap(),
            result
        );
    }
}

#[test]
fn test_set_buffer_text_and_revert_dired_arguments() {
    let ev = Evaluator::new();
    let editor = FakeEditor::new();
    assert_eq!(
        ev.eval_with("(files--set-buffer-text \"a\")", &editor)
            .unwrap_err(),
        "files--set-buffer-text expects 2 arguments, given 1"
    );
    assert!(ev
        .eval_with("(files--set-buffer-text 1 #f)", &editor)
        .is_err());
    assert!(ev.eval_with("(files--revert-dired 1)", &editor).is_err());
}

#[test]
fn test_revert_buffer() {
    let ev = Evaluator::new();
    let editor = FakeEditor::new();
    let dir = tempdir().unwrap();
    let path = dir.path().join("a.txt").to_str().unwrap().to_string();
    fs::write(&path, "new").unwrap();
    // ファイルのないバッファ
    ev.run_command("revert-buffer", &editor).unwrap();
    assert_eq!(
        editor.message(),
        "Buffer does not seem to be associated with any file"
    );
    // 変更なし: 聞いてから読み直す. no なら, そのまま
    visit(&editor, &path, "old", false);
    answer(&editor, &["no"]);
    ev.run_command("revert-buffer", &editor).unwrap();
    assert_eq!(
        prompts(&editor),
        vec![format!("Revert buffer from file {}? (yes or no) ", path)]
    );
    assert_eq!(editor.buffer().text.iter().collect::<String>(), "old");
    assert_eq!(editor.message(), "");
    answer(&editor, &["yes"]);
    ev.run_command("revert-buffer", &editor).unwrap();
    assert_eq!(editor.buffer().text.iter().collect::<String>(), "new");
    assert!(!editor.buffer().modified);
    // 変更あり: 捨てるかを聞く
    visit(&editor, &path, "old", true);
    answer(&editor, &["yes"]);
    ev.run_command("revert-buffer", &editor).unwrap();
    assert_eq!(
        prompts(&editor),
        vec![format!(
            "Discard edits and reread from {}? (yes or no) ",
            path
        )]
    );
    assert!(!editor.buffer().modified);
    // dired は, 聞かずに作り直す
    editor
        .buffers
        .lock()
        .unwrap()
        .insert("*dired*".to_string(), FakeBuffer::default());
    *editor.current.lock().unwrap() = "*dired*".to_string();
    answer(&editor, &[]);
    ev.run_command("revert-buffer", &editor).unwrap();
    assert_eq!(editor.message(), "Listed again");
    assert!(prompts(&editor).is_empty());
}

#[test]
fn test_recover_this_file() {
    let ev = Evaluator::new();
    let editor = FakeEditor::new();
    let dir = tempdir().unwrap();
    let path = dir.path().join("a.txt").to_str().unwrap().to_string();
    let auto = dir.path().join("#a.txt#").to_str().unwrap().to_string();
    ev.run_command("recover-this-file", &editor).unwrap();
    assert_eq!(editor.message(), "This buffer is not visiting a file");
    // ファイルがあって, 自動保存のファイルが無い
    fs::write(&path, "saved").unwrap();
    visit(&editor, &path, "saved", false);
    ev.run_command("recover-this-file", &editor).unwrap();
    assert_eq!(
        editor.message(),
        format!("Auto save file {} does not exist", auto)
    );
    // 自動保存のファイルの方が古い
    fs::write(&auto, "auto").unwrap();
    std::thread::sleep(std::time::Duration::from_millis(50));
    fs::write(&path, "saved").unwrap();
    ev.run_command("recover-this-file", &editor).unwrap();
    assert_eq!(
        editor.message(),
        format!("Auto-save file {} not current", auto)
    );
    // 自動保存のファイルの方が新しい: no ならそのまま, yes なら読み直して変更ありにする
    std::thread::sleep(std::time::Duration::from_millis(50));
    fs::write(&auto, "auto").unwrap();
    answer(&editor, &["no"]);
    ev.run_command("recover-this-file", &editor).unwrap();
    assert_eq!(
        prompts(&editor),
        vec![format!("Recover auto save file {}? (yes or no) ", auto)]
    );
    assert_eq!(editor.buffer().text.iter().collect::<String>(), "saved");
    assert_eq!(editor.message(), "");
    answer(&editor, &["yes"]);
    ev.run_command("recover-this-file", &editor).unwrap();
    assert_eq!(editor.buffer().text.iter().collect::<String>(), "auto");
    assert!(editor.buffer().modified);
    // ファイルが無くても, 自動保存のファイルがあれば読み直せる
    fs::remove_file(&path).unwrap();
    visit(&editor, &path, "", false);
    answer(&editor, &["yes"]);
    ev.run_command("recover-this-file", &editor).unwrap();
    assert_eq!(editor.buffer().text.iter().collect::<String>(), "auto");
}
