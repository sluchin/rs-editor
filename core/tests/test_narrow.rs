//! 絞り込み (`narrow-to-region`, `widen`, `save-restriction` など) のテスト. 期待値の一部は, `emacs -Q --batch` の結果.

use std::sync::Mutex;

use rseditor_core::buffer::Buffer;
use rseditor_core::editor::Editor;
use rseditor_core::scheme::editor_host::EditorHost;
use rseditor_core::scheme::evaluator::Evaluator;
use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::value::Value;
use rseditor_core::search::Pattern;
use rseditor_core::syntax::Dialect;

/// メッセージだけ受ける画面の偽物.
struct Quiet;

impl Host for Quiet {
    fn call(&self, op: &str, _args: Vec<Value>) -> Result<Value, String> {
        match op {
            "message" => Ok(Value::Nil),
            other => Err(format!("ui: {other}")),
        }
    }
}

/// 本文を入れたバッファで式を評価して, 最後の行を返す (失敗は `error: 理由`).
fn eval(text: &str, code: &str) -> String {
    let ev = Evaluator::new();
    let host = EditorHost::new(Editor::new(), Quiet);
    ev.eval_with(include_str!("../../lisp/simple.scm"), &host)
        .unwrap();
    host.editor().set_buffer("*scratch*").unwrap();
    host.editor().current_buffer_mut().load(text);
    match ev.eval_with(code, &host) {
        Ok(value) => value.lines().last().unwrap_or("").to_string(),
        Err(message) => format!("error: {message}"),
    }
}

/// `ab\ncde\n\nf` の 3 から 7 (`\ncde` の手前から `\n` の前まで) に絞り込んだバッファ.
fn narrowed() -> Buffer {
    let mut buffer = Buffer::with_text("n", "ab\ncde\n\nf");
    buffer.narrow_to_region(3, 6).unwrap();
    buffer
}

#[test]
fn test_narrow_and_widen() {
    let mut buffer = Buffer::with_text("n", "ab\ncde\n\nf");
    assert!(!buffer.narrowed());
    assert_eq!((buffer.begv(), buffer.zv()), (0, 9));
    buffer.set_point(8);
    // 始めと終わりは逆でもよい. カーソルは, 見える範囲に収める
    buffer.narrow_to_region(6, 3).unwrap();
    assert!(buffer.narrowed());
    assert_eq!((buffer.begv(), buffer.zv()), (3, 6));
    assert_eq!(buffer.point(), 6);
    assert_eq!(buffer.visible_text(), "cde");
    // バッファの大きさと全体の本文は, 絞り込みに関わらない
    assert_eq!(buffer.len(), 9);
    assert_eq!(buffer.text(), "ab\ncde\n\nf");
    assert_eq!(
        buffer.narrow_to_region(0, 10),
        Err("Args out of range".to_string())
    );
    buffer.widen();
    assert!(!buffer.narrowed());
    assert_eq!(buffer.visible_text(), "ab\ncde\n\nf");
    // 読み込むと, 絞り込みをやめる
    buffer.narrow_to_region(1, 2).unwrap();
    buffer.load("xyz");
    assert!(!buffer.narrowed());
}

#[test]
fn test_motion_stays_in_the_accessible_part() {
    let mut buffer = narrowed();
    buffer.set_point(0);
    assert_eq!(buffer.point(), 3);
    buffer.set_point(99);
    assert_eq!(buffer.point(), 6);
    assert!(buffer.eobp() && !buffer.bobp());
    assert!(buffer.eolp());
    buffer.set_point(3);
    assert!(buffer.bobp() && buffer.bolp() && !buffer.eobp());
    assert_eq!(buffer.char_after(3), Some('c'));
    assert_eq!(buffer.char_after(2), None);
    assert_eq!(buffer.char_after(6), None);
    assert_eq!(buffer.char_before(3), None);
    assert_eq!(buffer.char_before(6), Some('e'));
    assert_eq!(buffer.char_before(7), None);
    assert_eq!(
        buffer.forward_char(-1),
        Err("Beginning of buffer".to_string())
    );
    assert_eq!(buffer.point(), 3);
    assert_eq!(buffer.forward_char(4), Err("End of buffer".to_string()));
    assert_eq!(buffer.point(), 6);
    assert_eq!(buffer.forward_char(-3), Ok(()));
    assert_eq!(buffer.point(), 3);
}

#[test]
fn test_lines_in_the_accessible_part() {
    // 絞り込みの始めが行の途中: 見える範囲の最初の行は, 始めから
    let mut buffer = Buffer::with_text("n", "ab\ncde\n\nf");
    buffer.narrow_to_region(1, 5).unwrap();
    buffer.set_point(4);
    assert_eq!(buffer.line_beginning_position(1), 3);
    assert_eq!(buffer.line_end_position(1), 5);
    assert_eq!(buffer.line_beginning_position(0), 1);
    assert_eq!(buffer.line_end_position(0), 2);
    assert_eq!(buffer.line_end_position(2), 5);
    assert_eq!(buffer.line_beginning_position(-5), 1);
    assert_eq!(buffer.line_end_position(-5), 1);
    assert_eq!(buffer.current_column(), 1);
    // 前へ: 見える範囲の始めで止まる
    assert_eq!(buffer.forward_line(-1), 0);
    assert_eq!(buffer.point(), 1);
    assert_eq!(buffer.forward_line(-1), -1);
    assert_eq!(buffer.point(), 1);
    // 後ろへ: 見える範囲の終わりまで (終わりに改行が無い行は, 動けた 1 行)
    assert_eq!(buffer.forward_line(1), 0);
    assert_eq!(buffer.point(), 3);
    assert_eq!(buffer.forward_line(1), 0);
    assert_eq!(buffer.point(), 5);
    assert_eq!(buffer.forward_line(1), 1);
    assert_eq!(buffer.point(), 5);
    buffer.set_point(1);
    assert_eq!(buffer.current_column(), 0);
    assert_eq!(buffer.move_to_column(5), 1);
}

#[test]
fn test_editing_is_limited_to_the_accessible_part() {
    let mut buffer = narrowed();
    assert_eq!(
        buffer.delete_region(0, 4),
        Err("Args out of range".to_string())
    );
    assert_eq!(buffer.substring(3, 8), Err("Args out of range".to_string()));
    assert_eq!(
        buffer.insert_at(2, "x"),
        Err("Args out of range".to_string())
    );
    assert_eq!(
        buffer.insert_at(7, "x"),
        Err("Args out of range".to_string())
    );
    assert_eq!(buffer.substring(6, 4).unwrap(), "de");
    // 終わりに入れた文字は, 見える範囲に入る. 始めに入れた文字も入る (始めは動かない)
    buffer.insert_at(6, "!").unwrap();
    buffer.insert_at(3, "?").unwrap();
    assert_eq!(buffer.visible_text(), "?cde!");
    assert_eq!((buffer.begv(), buffer.zv()), (3, 8));
    buffer.delete_region(4, 6).unwrap();
    assert_eq!(buffer.visible_text(), "?e!");
    assert_eq!((buffer.begv(), buffer.zv()), (3, 6));
    // 範囲の置き換えも, 見える範囲の中だけ
    buffer.replace_region(3, 4, "XY").unwrap();
    assert_eq!(buffer.visible_text(), "XYe!");
    assert_eq!(buffer.zv(), 7);
    assert!(buffer.replace_region(0, 3, "").is_err());
    // 全体の本文では, 見えない部分は変わらない
    assert_eq!(buffer.text(), "ab\nXYe!\n\nf");
}

#[test]
fn test_save_and_restore_the_restriction() {
    let mut buffer = narrowed();
    let first = buffer.save_restriction();
    buffer.widen();
    let second = buffer.save_restriction();
    assert_ne!(first, second);
    // 覚えた範囲は, 本文の書き換えに付いていく
    buffer.insert_at(0, "123").unwrap();
    buffer.insert_at(12, "456").unwrap();
    buffer.delete_region(1, 2).unwrap();
    buffer.restore_restriction(first);
    assert_eq!((buffer.begv(), buffer.zv()), (5, 8));
    assert_eq!(buffer.visible_text(), "cde");
    // 覚えたものは, 戻すと解く. 空いた番号を使い回し, 無い番号と戻し済みの番号は何もしない
    buffer.restore_restriction(first);
    buffer.restore_restriction(99);
    assert_eq!((buffer.begv(), buffer.zv()), (5, 8));
    // 絞り込んでいなかったときに戻すと, 絞り込みをやめる
    buffer.restore_restriction(second);
    assert!(!buffer.narrowed());
    assert_eq!(buffer.save_restriction(), first.min(second));
    // 戻すと, カーソルは見える範囲に収まる
    buffer.set_point(0);
    let again = buffer.save_restriction();
    buffer.narrow_to_region(2, 4).unwrap();
    buffer.restore_restriction(again);
    assert!(!buffer.narrowed());
    assert_eq!(buffer.point(), 2);
    let wide = buffer.save_restriction();
    buffer.narrow_to_region(2, 4).unwrap();
    let narrow = buffer.save_restriction();
    buffer.widen();
    buffer.set_point(10);
    buffer.restore_restriction(narrow);
    assert_eq!(buffer.point(), 4);
    buffer.restore_restriction(wide);
}

#[test]
fn test_search_stays_in_the_accessible_part() {
    let pattern = |s: &str| Pattern::new(s, false, Dialect::Fundamental).unwrap();
    let mut buffer = Buffer::with_text("n", "ab\ncde\ncde\nf");
    buffer.narrow_to_region(3, 10).unwrap();
    buffer.set_point(3);
    // 後ろへ: 見える範囲の終わりを越えて探さない
    assert_eq!(buffer.re_search_forward(&pattern("cde"), None, 2), Some(10));
    buffer.set_point(3);
    assert_eq!(buffer.re_search_forward(&pattern("f"), None, 1), None);
    assert_eq!(
        buffer.re_search_forward(&pattern("cde"), Some(99), 1),
        Some(6)
    );
    // 前へ: 見える範囲の始めを越えて探さない
    buffer.set_point(10);
    assert_eq!(buffer.re_search_backward(&pattern("ab"), None, 1), None);
    assert_eq!(buffer.re_search_backward(&pattern("cde"), None, 2), Some(3));
    buffer.set_point(10);
    assert_eq!(
        buffer.re_search_backward(&pattern("cde"), Some(0), 1),
        Some(7)
    );
    // 今の所から合うか: 見える範囲の終わりまで
    buffer.set_point(8);
    assert!(buffer.looking_at(&pattern("de")));
    assert!(!buffer.looking_at(&pattern("def")));
    buffer.narrow_to_region(3, 9).unwrap();
    buffer.set_point(7);
    assert!(!buffer.looking_at(&pattern("cde")));
}

#[test]
fn test_skip_chars_stays_in_the_accessible_part() {
    use rseditor_core::search::CharSet;
    let all = CharSet::parse("a-z").unwrap();
    let mut buffer = Buffer::with_text("n", "abcdef");
    buffer.narrow_to_region(1, 4).unwrap();
    buffer.set_point(1);
    assert_eq!(buffer.skip_chars(&all, true, None), 3);
    assert_eq!(buffer.skip_chars(&all, false, None), -3);
    assert_eq!(buffer.skip_chars(&all, true, Some(99)), 3);
    assert_eq!(buffer.skip_chars(&all, false, Some(0)), -3);
}

#[test]
fn test_lisp_primitives() {
    let text = "ab\ncde\n\nf";
    let code = |tail: &str| format!("(narrow-to-region 4 7) {tail}");
    assert_eq!(eval(text, &code("(list (point-min) (point-max))")), "(4 7)");
    assert_eq!(eval(text, &code("(buffer-string)")), "\"cde\"");
    assert_eq!(eval(text, &code("(buffer-size)")), "9");
    assert_eq!(eval(text, &code("(buffer-narrowed-p)")), "#t");
    assert_eq!(eval(text, "(buffer-narrowed-p)"), "#f");
    assert_eq!(eval(text, &code("(widen) (buffer-narrowed-p)")), "#f");
    assert_eq!(eval(text, &code("(goto-char 1) (point)")), "4");
    assert_eq!(eval(text, &code("(goto-char 99) (point)")), "7");
    assert_eq!(
        eval(text, &code("(delete-char -1)")),
        "error: Beginning of buffer"
    );
    assert_eq!(
        eval(text, &code("(goto-char 7) (delete-char 1)")),
        "error: End of buffer"
    );
    assert_eq!(
        eval(text, &code("(goto-char 4) (delete-char 3) (buffer-string)")),
        "\"\""
    );
    assert_eq!(
        eval(text, &code("(erase-buffer) (list (point-min) (point-max))")),
        "(1 1)"
    );
    assert_eq!(
        eval(text, &code("(goto-char 4) (search-forward \"f\" #f #t)")),
        "#f"
    );
    assert_eq!(
        eval(
            text,
            &code("(goto-char 4) (search-forward \"f\" #f 1) (point)")
        ),
        "7"
    );
    assert_eq!(
        eval(
            text,
            &code("(goto-char 7) (search-backward \"a\" #f 1) (point)")
        ),
        "4"
    );
    assert_eq!(
        eval(text, "(narrow-to-region 8 99)"),
        "error: Args out of range"
    );
    // 絞り込んだ所の外の式は, 走査しない (括弧が合わない)
    assert_eq!(
        eval(
            "(a (b) c) (d)",
            "(narrow-to-region 4 7) (goto-char 4) (scan-lists 4 1 1)"
        ),
        "error: Scan error: \"Unbalanced parentheses\", 4, 7"
    );
}

#[test]
fn test_narrow_commands() {
    let text = "ab\ncde\n\nf";
    // リージョンに絞り込む (マークを使う状態のまま)
    assert_eq!(
        eval(
            text,
            "(goto-char 4) (set-mark 7) (narrow-to-region) (list (point-min) (point-max) (region-active-p))"
        ),
        "(4 7 #t)"
    );
    assert_eq!(
        eval(text, "(narrow-to-region)"),
        "error: The mark is not set now, so there is no region"
    );
    assert_eq!(
        eval(text, "(narrow-to-region 2 3) (widen) (buffer-narrowed-p)"),
        "#f"
    );
}

#[test]
fn test_save_restriction_form() {
    let text = "ab\ncde\n\nf";
    assert_eq!(
        eval(
            text,
            "(narrow-to-region 4 7) (save-restriction (widen) (goto-char 1)) (list (point-min) (point-max) (point))"
        ),
        "(4 7 4)"
    );
    // 絞り込んでいなければ, 本体の絞り込みを戻す
    assert_eq!(
        eval(
            text,
            "(save-restriction (narrow-to-region 4 7) (insert \"xy\")) (list (point-min) (point-max) (buffer-string))"
        ),
        "(1 12 \"ab\\nxycde\\n\\nf\")"
    );
    // 本体が失敗しても戻す
    assert_eq!(
        eval(
            text,
            "(narrow-to-region 4 7) (save-restriction (widen) (car 1)) (buffer-narrowed-p)"
        ),
        "error: car expects a non-empty list"
    );
    assert_eq!(
        eval(
            text,
            "(narrow-to-region 4 7) (unwind-protect (save-restriction (widen) (car 1)) #t) 1"
        ),
        "error: car expects a non-empty list"
    );
    // 値は, 本体の最後の値
    assert_eq!(eval(text, "(save-restriction 1 2)"), "2");
    // 本体で別のバッファに移っても, 覚えたバッファの絞り込みを戻し, 移った先に残る
    assert_eq!(
        eval(
            text,
            "(narrow-to-region 4 7) (save-restriction (widen) (set-buffer (get-buffer-create \"other\"))) (list (buffer-name) (with-current-buffer \"*scratch*\" (list (point-min) (point-max))))"
        ),
        "(\"other\" (4 7))"
    );
}

/// 絞り込みを覚えられない画面 (Tauri) の偽物: バッファの名前だけ答える.
struct NoRestriction {
    /// 呼ばれた操作.
    calls: Mutex<Vec<String>>,
    /// バッファを戻せないか.
    gone: bool,
}

impl Host for NoRestriction {
    fn call(&self, op: &str, _args: Vec<Value>) -> Result<Value, String> {
        self.calls.lock().unwrap().push(op.to_string());
        match op {
            "current-buffer" => Ok(Value::String("a".to_string())),
            "restriction-save" => Ok(Value::Number(0.0)),
            "set-buffer" if self.gone => Err("No such buffer".to_string()),
            _ => Err(format!("ui: {op}")),
        }
    }
}

#[test]
fn test_save_restriction_without_the_buffer() {
    // 覚えたバッファを消していれば, 戻さない
    let host = NoRestriction {
        calls: Mutex::new(Vec::new()),
        gone: true,
    };
    let ev = Evaluator::new();
    assert_eq!(ev.eval_with("(save-restriction 5)", &host).unwrap(), "5");
    assert!(!host
        .calls
        .lock()
        .unwrap()
        .contains(&"restriction-restore".to_string()));
    // 画面が絞り込みを覚えられなければ, 本体だけを評価する
    struct NoSave;
    impl Host for NoSave {
        fn call(&self, op: &str, _args: Vec<Value>) -> Result<Value, String> {
            match op {
                "current-buffer" => Ok(Value::String("a".to_string())),
                _ => Err(format!("ui: {op}")),
            }
        }
    }
    assert_eq!(ev.eval_with("(save-restriction 6)", &NoSave).unwrap(), "6");
}

#[test]
fn test_goto_line_with_a_narrowed_buffer() {
    // 期待値は, `emacs -Q --batch` の結果
    let text = "ab\ncde\n\nf";
    let narrow = "(narrow-to-region 4 7) ";
    // 絞り込みの外の行へ行くと, 絞り込みをやめる
    assert_eq!(
        eval(
            text,
            &format!("{narrow}(goto-line 1) (list (point) (buffer-narrowed-p) (point-min))")
        ),
        "(1 #f 1)"
    );
    assert_eq!(
        eval(
            text,
            &format!("{narrow}(goto-line 2) (list (point) (buffer-narrowed-p))")
        ),
        "(4 #t)"
    );
    assert_eq!(
        eval(
            text,
            &format!("{narrow}(goto-line 4) (list (point) (buffer-narrowed-p))")
        ),
        "(9 #f)"
    );
    // 相対なら, 見える範囲の始めから数える
    assert_eq!(
        eval(
            text,
            &format!(
                "{narrow}(goto-char 6) (goto-line-relative 1) (list (point) (buffer-narrowed-p))"
            )
        ),
        "(4 #t)"
    );
    // 引数を省くと, 前置引数で決める
    assert_eq!(
        eval(text, "(set-current-prefix-arg 2) (goto-line) (point)"),
        "4"
    );
}

#[test]
fn test_narrow_to_defun_with_comments() {
    // 期待値は, `emacs -Q --batch` の結果
    let text = "x\n\u{c}\u{3b}\u{3b} q\n\n(a b)\n(c)\n";
    let run = |tail: &str| {
        eval(
            text,
            &format!(
                "(set-syntax-dialect 'elisp) (goto-char 12) {tail} (list (point-min) (point-max))"
            ),
        )
    };
    assert_eq!(run("(narrow-to-defun)"), "(10 16)");
    assert_eq!(run("(narrow-to-defun #t)"), "(3 16)");
    assert_eq!(
        run("(set! narrow-to-defun-include-comments #t) (narrow-to-defun)"),
        "(3 16)"
    );
    assert_eq!(
        eval(
            text,
            "(set-syntax-dialect 'elisp) (narrow-to-defun #t) (list (point-min) (point-max))"
        ),
        "(1 3)"
    );
}

/// メッセージを覚える画面の偽物.
#[derive(Default)]
struct Recording {
    /// 最後のメッセージ.
    message: Mutex<Option<String>>,
}

impl Host for Recording {
    fn call(&self, op: &str, args: Vec<Value>) -> Result<Value, String> {
        match (op, args.first()) {
            ("message", Some(Value::String(text))) => {
                *self.message.lock().unwrap() = Some(text.clone());
                Ok(Value::Nil)
            }
            _ => Err(format!("ui: {op}")),
        }
    }
}

#[test]
fn test_what_cursor_position_shows_the_accessible_range() {
    // 期待値は, `emacs -Q --batch` の結果
    let ev = Evaluator::new();
    let ui = Recording::default();
    let host = EditorHost::new(Editor::new(), ui);
    ev.eval_with(include_str!("../../lisp/simple.scm"), &host)
        .unwrap();
    host.editor().set_buffer("*scratch*").unwrap();
    host.editor().current_buffer_mut().load("ab\ncde\n\nf");
    let message = |code: &str| {
        ev.eval_with(code, &host).unwrap();
        host.ui().message.lock().unwrap().clone().unwrap()
    };
    assert_eq!(
        message("(narrow-to-region 4 7) (goto-char 5) (what-cursor-position)"),
        "Char: d (100, #o144, #x64) point=5 of 9 (44%) <4-7> column=1"
    );
    assert_eq!(
        message("(goto-char 7) (what-cursor-position)"),
        "point=7 of 9 (67%) <4-7> column=3"
    );
    assert_eq!(
        message("(widen) (goto-char 10) (what-cursor-position)"),
        "point=10 of 9 (EOB) column=1"
    );
}
