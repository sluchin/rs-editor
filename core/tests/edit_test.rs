//! 編集のコマンド (`lisp/simple.scm`) のテスト. 期待値は, `emacs -Q --batch` で本文の全ての位置から動かした結果
//! (`core/tests/edit.json`. `tools/syntax/run.sh` で作る).

use rseditor_core::editor::Editor;
use rseditor_core::kill_ring::KillRing;
use rseditor_core::scheme::editor_host::EditorHost;
use rseditor_core::scheme::evaluator::Evaluator;
use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::value::Value;
use serde_json::{json, Value as Json};

/// 画面の要る操作を受ける偽物: 最後のメッセージを覚える.
#[derive(Default)]
struct FakeUi {
    message: std::sync::Mutex<Option<String>>,
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
            other => Err(format!("ui: {other}")),
        }
    }
}

/// 引数を、Scheme の式の文字にする.
fn scheme_arg(arg: &Json) -> String {
    match arg {
        Json::Null => "#f".to_string(),
        Json::Bool(true) => "#t".to_string(),
        Json::Bool(false) => "#f".to_string(),
        Json::Number(n) => n.to_string(),
        Json::String(s) => match s.strip_prefix("sym:") {
            Some(name) => format!("'{name}"),
            None => format!("{s:?}"),
        },
        Json::Object(o) if o.contains_key("c") => format!("{:?}", o["c"].as_str().unwrap()),
        Json::Object(o) if o.contains_key("l") => {
            let items: Vec<String> = o["l"].as_array().unwrap().iter().map(scheme_arg).collect();
            format!("'({})", items.join(" "))
        }
        other => panic!("unknown argument {other}"),
    }
}

/// 手順を動かして、結果を表の形にする.
fn run(
    ev: &Evaluator,
    host: &EditorHost<FakeUi>,
    steps: &[Json],
    seeds: &[Json],
    mark: Option<usize>,
    text: &str,
    pos: usize,
) -> Json {
    {
        let mut editor = host.editor();
        editor.kill_ring = KillRing::new();
        let buffer = editor.current_buffer_mut();
        buffer.load(text);
        buffer.set_point(pos);
        if let Some(mark) = mark {
            buffer.set_mark(mark);
        }
    }
    *host.ui().message.lock().unwrap() = None;
    let mut code = String::from("(set! simple--cycle-context #f) (set! last-command #f)\n");
    for seed in seeds {
        code.push_str(&format!("(kill-new {})\n", scheme_arg(seed)));
    }
    let mut result = Ok(String::new());
    for step in steps {
        let command = step[0].as_str().unwrap();
        let mut run = String::new();
        if step[2] != "keep" {
            run.push_str(&match step[2].as_str() {
                Some(last) => format!("(set! last-command '{last})"),
                None => "(set! last-command #f)".to_string(),
            });
        }
        run.push_str(&format!("(set! this-command '{command})"));
        let args = match &step[1] {
            Json::Array(items) => items.iter().map(scheme_arg).collect::<Vec<_>>().join(" "),
            _ => String::new(),
        };
        run.push_str(&format!("({command} {args})"));
        code.push_str(&run);
        code.push('\n');
    }
    // 手順を動かす前に, コマンドの終わりにマークを外す頼みを消す (Emacs の command loop と同じ)
    host.editor()
        .current_buffer_mut()
        .set_deactivate_requested(false);
    // 手順は 1 つの式の並びとして, 失敗したところで止める
    result = result.and_then(|_| ev.eval_with(&code, host));
    let editor = host.editor();
    let buffer = editor.current_buffer();
    let (body, point) = (buffer.text(), buffer.point());
    match result {
        Err(message) => json!(["error", message, body, point]),
        Ok(_) => {
            let top = editor.kill_ring.head().map(str::to_string);
            drop(editor);
            let this = ev
                .eval_with("this-command", host)
                .ok()
                .map(|s| s.lines().last().unwrap_or("").to_string())
                .filter(|s| s != "#f" && s != "nil");
            let editor = host.editor();
            let buffer = editor.current_buffer();
            json!([
                body,
                point,
                top,
                buffer.mark(),
                this,
                buffer.mark_active(),
                buffer.deactivate_requested(),
                host.ui().message.lock().unwrap().clone()
            ])
        }
    }
}

/// 表の 1 行: 名前, 手順, 種, マーク, 本文, 位置ごとの結果.
type Row = (String, Vec<Json>, Vec<Json>, Json, String, Vec<Json>);

#[test]
fn test_against_emacs() {
    let table: Vec<Row> = serde_json::from_str(include_str!("edit.json")).unwrap();
    assert!(table.len() > 300);
    let ev = Evaluator::new();
    let host = EditorHost::new(Editor::new(), FakeUi::default());
    ev.eval_with(include_str!("../../lisp/simple.scm"), &host)
        .unwrap();
    {
        let mut editor = host.editor();
        editor.get_buffer_create("other").load("OTHER");
        editor.set_buffer("*scratch*").unwrap();
    }
    let mut failures = Vec::new();
    for (name, steps, seeds, mark, text, results) in &table {
        for (pos, expected) in results.iter().enumerate() {
            let actual = run(
                &ev,
                &host,
                steps,
                seeds,
                mark.as_u64().map(|m| m as usize),
                text,
                pos,
            );
            if &actual != expected {
                failures.push(format!(
                    "{name} {} {text:?} at {pos}: expected {expected}, got {actual}",
                    json!(steps.iter().map(|s| s[1].clone()).collect::<Vec<_>>())
                ));
            }
        }
    }
    let mut summary = std::collections::BTreeMap::new();
    for f in &failures {
        let key = f.split(" at ").next().unwrap();
        let key = key.split(" \"").next().unwrap().to_string();
        *summary.entry(key).or_insert(0) += 1;
    }
    assert!(
        failures.is_empty(),
        "{} differences:\n{:#?}\n{}",
        failures.len(),
        summary,
        failures
            .iter()
            .filter(|f| std::env::var("SHOW").is_ok_and(|s| f.contains(&s)))
            .take(6)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}
