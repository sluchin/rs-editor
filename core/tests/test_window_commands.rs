//! ウィンドウのコマンド (`lisp/window.scm`) のテスト.
//! バッチの Emacs には窓が 1 つしか無いので, Emacs との比べは無く, ここで窓の動きを試す.

use std::collections::VecDeque;
use std::sync::Mutex;

use rseditor_core::editor::Editor;
use rseditor_core::scheme::editor_host::EditorHost;
use rseditor_core::scheme::evaluator::Evaluator;
use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::value::Value;

/// 答えを並べる画面の偽物 (窓の高さは 5 行).
#[derive(Default)]
struct Ui {
    answers: Mutex<VecDeque<String>>,
}

impl Host for Ui {
    fn call(&self, op: &str, _args: Vec<Value>) -> Result<Value, String> {
        match op {
            "message" => Ok(Value::Nil),
            "window-body-height" => Ok(Value::Number(5.0)),
            "recenter" => Ok(Value::Nil),
            "read-string" | "read-key" | "read-file-name" | "completing-read" => self
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

/// 式を評価して, 最後の行を返す (失敗は `error: 理由`).
fn eval_with(answers: &[&str], code: &str) -> String {
    let ev = Evaluator::new();
    let mut editor = Editor::new();
    // 窓の高さは 24 行 (最後の 1 行は, ミニバッファ), 幅は 80 桁
    editor.set_screen_size(25, 80);
    let host = EditorHost::new(editor, Ui::default());
    host.ui()
        .answers
        .lock()
        .unwrap()
        .extend(answers.iter().map(|a| a.to_string()));
    ev.eval_with(include_str!("../../lisp/simple.scm"), &host)
        .unwrap();
    match ev.eval_with(code, &host) {
        Ok(value) => value.lines().last().unwrap_or("").to_string(),
        Err(message) => format!("error: {message}"),
    }
}

fn eval(code: &str) -> String {
    eval_with(&[], code)
}

/// 本文の 30 行を持つバッファ `big` を 2 番目の窓に出した状態を作る式 (選んでいる窓は 1 番目で, `*scratch*` を出す).
const TWO_WINDOWS: &str = "(set-buffer (get-buffer-create \"big\")) \
    (let loop ((i 0)) (if (< i 30) (begin (insert (format \"line%d\\n\" i)) (loop (+ i 1))))) \
    (goto-char 1) (set-buffer \"*scratch*\") \
    (split-window 'below) (set-window-buffer (car (cdr (window-list))) \"big\")";

/// 2 番目の窓のバッファの, 点の行 (1 から).
const OTHER_LINE: &str = "(with-current-buffer \"big\" \
    (length (split-string (buffer-substring 1 (point)) \"\\n\")))";

#[test]
fn test_switch_to_buffer_other_window_and_display_buffer() {
    assert_eq!(
        eval(
            "(switch-to-buffer-other-window \"other\") \
             (list (buffer-name) (length (window-list)) (= (selected-window) (car (window-list))))"
        ),
        "(\"other\" 2 #f)"
    );
    // 名前を読む (既定は別のバッファ)
    assert_eq!(
        eval_with(&["foo"], "(switch-to-buffer-other-window) (buffer-name)"),
        "\"foo\""
    );
    assert_eq!(
        eval_with(
            &[""],
            "(switch-to-buffer-other-window) (length (window-list))"
        ),
        "1"
    );
    // display-buffer は, 選んだ窓を変えない
    assert_eq!(
        eval(
            "(display-buffer \"shown\") \
             (list (buffer-name) (length (window-list)) (window-buffer (car (cdr (window-list)))))"
        ),
        "(\"*scratch*\" 2 \"shown\")"
    );
    assert_eq!(
        eval_with(
            &["asked"],
            "(display-buffer) (window-buffer (car (cdr (window-list))))"
        ),
        "\"asked\""
    );
    assert_eq!(
        eval_with(&[""], "(display-buffer) (length (window-list))"),
        "1"
    );
}

#[test]
fn test_next_and_previous_buffer() {
    assert_eq!(eval("(next-buffer)"), "error: No other buffer to show");
    assert_eq!(
        eval(
            "(get-buffer-create \"b\") (get-buffer-create \" hidden\") (get-buffer-create \"c\") \
             (next-buffer) (list (buffer-name) (begin (next-buffer) (buffer-name)) \
             (begin (next-buffer) (buffer-name)) (begin (previous-buffer) (buffer-name)))"
        ),
        "(\"b\" \"c\" \"*scratch*\" \"c\")"
    );
    assert_eq!(
        eval("(get-buffer-create \"b\") (get-buffer-create \"c\") (next-buffer 2) (buffer-name)"),
        "\"c\""
    );
}

#[test]
fn test_delete_windows_on_kill_buffer_and_quit_window() {
    assert_eq!(
        eval(&format!(
            "{TWO_WINDOWS} (delete-windows-on \"big\") (list (length (window-list)) (buffer-name))"
        )),
        "(1 \"*scratch*\")"
    );
    // 窓が 1 つなら, 別のバッファを出す
    assert_eq!(
        eval(
            "(get-buffer-create \"b\") (switch-to-buffer \"b\") (delete-windows-on \"b\") (buffer-name)"
        ),
        "\"*scratch*\""
    );
    // 出していないバッファは, 何もしない
    assert_eq!(
        eval(&format!(
            "{TWO_WINDOWS} (get-buffer-create \"x\") (delete-windows-on \"x\") (length (window-list))"
        )),
        "2"
    );
    // 名前を読む. 空なら, 今のバッファ
    assert_eq!(
        eval_with(
            &[""],
            &format!("{TWO_WINDOWS} (delete-windows-on) (length (window-list))")
        ),
        "1"
    );
    assert_eq!(
        eval(&format!(
            "{TWO_WINDOWS} (kill-buffer-and-window) (list (length (window-list)) (buffer-name) (get-buffer \"*scratch*\"))"
        )),
        "(1 \"big\" #f)"
    );
    // 窓が 1 つなら, バッファだけ消し, 窓は消せない
    assert_eq!(
        eval("(get-buffer-create \"b\") (switch-to-buffer \"b\") (kill-buffer-and-window)"),
        "error: Attempt to delete minibuffer or sole ordinary window"
    );
    assert_eq!(
        eval(&format!(
            "{TWO_WINDOWS} (quit-window) (list (length (window-list)) (get-buffer \"big\"))"
        )),
        "(1 \"big\")"
    );
    assert_eq!(
        eval("(get-buffer-create \"b\") (switch-to-buffer \"b\") (quit-window) (buffer-name)"),
        "\"*scratch*\""
    );
    assert_eq!(
        eval(&format!(
            "{TWO_WINDOWS} (quit-window '(4)) (list (length (window-list)) (get-buffer \"*scratch*\"))"
        )),
        "(1 #f)"
    );
}

#[test]
fn test_scroll_other_window() {
    // 次の窓の点だけが動き, 選んだ窓は元のまま
    assert_eq!(
        eval(&format!(
            "{TWO_WINDOWS} (scroll-other-window) \
             (list (selected-window) (buffer-name) {OTHER_LINE})"
        )),
        "(1 \"*scratch*\" 4)"
    );
    assert_eq!(
        eval(&format!(
            "{TWO_WINDOWS} (scroll-other-window 7) (scroll-other-window-down 2) \
             {OTHER_LINE}"
        )),
        "6"
    );
    // 負の数と `-` は逆向き
    assert_eq!(
        eval(&format!(
            "{TWO_WINDOWS} (scroll-other-window 7) (scroll-other-window -2) \
             {OTHER_LINE}"
        )),
        "6"
    );
    assert_eq!(
        eval(&format!(
            "{TWO_WINDOWS} (scroll-other-window 9) (scroll-other-window '-) \
             (scroll-other-window-down '-) (scroll-other-window-down) \
             {OTHER_LINE}"
        )),
        "7"
    );
    // 端での失敗
    assert_eq!(
        eval(&format!("{TWO_WINDOWS} (scroll-other-window-down 1)")),
        "error: Beginning of buffer"
    );
    assert_eq!(
        eval(&format!("{TWO_WINDOWS} (scroll-other-window -1)")),
        "error: Beginning of buffer"
    );
    assert_eq!(
        eval(&format!(
            "{TWO_WINDOWS} (end-of-buffer-other-window) (scroll-other-window 1)"
        )),
        "error: End of buffer"
    );
    assert_eq!(
        eval(&format!(
            "{TWO_WINDOWS} (end-of-buffer-other-window) (scroll-other-window-down -1)"
        )),
        "error: End of buffer"
    );
    assert_eq!(
        eval("(scroll-other-window)"),
        "error: There is no other window"
    );
}

#[test]
fn test_buffer_ends_and_recenter_in_other_window() {
    assert_eq!(
        eval(&format!(
            "{TWO_WINDOWS} (end-of-buffer-other-window) \
             (list (point) (with-current-buffer \"big\" (point)))"
        )),
        "(1 201)"
    );
    assert_eq!(
        eval(&format!(
            "{TWO_WINDOWS} (end-of-buffer-other-window) (beginning-of-buffer-other-window) \
             (with-current-buffer \"big\" (point))"
        )),
        "1"
    );
    assert_eq!(
        eval(&format!(
            "{TWO_WINDOWS} (recenter-other-window) (selected-window)"
        )),
        "1"
    );
}

#[test]
fn test_split_root_and_balance_windows() {
    assert_eq!(
        eval(
            "(split-window-right) (split-root-window-below) \
             (list (length (window-list)) (selected-window) (window-total-height 3) (window-total-width 3))"
        ),
        "(3 1 12 80)"
    );
    assert_eq!(
        eval(
            "(split-window-below) (split-root-window-right) \
             (list (length (window-list)) (window-total-height 3) (window-total-width 3))"
        ),
        "(3 24 40)"
    );
    assert_eq!(
        eval("(split-window-below) (enlarge-window 6) (balance-windows) (window-total-height)"),
        "12"
    );
}

#[test]
fn test_enlarge_and_shrink_windows() {
    assert_eq!(
        eval(
            "(split-window-below) (let ((before (window-total-height))) \
             (enlarge-window 4) (list before (window-total-height) (window-total-height 2)))"
        ),
        "(12 16 8)"
    );
    assert_eq!(
        eval("(split-window-below) (enlarge-window 4) (shrink-window 2) (window-total-height)"),
        "14"
    );
    // 前置引数が無ければ 1 行. 負なら逆向き
    assert_eq!(
        eval("(split-window-below) (enlarge-window) (window-total-height)"),
        "13"
    );
    assert_eq!(
        eval("(split-window-below) (enlarge-window -2) (window-total-height)"),
        "10"
    );
    assert_eq!(
        eval(
            "(split-window-right) (enlarge-window-horizontally 10) (list (window-total-width) (window-total-width 2))"
        ),
        "(50 30)"
    );
    assert_eq!(
        eval("(split-window-right) (shrink-window-horizontally 20) (window-total-width)"),
        "20"
    );
    // 分けていない向きは, 変えられない
    assert_eq!(eval("(enlarge-window 1)"), "error: Cannot resize window 1");
    assert_eq!(
        eval("(split-window-right) (enlarge-window 1)"),
        "error: Cannot resize window 1"
    );
}

#[test]
fn test_fit_window_to_buffer() {
    // 空のバッファは, 最小の高さ (window-min-height) まで縮める
    assert_eq!(
        eval("(split-window-below) (shrink-window-if-larger-than-buffer) (window-total-height)"),
        "4"
    );
    assert_eq!(
        eval("(split-window-below) (fit-window-to-buffer) (fit-window-to-buffer) (window-total-height)"),
        "4"
    );
    // 窓が 1 つなら, 何もしない
    assert_eq!(
        eval("(shrink-window-if-larger-than-buffer) (fit-window-to-buffer) (window-total-height)"),
        "24"
    );
    // バッファの方が大きければ, shrink は何もせず, fit は広げる
    assert_eq!(
        eval(&format!(
            "{TWO_WINDOWS} (select-window 2) (shrink-window-if-larger-than-buffer) (window-total-height)"
        )),
        "12"
    );
    assert_eq!(
        eval(&format!(
            "{TWO_WINDOWS} (select-window 2) (fit-window-to-buffer) (window-total-height)"
        )),
        "23"
    );
    // 最後の改行は, 行を増やさない
    assert_eq!(
        eval(
            "(insert \"a\\nb\\n\\n\\n\\n\\n\") (split-window-below) (fit-window-to-buffer) (window-total-height)"
        ),
        "7"
    );
}
