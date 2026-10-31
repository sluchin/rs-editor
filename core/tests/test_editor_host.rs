//! `EditorHost` と, エディタの操作の手続き (`editor_ops`) のテスト. 評価器に Scheme を渡して確かめる.

use std::sync::Mutex;

use rseditor_core::editor::Editor;
use rseditor_core::scheme::editor_host::EditorHost;
use rseditor_core::scheme::evaluator::Evaluator;
use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::value::Value;

/// 画面の要る操作を受ける偽物: メッセージを覚え, ほかの操作は失敗にする.
#[derive(Default)]
struct FakeUi {
    messages: Mutex<Vec<String>>,
}

impl Host for FakeUi {
    fn call(&self, op: &str, args: Vec<Value>) -> Result<Value, String> {
        if op == "message" {
            let Value::String(text) = &args[0] else {
                panic!("not a string")
            };
            self.messages.lock().unwrap().push(text.clone());
            return Ok(Value::Nil);
        }
        Err(format!("ui: {op}"))
    }
}

/// 評価器とホストを作って, Scheme を評価する.
fn run(code: &str) -> Result<String, String> {
    let (ev, host) = setup();
    ev.eval_with(code, &host)
}

/// 評価器とホスト.
fn setup() -> (Evaluator, EditorHost<FakeUi>) {
    (
        Evaluator::new(),
        EditorHost::new(Editor::new(), FakeUi::default()),
    )
}

/// 始めに入れる本文: `ab\ncde\n\nf` で, カーソルは始め.
const SETUP: &str = "(insert \"ab\\ncde\\n\\nf\") (goto-char 1) ";

/// 評価した最後の結果 (失敗なら `ERR 内容`).
fn run_last(code: &str) -> String {
    match run(code) {
        Ok(out) => out.lines().last().unwrap_or("").to_string(),
        Err(e) => format!("ERR {e}"),
    }
}

/// (式, 結果). 式は, 始めの本文を入れたエディタで評価する. 結果は, `emacs -Q --batch` と比べて確かめた.
const CASES: &[(&str, &str)] = &[
    (
        r##"(list (point) (point-min) (point-max) (buffer-name) (buffer-size))"##,
        r##"(1 1 10 "*scratch*" 9)"##,
    ),
    (r##"(goto-char 99)"##, r##"10"##),
    (r##"(goto-char 0)"##, r##"1"##),
    (r##"(delete-region 1 99)"##, r##"ERR Args out of range"##),
    (r##"(buffer-substring 1 99)"##, r##"ERR Args out of range"##),
    (r##"(buffer-substring 3 1)"##, r##""ab""##),
    (
        r##"(list (char-after) (char-after 3) (char-after 99) (char-before) (char-before 2))"##,
        r##"("a" "\n" #f #f "a")"##,
    ),
    (r##"(forward-char 99)"##, r##"ERR End of buffer"##),
    (r##"(forward-char -1)"##, r##"ERR Beginning of buffer"##),
    (r##"(backward-char 1)"##, r##"ERR Beginning of buffer"##),
    (r##"(list (forward-line 2) (point))"##, r##"(0 8)"##),
    (r##"(list (forward-line 99) (point))"##, r##"(95 10)"##),
    (
        r##"(list (line-beginning-position) (line-end-position) (line-beginning-position 2) (line-end-position 0))"##,
        r##"(1 3 4 1)"##,
    ),
    (
        r##"(list (bobp) (eobp) (bolp) (eolp))"##,
        r##"(#t #f #t #f)"##,
    ),
    (r##"(delete-char 2) (buffer-string)"##, r##""\ncde\n\nf""##),
    (r##"(delete-char 99)"##, r##"ERR End of buffer"##),
    (r##"(delete-char -1)"##, r##"ERR Beginning of buffer"##),
    (
        r##"(list (current-column) (move-to-column 5))"##,
        r##"(0 2)"##,
    ),
    (r##"(search-forward "cd")"##, r##"6"##),
    (
        r##"(list (search-forward "cd") (match-beginning 0) (match-end 0) (match-data))"##,
        r##"(6 4 6 (4 6))"##,
    ),
    (r##"(search-forward "zz")"##, r##"ERR Search failed: "zz""##),
    (r##"(search-forward "zz" #f #t)"##, r##"#f"##),
    (
        r##"(list (search-forward "zz" 4 'move) (point))"##,
        r##"(#f 4)"##,
    ),
    (
        r##"(search-forward "cd" 2)"##,
        r##"ERR Search failed: "cd""##,
    ),
    (r##"(search-forward "e" 5)"##, r##"ERR Search failed: "e""##),
    (r##"(search-forward "b" 1)"##, r##"ERR Search failed: "b""##),
    (
        r##"(list (re-search-forward "\\(c\\)\\(d\\)") (match-string 1) (match-string 2) (match-string 5))"##,
        r##"(6 "c" "d" #f)"##,
    ),
    (
        r##"(goto-char 12) (list (re-search-backward "c") (point))"##,
        r##"(4 4)"##,
    ),
    (
        r##"(goto-char 12) (search-backward "c" #f #f 1)"##,
        r##"4"##,
    ),
    (r##"(search-forward "a" #f #f 0)"##, r##"1"##),
    (
        r##"(goto-char 12) (search-forward "c" #f #f -1)"##,
        r##"4"##,
    ),
    (r##"(looking-at "ab")"##, r##"#t"##),
    (r##"(looking-at "b")"##, r##"#f"##),
    (r##"(match-data)"##, r##"()"##),
    (
        r##"(set! case-fold-search #f) (search-forward "AB" #f #t)"##,
        r##"#f"##,
    ),
    (r##"(search-forward "AB" #f #t)"##, r##"3"##),
    (r##"(mark)"##, r##"#f"##),
    (
        r##"(set-mark 4) (list (mark) (region-beginning) (region-end) (use-region-p))"##,
        r##"(4 1 4 #t)"##,
    ),
    (
        r##"(region-beginning)"##,
        r##"ERR The mark is not set now, so there is no region"##,
    ),
    (r##"(set-mark 4) (set-mark #f) (use-region-p)"##, r##"#f"##),
    (
        r##"(push-mark 3) (push-mark 5 #t #t) (pop-mark) (mark)"##,
        r##"3"##,
    ),
    (
        r##"(kill-new "a") (kill-new "b") (list (current-kill 0) (current-kill 1) (current-kill 1 #t))"##,
        r##"("b" "a" "b")"##,
    ),
    (r##"(current-kill 0)"##, r##"ERR Kill ring is empty"##),
    (
        r##"(kill-region 1 3) (list (buffer-string) (current-kill 0))"##,
        r##"("\ncde\n\nf" "ab")"##,
    ),
    (
        r##"(kill-new "x") (kill-append "y" #f) (current-kill 0)"##,
        r##""xy""##,
    ),
    (
        r##"(insert-for-yank "zz") (buffer-string)"##,
        r##""zzab\ncde\n\nf""##,
    ),
    (
        r##"(undo-boundary) (insert "x") (undo-boundary) (buffer-undo-list)"##,
        r##"(#f (1 2) #f (1 10))"##,
    ),
    (
        r##"(insert "xy") (undo-boundary) (delete-region 1 3) (buffer-undo-list)"##,
        r##"(("xy" -1) 3 #f (1 3) (1 10))"##,
    ),
    (
        r##"(insert "XY") (undo-boundary) (primitive-undo 1 (buffer-undo-list)) (buffer-string)"##,
        r##""XYab\ncde\n\nf""##,
    ),
    (r##"(primitive-undo 1 '((1 2) #f))"##, r##"()"##),
    (
        r##"(primitive-undo 1 '(a))"##,
        r##"ERR primitive-undo: invalid undo entry"##,
    ),
    (
        r##"(list (current-prefix-arg) (prefix-numeric-value #f) (prefix-numeric-value '-) (prefix-numeric-value '(16)) (prefix-numeric-value 3))"##,
        r##"(#f 1 -1 16 3)"##,
    ),
    (
        r##"(set-current-prefix-arg '(4)) (current-prefix-arg)"##,
        r##"(4)"##,
    ),
    (
        r##"(set-current-prefix-arg '-) (current-prefix-arg)"##,
        r##"-"##,
    ),
    (
        r##"(set-current-prefix-arg 7) (current-prefix-arg)"##,
        r##"7"##,
    ),
    (
        r##"(set-current-prefix-arg "x")"##,
        r##"ERR set-current-prefix-arg: invalid prefix argument"##,
    ),
    (
        r##"(set-syntax-dialect 'elisp) (list (syntax-dialect) (scan-lists 1 1 0))"##,
        r##"(elisp #f)"##,
    ),
    (
        r##"(erase-buffer) (insert "(a (b) c) ;x\n") (set-syntax-dialect 'elisp) (list (scan-sexps 1 1) (scan-lists 4 1 1) (syntax-ppss 6) (char-syntax "(") (char-syntax 97))"##,
        r##"(10 10 (2 4 #f #f #f #f 0 #f #f (1 4) #f) "(" "w")"##,
    ),
    (
        r##"(erase-buffer) (insert "  ; c\nx") (set-syntax-dialect 'elisp) (goto-char 1) (list (forward-comment 1) (point))"##,
        r##"(#t 7)"##,
    ),
    (
        r##"(erase-buffer) (insert "'x") (set-syntax-dialect 'elisp) (backward-prefix-chars) (point)"##,
        r##"3"##,
    ),
    (
        r##"(set-syntax-dialect 'bad)"##,
        r##"ERR set-syntax-dialect: unknown dialect bad"##,
    ),
    (
        r##"(erase-buffer) (insert "a ; c\n  ") (set-syntax-dialect 'elisp) (list (forward-comment -1) (point) (forward-comment -1) (point))"##,
        r##"(#t 3 #f 2)"##,
    ),
    (
        r##"(erase-buffer) (insert "a #| c |# ") (set-syntax-dialect 'scheme) (list (forward-comment -1) (point))"##,
        r##"(#t 3)"##,
    ),
    (
        r##"(erase-buffer) (insert "a\n b") (set-syntax-dialect 'scheme) (goto-char 3) (list (forward-comment -1) (point) (forward-comment 0) (forward-comment 1) (point))"##,
        r##"(#f 2 #t #f 4)"##,
    ),
    (
        r##"(erase-buffer) (insert "ab  cd") (goto-char 1) (list (skip-syntax-forward "w") (point) (skip-syntax-forward "^w") (point) (skip-syntax-backward "^w") (point) (skip-syntax-backward "w" 1) (point))"##,
        r##"(2 3 2 5 -2 3 -2 1)"##,
    ),
    (
        r##"(skip-syntax-forward "q")"##,
        r##"ERR Invalid syntax description letter: q"##,
    ),
    (
        r##"(erase-buffer) (insert "a |# ") (set-syntax-dialect 'scheme) (list (forward-comment -1) (point))"##,
        r##"(#f 5)"##,
    ),
    (
        r##"(char-syntax "ab")"##,
        r##"ERR char-syntax expects a character"##,
    ),
    (
        r##"(list (selected-window) (window-list) (split-window) (window-list) (split-window 'right) (window-list))"##,
        r##"(1 (1) 2 (1 2) 3 (1 3 2))"##,
    ),
    (
        r##"(split-window 'sideways)"##,
        r##"ERR split-window: unknown direction sideways"##,
    ),
    (
        r##"(split-window) (select-window 2) (list (selected-window) (window-buffer) (window-buffer 1))"##,
        r##"(2 "*scratch*" "*scratch*")"##,
    ),
    (
        r##"(split-window) (other-window 1) (delete-window) (window-list)"##,
        r##"(1)"##,
    ),
    (
        r##"(delete-window)"##,
        r##"ERR Attempt to delete minibuffer or sole ordinary window"##,
    ),
    (
        r##"(split-window) (delete-other-windows) (window-list)"##,
        r##"(1)"##,
    ),
    (r##"(get-buffer "x")"##, r##"#f"##),
    (
        r##"(list (get-buffer-create "x") (generate-new-buffer "x") (buffer-list) (current-buffer))"##,
        r##"("x" "x<2>" ("*scratch*" "x" "x<2>") "*scratch*")"##,
    ),
    (
        r##"(get-buffer-create "x") (set-buffer "x") (insert "hi") (list (current-buffer) (buffer-string))"##,
        r##"("x" "hi")"##,
    ),
    (r##"(set-buffer "nope")"##, r##"ERR No such buffer: nope"##),
    (
        r##"(get-buffer-create "x") (set-window-buffer 1 "x") (window-buffer)"##,
        r##""x""##,
    ),
    (
        r##"(with-current-buffer "nope" 1)"##,
        r##"ERR No such buffer: nope"##,
    ),
    (
        r##"(get-buffer-create "x") (list (with-current-buffer "x" (insert "a") (buffer-name)) (buffer-name) (buffer-string))"##,
        r##"("x" "*scratch*" "ab\ncde\n\nf")"##,
    ),
    (
        r##"(list (save-excursion (goto-char 3) (point)) (point))"##,
        r##"(3 1)"##,
    ),
    (
        r##"(get-buffer-create "x") (save-current-buffer (set-buffer "x") (buffer-name))"##,
        r##""x""##,
    ),
    (
        r##"(get-buffer-create "x") (save-current-buffer (set-buffer "x") (kill-buffer "x")) (buffer-name)"##,
        r##""*scratch*""##,
    ),
    (
        r##"(with-current-buffer)"##,
        r##"ERR with-current-buffer expects a buffer"##,
    ),
    (r##"(read-key "x")"##, r##"ERR ui: read-key"##),
    (
        r##"(forward-char 1 2)"##,
        r##"ERR forward-char expects 0 to 1 arguments"##,
    ),
    (
        r##"(forward-char "a")"##,
        r##"ERR forward-char expects a number"##,
    ),
    (
        r##"(delete-char)"##,
        r##"ERR delete-char expects 1 arguments"##,
    ),
];

#[test]
fn test_operations() {
    for (code, expected) in CASES {
        assert_eq!(run_last(&format!("{SETUP}{code}")), *expected, "{code}");
    }
}

/// Scheme の文字列にする (`\` と `"` をエスケープする).
fn quoted(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}

#[test]
fn test_accessors_and_pattern_cache() {
    let (ev, host) = setup();
    ev.eval_with("(insert \"abc abc\") (goto-char 1)", &host)
        .unwrap();
    // 同じ正規表現を続けて使うと, 覚えたものを使う. 上限を超えたら, 作り直す
    for _ in 0..2 {
        assert_eq!(
            ev.eval_with("(goto-char 1) (re-search-forward \"b\")", &host)
                .unwrap()
                .lines()
                .last(),
            Some("3")
        );
    }
    for i in 0..70 {
        ev.eval_with(
            &format!("(goto-char 1) (re-search-forward \"a{{{}}}\" #f #t)", i + 1),
            &host,
        )
        .unwrap();
    }
    assert_eq!(host.editor().current_buffer().text(), "abc abc");
    assert!(host.ui().messages.lock().unwrap().is_empty());
}

#[test]
fn test_wrong_argument_types_at_the_host() {
    let host = EditorHost::new(Editor::new(), FakeUi::default());
    let bad = |op: &str, args: Vec<Value>| host.call(op, args).unwrap_err();
    assert_eq!(
        bad("goto-char", vec![Value::String("x".into())]),
        "goto-char expects a number"
    );
    assert_eq!(
        bad("kill-new", vec![Value::Number(1.0)]),
        "kill-new expects a string"
    );
    assert_eq!(
        bad("set-syntax-dialect", vec![Value::Number(1.0)]),
        "set-syntax-dialect expects a name"
    );
    assert_eq!(
        bad(
            "primitive-undo",
            vec![Value::Number(1.0), Value::Number(1.0)]
        ),
        "primitive-undo expects a list"
    );
}

#[test]
fn test_buffers_without_a_name_use_the_current_buffer() {
    assert_eq!(
        run_last("(list (buffer-modified-p) (buffer-file-name))"),
        "(#f #f)"
    );
    assert_eq!(
        run_last("(buffer-file-name \"nope\")"),
        "ERR No such buffer: nope"
    );
    assert_eq!(
        run_last("(list (set-buffer-modified-p #t) (buffer-modified-p))"),
        "(#t #t)"
    );
}

#[test]
fn test_syntax_classes_and_dialects() {
    let all = |dialect: &str| {
        run_last(&format!(
            "(set-syntax-dialect '{dialect}) (list (syntax-dialect) (char-syntax \" \") (char-syntax \".\") (char-syntax \"a\") (char-syntax \"-\") (char-syntax \"(\") (char-syntax \")\") (char-syntax \"\\\"\") (char-syntax \"\\\\\") (char-syntax \"'\") (char-syntax \";\") (char-syntax \"\\n\"))"
        ))
    };
    assert_eq!(
        all("fundamental"),
        r#"(fundamental " " "." "w" "_" "(" ")" "\"" "\\" "." "." " ")"#
    );
    assert_eq!(
        all("elisp"),
        r#"(elisp " " "_" "w" "_" "(" ")" "\"" "\\" "'" "<" ">")"#
    );
    assert_eq!(
        all("scheme"),
        r#"(scheme " " "_" "w" "_" "(" ")" "\"" "\\" "'" "<" ">")"#
    );
}

#[test]
fn test_undo_entries() {
    // 消した文字の記録 (カーソルが終わりにあれば位置が負). 区切りは nil
    assert_eq!(
        run_last("(erase-buffer) (insert \"hello\") (undo-boundary) (delete-region 2 4) (primitive-undo 1 (buffer-undo-list)) (buffer-string)"),
        "\"hello\""
    );
    assert_eq!(
        run_last("(erase-buffer) (insert \"hello\") (primitive-undo 1 '((\"xy\" -2) #f)) (buffer-string)"),
        "\"hxyello\""
    );
    assert_eq!(
        run_last("(erase-buffer) (insert \"hello\") (primitive-undo 1 '((\"xy\" 0)))"),
        "ERR primitive-undo: invalid undo entry"
    );
    assert_eq!(
        run_last("(erase-buffer) (primitive-undo 1 '((3 2)))"),
        "ERR primitive-undo: invalid undo entry"
    );
    // 残りの記録を返す
    assert_eq!(run_last("(primitive-undo 1 '(#f #f))"), "(#f)");
}

#[test]
fn test_search_errors_and_match_data() {
    assert_eq!(
        run_last("(insert \"abcdef\") (goto-char 5) (search-forward \"b\" 2)"),
        "ERR Invalid search bound (wrong side of point)"
    );
    assert_eq!(
        run_last("(insert \"abcdef\") (goto-char 1) (search-backward \"b\" 5)"),
        "ERR Invalid search bound (wrong side of point)"
    );
    // 合わなかった括弧は #f
    assert_eq!(
        run_last("(insert \"ab\") (goto-char 1) (re-search-forward \"\\\\(x\\\\)\\\\|\\\\(b\\\\)\") (match-data)"),
        "(2 3 #f #f 2 3)"
    );
    assert_eq!(
        run_last("(insert \"ab\") (goto-char 1) (re-search-forward \"b\") (match-beginning 9)"),
        "#f"
    );
    assert_eq!(
        run_last("(re-search-forward \"(\")"),
        "ERR Search failed: \"(\""
    );
    // 作れない正規表現は, 失敗にする
    assert!(run_last("(re-search-forward \"\\\\1\")").starts_with("ERR Invalid regexp:"));
}

#[test]
fn test_deactivate_mark_and_prefix_errors() {
    assert_eq!(
        run_last("(insert \"abc\") (set-mark 2) (deactivate-mark) (list (mark) (use-region-p))"),
        "(2 #f)"
    );
    assert_eq!(
        run_last("(set-current-prefix-arg '(1 2))"),
        "ERR set-current-prefix-arg: invalid prefix argument"
    );
}

#[test]
fn test_windows_and_buffers_errors() {
    assert_eq!(run_last("(select-window 9)"), "ERR No such window: 9");
    assert_eq!(
        run_last("(set-window-buffer 1 \"nope\")"),
        "ERR No such buffer: nope"
    );
    assert_eq!(run_last("(window-buffer 9)"), "ERR No such window: 9");
    assert_eq!(run_last("(delete-window 9)"), "ERR No such window: 9");
    assert_eq!(
        run_last("(split-window) (delete-window 2) (window-list)"),
        "(1)"
    );
    assert_eq!(run_last("(other-window)"), "nil");
    assert_eq!(
        run_last("(split-window) (other-window) (selected-window)"),
        "2"
    );
    assert_eq!(
        run_last("(split-window 'below) (split-window 'right) (window-list)"),
        "(1 3 2)"
    );
    assert_eq!(
        run_last("(split-window 5)"),
        "ERR split-window expects a name"
    );
    assert_eq!(
        run_last("(kill-buffer \"nope\")"),
        "ERR No such buffer: nope"
    );
}

#[test]
fn test_files() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("a.scm");
    let missing = dir.path().join("none.el");
    let plain = dir.path().join("p.txt");
    std::fs::write(&path, "(a b)\n").unwrap();
    std::fs::write(&plain, "text").unwrap();
    let (p, m, t) = (
        quoted(path.to_str().unwrap()),
        quoted(missing.to_str().unwrap()),
        quoted(plain.to_str().unwrap()),
    );
    let (ev, host) = setup();
    let eval = |code: &str| {
        ev.eval_with(code, &host)
            .map(|out| out.lines().last().unwrap_or("").to_string())
            .unwrap_or_else(|e| format!("ERR {e}"))
    };
    // 開く: 名前はファイルの名前. 同じファイルは, 同じバッファ
    assert_eq!(eval(&format!("(find-file-noselect {p})")), "\"a.scm\"");
    assert_eq!(eval(&format!("(find-file-noselect {p})")), "\"a.scm\"");
    assert_eq!(eval("(with-current-buffer \"a.scm\" (list (buffer-string) (point) (buffer-modified-p) (syntax-dialect)))"), "(\"(a b)\\n\" 1 #f scheme)");
    // 無いファイルは, 空のバッファ (拡張子で方言が決まる). 読めなければ失敗
    assert_eq!(eval(&format!("(find-file-noselect {m})")), "\"none.el\"");
    assert_eq!(
        eval("(with-current-buffer \"none.el\" (list (buffer-string) (syntax-dialect)))"),
        "(\"\" elisp)"
    );
    assert_eq!(eval(&format!("(find-file-noselect {t})")), "\"p.txt\"");
    assert_eq!(
        eval("(with-current-buffer \"p.txt\" (syntax-dialect))"),
        "fundamental"
    );
    assert!(eval(&format!(
        "(find-file-noselect {})",
        quoted(dir.path().to_str().unwrap())
    ))
    .starts_with("ERR "));
    // 同じ名前の別のファイルには, 番号を付ける
    let other = dir.path().join("sub");
    std::fs::create_dir(&other).unwrap();
    let other_file = other.join("a.scm");
    std::fs::write(&other_file, "x").unwrap();
    assert_eq!(
        eval(&format!(
            "(find-file-noselect {})",
            quoted(other_file.to_str().unwrap())
        )),
        "\"a.scm<2>\""
    );
    // 書き換えたファイルのバッファは消せない. 書けば消せる
    eval("(set-buffer \"a.scm\") (insert \"x\")");
    assert_eq!(
        eval("(kill-buffer \"a.scm\")"),
        "ERR Buffer a.scm is modified"
    );
    assert_eq!(eval("(files--write-buffer (buffer-file-name))"), "nil");
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "x(a b)\n");
    assert_eq!(
        host.ui().messages.lock().unwrap().last().unwrap(),
        &format!("Wrote {}", path.display())
    );
    assert_eq!(eval("(buffer-modified-p)"), "#f");
    // 別の名前に書く. 書けなければ失敗
    let out = dir.path().join("out.txt");
    assert_eq!(
        eval(&format!(
            "(files--write-buffer {})",
            quoted(out.to_str().unwrap())
        )),
        "nil"
    );
    assert_eq!(std::fs::read_to_string(&out).unwrap(), "x(a b)\n");
    assert!(eval("(files--write-buffer \"/nonexistent-dir/x/y\")").starts_with("ERR "));
    assert_eq!(
        eval("(files--write-buffer \"/tmp/zz\" \"nope\")"),
        "ERR No such buffer: nope"
    );
    assert_eq!(eval("(kill-buffer \"a.scm\")"), "#t");
    // 本文を置き換える: 同じなら変えない. 前後の同じ所は残す
    eval("(set-buffer \"p.txt\") (goto-char 3)");
    assert_eq!(eval("(files--set-buffer-text \"text\" #f) (list (buffer-string) (point) (buffer-modified-p))"), "(\"text\" 3 #f)");
    assert_eq!(eval("(files--set-buffer-text \"teXXt\" #t) (list (buffer-string) (point) (buffer-modified-p))"), "(\"teXXt\" 3 #t)");
    assert_eq!(
        eval("(files--set-buffer-text \"t\" #f) (list (buffer-string) (point))"),
        "(\"t\" 2)"
    );
    assert_eq!(
        eval("(files--set-buffer-text \"\" #f) (buffer-string)"),
        "\"\""
    );
    assert_eq!(eval("(files--revert-dired)"), "#f");
}

#[test]
fn test_showing_buffers_in_windows() {
    // 別のウィンドウに出す: 1 つだけなら分け, 2 つ以上なら別のウィンドウに出す. 行を渡すと, その行に置く
    assert_eq!(
        run_last("(display-buffer \"b\") (list (window-list) (window-buffer 1) (window-buffer 2) (selected-window))"),
        "((1 2) \"*scratch*\" \"b\" 1)"
    );
    assert_eq!(
        run_last("(split-window) (display-buffer \"b\") (list (window-buffer 1) (window-buffer 2) (length (window-list)))"),
        "(\"*scratch*\" \"b\" 2)"
    );
    assert_eq!(
        run_last("(get-buffer-create \"c\") (with-current-buffer \"c\" (insert \"1\\n2\\n3\")) (display-buffer \"c\" 3) (with-current-buffer \"c\" (point))"),
        "5"
    );
    // 選んだウィンドウを出し直す
    assert_eq!(
        run_last("(pop-to-buffer-same-window \"d\") (list (current-buffer) (window-buffer) (buffer-list))"),
        "(\"d\" \"d\" (\"*scratch*\" \"d\"))"
    );
}

#[test]
fn test_special_forms_restore_state() {
    // 本体が失敗しても戻す
    assert_eq!(
        run_last("(get-buffer-create \"x\") (with-current-buffer \"x\" (car '())) (buffer-name)"),
        "ERR car expects a non-empty list"
    );
    assert_eq!(
        run_last("(get-buffer-create \"x\") (list (save-excursion (set-buffer \"x\") (goto-char 1) 1) (buffer-name) (point))"),
        "(1 \"*scratch*\" 1)"
    );
}

#[test]
fn test_operations_are_forwarded_to_the_ui() {
    assert_eq!(run_last("(read-key \"k\")"), "ERR ui: read-key");
    assert_eq!(run_last("(message \"hi\")"), "\"hi\"");
}

#[test]
fn test_moving_and_argument_checks() {
    assert_eq!(
        run_last("(insert \"abcd\") (goto-char 1) (forward-char 2) (forward-char) (backward-char) (backward-char 2) (point)"),
        "1"
    );
    assert_eq!(
        run_last("(looking-at 5)"),
        "ERR looking-at expects a string"
    );
    assert_eq!(
        run_last("(insert \"(a)\") (syntax-ppss)"),
        "(0 #f #f #f #f #f 0 #f #f () #f)"
    );
    // 開いている括弧と, 文字列の中
    assert_eq!(
        run_last("(insert \"(a (b \\\"c\") (syntax-ppss)"),
        "(2 4 #f #t #f #f 0 #f 7 (1 4) #f)"
    );
}

#[test]
fn test_undo_list_can_be_replaced_and_markers_follow_the_text() {
    assert_eq!(
        run_last("(set-buffer-undo-list 5)"),
        "ERR set-buffer-undo-list expects a list"
    );
    assert_eq!(
        run_last("(set-buffer-undo-list '(a))"),
        "ERR primitive-undo: invalid undo entry"
    );
    // 履歴を置き換える: 古い方の 1 つだけにする
    assert_eq!(
        run_last("(insert \"ab\") (undo-boundary) (insert \"cd\") (set-buffer-undo-list (list (list 1 3))) (buffer-undo-list)"),
        "((1 3))"
    );
    // 位置の記録 (数) は、履歴に入り, 元に戻すと, カーソルを置く
    assert_eq!(
        run_last("(insert \"abc\") (primitive-undo 1 '(2))  (point)"),
        "2"
    );
    // マーカーは、本文を書き換えると、位置を追う
    assert_eq!(
        run_last("(insert \"abcdef\") (goto-char 4) (let ((m (point-marker))) (goto-char 1) (insert \"XX\") (list (marker-position m) (begin (marker-release m) (marker-position m))))"),
        "(6 #f)"
    );
}

#[test]
fn test_global_mark_ring() {
    assert_eq!(run_last("(global-mark-pop)"), "ERR No global mark set");
    // 別のバッファのマークも履歴に入り, 取り出すごとに回る
    assert_eq!(
        run_last(
            "(insert \"abcdef\") (push-mark 3 #t)
             (get-buffer-create \"other\") (set-buffer \"other\") (insert \"xyz\") (push-mark 2 #t)
             (list (global-mark-pop) (global-mark-pop) (global-mark-pop))"
        ),
        "((\"other\" 2) (\"*scratch*\" 3) (\"other\" 2))"
    );
    // 消えたバッファのマークは捨てる
    assert_eq!(
        run_last(
            "(get-buffer-create \"gone\") (set-buffer \"gone\") (insert \"abc\") (push-mark 2 #t)
             (set-buffer \"*scratch*\") (kill-buffer \"gone\") (global-mark-pop)"
        ),
        "ERR No global mark set"
    );
}

#[test]
fn test_replace_match_and_set_match_data_like_emacs() {
    // 値は `emacs -Q --batch` の結果. 一致が無いときと, 無い括弧は, 同じエラー
    assert_eq!(
        run_last("(replace-match \"x\")"),
        "ERR replace-match subexpression does not exist"
    );
    // 括弧の番号は 5 つ目の引数 (4 つ目は STRING)
    let after_b = "(insert \"ab\") (goto-char 1) (re-search-forward \"\\\\(x\\\\)\\\\|b\") ";
    assert_eq!(
        run_last(&format!("{after_b}(replace-match \"x\" #f #f #f 1)")),
        "ERR replace-match subexpression does not exist"
    );
    assert_eq!(
        run_last(&format!(
            "{after_b}(replace-match \"x\" #f #f #f 0) (buffer-string)"
        )),
        "\"ax\""
    );
    assert_eq!(
        run_last(&format!("{after_b}(replace-match \"x\" #f #f \"ab\")")),
        "ERR replace-match: STRING is not supported"
    );
    // 余った要素は捨てる. 数でも #f でもない要素は, エラー
    assert_eq!(
        run_last("(insert \"abc\") (set-match-data '(1 2 3)) (match-data)"),
        "(1 2)"
    );
    assert_eq!(
        run_last("(insert \"abc\") (set-match-data '(1 2 #f #f)) (match-data)"),
        "(1 2)"
    );
    assert_eq!(
        run_last("(set-match-data '(1 2 a b))"),
        "ERR set-match-data expects a number"
    );
    assert_eq!(
        run_last("(set-match-data 5)"),
        "ERR set-match-data expects a list"
    );
    assert_eq!(run_last("(set-match-data '()) (match-data)"), "()");
}

#[test]
fn test_indent_sexp_stops_at_an_unfinished_list() {
    // 行の終わりが閉じていないリストの中: 読めるところまで (`emacs -Q --batch` では, 何も変わらない)
    for text in ["a (b", "a (b\\n  c"] {
        assert_eq!(
            run_last(&format!(
                "(set-syntax-dialect 'scheme) (insert \"{text}\") (goto-char 1) (indent-sexp-op #t) (list (buffer-string) (point))"
            )),
            format!("(\"{text}\" 1)")
        );
    }
}
