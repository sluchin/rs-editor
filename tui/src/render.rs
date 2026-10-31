//! エディタの状態から画面を作ります: ウィンドウ (本文とモードライン) と, 最後の行 (エコー領域かミニバッファ).

use std::collections::BTreeMap;

use rseditor_core::buffer::{Buffer, TAB_WIDTH};
use rseditor_core::editor::Editor;
use rseditor_core::scheme::builtins::char_width;
use rseditor_core::syntax::Dialect;
use rseditor_core::window::FULL;

use crate::screen::{Cell, Frame};

/// ミニバッファ (問いと入力).
pub struct Minibuffer<'a> {
    /// 問い.
    pub prompt: &'a str,
    /// 入力.
    pub input: &'a str,
    /// カーソルの、入力の中の位置 (文字の数).
    pub cursor: usize,
}

/// 画面の出し方の設定 (Lisp の `truncate-lines` と `selective-display`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Display {
    /// 真なら, 長い行を折り返さず, 窓の端で切る.
    pub truncate_lines: bool,
    /// 数 N なら, N 桁以上字下げした行を出さない.
    pub selective_display: Option<usize>,
}

/// 文字を画面のマスにします (タブは桁に合わせて空白に, 制御文字は `^X` に, 幅の広い文字は 2 マスにする).
///
/// # Arguments
///
/// * `c` - 文字.
/// * `column` - この文字の前の桁.
fn cells_of(c: char, column: usize) -> Vec<char> {
    if c == '\t' {
        return vec![' '; (column / TAB_WIDTH + 1) * TAB_WIDTH - column];
    }
    if c.is_control() {
        let shown = if c == '\x7f' {
            '?'
        } else {
            char::from_u32(c as u32 + 64).unwrap_or('?')
        };
        return vec!['^', shown];
    }
    if char_width(c) == 2 {
        return vec![c, '\0'];
    }
    vec![c]
}

/// 行を, 窓の幅で折り返したマスの行にします.
fn wrap(line: &str, width: usize) -> Vec<Vec<char>> {
    let mut rows: Vec<Vec<char>> = vec![Vec::new()];
    let mut column = 0;
    for c in line.chars() {
        let cells = cells_of(c, column);
        column += cells.len();
        // 幅を越えるなら次の行へ (幅の広い文字を割らない)
        if rows.last().unwrap().len() + cells.len() > width {
            rows.push(Vec::new());
        }
        rows.last_mut().unwrap().extend(cells);
    }
    rows
}

/// `wrap` と同じに折り返して, マスごとに, 強調する範囲 (文字の位置) の中かを返します.
fn wrap_marks(line: &str, width: usize, spans: &[(usize, usize)]) -> Vec<Vec<bool>> {
    let mut rows: Vec<Vec<bool>> = vec![Vec::new()];
    let mut column = 0;
    for (i, c) in line.chars().enumerate() {
        let cells = cells_of(c, column);
        column += cells.len();
        if rows.last().unwrap().len() + cells.len() > width {
            rows.push(Vec::new());
        }
        let marked = spans.iter().any(|&(from, to)| from <= i && i < to);
        rows.last_mut().unwrap().extend(vec![marked; cells.len()]);
    }
    rows
}

/// 行の先頭の空白の桁の数.
fn indentation(line: &str) -> usize {
    let mut column = 0;
    for c in line.chars() {
        match c {
            ' ' => column += 1,
            '\t' => column = (column / TAB_WIDTH + 1) * TAB_WIDTH,
            _ => break,
        }
    }
    column
}

/// 行を出さないか (`selective-display`. 空の行は出す).
fn hidden(line: &str, display: Display) -> bool {
    display
        .selective_display
        .is_some_and(|n| !line.trim().is_empty() && indentation(line) >= n)
}

/// 行を, 画面のマスの行にします (出さない行は無く, 切る設定なら 1 行).
fn rows_for(line: &str, width: usize, display: Display) -> Vec<Vec<char>> {
    if hidden(line, display) {
        return Vec::new();
    }
    let mut rows = wrap(line, width);
    if display.truncate_lines {
        rows.truncate(1);
    }
    rows
}

/// カーソルの、行の中の画面の位置 (折り返した行の番号と、桁). 幅の広い文字を割らずに折り返すので、桁の数からは決まらない.
fn cursor_cell(buffer: &Buffer, width: usize) -> (usize, usize) {
    let line_start = buffer.line_beginning_position(1);
    let before: String = buffer
        .line_text(buffer.line_of_position(buffer.point()))
        .chars()
        .take(buffer.point() - line_start)
        .collect();
    let rows = wrap(&before, width);
    let last = rows.last().map_or(0, Vec::len);
    // ちょうど行の幅まで来たら、次の行の始め
    if last >= width {
        (rows.len(), 0)
    } else {
        (rows.len() - 1, last)
    }
}

/// モードラインの文字.
fn mode_line(buffer: &Buffer, width: usize) -> String {
    let flag = if buffer.modified() { "**" } else { "--" };
    let mode = match buffer.dialect() {
        Dialect::Fundamental => "Fundamental",
        Dialect::Elisp => "ELisp",
        Dialect::Scheme => "Scheme",
    };
    let line = buffer.line_of_position(buffer.point()) + 1;
    let text = format!("-:{flag}-  {}   L{line}   ({mode}) ", buffer.name());
    let used: usize = text.chars().map(char_width).sum();
    format!("{text}{}", "-".repeat(width.saturating_sub(used)))
}

/// 窓の枠の位置 (割合 %) を、画面のマスにします.
fn to_cells(percent: f64, total: usize) -> usize {
    (percent / 100.0 * total as f64).round() as usize
}

/// 画面を作ります.
///
/// # Arguments
///
/// * `editor` - エディタ.
/// * `tops` - ウィンドウごとの、画面の一番上の行 (カーソルが見えるように、ここで直す).
/// * `rows` - 画面の行の数.
/// * `cols` - 画面の桁の数.
/// * `echo` - エコー領域に出す文字.
/// * `minibuffer` - 入力を読んでいるときの、ミニバッファ.
pub fn render(
    editor: &Editor,
    tops: &mut BTreeMap<usize, usize>,
    rows: usize,
    cols: usize,
    echo: &str,
    minibuffer: Option<&Minibuffer>,
) -> Frame {
    render_with(
        editor,
        tops,
        rows,
        cols,
        echo,
        minibuffer,
        Display::default(),
    )
}

/// 出し方の設定を使って, 画面を作ります (引数は [`render`] と同じ. `display` は出し方の設定).
pub fn render_with(
    editor: &Editor,
    tops: &mut BTreeMap<usize, usize>,
    rows: usize,
    cols: usize,
    echo: &str,
    minibuffer: Option<&Minibuffer>,
    display: Display,
) -> Frame {
    let mut frame = Frame::blank(rows, cols);
    let body_rows = rows.saturating_sub(1);
    let selected = editor.selected_window();
    let mut cursor = (0, 0);
    for rect in editor.layout().rects(FULL) {
        let (x0, x1) = (
            to_cells(rect.left, cols),
            to_cells(rect.left + rect.width, cols),
        );
        let (y0, y1) = (
            to_cells(rect.top, body_rows),
            to_cells(rect.top + rect.height, body_rows),
        );
        if y1 < y0 + 2 || x1 <= x0 {
            continue;
        }
        let width = x1 - x0;
        let text_rows = y1 - y0 - 1;
        // 窓は、開いているバッファを出している
        let buffer = editor
            .get_buffer(editor.window_buffer(rect.id).unwrap())
            .unwrap();
        let top = tops.entry(rect.id).or_insert(0);
        let point_line = buffer.line_of_position(buffer.point());
        let is_selected = rect.id == selected;
        if is_selected {
            // カーソルの行が見えるように、一番上の行を直す
            *top = (*top).min(point_line);
            let in_line = if display.truncate_lines {
                0
            } else {
                cursor_cell(buffer, width).0
            };
            let fits = |top: usize| -> bool {
                let above: usize = (top..point_line)
                    .map(|l| rows_for(&buffer.line_text(l), width, display).len())
                    .sum();
                above + in_line < text_rows
            };
            while *top < point_line && !fits(*top) {
                *top += 1;
            }
        }
        let (in_cursor_row, cursor_col) = if display.truncate_lines {
            (0, buffer.current_column())
        } else {
            cursor_cell(buffer, width)
        };
        let mut row = 0;
        let mut line = *top;
        while row < text_rows && line < buffer.line_count() {
            let text = buffer.line_text(line);
            let marks = wrap_marks(&text, width, &buffer.highlighted_spans(line));
            for (i, cells) in rows_for(&text, width, display).into_iter().enumerate() {
                if row >= text_rows {
                    break;
                }
                for (x, ch) in cells.into_iter().enumerate().take(width) {
                    let underline = marks
                        .get(i)
                        .and_then(|m| m.get(x))
                        .copied()
                        .unwrap_or(false);
                    frame.cells[y0 + row][x0 + x] = Cell {
                        ch,
                        reverse: false,
                        underline,
                    };
                }
                if is_selected && line == point_line && i == in_cursor_row {
                    // 切った行では, 窓の端より先のカーソルは, 端に置く
                    cursor = (y0 + row, x0 + cursor_col.min(width - 1));
                }
                row += 1;
            }
            line += 1;
        }
        for (x, ch) in mode_line(buffer, width).chars().take(width).enumerate() {
            frame.cells[y1 - 1][x0 + x] = Cell {
                ch,
                reverse: true,
                underline: false,
            };
        }
    }
    // 最後の行: ミニバッファかエコー領域
    if rows > 0 {
        let last = rows - 1;
        let (text, at) = match minibuffer {
            Some(m) => {
                let before: String = m.input.chars().take(m.cursor).collect();
                let at: usize = m.prompt.chars().chain(before.chars()).map(char_width).sum();
                (format!("{}{}", m.prompt, m.input), Some(at))
            }
            None => (echo.to_string(), None),
        };
        let mut x = 0;
        for c in text.chars() {
            if x + char_width(c) > cols {
                break;
            }
            frame.cells[last][x].ch = c;
            if char_width(c) == 2 {
                frame.cells[last][x + 1].ch = '\0';
            }
            x += char_width(c);
        }
        if let Some(at) = at {
            cursor = (last, at.min(cols.saturating_sub(1)));
        }
    }
    frame.cursor = cursor;
    frame
}
