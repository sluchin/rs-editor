/**
 * Lisp の本文を行ごとに読みます (括弧の強調に必要な部分だけを, Emacs の `parse-partial-sexp` と同じように扱います).
 * 文字列, コメント (`;`, Scheme の `#| |#` と `#;` の式), 文字 (`#\(`, `?\(`) の中の括弧を除いて括弧を知らせます.
 */

import { Facet } from '@codemirror/state'

/** Lisp の方言 (`null` なら Lisp ではない Fundamental モード). */
export type Dialect = 'scheme' | 'elisp' | null

/** バッファの方言 (括弧の強調で使う). */
export const dialectConfig = Facet.define<Dialect, Dialect>({
  combine: (values) => values[0] ?? null,
})

/** Lisp の本文を行ごとに読むクラス. 文字列やコメントの途中かどうかを, 行をまたいで覚えます. */
export class LispParser {
  /** 文字列の中なら `true`. */
  private inString = false
  /** `#| |#` のコメントの深さ (Scheme). */
  private blockComment = 0
  /** `#;` でコメントにした式の括弧の深さ (Scheme). コメントの中でなければ `null`. */
  private datum: number | null = null

  /**
   * 読み取り器を作ります.
   *
   * @param scheme - Scheme なら `true` (Emacs Lisp なら `false`).
   * @param onParen - 括弧を読んだときに呼ぶ関数 (文字列, コメント, 文字の中の括弧は呼ばない).
   */
  constructor(
    private readonly scheme: boolean,
    private readonly onParen: (line: number, index: number, ch: string) => void,
  ) {}

  /**
   * 1 行を読みます.
   *
   * @param text - 行の本文.
   * @param line - 行番号 (0 から数える).
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
        // `#;` の後の式はコメントになる (Emacs の scheme-mode と同じ)
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
        // Scheme の文字 (`#\(`) では, 括弧も文字の名前の一部になる
        i += 3
      } else {
        // `\` の次の文字は読み飛ばす (Emacs Lisp の `?\(` の括弧は文字の一部. `?(` の `(` は Emacs と同じく開き括弧)
        i += ch === '\\' ? 2 : 1
      }
    }
  }

  /**
   * `#;` でコメントにした式の字句を 1 つ読み飛ばします. 式が終わったらコメントを終えます.
   *
   * @param text - 行の本文.
   * @param i - 読み始める位置.
   * @returns 次の位置.
   */
  private skipDatum(text: string, i: number): number {
    const ch = text[i]
    // 空白と, 式の前の印 (`'`, `#(` など. `#\` で始まる文字は除く) は, 後ろの式とまとめて 1 つの式として扱う
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
