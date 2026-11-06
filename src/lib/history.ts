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

/** 履歴をたどった結果: 新しい位置, または知らせ (エコー領域に出す文字). */
export type HistoryStep = { pos: number } | { error: string }

/**
 * 履歴の位置を 1 つ動かします (`M-p`, `M-n`).
 * 位置 0 は入力していた文字, 1 以上は履歴 (1 が最も新しい), -1 以下は既定の値 (`M-n` で出す).
 *
 * @param history - 履歴 (新しい順).
 * @param defaults - 既定の値.
 * @param pos - 今の位置.
 * @param older - 古い方へ動くなら `true` (`M-p`).
 * @returns 新しい位置. 動けなければ知らせ.
 */
export function stepHistory(
  history: readonly string[],
  defaults: readonly string[],
  pos: number,
  older: boolean,
): HistoryStep {
  if (older) {
    return pos >= history.length
      ? { error: 'Beginning of history; no preceding item' }
      : { pos: pos + 1 }
  }
  if (pos <= -defaults.length) {
    return {
      error:
        defaults.length === 0
          ? 'End of history; no default available'
          : 'End of defaults; no next item',
    }
  }
  return { pos: pos - 1 }
}

/**
 * 正規表現に合う履歴を探します (`M-r`, `M-s`).
 *
 * @param history - 履歴 (新しい順).
 * @param pos - 今の位置.
 * @param regexp - 正規表現.
 * @param older - 古い方へ探すなら `true` (`M-r`).
 * @returns 見つけた位置. 無ければ知らせ.
 */
export function searchHistory(
  history: readonly string[],
  pos: number,
  regexp: string,
  older: boolean,
): HistoryStep {
  let pattern: RegExp
  try {
    pattern = new RegExp(regexp, 'i')
  } catch (e) {
    return { error: (e as Error).message }
  }
  const range = older
    ? Array.from({ length: Math.max(history.length - pos, 0) }, (_, i) => pos + 1 + i)
    : Array.from({ length: Math.max(pos - 1, 0) }, (_, i) => pos - 1 - i)
  const found = range.find((p) => pattern.test(history[p - 1]))
  if (found === undefined) {
    return { error: older ? 'No earlier matching history item' : 'No later matching history item' }
  }
  return { pos: found }
}

/**
 * 履歴の位置にある文字を返します.
 *
 * @param history - 履歴 (新しい順).
 * @param defaults - 既定の値.
 * @param saved - 履歴を出す前に入力していた文字 (位置 0).
 * @param pos - 位置 (0 は入力していた文字, 1 以上は履歴, -1 以下は既定の値).
 * @returns 文字.
 */
export function historyText(
  history: readonly string[],
  defaults: readonly string[],
  saved: string,
  pos: number,
): string {
  if (pos === 0) return saved
  return pos > 0 ? history[pos - 1] : defaults[-pos - 1]
}
