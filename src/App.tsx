import { useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import Editor from './components/Editor'
import REPL from './components/REPL'
import './App.css'

function App() {
  const [code, setCode] = useState('(+ 1 2 3)')
  const [output, setOutput] = useState('')
  const [error, setError] = useState('')
  const [history, setHistory] = useState<{ code: string; result: string }[]>([])

  const handleEval = async (evalCode: string) => {
    try {
      const result = await invoke<{ result: string; error: string | null }>('eval_scheme', {
        code: evalCode,
      })

      if (result.error) {
        setError(result.error)
        setOutput('')
      } else {
        setOutput(result.result)
        setError('')
        setHistory([...history, { code: evalCode, result: result.result }])
      }
    } catch (err) {
      setError(String(err))
    }
  }

  const handleEditorChange = (value: string) => {
    setCode(value)
  }

  const handleRun = () => {
    handleEval(code)
  }

  return (
    <div className="app">
      <header className="app-header">
        <h1>Scheme Editor</h1>
      </header>
      <div className="app-container">
        <div className="editor-panel">
          <h2>Editor</h2>
          <Editor value={code} onChange={handleEditorChange} onRun={handleRun} />
        </div>
        <div className="repl-panel">
          <h2>Output</h2>
          <REPL output={output} error={error} history={history} onEval={handleEval} />
        </div>
      </div>
    </div>
  )
}

export default App
