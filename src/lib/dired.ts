/**
 * Dired (ディレクトリの一覧. `C-x d`). 一覧は, Emacs 30 と同じく `ls -al` で作ります.
 * エディタ (CodeMirror) に依存しないので, 単体でテストできます.
 */

import { directoryFileName } from './paths'

/**
 * dired のバッファの名前を返します (ディレクトリの名前. `/` は, そのまま).
 *
 * @param dir - ディレクトリのパス.
 * @returns バッファの名前.
 */
export function diredBufferName(dir: string): string {
  const path = directoryFileName(dir)
  return path === '/' ? path : path.slice(path.lastIndexOf('/') + 1)
}

/**
 * dired のバッファの本文を作ります. 始めの行にディレクトリを出し, `ls` の各行を 2 文字の字下げで並べます.
 * `total` の行は除きます (Emacs 30 の `dired-free-space` が `first` のとき).
 *
 * @param dir - ディレクトリのパス.
 * @param listing - `ls -al` の出力.
 * @returns 本文.
 */
export function diredListing(dir: string, listing: string): string {
  const lines = listing.split('\n').filter((line) => line !== '' && !/^total /.test(line))
  return [`${directoryFileName(dir)}:`, ...lines].map((line) => `  ${line}\n`).join('')
}
