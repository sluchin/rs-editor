//! 画面 (Tauri) への窓口: 描く内容を `frame` のイベントで送り, 問いと画面だけの操作を `editor-request` のイベントで頼みます.
//! 画面からの入力は, Tauri のコマンド [`session_input`] で受け, エディタのスレッド ([`session::run`]) に渡します.

use std::sync::{mpsc, Arc, Mutex};

use rseditor_core::frame::Frame;
use tauri::{AppHandle, Emitter, State};

use crate::commands::{init_file_path, EDITOR_REQUEST_EVENT};
use crate::scheme::host::Bridge;
use crate::scheme::value::Value;
use crate::session::{self, Front, Input};

/// 描く内容を送るイベントの名前.
pub const FRAME_EVENT: &str = "frame";

/// Tauri の画面.
pub struct TauriFront {
    /// アプリ (イベントを送る).
    app: AppHandle,
    /// 返事を待つ仕組み.
    bridge: Arc<Bridge>,
}

impl Front for TauriFront {
    fn frame(&self, frame: &Frame) {
        if let Err(error) = self.app.emit(FRAME_EVENT, frame) {
            log::warn!("Could not send a frame: {error}");
        }
    }

    fn request(&self, op: &str, args: Vec<Value>) -> Result<Value, String> {
        self.bridge.request(op, args, &|request| {
            self.app
                .emit(EDITOR_REQUEST_EVENT, request)
                .map_err(|e| e.to_string())
        })
    }
}

/// エディタのスレッドへの入力の送り口 (初めの入力で, スレッドを作る).
#[derive(Default)]
pub struct Inputs(Mutex<Option<mpsc::Sender<Input>>>);

/// 画面からの入力を, エディタに渡します. 初めの入力のときに, エディタのスレッドを作ります
/// (画面がイベントを受け始めてから, 初期設定ファイルを読み, 描くため).
///
/// # Errors
///
/// スレッドを作れないとき, エディタが止まっているとき.
#[tauri::command]
pub fn session_input(
    input: Input,
    app: AppHandle,
    bridge: State<'_, Arc<Bridge>>,
    inputs: State<'_, Inputs>,
) -> Result<(), String> {
    let mut sender = inputs.0.lock().unwrap();
    if sender.is_none() {
        let (tx, rx) = mpsc::channel();
        let front = Arc::new(TauriFront {
            app,
            bridge: Arc::clone(&bridge),
        });
        std::thread::Builder::new()
            .name("session".to_string())
            .spawn(move || {
                if let Err(error) = session::run(front, init_file_path().as_deref(), rx) {
                    log::error!("The editor stopped: {error}");
                }
            })
            .map_err(|e| e.to_string())?;
        *sender = Some(tx);
    }
    sender
        .as_ref()
        .expect("the editor thread is started")
        .send(input)
        .map_err(|e| e.to_string())
}
