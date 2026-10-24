import { describe, it, expect } from 'vitest'
import { forwardSexp, backwardSexp } from '@/lib/bracketMatch'

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
})

describe('bracketMatch (文字列と, 閉じていない括弧)', () => {
  it('文字列のリテラルを 1 つの S 式として飛ばす (\\" を含む)', () => {
    const text = '"a \\" b" c'
    expect(forwardSexp(text, 0)).toBe(8)
    expect(backwardSexp(text, 8)).toBe(0)
  })

  it('閉じていない文字列は, 最後までを 1 つにする', () => {
    expect(forwardSexp('"abc', 0)).toBe(4)
    expect(forwardSexp('"ab\\', 0)).toBe(4)
  })

  it('対応する括弧がなければ, 端まで動く', () => {
    expect(forwardSexp('(a b', 0)).toBe(4)
    expect(backwardSexp('a b)', 4)).toBe(0)
  })

  it('空白だけなら, 端まで動くこと', () => {
    expect(forwardSexp('a  ', 1)).toBe(3)
    expect(backwardSexp('  a', 2)).toBe(0)
  })
})
