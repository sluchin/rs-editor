//! Lisp (Scheme と Emacs Lisp) のシンタックスハイライト (`core/src/highlight/lisp.rs`) のテスト.
//! 期待値は, Emacs 30 の `scheme-mode` と `emacs-lisp-mode` の font-lock の結果 (`tests/lisp-highlight/` で, 同じ入力を Emacs と比べる).

use rseditor_core::editor::Editor;
use rseditor_core::highlight::{tokenize, Language};
use rseditor_core::scheme::editor_host::EditorHost;
use rseditor_core::scheme::evaluator::Evaluator;
use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::value::Value;

/// 字句に分けて, (本文の中の文字列, 種類の名前) の並びにする (括弧と, 色の付かない字句は含まない).
fn lex(language: Language, lines: &[&str]) -> Vec<(String, &'static str)> {
    let text = lines.join("\n");
    let chars: Vec<char> = text.chars().collect();
    tokenize(language, &text)
        .iter()
        .map(|s| (chars[s.start..s.end].iter().collect(), s.kind.name()))
        .collect()
}

/// Scheme の字句.
fn scheme(lines: &[&str]) -> Vec<(String, &'static str)> {
    lex(Language::Scheme, lines)
}

/// Emacs Lisp の字句.
fn elisp(lines: &[&str]) -> Vec<(String, &'static str)> {
    lex(Language::Elisp, lines)
}

/// `(文字列, 種類)` の並びを作る.
fn tokens(items: &[(&str, &'static str)]) -> Vec<(String, &'static str)> {
    items.iter().map(|(t, k)| (t.to_string(), *k)).collect()
}

#[test]
fn test_scheme_special_forms_and_defined_names() {
    assert_eq!(
        scheme(&["(define (square x) (if (car x) x 1))"]),
        tokens(&[
            ("define", "keyword"),
            ("square", "function-name"),
            ("if", "keyword"),
        ])
    );
    assert_eq!(
        scheme(&["(define ((curried a) b) a)"]),
        tokens(&[("define", "keyword"), ("curried", "function-name")])
    );
    assert_eq!(
        scheme(&["(define-syntax swap! (syntax-rules ()))"]),
        tokens(&[
            ("define-syntax", "keyword"),
            ("swap!", "variable-name"),
            ("syntax-rules", "keyword"),
        ])
    );
    assert_eq!(
        scheme(&["(define-module (a b))"]),
        tokens(&[("define-module", "keyword"), ("a", "type")])
    );
    // define-command は, Emacs と同じく色を付けない
    assert_eq!(
        scheme(&["(define-command hello (lambda () 1))"]),
        tokens(&[("lambda", "keyword")])
    );
    assert_eq!(
        scheme(&["(let loop ((i 0)) i) (let ((j 1)) j)"]),
        tokens(&[
            ("let", "keyword"),
            ("loop", "function-name"),
            ("let", "keyword"),
        ])
    );
    assert_eq!(
        scheme(&["(library (srfi 1) (import))"]),
        tokens(&[
            ("library", "keyword"),
            ("srfi", "type"),
            ("import", "keyword"),
        ])
    );
    // (の直後でなければ, キーワードでも色を付けない. [ の後も付けない
    assert_eq!(scheme(&["(list if 'define) [if]"]), vec![]);
    // `'(define y)` は, Emacs と同じく色を付ける
    assert_eq!(
        scheme(&["'(define y)"]),
        tokens(&[("define", "keyword"), ("y", "function-name")])
    );
    // 定義する名前は, 次の字句で, 同じ行にあるものだけ
    assert_eq!(
        scheme(&["(define \"x\" y)", "(define", " z)"]),
        tokens(&[
            ("define", "keyword"),
            ("\"x\"", "string"),
            ("define", "keyword"),
        ])
    );
}

#[test]
fn test_scheme_define_family() {
    assert_eq!(
        scheme(&["(define* (f) 1) (define-public g 1) (define-method (m x) x) (define-generic h)"]),
        tokens(&[
            ("define*", "keyword"),
            ("f", "function-name"),
            ("define-public", "keyword"),
            ("g", "function-name"),
            ("define-method", "keyword"),
            ("m", "function-name"),
            ("define-generic", "keyword"),
            ("h", "function-name"),
        ])
    );
    assert_eq!(
        scheme(&["(define-generic-procedure p) (define-macro mm) (define-class <c> ())"]),
        tokens(&[
            ("define-generic-procedure", "keyword"),
            ("p", "function-name"),
            ("define-macro", "keyword"),
            ("mm", "variable-name"),
            ("define-class", "keyword"),
            ("<c>", "type"),
        ])
    );
    // 仲間でない名前と, `define` で始まらない名前
    assert_eq!(scheme(&["(define-foo x) (defin x) (undefine x)"]), vec![]);
}

#[test]
fn test_scheme_types_and_keys() {
    assert_eq!(
        scheme(&["(make <point> :x #:y : x <> #: <a)"]),
        tokens(&[("<point>", "type"), (":x", "builtin"), ("#:y", "builtin")])
    );
}

#[test]
fn test_scheme_comments_and_strings_across_lines() {
    assert_eq!(
        scheme(&["(+ 1 2) ; sum (if"]),
        tokens(&[("; sum (if", "comment")])
    );
    assert_eq!(
        scheme(&["\"a \\\" (if", "b\" (when)"]),
        tokens(&[
            ("\"a \\\" (if", "string"),
            ("b\"", "string"),
            ("when", "keyword"),
        ])
    );
    assert_eq!(
        scheme(&["#| a #| b |# (if", "c |# (or)"]),
        tokens(&[
            ("#| a #| b |# (if", "comment"),
            ("c |#", "comment"),
            ("or", "keyword"),
        ])
    );
    // 閉じない `#|` は, 本文の終わりまで
    assert_eq!(
        scheme(&["#| a", "(if"]),
        tokens(&[("#| a", "comment"), ("(if", "comment")])
    );
}

#[test]
fn test_scheme_datum_comments() {
    let comments = |line: &str| -> String {
        scheme(&[line])
            .into_iter()
            .filter(|(_, kind)| *kind == "comment")
            .map(|(text, _)| text)
            .collect()
    };
    assert_eq!(comments("#;(if (a) \"b)\") (when)"), "#;(if (a) \"b)\")");
    assert_eq!(
        scheme(&["#;(if", "x) (when)"]).pop(),
        Some(("when".to_string(), "keyword"))
    );
    assert_eq!(comments("#; 'x (if)"), "#; 'x");
    assert_eq!(comments("#;\"abc"), "#;\"abc");
    assert_eq!(comments("#;[a] (if)"), "#;[a]");
    // 引用の印と, `,` `` ` `` も
    assert_eq!(comments("#;`(a ,b) (if)"), "#;`(a ,b)");
}

#[test]
fn test_scheme_characters_and_non_words() {
    // `#\(` は文字なので, 次の `(if` は (の直後
    assert_eq!(
        scheme(&["(list #\\( `(if ,x))"]),
        tokens(&[("if", "keyword")])
    );
    assert_eq!(scheme(&["#\\"]), vec![]);
}

#[test]
fn test_elisp_definitions() {
    assert_eq!(
        elisp(&["(defun my-fn (a &optional b) (car a))"]),
        tokens(&[
            ("defun", "keyword"),
            ("my-fn", "function-name"),
            ("&optional", "type"),
        ])
    );
    assert_eq!(
        elisp(&["(defvar v 1) (defalias 'f #'g) (defgroup my nil)"]),
        tokens(&[
            ("defvar", "keyword"),
            ("v", "variable-name"),
            ("defalias", "keyword"),
            ("f", "function-name"),
            ("defgroup", "keyword"),
            ("my", "type"),
        ])
    );
    // `(` の後の名前は, 関数なら色を付けず, 型なら付ける
    assert_eq!(
        elisp(&["(defun (f)) (cl-defstruct (st (:copier nil)))"]),
        tokens(&[
            ("defun", "keyword"),
            ("cl-defstruct", "keyword"),
            ("st", "type"),
            (":copier", "builtin"),
        ])
    );
    // 仲間でない名前と, `&` と `:` だけの名前
    assert_eq!(elisp(&["(defunx a) (a & :)"]), vec![]);
}

#[test]
fn test_elisp_macros_features_and_warnings() {
    assert_eq!(
        elisp(&["(require 'cl-lib) (throw (x)) (error \"x\") (when t) (car t)"]),
        tokens(&[
            ("require", "keyword"),
            ("cl-lib", "constant"),
            ("throw", "keyword"),
            ("error", "warning"),
            ("\"x\"", "string"),
            ("when", "keyword"),
        ])
    );
}

#[test]
fn test_elisp_quoted_lists_and_let_bindings() {
    assert_eq!(
        elisp(&["(defvar x '(progn)) '(a (when b))"]),
        tokens(&[
            ("defvar", "keyword"),
            ("x", "variable-name"),
            ("when", "keyword"),
        ])
    );
    assert_eq!(
        elisp(&["(let ((when 1)) (cond ((progn 1))))"]),
        tokens(&[
            ("let", "keyword"),
            ("cond", "keyword"),
            ("progn", "keyword"),
        ])
    );
    // `` `(`` と `#'(` は, 関数を呼ぶリスト
    assert_eq!(
        elisp(&["`(when ,(progn 1)) #'(lambda () 1)"]),
        tokens(&[
            ("when", "keyword"),
            ("progn", "keyword"),
            ("lambda", "keyword"),
        ])
    );
}

#[test]
fn test_elisp_strings() {
    // 正規表現のグループ. `\\\\` は, 正規表現の `\` そのもの
    assert_eq!(
        elisp(&[r#""\\(a\\|b\\)\\(?:c\\) \\\\(d""#]),
        tokens(&[
            ("\"", "string"),
            (r"\\(", "regexp-group"),
            ("a", "string"),
            (r"\\|", "regexp-group"),
            ("b", "string"),
            (r"\\)\\(?:", "regexp-group"),
            ("c", "string"),
            (r"\\)", "regexp-group"),
            (r#" \\\\(d""#, "string"),
        ])
    );
    // キー, キーマップ, `\\=`, `name'、要らない `\`
    assert_eq!(
        elisp(&[r#""\\[cmd] \\<map> \\= `name' \. \n""#]),
        tokens(&[
            (r#""\\["#, "string"),
            ("cmd", "constant"),
            (r"] \\<", "string"),
            ("map", "variable-name"),
            ("> ", "string"),
            (r"\\=", "builtin"),
            (" `", "string"),
            ("name", "constant"),
            ("' ", "string"),
            (r"\", "warning"),
            (r#". \n""#, "string"),
        ])
    );
    // 名前の中の `\` で始まる 2 文字は, 名前の一部 (`\]` の次の `]` で名前が終わる)
    assert_eq!(
        elisp(&[r#""\\[a\]b] ""#]),
        tokens(&[
            (r#""\\["#, "string"),
            (r"a\]b", "constant"),
            (r#"] ""#, "string"),
        ])
    );
    // 行の始めの `\(` は, 要るもの
    assert_eq!(
        elisp(&["\"a", "\\(b\""]),
        tokens(&[("\"a", "string"), ("\\(b\"", "string")])
    );
    // 数字つきのグループ, `{map}`, ‘name’
    assert_eq!(
        elisp(&[r#""\\(?12:x\\) \\{map} ‘sym’""#]),
        tokens(&[
            ("\"", "string"),
            (r"\\(?12:", "regexp-group"),
            ("x", "string"),
            (r"\\)", "regexp-group"),
            (r" \\{", "string"),
            ("map", "variable-name"),
            ("} ‘", "string"),
            ("sym", "constant"),
            ("’\"", "string"),
        ])
    );
    // 形が崩れていれば, 色を分けない (`(?x` は数字も `:` も無い, 閉じない `\\[`, `\\` の後が `(` でない)
    assert_eq!(
        elisp(&[r#""\\(?x \\[ab \\<c \\a `d "#]),
        tokens(&[
            ("\"", "string"),
            (r"\\(", "regexp-group"),
            (r"?x \\[ab \\<c \\a `d ", "string"),
        ])
    );
}

#[test]
fn test_elisp_comments() {
    assert_eq!(
        elisp(&[";; see `foo' and `bar'", "(when t)"]),
        tokens(&[
            (";; see `", "comment"),
            ("foo", "constant"),
            ("' and `", "comment"),
            ("bar", "constant"),
            ("'", "comment"),
            ("when", "keyword"),
        ])
    );
    // 閉じない `name は, そのまま
    assert_eq!(elisp(&[";; `open"]), tokens(&[(";; `open", "comment")]));
}

#[test]
fn test_elisp_arguments_after_a_closed_multiline_list() {
    assert_eq!(
        elisp(&["(foo (a", "      b) c \"s\")"]),
        tokens(&[("c \"s\")", "warning")])
    );
    assert_eq!(
        elisp(&["(foo (a", "      b)) c"]),
        tokens(&[("c", "warning")])
    );
    // 閉じる括弧と, コメントと, 行の終わりは, 警告しない
    assert_eq!(
        elisp(&["(foo (a", "      b) ; x", "  b))"]),
        tokens(&[("; x", "comment")])
    );
    // 同じ行で開いた括弧は, 警告しない
    assert_eq!(elisp(&["(foo (a b) c)"]), vec![]);
    // 空白だけが続いても, 警告しない (後ろの字句から)
    assert_eq!(elisp(&["(foo (a", "      b)   "]), vec![]);
}

#[test]
fn test_elisp_characters() {
    assert_eq!(
        elisp(&["(list ?( ?\\( ?a) (when)"]),
        tokens(&[("when", "keyword")])
    );
}

#[test]
fn test_non_breaking_space_is_a_space() {
    assert_eq!(
        elisp(&["(\u{a0}defun\u{a0}f)"]),
        tokens(&[("defun", "keyword"), ("f", "function-name")])
    );
}

/// 何も受けない偽の画面.
struct NoUi;

impl Host for NoUi {
    fn call(&self, op: &str, _args: Vec<Value>) -> Result<Value, rseditor_core::error::Error> {
        match op {
            "message" => Ok(Value::Nil),
            other => Err(format!("ui: {other}").into()),
        }
    }
}

/// 基本のコマンドを読んだ評価器とエディタ.
fn setup(text: &str) -> (Evaluator, EditorHost<NoUi>) {
    let ev = Evaluator::new();
    let host = EditorHost::new(Editor::new(), NoUi);
    ev.eval_with(include_str!("../../lisp/simple.scm"), &host)
        .unwrap();
    host.editor().current_buffer_mut().load(text);
    (ev, host)
}

#[test]
fn test_buffer_language_follows_the_syntax_dialect() {
    let (ev, host) = setup("(define x 1)");
    // 始めの `*scratch*` は, lisp-interaction-mode (方言は Fundamental のままでも, 色は Scheme)
    assert_eq!(
        host.editor().current_buffer().highlight_language(),
        Some(Language::Scheme)
    );
    ev.eval_with("(fundamental-mode)", &host).unwrap();
    assert_eq!(host.editor().current_buffer().highlight_language(), None);
    ev.eval_with("(scheme-mode)", &host).unwrap();
    assert_eq!(
        host.editor().current_buffer().highlight_language(),
        Some(Language::Scheme)
    );
    assert_eq!(
        ev.eval_with("(token-spans)", &host).unwrap(),
        "((2 8 keyword) (9 10 function-name))"
    );
    ev.eval_with("(emacs-lisp-mode)", &host).unwrap();
    assert_eq!(
        host.editor().current_buffer().highlight_language(),
        Some(Language::Elisp)
    );
    // `*scratch*` の lisp-interaction-mode は, Scheme (方言も Scheme)
    ev.eval_with("(lisp-interaction-mode)", &host).unwrap();
    assert_eq!(
        host.editor().current_buffer().highlight_language(),
        Some(Language::Scheme)
    );
    assert_eq!(
        ev.eval_with("major-mode", &host).unwrap(),
        "lisp-interaction-mode"
    );
    assert_eq!(
        ev.eval_with("(token-spans)", &host).unwrap(),
        "((2 8 keyword) (9 10 function-name))"
    );
    // C は, 方言が Fundamental で, `major-mode` の名前で決まる
    ev.eval_with("(c-mode)", &host).unwrap();
    assert_eq!(
        host.editor().current_buffer().highlight_language(),
        Some(Language::C)
    );
    ev.eval_with("(text-mode)", &host).unwrap();
    assert_eq!(host.editor().current_buffer().highlight_language(), None);
}

#[test]
fn test_lisp_languages_have_no_lexical_table() {
    assert!(Language::Scheme.spec().is_none());
    assert!(Language::Elisp.spec().is_none());
    assert!(Language::C.spec().is_some());
}
