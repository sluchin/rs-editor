import { useState, useEffect } from 'react'
import { getCurrentWindow } from '@tauri-apps/api/window'
import MenuBar from './components/MenuBar'
import CodeEditor, { CursorInfo } from './components/CodeEditor'
import ModeLine from './components/ModeLine'
import Minibuffer, { MinibufferState } from './components/Minibuffer'
import { readFile, writeFile, pathExists, getHomeDir } from './lib/fileOps'
import { findNextMatch } from './lib/isearch'
import './App.css'

const INITIAL_TEXT = `;; This buffer is for text that is not saved, and for Lisp evaluation.
;; To create a file, visit it with C-x C-f and enter text in its buffer.
`

type PendingAction = 'find-file' | 'write-file' | 'execute-command' | null

interface Command {
  name: string
  run: () => void | Promise<void>
}

function bufferNameFromPath(path: string | null): string {
  if (!path) return '*scratch*'
  const parts = path.split('/')
  return parts[parts.length - 1] || path
}

function App() {
  const [code, setCode] = useState(INITIAL_TEXT)
  const [modified, setModified] = useState(false)
  const [cursor, setCursor] = useState<CursorInfo>({ line: 1, column: 1, offset: 0 })
  const [filePath, setFilePath] = useState<string | null>(null)
  const [homeDir, setHomeDir] = useState('/')
  const [pendingAction, setPendingAction] = useState<PendingAction>(null)
  const [minibufferState, setMinibufferState] = useState<MinibufferState>({
    mode: 'message',
    text: '',
  })
  const [highlightRange, setHighlightRange] = useState<{ start: number; end: number } | null>(null)

  useEffect(() => {
    getHomeDir()
      .then((dir) => setHomeDir(dir.endsWith('/') ? dir : dir + '/'))
      .catch(() => {})
  }, [])

  const handleEditorChange = (value: string) => {
    setCode(value)
    setModified(true)
  }

  const handleFindFile = () => {
    setPendingAction('find-file')
    setMinibufferState({ mode: 'input', prompt: 'Find file: ', input: homeDir })
  }

  const handleSaveBuffer = async () => {
    if (filePath) {
      try {
        await writeFile(filePath, code)
        setModified(false)
        setMinibufferState({ mode: 'message', text: `Wrote ${filePath}` })
      } catch (err) {
        setMinibufferState({ mode: 'message', text: `Error: ${err}` })
      }
    } else {
      setPendingAction('write-file')
      setMinibufferState({ mode: 'input', prompt: 'Save file: ', input: homeDir })
    }
  }

  const handleSaveBuffersKillTerminal = async () => {
    if (filePath) {
      await handleSaveBuffer()
      await getCurrentWindow().close()
    } else {
      handleSaveBuffer()
    }
  }

  const handleIsearchForward = () => {
    setHighlightRange(null)
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
    setHighlightRange(null)
    setMinibufferState({
      mode: 'isearch',
      direction: 'backward',
      query: '',
      originalCursor: cursor.offset,
      matchStart: null,
      matchEnd: null,
    })
  }

  const handleIsearchRepeat = (direction: 'forward' | 'backward') => {
    const current = minibufferState
    if (current.mode !== 'isearch') return

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

    const match = findNextMatch(code, current.query, fromPos, direction)

    if (match) {
      setHighlightRange(match)
      setMinibufferState({
        ...current,
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

  const handleQuit = () => {
    setMinibufferState({ mode: 'message', text: 'Quit' })
  }

  const handleMinibufferInputChange = (value: string) => {
    const current = minibufferState
    if (current.mode === 'input') {
      setMinibufferState({ ...current, input: value })
    } else if (current.mode === 'isearch') {
      const match = findNextMatch(code, value, current.originalCursor, current.direction)
      if (match) {
        setHighlightRange(match)
        setMinibufferState({
          ...current,
          query: value,
          matchStart: match.start,
          matchEnd: match.end,
        })
      } else {
        setHighlightRange(null)
        setMinibufferState({ ...current, query: value, matchStart: null, matchEnd: null })
      }
    }
  }

  const commands: Command[] = [
    { name: 'find-file', run: handleFindFile },
    { name: 'save-buffer', run: handleSaveBuffer },
    { name: 'save-buffers-kill-terminal', run: handleSaveBuffersKillTerminal },
  ]

  const handleMinibufferSubmit = async (value: string) => {
    const action = pendingAction
    const currentState = minibufferState
    setPendingAction(null)

    if (action === 'find-file') {
      try {
        const exists = await pathExists(value)
        if (exists) {
          const content = await readFile(value)
          setCode(content)
          setFilePath(value)
          setModified(false)
          setMinibufferState({ mode: 'message', text: '' })
        } else {
          setCode('')
          setFilePath(value)
          setModified(false)
          setMinibufferState({ mode: 'message', text: '(New file)' })
        }
      } catch (err) {
        setMinibufferState({ mode: 'message', text: `Error: ${err}` })
      }
    } else if (action === 'write-file') {
      try {
        await writeFile(value, code)
        setFilePath(value)
        setModified(false)
        setMinibufferState({ mode: 'message', text: `Wrote ${value}` })
      } catch (err) {
        setMinibufferState({ mode: 'message', text: `Error: ${err}` })
      }
    } else if (action === 'execute-command') {
      const cmd = commands.find((c) => c.name === value.trim())
      if (cmd) {
        await cmd.run()
      } else {
        setMinibufferState({ mode: 'message', text: `Undefined command: ${value}` })
      }
    } else if (currentState.mode === 'isearch') {
      setHighlightRange(null)
      setMinibufferState({ mode: 'message', text: '' })
    }
  }

  const handleMinibufferCancel = () => {
    const current = minibufferState
    setPendingAction(null)

    if (current.mode === 'isearch') {
      setHighlightRange({ start: current.originalCursor, end: current.originalCursor })
      setMinibufferState({ mode: 'message', text: 'Quit' })
    } else {
      setMinibufferState({ mode: 'message', text: 'Quit' })
    }
  }

  return (
    <div className="app">
      <MenuBar />
      <CodeEditor
        value={code}
        onChange={handleEditorChange}
        onCursorChange={setCursor}
        onFindFile={handleFindFile}
        onSaveBuffer={handleSaveBuffer}
        onIsearchForward={handleIsearchForward}
        onIsearchBackward={handleIsearchBackward}
        onExecuteCommand={handleExecuteCommand}
        onQuit={handleQuit}
        highlightRange={highlightRange}
        disabled={minibufferState.mode !== 'message'}
      />
      <ModeLine
        bufferName={bufferNameFromPath(filePath)}
        modified={modified}
        cursor={cursor}
        mode="Lisp Interaction"
      />
      <Minibuffer
        state={minibufferState}
        onInputChange={handleMinibufferInputChange}
        onSubmit={handleMinibufferSubmit}
        onCancel={handleMinibufferCancel}
        onIsearchRepeat={handleIsearchRepeat}
      />
    </div>
  )
}

export default App
