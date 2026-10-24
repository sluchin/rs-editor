import { useState, useEffect, useRef } from 'react'
import type { UnlistenFn } from '@tauri-apps/api/event'
import { getCurrentWindow } from '@tauri-apps/api/window'
import CodeEditor, { CursorInfo, EditorHandle } from './components/CodeEditor'
import ModeLine from './components/ModeLine'
import Minibuffer, {
  MinibufferInputState,
  MinibufferIsearchState,
  MinibufferState,
} from './components/Minibuffer'
import BufferList from './components/BufferList'
import { readFile, writeFile, pathExists, getHomeDir, listDirectory } from './lib/fileOps'
import { completeFileName } from './lib/complete'
import { majorMode } from './lib/modes'
import { compileRegexp, findNextMatch } from './lib/isearch'
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

const INITIAL_TEXT = `;; This buffer is for text that is not saved, and for Lisp evaluation.
;; To create a file, visit it with C-x C-f and enter text in its buffer.
`

export interface Buffer {
  id: string
  name: string
  filePath: string | null
  content: string
  modified: boolean
}

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
  | null

/** 置換の種類 (query-replace, replace-string と, その正規表現の版). */
interface ReplaceKind {
  /** 1 つずつ聞くなら `true`. */
  query: boolean
  /** 正規表現で探すなら `true`. */
  regexp: boolean
}

/** 進めている置換. */
interface ReplaceState {
  kind: ReplaceKind
  /** 探す文字 (入力したら). */
  from?: string
  /** 置換の指定と, 聞いている所 (query-replace で, 聞いているあいだ). */
  spec?: ReplaceSpec
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

function generateBufferId(): string {
  return `buffer-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`
}

function bufferNameFromPath(path: string): string {
  const parts = path.split('/')
  return parts[parts.length - 1] || path
}

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

function App() {
  const [buffers, setBuffers] = useState<Buffer[]>([createScratchBuffer()])
  // ウィンドウの分け方と, 各ウィンドウに出すバッファと, カーソルの位置
  const [layout, setLayout] = useState<Layout>({ kind: 'window', id: FIRST_WINDOW })
  const [windowBuffers, setWindowBuffers] = useState<Record<string, string>>({
    [FIRST_WINDOW]: buffers[0].id,
  })
  const [cursors, setCursors] = useState<Record<string, CursorInfo>>({})
  const [activeWindow, setActiveWindow] = useState(FIRST_WINDOW)
  const [homeDir, setHomeDir] = useState('/')
  const [pendingAction, setPendingAction] = useState<PendingAction>(null)
  const [minibufferState, setMinibufferState] = useState<MinibufferState>({
    mode: 'message',
    text: '',
  })
  const [showBufferList, setShowBufferList] = useState(false)
  const [replace, setReplace] = useState<ReplaceState | null>(null)
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

  // Helper functions for buffer management
  const updateCurrentBuffer = (updates: Partial<Buffer>) => {
    setBuffers((prev) => prev.map((b) => (b.id === currentBufferId ? { ...b, ...updates } : b)))
  }

  const createNewBuffer = (
    name: string,
    filePath: string | null = null,
    content: string = '',
    modified = content !== '',
  ) => {
    const newBuffer: Buffer = {
      id: generateBufferId(),
      name,
      filePath,
      content,
      modified,
    }
    setBuffers((prev) => [...prev, newBuffer])
    setCurrentBufferId(newBuffer.id)
    return newBuffer
  }

  const switchToBuffer = (bufferId: string) => {
    setCurrentBufferId(bufferId)
  }

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

  useEffect(() => {
    getHomeDir()
      .then((dir) => setHomeDir(dir.endsWith('/') ? dir : dir + '/'))
      .catch(() => {})
  }, [])

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

  const handleOtherWindow = () => {
    const ids = windowIds(layout)
    selectWindow(ids[(ids.indexOf(activeWindowRef.current) + 1) % ids.length])
  }

  const handleDeleteWindow = () => {
    const next = deleteWindow(layout, activeWindowRef.current)
    if (!next) {
      showMessage('Attempt to delete minibuffer or sole ordinary window')
      return
    }
    setLayout(next)
    selectWindow(windowIds(next)[0])
  }

  const handleDeleteOtherWindows = () => {
    setLayout({ kind: 'window', id: activeWindowRef.current })
  }

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

  const handleFindFile = () => {
    setPendingAction('find-file')
    setMinibufferState({
      mode: 'input',
      prompt: 'Find file: ',
      input: homeDir,
      complete: completeFile,
    })
  }

  const handleSaveBuffer = async () => {
    if (currentBuffer.filePath) {
      try {
        await writeFile(currentBuffer.filePath, currentBuffer.content)
        updateCurrentBuffer({ modified: false })
        setMinibufferState({ mode: 'message', text: `Wrote ${currentBuffer.filePath}` })
      } catch (err) {
        setMinibufferState({ mode: 'message', text: `Error: ${err}` })
      }
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
    createNewBuffer('*Help*', null, header + lines.join('\n') + '\n', false)
  }

  const handleEvalLastSexp = async () => {
    // カーソル直前のS式の範囲を特定
    const cursorPos = cursor.offset
    const startPos = backwardSexp(currentBuffer.content, cursorPos)
    const endPos = forwardSexp(currentBuffer.content, startPos)

    if (startPos === endPos) {
      setMinibufferState({ mode: 'message', text: 'No S-expression found before cursor' })
      return
    }

    await evaluate(currentBuffer.content.slice(startPos, endPos))
  }

  const handleEvalExpression = () => {
    setPendingAction('eval-expression')
    setMinibufferState({ mode: 'input', prompt: 'Eval: ', input: '' })
  }

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
   * query-replace の問いに答えます.
   *
   * @param key - 押したキー.
   */
  const handleQueryKey = (key: string) => {
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

  const handleIsearchRepeat = (
    direction: 'forward' | 'backward',
    current: MinibufferIsearchState,
  ) => {
    if (direction === 'backward' && current.matchStart !== null && current.matchStart <= 0) {
      setMinibufferState({ ...current, matchStart: null, matchEnd: null })
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
      })
    } else {
      setMinibufferState({ ...current, matchStart: null, matchEnd: null })
    }
  }

  const handleExecuteCommand = () => {
    setPendingAction('execute-command')
    setMinibufferState({
      mode: 'input',
      prompt: 'M-x ',
      input: '',
      completions: [...(editor.current as EditorHandle).commandNames(), ...Object.keys(actions)],
    })
  }

  const handleMinibufferInputChange = (
    value: string,
    current: MinibufferInputState | MinibufferIsearchState,
  ) => {
    if (current.mode === 'input') {
      setMinibufferState({ ...current, input: value })
    } else {
      const match = findNextMatch(
        currentBuffer.content,
        value,
        current.originalCursor,
        current.direction,
        current.regexp,
      )
      if (match) {
        selectMatch(current.direction, match)
        setMinibufferState({
          ...current,
          query: value,
          matchStart: match.start,
          matchEnd: match.end,
        })
      } else {
        setMinibufferState({ ...current, query: value, matchStart: null, matchEnd: null })
      }
    }
  }

  // 画面のコマンド (キーと M-x から, 名前で呼ぶ). エディタのコマンドは, エディタが持つ.
  // Lisp で作ったコマンドは, 同じ名前のコマンドより先に使う
  const actions: Record<string, () => void> = {
    'find-file': handleFindFile,
    'save-buffer': () => void handleSaveBuffer(),
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
    ...Object.fromEntries(
      lispCommands.map((name) => [name, () => void handleRunLispCommand(name)]),
    ),
  }

  const handleMinibufferSubmit = async (value: string) => {
    const action = pendingAction
    setPendingAction(null)

    if (action === 'find-file') {
      try {
        const exists = await pathExists(value)
        if (exists) {
          const content = await readFile(value)
          const bufferName = bufferNameFromPath(value)
          createNewBuffer(bufferName, value, content)
          setMinibufferState({ mode: 'message', text: '' })
        } else {
          const bufferName = bufferNameFromPath(value)
          createNewBuffer(bufferName, value, '')
          setMinibufferState({ mode: 'message', text: '(New file)' })
        }
      } catch (err) {
        setMinibufferState({ mode: 'message', text: `Error: ${err}` })
      }
    } else if (action === 'write-file') {
      try {
        await writeFile(value, currentBuffer.content)
        updateCurrentBuffer({ filePath: value, modified: false })
        setMinibufferState({ mode: 'message', text: `Wrote ${value}` })
      } catch (err) {
        setMinibufferState({ mode: 'message', text: `Error: ${err}` })
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
      if (current.matchStart !== null) {
        editor.current?.setCursor(
          current.direction === 'forward' ? (current.matchEnd as number) : current.matchStart,
        )
      }
      setMinibufferState({ mode: 'message', text: '' })
    }
  }

  const handleMinibufferCancel = () => {
    const current = minibufferState
    setPendingAction(null)
    setReplace(null)

    if (current.mode === 'isearch') {
      editor.current?.setCursor(current.originalCursor)
    }
    setMinibufferState({ mode: 'message', text: 'Quit' })
  }

  return (
    <div className="app">
      <div className="windows">
        {windowRects(layout).map((rect) => {
          const bufferId = windowBuffers[rect.id]
          const buffer = buffers.find((b) => b.id === bufferId) as Buffer
          const active = rect.id === activeWindow
          const mode = majorMode(buffer.name, buffer.filePath)
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
        state={minibufferState}
        onInputChange={handleMinibufferInputChange}
        onSubmit={handleMinibufferSubmit}
        onCancel={handleMinibufferCancel}
        onIsearchRepeat={handleIsearchRepeat}
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
