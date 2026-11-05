/**
 * ウィンドウ (CodeMirror 6). エディタ (Rust) の描く内容を映すだけで, 本文を自分では書き換えません.
 * キーと, 日本語の入力で確定した文字と, クリックを, エディタに送ります.
 */

import { useEffect, useLayoutEffect, useRef, type Ref, useImperativeHandle } from 'react'
import {
  Annotation,
  Compartment,
  EditorSelection,
  EditorState,
  type Extension,
  type StateEffect,
  type Text,
} from '@codemirror/state'
import { EditorView, drawSelection } from '@codemirror/view'
import { codeFolding, foldEffect, foldedRanges, unfoldEffect } from '@codemirror/language'
import { controlChars } from '../lib/controlChars'
import type { DocEntry } from '../lib/docs'
import { type Highlight, regexpHighlighting, type Token, tokenHighlighting } from '../lib/faces'
import { emacsKey } from '../lib/keys'
import { dialectConfig } from '../lib/lispParser'
import type { MajorMode } from '../lib/modes'
import type { Input } from '../lib/session'
import { showParen } from '../lib/showParen'
import '../styles/CodeEditor.css'

/** エディタの描く内容を映す変更 (書き換えを止めない). */
const remote = Annotation.define<boolean>()

/** 隠した所 (Lisp の `outline-flag-region`. Emacs と同じく `...` を出す). */
const folding = codeFolding({ placeholderText: '...' })

/** 画面 (App) から, ウィンドウを動かす窓口. */
export interface WindowHandle {
  /** カーソルを真ん中の行にします (`recenter`). */
  recenter: () => void
  /** 一番上に見えている行 (0 から数える). */
  topLine: () => number
  /** 見えている行の数. */
  bodyHeight: () => number
  /**
   * 範囲を隠すか, 出します (Emacs の `outline-flag-region`. 位置は UTF-16 の単位).
   *
   * @param from - 始め.
   * @param to - 終わり.
   * @param hide - 隠すなら `true`.
   */
  foldRegion: (from: number, to: number, hide: boolean) => void
  /** 位置の文字を隠していれば `true`. */
  invisible: (pos: number) => boolean
  /** 本文. */
  doc: () => Text
}

/** {@link WindowView} のプロパティ. */
interface WindowViewProps {
  /** ウィンドウの番号. */
  windowId: number
  /** バッファの名前. */
  bufferName: string
  /** バッファの本文. */
  entry: DocEntry
  /** カーソル. */
  point: number
  /** リージョンの端 (無ければ `null`). */
  mark: number | null
  /** 選んでいるウィンドウなら `true` (フォーカスを取る). */
  active: boolean
  /** ミニバッファで聞いているあいだは `true` (フォーカスを取らない). */
  asking: boolean
  /** メジャーモード. */
  mode: MajorMode
  /** 色付けする所. */
  highlights: readonly Highlight[]
  /** シンタックスハイライトの字句 (核が返す). */
  tokens: readonly Token[]
  /** 入力をエディタに送ります. */
  onInput: (input: Input) => void
  /** 画面から, ウィンドウを動かす窓口. */
  handle?: Ref<WindowHandle>
}

/**
 * 本文を, 新しい本文に合わせる変更を作ります (前と後ろの同じ部分は, そのまま).
 *
 * @param old - 今の本文.
 * @param doc - 新しい本文.
 * @returns 変更.
 */
function diff(old: Text, doc: Text) {
  const a = old.toString()
  const b = doc.toString()
  let start = 0
  while (start < a.length && start < b.length && a[start] === b[start]) start++
  let end = 0
  while (
    end < a.length - start &&
    end < b.length - start &&
    a[a.length - 1 - end] === b[b.length - 1 - end]
  ) {
    end++
  }
  return { from: start, to: a.length - end, insert: b.slice(start, b.length - end) }
}

/**
 * ウィンドウを出します.
 *
 * @param props - プロパティ.
 * @returns ウィンドウ.
 */
export default function WindowView({
  windowId,
  bufferName,
  entry,
  point,
  mark,
  active,
  asking,
  mode,
  highlights,
  tokens,
  onInput,
  handle,
}: WindowViewProps) {
  const host = useRef<HTMLDivElement>(null)
  const view = useRef<EditorView>(undefined)
  // バッファごとの状態 (出していないバッファの分. 隠した所とスクロールを残す)
  const states = useRef(new Map<string, EditorState>())
  const shown = useRef(bufferName)
  const language = useRef(new Compartment())
  const underline = useRef(new Compartment())
  const latest = useRef({ onInput, windowId, entry })

  useEffect(() => {
    latest.current = { onInput, windowId, entry }
  })

  /**
   * 言語の拡張 (括弧の強調のための方言, タブの幅) を作ります. 色は, 核が返す字句 (`tokens`) で付けます.
   *
   * @returns 拡張.
   */
  const languageExtension = (): Extension => [
    dialectConfig.of(mode.language),
    EditorState.tabSize.of(mode.tabWidth),
  ]

  /**
   * バッファの状態を作ります.
   *
   * @param doc - 本文.
   * @returns 状態.
   */
  const createState = (doc: Text): EditorState =>
    EditorState.create({
      doc,
      extensions: [
        drawSelection(),
        showParen(),
        controlChars(),
        folding,
        language.current.of(languageExtension()),
        underline.current.of([regexpHighlighting(highlights), tokenHighlighting(tokens)]),
        // 本文はエディタが持つ. 日本語の変換中の文字のほかは, 自分で書き換えない
        EditorState.transactionFilter.of((tr) =>
          !tr.docChanged || tr.annotation(remote) || tr.isUserEvent('input.type.compose') ? tr : [],
        ),
        EditorView.domEventHandlers({
          keydown: (event) => {
            // 日本語などの変換中のキー (変換を確定する Enter も) は, IME に任せる
            if (event.isComposing || event.keyCode === 229) return false
            const key = emacsKey(event)
            if (key === null) return false
            event.preventDefault()
            latest.current.onInput({ kind: 'key', key })
            return true
          },
          compositionend: (event, v) => {
            // 変換中に入れた文字を戻してから, 確定した文字をエディタに送る
            setTimeout(() => {
              const doc = latest.current.entry.doc
              v.dispatch({ changes: diff(v.state.doc, doc), annotations: remote.of(true) })
            })
            if (event.data) latest.current.onInput({ kind: 'text', text: event.data })
            return false
          },
          paste: (event) => {
            event.preventDefault()
            const text = event.clipboardData?.getData('text/plain') ?? ''
            if (text) latest.current.onInput({ kind: 'text', text })
            return true
          },
          cut: (event) => {
            event.preventDefault()
            return true
          },
          drop: (event) => {
            event.preventDefault()
            return true
          },
          mouseup: (_event, v) => {
            const { anchor, head } = v.state.selection.main
            latest.current.onInput({
              kind: 'point',
              window: latest.current.windowId,
              pos: head,
              anchor: anchor === head ? null : anchor,
            })
            return false
          },
        }),
      ],
    })

  useImperativeHandle(handle, () => {
    const current = () => view.current as EditorView
    return {
      recenter: () => {
        const v = current()
        v.dispatch({
          effects: EditorView.scrollIntoView(v.state.selection.main.head, { y: 'center' }),
        })
      },
      topLine: () => {
        const v = current()
        const block = v.lineBlockAtHeight(v.scrollDOM.scrollTop)
        return v.state.doc.lineAt(block.from).number - 1
      },
      bodyHeight: () => {
        const v = current()
        return Math.max(1, Math.floor(v.scrollDOM.clientHeight / v.defaultLineHeight))
      },
      foldRegion: (from, to, hide) => {
        const v = current()
        const effects: StateEffect<unknown>[] = []
        foldedRanges(v.state).between(from, to, (a, b) => {
          if (a < to && b > from) effects.push(unfoldEffect.of({ from: a, to: b }))
        })
        if (hide && to > from) effects.push(foldEffect.of({ from, to }))
        v.dispatch({ effects })
      },
      invisible: (pos) => {
        let hidden = false
        foldedRanges(current().state).between(pos, pos, (a, b) => {
          if (a <= pos && pos < b) hidden = true
        })
        return hidden
      },
      doc: () => current().state.doc,
    }
  })

  // 本文を合わせる前に作る (描いたらすぐに合わせるので, レイアウトの効果で)
  useLayoutEffect(() => {
    const created = new EditorView({
      parent: host.current as HTMLDivElement,
      state: createState(entry.doc),
    })
    view.current = created
    return () => {
      created.destroy()
      view.current = undefined
    }
    // 作るのは 1 回だけ (バッファと本文は, 下の効果で合わせる)
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [])

  // バッファを切り替えたら, そのバッファの状態にする. 本文を, 描く内容に合わせる
  useLayoutEffect(() => {
    const v = view.current as EditorView
    if (shown.current !== bufferName) {
      states.current.set(shown.current, v.state)
      shown.current = bufferName
      const next = states.current.get(bufferName) ?? createState(entry.doc)
      states.current.delete(bufferName)
      v.setState(next)
    }
    const doc = v.state.doc
    let changed = false
    if (doc !== entry.doc) {
      changed = true
      const changes = doc === entry.prev && entry.changes ? entry.changes : diff(doc, entry.doc)
      v.dispatch({ changes, annotations: remote.of(true) })
    }
    const length = v.state.doc.length
    const head = Math.min(point, length)
    const anchor = Math.min(mark ?? point, length)
    const selection = v.state.selection.main
    if (changed || selection.head !== head || selection.anchor !== anchor) {
      v.dispatch({
        selection: EditorSelection.single(anchor, head),
        scrollIntoView: true,
        annotations: remote.of(true),
      })
    }
  })

  // メジャーモードが変わったら, 方言とタブの幅を合わせ直す
  useEffect(() => {
    ;(view.current as EditorView).dispatch({
      effects: language.current.reconfigure(languageExtension()),
    })
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [mode.language, mode.tabWidth, mode.indentTabs, bufferName])

  // 色付けが変わったら, 引き直す
  useEffect(() => {
    ;(view.current as EditorView).dispatch({
      effects: underline.current.reconfigure([
        regexpHighlighting(highlights),
        tokenHighlighting(tokens),
      ]),
    })
  }, [highlights, tokens])

  // 選んでいるウィンドウに, フォーカスを移す (ミニバッファで聞いているあいだは, 移さない)
  useEffect(() => {
    if (active && !asking) (view.current as EditorView).focus()
  }, [active, asking])

  return <div className="code-editor-container" ref={host} />
}
