/**
 * ディレクトリのコマンド. dired (C-x d), `M-x grep` と, その結果からファイルを開くこと (RET, M-g n, M-g p),
 * サイドバーのプロジェクトのツリー (F8) を受け持ちます.
 */

import { useRef, useState } from 'react'
import type { EditorHandle } from '../components/CodeEditor'
import { type Buffer, generateBufferId, toText, visitFile } from '../lib/buffer'
import { diredBufferName, diredListing } from '../lib/dired'
import { grepDirectory, listDirectoryLong, projectRoot } from '../lib/fileOps'
import { GREP_BUFFER_NAME, grepBufferText, grepMatchIndex } from '../lib/grep'
import type { KeyBinding } from '../lib/lisp'
import { directoryFileName } from '../lib/paths'
import type { AppContext } from './useBuffers'
import { type CommandSet, completeFile } from './useMinibuffer'

/** `M-x grep` の結果のバッファのキー (Emacs の grep-mode). */
const GREP_KEYS: KeyBinding[] = [{ key: 'Enter', command: 'compile-goto-error' }]

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
    currentBufferId,
    editor,
    homeDir,
    setCurrentBufferId,
    setCurrentText,
    defaultDirectory,
    replaceBuffer,
    displayInOtherWindow,
    visitLocation,
    setPendingAction,
    setMinibufferState,
    showMessage,
  } = ctx
  // 探す正規表現 (M-x grep で, ディレクトリを聞いているあいだ) と, next-error で出した, 合った行の添え字 (`*grep*` は 1 つ)
  const grepPattern = useRef('')
  const nextErrorIndex = useRef(-1)
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
   * 探す正規表現を, ミニバッファで聞きます (M-x grep). 続けて, 探すディレクトリを聞きます.
   */
  const handleGrep = () => {
    setPendingAction('grep')
    setMinibufferState({ mode: 'input', prompt: 'Grep (regexp): ', input: '' })
  }

  /**
   * 探す正規表現を覚えて, 探すディレクトリを聞きます. 空なら, やめます.
   *
   * @param value - 入力した正規表現.
   */
  const askGrepDirectory = (value: string) => {
    if (value === '') {
      showMessage('')
      return
    }
    grepPattern.current = value
    setPendingAction('grep-directory')
    setMinibufferState({
      mode: 'input',
      prompt: 'In directory: ',
      input: defaultDirectory(currentBuffer),
      complete: completeFile,
    })
  }

  /**
   * ディレクトリの下のファイルから, 正規表現に合う行を探して, `*grep*` バッファに出します
   * (ほかのウィンドウに出し, 選んでいるウィンドウは変えない. Emacs と同じ). 探すのは, エディタ (Rust) が行います.
   *
   * @param pattern - 正規表現.
   * @param dir - ディレクトリ.
   */
  const runGrep = async (pattern: string, dir: string) => {
    const directory = dir.endsWith('/') ? dir : dir + '/'
    try {
      const matches = await grepDirectory(pattern, directory)
      const content = grepBufferText(directory, homeDir, pattern, matches, new Date())
      const buffer: Buffer = {
        id: generateBufferId(),
        name: GREP_BUFFER_NAME,
        filePath: null,
        doc: toText(content),
        modified: false,
        keys: GREP_KEYS,
        grep: { directory, matches },
      }
      const old = buffers.find((b) => b.grep)
      if (old) replaceBuffer(old.id, buffer)
      else setBuffers((prev) => [...prev, buffer])
      nextErrorIndex.current = -1
      // 選んでいるウィンドウに前の結果を出していれば, そこで置き換える
      if (old?.id !== currentBufferId) displayInOtherWindow(buffer.id, false)
      const count = matches.length === 1 ? '1 match' : `${matches.length || 'no'} matches`
      showMessage(`Grep finished with ${count} found`)
    } catch (err) {
      showMessage(`Error: ${err}`)
    }
  }

  /**
   * `*grep*` バッファの, 合った行のファイルを開きます.
   *
   * @param buffer - `*grep*` バッファ.
   * @param index - 合った行の添え字.
   */
  const gotoGrepMatch = async (buffer: Buffer, index: number) => {
    const { directory, matches } = buffer.grep as NonNullable<Buffer['grep']>
    const match = matches[index]
    nextErrorIndex.current = index
    try {
      // 結果のバッファから開くときは, ほかのウィンドウに出す
      await visitLocation(directory + match.file, match.line, currentBufferId === buffer.id)
      showMessage('')
    } catch (err) {
      showMessage(`Error: ${err}`)
    }
  }

  /**
   * カーソルの行の, 合った行のファイルを開きます (`*grep*` の RET. Emacs の `compile-goto-error`).
   */
  const handleCompileGotoError = () => {
    const handle = editor.current as EditorHandle
    const line = handle.text().slice(0, handle.point()).split('\n').length
    const index = currentBuffer.grep ? grepMatchIndex(currentBuffer.grep.matches, line) : null
    if (index === null) {
      showMessage('No error here')
      return
    }
    void gotoGrepMatch(currentBuffer, index)
  }

  /**
   * `*grep*` の, 次 (前) の合った行のファイルを開きます (M-g n, M-g p. Emacs の `next-error`).
   *
   * @param delta - 進む数 (前は -1).
   */
  const handleNextError = (delta: number) => {
    const buffer = buffers.find((b) => b.grep)
    if (!buffer) {
      showMessage('No buffers contain error message locations')
      return
    }
    const index = nextErrorIndex.current + delta
    const { matches } = buffer.grep as NonNullable<Buffer['grep']>
    if (index < 0) {
      showMessage('Moved back before first grep hit')
    } else if (index >= matches.length) {
      showMessage('Moved past last grep hit')
    } else {
      void gotoGrepMatch(buffer, index)
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
      grep: handleGrep,
      'compile-goto-error': handleCompileGotoError,
      'next-error': () => handleNextError(1),
      'previous-error': () => handleNextError(-1),
      'project-tree': () => void handleProjectTree(),
    },
    submit: {
      dired: (value) => listDirectoryInto(value),
      grep: askGrepDirectory,
      'grep-directory': (value) => runGrep(grepPattern.current, value),
    },
  }

  return { commands, treeRoot, openFromTree, listDirectoryInto }
}
