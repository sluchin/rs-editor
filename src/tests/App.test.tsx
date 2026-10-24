import { describe, it, expect, vi, beforeEach } from 'vitest'
import { render, fireEvent, act } from '@testing-library/react'
import { isolateHistory } from '@codemirror/commands'
import { EditorView } from '@codemirror/view'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import App from '../App'
import * as fileOps from '../lib/fileOps'

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }))
vi.mock('@tauri-apps/api/event', () => ({ listen: vi.fn() }))
const close = vi.fn()
vi.mock('@tauri-apps/api/window', () => ({ getCurrentWindow: () => ({ close }) }))
vi.mock('../lib/fileOps', () => ({
  readFile: vi.fn(),
  writeFile: vi.fn(),
  pathExists: vi.fn(),
  getHomeDir: vi.fn(),
  listDirectory: vi.fn(),
}))

const mocked = vi.mocked(fileOps)
const mockedInvoke = vi.mocked(invoke)
const mockedListen = vi.mocked(listen)
const unlisten = vi.fn()

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
  extra: { commands?: string[]; keys?: { key: string; command: string }[] } = {},
) {
  return { result, error, commands: extra.commands ?? [], keys: extra.keys ?? [] }
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
  /** ミニバッファに入力して, Enter を押す. */
  const submit = async (value: string) => {
    fireEvent.change(input(), { target: { value } })
    await act(async () => {
      fireEvent.keyDown(input(), { key: 'Enter' })
    })
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
  return { ...view, editor, key, ctrlX, input, submit, type, doc, cursor, text }
}

describe('App', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    mocked.getHomeDir.mockResolvedValue('/home/u')
    mocked.writeFile.mockResolvedValue(undefined)
    mockedListen.mockResolvedValue(unlisten)
    // 初期設定ファイル (と, エディタの操作の返事) は, 何もしない
    mockedInvoke.mockResolvedValue(response(''))
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
    fireEvent.keyDown(editor.input(), { key: 'Escape' })

    await act(async () => editor.ctrlX('s'))
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

  it('C-x C-s で, ファイルのあるバッファは上書きし, 失敗はメッセージにする', async () => {
    const editor = await setup()
    mocked.pathExists.mockResolvedValueOnce(true)
    mocked.readFile.mockResolvedValueOnce('a')
    editor.ctrlX('f')
    await editor.submit('/tmp/a.scm')

    await act(async () => editor.ctrlX('s'))
    expect(mocked.writeFile).toHaveBeenCalledWith('/tmp/a.scm', 'a')
    expect(editor.text()).toContain('Wrote /tmp/a.scm')

    mocked.writeFile.mockRejectedValueOnce('disk full')
    await act(async () => editor.ctrlX('s'))
    expect(editor.text()).toContain('Error: disk full')
  })

  it('ファイルのないバッファの C-x C-s は, 保存先を聞く (失敗はメッセージ)', async () => {
    const editor = await setup()

    await act(async () => editor.ctrlX('s'))
    await editor.submit('/tmp/s.scm')
    expect(editor.text()).toContain('Wrote /tmp/s.scm')

    const other = await setup()
    mocked.writeFile.mockRejectedValueOnce('readonly')
    await act(async () => other.ctrlX('s'))
    await other.submit('/x')
    expect(other.text()).toContain('Error: readonly')
  })

  it('C-x b でバッファを切り替え, ない名前はメッセージにする', async () => {
    const editor = await setup()
    mocked.pathExists.mockResolvedValueOnce(false)
    editor.ctrlX('f')
    await editor.submit('/tmp/b.scm')

    editor.ctrlX('b', false)
    await editor.submit('*scratch*')
    expect(editor.text()).toContain('[1/2]')

    editor.ctrlX('b', false)
    await editor.submit('nothing')
    expect(editor.text()).toContain('No buffer named "nothing"')
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

  it('C-x k は, 変更したバッファを消さない', async () => {
    const editor = await setup()
    await editor.type('changed')

    editor.ctrlX('k', false)
    expect(editor.text()).toContain('is modified')
  })

  it('変更したバッファの C-x k のあと, 検索を Enter で確定しても, バッファは消さない', async () => {
    // C-x k の保留が, 次のミニバッファの確定 (検索) まで残る (今の作りの動き)
    const editor = await setup()
    await editor.type('changed')
    editor.ctrlX('k', false)

    editor.key('s', { ctrl: true })
    await act(async () => {
      fireEvent.keyDown(editor.input(), { key: 'Enter' })
    })
    // 変更があるので消さない (警告は, 直後の空のメッセージで消える)
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
    fireEvent.keyDown(editor.input(), { key: 'Escape' })

    editor.key('x', { alt: true })
    await editor.submit('save-buffer')
    expect(editor.text()).toContain('Save file')
    fireEvent.keyDown(editor.input(), { key: 'Escape' })

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
})
