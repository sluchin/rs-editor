/**
 * ウィンドウの分割 (`C-x 2`, `C-x 3`, `C-x o`, `C-x 0`, `C-x 1`).
 *
 * 分け方は木で持ちます. 描くときは, 各ウィンドウの位置と大きさ (画面に対する %) を計算して, 平らに並べます
 * (分け方が変わっても, ウィンドウのエディタを作り直さないため).
 */

/** ウィンドウの分け方. */
export type Layout =
  | { kind: 'window'; id: string }
  | {
      kind: 'split'
      /** `below`: 上下に分ける (`C-x 2`). `right`: 左右に分ける (`C-x 3`). */
      direction: 'below' | 'right'
      first: Layout
      second: Layout
    }

/** ウィンドウの位置と大きさ (画面に対する %). */
export interface WindowRect {
  /** ウィンドウの ID. */
  id: string
  /** 左端の位置. */
  left: number
  /** 上端の位置. */
  top: number
  /** 幅. */
  width: number
  /** 高さ. */
  height: number
}

/**
 * ウィンドウを, 左上から順に返します (`C-x o` で移る順).
 *
 * @param layout - 分け方.
 * @returns ウィンドウの ID.
 */
export function windowIds(layout: Layout): string[] {
  return layout.kind === 'window'
    ? [layout.id]
    : [...windowIds(layout.first), ...windowIds(layout.second)]
}

/**
 * ウィンドウを 2 つに分けます. 新しいウィンドウは, 下か右です.
 *
 * @param layout - 分け方.
 * @param id - 分けるウィンドウ.
 * @param newId - 新しいウィンドウ.
 * @param direction - 分ける向き.
 * @returns 新しい分け方.
 */
export function splitWindow(
  layout: Layout,
  id: string,
  newId: string,
  direction: 'below' | 'right',
): Layout {
  if (layout.kind === 'window') {
    return layout.id === id
      ? { kind: 'split', direction, first: layout, second: { kind: 'window', id: newId } }
      : layout
  }
  return {
    ...layout,
    first: splitWindow(layout.first, id, newId, direction),
    second: splitWindow(layout.second, id, newId, direction),
  }
}

/**
 * ウィンドウを消します. 隣のウィンドウが, 空いた所を使います.
 *
 * @param layout - 分け方.
 * @param id - 消すウィンドウ.
 * @returns 新しい分け方. ウィンドウが 1 つだけなら `null`.
 */
export function deleteWindow(layout: Layout, id: string): Layout | null {
  if (layout.kind === 'window') return layout.id === id ? null : layout
  const first = deleteWindow(layout.first, id)
  const second = deleteWindow(layout.second, id)
  if (!first) return second
  if (!second) return first
  return { ...layout, first, second }
}

/**
 * ウィンドウの位置と大きさを計算します (分けるときは, 半分ずつ).
 *
 * @param layout - 分け方.
 * @param rect - 分け方が使う範囲 (%).
 * @returns ウィンドウの位置と大きさ.
 */
export function windowRects(
  layout: Layout,
  rect: Omit<WindowRect, 'id'> = { left: 0, top: 0, width: 100, height: 100 },
): WindowRect[] {
  if (layout.kind === 'window') return [{ id: layout.id, ...rect }]
  const { left, top, width, height } = rect
  const [a, b] =
    layout.direction === 'below'
      ? [
          { left, top, width, height: height / 2 },
          { left, top: top + height / 2, width, height: height / 2 },
        ]
      : [
          { left, top, width: width / 2, height },
          { left: left + width / 2, top, width: width / 2, height },
        ]
  return [...windowRects(layout.first, a), ...windowRects(layout.second, b)]
}
