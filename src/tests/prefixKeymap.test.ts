import { describe, it, expect, vi } from 'vitest'
import { renderHook } from '@testing-library/react'
import { usePrefixKeymap } from '../lib/prefixKeymap'

describe('usePrefixKeymap', () => {
  it('calls onFindFile on C-x C-f', () => {
    const onFindFile = vi.fn()
    const onSaveBuffer = vi.fn()
    const { result } = renderHook(() => usePrefixKeymap({ onFindFile, onSaveBuffer }))
    const { handleKeyDown } = result.current

    // First press C-x
    const ctrlXEvent = {
      key: 'x',
      ctrlKey: true,
      altKey: false,
      metaKey: false,
      preventDefault: vi.fn(),
    } as any

    handleKeyDown(ctrlXEvent)
    expect(ctrlXEvent.preventDefault).toHaveBeenCalled()

    // Then press C-f
    const ctrlFEvent = {
      key: 'f',
      ctrlKey: true,
      altKey: false,
      metaKey: false,
      preventDefault: vi.fn(),
    } as any

    handleKeyDown(ctrlFEvent)
    expect(ctrlFEvent.preventDefault).toHaveBeenCalled()
    expect(onFindFile).toHaveBeenCalled()
    expect(onSaveBuffer).not.toHaveBeenCalled()
  })

  it('calls onSaveBuffer on C-x C-s', () => {
    const onFindFile = vi.fn()
    const onSaveBuffer = vi.fn()
    const { result } = renderHook(() => usePrefixKeymap({ onFindFile, onSaveBuffer }))
    const { handleKeyDown } = result.current

    // First press C-x
    const ctrlXEvent = {
      key: 'x',
      ctrlKey: true,
      altKey: false,
      metaKey: false,
      preventDefault: vi.fn(),
    } as any

    handleKeyDown(ctrlXEvent)

    // Then press C-s
    const ctrlSEvent = {
      key: 's',
      ctrlKey: true,
      altKey: false,
      metaKey: false,
      preventDefault: vi.fn(),
    } as any

    handleKeyDown(ctrlSEvent)
    expect(ctrlSEvent.preventDefault).toHaveBeenCalled()
    expect(onSaveBuffer).toHaveBeenCalled()
    expect(onFindFile).not.toHaveBeenCalled()
  })

  it('ignores other keys after C-x', () => {
    const onFindFile = vi.fn()
    const onSaveBuffer = vi.fn()
    const { result } = renderHook(() => usePrefixKeymap({ onFindFile, onSaveBuffer }))
    const { handleKeyDown } = result.current

    // First press C-x
    const ctrlXEvent = {
      key: 'x',
      ctrlKey: true,
      altKey: false,
      metaKey: false,
      preventDefault: vi.fn(),
    } as any

    handleKeyDown(ctrlXEvent)

    // Then press some random key
    const randomEvent = {
      key: 'a',
      ctrlKey: false,
      altKey: false,
      metaKey: false,
      preventDefault: vi.fn(),
    } as any

    handleKeyDown(randomEvent)
    expect(onFindFile).not.toHaveBeenCalled()
    expect(onSaveBuffer).not.toHaveBeenCalled()
  })

  it('returns false for regular keys', () => {
    const { result } = renderHook(() =>
      usePrefixKeymap({
        onFindFile: vi.fn(),
        onSaveBuffer: vi.fn(),
      }),
    )
    const { handleKeyDown } = result.current

    const event = {
      key: 'a',
      ctrlKey: false,
      altKey: false,
      metaKey: false,
      preventDefault: vi.fn(),
    } as any

    const keyResult = handleKeyDown(event)
    expect(keyResult).toBe(false)
  })
})
