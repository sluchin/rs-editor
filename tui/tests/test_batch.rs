//! バッチモード (`rseditor-tui --batch`) のテスト.

use std::io::{Cursor, Write};
use std::sync::{Arc, Mutex};

use rseditor_tui::batch::{run, Io, ERROR_STATUS};

/// 書いたものを覚える出力先.
#[derive(Clone, Default)]
struct Sink(Arc<Mutex<Vec<u8>>>);

impl Write for Sink {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Sink {
    fn text(&self) -> String {
        String::from_utf8(self.0.lock().unwrap().clone()).unwrap()
    }
}

/// 動かして (終了コード, 標準出力, 標準エラー出力) を返す.
fn batch(args: &[&str], input: &str) -> (i32, String, String) {
    let (out, err) = (Sink::default(), Sink::default());
    let io = Io {
        out: Box::new(out.clone()),
        err: Box::new(err.clone()),
        input: Box::new(Cursor::new(input.as_bytes().to_vec())),
    };
    let args: Vec<String> = args.iter().map(|a| a.to_string()).collect();
    let status = run(&args, io);
    (status, out.text(), err.text())
}

#[test]
fn test_eval_in_order() {
    let (status, out, err) = batch(
        &[
            "--batch",
            "--eval",
            "(princ 1)",
            "--eval=(princ 2)",
            "-eval",
            "(princ 3)",
            "--execute",
            "(princ 4)",
            "-execute",
            "(princ 5)",
        ],
        "",
    );
    assert_eq!((status, out.as_str(), err.as_str()), (0, "12345", ""));
}

#[test]
fn test_noninteractive_is_true() {
    let (_, out, _) = batch(&["-batch", "--eval", "(princ noninteractive)"], "");
    assert_eq!(out, "#t");
}

#[test]
fn test_load_and_funcall() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("f.scm");
    std::fs::write(&file, "(define (hello) (princ \"hello\"))").unwrap();
    let file = file.to_str().unwrap();
    for option in ["-l", "--load", "--script"] {
        let (status, out, _) = batch(&[option, file, "-f", "hello"], "");
        assert_eq!((status, out.as_str()), (0, "hello"), "{option}");
    }
    let (_, out, _) = batch(
        &[
            &format!("--load={file}"),
            "--funcall=hello",
            "-funcall",
            "hello",
        ],
        "",
    );
    assert_eq!(out, "hellohello");
    let (_, out, _) = batch(&["--funcall", "hello", "--eval", "(princ 1)"], "");
    assert_eq!(out, "");
}

#[test]
fn test_errors_exit_with_255() {
    let (status, out, err) = batch(
        &[
            "--eval",
            "(princ 1)",
            "--eval",
            "(error \"boom\")",
            "--eval",
            "(princ 2)",
        ],
        "",
    );
    assert_eq!(
        (status, out.as_str(), err.as_str()),
        (ERROR_STATUS, "1", "boom\n")
    );
    let (status, _, err) = batch(&["-l", "/none/f.scm"], "");
    assert_eq!(status, ERROR_STATUS);
    assert_eq!(
        err,
        "Cannot open load file: No such file or directory, /none/f.scm\n"
    );
    let (status, _, err) = batch(&["--bogus"], "");
    assert_eq!(
        (status, err.as_str()),
        (ERROR_STATUS, "Unknown option `--bogus'\n")
    );
    for option in ["-l", "--eval", "-f"] {
        let (status, _, err) = batch(&[option], "");
        assert_eq!(
            (status, err),
            (
                ERROR_STATUS,
                format!("Option `{option}' requires an argument\n")
            )
        );
    }
}

#[test]
fn test_kill_emacs() {
    let (status, out, _) = batch(
        &[
            "--eval",
            "(princ 1)",
            "--eval",
            "(kill-emacs 3)",
            "--eval",
            "(princ 2)",
        ],
        "",
    );
    assert_eq!((status, out.as_str()), (3, "1"));
    assert_eq!(batch(&["--eval", "(kill-emacs)"], "").0, 0);
    assert_eq!(batch(&["--eval", "(kill-emacs \"x\")"], "").0, 0);
}

#[test]
fn test_message_goes_to_stderr() {
    let (status, out, err) = batch(
        &["--eval", "(message \"a %d\" 1)", "--eval", "(princ \"b\")"],
        "",
    );
    assert_eq!((status, out.as_str(), err.as_str()), (0, "b", "a 1\n"));
}

#[test]
fn test_reading_from_stdin() {
    let (_, out, _) = batch(
        &[
            "--eval",
            "(princ (list (read-string \"a: \") (read-file-name \"f: \") (completing-read \"c: \" '(\"x\"))))",
            "--eval",
            "(princ (list (read-key \"k\") (read-key \"k\")))",
        ],
        "one\r\ntwo\nthree\nxyz\n\n",
    );
    assert_eq!(out, "(one two three)(x )");
    let (status, _, err) = batch(&["--eval", "(read-string \"a: \")"], "");
    assert_eq!(
        (status, err.as_str()),
        (ERROR_STATUS, "Error reading from stdin\n")
    );
}

#[test]
fn test_visiting_a_file() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("a.txt");
    std::fs::write(&file, "old\n").unwrap();
    let (status, out, _) = batch(
        &[
            file.to_str().unwrap(),
            "--eval",
            "(princ (list (buffer-name) (buffer-string)))",
        ],
        "",
    );
    assert_eq!((status, out.as_str()), (0, "(a.txt old\n)"));
    // ディレクトリは dired
    let (status, out, _) = batch(
        &[
            dir.path().to_str().unwrap(),
            "--eval",
            "(princ (buffer-name))",
        ],
        "",
    );
    assert_eq!(
        (status, out.as_str()),
        (0, dir.path().file_name().unwrap().to_str().unwrap())
    );
}

#[test]
fn test_relative_names_start_from_the_current_directory() {
    let (_, out, _) = batch(&["--eval", "(princ (expand-file-name \"x\"))"], "");
    let expected = std::env::current_dir().unwrap().join("x");
    assert_eq!(out, expected.to_str().unwrap());
}

#[test]
fn test_screen_operations_are_ignored_or_refused() {
    let (status, out, _) = batch(
        &[
            "--eval",
            "(set-background-color \"black\")",
            "--eval",
            "(princ (list (outline-invisible-p 1) (editor-command-names)))",
        ],
        "",
    );
    assert_eq!((status, out.as_str()), (0, "(#f ())"));
    let (status, _, err) = batch(&["--eval", "(recenter)"], "");
    assert_eq!(status, ERROR_STATUS);
    assert_eq!(err, "The batch mode does not support recenter\n");
}

#[test]
fn test_write_region_is_silent() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("w.txt");
    let code = format!("(write-region \"x\" #f {:?})", file.to_str().unwrap());
    let (status, out, err) = batch(&["--eval", &code], "");
    assert_eq!((status, out.as_str(), err.as_str()), (0, "", ""));
    assert_eq!(std::fs::read_to_string(&file).unwrap(), "x");
}

#[test]
fn test_message_after_output_starts_a_new_line() {
    // 標準出力に書いた後の、初めの message の前に、標準エラー出力に改行を出す (Emacs と同じ)
    let (_, out, err) = batch(
        &[
            "--eval",
            "(message \"a\")",
            "--eval",
            "(princ \"x\")",
            "--eval",
            "(message \"b\")",
            "--eval",
            "(message \"c\")",
        ],
        "",
    );
    assert_eq!((out.as_str(), err.as_str()), ("x", "a\n\nb\nc\n"));
}

#[test]
fn test_do_auto_save_tells_what_it_did() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("a.txt");
    std::fs::write(&file, "old\n").unwrap();
    let path = file.to_str().unwrap();
    let (status, _, err) = batch(&[path, "--eval", "(do-auto-save)"], "");
    assert_eq!((status, err.as_str()), (0, ""));
    let (status, _, err) = batch(
        &[path, "--eval", "(insert \"x\")", "--eval", "(do-auto-save)"],
        "",
    );
    assert_eq!(
        (status, err.as_str()),
        (0, "Auto-saving...\nAuto-saving...done\n")
    );
    assert_eq!(
        std::fs::read_to_string(dir.path().join("#a.txt#")).unwrap(),
        "xold\n"
    );
}

#[test]
fn test_yank_pop_after_a_non_yank_reads_a_kill() {
    let (status, out, _) = batch(
        &[
            "--eval",
            "(begin (kill-new \"first\") (kill-new \"second\") (with-current-buffer (generate-new-buffer \"b\") (yank-pop) (princ (buffer-string))))",
        ],
        "first\n",
    );
    assert_eq!((status, out.as_str()), (0, "first"));
    // 空のキルリング
    let (status, _, err) = batch(
        &[
            "--eval",
            "(with-current-buffer (generate-new-buffer \"b\") (yank-pop))",
        ],
        "",
    );
    assert_eq!(
        (status, err.as_str()),
        (ERROR_STATUS, "Kill ring is empty\n")
    );
}

#[test]
fn test_copy_region_as_kill_appends_after_a_kill() {
    let (_, out, _) = batch(
        &[
            "--eval",
            "(with-current-buffer (generate-new-buffer \"b\") (insert \"abc def\") (copy-region-as-kill 1 4) (set! last-command 'kill-region) (copy-region-as-kill 8 5) (princ (current-kill 0)))",
        ],
        "",
    );
    assert_eq!(out, "defabc");
}

#[test]
fn test_search_helpers_and_bookmarks() {
    let (_, out, _) = batch(
        &[
            "--eval",
            "(with-current-buffer (generate-new-buffer \"b\") (insert \"Foo, bar\\nbaz qux\") (goto-char 1) (princ (list (word-search-forward \"bar baz\") (line-number-at-pos) (search-forward-regexp \"q.x\") (word-search-backward \"foo\") (line-number-at-pos 10) (line-number-at-pos 10 #t) (word-search-regexp \"\") (word-search-regexp \"a b\" #t))))",
        ],
        "",
    );
    assert_eq!(out, "(13 2 17 1 2 2  a\\W+b)");
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("a.txt");
    std::fs::write(&file, "text\n").unwrap();
    let eval = format!(
        "(begin (set! bookmark-save-flag #f) (find-file \"{}\") (bookmark-set \"one\") (princ (if (assoc \"one\" bookmark-alist) 1 0)))",
        file.display()
    );
    let (_, out, _) = batch(&["--eval", &eval], "");
    assert_eq!(out, "1");
}

#[test]
fn test_kmacro_counter_and_end_macro_without_argument() {
    let (_, out, err) = batch(
        &[
            "--eval",
            "(with-current-buffer (generate-new-buffer \"b\") (kmacro-insert-counter 2) (kmacro-insert-counter '(4)) (kmacro-start-macro #f) (kmacro-end-macro #f) (princ (buffer-string)))",
        ],
        "",
    );
    assert_eq!(out, "00");
    assert!(err.contains("New macro counter value: 2 (2)"), "{err}");
}
