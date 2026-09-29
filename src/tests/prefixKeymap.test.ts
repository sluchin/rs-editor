import { describe, it, expect, vi } from 'vitest'
import { renderHook } from '@testing-library/react'
import { usePrefixKeymap } from '../lib/prefixKeymap'

/**
 * usePrefixKeymap フック のテストスイート.
 * C-x C-f (ファイルを開く) と C-x C-s (保存) のキーシーケンスをテストします.
 */
describe('usePrefixKeymap', () => {
  it('calls onFindFile on C-x C-f', () => {
    // C-x C-f キーシーケンスで onFindFile が呼ばれることを確認.
    const onFindFile = vi.fn()
    const onSaveBuffer = vi.fn()
    const { result } = renderHook(() => usePrefixKeymap({ onFindFile, onSaveBuffer }))
    const { handleKeyDown } = result.current

    // まず C-x を押す.
    const ctrlXEvent = {
      key: 'x',
      ctrlKey: true,
      altKey: false,
      metaKey: false,
      preventDefault: vi.fn(),
    } as any

    handleKeyDown(ctrlXEvent)
    expect(ctrlXEvent.preventDefault).toHaveBeenCalled()

    // 次に C-f を押す.
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
    // C-x C-s キーシーケンスで onSaveBuffer が呼ばれることを確認.
    const onFindFile = vi.fn()
    const onSaveBuffer = vi.fn()
    const { result } = renderHook(() => usePrefixKeymap({ onFindFile, onSaveBuffer }))
    const { handleKeyDown } = result.current

    // まず C-x を押す.
    const ctrlXEvent = {
      key: 'x',
      ctrlKey: true,
      altKey: false,
      metaKey: false,
      preventDefault: vi.fn(),
    } as any

    handleKeyDown(ctrlXEvent)

    // 次に C-s を押す.
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
    // C-x の後に他のキーが押された場合は何も呼ばれないことを確認.
    const onFindFile = vi.fn()
    const onSaveBuffer = vi.fn()
    const { result } = renderHook(() => usePrefixKeymap({ onFindFile, onSaveBuffer }))
    const { handleKeyDown } = result.current

    // まず C-x を押す.
    const ctrlXEvent = {
      key: 'x',
      ctrlKey: true,
      altKey: false,
      metaKey: false,
      preventDefault: vi.fn(),
    } as any

    handleKeyDown(ctrlXEvent)

    // その後、ランダムなキーを押す.
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
    // 通常のキーの場合は false を返すことを確認.
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
