//! 探す処理 (`search`) のテスト. 期待値は, `emacs -Q --batch` で全ての位置から動かした結果
//! (`core/tests/search.json`. `tools/syntax/run.sh` で作る).

use rseditor_core::buffer::Buffer;
use rseditor_core::search::{translate, Pattern};
use serde_json::{json, Value};

/// 結果を, JSON の表にある形 (動いた先の位置と, 合った所) にする.
fn outcome(buffer: &Buffer, moved: Option<usize>) -> Value {
    moved.map_or(Value::Null, |point| {
        let mut row = vec![json!(point)];
        row.extend(
            buffer
                .match_data()
                .iter()
                .map(|m| m.map_or(Value::Null, |(a, b)| json!([a, b]))),
        );
        Value::Array(row)
    })
}

#[test]
fn test_against_emacs() {
    let table: Vec<(String, String, String, Vec<Value>)> =
        serde_json::from_str(include_str!("search.json")).unwrap();
    assert!(table.len() > 2000);
    let mut failures = Vec::new();
    for (pattern, text, kind_name, results) in &table {
        let fold = kind_name.ends_with(" fold");
        let kind = kind_name.trim_end_matches(" fold").to_string();
        let compiled = match Pattern::new(pattern, fold) {
            Ok(p) => p,
            Err(e) => {
                failures.push(format!("{pattern:?}: {e}"));
                continue;
            }
        };
        for (pos, expected) in results.iter().enumerate() {
            let mut buffer = Buffer::with_text("t", text);
            buffer.set_point(pos);
            let actual = match kind.as_str() {
                "re-search-forward" => {
                    let moved = buffer.re_search_forward(&compiled, None, 1);
                    outcome(&buffer, moved)
                }
                "re-search-forward 2" => {
                    let moved = buffer.re_search_forward(&compiled, None, 2);
                    outcome(&buffer, moved)
                }
                "re-search-backward" => {
                    let moved = buffer.re_search_backward(&compiled, None, 1);
                    outcome(&buffer, moved)
                }
                _ => {
                    let matched = buffer.looking_at(&compiled);
                    outcome(&buffer, matched.then_some(pos))
                }
            };
            if &actual != expected {
                failures.push(format!(
                    "{pattern:?} {text:?} {kind} at {pos}: expected {expected}, got {actual}"
                ));
            }
        }
    }
    let mut summary = std::collections::BTreeMap::new();
    for f in &failures {
        let key = f.split(" at ").next().unwrap().to_string();
        *summary.entry(key).or_insert(0) += 1;
    }
    assert!(
        failures.is_empty(),
        "{} differences:\n{:#?}\n{}",
        failures.len(),
        summary,
        failures
            .iter()
            .filter(|f| std::env::var("SHOW").is_ok_and(|s| f.contains(&s)))
            .take(6)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}

/// 正規表現を作る (大文字と小文字を区別する).
fn pattern(text: &str) -> Pattern {
    Pattern::new(text, false).unwrap()
}

#[test]
fn test_bound_and_count_like_emacs() {
    // 値は `emacs -Q --batch` の結果 (位置は 0 から数える)
    let abc = "abc abc abc";
    let mut buffer = Buffer::with_text("t", abc);
    // 限りの前で終わる合い方だけ
    assert_eq!(
        buffer.re_search_forward(&pattern("abc"), Some(4), 1),
        Some(3)
    );
    assert_eq!(buffer.match_data(), [Some((0, 3))]);
    buffer.set_point(0);
    assert_eq!(buffer.re_search_forward(&pattern("abc"), Some(2), 1), None);
    assert_eq!(buffer.point(), 0);
    // 回数: 3 回目まで合い, 4 回目は合わない (動かないが, 合った所の記録は 3 回目のもの)
    assert_eq!(buffer.re_search_forward(&pattern("abc"), None, 3), Some(11));
    buffer.set_point(0);
    assert_eq!(buffer.re_search_forward(&pattern("abc"), None, 4), None);
    assert_eq!(buffer.point(), 0);
    assert_eq!(buffer.match_data(), [Some((8, 11))]);
    // 限りの上で始まって, 限りを越える合い方は, 合わない
    let mut short = Buffer::with_text("t", "ab");
    assert_eq!(short.re_search_forward(&pattern("b"), Some(1), 1), None);
    assert_eq!(short.re_search_forward(&pattern("b"), Some(2), 1), Some(2));
    // 後ろへ
    buffer.set_point(11);
    assert_eq!(
        buffer.re_search_backward(&pattern("abc"), Some(4), 1),
        Some(8)
    );
    buffer.set_point(11);
    assert_eq!(
        buffer.re_search_backward(&pattern("abc"), Some(5), 1),
        Some(8)
    );
    buffer.set_point(11);
    assert_eq!(buffer.re_search_backward(&pattern("abc"), None, 3), Some(0));
    buffer.set_point(11);
    assert_eq!(buffer.re_search_backward(&pattern("abc"), None, 4), None);
    assert_eq!(buffer.point(), 11);
    assert_eq!(buffer.match_data(), [Some((0, 3))]);
    // 合った所の終わりは, 探し始めの位置を越えない (空の合い方になる)
    let mut buffer = Buffer::with_text("t", "aaa");
    buffer.set_point(1);
    assert_eq!(buffer.re_search_backward(&pattern("a*"), None, 1), Some(1));
    assert_eq!(buffer.match_data(), [Some((1, 1))]);
}

#[test]
fn test_literal_pattern_and_case_fold() {
    let mut buffer = Buffer::with_text("t", "Abc a.c");
    // 文字そのものを探す: `.` は任意の文字でない
    assert_eq!(
        buffer.re_search_forward(&Pattern::literal("a.c", false), None, 1),
        Some(7)
    );
    assert_eq!(buffer.match_data(), [Some((4, 7))]);
    buffer.set_point(0);
    assert_eq!(
        buffer.re_search_forward(&Pattern::literal("aB", true), None, 1),
        Some(2)
    );
    buffer.set_point(0);
    assert_eq!(
        buffer.re_search_forward(&Pattern::literal("aB", false), None, 1),
        None
    );
    // 空の合い方
    buffer.set_point(0);
    assert!(buffer.looking_at(&pattern("")));
    assert!(!buffer.looking_at(&pattern("b")));
    assert_eq!(buffer.match_data(), [Some((0, 0))]);
}

#[test]
fn test_translate() {
    let cases = [
        (r"\(a\)\|b", "(a)|b"),
        (r"a\{2,3\}", "a{2,3}"),
        (r"(a){2}|b", r"\(a\)\{2\}\|b"),
        (r"*a", r"\*a"),
        (r"\(?:a\)", "(?:a)"),
        (r"a^b$c", r"a\^b\$c"),
        (r"\.\*\+\?", r"\.\*\+\?"),
        (r"[]a]", r"[\]a]"),
        (r"[^]a]", r"[^\]a]"),
        (r"[a\]", r"[a\\]"),
        (r"[[:digit:]]", "[0-9]"),
    ];
    for (emacs, rust) in cases {
        assert_eq!(translate(emacs).unwrap(), rust, "{emacs}");
    }
    let errors = [
        (r"a\", "Trailing backslash"),
        (r"\(?1:a\)", "Numbered groups are not supported"),
        (r"a\{1", "Unmatched \\{"),
        (r"a\{x\}", "Invalid content of \\{\\}"),
        (r"a\{\}", "Invalid content of \\{\\}"),
        (r"a\}", "Unmatched \\}"),
        (r"\s", "Invalid syntax designator"),
        (r"\sq", "Unsupported syntax class"),
        (r"\_x", "Invalid regular expression"),
        (r"\=", "Unsupported regular expression"),
        (r"[a", "Unmatched [ or [^"),
        (r"[[:alpha", "Unmatched [ or [^"),
        (r"[[:foo:]]", "Invalid character class name: foo"),
    ];
    for (emacs, message) in errors {
        assert_eq!(translate(emacs).unwrap_err(), message, "{emacs}");
    }
    assert!(Pattern::new(r"\(a", false)
        .unwrap_err()
        .starts_with("Invalid regexp"));
}

#[test]
fn test_classes_and_syntax_sets() {
    // どの書き方も作れて, 想定の文字に合う
    let cases = [
        (r"[[:alnum:]]+", "é1", true),
        (r"[[:xdigit:]]+", "fF0", true),
        (r"[[:space:]]", " ", true),
        (r"[[:cntrl:]]", "\u{1}", true),
        (r"[[:graph:]]", "a", true),
        (r"[[:print:]]", " ", true),
        (r"[[:ascii:]]", "a", true),
        (r"[[:nonascii:]]", "あ", true),
        (r"[[:multibyte:]]", "あ", true),
        (r"[[:unibyte:]]", "a", true),
        (r"\W", "a", false),
        (r"\S-", " ", false),
        (r"\Sw", "a", false),
        (r"\s_", "-", true),
        (r"\B", "", false),
    ];
    for (emacs, text, matches) in cases {
        let mut buffer = Buffer::with_text("t", text);
        assert_eq!(buffer.looking_at(&pattern(emacs)), matches, "{emacs}");
    }
    // 単語の中は \B, 端は \b (空のバッファの端も)
    let mut buffer = Buffer::with_text("t", "xx");
    buffer.set_point(1);
    assert!(buffer.looking_at(&pattern(r"\B")) && !buffer.looking_at(&pattern(r"\b")));
    buffer.set_point(0);
    assert!(buffer.looking_at(&pattern(r"\b")));
}
