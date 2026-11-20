//! `paredit-mode` (`lisp/paredit.scm`) のテスト.

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

/// 本文と点 (`|` の所) を決める. 点を書かなければ, 終わり.
fn put(commands: &CommandLoop<Ui>, text: &str) {
    let point = text.find('|').map(|i| text[..i].chars().count() + 1);
    let body = text.replace('|', "");
    eval(
        commands,
        &format!(
            "(erase-buffer) (insert {body:?}) (set-syntax-dialect 'scheme) {}",
            point.map_or(String::new(), |p| format!("(goto-char {p})"))
        ),
    );
}

/// 本文を, 点を `|` で入れて返す.
fn shown(commands: &CommandLoop<Ui>) -> String {
    let printed = eval(commands, "(buffer-string)");
    let mut text = String::new();
    let mut chars = printed[1..printed.len() - 1].chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => text.push('\n'),
                Some(other) => text.push(other),
                None => {}
            }
        } else {
            text.push(c);
        }
    }
    let point: usize = eval(commands, "(point)").parse().unwrap();
    let mut chars: Vec<char> = text.chars().collect();
    chars.insert(point - 1, '|');
    chars.into_iter().collect()
}

/// コマンドを呼んで, 結果を返す.
fn run(commands: &CommandLoop<Ui>, text: &str, command: &str) -> String {
    put(commands, text);
    eval(commands, &format!("({command})"));
    shown(commands)
}

fn on(commands: &CommandLoop<Ui>) {
    eval(commands, "(paredit-mode 1)");
}

#[test]
fn test_paredit_mode_binds_keys_and_falls_back_when_off() {
    let commands = setup();
    put(&commands, "|");
    // 切れていれば, 文字をそのまま入れる
    eval(
        &commands,
        "(paredit-open-round) (paredit-close-round) (paredit-doublequote)",
    );
    assert_eq!(shown(&commands), "()\"|");
    put(&commands, "ab|c");
    eval(&commands, "(paredit-backward-delete)");
    assert_eq!(shown(&commands), "a|c");
    eval(&commands, "(paredit-forward-delete)");
    assert_eq!(shown(&commands), "a|");
    put(&commands, "a|bc\ndef");
    eval(&commands, "(paredit-kill)");
    assert_eq!(shown(&commands), "a|\ndef");
    // 入れると, キーを割り当てる
    assert_eq!(eval(&commands, "(paredit-mode 1)"), "#t");
    // キーを押すと, コマンドが動く
    put(&commands, "(|)");
    commands.handle_key("(");
    assert_eq!(shown(&commands), "((|))");
    assert_eq!(eval(&commands, "(paredit-mode 0)"), "#f");
}

#[test]
fn test_paredit_insert_pairs() {
    let commands = setup();
    on(&commands);
    assert_eq!(run(&commands, "|", "paredit-open-round"), "(|)");
    assert_eq!(run(&commands, "foo|", "paredit-open-round"), "foo (|)");
    assert_eq!(run(&commands, "(|", "paredit-open-round"), "((|)");
    assert_eq!(run(&commands, "\"a|\"", "paredit-open-round"), "\"a(|\"");
    assert_eq!(run(&commands, "; x|", "paredit-open-round"), "; x(|");
    assert_eq!(run(&commands, "|", "paredit-doublequote"), "\"|\"");
    assert_eq!(run(&commands, "\"ab|\"", "paredit-doublequote"), "\"ab\"|");
    assert_eq!(
        run(&commands, "\"a|b\"", "paredit-doublequote"),
        "\"a\\\"|b\""
    );
    assert_eq!(run(&commands, "; a|", "paredit-doublequote"), "; a\"|");
    assert_eq!(run(&commands, "(a |b)", "paredit-close-round"), "(a b)|");
    assert_eq!(run(&commands, "\"a|\"", "paredit-close-round"), "\"a)|\"");
    // 閉じる所が無い
    assert_eq!(run(&commands, "a|", "paredit-close-round"), "a|");
}

#[test]
fn test_paredit_delete() {
    let commands = setup();
    on(&commands);
    assert_eq!(run(&commands, "(|)", "paredit-forward-delete"), "|");
    assert_eq!(run(&commands, "|(a)", "paredit-forward-delete"), "(|a)");
    assert_eq!(run(&commands, "(a|)", "paredit-forward-delete"), "(a|)");
    assert_eq!(run(&commands, "(|)", "paredit-backward-delete"), "|");
    assert_eq!(run(&commands, "(|a)", "paredit-backward-delete"), "(|a)");
    assert_eq!(run(&commands, "(a)|", "paredit-backward-delete"), "(a|)");
    assert_eq!(run(&commands, "(a|)", "paredit-backward-delete"), "(|)");
    assert_eq!(run(&commands, "|a", "paredit-forward-delete"), "|");
    assert_eq!(run(&commands, "a|", "paredit-backward-delete"), "|");
    // 文字列
    assert_eq!(run(&commands, "|\"\"", "paredit-forward-delete"), "|");
    assert_eq!(run(&commands, "|\"a\"", "paredit-forward-delete"), "\"|a\"");
    assert_eq!(run(&commands, "\"|\"", "paredit-forward-delete"), "|");
    assert_eq!(run(&commands, "\"a|\"", "paredit-forward-delete"), "\"a|\"");
    assert_eq!(
        run(&commands, "\"a|b\"", "paredit-forward-delete"),
        "\"a|\""
    );
    assert_eq!(run(&commands, "\"|\"", "paredit-backward-delete"), "|");
    assert_eq!(
        run(&commands, "\"|a\"", "paredit-backward-delete"),
        "\"|a\""
    );
    assert_eq!(
        run(&commands, "\"ab|\"", "paredit-backward-delete"),
        "\"a|\""
    );
    assert_eq!(
        run(&commands, "\"a\"|", "paredit-backward-delete"),
        "\"a|\""
    );
    // コメントの中は, ふつうに消す
    assert_eq!(run(&commands, "; a|b", "paredit-forward-delete"), "; a|");
}

#[test]
fn test_paredit_kill() {
    let commands = setup();
    on(&commands);
    assert_eq!(run(&commands, "(a |b c)\nx", "paredit-kill"), "(a |)\nx");
    assert_eq!(run(&commands, "|(a b) (c)\nx", "paredit-kill"), "|\nx");
    assert_eq!(
        run(&commands, "(a |(b\nc) d)\nx", "paredit-kill"),
        "(a |)\nx"
    );
    assert_eq!(run(&commands, "(a |; note\nb)", "paredit-kill"), "(a |\nb)");
    assert_eq!(
        run(&commands, "(a \"b|c d\" e)", "paredit-kill"),
        "(a \"b|\" e)"
    );
    assert_eq!(
        run(&commands, "(a \"b|c\nd\" e)", "paredit-kill"),
        "(a \"b|\nd\" e)"
    );
    assert_eq!(run(&commands, "a|\nb", "paredit-kill"), "a|b");
    assert_eq!(run(&commands, "; a|b\nc", "paredit-kill"), "; a|\nc");
    // 閉じていない式は, 行の終わりまで
    assert_eq!(run(&commands, "|(a b\nc", "paredit-kill"), "|\nc");
}

#[test]
fn test_paredit_slurp_and_barf() {
    let commands = setup();
    on(&commands);
    assert_eq!(
        run(&commands, "(a |b) c d", "paredit-forward-slurp-sexp"),
        "(a |b c) d"
    );
    assert_eq!(
        run(&commands, "(a b|) c", "paredit-forward-barf-sexp"),
        "(a|) b c"
    );
    assert_eq!(
        run(&commands, "(a |b c)", "paredit-forward-barf-sexp"),
        "(a |b) c"
    );
    assert_eq!(
        run(&commands, "a (b |c)", "paredit-backward-slurp-sexp"),
        "(a b |c)"
    );
    assert_eq!(
        run(&commands, "(a |b c)", "paredit-backward-barf-sexp"),
        "a (|b c)"
    );
    put(&commands, "(a |b)");
    let error = commands
        .evaluator()
        .eval_with("(paredit-forward-slurp-sexp)", commands.host())
        .unwrap_err()
        .to_string();
    assert_eq!(error, "Nothing to slurp");
}

#[test]
fn test_paredit_splice_raise_and_wrap() {
    let commands = setup();
    on(&commands);
    assert_eq!(run(&commands, "(a |b c)", "paredit-splice-sexp"), "a |b c");
    assert_eq!(run(&commands, "x(|a)", "paredit-splice-sexp"), "x|a");
    assert_eq!(
        run(&commands, "(a (b |c) d)", "paredit-raise-sexp"),
        "(a |c d)"
    );
    assert_eq!(
        run(&commands, "(a (|b c) d)", "paredit-raise-sexp"),
        "(a |b d)"
    );
    assert_eq!(
        run(&commands, "(a |b c)", "paredit-wrap-round"),
        "(a (|b) c)"
    );
    assert_eq!(
        run(&commands, "(a |(b c))", "paredit-wrap-round"),
        "(a (|(b c)))"
    );
    // 式が無ければ, 失敗する
    put(&commands, "(a |)");
    let error = commands
        .evaluator()
        .eval_with("(paredit-wrap-round)", commands.host())
        .unwrap_err()
        .to_string();
    assert_eq!(error, "Nothing to wrap");
    put(&commands, "|a");
    let error = commands
        .evaluator()
        .eval_with("(paredit-splice-sexp)", commands.host())
        .unwrap_err()
        .to_string();
    assert_eq!(error, "Not inside a list");
    // 要素の無い所の「引っ張る」は失敗する
    put(&commands, "(|)");
    for command in ["paredit-forward-barf-sexp", "paredit-backward-barf-sexp"] {
        let error = commands
            .evaluator()
            .eval_with(&format!("({command})"), commands.host())
            .unwrap_err()
            .to_string();
        assert_eq!(error, "Nothing to barf");
    }
    put(&commands, "(|a)");
    let error = commands
        .evaluator()
        .eval_with("(paredit-backward-slurp-sexp)", commands.host())
        .unwrap_err()
        .to_string();
    assert_eq!(error, "Nothing to slurp");
}
