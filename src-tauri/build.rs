//! ビルドスクリプト. Tauri の設定 (`tauri.conf.json`) などから, アプリに組み込むものを作ります.

/// Tauri のビルドの処理を行います.
fn main() {
    tauri_build::build()
}
