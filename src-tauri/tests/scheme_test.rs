//! Scheme の評価器 (`scheme`) のテスト. 言語の核, 組み込みの手続き, エディタの操作, エラーの経路を確かめる.

use std::sync::Mutex;

use rseditor::scheme::evaluator::{Evaluator, KeyBinding, MAX_DEPTH};
use rseditor::scheme::host::Host;
use rseditor::scheme::parser::Parser;
use rseditor::scheme::value::Value;

/// 評価して, 結果を返す.
///
/// # Arguments
///
/// * `code` - コード.
///
/// # Returns
///
/// 結果 (失敗したら panic する).
fn eval(code: &str) -> String {
    Evaluator::new().eval(code).unwrap()
}

/// 評価して, 失敗の内容を返す.
///
/// # Arguments
///
/// * `code` - コード.
///
/// # Returns
///
/// 失敗の内容 (成功したら panic する).
fn eval_err(code: &str) -> String {
    Evaluator::new().eval(code).unwrap_err()
}

/// 偽のエディタ. 本文とカーソル (1 から数える) を持ち, 呼ばれた操作を覚える.
#[derive(Default)]
struct FakeEditor {
    /// 本文.
    text: Mutex<String>,
    /// カーソルの位置 (1 から数える).
    point: Mutex<usize>,
    /// 呼ばれた操作 (`insert "a"` のように, 名前と引数).
    calls: Mutex<Vec<String>>,
}

impl FakeEditor {
    /// カーソルが先頭にある, 偽のエディタを作る.
    ///
    /// # Arguments
    ///
    /// * `text` - 本文.
    ///
    /// # Returns
    ///
    /// 偽のエディタ.
    fn new(text: &str) -> Self {
        FakeEditor {
            text: Mutex::new(text.to_string()),
            point: Mutex::new(1),
            calls: Mutex::new(Vec::new()),
        }
    }

    /// 本文を返す.
    ///
    /// # Returns
    ///
    /// 本文.
    fn text(&self) -> String {
        self.text.lock().unwrap().clone()
    }

    /// 呼ばれた操作を返す.
    ///
    /// # Returns
    ///
    /// 呼ばれた操作 (呼ばれた順).
    fn calls(&self) -> Vec<String> {
        self.calls.lock().unwrap().clone()
    }
}

/// 数を, 位置 (0 から数える) にする.
///
/// # Arguments
///
/// * `v` - 位置 (1 から数える数).
///
/// # Returns
///
/// 位置 (0 から数える. 数でなければ panic する).
fn offset(v: &Value) -> usize {
    match v {
        Value::Number(n) => *n as usize - 1,
        _ => panic!("not a number"),
    }
}

impl Host for FakeEditor {
    fn call(&self, op: &str, args: Vec<Value>) -> Result<Value, String> {
        let shown: Vec<String> = args.iter().map(|a| a.to_string()).collect();
        self.calls
            .lock()
            .unwrap()
            .push(format!("{} {}", op, shown.join(" ")).trim().to_string());
        let mut text = self.text.lock().unwrap();
        let mut point = self.point.lock().unwrap();
        Ok(match op {
            "point" => Value::Number(*point as f64),
            "point-min" => Value::Number(1.0),
            "point-max" => Value::Number(text.len() as f64 + 1.0),
            "goto-char" => {
                *point = offset(&args[0]) + 1;
                args[0].clone()
            }
            "insert" => {
                for arg in &args {
                    let Value::String(s) = arg else { panic!() };
                    text.insert_str(*point - 1, s);
                    *point += s.len();
                }
                Value::Nil
            }
            "delete-region" => {
                text.replace_range(offset(&args[0])..offset(&args[1]), "");
                Value::Nil
            }
            "buffer-string" => Value::String(text.clone()),
            "buffer-substring" => {
                Value::String(text[offset(&args[0])..offset(&args[1])].to_string())
            }
            "buffer-name" => Value::String("*scratch*".to_string()),
            "message" => Value::Nil,
            "command" => match &args[0] {
                Value::String(name) if name == "forward-char" => {
                    *point += 1;
                    Value::Boolean(true)
                }
                _ => return Err("Undefined command".to_string()),
            },
            _ => panic!("unknown op {}", op),
        })
    }
}

#[test]
fn test_define_lambda_and_closures() {
    let ev = Evaluator::new();
    assert_eq!(ev.eval("(define (square x) (* x x))").unwrap(), "square");
    assert_eq!(ev.eval("(square 7)").unwrap(), "49");
    assert_eq!(ev.eval("square").unwrap(), "#<procedure:square>");
    assert_eq!(
        ev.eval("(define add (lambda (a b) (+ a b)))").unwrap(),
        "add"
    );
    assert_eq!(ev.eval("add").unwrap(), "#<procedure:add>");
    assert_eq!(ev.eval("(define f car)").unwrap(), "f");
    assert_eq!(ev.eval("(lambda (x) x)").unwrap(), "#<procedure>");
    assert_eq!(ev.eval("((lambda (x) (+ x 1)) 1)").unwrap(), "2");
    assert_eq!(ev.eval("(define nothing)").unwrap(), "nothing");
    assert_eq!(ev.eval("nothing").unwrap(), "nil");
    // 閉包は, 作ったときの環境を覚える
    ev.eval("(define (make-counter) (let ((n 0)) (lambda () (set! n (+ n 1)) n)))")
        .unwrap();
    ev.eval("(define c (make-counter))").unwrap();
    assert_eq!(ev.eval("(c) (c) (c)").unwrap(), "1\n2\n3");
    // 残りの引数
    assert_eq!(ev.eval("((lambda args args) 1 2)").unwrap(), "(1 2)");
    assert_eq!(
        ev.eval("((lambda (a . rest) rest) 1 2 3)").unwrap(),
        "(2 3)"
    );
    assert_eq!(
        ev.eval("(define (g a . r) (list a r)) (g 1)").unwrap(),
        "g\n(1 ())"
    );
    assert_eq!(ev.eval("((lambda ()))").unwrap(), "nil");
}

#[test]
fn test_releases_the_environments_after_evaluation() {
    // 名前付き let, letrec, 内側の define は, 環境と手続きの輪を作る. 評価のあとに放す
    let ev = Evaluator::new();
    let code = "(let loop ((i 0)) (if (< i 100) (loop (+ i 1)) i))
                (letrec ((even? (lambda (n) (if (= n 0) #t (odd? (- n 1)))))
                         (odd? (lambda (n) (if (= n 0) #f (even? (- n 1))))))
                  (even? 10))
                (define (f) (define (g) 1) (g))
                (f)";
    assert_eq!(ev.eval(code).unwrap(), "100\n#t\nf\n1");
    assert_eq!(ev.live_frames(), 0);
    // 一番外側の変数から指す手続き (クロージャ) の環境は, 残す
    ev.eval("(define counter (let ((n 0)) (lambda () (set! n (+ n 1)) n)))")
        .unwrap();
    assert_eq!(ev.eval("(counter) (counter)").unwrap(), "1\n2");
    assert_eq!(ev.live_frames(), 1);
}

#[test]
fn test_control_forms() {
    assert_eq!(eval("(if #t 1 2) (if #f 1 2) (if #f 1)"), "1\n2\nnil");
    assert_eq!(eval("(if 0 'yes 'no)"), "yes");
    assert_eq!(eval("(begin 1 2 3) (begin)"), "3\nnil");
    assert_eq!(eval("(let ((a 1) (b 2)) (+ a b))"), "3");
    assert_eq!(eval("(let* ((a 1) (b (+ a 1))) (* a b))"), "2");
    assert_eq!(
        eval("(letrec ((even? (lambda (n) (if (= n 0) #t (odd? (- n 1))))) (odd? (lambda (n) (if (= n 0) #f (even? (- n 1)))))) (even? 10))"),
        "#t"
    );
    assert_eq!(eval("(letrec* ((a 1) (b (+ a 1))) b)"), "2");
    assert_eq!(
        eval("(let loop ((i 0) (acc '())) (if (= i 3) acc (loop (+ i 1) (cons i acc))))"),
        "(2 1 0)"
    );
    assert_eq!(
        eval("(cond ((> 1 2) 'a) ((< 1 2) 'b) (else 'c)) (cond (#f 1) (else 2)) (cond (#f 1)) (cond (5))"),
        "b\n2\nnil\n5"
    );
    assert_eq!(
        eval("(case 3 ((1 2) 'low) ((3 4) 'mid) (else 'high)) (case 9 ((1) 'a) (else 'b)) (case 9 ((1) 'a))"),
        "mid\nb\nnil"
    );
    assert_eq!(eval("(and) (and 1 2) (and 1 #f 2)"), "#t\n2\n#f");
    assert_eq!(eval("(or) (or #f 2) (or #f #f) (or 1 2)"), "#f\n2\n#f\n1");
    assert_eq!(
        eval("(when #t 1 2) (when #f 1) (unless #f 3) (unless #t 3)"),
        "2\nnil\n3\nnil"
    );
    assert_eq!(eval("(define x 1) (set! x 5) x"), "x\nnil\n5");
}

#[test]
fn test_tail_calls_do_not_use_the_depth() {
    assert_eq!(
        eval("(define (count n) (if (= n 0) 'done (count (- n 1)))) (count 100000)"),
        "count\ndone"
    );
    // 末尾でない入れ子は, 深さの上限まで
    assert_eq!(
        eval("(define (sum n) (if (= n 0) 0 (+ n (sum (- n 1))))) (sum 1000)"),
        "sum\n500500"
    );
    assert_eq!(
        eval_err(&format!(
            "(define (sum n) (if (= n 0) 0 (+ n (sum (- n 1))))) (sum {})",
            MAX_DEPTH
        )),
        "Too deep recursion"
    );
}

#[test]
fn test_number_procedures() {
    assert_eq!(eval("(+) (*) (- 10 1 2) (/ 1 2 2)"), "0\n1\n7\n0.25");
    assert_eq!(
        eval("(= 1 1 1) (< 1 2 3) (< 1 3 2) (> 3 2) (<= 1 1) (>= 2 3)"),
        "#t\n#t\n#f\n#t\n#t\n#f"
    );
    assert_eq!(eval("(abs -3) (min 3 1 2) (max 3 1 2)"), "3\n1\n3");
    assert_eq!(
        eval("(quotient 7 2) (remainder -7 2) (modulo -7 2)"),
        "3\n-1\n1"
    );
    assert_eq!(
        eval("(number? 1) (number? 'a) (integer? 1) (integer? 1.5) (zero? 0) (zero? 1)"),
        "#t\n#f\n#t\n#f\n#t\n#f"
    );
}

#[test]
fn test_predicates_and_equality() {
    assert_eq!(
        eval("(not #f) (not 0) (boolean? #f) (boolean? 0)"),
        "#t\n#f\n#t\n#f"
    );
    assert_eq!(eval("(symbol? 'a) (string? \"a\") (procedure? car) (procedure? (lambda () 1)) (procedure? 1)"), "#t\n#t\n#t\n#t\n#f");
    assert_eq!(
        eval("(null? '()) (null? '(1)) (pair? '(1)) (pair? '()) (list? '()) (list? 1)"),
        "#t\n#f\n#t\n#f\n#t\n#f"
    );
    assert_eq!(
        eval("(eq? 'a 'a) (eqv? 1 2) (equal? '(1 (2)) '(1 (2)))"),
        "#t\n#f\n#t"
    );
    assert_eq!(
        eval("(define (f) 1) (eq? f f) (eq? f (lambda () 1))"),
        "f\n#t\n#f"
    );
}

#[test]
fn test_list_procedures() {
    assert_eq!(
        eval("(car '(1 2)) (cdr '(1 2)) (cons 0 '(1))"),
        "1\n(2)\n(0 1)"
    );
    assert_eq!(
        eval("(length '(1 2 3)) (append '(1) '() '(2 3)) (append) (reverse '(1 2 3))"),
        "3\n(1 2 3)\n()\n(3 2 1)"
    );
    assert_eq!(eval("(list-ref '(a b c) 1)"), "b");
    assert_eq!(
        eval("(map (lambda (x) (* x x)) '(1 2 3)) (map + '(1 2) '(10 20 30))"),
        "(1 4 9)\n(11 22)"
    );
    assert_eq!(
        eval("(define acc '()) (for-each (lambda (x) (set! acc (cons x acc))) '(1 2)) acc"),
        "acc\nnil\n(2 1)"
    );
    assert_eq!(eval("(filter (lambda (x) (> x 1)) '(1 2 3))"), "(2 3)");
    assert_eq!(eval("(apply + 1 2 '(3 4)) (apply list '())"), "10\n()");
    assert_eq!(
        eval("(assoc 'b '((a 1) (b 2))) (assoc 'z '((a 1)))"),
        "(b 2)\n#f"
    );
    assert_eq!(eval("(member 2 '(1 2 3)) (member 9 '(1))"), "(2 3)\n#f");
    assert_eq!(eval("()"), "()");
}

#[test]
fn test_string_procedures() {
    assert_eq!(
        eval("(string-length \"あいう\") (string-append \"a\" \"b\" \"c\")"),
        "3\n\"abc\""
    );
    assert_eq!(
        eval("(substring \"hello\" 1 3) (substring \"hello\" 2)"),
        "\"el\"\n\"llo\""
    );
    assert_eq!(
        eval("(string=? \"a\" \"a\") (string<? \"a\" \"b\")"),
        "#t\n#t"
    );
    assert_eq!(
        eval("(string-upcase \"ab\") (string-downcase \"AB\")"),
        "\"AB\"\n\"ab\""
    );
    assert_eq!(
        eval("(number->string 42) (string->number \" 1.5 \") (string->number \"x\")"),
        "\"42\"\n1.5\n#f"
    );
    assert_eq!(
        eval("(symbol->string 'abc) (string->symbol \"abc\")"),
        "\"abc\"\nabc"
    );
    assert_eq!(
        eval("(format \"%s=%S %d%% %s\" \"a\" \"b\" 3.7 '(1 \"x\"))"),
        "\"a=\\\"b\\\" 3% (1 \\\"x\\\")\""
    );
}

#[test]
fn test_editor_procedures_use_the_host() {
    let ev = Evaluator::new();
    let editor = FakeEditor::new("hello");
    let run = |code: &str| ev.eval_with(code, &editor);
    assert_eq!(run("(point) (point-min) (point-max)").unwrap(), "1\n1\n6");
    assert_eq!(
        run("(goto-char 6) (insert \" world\" \"!\")").unwrap(),
        "6\nnil"
    );
    assert_eq!(editor.text(), "hello world!");
    assert_eq!(
        run("(buffer-substring 1 6) (buffer-name)").unwrap(),
        "\"hello\"\n\"*scratch*\""
    );
    assert_eq!(
        run("(delete-region 6 13) (buffer-string)").unwrap(),
        "nil\n\"hello\""
    );
    assert_eq!(run("(message \"at %d\" (point))").unwrap(), "\"at 13\"");
    assert_eq!(
        run("(goto-char 1) (call-interactively 'forward-char) (point)").unwrap(),
        "1\n#t\n2"
    );
    assert_eq!(run("(command-execute \"forward-char\")").unwrap(), "#t");
    assert_eq!(
        run("(call-interactively 'none)").unwrap_err(),
        "Undefined command"
    );
    assert!(editor.calls().contains(&"message \"at 13\"".to_string()));
    // エディタがなければ, 失敗する
    assert_eq!(
        ev.eval("(point)").unwrap_err(),
        "No editor is attached (point)"
    );
}

#[test]
fn test_define_command_and_global_set_key() {
    let ev = Evaluator::new();
    let editor = FakeEditor::new("");
    assert_eq!(
        ev.eval("(define-command (say-hi) (insert \"hi\"))")
            .unwrap(),
        "say-hi"
    );
    assert_eq!(ev.eval("(define-command shout say-hi)").unwrap(), "shout");
    assert_eq!(
        ev.eval("(define-command (say-hi) (insert \"hi!\"))")
            .unwrap(),
        "say-hi"
    );
    assert_eq!(ev.commands(), vec!["say-hi", "shout"]);
    assert_eq!(ev.run_command("say-hi", &editor).unwrap(), "nil");
    assert_eq!(editor.text(), "hi!");
    // Lisp のコマンドは, エディタに頼まずに呼ぶ
    assert_eq!(
        ev.eval_with("(call-interactively 'say-hi)", &editor)
            .unwrap(),
        "nil"
    );
    assert_eq!(editor.text(), "hi!hi!");
    assert_eq!(
        ev.run_command("none", &editor).unwrap_err(),
        "Undefined command: none"
    );
    // 内側で作ったコマンドは, 外からは見えない
    ev.eval("(let () (define-command (inner) 1))").unwrap();
    assert_eq!(
        ev.run_command("inner", &editor).unwrap_err(),
        "Unbound variable: inner"
    );

    assert_eq!(
        ev.eval("(global-set-key \"C-c h\" 'say-hi)").unwrap(),
        "say-hi"
    );
    ev.eval("(global-set-key \"<f5>\" \"shout\")").unwrap();
    ev.eval("(global-set-key \"C-c h\" 'shout)").unwrap();
    assert_eq!(
        ev.key_bindings(),
        vec![
            KeyBinding {
                key: "F5".to_string(),
                command: "shout".to_string()
            },
            KeyBinding {
                key: "Ctrl-c h".to_string(),
                command: "shout".to_string()
            },
        ]
    );
    assert_eq!(
        ev.eval("(global-set-key \"C-\" 'x)").unwrap_err(),
        "Invalid key: \"C-\""
    );
}

#[test]
fn test_evaluator_reports_the_errors() {
    let cases = [
        ("(1 2)", "Not a procedure: 1"),
        ("undefined", "Unbound variable: undefined"),
        ("(-)", "- expects at least one argument"),
        ("(- \"a\")", "- expects numbers"),
        ("(* 2 #t)", "* expects numbers"),
        ("(/ 1)", "/ expects at least two arguments"),
        ("(/ #f 1)", "/ expects numbers"),
        ("(quote 1 2)", "quote expects 1 argument"),
        ("(if 1)", "if expects 2 or 3 arguments"),
        ("(define)", "define expects a name and a value"),
        ("(define 1 2)", "define expects a name and a value"),
        ("(define (1) 2)", "define expects a name"),
        ("(define-command x 1)", "define-command expects a procedure"),
        ("(set! undefined-x 1)", "Unbound variable: undefined-x"),
        ("(set! 1 2)", "set! expects a name and a value"),
        ("(lambda)", "lambda expects parameters"),
        ("(lambda 1 2)", "Invalid parameter list"),
        ("(lambda (1) 2)", "Invalid parameter list"),
        ("(lambda (a .) 2)", "Invalid parameter list"),
        ("(lambda (a . b c) 2)", "Invalid parameter list"),
        ("(let)", "let expects bindings"),
        ("(let 1 2)", "let expects bindings"),
        ("(let (1) 2)", "let expects bindings"),
        ("(let ((a)) 2)", "let expects bindings"),
        ("(let*)", "let* expects bindings"),
        ("(cond 1)", "cond expects clauses"),
        ("(cond ())", "cond expects clauses"),
        ("(case)", "case expects a key"),
        ("(case 1 2)", "case expects clauses"),
        ("(case 1 (2 3))", "case expects clauses"),
        ("(when)", "when expects a test"),
        (
            "((lambda (a) a))",
            "#<procedure> expects 1 arguments, given 0",
        ),
        (
            "(define (f a . r) a) (f)",
            "f expects at least 1 arguments, given 0",
        ),
        ("(< 'a)", "< expects numbers"),
        ("(<)", "< expects at least one argument"),
        ("(max)", "max expects at least one argument"),
        ("(abs 1 2)", "abs expects 1 argument, given 2"),
        ("(eq? 1)", "eq? expects 2 arguments, given 1"),
        ("(point 1)", "point expects 0 arguments, given 1"),
        ("(modulo 1 0)", "modulo: division by zero"),
        ("(zero? 'a)", "zero? expects numbers"),
        ("(car '())", "car expects a non-empty list"),
        ("(cons 1 2)", "cons expects a list as the second argument"),
        ("(length 1)", "length expects lists"),
        ("(list-ref '(1) 1)", "list-ref: index out of range"),
        (
            "(list-ref '(1) -1)",
            "list-ref expects a non-negative integer index",
        ),
        ("(map car)", "map expects a procedure and lists"),
        ("(map)", "map expects a procedure and lists"),
        ("(map car 1)", "map expects lists"),
        ("(apply)", "apply expects a procedure and a list"),
        ("(apply +)", "apply expects a procedure and a list"),
        ("(assoc 1 '(1))", "assoc expects a list of lists"),
        ("(string-length 1)", "string-length expects strings"),
        ("(substring \"a\")", "substring expects 2 or 3 arguments"),
        ("(substring \"abc\" 2 1)", "substring: index out of range"),
        ("(substring \"abc\" 0 9)", "substring: index out of range"),
        ("(symbol->string \"a\")", "symbol->string expects a symbol"),
        ("(format)", "format expects a format string"),
        ("(format \"%s\")", "format: not enough arguments"),
        ("(format \"%x\" 1)", "format: invalid directive"),
        ("(format \"%\")", "format: not enough arguments"),
        ("(format \"%d\" \"a\")", "format expects numbers"),
        ("(goto-char \"a\")", "goto-char expects numbers"),
        ("(delete-region 1 \"a\")", "delete-region expects numbers"),
        ("(insert 1)", "insert expects strings"),
        ("(message 1)", "format expects strings"),
        (
            "(call-interactively 1)",
            "call-interactively expects a command name",
        ),
        (
            "(global-set-key 1 2)",
            "global-set-key expects a key and a command name",
        ),
        ("(filter car 1)", "filter expects lists"),
        (
            "(filter (lambda (x) (car x)) '(1))",
            "car expects a non-empty list",
        ),
    ];
    for (code, expected) in cases {
        assert_eq!(eval_err(code), expected, "{}", code);
    }
}

#[test]
fn test_parser_reads_escapes_and_trailing_spaces() {
    let values = Parser::new("\"a\\tb\\\"c\\\\\" ").parse().unwrap();
    assert_eq!(values, vec![Value::String("a\tb\"c\\".to_string())]);
    // マイナスだけは, 記号
    assert_eq!(
        Parser::new("-").parse().unwrap(),
        vec![Value::Symbol("-".to_string())]
    );
}

#[test]
fn test_parses_deep_nesting_on_the_big_stack() {
    // 読むのも, 評価の大きなスタックのスレッドで行う (呼ぶスレッドのスタックはあふれない)
    let depth = 2000;
    let code = format!("{}1{}", "(begin ".repeat(depth), ")".repeat(depth));
    assert_eq!(eval(&code), "1");
    // 深すぎる入れ子は, 読むときに断る
    let deep = format!("'{}{}", "(".repeat(MAX_DEPTH), ")".repeat(MAX_DEPTH));
    assert_eq!(eval_err(&deep), "Too deeply nested");
}

#[test]
fn test_parser_reports_the_errors() {
    let err = |code: &str| Parser::new(code).parse().unwrap_err();
    assert_eq!(err("@"), "Unexpected character: @");
    assert_eq!(err("\"a\\x\""), "Invalid escape sequence");
    assert_eq!(err("\"abc"), "Unclosed string");
    assert_eq!(err("'"), "Unexpected end of input");
}

#[test]
fn test_value_display_and_equality_of_nil_and_functions() {
    assert_eq!(Value::Nil.to_string(), "nil");
    assert_eq!(
        Value::Function("car".to_string()).to_string(),
        "#<procedure:car>"
    );
    assert_eq!(Value::Nil, Value::Nil);
    assert_ne!(Value::Nil, Value::Boolean(false));
    assert_eq!(eval("car"), "#<procedure:car>");
}
