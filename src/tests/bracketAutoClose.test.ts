import { describe, it, expect, vi } from 'vitest'
import { renderHook } from '@testing-library/react'
import { useBracketAutoClose } from '../lib/bracketAutoClose'

/**
 * useBracketAutoClose フック のテストスイート.
 * 括弧と引用符の自動閉鎖機能をテストします.
 */
describe('useBracketAutoClose', () => {
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
      } as any,
    }
    return ref
  }

  it('auto-closes opening paren', () => {
    // 開き括弧を入力すると, 閉じ括弧が自動的に追加されることを確認.
    const textareaRef = createTextareaRef('', 0, 0)
    const onChange = vi.fn()
    const { result } = renderHook(() => useBracketAutoClose({ textareaRef, value: '', onChange }))

    const event = {
      key: '(',
      ctrlKey: false,
      altKey: false,
      metaKey: false,
      preventDefault: vi.fn(),
    } as any

    const res = result.current.handleKeyDown(event)

    expect(res).toBe(true)
    expect(event.preventDefault).toHaveBeenCalled()
    expect(onChange).toHaveBeenCalledWith('()', 1)
  })

  it('auto-closes opening quote', () => {
    // 開き引用符を入力すると, 閉じ引用符が自動的に追加されることを確認.
    const textareaRef = createTextareaRef('', 0, 0)
    const onChange = vi.fn()
    const { result } = renderHook(() => useBracketAutoClose({ textareaRef, value: '', onChange }))

    const event = {
      key: '"',
      ctrlKey: false,
      altKey: false,
      metaKey: false,
      preventDefault: vi.fn(),
    } as any

    const res = result.current.handleKeyDown(event)

    expect(res).toBe(true)
    expect(onChange).toHaveBeenCalledWith('""', 1)
  })

  it('wraps selection with brackets', () => {
    // 選択されたテキストを括弧で囲むことを確認.
    const textareaRef = createTextareaRef('hello', 0, 5)
    const onChange = vi.fn()
    const { result } = renderHook(() => useBracketAutoClose({ textareaRef, value: 'hello', onChange }))

    const event = {
      key: '(',
      ctrlKey: false,
      altKey: false,
      metaKey: false,
      preventDefault: vi.fn(),
    } as any

    result.current.handleKeyDown(event)
    // ラップ後, カーソルは選択されたテキストの後に配置されます.
    expect(onChange).toHaveBeenCalledWith('(hello)', 6)
  })

  it('skips over existing closing paren', () => {
    // 既に閉じ括弧が存在する場合, その上をスキップして移動することを確認.
    const textareaRef = createTextareaRef('()', 1, 1)
    const onChange = vi.fn()
    const { result } = renderHook(() => useBracketAutoClose({ textareaRef, value: '()', onChange }))

    const event = {
      key: ')',
      ctrlKey: false,
      altKey: false,
      metaKey: false,
      preventDefault: vi.fn(),
    } as any

    const res = result.current.handleKeyDown(event)

    expect(res).toBe(true)
    expect(onChange).toHaveBeenCalledWith('()', 2)
  })

  it('deletes both parens on backspace in empty pair', () => {
    // 空の括弧ペア内でバックスペースを押すと, 両方削除されることを確認.
    const textareaRef = createTextareaRef('()', 1, 1)
    const onChange = vi.fn()
    const { result } = renderHook(() => useBracketAutoClose({ textareaRef, value: '()', onChange }))

    const event = {
      key: 'Backspace',
      ctrlKey: false,
      altKey: false,
      metaKey: false,
      preventDefault: vi.fn(),
    } as any

    const res = result.current.handleKeyDown(event)

    expect(res).toBe(true)
    expect(onChange).toHaveBeenCalledWith('', 0)
  })

  it('ignores auto-close when ctrl key is pressed', () => {
    // Ctrl キーが押されている場合, 自動閉鎖が無視されることを確認.
    const textareaRef = createTextareaRef('', 0, 0)
    const onChange = vi.fn()
    const { result } = renderHook(() => useBracketAutoClose({ textareaRef, value: '', onChange }))

    const event = {
      key: '(',
      ctrlKey: true,
      altKey: false,
      metaKey: false,
      preventDefault: vi.fn(),
    } as any

    const res = result.current.handleKeyDown(event)

    expect(res).toBe(false)
    expect(onChange).not.toHaveBeenCalled()
  })

  it('ignores auto-close when alt key is pressed', () => {
    // Alt キーが押されている場合, 自動閉鎖が無視されることを確認.
    const textareaRef = createTextareaRef('', 0, 0)
    const onChange = vi.fn()
    const { result } = renderHook(() => useBracketAutoClose({ textareaRef, value: '', onChange }))

    const event = {
      key: '(',
      ctrlKey: false,
      altKey: true,
      metaKey: false,
      preventDefault: vi.fn(),
    } as any

    const res = result.current.handleKeyDown(event)

    expect(res).toBe(false)
    expect(onChange).not.toHaveBeenCalled()
  })

  it('returns false for regular keys', () => {
    // 通常のキーの場合は false を返すことを確認.
    const textareaRef = createTextareaRef('', 0, 0)
    const onChange = vi.fn()
    const { result } = renderHook(() => useBracketAutoClose({ textareaRef, value: '', onChange }))

    const event = {
      key: 'a',
      ctrlKey: false,
      altKey: false,
      metaKey: false,
      preventDefault: vi.fn(),
    } as any

    const res = result.current.handleKeyDown(event)

    expect(res).toBe(false)
  })
})
