import { Token, tokenize } from './schemeTokenizer'

export interface BracketMatch {
  open: number // index in text of the opening paren
  close: number // index in text of the closing paren
}

/**
 * Given cursor position, find the paren pair adjacent to the cursor
 * (Emacs-like: if the char right before or right after the cursor is a paren).
 */
export function findMatchingBracket(text: string, cursorPos: number): BracketMatch | null {
  const tokens = tokenize(text)
  const parens = tokens.filter((t) => t.type === 'paren')

  // Check char immediately after cursor, then char immediately before
  const candidates = [cursorPos, cursorPos - 1]

  for (const pos of candidates) {
    const paren = parens.find((p) => p.start === pos)
    if (!paren) continue

    if (paren.value === '(') {
      const close = findClose(parens, paren)
      if (close) return { open: paren.start, close: close.start }
    } else {
      const open = findOpen(parens, paren)
      if (open) return { open: open.start, close: paren.start }
    }
  }

  return null
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
 * Check if parens in the text are balanced. Returns the index of the
 * first unmatched closing paren, or -1 if none, or null if fully balanced
 * (unclosed opens are fine, they just mean "not finished yet").
 */
export function findUnmatchedClose(text: string): number | null {
  const tokens = tokenize(text).filter((t) => t.type === 'paren')
  let depth = 0
  for (const t of tokens) {
    if (t.value === '(') depth++
    else {
      depth--
      if (depth < 0) return t.start
    }
  }
  return null
}
