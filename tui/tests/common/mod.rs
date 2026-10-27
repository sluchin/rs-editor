//! テストで共有する、偽の画面.

#![allow(dead_code)]

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use rseditor_tui::app::App;
use rseditor_tui::screen::{Frame, Screen};

/// 偽の画面: 決めておいたキーを返し、描いた画面を覚える.
pub struct FakeScreen {
    /// 画面の大きさ (行, 桁).
    pub size: (usize, usize),
    /// これから読むキー.
    pub keys: Arc<Mutex<VecDeque<String>>>,
    /// 描いた画面 (最後が最新).
    pub frames: Arc<Mutex<Vec<Frame>>>,
}

impl Screen for FakeScreen {
    fn size(&self) -> (usize, usize) {
        self.size
    }

    fn draw(&mut self, frame: &Frame) {
        self.frames.lock().unwrap().push(frame.clone());
    }

    fn read_key(&mut self) -> Option<String> {
        self.keys.lock().unwrap().pop_front()
    }
}

/// 偽の画面を使うエディタと、キーの入れ口、描いた画面.
pub struct Fixture {
    /// エディタ.
    pub app: App,
    /// これから読むキー (エディタが読む).
    pub keys: Arc<Mutex<VecDeque<String>>>,
    /// 描いた画面.
    pub frames: Arc<Mutex<Vec<Frame>>>,
}

impl Fixture {
    /// 画面の大きさ (行, 桁) を決めて作る.
    pub fn sized(rows: usize, cols: usize, files: &[String]) -> Result<Self, String> {
        let keys = Arc::new(Mutex::new(VecDeque::new()));
        let frames = Arc::new(Mutex::new(Vec::new()));
        let screen = FakeScreen {
            size: (rows, cols),
            keys: keys.clone(),
            frames: frames.clone(),
        };
        Ok(Fixture {
            app: App::new(Box::new(screen), files)?,
            keys,
            frames,
        })
    }

    /// 12 行 30 桁の画面で、ファイルを開かずに作る.
    pub fn new() -> Self {
        Self::sized(12, 30, &[]).unwrap()
    }

    /// これから読むキーを足す (空白で区切る).
    pub fn feed(&self, keys: &str) {
        self.keys.lock().unwrap().extend(
            keys.split(' ')
                .filter(|k| !k.is_empty())
                .map(str::to_string),
        );
    }

    /// キーを順に処理する (空白で区切る).
    pub fn press(&mut self, keys: &str) {
        for key in keys.split(' ').filter(|k| !k.is_empty()) {
            self.app.handle_key(key);
        }
    }

    /// 今のバッファの本文.
    pub fn text(&self) -> String {
        self.app.host().editor().current_buffer().text()
    }

    /// 今のバッファのカーソル (0 から).
    pub fn point(&self) -> usize {
        self.app.host().editor().current_buffer().point()
    }

    /// 最後に描いた画面の行.
    pub fn screen(&self) -> Vec<String> {
        self.frames.lock().unwrap().last().unwrap().lines()
    }

    /// 最後に描いた画面のカーソル.
    pub fn cursor(&self) -> (usize, usize) {
        self.frames.lock().unwrap().last().unwrap().cursor
    }
}
