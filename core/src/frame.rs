//! 描く内容 (frame): 画面 (Tauri) が描くための、ウィンドウとバッファの状態.
//!
//! コマンドの後に作って、画面に送ります. 本文は、前に送ったバッファなら変更だけ (UTF-16 の位置) を、
//! 初めて送るバッファと、読み直したバッファは全体を送ります.

use std::collections::HashSet;

use serde::Serialize;

use crate::buffer::{Changes, Highlight, TextChange};
use crate::editor::Editor;
use crate::mode::ModeInfo;
use crate::window::FULL;

/// ウィンドウの描く内容.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WindowFrame {
    /// ウィンドウの番号.
    pub id: usize,
    /// 左の位置 (画面の幅に対する %).
    pub left: f64,
    /// 上の位置 (画面の高さに対する %).
    pub top: f64,
    /// 幅 (%).
    pub width: f64,
    /// 高さ (%).
    pub height: f64,
    /// 出しているバッファの名前.
    pub buffer: String,
    /// カーソル (UTF-16 の位置).
    pub point: usize,
    /// リージョンの端 (マークを使っているときだけ. UTF-16 の位置).
    pub mark: Option<usize>,
}

/// 強調する正規表現 (Lisp の `highlight-regexp`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HighlightFrame {
    /// 正規表現 (Emacs の書き方).
    pub regexp: String,
    /// face の名前.
    pub face: String,
    /// 色付けする括弧の番号.
    pub group: usize,
}

/// シンタックスハイライトの字句 (種類と範囲).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TokenFrame {
    /// 始まり (UTF-16 の位置).
    pub from: usize,
    /// 終わり (UTF-16 の位置. この位置の文字は含まない).
    pub to: usize,
    /// 種類の名前 (`comment`, `keyword` など. `TokenKind::name`).
    pub kind: &'static str,
}

/// バッファの描く内容.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct BufferFrame {
    /// 名前.
    pub name: String,
    /// 本文の版.
    pub version: u64,
    /// 本文の全体 (初めて送るときと, 読み直したとき. ほかは `None`).
    pub text: Option<String>,
    /// 前に送ってからの変更 (全体を送るときと, ウィンドウに出していないときは空).
    pub changes: Vec<TextChange>,
    /// 書き換えたか.
    pub modified: bool,
    /// 読み取り専用か.
    pub read_only: bool,
    /// ファイルのパス.
    pub file: Option<String>,
    /// dired のディレクトリ.
    pub directory: Option<String>,
    /// 強調する正規表現.
    pub highlights: Vec<HighlightFrame>,
    /// シンタックスハイライトの字句 (言語があるバッファで、ウィンドウに出しているものだけ).
    pub tokens: Vec<TokenFrame>,
    /// メジャーモードの情報 (モードラインの名前、Lisp の方言、字下げ).
    pub mode: ModeInfo,
}

/// 描く内容.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Frame {
    /// ウィンドウ (左上から).
    pub windows: Vec<WindowFrame>,
    /// 選んでいるウィンドウの番号.
    pub selected: usize,
    /// バッファ (開いた順. 本文は, ウィンドウに出しているものだけ).
    pub buffers: Vec<BufferFrame>,
    /// エコー領域.
    pub message: String,
    /// タブの名前 (タブが 2 つ未満なら空. 空ならタブの行を出さない).
    pub tabs: Vec<String>,
    /// 選んでいるタブの番号.
    pub tab: usize,
}

/// 描く内容を作るもの (前に本文を送ったバッファを覚える).
#[derive(Debug, Default)]
pub struct FrameBuilder {
    /// 本文を送ったバッファの名前.
    sent: HashSet<String>,
}

impl FrameBuilder {
    /// 作ります.
    pub fn new() -> Self {
        Self::default()
    }

    /// 全ての本文を送り直すようにします (画面を作り直したとき).
    pub fn reset(&mut self) {
        self.sent.clear();
    }

    /// 描く内容を作ります. ウィンドウに出しているバッファの変更を取り出す.
    ///
    /// # Arguments
    ///
    /// * `editor` - エディタ.
    /// * `message` - エコー領域の文字.
    pub fn build(&mut self, editor: &mut Editor, message: &str) -> Frame {
        let shown: HashSet<String> = editor
            .window_list()
            .into_iter()
            .filter_map(|w| editor.window_buffer(w).map(str::to_string))
            .collect();
        let names = editor.buffer_list();
        // 消したバッファは, 忘れる
        self.sent.retain(|name| names.contains(name));
        let mut buffers = Vec::new();
        for name in &names {
            let buffer = editor.get_buffer_mut(name).expect("listed buffer");
            let (text, changes) = if shown.contains(name) {
                match (buffer.take_changes(), self.sent.insert(name.clone())) {
                    (Changes::Edits(changes), false) => (None, changes),
                    _ => (Some(buffer.text()), Vec::new()),
                }
            } else {
                (None, Vec::new())
            };
            buffers.push(BufferFrame {
                name: name.clone(),
                version: buffer.version(),
                text,
                changes,
                modified: buffer.modified(),
                read_only: buffer.read_only(),
                file: buffer.file().map(str::to_string),
                directory: buffer.directory().map(str::to_string),
                highlights: buffer.highlights().iter().map(highlight_frame).collect(),
                mode: buffer.mode_info(),
                tokens: if shown.contains(name) {
                    buffer
                        .token_spans()
                        .iter()
                        .map(|span| TokenFrame {
                            from: buffer.utf16_position(span.start),
                            to: buffer.utf16_position(span.end),
                            kind: span.kind.name(),
                        })
                        .collect()
                } else {
                    Vec::new()
                },
            });
        }
        let windows = editor
            .layout()
            .rects(FULL)
            .into_iter()
            .map(|r| {
                let name = editor.window_buffer(r.id).unwrap_or_default().to_string();
                let buffer = editor.get_buffer(&name).expect("window buffer");
                let utf16 = |pos: usize| buffer.utf16_position(pos);
                WindowFrame {
                    id: r.id,
                    left: r.left,
                    top: r.top,
                    width: r.width,
                    height: r.height,
                    point: utf16(buffer.point()),
                    mark: buffer.mark().filter(|_| buffer.mark_active()).map(utf16),
                    buffer: name,
                }
            })
            .collect();
        Frame {
            windows,
            selected: editor.selected_window(),
            buffers,
            message: message.to_string(),
            tabs: editor.tab_bar().0.to_vec(),
            tab: editor.tab_bar().1,
        }
    }
}

/// 強調する正規表現の描く内容.
fn highlight_frame(h: &Highlight) -> HighlightFrame {
    HighlightFrame {
        regexp: h.regexp.clone(),
        face: h.face.clone(),
        group: h.group,
    }
}
