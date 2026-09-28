import { useRef, useState, useEffect, KeyboardEvent } from 'react'
import { usePrefixKeymap } from '../lib/prefixKeymap'
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
  onFindFile: () => void
  onSaveBuffer: () => void
  disabled?: boolean
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

export default function CodeEditor({
  value,
  onChange,
  onCursorChange,
  onFindFile,
  onSaveBuffer,
  disabled,
}: CodeEditorProps) {
  const textareaRef = useRef<HTMLTextAreaElement>(null)
  const [cursorPos, setCursorPos] = useState(0)

  const { handleKeyDown: prefixKeyDown } = usePrefixKeymap({ onFindFile, onSaveBuffer })

  const updateCursorFromEl = () => {
    const el = textareaRef.current
    if (!el) return
    setCursorPos(el.selectionStart)
  }

  useEffect(() => {
    onCursorChange?.(cursorInfoFromOffset(value, cursorPos))
  }, [value, cursorPos, onCursorChange])

  const handleKeyDown = (e: KeyboardEvent<HTMLTextAreaElement>) => {
    prefixKeyDown(e)
  }

  return (
    <textarea
      ref={textareaRef}
      className="code-editor-textarea"
      value={value}
      onChange={(e) => {
        onChange(e.target.value)
        setCursorPos(e.target.selectionStart)
      }}
      onKeyDown={handleKeyDown}
      onKeyUp={updateCursorFromEl}
      onClick={updateCursorFromEl}
      readOnly={disabled}
      spellCheck={false}
      autoFocus
    />
  )
}
