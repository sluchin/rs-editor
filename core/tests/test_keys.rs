//! `scheme::keys` のテスト.

use rseditor_core::scheme::keys::*;
#[test]
fn converts_modifiers_and_sequences() {
    assert_eq!(to_codemirror("C-c a").unwrap(), "Ctrl-c a");
    assert_eq!(to_codemirror("C-M-f").unwrap(), "Ctrl-Alt-f");
    assert_eq!(to_codemirror("M-x").unwrap(), "Alt-x");
    assert_eq!(to_codemirror("S-<left>").unwrap(), "Shift-ArrowLeft");
    assert_eq!(to_codemirror("s-a").unwrap(), "Meta-a");
    assert_eq!(to_codemirror("C--").unwrap(), "Ctrl--");
    assert_eq!(to_codemirror(" C-x  C-f ").unwrap(), "Ctrl-x Ctrl-f");
}

#[test]
fn converts_named_keys() {
    assert_eq!(to_codemirror("C-SPC").unwrap(), "Ctrl-Space");
    assert_eq!(to_codemirror("RET").unwrap(), "Enter");
    assert_eq!(to_codemirror("<f5>").unwrap(), "F5");
    assert_eq!(to_codemirror("<f12>").unwrap(), "F12");
    assert_eq!(to_codemirror("<prior>").unwrap(), "PageUp");
    assert_eq!(to_codemirror("あ").unwrap(), "あ");
}

#[test]
fn rejects_invalid_keys() {
    for keys in [
        "", "  ", "C-", "C-foo", "<f13>", "<f0>", "<fx>", "<f5", "f5>",
    ] {
        assert_eq!(
            to_codemirror(keys).map_err(|e| e.to_string()),
            Err(format!("Invalid key: {:?}", keys)),
            "{}",
            keys
        );
    }
}
