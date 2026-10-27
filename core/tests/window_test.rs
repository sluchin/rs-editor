//! ウィンドウの分け方 (`window`) のテスト. `tests/frontend/lib/windows.test.ts` と同じ計算.

use rseditor_core::window::{Area, Direction, Layout, WindowRect, FULL};

fn rect(id: usize, left: f64, top: f64, width: f64, height: f64) -> WindowRect {
    WindowRect {
        id,
        left,
        top,
        width,
        height,
    }
}

#[test]
fn test_split_order_and_rects() {
    let one = Layout::Window(1);
    let below = one.clone().split(1, 2, Direction::Below);
    let both = below.split(2, 3, Direction::Right);
    assert_eq!(both.window_ids(), vec![1, 2, 3]);
    assert_eq!(
        both.rects(FULL),
        vec![
            rect(1, 0.0, 0.0, 100.0, 50.0),
            rect(2, 0.0, 50.0, 50.0, 50.0),
            rect(3, 50.0, 50.0, 50.0, 50.0)
        ]
    );
    // 無いウィンドウは, 分けない
    assert_eq!(one.clone().split(9, 8, Direction::Below), one);
    // 範囲を指す
    let area = Area {
        left: 10.0,
        top: 20.0,
        width: 40.0,
        height: 60.0,
    };
    assert_eq!(one.rects(area), vec![rect(1, 10.0, 20.0, 40.0, 60.0)]);
}

#[test]
fn test_delete() {
    let one = Layout::Window(1);
    let layout = one
        .clone()
        .split(1, 2, Direction::Below)
        .split(2, 3, Direction::Right);
    // 隣のウィンドウが, 空いた所を使う
    assert_eq!(layout.clone().delete(1).unwrap().window_ids(), vec![2, 3]);
    assert_eq!(layout.clone().delete(2).unwrap().window_ids(), vec![1, 3]);
    assert_eq!(layout.clone().delete(3).unwrap().window_ids(), vec![1, 2]);
    assert_eq!(layout.clone().delete(9), Some(layout));
    // 1 つだけなら消さない
    assert_eq!(one.delete(1), None);
}
