import { describe, it, expect, vi } from 'vitest'
import { render, screen, fireEvent } from '@testing-library/react'
import CodeEditor, { CursorInfo } from '../components/CodeEditor'
import Minibuffer, { MinibufferState } from '../components/Minibuffer'
import MenuBar from '../components/MenuBar'
import ModeLine from '../components/ModeLine'
import REPL from '../components/REPL'

describe('CodeEditor Component', () => {
  it('renders textarea with initial value', () => {
    const { container } = render(
      <CodeEditor
        value="(+ 1 2)"
        onChange={vi.fn()}
        onFindFile={vi.fn()}
        onSaveBuffer={vi.fn()}
      />,
    )

    const textarea = container.querySelector('textarea') as HTMLTextAreaElement
    expect(textarea).toBeTruthy()
    expect(textarea.value).toBe('(+ 1 2)')
  })

  it('calls onChange when text is modified', () => {
    const onChange = vi.fn()
    const { container } = render(
      <CodeEditor
        value=""
        onChange={onChange}
        onFindFile={vi.fn()}
        onSaveBuffer={vi.fn()}
      />,
    )

    const textarea = container.querySelector('textarea') as HTMLTextAreaElement
    fireEvent.change(textarea, { target: { value: '(+ 1 1)' } })

    expect(onChange).toHaveBeenCalledWith('(+ 1 1)')
  })

  it('calls onCursorChange with cursor position info', () => {
    const onCursorChange = vi.fn()
    const { container } = render(
      <CodeEditor
        value="hello\nworld"
        onChange={vi.fn()}
        onFindFile={vi.fn()}
        onSaveBuffer={vi.fn()}
        onCursorChange={onCursorChange}
      />,
    )

    const textarea = container.querySelector('textarea') as HTMLTextAreaElement
    // Simulate cursor at position 6 (start of second line)
    fireEvent.click(textarea)
    textarea.setSelectionRange(6, 6)
    fireEvent.click(textarea)

    expect(onCursorChange).toHaveBeenCalled()
  })

  it('disables textarea when disabled prop is true', () => {
    const { container } = render(
      <CodeEditor
        value="text"
        onChange={vi.fn()}
        onFindFile={vi.fn()}
        onSaveBuffer={vi.fn()}
        disabled={true}
      />,
    )

    const textarea = container.querySelector('textarea') as HTMLTextAreaElement
    expect(textarea.readOnly).toBe(true)
  })

  it('handles C-x C-f key sequence', () => {
    const onFindFile = vi.fn()
    const { container } = render(
      <CodeEditor
        value=""
        onChange={vi.fn()}
        onFindFile={onFindFile}
        onSaveBuffer={vi.fn()}
      />,
    )

    const textarea = container.querySelector('textarea') as HTMLTextAreaElement

    // Simulate C-x
    fireEvent.keyDown(textarea, { key: 'x', ctrlKey: true })
    // Simulate C-f
    fireEvent.keyDown(textarea, { key: 'f', ctrlKey: true })

    expect(onFindFile).toHaveBeenCalled()
  })

  it('handles C-x C-s key sequence', () => {
    const onSaveBuffer = vi.fn()
    const { container } = render(
      <CodeEditor
        value=""
        onChange={vi.fn()}
        onFindFile={vi.fn()}
        onSaveBuffer={onSaveBuffer}
      />,
    )

    const textarea = container.querySelector('textarea') as HTMLTextAreaElement

    // Simulate C-x
    fireEvent.keyDown(textarea, { key: 'x', ctrlKey: true })
    // Simulate C-s
    fireEvent.keyDown(textarea, { key: 's', ctrlKey: true })

    expect(onSaveBuffer).toHaveBeenCalled()
  })
})

describe('Minibuffer Component', () => {
  it('displays message when in message mode', () => {
    const state: MinibufferState = { mode: 'message', text: 'Test message' }
    render(
      <Minibuffer
        state={state}
        onInputChange={vi.fn()}
        onSubmit={vi.fn()}
        onCancel={vi.fn()}
      />,
    )

    expect(screen.getByText('Test message')).toBeTruthy()
  })

  it('displays default message when text is empty', () => {
    const state: MinibufferState = { mode: 'message', text: '' }
    render(
      <Minibuffer
        state={state}
        onInputChange={vi.fn()}
        onSubmit={vi.fn()}
        onCancel={vi.fn()}
      />,
    )

    expect(screen.getByText('For information about this editor, see the README.')).toBeTruthy()
  })

  it('displays input field when in input mode', () => {
    const state: MinibufferState = { mode: 'input', prompt: 'Open file: ', input: '' }
    const { container } = render(
      <Minibuffer
        state={state}
        onInputChange={vi.fn()}
        onSubmit={vi.fn()}
        onCancel={vi.fn()}
      />,
    )

    expect(container.querySelector('input')).toBeTruthy()
    const prompt = container.querySelector('.minibuffer-prompt')
    expect(prompt?.textContent).toBe('Open file: ')
  })

  it('calls onInputChange when input changes', () => {
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
    const onSubmit = vi.fn()
    const state: MinibufferState = { mode: 'input', prompt: 'Open file: ', input: 'test.scm' }
    const { container } = render(
      <Minibuffer
        state={state}
        onInputChange={vi.fn()}
        onSubmit={onSubmit}
        onCancel={vi.fn()}
      />,
    )

    const input = container.querySelector('input') as HTMLInputElement
    fireEvent.keyDown(input, { key: 'Enter' })

    expect(onSubmit).toHaveBeenCalledWith('test.scm')
  })

  it('calls onCancel on Escape key', () => {
    const onCancel = vi.fn()
    const state: MinibufferState = { mode: 'input', prompt: 'Open file: ', input: 'test.scm' }
    const { container } = render(
      <Minibuffer
        state={state}
        onInputChange={vi.fn()}
        onSubmit={vi.fn()}
        onCancel={onCancel}
      />,
    )

    const input = container.querySelector('input') as HTMLInputElement
    fireEvent.keyDown(input, { key: 'Escape' })

    expect(onCancel).toHaveBeenCalled()
  })

  it('calls onCancel on C-g', () => {
    const onCancel = vi.fn()
    const state: MinibufferState = { mode: 'input', prompt: 'Open file: ', input: 'test.scm' }
    const { container } = render(
      <Minibuffer
        state={state}
        onInputChange={vi.fn()}
        onSubmit={vi.fn()}
        onCancel={onCancel}
      />,
    )

    const input = container.querySelector('input') as HTMLInputElement
    fireEvent.keyDown(input, { key: 'g', ctrlKey: true })

    expect(onCancel).toHaveBeenCalled()
  })
})

describe('ModeLine Component', () => {
  it('renders with cursor info', () => {
    const cursorInfo: CursorInfo = { line: 5, column: 10, offset: 42 }
    const { container } = render(
      <ModeLine bufferName="test.scm" modified={false} cursor={cursorInfo} />
    )

    expect(container.textContent).toContain('L5')
  })

  it('displays filename when provided', () => {
    const cursorInfo: CursorInfo = { line: 1, column: 1, offset: 0 }
    const { container } = render(
      <ModeLine bufferName="test.scm" modified={false} cursor={cursorInfo} />
    )

    expect(container.textContent).toContain('test.scm')
  })

  it('displays modified indicator', () => {
    const cursorInfo: CursorInfo = { line: 1, column: 1, offset: 0 }
    const { container } = render(
      <ModeLine bufferName="test.scm" modified={true} cursor={cursorInfo} />
    )

    expect(container.textContent).toContain('**')
  })
})

describe('MenuBar Component', () => {
  it('renders menu options', () => {
    render(<MenuBar />)

    expect(screen.getByText('File')).toBeTruthy()
    expect(screen.getByText('Edit')).toBeTruthy()
    expect(screen.getByText('Help')).toBeTruthy()
  })
})

describe('REPL Component', () => {
  it('renders with initial empty output', () => {
    render(
      <REPL
        output=""
        error=""
        history={[]}
        onEval={vi.fn()}
      />,
    )

    // REPL should render without errors
    expect(screen.getByText('Ready')).toBeTruthy()
  })

  it('displays output history', () => {
    const history = [
      { code: '(+ 1 2)', result: '3' },
      { code: '(* 3 4)', result: '12' },
    ]

    const { container } = render(
      <REPL
        output=""
        error=""
        history={history}
        onEval={vi.fn()}
      />,
    )

    expect(container.textContent).toContain('(+ 1 2)')
    expect(container.textContent).toContain('3')
  })

  it('calls onEval when input is submitted', () => {
    const onEval = vi.fn()
    const { container } = render(
      <REPL
        output=""
        error=""
        history={[]}
        onEval={onEval}
      />,
    )

    const input = container.querySelector('input') as HTMLInputElement
    fireEvent.change(input, { target: { value: '(+ 1 1)' } })
    fireEvent.keyDown(input, { key: 'Enter' })

    expect(onEval).toHaveBeenCalledWith('(+ 1 1)')
  })

  it('displays error message when error occurs', () => {
    render(
      <REPL
        output=""
        error="Division by zero"
        history={[]}
        onEval={vi.fn()}
      />,
    )

    expect(screen.getByText(/Division by zero/)).toBeTruthy()
  })
})
