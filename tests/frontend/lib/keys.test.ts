import { describe, it, expect } from 'vitest'
import { emacsKey, eventKeyName, toEmacsKey } from '@/lib/keys'

/**
 * キーの事象を作ります.
 *
 * @param key - キー.
 * @param init - 修飾キー.
 * @returns 事象.
 */
const press = (key: string, init: KeyboardEventInit = {}) =>
  new KeyboardEvent('keydown', { key, ...init })

describe('toEmacsKey', () => {
  it('CodeMirror の書き方を, Emacs の書き方にすること', () => {
    expect(toEmacsKey('Ctrl-x Ctrl-f')).toBe('C-x C-f')
    expect(toEmacsKey('Alt-Shift-a')).toBe('M-S-a')
    expect(toEmacsKey('Meta-x')).toBe('s-x')
    expect(toEmacsKey('Hyper-x')).toBe('Hyper-x')
    expect(toEmacsKey('Enter')).toBe('RET')
    expect(toEmacsKey('F8')).toBe('<f8>')
    expect(toEmacsKey('Ctrl--')).toBe('C--')
  })
})

describe('eventKeyName と emacsKey', () => {
  it('押したキーを, 修飾キーを付けて, Emacs の書き方にすること', () => {
    expect(eventKeyName(press('f', { ctrlKey: true, altKey: true, metaKey: true }))).toBe(
      'Ctrl-Alt-Meta-f',
    )
    expect(emacsKey(press('a'))).toBe('a')
    expect(emacsKey(press('A', { shiftKey: true }))).toBe('A')
    expect(emacsKey(press(' '))).toBe('SPC')
    expect(emacsKey(press(' ', { ctrlKey: true }))).toBe('C-SPC')
    expect(emacsKey(press('Backspace'))).toBe('DEL')
    expect(emacsKey(press('ArrowLeft'))).toBe('<left>')
    expect(emacsKey(press('x', { altKey: true }))).toBe('M-x')
    expect(emacsKey(press('/', { ctrlKey: true }))).toBe('C-/')
    // 修飾キーだけなら, 送らない
    expect(emacsKey(press('Control', { ctrlKey: true }))).toBeNull()
    expect(emacsKey(press('Shift'))).toBeNull()
  })
})
