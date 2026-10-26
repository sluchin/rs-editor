/**
 * ファイルのコマンド. バッファをファイルに書くこと (Lisp の save-buffer などが使う), 読み直す
 * (`revert-buffer`, `recover-this-file`) と, 自動保存 (`#名前#`), バックアップ (`名前~`) を受け持ちます.
 */

import { useEffect, useRef } from 'react'
import type { Text } from '@codemirror/state'
import { type Buffer, bufferNameFromPath } from '../lib/buffer'
import {
  backupFile,
  deleteFile,
  fileNewerThan,
  pathExists,
  readFile,
  writeFile,
} from '../lib/fileOps'
import { autoSaveFileName } from '../lib/paths'
import type { AppContext } from './useBuffers'
import type { CommandSet } from './useMinibuffer'

/** 自動保存するまでの, 書き換えの無い時間 (Emacs の `auto-save-timeout`. ミリ秒). */
export const AUTO_SAVE_TIMEOUT = 30000

/**
 * ファイルのコマンドを作ります.
 *
 * @param ctx - ミニバッファと, バッファとウィンドウ.
 * @param listDirectoryInto - dired のバッファの一覧を作り直す関数 (dired の `revert-buffer`).
 * @returns コマンドと, バッファをファイルに書く関数.
 */
export function useFiles(
  ctx: AppContext,
  listDirectoryInto: (path: string, revert: boolean) => Promise<void>,
) {
  const {
    buffers,
    setBuffers,
    currentBuffer,
    setCurrentText,
    setPendingAction,
    setMinibufferState,
    showMessage,
  } = ctx
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

  /**
   * バッファを, ファイルから読み直します (M-x revert-buffer). 読み直してよいか, 聞きます.
   * dired のバッファは, 聞かずに, 一覧を作り直します.
   */
  const handleRevertBuffer = () => {
    if (currentBuffer.directory !== undefined) {
      void listDirectoryInto(currentBuffer.directory, true)
      return
    }
    if (!currentBuffer.filePath) {
      showMessage('Buffer does not seem to be associated with any file')
      return
    }
    setPendingAction('confirm-revert')
    setMinibufferState({
      mode: 'input',
      prompt: currentBuffer.modified
        ? `Discard edits and reread from ${currentBuffer.filePath}? (yes or no) `
        : `Revert buffer from file ${currentBuffer.filePath}? (yes or no) `,
      input: '',
    })
  }

  /**
   * バッファを, ファイルから読み直します.
   *
   * @param path - ファイルのパス.
   */
  const revertBuffer = async (path: string) => {
    try {
      setCurrentText(await readFile(path), false)
      autoSaved.current.delete(currentBuffer.id)
      showMessage('')
    } catch (err) {
      showMessage(`Error: ${err}`)
    }
  }

  /**
   * 自動保存のファイルから, バッファを読み直します (M-x recover-this-file). 読み直してよいか, 聞きます.
   */
  const handleRecoverThisFile = async () => {
    const path = currentBuffer.filePath
    if (!path) {
      showMessage('This buffer is not visiting a file')
      return
    }
    const autoSave = autoSaveFileName(path)
    try {
      if ((await pathExists(path)) && !(await pathExists(autoSave))) {
        showMessage(`Auto save file ${autoSave} does not exist`)
        return
      }
      if (!(await fileNewerThan(autoSave, path))) {
        showMessage(`Auto-save file ${autoSave} not current`)
        return
      }
    } catch (err) {
      showMessage(`Error: ${err}`)
      return
    }
    setPendingAction('confirm-recover')
    setMinibufferState({
      mode: 'input',
      prompt: `Recover auto save file ${autoSave}? (yes or no) `,
      input: '',
    })
  }

  /**
   * 自動保存のファイルから, バッファを読み直します (変更ありにする).
   *
   * @param path - ファイルのパス.
   */
  const recoverFile = async (path: string) => {
    try {
      setCurrentText(await readFile(autoSaveFileName(path)), true)
      showMessage('')
    } catch (err) {
      showMessage(`Error: ${err}`)
    }
  }

  const commands: CommandSet = {
    actions: {
      'revert-buffer': handleRevertBuffer,
      'recover-this-file': () => void handleRecoverThisFile(),
    },
    submit: {
      'confirm-revert': async (value) => {
        if (value.trim() === 'yes') await revertBuffer(currentBuffer.filePath as string)
        else showMessage('')
      },
      'confirm-recover': async (value) => {
        if (value.trim() === 'yes') await recoverFile(currentBuffer.filePath as string)
        else showMessage('')
      },
    },
  }

  return { commands, saveBuffer }
}
