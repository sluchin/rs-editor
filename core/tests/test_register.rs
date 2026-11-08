//! レジスタの命令 (`lisp/register.scm`) のテスト. 値を入れて取り出す命令の期待値は, `emacs -Q --batch` の結果
//! (`core/tests/edit.json`. `core/tests/test_edit.rs` で比べる). ここは, 画面の要る命令を試す.

use rseditor_core::editor::Editor;
use rseditor_core::scheme::editor_host::EditorHost;
use rseditor_core::scheme::evaluator::Evaluator;
use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::value::Value;

/// メッセージだけ受ける画面の偽物.
struct Quiet;

impl Host for Quiet {
    fn call(&self, op: &str, _args: Vec<Value>) -> Result<Value, rseditor_core::error::Error> {
        match op {
            "message" => Ok(Value::Nil),
            // 打ったことにするキーは, 文字列にして返す
            "execute-kbd-macro" => Ok(Value::Nil),
            other => Err(format!("ui: {other}").into()),
        }
    }
}

/// 式を評価して, 最後の行を返す (失敗は `error: 理由`).
fn eval(code: &str) -> String {
    let ev = Evaluator::new();
    let host = EditorHost::new(Editor::new(), Quiet);
    for source in [
        include_str!("../../lisp/simple.scm"),
        include_str!("../../lisp/rect.scm"),
        include_str!("../../lisp/register.scm"),
    ] {
        ev.eval_with(source, &host).unwrap();
    }
    match ev.eval_with(code, &host) {
        Ok(value) => value.lines().last().unwrap_or("").to_string(),
        Err(message) => format!("error: {message}"),
    }
}

#[test]
fn test_window_configuration_register() {
    // ウィンドウの設定と, そのときのカーソルを戻す
    assert_eq!(
        eval(
            "(insert \"hello world\") (goto-char 3) (split-window-below) (window-configuration-to-register \"w\")
             (delete-other-windows) (goto-char 8) (jump-to-register \"w\")
             (list (length (window-list)) (point))"
        ),
        "(2 3)"
    );
    // 覚えたバッファが無ければ, カーソルは戻さない
    assert_eq!(
        eval(
            "(set-buffer (get-buffer-create \"tmp\")) (window-configuration-to-register \"w\")
             (set-buffer \"*scratch*\") (kill-buffer \"tmp\") (jump-to-register \"w\") 1"
        ),
        "1"
    );
}

#[test]
fn test_point_register_across_buffers() {
    // 別のバッファから戻ると, そのバッファに移る. カーソルが動くなら, マークを置く
    assert_eq!(
        eval(
            "(insert \"abcdef\") (goto-char 3) (point-to-register \"p\") (goto-char 5) (switch-to-buffer \"other\")
             (jump-to-register \"p\") (list (buffer-name) (point) (mark))"
        ),
        "(\"*scratch*\" 3 5)"
    );
    // 続けて飛ぶときは, マークを置かない
    assert_eq!(
        eval(
            "(insert \"abcdef\") (goto-char 3) (point-to-register \"p\") (goto-char 5) (set-mark #f)
             (set! last-command 'jump-to-register) (jump-to-register \"p\") (list (point) (mark))"
        ),
        "(3 #f)"
    );
    // バッファが無くなっていれば, 失敗する
    assert_eq!(
        eval(
            "(switch-to-buffer \"tmp\") (point-to-register \"p\") (switch-to-buffer \"*scratch*\")
             (kill-buffer \"tmp\") (jump-to-register \"p\")"
        ),
        "error: That register's buffer no longer exists"
    );
    // 数や文字の入ったレジスタには飛べない
    assert_eq!(
        eval("(set-register \"n\" 5) (jump-to-register \"n\")"),
        "error: Register doesn’t contain a buffer position or configuration"
    );
}

#[test]
fn test_increment_register_errors() {
    assert_eq!(
        eval("(point-to-register \"p\") (increment-register 1 \"p\")"),
        "error: Register does not contain a number or text"
    );
    assert_eq!(
        eval("(point-to-register \"p\") (append-to-register \"p\" 1 1)"),
        "error: Register does not contain text"
    );
}

#[test]
fn test_kmacro_to_register_stores_and_runs_the_macro() {
    assert_eq!(
        eval("(kmacro-to-register \"m\")"),
        "error: No keyboard macro defined"
    );
    assert_eq!(
        eval(
            "(set! last-kbd-macro '(\"a\" \"b\")) (kmacro-to-register \"m\") (get-register \"m\")"
        ),
        "(kbd-macro (\"a\" \"b\"))"
    );
    // 戻ると, マクロを動かす (動かすのは画面なので, ここでは, 失敗しない所まで)
    assert_eq!(
        eval(
            "(set! last-kbd-macro '(\"a\")) (kmacro-to-register \"m\") (jump-to-register \"m\") 7"
        ),
        "7"
    );
}

#[test]
fn test_rectangle_mark_mode_kills_and_saves_the_rectangle() {
    let setup = "(insert \"abcd\\nefgh\\nijkl\\n\") (goto-char 2) (push-mark (point)) (activate-mark) (goto-char 13) ";
    // 切る: 2 列目の 1 文字, 3 行 (消すのを頼むだけなので, 領域はまだ生きている)
    assert_eq!(
        eval(&format!(
            "{setup} (rectangle-mark-mode) (kill-region) (list (buffer-string) killed-rectangle rectangle-mark-mode--on (region-active-p))"
        )),
        "(\"acd\\negh\\nikl\\n\" (\"b\" \"f\" \"j\") #f #t)"
    );
    // 写す: 本文は変わらない
    assert_eq!(
        eval(&format!(
            "{setup} (rectangle-mark-mode) (kill-ring-save) (list (buffer-string) killed-rectangle rectangle-mark-mode--on)"
        )),
        "(\"abcd\\nefgh\\nijkl\\n\" (\"b\" \"f\" \"j\") #f)"
    );
    // 引数で入り切りし, 切ると普通の領域に戻る
    assert_eq!(
        eval(&format!(
            "{setup} (rectangle-mark-mode 1) (rectangle-mark-mode 0) (kill-region) (buffer-string)"
        )),
        "\"akl\\n\""
    );
    // もう一度で切る. 領域が無ければ, 印を付けて入る
    assert_eq!(
        eval(&format!(
            "{setup} (rectangle-mark-mode) (rectangle-mark-mode) rectangle-mark-mode--on"
        )),
        "#f"
    );
    assert_eq!(
        eval("(rectangle-mark-mode) (list rectangle-mark-mode--on (region-active-p))"),
        "(#t #t)"
    );
}
