//! ヘルプ (`lisp/help-fns.scm` の `describe-function` と, `src/scheme/help.rs` の組み込みの手続き) のテスト.

use rseditor_core::scheme::evaluator::Evaluator;
use tempfile::tempdir;

mod common;
use common::FakeEditor;

/// 言語を英語にした評価器と, 偽のエディタを作る.
///
/// # Returns
///
/// 評価器と, 偽のエディタ.
fn setup() -> (Evaluator, FakeEditor) {
    let ev = Evaluator::new();
    ev.eval("(set! help-language \"en\")").unwrap();
    (ev, FakeEditor::new())
}

/// 関数のヘルプを出して, *Help* の本文を返す.
///
/// # Arguments
///
/// * `ev` - 評価器.
/// * `editor` - 偽のエディタ.
/// * `function` - 関数の名前.
///
/// # Returns
///
/// *Help* の本文.
fn describe(ev: &Evaluator, editor: &FakeEditor, function: &str) -> String {
    ev.eval_with(&format!("(describe-function '{})", function), editor)
        .unwrap();
    editor.buffers.lock().unwrap()["*Help*"]
        .text
        .iter()
        .collect()
}

#[test]
fn test_describe_function_interactively() {
    let (ev, editor) = setup();
    let before = editor.name();
    editor.answers.lock().unwrap().push("find-file".to_string());
    ev.run_command("describe-function", &editor).unwrap();
    assert_eq!(*editor.prompts.lock().unwrap(), vec!["Describe function: "]);
    // 候補は, Lisp と Rust の関数と, 画面のコマンド (重ならず, 名前の順)
    let completions = editor.completions.lock().unwrap().clone();
    for name in ["car", "describe-function", "find-file", "list-buffers"] {
        assert_eq!(
            completions.iter().filter(|c| *c == name).count(),
            1,
            "{}",
            name
        );
    }
    let mut sorted = completions.clone();
    sorted.sort();
    assert_eq!(completions, sorted);
    // 変数は, 候補でない
    assert!(!completions.contains(&"help-language".to_string()));
    // *Help* を, ほかのウィンドウに出し, 今のバッファはそのまま
    assert_eq!(editor.name(), before);
    assert_eq!(
        *editor.displayed.lock().unwrap(),
        vec![("*Help*".to_string(), Some(1))]
    );
    let help: String = editor.buffers.lock().unwrap()["*Help*"]
        .text
        .iter()
        .collect();
    assert_eq!(
        help,
        "find-file is an interactive Lisp function in `files.scm'.\n\n\
         It is bound to C-x C-f.\n\n\
         (find-file &rest ARGS)\n\n\
         Edit file FILENAME.\n\
         Switch to a buffer visiting file FILENAME, creating one if none already exists.\n\
         Interactively, read FILENAME in the minibuffer with file name completion.\n"
    );
    assert!(!editor.buffers.lock().unwrap()["*Help*"].modified);
}

#[test]
fn test_describe_function_kinds() {
    let (ev, editor) = setup();
    // 画面 (TypeScript) のコマンド
    assert_eq!(
        describe(&ev, &editor, "list-buffers"),
        "list-buffers is an interactive built-in command.\n\n\
         It is bound to C-x C-b.\n\n\
         Not documented.\n"
    );
    // 組み込みの手続き (Rust)
    assert_eq!(
        describe(&ev, &editor, "car"),
        "car is a built-in function.\n\nNot documented.\n"
    );
    // ファイルから読んでいない, Lisp の関数 (前の *Help* は, 書き直す)
    ev.eval("(define (f x . more) \"Do X.\" x) (define (g) 1)")
        .unwrap();
    assert_eq!(
        describe(&ev, &editor, "f"),
        "f is a Lisp function.\n\n(f X &rest MORE)\n\nDo X.\n"
    );
    assert_eq!(
        describe(&ev, &editor, "g"),
        "g is a Lisp function.\n\n(g)\n\nNot documented.\n"
    );
    assert_eq!(
        describe(&ev, &editor, "nothing"),
        "nothing is not defined as a function.\n\nNot documented.\n"
    );
}

#[test]
fn test_describe_function_in_the_help_language() {
    let (ev, editor) = setup();
    ev.eval("(set! help-language \"ja\")").unwrap();
    let help = describe(&ev, &editor, "find-file");
    assert!(help.ends_with(
        "(find-file &rest ARGS)\n\n\
         ファイル FILENAME を編集する.\n\
         FILENAME のファイルのバッファに切り替える (無ければ, 作る).\n\
         対話的に呼ぶと, FILENAME をミニバッファで (ファイルの名前を補完しながら) 聞く.\n"
    ));
    // 翻訳の無い言語は, 元の英語
    ev.eval("(set! help-language \"fr\")").unwrap();
    assert!(describe(&ev, &editor, "find-file").ends_with("file name completion.\n"));
}

#[test]
fn test_help_language_from_locale() {
    let (ev, _) = setup();
    for (locale, language) in [
        ("\"ja_JP.UTF-8\"", "ja"),
        ("\"de_DE\"", "de"),
        ("\"fr\"", "fr"),
        ("\"C.UTF-8\"", "en"),
        ("\"C\"", "en"),
        ("\"POSIX\"", "en"),
        ("\"\"", "en"),
        ("#f", "en"),
    ] {
        assert_eq!(
            ev.eval(&format!("(help-language-from-locale {})", locale))
                .unwrap(),
            format!("{:?}", language),
            "{}",
            locale
        );
    }
    // 変数の docstring
    assert!(ev
        .eval("(documentation-property 'help-language 'variable-documentation)")
        .unwrap()
        .starts_with("\"Language of the docstrings"));
}

#[test]
fn test_help_builtins() {
    let (ev, editor) = setup();
    // documentation: 無名の手続きは, 訳さない. 組み込みの手続きは #f
    assert_eq!(
        ev.eval("(documentation (lambda () \"Doc.\" 1))").unwrap(),
        "\"Doc.\""
    );
    assert_eq!(ev.eval("(documentation car)").unwrap(), "#f");
    assert_eq!(
        ev.eval("(documentation 'car 1 2)").unwrap_err(),
        "documentation expects 1 or 2 arguments"
    );
    assert_eq!(
        ev.eval("(documentation-property 'x)").unwrap_err(),
        "documentation-property expects 2 or 3 arguments"
    );
    // help-language が文字列でなければ, 訳さない
    ev.eval("(set! help-language #f)").unwrap();
    assert_eq!(
        ev.eval("(documentation-property 'howm-directory 'variable-documentation)")
            .unwrap(),
        "#f"
    );
    assert!(ev
        .eval("(documentation 'find-file)")
        .unwrap()
        .starts_with("\"Edit file FILENAME."));
    ev.eval("(set! help-language \"en\")").unwrap();
    // help-function-arglist
    assert_eq!(
        ev.eval("(help-function-arglist (lambda (a . b) a))")
            .unwrap(),
        "(a &rest b)"
    );
    assert_eq!(ev.eval("(help-function-arglist 'car)").unwrap(), "#t");
    assert_eq!(
        ev.eval("(help-function-arglist 1)").unwrap_err(),
        "Invalid function: 1"
    );
    assert_eq!(
        ev.eval("(help-function-arglist 'nothing)").unwrap_err(),
        "Symbol's function definition is void: nothing"
    );
    // commandp
    assert_eq!(
        ev.eval("(list (commandp 'find-file) (commandp find-file) (commandp 'car) (commandp 1))")
            .unwrap(),
        "(#t #t #f #f)"
    );
    // fboundp
    assert_eq!(
        ev.eval("(define v 1) (list (fboundp 'car) (fboundp 'find-file) (fboundp 'v) (fboundp 'nothing))")
            .unwrap(),
        "v\n(#t #t #f #f)"
    );
    assert_eq!(
        ev.eval("(fboundp \"car\")").unwrap_err(),
        "fboundp expects a symbol"
    );
    // symbol-file: 起動のときに読む Lisp は lisp/ からのパス. load したファイルは, そのパス
    assert_eq!(
        ev.eval("(list (symbol-file 'find-file) (symbol-file 'car 'defun))")
            .unwrap(),
        "(\"lisp/files.scm\" #f)"
    );
    let dir = tempdir().unwrap();
    let file = dir.path().join("lib.scm");
    std::fs::write(&file, "(define (lib-f) (define inner 1) inner) (lib-f)").unwrap();
    ev.eval(&format!("(load {:?})", file.to_string_lossy()))
        .unwrap();
    assert_eq!(
        ev.eval("(list (symbol-file 'lib-f) (symbol-file 'inner))")
            .unwrap(),
        format!("({:?} #f)", file.to_string_lossy())
    );
    assert_eq!(
        ev.eval("(symbol-file \"x\")").unwrap_err(),
        "symbol-file expects a symbol"
    );
    // mapatoms
    assert_eq!(
        ev.eval("(let ((n 0)) (mapatoms (lambda (s) (when (eq? s 'car) (set! n (+ n 1))))) n)")
            .unwrap(),
        "1"
    );
    // getenv
    std::env::set_var("RSEDITOR_HELP_TEST", "yes");
    assert_eq!(
        ev.eval("(list (getenv \"RSEDITOR_HELP_TEST\") (getenv \"RSEDITOR_NO_SUCH_VARIABLE\"))")
            .unwrap(),
        "(\"yes\" #f)"
    );
    // エディタに聞く手続き
    assert_eq!(
        ev.eval_with("(where-is-internal 'list-buffers)", &editor)
            .unwrap(),
        "(\"C-x C-b\")"
    );
    assert_eq!(
        ev.eval("(where-is-internal \"save-buffer\")").unwrap_err(),
        "where-is-internal expects a symbol"
    );
    assert_eq!(
        ev.eval("(editor-command-names 1)").unwrap_err(),
        "editor-command-names expects 0 arguments, given 1"
    );
}
