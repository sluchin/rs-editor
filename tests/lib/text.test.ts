import { describe, it, expect } from 'vitest'
import { changeCase, clampLine, wordBackward, wordForward } from '@/lib/text'

describe('wordForward と wordBackward', () => {
  it('区切りを飛ばして, 単語の終わりと始めへ動くこと', () => {
    expect(wordForward('  foo bar', 0)).toBe(5)
    expect(wordForward('foo', 3)).toBe(3)
    expect(wordBackward('foo bar  ', 9)).toBe(4)
    expect(wordBackward('foo', 0)).toBe(0)
  })
})

describe('changeCase', () => {
  it('大文字, 小文字, 単語ごとの先頭の大文字にすること', () => {
    expect(changeCase('Foo bar', 'upcase')).toBe('FOO BAR')
    expect(changeCase('Foo BAR', 'downcase')).toBe('foo bar')
    expect(changeCase('fOO bAR-baz', 'capitalize')).toBe('Foo Bar-Baz')
  })
})

describe('clampLine', () => {
  it('行の番号を, 1 から行の数までに収めること', () => {
    expect(clampLine(3, 5)).toBe(3)
    expect(clampLine(0, 5)).toBe(1)
    expect(clampLine(9, 5)).toBe(5)
    expect(clampLine(2.7, 5)).toBe(2)
    expect(clampLine(Number.NaN, 5)).toBe(1)
  })
})
