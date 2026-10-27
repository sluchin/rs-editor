//! undo と undo-redo (`lisp/simple.scm`) のテスト. 期待値は, `emacs -Q --batch` で編集と undo の手順を動かした結果
//! (`core/tests/undo.json`. `tools/syntax/run.sh` で作る).

use rseditor_core::editor::Editor;
use rseditor_core::scheme::editor_host::EditorHost;
use rseditor_core::scheme::evaluator::Evaluator;
use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::value::Value;
use serde_json::{json, Value as Json};

/// 画面の要る操作を受ける偽物: メッセージは捨てる.
struct FakeUi;

impl Host for FakeUi {
    fn call(&self, op: &str, _args: Vec<Value>) -> Result<Value, String> {
        match op {
            "message" => Ok(Value::Nil),
            other => Err(format!("ui: {other}")),
        }
    }
}

/// 表の 1 行: 名前, 初めの本文, 初めのカーソル, 手順, 動作ごとの結果.
type Row = (String, String, usize, Vec<Json>, Vec<Json>);

#[test]
fn test_against_emacs() {
    let table: Vec<Row> = serde_json::from_str(include_str!("undo.json")).unwrap();
    assert!(table.len() >= 7);
    let mut failures = Vec::new();
    for (name, text, point, steps, expected) in &table {
        let ev = Evaluator::new();
        let host = EditorHost::new(Editor::new(), FakeUi);
        ev.eval_with(include_str!("../../lisp/simple.scm"), &host)
            .unwrap();
        {
            let mut editor = host.editor();
            let buffer = editor.current_buffer_mut();
            buffer.load(text);
            buffer.set_point(*point);
        }
        // 前のコマンド: undo と undo-redo は, this-command を undo にするので, それを次の last-command にする
        let mut last = "#f".to_string();
        let mut actual = Vec::new();
        for step in steps {
            let kind = step[0].as_str().unwrap();
            let undoing = matches!(kind, "undo" | "undo-redo");
            let call = match kind {
                "ins" => format!("(insert {:?})", step[1].as_str().unwrap()),
                "del" => format!(
                    "(delete-region {} {})",
                    step[1].as_u64().unwrap() + 1,
                    step[2].as_u64().unwrap() + 1
                ),
                "goto" => format!("(goto-char {})", step[1].as_u64().unwrap() + 1),
                "undo" => match step.get(1) {
                    Some(n) => format!("(undo {n})"),
                    None => "(undo)".to_string(),
                },
                _ => "(undo-redo)".to_string(),
            };
            let this = if undoing {
                format!("'{kind}")
            } else {
                "#f".to_string()
            };

            // 動作の前に区切りを入れ, 前の動作が undo なら, last-command は undo
            let code = format!(
                "(undo-boundary) (set! last-command {last}) (set! this-command {this}) {call}"
            );
            let result = ev.eval_with(&code, &host);
            let editor = host.editor();
            let buffer = editor.current_buffer();
            actual.push(match result {
                Ok(_) => json!([buffer.text(), buffer.point()]),
                Err(message) => json!(["error", message, buffer.text(), buffer.point()]),
            });
            last = if undoing {
                let value = ev.eval_with("this-command", &host).unwrap();
                format!("'{}", value.lines().last().unwrap())
            } else {
                "#f".to_string()
            };
        }
        if &actual != expected {
            failures.push(format!(
                "{name}:\n  expected {}\n  got      {}",
                json!(expected),
                json!(actual)
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
