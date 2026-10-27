//! ウィンドウの分割 (`C-x 2`, `C-x 3`, `C-x o`, `C-x 0`, `C-x 1`).
//!
//! 分け方は木で持ちます. 描くときは, 各ウィンドウの位置と大きさ (画面に対する %) を計算します.
//! 画面 (`src/lib/windows.ts`) と同じ計算です.

/// 分ける向き.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// 上下に分ける (`C-x 2`).
    Below,
    /// 左右に分ける (`C-x 3`).
    Right,
}

/// ウィンドウの分け方.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Layout {
    /// 1 つのウィンドウ (番号).
    Window(usize),
    /// 2 つに分けたもの.
    Split {
        /// 分けた向き.
        direction: Direction,
        /// 上か左.
        first: Box<Layout>,
        /// 下か右.
        second: Box<Layout>,
    },
}

/// ウィンドウの位置と大きさ (画面に対する %).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindowRect {
    /// ウィンドウの番号.
    pub id: usize,
    /// 左端の位置.
    pub left: f64,
    /// 上端の位置.
    pub top: f64,
    /// 幅.
    pub width: f64,
    /// 高さ.
    pub height: f64,
}

/// 分け方が使う範囲 (%).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Area {
    /// 左端の位置.
    pub left: f64,
    /// 上端の位置.
    pub top: f64,
    /// 幅.
    pub width: f64,
    /// 高さ.
    pub height: f64,
}

/// 画面の全体.
pub const FULL: Area = Area {
    left: 0.0,
    top: 0.0,
    width: 100.0,
    height: 100.0,
};

impl Layout {
    /// ウィンドウを, 左上から順に返します (`C-x o` で移る順).
    pub fn window_ids(&self) -> Vec<usize> {
        match self {
            Layout::Window(id) => vec![*id],
            Layout::Split { first, second, .. } => {
                let mut ids = first.window_ids();
                ids.extend(second.window_ids());
                ids
            }
        }
    }

    /// ウィンドウを 2 つに分けます. 新しいウィンドウは, 下か右です.
    ///
    /// # Arguments
    ///
    /// * `id` - 分けるウィンドウ (無ければ, 分けない).
    /// * `new_id` - 新しいウィンドウ.
    /// * `direction` - 分ける向き.
    pub fn split(self, id: usize, new_id: usize, direction: Direction) -> Layout {
        match self {
            Layout::Window(window) if window == id => Layout::Split {
                direction,
                first: Box::new(Layout::Window(window)),
                second: Box::new(Layout::Window(new_id)),
            },
            Layout::Window(_) => self,
            Layout::Split {
                direction: d,
                first,
                second,
            } => Layout::Split {
                direction: d,
                first: Box::new(first.split(id, new_id, direction)),
                second: Box::new(second.split(id, new_id, direction)),
            },
        }
    }

    /// ウィンドウを消します. 隣のウィンドウが, 空いた所を使います.
    ///
    /// # Returns
    ///
    /// 新しい分け方. ウィンドウが 1 つだけで, それを消すなら `None`.
    pub fn delete(self, id: usize) -> Option<Layout> {
        match self {
            Layout::Window(window) => (window != id).then_some(self),
            Layout::Split {
                direction,
                first,
                second,
            } => match (first.delete(id), second.delete(id)) {
                (None, second) => second,
                (first, None) => first,
                (Some(first), Some(second)) => Some(Layout::Split {
                    direction,
                    first: Box::new(first),
                    second: Box::new(second),
                }),
            },
        }
    }

    /// ウィンドウの位置と大きさを計算します (分けるときは, 半分ずつ).
    pub fn rects(&self, area: Area) -> Vec<WindowRect> {
        let Area {
            left,
            top,
            width,
            height,
        } = area;
        match self {
            Layout::Window(id) => vec![WindowRect {
                id: *id,
                left,
                top,
                width,
                height,
            }],
            Layout::Split {
                direction,
                first,
                second,
            } => {
                let (a, b) = match direction {
                    Direction::Below => (
                        Area {
                            height: height / 2.0,
                            ..area
                        },
                        Area {
                            top: top + height / 2.0,
                            height: height / 2.0,
                            ..area
                        },
                    ),
                    Direction::Right => (
                        Area {
                            width: width / 2.0,
                            ..area
                        },
                        Area {
                            left: left + width / 2.0,
                            width: width / 2.0,
                            ..area
                        },
                    ),
                };
                let mut rects = first.rects(a);
                rects.extend(second.rects(b));
                rects
            }
        }
    }
}
