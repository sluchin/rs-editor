import { useState } from 'react'
import '../styles/Editor.css'

interface EditorProps {
  value: string
  onChange: (value: string) => void
  onRun: () => void
}

export default function Editor({ value, onChange, onRun }: EditorProps) {
  return (
    <div className="editor">
      <div className="editor-toolbar">
        <button onClick={onRun} className="btn-run">
          ▶ Run (Ctrl+Enter)
        </button>
      </div>
      <textarea
        className="editor-textarea"
        value={value}
        onChange={(e) => onChange(e.target.value)}
        onKeyDown={(e) => {
          if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
            onRun()
          }
        }}
        spellCheck="false"
      />
    </div>
  )
}
