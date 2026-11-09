//! Lisp のエラー (Emacs の `signal` のデータ): 種類 (`wrong-type-argument` など) と, データ.
//!
//! `condition-case` は種類の `error-conditions` (種類と, その親) で受けるかを決め, `error-message-string` と
//! 捕まえなかったエラーの表示は, 種類の説明とデータから文を作る (Emacs の `print_error_message` と同じ).

use std::fmt;
use std::ops::Deref;

use crate::scheme::builtins::print_string;
use crate::scheme::value::Value;

/// Lisp のエラー.
///
/// 文字列のように扱えます (`Deref<Target = str>` と文字列との比較は, 表示する文に対して).
#[derive(Clone)]
pub struct Error {
    /// 種類 (`error`, `wrong-type-argument` など).
    kind: String,
    /// データ (`signal` の第 2 引数の要素).
    data: Vec<Value>,
    /// 受ける条件 (`error-conditions`. 種類が先).
    conditions: Vec<String>,
    /// 表示する文 (`error-message-string` と同じ).
    message: String,
    /// 失敗した式 (内側から外側へ. `init-file-debug` のときだけ, 評価器が足す).
    trace: Option<Box<Trace>>,
}

/// 失敗した式の並び (`Error` を小さく保つため, 箱に入れて持つ).
#[derive(Clone, Default)]
struct Trace {
    /// 式を表した文字列 (内側から外側へ).
    forms: Vec<String>,
}

/// 呼び出しの跡に覚える式の数の上限.
const MAX_TRACE: usize = 30;

/// 組み込みの種類の, 説明 (`error-message`) と, 親の種類 (`error-conditions` の 2 番目から).
const KINDS: &[(&str, &str, &[&str])] = &[
    ("error", "error", &[]),
    ("quit", "Quit", &[]),
    ("minibuffer-quit", "Quit", &["quit"]),
    ("user-error", "", &["error"]),
    ("args-out-of-range", "Args out of range", &["error"]),
    ("arith-error", "Arithmetic error", &["error"]),
    (
        "overflow-error",
        "Arithmetic overflow error",
        &["range-error", "arith-error", "error"],
    ),
    ("beginning-of-buffer", "Beginning of buffer", &["error"]),
    ("end-of-buffer", "End of buffer", &["error"]),
    ("buffer-read-only", "Buffer is read-only", &["error"]),
    (
        "text-read-only",
        "Text is read-only",
        &["buffer-read-only", "error"],
    ),
    ("invalid-function", "Invalid function", &["error"]),
    ("invalid-read-syntax", "Invalid read syntax", &["error"]),
    ("invalid-regexp", "Invalid regexp", &["error"]),
    ("no-catch", "No catch for tag", &["error"]),
    ("scan-error", "Scan error", &["error"]),
    ("search-failed", "Search failed", &["error"]),
    ("end-of-file", "End of file during parsing", &["error"]),
    ("file-error", "File error", &["error"]),
    ("file-missing", "File is missing", &["file-error", "error"]),
    ("file-supersession", "", &["file-error", "error"]),
    // 外で書き換わったファイルのバッファを、初めて書き換えようとした (EditorHost が受けて、使う人に聞く. 受けなかったときのために、文を持つ)
    ("file-supersession-threat", "", &["file-error", "error"]),
    (
        "file-already-exists",
        "File already exists",
        &["file-error", "error"],
    ),
    (
        "file-date-error",
        "Cannot set file date",
        &["file-error", "error"],
    ),
    ("wrong-type-argument", "Wrong type argument", &["error"]),
    (
        "wrong-number-of-arguments",
        "Wrong number of arguments",
        &["error"],
    ),
    (
        "void-variable",
        "Symbol's value as variable is void",
        &["error"],
    ),
    (
        "void-function",
        "Symbol's function definition is void",
        &["error"],
    ),
    (
        "setting-constant",
        "Attempt to set a constant symbol",
        &["error"],
    ),
    ("cl-assertion-failed", "Assertion failed", &["error"]),
];

impl Error {
    /// 種類とデータから作ります. 組み込みの種類は, 説明と受ける条件を知っています (知らない種類は, 受ける条件が無い).
    ///
    /// # Arguments
    ///
    /// * `kind` - 種類の名前.
    /// * `data` - データ.
    pub fn new(kind: &str, data: Vec<Value>) -> Self {
        let known = KINDS.iter().find(|(name, _, _)| *name == kind);
        // 知らない種類は, 受ける条件が無い (Emacs と同じ. `t` の節だけが受ける)
        let conditions = known.map_or_else(Vec::new, |(_, _, parents)| {
            std::iter::once(kind)
                .chain(parents.iter().copied())
                .map(str::to_string)
                .collect()
        });
        let description = known.map(|(_, text, _)| *text);
        Self::signal(kind, data, conditions, description)
    }

    /// `signal` で投げられたものから作ります (`define-error` で決めた, 説明と受ける条件を使う).
    ///
    /// # Arguments
    ///
    /// * `kind` - 種類の名前.
    /// * `data` - データ.
    /// * `conditions` - 受ける条件 (種類が先).
    /// * `description` - 種類の説明 (無ければ `peculiar error`).
    pub fn signal(
        kind: &str,
        data: Vec<Value>,
        conditions: Vec<String>,
        description: Option<&str>,
    ) -> Self {
        let message = Self::text(kind, &data, &conditions, description);
        Self {
            kind: kind.to_string(),
            data,
            conditions,
            message,
            trace: None,
        }
    }

    /// 失敗した式を, 呼び出しの跡に足します (内側の式から順に呼ぶ). 長い跡は, 外側を足さない.
    ///
    /// # Arguments
    ///
    /// * `form` - 式を表した文字列.
    pub fn push_trace(&mut self, form: String) {
        let trace = self.trace.get_or_insert_with(Default::default);
        if trace.forms.len() < MAX_TRACE {
            trace.forms.push(form);
        }
    }

    /// 呼び出しの跡 (失敗した式. 内側から外側へ).
    pub fn trace(&self) -> &[String] {
        self.trace
            .as_deref()
            .map_or(&[], |trace| trace.forms.as_slice())
    }

    /// 普通のエラー (`error` の種類で, データはメッセージ 1 つ).
    ///
    /// # Arguments
    ///
    /// * `message` - メッセージ.
    pub fn plain(message: impl Into<String>) -> Self {
        Self::new("error", vec![Value::String(message.into())])
    }

    /// 種類 (`error`, `wrong-type-argument` など).
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// データ.
    pub fn data(&self) -> &[Value] {
        &self.data
    }

    /// 受ける条件 (`error-conditions`. 種類が先).
    pub fn conditions(&self) -> &[String] {
        &self.conditions
    }

    /// 表示する文 (`error-message-string`).
    pub fn message(&self) -> &str {
        &self.message
    }

    /// `quit` (`C-g`) か.
    pub fn is_quit(&self) -> bool {
        self.conditions.iter().any(|c| c == "quit")
    }

    /// `condition-case` の節の条件が, このエラーを受けるか. `t` は何でも, ほかは `error-conditions` にあれば.
    ///
    /// # Arguments
    ///
    /// * `condition` - 節の条件の名前.
    pub fn matches(&self, condition: &str) -> bool {
        condition == "t" || self.conditions.iter().any(|c| c == condition)
    }

    /// `(種類 . データ)` のリスト (`condition-case` の変数の値).
    pub fn to_value(&self) -> Value {
        let mut items = vec![Value::Symbol(self.kind.clone())];
        items.extend(self.data.iter().cloned());
        Value::List(items.into())
    }

    /// `(種類 . データ)` のリストから作ります (`signal` に渡された形).
    ///
    /// # Arguments
    ///
    /// * `kind` - 種類の名前.
    /// * `data` - データのリスト.
    pub fn from_data(kind: &str, data: &Value) -> Self {
        let data = match data {
            Value::List(items) => items.to_vec(),
            Value::Boolean(false) | Value::Nil => Vec::new(),
            other => vec![other.clone()],
        };
        Self::new(kind, data)
    }

    /// 種類の説明とデータから文を作ります (Emacs の `print_error_message`).
    fn text(
        kind: &str,
        data: &[Value],
        conditions: &[String],
        description: Option<&str>,
    ) -> String {
        let file_error = conditions.iter().any(|c| c == "file-error");
        let (mut head, mut tail) = match (kind, data.first()) {
            ("error", Some(Value::String(text))) => (text.clone(), &data[1..]),
            ("error", _) => (
                "peculiar error".to_string(),
                data.get(1..).unwrap_or_default(),
            ),
            _ => (description.unwrap_or("peculiar error").to_string(), data),
        };
        // file-error は, 最初の文字列が説明
        if file_error {
            if let Some((first, rest)) = tail.split_first() {
                head = print_string(first, false);
                tail = rest;
            }
        }
        // データは, file-error と end-of-file と user-error は princ, ほかは prin1 の書き方
        let plain = file_error || kind == "end-of-file" || kind == "user-error";
        let mut text = head;
        for (i, item) in tail.iter().enumerate() {
            let separator = match i {
                0 if text.is_empty() => "",
                0 => ": ",
                _ => ", ",
            };
            text.push_str(separator);
            text.push_str(&print_string(item, !plain));
        }
        text
    }

    /// `wrong-type-argument`: 述語に合わない値.
    ///
    /// # Arguments
    ///
    /// * `predicate` - 合わなかった述語の名前 (`listp`, `stringp` など).
    /// * `value` - 渡された値.
    pub fn wrong_type(predicate: &str, value: &Value) -> Self {
        Self::new(
            "wrong-type-argument",
            vec![Value::Symbol(predicate.to_string()), value.clone()],
        )
    }

    /// `wrong-number-of-arguments`: 引数の数が合わない (データは, 手続きの名前と, 渡された個数).
    ///
    /// # Arguments
    ///
    /// * `name` - 手続きの名前.
    /// * `given` - 渡された引数の個数.
    pub fn wrong_number_of_arguments(name: &str, given: usize) -> Self {
        Self::new(
            "wrong-number-of-arguments",
            vec![Value::Symbol(name.to_string()), Value::Number(given as f64)],
        )
    }

    /// `args-out-of-range`: 範囲の外の引数.
    ///
    /// # Arguments
    ///
    /// * `data` - 引数の値 (バッファは `#<buffer 名前>` の記号).
    pub fn args_out_of_range(data: Vec<Value>) -> Self {
        Self::new("args-out-of-range", data)
    }

    /// `search-failed`: 探した文字列が無い.
    ///
    /// # Arguments
    ///
    /// * `pattern` - 探した文字列.
    pub fn search_failed(pattern: &str) -> Self {
        Self::new("search-failed", vec![Value::String(pattern.to_string())])
    }

    /// `void-variable`: 値の無い変数.
    ///
    /// # Arguments
    ///
    /// * `name` - 変数の名前.
    pub fn void_variable(name: &str) -> Self {
        Self::new("void-variable", vec![Value::Symbol(name.to_string())])
    }

    /// `void-function`: 定義の無い関数.
    ///
    /// # Arguments
    ///
    /// * `name` - 関数の名前.
    pub fn void_function(name: &str) -> Self {
        Self::new("void-function", vec![Value::Symbol(name.to_string())])
    }

    /// ファイルを開けなかった (`file-missing` か `file-error`. Emacs の `report_file_error`).
    ///
    /// # Arguments
    ///
    /// * `cause` - 入出力の失敗.
    /// * `path` - ファイルのパス.
    pub fn open_input(cause: &std::io::Error, path: &str) -> Self {
        let (kind, reason) = match cause.kind() {
            std::io::ErrorKind::NotFound => {
                ("file-missing", "No such file or directory".to_string())
            }
            std::io::ErrorKind::PermissionDenied => ("file-error", "Permission denied".to_string()),
            _ => ("file-error", cause.to_string()),
        };
        Self::strings(kind, &["Opening input file", &reason, path])
    }

    /// データの無い種類 (`quit`, `end-of-buffer`, `beginning-of-buffer`, `arith-error`).
    ///
    /// # Arguments
    ///
    /// * `kind` - 種類の名前.
    pub fn bare(kind: &str) -> Self {
        Self::new(kind, Vec::new())
    }

    /// 文字列のデータだけを持つ種類 (`end-of-file`, `scan-error`, `invalid-regexp`, `file-already-exists` など).
    ///
    /// # Arguments
    ///
    /// * `kind` - 種類の名前.
    /// * `strings` - 文字列のデータ.
    pub fn strings(kind: &str, strings: &[&str]) -> Self {
        Self::new(
            kind,
            strings
                .iter()
                .map(|s| Value::String((*s).to_string()))
                .collect(),
        )
    }
}

impl Deref for Error {
    type Target = str;

    fn deref(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.message)
    }
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Self::plain(message)
    }
}

impl From<&str> for Error {
    fn from(message: &str) -> Self {
        Self::plain(message)
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.data == other.data
    }
}

impl PartialEq<str> for Error {
    fn eq(&self, other: &str) -> bool {
        self.message == other
    }
}

impl PartialEq<&str> for Error {
    fn eq(&self, other: &&str) -> bool {
        self.message == *other
    }
}

impl PartialEq<String> for Error {
    fn eq(&self, other: &String) -> bool {
        &self.message == other
    }
}

impl serde::Serialize for Error {
    /// 表示する文として書き出す.
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.message)
    }
}
