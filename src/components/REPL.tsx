import { useState } from 'react'
import '../styles/REPL.css'

interface REPLProps {
  output: string
  error: string
  history: { code: string; result: string }[]
  onEval: (code: string) => void
}

export default function REPL({ output, error, history, onEval }: REPLProps) {
  const [input, setInput] = useState('')
  const [historyIndex, setHistoryIndex] = useState(-1)

  const handleSubmit = () => {
    if (input.trim()) {
      onEval(input)
      setInput('')
      setHistoryIndex(-1)
    }
  }

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault()
      handleSubmit()
    } else if (e.key === 'ArrowUp') {
      e.preventDefault()
      const nextIndex = historyIndex + 1
      if (nextIndex < history.length) {
        setHistoryIndex(nextIndex)
        setInput(history[history.length - 1 - nextIndex].code)
      }
    } else if (e.key === 'ArrowDown') {
      e.preventDefault()
      if (historyIndex > 0) {
        const nextIndex = historyIndex - 1
        setHistoryIndex(nextIndex)
        setInput(history[history.length - 1 - nextIndex].code)
      } else if (historyIndex === 0) {
        setHistoryIndex(-1)
        setInput('')
      }
    }
  }

  return (
    <div className="repl">
      <div className="repl-output">
        {error ? (
          <div className="error-message">Error: {error}</div>
        ) : output ? (
          <div className="output-message">{output}</div>
        ) : (
          <div className="empty-message">Ready</div>
        )}
      </div>
      <div className="repl-history">
        <h3>History</h3>
        <div className="history-list">
          {history.slice(-10).reverse().map((item, idx) => (
            <div key={idx} className="history-item">
              <div className="history-code">{item.code}</div>
              <div className="history-result">{item.result}</div>
            </div>
          ))}
        </div>
      </div>
      <div className="repl-input">
        <input
          type="text"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder="Enter Scheme code..."
          className="input-field"
        />
        <button onClick={handleSubmit} className="btn-submit">
          Eval
        </button>
      </div>
    </div>
  )
}
