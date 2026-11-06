import { describe, it, expect, vi, beforeEach } from 'vitest'
import { act, fireEvent, render } from '@testing-library/react'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import App from '@/App'
import * as fileOps from '@/lib/fileOps'
import type { BufferFrame, Frame, WindowFrame } from '@/lib/session'

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }))
vi.mock('@tauri-apps/api/event', () => ({ listen: vi.fn() }))
/** アプリの窓を閉じる関数の偽物. */
const close = vi.fn()
/** 窓を閉じるときの頼みを受ける関数の偽物. */
const onCloseRequested = vi.fn()
vi.mock('@tauri-apps/api/window', () => ({
  getCurrentWindow: () => ({ close, onCloseRequested }),
}))
vi.mock('@/lib/fileOps', () => ({
  getHomeDir: vi.fn(),
  listDirectory: vi.fn(),
  projectRoot: vi.fn(),
}))

const mocked = vi.mocked(fileOps)
const mockedInvoke = vi.mocked(invoke)
const mockedListen = vi.mocked(listen)
/** イベントを受けるのをやめる関数の偽物. */
const unlisten = vi.fn()

/**
 * 受けたイベントの関数を返します.
 *
 * @param name - イベントの名前.
 * @returns 関数.
 */
const handlerOf = (name: string) =>
  [...mockedListen.mock.calls].reverse().find(([event]) => event === name)?.[1] as (event: {
    payload: unknown
  }) => void

/**
 * ウィンドウの描く内容を作ります.
 *
 * @param fields - 変える項目.
 * @returns 描く内容.
 */
const win = (fields: Partial<WindowFrame> = {}): WindowFrame => ({
  id: 1,
  left: 0,
  top: 0,
  width: 100,
  height: 100,
  buffer: '*scratch*',
  point: 0,
  mark: null,
  ...fields,
})

/**
 * バッファの描く内容を作ります.
 *
 * @param name - 名前.
 * @param fields - 変える項目.
 * @returns 描く内容.
 */
const buf = (name: string, fields: Partial<BufferFrame> = {}): BufferFrame => ({
  name,
  version: 0,
  text: null,
  changes: [],
  modified: false,
  read_only: false,
  file: null,
  directory: null,
  highlights: [],
  tokens: [],
  mode:
    name === '*scratch*'
      ? { name: 'Lisp Interaction', language: 'scheme', indent_tabs: true, tab_width: 8 }
      : { name: 'Fundamental', language: null, indent_tabs: true, tab_width: 8 },
  ...fields,
})

/**
 * 描く内容を送ります.
 *
 * @param frame - 描く内容 (省いた項目は, 1 つのウィンドウに *scratch*).
 */
const sendFrame = (frame: Partial<Frame> = {}) =>
  act(() =>
    handlerOf('frame')({
      payload: {
        windows: [win()],
        selected: 1,
        buffers: [buf('*scratch*', { text: ';; scratch' })],
        message: '',
        ...frame,
      },
    }),
  )

/** 頼みの番号. */
let nextId = 1

/**
 * エディタの頼みを送り, 返事を待ちます.
 *
 * @param op - 操作.
 * @param args - 引数.
 * @returns 返事 (値と失敗).
 */
async function request(op: string, args: unknown[] = []) {
  const id = nextId++
  act(() => handlerOf('editor-request')({ payload: { id, op, args } }))
  return reply(id)
}

/**
 * 頼みの返事を待ちます.
 *
 * @param id - 頼みの番号.
 * @returns 返事.
 */
async function reply(id: number): Promise<{ value: unknown; error: string | null }> {
  let found: { value: unknown; error: string | null } | undefined
  await vi.waitFor(() => {
    const call = mockedInvoke.mock.calls.find(
      ([cmd, arg]) => cmd === 'editor_reply' && (arg as { id: number }).id === id,
    )
    expect(call).toBeDefined()
    found = call?.[1] as { value: unknown; error: string | null }
  })
  return found as { value: unknown; error: string | null }
}

/**
 * 頼みを送ります (返事を待たない. 問いのとき).
 *
 * @param op - 操作.
 * @param args - 引数.
 * @returns 頼みの番号.
 */
function ask(op: string, args: unknown[]): number {
  const id = nextId++
  act(() => handlerOf('editor-request')({ payload: { id, op, args } }))
  return id
}

/**
 * 送った入力.
 *
 * @returns 入力.
 */
const inputs = () =>
  mockedInvoke.mock.calls
    .filter(([cmd]) => cmd === 'session_input')
    .map(([, arg]) => (arg as { input: unknown }).input)

/**
 * アプリを出して, 描く内容を受けるまで待ちます.
 *
 * @returns 出したもの.
 */
async function setup() {
  const view = render(<App />)
  await vi.waitFor(() => expect(inputs()).toContainEqual({ kind: 'redraw' }))
  sendFrame()
  return view
}

/**
 * ミニバッファの入力欄.
 *
 * @param view - 出したもの.
 * @returns 入力欄.
 */
const minibufferInput = (view: ReturnType<typeof render>) =>
  view.container.querySelector('.minibuffer input') as HTMLInputElement

beforeEach(() => {
  vi.clearAllMocks()
  mockedInvoke.mockResolvedValue(undefined)
  mockedListen.mockResolvedValue(unlisten)
  onCloseRequested.mockResolvedValue(unlisten)
  mocked.getHomeDir.mockResolvedValue('/home/u')
  mocked.listDirectory.mockResolvedValue(['a.txt', 'b/'])
  mocked.projectRoot.mockResolvedValue('/proj/')
  close.mockResolvedValue(undefined)
})

describe('App (描く内容)', () => {
  it('描く内容を受けて, ウィンドウとモードラインとエコー領域を出すこと', async () => {
    const view = await setup()
    expect(view.container.querySelector('.cm-content')?.textContent).toBe(';; scratch')
    expect(view.container.querySelector('.mode-line')?.textContent).toContain('*scratch*')
    expect(view.container.querySelector('.mode-line')?.textContent).toContain('Lisp Interaction')
    // 変更を当てる. 2 つのウィンドウ, 2 つのバッファ, dired, エコー領域
    sendFrame({
      windows: [win({ width: 50, point: 3 }), win({ id: 2, left: 50, width: 50, buffer: 'd' })],
      selected: 2,
      buffers: [
        buf('*scratch*', { changes: [{ from: 0, to: 0, insert: 'x' }], modified: true }),
        buf('d', {
          text: '  /tmp:',
          directory: '/tmp',
          read_only: true,
          mode: { name: 'Dired by name', language: null, indent_tabs: true, tab_width: 8 },
        }),
      ],
      message: 'hello',
    })
    const lines = [...view.container.querySelectorAll('.mode-line')].map((e) => e.textContent)
    expect(lines[0]).toContain('-:**- *scratch* [1/2]')
    expect(lines[1]).toContain('Dired')
    expect(lines[1]).toContain('%%')
    expect(view.container.querySelector('.window-left-of')).not.toBeNull()
    expect(view.container.querySelector('.cm-content')?.textContent).toBe('x;; scratch')
    expect(view.container.querySelector('.minibuffer')?.textContent).toBe('hello')
  })

  it('見えている行の数を知らせ, 変わらなければ知らせないこと', async () => {
    await setup()
    await vi.waitFor(() =>
      expect(inputs()).toContainEqual({ kind: 'resize', heights: [[1, expect.any(Number)]] }),
    )
    const count = inputs().length
    sendFrame({ message: 'again' })
    expect(inputs().length).toBe(count)
  })

  it('色付けは, 使える正規表現だけにし, 既定の face は下線にすること', async () => {
    const view = await setup()
    sendFrame({
      buffers: [
        buf('*scratch*', {
          highlights: [
            { regexp: 'scratch', face: 'hi-yellow', group: 0 },
            { regexp: '(;;) ', face: 'org-todo', group: 1 },
            { regexp: 'x*', face: 'hi-yellow', group: 0 },
            { regexp: '(', face: 'hi-yellow', group: 0 },
          ],
        }),
      ],
    })
    expect(view.container.querySelector('.cm-lisp-highlight')?.textContent).toBe('scratch')
    expect(view.container.querySelector('.cm-face-org-todo')?.textContent).toBe(';;')
  })

  it('核が返した字句を色付けし, 同じ内容なら引き直さないこと', async () => {
    const view = await setup()
    const tokens = [{ from: 0, to: 2, kind: 'comment-delimiter' }]
    sendFrame({ buffers: [buf('*scratch*', { tokens })] })
    expect(view.container.querySelector('.cm-tok-comment-delimiter')?.textContent).toBe(';;')
    // 内容が同じなら (長さも、中身も)、引き直さない
    sendFrame({ buffers: [buf('*scratch*', { tokens: tokens.map((t) => ({ ...t })) })] })
    expect(view.container.querySelector('.cm-tok-comment-delimiter')?.textContent).toBe(';;')
    // 中身が違っても、長さが同じなら、引き直す
    sendFrame({ buffers: [buf('*scratch*', { tokens: [{ from: 3, to: 10, kind: 'string' }] })] })
    expect(view.container.querySelector('.cm-tok-comment-delimiter')).toBeNull()
    expect(view.container.querySelector('.cm-tok-string')?.textContent).toBe('scratch')
    // 長さが違えば、引き直す
    sendFrame({ buffers: [buf('*scratch*', { tokens: [] })] })
    expect(view.container.querySelector('.cm-tok-string')).toBeNull()
  })

  it('ウィンドウの外で押したキーも, エディタに送ること', async () => {
    await setup()
    fireEvent.keyDown(document.body, { key: 'x', ctrlKey: true })
    expect(inputs()).toContainEqual({ kind: 'key', key: 'C-x' })
    // 修飾キーだけと, 入力欄の中のキーは送らない
    const count = inputs().length
    fireEvent.keyDown(document.body, { key: 'Control' })
    const field = document.createElement('input')
    document.body.appendChild(field)
    fireEvent.keyDown(field, { key: 'a' })
    field.remove()
    expect(inputs().length).toBe(count)
  })

  it('受け始める前に閉じたら, 受けるのをやめること', async () => {
    let resolve: (f: () => void) => void = () => {}
    mockedListen.mockReturnValue(new Promise((r) => (resolve = r)))
    mocked.getHomeDir.mockRejectedValue(new Error('no home'))
    const view = render(<App />)
    view.unmount()
    await act(async () => resolve(unlisten))
    expect(unlisten).toHaveBeenCalled()
    expect(inputs()).not.toContainEqual({ kind: 'redraw' })
  })
})

describe('App (問い)', () => {
  it('read-string: 入力を返し, 履歴を M-p で出し, C-g でやめること', async () => {
    const view = await setup()
    let id = ask('read-string', ['Name: ', 'init'])
    expect(view.container.querySelector('.minibuffer-prompt')?.textContent).toBe('Name: ')
    expect(minibufferInput(view).value).toBe('init')
    fireEvent.change(minibufferInput(view), { target: { value: 'typed' } })
    fireEvent.keyDown(minibufferInput(view), { key: 'Enter' })
    expect(await reply(id)).toMatchObject({ value: 'typed', error: null })
    // 履歴
    id = ask('read-string', ['Name: '])
    fireEvent.keyDown(minibufferInput(view), { key: 'p', altKey: true })
    expect(minibufferInput(view).value).toBe('typed')
    fireEvent.keyDown(minibufferInput(view), { key: 'g', ctrlKey: true })
    expect(await reply(id)).toMatchObject({ value: null, error: 'Quit' })
    // M-x は, 別の履歴
    id = ask('read-string', ['M-x '])
    fireEvent.keyDown(minibufferInput(view), { key: 'p', altKey: true })
    expect(minibufferInput(view).value).toBe('')
    fireEvent.keyDown(minibufferInput(view), { key: 'Escape' })
    await reply(id)
  })

  it('read-string: M-n で核が渡した既定の値を出すこと', async () => {
    const view = await setup()
    const id = ask('read-string', ['Name: ', '', 'minibuffer', ['dflt', 1]])
    fireEvent.keyDown(minibufferInput(view), { key: 'n', altKey: true })
    expect(minibufferInput(view).value).toBe('dflt')
    fireEvent.keyDown(minibufferInput(view), { key: 'Enter' })
    expect(await reply(id)).toMatchObject({ value: 'dflt', error: null })
  })

  it('read-string: 核が渡した履歴を, 始めの位置からたどり, 加えないこと', async () => {
    const view = await setup()
    let id = ask('read-string', ['I-search: ', 'foo', ['isearch-ring', 1], [], ['foo', 'bar'], 1])
    expect(minibufferInput(view).value).toBe('foo')
    fireEvent.keyDown(minibufferInput(view), { key: 'p', altKey: true })
    expect(minibufferInput(view).value).toBe('bar')
    fireEvent.keyDown(minibufferInput(view), { key: 'n', altKey: true })
    fireEvent.keyDown(minibufferInput(view), { key: 'n', altKey: true })
    expect(minibufferInput(view).value).toBe('foo')
    fireEvent.keyDown(minibufferInput(view), { key: 'Enter' })
    expect(await reply(id)).toMatchObject({ value: 'foo', error: null })
    // 名前の履歴には加えない. 始めの位置は, 履歴の数までにする
    id = ask('read-string', ['Name: ', '', 'minibuffer', [], null, 5])
    fireEvent.keyDown(minibufferInput(view), { key: 'p', altKey: true })
    expect(minibufferInput(view).value).toBe('')
    fireEvent.keyDown(minibufferInput(view), { key: 'Enter' })
    await reply(id)
  })

  it('read-file-name: 既定のディレクトリから始め, ~ を展開して補完すること', async () => {
    const view = await setup()
    await vi.waitFor(() => expect(mocked.getHomeDir).toHaveBeenCalled())
    let id = ask('read-file-name', ['Find file: ', ''])
    expect(minibufferInput(view).value).toBe('/home/u/')
    fireEvent.change(minibufferInput(view), { target: { value: '~/a' } })
    await act(async () => {
      fireEvent.keyDown(minibufferInput(view), { key: 'Tab' })
    })
    expect(mocked.listDirectory).toHaveBeenCalledWith('/home/u/')
    expect(minibufferInput(view).value).toBe('~/a.txt')
    fireEvent.keyDown(minibufferInput(view), { key: 'Enter' })
    expect((await reply(id)).value).toBe('~/a.txt')
    // ファイルのバッファは, そのディレクトリ. dired はそのディレクトリ. 渡した始めの入力は, そのまま
    sendFrame({ buffers: [buf('*scratch*', { file: '/w/x.scm' })] })
    id = ask('read-file-name', ['Find file: '])
    expect(minibufferInput(view).value).toBe('/w/')
    fireEvent.keyDown(minibufferInput(view), { key: 'Escape' })
    await reply(id)
    sendFrame({ buffers: [buf('*scratch*', { directory: '/d' })] })
    id = ask('read-file-name', ['Find file: ', ''])
    expect(minibufferInput(view).value).toBe('/d/')
    fireEvent.keyDown(minibufferInput(view), { key: 'Escape' })
    await reply(id)
    sendFrame({ buffers: [buf('*scratch*', { directory: '/d/' })] })
    id = ask('read-file-name', ['Find file: ', '/given/'])
    expect(minibufferInput(view).value).toBe('/given/')
    fireEvent.change(minibufferInput(view), { target: { value: '/abs/' } })
    await act(async () => {
      fireEvent.keyDown(minibufferInput(view), { key: 'Tab' })
    })
    expect(mocked.listDirectory).toHaveBeenLastCalledWith('/abs/')
    fireEvent.keyDown(minibufferInput(view), { key: 'Escape' })
    await reply(id)
  })

  it('ホームディレクトリが / で終わっていれば, そのまま使うこと', async () => {
    mocked.getHomeDir.mockResolvedValue('/')
    const view = await setup()
    await vi.waitFor(() => expect(mocked.getHomeDir).toHaveBeenCalled())
    const id = ask('read-file-name', ['Find file: ', ''])
    expect(minibufferInput(view).value).toBe('/')
    fireEvent.keyDown(minibufferInput(view), { key: 'Escape' })
    await reply(id)
  })

  it('completing-read と read-key に答えること', async () => {
    const view = await setup()
    let id = ask('completing-read', ['Switch to buffer: ', ['*scratch*', 'other'], ''])
    fireEvent.change(minibufferInput(view), { target: { value: 'ot' } })
    fireEvent.keyDown(minibufferInput(view), { key: 'Tab' })
    fireEvent.keyDown(minibufferInput(view), { key: 'Enter' })
    expect((await reply(id)).value).toBe('other')
    id = ask('completing-read', ['M-x ', ['forward-char']])
    fireEvent.keyDown(minibufferInput(view), { key: 'Enter' })
    expect((await reply(id)).value).toBe('')
    id = ask('read-key', ['Key: '])
    expect(view.container.querySelector('.minibuffer-prompt')?.textContent).toBe('Key: ')
    fireEvent.keyDown(minibufferInput(view), { key: 'g', ctrlKey: true })
    expect((await reply(id)).value).toBe('C-g')
    id = ask('read-key', [])
    fireEvent.keyDown(minibufferInput(view), { key: 'y' })
    expect((await reply(id)).value).toBe('y')
  })
})

describe('App (画面だけの操作)', () => {
  it('スクロール, 隠すこと, 一番上の行に答えること', async () => {
    await setup()
    expect((await request('window-top-line', [1])).value).toBe(0)
    expect((await request('window-top-line', [9])).value).toBe(0)
    expect((await request('recenter')).error).toBeNull()
    // 位置は 1 から, 文字で数える
    expect((await request('outline-flag-region', [3, 6, true])).error).toBeNull()
    expect((await request('outline-invisible-p', [4])).value).toBe(true)
    expect((await request('outline-flag-region', [6, 3, false])).error).toBeNull()
    expect((await request('outline-invisible-p', [4])).value).toBe(false)
    expect((await request('outline-invisible-p', [99])).value).toBe(false)
    expect((await request('outline-invisible-p', [0])).value).toBe(false)
    // ウィンドウが無ければ, 失敗
    sendFrame({ windows: [] })
    expect((await request('recenter')).error).toBe('No window is selected')
  })

  it('色とフォントを変え, 読めなければ失敗すること', async () => {
    await setup()
    const root = document.documentElement
    expect((await request('set-background-color', ['black'])).error).toBeNull()
    expect(root.style.getPropertyValue('--background')).toBe('black')
    expect((await request('set-foreground-color', ['white'])).error).toBeNull()
    expect(root.style.getPropertyValue('--foreground')).toBe('white')
    expect((await request('set-font', ['Mono', 12])).error).toBeNull()
    expect(root.style.getPropertyValue('--font-size')).toBe('12pt')
    expect((await request('set-font', [null, null])).error).toBeNull()
    expect((await request('set-background-color', ['nocolor'])).error).toContain('Undefined')
    expect((await request('no-such-op')).error).toBe('Unknown editor operation: no-such-op')
  })

  it('クリップボードに書き, 読み, 使えなければ nil を返すこと', async () => {
    await setup()
    const clipboard = {
      writeText: vi.fn().mockResolvedValue(undefined),
      readText: vi.fn().mockResolvedValue('pasted'),
    }
    Object.defineProperty(navigator, 'clipboard', { value: clipboard, configurable: true })
    expect(await request('gui-set-selection', ['copied'])).toMatchObject({
      value: null,
      error: null,
    })
    expect(clipboard.writeText).toHaveBeenCalledWith('copied')
    expect(await request('gui-selection-value')).toMatchObject({ value: 'pasted', error: null })
    clipboard.writeText.mockRejectedValue(new Error('denied'))
    clipboard.readText.mockRejectedValue(new Error('denied'))
    expect(await request('gui-set-selection', ['x'])).toMatchObject({ value: null, error: null })
    expect(await request('gui-selection-value')).toMatchObject({ value: null, error: null })
    Object.defineProperty(navigator, 'clipboard', { value: undefined, configurable: true })
  })

  it('サイドバーのツリーを出し, ファイルを選ぶと開き, もう一度で消すこと', async () => {
    mocked.listDirectory.mockResolvedValue(['a.txt'])
    const view = await setup()
    expect((await request('toggle-project-tree')).error).toBeNull()
    expect(mocked.projectRoot).toHaveBeenCalledWith('/home/u/')
    const file = await vi.waitFor(() => {
      const found = [...view.container.querySelectorAll('.project-tree-item')].find(
        (e) => e.textContent === 'a.txt',
      )
      expect(found).toBeDefined()
      return found as HTMLElement
    })
    fireEvent.click(file)
    expect(inputs()).toContainEqual({ kind: 'find-file', path: '/proj/a.txt' })
    await request('toggle-project-tree')
    expect(view.container.querySelector('.project-tree')).toBeNull()
    // 根が / なら, そのまま
    mocked.projectRoot.mockResolvedValue('/')
    await request('toggle-project-tree')
    expect(view.container.querySelector('.project-tree-title')?.textContent).toBe('/')
  })

  it('窓を閉じるときはエディタに聞き, kill-emacs で閉じること', async () => {
    await setup()
    const onClose = onCloseRequested.mock.calls[0][0] as (event: {
      preventDefault: () => void
    }) => void
    const event = { preventDefault: vi.fn() }
    onClose(event)
    expect(event.preventDefault).toHaveBeenCalled()
    expect(inputs()).toContainEqual({ kind: 'command', name: 'save-buffers-kill-terminal' })
    expect((await request('kill-emacs')).error).toBeNull()
    expect(close).toHaveBeenCalled()
    // 終わってよいと答えた後は, そのまま閉じる
    const after = { preventDefault: vi.fn() }
    onClose(after)
    expect(after.preventDefault).not.toHaveBeenCalled()
  })
})
