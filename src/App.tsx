/**
 * アプリの画面. バッファとウィンドウを出し, 機能ごとのフック (`src/hooks/`) のコマンドを, キーと M-x と
 * ミニバッファにつなぎます. バッファのコマンド (C-x b, C-x k, C-x C-b) と, 終わること (C-x C-c) も受け持ちます.
 */

import { useEffect, useRef, useState } from 'react'
import { getCurrentWindow } from '@tauri-apps/api/window'
import CodeEditor, { EditorHandle } from './components/CodeEditor'
import ModeLine from './components/ModeLine'
import Minibuffer, { MinibufferInputState, MinibufferIsearchState } from './components/Minibuffer'
import BufferList from './components/BufferList'
import ProjectTree from './components/ProjectTree'
import { type Buffer, headLines } from './lib/buffer'
import { listDirectory } from './lib/fileOps'
import { DIRED_MODE, GREP_MODE, majorMode } from './lib/modes'
import { activeKeyBindings, toEmacsKey } from './lib/emacs'
import type { KeyBinding } from './lib/lisp'
import { windowRects } from './lib/windows'
import { type CommandSet, type Submitters, useMinibuffer } from './hooks/useMinibuffer'
import { type AppContext, START_CURSOR, useBuffers } from './hooks/useBuffers'
import { useDirectories } from './hooks/useDirectories'
import { useFiles } from './hooks/useFiles'
import { useSearch } from './hooks/useSearch'
import { useBookmarks } from './hooks/useBookmarks'
import { useLisp } from './hooks/useLisp'
import './App.css'

/**
 * バッファだけのキーと, Lisp のキーを合わせたもの (同じなら同じ配列にして, エディタが割り当てを作り直さないように).
 * バッファだけのキーの配列ごとに, 合わせた Lisp のキーと, 合わせたものを覚えます.
 */
const mergedKeys = new WeakMap<KeyBinding[], { base: KeyBinding[]; merged: KeyBinding[] }>()

/** 保存していないバッファがあるとき, 終わってよいか聞く文. */
const CONFIRM_EXIT_PROMPT = 'Modified buffers exist; exit anyway? (yes or no) '

/**
 * アプリの画面 (ウィンドウ, モードライン, ミニバッファ, バッファの一覧).
 *
 * @returns 画面.
 */
function App() {
  const minibuffer = useMinibuffer()
  const { pendingAction, setPendingAction, minibufferState, setMinibufferState, showMessage } =
    minibuffer
  const bufs = useBuffers(showMessage)
  const {
    buffers,
    layout,
    windowBuffers,
    cursors,
    setCursors,
    activeWindow,
    handles,
    editor,
    currentBufferId,
    currentBuffer,
    selectWindow,
    setCurrentBufferId,
    createNewBuffer,
    killCurrentBuffer,
    handleEditorChange,
  } = bufs
  const ctx: AppContext = { ...minibuffer, ...bufs }
  const directories = useDirectories(ctx)
  const files = useFiles(ctx, directories.listDirectoryInto)
  const search = useSearch(ctx)
  const bookmarks = useBookmarks(ctx)
  const lisp = useLisp(ctx)
  const { lispKeys } = lisp
  // バッファの一覧 (C-x C-b) を出しているか
  const [showBufferList, setShowBufferList] = useState(false)
  // 最後に C-x r t で入れた文字 (次の既定)
  const rectangleString = useRef<string | null>(null)

  // 窓を閉じるとき (× のボタンも) に見る, 今のバッファと, 終わってよいと答えたか
  const buffersRef = useRef(buffers)
  const exitConfirmed = useRef(false)
  useEffect(() => {
    buffersRef.current = buffers
  }, [buffers])

  // 保存していないバッファがあれば, 窓を閉じずに, 終わってよいか聞く
  useEffect(() => {
    const unlisten = getCurrentWindow().onCloseRequested((event) => {
      if (exitConfirmed.current || !buffersRef.current.some((b) => b.modified)) return
      event.preventDefault()
      setPendingAction('confirm-exit')
      setMinibufferState({ mode: 'input', prompt: CONFIRM_EXIT_PROMPT, input: '' })
    })
    return () => {
      void unlisten.then((f) => f())
    }
  }, [setPendingAction, setMinibufferState])

  /**
   * 出すバッファの名前を, ミニバッファで聞きます (C-x b).
   */
  const handleSwitchBuffer = () => {
    const bufferNames = buffers.map((b) => b.name).join(', ')
    setPendingAction('switch-buffer')
    setMinibufferState({
      mode: 'input',
      prompt: `Switch to buffer (${bufferNames}): `,
      input: '',
      completions: buffers.map((b) => b.name),
    })
  }

  /**
   * 選んでいるウィンドウのバッファを消します (C-x k). 保存していなければ, 消してよいか聞きます.
   */
  const handleKillBuffer = () => {
    if (currentBuffer.modified) {
      setPendingAction('kill-buffer')
      setMinibufferState({
        mode: 'input',
        prompt: `Buffer ${currentBuffer.name} modified; kill anyway? (yes or no) `,
        input: '',
      })
    } else {
      killCurrentBuffer()
    }
  }

  /**
   * キーの割り当ての一覧を, *Help* バッファに出します (前の *Help* は, 作り直す).
   */
  const handleDescribeBindings = () => {
    const lines = activeKeyBindings(lispKeys).map(
      ([key, command]) => `${toEmacsKey(key).padEnd(24)}${command}`,
    )
    const old = buffers.find((b) => b.name === '*Help*')
    if (old) {
      for (const handle of handles.current.values()) handle.forget(old.id)
      bufs.setBuffers((prev) => prev.filter((b) => b.id !== old.id))
    }
    const header = `${'key'.padEnd(24)}binding\n${'---'.padEnd(24)}-------\n`
    createNewBuffer('*Help*', null, header + lines.join('\n') + '\n')
  }

  /**
   * アプリを終わります (C-x C-c). 保存していないバッファがあれば, 終わってよいか聞きます.
   */
  const handleSaveBuffersKillTerminal = async () => {
    // 保存していないバッファがあれば, 終わってよいか聞く (Emacs と同じ)
    if (buffers.some((b) => b.modified)) {
      setPendingAction('confirm-exit')
      setMinibufferState({ mode: 'input', prompt: CONFIRM_EXIT_PROMPT, input: '' })
      return
    }
    await getCurrentWindow().close()
  }

  /**
   * 矩形を置き換える文字を, ミニバッファで聞きます (C-x r t).
   */
  const handleStringRectangle = () => {
    if (!editor.current?.hasMark()) {
      showMessage('The mark is not set now, so there is no region')
      return
    }
    const last = rectangleString.current
    setPendingAction('string-rectangle')
    setMinibufferState({
      mode: 'input',
      prompt: last === null ? 'String rectangle: ' : `String rectangle (default ${last}): `,
      input: '',
    })
  }

  /**
   * 移動する行の番号を, ミニバッファで聞きます (M-g g).
   */
  const handleGotoLine = () => {
    setPendingAction('goto-line')
    setMinibufferState({ mode: 'input', prompt: 'Goto line: ', input: '' })
  }

  /**
   * 実行するコマンドの名前を, ミニバッファで聞きます (M-x).
   */
  const handleExecuteCommand = () => {
    setPendingAction('execute-command')
    setMinibufferState({
      mode: 'input',
      prompt: 'M-x ',
      input: '',
      completions: [...(editor.current as EditorHandle).commandNames(), ...Object.keys(actions)],
    })
  }

  // 画面のコマンドのうち, ここで受け持つもの
  const appCommands: CommandSet = {
    actions: {
      'switch-to-buffer': handleSwitchBuffer,
      'kill-buffer': handleKillBuffer,
      'list-buffers': () => setShowBufferList(true),
      'describe-bindings': handleDescribeBindings,
      'save-buffers-kill-terminal': () => void handleSaveBuffersKillTerminal(),
      'execute-extended-command': handleExecuteCommand,
      'goto-line': handleGotoLine,
      'string-rectangle': handleStringRectangle,
      'abort-minibuffers': () => abortMinibuffer('Not in a minibuffer'),
      'abort-recursive-edit': () => abortMinibuffer('No recursive edit is in progress'),
    },
    submit: {
      'switch-buffer': (value) => {
        const targetBuffer = buffers.find((b) => b.name === value.trim())
        if (targetBuffer) {
          setCurrentBufferId(targetBuffer.id)
          showMessage('')
        } else {
          showMessage(`No buffer named "${value}"`)
        }
      },
      'kill-buffer': (value) => {
        if (value.trim() === 'yes') killCurrentBuffer()
        showMessage('')
      },
      'execute-command': (value) => {
        showMessage('')
        if (!editor.current?.run(value.trim())) showMessage(`Undefined command: ${value}`)
      },
      'goto-line': (value) => {
        editor.current?.gotoLine(Number(value))
        showMessage('')
      },
      'confirm-exit': async (value) => {
        if (value.trim() === 'yes') {
          exitConfirmed.current = true
          await getCurrentWindow().close()
        } else {
          showMessage('')
        }
      },
      'string-rectangle': (value) => {
        // 空の入力は, 次の既定にしない (Emacs の履歴と同じ)
        const string = value === '' ? (rectangleString.current ?? '') : value
        if (string !== '') rectangleString.current = string
        editor.current?.stringRectangle(string)
        showMessage('')
      },
    },
  }

  // 画面のコマンド (キーと M-x から, 名前で呼ぶ). エディタのコマンドは, エディタが持つ.
  // Lisp で作ったコマンドは, 同じ名前のコマンドより先に使う (最後に置く)
  const actions: Record<string, () => void> = {
    ...appCommands.actions,
    ...bufs.windowCommands.actions,
    ...files.commands.actions,
    ...directories.commands.actions,
    ...search.commands.actions,
    ...bookmarks.actions,
    ...lisp.commands.actions,
  }
  // Lisp の editor-command-names (C-h f の候補) に, 画面のコマンドの名前を渡す
  useEffect(() => {
    lisp.setScreenCommandNames(Object.keys(actions))
  })
  const submitters: Submitters = {
    ...appCommands.submit,
    ...bufs.windowCommands.submit,
    ...files.commands.submit,
    ...directories.commands.submit,
    ...search.commands.submit,
    ...bookmarks.submit,
    ...lisp.commands.submit,
  }

  /**
   * ミニバッファの入力が変わったときに, 入力を覚えます. isearch なら, 入力した文字で探し直します.
   *
   * @param value - 入力.
   * @param current - ミニバッファの状態.
   */
  const handleMinibufferInputChange = (
    value: string,
    current: MinibufferInputState | MinibufferIsearchState,
  ) => {
    if (current.mode === 'input') {
      setMinibufferState({ ...current, input: value })
    } else {
      search.searchIsearch(current, value)
    }
  }

  /**
   * ミニバッファの入力を確定したときに, 入力を待っていたコマンドを行います.
   * 待っているコマンドがなければ, isearch を終わります (コマンドのない入力欄は, isearch だけ).
   *
   * @param value - 入力.
   */
  const handleMinibufferSubmit = async (value: string) => {
    const action = pendingAction
    setPendingAction(null)
    minibuffer.addHistory(action, value)
    const submit = action && submitters[action]
    if (submit) {
      await submit(value)
    } else {
      search.finishIsearch(minibufferState as MinibufferIsearchState)
    }
  }

  /**
   * 1 つのキーで答える問い (C-x s, ファイルを置き換えてよいか, query-replace) に答えます.
   *
   * @param key - 押したキー.
   */
  const handleQueryKey = (key: string) => {
    if (!files.answerQuery(key)) search.answerQuery(key)
  }

  /**
   * ミニバッファの入力をやめます (C-g). isearch なら, カーソルを探す前の位置に戻します.
   */
  const handleMinibufferCancel = () => {
    setPendingAction(null)
    files.cancel()
    search.cancel(minibufferState)
    lisp.cancel()
    showMessage('Quit')
  }

  /**
   * ミニバッファを使っていれば, やめます (Emacs の `abort-minibuffers` と `abort-recursive-edit`.
   * エディタに移っていても, やめられる). 使っていなければ, 誤りを出します.
   *
   * @param error - ミニバッファを使っていないときに出す誤り.
   */
  const abortMinibuffer = (error: string) => {
    if (minibuffer.isMinibufferActive()) handleMinibufferCancel()
    else showMessage(error)
  }

  /**
   * バッファで使うキーの割り当て (バッファだけのキーを, Lisp のキーより先に置く) を返します.
   *
   * @param buffer - バッファ.
   * @returns 割り当て. 前と同じなら, 前と同じ配列.
   */
  const keysFor = (buffer: Buffer): KeyBinding[] => {
    if (!buffer.keys) return lispKeys
    const cached = mergedKeys.get(buffer.keys)
    if (cached?.base === lispKeys) return cached.merged
    const merged = [...buffer.keys, ...lispKeys]
    mergedKeys.set(buffer.keys, { base: lispKeys, merged })
    return merged
  }

  return (
    <div className="app">
      <div className="main">
        {directories.treeRoot !== null && (
          <ProjectTree
            root={directories.treeRoot}
            currentFile={currentBuffer.filePath}
            listDirectory={listDirectory}
            onOpenFile={(path) => void directories.openFromTree(path)}
          />
        )}
        <div className="windows">
          {windowRects(layout).map((rect) => {
            const bufferId = windowBuffers[rect.id]
            const buffer = buffers.find((b) => b.id === bufferId) as Buffer
            const active = rect.id === activeWindow
            const mode =
              buffer.directory !== undefined
                ? DIRED_MODE
                : buffer.grep
                  ? GREP_MODE
                  : majorMode(buffer.name, buffer.filePath, headLines(buffer.doc))
            return (
              <div
                key={rect.id}
                className={`window${rect.left + rect.width < 100 ? ' window-left-of' : ''}`}
                style={{
                  left: `${rect.left}%`,
                  top: `${rect.top}%`,
                  width: `${rect.width}%`,
                  height: `${rect.height}%`,
                }}
                onFocus={() => selectWindow(rect.id)}
              >
                <CodeEditor
                  bufferId={bufferId}
                  initialContent={buffer.doc}
                  highlights={buffer.highlights}
                  onChange={(change) => handleEditorChange(rect.id, change)}
                  onCursorChange={(info) => setCursors((prev) => ({ ...prev, [rect.id]: info }))}
                  actions={actions}
                  keyBindings={keysFor(buffer)}
                  onMessage={showMessage}
                  disabled={minibufferState.mode !== 'message' || showBufferList}
                  active={active}
                  language={mode.language}
                  indentTabs={mode.indentTabs}
                  tabWidth={mode.tabWidth}
                  handle={(handle) => {
                    if (handle) handles.current.set(rect.id, handle)
                    else handles.current.delete(rect.id)
                  }}
                />
                <ModeLine
                  bufferName={buffer.name}
                  modified={buffer.modified}
                  cursor={cursors[rect.id] ?? START_CURSOR}
                  mode={mode.name}
                  bufferCount={buffers.length}
                  currentBufferIndex={buffers.findIndex((b) => b.id === bufferId) + 1}
                  active={active}
                />
              </div>
            )
          })}
        </div>
      </div>
      <Minibuffer
        state={
          minibufferState.mode === 'input'
            ? { ...minibufferState, history: minibuffer.history }
            : minibufferState
        }
        onInputChange={handleMinibufferInputChange}
        onSubmit={handleMinibufferSubmit}
        onCancel={handleMinibufferCancel}
        onIsearchRepeat={search.handleIsearchRepeat}
        onIsearchCommand={search.handleIsearchCommand}
        onQueryKey={handleQueryKey}
      />
      {showBufferList && (
        <BufferList
          buffers={buffers}
          currentBufferId={currentBufferId}
          onSelectBuffer={(bufferId) => {
            setCurrentBufferId(bufferId)
            setShowBufferList(false)
          }}
          onClose={() => setShowBufferList(false)}
        />
      )}
    </div>
  )
}

export default App
