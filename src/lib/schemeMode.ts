/**
 * Scheme のシンタックスハイライト (Emacs の scheme-mode の font-lock と同じ色分け).
 *
 * 色を付けるのは, コメント, 文字列, `(` の直後の特別な形 (`define`, `lambda`, `if` など) と,
 * `define` で定義する名前だけです (`car` などの関数や, 数には付けない).
 */

import {
  HighlightStyle,
  StreamLanguage,
  type StreamParser,
  type StringStream,
  syntaxHighlighting,
} from '@codemirror/language'
import { tags } from '@lezer/highlight'

/** `(` の直後で, キーワードの色にする名前 (`define` の仲間は, {@link definitionKind} で見る). */
const KEYWORDS = new Set([
  'and',
  'begin',
  'call-with-current-continuation',
  'call-with-input-file',
  'call-with-output-file',
  'call/cc',
  'case',
  'case-lambda',
  'cond',
  'delay',
  'do',
  'else',
  'for-each',
  'guard',
  'if',
  'lambda',
  'λ',
  'let',
  'let*',
  'let*-values',
  'let-syntax',
  'let-values',
  'letrec',
  'letrec*',
  'letrec-syntax',
  'map',
  'or',
  'parameterize',
  'quasiquote',
  'quote',
  'syntax',
  'syntax-rules',
  'unless',
  'unquote',
  'unquote-splicing',
  'when',
])

/** `define` の仲間で定義する名前の色. */
type DefinitionKind = 'function' | 'variable' | 'type'

/** 字句の種類 (定義する名前). */
const DEFINITION_TOKEN: Record<DefinitionKind, string> = {
  function: 'definedFunction',
  variable: 'definedVariable',
  type: 'definedType',
}

/**
 * `define` の仲間なら, 定義する名前の色を返します. Emacs と同じく, `define` と `define-public` などは関数,
 * `define-syntax` と `define-macro` は変数, それ以外の `define-xxx` (`define-command` など) は型の色です.
 *
 * @param word - 名前.
 * @returns 定義する名前の色. `define` の仲間でなければ `null`.
 */
export function definitionKind(word: string): DefinitionKind | null {
  const m = /^define\*?(?:(|-public|-method|-generic(?:-procedure)?)|(-syntax|-macro)|-\S+)$/.exec(
    word,
  )
  if (!m) return null
  return m[1] !== undefined ? 'function' : m[2] ? 'variable' : 'type'
}

/** 行をまたいで覚える状態. */
export interface SchemeState {
  /** 文字列の途中なら `true`. */
  inString: boolean
  /** `#| |#` のコメントの深さ (入れ子にできる). */
  commentDepth: number
  /** 直前の字句が `(` なら `true`. */
  afterParen: boolean
  /** `define` の仲間の後で, 定義する名前を待っているなら, その色. */
  defining: DefinitionKind | null
}

/** 名前 (シンボル, 数, `#t` など) の文字. */
const WORD = /[^\s()[\]";'`,]/

/**
 * 文字列を, 閉じる `"` か, 行の終わりまで読みます.
 *
 * @param stream - 行.
 * @param state - 状態.
 * @returns 字句の種類.
 */
function readString(stream: StringStream, state: SchemeState): string {
  let escaped = false
  for (let ch = stream.next(); ch !== undefined; ch = stream.next()) {
    if (ch === '"' && !escaped) {
      state.inString = false
      break
    }
    escaped = !escaped && ch === '\\'
  }
  return 'string'
}

/**
 * `#| |#` のコメントを, 閉じるところか, 行の終わりまで読みます.
 *
 * @param stream - 行.
 * @param state - 状態.
 * @returns 字句の種類.
 */
function readBlockComment(stream: StringStream, state: SchemeState): string {
  while (!stream.eol()) {
    if (stream.match('|#')) {
      state.commentDepth--
      if (state.commentDepth === 0) break
    } else if (stream.match('#|')) {
      state.commentDepth++
    } else {
      stream.next()
    }
  }
  return 'comment'
}

/** Scheme の字句を読みます (CodeMirror 5 の書き方). */
export const schemeParser: StreamParser<SchemeState> = {
  name: 'scheme',
  startState: () => ({ inString: false, commentDepth: 0, afterParen: false, defining: null }),
  token: (stream, state) => {
    if (state.inString) return readString(stream, state)
    if (state.commentDepth > 0) return readBlockComment(stream, state)
    if (stream.eatSpace()) return null
    const { afterParen, defining } = state
    state.afterParen = false
    state.defining = null
    const ch = stream.next() as string
    if (ch === ';') {
      stream.skipToEnd()
      return 'comment'
    }
    if (ch === '"') {
      state.inString = true
      return readString(stream, state)
    }
    if (ch === '#' && stream.eat('|')) {
      state.commentDepth = 1
      return readBlockComment(stream, state)
    }
    if (ch === '(' || ch === '[') {
      state.afterParen = ch === '('
      // `(define (f x) ...)` の `f` も, 定義する名前
      state.defining = defining
      return 'paren'
    }
    if (ch === ')' || ch === ']') return 'paren'
    if (!WORD.test(ch)) return null
    // 文字 (`#\(` など) は, 括弧も名前の文字にする
    if (ch === '#' && stream.eat('\\')) stream.next()
    stream.eatWhile(WORD)
    if (defining) return DEFINITION_TOKEN[defining]
    if (!afterParen) return null
    const kind = definitionKind(stream.current())
    if (kind) {
      state.defining = kind
      return 'keyword'
    }
    return KEYWORDS.has(stream.current()) ? 'keyword' : null
  },
  tokenTable: {
    definedFunction: tags.function(tags.definition(tags.variableName)),
    definedVariable: tags.definition(tags.variableName),
    definedType: tags.typeName,
  },
  languageData: { commentTokens: { line: ';' } },
}

/** Emacs の既定の色 (明るい背景の font-lock の face). */
const emacsHighlightStyle = HighlightStyle.define([
  // font-lock-keyword-face (Purple)
  { tag: tags.keyword, color: '#a020f0' },
  // font-lock-comment-face (Firebrick)
  { tag: tags.comment, color: '#b22222' },
  // font-lock-string-face (VioletRed4)
  { tag: tags.string, color: '#8b2252' },
  // font-lock-function-name-face (Blue1)
  { tag: tags.function(tags.definition(tags.variableName)), color: '#0000ff' },
  // font-lock-variable-name-face (sienna)
  { tag: tags.definition(tags.variableName), color: '#a0522d' },
  // font-lock-type-face (ForestGreen)
  { tag: tags.typeName, color: '#228b22' },
])

/** Scheme のシンタックスハイライトの拡張. */
export const schemeHighlighting = [
  StreamLanguage.define(schemeParser),
  syntaxHighlighting(emacsHighlightStyle),
]
