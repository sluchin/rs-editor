import { useEffect, useRef, useState, KeyboardEvent } from 'react'
import { complete, type Completion } from '../lib/complete'
import '../styles/Minibuffer.css'

interface MinibufferMessageState {
  mode: 'message'
  text: string
}

export interface MinibufferInputState {
  mode: 'input'
  prompt: string
  input: string
  /** Tab で補完する候補 (無ければ, 補完しない). */
  completions?: readonly string[]
  /** Tab で補完する関数 (ファイルの名前など, 候補を入力から探すとき. `completions` より先に使う). */
  complete?: (input: string) => Promise<Completion>
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
  // Tab で補完したときの, 候補の一覧 (`[No match]` のときは null). 別の入力 (プロンプト) に移ったら, 出さない
  const inputKey = `${state.mode}:${state.mode === 'input' ? state.prompt : ''}`
  const [completion, setCompletion] = useState<{ key: string; list: string[] | null }>({
    key: '',
    list: [],
  })
  const shown = completion.key === inputKey ? completion.list : []
  const setShown = (list: string[] | null) => setCompletion({ key: inputKey, list })

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

      if (e.key === 'Tab' && state.mode === 'input' && (state.complete || state.completions)) {
        e.preventDefault()
        const show = ({ value, matches }: Completion) => {
          setShown(matches.length === 0 ? null : matches.length === 1 ? [] : matches)
          onInputChange(value, state)
        }
        if (state.complete) {
          void state.complete(state.input).then((result) => {
            // 補完を待つあいだに, 入力を変えたか, ミニバッファを閉じたなら, 捨てる
            if (inputRef.current?.value === state.input) show(result)
          })
        } else {
          show(complete(state.input, state.completions as readonly string[]))
        }
        return
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
          onChange={(e) => {
            setShown([])
            onInputChange(e.target.value, state)
          }}
          onKeyDown={handleKeyDown}
          spellCheck={false}
        />
        {shown === null && <span className="minibuffer-note">[No match]</span>}
        {shown !== null && shown.length > 0 && (
          <div className="minibuffer-completions">
            <div className="minibuffer-completions-title">Possible completions:</div>
            {shown.map((c) => (
              <div key={c}>{c}</div>
            ))}
          </div>
        )}
      </div>
    )
  }

  return <div className="minibuffer">{state.text || DEFAULT_MESSAGE}</div>
}
