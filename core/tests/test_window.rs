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

/// 位置と大きさが, 計算の誤差の範囲で等しいか調べる.
fn assert_near(a: WindowRect, b: WindowRect) {
    let near = |x: f64, y: f64| (x - y).abs() < 1e-9;
    assert!(
        a.id == b.id
            && near(a.left, b.left)
            && near(a.top, b.top)
            && near(a.width, b.width)
            && near(a.height, b.height),
        "{a:?} != {b:?}"
    );
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

#[test]
fn test_split_root() {
    let layout = Layout::Window(1)
        .split(1, 2, Direction::Right)
        .split_root(3, Direction::Below);
    assert_eq!(layout.window_ids(), vec![1, 2, 3]);
    assert_eq!(
        layout.rects(FULL),
        vec![
            rect(1, 0.0, 0.0, 50.0, 50.0),
            rect(2, 50.0, 0.0, 50.0, 50.0),
            rect(3, 0.0, 50.0, 100.0, 50.0)
        ]
    );
}

#[test]
fn test_balance() {
    // 横に 3 つ並べると, 1 つ目は 1/3 を使う
    let mut row = Layout::Window(1)
        .split(1, 2, Direction::Right)
        .split(2, 3, Direction::Right);
    row.resize(1, Direction::Right, 30.0, FULL);
    row.balance();
    let widths: Vec<f64> = row.rects(FULL).iter().map(|r| r.width).collect();
    assert!(widths.iter().all(|w| (w - 100.0 / 3.0).abs() < 1e-9));
    // 向きの違う分け方は, 1 つと数える
    let mut mixed = Layout::Window(1)
        .split(1, 2, Direction::Below)
        .split(2, 3, Direction::Right);
    mixed.resize(1, Direction::Below, 20.0, FULL);
    mixed.balance();
    assert_eq!(mixed.rects(FULL)[0], rect(1, 0.0, 0.0, 100.0, 50.0));
    // 1 つだけなら, 何も変わらない
    let mut one = Layout::Window(1);
    one.balance();
    assert_eq!(one, Layout::Window(1));
}

#[test]
fn test_resize() {
    let mut layout = Layout::Window(1)
        .split(1, 2, Direction::Below)
        .split(2, 3, Direction::Right);
    // 上のウィンドウを高くすると, 下が低くなる
    assert!(layout.resize(1, Direction::Below, 10.0, FULL));
    assert_near(layout.rects(FULL)[0], rect(1, 0.0, 0.0, 100.0, 60.0));
    assert_near(layout.rects(FULL)[1], rect(2, 0.0, 60.0, 50.0, 40.0));
    // 下のウィンドウを低くする (下の組の割合が変わる)
    assert!(layout.resize(3, Direction::Below, -20.0, FULL));
    assert_near(layout.rects(FULL)[0], rect(1, 0.0, 0.0, 100.0, 80.0));
    // 横は, 内側の分け方で変える
    assert!(layout.resize(2, Direction::Right, 10.0, FULL));
    assert_near(layout.rects(FULL)[1], rect(2, 0.0, 80.0, 60.0, 20.0));
    assert!(layout.resize(3, Direction::Right, 10.0, FULL));
    assert_near(layout.rects(FULL)[1], rect(2, 0.0, 80.0, 50.0, 20.0));
    // 割合は 5% から 95% まで
    assert!(layout.resize(1, Direction::Below, 500.0, FULL));
    assert!((layout.rects(FULL)[0].height - 95.0).abs() < 1e-9);
    assert!(layout.resize(1, Direction::Below, -500.0, FULL));
    assert!((layout.rects(FULL)[0].height - 5.0).abs() < 1e-9);
    // その向きに分けていないウィンドウと, 無いウィンドウは, 変えられない
    assert!(!layout.resize(1, Direction::Right, 10.0, FULL));
    assert!(!layout.resize(9, Direction::Below, 10.0, FULL));
    assert!(!Layout::Window(1).resize(1, Direction::Below, 10.0, FULL));
}
