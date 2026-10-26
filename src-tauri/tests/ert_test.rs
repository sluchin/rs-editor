//! ERT (`ert-deftest`, `should` など) のテスト. 定義, 確かめる形, 実行の報告, `M-x ert`, 一時ファイルを確かめる.

use std::fs;
use std::sync::Mutex;

use rseditor::scheme::evaluator::Evaluator;
use rseditor::scheme::host::Host;
use rseditor::scheme::value::Value;

/// 報告から, 時間 (`(0.000012 sec)`) を除く (毎回変わるため).
///
/// # Arguments
///
/// * `report` - 報告.
///
/// # Returns
///
/// 時間を `(T sec)` にした報告.
fn untimed(report: &str) -> String {
    regex::Regex::new(r"\([0-9.]+ sec\)")
        .unwrap()
        .replace_all(report, "(T sec)")
        .into_owned()
}

/// 評価して, 文字列の値 (報告) を返す (ファイルに書いて読む).
///
/// # Arguments
///
/// * `ev` - 評価器.
/// * `code` - コード (値が文字列).
///
/// # Returns
///
/// 文字列 (時間は除く).
fn report(ev: &Evaluator, code: &str) -> String {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("report.txt");
    ev.eval(&format!("(write-file-string {:?} {})", path, code))
        .unwrap();
    untimed(&fs::read_to_string(path).unwrap())
}

#[test]
fn test_passing_and_failing_tests() {
    let ev = Evaluator::new();
    ev.eval(
        r#"
(ert-deftest ok-test ()
  "説明."
  :tags '(quick)
  (should (= (+ 1 1) 2))
  (should-not (string=? "a" "b"))
  (should (equal (should-error (car 1)) '(error "car expects a non-empty list")))
  (should-error (car 1) :type 'wrong-type-argument))
(ert-deftest fail-test () (should (equal (+ 1 1) 3)))
(ert-deftest fail-not-test () (should-not (+ 1 1)))
(ert-deftest no-error-test () (should-error (+ 1 2)))
(ert-deftest error-test () (car 1))
(ert-deftest special-test () (should (and #t #f)))
(ert-deftest expected-failure-test () :expected-result :failed (should #f))
(ert-deftest unexpected-pass-test () :expected-result :failed (should #t))
(ert-deftest skip-test () (skip-unless (= 1 2)) (should #f))
(ert-deftest explicit-test () (ert-fail "わざと"))
(ert-deftest explicit-skip-test () (ert-skip '(no editor)))
(ert-deftest nested-test () (should-error (should #f)))
(ert-deftest doc-only-test () "説明だけ")
"#,
    )
    .unwrap();
    assert_eq!(
        report(&ev, "(ert-run-tests-batch)"),
        r#"Running 13 tests (selector `t')
   passed  1/13  ok-test (T sec)
Test fail-test condition:
    (ert-test-failed
     ((should (equal (+ 1 1) 3)) :form (equal 2 3) :value #f))
   FAILED  2/13  fail-test (T sec)
Test fail-not-test condition:
    (ert-test-failed
     ((should-not (+ 1 1)) :form (+ 1 1) :value 2))
   FAILED  3/13  fail-not-test (T sec)
Test no-error-test condition:
    (ert-test-failed
     ((should-error (+ 1 2)) :form (+ 1 2) :value 3 :fail-reason "did not signal an error"))
   FAILED  4/13  no-error-test (T sec)
Test error-test condition:
    (error "car expects a non-empty list")
   FAILED  5/13  error-test (T sec)
Test special-test condition:
    (ert-test-failed
     ((should (and #t #f)) :form (and #t #f) :value #f))
   FAILED  6/13  special-test (T sec)
Test expected-failure-test condition:
    (ert-test-failed
     ((should #f) :form #f :value #f))
   failed  7/13  expected-failure-test (T sec)
   PASSED  8/13  unexpected-pass-test (T sec)
Test skip-test skipped: (skip-unless (= 1 2))
  skipped  9/13  skip-test (T sec)
Test explicit-test condition:
    (ert-test-failed
     わざと)
   FAILED  10/13  explicit-test (T sec)
Test explicit-skip-test skipped: (no editor)
  skipped  11/13  explicit-skip-test (T sec)
Test nested-test condition:
    (ert-test-failed
     ((should #f) :form #f :value #f))
   FAILED  12/13  nested-test (T sec)
   passed  13/13  doc-only-test (T sec)

Ran 13 tests, 3 results as expected, 8 unexpected, 2 skipped (T sec)

8 unexpected results:
   FAILED  fail-test
   FAILED  fail-not-test
   FAILED  no-error-test
   FAILED  error-test
   FAILED  special-test
   PASSED  unexpected-pass-test
   FAILED  explicit-test
   FAILED  nested-test
"#
    );
    // 選ぶ: 名前の正規表現, 名前
    assert_eq!(
        report(&ev, "(ert-run-tests-batch \"^ok\")"),
        "Running 1 test (selector `\"^ok\"')\n   passed  1/1  ok-test (T sec)\n\nRan 1 test, 1 result as expected, 0 unexpected (T sec)\n"
    );
    assert!(report(&ev, "(ert-run-tests-batch 'fail-test)")
        .starts_with("Running 1 test (selector `fail-test')"));
    assert!(report(&ev, "(ert-run-tests-batch \"none\")").starts_with("Running 0 tests"));
    // and-exit: 思ったとおりでない結果があれば, 失敗にする
    assert!(ev
        .eval("(ert-run-tests-batch-and-exit 'fail-test)")
        .unwrap_err()
        .contains("1 unexpected results:"));
    assert!(ev
        .eval("(ert-run-tests-batch-and-exit 'ok-test)")
        .unwrap()
        .contains("Ran 1 test, 1 result as expected"));
    // 同じ名前は, 置き換える. 消す
    ev.eval("(ert-deftest fail-test () (should #t))").unwrap();
    assert!(ev.eval("(ert-run-tests-batch-and-exit 'fail-test)").is_ok());
    assert_eq!(
        ev.eval("(ert-test-boundp 'ok-test) (ert-delete-test 'ok-test) (ert-test-boundp 'ok-test) (ert-delete-test 'ok-test)")
            .unwrap(),
        "#t\n#t\n#f\n#f"
    );
    ev.eval("(ert-delete-all-tests)").unwrap();
    assert!(report(&ev, "(ert-run-tests-batch)").starts_with("Running 0 tests"));
    // テストの外の should
    assert_eq!(ev.eval("(should 1) (should-not #f)").unwrap(), "1\nnil");
    assert!(ev
        .eval("(should #f)")
        .unwrap_err()
        .starts_with("Test failed: "));
}

#[test]
fn test_errors() {
    let ev = Evaluator::new();
    for (code, error) in [
        (
            "(ert-deftest t1 (x) 1)",
            "ert-deftest expects a name and an empty argument list",
        ),
        (
            "(ert-deftest)",
            "ert-deftest expects a name and an empty argument list",
        ),
        (
            "(ert-deftest t1 () :bad 1 2)",
            "ert-deftest: unknown keyword :bad",
        ),
        ("(should)", "should expects 1 argument"),
        ("(should-not 1 2)", "should-not expects 1 argument"),
        (
            "(should-error)",
            "should-error expects a form and an optional :type",
        ),
        (
            "(should-error 1 2)",
            "should-error expects a form and an optional :type",
        ),
        ("(skip-unless)", "skip-unless expects 1 argument"),
        ("(ert-run-tests-batch 1)", "Invalid test selector: 1"),
        (
            "(ert-run-tests-batch \"(\")",
            "Invalid regexp: regex parse error:\n    (\n    ^\nerror: unclosed group",
        ),
        (
            "(ert-run-tests-batch 1 2)",
            "ert-run-tests-batch expects 0 or 1 arguments",
        ),
        (
            "(ert-test-boundp \"x\")",
            "ert-test-boundp expects a test name",
        ),
        (
            "(ert-delete-all-tests 1)",
            "ert-delete-all-tests expects 0 arguments, given 1",
        ),
        ("(ert-fail)", "ert-fail expects 1 argument, given 0"),
        ("(unwind-protect)", "unwind-protect expects a body form"),
    ] {
        assert_eq!(ev.eval(code).unwrap_err(), error, "{}", code);
    }
    // 引数の評価が失敗する should, :tags の評価が失敗する
    assert_eq!(
        ev.eval("(should (= 1 (car 1)))").unwrap_err(),
        "car expects a non-empty list"
    );
    assert_eq!(
        ev.eval("(ert-deftest t2 () :tags (car 1) 1)").unwrap_err(),
        "car expects a non-empty list"
    );
    assert_eq!(
        ev.eval("(ert-deftest t3 () :expected-result (car 1))")
            .unwrap_err(),
        "car expects a non-empty list"
    );
    // require しなくても使える
    assert_eq!(
        ev.eval("(featurep 'ert) (require 'ert)").unwrap(),
        "#t\nert"
    );
}

#[test]
fn test_unwind_protect() {
    let ev = Evaluator::new();
    assert_eq!(
        ev.eval("(define log '()) (unwind-protect (+ 1 2) (set! log (cons 'a log)) (set! log (cons 'b log))) log")
            .unwrap(),
        "log\n3\n(b a)"
    );
    // 本体が失敗しても, 後始末をして, 失敗を返す
    assert_eq!(
        ev.eval("(unwind-protect (car 1) (set! log '(done)))")
            .unwrap_err(),
        "car expects a non-empty list"
    );
    assert_eq!(ev.eval("log").unwrap(), "(done)");
    // 後始末の失敗
    assert_eq!(
        ev.eval("(unwind-protect 1 (car 1))").unwrap_err(),
        "car expects a non-empty list"
    );
}

#[test]
fn test_temp_files() {
    let ev = Evaluator::new();
    let dir = ev
        .eval("(make-temp-file \"ert-dir-\" #t)")
        .unwrap()
        .trim_matches('"')
        .to_string();
    assert!(fs::metadata(&dir).unwrap().is_dir());
    assert!(dir.contains("ert-dir-"));
    let file = ev
        .eval("(make-temp-file \"ert-file-\")")
        .unwrap()
        .trim_matches('"')
        .to_string();
    assert!(fs::metadata(&file).unwrap().is_file());
    ev.eval(&format!("(delete-file {:?})", file)).unwrap();
    assert!(!fs::exists(&file).unwrap());
    // 中身のあるディレクトリは, 中身ごと消すときだけ消す
    fs::write(format!("{}/a.txt", dir), "a").unwrap();
    assert!(ev
        .eval(&format!("(delete-directory {:?})", dir))
        .unwrap_err()
        .starts_with(&format!("{}: ", dir)));
    ev.eval(&format!("(delete-directory {:?} #t)", dir))
        .unwrap();
    assert!(!fs::exists(&dir).unwrap());
    let empty = ev.eval("(make-temp-file \"ert-empty-\" #t)").unwrap();
    ev.eval(&format!("(delete-directory {})", empty)).unwrap();
    for (code, error) in [
        (
            "(make-temp-file)",
            "make-temp-file expects 1 or 2 arguments",
        ),
        ("(make-temp-file 1)", "make-temp-file expects strings"),
        (
            "(delete-directory)",
            "delete-directory expects 1 or 2 arguments",
        ),
        ("(delete-directory 1)", "delete-directory expects strings"),
    ] {
        assert_eq!(ev.eval(code).unwrap_err(), error, "{}", code);
    }
    assert!(ev
        .eval("(make-temp-file \"no/such/dir/x\")")
        .unwrap_err()
        .starts_with("make-temp-file: "));
    assert!(ev.eval(&format!("(delete-file {:?})", file)).is_err());
}

/// 偽のエディタ (呼ばれた操作を覚え, read-string には決めた答えを返す).
struct Recorder {
    /// 呼ばれた操作.
    calls: Mutex<Vec<String>>,
    /// read-string の答え.
    answer: String,
}

impl Host for Recorder {
    fn call(&self, op: &str, args: Vec<Value>) -> Result<Value, String> {
        let shown: Vec<String> = args.iter().map(|a| a.to_string()).collect();
        self.calls
            .lock()
            .unwrap()
            .push(format!("{} {}", op, shown.join(" ")).trim().to_string());
        Ok(match op {
            "read-string" => Value::String(self.answer.clone()),
            "point-min" => Value::Number(1.0),
            "point-max" => Value::Number(1.0),
            _ => Value::Nil,
        })
    }
}

#[test]
fn test_ert_command() {
    let ev = Evaluator::new();
    // M-x ert は, 始めからあるコマンド
    assert!(ev.commands().contains(&"ert".to_string()));
    ev.eval("(ert-deftest a-test () (should #t)) (ert-deftest b-test () (should #t))")
        .unwrap();
    for (answer, selector, count) in [("", "t", "2 tests"), ("^a", "\"^a\"", "1 test")] {
        let editor = Recorder {
            calls: Mutex::new(Vec::new()),
            answer: answer.to_string(),
        };
        ev.run_command("ert", &editor).unwrap();
        let calls: Vec<String> = editor
            .calls
            .lock()
            .unwrap()
            .iter()
            .map(|c| untimed(c))
            .collect();
        assert_eq!(calls[0], "read-string \"Run tests (default t): \" \"\"");
        assert_eq!(calls[1], "pop-to-buffer-same-window \"*ert*\"");
        assert!(
            calls[5].starts_with(&format!(
                "insert \"Running {} (selector `{}')",
                count,
                selector.replace('"', "\\\"")
            )),
            "{}",
            calls[5]
        );
        assert_eq!(
            calls.last().unwrap(),
            &format!(
                "message \"Ran {}, {} as expected, 0 unexpected (T sec)\"",
                count,
                count.replace("test", "result")
            )
        );
    }
}

#[test]
fn test_body_starting_with_a_symbol() {
    let ev = Evaluator::new();
    ev.eval(
        r#"
(define x 1)
(ert-deftest symbol-body-test () x (skip-unless (= x 1)) (should (= x 1)))
"#,
    )
    .unwrap();
    assert_eq!(
        report(&ev, "(ert-run-tests-batch 'symbol-body-test)"),
        "Running 1 test (selector `symbol-body-test')\n   passed  1/1  symbol-body-test (T sec)\n\nRan 1 test, 1 result as expected, 0 unexpected (T sec)\n"
    );
}
