import { describe, it, expect } from 'vitest'
import { forwardSexp, backwardSexp, killSexp } from '../lib/bracketMatch'

describe('Phase 4: Edit Command Enhancements', () => {
  describe('forwardSexp', () => {
    it('should move forward past an opening paren and its matching close', () => {
      const text = '(+ 1 2) (+ 3 4)'
      expect(forwardSexp(text, 0)).toBe(7)
    })

    it('should move forward past a symbol', () => {
      const text = 'hello world'
      expect(forwardSexp(text, 0)).toBe(5)
    })

    it('should move forward past a number', () => {
      const text = '123 456'
      expect(forwardSexp(text, 0)).toBe(3)
    })

    it('should return text length at end of buffer', () => {
      const text = 'hello'
      expect(forwardSexp(text, 5)).toBe(5)
    })
  })

  describe('backwardSexp', () => {
    it('should move backward to start of opening paren', () => {
      const text = '(+ 1 2) (+ 3 4)'
      expect(backwardSexp(text, 7)).toBe(0)
    })

    it('should move backward past a symbol', () => {
      const text = 'hello world'
      expect(backwardSexp(text, 11)).toBe(6)
    })

    it('should return 0 at start of buffer', () => {
      const text = 'hello'
      expect(backwardSexp(text, 0)).toBe(0)
    })
  })

  describe('killSexp', () => {
    it('should kill forward to end of sexp', () => {
      const text = '(+ 1 2) other'
      const [newText, newPos] = killSexp(text, 0)
      expect(newText).toBe(' other')
      expect(newPos).toBe(0)
    })

    it('should kill a symbol', () => {
      const text = 'hello world'
      const [newText, newPos] = killSexp(text, 0)
      expect(newText).toBe(' world')
      expect(newPos).toBe(0)
    })

    it('should handle empty kill', () => {
      const text = 'hello'
      const [newText, newPos] = killSexp(text, 5)
      expect(newText).toBe(text)
      expect(newPos).toBe(5)
    })
  })
})
