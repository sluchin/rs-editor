//! 構文の走査 (`syntax`) のテスト. 期待値は, `emacs -Q --batch` で全ての位置から動かした結果
//! (`core/tests/syntax.json`. `tools/syntax/run.sh` で作る).

use rseditor_core::syntax::{Class, Dialect, ParseState, ScanError, Source};
use serde_json::{json, Value};

/// 結果を, JSON の表にある形にする.
fn scan(result: Result<Option<usize>, ScanError>) -> Value {
    match result {
        Ok(pos) => json!(pos),
        Err(e) => json!(["error", e.message, e.from, e.to]),
    }
}

/// 構文の状態を, JSON の表にある形にする.
fn state(s: ParseState) -> Value {
    json!([
        s.depth,
        s.opens,
        s.in_string,
        s.in_comment,
        s.nested,
        s.start
    ])
}

/// 表の問いを, 位置から動かした結果.
fn run(source: &Source, kind: &str, pos: usize) -> Value {
    let words: Vec<&str> = kind.split(' ').collect();
    let number = |i: usize| words[i].parse::<i64>().unwrap();
    match words[0] {
        "ppss" => state(source.parse_state(pos)),
        "scan-lists" => scan(source.scan_lists(pos, number(1), number(2), false)),
        "scan-sexps" => scan(source.scan_lists(pos, number(1), 0, true)),
        "forward-comment" => json!(source.forward_comment(pos, number(1) as usize)),
        "backward-prefix-chars" => json!(source.backward_prefix_chars(pos)),
        other => panic!("unknown query {other}"),
    }
}

#[test]
fn test_against_emacs() {
    let table: Vec<(String, String, String, Vec<Value>)> =
        serde_json::from_str(include_str!("syntax.json")).unwrap();
    assert!(table.len() > 1000);
    let mut failures = Vec::new();
    let mut skipped = 0;
    let mut compared = 0;
    for (mode, text, kind, results) in &table {
        let dialect = match mode.as_str() {
            "elisp" => Dialect::Elisp,
            "scheme" => Dialect::Scheme,
            _ => Dialect::Fundamental,
        };
        let source = Source::new(text, dialect);
        for (pos, expected) in results.iter().enumerate() {
            // 入れ子にできるコメント (`#| |#`) の中から後ろへ読む結果は, 比べない. Emacs は, コメントの中と知らずに
            // 読み (`|` を文字列の印とするなど), 意味のある結果にならない. 外から読んだ結果は, 比べる.
            let state = source.parse_state(pos);
            if kind.contains(" -") && state.in_comment && state.nested {
                skipped += 1;
                continue;
            }
            compared += 1;
            let actual = run(&source, kind, pos);
            if &actual != expected {
                failures.push(format!(
                    "{mode} {text:?} {kind} at {pos}: expected {expected}, got {actual}"
                ));
            }
        }
    }
    let mut kinds = std::collections::BTreeMap::new();
    for f in &failures {
        let key: Vec<&str> = f.splitn(3, ' ').collect();
        *kinds
            .entry(f.split(" at ").next().unwrap().to_string())
            .or_insert(0) += 1;
        let _ = key;
    }
    // 除くのは, ごく一部
    assert!(skipped * 50 < compared, "skipped {skipped} of {compared}");
    assert!(
        failures.is_empty(),
        "{} differences:\n{:#?}",
        failures.len(),
        kinds
    );
}

#[test]
fn test_scan_error_display_is_one_based() {
    let source = Source::new("(a", Dialect::Elisp);
    let error = source.scan_lists(0, 1, 0, false).unwrap_err();
    assert_eq!(
        error.to_string(),
        "Scan error: \"Unbalanced parentheses\", 1, 3"
    );
}

#[test]
fn test_source_range_and_predicates() {
    use rseditor_core::buffer::Buffer;
    use rseditor_core::syntax::{syntax_of, Class};
    let buffer = Buffer::with_text("b", "(a) ;c\n");
    let source = Source::from_buffer(&buffer, Dialect::Elisp);
    assert!(source.is_open_paren(0) && !source.is_open_paren(1) && !source.is_open_paren(99));
    assert!(source.is_whitespace(3) && !source.is_whitespace(0) && !source.is_whitespace(99));
    assert!(
        source.is_comment_start(4) && !source.is_comment_start(0) && !source.is_comment_start(99)
    );
    // 範囲を狭める: 範囲の外は読まない. 範囲は本文の中に収める
    let narrow = Source::new("(a) (b)", Dialect::Elisp).narrow(4, 99);
    assert_eq!(narrow.scan_lists(4, 1, 0, false), Ok(Some(7)));
    assert_eq!(narrow.scan_lists(7, -1, 0, false), Ok(Some(4)));
    assert_eq!(narrow.scan_lists(4, -1, 0, false), Ok(None));
    let inverted = Source::new("abc", Dialect::Fundamental).narrow(5, 2);
    assert_eq!(inverted.scan_lists(2, 1, 0, true), Ok(None));
    // 構文の種類
    assert_eq!(syntax_of('a', Dialect::Elisp).class, Class::Word);
    assert_eq!(syntax_of('\u{3001}', Dialect::Elisp).class, Class::Punct);
    assert_eq!(syntax_of('\u{2605}', Dialect::Elisp).class, Class::Symbol);
    assert_eq!(syntax_of('\u{1f600}', Dialect::Elisp).class, Class::Word);
    assert_eq!(
        syntax_of('\u{3042}', Dialect::Fundamental).class,
        Class::Word
    );
}

#[test]
fn test_symbol_next_to_a_comment_end_and_dollar_words() {
    // 値は `emacs -Q --batch` の結果 (位置は 0 から数える)
    let source = Source::new("a ; c\nb", Dialect::Elisp);
    assert_eq!(source.scan_lists(7, -1, 0, true), Ok(Some(6)));
    assert_eq!(source.scan_lists(3, -1, 0, true), Ok(Some(0)));
    let source = Source::new("$a %b", Dialect::Fundamental);
    assert_eq!(source.scan_lists(0, 1, 0, true), Ok(Some(2)));
    assert_eq!(source.scan_lists(5, -1, 0, true), Ok(Some(3)));
}

#[test]
fn test_designators_and_class_chars() {
    use rseditor_core::syntax::{class_chars, class_of_designator};
    let expected = [
        (' ', Class::Space),
        ('-', Class::Space),
        ('.', Class::Punct),
        ('w', Class::Word),
        ('_', Class::Symbol),
        ('(', Class::Open),
        (')', Class::Close),
        ('"', Class::Str),
        ('\\', Class::Escape),
        ('\'', Class::Prefix),
        ('<', Class::CommentStart),
        ('>', Class::CommentEnd),
    ];
    for (designator, class) in expected {
        assert_eq!(class_of_designator(designator), Some(class), "{designator}");
    }
    assert_eq!(class_of_designator('q'), None);
    // 構文表の ASCII の文字と, ASCII でない括弧と空白
    assert!(class_chars(Dialect::Fundamental, Class::Space).contains('\t'));
    assert!(class_chars(Dialect::Elisp, Class::Prefix).contains(','));
    assert!(class_chars(Dialect::Fundamental, Class::Prefix).is_empty());
    assert!(class_chars(Dialect::Fundamental, Class::Open).contains('「'));
    assert!(class_chars(Dialect::Fundamental, Class::Close).contains('」'));
    assert!(class_chars(Dialect::Fundamental, Class::Word).contains("\\p{L}"));
}
