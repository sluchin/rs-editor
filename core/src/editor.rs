//! エディタの状態: バッファの一覧, 今のバッファ, ウィンドウ, kill-ring, 前置引数.
//!
//! Emacs の C の関数に当たるもの (`get-buffer`, `set-buffer`, `generate-new-buffer`, `split-window`,
//! `select-window`, `set-window-buffer`, `kill-buffer`, `prefix-numeric-value` など) を持ちます.
//! バッファは名前で引きます (名前は重ならない). 画面にもターミナルにも頼みません.

use std::collections::BTreeMap;

use crate::buffer::{Buffer, Groups};
use crate::kbd_macro::KbdMacro;
use crate::kill_ring::KillRing;
use crate::window::{Direction, Layout, FULL};

/// 前置引数 (Emacs の `current-prefix-arg`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrefixArg {
    /// `C-u -` の `-`.
    Minus,
    /// `C-u` を押した回数に応じた 4 のべき乗 (`(4)`, `(16)`).
    Universal(i64),
    /// 数 (`C-u 5`, `M-5`).
    Number(i64),
}

/// 前置引数の数としての値 (Emacs の `prefix-numeric-value`). 無ければ 1.
pub fn prefix_numeric_value(arg: Option<PrefixArg>) -> i64 {
    match arg {
        None => 1,
        Some(PrefixArg::Minus) => -1,
        Some(PrefixArg::Universal(n) | PrefixArg::Number(n)) => n,
    }
}

/// エディタ.
#[derive(Debug, Clone)]
pub struct Editor {
    /// バッファ (開いた順).
    buffers: Vec<Buffer>,
    /// 今のバッファの名前 (Emacs の `current-buffer`. 選んだウィンドウのバッファと違うことがある).
    current: String,
    /// ウィンドウの分け方.
    layout: Layout,
    /// ウィンドウごとのバッファの名前.
    window_buffers: BTreeMap<usize, String>,
    /// 選んでいるウィンドウ.
    selected: usize,
    /// 次に作るウィンドウの番号.
    next_window: usize,
    /// 画面の大きさ (行, 桁). 窓の大きさの計算に使う.
    screen: (usize, usize),
    /// kill-ring.
    pub kill_ring: KillRing,
    /// キーボードマクロ.
    pub kbd_macro: KbdMacro,
    /// キーボードマクロを動かしているか (`executing-kbd-macro`. 動かしている間は, "Mark set" などを知らせない).
    pub executing_kbd_macro: bool,
    /// 前に自動保存してから読んだキーの数 (`files::note_key` が数える).
    pub keys_since_auto_save: usize,
    /// 前置引数.
    pub prefix_arg: Option<PrefixArg>,
    /// バッファをまたぐマークの履歴 (Emacs の `global-mark-ring`. 新しいものが先. バッファの名前と, そのバッファのマーカー).
    pub global_mark_ring: Vec<(String, usize)>,
    /// 覚えたウィンドウの設定 (Emacs の `current-window-configuration`. 番号で引く).
    window_configurations: Vec<WindowConfiguration>,
    /// 文字列に `string-match` した結果の括弧の範囲 (文字の位置. 0 から)と, 決めた順番.
    pub string_match: Option<(Groups, u64)>,
}

/// ウィンドウの設定: 分け方と, ウィンドウごとのバッファと, 選んでいるウィンドウ.
#[derive(Debug, Clone)]
struct WindowConfiguration {
    /// ウィンドウの分け方.
    layout: Layout,
    /// ウィンドウごとのバッファの名前.
    window_buffers: BTreeMap<usize, String>,
    /// 選んでいるウィンドウ.
    selected: usize,
}

/// `global-mark-ring` の長さの上限 (Emacs の `global-mark-ring-max`).
pub const GLOBAL_MARK_RING_MAX: usize = 16;

/// 始めのバッファの名前.
pub const SCRATCH: &str = "*scratch*";

impl Default for Editor {
    fn default() -> Self {
        Editor::new()
    }
}

impl Editor {
    /// `*scratch*` だけを開いた, ウィンドウが 1 つのエディタを作ります.
    pub fn new() -> Self {
        Editor {
            buffers: vec![Buffer::new(SCRATCH)],
            current: SCRATCH.to_string(),
            layout: Layout::Window(1),
            window_buffers: BTreeMap::from([(1, SCRATCH.to_string())]),
            selected: 1,
            next_window: 2,
            screen: (24, 80),
            kill_ring: KillRing::new(),
            kbd_macro: KbdMacro::default(),
            executing_kbd_macro: false,
            keys_since_auto_save: 0,
            prefix_arg: None,
            global_mark_ring: Vec::new(),
            window_configurations: Vec::new(),
            string_match: None,
        }
    }

    /// 直前に合った所 (Emacs の `match-data`. 全体で 1 つ): 文字列を探した結果と, バッファごとの結果のうち, 一番新しいもの.
    ///
    /// # Returns
    ///
    /// 括弧ごとの範囲 (文字の位置, 0 から) と, 文字列の結果なら `true`.
    pub fn latest_match(&self) -> (Groups, bool) {
        let current = self.current_buffer();
        let newest = self.buffers.iter().max_by_key(|b| b.match_seq());
        let newest = newest.unwrap_or(current);
        match &self.string_match {
            Some((data, seq)) if *seq > newest.match_seq() => (data.clone(), true),
            _ => (newest.match_data().to_vec(), false),
        }
    }

    /// バッファの名前 (開いた順. Emacs の `buffer-list`).
    pub fn buffer_list(&self) -> Vec<String> {
        self.buffers.iter().map(|b| b.name().to_string()).collect()
    }

    /// 名前のバッファ (Emacs の `get-buffer`).
    pub fn get_buffer(&self, name: &str) -> Option<&Buffer> {
        self.buffers.iter().find(|b| b.name() == name)
    }

    /// 名前のバッファ (書き換える).
    pub fn get_buffer_mut(&mut self, name: &str) -> Option<&mut Buffer> {
        self.buffers.iter_mut().find(|b| b.name() == name)
    }

    /// 名前のバッファを返します. 無ければ作ります (Emacs の `get-buffer-create`).
    pub fn get_buffer_create(&mut self, name: &str) -> &mut Buffer {
        if self.get_buffer(name).is_none() {
            self.buffers.push(Buffer::new(name));
        }
        self.get_buffer_mut(name).unwrap()
    }

    /// 重ならない名前のバッファを作ります (Emacs の `generate-new-buffer`). 重なれば `<2>`, `<3>` を付ける.
    ///
    /// # Returns
    ///
    /// 作ったバッファの名前.
    pub fn generate_new_buffer(&mut self, name: &str) -> String {
        let mut candidate = name.to_string();
        let mut n = 2;
        while self.get_buffer(&candidate).is_some() {
            candidate = format!("{name}<{n}>");
            n += 1;
        }
        self.buffers.push(Buffer::new(&candidate));
        candidate
    }

    /// 使われていないバッファの名前を返します (Emacs の `generate-new-buffer-name`). 使われていれば `<2>`, `<3>` を付ける.
    ///
    /// # Arguments
    ///
    /// * `name` - 欲しい名前.
    /// * `ignore` - 使われていないものとして扱う名前.
    pub fn generate_new_buffer_name(&self, name: &str, ignore: Option<&str>) -> String {
        let free =
            |candidate: &str| Some(candidate) == ignore || self.get_buffer(candidate).is_none();
        let mut candidate = name.to_string();
        let mut n = 2;
        while !free(&candidate) {
            candidate = format!("{name}<{n}>");
            n += 1;
        }
        candidate
    }

    /// 今のバッファの名前を変えます (Emacs の `rename-buffer`).
    ///
    /// # Arguments
    ///
    /// * `new` - 新しい名前.
    /// * `unique` - 真なら, 使われていれば `<2>` などを付ける. 偽なら失敗する.
    ///
    /// # Returns
    ///
    /// 付けた名前.
    ///
    /// # Errors
    ///
    /// 名前が空のとき, または使われているとき.
    pub fn rename_buffer(&mut self, new: &str, unique: bool) -> Result<String, String> {
        if new.is_empty() {
            return Err("Empty string is invalid as a buffer name".to_string());
        }
        let old = self.current.clone();
        let new = if unique {
            self.generate_new_buffer_name(new, Some(&old))
        } else {
            new.to_string()
        };
        if new != old && self.get_buffer(&new).is_some() {
            return Err(format!("Buffer name ‘{new}’ is in use"));
        }
        let rename = |name: &mut String| {
            if *name == old {
                *name = new.clone();
            }
        };
        self.current_buffer_mut().set_name(&new);
        self.current = new.clone();
        self.window_buffers.values_mut().for_each(rename);
        for configuration in &mut self.window_configurations {
            configuration.window_buffers.values_mut().for_each(rename);
        }
        for (name, _) in &mut self.global_mark_ring {
            rename(name);
        }
        Ok(new)
    }

    /// 今のバッファの名前.
    pub fn current_buffer_name(&self) -> &str {
        &self.current
    }

    /// 今のバッファ.
    pub fn current_buffer(&self) -> &Buffer {
        // 今のバッファは, 必ず開いている
        self.get_buffer(&self.current).unwrap()
    }

    /// 今のバッファ (書き換える).
    pub fn current_buffer_mut(&mut self) -> &mut Buffer {
        let name = self.current.clone();
        self.get_buffer_mut(&name).unwrap()
    }

    /// 今のバッファを変えます (Emacs の `set-buffer`). ウィンドウは変えない.
    ///
    /// # Errors
    ///
    /// バッファが無いとき.
    pub fn set_buffer(&mut self, name: &str) -> Result<(), String> {
        if self.get_buffer(name).is_none() {
            return Err(format!("No such buffer: {name}"));
        }
        self.current = name.to_string();
        Ok(())
    }

    /// バッファを今のバッファにして, `f` を動かします (Emacs の `with-current-buffer`). 動かした後は, 元の今のバッファに戻す.
    ///
    /// # Errors
    ///
    /// バッファが無いとき.
    pub fn with_buffer<T>(
        &mut self,
        name: &str,
        f: impl FnOnce(&mut Editor) -> T,
    ) -> Result<T, String> {
        let saved = self.current.clone();
        self.set_buffer(name)?;
        let result = f(self);
        // 動かしている間に消えていたら, 戻さない
        if self.get_buffer(&saved).is_some() {
            self.current = saved;
        }
        Ok(result)
    }

    /// バッファを消します (Emacs の `kill-buffer`). 出していたウィンドウには, 別のバッファを出す.
    /// 最後のバッファなら, `*scratch*` を作る.
    ///
    /// # Errors
    ///
    /// バッファが無いとき.
    pub fn kill_buffer(&mut self, name: &str) -> Result<(), String> {
        let index = self
            .buffers
            .iter()
            .position(|b| b.name() == name)
            .ok_or_else(|| format!("No such buffer: {name}"))?;
        self.buffers.remove(index);
        if self.buffers.is_empty() {
            self.buffers.push(Buffer::new(SCRATCH));
        }
        // 前のバッファの次に開いたものを, 代わりに出す
        let other = self.buffers[index.min(self.buffers.len() - 1)]
            .name()
            .to_string();
        for shown in self.window_buffers.values_mut() {
            if shown == name {
                *shown = other.clone();
            }
        }
        if self.current == name {
            self.current = other;
        }
        Ok(())
    }

    /// 選んでいるウィンドウ (Emacs の `selected-window`).
    pub fn selected_window(&self) -> usize {
        self.selected
    }

    /// ウィンドウを左上から順に返します (Emacs の `window-list`).
    pub fn window_list(&self) -> Vec<usize> {
        self.layout.window_ids()
    }

    /// ウィンドウの分け方.
    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    /// ウィンドウが出しているバッファの名前 (Emacs の `window-buffer`).
    pub fn window_buffer(&self, window: usize) -> Option<&str> {
        self.window_buffers.get(&window).map(String::as_str)
    }

    /// ウィンドウにバッファを出します (Emacs の `set-window-buffer`). 今のバッファは変えない.
    ///
    /// # Errors
    ///
    /// ウィンドウかバッファが無いとき.
    pub fn set_window_buffer(&mut self, window: usize, name: &str) -> Result<(), String> {
        if self.get_buffer(name).is_none() {
            return Err(format!("No such buffer: {name}"));
        }
        let shown = self
            .window_buffers
            .get_mut(&window)
            .ok_or_else(|| format!("No such window: {window}"))?;
        *shown = name.to_string();
        Ok(())
    }

    /// ウィンドウを選びます (Emacs の `select-window`). 今のバッファは, そのウィンドウのバッファになる.
    ///
    /// # Errors
    ///
    /// ウィンドウが無いとき.
    pub fn select_window(&mut self, window: usize) -> Result<(), String> {
        let name = self
            .window_buffers
            .get(&window)
            .ok_or_else(|| format!("No such window: {window}"))?
            .clone();
        self.selected = window;
        self.current = name;
        Ok(())
    }

    /// 選んでいるウィンドウを 2 つに分けます (Emacs の `split-window`). 新しいウィンドウは, 同じバッファを出す.
    /// 選んでいるウィンドウは変わらない.
    ///
    /// # Returns
    ///
    /// 新しいウィンドウ.
    pub fn split_window(&mut self, direction: Direction) -> usize {
        let new = self.next_window;
        self.next_window += 1;
        let layout = std::mem::replace(&mut self.layout, Layout::Window(0));
        self.layout = layout.split(self.selected, new, direction);
        let shown = self.window_buffers[&self.selected].clone();
        self.window_buffers.insert(new, shown);
        new
    }

    /// ウィンドウを消します (Emacs の `delete-window`). 選んでいたウィンドウなら, 隣を選ぶ.
    ///
    /// # Errors
    ///
    /// ウィンドウが 1 つだけのとき, またはウィンドウが無いとき.
    pub fn delete_window(&mut self, window: usize) -> Result<(), String> {
        if !self.window_buffers.contains_key(&window) {
            return Err(format!("No such window: {window}"));
        }
        let ids = self.layout.window_ids();
        let layout = std::mem::replace(&mut self.layout, Layout::Window(0));
        match layout.delete(window) {
            None => {
                self.layout = Layout::Window(window);
                Err("Attempt to delete minibuffer or sole ordinary window".to_string())
            }
            Some(layout) => {
                self.layout = layout;
                self.window_buffers.remove(&window);
                if self.selected == window {
                    // 消したウィンドウの前 (無ければ次) のウィンドウを選ぶ
                    let index = ids.iter().position(|w| *w == window).unwrap_or(0);
                    let next = ids[if index > 0 { index - 1 } else { 1 }];
                    self.select_window(next)?;
                }
                Ok(())
            }
        }
    }

    /// 選んでいるウィンドウだけにします (Emacs の `delete-other-windows`).
    pub fn delete_other_windows(&mut self) {
        self.layout = Layout::Window(self.selected);
        let shown = self.window_buffers[&self.selected].clone();
        self.window_buffers = BTreeMap::from([(self.selected, shown)]);
    }

    /// 画面の大きさ (行, 桁) を知らせます. 窓の大きさを, 行と桁で数えるのに使う.
    pub fn set_screen_size(&mut self, rows: usize, cols: usize) {
        self.screen = (rows, cols);
    }

    /// ウィンドウの全体の高さ (行. モードラインを含む. 画面の最後の 1 行は, ミニバッファ).
    pub fn window_total_height(&self, window: usize) -> Option<usize> {
        let rows = self.screen.0.saturating_sub(1);
        self.layout
            .rects(FULL)
            .into_iter()
            .find(|r| r.id == window)
            .map(|r| to_cells(r.top + r.height, rows) - to_cells(r.top, rows))
    }

    /// ウィンドウの全体の幅 (桁).
    pub fn window_total_width(&self, window: usize) -> Option<usize> {
        let cols = self.screen.1;
        self.layout
            .rects(FULL)
            .into_iter()
            .find(|r| r.id == window)
            .map(|r| to_cells(r.left + r.width, cols) - to_cells(r.left, cols))
    }

    /// 画面の全体を 2 つに分けます (Emacs の `split-root-window`). 新しいウィンドウは, 下か右で, 選んでいるウィンドウと同じバッファを出す.
    /// 選んでいるウィンドウは変わらない.
    ///
    /// # Returns
    ///
    /// 新しいウィンドウ.
    pub fn split_root_window(&mut self, direction: Direction) -> usize {
        let new = self.next_window;
        self.next_window += 1;
        let layout = std::mem::replace(&mut self.layout, Layout::Window(0));
        self.layout = layout.split_root(new, direction);
        let shown = self.window_buffers[&self.selected].clone();
        self.window_buffers.insert(new, shown);
        new
    }

    /// 全てのウィンドウを同じ大きさにします (Emacs の `balance-windows`).
    pub fn balance_windows(&mut self) {
        self.layout.balance();
    }

    /// ウィンドウの大きさを変えます (Emacs の `enlarge-window`. 負の数なら小さくする).
    ///
    /// # Arguments
    ///
    /// * `window` - 大きさを変えるウィンドウ.
    /// * `delta` - 変える行数 (`horizontal` なら桁数).
    /// * `horizontal` - 幅を変えるか.
    ///
    /// # Errors
    ///
    /// ウィンドウが無いとき, またはその向きに変えられないとき.
    pub fn resize_window(
        &mut self,
        window: usize,
        delta: i64,
        horizontal: bool,
    ) -> Result<(), String> {
        if !self.window_buffers.contains_key(&window) {
            return Err(format!("No such window: {window}"));
        }
        let (direction, total) = if horizontal {
            (Direction::Right, self.screen.1)
        } else {
            (Direction::Below, self.screen.0.saturating_sub(1))
        };
        let percent = delta as f64 * 100.0 / total.max(1) as f64;
        if self.layout.resize(window, direction, percent, FULL) {
            Ok(())
        } else {
            Err(format!("Cannot resize window {window}"))
        }
    }

    /// 今のウィンドウの設定を覚えます (Emacs の `current-window-configuration`).
    ///
    /// # Returns
    ///
    /// 覚えた設定の番号.
    pub fn save_window_configuration(&mut self) -> usize {
        self.window_configurations.push(WindowConfiguration {
            layout: self.layout.clone(),
            window_buffers: self.window_buffers.clone(),
            selected: self.selected,
        });
        self.window_configurations.len() - 1
    }

    /// 覚えたウィンドウの設定に戻します (Emacs の `set-window-configuration`). 選ぶウィンドウのバッファが、今のバッファになる.
    /// その後に消したバッファを出していたウィンドウは、今のバッファを出す.
    ///
    /// # Errors
    ///
    /// 番号の設定が無いとき.
    pub fn restore_window_configuration(&mut self, id: usize) -> Result<(), String> {
        let config = self
            .window_configurations
            .get(id)
            .ok_or("Wrong type argument: window-configuration-p")?
            .clone();
        let fallback = self.current.clone();
        self.layout = config.layout;
        self.window_buffers = config
            .window_buffers
            .into_iter()
            .map(|(window, name)| {
                let name = if self.get_buffer(&name).is_some() {
                    name
                } else {
                    fallback.clone()
                };
                (window, name)
            })
            .collect();
        // 設定を覚えた後に作ったウィンドウの番号と、重ならない
        self.next_window = self
            .next_window
            .max(self.window_buffers.keys().max().map_or(1, |m| m + 1));
        self.select_window(config.selected)
    }

    /// 次のウィンドウを選びます (Emacs の `other-window` の 1). 最後の次は, 最初.
    pub fn other_window(&mut self, count: i64) {
        let ids = self.layout.window_ids();
        let index = ids.iter().position(|w| *w == self.selected).unwrap_or(0) as i64;
        let next = ids[(index + count).rem_euclid(ids.len() as i64) as usize];
        // 分け方にあるウィンドウは, 必ず出している
        let _ = self.select_window(next);
    }
}

/// 窓の枠の位置 (割合 %) を、画面のマスにします.
fn to_cells(percent: f64, total: usize) -> usize {
    (percent / 100.0 * total as f64).round() as usize
}
