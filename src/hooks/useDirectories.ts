/**
 * ディレクトリのコマンド. dired (C-x d) と,
 * サイドバーのプロジェクトのツリー (F8) を受け持ちます.
 */

import { useState } from 'react'
import { type Buffer, generateBufferId, toText, visitFile } from '../lib/buffer'
import { diredBufferName, diredListing } from '../lib/dired'
import { listDirectoryLong, projectRoot } from '../lib/fileOps'
import { directoryFileName } from '../lib/paths'
import type { AppContext } from './useBuffers'
import { type CommandSet, completeFile } from './useMinibuffer'

/**
 * ディレクトリのコマンドを作ります.
 *
 * @param ctx - ミニバッファと, バッファとウィンドウ.
 * @returns コマンドと, ツリーの根と, ほかのフックが使う関数.
 */
export function useDirectories(ctx: AppContext) {
  const {
    buffers,
    setBuffers,
    currentBuffer,
    setCurrentBufferId,
    setCurrentText,
    defaultDirectory,
    setPendingAction,
    setMinibufferState,
    showMessage,
  } = ctx
  // サイドバーのプロジェクトのツリーの根 (出していなければ null)
  const [treeRoot, setTreeRoot] = useState<string | null>(null)

  /**
   * 開くディレクトリの名前を, ミニバッファで聞きます (C-x d).
   */
  const handleDired = () => {
    setPendingAction('dired')
    setMinibufferState({
      mode: 'input',
      prompt: 'Dired (directory): ',
      input: defaultDirectory(currentBuffer),
      complete: completeFile,
    })
  }

  /**
   * ディレクトリの一覧を, dired のバッファに出します (同じディレクトリの dired があれば, それを出す).
   *
   * @param path - ディレクトリのパス.
   * @param revert - 選んでいる dired のバッファを, 作り直すなら `true`.
   */
  const listDirectoryInto = async (path: string, revert = false) => {
    const dir = directoryFileName(path)
    const open = buffers.find((b) => b.directory === dir)
    if (open && !revert) {
      setCurrentBufferId(open.id)
      showMessage('')
      return
    }
    try {
      const content = diredListing(dir, await listDirectoryLong(dir))
      if (revert) {
        setCurrentText(content, false)
      } else {
        const buffer: Buffer = {
          id: generateBufferId(),
          name: diredBufferName(dir),
          filePath: null,
          doc: toText(content),
          modified: false,
          directory: dir,
        }
        setBuffers((prev) => [...prev, buffer])
        setCurrentBufferId(buffer.id)
      }
      showMessage('')
    } catch (err) {
      showMessage(`Error: ${err}`)
    }
  }

  /**
   * サイドバーのプロジェクトのツリーを, 出すか, 消します (F8). 根は, バッファのディレクトリから上へたどって,
   * `.git` のある所 (無ければ, バッファのディレクトリ) です.
   */
  const handleProjectTree = async () => {
    if (treeRoot !== null) {
      setTreeRoot(null)
      return
    }
    try {
      const root = await projectRoot(defaultDirectory(currentBuffer))
      setTreeRoot(root.replace(/\/+$/, '') || '/')
    } catch (err) {
      showMessage(`Error: ${err}`)
    }
  }

  /**
   * ツリーで選んだファイルを, 選んでいるウィンドウに出します (開いていれば, そのバッファ).
   *
   * @param path - ファイルのパス.
   */
  const openFromTree = async (path: string) => {
    const open = buffers.find((b) => b.filePath === path)
    if (open) {
      setCurrentBufferId(open.id)
      return
    }
    try {
      const { buffer, message } = await visitFile(path)
      setBuffers((prev) => [...prev, buffer])
      setCurrentBufferId(buffer.id)
      showMessage(message)
    } catch (err) {
      showMessage(`Error: ${err}`)
    }
  }

  const commands: CommandSet = {
    actions: {
      dired: handleDired,
      'project-tree': () => void handleProjectTree(),
    },
    submit: {
      dired: (value) => listDirectoryInto(value),
    },
  }

  return { commands, treeRoot, openFromTree, listDirectoryInto }
}
