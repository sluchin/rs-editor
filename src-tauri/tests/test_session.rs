//! 画面のエディタ (`session`) のテスト. 画面は偽物 (描く内容を覚え, 問いに決めておいた答えを返す).

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use rseditor::scheme::value::Value;
use rseditor::session::{
    append_message, run, Front, Input, Session, AUTO_SAVE_TIMEOUT, MESSAGES_BUFFER, SCRATCH_TEXT,
};
use rseditor_core::editor::Editor;
use rseditor_core::frame::Frame;

/// 偽の画面.
#[derive(Default)]
struct FakeFront {
    /// 描いた内容 (最後が最新).
    frames: Mutex<Vec<Frame>>,
    /// 頼まれた操作.
    requests: Mutex<Vec<String>>,
    /// 問いの答え (無ければ Quit).
    answers: Mutex<VecDeque<Value>>,
}

impl Front for FakeFront {
    fn frame(&self, frame: &Frame) {
        self.frames.lock().unwrap().push(frame.clone());
    }

    fn request(&self, op: &str, _args: Vec<Value>) -> Result<Value, String> {
        self.requests.lock().unwrap().push(op.to_string());
        if op == "window-top-line" {
            // 答えを並べていれば, それを返す (画面がまだ描いていないときの 0 など)
            return Ok(self
                .answers
                .lock()
                .unwrap()
                .pop_front()
                .unwrap_or(Value::Number(2.0)));
        }
        if op.starts_with("read-") || op == "completing-read" {
            return self
                .answers
                .lock()
                .unwrap()
                .pop_front()
                .ok_or_else(|| "Quit".to_string());
        }
        Ok(Value::Nil)
    }
}

fn setup() -> (Session<FakeFront>, Arc<FakeFront>) {
    let front = Arc::new(FakeFront::default());
    (Session::new(front.clone()).unwrap(), front)
}

fn key(session: &mut Session<FakeFront>, keys: &str) {
    for key in keys.split(' ') {
        session.handle(Input::Key {
            key: key.to_string(),
        });
    }
}

fn last(front: &FakeFront) -> Frame {
    front.frames.lock().unwrap().last().unwrap().clone()
}

fn text(session: &Session<FakeFront>) -> String {
    session.host().editor().current_buffer().text()
}

#[test]
fn test_startup_buffers() {
    let (session, _) = setup();
    let editor = session.host().editor();
    assert_eq!(editor.buffer_list(), vec!["*scratch*", MESSAGES_BUFFER]);
    assert_eq!(editor.current_buffer().text(), SCRATCH_TEXT);
    assert!(!editor.current_buffer().modified());
    assert!(editor.get_buffer(MESSAGES_BUFFER).unwrap().read_only());
}

#[test]
fn test_keys_text_and_frames() {
    let (mut session, front) = setup();
    session.eval("(erase-buffer)").unwrap();
    key(&mut session, "a b C-b");
    let frame = last(&front);
    assert_eq!(frame.windows[0].point, 1);
    session.handle(Input::Text {
        text: "日本".to_string(),
    });
    assert_eq!(text(&session), "a日本b");
    // 前に送った本文の変更だけ
    let frame = last(&front);
    assert_eq!(frame.buffers[0].text, None);
    assert_eq!(frame.buffers[0].changes[0].insert, "日本");
    // 送り直す
    session.handle(Input::Redraw);
    assert_eq!(last(&front).buffers[0].text.as_deref(), Some("a日本b"));
    // クリックで, カーソルを動かす (知らないウィンドウは, 何もしない)
    session.handle(Input::Point {
        window: 1,
        pos: 1,
        anchor: None,
    });
    assert_eq!(session.host().editor().current_buffer().point(), 1);
    session.handle(Input::Point {
        window: 9,
        pos: 0,
        anchor: None,
    });
    assert_eq!(session.host().editor().current_buffer().point(), 1);
    // ドラッグで選ぶと, 始めにマークを置く
    session.handle(Input::Point {
        window: 1,
        pos: 3,
        anchor: Some(1),
    });
    assert_eq!(last(&front).windows[0].mark, Some(1));
    session.handle(Input::Point {
        window: 1,
        pos: 1,
        anchor: Some(1),
    });
    assert_eq!(last(&front).windows[0].mark, None);
    // コマンドを名前で動かす
    session.handle(Input::Command {
        name: "beginning-of-buffer".to_string(),
    });
    assert_eq!(session.host().editor().current_buffer().point(), 0);
    // 割り当ての無いキー
    key(&mut session, "C-x C-z");
    assert_eq!(last(&front).message, "C-x C-z is undefined");
}

#[test]
fn test_messages_questions_and_screen_requests() {
    let (mut session, front) = setup();
    session.eval("(message \"one\") (message \"\")").unwrap();
    assert_eq!(last(&front).message, "");
    let messages = session
        .host()
        .editor()
        .get_buffer(MESSAGES_BUFFER)
        .unwrap()
        .text();
    assert_eq!(messages, "one\n");
    // 失敗はエコー領域と *Messages* に出す
    assert!(session.eval("(car 1)").is_err());
    assert!(last(&front).message.starts_with("car"));
    // 問いの前に描く. 答えを返す
    front
        .answers
        .lock()
        .unwrap()
        .push_back(Value::String("typed".into()));
    let frames = front.frames.lock().unwrap().len();
    assert_eq!(
        session.eval("(read-string \"Name: \")").unwrap(),
        "\"typed\""
    );
    assert!(front.frames.lock().unwrap().len() > frames + 1);
    // C-g
    assert_eq!(
        session.eval("(read-string \"Name: \")").unwrap_err(),
        "Quit"
    );
    // 画面だけの操作は, 画面に頼む
    session.eval("(set-background-color \"black\")").unwrap();
    assert!(front
        .requests
        .lock()
        .unwrap()
        .contains(&"set-background-color".to_string()));
    // 読み取り専用の *Messages* にも書ける. カーソルが終わりでなければ, そのまま
    let mut editor = Editor::new();
    append_message(&mut editor, "a");
    editor.get_buffer_mut(MESSAGES_BUFFER).unwrap().set_point(0);
    append_message(&mut editor, "b");
    let buffer = editor.get_buffer(MESSAGES_BUFFER).unwrap();
    assert_eq!(buffer.text(), "a\nb\n");
    assert_eq!(buffer.point(), 0);
}

#[test]
fn test_keys_bindings_heights_and_macros() {
    let (mut session, front) = setup();
    assert_eq!(
        session.eval("(where-is-internal 'find-file)").unwrap(),
        "(\"C-x C-f\")"
    );
    assert_eq!(
        session
            .eval(
                "(list (key-binding \"C-x\") (key-binding \"C-x C-f\") (key-binding \"C-x C-z\"))"
            )
            .unwrap(),
        "(#t \"find-file\" #f)"
    );
    assert!(
        session
            .eval("(length (all-key-bindings))")
            .unwrap()
            .parse::<usize>()
            .unwrap()
            > 100
    );
    key(&mut session, "C-f");
    assert!(session.eval("(recent-keys)").unwrap().contains("\"C-f\""));
    assert_eq!(session.eval("(editor-command-names)").unwrap(), "()");
    assert!(session.eval("(command \"x\")").is_err());
    // 見えている行の数 (知らなければ 20)
    assert_eq!(session.eval("(window-body-height)").unwrap(), "20");
    session.handle(Input::Resize {
        heights: vec![(1, 7)],
    });
    assert_eq!(session.eval("(window-body-height)").unwrap(), "7");
    // キーボードマクロ
    session.eval("(erase-buffer)").unwrap();
    key(&mut session, "C-x ( a b C-x ) C-x e");
    assert_eq!(text(&session), "abab");
    session.eval("(execute-kbd-macro '(\"c\") 2)").unwrap();
    // 打ったキーが失敗すると, 待っているマクロのキーは捨てる. 失敗しないキーで流す
    key(&mut session, "C-e");
    assert_eq!(text(&session), "ababcc");
    assert_eq!(session.eval("(executing-kbd-macro-p)").unwrap(), "#f");
    // 繰り返すマクロは, 失敗するまで (ここでは, 打ったことにするキーが尽きるまで止める)
    session.eval("(execute-kbd-macro '(\"C-b\") 0)").unwrap();
    key(&mut session, "C-e");
    assert_eq!(session.host().editor().current_buffer().point(), 0);
    assert_eq!(last(&front).message, "Beginning of buffer");
}

#[test]
fn test_find_file_and_read_key_in_a_macro() {
    let (mut session, front) = setup();
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("a.txt");
    std::fs::write(&path, "note").unwrap();
    session.handle(Input::FindFile {
        path: path.to_string_lossy().into_owned(),
    });
    assert_eq!(text(&session), "note");
    assert_eq!(last(&front).windows[0].buffer, "a.txt");
    // 開けないファイル (UTF-8 でない) は, エコー領域に出す
    let bad = dir.path().join("bad.bin");
    std::fs::write(&bad, [0xff, 0xfe]).unwrap();
    session.handle(Input::FindFile {
        path: bad.to_string_lossy().into_owned(),
    });
    assert!(!last(&front).message.is_empty());
    // マクロの中の read-key は, マクロのキーを読む
    session
        .eval("(define-command (ask) (insert (read-key \"Key: \")))")
        .unwrap();
    session.eval("(global-set-key \"C-c a\" 'ask)").unwrap();
    session
        .eval("(execute-kbd-macro '(\"C-c\" \"a\" \"z\"))")
        .unwrap();
    key(&mut session, "C-e");
    assert!(text(&session).contains('z'));
    // マクロでないときは, 画面に聞き, 最近のキーに覚える
    front
        .answers
        .lock()
        .unwrap()
        .push_back(Value::String("q".into()));
    key(&mut session, "C-c a");
    assert!(text(&session).contains('q'));
    assert!(session.eval("(recent-keys)").unwrap().contains("\"q\""));
}

#[test]
fn test_read_key_names_and_quit() {
    let (mut session, front) = setup();
    let answer = |key: &str| {
        front
            .answers
            .lock()
            .unwrap()
            .push_back(Value::String(key.into()))
    };
    answer("RET");
    assert_eq!(session.eval("(read-key \"Key: \")").unwrap(), "\"Enter\"");
    // C-g は, 許されていなければ, やめる
    answer("C-g");
    assert_eq!(session.eval("(read-key \"Key: \")").unwrap_err(), "Quit");
    answer("C-g");
    assert_eq!(session.eval("(read-key \"Key: \" #t)").unwrap(), "\"C-g\"");
    // 画面に聞いたキーは, マクロに覚える
    key(&mut session, "C-x (");
    answer("SPC");
    assert_eq!(session.eval("(read-key)").unwrap(), "\" \"");
    key(&mut session, "C-x )");
    assert!(session
        .host()
        .editor()
        .kbd_macro
        .last()
        .contains(&"SPC".to_string()));
}

#[test]
fn test_move_to_window_line() {
    let (mut session, front) = setup();
    session.eval("(erase-buffer)").unwrap();
    session
        .eval(
            "(let loop ((i 0)) (when (< i 30) (insert (format \"line %d\\n\" i)) (loop (+ i 1))))",
        )
        .unwrap();
    session.handle(Input::Resize {
        heights: vec![(1, 10)],
    });
    let point = |session: &mut Session<FakeFront>| session.eval("(point)").unwrap();
    // 一番上の行 (画面に聞く. ここでは 2) から数える
    assert_eq!(session.eval("(move-to-window-line 0)").unwrap(), "0");
    assert_eq!(point(&mut session), "15");
    assert_eq!(session.eval("(move-to-window-line -1)").unwrap(), "9");
    assert_eq!(point(&mut session), "79");
    session.eval("(move-to-window-line)").unwrap();
    assert_eq!(point(&mut session), "50");
    assert!(front
        .requests
        .lock()
        .unwrap()
        .contains(&"window-top-line".to_string()));
}

#[test]
fn test_init_file_and_run() {
    let dir = tempfile::tempdir().unwrap();
    let init = dir.path().join("init.scm");
    std::fs::write(&init, "(global-set-key \"C-c z\" 'end-of-buffer)").unwrap();
    let (mut session, front) = setup();
    session.load_init(Some(&init));
    assert_eq!(
        session.eval("user-init-file").unwrap(),
        format!("{:?}", init.to_string_lossy())
    );
    assert_eq!(
        session.eval("(key-binding \"C-c z\")").unwrap(),
        "\"end-of-buffer\""
    );
    // 無いファイルは, 読まない
    session.load_init(None);
    session.load_init(Some(&dir.path().join("none.scm")));
    // 失敗は, エコー領域と *Warnings* に出す
    std::fs::write(&init, "(car 1)").unwrap();
    session.load_init(Some(&init));
    session.handle(Input::Redraw);
    assert!(last(&front).message.contains("init.scm"));
    let warnings = session
        .host()
        .editor()
        .get_buffer("*Warnings*")
        .unwrap()
        .text();
    assert!(warnings.contains("An error occurred while loading the init file"));
    // 入力を順に処理し, 送り手が無くなったら終わる
    let front = Arc::new(FakeFront::default());
    let (sender, receiver) = std::sync::mpsc::channel();
    sender
        .send(Input::Text {
            text: "x".to_string(),
        })
        .unwrap();
    drop(sender);
    run(front.clone(), None, receiver, AUTO_SAVE_TIMEOUT).unwrap();
    let frames = front.frames.lock().unwrap();
    assert_eq!(frames.len(), 2);
    assert!(frames[1].buffers[0].changes[0].insert == "x");
}

#[test]
fn test_auto_save_and_backup() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("a.txt");
    let path = file.to_string_lossy().into_owned();
    std::fs::write(&file, "hi\n").unwrap();
    let (mut session, front) = setup();
    session.eval(&format!("(find-file {path:?})")).unwrap();
    session.handle(Input::Text {
        text: "x".to_string(),
    });
    // 書き換えたバッファを, #名前# に書く
    session.auto_save();
    let auto = dir.path().join("#a.txt#");
    assert_eq!(std::fs::read_to_string(&auto).unwrap(), "xhi\n");
    assert_eq!(last(&front).message, "Auto-saving...done");
    // 書き換えていなければ, 書かない (描かない)
    let count = front.frames.lock().unwrap().len();
    session.auto_save();
    assert_eq!(front.frames.lock().unwrap().len(), count);
    // 保存すると, バックアップを残し, 自動保存のファイルを消す
    key(&mut session, "C-x C-s");
    assert_eq!(std::fs::read_to_string(&file).unwrap(), "xhi\n");
    assert_eq!(
        std::fs::read_to_string(dir.path().join("a.txt~")).unwrap(),
        "hi\n"
    );
    assert!(!auto.exists());
    // 入力が無いまま時間が経つと, 自動保存する
    let front = Arc::new(FakeFront::default());
    let (sender, receiver) = std::sync::mpsc::channel();
    let watcher = front.clone();
    let thread = std::thread::spawn(move || {
        // 始めの描画 (エディタを作り終えた) を待ってから, 入力の無い時間を置く
        while watcher.frames.lock().unwrap().is_empty() {
            std::thread::yield_now();
        }
        std::thread::sleep(Duration::from_millis(50));
        drop(sender);
    });
    run(front.clone(), None, receiver, Duration::from_millis(5)).unwrap();
    thread.join().unwrap();
}

#[test]
fn test_find_file_and_save_hooks() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("a.txt").to_string_lossy().into_owned();
    let (mut session, front) = setup();
    session
        .eval(
            "(define found '()) (define saved '())
             (add-hook 'find-file-hook (lambda () (set! found (cons (buffer-name) found))))
             (add-hook 'after-save-hook (lambda () (set! saved (cons (buffer-name) saved))))",
        )
        .unwrap();
    // 新しいバッファだけ, find-file-hook を動かす (そのバッファで)
    session.eval(&format!("(find-file {path:?})")).unwrap();
    session.eval("(switch-to-buffer \"*scratch*\")").unwrap();
    session.eval(&format!("(find-file {path:?})")).unwrap();
    assert_eq!(session.eval("found").unwrap(), "(\"a.txt\")");
    // 保存したら, after-save-hook を動かす. "Wrote" の知らせは残す
    session.handle(Input::Text {
        text: "x".to_string(),
    });
    key(&mut session, "C-x C-s");
    assert_eq!(session.eval("saved").unwrap(), "(\"a.txt\")");
    assert_eq!(last(&front).message, format!("Wrote {path}"));
    // 別のバッファから保存しても (save-some-buffers など), そのバッファで動かす
    session
        .eval("(switch-to-buffer \"*scratch*\") (with-current-buffer \"a.txt\" (set-buffer-modified-p #t) (basic-save-buffer))")
        .unwrap();
    assert_eq!(session.eval("saved").unwrap(), "(\"a.txt\" \"a.txt\")");
}

#[test]
fn test_screen_answers_and_recent_keys() {
    let (mut session, front) = setup();
    // 画面の一番上の行が分からなければ (0 や nil), 本文の始めから数える
    for top in [Value::Number(0.0), Value::Nil] {
        front.answers.lock().unwrap().push_back(top);
        session.eval("(move-to-window-line 0)").unwrap();
        assert_eq!(session.host().editor().current_buffer().point(), 0);
    }
    // キーの並びでないものは, 何もしない. 回数が数でなければ 1 回
    session.eval("(execute-kbd-macro \"x\")").unwrap();
    session.eval("(execute-kbd-macro '(\"a\") \"n\")").unwrap();
    // 最近のキー (C-h l) は, 300 個まで覚える
    for _ in 0..301 {
        key(&mut session, "C-f");
    }
    assert_eq!(session.eval("(length (recent-keys))").unwrap(), "300");
}
