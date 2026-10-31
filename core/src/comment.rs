//! Lisp のコメントの付け外し (Emacs の `comment-dwim`. `M-;`). コメントは `;` で始めます.
//!
//! - リージョンがあれば, リージョンの行をコメントにします. 全ての行がコメントなら, コメントを外します.
//! - リージョンが無ければ, カーソルの行の, コメントへ動きます. コメントが無ければ, 作ります
//!   (空白だけの行は, 式と同じ字下げで `;; ` を, 式の後ろは `comment-column` (40 桁) に `; ` を).

use crate::indent::{
    column_at, edits_of, indent_length, indent_lines, line_starts, map_pos, whitespace, Config,
    Edit, IndentLines, LispParser, Plan,
};
use crate::syntax::Dialect;

/// リージョンをコメントにするときに, 行に付ける文字 (Emacs の `comment-add` が 1 なので, `;` 2 つ).
const REGION_COMMENT: &str = ";; ";

/// 行の始めの, 字下げと `;` の並びと, 後ろの空白 1 つの長さ. コメントの行でなければ `None`.
fn line_comment(text: &[char]) -> Option<(usize, usize)> {
    let indent = indent_length(text);
    let semis = text[indent..].iter().take_while(|c| **c == ';').count();
    if semis == 0 {
        return None;
    }
    let after = indent + semis;
    Some((indent, after + usize::from(text.get(after) == Some(&' '))))
}

/// 行ごとの, コメントの `;` の位置を返します (文字列の中の `;` を除く. 行の始めから. コメントが無ければ `None`).
fn comment_starts(lines: &[Vec<char>], config: &Config) -> Vec<Option<usize>> {
    // 文字列は, 行をまたぐので, 始めから読む
    let mut parser = LispParser::new(config.dialect == Dialect::Scheme, config.tab_width);
    lines
        .iter()
        .enumerate()
        .map(|(n, text)| {
            parser.scan_line(text, n, 0, text.len());
            parser.comment
        })
        .collect()
}

/// 式の後ろのコメントの桁: 前の行に, 式の後ろのコメントがあれば, その桁. 無ければ `comment_column`.
fn trailing_column(
    lines: &[Vec<char>],
    starts: &[Option<usize>],
    n: usize,
    comment_column: usize,
    config: &Config,
) -> usize {
    let above = n.checked_sub(2).and_then(|k| Some((k, starts[k]?)));
    match above {
        Some((k, c)) if !lines[k][..c].iter().all(|ch| matches!(ch, ' ' | '\t')) => {
            column_at(&lines[k], c, config.tab_width)
        }
        _ => comment_column,
    }
}

/// 行の, 桁の位置を返します (字下げの中で, 桁に届く始めの位置).
fn index_at_column(text: &[char], col: usize, tab_width: usize) -> usize {
    let mut i = 0;
    while column_at(text, i, tab_width) < col {
        i += 1;
    }
    i
}

/// 行 `n` (1 から) の, `c` から始まるコメントを, Emacs の `comment-indent` のように置き直して, コメントの文字の後ろ
/// (`;` の並びと, 続く空白の後) へ動きます. 行だけのコメントは, 式と同じ字下げに, `;` 1 つの式の後ろのコメントは,
/// `comment_column` (式がそれより長ければ, 空白 1 つの後) にします. `;;` 以上の式の後ろのコメントは, 動かしません.
fn move_to_comment(
    lines: &[Vec<char>],
    n: usize,
    c: usize,
    comment_column: usize,
    config: &Config,
) -> Plan {
    let starts = line_starts(lines);
    let from = starts[n - 1];
    let text = &lines[n - 1];
    let semis = text[c..].iter().take_while(|ch| **ch == ';').count();
    let (edits, shift) = if text[..c].iter().all(|ch| matches!(ch, ' ' | '\t')) {
        let options = IndentLines {
            from: n,
            to: n,
            ..IndentLines::default()
        };
        let edits = edits_of(lines, &indent_lines(lines, options, config));
        let after = edits.first().map_or(c, |e| e.insert.chars().count());
        (edits, after)
    } else if semis == 1 {
        let mut code = c;
        while matches!(text[code - 1], ' ' | '\t') {
            code -= 1;
        }
        let col = column_at(text, code, config.tab_width);
        let insert = whitespace(col, comment_column.max(col + 1), config);
        let after = code + insert.chars().count();
        let edit = Edit {
            from: from + code,
            to: from + c,
            insert,
        };
        (vec![edit], after)
    } else {
        (Vec::new(), c)
    };
    let spaces = text[c + semis..]
        .iter()
        .take_while(|ch| matches!(ch, ' ' | '\t'))
        .count();
    Plan {
        edits,
        cursor: Some(from + shift + semis + spaces),
    }
}

/// リージョンの行をコメントにするか, 全ての行がコメントなら, コメントを外します
/// (Emacs の `comment-or-uncomment-region`). 空白だけの行は, 変えません.
/// 終わりが行の始めなら, その行は含めません (`indent-region` と同じ).
pub fn comment_region(
    lines: &[Vec<char>],
    from: usize,
    to: usize,
    pos: usize,
    config: &Config,
) -> Plan {
    let starts = line_starts(lines);
    let line_of = |p: usize| starts.partition_point(|&s| s <= p);
    let end = line_of(to);
    let last = if to == starts[end - 1] { end - 1 } else { end };
    let targets: Vec<usize> = (line_of(from)..=last)
        .filter(|n| !lines[n - 1].iter().all(|c| matches!(c, ' ' | '\t')))
        .collect();
    let all_comments = !targets.is_empty()
        && targets
            .iter()
            .all(|n| line_comment(&lines[n - 1]).is_some());
    let edits: Vec<Edit> = if all_comments {
        targets
            .iter()
            .map(|n| {
                let (indent, len) = line_comment(&lines[n - 1]).unwrap_or_default();
                Edit {
                    from: starts[n - 1] + indent,
                    to: starts[n - 1] + len,
                    insert: String::new(),
                }
            })
            .collect()
    } else {
        // コメントの `;` は, 一番浅い字下げの桁にそろえる
        let col = targets
            .iter()
            .map(|n| {
                column_at(
                    &lines[n - 1],
                    indent_length(&lines[n - 1]),
                    config.tab_width,
                )
            })
            .min()
            .unwrap_or(0);
        targets
            .iter()
            .map(|n| {
                let at = starts[n - 1] + index_at_column(&lines[n - 1], col, config.tab_width);
                Edit {
                    from: at,
                    to: at,
                    insert: REGION_COMMENT.to_string(),
                }
            })
            .collect()
    };
    Plan {
        cursor: Some(map_pos(&edits, pos, false)),
        edits,
    }
}

/// コメントを付けるか, コメントへ動きます (Emacs の `comment-dwim`). `mark` が `pos` と違えば, リージョンを変えます.
/// コメントの書き方が無い (Lisp でない) 場合は, 知らせる文字を返します.
pub fn comment_dwim(
    lines: &[Vec<char>],
    pos: usize,
    mark: Option<usize>,
    comment_column: usize,
    config: &Config,
) -> Result<Plan, String> {
    if config.dialect == Dialect::Fundamental {
        return Err("No comment syntax is defined".to_string());
    }
    if let Some(m) = mark.filter(|m| *m != pos) {
        return Ok(comment_region(lines, m.min(pos), m.max(pos), pos, config));
    }
    let starts = line_starts(lines);
    let n = starts.partition_point(|&s| s <= pos);
    let text = &lines[n - 1];
    let from = starts[n - 1];
    let found = comment_starts(lines, config);
    if let Some(c) = found[n - 1] {
        let column = trailing_column(lines, &found, n, comment_column, config);
        return Ok(move_to_comment(lines, n, c, column, config));
    }
    if text.iter().all(|c| matches!(c, ' ' | '\t')) {
        // 空白だけの行は, 式と同じ字下げにして, `;; ` を入れる
        let options = IndentLines {
            from: n,
            to: n,
            empty_lines: true,
            ..IndentLines::default()
        };
        let result = indent_lines(lines, options, config);
        let base = match edits_of(lines, &result).pop() {
            Some(edit) => edit.insert,
            None => text.iter().collect(),
        };
        let cursor = from + base.chars().count() + REGION_COMMENT.len();
        return Ok(Plan {
            edits: vec![Edit {
                from,
                to: from + text.len(),
                insert: base + REGION_COMMENT,
            }],
            cursor: Some(cursor),
        });
    }
    // 式の後ろは, 後ろの空白を除いて, comment-column (式がそれより長ければ, 空白 1 つの後) に `; ` を入れる
    let mut code = text.len();
    while matches!(text[code - 1], ' ' | '\t') {
        code -= 1;
    }
    let col = column_at(text, code, config.tab_width);
    let column = trailing_column(lines, &found, n, comment_column, config);
    let insert = whitespace(col, column.max(col + 1), config) + ";";
    let cursor = from + code + insert.chars().count();
    Ok(Plan {
        edits: vec![Edit {
            from: from + code,
            to: from + text.len(),
            insert,
        }],
        cursor: Some(cursor),
    })
}
