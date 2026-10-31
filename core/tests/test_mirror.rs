//! 画面のバッファを核に写す仕組み (`mirror`) のテスト. 偽の画面を相手に、写す・返す・やり直す流れを確かめる.

use std::sync::Mutex;

use rseditor_core::scheme::evaluator::Evaluator;
use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::mirror::with_mirror;
use rseditor_core::scheme::value::Value;

/// 偽の画面のバッファ.
#[derive(Clone)]
struct Buf {
    name: String,
    text: String,
    /// カーソル (1 から).
    point: usize,
    /// マーク (1 から).
    mark: Option<usize>,
    file: Option<String>,
}

/// 偽の画面: バッファを持ち、呼ばれた操作を記録する.
struct Frontend {
    buffers: Mutex<Vec<Buf>>,
    current: Mutex<usize>,
    log: Mutex<Vec<String>>,
    /// 返事を壊すか (`editor-snapshot` が変な値を返す).
    broken: bool,
    /// `editor-apply` を受けないか.
    refuse_apply: bool,
}

impl Frontend {
    fn new(text: &str) -> Self {
        let buf = |name: &str, text: &str| Buf {
            name: name.to_string(),
            text: text.to_string(),
            point: 1,
            mark: None,
            file: None,
        };
        Frontend {
            buffers: Mutex::new(vec![buf("a", text), buf("b", "other")]),
            current: Mutex::new(0),
            log: Mutex::new(Vec::new()),
            broken: false,
            refuse_apply: false,
        }
    }

    fn log(&self) -> Vec<String> {
        self.log.lock().unwrap().clone()
    }

    fn with_current<T>(&self, f: impl FnOnce(&mut Buf) -> T) -> T {
        let index = *self.current.lock().unwrap();
        f(&mut self.buffers.lock().unwrap()[index])
    }
}

impl Host for Frontend {
    fn call(&self, op: &str, args: Vec<Value>) -> Result<Value, String> {
        self.log.lock().unwrap().push(op.to_string());
        let number = |i: usize| match &args[i] {
            Value::Number(n) => *n as usize,
            other => panic!("not a number: {other}"),
        };
        match op {
            "editor-snapshot" if self.broken => Ok(Value::String("oops".into())),
            "editor-snapshot" => Ok(self.with_current(|b| {
                Value::List(
                    vec![
                        Value::String(b.name.clone()),
                        Value::String(b.text.clone()),
                        Value::Number(b.point as f64),
                        b.mark
                            .map_or(Value::Boolean(false), |m| Value::Number(m as f64)),
                        b.file.clone().map_or(Value::Boolean(false), Value::String),
                    ]
                    .into_iter()
                    .collect(),
                )
            })),
            "editor-apply" if self.refuse_apply => Err("cannot apply".into()),
            "editor-apply" => {
                let Value::String(text) = &args[0] else {
                    panic!("not a string")
                };
                let (point, mark) = (
                    number(1),
                    matches!(args[2], Value::Number(_)).then(|| number(2)),
                );
                self.with_current(|b| {
                    b.text = text.clone();
                    b.point = point;
                    b.mark = mark;
                });
                Ok(Value::Nil)
            }
            "point" => Ok(Value::Number(self.with_current(|b| b.point) as f64)),
            "goto-char" => {
                let pos = number(0);
                self.with_current(|b| b.point = pos);
                Ok(Value::Nil)
            }
            "pop-to-buffer-same-window" => {
                *self.current.lock().unwrap() = 1;
                Ok(Value::Nil)
            }
            "message" => Ok(Value::Nil),
            "buffer-name" | "current-buffer" => {
                Ok(Value::String(self.with_current(|b| b.name.clone())))
            }
            "set-buffer" => Ok(args[0].clone()),
            other => Err(format!("frontend: {other}")),
        }
    }
}

/// 評価して、結果と、最後に返せなかった失敗を返す.
fn run(frontend: &Frontend, code: &str) -> (Result<String, String>, Option<String>) {
    let ev = Evaluator::new();
    with_mirror(frontend, |host| ev.eval_with(code, host))
}

#[test]
fn test_native_operations_load_once_and_flush_at_the_end() {
    let frontend = Frontend::new("abc abc");
    let (result, flush) = run(
        &frontend,
        "(re-search-forward \"b\") (re-search-forward \"b\") (list (match-beginning 0) (mark))",
    );
    assert_eq!(result.unwrap().lines().last(), Some("(6 #f)"));
    assert_eq!(flush, None);
    // 画面には 1 回だけ頼み (写す), 終わりに変更を返す
    assert_eq!(frontend.log(), vec!["editor-snapshot", "editor-apply"]);
    assert_eq!(frontend.with_current(|b| b.point), 7);
    // 変更が無ければ、返さない
    let frontend = Frontend::new("abc");
    let (result, _) = run(&frontend, "(list (char-after) (looking-at \"ab\"))");
    assert_eq!(result.unwrap().lines().last(), Some("(\"a\" #t)"));
    assert_eq!(frontend.log(), vec!["editor-snapshot"]);
}

#[test]
fn test_changes_are_returned_before_asking_the_frontend() {
    let frontend = Frontend::new("abc");
    let (result, _) = run(
        &frontend,
        "(forward-char 2) (message \"hi\") (list (char-after) (point))",
    );
    // message の前に返す. message は、写したバッファを変えないので、char-after は写し直さない
    assert_eq!(result.unwrap().lines().last(), Some("(\"c\" 3)"));
    assert_eq!(
        frontend.log(),
        vec!["editor-snapshot", "editor-apply", "message", "point"]
    );
}

#[test]
fn test_frontend_edits_invalidate_the_mirror() {
    let frontend = Frontend::new("abcdef");
    let (result, _) = run(
        &frontend,
        "(forward-char 2) (goto-char 5) (list (char-after) (point))",
    );
    // goto-char は画面が受け持ち、写したバッファが古くなるので、次の char-after で写し直す
    assert_eq!(result.unwrap().lines().last(), Some("(\"e\" 5)"));
    assert_eq!(
        frontend.log(),
        vec![
            "editor-snapshot",
            "editor-apply",
            "goto-char",
            "editor-snapshot",
            "point"
        ]
    );
}

#[test]
fn test_switching_buffers_loads_the_new_current_buffer() {
    let frontend = Frontend::new("abc");
    let (result, _) = run(
        &frontend,
        "(forward-char 1) (pop-to-buffer-same-window \"b\") (list (char-after) (buffer-name))",
    );
    assert_eq!(result.unwrap().lines().last(), Some("(\"o\" \"b\")"));
    // a には、動かしたカーソルを返し、b を写す
    assert_eq!(frontend.buffers.lock().unwrap()[0].point, 2);
}

#[test]
fn test_text_and_mark_are_returned() {
    let frontend = Frontend::new("hello");
    let (result, flush) = run(
        &frontend,
        "(set-mark 2) (goto-char 1) (erase-buffer) (list (mark) (buffer-size))",
    );
    // goto-char は画面に頼むので、その前に返される. erase-buffer の後は、核が持つ
    assert!(result.is_ok());
    assert_eq!(flush, None);
    frontend.with_current(|b| {
        assert_eq!(b.text, "");
    });
    // マークと方言を写す
    let frontend = Frontend::new("(a b)");
    frontend.with_current(|b| {
        b.mark = Some(3);
        b.file = Some("/tmp/a.el".to_string());
    });
    let (result, _) = run(
        &frontend,
        "(list (mark) (syntax-dialect) (scan-lists 1 1 0))",
    );
    assert_eq!(result.unwrap().lines().last(), Some("(3 elisp 6)"));
    assert_eq!(frontend.with_current(|b| b.mark), Some(3));
    let frontend = Frontend::new("x");
    frontend.with_current(|b| b.mark = Some(1));
    let (result, _) = run(&frontend, "(set-mark #f)");
    assert!(result.is_ok());
    assert_eq!(frontend.with_current(|b| b.mark), Some(1));
}

#[test]
fn test_failures() {
    // 写す返事が変なら、失敗
    let mut frontend = Frontend::new("abc");
    frontend.broken = true;
    let (result, _) = run(&frontend, "(char-after)");
    assert_eq!(result.unwrap_err(), "editor-snapshot: unexpected reply");
    // 画面が、写すことを受けないなら、失敗
    let ev = Evaluator::new();
    struct NoSnapshot;
    impl Host for NoSnapshot {
        fn call(&self, op: &str, _args: Vec<Value>) -> Result<Value, String> {
            match op {
                "editor-snapshot" => {
                    Ok(Value::List(vec![Value::Number(1.0)].into_iter().collect()))
                }
                other => Err(other.to_string()),
            }
        }
    }
    let (result, _) = with_mirror(&NoSnapshot, |host| ev.eval_with("(char-after)", host));
    assert_eq!(result.unwrap_err(), "editor-snapshot: unexpected reply");
    // 変更を返せなければ、評価の後に知らせる. 画面に頼む前に返せなければ、その操作は失敗
    let mut frontend = Frontend::new("abc");
    frontend.refuse_apply = true;
    let (result, flush) = run(&frontend, "(forward-char 1)");
    assert!(result.is_ok());
    assert_eq!(flush.unwrap(), "cannot apply");
    let (result, _) = run(&frontend, "(forward-char 1) (message \"x\")");
    assert_eq!(result.unwrap_err(), "cannot apply");
}

#[test]
fn test_save_excursion_falls_back_to_the_point_number() {
    // 画面にはマーカーが無いので, save-excursion は, 位置の数で覚えて戻す
    let frontend = Frontend::new("abcdef");
    let (result, _) = run(&frontend, "(save-excursion (forward-char 3) (char-after))");
    assert_eq!(result.unwrap().lines().last(), Some("\"d\""));
    assert_eq!(frontend.with_current(|b| b.point), 1);
}
