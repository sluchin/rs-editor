//! S 式, defun, 段落, 文のコマンド (`lisp/simple.scm`) のテスト. 期待値は, `emacs -Q --batch` で, モードごとに, 本文の全ての位置から対話的に動かした結果
//! (`core/tests/lisp.json`. `tools/syntax/run.sh` で作る).

use rseditor_core::editor::Editor;
use rseditor_core::kill_ring::KillRing;
use rseditor_core::scheme::editor_host::EditorHost;
use rseditor_core::scheme::evaluator::Evaluator;
use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::value::Value;
use rseditor_core::syntax::Dialect;
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
    mark: Option<usize>,
    (dialect, tabs): (Dialect, bool),
    text: &str,
    pos: usize,
) -> Json {
    {
        let mut editor = host.editor();
        editor.kill_ring = KillRing::new();
        let buffer = editor.current_buffer_mut();
        buffer.load(text);
        buffer.set_dialect(dialect);
        buffer.set_point(pos);
        if let Some(mark) = mark {
            buffer.set_mark(mark);
        }
    }
    *host.ui().message.lock().unwrap() = None;
    let mut code = String::from(
        "(set! simple--cycle-context #f) (set! simple--interactive #t) (set! last-command #f)\n",
    );
    code.push_str(&format!(
        "(set! indent-tabs-mode {})\n",
        if tabs { "#t" } else { "#f" }
    ));
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
        // キーから動かしたときと同じに, 前置引数を決めて, 引数なしで呼ぶ
        let prefix = match step[1].as_array().and_then(|a| a.first()) {
            Some(arg) => scheme_arg(arg),
            None => "#f".to_string(),
        };
        run.push_str(&format!("(set-current-prefix-arg {prefix})({command})"));
        code.push_str(&run);
        code.push('\n');
    }
    // 手順を動かす前に, コマンドの終わりにマークを外す頼みを消す (Emacs の command loop と同じ)
    host.editor()
        .current_buffer_mut()
        .set_deactivate_requested(false);
    // 手順は 1 つの式の並びとして, 失敗したところで止める (最後の式は, this-command)
    code.push_str("this-command\n");
    result = result.and_then(|_| ev.eval_with(&code, host));
    let editor = host.editor();
    let buffer = editor.current_buffer();
    let (body, point) = (buffer.text(), buffer.point());
    // 本文が動かす前と同じなら, null
    let body = if body == text {
        Json::Null
    } else {
        json!(body)
    };
    match result {
        Err(message) => json!(["error", message, body, point]),
        Ok(value) => {
            let top = editor.kill_ring.head().map(str::to_string);
            drop(editor);
            let this = Some(value.lines().last().unwrap_or("").to_string())
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
    let table: Vec<Row> = serde_json::from_str(include_str!("lisp.json")).unwrap();
    assert!(table.len() > 1000);
    let ev = Evaluator::new();
    let host = EditorHost::new(Editor::new(), FakeUi::default());
    ev.eval_with(include_str!("../../lisp/simple.scm"), &host)
        .unwrap();
    {
        host.editor().set_buffer("*scratch*").unwrap();
    }
    let mut failures = Vec::new();
    for (name, steps, _seeds, mark, text, results) in &table {
        if std::env::var("ONLY").is_ok_and(|only| !name.contains(&only)) {
            continue;
        }
        let (mode, _) = name.split_once(':').unwrap();
        let dialect = match mode {
            "emacs-lisp-mode" => Dialect::Elisp,
            "scheme-mode" => Dialect::Scheme,
            _ => Dialect::Fundamental,
        };
        for (pos, expected) in results.iter().enumerate() {
            if std::env::var("TRACE").is_ok() {
                eprintln!("TRACE {name} {text:?} {pos} {steps:?} {mark}");
            }
            let actual = run(
                &ev,
                &host,
                steps,
                mark.as_u64().map(|m| m as usize),
                (dialect, !name.ends_with("/notabs")),
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
            .take(40)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}
