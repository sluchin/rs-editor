//! 画面の要る操作 (`message`, `read-string`, `completing-read`, `read-key` など) を受ける [`Host`].
//!
//! 評価器は、別のスレッドでコマンドを動かし、画面の要る操作で止まって、キーが来るのを待ちます。
//! ここでは、その場で画面を描き、キーを読んで,答えを返します。

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, MutexGuard};

use rseditor_core::editor::Editor;
use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::value::Value;

use crate::keymap::Keymap;
use crate::render::{render, Minibuffer};
use crate::screen::Screen;

/// 画面とその状態.
pub struct UiState {
    /// 画面.
    pub screen: Box<dyn Screen>,
    /// エコー領域の文字.
    pub message: String,
    /// ウィンドウごとの、画面の一番上の行.
    pub tops: BTreeMap<usize, usize>,
    /// 終わるか (`kill-emacs`).
    pub quit: bool,
}

/// 入力の補完.
enum Completion {
    /// 補完しない.
    None,
    /// 候補から.
    Candidates(Vec<String>),
    /// ファイルの名前.
    Files,
}

/// 画面の要る操作を受けるもの.
pub struct UiHost {
    /// 画面とその状態.
    state: Arc<Mutex<UiState>>,
    /// エディタ (描くために読む).
    editor: Arc<Mutex<Editor>>,
    /// キーの割り当て (`where-is-internal` に使う).
    keymap: Arc<Mutex<Keymap>>,
}

impl UiHost {
    /// 作ります.
    pub fn new(
        state: Arc<Mutex<UiState>>,
        editor: Arc<Mutex<Editor>>,
        keymap: Arc<Mutex<Keymap>>,
    ) -> Self {
        UiHost {
            state,
            editor,
            keymap,
        }
    }

    /// 状態を借ります.
    fn state(&self) -> MutexGuard<'_, UiState> {
        self.state.lock().unwrap()
    }

    /// 画面を描きます. 入力を読んでいるときは、ミニバッファを出す.
    fn draw(&self, minibuffer: Option<&Minibuffer>) {
        let editor = self.editor.lock().unwrap();
        let mut state = self.state();
        let (rows, cols) = state.screen.size();
        let message = state.message.clone();
        let frame = render(&editor, &mut state.tops, rows, cols, &message, minibuffer);
        state.screen.draw(&frame);
    }

    /// キーを読みます. 入力が終わっていたら、失敗にする.
    fn read_key(&self) -> Result<String, String> {
        self.state()
            .screen
            .read_key()
            .ok_or_else(|| "Quit".to_string())
    }

    /// ミニバッファで 1 行を読みます.
    fn read_line(
        &self,
        prompt: &str,
        initial: &str,
        completion: Completion,
    ) -> Result<String, String> {
        let mut input: Vec<char> = initial.chars().collect();
        let mut cursor = input.len();
        loop {
            let text: String = input.iter().collect();
            self.draw(Some(&Minibuffer {
                prompt,
                input: &text,
                cursor,
            }));
            let key = self.read_key()?;
            match key.as_str() {
                "RET" => return Ok(text),
                "C-g" => return Err("Quit".to_string()),
                "DEL" => {
                    if cursor > 0 {
                        cursor -= 1;
                        input.remove(cursor);
                    }
                }
                "C-d" | "<delete>" => {
                    if cursor < input.len() {
                        input.remove(cursor);
                    }
                }
                "C-a" | "<home>" => cursor = 0,
                "C-e" | "<end>" => cursor = input.len(),
                "C-b" | "<left>" => cursor = cursor.saturating_sub(1),
                "C-f" | "<right>" => cursor = (cursor + 1).min(input.len()),
                "C-k" => input.truncate(cursor),
                "TAB" => {
                    let (completed, shown) = complete(&text, &completion);
                    input = completed.chars().collect();
                    cursor = input.len();
                    self.state().message = shown;
                }
                other => {
                    if let Some(c) = printable(other) {
                        input.insert(cursor, c);
                        cursor += 1;
                    }
                }
            }
        }
    }
}

/// キーが、入れる文字なら、その文字.
pub fn printable(key: &str) -> Option<char> {
    if key == "SPC" {
        return Some(' ');
    }
    let mut chars = key.chars();
    match (chars.next(), chars.next()) {
        (Some(c), None) if !c.is_control() => Some(c),
        _ => None,
    }
}

/// 共通の前の部分 (`items` は、空でない).
fn common_prefix(items: &[String]) -> String {
    let mut prefix: Vec<char> = items[0].chars().collect();
    for item in &items[1..] {
        let shared = prefix
            .iter()
            .zip(item.chars())
            .take_while(|(a, b)| **a == *b)
            .count();
        prefix.truncate(shared);
    }
    prefix.into_iter().collect()
}

/// 入力を補完します.
///
/// # Returns
///
/// 補完した入力と、エコー領域に出す文字.
fn complete(input: &str, completion: &Completion) -> (String, String) {
    let candidates: Vec<String> = match completion {
        Completion::None => return (input.to_string(), String::new()),
        Completion::Candidates(items) => items
            .iter()
            .filter(|c| c.starts_with(input))
            .cloned()
            .collect(),
        Completion::Files => file_candidates(input),
    };
    match candidates.len() {
        0 => (input.to_string(), "[No match]".to_string()),
        1 => (candidates[0].clone(), "[Sole completion]".to_string()),
        _ => {
            let prefix = common_prefix(&candidates);
            let shown: Vec<&str> = candidates.iter().take(8).map(String::as_str).collect();
            (prefix, shown.join(" "))
        }
    }
}

/// ファイルの名前の補完の候補 (ディレクトリには `/` を付ける).
fn file_candidates(input: &str) -> Vec<String> {
    let (dir, name) = match input.rfind('/') {
        Some(i) => (&input[..=i], &input[i + 1..]),
        None => ("", input),
    };
    let listing = std::fs::read_dir(if dir.is_empty() { "." } else { dir });
    let mut found: Vec<String> = listing
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|entry| {
            let file = entry.file_name().to_string_lossy().into_owned();
            let is_dir = entry.path().is_dir();
            file.starts_with(name)
                .then(|| format!("{dir}{file}{}", if is_dir { "/" } else { "" }))
        })
        .collect();
    found.sort();
    found
}

/// キーの名前を, `read-key` が返す名前 (Lisp のコマンドが決まりにしている、画面のキーの名前) にします.
fn read_key_name(key: &str) -> String {
    match key {
        "SPC" => " ",
        "RET" => "Enter",
        "DEL" => "Backspace",
        "<delete>" => "Delete",
        "ESC" => "Escape",
        other => other,
    }
    .to_string()
}

/// 文字列の引数.
fn text(args: &[Value], i: usize) -> String {
    match args.get(i) {
        Some(Value::String(s)) => s.clone(),
        _ => String::new(),
    }
}

impl Host for UiHost {
    fn call(&self, op: &str, args: Vec<Value>) -> Result<Value, String> {
        match op {
            "message" => {
                self.state().message = text(&args, 0);
                Ok(Value::Nil)
            }
            "read-string" => self
                .read_line(&text(&args, 0), &text(&args, 1), Completion::None)
                .map(Value::String),
            "read-file-name" => self
                .read_line(&text(&args, 0), &text(&args, 1), Completion::Files)
                .map(Value::String),
            "completing-read" => {
                let candidates = match args.get(1) {
                    Some(Value::List(items)) => items
                        .iter()
                        .filter_map(|v| match v {
                            Value::String(s) => Some(s.clone()),
                            _ => None,
                        })
                        .collect(),
                    _ => Vec::new(),
                };
                self.read_line(
                    &text(&args, 0),
                    &text(&args, 2),
                    Completion::Candidates(candidates),
                )
                .map(Value::String)
            }
            "read-key" => {
                self.state().message = String::new();
                self.draw(Some(&Minibuffer {
                    prompt: &text(&args, 0),
                    input: "",
                    cursor: 0,
                }));
                let key = self.read_key()?;
                if key == "C-g" {
                    return Err("Quit".to_string());
                }
                Ok(Value::String(read_key_name(&key)))
            }
            "window-body-height" => {
                let editor = self.editor.lock().unwrap();
                let state = self.state();
                let rows = state.screen.size().0.saturating_sub(1);
                let height = editor
                    .layout()
                    .rects(rseditor_core::window::FULL)
                    .into_iter()
                    .find(|r| r.id == editor.selected_window())
                    .map_or(rows, |r| (r.height / 100.0 * rows as f64).round() as usize);
                Ok(Value::Number(height.saturating_sub(1) as f64))
            }
            "recenter" => {
                let editor = self.editor.lock().unwrap();
                let mut state = self.state();
                let rows = state.screen.size().0;
                let line = editor
                    .current_buffer()
                    .line_of_position(editor.current_buffer().point());
                let half = rows.saturating_sub(2) / 2;
                state
                    .tops
                    .insert(editor.selected_window(), line.saturating_sub(half));
                Ok(Value::Nil)
            }
            "kill-emacs" => {
                self.state().quit = true;
                Ok(Value::Nil)
            }
            "where-is-internal" => {
                let keys = self.keymap.lock().unwrap().keys_for(&text(&args, 0));
                Ok(Value::List(keys.into_iter().map(Value::String).collect()))
            }
            "editor-command-names" => Ok(Value::List(Vec::new().into())),
            "command" => Err(format!("Undefined command: {}", text(&args, 0))),
            // 色, 下線, 隠すこと: 端末では何もしない
            "highlight-regexp"
            | "unhighlight-regexp"
            | "outline-flag-region"
            | "local-set-key"
            | "set-background-color"
            | "set-foreground-color"
            | "set-font" => Ok(Value::Nil),
            "outline-invisible-p" => Ok(Value::Boolean(false)),
            other => Err(format!("The terminal does not support {other}")),
        }
    }
}
