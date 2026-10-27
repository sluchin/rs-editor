//! 画面のバッファを核に写して、本文の操作を核で動かす仕組み (今の画面 (Tauri) をつなぐ).
//!
//! 評価の間だけ、画面の今のバッファ(本文, カーソル, マーク)を核の [`Editor`] に写します (スナップショット).
//! 探す, マーク, 構文などの新しい組み込みは、写した本文に対して、画面に頼まずに動かします.
//! 画面に頼む操作の前と、評価の終わりに、核での変更を画面に返します.
//! バッファの出し入れ, ファイル, ウィンドウ, 既存の操作 (`point`, `insert` など) は、これまでどおり画面が受け持ちます.

use std::sync::{Arc, Mutex};

use crate::editor::Editor;
use crate::scheme::editor_host::{dialect_of, EditorHost};
use crate::scheme::host::Host;
use crate::scheme::value::Value;

/// 核で動かす操作 (今のバッファの本文に対する、新しい組み込み).
const NATIVE: &[&str] = &[
    "buffer-size",
    "char-after",
    "char-before",
    "forward-char",
    "backward-char",
    "forward-line",
    "line-beginning-position",
    "line-end-position",
    "bobp",
    "eobp",
    "bolp",
    "eolp",
    "delete-char",
    "current-column",
    "move-to-column",
    "skip-chars-forward",
    "skip-chars-backward",
    "erase-buffer",
    "search-forward",
    "search-backward",
    "re-search-forward",
    "re-search-backward",
    "looking-at",
    "match-beginning",
    "match-end",
    "match-data",
    "match-string",
    "mark",
    "set-mark",
    "set-mark-position",
    "push-mark",
    "pop-mark",
    "deactivate-mark",
    "region-beginning",
    "region-end",
    "use-region-p",
    "region-active-p",
    "scan-lists",
    "scan-sexps",
    "syntax-ppss",
    "char-syntax",
    "forward-comment",
    "backward-prefix-chars",
    "up-list-position",
    "beginning-of-defun-position",
    "end-of-defun-position",
    "mark-defun-range",
    "forward-paragraph-position",
    "forward-sentence-position",
    "syntax-dialect",
    "set-syntax-dialect",
    "prefix-numeric-value",
];

/// 画面に頼んでも、写したバッファが変わらない操作 (問い合わせと、画面だけの操作).
const KEEPS_MIRROR: &[&str] = &[
    "point",
    "point-min",
    "point-max",
    "buffer-string",
    "buffer-substring",
    "buffer-name",
    "message",
    "read-string",
    "read-file-name",
    "read-key",
    "completing-read",
    "highlight-regexp",
    "unhighlight-regexp",
    "outline-flag-region",
    "outline-invisible-p",
    "where-is-internal",
    "editor-command-names",
    "local-set-key",
    "buffer-file-name",
    "buffer-modified-p",
    "buffer-list",
    "kill-new",
    "set-background-color",
    "set-foreground-color",
    "set-font",
];

/// 写したときのバッファの状態.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Loaded {
    /// バッファの名前.
    name: String,
    /// 本文.
    text: String,
    /// カーソル (0 から).
    point: usize,
    /// マーク (0 から).
    mark: Option<usize>,
}

/// 画面のバッファを核に写して、核での変更を画面に返すもの.
pub struct Mirror<'a> {
    /// 画面.
    frontend: &'a dyn Host,
    /// 写した先の核.
    editor: Arc<Mutex<Editor>>,
    /// 写したときの状態 (写していなければ `None`).
    loaded: Mutex<Option<Loaded>>,
}

/// 値を、位置 (1 から数える数) にします.
fn position(v: Option<&Value>) -> Option<usize> {
    match v {
        Some(Value::Number(n)) if *n >= 1.0 => Some(*n as usize - 1),
        _ => None,
    }
}

/// 画面の返事 (名前, 本文, カーソル, マーク, ファイル) を, 写したときの状態と, ファイルにします.
fn parse_snapshot(snapshot: &Value) -> Option<(Loaded, Option<String>)> {
    let Value::List(items) = snapshot else {
        return None;
    };
    let (Some(Value::String(name)), Some(Value::String(text))) = (items.first(), items.get(1))
    else {
        return None;
    };
    let file = match items.get(4) {
        Some(Value::String(file)) => Some(file.clone()),
        _ => None,
    };
    let state = Loaded {
        name: name.clone(),
        text: text.clone(),
        point: position(items.get(2)).unwrap_or(0),
        mark: position(items.get(3)),
    };
    Some((state, file))
}

impl<'a> Mirror<'a> {
    /// 画面と、写し先の核から作ります.
    pub fn new(frontend: &'a dyn Host, editor: Arc<Mutex<Editor>>) -> Self {
        Mirror {
            frontend,
            editor,
            loaded: Mutex::new(None),
        }
    }

    /// 画面の今のバッファを、核に写します (写していなければ).
    fn ensure_loaded(&self) -> Result<(), String> {
        let mut loaded = self.loaded.lock().unwrap();
        if loaded.is_some() {
            return Ok(());
        }
        let snapshot = self.frontend.call("editor-snapshot", vec![])?;
        let (state, file) = parse_snapshot(&snapshot).ok_or("editor-snapshot: unexpected reply")?;
        // 核は写したバッファだけを持つ (前に写したバッファは、古くなっている)
        let mut editor = self.editor.lock().unwrap();
        *editor = Editor::new();
        let buffer = editor.get_buffer_create(&state.name);
        buffer.load(&state.text);
        buffer.set_point(state.point);
        if let Some(mark) = state.mark {
            buffer.set_mark(mark);
        }
        if let Some(file) = file {
            buffer.set_dialect(dialect_of(&file));
        }
        // 写したバッファが、今のバッファ
        editor.set_buffer(&state.name)?;
        *loaded = Some(state);
        Ok(())
    }

    /// 核での変更を、画面に返します. 変更が無ければ、何もしない.
    ///
    /// # Errors
    ///
    /// 画面が受けなかったとき.
    pub fn flush(&self) -> Result<(), String> {
        let mut loaded = self.loaded.lock().unwrap();
        let Some(before) = loaded.as_mut() else {
            return Ok(());
        };
        let now = {
            let editor = self.editor.lock().unwrap();
            let buffer = editor.current_buffer();
            Loaded {
                name: before.name.clone(),
                text: buffer.text(),
                point: buffer.point(),
                mark: buffer.mark(),
            }
        };
        if now == *before {
            return Ok(());
        }
        self.frontend.call(
            "editor-apply",
            vec![
                Value::String(now.text.clone()),
                Value::Number(now.point as f64 + 1.0),
                now.mark
                    .map_or(Value::Boolean(false), |m| Value::Number(m as f64 + 1.0)),
            ],
        )?;
        *before = now;
        Ok(())
    }
}

impl Host for Mirror<'_> {
    fn call(&self, op: &str, args: Vec<Value>) -> Result<Value, String> {
        // 画面に頼む前に、核での変更を画面に返す
        self.flush()?;
        let result = self.frontend.call(op, args);
        if !KEEPS_MIRROR.contains(&op) {
            *self.loaded.lock().unwrap() = None;
        }
        result
    }

    fn is_native(&self, op: &str) -> bool {
        NATIVE.contains(&op)
    }

    fn before_native(&self, _op: &str) -> Result<(), String> {
        self.ensure_loaded()
    }
}

/// 画面のバッファを核に写す [`Host`] を作って `f` を動かし、終わったら核での変更を画面に返します.
///
/// # Arguments
///
/// * `frontend` - 画面.
/// * `f` - ホストを使う処理 (Scheme の評価など).
///
/// # Returns
///
/// `f` の結果と, 最後に画面に返せなかったときの失敗の内容.
pub fn with_mirror<R>(frontend: &dyn Host, f: impl FnOnce(&dyn Host) -> R) -> (R, Option<String>) {
    let editor = Arc::new(Mutex::new(Editor::new()));
    let host = EditorHost::shared(editor.clone(), Mirror::new(frontend, editor));
    let result = f(&host);
    (result, host.ui().flush().err())
}
