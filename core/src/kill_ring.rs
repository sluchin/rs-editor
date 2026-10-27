//! kill-ring: 切り取った文字の履歴 (Emacs の `kill-ring` と `kill-ring-yank-pointer`).

/// kill-ring の長さの上限 (Emacs の `kill-ring-max`).
pub const KILL_RING_MAX: usize = 120;

/// kill-ring. 新しいものが先.
#[derive(Debug, Clone, Default)]
pub struct KillRing {
    /// 切り取った文字 (新しいものが先).
    items: Vec<String>,
    /// 次に貼る (yank する) もの (`items` の番号).
    pointer: usize,
}

impl KillRing {
    /// 空の kill-ring を作ります.
    pub fn new() -> Self {
        KillRing::default()
    }

    /// 入っている数.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// 空か.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// kill-ring の先頭 (一番新しい kill. Emacs の `(car kill-ring)`). 次に貼るものとは、限らない.
    pub fn head(&self) -> Option<&str> {
        self.items.first().map(String::as_str)
    }

    /// 文字を kill-ring に入れます (Emacs の `kill-new`). 入れたものが次に貼るものになる.
    ///
    /// # Arguments
    ///
    /// * `text` - 文字.
    /// * `replace` - 先頭のものを置き換えるか (空でないとき).
    pub fn kill_new(&mut self, text: &str, replace: bool) {
        if replace && !self.items.is_empty() {
            self.items[0] = text.to_string();
        } else {
            self.items.insert(0, text.to_string());
            self.items.truncate(KILL_RING_MAX);
        }
        self.pointer = 0;
    }

    /// 先頭のものに文字を足します (Emacs の `kill-append`). 空なら, 新しく入れる.
    ///
    /// # Arguments
    ///
    /// * `text` - 足す文字.
    /// * `before` - 前に足すか (後ろ向きに切ったとき).
    pub fn kill_append(&mut self, text: &str, before: bool) {
        match self.items.first() {
            None => self.kill_new(text, false),
            Some(first) => {
                let joined = if before {
                    format!("{text}{first}")
                } else {
                    format!("{first}{text}")
                };
                self.kill_new(&joined, true);
            }
        }
    }

    /// 次に貼るものから `n` つ先 (古い方) のものを返します (Emacs の `current-kill`). 一周する.
    ///
    /// # Arguments
    ///
    /// * `n` - 進める数 (負なら新しい方).
    /// * `do_not_move` - 真なら, 次に貼るものを動かさない.
    ///
    /// # Errors
    ///
    /// kill-ring が空のとき.
    pub fn current_kill(&mut self, n: i64, do_not_move: bool) -> Result<String, String> {
        if self.items.is_empty() {
            return Err("Kill ring is empty".to_string());
        }
        let len = self.items.len() as i64;
        let index = (self.pointer as i64 + n).rem_euclid(len) as usize;
        if !do_not_move {
            self.pointer = index;
        }
        Ok(self.items[index].clone())
    }
}
