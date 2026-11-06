import { describe, it, expect } from 'vitest'
import { addToHistory, historyText, searchHistory, stepHistory } from '@/lib/history'

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

describe('stepHistory と searchHistory', () => {
  const history = ['foo3', 'bar2', 'foo1']

  it('stepHistory は履歴の端と既定の値を扱うこと', () => {
    expect(stepHistory(history, [], 0, true)).toEqual({ pos: 1 })
    expect(stepHistory(history, [], 3, true)).toEqual({
      error: 'Beginning of history; no preceding item',
    })
    expect(stepHistory(history, [], 0, false)).toEqual({
      error: 'End of history; no default available',
    })
    expect(stepHistory(history, ['d'], 0, false)).toEqual({ pos: -1 })
    expect(stepHistory(history, ['d'], -1, false)).toEqual({
      error: 'End of defaults; no next item',
    })
  })

  it('searchHistory は向きごとに合う履歴を探し, 無ければ知らせること', () => {
    expect(searchHistory(history, 0, 'foo', true)).toEqual({ pos: 1 })
    expect(searchHistory(history, 1, 'foo', true)).toEqual({ pos: 3 })
    expect(searchHistory(history, 3, 'foo', true)).toEqual({
      error: 'No earlier matching history item',
    })
    expect(searchHistory(history, 3, 'foo3', false)).toEqual({ pos: 1 })
    expect(searchHistory(history, 1, 'foo', false)).toEqual({
      error: 'No later matching history item',
    })
    expect('error' in searchHistory(history, 0, '(', true)).toBe(true)
  })
})

describe('historyText', () => {
  it('位置ごとに, 入力していた文字, 履歴, 既定の値を返すこと', () => {
    expect(historyText(['a', 'b'], ['d'], 'typed', 0)).toBe('typed')
    expect(historyText(['a', 'b'], ['d'], 'typed', 2)).toBe('b')
    expect(historyText(['a', 'b'], ['d'], 'typed', -1)).toBe('d')
  })
})
