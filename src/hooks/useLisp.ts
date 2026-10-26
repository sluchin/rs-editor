/**
 * Lisp の評価 (C-x C-e, M-:) と, Lisp で作ったコマンドとキー. 評価器からのエディタの操作の頼みを受け,
 * 初期設定ファイルを読みます.
 */

import { useEffect, useRef, useState } from 'react'
import type { UnlistenFn } from '@tauri-apps/api/event'
import { Text } from '@codemirror/state'
import type { EditorHandle } from '../components/CodeEditor'
import { backwardSexp, forwardSexp } from '../lib/bracketMatch'
import { type Buffer, generateBufferId, visitFile } from '../lib/buffer'
import {
  type EvalResponse,
  type KeyBinding,
  type LispHost,
  evalLisp,
  listenEditorRequests,
  loadInitFile,
  runLispCommand,
} from '../lib/lisp'
import { setBackgroundColor, setFont, setForegroundColor } from '../lib/theme'
import type { AppContext } from './useBuffers'
import type { CommandSet } from './useMinibuffer'

/**
 * Lisp の評価と, Lisp で作ったコマンドを作ります.
 *
 * @param ctx - ミニバッファと, バッファとウィンドウ.
 * @returns コマンドと, Lisp で割り当てたキーと, やめるときの関数.
 */
export function useLisp(ctx: AppContext) {
  const {
    buffers,
    setBuffers,
    currentBuffer,
    currentBufferId,
    cursor,
    editor,
    handles,
    windowBuffers,
    activeWindow,
    setCurrentBufferId,
    updateCurrentBuffer,
    killBuffer,
    displayInOtherWindow,
    setPendingAction,
    setMinibufferState,
    showMessage,
  } = ctx
  // Lisp で作ったコマンドと, 割り当てたキー
  const [lispCommands, setLispCommands] = useState<string[]>([])
  const [lispKeys, setLispKeys] = useState<KeyBinding[]>([])
  // Lisp から動かすエディタ (画面の今の状態を使うので, 描くたびに作り直す)
  const lispHost = useRef<LispHost>(undefined)
  // Lisp が, バッファを出してから続けるために, 描き直すのを待っている関数
  const bufferWaiters = useRef<(() => void)[]>([])
  // 最後に描いたときのバッファ (描き直すのを待った後で, バッファの名前を引くため)
  const latestBuffers = useRef(buffers)
  // Lisp が待っている, ミニバッファの入力 (read-string)
  const lispInput = useRef<{ resolve: (value: string) => void; reject: (err: Error) => void }>(null)

  /**
   * 評価の結果を出して, Lisp で作ったコマンドと, 割り当てたキーを覚えます.
   *
   * @param response - 評価の結果.
   * @param quiet - 成功したときは, 結果を出さない (コマンドと, 初期設定ファイル).
   */
  const showEvalResult = (response: EvalResponse, quiet = false) => {
    setLispCommands(response.commands)
    setLispKeys(response.keys)
    if (response.error === 'Quit') {
      // read-string を C-g でやめた
      showMessage('Quit')
    } else if (response.error) {
      showMessage(`Error: ${response.error}`)
    } else if (!quiet) {
      showMessage(response.result)
    }
  }

  /**
   * コードを評価して, 結果を出します.
   *
   * @param code - コード.
   */
  const evaluate = async (code: string) => {
    try {
      showEvalResult(await evalLisp(code))
    } catch (err) {
      showMessage(`Eval error: ${err}`)
    }
  }

  /**
   * Lisp で作ったコマンドを実行します.
   *
   * @param name - コマンドの名前.
   */
  const handleRunLispCommand = async (name: string) => {
    try {
      showEvalResult(await runLispCommand(name), true)
    } catch (err) {
      showMessage(`Eval error: ${err}`)
    }
  }

  // 評価器からの, エディタの操作の頼みを受ける. 受け始めてから, 初期設定ファイルを読む
  useEffect(() => {
    let stopped = false
    let unlisten: UnlistenFn | undefined
    void listenEditorRequests(() => lispHost.current as LispHost).then((stop) => {
      // 受け始める前に, 画面を閉じた (開発版の StrictMode では, 1 度閉じて, 開き直す)
      if (stopped) {
        stop()
        return
      }
      unlisten = stop
      loadInitFile()
        .then((response) => showEvalResult(response, true))
        .catch((err) => showMessage(`Init file error: ${err}`))
    })
    return () => {
      stopped = true
      unlisten?.()
    }
    // 始めの 1 回だけ
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [])

  // Lisp から動かすエディタを, 今の状態で作り直す
  useEffect(() => {
    const handle = () => editor.current as EditorHandle
    const root = document.documentElement
    /**
     * バッファを出して, 出し終わるのを待ちます (出したバッファで, Lisp が続きの操作をするため).
     *
     * @param id - バッファ.
     * @returns 出し終わったら, 終わる Promise.
     */
    const showBuffer = (id: string) => {
      if (id === currentBufferId) return Promise.resolve()
      setCurrentBufferId(id)
      return waitRender()
    }
    /**
     * 描き直すのを待ちます (状態を変えた後に呼ぶ. 変えていなければ, 描き直さないので, 待ち続ける).
     *
     * @returns 描き直したら, 終わる Promise.
     */
    const waitRender = () => new Promise<void>((resolve) => bufferWaiters.current.push(resolve))
    /**
     * 名前のバッファを返します (省くと, 今のバッファ).
     *
     * @param name - バッファの名前.
     * @returns バッファ.
     * @throws バッファが無いとき.
     */
    const bufferNamed = (name?: string): Buffer => {
      const found = name === undefined ? currentBuffer : buffers.find((b) => b.name === name)
      if (!found) throw new Error(`No such buffer: ${name}`)
      return found
    }
    /**
     * 名前のバッファ (無ければ, 作る) を返します.
     *
     * @param name - バッファの名前.
     * @returns バッファと, 作ったら `true`.
     */
    const getBufferCreate = (name: string): { buffer: Buffer; created: boolean } => {
      const open = buffers.find((b) => b.name === name)
      if (open) return { buffer: open, created: false }
      const buffer: Buffer = {
        id: generateBufferId(),
        name,
        filePath: null,
        doc: Text.empty,
        modified: false,
      }
      setBuffers((prev) => [...prev, buffer])
      return { buffer, created: true }
    }
    /**
     * 今のバッファの下線を, 変えます.
     *
     * @param change - 今の下線から, 新しい下線を作る関数.
     */
    const updateHighlights = (change: (highlights: string[]) => string[]) =>
      setBuffers((prev) =>
        prev.map((b) =>
          b.id === currentBufferId ? { ...b, highlights: change(b.highlights ?? []) } : b,
        ),
      )
    lispHost.current = {
      text: () => handle().text(),
      point: () => handle().point(),
      setPoint: (pos) => handle().setCursor(pos),
      insert: (text) => handle().insert(text),
      deleteRegion: (from, to) => handle().deleteRegion(from, to),
      bufferName: () => currentBuffer.name,
      message: (text) => setMinibufferState({ mode: 'message', text }),
      command: (name) => handle().run(name),
      findFile: async (path) => {
        const open = buffers.find((b) => b.filePath === path)
        if (open) return showBuffer(open.id)
        const { buffer, message } = await visitFile(path)
        setBuffers((prev) => [...prev, buffer])
        showMessage(message)
        return showBuffer(buffer.id)
      },
      switchToBuffer: (name) => showBuffer(getBufferCreate(name).buffer.id),
      bufferFileName: (name) => bufferNamed(name).filePath,
      bufferModified: (name) => bufferNamed(name).modified,
      bufferList: () => buffers.map((b) => b.name),
      killBuffer: (name) => {
        const buffer = bufferNamed(name)
        // Emacs は聞くが, Lisp からは, 書き換えたファイルのバッファを消さない
        if (buffer.filePath && buffer.modified) {
          throw new Error(`Buffer ${buffer.name} is modified`)
        }
        killBuffer(buffer.id)
        return waitRender()
      },
      killNew: (text) => handle().killNew(text),
      completingRead: (prompt, candidates, initial) =>
        new Promise((resolve, reject) => {
          lispInput.current = { resolve, reject }
          setPendingAction('read-string')
          setMinibufferState({ mode: 'input', prompt, input: initial, completions: candidates })
        }),
      displayBuffer: async (name, line) => {
        const { buffer, created } = getBufferCreate(name)
        // 出していれば, そのウィンドウ. 出していなければ, ほかのウィンドウに出す
        const shown =
          buffer.id === currentBufferId
            ? activeWindow
            : Object.keys(windowBuffers).find((w) => windowBuffers[w] === buffer.id)
        const target = shown ?? displayInOtherWindow(buffer.id, false)
        if (created || !shown) await waitRender()
        if (line !== null) handles.current.get(target)?.gotoLine(line)
      },
      findFileNoselect: async (path) => {
        const open = buffers.find((b) => b.filePath === path)
        if (open) return open.name
        const { buffer } = await visitFile(path)
        setBuffers((prev) => [...prev, buffer])
        await waitRender()
        // 同じ名前のファイルがあれば, 名前を変えているので, 描き直した後のバッファから引く
        return (latestBuffers.current.find((b) => b.id === buffer.id) as Buffer).name
      },
      highlightRegexp: (regexp) => {
        // 誤った正規表現は, ここで失敗にする
        new RegExp(regexp)
        updateHighlights((prev) => [...prev.filter((r) => r !== regexp), regexp])
      },
      unhighlightRegexp: (regexp) =>
        updateHighlights((prev) => (regexp === true ? [] : prev.filter((r) => r !== regexp))),
      setModified: (modified) => updateCurrentBuffer({ modified }),
      localSetKey: (key, command) =>
        setBuffers((prev) =>
          prev.map((b) =>
            b.id === currentBufferId
              ? { ...b, keys: [{ key, command }, ...(b.keys ?? []).filter((k) => k.key !== key)] }
              : b,
          ),
        ),
      readString: (prompt, initial) =>
        new Promise((resolve, reject) => {
          lispInput.current = { resolve, reject }
          setPendingAction('read-string')
          setMinibufferState({ mode: 'input', prompt, input: initial })
        }),
      setBackgroundColor: (color) => setBackgroundColor(root, color),
      setForegroundColor: (color) => setForegroundColor(root, color),
      setFont: (family, size) => setFont(root, family, size),
    }
    latestBuffers.current = buffers
    // 出すのを待っていたバッファは, 描き直したので, 出している (待つのを終える)
    for (const resolve of bufferWaiters.current) resolve()
    bufferWaiters.current = []
  })

  /**
   * カーソルの前の S 式を評価して, 結果を出します (C-x C-e).
   */
  const handleEvalLastSexp = async () => {
    // カーソルの直前の S 式の範囲を求める
    const cursorPos = cursor.offset
    const text = currentBuffer.doc.toString()
    const startPos = backwardSexp(text, cursorPos)
    const endPos = forwardSexp(text, startPos)

    if (startPos === endPos) {
      setMinibufferState({ mode: 'message', text: 'No S-expression found before cursor' })
      return
    }

    await evaluate(text.slice(startPos, endPos))
  }

  /**
   * 評価する式を, ミニバッファで聞きます (M-:).
   */
  const handleEvalExpression = () => {
    setPendingAction('eval-expression')
    setMinibufferState({ mode: 'input', prompt: 'Eval: ', input: '' })
  }

  /**
   * Lisp が待っている入力 (read-string) を, やめます.
   */
  const cancel = () => {
    lispInput.current?.reject(new Error('Quit'))
    lispInput.current = null
  }

  // Lisp で作ったコマンドは, 同じ名前のコマンドより先に使う (コマンドを合わせるときは, 最後に置く)
  const commands: CommandSet = {
    actions: {
      'eval-last-sexp': () => void handleEvalLastSexp(),
      'eval-expression': handleEvalExpression,
      ...Object.fromEntries(
        lispCommands.map((name) => [name, () => void handleRunLispCommand(name)]),
      ),
    },
    submit: {
      'read-string': (value) => {
        showMessage('')
        lispInput.current?.resolve(value)
        lispInput.current = null
      },
      'eval-expression': evaluate,
    },
  }

  return { commands, lispKeys, cancel }
}
