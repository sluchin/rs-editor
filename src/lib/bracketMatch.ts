// Tokenize text by extracting parens and symbols
// Used for S-expression navigation (forwardSexp, backwardSexp)
interface Token {
  type: 'paren' | 'other'
  value: string
  start: number
}

function tokenizeBrackets(text: string): Token[] {
  const tokens: Token[] = []
  let i = 0
  while (i < text.length) {
    if (text[i] === '(' || text[i] === ')') {
      tokens.push({ type: 'paren', value: text[i], start: i })
      i++
    } else if (/\s/.test(text[i])) {
      // Skip whitespace
      i++
    } else {
      // Read symbol/string/number until delimiter
      const start = i
      while (i < text.length && !/[\s()"]/.test(text[i])) i++
      tokens.push({ type: 'other', value: text.slice(start, i), start })
    }
  }
  return tokens
}

function findClose(parens: Token[], openToken: Token): Token | null {
  let depth = 0
  const startIdx = parens.indexOf(openToken)
  for (let i = startIdx; i < parens.length; i++) {
    if (parens[i].value === '(') depth++
    else depth--
    if (depth === 0) return parens[i]
  }
  return null
}

function findOpen(parens: Token[], closeToken: Token): Token | null {
  let depth = 0
  const startIdx = parens.indexOf(closeToken)
  for (let i = startIdx; i >= 0; i--) {
    if (parens[i].value === ')') depth++
    else depth--
    if (depth === 0) return parens[i]
  }
  return null
}

/**
 * Move forward to the end of the next S-expression.
 * If cursor is on an opening paren, move to its matching close.
 * Otherwise, move to the end of the next symbol/number/string.
 */
export function forwardSexp(text: string, cursorPos: number): number {
  const tokens = tokenizeBrackets(text)

  // Skip whitespace forward
  let pos = cursorPos
  while (pos < text.length && /\s/.test(text[pos])) pos++
  if (pos >= text.length) return text.length

  // If on opening paren, find matching close and move past it
  const parenToken = tokens.find((t) => t.type === 'paren' && t.start === pos)
  if (parenToken && parenToken.value === '(') {
    const parens = tokens.filter((t) => t.type === 'paren')
    const close = findClose(parens, parenToken)
    return close ? close.start + 1 : text.length
  }

  // Otherwise find the end of current token
  const token = tokens.find((t) => t.start <= pos && pos < t.start + t.value.length)
  if (token) {
    return token.start + token.value.length
  }

  return Math.min(pos + 1, text.length)
}

/**
 * Move backward to the start of the previous S-expression.
 */
export function backwardSexp(text: string, cursorPos: number): number {
  const tokens = tokenizeBrackets(text)

  // Skip whitespace backward
  let pos = cursorPos - 1
  while (pos >= 0 && /\s/.test(text[pos])) pos--
  if (pos < 0) return 0

  // Find token at this position
  const token = tokens.find((t) => t.start <= pos && pos < t.start + t.value.length)
  if (!token) return 0

  // If on closing paren, find matching open and move to it
  if (token.type === 'paren' && token.value === ')') {
    const parens = tokens.filter((t) => t.type === 'paren')
    const open = findOpen(parens, token)
    return open ? open.start : 0
  }

  // Otherwise move to start of current token
  return token.start
}

/**
 * Kill (delete) the next S-expression starting from cursor position.
 * Returns [newText, newCursorPos].
 */
export function killSexp(text: string, cursorPos: number): [string, number] {
  const endPos = forwardSexp(text, cursorPos)
  if (endPos === cursorPos) return [text, cursorPos]
  return [text.slice(0, cursorPos) + text.slice(endPos), cursorPos]
}
