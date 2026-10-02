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

export interface Buffer {
  id: string
  name: string
  filePath: string | null
  content: string
  modified: boolean
}

type PendingAction =
  'find-file' | 'write-file' | 'execute-command' | 'switch-buffer' | 'kill-buffer' | null

interface Command {
  name: string
  run: () => void | Promise<void>
}

function generateBufferId(): string {
  return `buffer-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`
}

function bufferNameFromPath(path: string | null): string {
  if (!path) return '*scratch*'
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
  const [highlightRange, setHighlightRange] = useState<{ start: number; end: number } | null>(null)
  const [undoStack, setUndoStack] = useState<{ text: string; cursorPos: number }[]>([])
  const [redoStack, setRedoStack] = useState<{ text: string; cursorPos: number }[]>([])

  const currentBuffer = buffers.find((b) => b.id === currentBufferId) || buffers[0]

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

  const killBuffer = (bufferId: string) => {
    const bufferToKill = buffers.find((b) => b.id === bufferId)
    if (!bufferToKill) return false

    if (bufferToKill.modified) {
      setMinibufferState({
        mode: 'message',
        text: `Buffer "${bufferToKill.name}" is modified. Use C-x C-s to save first.`,
      })
      return false
    }

    const newBuffers = buffers.filter((b) => b.id !== bufferId)
    if (newBuffers.length === 0) {
      // Create a new scratch buffer if all are killed
      const scratch = createScratchBuffer()
      setBuffers([scratch])
      setCurrentBufferId(scratch.id)
    } else {
      setBuffers(newBuffers)
      if (currentBufferId === bufferId) {
        setCurrentBufferId(newBuffers[0].id)
      }
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
    setRedoStack([])
  }

  const handlePushUndo = (currentCode: string, currentCursorPos: number) => {
    setUndoStack((prev) => [...prev, { text: currentCode, cursorPos: currentCursorPos }])
    setRedoStack([])
  }

  const handleUndo = () => {
    if (undoStack.length === 0) return
    const lastState = undoStack[undoStack.length - 1]
    setRedoStack((prev) => [...prev, { text: currentBuffer.content, cursorPos: cursor.offset }])
    updateCurrentBuffer({ content: lastState.text, modified: true })
    setCursor({
      line: 1,
      column: 1,
      offset: Math.min(lastState.cursorPos, lastState.text.length),
    })
    setUndoStack((prev) => prev.slice(0, -1))
  }

  const handleRedo = () => {
    if (redoStack.length === 0) return
    const nextState = redoStack[redoStack.length - 1]
    setUndoStack((prev) => [...prev, { text: currentBuffer.content, cursorPos: cursor.offset }])
    updateCurrentBuffer({ content: nextState.text, modified: true })
    setCursor({
      line: 1,
      column: 1,
      offset: Math.min(nextState.cursorPos, nextState.text.length),
    })
    setRedoStack((prev) => prev.slice(0, -1))
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
      killBuffer(currentBuffer.id)
    }
  }

  const handleSaveBuffersKillTerminal = async () => {
    if (currentBuffer.filePath) {
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
      const match = findNextMatch(
        currentBuffer.content,
        value,
        current.originalCursor,
        current.direction,
      )
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
      killBuffer(currentBuffer.id)
      setMinibufferState({ mode: 'message', text: '' })
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
        value={currentBuffer.content}
        onChange={handleEditorChange}
        onCursorChange={setCursor}
        onFindFile={handleFindFile}
        onSaveBuffer={handleSaveBuffer}
        onIsearchForward={handleIsearchForward}
        onIsearchBackward={handleIsearchBackward}
        onExecuteCommand={handleExecuteCommand}
        onQuit={handleQuit}
        onUndo={handleUndo}
        onRedo={handleRedo}
        onPushUndo={handlePushUndo}
        onSwitchBuffer={handleSwitchBuffer}
        onKillBuffer={handleKillBuffer}
        highlightRange={highlightRange}
        disabled={minibufferState.mode !== 'message'}
      />
      <ModeLine
        bufferName={currentBuffer.name}
        modified={currentBuffer.modified}
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
