//! コマンド (`commands`) のテスト.

use std::fs;

use tempfile::tempdir;

use rseditor::commands::{
    backup_file, bookmark_file_path, bookmark_file_path_from, delete_file, eval_code,
    eval_code_with, file_newer_than, home_dir, init_file_path, init_file_path_from, list_directory,
    list_directory_long, list_directory_long_with, load_init_file, path_exists, read_file_content,
    run_command, write_file_content, EvalResponse,
};
use rseditor::scheme::evaluator::{Evaluator, KeyBinding};
use rseditor::scheme::host::NoHost;

#[test]
fn test_eval_code_returns_the_result_or_the_error() {
    let evaluator = Evaluator::new();
    assert_eq!(
        eval_code(&evaluator, "(+ 1 2)"),
        EvalResponse {
            result: "3".to_string(),
            error: None,
            commands: vec![],
            keys: vec![],
        }
    );
    assert_eq!(eval_code(&Evaluator::default(), "1").result, "1");
    let failed = eval_code(&evaluator, "(");
    assert!(failed.result.is_empty());
    assert!(failed.error.is_some());
}

#[test]
fn test_write_and_read_file_content() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("a.scm").to_string_lossy().into_owned();
    assert!(!path_exists(path.clone()));
    write_file_content(path.clone(), "(+ 1 2)".to_string()).unwrap();
    assert!(path_exists(path.clone()));
    assert_eq!(read_file_content(path.clone()).unwrap(), "(+ 1 2)");
    assert_eq!(fs::read_to_string(&path).unwrap(), "(+ 1 2)");
}

#[test]
fn test_read_and_write_fail_with_the_reason() {
    let dir = tempdir().unwrap();
    let missing = dir.path().join("none/a.scm").to_string_lossy().into_owned();
    assert!(read_file_content(missing.clone()).is_err());
    assert!(write_file_content(missing, "x".to_string()).is_err());
}

#[test]
fn test_list_directory_marks_the_directories() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("b.scm"), "").unwrap();
    fs::create_dir(dir.path().join("a")).unwrap();
    let path = dir.path().to_string_lossy().into_owned();
    assert_eq!(list_directory(path.clone()).unwrap(), vec!["a/", "b.scm"]);
    assert!(list_directory(path + "/none").is_err());
}

#[test]
fn test_home_dir_reads_home() {
    // テストの実行環境には, HOME がある
    assert_eq!(home_dir().unwrap(), std::env::var("HOME").unwrap());
}

#[test]
fn test_responses_carry_the_lisp_commands_and_keys() {
    let evaluator = Evaluator::new();
    let defined = eval_code_with(
        &evaluator,
        "(define-command (hi) 1) (global-set-key \"C-c h\" 'hi)",
        &NoHost,
    );
    assert_eq!(defined.result, "hi\nhi");
    assert_eq!(defined.commands, vec!["hi"]);
    assert_eq!(
        defined.keys,
        vec![KeyBinding {
            key: "Ctrl-c h".to_string(),
            command: "hi".to_string(),
        }]
    );
    assert_eq!(run_command(&evaluator, "hi", &NoHost).result, "1");
    assert_eq!(
        run_command(&evaluator, "none", &NoHost).error.unwrap(),
        "Undefined command: none"
    );
}

#[test]
fn test_load_init_file() {
    let dir = tempdir().unwrap();
    let path = init_file_path_from(dir.path());
    assert!(path.ends_with("rseditor/init.scm"));
    assert!(init_file_path().unwrap().ends_with("rseditor/init.scm"));
    let evaluator = Evaluator::new();
    // ファイルがなければ, 何もしない
    assert_eq!(
        load_init_file(&evaluator, Some(&path), &NoHost),
        eval_code(&evaluator, "")
    );
    assert_eq!(load_init_file(&evaluator, None, &NoHost).error, None);

    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, "(define-command (hello) 1)").unwrap();
    let loaded = load_init_file(&evaluator, Some(&path), &NoHost);
    assert_eq!(loaded.error, None);
    assert_eq!(loaded.commands, vec!["hello"]);

    fs::write(&path, "(car 1)").unwrap();
    assert_eq!(
        load_init_file(&evaluator, Some(&path), &NoHost)
            .error
            .unwrap(),
        format!("{}: car expects a non-empty list", path.display())
    );

    // 読めない (ディレクトリ)
    let unreadable = dir.path();
    let failed = load_init_file(&evaluator, Some(unreadable), &NoHost);
    assert!(failed
        .error
        .unwrap()
        .starts_with(&format!("{}: ", unreadable.display())));
}

#[test]
fn test_bookmark_file_path_makes_the_directory() {
    let dir = tempdir().unwrap();
    let path = bookmark_file_path_from(Some(dir.path())).unwrap();
    assert!(path.ends_with("rseditor/bookmarks"));
    assert!(dir.path().join("rseditor").is_dir());
    assert_eq!(
        bookmark_file_path_from(None).unwrap_err(),
        "No config directory"
    );
    // 作れない (ファイルの下の) ディレクトリ
    let file = dir.path().join("file");
    std::fs::write(&file, "").unwrap();
    assert!(bookmark_file_path_from(Some(&file)).is_err());
}

#[test]
fn test_bookmark_file_path_uses_the_config_directory() {
    assert!(bookmark_file_path()
        .unwrap()
        .ends_with("rseditor/bookmarks"));
}

#[test]
fn test_backup_file_copies_the_previous_version() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("a.scm").to_string_lossy().into_owned();
    // ファイルが無ければ, 何もしない
    backup_file(path.clone()).unwrap();
    assert!(!path_exists(format!("{}~", path)));
    fs::write(&path, "old").unwrap();
    backup_file(path.clone()).unwrap();
    assert_eq!(fs::read_to_string(format!("{}~", path)).unwrap(), "old");
    // バックアップを書けなければ, 理由を返す
    fs::create_dir(format!("{}~", dir.path().join("b").display())).unwrap();
    let b = dir.path().join("b").to_string_lossy().into_owned();
    fs::write(&b, "b").unwrap();
    assert!(backup_file(b).is_err());
}

#[test]
fn test_delete_file_ignores_missing_files() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("#a#").to_string_lossy().into_owned();
    delete_file(path.clone()).unwrap();
    fs::write(&path, "x").unwrap();
    delete_file(path.clone()).unwrap();
    assert!(!path_exists(path));
    // ディレクトリは消せない
    assert!(delete_file(dir.path().to_string_lossy().into_owned()).is_err());
}

#[test]
fn test_file_newer_than_compares_the_modified_times() {
    let dir = tempdir().unwrap();
    let a = dir.path().join("a").to_string_lossy().into_owned();
    let b = dir.path().join("b").to_string_lossy().into_owned();
    assert!(!file_newer_than(a.clone(), b.clone()));
    fs::write(&a, "a").unwrap();
    assert!(file_newer_than(a.clone(), b.clone()));
    fs::write(&b, "b").unwrap();
    let old = std::time::SystemTime::now() - std::time::Duration::from_secs(60);
    fs::File::options()
        .write(true)
        .open(&a)
        .unwrap()
        .set_modified(old)
        .unwrap();
    assert!(!file_newer_than(a.clone(), b.clone()));
    assert!(file_newer_than(b, a));
}

#[test]
fn test_list_directory_long_runs_ls() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("a.txt"), "hi").unwrap();
    let path = dir.path().to_string_lossy().into_owned();
    let listing = list_directory_long(path.clone()).unwrap();
    assert!(listing.lines().any(|line| line.ends_with(" a.txt")));
    // ディレクトリを読めないか, ls が無いか, 失敗すれば, 理由を返す
    assert!(list_directory_long(format!("{}/none", path)).is_err());
    assert!(list_directory_long_with("/nonexistent-ls", &path).is_err());
    assert_eq!(list_directory_long_with("false", &path), Err(String::new()));
}
