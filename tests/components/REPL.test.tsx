import { describe, it, expect, vi } from 'vitest'
import { render, screen, fireEvent } from '@testing-library/react'
import REPL from '@/components/REPL'

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

describe('REPL Component (入力と履歴)', () => {
  const history = [
    { code: '(+ 1 2)', result: '3' },
    { code: '(* 2 3)', result: '6' },
  ]

  /**
   * REPL を表示して, 入力欄を返します.
   *
   * @param onEval - 評価の関数.
   * @param props - 出力とエラー.
   * @returns 入力欄.
   */
  const setup = (onEval = vi.fn(), props = { output: '', error: '' }) => {
    render(<REPL output={props.output} error={props.error} history={history} onEval={onEval} />)
    return screen.getByPlaceholderText('Enter Scheme code...') as HTMLInputElement
  }

  it('Enter とボタンで評価し, 空なら評価しない (Shift+Enter は評価しない)', () => {
    const onEval = vi.fn()
    const input = setup(onEval)

    fireEvent.keyDown(input, { key: 'Enter' })
    expect(onEval).not.toHaveBeenCalled()

    fireEvent.change(input, { target: { value: '(car x)' } })
    fireEvent.keyDown(input, { key: 'Enter', shiftKey: true })
    expect(onEval).not.toHaveBeenCalled()

    fireEvent.keyDown(input, { key: 'Enter' })
    expect(onEval).toHaveBeenCalledWith('(car x)')
    expect(input.value).toBe('')

    fireEvent.change(input, { target: { value: '(cdr x)' } })
    fireEvent.click(screen.getByText('Eval'))
    expect(onEval).toHaveBeenLastCalledWith('(cdr x)')
  })

  it('↑ で新しい順に履歴をたどり, ↓ で戻って, 最後は空にする', () => {
    const input = setup()

    fireEvent.keyDown(input, { key: 'ArrowUp' })
    expect(input.value).toBe('(* 2 3)')
    fireEvent.keyDown(input, { key: 'ArrowUp' })
    expect(input.value).toBe('(+ 1 2)')
    fireEvent.keyDown(input, { key: 'ArrowUp' })
    expect(input.value).toBe('(+ 1 2)')

    fireEvent.keyDown(input, { key: 'ArrowDown' })
    expect(input.value).toBe('(* 2 3)')
    fireEvent.keyDown(input, { key: 'ArrowDown' })
    expect(input.value).toBe('')
    fireEvent.keyDown(input, { key: 'ArrowDown' })
    expect(input.value).toBe('')

    fireEvent.keyDown(input, { key: 'a' })
    expect(input.value).toBe('')
  })

  it('エラー, 出力, 何もないときの表示', () => {
    const { unmount } = render(<REPL output="" error="boom" history={[]} onEval={vi.fn()} />)
    expect(screen.getByText('Error: boom')).toBeTruthy()
    unmount()

    const second = render(<REPL output="42" error="" history={[]} onEval={vi.fn()} />)
    expect(screen.getByText('42')).toBeTruthy()
    second.unmount()

    render(<REPL output="" error="" history={[]} onEval={vi.fn()} />)
    expect(screen.getByText('Ready')).toBeTruthy()
  })
})
