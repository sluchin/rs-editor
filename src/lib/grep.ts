/**
 * `M-x grep` の結果のバッファ (`*grep*`). 探すのは, エディタ (Rust) が行います (外のコマンドは使わない).
 *
 * バッファは Emacs の grep-mode と同じく, `ファイル:行:行の文字` を並べます. `RET` で, その行のファイルを開きます.
 * 行と, 合った行は, 行の番号で結び付けます (`ファイル:行:` を読むと, `:` を含む名前で誤るため).
 */

import type { GrepMatch } from './fileOps'

/** 結果のバッファの名前. */
export const GREP_BUFFER_NAME = '*grep*'

/** 曜日 (Emacs の `current-time-string`). */
const DAYS = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat']

/** 月 (Emacs の `current-time-string`). */
const MONTHS = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec']

/**
 * 時刻を, Emacs の `current-time-string` の形 (`Sun Oct 25 19:30:12 2026`) にします.
 *
 * @param date - 時刻.
 * @returns 文字列.
 */
export function timeString(date: Date): string {
  const pad = (n: number) => String(n).padStart(2, '0')
  const time = `${pad(date.getHours())}:${pad(date.getMinutes())}:${pad(date.getSeconds())}`
  const day = String(date.getDate()).padStart(2, ' ')
  return `${DAYS[date.getDay()]} ${MONTHS[date.getMonth()]} ${day} ${time} ${date.getFullYear()}`
}

/**
 * ホームディレクトリの下のパスを, `~` で書きます.
 *
 * @param path - パス.
 * @param home - ホームディレクトリ (`/` で終わる).
 * @returns パス.
 */
function abbreviateHome(path: string, home: string): string {
  return path.startsWith(home) ? '~/' + path.slice(home.length) : path
}

/**
 * 結果のバッファの本文を作ります (Emacs の grep-mode と同じ形).
 *
 * @param directory - 探したディレクトリ (`/` で終わる).
 * @param home - ホームディレクトリ (`/` で終わる).
 * @param pattern - 正規表現.
 * @param matches - 合った行.
 * @param date - 探した時刻.
 * @returns 本文.
 */
export function grepBufferText(
  directory: string,
  home: string,
  pattern: string,
  matches: readonly GrepMatch[],
  date: Date,
): string {
  const count =
    matches.length === 0
      ? 'no matches'
      : matches.length === 1
        ? '1 match'
        : `${matches.length} matches`
  return [
    `-*- mode: grep; default-directory: "${abbreviateHome(directory, home)}" -*-`,
    `Grep started at ${timeString(date)}`,
    '',
    `grep (built-in) -r -e ${pattern} .`,
    ...matches.map((m) => `${m.file}:${m.line}:${m.text}`),
    '',
    `Grep finished with ${count} found at ${timeString(date)}`,
    '',
  ].join('\n')
}

/** 結果のバッファの, 合った行の前の行の数 (最初の合った行は, この次の行). */
export const GREP_HEADER_LINES = 4

/**
 * 結果のバッファの行の番号から, 合った行を返します.
 *
 * @param matches - 合った行.
 * @param line - バッファの行の番号 (1 から).
 * @returns 合った行の添え字. 合った行の行でなければ `null`.
 */
export function grepMatchIndex(matches: readonly GrepMatch[], line: number): number | null {
  const index = line - GREP_HEADER_LINES - 1
  return index >= 0 && index < matches.length ? index : null
}

/**
 * 本文の行の始めの位置を返します.
 *
 * @param text - 本文.
 * @param line - 行の番号 (1 から. 行の数より大きければ, 最後の行).
 * @returns 位置 (0 から).
 */
export function lineStart(text: string, line: number): number {
  let pos = 0
  for (let n = 1; n < line; n++) {
    const next = text.indexOf('\n', pos)
    if (next === -1) break
    pos = next + 1
  }
  return pos
}
