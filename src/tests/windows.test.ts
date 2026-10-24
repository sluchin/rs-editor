import { describe, it, expect } from 'vitest'
import { type Layout, deleteWindow, splitWindow, windowIds, windowRects } from '../lib/windows'

describe('windows', () => {
  const one: Layout = { kind: 'window', id: 'a' }

  it('分けて, 順に並べ, 位置と大きさを計算すること', () => {
    const below = splitWindow(one, 'a', 'b', 'below')
    const both = splitWindow(below, 'b', 'c', 'right')
    expect(windowIds(both)).toEqual(['a', 'b', 'c'])
    expect(windowRects(both)).toEqual([
      { id: 'a', left: 0, top: 0, width: 100, height: 50 },
      { id: 'b', left: 0, top: 50, width: 50, height: 50 },
      { id: 'c', left: 50, top: 50, width: 50, height: 50 },
    ])
    // 無いウィンドウは, 分けない
    expect(splitWindow(one, 'z', 'y', 'below')).toBe(one)
  })

  it('消すと, 隣のウィンドウが空いた所を使い, 1 つだけなら消さないこと', () => {
    const layout = splitWindow(splitWindow(one, 'a', 'b', 'below'), 'b', 'c', 'right')
    expect(windowIds(deleteWindow(layout, 'a') as Layout)).toEqual(['b', 'c'])
    expect(windowIds(deleteWindow(layout, 'b') as Layout)).toEqual(['a', 'c'])
    expect(windowIds(deleteWindow(layout, 'c') as Layout)).toEqual(['a', 'b'])
    expect(deleteWindow(layout, 'z')).toEqual(layout)
    expect(deleteWindow(one, 'a')).toBeNull()
  })
})
