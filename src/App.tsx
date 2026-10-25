/**
 * アプリの画面. バッファとウィンドウを持ち, ファイル, バッファ, 検索, 置換, 評価のコマンドを受け持ちます.
 */

import { useState, useEffect, useRef } from 'react'
import type { UnlistenFn } from '@tauri-apps/api/event'
import { getCurrentWindow } from '@tauri-apps/api/window'
import CodeEditor, { CursorInfo, EditorHandle } from './components/CodeEditor'
import ModeLine from './components/ModeLine'
import Minibuffer, {
  type IsearchCommand,
  MinibufferInputState,
  MinibufferIsearchState,
  MinibufferState,
} from './components/Minibuffer'
import BufferList from './components/BufferList'
import {
  readFile,
  writeFile,
  pathExists,
  getHomeDir,
  listDirectory,
  backupFile,
  deleteFile,
  fileNewerThan,
  listDirectoryLong,
} from './lib/fileOps'
import { completeFileName } from './lib/complete'
import {
  type Bookmark,
  bookmarkPosition,
  loadBookmarks,
  makeBookmark,
  saveBookmarks,
  storeBookmark,
} from './lib/bookmarks'
import { DIRED_MODE, majorMode } from './lib/modes'
import { diredBufferName, diredListing } from './lib/dired'
import { autoSaveFileName, directoryFileName, fileDirectory } from './lib/paths'
import { compileRegexp, findNextMatch, quoteRegexp, yankWordOrCharEnd } from './lib/isearch'
import { HISTORY_LENGTH, SEARCH_RING_MAX, addToHistory } from './lib/history'
import {
  type ReplaceMatch,
  type ReplaceSpec,
  findReplaceMatch,
  replacedMessage,
  replaceRemaining,
} from './lib/replace'
import { backwardSexp, forwardSexp } from './lib/bracketMatch'
import { KEY_BINDINGS, toEmacsKey } from './lib/emacs'
import { type Layout, deleteWindow, splitWindow, windowIds, windowRects } from './lib/windows'
import {
  type EvalResponse,
  type KeyBinding,
  type LispHost,
  evalLisp,
  listenEditorRequests,
  loadInitFile,
  runLispCommand,
} from './lib/lisp'
import './App.css'

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
}

/** ミニバッファの入力を待っているコマンド (入力を確定したときに行う). 待っていなければ `null`. */
type PendingAction =
  | 'find-file'
  | 'write-file'
  | 'execute-command'
  | 'switch-buffer'
  | 'kill-buffer'
  | 'eval-expression'
  | 'goto-line'
  | 'confirm-exit'
  | 'replace-from'
  | 'replace-to'
  | 'string-rectangle'
  | 'bookmark-set'
  | 'bookmark-jump'
  | 'insert-file'
  | 'find-alternate-file'
  | 'confirm-alternate'
  | 'confirm-revert'
  | 'confirm-recover'
  | 'dired'
  | null

/** 入力を待っているコマンドと, 入力の履歴の名前 (同じ名前のコマンドは, 履歴を分け合う). */
const HISTORY_NAMES: Partial<Record<NonNullable<PendingAction>, string>> = {
  'find-file': 'file-name',
  'write-file': 'file-name',
  'insert-file': 'file-name',
  'find-alternate-file': 'file-name',
  dired: 'file-name',
  'execute-command': 'extended-command',
  'switch-buffer': 'buffer-name',
  'eval-expression': 'read-expression',
  'goto-line': 'goto-line',
  'replace-from': 'query-replace',
  'replace-to': 'query-replace',
  'string-rectangle': 'string-rectangle',
  'bookmark-set': 'bookmark',
  'bookmark-jump': 'bookmark',
  'confirm-exit': 'yes-or-no-p',
  'confirm-alternate': 'yes-or-no-p',
  'confirm-revert': 'yes-or-no-p',
  'confirm-recover': 'yes-or-no-p',
}

/** 置換の種類 (query-replace, replace-string と, その正規表現の版). */
interface ReplaceKind {
  /** 1 つずつ聞くなら `true`. */
  query: boolean
  /** 正規表現で探すなら `true`. */
  regexp: boolean
}

/** 進めている置換. */
interface ReplaceState {
  /** 置換の種類. */
  kind: ReplaceKind
  /** 探す文字 (入力したら). */
  from?: string
  /** 置換の指定と, 聞いている所 (query-replace で, 聞いているあいだ). */
  spec?: ReplaceSpec
  /** query-replace で, 聞いている所. */
  match?: ReplaceMatch
  /** 置き換えた数. */
  count: number
}

/**
 * ファイルの名前を補完します (`C-x C-f`, 保存するファイル).
 *
 * @param input - 入力 (パス).
 * @returns 補完した入力と, 候補.
 */
const completeFile = (input: string) => completeFileName(input, listDirectory)

/**
 * 置換の名前 (プロンプトに出す) を返します.
 *
 * @param kind - 置換の種類.
 * @returns 名前.
 */
function replaceTitle({ query, regexp }: ReplaceKind): string {
  if (query) return regexp ? 'Query replace regexp' : 'Query replace'
  return regexp ? 'Replace regexp' : 'Replace string'
}

/**
 * バッファの ID を作ります.
 *
 * @returns ID.
 */
function generateBufferId(): string {
  return `buffer-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`
}

/**
 * ファイルのパスから, バッファの名前 (ファイルの名前) を作ります.
 *
 * @param path - ファイルのパス.
 * @returns バッファの名前. パスが `/` で終わる場合は, パスのまま.
 */
function bufferNameFromPath(path: string): string {
  const parts = path.split('/')
  return parts[parts.length - 1] || path
}

/**
 * *scratch* バッファを作ります.
 *
 * @returns *scratch* バッファ.
 */
function createScratchBuffer(): Buffer {
  return {
    id: generateBufferId(),
    name: '*scratch*',
    filePath: null,
    content: INITIAL_TEXT,
    modified: false,
  }
}

/** 始めのウィンドウ. */
const FIRST_WINDOW = 'window-1'

/** 自動保存するまでの, 書き換えの無い時間 (Emacs の `auto-save-timeout`. ミリ秒). */
export const AUTO_SAVE_TIMEOUT = 30000

/** カーソルの位置の始め. */
const START_CURSOR: CursorInfo = { line: 1, column: 1, offset: 0 }

/**
 * ウィンドウの ID を作ります.
 *
 * @returns ID.
 */
function generateWindowId(): string {
  return `window-${Date.now()}-${Math.random().toString(36).slice(2, 11)}`
}

/**
 * アプリの画面 (ウィンドウ, モードライン, ミニバッファ, バッファの一覧).
 *
 * @returns 画面.
 */
function App() {
  // 開いているバッファ (開いた順)
  const [buffers, setBuffers] = useState<Buffer[]>([createScratchBuffer()])
  // ウィンドウの分け方と, 各ウィンドウに出すバッファと, カーソルの位置
  const [layout, setLayout] = useState<Layout>({ kind: 'window', id: FIRST_WINDOW })
  const [windowBuffers, setWindowBuffers] = useState<Record<string, string>>({
    [FIRST_WINDOW]: buffers[0].id,
  })
  const [cursors, setCursors] = useState<Record<string, CursorInfo>>({})
  const [activeWindow, setActiveWindow] = useState(FIRST_WINDOW)
  // ホームディレクトリ (ファイルの名前の入力の始め. `/` で終わる)
  const [homeDir, setHomeDir] = useState('/')
  // ミニバッファの入力を待っているコマンドと, ミニバッファの状態
  const [pendingAction, setPendingAction] = useState<PendingAction>(null)
  const [minibufferState, setMinibufferState] = useState<MinibufferState>({
    mode: 'message',
    text: '',
  })
  // バッファの一覧 (C-x C-b) を出しているか
  const [showBufferList, setShowBufferList] = useState(false)
  // 進めている置換
  const [replace, setReplace] = useState<ReplaceState | null>(null)
  // 置き換えてよいか聞いている, ファイルのパス (C-x C-w で, ほかのファイルに書くとき)
  const [overwritePath, setOverwritePath] = useState<string | null>(null)
  // Lisp で作ったコマンドと, 割り当てたキー
  const [lispCommands, setLispCommands] = useState<string[]>([])
  const [lispKeys, setLispKeys] = useState<KeyBinding[]>([])
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
  // Lisp から動かすエディタ (画面の今の状態を使うので, 描くたびに作り直す)
  const lispHost = useRef<LispHost>(undefined)
  // 最後に C-x r t で入れた文字 (次の既定)
  const rectangleString = useRef<string | null>(null)
  // ブックマークの一覧 (新しい順) と, 最後に使ったブックマークの名前 (C-x r b の既定)
  const [bookmarks, setBookmarks] = useState<Bookmark[]>([])
  const lastBookmark = useRef<string | null>(null)
  // バッファを出したら置く, カーソルの位置 (ブックマークへ移るとき)
  const pendingPoint = useRef<number | null>(null)
  // 聞いているあいだの, 開き直すファイル (C-x C-v)
  const alternatePath = useRef('')
  // 保存するか聞いているバッファ (C-x s) と, 保存した数
  const [saveQueue, setSaveQueue] = useState<{ buffers: Buffer[]; saved: number } | null>(null)
  // バッファごとの, 最後に自動保存した本文
  const autoSaved = useRef(new Map<string, string>())
  // ミニバッファの入力の履歴 (履歴の名前ごと. 新しい順)
  const [histories, setHistories] = useState<Record<string, string[]>>({})
  // isearch の検索の履歴 (文字と, 正規表現. 新しい順)
  const searchRing = useRef({ plain: [] as string[], regexp: [] as string[] })

  // 選んでいるウィンドウのバッファと, カーソルの位置
  const currentBufferId = windowBuffers[activeWindow]
  const cursor = cursors[activeWindow] ?? START_CURSOR
  // バッファを消すときは, ウィンドウのバッファも一緒に変えるので, 必ず見つかる
  const currentBuffer = buffers.find((b) => b.id === currentBufferId) as Buffer

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
      content,
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
   * 選んでいるウィンドウに, バッファを出します.
   *
   * @param bufferId - バッファ.
   */
  const switchToBuffer = (bufferId: string) => {
    setCurrentBufferId(bufferId)
  }

  /**
   * 選んでいるウィンドウのバッファを消します. 全て消したら, *scratch* を作り直します.
   * 保存していないバッファは消さず, メッセージを出します.
   *
   * @returns 消したら `true`.
   */
  const killCurrentBuffer = () => {
    if (currentBuffer.modified) {
      setMinibufferState({
        mode: 'message',
        text: `Buffer "${currentBuffer.name}" is modified. Use C-x C-s to save first.`,
      })
      return false
    }

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
    return true
  }

  // ホームディレクトリを, 始めに 1 回だけ取る (取れなければ, `/` のまま)
  useEffect(() => {
    getHomeDir()
      .then((dir) => setHomeDir(dir.endsWith('/') ? dir : dir + '/'))
      .catch(() => {})
  }, [])

  // 保存したブックマークを, 始めに 1 回だけ読む
  useEffect(() => {
    loadBookmarks()
      .then(setBookmarks)
      .catch((err) => setMinibufferState({ mode: 'message', text: `Bookmark file error: ${err}` }))
  }, [])

  // ブックマークへ移るときは, バッファを出してから, カーソルを置く
  useEffect(() => {
    if (pendingPoint.current === null) return
    editor.current?.setCursor(pendingPoint.current)
    pendingPoint.current = null
  }, [currentBufferId, editor])

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
  }, [buffers])

  // Lisp から動かすエディタを, 今の状態で作り直す
  useEffect(() => {
    const handle = () => editor.current as EditorHandle
    lispHost.current = {
      text: () => handle().text(),
      point: () => handle().point(),
      setPoint: (pos) => handle().setCursor(pos),
      insert: (text) => handle().insert(text),
      deleteRegion: (from, to) => handle().deleteRegion(from, to),
      bufferName: () => currentBuffer.name,
      message: (text) => setMinibufferState({ mode: 'message', text }),
      command: (name) => handle().run(name),
    }
  })

  /**
   * ウィンドウで書き換えた本文を, バッファに覚えます. 同じバッファを出している別のウィンドウにも写します.
   *
   * @param windowId - 書き換えたウィンドウ.
   * @param value - 本文.
   */
  const handleEditorChange = (windowId: string, value: string) => {
    const bufferId = windowBuffers[windowId]
    setBuffers((prev) =>
      prev.map((b) => (b.id === bufferId ? { ...b, content: value, modified: true } : b)),
    )
    for (const [w, b] of Object.entries(windowBuffers)) {
      if (w !== windowId && b === bufferId) handles.current.get(w)?.sync(value)
    }
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

  /**
   * ミニバッファに, メッセージを出します.
   *
   * @param text - メッセージ.
   */
  const showMessage = (text: string) => {
    setMinibufferState({ mode: 'message', text })
  }

  /**
   * 評価の結果を出して, Lisp で作ったコマンドと, 割り当てたキーを覚えます.
   *
   * @param response - 評価の結果.
   * @param quiet - 成功したときは, 結果を出さない (コマンドと, 初期設定ファイル).
   */
  const showEvalResult = (response: EvalResponse, quiet = false) => {
    setLispCommands(response.commands)
    setLispKeys(response.keys)
    if (response.error) {
      showMessage(`Error: ${response.error}`)
    } else if (!quiet) {
      showMessage(response.result)
    }
  }

  /**
   * コードを評価して, 結果を出します.
   *
   * @param code - コード.
   */
  const evaluate = async (code: string) => {
    try {
      showEvalResult(await evalLisp(code))
    } catch (err) {
      showMessage(`Eval error: ${err}`)
    }
  }

  /**
   * Lisp で作ったコマンドを実行します.
   *
   * @param name - コマンドの名前.
   */
  const handleRunLispCommand = async (name: string) => {
    try {
      showEvalResult(await runLispCommand(name), true)
    } catch (err) {
      showMessage(`Eval error: ${err}`)
    }
  }

  // 評価器からの, エディタの操作の頼みを受ける. 受け始めてから, 初期設定ファイルを読む
  useEffect(() => {
    let stopped = false
    let unlisten: UnlistenFn | undefined
    void listenEditorRequests(() => lispHost.current as LispHost).then((stop) => {
      // 受け始める前に, 画面を閉じた (開発版の StrictMode では, 1 度閉じて, 開き直す)
      if (stopped) {
        stop()
        return
      }
      unlisten = stop
      loadInitFile()
        .then((response) => showEvalResult(response, true))
        .catch((err) => showMessage(`Init file error: ${err}`))
    })
    return () => {
      stopped = true
      unlisten?.()
    }
    // 始めの 1 回だけ
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [])

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
    setBuffers((prev) =>
      prev.map((b) =>
        b.id === buffer.id ? { ...b, filePath: path, name, modified: false, backedUp: true } : b,
      ),
    )
    showMessage(`Wrote ${path}`)
    return true
  }

  /**
   * ファイルを読んで, バッファを作ります (まだ出さない). 自動保存のファイルの方が新しければ, そう知らせます.
   *
   * @param path - ファイルのパス.
   * @returns バッファと, 出すメッセージ. ファイルが無ければ, 空のバッファ.
   */
  const visitFile = async (path: string): Promise<{ buffer: Buffer; message: string }> => {
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

  /**
   * 選んでいるウィンドウのバッファを, 別のバッファに置き換えます (同じバッファを出しているウィンドウも).
   *
   * @param next - 新しいバッファ.
   */
  const replaceCurrentBuffer = (next: Buffer) => {
    const old = currentBuffer.id
    for (const handle of handles.current.values()) handle.forget(old)
    setBuffers((prev) => prev.map((b) => (b.id === old ? next : b)))
    setWindowBuffers((prev) =>
      Object.fromEntries(Object.entries(prev).map(([w, b]) => [w, b === old ? next.id : b])),
    )
  }

  /**
   * ファイルを開いたバッファで, 選んでいるウィンドウのバッファを置き換えます (C-x C-v).
   *
   * @param path - ファイルのパス.
   */
  const findAlternateFile = async (path: string) => {
    try {
      const { buffer, message } = await visitFile(path)
      replaceCurrentBuffer(buffer)
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
   * 選んでいるウィンドウのバッファの本文を, 置き換えます.
   *
   * @param content - 本文.
   * @param modified - 置き換えた後に, 変更ありにするなら `true`.
   */
  const setCurrentText = (content: string, modified: boolean) => {
    ;(editor.current as EditorHandle).setText(content)
    updateCurrentBuffer({ content, modified })
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
      switchToBuffer(open.id)
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
          content,
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
   * 出すバッファの名前を, ミニバッファで聞きます (C-x b).
   */
  const handleSwitchBuffer = () => {
    const bufferNames = buffers.map((b) => b.name).join(', ')
    setPendingAction('switch-buffer')
    setMinibufferState({
      mode: 'input',
      prompt: `Switch to buffer (${bufferNames}): `,
      input: '',
      completions: buffers.map((b) => b.name),
    })
  }

  /**
   * 選んでいるウィンドウのバッファを消します (C-x k). 保存していなければ, 消さずにメッセージを出します.
   */
  const handleKillBuffer = () => {
    if (currentBuffer.modified) {
      setPendingAction('kill-buffer')
      setMinibufferState({
        mode: 'message',
        text: `Buffer "${currentBuffer.name}" is modified. Use C-x C-s to save.`,
      })
    } else {
      killCurrentBuffer()
    }
  }

  /**
   * バッファの一覧を出します (C-x C-b).
   */
  const handleListBuffers = () => {
    setShowBufferList(true)
  }

  /**
   * キーの割り当ての一覧を, *Help* バッファに出します (前の *Help* は, 作り直す).
   */
  const handleDescribeBindings = () => {
    // Lisp の割り当ては, 同じキーの既定の割り当てより先に使う
    const bindings = [
      ...lispKeys.map(({ key, command }) => [key, command] as const),
      ...KEY_BINDINGS,
    ]
    const seen = new Set<string>()
    const lines = bindings
      .filter(([key]) => !seen.has(key) && seen.add(key))
      .map(([key, command]) => `${toEmacsKey(key).padEnd(24)}${command}`)
    const old = buffers.find((b) => b.name === '*Help*')
    if (old) {
      for (const handle of handles.current.values()) handle.forget(old.id)
      setBuffers((prev) => prev.filter((b) => b.id !== old.id))
    }
    const header = `${'key'.padEnd(24)}binding\n${'---'.padEnd(24)}-------\n`
    createNewBuffer('*Help*', null, header + lines.join('\n') + '\n')
  }

  /**
   * カーソルの前の S 式を評価して, 結果を出します (C-x C-e).
   */
  const handleEvalLastSexp = async () => {
    // カーソルの直前の S 式の範囲を求める
    const cursorPos = cursor.offset
    const startPos = backwardSexp(currentBuffer.content, cursorPos)
    const endPos = forwardSexp(currentBuffer.content, startPos)

    if (startPos === endPos) {
      setMinibufferState({ mode: 'message', text: 'No S-expression found before cursor' })
      return
    }

    await evaluate(currentBuffer.content.slice(startPos, endPos))
  }

  /**
   * 評価する式を, ミニバッファで聞きます (M-:).
   */
  const handleEvalExpression = () => {
    setPendingAction('eval-expression')
    setMinibufferState({ mode: 'input', prompt: 'Eval: ', input: '' })
  }

  /**
   * アプリを終わります (C-x C-c). 保存していないバッファがあれば, 終わってよいか聞きます.
   */
  const handleSaveBuffersKillTerminal = async () => {
    // 保存していないバッファがあれば, 終わってよいか聞く (Emacs と同じ)
    if (buffers.some((b) => b.modified)) {
      setPendingAction('confirm-exit')
      setMinibufferState({
        mode: 'input',
        prompt: 'Modified buffers exist; exit anyway? (yes or no) ',
        input: '',
      })
      return
    }
    await getCurrentWindow().close()
  }

  /**
   * 矩形を置き換える文字を, ミニバッファで聞きます (C-x r t).
   */
  const handleStringRectangle = () => {
    if (!editor.current?.hasMark()) {
      showMessage('The mark is not set now, so there is no region')
      return
    }
    const last = rectangleString.current
    setPendingAction('string-rectangle')
    setMinibufferState({
      mode: 'input',
      prompt: last === null ? 'String rectangle: ' : `String rectangle (default ${last}): `,
      input: '',
    })
  }

  /**
   * ブックマークの名前を, ミニバッファで聞きます (C-x r m). 既定は, ファイルの名前です.
   */
  const handleBookmarkSet = () => {
    if (!currentBuffer.filePath) {
      showMessage('Buffer not visiting a file or directory')
      return
    }
    setPendingAction('bookmark-set')
    setMinibufferState({
      mode: 'input',
      prompt: `Set bookmark named (default ${bufferNameFromPath(currentBuffer.filePath)}): `,
      input: '',
      completions: bookmarks.map((b) => b.name),
    })
  }

  /**
   * 移るブックマークの名前を, ミニバッファで聞きます (C-x r b). 既定は, 最後に使ったブックマークです.
   */
  const handleBookmarkJump = () => {
    const last = lastBookmark.current
    setPendingAction('bookmark-jump')
    setMinibufferState({
      mode: 'input',
      prompt: last === null ? 'Jump to bookmark: ' : `Jump to bookmark (default ${last}): `,
      input: '',
      completions: bookmarks.map((b) => b.name),
    })
  }

  /**
   * ブックマークを付けて, 保存します.
   *
   * @param value - 入力した名前 (空なら, ファイルの名前).
   */
  const setBookmark = async (value: string) => {
    const filePath = currentBuffer.filePath as string
    const name = value === '' ? bufferNameFromPath(filePath) : value
    const handle = editor.current as EditorHandle
    const next = storeBookmark(
      bookmarks,
      makeBookmark(name, filePath, handle.text(), handle.point()),
    )
    setBookmarks(next)
    lastBookmark.current = name
    showMessage('')
    try {
      await saveBookmarks(next)
    } catch (err) {
      showMessage(`Error: ${err}`)
    }
  }

  /**
   * ブックマークのファイルを出して, 位置へ動きます. ファイルを開いたバッファがあれば, それを使います.
   *
   * @param value - 入力した名前 (空なら, 最後に使ったブックマーク).
   */
  const jumpToBookmark = async (value: string) => {
    const name = value === '' ? lastBookmark.current : value
    const bookmark = bookmarks.find((b) => b.name === name)
    if (!bookmark) {
      showMessage(name ? `Invalid bookmark ${name}` : 'No bookmark specified')
      return
    }
    lastBookmark.current = bookmark.name
    showMessage('')
    const open = buffers.find((b) => b.filePath === bookmark.filename)
    if (open) {
      const pos = bookmarkPosition(bookmark, open.content)
      if (open.id === currentBufferId) {
        editor.current?.setCursor(pos)
      } else {
        pendingPoint.current = pos
        switchToBuffer(open.id)
      }
      return
    }
    try {
      const content = await readFile(bookmark.filename)
      pendingPoint.current = bookmarkPosition(bookmark, content)
      createNewBuffer(bufferNameFromPath(bookmark.filename), bookmark.filename, content)
    } catch (err) {
      showMessage(`Error: ${err}`)
    }
  }

  /**
   * 移動する行の番号を, ミニバッファで聞きます (M-g g).
   */
  const handleGotoLine = () => {
    setPendingAction('goto-line')
    setMinibufferState({ mode: 'input', prompt: 'Goto line: ', input: '' })
  }

  /**
   * 見つかったところを選びます. カーソルは, 前へ探すときは終わりに, 後ろへ探すときは始めに置きます.
   *
   * @param direction - 探す向き.
   * @param match - 見つかったところ.
   */
  const selectMatch = (
    direction: 'forward' | 'backward',
    match: { start: number; end: number },
  ) => {
    if (direction === 'forward') {
      editor.current?.select(match.start, match.end)
    } else {
      editor.current?.select(match.end, match.start)
    }
  }

  /**
   * isearch を始めます.
   *
   * @param direction - 探す向き.
   * @param regexp - 正規表現で探すなら `true`.
   */
  const handleIsearch = (direction: 'forward' | 'backward', regexp: boolean) => {
    setMinibufferState({
      mode: 'isearch',
      direction,
      regexp,
      query: '',
      originalCursor: cursor.offset,
      matchStart: null,
      matchEnd: null,
    })
  }

  /**
   * 置換を始めます (探す文字を聞く).
   *
   * @param kind - 置換の種類.
   */
  const handleReplace = (kind: ReplaceKind) => {
    setReplace({ kind, count: 0 })
    setPendingAction('replace-from')
    setMinibufferState({ mode: 'input', prompt: `${replaceTitle(kind)}: `, input: '' })
  }

  /**
   * query-replace で, 置き換える所を選んで聞きます. 無ければ, 終えます.
   *
   * @param state - 進めている置換 (spec がある).
   * @param pos - 探し始める位置.
   */
  const askReplace = (state: ReplaceState, pos: number) => {
    const handle = editor.current as EditorHandle
    const spec = state.spec as ReplaceSpec
    const match = findReplaceMatch(handle.text(), spec, pos)
    if (!match) {
      finishReplace(state.count, pos)
      return
    }
    handle.select(match.start, match.end)
    setReplace({ ...state, match })
    setMinibufferState({
      mode: 'query',
      prompt: `Query replacing ${spec.from} with ${spec.to}: (y, n, !, ., q) `,
    })
  }

  /**
   * 置換を終えて, 置き換えた数を出します.
   *
   * @param count - 置き換えた数.
   * @param pos - カーソルを置く位置.
   */
  const finishReplace = (count: number, pos: number) => {
    editor.current?.setCursor(pos)
    setReplace(null)
    showMessage(replacedMessage(count))
  }

  /**
   * 1 つのキーで答える問い (ファイルを置き換えてよいか, query-replace) に答えます.
   *
   * @param key - 押したキー.
   */
  const handleQueryKey = (key: string) => {
    if (saveQueue !== null) {
      void answerSave(key, saveQueue)
      return
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
      return
    }
    const state = replace as ReplaceState
    const match = state.match as ReplaceMatch
    const handle = editor.current as EditorHandle
    const replaceMatch = () => {
      handle.replace(match.start, match.end, match.replacement)
      return match.start + match.replacement.length
    }
    if (key === 'y' || key === ' ') {
      const pos = replaceMatch()
      askReplace({ ...state, count: state.count + 1 }, pos)
    } else if (key === 'n' || key === 'Backspace' || key === 'Delete') {
      askReplace(state, match.end)
    } else if (key === '!') {
      const rest = replaceRemaining(handle, state.spec as ReplaceSpec, replaceMatch())
      finishReplace(state.count + 1 + rest.count, rest.pos)
    } else if (key === '.') {
      finishReplace(state.count + 1, replaceMatch())
    } else if (key === 'q' || key === 'Enter') {
      finishReplace(state.count, match.start)
    }
  }

  /**
   * isearch で, 同じ文字の次 (または前) に合う所へ進みます (isearch 中の C-s / C-r).
   * 前へ探していて, 本文の始めで合っている場合は, 失敗にします (もう 1 度押すと, 現在位置から探し直す).
   *
   * @param direction - 探す向き.
   * @param current - isearch の状態.
   */
  const handleIsearchRepeat = (
    direction: 'forward' | 'backward',
    current: MinibufferIsearchState,
  ) => {
    // 探す文字が空なら, 最後に探した文字で探す
    if (current.query === '' && direction === current.direction) {
      const ring = searchRing.current[current.regexp ? 'regexp' : 'plain']
      if (ring.length > 0) searchIsearch(current, ring[0], { ringIndex: 0 })
      return
    }
    if (direction === 'backward' && current.matchStart !== null && current.matchStart <= 0) {
      setMinibufferState({ ...current, matchStart: null, matchEnd: null, yankBase: undefined })
      return
    }

    const fromPos =
      current.matchStart === null
        ? current.originalCursor
        : direction === 'forward'
          ? current.matchStart + 1
          : current.matchStart - 1

    const match = findNextMatch(
      currentBuffer.content,
      current.query,
      fromPos,
      direction,
      current.regexp,
    )

    if (match) {
      selectMatch(direction, match)
      setMinibufferState({
        ...current,
        direction,
        matchStart: match.start,
        matchEnd: match.end,
        yankBase: undefined,
      })
    } else {
      setMinibufferState({ ...current, matchStart: null, matchEnd: null, yankBase: undefined })
    }
  }

  /**
   * isearch の探す文字を変えて, 探し始めた位置から探し直します.
   *
   * @param current - isearch の状態.
   * @param query - 探す文字.
   * @param changes - ほかに変える項目 (続けた M-y の文字などは, 変えなければ捨てる).
   */
  const searchIsearch = (
    current: MinibufferIsearchState,
    query: string,
    changes: Partial<MinibufferIsearchState> = {},
  ) => {
    const match = findNextMatch(
      currentBuffer.content,
      query,
      current.originalCursor,
      current.direction,
      current.regexp,
    )
    if (match) selectMatch(current.direction, match)
    setMinibufferState({
      ...current,
      yankBase: undefined,
      ...changes,
      query,
      matchStart: match ? match.start : null,
      matchEnd: match ? match.end : null,
    })
  }

  /**
   * isearch の中のコマンドを行います. C-w でカーソルの後ろの単語か文字を, M-y でキルした文字を取り込み,
   * M-p と M-n で, 検索の履歴を出します.
   *
   * @param command - コマンド.
   * @param current - isearch の状態.
   */
  const handleIsearchCommand = (command: IsearchCommand, current: MinibufferIsearchState) => {
    const quote = (text: string) => (current.regexp ? quoteRegexp(text) : text)
    const handle = editor.current as EditorHandle
    if (command === 'yank-word-or-char') {
      // 合っていれば, 合った所の終わりから (前へ探していても)
      const text = currentBuffer.content
      const from = current.matchEnd ?? current.yankPoint ?? current.originalCursor
      const end = yankWordOrCharEnd(text, from)
      if (end === from) return
      const failing = current.matchStart === null && current.query !== ''
      searchIsearch(current, current.query + quote(text.slice(from, end)), {
        yankPoint: failing ? end : null,
      })
    } else if (command === 'yank-pop') {
      // 続けた M-y は, 取り込んだ文字を, 1 つ古いキルした文字に置き換える
      const base = current.yankBase ?? current.query
      const kill = current.yankBase === undefined ? handle.currentKill() : handle.rotateKill()
      if (kill === undefined) return
      searchIsearch(current, base + quote(kill), { yankBase: base })
    } else {
      const ring = searchRing.current[current.regexp ? 'regexp' : 'plain']
      if (ring.length === 0) return
      const advance = command === 'ring-advance'
      const index =
        ((current.ringIndex ?? (advance ? 0 : -1)) + (advance ? -1 : 1) + ring.length) % ring.length
      searchIsearch(current, ring[index], { ringIndex: index })
    }
  }

  /**
   * 実行するコマンドの名前を, ミニバッファで聞きます (M-x).
   */
  const handleExecuteCommand = () => {
    setPendingAction('execute-command')
    setMinibufferState({
      mode: 'input',
      prompt: 'M-x ',
      input: '',
      completions: [...(editor.current as EditorHandle).commandNames(), ...Object.keys(actions)],
    })
  }

  /**
   * ミニバッファの入力が変わったときに, 入力を覚えます. isearch なら, 入力した文字で探し直します.
   *
   * @param value - 入力.
   * @param current - ミニバッファの状態.
   */
  const handleMinibufferInputChange = (
    value: string,
    current: MinibufferInputState | MinibufferIsearchState,
  ) => {
    if (current.mode === 'input') {
      setMinibufferState({ ...current, input: value })
    } else {
      searchIsearch(current, value)
    }
  }

  // 画面のコマンド (キーと M-x から, 名前で呼ぶ). エディタのコマンドは, エディタが持つ.
  // Lisp で作ったコマンドは, 同じ名前のコマンドより先に使う
  const actions: Record<string, () => void> = {
    'find-file': handleFindFile,
    'save-buffer': () => void handleSaveBuffer(),
    'write-file': handleWriteFile,
    'switch-to-buffer': handleSwitchBuffer,
    'kill-buffer': handleKillBuffer,
    'list-buffers': handleListBuffers,
    'split-window-below': () => handleSplitWindow('below'),
    'split-window-right': () => handleSplitWindow('right'),
    'other-window': handleOtherWindow,
    'delete-window': handleDeleteWindow,
    'delete-other-windows': handleDeleteOtherWindows,
    'describe-bindings': handleDescribeBindings,
    'save-buffers-kill-terminal': () => void handleSaveBuffersKillTerminal(),
    'execute-extended-command': handleExecuteCommand,
    'eval-last-sexp': () => void handleEvalLastSexp(),
    'eval-expression': handleEvalExpression,
    'isearch-forward': () => handleIsearch('forward', false),
    'isearch-backward': () => handleIsearch('backward', false),
    'isearch-forward-regexp': () => handleIsearch('forward', true),
    'isearch-backward-regexp': () => handleIsearch('backward', true),
    'query-replace': () => handleReplace({ query: true, regexp: false }),
    'query-replace-regexp': () => handleReplace({ query: true, regexp: true }),
    'replace-string': () => handleReplace({ query: false, regexp: false }),
    'replace-regexp': () => handleReplace({ query: false, regexp: true }),
    'goto-line': handleGotoLine,
    'string-rectangle': handleStringRectangle,
    'bookmark-set': handleBookmarkSet,
    'bookmark-jump': handleBookmarkJump,
    'insert-file': handleInsertFile,
    'find-alternate-file': handleFindAlternateFile,
    'save-some-buffers': handleSaveSomeBuffers,
    'revert-buffer': handleRevertBuffer,
    'recover-this-file': () => void handleRecoverThisFile(),
    dired: handleDired,
    ...Object.fromEntries(
      lispCommands.map((name) => [name, () => void handleRunLispCommand(name)]),
    ),
  }

  /**
   * ミニバッファの入力を確定したときに, 入力を待っていたコマンドを行います.
   * 待っているコマンドがなければ, isearch を終わります.
   *
   * @param value - 入力.
   */
  const handleMinibufferSubmit = async (value: string) => {
    const action = pendingAction
    setPendingAction(null)
    const historyName = action && HISTORY_NAMES[action]
    if (historyName) {
      setHistories((prev) => ({
        ...prev,
        [historyName]: addToHistory(prev[historyName] ?? [], value, HISTORY_LENGTH),
      }))
    }

    if (action === 'find-file') {
      try {
        const { buffer, message } = await visitFile(value)
        setBuffers((prev) => [...prev, buffer])
        setCurrentBufferId(buffer.id)
        showMessage(message)
      } catch (err) {
        setMinibufferState({ mode: 'message', text: `Error: ${err}` })
      }
    } else if (action === 'find-alternate-file') {
      // 書き換えたファイルのバッファは, 捨ててよいか聞く
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
    } else if (action === 'confirm-alternate') {
      if (value.trim() === 'yes') await findAlternateFile(alternatePath.current)
      else showMessage('Aborted')
    } else if (action === 'confirm-revert') {
      if (value.trim() === 'yes') await revertBuffer(currentBuffer.filePath as string)
      else showMessage('')
    } else if (action === 'confirm-recover') {
      if (value.trim() === 'yes') await recoverFile(currentBuffer.filePath as string)
      else showMessage('')
    } else if (action === 'insert-file') {
      await insertFile(value)
    } else if (action === 'dired') {
      await listDirectoryInto(value)
    } else if (action === 'write-file') {
      // バッファのファイルでない, ほかのファイルを置き換えるときは, 聞く (Emacs と同じ)
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
    } else if (action === 'switch-buffer') {
      const targetBuffer = buffers.find((b) => b.name === value.trim())
      if (targetBuffer) {
        switchToBuffer(targetBuffer.id)
        setMinibufferState({ mode: 'message', text: '' })
      } else {
        setMinibufferState({ mode: 'message', text: `No buffer named "${value}"` })
      }
    } else if (action === 'kill-buffer') {
      killCurrentBuffer()
      setMinibufferState({ mode: 'message', text: '' })
    } else if (action === 'execute-command') {
      setMinibufferState({ mode: 'message', text: '' })
      if (!editor.current?.run(value.trim())) {
        setMinibufferState({ mode: 'message', text: `Undefined command: ${value}` })
      }
    } else if (action === 'goto-line') {
      editor.current?.gotoLine(Number(value))
      setMinibufferState({ mode: 'message', text: '' })
    } else if (action === 'confirm-exit') {
      if (value.trim() === 'yes') {
        await getCurrentWindow().close()
      } else {
        setMinibufferState({ mode: 'message', text: '' })
      }
    } else if (action === 'eval-expression') {
      await evaluate(value)
    } else if (action === 'replace-from') {
      const state = replace as ReplaceState
      if (value === '') {
        setReplace(null)
        showMessage('')
      } else if (state.kind.regexp && !compileRegexp(value)) {
        setReplace(null)
        showMessage(`Invalid regexp: ${value}`)
      } else {
        setReplace({ ...state, from: value })
        setPendingAction('replace-to')
        setMinibufferState({
          mode: 'input',
          prompt: `${replaceTitle(state.kind)} ${value} with: `,
          input: '',
        })
      }
    } else if (action === 'string-rectangle') {
      // 空の入力は, 次の既定にしない (Emacs の履歴と同じ)
      const string = value === '' ? (rectangleString.current ?? '') : value
      if (string !== '') rectangleString.current = string
      editor.current?.stringRectangle(string)
      showMessage('')
    } else if (action === 'bookmark-set') {
      await setBookmark(value)
    } else if (action === 'bookmark-jump') {
      await jumpToBookmark(value)
    } else if (action === 'replace-to') {
      const state = replace as ReplaceState
      const spec = { from: state.from as string, to: value, regexp: state.kind.regexp }
      const pos = (editor.current as EditorHandle).point()
      if (state.kind.query) {
        askReplace({ ...state, spec }, pos)
      } else {
        const done = replaceRemaining(editor.current as EditorHandle, spec, pos)
        finishReplace(done.count, done.pos)
      }
    } else {
      // 保留中のコマンドがなければ, isearch の確定 (コマンドのない入力欄は, isearch だけ).
      // 見つかったところの, 探した向きの側にカーソルを置く
      const current = minibufferState as MinibufferIsearchState
      // 探した文字を, 検索の履歴に加える
      const kind = current.regexp ? 'regexp' : 'plain'
      searchRing.current[kind] = addToHistory(
        searchRing.current[kind],
        current.query,
        SEARCH_RING_MAX,
        true,
      )
      if (current.matchStart !== null) {
        editor.current?.setCursor(
          current.direction === 'forward' ? (current.matchEnd as number) : current.matchStart,
        )
      }
      setMinibufferState({ mode: 'message', text: '' })
    }
  }

  /**
   * ミニバッファの入力をやめます (C-g). isearch なら, カーソルを探す前の位置に戻します.
   */
  const handleMinibufferCancel = () => {
    const current = minibufferState
    setPendingAction(null)
    setReplace(null)
    setOverwritePath(null)
    setSaveQueue(null)

    if (current.mode === 'isearch') {
      editor.current?.setCursor(current.originalCursor)
    }
    setMinibufferState({ mode: 'message', text: 'Quit' })
  }

  // 入力を待っているコマンドの, 入力の履歴の名前
  const historyName = pendingAction ? HISTORY_NAMES[pendingAction] : undefined

  return (
    <div className="app">
      <div className="windows">
        {windowRects(layout).map((rect) => {
          const bufferId = windowBuffers[rect.id]
          const buffer = buffers.find((b) => b.id === bufferId) as Buffer
          const active = rect.id === activeWindow
          const mode =
            buffer.directory === undefined
              ? majorMode(buffer.name, buffer.filePath, buffer.content)
              : DIRED_MODE
          return (
            <div
              key={rect.id}
              className={`window${rect.left + rect.width < 100 ? ' window-left-of' : ''}`}
              style={{
                left: `${rect.left}%`,
                top: `${rect.top}%`,
                width: `${rect.width}%`,
                height: `${rect.height}%`,
              }}
              onFocus={() => selectWindow(rect.id)}
            >
              <CodeEditor
                bufferId={bufferId}
                initialContent={buffer.content}
                onChange={(value) => handleEditorChange(rect.id, value)}
                onCursorChange={(info) => setCursors((prev) => ({ ...prev, [rect.id]: info }))}
                actions={actions}
                keyBindings={lispKeys}
                onMessage={showMessage}
                disabled={minibufferState.mode !== 'message' || showBufferList}
                active={active}
                language={mode.language}
                indentTabs={mode.indentTabs}
                tabWidth={mode.tabWidth}
                handle={(handle) => {
                  if (handle) handles.current.set(rect.id, handle)
                  else handles.current.delete(rect.id)
                }}
              />
              <ModeLine
                bufferName={buffer.name}
                modified={buffer.modified}
                cursor={cursors[rect.id] ?? START_CURSOR}
                mode={mode.name}
                bufferCount={buffers.length}
                currentBufferIndex={buffers.findIndex((b) => b.id === bufferId) + 1}
                active={active}
              />
            </div>
          )
        })}
      </div>
      <Minibuffer
        state={
          minibufferState.mode === 'input'
            ? { ...minibufferState, history: histories[historyName as string] }
            : minibufferState
        }
        onInputChange={handleMinibufferInputChange}
        onSubmit={handleMinibufferSubmit}
        onCancel={handleMinibufferCancel}
        onIsearchRepeat={handleIsearchRepeat}
        onIsearchCommand={handleIsearchCommand}
        onQueryKey={handleQueryKey}
      />
      {showBufferList && (
        <BufferList
          buffers={buffers}
          currentBufferId={currentBufferId}
          onSelectBuffer={(bufferId) => {
            switchToBuffer(bufferId)
            setShowBufferList(false)
          }}
          onClose={() => setShowBufferList(false)}
        />
      )}
    </div>
  )
}

export default App
