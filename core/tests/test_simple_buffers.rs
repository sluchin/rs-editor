//! シェルのコマンドと, バッファの名前, 読み取り専用, 複製のコマンド (`lisp/simple.scm`, `lisp/files.scm`) のテスト.
//! 本文に出るものの期待値は, `emacs -Q --batch` の結果 (`core/tests/edit.json`. `core/tests/test_edit.rs` で比べる).
//! ここは, Emacs の試験に入れないもの (名前, ファイル, 非同期, 問い) を試す.

use std::collections::VecDeque;
use std::fs;
use std::sync::Mutex;

use rseditor_core::editor::Editor;
use rseditor_core::scheme::editor_host::EditorHost;
use rseditor_core::scheme::evaluator::Evaluator;
use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::value::Value;
use tempfile::tempdir;

/// 答えを並べて, メッセージを覚える画面の偽物.
#[derive(Default)]
struct Ui {
    messages: Mutex<Vec<String>>,
    answers: Mutex<VecDeque<String>>,
}

impl Host for Ui {
    fn call(&self, op: &str, args: Vec<Value>) -> Result<Value, String> {
        match op {
            "message" => {
                if let Some(Value::String(text)) = args.first() {
                    self.messages.lock().unwrap().push(text.clone());
                }
                Ok(Value::Nil)
            }
            "read-string" | "read-key" | "read-file-name" => self
                .answers
                .lock()
                .unwrap()
                .pop_front()
                .map(Value::String)
                .ok_or_else(|| "no answer".to_string()),
            other => Err(format!("ui: {other}")),
        }
    }
}

/// 式を評価して, 最後の行と, 出したメッセージを返す (失敗は `error: 理由`).
fn eval_with(answers: &[&str], code: &str) -> (String, Vec<String>) {
    let ev = Evaluator::new();
    let host = EditorHost::new(Editor::new(), Ui::default());
    host.ui()
        .answers
        .lock()
        .unwrap()
        .extend(answers.iter().map(|a| a.to_string()));
    for source in [
        include_str!("../../lisp/simple.scm"),
        include_str!("../../lisp/files.scm"),
    ] {
        ev.eval_with(source, &host).unwrap();
    }
    let result = match ev.eval_with(code, &host) {
        Ok(value) => value.lines().last().unwrap_or("").to_string(),
        Err(message) => format!("error: {message}"),
    };
    let messages = host.ui().messages.lock().unwrap().clone();
    (result, messages)
}

/// 式を評価して, 最後の行を返す.
fn eval(code: &str) -> String {
    eval_with(&[], code).0
}

#[test]
fn test_rename_buffer() {
    assert_eq!(eval("(rename-buffer \"foo\")"), "\"foo\"");
    // 使われていれば失敗. unique なら <2>
    assert_eq!(
        eval("(get-buffer-create \"foo\") (rename-buffer \"foo\")"),
        "error: Buffer name ‘foo’ is in use"
    );
    assert_eq!(
        eval("(get-buffer-create \"foo\") (rename-buffer \"foo\" #t)"),
        "\"foo<2>\""
    );
    // 対話では, 名前を読み, 前置引数があれば unique
    assert_eq!(eval_with(&["bar"], "(rename-buffer)").0, "\"bar\"");
    assert_eq!(
        eval("(generate-new-buffer-name \"*scratch*\")"),
        "\"*scratch*<2>\""
    );
    assert_eq!(
        eval("(generate-new-buffer-name \"*scratch*\" \"*scratch*\")"),
        "\"*scratch*\""
    );
    assert_eq!(
        eval("(generate-new-buffer-name \"*scratch*\" #f)"),
        "\"*scratch*<2>\""
    );
    // 数の付いた名前を外して, 空いた名前にする
    assert_eq!(
        eval("(rename-buffer \"w<3>\") (rename-uniquely) (buffer-name)"),
        "\"w\""
    );
    assert_eq!(
        eval("(rename-buffer \"w\") (get-buffer-create \"w<3>\") (rename-uniquely) (buffer-name)"),
        "\"w<2>\""
    );
    // ファイルの名前に数が付いているときは, 外さない
    let dir = tempdir().unwrap();
    let path = dir.path().join("a<2>");
    fs::write(&path, "x").unwrap();
    assert_eq!(
        eval(&format!(
            "(find-file {:?}) (rename-uniquely) (buffer-name)",
            path.display().to_string()
        )),
        "\"a<2><2>\""
    );
}

#[test]
fn test_read_only_mode() {
    assert_eq!(eval("(read-only-mode 1) (buffer-read-only)"), "#t");
    assert_eq!(
        eval("(read-only-mode 1) (read-only-mode 0) (buffer-read-only)"),
        "#f"
    );
    assert_eq!(
        eval("(read-only-mode 1) (read-only-mode -1) (buffer-read-only)"),
        "#f"
    );
    assert_eq!(eval("(read-only-mode 'toggle) (buffer-read-only)"), "#t");
    // 引数が偽 (nil) なら, 読み取り専用にする
    assert_eq!(eval("(read-only-mode #f) (buffer-read-only)"), "#t");
    assert_eq!(
        eval("(read-only-mode 1) (insert \"x\")"),
        "error: Buffer is read-only: #<buffer *scratch*>"
    );
    // 対話では前置引数 (無ければ切り替える). メッセージを出す
    let (result, messages) = eval_with(&[], "(read-only-mode) (read-only-mode)");
    assert_eq!(result, "#f");
    assert_eq!(
        messages,
        vec![
            "Read-Only mode enabled in current buffer",
            "Read-Only mode disabled in current buffer"
        ]
    );
    assert_eq!(
        eval("(set-current-prefix-arg '(4)) (read-only-mode) (buffer-read-only)"),
        "#t"
    );
    assert_eq!(
        eval("(set-current-prefix-arg '-) (read-only-mode 1) (read-only-mode) (buffer-read-only)"),
        "#f"
    );
    assert_eq!(
        eval("(set-current-prefix-arg 0) (read-only-mode 1) (read-only-mode) (buffer-read-only)"),
        "#f"
    );
    // 読み取り専用でも, 読み直しの書き換えはできる
    let dir = tempdir().unwrap();
    let path = dir.path().join("f.txt");
    fs::write(&path, "one").unwrap();
    let file = path.display().to_string();
    assert_eq!(
        eval(&format!(
            "(find-file {file:?}) (read-only-mode 1) (write-file-string {file:?} \"two\") (revert-buffer #t #t) (buffer-string)"
        )),
        "\"two\""
    );
}

#[test]
fn test_find_file_read_only_and_revert_quick() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("f.txt");
    fs::write(&path, "one").unwrap();
    let file = path.display().to_string();
    assert_eq!(
        eval(&format!(
            "(find-file-read-only {file:?}) (list (buffer-read-only) (buffer-string))"
        )),
        "(#t \"one\")"
    );
    assert_eq!(
        eval(&format!(
            "(find-file-read-only-other-window {file:?}) (list (buffer-read-only) (length (window-list)))"
        )),
        "(#t 2)"
    );
    // 対話では, ファイル名を読む
    assert_eq!(
        eval_with(&[&file], "(find-file-read-only) (buffer-read-only)").0,
        "#t"
    );
    assert_eq!(
        eval_with(
            &[&file],
            "(find-file-read-only-other-window) (buffer-read-only)"
        )
        .0,
        "#t"
    );
    // 無いファイルは失敗
    let missing = dir.path().join("none").display().to_string();
    assert_eq!(
        eval(&format!("(find-file-read-only {missing:?})")),
        format!("error: {missing} does not exist")
    );
    // revert-buffer-quick: 書き換えていなければ尋ねずに読み直す
    fs::write(&path, "two").unwrap();
    let (result, messages) = eval_with(
        &[],
        &format!("(find-file {file:?}) (write-file-string {file:?} \"three\") (revert-buffer-quick) (buffer-string)"),
    );
    assert_eq!(result, "\"three\"");
    assert_eq!(messages.last().unwrap(), "Reverted buffer");
    // 書き換えていれば尋ねる (y)
    assert_eq!(
        eval_with(
            &["yes"],
            &format!("(find-file {file:?}) (insert \"x\") (write-file-string {file:?} \"four\") (revert-buffer-quick) (buffer-string)"),
        )
        .0,
        "\"four\""
    );
}

#[test]
fn test_not_modified() {
    let (result, messages) = eval_with(&[], "(insert \"x\") (not-modified) (buffer-modified-p)");
    assert_eq!(result, "#f");
    assert_eq!(messages, vec!["Modification-flag cleared"]);
    let (result, messages) = eval_with(&[], "(not-modified #t) (buffer-modified-p)");
    assert_eq!(result, "#t");
    assert_eq!(messages, vec!["Modification-flag set"]);
    assert_eq!(
        eval("(set-current-prefix-arg '(4)) (not-modified) (buffer-modified-p)"),
        "#t"
    );
}

#[test]
fn test_clone_buffer() {
    assert_eq!(
        eval("(insert \"abc\") (define n (clone-buffer)) (list n (with-current-buffer n (buffer-string)))"),
        "(\"*scratch*<2>\" \"abc\")"
    );
    // 数の付いた名前から作ると, 数を付け直す
    assert_eq!(eval("(rename-buffer \"a<2>\") (clone-buffer)"), "\"a\"");
    // 対話では, 表示する. 前置引数があれば名前を読む
    assert_eq!(eval("(clone-buffer) (length (window-list))"), "2");
    assert_eq!(
        eval_with(&["named"], "(set-current-prefix-arg '(4)) (clone-buffer)").0,
        "\"named\""
    );
    assert_eq!(eval("(clone-buffer \"x\" #t) (length (window-list))"), "2");
    // ファイルのバッファは複製できない
    let dir = tempdir().unwrap();
    let path = dir.path().join("f.txt");
    fs::write(&path, "one").unwrap();
    let file = path.display().to_string();
    assert_eq!(
        eval(&format!("(find-file {file:?}) (clone-buffer)")),
        "error: Cannot clone a file-visiting buffer"
    );
    // 別のウィンドウに出して, そちらを選ぶ
    assert_eq!(
        eval("(insert \"q\") (clone-indirect-buffer-other-window) (list (buffer-name) (buffer-string))"),
        "(\"*scratch*<2>\" \"q\")"
    );
    assert_eq!(
        eval_with(
            &["ind"],
            "(set-current-prefix-arg '(4)) (clone-indirect-buffer-other-window) (buffer-name)"
        )
        .0,
        "\"ind\""
    );
    assert_eq!(
        eval("(clone-indirect-buffer-other-window \"z\") (buffer-name)"),
        "\"z\""
    );
}

#[test]
fn test_shell_commands() {
    // 出力が短ければ, エコー領域に出す
    let (result, messages) = eval_with(&[], "(shell-command \"echo hi\") (buffer-name)");
    assert_eq!(result, "\"*scratch*\"");
    assert_eq!(messages, vec!["hi"]);
    // 対話では, コマンドを読む. 前置引数があれば, 今のバッファに入れる
    assert_eq!(
        eval_with(
            &["echo ho"],
            "(set-current-prefix-arg '(4)) (shell-command) (buffer-string)"
        )
        .0,
        "\"ho\\n\""
    );
    assert_eq!(
        eval_with(
            &["echo ho"],
            "(shell-command) (with-current-buffer \"*Shell Command Output*\" (buffer-string))"
        )
        .0,
        "\"ho\\n\""
    );
    // 読み取り専用のバッファには入れられない
    assert_eq!(
        eval("(read-only-mode 1) (shell-command \"echo x\" #t)"),
        "error: Buffer is read-only: #<buffer *scratch*>"
    );
    // 範囲の文字を入力にして, 領域を置き換える
    assert_eq!(
        eval("(insert \"abc\") (shell-command-on-region 1 3 \"tr a-z A-Z\" #f #t) (buffer-string)"),
        "\"ABc\""
    );
    // 対話では, マークが無ければ失敗
    assert_eq!(
        eval("(shell-command-on-region)"),
        "error: The mark is not set now, so there is no region"
    );
    assert_eq!(
        eval_with(&["tr a-z A-Z"], "(insert \"abc\") (set-mark 1) (shell-command-on-region) (with-current-buffer \"*Shell Command Output*\" (buffer-string))").0,
        "\"ABC\""
    );
    // 長い出力は, バッファを出して, エコー領域には出さない
    let (_, messages) = eval_with(&[], "(shell-command \"seq 1 20\") (length (window-list))");
    assert!(messages.is_empty());
    assert_eq!(
        eval("(shell-command \"seq 1 20\") (length (window-list))"),
        "2"
    );
    // 出力が無ければ, 終了コードを知らせる
    let (_, messages) = eval_with(&[], "(shell-command \"true\") (shell-command \"false\")");
    assert_eq!(
        messages,
        vec![
            "(Shell command succeeded with no output)",
            "(Shell command failed with code 1 and no output)"
        ]
    );
    // 非同期のコマンドは, 終わるまで動かして, *Async Shell Command* に出す
    let (result, messages) = eval_with(
        &[],
        "(shell-command \"echo async &\") (with-current-buffer \"*Async Shell Command*\" (buffer-string))",
    );
    assert_eq!(result, "\"async\\n\"");
    assert_eq!(messages, vec!["echo async: finished."]);
    let (_, messages) = eval_with(&[], "(async-shell-command \"exit 2\")");
    assert_eq!(messages, vec!["exit 2: exited abnormally with code 2."]);
    assert_eq!(
        eval_with(
            &["echo q"],
            "(async-shell-command) (with-current-buffer \"*Async Shell Command*\" (buffer-string))"
        )
        .0,
        "\"q\\n\""
    );
    assert_eq!(
        eval("(async-shell-command \"echo w\" \"*out*\") (with-current-buffer \"*out*\" (buffer-string))"),
        "\"w\\n\""
    );
    // 出力のバッファが今のバッファなら, 範囲だけを残して置き換える
    assert_eq!(
        eval("(insert \"abcdef\") (shell-command-on-region 2 4 \"tr a-z A-Z\" \"*scratch*\") (buffer-string)"),
        "\"BC\""
    );
}

#[test]
fn test_shell_quote_argument_and_list_directory() {
    assert_eq!(eval("(shell-quote-argument \"a b\")"), "\"'a b'\"");
    assert_eq!(eval("(shell-quote-argument \"it's\")"), "\"'it'\\\\''s'\"");
    assert_eq!(eval("(shell-quote-argument \"\")"), "\"''\"");
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("one.txt"), "1").unwrap();
    fs::write(dir.path().join("two.rs"), "22").unwrap();
    let path = fs::canonicalize(dir.path()).unwrap();
    let name = path.display().to_string();
    // 簡単な一覧: 先頭の行は Directory と名前
    let listing = eval(&format!(
        "(list-directory {name:?}) (with-current-buffer \"*Directory*\" (buffer-string))"
    ));
    assert!(
        listing.starts_with(&format!("\"Directory {name}\\n")),
        "{listing}"
    );
    assert!(
        listing.contains("one.txt") && listing.contains("two.rs"),
        "{listing}"
    );
    // 詳しい一覧
    let verbose = eval(&format!(
        "(list-directory {name:?} #t) (with-current-buffer \"*Directory*\" (buffer-string))"
    ));
    assert!(
        verbose.contains("total") && verbose.contains("-rw-r--r--"),
        "{verbose}"
    );
    // ワイルドカード
    let globbed = eval(&format!(
        "(list-directory {:?}) (with-current-buffer \"*Directory*\" (buffer-string))",
        format!("{name}/*.rs")
    ));
    assert!(
        globbed.contains("two.rs") && !globbed.contains("one.txt"),
        "{globbed}"
    );
    // 対話では, 名前を読む (前置引数で詳しく)
    let asked = eval_with(
        &[&name],
        "(set-current-prefix-arg '(4)) (list-directory) (with-current-buffer \"*Directory*\" (buffer-string))",
    )
    .0;
    assert!(asked.contains("total"), "{asked}");
    let brief = eval_with(
        &[&name],
        "(list-directory) (with-current-buffer \"*Directory*\" (buffer-string))",
    )
    .0;
    assert!(brief.contains("one.txt"), "{brief}");
}
