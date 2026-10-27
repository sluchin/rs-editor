//! ファイルを原子的に書く (書いている途中で落ちても, 前の中身か, 新しい中身のどちらかが残る).
//!
//! 同じディレクトリの一時ファイルに書いて, ディスクに書き出してから, 名前を変えて置き換えます.

use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

/// シンボリックリンクを, たどる数の上限 (輪になったリンクで, 終わるように. Linux の `ELOOP` と同じ).
const MAX_LINKS: usize = 40;

/// 一時ファイルの名前に付ける, 書くたびに増やす番号 (同じプロセスで, 同時に書いても重ならないように).
static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

/// ファイルを原子的に書きます. 同じディレクトリの一時ファイルに書いて, ディスクに書き出してから, 名前を変えて置き換えます.
///
/// - ファイルがあれば, そのアクセス権を引き継ぎます.
/// - シンボリックリンクは, 指す先のファイルに書きます (リンクは, そのまま残す).
/// - ディレクトリに一時ファイルを作れない (ファイルにだけ書ける) ときは, ファイルにそのまま書きます (原子的でない).
///
/// # Arguments
///
/// * `path` - ファイルのパス.
/// * `content` - 中身.
///
/// # Returns
///
/// 書けなければ [`Err`]. 失敗したときは, 一時ファイルを消し, もとのファイルは変えません.
pub fn write_atomic(path: &Path, content: &[u8]) -> io::Result<()> {
    let target = resolve_symlinks(path);
    let temp = temp_path(&target);
    let mut file = match OpenOptions::new().write(true).create_new(true).open(&temp) {
        Ok(file) => file,
        Err(_) => return std::fs::write(&target, content),
    };
    let result = (|| {
        file.write_all(content)?;
        if let Ok(metadata) = std::fs::metadata(&target) {
            file.set_permissions(metadata.permissions())?;
        }
        file.sync_all()?;
        std::fs::rename(&temp, &target)
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&temp);
        return result;
    }
    // 名前の変更も, ディスクに書き出す (できなくても, 書いたことにする)
    if let Some(dir) = target.parent() {
        let _ = File::open(dir).and_then(|dir| dir.sync_all());
    }
    Ok(())
}

/// シンボリックリンクを, リンクでないパスまでたどります (指す先が無くても, そのパスを返す).
///
/// # Arguments
///
/// * `path` - パス.
///
/// # Returns
///
/// たどった先のパス.
fn resolve_symlinks(path: &Path) -> PathBuf {
    let mut path = path.to_path_buf();
    for _ in 0..MAX_LINKS {
        let Ok(link) = std::fs::read_link(&path) else {
            break;
        };
        // 相対のリンクは, リンクのあるディレクトリから数える (絶対のリンクは, そのまま)
        path = path.with_file_name(link);
    }
    path
}

/// 一時ファイルのパスを返します (同じディレクトリの `.#<名前>.<プロセス>-<番号>.tmp`).
///
/// # Arguments
///
/// * `target` - 書くファイルのパス.
///
/// # Returns
///
/// 一時ファイルのパス.
fn temp_path(target: &Path) -> PathBuf {
    let name = target.file_name().unwrap_or_default().to_string_lossy();
    let count = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    target.with_file_name(format!(".#{}.{}-{}.tmp", name, std::process::id(), count))
}
