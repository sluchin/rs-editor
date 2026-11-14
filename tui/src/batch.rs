//! バッチモード (`rseditor-tui --batch`): Emacs の `emacs --batch` と同じく, 画面を使わずに, 引数の順に動かす.
//!
//! 引数は左から順に処理する.
//!
//! - `-l FILE`, `--load FILE`, `--script FILE`: ファイルを読む.
//! - `--eval EXPR`, `--eval=EXPR`: 式を評価する.
//! - `-f FUNC`, `--funcall FUNC`: 引数の無い関数 (コマンド) を呼ぶ.
//! - `FILE`: ファイルを訪れる (`find-file`).
//!
//! `princ` などは標準出力に, `message` は標準エラー出力に書く. `(kill-emacs N)` で終了コード N (既定は 0),
//! 捕まえなかったエラーは標準エラー出力に出して終了コード 255 で終わる.

use rseditor_core::error::Error;
use std::collections::VecDeque;
use std::io::{BufRead, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, Weak};

use crate::history::{HistoryArgs, Navigator};
use rseditor_core::command_loop::{read_key_name, CommandLoop, LoopUi};
use rseditor_core::editor::Editor;
use rseditor_core::keymap::{Keymap, Lookup};
use rseditor_core::scheme::editor_host::EditorHost;
use rseditor_core::scheme::evaluator::Evaluator;
use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::value::Value;
use rseditor_core::sync::LockExt;

/// 捕まえなかったエラーで終わるときの終了コード (Emacs と同じ).
pub const ERROR_STATUS: i32 = 255;

/// バッチモードの入出力.
pub struct Io {
    /// 標準出力 (`princ` など).
    pub out: Box<dyn Write + Send>,
    /// 標準エラー出力 (`message` とエラー).
    pub err: Box<dyn Write + Send>,
    /// 標準入力 (`read-string` など).
    pub input: Box<dyn BufRead + Send>,
}

/// 画面の代わりに, 標準入出力で答えるもの.
struct BatchUi {
    /// 標準出力 (ベル).
    out: Mutex<Box<dyn Write + Send>>,
    /// 標準エラー出力.
    err: Mutex<Box<dyn Write + Send>>,
    /// 標準入力.
    input: Mutex<Box<dyn BufRead + Send>>,
    /// 標準出力に書いた後で, まだ改行を出していないか (Emacs は, 次の `message` の前に, 標準エラー出力に改行を出す).
    need_newline: Arc<AtomicBool>,
    /// `kill-emacs` の終了コード.
    exit: Arc<Mutex<Option<i32>>>,
    /// 打ったことにするキー (`execute-kbd-macro`. `read-key` などが先に読む).
    keys: Mutex<VecDeque<String>>,
    /// キーのコマンドが失敗したか (失敗したら後のキーは捨てる).
    stopped: AtomicBool,
    /// キーのコマンドが失敗したときの, エラーの文.
    error: Mutex<Option<String>>,
    /// キーを処理するコマンドループ.
    commands: Mutex<Weak<CommandLoop<BatchUi>>>,
    /// 最後に失敗したマクロの, 終えた繰り返しの数とエラーの文 (捕まえなかったエラーに, 前置きを付けるため).
    failure: Arc<Mutex<Option<(usize, String)>>>,
}

impl LoopUi for BatchUi {
    fn say(&self, text: &str) {
        // 失敗したコマンドのエラーの文だけ覚える (他のエコー領域の表示は, バッチモードでは要らない)
        if self.stopped.load(Ordering::Relaxed) && !text.is_empty() {
            self.error
                .lock_recover()
                .get_or_insert_with(|| text.to_string());
        }
    }

    fn stop_macro(&self) {
        self.stopped.store(true, Ordering::Relaxed);
        self.keys.lock_recover().clear();
    }

    fn set_display(&self, _truncate_lines: bool, _selective_display: Option<usize>) {}
}

/// `execute-kbd-macro` の引数 (キーの名前の並び, または文字列) を, キーの名前の並びにする.
fn key_names(keys: Option<&Value>) -> Vec<String> {
    match keys {
        Some(Value::List(items)) => items
            .iter()
            .filter_map(|item| match item {
                Value::String(key) => Some(key.clone()),
                _ => None,
            })
            .collect(),
        Some(Value::String(text)) => text.chars().map(char_key).collect(),
        _ => Vec::new(),
    }
}

/// 1 文字を, キーの名前にする (制御文字は `C-a` など).
fn char_key(c: char) -> String {
    match c {
        '\r' => "RET".to_string(),
        '\t' => "TAB".to_string(),
        '\u{1b}' => "ESC".to_string(),
        '\u{7f}' => "DEL".to_string(),
        ' ' => "SPC".to_string(),
        c if (c as u32) < 32 => format!("C-{}", char::from(c as u8 + 96)),
        c => c.to_string(),
    }
}

impl BatchUi {
    /// 標準入力から読む前に, プロンプトを標準出力に出す (Emacs と同じ).
    fn show_prompt(&self, args: &[Value]) {
        if let Some(Value::String(prompt)) = args.first() {
            if !prompt.is_empty() {
                self.need_newline.store(true, Ordering::Relaxed);
                let mut out = self.out.lock_recover();
                let _ = out.write_all(prompt.as_bytes());
                let _ = out.flush();
            }
        }
    }

    /// 打ったことにするキーを, 改行 (`RET`) まで読んで文字列にする. キーが無ければ `None`.
    /// 始めの文字 `initial` の後ろに足す. `M-p`, `M-n` は, 履歴 `history` をたどる (画面のミニバッファと同じ).
    fn read_typed_line(&self, initial: &str, history: &HistoryArgs) -> Option<String> {
        let mut keys = self.keys.lock_recover();
        keys.front()?;
        let items = history.items.clone().unwrap_or_default();
        let mut navigator = Navigator::starting_at(history.start, initial, &items);
        let mut line = initial.to_string();
        while let Some(key) = keys.pop_front() {
            match key.as_str() {
                "RET" => break,
                "SPC" => line.push(' '),
                "DEL" => {
                    line.pop();
                }
                "M-p" | "M-n" => {
                    let found = if key == "M-p" {
                        navigator.previous(&line, &items, &history.defaults)
                    } else {
                        navigator.next(&line, &items, &history.defaults)
                    };
                    if let Ok(found) = found {
                        line = found;
                    }
                }
                other if other.chars().count() == 1 => line.push_str(other),
                _ => {}
            }
        }
        Some(line)
    }

    /// キーを打ったことにして, そのコマンドを動かす (`COUNT` 回. 0 なら失敗するまで).
    fn execute_keys(&self, keys: &[String], count: usize) -> Result<(), Error> {
        let commands = self
            .commands
            .lock_recover()
            .upgrade()
            .ok_or_else(|| "The batch mode has no command loop".to_string())?;
        self.stopped.store(false, Ordering::Relaxed);
        *self.error.lock_recover() = None;
        let before = commands.variable("executing-kbd-macro");
        commands
            .evaluator()
            .global()
            .define("executing-kbd-macro", Value::Boolean(true));
        commands.host().editor().executing_kbd_macro = true;
        // 入れ子のマクロが, 外側の残りのキーを読まないように, 自分のキーが尽きるまで
        let base = self.keys.lock_recover().len();
        let mut round = 0;
        while !keys.is_empty() && (count == 0 || round < count) {
            round += 1;
            {
                let mut queue = self.keys.lock_recover();
                for key in keys.iter().rev() {
                    queue.push_front(key.clone());
                }
            }
            while self.keys.lock_recover().len() > base {
                let key = self.keys.lock_recover().pop_front().unwrap_or_default();
                commands.handle_key(&key);
                if self.stopped.load(Ordering::Relaxed) {
                    break;
                }
            }
            if self.stopped.load(Ordering::Relaxed) {
                break;
            }
        }
        commands.host().editor().executing_kbd_macro = false;
        commands
            .evaluator()
            .global()
            .define("executing-kbd-macro", before);
        let error = self.error.lock_recover().take();
        if let Some(error) = &error {
            *self.failure.lock_recover() = Some((round.saturating_sub(1), error.clone()));
        }
        error.map_or(Ok(()), |e| Err(Error::plain(e)))
    }

    /// 標準入力から 1 行読む (改行は除く).
    fn read_line(&self) -> Result<String, Error> {
        let mut line = String::new();
        match self.input.lock_recover().read_line(&mut line) {
            Ok(0) | Err(_) => Err(Error::strings("end-of-file", &["Error reading from stdin"])),
            Ok(_) => Ok(line.trim_end_matches(['\n', '\r']).to_string()),
        }
    }
}

impl Host for BatchUi {
    fn call(&self, op: &str, args: Vec<Value>) -> Result<Value, Error> {
        match op {
            // バッチには描く画面が無く, `C-g` も押されない
            "redisplay" => Ok(Value::Nil),
            "quit-requested-p" => Ok(Value::Boolean(false)),
            "message" => {
                if let Some(Value::String(text)) = args.first() {
                    let mut err = self.err.lock_recover();
                    if self.need_newline.swap(false, Ordering::Relaxed) {
                        let _ = writeln!(err);
                    }
                    let _ = writeln!(err, "{text}");
                    let _ = err.flush();
                }
                Ok(Value::Nil)
            }
            "read-string" | "read-file-name" | "completing-read" => match self.read_typed_line(
                // 始めの文字は, read-string だけ (read-file-name の 2 つ目はディレクトリ)
                &match (op, args.get(1)) {
                    ("read-string", Some(Value::String(initial))) => initial.clone(),
                    _ => String::new(),
                },
                &match op {
                    "completing-read" => HistoryArgs::default(),
                    _ => HistoryArgs::from_args(&args, 2),
                },
            ) {
                Some(line) => Ok(Value::String(line)),
                None => {
                    self.show_prompt(&args);
                    self.read_line().map(Value::String)
                }
            },
            "read-key" => {
                let typed = self.keys.lock_recover().pop_front();
                match typed {
                    Some(key) => Ok(Value::String(read_key_name(&key))),
                    None => {
                        self.show_prompt(&args);
                        self.read_line().map(|line| {
                            Value::String(line.chars().next().map(String::from).unwrap_or_default())
                        })
                    }
                }
            }
            "execute-kbd-macro" => {
                let count = match args.get(1) {
                    Some(Value::Number(n)) => *n as usize,
                    _ => 1,
                };
                self.execute_keys(&key_names(args.first()), count)
                    .map(|()| Value::Nil)
            }
            "ding" => {
                // Emacs のバッチモードは, ベルの文字を標準出力に出す
                self.need_newline.store(true, Ordering::Relaxed);
                let mut out = self.out.lock_recover();
                let _ = out.write_all(b"\x07");
                let _ = out.flush();
                Ok(Value::Nil)
            }
            "kill-emacs" => {
                let code = match args.first() {
                    Some(Value::Number(n)) => *n as i32,
                    _ => 0,
                };
                *self.exit.lock_recover() = Some(code);
                // 後ろを評価しないために, 失敗にする (呼んだ側が, 終了コードを見分ける)
                Err("kill-emacs".to_string().into())
            }
            "editor-command-names" => Ok(Value::List(Vec::new().into())),
            "key-binding" => {
                // キーの並びの割り当て: コマンドの名前, 続きがあれば #t, 無ければ #f
                let commands = self
                    .commands
                    .lock_recover()
                    .upgrade()
                    .ok_or_else(|| "The batch mode has no command loop".to_string())?;
                let keys: Vec<String> = crate::ui::text(&args, 0)
                    .split(' ')
                    .map(str::to_string)
                    .collect();
                let local =
                    rseditor_core::keymap::buffer_keymap(commands.host().editor().current_buffer());
                let found = rseditor_core::keymap::lookup_in(
                    &local,
                    &commands.keymap().lock_recover(),
                    &keys,
                );
                Ok(match found {
                    Lookup::Command(command) => Value::String(command),
                    Lookup::Prefix => Value::Boolean(true),
                    Lookup::Unbound => Value::Boolean(false),
                })
            }
            // 色, 下線, 隠すこと: バッチモードでは何もしない
            "highlight-regexp"
            | "unhighlight-regexp"
            | "outline-flag-region"
            | "set-buffer-directory"
            | "set-background-color"
            | "set-foreground-color"
            | "set-font" => Ok(Value::Nil),
            "outline-invisible-p" => Ok(Value::Boolean(false)),
            other => Err(format!("The batch mode does not support {other}").into()),
        }
    }
}

/// 引数を 1 つ処理する.
enum Step {
    /// 評価する Scheme.
    Eval(String),
    /// 読むファイル.
    Load(String),
}

/// 引数を, 動かす順の手順にする.
///
/// # Errors
///
/// 知らない引数のとき, 引数が足りないとき.
fn parse(args: &[String]) -> Result<Vec<Step>, String> {
    let mut steps = Vec::new();
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        let mut value = |option: &str| {
            args.next()
                .cloned()
                .ok_or_else(|| format!("Option `{option}' requires an argument"))
        };
        match arg.as_str() {
            "--batch" | "-batch" => {}
            "-l" | "--load" | "--script" => steps.push(Step::Load(value(arg)?)),
            "--eval" | "-eval" | "--execute" | "-execute" => steps.push(Step::Eval(value(arg)?)),
            "-f" | "--funcall" | "-funcall" => {
                steps.push(Step::Eval(format!("({})", value(arg)?)));
            }
            _ => {
                if let Some(expr) = arg.strip_prefix("--eval=") {
                    steps.push(Step::Eval(expr.to_string()));
                } else if let Some(file) = arg.strip_prefix("--load=") {
                    steps.push(Step::Load(file.to_string()));
                } else if let Some(func) = arg.strip_prefix("--funcall=") {
                    steps.push(Step::Eval(format!("({func})")));
                } else if arg.starts_with('-') && arg.len() > 1 {
                    return Err(format!("Unknown option `{arg}'"));
                } else {
                    steps.push(Step::Eval(format!(
                        "(find-file {})",
                        Value::String(arg.clone())
                    )));
                }
            }
        }
    }
    Ok(steps)
}

/// バッチモードで動かします.
///
/// # Arguments
///
/// * `args` - コマンドラインの引数 (実行ファイルの名前は除く).
/// * `io` - 標準入出力.
///
/// # Returns
///
/// 終了コード.
pub fn run(args: &[String], io: Io) -> i32 {
    let Io { out, err, input } = io;
    let out = Arc::new(Mutex::new(out));
    let err = Arc::new(Mutex::new(err));
    let exit = Arc::new(Mutex::new(None));
    let need_newline = Arc::new(AtomicBool::new(false));
    let failure = Arc::new(Mutex::new(None));
    let fail = |text: &str| {
        let _ = writeln!(err.lock_recover(), "{text}");
        ERROR_STATUS
    };
    let steps = match parse(args) {
        Ok(steps) => steps,
        Err(error) => return fail(&error),
    };
    let ui = BatchUi {
        out: Mutex::new(Box::new(Shared(out.clone()))),
        err: Mutex::new(Box::new(Shared(err.clone()))),
        input: Mutex::new(input),
        need_newline: need_newline.clone(),
        exit: exit.clone(),
        keys: Mutex::new(VecDeque::new()),
        stopped: AtomicBool::new(false),
        error: Mutex::new(None),
        commands: Mutex::new(Weak::new()),
        failure: failure.clone(),
    };
    let host = EditorHost::new(Editor::new(), ui);
    let writer = out.clone();
    let wrote = need_newline.clone();
    host.set_output(Box::new(move |text| {
        wrote.store(true, Ordering::Relaxed);
        let mut out = writer.lock_recover();
        let _ = out.write_all(text.as_bytes());
        let _ = out.flush();
    }));
    let evaluator = Evaluator::new();
    // 相対パスは, 起動したディレクトリから (Emacs の default-directory)
    if let Ok(dir) = std::env::current_dir() {
        evaluator.set_default_directory(dir);
    }
    let keymap = Arc::new(Mutex::new(Keymap::default()));
    let result = CommandLoop::new(host, evaluator, keymap)
        .map(Arc::new)
        .and_then(|commands| {
            *commands.host().ui().commands.lock_recover() = Arc::downgrade(&commands);
            commands.eval("(set! noninteractive #t)")?;
            steps.iter().try_for_each(|step| {
                let code = match step {
                    Step::Eval(code) => code.clone(),
                    Step::Load(file) => std::fs::read_to_string(file).map_err(|_| {
                        format!("Cannot open load file: No such file or directory, {file}")
                    })?,
                };
                commands.eval(&code).map(|_| ())
            })
        });
    let killed = *exit.lock_recover();
    match (killed, result) {
        (Some(code), _) => code,
        (None, Ok(())) => 0,
        (None, Err(error)) => {
            // マクロの中のコマンドの失敗を捕まえなかったときは, Emacs と同じく, 繰り返しの数を前置きにする
            let failed = failure.lock_recover().take();
            match failed {
                Some((done, text)) if text == error.message() => {
                    fail(&format!("After {done} kbd macro iterations: {error}"))
                }
                _ => fail(&error),
            }
        }
    }
}

/// 共有した出力先に書く.
struct Shared(Arc<Mutex<Box<dyn Write + Send>>>);

impl Write for Shared {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.lock_recover().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.lock_recover().flush()
    }
}
