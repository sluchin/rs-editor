//! 文字ごとの種類を持つ塗り分け. Emacs の font-lock の規則 (先に付いた色を上書きするか) を, Rust と TypeScript で共有します.

use super::{Span, TokenKind};

/// 規則 1 つの, 色を付け方 (font-lock の `override`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Fill {
    /// 範囲に 1 文字でも色が付いていれば, 付けない (`nil`).
    Whole,
    /// 色の付いていない文字にだけ付ける (`keep`).
    Gaps,
    /// 付いている色を上書きする (`t`).
    Over,
}

/// 文字ごとの種類.
pub(super) struct Painter {
    /// 本文の文字.
    pub(super) chars: Vec<char>,
    /// 文字ごとの種類 (色が付かなければ `None`).
    pub(super) kinds: Vec<Option<TokenKind>>,
    /// 文字の位置から, 本文の中のバイトの位置へ (最後は, 本文の長さ).
    pub(super) byte_of: Vec<usize>,
    /// 本文の中のバイトの位置から, 文字の位置へ (文字の途中のバイトは, その文字).
    pub(super) char_of: Vec<usize>,
}

impl Painter {
    /// 本文から作ります (色は, まだ無い).
    pub(super) fn new(text: &str) -> Painter {
        let chars: Vec<char> = text.chars().collect();
        let mut byte_of = Vec::with_capacity(chars.len() + 1);
        let mut char_of = vec![0; text.len() + 1];
        for (n, (byte, c)) in text.char_indices().enumerate() {
            byte_of.push(byte);
            char_of[byte..byte + c.len_utf8()].fill(n);
        }
        byte_of.push(text.len());
        char_of[text.len()] = chars.len();
        Painter {
            kinds: vec![None; chars.len()],
            chars,
            byte_of,
            char_of,
        }
    }

    /// 文字の位置 `i` (本文の外は `'\0'`).
    pub(super) fn at(&self, i: usize) -> char {
        self.chars.get(i).copied().unwrap_or('\0')
    }

    /// `from` から `to` の前まで, `kind` にします (`None` は, 色を消す. `Over` だけが使う).
    pub(super) fn paint(&mut self, from: usize, to: usize, kind: Option<TokenKind>, fill: Fill) {
        let range = from..to.min(self.chars.len());
        match fill {
            Fill::Whole if self.kinds[range.clone()].iter().all(Option::is_none) => {
                self.kinds[range].fill(kind);
            }
            Fill::Whole => {}
            Fill::Gaps => {
                for slot in &mut self.kinds[range] {
                    if slot.is_none() {
                        *slot = kind;
                    }
                }
            }
            Fill::Over => self.kinds[range].fill(kind),
        }
    }

    /// 文字ごとの種類から, 範囲の並びを作ります (改行で切る).
    pub(super) fn spans(&self) -> Vec<Span> {
        let mut spans: Vec<Span> = Vec::new();
        for (i, kind) in self.kinds.iter().enumerate() {
            let Some(kind) = *kind else { continue };
            if self.chars[i] == '\n' {
                continue;
            }
            match spans.last_mut() {
                Some(last) if last.end == i && last.kind == kind => last.end = i + 1,
                _ => spans.push(Span {
                    start: i,
                    end: i + 1,
                    kind,
                }),
            }
        }
        spans
    }
}
