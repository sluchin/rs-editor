//! キーの書き方の変換 (Emacs の `C-c a` を, 画面のエディタ (CodeMirror) の `Ctrl-c a` にする).

/// Emacs の名前の付いたキーと, CodeMirror のキーの名前.
const NAMED_KEYS: &[(&str, &str)] = &[
    ("SPC", "Space"),
    ("RET", "Enter"),
    ("TAB", "Tab"),
    ("DEL", "Backspace"),
    ("ESC", "Escape"),
    ("<return>", "Enter"),
    ("<tab>", "Tab"),
    ("<backspace>", "Backspace"),
    ("<escape>", "Escape"),
    ("<delete>", "Delete"),
    ("<insert>", "Insert"),
    ("<home>", "Home"),
    ("<end>", "End"),
    ("<prior>", "PageUp"),
    ("<next>", "PageDown"),
    ("<left>", "ArrowLeft"),
    ("<right>", "ArrowRight"),
    ("<up>", "ArrowUp"),
    ("<down>", "ArrowDown"),
];

/// 修飾キー (Emacs の書き方と, CodeMirror の書き方).
const MODIFIERS: &[(&str, &str)] = &[
    ("C-", "Ctrl-"),
    ("M-", "Alt-"),
    ("S-", "Shift-"),
    ("s-", "Meta-"),
];

/// Emacs のキーの書き方 (`C-c a`, `C-M-f`, `<f5>` など. 空白で区切ると, 続けて押すキー) を, CodeMirror の書き方にします.
///
/// # Arguments
///
/// * `keys` - Emacs の書き方.
///
/// # Returns
///
/// CodeMirror の書き方. 読めなければ, 理由の文字列の [`Err`].
pub fn to_codemirror(keys: &str) -> Result<String, String> {
    let strokes: Result<Vec<String>, String> = keys.split_whitespace().map(stroke).collect();
    match strokes {
        Ok(strokes) if !strokes.is_empty() => Ok(strokes.join(" ")),
        _ => Err(format!("Invalid key: {:?}", keys)),
    }
}

/// 1 回に押すキー (`C-M-f` など) を, CodeMirror の書き方にします.
///
/// # Arguments
///
/// * `text` - 1 回に押すキーの, Emacs の書き方.
///
/// # Returns
///
/// CodeMirror の書き方 (`Ctrl-Alt-f` など). 読めなければ [`Err`] (理由は空. 呼ぶ側が書く).
fn stroke(text: &str) -> Result<String, String> {
    let mut rest = text;
    let mut result = String::new();
    // 修飾キーの後には, キーが要る (`C-` の `-` は, キーではない)
    while let Some((prefix, name)) = MODIFIERS
        .iter()
        .find(|(prefix, _)| rest.len() > prefix.len() && rest.starts_with(prefix))
    {
        result.push_str(name);
        rest = &rest[prefix.len()..];
    }
    // 理由は, 呼ぶ側 (to_codemirror) が, キーの全体で書く
    result.push_str(&key_name(rest).ok_or_else(String::new)?);
    Ok(result)
}

/// 修飾キーを除いたキーを, CodeMirror の名前にします.
///
/// # Arguments
///
/// * `key` - キーの Emacs の名前 (`a`, `RET`, `<f5>` など).
///
/// # Returns
///
/// CodeMirror の名前 (`a`, `Enter`, `F5` など). 知らない名前は [`None`].
fn key_name(key: &str) -> Option<String> {
    if key.chars().count() == 1 {
        return Some(key.to_string());
    }
    if let Some((_, name)) = NAMED_KEYS.iter().find(|(emacs, _)| *emacs == key) {
        return Some(name.to_string());
    }
    // ファンクションキー (<f1> から <f12>)
    let number: u8 = key.strip_prefix("<f")?.strip_suffix('>')?.parse().ok()?;
    (1..=12).contains(&number).then(|| format!("F{}", number))
}

#[cfg(test)]
mod tests {
    use super::*;

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
                to_codemirror(keys),
                Err(format!("Invalid key: {:?}", keys)),
                "{}",
                keys
            );
        }
    }
}
