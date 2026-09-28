import { RefObject, KeyboardEvent } from 'react'

interface BracketAutoCloseOptions {
  textareaRef: RefObject<HTMLTextAreaElement>
  value: string
  onChange: (value: string, cursorPos: number) => void
}

const PAIRS: Record<string, string> = {
  '(': ')',
  '"': '"',
}

export function useBracketAutoClose({ textareaRef, value, onChange }: BracketAutoCloseOptions) {
  const handleKeyDown = (e: KeyboardEvent<HTMLTextAreaElement>): boolean => {
    const el = textareaRef.current
    if (!el || e.ctrlKey || e.altKey || e.metaKey) return false
    const pos = el.selectionStart
    const selEnd = el.selectionEnd

    // Auto-insert closing paren/quote
    if (e.key in PAIRS) {
      e.preventDefault()
      const close = PAIRS[e.key]
      if (pos !== selEnd) {
        // wrap selection
        const selected = value.slice(pos, selEnd)
        onChange(
          value.slice(0, pos) + e.key + selected + close + value.slice(selEnd),
          pos + 1 + selected.length,
        )
      } else {
        onChange(value.slice(0, pos) + e.key + close + value.slice(pos), pos + 1)
      }
      return true
    }

    // Skip over closing paren/quote if it's already there (type-through)
    if ((e.key === ')' || e.key === '"') && value[pos] === e.key) {
      e.preventDefault()
      onChange(value, pos + 1)
      return true
    }

    // Backspace on empty pair removes both chars
    if (e.key === 'Backspace' && pos === selEnd && pos > 0) {
      const before = value[pos - 1]
      const after = value[pos]
      if (PAIRS[before] === after) {
        e.preventDefault()
        onChange(value.slice(0, pos - 1) + value.slice(pos + 1), pos - 1)
        return true
      }
    }

    return false
  }

  return { handleKeyDown }
}
