//! `isearch-forward` などの (`lisp/isearch.scm`) テスト. 期待値は, `emacs -Q --batch` で, キーを同じ名前のコマンドに直して順に呼んだ結果
//! (`core/tests/isearch.json`. `tools/syntax/run.sh` で作る): 終わったときのカーソル, マーク, メッセージ.

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
}

impl Host for FakeUi {
    fn call(&self, op: &str, args: Vec<Value>) -> Result<Value, rseditor_core::error::Error> {
        match op {
            "message" => {
                if let Some(Value::String(text)) = args.first() {
                    *self.message.lock().unwrap() = Some(text.clone());
                }
                Ok(Value::Nil)
            }
            "read-key" => {
                // Emacs は, 入力のたびに, 問い (エコー) を出す
                if let Some(Value::String(text)) = args.first() {
                    *self.message.lock().unwrap() = Some(text.clone());
                }
                self.keys
                    .lock()
                    .unwrap()
                    .pop_front()
                    .map(Value::String)
                    .ok_or_else(|| "no more keys".into())
            }
            other => Err(format!("ui: {other}").into()),
        }
    }
}

/// 表の 1 行: 開始のコマンド, 本文, キー, 位置ごとの結果.
type Row = (String, String, Vec<String>, Vec<Json>);

/// 手順を動かして, 結果を表の形にする.
fn run(ev: &Evaluator, host: &EditorHost<FakeUi>, row: &Row, pos: usize) -> Json {
    {
        let mut editor = host.editor();
        let buffer = editor.current_buffer_mut();
        buffer.load(&row.1);
        buffer.set_point(pos);
        buffer.deactivate_mark();
        buffer.set_mark(pos);
        buffer.deactivate_mark();
    }
    *host.ui().message.lock().unwrap() = None;
    // Emacs の側は, 終わっていなければ Enter で終える
    let mut keys: Vec<String> = row.2.iter().filter(|k| *k != "Enter").cloned().collect();
    keys.push("Enter".to_string());
    *host.ui().keys.lock().unwrap() = keys.into_iter().collect();
    let code = format!(
        "(set! isearch-ring '()) (set! isearch-regexp-ring '()) (set-current-prefix-arg #f) ({})",
        row.0
    );
    let result = ev.eval_with(&code, host);
    let editor = host.editor();
    let buffer = editor.current_buffer();
    // マークは, 動かす前に置いた位置のままなら, 置いていない
    let mark = buffer.mark().filter(|m| *m != pos || buffer.point() != pos);
    match result {
        Err(e) if e == "Quit" => json!(["quit", buffer.point(), None::<usize>]),
        Err(e) => json!(["error", e.to_string()]),
        Ok(_) => json!([
            buffer.point(),
            mark,
            host.ui().message.lock().unwrap().clone()
        ]),
    }
}

#[test]
fn test_against_emacs() {
    let table: Vec<Row> = serde_json::from_str(include_str!("isearch.json")).unwrap();
    assert!(table.len() > 100);
    // 手順を何本かのスレッドに分けて動かす (手順ごとに独立しているので, 本数だけ速くなる)
    let setup = || {
        let ev = Evaluator::new();
        let host = EditorHost::new(Editor::new(), FakeUi::default());
        ev.eval_with(include_str!("../../lisp/simple.scm"), &host)
            .unwrap();
        ev.eval_with(include_str!("../../lisp/replace.scm"), &host)
            .unwrap();
        ev.eval_with(include_str!("../../lisp/isearch.scm"), &host)
            .unwrap();
        host.editor().set_buffer("*scratch*").unwrap();
        (ev, host)
    };
    let workers = std::thread::available_parallelism().map_or(1, |n| n.get());
    let failures: Vec<String> = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..workers)
            .map(|worker| {
                let (table, setup) = (&table, &setup);
                scope.spawn(move || {
                    let (ev, host) = setup();
                    let mut failures = Vec::new();
                    for row in table.iter().skip(worker).step_by(workers) {
                        if std::env::var("ONLY").is_ok_and(|only| !row.0.contains(&only)) {
                            continue;
                        }
                        for (pos, expected) in row.3.iter().enumerate() {
                            if expected[0] == "skip" {
                                continue;
                            }
                            let actual = run(&ev, &host, row, pos);
                            if &actual != expected {
                                failures.push(format!(
                                    "{} {:?} {:?} at {pos}: expected {expected}, got {actual}",
                                    row.0, row.1, row.2
                                ));
                            }
                        }
                    }
                    failures
                })
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|handle| handle.join().unwrap())
            .collect()
    });
    let mut summary = std::collections::BTreeMap::new();
    for f in &failures {
        let key = f.split(" at ").next().unwrap();
        let key = key.split(" \"").next().unwrap().to_string()
            + " "
            + key.rsplit_once(' ').map_or("", |(_, k)| k);
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
            .take(20)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}

/// 本文とカーソルを決めて, 手順を動かし, カーソルと最後のメッセージを返す.
fn thing_at_point(
    text: &str,
    mark: Option<usize>,
    pos: usize,
    keys: &[&str],
) -> (usize, Option<String>) {
    let ev = Evaluator::new();
    let host = EditorHost::new(Editor::new(), FakeUi::default());
    for source in [
        include_str!("../../lisp/simple.scm"),
        include_str!("../../lisp/replace.scm"),
        include_str!("../../lisp/isearch.scm"),
    ] {
        ev.eval_with(source, &host).unwrap();
    }
    host.editor().set_buffer("*scratch*").unwrap();
    {
        let mut editor = host.editor();
        let buffer = editor.current_buffer_mut();
        buffer.load(text);
        buffer.set_point(pos);
        if let Some(mark) = mark {
            buffer.set_mark(mark);
            buffer.set_point(pos);
        }
    }
    *host.ui().keys.lock().unwrap() = keys.iter().map(|k| k.to_string()).collect();
    // 入力が尽きたら失敗する (そのときの問いが, 最後のメッセージ)
    let _ = ev.eval_with("(isearch-forward-thing-at-point)", &host);
    let point = host.editor().current_buffer().point();
    let message = host.ui().message.lock().unwrap().clone();
    (point, message)
}

#[test]
fn test_thing_at_point_uses_the_region_and_reports_no_thing() {
    // 期待値は, `emacs -Q --batch` の結果
    // リージョンがあれば, その文字を探す文字列にする (カーソルがリージョンの終わりなら, 始めから探す)
    let (point, _) = thing_at_point("foo bar foo bar", Some(4), 7, &["Enter"]);
    assert_eq!(point, 7);
    // 物が無ければ, 「物が無い」と出して, そのまま入力を待つ
    let (_, message) = thing_at_point("   ", None, 1, &[]);
    assert_eq!(message.as_deref(), Some("I-search:  [No thing at point]"));
}
