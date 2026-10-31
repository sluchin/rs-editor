//! `occur` (`lisp/replace.scm`) のテスト. 期待値は, `emacs -Q --batch` で動かした結果 (`core/tests/occur.json`.
//! `tools/syntax/run.sh` で作る): *Occur* バッファの本文と, 出したメッセージ.

use std::sync::Mutex;

use rseditor_core::editor::Editor;
use rseditor_core::scheme::editor_host::EditorHost;
use rseditor_core::scheme::evaluator::Evaluator;
use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::value::Value;
use serde_json::{json, Value as Json};

/// 画面の要る操作を受ける偽物: 最後のメッセージを覚える.
#[derive(Default)]
struct FakeUi {
    message: Mutex<Option<String>>,
}

impl Host for FakeUi {
    fn call(&self, op: &str, args: Vec<Value>) -> Result<Value, String> {
        match op {
            "message" => {
                if let Some(Value::String(text)) = args.first() {
                    *self.message.lock().unwrap() = Some(text.clone());
                }
                Ok(Value::Nil)
            }
            "local-set-key" => Ok(Value::Nil),
            other => Err(format!("ui: {other}")),
        }
    }
}

/// (本文, 正規表現, 行数, 期待する *Occur* の本文, 期待するメッセージ)
type Case = (String, String, Option<i64>, Option<String>, Option<String>);

#[test]
fn test_against_emacs() {
    let table: Vec<Case> = serde_json::from_str(include_str!("occur.json")).unwrap();
    assert!(table.len() > 100);
    let ev = Evaluator::new();
    let host = EditorHost::new(Editor::new(), FakeUi::default());
    ev.eval_with(include_str!("../../lisp/simple.scm"), &host)
        .unwrap();
    ev.eval_with(include_str!("../../lisp/replace.scm"), &host)
        .unwrap();
    let mut failures = Vec::new();
    for (text, regexp, nlines, expected, message) in &table {
        {
            let mut editor = host.editor();
            editor.get_buffer_create("src");
            editor.set_buffer("src").unwrap();
            if editor.get_buffer("*Occur*").is_some() {
                editor.kill_buffer("*Occur*").unwrap();
            }
            let buffer = editor.current_buffer_mut();
            buffer.load(text);
            buffer.set_point(0);
        }
        *host.ui().message.lock().unwrap() = None;
        let nlines = nlines.map_or("#f".to_string(), |n| n.to_string());
        let code = format!("(occur {regexp:?} {nlines})");
        let result = ev.eval_with(&code, &host);
        let actual_text = host
            .editor()
            .get_buffer("*Occur*")
            .map(|b| b.text().to_string());
        let actual = json!([
            result.err(),
            actual_text,
            host.ui().message.lock().unwrap().clone()
        ]);
        let want = json!([Json::Null, expected, message]);
        if actual != want {
            failures.push(format!(
                "{text:?} {regexp:?} {nlines}: expected {want}, got {actual}"
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "{} differences:\n{}",
        failures.len(),
        failures
            .iter()
            .take(8)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}
