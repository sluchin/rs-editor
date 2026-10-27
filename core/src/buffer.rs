//! バッファ: 本文 (rope), カーソル (point), マーク, 元に戻す履歴 (undo).
//!
//! Emacs の C の関数に当たるもの (`insert`, `delete-region`, `forward-line`, `current-column` など) を持ちます.
//! 位置は 0 から数える文字の位置です (Lisp の位置は 1 から数えるので, Lisp との間で 1 を足し引きします).
//! 画面 (Tauri) にもターミナルにも頼らないので, どちらからも同じように使えます.

use ropey::Rope;

use crate::scheme::builtins::char_width;
use crate::syntax::Dialect;

/// タブの幅 (Emacs の `tab-width` の始めの値).
pub const TAB_WIDTH: usize = 8;

/// マークの履歴 (Emacs の `mark-ring`) の長さの上限 (Emacs の `mark-ring-max`).
pub const MARK_RING_MAX: usize = 16;

/// 元に戻す履歴の 1 つ (Emacs の `buffer-undo-list` の要素に当たる).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UndoEntry {
    /// 本文を入れた (始めの位置と長さ). 続けて入れたものは 1 つにまとめる.
    Insert {
        /// 始めの位置.
        pos: usize,
        /// 入れた文字の数.
        len: usize,
    },
    /// 本文を消した.
    Delete {
        /// 消した所の位置.
        pos: usize,
        /// 消した文字.
        text: String,
        /// 消す前のカーソルが, 消した所の終わりにあったか (元に戻すと, カーソルをそこへ置く).
        point_at_end: bool,
    },
    /// 元に戻したときに, カーソルを置く位置 (Emacs の履歴の, 位置だけの要素).
    /// 区切りの直後の変更の前に, カーソルが変更の所になかったとき, 前の区切りでのカーソルを覚える.
    Point(usize),
    /// まとまりの区切り (Emacs の `nil`. 元に戻すのは, 区切りまでを 1 回とする).
    Boundary,
}

/// バッファ.
#[derive(Debug, Clone)]
pub struct Buffer {
    /// バッファの名前.
    name: String,
    /// 本文.
    pub(crate) text: Rope,
    /// カーソルの位置.
    pub(crate) point: usize,
    /// マークの位置 (無ければ `None`).
    mark: Option<usize>,
    /// マークを使うか (Emacs の `mark-active`).
    mark_active: bool,
    /// マークの履歴 (新しいものが先).
    mark_ring: Vec<usize>,
    /// 書き換えたか.
    modified: bool,
    /// ファイルのパス (ファイルを訪れていなければ `None`).
    file: Option<String>,
    /// 元に戻す履歴 (古いものが先).
    undo: Vec<UndoEntry>,
    /// 最後に合った所 (全体と, 括弧ごと. Emacs の `match-data`). 位置は文字の位置.
    pub(crate) match_data: Vec<Option<(usize, usize)>>,
    /// 構文の方言 (メジャーモードに当たる).
    dialect: Dialect,
    /// マーカー (位置を覚えておいて, 本文を書き換えると, 位置を追う. 解いたものは `None`).
    markers: Vec<Option<usize>>,
    /// 最後の区切りを入れたときのカーソル (Emacs の `point_before_last_command_or_undo`).
    point_at_boundary: Option<usize>,
    /// コマンドの終わりにマークを外すよう頼まれているか (Emacs の変数 `deactivate-mark`. 本文を書き換えると, 真になる).
    deactivate_requested: bool,
}

impl Buffer {
    /// 空のバッファを作ります.
    ///
    /// # Arguments
    ///
    /// * `name` - バッファの名前.
    pub fn new(name: &str) -> Self {
        Buffer {
            name: name.to_string(),
            text: Rope::new(),
            point: 0,
            mark: None,
            mark_active: false,
            mark_ring: Vec::new(),
            modified: false,
            file: None,
            undo: Vec::new(),
            match_data: Vec::new(),
            dialect: Dialect::Fundamental,
            markers: Vec::new(),
            point_at_boundary: None,
            deactivate_requested: false,
        }
    }

    /// 本文のあるバッファを作ります (書き換えていない, 履歴は空, カーソルは始め).
    ///
    /// # Arguments
    ///
    /// * `name` - バッファの名前.
    /// * `text` - 本文.
    pub fn with_text(name: &str, text: &str) -> Self {
        let mut buffer = Buffer::new(name);
        buffer.text = Rope::from_str(text);
        buffer
    }

    /// 本文を読み込みます (ファイルを開いたとき). カーソルは始め, 書き換えていない状態で, 履歴は空にする.
    pub fn load(&mut self, text: &str) {
        self.text = Rope::from_str(text);
        self.point = 0;
        self.mark = None;
        self.mark_active = false;
        self.mark_ring.clear();
        self.modified = false;
        self.undo.clear();
    }

    /// バッファの名前.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// ファイルのパス.
    pub fn file(&self) -> Option<&str> {
        self.file.as_deref()
    }

    /// ファイルのパスを決めます.
    pub fn set_file(&mut self, file: Option<String>) {
        self.file = file;
    }

    /// 構文の方言 (Fundamental, emacs-lisp-mode, scheme-mode に当たる).
    pub fn dialect(&self) -> Dialect {
        self.dialect
    }

    /// 構文の方言を決めます.
    pub fn set_dialect(&mut self, dialect: Dialect) {
        self.dialect = dialect;
    }

    /// 書き換えたか.
    pub fn modified(&self) -> bool {
        self.modified
    }

    /// 書き換えたかを決めます (保存したときは `false`).
    pub fn set_modified(&mut self, modified: bool) {
        self.modified = modified;
    }

    /// 本文の長さ (文字の数. Emacs の `buffer-size`).
    pub fn len(&self) -> usize {
        self.text.len_chars()
    }

    /// 本文が空か.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 本文の全体.
    pub fn text(&self) -> String {
        self.text.to_string()
    }

    /// 範囲の本文 (Emacs の `buffer-substring`). 始めと終わりは, 逆でもよい.
    ///
    /// # Errors
    ///
    /// 範囲がバッファの外のとき.
    pub fn substring(&self, from: usize, to: usize) -> Result<String, String> {
        let (from, to) = self.range(from, to)?;
        Ok(self.text.slice(from..to).to_string())
    }

    /// カーソルの位置.
    pub fn point(&self) -> usize {
        self.point
    }

    /// カーソルを置きます (バッファの外なら, 端に置く).
    pub fn set_point(&mut self, pos: usize) {
        self.point = pos.min(self.len());
    }

    /// 範囲を, 始めが小さい順にして, バッファの中か確かめます.
    fn range(&self, from: usize, to: usize) -> Result<(usize, usize), String> {
        let (from, to) = if from <= to { (from, to) } else { (to, from) };
        if to > self.len() {
            return Err("Args out of range".to_string());
        }
        Ok((from, to))
    }

    /// 行の数 (最後の改行の後ろの, 空の行も 1 行とする).
    pub fn line_count(&self) -> usize {
        self.text.len_lines()
    }

    /// 行 (0 から数える) の文字. 終わりの改行は含めない. 行が無ければ空.
    pub fn line_text(&self, line: usize) -> String {
        if line >= self.line_count() {
            return String::new();
        }
        let text = self.text.line(line).to_string();
        text.strip_suffix('\n').unwrap_or(&text).to_string()
    }

    /// 位置のある行の番号 (0 から数える).
    pub fn line_of_position(&self, pos: usize) -> usize {
        self.line_of(pos.min(self.len()))
    }

    /// 位置の文字 (Emacs の `char-after`). バッファの外なら `None`.
    pub fn char_after(&self, pos: usize) -> Option<char> {
        (pos < self.len()).then(|| self.text.char(pos))
    }

    /// 位置の前の文字 (Emacs の `char-before`). 無ければ `None`.
    pub fn char_before(&self, pos: usize) -> Option<char> {
        (pos > 0 && pos <= self.len()).then(|| self.text.char(pos - 1))
    }

    /// カーソルがバッファの始めか (Emacs の `bobp`).
    pub fn bobp(&self) -> bool {
        self.point == 0
    }

    /// カーソルがバッファの終わりか (Emacs の `eobp`).
    pub fn eobp(&self) -> bool {
        self.point == self.len()
    }

    /// カーソルが行の始めか (Emacs の `bolp`).
    pub fn bolp(&self) -> bool {
        self.point == 0 || self.char_before(self.point) == Some('\n')
    }

    /// カーソルが行の終わりか (Emacs の `eolp`).
    pub fn eolp(&self) -> bool {
        self.eobp() || self.char_after(self.point) == Some('\n')
    }

    /// カーソルを文字の数だけ動かします (Emacs の `forward-char`). 負なら後ろへ.
    ///
    /// # Errors
    ///
    /// 端を越えるとき. カーソルは端まで動かしてから, 失敗にする ("Beginning of buffer" か "End of buffer").
    pub fn forward_char(&mut self, n: i64) -> Result<(), String> {
        let target = self.point as i64 + n;
        if target < 0 {
            self.point = 0;
            return Err("Beginning of buffer".to_string());
        }
        if target > self.len() as i64 {
            self.point = self.len();
            return Err("End of buffer".to_string());
        }
        self.point = target as usize;
        Ok(())
    }

    /// 位置のある行の番号 (0 から).
    fn line_of(&self, pos: usize) -> usize {
        self.text.char_to_line(pos)
    }

    /// 行の始めの位置 (番号が行の数以上なら, バッファの終わり).
    fn line_start(&self, line: usize) -> usize {
        if line >= self.text.len_lines() {
            self.len()
        } else {
            self.text.line_to_char(line)
        }
    }

    /// 今の行から `n - 1` 行先の行の番号 (行の外の前なら `None`).
    fn relative_line(&self, n: i64) -> Option<usize> {
        let line = self.line_of(self.point) as i64 + n - 1;
        (line >= 0).then_some(line as usize)
    }

    /// 行の始めの位置 (Emacs の `line-beginning-position`). `n` が 1 なら今の行, 2 なら次の行, 0 なら前の行.
    /// 前に行が無ければバッファの始め, 後ろに行が無ければバッファの終わり.
    pub fn line_beginning_position(&self, n: i64) -> usize {
        self.relative_line(n)
            .map_or(0, |line| self.line_start(line))
    }

    /// 行の終わりの位置 (Emacs の `line-end-position`). `n` の数え方は `line_beginning_position` と同じ.
    pub fn line_end_position(&self, n: i64) -> usize {
        match self.relative_line(n) {
            None => 0,
            Some(line) if line + 1 >= self.text.len_lines() => self.len(),
            Some(line) => self.line_start(line + 1) - 1,
        }
    }

    /// 行を動きます (Emacs の `forward-line`). カーソルは行の始めに置きます.
    ///
    /// # Returns
    ///
    /// 動けなかった行の数 (前へ動けなかったときは負). 終わりに改行の無い行は, 動けた 1 行と数える.
    pub fn forward_line(&mut self, n: i64) -> i64 {
        if n <= 0 {
            let mut pos = self.line_beginning_position(1);
            let mut moved = 0;
            while moved < -n && pos > 0 {
                pos = self.line_start(self.line_of(pos - 1));
                moved += 1;
            }
            self.point = pos;
            return n + moved;
        }
        let mut pos = self.point;
        let mut moved = 0;
        while moved < n {
            match self.next_newline(pos) {
                Some(newline) => pos = newline + 1,
                None => {
                    // 終わりに改行の無い行に文字があれば, 1 行動いたと数える
                    if pos < self.len() {
                        moved += 1;
                    }
                    pos = self.len();
                    break;
                }
            }
            moved += 1;
        }
        self.point = pos;
        n - moved
    }

    /// 位置以降で, 最初の改行の位置.
    fn next_newline(&self, pos: usize) -> Option<usize> {
        let line = self.line_of(pos);
        let end = self.line_start(line + 1);
        if line + 1 < self.text.len_lines() {
            Some(end - 1)
        } else {
            None
        }
    }

    /// カーソルの桁 (Emacs の `current-column`). タブは `TAB_WIDTH` の倍数まで, 幅の広い文字は 2.
    pub fn current_column(&self) -> usize {
        self.column_of(self.point)
    }

    /// 位置の桁.
    fn column_of(&self, pos: usize) -> usize {
        let start = self.line_start(self.line_of(pos));
        self.text
            .slice(start..pos)
            .chars()
            .fold(0, Self::advance_column)
    }

    /// 文字を 1 つ進めた後の桁.
    fn advance_column(column: usize, c: char) -> usize {
        if c == '\t' {
            (column / TAB_WIDTH + 1) * TAB_WIDTH
        } else {
            column + char_width(c)
        }
    }

    /// 桁まで動きます (Emacs の `move-to-column`). 行の終わりを越えては動かない.
    ///
    /// # Returns
    ///
    /// 動いた先の桁 (タブや幅の広い文字で, 指定の桁を越えることがある).
    pub fn move_to_column(&mut self, column: usize) -> usize {
        let mut pos = self.line_beginning_position(1);
        let mut current = 0;
        while current < column {
            match self.char_after(pos) {
                Some(c) if c != '\n' => {
                    current = Self::advance_column(current, c);
                    pos += 1;
                }
                _ => break,
            }
        }
        self.point = pos;
        current
    }

    /// 位置の記録 (カーソル, マーク, マークの履歴) を, 入れた文字の分だけずらします.
    fn shift_for_insert(&mut self, pos: usize, len: usize) {
        let shift = |p: &mut usize| {
            if *p > pos {
                *p += len;
            }
        };
        shift(&mut self.point);
        self.mark.iter_mut().for_each(shift);
        self.mark_ring.iter_mut().for_each(shift);
        self.markers.iter_mut().flatten().for_each(shift);
    }

    /// 位置の記録を, 消した範囲の分だけずらします.
    fn shift_for_delete(&mut self, from: usize, to: usize) {
        let shift = |p: &mut usize| {
            if *p >= to {
                *p -= to - from;
            } else if *p > from {
                *p = from;
            }
        };
        shift(&mut self.point);
        self.mark.iter_mut().for_each(shift);
        self.mark_ring.iter_mut().for_each(shift);
        self.markers.iter_mut().flatten().for_each(shift);
    }

    /// 区切りの直後の変更の前に、区切りでのカーソルを履歴に覚えます (変更の所と同じなら, 要らない).
    fn record_point(&mut self, beginning: usize) {
        let at_boundary = matches!(self.undo.last(), None | Some(UndoEntry::Boundary));
        if let (true, Some(point)) = (at_boundary, self.point_at_boundary) {
            if point != beginning {
                self.undo.push(UndoEntry::Point(point));
            }
        }
    }

    /// 履歴に, 入れた文字を記録します (続けて入れたら, 1 つにまとめる).
    fn record_insert(&mut self, pos: usize, len: usize) {
        if let Some(UndoEntry::Insert { pos: last, len: n }) = self.undo.last_mut() {
            if *last + *n == pos {
                *n += len;
                return;
            }
        }
        self.record_point(pos);
        self.undo.push(UndoEntry::Insert { pos, len });
    }

    /// 位置に文字を入れます. 位置より後ろのカーソルとマークはずれ, 履歴に記録します.
    /// カーソルが位置と同じなら動かしません.
    ///
    /// # Errors
    ///
    /// 位置がバッファの外のとき.
    pub fn insert_at(&mut self, pos: usize, text: &str) -> Result<(), String> {
        if pos > self.len() {
            return Err("Args out of range".to_string());
        }
        let len = text.chars().count();
        if len == 0 {
            return Ok(());
        }
        self.text.insert(pos, text);
        self.deactivate_requested = true;
        self.shift_for_insert(pos, len);
        self.record_insert(pos, len);
        self.modified = true;
        Ok(())
    }

    /// カーソルの位置に文字を入れて, カーソルを入れた文字の後ろへ動かします (Emacs の `insert`).
    pub fn insert(&mut self, text: &str) {
        let pos = self.point;
        // カーソルの位置は, バッファの中にある
        let _ = self.insert_at(pos, text);
        self.point = pos + text.chars().count();
    }

    /// 範囲を消して, 消した文字を返します (Emacs の `delete-region`). 始めと終わりは, 逆でもよい.
    ///
    /// # Errors
    ///
    /// 範囲がバッファの外のとき.
    pub fn delete_region(&mut self, from: usize, to: usize) -> Result<String, String> {
        let (from, to) = self.range(from, to)?;
        if from == to {
            return Ok(String::new());
        }
        let text = self.text.slice(from..to).to_string();
        self.text.remove(from..to);
        self.deactivate_requested = true;
        self.record_point(from);
        self.undo.push(UndoEntry::Delete {
            pos: from,
            text: text.clone(),
            point_at_end: self.point == to,
        });
        self.shift_for_delete(from, to);
        self.modified = true;
        Ok(text)
    }

    /// 位置を覚えるマーカーを作ります (Emacs の `point-marker`). 本文を書き換えると, 位置を追う
    /// (入れた所と同じ位置なら, 動かない. 消した範囲の中なら, 消した所の始め).
    ///
    /// # Returns
    ///
    /// マーカーの番号.
    pub fn marker_new(&mut self, pos: usize) -> usize {
        let pos = pos.min(self.len());
        if let Some(free) = self.markers.iter().position(Option::is_none) {
            self.markers[free] = Some(pos);
            return free;
        }
        self.markers.push(Some(pos));
        self.markers.len() - 1
    }

    /// マーカーの位置. 解いたマーカーか, 無いマーカーなら `None`.
    pub fn marker_position(&self, id: usize) -> Option<usize> {
        self.markers.get(id).copied().flatten()
    }

    /// マーカーを解きます.
    pub fn marker_release(&mut self, id: usize) {
        if let Some(slot) = self.markers.get_mut(id) {
            *slot = None;
        }
    }

    /// マークの位置 (Emacs の `mark`).
    pub fn mark(&self) -> Option<usize> {
        self.mark
    }

    /// マークを置いて, 使う状態にします (Emacs の `set-mark`).
    pub fn set_mark(&mut self, pos: usize) {
        self.mark = Some(pos.min(self.len()));
        self.mark_active = true;
    }

    /// マークの位置だけを決めます (Emacs の `(set-marker (mark-marker) POS)`. 使う状態は変えない).
    pub fn set_mark_position(&mut self, pos: usize) {
        self.mark = Some(pos.min(self.len()));
    }

    /// マークを使う状態にします (Emacs の `activate-mark`). マークが無ければ, 何もしない.
    pub fn activate_mark(&mut self) {
        self.mark_active = self.mark.is_some();
    }

    /// マークを使う状態か (Emacs の変数 `mark-active`).
    pub fn mark_active(&self) -> bool {
        self.mark_active
    }

    /// コマンドの終わりにマークを外すよう頼まれているか (Emacs の変数 `deactivate-mark`).
    pub fn deactivate_requested(&self) -> bool {
        self.deactivate_requested
    }

    /// コマンドの終わりにマークを外すよう頼みます (Emacs の `(setq deactivate-mark t)`. 外す印は, nil で消す).
    pub fn set_deactivate_requested(&mut self, requested: bool) {
        self.deactivate_requested = requested;
    }

    /// マークを使わない状態にします (Emacs の `deactivate-mark`). マークの位置は残す.
    pub fn deactivate_mark(&mut self) {
        self.mark_active = false;
    }

    /// 今のマークを履歴に入れて, 新しいマークを置きます (Emacs の `push-mark`).
    ///
    /// # Arguments
    ///
    /// * `pos` - 新しいマークの位置.
    /// * `activate` - マークを使う状態にするか.
    pub fn push_mark(&mut self, pos: usize, activate: bool) {
        if let Some(old) = self.mark {
            self.mark_ring.insert(0, old);
            self.mark_ring.truncate(MARK_RING_MAX);
        }
        self.mark = Some(pos.min(self.len()));
        self.mark_active = activate;
    }

    /// マークの履歴の先頭を, マークにします (Emacs の `pop-mark`). 今のマークは履歴の最後に回す.
    /// 履歴が空なら, 何もしない. マークは使わない状態になる.
    pub fn pop_mark(&mut self) {
        if self.mark_ring.is_empty() {
            return;
        }
        let top = self.mark_ring.remove(0);
        if let Some(old) = self.mark {
            self.mark_ring.push(old);
        }
        self.mark = Some(top);
        self.mark_active = false;
    }

    /// リージョンの始め (Emacs の `region-beginning`).
    ///
    /// # Errors
    ///
    /// マークが無いとき.
    pub fn region_beginning(&self) -> Result<usize, String> {
        self.region().map(|(from, _)| from)
    }

    /// リージョンの終わり (Emacs の `region-end`).
    ///
    /// # Errors
    ///
    /// マークが無いとき.
    pub fn region_end(&self) -> Result<usize, String> {
        self.region().map(|(_, to)| to)
    }

    /// リージョン (カーソルとマークの間).
    fn region(&self) -> Result<(usize, usize), String> {
        let mark = self
            .mark
            .ok_or("The mark is not set now, so there is no region")?;
        Ok((self.point.min(mark), self.point.max(mark)))
    }

    /// マークを使う状態か (Emacs の `region-active-p`. リージョンが空でもよい).
    pub fn region_active(&self) -> bool {
        self.mark_active && self.mark.is_some()
    }

    /// リージョンを使うか (Emacs の `use-region-p`). マークを使う状態で, リージョンが空でないとき.
    pub fn use_region_p(&self) -> bool {
        self.mark_active && self.mark.is_some_and(|mark| mark != self.point)
    }

    /// 元に戻す履歴.
    pub fn undo_list(&self) -> &[UndoEntry] {
        &self.undo
    }

    /// 元に戻す履歴を置き換えます (Emacs の `(setq buffer-undo-list ...)`. `undo-redo` が履歴を巻き戻すときに使う).
    pub fn set_undo_list(&mut self, list: Vec<UndoEntry>) {
        self.undo = list;
    }

    /// 元に戻すまとまりの区切りを入れます (Emacs の `undo-boundary`). 履歴が空か, 既に区切りなら入れない.
    pub fn undo_boundary(&mut self) {
        // 区切りでのカーソルは、履歴が空でも覚える
        self.point_at_boundary = Some(self.point);
        if !matches!(self.undo.last(), None | Some(UndoEntry::Boundary)) {
            self.undo.push(UndoEntry::Boundary);
        }
    }

    /// 履歴の `end` (その手前まで) から前へ, `n` つのまとまりを取り出します.
    ///
    /// # Arguments
    ///
    /// * `end` - 取り出す範囲の終わり (履歴の長さから始めて, 続けて元に戻すときは, 前に返した値).
    /// * `n` - まとまりの数.
    ///
    /// # Returns
    ///
    /// 取り出した記録 (新しいものが先. 区切りは除く) と, 次の `end`.
    pub fn undo_groups(&self, end: usize, n: usize) -> (Vec<UndoEntry>, usize) {
        let mut end = end.min(self.undo.len());
        let mut entries = Vec::new();
        for _ in 0..n {
            while end > 0 && self.undo[end - 1] == UndoEntry::Boundary {
                end -= 1;
            }
            while end > 0 && self.undo[end - 1] != UndoEntry::Boundary {
                end -= 1;
                entries.push(self.undo[end].clone());
            }
        }
        (entries, end)
    }

    /// 記録を元に戻します (Emacs の `primitive-undo`). 記録は新しいものが先の順に渡す.
    /// 元に戻すことも, 履歴に記録する (元に戻すことを, 元に戻せる).
    ///
    /// # Errors
    ///
    /// 記録の位置が, 今の本文に合わないとき.
    pub fn primitive_undo(&mut self, entries: &[UndoEntry]) -> Result<(), String> {
        for entry in entries {
            match entry {
                UndoEntry::Boundary => {}
                UndoEntry::Point(pos) => self.point = (*pos).min(self.len()),
                UndoEntry::Insert { pos, len } => {
                    // Emacs は, 先にカーソルを置く (消した記録の, カーソルの印が, 始めになる)
                    self.point = (*pos).min(self.len());
                    self.delete_region(*pos, pos + len)?;
                    self.point = *pos;
                }
                UndoEntry::Delete {
                    pos,
                    text,
                    point_at_end,
                } => {
                    self.insert_at(*pos, text)?;
                    self.point = if *point_at_end {
                        pos + text.chars().count()
                    } else {
                        *pos
                    };
                }
            }
        }
        Ok(())
    }
}
