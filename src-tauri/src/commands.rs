//! 画面 (フロントエンド) から呼ぶコマンド. Tauri に依存する部分は薄くして, 本体は普通の関数にします.

use std::path::{Path, PathBuf};
#[cfg(not(tarpaulin_include))]
use std::sync::Arc;

use serde::{Deserialize, Serialize};
#[cfg(not(tarpaulin_include))]
use tauri::{AppHandle, Emitter, State};

use crate::atomic::write_atomic;
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

/// 初期設定ファイルの候補を, 探す順に返します.
///
/// # Arguments
///
/// * `home` - ホームディレクトリ (分からなければ `None`).
/// * `config_dir` - 設定のディレクトリ (Linux は `~/.config`. 分からなければ `None`).
///
/// # Returns
///
/// `<home>/.rseditor` と, `<config_dir>/rseditor/init.scm`.
pub fn init_file_candidates(home: Option<&Path>, config_dir: Option<&Path>) -> Vec<PathBuf> {
    home.map(|dir| dir.join(".rseditor"))
        .into_iter()
        .chain(config_dir.map(|dir| dir.join("rseditor").join("init.scm")))
        .collect()
}

/// 初期設定ファイルのパスを返します. `~/.rseditor` が無ければ, `~/.config/rseditor/init.scm` を使います.
///
/// # Returns
///
/// あるファイルのパス. どちらも無ければ `None`.
pub fn init_file_path() -> Option<PathBuf> {
    init_file_candidates(dirs::home_dir().as_deref(), dirs::config_dir().as_deref())
        .into_iter()
        .find(|path| path.exists())
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

/// 初期設定ファイル (`~/.rseditor`, または `~/.config/rseditor/init.scm`) を読んで, 評価します (アプリの評価器を使う).
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

/// ファイルに書きます. 原子的に書くので, 書いている途中で落ちても, ファイルが途中で切れません ([`write_atomic`]).
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
    write_atomic(Path::new(&path), content.as_bytes()).map_err(|e| {
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

/// ファイルのバックアップ (`<path>~`) を作ります (Emacs の `make-backup-files`. 前の版を写す).
/// ファイルが無ければ, 何もしません.
///
/// # Arguments
///
/// * `path` - ファイルのパス.
///
/// # Returns
///
/// 写せなければ, 理由の文字列の [`Err`].
#[tauri::command]
pub fn backup_file(path: String) -> Result<(), String> {
    if !Path::new(&path).is_file() {
        return Ok(());
    }
    log::debug!("Backing up {}", path);
    std::fs::copy(&path, format!("{}~", path))
        .map(|_| ())
        .map_err(|e| e.to_string())
}

/// ファイルを消します. 無ければ, 何もしません (自動保存のファイル).
///
/// # Arguments
///
/// * `path` - ファイルのパス.
///
/// # Returns
///
/// 消せなければ, 理由の文字列の [`Err`].
#[tauri::command]
pub fn delete_file(path: String) -> Result<(), String> {
    match std::fs::remove_file(&path) {
        Err(e) if e.kind() != std::io::ErrorKind::NotFound => Err(e.to_string()),
        _ => Ok(()),
    }
}

/// ファイルの更新日時を返します.
///
/// # Arguments
///
/// * `path` - ファイルのパス.
///
/// # Returns
///
/// 更新日時. ファイルが無ければ `None`.
fn modified_time(path: &str) -> Option<std::time::SystemTime> {
    std::fs::metadata(path).and_then(|m| m.modified()).ok()
}

/// 1 つ目のファイルが, 2 つ目のファイルより新しければ `true` (Emacs の `file-newer-than-file-p`).
/// 1 つ目だけがあるときも `true` です.
///
/// # Arguments
///
/// * `file1` - 1 つ目のファイル.
/// * `file2` - 2 つ目のファイル.
///
/// # Returns
///
/// 新しければ `true`.
#[tauri::command]
pub fn file_newer_than(file1: String, file2: String) -> bool {
    match (modified_time(&file1), modified_time(&file2)) {
        (Some(a), Some(b)) => a > b,
        (a, _) => a.is_some(),
    }
}

/// ディレクトリの一覧を, `ls -al` で作ります (Emacs の dired の `dired-listing-switches`).
///
/// # Arguments
///
/// * `program` - `ls` のプログラム.
/// * `path` - ディレクトリのパス.
///
/// # Returns
///
/// `ls` の出力. ディレクトリを読めないか, `ls` が失敗すれば, 理由の文字列の [`Err`].
pub fn list_directory_long_with(program: &str, path: &str) -> Result<String, String> {
    std::fs::read_dir(path).map_err(|e| e.to_string())?;
    let output = std::process::Command::new(program)
        .args(["-al", "--", path])
        .output()
        .map_err(|e| e.to_string())?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// ディレクトリの一覧を, `ls -al` で作ります (`C-x d`).
///
/// # Arguments
///
/// * `path` - ディレクトリのパス.
///
/// # Returns
///
/// `ls` の出力. 作れなければ, 理由の文字列の [`Err`].
#[tauri::command]
pub fn list_directory_long(path: String) -> Result<String, String> {
    list_directory_long_with("ls", &path)
}

/// ディレクトリの下のファイルから, 正規表現に合う行を探します (`M-x grep`). 探すのは, 別のスレッドで行います.
///
/// # Arguments
///
/// * `pattern` - 正規表現.
/// * `directory` - ディレクトリ.
///
/// # Returns
///
/// 合った行. 正規表現が誤っているか, ディレクトリを読めなければ, 理由の文字列の [`Err`].
#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn grep_directory(
    pattern: String,
    directory: String,
) -> Result<Vec<crate::search::GrepMatch>, String> {
    tauri::async_runtime::spawn_blocking(move || {
        crate::search::grep_directory(&pattern, Path::new(&directory))
    })
    .await
    .map_err(|e| e.to_string())?
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
