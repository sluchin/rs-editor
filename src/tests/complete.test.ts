import { describe, it, expect } from 'vitest'
import { complete } from '../lib/complete'

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
