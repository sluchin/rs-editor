import { describe, it, expect } from 'vitest'
import { findMatchingBracket, findUnmatchedClose } from './bracketMatch'

describe('findMatchingBracket', () => {
  const text = '(define (f x) (+ x 1))'
  // indices:      0123456789012345678901
  //                         1111111111222

  it('matches the outermost pair from just after the opening paren', () => {
    expect(findMatchingBracket(text, 1)).toEqual({ open: 0, close: 21 })
  })

  it('matches the outermost pair from just after the closing paren', () => {
    expect(findMatchingBracket(text, text.length)).toEqual({ open: 0, close: 21 })
  })

  it('matches a nested pair', () => {
    expect(findMatchingBracket(text, 9)).toEqual({ open: 8, close: 12 })
  })

  it('returns null when cursor is not adjacent to a paren', () => {
    expect(findMatchingBracket(text, 3)).toBeNull()
  })

  it('returns null for unbalanced input', () => {
    expect(findMatchingBracket('(+ 1 2', 1)).toBeNull()
  })
})

describe('findUnmatchedClose', () => {
  it('returns null for balanced parens', () => {
    expect(findUnmatchedClose('(+ 1 2)')).toBeNull()
  })

  it('returns null for an unclosed open paren', () => {
    expect(findUnmatchedClose('(+ 1 2')).toBeNull()
  })

  it('finds an extra closing paren', () => {
    const text = '(+ 1 2))'
    expect(findUnmatchedClose(text)).toBe(text.length - 1)
  })
})
