//! `fill-paragraph` (`M-q`) のテスト. 期待値は, `emacs -Q --batch` で, text-mode, scheme-mode, emacs-lisp-mode の
//! `fill-paragraph` を動かした結果 (`core/tests/fill.json`).

use rseditor_core::fill::fill_paragraph;
use rseditor_core::indent::{Config, Plan};
use rseditor_core::syntax::Dialect;
use serde_json::Value as Json;

/// 変更の計画を, 本文 (文字の並び) に行います.
fn apply(text: &[char], plan: &Plan) -> Vec<char> {
    let mut out = text.to_vec();
    for e in plan.edits.iter().rev() {
        out.splice(e.from..e.to, e.insert.chars());
    }
    out
}

/// `|` をカーソルとして, M-q を押した後の本文を返します.
fn fill(dialect: Dialect, doc: &str, prefix: Option<&str>) -> String {
    let at = doc.chars().position(|c| c == '|').unwrap();
    let text: Vec<char> = doc.chars().filter(|c| *c != '|').collect();
    let lines: Vec<Vec<char>> = text.split(|c| *c == '\n').map(<[char]>::to_vec).collect();
    let config = Config {
        dialect,
        indent_tabs: true,
        tab_width: 8,
    };
    let (next, cursor) = match fill_paragraph(&lines, at, &config, 70, prefix) {
        Some(plan) => (apply(&text, &plan), plan.cursor.unwrap()),
        None => (text, at),
    };
    let mut out: String = next[..cursor].iter().collect();
    out.push('|');
    out.extend(&next[cursor..]);
    out
}

#[test]
fn test_against_emacs() {
    let cases: Vec<Json> = serde_json::from_str(include_str!("fill.json")).unwrap();
    assert_eq!(cases.len(), 40);
    for case in cases {
        let dialect = match case[0].as_str().unwrap() {
            "text" => Dialect::Fundamental,
            "scheme" => Dialect::Scheme,
            _ => Dialect::Elisp,
        };
        let doc = case[1].as_str().unwrap();
        assert_eq!(
            fill(dialect, doc, None),
            case[2].as_str().unwrap(),
            "{doc:?}"
        );
    }
}

#[test]
fn test_no_paragraph_and_unchanged_text() {
    assert_eq!(fill(Dialect::Fundamental, "|\n\n", None), "|\n\n");
    let config = Config {
        dialect: Dialect::Fundamental,
        indent_tabs: true,
        tab_width: 8,
    };
    let lines = vec![vec![], vec![]];
    assert!(fill_paragraph(&lines, 0, &config, 70, None).is_none());
    let plan = fill_paragraph(&[vec!['a']], 0, &config, 70, None).unwrap();
    assert!(plan.edits.is_empty());
}

#[test]
fn test_fill_prefix() {
    assert_eq!(
        fill(
            Dialect::Fundamental,
            "|aa bb cc dd ee ff gg hh ii jj kk ll mm nn oo pp qq rr ss tt uu vv ww xx yy zz",
            Some("# ")
        ),
        "|aa bb cc dd ee ff gg hh ii jj kk ll mm nn oo pp qq rr ss tt uu vv ww\n# xx yy zz"
    );
}
