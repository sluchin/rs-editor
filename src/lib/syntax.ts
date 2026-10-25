/**
 * 構文表と, 式の走査 (Emacs 30 の syntax.c の `scan-lists`, `parse-partial-sexp`, `forward-comment`).
 * エディタ (CodeMirror) に依存しないので, 単体でテストできます.
 *
 * 構文表は, Emacs 30 の Fundamental モード (`standard-syntax-table`), emacs-lisp-mode, scheme-mode のものです.
 * Lisp のモードでは, `;` から行の終わりまでがコメントで, Scheme では `#| |#` もコメントです (入れ子にできる).
 */

/** Lisp の方言 (`null` なら, Lisp でない Fundamental モード). */
export type Dialect = 'scheme' | 'elisp' | null

/**
 * 構文の種類 (Emacs の syntax class). 空白 (` `), 句読点 (`.`), 単語 (`w`), 記号 (`_`), 開く括弧 (`(`),
 * 閉じる括弧 (`)`), 文字列 (`"`), エスケープ (`\`), 式の前の印 (`'`), コメントの始め (`<`) と終わり (`>`).
 */
type SyntaxClass = ' ' | '.' | 'w' | '_' | '(' | ')' | '"' | '\\' | "'" | '<' | '>'

/** 文字の構文. */
interface Syntax {
  /** 種類. */
  cls: SyntaxClass
  /**
   * 2 文字のコメントの印 (Emacs の flag). `1` と `2` は, 始めの 1 文字目と 2 文字目, `3` と `4` は,
   * 終わりの 1 文字目と 2 文字目 (Scheme の `#|` と `|#`).
   */
  flags: string
}

/** Fundamental モードの, ASCII の文字の構文 (書いていない文字は句読点). */
const FUNDAMENTAL: Record<string, SyntaxClass> = {
  ...Object.fromEntries([...'\t\n\f\r '].map((c) => [c, ' '])),
  ...Object.fromEntries([...'$%'].map((c) => [c, 'w'])),
  ...Object.fromEntries([...'&*+-/<=>_|'].map((c) => [c, '_'])),
  '"': '"',
  '\\': '\\',
  '(': '(',
  '[': '(',
  '{': '(',
  ')': ')',
  ']': ')',
  '}': ')',
}

/** emacs-lisp-mode の, ASCII の文字の構文 (書いていない文字は記号). */
const ELISP: Record<string, SyntaxClass> = {
  ...Object.fromEntries([...'\t\f '].map((c) => [c, ' '])),
  ...Object.fromEntries([..."#',`"].map((c) => [c, "'"])),
  '\n': '>',
  ';': '<',
  '"': '"',
  '\\': '\\',
  '(': '(',
  '[': '(',
  ')': ')',
  ']': ')',
}

/** scheme-mode の, ASCII の文字の構文 (書いていない文字は記号). */
const SCHEME: Record<string, SyntaxClass> = {
  ...Object.fromEntries([...'\t\f\r '].map((c) => [c, ' '])),
  ...Object.fromEntries([..."#',@`"].map((c) => [c, "'"])),
  '\n': '>',
  ';': '<',
  '"': '"',
  '|': '"',
  '\\': '\\',
  '(': '(',
  '[': '(',
  '{': '(',
  ')': ')',
  ']': ')',
  '}': ')',
}

/** ASCII でない括弧 (開く括弧と閉じる括弧を, 交互に並べる). */
const WIDE_PARENS = '「」『』（）【】〔〕〈〉《》［］｛｝〖〗〘〙〚〛｟｠⦅⦆⟨⟩'

/** 空白の, ASCII でない文字 (Emacs の `standard-syntax-table`). */
const WIDE_SPACES = ' 　'

/**
 * 文字の構文を返します.
 *
 * @param ch - 文字 (UTF-16 の 1 単位. サロゲートは単語の文字とする).
 * @param dialect - Lisp の方言.
 * @returns 構文.
 */
export function syntaxOf(ch: string, dialect: Dialect): Syntax {
  if (ch < '\u0080') {
    if (dialect === 'scheme' && (ch === '#' || ch === '|')) {
      return { cls: SCHEME[ch], flags: ch === '#' ? '14' : '23' }
    }
    if (/[0-9A-Za-z]/.test(ch)) return { cls: 'w', flags: '' }
    const table = dialect === 'scheme' ? SCHEME : dialect === 'elisp' ? ELISP : FUNDAMENTAL
    return { cls: table[ch] ?? (dialect ? '_' : '.'), flags: '' }
  }
  const index = WIDE_PARENS.indexOf(ch)
  if (index >= 0) return { cls: index % 2 === 0 ? '(' : ')', flags: '' }
  if (WIDE_SPACES.includes(ch)) return { cls: ' ', flags: '' }
  if (/\p{P}/u.test(ch)) return { cls: '.', flags: '' }
  if (/\p{S}/u.test(ch) && !/\p{Emoji_Presentation}/u.test(ch)) return { cls: '_', flags: '' }
  return { cls: 'w', flags: '' }
}

/** 式の走査の失敗 (Emacs の `scan-error`). */
export class ScanError extends Error {
  /**
   * 失敗を作ります.
   *
   * @param message - 理由 (`Unbalanced parentheses` など).
   * @param from - 最後に読めた所.
   * @param to - 失敗した所.
   */
  constructor(
    message: string,
    readonly from: number,
    readonly to: number,
  ) {
    super(message)
  }

  /** Emacs の表示 (`Scan error: "Unbalanced parentheses", 4, 11`. 位置は 1 から数える). */
  get display(): string {
    return `Scan error: "${this.message}", ${this.from + 1}, ${this.to + 1}`
  }
}

/** コマンドの失敗 (Emacs の `user-error` など). 理由は, ミニバッファに出します. */
export class UserError extends Error {}

/** 本文と, 読む範囲 (Emacs のナローイング), 方言. */
export interface Source {
  /** 本文. */
  text: string
  /** Lisp の方言. */
  dialect: Dialect
  /** 範囲の始め (省くと 0). */
  min?: number
  /** 範囲の終わり (省くと本文の終わり). */
  max?: number
}

/** 位置までの構文の状態 (Emacs の `parse-partial-sexp` の結果). */
export interface ParseState {
  /** 括弧の深さ. */
  depth: number
  /** 開いている括弧の位置 (外から順). */
  opens: number[]
  /** 文字列の中なら `true`. */
  inString: boolean
  /** コメントの中なら `true`. */
  inComment: boolean
  /** コメントが, 入れ子にできるコメント (Scheme の `#| |#`) なら `true`. */
  nested: boolean
  /** 文字列かコメントの始め (どちらの中でもなければ `null`. Emacs の 8 番目の要素). */
  start: number | null
}

/**
 * 範囲を返します.
 *
 * @param source - 本文.
 * @returns 始めと終わり.
 */
function bounds(source: Source): [number, number] {
  return [source.min ?? 0, source.max ?? source.text.length]
}

/**
 * 位置の文字の構文を返します.
 *
 * @param source - 本文.
 * @param pos - 位置.
 * @returns 構文.
 */
function syntaxAt(source: Source, pos: number): Syntax {
  const { text, dialect } = source
  if (dialect === 'elisp') {
    // `,@` の `@` は, 後ろの記号に付けない. `##` は, 空の名前の記号 (Emacs の `elisp-mode-syntax-propertize`)
    if (text[pos] === '@' && text[pos - 1] === ',') return { cls: "'", flags: '' }
    if (text[pos] === '#' && (text[pos + 1] === '#' || text[pos - 1] === '#')) {
      return { cls: '_', flags: '' }
    }
  }
  return syntaxOf(text[pos], dialect)
}

/**
 * 位置の 2 文字が, 構文の印 `first` と `second` を持てば `true` (`#|` など).
 *
 * @param source - 本文.
 * @param pos - 1 文字目の位置.
 * @param first - 1 文字目の印.
 * @param second - 2 文字目の印.
 * @param max - 範囲の終わり.
 * @returns 持てば `true`.
 */
function pair(source: Source, pos: number, first: string, second: string, max: number): boolean {
  return (
    pos + 1 < max &&
    syntaxAt(source, pos).flags.includes(first) &&
    syntaxAt(source, pos + 1).flags.includes(second)
  )
}

/**
 * 範囲の始めから位置までを読んで, 構文の状態を返します (Emacs の `syntax-ppss`).
 *
 * @param source - 本文.
 * @param pos - 位置.
 * @returns 状態.
 */
export function parseState(source: Source, pos: number): ParseState {
  const [min] = bounds(source)
  const state: ParseState = {
    depth: 0,
    opens: [],
    inString: false,
    inComment: false,
    nested: false,
    start: null,
  }
  // 文字列の終わりの文字, コメントの入れ子の深さ (`;` のコメントは 0)
  let term = ''
  let nesting = 0
  for (let i = min; i < pos; i++) {
    const { cls } = syntaxAt(source, i)
    const ch = source.text[i]
    if (state.inString) {
      if (ch === term) state.inString = false
      else if (cls === '\\') i++
    } else if (state.inComment) {
      if (nesting === 0) {
        if (cls === '>') state.inComment = false
      } else if (pair(source, i, '3', '4', pos)) {
        i++
        if (--nesting === 0) state.inComment = false
      } else if (pair(source, i, '1', '2', pos)) {
        i++
        nesting++
      }
    } else if (pair(source, i, '1', '2', pos)) {
      Object.assign(state, { inComment: true, nested: true, start: i })
      nesting = 1
      i++
    } else if (cls === '<') {
      Object.assign(state, { inComment: true, nested: false, start: i })
      nesting = 0
    } else if (cls === '"') {
      Object.assign(state, { inString: true, start: i })
      term = ch
    } else if (cls === '\\') {
      i++
    } else if (cls === '(') {
      state.opens.push(i)
      state.depth++
    } else if (cls === ')') {
      state.opens.pop()
      state.depth--
    }
    if (!state.inString && !state.inComment) state.start = null
  }
  return state
}

/**
 * コメントの終わりまで読みます (Emacs の `forw_comment`).
 *
 * @param source - 本文.
 * @param from - コメントの始めの印の後ろ.
 * @param nesting - 入れ子の深さ (`;` のコメントは 0, `#|` のコメントは 1).
 * @param max - 範囲の終わり.
 * @returns コメントの後ろ. 終わらなければ `null`.
 */
function forwardCommentBody(
  source: Source,
  from: number,
  nesting: number,
  max: number,
): number | null {
  let i = from
  while (i < max) {
    if (nesting === 0) {
      if (syntaxAt(source, i).cls === '>') return i + 1
    } else if (pair(source, i, '3', '4', max)) {
      i++
      if (--nesting === 0) return i + 1
    } else if (pair(source, i, '1', '2', max)) {
      i++
      nesting++
    }
    i++
  }
  return null
}

/**
 * 位置の文字が, エスケープ (`\`) の後なら `true` (Emacs の `char_quoted`).
 *
 * @param source - 本文.
 * @param pos - 位置.
 * @returns エスケープの後なら `true`.
 */
function charQuoted(source: Source, pos: number): boolean {
  const [min] = bounds(source)
  let count = 0
  while (pos - 1 - count >= min && syntaxAt(source, pos - 1 - count).cls === '\\') count++
  return count % 2 === 1
}

/**
 * 終わりの印 (`;` のコメントの改行, `|#`) から, コメントの始めを探します (Emacs の `back_comment`).
 *
 * @param source - 本文.
 * @param from - 終わりの印の (1 文字目の) 位置.
 * @param nested - 終わりの印が `|#` なら `true`.
 * @returns コメントの始め. その種類のコメントの終わりでなければ (`#| |#` の中の改行など) `null`.
 */
function backComment(source: Source, from: number, nested: boolean): number | null {
  const state = parseState(source, from)
  return state.inComment && state.nested === nested ? state.start : null
}

/**
 * 括弧の深さを数えながら, 式を読みます (Emacs の `scan-lists` と `scan-sexps`).
 *
 * @param source - 本文.
 * @param from - 読み始める位置.
 * @param count - 読む数 (負なら, 前へ).
 * @param depth - 始めの深さ (`1` で外のリストの終わりまで, `-1` で中のリストの始めまで).
 * @param sexp - 記号や文字列も, 式として数えるなら `true` (`scan-sexps`).
 * @returns 読んだ後の位置. 範囲の端まで式が無ければ `null`.
 * @throws {ScanError} 括弧が合わないとき.
 */
export function scanLists(
  source: Source,
  from: number,
  count: number,
  depth: number,
  sexp: boolean,
): number | null {
  const [min, max] = bounds(source)
  const ignoreComments = source.dialect !== null
  const minDepth = depth > 0 ? 0 : depth
  let lastGood = from
  const lose = () => new ScanError('Unbalanced parentheses', lastGood, from)
  const premature = () => new ScanError('Containing expression ends prematurely', lastGood, from)
  /** 記号や単語の続きの文字なら `true`. */
  const symbolChar = (cls: SyntaxClass) => cls === 'w' || cls === '_' || cls === "'"

  while (count > 0) {
    let done = false
    while (from < max && !done) {
      let { cls } = syntaxAt(source, from)
      if (depth === minDepth) lastGood = from
      // `#|` のコメント
      let nesting = 0
      if (ignoreComments && pair(source, from, '1', '2', max)) {
        cls = '<'
        nesting = 1
        from++
      }
      from++
      switch (cls) {
        case '\\':
          if (from === max) throw lose()
          from++
        // falls through
        case 'w':
        case '_':
          if (depth || !sexp) break
          while (from < max) {
            const next = syntaxAt(source, from).cls
            if (next === '\\') {
              from++
              if (from === max) throw lose()
            } else if (!symbolChar(next)) {
              break
            }
            from++
          }
          done = true
          break
        case '<': {
          const end = forwardCommentBody(source, from, nesting, max)
          from = end ?? max
          if (end === null) {
            if (depth === 0) done = true
            else throw lose()
          }
          break
        }
        case '(':
          if (!++depth) done = true
          break
        case ')':
          if (!--depth) done = true
          else if (depth < minDepth) throw premature()
          break
        case '"': {
          const term = source.text[from - 1]
          for (;;) {
            if (from >= max) throw lose()
            const ch = source.text[from]
            if (ch === term) break
            if (syntaxAt(source, from).cls === '\\') from++
            from++
          }
          from++
          if (!depth && sexp) done = true
          break
        }
      }
    }
    if (!done) {
      if (depth) throw lose()
      return null
    }
    count--
  }

  while (count < 0) {
    let done = false
    while (from > min && !done) {
      from--
      let { cls } = syntaxAt(source, from)
      if (depth === minDepth) lastGood = from
      // `|#` のコメントの終わり
      const nested = ignoreComments && from > min && pair(source, from - 1, '3', '4', max)
      if (nested) {
        cls = '>'
        from--
      }
      if (cls !== '>' && charQuoted(source, from)) {
        from--
        cls = 'w'
      }
      switch (cls) {
        case 'w':
        case '_':
        case '\\':
          if (depth || !sexp) break
          while (from > min) {
            const prev = syntaxAt(source, from - 1).cls
            if (prev === '>') break
            const quoted = charQuoted(source, from - 1)
            if (quoted) from--
            else if (!symbolChar(syntaxAt(source, from - 1).cls)) break
            from--
          }
          done = true
          break
        case ')':
          // 前へ読むときは, 深さは負から始まらないので, 閉じる括弧で式は終わらない
          depth++
          break
        case '(':
          if (!--depth) done = true
          else if (depth < minDepth) throw premature()
          break
        case '>': {
          const start = backComment(source, from, nested)
          if (start !== null) from = start
          break
        }
        case '"': {
          const term = source.text[from]
          for (;;) {
            if (from === min) throw lose()
            from--
            if (!charQuoted(source, from) && source.text[from] === term) break
          }
          if (!depth && sexp) done = true
          break
        }
      }
    }
    if (!done) {
      if (depth) throw lose()
      return null
    }
    count++
  }
  return from
}

/**
 * 空白とコメントを飛ばします (Emacs の `forward-comment` の前向き. 改行も空白とする).
 *
 * @param source - 本文.
 * @param from - 位置.
 * @param count - 飛ばすコメントの数 (`Infinity` なら, 全て).
 * @returns 飛ばした後の位置.
 */
export function forwardComment(source: Source, from: number, count: number): number {
  const [, max] = bounds(source)
  let pos = from
  for (let n = 0; n < count; n++) {
    while (pos < max && /[ >]/.test(syntaxAt(source, pos).cls)) pos++
    const nesting = pair(source, pos, '1', '2', max) ? 1 : 0
    if (nesting === 0 && !isCommentStart(source, pos)) return pos
    pos = forwardCommentBody(source, pos + 1 + nesting, nesting, max) ?? max
  }
  return pos
}

/**
 * 前の, 式の前の印 (`'` など) を飛ばします (Emacs の `backward-prefix-chars`).
 *
 * @param source - 本文.
 * @param pos - 位置.
 * @returns 飛ばした後の位置.
 */
export function backwardPrefixChars(source: Source, pos: number): number {
  const [min] = bounds(source)
  while (pos > min && syntaxAt(source, pos - 1).cls === "'" && !charQuoted(source, pos - 1)) pos--
  return pos
}

/**
 * 位置の文字が, 空白の構文なら `true`.
 *
 * @param source - 本文.
 * @param pos - 位置.
 * @returns 空白なら `true`.
 */
export function isWhitespace(source: Source, pos: number): boolean {
  return pos < source.text.length && syntaxAt(source, pos).cls === ' '
}

/**
 * 位置の文字が, 開く括弧の構文なら `true`.
 *
 * @param source - 本文.
 * @param pos - 位置.
 * @returns 開く括弧なら `true`.
 */
export function isOpenParen(source: Source, pos: number): boolean {
  return pos < source.text.length && syntaxAt(source, pos).cls === '('
}

/**
 * 位置の文字が, コメントの始めの構文なら `true` (Emacs の `\s<`).
 *
 * @param source - 本文.
 * @param pos - 位置.
 * @returns コメントの始めなら `true`.
 */
export function isCommentStart(source: Source, pos: number): boolean {
  return pos < source.text.length && syntaxAt(source, pos).cls === '<'
}
