import { useState, useEffect } from 'react'
import MenuBar from './components/MenuBar'
import CodeEditor, { CursorInfo } from './components/CodeEditor'
import ModeLine from './components/ModeLine'
import Minibuffer, { MinibufferState } from './components/Minibuffer'
import { readFile, writeFile, pathExists, getHomeDir } from './lib/fileOps'
import './App.css'

const INITIAL_TEXT = `;; This buffer is for text that is not saved, and for Lisp evaluation.
;; To create a file, visit it with C-x C-f and enter text in its buffer.
`

type PendingAction = 'find-file' | 'write-file' | null

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

  const handleMinibufferInputChange = (value: string) => {
    setMinibufferState((prev) => (prev.mode === 'input' ? { ...prev, input: value } : prev))
  }

  const handleMinibufferSubmit = async (value: string) => {
    const action = pendingAction
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
    }
  }

  const handleMinibufferCancel = () => {
    setPendingAction(null)
    setMinibufferState({ mode: 'message', text: 'Quit' })
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
        disabled={minibufferState.mode === 'input'}
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
      />
    </div>
  )
}

export default App
