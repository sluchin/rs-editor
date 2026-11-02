/**
 * サイドバーのプロジェクトのツリー (F8) を受け持ちます. dired (C-x d) は, Lisp (`lisp/dired.scm`) です.
 */

import { useState } from 'react'
import { visitFile } from '../lib/buffer'
import { projectRoot } from '../lib/fileOps'
import type { AppContext } from './useBuffers'
import type { CommandSet } from './useMinibuffer'

/**
 * ディレクトリのコマンドを作ります.
 *
 * @param ctx - ミニバッファと, バッファとウィンドウ.
 * @returns コマンドと, ツリーの根と, ツリーで選んだファイルを開く関数.
 */
export function useDirectories(ctx: AppContext) {
  const { buffers, setBuffers, currentBuffer, setCurrentBufferId, defaultDirectory, showMessage } =
    ctx
  // サイドバーのプロジェクトのツリーの根 (出していなければ null)
  const [treeRoot, setTreeRoot] = useState<string | null>(null)

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
      'project-tree': () => void handleProjectTree(),
    },
  }

  return { commands, treeRoot, openFromTree }
}
