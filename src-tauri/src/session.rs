//! 画面 (Tauri) のエディタ: 核のエディタとコマンドループを持ち, 画面からの入力 (キー, 文字, クリック) を
//! 順に処理して, 描く内容 ([`Frame`]) を画面に送ります.
//!
//! 問い (`read-string`, `completing-read`, `read-key` など) と, 画面だけの操作 (色, フォント, スクロール) は,
//! 画面に頼みます ([`Front`]). Tauri に依存する部分 (イベントとスレッド) は [`crate::commands`] に置きます.

use rseditor_core::error::Error;
use std::collections::VecDeque;
use std::path::Path;
use std::sync::mpsc::{Receiver, RecvTimeoutError};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

use rseditor_core::command_loop::{read_key_name, CommandLoop, LoopUi};
use rseditor_core::editor::Editor;
use rseditor_core::frame::{Frame, FrameBuilder};
use rseditor_core::keymap::{buffer_keymap, lookup_in, Keymap, Lookup};
use rseditor_core::scheme::editor_host::EditorHost;
use rseditor_core::scheme::evaluator::Evaluator;
use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::value::Value;
use rseditor_core::startup::Options;
use rseditor_core::sync::LockExt;
use rseditor_core::syntax::Dialect;
use serde::Deserialize;

/// *scratch* バッファの始めの本文 (Emacs と同じ).
pub const SCRATCH_TEXT: &str =
    ";; This buffer is for text that is not saved, and for Lisp evaluation.
;; To create a file, visit it with C-x C-f and enter text in its buffer.
";

/// メッセージを書き溜めるバッファの名前 (Emacs と同じ).
pub const MESSAGES_BUFFER: &str = "*Messages*";

/// 覚えておく最近のキーの数 (Emacs の `recent-keys` と同じ).
const RECENT_KEYS_MAX: usize = 300;

/// 画面: 描く内容を受け取り, 問いと画面だけの操作に答える.
pub trait Front: Send + Sync {
    /// 描く内容を送ります.
    fn frame(&self, frame: &Frame);

    /// 問いと画面だけの操作を頼み, 返事を待ちます.
    ///
    /// # Errors
    ///
    /// 画面が失敗を返したとき (`C-g` でやめたときは `Quit`).
    fn request(&self, op: &str, args: Vec<Value>) -> Result<Value, Error>;

    /// 画面で `C-g` を押したか (押していれば, 押したことを忘れて `true`).
    /// 入力の列で `C-g` が順番を待っている間も, キーボードマクロの繰り返しを止めるため.
    fn interrupted(&self) -> bool;
}

/// 画面からの入力.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum Input {
    /// キー (Emacs の書き方. `C-x`, `a`, `RET`).
    Key {
        /// キー.
        key: String,
    },
    /// 文字 (日本語の入力で確定した文字など. 1 回の入力として入れる).
    Text {
        /// 文字.
        text: String,
    },
    /// クリック: ウィンドウを選び, カーソルを動かす. ドラッグで選んだときは, 始めにマークを置く.
    Point {
        /// ウィンドウの番号.
        window: usize,
        /// 位置 (UTF-16).
        pos: usize,
        /// ドラッグを始めた位置 (UTF-16. クリックだけなら無し).
        #[serde(default)]
        anchor: Option<usize>,
    },
    /// コマンドを名前で動かす (`M-x` を経ない. サイドバーやメニューから).
    Command {
        /// コマンドの名前.
        name: String,
    },
    /// ファイルを開く (サイドバーから).
    FindFile {
        /// パス.
        path: String,
    },
    /// タブを選ぶ (タブの行から).
    SelectTab {
        /// タブの番号 (1 から).
        number: usize,
    },
    /// ウィンドウの見えている行の数 (`window-body-height`. ウィンドウの番号と行の数).
    Resize {
        /// ウィンドウごとの行の数.
        heights: Vec<(usize, usize)>,
    },
    /// 描く内容を, 本文の全体から送り直す (画面を読み直したとき).
    Redraw,
}

/// 画面の状態.
#[derive(Debug, Default)]
struct State {
    /// エコー領域の文字.
    message: String,
    /// 最近のキー (古いものから).
    recent_keys: VecDeque<String>,
    /// 打ったことにするキー (キーボードマクロ).
    macro_keys: VecDeque<String>,
    /// 失敗するまで繰り返すマクロのキー.
    macro_loop: Vec<String>,
    /// 今のキーが, キーボードマクロから来たものか.
    executing_macro: bool,
    /// ウィンドウごとの見えている行の数.
    heights: Vec<(usize, usize)>,
    /// 描く内容を作るもの.
    frames: FrameBuilder,
}

/// コマンドループが画面に頼むこと (エディタの核が受けない操作) を受けるもの.
pub struct SessionUi<F: Front> {
    /// 画面.
    front: Arc<F>,
    /// エディタ.
    editor: Arc<Mutex<Editor>>,
    /// キーの割り当て.
    keymap: Arc<Mutex<Keymap>>,
    /// 画面の状態.
    state: Mutex<State>,
}

/// 文字列の引数.
fn text(args: &[Value], i: usize) -> String {
    match args.get(i) {
        Some(Value::String(s)) | Some(Value::Symbol(s)) => s.clone(),
        _ => String::new(),
    }
}

/// 文字列のリスト.
fn strings(items: impl IntoIterator<Item = String>) -> Value {
    Value::List(items.into_iter().map(Value::String).collect())
}

impl<F: Front> SessionUi<F> {
    /// 状態を借ります.
    fn state(&self) -> MutexGuard<'_, State> {
        self.state.lock_recover()
    }

    /// 描く内容を作って画面に送ります.
    pub fn send_frame(&self) {
        let frame = {
            let mut editor = self.editor.lock_recover();
            let mut state = self.state();
            let message = state.message.clone();
            state.frames.build(&mut editor, &message)
        };
        self.front.frame(&frame);
    }

    /// エコー領域に出し, *Messages* にも書きます (空の文は, 書かない).
    fn show_message(&self, message: &str) {
        self.state().message = message.to_string();
        if !message.is_empty() {
            append_message(&mut self.editor.lock_recover(), message);
        }
    }

    /// 選んでいるウィンドウの, 見えている行の数 (知らなければ 20).
    fn body_height(&self) -> usize {
        let selected = self.editor.lock_recover().selected_window();
        self.state()
            .heights
            .iter()
            .find(|(w, _)| *w == selected)
            .map_or(20, |(_, h)| *h)
    }

    /// 選んでいるウィンドウの, 上から `row` 行目 (負なら下から. 省くと真ん中) の行の始めへ動きます.
    /// 一番上の行は, 画面に聞きます (`window-top-line`).
    fn move_to_window_line(&self, row: Option<&Value>) -> Result<Value, Error> {
        let height = self.body_height();
        let window = self.editor.lock_recover().selected_window();
        self.send_frame();
        let top = match self
            .front
            .request("window-top-line", vec![Value::Number(window as f64)])?
        {
            Value::Number(n) if n > 0.0 => n as usize,
            _ => 0,
        };
        let row = match row {
            Some(Value::Number(n)) if *n >= 0.0 => *n as usize,
            Some(Value::Number(n)) => height.saturating_sub((-n) as usize),
            _ => height / 2,
        }
        .min(height.saturating_sub(1));
        let mut editor = self.editor.lock_recover();
        let buffer = editor.current_buffer_mut();
        let last = buffer.line_count().saturating_sub(1);
        let line = (top + row).min(last);
        let current = buffer.line_of_position(buffer.point());
        buffer.forward_line(line as i64 - current as i64);
        Ok(Value::Number((line - top.min(line)) as f64))
    }

    /// キーボードマクロのキーを 1 つ取ります (繰り返すマクロは, 尽きたら始めから).
    /// `C-g` を押したことを忘れます.
    fn clear_interrupt(&self) {
        self.front.interrupted();
    }

    /// 打ったことにするキーが残っている間に `C-g` を押したら, マクロを止めて `None` を返します.
    fn pop_macro_key(&self) -> Option<String> {
        let mut state = self.state();
        if (!state.macro_keys.is_empty() || !state.macro_loop.is_empty())
            && self.front.interrupted()
        {
            state.macro_keys.clear();
            state.macro_loop.clear();
            state.message = "Quit".to_string();
            state.executing_macro = false;
            return None;
        }
        if state.macro_keys.is_empty() && !state.macro_loop.is_empty() {
            state.macro_keys = state.macro_loop.iter().cloned().collect();
        }
        let key = state.macro_keys.pop_front();
        state.executing_macro = key.is_some();
        key
    }
}

/// *Messages* の終わりに 1 行書きます (無ければ読み取り専用で作る).
pub fn append_message(editor: &mut Editor, message: &str) {
    let buffer = editor.get_buffer_create(MESSAGES_BUFFER);
    buffer.set_read_only(false);
    let point = buffer.point();
    let end = buffer.len();
    buffer.set_point(end);
    buffer.insert(&format!("{message}\n"));
    if point != end {
        buffer.set_point(point);
    }
    buffer.set_modified(false);
    buffer.set_read_only(true);
}

impl<F: Front> Host for SessionUi<F> {
    fn call(&self, op: &str, args: Vec<Value>) -> Result<Value, Error> {
        match op {
            "message" => {
                self.show_message(&text(&args, 0));
                Ok(Value::Nil)
            }
            // 問い: 今の本文を見せてから聞く
            "read-string" | "read-file-name" | "completing-read" => {
                self.send_frame();
                self.front.request(op, args)
            }
            // キーを 1 つ読む (マクロの中ならマクロのキー). C-g は, 許されていなければ, やめる
            "read-key" => {
                let allow_quit = text(&args, 1) == "allow-quit";
                let from_macro = if self.state().executing_macro {
                    self.pop_macro_key()
                } else {
                    None
                };
                let key = match from_macro {
                    Some(key) => key,
                    None => {
                        self.send_frame();
                        let key = text(&[self.front.request(op, args)?], 0);
                        self.note_key(&key);
                        self.editor.lock_recover().kbd_macro.record(&key);
                        key
                    }
                };
                if key == "C-g" && !allow_quit {
                    return Err(Error::bare("quit"));
                }
                Ok(Value::String(read_key_name(&key)))
            }
            "execute-kbd-macro" => {
                let keys: Vec<String> = match args.first() {
                    Some(Value::List(items)) => items
                        .iter()
                        .map(|v| text(std::slice::from_ref(v), 0))
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
            // 長い仕事の途中で, 画面を描き直し, `C-g` を押したか確かめる (押したことは忘れる)
            "redisplay" => {
                self.send_frame();
                Ok(Value::Nil)
            }
            "quit-requested-p" => Ok(Value::Boolean(self.front.interrupted())),
            "window-body-height" => Ok(Value::Number(self.body_height() as f64)),
            "move-to-window-line" => self.move_to_window_line(args.first()),
            "where-is-internal" => Ok(strings(
                self.keymap.lock_recover().keys_for(&text(&args, 0)),
            )),
            "key-binding" => {
                let keys: Vec<String> = text(&args, 0).split(' ').map(str::to_string).collect();
                let local = buffer_keymap(self.editor.lock_recover().current_buffer());
                Ok(
                    match lookup_in(&local, &self.keymap.lock_recover(), &keys) {
                        Lookup::Command(command) => Value::String(command),
                        Lookup::Prefix => Value::Boolean(true),
                        Lookup::Unbound => Value::Boolean(false),
                    },
                )
            }
            "all-key-bindings" => Ok(Value::List(
                self.keymap
                    .lock_recover()
                    .bindings()
                    .into_iter()
                    .map(|(keys, command)| strings([keys, command]))
                    .collect(),
            )),
            "recent-keys" => Ok(strings(self.state().recent_keys.iter().cloned())),
            "editor-command-names" => Ok(Value::List(Vec::new().into())),
            "command" => Err(format!("Undefined command: {}", text(&args, 0)).into()),
            // スクロール, 色, フォント, 隠すこと, 終わること: 画面に頼む
            _ => {
                self.send_frame();
                self.front.request(op, args)
            }
        }
    }
}

impl<F: Front> LoopUi for SessionUi<F> {
    fn say(&self, text: &str) {
        self.state().message = text.to_string();
    }

    fn stop_macro(&self) {
        let mut state = self.state();
        state.macro_keys.clear();
        state.macro_loop.clear();
    }

    fn set_display(&self, _truncate_lines: bool, _selective_display: Option<usize>) {}
}

impl<F: Front> SessionUi<F> {
    /// 画面から読んだキーを, 最近のキーに覚えます.
    fn note_key(&self, key: &str) {
        let mut state = self.state();
        state.recent_keys.push_back(key.to_string());
        if state.recent_keys.len() > RECENT_KEYS_MAX {
            state.recent_keys.pop_front();
        }
    }
}

/// 画面のエディタ.
pub struct Session<F: Front> {
    /// コマンドループ.
    commands: CommandLoop<SessionUi<F>>,
}

impl<F: Front> Session<F> {
    /// 作ります. `*scratch*` (始めの文) と `*Messages*` を作り, エディタの基本のコマンドを読む.
    ///
    /// # Errors
    ///
    /// 基本のコマンドを読めないとき.
    pub fn new(front: Arc<F>) -> Result<Self, Error> {
        let mut editor = Editor::new();
        let scratch = editor.current_buffer_mut();
        scratch.insert(SCRATCH_TEXT);
        scratch.set_modified(false);
        scratch.set_dialect(Dialect::Scheme);
        let messages = editor.get_buffer_create(MESSAGES_BUFFER);
        messages.set_read_only(true);
        let editor = Arc::new(Mutex::new(editor));
        let keymap = Arc::new(Mutex::new(Keymap::default()));
        let ui = SessionUi {
            front,
            editor: editor.clone(),
            keymap: keymap.clone(),
            state: Mutex::new(State::default()),
        };
        let host = EditorHost::shared(editor, ui);
        let commands = CommandLoop::new(host, Evaluator::new(), keymap)?;
        Ok(Session { commands })
    }

    /// エディタの窓口.
    pub fn host(&self) -> &EditorHost<SessionUi<F>> {
        self.commands.host()
    }

    /// コードを評価し, 失敗したらエコー領域に出します. 終わったら描く (初期設定ファイルを読むときなど).
    pub fn eval(&mut self, code: &str) -> Result<String, Error> {
        let result = self.commands.eval(code);
        if let Err(error) = &result {
            self.host().ui().show_message(error);
        }
        self.commands.sync_lisp_keys();
        self.host().ui().send_frame();
        result
    }

    /// 起動の引数を当てます: 始めのディレクトリ (`--chdir`), `user-emacs-directory` (`--init-directory`),
    /// `init-file-debug` (`--debug-init`). 初期設定ファイルを読む前に呼びます.
    pub fn apply_startup(&mut self, options: &Options) {
        let global = self.commands.evaluator().global();
        if let Some(dir) = &options.chdir {
            self.commands
                .evaluator()
                .set_default_directory(std::path::PathBuf::from(dir));
        }
        if let Some(dir) = &options.init_directory {
            let dir = if dir.ends_with('/') {
                dir.clone()
            } else {
                format!("{dir}/")
            };
            global.define("user-emacs-directory", Value::String(dir));
        }
        global.define("init-file-debug", Value::Boolean(options.debug_init));
    }

    /// 起動の引数のファイルを開きます (最後のファイルが, 出ているバッファになる. Emacs と同じ).
    pub fn open_files(&mut self, files: &[String]) {
        for file in files {
            let _ = self.eval(&format!("(find-file {})", Value::String(file.clone())));
        }
    }

    /// 初期設定ファイルを読みます (`user-init-file` にする). 無ければ何もしません.
    /// 失敗したら, エコー領域と `*Warnings*` に出します (Emacs と同じ).
    pub fn load_init(&mut self, path: Option<&Path>) {
        let Some(path) = path.filter(|p| p.exists()) else {
            return;
        };
        log::info!("Loading {}", path.display());
        let debug = self.commands.evaluator().global().get("init-file-debug");
        let options_debug = debug.is_some_and(|value| value.is_non_nil());
        self.commands.evaluator().global().define(
            "user-init-file",
            Value::String(path.to_string_lossy().into_owned()),
        );
        let outcome = std::fs::read_to_string(path)
            .map_err(|e| e.to_string())
            .and_then(|code| {
                self.commands
                    .eval(&code)
                    .map_err(|e| init_error_text(&e, options_debug))
            });
        if let Err(error) = outcome {
            log::warn!("Could not load {}: {}", path.display(), error);
            let error = format!("{}: {}", path.display(), error);
            let warning = format!("An error occurred while loading the init file: {error}");
            let _ = self.commands.eval(&format!(
                "(display-warning 'initialization {} 'error)",
                Value::String(warning)
            ));
            self.host().ui().show_message(&error);
        }
        self.commands.sync_lisp_keys();
    }

    /// 書き換えたファイルのバッファを, 自動保存のファイル (`#名前#`) に書きます (Emacs の `do-auto-save`).
    /// 書いたら, "Auto-saving...done" と出して描きます.
    pub fn auto_save(&mut self) {
        let saved = rseditor_core::files::auto_save(&mut self.host().editor());
        if saved > 0 {
            self.host().ui().show_message("Auto-saving...done");
            self.host().ui().send_frame();
        }
    }

    /// ウィンドウを選び, カーソルを動かします (マウスで押した所. `anchor` が違えばそこから選ぶ).
    fn point(&mut self, window: usize, pos: usize, anchor: Option<usize>) {
        let mut editor = self.host().editor();
        if editor.select_window(window).is_ok() {
            let buffer = editor.current_buffer_mut();
            let pos = buffer.char_position(pos);
            match anchor.map(|a| buffer.char_position(a)) {
                Some(anchor) if anchor != pos => buffer.push_mark(anchor, true),
                _ => buffer.deactivate_mark(),
            }
            buffer.set_point(pos);
        }
    }

    /// 入力を 1 つ処理して描く内容を送ります.
    pub fn handle(&mut self, input: Input) {
        match input {
            Input::Key { key } => {
                self.host().ui().note_key(&key);
                self.key(&key);
                // キーボードマクロのキーが残っていれば, 続けて処理する
                while let Some(key) = self.host().ui().pop_macro_key() {
                    self.key(&key);
                }
            }
            Input::Text { text } => self.commands.self_insert(&text),
            // 1 行に書く (行ごとのカバレッジで, パターンの行を数えないように)
            #[rustfmt::skip]
            Input::Point { window, pos, anchor } => self.point(window, pos, anchor),
            Input::Command { name } => self.commands.execute(&name),
            Input::FindFile { path } => {
                let code = format!("(find-file {})", Value::String(path));
                let _ = self
                    .commands
                    .eval(&code)
                    .map_err(|e| self.host().ui().show_message(&e));
            }
            Input::SelectTab { number } => {
                let _ = self
                    .commands
                    .eval(&format!("(tab-bar-select-tab {number})"))
                    .map_err(|e| self.host().ui().show_message(&e));
            }
            Input::Resize { heights } => self.host().ui().state().heights = heights,
            Input::Redraw => self.host().ui().state().frames.reset(),
        }
        self.host().ui().send_frame();
    }

    /// キーを 1 つ処理します (キーボードマクロを覚えているなら, 覚える).
    fn key(&mut self, key: &str) {
        // `C-g` の順番が来たので, 押したことを忘れる (マクロを止めたあとも残さない)
        if key == "C-g" {
            self.host().ui().clear_interrupt();
        }
        let from_macro = self.host().ui().state().executing_macro;
        if self.commands.at_command_start() {
            self.host().editor().kbd_macro.begin_command();
        }
        if !from_macro {
            self.host().editor().kbd_macro.record(key);
        }
        self.commands.handle_key(key);
        // 画面から読んだキーが 300 個になったら, 自動保存する (Emacs の `auto-save-interval`)
        if !from_macro && rseditor_core::files::note_key(&mut self.host().editor()) {
            self.auto_save();
        }
    }
}

/// 入力が無いまま, 自動保存するまでの時間 (Emacs の `auto-save-timeout`).
pub const AUTO_SAVE_TIMEOUT: Duration = rseditor_core::files::AUTO_SAVE_TIMEOUT;

/// 画面のエディタを動かします: 作って, 起動の引数を当て, 初期設定ファイルを読み, 引数のファイルを開き, 描いてから入力を順に処理します.
/// 入力が `auto_save_timeout` の間無ければ自動保存します. 入力の送り手が無くなったら, 終わります.
///
/// # Errors
///
/// エディタを作れないとき.
pub fn run<F: Front>(
    front: Arc<F>,
    init_file: Option<&Path>,
    startup: &Options,
    inputs: Receiver<Input>,
    auto_save_timeout: Duration,
) -> Result<(), Error> {
    let mut session = Session::new(front)?;
    session.apply_startup(startup);
    session.load_init(init_file);
    session.open_files(&startup.files);
    session.host().ui().send_frame();
    loop {
        match inputs.recv_timeout(auto_save_timeout) {
            Ok(input) => session.handle(input),
            Err(RecvTimeoutError::Timeout) => session.auto_save(),
            Err(RecvTimeoutError::Disconnected) => return Ok(()),
        }
    }
}

/// 初期設定ファイルの失敗を, 文にします. `--debug-init` (`debug` が真) なら, 失敗した式を内側から並べます
/// (Emacs の `debug-on-error` の, バックトレースに当たる).
///
/// # Arguments
///
/// * `error` - 失敗.
/// * `debug` - 呼び出しの跡を付けるか.
///
/// # Returns
///
/// 失敗の文 (跡があれば, 改行して `Backtrace:` と式を続ける).
pub fn init_error_text(error: &rseditor_core::error::Error, debug: bool) -> String {
    let mut text = error.to_string();
    if debug && !error.trace().is_empty() {
        text.push_str("\nBacktrace:");
        for form in error.trace() {
            text.push_str("\n  ");
            text.push_str(form);
        }
    }
    text
}
