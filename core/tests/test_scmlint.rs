//! Scheme の lint (`scmlint`) のテスト.

use rseditor_core::scmlint::{lint, run};

fn check(source: &str) -> Vec<String> {
    lint(&[("a.scm".to_string(), source.to_string())])
        .into_iter()
        .map(|(_, line, message)| format!("{line}: {message}"))
        .collect()
}

#[test]
fn test_lint_accepts_valid_code() {
    let source = r#"
;; comment
#| block
comment |#
(define (add a b . rest) "doc" (+ a b (length rest)))
(define-command (my-command . args)
  "A command."
  (let loop ((i 0) (acc '()))
    (if (< i 3) (loop (+ i 1) (cons i acc)) acc))
  #f)
(define x 1 "doc")
(define y '(a . b))
(begin (define z 2))
(define (f lst)
  (define (helper n) (* n 2))
  (let* ((a 1) (b (+ a 1))) (helper (+ a b)))
  (letrec ((even? (lambda (n) (if (= n 0) #t (odd? (- n 1)))))
           (odd? (lambda (n) (if (= n 0) #f (even? (- n 1))))))
    (even? 4))
  (let ((p 1) q) (list p q))
  (do ((i 0 (+ i 1))) ((= i 3) i) (message "%d" i))
  (cond ((null? lst) 1) ((car lst) (car lst)) (else 2))
  (case (car lst) ((1 2) 'low) (else 'high))
  (set! x 3)
  (condition-case err (car lst) (error (message "%s" err)) (quit 1))
  (condition-case #f (car lst))
  `(a ,x ,@lst (b ,(car lst)))
  ((lambda (n) n) 1)
  (lambda args args)
  [list 1 2]
  (list "100%%" "%-5s|%05d|%x|%o|%X" #t :key 1.5 #;(ignored thing) 'sym))
(ert-deftest my-test ()
  (should (equal (f '(1)) 1))
  (should-not #f))
(when (> x 1) (message "%d" x))
(define text "a\"b
c")
(define (f2) (define v 1) (define) (let ("x") v) (lambda) (do) (do ((i)) (#t)) (let) (let loop) (list 1 #;2))
(define "ignored")
(define (f3) (format "\"%-5s|%05d|%x|%o|%X|%S %%" 1 2 3 4 5 6))
"#;
    assert_eq!(check(source), Vec::<String>::new());
}

#[test]
fn test_lint_reports_mistakes() {
    let source = r#"(define (f x)
  (undefined-function x)
  (+ x undefined-variable)
  (set! unknown-target 1))
(define (g)
  (list 'a . 'b))
(define (h) #\a)
(define (i) #(1 2 (3)))
(define lst '())
(define (j x) (cond ((assoc x lst) => cdr) (else 1)))
(define (k) (format "%5.2f and %" 1))
(define (l) (if '() 1 2))
(define (m) (when '() 1))
(define-command (no-doc . args) #f)
(define-command (one . args) "only a string")
(define (n) (list 1 (undefined-in-vector) (ok? ")"))) ; scmlint:ignore
(define (c) #\()
"#;
    assert_eq!(
        check(source),
        vec![
            "2: undefined name: undefined-function",
            "3: undefined name: undefined-variable",
            "4: undefined name: unknown-target",
            "6: dotted pairs are not supported",
            "7: character literals are not supported",
            "8: vectors are not supported",
            "10: cond => is not supported",
            "11: format does not support %.",
            "11: format does not support %end",
            "12: '() is true in this Scheme",
            "13: '() is true in this Scheme",
            "14: define-command without a docstring",
            "15: define-command without a docstring",
            "17: character literals are not supported",
        ]
    );
}

#[test]
fn test_lint_reports_syntax_errors_and_duplicates() {
    assert_eq!(check("(define (f)\n  (g)"), vec!["1: unclosed parenthesis"]);
    assert_eq!(
        check("(define x 1)\n)"),
        vec!["2: unexpected closing parenthesis"]
    );
    assert_eq!(check("#(1 2"), vec!["1: unclosed parenthesis"]);
    assert_eq!(check("'"), Vec::<String>::new());
    assert_eq!(check("\"unclosed \\"), Vec::<String>::new());
    // 同じ名前の定義 (変数と関数でも. ファイルをまたいでも)
    let found = lint(&[
        (
            "a.scm".to_string(),
            "(define (f) 1)\n(define f 2)".to_string(),
        ),
        (
            "b.scm".to_string(),
            "(begin (define-command (f) \"d\" 1))".to_string(),
        ),
    ]);
    assert_eq!(
        found,
        vec![
            (
                "a.scm".to_string(),
                2,
                "f is already defined at a.scm:1".to_string()
            ),
            (
                "b.scm".to_string(),
                1,
                "f is already defined at a.scm:1".to_string()
            ),
        ]
    );
}

#[test]
fn test_run_prints_findings_and_codes() {
    let dir = tempfile::tempdir().unwrap();
    let good = dir.path().join("good.scm");
    let bad = dir.path().join("bad.scm");
    std::fs::write(&good, "(define a 1)\n").unwrap();
    std::fs::write(&bad, "(define b nowhere)\n").unwrap();
    let (good, bad) = (
        good.to_str().unwrap().to_string(),
        bad.to_str().unwrap().to_string(),
    );
    let mut out = String::new();
    assert_eq!(run(std::slice::from_ref(&good), &mut out), 0);
    assert_eq!(out, "");
    assert_eq!(run(&[good.clone(), bad.clone()], &mut out), 1);
    assert_eq!(out, format!("{bad}:1: undefined name: nowhere\n"));
    let mut out = String::new();
    assert_eq!(run(&["/no/such/file.scm".to_string()], &mut out), 2);
    assert_eq!(out, "/no/such/file.scm: cannot read\n");
}
