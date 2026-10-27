//! rseditor-core: Emacs のように使えるエディタの核. 画面 (Tauri) にもターミナルにも頼らない.
//!
//! Scheme の評価器 ([`scheme`]), ファイルの原子的な書き込み ([`atomic`]), バッファ ([`buffer`]. 本文, カーソル, マーク, 元に戻す履歴) と
//! kill-ring ([`kill_ring`]), 探す処理 ([`search`]), 構文 ([`syntax`]), ウィンドウの分け方 ([`window`]) と
//! エディタの状態 ([`editor`]. バッファの一覧, ウィンドウ, 前置引数) を受け持ちます.

pub mod atomic;
pub mod buffer;
pub mod editor;
pub mod kill_ring;
pub mod paragraph;
pub mod scheme;
pub mod search;
pub mod sexp;
pub mod syntax;
pub mod window;
