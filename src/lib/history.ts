/**
 * ミニバッファの入力の履歴.
 * エディタ (CodeMirror) に依存しないので, 単体でテストできます.
 */

/** ミニバッファの履歴の数の限り (Emacs の `history-length`). */
export const HISTORY_LENGTH = 100

/**
 * 履歴の先頭に加えます (Emacs の `add-to-history`). 空の文字と, 先頭と同じ文字は, 加えません.
 *
 * @param history - 履歴 (新しい順).
 * @param item - 加える文字.
 * @param max - 数の限り (超えたら, 古い方から捨てる).
 * @param deleteDuplicates - 同じ文字を, 先頭のほかからも除くなら `true` (Emacs の `history-delete-duplicates`).
 * @returns 新しい履歴.
 */
export function addToHistory(
  history: readonly string[],
  item: string,
  max: number,
  deleteDuplicates = false,
): string[] {
  if (item === '' || history[0] === item) return [...history]
  const rest = deleteDuplicates ? history.filter((h) => h !== item) : history
  return [item, ...rest].slice(0, max)
}
