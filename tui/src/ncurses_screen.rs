//! ncurses の画面. 端末を相手にするので、自動のテストはしない (カバレッジから除く).

use ncurses::*;

use rseditor_core::highlight::TokenKind;

use crate::screen::{token_pair, token_style, Frame, Screen};

/// ncurses の画面.
pub struct NcursesScreen {
    /// 色を使えるか (端末が色に対応していて、色の対を作れたか).
    colors: bool,
}

impl NcursesScreen {
    /// ncurses を始めます.
    pub fn start() -> Self {
        // UTF-8 の文字を使うために, 端末の文字の設定を使う
        let _ = setlocale(LcCategory::all, "");
        initscr();
        raw();
        noecho();
        keypad(stdscr(), true);
        set_escdelay(25);
        NcursesScreen {
            colors: Self::start_colors(),
        }
    }

    /// 字句の種類ごとの色の対を作ります (対の番号は、`token_pair`).
    ///
    /// # Returns
    ///
    /// 色を使えるか (端末が色に対応していなければ、偽).
    fn start_colors() -> bool {
        if !has_colors() {
            return false;
        }
        start_color();
        // 端末の背景の色を、そのまま使う
        use_default_colors();
        for kind in TokenKind::ALL {
            if let Some(style) = token_style(kind) {
                init_pair(token_pair(kind), style.color, -1);
            }
        }
        true
    }

    /// ncurses を終えます.
    pub fn stop() {
        endwin();
    }

    /// 1 つのキーを、Emacs の書き方にします.
    fn describe(code: WchResult) -> Option<String> {
        match code {
            WchResult::KeyCode(code) => Some(
                match code {
                    KEY_UP => "<up>",
                    KEY_DOWN => "<down>",
                    KEY_LEFT => "<left>",
                    KEY_RIGHT => "<right>",
                    KEY_HOME => "<home>",
                    KEY_END => "<end>",
                    KEY_NPAGE => "<next>",
                    KEY_PPAGE => "<prior>",
                    KEY_DC => "<delete>",
                    code if code == KEY_F(3) => "<f3>",
                    code if code == KEY_F(4) => "<f4>",
                    KEY_BACKSPACE => "DEL",
                    KEY_ENTER => "RET",
                    _ => return Some(String::new()),
                }
                .to_string(),
            ),
            WchResult::Char(code) => Some(match code {
                0 => "C-SPC".to_string(),
                9 => "TAB".to_string(),
                10 | 13 => "RET".to_string(),
                27 => "ESC".to_string(),
                32 => "SPC".to_string(),
                127 | 8 => "DEL".to_string(),
                1..=26 => format!("C-{}", (code as u8 + 96) as char),
                28 => "C-\\".to_string(),
                29 => "C-]".to_string(),
                30 => "C-^".to_string(),
                31 => "C-_".to_string(),
                _ => char::from_u32(code)?.to_string(),
            }),
        }
    }
}

impl Screen for NcursesScreen {
    fn size(&self) -> (usize, usize) {
        let (mut rows, mut cols) = (0, 0);
        getmaxyx(stdscr(), &mut rows, &mut cols);
        (rows as usize, cols as usize)
    }

    fn draw(&mut self, frame: &Frame) {
        erase();
        for (y, row) in frame.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if cell.ch == '\0' {
                    continue;
                }
                if cell.reverse {
                    attron(A_REVERSE);
                }
                if cell.underline {
                    attron(A_UNDERLINE);
                }
                let style = cell
                    .token
                    .filter(|_| self.colors)
                    .and_then(|kind| token_style(kind).map(|style| (kind, style)));
                if let Some((kind, style)) = style {
                    attron(COLOR_PAIR(token_pair(kind)));
                    if style.bold {
                        attron(A_BOLD);
                    }
                }
                let _ = mvaddstr(y as i32, x as i32, &cell.ch.to_string());
                if let Some((kind, style)) = style {
                    if style.bold {
                        attroff(A_BOLD);
                    }
                    attroff(COLOR_PAIR(token_pair(kind)));
                }
                if cell.underline {
                    attroff(A_UNDERLINE);
                }
                if cell.reverse {
                    attroff(A_REVERSE);
                }
            }
        }
        mv(frame.cursor.0 as i32, frame.cursor.1 as i32);
        refresh();
    }

    fn interrupted(&mut self) -> bool {
        timeout(0);
        let code = get_wch();
        timeout(-1);
        match code {
            Some(WchResult::Char(7)) => true,
            // ほかのキーは、次に読めるように戻す
            Some(WchResult::Char(code)) => {
                unget_wch(code);
                false
            }
            Some(WchResult::KeyCode(code)) => {
                ungetch(code);
                false
            }
            None => false,
        }
    }

    fn read_key(&mut self) -> Option<String> {
        loop {
            let code = get_wch()?;
            let key = Self::describe(code)?;
            if key.is_empty() {
                continue;
            }
            // ESC に続く文字は、Meta を付けたキー
            if key == "ESC" {
                timeout(25);
                let next = get_wch();
                timeout(-1);
                return match next.and_then(Self::describe) {
                    // Meta と Control を重ねたキーは, Emacs の書き方 (`C-M-x`) にする
                    Some(next) if !next.is_empty() => Some(match next.strip_prefix("C-") {
                        Some(rest) => format!("C-M-{rest}"),
                        None => format!("M-{next}"),
                    }),
                    _ => Some("ESC".to_string()),
                };
            }
            return Some(key);
        }
    }
}
