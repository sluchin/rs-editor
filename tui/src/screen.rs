//! 画面 (端末) の窓口: 描くことと, キーを読むこと. ncurses の実装は `ncurses_screen.rs`, テストでは偽物を使う.

/// 画面の 1 マス.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    /// 文字. `'\0'` は, 幅の広い文字の 2 マス目.
    pub ch: char,
    /// 反転して描くか (モードライン).
    pub reverse: bool,
}

impl Cell {
    /// 空白のマス.
    pub const BLANK: Cell = Cell {
        ch: ' ',
        reverse: false,
    };
}

/// 画面の全体.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    /// マス (行ごと).
    pub cells: Vec<Vec<Cell>>,
    /// カーソルの位置 (行, 桁).
    pub cursor: (usize, usize),
}

impl Frame {
    /// 空白で埋めた画面を作ります.
    pub fn blank(rows: usize, cols: usize) -> Self {
        Frame {
            cells: vec![vec![Cell::BLANK; cols]; rows],
            cursor: (0, 0),
        }
    }

    /// 行の文字 (幅の広い文字の 2 マス目は除き, 終わりの空白も除く).
    pub fn line(&self, row: usize) -> String {
        let text: String = self.cells[row]
            .iter()
            .filter(|c| c.ch != '\0')
            .map(|c| c.ch)
            .collect();
        text.trim_end().to_string()
    }

    /// 全ての行の文字.
    pub fn lines(&self) -> Vec<String> {
        (0..self.cells.len()).map(|row| self.line(row)).collect()
    }
}

/// 画面.
pub trait Screen: Send {
    /// 画面の大きさ (行の数, 桁の数).
    fn size(&self) -> (usize, usize);

    /// 画面を描きます.
    fn draw(&mut self, frame: &Frame);

    /// キーを 1 つ読みます (Emacs の書き方. `C-f`, `M-x`, `RET`, `<up>` など).
    ///
    /// # Returns
    ///
    /// キー. 入力が終わったら `None`.
    fn read_key(&mut self) -> Option<String>;
}
