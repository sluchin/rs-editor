import { useState, useEffect, useRef } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'
import MenuBar from './components/MenuBar'
import CodeEditor, { CursorInfo, EditorHandle } from './components/CodeEditor'
import ModeLine from './components/ModeLine'
import Minibuffer, {
  MinibufferInputState,
  MinibufferIsearchState,
  MinibufferState,
} from './components/Minibuffer'
import BufferList from './components/BufferList'
import { readFile, writeFile, pathExists, getHomeDir } from './lib/fileOps'
import { findNextMatch } from './lib/isearch'
import { backwardSexp, forwardSexp } from './lib/bracketMatch'
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
  | null

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

function App() {
  const [buffers, setBuffers] = useState<Buffer[]>([createScratchBuffer()])
  const [currentBufferId, setCurrentBufferId] = useState(buffers[0].id)
  const [cursor, setCursor] = useState<CursorInfo>({ line: 1, column: 1, offset: 0 })
  const [homeDir, setHomeDir] = useState('/')
  const [pendingAction, setPendingAction] = useState<PendingAction>(null)
  const [minibufferState, setMinibufferState] = useState<MinibufferState>({
    mode: 'message',
    text: '',
  })
  const [showBufferList, setShowBufferList] = useState(false)
  const editor = useRef<EditorHandle>(null)

  // バッファを消すときは, 今のバッファの ID も一緒に変えるので, 必ず見つかる
  const currentBuffer = buffers.find((b) => b.id === currentBufferId) as Buffer

  // Helper functions for buffer management
  const updateCurrentBuffer = (updates: Partial<Buffer>) => {
    setBuffers((prev) => prev.map((b) => (b.id === currentBufferId ? { ...b, ...updates } : b)))
  }

  const createNewBuffer = (name: string, filePath: string | null = null, content: string = '') => {
    const newBuffer: Buffer = {
      id: generateBufferId(),
      name,
      filePath,
      content,
      modified: content !== '',
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

    editor.current?.forget(currentBuffer.id)
    const newBuffers = buffers.filter((b) => b.id !== currentBuffer.id)
    if (newBuffers.length === 0) {
      // Create a new scratch buffer if all are killed
      const scratch = createScratchBuffer()
      setBuffers([scratch])
      setCurrentBufferId(scratch.id)
    } else {
      setBuffers(newBuffers)
      setCurrentBufferId(newBuffers[0].id)
    }
    return true
  }

  useEffect(() => {
    getHomeDir()
      .then((dir) => setHomeDir(dir.endsWith('/') ? dir : dir + '/'))
      .catch(() => {})
  }, [])

  const handleEditorChange = (value: string) => {
    updateCurrentBuffer({ content: value, modified: true })
  }

  const showMessage = (text: string) => {
    setMinibufferState({ mode: 'message', text })
  }

  const handleFindFile = () => {
    setPendingAction('find-file')
    setMinibufferState({ mode: 'input', prompt: 'Find file: ', input: homeDir })
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
      setMinibufferState({ mode: 'input', prompt: 'Save file: ', input: homeDir })
    }
  }

  const handleSwitchBuffer = () => {
    const bufferNames = buffers.map((b) => b.name).join(', ')
    setPendingAction('switch-buffer')
    setMinibufferState({
      mode: 'input',
      prompt: `Switch to buffer (${bufferNames}): `,
      input: '',
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

  const handleEvalLastSexp = async () => {
    // カーソル直前のS式の範囲を特定
    const cursorPos = cursor.offset
    const startPos = backwardSexp(currentBuffer.content, cursorPos)
    const endPos = forwardSexp(currentBuffer.content, startPos)

    if (startPos === endPos) {
      setMinibufferState({ mode: 'message', text: 'No S-expression found before cursor' })
      return
    }

    const sexp = currentBuffer.content.slice(startPos, endPos)

    try {
      const result = await invoke<{ result: string; error: string | null }>('eval_scheme', {
        code: sexp,
      })

      if (result.error) {
        setMinibufferState({ mode: 'message', text: `Error: ${result.error}` })
      } else {
        setMinibufferState({ mode: 'message', text: result.result })
      }
    } catch (err) {
      setMinibufferState({ mode: 'message', text: `Eval error: ${err}` })
    }
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

  const handleIsearchForward = () => {
    setMinibufferState({
      mode: 'isearch',
      direction: 'forward',
      query: '',
      originalCursor: cursor.offset,
      matchStart: null,
      matchEnd: null,
    })
  }

  const handleIsearchBackward = () => {
    setMinibufferState({
      mode: 'isearch',
      direction: 'backward',
      query: '',
      originalCursor: cursor.offset,
      matchStart: null,
      matchEnd: null,
    })
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

    const match = findNextMatch(currentBuffer.content, current.query, fromPos, direction)

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
    setMinibufferState({ mode: 'input', prompt: 'M-x ', input: '' })
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

  // 画面のコマンド (キーと M-x から, 名前で呼ぶ). エディタのコマンドは, エディタが持つ
  const actions: Record<string, () => void> = {
    'find-file': handleFindFile,
    'save-buffer': () => void handleSaveBuffer(),
    'switch-to-buffer': handleSwitchBuffer,
    'kill-buffer': handleKillBuffer,
    'list-buffers': handleListBuffers,
    'save-buffers-kill-terminal': () => void handleSaveBuffersKillTerminal(),
    'execute-extended-command': handleExecuteCommand,
    'eval-last-sexp': () => void handleEvalLastSexp(),
    'eval-expression': handleEvalExpression,
    'isearch-forward': handleIsearchForward,
    'isearch-backward': handleIsearchBackward,
    'goto-line': handleGotoLine,
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
      try {
        const result = await invoke<{ result: string; error: string | null }>('eval_scheme', {
          code: value,
        })

        if (result.error) {
          setMinibufferState({ mode: 'message', text: `Error: ${result.error}` })
        } else {
          setMinibufferState({ mode: 'message', text: result.result })
        }
      } catch (err) {
        setMinibufferState({ mode: 'message', text: `Eval error: ${err}` })
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

    if (current.mode === 'isearch') {
      editor.current?.setCursor(current.originalCursor)
    }
    setMinibufferState({ mode: 'message', text: 'Quit' })
  }

  return (
    <div className="app">
      <MenuBar />
      <CodeEditor
        bufferId={currentBufferId}
        initialContent={currentBuffer.content}
        onChange={handleEditorChange}
        onCursorChange={setCursor}
        actions={actions}
        onMessage={showMessage}
        disabled={minibufferState.mode !== 'message' || showBufferList}
        handle={editor}
      />
      <ModeLine
        bufferName={currentBuffer.name}
        modified={currentBuffer.modified}
        cursor={cursor}
        mode="Lisp Interaction"
        bufferCount={buffers.length}
        currentBufferIndex={buffers.findIndex((b) => b.id === currentBufferId) + 1}
      />
      <Minibuffer
        state={minibufferState}
        onInputChange={handleMinibufferInputChange}
        onSubmit={handleMinibufferSubmit}
        onCancel={handleMinibufferCancel}
        onIsearchRepeat={handleIsearchRepeat}
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
