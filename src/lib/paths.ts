/**
 * ファイルのパスの計算 (ディレクトリ, 自動保存のファイル).
 * エディタ (CodeMirror) に依存しないので, 単体でテストできます.
 */

/**
 * パスのディレクトリを返します (Emacs の `file-name-directory`).
 *
 * @param path - パス.
 * @returns ディレクトリ (`/` で終わる). パスに `/` が無ければ, 空.
 */
export function fileDirectory(path: string): string {
  return path.slice(0, path.lastIndexOf('/') + 1)
}

/**
 * ディレクトリのパスの, 後ろの `/` を除きます (Emacs の `directory-file-name`). `/` は, そのままです.
 *
 * @param path - ディレクトリのパス.
 * @returns 後ろの `/` を除いたパス.
 */
export function directoryFileName(path: string): string {
  const trimmed = path.replace(/\/+$/, '')
  return trimmed === '' && path !== '' ? '/' : trimmed
}

/**
 * 自動保存のファイルのパスを返します (Emacs の `make-auto-save-file-name`. 同じディレクトリの `#名前#`).
 *
 * @param path - ファイルのパス.
 * @returns 自動保存のファイルのパス.
 */
export function autoSaveFileName(path: string): string {
  const dir = fileDirectory(path)
  return `${dir}#${path.slice(dir.length)}#`
}
