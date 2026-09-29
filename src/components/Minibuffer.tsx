import { useEffect, useRef, KeyboardEvent } from 'react'
import '../styles/Minibuffer.css'

interface MinibufferMessageState {
  mode: 'message'
  text: string
}

interface MinibufferInputState {
  mode: 'input'
  prompt: string
  input: string
}

interface MinibufferIsearchState {
  mode: 'isearch'
  direction: 'forward' | 'backward'
  query: string
  originalCursor: number
  matchStart: number | null
  matchEnd: number | null
}

export type MinibufferState = MinibufferMessageState | MinibufferInputState | MinibufferIsearchState

interface MinibufferProps {
  state: MinibufferState
  onInputChange: (value: string) => void
  onSubmit: (value: string) => void
  onCancel: () => void
  onIsearchRepeat?: (direction: 'forward' | 'backward') => void
}

const DEFAULT_MESSAGE = 'For information about this editor, see the README.'

export default function Minibuffer({
  state,
  onInputChange,
  onSubmit,
  onCancel,
  onIsearchRepeat,
}: MinibufferProps) {
  const inputRef = useRef<HTMLInputElement>(null)

  useEffect(() => {
    if (state.mode === 'input' || state.mode === 'isearch') {
      inputRef.current?.focus()
    }
  }, [state.mode])

  if (state.mode === 'input' || state.mode === 'isearch') {
    const getPrompt = (): string => {
      if (state.mode === 'input') {
        return state.prompt
      }
      const prefix = state.direction === 'forward' ? 'I-search' : 'I-search backward'
      const failing = state.matchStart === null && state.query !== '' ? 'Failing ' : ''
      return `${failing}${prefix}: `
    }

    const getValue = (): string => {
      return state.mode === 'input' ? state.input : state.query
    }

    const handleKeyDown = (e: KeyboardEvent<HTMLInputElement>) => {
      if (state.mode === 'isearch') {
        if ((e.ctrlKey && e.key === 's') || (e.ctrlKey && e.key === 'r')) {
          e.preventDefault()
          const dir = e.key === 's' ? 'forward' : 'backward'
          onIsearchRepeat?.(dir)
          return
        }
      }

      if (e.key === 'Enter') {
        e.preventDefault()
        onSubmit(getValue())
      } else if (e.key === 'Escape' || (e.ctrlKey && e.key === 'g')) {
        e.preventDefault()
        onCancel()
      }
    }

    return (
      <div className="minibuffer">
        <span className="minibuffer-prompt">{getPrompt()}</span>
        <input
          ref={inputRef}
          className="minibuffer-input"
          value={getValue()}
          onChange={(e) => onInputChange(e.target.value)}
          onKeyDown={handleKeyDown}
          spellCheck={false}
        />
      </div>
    )
  }

  return <div className="minibuffer">{state.text || DEFAULT_MESSAGE}</div>
}
