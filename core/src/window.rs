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
#[derive(Debug, Clone, PartialEq)]
pub enum Layout {
    /// 1 つのウィンドウ (番号).
    Window(usize),
    /// 2 つに分けたもの.
    Split {
        /// 分けた向き.
        direction: Direction,
        /// 上か左が使う割合 (0 から 1. 分けた時は半分).
        ratio: f64,
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
                ratio: 0.5,
                first: Box::new(Layout::Window(window)),
                second: Box::new(Layout::Window(new_id)),
            },
            Layout::Window(_) => self,
            Layout::Split {
                direction: d,
                ratio,
                first,
                second,
            } => Layout::Split {
                direction: d,
                ratio,
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
                ratio,
                first,
                second,
            } => match (first.delete(id), second.delete(id)) {
                (None, second) => second,
                (first, None) => first,
                (Some(first), Some(second)) => Some(Layout::Split {
                    direction,
                    ratio,
                    first: Box::new(first),
                    second: Box::new(second),
                }),
            },
        }
    }

    /// 分け方の根を, 新しいウィンドウと並べます (`split-root-window-below`). 新しいウィンドウは, 下か右で半分を使う.
    pub fn split_root(self, new_id: usize, direction: Direction) -> Layout {
        Layout::Split {
            direction,
            ratio: 0.5,
            first: Box::new(self),
            second: Box::new(Layout::Window(new_id)),
        }
    }

    /// 向きの方向に並ぶウィンドウの数 (向きの違う分け方は, 多い方の数).
    fn span(&self, direction: Direction) -> usize {
        match self {
            Layout::Window(_) => 1,
            Layout::Split {
                direction: d,
                first,
                second,
                ..
            } => {
                let (a, b) = (first.span(direction), second.span(direction));
                if *d == direction {
                    a + b
                } else {
                    a.max(b)
                }
            }
        }
    }

    /// 全てのウィンドウを同じ大きさにします (`balance-windows`).
    pub fn balance(&mut self) {
        if let Layout::Split {
            direction,
            ratio,
            first,
            second,
        } = self
        {
            first.balance();
            second.balance();
            let (a, b) = (first.span(*direction), second.span(*direction));
            *ratio = a as f64 / (a + b) as f64;
        }
    }

    /// ウィンドウを含む, 向きの分け方の割合を変えて, ウィンドウの大きさを `delta` (画面に対する %) だけ変えます
    /// (`enlarge-window`. 負の数なら小さくする). 分け方の割合が 5% から 95% の外に出ないようにする.
    ///
    /// # Arguments
    ///
    /// * `id` - 大きさを変えるウィンドウ.
    /// * `direction` - 変える向き (`Below` なら高さ, `Right` なら幅).
    /// * `delta` - 変える量 (画面に対する %).
    /// * `area` - この分け方が使う範囲.
    ///
    /// # Returns
    ///
    /// 変えられたか (ウィンドウが, 向きの分け方に入っていないと `false`).
    pub fn resize(&mut self, id: usize, direction: Direction, delta: f64, area: Area) -> bool {
        let Layout::Split {
            direction: d,
            ratio,
            first,
            second,
        } = self
        else {
            return false;
        };
        let in_first = first.window_ids().contains(&id);
        if !in_first && !second.window_ids().contains(&id) {
            return false;
        }
        let (a, b) = split_area(*d, *ratio, area);
        let (child, child_area) = if in_first {
            (&mut **first, a)
        } else {
            (&mut **second, b)
        };
        // 内側の分け方で変えられるなら, そちらで変える
        if child.resize(id, direction, delta, child_area) {
            return true;
        }
        if *d != direction {
            return false;
        }
        let total = match direction {
            Direction::Below => area.height,
            Direction::Right => area.width,
        };
        let change = delta / total;
        let new = if in_first {
            *ratio + change
        } else {
            *ratio - change
        };
        *ratio = new.clamp(0.05, 0.95);
        true
    }

    /// ウィンドウの位置と大きさを計算します (割合に従う. 分けたときは半分ずつ).
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
                ratio,
                first,
                second,
            } => {
                let (a, b) = split_area(*direction, *ratio, area);
                let mut rects = first.rects(a);
                rects.extend(second.rects(b));
                rects
            }
        }
    }
}

/// 分け方の範囲を, 割合に従って 2 つに分けます (上か左, 下か右).
fn split_area(direction: Direction, ratio: f64, area: Area) -> (Area, Area) {
    match direction {
        Direction::Below => (
            Area {
                height: area.height * ratio,
                ..area
            },
            Area {
                top: area.top + area.height * ratio,
                height: area.height * (1.0 - ratio),
                ..area
            },
        ),
        Direction::Right => (
            Area {
                width: area.width * ratio,
                ..area
            },
            Area {
                left: area.left + area.width * ratio,
                width: area.width * (1.0 - ratio),
                ..area
            },
        ),
    }
}
