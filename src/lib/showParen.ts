/**
 * 括弧の強調 (Emacs 30 の `show-paren-mode`). カーソルの前の閉じる括弧か, 後ろの開く括弧と,
 * 対になる括弧に色を付けます. 対が無いか, 括弧の種類が違えば, 合わない色にします.
 */

import type { Text } from '@codemirror/state'
import {
  Decoration,
  type DecorationSet,
  type EditorView,
  ViewPlugin,
  type ViewUpdate,
} from '@codemirror/view'
import { indentConfig, LispParser } from './lispIndent'

/** 括弧の対. */
interface Paren {
  /** 対になる括弧の位置 (無ければ `null`). */
  match: number | null
  /** 対が無いか, 括弧の種類が違えば `true`. */
  mismatch: boolean
}

/** Lisp の方言 (`null` なら, Lisp でない Fundamental モード). */
type Dialect = 'scheme' | 'elisp' | null

/** 開く括弧と, 対になる閉じる括弧. */
const PAIRS: Record<string, string> = { '(': ')', '[': ']', '{': '}' }
/** 閉じる括弧と, 対になる開く括弧. */
const CLOSERS: Record<string, string> = { ')': '(', ']': '[', '}': '{' }

/** 本文ごとの, 括弧の対 (カーソルを動かすたびに読み直さない). */
const cache = new WeakMap<Text, Map<Dialect, Map<number, Paren>>>()

/**
 * 括弧の種類を返します. Lisp のモードでは `{}` は括弧でない (Emacs の構文表).
 *
 * @param ch - 文字.
 * @param dialect - Lisp の方言 (`null` なら Fundamental モード).
 * @returns 開く括弧なら `'open'`, 閉じる括弧なら `'close'`. 括弧でなければ `null`.
 */
function bracketKind(ch: string, dialect: Dialect): 'open' | 'close' | null {
  if (dialect && (ch === '{' || ch === '}')) return null
  return ch in PAIRS ? 'open' : ch in CLOSERS ? 'close' : null
}

/**
 * 本文の括弧 (文字列とコメントの中のものを除く) を, 対にします.
 *
 * @param doc - 本文.
 * @param dialect - Lisp の方言 (`null` なら Fundamental モード).
 * @returns 括弧の位置と, 対.
 */
function parens(doc: Text, dialect: Dialect): Map<number, Paren> {
  let byDialect = cache.get(doc)
  if (!byDialect) cache.set(doc, (byDialect = new Map()))
  const cached = byDialect.get(dialect)
  if (cached) return cached
  const result = new Map<number, Paren>()
  const open: { pos: number; ch: string }[] = []
  const paren = (pos: number, ch: string) => {
    if (bracketKind(ch, dialect) === 'open') {
      open.push({ pos, ch })
      return
    }
    const top = open.pop()
    if (!top) {
      result.set(pos, { match: null, mismatch: true })
      return
    }
    const mismatch = PAIRS[top.ch] !== ch
    result.set(pos, { match: top.pos, mismatch })
    result.set(top.pos, { match: pos, mismatch })
  }
  if (dialect) {
    // Lisp は, 字下げと同じく読む (文字列, コメント, `#;` の式, 文字の括弧を除く)
    const parser = new LispParser(dialect === 'scheme', 8)
    const starts: number[] = []
    parser.onParen = (line, index, ch) => paren(starts[line] + index, ch)
    for (let n = 1; n <= doc.lines; n++) {
      const line = doc.line(n)
      starts.push(line.from)
      parser.scanLine(line.text, n - 1)
    }
  } else {
    // Fundamental モード (Emacs の標準の構文表): `"` で文字列, `\` で次の文字をそのままにする
    const text = doc.toString()
    let inString = false
    for (let i = 0; i < text.length; i++) {
      const ch = text[i]
      if (ch === '\\') i++
      else if (ch === '"') inString = !inString
      else if (!inString && bracketKind(ch, null)) paren(i, ch)
    }
  }
  for (const { pos } of open) result.set(pos, { match: null, mismatch: true })
  byDialect.set(dialect, result)
  return result
}

/**
 * 位置の文字が, `\` で始まる文字 (`\(`, Scheme の `#\(`) なら `true` を返します.
 *
 * @param text - 本文.
 * @param pos - 位置.
 * @returns `\` の後なら `true`.
 */
function escaped(text: string, pos: number): boolean {
  let count = 0
  while (text[pos - 1 - count] === '\\') count++
  return count % 2 === 1
}

/**
 * 文字列とコメントの中の括弧の対を, 同じ行の中で探します (Emacs も, 中の括弧を対にする).
 * `"` で終わります.
 *
 * @param text - 本文.
 * @param pos - 括弧の位置.
 * @param dialect - Lisp の方言.
 * @returns 対.
 */
function scanNearby(text: string, pos: number, dialect: Dialect): Paren {
  const ch = text[pos]
  const forward = bracketKind(ch, dialect) === 'open'
  const step = forward ? 1 : -1
  const stack = [ch]
  for (let i = pos + step; i >= 0 && i < text.length; i += step) {
    const c = text[i]
    if (c === '\n' || c === '"') break
    const kind = bracketKind(c, dialect)
    if (!kind || escaped(text, i)) continue
    if (kind === (forward ? 'open' : 'close')) {
      stack.push(c)
      continue
    }
    const top = stack.pop() as string
    if (stack.length === 0) {
      const mismatch = forward ? PAIRS[top] !== c : PAIRS[c] !== top
      return { match: i, mismatch }
    }
  }
  return { match: null, mismatch: true }
}

/**
 * カーソルの近くの括弧と, 対を返します (Emacs の `show-paren--default`). 前の閉じる括弧を先に見ます.
 *
 * @param doc - 本文.
 * @param pos - カーソルの位置.
 * @param dialect - Lisp の方言 (`null` なら Fundamental モード).
 * @returns 括弧の位置と対. 無ければ `null`.
 */
export function findParen(
  doc: Text,
  pos: number,
  dialect: Dialect,
): { here: number; paren: Paren } | null {
  const text = doc.toString()
  const candidate = (at: number, kind: 'open' | 'close') => {
    if (at < 0 || bracketKind(text[at] ?? '', dialect) !== kind || escaped(text, at)) return null
    const paren = parens(doc, dialect).get(at) ?? scanNearby(text, at, dialect)
    return { here: at, paren }
  }
  return candidate(pos - 1, 'close') ?? candidate(pos, 'open')
}

/** 対になる括弧の色. */
const matchMark = Decoration.mark({ class: 'cm-show-paren-match' })
/** 合わない括弧の色. */
const mismatchMark = Decoration.mark({ class: 'cm-show-paren-mismatch' })

/**
 * 括弧の強調の色を作ります. フォーカスのあるエディタだけに付けます.
 *
 * @param view - エディタ.
 * @returns 色.
 */
function decorations(view: EditorView): DecorationSet {
  const { state } = view
  if (!view.hasFocus) return Decoration.none
  const found = findParen(state.doc, state.selection.main.head, state.facet(indentConfig).dialect)
  if (!found) return Decoration.none
  const { here, paren } = found
  const mark = paren.mismatch ? mismatchMark : matchMark
  const ranges = [here, ...(paren.match === null ? [] : [paren.match])].sort((a, b) => a - b)
  return Decoration.set(ranges.map((at) => mark.range(at, at + 1)))
}

/**
 * 括弧の強調の拡張を返します.
 *
 * @returns 拡張.
 */
export function showParen() {
  return ViewPlugin.fromClass(
    class {
      /** 括弧の強調. */
      decorations: DecorationSet

      /**
       * 括弧の強調を作ります.
       *
       * @param view - エディタ.
       */
      constructor(view: EditorView) {
        this.decorations = decorations(view)
      }

      /**
       * 本文, カーソル, フォーカス, 字下げの設定 (方言) が変わったら, 括弧の強調を作り直します.
       *
       * @param update - エディタの変化.
       */
      update(update: ViewUpdate) {
        if (
          update.docChanged ||
          update.selectionSet ||
          update.focusChanged ||
          update.startState.facet(indentConfig) !== update.state.facet(indentConfig)
        ) {
          this.decorations = decorations(update.view)
        }
      }
    },
    { decorations: (plugin) => plugin.decorations },
  )
}
