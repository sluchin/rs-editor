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

export type MinibufferState = MinibufferMessageState | MinibufferInputState

interface MinibufferProps {
  state: MinibufferState
  onInputChange: (value: string) => void
  onSubmit: (value: string) => void
  onCancel: () => void
}

const DEFAULT_MESSAGE = 'For information about this editor, see the README.'

export default function Minibuffer({ state, onInputChange, onSubmit, onCancel }: MinibufferProps) {
  const inputRef = useRef<HTMLInputElement>(null)

  useEffect(() => {
    if (state.mode === 'input') {
      inputRef.current?.focus()
    }
  }, [state.mode])

  if (state.mode === 'input') {
    const handleKeyDown = (e: KeyboardEvent<HTMLInputElement>) => {
      if (e.key === 'Enter') {
        e.preventDefault()
        onSubmit(state.input)
      } else if (e.key === 'Escape' || (e.ctrlKey && e.key === 'g')) {
        e.preventDefault()
        onCancel()
      }
    }

    return (
      <div className="minibuffer">
        <span className="minibuffer-prompt">{state.prompt}</span>
        <input
          ref={inputRef}
          className="minibuffer-input"
          value={state.input}
          onChange={(e) => onInputChange(e.target.value)}
          onKeyDown={handleKeyDown}
          spellCheck={false}
        />
      </div>
    )
  }

  return <div className="minibuffer">{state.text || DEFAULT_MESSAGE}</div>
}
