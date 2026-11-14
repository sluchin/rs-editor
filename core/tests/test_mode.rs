//! メジャーモードの情報 (`core/src/mode.rs`) と, ファイルの変数, `auto-mode-alist` と `set-auto-mode` のテスト.

use rseditor_core::editor::Editor;
use rseditor_core::mode::{file_variables, FileVariables};
use rseditor_core::scheme::editor_host::EditorHost;
use rseditor_core::scheme::evaluator::Evaluator;
use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::value::Value;

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

/// 基本のコマンドとファイルのコマンドを読んだ評価器とエディタ.
fn setup() -> (Evaluator, EditorHost<NoUi>) {
    let ev = Evaluator::new();
    let host = EditorHost::new(Editor::new(), NoUi);
    ev.eval_with(include_str!("../../lisp/simple.scm"), &host)
        .unwrap();
    ev.eval_with(include_str!("../../lisp/files.scm"), &host)
        .unwrap();
    (ev, host)
}

/// ファイルを訪れたバッファにして, `set-auto-mode` でモードを決める.
fn visit(ev: &Evaluator, host: &EditorHost<NoUi>, file: &str, text: &str) {
    host.editor().current_buffer_mut().load(text);
    ev.eval_with(
        &format!(r#"(files--set-buffer-file-name "{file}") (set-auto-mode)"#),
        host,
    )
    .unwrap_or_else(|e| panic!("{file}: {e}"));
}

#[test]
fn test_file_variables() {
    assert_eq!(
        file_variables(
            ";; -*- mode: scheme; lexical-binding: t; indent-tabs-mode: nil; tab-width: 4 -*-",
            ""
        ),
        FileVariables {
            mode: Some("scheme".to_string()),
            lexical_binding: Some(true),
            indent_tabs_mode: Some(false),
            tab_width: Some(4),
        }
    );
    // 名前だけならモード. `nil` は偽
    assert_eq!(
        file_variables("-*- lisp-data -*-", "").mode.as_deref(),
        Some("lisp-data")
    );
    assert_eq!(
        file_variables("-*- lexical-binding: nil -*-", "").lexical_binding,
        Some(false)
    );
    // `#!` の行なら, 2 行目
    assert_eq!(
        file_variables("#!/usr/bin/env guile", ";; -*- tab-width: 2 -*-").tab_width,
        Some(2)
    );
    assert_eq!(
        file_variables("#!/bin/sh", "-*- c -*-").mode.as_deref(),
        Some("c")
    );
    // 印が無い, 閉じない, 知らない変数, タブの幅が数でないか 0 から始まる
    assert_eq!(file_variables("plain", ""), FileVariables::default());
    assert_eq!(file_variables("-*- mode: c", ""), FileVariables::default());
    assert_eq!(
        file_variables("-*- other: 1; tab-width: x -*-", ""),
        FileVariables::default()
    );
    assert_eq!(
        file_variables("-*- tab-width: 0 -*-", ""),
        FileVariables::default()
    );
    // `:` の無い項目は, キーだけ
    assert_eq!(
        file_variables("-*- a: 1; b -*-", ""),
        FileVariables::default()
    );
}

#[test]
fn test_mode_info_names() {
    let (ev, host) = setup();
    let name = |host: &EditorHost<NoUi>| host.editor().current_buffer().mode_info().name;
    // `*scratch*` は Lisp Interaction で, 括弧は Scheme で読む
    let info = host.editor().current_buffer().mode_info();
    assert_eq!(
        (
            info.name.as_str(),
            info.language,
            info.indent_tabs,
            info.tab_width
        ),
        ("Lisp Interaction", Some("scheme"), true, 8)
    );
    for (command, expected) in [
        ("(fundamental-mode)", "Fundamental"),
        ("(text-mode)", "Text"),
        ("(c-mode)", "C"),
        ("(c++-mode)", "C++"),
        ("(rust-mode)", "Rust"),
        ("(typescript-mode)", "TypeScript"),
        ("(scheme-mode)", "Scheme"),
        ("(emacs-lisp-mode)", "ELisp/d"),
        ("(lisp-data-mode)", "Lisp-Data"),
        ("(set-major-mode-name 'org-mode)", "Org"),
    ] {
        ev.eval_with(command, &host).unwrap();
        assert_eq!(name(&host), expected, "{command}");
    }
    // Lisp の方言のモードだけが, 括弧の読み方を持つ
    ev.eval_with("(emacs-lisp-mode)", &host).unwrap();
    assert_eq!(
        host.editor().current_buffer().mode_info().language,
        Some("elisp")
    );
    ev.eval_with("(c-mode)", &host).unwrap();
    assert_eq!(host.editor().current_buffer().mode_info().language, None);
}

#[test]
fn test_mode_info_of_special_buffers_and_file_variables() {
    let (ev, host) = setup();
    let info = |host: &EditorHost<NoUi>| host.editor().current_buffer().mode_info();
    // dired
    host.editor()
        .current_buffer_mut()
        .set_directory(Some("/tmp".to_string()));
    assert_eq!(info(&host).name, "Dired by name");
    host.editor().current_buffer_mut().set_directory(None);
    // ヘルプと grep
    for (buffer, expected) in [("*Help*", "Help"), ("*grep*", "Grep")] {
        ev.eval_with(
            &format!(r#"(set-buffer (generate-new-buffer "{buffer}"))"#),
            &host,
        )
        .unwrap();
        assert_eq!(info(&host).name, expected);
    }
    // ファイルの変数 (字句の束縛, 字下げ, タブの幅) は, ファイルを訪れたバッファだけ
    ev.eval_with(r#"(set-buffer (generate-new-buffer "f.el"))"#, &host)
        .unwrap();
    visit(
        &ev,
        &host,
        "/tmp/f.el",
        ";;; -*- lexical-binding: t; indent-tabs-mode: nil; tab-width: 4 -*-\n",
    );
    let found = info(&host);
    assert_eq!(
        (found.name.as_str(), found.indent_tabs, found.tab_width),
        ("ELisp/l", false, 4)
    );
    host.editor().current_buffer_mut().set_file(None);
    let plain = info(&host);
    assert_eq!(
        (plain.name.as_str(), plain.indent_tabs, plain.tab_width),
        ("ELisp/d", true, 8)
    );
}

#[test]
fn test_set_auto_mode_uses_the_alist_and_the_file_variable() {
    let (ev, host) = setup();
    let name = |host: &EditorHost<NoUi>| host.editor().current_buffer().mode_info().name;
    for (file, expected) in [
        ("/tmp/a.el", "ELisp/d"),
        ("/tmp/.dir-locals.el", "Lisp-Data"),
        ("/tmp/x.eld", "Lisp-Data"),
        ("/tmp/places", "Lisp-Data"),
        ("/tmp/Project.ede", "ELisp/d"),
        ("/tmp/.emacs", "ELisp/d"),
        ("/tmp/_emacs", "ELisp/d"),
        ("/tmp/a.scm", "Scheme"),
        ("/tmp/a.sld", "Scheme"),
        ("/tmp/.guile", "Scheme"),
        ("/tmp/a.scm.1", "Scheme"),
        ("/tmp/a.oak", "Scheme"),
        ("/tmp/a.txt", "Text"),
        ("/tmp/a.c", "C"),
        ("/tmp/a.cpp", "C++"),
        ("/tmp/a.rs", "Rust"),
        ("/tmp/a.ts", "TypeScript"),
        ("/tmp/a.unknown", "Fundamental"),
    ] {
        visit(&ev, &host, file, "");
        assert_eq!(name(&host), expected, "{file}");
    }
    // 1 行目の `mode:` は, ファイルの名前より先
    for (first, expected) in [
        ("-*- mode: emacs-lisp -*-", "ELisp/d"),
        ("-*- mode: lisp-data -*-", "Lisp-Data"),
        ("-*- scheme -*-", "Scheme"),
        ("-*- mode: c -*-", "C"),
        ("-*- mode: c++ -*-", "C++"),
        ("-*- mode: fundamental -*-", "Fundamental"),
        // 知らないモードは, ファイルの名前で決める
        ("-*- mode: unknown -*-", "Text"),
    ] {
        visit(&ev, &host, "/tmp/a.txt", first);
        assert_eq!(name(&host), expected, "{first}");
    }
    // ファイルの無いバッファは, Fundamental
    ev.eval_with("(files--set-buffer-file-name #f) (set-auto-mode)", &host)
        .unwrap();
    assert_eq!(name(&host), "Fundamental");
}
