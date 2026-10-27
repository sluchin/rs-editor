import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { render, fireEvent, act } from '@testing-library/react'
import { isolateHistory } from '@codemirror/commands'
import { EditorView } from '@codemirror/view'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import App from '@/App'
import { AUTO_SAVE_TIMEOUT } from '@/hooks/useFiles'
import * as fileOps from '@/lib/fileOps'
import * as bookmarks from '@/lib/bookmarks'
import { autoSaveFileName } from '@/lib/paths'

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }))
vi.mock('@tauri-apps/api/event', () => ({ listen: vi.fn() }))
/** アプリの窓を閉じる関数の偽物 (C-x C-c で呼ぶ). */
const close = vi.fn()
/** 窓を閉じるときの頼みを受ける関数の偽物 (受ける関数を, 最後の引数で見る). */
const onCloseRequested = vi.fn()
vi.mock('@tauri-apps/api/window', () => ({
  getCurrentWindow: () => ({ close, onCloseRequested }),
}))
vi.mock('@/lib/fileOps', () => ({
  readFile: vi.fn(),
  writeFile: vi.fn(),
  pathExists: vi.fn(),
  getHomeDir: vi.fn(),
  listDirectory: vi.fn(),
  backupFile: vi.fn(),
  deleteFile: vi.fn(),
  fileNewerThan: vi.fn(),
  listDirectoryLong: vi.fn(),
  grepDirectory: vi.fn(),
  projectRoot: vi.fn(),
}))

// ブックマークのファイルの読み書きだけを, 偽物にする
vi.mock('@/lib/bookmarks', async (original) => ({
  ...(await original<typeof import('@/lib/bookmarks')>()),
  loadBookmarks: vi.fn(),
  saveBookmarks: vi.fn(),
}))

const mocked = vi.mocked(fileOps)
const mockedBookmarks = vi.mocked(bookmarks)
const mockedInvoke = vi.mocked(invoke)
const mockedListen = vi.mocked(listen)
/** エディタの操作の頼みを受けるのをやめる関数の偽物. */
const unlisten = vi.fn()

/**
 * 起動したときに読む Lisp (lisp/files.scm, lisp/window.scm) のコマンドの偽物. Lisp と同じ順で,
 * エディタの操作を頼みます (問いは省く).
 */
const startupCommands: Record<string, () => Promise<void>> = {
  'find-file': async () => {
    const filename = await editorRequest('read-file-name', ['Find file: ', ''])
    const buffer = await editorRequest('find-file-noselect', [filename])
    await editorRequest('pop-to-buffer-same-window', [buffer])
  },
  'switch-to-buffer': async () => {
    const names = await editorRequest('buffer-list', [])
    const name = await editorRequest('completing-read', ['Switch to buffer: ', names, ''])
    if (name !== '') await editorRequest('pop-to-buffer-same-window', [name])
  },
  'write-file': async () => {
    await editorRequest('files--write-buffer', [
      await editorRequest('read-file-name', ['Write file: ', '']),
    ])
  },
  'revert-buffer': async () => {
    if (await editorRequest('files--revert-dired', [])) return
    const file = (await editorRequest('buffer-file-name', [])) as string | null
    if (file === null) {
      await editorRequest('message', ['Buffer does not seem to be associated with any file'])
      return
    }
    const modified = await editorRequest('buffer-modified-p', [])
    const answer = await editorRequest('read-string', [
      modified
        ? `Discard edits and reread from ${file}? (yes or no) `
        : `Revert buffer from file ${file}? (yes or no) `,
      '',
    ])
    if ((answer as string).trim() === 'yes') {
      try {
        await editorRequest('files--set-buffer-text', [await mocked.readFile(file), false])
        await editorRequest('message', [''])
      } catch (err) {
        await editorRequest('message', [`Error: ${err}`])
      }
    } else {
      await editorRequest('message', [''])
    }
  },
  'recover-this-file': async () => {
    const file = (await editorRequest('buffer-file-name', [])) as string | null
    if (file === null) {
      await editorRequest('message', ['This buffer is not visiting a file'])
      return
    }
    const auto = autoSaveFileName(file)
    try {
      if ((await mocked.pathExists(file)) && !(await mocked.pathExists(auto))) {
        await editorRequest('message', [`Auto save file ${auto} does not exist`])
        return
      }
      if (!(await mocked.fileNewerThan(auto, file))) {
        await editorRequest('message', [`Auto-save file ${auto} not current`])
        return
      }
    } catch (err) {
      await editorRequest('message', [`Error: ${err}`])
      return
    }
    const answer = await editorRequest('read-string', [
      `Recover auto save file ${auto}? (yes or no) `,
      '',
    ])
    if ((answer as string).trim() === 'yes') {
      await editorRequest('files--set-buffer-text', [await mocked.readFile(auto), true])
    }
    await editorRequest('message', [''])
  },
  'save-buffer': async () => {
    const file = await editorRequest('buffer-file-name', [])
    if (file === null) {
      await editorRequest('files--write-buffer', [
        await editorRequest('read-file-name', ['Save file: ', '']),
      ])
    } else if (await editorRequest('buffer-modified-p', [])) {
      await editorRequest('files--write-buffer', [file])
    } else {
      await editorRequest('message', ['(No changes need to be saved)'])
    }
  },
}

/**
 * 評価の結果を作ります.
 *
 * @param result - 結果.
 * @param error - 失敗の内容.
 * @param extra - Lisp で作ったコマンドと, 割り当てたキー.
 * @returns 評価の結果.
 */
function response(
  result: string,
  error: string | null = null,
  extra: {
    commands?: string[]
    keys?: { key: string; command: string }[]
    hooks?: string[]
  } = {},
) {
  return {
    result,
    error,
    // 起動したときに読む Lisp (lisp/files.scm, lisp/window.scm) のコマンド
    commands: [...new Set([...(extra.commands ?? []), ...Object.keys(startupCommands)])],
    keys: extra.keys ?? [],
    ...(extra.hooks ? { hooks: extra.hooks } : {}),
  }
}

/** 評価の結果の既定 (Lisp のバックエンドの偽物が返す). */
let defaultResponse = response('')
/** 返事を待っているエディタの操作の頼み (番号 -> 返事を受ける関数). */
const pendingReplies = new Map<number, (reply: { value: unknown; error: string | null }) => void>()
/** エディタの操作の頼みの番号. */
let requestId = 1000

/**
 * Lisp のバックエンドの偽物から, エディタの操作を頼み, 返事を待ちます.
 *
 * @param op - 操作.
 * @param args - 引数.
 * @returns 返事の値 (失敗したら, その内容で失敗する).
 */
async function editorRequest(op: string, args: unknown[]) {
  const id = ++requestId
  const listened = mockedListen.mock.calls.filter(([name]) => name === 'editor-request')
  const handler = listened[listened.length - 1][1]
  const reply = new Promise<{ value: unknown; error: string | null }>((resolve) =>
    pendingReplies.set(id, resolve),
  )
  handler({ event: 'editor-request', id: 0, payload: { id, op, args } })
  const { value, error } = await reply
  if (error !== null) throw error
  return value
}

/**
 * Lisp のバックエンド (invoke) の偽物. 起動したときに読む Lisp のコマンドは, 偽物で動かします.
 *
 * @param cmd - バックエンドのコマンド.
 * @param args - 引数.
 * @returns 結果.
 */
async function lispBackend(cmd: string, args?: unknown) {
  if (cmd === 'editor_reply') {
    const reply = args as { id: number; value: unknown; error: string | null }
    pendingReplies.get(reply.id)?.(reply)
    pendingReplies.delete(reply.id)
    return null
  }
  const command = cmd === 'run_lisp_command' && startupCommands[(args as { name: string }).name]
  if (command) {
    try {
      await command()
      return defaultResponse
    } catch (error) {
      return { ...defaultResponse, error: String(error) }
    }
  }
  return defaultResponse
}

/** キーの修飾. */
interface Mods {
  ctrl?: boolean
  alt?: boolean
}

/**
 * エディタ (App) を表示して, 操作の関数を返します.
 *
 * @returns 操作の関数.
 */
async function setup() {
  const view = render(<App />)
  const editor = EditorView.findFromDOM(
    view.container.querySelector('.cm-editor') as HTMLElement,
  ) as EditorView
  // ホームのディレクトリの取得 (useEffect) を待つ
  await act(async () => {})
  /** エディタで, キーを押す. */
  const key = (k: string, mods: Mods = {}) =>
    fireEvent.keyDown(editor.contentDOM, {
      key: k,
      ctrlKey: mods.ctrl ?? false,
      altKey: mods.alt ?? false,
    })
  /** C-x に続けて, キーを押す. */
  const ctrlX = (k: string, ctrl = true) => {
    key('x', { ctrl: true })
    key(k, { ctrl })
  }
  /** ミニバッファの入力欄. */
  const input = () => view.container.querySelector('input') as HTMLInputElement
  /** Lisp のコマンドが, 続けて頼むエディタの操作を待つ. */
  const settle = async () => {
    for (let i = 0; i < 5; i++) await act(async () => {})
  }
  /** ミニバッファに入力して, Enter を押す (Lisp のコマンドが聞くのも待つ). */
  const submit = async (value: string) => {
    await settle()
    fireEvent.change(input(), { target: { value } })
    await act(async () => {
      fireEvent.keyDown(input(), { key: 'Enter' })
    })
    await settle()
  }
  /** エディタの文字を書き換えて, カーソルを置く (元に戻す履歴は, 前の変更とつなげない). */
  const type = (value: string, cursor = value.length) =>
    act(() => {
      editor.dispatch({
        changes: { from: 0, to: editor.state.doc.length, insert: value },
        selection: { anchor: cursor },
        annotations: isolateHistory.of('full'),
      })
    })
  /** エディタの本文. */
  const doc = () => editor.state.doc.toString()
  /** カーソルの位置. */
  const cursor = () => editor.state.selection.main.head
  const text = () => view.container.textContent ?? ''
  return { ...view, editor, key, ctrlX, input, settle, submit, type, doc, cursor, text }
}

describe('App', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    mocked.getHomeDir.mockResolvedValue('/home/u')
    mocked.writeFile.mockResolvedValue(undefined)
    mocked.backupFile.mockResolvedValue(undefined)
    mocked.deleteFile.mockResolvedValue(undefined)
    mocked.fileNewerThan.mockResolvedValue(false)
    mockedListen.mockResolvedValue(unlisten)
    onCloseRequested.mockResolvedValue(unlisten)
    mockedBookmarks.loadBookmarks.mockResolvedValue([])
    mockedBookmarks.saveBookmarks.mockResolvedValue(undefined)
    // 初期設定ファイル (と, エディタの操作の返事) は, 何もしない
    defaultResponse = response('')
    pendingReplies.clear()
    mockedInvoke.mockImplementation(lispBackend as typeof invoke)
  })

  it('最初は *scratch* で, ホームのディレクトリを取れなくても動く', async () => {
    mocked.getHomeDir.mockRejectedValue(new Error('no home'))
    const editor = await setup()
    expect(editor.text()).toContain('*scratch*')
  })

  it('ホームのディレクトリが / で終わっていれば, そのまま使う', async () => {
    mocked.getHomeDir.mockResolvedValue('/home/v/')
    const editor = await setup()
    editor.ctrlX('f')
    expect(editor.input().value).toBe('/home/v/')
  })

  it('C-x C-f で, あるファイルを開き, ないファイルは新しく作る', async () => {
    const editor = await setup()
    mocked.pathExists.mockResolvedValueOnce(true)
    mocked.readFile.mockResolvedValueOnce('(+ 1 2)')

    editor.ctrlX('f')
    expect(editor.input().value).toBe('/home/u/')
    await editor.submit('/tmp/a.scm')
    expect(editor.text()).toContain('a.scm')
    expect(editor.doc()).toBe('(+ 1 2)')

    mocked.pathExists.mockResolvedValueOnce(false)
    editor.ctrlX('f')
    await editor.submit('/tmp/new.scm')
    expect(editor.text()).toContain('(New file)')
    expect(editor.text()).toContain('[3/3]')
  })

  it('C-x C-f と保存先で, Tab でファイルの名前を補完する', async () => {
    const editor = await setup()
    mocked.listDirectory.mockResolvedValue(['init.scm', 'src/'])
    editor.ctrlX('f')
    fireEvent.change(editor.input(), { target: { value: '/home/u/i' } })
    await act(async () => {
      fireEvent.keyDown(editor.input(), { key: 'Tab' })
    })
    expect(mocked.listDirectory).toHaveBeenCalledWith('/home/u/')
    expect(editor.input().value).toBe('/home/u/init.scm')
    // やめたら, Lisp の find-file が Quit で終わるのを待つ
    await act(async () => {
      fireEvent.keyDown(editor.input(), { key: 'Escape' })
    })

    await act(async () => editor.ctrlX('s'))
    await editor.settle()
    fireEvent.change(editor.input(), { target: { value: '/home/u/' } })
    await act(async () => {
      fireEvent.keyDown(editor.input(), { key: 'Tab' })
    })
    expect(editor.text()).toContain('Possible completions:')
  })

  it('モードラインに, バッファのメジャーモードを出す', async () => {
    const editor = await setup()
    expect(editor.text()).toContain('(Lisp Interaction)')
    mocked.pathExists.mockResolvedValue(false)
    editor.ctrlX('f')
    await editor.submit('/tmp/a.scm')
    expect(editor.text()).toContain('(Scheme)')
    editor.ctrlX('f')
    await editor.submit('/tmp/a.txt')
    expect(editor.text()).toContain('(Fundamental)')
    editor.ctrlX('f')
    await editor.submit('/home/u/.emacs.el')
    expect(editor.text()).toContain('(ELisp/d)')
  })

  it('C-x C-f で, / で終わるパスは, パスをそのままバッファの名前にする', async () => {
    const editor = await setup()
    mocked.pathExists.mockResolvedValueOnce(false)

    editor.ctrlX('f')
    await editor.submit('/tmp/dir/')
    expect(editor.text()).toContain('/tmp/dir/')
  })

  it('C-x C-f の失敗は, メッセージにする', async () => {
    const editor = await setup()
    mocked.pathExists.mockRejectedValueOnce('denied')

    editor.ctrlX('f')
    await editor.submit('/root/x')
    expect(editor.text()).toContain('Error: denied')
  })

  it('同じ名前のファイルは, バッファの名前をディレクトリで区別し, 開いているファイルはそのバッファを出す', async () => {
    const editor = await setup()
    mocked.pathExists.mockResolvedValueOnce(true).mockResolvedValueOnce(true)
    mocked.readFile.mockResolvedValueOnce('').mockResolvedValueOnce('')
    editor.ctrlX('f')
    await editor.submit('/p/src/index.ts')
    expect(editor.text()).toContain('-:--- index.ts')
    editor.ctrlX('f')
    await editor.submit('/p/lib/index.ts')
    expect(editor.text()).toContain('-:--- index.ts<lib>')
    expect(editor.text()).toContain('[3/3]')

    // C-x b で, 区別した名前で移る
    editor.ctrlX('b', false)
    await editor.submit('index.ts<src>')
    expect(editor.text()).toContain('-:--- index.ts<src>')
    expect(editor.text()).toContain('[2/3]')

    // 開いているファイルは, 読み直さずに, そのバッファを出す
    mocked.readFile.mockClear()
    editor.ctrlX('f')
    await editor.submit('/p/lib/index.ts')
    expect(mocked.readFile).not.toHaveBeenCalled()
    expect(editor.text()).toContain('[3/3]')

    // 消すと, 残ったバッファの名前を戻す
    editor.ctrlX('k', false)
    editor.ctrlX('b', false)
    await editor.submit('index.ts')
    expect(editor.text()).toContain('-:--- index.ts ')
  })

  it('C-x C-s で, 書き換えたファイルのバッファは上書きし, 失敗はメッセージにする', async () => {
    const editor = await setup()
    mocked.pathExists.mockResolvedValueOnce(true)
    mocked.readFile.mockResolvedValueOnce('a')
    editor.ctrlX('f')
    await editor.submit('/tmp/a.scm')

    // 書き換えていなければ, 書かない
    await act(async () => editor.ctrlX('s'))
    await editor.settle()
    expect(mocked.writeFile).not.toHaveBeenCalled()
    expect(editor.text()).toContain('(No changes need to be saved)')

    await editor.type('ab')
    await act(async () => editor.ctrlX('s'))
    await editor.settle()
    expect(mocked.writeFile).toHaveBeenCalledWith('/tmp/a.scm', 'ab')
    expect(editor.text()).toContain('Wrote /tmp/a.scm')

    await editor.type('abc')
    mocked.writeFile.mockRejectedValueOnce('disk full')
    await act(async () => editor.ctrlX('s'))
    await editor.settle()
    expect(editor.text()).toContain('Error: disk full')
  })

  it('書いているあいだに書き換えた分は, 保存していないので, 変更ありのままにする', async () => {
    const editor = await setup()
    mocked.pathExists.mockResolvedValueOnce(true)
    mocked.readFile.mockResolvedValueOnce('a')
    editor.ctrlX('f')
    await editor.submit('/tmp/a.scm')
    await editor.type('ab')

    let finish = () => {}
    mocked.writeFile.mockImplementationOnce(
      () => new Promise<void>((resolve) => (finish = resolve)),
    )
    await act(async () => editor.ctrlX('s'))
    await editor.settle()
    await editor.type('abc')
    await act(async () => finish())
    expect(mocked.writeFile).toHaveBeenCalledWith('/tmp/a.scm', 'ab')
    expect(editor.text()).toContain('Wrote /tmp/a.scm')
    expect(editor.text()).toContain('-:**- a.scm')

    // 書き換えていなければ, 変更なしにする
    await act(async () => editor.ctrlX('s'))
    await editor.settle()
    expect(editor.text()).toContain('-:--- a.scm')
  })

  it('ファイルのないバッファの C-x C-s は, 保存先を聞く (失敗はメッセージ)', async () => {
    const editor = await setup()

    await act(async () => editor.ctrlX('s'))
    await editor.settle()
    await editor.submit('/tmp/s.scm')
    expect(editor.text()).toContain('Wrote /tmp/s.scm')

    const other = await setup()
    mocked.writeFile.mockRejectedValueOnce('readonly')
    await act(async () => other.ctrlX('s'))
    await other.settle()
    await other.submit('/x')
    expect(other.text()).toContain('Error: readonly')
  })

  it('C-x C-w で, 別のファイルに書き, バッファをそのファイルのバッファにする', async () => {
    const editor = await setup()
    mocked.pathExists.mockResolvedValue(false)

    editor.ctrlX('w')
    await editor.settle()
    expect(editor.text()).toContain('Write file: ')
    expect(editor.input().value).toBe('/home/u/')
    await editor.submit('/tmp/w.scm')
    expect(mocked.writeFile).toHaveBeenCalledWith('/tmp/w.scm', expect.any(String))
    expect(editor.text()).toContain('Wrote /tmp/w.scm')
    expect(editor.container.querySelector('.mode-line')?.textContent).toContain('w.scm')

    // 入力の始めは, バッファのファイルのディレクトリ. バッファのファイルには, 聞かずに書く
    mocked.pathExists.mockResolvedValue(true)
    editor.ctrlX('w')
    await editor.settle()
    expect(editor.input().value).toBe('/tmp/')
    await editor.submit('/tmp/w.scm')
    expect(mocked.writeFile).toHaveBeenCalledTimes(2)
  })

  it('C-x b でバッファを切り替え, ない名前のバッファは作る', async () => {
    const editor = await setup()
    mocked.pathExists.mockResolvedValueOnce(false)
    editor.ctrlX('f')
    await editor.submit('/tmp/b.scm')

    editor.ctrlX('b', false)
    await editor.submit('*scratch*')
    expect(editor.text()).toContain('[1/2]')

    editor.ctrlX('b', false)
    await editor.submit('nothing')
    expect(editor.text()).toContain('-:--- nothing [3/3]')
  })

  it('C-x k で, 変更のないバッファを消し, 最後の 1 つを消すと *scratch* を作り直す', async () => {
    const editor = await setup()
    mocked.pathExists.mockResolvedValueOnce(false)
    editor.ctrlX('f')
    await editor.submit('/tmp/c.scm')
    expect(editor.text()).toContain('[2/2]')

    editor.ctrlX('k', false)
    expect(editor.text()).not.toContain('[2/2]')
    expect(editor.text()).toContain('*scratch*')

    editor.ctrlX('k', false)
    expect(editor.text()).toContain('*scratch*')
  })

  it('C-x k は, 変更したバッファを消してよいか聞き, yes でだけ消す', async () => {
    const editor = await setup()
    await editor.type('changed')

    editor.ctrlX('k', false)
    expect(editor.text()).toContain('Buffer *scratch* modified; kill anyway? (yes or no)')
    await editor.submit('no')
    expect(editor.doc()).toBe('changed')

    editor.ctrlX('k', false)
    await editor.submit('yes')
    // 全て消したので, *scratch* を作り直す
    expect(editor.doc()).toContain(';; This buffer is for text that is not saved')
  })

  it('変更したバッファの C-x k を C-g でやめたあと, 検索を Enter で確定しても, バッファは消さない', async () => {
    const editor = await setup()
    await editor.type('changed')
    editor.ctrlX('k', false)
    fireEvent.keyDown(editor.input(), { key: 'g', ctrlKey: true })

    editor.key('s', { ctrl: true })
    await act(async () => {
      fireEvent.keyDown(editor.input(), { key: 'Enter' })
    })
    expect(editor.doc()).toBe('changed')
    expect(editor.text()).toContain('*scratch*')
  })

  it('C-x C-b でバッファの一覧を出し, 選ぶか閉じる', async () => {
    const editor = await setup()

    editor.ctrlX('b')
    expect(editor.text()).toContain('Buffer List')
    fireEvent.click(editor.container.querySelector('.buffer-row') as Element)
    expect(editor.text()).not.toContain('Buffer List')

    editor.ctrlX('b')
    fireEvent.click(editor.getByText('×'))
    expect(editor.text()).not.toContain('Buffer List')
  })

  it('C-x C-e で, カーソルの前の S 式を評価する (結果, エラー, 失敗, S 式なし)', async () => {
    const editor = await setup()
    await editor.type('(+ 1 2)')

    mockedInvoke.mockResolvedValueOnce(response('3'))
    await act(async () => editor.ctrlX('e'))
    expect(mockedInvoke).toHaveBeenCalledWith('eval_scheme', { code: '(+ 1 2)' })
    expect(editor.text()).toContain('3')

    mockedInvoke.mockResolvedValueOnce(response('', 'bad'))
    await act(async () => editor.ctrlX('e'))
    expect(editor.text()).toContain('Error: bad')

    mockedInvoke.mockRejectedValueOnce('crash')
    await act(async () => editor.ctrlX('e'))
    expect(editor.text()).toContain('Eval error: crash')

    await editor.type('', 0)
    await act(async () => editor.ctrlX('e'))
    expect(editor.text()).toContain('No S-expression found before cursor')
  })

  it('M-: で式を評価する (結果, エラー, 失敗)', async () => {
    const editor = await setup()

    mockedInvoke.mockResolvedValueOnce(response('42'))
    editor.key(':', { alt: true })
    await editor.submit('(* 6 7)')
    expect(editor.text()).toContain('42')

    mockedInvoke.mockResolvedValueOnce(response('', 'oops'))
    editor.key(':', { alt: true })
    await editor.submit('(car 1)')
    expect(editor.text()).toContain('Error: oops')

    mockedInvoke.mockRejectedValueOnce('down')
    editor.key(':', { alt: true })
    await editor.submit('x')
    expect(editor.text()).toContain('Eval error: down')
  })

  it('起動のときに, 頼みを受け始めてから, 初期設定ファイルを読む', async () => {
    mockedInvoke.mockResolvedValueOnce(response('', 'init.scm: bad'))
    const editor = await setup()
    expect(mockedListen).toHaveBeenCalledWith('editor-request', expect.any(Function))
    expect(mockedInvoke).toHaveBeenCalledWith('load_init')
    expect(editor.text()).toContain('Error: init.scm: bad')
    editor.unmount()
    expect(unlisten).toHaveBeenCalled()
  })

  it('ファイルを開いたら find-file-hook を, 保存したら after-save-hook を, そのバッファで実行する', async () => {
    const hooks = { hooks: ['find-file-hook', 'after-save-hook'] }
    defaultResponse = response('', null, hooks)
    const editor = await setup()
    const ran = (hook: string) =>
      mockedInvoke.mock.calls.filter(
        ([cmd, args]) =>
          cmd === 'eval_scheme' && (args as { code: string }).code === `(run-hooks '${hook})`,
      ).length
    mocked.pathExists.mockResolvedValue(false)
    editor.ctrlX('f')
    await editor.submit('/tmp/a.txt')
    expect(ran('find-file-hook')).toBe(1)
    // 開いたバッファは, 1 度だけ
    await act(async () => editor.type('x'))
    await act(async () => editor.ctrlX('s'))
    await editor.settle()
    expect(ran('find-file-hook')).toBe(1)
    expect(ran('after-save-hook')).toBe(1)
    // 開いたが, 今のバッファにしていないバッファは, 今のバッファにしたときに実行する
    editor.ctrlX('f')
    await editor.submit('/tmp/b.txt')
    expect(ran('find-file-hook')).toBe(2)
    editor.ctrlX('b', false)
    await editor.submit('a.txt')
    expect(ran('find-file-hook')).toBe(2)
    // フックが失敗した
    mockedInvoke.mockImplementation(((cmd: string, args?: unknown) =>
      cmd === 'eval_scheme' ? Promise.reject('down') : lispBackend(cmd, args)) as typeof invoke)
    await act(async () => editor.type('y'))
    await act(async () => editor.ctrlX('s'))
    await editor.settle()
    mockedInvoke.mockImplementation(lispBackend as typeof invoke)
    defaultResponse = response('')
    expect(ran('after-save-hook')).toBe(2)
    expect(editor.text()).toContain('Eval error: down')
    // 評価の結果で, フックに手続きが無くなったら, 実行しない
    await act(async () => editor.type('z'))
    await act(async () => editor.ctrlX('s'))
    await editor.settle()
    expect(ran('after-save-hook')).toBe(3)
    await act(async () => editor.type('w'))
    await act(async () => editor.ctrlX('s'))
    await editor.settle()
    expect(ran('after-save-hook')).toBe(3)
  })

  it('初期設定ファイルを読めなくても, 起動する', async () => {
    mockedInvoke.mockRejectedValueOnce('no backend')
    const editor = await setup()
    expect(editor.text()).toContain('Init file error: no backend')
  })

  it('頼みを受け始める前に閉じたら, すぐに受けるのをやめる', async () => {
    let resolve: (stop: () => void) => void = () => {}
    mockedListen.mockReturnValueOnce(new Promise((r) => (resolve = r)))
    const view = render(<App />)
    view.unmount()
    await act(async () => resolve(unlisten))
    expect(unlisten).toHaveBeenCalled()
    expect(mockedInvoke).not.toHaveBeenCalledWith('load_init')
  })

  it('Lisp で作ったコマンドを M-x とキーで実行する (成功, 失敗, バックエンドの失敗)', async () => {
    const defined = { commands: ['hello'], keys: [{ key: 'Ctrl-c h', command: 'hello' }] }
    mockedInvoke.mockResolvedValueOnce(response('', null, defined))
    const editor = await setup()

    mockedInvoke.mockResolvedValueOnce(response('nil', null, defined))
    editor.key('x', { alt: true })
    await editor.submit('hello')
    expect(mockedInvoke).toHaveBeenCalledWith('run_lisp_command', { name: 'hello' })
    // 成功したときは, 結果を出さない
    expect(editor.text()).not.toContain('nil')

    mockedInvoke.mockResolvedValueOnce(response('', 'boom', defined))
    await act(async () => {
      editor.key('c', { ctrl: true })
      editor.key('h')
    })
    expect(editor.text()).toContain('Error: boom')

    mockedInvoke.mockRejectedValueOnce('gone')
    editor.key('x', { alt: true })
    await editor.submit('hello')
    expect(editor.text()).toContain('Eval error: gone')
  })

  it('評価器の頼みで, エディタを操作する', async () => {
    const editor = await setup()
    await editor.type('abc', 0)
    const handler = mockedListen.mock.calls[0][1]
    const send = (op: string, ...args: unknown[]) =>
      act(() => {
        handler({ event: 'editor-request', id: 0, payload: { id: 7, op, args } })
      })
    const lastReply = () => mockedInvoke.mock.calls[mockedInvoke.mock.calls.length - 1]

    await send('goto-char', 2)
    await send('insert', 'X')
    expect(editor.doc()).toBe('aXbc')
    await send('point')
    expect(lastReply()).toEqual(['editor_reply', { id: 7, value: 3, error: null }])
    await send('delete-region', 1, 3)
    expect(editor.doc()).toBe('bc')
    await send('buffer-name')
    expect(lastReply()).toEqual(['editor_reply', { id: 7, value: '*scratch*', error: null }])
    await send('message', 'from lisp')
    expect(editor.text()).toContain('from lisp')
    await send('command', 'end-of-buffer')
    expect(editor.cursor()).toBe(2)
  })

  it('評価器が、今のバッファを核に写し (editor-snapshot)、核での変更を返す (editor-apply)', async () => {
    const editor = await setup()
    await editor.type('abc', 1)
    const handler = mockedListen.mock.calls[0][1]
    let id = 500
    const send = async (op: string, ...args: unknown[]) => {
      const sent = ++id
      await act(async () => {
        handler({ event: 'editor-request', id: 0, payload: { id: sent, op, args } })
      })
      const reply = [...mockedInvoke.mock.calls]
        .reverse()
        .find(([name, r]) => name === 'editor_reply' && (r as { id: number }).id === sent)
      return (reply?.[1] as { value: unknown }).value
    }
    // 名前, 本文, カーソル, マーク, ファイル
    expect(await send('editor-snapshot')).toEqual(['*scratch*', 'abc', 2, false, false])
    // 本文を置き換え, カーソルとマークを置く (変更ありになる)
    await send('editor-apply', 'xabcd', 3, 1)
    expect(editor.doc()).toBe('xabcd')
    expect(editor.cursor()).toBe(2)
    expect(editor.editor.state.selection.main.head).toBe(2)
    expect(await send('editor-snapshot')).toEqual(['*scratch*', 'xabcd', 3, 1, false])
    // 本文が同じなら、書き換えない. マークは外せる
    await send('editor-apply', 'xabcd', 1, false)
    expect(editor.cursor()).toBe(0)
    expect(await send('editor-snapshot')).toEqual(['*scratch*', 'xabcd', 1, false, false])
  })

  it('C-M-s と C-M-r で, 正規表現で探す', async () => {
    const editor = await setup()
    await editor.type('ab12 cd345', 0)
    editor.key('s', { ctrl: true, alt: true })
    expect(editor.text()).toContain('Regexp I-search: ')
    fireEvent.change(editor.input(), { target: { value: '[0-9]+' } })
    expect(editor.editor.state.selection.main.from).toBe(2)
    fireEvent.keyDown(editor.input(), { key: 's', ctrlKey: true })
    expect(editor.editor.state.selection.main.from).toBe(7)
    await act(async () => {
      fireEvent.keyDown(editor.input(), { key: 'Enter' })
    })
    expect(editor.cursor()).toBe(10)

    editor.key('r', { ctrl: true, alt: true })
    expect(editor.text()).toContain('Regexp I-search backward: ')
    fireEvent.change(editor.input(), { target: { value: '[a-z]+' } })
    expect(editor.editor.state.selection.main.from).toBe(5)
  })

  it('M-% で, 1 つずつ聞いて置き換える (y, n, SPC, DEL, q)', async () => {
    const editor = await setup()
    await editor.type('a a a a a', 0)
    const answer = (key: string) =>
      act(() => {
        fireEvent.keyDown(editor.input(), { key })
      })
    editor.key('%', { alt: true })
    expect(editor.text()).toContain('Query replace: ')
    await editor.submit('a')
    expect(editor.text()).toContain('Query replace a with: ')
    await editor.submit('X')
    expect(editor.text()).toContain('Query replacing a with X: (y, n, !, ., q)')
    expect(editor.editor.state.selection.main.from).toBe(0)
    await answer('n')
    await answer('y')
    await answer('Backspace')
    await answer('z')
    await answer(' ')
    expect(editor.doc()).toBe('a X a X a')
    await answer('q')
    expect(editor.text()).toContain('Replaced 2 occurrences')
    expect(editor.cursor()).toBe(8)
  })

  it('M-% の !, ., Enter と, 見つからないとき, 取り消したとき', async () => {
    const editor = await setup()
    const answer = (key: string) =>
      act(() => {
        fireEvent.keyDown(editor.input(), { key })
      })
    const start = async (from: string, to: string) => {
      editor.key('%', { alt: true })
      await editor.submit(from)
      await editor.submit(to)
    }
    await editor.type('a a a', 0)
    await start('a', 'b')
    await answer('!')
    expect(editor.doc()).toBe('b b b')
    expect(editor.text()).toContain('Replaced 3 occurrences')
    expect(editor.cursor()).toBe(5)

    await editor.type('a a', 0)
    await start('a', 'c')
    await answer('.')
    expect(editor.doc()).toBe('c a')
    expect(editor.text()).toContain('Replaced 1 occurrence')

    await editor.type('a a', 0)
    await start('a', 'c')
    await answer('Enter')
    expect(editor.doc()).toBe('a a')
    expect(editor.text()).toContain('Replaced 0 occurrences')

    await start('zzz', 'c')
    expect(editor.text()).toContain('Replaced 0 occurrences')

    await start('a', 'c')
    await answer('Escape')
    expect(editor.text()).toContain('Quit')
    expect(editor.doc()).toBe('a a')

    editor.key('%', { alt: true })
    await editor.submit('')
    expect(editor.text()).not.toContain('Query replace')
  })

  it('C-M-% と M-x replace-string, replace-regexp で置き換える', async () => {
    const editor = await setup()
    const answer = (key: string) =>
      act(() => {
        fireEvent.keyDown(editor.input(), { key })
      })
    await editor.type('ab cd', 0)
    editor.key('%', { ctrl: true, alt: true })
    expect(editor.text()).toContain('Query replace regexp: ')
    await editor.submit('(')
    expect(editor.text()).toContain('Invalid regexp: (')

    editor.key('%', { ctrl: true, alt: true })
    await editor.submit('([a-z])([a-z])')
    await editor.submit('\\2\\1')
    await answer('!')
    expect(editor.doc()).toBe('ba dc')

    editor.key('x', { alt: true })
    await editor.submit('replace-string')
    expect(editor.text()).toContain('Replace string: ')
    await editor.submit('ba')
    await editor.submit('X')
    expect(editor.doc()).toBe('ba dc')
    expect(editor.text()).toContain('Replaced 0 occurrences')

    await editor.type('ba ba', 0)
    editor.key('x', { alt: true })
    await editor.submit('replace-string')
    await editor.submit('ba')
    await editor.submit('X')
    expect(editor.doc()).toBe('X X')
    expect(editor.cursor()).toBe(3)

    editor.key('x', { alt: true })
    await editor.submit('replace-regexp')
    expect(editor.text()).toContain('Replace regexp: ')
    await editor.submit('X')
    expect(editor.text()).toContain('Replace regexp X with: ')
    fireEvent.keyDown(editor.input(), { key: 'Escape' })
    expect(editor.text()).toContain('Quit')
  })

  it('C-h b で, キーの割り当ての一覧を *Help* バッファに出す (Lisp の割り当てを先に)', async () => {
    mockedInvoke.mockResolvedValueOnce(
      response('', null, {
        commands: ['hello'],
        keys: [{ key: 'Ctrl-x Ctrl-f', command: 'hello' }],
      }),
    )
    const editor = await setup()
    await act(async () => {
      editor.key('h', { ctrl: true })
      editor.key('b')
    })
    expect(editor.text()).toContain('*Help*')
    expect(editor.doc()).toMatch(/^C-x C-f +hello$/m)
    expect(editor.doc()).not.toMatch(/^C-x C-f +find-file$/m)
    expect(editor.doc()).toMatch(/^C-h k +describe-key$/m)
    // もう一度出すと, 作り直す (変更なしなので, すぐに閉じられる)
    await act(async () => {
      editor.key('h', { ctrl: true })
      editor.key('b')
    })
    await act(async () => editor.ctrlX('k', false))
    expect(editor.text()).not.toContain('*Help*')
  })

  it('M-x と C-x b で, Tab で名前を補完する', async () => {
    const editor = await setup()
    editor.key('x', { alt: true })
    fireEvent.change(editor.input(), { target: { value: 'find-fi' } })
    fireEvent.keyDown(editor.input(), { key: 'Tab' })
    expect(editor.input().value).toBe('find-file')
    fireEvent.change(editor.input(), { target: { value: 'describe-k' } })
    fireEvent.keyDown(editor.input(), { key: 'Tab' })
    expect(editor.input().value).toBe('describe-key')
    fireEvent.keyDown(editor.input(), { key: 'Escape' })

    editor.ctrlX('b', false)
    await editor.settle()
    fireEvent.change(editor.input(), { target: { value: '*s' } })
    fireEvent.keyDown(editor.input(), { key: 'Tab' })
    expect(editor.input().value).toBe('*scratch*')
  })

  it('C-x 2, C-x 3 でウィンドウを分け, C-x o で移り, 同じバッファの書き換えを写す', async () => {
    const editor = await setup()
    /** 全てのウィンドウのエディタ. */
    const views = () =>
      [...editor.container.querySelectorAll('.cm-editor')].map(
        (el) => EditorView.findFromDOM(el as HTMLElement) as EditorView,
      )
    const inactive = () => editor.container.querySelectorAll('.mode-line-inactive').length
    await act(async () => editor.ctrlX('2', false))
    expect(views()).toHaveLength(2)
    expect(inactive()).toBe(1)
    // 書き換えは, 同じバッファを出している別のウィンドウにも写す
    await editor.type('(+ 1 2)')
    expect(views()[1].state.doc.toString()).toBe('(+ 1 2)')

    await act(async () => editor.ctrlX('o', false))
    expect(views()[1].hasFocus).toBe(true)
    // 2 つ目のウィンドウで, 別のバッファ (*Help*) を出す
    await act(async () => {
      fireEvent.keyDown(views()[1].contentDOM, { key: 'h', ctrlKey: true })
      fireEvent.keyDown(views()[1].contentDOM, { key: 'b' })
    })
    expect(views()[0].state.doc.toString()).toBe('(+ 1 2)')
    expect(views()[1].state.doc.toString()).toContain('describe-key')

    await act(async () => {
      fireEvent.keyDown(views()[1].contentDOM, { key: 'x', ctrlKey: true })
      fireEvent.keyDown(views()[1].contentDOM, { key: '3' })
    })
    expect(views()).toHaveLength(3)
    expect(editor.container.querySelectorAll('.window-left-of')).toHaveLength(1)
    // クリックなどで, フォーカスを移すと, そのウィンドウを選ぶ
    await act(async () => views()[0].focus())
    expect(inactive()).toBe(2)
    expect(
      editor.container.querySelector('.mode-line:not(.mode-line-inactive)')?.textContent,
    ).toContain('*scratch*')

    await act(async () => editor.ctrlX('1', false))
    expect(views()).toHaveLength(1)
    expect(views()[0].state.doc.toString()).toBe('(+ 1 2)')
    await act(async () => editor.ctrlX('0', false))
    expect(editor.text()).toContain('Attempt to delete minibuffer or sole ordinary window')
  })

  it('C-x 0 で選んでいるウィンドウを消し, 消したバッファを出していたウィンドウは, 別のバッファを出す', async () => {
    const editor = await setup()
    const views = () =>
      [...editor.container.querySelectorAll('.cm-editor')].map(
        (el) => EditorView.findFromDOM(el as HTMLElement) as EditorView,
      )
    const modeLine = (i: number) => editor.container.querySelectorAll('.mode-line')[i].textContent
    /** 2 つ目のウィンドウで, キーを押す. */
    const keyIn2 = (...keys: [string, boolean][]) =>
      act(async () => {
        for (const [key, ctrlKey] of keys) {
          fireEvent.keyDown(views()[1].contentDOM, { key, ctrlKey })
        }
      })
    await act(async () => editor.ctrlX('3', false))
    // 両方のウィンドウで出している *scratch* を消すと, 両方とも新しい *scratch* を出す
    await act(async () => editor.ctrlX('k', false))
    expect(modeLine(0)).toContain('*scratch*')
    expect(modeLine(1)).toContain('*scratch*')

    // 2 つ目のウィンドウに *Help* を出し, 1 つ目の *scratch* を書き換えても, *Help* は変わらない
    await act(async () => editor.ctrlX('o', false))
    await keyIn2(['h', true], ['b', false])
    await act(async () => editor.ctrlX('o', false))
    await editor.type('edited')
    expect(views()[1].state.doc.toString()).toContain('describe-key')

    // 2 つ目で *Help* を消すと, 2 つ目だけ, 別のバッファを出す
    await act(async () => editor.ctrlX('o', false))
    await keyIn2(['x', true], ['k', false])
    expect(modeLine(0)).toContain('*scratch*')
    expect(modeLine(1)).toContain('*scratch*')
    expect(views()[1].state.doc.toString()).toBe('edited')

    await keyIn2(['x', true], ['0', false])
    expect(views()).toHaveLength(1)
    expect(views()[0].hasFocus).toBe(true)
  })

  it('M-x でコマンドを実行し, ないコマンドはメッセージにする', async () => {
    const editor = await setup()

    editor.key('x', { alt: true })
    await editor.submit('find-file')
    expect(editor.input().value).toBe('/home/u/')
    await act(async () => {
      fireEvent.keyDown(editor.input(), { key: 'Escape' })
    })
    expect(editor.text()).toContain('Quit')

    editor.key('x', { alt: true })
    await editor.submit('save-buffer')
    await editor.settle()
    expect(editor.text()).toContain('Save file')
    await act(async () => {
      fireEvent.keyDown(editor.input(), { key: 'Escape' })
    })
    await editor.settle()

    editor.key('x', { alt: true })
    await editor.submit('end-of-buffer')
    expect(editor.cursor()).toBe(editor.doc().length)

    editor.key('x', { alt: true })
    await editor.submit('no-such')
    expect(editor.text()).toContain('Undefined command: no-such')
  })

  it('C-x C-c は, 変更したバッファがなければ, すぐに閉じる', async () => {
    const editor = await setup()
    await act(async () => editor.ctrlX('c'))
    expect(close).toHaveBeenCalled()
  })

  it('C-x C-f で開いただけのファイルは, 変更なしで, C-x C-c ですぐに閉じる', async () => {
    const editor = await setup()
    mocked.pathExists.mockResolvedValueOnce(true)
    mocked.readFile.mockResolvedValueOnce('(+ 1 2)')
    editor.ctrlX('f')
    await editor.submit('/tmp/a.scm')
    expect(editor.text()).toContain('-:--- a.scm')

    await act(async () => editor.ctrlX('c'))
    expect(close).toHaveBeenCalled()
  })

  it('C-x C-c は, 変更したバッファがあれば, 終わってよいか聞き, yes でだけ閉じる', async () => {
    const editor = await setup()
    await editor.type('changed')

    await act(async () => editor.ctrlX('c'))
    expect(editor.text()).toContain('Modified buffers exist; exit anyway?')
    await editor.submit('no')
    expect(close).not.toHaveBeenCalled()

    await act(async () => editor.ctrlX('c'))
    await editor.submit('yes')
    expect(close).toHaveBeenCalled()
  })

  it('窓を閉じるときは, 変更したバッファがあれば閉じずに聞き, yes と答えたら閉じる', async () => {
    const editor = await setup()
    /** 窓を閉じる頼みを送り, 閉じるのを止めたかを返す. */
    const requestClose = () => {
      const event = { preventDefault: vi.fn() }
      const handler = onCloseRequested.mock.lastCall?.[0] as (e: typeof event) => void
      act(() => handler(event))
      return event.preventDefault.mock.calls.length > 0
    }
    expect(requestClose()).toBe(false)

    await editor.type('changed')
    expect(requestClose()).toBe(true)
    expect(editor.text()).toContain('Modified buffers exist; exit anyway?')
    await editor.submit('no')
    expect(close).not.toHaveBeenCalled()

    expect(requestClose()).toBe(true)
    await editor.submit('yes')
    expect(close).toHaveBeenCalled()
    expect(requestClose()).toBe(false)
  })

  it('窓を閉じたら, 窓を閉じる頼みを受けるのをやめる', async () => {
    const editor = await setup()
    editor.unmount()
    await act(async () => {})
    expect(unlisten).toHaveBeenCalled()
  })

  it('M-g g で, 入れた行へ動く', async () => {
    const editor = await setup()
    await editor.type('a\nb\nc', 0)

    editor.key('g', { alt: true })
    editor.key('g')
    await editor.submit('3')
    expect(editor.cursor()).toBe(4)
  })

  it('C-/ で元に戻し, C-x C-/ でやり直す (履歴がなければ, 何もしない)', async () => {
    const editor = await setup()
    await act(async () => editor.ctrlX('/'))
    editor.key('/', { ctrl: true })

    await editor.type('abc\ndef', 0)
    editor.key('k', { ctrl: true })
    expect(editor.doc()).toBe('\ndef')

    editor.key('/', { ctrl: true })
    expect(editor.doc()).toBe('abc\ndef')

    editor.ctrlX('/')
    expect(editor.doc()).toBe('\ndef')
  })

  it('C-s と C-r で検索し, 繰り返し, Enter で確定, C-g でやめる', async () => {
    const editor = await setup()
    await editor.type('ab ab ab', 0)

    editor.key('s', { ctrl: true })
    fireEvent.change(editor.input(), { target: { value: 'ab' } })
    expect(editor.editor.state.selection.main).toMatchObject({ from: 0, to: 2, head: 2 })
    fireEvent.keyDown(editor.input(), { key: 's', ctrlKey: true })
    expect(editor.editor.state.selection.main).toMatchObject({ from: 3, to: 5 })
    fireEvent.keyDown(editor.input(), { key: 'r', ctrlKey: true })
    // 後ろへ探すときは, カーソルを見つかったところの始めに置く
    expect(editor.editor.state.selection.main).toMatchObject({ from: 0, to: 2, head: 0 })
    fireEvent.keyDown(editor.input(), { key: 'r', ctrlKey: true })
    fireEvent.change(editor.input(), { target: { value: 'zz' } })
    expect(editor.text()).toContain('Failing')
    fireEvent.keyDown(editor.input(), { key: 's', ctrlKey: true })
    await act(async () => {
      fireEvent.keyDown(editor.input(), { key: 'Enter' })
    })
    expect(editor.input()).toBeNull()

    // 確定すると, 見つかったところの終わり (後ろへ探したときは始め) にカーソルを置く
    editor.key('s', { ctrl: true })
    fireEvent.change(editor.input(), { target: { value: 'ab' } })
    fireEvent.keyDown(editor.input(), { key: 's', ctrlKey: true })
    await act(async () => {
      fireEvent.keyDown(editor.input(), { key: 'Enter' })
    })
    expect(editor.cursor()).toBe(5)
    editor.key('r', { ctrl: true })
    fireEvent.change(editor.input(), { target: { value: 'ab' } })
    await act(async () => {
      fireEvent.keyDown(editor.input(), { key: 'Enter' })
    })
    expect(editor.cursor()).toBe(3)

    // C-g でやめると, 探し始めた位置に戻す
    editor.key('r', { ctrl: true })
    fireEvent.change(editor.input(), { target: { value: 'ab' } })
    fireEvent.keyDown(editor.input(), { key: 'g', ctrlKey: true })
    expect(editor.text()).toContain('Quit')
    expect(editor.cursor()).toBe(3)
  })

  it('C-g は Quit を出す', async () => {
    const editor = await setup()
    editor.key('g', { ctrl: true })
    expect(editor.text()).toContain('Quit')
  })

  it('C-] (abort-recursive-edit) は, エディタに移っていても, ミニバッファをやめる', async () => {
    const editor = await setup()
    // ミニバッファを使っていなければ, 誤り
    editor.key(']', { ctrl: true })
    expect(editor.text()).toContain('No recursive edit is in progress')
    editor.key('x', { alt: true })
    expect(editor.text()).toContain('M-x')
    editor.key(']', { ctrl: true })
    expect(editor.text()).toContain('Quit')
    expect(editor.text()).not.toContain('M-x')
  })

  it('M-x abort-minibuffers は, ミニバッファの外では誤り', async () => {
    const editor = await setup()
    editor.key('x', { alt: true })
    fireEvent.change(editor.input(), { target: { value: 'abort-minibuffers' } })
    await act(async () => {
      fireEvent.keyDown(editor.input(), { key: 'Enter' })
    })
    expect(editor.text()).toContain('Not in a minibuffer')
  })

  describe('矩形とブックマーク', () => {
    /**
     * C-x r に続けて, キーを押します.
     *
     * @param editor - エディタ.
     * @param k - キー.
     */
    const ctrlXr = (editor: Awaited<ReturnType<typeof setup>>, k: string) => {
      editor.key('x', { ctrl: true })
      editor.key('r')
      editor.key(k)
    }

    /**
     * ファイルを開きます (C-x C-f).
     *
     * @param editor - エディタ.
     * @param path - パス.
     * @param content - 中身.
     */
    const open = async (
      editor: Awaited<ReturnType<typeof setup>>,
      path: string,
      content: string,
    ) => {
      mocked.pathExists.mockResolvedValueOnce(true)
      mocked.readFile.mockResolvedValueOnce(content)
      editor.ctrlX('f')
      await editor.submit(path)
    }

    it('C-x r t で, 文字を聞いて, 矩形を置き換えること (2 回目からは, 前の文字が既定)', async () => {
      const editor = await setup()
      ctrlXr(editor, 't')
      expect(editor.text()).toContain('The mark is not set now, so there is no region')
      // 前の文字が無ければ, 空の文字 (矩形を消す)
      await editor.type('xy', 0)
      editor.key(' ', { ctrl: true })
      act(() => editor.editor.dispatch({ selection: { anchor: 1 } }))
      ctrlXr(editor, 't')
      await editor.submit('')
      expect(editor.doc()).toBe('y')
      await editor.type('abc\ndef', 1)
      editor.key(' ', { ctrl: true })
      act(() => editor.editor.dispatch({ selection: { anchor: 6 } }))
      ctrlXr(editor, 't')
      expect(editor.text()).toContain('String rectangle: ')
      await editor.submit('XY')
      expect(editor.doc()).toBe('aXYc\ndXYf')
      act(() => editor.editor.dispatch({ selection: { anchor: 0 } }))
      editor.key(' ', { ctrl: true })
      act(() => editor.editor.dispatch({ selection: { anchor: 1 } }))
      ctrlXr(editor, 't')
      expect(editor.text()).toContain('String rectangle (default XY): ')
      await editor.submit('')
      expect(editor.doc()).toBe('XYXYc\ndXYf')
    })

    it('C-x r m は, ファイルのないバッファでは, 知らせること', async () => {
      const editor = await setup()
      ctrlXr(editor, 'm')
      expect(editor.text()).toContain('Buffer not visiting a file or directory')
    })

    it('C-x r m で, ファイルの名前を既定の名前として, ブックマークを付けて保存すること', async () => {
      const editor = await setup()
      await open(editor, '/tmp/a.scm', '(define x 1)\n')
      act(() => editor.editor.dispatch({ selection: { anchor: 8 } }))
      ctrlXr(editor, 'm')
      expect(editor.text()).toContain('Set bookmark named (default a.scm): ')
      await editor.submit('')
      expect(mockedBookmarks.saveBookmarks).toHaveBeenCalledWith([
        { name: 'a.scm', filename: '/tmp/a.scm', position: 8, front: 'x 1)\n', rear: '(define ' },
      ])
      // 保存できなければ, 知らせる
      mockedBookmarks.saveBookmarks.mockRejectedValueOnce('disk full')
      ctrlXr(editor, 'm')
      await editor.submit('other')
      expect(editor.text()).toContain('Error: disk full')
    })

    it('C-x r b で, ブックマークの位置へ動くこと (既定は, 最後に使ったもの)', async () => {
      const editor = await setup()
      await open(editor, '/tmp/a.scm', '(define x 1)\n')
      act(() => editor.editor.dispatch({ selection: { anchor: 8 } }))
      ctrlXr(editor, 'm')
      await editor.submit('here')
      // 同じバッファ
      act(() => editor.editor.dispatch({ selection: { anchor: 0 } }))
      ctrlXr(editor, 'b')
      expect(editor.text()).toContain('Jump to bookmark (default here): ')
      await editor.submit('')
      expect(editor.cursor()).toBe(8)
      // 別のバッファから
      editor.ctrlX('b', false)
      await editor.submit('*scratch*')
      ctrlXr(editor, 'b')
      await editor.submit('here')
      expect(editor.doc()).toBe('(define x 1)\n')
      expect(editor.cursor()).toBe(8)
      // 無い名前
      ctrlXr(editor, 'b')
      await editor.submit('nope')
      expect(editor.text()).toContain('Invalid bookmark nope')
    })

    it('C-x r b で, 開いていないファイルを開き, 前後の文字で位置を直すこと', async () => {
      mockedBookmarks.loadBookmarks.mockResolvedValue([
        { name: 'b', filename: '/tmp/b.scm', position: 2, front: 'cd', rear: 'ab' },
      ])
      const editor = await setup()
      ctrlXr(editor, 'b')
      expect(editor.text()).toContain('Jump to bookmark: ')
      await editor.submit('')
      expect(editor.text()).toContain('No bookmark specified')
      // 読めなければ, 知らせる
      mocked.readFile.mockRejectedValueOnce('gone')
      ctrlXr(editor, 'b')
      await editor.submit('b')
      expect(editor.text()).toContain('Error: gone')
      mocked.readFile.mockResolvedValueOnce('xxabcd')
      ctrlXr(editor, 'b')
      await editor.submit('b')
      expect(editor.text()).toContain('b.scm')
      expect(editor.cursor()).toBe(4)
    })

    it('ブックマークのファイルを読めなければ, 知らせること', async () => {
      mockedBookmarks.loadBookmarks.mockRejectedValue('bad json')
      const editor = await setup()
      expect(editor.text()).toContain('Bookmark file error: bad json')
    })
  })

  describe('ファイルのコマンド, 自動保存とバックアップ, dired', () => {
    afterEach(() => {
      vi.useRealTimers()
    })

    /**
     * ファイルを開きます (C-x C-f).
     *
     * @param editor - エディタ.
     * @param path - パス.
     * @param content - 中身.
     */
    const open = async (
      editor: Awaited<ReturnType<typeof setup>>,
      path: string,
      content: string,
    ) => {
      mocked.pathExists.mockResolvedValueOnce(true)
      mocked.readFile.mockResolvedValueOnce(content)
      editor.ctrlX('f')
      await editor.submit(path)
    }

    /**
     * M-x でコマンドを実行します.
     *
     * @param editor - エディタ.
     * @param name - コマンドの名前.
     */
    const mx = async (editor: Awaited<ReturnType<typeof setup>>, name: string) => {
      editor.key('x', { alt: true })
      await editor.submit(name)
    }

    /** モードライン (選んでいるウィンドウ) の文字. */
    const modeLine = (editor: Awaited<ReturnType<typeof setup>>) =>
      editor.container.querySelector('.mode-line')?.textContent ?? ''

    it('始めて保存するときだけ, 前の版をバックアップにし, 自動保存のファイルを消すこと', async () => {
      const editor = await setup()
      await open(editor, '/tmp/a.txt', 'a')
      await editor.type('ab')
      await act(async () => editor.ctrlX('s'))
      await editor.settle()
      expect(mocked.backupFile).toHaveBeenCalledWith('/tmp/a.txt')
      expect(mocked.deleteFile).toHaveBeenCalledWith('/tmp/#a.txt#')
      await editor.type('abc')
      await act(async () => editor.ctrlX('s'))
      await editor.settle()
      expect(mocked.backupFile).toHaveBeenCalledTimes(1)
      // 別のファイルに書くときは, そのファイルをバックアップにする
      editor.ctrlX('w')
      await editor.settle()
      await editor.submit('/tmp/b.txt')
      expect(mocked.backupFile).toHaveBeenLastCalledWith('/tmp/b.txt')
      // バックアップや, 自動保存のファイルの削除に失敗しても, 保存はする
      await open(editor, '/tmp/c.txt', 'c')
      mocked.backupFile.mockRejectedValueOnce('denied')
      mocked.deleteFile.mockRejectedValueOnce('denied')
      await editor.type('cd')
      await act(async () => editor.ctrlX('s'))
      await editor.settle()
      expect(editor.text()).toContain('Wrote /tmp/c.txt')
    })

    it('書き換えが止まってしばらくしたら, 変更したファイルのバッファを自動保存すること', async () => {
      const editor = await setup()
      vi.useFakeTimers({ toFake: ['setTimeout', 'clearTimeout'] })
      // ファイルのないバッファは, 自動保存しない
      editor.type('scratch')
      await act(async () => {
        vi.advanceTimersByTime(AUTO_SAVE_TIMEOUT)
      })
      await open(editor, '/tmp/a.txt', 'a')
      editor.type('b')
      await act(async () => {
        vi.advanceTimersByTime(AUTO_SAVE_TIMEOUT - 1)
      })
      expect(mocked.writeFile).not.toHaveBeenCalled()
      await act(async () => {
        vi.advanceTimersByTime(1)
      })
      expect(mocked.writeFile).toHaveBeenCalledWith('/tmp/#a.txt#', 'b')
      expect(editor.text()).toContain('Auto-saving...done')
      // 同じ本文は, もう保存しない
      editor.ctrlX('b', false)
      await act(async () => {
        vi.advanceTimersByTime(AUTO_SAVE_TIMEOUT)
      })
      expect(mocked.writeFile).toHaveBeenCalledTimes(1)
      // ミニバッファで入力しているあいだは, 知らせない
      await editor.submit('a.txt')
      editor.type('c')
      editor.key('x', { alt: true })
      await act(async () => {
        vi.advanceTimersByTime(AUTO_SAVE_TIMEOUT)
      })
      expect(mocked.writeFile).toHaveBeenLastCalledWith('/tmp/#a.txt#', 'c')
      expect(editor.text()).toContain('M-x ')
      expect(editor.text()).not.toContain('Auto-saving')
      // 失敗したら, 知らせる
      await editor.submit('keyboard-quit')
      editor.type('d')
      mocked.writeFile.mockRejectedValueOnce('disk full')
      await act(async () => {
        vi.advanceTimersByTime(AUTO_SAVE_TIMEOUT)
      })
      expect(editor.text()).toContain('Error: disk full')
    })

    it('自動保存のファイルの方が新しければ, 開いたときに知らせ, recover-this-file で読み直すこと', async () => {
      const editor = await setup()
      mocked.fileNewerThan.mockResolvedValueOnce(true)
      await open(editor, '/tmp/a.txt', 'old')
      expect(mocked.fileNewerThan).toHaveBeenCalledWith('/tmp/#a.txt#', '/tmp/a.txt')
      expect(editor.text()).toContain('a.txt has auto save data; consider M-x recover-this-file')

      mocked.pathExists.mockResolvedValue(true)
      mocked.fileNewerThan.mockResolvedValue(true)
      await mx(editor, 'recover-this-file')
      expect(editor.text()).toContain('Recover auto save file /tmp/#a.txt#? (yes or no) ')
      await editor.submit('no')
      expect(editor.doc()).toBe('old')
      await mx(editor, 'recover-this-file')
      mocked.readFile.mockResolvedValueOnce('saved')
      await editor.submit('yes')
      expect(mocked.readFile).toHaveBeenLastCalledWith('/tmp/#a.txt#')
      expect(editor.doc()).toBe('saved')
      expect(modeLine(editor)).toContain('**')
      // 読めなければ, 知らせる
      await mx(editor, 'recover-this-file')
      mocked.readFile.mockRejectedValueOnce('gone')
      await editor.submit('yes')
      expect(editor.text()).toContain('Error: gone')
    })

    it('recover-this-file は, 自動保存のファイルが無いか, 古ければ, 知らせること', async () => {
      const editor = await setup()
      await mx(editor, 'recover-this-file')
      expect(editor.text()).toContain('This buffer is not visiting a file')
      await open(editor, '/tmp/a.txt', 'a')
      mocked.pathExists.mockResolvedValueOnce(true).mockResolvedValueOnce(false)
      await mx(editor, 'recover-this-file')
      expect(editor.text()).toContain('Auto save file /tmp/#a.txt# does not exist')
      mocked.pathExists.mockResolvedValueOnce(false)
      await mx(editor, 'recover-this-file')
      expect(editor.text()).toContain('Auto-save file /tmp/#a.txt# not current')
      mocked.pathExists.mockRejectedValueOnce('denied')
      await mx(editor, 'recover-this-file')
      expect(editor.text()).toContain('Error: denied')
    })

    it('revert-buffer で, 聞いてから, ファイルから読み直すこと', async () => {
      const editor = await setup()
      await mx(editor, 'revert-buffer')
      expect(editor.text()).toContain('Buffer does not seem to be associated with any file')
      await open(editor, '/tmp/a.txt', 'abc')
      await mx(editor, 'revert-buffer')
      expect(editor.text()).toContain('Revert buffer from file /tmp/a.txt? (yes or no) ')
      await editor.submit('no')
      // 同じ本文なら, 書き換えない
      await mx(editor, 'revert-buffer')
      mocked.readFile.mockResolvedValueOnce('abc')
      await editor.submit('yes')
      expect(editor.doc()).toBe('abc')
      editor.type('xbc', 2)
      await mx(editor, 'revert-buffer')
      expect(editor.text()).toContain('Discard edits and reread from /tmp/a.txt? (yes or no) ')
      mocked.readFile.mockResolvedValueOnce('abc')
      await editor.submit('yes')
      expect(editor.doc()).toBe('abc')
      expect(modeLine(editor)).not.toContain('**')
      // 元に戻せる
      editor.key('/', { ctrl: true })
      expect(editor.doc()).toBe('xbc')
      // 読めなければ, 知らせる
      await mx(editor, 'revert-buffer')
      mocked.readFile.mockRejectedValueOnce('gone')
      await editor.submit('yes')
      expect(editor.text()).toContain('Error: gone')
    })

    it('C-x d で, ディレクトリの一覧を dired のバッファに出し, revert-buffer で作り直すこと', async () => {
      const editor = await setup()
      mocked.listDirectoryLong.mockResolvedValueOnce(
        'total 4\n-rw-r--r-- 1 u u 3 Oct 25 16:42 a.txt\n',
      )
      editor.ctrlX('d', false)
      expect(editor.text()).toContain('Dired (directory): ')
      expect(editor.input().value).toBe('/home/u/')
      await editor.submit('/home/u/src/')
      expect(mocked.listDirectoryLong).toHaveBeenCalledWith('/home/u/src')
      expect(editor.doc()).toBe('  /home/u/src:\n  -rw-r--r-- 1 u u 3 Oct 25 16:42 a.txt\n')
      expect(modeLine(editor)).toContain('src')
      expect(modeLine(editor)).toContain('Dired by name')
      // dired のバッファの既定のディレクトリは, そのディレクトリ
      editor.ctrlX('f')
      expect(editor.input().value).toBe('/home/u/src/')
      await act(async () => {
        fireEvent.keyDown(editor.input(), { key: 'g', ctrlKey: true })
      })
      // 作り直す (聞かない)
      mocked.listDirectoryLong.mockResolvedValueOnce(
        'total 0\n-rw-r--r-- 1 u u 3 Oct 25 16:42 b.txt\n',
      )
      await mx(editor, 'revert-buffer')
      expect(editor.doc()).toBe('  /home/u/src:\n  -rw-r--r-- 1 u u 3 Oct 25 16:42 b.txt\n')
      expect(modeLine(editor)).not.toContain('**')
      // 同じディレクトリの dired があれば, それを出す
      editor.ctrlX('b', false)
      await editor.submit('*scratch*')
      editor.ctrlX('d', false)
      await editor.submit('/home/u/src')
      expect(mocked.listDirectoryLong).toHaveBeenCalledTimes(2)
      expect(editor.doc()).toContain('b.txt')
      // ルートのディレクトリ. 読めなければ, 知らせる
      mocked.listDirectoryLong.mockResolvedValueOnce('')
      editor.ctrlX('d', false)
      await editor.submit('/')
      expect(editor.doc()).toBe('  /:\n')
      mocked.listDirectoryLong.mockRejectedValueOnce('No such file or directory')
      editor.ctrlX('d', false)
      expect(editor.input().value).toBe('/')
      await editor.submit('/none')
      expect(editor.text()).toContain('Error: No such file or directory')
    })
  })

  describe('isearch の取り込みと履歴, ミニバッファの履歴', () => {
    /**
     * ミニバッファで, キーを押します.
     *
     * @param editor - エディタ.
     * @param key - キー.
     * @param mods - 修飾.
     */
    const press = (editor: Awaited<ReturnType<typeof setup>>, key: string, mods: Mods = {}) =>
      act(async () => {
        fireEvent.keyDown(editor.input(), {
          key,
          ctrlKey: mods.ctrl ?? false,
          altKey: mods.alt ?? false,
        })
      })

    /** 選んでいる範囲. */
    const selection = (editor: Awaited<ReturnType<typeof setup>>) => {
      const { from, to } = editor.editor.state.selection.main
      return [from, to]
    }

    it('isearch の C-w で, 単語か文字を取り込み, 正規表現では, そのまま合うようにする', async () => {
      const editor = await setup()
      await editor.type('foo.bar foo.bar', 0)
      editor.key('s', { ctrl: true })
      await press(editor, 'w', { ctrl: true })
      expect(editor.input().value).toBe('foo')
      expect(selection(editor)).toEqual([0, 3])
      // 合った所の終わりから取り込む
      await press(editor, 'w', { ctrl: true })
      expect(editor.input().value).toBe('foo.bar')
      expect(selection(editor)).toEqual([0, 7])
      await press(editor, 'Enter')
      // 正規表現では, 取り込んだ文字を, そのまま合うようにする
      await editor.type('foo.bar foo.bar', 0)
      editor.key('s', { ctrl: true, alt: true })
      expect(editor.text()).toContain('Regexp I-search: ')
      await press(editor, 'w', { ctrl: true })
      await press(editor, 'w', { ctrl: true })
      await press(editor, 'w', { ctrl: true })
      expect(editor.input().value).toBe('foo\\.bar foo')
      expect(selection(editor)).toEqual([0, 11])
      await press(editor, 'Enter')
      // 本文の終わりからは, 取り込まない
      await editor.type('foo.bar foo.bar')
      editor.key('s', { ctrl: true })
      await press(editor, 'w', { ctrl: true })
      expect(editor.input().value).toBe('')
      await press(editor, 'Enter')
      // 前へ探していても, 合った所の終わりから取り込む
      editor.key('r', { ctrl: true })
      fireEvent.change(editor.input(), { target: { value: 'foo' } })
      await press(editor, 'w', { ctrl: true })
      expect(editor.input().value).toBe('foo.bar')
      expect(selection(editor)).toEqual([8, 15])
    })

    it('合わないときの C-w は, 前に取り込んだ所の続きから取り込む', async () => {
      const editor = await setup()
      await editor.type('ab ac', 0)
      editor.key('s', { ctrl: true })
      fireEvent.change(editor.input(), { target: { value: 'z' } })
      await press(editor, 'w', { ctrl: true })
      expect(editor.input().value).toBe('zab')
      await press(editor, 'w', { ctrl: true })
      expect(editor.input().value).toBe('zab ac')
      expect(editor.text()).toContain('Failing I-search: ')
    })

    it('isearch の M-y で, キルした文字を取り込み, 続けると, 古いキルした文字に替える', async () => {
      const editor = await setup()
      // キルした文字が無ければ, 取り込まない
      editor.key('s', { ctrl: true })
      await press(editor, 'y', { alt: true })
      expect(editor.input().value).toBe('')
      await press(editor, 'Enter')
      await editor.type('xyz\nabc\nxyz abc', 0)
      editor.key('k', { ctrl: true })
      editor.key('d', { ctrl: true })
      editor.key('k', { ctrl: true })
      expect(editor.doc()).toBe('\nxyz abc')
      editor.key('s', { ctrl: true })
      await press(editor, 'y', { alt: true })
      expect(editor.input().value).toBe('abc')
      expect(selection(editor)).toEqual([5, 8])
      await press(editor, 'y', { alt: true })
      expect(editor.input().value).toBe('xyz')
      expect(selection(editor)).toEqual([1, 4])
      // ほかのコマンドの後の M-y は, 足す
      await press(editor, 's', { ctrl: true })
      await press(editor, 'y', { alt: true })
      expect(editor.input().value).toBe('xyzxyz')
      fireEvent.change(editor.input(), { target: { value: 'x' } })
      await press(editor, 'y', { alt: true })
      expect(editor.input().value).toBe('xxyz')
      await press(editor, 'r', { ctrl: true })
      await press(editor, 'y', { alt: true })
      expect(editor.input().value).toBe('xxyzxyz')
      // 正規表現では, そのまま合うようにする
      await press(editor, 'Enter')
      await editor.type('a.b', 0)
      editor.key('k', { ctrl: true })
      editor.key('s', { ctrl: true, alt: true })
      await press(editor, 'y', { alt: true })
      expect(editor.input().value).toBe('a\\.b')
    })

    it('isearch の M-p と M-n で検索の履歴を出し, C-s を続けると, 最後に探した文字で探す', async () => {
      const editor = await setup()
      await editor.type('ab cd ab cd', 0)
      const search = async (query: string) => {
        editor.key('s', { ctrl: true })
        fireEvent.change(editor.input(), { target: { value: query } })
        await press(editor, 'Enter')
      }
      // 履歴が無ければ, 何もしない
      editor.key('s', { ctrl: true })
      await press(editor, 'p', { alt: true })
      await press(editor, 's', { ctrl: true })
      expect(editor.input().value).toBe('')
      await press(editor, 'Enter')
      await search('ab')
      await search('cd')
      // C-g でやめた文字と, 空の文字は, 履歴に入れない
      editor.key('s', { ctrl: true })
      fireEvent.change(editor.input(), { target: { value: 'zz' } })
      await press(editor, 'g', { ctrl: true })
      await search('')
      await editor.type('ab cd ab cd', 0)
      editor.key('s', { ctrl: true })
      await press(editor, 'p', { alt: true })
      expect(editor.input().value).toBe('cd')
      expect(selection(editor)).toEqual([3, 5])
      await press(editor, 'p', { alt: true })
      expect(editor.input().value).toBe('ab')
      await press(editor, 'p', { alt: true })
      expect(editor.input().value).toBe('cd')
      await press(editor, 'n', { alt: true })
      expect(editor.input().value).toBe('ab')
      await press(editor, 'Enter')
      // 同じ文字は, 前の方から除く
      editor.key('s', { ctrl: true })
      await press(editor, 'n', { alt: true })
      expect(editor.input().value).toBe('cd')
      await press(editor, 'g', { ctrl: true })
      // C-s を続けると, 最後に探した文字. その後の M-p は, 1 つ古い文字
      await editor.type('ab cd ab cd', 0)
      editor.key('s', { ctrl: true })
      await press(editor, 's', { ctrl: true })
      expect(editor.input().value).toBe('ab')
      expect(selection(editor)).toEqual([0, 2])
      await press(editor, 'p', { alt: true })
      expect(editor.input().value).toBe('cd')
      await press(editor, 'g', { ctrl: true })
      // 正規表現の履歴は, 別. 向きを変えた C-r は, 履歴を使わない
      editor.key('s', { ctrl: true, alt: true })
      await press(editor, 'p', { alt: true })
      await press(editor, 's', { ctrl: true })
      await press(editor, 'r', { ctrl: true })
      expect(editor.input().value).toBe('')
    })

    it('ミニバッファの M-p と M-n で, 入力の履歴を出し, 端では知らせる', async () => {
      const editor = await setup()
      await editor.type('abc', 0)
      editor.key('x', { alt: true })
      await press(editor, 'p', { alt: true })
      expect(editor.text()).toContain('[Beginning of history; no preceding item]')
      await press(editor, 'n', { alt: true })
      expect(editor.text()).toContain('[End of history; no default available]')
      // 入力すると, 知らせを消す
      fireEvent.change(editor.input(), { target: { value: 'forward-char' } })
      expect(editor.text()).not.toContain('[End of history')
      await press(editor, 'Enter')
      expect(editor.cursor()).toBe(1)
      editor.key('x', { alt: true })
      await editor.submit('forward-word')
      editor.key('x', { alt: true })
      fireEvent.change(editor.input(), { target: { value: 'typed' } })
      await press(editor, 'p', { alt: true })
      expect(editor.input().value).toBe('forward-word')
      await press(editor, 'p', { alt: true })
      expect(editor.input().value).toBe('forward-char')
      await press(editor, 'p', { alt: true })
      expect(editor.text()).toContain('[Beginning of history; no preceding item]')
      await press(editor, 'n', { alt: true })
      await press(editor, 'n', { alt: true })
      // 履歴を出す前に入力していた文字に戻る
      expect(editor.input().value).toBe('typed')
      await press(editor, 'g', { ctrl: true })
      // 履歴は, コマンドの種類ごと (C-x C-f と C-x C-w は, 同じ履歴)
      mocked.pathExists.mockResolvedValue(false)
      editor.ctrlX('f')
      await editor.submit('/tmp/a.txt')
      editor.ctrlX('w')
      await editor.settle()
      await press(editor, 'p', { alt: true })
      expect(editor.input().value).toBe('/tmp/a.txt')
      await press(editor, 'p', { alt: true })
      expect(editor.text()).toContain('[Beginning of history; no preceding item]')
    })
  })

  describe('Lisp からの, バッファ, 入力, キー, 色とフォント', () => {
    afterEach(() => {
      document.documentElement.removeAttribute('style')
      delete document.documentElement.dataset.backgroundMode
    })

    /**
     * 評価器の頼みを送る関数と, 最後の返事を返す関数を作ります.
     *
     * @returns 送る関数と, 返事を返す関数.
     */
    const requests = () => {
      const handler = mockedListen.mock.calls[0][1]
      let id = 100
      /** 頼みを送って, 返事を待つ. */
      const send = async (op: string, ...args: unknown[]) => {
        const sent = ++id
        await act(async () => {
          handler({ event: 'editor-request', id: 0, payload: { id: sent, op, args } })
        })
        await vi.waitFor(() =>
          expect(
            mockedInvoke.mock.calls.some(
              ([name, reply]) => name === 'editor_reply' && (reply as { id: number }).id === sent,
            ),
          ).toBe(true),
        )
        const [, reply] = mockedInvoke.mock.calls.find(
          ([name, reply]) => name === 'editor_reply' && (reply as { id: number }).id === sent,
        ) as [string, { value: unknown; error: string | null }]
        return reply
      }
      /** ファイルを開いて, そのバッファを出す (lisp/files.scm の find-file と同じ順). */
      const findFile = async (path: string) => {
        const opened = await send('find-file-noselect', path)
        return opened.error === null ? send('pop-to-buffer-same-window', opened.value) : opened
      }
      return { send, findFile }
    }

    it('バッファを出してから, 続きの操作をする', async () => {
      const editor = await setup()
      const { send } = requests()
      expect(await send('pop-to-buffer-same-window', '*howm*')).toMatchObject({
        value: null,
        error: null,
      })
      expect(editor.text()).toContain('*howm*')
      await send('insert', 'menu')
      expect(editor.doc()).toBe('menu')
      expect(editor.text()).toContain('**')
      await send('set-buffer-modified-p', false)
      expect(editor.text()).not.toContain('**')
      // 前からあるバッファと, 出しているバッファ
      await send('pop-to-buffer-same-window', '*scratch*')
      expect(editor.doc()).toContain('This buffer is for text')
      expect(await send('pop-to-buffer-same-window', '*scratch*')).toMatchObject({ error: null })
      await send('pop-to-buffer-same-window', '*howm*')
      expect(editor.doc()).toBe('menu')
      expect(await send('buffer-file-name')).toMatchObject({ value: null })
    })

    it('コマンドのキーと, 画面のコマンドの名前を答える (C-h f)', async () => {
      // Lisp のキーは, 同じキーの既定のキーより先に使う
      const keys = [
        { key: 'Ctrl-h f', command: 'describe-function' },
        { key: 'Ctrl-x Ctrl-s', command: 'my-save' },
      ]
      defaultResponse = response('', null, { commands: ['describe-function', 'my-save'], keys })
      await setup()
      const { send } = requests()
      expect(await send('where-is-internal', 'describe-function')).toMatchObject({
        value: ['C-h f'],
      })
      expect(await send('where-is-internal', 'my-save')).toMatchObject({ value: ['C-x C-s'] })
      expect(await send('where-is-internal', 'save-buffer')).toMatchObject({ value: [] })
      expect(await send('where-is-internal', 'forward-char')).toMatchObject({
        value: expect.arrayContaining(['C-f']),
      })
      const names = (await send('editor-command-names')).value as string[]
      // エディタのコマンドと, 画面のコマンド
      expect(names).toEqual(expect.arrayContaining(['forward-char', 'save-buffer', 'list-buffers']))
    })

    it('ファイルを開く (開いていれば, そのバッファを出す)', async () => {
      const editor = await setup()
      const { send, findFile } = requests()
      mocked.pathExists.mockResolvedValue(true)
      mocked.readFile.mockResolvedValue('note')
      await findFile('/home/u/howm/a.txt')
      expect(editor.doc()).toBe('note')
      expect(await send('buffer-file-name')).toMatchObject({ value: '/home/u/howm/a.txt' })
      await send('pop-to-buffer-same-window', '*scratch*')
      await findFile('/home/u/howm/a.txt')
      expect(editor.doc()).toBe('note')
      expect(mocked.readFile).toHaveBeenCalledTimes(1)
      // 無いファイル
      mocked.pathExists.mockResolvedValue(false)
      await findFile('/home/u/howm/new.txt')
      expect(editor.doc()).toBe('')
      expect(editor.text()).toContain('(New file)')
      // 読めない
      mocked.pathExists.mockRejectedValueOnce(new Error('denied'))
      expect(await findFile('/x')).toMatchObject({ error: 'denied' })
    })

    it('バッファだけのキーを, Lisp のキーより先に使う', async () => {
      const defined = { commands: ['hello'], keys: [{ key: 'Ctrl-c h', command: 'hello' }] }
      mockedInvoke.mockResolvedValueOnce(response('', null, defined))
      const editor = await setup()
      const { send } = requests()
      await send('pop-to-buffer-same-window', '*howm*')
      await send('local-set-key', 'c', 'hello')
      await send('local-set-key', 'Ctrl-c h', 'other')
      await send('local-set-key', 'c', 'hello')
      mockedInvoke.mockResolvedValueOnce(response('', null, defined))
      await act(async () => {
        editor.key('c')
      })
      expect(mockedInvoke).toHaveBeenCalledWith('run_lisp_command', { name: 'hello' })
      expect(editor.doc()).toBe('')
      // Lisp のキーが変わっても, バッファだけのキーを先に使う
      const more = { commands: ['hello', 'bye'], keys: [{ key: 'c', command: 'bye' }] }
      mockedInvoke.mockResolvedValueOnce(response('', null, more))
      editor.key(':', { alt: true })
      await editor.submit('(global-set-key "c" (quote bye))')
      mockedInvoke.mockClear()
      mockedInvoke.mockResolvedValue(response('', null, more))
      await act(async () => {
        editor.key('c')
      })
      expect(mockedInvoke).toHaveBeenCalledWith('run_lisp_command', { name: 'hello' })
      // ほかのバッファでは, 使わない
      await send('pop-to-buffer-same-window', '*scratch*')
      mockedInvoke.mockClear()
      await act(async () => {
        editor.key('c')
      })
      expect(mockedInvoke).toHaveBeenCalledWith('run_lisp_command', { name: 'bye' })
    })

    it('ミニバッファで入力してもらう (C-g でやめたら, Quit)', async () => {
      const editor = await setup()
      const { send } = requests()
      const handler = mockedListen.mock.calls[0][1]
      await act(async () => {
        handler({
          event: 'editor-request',
          id: 0,
          payload: { id: 1, op: 'read-string', args: ['Search: ', 'ab'] },
        })
      })
      expect(editor.text()).toContain('Search: ')
      expect(editor.input().value).toBe('ab')
      await editor.submit('abc')
      await vi.waitFor(() =>
        expect(mockedInvoke).toHaveBeenCalledWith('editor_reply', {
          id: 1,
          value: 'abc',
          error: null,
        }),
      )
      await act(async () => {
        handler({
          event: 'editor-request',
          id: 0,
          payload: { id: 2, op: 'read-string', args: ['Search: ', ''] },
        })
      })
      await act(async () => {
        fireEvent.keyDown(editor.input(), { key: 'g', ctrlKey: true })
      })
      await vi.waitFor(() =>
        expect(mockedInvoke).toHaveBeenCalledWith('editor_reply', {
          id: 2,
          value: null,
          error: 'Quit',
        }),
      )
      // 評価が Quit で終わったら, Quit とだけ出す
      mockedInvoke.mockResolvedValueOnce(response('', 'Quit'))
      editor.key(':', { alt: true })
      await editor.submit('(read-string "a")')
      expect(editor.text()).toContain('Quit')
      expect(editor.text()).not.toContain('Error: Quit')
      expect(await send('point')).toMatchObject({ error: null })
    })

    it('1 つのキーで答えてもらう (C-g でやめたら, Quit)', async () => {
      const editor = await setup()
      const handler = mockedListen.mock.calls[0][1]
      const ask = (id: number, args: unknown[]) =>
        act(async () => {
          handler({ event: 'editor-request', id: 0, payload: { id, op: 'read-key', args } })
        })
      const press = (key: string, ctrlKey = false) =>
        act(async () => {
          fireEvent.keyDown(editor.input(), { key, ctrlKey })
        })
      await ask(1, ['Save? '])
      expect(editor.text()).toContain('Save? ')
      await press('y')
      await vi.waitFor(() =>
        expect(mockedInvoke).toHaveBeenCalledWith('editor_reply', {
          id: 1,
          value: 'y',
          error: null,
        }),
      )
      expect(editor.text()).not.toContain('Save? ')
      // 問いを省ける
      await ask(2, [])
      await press('g', true)
      await vi.waitFor(() =>
        expect(mockedInvoke).toHaveBeenCalledWith('editor_reply', {
          id: 2,
          value: null,
          error: 'Quit',
        }),
      )
      expect(editor.text()).toContain('Quit')
    })

    it('バッファを, ファイルに書く (名前を渡すと, そのバッファ. 書けなければ, 失敗)', async () => {
      const editor = await setup()
      const { send } = requests()
      await send('pop-to-buffer-same-window', '*b*')
      await send('insert', 'x')
      await send('pop-to-buffer-same-window', '*scratch*')
      expect(await send('files--write-buffer', '/tmp/b.txt', '*b*')).toMatchObject({
        value: null,
        error: null,
      })
      expect(mocked.writeFile).toHaveBeenCalledWith('/tmp/b.txt', 'x')
      expect(editor.text()).toContain('Wrote /tmp/b.txt')
      expect(await send('buffer-list')).toMatchObject({ value: ['*scratch*', 'b.txt'] })
      expect(await send('buffer-modified-p', 'b.txt')).toMatchObject({ value: false })
      // 書き換えていないことにするのは, 書き換えていなければ, 何もしない
      expect(await send('set-buffer-modified-p', false)).toMatchObject({ error: null })
      mocked.writeFile.mockRejectedValueOnce('disk full')
      expect(await send('files--write-buffer', '/tmp/s.txt')).toMatchObject({
        error: 'disk full',
      })
      expect(await send('buffer-file-name')).toMatchObject({ value: null })
    })

    it('バッファの一覧, 書き換えたか, 消す', async () => {
      const editor = await setup()
      const { send, findFile } = requests()
      await send('pop-to-buffer-same-window', '*b*')
      expect(await send('buffer-modified-p')).toMatchObject({ value: false })
      await send('insert', 'x')
      expect(await send('buffer-modified-p', '*b*')).toMatchObject({ value: true })
      expect(await send('buffer-list')).toMatchObject({ value: ['*scratch*', '*b*'] })
      expect(await send('buffer-file-name', 'zz')).toMatchObject({ error: 'No such buffer: zz' })
      // 書き換えていても, ファイルの無いバッファは消す
      expect(await send('kill-buffer', '*b*')).toMatchObject({ value: true, error: null })
      expect(await send('buffer-list')).toMatchObject({ value: ['*scratch*'] })
      // 書き換えたファイルのバッファは, 消さない
      mocked.pathExists.mockResolvedValue(true)
      mocked.readFile.mockResolvedValue('note')
      await findFile('/home/u/a.txt')
      await send('insert', 'x')
      expect(await send('kill-buffer')).toMatchObject({ error: 'Buffer a.txt is modified' })
      await send('set-buffer-modified-p', false)
      expect(await send('kill-buffer')).toMatchObject({ error: null })
      expect(editor.doc()).toContain('This buffer is for text')
    })

    it('kill ring に入れ, 補完して入力してもらう', async () => {
      const editor = await setup()
      const { send } = requests()
      await send('kill-new', 'killed')
      await act(async () => {
        editor.key('y', { ctrl: true })
      })
      expect(editor.doc()).toContain('killed')
      const handler = mockedListen.mock.calls[0][1]
      await act(async () => {
        handler({
          event: 'editor-request',
          id: 0,
          payload: { id: 1, op: 'completing-read', args: ['Keyword: ', ['Emacs', 'howm'], 'h'] },
        })
      })
      expect(editor.text()).toContain('Keyword: ')
      expect(editor.input().value).toBe('h')
      await editor.submit('howm')
      await vi.waitFor(() =>
        expect(mockedInvoke).toHaveBeenCalledWith('editor_reply', {
          id: 1,
          value: 'howm',
          error: null,
        }),
      )
    })

    it('バッファを, ほかのウィンドウに出す', async () => {
      const editor = await setup()
      const { send } = requests()
      const editors = () => editor.container.querySelectorAll('.cm-editor')
      const cursorLine = (index: number) => {
        const view = EditorView.findFromDOM(editors()[index] as HTMLElement) as EditorView
        return view.state.doc.lineAt(view.state.selection.main.head).number
      }
      await send('pop-to-buffer-same-window', '*b*')
      await send('insert', 'a\nb\nc')
      await send('pop-to-buffer-same-window', '*scratch*')
      // 出していないバッファは, ほかのウィンドウに出して, 行へ動く
      expect(await send('display-buffer', '*b*', 3)).toMatchObject({ error: null })
      expect(editors()).toHaveLength(2)
      expect(cursorLine(1)).toBe(3)
      expect(editor.doc()).toContain('This buffer is for text')
      // 出しているバッファは, そのウィンドウ
      await send('display-buffer', '*b*', 1)
      expect(cursorLine(1)).toBe(1)
      await send('display-buffer', '*scratch*')
      expect(editors()).toHaveLength(2)
      // 無いバッファは作る
      await send('display-buffer', '*new*', 1)
      expect(await send('buffer-list')).toMatchObject({ value: ['*scratch*', '*b*', '*new*'] })
    })

    it('ファイルを, 出さずに開く', async () => {
      const editor = await setup()
      const { send } = requests()
      mocked.pathExists.mockResolvedValue(true)
      mocked.readFile.mockResolvedValue('note')
      expect(await send('find-file-noselect', '/home/u/a.txt')).toMatchObject({ value: 'a.txt' })
      expect(editor.doc()).toContain('This buffer is for text')
      expect(await send('find-file-noselect', '/home/u/a.txt')).toMatchObject({ value: 'a.txt' })
      expect(mocked.readFile).toHaveBeenCalledTimes(1)
      // 同じ名前のファイルは, 名前を変える
      const other = await send('find-file-noselect', '/home/v/a.txt')
      expect(other.value).not.toBe('a.txt')
      expect(await send('buffer-file-name', other.value)).toMatchObject({ value: '/home/v/a.txt' })
    })

    it('正規表現に合う所に, 下線を引く (バッファごと)', async () => {
      const editor = await setup()
      const { send } = requests()
      const marks = () =>
        [...editor.container.querySelectorAll('.cm-lisp-highlight')].map((e) => e.textContent)
      await send('pop-to-buffer-same-window', '*b*')
      await send('insert', 'menu memo')
      await send('highlight-regexp', 'me.')
      await send('highlight-regexp', 'o')
      await send('highlight-regexp', 'me.')
      expect(marks()).toEqual(['men', 'mem', 'o'])
      expect(await send('highlight-regexp', '(')).toMatchObject({
        error: expect.stringContaining('Invalid regular expression'),
      })
      // ほかのバッファには引かない. 戻ると, また引く
      await send('pop-to-buffer-same-window', '*scratch*')
      expect(marks()).toEqual([])
      await send('pop-to-buffer-same-window', '*b*')
      expect(marks()).toEqual(['men', 'mem', 'o'])
      await send('unhighlight-regexp', 'me.')
      expect(marks()).toEqual(['o'])
      await send('unhighlight-regexp', true)
      expect(marks()).toEqual([])
    })

    it('face を渡すと, その face の色にする (括弧の番号を渡すと, その括弧だけ)', async () => {
      const editor = await setup()
      const { send } = requests()
      const faces = (face: string) =>
        [...editor.container.querySelectorAll(`.cm-face-${face}`)].map((e) => e.textContent)
      await send('pop-to-buffer-same-window', '*b*')
      await send('insert', '* a\n** b c\nTODO x')
      await send('highlight-regexp', '^\\*+ (.*)', 'org-level-1', 1)
      await send('highlight-regexp', 'TODO', 'org-todo')
      await send('highlight-regexp', 'x(y)?', 'org-done', 1)
      await send('highlight-regexp', 'x', 'org-todo', 0)
      expect(faces('org-level-1')).toEqual(['a', 'b c'])
      expect(faces('org-todo')).toEqual(['TODO', 'x'])
      expect(faces('org-done')).toEqual([])
      expect(await send('highlight-regexp', '^', 'org-done')).toMatchObject({
        error: 'Regexp matches the empty string: ^',
      })
    })

    it('範囲を隠して, ... を出す. 隠した所に掛かる範囲を出すと, 隠すのをやめる', async () => {
      const editor = await setup()
      const { send } = requests()
      await send('pop-to-buffer-same-window', '*b*')
      await send('insert', '* a\nbody\n* b\nmore')
      const folded = () => editor.container.querySelectorAll('.cm-foldPlaceholder').length
      await send('outline-flag-region', 4, 9, true)
      expect(folded()).toBe(1)
      expect(editor.text()).toContain('...')
      expect(await send('outline-invisible-p', 5)).toMatchObject({ value: true })
      expect(await send('outline-invisible-p', 9)).toMatchObject({ value: false })
      // 隠した所を, 付け直す (前に隠した所は除く)
      await send('outline-flag-region', 4, 9, true)
      await send('outline-flag-region', 14, 19, true)
      expect(folded()).toBe(2)
      await send('outline-flag-region', 1, 9, false)
      expect(folded()).toBe(1)
      expect(await send('outline-invisible-p', 5)).toMatchObject({ value: false })
      // 空の範囲は隠さない
      await send('outline-flag-region', 3, 3, true)
      expect(folded()).toBe(1)
      // 隣の隠した所 (重ならない所) は, そのまま
      await send('outline-flag-region', 9, 14, true)
      expect(folded()).toBe(2)
    })

    it('背景, 文字の色と, フォントを変える', async () => {
      await setup()
      const { send } = requests()
      const root = document.documentElement
      await send('set-background-color', 'black')
      expect(root.style.getPropertyValue('--background')).toBe('black')
      expect(root.dataset.backgroundMode).toBe('dark')
      await send('set-foreground-color', 'white')
      expect(root.style.getPropertyValue('--foreground')).toBe('white')
      await send('set-font', 'DejaVu Sans Mono', 12)
      expect(root.style.getPropertyValue('--font-size')).toBe('12pt')
      expect(await send('set-foreground-color', 'nocolor')).toMatchObject({
        error: 'Undefined color: nocolor',
      })
    })
  })

  describe('M-x grep と, プロジェクトのツリー', () => {
    /** 2 つ目のウィンドウのエディタ. */
    const secondEditor = (editor: Awaited<ReturnType<typeof setup>>) =>
      EditorView.findFromDOM(
        editor.container.querySelectorAll('.cm-editor')[1] as HTMLElement,
      ) as EditorView

    /** M-x でコマンドを実行する. */
    const mx = async (editor: Awaited<ReturnType<typeof setup>>, command: string) => {
      editor.key('x', { alt: true })
      await editor.submit(command)
    }

    /** M-x grep で探す. */
    const grep = async (
      editor: Awaited<ReturnType<typeof setup>>,
      pattern: string,
      dir: string,
    ) => {
      await mx(editor, 'grep')
      expect(editor.text()).toContain('Grep (regexp): ')
      await editor.submit(pattern)
      expect(editor.text()).toContain('In directory: ')
      await editor.submit(dir)
    }

    const matches = [
      { file: 'a.txt', line: 3, text: 'foo 1' },
      { file: 'a.txt', line: 5, text: 'foo 2' },
      { file: 'src/b.ts', line: 1, text: 'foo 3' },
    ]

    it('探して, 結果をほかのウィンドウに出し, M-g n と M-g p で, 合った行へ動く', async () => {
      const editor = await setup()
      mocked.grepDirectory.mockResolvedValue(matches)
      mocked.pathExists.mockResolvedValue(true)
      mocked.readFile.mockImplementation(async (path) =>
        path.endsWith('a.txt') ? 'l1\nl2\nl3\nl4\nl5\n' : 'b1\n',
      )
      await grep(editor, 'foo', '/home/u/p')
      expect(mocked.grepDirectory).toHaveBeenCalledWith('foo', '/home/u/p/')
      expect(editor.container.querySelectorAll('.cm-editor')).toHaveLength(2)
      expect(secondEditor(editor).state.doc.toString()).toContain('a.txt:3:foo 1\na.txt:5:foo 2')
      expect(editor.text()).toContain('Grep finished with 3 matches found')
      expect(editor.text()).toContain('(Grep)')
      // 選んでいるウィンドウは, そのまま
      expect(editor.doc()).toContain('This buffer is for text')

      editor.key('g', { alt: true })
      await act(async () => editor.key('n'))
      expect(editor.doc()).toBe('l1\nl2\nl3\nl4\nl5\n')
      expect(editor.cursor()).toBe(6)
      // 同じファイルの, 次の行
      editor.key('g', { alt: true })
      await act(async () => editor.key('n', { alt: true }))
      expect(editor.cursor()).toBe(12)
      await act(async () => {
        editor.key('x', { ctrl: true })
        editor.key('`')
      })
      expect(editor.doc()).toBe('b1\n')
      editor.key('g', { alt: true })
      await act(async () => editor.key('n'))
      expect(editor.text()).toContain('Moved past last grep hit')
      editor.key('g', { alt: true })
      await act(async () => editor.key('p'))
      expect(editor.cursor()).toBe(12)
      editor.key('g', { alt: true })
      await act(async () => editor.key('p', { alt: true }))
      editor.key('g', { alt: true })
      await act(async () => editor.key('p'))
      expect(editor.text()).toContain('Moved back before first grep hit')
    })

    it('結果のバッファの RET で, ほかのウィンドウに開く', async () => {
      const editor = await setup()
      mocked.grepDirectory.mockResolvedValue([matches[0]])
      mocked.pathExists.mockResolvedValue(true)
      mocked.readFile.mockResolvedValue('l1\nl2\nl3\n')
      await grep(editor, 'foo', '/home/u/p/')
      expect(editor.text()).toContain('Grep finished with 1 match found')
      editor.ctrlX('o', false)
      const second = secondEditor(editor)
      // 始めの行 (合った行でない)
      await act(async () => {
        fireEvent.keyDown(second.contentDOM, { key: 'Enter' })
      })
      expect(editor.text()).toContain('No error here')
      act(() => second.dispatch({ selection: { anchor: second.state.doc.line(5).from } }))
      await act(async () => {
        fireEvent.keyDown(second.contentDOM, { key: 'Enter' })
      })
      // 1 つ目のウィンドウに開いて, 選ぶ
      expect(editor.doc()).toBe('l1\nl2\nl3\n')
      expect(editor.cursor()).toBe(6)
      expect(mocked.readFile).toHaveBeenCalledWith('/home/u/p/a.txt')

      // 結果を出しているウィンドウで探し直すと, そこで置き換える
      editor.ctrlX('o', false)
      mocked.grepDirectory.mockResolvedValue([])
      await act(async () => {
        fireEvent.keyDown(second.contentDOM, { key: 'x', altKey: true })
      })
      await editor.submit('grep')
      await editor.submit('zzz')
      await editor.submit('/home/u/p/')
      expect(editor.container.querySelectorAll('.cm-editor')).toHaveLength(2)
      expect(editor.text()).toContain('Grep finished with no matches found')
      expect(secondEditor(editor).state.doc.toString()).toContain('with no matches found')
    })

    it('空の正規表現, 探せないとき, 開けないとき, 結果が無いとき', async () => {
      const editor = await setup()
      await mx(editor, 'grep')
      await editor.submit('')
      expect(mocked.grepDirectory).not.toHaveBeenCalled()

      editor.key('g', { alt: true })
      await act(async () => editor.key('n'))
      expect(editor.text()).toContain('No buffers contain error message locations')
      await mx(editor, 'compile-goto-error')
      expect(editor.text()).toContain('No error here')

      mocked.grepDirectory.mockRejectedValueOnce('Invalid regexp: (')
      await grep(editor, '(', '/home/u/')
      expect(editor.text()).toContain('Error: Invalid regexp: (')

      mocked.grepDirectory.mockResolvedValue([matches[0]])
      await grep(editor, 'foo', '/home/u/')
      mocked.pathExists.mockRejectedValueOnce(new Error('denied'))
      editor.key('g', { alt: true })
      await act(async () => editor.key('n'))
      expect(editor.text()).toContain('Error: Error: denied')
    })

    it('F8 で, プロジェクトのツリーを出し, ファイルを開く', async () => {
      const editor = await setup()
      mocked.projectRoot.mockResolvedValue('/home/u/p')
      mocked.listDirectory.mockImplementation(async (path) => {
        if (path === '/home/u/p/') return ['.git/', 'a.txt', 'src/']
        if (path === '/home/u/p/src/') return ['b.ts']
        throw new Error('denied')
      })
      mocked.pathExists.mockResolvedValue(true)
      mocked.readFile.mockResolvedValue('A')
      await act(async () => editor.key('F8'))
      expect(mocked.projectRoot).toHaveBeenCalledWith('/home/u/')
      const tree = () => editor.container.querySelector('.project-tree') as HTMLElement
      expect(tree().textContent).toContain('P')
      expect(tree().textContent).toContain('a.txt')
      expect(tree().textContent).not.toContain('.git')
      const item = (name: string) =>
        Array.from(tree().querySelectorAll('.project-tree-item')).find((e) =>
          e.textContent?.endsWith(name),
        ) as HTMLElement

      // ディレクトリを開く, 閉じる
      await act(async () => fireEvent.click(item('src')))
      expect(tree().textContent).toContain('b.ts')
      await act(async () => fireEvent.click(item('src')))
      expect(tree().textContent).not.toContain('b.ts')

      // ファイルを開く (開いていれば, そのバッファ)
      await act(async () => fireEvent.click(item('a.txt')))
      expect(editor.doc()).toBe('A')
      expect(item('a.txt').className).toContain('project-tree-current')
      editor.ctrlX('b', false)
      await editor.submit('*scratch*')
      await act(async () => fireEvent.click(item('a.txt')))
      expect(editor.doc()).toBe('A')
      expect(mocked.readFile).toHaveBeenCalledTimes(1)

      // 開けない
      mocked.pathExists.mockRejectedValueOnce(new Error('gone'))
      await act(async () => {
        fireEvent.click(item('src'))
      })
      await act(async () => fireEvent.click(item('b.ts')))
      expect(editor.text()).toContain('Error: Error: gone')

      // 読み直す (読めないディレクトリは, 理由を出す)
      mocked.listDirectory.mockImplementation(async (path) => {
        if (path === '/home/u/p/') return ['a.txt', 'src/']
        throw new Error('denied')
      })
      await act(async () =>
        fireEvent.click(tree().querySelector('.project-tree-refresh') as HTMLElement),
      )
      expect(tree().textContent).toContain('Error: denied')
      // 押してもフォーカスを取らない
      expect(fireEvent.mouseDown(tree())).toBe(false)

      // 消す
      await act(async () => editor.key('F8'))
      expect(editor.container.querySelector('.project-tree')).toBeNull()
    })

    it('根が / のときと, 根が分からないとき', async () => {
      const editor = await setup()
      mocked.projectRoot.mockResolvedValueOnce('/')
      mocked.listDirectory.mockResolvedValue(['etc/'])
      await act(async () => editor.key('F8'))
      expect(mocked.listDirectory).toHaveBeenCalledWith('/')
      const tree = editor.container.querySelector('.project-tree') as HTMLElement
      expect(tree.querySelector('.project-tree-title')?.textContent).toBe('/')
      await act(async () =>
        fireEvent.click(tree.querySelector('.project-tree-item') as HTMLElement),
      )
      expect(mocked.listDirectory).toHaveBeenCalledWith('/etc/')
      await act(async () => editor.key('F8'))

      mocked.projectRoot.mockRejectedValueOnce('no backend')
      await act(async () => editor.key('F8'))
      expect(editor.text()).toContain('Error: no backend')
    })
  })
})
