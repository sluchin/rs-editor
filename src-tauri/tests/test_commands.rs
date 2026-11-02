//! コマンド (`commands`) のテスト.

use std::fs;

use tempfile::tempdir;

use rseditor::commands::{
    home_dir, init_file_candidates, init_file_path, list_directory, project_root,
};

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
    assert_eq!(
        home_dir().unwrap(),
        dirs::home_dir().unwrap().to_string_lossy()
    );
    assert_eq!(home_dir().unwrap(), std::env::var("HOME").unwrap());
}

#[test]
fn test_init_file_candidates() {
    let dir = tempdir().unwrap();
    let home = dir.path().join("home");
    let config = dir.path().join("config");
    assert_eq!(
        init_file_candidates(Some(&home), Some(&config)),
        vec![
            home.join(".rseditor"),
            home.join(".rseditor.d/init.scm"),
            config.join("rseditor/init.scm")
        ]
    );
    assert!(init_file_candidates(None, None).is_empty());
    // あるファイルだけ (どれも無いこともある)
    if let Some(found) = init_file_path() {
        assert!(found.exists());
    }
}

#[test]
fn test_project_root_finds_git() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join(".git")).unwrap();
    fs::create_dir_all(dir.path().join("a/b")).unwrap();
    let sub = dir.path().join("a/b").to_string_lossy().into_owned();
    assert_eq!(project_root(sub), dir.path().to_string_lossy());
}
