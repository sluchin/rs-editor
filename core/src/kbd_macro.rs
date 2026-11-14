//! キーボードマクロ (Emacs の `macro.c`. `C-x (`, `C-x )`, `C-x e`). 打ったキーを覚え, 後で同じキーを打ったことにします.
//! キーを読む画面が, 打ったキーを [`KbdMacro::record`] で渡し, 覚えたキーを打ち直します.

/// キーボードマクロの状態.
#[derive(Debug, Clone, Default)]
pub struct KbdMacro {
    /// 覚えているところか (`defining-kbd-macro`).
    defining: bool,
    /// 覚えているキー.
    current: Vec<String>,
    /// 今のコマンドの始めのキーの位置 (`current` の中). マクロを終えるコマンドのキーを除くのに使う.
    command_start: usize,
    /// 最後に覚え終えたマクロ (`last-kbd-macro`).
    last: Vec<String>,
}

impl KbdMacro {
    /// マクロを覚え始めます (Emacs の `start-kbd-macro`).
    ///
    /// # Arguments
    ///
    /// * `append` - 真なら, 最後のマクロの続きに覚える.
    ///
    /// # Errors
    ///
    /// 既に覚えているとき.
    pub fn start(&mut self, append: bool) -> Result<(), String> {
        if self.defining {
            return Err("Already defining kbd macro".to_string());
        }
        self.current = if append {
            self.last.clone()
        } else {
            Vec::new()
        };
        self.command_start = self.current.len();
        self.defining = true;
        Ok(())
    }

    /// マクロを覚え終えます (Emacs の `end-kbd-macro`). 終えるコマンドのキーは, マクロに入れない.
    ///
    /// # Errors
    ///
    /// 覚えていないとき.
    pub fn end(&mut self) -> Result<(), String> {
        if !self.defining {
            return Err("Not defining kbd macro".to_string());
        }
        self.current.truncate(self.command_start);
        self.last = std::mem::take(&mut self.current);
        self.defining = false;
        Ok(())
    }

    /// 覚えているところならキーを覚えます.
    pub fn record(&mut self, key: &str) {
        if self.defining {
            self.current.push(key.to_string());
        }
    }

    /// 新しいコマンドのキーが始まることを知らせます (マクロを終えるコマンドのキーを除くため).
    pub fn begin_command(&mut self) {
        self.command_start = self.current.len();
    }

    /// 覚えているところか.
    pub fn defining(&self) -> bool {
        self.defining
    }

    /// 最後に覚え終えたマクロのキー.
    pub fn last(&self) -> &[String] {
        &self.last
    }
}
