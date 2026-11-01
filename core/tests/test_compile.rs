//! `grep` と, `next-error` の仲間 (`lisp/compile.scm`) のテスト.

use std::fs;
use std::sync::Mutex;

use rseditor_core::editor::Editor;
use rseditor_core::scheme::editor_host::EditorHost;
use rseditor_core::scheme::evaluator::Evaluator;
use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::value::Value;
use tempfile::tempdir;

/// メッセージを覚える画面の偽物.
#[derive(Default)]
struct Ui {
    messages: Mutex<Vec<String>>,
}

impl Host for Ui {
    fn call(&self, op: &str, args: Vec<Value>) -> Result<Value, String> {
        match (op, args.first()) {
            ("message", Some(Value::String(text))) => {
                self.messages.lock().unwrap().push(text.clone());
                Ok(Value::Nil)
            }
            _ => Err(format!("ui: {op}")),
        }
    }
}

/// 探すファイルのあるディレクトリを作る.
fn project() -> (tempfile::TempDir, String) {
    let dir = tempdir().unwrap();
    let path = dir.path().to_str().unwrap().to_string();
    fs::create_dir(format!("{path}/sub")).unwrap();
    fs::create_dir(format!("{path}/.git")).unwrap();
    fs::write(format!("{path}/a.txt"), "foo\nbar foo\n").unwrap();
    fs::write(format!("{path}/sub/b.txt"), "x\nfoo\n").unwrap();
    fs::write(format!("{path}/.git/c.txt"), "foo\n").unwrap();
    (dir, path)
}

/// 式を順に評価して, 最後の式の結果の最後の行と, 出したメッセージを返す (失敗は `error: 理由`).
fn run(codes: &[&str]) -> (String, Vec<String>) {
    let ev = Evaluator::new();
    let host = EditorHost::new(Editor::new(), Ui::default());
    for source in [
        include_str!("../../lisp/simple.scm"),
        include_str!("../../lisp/compile.scm"),
        include_str!("../../lisp/xref.scm"),
    ] {
        ev.eval_with(source, &host).unwrap();
    }
    let mut result = String::new();
    for code in codes {
        result = match ev.eval_with(code, &host) {
            Ok(value) => value.lines().last().unwrap_or("").to_string(),
            Err(message) => format!("error: {message}"),
        };
    }
    let messages = host.ui().messages.lock().unwrap().clone();
    (result, messages)
}

/// 今のバッファのファイルの名前と, 点の行の文字を返す式.
const WHERE: &str = "(list (file-name-nondirectory (buffer-file-name)) (buffer-substring (line-beginning-position) (line-end-position)))";

#[test]
fn test_grep_lists_the_matches() {
    let (_dir, path) = project();
    let (text, messages) = run(&[
        &format!("(grep \"foo\" \"{path}\")"),
        "(with-current-buffer \"*grep*\" (buffer-string))",
    ]);
    assert!(text.contains("a.txt:1:foo"), "{text}");
    assert_eq!(messages, vec!["Grep finished with 3 matches found"]);
    let (text, _) = run(&[
        &format!("(grep \"foo\" \"{path}\")"),
        "(with-current-buffer \"*grep*\" (buffer-string))",
    ]);
    // 出力は 1 行にまとまっている (\n は文字のまま)
    assert!(text.contains("a.txt:1:foo\\na.txt:2:bar foo\\nsub/b.txt:2:foo\\n"));
    assert!(!text.contains(".git"));
    assert!(text.contains("grep (built-in) -r -e foo ."));
    assert!(text.contains("Grep finished with 3 matches found at "));
}

#[test]
fn test_grep_counts() {
    let (_dir, path) = project();
    let (_, messages) = run(&[&format!("(grep \"nothing-here\" \"{path}\")")]);
    assert_eq!(messages, vec!["Grep finished with no matches found"]);
    let (_, messages) = run(&[&format!("(grep \"x\" \"{path}/\")")]);
    assert_eq!(messages, vec!["Grep finished with 1 match found"]);
}

#[test]
fn test_next_error_and_previous_error() {
    let (_dir, path) = project();
    let start = format!("(grep \"foo\" \"{path}\")");
    let next = |n: usize| {
        let mut codes = vec![start.as_str()];
        codes.extend(std::iter::repeat_n("(next-error)", n));
        codes.push(WHERE);
        run(&codes).0
    };
    assert_eq!(next(1), "(\"a.txt\" \"foo\")");
    assert_eq!(next(2), "(\"a.txt\" \"bar foo\")");
    assert_eq!(next(3), "(\"b.txt\" \"foo\")");
    assert_eq!(
        run(&[
            &start,
            "(next-error)",
            "(next-error)",
            "(next-error)",
            "(next-error)"
        ])
        .0,
        "error: Moved past last grep hit"
    );
    assert_eq!(
        run(&[
            &start,
            "(next-error)",
            "(next-error)",
            "(previous-error)",
            WHERE
        ])
        .0,
        "(\"a.txt\" \"foo\")"
    );
    assert_eq!(
        run(&[&start, "(previous-error)"]).0,
        "error: Moved back before first grep hit"
    );
    assert_eq!(
        run(&[
            &start,
            "(next-error)",
            "(next-error)",
            "(first-error)",
            WHERE
        ])
        .0,
        "(\"a.txt\" \"foo\")"
    );
}

#[test]
fn test_next_error_without_results() {
    assert_eq!(
        run(&["(next-error)"]).0,
        "error: No buffers contain error message locations"
    );
    assert_eq!(
        run(&["(first-error)"]).0,
        "error: No buffers contain error message locations"
    );
}

#[test]
fn test_compile_goto_error() {
    let (_dir, path) = project();
    let start = format!("(grep \"foo\" \"{path}\")");
    // 見出しの行には, 場所が無い
    assert_eq!(
        run(&[&start, "(set-buffer \"*grep*\")", "(compile-goto-error)"]).0,
        "error: No error here"
    );
    // 合った行で呼ぶと, その場所へ移り, 次の `next-error` はその次から
    assert_eq!(
        run(&[
            &start,
            "(set-buffer \"*grep*\")",
            "(goto-char (point-min))",
            "(search-forward \"sub/b.txt\")",
            "(compile-goto-error)",
            WHERE,
        ])
        .0,
        "(\"b.txt\" \"foo\")"
    );
}

#[test]
fn test_next_error_follows_xref_references() {
    let (_dir, path) = project();
    let (result, _) = run(&[
        &format!("(set! xref-project-directory \"{path}\")"),
        "(xref-find-references \"bar\")",
        "(next-error)",
        WHERE,
    ]);
    assert_eq!(result, "(\"a.txt\" \"bar foo\")");
}
