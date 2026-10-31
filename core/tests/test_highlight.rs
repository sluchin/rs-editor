//! `highlight-regexp` と, hi-lock のコマンド (`lisp/replace.scm`) と, 強調する範囲 (`Buffer::highlighted_spans`) のテスト.

use rseditor_core::editor::Editor;
use rseditor_core::scheme::editor_host::EditorHost;
use rseditor_core::scheme::evaluator::Evaluator;
use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::value::Value;

/// 何も受けない偽の画面.
struct NoUi;

impl Host for NoUi {
    fn call(&self, op: &str, _args: Vec<Value>) -> Result<Value, String> {
        match op {
            "message" => Ok(Value::Nil),
            other => Err(format!("ui: {other}")),
        }
    }
}

/// 基本のコマンドと, 置換のコマンドを読んだ評価器とエディタ.
fn setup(text: &str) -> (Evaluator, EditorHost<NoUi>) {
    let ev = Evaluator::new();
    let host = EditorHost::new(Editor::new(), NoUi);
    ev.eval_with(include_str!("../../lisp/simple.scm"), &host)
        .unwrap();
    ev.eval_with(include_str!("../../lisp/replace.scm"), &host)
        .unwrap();
    host.editor().current_buffer_mut().load(text);
    (ev, host)
}

#[test]
fn test_highlight_regexp_and_spans() {
    let (ev, host) = setup("Foo foo\nbar FOO");
    ev.eval_with(r#"(highlight-regexp "foo")"#, &host).unwrap();
    let editor = host.editor();
    let buffer = editor.current_buffer();
    // 大文字が無い正規表現は, 大文字と小文字を区別しない
    assert_eq!(buffer.highlighted_spans(0), vec![(0, 3), (4, 7)]);
    assert_eq!(buffer.highlighted_spans(1), vec![(4, 7)]);
    assert_eq!(buffer.highlights()[0].face, "hi-yellow");
}

#[test]
fn test_highlight_group_case_and_removal() {
    let (ev, host) = setup("Foo foo ab");
    ev.eval_with(
        r#"(highlight-regexp "Foo" 'hi-pink) (highlight-regexp "\\(a\\)b" 'hi-green 1)"#,
        &host,
    )
    .unwrap();
    {
        let editor = host.editor();
        let buffer = editor.current_buffer();
        // 大文字がある正規表現は, 区別する. 括弧の番号を渡すと, その括弧だけ
        assert_eq!(buffer.highlighted_spans(0), vec![(0, 3), (8, 9)]);
    }
    let regexps = ev.eval_with("(highlighted-regexps)", &host).unwrap();
    assert_eq!(regexps, r#"("Foo" "\\(a\\)b")"#);
    ev.eval_with(r#"(unhighlight-regexp "Foo")"#, &host)
        .unwrap();
    assert_eq!(
        ev.eval_with("(highlighted-regexps)", &host).unwrap(),
        r#"("\\(a\\)b")"#
    );
    ev.eval_with("(unhighlight-regexp #t)", &host).unwrap();
    assert_eq!(ev.eval_with("(highlighted-regexps)", &host).unwrap(), "()");
    assert!(host
        .editor()
        .current_buffer()
        .highlighted_spans(0)
        .is_empty());
    // 誤った正規表現は, 失敗にする
    assert!(ev.eval_with(r#"(highlight-regexp "\\(")"#, &host).is_err());
}

#[test]
fn test_hi_lock_commands() {
    let (ev, host) = setup("foo bar\nfoo  bar baz\nqux");
    ev.eval_with(r#"(highlight-phrase "foo bar")"#, &host)
        .unwrap();
    {
        let editor = host.editor();
        let buffer = editor.current_buffer();
        assert_eq!(buffer.highlighted_spans(0), vec![(0, 7)]);
        assert_eq!(buffer.highlighted_spans(1), Vec::<(usize, usize)>::new());
    }
    ev.eval_with(
        r#"(unhighlight-regexp #t) (goto-char 2) (highlight-symbol-at-point)"#,
        &host,
    )
    .unwrap();
    assert_eq!(
        host.editor().current_buffer().highlighted_spans(1),
        vec![(0, 3)]
    );
    ev.eval_with(r#"(highlight-lines-matching-regexp "ba")"#, &host)
        .unwrap();
    assert_eq!(
        host.editor().current_buffer().highlighted_spans(1),
        vec![(0, 3), (0, 12)]
    );
    // 色は, 順に変わる
    let faces: Vec<String> = host
        .editor()
        .current_buffer()
        .highlights()
        .iter()
        .map(|h| h.face.clone())
        .collect();
    assert_eq!(faces, vec!["hi-pink", "hi-green"]);
    // 記号が無い所では, 失敗する
    host.editor().current_buffer_mut().load("  ");
    host.editor().current_buffer_mut().set_point(1);
    assert!(ev.eval_with("(highlight-symbol-at-point)", &host).is_err());
}
