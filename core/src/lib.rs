//! rseditor-core: Emacs のように使えるエディタの核. 画面 (Tauri) にもターミナルにも頼らない.
//!
//! Scheme の評価器 ([`scheme`]) と, ファイルの原子的な書き込み ([`atomic`]) を受け持ちます.

pub mod atomic;
pub mod scheme;
