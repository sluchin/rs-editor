//! ヘルプ, 定義へ移るコマンド, Info (`lisp/help.scm`, `lisp/xref.scm`, `lisp/info.scm`) のテスト.
//! キーの割り当てと最近のキーは, ターミナルのアプリが答える. ここでは, 偽の画面が表で答える.

use std::collections::VecDeque;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};
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

fn list(items: &[&str]) -> Value {
    Value::List(items.iter().map(|s| Value::String(s.to_string())).collect())
}

impl Host for Ui {
    fn call(&self, op: &str, args: Vec<Value>) -> Result<Value, String> {
        let text = |i: usize| match args.get(i) {
            Some(Value::String(s)) => s.clone(),
            _ => String::new(),
        };
        match op {
            "message" => {
                self.messages.lock().unwrap().push(text(0));
                Ok(Value::Nil)
            }
            "read-string" | "read-key" | "read-file-name" | "completing-read" => self
                .answers
                .lock()
                .unwrap()
                .pop_front()
                .map(Value::String)
                .ok_or_else(|| "no answer".to_string()),
            "key-binding" => Ok(match text(0).as_str() {
                "C-x C-f" => Value::String("find-file".to_string()),
                "C-x" => Value::Boolean(true),
                _ => Value::Boolean(false),
            }),
            "where-is-internal" => Ok(match text(0).as_str() {
                "find-file" => list(&["C-x C-f", "C-x 4 f"]),
                _ => Value::List(Vec::new().into()),
            }),
            "all-key-bindings" => Ok(Value::List(
                vec![
                    list(&["C-x C-f", "find-file"]),
                    list(&["C-a", "beginning-of-line"]),
                ]
                .into_iter()
                .collect(),
            )),
            "recent-keys" => Ok(list(&["C-x", "C-f", "a"])),
            "editor-command-names" => Ok(Value::List(Vec::new().into())),
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
        include_str!("../../lisp/replace.scm"),
        include_str!("../../lisp/isearch.scm"),
        include_str!("../../lisp/help.scm"),
        include_str!("../../lisp/compile.scm"),
        include_str!("../../lisp/xref.scm"),
        include_str!("../../lisp/info.scm"),
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

fn eval(code: &str) -> String {
    eval_with(&[], code).0
}

/// バッファの本文を返す式.
fn text_of(name: &str) -> String {
    format!("(with-current-buffer \"{name}\" (buffer-string))")
}

/// 評価の結果の文字列 (引用符を外し, 改行の `\n` を戻す).
fn plain(result: &str) -> String {
    result
        .trim_matches('"')
        .replace("\\n", "\n")
        .replace("\\\"", "\"")
}

#[test]
fn test_describe_variable_command_and_symbol() {
    assert_eq!(
        plain(&eval(&format!(
            "(define my-var 3 \"A variable.\") (describe-variable 'my-var) {}",
            text_of("*Help*")
        ))),
        "my-var is a variable.\n\nIts value is 3.\n\nA variable.\n"
    );
    // 値だけで, docstring の無い変数
    assert_eq!(
        plain(&eval(&format!(
            "(define bare '(1 2)) (describe-variable 'bare) {}",
            text_of("*Help*")
        ))),
        "bare is a variable.\n\nIts value is (1 2).\n\nNot documented.\n"
    );
    // 名前を読む. 無い変数は失敗
    assert!(eval_with(
        &["my-var"],
        &format!(
            "(define my-var 3 \"Doc.\") (describe-variable) {}",
            text_of("*Help*")
        )
    )
    .0
    .contains("Its value is 3."));
    assert_eq!(
        eval("(describe-variable 'no-such-variable)"),
        "error: no-such-variable is void as a variable"
    );
    // コマンド
    assert!(plain(&eval(&format!(
        "(describe-command 'find-file) {}",
        text_of("*Help*")
    )))
    .starts_with("find-file is an interactive Lisp function in `files.scm'."));
    assert!(eval_with(
        &["find-file"],
        &format!("(describe-command) {}", text_of("*Help*"))
    )
    .0
    .contains("(find-file &rest ARGS)"));
    // 記号は, 関数, 変数, 両方, どちらも無い
    assert!(plain(&eval(&format!(
        "(describe-symbol 'find-file) {}",
        text_of("*Help*")
    )))
    .starts_with("find-file is"));
    assert!(plain(&eval(&format!(
        "(define v 1 \"Doc v.\") (describe-symbol 'v) {}",
        text_of("*Help*")
    )))
    .starts_with("v is a variable."));
    assert_eq!(
        eval("(describe-symbol 'nothing-like-this)"),
        "error: I don't know anything about nothing-like-this"
    );
    assert!(eval_with(
        &["find-file"],
        &format!("(describe-symbol) {}", text_of("*Help*"))
    )
    .0
    .contains("find-file is"));
}

#[test]
fn test_describe_key_and_where_is() {
    // 続きのあるキーは, 続けて読む
    let (result, _) = eval_with(
        &["C-x", "C-f"],
        &format!("(describe-key) {}", text_of("*Help*")),
    );
    assert!(plain(&result).starts_with(
        "C-x C-f runs the command find-file (found in global-map)\n\nfind-file is an interactive Lisp function"
    ));
    // 空白, RET などは, キーの名前に直す
    let (result, _) = eval_with(&["C-z"], &format!("(describe-key) {}", text_of("*Help*")));
    assert_eq!(plain(&result), "C-z is undefined\n");
    assert_eq!(
        plain(&eval(&format!(
            "(describe-key \"C-x C-f\") {}",
            text_of("*Help*")
        )))
        .lines()
        .next(),
        Some("C-x C-f runs the command find-file (found in global-map)")
    );
    // 簡単な説明は, メッセージ
    let (_, messages) = eval_with(&["C-x", "C-f"], "(describe-key-briefly)");
    assert_eq!(
        messages,
        vec!["C-x C-f runs the command find-file (found in global-map)"]
    );
    let (_, messages) = eval_with(&["C-z"], "(describe-key-briefly)");
    assert_eq!(messages, vec!["C-z is undefined"]);
    let (_, messages) = eval_with(&[], "(describe-key-briefly \"C-x C-f\")");
    assert_eq!(messages.len(), 1);
    // where-is
    let (_, messages) = eval_with(&[], "(where-is 'find-file)");
    assert_eq!(messages, vec!["find-file is on C-x C-f, C-x 4 f"]);
    let (_, messages) = eval_with(&[], "(where-is 'forward-char)");
    assert_eq!(messages, vec!["forward-char is not on any key"]);
    let (_, messages) = eval_with(&["find-file"], "(where-is)");
    assert_eq!(messages, vec!["find-file is on C-x C-f, C-x 4 f"]);
}

#[test]
fn test_read_key_names_are_converted() {
    // `read-key` の名前 (" ", "Enter", "Backspace", "Delete", "Escape") は, キーの名前に直す
    let (_, messages) = eval_with(&[" "], "(describe-key-briefly)");
    assert_eq!(messages, vec!["SPC is undefined"]);
    for (read, name) in [
        ("Enter", "RET"),
        ("Backspace", "DEL"),
        ("Delete", "<delete>"),
        ("Escape", "ESC"),
    ] {
        let (_, messages) = eval_with(&[read], "(describe-key-briefly)");
        assert_eq!(messages, vec![format!("{name} is undefined")]);
    }
}

#[test]
fn test_describe_bindings_lossage_and_messages() {
    let help = plain(&eval(&format!("(describe-bindings) {}", text_of("*Help*"))));
    assert!(help.starts_with("Global Bindings:\n\nKey                     Binding\n"));
    // キーの順に並べる
    assert!(help.find("C-a ").unwrap() < help.find("C-x C-f ").unwrap());
    assert!(help.contains("C-x C-f                 find-file\n"));
    // 最近のキー
    assert_eq!(
        plain(&eval(&format!("(view-lossage) {}", text_of("*Help*")))),
        "Recent input:\n\nC-x C-f a\n"
    );
    // 長い並びは, 折り返す
    assert_eq!(
        plain(&eval("(help--wrap \"aaa bbb ccc ddd\" 7)")),
        "aaa bbb\nccc ddd"
    );
    // `*Messages*` は, 出したメッセージの記録. 空のメッセージは, 記録しない
    assert_eq!(
        plain(&eval(&format!(
            "(message \"one\") (message \"\") (message \"two %d\" 2) (view-echo-area-messages) {}",
            text_of("*Messages*")
        ))),
        "one\ntwo 2\n"
    );
    assert_eq!(
        eval(
            "(message \"one\") (view-echo-area-messages) \
             (list (length (window-list)) (with-current-buffer \"*Messages*\" (list (point) (buffer-read-only))))"
        ),
        "(2 (5 #t))"
    );
    assert_eq!(eval("(message-log)"), "()");
    // 記録は 1000 行まで (古いものから捨てる)
    assert_eq!(
        eval(
            "(let loop ((i 0)) (if (< i 1005) (begin (message \"m%d\" i) (loop (+ i 1))))) \
             (list (length (message-log)) (car (message-log)))"
        ),
        "(1000 \"m5\")"
    );
}

#[test]
fn test_help_for_help_and_quick_help() {
    let text = plain(&eval(&format!("(help-for-help) {}", text_of("*Help*"))));
    assert!(text.starts_with("You have typed C-h, the help character."));
    assert!(text.contains("f           Display documentation of the given function."));
    assert_eq!(eval("(help-quit)"), "#f");
    // 簡単な説明は, 2 回呼ぶと閉じる
    assert_eq!(
        eval("(help-quick-toggle) (list (length (window-list)) (if (get-buffer \"*Quick Help*\") #t #f))"),
        "(2 #t)"
    );
    assert_eq!(
        eval(
            "(help-quick-toggle) (help-quick-toggle) \
             (list (length (window-list)) (if (get-buffer \"*Quick Help*\") #t #f))"
        ),
        "(1 #f)"
    );
    // `C-h C-s`: ヘルプの窓を選んで, 前向きに探す
    assert_eq!(
        eval_with(
            &["f", "RET"],
            "(search-forward-help-for-help) (list (buffer-name) (char-before))"
        )
        .0,
        "(\"*Help*\" \"f\")"
    );
}

#[test]
fn test_describe_syntax() {
    let text = plain(&eval(&format!("(describe-syntax) {}", text_of("*Help*"))));
    assert!(text.starts_with("The syntax table of the current buffer:\n\n"));
    assert!(text.contains("^I .. ^J    whitespace\n"));
    assert!(text.contains("A .. Z      word\n"));
    assert!(text.contains("(           open\n"));
    assert!(text.contains(")           close\n"));
    assert!(text.contains("\"           string\n"));
    assert!(text.contains("_           symbol\n"));
}

/// 定義を探すファイル (`(load ...)` で読む) と, その下のディレクトリを作る.
fn project() -> (tempfile::TempDir, String) {
    let dir = tempdir().unwrap();
    let path = dir.path().to_str().unwrap().to_string();
    fs::write(
        format!("{path}/defs.scm"),
        "; defs\n(define (alpha-fn x)\n  x)\n\n(define-command (beta-cmd . args)\n  \"Doc.\"\n  1)\n\n(define gamma-var 3 \"Gamma.\")\n",
    )
    .unwrap();
    fs::write(
        format!("{path}/use.scm"),
        "(alpha-fn 1)\n(beta-cmd)\n(alpha-fn-other 2)\n",
    )
    .unwrap();
    (dir, path)
}

#[test]
fn test_xref_find_definitions_and_history() {
    let (_dir, path) = project();
    let load = format!("(load \"{path}/defs.scm\") ");
    // 関数, コマンド, 変数の定義に, 行の先頭の字下げの後ろまで動く
    assert_eq!(
        eval(
            &format!(
            "{load}(xref-find-definitions \"alpha-fn\") (list (buffer-name) (count-lines-before))"
        )
            .replace(
                "(count-lines-before)",
                "(length (split-string (buffer-substring 1 (point)) \"\\n\"))"
            )
        ),
        "(\"defs.scm\" 2)"
    );
    assert_eq!(
        eval(&format!(
            "{load}(xref-find-definitions \"beta-cmd\") (length (split-string (buffer-substring 1 (point)) \"\\n\"))"
        )),
        "5"
    );
    assert_eq!(
        eval(&format!(
            "{load}(xref-find-definitions \"gamma-var\") (length (split-string (buffer-substring 1 (point)) \"\\n\"))"
        )),
        "9"
    );
    // 別の窓
    assert_eq!(
        eval(&format!(
            "{load}(xref-find-definitions-other-window \"alpha-fn\") (list (buffer-name) (length (window-list)))"
        )),
        "(\"defs.scm\" 2)"
    );
    // 定義が無い, ファイルが無い名前
    assert_eq!(
        eval(&format!("{load}(xref-find-definitions \"nothing-like\")")),
        "error: No definition found for: nothing-like"
    );
    assert_eq!(
        eval("(xref-find-definitions \"car\")"),
        "error: No definition found for: car"
    );
    // 前に戻る (M-,), 先へ進む (C-M-,)
    assert_eq!(
        eval(&format!(
            "{load}(xref-find-definitions \"alpha-fn\") (xref-go-back) (list (buffer-name) (point))"
        )),
        "(\"*scratch*\" 1)"
    );
    assert_eq!(
        eval(&format!(
            "{load}(xref-find-definitions \"alpha-fn\") (xref-go-back) (xref-go-forward) (buffer-name)"
        )),
        "\"defs.scm\""
    );
    assert_eq!(eval("(xref-go-back)"), "error: At start of xref history");
    assert_eq!(eval("(xref-go-forward)"), "error: At end of xref history");
    // 場所を覚えたバッファが消えていたら, 失敗
    assert_eq!(
        eval(&format!(
            "{load}(get-buffer-create \"tmp\") (switch-to-buffer \"tmp\") (xref-find-definitions \"alpha-fn\") \
             (kill-buffer \"tmp\") (xref-go-back)"
        )),
        "error: The buffer of the location was killed"
    );
}

#[test]
fn test_xref_identifier_at_point_and_prompt() {
    let (_dir, path) = project();
    let load = format!("(load \"{path}/defs.scm\") ");
    // 点の所の記号が既定
    assert_eq!(
        eval(&format!(
            "{load}(insert \"(alpha-fn 1)\") (goto-char 4) (xref-find-definitions) (buffer-name)"
        )),
        "\"defs.scm\""
    );
    // 記号が無ければ, 読む (空なら, 既定も無いので失敗)
    assert_eq!(
        eval_with(
            &["beta-cmd"],
            &format!("{load}(xref-find-definitions) (buffer-name)")
        )
        .0,
        "\"defs.scm\""
    );
    assert_eq!(
        eval_with(&[""], &format!("{load}(xref-find-definitions)")).0,
        "error: There is no default identifier"
    );
    // 既定がある所で, 空の答えは, 既定を使う
    assert_eq!(
        eval_with(
            &[""],
            &format!("{load}(insert \"alpha-fn\") (set! xref-prompt-for-identifier #t) (xref-find-definitions) (buffer-name)")
        )
        .0,
        "\"defs.scm\""
    );
    // help-find-source は, 関数の名前を読む
    assert_eq!(
        eval_with(
            &["alpha-fn"],
            &format!("{load}(help-find-source) (buffer-name)")
        )
        .0,
        "\"defs.scm\""
    );
    assert_eq!(
        eval(&format!(
            "{load}(help-find-source 'beta-cmd) (length (window-list))"
        )),
        "2"
    );
}

#[test]
fn test_xref_find_references_and_apropos() {
    let (_dir, path) = project();
    let set = format!("(set! xref-project-directory \"{path}\") ");
    let text = text_of("*xref*");
    let found = plain(&eval(&format!(
        "{set}(xref-find-references \"alpha-fn\") {text}"
    )));
    assert!(found.starts_with("References to alpha-fn:\n\n"));
    assert!(found.contains("/defs.scm:2:(define (alpha-fn x)\n"));
    assert!(found.contains("/use.scm:1:(alpha-fn 1)\n"));
    // 単語の境界: alpha-fn-other は, alpha-fn の参照に含まれない (`-` は単語の区切り) ので, 行は 3 つ
    assert_eq!(found.lines().filter(|l| l.contains(".scm:")).count(), 3);
    assert_eq!(
        eval(&format!("{set}(xref-find-references \"zzz-nowhere\")")),
        "error: No references found for: zzz-nowhere"
    );
    // 名前の記号が既定
    assert_eq!(
        eval(&format!(
            "{set}(insert \"beta-cmd\") (xref-find-references) (length (window-list))"
        )),
        "2"
    );
    // 探す対象から, .git などは外す
    fs::create_dir_all(format!("{path}/.git")).unwrap();
    fs::write(format!("{path}/.git/x.scm"), "(alpha-fn 9)\n").unwrap();
    let found = plain(&eval(&format!(
        "{set}(xref-find-references \"alpha-fn\") {text}"
    )));
    assert!(!found.contains(".git"));
    // apropos: 名前が正規表現に合う定義
    let (_dir2, path2) = project();
    let load = format!("(load \"{path2}/defs.scm\") ");
    let found = plain(&eval(&format!(
        "{load}(xref-find-apropos \"^alpha\") {text}"
    )));
    assert!(found.starts_with("Definitions matching ^alpha:\n\nalpha-fn  ("));
    let found = plain(&eval(&format!("(xref-find-apropos \"^car$\") {text}")));
    assert!(found.contains("\ncar\n"));
    assert_eq!(
        eval("(xref-find-apropos \"^zzz-no-such\")"),
        "error: No apropos matches for: ^zzz-no-such"
    );
    assert!(
        eval_with(&["^alpha-f"], &format!("{load}(xref-find-apropos) {text}"))
            .0
            .contains("alpha-fn")
    );
}

/// Info のファイル (節の区切りは, 0x1f).
const MANUAL: &str = "This is a test manual.\n\x1f\nFile: m.info,  Node: Top,  Next: Visiting\n\nTop node text.\n\n* Menu:\n\n* Visiting::\n\x1f\nFile: m.info,  Node: Visiting,  Prev: Top\n\nHow to visit files.\n\x1f\nTag Table:\nNode: Top\x7f10\n\x1f\nEnd Tag Table\n";

/// Emacs のマニュアルの索引だけを持つ, Info のファイル.
const EMACS_MANUAL: &str = "\x1f\nFile: emacs.info,  Node: Top,  Next: Visiting\n\nEmacs.\n\x1f\nFile: emacs.info,  Node: Visiting,  Prev: Top\n\nVisiting files: C-x C-f.\n\x1f\nFile: emacs.info,  Node: Command Index\n\n* find-file: Visiting.          (line 3)\n";

fn info_dir() -> (tempfile::TempDir, String) {
    let dir = tempdir().unwrap();
    let path = dir.path().to_str().unwrap().to_string();
    fs::write(format!("{path}/m.info"), MANUAL).unwrap();
    fs::write(
        format!("{path}/dir"),
        "This is the Info directory.\n* m: (m).   A test manual.\n",
    )
    .unwrap();
    // 圧縮したファイル
    let mut gzip = Command::new("gzip")
        .arg("-c")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    gzip.stdin
        .take()
        .unwrap()
        .write_all(EMACS_MANUAL.as_bytes())
        .unwrap();
    let output = gzip.wait_with_output().unwrap();
    fs::write(format!("{path}/emacs.info.gz"), output.stdout).unwrap();
    (dir, path)
}

#[test]
fn test_info() {
    let (_dir, path) = info_dir();
    let set = format!("(set! Info-directory-list (list \"/nonexistent\" \"{path}\")) ");
    let text = text_of("*info*");
    // 圧縮していないマニュアル. 節の区切りは消す
    let shown = plain(&eval(&format!("{set}(info-display-manual \"m\") {text}")));
    assert!(shown.starts_with("This is a test manual.\n\nFile: m.info,  Node: Top"));
    assert!(!shown.contains('\u{1f}'));
    assert_eq!(
        eval(&format!(
            "{set}(info-display-manual \"m\") (list (buffer-name) (point) (buffer-read-only))"
        )),
        "(\"*info*\" 1 #t)"
    );
    // 引数が無ければ, 名前を読む
    assert!(
        eval_with(&["m"], &format!("{set}(info-display-manual) {text}"))
            .0
            .contains("Top node text.")
    );
    // info は, ディレクトリ. 別の窓に出すこともできる
    assert!(plain(&eval(&format!("{set}(info) {text}"))).starts_with("This is the Info directory."));
    assert_eq!(
        eval(&format!(
            "{set}(info-other-window \"m\") (length (window-list))"
        )),
        "2"
    );
    assert!(plain(&eval(&format!("{set}(info-other-window) {text}"))).contains("A test manual."));
    // 無いマニュアル
    assert_eq!(
        eval(&format!("{set}(info-display-manual \"nope\")")),
        "error: Info file nope not found"
    );
    // 圧縮したマニュアル (Emacs)
    assert!(plain(&eval(&format!("{set}(info-emacs-manual) {text}")))
        .contains("Visiting files: C-x C-f."));
}

#[test]
fn test_info_goto_emacs_nodes() {
    let (_dir, path) = info_dir();
    let set = format!("(set! Info-directory-list (list \"{path}\")) ");
    // 索引の項目から, 節の頭へ
    assert_eq!(
        eval(&format!(
            "{set}(Info-goto-emacs-command-node 'find-file) (looking-at \"File: emacs.info,  Node: Visiting\")"
        )),
        "#t"
    );
    assert_eq!(
        eval(&format!(
            "{set}(Info-goto-emacs-command-node 'no-such-command)"
        )),
        "error: Couldn't find documentation for no-such-command"
    );
    // 名前を読む
    assert_eq!(
        eval_with(
            &["find-file"],
            &format!("{set}(Info-goto-emacs-command-node) (looking-at \"File: emacs.info,  Node: Visiting\")")
        )
        .0,
        "#t"
    );
    // キーから (キーの割り当ての表は, 偽の画面が持つ)
    assert_eq!(
        eval(&format!(
            "{set}(Info-goto-emacs-key-command-node \"C-x C-f\") (looking-at \"File: emacs.info,  Node: Visiting\")"
        )),
        "#t"
    );
    assert_eq!(
        eval(&format!("{set}(Info-goto-emacs-key-command-node \"C-z\")")),
        "error: C-z is undefined"
    );
    assert_eq!(
        eval_with(
            &["C-x", "C-f"],
            &format!("{set}(Info-goto-emacs-key-command-node) (looking-at \"File: emacs.info,  Node: Visiting\")")
        )
        .0,
        "#t"
    );
    // 節が見つからなくても, 索引の項目の所に残る
    let _ = &path;
}
