//! 置換のコマンド (`lisp/replace.scm`) のテスト. 期待値は, `emacs -Q --batch` で, 本文の全ての位置から動かした結果
//! (`core/tests/replace.json`. `tools/syntax/run.sh` で作る).

use std::collections::VecDeque;
use std::sync::Mutex;

use rseditor_core::editor::Editor;
use rseditor_core::scheme::editor_host::EditorHost;
use rseditor_core::scheme::evaluator::Evaluator;
use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::value::Value;
use serde_json::{json, Value as Json};

/// 画面の要る操作を受ける偽物: 最後のメッセージを覚え, `read-key` に決めた答えを返す.
#[derive(Default)]
struct FakeUi {
    message: Mutex<Option<String>>,
    keys: Mutex<VecDeque<String>>,
    prompt: Mutex<Option<String>>,
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
            "read-key" => match self.keys.lock().unwrap().pop_front() {
                Some(key)
                    if {
                        *self.prompt.lock().unwrap() = Some(text_of(&args));
                        true
                    } =>
                {
                    Ok(Value::String(key))
                }
                Some(key) => Ok(Value::String(key)),
                None => Err("no more keys".to_string()),
            },
            other => Err(format!("ui: {other}")),
        }
    }
}

/// 最初の引数の文字列.
fn text_of(args: &[Value]) -> String {
    match args.first() {
        Some(Value::String(s)) => s.clone(),
        _ => String::new(),
    }
}

/// 表の 1 行: 名前, 本文, Scheme の式, キー, 位置ごとの結果.
type Row = (String, String, String, Vec<String>, Vec<Json>);

/// 式を動かして, 結果を表の形にする.
fn run(ev: &Evaluator, host: &EditorHost<FakeUi>, row: &Row, pos: usize) -> Json {
    {
        let mut editor = host.editor();
        let buffer = editor.current_buffer_mut();
        buffer.load(&row.1);
        buffer.set_point(pos);
        buffer.deactivate_mark();
    }
    *host.ui().message.lock().unwrap() = None;
    *host.ui().prompt.lock().unwrap() = None;
    *host.ui().keys.lock().unwrap() = row.3.iter().cloned().collect();
    let result = ev.eval_with(&format!("(set! simple--interactive #t)\n{}", row.2), host);
    // 読まれなかったキーが残っていれば, Emacs は, 終わりのメッセージを出さない (最後の問いが残る)
    if !host.ui().keys.lock().unwrap().is_empty() {
        *host.ui().message.lock().unwrap() = Some(
            host.ui()
                .prompt
                .lock()
                .unwrap()
                .clone()
                .unwrap_or("Mark set".to_string()),
        );
    }
    let editor = host.editor();
    let buffer = editor.current_buffer();
    let body = buffer.text();
    let body = if body == row.1 {
        Json::Null
    } else {
        json!(body)
    };
    match result {
        Err(message) => json!(["error", message, body, buffer.point()]),
        Ok(_) => json!([
            body,
            buffer.point(),
            buffer.mark(),
            host.ui().message.lock().unwrap().clone(),
            buffer.mark_active()
        ]),
    }
}

#[test]
fn test_against_emacs() {
    let table: Vec<(String, String, String, Vec<String>, Vec<Json>)> =
        serde_json::from_str(include_str!("replace.json")).unwrap();
    assert!(table.len() > 100);
    let ev = Evaluator::new();
    let host = EditorHost::new(Editor::new(), FakeUi::default());
    ev.eval_with(include_str!("../../lisp/simple.scm"), &host)
        .unwrap();
    ev.eval_with(include_str!("../../lisp/replace.scm"), &host)
        .unwrap();
    host.editor().set_buffer("*scratch*").unwrap();
    let mut failures = Vec::new();
    for row in &table {
        if std::env::var("ONLY").is_ok_and(|only| !row.0.contains(&only)) {
            continue;
        }
        for (pos, expected) in row.4.iter().enumerate() {
            let actual = run(&ev, &host, row, pos);
            if &actual != expected {
                failures.push(format!(
                    "{} {:?} at {pos}: expected {expected}, got {actual}",
                    row.0, row.1
                ));
            }
        }
    }
    let mut summary = std::collections::BTreeMap::new();
    for f in &failures {
        *summary
            .entry(f.split(' ').next().unwrap().to_string())
            .or_insert(0) += 1;
    }
    assert!(
        failures.is_empty(),
        "{} differences:\n{:#?}\n{}",
        failures.len(),
        summary,
        failures
            .iter()
            .filter(|f| std::env::var("SHOW").is_ok_and(|s| f.contains(&s)))
            .take(30)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}
