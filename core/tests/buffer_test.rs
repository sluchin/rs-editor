//! バッファ (`buffer`) のテスト. 行と桁の期待値は, `emacs -Q --batch` の結果 (位置は Emacs が 1 から数えるので, 1 を引く).

use rseditor_core::buffer::{Buffer, UndoEntry, MARK_RING_MAX};

/// 行のテストに使う本文: `ab\ncde\n\nf` (Emacs の位置 1 から 10).
fn lines() -> Buffer {
    Buffer::with_text("lines", "ab\ncde\n\nf")
}

#[test]
fn test_basic_accessors() {
    let mut buffer = Buffer::new("a");
    assert_eq!(buffer.name(), "a");
    assert!(buffer.is_empty());
    assert_eq!(buffer.len(), 0);
    assert_eq!(buffer.file(), None);
    buffer.set_file(Some("/tmp/a".to_string()));
    assert_eq!(buffer.file(), Some("/tmp/a"));
    assert!(!buffer.modified());
    buffer.set_modified(true);
    assert!(buffer.modified());
    let buffer = Buffer::with_text("b", "あいう");
    assert_eq!(buffer.len(), 3);
    assert_eq!(buffer.text(), "あいう");
    assert!(!buffer.modified());
    assert!(buffer.undo_list().is_empty());
}

#[test]
fn test_substring_and_chars() {
    let buffer = lines();
    assert_eq!(buffer.substring(0, 2).unwrap(), "ab");
    // 始めと終わりは逆でもよい
    assert_eq!(buffer.substring(6, 3).unwrap(), "cde");
    assert_eq!(buffer.substring(9, 9).unwrap(), "");
    assert_eq!(buffer.substring(0, 99).unwrap_err(), "Args out of range");
    assert_eq!(buffer.char_after(0), Some('a'));
    assert_eq!(buffer.char_after(2), Some('\n'));
    assert_eq!(buffer.char_after(9), None);
    assert_eq!(buffer.char_before(0), None);
    assert_eq!(buffer.char_before(1), Some('a'));
    assert_eq!(buffer.char_before(9), Some('f'));
    assert_eq!(buffer.char_before(10), None);
}

#[test]
fn test_point_and_predicates() {
    let mut buffer = lines();
    assert!(buffer.bobp() && buffer.bolp() && !buffer.eobp() && !buffer.eolp());
    buffer.set_point(2);
    assert!(!buffer.bobp() && !buffer.bolp() && buffer.eolp());
    buffer.set_point(3);
    assert!(buffer.bolp() && !buffer.eolp());
    buffer.set_point(99);
    assert_eq!(buffer.point(), 9);
    assert!(buffer.eobp() && buffer.eolp() && !buffer.bolp());
    // 空のバッファ
    let empty = Buffer::new("e");
    assert!(empty.bobp() && empty.eobp() && empty.bolp() && empty.eolp());
}

#[test]
fn test_forward_char() {
    let mut buffer = lines();
    buffer.forward_char(5).unwrap();
    assert_eq!(buffer.point(), 5);
    buffer.forward_char(-2).unwrap();
    assert_eq!(buffer.point(), 3);
    // 端を越えるなら, 端まで動いて失敗にする (Emacs と同じ)
    assert_eq!(buffer.forward_char(-5).unwrap_err(), "Beginning of buffer");
    assert_eq!(buffer.point(), 0);
    assert_eq!(buffer.forward_char(100).unwrap_err(), "End of buffer");
    assert_eq!(buffer.point(), 9);
}

#[test]
fn test_line_positions() {
    // 位置は Emacs の 1 から 10 を 0 から 9 にしたもの: (カーソル, -5 0 1 2 3 9 のときの結果)
    let begin: [(usize, [usize; 6]); 8] = [
        (0, [0, 0, 0, 3, 7, 9]),
        (1, [0, 0, 0, 3, 7, 9]),
        (2, [0, 0, 0, 3, 7, 9]),
        (3, [0, 0, 3, 7, 8, 9]),
        (5, [0, 0, 3, 7, 8, 9]),
        (7, [0, 3, 7, 8, 9, 9]),
        (8, [0, 7, 8, 9, 9, 9]),
        (9, [0, 7, 8, 9, 9, 9]),
    ];
    let end: [(usize, [usize; 6]); 8] = [
        (0, [0, 0, 2, 6, 7, 9]),
        (1, [0, 0, 2, 6, 7, 9]),
        (2, [0, 0, 2, 6, 7, 9]),
        (3, [0, 2, 6, 7, 9, 9]),
        (5, [0, 2, 6, 7, 9, 9]),
        (7, [0, 6, 7, 9, 9, 9]),
        (8, [0, 7, 9, 9, 9, 9]),
        (9, [0, 7, 9, 9, 9, 9]),
    ];
    let mut buffer = lines();
    for (point, expected) in begin {
        buffer.set_point(point);
        let got: Vec<usize> = [-5, 0, 1, 2, 3, 9]
            .iter()
            .map(|n| buffer.line_beginning_position(*n))
            .collect();
        assert_eq!(got, expected, "line-beginning-position at {point}");
    }
    for (point, expected) in end {
        buffer.set_point(point);
        let got: Vec<usize> = [-5, 0, 1, 2, 3, 9]
            .iter()
            .map(|n| buffer.line_end_position(*n))
            .collect();
        assert_eq!(got, expected, "line-end-position at {point}");
    }
}

#[test]
fn test_forward_line() {
    // (始めの位置, 行の数, 動けなかった行の数, 動いた先)
    let cases: [(usize, i64, i64, usize); 15] = [
        (0, 1, 0, 3),
        (0, 2, 0, 7),
        (0, 3, 0, 8),
        (0, 4, 0, 9),
        (0, 5, 1, 9),
        (0, 9, 5, 9),
        (4, -1, 0, 0),
        (4, -2, -1, 0),
        (4, -9, -8, 0),
        (9, 1, 1, 9),
        (9, 0, 0, 8),
        (9, -1, 0, 7),
        (8, 1, 0, 9),
        (7, 1, 0, 8),
        (7, 0, 0, 7),
    ];
    for (start, n, shortage, end) in cases {
        let mut buffer = lines();
        buffer.set_point(start);
        assert_eq!(
            buffer.forward_line(n),
            shortage,
            "forward-line {n} from {start}"
        );
        assert_eq!(
            buffer.point(),
            end,
            "point after forward-line {n} from {start}"
        );
    }
    // 終わりに改行がある本文: 空の最後の行へ動ける
    let mut buffer = Buffer::with_text("n", "a\n");
    assert_eq!(buffer.forward_line(1), 0);
    assert_eq!(buffer.point(), 2);
    assert_eq!(buffer.forward_line(1), 1);
    // 空のバッファ
    let mut empty = Buffer::new("e");
    assert_eq!(empty.forward_line(1), 1);
    assert_eq!(empty.forward_line(-1), -1);
}

#[test]
fn test_current_column_and_move_to_column() {
    // a, タブ, b, あ (幅 2), c: 桁は 0, 1, 8, 9, 11, 12
    let mut buffer = Buffer::with_text("c", "a\tb\u{3042}c");
    for (point, column) in [(0, 0), (1, 1), (2, 8), (3, 9), (4, 11), (5, 12)] {
        buffer.set_point(point);
        assert_eq!(buffer.current_column(), column, "current-column at {point}");
    }
    // (桁, 動いた先の桁, 位置): タブと幅の広い文字を越えることがある
    for (target, column, point) in [
        (0, 0, 0),
        (1, 1, 1),
        (3, 8, 2),
        (8, 8, 2),
        (9, 9, 3),
        (10, 11, 4),
        (11, 11, 4),
        (99, 12, 5),
    ] {
        buffer.set_point(0);
        assert_eq!(
            buffer.move_to_column(target),
            column,
            "move-to-column {target}"
        );
        assert_eq!(buffer.point(), point, "point after move-to-column {target}");
    }
    // 行の終わりを越えては動かない
    let mut buffer = Buffer::with_text("c", "ab\ncd");
    buffer.set_point(4);
    assert_eq!(buffer.move_to_column(99), 2);
    assert_eq!(buffer.point(), 5);
}

#[test]
fn test_insert_moves_point_and_marks() {
    let mut buffer = Buffer::with_text("i", "abcd");
    buffer.set_point(2);
    buffer.push_mark(3, true);
    buffer.push_mark(1, true);
    buffer.insert("XY");
    assert_eq!(buffer.text(), "abXYcd");
    assert_eq!(buffer.point(), 4);
    // 入れた所より後ろのマークはずれ, 前のマークはそのまま
    assert_eq!(buffer.mark(), Some(1));
    buffer.pop_mark();
    assert_eq!(buffer.mark(), Some(5));
    assert!(buffer.modified());
    // 空の文字は何もしない
    let before = buffer.undo_list().len();
    buffer.insert("");
    assert_eq!(buffer.undo_list().len(), before);
    // 位置を指して入れる: カーソルより前なら, カーソルはずれる
    buffer.insert_at(0, "Z").unwrap();
    assert_eq!(buffer.point(), 5);
    // カーソルと同じ位置に入れても, カーソルは動かない
    buffer.insert_at(5, "W").unwrap();
    assert_eq!(buffer.point(), 5);
    assert_eq!(buffer.insert_at(99, "x").unwrap_err(), "Args out of range");
}

#[test]
fn test_delete_region() {
    let mut buffer = Buffer::with_text("d", "abcdef");
    buffer.set_point(5);
    buffer.push_mark(3, false);
    buffer.push_mark(4, false);
    // 逆の範囲でもよい. 消した範囲の中のカーソルとマークは, 消した所の始めになる
    assert_eq!(buffer.delete_region(4, 2).unwrap(), "cd");
    assert_eq!(buffer.text(), "abef");
    assert_eq!(buffer.point(), 3);
    assert_eq!(buffer.mark(), Some(2));
    buffer.pop_mark();
    assert_eq!(buffer.mark(), Some(2));
    // 空の範囲は何もしない. バッファの外は失敗にする
    let before = buffer.undo_list().len();
    assert_eq!(buffer.delete_region(1, 1).unwrap(), "");
    assert_eq!(buffer.undo_list().len(), before);
    assert_eq!(
        buffer.delete_region(0, 99).unwrap_err(),
        "Args out of range"
    );
    // 消した範囲より前のカーソルは動かない
    buffer.set_point(0);
    buffer.delete_region(1, 2).unwrap();
    assert_eq!(buffer.point(), 0);
}

#[test]
fn test_mark_and_region() {
    let mut buffer = Buffer::with_text("m", "abcdef");
    let message = "The mark is not set now, so there is no region";
    assert_eq!(buffer.region_beginning().unwrap_err(), message);
    assert_eq!(buffer.region_end().unwrap_err(), message);
    assert!(!buffer.use_region_p());
    assert_eq!(buffer.mark(), None);
    // set-mark は, マークを使う状態にする
    buffer.set_mark(4);
    buffer.set_point(1);
    assert_eq!(
        (buffer.region_beginning(), buffer.region_end()),
        (Ok(1), Ok(4))
    );
    assert!(buffer.use_region_p());
    // カーソルがマークの後ろでもよい. 空のリージョンは使わない
    buffer.set_point(5);
    assert_eq!(
        (buffer.region_beginning(), buffer.region_end()),
        (Ok(4), Ok(5))
    );
    buffer.set_point(4);
    assert!(!buffer.use_region_p());
    buffer.set_point(1);
    buffer.deactivate_mark();
    assert!(!buffer.use_region_p());
    assert_eq!(buffer.mark(), Some(4));
    // バッファの外は端にする
    buffer.set_mark(99);
    assert_eq!(buffer.mark(), Some(6));
}

#[test]
fn test_mark_ring() {
    let mut buffer = Buffer::with_text("r", "0123456789");
    // 履歴が空なら, pop-mark は何もしない
    buffer.pop_mark();
    assert_eq!(buffer.mark(), None);
    buffer.push_mark(1, false);
    assert!(!buffer.use_region_p());
    buffer.push_mark(2, true);
    buffer.push_mark(3, false);
    assert_eq!(buffer.mark(), Some(3));
    // 先頭の 2 がマークになり, 今のマーク 3 は履歴の最後に回る
    buffer.pop_mark();
    assert_eq!(buffer.mark(), Some(2));
    buffer.pop_mark();
    assert_eq!(buffer.mark(), Some(1));
    buffer.pop_mark();
    assert_eq!(buffer.mark(), Some(3));
    // 履歴の長さには上限がある
    let mut buffer = Buffer::with_text("r", "0123456789");
    for i in 0..(MARK_RING_MAX + 5) {
        buffer.push_mark(i % 10, false);
    }
    for _ in 0..(MARK_RING_MAX * 2) {
        buffer.pop_mark();
    }
    assert!(buffer.mark().is_some());
}

#[test]
fn test_undo_records_and_boundaries() {
    let mut buffer = Buffer::new("u");
    // 履歴が空なら, 区切りは入れない
    buffer.undo_boundary();
    assert!(buffer.undo_list().is_empty());
    // 続けて入れた文字は 1 つにまとまる
    buffer.insert("a");
    buffer.insert("bc");
    assert_eq!(buffer.undo_list(), [UndoEntry::Insert { pos: 0, len: 3 }]);
    buffer.undo_boundary();
    buffer.undo_boundary();
    assert_eq!(buffer.undo_list().len(), 2);
    // 離れた所に入れたら別にする
    buffer.insert_at(0, "x").unwrap();
    assert_eq!(buffer.undo_list()[2], UndoEntry::Insert { pos: 0, len: 1 });
    // 消した文字は, 文字と, カーソルが終わりにあったかを覚える
    buffer.set_point(4);
    buffer.delete_region(2, 4).unwrap();
    assert_eq!(
        buffer.undo_list()[3],
        UndoEntry::Delete {
            pos: 2,
            text: "bc".to_string(),
            point_at_end: true
        }
    );
    buffer.set_point(0);
    buffer.delete_region(0, 1).unwrap();
    assert_eq!(
        buffer.undo_list()[4],
        UndoEntry::Delete {
            pos: 0,
            text: "x".to_string(),
            point_at_end: false
        }
    );
}

#[test]
fn test_undo_groups_and_primitive_undo() {
    let mut buffer = Buffer::new("u");
    buffer.insert("hello");
    buffer.undo_boundary();
    buffer.set_point(0);
    buffer.insert("> ");
    buffer.undo_boundary();
    buffer.set_point(5);
    buffer.delete_region(2, 5).unwrap();
    buffer.undo_boundary();
    assert_eq!(buffer.text(), "> lo");
    // 新しいまとまりから 1 つ取り出す. 区切りは飛ばす
    let end = buffer.undo_list().len();
    let (entries, end) = buffer.undo_groups(end, 1);
    assert_eq!(entries.len(), 1);
    buffer.primitive_undo(&entries).unwrap();
    assert_eq!(buffer.text(), "> hello");
    // 消す前のカーソルは終わりにあった. 元に戻すと, 入れ直した文字の終わりに置く
    assert_eq!(buffer.point(), 5);
    // 続けて取り出す (元に戻すことも履歴に入るが, 範囲を指すので影響しない)
    let (entries, end) = buffer.undo_groups(end, 1);
    buffer.primitive_undo(&entries).unwrap();
    assert_eq!(buffer.text(), "hello");
    assert_eq!(buffer.point(), 0);
    let (entries, _) = buffer.undo_groups(end, 5);
    buffer.primitive_undo(&entries).unwrap();
    assert_eq!(buffer.text(), "");
    // 履歴を使い切ったら, 何も取り出さない. 範囲の終わりは履歴の長さに収める
    let (entries, end) = buffer.undo_groups(0, 3);
    assert!(entries.is_empty());
    assert_eq!(end, 0);
    let (_, end) = buffer.undo_groups(99, 0);
    assert_eq!(end, buffer.undo_list().len());
    // 元に戻したことを, 元に戻す (redo)
    let len = buffer.undo_list().len();
    buffer.undo_boundary();
    let (entries, _) = buffer.undo_groups(buffer.undo_list().len(), 1);
    assert!(!entries.is_empty());
    buffer.primitive_undo(&entries).unwrap();
    assert!(buffer.undo_list().len() > len);
}

#[test]
fn test_primitive_undo_errors_and_boundaries() {
    let mut buffer = Buffer::with_text("u", "abc");
    // 区切りは何もしない
    buffer.primitive_undo(&[UndoEntry::Boundary]).unwrap();
    assert_eq!(buffer.text(), "abc");
    // 本文に合わない記録は, 失敗にする
    let entry = UndoEntry::Insert { pos: 10, len: 2 };
    assert_eq!(
        buffer.primitive_undo(&[entry]).unwrap_err(),
        "Args out of range"
    );
    let entry = UndoEntry::Delete {
        pos: 10,
        text: "x".to_string(),
        point_at_end: false,
    };
    assert_eq!(
        buffer.primitive_undo(&[entry]).unwrap_err(),
        "Args out of range"
    );
}
