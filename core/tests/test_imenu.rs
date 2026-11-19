//! `imenu` (`lisp/imenu.scm`) のテスト.

use std::sync::{Arc, Mutex};

use rseditor_core::command_loop::{CommandLoop, LoopUi};
use rseditor_core::editor::Editor;
use rseditor_core::keymap::Keymap;
use rseditor_core::scheme::editor_host::EditorHost;
use rseditor_core::scheme::evaluator::Evaluator;
use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::value::Value;

/// 偽の画面 (何も頼まない).
struct Ui;

impl Host for Ui {
    fn call(&self, _op: &str, _args: Vec<Value>) -> Result<Value, rseditor_core::error::Error> {
        Ok(Value::Nil)
    }
}

impl LoopUi for Ui {
    fn say(&self, _text: &str) {}

    fn stop_macro(&self) {}

    fn set_display(&self, _truncate_lines: bool, _selective_display: Option<usize>) {}
}

fn setup() -> CommandLoop<Ui> {
    let host = EditorHost::new(Editor::new(), Ui);
    CommandLoop::new(
        host,
        Evaluator::new(),
        Arc::new(Mutex::new(Keymap::default())),
    )
    .unwrap()
}

fn eval(commands: &CommandLoop<Ui>, code: &str) -> String {
    commands
        .evaluator()
        .eval_with(code, commands.host())
        .unwrap()
        .to_string()
}

fn eval_error(commands: &CommandLoop<Ui>, code: &str) -> String {
    commands
        .evaluator()
        .eval_with(code, commands.host())
        .unwrap_err()
        .to_string()
}

/// モードの名前を決めて, 本文を入れる.
fn fill(commands: &CommandLoop<Ui>, mode: &str, text: &str) {
    eval(
        commands,
        &format!("(erase-buffer) (set-major-mode-name '{mode}) (insert {text:?})"),
    );
}

#[test]
fn test_imenu_index_of_each_language() {
    let commands = setup();
    fill(
        &commands,
        "scheme-mode",
        "(define x 1)\n(define (foo a) a)\n(define-command (bar . args) 1)\n  (define inner 2)\n",
    );
    assert_eq!(
        eval(&commands, "(map car (imenu--index))"),
        "(\"x\" \"foo\" \"bar\")"
    );
    fill(
        &commands,
        "c-mode",
        "int x;\nstatic int main(int argc)\n{\n}\nvoid helper(void);\n",
    );
    assert_eq!(eval(&commands, "(map car (imenu--index))"), "(\"main\")");
    fill(
        &commands,
        "rust-mode",
        "pub fn run() {}\n    struct S;\npub(crate) async fn go() {}\nlet x = 1;\n",
    );
    assert_eq!(
        eval(&commands, "(map car (imenu--index))"),
        "(\"run\" \"S\" \"go\")"
    );
    fill(
        &commands,
        "typescript-mode",
        "export default class A {}\nfunction f() {}\nconst x = 1;\n",
    );
    assert_eq!(eval(&commands, "(map car (imenu--index))"), "(\"A\" \"f\")");
    fill(&commands, "org-mode", "* One\ntext\n** Two\n");
    assert_eq!(
        eval(&commands, "(map car (imenu--index))"),
        "(\"One\" \"Two\")"
    );
}

#[test]
fn test_imenu_jumps_and_reports_errors() {
    let commands = setup();
    fill(&commands, "rust-mode", "fn a() {}\nfn b() {}\n");
    eval(&commands, "(goto-char 1) (imenu \"b\")");
    assert_eq!(eval(&commands, "(point)"), "11");
    assert_eq!(
        eval_error(&commands, "(imenu \"zzz\")"),
        "No such index entry: zzz"
    );
    // 何も見つからない
    fill(&commands, "rust-mode", "let x = 1;\n");
    assert_eq!(
        eval_error(&commands, "(imenu \"x\")"),
        "No items suitable for an index found in this buffer"
    );
    // 索引を作れないモード
    fill(&commands, "text-mode", "hello\n");
    assert_eq!(
        eval_error(&commands, "(imenu \"x\")"),
        "No imenu tags in this buffer"
    );
    eval(&commands, "(prog-mode)");
    assert_eq!(eval(&commands, "(major-mode-name)"), "prog-mode");
}
