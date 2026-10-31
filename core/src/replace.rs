//! 置き換える文字列の組み立て (Emacs の `replace-match`): `\&` `\N` `\\` の展開と, 合った文字の大文字と小文字に合わせた変換.

/// 変換のしかた (Emacs の `replace-match` の `case_action`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaseAction {
    /// そのまま.
    NoChange,
    /// 全て大文字.
    AllCaps,
    /// 各語の頭だけ大文字.
    CapInitial,
}

/// 単語の文字か (Emacs の標準の構文表).
fn is_word(c: char) -> bool {
    c.is_alphanumeric() || c == '$' || c == '%'
}

/// 合った文字から, 置き換える文字列の変換のしかたを決めます.
pub fn case_action(matched: &str) -> CaseAction {
    let (mut lower, mut upper, mut non_upper_initial, mut multiletter) =
        (false, false, false, false);
    let mut prev = '\n';
    for c in matched.chars() {
        if c.is_lowercase() {
            // 小文字があれば, 全て大文字ではない
            lower = true;
            if is_word(prev) {
                multiletter = true;
            } else {
                non_upper_initial = true;
            }
        } else if c.is_uppercase() {
            upper = true;
            if is_word(prev) {
                multiletter = true;
            }
        } else if !is_word(prev) {
            // 大文字小文字の無い文字が語の頭なら, 小文字の頭と同じに扱う
            non_upper_initial = true;
        }
        prev = c;
    }
    if !lower && multiletter {
        CaseAction::AllCaps
    } else if !non_upper_initial && multiletter {
        CaseAction::CapInitial
    } else if !non_upper_initial && upper {
        CaseAction::AllCaps
    } else {
        CaseAction::NoChange
    }
}

/// 変換のしかたに合わせて, 文字列を変えます.
pub fn apply_case(text: &str, action: CaseAction) -> String {
    match action {
        CaseAction::NoChange => text.to_string(),
        CaseAction::AllCaps => text.to_uppercase(),
        CaseAction::CapInitial => {
            // 各語の頭だけを大文字にし, 残りはそのまま (Emacs の `upcase-initials`)
            let mut out = String::new();
            let mut prev = '\n';
            for c in text.chars() {
                if is_word(prev) {
                    out.push(c);
                } else {
                    out.extend(c.to_uppercase());
                }
                prev = c;
            }
            out
        }
    }
}

/// `\&` `\N` `\\` を展開します. `groups[n]` は, `n` 番目の括弧に合った文字列 (合わなければ `None`).
///
/// # Errors
///
/// `\` の後ろが, `&` 数字 `\` のどれでもないとき.
pub fn expand(newtext: &str, groups: &[Option<String>]) -> Result<String, String> {
    let mut out = String::new();
    let mut chars = newtext.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('&') => out.push_str(groups.first().and_then(Option::as_deref).unwrap_or("")),
            Some(d @ '0'..='9') => {
                let n = d as usize - '0' as usize;
                // 合わなかった括弧と, 無い括弧は, 空の文字列
                out.push_str(groups.get(n).and_then(Option::as_deref).unwrap_or(""));
            }
            Some('\\') => out.push('\\'),
            _ => return Err("Invalid use of ‘\\’ in replacement text".to_string()),
        }
    }
    Ok(out)
}
