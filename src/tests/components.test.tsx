import { describe, it, expect, vi } from 'vitest'
import { render, screen, fireEvent } from '@testing-library/react'
import CodeEditor, { CursorInfo } from '../components/CodeEditor'
import Minibuffer, { MinibufferState } from '../components/Minibuffer'
import MenuBar from '../components/MenuBar'
import ModeLine from '../components/ModeLine'
import REPL from '../components/REPL'

/**
 * CodeEditor コンポーネント のテストスイート.
 * コード入力エリアの機能をテストします.
 */
describe('CodeEditor Component', () => {
  it('renders textarea with initial value', () => {
    // 初期値を持つテキストエリアがレンダリングされることを確認.
    const { container } = render(
      <CodeEditor
        value="(+ 1 2)"
        onChange={vi.fn()}
        onFindFile={vi.fn()}
        onSaveBuffer={vi.fn()}
        onIsearchForward={vi.fn()}
        onIsearchBackward={vi.fn()}
        onExecuteCommand={vi.fn()}
        onQuit={vi.fn()}
      />,
    )

    const textarea = container.querySelector('textarea') as HTMLTextAreaElement
    expect(textarea).toBeTruthy()
    expect(textarea.value).toBe('(+ 1 2)')
  })

  it('calls onChange when text is modified', () => {
    // テキストが変更されると onChange が呼ばれることを確認.
    const onChange = vi.fn()
    const { container } = render(
      <CodeEditor
        value=""
        onChange={onChange}
        onFindFile={vi.fn()}
        onSaveBuffer={vi.fn()}
        onIsearchForward={vi.fn()}
        onIsearchBackward={vi.fn()}
        onExecuteCommand={vi.fn()}
        onQuit={vi.fn()}
      />,
    )

    const textarea = container.querySelector('textarea') as HTMLTextAreaElement
    fireEvent.change(textarea, { target: { value: '(+ 1 1)' } })

    expect(onChange).toHaveBeenCalledWith('(+ 1 1)')
  })

  it('calls onCursorChange with cursor position info', () => {
    // カーソル位置情報が更新されると onCursorChange が呼ばれることを確認.
    const onCursorChange = vi.fn()
    const { container } = render(
      <CodeEditor
        value="hello\nworld"
        onChange={vi.fn()}
        onFindFile={vi.fn()}
        onSaveBuffer={vi.fn()}
        onCursorChange={onCursorChange}
        onIsearchForward={vi.fn()}
        onIsearchBackward={vi.fn()}
        onExecuteCommand={vi.fn()}
        onQuit={vi.fn()}
      />,
    )

    const textarea = container.querySelector('textarea') as HTMLTextAreaElement
    // カーソルを2行目の開始位置 (位置6) に移動.
    fireEvent.click(textarea)
    textarea.setSelectionRange(6, 6)
    fireEvent.click(textarea)

    expect(onCursorChange).toHaveBeenCalled()
  })

  it('disables textarea when disabled prop is true', () => {
    // disabled プロップが true のとき, テキストエリアが無効化されることを確認.
    const { container } = render(
      <CodeEditor
        value="text"
        onChange={vi.fn()}
        onFindFile={vi.fn()}
        onSaveBuffer={vi.fn()}
        onIsearchForward={vi.fn()}
        onIsearchBackward={vi.fn()}
        onExecuteCommand={vi.fn()}
        onQuit={vi.fn()}
        disabled={true}
      />,
    )

    const textarea = container.querySelector('textarea') as HTMLTextAreaElement
    expect(textarea.readOnly).toBe(true)
  })

  it('handles C-x C-f key sequence', () => {
    // C-x C-f キーシーケンスで onFindFile が呼ばれることを確認.
    const onFindFile = vi.fn()
    const { container } = render(
      <CodeEditor
        value=""
        onChange={vi.fn()}
        onFindFile={onFindFile}
        onSaveBuffer={vi.fn()}
        onIsearchForward={vi.fn()}
        onIsearchBackward={vi.fn()}
        onExecuteCommand={vi.fn()}
        onQuit={vi.fn()}
      />,
    )

    const textarea = container.querySelector('textarea') as HTMLTextAreaElement

    // C-x を押す.
    fireEvent.keyDown(textarea, { key: 'x', ctrlKey: true })
    // C-f を押す.
    fireEvent.keyDown(textarea, { key: 'f', ctrlKey: true })

    expect(onFindFile).toHaveBeenCalled()
  })

  it('handles C-x C-s key sequence', () => {
    // C-x C-s キーシーケンスで onSaveBuffer が呼ばれることを確認.
    const onSaveBuffer = vi.fn()
    const { container } = render(
      <CodeEditor
        value=""
        onChange={vi.fn()}
        onFindFile={vi.fn()}
        onSaveBuffer={onSaveBuffer}
        onIsearchForward={vi.fn()}
        onIsearchBackward={vi.fn()}
        onExecuteCommand={vi.fn()}
        onQuit={vi.fn()}
      />,
    )

    const textarea = container.querySelector('textarea') as HTMLTextAreaElement

    // C-x を押す.
    fireEvent.keyDown(textarea, { key: 'x', ctrlKey: true })
    // C-s を押す.
    fireEvent.keyDown(textarea, { key: 's', ctrlKey: true })

    expect(onSaveBuffer).toHaveBeenCalled()
  })

  it('C-k kills to end of line via emacs keymap', () => {
    // C-k で行末まで kill されることを確認.
    const onChange = vi.fn()
    const { container } = render(
      <CodeEditor
        value="hello world"
        onChange={onChange}
        onFindFile={vi.fn()}
        onSaveBuffer={vi.fn()}
        onIsearchForward={vi.fn()}
        onIsearchBackward={vi.fn()}
        onExecuteCommand={vi.fn()}
        onQuit={vi.fn()}
      />,
    )

    const textarea = container.querySelector('textarea') as HTMLTextAreaElement
    textarea.setSelectionRange(5, 5)
    fireEvent.keyDown(textarea, { key: 'k', ctrlKey: true })

    expect(onChange).toHaveBeenCalledWith('hello')
  })

  it('C-f moves cursor without emitting onChange', () => {
    // C-f でカーソルだけ動かし onChange は呼ばれないことを確認.
    const onChange = vi.fn()
    const { container } = render(
      <CodeEditor
        value="hello"
        onChange={onChange}
        onFindFile={vi.fn()}
        onSaveBuffer={vi.fn()}
        onIsearchForward={vi.fn()}
        onIsearchBackward={vi.fn()}
        onExecuteCommand={vi.fn()}
        onQuit={vi.fn()}
      />,
    )

    const textarea = container.querySelector('textarea') as HTMLTextAreaElement
    textarea.setSelectionRange(0, 0)
    fireEvent.keyDown(textarea, { key: 'f', ctrlKey: true })

    expect(onChange).not.toHaveBeenCalled()
    expect(textarea.selectionStart).toBe(1)
  })

  it('applies highlightRange to textarea selection', () => {
    // highlightRange prop が textarea の selection に反映されることを確認.
    const { container } = render(
      <CodeEditor
        value="hello world"
        onChange={vi.fn()}
        onFindFile={vi.fn()}
        onSaveBuffer={vi.fn()}
        onIsearchForward={vi.fn()}
        onIsearchBackward={vi.fn()}
        onExecuteCommand={vi.fn()}
        onQuit={vi.fn()}
        highlightRange={{ start: 0, end: 5 }}
      />,
    )

    const textarea = container.querySelector('textarea') as HTMLTextAreaElement
    expect(textarea.selectionStart).toBe(0)
    expect(textarea.selectionEnd).toBe(5)
  })
})

/**
 * Minibuffer コンポーネント のテストスイート.
 * メッセージ表示と入力プロンプトをテストします.
 */
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

    expect(onInputChange).toHaveBeenCalledWith('test.scm')
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

    expect(onIsearchRepeat).toHaveBeenCalledWith('forward')
  })

  it('calls onIsearchRepeat with backward direction on C-r during isearch', () => {
    // isearch 中に C-r が押されると onIsearchRepeat が呼ばれることを確認.
    const onIsearchRepeat = vi.fn()
    const state: MinibufferState = {
      mode: 'isearch',
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

    expect(onIsearchRepeat).toHaveBeenCalledWith('backward')
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
 * MenuBar コンポーネント のテストスイート.
 * メニュー項目の表示をテストします.
 */
describe('MenuBar Component', () => {
  it('renders menu options', () => {
    // メニュー項目が表示されることを確認.
    render(<MenuBar />)

    expect(screen.getByText('File')).toBeTruthy()
    expect(screen.getByText('Edit')).toBeTruthy()
    expect(screen.getByText('Help')).toBeTruthy()
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
