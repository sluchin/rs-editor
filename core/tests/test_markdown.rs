//! `markdown-mode` (`lisp/markdown.scm`) のテスト.

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

const DOC: &str = "# One\ntext\n## A\nbody\n### A1\n## B\n# Two\n";

/// 本文を入れて, 点を行の先頭 (1 から数える) に置く.
fn open(commands: &CommandLoop<Ui>, line: usize) {
    eval(
        commands,
        &format!(
            "(erase-buffer) (insert {DOC:?}) (goto-char (point-min)) (forward-line {})",
            line - 1
        ),
    );
}

/// 点のある行の本文.
fn line(commands: &CommandLoop<Ui>) -> String {
    eval(
        commands,
        "(buffer-substring (line-beginning-position) (line-end-position))",
    )
    .trim_matches('"')
    .to_string()
}

#[test]
fn test_markdown_mode_sets_the_mode_keys_and_highlight() {
    let commands = setup();
    eval(&commands, "(markdown-mode)");
    assert_eq!(eval(&commands, "(major-mode-name)"), "markdown-mode");
    eval(&commands, "(insert \"# Title\\n\")");
    commands.handle_key("C-c");
    commands.handle_key("C-p");
    assert_eq!(eval(&commands, "(point)"), "1");
    // find-file-hook は, 拡張子で決める
    eval(
        &commands,
        "(erase-buffer) (set-major-mode-name 'fundamental)",
    );
    eval(&commands, "(markdown--find-file-hook)");
    assert_eq!(eval(&commands, "(major-mode-name)"), "fundamental");
    let dir = tempfile::tempdir().unwrap();
    for (name, markdown) in [("a.md", true), ("a.markdown", true), ("a.txt", false)] {
        let path = dir.path().join(name);
        std::fs::write(&path, "# T\n").unwrap();
        eval(
            &commands,
            &format!("(find-file {:?})", path.to_str().unwrap()),
        );
        assert_eq!(
            eval(&commands, "(major-mode-name)") == "markdown-mode",
            markdown,
            "{name}"
        );
    }
}

#[test]
fn test_markdown_outline_movement() {
    let commands = setup();
    eval(&commands, "(markdown-mode)");
    open(&commands, 1);
    eval(&commands, "(markdown-outline-next)");
    assert_eq!(line(&commands), "## A");
    eval(&commands, "(markdown-outline-next)");
    assert_eq!(line(&commands), "### A1");
    eval(&commands, "(markdown-outline-previous)");
    assert_eq!(line(&commands), "## A");
    open(&commands, 7);
    assert_eq!(
        eval_error(&commands, "(markdown-outline-next)"),
        "No more headings"
    );
    open(&commands, 1);
    assert_eq!(
        eval_error(&commands, "(markdown-outline-previous)"),
        "No more headings"
    );
    // 上の見出し
    open(&commands, 5);
    eval(&commands, "(markdown-outline-up)");
    assert_eq!(line(&commands), "## A");
    eval(&commands, "(markdown-outline-up)");
    assert_eq!(line(&commands), "# One");
    assert_eq!(
        eval_error(&commands, "(markdown-outline-up)"),
        "Already at top level"
    );
    // 見出しでない行からでも, その節の見出しを元にする
    open(&commands, 4);
    eval(&commands, "(markdown-outline-up)");
    assert_eq!(line(&commands), "# One");
    eval(&commands, "(erase-buffer) (insert \"no heading\\n\")");
    assert_eq!(
        eval_error(&commands, "(markdown-outline-up)"),
        "Not in a section"
    );
    assert_eq!(
        eval_error(&commands, "(markdown-forward-same-level)"),
        "Not in a section"
    );
}

#[test]
fn test_markdown_same_level_movement() {
    let commands = setup();
    eval(&commands, "(markdown-mode)");
    open(&commands, 3);
    eval(&commands, "(markdown-forward-same-level)");
    assert_eq!(line(&commands), "## B");
    eval(&commands, "(markdown-backward-same-level)");
    assert_eq!(line(&commands), "## A");
    // 同じ節の中に無ければ, 動かない
    open(&commands, 6);
    assert_eq!(
        eval_error(&commands, "(markdown-forward-same-level)"),
        "No more headings at this level"
    );
    assert_eq!(line(&commands), "## B");
    open(&commands, 1);
    eval(&commands, "(markdown-forward-same-level)");
    assert_eq!(line(&commands), "# Two");
    // 浅い見出しの先は, 越えない
    open(&commands, 5);
    assert_eq!(
        eval_error(&commands, "(markdown-forward-same-level)"),
        "No more headings at this level"
    );
    // 本文の行からは, その節の見出しから数える
    open(&commands, 4);
    eval(&commands, "(markdown-forward-same-level)");
    assert_eq!(line(&commands), "## B");
    // 見出しが無くなる
    open(&commands, 7);
    assert_eq!(
        eval_error(&commands, "(markdown-forward-same-level)"),
        "No more headings"
    );
    // 深い見出しにしか当たらないときは, 動かない
    eval(
        &commands,
        "(erase-buffer) (insert \"## a\\n### b\\n\") (goto-char 1)",
    );
    assert_eq!(
        eval_error(&commands, "(markdown-forward-same-level)"),
        "No more headings"
    );
}

#[test]
fn test_markdown_promote_and_demote() {
    let commands = setup();
    eval(&commands, "(markdown-mode)");
    open(&commands, 3);
    eval(&commands, "(markdown-promote)");
    assert_eq!(line(&commands), "# A");
    assert_eq!(
        eval_error(&commands, "(markdown-promote)"),
        "Already at the top level"
    );
    eval(&commands, "(markdown-demote)");
    assert_eq!(line(&commands), "## A");
    open(&commands, 2);
    assert_eq!(
        eval_error(&commands, "(markdown-promote)"),
        "Not on a heading"
    );
    assert_eq!(
        eval_error(&commands, "(markdown-demote)"),
        "Not on a heading"
    );
    eval(&commands, "(erase-buffer) (insert \"###### x\")");
    eval(&commands, "(beginning-of-line)");
    assert_eq!(
        eval_error(&commands, "(markdown-demote)"),
        "Already at the lowest level"
    );
}
