/**
 * 単語の補完 (Emacs の `dabbrev-expand`. `M-/`). カーソルの前の単語の始め (略語) で始まる単語を,
 * 同じバッファから探します. カーソルの前を近い順に, 次にカーソルの後ろを近い順に探します.
 * エディタ (CodeMirror) に依存しないので, 単体でテストできます.
 */

/** 単語 (英数字と `_`). */
const WORD = /\w+/g

/** Lisp の記号 (Emacs の構文表で, 単語と記号の文字. `-` や `?` を含む). */
const SYMBOL = /[^\s()[\]";'`,]+/g

/** 略語と, 補完の候補. */
export interface Expansions {
  /** 略語の始めの位置 (略語の終わりは, カーソルの位置). */
  from: number
  /** 略語. */
  abbrev: string
  /** 候補 (探した順. 同じものは除く). */
  candidates: string[]
}

/**
 * カーソルの前の略語と, 補完の候補を返します.
 *
 * @param text - 本文.
 * @param pos - カーソルの位置.
 * @param lisp - Lisp なら `true` (記号を, 1 つの単語として扱う).
 * @returns 略語と候補. カーソルの前が単語でなければ `null`.
 */
export function findExpansions(text: string, pos: number, lisp: boolean): Expansions | null {
  const pattern = lisp ? SYMBOL : WORD
  const words = [...text.matchAll(pattern)].map((m) => ({
    start: m.index as number,
    word: m[0],
  }))
  // カーソルの前で終わる (またはカーソルを含む) 単語の, カーソルまでが略語
  const current = words.find((w) => w.start < pos && pos <= w.start + w.word.length)
  if (!current) return null
  const abbrev = text.slice(current.start, pos)
  const matches = (w: { start: number; word: string }) =>
    w.start !== current.start && w.word.startsWith(abbrev) && w.word !== abbrev
  const before = words.filter((w) => w.start < current.start && matches(w)).reverse()
  const after = words.filter((w) => w.start > current.start && matches(w))
  const candidates = [...new Set([...before, ...after].map((w) => w.word))]
  return { from: current.start, abbrev, candidates }
}
