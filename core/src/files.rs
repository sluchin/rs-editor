//! バッファをファイルに書くこと (Emacs の files.el のうち, バックアップと自動保存).
//!
//! - 訪れてから初めて保存するとき, 前のファイルをバックアップ (`名前~`) に残します.
//! - 書き換えたファイルのバッファは, 時々, 自動保存のファイル (同じディレクトリの `#名前#`) に書きます.
//!   保存したら, 自動保存のファイルを消します.

use std::path::Path;

use crate::atomic::write_atomic;
use crate::editor::Editor;

/// 自動保存のファイルの名前を返します (同じディレクトリの `#名前#`. Emacs の `make-auto-save-file-name`).
///
/// # Arguments
///
/// * `path` - ファイルのパス.
pub fn auto_save_file_name(path: &str) -> String {
    let (dir, base) = match path.rfind('/') {
        Some(i) => path.split_at(i + 1),
        None => ("", path),
    };
    format!("{dir}#{base}#")
}

/// バックアップのファイルの名前を返します (`名前~`. Emacs の `make-backup-file-name`).
///
/// # Arguments
///
/// * `path` - ファイルのパス.
pub fn backup_file_name(path: &str) -> String {
    format!("{path}~")
}

/// バッファをファイルに書きます (Lisp の `files--write-buffer`). バッファはそのファイルを訪れ, 書き換えていないことになります.
///
/// 訪れてから初めて書くとき, ファイルがあれば, バックアップに写します. 書いたら, 自動保存のファイルを消します.
///
/// # Arguments
///
/// * `editor` - エディタ.
/// * `name` - バッファの名前.
/// * `path` - ファイルのパス.
///
/// # Errors
///
/// バッファが無いとき, バックアップかファイルを書けないとき.
pub fn write_buffer(editor: &mut Editor, name: &str, path: &str) -> Result<(), String> {
    let buffer = editor
        .get_buffer_mut(name)
        .ok_or_else(|| format!("No such buffer: {name}"))?;
    // 別のファイルに書く (write-file) ときは, 訪れ直したことにする
    if buffer.file() != Some(path) {
        buffer.set_backed_up(false);
    }
    if !buffer.backed_up() && Path::new(path).is_file() {
        let backup = backup_file_name(path);
        std::fs::copy(path, &backup).map_err(|e| format!("{backup}: {e}"))?;
    }
    write_atomic(Path::new(path), buffer.text().as_bytes()).map_err(|e| format!("{path}: {e}"))?;
    buffer.set_backed_up(true);
    buffer.set_file(Some(path.to_string()));
    buffer.set_modified(false);
    let version = buffer.version();
    buffer.set_auto_saved_version(version);
    let _ = std::fs::remove_file(auto_save_file_name(path));
    Ok(())
}

/// 前に自動保存してから書き換えた, ファイルのバッファを, 自動保存のファイルに書きます (Emacs の `do-auto-save`).
///
/// # Arguments
///
/// * `editor` - エディタ.
///
/// # Returns
///
/// 書いたバッファの数 (書けなかったものは, 数えない. 次の機会に, また書く).
pub fn auto_save(editor: &mut Editor) -> usize {
    let mut saved = 0;
    for name in editor.buffer_list() {
        let buffer = editor.get_buffer_mut(&name).unwrap();
        let Some(path) = buffer.file() else { continue };
        if !buffer.modified() || buffer.version() == buffer.auto_saved_version() {
            continue;
        }
        if std::fs::write(auto_save_file_name(path), buffer.text()).is_ok() {
            let version = buffer.version();
            buffer.set_auto_saved_version(version);
            saved += 1;
        }
    }
    saved
}
