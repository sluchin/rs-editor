import { describe, it, expect, vi } from 'vitest'
import { createRef } from 'react'
import { act, render, screen, fireEvent } from '@testing-library/react'
import { EditorView } from '@codemirror/view'
import { language } from '@codemirror/language'
import CodeEditor, { CursorInfo, EditorHandle } from '@/components/CodeEditor'
import Minibuffer, { MinibufferInputState, MinibufferState } from '@/components/Minibuffer'
import ModeLine from '@/components/ModeLine'
import REPL from '@/components/REPL'

/**
 * CodeEditor を表示します.
 *
 * @param props - 変える引数.
 * @returns 表示と, エディタ (CodeMirror) と, 呼ばれた関数.
 */
function renderEditor(props: Partial<Parameters<typeof CodeEditor>[0]> = {}) {
  const handlers = {
    onChange: vi.fn(),
    onCursorChange: vi.fn(),
    onMessage: vi.fn(),
    actions: { 'find-file': vi.fn() },
  }
  const handle = createRef<EditorHandle>()
  const element = (extra: Partial<Parameters<typeof CodeEditor>[0]> = {}) => (
    <CodeEditor
      bufferId="a"
      initialContent="(+ 1 2)"
      keyBindings={[]}
      disabled={false}
      handle={handle}
      {...handlers}
      {...props}
      {...extra}
    />
  )
  const view = render(element())
  const editor = EditorView.findFromDOM(
    view.container.querySelector('.cm-editor') as HTMLElement,
  ) as EditorView
  /** キーを押します. */
  const key = (k: string, mods: { ctrl?: boolean; alt?: boolean } = {}) =>
    fireEvent.keyDown(editor.contentDOM, {
      key: k,
      ctrlKey: mods.ctrl ?? false,
      altKey: mods.alt ?? false,
    })
  return { ...view, editor, handle, key, element, ...handlers }
}

describe('CodeEditor Component', () => {
  it('前置引数で, エディタのコマンドは繰り返し, 画面のコマンドは 1 回だけ動かすこと', () => {
    const { editor, key, actions, onMessage } = renderEditor()
    key('u', { ctrl: true })
    expect(onMessage).toHaveBeenCalledWith('C-u 4-')
    key('f', { ctrl: true })
    expect(editor.state.selection.main.head).toBe(4)
    key('u', { ctrl: true })
    key('x', { ctrl: true })
    key('f', { ctrl: true })
    expect(actions['find-file']).toHaveBeenCalledTimes(1)
  })

  it('別のウィンドウの書き換えを写し, 元に戻す履歴には入れないこと', () => {
    const { editor, handle, onChange } = renderEditor()
    act(() => handle.current?.sync('(+ 1 2 3)'))
    expect(editor.state.doc.toString()).toBe('(+ 1 2 3)')
    act(() => handle.current?.sync('(+ 1 2 3)'))
    act(() => handle.current?.sync('x'))
    expect(editor.state.doc.toString()).toBe('x')
    expect(onChange).not.toHaveBeenCalled()
    expect(handle.current?.run('undo')).toBe(true)
    expect(editor.state.doc.toString()).toBe('x')
  })

  it('言語を渡すと色を付け, 言語やバッファが変わると付け直すこと', () => {
    const { editor, rerender, element } = renderEditor({ language: 'scheme' })
    expect(editor.state.facet(language)?.name).toBe('scheme')
    rerender(element({ language: null }))
    expect(editor.state.facet(language)).toBeNull()
    // 色を付けないバッファから, 色を付けるバッファに切り替える
    rerender(element({ bufferId: 'b', language: 'scheme' }))
    expect(editor.state.facet(language)?.name).toBe('scheme')
    rerender(element({ bufferId: 'b', language: 'elisp' }))
    expect(editor.state.facet(language)?.name).toBe('elisp')
  })

  it('選んでいないウィンドウのエディタは, フォーカスを取らず, 選ぶと取ること', () => {
    const { editor, rerender, element } = renderEditor({ active: false })
    expect(editor.hasFocus).toBe(false)
    rerender(element({ active: false, disabled: true }))
    rerender(element({ active: false, disabled: false }))
    expect(editor.hasFocus).toBe(false)
    rerender(element({ active: true, disabled: true }))
    expect(editor.hasFocus).toBe(false)
    rerender(element({ active: true, disabled: false }))
    expect(editor.hasFocus).toBe(true)
  })

  it('Lisp から使う操作 (本文, 位置, 入れる, 消す) をすること', () => {
    const { editor, handle } = renderEditor()
    act(() => handle.current?.setCursor(3))
    expect(handle.current?.point()).toBe(3)
    act(() => handle.current?.insert('xy'))
    expect(handle.current?.text()).toBe('(+ xy1 2)')
    expect(editor.state.selection.main.head).toBe(5)
    act(() => handle.current?.deleteRegion(0, 3))
    expect(handle.current?.text()).toBe('xy1 2)')
  })

  it('Lisp で割り当てたキーを, 既定のキーより先に使い, バッファを切り替えても使うこと', () => {
    const run = vi.fn()
    const { editor, key, element, rerender } = renderEditor({ actions: { mine: run } })
    // 画面のコマンドは, 同じ名前のエディタのコマンドより先に使う
    const ownEnd = vi.fn()
    rerender(element({ actions: { 'end-of-buffer': ownEnd } }))
    key('>', { alt: true })
    expect(ownEnd).toHaveBeenCalled()
    expect(editor.state.selection.main.head).toBe(0)

    const keys = [{ key: 'Ctrl-e', command: 'mine' }]
    rerender(element({ actions: { mine: run }, keyBindings: keys }))
    key('e', { ctrl: true })
    expect(run).toHaveBeenCalledTimes(1)
    // 同じ割り当てなら, 作り直さない
    rerender(element({ actions: { mine: run }, keyBindings: keys }))
    rerender(element({ actions: { mine: run }, keyBindings: keys, bufferId: 'b' }))
    key('e', { ctrl: true })
    expect(run).toHaveBeenCalledTimes(2)
  })

  it('始めの本文を表示し, カーソルの位置を知らせること', () => {
    const { editor, onCursorChange } = renderEditor()
    expect(editor.state.doc.toString()).toBe('(+ 1 2)')
    expect(onCursorChange).toHaveBeenCalledWith({ line: 1, column: 1, offset: 0 })
  })

  it('書き換えと, カーソルの移動を知らせること', () => {
    const { editor, onChange, onCursorChange } = renderEditor()
    editor.dispatch({ changes: { from: 0, insert: 'a\n' }, selection: { anchor: 3 } })
    expect(onChange).toHaveBeenCalledWith('a\n(+ 1 2)')
    expect(onCursorChange).toHaveBeenLastCalledWith({ line: 2, column: 2, offset: 3 })
    editor.dispatch({ selection: { anchor: 0 } })
    expect(onCursorChange).toHaveBeenLastCalledWith({ line: 1, column: 1, offset: 0 })
  })

  it('Emacs のキーでエディタを動かし, 画面のコマンドも呼ぶこと', () => {
    const { editor, key, actions } = renderEditor()
    key('e', { ctrl: true })
    expect(editor.state.selection.main.head).toBe(7)
    key('x', { ctrl: true })
    key('f', { ctrl: true })
    expect(actions['find-file']).toHaveBeenCalled()
  })

  it('コマンドを名前で実行し, 無いコマンドは false を返すこと', () => {
    const { editor, handle, actions } = renderEditor()
    expect(handle.current?.run('end-of-buffer')).toBe(true)
    expect(editor.state.selection.main.head).toBe(7)
    expect(handle.current?.run('find-file')).toBe(true)
    expect(actions['find-file']).toHaveBeenCalled()
    expect(handle.current?.run('no-such-command')).toBe(false)
  })

  it('範囲を選び, カーソルを置き, 行へ動くこと', () => {
    const { editor, handle } = renderEditor({ initialContent: 'ab\ncd' })
    act(() => handle.current?.select(1, 2))
    expect(editor.state.selection.main).toMatchObject({ from: 1, to: 2 })
    act(() => handle.current?.setCursor(4))
    expect(editor.state.selection.main.head).toBe(4)
    act(() => handle.current?.gotoLine(1))
    expect(editor.state.selection.main.head).toBe(0)
  })

  it('コマンドの知らせを, ミニバッファへ渡すこと', () => {
    const { key, onMessage } = renderEditor()
    key('g', { ctrl: true })
    expect(onMessage).toHaveBeenCalledWith('Quit')
  })

  it('書き換えさせないあいだは読み取り専用にし, 終わったら戻すこと', () => {
    const { editor, rerender, element } = renderEditor({ disabled: true })
    expect(editor.state.readOnly).toBe(true)
    rerender(element({ disabled: false }))
    expect(editor.state.readOnly).toBe(false)
  })

  it('バッファを切り替えると, バッファごとの本文とカーソルに切り替え, 消したバッファは作り直すこと', () => {
    const { editor, rerender, element, handle, onChange } = renderEditor()
    editor.dispatch({ selection: { anchor: 3 } })
    rerender(element({ bufferId: 'b', initialContent: 'other' }))
    expect(editor.state.doc.toString()).toBe('other')
    rerender(element({ bufferId: 'a', initialContent: '(+ 1 2)' }))
    expect(editor.state.doc.toString()).toBe('(+ 1 2)')
    expect(editor.state.selection.main.head).toBe(3)
    // 隠していたあいだに (別のウィンドウで) 書き換えた本文は, 写す (書き換えとしては知らせない)
    rerender(element({ bufferId: 'b', initialContent: 'other' }))
    onChange.mockClear()
    rerender(element({ bufferId: 'a', initialContent: '(+ 10 2) ;' }))
    expect(editor.state.doc.toString()).toBe('(+ 10 2) ;')
    expect(editor.state.selection.main.head).toBe(3)
    expect(onChange).not.toHaveBeenCalled()
    // 同じバッファのまま描き直しても, 状態を変えない
    rerender(element({ bufferId: 'a', initialContent: 'ignored' }))
    expect(editor.state.doc.toString()).toBe('(+ 10 2) ;')
    act(() => handle.current?.forget('b'))
    rerender(element({ bufferId: 'b', initialContent: 'new' }))
    expect(editor.state.doc.toString()).toBe('new')
  })
})

/**
 * Minibuffer コンポーネント のテストスイート.
 * メッセージ表示と入力プロンプトをテストします.
 */
describe('Minibuffer Component (補完)', () => {
  it('Tab で補完し, 候補が複数なら一覧を, 無ければ [No match] を出すこと', () => {
    let state: MinibufferState = {
      mode: 'input',
      prompt: 'M-x ',
      input: 'kill',
      completions: ['kill-line', 'kill-word', 'yank'],
    }
    const onInputChange = vi.fn((value: string) => {
      state = { ...(state as MinibufferInputState), input: value }
    })
    const props = { onInputChange, onSubmit: vi.fn(), onCancel: vi.fn() }
    const view = render(<Minibuffer state={state} {...props} />)
    const input = () => view.container.querySelector('input') as HTMLInputElement
    const tab = () => {
      fireEvent.keyDown(input(), { key: 'Tab' })
      view.rerender(<Minibuffer state={state} {...props} />)
    }
    tab()
    expect(onInputChange).toHaveBeenLastCalledWith('kill-', expect.anything())
    expect(view.container.textContent).toContain('Possible completions:')
    expect(view.container.textContent).toContain('kill-word')
    // 入力すると, 一覧を消す
    fireEvent.change(input(), { target: { value: 'kill-l' } })
    view.rerender(<Minibuffer state={state} {...props} />)
    expect(view.container.textContent).not.toContain('Possible completions:')
    tab()
    expect(onInputChange).toHaveBeenLastCalledWith('kill-line', expect.anything())
    expect(view.container.textContent).not.toContain('Possible completions:')
    state = { ...(state as MinibufferInputState), input: 'zz' }
    view.rerender(<Minibuffer state={state} {...props} />)
    tab()
    expect(view.container.textContent).toContain('[No match]')
    // 別の入力に移ると, 出さない
    state = { mode: 'input', prompt: 'Find file: ', input: '' }
    view.rerender(<Minibuffer state={state} {...props} />)
    expect(view.container.textContent).not.toContain('[No match]')
    // 候補がなければ, Tab で補完しない
    fireEvent.keyDown(input(), { key: 'Tab' })
    expect(onInputChange).toHaveBeenCalledTimes(4)
  })
})

describe('Minibuffer Component (ファイルの名前の補完)', () => {
  it('Tab で, 補完する関数の結果を待って補完し, 待つあいだに入力が変われば捨てること', async () => {
    let state: MinibufferState = {
      mode: 'input',
      prompt: 'Find file: ',
      input: '/home/u/s',
      complete: async (input) => ({ value: input + 'rc/', matches: ['src/'] }),
    }
    const onInputChange = vi.fn((value: string) => {
      state = { ...(state as MinibufferInputState), input: value }
    })
    const props = { onInputChange, onSubmit: vi.fn(), onCancel: vi.fn() }
    const view = render(<Minibuffer state={state} {...props} />)
    const input = () => view.container.querySelector('input') as HTMLInputElement
    await act(async () => {
      fireEvent.keyDown(input(), { key: 'Tab' })
    })
    expect(onInputChange).toHaveBeenLastCalledWith('/home/u/src/', expect.anything())
    view.rerender(<Minibuffer state={state} {...props} />)
    // 補完を待つあいだに, ミニバッファを閉じた
    let finish: (value: { value: string; matches: string[] }) => void = () => {}
    state = {
      ...(state as MinibufferInputState),
      complete: () => new Promise((resolve) => (finish = resolve)),
    }
    view.rerender(<Minibuffer state={state} {...props} />)
    fireEvent.keyDown(input(), { key: 'Tab' })
    view.rerender(<Minibuffer state={{ mode: 'message', text: '' }} {...props} />)
    await act(async () => finish({ value: '/x', matches: [] }))
    expect(onInputChange).toHaveBeenCalledTimes(1)
  })
})

describe('Minibuffer Component (問い)', () => {
  it('問いでは, 押したキーを知らせ, Esc と C-g で取り消すこと', () => {
    const onQueryKey = vi.fn()
    const onCancel = vi.fn()
    const props = {
      state: { mode: 'query', prompt: 'Query replacing a with b: ' } as MinibufferState,
      onInputChange: vi.fn(),
      onSubmit: vi.fn(),
      onCancel,
    }
    const view = render(<Minibuffer {...props} onQueryKey={onQueryKey} />)
    expect(view.container.textContent).toContain('Query replacing a with b: ')
    const input = view.container.querySelector('input') as HTMLInputElement
    fireEvent.keyDown(input, { key: 'y' })
    expect(onQueryKey).toHaveBeenCalledWith('y')
    fireEvent.keyDown(input, { key: 'g', ctrlKey: true })
    fireEvent.keyDown(input, { key: 'Escape' })
    expect(onCancel).toHaveBeenCalledTimes(2)
    // 知らせる先がなくても, 落ちない
    view.rerender(<Minibuffer {...props} />)
    fireEvent.keyDown(view.container.querySelector('input') as HTMLInputElement, { key: 'n' })
    expect(onQueryKey).toHaveBeenCalledTimes(1)
  })
})

describe('Minibuffer Component', () => {
  it('displays message when in message mode', () => {
    // メッセージモードでメッセージが表示されることを確認.
    const state: MinibufferState = { mode: 'message', text: 'Test message' }
    render(
      <Minibuffer state={state} onInputChange={vi.fn()} onSubmit={vi.fn()} onCancel={vi.fn()} />,
    )

    expect(screen.getByText('Test message')).toBeTruthy()
  })

  it('displays default message when text is empty', () => {
    // テキストが空のときデフォルトメッセージが表示されることを確認.
    const state: MinibufferState = { mode: 'message', text: '' }
    render(
      <Minibuffer state={state} onInputChange={vi.fn()} onSubmit={vi.fn()} onCancel={vi.fn()} />,
    )

    expect(screen.getByText('For information about this editor, see the README.')).toBeTruthy()
  })

  it('displays input field when in input mode', () => {
    // 入力モードで入力フィールドが表示されることを確認.
    const state: MinibufferState = { mode: 'input', prompt: 'Open file: ', input: '' }
    const { container } = render(
      <Minibuffer state={state} onInputChange={vi.fn()} onSubmit={vi.fn()} onCancel={vi.fn()} />,
    )

    expect(container.querySelector('input')).toBeTruthy()
    const prompt = container.querySelector('.minibuffer-prompt')
    expect(prompt?.textContent).toBe('Open file: ')
  })

  it('calls onInputChange when input changes', () => {
    // 入力値が変更されると onInputChange が呼ばれることを確認.
    const onInputChange = vi.fn()
    const state: MinibufferState = { mode: 'input', prompt: 'Open file: ', input: '' }
    const { container } = render(
      <Minibuffer
        state={state}
        onInputChange={onInputChange}
        onSubmit={vi.fn()}
        onCancel={vi.fn()}
      />,
    )

    const input = container.querySelector('input') as HTMLInputElement
    fireEvent.change(input, { target: { value: 'test.scm' } })

    expect(onInputChange).toHaveBeenCalledWith('test.scm', state)
  })

  it('calls onSubmit on Enter key', () => {
    // Enter キーが押されると onSubmit が呼ばれることを確認.
    const onSubmit = vi.fn()
    const state: MinibufferState = { mode: 'input', prompt: 'Open file: ', input: 'test.scm' }
    const { container } = render(
      <Minibuffer state={state} onInputChange={vi.fn()} onSubmit={onSubmit} onCancel={vi.fn()} />,
    )

    const input = container.querySelector('input') as HTMLInputElement
    fireEvent.keyDown(input, { key: 'Enter' })

    expect(onSubmit).toHaveBeenCalledWith('test.scm')
  })

  it('calls onCancel on Escape key', () => {
    // Escape キーが押されると onCancel が呼ばれることを確認.
    const onCancel = vi.fn()
    const state: MinibufferState = { mode: 'input', prompt: 'Open file: ', input: 'test.scm' }
    const { container } = render(
      <Minibuffer state={state} onInputChange={vi.fn()} onSubmit={vi.fn()} onCancel={onCancel} />,
    )

    const input = container.querySelector('input') as HTMLInputElement
    fireEvent.keyDown(input, { key: 'Escape' })

    expect(onCancel).toHaveBeenCalled()
  })

  it('calls onCancel on C-g', () => {
    // C-g が押されると onCancel が呼ばれることを確認.
    const onCancel = vi.fn()
    const state: MinibufferState = { mode: 'input', prompt: 'Open file: ', input: 'test.scm' }
    const { container } = render(
      <Minibuffer state={state} onInputChange={vi.fn()} onSubmit={vi.fn()} onCancel={onCancel} />,
    )

    const input = container.querySelector('input') as HTMLInputElement
    fireEvent.keyDown(input, { key: 'g', ctrlKey: true })

    expect(onCancel).toHaveBeenCalled()
  })

  it('renders isearch mode with forward prompt', () => {
    // isearch フォワードモードでプロンプトが表示されることを確認.
    const state: MinibufferState = {
      mode: 'isearch',
      regexp: false,
      direction: 'forward',
      query: 'test',
      originalCursor: 0,
      matchStart: 5,
      matchEnd: 9,
    }
    const { container } = render(
      <Minibuffer state={state} onInputChange={vi.fn()} onSubmit={vi.fn()} onCancel={vi.fn()} />,
    )

    const prompt = container.querySelector('.minibuffer-prompt')
    expect(prompt?.textContent).toBe('I-search: ')
    const input = container.querySelector('input') as HTMLInputElement
    expect(input.value).toBe('test')
  })

  it('renders isearch mode with backward prompt', () => {
    // isearch バックワードモードでプロンプトが表示されることを確認.
    const state: MinibufferState = {
      mode: 'isearch',
      regexp: false,
      direction: 'backward',
      query: 'test',
      originalCursor: 20,
      matchStart: 15,
      matchEnd: 19,
    }
    const { container } = render(
      <Minibuffer state={state} onInputChange={vi.fn()} onSubmit={vi.fn()} onCancel={vi.fn()} />,
    )

    const prompt = container.querySelector('.minibuffer-prompt')
    expect(prompt?.textContent).toBe('I-search backward: ')
  })

  it('displays Failing prefix when isearch has no match', () => {
    // isearch でマッチが無いときに Failing プレフィックスが表示されることを確認.
    const state: MinibufferState = {
      mode: 'isearch',
      regexp: false,
      direction: 'forward',
      query: 'xyz',
      originalCursor: 0,
      matchStart: null,
      matchEnd: null,
    }
    const { container } = render(
      <Minibuffer state={state} onInputChange={vi.fn()} onSubmit={vi.fn()} onCancel={vi.fn()} />,
    )

    const prompt = container.querySelector('.minibuffer-prompt')
    expect(prompt?.textContent).toBe('Failing I-search: ')
  })

  it('calls onIsearchRepeat with forward direction on C-s during isearch', () => {
    // isearch 中に C-s が押されると onIsearchRepeat が呼ばれることを確認.
    const onIsearchRepeat = vi.fn()
    const state: MinibufferState = {
      mode: 'isearch',
      regexp: false,
      direction: 'forward',
      query: 'test',
      originalCursor: 0,
      matchStart: 5,
      matchEnd: 9,
    }
    const { container } = render(
      <Minibuffer
        state={state}
        onInputChange={vi.fn()}
        onSubmit={vi.fn()}
        onCancel={vi.fn()}
        onIsearchRepeat={onIsearchRepeat}
      />,
    )

    const input = container.querySelector('input') as HTMLInputElement
    fireEvent.keyDown(input, { key: 's', ctrlKey: true })

    expect(onIsearchRepeat).toHaveBeenCalledWith('forward', state)
  })

  it('calls onIsearchRepeat with backward direction on C-r during isearch', () => {
    // isearch 中に C-r が押されると onIsearchRepeat が呼ばれることを確認.
    const onIsearchRepeat = vi.fn()
    const state: MinibufferState = {
      mode: 'isearch',
      regexp: false,
      direction: 'forward',
      query: 'test',
      originalCursor: 20,
      matchStart: 5,
      matchEnd: 9,
    }
    const { container } = render(
      <Minibuffer
        state={state}
        onInputChange={vi.fn()}
        onSubmit={vi.fn()}
        onCancel={vi.fn()}
        onIsearchRepeat={onIsearchRepeat}
      />,
    )

    const input = container.querySelector('input') as HTMLInputElement
    fireEvent.keyDown(input, { key: 'r', ctrlKey: true })

    expect(onIsearchRepeat).toHaveBeenCalledWith('backward', state)
  })
})

/**
 * ModeLine コンポーネント のテストスイート.
 * 行番号とファイル名を表示するモードラインをテストします.
 */
describe('ModeLine Component', () => {
  it('renders with cursor info', () => {
    // カーソル情報を含めてレンダリングされることを確認.
    const cursorInfo: CursorInfo = { line: 5, column: 10, offset: 42 }
    const { container } = render(
      <ModeLine bufferName="test.scm" modified={false} cursor={cursorInfo} />,
    )

    expect(container.textContent).toContain('L5')
  })

  it('displays filename when provided', () => {
    // ファイル名が表示されることを確認.
    const cursorInfo: CursorInfo = { line: 1, column: 1, offset: 0 }
    const { container } = render(
      <ModeLine bufferName="test.scm" modified={false} cursor={cursorInfo} />,
    )

    expect(container.textContent).toContain('test.scm')
  })

  it('displays modified indicator', () => {
    // 変更されたことを示すインジケーター (**) が表示されることを確認.
    const cursorInfo: CursorInfo = { line: 1, column: 1, offset: 0 }
    const { container } = render(
      <ModeLine bufferName="test.scm" modified={true} cursor={cursorInfo} />,
    )

    expect(container.textContent).toContain('**')
  })
})

/**
 * REPL コンポーネント のテストスイート.
 * Scheme REPL の入出力をテストします.
 */
describe('REPL Component', () => {
  it('renders with initial empty output', () => {
    // 初期状態でレディメッセージが表示されることを確認.
    render(<REPL output="" error="" history={[]} onEval={vi.fn()} />)

    expect(screen.getByText('Ready')).toBeTruthy()
  })

  it('displays output history', () => {
    // 評価履歴が表示されることを確認.
    const history = [
      { code: '(+ 1 2)', result: '3' },
      { code: '(* 3 4)', result: '12' },
    ]

    const { container } = render(<REPL output="" error="" history={history} onEval={vi.fn()} />)

    expect(container.textContent).toContain('(+ 1 2)')
    expect(container.textContent).toContain('3')
  })

  it('calls onEval when input is submitted', () => {
    // 入力を送信すると onEval が呼ばれることを確認.
    const onEval = vi.fn()
    const { container } = render(<REPL output="" error="" history={[]} onEval={onEval} />)

    const input = container.querySelector('input') as HTMLInputElement
    fireEvent.change(input, { target: { value: '(+ 1 1)' } })
    fireEvent.keyDown(input, { key: 'Enter' })

    expect(onEval).toHaveBeenCalledWith('(+ 1 1)')
  })

  it('displays error message when error occurs', () => {
    // エラーが発生したときエラーメッセージが表示されることを確認.
    render(<REPL output="" error="Division by zero" history={[]} onEval={vi.fn()} />)

    expect(screen.getByText(/Division by zero/)).toBeTruthy()
  })
})
