import { describe, it, expect, vi } from 'vitest'
import { render, screen, fireEvent } from '@testing-library/react'
import BufferList from '@/components/BufferList'
import REPL from '@/components/REPL'
import ModeLine from '@/components/ModeLine'
import Minibuffer, { MinibufferState } from '@/components/Minibuffer'
import { backwardSexp, forwardSexp } from '@/lib/bracketMatch'

describe('bracketMatch (文字列と, 閉じていない括弧)', () => {
  it('文字列のリテラルを 1 つの S 式として飛ばす (\\" を含む)', () => {
    const text = '"a \\" b" c'
    expect(forwardSexp(text, 0)).toBe(8)
    expect(backwardSexp(text, 8)).toBe(0)
  })

  it('閉じていない文字列は, 最後までを 1 つにする', () => {
    expect(forwardSexp('"abc', 0)).toBe(4)
    expect(forwardSexp('"ab\\', 0)).toBe(4)
  })

  it('対応する括弧がなければ, 端まで動く', () => {
    expect(forwardSexp('(a b', 0)).toBe(4)
    expect(backwardSexp('a b)', 4)).toBe(0)
  })

  it('空白だけなら, 端まで動くこと', () => {
    expect(forwardSexp('a  ', 1)).toBe(3)
    expect(backwardSexp('  a', 2)).toBe(0)
  })
})

describe('BufferList Component', () => {
  const buffers = [
    { id: '1', name: 'a.scm', filePath: '/tmp/a.scm', content: '(+ 1 2)', modified: true },
    { id: '2', name: '*scratch*', filePath: null, content: '', modified: false },
  ]

  it('バッファの一覧 (今のバッファ, 変更, 大きさ, ファイル) を表示する', () => {
    const { container } = render(
      <BufferList
        buffers={buffers}
        currentBufferId="1"
        onSelectBuffer={vi.fn()}
        onClose={vi.fn()}
      />,
    )
    const rows = container.querySelectorAll('tbody tr')

    expect(rows).toHaveLength(2)
    expect(rows[0].className).toContain('current')
    expect(rows[0].textContent).toContain('>*')
    expect(rows[0].textContent).toContain('7')
    expect(rows[0].textContent).toContain('/tmp/a.scm')
    expect(rows[1].className).not.toContain('current')
    expect(rows[1].textContent).toContain('--')
  })

  it('行を押すと選び, × と外側で閉じる (中を押しても閉じない)', () => {
    const onSelectBuffer = vi.fn()
    const onClose = vi.fn()
    const { container } = render(
      <BufferList
        buffers={buffers}
        currentBufferId="1"
        onSelectBuffer={onSelectBuffer}
        onClose={onClose}
      />,
    )

    fireEvent.click(screen.getByText('*scratch*'))
    expect(onSelectBuffer).toHaveBeenCalledWith('2')

    fireEvent.click(container.querySelector('.buffer-list-container') as Element)
    expect(onClose).not.toHaveBeenCalled()

    fireEvent.click(screen.getByText('×'))
    fireEvent.click(container.querySelector('.buffer-list-overlay') as Element)
    expect(onClose).toHaveBeenCalledTimes(2)
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

describe('ModeLine Component (バッファの数)', () => {
  it('バッファが複数なら, 何番目かを表示する', () => {
    const { container } = render(
      <ModeLine
        bufferName="a.scm"
        modified={false}
        cursor={{ line: 1, column: 0, offset: 0 }}
        bufferCount={3}
        currentBufferIndex={2}
      />,
    )
    expect(container.textContent).toContain('[2/3]')
  })
})

describe('Minibuffer Component (isearch の繰り返し)', () => {
  const isearch: MinibufferState = {
    mode: 'isearch',
    regexp: false,
    direction: 'forward',
    query: 'a',
    originalCursor: 0,
    matchStart: 0,
    matchEnd: 1,
  }

  it('繰り返しの関数がなくても, C-s と C-r は例外にならない', () => {
    const { container } = render(
      <Minibuffer state={isearch} onInputChange={vi.fn()} onSubmit={vi.fn()} onCancel={vi.fn()} />,
    )
    const input = container.querySelector('input') as HTMLInputElement

    expect(() => fireEvent.keyDown(input, { key: 's', ctrlKey: true })).not.toThrow()
    expect(() => fireEvent.keyDown(input, { key: 'r', ctrlKey: true })).not.toThrow()
  })

  it('Ctrl と g 以外のキーは, 取り消さない', () => {
    const onCancel = vi.fn()
    const { container } = render(
      <Minibuffer
        state={{ mode: 'input', prompt: 'M-x ', input: '' }}
        onInputChange={vi.fn()}
        onSubmit={vi.fn()}
        onCancel={onCancel}
      />,
    )
    fireEvent.keyDown(container.querySelector('input') as HTMLInputElement, {
      key: 'a',
      ctrlKey: true,
    })
    expect(onCancel).not.toHaveBeenCalled()
  })

  it('isearch では, Enter で検索の文字列を確定する', () => {
    const onSubmit = vi.fn()
    const { container } = render(
      <Minibuffer state={isearch} onInputChange={vi.fn()} onSubmit={onSubmit} onCancel={vi.fn()} />,
    )
    fireEvent.keyDown(container.querySelector('input') as HTMLInputElement, { key: 'Enter' })
    expect(onSubmit).toHaveBeenCalledWith('a')
  })
})
