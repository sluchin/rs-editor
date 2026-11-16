//! 組み込みの手続きの docstring (`builtin_docs.rs` と `emacs_docs.rs`) のテスト.

use rseditor_core::scheme::builtin_docs;
use rseditor_core::scheme::emacs_docs;
use rseditor_core::scheme::registry::registry;

#[test]
fn test_every_registered_procedure_has_a_docstring() {
    let missing: Vec<&str> = registry()
        .names()
        .filter(|name| builtin_docs::lookup(name).is_none())
        .collect();
    assert!(
        missing.is_empty(),
        "no docstring (add it to builtin_docs.rs): {missing:?}"
    );
}

#[test]
fn test_tables_are_sorted_and_have_no_unknown_name() {
    for table in [builtin_docs::DOCS, emacs_docs::DOCS] {
        assert!(table.windows(2).all(|pair| pair[0].0 < pair[1].0));
        for (name, doc) in table {
            assert!(registry().get(name).is_some(), "{name} is not registered");
            assert!(!doc.is_empty(), "{name} has an empty docstring");
        }
    }
}

#[test]
fn test_hand_written_docstring_wins_over_emacs() {
    let emacs: Vec<&str> = emacs_docs::DOCS.iter().map(|(name, _)| *name).collect();
    for (name, _) in builtin_docs::DOCS {
        assert!(!emacs.contains(name), "{name} is in both tables");
    }
    assert_eq!(
        builtin_docs::lookup("zero?"),
        Some("Return #t if the number N is zero.\n\n(zero? N)")
    );
    assert!(builtin_docs::lookup("no-such-procedure").is_none());
}
