import { describe, it, expect } from 'vitest'
import { addToHistory } from '@/lib/history'

describe('履歴', () => {
  it('先頭に加え, 空の文字と, 先頭と同じ文字は, 加えないこと', () => {
    expect(addToHistory([], 'a', 3)).toEqual(['a'])
    expect(addToHistory(['a'], 'a', 3)).toEqual(['a'])
    expect(addToHistory(['a'], '', 3)).toEqual(['a'])
    expect(addToHistory(['a', 'b'], 'b', 3)).toEqual(['b', 'a', 'b'])
  })

  it('同じ文字を除くときは, 先頭のほかからも除き, 限りを超えたら古い方を捨てること', () => {
    expect(addToHistory(['a', 'b'], 'b', 3, true)).toEqual(['b', 'a'])
    expect(addToHistory(['a', 'b', 'c'], 'd', 3)).toEqual(['d', 'a', 'b'])
  })
})
