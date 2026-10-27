//! ターミナルのエディタ: キーを受けて、コマンドを動かし、画面を描く.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use rseditor_core::editor::Editor;
use rseditor_core::scheme::editor_host::EditorHost;
use rseditor_core::scheme::evaluator::Evaluator;
use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::value::Value;

use crate::keymap::{from_codemirror, Keymap, Lookup};
use crate::render::render;
use crate::screen::Screen;
use crate::ui::{printable, UiHost, UiState};

/// 基本の編集のコマンド (Lisp).
const SIMPLE: &str = include_str!("../../lisp/simple.scm");

/// ターミナルのエディタ.
pub struct App {
    /// エディタを動かす窓口.
    host: EditorHost<UiHost>,
    /// 評価器.
    evaluator: Evaluator,
    /// キーの割り当て.
    keymap: Arc<Mutex<Keymap>>,
    /// 画面とその状態.
    state: Arc<Mutex<UiState>>,
    /// 読んでいる途中のキー (`C-x` など).
    pending: Vec<String>,
}

/// コマンドの名前の記号.
fn symbol(name: &str) -> Value {
    Value::Symbol(name.to_string())
}

impl App {
    /// エディタを作ります. 基本の編集のコマンド (`lisp/simple.scm`) を読み、ファイルを開く.
    ///
    /// # Arguments
    ///
    /// * `screen` - 画面.
    /// * `files` - 開くファイル.
    ///
    /// # Errors
    ///
    /// 基本のコマンドを読めないとき.
    pub fn new(screen: Box<dyn Screen>, files: &[String]) -> Result<Self, String> {
        let editor = Arc::new(Mutex::new(Editor::new()));
        let keymap = Arc::new(Mutex::new(Keymap::default()));
        let state = Arc::new(Mutex::new(UiState {
            screen,
            message: String::new(),
            tops: BTreeMap::new(),
            quit: false,
        }));
        let ui = UiHost::new(state.clone(), editor.clone(), keymap.clone());
        let host = EditorHost::shared(editor, ui);
        let evaluator = Evaluator::new();
        evaluator.eval_with(SIMPLE, &host)?;
        let app = App {
            host,
            evaluator,
            keymap,
            state,
            pending: Vec::new(),
        };
        // 最後のファイルを出す (Emacs と同じ)
        for file in files {
            let name = app
                .host
                .call("find-file-noselect", vec![Value::String(file.clone())])?;
            app.host.call("pop-to-buffer-same-window", vec![name])?;
        }
        Ok(app)
    }

    /// Scheme を評価します (初期設定ファイルを読むときなど).
    ///
    /// # Errors
    ///
    /// 評価に失敗したとき.
    pub fn eval(&self, code: &str) -> Result<String, String> {
        self.evaluator.eval_with(code, &self.host)
    }

    /// エディタの状態.
    pub fn host(&self) -> &EditorHost<UiHost> {
        &self.host
    }

    /// 終わるか.
    pub fn quit(&self) -> bool {
        self.state.lock().unwrap().quit
    }

    /// エコー領域の文字.
    pub fn message(&self) -> String {
        self.state.lock().unwrap().message.clone()
    }

    /// 画面を描きます.
    pub fn draw(&self) {
        let editor = self.host.editor();
        let mut state = self.state.lock().unwrap();
        let (rows, cols) = state.screen.size();
        let message = state.message.clone();
        let frame = render(&editor, &mut state.tops, rows, cols, &message, None);
        state.screen.draw(&frame);
    }

    /// 画面を描いて、キーを読み、終わるまで繰り返します.
    pub fn run(&mut self) {
        loop {
            self.draw();
            let key = self.state.lock().unwrap().screen.read_key();
            match key {
                Some(key) => self.handle_key(&key),
                None => break,
            }
            if self.quit() {
                break;
            }
        }
    }

    /// エコー領域に出します.
    fn say(&self, text: &str) {
        self.state.lock().unwrap().message = text.to_string();
    }

    /// Lisp の変数を返します (無ければ偽).
    fn variable(&self, name: &str) -> Value {
        self.evaluator
            .global()
            .get(name)
            .unwrap_or(Value::Boolean(false))
    }

    /// 前置引数を読んでいる間 (`universal-argument-map`) のキーなら, 受けるコマンドの名前を返します.
    fn prefix_command(&self, key: &str) -> Option<&'static str> {
        if !self.pending.is_empty()
            || self.variable("universal-argument-mode") == Value::Boolean(false)
        {
            return None;
        }
        let last = key.chars().last().unwrap_or(' ');
        match key {
            "C-u" => Some("universal-argument-more"),
            "-" | "M--" | "C--" if !matches!(self.variable("prefix-arg"), Value::Number(_)) => {
                Some("negative-argument")
            }
            "M--" | "C--" => Some("negative-argument"),
            _ if last.is_ascii_digit() && (key.len() == 1 || key.starts_with(['M', 'C'])) => {
                Some("digit-argument")
            }
            _ => None,
        }
    }

    /// 前置引数をエコー領域に出します.
    fn say_prefix(&self) {
        let text = match self.variable("prefix-arg") {
            Value::Symbol(_) => "C-u -".to_string(),
            Value::Number(n) => format!("C-u {n}-"),
            Value::List(items) => format!("C-u {}-", items[0]),
            _ => return,
        };
        self.say(&text);
    }

    /// キーを 1 つ処理します.
    pub fn handle_key(&mut self, key: &str) {
        if key == "C-g" && !self.pending.is_empty() {
            self.pending.clear();
            self.say("Quit");
            return;
        }
        self.evaluator
            .global()
            .define("last-command-event", Value::String(key.to_string()));
        if let Some(command) = self.prefix_command(key) {
            self.execute(command);
            return;
        }
        self.pending.push(key.to_string());
        // Lisp で割り当てたキーが先
        {
            let mut keymap = self.keymap.lock().unwrap();
            for binding in self.evaluator.key_bindings() {
                keymap.bind(&from_codemirror(&binding.key), &binding.command);
            }
        }
        let lookup = self.keymap.lock().unwrap().lookup(&self.pending);
        match lookup {
            Lookup::Prefix => {
                let shown = self.pending.join(" ");
                self.say(&format!("{shown}-"));
            }
            Lookup::Command(command) => {
                self.pending.clear();
                self.execute(&command);
            }
            Lookup::Unbound => {
                let keys = std::mem::take(&mut self.pending);
                match (keys.as_slice(), printable(key)) {
                    ([_], Some(c)) => self.self_insert(c),
                    _ => self.say(&format!("{} is undefined", keys.join(" "))),
                }
            }
        }
    }

    /// 文字を入れます (前置引数の数だけ).
    fn self_insert(&mut self, c: char) {
        self.take_prefix();
        let count =
            rseditor_core::editor::prefix_numeric_value(self.host.editor().prefix_arg).max(1);
        let text: String = std::iter::repeat_n(c, count as usize).collect();
        self.evaluator
            .global()
            .define("this-command", symbol("self-insert-command"));
        self.finish(
            "self-insert-command",
            self.host
                .call("insert", vec![Value::String(text)])
                .map(|_| ()),
        );
    }

    /// Lisp の `prefix-arg` を, 次のコマンドの前置引数にします (`prefix-arg` は空にする).
    fn take_prefix(&self) {
        let arg = self.variable("prefix-arg");
        let globals = self.evaluator.global();
        globals.define("prefix-arg", Value::Boolean(false));
        globals.define("universal-argument-mode", Value::Boolean(false));
        // 前置引数は, 前置引数のコマンドが作ったものだけなので, 失敗しない
        let _ = self.host.call("set-current-prefix-arg", vec![arg]);
    }

    /// コマンドを動かします.
    fn execute(&mut self, command: &str) {
        self.say("");
        self.take_prefix();
        self.host
            .editor()
            .current_buffer_mut()
            .set_deactivate_requested(false);
        let globals = self.evaluator.global();
        globals.define("this-command", symbol(command));
        let outcome = self.evaluator.run_command(command, &self.host).map(|_| ());
        self.finish(command, outcome);
    }

    /// コマンドを終えた後の処理: 失敗を知らせ、元に戻す区切りを入れ,前のコマンドを覚え、マークを外す.
    fn finish(&mut self, command: &str, outcome: Result<(), String>) {
        if let Err(error) = outcome {
            self.say(&error);
        }
        // 区切りのための呼び出しは、失敗しない
        let _ = self.host.call("undo-boundary", vec![]);
        let globals = self.evaluator.global();
        let this = globals
            .get("this-command")
            .unwrap_or_else(|| symbol(command));
        globals.define(
            "last-command",
            if matches!(this, Value::Symbol(_)) {
                this
            } else {
                symbol(command)
            },
        );
        globals.define("this-command", symbol(command));
        self.host.editor().prefix_arg = None;
        // 前置引数のコマンドの後は, 次のコマンドに前置引数を渡す
        self.say_prefix();
        // 変更したコマンドの終わりには, マークを使わない状態にする (Emacs の deactivate-mark)
        let mut editor = self.host.editor();
        let buffer = editor.current_buffer_mut();
        if buffer.deactivate_requested() {
            buffer.deactivate_mark();
            buffer.set_deactivate_requested(false);
        }
    }
}
