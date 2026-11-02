import { describe, it, expect, vi } from 'vitest'
import { act, render, screen, fireEvent } from '@testing-library/react'
import Minibuffer, { MinibufferInputState, MinibufferState } from '@/components/Minibuffer'

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
    const props = { onInputChange, onSubmit: vi.fn(), onCancel: vi.fn(), onQueryKey: vi.fn() }
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
    const props = { onInputChange, onSubmit: vi.fn(), onCancel: vi.fn(), onQueryKey: vi.fn() }
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
  it('問いでは, 押したキーを Emacs の書き方で知らせること (C-g も. 修飾キーだけと, 変換中のキーは知らせない)', () => {
    const onQueryKey = vi.fn()
    const onCancel = vi.fn()
    const view = render(
      <Minibuffer
        state={{ mode: 'query', prompt: 'Query replacing a with b: ' }}
        onInputChange={vi.fn()}
        onSubmit={vi.fn()}
        onCancel={onCancel}
        onQueryKey={onQueryKey}
      />,
    )
    expect(view.container.textContent).toContain('Query replacing a with b: ')
    const input = view.container.querySelector('input') as HTMLInputElement
    fireEvent.keyDown(input, { key: 'y' })
    expect(onQueryKey).toHaveBeenLastCalledWith('y')
    fireEvent.keyDown(input, { key: ' ' })
    expect(onQueryKey).toHaveBeenLastCalledWith('SPC')
    fireEvent.keyDown(input, { key: 'Enter' })
    expect(onQueryKey).toHaveBeenLastCalledWith('RET')
    fireEvent.keyDown(input, { key: 'x', ctrlKey: true })
    expect(onQueryKey).toHaveBeenLastCalledWith('C-x')
    fireEvent.keyDown(input, { key: 'x', altKey: true })
    expect(onQueryKey).toHaveBeenLastCalledWith('M-x')
    fireEvent.keyDown(input, { key: 'x', metaKey: true })
    expect(onQueryKey).toHaveBeenLastCalledWith('s-x')
    fireEvent.keyDown(input, { key: 'g', ctrlKey: true })
    expect(onQueryKey).toHaveBeenLastCalledWith('C-g')
    fireEvent.keyDown(input, { key: 'Control', ctrlKey: true })
    fireEvent.keyDown(input, { key: 'Shift' })
    fireEvent.keyDown(input, { key: 'a', isComposing: true })
    fireEvent.keyDown(input, { key: 'a', keyCode: 229 })
    expect(onQueryKey).toHaveBeenCalledTimes(7)
    expect(onCancel).not.toHaveBeenCalled()
  })
})

describe('Minibuffer Component', () => {
  it('displays message when in message mode', () => {
    // メッセージモードでメッセージが表示されることを確認.
    const state: MinibufferState = { mode: 'message', text: 'Test message' }
    render(
      <Minibuffer
        state={state}
        onInputChange={vi.fn()}
        onSubmit={vi.fn()}
        onCancel={vi.fn()}
        onQueryKey={vi.fn()}
      />,
    )

    expect(screen.getByText('Test message')).toBeTruthy()
  })

  it('displays default message when text is empty', () => {
    // テキストが空のときデフォルトメッセージが表示されることを確認.
    const state: MinibufferState = { mode: 'message', text: '' }
    render(
      <Minibuffer
        state={state}
        onInputChange={vi.fn()}
        onSubmit={vi.fn()}
        onCancel={vi.fn()}
        onQueryKey={vi.fn()}
      />,
    )

    expect(screen.getByText('For information about this editor, see the README.')).toBeTruthy()
  })

  it('displays input field when in input mode', () => {
    // 入力モードで入力フィールドが表示されることを確認.
    const state: MinibufferState = { mode: 'input', prompt: 'Open file: ', input: '' }
    const { container } = render(
      <Minibuffer
        state={state}
        onInputChange={vi.fn()}
        onSubmit={vi.fn()}
        onCancel={vi.fn()}
        onQueryKey={vi.fn()}
      />,
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
        onQueryKey={vi.fn()}
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
      <Minibuffer
        state={state}
        onInputChange={vi.fn()}
        onSubmit={onSubmit}
        onCancel={vi.fn()}
        onQueryKey={vi.fn()}
      />,
    )

    const input = container.querySelector('input') as HTMLInputElement
    fireEvent.keyDown(input, { key: 'Enter' })

    expect(onSubmit).toHaveBeenCalledWith('test.scm')
  })

  it('変換中のキー (変換を確定する Enter, Escape) では, 確定もやめもしない', () => {
    const onSubmit = vi.fn()
    const onCancel = vi.fn()
    const state: MinibufferState = { mode: 'input', prompt: 'Find file: ', input: 'にほんご' }
    const { container } = render(
      <Minibuffer
        state={state}
        onInputChange={vi.fn()}
        onSubmit={onSubmit}
        onCancel={onCancel}
        onQueryKey={vi.fn()}
      />,
    )
    const input = container.querySelector('input') as HTMLInputElement
    fireEvent.keyDown(input, { key: 'Enter', isComposing: true })
    fireEvent.keyDown(input, { key: 'Escape', isComposing: true })
    fireEvent.keyDown(input, { key: 'Enter', keyCode: 229 })
    expect(onSubmit).not.toHaveBeenCalled()
    expect(onCancel).not.toHaveBeenCalled()

    // 確定のあとの Enter で, 入力を確定する
    fireEvent.keyDown(input, { key: 'Enter' })
    expect(onSubmit).toHaveBeenCalledWith('にほんご')
  })

  it('calls onCancel on Escape key', () => {
    // Escape キーが押されると onCancel が呼ばれることを確認.
    const onCancel = vi.fn()
    const state: MinibufferState = { mode: 'input', prompt: 'Open file: ', input: 'test.scm' }
    const { container } = render(
      <Minibuffer
        state={state}
        onInputChange={vi.fn()}
        onSubmit={vi.fn()}
        onCancel={onCancel}
        onQueryKey={vi.fn()}
      />,
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
      <Minibuffer
        state={state}
        onInputChange={vi.fn()}
        onSubmit={vi.fn()}
        onCancel={onCancel}
        onQueryKey={vi.fn()}
      />,
    )

    const input = container.querySelector('input') as HTMLInputElement
    fireEvent.keyDown(input, { key: 'g', ctrlKey: true })

    expect(onCancel).toHaveBeenCalled()
    // C-] (abort-recursive-edit) でも, やめる. Ctrl の無い ] は, 入力
    fireEvent.keyDown(input, { key: ']', ctrlKey: true })
    expect(onCancel).toHaveBeenCalledTimes(2)
    fireEvent.keyDown(input, { key: ']' })
    expect(onCancel).toHaveBeenCalledTimes(2)
  })
})

describe('Minibuffer Component (やめるキー)', () => {
  it('Ctrl と g 以外のキーは, 取り消さない', () => {
    const onCancel = vi.fn()
    const { container } = render(
      <Minibuffer
        state={{ mode: 'input', prompt: 'M-x ', input: '' }}
        onInputChange={vi.fn()}
        onSubmit={vi.fn()}
        onCancel={onCancel}
        onQueryKey={vi.fn()}
      />,
    )
    fireEvent.keyDown(container.querySelector('input') as HTMLInputElement, {
      key: 'a',
      ctrlKey: true,
    })
    expect(onCancel).not.toHaveBeenCalled()
  })
})

describe('Minibuffer Component (履歴の端)', () => {
  it('履歴の新しい端より先へは進まず, 知らせること', () => {
    const { container } = render(
      <Minibuffer
        state={{ mode: 'input', prompt: 'M-x ', input: '', history: ['a'] }}
        onInputChange={vi.fn()}
        onSubmit={vi.fn()}
        onCancel={vi.fn()}
        onQueryKey={vi.fn()}
      />,
    )
    fireEvent.keyDown(container.querySelector('input') as HTMLInputElement, {
      key: 'n',
      altKey: true,
    })
    expect(container.textContent).toContain('[End of history; no default available]')
  })
})

describe('Minibuffer Component (履歴を戻る)', () => {
  it('M-p で古い方へ, M-n で入力していた文字に戻ること', () => {
    let state: MinibufferInputState = {
      mode: 'input',
      prompt: 'M-x ',
      input: 'typed',
      history: ['a', 'b'],
    }
    const onInputChange = vi.fn((value: string, next: MinibufferInputState) => {
      state = { ...next, input: value }
    })
    const props = { onSubmit: vi.fn(), onCancel: vi.fn(), onQueryKey: vi.fn(), onInputChange }
    const view = render(<Minibuffer state={state} {...props} />)
    const key = (k: string) => {
      fireEvent.keyDown(view.container.querySelector('input') as HTMLInputElement, {
        key: k,
        altKey: true,
      })
      view.rerender(<Minibuffer state={state} {...props} />)
    }
    key('p')
    key('p')
    expect(state.input).toBe('b')
    key('n')
    expect(state.input).toBe('a')
    key('n')
    expect(state.input).toBe('typed')
  })
})
