//! `skip-chars-forward` と `skip-chars-backward` のテスト. 期待値は, `emacs -Q --batch` で全ての位置から動かした結果
//! (`core/tests/skip.json`. `tools/syntax/run.sh` で作る).

use rseditor_core::buffer::Buffer;
use rseditor_core::search::CharSet;
use serde_json::{json, Value};

#[test]
fn test_against_emacs() {
    let table: Vec<(String, String, Value, String, Vec<Value>)> =
        serde_json::from_str(include_str!("skip.json")).unwrap();
    assert!(table.len() > 500);
    let mut failures = Vec::new();
    for (spec, dir, lim, text, results) in &table {
        let set = CharSet::parse(spec).unwrap();
        for (pos, expected) in results.iter().enumerate() {
            let mut buffer = Buffer::with_text("t", text);
            buffer.set_point(pos);
            // 限りは, 0 より大きければ, 位置からの距離 (本文の外は端) , 負なら手前
            let limit = lim.as_i64().map(|l| {
                if l > 0 {
                    (pos + l as usize).min(text.chars().count())
                } else {
                    pos.saturating_sub((-l) as usize)
                }
            });
            let moved = buffer.skip_chars(&set, dir == "forward", limit);
            let actual = json!([moved, buffer.point()]);
            if &actual != expected {
                failures.push(format!(
                    "{spec:?} {dir} {lim} {text:?} at {pos}: expected {expected}, got {actual}"
                ));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{} differences:\n{}",
        failures.len(),
        failures
            .iter()
            .take(8)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn test_char_set_errors_and_classes() {
    assert_eq!(
        CharSet::parse("[:foo:]").unwrap_err(),
        "Invalid ISO C character class: foo"
    );
    for (name, c, expected) in [
        ("alnum", 'é', true),
        ("xdigit", 'f', true),
        ("space", '\t', true),
        ("blank", ' ', true),
        ("blank", '\n', false),
        ("punct", '!', true),
        ("punct", '。', true),
        ("punct", 'a', false),
        ("word", '$', true),
        ("cntrl", '\u{1}', true),
        ("graph", 'a', true),
        ("print", ' ', true),
        ("ascii", 'a', true),
        ("unibyte", 'あ', false),
        ("nonascii", 'あ', true),
        ("multibyte", 'a', false),
        ("lower", 'a', true),
        ("upper", 'a', false),
        ("digit", '5', true),
    ] {
        let set = CharSet::parse(&format!("[:{name}:]")).unwrap();
        assert_eq!(set.contains(c), expected, "{name} {c:?}");
    }
}

#[test]
fn test_trailing_backslash_is_a_character() {
    let set = CharSet::parse("a\\").unwrap();
    assert!(set.contains('a') && set.contains('\\') && !set.contains('b'));
}
