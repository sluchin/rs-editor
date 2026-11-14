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
    /// 描き直した回数 (`redisplay`).
    redisplays: Mutex<usize>,
    /// `C-g` を押したことにする描き直しの回数 (0 なら押さない).
    quit_after: Mutex<usize>,
}

impl Host for Ui {
    fn call(&self, op: &str, args: Vec<Value>) -> Result<Value, rseditor_core::error::Error> {
        match (op, args.first()) {
            ("message", Some(Value::String(text))) => {
                self.messages.lock().unwrap().push(text.clone());
                Ok(Value::Nil)
            }
            ("local-set-key", _) => Ok(Value::Nil),
            ("redisplay", _) => {
                *self.redisplays.lock().unwrap() += 1;
                Ok(Value::Nil)
            }
            ("quit-requested-p", _) => {
                let quit_after = *self.quit_after.lock().unwrap();
                Ok(Value::Boolean(
                    quit_after > 0 && *self.redisplays.lock().unwrap() >= quit_after,
                ))
            }
            _ => Err(format!("ui: {op}").into()),
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

/// `*grep*` の本文を返す式.
const GREP_TEXT: &str = "(with-current-buffer \"*grep*\" (buffer-string))";

#[test]
fn test_rgrep_selects_files_by_wildcard() {
    let (_dir, path) = project();
    fs::write(format!("{path}/c.md"), "foo md\n").unwrap();
    let (text, _) = run(&[&format!("(rgrep \"foo\" \"*.txt\" \"{path}\")"), GREP_TEXT]);
    assert!(text.contains("a.txt:1:foo"), "{text}");
    assert!(text.contains("sub/b.txt:2:foo"), "{text}");
    assert!(!text.contains("c.md"), "{text}");
    let (text, _) = run(&[
        &format!("(rgrep \"foo\" \"*.md b.t?t\" \"{path}\")"),
        GREP_TEXT,
    ]);
    assert!(text.contains("c.md:1:foo md"), "{text}");
    assert!(text.contains("sub/b.txt:2:foo"), "{text}");
    assert!(!text.contains("a.txt"), "{text}");
}

#[test]
fn test_lgrep_stays_in_the_directory() {
    let (_dir, path) = project();
    let (text, messages) = run(&[&format!("(lgrep \"foo\" \"*\" \"{path}\")"), GREP_TEXT]);
    assert!(text.contains("a.txt:2:bar foo"), "{text}");
    assert!(!text.contains("sub/"), "{text}");
    assert_eq!(messages, vec!["Grep finished with 2 matches found"]);
}

#[test]
fn test_rgrep_reads_the_wildcard_and_directory_interactively() {
    let (_dir, path) = project();
    // 偽の画面は read-string を知らないので, 失敗の内容で, 聞いた順を確かめる
    let (result, _) = run(&["(rgrep \"foo\")"]);
    assert!(result.starts_with("error:"), "{result}");
    let (text, _) = run(&[
        &format!("(rgrep \"foo\" \"*.txt\" \"{path}/sub\")"),
        GREP_TEXT,
    ]);
    assert!(text.contains("b.txt:2:foo"), "{text}");
    assert!(!text.contains("a.txt"), "{text}");
}

#[test]
fn test_zrgrep_searches_inside_gz_files() {
    use std::io::Write;
    let (_dir, path) = project();
    let mut encoder = flate2::write::GzEncoder::new(
        fs::File::create(format!("{path}/z.txt.gz")).unwrap(),
        flate2::Compression::default(),
    );
    encoder.write_all(b"zipped foo\n").unwrap();
    encoder.finish().unwrap();
    let (text, messages) = run(&[
        &format!("(zrgrep \"foo\" \"*.gz *.txt\" \"{path}\")"),
        GREP_TEXT,
    ]);
    assert!(text.contains("zgrep (built-in)"), "{text}");
    assert!(text.contains("z.txt.gz:1:zipped foo"), "{text}");
    assert!(text.contains("a.txt:2:bar foo"), "{text}");
    assert_eq!(messages, vec!["Grep finished with 4 matches found"]);
    // もう一度, 同じ種類で探す
    let (text, _) = run(&[
        &format!("(zrgrep \"foo\" \"*.gz\" \"{path}\")"),
        "(grep-again)",
        GREP_TEXT,
    ]);
    assert!(text.contains("z.txt.gz:1:zipped foo"), "{text}");
    // 聞く
    let (result, _) = run(&["(zrgrep \"foo\")"]);
    assert!(result.starts_with("error:"), "{result}");
}

#[test]
fn test_find_grep_searches_all_files() {
    let (_dir, path) = project();
    let (text, messages) = run(&[&format!("(find-grep \"foo\" \"{path}\")"), GREP_TEXT]);
    assert!(
        text.contains("find . -type f -exec grep (built-in) -nH -e foo {} +"),
        "{text}"
    );
    assert!(text.contains("a.txt:2:bar foo"), "{text}");
    assert!(text.contains("sub/b.txt:2:foo"), "{text}");
    assert_eq!(messages, vec!["Grep finished with 3 matches found"]);
    let (result, _) = run(&["(find-grep \"foo\")"]);
    assert!(result.starts_with("error:"), "{result}");
}

#[test]
fn test_grep_again_runs_the_last_search() {
    let (_dir, path) = project();
    let (result, _) = run(&["(grep-again)"]);
    assert_eq!(result, "error: No grep to run again");
    let (text, messages) = run(&[
        &format!("(lgrep \"foo\" \"*.txt\" \"{path}\")"),
        &format!("(write-region \"foo foo\\n\" #f \"{path}/d.txt\")"),
        "(grep-again)",
        GREP_TEXT,
    ]);
    assert!(text.contains("d.txt:1:foo foo"), "{text}");
    assert_eq!(
        messages.last().unwrap(),
        "Grep finished with 3 matches found"
    );
}

/// 1 回に探すファイルを 1 つにして, 描き直しの回数と, 中断の様子を調べる.
fn run_batched(quit_after: usize, code: &str) -> (String, usize) {
    let (_dir, path) = project();
    let ev = Evaluator::new();
    let ui = Ui::default();
    *ui.quit_after.lock().unwrap() = quit_after;
    let host = EditorHost::new(Editor::new(), ui);
    for source in [
        include_str!("../../lisp/simple.scm"),
        include_str!("../../lisp/compile.scm"),
    ] {
        ev.eval_with(source, &host).unwrap();
    }
    ev.eval_with("(set! grep--batch-size 1)", &host).unwrap();
    ev.eval_with(&format!("(grep \"foo\" \"{path}\")"), &host)
        .unwrap();
    let text = ev.eval_with(code, &host).unwrap();
    let redisplays = *host.ui().redisplays.lock().unwrap();
    (text, redisplays)
}

#[test]
fn test_grep_shows_the_matches_batch_by_batch() {
    // 2 つのファイルを 1 つずつ探す: 描き直しは, 最後の 1 つの前に 1 回だけ
    let (text, redisplays) = run_batched(0, "(with-current-buffer \"*grep*\" (buffer-string))");
    assert_eq!(redisplays, 1);
    assert!(text.contains("a.txt:1:foo"));
    assert!(text.contains("sub/b.txt:2:foo"));
    assert!(text.contains("Grep finished with 3 matches found at "));
}

#[test]
fn test_grep_stops_when_c_g_is_pressed() {
    let (text, redisplays) = run_batched(1, "(with-current-buffer \"*grep*\" (buffer-string))");
    assert_eq!(redisplays, 1);
    assert!(text.contains("a.txt:2:bar foo"));
    assert!(!text.contains("sub/b.txt"));
    assert!(text.contains("Grep interrupted with 2 matches found at "));
}

#[test]
fn test_kill_compilation_reports_that_nothing_runs() {
    let (result, _) = run(&["(kill-compilation)"]);
    assert_eq!(result, "error: The *grep* process is not running");
}
