//! 大文字と小文字の変換 (`casefiddle`) と, 位置を保ってバッファを置き換える `Buffer::replace_region` のテスト.
//! Emacs との比較は `test_edit.rs` (`upcase-word` など).

use rseditor_core::buffer::Buffer;
use rseditor_core::casefiddle::{convert, Case};
use rseditor_core::editor::Editor;
use rseditor_core::scheme::editor_host::EditorHost;
use rseditor_core::scheme::evaluator::Evaluator;
use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::value::Value;

/// 画面の要る操作を受ける偽物.
struct FakeUi;

impl Host for FakeUi {
    fn call(&self, op: &str, _args: Vec<Value>) -> Result<Value, String> {
        Err(format!("ui: {op}"))
    }
}

#[test]
fn test_convert() {
    let word = |text: &'static str| {
        let chars: Vec<char> = text.chars().collect();
        move |i: usize| chars[i].is_alphanumeric()
    };
    assert_eq!(convert("ab Cd", Case::Up, word("ab Cd")), "AB CD");
    assert_eq!(convert("ab Cd", Case::Down, word("ab Cd")), "ab cd");
    assert_eq!(
        convert("foo bAR", Case::Capitalize, word("foo bAR")),
        "Foo Bar"
    );
    // 2 文字になる文字は, 大文字にすると 2 文字とも大文字, 単語の初めなら 2 文字目は小文字
    assert_eq!(convert("straße", Case::Up, |_| true), "STRASSE");
    assert_eq!(
        convert("foo bAR", Case::Initials, word("foo bAR")),
        "Foo BAR"
    );
    assert_eq!(Case::parse("upcase-initials"), Some(Case::Initials));
    assert_eq!(convert("ßa", Case::Capitalize, |_| true), "Ssa");
    assert_eq!(Case::parse("upcase"), Some(Case::Up));
    assert_eq!(Case::parse("downcase"), Some(Case::Down));
    assert_eq!(Case::parse("capitalize"), Some(Case::Capitalize));
    assert_eq!(Case::parse("other"), None);
}

#[test]
fn test_replace_region_keeps_positions() {
    let mut b = Buffer::with_text("b", "abcdef");
    b.set_point(4);
    b.set_mark(1);
    let inside = b.marker_new(2);
    let after = b.marker_new(5);
    // 同じ文字数: 位置は動かない
    b.replace_region(1, 3, "XY").unwrap();
    assert_eq!(b.text(), "aXYdef");
    assert_eq!((b.point(), b.mark()), (4, Some(1)));
    assert_eq!(b.marker_position(inside), Some(2));
    // 増やす: 範囲の後ろはずれ, 範囲の中は終わりに収める. 逆の向きの指定でもよい
    b.replace_region(3, 1, "123").unwrap();
    assert_eq!(b.text(), "a123def");
    assert_eq!(b.marker_position(after), Some(6));
    assert_eq!(b.marker_position(inside), Some(2));
    // 減らす: 範囲の中の位置は新しい終わりまで
    b.replace_region(1, 4, "Z").unwrap();
    assert_eq!(b.text(), "aZdef");
    assert_eq!(b.marker_position(inside), Some(2));
    assert_eq!(b.marker_position(after), Some(4));
    // 同じ文字なら何もしない (履歴にも残さない). 空の範囲に空の文字列も何もしない
    let undo = b.undo_list().len();
    b.replace_region(0, 2, "aZ").unwrap();
    b.replace_region(1, 1, "").unwrap();
    assert_eq!(b.undo_list().len(), undo);
    // 空の範囲に文字を入れる
    b.replace_region(5, 5, "!").unwrap();
    assert_eq!(b.text(), "aZdef!");
    assert!(b.replace_region(0, 99, "x").is_err());
}

#[test]
fn test_casify_region_op() {
    let host = EditorHost::new(Editor::new(), FakeUi);
    let ev = Evaluator::new();
    host.editor().current_buffer_mut().load("hello world");
    let value = ev
        .eval_with("(casify-region \"capitalize\" 1 12)", &host)
        .unwrap();
    assert_eq!(value, "12");
    assert_eq!(host.editor().current_buffer().text(), "Hello World");
    let error = ev
        .eval_with("(casify-region \"nope\" 1 2)", &host)
        .unwrap_err();
    assert!(error.contains("unknown case nope"), "{error}");
}
