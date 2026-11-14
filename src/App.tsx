/**
 * アプリの画面. エディタ (Rust) が送る描く内容 (frame) を, ウィンドウとモードラインとミニバッファに映します.
 * キーと文字とクリックはエディタに送り, エディタの問い (`read-string` など) にミニバッファで答えます.
 */

import { useEffect, useRef, useState } from 'react'
import { getCurrentWindow } from '@tauri-apps/api/window'
import WindowView, { type WindowHandle } from './components/WindowView'
import ModeLine from './components/ModeLine'
import Minibuffer, {
  type MinibufferInputState,
  type MinibufferState,
} from './components/Minibuffer'
import ProjectTree from './components/ProjectTree'
import TabBar from './components/TabBar'
import { completeFileName } from './lib/complete'
import { type DocEntry, DocStore } from './lib/docs'
import type { Highlight, Token } from './lib/faces'
import { getHomeDir, listDirectory, projectRoot } from './lib/fileOps'
import { HISTORY_LENGTH, addToHistory } from './lib/history'
import { emacsKey } from './lib/keys'
import { fileDirectory } from './lib/paths'
import {
  type BufferFrame,
  type Frame,
  type HighlightFrame,
  type Input,
  listenFrames,
  listenRequests,
  sendInput,
  utf16Offset,
} from './lib/session'
import { setBackgroundColor, setFont, setForegroundColor } from './lib/theme'
import './App.css'

/** 色付けの face が既定 (`hi-yellow`) なら, 下線にする (howm のリンクなど). */
const DEFAULT_FACE = 'hi-yellow'

/** 始めの描く内容 (エディタから受けるまで). */
const EMPTY_FRAME: Frame = { windows: [], selected: 0, buffers: [], message: '', tabs: [], tab: 0 }

/** ミニバッファで答えを待っている問い. */
interface Question {
  /** 出す状態. */
  state: Exclude<MinibufferState, { mode: 'message' }>
  /** 入力の履歴の名前. */
  history: string
  /** 核が渡した履歴の中身 (Lisp の変数. 名前の履歴の代わりに使い, 加えない). */
  items?: readonly string[]
  /** 答える. */
  resolve: (value: string) => void
  /** やめる. */
  reject: (error: Error) => void
}

/** 色付けの, 前に作ったもの (同じなら, 同じ配列にして引き直さない). */
const highlightCache = new Map<string, Highlight[]>()

/** バッファごとの, 前の字句 (同じ内容なら同じ配列を返すため). */
const previousTokens = new Map<string, readonly Token[]>()

/**
 * 字句が前と同じなら, 前の配列を返します (画面の色付けを, 引き直さないため).
 *
 * @param name - バッファの名前.
 * @param tokens - 描く内容の字句.
 * @returns 字句 (同じ内容なら同じ配列).
 */
function stableTokens(name: string, tokens: readonly Token[]): readonly Token[] {
  const previous = previousTokens.get(name)
  if (
    previous &&
    previous.length === tokens.length &&
    previous.every(
      (t, i) => t.from === tokens[i].from && t.to === tokens[i].to && t.kind === tokens[i].kind,
    )
  ) {
    return previous
  }
  previousTokens.set(name, tokens)
  return tokens
}

/**
 * 描く内容の色付けを, ウィンドウの色付けにします.
 *
 * @param highlights - 描く内容の色付け.
 * @returns 色付け (同じ内容なら同じ配列).
 */
function toHighlights(highlights: readonly HighlightFrame[]): Highlight[] {
  const key = JSON.stringify(highlights)
  let cached = highlightCache.get(key)
  if (!cached) {
    cached = highlights.flatMap(({ regexp, face, group }) => {
      // JavaScript で使えない正規表現と, 空の文字に合う正規表現は, 除く
      try {
        if (new RegExp(regexp).test('')) return []
      } catch {
        return []
      }
      const highlight: Highlight = { regexp }
      if (face !== DEFAULT_FACE) highlight.face = face
      if (group) highlight.group = group
      return [highlight]
    })
    highlightCache.set(key, cached)
  }
  return cached
}

/**
 * パスの `~/` を, ホームディレクトリにします (ディレクトリを読むため).
 *
 * @param dir - ディレクトリ.
 * @param home - ホームディレクトリ.
 * @returns 展開したディレクトリ.
 */
function expandTilde(dir: string, home: string): string {
  return dir.startsWith('~/') ? home.replace(/\/+$/, '') + dir.slice(1) : dir
}

/**
 * アプリの画面 (サイドバー, ウィンドウ, モードライン, ミニバッファ).
 *
 * @returns 画面.
 */
/**
 * 核が渡す引数から, 文字列のリストを取り出します.
 *
 * @param value - 引数 (文字列の配列のはず).
 * @returns 文字列だけの配列 (配列でなければ空).
 */
function stringList(value: unknown): string[] {
  return Array.isArray(value) ? value.filter((v): v is string => typeof v === 'string') : []
}

function App() {
  // 描く内容と, 出しているバッファの本文
  const [{ frame, entries }, setShown] = useState<{
    frame: Frame
    entries: Record<string, DocEntry>
  }>({ frame: EMPTY_FRAME, entries: {} })
  const docs = useRef(new DocStore())
  const [question, setQuestion] = useState<Question | null>(null)
  const [inputState, setInputState] = useState<MinibufferInputState | null>(null)
  const [histories, setHistories] = useState<Record<string, string[]>>({})
  const [treeRoot, setTreeRoot] = useState<string | null>(null)
  // ツリーへフォーカスを移す頼みの数 (増えるたびに移す)
  const [treeFocus, setTreeFocus] = useState(0)
  const views = useRef(new Map<number, WindowHandle>())
  const home = useRef('/')
  const heights = useRef('')
  const exitConfirmed = useRef(false)

  const buffer = (name: string): BufferFrame | undefined =>
    frame.buffers.find((b) => b.name === name)
  const selectedWindow = frame.windows.find((w) => w.id === frame.selected)
  const selectedBuffer = selectedWindow && buffer(selectedWindow.buffer)

  /**
   * バッファの既定のディレクトリ (後ろに `/` を付ける).
   *
   * @param b - バッファ (無ければホームディレクトリ).
   * @returns ディレクトリ.
   */
  const defaultDirectory = (b: BufferFrame | undefined): string => {
    if (b?.directory) return b.directory.replace(/\/*$/, '/')
    if (b?.file) return fileDirectory(b.file)
    return home.current.endsWith('/') ? home.current : `${home.current}/`
  }

  /**
   * ミニバッファで聞きます.
   *
   * @param state - 出す状態.
   * @param history - 入力の履歴の名前.
   * @param items - 核が渡した履歴の中身 (省くと, 名前の履歴).
   * @returns 答え. やめたら `Quit` で失敗する.
   */
  const ask = (
    state: Question['state'],
    history: string,
    items?: readonly string[],
  ): Promise<string> =>
    new Promise((resolve, reject) => {
      setInputState(state.mode === 'input' ? state : null)
      setQuestion({ state, history, items, resolve, reject })
    })

  /**
   * 選んでいるウィンドウを返します.
   *
   * @returns ウィンドウ (無ければ失敗).
   */
  const selectedView = (): WindowHandle => {
    const view = views.current.get(frame.selected)
    if (!view) throw new Error('No window is selected')
    return view
  }

  /**
   * 1 から数えた文字の位置を, 選んでいるウィンドウの本文の UTF-16 の位置にします.
   *
   * @param value - 位置.
   * @returns UTF-16 の位置.
   */
  const viewOffset = (value: unknown): number =>
    utf16Offset(selectedView().doc().toString(), Math.max(Number(value) - 1, 0))

  /**
   * エディタの頼みに答えます.
   *
   * @param op - 操作.
   * @param args - 引数.
   * @returns 答え.
   */
  const perform = (op: string, args: unknown[]): unknown => {
    const root = document.documentElement
    switch (op) {
      case 'read-string': {
        const prompt = String(args[0])
        const history = prompt === 'M-x ' ? 'extended-command' : 'minibuffer'
        const input = String(args[1] ?? '')
        // 履歴の中身 (Lisp の変数) と, 始めの位置 (Emacs の HISTORY の (SYMBOL . POSITION))
        const items = Array.isArray(args[4]) ? stringList(args[4]) : undefined
        const start = Math.min(
          Math.max(Number(args[5] ?? 0), 0),
          (items ?? histories[history] ?? []).length,
        )
        return ask(
          {
            mode: 'input',
            prompt,
            input,
            defaults: stringList(args[3]),
            historyPos: start,
            historySaved: input,
          },
          history,
          items,
        )
      }
      case 'read-file-name':
        return ask(
          {
            mode: 'input',
            prompt: String(args[0]),
            input: String(args[1] ?? '') || defaultDirectory(selectedBuffer),
            defaults: stringList(args[3]),
            complete: (text) =>
              completeFileName(text, (dir) => listDirectory(expandTilde(dir, home.current))),
          },
          'file-name',
        )
      case 'completing-read': {
        const prompt = String(args[0])
        return ask(
          {
            mode: 'input',
            prompt,
            input: String(args[2] ?? ''),
            completions: (args[1] as unknown[]).map(String),
          },
          prompt === 'M-x ' ? 'extended-command' : 'minibuffer',
        )
      }
      case 'read-key':
        return ask({ mode: 'query', prompt: String(args[0] ?? '') }, '')
      case 'window-top-line':
        return views.current.get(Number(args[0]))?.topLine() ?? 0
      case 'recenter':
        selectedView().recenter()
        return null
      case 'outline-flag-region': {
        const [a, b] = [viewOffset(args[0]), viewOffset(args[1])]
        selectedView().foldRegion(Math.min(a, b), Math.max(a, b), args[2] === true)
        return null
      }
      case 'outline-invisible-p': {
        const pos = viewOffset(args[0])
        return pos < selectedView().doc().length && selectedView().invisible(pos)
      }
      case 'set-background-color':
        setBackgroundColor(root, String(args[0]))
        return null
      case 'set-foreground-color':
        setForegroundColor(root, String(args[0]))
        return null
      case 'set-font':
        setFont(
          root,
          typeof args[0] === 'string' ? args[0] : null,
          typeof args[1] === 'number' ? args[1] : null,
        )
        return null
      case 'toggle-project-tree': {
        // 'focus なら, 消さずに, ツリーへフォーカスを移す
        const focus = args[0] === 'focus'
        if (treeRoot !== null) {
          if (focus) setTreeFocus((n) => n + 1)
          else setTreeRoot(null)
          return null
        }
        return projectRoot(defaultDirectory(selectedBuffer)).then((dir) => {
          setTreeRoot(dir.replace(/\/+$/, '') || '/')
          if (focus) setTreeFocus((n) => n + 1)
          return null
        })
      }
      // クリップボード (Emacs の gui-set-selection, gui-selection-value). 使えなければ, 何もしない (nil)
      case 'gui-set-selection':
        return Promise.resolve()
          .then(() => navigator.clipboard.writeText(String(args[0])))
          .then(
            () => null,
            () => null,
          )
      case 'gui-selection-value':
        return Promise.resolve()
          .then(() => navigator.clipboard.readText())
          .catch(() => null)
      case 'kill-emacs':
        exitConfirmed.current = true
        return getCurrentWindow()
          .close()
          .then(() => null)
      default:
        throw new Error(`Unknown editor operation: ${op}`)
    }
  }
  const latestPerform = useRef(perform)
  useEffect(() => {
    latestPerform.current = perform
  })

  // 描く内容と頼みを受け始めてから, エディタに描き直してもらう (エディタは, 初めの入力で動き出す)
  useEffect(() => {
    let stopped = false
    const stops: (() => void)[] = []
    void getHomeDir()
      .then((dir) => {
        home.current = dir
      })
      .catch(() => undefined)
    void Promise.all([
      listenFrames((next) => {
        docs.current.apply(next.buffers)
        const shown = next.windows.map((w) => [w.buffer, docs.current.get(w.buffer)])
        setShown({ frame: next, entries: Object.fromEntries(shown) })
      }),
      listenRequests((op, args) => latestPerform.current(op, args)),
    ]).then((unlistens) => {
      stops.push(...unlistens)
      // 受け始める前に, 画面を閉じた (開発版の StrictMode では, 1 度閉じて開き直す)
      if (stopped) stops.forEach((stop) => stop())
      else void sendInput({ kind: 'redraw' })
    })
    return () => {
      stopped = true
      stops.forEach((stop) => stop())
    }
  }, [])

  // 窓を閉じるときは, エディタに聞く (保存していないバッファがあれば, 終わってよいか聞く)
  useEffect(() => {
    const unlisten = getCurrentWindow().onCloseRequested((event) => {
      if (exitConfirmed.current) return
      event.preventDefault()
      void sendInput({ kind: 'command', name: 'save-buffers-kill-terminal' })
    })
    return () => {
      void unlisten.then((f) => f())
    }
  }, [])

  // ウィンドウのフォーカスが外れていても (モードラインを押したときなど), キーをエディタに送る
  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.target !== document.body) return
      const key = emacsKey(event)
      if (key === null) return
      event.preventDefault()
      void sendInput({ kind: 'key', key })
    }
    window.addEventListener('keydown', onKeyDown)
    return () => window.removeEventListener('keydown', onKeyDown)
  }, [])

  // 描いた後に, ウィンドウの見えている行の数を知らせる (変わったときだけ)
  useEffect(() => {
    const list: [number, number][] = [...views.current].map(([id, view]) => [id, view.bodyHeight()])
    const key = JSON.stringify(list)
    if (list.length > 0 && key !== heights.current) {
      heights.current = key
      void sendInput({ kind: 'resize', heights: list })
    }
  })

  /**
   * 問いを閉じます.
   */
  const closeQuestion = () => {
    setQuestion(null)
    setInputState(null)
  }

  /**
   * 入力を確定します.
   *
   * @param value - 入力.
   */
  const handleSubmit = (value: string) => {
    const { history, items, resolve } = question as Question
    if (!items) {
      setHistories((prev) => ({
        ...prev,
        [history]: addToHistory(prev[history] ?? [], value, HISTORY_LENGTH),
      }))
    }
    closeQuestion()
    resolve(value)
  }

  /**
   * 入力をやめます (C-g).
   */
  const handleCancel = () => {
    closeQuestion()
    ;(question as Question).reject(new Error('Quit'))
  }

  /**
   * 1 つのキーの問いに答えます.
   *
   * @param key - キー (Emacs の書き方).
   */
  const handleQueryKey = (key: string) => {
    closeQuestion()
    ;(question as Question).resolve(key)
  }

  /**
   * 入力をエディタに送ります.
   *
   * @param next - 入力.
   */
  const send = (next: Input) => {
    void sendInput(next)
  }

  const minibufferState: MinibufferState = !question
    ? { mode: 'message', text: frame.message }
    : inputState
      ? { ...inputState, history: question.items ?? histories[question.history] }
      : question.state

  return (
    <div className="app">
      <TabBar
        tabs={frame.tabs}
        current={frame.tab}
        onSelect={(number) => send({ kind: 'select-tab', number })}
        onNew={() => send({ kind: 'command', name: 'tab-new' })}
      />
      <div className="main">
        {treeRoot !== null && (
          <ProjectTree
            root={treeRoot}
            currentFile={selectedBuffer?.file ?? null}
            listDirectory={listDirectory}
            onOpenFile={(path) => send({ kind: 'find-file', path })}
            focusSignal={treeFocus}
          />
        )}
        <div className="windows">
          {frame.windows.map((w) => {
            const b = buffer(w.buffer) as BufferFrame
            const entry = entries[w.buffer]
            const active = w.id === frame.selected
            const mode = b.mode
            const line = entry.doc.lineAt(Math.min(w.point, entry.doc.length))
            return (
              <div
                key={w.id}
                className={`window${w.left + w.width < 100 ? ' window-left-of' : ''}`}
                style={{
                  left: `${w.left}%`,
                  top: `${w.top}%`,
                  width: `${w.width}%`,
                  height: `${w.height}%`,
                }}
              >
                <WindowView
                  windowId={w.id}
                  bufferName={w.buffer}
                  entry={entry}
                  point={w.point}
                  mark={w.mark}
                  active={active}
                  asking={question !== null}
                  mode={mode}
                  highlights={toHighlights(b.highlights)}
                  tokens={stableTokens(b.name, b.tokens)}
                  onInput={send}
                  handle={(handle) => {
                    if (handle) views.current.set(w.id, handle)
                    else views.current.delete(w.id)
                  }}
                />
                <ModeLine
                  bufferName={b.name}
                  modified={b.modified}
                  readOnly={b.read_only}
                  cursor={{ line: line.number, column: w.point - line.from + 1 }}
                  mode={mode.name}
                  bufferCount={frame.buffers.length}
                  currentBufferIndex={frame.buffers.indexOf(b) + 1}
                  active={active}
                />
              </div>
            )
          })}
        </div>
      </div>
      <Minibuffer
        state={minibufferState}
        onInputChange={(value, state) => setInputState({ ...state, input: value })}
        onSubmit={handleSubmit}
        onCancel={handleCancel}
        onQueryKey={handleQueryKey}
      />
    </div>
  )
}

export default App
