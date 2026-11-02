//! ターミナルのエディタ: キーを受けて、コマンドを動かし (コマンドループは核の [`CommandLoop`])、画面を描く.

use std::collections::{BTreeMap, VecDeque};
use std::sync::{Arc, Mutex};

use rseditor_core::command_loop::CommandLoop;
use rseditor_core::editor::Editor;
use rseditor_core::keymap::Keymap;
use rseditor_core::scheme::editor_host::EditorHost;
use rseditor_core::scheme::evaluator::Evaluator;
use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::value::Value;

use crate::render::{render_with, Display};
use crate::screen::Screen;
use crate::ui::{next_key, UiHost, UiState};

/// ターミナルのエディタ.
pub struct App {
    /// コマンドループ (エディタと評価器を持つ).
    commands: CommandLoop<UiHost>,
    /// 画面とその状態.
    state: Arc<Mutex<UiState>>,
}

impl App {
    /// エディタを作ります. 基本の編集のコマンド (`lisp/simple.scm` など) を読み、ファイルを開く.
    ///
    /// # Arguments
    ///
    /// * `screen` - 画面.
    /// * `files` - 開くファイル.
    ///
    /// # Errors
    ///
    /// 基本のコマンドを読めないとき.
    pub fn new(screen: Box<dyn Screen>, files: &[String]) -> Result<Self, String> {
        let editor = Arc::new(Mutex::new(Editor::new()));
        let keymap = Arc::new(Mutex::new(Keymap::default()));
        let state = Arc::new(Mutex::new(UiState {
            screen,
            message: String::new(),
            tops: BTreeMap::new(),
            quit: false,
            display: Display::default(),
            macro_keys: Default::default(),
            macro_loop: Vec::new(),
            executing_macro: false,
            recent_keys: VecDeque::new(),
        }));
        let ui = UiHost::new(state.clone(), editor.clone(), keymap.clone());
        let host = EditorHost::shared(editor, ui);
        let commands = CommandLoop::new(host, Evaluator::new(), keymap)?;
        let app = App { commands, state };
        // 最後のファイルを出す (Emacs と同じ)
        for file in files {
            let host = app.host();
            let name = host.call("find-file-noselect", vec![Value::String(file.clone())])?;
            host.call("pop-to-buffer-same-window", vec![name])?;
        }
        Ok(app)
    }

    /// Scheme を評価します (初期設定ファイルを読むときなど).
    ///
    /// # Errors
    ///
    /// 評価に失敗したとき.
    pub fn eval(&self, code: &str) -> Result<String, String> {
        self.commands.eval(code)
    }

    /// エディタの状態.
    pub fn host(&self) -> &EditorHost<UiHost> {
        self.commands.host()
    }

    /// 終わるか.
    pub fn quit(&self) -> bool {
        self.state.lock().unwrap().quit
    }

    /// エコー領域の文字.
    pub fn message(&self) -> String {
        self.state.lock().unwrap().message.clone()
    }

    /// 画面を描きます.
    pub fn draw(&self) {
        let mut editor = self.host().editor();
        let mut state = self.state.lock().unwrap();
        let (rows, cols) = state.screen.size();
        editor.set_screen_size(rows, cols);
        let message = state.message.clone();
        let display = state.display;
        let frame = render_with(
            &editor,
            &mut state.tops,
            rows,
            cols,
            &message,
            None,
            display,
        );
        state.screen.draw(&frame);
    }

    /// 画面を描いて、キーを読み、終わるまで繰り返します.
    pub fn run(&mut self) {
        loop {
            self.draw();
            match next_key(&self.state) {
                Some((key, from_screen)) => {
                    // 新しいコマンドの始めのキーは, マクロを終えるコマンドのキーを除くために, 覚える前に知らせる
                    if self.commands.at_command_start() {
                        self.host().editor().kbd_macro.begin_command();
                    }
                    if from_screen {
                        self.host().editor().kbd_macro.record(&key);
                    }
                    self.handle_key(&key);
                }
                None => break,
            }
            if self.quit() {
                break;
            }
        }
    }

    /// キーボードマクロのキーが残っていれば, 全て処理します (`run` は, キーを読むときに同じ順で処理する).
    pub fn flush_macro(&mut self) {
        loop {
            let key = self.state.lock().unwrap().pop_macro_key();
            match key {
                Some(key) => self.handle_key(&key),
                None => break,
            }
        }
    }

    /// キーを 1 つ処理します.
    pub fn handle_key(&mut self, key: &str) {
        self.commands.handle_key(key);
    }
}
