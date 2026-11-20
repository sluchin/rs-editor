import { describe, it, expect, vi } from 'vitest'
import { complete, completeFileName, partialMatch } from '@/lib/complete'

describe('complete', () => {
  it('前方一致する候補の, 共通する先頭の部分まで補完すること', () => {
    expect(complete('find', ['find-file', 'find-file-other', 'forward-char'])).toEqual({
      value: 'find-file',
      matches: ['find-file', 'find-file-other'],
    })
    expect(complete('fo', ['forward-char', 'forward-char'])).toEqual({
      value: 'forward-char',
      matches: ['forward-char'],
    })
    expect(complete('k', ['kill-line', 'kill-word'])).toEqual({
      value: 'kill-',
      matches: ['kill-line', 'kill-word'],
    })
    expect(complete('zz', ['a'])).toEqual({ value: 'zz', matches: [] })
  })
})

describe('completeFileName', () => {
  it('最後の / の後ろを, そのディレクトリの中の名前で補完すること', async () => {
    const list = vi.fn(async (dir: string) =>
      dir === '/home/u/' ? ['.emacs', 'src/', 'scheme.scm', 'scratch/'] : [],
    )
    expect(await completeFileName('/home/u/sc', list)).toEqual({
      value: '/home/u/sc',
      matches: ['scheme.scm', 'scratch/'],
    })
    expect(list).toHaveBeenCalledWith('/home/u/')
    // ディレクトリは, / まで補完する
    expect(await completeFileName('/home/u/sr', list)).toEqual({
      value: '/home/u/src/',
      matches: ['src/'],
    })
    expect(await completeFileName('/home/u/', list)).toEqual({
      value: '/home/u/',
      matches: ['.emacs', 'scheme.scm', 'scratch/', 'src/'],
    })
  })

  it('ディレクトリを読めなければ, 候補なしにすること', async () => {
    const list = vi.fn().mockRejectedValue('No such file or directory')
    expect(await completeFileName('/none/a', list)).toEqual({ value: '/none/a', matches: [] })
  })
})

describe('partial-completion', () => {
  it('区切りの文字で分けた各部分が, 同じ位置の単語の前方なら当たること', () => {
    expect(partialMatch('f-f', 'find-file')).toBe(true)
    expect(partialMatch('fi-f', 'find-file')).toBe(true)
    expect(partialMatch('f-f', 'find-file-other')).toBe(false)
    expect(partialMatch('g-f', 'find-file')).toBe(false)
    expect(partialMatch('f_f', 'find-file')).toBe(false)
  })

  it('前方一致が無いとき, 1 つに決まれば補完し, 複数なら入力のまま一覧に出すこと', () => {
    const names = ['find-file', 'find-alternate-file', 'forward-char', 'file-name-case']
    expect(complete('f-f', names)).toEqual({ value: 'find-file', matches: ['find-file'] })
    expect(complete('f-c', names)).toEqual({ value: 'forward-char', matches: ['forward-char'] })
    expect(complete('f-n-x', names)).toEqual({ value: 'f-n-x', matches: [] })
    const several = ['find-file', 'fill-file']
    expect(complete('f-f', several)).toEqual({ value: 'f-f', matches: ['fill-file', 'find-file'] })
    // 入力が空なら, 全て前方一致
    expect(complete('', ['a-b']).matches).toEqual(['a-b'])
    // 候補が無く, 入力も空なら, 何もしない
    expect(complete('', [])).toEqual({ value: '', matches: [] })
  })
})
