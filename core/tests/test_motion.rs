//! 動きのコマンド (`lisp/simple.scm`) のテスト. 期待値は, `emacs -Q --batch` で本文の全ての位置から動かした結果
//! (`core/tests/motion.json`. `tools/syntax/run.sh` で作る).

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

/// 基本の編集のコマンドを読んだ、評価器とホスト.
fn setup() -> (Evaluator, EditorHost<FakeUi>) {
    let ev = Evaluator::new();
    let host = EditorHost::new(Editor::new(), FakeUi);
    ev.eval_with(include_str!("../../lisp/simple.scm"), &host)
        .unwrap();
    (ev, host)
}

/// 本文とカーソルを決める.
fn place(host: &EditorHost<FakeUi>, text: &str, pos: usize) {
    let mut editor = host.editor();
    let buffer = editor.current_buffer_mut();
    buffer.load(text);
    buffer.set_point(pos);
}

/// カーソルとマーク.
fn state(host: &EditorHost<FakeUi>) -> (usize, Option<usize>) {
    let editor = host.editor();
    let buffer = editor.current_buffer();
    (buffer.point(), buffer.mark())
}

/// 結果を、表の形にする.
fn outcome(host: &EditorHost<FakeUi>, result: Result<String, String>) -> Json {
    let (point, mark) = state(host);
    match result {
        Ok(_) => json!([point, mark]),
        Err(message) => json!(["error", message, point]),
    }
}

#[test]
fn test_against_emacs() {
    let table: Vec<(String, Json, String, Vec<Json>)> =
        serde_json::from_str(include_str!("motion.json")).unwrap();
    assert!(table.len() > 300);
    let (ev, host) = setup();
    let mut failures = Vec::new();
    for (command, arg, text, results) in &table {
        for (pos, expected) in results.iter().enumerate() {
            place(&host, text, pos);
            let actual = if command == "set-goal-column" {
                goal_column(&ev, &host, text, pos)
            } else {
                let call = match arg {
                    Json::Number(n) => format!("({command} {n})"),
                    _ => format!("({command})"),
                };
                let code = format!("(set! last-command #f) (set! goal-column #f) {call}");
                let result = ev.eval_with(&code, &host);
                outcome(&host, result)
            };
            if &actual != expected {
                failures.push(format!(
                    "({command} {arg}) {text:?} at {pos}: expected {expected}, got {actual}"
                ));
            }
        }
    }
    let mut summary = std::collections::BTreeMap::new();
    for f in &failures {
        let key = f.split(" at ").next().unwrap().to_string();
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
            .take(8)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}

/// 位置で目標の桁を決めてから、先頭で next-line を 2 回, previous-line を 1 回.
fn goal_column(ev: &Evaluator, host: &EditorHost<FakeUi>, text: &str, pos: usize) -> Json {
    let run = |code: &str| ev.eval_with(&format!("(set! last-command #f) {code}"), host);
    let mut points = Vec::new();
    let result = run("(set! goal-column #f) (set-goal-column #f)").and_then(|_| {
        host.editor().current_buffer_mut().set_point(0);
        for (i, step) in ["(next-line 1)", "(next-line 1)", "(previous-line 1)"]
            .into_iter()
            .enumerate()
        {
            // 続けて動かすので、前のコマンドは、動いたコマンド
            ev.eval_with(
                &format!(
                    "(set! last-command {}) {step}",
                    if i == 0 { "#f" } else { "'next-line" }
                ),
                host,
            )?;
            points.push(state(host).0);
        }
        Ok(String::new())
    });
    let _ = (text, pos);
    ev.eval_with("(set! goal-column #f)", host).unwrap();
    match result {
        Ok(_) => json!(points),
        Err(message) => json!(["error", message, state(host).0]),
    }
}
