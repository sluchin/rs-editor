//! 画面の要る操作 (`message`, `read-string`, `completing-read`, `read-key` など) を受ける [`Host`].
//!
//! 評価器は、別のスレッドでコマンドを動かし、画面の要る操作で止まって、キーが来るのを待ちます。
//! ここでは、その場で画面を描き、キーを読んで,答えを返します。

use rseditor_core::error::Error;
use std::collections::{BTreeMap, VecDeque};
use std::sync::{Arc, Mutex, MutexGuard};

use rseditor_core::editor::Editor;
use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::value::Value;

use crate::history::{add_to_history, HistoryArgs, Navigator};
use crate::render::{render_with, Display, Minibuffer};
use crate::screen::Screen;
use rseditor_core::command_loop::LoopUi;
pub use rseditor_core::command_loop::{printable, read_key_name};
use rseditor_core::keymap::{Keymap, Lookup};
use rseditor_core::sync::LockExt;

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
    /// 画面の出し方の設定.
    pub display: Display,
    /// 打ったことにするキー (キーボードマクロを動かしているとき. 画面から読む前に, これを読む).
    pub macro_keys: VecDeque<String>,
    /// 失敗するまで繰り返すマクロのキー (`call-last-kbd-macro` の引数が 0 のとき).
    pub macro_loop: Vec<String>,
    /// 今のキーが, キーボードマクロから来たものか.
    pub executing_macro: bool,
    /// 画面から読んだ最近のキー (古いものから. `view-lossage` に使う).
    pub recent_keys: VecDeque<String>,
    /// ミニバッファの入力の履歴 (履歴の名前ごと. 新しい順).
    pub histories: BTreeMap<String, Vec<String>>,
    /// `M-r` と `M-s` で最後に探した正規表現.
    pub history_regexp: String,
}

/// 覚えておく最近のキーの数 (Emacs の `recent-keys` と同じ).
const RECENT_KEYS_MAX: usize = 300;

impl UiState {
    /// 打ったことにするキーを 1 つ取ります (繰り返すマクロは, 尽きたら始めから).
    pub fn pop_macro_key(&mut self) -> Option<String> {
        // 打ったことにするキーが残っている間は、キーを押したか確かめる (`C-g` で止める. Emacs は `quit`)
        if (!self.macro_keys.is_empty() || !self.macro_loop.is_empty()) && self.screen.interrupted()
        {
            self.stop_macro();
            self.message = "Quit".to_string();
            self.executing_macro = false;
            return None;
        }
        if self.macro_keys.is_empty() && !self.macro_loop.is_empty() {
            self.macro_keys = self.macro_loop.iter().cloned().collect();
        }
        let key = self.macro_keys.pop_front();
        self.executing_macro = key.is_some();
        key
    }

    /// 動かしているキーボードマクロを止めます.
    pub fn stop_macro(&mut self) {
        self.macro_keys.clear();
        self.macro_loop.clear();
    }
}

/// キーが来るまで, `timeout` の間だけ待ちます. 打ったことにするキー (キーボードマクロ) があれば, 待たない.
///
/// # Returns
///
/// キーが来たか (入力が終わったときも `true`).
pub fn wait_key(state: &Mutex<UiState>, timeout: std::time::Duration) -> bool {
    let mut state = state.lock_recover();
    !state.macro_keys.is_empty() || !state.macro_loop.is_empty() || state.screen.wait_key(timeout)
}

/// キーを 1 つ読みます. キーボードマクロを動かしているなら, そのキー. なければ, 画面から読みます.
///
/// # Returns
///
/// キーと, 画面から読んだなら `true`. 入力が終わっていれば `None`.
pub fn next_key(state: &Mutex<UiState>) -> Option<(String, bool)> {
    let mut state = state.lock_recover();
    if let Some(key) = state.pop_macro_key() {
        return Some((key, false));
    }
    let key = state.screen.read_key()?;
    state.recent_keys.push_back(key.clone());
    if state.recent_keys.len() > RECENT_KEYS_MAX {
        state.recent_keys.pop_front();
    }
    Some((key, true))
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
        self.state.lock_recover()
    }

    /// 画面を描きます. 入力を読んでいるときは、ミニバッファを出す.
    fn draw(&self, minibuffer: Option<&Minibuffer>) {
        let mut editor = self.editor.lock_recover();
        let mut state = self.state();
        let (rows, cols) = state.screen.size();
        editor.set_screen_size(rows, cols);
        let message = state.message.clone();
        let display = state.display;
        let frame = render_with(
            &editor,
            &mut state.tops,
            rows,
            cols,
            &message,
            minibuffer,
            display,
        );
        state.screen.draw(&frame);
    }

    /// キーを読みます. 入力が終わっていたら、失敗にする.
    fn read_key(&self) -> Result<String, Error> {
        let (key, from_screen) = next_key(&self.state).ok_or_else(|| Error::bare("quit"))?;
        if from_screen {
            // 画面から読んだキーは, マクロを覚えているなら覚える
            self.editor.lock_recover().kbd_macro.record(&key);
        }
        Ok(key)
    }

    /// ミニバッファで 1 行を読みます.
    fn read_line(
        &self,
        prompt: &str,
        initial: &str,
        completion: Completion,
        history: &HistoryArgs,
    ) -> Result<String, Error> {
        let mut input: Vec<char> = initial.chars().collect();
        let mut cursor = input.len();
        let defaults = &history.defaults;
        // Lisp の変数の履歴は, 画面の履歴の代わりに使い, 加えない (isearch の `history-add-new-input` が nil)
        let items = match &history.items {
            Some(items) => items.clone(),
            None => self
                .state()
                .histories
                .get(&history.name)
                .cloned()
                .unwrap_or_default(),
        };
        let mut navigator = Navigator::starting_at(history.start, initial, &items);
        loop {
            let text: String = input.iter().collect();
            self.draw(Some(&Minibuffer {
                prompt,
                input: &text,
                cursor,
            }));
            let key = self.read_key()?;
            match key.as_str() {
                "RET" => {
                    if !history.name.is_empty() && history.items.is_none() {
                        add_to_history(
                            self.state()
                                .histories
                                .entry(history.name.clone())
                                .or_default(),
                            &text,
                        );
                    }
                    return Ok(text);
                }
                "C-g" => return Err(Error::bare("quit")),
                "M-p" | "M-n" | "M-r" | "M-s" => {
                    let result = match key.as_str() {
                        "M-p" => navigator.previous(&text, &items, defaults),
                        "M-n" => navigator.next(&text, &items, defaults),
                        _ => self.search_history(&mut navigator, &key, &text, &items, defaults),
                    };
                    match result {
                        Ok(found) => {
                            input = found.chars().collect();
                            cursor = input.len();
                            self.state().message = String::new();
                        }
                        Err(message) => self.state().message = format!("[{message}]"),
                    }
                }
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

impl UiHost {
    /// `M-r` / `M-s`: 正規表現を読んで, 履歴から探します (入力を空にすると, 前に探した正規表現を使う).
    fn search_history(
        &self,
        navigator: &mut Navigator,
        key: &str,
        text: &str,
        items: &[String],
        defaults: &[String],
    ) -> Result<String, String> {
        let older = key == "M-r";
        let prompt = if older {
            "Previous element matching (regexp): "
        } else {
            "Next element matching (regexp): "
        };
        let regexp = self
            .read_line(prompt, "", Completion::None, &HistoryArgs::default())
            .map_err(|e| e.to_string())?;
        let regexp = if regexp.is_empty() {
            self.state().history_regexp.clone()
        } else {
            self.state().history_regexp = regexp.clone();
            regexp
        };
        navigator.search(&regexp, older, text, items, defaults)
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

/// 文字列の引数.
pub(crate) fn text(args: &[Value], i: usize) -> String {
    match args.get(i) {
        Some(Value::String(s)) => s.clone(),
        _ => String::new(),
    }
}

impl Host for UiHost {
    fn call(&self, op: &str, args: Vec<Value>) -> Result<Value, Error> {
        match op {
            "message" => {
                self.state().message = text(&args, 0);
                Ok(Value::Nil)
            }
            "read-string" => self
                .read_line(
                    &text(&args, 0),
                    &text(&args, 1),
                    Completion::None,
                    &HistoryArgs::from_args(&args, 2),
                )
                .map(Value::String),
            "read-file-name" => self
                .read_line(
                    &text(&args, 0),
                    &text(&args, 1),
                    Completion::Files,
                    &HistoryArgs::from_args(&args, 2),
                )
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
                    &HistoryArgs::named("minibuffer", Vec::new()),
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
                let allow_quit = text(&args, 1) == "allow-quit";
                if key == "C-g" && !allow_quit {
                    return Err(Error::bare("quit"));
                }
                Ok(Value::String(read_key_name(&key)))
            }
            "execute-kbd-macro" => {
                let keys: Vec<String> = match args.first() {
                    Some(Value::List(items)) => items
                        .iter()
                        .filter_map(|v| match v {
                            Value::String(s) => Some(s.clone()),
                            _ => None,
                        })
                        .collect(),
                    _ => Vec::new(),
                };
                let count = match args.get(1) {
                    Some(Value::Number(n)) => *n as usize,
                    _ => 1,
                };
                let mut state = self.state();
                if count == 0 {
                    state.macro_loop = keys;
                } else {
                    // 今のマクロのキーより先に打つ
                    let repeated: Vec<String> = keys
                        .iter()
                        .cloned()
                        .cycle()
                        .take(keys.len() * count)
                        .collect();
                    for key in repeated.into_iter().rev() {
                        state.macro_keys.push_front(key);
                    }
                }
                Ok(Value::Nil)
            }
            "executing-kbd-macro-p" => Ok(Value::Boolean(self.state().executing_macro)),
            "window-body-height" => {
                let editor = self.editor.lock_recover();
                let state = self.state();
                Ok(Value::Number(body_height(&editor, &state) as f64))
            }
            "move-to-window-line" => {
                let mut editor = self.editor.lock_recover();
                let state = self.state();
                let height = body_height(&editor, &state);
                let top = state
                    .tops
                    .get(&editor.selected_window())
                    .copied()
                    .unwrap_or(0);
                // 行の番号は, 窓の一番上の行から. 負の数は一番下の行から数える
                let row = match args.first() {
                    Some(Value::Number(n)) if *n >= 0.0 => *n as usize,
                    Some(Value::Number(n)) => height.saturating_sub((-n) as usize),
                    _ => height / 2,
                }
                .min(height.saturating_sub(1));
                let buffer = editor.current_buffer_mut();
                let last = buffer.line_count().saturating_sub(1);
                let line = (top + row).min(last);
                let current = buffer.line_of_position(buffer.point());
                buffer.forward_line(line as i64 - current as i64);
                Ok(Value::Number((line - top.min(line)) as f64))
            }
            "recenter" => {
                let editor = self.editor.lock_recover();
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
                let keys = self.keymap.lock_recover().keys_for(&text(&args, 0));
                Ok(Value::List(keys.into_iter().map(Value::String).collect()))
            }
            // キーの並び ("C-x C-f") の割り当て: コマンドの名前, 続きがあれば #t, 無ければ #f
            "key-binding" => {
                let keys: Vec<String> = text(&args, 0).split(' ').map(str::to_string).collect();
                Ok(match self.keymap.lock_recover().lookup(&keys) {
                    Lookup::Command(command) => Value::String(command),
                    Lookup::Prefix => Value::Boolean(true),
                    Lookup::Unbound => Value::Boolean(false),
                })
            }
            // 全ての割り当て: (キー コマンド) のリスト
            "all-key-bindings" => Ok(Value::List(
                self.keymap
                    .lock_recover()
                    .bindings()
                    .into_iter()
                    .map(|(keys, command)| {
                        Value::List(
                            vec![Value::String(keys), Value::String(command)]
                                .into_iter()
                                .collect(),
                        )
                    })
                    .collect(),
            )),
            // 最近のキー (古いものから)
            "recent-keys" => Ok(Value::List(
                self.state()
                    .recent_keys
                    .iter()
                    .cloned()
                    .map(Value::String)
                    .collect(),
            )),
            "editor-command-names" => Ok(Value::List(Vec::new().into())),
            "command" => Err(format!("Undefined command: {}", text(&args, 0)).into()),
            // 色, 下線, 隠すこと: 端末では何もしない
            "highlight-regexp"
            | "unhighlight-regexp"
            | "outline-flag-region"
            | "set-buffer-directory"
            | "set-background-color"
            | "set-foreground-color"
            | "set-font" => Ok(Value::Nil),
            "outline-invisible-p" => Ok(Value::Boolean(false)),
            other => Err(format!("The terminal does not support {other}").into()),
        }
    }
}

impl LoopUi for UiHost {
    fn say(&self, text: &str) {
        self.state().message = text.to_string();
    }

    fn stop_macro(&self) {
        self.state().stop_macro();
    }

    fn set_display(&self, truncate_lines: bool, selective_display: Option<usize>) {
        self.state().display = Display {
            truncate_lines,
            selective_display,
        };
    }
}

/// 選んでいる窓の, 本文の行の数 (モードラインは含まない).
fn body_height(editor: &Editor, state: &UiState) -> usize {
    let rows = state.screen.size().0.saturating_sub(1);
    let height = editor
        .layout()
        .rects(rseditor_core::window::FULL)
        .into_iter()
        .find(|r| r.id == editor.selected_window())
        .map_or(rows, |r| (r.height / 100.0 * rows as f64).round() as usize);
    height.saturating_sub(1)
}
