//! バックアップと自動保存 (`core/src/files.rs`) のテスト.

use rseditor_core::editor::Editor;
use rseditor_core::files::{auto_save, auto_save_file_name, backup_file_name, write_buffer};

#[test]
fn test_file_names() {
    assert_eq!(auto_save_file_name("/tmp/a.txt"), "/tmp/#a.txt#");
    assert_eq!(auto_save_file_name("a.txt"), "#a.txt#");
    assert_eq!(backup_file_name("/tmp/a.txt"), "/tmp/a.txt~");
}

#[test]
fn test_write_buffer_keeps_a_backup_once_and_removes_the_auto_save_file() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("a.txt");
    let path = file.to_str().unwrap();
    std::fs::write(&file, "old\n").unwrap();
    let mut editor = Editor::new();
    let buffer = editor.get_buffer_create("a.txt");
    buffer.set_file(Some(path.to_string()));
    buffer.insert("new\n");
    // 自動保存してから保存すると, 自動保存のファイルを消す
    assert_eq!(auto_save(&mut editor), 1);
    assert_eq!(
        std::fs::read_to_string(auto_save_file_name(path)).unwrap(),
        "new\n"
    );
    // 変えていなければ, 自動保存しない
    assert_eq!(auto_save(&mut editor), 0);
    write_buffer(&mut editor, "a.txt", path).unwrap();
    assert_eq!(std::fs::read_to_string(&file).unwrap(), "new\n");
    assert_eq!(
        std::fs::read_to_string(backup_file_name(path)).unwrap(),
        "old\n"
    );
    assert!(!std::path::Path::new(&auto_save_file_name(path)).exists());
    assert!(!editor.get_buffer("a.txt").unwrap().modified());
    // 2 度目の保存は, バックアップを作り直さない
    editor.get_buffer_mut("a.txt").unwrap().insert("more\n");
    write_buffer(&mut editor, "a.txt", path).unwrap();
    assert_eq!(
        std::fs::read_to_string(backup_file_name(path)).unwrap(),
        "old\n"
    );
    // 別のファイルに書くと, そのファイルのバックアップを作る
    let other = dir.path().join("b.txt");
    let other_path = other.to_str().unwrap();
    std::fs::write(&other, "b\n").unwrap();
    write_buffer(&mut editor, "a.txt", other_path).unwrap();
    assert_eq!(
        std::fs::read_to_string(backup_file_name(other_path)).unwrap(),
        "b\n"
    );
    // 保存した後は, 書き換えていないので, 自動保存しない
    assert_eq!(auto_save(&mut editor), 0);
}

#[test]
fn test_write_buffer_errors() {
    let dir = tempfile::tempdir().unwrap();
    let mut editor = Editor::new();
    assert_eq!(
        write_buffer(&mut editor, "none", "/tmp/none"),
        Err("No such buffer: none".to_string())
    );
    // バックアップを書けない (同じ名前のディレクトリがある)
    let file = dir.path().join("a.txt");
    let path = file.to_str().unwrap();
    std::fs::write(&file, "old\n").unwrap();
    std::fs::create_dir(backup_file_name(path)).unwrap();
    editor.get_buffer_create("a.txt");
    assert!(write_buffer(&mut editor, "a.txt", path)
        .unwrap_err()
        .starts_with(&backup_file_name(path)));
    // ファイルを書けない (ディレクトリが無い)
    let missing = dir.path().join("none").join("a.txt");
    let missing = missing.to_str().unwrap();
    assert!(write_buffer(&mut editor, "a.txt", missing)
        .unwrap_err()
        .starts_with(missing));
    // 自動保存のファイルを書けないものは, 数えない
    let buffer = editor.get_buffer_mut("a.txt").unwrap();
    buffer.set_file(Some(missing.to_string()));
    buffer.insert("x");
    assert_eq!(auto_save(&mut editor), 0);
}
