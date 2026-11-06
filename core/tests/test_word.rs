//! 単語の移動 (`core/src/word.rs`). Emacs の結果は `tests/batch/cases/56-word-motion` で比べる.

use rseditor_core::word::{forward_word, is_word};

fn chars(text: &str) -> Vec<char> {
    text.chars().collect()
}

/// 先頭から 1 語ずつ進んだ位置 (0 から).
fn stops(text: &str) -> Vec<usize> {
    let c = chars(text);
    let mut at = 0;
    let mut found = Vec::new();
    while at < c.len() {
        let next = forward_word(&c, at, 1, 0, c.len());
        if next == at {
            break;
        }
        found.push(next);
        at = next;
    }
    found
}

/// 終わりから 1 語ずつ戻った位置 (0 から).
fn back_stops(text: &str) -> Vec<usize> {
    let c = chars(text);
    let mut at = c.len();
    let mut found = Vec::new();
    while at > 0 {
        let next = forward_word(&c, at, -1, 0, c.len());
        if next == at {
            break;
        }
        found.push(next);
        at = next;
    }
    found
}

#[test]
fn test_word_characters() {
    assert!(is_word('a') && is_word('7') && is_word('$') && is_word('%') && is_word('日'));
    assert!(!is_word('_') && !is_word(' ') && !is_word('-') && !is_word('，'));
}

#[test]
fn test_ascii_words() {
    assert_eq!(stops("foo_bar baz$qux 100% x-y"), [3, 7, 15, 20, 22, 24]);
    assert_eq!(back_stops("foo_bar baz$qux"), [8, 4, 0]);
}

#[test]
fn test_japanese_words_follow_the_scripts() {
    // 漢字とひらがなは 1 語, ひらがなの後のカタカナは別の語
    assert_eq!(stops("日本語かな漢字ひらがなカタカナ"), [5, 11, 15]);
    assert_eq!(back_stops("日本語かな漢字ひらがなカタカナ"), [11, 5, 0]);
    // カタカナの後のひらがなは続く. 長音符はどちらにも付く
    assert_eq!(stops("カナひら"), [4]);
    assert_eq!(stops("ひらーカナ"), [5]);
    // 全角の英数字は 1 語. 々は漢字と別
    assert_eq!(stops("ＡＢＣ，ＤＥＦ"), [3, 7]);
    assert_eq!(stops("日々"), [1, 2]);
    // 拡張の漢字
    assert_eq!(stops("\u{20000}\u{20001}"), [2]);
}

#[test]
fn test_other_scripts_and_combining_marks() {
    // 書記体系が違えば切る. 結合文字はつなぐ
    assert_eq!(stops("abcαβγ абв 한국"), [3, 6, 10, 13]);
    assert_eq!(stops("e\u{301}x"), [3]);
    assert_eq!(stops("é ü naïve"), [1, 3, 9]);
    // 上に無い書記体系は, 同じとして扱う
    assert_eq!(stops("שלום עולם"), [4, 9]);
}

#[test]
fn test_counts_limits_and_direction() {
    let c = chars("one two three");
    assert_eq!(forward_word(&c, 0, 2, 0, c.len()), 7);
    assert_eq!(forward_word(&c, 0, 9, 0, c.len()), 13);
    assert_eq!(forward_word(&c, 13, -2, 0, c.len()), 4);
    assert_eq!(forward_word(&c, 13, -9, 0, c.len()), 0);
    // 絞り込んだ範囲の中だけ
    assert_eq!(forward_word(&c, 4, 5, 4, 7), 7);
    assert_eq!(forward_word(&c, 7, -5, 4, 7), 4);
    assert_eq!(forward_word(&c, 3, 0, 0, c.len()), 3);
}
