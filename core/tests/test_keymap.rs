//! キーの割り当て (`keymap`) のテスト.

use rseditor_core::keymap::{from_codemirror, unshifted, Keymap, Lookup};

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

#[test]
fn test_unshifted() {
    assert_eq!(unshifted("C-M-S-v"), Some("C-M-v".to_string()));
    assert_eq!(unshifted("S-<insert>"), Some("<insert>".to_string()));
    assert_eq!(unshifted("C-S-DEL"), Some("C-DEL".to_string()));
    assert_eq!(unshifted("C-v"), None);
    assert_eq!(unshifted("S"), None);
    assert_eq!(unshifted("s-a"), None);
}

#[test]
fn test_default_keys_of_existing_commands() {
    let keymap = Keymap::default();
    for (key, command) in [
        ("C-S-DEL", "kill-whole-line"),
        ("C-x f", "set-fill-column"),
        ("C-x =", "what-cursor-position"),
        ("C-x 8 RET", "insert-char"),
        ("C-?", "undo-redo"),
        ("M-g c", "goto-char"),
        ("M-g TAB", "move-to-column"),
        ("C-x n g", "goto-line-relative"),
        ("C-x x t", "toggle-truncate-lines"),
        ("C-M-S-v", "scroll-other-window-down"),
        ("M-<prior>", "scroll-other-window-down"),
        ("C-M-S-l", "recenter-other-window"),
        ("C-x C-@", "pop-global-mark"),
        ("C-x C-SPC", "pop-global-mark"),
    ] {
        assert_eq!(
            keymap.lookup(&keys(key)),
            Lookup::Command(command.to_string()),
            "{key}"
        );
    }
}
