//! コマンドループ: キーを引いて、コマンドを Lisp で動かす (Emacs の `command-loop` に当たる).
//!
//! 画面 (Tauri) とターミナルのアプリで共通です. 前置引数, `this-command`, `last-command`,
//! 元に戻す区切り (`undo-boundary`), 変更した後のマークの解除を受け持ちます.
//! 画面の違い (エコー領域, キーボードマクロ, 出し方の設定) は [`LoopUi`] で受けます.

use crate::error::Error;
use std::sync::{Arc, Mutex};

use crate::editor::prefix_numeric_value;
use crate::keymap::{from_codemirror, Keymap, Lookup};
use crate::scheme::editor_host::EditorHost;
use crate::scheme::evaluator::Evaluator;
use crate::scheme::host::Host;
use crate::scheme::value::Value;
use crate::sync::LockExt;

/// コマンドループが画面に頼むこと.
pub trait LoopUi {
    /// エコー領域に出します (空ならエコー領域を消す).
    fn say(&self, text: &str);

    /// 動かしているキーボードマクロを止めます (コマンドが失敗したとき).
    fn stop_macro(&self);

    /// 画面の出し方の設定 (Lisp の `truncate-lines` と `selective-display`) を知らせます. コマンドの後に呼ぶ.
    fn set_display(&self, truncate_lines: bool, selective_display: Option<usize>);
}

/// コマンドループ.
pub struct CommandLoop<U: Host + LoopUi> {
    /// エディタを動かす窓口.
    host: EditorHost<U>,
    /// 評価器.
    evaluator: Evaluator,
    /// キーの割り当て.
    keymap: Arc<Mutex<Keymap>>,
    /// 読んでいる途中のキー (`C-x` など).
    pending: Mutex<Vec<String>>,
}

/// コマンドの名前の記号.
fn symbol(name: &str) -> Value {
    Value::Symbol(name.to_string())
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

/// キーの名前を, `read-key` が返す名前 (Lisp のコマンドが決まりにしている、画面のキーの名前) にします.
pub fn read_key_name(key: &str) -> String {
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

impl<U: Host + LoopUi> CommandLoop<U> {
    /// エディタの窓口と、評価器と、キーの割り当てから作り, エディタの基本のコマンド (`lisp/simple.scm` など) を読みます.
    ///
    /// # Errors
    ///
    /// 基本のコマンドを読めないとき.
    pub fn new(
        host: EditorHost<U>,
        evaluator: Evaluator,
        keymap: Arc<Mutex<Keymap>>,
    ) -> Result<Self, Error> {
        evaluator.load_editor_files(&host)?;
        Ok(CommandLoop {
            host,
            evaluator,
            keymap,
            pending: Mutex::new(Vec::new()),
        })
    }

    /// エディタの窓口.
    pub fn host(&self) -> &EditorHost<U> {
        &self.host
    }

    /// 評価器.
    pub fn evaluator(&self) -> &Evaluator {
        &self.evaluator
    }

    /// キーの割り当て.
    pub fn keymap(&self) -> &Arc<Mutex<Keymap>> {
        &self.keymap
    }

    /// 続きのキーを待っていなければ `true` (新しいコマンドの始め).
    pub fn at_command_start(&self) -> bool {
        self.pending.lock_recover().is_empty()
    }

    /// Scheme を評価します (初期設定ファイルを読むときなど).
    ///
    /// # Errors
    ///
    /// 評価に失敗したとき.
    pub fn eval(&self, code: &str) -> Result<String, Error> {
        self.evaluator.eval_with(code, &self.host)
    }

    /// Lisp の変数を返します (無ければ偽).
    pub fn variable(&self, name: &str) -> Value {
        self.evaluator
            .global()
            .get(name)
            .unwrap_or(Value::Boolean(false))
    }

    /// エコー領域に出します.
    fn say(&self, text: &str) {
        self.host.ui().say(text);
    }

    /// 前置引数を読んでいる間 (`universal-argument-map`) のキーなら, 受けるコマンドの名前を返します.
    fn prefix_command(&self, key: &str) -> Option<&'static str> {
        if !self.pending.lock_recover().is_empty()
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

    /// Lisp の `truncate-lines` と `selective-display` を, 画面に知らせます.
    fn sync_display(&self) {
        let truncate_lines = !matches!(self.variable("truncate-lines"), Value::Boolean(false));
        let selective_display = match self.variable("selective-display") {
            Value::Number(n) if n >= 0.0 => Some(n as usize),
            _ => None,
        };
        self.host
            .ui()
            .set_display(truncate_lines, selective_display);
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

    /// Lisp で割り当てたキー (`global-set-key`) を, キーの割り当てに写します.
    pub fn sync_lisp_keys(&self) {
        let mut keymap = self.keymap.lock_recover();
        for binding in self.evaluator.key_bindings() {
            keymap.bind(&from_codemirror(&binding.key), &binding.command);
        }
    }

    /// キーを 1 つ処理します.
    pub fn handle_key(&self, key: &str) {
        if key == "C-g" && !self.pending.lock_recover().is_empty() {
            self.pending.lock_recover().clear();
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
        self.pending.lock_recover().push(key.to_string());
        // Lisp で割り当てたキーが先
        self.sync_lisp_keys();
        let lookup = self.lookup_pending();
        match lookup {
            Lookup::Prefix => {
                let shown = self.pending.lock_recover().join(" ");
                self.say(&format!("{shown}-"));
            }
            Lookup::Command(command) => {
                self.pending.lock_recover().clear();
                self.execute(&command);
            }
            Lookup::Unbound => {
                let keys = std::mem::take(&mut *self.pending.lock_recover());
                match (keys.as_slice(), printable(key)) {
                    ([_], Some(c)) => self.self_insert(&c.to_string()),
                    _ => self.say(&format!("{} is undefined", keys.join(" "))),
                }
            }
        }
    }

    /// 読んでいるキーを引きます. バッファだけのキー (`local-set-key`) を, 全体の割り当てより先に引く.
    fn lookup_pending(&self) -> Lookup {
        let local = self.host.editor().current_buffer().local_keys().to_vec();
        if !local.is_empty() {
            let mut keymap = Keymap::empty();
            for (keys, command) in &local {
                keymap.bind(keys, command);
            }
            let found = keymap.lookup(&self.pending.lock_recover());
            if found != Lookup::Unbound {
                return found;
            }
        }
        self.keymap
            .lock_recover()
            .lookup(&self.pending.lock_recover())
    }

    /// 文字を入れます (前置引数の数だけ繰り返す). 画面の IME で確定した文字も, ここから入れる.
    pub fn self_insert(&self, text: &str) {
        self.pending.lock_recover().clear();
        self.say("");
        self.take_prefix();
        let count = prefix_numeric_value(self.host.editor().prefix_arg).max(1);
        let text = text.repeat(count as usize);
        self.evaluator
            .global()
            .define("this-command", symbol("self-insert-command"));
        let outcome = self
            .host
            .call("insert", vec![Value::String(text)])
            .map(|_| ());
        self.finish("self-insert-command", outcome);
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

    /// コマンドを動かします (キーから, または `M-x` から).
    pub fn execute(&self, command: &str) {
        self.say("");
        self.take_prefix();
        self.host
            .editor()
            .current_buffer_mut()
            .set_deactivate_requested(false);
        let globals = self.evaluator.global();
        globals.define("this-command", symbol(command));
        // キーから動かすコマンドは, 対話的 (called-interactively-p)
        globals.define("simple--interactive", Value::Boolean(true));
        let outcome = self.evaluator.run_command(command, &self.host).map(|_| ());
        self.evaluator
            .global()
            .define("simple--interactive", Value::Boolean(false));
        self.finish(command, outcome);
    }

    /// コマンドを終えた後の処理: 失敗を知らせ、元に戻す区切りを入れ,前のコマンドを覚え、マークを外す.
    fn finish(&self, command: &str, outcome: Result<(), Error>) {
        if let Err(error) = outcome {
            // マクロは, 失敗したところで止める
            self.host.ui().stop_macro();
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
        self.sync_display();
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
