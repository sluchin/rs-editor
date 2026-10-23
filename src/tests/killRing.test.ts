import { describe, it, expect } from 'vitest'
import { KILL_RING_MAX, KillRing } from '../lib/killRing'

describe('KillRing', () => {
  it('新しいものから取り出し, rotate で古い方へ進み, 最後の次は最新に戻ること', () => {
    const ring = new KillRing()
    ring.push('a')
    ring.push('b')
    expect(ring.current()).toBe('b')
    expect(ring.rotate()).toBe('a')
    expect(ring.rotate()).toBe('b')
    ring.rotate()
    ring.reset()
    expect(ring.current()).toBe('b')
  })

  it('続けてキルしたときは, 先頭の項目の後ろか前につなげること', () => {
    const ring = new KillRing()
    ring.push('b')
    ring.push('c', 'append')
    ring.push('a', 'prepend')
    expect(ring.current()).toBe('abc')
    expect(ring.rotate()).toBe('abc')
  })

  it('空のときは, 何も取り出さず, つなげる指定でも新しい項目にすること', () => {
    const ring = new KillRing()
    expect(ring.current()).toBeUndefined()
    expect(ring.rotate()).toBeUndefined()
    ring.push('x', 'append')
    expect(ring.current()).toBe('x')
  })

  it('覚えておく数を超えたら, 古い方から捨てること', () => {
    const ring = new KillRing()
    for (let i = 0; i <= KILL_RING_MAX; i++) ring.push(String(i))
    for (let i = 0; i < KILL_RING_MAX - 1; i++) ring.rotate()
    expect(ring.rotate()).toBe(String(KILL_RING_MAX))
  })
})
