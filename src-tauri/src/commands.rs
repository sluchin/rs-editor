//! 画面 (フロントエンド) から呼ぶコマンド. Tauri に依存する部分は薄くして, 本体は普通の関数にします.

use std::path::{Path, PathBuf};
#[cfg(not(tarpaulin_include))]
use std::sync::Arc;

use serde::{Deserialize, Serialize};
#[cfg(not(tarpaulin_include))]
use tauri::{AppHandle, Emitter, State};

use crate::scheme::evaluator::{Evaluator, KeyBinding};
#[cfg(not(tarpaulin_include))]
use crate::scheme::host::{Bridge, BridgeHost, EditorRequest};
use crate::scheme::host::{Host, NoHost};

/// Scheme の評価の結果.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct EvalResponse {
    /// 結果 (失敗したときは, 空).
    pub result: String,
    /// 失敗の内容 (成功したときは, `None`).
    pub error: Option<String>,
    /// Lisp で作ったコマンドの名前 (`define-command`).
    pub commands: Vec<String>,
    /// Lisp で割り当てたキー (`global-set-key`).
    pub keys: Vec<KeyBinding>,
}

/// 評価の結果に, 評価器が覚えているコマンドとキーを添えます.
///
/// # Arguments
///
/// * `evaluator` - 評価器.
/// * `outcome` - 評価の結果, または失敗の内容.
///
/// # Returns
///
/// 画面へ返す評価の結果.
fn respond(evaluator: &Evaluator, outcome: Result<String, String>) -> EvalResponse {
    let (result, error) = match outcome {
        Ok(result) => (result, None),
        Err(e) => (String::new(), Some(e)),
    };
    EvalResponse {
        result,
        error,
        commands: evaluator.commands(),
        keys: evaluator.key_bindings(),
    }
}

/// Scheme のコードを評価します (エディタの操作は, 失敗します).
///
/// # Arguments
///
/// * `evaluator` - 評価器.
/// * `code` - コード.
///
/// # Returns
///
/// 結果, または失敗の内容.
pub fn eval_code(evaluator: &Evaluator, code: &str) -> EvalResponse {
    eval_code_with(evaluator, code, &NoHost)
}

/// Scheme のコードを評価します.
///
/// # Arguments
///
/// * `evaluator` - 評価器.
/// * `code` - コード.
/// * `host` - エディタ.
///
/// # Returns
///
/// 結果, または失敗の内容.
pub fn eval_code_with(evaluator: &Evaluator, code: &str, host: &dyn Host) -> EvalResponse {
    respond(evaluator, evaluator.eval_with(code, host))
}

/// Lisp で作ったコマンドを実行します.
///
/// # Arguments
///
/// * `evaluator` - 評価器.
/// * `name` - コマンドの名前.
/// * `host` - エディタ.
///
/// # Returns
///
/// 結果, または失敗の内容.
pub fn run_command(evaluator: &Evaluator, name: &str, host: &dyn Host) -> EvalResponse {
    respond(evaluator, evaluator.run_command(name, host))
}

/// 設定のディレクトリから, 初期設定ファイルのパスを作ります.
///
/// # Arguments
///
/// * `config_dir` - 設定のディレクトリ (Linux は `~/.config`).
///
/// # Returns
///
/// `<config_dir>/rseditor/init.scm`.
pub fn init_file_path_from(config_dir: &Path) -> PathBuf {
    config_dir.join("rseditor").join("init.scm")
}

/// 初期設定ファイルのパスを返します.
///
/// # Returns
///
/// パス. 設定のディレクトリが分からなければ `None`.
pub fn init_file_path() -> Option<PathBuf> {
    dirs::config_dir().map(|dir| init_file_path_from(&dir))
}

/// 設定のディレクトリから, ブックマークのファイルのパスを作ります. ディレクトリが無ければ, 作ります.
///
/// # Arguments
///
/// * `config_dir` - 設定のディレクトリ (Linux は `~/.config`. 分からなければ `None`).
///
/// # Returns
///
/// `<config_dir>/rseditor/bookmarks`. 設定のディレクトリが分からないか, 作れなければ, 理由の文字列の [`Err`].
pub fn bookmark_file_path_from(config_dir: Option<&Path>) -> Result<String, String> {
    let dir = config_dir
        .ok_or_else(|| "No config directory".to_string())?
        .join("rseditor");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir.join("bookmarks").to_string_lossy().into_owned())
}

/// ブックマークのファイルのパス (`~/.config/rseditor/bookmarks`) を返します. ディレクトリが無ければ, 作ります.
///
/// # Returns
///
/// パス. 設定のディレクトリが分からないか, 作れなければ, 理由の文字列の [`Err`].
#[tauri::command]
pub fn bookmark_file_path() -> Result<String, String> {
    bookmark_file_path_from(dirs::config_dir().as_deref())
}

/// 初期設定ファイルを読んで, 評価します. ファイルがなければ, 何もしません.
///
/// # Arguments
///
/// * `evaluator` - 評価器.
/// * `path` - ファイルのパス (`None` なら, 何もしない).
/// * `host` - エディタ.
///
/// # Returns
///
/// 結果. 読めないときと, 評価が失敗したときは, ファイルのパスを添えた失敗の内容.
pub fn load_init_file(evaluator: &Evaluator, path: Option<&Path>, host: &dyn Host) -> EvalResponse {
    let Some(path) = path.filter(|p| p.exists()) else {
        return respond(evaluator, Ok(String::new()));
    };
    log::info!("Loading {}", path.display());
    let outcome = std::fs::read_to_string(path)
        .map_err(|e| e.to_string())
        .and_then(|code| evaluator.eval_with(&code, host))
        .map_err(|e| {
            log::warn!("Could not load {}: {}", path.display(), e);
            format!("{}: {}", path.display(), e)
        });
    respond(evaluator, outcome)
}

/// 画面へ, エディタの操作を頼むイベントの名前.
pub const EDITOR_REQUEST_EVENT: &str = "editor-request";

/// 画面に頼むエディタ (Tauri のイベントで頼み, [`editor_reply`] で返事を受け取る).
///
/// # Arguments
///
/// * `app` - アプリ (イベントを送る).
/// * `bridge` - 返事を待つ仕組み.
///
/// # Returns
///
/// エディタ.
#[cfg(not(tarpaulin_include))]
fn app_host<'a>(
    app: &'a AppHandle,
    bridge: &'a Bridge,
) -> BridgeHost<'a, impl Fn(EditorRequest) -> Result<(), String> + Sync + 'a> {
    BridgeHost::new(bridge, move |request| {
        app.emit(EDITOR_REQUEST_EVENT, request)
            .map_err(|e| e.to_string())
    })
}

/// Scheme のコードを評価します (アプリの評価器を使う). 評価は, 画面の返事を待つので, 別のスレッドで行います.
///
/// # Arguments
///
/// * `code` - コード.
/// * `app` - アプリ.
/// * `evaluator` - アプリの評価器.
/// * `bridge` - 画面に頼む仕組み.
///
/// # Returns
///
/// 結果, または失敗の内容.
#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn eval_scheme(
    code: String,
    app: AppHandle,
    evaluator: State<'_, Arc<Evaluator>>,
    bridge: State<'_, Arc<Bridge>>,
) -> Result<EvalResponse, String> {
    let (evaluator, bridge) = (Arc::clone(&evaluator), Arc::clone(&bridge));
    tauri::async_runtime::spawn_blocking(move || {
        eval_code_with(&evaluator, &code, &app_host(&app, &bridge))
    })
    .await
    .map_err(|e| e.to_string())
}

/// Lisp で作ったコマンドを実行します (アプリの評価器を使う).
///
/// # Arguments
///
/// * `name` - コマンドの名前.
/// * `app` - アプリ.
/// * `evaluator` - アプリの評価器.
/// * `bridge` - 画面に頼む仕組み.
///
/// # Returns
///
/// 結果, または失敗の内容.
#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn run_lisp_command(
    name: String,
    app: AppHandle,
    evaluator: State<'_, Arc<Evaluator>>,
    bridge: State<'_, Arc<Bridge>>,
) -> Result<EvalResponse, String> {
    let (evaluator, bridge) = (Arc::clone(&evaluator), Arc::clone(&bridge));
    tauri::async_runtime::spawn_blocking(move || {
        run_command(&evaluator, &name, &app_host(&app, &bridge))
    })
    .await
    .map_err(|e| e.to_string())
}

/// 初期設定ファイル (`~/.config/rseditor/init.scm`) を読んで, 評価します (アプリの評価器を使う).
///
/// # Arguments
///
/// * `app` - アプリ.
/// * `evaluator` - アプリの評価器.
/// * `bridge` - 画面に頼む仕組み.
///
/// # Returns
///
/// 結果, または失敗の内容.
#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn load_init(
    app: AppHandle,
    evaluator: State<'_, Arc<Evaluator>>,
    bridge: State<'_, Arc<Bridge>>,
) -> Result<EvalResponse, String> {
    let (evaluator, bridge) = (Arc::clone(&evaluator), Arc::clone(&bridge));
    tauri::async_runtime::spawn_blocking(move || {
        let path = init_file_path();
        load_init_file(&evaluator, path.as_deref(), &app_host(&app, &bridge))
    })
    .await
    .map_err(|e| e.to_string())
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

/// ファイルを読みます.
///
/// # Arguments
///
/// * `path` - ファイルのパス.
///
/// # Returns
///
/// 中身. 読めなければ, 理由の文字列の [`Err`].
#[tauri::command]
pub fn read_file_content(path: String) -> Result<String, String> {
    log::debug!("Reading {}", path);
    std::fs::read_to_string(&path).map_err(|e| {
        log::warn!("Could not read {}: {}", path, e);
        e.to_string()
    })
}

/// ファイルに書きます.
///
/// # Arguments
///
/// * `path` - ファイルのパス.
/// * `content` - 中身.
///
/// # Returns
///
/// 書けなければ, 理由の文字列の [`Err`].
#[tauri::command]
pub fn write_file_content(path: String, content: String) -> Result<(), String> {
    log::debug!("Writing {}", path);
    std::fs::write(&path, content).map_err(|e| {
        log::warn!("Could not write {}: {}", path, e);
        e.to_string()
    })
}

/// パスがあるかを返します.
///
/// # Arguments
///
/// * `path` - パス.
///
/// # Returns
///
/// あれば `true`.
#[tauri::command]
pub fn path_exists(path: String) -> bool {
    std::path::Path::new(&path).exists()
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

/// ホームディレクトリ (環境変数 `HOME`) を返します.
///
/// # Returns
///
/// ホームディレクトリ. 環境変数がなければ, 理由の文字列の [`Err`].
#[tauri::command]
pub fn home_dir() -> Result<String, String> {
    std::env::var("HOME").map_err(|e| e.to_string())
}
