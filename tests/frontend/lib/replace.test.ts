import { describe, it, expect } from 'vitest'
import {
  expandReplacement,
  findReplaceMatch,
  replacedMessage,
  replaceRemaining,
} from '@/lib/replace'

/**
 * 置き換える本文 (文字列を持つ) を作ります.
 *
 * @param text - 始めの本文.
 * @returns 置き換える本文.
 */
function target(text: string) {
  const state = { text }
  return {
    state,
    text: () => state.text,
    replace: (from: number, to: number, insert: string) => {
      state.text = state.text.slice(0, from) + insert + state.text.slice(to)
    },
  }
}

describe('replace', () => {
  it('\\&, \\1, \\\\ を展開すること', () => {
    expect(expandReplacement('<\\&>[\\1|\\2]\\\\', ['ab', 'a', undefined])).toBe('<ab>[a|]\\')
  })

  it('文字で探して, 置き換える所を返すこと', () => {
    const spec = { from: 'ab', to: 'X', regexp: false }
    expect(findReplaceMatch('ab ab', spec, 1)).toEqual({ start: 3, end: 5, replacement: 'X' })
    expect(findReplaceMatch('ab', spec, 1)).toBeNull()
    expect(findReplaceMatch('ab', { ...spec, from: '' }, 0)).toBeNull()
  })

  it('正規表現で探し, 空の所を飛ばし, 誤った正規表現は見つけないこと', () => {
    const spec = { from: '([a-z])([0-9]*)', to: '\\2\\1', regexp: true }
    expect(findReplaceMatch('-- a12', spec, 0)).toEqual({ start: 3, end: 6, replacement: '12a' })
    expect(findReplaceMatch('ab', { from: 'x*', to: '', regexp: true }, 0)).toBeNull()
    expect(findReplaceMatch('ab', { from: 'x*b', to: '', regexp: true }, 0)).toEqual({
      start: 1,
      end: 2,
      replacement: '',
    })
    expect(findReplaceMatch('ab', { from: '(', to: '', regexp: true }, 0)).toBeNull()
  })

  it('位置から後ろを全て置き換え, 数と位置を返すこと', () => {
    const t = target('a a a')
    expect(replaceRemaining(t, { from: 'a', to: 'aa', regexp: false }, 1)).toEqual({
      count: 2,
      pos: 7,
    })
    expect(t.state.text).toBe('a aa aa')
    expect(replaceRemaining(t, { from: 'z', to: '', regexp: false }, 3)).toEqual({
      count: 0,
      pos: 3,
    })
  })

  it('置き換えた数のメッセージを作ること', () => {
    expect(replacedMessage(1)).toBe('Replaced 1 occurrence')
    expect(replacedMessage(2)).toBe('Replaced 2 occurrences')
  })
})
