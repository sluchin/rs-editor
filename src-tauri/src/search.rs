//! プロジェクトの根 (サイドバーのツリー) を探す.

use std::path::{Path, PathBuf};

/// プロジェクトの根を探します. ディレクトリから上へたどって, `.git` のある所です (Emacs の project.el の VC のプロジェクト).
///
/// # Arguments
///
/// * `dir` - ディレクトリ.
///
/// # Returns
///
/// `.git` のあるディレクトリ. 無ければ, `dir` のまま.
pub fn project_root(dir: &Path) -> PathBuf {
    dir.ancestors()
        .find(|d| d.join(".git").exists())
        .unwrap_or(dir)
        .to_path_buf()
}
