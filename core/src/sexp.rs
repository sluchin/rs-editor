//! S 式, リスト, defun の移動 (Emacs 30 の lisp.el. `C-M-f`, `C-M-u`, `C-M-d`, `C-M-a`, `C-M-e`, `C-M-h`).
//!
//! defun は, 行の始め (0 桁) の開く括弧から始まる式です (Emacs の `open-paren-in-column-0-is-defun-start`).

use crate::syntax::{ScanError, Source};

/// 動くのに失敗したとき: 失敗の理由と, 失敗したときのカーソル (Emacs は, 動いた所で止まる).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MotionError {
    /// 失敗したときのカーソル.
    pub point: usize,
    /// 理由 (`Unbalanced parentheses` など).
    pub message: String,
}

/// 行の始めを返します.
fn line_start(source: &Source, pos: usize) -> usize {
    let mut p = pos.min(source.size());
    while p > 0 && source.char_at(p - 1) != Some('\n') {
        p -= 1;
    }
    p
}

/// 行の終わりを返します (改行の前).
fn line_end(source: &Source, pos: usize) -> usize {
    let mut p = pos;
    while p < source.size() && source.char_at(p) != Some('\n') {
        p += 1;
    }
    p
}

/// 次の行の始めを返します (Emacs の `(forward-line 1)`. 最後の行なら, 本文の終わり).
fn next_line(source: &Source, pos: usize) -> usize {
    let end = line_end(source, pos);
    if end < source.size() {
        end + 1
    } else {
        end
    }
}

/// S 式を 1 つ越えて進みます (Emacs の `forward-sexp`).
///
/// # Returns
///
/// 越えた後の位置. 式が無ければ, 範囲の終わり.
///
/// # Errors
///
/// 括弧が合わないとき.
pub fn forward_sexp(source: &Source, pos: usize) -> Result<usize, ScanError> {
    let end = source.scan_lists(pos, 1, 0, true)?;
    Ok(end.unwrap_or(source.range().1))
}

/// 外のリストへ出ます (Emacs の, キーから動かしたときの `up-list` と `backward-up-list`).
/// 文字列の中では, 文字列の中のリストだけを見て, 無ければ文字列の外へ出ます.
///
/// # Arguments
///
/// * `source` - 本文.
/// * `pos` - 位置.
/// * `count` - 出る数 (正なら後ろへ, 負なら前へ).
///
/// # Errors
///
/// 外にリストが無いとき (`At top level`).
pub fn up_list(source: &Source, pos: usize, count: i64) -> Result<usize, MotionError> {
    let inc = if count > 0 { 1 } else { -1 };
    let mut arg = count;
    let mut pos = pos;
    while arg != 0 {
        let state = source.parse_state(pos);
        // 文字列かコメントの中なら, その中だけを見る
        let inner = match state.start {
            Some(start) => {
                let end = if state.in_string {
                    forward_sexp(source, start).unwrap_or(source.size())
                } else {
                    source.forward_comment(start, usize::MAX)
                };
                source.clone().narrow(start, end)
            }
            None => source.clone(),
        };
        match inner.scan_lists(pos, inc, 1, false) {
            Ok(Some(found)) => pos = found,
            _ => {
                let start = state.start.unwrap_or(0);
                // コメントの後ろ (閉じていなければ, まだコメントの中)
                let after = if state.in_comment && inc > 0 {
                    source.forward_comment(start, 1)
                } else {
                    start
                };
                if state.in_string {
                    // 文字列の外へ (後ろへは, 文字列が閉じていなければ, 出られない)
                    pos = if inc > 0 {
                        forward_sexp(source, start).map_err(|e| MotionError {
                            // 文字列の始めまで動いてから失敗する
                            point: start,
                            message: e.to_string(),
                        })?
                    } else {
                        start
                    };
                } else if state.in_comment && !source.parse_state(after).in_comment {
                    // コメントの外から, もう一度
                    pos = after;
                    arg += inc;
                } else {
                    // コメントの中なら, コメントの外 (閉じていなければ, 終わり) まで動いてから失敗する
                    if state.in_comment {
                        pos = after;
                    }
                    return Err(MotionError {
                        point: pos,
                        message: "At top level".to_string(),
                    });
                }
            }
        }
        arg -= inc;
    }
    Ok(pos)
}

/// 行の始めの開く括弧なら `true`.
fn starts_defun(source: &Source, at: usize) -> bool {
    line_start(source, at) == at && source.is_open_paren(at)
}

/// defun の始め (行の始めの開く括弧) を探します (Emacs の `beginning-of-defun-raw`).
/// 文字列とコメントの中の括弧は除きます.
///
/// # Arguments
///
/// * `source` - 本文.
/// * `pos` - 位置.
/// * `forward` - `false` なら前へ, `true` なら後ろへ探す.
///
/// # Returns
///
/// 見つけた括弧の位置と, 見つけたか. 見つからなければ, 本文の端.
fn beginning_of_defun_raw(source: &Source, pos: usize, forward: bool) -> (usize, bool) {
    let len = source.size();
    let mut p = if forward && pos < len { pos + 1 } else { pos };
    loop {
        let found = if forward {
            (p..len).find(|&at| starts_defun(source, at))
        } else {
            (0..p).rev().find(|&at| starts_defun(source, at))
        };
        let Some(found) = found else {
            return if forward { (len, false) } else { (0, false) };
        };
        p = if forward { found + 1 } else { found };
        // 文字列とコメントの中なら, その先から, もう一度
        if source.parse_state(p).start.is_none() {
            return (found, true);
        }
    }
}

/// defun の始めへ動きます (Emacs の `beginning-of-defun`).
///
/// # Returns
///
/// 動いた後の位置. defun が無ければ, 本文の始め.
pub fn beginning_of_defun(source: &Source, pos: usize) -> usize {
    beginning_of_defun_raw(source, pos, false).0
}

/// defun の終わりへ動きます (Emacs の `end-of-defun`). 終わりの後ろが, 空白かコメントだけなら, 次の行の始めへ.
///
/// # Arguments
///
/// * `source` - 本文.
/// * `pos` - 位置.
/// * `arg` - 越える数 (1 以上).
///
/// # Errors
///
/// 括弧が合わないとき.
pub fn end_of_defun(source: &Source, pos: usize, arg: i64) -> Result<usize, MotionError> {
    // defun の後ろの空白と, 改行かコメントまでを飛ばす
    let skip = |mut p: usize| {
        if line_start(source, p) == p {
            return p;
        }
        while matches!(source.char_at(p), Some(' ' | '\t')) {
            p += 1;
        }
        if source.is_comment_start(p) || source.char_at(p) == Some('\n') {
            next_line(source, p)
        } else {
            p
        }
    };
    let scan = |from: usize| {
        forward_sexp(source, from).map_err(|e| MotionError {
            point: from,
            message: e.message.to_string(),
        })
    };
    let mut arg = arg.max(1);
    let beg = beginning_of_defun_raw(source, line_end(source, pos), false).0;
    let mut p = scan(beg)?;
    if arg <= 1 {
        p = skip(p);
    }
    if p > pos {
        // defun の中から始めたので, 1 つ越えた
        arg -= 1;
    } else {
        p = pos;
    }
    // 残りの数だけ, 次の defun の終わりへ
    if arg > 0 {
        let mut q = p;
        let mut found = true;
        for _ in 0..arg {
            let (at, ok) = beginning_of_defun_raw(source, q, true);
            q = at;
            found = ok;
            if !ok {
                break;
            }
        }
        p = if found { scan(q)? } else { q };
    }
    Ok(skip(p))
}

/// 行が, 空白とコメントだけなら `true` (Emacs の `beginning-of-defun--in-emptyish-line-p`).
fn emptyish_line(source: &Source, pos: usize) -> bool {
    let start = line_start(source, pos);
    let state = source.parse_state(start);
    if state.in_string {
        return false;
    }
    let from = if state.in_comment {
        state.start.unwrap_or(start)
    } else {
        start
    };
    line_end(source, pos) < source.forward_comment(from, usize::MAX)
}

/// defun の始めへ動き, 前に続くコメントの行の始めへ動きます (Emacs の `beginning-of-defun-comments`).
fn beginning_of_defun_comments(source: &Source, pos: usize) -> usize {
    let mut p = beginning_of_defun(source, pos);
    let first_line = loop {
        // 前の行へ (Emacs の `(forward-line -1)`)
        let start = line_start(source, p);
        let first_line = start == 0;
        p = if first_line {
            0
        } else {
            line_start(source, start - 1)
        };
        // 行の終わりが, 前の行から続くコメントの中なら, コメントの始めの行へ
        p = line_end(source, p);
        let mut state = source.parse_state(p);
        while let Some(comment) = state
            .start
            .filter(|&s| state.in_comment && s < line_start(source, p))
        {
            p = line_end(source, comment);
            state = source.parse_state(p);
        }
        if first_line {
            break true;
        }
        while p > line_start(source, p) && source.is_whitespace(p - 1) {
            p -= 1;
        }
        // 空の行か, コメントでない文字がある行なら, 止める
        if p == line_start(source, p) || !emptyish_line(source, p) {
            break false;
        }
    };
    if first_line {
        line_start(source, p)
    } else {
        next_line(source, p)
    }
}

/// defun を選ぶ範囲を返します (Emacs の `mark-defun`). 前のコメントと, 前の空の行も含めます.
/// リージョンがあれば, マークを次の defun の終わりへ延ばします.
///
/// # Arguments
///
/// * `source` - 本文.
/// * `pos` - カーソル.
/// * `mark` - リージョンのマーク (リージョンが無ければ `None`).
///
/// # Returns
///
/// カーソルとマーク.
///
/// # Errors
///
/// 括弧が合わないとき.
pub fn mark_defun(
    source: &Source,
    pos: usize,
    mark: Option<usize>,
) -> Result<(usize, usize), MotionError> {
    let (mut head, new_mark);
    match mark.filter(|&m| m != pos) {
        Some(m) => {
            // save-excursion の中なので, 失敗してもカーソルは動かない
            new_mark = end_of_defun(source, m, 1).map_err(|e| MotionError { point: pos, ..e })?;
            head = pos;
        }
        None => {
            let mut beg = beginning_of_defun_comments(source, pos);
            let mut end = end_of_defun(source, beg, 1)?;
            if end <= pos || beg == 0 {
                // 前の defun を見つけたので, 後ろの defun にする
                end = end_of_defun(source, pos, 1)?;
                beg = beginning_of_defun_comments(source, end);
            }
            new_mark = end_of_defun(source, beg, 1)?;
            head = beg;
        }
    }
    // 前の空の行も含める
    while head > 0 && (source.char_at(head - 1) == Some('\n') || source.is_whitespace(head - 1)) {
        head -= 1;
    }
    if head > 0 {
        head = next_line(source, head);
    }
    Ok((head, new_mark))
}
