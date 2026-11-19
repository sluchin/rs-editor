//! 副モード (`lisp/modes.scm`) のテスト: 打った文字を変えるモードと, 画面が見るモードの名前.

use std::sync::{Arc, Mutex};

use rseditor_core::command_loop::{CommandLoop, LoopUi};
use rseditor_core::editor::Editor;
use rseditor_core::frame::FrameBuilder;
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

fn type_text(commands: &mut CommandLoop<Ui>, text: &str) {
    for c in text.chars() {
        commands.self_insert(&c.to_string());
    }
}

fn buffer_text(commands: &CommandLoop<Ui>) -> String {
    commands.host().editor().current_buffer().text()
}

#[test]
fn test_overwrite_mode_replaces_the_characters_after_point() {
    let mut commands = setup();
    eval(&commands, "(insert \"abcd\\nxy\") (goto-char 2)");
    eval(&commands, "(overwrite-mode 1)");
    type_text(&mut commands, "XYZ");
    assert_eq!(buffer_text(&commands), "aXYZ\nxy");
    // 行の終わりを越えて, 次の行は消さない
    type_text(&mut commands, "Q");
    assert_eq!(buffer_text(&commands), "aXYZQ\nxy");
    assert_eq!(eval(&commands, "(buffer-minor-mode-p 'overwrite)"), "#t");
    // 切ると, 普通に挿入する
    eval(&commands, "(overwrite-mode 0) (goto-char 1)");
    type_text(&mut commands, "_");
    assert_eq!(buffer_text(&commands), "_aXYZQ\nxy");
    // 引数が無ければ切り替える
    eval(&commands, "(overwrite-mode)");
    assert_eq!(eval(&commands, "(buffer-minor-mode-p 'overwrite)"), "#t");
}

#[test]
fn test_electric_pair_mode_inserts_the_closing_character_and_skips_over_it() {
    let mut commands = setup();
    eval(&commands, "(electric-pair-mode 1)");
    type_text(&mut commands, "(a");
    assert_eq!(buffer_text(&commands), "(a)");
    // 閉じる括弧を打つと, 越える
    type_text(&mut commands, ")");
    assert_eq!(buffer_text(&commands), "(a)");
    assert_eq!(eval(&commands, "(point)"), "4");
    // 引用符も同じ. 語の後ろの引用符は, 対にしない
    type_text(&mut commands, " \"b");
    assert_eq!(buffer_text(&commands), "(a) \"b\"");
    type_text(&mut commands, "\"");
    assert_eq!(buffer_text(&commands), "(a) \"b\"");
    type_text(&mut commands, "d\"");
    assert_eq!(buffer_text(&commands), "(a) \"b\"d\"");
    // 次が文字なら, 対にしない
    eval(&commands, "(goto-char 1)");
    type_text(&mut commands, "[");
    assert_eq!(buffer_text(&commands), "[(a) \"b\"d\"");
    // 切ると何もしない
    eval(&commands, "(electric-pair-mode 0) (erase-buffer)");
    type_text(&mut commands, "(");
    assert_eq!(buffer_text(&commands), "(");
}

#[test]
fn test_auto_fill_mode_breaks_the_line_at_the_fill_column() {
    let mut commands = setup();
    eval(&commands, "(set-fill-column 10) (auto-fill-mode 1)");
    type_text(&mut commands, "aaa bbb ccc ddd ");
    assert_eq!(buffer_text(&commands), "aaa bbb\nccc ddd ");
    // 字下げは, 前の行に合わせる
    eval(&commands, "(erase-buffer)");
    type_text(&mut commands, "  aaa bbb ccc ");
    assert_eq!(buffer_text(&commands), "  aaa bbb\n  ccc ");
    // 長い 1 語は, その後ろで折る
    eval(&commands, "(erase-buffer)");
    type_text(&mut commands, "abcdefghijklmnop q ");
    assert_eq!(buffer_text(&commands), "abcdefghijklmnop\nq ");
    // 後ろに続きが無い 1 語は, 折らない
    eval(&commands, "(erase-buffer)");
    type_text(&mut commands, "abcdefghijklmnop ");
    assert_eq!(buffer_text(&commands), "abcdefghijklmnop ");
    // fill-prefix があれば, それを入れる
    eval(&commands, "(erase-buffer) (set! fill-prefix \"> \")");
    type_text(&mut commands, "aaa bbb ccc ");
    assert_eq!(buffer_text(&commands), "aaa bbb\n> ccc ");
    // 切ると折らない
    eval(
        &commands,
        "(set! fill-prefix #f) (auto-fill-mode 0) (erase-buffer)",
    );
    type_text(&mut commands, "aaa bbb ccc ddd ");
    assert_eq!(buffer_text(&commands), "aaa bbb ccc ddd ");
}

#[test]
fn test_minor_mode_commands_report_modes_to_the_frame() {
    let commands = setup();
    let modes = |commands: &CommandLoop<Ui>| {
        let mut builder = FrameBuilder::default();
        let mut editor_guard = commands.host().editor();
        let frame = builder.build(&mut editor_guard, "");
        frame.buffers[0].minor_modes.clone()
    };
    assert_eq!(
        modes(&commands),
        vec![
            "column-number",
            "electric-indent",
            "line-number",
            "show-paren"
        ]
    );
    eval(
        &commands,
        "(visual-line-mode) (display-line-numbers-mode 1) (hl-line-mode 'toggle) (column-number-mode 0)",
    );
    assert_eq!(
        modes(&commands),
        vec![
            "display-line-numbers",
            "electric-indent",
            "hl-line",
            "line-number",
            "show-paren",
            "visual-line"
        ]
    );
    // 全てのバッファのモード
    eval(&commands, "(visual-line-mode 0) (global-visual-line-mode 1) (global-hl-line-mode 1) (global-display-line-numbers-mode 1) (line-number-mode 0) (show-paren-mode 0) (electric-indent-mode 0)");
    assert_eq!(
        modes(&commands),
        vec!["display-line-numbers", "hl-line", "visual-line"]
    );
    assert_eq!(eval(&commands, "(global-minor-mode-p 'hl-line)"), "#t");
}
