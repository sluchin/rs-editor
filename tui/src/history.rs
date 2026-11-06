//! ミニバッファの入力の履歴 (Emacs の `M-p`, `M-n`, `M-r`, `M-s`).
//!
//! 履歴は新しい順に並べる. 位置 0 は入力中の文字, 位置 1 以上は履歴 (1 が最も新しい),
//! 位置 -1 以下は既定の値 (`M-n` で出す. Emacs の future history).

use rseditor_core::scheme::value::Value;
use rseditor_core::search::Pattern;
use rseditor_core::syntax::Dialect;

/// 問いの履歴 (核の `read-string` などが渡す, 履歴の名前, 既定の値, 中身, 始めの位置).
#[derive(Debug, Default, Clone, PartialEq)]
pub struct HistoryArgs {
    /// 履歴の名前 (空なら, 履歴を使わない).
    pub name: String,
    /// 既定の値 (`M-n` で, 履歴の先に出す).
    pub defaults: Vec<String>,
    /// 履歴の中身 (Lisp の変数の値. 新しい順). `None` なら, 画面が名前ごとに覚えている履歴を使う.
    pub items: Option<Vec<String>>,
    /// 始めの位置 (Emacs の HISTORY の `(SYMBOL . POSITION)`. 0 は打っている文字).
    pub start: i64,
}

impl HistoryArgs {
    /// 名前と既定の値から作ります (中身は画面が覚えているもの, 始めの位置は 0).
    pub fn named(name: &str, defaults: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            defaults,
            ..Self::default()
        }
    }

    /// 核の頼みの引数 (`i` 番目から: 名前, 既定の値, 中身, 始めの位置) から作ります.
    pub fn from_args(args: &[Value], i: usize) -> Self {
        let strings = |v: Option<&Value>| match v {
            Some(Value::List(list)) => Some(
                list.iter()
                    .filter_map(|v| match v {
                        Value::String(s) => Some(s.clone()),
                        _ => None,
                    })
                    .collect::<Vec<_>>(),
            ),
            _ => None,
        };
        Self {
            name: match args.get(i) {
                Some(Value::String(s)) => s.clone(),
                _ => String::new(),
            },
            defaults: strings(args.get(i + 1)).unwrap_or_default(),
            items: strings(args.get(i + 2)),
            start: match args.get(i + 3) {
                Some(Value::Number(n)) => *n as i64,
                _ => 0,
            },
        }
    }
}

/// 履歴の数の限り (Emacs の `history-length`).
pub const HISTORY_LENGTH: usize = 100;

/// 履歴の先頭に加えます (Emacs の `add-to-history`). 空の文字と, 先頭と同じ文字は, 加えません.
///
/// # Arguments
///
/// * `history` - 履歴 (新しい順).
/// * `item` - 加える文字.
pub fn add_to_history(history: &mut Vec<String>, item: &str) {
    if item.is_empty() || history.first().is_some_and(|first| first == item) {
        return;
    }
    history.insert(0, item.to_string());
    history.truncate(HISTORY_LENGTH);
}

/// 履歴をたどる状態 (1 回の入力の間だけ持つ).
#[derive(Debug, Default)]
pub struct Navigator {
    /// 今の位置 (0 が入力中の文字).
    pos: i64,
    /// 履歴へ動く前に打っていた文字.
    saved: String,
}

impl Navigator {
    /// 作ります.
    pub fn new() -> Self {
        Self::default()
    }

    /// 履歴の位置 `pos` から始める状態を作ります (Emacs の HISTORY の `(SYMBOL . POSITION)`).
    /// 位置 0 に戻ると, 始めの文字 `input` を出す.
    pub fn starting_at(pos: i64, input: &str, history: &[String]) -> Self {
        Self {
            pos: pos.clamp(0, history.len() as i64),
            saved: input.to_string(),
        }
    }

    /// 位置 `pos` の文字を返します (0 は, 打っていた文字).
    fn at(&self, pos: i64, history: &[String], defaults: &[String]) -> String {
        match pos {
            0 => self.saved.clone(),
            p if p > 0 => history[p as usize - 1].clone(),
            p => defaults[(-p) as usize - 1].clone(),
        }
    }

    /// 動く先を決めます (位置 0 を離れるときは, 打っていた文字を覚える).
    fn move_to(
        &mut self,
        pos: i64,
        input: &str,
        history: &[String],
        defaults: &[String],
    ) -> String {
        if self.pos == 0 {
            self.saved = input.to_string();
        }
        self.pos = pos;
        self.at(pos, history, defaults)
    }

    /// `M-p`: 1 つ古い履歴を返します.
    ///
    /// # Errors
    ///
    /// 古い履歴が無いとき (エコー領域に出す文字).
    pub fn previous(
        &mut self,
        input: &str,
        history: &[String],
        defaults: &[String],
    ) -> Result<String, String> {
        if self.pos >= history.len() as i64 {
            return Err("Beginning of history; no preceding item".to_string());
        }
        Ok(self.move_to(self.pos + 1, input, history, defaults))
    }

    /// `M-n`: 1 つ新しい履歴を返します. 履歴の先では, 既定の値を返します.
    ///
    /// # Errors
    ///
    /// 新しい履歴も既定の値も無いとき (エコー領域に出す文字).
    pub fn next(
        &mut self,
        input: &str,
        history: &[String],
        defaults: &[String],
    ) -> Result<String, String> {
        if self.pos <= -(defaults.len() as i64) {
            return Err(if defaults.is_empty() {
                "End of history; no default available".to_string()
            } else {
                "End of defaults; no next item".to_string()
            });
        }
        Ok(self.move_to(self.pos - 1, input, history, defaults))
    }

    /// `M-r` (`older` が真) と `M-s`: 正規表現に合う履歴を探して返します.
    ///
    /// # Arguments
    ///
    /// * `regexp` - Emacs の正規表現.
    /// * `older` - 古い方へ探すか (`M-r`). 偽なら新しい方へ (`M-s`).
    /// * `input` - 打っていた文字.
    ///
    /// # Errors
    ///
    /// 合うものが無いとき, 正規表現が誤っているとき (エコー領域に出す文字).
    pub fn search(
        &mut self,
        regexp: &str,
        older: bool,
        input: &str,
        history: &[String],
        defaults: &[String],
    ) -> Result<String, String> {
        let pattern =
            Pattern::new(regexp, true, Dialect::Fundamental).map_err(|e| e.to_string())?;
        let found = if older {
            (self.pos + 1..=history.len() as i64)
                .find(|&p| matches(&pattern, &history[p as usize - 1]))
        } else {
            (1..self.pos)
                .rev()
                .find(|&p| matches(&pattern, &history[p as usize - 1]))
        };
        match found {
            Some(pos) => Ok(self.move_to(pos, input, history, defaults)),
            None if older => Err("No earlier matching history item".to_string()),
            None => Err("No later matching history item".to_string()),
        }
    }
}

/// 文字が正規表現に合うか.
fn matches(pattern: &Pattern, text: &str) -> bool {
    pattern.captures_from(text, 0).is_some()
}
