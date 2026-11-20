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

/** 単語を分ける文字 (`partial-completion` が, 入力の部分と候補の単語を対応させる). */
const WORD_SEPARATOR = /([-_. /])/

/**
 * 入力が, 候補に `partial-completion` で当たるか (`f-f` が `find-file` に当たる).
 * 入力と候補を, 区切りの文字 (`-` `_` `.` 空白 `/`) で分け, 区切りが同じで, 入力の各部分が候補の同じ位置の単語の前方であれば当たる.
 *
 * @param input - 入力.
 * @param candidate - 候補.
 * @returns 当たるなら `true`.
 */
export function partialMatch(input: string, candidate: string): boolean {
  const wanted = input.split(WORD_SEPARATOR)
  const actual = candidate.split(WORD_SEPARATOR)
  return (
    wanted.length === actual.length &&
    wanted.every((part, i) => (i % 2 === 1 ? part === actual[i] : actual[i].startsWith(part)))
  )
}

/**
 * 入力を, 前方一致する候補の, 共通する先頭の部分まで補完します.
 * 前方一致する候補が無ければ, `partial-completion` (`f-f` が `find-file` に当たる) で探し,
 * 1 つに決まれば, その候補まで補完します (複数なら入力のまま. 一覧に出す).
 *
 * @param input - 入力.
 * @param candidates - 候補.
 * @returns 補完した入力と, 一致する候補 (無ければ入力のまま).
 */
export function complete(input: string, candidates: readonly string[]): Completion {
  let matches = [...new Set(candidates.filter((c) => c.startsWith(input)))].sort()
  if (matches.length === 0 && input !== '') {
    matches = [...new Set(candidates.filter((c) => partialMatch(input, c)))].sort()
    return { value: matches.length === 1 ? matches[0] : input, matches }
  }
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
