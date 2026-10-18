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
    } else if (text[i] === '"') {
      // 文字列のリテラルは, 閉じる " まで (\" は飛ばす) を 1 つの字句にする.
      // 閉じていなければ, 最後まで
      const start = i
      i++
      while (i < text.length && text[i] !== '"') i += text[i] === '\\' ? 2 : 1
      i = Math.min(i + 1, text.length)
      tokens.push({ type: 'other', value: text.slice(start, i), start })
    } else {
      // Read symbol/number until delimiter
      const start = i
      while (i < text.length && !/[\s()"]/.test(text[i])) i++
      tokens.push({ type: 'other', value: text.slice(start, i), start })
    }
  }
  return tokens
}

/**
 * 位置の文字を含む字句を返します (始まりが位置以前の, 最後の字句).
 * 位置は, 空白でない文字を指していること (そのとき, 字句は必ずある).
 *
 * @param tokens - 字句 (始まりの順).
 * @param pos - 位置.
 * @returns 字句.
 */
function tokenAt(tokens: Token[], pos: number): Token {
  return tokens.reduce((found, t) => (t.start <= pos ? t : found), tokens[0])
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

  // pos は空白でない文字なので, その文字を含む字句 (始まりが pos 以前の, 最後の字句) が必ずある
  const token = tokenAt(tokens, pos)

  // If on opening paren, find matching close and move past it
  if (token.type === 'paren' && token.value === '(') {
    const parens = tokens.filter((t) => t.type === 'paren')
    const close = findClose(parens, token)
    return close ? close.start + 1 : text.length
  }

  // Otherwise move to the end of current token
  return token.start + token.value.length
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

  // pos は空白でない文字なので, その文字を含む字句が必ずある
  const token = tokenAt(tokens, pos)

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
