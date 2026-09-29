import { describe, it, expect, vi } from 'vitest'
import { renderHook } from '@testing-library/react'
import { useBracketAutoClose } from '../lib/bracketAutoClose'

describe('useBracketAutoClose', () => {
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
    // After wrapping, cursor should be after the selected text, before the closing paren
    expect(onChange).toHaveBeenCalledWith('(hello)', 6)
  })

  it('skips over existing closing paren', () => {
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
