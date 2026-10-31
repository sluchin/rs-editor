//! キーの割り当て (`keymap`) のテスト.

use rseditor_tui::keymap::{from_codemirror, Keymap, Lookup};

fn keys(text: &str) -> Vec<String> {
    text.split(' ').map(str::to_string).collect()
}

#[test]
fn test_lookup() {
    let keymap = Keymap::default();
    assert_eq!(
        keymap.lookup(&keys("C-f")),
        Lookup::Command("forward-char".into())
    );
    assert_eq!(
        keymap.lookup(&keys("C-x C-f")),
        Lookup::Command("find-file".into())
    );
    assert_eq!(keymap.lookup(&keys("C-x")), Lookup::Prefix);
    assert_eq!(keymap.lookup(&keys("M-g")), Lookup::Prefix);
    assert_eq!(keymap.lookup(&keys("C-x C-z")), Lookup::Unbound);
    assert_eq!(keymap.lookup(&keys("a")), Lookup::Unbound);
}

#[test]
fn test_bind_replaces_and_where_is() {
    let mut keymap = Keymap::default();
    keymap.bind("C-f", "my-forward");
    assert_eq!(
        keymap.lookup(&keys("C-f")),
        Lookup::Command("my-forward".into())
    );
    assert_eq!(keymap.keys_for("my-forward"), vec!["C-f"]);
    assert!(keymap
        .keys_for("forward-char")
        .contains(&"<right>".to_string()));
    keymap.bind("C-c a", "x");
    assert_eq!(keymap.lookup(&keys("C-c")), Lookup::Prefix);
    assert!(keymap.keys_for("nothing").is_empty());
}

#[test]
fn test_from_codemirror() {
    for (cm, emacs) in [
        ("Ctrl-h f", "C-h f"),
        ("Alt-x", "M-x"),
        ("Meta-b", "M-b"),
        ("Ctrl-Alt-f", "C-M-f"),
        ("Shift-a", "S-a"),
        ("Ctrl-x Enter", "C-x RET"),
        ("Backspace", "DEL"),
        ("Delete", "<delete>"),
        ("Escape", "ESC"),
        ("Tab", "TAB"),
        ("Ctrl-Space", "C-SPC"),
        ("ArrowUp", "<up>"),
        ("ArrowDown", "<down>"),
        ("ArrowLeft", "<left>"),
        ("ArrowRight", "<right>"),
        ("Home", "<home>"),
        ("End", "<end>"),
        ("PageUp", "<prior>"),
        ("PageDown", "<next>"),
        ("Ctrl--", "C--"),
        ("Hyper-a", "a"),
        ("Alt-<", "M-<"),
    ] {
        assert_eq!(from_codemirror(cm), emacs, "{cm}");
    }
}
