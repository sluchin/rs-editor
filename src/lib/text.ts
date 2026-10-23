/**
 * 文字列の上の, 位置の計算 (単語, 行) と, 単語の大文字・小文字の変換.
 * エディタ (CodeMirror) に依存しないので, 単体でテストできます.
 */

/** 単語の文字 (英数字と `_`). */
const WORD = /\w/

/**
 * 位置から次の単語の終わりを返します (Emacs の forward-word). 単語の前の区切りは飛ばします.
 *
 * @param text - 文字列.
 * @param pos - 位置.
 * @returns 単語の終わりの位置. 単語が無ければ, 文字列の終わり.
 */
export function wordForward(text: string, pos: number): number {
  let i = pos
  while (i < text.length && !WORD.test(text[i])) i++
  while (i < text.length && WORD.test(text[i])) i++
  return i
}

/**
 * 位置から前の単語の始めを返します (Emacs の backward-word). 単語の後の区切りは飛ばします.
 *
 * @param text - 文字列.
 * @param pos - 位置.
 * @returns 単語の始めの位置. 単語が無ければ, 0.
 */
export function wordBackward(text: string, pos: number): number {
  let i = pos
  while (i > 0 && !WORD.test(text[i - 1])) i--
  while (i > 0 && WORD.test(text[i - 1])) i--
  return i
}

/** 単語の大文字・小文字の変え方. */
export type CaseKind = 'upcase' | 'downcase' | 'capitalize'

/**
 * 文字列の大文字・小文字を変えます. `capitalize` は, 単語ごとに, 始めを大文字に, 残りを小文字にします.
 *
 * @param text - 文字列.
 * @param kind - 変え方.
 * @returns 変えた文字列.
 */
export function changeCase(text: string, kind: CaseKind): string {
  if (kind === 'upcase') return text.toUpperCase()
  if (kind === 'downcase') return text.toLowerCase()
  return text.replace(/\w+/g, (word) => word[0].toUpperCase() + word.slice(1).toLowerCase())
}

/**
 * 1 から数えた行の番号を, 文字列の行の数の範囲に収めます.
 *
 * @param line - 行の番号 (入力のまま. 数でなければ 1).
 * @param lines - 行の数.
 * @returns 1 から `lines` までの行の番号.
 */
export function clampLine(line: number, lines: number): number {
  if (!Number.isFinite(line)) return 1
  return Math.min(Math.max(Math.trunc(line), 1), lines)
}
