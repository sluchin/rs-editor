/**
 * ファイルのコマンドが使うこと. バッファをファイルに書くこと (Lisp の save-buffer などが使う) と,
 * 自動保存 (`#名前#`), バックアップ (`名前~`) を受け持ちます. コマンドは Lisp (`lisp/files.scm`) で書きます.
 */

import { useEffect, useRef } from 'react'
import type { Text } from '@codemirror/state'
import { type Buffer, bufferNameFromPath } from '../lib/buffer'
import { backupFile, deleteFile, writeFile } from '../lib/fileOps'
import { autoSaveFileName } from '../lib/paths'
import type { AppContext } from './useBuffers'

/** 自動保存するまでの, 書き換えの無い時間 (Emacs の `auto-save-timeout`. ミリ秒). */
export const AUTO_SAVE_TIMEOUT = 30000

/**
 * ファイルのコマンドを作ります.
 *
 * @param ctx - ミニバッファと, バッファとウィンドウ.
 * @returns バッファをファイルに書く関数.
 */
export function useFiles(ctx: AppContext) {
  const { buffers, setBuffers, setMinibufferState, showMessage } = ctx
  // バッファごとの, 最後に自動保存した本文 (本文は変えられないので, 同じものかで比べる)
  const autoSaved = useRef(new Map<string, Text>())

  // 書き換えが止まってから, しばらくしたら, 変更したファイルのバッファを自動保存する (`#名前#`)
  useEffect(() => {
    const timer = setTimeout(() => {
      const targets = buffers.filter(
        (b) => b.filePath && b.modified && autoSaved.current.get(b.id) !== b.doc,
      )
      if (targets.length === 0) return
      void Promise.all(
        targets.map(async (b) => {
          await writeFile(autoSaveFileName(b.filePath as string), b.doc.toString())
          autoSaved.current.set(b.id, b.doc)
        }),
      )
        .then(() =>
          setMinibufferState((prev) =>
            prev.mode === 'message' ? { mode: 'message', text: 'Auto-saving...done' } : prev,
          ),
        )
        .catch((err) => setMinibufferState({ mode: 'message', text: `Error: ${err}` }))
    }, AUTO_SAVE_TIMEOUT)
    return () => clearTimeout(timer)
  }, [buffers, setMinibufferState])

  /**
   * バッファを, ファイルに保存します. 開いてから始めて保存するときは, 前の版をバックアップ (`<file>~`) にします.
   * 保存したら, 自動保存のファイルを消します.
   *
   * @param buffer - バッファ.
   * @param path - ファイルのパス (バッファのファイルと違えば, そのファイルのバッファにする).
   * @throws 書けないとき.
   */
  const saveBuffer = async (buffer: Buffer, path: string) => {
    // バックアップを作れなくても, 保存はする
    if (!buffer.backedUp || buffer.filePath !== path) await backupFile(path).catch(() => {})
    await writeFile(path, buffer.doc.toString())
    if (buffer.filePath) void deleteFile(autoSaveFileName(buffer.filePath)).catch(() => {})
    autoSaved.current.delete(buffer.id)
    const name = buffer.filePath === path ? buffer.name : bufferNameFromPath(path)
    // 書いているあいだに書き換えた分は, 保存していないので, 変更ありのままにする
    setBuffers((prev) =>
      prev.map((b) =>
        b.id === buffer.id
          ? {
              ...b,
              filePath: path,
              name,
              modified: b.modified && !b.doc.eq(buffer.doc),
              backedUp: true,
              saveCount: (b.saveCount ?? 0) + 1,
            }
          : b,
      ),
    )
    showMessage(`Wrote ${path}`)
  }

  return { saveBuffer }
}
