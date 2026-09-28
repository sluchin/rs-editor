import { useRef, useState, useEffect } from 'react'
import '../styles/CodeEditor.css'

export interface CursorInfo {
  line: number
  column: number
  offset: number
}

interface CodeEditorProps {
  value: string
  onChange: (value: string) => void
  onCursorChange?: (info: CursorInfo) => void
}

function cursorInfoFromOffset(text: string, offset: number): CursorInfo {
  const before = text.slice(0, offset)
  const lines = before.split('\n')
  return {
    line: lines.length,
    column: lines[lines.length - 1].length + 1,
    offset,
  }
}

export default function CodeEditor({ value, onChange, onCursorChange }: CodeEditorProps) {
  const textareaRef = useRef<HTMLTextAreaElement>(null)
  const [cursorPos, setCursorPos] = useState(0)

  const updateCursorFromEl = () => {
    const el = textareaRef.current
    if (!el) return
    setCursorPos(el.selectionStart)
  }

  useEffect(() => {
    onCursorChange?.(cursorInfoFromOffset(value, cursorPos))
  }, [value, cursorPos, onCursorChange])

  return (
    <textarea
      ref={textareaRef}
      className="code-editor-textarea"
      value={value}
      onChange={(e) => {
        onChange(e.target.value)
        setCursorPos(e.target.selectionStart)
      }}
      onKeyUp={updateCursorFromEl}
      onClick={updateCursorFromEl}
      spellCheck={false}
      autoFocus
    />
  )
}
