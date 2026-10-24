/**
 * Lisp (Scheme と Emacs Lisp) のシンタックスハイライト. Emacs 30 の scheme-mode と emacs-lisp-mode の
 * font-lock と同じ所に, 同じ色を付けます (Emacs の `scheme-font-lock-keywords-2` と
 * `lisp-el-font-lock-keywords-2` の決まりを, 字句ごとに読み替えたもの).
 *
 * 色を付けるのは, コメント, 文字列, `(` の直後の特別な形 (`define`, `defun`, `if` など), 特別な形で定義する名前,
 * `:key` などの印です (`car` などの関数や, 数には付けない). Emacs Lisp では, 文字列の中の正規表現の括弧,
 * 要らない `\`, 閉じた括弧の後ろに続く引数 (見落としやすい) にも, 色を付けます.
 */

import {
  HighlightStyle,
  StreamLanguage,
  type StreamParser,
  type StringStream,
  syntaxHighlighting,
} from '@codemirror/language'
import { Tag, tags } from '@lezer/highlight'
import { ELISP_KEYWORDS } from './elispKeywords'

/** 特別な形の後の名前の色. */
type NameKind = 'function' | 'variable' | 'type' | 'constant'

/** 特別な形の後で, 色を付ける名前を待つ. */
interface Pending {
  /** 名前の色 (`null` なら, 色を付けない). */
  kind: NameKind | null
  /** `(` の後の名前の色. */
  inParen: NameKind | null
  /** 名前の前に, 飛ばせる `(` の数. */
  parens: number
}

/** `(` の直後の名前の色と, その後で待つ名前. */
interface Head {
  face: 'keyword' | 'warning'
  next: Pending | null
}

/** Lisp の方言ごとの決まり. */
interface Dialect {
  /**
   * `(` の直後の名前を見ます.
   *
   * @param word - 名前.
   * @param funcall - 関数を呼ぶ所 (引用したリストや, 引数のリストでない) なら `true`.
   */
  head: (word: string, funcall: boolean) => Head | null
  /** どこにあっても色を付ける名前 (`:key` など) を見ます. */
  word: (word: string) => string | null
  /**
   * Emacs Lisp なら `true`. 名前を待つあいだに `'` を飛ばし (`(defalias 'f`), 文字列とコメントの中の
   * `` `name' `` などに色を付け, 閉じた括弧の後ろに続く引数を警告します. `false` (Scheme) なら,
   * `#| |#` と `#;` のコメントと, `#\(` の文字を読みます.
   */
  emacsLisp: boolean
}

/** 字句の種類 (特別な形の後の名前). */
const NAME_TOKEN: Record<NameKind, string> = {
  function: 'definedFunction',
  variable: 'definedVariable',
  type: 'typeName',
  constant: 'constant',
}

/**
 * 名前を待ちます.
 *
 * @param kind - 名前の色.
 * @param inParen - `(` の後の名前の色.
 * @param parens - 飛ばせる `(` の数.
 * @returns 待つ名前.
 */
const pending = (kind: NameKind | null, inParen: NameKind | null, parens: number): Pending => ({
  kind,
  inParen,
  parens,
})

/** Scheme で, `(` の直後でキーワードの色にする名前 (Emacs の `scheme-font-lock-keywords-2`). */
const SCHEME_KEYWORDS = new Set([
  'and',
  'begin',
  'call-with-current-continuation',
  'call-with-input-file',
  'call-with-output-file',
  'call-with-port',
  'call/cc',
  'case',
  'case-lambda',
  'cond',
  'cond-expand',
  'define-library',
  'define-record-type',
  'define-values',
  'delay',
  'delay-force',
  'do',
  'else',
  'except',
  'export',
  'for-each',
  'force',
  'guard',
  'if',
  'import',
  'include',
  'include-ci',
  'include-library-declarations',
  'lambda',
  'let',
  'let*',
  'let*-values',
  'let-syntax',
  'let-values',
  'letrec',
  'letrec*',
  'letrec-syntax',
  'map',
  'only',
  'or',
  'parameterize',
  'prefix',
  'receive',
  'rename',
  'syntax',
  'syntax-error',
  'syntax-rules',
  'unless',
  'when',
  'λ',
])

/** Scheme の `define` の仲間 (名前の色を, 何で決めるか). */
const SCHEME_DEFINE =
  /^define\*?(?:(|-public|-method|-generic(?:-procedure)?)|(-syntax|-macro)|-class|-module)$/

/** Scheme の決まり (Emacs の scheme-mode). */
const scheme: Dialect = {
  head: (word) => {
    const m = SCHEME_DEFINE.exec(word)
    if (m) {
      // define と define-public などは関数, define-syntax と define-macro は変数, define-class などは型の色.
      // `(define ((f a) b)` のように, 前の `(` は幾つでも飛ばす
      const kind = m[1] !== undefined ? 'function' : m[2] ? 'variable' : 'type'
      return { face: 'keyword', next: pending(kind, kind, Infinity) }
    }
    // 名前付きの let の名前は, 関数の色
    if (word === 'let') return { face: 'keyword', next: pending('function', null, 0) }
    if (word === 'library') return { face: 'keyword', next: pending('type', 'type', 1) }
    return SCHEME_KEYWORDS.has(word) ? { face: 'keyword', next: null } : null
  },
  word: (word) => {
    if (/^<.+>$/.test(word)) return 'typeName'
    return /^#?:./.test(word) ? 'builtin' : null
  },
  emacsLisp: false,
}

/** Emacs Lisp の定義の形と, 定義する名前の色 (Emacs の `lisp-define-type`). */
const ELISP_DEFINITIONS = new Map<string, NameKind>(
  Object.entries({
    'cl-defgeneric': 'function',
    'cl-defmacro': 'function',
    'cl-defmethod': 'function',
    'cl-defstruct': 'type',
    'cl-defsubst': 'function',
    'cl-deftype': 'type',
    'cl-defun': 'function',
    defadvice: 'function',
    defalias: 'function',
    defclass: 'type',
    defconst: 'variable',
    defcustom: 'variable',
    defface: 'variable',
    defgeneric: 'function',
    defgroup: 'type',
    'define-advice': 'function',
    'define-derived-mode': 'function',
    'define-error': 'variable',
    'define-generic-mode': 'function',
    'define-global-minor-mode': 'function',
    'define-globalized-minor-mode': 'function',
    'define-inline': 'function',
    'define-minor-mode': 'function',
    'define-skeleton': 'function',
    'define-widget': 'function',
    defmacro: 'function',
    defmethod: 'function',
    defsubst: 'function',
    deftheme: 'type',
    defun: 'function',
    defvar: 'variable',
    'defvar-local': 'variable',
    defvaralias: 'variable',
    'ert-deftest': 'function',
  }) as [string, NameKind][],
)

/** Emacs Lisp で, 後ろの名前を定数の色にする形. */
const ELISP_FEATURES = new Set(['catch', 'throw', 'featurep', 'provide', 'require'])

/** Emacs Lisp で, 警告の色にする形. */
const ELISP_WARNINGS = new Set([
  'cl-assert',
  'cl-check-type',
  'error',
  'signal',
  'user-error',
  'warn',
])

const ELISP_KEYWORD_SET = new Set(ELISP_KEYWORDS)

/** Emacs Lisp の決まり (Emacs の emacs-lisp-mode). */
const elisp: Dialect = {
  head: (word, funcall) => {
    const kind = ELISP_DEFINITIONS.get(word)
    if (kind) {
      // 関数の名前は, `(` の後なら色を付けない (変数と型は付ける)
      return { face: 'keyword', next: pending(kind, kind === 'function' ? null : kind, 1) }
    }
    if (ELISP_FEATURES.has(word)) return { face: 'keyword', next: pending('constant', null, 0) }
    if (ELISP_WARNINGS.has(word)) return { face: 'warning', next: null }
    // マクロとスペシャルフォームは, 関数を呼ぶ所だけ
    return funcall && ELISP_KEYWORD_SET.has(word) ? { face: 'keyword', next: null } : null
  },
  word: (word) => {
    if (/^:./.test(word)) return 'builtin'
    return /^&./.test(word) ? 'typeName' : null
  },
  emacsLisp: true,
}

/** Emacs Lisp で, 1 つ目の引数が束縛のリストの形. */
const ELISP_LETS = new Set([
  'let',
  'let*',
  'letrec',
  'dlet',
  'if-let',
  'if-let*',
  'when-let',
  'when-let*',
  'and-let*',
  'while-let',
])

/** 開いているリスト. */
interface Frame {
  /** リストの頭の名前 (無ければ `null`). */
  head: string | null
  /** 要素の数 (頭を含む). */
  count: number
  /** 関数を呼ぶリストなら `true` (Emacs Lisp). */
  funcall: boolean
  /** 束縛のリスト (`let` の 1 つ目の引数) なら `true` (Emacs Lisp). */
  bindings: boolean
}

/** 行をまたいで覚える状態. */
export interface LispState {
  /** 文字列の途中なら `true`. */
  inString: boolean
  /** `;` のコメントの途中なら `true` (`` `name' `` で分けたとき). */
  lineComment: boolean
  /** 続けて読む字句 (文字列やコメントの中を分けたとき). 長さと種類. */
  queue: [number, string][]
  /** `#| |#` のコメントの深さ (入れ子にできる). */
  commentDepth: number
  /** `#;` でコメントにした式の, 括弧の深さ (コメントでなければ `null`). */
  datum: number | null
  /** 直前の字句が `(` なら `true`. */
  afterParen: boolean
  /** 直前の字句が `'` なら `true`. */
  afterQuote: boolean
  /** 特別な形の後で, 色を付ける名前を待っているなら, その色. */
  pending: Pending | null
  /** 開いているリスト (外から順に). */
  stack: Frame[]
  /** 行の始めからの, 括弧の深さ. */
  lineDepth: number
  /**
   * 閉じた括弧の後ろに続く引数の警告 (Emacs Lisp). 0: 無し, 1: 次の空白でない字句から, 2: 警告している.
   * 行ごとに戻す.
   */
  hiddenArg: 0 | 1 | 2
}

/** 名前 (シンボル, 数, `#t` など) の文字. */
const WORD = /[^\s()[\]";'`,]/

/** Emacs Lisp の文字列とコメントの中の, シンボルの名前 (`\` で始まる 2 文字も含む). */
const SYMBOL = String.raw`(?:[^\s()[\]"'\`‘’,;\\]|\\.)+`

/**
 * Emacs Lisp の文字列の中で, 色を分ける所. 合った所の, 部分ごとの種類 (`null` は文字列のまま).
 * 上から順に試します.
 */
const STRING_SPANS: [RegExp, (string | null)[]][] = [
  // `\\\\` (正規表現の, `\` そのもの) は, そのまま
  [/^(\\\\\\\\)/, [null]],
  // 正規表現のグループ: `\\(`, `\\(?:`, `\\(?1:`, `\\|`, `\\)`
  [/^(\\\\)(\((?:\?[0-9]*:)?|[|)])/, ['regexpBackslash', 'regexpConstruct']],
  // コマンドのキー: `\\[command]`. キーマップ: `\\<map>`, `\\{map}`
  [new RegExp(String.raw`^(\\\\\[)(${SYMBOL})(?=\])`), [null, 'constant']],
  [new RegExp(String.raw`^(\\\\[<{])(${SYMBOL})(?=[>}])`), [null, 'definedVariable']],
  [/^(\\\\=)/, ['builtin']],
  // `name' と ‘name’
  [new RegExp(String.raw`^([\`‘'])(${SYMBOL})(?=['’])`), [null, 'constant']],
  // 要らない `\` (`\.` などは, `.` と同じ. Emacs 30 で, ASCII の文字ごとに確かめたもの)
  [/^(\\)(?=[!#$%&'()*+,\-./89:;<=>?@BDEFGIJKLOPQRTVWXYZ[\]_`cghijklmopqwyz{|}~])/, ['warning']],
]

/** Emacs Lisp のコメントの中で, 色を分ける所 (`` `name' ``). */
const COMMENT_SPANS = STRING_SPANS.slice(5, 6)

/**
 * 続けて読む字句を, 1 つ読みます.
 *
 * @param stream - 行.
 * @param state - 状態.
 * @returns 字句の種類. 続けて読む字句が無ければ `undefined`.
 */
function readQueued(stream: StringStream, state: LispState): string | undefined {
  const next = state.queue.shift()
  if (!next) return undefined
  for (let i = 0; i < next[0]; i++) stream.next()
  return next[1]
}

/**
 * 文字列かコメントの中で, 色を分ける所なら, そこで字句を分けます. 字句の始まりなら, 分けた字句を
 * 続けて読む字句にします.
 *
 * @param stream - 行.
 * @param state - 状態.
 * @param spans - 色を分ける所.
 * @param type - 文字列かコメントの, 字句の種類.
 * @returns 分けるなら `true`.
 */
function atSpan(
  stream: StringStream,
  state: LispState,
  spans: [RegExp, (string | null)[]][],
  type: string,
): boolean {
  // 行の始めの `\(` は, Emacs が括弧を見誤らないための `\` (要るもの)
  for (const [re, kinds] of stream.pos === 0 ? spans.slice(0, -1) : spans) {
    const m = stream.match(re, false) as RegExpMatchArray | null
    if (!m) continue
    if (stream.pos > stream.start) return true
    state.queue = kinds.map((kind, i) => [m[i + 1].length, kind ?? type])
    return true
  }
  return false
}

/**
 * 文字列を, 閉じる `"` か, 行の終わりまで読みます.
 *
 * @param stream - 行.
 * @param state - 状態.
 * @param dialect - 方言.
 * @returns 字句の種類.
 */
function readString(stream: StringStream, state: LispState, dialect: Dialect): string {
  while (!stream.eol()) {
    if (dialect.emacsLisp && atSpan(stream, state, STRING_SPANS, 'string')) {
      return readQueued(stream, state) ?? 'string'
    }
    const ch = stream.next()
    if (ch === '"') {
      state.inString = false
      break
    }
    if (ch === '\\') stream.next()
  }
  return 'string'
}

/**
 * `;` のコメントを, 行の終わりまで読みます.
 *
 * @param stream - 行.
 * @param state - 状態.
 * @param dialect - 方言.
 * @returns 字句の種類.
 */
function readLineComment(stream: StringStream, state: LispState, dialect: Dialect): string {
  while (!stream.eol()) {
    if (dialect.emacsLisp && atSpan(stream, state, COMMENT_SPANS, 'comment')) {
      return readQueued(stream, state) ?? 'comment'
    }
    stream.next()
  }
  state.lineComment = false
  return 'comment'
}

/**
 * `#| |#` のコメントを, 閉じるところか, 行の終わりまで読みます.
 *
 * @param stream - 行.
 * @param state - 状態.
 * @returns 字句の種類.
 */
function readBlockComment(stream: StringStream, state: LispState): string {
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

/**
 * `#;` でコメントにした式を, 1 つの字句ずつ読みます. 式が終わったら, コメントを終えます.
 *
 * @param stream - 行.
 * @param state - 状態.
 * @returns 字句の種類.
 */
function readDatumComment(stream: StringStream, state: LispState): string {
  if (stream.eatSpace()) return 'comment'
  const depth = state.datum as number
  const ch = stream.next() as string
  if (ch === '(' || ch === '[') {
    state.datum = depth + 1
  } else if (ch === ')' || ch === ']') {
    state.datum = depth - 1
  } else if (ch === '"') {
    // 閉じる `"` まで (同じ行に無ければ, 行の終わりまで)
    if (!stream.skipTo('"')) stream.skipToEnd()
    stream.next()
  } else if ("'`,".includes(ch)) {
    // 引用の印は, 後ろの式と合わせて 1 つ
    return 'comment'
  } else {
    stream.eatWhile(WORD)
  }
  if ((state.datum as number) <= 0) state.datum = null
  return 'comment'
}

/**
 * 開いているリストに, 要素を 1 つ足します.
 *
 * @param state - 状態.
 * @param word - 要素が名前なら, その名前.
 */
function addElement(state: LispState, word: string | null = null) {
  const top = state.stack[state.stack.length - 1]
  if (!top) return
  // 状態は写して使い回すので, 書き換えずに作り直す
  state.stack[state.stack.length - 1] = {
    ...top,
    head: top.count === 0 ? word : top.head,
    count: top.count + 1,
  }
}

/**
 * `(` を開きます. Emacs Lisp では, 関数を呼ぶリストかを決めます (Emacs の `lisp--el-funcall-position-p`.
 * `'(` と, `let` の束縛は, 関数を呼ばない).
 *
 * @param state - 状態.
 * @param quoted - 直前が `'` なら `true`.
 */
function openList(state: LispState, quoted: boolean) {
  const parent = state.stack[state.stack.length - 1] as Frame | undefined
  const funcall = !quoted && !parent?.bindings
  const bindings = !!parent && ELISP_LETS.has(parent.head as string) && parent.count === 1
  addElement(state)
  state.stack.push({ head: null, count: 0, funcall, bindings })
}

/**
 * 字句を 1 つ読みます.
 *
 * @param stream - 行.
 * @param state - 状態.
 * @param dialect - 方言.
 * @returns 字句の種類 (色を付けなければ `null`).
 */
function readToken(stream: StringStream, state: LispState, dialect: Dialect): string | null {
  const queued = readQueued(stream, state)
  if (queued) return queued
  if (state.inString) return readString(stream, state, dialect)
  if (state.lineComment) return readLineComment(stream, state, dialect)
  if (state.commentDepth > 0) return readBlockComment(stream, state)
  if (state.datum !== null) return readDatumComment(stream, state)
  if (stream.eatSpace()) return null
  const { afterParen, afterQuote, pending: waiting } = state
  state.afterParen = false
  state.afterQuote = false
  state.pending = null
  const ch = stream.next() as string
  if (ch === ';') {
    state.lineComment = true
    return readLineComment(stream, state, dialect)
  }
  if (ch === '"') {
    addElement(state)
    state.inString = true
    return readString(stream, state, dialect)
  }
  if (!dialect.emacsLisp && ch === '#' && stream.eat('|')) {
    state.commentDepth = 1
    return readBlockComment(stream, state)
  }
  if (!dialect.emacsLisp && ch === '#' && stream.eat(';')) {
    state.datum = 0
    return 'comment'
  }
  if (ch === '(' || ch === '[') {
    state.afterParen = ch === '('
    state.lineDepth++
    openList(state, afterQuote)
    if (ch === '(' && waiting && waiting.parens > 0) {
      state.pending = pending(waiting.inParen, waiting.inParen, waiting.parens - 1)
    }
    return 'paren'
  }
  if (ch === ')' || ch === ']') {
    state.stack.pop()
    state.lineDepth--
    // 前の行で開いた括弧を閉じて, 後ろに引数が続くなら, 警告する (`;` のコメントと, 閉じる括弧は除く)
    if (
      dialect.emacsLisp &&
      state.lineDepth < 0 &&
      state.hiddenArg === 0 &&
      !stream.match(/^\s*(?:$|;|[)\]])/, false)
    ) {
      state.hiddenArg = 1
    }
    return 'paren'
  }
  if (ch === "'") {
    state.afterQuote = true
    // Emacs Lisp の `(defalias 'f` は, `'` の後の名前を待つ
    if (dialect.emacsLisp) state.pending = waiting
    return null
  }
  // `#'` は, 後ろの式と合わせて 1 つ (`#'(lambda` は, 関数を呼ぶリスト)
  if (dialect.emacsLisp && ch === '#' && stream.eat("'")) return null
  if (!WORD.test(ch)) return null
  // 文字 (Scheme の `#\(`, Emacs Lisp の `?(` と `?\(`) は, 括弧も名前の文字にする
  if (!dialect.emacsLisp && ch === '#' && stream.eat('\\')) stream.next()
  if (dialect.emacsLisp && ch === '?') {
    stream.eat('\\')
    stream.next()
  }
  stream.eatWhile(WORD)
  const word = stream.current()
  addElement(state, word)
  if (waiting) return waiting.kind && NAME_TOKEN[waiting.kind]
  const top = state.stack[state.stack.length - 1]
  const head = afterParen ? dialect.head(word, top.funcall) : null
  if (head) {
    state.pending = head.next
    return head.face
  }
  return dialect.word(word)
}

/**
 * 字句を 1 つ読みます. 閉じた括弧の後ろに続く引数は, 警告の色にします.
 *
 * @param stream - 行.
 * @param state - 状態.
 * @param dialect - 方言.
 * @returns 字句の種類 (色を付けなければ `null`).
 */
function token(stream: StringStream, state: LispState, dialect: Dialect): string | null {
  if (stream.sol()) {
    // 特別な形の後の名前は, 同じ行にあるときだけ
    state.pending = null
    state.lineDepth = 0
    state.hiddenArg = 0
  }
  const hidden = state.hiddenArg
  const type = readToken(stream, state, dialect)
  // 警告は, 括弧の後ろの, 空白でない字句から, 行の終わりまで
  if (hidden === 0 || (hidden === 1 && /^\s+$/.test(stream.current()))) return type
  state.hiddenArg = 2
  return 'warning'
}

/** Emacs の警告の色 (`font-lock-warning-face`) にする字句. */
const warningTag = Tag.define()

/** Emacs の正規表現のグループの色 (`font-lock-regexp-grouping-backslash` など) にする字句. */
const regexpGroupTag = Tag.define()

/**
 * 方言の字句を読む関数を作ります.
 *
 * @param name - 言語の名前.
 * @param dialect - 方言.
 * @returns 字句を読む関数 (CodeMirror 5 の書き方).
 */
function lispParser(name: string, dialect: Dialect): StreamParser<LispState> {
  return {
    name,
    startState: () => ({
      inString: false,
      lineComment: false,
      queue: [],
      commentDepth: 0,
      datum: null,
      afterParen: false,
      afterQuote: false,
      pending: null,
      stack: [],
      lineDepth: 0,
      hiddenArg: 0,
    }),
    // 開いているリストは, 書き換えずに作り直すので, 配列を写すだけでよい
    copyState: (state) => ({ ...state, queue: [...state.queue], stack: [...state.stack] }),
    token: (stream, state) => token(stream, state, dialect),
    tokenTable: {
      definedFunction: tags.function(tags.definition(tags.variableName)),
      definedVariable: tags.definition(tags.variableName),
      constant: tags.constant(tags.name),
      builtin: tags.standard(tags.name),
      warning: warningTag,
      regexpBackslash: regexpGroupTag,
      regexpConstruct: regexpGroupTag,
    },
    languageData: { commentTokens: { line: ';' } },
  }
}

/** Scheme の字句を読む関数. */
export const schemeParser = lispParser('scheme', scheme)

/** Emacs Lisp の字句を読む関数. */
export const elispParser = lispParser('elisp', elisp)

/** Emacs の既定の色 (明るい背景の font-lock の face). */
const emacsHighlightStyle = HighlightStyle.define([
  // font-lock-keyword-face (Purple)
  { tag: tags.keyword, color: '#a020f0' },
  // font-lock-comment-face (Firebrick)
  { tag: tags.comment, color: '#b22222' },
  // font-lock-string-face (VioletRed4). font-lock-doc-face も同じ色
  { tag: tags.string, color: '#8b2252' },
  // font-lock-function-name-face (Blue1)
  { tag: tags.function(tags.definition(tags.variableName)), color: '#0000ff' },
  // font-lock-variable-name-face (sienna)
  { tag: tags.definition(tags.variableName), color: '#a0522d' },
  // font-lock-type-face (ForestGreen)
  { tag: tags.typeName, color: '#228b22' },
  // font-lock-constant-face (dark cyan)
  { tag: tags.constant(tags.name), color: '#008b8b' },
  // font-lock-builtin-face (dark slate blue)
  { tag: tags.standard(tags.name), color: '#483d8b' },
  // font-lock-warning-face (Red1, 太字)
  { tag: warningTag, color: '#ff0000', fontWeight: 'bold' },
  // font-lock-regexp-grouping-backslash と -construct (文字列の色の, 太字)
  { tag: regexpGroupTag, color: '#8b2252', fontWeight: 'bold' },
])

/** 言語ごとの, シンタックスハイライトの拡張. */
export const lispHighlighting = {
  scheme: [StreamLanguage.define(schemeParser), syntaxHighlighting(emacsHighlightStyle)],
  elisp: [StreamLanguage.define(elispParser), syntaxHighlighting(emacsHighlightStyle)],
}
