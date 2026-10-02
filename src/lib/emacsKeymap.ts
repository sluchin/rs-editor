import { RefObject, KeyboardEvent, useRef } from 'react'
import { forwardSexp, backwardSexp, killSexp } from './bracketMatch'

interface EmacsKeymapOptions {
  textareaRef: RefObject<HTMLTextAreaElement>
  value: string
  onChange: (value: string, cursorPos: number) => void
  onIsearchForward?: () => void
  onIsearchBackward?: () => void
  onExecuteCommand?: () => void
  onQuit?: () => void
  onUndo?: () => void
  onPushUndo?: (currentCode: string, currentCursorPos: number) => void
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

export function useEmacsKeymap({
  textareaRef,
  value,
  onChange,
  onIsearchForward,
  onIsearchBackward,
  onExecuteCommand,
  onQuit,
  onUndo,
  onPushUndo,
}: EmacsKeymapOptions) {
  const killRing = useRef<string[]>([])
  const killRingIndex = useRef(0)
  const markPos = useRef<number | null>(null)

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
          onPushUndo?.(value, pos)
          const { end } = lineBounds(value, pos)
          const killEnd = pos === end ? Math.min(value.length, end + 1) : end
          const killed = value.slice(pos, killEnd)
          killRing.current.unshift(killed)
          if (killRing.current.length > 50) killRing.current.pop()
          killRingIndex.current = 0
          onChange(value.slice(0, pos) + value.slice(killEnd), pos)
          return
        }
        case 'y': {
          // yank
          e.preventDefault()
          const text = killRing.current[killRingIndex.current] || ''
          onChange(value.slice(0, pos) + text + value.slice(selEnd), pos + text.length)
          return
        }
        case 'w': {
          // kill-region
          e.preventDefault()
          onPushUndo?.(value, pos)
          if (pos !== selEnd) {
            const [from, to] = pos < selEnd ? [pos, selEnd] : [selEnd, pos]
            const killed = value.slice(from, to)
            killRing.current.unshift(killed)
            if (killRing.current.length > 50) killRing.current.pop()
            killRingIndex.current = 0
            onChange(value.slice(0, from) + value.slice(to), from)
          }
          markPos.current = null
          return
        }
        case '/': {
          // undo
          e.preventDefault()
          onPushUndo?.(value, pos)
          onUndo?.()
          return
        }
        case 't': {
          // transpose-chars
          e.preventDefault()
          if (value.length < 2) return
          let start = pos - 1
          let end = pos
          if (pos === 0) {
            start = 0
            end = 2
          } else if (pos === value.length) {
            start = value.length - 2
            end = value.length
          }
          if (start >= 0 && end <= value.length) {
            const char1 = value[start]
            const char2 = value[end - 1]
            const newValue =
              value.slice(0, start) +
              char2 +
              value.slice(start + 1, end - 1) +
              char1 +
              value.slice(end)
            onPushUndo?.(value, pos)
            onChange(newValue, Math.min(pos + 1, newValue.length))
          }
          return
        }
        case 's': // isearch-forward
          e.preventDefault()
          onIsearchForward?.()
          return
        case 'r': // isearch-backward
          e.preventDefault()
          onIsearchBackward?.()
          return
        case ' ': // set-mark
          e.preventDefault()
          markPos.current = pos
          return
        case 'g': // keyboard-quit
          e.preventDefault()
          markPos.current = null
          onQuit?.()
          return
      }
    }

    if (ctrl && meta) {
      switch (e.key) {
        case 'f': {
          // forward-sexp
          e.preventDefault()
          setCursor(forwardSexp(value, pos))
          return
        }
        case 'b': {
          // backward-sexp
          e.preventDefault()
          setCursor(backwardSexp(value, pos))
          return
        }
        case 'k': {
          // kill-sexp
          e.preventDefault()
          onPushUndo?.(value, pos)
          const [newText, newPos] = killSexp(value, pos)
          killRing.current.unshift(value.slice(pos, pos + (value.length - newText.length)))
          if (killRing.current.length > 50) killRing.current.pop()
          killRingIndex.current = 0
          onChange(newText, newPos)
          return
        }
      }
    }

    if (meta && !ctrl) {
      switch (e.key) {
        case 'x': // execute-extended-command
          e.preventDefault()
          onExecuteCommand?.()
          return
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
            killRing.current.unshift(value.slice(from, to))
            if (killRing.current.length > 50) killRing.current.pop()
            killRingIndex.current = 0
          }
          markPos.current = null
          return
        case 'y': {
          // yank-pop
          e.preventDefault()
          if (killRing.current.length === 0) return
          killRingIndex.current = (killRingIndex.current + 1) % killRing.current.length
          const text = killRing.current[killRingIndex.current]
          onChange(value.slice(0, pos) + text + value.slice(selEnd), pos + text.length)
          return
        }
        case '<': {
          // buffer-start
          e.preventDefault()
          setCursor(0)
          return
        }
        case '>': {
          // buffer-end
          e.preventDefault()
          setCursor(value.length)
          return
        }
      }
    }
  }

  return { handleKeyDown }
}
