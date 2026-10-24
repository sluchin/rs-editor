/**
 * ミニバッファの補完 (`M-x` のコマンドの名前, `C-x b` のバッファの名前, `C-x C-f` のファイルの名前).
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

/**
 * ファイルの名前を補完します. 最後の `/` の後ろを, そのディレクトリの中の名前で補完します.
 *
 * @param input - 入力 (パス).
 * @param list - ディレクトリの中の名前を返す関数 (ディレクトリには, 後ろに `/` が付く).
 * @returns 補完した入力と, 前方一致する名前 (ディレクトリを読めなければ, 候補なし).
 */
export async function completeFileName(
  input: string,
  list: (dir: string) => Promise<string[]>,
): Promise<Completion> {
  const dir = input.slice(0, input.lastIndexOf('/') + 1)
  const names = await list(dir).catch(() => [])
  const { value, matches } = complete(input.slice(dir.length), names)
  return { value: dir + value, matches }
}
