import {
  useRef,
  useState,
  useMemo,
  useEffect,
  useLayoutEffect,
  KeyboardEvent,
  UIEvent,
} from 'react'
import { usePrefixKeymap } from '../lib/prefixKeymap'
import { useEmacsKeymap } from '../lib/emacsKeymap'
import { tokenize, Token, TokenType } from '../lib/schemeTokenizer'
import { findMatchingBracket, findUnmatchedClose } from '../lib/bracketMatch'
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

const TOKEN_CLASS: Record<TokenType, string> = {
  paren: 'token-paren',
  keyword: 'token-keyword',
  string: 'token-string',
  comment: 'token-comment',
  number: 'token-number',
  boolean: 'token-boolean',
  symbol: 'token-symbol',
  whitespace: 'token-whitespace',
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

function renderTokens(
  tokens: Token[],
  matched: { open: number; close: number } | null,
  unmatchedClose: number | null,
) {
  return tokens.map((t, i) => {
    let className = TOKEN_CLASS[t.type]
    if (t.type === 'paren') {
      if (matched && (t.start === matched.open || t.start === matched.close)) {
        className += ' token-paren-matched'
      }
      if (t.start === unmatchedClose) {
        className += ' token-paren-unmatched'
      }
    }
    return (
      <span key={i} className={className}>
        {t.value}
      </span>
    )
  })
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
  const highlightRef = useRef<HTMLPreElement>(null)
  const [cursorPos, setCursorPos] = useState(0)
  const pendingCursorRef = useRef<number | null>(null)

  const applyProgrammaticChange = (newValue: string, pos: number) => {
    pendingCursorRef.current = pos
    onChange(newValue)
    setCursorPos(pos)
  }

  const { handleKeyDown: prefixKeyDown } = usePrefixKeymap({ onFindFile, onSaveBuffer })
  const { handleKeyDown: emacsKeyDown } = useEmacsKeymap({
    textareaRef,
    value,
    onChange: applyProgrammaticChange,
  })

  const tokens = useMemo(() => tokenize(value), [value])
  const matched = useMemo(() => findMatchingBracket(value, cursorPos), [value, cursorPos])
  const unmatchedClose = useMemo(() => findUnmatchedClose(value), [value])

  const updateCursorFromEl = () => {
    const el = textareaRef.current
    if (!el) return
    setCursorPos(el.selectionStart)
  }

  useEffect(() => {
    onCursorChange?.(cursorInfoFromOffset(value, cursorPos))
  }, [value, cursorPos, onCursorChange])

  useLayoutEffect(() => {
    if (pendingCursorRef.current === null) return
    textareaRef.current?.setSelectionRange(pendingCursorRef.current, pendingCursorRef.current)
    pendingCursorRef.current = null
  }, [value, cursorPos])

  const handleKeyDown = (e: KeyboardEvent<HTMLTextAreaElement>) => {
    if (prefixKeyDown(e)) return
    emacsKeyDown(e)
  }

  const handleScroll = (e: UIEvent<HTMLTextAreaElement>) => {
    const pre = highlightRef.current
    if (!pre) return
    pre.scrollTop = e.currentTarget.scrollTop
    pre.scrollLeft = e.currentTarget.scrollLeft
  }

  return (
    <div className="code-editor-container">
      <pre className="code-editor-highlight" aria-hidden="true" ref={highlightRef}>
        {renderTokens(tokens, matched, unmatchedClose)}
        {value.endsWith('\n') ? ' ' : null}
      </pre>
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
        onScroll={handleScroll}
        readOnly={disabled}
        spellCheck={false}
        autoFocus
      />
    </div>
  )
}
