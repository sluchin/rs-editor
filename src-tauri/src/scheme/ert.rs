//! Emacs の ERT (Emacs Lisp Regression Testing) と同じ書き方の, テストの仕組み.
//!
//! - `(ert-deftest 名前 () [説明] [:expected-result :failed] [:tags '(...)] 本体...)` でテストを定義する.
//! - 本体では, `(should 式)`, `(should-not 式)`, `(should-error 式)`, `(skip-unless 式)` で確かめる.
//!   `should` が失敗すると, 式と, 引数を評価した式と, 値を, 失敗の内容に書く.
//! - `(ert-run-tests-batch [選ぶもの])` は, テストを実行して, Emacs の `-batch` と同じ形の報告を返す.
//!   `(ert-run-tests-batch-and-exit [選ぶもの])` は, 思ったとおりでない結果があれば, 報告を失敗として返す.
//!   選ぶものは, `#t` (全て), 文字列 (名前の正規表現), 記号 (その名前).
//! - `M-x ert` は, 選ぶものを聞いて実行し, 報告を `*ert*` のバッファに出す ([`PRELUDE`]).

use std::time::Instant;

use crate::scheme::builtins::fixed;
use crate::scheme::env::Env;
use crate::scheme::evaluator::{make_lambda, Machine};
use crate::scheme::parser::Parser;
use crate::scheme::system;
use crate::scheme::value::Value;

/// テストを実行する組み込みの手続きの名前.
pub const NAMES: &[&str] = &[
    "ert-run-tests-batch",
    "ert-run-tests-batch-and-exit",
    "ert-test-boundp",
    "ert-delete-test",
    "ert-delete-all-tests",
    "ert-fail",
    "ert-skip",
];

/// 評価器を作るときに評価する, Scheme で書いたコマンド (`M-x ert`).
pub const PRELUDE: &str = r#"
(define (ert--show report)
  (switch-to-buffer "*ert*")
  (delete-region (point-min) (point-max))
  (insert report)
  (goto-char (point-min))
  (set-buffer-modified-p #f)
  (message "%s" (car (regexp-match "(?m)^Ran .*$" report))))
(define-command (ert)
  (let ((selector (read-string "Run tests (default t): ")))
    (ert--show (ert-run-tests-batch (if (string=? selector "") #t selector)))))
"#;

/// `should` などが失敗したときの, 失敗の内容の始め.
const FAILED: &str = "Test failed: ";

/// `skip-unless` などで飛ばしたときの, 失敗の内容の始め.
const SKIPPED: &str = "Test skipped: ";

/// 定義したテスト.
#[derive(Clone)]
pub struct ErtTest {
    /// 名前.
    name: String,
    /// 本体 (引数の無い手続き).
    body: Value,
    /// 失敗するのが, 思ったとおりなら `true` (`:expected-result :failed`).
    expect_failure: bool,
}

/// テストの結果.
enum Outcome {
    /// 通った.
    Passed,
    /// 失敗した (Emacs の書き方の, 失敗の内容).
    Failed(String),
    /// 飛ばした (理由).
    Skipped(String),
}

/// ERT の特別な形 (`ert-deftest`, `should`, `should-not`, `should-error`, `skip-unless`) を評価します.
///
/// # Arguments
///
/// * `machine` - 評価.
/// * `op` - 形の名前.
/// * `args` - 形の名前の後の, 評価していない式.
/// * `env` - 評価する環境.
///
/// # Returns
///
/// 値. ERT の形でなければ `None`. 確かめた結果が違うときは, その内容の [`Err`].
pub(crate) fn special_form(
    machine: &mut Machine,
    op: &str,
    args: &[Value],
    env: &Env,
) -> Result<Option<Value>, String> {
    let value = match op {
        "ert-deftest" => deftest(machine, args, env)?,
        "should" | "should-not" => {
            let [form] = args else {
                return Err(format!("{} expects 1 argument", op));
            };
            let (value, shown) = eval_shown(machine, form, env)?;
            if value.is_true() != (op == "should") {
                return Err(format!(
                    "{}(({} {}) :form {} :value {})",
                    FAILED, op, form, shown, value
                ));
            }
            if op == "should" {
                value
            } else {
                Value::Nil
            }
        }
        "should-error" => match args {
            // :type は, 失敗に種類が無いので, 読むだけ
            [form, rest @ ..] if rest.is_empty() || matches!(rest, [Value::Symbol(k), _] if k == ":type") => {
                match machine.eval(form, env) {
                    Ok(value) => {
                        return Err(format!(
                            "{}((should-error {}) :form {} :value {} :fail-reason \"did not signal an error\")",
                            FAILED, form, form, value
                        ))
                    }
                    Err(e) if e.starts_with(FAILED) || e.starts_with(SKIPPED) => return Err(e),
                    Err(e) => Value::List([Value::Symbol("error".to_string()), Value::String(e)].into()),
                }
            }
            _ => return Err("should-error expects a form and an optional :type".to_string()),
        },
        "skip-unless" => {
            let [form] = args else {
                return Err("skip-unless expects 1 argument".to_string());
            };
            if !machine.eval(form, env)?.is_true() {
                return Err(format!("{}(skip-unless {})", SKIPPED, form));
            }
            Value::Nil
        }
        _ => return Ok(None),
    };
    Ok(Some(value))
}

/// `(ert-deftest 名前 () [説明] [キーワード 値]... 本体...)` を評価して, テストを覚えます.
///
/// # Arguments
///
/// * `machine` - 評価.
/// * `args` - `ert-deftest` の後の式.
/// * `env` - 定義する環境.
///
/// # Returns
///
/// テストの名前 (記号). 書き方が誤っていれば [`Err`].
fn deftest(machine: &mut Machine, args: &[Value], env: &Env) -> Result<Value, String> {
    let (name, mut rest) = match args {
        [Value::Symbol(name), Value::List(params), rest @ ..] if params.is_empty() => {
            (name.clone(), rest)
        }
        _ => return Err("ert-deftest expects a name and an empty argument list".to_string()),
    };
    // 説明 (後に本体があるときだけ)
    if let [Value::String(_), more @ ..] = rest {
        if !more.is_empty() {
            rest = more;
        }
    }
    let mut expect_failure = false;
    while let [Value::Symbol(keyword), value, more @ ..] = rest {
        match keyword.as_str() {
            ":expected-result" => {
                expect_failure = machine.eval(value, env)? == Value::Symbol(":failed".to_string());
            }
            ":tags" => {
                machine.eval(value, env)?;
            }
            _ if keyword.starts_with(':') => {
                return Err(format!("ert-deftest: unknown keyword {}", keyword))
            }
            _ => break,
        }
        rest = more;
    }
    let body = make_lambda(Some(&name), &Value::List(Vec::new().into()), rest, env)?;
    let mut tests = machine.evaluator.tests();
    tests.retain(|t| t.name != name);
    tests.push(ErtTest {
        name: name.clone(),
        body,
        expect_failure,
    });
    Ok(Value::Symbol(name))
}

/// 式を評価します. 手続きの呼び出しなら, 引数を評価した式も返します (`should` の失敗に書く).
///
/// # Arguments
///
/// * `machine` - 評価.
/// * `form` - 式.
/// * `env` - 評価する環境.
///
/// # Returns
///
/// 値と, 引数を評価した式 (手続きの呼び出しでなければ, 元の式).
fn eval_shown(machine: &mut Machine, form: &Value, env: &Env) -> Result<(Value, Value), String> {
    if let Value::List(items) = form {
        if let Some((Value::Symbol(head), args)) = items.split_first() {
            if let Some(procedure @ (Value::Lambda(_) | Value::Function(_))) = env.get(head) {
                let args = args
                    .iter()
                    .map(|arg| machine.eval(arg, env))
                    .collect::<Result<Vec<Value>, String>>()?;
                let shown = Value::List(
                    std::iter::once(Value::Symbol(head.clone()))
                        .chain(args.iter().cloned())
                        .collect(),
                );
                return Ok((machine.apply(procedure, args)?, shown));
            }
        }
    }
    Ok((machine.eval(form, env)?, form.clone()))
}

/// ERT の組み込みの手続きを呼びます.
///
/// # Arguments
///
/// * `machine` - 評価.
/// * `name` - 手続きの名前 ([`NAMES`] のどれか).
/// * `args` - 引数.
///
/// # Returns
///
/// 結果, または失敗の内容.
pub(crate) fn call(machine: &mut Machine, name: &str, args: Vec<Value>) -> Result<Value, String> {
    match name {
        "ert-run-tests-batch" | "ert-run-tests-batch-and-exit" => {
            let selector = match args.as_slice() {
                [] => Value::Boolean(true),
                [selector] => selector.clone(),
                _ => return Err(format!("{} expects 0 or 1 arguments", name)),
            };
            let (report, unexpected) = run_tests(machine, &selector)?;
            if unexpected > 0 && name == "ert-run-tests-batch-and-exit" {
                Err(report)
            } else {
                Ok(Value::String(report))
            }
        }
        "ert-test-boundp" | "ert-delete-test" => {
            let [test] = fixed::<1>(name, args)?;
            let Value::Symbol(test) = test else {
                return Err(format!("{} expects a test name", name));
            };
            let mut tests = machine.evaluator.tests();
            let found = tests.iter().any(|t| t.name == test);
            if name == "ert-delete-test" {
                tests.retain(|t| t.name != test);
            }
            Ok(Value::Boolean(found))
        }
        "ert-delete-all-tests" => {
            fixed::<0>(name, args)?;
            machine.evaluator.tests().clear();
            Ok(Value::Nil)
        }
        _ => {
            // ert-fail と ert-skip
            let [reason] = fixed::<1>(name, args)?;
            let reason = match &reason {
                Value::String(text) => text.clone(),
                other => other.to_string(),
            };
            Err(format!(
                "{}{}",
                if name == "ert-fail" { FAILED } else { SKIPPED },
                reason
            ))
        }
    }
}

/// 選んだテストを実行して, 報告 (Emacs の `ert-run-tests-batch` と同じ形) を作ります.
///
/// # Arguments
///
/// * `machine` - 評価.
/// * `selector` - 選ぶもの (`#t`, 名前の正規表現, 名前の記号).
///
/// # Returns
///
/// 報告と, 思ったとおりでなかった結果の数. 選ぶものが誤っていれば [`Err`].
fn run_tests(machine: &mut Machine, selector: &Value) -> Result<(String, usize), String> {
    let all = machine.evaluator.tests().clone();
    let selected: Vec<ErtTest> = match selector {
        Value::Boolean(true) => all,
        Value::String(pattern) => {
            let regex = system::regexp(pattern)?;
            all.into_iter()
                .filter(|t| regex.is_match(&t.name))
                .collect()
        }
        Value::Symbol(name) => all.into_iter().filter(|t| &t.name == name).collect(),
        other => return Err(format!("Invalid test selector: {}", other)),
    };
    let total = selected.len();
    let started = Instant::now();
    let mut lines = vec![format!(
        "Running {} {} (selector `{}')",
        total,
        plural(total, "test"),
        selector_name(selector)
    )];
    let (mut expected, mut skipped, mut unexpected) = (0, 0, Vec::new());
    for (i, test) in selected.iter().enumerate() {
        let start = Instant::now();
        let outcome = match machine.apply(test.body.clone(), vec![]) {
            Ok(_) => Outcome::Passed,
            Err(e) if e.starts_with(SKIPPED) => Outcome::Skipped(e[SKIPPED.len()..].to_string()),
            Err(e) => Outcome::Failed(match e.strip_prefix(FAILED) {
                Some(details) => format!("(ert-test-failed\n     {})", details),
                None => format!("(error {:?})", e),
            }),
        };
        let seconds = start.elapsed().as_secs_f64();
        let status = match (&outcome, test.expect_failure) {
            (Outcome::Passed, false) => "passed",
            (Outcome::Failed(_), true) => "failed",
            (Outcome::Passed, true) => "PASSED",
            (Outcome::Failed(_), false) => "FAILED",
            (Outcome::Skipped(_), _) => "skipped",
        };
        match &outcome {
            Outcome::Failed(condition) => {
                lines.push(format!("Test {} condition:\n    {}", test.name, condition))
            }
            Outcome::Skipped(reason) => {
                lines.push(format!("Test {} skipped: {}", test.name, reason))
            }
            Outcome::Passed => {}
        }
        lines.push(format!(
            "{:>9}  {}/{}  {} ({:.6} sec)",
            status,
            i + 1,
            total,
            test.name,
            seconds
        ));
        match status {
            "passed" | "failed" => expected += 1,
            "skipped" => skipped += 1,
            _ => unexpected.push(format!("{:>9}  {}", status, test.name)),
        }
    }
    lines.push(String::new());
    lines.push(format!(
        "Ran {} {}, {} {} as expected, {} unexpected{} ({:.6} sec)",
        total,
        plural(total, "test"),
        expected,
        plural(expected, "result"),
        unexpected.len(),
        if skipped > 0 {
            format!(", {} skipped", skipped)
        } else {
            String::new()
        },
        started.elapsed().as_secs_f64()
    ));
    if !unexpected.is_empty() {
        lines.push(String::new());
        lines.push(format!("{} unexpected results:", unexpected.len()));
        lines.extend(unexpected.iter().cloned());
    }
    Ok((lines.join("\n") + "\n", unexpected.len()))
}

/// 選ぶものを, 報告に書く形にします.
///
/// # Arguments
///
/// * `selector` - 選ぶもの.
///
/// # Returns
///
/// `t`, `"regexp"`, 名前.
fn selector_name(selector: &Value) -> String {
    match selector {
        Value::Boolean(true) => "t".to_string(),
        other => other.to_string(),
    }
}

/// 数に合わせて, 複数形にします.
///
/// # Arguments
///
/// * `count` - 数.
/// * `word` - 単数形.
///
/// # Returns
///
/// 1 なら単数形, ほかは `s` を付けた形.
fn plural(count: usize, word: &str) -> String {
    if count == 1 {
        word.to_string()
    } else {
        format!("{}s", word)
    }
}

/// [`PRELUDE`] を読みます (評価器を作るときに使う).
///
/// # Returns
///
/// 式.
pub(crate) fn prelude() -> Vec<Value> {
    Parser::new(PRELUDE)
        .parse()
        .expect("the ERT prelude is valid")
}
