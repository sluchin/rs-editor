//! テストで共有する, 偽のエディタ (バッファを名前で持ち, Lisp のエディタの操作に答える).

#![allow(dead_code)]

use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;

use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::value::Value;

/// 偽のエディタのバッファ.
#[derive(Default, Clone)]
pub struct FakeBuffer {
    /// 本文 (1 文字ずつ).
    pub text: Vec<char>,
    /// カーソルの位置 (1 から数える).
    pub point: usize,
    /// ファイルのパス.
    pub file: Option<String>,
    /// 書き換えたら `true`.
    pub modified: bool,
    /// バッファだけのキー (キーとコマンド).
    pub keys: HashMap<String, String>,
    /// 下線を引く正規表現.
    pub highlights: Vec<String>,
    /// 色付けの face と括弧の番号 (正規表現ごと).
    pub faces: HashMap<String, (Option<String>, Option<usize>)>,
    /// 隠した範囲 (始めと終わり. 1 から数える).
    pub folded: Vec<(usize, usize)>,
    /// 訪れてから, バックアップを作ったか.
    pub backed_up: bool,
}

/// 偽のエディタ (バッファを名前で持つ).
#[derive(Default)]
pub struct FakeEditor {
    /// バッファ (名前ごと).
    pub buffers: Mutex<HashMap<String, FakeBuffer>>,
    /// 出しているバッファの名前.
    pub current: Mutex<String>,
    /// 最後のメッセージ.
    pub message: Mutex<String>,
    /// read-string, read-file-name, read-key と completing-read の答え (前から使う).
    pub answers: Mutex<Vec<String>>,
    /// read-string, read-file-name, read-key と completing-read で出した問い.
    pub prompts: Mutex<Vec<String>>,
    /// 最後の completing-read の候補.
    pub completions: Mutex<Vec<String>>,
    /// kill ring (kill-new で入れたもの).
    pub kill_ring: Mutex<Vec<String>>,
    /// display-buffer で出したもの (バッファの名前と行).
    pub displayed: Mutex<Vec<(String, Option<usize>)>>,
    /// command で動かしたコマンドの名前.
    pub commands: Mutex<Vec<String>>,
    /// 訪れてから、ファイルが外で書き換わったことにするか (`verify-visited-file-modtime` が偽を返す).
    pub file_changed: Mutex<bool>,
    /// push-mark で置いたマークの位置.
    pub marks: Mutex<Vec<usize>>,
}

impl FakeEditor {
    /// *scratch* だけの, 偽のエディタを作る.
    ///
    /// # Returns
    ///
    /// 偽のエディタ.
    pub fn new() -> Self {
        let editor = FakeEditor::default();
        editor.buffers.lock().unwrap().insert(
            "*scratch*".to_string(),
            FakeBuffer {
                point: 1,
                ..Default::default()
            },
        );
        *editor.current.lock().unwrap() = "*scratch*".to_string();
        editor
    }

    /// 出しているバッファを返す.
    ///
    /// # Returns
    ///
    /// バッファの複製.
    pub fn buffer(&self) -> FakeBuffer {
        let current = self.current.lock().unwrap().clone();
        self.buffers.lock().unwrap()[&current].clone()
    }

    /// 出しているバッファの名前を返す.
    ///
    /// # Returns
    ///
    /// 名前.
    pub fn name(&self) -> String {
        self.current.lock().unwrap().clone()
    }

    /// 出しているバッファの本文を返す.
    ///
    /// # Returns
    ///
    /// 本文.
    pub fn text(&self) -> String {
        self.buffer().text.iter().collect()
    }

    /// 出しているバッファの, カーソルの行を返す.
    ///
    /// # Returns
    ///
    /// 行.
    pub fn current_line(&self) -> String {
        let buffer = self.buffer();
        let before: String = buffer.text[..buffer.point - 1].iter().collect();
        let after: String = buffer.text[buffer.point - 1..].iter().collect();
        let start = before.rsplit('\n').next().unwrap().to_string();
        start + after.split('\n').next().unwrap()
    }

    /// 最後のメッセージを返す.
    ///
    /// # Returns
    ///
    /// メッセージ.
    pub fn message(&self) -> String {
        self.message.lock().unwrap().clone()
    }

    /// カーソルを, 文字の始めに置く.
    ///
    /// # Arguments
    ///
    /// * `text` - 探す文字 (出しているバッファの中).
    pub fn move_to(&self, text: &str) {
        let body = self.text();
        let byte = body
            .find(text)
            .unwrap_or_else(|| panic!("{} in {}", text, body));
        let point = body[..byte].chars().count() + 1;
        self.with_buffer(|b| b.point = point);
    }

    /// 出しているバッファを書き換える.
    ///
    /// # Arguments
    ///
    /// * `f` - 書き換える関数.
    pub fn with_buffer<T>(&self, f: impl FnOnce(&mut FakeBuffer) -> T) -> T {
        let current = self.current.lock().unwrap().clone();
        let mut buffers = self.buffers.lock().unwrap();
        f(buffers.get_mut(&current).unwrap())
    }

    /// バッファを出す (無ければ, 作る).
    ///
    /// # Arguments
    ///
    /// * `name` - 名前.
    /// * `make` - 無いときに作るバッファ.
    pub fn show(&self, name: &str, make: impl FnOnce() -> FakeBuffer) {
        self.buffers
            .lock()
            .unwrap()
            .entry(name.to_string())
            .or_insert_with(make);
        *self.current.lock().unwrap() = name.to_string();
    }

    /// 名前のバッファ (省けば, 出しているバッファ) の名前を返す.
    ///
    /// # Arguments
    ///
    /// * `args` - 省ける, バッファの名前.
    ///
    /// # Returns
    ///
    /// 名前.
    pub fn named(&self, args: &[Value]) -> String {
        args.first().map_or_else(|| self.name(), text)
    }

    /// ファイルのバッファを作る (あれば, その名前を返す).
    ///
    /// # Arguments
    ///
    /// * `path` - パス.
    ///
    /// # Returns
    ///
    /// バッファの名前.
    pub fn visit(&self, path: &str) -> String {
        let mut buffers = self.buffers.lock().unwrap();
        if let Some((name, _)) = buffers
            .iter()
            .find(|(_, b)| b.file.as_deref() == Some(path))
        {
            return name.clone();
        }
        buffers.insert(
            path.to_string(),
            FakeBuffer {
                text: fs::read_to_string(path)
                    .unwrap_or_default()
                    .chars()
                    .collect(),
                point: 1,
                file: Some(path.to_string()),
                ..Default::default()
            },
        );
        path.to_string()
    }

    /// 今のバッファの名前を, ファイルのパスにする (偽のエディタでは, ファイルのバッファの名前はパス).
    ///
    /// # Arguments
    ///
    /// * `path` - パス.
    pub fn name_after_file(&self, path: &str) {
        let old = self.name();
        let mut buffers = self.buffers.lock().unwrap();
        if old != path && !buffers.contains_key(path) {
            let buffer = buffers.remove(&old).unwrap();
            buffers.insert(path.to_string(), buffer);
            *self.current.lock().unwrap() = path.to_string();
        }
    }

    /// カーソルを, 次か前の行の始めに動かす.
    ///
    /// # Arguments
    ///
    /// * `down` - 次の行なら `true`.
    pub fn move_line(&self, down: bool) {
        self.with_buffer(|b| {
            let starts: Vec<usize> = std::iter::once(1)
                .chain(
                    b.text
                        .iter()
                        .enumerate()
                        .filter(|(_, c)| **c == '\n')
                        .map(|(i, _)| i + 2),
                )
                .collect();
            let line = starts.iter().rposition(|s| *s <= b.point).unwrap();
            let target = if down {
                (line + 1).min(starts.len() - 1)
            } else {
                line.saturating_sub(1)
            };
            b.point = starts[target];
        });
    }

    /// command で動かすコマンドの名前を覚えて, 返す.
    ///
    /// # Arguments
    ///
    /// * `name` - コマンドの名前.
    ///
    /// # Returns
    ///
    /// 名前.
    pub fn run_command(&self, name: &Value) -> String {
        let name = text(name);
        self.commands.lock().unwrap().push(name.clone());
        name
    }

    /// 出しているバッファの, キーのコマンドを返す.
    ///
    /// # Arguments
    ///
    /// * `key` - キー.
    ///
    /// # Returns
    ///
    /// コマンドの名前.
    pub fn key(&self, key: &str) -> Option<String> {
        self.buffer().keys.get(key).cloned()
    }
}

/// 位置の数を返す.
///
/// # Arguments
///
/// * `v` - 位置.
///
/// # Returns
///
/// 位置 (1 から).
pub fn pos(v: &Value) -> usize {
    match v {
        Value::Number(n) => *n as usize,
        _ => panic!("not a position: {}", v),
    }
}

/// 文字列を返す.
///
/// # Arguments
///
/// * `v` - 値.
///
/// # Returns
///
/// 文字列.
pub fn text(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        _ => panic!("not a string: {}", v),
    }
}

impl Host for FakeEditor {
    fn call(&self, op: &str, args: Vec<Value>) -> Result<Value, rseditor_core::error::Error> {
        Ok(match op {
            "point" => Value::Number(self.buffer().point as f64),
            "point-min" => Value::Number(1.0),
            "point-max" => Value::Number(self.buffer().text.len() as f64 + 1.0),
            "goto-char" => {
                let point = pos(&args[0]).clamp(1, self.buffer().text.len() + 1);
                self.with_buffer(|b| b.point = point);
                Value::Number(point as f64)
            }
            "insert" => {
                let inserted: Vec<char> =
                    args.iter().map(text).collect::<String>().chars().collect();
                self.with_buffer(|b| {
                    let at = b.point - 1;
                    b.text.splice(at..at, inserted.iter().copied());
                    b.point += inserted.len();
                    b.modified = true;
                });
                Value::Nil
            }
            "delete-region" => {
                let (from, to) = (pos(&args[0]), pos(&args[1]));
                self.with_buffer(|b| {
                    b.text.drain(from - 1..to - 1);
                    b.point = b.point.min(b.text.len() + 1);
                    b.modified = true;
                });
                Value::Nil
            }
            "buffer-string" => Value::String(self.text()),
            "buffer-substring" => {
                let (from, to) = (pos(&args[0]), pos(&args[1]));
                Value::String(self.buffer().text[from - 1..to - 1].iter().collect())
            }
            "buffer-name" | "current-buffer" => Value::String(self.name()),
            // with-current-buffer (find-file-hook と after-save-hook を, そのバッファで動かす)
            "set-buffer" => {
                let name = text(&args[0]);
                if !self.buffers.lock().unwrap().contains_key(&name) {
                    return Err(format!("No such buffer: {name}").into());
                }
                *self.current.lock().unwrap() = name;
                Value::Nil
            }
            "buffer-file-name" => {
                let name = self.named(&args);
                self.buffers.lock().unwrap()[&name]
                    .file
                    .clone()
                    .map_or(Value::Nil, Value::String)
            }
            "buffer-modified-p" => {
                let name = self.named(&args);
                Value::Boolean(self.buffers.lock().unwrap()[&name].modified)
            }
            "buffer-list" => {
                let mut names: Vec<String> = self.buffers.lock().unwrap().keys().cloned().collect();
                names.sort();
                Value::List(names.into_iter().map(Value::String).collect())
            }
            "kill-buffer" => {
                let name = self.named(&args);
                let mut buffers = self.buffers.lock().unwrap();
                if buffers[&name].file.is_some() && buffers[&name].modified {
                    return Err(format!("Buffer {} is modified", name).into());
                }
                buffers.remove(&name);
                let mut current = self.current.lock().unwrap();
                if *current == name {
                    *current = "*scratch*".to_string();
                    buffers
                        .entry(current.clone())
                        .or_insert_with(|| FakeBuffer {
                            point: 1,
                            ..Default::default()
                        });
                }
                Value::Boolean(true)
            }
            "kill-new" => {
                self.kill_ring.lock().unwrap().push(text(&args[0]));
                Value::Nil
            }
            "completing-read" => {
                let Value::List(candidates) = &args[1] else {
                    panic!("not a list: {}", args[1]);
                };
                *self.completions.lock().unwrap() = candidates.iter().map(text).collect();
                self.prompts.lock().unwrap().push(text(&args[0]));
                Value::String(self.answers.lock().unwrap().remove(0))
            }
            "display-buffer" => {
                let line = args.get(1).map(pos);
                self.displayed.lock().unwrap().push((text(&args[0]), line));
                Value::Nil
            }
            // ファイルを開く (find-file-noselect は Lisp. バッファを作り, 中身を読む)
            "generate-new-buffer" => {
                let base = text(&args[0]);
                let mut buffers = self.buffers.lock().unwrap();
                let name = (1..)
                    .map(|n| {
                        if n == 1 {
                            base.clone()
                        } else {
                            format!("{base}<{n}>")
                        }
                    })
                    .find(|name| !buffers.contains_key(name))
                    .unwrap();
                buffers.insert(
                    name.clone(),
                    FakeBuffer {
                        point: 1,
                        ..Default::default()
                    },
                );
                Value::String(name)
            }
            "insert-file-contents" => {
                // カーソルの後ろに入れる. VISIT なら, そのファイルを訪れる
                let path = text(&args[0]);
                let content = fs::read_to_string(&path).map_err(|e| format!("{path}: {e}"))?;
                let visit = args.get(1).is_some_and(Value::is_non_nil);
                self.with_buffer(|b| {
                    let at = b.point - 1;
                    b.text.splice(at..at, content.chars());
                    if visit {
                        b.file = Some(path.clone());
                        b.modified = false;
                    }
                });
                if visit {
                    self.name_after_file(&path);
                }
                Value::List(
                    vec![
                        Value::String(path),
                        Value::Number(content.chars().count() as f64),
                    ]
                    .into_iter()
                    .collect(),
                )
            }
            "files--set-buffer-file-name" => {
                let file = args.first().filter(|v| v.is_non_nil()).map(text);
                self.with_buffer(|b| b.file = file.clone());
                if let Some(file) = file {
                    self.name_after_file(&file);
                }
                Value::Nil
            }
            "verify-visited-file-modtime" => Value::Boolean(!*self.file_changed.lock().unwrap()),
            "set-visited-file-modtime" => {
                *self.file_changed.lock().unwrap() = false;
                Value::Nil
            }
            "set-syntax-dialect" | "set-major-mode-name" => Value::Nil,
            // 偽のエディタのファイルには、ファイルの変数が無い
            "file-local-mode" => Value::Boolean(false),
            "rename-buffer" => {
                // 同じ名前があれば, 番号を付ける
                let base = text(&args[0]);
                let old = self.name();
                let mut buffers = self.buffers.lock().unwrap();
                let buffer = buffers.remove(&old).unwrap();
                let name = (1..)
                    .map(|n| {
                        if n == 1 {
                            base.clone()
                        } else {
                            format!("{base}<{n}>")
                        }
                    })
                    .find(|name| !buffers.contains_key(name))
                    .unwrap();
                buffers.insert(name.clone(), buffer);
                *self.current.lock().unwrap() = name.clone();
                Value::String(name)
            }
            // 画面のキーとコマンド (goto-line だけ. find-file は Lisp と重なる)
            "where-is-internal" => Value::List(
                match text(&args[0]).as_str() {
                    "goto-line" => vec!["M-g g"],
                    "find-file" => vec!["C-x C-f"],
                    _ => vec![],
                }
                .into_iter()
                .map(|key| Value::String(key.to_string()))
                .collect(),
            ),
            "editor-command-names" => Value::List(
                ["goto-line", "find-file"]
                    .into_iter()
                    .map(|name| Value::String(name.to_string()))
                    .collect(),
            ),
            "highlight-regexp" => {
                let regexp = text(&args[0]);
                let face = match &args[1] {
                    Value::String(face) => Some(face.clone()),
                    _ => None,
                };
                let group = match &args[2] {
                    Value::Number(n) => Some(*n as usize),
                    _ => None,
                };
                self.with_buffer(|b| {
                    b.highlights.retain(|r| *r != regexp);
                    b.highlights.push(regexp.clone());
                    b.faces.insert(regexp, (face, group));
                });
                Value::Nil
            }
            "outline-flag-region" => {
                let (from, to) = (pos(&args[0]), pos(&args[1]));
                let (from, to) = (from.min(to), from.max(to));
                let hide = args[2] == Value::Boolean(true);
                self.with_buffer(|b| {
                    b.folded.retain(|(a, z)| !(*a < to && *z > from));
                    if hide && to > from {
                        b.folded.push((from, to));
                        b.folded.sort();
                    }
                });
                Value::Nil
            }
            "outline-invisible-p" => {
                let at = pos(&args[0]);
                Value::Boolean(
                    self.buffer()
                        .folded
                        .iter()
                        .any(|(a, z)| *a <= at && at < *z),
                )
            }
            "unhighlight-regexp" => {
                self.with_buffer(|b| match &args[0] {
                    Value::String(regexp) => {
                        b.highlights.retain(|r| r != regexp);
                        b.faces.remove(regexp);
                    }
                    _ => {
                        b.highlights.clear();
                        b.faces.clear();
                    }
                });
                Value::Nil
            }
            "message" => {
                *self.message.lock().unwrap() = text(&args[0]);
                Value::Nil
            }
            "pop-to-buffer-same-window" => {
                self.show(&text(&args[0]), || FakeBuffer {
                    point: 1,
                    ..Default::default()
                });
                Value::Nil
            }
            "set-buffer-modified-p" => {
                let modified = args[0].is_true();
                self.with_buffer(|b| b.modified = modified);
                Value::Nil
            }
            "local-set-key" => {
                let (key, command) = (text(&args[0]), text(&args[1]));
                self.with_buffer(|b| b.keys.insert(key, command));
                Value::Nil
            }
            // 保存する (basic-save-buffer は Lisp). 書くのは, バッファの全て
            "files--buffer-backed-up-p" => Value::Boolean(self.buffer().backed_up),
            "files--set-buffer-backed-up" => {
                let flag = args[0].is_non_nil();
                self.with_buffer(|b| b.backed_up = flag);
                Value::Nil
            }
            "write-region" => {
                let path = text(&args[2]);
                let content: String = self.buffer().text.iter().collect();
                fs::write(&path, content).map_err(|e| e.to_string())?;
                let visit = match args.get(4) {
                    Some(Value::String(file)) => Some(file.clone()),
                    Some(v) if v.is_non_nil() => Some(path.clone()),
                    _ => None,
                };
                if let Some(file) = visit {
                    self.with_buffer(|b| {
                        b.file = Some(file);
                        b.modified = false;
                    });
                }
                *self.message.lock().unwrap() = format!("Wrote {}", path);
                Value::Nil
            }
            "files--set-buffer-text" => {
                // 今のバッファの本文を置き換える (カーソルは, 本文の長さに収める)
                let (new, modified) = (text(&args[0]), args[1].is_true());
                self.with_buffer(|b| {
                    b.text = new.chars().collect();
                    b.point = b.point.min(b.text.len() + 1);
                    b.modified = modified;
                });
                Value::Nil
            }
            "read-string" | "read-file-name" | "read-key" => {
                self.prompts.lock().unwrap().push(text(&args[0]));
                Value::String(self.answers.lock().unwrap().remove(0))
            }
            "command" => match self.run_command(&args[0]).as_str() {
                "indent-for-tab-command" | "forward-word" | "backward-word" => Value::Boolean(true),
                "newline" => return self.call("insert", vec![Value::String("\n".to_string())]),
                "next-line" | "previous-line" => {
                    self.move_line(text(&args[0]) == "next-line");
                    Value::Boolean(true)
                }
                "list-buffers" => {
                    *self.message.lock().unwrap() = "list-buffers".to_string();
                    Value::Boolean(true)
                }
                other => return Err(format!("Undefined command: {}", other).into()),
            },
            // 偽のエディタは, マークと絞り込みを持たない (save-excursion は, 点を覚える)
            "point-marker" | "restriction-save" => {
                return Err(format!("{op} is not supported").into())
            }
            "widen" => Value::Nil,
            // 偽のエディタのウィンドウは, 専用にしない
            "window-dedicated-p" => Value::Boolean(false),
            "push-mark" => {
                let at = match args.first() {
                    Some(Value::Number(n)) => *n as usize,
                    _ => self.buffer().point,
                };
                self.marks.lock().unwrap().push(at);
                Value::Nil
            }
            _ => panic!("unknown op {}", op),
        })
    }
}
