//! キーボードマクロ (`kbd_macro`) と, マクロを動かす組み込み `execute-kbd-macro` のテスト.
//! キーを打ち直す画面を含む動きは, `tui/tests/test_app.rs`.

use std::sync::Mutex;

use rseditor_core::editor::Editor;
use rseditor_core::kbd_macro::KbdMacro;
use rseditor_core::scheme::editor_host::EditorHost;
use rseditor_core::scheme::evaluator::Evaluator;
use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::value::Value;

#[test]
fn test_record_and_end() {
    let mut m = KbdMacro::default();
    // 覚えていなければ, 覚えない
    m.record("x");
    assert!(!m.defining() && m.last().is_empty());
    assert_eq!(m.end(), Err("Not defining kbd macro".to_string()));
    m.start(false).unwrap();
    assert_eq!(
        m.start(false),
        Err("Already defining kbd macro".to_string())
    );
    m.begin_command();
    m.record("a");
    m.begin_command();
    m.record("C-x");
    m.record(")");
    // 終えるコマンドのキーは, 入れない
    m.end().unwrap();
    assert_eq!(m.last(), ["a"]);
    // 続きに覚える
    m.start(true).unwrap();
    m.begin_command();
    m.record("b");
    m.begin_command();
    m.end().unwrap();
    assert_eq!(m.last(), ["a", "b"]);
    m.start(false).unwrap();
    m.end().unwrap();
    assert!(m.last().is_empty());
}

/// 画面の要る操作を覚える偽物.
#[derive(Default)]
struct FakeUi {
    calls: Mutex<Vec<(String, Vec<Value>)>>,
}

impl Host for FakeUi {
    fn call(&self, op: &str, args: Vec<Value>) -> Result<Value, String> {
        self.calls.lock().unwrap().push((op.to_string(), args));
        Ok(Value::Nil)
    }
}

#[test]
fn test_kbd_macro_operations() {
    let host = EditorHost::new(Editor::new(), FakeUi::default());
    let ev = Evaluator::new();
    let run = |code: &str| ev.eval_with(code, &host);
    assert_eq!(run("(kbd-macro-defining-p)").unwrap(), "#f");
    assert_eq!(run("(last-kbd-macro)").unwrap(), "()");
    run("(start-kbd-macro)").unwrap();
    assert_eq!(run("(kbd-macro-defining-p)").unwrap(), "#t");
    host.editor().kbd_macro.record("a");
    host.editor().kbd_macro.begin_command();
    run("(end-kbd-macro)").unwrap();
    assert_eq!(run("(last-kbd-macro)").unwrap(), r#"("a")"#);
    assert!(run("(end-kbd-macro)").is_err());
    // 動かすのは画面: キーの並びと回数を渡す
    run(r#"(execute-kbd-macro '("a") 3)"#).unwrap();
    run(r#"(execute-kbd-macro '("a"))"#).unwrap();
    let calls = host.ui().calls.lock().unwrap();
    let executed: Vec<_> = calls
        .iter()
        .filter(|(op, _)| op == "execute-kbd-macro")
        .map(|(_, args)| args[1].clone())
        .collect();
    assert_eq!(executed, [Value::Number(3.0), Value::Number(1.0)]);
    drop(calls);
    assert!(run("(execute-kbd-macro)").is_err());
    assert!(run("(execute-kbd-macro '() 1 2)").is_err());
}
