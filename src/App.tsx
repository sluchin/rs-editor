import { useState } from 'react'
import MenuBar from './components/MenuBar'
import CodeEditor, { CursorInfo } from './components/CodeEditor'
import ModeLine from './components/ModeLine'
import Minibuffer from './components/Minibuffer'
import './App.css'

const INITIAL_TEXT = `;; This buffer is for text that is not saved, and for Lisp evaluation.
;; To create a file, visit it with C-x C-f and enter text in its buffer.
`

function App() {
  const [code, setCode] = useState(INITIAL_TEXT)
  const [modified, setModified] = useState(false)
  const [cursor, setCursor] = useState<CursorInfo>({ line: 1, column: 1, offset: 0 })

  const handleEditorChange = (value: string) => {
    setCode(value)
    setModified(true)
  }

  return (
    <div className="app">
      <MenuBar />
      <CodeEditor value={code} onChange={handleEditorChange} onCursorChange={setCursor} />
      <ModeLine bufferName="*scratch*" modified={modified} cursor={cursor} mode="Lisp Interaction" />
      <Minibuffer />
    </div>
  )
}

export default App
