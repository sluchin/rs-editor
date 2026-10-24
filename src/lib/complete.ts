/**
 * ミニバッファの補完 (`M-x` のコマンドの名前, `C-x b` のバッファの名前).
 */

/** 補完の結果. */
export interface Completion {
  /** 補完した入力 (候補に共通する, 先頭の部分まで). */
  value: string
  /** 入力で始まる候補. */
  matches: string[]
}

/**
 * 入力を, 前方一致する候補の, 共通する先頭の部分まで補完します.
 *
 * @param input - 入力.
 * @param candidates - 候補.
 * @returns 補完した入力と, 前方一致する候補 (無ければ, 入力のまま).
 */
export function complete(input: string, candidates: readonly string[]): Completion {
  const matches = [...new Set(candidates.filter((c) => c.startsWith(input)))].sort()
  if (matches.length === 0) return { value: input, matches }
  let prefix = matches[0]
  for (const match of matches) {
    while (!match.startsWith(prefix)) prefix = prefix.slice(0, -1)
  }
  return { value: prefix, matches }
}
