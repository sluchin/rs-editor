import { useEffect, useRef, KeyboardEvent } from 'react'
import '../styles/Minibuffer.css'

interface MinibufferMessageState {
  mode: 'message'
  text: string
}

export interface MinibufferInputState {
  mode: 'input'
  prompt: string
  input: string
}

export interface MinibufferIsearchState {
  mode: 'isearch'
  direction: 'forward' | 'backward'
  /** 正規表現で探すなら `true`. */
  regexp: boolean
  query: string
  originalCursor: number
  matchStart: number | null
  matchEnd: number | null
}

/** 1 つのキーで答える問い (query-replace の `y`, `n` など). */
export interface MinibufferQueryState {
  mode: 'query'
  prompt: string
}

export type MinibufferState =
  MinibufferMessageState | MinibufferInputState | MinibufferIsearchState | MinibufferQueryState

interface MinibufferProps {
  state: MinibufferState
  onInputChange: (value: string, state: MinibufferInputState | MinibufferIsearchState) => void
  onSubmit: (value: string) => void
  onCancel: () => void
  onIsearchRepeat?: (direction: 'forward' | 'backward', state: MinibufferIsearchState) => void
  /** 問いに答えたキー (`y`, `n`, `!` など). */
  onQueryKey?: (key: string) => void
}

const DEFAULT_MESSAGE = 'For information about this editor, see the README.'

export default function Minibuffer({
  state,
  onInputChange,
  onSubmit,
  onCancel,
  onIsearchRepeat,
  onQueryKey,
}: MinibufferProps) {
  const inputRef = useRef<HTMLInputElement>(null)

  useEffect(() => {
    if (state.mode !== 'message') {
      inputRef.current?.focus()
    }
  }, [state.mode])

  if (state.mode === 'query') {
    const handleQueryKey = (e: KeyboardEvent<HTMLInputElement>) => {
      e.preventDefault()
      if (e.key === 'Escape' || (e.ctrlKey && e.key === 'g')) {
        onCancel()
      } else {
        onQueryKey?.(e.key)
      }
    }
    return (
      <div className="minibuffer">
        <span className="minibuffer-prompt">{state.prompt}</span>
        <input
          ref={inputRef}
          className="minibuffer-input"
          value=""
          readOnly
          onKeyDown={handleQueryKey}
        />
      </div>
    )
  }

  if (state.mode === 'input' || state.mode === 'isearch') {
    const getPrompt = (): string => {
      if (state.mode === 'input') {
        return state.prompt
      }
      const prefix = state.direction === 'forward' ? 'I-search' : 'I-search backward'
      const failing = state.matchStart === null && state.query !== '' ? 'Failing ' : ''
      const regexp = state.regexp ? 'Regexp ' : ''
      return `${failing}${regexp}${prefix}: `
    }

    const getValue = (): string => {
      return state.mode === 'input' ? state.input : state.query
    }

    const handleKeyDown = (e: KeyboardEvent<HTMLInputElement>) => {
      if (state.mode === 'isearch') {
        if ((e.ctrlKey && e.key === 's') || (e.ctrlKey && e.key === 'r')) {
          e.preventDefault()
          const dir = e.key === 's' ? 'forward' : 'backward'
          onIsearchRepeat?.(dir, state)
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
          onChange={(e) => onInputChange(e.target.value, state)}
          onKeyDown={handleKeyDown}
          spellCheck={false}
        />
      </div>
    )
  }

  return <div className="minibuffer">{state.text || DEFAULT_MESSAGE}</div>
}
