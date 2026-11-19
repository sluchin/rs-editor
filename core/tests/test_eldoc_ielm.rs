//! `eldoc-mode` (`lisp/eldoc.scm`) と `ielm` (`lisp/ielm.scm`) のテスト.

use std::sync::{Arc, Mutex};

use rseditor_core::command_loop::{CommandLoop, LoopUi};
use rseditor_core::editor::Editor;
use rseditor_core::keymap::Keymap;
use rseditor_core::scheme::editor_host::EditorHost;
use rseditor_core::scheme::evaluator::Evaluator;
use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::value::Value;

/// 偽の画面 (何も頼まない).
#[derive(Default)]
struct Ui {
    /// 出した知らせ.
    messages: Mutex<Vec<String>>,
}

impl Host for Ui {
    fn call(&self, op: &str, args: Vec<Value>) -> Result<Value, rseditor_core::error::Error> {
        if op == "message" {
            if let Some(Value::String(text)) = args.first() {
                self.messages.lock().unwrap().push(text.clone());
            }
        }
        Ok(Value::Nil)
    }
}

impl LoopUi for Ui {
    fn say(&self, _text: &str) {}

    fn stop_macro(&self) {}

    fn set_display(&self, _truncate_lines: bool, _selective_display: Option<usize>) {}
}

fn setup() -> CommandLoop<Ui> {
    let host = EditorHost::new(Editor::new(), Ui::default());
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

#[test]
fn test_eldoc_shows_the_arguments_of_the_function_at_point() {
    let commands = setup();
    eval(
        &commands,
        "(set-syntax-dialect 'scheme) (define (my-fn a b) a) (insert \"(my-fn 1 \")",
    );
    // 引数の一覧は, 関数の呼び出しの中で
    assert_eq!(eval(&commands, "(eldoc--string)"), "\"(my-fn A B)\"");
    // 組み込みの手続きは, 説明の最後の使い方の行
    eval(
        &commands,
        "(erase-buffer) (insert \"(buffer-minor-mode-p \")",
    );
    assert_eq!(
        eval(&commands, "(eldoc--string)"),
        "\"(buffer-minor-mode-p NAME)\""
    );
    // 使い方の行が無い手続きは, 出さない
    eval(&commands, "(erase-buffer) (insert \"(car \")");
    assert_eq!(eval(&commands, "(eldoc--string)"), "#f");
    // 関数でない所と, 呼び出しの外では, 出さない
    eval(
        &commands,
        "(erase-buffer) (insert \"(zzz-no-such-function 1\")",
    );
    assert_eq!(eval(&commands, "(eldoc--string)"), "#f");
    eval(&commands, "(erase-buffer) (insert \"plain text\")");
    assert_eq!(eval(&commands, "(eldoc--string)"), "#f");
    // Lisp のモードでなければ, 出さない
    eval(
        &commands,
        "(erase-buffer) (insert \"(my-fn a\") (set-major-mode-name 'text-mode)",
    );
    assert_eq!(eval(&commands, "(eldoc--string)"), "#f");
}

#[test]
fn test_eldoc_mode_shows_once_after_a_command() {
    let commands = setup();
    eval(
        &commands,
        "(set-syntax-dialect 'scheme) (define (my-fn a b) a) (insert \"(my-fn 1\")",
    );
    assert_eq!(eval(&commands, "(eldoc-mode 1)"), "#t");
    eval(&commands, "(run-hooks 'post-command-hook)");
    eval(&commands, "(run-hooks 'post-command-hook)");
    let messages = commands.host().ui().messages.lock().unwrap().clone();
    assert_eq!(messages, vec!["(my-fn A B)".to_string()]);
    // 切ると, 出さない
    assert_eq!(eval(&commands, "(eldoc-mode 0)"), "#f");
    eval(&commands, "(run-hooks 'post-command-hook)");
    assert_eq!(commands.host().ui().messages.lock().unwrap().len(), 1);
    assert_eq!(eval(&commands, "(eldoc-mode)"), "#t");
}

#[test]
fn test_ielm_evaluates_the_input_after_the_prompt() {
    let commands = setup();
    eval(&commands, "(ielm)");
    assert_eq!(eval(&commands, "(buffer-name)"), "\"*ielm*\"");
    assert!(eval(&commands, "(buffer-string)").ends_with("ELISP> \""));
    eval(&commands, "(insert \"(+ 1 2)\") (ielm-return)");
    assert!(eval(&commands, "(buffer-string)").ends_with("ELISP> (+ 1 2)\\n3\\nELISP> \""));
    // 失敗は, 値の代わりに知らせる
    eval(&commands, "(insert \"(car 5)\") (ielm-return)");
    assert!(eval(&commands, "(buffer-string)").contains("\\n*** "));
    // 空の入力は, 新しい行だけ
    eval(&commands, "(ielm-return)");
    assert!(eval(&commands, "(buffer-string)").ends_with("ELISP> \\nELISP> \""));
    // 履歴
    eval(&commands, "(ielm-previous-input)");
    assert!(eval(&commands, "(buffer-string)").ends_with("ELISP> (car 5)\""));
    eval(&commands, "(ielm-previous-input) (ielm-previous-input)");
    assert!(eval(&commands, "(buffer-string)").ends_with("ELISP> (+ 1 2)\""));
    eval(&commands, "(ielm-next-input)");
    assert!(eval(&commands, "(buffer-string)").ends_with("ELISP> (car 5)\""));
    eval(&commands, "(ielm-next-input) (ielm-next-input)");
    assert!(eval(&commands, "(buffer-string)").ends_with("ELISP> \""));
    // 2 回目に開いても, 中身はそのまま
    let before = eval(&commands, "(buffer-string)");
    eval(&commands, "(ielm)");
    assert_eq!(eval(&commands, "(buffer-string)"), before);
    // 入力が無いときの履歴は何もしない
    eval(&commands, "(set! ielm-history '()) (ielm-previous-input)");
}
