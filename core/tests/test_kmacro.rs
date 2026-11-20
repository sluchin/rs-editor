//! キーボードマクロの環 (`lisp/kmacro.scm`) のテスト.

use std::sync::{Arc, Mutex};

use rseditor_core::command_loop::{CommandLoop, LoopUi};
use rseditor_core::editor::Editor;
use rseditor_core::keymap::Keymap;
use rseditor_core::scheme::editor_host::EditorHost;
use rseditor_core::scheme::evaluator::Evaluator;
use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::value::Value;

/// 偽の画面 (動かすマクロのキーを覚える).
#[derive(Default)]
struct Ui {
    /// `execute-kbd-macro` に渡されたキーと回数.
    executed: Mutex<Vec<(String, String)>>,
    /// マクロを動かしているか (`executing-kbd-macro-p`).
    executing: Mutex<bool>,
    /// 続けるかの問いの答え.
    answer: Mutex<bool>,
    /// `read-key` が返すキー (先頭から).
    keys: Mutex<Vec<String>>,
}

impl Host for Ui {
    fn call(&self, op: &str, args: Vec<Value>) -> Result<Value, rseditor_core::error::Error> {
        if op == "execute-kbd-macro" {
            self.executed
                .lock()
                .unwrap()
                .push((args[0].to_string(), args[1].to_string()));
        }
        match op {
            "executing-kbd-macro-p" => Ok(Value::Boolean(*self.executing.lock().unwrap())),
            "read-key" => {
                let mut keys = self.keys.lock().unwrap();
                // 答えが尽きたら, 止める
                Ok(Value::String(if keys.is_empty() {
                    "q".to_string()
                } else {
                    keys.remove(0)
                }))
            }
            "key-binding" => Ok(Value::Boolean(args[0].to_string() == "\"C-x\"")),
            "kbd-macro--ask" => Ok(Value::Boolean(*self.answer.lock().unwrap())),
            _ => Ok(Value::Nil),
        }
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

fn eval_error(commands: &CommandLoop<Ui>, code: &str) -> String {
    commands
        .evaluator()
        .eval_with(code, commands.host())
        .unwrap_err()
        .to_string()
}

/// マクロを 3 つ作る (a, b, c の順. 数え上げは, 作るたびに 0).
fn define_three(commands: &CommandLoop<Ui>) {
    eval(
        commands,
        "(set! last-kbd-macro '(\"a\")) (kmacro--push-last) (set! last-kbd-macro '(\"b\")) (kmacro--push-last) (set! last-kbd-macro '(\"c\"))",
    );
}

#[test]
fn test_starting_a_macro_puts_the_last_one_on_the_ring() {
    let commands = setup();
    define_three(&commands);
    assert_eq!(eval(&commands, "last-kbd-macro"), "(\"c\")");
    assert_eq!(
        eval(&commands, "(map car kmacro-ring)"),
        "((\"b\") (\"a\"))"
    );
    // 環の大きさを越えたら, 古いものを捨てる
    eval(&commands, "(set! kmacro-ring-max 2) (kmacro--push-last)");
    assert_eq!(eval(&commands, "(length kmacro-ring)"), "2");
    // マクロを覚え始めると, 今のマクロが環に入る
    eval(&commands, "(set! kmacro-ring '()) (kmacro-start-macro)");
    assert_eq!(eval(&commands, "(map car kmacro-ring)"), "((\"c\"))");
}

#[test]
fn test_cycling_swapping_and_deleting_the_ring() {
    let commands = setup();
    assert_eq!(
        eval_error(&commands, "(kmacro-cycle-ring-next)"),
        "No other keyboard macros"
    );
    define_three(&commands);
    eval(&commands, "(kmacro-cycle-ring-next)");
    assert_eq!(eval(&commands, "last-kbd-macro"), "(\"b\")");
    assert_eq!(
        eval(&commands, "(map car kmacro-ring)"),
        "((\"a\") (\"c\"))"
    );
    eval(&commands, "(kmacro-cycle-ring-previous)");
    assert_eq!(eval(&commands, "last-kbd-macro"), "(\"c\")");
    assert_eq!(
        eval(&commands, "(map car kmacro-ring)"),
        "((\"b\") (\"a\"))"
    );
    eval(&commands, "(kmacro-swap-ring)");
    assert_eq!(eval(&commands, "last-kbd-macro"), "(\"b\")");
    assert_eq!(
        eval(&commands, "(map car kmacro-ring)"),
        "((\"c\") (\"a\"))"
    );
    // 2 番目を動かす
    eval(&commands, "(kmacro-call-ring-2nd)");
    assert_eq!(
        commands
            .host()
            .ui()
            .executed
            .lock()
            .unwrap()
            .last()
            .unwrap(),
        &("(\"c\")".to_string(), "1".to_string())
    );
    // 先頭を消すと, 次が先頭になり, 最後の 1 つを消すとマクロは無くなる
    eval(&commands, "(kmacro-delete-ring-head)");
    assert_eq!(eval(&commands, "last-kbd-macro"), "(\"c\")");
    eval(
        &commands,
        "(kmacro-delete-ring-head) (kmacro-delete-ring-head)",
    );
    assert_eq!(eval(&commands, "last-kbd-macro"), "#f");
}

#[test]
fn test_counter_format_view_and_bind() {
    let commands = setup();
    assert_eq!(
        eval_error(&commands, "(kmacro-view-macro)"),
        "No keyboard macro defined"
    );
    assert_eq!(
        eval_error(&commands, "(kmacro-bind-to-key \"C-c a\")"),
        "No keyboard macro defined"
    );
    eval(&commands, "(set! last-kbd-macro '(\"x\" \"y\"))");
    eval(&commands, "(kmacro-view-macro)");
    eval(
        &commands,
        "(kmacro-set-counter 5) (kmacro-add-counter 3) (kmacro-set-format \"<%d>\")",
    );
    assert_eq!(eval(&commands, "kmacro-counter"), "8");
    assert_eq!(eval(&commands, "kmacro-counter-format"), "\"<%d>\"");
    eval(&commands, "(kmacro-set-format \"\")");
    assert_eq!(eval(&commands, "kmacro-counter-format"), "\"%d\"");
    // キーへの割り当て
    eval(&commands, "(kmacro-bind-to-key \"C-c a\")");
    eval(&commands, "(kmacro-macro-1)");
    assert_eq!(
        commands
            .host()
            .ui()
            .executed
            .lock()
            .unwrap()
            .last()
            .unwrap(),
        &("(\"x\" \"y\")".to_string(), "1".to_string())
    );
}

#[test]
fn test_apply_macro_to_region_lines() {
    let commands = setup();
    assert_eq!(
        eval_error(&commands, "(apply-macro-to-region-lines)"),
        "No keyboard macro defined"
    );
    eval(
        &commands,
        "(insert \"a\\nb\\nc\\n\") (set! last-kbd-macro '(\"X\")) (apply-macro-to-region-lines 1 5)",
    );
    assert_eq!(
        commands
            .host()
            .ui()
            .executed
            .lock()
            .unwrap()
            .last()
            .unwrap(),
        &(
            "(\"C-a\" \"X\" \"C-n\" \"C-a\" \"X\" \"C-n\")".to_string(),
            "1".to_string()
        )
    );
    assert_eq!(eval(&commands, "(point)"), "1");
}

#[test]
fn test_kbd_macro_query() {
    let commands = setup();
    assert_eq!(
        eval_error(&commands, "(kbd-macro-query)"),
        "Not defining or executing kbd macro"
    );
    // 覚えているあいだは, 何もしない
    eval(&commands, "(kmacro-start-macro) (kbd-macro-query)");
    // マクロを動かしているあいだは, 聞く
    *commands.host().ui().executing.lock().unwrap() = true;
    *commands.host().ui().answer.lock().unwrap() = true;
    eval(&commands, "(kbd-macro-query)");
}

#[test]
fn test_step_edit_macro_asks_before_each_command() {
    let commands = setup();
    assert_eq!(
        eval_error(&commands, "(kmacro-step-edit-macro)"),
        "No keyboard macro defined"
    );
    // C-x C-f は 1 つのコマンド, a は自分を入れる 1 つのコマンド
    assert_eq!(
        eval(
            &commands,
            "(kmacro--commands '(\"C-a\" \"C-x\" \"C-f\" \"a\" \"C-x\"))"
        ),
        "((\"C-a\") (\"C-x\" \"C-f\") (\"a\") (\"C-x\"))"
    );
    let steps = |answers: &[&str]| {
        eval(
            &commands,
            "(set! last-kbd-macro '(\"C-a\" \"a\" \"C-e\" \"b\"))",
        );
        let ui = commands.host().ui();
        ui.executed.lock().unwrap().clear();
        *ui.keys.lock().unwrap() = answers.iter().map(|k| (*k).to_string()).collect();
        eval(&commands, "(kmacro-step-edit-macro)");
        ui.executed
            .lock()
            .unwrap()
            .iter()
            .map(|(keys, _)| keys.clone())
            .collect::<Vec<_>>()
            .join(",")
    };
    assert_eq!(steps(&[" ", "n", "y", "q"]), "(\"C-a\"),(\"C-e\")");
    assert_eq!(steps(&["n", "!"]), "(\"a\"),(\"C-e\"),(\"b\")");
    // 答えが尽きると止まる
    assert_eq!(steps(&[]), "");
}
