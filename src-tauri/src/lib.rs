//! rseditor: Emacs のように使えるエディタ (Scheme の評価器付き) のバックエンド.
//!
//! ファイルの読み書き ([`commands`]), Scheme の評価 ([`scheme`]), ログ ([`logging`]) を受け持ちます.

pub mod commands;
pub mod logging;
pub mod scheme;

use scheme::evaluator::Evaluator;

/// アプリケーションを起動します.
///
/// # Panics
///
/// Tauri の起動に失敗したとき.
#[cfg(not(tarpaulin_include))]
pub fn run() {
    logging::init_logging();
    log::info!("Started rseditor {}", env!("CARGO_PKG_VERSION"));
    let evaluator = Evaluator::new();

    tauri::Builder::default()
        .manage(evaluator)
        .invoke_handler(tauri::generate_handler![
            logging::log_frontend_message,
            commands::eval_scheme,
            commands::read_file_content,
            commands::write_file_content,
            commands::path_exists,
            commands::home_dir
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
