import { describe, it, expect, vi } from 'vitest'
import { renderHook } from '@testing-library/react'
import { usePrefixKeymap } from '../lib/prefixKeymap'
import type { KeyboardEvent } from 'react'

/**
 * テスト用の C-x のキーマップを作ります.
 *
 * @param withCallbacks - 任意のコールバックを渡すか.
 * @returns キーを押す関数と, モック.
 */
function setup(withCallbacks = true) {
  const callbacks = {
    onFindFile: vi.fn(),
    onSaveBuffer: vi.fn(),
    onRedo: vi.fn(),
    onSwitchBuffer: vi.fn(),
    onKillBuffer: vi.fn(),
    onListBuffers: vi.fn(),
    onEvalLastSexp: vi.fn(),
  }
  const { result } = renderHook(() =>
    usePrefixKeymap(
      withCallbacks
        ? callbacks
        : { onFindFile: callbacks.onFindFile, onSaveBuffer: callbacks.onSaveBuffer },
    ),
  )
  const target = { value: 'abc', setSelectionRange: vi.fn() }
  const press = (key: string, ctrlKey: boolean) =>
    result.current.handleKeyDown({
      key,
      ctrlKey,
      preventDefault: vi.fn(),
      currentTarget: target,
    } as unknown as KeyboardEvent<HTMLTextAreaElement>)
  /** C-x に続けてキーを押す. */
  const afterCtrlX = (key: string, ctrlKey: boolean) => {
    press('x', true)
    return press(key, ctrlKey)
  }
  return { callbacks, target, afterCtrlX }
}

describe('usePrefixKeymap (全てのコマンド)', () => {
  it('C-x に続くキーで, それぞれのコマンドを呼ぶ', () => {
    const { callbacks, afterCtrlX } = setup()

    expect(afterCtrlX('/', true)).toBe(true)
    expect(afterCtrlX('e', true)).toBe(true)
    expect(afterCtrlX('b', true)).toBe(true)
    expect(afterCtrlX('b', false)).toBe(true)
    expect(afterCtrlX('k', false)).toBe(true)

    expect(callbacks.onRedo).toHaveBeenCalled()
    expect(callbacks.onEvalLastSexp).toHaveBeenCalled()
    expect(callbacks.onListBuffers).toHaveBeenCalled()
    expect(callbacks.onSwitchBuffer).toHaveBeenCalled()
    expect(callbacks.onKillBuffer).toHaveBeenCalled()
  })

  it('C-x h は, 全体を選択する', () => {
    const { target, afterCtrlX } = setup()

    expect(afterCtrlX('h', false)).toBe(true)
    expect(target.setSelectionRange).toHaveBeenCalledWith(0, 3)
  })

  it('任意のコールバックがなくても, キーは処理する', () => {
    const { afterCtrlX } = setup(false)

    for (const [key, ctrlKey] of [
      ['/', true],
      ['e', true],
      ['b', true],
      ['b', false],
      ['k', false],
    ] as const) {
      expect(afterCtrlX(key, ctrlKey)).toBe(true)
    }
  })
})
