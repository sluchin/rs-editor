/**
 * Lisp の本文を, 行ごとに読みます (Emacs の `parse-partial-sexp` のうち, 括弧の強調に要る所).
 * 文字列, コメント (`;`, Scheme の `#| |#` と `#;` の式), 文字 (`#\(`, `?\(`) の中の括弧を除いて, 括弧を知らせます.
 */

import { Facet } from '@codemirror/state'

/** Lisp の方言 (`null` なら, Lisp でない Fundamental モード). */
export type Dialect = 'scheme' | 'elisp' | null

/** バッファの方言 (括弧の強調で使う). */
export const dialectConfig = Facet.define<Dialect, Dialect>({
  combine: (values) => values[0] ?? null,
})

/** Lisp の本文を, 行ごとに読むもの. 文字列とコメントの中かを, 行をまたいで覚えます. */
export class LispParser {
  /** 文字列の中なら `true`. */
  private inString = false
  /** `#| |#` のコメントの深さ (Scheme). */
  private blockComment = 0
  /** `#;` でコメントにした式の, 括弧の深さ (Scheme. コメントでなければ `null`). */
  private datum: number | null = null

  /**
   * 作ります.
   *
   * @param scheme - Scheme なら `true` (Emacs Lisp なら `false`).
   * @param onParen - 括弧 (文字列, コメント, 文字の中のものを除く) を読んだときに呼びます.
   */
  constructor(
    private readonly scheme: boolean,
    private readonly onParen: (line: number, index: number, ch: string) => void,
  ) {}

  /**
   * 1 行を読みます.
   *
   * @param text - 行.
   * @param line - 行 (0 から).
   */
  scanLine(text: string, line: number) {
    let i = 0
    while (i < text.length) {
      const ch = text[i]
      if (this.inString) {
        if (ch === '"') this.inString = false
        i += ch === '\\' ? 2 : 1
      } else if (this.blockComment > 0) {
        const close = text.startsWith('|#', i)
        const open = text.startsWith('#|', i)
        this.blockComment += close ? -1 : open ? 1 : 0
        i += close || open ? 2 : 1
      } else if (ch === ';') {
        break
      } else if (this.datum !== null) {
        i = this.skipDatum(text, i)
      } else if (this.scheme && (text.startsWith('#|', i) || text.startsWith('#;', i))) {
        // `#;` の後の式は, コメント (Emacs の scheme-mode)
        if (text[i + 1] === '|') this.blockComment = 1
        else this.datum = 0
        i += 2
      } else if (ch === '"') {
        this.inString = true
        i++
      } else if ('()[]'.includes(ch)) {
        this.onParen(line, i, ch)
        i++
      } else if (this.scheme && text.startsWith('#\\', i)) {
        // Scheme の文字 (`#\(`) は, 括弧も名前の文字
        i += 3
      } else {
        // Emacs Lisp の `?\(` は, `\` で括弧を名前の文字にする (`?(` の `(` は, Emacs と同じく開く括弧)
        i += ch === '\\' ? 2 : 1
      }
    }
  }

  /**
   * `#;` でコメントにした式の, 字句を 1 つ読み飛ばします. 式が終わったら, コメントを終えます.
   *
   * @param text - 行.
   * @param i - 位置.
   * @returns 次の位置.
   */
  private skipDatum(text: string, i: number): number {
    const ch = text[i]
    // 空白と, 式の前の印 (`'`, `#(` など. `#\` で始まる文字を除く) は, 後ろの式と合わせて 1 つ
    if (" \t'`,".includes(ch) || (ch === '#' && text[i + 1] !== '\\')) return i + 1
    const depth = this.datum as number
    let next = i + 1
    if (ch === '(' || ch === '[') this.datum = depth + 1
    else if (ch === ')' || ch === ']') this.datum = depth - 1
    else if (ch === '"') this.inString = true
    else {
      if (text.startsWith('#\\', i)) next = i + 3
      while (next < text.length && !/[\s()[\]";'`,]/.test(text[next])) next++
    }
    if ((this.datum as number) <= 0) this.datum = null
    return next
  }
}
