import { describe, it, expect, vi } from 'vitest'
import { renderHook } from '@testing-library/react'
import { useEmacsKeymap } from '../lib/emacsKeymap'

/**
 * useEmacsKeymap フック のテストスイート.
 * Emacs スタイルのキーバインディングをテストします.
 */
describe('useEmacsKeymap', () => {
  /**
   * テスト用のテキストエリア参照を作成します.
   * @param text - テキストエリアの内容.
   * @param selectionStart - 選択開始位置.
   * @param selectionEnd - 選択終了位置.
   * @returns テキストエリア参照オブジェクト.
   */
  const createTextareaRef = (text: string, selectionStart: number, selectionEnd: number) => {
    const ref = {
      current: {
        value: text,
        selectionStart,
        selectionEnd,
        setSelectionRange: vi.fn(),
      } as unknown as HTMLTextAreaElement,
    }
    return ref
  }

  describe('movement commands', () => {
    // 移動コマンド (カーソル移動) をテストします.

    it('C-f moves forward one character', () => {
      // C-f で1文字前に移動することを確認.
      const textareaRef = createTextareaRef('hello', 0, 0)
      const onChange = vi.fn()
      const { result } = renderHook(() =>
        useEmacsKeymap({
          textareaRef,
          value: 'hello',
          onChange,
        }),
      )

      const event = {
        key: 'f',
        ctrlKey: true,
        altKey: false,
        preventDefault: vi.fn(),
      } as unknown as KeyboardEvent<HTMLTextAreaElement>

      result.current.handleKeyDown(event)

      expect(event.preventDefault).toHaveBeenCalled()
      expect(textareaRef.current.setSelectionRange).toHaveBeenCalledWith(1, 1)
    })

    it('C-b moves backward one character', () => {
      // C-b で1文字後ろに移動することを確認.
      const textareaRef = createTextareaRef('hello', 3, 3)
      const onChange = vi.fn()
      const { result } = renderHook(() =>
        useEmacsKeymap({
          textareaRef,
          value: 'hello',
          onChange,
        }),
      )

      const event = {
        key: 'b',
        ctrlKey: true,
        altKey: false,
        preventDefault: vi.fn(),
      } as unknown as KeyboardEvent<HTMLTextAreaElement>

      result.current.handleKeyDown(event)

      expect(textareaRef.current.setSelectionRange).toHaveBeenCalledWith(2, 2)
    })

    it('C-a moves to beginning of line', () => {
      // C-a で行頭に移動することを確認.
      const text = 'line1\nline2'
      const textareaRef = createTextareaRef(text, 8, 8) // 'line2' の途中
      const onChange = vi.fn()
      const { result } = renderHook(() =>
        useEmacsKeymap({
          textareaRef,
          value: text,
          onChange,
        }),
      )

      const event = {
        key: 'a',
        ctrlKey: true,
        altKey: false,
        preventDefault: vi.fn(),
      } as unknown as KeyboardEvent<HTMLTextAreaElement>

      result.current.handleKeyDown(event)

      // 'line2' の行頭に移動.
      expect(textareaRef.current.setSelectionRange).toHaveBeenCalledWith(6, 6)
    })

    it('C-e moves to end of line', () => {
      // C-e で行末に移動することを確認.
      const text = 'line1\nline2'
      const textareaRef = createTextareaRef(text, 6, 6)
      const onChange = vi.fn()
      const { result } = renderHook(() =>
        useEmacsKeymap({
          textareaRef,
          value: text,
          onChange,
        }),
      )

      const event = {
        key: 'e',
        ctrlKey: true,
        altKey: false,
        preventDefault: vi.fn(),
      } as unknown as KeyboardEvent<HTMLTextAreaElement>

      result.current.handleKeyDown(event)

      // line2 の行末に移動.
      expect(textareaRef.current.setSelectionRange).toHaveBeenCalledWith(11, 11)
    })

    it('M-f moves forward one word', () => {
      // M-f で1単語前に移動することを確認.
      const textareaRef = createTextareaRef('hello world', 0, 0)
      const onChange = vi.fn()
      const { result } = renderHook(() =>
        useEmacsKeymap({
          textareaRef,
          value: 'hello world',
          onChange,
        }),
      )

      const event = {
        key: 'f',
        ctrlKey: false,
        altKey: true,
        preventDefault: vi.fn(),
      } as unknown as KeyboardEvent<HTMLTextAreaElement>

      result.current.handleKeyDown(event)

      expect(event.preventDefault).toHaveBeenCalled()
      expect(textareaRef.current.setSelectionRange).toHaveBeenCalledWith(5, 5)
    })

    it('M-b moves backward one word', () => {
      // M-b で1単語後ろに移動することを確認.
      const textareaRef = createTextareaRef('hello world', 11, 11)
      const onChange = vi.fn()
      const { result } = renderHook(() =>
        useEmacsKeymap({
          textareaRef,
          value: 'hello world',
          onChange,
        }),
      )

      const event = {
        key: 'b',
        ctrlKey: false,
        altKey: true,
        preventDefault: vi.fn(),
      } as unknown as KeyboardEvent<HTMLTextAreaElement>

      result.current.handleKeyDown(event)

      expect(textareaRef.current.setSelectionRange).toHaveBeenCalledWith(6, 6)
    })
  })

  describe('deletion commands', () => {
    // 削除コマンドをテストします.

    it('C-d deletes character at point', () => {
      // C-d でカーソル位置の文字を削除することを確認.
      const onChange = vi.fn()
      const textareaRef = createTextareaRef('hello', 1, 1)
      const { result } = renderHook(() =>
        useEmacsKeymap({
          textareaRef,
          value: 'hello',
          onChange,
        }),
      )

      const event = {
        key: 'd',
        ctrlKey: true,
        altKey: false,
        preventDefault: vi.fn(),
      } as unknown as KeyboardEvent<HTMLTextAreaElement>

      result.current.handleKeyDown(event)

      expect(event.preventDefault).toHaveBeenCalled()
      expect(onChange).toHaveBeenCalledWith('hllo', 1)
    })

    it('C-k kills from point to end of line', () => {
      // C-k でカーソルから行末までを削除することを確認.
      const text = 'hello\nworld'
      const onChange = vi.fn()
      const textareaRef = createTextareaRef(text, 1, 1)
      const { result } = renderHook(() =>
        useEmacsKeymap({
          textareaRef,
          value: text,
          onChange,
        }),
      )

      const event = {
        key: 'k',
        ctrlKey: true,
        altKey: false,
        preventDefault: vi.fn(),
      } as unknown as KeyboardEvent<HTMLTextAreaElement>

      result.current.handleKeyDown(event)

      expect(event.preventDefault).toHaveBeenCalled()
      expect(onChange).toHaveBeenCalledWith('h\nworld', 1)
    })

    it('C-w kills region', () => {
      // C-w で選択領域を削除することを確認.
      const onChange = vi.fn()
      const textareaRef = createTextareaRef('hello world', 0, 5)
      const { result } = renderHook(() =>
        useEmacsKeymap({
          textareaRef,
          value: 'hello world',
          onChange,
        }),
      )

      const event = {
        key: 'w',
        ctrlKey: true,
        altKey: false,
        preventDefault: vi.fn(),
      } as unknown as KeyboardEvent<HTMLTextAreaElement>

      result.current.handleKeyDown(event)

      expect(event.preventDefault).toHaveBeenCalled()
      expect(onChange).toHaveBeenCalledWith(' world', 0)
    })
  })

  describe('mark and region', () => {
    // マークと選択領域に関連するコマンドをテストします.

    it('C-space sets mark', () => {
      // C-space でマークを設定することを確認.
      const onChange = vi.fn()
      const textareaRef = createTextareaRef('hello', 2, 2)
      const { result } = renderHook(() =>
        useEmacsKeymap({
          textareaRef,
          value: 'hello',
          onChange,
        }),
      )

      const event = {
        key: ' ',
        ctrlKey: true,
        altKey: false,
        preventDefault: vi.fn(),
      } as unknown as KeyboardEvent<HTMLTextAreaElement>

      result.current.handleKeyDown(event)

      expect(event.preventDefault).toHaveBeenCalled()
    })

    it('C-g clears mark', () => {
      // C-g でマークを消去することを確認.
      const onChange = vi.fn()
      const textareaRef = createTextareaRef('hello', 2, 2)
      const { result } = renderHook(() =>
        useEmacsKeymap({
          textareaRef,
          value: 'hello',
          onChange,
        }),
      )

      // まずマークを設定.
      const markEvent = {
        key: ' ',
        ctrlKey: true,
        altKey: false,
        preventDefault: vi.fn(),
      } as unknown as KeyboardEvent<HTMLTextAreaElement>

      result.current.handleKeyDown(markEvent)

      // 次にマークを消去.
      const quitEvent = {
        key: 'g',
        ctrlKey: true,
        altKey: false,
        preventDefault: vi.fn(),
      } as unknown as KeyboardEvent<HTMLTextAreaElement>

      result.current.handleKeyDown(quitEvent)

      expect(quitEvent.preventDefault).toHaveBeenCalled()
    })
  })

  describe('C-x handling', () => {
    it('does not intercept C-x (ownership moved to prefixKeymap)', () => {
      const onChange = vi.fn()
      const textareaRef = createTextareaRef('hello', 0, 0)
      const { result } = renderHook(() => useEmacsKeymap({ textareaRef, value: 'hello', onChange }))

      const event = { key: 'x', ctrlKey: true, altKey: false, preventDefault: vi.fn() } as unknown as KeyboardEvent<HTMLTextAreaElement>
      result.current.handleKeyDown(event)

      expect(event.preventDefault).not.toHaveBeenCalled()
      expect(onChange).not.toHaveBeenCalled()
    })
  })

  describe('isearch and extended commands', () => {
    it('C-s calls onIsearchForward', () => {
      const onChange = vi.fn()
      const onIsearchForward = vi.fn()
      const textareaRef = createTextareaRef('hello', 0, 0)
      const { result } = renderHook(() =>
        useEmacsKeymap({ textareaRef, value: 'hello', onChange, onIsearchForward }),
      )

      const event = { key: 's', ctrlKey: true, altKey: false, preventDefault: vi.fn() } as unknown as KeyboardEvent<HTMLTextAreaElement>
      result.current.handleKeyDown(event)

      expect(event.preventDefault).toHaveBeenCalled()
      expect(onIsearchForward).toHaveBeenCalled()
    })

    it('C-r calls onIsearchBackward', () => {
      const onChange = vi.fn()
      const onIsearchBackward = vi.fn()
      const textareaRef = createTextareaRef('hello', 5, 5)
      const { result } = renderHook(() =>
        useEmacsKeymap({ textareaRef, value: 'hello', onChange, onIsearchBackward }),
      )

      const event = { key: 'r', ctrlKey: true, altKey: false, preventDefault: vi.fn() } as unknown as KeyboardEvent<HTMLTextAreaElement>
      result.current.handleKeyDown(event)

      expect(event.preventDefault).toHaveBeenCalled()
      expect(onIsearchBackward).toHaveBeenCalled()
    })

    it('M-x calls onExecuteCommand', () => {
      const onChange = vi.fn()
      const onExecuteCommand = vi.fn()
      const textareaRef = createTextareaRef('hello', 0, 0)
      const { result } = renderHook(() =>
        useEmacsKeymap({ textareaRef, value: 'hello', onChange, onExecuteCommand }),
      )

      const event = { key: 'x', ctrlKey: false, altKey: true, preventDefault: vi.fn() } as unknown as KeyboardEvent<HTMLTextAreaElement>
      result.current.handleKeyDown(event)

      expect(event.preventDefault).toHaveBeenCalled()
      expect(onExecuteCommand).toHaveBeenCalled()
    })

    it('C-g calls onQuit', () => {
      const onChange = vi.fn()
      const onQuit = vi.fn()
      const textareaRef = createTextareaRef('hello', 0, 0)
      const { result } = renderHook(() =>
        useEmacsKeymap({ textareaRef, value: 'hello', onChange, onQuit }),
      )

      const event = { key: 'g', ctrlKey: true, altKey: false, preventDefault: vi.fn() } as unknown as KeyboardEvent<HTMLTextAreaElement>
      result.current.handleKeyDown(event)

      expect(event.preventDefault).toHaveBeenCalled()
      expect(onQuit).toHaveBeenCalled()
    })
  })
})
