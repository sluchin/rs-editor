//! アプリケーションのエントリポイントモジュール.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// アプリケーションのエントリポイント.
///
/// `rseditor` ライブラリの実行関数を呼び出し, アプリケーションを起動します.
#[cfg(not(tarpaulin_include))]
fn main() {
    rseditor::run();
}
