//! rseditor-core: Emacs のように使えるエディタの核. 画面 (Tauri) にもターミナルにも頼らない.
//!
//! Scheme の評価器 ([`scheme`]), ファイルの原子的な書き込み ([`atomic`]), バッファ ([`buffer`]. 本文, カーソル, マーク, 元に戻す履歴) と
//! kill-ring ([`kill_ring`]), ファイルの読み書きと自動保存 ([`files`]), 探す処理 ([`search`]), 構文 ([`syntax`]), ウィンドウの分け方 ([`window`]) と
//! エディタの状態 ([`editor`]. バッファの一覧, ウィンドウ, 前置引数), キーの割り当て ([`keymap`]) と
//! コマンドループ ([`command_loop`]. キーを引いてコマンドを動かす. 画面とターミナルで共通) を受け持ちます.

pub mod atomic;
pub mod buffer;
pub mod casefiddle;
pub mod command_loop;
pub mod comment;
pub mod editor;
pub mod error;
pub mod files;
pub mod fill;
pub mod frame;
pub mod indent;
pub mod indent_specs;
pub mod kbd_macro;
pub mod keymap;
pub mod kill_ring;
pub mod paragraph;
pub mod replace;
pub mod scheme;
pub mod search;
pub mod sexp;
pub mod sync;
pub mod syntax;
pub mod window;
