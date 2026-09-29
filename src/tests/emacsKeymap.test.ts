import { describe, it, expect, vi } from 'vitest'
import { renderHook } from '@testing-library/react'
import { useEmacsKeymap } from '../lib/emacsKeymap'

describe('useEmacsKeymap', () => {
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

  describe('movement commands', () => {
    it('C-f moves forward one character', () => {
      const textareaRef = createTextareaRef('hello', 0, 0)
      const onChange = vi.fn()
      const { result } = renderHook(() =>
        useEmacsKeymap({
          textareaRef,
          value: 'hello',
          onChange,
          onSave: vi.fn(),
        }),
      )

      const event = {
        key: 'f',
        ctrlKey: true,
        altKey: false,
        preventDefault: vi.fn(),
      } as any

      result.current.handleKeyDown(event)

      expect(event.preventDefault).toHaveBeenCalled()
      expect(textareaRef.current.setSelectionRange).toHaveBeenCalledWith(1, 1)
    })

    it('C-b moves backward one character', () => {
      const textareaRef = createTextareaRef('hello', 3, 3)
      const onChange = vi.fn()
      const { result } = renderHook(() =>
        useEmacsKeymap({
          textareaRef,
          value: 'hello',
          onChange,
          onSave: vi.fn(),
        }),
      )

      const event = {
        key: 'b',
        ctrlKey: true,
        altKey: false,
        preventDefault: vi.fn(),
      } as any

      result.current.handleKeyDown(event)

      expect(textareaRef.current.setSelectionRange).toHaveBeenCalledWith(2, 2)
    })

    it('C-a moves to beginning of line', () => {
      const text = 'line1\nline2'
      const textareaRef = createTextareaRef(text, 8, 8) // middle of 'line2'
      const onChange = vi.fn()
      const { result } = renderHook(() =>
        useEmacsKeymap({
          textareaRef,
          value: text,
          onChange,
          onSave: vi.fn(),
        }),
      )

      const event = {
        key: 'a',
        ctrlKey: true,
        altKey: false,
        preventDefault: vi.fn(),
      } as any

      result.current.handleKeyDown(event)

      expect(textareaRef.current.setSelectionRange).toHaveBeenCalledWith(6, 6) // start of 'line2'
    })

    it('C-e moves to end of line', () => {
      const text = 'line1\nline2'
      const textareaRef = createTextareaRef(text, 6, 6)
      const onChange = vi.fn()
      const { result } = renderHook(() =>
        useEmacsKeymap({
          textareaRef,
          value: text,
          onChange,
          onSave: vi.fn(),
        }),
      )

      const event = {
        key: 'e',
        ctrlKey: true,
        altKey: false,
        preventDefault: vi.fn(),
      } as any

      result.current.handleKeyDown(event)

      expect(textareaRef.current.setSelectionRange).toHaveBeenCalledWith(11, 11) // end of line2
    })

    it('M-f moves forward one word', () => {
      const textareaRef = createTextareaRef('hello world', 0, 0)
      const onChange = vi.fn()
      const { result } = renderHook(() =>
        useEmacsKeymap({
          textareaRef,
          value: 'hello world',
          onChange,
          onSave: vi.fn(),
        }),
      )

      const event = {
        key: 'f',
        ctrlKey: false,
        altKey: true,
        preventDefault: vi.fn(),
      } as any

      result.current.handleKeyDown(event)

      expect(event.preventDefault).toHaveBeenCalled()
      expect(textareaRef.current.setSelectionRange).toHaveBeenCalledWith(5, 5)
    })

    it('M-b moves backward one word', () => {
      const textareaRef = createTextareaRef('hello world', 11, 11)
      const onChange = vi.fn()
      const { result } = renderHook(() =>
        useEmacsKeymap({
          textareaRef,
          value: 'hello world',
          onChange,
          onSave: vi.fn(),
        }),
      )

      const event = {
        key: 'b',
        ctrlKey: false,
        altKey: true,
        preventDefault: vi.fn(),
      } as any

      result.current.handleKeyDown(event)

      expect(textareaRef.current.setSelectionRange).toHaveBeenCalledWith(6, 6)
    })
  })

  describe('deletion commands', () => {
    it('C-d deletes character at point', () => {
      const onChange = vi.fn()
      const textareaRef = createTextareaRef('hello', 1, 1)
      const { result } = renderHook(() =>
        useEmacsKeymap({
          textareaRef,
          value: 'hello',
          onChange,
          onSave: vi.fn(),
        }),
      )

      const event = {
        key: 'd',
        ctrlKey: true,
        altKey: false,
        preventDefault: vi.fn(),
      } as any

      result.current.handleKeyDown(event)

      expect(event.preventDefault).toHaveBeenCalled()
      expect(onChange).toHaveBeenCalledWith('hllo', 1)
    })

    it('C-k kills from point to end of line', () => {
      const text = 'hello\nworld'
      const onChange = vi.fn()
      const textareaRef = createTextareaRef(text, 1, 1)
      const { result } = renderHook(() =>
        useEmacsKeymap({
          textareaRef,
          value: text,
          onChange,
          onSave: vi.fn(),
        }),
      )

      const event = {
        key: 'k',
        ctrlKey: true,
        altKey: false,
        preventDefault: vi.fn(),
      } as any

      result.current.handleKeyDown(event)

      expect(event.preventDefault).toHaveBeenCalled()
      expect(onChange).toHaveBeenCalledWith('h\nworld', 1)
    })

    it('C-w kills region', () => {
      const onChange = vi.fn()
      const textareaRef = createTextareaRef('hello world', 0, 5)
      const { result } = renderHook(() =>
        useEmacsKeymap({
          textareaRef,
          value: 'hello world',
          onChange,
          onSave: vi.fn(),
        }),
      )

      const event = {
        key: 'w',
        ctrlKey: true,
        altKey: false,
        preventDefault: vi.fn(),
      } as any

      result.current.handleKeyDown(event)

      expect(event.preventDefault).toHaveBeenCalled()
      expect(onChange).toHaveBeenCalledWith(' world', 0)
    })
  })

  describe('mark and region', () => {
    it('C-space sets mark', () => {
      const onChange = vi.fn()
      const textareaRef = createTextareaRef('hello', 2, 2)
      const { result } = renderHook(() =>
        useEmacsKeymap({
          textareaRef,
          value: 'hello',
          onChange,
          onSave: vi.fn(),
        }),
      )

      const event = {
        key: ' ',
        ctrlKey: true,
        altKey: false,
        preventDefault: vi.fn(),
      } as any

      result.current.handleKeyDown(event)

      expect(event.preventDefault).toHaveBeenCalled()
    })

    it('C-g clears mark', () => {
      const onChange = vi.fn()
      const textareaRef = createTextareaRef('hello', 2, 2)
      const { result } = renderHook(() =>
        useEmacsKeymap({
          textareaRef,
          value: 'hello',
          onChange,
          onSave: vi.fn(),
        }),
      )

      // Set mark first
      const markEvent = {
        key: ' ',
        ctrlKey: true,
        altKey: false,
        preventDefault: vi.fn(),
      } as any

      result.current.handleKeyDown(markEvent)

      // Then clear it
      const quitEvent = {
        key: 'g',
        ctrlKey: true,
        altKey: false,
        preventDefault: vi.fn(),
      } as any

      result.current.handleKeyDown(quitEvent)

      expect(quitEvent.preventDefault).toHaveBeenCalled()
    })
  })

  describe('save command', () => {
    it('C-x C-s calls onSave', () => {
      const onSave = vi.fn()
      const onChange = vi.fn()
      const textareaRef = createTextareaRef('hello', 0, 0)
      const { result } = renderHook(() =>
        useEmacsKeymap({
          textareaRef,
          value: 'hello',
          onChange,
          onSave,
        }),
      )

      // Press C-x
      const ctrlXEvent = {
        key: 'x',
        ctrlKey: true,
        altKey: false,
        preventDefault: vi.fn(),
      } as any

      result.current.handleKeyDown(ctrlXEvent)

      // Press C-s
      const ctrlSEvent = {
        key: 's',
        ctrlKey: true,
        altKey: false,
        preventDefault: vi.fn(),
      } as any

      result.current.handleKeyDown(ctrlSEvent)

      expect(ctrlSEvent.preventDefault).toHaveBeenCalled()
      expect(onSave).toHaveBeenCalled()
    })
  })
})
