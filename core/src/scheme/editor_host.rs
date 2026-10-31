//! `Editor` (バッファ, マーク, undo, kill-ring, ウィンドウ, 前置引数) を直接動かす [`Host`].
//!
//! 評価器が頼む操作 (`point`, `insert`, `re-search-forward`, `split-window` など) を, 画面に頼まずに, [`Editor`] に対して行います.
//! 画面の要る操作 (`message`, `read-string`, `highlight-regexp` など) は, 下位の [`Host`] (`ui`) に渡します.
//! ターミナルのアプリは, 画面の要る操作だけを自分で受ければ, エディタの操作は全てここで動きます.
//!
//! 位置は Emacs と同じく 1 から数えます (核の [`Buffer`] は 0 から数えるので, ここで 1 を足し引きします).

use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::atomic::write_atomic;
use crate::buffer::{Buffer, UndoEntry};
use crate::editor::{prefix_numeric_value, Editor, PrefixArg, GLOBAL_MARK_RING_MAX};
use crate::indent::{self, Config as IndentConfig, Plan};
use crate::paragraph::{forward_paragraph, forward_sentence};
use crate::scheme::host::Host;
use crate::scheme::value::Value;
use crate::search::{CharSet, Pattern};
use crate::sexp::{
    beginning_of_defun, end_of_defun, forward_sexp, mark_defun, up_list, MotionError,
};
use crate::syntax::{class_of_designator, syntax_of, Class, Dialect, Source};
use crate::window::Direction;
use crate::{comment, fill, replace};

/// 覚えておく正規表現の数の上限.
const PATTERN_CACHE_SIZE: usize = 64;

/// エディタを直接動かす [`Host`].
pub struct EditorHost<U: Host> {
    /// エディタの状態.
    editor: Arc<Mutex<Editor>>,
    /// 画面の要る操作を受けるもの.
    ui: U,
    /// 作った正規表現 (正規表現, 大文字と小文字を区別しないか).
    patterns: Mutex<HashMap<(String, bool, bool, Dialect), Pattern>>,
}

impl<U: Host> EditorHost<U> {
    /// エディタと, 画面の要る操作を受けるものから作ります.
    pub fn new(editor: Editor, ui: U) -> Self {
        Self::shared(Arc::new(Mutex::new(editor)), ui)
    }

    /// 共有するエディタから作ります (画面の要る操作を受けるものが, 描くためにエディタを読めるように).
    /// 画面の要る操作を頼むときは, エディタを閉じているので, 受けるものはエディタを読んでよい.
    pub fn shared(editor: Arc<Mutex<Editor>>, ui: U) -> Self {
        EditorHost {
            editor,
            ui,
            patterns: Mutex::new(HashMap::new()),
        }
    }

    /// エディタ (描くときや, キーを処理するときに使う).
    pub fn editor(&self) -> MutexGuard<'_, Editor> {
        self.editor.lock().unwrap()
    }

    /// 画面の要る操作を受けるもの.
    pub fn ui(&self) -> &U {
        &self.ui
    }

    /// 正規表現を作ります (作ったものは覚える).
    fn pattern(
        &self,
        text: &str,
        regexp: bool,
        fold: bool,
        dialect: Dialect,
    ) -> Result<Pattern, String> {
        let mut cache = self.patterns.lock().unwrap();
        let key = (text.to_string(), regexp, fold, dialect);
        if let Some(p) = cache.get(&key) {
            return Ok(p.clone());
        }
        let pattern = if regexp {
            Pattern::new(text, fold, dialect)?
        } else {
            Pattern::literal(text, fold)
        };
        if cache.len() >= PATTERN_CACHE_SIZE {
            cache.clear();
        }
        cache.insert(key, pattern.clone());
        Ok(pattern)
    }
}

impl<U: Host> Host for EditorHost<U> {
    fn call(&self, op: &str, args: Vec<Value>) -> Result<Value, String> {
        if !self.ui.is_native(op) {
            return self.ui.call(op, args);
        }
        self.ui.before_native(op)?;
        let outcome = {
            let mut editor = self.editor.lock().unwrap();
            self.dispatch(&mut editor, op, &args)?
        };
        match outcome {
            Some(Outcome::Value(value)) => Ok(value),
            // 操作が出すメッセージ (ファイルに書いたこと, マークを置いたこと) は, 画面に知らせる
            Some(Outcome::Message(text, value)) => {
                self.ui.call("message", vec![Value::String(text)])?;
                Ok(value)
            }
            // 受け持たない操作は, 画面に頼む
            None => self.ui.call(op, args),
        }
    }
}

/// 操作の結果.
enum Outcome {
    /// 評価器に返す値.
    Value(Value),
    /// 画面に出すメッセージと, 評価器に返す値.
    Message(String, Value),
}

/// 偽の値 (Emacs の nil).
fn nil() -> Value {
    Value::Boolean(false)
}

/// 位置の値 (1 から数える) を返します.
fn at(pos: usize) -> Value {
    Value::Number(pos as f64 + 1.0)
}

/// `indent-sexp` で字下げする式の終わり: 点の後ろの式の終わり. 終わりが行の途中で, 行の終わりが式の中なら, 行の終わりを越える式まで.
fn sexp_end_for_indent(source: &Source, point: usize) -> Result<usize, String> {
    // 点が, `(` の直後の文字の途中で, そのシンボルの後ろに式が無ければ (行末かコメントまで), Emacs はエラーにする
    let symbol = |c: Option<char>| c.is_some_and(|c| !indent::delimiter(c));
    if point > 0 && symbol(source.char_at(point - 1)) && symbol(source.char_at(point)) {
        let mut start = point;
        while start > 0 && symbol(source.char_at(start - 1)) {
            start -= 1;
        }
        let mut stop = point;
        while symbol(source.char_at(stop)) {
            stop += 1;
        }
        let mut rest = stop;
        while matches!(source.char_at(rest), Some(' ' | '\t')) {
            rest += 1;
        }
        if start > 0
            && source.char_at(start - 1) == Some('(')
            && matches!(source.char_at(rest), None | Some('\n' | ';'))
        {
            return Err("End position is smaller than start position".to_string());
        }
    }
    let mut end = forward_sexp(source, point).map_err(|e| e.to_string())?;
    let mut eol = end;
    while eol < source.size() && source.char_at(eol) != Some('\n') {
        eol += 1;
    }
    // 行の終わりが式の中にあれば, 行の終わりを越えるまで, 式を読み続ける (読めなければ, そこまで)
    if end < eol && source.parse_state(eol).depth > source.parse_state(end).depth {
        while end < eol {
            match forward_sexp(source, end) {
                Ok(next) if next > end => end = next,
                _ => break,
            }
        }
    }
    Ok(end)
}

/// 字下げの計画を, バッファに行います (後ろの変更から). カーソルは, 計画にあればそこへ, 無ければ変更から写す.
fn apply_plan(b: &mut Buffer, point: usize, plan: Plan) -> Result<(), String> {
    let cursor = plan
        .cursor
        .unwrap_or_else(|| indent::map_pos(&plan.edits, point, true));
    for e in plan.edits.iter().rev() {
        b.delete_region(e.from, e.to)?;
        b.insert_at(e.from, &e.insert)?;
    }
    b.set_point(cursor);
    Ok(())
}

/// 動いた先の位置の値. 失敗したら, Emacs と同じに, 動いた所にカーソルを置いて, 失敗にする.
fn moved(b: &mut Buffer, result: Result<usize, MotionError>) -> Result<Value, String> {
    match result {
        Ok(found) => Ok(at(found)),
        Err(e) => {
            b.set_point(e.point);
            Err(e.message)
        }
    }
}

/// 位置があれば, 位置の値. 無ければ nil.
fn at_opt(pos: Option<usize>) -> Value {
    pos.map_or_else(nil, at)
}

/// 引数の数を取り出します.
fn number(op: &str, args: &[Value], i: usize) -> Result<f64, String> {
    match args.get(i) {
        Some(Value::Number(n)) => Ok(*n),
        _ => Err(format!("{op} expects a number")),
    }
}

/// 整数を取り出します (小数は切り捨てる).
fn int(op: &str, args: &[Value], i: usize) -> Result<i64, String> {
    number(op, args, i).map(|n| n as i64)
}

/// 省ける整数 (無いか nil なら, 既定の値).
fn int_or(op: &str, args: &[Value], i: usize, default: i64) -> Result<i64, String> {
    match args.get(i) {
        Some(v) if v.is_non_nil() => int(op, args, i),
        _ => Ok(default),
    }
}

/// 位置の引数 (1 から数える) を, 0 から数える位置にします (1 より小さければ 0).
fn pos(op: &str, args: &[Value], i: usize) -> Result<usize, String> {
    number(op, args, i).map(|n| (n as i64 - 1).max(0) as usize)
}

/// 省ける位置の引数.
fn pos_opt(op: &str, args: &[Value], i: usize) -> Result<Option<usize>, String> {
    match args.get(i) {
        Some(v) if v.is_non_nil() => pos(op, args, i).map(Some),
        _ => Ok(None),
    }
}

/// 文字列の引数.
fn text<'a>(op: &str, args: &'a [Value], i: usize) -> Result<&'a str, String> {
    match args.get(i) {
        Some(Value::String(s)) => Ok(s),
        _ => Err(format!("{op} expects a string")),
    }
}

/// 省ける文字列の引数 (無いか nil なら `None`).
fn text_opt<'a>(op: &str, args: &'a [Value], i: usize) -> Result<Option<&'a str>, String> {
    match args.get(i) {
        Some(v) if v.is_non_nil() => text(op, args, i).map(Some),
        _ => Ok(None),
    }
}

/// 名前か記号の引数.
fn name<'a>(op: &str, args: &'a [Value], i: usize) -> Result<&'a str, String> {
    match args.get(i) {
        Some(Value::String(s) | Value::Symbol(s)) => Ok(s),
        _ => Err(format!("{op} expects a name")),
    }
}

/// バッファの引数 (名前. 無いか nil なら, 今のバッファ) を, 名前にします.
fn buffer_name(editor: &Editor, op: &str, args: &[Value], i: usize) -> Result<String, String> {
    Ok(match text_opt(op, args, i)? {
        Some(name) => name.to_string(),
        None => editor.current_buffer_name().to_string(),
    })
}

/// ウィンドウの引数 (無いか nil なら, 選んでいるウィンドウ).
fn window(editor: &Editor, op: &str, args: &[Value], i: usize) -> Result<usize, String> {
    match args.get(i) {
        Some(v) if v.is_non_nil() => Ok(int(op, args, i)? as usize),
        _ => Ok(editor.selected_window()),
    }
}

/// 文字の引数 (1 文字の文字列か, 文字の番号).
fn character(op: &str, args: &[Value], i: usize) -> Result<char, String> {
    match args.get(i) {
        Some(Value::String(s)) if s.chars().count() == 1 => Ok(s.chars().next().unwrap()),
        Some(Value::Number(n)) => {
            char::from_u32(*n as u32).ok_or(format!("{op}: invalid character"))
        }
        _ => Err(format!("{op} expects a character")),
    }
}

/// 構文の種類の文字 (Emacs の `char-syntax`).
fn class_char(class: Class) -> &'static str {
    match class {
        Class::Space => " ",
        Class::Punct => ".",
        Class::Word => "w",
        Class::Symbol => "_",
        Class::Open => "(",
        Class::Close => ")",
        Class::Str => "\"",
        Class::Escape => "\\",
        Class::Prefix => "'",
        Class::CommentStart => "<",
        Class::CommentEnd => ">",
    }
}

/// ファイルの拡張子から, 構文の方言を決めます.
pub(crate) fn dialect_of(path: &str) -> Dialect {
    match Path::new(path).extension().and_then(|e| e.to_str()) {
        Some("el") => Dialect::Elisp,
        Some("scm" | "ss" | "sld" | "sls" | "rkt") => Dialect::Scheme,
        _ => Dialect::Fundamental,
    }
}

/// 元に戻す記録を, 評価器の値にします (Emacs の `buffer-undo-list`. ドット対は無いので, 2 つの要素のリスト).
/// 入れた文字は `(開始 終了)`, 消した文字は `(文字 位置)` (カーソルが終わりにあったときは, 位置が負), 区切りは nil.
fn entry_value(entry: &UndoEntry) -> Value {
    match entry {
        UndoEntry::Insert { pos, len } => {
            Value::List(vec![at(*pos), at(pos + len)].into_iter().collect())
        }
        UndoEntry::Delete {
            pos,
            text,
            point_at_end,
        } => {
            let position = (*pos as f64 + 1.0) * if *point_at_end { -1.0 } else { 1.0 };
            Value::List(
                vec![Value::String(text.clone()), Value::Number(position)]
                    .into_iter()
                    .collect(),
            )
        }
        UndoEntry::Point(pos) => at(*pos),
        UndoEntry::Boundary => nil(),
    }
}

/// 評価器の値を, 元に戻す記録にします.
fn value_entry(value: &Value) -> Result<UndoEntry, String> {
    let invalid = || "primitive-undo: invalid undo entry".to_string();
    match value {
        Value::Boolean(false) | Value::Nil => Ok(UndoEntry::Boundary),
        Value::Number(n) if *n >= 1.0 => Ok(UndoEntry::Point(*n as usize - 1)),
        Value::List(items) => match &items[..] {
            [Value::Number(a), Value::Number(b)] if *a >= 1.0 && b >= a => Ok(UndoEntry::Insert {
                pos: *a as usize - 1,
                len: (*b - *a) as usize,
            }),
            [Value::String(text), Value::Number(p)] if *p != 0.0 => Ok(UndoEntry::Delete {
                pos: p.abs() as usize - 1,
                text: text.clone(),
                point_at_end: *p < 0.0,
            }),
            _ => Err(invalid()),
        },
        _ => Err(invalid()),
    }
}

impl<U: Host> EditorHost<U> {
    /// 受け持つ操作を動かします. 受け持たない操作なら `None`.
    fn dispatch(
        &self,
        ed: &mut Editor,
        op: &str,
        args: &[Value],
    ) -> Result<Option<Outcome>, String> {
        // 窓, バッファの一覧, kill-ring, 前置引数, ファイル
        if let Some(outcome) = self.editor_op(ed, op, args)? {
            return Ok(Some(outcome));
        }
        // 今のバッファ
        Ok(self.buffer_op(ed, op, args)?.map(Outcome::Value))
    }

    /// エディタ全体に対する操作. 当たらなければ `None`.
    fn editor_op(
        &self,
        ed: &mut Editor,
        op: &str,
        args: &[Value],
    ) -> Result<Option<Outcome>, String> {
        let value = match op {
            "buffer-list" => Value::List(ed.buffer_list().into_iter().map(Value::String).collect()),
            "get-buffer" => ed
                .get_buffer(text(op, args, 0)?)
                .map_or_else(nil, |b| Value::String(b.name().to_string())),
            "get-buffer-create" => {
                Value::String(ed.get_buffer_create(text(op, args, 0)?).name().to_string())
            }
            "generate-new-buffer" => Value::String(ed.generate_new_buffer(text(op, args, 0)?)),
            "set-buffer" => {
                let name = text(op, args, 0)?;
                ed.set_buffer(name)?;
                Value::String(name.to_string())
            }
            "current-buffer" => Value::String(ed.current_buffer_name().to_string()),
            "kill-buffer" => {
                let name = buffer_name(ed, op, args, 0)?;
                let buffer = ed
                    .get_buffer(&name)
                    .ok_or_else(|| format!("No such buffer: {name}"))?;
                if buffer.file().is_some() && buffer.modified() {
                    return Err(format!("Buffer {name} is modified"));
                }
                ed.kill_buffer(&name)?;
                Value::Boolean(true)
            }
            "kill-new" => {
                ed.kill_ring.kill_new(
                    text(op, args, 0)?,
                    args.get(1).is_some_and(Value::is_non_nil),
                );
                Value::Nil
            }
            "kill-append" => {
                ed.kill_ring.kill_append(
                    text(op, args, 0)?,
                    args.get(1).is_some_and(Value::is_non_nil),
                );
                Value::Nil
            }
            "current-kill" => {
                let n = int(op, args, 0)?;
                let keep = args.get(1).is_some_and(Value::is_non_nil);
                Value::String(ed.kill_ring.current_kill(n, keep)?)
            }
            "current-prefix-arg" => match ed.prefix_arg {
                None => nil(),
                Some(PrefixArg::Minus) => Value::Symbol("-".to_string()),
                Some(PrefixArg::Universal(n)) => {
                    Value::List(vec![Value::Number(n as f64)].into_iter().collect())
                }
                Some(PrefixArg::Number(n)) => Value::Number(n as f64),
            },
            "set-current-prefix-arg" => {
                ed.prefix_arg = prefix_arg_of(op, args.first())?;
                Value::Nil
            }
            "prefix-numeric-value" => {
                Value::Number(prefix_numeric_value(prefix_arg_of(op, args.first())?) as f64)
            }
            "selected-window" => Value::Number(ed.selected_window() as f64),
            "window-list" => Value::List(
                ed.window_list()
                    .into_iter()
                    .map(|w| Value::Number(w as f64))
                    .collect(),
            ),
            "split-window" => {
                let direction = match args.first() {
                    Some(v) if v.is_non_nil() => match name(op, args, 0)? {
                        "below" | "below-sides" => Direction::Below,
                        "right" | "right-sides" => Direction::Right,
                        other => return Err(format!("split-window: unknown direction {other}")),
                    },
                    _ => Direction::Below,
                };
                Value::Number(ed.split_window(direction) as f64)
            }
            "delete-window" => {
                let w = window(ed, op, args, 0)?;
                ed.delete_window(w)?;
                Value::Nil
            }
            "delete-other-windows" => {
                ed.delete_other_windows();
                Value::Nil
            }
            "other-window" => {
                ed.other_window(int_or(op, args, 0, 1)?);
                Value::Nil
            }
            "select-window" => {
                ed.select_window(int(op, args, 0)? as usize)?;
                Value::Nil
            }
            "set-window-buffer" => {
                let w = window(ed, op, args, 0)?;
                ed.set_window_buffer(w, text(op, args, 1)?)?;
                Value::Nil
            }
            "window-buffer" => {
                let w = window(ed, op, args, 0)?;
                ed.window_buffer(w)
                    .map(|b| Value::String(b.to_string()))
                    .ok_or_else(|| format!("No such window: {w}"))?
            }
            "find-file-noselect" => {
                let path = text(op, args, 0)?;
                return Ok(Some(Outcome::Value(Value::String(visit(ed, path)?))));
            }
            "pop-to-buffer-same-window" => {
                let name = text(op, args, 0)?;
                ed.get_buffer_create(name);
                let w = ed.selected_window();
                ed.set_window_buffer(w, name)?;
                ed.select_window(w)?;
                Value::Nil
            }
            "display-buffer" => {
                let name = text(op, args, 0)?.to_string();
                ed.get_buffer_create(&name);
                // 別のウィンドウに出す (1 つだけなら, 分ける)
                let selected = ed.selected_window();
                if ed.window_list().len() == 1 {
                    ed.split_window(Direction::Below);
                }
                let target = ed
                    .window_list()
                    .into_iter()
                    .find(|w| *w != selected)
                    .unwrap_or(selected);
                ed.set_window_buffer(target, &name)?;
                if let Some(line) = pos_opt(op, args, 1)? {
                    let buffer = ed.get_buffer_mut(&name).unwrap();
                    buffer.set_point(0);
                    buffer.forward_line(line as i64);
                }
                Value::Nil
            }
            "files--write-buffer" => {
                let path = text(op, args, 0)?.to_string();
                let name = buffer_name(ed, op, args, 1)?;
                let buffer = ed
                    .get_buffer_mut(&name)
                    .ok_or_else(|| format!("No such buffer: {name}"))?;
                write_atomic(Path::new(&path), buffer.text().as_bytes())
                    .map_err(|e| format!("{path}: {e}"))?;
                buffer.set_file(Some(path.clone()));
                buffer.set_modified(false);
                return Ok(Some(Outcome::Message(format!("Wrote {path}"), Value::Nil)));
            }
            "push-mark" => {
                let name = ed.current_buffer_name().to_string();
                let b = ed.current_buffer_mut();
                let p = pos_opt(op, args, 0)?.unwrap_or(b.point());
                b.push_mark(p, args.get(2).is_some_and(Value::is_non_nil));
                // 前に履歴に入れたマークが同じバッファでなければ, バッファをまたぐ履歴にも入れる
                if ed
                    .global_mark_ring
                    .first()
                    .is_none_or(|(top, _)| *top != name)
                {
                    let b = ed.current_buffer_mut();
                    let id = b.marker_new(p);
                    ed.global_mark_ring.insert(0, (name, id));
                    ed.global_mark_ring.truncate(GLOBAL_MARK_RING_MAX);
                }
                // 第 2 引数 (NOMSG) が無ければ, Emacs は "Mark set" と出す
                if !args.get(1).is_some_and(Value::is_non_nil) {
                    return Ok(Some(Outcome::Message("Mark set".to_string(), Value::Nil)));
                }
                Value::Nil
            }
            "global-mark-pop" => {
                // 先頭のマークの場所を返し, 履歴の最後に回す (消えたバッファのマークは, 捨てる)
                let ring = std::mem::take(&mut ed.global_mark_ring);
                let mut ring: Vec<(String, usize)> = ring
                    .into_iter()
                    .filter(|(name, id)| {
                        ed.get_buffer(name)
                            .is_some_and(|b| b.marker_position(*id).is_some())
                    })
                    .collect();
                if ring.is_empty() {
                    return Err("No global mark set".to_string());
                }
                let (name, id) = ring.remove(0);
                let position = ed.get_buffer(&name).unwrap().marker_position(id).unwrap();
                ring.push((name.clone(), id));
                ed.global_mark_ring = ring;
                Value::List(
                    vec![Value::String(name), at(position)]
                        .into_iter()
                        .collect(),
                )
            }
            "files--revert-dired" => nil(),
            _ => return Ok(None),
        };
        Ok(Some(Outcome::Value(value)))
    }

    /// 今のバッファに対する操作.
    fn buffer_op(
        &self,
        ed: &mut Editor,
        op: &str,
        args: &[Value],
    ) -> Result<Option<Value>, String> {
        // 名前でバッファを引く操作は, ここで先に済ませる
        match op {
            "buffer-file-name" | "buffer-modified-p" => {
                let name = buffer_name(ed, op, args, 0)?;
                let buffer = ed
                    .get_buffer(&name)
                    .ok_or_else(|| format!("No such buffer: {name}"))?;
                return Ok(Some(if op == "buffer-modified-p" {
                    Value::Boolean(buffer.modified())
                } else {
                    buffer
                        .file()
                        .map_or(Value::Nil, |f| Value::String(f.to_string()))
                }));
            }
            "kill-region" => {
                let (from, to) = (pos(op, args, 0)?, pos(op, args, 1)?);
                let killed = ed.current_buffer_mut().delete_region(from, to)?;
                ed.kill_ring.kill_new(&killed, false);
                return Ok(Some(Value::Nil));
            }
            "insert-for-yank" => {
                ed.current_buffer_mut().insert(text(op, args, 0)?);
                return Ok(Some(Value::Nil));
            }
            _ => {}
        }
        let fold = args
            .last()
            .is_some_and(|v| matches!(v, Value::Boolean(true)));
        let b = ed.current_buffer_mut();
        Ok(Some(match op {
            "point" => at(b.point()),
            "point-min" => at(0),
            "point-max" => at(b.len()),
            "goto-char" => {
                b.set_point(pos(op, args, 0)?);
                at(b.point())
            }
            "insert" => {
                for i in 0..args.len() {
                    b.insert(text(op, args, i)?);
                }
                Value::Nil
            }
            "insert-before-markers" => {
                b.insert_before_markers(text(op, args, 0)?);
                Value::Nil
            }
            "delete-region" => {
                b.delete_region(pos(op, args, 0)?, pos(op, args, 1)?)?;
                Value::Nil
            }
            "erase-buffer" => {
                let len = b.len();
                b.delete_region(0, len)?;
                Value::Nil
            }
            "buffer-string" => Value::String(b.text()),
            "buffer-substring" => Value::String(b.substring(pos(op, args, 0)?, pos(op, args, 1)?)?),
            "buffer-name" => Value::String(b.name().to_string()),
            "buffer-size" => Value::Number(b.len() as f64),
            "set-buffer-modified-p" => {
                b.set_modified(args.first().is_some_and(Value::is_true));
                Value::Nil
            }
            "files--set-buffer-text" => {
                replace_text(b, text(op, args, 0)?);
                b.set_modified(args.get(1).is_some_and(Value::is_true));
                Value::Nil
            }
            "char-after" => {
                let p = pos_opt(op, args, 0)?.unwrap_or(b.point());
                b.char_after(p)
                    .map_or_else(nil, |c| Value::String(c.to_string()))
            }
            "char-before" => {
                let p = pos_opt(op, args, 0)?.unwrap_or(b.point());
                b.char_before(p)
                    .map_or_else(nil, |c| Value::String(c.to_string()))
            }
            "forward-char" => {
                b.forward_char(int_or(op, args, 0, 1)?)?;
                Value::Nil
            }
            "backward-char" => {
                b.forward_char(-int_or(op, args, 0, 1)?)?;
                Value::Nil
            }
            "forward-line" => Value::Number(b.forward_line(int_or(op, args, 0, 1)?) as f64),
            "line-beginning-position" => at(b.line_beginning_position(int_or(op, args, 0, 1)?)),
            "line-end-position" => at(b.line_end_position(int_or(op, args, 0, 1)?)),
            "bobp" => Value::Boolean(b.bobp()),
            "eobp" => Value::Boolean(b.eobp()),
            "bolp" => Value::Boolean(b.bolp()),
            "eolp" => Value::Boolean(b.eolp()),
            "delete-char" => {
                let n = int(op, args, 0)?;
                let target = b.point() as i64 + n;
                if target < 0 {
                    return Err("Beginning of buffer".to_string());
                }
                if target > b.len() as i64 {
                    return Err("End of buffer".to_string());
                }
                b.delete_region(b.point(), target as usize)?;
                Value::Nil
            }
            "current-column" => Value::Number(b.current_column() as f64),
            "skip-chars-forward" | "skip-chars-backward" => {
                let set = CharSet::parse(text(op, args, 0)?)?;
                let limit = pos_opt(op, args, 1)?;
                Value::Number(b.skip_chars(&set, op.ends_with("forward"), limit) as f64)
            }
            "skip-syntax-forward" | "skip-syntax-backward" => {
                let spec = text(op, args, 0)?;
                let (negate, designators) = match spec.strip_prefix('^') {
                    Some(rest) => (true, rest),
                    None => (false, spec),
                };
                let classes = designators
                    .chars()
                    .map(|c| {
                        class_of_designator(c)
                            .ok_or_else(|| format!("Invalid syntax description letter: {c}"))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let forward = op.ends_with("forward");
                let source = Source::from_buffer(b, b.dialect());
                let limit =
                    pos_opt(op, args, 1)?.unwrap_or(if forward { source.size() } else { 0 });
                let from = b.point();
                let to = source.skip_syntax(from, &classes, negate, forward, limit);
                b.set_point(to);
                Value::Number(to as f64 - from as f64)
            }
            "move-to-column" => {
                Value::Number(b.move_to_column(int(op, args, 0)?.max(0) as usize) as f64)
            }
            "search-forward" | "search-backward" | "re-search-forward" | "re-search-backward" => {
                self.search(b, op, args, fold)?
            }
            "looking-at" => {
                let pattern = self.pattern(text(op, args, 0)?, true, fold, b.dialect())?;
                Value::Boolean(b.looking_at(&pattern))
            }
            "match-beginning" | "match-end" => {
                let n = int(op, args, 0)?.max(0) as usize;
                let group = b.match_data().get(n).copied().flatten();
                at_opt(group.map(|(s, e)| if op == "match-beginning" { s } else { e }))
            }
            "match-data" => Value::List(
                b.match_data()
                    .iter()
                    .flat_map(|m| match m {
                        Some((s, e)) => [at(*s), at(*e)],
                        None => [nil(), nil()],
                    })
                    .collect(),
            ),
            "match-string" => {
                let n = int(op, args, 0)?.max(0) as usize;
                match b.match_data().get(n).copied().flatten() {
                    Some((s, e)) => Value::String(b.substring(s, e)?),
                    None => nil(),
                }
            }
            "replace-match" | "match-substitute-replacement" => {
                let fixed = args.get(1).is_some_and(Value::is_non_nil);
                let literal = args.get(2).is_some_and(Value::is_non_nil);
                // 引数は Emacs と同じ (NEWTEXT FIXEDCASE LITERAL STRING SUBEXP). 文字列の中の置き換え (STRING) は, まだ無い
                if args.get(3).is_some_and(Value::is_non_nil) {
                    return Err(format!("{op}: STRING is not supported"));
                }
                let sub = int_or(op, args, 4, 0)?.max(0) as usize;
                let data = b.match_data().to_vec();
                let Some((from, to)) = data.get(sub).copied().flatten() else {
                    return Err("replace-match subexpression does not exist".to_string());
                };
                let newtext = text(op, args, 0)?;
                let matched = b.substring(from, to)?;
                let mut new = if literal {
                    newtext.to_string()
                } else {
                    let groups: Vec<Option<String>> = data
                        .iter()
                        .map(|g| g.and_then(|(s, e)| b.substring(s, e).ok()))
                        .collect();
                    replace::expand(newtext, &groups)?
                };
                if !fixed {
                    new = replace::apply_case(&new, replace::case_action(&matched));
                }
                if op == "match-substitute-replacement" {
                    return Ok(Some(Value::String(new)));
                }
                b.delete_region(from, to)?;
                // 空の範囲を置き換えるときは, そこにある記録 (マークなど) も後ろへずらす (Emacs の `replace_range`)
                b.insert_at_with(from, &new, from == to)?;
                let end = from + new.chars().count();
                b.set_point(end);
                // 合った範囲は, 置き換えた分だけ変わる (括弧の範囲は, 捨てる)
                let delta = end as i64 - to as i64;
                let (start0, end0) = data[0].unwrap_or((from, to));
                b.match_data = vec![Some((start0, (end0 as i64 + delta) as usize))];
                Value::Nil
            }
            "set-match-data" => {
                let items = match args.first() {
                    Some(Value::List(items)) => items.to_vec(),
                    Some(Value::Nil) => Vec::new(),
                    _ => return Err("set-match-data expects a list".to_string()),
                };
                // (始め 終わり) の組ごと. 合わなかった括弧は #f の組. 余った要素は捨てる (Emacs と同じ)
                let position = |n: f64| (n as i64 - 1).max(0) as usize;
                let mut data = Vec::new();
                for [s, e] in items.as_chunks::<2>().0 {
                    data.push(match (s, e) {
                        (Value::Number(s), Value::Number(e)) => Some((position(*s), position(*e))),
                        (s, e) if !s.is_non_nil() && !e.is_non_nil() => None,
                        _ => return Err("set-match-data expects a number".to_string()),
                    });
                }
                // 終わりの合わなかった括弧は, 持たない (Emacs の `match-data` は返さない)
                while data.last() == Some(&None) {
                    data.pop();
                }
                b.match_data = data;
                Value::Nil
            }
            "highlight-regexp" => {
                let regexp = text(op, args, 0)?.to_string();
                let face = text_opt(op, args, 1)?.unwrap_or("hi-yellow").to_string();
                let group = int_or(op, args, 2, 0)?.max(0) as usize;
                // 正規表現に大文字が無ければ, 大文字と小文字を区別しない (hi-lock と同じ)
                let fold = regexp == regexp.to_lowercase();
                Pattern::new(&regexp, fold, b.dialect())?;
                b.add_highlight(crate::buffer::Highlight {
                    regexp,
                    face,
                    group,
                    fold,
                });
                Value::Nil
            }
            "unhighlight-regexp" => {
                match args.first() {
                    Some(Value::String(regexp)) => b.remove_highlight(Some(regexp)),
                    _ => b.remove_highlight(None),
                };
                Value::Nil
            }
            "highlighted-regexps" => Value::List(
                b.highlights()
                    .iter()
                    .map(|h| Value::String(h.regexp.clone()))
                    .collect(),
            ),
            "mark" => at_opt(b.mark()),
            "set-mark" => {
                match pos_opt(op, args, 0)? {
                    Some(p) => b.set_mark(p),
                    None => b.deactivate_mark(),
                }
                Value::Nil
            }
            "set-mark-position" => {
                b.set_mark_position(pos(op, args, 0)?);
                Value::Nil
            }
            "pop-mark" => {
                b.pop_mark();
                Value::Nil
            }
            "deactivate-mark" => {
                b.deactivate_mark();
                Value::Nil
            }
            "region-beginning" => at(b.region_beginning()?),
            "region-end" => at(b.region_end()?),
            "use-region-p" => Value::Boolean(b.use_region_p()),
            "region-active-p" => Value::Boolean(b.region_active()),
            "activate-mark" => {
                b.activate_mark();
                Value::Nil
            }
            "deactivate-mark-requested" => Value::Boolean(b.deactivate_requested()),
            "set-deactivate-mark" => {
                b.set_deactivate_requested(args.first().is_some_and(Value::is_non_nil));
                Value::Nil
            }
            "point-marker" => Value::Number(b.marker_new(b.point()) as f64),
            "marker-position" => at_opt(b.marker_position(int(op, args, 0)?.max(0) as usize)),
            "marker-release" => {
                b.marker_release(int(op, args, 0)?.max(0) as usize);
                Value::Nil
            }
            "undo-boundary" => {
                b.undo_boundary();
                Value::Nil
            }
            "buffer-undo-list" => {
                Value::List(b.undo_list().iter().rev().map(entry_value).collect())
            }
            "primitive-undo" => primitive_undo(b, op, args)?,
            "set-buffer-undo-list" => {
                let Some(Value::List(items)) = args.first() else {
                    return Err(format!("{op} expects a list"));
                };
                let mut entries = items
                    .iter()
                    .map(value_entry)
                    .collect::<Result<Vec<_>, _>>()?;
                // 評価器の値は新しいものが先, 履歴は古いものが先
                entries.reverse();
                b.set_undo_list(entries);
                Value::Nil
            }
            "scan-lists" | "scan-sexps" => {
                let sexp = op == "scan-sexps";
                let from = pos(op, args, 0)?;
                let count = int(op, args, 1)?;
                let depth = if sexp { 0 } else { int(op, args, 2)? };
                let source = Source::from_buffer(b, b.dialect());
                let found = source
                    .scan_lists(from, count, depth, sexp)
                    .map_err(|e| e.to_string())?;
                at_opt(found)
            }
            "up-list-position" => {
                let source = Source::from_buffer(b, b.dialect());
                moved(b, up_list(&source, pos(op, args, 0)?, int(op, args, 1)?))?
            }
            "beginning-of-defun-position" => {
                let source = Source::from_buffer(b, b.dialect());
                at(beginning_of_defun(&source, pos(op, args, 0)?))
            }
            "end-of-defun-position" => {
                let source = Source::from_buffer(b, b.dialect());
                moved(
                    b,
                    end_of_defun(&source, pos(op, args, 0)?, int_or(op, args, 1, 1)?),
                )?
            }
            "mark-defun-range" => {
                let source = Source::from_buffer(b, b.dialect());
                let range = mark_defun(&source, pos(op, args, 0)?, pos_opt(op, args, 1)?);
                match range {
                    Ok((head, mark)) => Value::List(vec![at(head), at(mark)].into_iter().collect()),
                    Err(e) => {
                        b.set_point(e.point);
                        return Err(e.message);
                    }
                }
            }
            "forward-paragraph-position" => {
                let chars: Vec<char> = b.text().chars().collect();
                at(forward_paragraph(
                    &chars,
                    pos(op, args, 0)?,
                    int(op, args, 1)?,
                    b.dialect(),
                ))
            }
            "forward-sentence-position" => {
                let chars: Vec<char> = b.text().chars().collect();
                let found =
                    forward_sentence(&chars, pos(op, args, 0)?, int(op, args, 1)?, b.dialect());
                moved(b, found)?
            }
            "indent-for-tab-op"
            | "indent-region-op"
            | "indent-sexp-op"
            | "newline-and-indent-op"
            | "fill-paragraph-op"
            | "comment-dwim-op"
            | "comment-region-op" => {
                let tabs = args.last().is_some_and(Value::is_non_nil);
                let config = IndentConfig {
                    dialect: b.dialect(),
                    indent_tabs: tabs,
                    tab_width: crate::buffer::TAB_WIDTH,
                };
                let text = b.text();
                let lines: Vec<Vec<char>> = text.split('\n').map(|l| l.chars().collect()).collect();
                let point = b.point();
                let plan = match op {
                    "fill-paragraph-op" => {
                        let column = usize::try_from(int(op, args, 0)?).unwrap_or(0);
                        match fill::fill_paragraph(
                            &lines,
                            point,
                            &config,
                            column,
                            text_opt(op, args, 1)?,
                        ) {
                            Some(plan) => plan,
                            None => return Ok(Some(Value::Nil)),
                        }
                    }
                    "comment-dwim-op" => comment::comment_dwim(
                        &lines,
                        point,
                        pos_opt(op, args, 0)?,
                        usize::try_from(int(op, args, 1)?).unwrap_or(0),
                        &config,
                    )?,
                    "comment-region-op" => comment::comment_region(
                        &lines,
                        pos(op, args, 0)?,
                        pos(op, args, 1)?,
                        point,
                        &config,
                    ),
                    "indent-for-tab-op" => indent::indent_for_tab(&lines, point, &config),
                    "indent-region-op" => indent::indent_region(
                        &lines,
                        pos(op, args, 0)?,
                        pos(op, args, 1)?,
                        point,
                        &config,
                    ),
                    "indent-sexp-op" => {
                        let source = Source::from_buffer(b, b.dialect());
                        let end = sexp_end_for_indent(&source, point)?;
                        indent::indent_sexp(&lines, point, end, &config)
                    }
                    _ => indent::newline_and_indent(&lines, point, &config),
                };
                apply_plan(b, point, plan)?;
                Value::Nil
            }
            "syntax-ppss" => {
                let p = pos_opt(op, args, 0)?.unwrap_or(b.point());
                let state = Source::from_buffer(b, b.dialect()).parse_state(p);
                let opens: Vec<Value> = state.opens.iter().map(|o| at(*o)).collect();
                let fields = vec![
                    Value::Number(state.depth as f64),
                    at_opt(state.opens.last().copied()),
                    nil(),
                    Value::Boolean(state.in_string),
                    Value::Boolean(state.in_comment),
                    nil(),
                    Value::Number(0.0),
                    Value::Boolean(state.nested),
                    at_opt(state.start),
                    Value::List(opens.into()),
                    nil(),
                ];
                Value::List(fields.into())
            }
            "char-syntax" => {
                let c = character(op, args, 0)?;
                Value::String(class_char(syntax_of(c, b.dialect()).class).to_string())
            }
            "syntax-class-at" => {
                let source = Source::from_buffer(b, b.dialect());
                Value::String(class_char(source.class_of(pos(op, args, 0)?)).to_string())
            }
            "forward-comment" => {
                let source = Source::from_buffer(b, b.dialect());
                let (to, done) = source.skip_comments(b.point(), int(op, args, 0)?);
                b.set_point(to);
                Value::Boolean(done)
            }
            "backward-prefix-chars" => {
                let source = Source::from_buffer(b, b.dialect());
                let to = source.backward_prefix_chars(b.point());
                b.set_point(to);
                Value::Nil
            }
            "syntax-dialect" => Value::Symbol(
                match b.dialect() {
                    Dialect::Fundamental => "fundamental",
                    Dialect::Elisp => "elisp",
                    Dialect::Scheme => "scheme",
                }
                .to_string(),
            ),
            "set-syntax-dialect" => {
                b.set_dialect(match name(op, args, 0)? {
                    "fundamental" => Dialect::Fundamental,
                    "elisp" => Dialect::Elisp,
                    "scheme" => Dialect::Scheme,
                    other => return Err(format!("set-syntax-dialect: unknown dialect {other}")),
                });
                Value::Nil
            }
            _ => return Ok(None),
        }))
    }

    /// 探します (`search-forward`, `re-search-backward` など).
    /// 引数は, 探す文字, 限り, 見つからないときの扱い, 回数, 最後に, 大文字と小文字を区別しないか.
    fn search(
        &self,
        b: &mut Buffer,
        op: &str,
        args: &[Value],
        fold: bool,
    ) -> Result<Value, String> {
        let regexp = op.starts_with("re-");
        let mut forward = op.ends_with("forward");
        let needle = text(op, args, 0)?;
        let bound = pos_opt(op, args, 1)?;
        let noerror = args.get(2).cloned().unwrap_or(Value::Boolean(false));
        let mut count = int_or(op, args, 3, 1)?;
        if count < 0 {
            forward = !forward;
            count = -count;
        }
        if count == 0 {
            return Ok(at(b.point()));
        }
        if bound.is_some_and(|limit| {
            if forward {
                limit < b.point()
            } else {
                limit > b.point()
            }
        }) {
            return Err("Invalid search bound (wrong side of point)".to_string());
        }
        let pattern = self.pattern(needle, regexp, fold, b.dialect())?;
        let found = if forward {
            b.re_search_forward(&pattern, bound, count as usize)
        } else {
            b.re_search_backward(&pattern, bound, count as usize)
        };
        match found {
            // 文字を探すときは, 後ろ向きでも, 合った所の始めに止まる (Emacs と同じ)
            Some(p) => Ok(at(p)),
            None => match noerror {
                Value::Boolean(false) | Value::Nil => Err(format!("Search failed: \"{needle}\"")),
                Value::Boolean(true) => Ok(nil()),
                _ => {
                    b.set_point(bound.unwrap_or(if forward { b.len() } else { 0 }));
                    Ok(nil())
                }
            },
        }
    }
}

/// 前置引数の値 (nil, `-`, `(4)`, 数) を, 前置引数にします.
fn prefix_arg_of(op: &str, v: Option<&Value>) -> Result<Option<PrefixArg>, String> {
    Ok(match v {
        None | Some(Value::Boolean(false) | Value::Nil) => None,
        Some(Value::Symbol(s)) if s == "-" => Some(PrefixArg::Minus),
        Some(Value::Number(n)) => Some(PrefixArg::Number(*n as i64)),
        Some(Value::List(items)) => match &items[..] {
            [Value::Number(n)] => Some(PrefixArg::Universal(*n as i64)),
            _ => return Err(format!("{op}: invalid prefix argument")),
        },
        Some(_) => return Err(format!("{op}: invalid prefix argument")),
    })
}

/// 元に戻します (Emacs の `primitive-undo`): `(primitive-undo N LIST)`. 区切りまでを 1 つのまとまりとして N 回元に戻し,
/// 残りの記録のリストを返す.
fn primitive_undo(b: &mut Buffer, op: &str, args: &[Value]) -> Result<Value, String> {
    let mut groups = int(op, args, 0)?;
    let Some(Value::List(list)) = args.get(1) else {
        return Err(format!("{op} expects a list"));
    };
    let mut rest: &[Value] = &list[..];
    while groups > 0 {
        let mut group = Vec::new();
        while let Some((first, tail)) = rest.split_first() {
            rest = tail;
            match value_entry(first)? {
                UndoEntry::Boundary => break,
                entry => group.push(entry),
            }
        }
        b.primitive_undo(&group)?;
        groups -= 1;
    }
    Ok(Value::List(rest.iter().cloned().collect()))
}

/// バッファの本文を置き換えます. 前と後ろの同じ所は残すので, カーソルは同じ文字の所に残る.
fn replace_text(b: &mut Buffer, new: &str) {
    let old: Vec<char> = b.text().chars().collect();
    let new: Vec<char> = new.chars().collect();
    let start = old.iter().zip(&new).take_while(|(a, b)| a == b).count();
    let end = old[start..]
        .iter()
        .rev()
        .zip(new[start..].iter().rev())
        .take_while(|(a, b)| a == b)
        .count();
    if old.len() == new.len() && start + end >= old.len() {
        return;
    }
    // 範囲は本文の中にあるので, 失敗しない
    let _ = b.delete_region(start, old.len() - end);
    let inserted: String = new[start..new.len() - end].iter().collect();
    let _ = b.insert_at(start, &inserted);
}

/// ファイルを訪れるバッファを返します (Emacs の `find-file-noselect`). 既に訪れていれば, そのバッファ.
/// 無ければ, ファイルを読んで作る (ファイルが無ければ, 空のバッファ).
fn visit(ed: &mut Editor, path: &str) -> Result<String, String> {
    if let Some(existing) = ed
        .buffer_list()
        .into_iter()
        .find(|name| ed.get_buffer(name).and_then(Buffer::file) == Some(path))
    {
        return Ok(existing);
    }
    let content = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(e) => return Err(format!("{path}: {e}")),
    };
    let base = Path::new(path)
        .file_name()
        .map_or(path.to_string(), |n| n.to_string_lossy().into_owned());
    let name = ed.generate_new_buffer(&base);
    let buffer = ed.get_buffer_mut(&name).unwrap();
    buffer.load(&content);
    buffer.set_file(Some(path.to_string()));
    buffer.set_dialect(dialect_of(path));
    Ok(name)
}
