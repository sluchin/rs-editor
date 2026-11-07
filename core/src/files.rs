//! ファイルの読み書きの組み込み (Emacs の C の `insert-file-contents`, `write-region`, `do-auto-save` に当たる).
//!
//! ファイルを開くこと (`find-file-noselect`), 保存すること (`basic-save-buffer`), バックアップ (`backup-buffer`) と
//! フックは, Emacs と同じく Lisp (`lisp/files.scm`) で書きます. ここは, 本文とファイルの間で読み書きするだけです.

use crate::error::Error;
use std::io::Write;
use std::path::Path;

use crate::atomic::write_atomic;
use crate::buffer::Buffer;
use crate::editor::Editor;

/// 自動保存のファイルの名前を返します (同じディレクトリの `#名前#`. Lisp の `make-auto-save-file-name` と同じ).
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

/// ファイルの更新時刻 (ファイルが無いときは `None`).
fn modtime(path: &str) -> Option<std::time::SystemTime> {
    std::fs::metadata(path).and_then(|m| m.modified()).ok()
}

/// バッファが訪れているファイルの更新時刻が、訪れたとき（最後に保存したとき）と同じか (Emacs の `verify-visited-file-modtime`).
/// ファイルを訪れていないバッファと、更新時刻を覚えていないバッファは、同じとする.
pub fn verify_visited_file_modtime(buffer: &Buffer) -> bool {
    match (buffer.file(), buffer.visited_modtime()) {
        (Some(path), Some(recorded)) => modtime(path) == Some(recorded),
        _ => true,
    }
}

/// 訪れているファイルの今の更新時刻を覚えます (Emacs の `set-visited-file-modtime`).
pub fn set_visited_file_modtime(buffer: &mut Buffer) {
    let time = buffer.file().and_then(modtime);
    buffer.set_visited_modtime(time);
}

/// バッファがファイルを訪れることにします. 書き換えていないことにし, バックアップを作り直せるようにします.
fn visit(buffer: &mut Buffer, path: &str) {
    if buffer.file() != Some(path) {
        buffer.set_backed_up(false);
    }
    buffer.set_file(Some(path.to_string()));
    buffer.set_visited_modtime(modtime(path));
    buffer.set_modified(false);
    let version = buffer.version();
    buffer.set_auto_saved_version(version);
}

/// ファイルの中身を, カーソルの後ろに入れます (Emacs の `insert-file-contents`). カーソルは, 入れた文字の前に残ります.
///
/// # Arguments
///
/// * `buffer` - バッファ.
/// * `path` - ファイルのパス.
/// * `visit_file` - 真なら, バッファがそのファイルを訪れ, 書き換えていないことにする (Emacs の VISIT).
///
/// # Returns
///
/// 入れた文字の数.
///
/// # Errors
///
/// ファイルを読めないとき, バッファが読み取り専用のとき.
pub fn insert_file_contents(
    buffer: &mut Buffer,
    path: &str,
    visit_file: bool,
) -> Result<usize, Error> {
    let content = std::fs::read_to_string(path).map_err(|e| Error::open_input(&e, path))?;
    if visit_file && buffer.is_empty() {
        // 空のバッファに訪れるときは, 元に戻す履歴に残さずに読み込む (ファイルを開くとき. 大きなファイルでも速い)
        buffer.load(&content);
    } else {
        let point = buffer.point();
        buffer.insert_at(point, &content)?;
        buffer.set_point(point);
    }
    if visit_file {
        visit(buffer, path);
    }
    Ok(content.chars().count())
}

/// 書く本文 (Emacs の `write-region` の START と END).
pub enum Region<'a> {
    /// バッファの全て (START が nil).
    Whole,
    /// バッファの範囲 (文字の位置).
    Range(usize, usize),
    /// 文字列 (START が文字列).
    Text(&'a str),
}

/// 本文をファイルに書きます (Emacs の `write-region`). 足さないときは, 原子的に書きます.
///
/// # Arguments
///
/// * `buffer` - バッファ.
/// * `region` - 書く本文.
/// * `path` - ファイルのパス.
/// * `append` - 真なら, ファイルの後ろに足す.
/// * `visit_file` - 訪れるファイル (Emacs の VISIT が `t` か文字列). バッファはそのファイルを訪れ, 書き換えていないことになる.
///
/// # Errors
///
/// 範囲が本文の外のとき, ファイルを書けないとき.
pub fn write_region(
    buffer: &mut Buffer,
    region: Region,
    path: &str,
    append: bool,
    visit_file: Option<&str>,
) -> Result<(), Error> {
    let text = match region {
        Region::Whole => buffer.text(),
        Region::Range(start, end) => buffer.substring(start.min(end), start.max(end))?,
        Region::Text(text) => text.to_string(),
    };
    let written = if append {
        std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)
            .and_then(|mut file| file.write_all(text.as_bytes()))
    } else {
        write_atomic(Path::new(path), text.as_bytes())
    };
    written.map_err(|e| format!("{path}: {e}"))?;
    if let Some(file) = visit_file {
        visit(buffer, file);
    }
    Ok(())
}

/// 自動保存するまでのキーの数 (Emacs の `auto-save-interval`).
pub const AUTO_SAVE_INTERVAL: usize = 300;

/// 入力が無いまま, 自動保存するまでの時間 (Emacs の `auto-save-timeout`).
pub const AUTO_SAVE_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);

/// 画面から読んだキーを 1 つ数えます (キーボードマクロのキーは数えない. Emacs の `num_nonmacro_input_events` と同じ).
///
/// # Arguments
///
/// * `editor` - エディタ.
///
/// # Returns
///
/// 前に自動保存してから `AUTO_SAVE_INTERVAL` 個のキーを読んだら `true` (呼び出し側が `auto_save` を呼ぶ).
pub fn note_key(editor: &mut Editor) -> bool {
    editor.keys_since_auto_save += 1;
    editor.keys_since_auto_save >= AUTO_SAVE_INTERVAL
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
    editor.keys_since_auto_save = 0;
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
