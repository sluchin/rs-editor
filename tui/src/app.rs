//! ターミナルのエディタ: キーを受けて、コマンドを動かし、画面を描く.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use rseditor_core::editor::{Editor, PrefixArg};
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

/// 前置引数を読んでいる途中の状態.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Prefix {
    /// `C-u` を押した回数 (`C-u` は 4, `C-u C-u` は 16).
    Universal(i64),
    /// 数を読んでいる (符号と, 今までの数. 数が無ければ `None`).
    Digits(bool, Option<i64>),
}

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
    /// 前置引数を読んでいる途中の状態.
    prefix: Option<Prefix>,
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
            prefix: None,
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

    /// 前置引数を決めます.
    fn set_prefix(&self, prefix: Option<PrefixArg>) {
        self.host.editor().prefix_arg = prefix;
    }

    /// 前置引数のキーなら、受けて `true`.
    fn handle_prefix_key(&mut self, key: &str) -> bool {
        if !self.pending.is_empty() {
            return false;
        }
        let digit = key
            .strip_prefix("M-")
            .or(Some(key))
            .and_then(|k| k.parse::<i64>().ok())
            .filter(|_| {
                key.starts_with("M-")
                    || matches!(self.prefix, Some(Prefix::Universal(_) | Prefix::Digits(..)))
            });
        let next = match (key, self.prefix, digit) {
            ("C-u", Some(Prefix::Universal(n)), _) => Prefix::Universal(n * 4),
            ("C-u", _, _) => Prefix::Universal(4),
            ("M--" | "C--", None, _) => Prefix::Digits(true, None),
            ("-", Some(Prefix::Universal(_)), _) => Prefix::Digits(true, None),
            (_, Some(Prefix::Digits(negative, so_far)), Some(d)) => {
                Prefix::Digits(negative, Some(so_far.unwrap_or(0) * 10 + d))
            }
            (_, Some(Prefix::Universal(_)), Some(d)) => Prefix::Digits(false, Some(d)),
            (_, None, Some(d)) => Prefix::Digits(false, Some(d)),
            _ => return false,
        };
        self.prefix = Some(next);
        let arg = match next {
            Prefix::Universal(n) => PrefixArg::Universal(n),
            Prefix::Digits(true, None) => PrefixArg::Minus,
            Prefix::Digits(negative, so_far) => {
                let n = so_far.unwrap_or(0);
                PrefixArg::Number(if negative { -n } else { n })
            }
        };
        self.set_prefix(Some(arg));
        self.say(&match arg {
            PrefixArg::Universal(n) => format!("C-u {n}-"),
            PrefixArg::Minus => "C-u -".to_string(),
            PrefixArg::Number(n) => format!("C-u {n}-"),
        });
        true
    }

    /// キーを 1 つ処理します.
    pub fn handle_key(&mut self, key: &str) {
        if key == "C-g" && !self.pending.is_empty() {
            self.pending.clear();
            self.say("Quit");
            return;
        }
        if self.handle_prefix_key(key) {
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

    /// コマンドを動かします.
    fn execute(&mut self, command: &str) {
        self.say("");
        let globals = self.evaluator.global();
        globals.define("this-command", symbol(command));
        let outcome = self.evaluator.run_command(command, &self.host).map(|_| ());
        self.finish(command, outcome);
    }

    /// コマンドを終えた後の処理: 失敗を知らせ、元に戻す区切りを入れ,前のコマンドを覚え、前置引数を消す.
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
        self.prefix = None;
        self.set_prefix(None);
    }
}
