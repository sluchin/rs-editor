//! rseditor: Emacs のように使えるエディタ (Scheme の評価器付き) のバックエンド.
//!
//! ファイルの読み書き ([`commands`]. 書くのは原子的に ([`atomic`])), Scheme の評価 ([`scheme`]), ディレクトリの中の検索 ([`search`]), ログ ([`logging`]) を受け持ちます.

pub mod atomic;
pub mod commands;
pub mod logging;
pub mod scheme;
pub mod search;

use std::sync::Arc;
use std::time::Duration;

use scheme::evaluator::Evaluator;
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
        .manage(Arc::new(Evaluator::new()))
        .manage(Arc::new(Bridge::new(EDITOR_TIMEOUT)))
        .invoke_handler(tauri::generate_handler![
            logging::log_frontend_message,
            commands::eval_scheme,
            commands::run_lisp_command,
            commands::load_init,
            commands::editor_reply,
            commands::read_file_content,
            commands::write_file_content,
            commands::path_exists,
            commands::list_directory,
            commands::home_dir,
            commands::bookmark_file_path,
            commands::backup_file,
            commands::delete_file,
            commands::file_newer_than,
            commands::list_directory_long,
            commands::grep_directory,
            commands::project_root
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
