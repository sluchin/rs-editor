//! 画面 (フロントエンド) から呼ぶコマンド. Tauri に依存する部分は薄くして, 本体は普通の関数にします.
//! エディタの入力は `front::session_input` で受けます. ここは, 画面だけで使うもの (ファイル名の補完, サイドバーのツリー) と,
//! 画面に頼んだ操作の返事です.

use std::path::{Path, PathBuf};

use rseditor_core::startup::Options;
#[cfg(not(tarpaulin_include))]
use std::sync::Arc;

#[cfg(not(tarpaulin_include))]
use tauri::State;

#[cfg(not(tarpaulin_include))]
use crate::scheme::host::Bridge;

/// 画面へ, エディタの操作を頼むイベントの名前.
pub const EDITOR_REQUEST_EVENT: &str = "editor-request";

/// 初期設定ファイルの候補を, 探す順に返します.
///
/// # Arguments
///
/// * `home` - ホームディレクトリ (分からなければ `None`).
/// * `config_dir` - 設定のディレクトリ (Linux は `~/.config`. 分からなければ `None`).
///
/// # Returns
///
/// `<home>/.rseditor`, `<home>/.rseditor.d/init.scm` (Emacs の `~/.emacs.d/init.el` と同じ) と,
/// `<config_dir>/rseditor/init.scm`.
pub fn init_file_candidates(home: Option<&Path>, config_dir: Option<&Path>) -> Vec<PathBuf> {
    home.into_iter()
        .flat_map(|dir| {
            [
                dir.join(".rseditor"),
                dir.join(".rseditor.d").join("init.scm"),
            ]
        })
        .chain(config_dir.map(|dir| dir.join("rseditor").join("init.scm")))
        .collect()
}

/// 初期設定ファイルのパスを返します. `~/.rseditor`, `~/.rseditor.d/init.scm`,
/// `~/.config/rseditor/init.scm` の順に探して, 初めにあるものを使います.
///
/// # Returns
///
/// あるファイルのパス. どちらも無ければ `None`.
pub fn init_file_path() -> Option<PathBuf> {
    init_file_candidates(dirs::home_dir().as_deref(), dirs::config_dir().as_deref())
        .into_iter()
        .find(|path| path.exists())
}

/// 起動の引数に従って, 読む初期設定ファイルを決めます.
///
/// # Arguments
///
/// * `options` - 起動の引数.
/// * `default` - 引数が無いときに読むファイル (`init_file_path`).
///
/// # Returns
///
/// `-q` と `-Q` では `None`. `--init-directory` があれば, そのディレクトリの `init.scm` (無ければ `None`).
/// どちらも無ければ `default`.
pub fn choose_init_file(options: &Options, default: Option<PathBuf>) -> Option<PathBuf> {
    if options.no_init_file {
        return None;
    }
    match &options.init_directory {
        Some(dir) => Some(Path::new(dir).join("init.scm")).filter(|path| path.exists()),
        None => default,
    }
}

/// 起動の引数を返します (初めに呼んだときに, 環境の引数から読む. 読めなければ, 引数は無いものとする).
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

/// プロジェクトの根 (`.git` のある, 上のディレクトリ. 無ければ, そのディレクトリ) を返します.
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
