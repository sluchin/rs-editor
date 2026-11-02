/**
 * Lisp の評価 (C-x C-e, M-:) と, Lisp で作ったコマンドとキー. 評価器からのエディタの操作の頼みを受け,
 * 初期設定ファイルを読みます.
 */

import { useEffect, useRef, useState } from 'react'
import type { UnlistenFn } from '@tauri-apps/api/event'
import { Text } from '@codemirror/state'
import type { EditorHandle } from '../components/CodeEditor'
import { backwardSexp, forwardSexp } from '../lib/bracketMatch'
import {
  MODIFIER_KEYS,
  activeKeyBindings,
  eventKeyName,
  lookupKeyBinding,
  toEmacsKey,
} from '../lib/emacs'
import { type Buffer, generateBufferId, visitFile } from '../lib/buffer'
import { checkHighlightRegexp, type Highlight } from '../lib/faces'
import {
  type EvalResponse,
  type KeyBinding,
  type LispHost,
  evalLisp,
  listenEditorRequests,
  loadInitFile,
  runHooks,
  runLispCommand,
} from '../lib/lisp'
import { windowIds } from '../lib/windows'
import { setBackgroundColor, setFont, setForegroundColor } from '../lib/theme'
import type { AppContext } from './useBuffers'
import { type CommandSet, completeFile } from './useMinibuffer'

/** 覚えておく最近のキーの数 (Emacs の `view-lossage` と同じ). */
const RECENT_KEYS_SIZE = 300

/**
 * Lisp の評価と, Lisp で作ったコマンドを作ります.
 *
 * @param ctx - ミニバッファと, バッファとウィンドウ.
 * @param saveBuffer - バッファを, ファイルに書く関数 (書けなければ, 失敗).
 * @returns コマンドと, Lisp で割り当てたキーと, 1 つのキーで答える問いと, やめるときの関数.
 */
export function useLisp(
  ctx: AppContext,
  saveBuffer: (buffer: Buffer, path: string) => Promise<void>,
) {
  const {
    buffers,
    setBuffers,
    currentBuffer,
    currentBufferId,
    cursor,
    editor,
    handles,
    windowBuffers,
    layout,
    activeWindow,
    selectWindow,
    setCurrentBufferId,
    updateCurrentBuffer,
    setCurrentText,
    killBuffer,
    displayInOtherWindow,
    setPendingAction,
    setMinibufferState,
    showMessage,
    defaultDirectory,
  } = ctx
  // Lisp で作ったコマンドと, 割り当てたキー
  const [lispCommands, setLispCommands] = useState<string[]>([])
  const [lispKeys, setLispKeys] = useState<KeyBinding[]>([])
  // 画面のコマンドの名前 (App が, 描くたびに入れる. Lisp の editor-command-names)
  const screenCommandNames = useRef<string[]>([])
  // 手続きのある, エディタが実行するフック (find-file-hook, after-save-hook)
  const [lispHooks, setLispHooks] = useState<string[]>([])
  // ファイルを開いたバッファ (覚えたものと, find-file-hook をまだ実行していないもの) と, 保存した回数
  const visitedBuffers = useRef(new Set<string>())
  const unhookedBuffers = useRef(new Set<string>())
  const saveCounts = useRef(new Map<string, number>())
  // Lisp から動かすエディタ (画面の今の状態を使うので, 描くたびに作り直す)
  const lispHost = useRef<LispHost>(undefined)
  // Lisp が, バッファを出してから続けるために, 描き直すのを待っている関数
  const bufferWaiters = useRef<(() => void)[]>([])
  // 最後に描いたときのバッファ (描き直すのを待った後で, バッファの名前を引くため)
  const latestBuffers = useRef(buffers)
  // Lisp が待っている, ミニバッファの入力 (read-string)
  const lispInput = useRef<{ resolve: (value: string) => void; reject: (err: Error) => void }>(null)
  // Lisp が待っている, 1 つのキーの答え (read-key)
  const lispKey = useRef<{ resolve: (key: string) => void; reject: (err: Error) => void }>(null)

  // 最近押したキー (Emacs の書き方. 古いものから. `recent-keys`, `view-lossage`)
  const recentKeys = useRef<string[]>([])
  useEffect(() => {
    const record = (event: KeyboardEvent) => {
      if (MODIFIER_KEYS.includes(event.key)) return
      recentKeys.current.push(toEmacsKey(eventKeyName(event)))
      if (recentKeys.current.length > RECENT_KEYS_SIZE) recentKeys.current.shift()
    }
    // エディタが受ける前に残す (キーを処理したコマンドが, 伝わりを止めても残る)
    window.addEventListener('keydown', record, true)
    return () => window.removeEventListener('keydown', record, true)
  }, [])

  /**
   * 評価の結果を出して, Lisp で作ったコマンドと, 割り当てたキーを覚えます.
   *
   * @param response - 評価の結果.
   * @param quiet - 成功したときは, 結果を出さない (コマンドと, 初期設定ファイル).
   */
  const showEvalResult = (response: EvalResponse, quiet = false) => {
    setLispCommands(response.commands)
    setLispKeys(response.keys)
    setLispHooks(response.hooks ?? [])
    if (response.error === 'Quit') {
      // read-string と read-key を C-g でやめた
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
        .then((response) => {
          showEvalResult(response, true)
          // 読めなかったら, *Warnings* にも書く (Emacs と同じ)
          if (response.error && response.error !== 'Quit') {
            const text = `An error occurred while loading the init file: ${response.error}`
            return evalLisp(`(display-warning 'initialization ${JSON.stringify(text)} 'error)`)
          }
        })
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
     * 使われていないバッファの名前を返します (Emacs の `generate-new-buffer-name`).
     *
     * @param name - もとの名前.
     * @param ignore - あいていると見なす名前.
     * @returns `name`, `name<2>`, `name<3>`, ... のうち, 最初の使われていない名前.
     */
    const uniqueBufferName = (name: string, ignore: string | null): string => {
      const used = (candidate: string) =>
        candidate !== ignore && buffers.some((b) => b.name === candidate)
      let candidate = name
      for (let n = 2; used(candidate); n++) candidate = `${name}<${n}>`
      return candidate
    }
    /**
     * 今のバッファの下線を, 変えます.
     *
     * @param change - 今の下線から, 新しい下線を作る関数.
     */
    const updateHighlights = (change: (highlights: Highlight[]) => Highlight[]) =>
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
      message: (text) => showMessage(text),
      command: (name) => handle().run(name),
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
        const target = shown ?? displayInOtherWindow(buffer.id)
        if (created || !shown) await waitRender()
        if (line !== null) handles.current.get(target)?.gotoLine(line)
      },
      findFileNoselect: async (path) => {
        const open = buffers.find((b) => b.filePath === path)
        if (open) return open.name
        // 新しいファイルなら, (New file) と出す (Emacs と同じ)
        const { buffer, message } = await visitFile(path)
        setBuffers((prev) => [...prev, buffer])
        showMessage(message)
        await waitRender()
        // 同じ名前のファイルがあれば, 名前を変えているので, 描き直した後のバッファから引く
        return (latestBuffers.current.find((b) => b.id === buffer.id) as Buffer).name
      },
      highlightRegexp: (regexp, face, group) => {
        // 誤った正規表現は, ここで失敗にする
        checkHighlightRegexp(regexp)
        const highlight: Highlight = { regexp }
        if (face) highlight.face = face
        if (group) highlight.group = group
        updateHighlights((prev) => [...prev.filter((h) => h.regexp !== regexp), highlight])
      },
      unhighlightRegexp: (regexp) =>
        updateHighlights((prev) =>
          regexp === true ? [] : prev.filter((h) => h.regexp !== regexp),
        ),
      outlineFlagRegion: (from, to, hide) => handle().foldRegion(from, to, hide),
      invisible: (pos) => handle().invisible(pos),
      setModified: (modified) => {
        if (currentBuffer.modified === modified) return Promise.resolve()
        updateCurrentBuffer({ modified })
        return waitRender()
      },
      localSetKey: (key, command) =>
        setBuffers((prev) =>
          prev.map((b) =>
            b.id === currentBufferId
              ? { ...b, keys: [{ key, command }, ...(b.keys ?? []).filter((k) => k.key !== key)] }
              : b,
          ),
        ),
      whereIs: (command) =>
        activeKeyBindings(lispKeys)
          .filter(([, bound]) => bound === command)
          .map(([key]) => toEmacsKey(key)),
      editorCommandNames: () => [...handle().commandNames(), ...screenCommandNames.current],
      getBufferCreate: async (name) => {
        const { created } = getBufferCreate(name)
        if (created) await waitRender()
        return name
      },
      getBuffer: (name) => buffers.find((b) => b.name === name)?.name ?? null,
      generateNewBufferName: (name, ignore) => uniqueBufferName(name, ignore),
      generateNewBuffer: async (name) => {
        const unique = uniqueBufferName(name, null)
        getBufferCreate(unique)
        await waitRender()
        return unique
      },
      setBuffer: async (name) => {
        const buffer = bufferNamed(name)
        await showBuffer(buffer.id)
        return name
      },
      otherWindow: async (count) => {
        const ids = windowIds(layout)
        if (ids.length < 2) return
        const size = ids.length
        selectWindow(ids[(((ids.indexOf(activeWindow) + count) % size) + size) % size])
        await waitRender()
      },
      keyBinding: (keys) => lookupKeyBinding(lispKeys, keys),
      allKeyBindings: () =>
        activeKeyBindings(lispKeys).map(([key, command]) => [toEmacsKey(key), command]),
      recentKeys: () => [...recentKeys.current],
      readString: (prompt, initial) =>
        new Promise((resolve, reject) => {
          lispInput.current = { resolve, reject }
          setPendingAction('read-string')
          setMinibufferState({ mode: 'input', prompt, input: initial })
        }),
      readFileName: (prompt, dir) =>
        new Promise((resolve, reject) => {
          lispInput.current = { resolve, reject }
          setPendingAction('read-file-name')
          setMinibufferState({
            mode: 'input',
            prompt,
            input: dir || defaultDirectory(currentBuffer),
            complete: completeFile,
          })
        }),
      readKey: (prompt) =>
        new Promise((resolve, reject) => {
          lispKey.current = { resolve, reject }
          setMinibufferState({ mode: 'query', prompt })
        }),
      writeBuffer: async (path, name) => {
        await saveBuffer(bufferNamed(name), path)
        await waitRender()
      },
      setBufferText: (text, modified) => setCurrentText(text, modified),
      snapshot: () => ({
        name: currentBuffer.name,
        text: handle().text(),
        point: handle().point(),
        mark: handle().mark(),
        file: currentBuffer.filePath,
      }),
      applyBuffer: (text, point, mark) => {
        // 本文が違えば置き換え (変わった所だけ. 変更ありにする), カーソルとマークを置く
        if (handle().text() !== text) setCurrentText(text, true)
        handle().setCursor(point)
        handle().setMark(mark)
      },
      setBufferDirectory: (dir) =>
        setBuffers((prev) =>
          prev.map((b) => (b.id === currentBufferId ? { ...b, directory: dir } : b)),
        ),
      setBackgroundColor: (color) => setBackgroundColor(root, color),
      setForegroundColor: (color) => setForegroundColor(root, color),
      setFont: (family, size) => setFont(root, family, size),
    }
    latestBuffers.current = buffers
    // 出すのを待っていたバッファは, 描き直したので, 出している (待つのを終える)
    for (const resolve of bufferWaiters.current) resolve()
    bufferWaiters.current = []
  })

  // ファイルを開いたバッファが, 初めて今のバッファになったら find-file-hook を,
  // 今のバッファを保存したら after-save-hook を, 今のバッファで実行する (フックに手続きがあるときだけ)
  useEffect(() => {
    const hooks: string[] = []
    for (const b of buffers) {
      if (b.filePath && !visitedBuffers.current.has(b.id)) {
        visitedBuffers.current.add(b.id)
        unhookedBuffers.current.add(b.id)
      }
      const saves = b.saveCount ?? 0
      if (b.id === currentBufferId && saves > (saveCounts.current.get(b.id) ?? 0)) {
        hooks.push('after-save-hook')
      }
      saveCounts.current.set(b.id, saves)
    }
    if (unhookedBuffers.current.delete(currentBufferId)) hooks.unshift('find-file-hook')
    for (const hook of hooks.filter((h) => lispHooks.includes(h))) {
      runHooks(hook)
        .then((response) => showEvalResult(response, true))
        .catch((err) => showMessage(`Eval error: ${err}`))
    }
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
   * Lisp が待っている, 1 つのキーの答え (read-key) を返します.
   *
   * @param key - 押したキー.
   * @returns Lisp が待っていれば `true` (ほかの問いなら `false`).
   */
  const answerQuery = (key: string): boolean => {
    const waiting = lispKey.current
    if (!waiting) return false
    lispKey.current = null
    showMessage('')
    waiting.resolve(key)
    return true
  }

  /**
   * Lisp が待っている入力 (read-string, read-key) を, やめます.
   */
  const cancel = () => {
    lispInput.current?.reject(new Error('Quit'))
    lispInput.current = null
    lispKey.current?.reject(new Error('Quit'))
    lispKey.current = null
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
      'read-file-name': (value) => {
        showMessage('')
        lispInput.current?.resolve(value)
        lispInput.current = null
      },
      'eval-expression': evaluate,
    },
  }

  /**
   * 画面のコマンドの名前を覚えます (Lisp の editor-command-names で返す).
   *
   * @param names - 画面のコマンドの名前.
   */
  const setScreenCommandNames = (names: string[]) => {
    screenCommandNames.current = names
  }

  return { commands, lispKeys, answerQuery, cancel, setScreenCommandNames }
}
