import { describe, it, expect } from 'vitest'
import { findNextMatch } from './isearch'

describe('isearch utilities', () => {
  describe('findNextMatch', () => {
    it('finds forward match from given position', () => {
      const text = 'hello world hello'
      const result = findNextMatch(text, 'hello', 0, 'forward')
      expect(result).toEqual({ start: 0, end: 5 })
    })

    it('finds next forward match after overlapping results', () => {
      const text = 'aaaa'
      const result = findNextMatch(text, 'aa', 1, 'forward')
      expect(result).toEqual({ start: 1, end: 3 })
    })

    it('returns null when forward match not found', () => {
      const text = 'hello world'
      const result = findNextMatch(text, 'xyz', 0, 'forward')
      expect(result).toBeNull()
    })

    it('finds backward match from given position', () => {
      const text = 'hello world hello'
      const result = findNextMatch(text, 'hello', 17, 'backward')
      expect(result).toEqual({ start: 12, end: 17 })
    })

    it('finds previous backward match from overlapping results', () => {
      const text = 'aaaa'
      const result = findNextMatch(text, 'aa', 3, 'backward')
      expect(result).toEqual({ start: 2, end: 4 })
    })

    it('returns null when backward match not found', () => {
      const text = 'hello world'
      const result = findNextMatch(text, 'xyz', 11, 'backward')
      expect(result).toBeNull()
    })

    it('finds backward match at position 0', () => {
      const text = 'foo'
      const result = findNextMatch(text, 'foo', 0, 'backward')
      expect(result).toEqual({ start: 0, end: 3 })
    })

    it('clamps negative fromPos to 0 in backward search', () => {
      const text = 'foo'
      const result = findNextMatch(text, 'foo', -1, 'backward')
      expect(result).toEqual({ start: 0, end: 3 })
    })

    it('returns null for empty query', () => {
      const text = 'hello'
      const resultForward = findNextMatch(text, '', 0, 'forward')
      const resultBackward = findNextMatch(text, '', 5, 'backward')
      expect(resultForward).toBeNull()
      expect(resultBackward).toBeNull()
    })

    it('finds match longer than remaining text', () => {
      const text = 'hello world'
      const result = findNextMatch(text, 'world', 6, 'forward')
      expect(result).toEqual({ start: 6, end: 11 })
    })
  })
})
