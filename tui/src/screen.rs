//! 画面 (端末) の窓口: 描くことと, キーを読むこと. ncurses の実装は `ncurses_screen.rs`, テストでは偽物を使う.

use rseditor_core::highlight::TokenKind;

/// 画面の 1 マス.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    /// 文字. `'\0'` は, 幅の広い文字の 2 マス目.
    pub ch: char,
    /// 反転して描くか (モードライン).
    pub reverse: bool,
    /// 下線を引いて描くか (`highlight-regexp` で強調した所).
    pub underline: bool,
    /// シンタックスハイライトの字句の種類 (色は、`token_style` で決める).
    pub token: Option<TokenKind>,
}

/// 字句の色 (ncurses の色の番号: 0 黒, 1 赤, 2 緑, 3 黄, 4 青, 5 マゼンタ, 6 シアン, 7 白).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenStyle {
    /// 前景の色の番号.
    pub color: i16,
    /// 太字か.
    pub bold: bool,
}

/// 字句の種類の、ncurses の色の対の番号 (1 から. `TokenKind::ALL` の位置 + 1).
pub fn token_pair(kind: TokenKind) -> i16 {
    TokenKind::ALL
        .iter()
        .position(|k| *k == kind)
        .map_or(0, |i| i as i16 + 1)
}

/// 字句の種類の色. 8 色の端末で、暗い背景の Emacs の font-lock の色に合わせる
/// (コメントは黄, 文字列とドキュメントは緑, キーワードはシアンの太字, 型は緑, 定数はマゼンタ, プリプロセッサと関数の名前は青の太字,
/// 変数の名前は黄の太字).
///
/// # Returns
///
/// 色 (数値のように、色を付けない種類は `None`).
pub fn token_style(kind: TokenKind) -> Option<TokenStyle> {
    let (color, bold) = match kind {
        TokenKind::Comment | TokenKind::CommentDelimiter => (3, false),
        TokenKind::DocComment | TokenKind::String | TokenKind::Type => (2, false),
        TokenKind::Keyword => (6, true),
        TokenKind::Constant => (5, false),
        TokenKind::Preprocessor | TokenKind::FunctionName => (4, true),
        TokenKind::VariableName => (3, true),
        TokenKind::Number => return None,
    };
    Some(TokenStyle { color, bold })
}

impl Cell {
    /// 空白のマス.
    pub const BLANK: Cell = Cell {
        ch: ' ',
        reverse: false,
        underline: false,
        token: None,
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

    /// 待たずに、`C-g` を押したかを確かめます (キーボードマクロを繰り返している間に、止めるため).
    /// `C-g` なら、読んで捨てて `true`. ほかのキーなら、次の `read_key` が読めるように残して `false`.
    fn interrupted(&mut self) -> bool;
}
