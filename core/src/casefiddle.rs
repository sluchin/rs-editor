//! 大文字と小文字の変換 (Emacs の `casefiddle.c`. `upcase-region`, `capitalize-word` など).

/// 変換の種類.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Case {
    /// 全て大文字 (`upcase`).
    Up,
    /// 全て小文字 (`downcase`).
    Down,
    /// 単語の初めを大文字, 残りを小文字 (`capitalize`).
    Capitalize,
}

impl Case {
    /// 名前 (`upcase`, `downcase`, `capitalize`) から種類を返します.
    pub fn parse(name: &str) -> Option<Self> {
        match name {
            "upcase" => Some(Case::Up),
            "downcase" => Some(Case::Down),
            "capitalize" => Some(Case::Capitalize),
            _ => None,
        }
    }
}

/// 文字を大文字にします. 2 文字以上になる文字 (`ß` → `SS`) は, 先頭だけ大文字 (`Ss`) にして, 単語の初めの
/// 文字 (`title`) に使います.
fn upcase(c: char, title: bool, out: &mut String) {
    let mut upper = c.to_uppercase();
    if let Some(first) = upper.next() {
        out.push(first);
    }
    for rest in upper {
        if title {
            out.extend(rest.to_lowercase());
        } else {
            out.push(rest);
        }
    }
}

/// 文字列を変換します (Emacs の `casify_region` の文字ごとの規則).
///
/// # Arguments
///
/// * `text` - 変換する文字列.
/// * `case` - 変換の種類.
/// * `is_word` - `i` 番目の文字が単語の文字なら `true`. `Capitalize` で, 単語の境目を決めるのに使う.
pub fn convert(text: &str, case: Case, is_word: impl Fn(usize) -> bool) -> String {
    let mut out = String::with_capacity(text.len());
    let mut in_word = false;
    for (i, c) in text.chars().enumerate() {
        match case {
            Case::Up => upcase(c, false, &mut out),
            Case::Down => out.extend(c.to_lowercase()),
            Case::Capitalize if in_word => out.extend(c.to_lowercase()),
            Case::Capitalize => upcase(c, true, &mut out),
        }
        in_word = is_word(i);
    }
    out
}
