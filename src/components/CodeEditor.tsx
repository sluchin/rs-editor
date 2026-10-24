import { useEffect, useImperativeHandle, useRef, type Ref } from 'react'
import { standardKeymap } from '@codemirror/commands'
import {
  Annotation,
  Compartment,
  EditorSelection,
  EditorState,
  Prec,
  Transaction,
} from '@codemirror/state'
import { EditorView, drawSelection, keymap } from '@codemirror/view'
import { type Command, createCommands, emacsKeymap, emacsState, gotoLine } from '../lib/emacs'
import { KillRing } from '../lib/killRing'
import { KeyboardMacro } from '../lib/macro'
import type { MajorMode } from '../lib/modes'
import { lispHighlighting } from '../lib/lispMode'
import { indentConfig } from '../lib/lispIndent'
import { showParen } from '../lib/showParen'
import type { KeyBinding } from '../lib/lisp'
import '../styles/CodeEditor.css'

/** 別のウィンドウでの書き換えを写した変更 (書き換えとして知らせない). */
const syncAnnotation = Annotation.define<boolean>()

/**
 * 本文を, 新しい本文に合わせる変更を作ります (前と後ろの同じ部分は, そのまま).
 *
 * @param state - エディタの状態.
 * @param content - 新しい本文.
 * @returns 変更. 同じなら `null`.
 */
function syncChange(state: EditorState, content: string) {
  const old = state.doc.toString()
  if (old === content) return null
  let start = 0
  while (start < old.length && start < content.length && old[start] === content[start]) start++
  let end = 0
  while (
    end < old.length - start &&
    end < content.length - start &&
    old[old.length - 1 - end] === content[content.length - 1 - end]
  ) {
    end++
  }
  return {
    changes: {
      from: start,
      to: old.length - end,
      insert: content.slice(start, content.length - end),
    },
    annotations: [syncAnnotation.of(true), Transaction.addToHistory.of(false)],
  }
}

export interface CursorInfo {
  line: number
  column: number
  offset: number
}

/** 画面 (App) から, エディタを動かす窓口. */
export interface EditorHandle {
  /**
   * コマンドを名前で実行します (M-x).
   *
   * @returns コマンドがあれば `true`.
   */
  run: (name: string) => boolean
  /** 範囲を選びます (検索で見つかったところ). */
  select: (from: number, to: number) => void
  /** カーソルを置きます. */
  setCursor: (pos: number) => void
  /** 1 から数えた行へ動きます. */
  gotoLine: (line: number) => void
  /** 本文を返します. */
  text: () => string
  /** カーソルの位置を返します. */
  point: () => number
  /** カーソルの位置に入れて, カーソルを入れた文字の後へ動かします. */
  insert: (text: string) => void
  /** 範囲を消します. */
  deleteRegion: (from: number, to: number) => void
  /** エディタのコマンドの名前を返します (M-x の補完). */
  commandNames: () => string[]
  /** 別のウィンドウで書き換えた本文を写します (元に戻す履歴には入れない). */
  sync: (content: string) => void
  /** 範囲を置き換えて, カーソルを置き換えた文字の後へ動かします. */
  replace: (from: number, to: number, text: string) => void
  /** 消したバッファの状態 (元に戻す履歴など) を捨てます. */
  forget: (bufferId: string) => void
}

interface CodeEditorProps {
  /** 表示するバッファ. 変わると, そのバッファの状態 (カーソル, 元に戻す履歴) に切り替えます. */
  bufferId: string
  /** バッファの始めの本文 (始めて表示するときだけ使う). */
  initialContent: string
  /** 本文が変わったときに呼びます. */
  onChange: (value: string) => void
  /** カーソルが動いたときに呼びます. */
  onCursorChange: (info: CursorInfo) => void
  /** 画面のコマンド (ファイル, バッファ, 検索, 評価, Lisp で作ったコマンド). 名前で引きます. */
  actions: Record<string, () => void>
  /** Lisp で割り当てたキー (既定の割り当てより先に使う). */
  keyBindings: readonly KeyBinding[]
  /** ミニバッファに出します. */
  onMessage: (text: string) => void
  /** ミニバッファで入力しているあいだは `true` (書き換えさせない). */
  disabled: boolean
  /** 選んでいるウィンドウなら `true` (フォーカスを取る). 省くと `true`. */
  active?: boolean
  /** シンタックスハイライトと字下げの言語 (省くと, 色を付けず, Fundamental モードの字下げ). */
  language?: MajorMode['language']
  /** 字下げにタブを使うなら `true`. 省くと `true` (Emacs の既定). */
  indentTabs?: boolean
  /** タブの幅. 省くと 8 (Emacs の既定). */
  tabWidth?: number
  /** 画面から, エディタを動かす窓口. */
  handle: Ref<EditorHandle>
}

/**
 * カーソルの位置の情報 (行と桁は 1 から) を返します.
 *
 * @param state - エディタの状態.
 * @returns カーソルの位置の情報.
 */
function cursorInfo(state: EditorState): CursorInfo {
  const offset = state.selection.main.head
  const line = state.doc.lineAt(offset)
  return { line: line.number, column: offset - line.from + 1, offset }
}

/**
 * Emacs のキー操作のエディタ (CodeMirror 6). バッファごとに, エディタの状態を持ちます.
 *
 * @param props - 引数.
 * @returns 画面.
 */
export default function CodeEditor({
  bufferId,
  initialContent,
  onChange,
  onCursorChange,
  actions,
  keyBindings,
  onMessage,
  disabled,
  active = true,
  language = null,
  indentTabs = true,
  tabWidth = 8,
  handle,
}: CodeEditorProps) {
  const host = useRef<HTMLDivElement>(null)
  const view = useRef<EditorView>(undefined)
  // バッファごとの状態 (表示していないバッファの分)
  const states = useRef(new Map<string, EditorState>())
  const shown = useRef(bufferId)
  const readOnly = useRef(new Compartment())
  const keys = useRef(new Compartment())
  const highlight = useRef(new Compartment())
  // キーボードマクロ (全てのバッファで共通)
  const macro = useRef(new KeyboardMacro())
  const latestKeys = useRef(keyBindings)
  const latest = useRef({ onChange, onCursorChange, actions, onMessage })
  const commands = useRef<Record<string, Command>>(undefined)
  const initial = useRef({ bufferId, initialContent, disabled, active })

  useEffect(() => {
    latest.current = { onChange, onCursorChange, actions, onMessage }
  }, [onChange, onCursorChange, actions, onMessage])

  /**
   * コマンドを名前で引きます. 画面のコマンドを先に引きます (Lisp で, エディタのコマンドを作り直せるように).
   *
   * @param name - コマンドの名前.
   * @returns コマンド. なければ `undefined`.
   */
  const lookup = (name: string): Command | undefined => {
    const action = latest.current.actions[name]
    if (action) {
      return () => {
        action()
        return true
      }
    }
    return commands.current?.[name]
  }

  /**
   * キーの割り当て (Lisp の割り当てを含む) の拡張を作ります.
   *
   * @returns 拡張.
   */
  const keyExtension = () =>
    // Emacs のキーを, CodeMirror の既定のキー (Ctrl-a で全て選ぶ, など) より先に使う
    Prec.highest(
      keymap.of(
        emacsKeymap(lookup, {
          extra: latestKeys.current,
          // 前置引数で繰り返すのは, エディタのコマンドだけ (画面のコマンドは 1 回)
          repeatable: (name) => !latest.current.actions[name],
          macro: macro.current,
          message: (text) => latest.current.onMessage(text),
        }),
      ),
    )

  /**
   * メジャーモードの拡張 (シンタックスハイライトと字下げ) を作ります.
   *
   * @returns 拡張.
   */
  const highlightExtension = () => [
    language ? lispHighlighting[language] : [],
    indentConfig.of({ dialect: language, indentTabs, tabWidth }),
    EditorState.tabSize.of(tabWidth),
  ]

  /**
   * バッファの始めの状態を作ります.
   *
   * @param doc - 本文.
   * @param isReadOnly - 書き換えさせないなら `true`.
   * @returns 状態.
   */
  const createState = (doc: string, isReadOnly: boolean): EditorState =>
    EditorState.create({
      doc,
      extensions: [
        emacsState(),
        drawSelection(),
        showParen(),
        readOnly.current.of(EditorState.readOnly.of(isReadOnly)),
        keys.current.of(keyExtension()),
        highlight.current.of(highlightExtension()),
        // キーボードマクロを記録する (キーの割り当てより先に見る)
        Prec.highest(
          EditorView.domEventHandlers({
            keydown: (event) => {
              macro.current.record(event)
              return false
            },
          }),
        ),
        // macOS の Ctrl-h (1 文字消す) は, C-h (ヘルプ) の前置きのキーとぶつかるので, 除く
        keymap.of(standardKeymap.filter((b) => b.mac !== 'Ctrl-h')),
        EditorView.updateListener.of((update) => {
          const synced = update.transactions.some((tr) => tr.annotation(syncAnnotation))
          if (update.docChanged && !synced) latest.current.onChange(update.state.doc.toString())
          if (update.docChanged || update.selectionSet) {
            latest.current.onCursorChange(cursorInfo(update.state))
          }
        }),
      ],
    })

  useImperativeHandle(handle, () => {
    const current = () => view.current as EditorView
    return {
      run: (name) => {
        const command = lookup(name)
        return command ? (command(current()), true) : false
      },
      select: (from, to) => {
        current().dispatch({ selection: EditorSelection.range(from, to), scrollIntoView: true })
      },
      setCursor: (pos) => {
        current().dispatch({ selection: EditorSelection.cursor(pos), scrollIntoView: true })
      },
      gotoLine: (line) => gotoLine(current(), line),
      text: () => current().state.doc.toString(),
      point: () => current().state.selection.main.head,
      insert: (text) => {
        const pos = current().state.selection.main.head
        current().dispatch({
          changes: { from: pos, insert: text },
          selection: EditorSelection.cursor(pos + text.length),
          scrollIntoView: true,
        })
      },
      deleteRegion: (from, to) => {
        current().dispatch({ changes: { from, to }, scrollIntoView: true })
      },
      commandNames: () => Object.keys(commands.current as Record<string, Command>),
      sync: (content) => {
        const change = syncChange(current().state, content)
        if (change) current().dispatch(change)
      },
      replace: (from, to, text) => {
        current().dispatch({
          changes: { from, to, insert: text },
          selection: EditorSelection.cursor(from + text.length),
          scrollIntoView: true,
        })
      },
      forget: (id) => {
        states.current.delete(id)
      },
    }
  })

  useEffect(() => {
    commands.current = createCommands({
      killRing: new KillRing(),
      message: (text) => latest.current.onMessage(text),
      macro: macro.current,
    })
    const created = new EditorView({
      parent: host.current as HTMLDivElement,
      state: createState(initial.current.initialContent, initial.current.disabled),
    })
    view.current = created
    latest.current.onCursorChange(cursorInfo(created.state))
    if (initial.current.active) created.focus()
    return () => {
      created.destroy()
      view.current = undefined
    }
    // 作るのは 1 回だけ (バッファの切り替えと, 書き換えの禁止は, 下の効果で行う)
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [])

  // バッファを切り替えたら, そのバッファの状態にする (表示していたバッファの状態は, 覚えておく)
  useEffect(() => {
    const current = view.current as EditorView
    if (shown.current === bufferId) return
    states.current.set(shown.current, current.state)
    shown.current = bufferId
    const next = states.current.get(bufferId) ?? createState(initialContent, disabled)
    states.current.delete(bufferId)
    current.setState(next)
    // 隠していたあいだに, Lisp でキーを割り当てたかもしれない
    current.dispatch({
      effects: [
        readOnly.current.reconfigure(EditorState.readOnly.of(disabled)),
        keys.current.reconfigure(keyExtension()),
        highlight.current.reconfigure(highlightExtension()),
      ],
    })
    // 隠していたあいだに, 別のウィンドウで書き換えたかもしれない
    const change = syncChange(current.state, initialContent)
    if (change) current.dispatch(change)
    latest.current.onCursorChange(cursorInfo(current.state))
    // initialContent は, 始めて表示するときと, 切り替えたときにだけ使う. disabled は, 下の効果で合わせる
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [bufferId])

  // Lisp でキーを割り当てたら, 割り当てを作り直す
  useEffect(() => {
    if (latestKeys.current === keyBindings) return
    latestKeys.current = keyBindings
    ;(view.current as EditorView).dispatch({
      effects: keys.current.reconfigure(keyExtension()),
    })
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [keyBindings])

  // ファイルの名前を変えて (保存するファイル), メジャーモードが変わったら, 色と字下げを合わせ直す
  useEffect(() => {
    ;(view.current as EditorView).dispatch({
      effects: highlight.current.reconfigure(highlightExtension()),
    })
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [language, indentTabs, tabWidth])

  // ミニバッファで入力しているあいだは, 書き換えさせない. 終わったら, 選んでいるウィンドウのエディタに戻る
  useEffect(() => {
    const current = view.current as EditorView
    current.dispatch({ effects: readOnly.current.reconfigure(EditorState.readOnly.of(disabled)) })
    if (!disabled && active) current.focus()
    // active が変わったときは, 下の効果で合わせる
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [disabled])

  // 選んだウィンドウのエディタに, フォーカスを移す
  useEffect(() => {
    if (active && !disabled) (view.current as EditorView).focus()
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [active])

  return <div className="code-editor-container" ref={host} />
}
