//! コマンド (`commands`) のテスト.

use std::fs;

use tempfile::tempdir;

use rseditor::commands::{
    eval_code, home_dir, path_exists, read_file_content, write_file_content, EvalResponse,
};
use rseditor::scheme::evaluator::Evaluator;

#[test]
fn test_eval_code_returns_the_result_or_the_error() {
    let evaluator = Evaluator::new();
    assert_eq!(
        eval_code(&evaluator, "(+ 1 2)"),
        EvalResponse {
            result: "3".to_string(),
            error: None,
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
fn test_home_dir_reads_home() {
    // テストの実行環境には, HOME がある
    assert_eq!(home_dir().unwrap(), std::env::var("HOME").unwrap());
}
