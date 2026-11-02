//! rseditor: Emacs のように使えるエディタ (Scheme の評価器付き) のバックエンド.
//!
//! 画面のエディタ ([`session`]. 画面への窓口は `front`), 画面だけで使うコマンド ([`commands`]), Scheme の評価 ([`scheme`]), ディレクトリの中の検索 ([`search`]), ログ ([`logging`]) を受け持ちます.

pub mod commands;
#[cfg(not(tarpaulin_include))]
pub mod front;
pub mod logging;
pub mod search;
pub mod session;

pub use rseditor_core::{atomic, scheme};

use std::sync::Arc;
use std::time::Duration;

use scheme::host::Bridge;

/// 画面の返事を待つ長さ (エディタの操作 1 つごと).
const EDITOR_TIMEOUT: Duration = Duration::from_secs(5);

/// アプリケーションを起動します.
///
/// # Panics
///
/// Tauri の起動に失敗したとき.
#[cfg(not(tarpaulin_include))]
pub fn run() {
    logging::init_logging();
    log::info!("Started rseditor {}", env!("CARGO_PKG_VERSION"));
    tauri::Builder::default()
        .manage(Arc::new(Bridge::new(EDITOR_TIMEOUT)))
        .manage(front::Inputs::default())
        .invoke_handler(tauri::generate_handler![
            logging::log_frontend_message,
            commands::editor_reply,
            front::session_input,
            commands::list_directory,
            commands::home_dir,
            commands::project_root
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
