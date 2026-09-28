export type TokenType =
  'paren' | 'keyword' | 'string' | 'comment' | 'number' | 'boolean' | 'symbol' | 'whitespace'

export interface Token {
  type: TokenType
  value: string
  start: number
  end: number
  depth?: number // paren nesting depth, used for rainbow parens / matching
}

const KEYWORDS = new Set([
  'define',
  'lambda',
  'if',
  'cond',
  'case',
  'else',
  'let',
  'let*',
  'letrec',
  'letrec*',
  'let-values',
  'begin',
  'set!',
  'quote',
  'quasiquote',
  'unquote',
  'unquote-splicing',
  'and',
  'or',
  'not',
  'when',
  'unless',
  'do',
  'define-syntax',
  'let-syntax',
  'letrec-syntax',
  'syntax-rules',
  'define-record-type',
  'delay',
  'force',
  'dynamic-wind',
  'call/cc',
  'call-with-current-continuation',
  'values',
  'call-with-values',
])

function isSymbolChar(c: string): boolean {
  return /[^\s()"';]/.test(c)
}

export function tokenize(input: string): Token[] {
  const tokens: Token[] = []
  let i = 0
  let depth = 0
  const n = input.length

  while (i < n) {
    const c = input[i]

    // whitespace
    if (/\s/.test(c)) {
      const start = i
      while (i < n && /\s/.test(input[i])) i++
      tokens.push({ type: 'whitespace', value: input.slice(start, i), start, end: i })
      continue
    }

    // comment
    if (c === ';') {
      const start = i
      while (i < n && input[i] !== '\n') i++
      tokens.push({ type: 'comment', value: input.slice(start, i), start, end: i })
      continue
    }

    // parens
    if (c === '(' || c === ')') {
      const start = i
      if (c === '(') {
        depth++
        tokens.push({ type: 'paren', value: c, start, end: i + 1, depth })
      } else {
        tokens.push({ type: 'paren', value: c, start, end: i + 1, depth })
        depth = Math.max(0, depth - 1)
      }
      i++
      continue
    }

    // string
    if (c === '"') {
      const start = i
      i++
      while (i < n && input[i] !== '"') {
        if (input[i] === '\\') i++
        i++
      }
      i++ // consume closing quote (or reach end)
      tokens.push({ type: 'string', value: input.slice(start, i), start, end: i })
      continue
    }

    // boolean
    if (c === '#' && (input[i + 1] === 't' || input[i + 1] === 'f')) {
      const start = i
      i += 2
      tokens.push({ type: 'boolean', value: input.slice(start, i), start, end: i })
      continue
    }

    // number
    if (/[0-9]/.test(c) || (c === '-' && /[0-9]/.test(input[i + 1] ?? ''))) {
      const start = i
      i++
      while (i < n && /[0-9.eE+-]/.test(input[i])) i++
      tokens.push({ type: 'number', value: input.slice(start, i), start, end: i })
      continue
    }

    // symbol / keyword
    if (isSymbolChar(c)) {
      const start = i
      while (i < n && isSymbolChar(input[i])) i++
      const value = input.slice(start, i)
      tokens.push({
        type: KEYWORDS.has(value) ? 'keyword' : 'symbol',
        value,
        start,
        end: i,
      })
      continue
    }

    // fallback: consume single unknown char
    tokens.push({ type: 'symbol', value: c, start: i, end: i + 1 })
    i++
  }

  return tokens
}
