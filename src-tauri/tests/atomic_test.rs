//! ファイルを原子的に書く ([`write_atomic`]) のテスト.

use std::fs;
use std::os::unix::fs::{symlink, PermissionsExt};
use std::path::Path;

use rseditor::atomic::write_atomic;
use tempfile::tempdir;

/// ディレクトリの中の名前を, 名前の順に返す (一時ファイルが残っていないかを見る).
///
/// # Arguments
///
/// * `dir` - ディレクトリ.
///
/// # Returns
///
/// 名前.
fn names(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

#[test]
fn test_write_atomic_replaces_the_file_and_keeps_the_permissions() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("a.txt");
    // 無ければ, 作る
    write_atomic(&path, b"first").unwrap();
    assert_eq!(fs::read_to_string(&path).unwrap(), "first");
    // あれば, 置き換えて, アクセス権を引き継ぐ
    fs::set_permissions(&path, fs::Permissions::from_mode(0o640)).unwrap();
    write_atomic(&path, b"second").unwrap();
    assert_eq!(fs::read_to_string(&path).unwrap(), "second");
    assert_eq!(
        fs::metadata(&path).unwrap().permissions().mode() & 0o777,
        0o640
    );
    // 一時ファイルは, 残らない
    assert_eq!(names(dir.path()), vec!["a.txt"]);
}

#[test]
fn test_write_atomic_writes_through_symlinks() {
    let dir = tempdir().unwrap();
    let real = dir.path().join("real.txt");
    fs::write(&real, "old").unwrap();
    // 相対のリンクと, リンクを指すリンク
    let link = dir.path().join("link.txt");
    symlink("real.txt", &link).unwrap();
    let chain = dir.path().join("chain.txt");
    symlink(&link, &chain).unwrap();

    write_atomic(&chain, b"new").unwrap();
    assert_eq!(fs::read_to_string(&real).unwrap(), "new");
    assert!(fs::symlink_metadata(&link)
        .unwrap()
        .file_type()
        .is_symlink());
    assert!(fs::symlink_metadata(&chain)
        .unwrap()
        .file_type()
        .is_symlink());

    // 指す先が無いリンクは, 指す先に作る
    let dangling = dir.path().join("dangling.txt");
    symlink("created.txt", &dangling).unwrap();
    write_atomic(&dangling, b"made").unwrap();
    assert_eq!(
        fs::read_to_string(dir.path().join("created.txt")).unwrap(),
        "made"
    );
    assert!(fs::symlink_metadata(&dangling)
        .unwrap()
        .file_type()
        .is_symlink());
}

#[test]
fn test_write_atomic_keeps_the_file_on_failure() {
    let dir = tempdir().unwrap();
    // 置き換えられない (ディレクトリ) ときは, 一時ファイルを消して, 失敗にする
    let target = dir.path().join("sub");
    fs::create_dir(&target).unwrap();
    fs::write(target.join("x.txt"), "keep").unwrap();
    assert!(write_atomic(&target, b"data").is_err());
    assert_eq!(names(dir.path()), vec!["sub"]);
    assert_eq!(fs::read_to_string(target.join("x.txt")).unwrap(), "keep");
    // ディレクトリが無ければ, 失敗にする
    assert!(write_atomic(&dir.path().join("none/a.txt"), b"data").is_err());
}

#[test]
fn test_write_atomic_writes_in_place_when_the_directory_is_read_only() {
    let dir = tempdir().unwrap();
    let locked = dir.path().join("locked");
    fs::create_dir(&locked).unwrap();
    let path = locked.join("a.txt");
    fs::write(&path, "old").unwrap();
    fs::set_permissions(&locked, fs::Permissions::from_mode(0o555)).unwrap();

    // 一時ファイルを作れなくても, ファイルには書ける
    let result = write_atomic(&path, b"new");
    fs::set_permissions(&locked, fs::Permissions::from_mode(0o755)).unwrap();
    result.unwrap();
    assert_eq!(fs::read_to_string(&path).unwrap(), "new");
    assert_eq!(names(&locked), vec!["a.txt"]);
}
