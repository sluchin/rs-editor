import { describe, it, expect, vi } from 'vitest'
import { renderHook } from '@testing-library/react'
import { useEmacsKeymap } from '../lib/emacsKeymap'
import type { KeyboardEvent } from 'react'

/** キーの修飾. */
interface Mods {
  ctrl?: boolean
  alt?: boolean
}

/**
 * テスト用のエディタ (テキストエリアの代わりと, フック) を作ります.
 *
 * @param value - 文字列.
 * @param start - 選択の開始位置.
 * @param end - 選択の終了位置 (既定は開始位置).
 * @param withCallbacks - 任意のコールバックを渡すか.
 * @returns キーを押す関数と, モック.
 */
function setup(value: string, start: number, end = start, withCallbacks = true) {
  const el = {
    value,
    selectionStart: start,
    selectionEnd: end,
    setSelectionRange: vi.fn(),
  } as unknown as HTMLTextAreaElement
  const textareaRef = { current: el }
  const onChange = vi.fn()
  const callbacks = {
    onIsearchForward: vi.fn(),
    onIsearchBackward: vi.fn(),
    onExecuteCommand: vi.fn(),
    onQuit: vi.fn(),
    onUndo: vi.fn(),
    onPushUndo: vi.fn(),
    onEvalExpression: vi.fn(),
  }
  const { result } = renderHook(() =>
    useEmacsKeymap({
      textareaRef,
      value,
      onChange,
      ...(withCallbacks ? callbacks : {}),
    }),
  )
  const press = (key: string, mods: Mods = {}) => {
    const event = {
      key,
      ctrlKey: mods.ctrl ?? false,
      altKey: mods.alt ?? false,
      preventDefault: vi.fn(),
    } as unknown as KeyboardEvent<HTMLTextAreaElement>
    result.current.handleKeyDown(event)
    return event
  }
  return { el, press, onChange, callbacks }
}

const ctrl = { ctrl: true }
const meta = { alt: true }
const ctrlMeta = { ctrl: true, alt: true }

describe('useEmacsKeymap (全てのコマンド)', () => {
  it('テキストエリアがなければ, 何もしない', () => {
    const onChange = vi.fn()
    const { result } = renderHook(() =>
      useEmacsKeymap({ textareaRef: { current: null }, value: 'abc', onChange }),
    )
    const event = {
      key: 'f',
      ctrlKey: true,
      altKey: false,
      preventDefault: vi.fn(),
    } as unknown as KeyboardEvent<HTMLTextAreaElement>

    result.current.handleKeyDown(event)

    expect(event.preventDefault).not.toHaveBeenCalled()
  })

  it('C-f と C-b は, 文字列の端で止まる', () => {
    const atEnd = setup('ab', 2)
    atEnd.press('f', ctrl)
    expect(atEnd.el.setSelectionRange).toHaveBeenCalledWith(2, 2)

    const atStart = setup('ab', 0)
    atStart.press('b', ctrl)
    expect(atStart.el.setSelectionRange).toHaveBeenCalledWith(0, 0)
  })

  it('C-n は次の行へ, 最後の行では文字列の終わりへ', () => {
    const first = setup('ab\ncd', 1)
    first.press('n', ctrl)
    expect(first.el.setSelectionRange).toHaveBeenCalledWith(3, 3)

    const last = setup('ab\ncd', 4)
    last.press('n', ctrl)
    expect(last.el.setSelectionRange).toHaveBeenCalledWith(5, 5)
  })

  it('C-p は前の行の同じ列へ (短い行では行末), 最初の行では動かない', () => {
    const second = setup('abc\nde', 5)
    second.press('p', ctrl)
    expect(second.el.setSelectionRange).toHaveBeenCalledWith(1, 1)

    const shorter = setup('a\nbcd', 5)
    shorter.press('p', ctrl)
    expect(shorter.el.setSelectionRange).toHaveBeenCalledWith(1, 1)

    const first = setup('abc\nde', 1)
    first.press('p', ctrl)
    expect(first.el.setSelectionRange).not.toHaveBeenCalled()
  })

  it('C-d は文字を消し, 文字列の終わりでは何もしない', () => {
    const middle = setup('abc', 1)
    middle.press('d', ctrl)
    expect(middle.onChange).toHaveBeenCalledWith('ac', 1)

    const end = setup('abc', 3)
    end.press('d', ctrl)
    expect(end.onChange).not.toHaveBeenCalled()
  })

  it('C-k は行末まで消し, 行末では改行を消して, C-y で戻せる', () => {
    const editor = setup('ab\ncd', 2)
    editor.press('k', ctrl)
    expect(editor.onChange).toHaveBeenLastCalledWith('abcd', 2)
    expect(editor.callbacks.onPushUndo).toHaveBeenCalledWith('ab\ncd', 2)

    editor.press('y', ctrl)
    expect(editor.onChange).toHaveBeenLastCalledWith('ab\n\ncd', 3)

    const lastLine = setup('ab', 2)
    lastLine.press('k', ctrl)
    expect(lastLine.onChange).toHaveBeenCalledWith('ab', 2)
  })

  it('キルリングは 50 個まで (C-k, C-w, M-w, C-M-k)', () => {
    const editor = setup('abc', 0, 1)
    for (let i = 0; i < 51; i++) {
      editor.press('k', ctrl)
      editor.press('w', ctrl)
      editor.press('w', meta)
      editor.press('k', ctrlMeta)
    }
    editor.press('y', ctrl)
    expect(editor.onChange).toHaveBeenCalled()
  })

  it('C-y はキルリングが空なら, 何も挿入しない', () => {
    const editor = setup('ab', 1)
    editor.press('y', ctrl)
    expect(editor.onChange).toHaveBeenCalledWith('ab', 1)
  })

  it('C-w は選択を消し (逆向きの選択も), 選択がなければ消さない', () => {
    const forward = setup('abcd', 1, 3)
    forward.press('w', ctrl)
    expect(forward.onChange).toHaveBeenCalledWith('ad', 1)

    const backward = setup('abcd', 3, 1)
    backward.press('w', ctrl)
    expect(backward.onChange).toHaveBeenCalledWith('ad', 1)

    const empty = setup('abcd', 2)
    empty.press('w', ctrl)
    expect(empty.onChange).not.toHaveBeenCalled()
  })

  it('C-/ は元に戻す', () => {
    const editor = setup('ab', 1)
    editor.press('/', ctrl)
    expect(editor.callbacks.onPushUndo).toHaveBeenCalledWith('ab', 1)
    expect(editor.callbacks.onUndo).toHaveBeenCalled()
  })

  it('C-t は文字を入れ替える (先頭, 末尾, 途中)', () => {
    const middle = setup('abc', 1)
    middle.press('t', ctrl)
    expect(middle.onChange).toHaveBeenCalledWith('bac', 2)

    const start = setup('abc', 0)
    start.press('t', ctrl)
    expect(start.onChange).toHaveBeenCalledWith('bac', 1)

    const end = setup('abc', 3)
    end.press('t', ctrl)
    expect(end.onChange).toHaveBeenCalledWith('acb', 3)
  })

  it('C-t は 2 文字未満か, 位置が範囲の外なら, 何もしない', () => {
    const short = setup('a', 1)
    short.press('t', ctrl)
    expect(short.onChange).not.toHaveBeenCalled()

    const outside = setup('ab', 4)
    outside.press('t', ctrl)
    expect(outside.onChange).not.toHaveBeenCalled()
  })

  it('C-space でマークを置き, C-g で外す', () => {
    const editor = setup('ab', 1)
    expect(editor.press(' ', ctrl).preventDefault).toHaveBeenCalled()
    editor.press('g', ctrl)
    expect(editor.callbacks.onQuit).toHaveBeenCalled()
  })

  it('割り当てのないキーは, そのまま通す', () => {
    const editor = setup('ab', 1)
    expect(editor.press('z', ctrl).preventDefault).not.toHaveBeenCalled()
    expect(editor.press('z', ctrlMeta).preventDefault).not.toHaveBeenCalled()
    expect(editor.press('z', meta).preventDefault).not.toHaveBeenCalled()
    expect(editor.press('z').preventDefault).not.toHaveBeenCalled()
  })

  it('C-M-f, C-M-b は S 式を移動し, C-M-k は S 式を消す', () => {
    const editor = setup('(a b) c', 0)
    editor.press('f', ctrlMeta)
    expect(editor.el.setSelectionRange).toHaveBeenCalledWith(5, 5)

    const back = setup('(a b) c', 5)
    back.press('b', ctrlMeta)
    expect(back.el.setSelectionRange).toHaveBeenCalledWith(0, 0)

    const kill = setup('(a b) c', 0)
    kill.press('k', ctrlMeta)
    expect(kill.onChange).toHaveBeenCalledWith(' c', 0)
    kill.press('y', ctrl)
    expect(kill.onChange).toHaveBeenLastCalledWith('(a b)(a b) c', 5)
  })

  it('M-f, M-b は単語を移動する (記号を飛ばす)', () => {
    const forward = setup('  ab  cd', 0)
    forward.press('f', meta)
    expect(forward.el.setSelectionRange).toHaveBeenCalledWith(4, 4)

    const backward = setup('ab  cd  ', 8)
    backward.press('b', meta)
    expect(backward.el.setSelectionRange).toHaveBeenCalledWith(4, 4)
  })

  it('M-w は選択をコピーし (逆向きも), M-y で前のものと入れ替える', () => {
    const editor = setup('abcd', 1, 2)
    editor.press('w', meta)
    const reversed = setup('abcd', 3, 2)
    reversed.press('w', meta)
    editor.press('k', ctrl)
    editor.press('y', meta)
    // キルリングは ['bcd', 'b'] で, M-y は 1 つ前の 'b' を, 選択 (1, 2) に入れる
    expect(editor.onChange).toHaveBeenLastCalledWith('abcd', 2)
    expect(reversed.onChange).not.toHaveBeenCalled()

    const empty = setup('abcd', 2)
    empty.press('w', meta)
    empty.press('y', meta)
    expect(empty.onChange).not.toHaveBeenCalled()
  })

  it('M-<, M-> は先頭と末尾へ, M-x と M-: はコマンドを呼ぶ', () => {
    const editor = setup('abc', 1)
    editor.press('<', meta)
    expect(editor.el.setSelectionRange).toHaveBeenCalledWith(0, 0)
    editor.press('>', meta)
    expect(editor.el.setSelectionRange).toHaveBeenCalledWith(3, 3)
    editor.press('x', meta)
    expect(editor.callbacks.onExecuteCommand).toHaveBeenCalled()
    editor.press(':', meta)
    expect(editor.callbacks.onEvalExpression).toHaveBeenCalled()
  })

  it('任意のコールバックがなくても, 例外にならない', () => {
    const editor = setup('abc', 1, 2, false)
    for (const [key, mods] of [
      ['k', ctrl],
      ['w', ctrl],
      ['/', ctrl],
      ['t', ctrl],
      ['s', ctrl],
      ['r', ctrl],
      ['g', ctrl],
      ['k', ctrlMeta],
      ['x', meta],
      [':', meta],
    ] as const) {
      expect(() => editor.press(key, mods)).not.toThrow()
    }
  })
})
