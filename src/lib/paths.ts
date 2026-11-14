/**
 * ファイルのパスの計算 (ディレクトリ).
 * エディタ (CodeMirror) に依存しないので単体でテストできます.
 */

/**
 * パスのディレクトリを返します (Emacs の `file-name-directory`).
 *
 * @param path - パス.
 * @returns ディレクトリ (`/` で終わる). パスに `/` が無ければ空.
 */
export function fileDirectory(path: string): string {
  return path.slice(0, path.lastIndexOf('/') + 1)
}
