/**
 * ファイルのコマンド. 開く (C-x C-f, C-x C-v, C-x i), 保存する (C-x C-s, C-x C-w, C-x s), 読み直す
 * (`revert-buffer`, `recover-this-file`) と, 自動保存 (`#名前#`), バックアップ (`名前~`) を受け持ちます.
 */

import { useEffect, useRef, useState } from 'react'
import type { EditorHandle } from '../components/CodeEditor'
import { type Buffer, bufferNameFromPath, visitFile } from '../lib/buffer'
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
import { type CommandSet, completeFile } from './useMinibuffer'

/** 自動保存するまでの, 書き換えの無い時間 (Emacs の `auto-save-timeout`. ミリ秒). */
export const AUTO_SAVE_TIMEOUT = 30000

/**
 * ファイルのコマンドを作ります.
 *
 * @param ctx - ミニバッファと, バッファとウィンドウ.
 * @param listDirectoryInto - dired のバッファの一覧を作り直す関数 (dired の `revert-buffer`).
 * @returns コマンドと, 1 つのキーで答える問いと, やめるときの関数.
 */
export function useFiles(
  ctx: AppContext,
  listDirectoryInto: (path: string, revert: boolean) => Promise<void>,
) {
  const {
    buffers,
    setBuffers,
    currentBuffer,
    editor,
    homeDir,
    setCurrentBufferId,
    setCurrentText,
    defaultDirectory,
    replaceBuffer,
    setPendingAction,
    setMinibufferState,
    showMessage,
  } = ctx
  // 置き換えてよいか聞いている, ファイルのパス (C-x C-w で, ほかのファイルに書くとき)
  const [overwritePath, setOverwritePath] = useState<string | null>(null)
  // 聞いているあいだの, 開き直すファイル (C-x C-v)
  const alternatePath = useRef('')
  // 保存するか聞いているバッファ (C-x s) と, 保存した数
  const [saveQueue, setSaveQueue] = useState<{ buffers: Buffer[]; saved: number } | null>(null)
  // バッファごとの, 最後に自動保存した本文
  const autoSaved = useRef(new Map<string, string>())

  // 書き換えが止まってから, しばらくしたら, 変更したファイルのバッファを自動保存する (`#名前#`)
  useEffect(() => {
    const timer = setTimeout(() => {
      const targets = buffers.filter(
        (b) => b.filePath && b.modified && autoSaved.current.get(b.id) !== b.content,
      )
      if (targets.length === 0) return
      void Promise.all(
        targets.map(async (b) => {
          await writeFile(autoSaveFileName(b.filePath as string), b.content)
          autoSaved.current.set(b.id, b.content)
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
   * 開くファイルの名前を, ミニバッファで聞きます (C-x C-f).
   */
  const handleFindFile = () => {
    setPendingAction('find-file')
    setMinibufferState({
      mode: 'input',
      prompt: 'Find file: ',
      input: defaultDirectory(currentBuffer),
      complete: completeFile,
    })
  }

  /**
   * ファイルを開いて, 選んでいるウィンドウに出します. 開いていれば, そのバッファを出します (Emacs と同じ).
   *
   * @param path - ファイルのパス.
   */
  const findFile = async (path: string) => {
    const open = buffers.find((b) => b.filePath === path)
    if (open) {
      setCurrentBufferId(open.id)
      showMessage('')
      return
    }
    try {
      const { buffer, message } = await visitFile(path)
      setBuffers((prev) => [...prev, buffer])
      setCurrentBufferId(buffer.id)
      showMessage(message)
    } catch (err) {
      setMinibufferState({ mode: 'message', text: `Error: ${err}` })
    }
  }

  /**
   * バッファを, ファイルに保存します (C-x C-s). ファイルのないバッファは, 保存するファイルの名前を聞きます.
   */
  const handleSaveBuffer = async () => {
    if (currentBuffer.filePath) {
      await saveBuffer(currentBuffer, currentBuffer.filePath)
    } else {
      setPendingAction('write-file')
      setMinibufferState({
        mode: 'input',
        prompt: 'Save file: ',
        input: homeDir,
        complete: completeFile,
      })
    }
  }

  /**
   * 書くファイルの名前を, ミニバッファで聞きます (C-x C-w). 入力の始めは, バッファのファイルのディレクトリ
   * (ファイルのないバッファは, ホームディレクトリ).
   */
  const handleWriteFile = () => {
    const path = currentBuffer.filePath
    setPendingAction('write-file')
    setMinibufferState({
      mode: 'input',
      prompt: 'Write file: ',
      input: path ? path.slice(0, path.lastIndexOf('/') + 1) : homeDir,
      complete: completeFile,
    })
  }

  /**
   * 入力したファイルに, バッファを書きます. バッファのファイルでない, ほかのファイルを置き換えるときは,
   * 聞きます (Emacs と同じ).
   *
   * @param value - ファイルのパス.
   */
  const writeFileAsked = async (value: string) => {
    let exists: boolean
    try {
      exists = value !== currentBuffer.filePath && (await pathExists(value))
    } catch (err) {
      showMessage(`Error: ${err}`)
      return
    }
    if (exists) {
      setOverwritePath(value)
      setMinibufferState({
        mode: 'query',
        prompt: `File ‘${value}’ exists; overwrite? (y or n) `,
      })
    } else {
      await writeBufferTo(value)
    }
  }

  /**
   * バッファを, ファイルに書いて, そのファイルのバッファにします (バッファの名前も, ファイルの名前にする).
   *
   * @param path - ファイルのパス.
   */
  const writeBufferTo = async (path: string) => {
    await saveBuffer(currentBuffer, path)
  }

  /**
   * バッファを, ファイルに保存します. 開いてから始めて保存するときは, 前の版をバックアップ (`<file>~`) にします.
   * 保存したら, 自動保存のファイルを消します.
   *
   * @param buffer - バッファ.
   * @param path - ファイルのパス (バッファのファイルと違えば, そのファイルのバッファにする).
   * @returns 保存したら `true`.
   */
  const saveBuffer = async (buffer: Buffer, path: string): Promise<boolean> => {
    try {
      // バックアップを作れなくても, 保存はする
      if (!buffer.backedUp || buffer.filePath !== path) await backupFile(path).catch(() => {})
      await writeFile(path, buffer.content)
    } catch (err) {
      showMessage(`Error: ${err}`)
      return false
    }
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
              modified: b.modified && b.content !== buffer.content,
              backedUp: true,
            }
          : b,
      ),
    )
    showMessage(`Wrote ${path}`)
    return true
  }

  /**
   * ファイルを開いたバッファで, 選んでいるウィンドウのバッファを置き換えます (C-x C-v).
   *
   * @param path - ファイルのパス.
   */
  const findAlternateFile = async (path: string) => {
    try {
      const { buffer, message } = await visitFile(path)
      replaceBuffer(currentBuffer.id, buffer)
      showMessage(message)
    } catch (err) {
      showMessage(`Error: ${err}`)
    }
  }

  /**
   * 開き直すファイルの名前を, ミニバッファで聞きます (C-x C-v). 入力の始めは, バッファのファイルです.
   */
  const handleFindAlternateFile = () => {
    setPendingAction('find-alternate-file')
    setMinibufferState({
      mode: 'input',
      prompt: 'Find alternate file: ',
      input: currentBuffer.filePath ?? defaultDirectory(currentBuffer),
      complete: completeFile,
    })
  }

  /**
   * 入力したファイルで, バッファを置き換えます. 書き換えたファイルのバッファは, 捨ててよいか聞きます.
   *
   * @param value - ファイルのパス.
   */
  const findAlternateFileAsked = async (value: string) => {
    if (currentBuffer.modified && currentBuffer.filePath) {
      alternatePath.current = value
      setPendingAction('confirm-alternate')
      setMinibufferState({
        mode: 'input',
        prompt: `Kill and replace buffer ‘${currentBuffer.name}’ without saving it? (yes or no) `,
        input: '',
      })
    } else {
      await findAlternateFile(value)
    }
  }

  /**
   * 入れるファイルの名前を, ミニバッファで聞きます (C-x i).
   */
  const handleInsertFile = () => {
    setPendingAction('insert-file')
    setMinibufferState({
      mode: 'input',
      prompt: 'Insert file: ',
      input: defaultDirectory(currentBuffer),
      complete: completeFile,
    })
  }

  /**
   * ファイルの中身を, カーソルの位置に入れます. カーソルは, 入れた文字の前に残します.
   *
   * @param path - ファイルのパス.
   */
  const insertFile = async (path: string) => {
    try {
      const content = await readFile(path)
      const handle = editor.current as EditorHandle
      const pos = handle.point()
      handle.insert(content)
      handle.setCursor(pos)
      showMessage('')
    } catch (err) {
      showMessage(`Error: ${err}`)
    }
  }

  /**
   * 変更したファイルのバッファを, 1 つずつ保存するか聞きます (C-x s).
   */
  const handleSaveSomeBuffers = () => {
    const modified = buffers.filter((b) => b.modified && b.filePath)
    if (modified.length === 0) {
      showMessage('(No files need saving)')
      return
    }
    askSave(modified, 0)
  }

  /**
   * 保存するか聞くバッファの, 始めのバッファを聞きます. 無ければ, 終えます.
   *
   * @param queue - 聞くバッファ.
   * @param saved - 保存した数.
   */
  const askSave = (queue: Buffer[], saved: number) => {
    if (queue.length === 0) {
      setSaveQueue(null)
      if (saved === 0) showMessage('')
      return
    }
    setSaveQueue({ buffers: queue, saved })
    setMinibufferState({
      mode: 'query',
      prompt: `Save file ${queue[0].filePath}? (y, n, !, ., q) `,
    })
  }

  /**
   * C-x s の問いに答えます (y で保存, n で保存しない, ! で残りを全て保存, . で保存して終える, q で終える).
   *
   * @param key - 押したキー.
   * @param queue - 聞いているバッファと, 保存した数.
   */
  const answerSave = async (
    key: string,
    { buffers: queue, saved }: { buffers: Buffer[]; saved: number },
  ) => {
    const [first, ...rest] = queue
    const save = async (b: Buffer) => ((await saveBuffer(b, b.filePath as string)) ? 1 : 0)
    if (key === 'y' || key === ' ') {
      askSave(rest, saved + (await save(first)))
    } else if (key === 'n' || key === 'Backspace' || key === 'Delete') {
      askSave(rest, saved)
    } else if (key === '!') {
      let count = saved
      for (const b of queue) count += await save(b)
      askSave([], count)
    } else if (key === '.') {
      askSave([], saved + (await save(first)))
    } else if (key === 'q' || key === 'Enter') {
      askSave([], saved)
    }
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

  /**
   * 1 つのキーで答える問い (C-x s, ファイルを置き換えてよいか) に答えます.
   *
   * @param key - 押したキー.
   * @returns 問いに答えたら `true` (ほかの問いなら `false`).
   */
  const answerQuery = (key: string): boolean => {
    if (saveQueue !== null) {
      void answerSave(key, saveQueue)
      return true
    }
    // ファイルを置き換えてよいかの問い (y か n のほかのキーは, 答えにしない)
    if (overwritePath !== null) {
      if (key === 'y') {
        setOverwritePath(null)
        void writeBufferTo(overwritePath)
      } else if (key === 'n') {
        setOverwritePath(null)
        showMessage('Canceled')
      }
      return true
    }
    return false
  }

  /**
   * 聞いている問いを, やめます.
   */
  const cancel = () => {
    setOverwritePath(null)
    setSaveQueue(null)
  }

  const commands: CommandSet = {
    actions: {
      'find-file': handleFindFile,
      'save-buffer': () => void handleSaveBuffer(),
      'write-file': handleWriteFile,
      'insert-file': handleInsertFile,
      'find-alternate-file': handleFindAlternateFile,
      'save-some-buffers': handleSaveSomeBuffers,
      'revert-buffer': handleRevertBuffer,
      'recover-this-file': () => void handleRecoverThisFile(),
    },
    submit: {
      'find-file': findFile,
      'find-alternate-file': findAlternateFileAsked,
      'confirm-alternate': async (value) => {
        if (value.trim() === 'yes') await findAlternateFile(alternatePath.current)
        else showMessage('Aborted')
      },
      'confirm-revert': async (value) => {
        if (value.trim() === 'yes') await revertBuffer(currentBuffer.filePath as string)
        else showMessage('')
      },
      'confirm-recover': async (value) => {
        if (value.trim() === 'yes') await recoverFile(currentBuffer.filePath as string)
        else showMessage('')
      },
      'insert-file': insertFile,
      'write-file': writeFileAsked,
    },
  }

  return { commands, answerQuery, cancel }
}
