/**
 * バッファ (ファイルを開いたもの, または *scratch* などの, ファイルのないもの) と, バッファを作る関数.
 */

import type { GrepMatch } from './fileOps'
import { fileNewerThan, pathExists, readFile } from './fileOps'
import type { KeyBinding } from './lisp'
import { autoSaveFileName } from './paths'

/** *scratch* バッファの始めの本文 (Emacs と同じ). */
const INITIAL_TEXT = `;; This buffer is for text that is not saved, and for Lisp evaluation.
;; To create a file, visit it with C-x C-f and enter text in its buffer.
`

/** バッファ (ファイルを開いたもの, または *scratch* などの, ファイルのないもの). */
export interface Buffer {
  /** バッファを区別する ID. */
  id: string
  /** バッファの名前 (ファイルの名前, または `*scratch*` など). */
  name: string
  /** バッファのファイルのパス. ファイルのないバッファは `null`. */
  filePath: string | null
  /** 本文. */
  content: string
  /** 保存してから, 書き換えたら `true`. */
  modified: boolean
  /** 開いてから, ファイルのバックアップ (`<file>~`) を作ったら `true` (Emacs の `buffer-backed-up`). */
  backedUp?: boolean
  /** dired のバッファなら, そのディレクトリ (後ろに `/` は付けない). */
  directory?: string
  /** バッファだけのキー (Lisp の `local-set-key`. Lisp の割り当てより先に使う). */
  keys?: KeyBinding[]
  /** `M-x grep` の結果のバッファなら, 探したディレクトリ (`/` で終わる) と, 合った行. */
  grep?: { directory: string; matches: GrepMatch[] }
}

/**
 * バッファの ID を作ります.
 *
 * @returns ID.
 */
export function generateBufferId(): string {
  return `buffer-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`
}

/**
 * ファイルのパスから, バッファの名前 (ファイルの名前) を作ります.
 *
 * @param path - ファイルのパス.
 * @returns バッファの名前. パスが `/` で終わる場合は, パスのまま.
 */
export function bufferNameFromPath(path: string): string {
  const parts = path.split('/')
  return parts[parts.length - 1] || path
}

/**
 * *scratch* バッファを作ります.
 *
 * @returns *scratch* バッファ.
 */
export function createScratchBuffer(): Buffer {
  return {
    id: generateBufferId(),
    name: '*scratch*',
    filePath: null,
    content: INITIAL_TEXT,
    modified: false,
  }
}

/**
 * ファイルを読んで, バッファを作ります (まだ出さない). 自動保存のファイルの方が新しければ, そう知らせます.
 *
 * @param path - ファイルのパス.
 * @returns バッファと, 出すメッセージ. ファイルが無ければ, 空のバッファ.
 */
export async function visitFile(path: string): Promise<{ buffer: Buffer; message: string }> {
  const exists = await pathExists(path)
  const content = exists ? await readFile(path) : ''
  const name = bufferNameFromPath(path)
  const buffer: Buffer = {
    id: generateBufferId(),
    name,
    filePath: path,
    content,
    modified: false,
  }
  if (await fileNewerThan(autoSaveFileName(path), path)) {
    return { buffer, message: `${name} has auto save data; consider M-x recover-this-file` }
  }
  return { buffer, message: exists ? '' : '(New file)' }
}
