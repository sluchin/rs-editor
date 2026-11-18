//! 画面 (フロントエンド) から呼ぶコマンド. Tauri に依存する部分は薄くして, 本体は普通の関数にします.
//! エディタの入力は `front::session_input` で受けます. ここは, 画面だけで使うもの (ファイル名の補完, サイドバーのツリー) と,
//! 画面に頼んだ操作の返事です.

use std::path::Path;

use rseditor_core::startup::Options;
#[cfg(not(tarpaulin_include))]
use std::sync::Arc;

#[cfg(not(tarpaulin_include))]
use tauri::State;

#[cfg(not(tarpaulin_include))]
use crate::scheme::host::Bridge;

/// 画面へ, エディタの操作を頼むイベントの名前.
pub const EDITOR_REQUEST_EVENT: &str = "editor-request";

pub use rseditor_core::startup::{choose_init_file, init_file_candidates, init_file_path};

/// 起動の引数を返します (初めに呼んだときに, 環境の引数から読む. 読めなければ引数は無いものとする).
///
/// # Returns
///
/// 起動の引数.
// 実際のプロセスの引数を読むため, カバレッジの対象外にする (読み方は `rseditor_core::startup::parse` で検証する).
#[cfg(not(tarpaulin_include))]
pub fn startup_options() -> &'static Options {
    static OPTIONS: std::sync::OnceLock<Options> = std::sync::OnceLock::new();
    OPTIONS.get_or_init(|| {
        let args: Vec<String> = std::env::args().skip(1).collect();
        rseditor_core::startup::parse(&args).unwrap_or_else(|error| {
            log::warn!("Could not read the arguments: {error}");
            Options::default()
        })
    })
}

/// 画面からの, エディタの操作の返事を受け取ります.
///
/// # Arguments
///
/// * `id` - 頼みの番号.
/// * `value` - 結果.
/// * `error` - 失敗の内容 (成功したときは `None`).
/// * `bridge` - 画面に頼む仕組み.
#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub fn editor_reply(
    id: u64,
    value: serde_json::Value,
    error: Option<String>,
    bridge: State<'_, Arc<Bridge>>,
) {
    if !bridge.reply(id, value, error) {
        log::warn!("No request is waiting for the reply {}", id);
    }
}

/// ディレクトリの中の名前を, 名前の順に返します (ファイル名の補完). ディレクトリには, 後ろに `/` を付けます.
///
/// # Arguments
///
/// * `path` - ディレクトリのパス.
///
/// # Returns
///
/// 名前. 読めなければ, 理由の文字列の [`Err`].
#[tauri::command]
pub fn list_directory(path: String) -> Result<Vec<String>, String> {
    let entries = std::fs::read_dir(&path).map_err(|e| e.to_string())?;
    let mut names: Vec<String> = entries
        .flatten()
        .map(|entry| {
            let name = entry.file_name().to_string_lossy().into_owned();
            // シンボリックリンクは, 指す先で決める
            if entry.path().is_dir() {
                name + "/"
            } else {
                name
            }
        })
        .collect();
    names.sort();
    Ok(names)
}

/// ファイルかディレクトリを作ります (サイドバーのツリーから). 同じ名前があれば失敗します.
///
/// # Arguments
///
/// * `path` - 作るパス (上のディレクトリは, あること).
/// * `directory` - ディレクトリを作るなら真, 空のファイルを作るなら偽.
///
/// # Returns
///
/// 作れなければ, 理由の文字列の [`Err`].
#[tauri::command]
pub fn create_path(path: String, directory: bool) -> Result<(), String> {
    if directory {
        std::fs::create_dir(&path)
    } else {
        std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map(|_| ())
    }
    .map_err(|e| format!("{path}: {e}"))
}

/// ファイルかディレクトリの名前を変えます (サイドバーのツリーから). 変え先に同じ名前があれば失敗します (上書きしない).
///
/// # Arguments
///
/// * `from` - 今のパス.
/// * `to` - 変えたあとのパス.
///
/// # Returns
///
/// 変えられなければ, 理由の文字列の [`Err`].
#[tauri::command]
pub fn rename_path(from: String, to: String) -> Result<(), String> {
    if std::fs::symlink_metadata(&to).is_ok() {
        return Err(format!("{to}: File exists"));
    }
    std::fs::rename(&from, &to).map_err(|e| format!("{from}: {e}"))
}

/// ファイルか, ディレクトリ (中身ごと) を消します (サイドバーのツリーから. 確かめるのは画面の側).
///
/// # Arguments
///
/// * `path` - 消すパス. シンボリックリンクは, リンクだけを消す.
///
/// # Returns
///
/// 消せなければ, 理由の文字列の [`Err`].
#[tauri::command]
pub fn delete_path(path: String) -> Result<(), String> {
    let result = match std::fs::symlink_metadata(&path) {
        Ok(meta) if meta.is_dir() => std::fs::remove_dir_all(&path),
        Ok(_) => std::fs::remove_file(&path),
        Err(e) => Err(e),
    };
    result.map_err(|e| format!("{path}: {e}"))
}

/// ホームディレクトリを返します (Unix は環境変数 `HOME`, 無ければ passwd. Windows はユーザーのプロファイル).
///
/// # Returns
///
/// ホームディレクトリ. 分からなければ, 理由の文字列の [`Err`].
#[tauri::command]
pub fn home_dir() -> Result<String, String> {
    dirs::home_dir()
        .map(|home| home.to_string_lossy().into_owned())
        .ok_or_else(|| "No home directory".to_string())
}

/// プロジェクトの根 (`.git` のある, 上のディレクトリ. 無ければそのディレクトリ) を返します.
///
/// # Arguments
///
/// * `directory` - ディレクトリ.
///
/// # Returns
///
/// プロジェクトの根.
#[tauri::command]
pub fn project_root(directory: String) -> String {
    crate::search::project_root(Path::new(&directory))
        .to_string_lossy()
        .into_owned()
}
