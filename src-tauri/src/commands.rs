//! 画面 (フロントエンド) から呼ぶコマンド. Tauri に依存する部分は薄くして, 本体は普通の関数にします.

use serde::{Deserialize, Serialize};
use tauri::State;

use crate::scheme::evaluator::Evaluator;

/// Scheme の評価の結果.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct EvalResponse {
    /// 結果 (失敗したときは, 空).
    pub result: String,
    /// 失敗の内容 (成功したときは, `None`).
    pub error: Option<String>,
}

/// Scheme のコードを評価します.
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
    match evaluator.eval(code) {
        Ok(result) => EvalResponse {
            result,
            error: None,
        },
        Err(e) => EvalResponse {
            result: String::new(),
            error: Some(e),
        },
    }
}

/// Scheme のコードを評価します (アプリの評価器を使う).
///
/// # Arguments
///
/// * `code` - コード.
/// * `state` - アプリの評価器.
///
/// # Returns
///
/// 結果, または失敗の内容.
#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub fn eval_scheme(code: String, state: State<'_, Evaluator>) -> EvalResponse {
    eval_code(&state, &code)
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

/// ホームディレクトリ (環境変数 `HOME`) を返します.
///
/// # Returns
///
/// ホームディレクトリ. 環境変数がなければ, 理由の文字列の [`Err`].
#[tauri::command]
pub fn home_dir() -> Result<String, String> {
    std::env::var("HOME").map_err(|e| e.to_string())
}
