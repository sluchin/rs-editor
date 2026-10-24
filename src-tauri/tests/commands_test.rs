//! コマンド (`commands`) のテスト.

use std::fs;

use tempfile::tempdir;

use rseditor::commands::{
    eval_code, eval_code_with, home_dir, init_file_path, init_file_path_from, list_directory,
    load_init_file, path_exists, read_file_content, run_command, write_file_content, EvalResponse,
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
