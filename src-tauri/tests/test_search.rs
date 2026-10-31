//! ディレクトリの中の検索 (`M-x grep`) と, プロジェクトの根のテスト.

use std::fs;
use std::os::unix::fs::PermissionsExt;

use rseditor::commands::project_root;
use rseditor::search::{
    grep_directory, has_upper_case, GrepMatch, MAX_GREP_FILE_SIZE, MAX_GREP_MATCHES,
};
use tempfile::tempdir;

/// 合った行を作る.
///
/// # Arguments
///
/// * `file` - ファイル.
/// * `line` - 行の番号.
/// * `text` - 行.
///
/// # Returns
///
/// 合った行.
fn found(file: &str, line: usize, text: &str) -> GrepMatch {
    GrepMatch {
        file: file.to_string(),
        line,
        text: text.to_string(),
    }
}

#[test]
fn test_has_upper_case() {
    assert!(has_upper_case("Foo"));
    assert!(!has_upper_case("foo"));
    assert!(!has_upper_case("\\Wfoo\\S"));
    assert!(has_upper_case("\\\\F"));
}

#[test]
fn test_grep_directory() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::create_dir_all(root.join("src/lib")).unwrap();
    fs::write(root.join("b.txt"), "Foo\nbar\nfoo bar\n").unwrap();
    fs::write(root.join("src/lib/a.ts"), "const foo = 1\n").unwrap();
    // 探さないもの: . で始まるディレクトリ, node_modules, target, NUL を含むファイル, UTF-8 でないファイル
    for ignored in [".git", "node_modules", "target"] {
        fs::create_dir_all(root.join(ignored)).unwrap();
        fs::write(root.join(ignored).join("x.txt"), "foo\n").unwrap();
    }
    fs::write(root.join("bin.dat"), b"foo\0").unwrap();
    fs::write(root.join("latin1.txt"), b"foo \xff\n").unwrap();
    // . で始まるファイルは探す
    fs::write(root.join(".env"), "FOO=foo\n").unwrap();
    // ディレクトリへのシンボリックリンクは, たどらない (輪になっていても, 終わる)
    std::os::unix::fs::symlink(root, root.join("src/loop")).unwrap();

    // 小文字だけなら, 大文字と小文字を区別しない
    assert_eq!(
        grep_directory("foo", root).unwrap(),
        vec![
            found(".env", 1, "FOO=foo"),
            found("b.txt", 1, "Foo"),
            found("b.txt", 3, "foo bar"),
            found("src/lib/a.ts", 1, "const foo = 1"),
        ]
    );
    assert_eq!(
        grep_directory("^Foo$", root).unwrap(),
        vec![found("b.txt", 1, "Foo")]
    );
    assert!(grep_directory("zzz", root).unwrap().is_empty());
    assert!(grep_directory("(", root)
        .unwrap_err()
        .starts_with("Invalid regexp: "));
    let missing = root.join("missing");
    assert!(grep_directory("a", &missing)
        .unwrap_err()
        .starts_with(&format!("{}: ", missing.display())));
}

#[test]
fn test_grep_skips_unreadable() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("a.txt"), "foo\n").unwrap();
    fs::write(root.join("secret.txt"), "foo\n").unwrap();
    fs::create_dir_all(root.join("closed")).unwrap();
    fs::write(root.join("closed/c.txt"), "foo\n").unwrap();
    fs::set_permissions(root.join("secret.txt"), fs::Permissions::from_mode(0o000)).unwrap();
    fs::set_permissions(root.join("closed"), fs::Permissions::from_mode(0o000)).unwrap();
    let result = grep_directory("foo", root);
    fs::set_permissions(root.join("closed"), fs::Permissions::from_mode(0o755)).unwrap();
    // root で動かすと, 読めてしまう
    if fs::read(root.join("secret.txt")).is_err() {
        assert_eq!(result.unwrap(), vec![found("a.txt", 1, "foo")]);
    }
}

#[test]
fn test_grep_skips_large_files() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("a.txt"), "foo\n").unwrap();
    let mut large = "foo\n".repeat(MAX_GREP_FILE_SIZE as usize / 4);
    large.push('x');
    fs::write(root.join("large.txt"), large).unwrap();
    assert_eq!(
        grep_directory("foo", root).unwrap(),
        vec![found("a.txt", 1, "foo")]
    );
}

#[test]
fn test_grep_stops_at_the_limit() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("a.txt"), "foo\n".repeat(MAX_GREP_MATCHES + 1)).unwrap();
    fs::write(root.join("b.txt"), "foo\n").unwrap();
    let result = grep_directory("foo", root).unwrap();
    assert_eq!(result.len(), MAX_GREP_MATCHES);
    assert_eq!(
        result.last().unwrap(),
        &found("a.txt", MAX_GREP_MATCHES, "foo")
    );
}

#[test]
fn test_project_root() {
    let dir = tempdir().unwrap();
    let root = dir.path().join("proj");
    let deep = root.join("src/lib");
    fs::create_dir_all(&deep).unwrap();
    // .git が無ければ, そのまま
    assert_eq!(
        project_root(deep.to_string_lossy().into_owned()),
        deep.to_string_lossy()
    );
    fs::create_dir_all(root.join(".git")).unwrap();
    assert_eq!(
        project_root(deep.to_string_lossy().into_owned()),
        root.to_string_lossy()
    );
}
