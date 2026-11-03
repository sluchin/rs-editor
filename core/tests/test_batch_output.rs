//! バッチモードのための組み込み (`princ`, `prin1`, `print`, `terpri`, `buffer-name` の引数,
//! 出力先, 既定のディレクトリ, 最後の改行) のテスト.

use std::sync::{Arc, Mutex};

use rseditor_core::editor::Editor;
use rseditor_core::scheme::editor_host::EditorHost;
use rseditor_core::scheme::evaluator::Evaluator;
use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::value::Value;

/// メッセージを覚え, ほかの操作は失敗にする画面の偽物.
#[derive(Default)]
struct Ui(Arc<Mutex<Vec<String>>>);

impl Host for Ui {
    fn call(&self, op: &str, args: Vec<Value>) -> Result<Value, String> {
        if op == "message" {
            if let Some(Value::String(text)) = args.first() {
                self.0.lock().unwrap().push(text.clone());
            }
            return Ok(Value::Nil);
        }
        Err(format!("ui: {op}"))
    }
}

/// 評価器, ホスト, 画面のメッセージ.
fn setup() -> (Evaluator, EditorHost<Ui>, Arc<Mutex<Vec<String>>>) {
    let messages = Arc::new(Mutex::new(Vec::new()));
    let host = EditorHost::new(Editor::new(), Ui(messages.clone()));
    (Evaluator::new(), host, messages)
}

#[test]
fn test_output_goes_to_the_echo_area_by_default() {
    let (ev, host, messages) = setup();
    assert_eq!(
        ev.eval_with("(princ \"hi\\n\")", &host).unwrap(),
        "\"hi\\n\""
    );
    ev.eval_with("(terpri)", &host).unwrap();
    // 末尾の改行は落として, メッセージにする
    assert_eq!(
        *messages.lock().unwrap(),
        vec!["hi".to_string(), "".to_string()]
    );
    // 文字列でない出力は失敗する
    assert_eq!(
        host.call("standard-output", vec![Value::Nil]),
        Err("standard-output expects a string".to_string())
    );
}

#[test]
fn test_output_goes_to_the_sink_in_batch_mode() {
    let (ev, host, messages) = setup();
    let out = Arc::new(Mutex::new(String::new()));
    let sink = out.clone();
    host.set_output(Box::new(move |text| sink.lock().unwrap().push_str(text)));
    let eval = |code: &str| ev.eval_with(code, &host).unwrap();
    assert_eq!(eval("(princ 12)"), "12");
    eval("(princ \"a\") (princ '(\"b\" 2 (\"c\")))");
    eval("(prin1 \"q\\\"\\\\\") (prin1 '(\"x\" y))");
    eval("(print \"p\")");
    assert_eq!(eval("(terpri)"), "#t");
    assert_eq!(
        *out.lock().unwrap(),
        "12a(b 2 (c))\"q\\\"\\\\\"(\"x\" y)\n\"p\"\n\n"
    );
    assert!(messages.lock().unwrap().is_empty());
    // 引数の数
    assert_eq!(
        ev.eval_with("(princ)", &host),
        Err("princ expects 1 argument, given 0".to_string())
    );
    assert_eq!(
        ev.eval_with("(terpri 1)", &host),
        Err("terpri expects 0 arguments, given 1".to_string())
    );
}

#[test]
fn test_buffer_name_takes_a_buffer() {
    let (ev, host, _) = setup();
    let eval = |code: &str| ev.eval_with(code, &host).unwrap();
    eval("(generate-new-buffer \"foo\")");
    assert_eq!(eval("(buffer-name \"foo\")"), "\"foo\"");
    assert_eq!(eval("(buffer-name \"none\")"), "#f");
    assert_eq!(
        ev.eval_with("(buffer-name 1)", &host),
        Err("buffer-name expects strings".to_string())
    );
    assert_eq!(
        ev.eval_with("(buffer-name \"a\" \"b\")", &host),
        Err("buffer-name expects 1 argument, given 2".to_string())
    );
}

#[test]
fn test_write_region_is_silent_in_batch_mode() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("a.txt");
    let code = format!("(write-region \"x\" #f {:?})", file.to_str().unwrap());
    let (ev, host, messages) = setup();
    ev.eval_with(&code, &host).unwrap();
    assert_eq!(messages.lock().unwrap().len(), 1);
    let (ev, host, messages) = setup();
    host.set_output(Box::new(|_| {}));
    ev.eval_with(&code, &host).unwrap();
    assert!(messages.lock().unwrap().is_empty());
    assert_eq!(std::fs::read_to_string(&file).unwrap(), "x");
}

#[test]
fn test_default_directory_is_the_base_of_relative_names() {
    let dir = tempfile::tempdir().unwrap();
    let ev = Evaluator::new();
    let home = dirs::home_dir().unwrap();
    assert_eq!(
        ev.eval("(expand-file-name \"x\")").unwrap(),
        format!("{:?}", home.join("x").to_str().unwrap())
    );
    ev.set_default_directory(dir.path().to_path_buf());
    assert_eq!(
        ev.eval("(expand-file-name \"x\")").unwrap(),
        format!("{:?}", dir.path().join("x").to_str().unwrap())
    );
}

#[test]
fn test_saving_adds_the_final_newline() {
    let dir = tempfile::tempdir().unwrap();
    let path = |name: &str| dir.path().join(name).to_str().unwrap().to_string();
    let (ev, host, _) = setup();
    let eval = |code: &str| ev.eval_with(code, &host).unwrap();
    for (name, text) in [("a.txt", "abc"), ("b.txt", "abc\n"), ("c.txt", "")] {
        eval(&format!(
            "(switch-to-buffer (find-file-noselect {:?})) (insert {text:?}) (set-buffer-modified-p #t) (save-buffer)",
            path(name)
        ));
    }
    assert_eq!(std::fs::read_to_string(path("a.txt")).unwrap(), "abc\n");
    assert_eq!(std::fs::read_to_string(path("b.txt")).unwrap(), "abc\n");
    assert_eq!(std::fs::read_to_string(path("c.txt")).unwrap(), "");
    // 絞り込んでいても、全体の最後に足す. 無効にもできる
    eval(&format!(
        "(switch-to-buffer (find-file-noselect {:?})) (insert \"x\") (narrow-to-region 1 1) (save-buffer)",
        path("d.txt")
    ));
    assert_eq!(std::fs::read_to_string(path("d.txt")).unwrap(), "x\n");
    eval("(set! mode-require-final-newline #f)");
    eval(&format!(
        "(switch-to-buffer (find-file-noselect {:?})) (insert \"y\") (save-buffer)",
        path("e.txt")
    ));
    assert_eq!(std::fs::read_to_string(path("e.txt")).unwrap(), "y");
}

#[test]
fn test_find_file_noselect_of_a_directory_is_a_dired_buffer() {
    let dir = tempfile::tempdir().unwrap();
    let (ev, host, _) = setup();
    host.editor().set_screen_size(24, 80);
    let name = ev
        .eval_with(
            &format!("(find-file-noselect {:?})", dir.path().to_str().unwrap()),
            &host,
        )
        .unwrap();
    assert!(name.contains(dir.path().file_name().unwrap().to_str().unwrap()));
}

#[test]
fn test_set_marker_and_format_s() {
    let (ev, host, _) = setup();
    ev.load_editor_files(&host).unwrap();
    let eval = |code: &str| {
        let out = ev.eval_with(code, &host).unwrap();
        out.lines().last().unwrap_or("").to_string()
    };
    eval("(insert \"abcdef\")");
    // マーカーを動かす. 解いたマーカーは動かせない
    assert_eq!(eval("(define m (copy-marker 2)) (marker-position m)"), "2");
    assert_eq!(eval("(marker-position (set-marker m 5))"), "5");
    eval("(goto-char 1) (insert \"XX\")");
    assert_eq!(eval("(marker-position m)"), "7");
    assert_eq!(eval("(marker-set 99 1)"), "#f");
    // %S は, prin1 と同じ (改行はそのまま. 引用符と \ は \ を付ける)
    assert_eq!(
        eval("(format \"%S|%S|%S\" \"a\\nb\" \"q\\\"\" '(\"x\" 1))"),
        "\"\\\"a\\nb\\\"|\\\"q\\\\\\\"\\\"|(\\\"x\\\" 1)\""
    );
}
