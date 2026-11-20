//! 整形 (`scmfmt`) のテスト.

use rseditor_core::scmfmt::{format_source, run};
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
fn test_format_fixes_indentation_spaces_and_blank_lines() {
    assert_eq!(format_source(""), "");
    assert_eq!(format_source("\n\n"), "");
    assert_eq!(
        format_source("\n\n(define (f x)   \n      (+ x 1))\n\n\n\n(g)\n\n"),
        "(define (f x)\n  (+ x 1))\n\n(g)\n"
    );
    // 閉じ括弧だけの行は, 前の行に付ける
    assert_eq!(
        format_source("(define (f)\n  (g)\n  )\n(h)"),
        "(define (f)\n  (g))\n(h)\n"
    );
    // 前の行がコメントなら, 付けない
    assert_eq!(format_source("(f 1 ; note\n)\n"), "(f 1 ; note\n   )\n");
    // 付ける前の行が空行なら (始めの閉じ括弧), そのまま
    assert_eq!(format_source(")\n"), ")\n");
    // 方言の形: with-current-buffer, unwind-protect など
    assert_eq!(
        format_source("(with-current-buffer b\n(insert \"x\"))\n(unwind-protect\n(a)\n(b))\n"),
        "(with-current-buffer b\n  (insert \"x\"))\n(unwind-protect\n    (a)\n  (b))\n"
    );
}

#[test]
fn test_format_leaves_strings_and_block_comments() {
    // 文字列の中 (行をまたぐ文字列の, 行の終わりの空白と空行) は変えない
    let source = "(define s \"a  \n\n\n   b\\\" \n c\")  \n";
    assert_eq!(
        format_source(source),
        "(define s \"a  \n\n\n   b\\\" \n c\")\n"
    );
    // ブロックのコメントの中は, 変えない
    let source = "#| a  \n\n\n   b |#  \n(f)\n";
    assert_eq!(format_source(source), "#| a  \n\n\n   b |#\n(f)\n");
    // 文字 `#\\"` と `#\\;` は, 文字列とコメントの始まりではない
    assert_eq!(
        format_source("(list #\\\" #\\; 1)  \n   (g)\n"),
        "(list #\\\" #\\; 1)\n(g)\n"
    );
    // 整っていれば, そのまま
    let tidy = "(define (f x)\n  (let loop ((i 0))\n    (when (< i x)\n      (loop (+ i 1)))))\n";
    assert_eq!(format_source(tidy), tidy);
}

#[test]
fn test_format_equals_indent_region() {
    let commands = setup();
    let source = "(define (f x)\n(cond ((a) 1)\n((b)\n2)\n(else (let loop ((i 0))\n(when (< i 3)\n(loop (+ i 1)))))))\n(do ((i 0 (+ i 1)))\n((= i 3))\n(g i))\n";
    eval(
        &commands,
        &format!(
            "(erase-buffer) (insert {source:?}) (set-syntax-dialect 'scheme) (set! indent-tabs-mode #f) (indent-region (point-min) (point-max))"
        ),
    );
    let printed = eval(&commands, "(buffer-string)");
    let indented = printed[1..printed.len() - 1].replace("\\n", "\n");
    assert_eq!(format_source(source), indented);
}

#[test]
fn test_run_checks_and_writes_files() {
    let dir = tempfile::tempdir().unwrap();
    let bad = dir.path().join("bad.scm");
    let good = dir.path().join("good.scm");
    std::fs::write(&bad, "(f\n   1)\n").unwrap();
    std::fs::write(&good, "(f\n 1)\n").unwrap();
    let paths = [
        bad.to_str().unwrap().to_string(),
        good.to_str().unwrap().to_string(),
    ];
    // --check: 書き換えない. 直す所があるファイルだけ出し, 1 を返す
    let mut out = String::new();
    let mut args = vec!["--check".to_string()];
    args.extend(paths.clone());
    assert_eq!(run(&args, &mut out), 1);
    assert_eq!(out, format!("{}\n", paths[0]));
    assert_eq!(std::fs::read_to_string(&bad).unwrap(), "(f\n   1)\n");
    // 書き換える
    let mut out = String::new();
    assert_eq!(run(&paths, &mut out), 0);
    assert_eq!(out, format!("{}\n", paths[0]));
    assert_eq!(std::fs::read_to_string(&bad).unwrap(), "(f\n 1)\n");
    // 直したあとは, 何も出さない
    let mut out = String::new();
    let mut args = vec!["--check".to_string()];
    args.extend(paths.clone());
    assert_eq!(run(&args, &mut out), 0);
    assert_eq!(out, "");
    // 読めないファイル
    let mut out = String::new();
    let missing = dir.path().join("missing.scm").to_str().unwrap().to_string();
    assert_eq!(run(std::slice::from_ref(&missing), &mut out), 2);
    assert_eq!(out, format!("{missing}: cannot read\n"));
    // 書けないファイル
    let mut permissions = std::fs::metadata(&bad).unwrap().permissions();
    permissions.set_readonly(true);
    std::fs::write(&bad, "(f\n   1)\n").ok();
    std::fs::set_permissions(&bad, permissions).unwrap();
    let mut out = String::new();
    let code = run(&paths[..1], &mut out);
    if std::fs::write(&bad, "x").is_err() {
        assert_eq!(code, 2);
        assert!(out.ends_with("cannot write\n"));
    }
}
