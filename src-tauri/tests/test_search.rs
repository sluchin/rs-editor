//! プロジェクトの根のテスト.

use std::fs;

use rseditor::commands::project_root;
use tempfile::tempdir;

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
