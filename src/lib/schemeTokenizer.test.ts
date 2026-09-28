import { describe, it, expect } from 'vitest'
import { tokenize } from './schemeTokenizer'

describe('tokenize', () => {
  it('tokenizes parens and symbols', () => {
    const tokens = tokenize('(+ 1 2)')
    expect(tokens.map((t) => t.type)).toEqual([
      'paren',
      'symbol',
      'whitespace',
      'number',
      'whitespace',
      'number',
      'paren',
    ])
  })

  it('recognizes keywords', () => {
    const tokens = tokenize('(define x 1)')
    const defineToken = tokens.find((t) => t.value === 'define')
    expect(defineToken?.type).toBe('keyword')
  })

  it('does not misclassify keyword-like symbols', () => {
    const tokens = tokenize('define-my-thing')
    expect(tokens[0].type).toBe('symbol')
  })

  it('tokenizes strings including escaped quotes', () => {
    const tokens = tokenize('"hello \\"world\\""')
    expect(tokens).toHaveLength(1)
    expect(tokens[0].type).toBe('string')
    expect(tokens[0].value).toBe('"hello \\"world\\""')
  })

  it('tokenizes comments to end of line', () => {
    const tokens = tokenize('; a comment\n(+ 1 1)')
    expect(tokens[0].type).toBe('comment')
    expect(tokens[0].value).toBe('; a comment')
  })

  it('tokenizes booleans', () => {
    const tokens = tokenize('#t #f')
    expect(tokens[0]).toMatchObject({ type: 'boolean', value: '#t' })
    expect(tokens[2]).toMatchObject({ type: 'boolean', value: '#f' })
  })

  it('tokenizes negative numbers', () => {
    const tokens = tokenize('-42')
    expect(tokens[0]).toMatchObject({ type: 'number', value: '-42' })
  })

  it('does not treat a lone minus symbol as a number', () => {
    const tokens = tokenize('(- 1 2)')
    expect(tokens[1]).toMatchObject({ type: 'symbol', value: '-' })
  })

  it('tracks paren nesting depth', () => {
    const tokens = tokenize('(a (b) c)')
    const parens = tokens.filter((t) => t.type === 'paren')
    expect(parens.map((p) => p.depth)).toEqual([1, 2, 2, 1])
  })

  it('round-trips token values back to the original text', () => {
    const input = '(define (f x) (+ x 1)) ; comment\n"str"'
    const tokens = tokenize(input)
    expect(tokens.map((t) => t.value).join('')).toBe(input)
  })
})
