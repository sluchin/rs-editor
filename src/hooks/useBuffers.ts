/**
 * バッファとウィンドウ. 開いているバッファ, ウィンドウの分け方, 各ウィンドウのエディタとカーソルを持ち,
 * ウィンドウのコマンド (C-x 2, C-x 3, C-x o, C-x 0, C-x 1) を受け持ちます.
 */

import { type SetStateAction, useCallback, useEffect, useRef, useState } from 'react'
import type { CursorInfo, DocChange, EditorHandle } from '../components/CodeEditor'
import {
  type Buffer,
  createScratchBuffer,
  generateBufferId,
  toText,
  uniquifyBufferNames,
  visitFile,
} from '../lib/buffer'
import { getHomeDir } from '../lib/fileOps'
import { lineStart } from '../lib/grep'
import { fileDirectory } from '../lib/paths'
import { type Layout, deleteWindow, splitWindow, windowIds } from '../lib/windows'
import type { CommandSet, MinibufferContext } from './useMinibuffer'

/** 始めのウィンドウ. */
const FIRST_WINDOW = 'window-1'

/** カーソルの位置の始め. */
export const START_CURSOR: CursorInfo = { line: 1, column: 1, offset: 0 }

/**
 * ウィンドウの ID を作ります.
 *
 * @returns ID.
 */
function generateWindowId(): string {
  return `window-${Date.now()}-${Math.random().toString(36).slice(2, 11)}`
}

/**
 * バッファとウィンドウを持ちます.
 *
 * @param showMessage - ミニバッファに, メッセージを出す関数.
 * @returns 状態と, 操作の関数と, ウィンドウのコマンド.
 */
export function useBuffers(showMessage: (text: string) => void) {
  // 開いているバッファ (開いた順)
  const [buffers, setBuffersRaw] = useState<Buffer[]>(() => [createScratchBuffer()])
  // バッファを変えるたびに, 同じ名前のファイルのバッファの名前を, ディレクトリで区別する (Emacs の uniquify)
  const setBuffers = useCallback(
    (update: SetStateAction<Buffer[]>) =>
      setBuffersRaw((prev) =>
        uniquifyBufferNames(typeof update === 'function' ? update(prev) : update),
      ),
    [],
  )
  // ウィンドウの分け方と, 各ウィンドウに出すバッファと, カーソルの位置
  const [layout, setLayout] = useState<Layout>({ kind: 'window', id: FIRST_WINDOW })
  const [windowBuffers, setWindowBuffers] = useState<Record<string, string>>({
    [FIRST_WINDOW]: buffers[0].id,
  })
  const [cursors, setCursors] = useState<Record<string, CursorInfo>>({})
  const [activeWindow, setActiveWindow] = useState(FIRST_WINDOW)
  // ホームディレクトリ (ファイルの名前の入力の始め. `/` で終わる)
  const [homeDir, setHomeDir] = useState('/')
  // ウィンドウごとのエディタ
  const handles = useRef(new Map<string, EditorHandle>())
  // 選んでいるウィンドウ (描くのを待たずに, すぐ使えるように)
  const activeWindowRef = useRef(FIRST_WINDOW)
  // 選んでいるウィンドウのエディタ
  const [editor] = useState(() => ({
    get current(): EditorHandle | undefined {
      return handles.current.get(activeWindowRef.current)
    },
  }))
  // バッファを出したら置く, カーソルの位置 (ブックマークへ移るとき)
  const pendingPoint = useRef<number | null>(null)

  // 選んでいるウィンドウのバッファと, カーソルの位置
  const currentBufferId = windowBuffers[activeWindow]
  const cursor = cursors[activeWindow] ?? START_CURSOR
  // バッファを消すときは, ウィンドウのバッファも一緒に変えるので, 必ず見つかる
  const currentBuffer = buffers.find((b) => b.id === currentBufferId) as Buffer

  // ホームディレクトリを, 始めに 1 回だけ取る (取れなければ, `/` のまま)
  useEffect(() => {
    getHomeDir()
      .then((dir) => setHomeDir(dir.endsWith('/') ? dir : dir + '/'))
      .catch(() => {})
  }, [])

  // ブックマークへ移るときは, バッファを出してから, カーソルを置く
  useEffect(() => {
    if (pendingPoint.current === null) return
    editor.current?.setCursor(pendingPoint.current)
    pendingPoint.current = null
  }, [currentBufferId, editor])

  /**
   * バッファを出したら置く, カーソルの位置を決めます (次にバッファを出したときに置く).
   *
   * @param pos - カーソルの位置.
   */
  const setPendingPoint = (pos: number) => {
    pendingPoint.current = pos
  }

  /**
   * ウィンドウを選びます.
   *
   * @param id - ウィンドウ.
   */
  const selectWindow = (id: string) => {
    activeWindowRef.current = id
    setActiveWindow(id)
  }

  /**
   * 選んでいるウィンドウに, バッファを出します.
   *
   * @param bufferId - バッファ.
   */
  const setCurrentBufferId = (bufferId: string) => {
    setWindowBuffers((prev) => ({ ...prev, [activeWindowRef.current]: bufferId }))
  }

  /**
   * 選んでいるウィンドウのバッファを, 書き換えます.
   *
   * @param updates - 書き換える項目.
   */
  const updateCurrentBuffer = (updates: Partial<Buffer>) => {
    setBuffers((prev) => prev.map((b) => (b.id === currentBufferId ? { ...b, ...updates } : b)))
  }

  /**
   * 選んでいるウィンドウのバッファの本文を, 置き換えます.
   *
   * @param content - 本文.
   * @param modified - 置き換えた後に, 変更ありにするなら `true`.
   */
  const setCurrentText = (content: string, modified: boolean) => {
    const handle = editor.current as EditorHandle
    handle.setText(content)
    updateCurrentBuffer({ doc: handle.doc(), modified })
  }

  /**
   * バッファを作って, 選んでいるウィンドウに出します. 作ったばかりのバッファは, 変更なしです
   * (開いたファイルの中身は, ファイルと同じ).
   *
   * @param name - バッファの名前.
   * @param filePath - バッファのファイル.
   * @param content - 本文.
   * @returns 作ったバッファ.
   */
  const createNewBuffer = (name: string, filePath: string | null = null, content: string = '') => {
    const newBuffer: Buffer = {
      id: generateBufferId(),
      name,
      filePath,
      doc: toText(content),
      modified: false,
    }
    setBuffers((prev) => [...prev, newBuffer])
    setCurrentBufferId(newBuffer.id)
    return newBuffer
  }

  /**
   * バッファの既定のディレクトリを返します (Emacs の `default-directory`). ファイルのディレクトリ,
   * dired のディレクトリ, またはホームディレクトリです.
   *
   * @param buffer - バッファ.
   * @returns ディレクトリ (`/` で終わる).
   */
  const defaultDirectory = (buffer: Buffer): string => {
    if (buffer.directory !== undefined) {
      return buffer.directory.endsWith('/') ? buffer.directory : buffer.directory + '/'
    }
    return buffer.filePath ? fileDirectory(buffer.filePath) : homeDir
  }

  /**
   * 選んでいるウィンドウのバッファを消します (保存していなくても消す). 全て消したら, *scratch* を作り直します.
   */
  const killCurrentBuffer = () => {
    for (const handle of handles.current.values()) handle.forget(currentBuffer.id)
    const newBuffers = buffers.filter((b) => b.id !== currentBuffer.id)
    // 全て消したら, *scratch* を作り直す
    const next = newBuffers.length === 0 ? createScratchBuffer() : newBuffers[0]
    setBuffers(newBuffers.length === 0 ? [next] : newBuffers)
    // 消したバッファを出していたウィンドウには, 別のバッファを出す
    setWindowBuffers((prev) =>
      Object.fromEntries(
        Object.entries(prev).map(([w, b]) => [w, b === currentBuffer.id ? next.id : b]),
      ),
    )
  }

  /**
   * バッファを, 別のバッファに置き換えます (出しているウィンドウも).
   *
   * @param old - 置き換えるバッファ.
   * @param next - 新しいバッファ.
   */
  const replaceBuffer = (old: string, next: Buffer) => {
    for (const handle of handles.current.values()) handle.forget(old)
    setBuffers((prev) => prev.map((b) => (b.id === old ? next : b)))
    setWindowBuffers((prev) =>
      Object.fromEntries(Object.entries(prev).map(([w, b]) => [w, b === old ? next.id : b])),
    )
  }

  /**
   * ウィンドウで書き換えた本文を, バッファに覚えます. 同じバッファを出している別のウィンドウにも写します.
   * 本文は, エディタの `Text` をそのまま覚え, 別のウィンドウには, 変更だけを写します (本文の全体を写さない).
   *
   * @param windowId - 書き換えたウィンドウ.
   * @param change - 書き換え.
   */
  const handleEditorChange = (windowId: string, change: DocChange) => {
    const bufferId = windowBuffers[windowId]
    setBuffers((prev) =>
      prev.map((b) => (b.id === bufferId ? { ...b, doc: change.doc, modified: true } : b)),
    )
    for (const [w, b] of Object.entries(windowBuffers)) {
      if (w !== windowId && b === bufferId) handles.current.get(w)?.sync(change)
    }
  }

  /**
   * ほかのウィンドウに, バッファを出します. ウィンドウが 1 つなら, 下に分けます (Emacs の `display-buffer`).
   *
   * @param bufferId - バッファ.
   * @param select - そのウィンドウを選ぶなら `true`.
   */
  const displayInOtherWindow = (bufferId: string, select: boolean) => {
    const ids = windowIds(layout)
    let target: string
    if (ids.length === 1) {
      target = generateWindowId()
      setLayout((prev) => splitWindow(prev, activeWindowRef.current, target, 'below'))
    } else {
      target = ids[(ids.indexOf(activeWindowRef.current) + 1) % ids.length]
    }
    setWindowBuffers((prev) => ({ ...prev, [target]: bufferId }))
    if (select) selectWindow(target)
  }

  /**
   * ファイルを開いて (開いていれば, そのバッファを出して), 行の始めへ動きます.
   *
   * @param path - ファイルのパス.
   * @param line - 行の番号 (1 から).
   * @param other - ほかのウィンドウに出して, 選ぶなら `true`. `false` なら, 選んでいるウィンドウに出す.
   */
  const visitLocation = async (path: string, line: number, other: boolean) => {
    let buffer = buffers.find((b) => b.filePath === path)
    if (!buffer) {
      const visited = (await visitFile(path)).buffer
      setBuffers((prev) => [...prev, visited])
      buffer = visited
    }
    const pos = lineStart(buffer.doc.toString(), line)
    if (!other && buffer.id === currentBufferId) {
      editor.current?.setCursor(pos)
      return
    }
    // バッファを出してから, カーソルを置く
    setPendingPoint(pos)
    if (other) displayInOtherWindow(buffer.id, true)
    else setCurrentBufferId(buffer.id)
  }

  /**
   * 選んでいるウィンドウを分けます (新しいウィンドウにも, 同じバッファを出す).
   *
   * @param direction - 分ける向き.
   */
  const handleSplitWindow = (direction: 'below' | 'right') => {
    const id = generateWindowId()
    setLayout((prev) => splitWindow(prev, activeWindowRef.current, id, direction))
    setWindowBuffers((prev) => ({ ...prev, [id]: prev[activeWindowRef.current] }))
  }

  /**
   * 次のウィンドウを選びます (C-x o. 最後の次は, 最初).
   */
  const handleOtherWindow = () => {
    const ids = windowIds(layout)
    selectWindow(ids[(ids.indexOf(activeWindowRef.current) + 1) % ids.length])
  }

  /**
   * 選んでいるウィンドウを閉じます (C-x 0). ウィンドウが 1 つなら, 閉じずにメッセージを出します.
   */
  const handleDeleteWindow = () => {
    const next = deleteWindow(layout, activeWindowRef.current)
    if (!next) {
      showMessage('Attempt to delete minibuffer or sole ordinary window')
      return
    }
    setLayout(next)
    selectWindow(windowIds(next)[0])
  }

  /**
   * 選んでいるウィンドウのほかを, 全て閉じます (C-x 1).
   */
  const handleDeleteOtherWindows = () => {
    setLayout({ kind: 'window', id: activeWindowRef.current })
  }

  const windowCommands: CommandSet = {
    actions: {
      'split-window-below': () => handleSplitWindow('below'),
      'split-window-right': () => handleSplitWindow('right'),
      'other-window': handleOtherWindow,
      'delete-window': handleDeleteWindow,
      'delete-other-windows': handleDeleteOtherWindows,
    },
  }

  return {
    buffers,
    setBuffers,
    layout,
    windowBuffers,
    cursors,
    setCursors,
    activeWindow,
    homeDir,
    handles,
    editor,
    currentBufferId,
    cursor,
    currentBuffer,
    selectWindow,
    setPendingPoint,
    setCurrentBufferId,
    updateCurrentBuffer,
    setCurrentText,
    createNewBuffer,
    defaultDirectory,
    killCurrentBuffer,
    replaceBuffer,
    handleEditorChange,
    displayInOtherWindow,
    visitLocation,
    windowCommands,
  }
}

/** バッファとウィンドウの状態と, 操作の関数. */
export type BuffersContext = ReturnType<typeof useBuffers>

/** 機能ごとのフックに渡す, ミニバッファと, バッファとウィンドウの状態と, 操作の関数. */
export type AppContext = MinibufferContext & BuffersContext
