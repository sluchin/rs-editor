import { describe, it, expect, vi } from 'vitest'
import { complete, completeFileName } from '@/lib/complete'

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
