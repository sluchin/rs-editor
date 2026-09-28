import { RefObject, KeyboardEvent, useRef } from 'react'

interface EmacsKeymapOptions {
  textareaRef: RefObject<HTMLTextAreaElement>
  value: string
  onChange: (value: string, cursorPos: number) => void
  onSave?: () => void
}

function lineBounds(text: string, pos: number): { start: number; end: number } {
  const start = text.lastIndexOf('\n', pos - 1) + 1
  const end = text.indexOf('\n', pos)
  return { start, end: end === -1 ? text.length : end }
}

function wordForward(text: string, pos: number): number {
  let i = pos
  const n = text.length
  while (i < n && !/\w/.test(text[i])) i++
  while (i < n && /\w/.test(text[i])) i++
  return i
}

function wordBackward(text: string, pos: number): number {
  let i = pos
  while (i > 0 && !/\w/.test(text[i - 1])) i--
  while (i > 0 && /\w/.test(text[i - 1])) i--
  return i
}

export function useEmacsKeymap({ textareaRef, value, onChange, onSave }: EmacsKeymapOptions) {
  const killRing = useRef('')
  const markPos = useRef<number | null>(null)
  const awaitingPrefixX = useRef(false)

  const setCursor = (pos: number, extendSelection = false) => {
    const el = textareaRef.current
    if (!el) return
    if (extendSelection && markPos.current !== null) {
      if (markPos.current < pos) el.setSelectionRange(markPos.current, pos)
      else el.setSelectionRange(pos, markPos.current)
    } else {
      el.setSelectionRange(pos, pos)
    }
  }

  const handleKeyDown = (e: KeyboardEvent<HTMLTextAreaElement>) => {
    const el = textareaRef.current
    if (!el) return
    const pos = el.selectionStart
    const selEnd = el.selectionEnd
    const ctrl = e.ctrlKey
    const meta = e.altKey // Meta/Alt used for M- bindings

    // C-x C-s : save
    if (ctrl && e.key === 'x') {
      awaitingPrefixX.current = true
      e.preventDefault()
      return
    }
    if (awaitingPrefixX.current) {
      awaitingPrefixX.current = false
      if (ctrl && e.key === 's') {
        e.preventDefault()
        onSave?.()
        return
      }
    }

    if (ctrl && !meta) {
      switch (e.key) {
        case 'f': // forward-char
          e.preventDefault()
          setCursor(Math.min(value.length, pos + 1))
          return
        case 'b': // backward-char
          e.preventDefault()
          setCursor(Math.max(0, pos - 1))
          return
        case 'n': {
          // next-line
          e.preventDefault()
          const { end } = lineBounds(value, pos)
          setCursor(Math.min(value.length, end + 1))
          return
        }
        case 'p': {
          // previous-line
          e.preventDefault()
          const { start } = lineBounds(value, pos)
          if (start === 0) return
          const prevLineStart = value.lastIndexOf('\n', start - 2) + 1
          const col = pos - start
          setCursor(Math.min(prevLineStart + col, start - 1))
          return
        }
        case 'a': {
          // beginning-of-line
          e.preventDefault()
          const { start } = lineBounds(value, pos)
          setCursor(start)
          return
        }
        case 'e': {
          // end-of-line
          e.preventDefault()
          const { end } = lineBounds(value, pos)
          setCursor(end)
          return
        }
        case 'd': {
          // delete-char
          e.preventDefault()
          if (pos < value.length) {
            onChange(value.slice(0, pos) + value.slice(pos + 1), pos)
          }
          return
        }
        case 'k': {
          // kill-line
          e.preventDefault()
          const { end } = lineBounds(value, pos)
          const killEnd = pos === end ? Math.min(value.length, end + 1) : end
          killRing.current = value.slice(pos, killEnd)
          onChange(value.slice(0, pos) + value.slice(killEnd), pos)
          return
        }
        case 'y': {
          // yank
          e.preventDefault()
          const text = killRing.current
          onChange(value.slice(0, pos) + text + value.slice(selEnd), pos + text.length)
          return
        }
        case 'w': {
          // kill-region
          e.preventDefault()
          if (pos !== selEnd) {
            const [from, to] = pos < selEnd ? [pos, selEnd] : [selEnd, pos]
            killRing.current = value.slice(from, to)
            onChange(value.slice(0, from) + value.slice(to), from)
          }
          markPos.current = null
          return
        }
        case ' ': // set-mark
          e.preventDefault()
          markPos.current = pos
          return
        case 'g': // keyboard-quit
          e.preventDefault()
          markPos.current = null
          return
      }
    }

    if (meta && !ctrl) {
      switch (e.key) {
        case 'f': // forward-word
          e.preventDefault()
          setCursor(wordForward(value, pos))
          return
        case 'b': // backward-word
          e.preventDefault()
          setCursor(wordBackward(value, pos))
          return
        case 'w': // kill-ring-save (copy)
          e.preventDefault()
          if (pos !== selEnd) {
            const [from, to] = pos < selEnd ? [pos, selEnd] : [selEnd, pos]
            killRing.current = value.slice(from, to)
          }
          markPos.current = null
          return
      }
    }
  }

  return { handleKeyDown }
}
