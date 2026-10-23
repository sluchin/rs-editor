import { describe, it, expect, vi, beforeEach } from 'vitest'
import { render, fireEvent, act } from '@testing-library/react'
import { isolateHistory } from '@codemirror/commands'
import { EditorView } from '@codemirror/view'
import { invoke } from '@tauri-apps/api/core'
import App from '../App'
import * as fileOps from '../lib/fileOps'

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }))
const close = vi.fn()
vi.mock('@tauri-apps/api/window', () => ({ getCurrentWindow: () => ({ close }) }))
vi.mock('../lib/fileOps', () => ({
  readFile: vi.fn(),
  writeFile: vi.fn(),
  pathExists: vi.fn(),
  getHomeDir: vi.fn(),
}))

const mocked = vi.mocked(fileOps)
const mockedInvoke = vi.mocked(invoke)

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

    mockedInvoke.mockResolvedValueOnce({ result: '3', error: null })
    await act(async () => editor.ctrlX('e'))
    expect(mockedInvoke).toHaveBeenCalledWith('eval_scheme', { code: '(+ 1 2)' })
    expect(editor.text()).toContain('3')

    mockedInvoke.mockResolvedValueOnce({ result: '', error: 'bad' })
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

    mockedInvoke.mockResolvedValueOnce({ result: '42', error: null })
    editor.key(':', { alt: true })
    await editor.submit('(* 6 7)')
    expect(editor.text()).toContain('42')

    mockedInvoke.mockResolvedValueOnce({ result: '', error: 'oops' })
    editor.key(':', { alt: true })
    await editor.submit('(car 1)')
    expect(editor.text()).toContain('Error: oops')

    mockedInvoke.mockRejectedValueOnce('down')
    editor.key(':', { alt: true })
    await editor.submit('x')
    expect(editor.text()).toContain('Eval error: down')
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
