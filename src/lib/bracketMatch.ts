/**
 * S 式の移動 (`forwardSexp`, `backwardSexp`). 本文を, 括弧とそれ以外 (記号, 数, 文字列) の字句に分けて数えます.
 */

/** 本文の字句. */
interface Token {
  /** 括弧 (`(` か `)`) か, それ以外 (記号, 数, 文字列). */
  type: 'paren' | 'other'
  /** 字句の文字. */
  value: string
  /** 字句の始まりの位置. */
  start: number
}

/**
 * 本文を, 字句に分けます. 空白は飛ばします.
 *
 * @param text - 本文.
 * @returns 字句 (始まりの順).
 */
function tokenizeBrackets(text: string): Token[] {
  const tokens: Token[] = []
  let i = 0
  while (i < text.length) {
    if (text[i] === '(' || text[i] === ')') {
      tokens.push({ type: 'paren', value: text[i], start: i })
      i++
    } else if (/\s/.test(text[i])) {
      // 空白は, 字句にしない
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
      // 記号や数は, 区切り (空白, 括弧, ") の前まで
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

/**
 * 開き括弧に対応する, 閉じ括弧を探します.
 *
 * @param parens - 括弧の字句 (始まりの順).
 * @param openToken - 開き括弧.
 * @returns 対応する閉じ括弧. 閉じていなければ `null`.
 */
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

/**
 * 閉じ括弧に対応する, 開き括弧を探します.
 *
 * @param parens - 括弧の字句 (始まりの順).
 * @param closeToken - 閉じ括弧.
 * @returns 対応する開き括弧. 無ければ `null`.
 */
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
 * 次の S 式の終わりの位置を返します (Emacs の `forward-sexp`). 空白は飛ばします.
 * 開き括弧なら, 対応する閉じ括弧の後ろ. それ以外なら, 記号, 数, 文字列の終わり.
 *
 * @param text - 本文.
 * @param cursorPos - カーソルの位置.
 * @returns 移動先の位置. 括弧が閉じていないか, 後ろに S 式が無ければ, 本文の終わり.
 */
export function forwardSexp(text: string, cursorPos: number): number {
  const tokens = tokenizeBrackets(text)

  // 前の空白を飛ばす
  let pos = cursorPos
  while (pos < text.length && /\s/.test(text[pos])) pos++
  if (pos >= text.length) return text.length

  // pos は空白でない文字なので, その文字を含む字句 (始まりが pos 以前の, 最後の字句) が必ずある
  const token = tokenAt(tokens, pos)

  // 開き括弧なら, 対応する閉じ括弧の後ろへ
  if (token.type === 'paren' && token.value === '(') {
    const parens = tokens.filter((t) => t.type === 'paren')
    const close = findClose(parens, token)
    return close ? close.start + 1 : text.length
  }

  // それ以外なら, その字句の終わりへ
  return token.start + token.value.length
}

/**
 * 前の S 式の始まりの位置を返します (Emacs の `backward-sexp`). 空白は飛ばします.
 * 閉じ括弧なら, 対応する開き括弧. それ以外なら, 記号, 数, 文字列の始まり.
 *
 * @param text - 本文.
 * @param cursorPos - カーソルの位置.
 * @returns 移動先の位置. 対応する開き括弧が無いか, 前に S 式が無ければ, 本文の始まり.
 */
export function backwardSexp(text: string, cursorPos: number): number {
  const tokens = tokenizeBrackets(text)

  // 後ろ (カーソルの前) の空白を飛ばす
  let pos = cursorPos - 1
  while (pos >= 0 && /\s/.test(text[pos])) pos--
  if (pos < 0) return 0

  // pos は空白でない文字なので, その文字を含む字句が必ずある
  const token = tokenAt(tokens, pos)

  // 閉じ括弧なら, 対応する開き括弧へ
  if (token.type === 'paren' && token.value === ')') {
    const parens = tokens.filter((t) => t.type === 'paren')
    const open = findOpen(parens, token)
    return open ? open.start : 0
  }

  // それ以外なら, その字句の始まりへ
  return token.start
}
