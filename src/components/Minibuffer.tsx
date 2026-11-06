/**
 * ミニバッファ (画面の下の 1 行). メッセージ, 入力 (補完付き), 1 つのキーで答える問い (`read-key`. isearch も) を出します.
 */

import { useEffect, useRef, useState, KeyboardEvent } from 'react'
import { emacsKey } from '../lib/keys'
import { complete, type Completion } from '../lib/complete'
import { historyText, searchHistory, stepHistory } from '../lib/history'
import '../styles/Minibuffer.css'

/** メッセージを出している状態. */
interface MinibufferMessageState {
  mode: 'message'
  /** メッセージ (空なら, 既定のメッセージ). */
  text: string
}

/** 入力を待っている状態. */
export interface MinibufferInputState {
  mode: 'input'
  /** プロンプト. */
  prompt: string
  /** 入力. */
  input: string
  /** Tab で補完する候補 (無ければ, 補完しない). */
  completions?: readonly string[]
  /** Tab で補完する関数 (ファイルの名前など, 候補を入力から探すとき. `completions` より先に使う). */
  complete?: (input: string) => Promise<Completion>
  /** M-p と M-n で出す, 入力の履歴 (新しい順). */
  history?: readonly string[]
  /** 出している履歴の位置 (1 から数える. 0 なら, 入力していた文字). */
  historyPos?: number
  /** 履歴を出す前に, 入力していた文字. */
  historySaved?: string
  /** `M-n` で履歴の先に出す, 既定の値 (位置 -1, -2, ...). */
  defaults?: readonly string[]
  /** `M-r` / `M-s` で正規表現を入力しているとき (`older` は古い方へ探すか). */
  historySearch?: { older: boolean }
  /** `M-r` / `M-s` で最後に探した正規表現. */
  historyRegexp?: string
}

/** 1 つのキーで答える問い (`read-key`. query-replace の `y`, `n`, isearch の文字など). */
export interface MinibufferQueryState {
  mode: 'query'
  /** プロンプト. */
  prompt: string
}

/** ミニバッファの状態. */
export type MinibufferState = MinibufferMessageState | MinibufferInputState | MinibufferQueryState

/** {@link Minibuffer} のプロパティ. */
interface MinibufferProps {
  /** ミニバッファの状態. */
  state: MinibufferState
  /** 入力が変わったときに呼びます (Tab で補完したときも). */
  onInputChange: (value: string, state: MinibufferInputState) => void
  /** Enter で入力を確定したときに呼びます. */
  onSubmit: (value: string) => void
  /** C-g (`abort-minibuffers`), C-] (`abort-recursive-edit`) か Escape で, やめたときに呼びます. */
  onCancel: () => void
  /** 問いに答えたキー (Emacs の書き方. `y`, `SPC`, `C-g` など). */
  onQueryKey: (key: string) => void
}

/**
 * ミニバッファをやめるキーなら `true` (Emacs の `C-g` (`abort-minibuffers`) と `C-]` (`abort-recursive-edit`). Escape も).
 *
 * @param e - キーのイベント.
 * @returns やめるキーなら `true`.
 */
function isQuitKey(e: KeyboardEvent<HTMLInputElement>): boolean {
  return e.key === 'Escape' || (e.ctrlKey && (e.key === 'g' || e.key === ']'))
}

/** メッセージが空のときに出す, 既定のメッセージ. */
const DEFAULT_MESSAGE = 'For information about this editor, see the README.'

/**
 * ミニバッファを出します.
 *
 * @param props - プロパティ.
 * @returns ミニバッファ.
 */
export default function Minibuffer({
  state,
  onInputChange,
  onSubmit,
  onCancel,
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
  // 入力の後ろに出す知らせ (履歴の端). 入力を変えたら, 出さない
  const [note, setNote] = useState({ key: '', text: '' })
  const shownNote = note.key === inputKey ? note.text : ''

  // 入力を待つときは, 入力欄にフォーカスを移す
  useEffect(() => {
    if (state.mode !== 'message') {
      inputRef.current?.focus()
    }
  }, [state.mode])

  if (state.mode === 'query') {
    /**
     * 問いに答えたキーを, Emacs の書き方で返します (C-g も. やめるかは, エディタが決める).
     * 修飾キーだけなら, 次のキーを待ちます. 日本語の変換中のキーは, IME に任せます.
     *
     * @param e - キーのイベント.
     */
    const handleQueryKey = (e: KeyboardEvent<HTMLInputElement>) => {
      if (e.nativeEvent.isComposing || e.keyCode === 229) return
      const key = emacsKey(e.nativeEvent)
      if (key === null) return
      e.preventDefault()
      onQueryKey(key)
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

  if (state.mode === 'input') {
    /**
     * 入力欄のキーを受けます. Tab での補完, Enter での確定, C-g / C-] / Escape でやめる.
     *
     * @param e - キーのイベント.
     */
    /**
     * 履歴の位置を動かして, その文字を入力欄に出します.
     *
     * @param pos - 新しい位置.
     * @param items - 履歴.
     * @param defaults - 既定の値.
     * @param saved - 履歴を出す前に入力していた文字.
     * @param base - 入力の状態 (省くと, 今の状態).
     */
    const showHistory = (
      pos: number,
      items: readonly string[],
      defaults: readonly string[],
      saved: string,
      base: MinibufferInputState = state,
    ) => {
      const text = historyText(items, defaults, saved, pos)
      setShown([])
      setNote({ key: '', text: '' })
      onInputChange(text, { ...base, historyPos: pos, historySaved: saved })
    }

    const handleKeyDown = (e: KeyboardEvent<HTMLInputElement>) => {
      // 日本語などの変換中のキー (変換を確定する Enter も) は, IME に任せる
      // (WebKit は, 確定の Enter で isComposing が false でも, keyCode を 229 にする)
      if (e.nativeEvent.isComposing || e.keyCode === 229) return

      // 入力の履歴 (M-p で古い方へ, M-n で新しい方へ. 履歴の先では既定の値)
      if (e.altKey && (e.key === 'p' || e.key === 'n') && !state.historySearch) {
        e.preventDefault()
        const items = state.history ?? []
        const defaults = state.defaults ?? []
        const current = state.historyPos ?? 0
        const step = stepHistory(items, defaults, current, e.key === 'p')
        if ('error' in step) {
          setNote({ key: inputKey, text: `[${step.error}]` })
        } else {
          showHistory(
            step.pos,
            items,
            defaults,
            current === 0 ? state.input : (state.historySaved as string),
          )
        }
        return
      }

      // 入力の履歴を正規表現で探す (M-r で古い方へ, M-s で新しい方へ). 正規表現はこの入力欄に打つ
      if (e.altKey && (e.key === 'r' || e.key === 's') && !state.historySearch) {
        e.preventDefault()
        const saved = (state.historyPos ?? 0) === 0 ? state.input : (state.historySaved as string)
        setNote({ key: '', text: '' })
        onInputChange('', {
          ...state,
          historySearch: { older: e.key === 'r' },
          historySaved: saved,
        })
        return
      }
      if (state.historySearch && e.key === 'Enter') {
        e.preventDefault()
        const regexp = state.input === '' ? (state.historyRegexp ?? '') : state.input
        const items = state.history ?? []
        const current = state.historyPos ?? 0
        const restored = { ...state, historySearch: undefined, historyRegexp: regexp }
        const step = searchHistory(items, current, regexp, state.historySearch.older)
        if ('error' in step) {
          setNote({ key: inputKey, text: `[${step.error}]` })
          onInputChange(
            historyText(items, state.defaults ?? [], state.historySaved as string, current),
            restored,
          )
        } else {
          showHistory(step.pos, items, state.defaults ?? [], state.historySaved as string, restored)
        }
        return
      }

      if (e.key === 'Tab' && (state.complete || state.completions)) {
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
        onSubmit(state.input)
      } else if (isQuitKey(e)) {
        e.preventDefault()
        onCancel()
      }
    }

    return (
      <div className="minibuffer">
        <span className="minibuffer-prompt">
          {state.historySearch
            ? `${state.historySearch.older ? 'Previous' : 'Next'} element matching (regexp): `
            : state.prompt}
        </span>
        <input
          ref={inputRef}
          className="minibuffer-input"
          value={state.input}
          onChange={(e) => {
            setShown([])
            setNote({ key: '', text: '' })
            onInputChange(e.target.value, state)
          }}
          onKeyDown={handleKeyDown}
          spellCheck={false}
        />
        {shown === null && <span className="minibuffer-note">[No match]</span>}
        {shownNote !== '' && <span className="minibuffer-note">{` ${shownNote}`}</span>}
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
