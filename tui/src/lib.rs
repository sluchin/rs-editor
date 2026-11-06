//! rseditor-tui: ターミナルで動くエディタ (ncurses). エディタの核 (`rseditor-core`) を、キーと画面でつなぎます.
//!
//! - `batch`: バッチモード (`--batch`. 画面を使わずに、引数の順に動かす).
//! - `app`: キーを受けて、コマンドを動かし、画面を描く.
//! - `render`: エディタの状態から画面を作る.
//! - `ui`: 画面の要る操作 (ミニバッファの入力など).
//! - `screen`: 画面の窓口 (ncurses の実装は、アプリの本体にある).

pub mod app;
pub mod batch;
pub mod history;
pub mod render;
pub mod screen;
pub mod ui;

#[cfg(not(tarpaulin_include))]
pub mod ncurses_screen;
