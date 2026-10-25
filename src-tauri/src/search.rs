//! ディレクトリの中を探す (エディタの `M-x grep`. 外のコマンドは使わない) と, プロジェクトの根 (サイドバーのツリー) を探す.

use std::path::{Path, PathBuf};

use regex::Regex;
use serde::{Deserialize, Serialize};

/// 探さないディレクトリ (`.` で始まるディレクトリ (`.git` など) も, 探さない).
pub const IGNORED_DIRECTORIES: &[&str] = &["node_modules", "target"];

/// 合った行.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GrepMatch {
    /// ファイル (探したディレクトリからのパス).
    pub file: String,
    /// 行の番号 (1 から).
    pub line: usize,
    /// 行.
    pub text: String,
}

/// 正規表現に, 大文字があるかを返します (`\W` のような, `\` の後の文字は数えない. Emacs の `isearch-no-upper-case-p`).
///
/// # Arguments
///
/// * `pattern` - 正規表現.
///
/// # Returns
///
/// 大文字があれば `true`.
pub fn has_upper_case(pattern: &str) -> bool {
    let mut escaped = false;
    for c in pattern.chars() {
        if escaped {
            escaped = false;
        } else if c == '\\' {
            escaped = true;
        } else if c.is_uppercase() {
            return true;
        }
    }
    false
}

/// ディレクトリの下 (深くまで) のファイルから, 正規表現に合う行を探します.
/// 正規表現に大文字が無ければ, 大文字と小文字を区別しません (Emacs の `case-fold-search`).
/// 読めないファイルと, 文字のファイルでないもの (NUL を含むか, UTF-8 でない) は, 飛ばします.
///
/// # Arguments
///
/// * `pattern` - 正規表現 (Rust の regex の書き方).
/// * `dir` - ディレクトリ.
///
/// # Returns
///
/// 合った行 (ファイルの名前の順, 行の順). 正規表現が誤っているか, ディレクトリを読めなければ, 理由の文字列の [`Err`].
pub fn grep_directory(pattern: &str, dir: &Path) -> Result<Vec<GrepMatch>, String> {
    let source = if has_upper_case(pattern) {
        pattern.to_string()
    } else {
        format!("(?i){}", pattern)
    };
    let regex = Regex::new(&source).map_err(|e| format!("Invalid regexp: {}", e))?;
    std::fs::read_dir(dir).map_err(|e| format!("{}: {}", dir.display(), e))?;
    let mut files = Vec::new();
    collect_files(dir, &mut files);
    let mut found = Vec::new();
    for path in files {
        let Ok(bytes) = std::fs::read(&path) else {
            continue;
        };
        if bytes.contains(&0) {
            continue;
        }
        let Ok(text) = String::from_utf8(bytes) else {
            continue;
        };
        // 探したディレクトリの下のファイルなので, 必ず外せる
        let file = path
            .strip_prefix(dir)
            .unwrap_or(&path)
            .to_string_lossy()
            .into_owned();
        for (i, line) in text.lines().enumerate() {
            if regex.is_match(line) {
                found.push(GrepMatch {
                    file: file.clone(),
                    line: i + 1,
                    text: line.to_string(),
                });
            }
        }
    }
    Ok(found)
}

/// ディレクトリの下 (深くまで) のファイルを, 名前の順に集めます. 探さないディレクトリと, ディレクトリへの
/// シンボリックリンク (輪になっていると終わらない. `grep -r` と同じく, たどらない) は, 除きます.
///
/// # Arguments
///
/// * `dir` - ディレクトリ (読めなければ, 何もしない).
/// * `files` - 集めたファイルを, 入れる先.
fn collect_files(dir: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut paths: Vec<PathBuf> = entries.flatten().map(|e| e.path()).collect();
    paths.sort();
    for path in paths {
        if path.is_dir() {
            let name = path.file_name().unwrap_or_default().to_string_lossy();
            if !path.is_symlink()
                && !name.starts_with('.')
                && !IGNORED_DIRECTORIES.contains(&name.as_ref())
            {
                collect_files(&path, files);
            }
        } else {
            files.push(path);
        }
    }
}

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
