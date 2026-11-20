//! `edit-kbd-macro` (`lisp/kmacro.scm`) のテスト.

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

#[test]
fn test_edit_kbd_macro_edits_the_keys_as_text() {
    let commands = setup();
    assert_eq!(
        commands
            .evaluator()
            .eval_with("(kmacro-edit-macro)", commands.host())
            .unwrap_err()
            .to_string(),
        "No keyboard macro defined"
    );
    eval(
        &commands,
        "(set! last-kbd-macro '(\"C-a\" \"a\" \"b\" \"RET\"))",
    );
    eval(&commands, "(edit-kbd-macro)");
    assert_eq!(eval(&commands, "(buffer-name)"), "\"*Edit Macro*\"");
    assert!(eval(&commands, "(buffer-string)").ends_with("Macro:\\n\\nC-a a b RET\\n\""));
    // 書き換えて, 確定する (コメントの行は, 数えない)
    eval(&commands, "(insert \"C-e x\\n;; note\\n\")");
    eval(&commands, "(kmacro-edit-finish)");
    assert_eq!(
        eval(&commands, "last-kbd-macro"),
        "(\"C-a\" \"a\" \"b\" \"RET\" \"C-e\" \"x\")"
    );
    assert_ne!(eval(&commands, "(buffer-name)"), "\"*Edit Macro*\"");
    // 空にすると, マクロが無くなる
    eval(
        &commands,
        "(kmacro-edit-macro) (erase-buffer) (kmacro-edit-finish)",
    );
    assert_eq!(eval(&commands, "last-kbd-macro"), "#f");
    // C-c C-c は, 編集のバッファだけ
    assert_eq!(
        commands
            .evaluator()
            .eval_with("(kmacro-edit-finish)", commands.host())
            .unwrap_err()
            .to_string(),
        "Not in the *Edit Macro* buffer"
    );
}
