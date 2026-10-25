/**
 * isearch の検索 (文字と, 正規表現).
 */

/** 合った所. */
export interface IsearchMatch {
  /** 始まりの位置. */
  start: number
  /** 終わりの位置 (この位置の文字は含まない). */
  end: number
}

/**
 * 正規表現を作ります. `^` と `$` は, 行の始めと終わりに合います (Emacs と同じ).
 *
 * @param pattern - 正規表現 (JavaScript の書き方).
 * @returns 正規表現. 書き方が誤っていれば `null`.
 */
export function compileRegexp(pattern: string): RegExp | null {
  try {
    return new RegExp(pattern, 'gm')
  } catch {
    return null
  }
}

/**
 * 正規表現に合う, 空でない全ての所を返します.
 *
 * @param text - 本文.
 * @param pattern - 正規表現.
 * @returns 合う所 (前から順に). 正規表現が誤っていれば, 空.
 */
function regexpMatches(text: string, pattern: string): IsearchMatch[] {
  const re = compileRegexp(pattern)
  if (!re) return []
  return [...text.matchAll(re)]
    .filter((m) => m[0] !== '')
    .map((m) => ({ start: m.index as number, end: (m.index as number) + m[0].length }))
}

/**
 * 次に合う所を探します.
 *
 * @param text - 本文.
 * @param query - 探す文字 (`regexp` なら正規表現).
 * @param fromPos - 探し始める位置 (後ろへ探すときは, この位置までに始まる所).
 * @param direction - 探す向き.
 * @param regexp - 正規表現で探すなら `true`.
 * @returns 合う所. 無ければ `null`.
 */
export function findNextMatch(
  text: string,
  query: string,
  fromPos: number,
  direction: 'forward' | 'backward',
  regexp = false,
): IsearchMatch | null {
  if (query === '') return null

  if (regexp) {
    const matches = regexpMatches(text, query)
    if (direction === 'forward') return matches.find((m) => m.start >= fromPos) ?? null
    const before = matches.filter((m) => m.start <= fromPos)
    return before.length > 0 ? before[before.length - 1] : null
  }

  if (direction === 'forward') {
    const idx = text.indexOf(query, fromPos)
    return idx === -1 ? null : { start: idx, end: idx + query.length }
  }

  const idx = text.lastIndexOf(query, Math.max(0, fromPos))
  return idx === -1 ? null : { start: idx, end: idx + query.length }
}
