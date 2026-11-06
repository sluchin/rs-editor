//! 単語の移動 (Emacs の `forward-word` と `backward-word`). 単語の文字は英数字と `$` `%`.
//! 続く 2 つの単語の文字が, 1 つの単語か, 別の単語かは, Emacs の `word_boundary_p` と同じく,
//! 文字の書記体系 (`char-script-table`) と, 漢字・ひらがな・カタカナの区分 (`word-combining-categories`,
//! `word-separating-categories`) で決める.

/// 文字の書記体系 (Emacs の `char-script-table` のうち, 単語の区切りに関わるもの).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Script {
    /// 漢字.
    Han,
    /// ひらがなとカタカナ.
    Kana,
    /// 全角の記号と数字など.
    CjkMisc,
    /// ハングル.
    Hangul,
    /// ギリシャ文字.
    Greek,
    /// キリル文字.
    Cyrillic,
    /// ラテン文字.
    Latin,
    /// そのほか (互いに同じとして扱う).
    Other,
}

/// 文字の書記体系.
fn script(c: char) -> Script {
    match c as u32 {
        0x3005..=0x3007 | 0xFF01..=0xFF60 | 0xFFE0..=0xFFEE | 0x3000..=0x3004 | 0x3008..=0x3020 => {
            Script::CjkMisc
        }
        0x2E80..=0x2FDF
        | 0x3400..=0x4DBF
        | 0x4E00..=0x9FFF
        | 0xF900..=0xFAFF
        | 0x20000..=0x2FA1F => Script::Han,
        0x3041..=0x30FF | 0x31F0..=0x31FF | 0xFF61..=0xFF9F | 0x1B000..=0x1B16F => Script::Kana,
        0xAC00..=0xD7AF | 0x1100..=0x11FF | 0x3130..=0x318F => Script::Hangul,
        0x0370..=0x03FF | 0x1F00..=0x1FFF => Script::Greek,
        0x0400..=0x052F => Script::Cyrillic,
        0x0..=0x24F | 0x1E00..=0x1EFF => Script::Latin,
        _ => Script::Other,
    }
}

/// 漢字か (カテゴリ `C`).
fn is_han(c: char) -> bool {
    matches!(script(c), Script::Han) || matches!(c as u32, 0x3005..=0x3007)
}

/// ひらがなか (カテゴリ `H`. 長音符 `ー` と濁点は, カタカナでもある).
fn is_hiragana(c: char) -> bool {
    matches!(c as u32, 0x3041..=0x309F | 0x30FC)
}

/// カタカナか (カテゴリ `K`).
fn is_katakana(c: char) -> bool {
    matches!(c as u32, 0x30A0..=0x30FF | 0x31F0..=0x31FF | 0x309B | 0x309C)
}

/// 結合文字か (カテゴリ `^`).
fn is_combining(c: char) -> bool {
    matches!(c as u32, 0x0300..=0x036F | 0x1AB0..=0x1AFF | 0x1DC0..=0x1DFF | 0x20D0..=0x20FF | 0xFE20..=0xFE2F)
}

/// 単語の文字か (結合文字も単語の文字).
pub fn is_word(c: char) -> bool {
    c.is_alphanumeric() || c == '$' || c == '%' || is_combining(c)
}

/// 続く 2 つの単語の文字の間で, 単語が切れるか (Emacs の `word_boundary_p`).
fn boundary(c1: char, c2: char) -> bool {
    if script(c1) == script(c2) {
        // 同じ書記体系: ひらがなの後のカタカナだけ切る
        return is_hiragana(c1) && !is_katakana(c1) && is_katakana(c2) && !is_hiragana(c2);
    }
    // 違う書記体系: 結合文字と, 漢字の後のひらがな・カタカナはつなぐ
    let combining = is_combining(c2) || (is_combining(c1) && !is_combining(c2));
    let kanji_kana = is_han(c1)
        && ((!is_hiragana(c1) && is_hiragana(c2)) || (!is_katakana(c1) && is_katakana(c2)));
    !(combining || kanji_kana)
}

/// N 個の単語の先 (N が負なら前) の位置を返します.
///
/// # Arguments
///
/// * `chars` - 本文.
/// * `pos` - 位置 (0 から).
/// * `n` - 越える単語の数 (負なら前へ).
/// * `lo` - 見える範囲の始め.
/// * `hi` - 見える範囲の終わり.
///
/// # Returns
///
/// 動いた先. 単語が足りなければ, 範囲の端.
pub fn forward_word(chars: &[char], pos: usize, n: i64, lo: usize, hi: usize) -> usize {
    let mut at = pos;
    for _ in 0..n.unsigned_abs() {
        if n > 0 {
            while at < hi && !is_word(chars[at]) {
                at += 1;
            }
            if at >= hi {
                return hi;
            }
            at += 1;
            while at < hi && is_word(chars[at]) && !boundary(chars[at - 1], chars[at]) {
                at += 1;
            }
        } else {
            while at > lo && !is_word(chars[at - 1]) {
                at -= 1;
            }
            if at <= lo {
                return lo;
            }
            at -= 1;
            while at > lo && is_word(chars[at - 1]) && !boundary(chars[at - 1], chars[at]) {
                at -= 1;
            }
        }
    }
    at
}
