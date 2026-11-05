//! 補完の命令 (`lisp/minibuffer.scm`) のテスト. `dabbrev-completion` の期待値は, `emacs -Q --batch` の結果
//! (`core/tests/edit.json`. `core/tests/test_edit.rs` で比べる). ここは, Emacs の試験に入れない `complete-symbol` と,
//! 補完の一覧を試す.

use rseditor_core::editor::Editor;
use rseditor_core::scheme::editor_host::EditorHost;
use rseditor_core::scheme::evaluator::Evaluator;
use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::value::Value;
use std::sync::Mutex;

/// メッセージを覚える画面の偽物.
#[derive(Default)]
struct Messages(Mutex<Vec<String>>);

impl Host for Messages {
    fn call(&self, op: &str, args: Vec<Value>) -> Result<Value, rseditor_core::error::Error> {
        match op {
            "message" => {
                if let Some(Value::String(text)) = args.first() {
                    self.0.lock().unwrap().push(text.clone());
                }
                Ok(Value::Nil)
            }
            other => Err(format!("ui: {other}").into()),
        }
    }
}

/// 式を評価して, 最後の行を返す (失敗は `error: 理由`).
fn eval(code: &str) -> String {
    let ev = Evaluator::new();
    let host = EditorHost::new(Editor::new(), Messages::default());
    for source in [
        include_str!("../../lisp/simple.scm"),
        include_str!("../../lisp/replace.scm"),
        include_str!("../../lisp/minibuffer.scm"),
        include_str!("../../lisp/dabbrev.scm"),
    ] {
        ev.eval_with(source, &host).unwrap();
    }
    match ev.eval_with(code, &host) {
        Ok(value) => value.lines().last().unwrap_or("").to_string(),
        Err(message) => format!("error: {message}"),
    }
}

#[test]
fn test_complete_symbol() {
    // 1 つに決まれば, 展開する
    assert_eq!(
        eval("(insert \"(string-appe\") (complete-symbol) (buffer-string)"),
        "\"(string-append\""
    );
    // 共通の先頭まで進む
    assert_eq!(
        eval("(insert \"(string-up\") (complete-symbol) (buffer-string)"),
        "\"(string-upcase\""
    );
    // 進まないときは, *Completions* に一覧を出す
    assert_eq!(
        eval(
            "(insert \"string-\") (complete-symbol)
             (list (buffer-name) (with-current-buffer \"*Completions*\"
               (string-prefix? \"Possible completions are:\\nstring-\" (buffer-string))))"
        ),
        "(\"*scratch*\" #t)"
    );
    // 合うものが無ければ, そのまま
    assert_eq!(
        eval("(insert \"zzzq\") (complete-symbol) (buffer-string)"),
        "\"zzzq\""
    );
    // 完全に合う 1 つだけなら, そのまま
    assert_eq!(
        eval("(insert \"string-append\") (complete-symbol) (buffer-string)"),
        "\"string-append\""
    );
}

#[test]
fn test_completion_at_point() {
    // 関数が無ければ, 何もしない
    assert_eq!(
        eval("(insert \"ab\") (completion-at-point) (buffer-string)"),
        "\"ab\""
    );
    // 関数が #f を返せば, 次の関数を試す
    assert_eq!(
        eval(
            "(set! completion-at-point-functions
               (list (lambda () #f) (lambda () (list 1 3 '(\"abc\" \"abd\")))))
             (insert \"ab\") (completion-at-point) (buffer-string)"
        ),
        "\"ab\""
    );
    // dabbrev-completion: 候補が無ければ失敗
    assert_eq!(
        eval("(insert \"xyz\") (dabbrev-completion)"),
        "error: No dynamic expansion for \"xyz\" found in this-buffer"
    );
    // 前置引数があれば, 他のバッファも調べる (表示だけ)
    assert_eq!(
        eval("(insert \"xyz\") (dabbrev-completion 16)"),
        "error: No dynamic expansion for \"xyz\" found"
    );
}
