/**
 * 押したキーの名前. 画面は, キーを Emacs の書き方 (`C-x`, `M-f`, `RET`) にして, エディタ (Rust) に送ります.
 */

/** 修飾キー (これだけを押しても, キーとして送らない). */
export const MODIFIER_KEYS = ['Control', 'Alt', 'Shift', 'Meta']

/** CodeMirror のキーの名前と, Emacs のキーの名前. */
const EMACS_KEY_NAMES: Record<string, string> = {
  Space: 'SPC',
  ' ': 'SPC',
  Enter: 'RET',
  Tab: 'TAB',
  Backspace: 'DEL',
  Escape: 'ESC',
  Delete: '<delete>',
  Insert: '<insert>',
  Home: '<home>',
  End: '<end>',
  PageUp: '<prior>',
  PageDown: '<next>',
  ArrowLeft: '<left>',
  ArrowRight: '<right>',
  ArrowUp: '<up>',
  ArrowDown: '<down>',
}

/** CodeMirror の修飾キーと, Emacs の修飾キー. */
const EMACS_MODIFIERS: Record<string, string> = {
  Ctrl: 'C-',
  Alt: 'M-',
  Shift: 'S-',
  Meta: 's-',
}

/**
 * CodeMirror のキーの書き方 (`Ctrl-x Ctrl-f`) を, Emacs の書き方 (`C-x C-f`) にします.
 *
 * @param keys - CodeMirror の書き方 (空白で区切ると, 続けて押すキー).
 * @returns Emacs の書き方.
 */
export function toEmacsKey(keys: string): string {
  return keys.split(' ').map(strokeName).join(' ')
}

/**
 * 1 つのキー (CodeMirror の書き方. `Ctrl-x`, `Ctrl- `) を, Emacs の書き方にします.
 *
 * @param stroke - キー.
 * @returns Emacs の書き方.
 */
function strokeName(stroke: string): string {
  const parts = stroke.split(/-(?!$)/)
  const key = parts.pop() as string
  const modifiers = parts.map((m) => EMACS_MODIFIERS[m] ?? `${m}-`).join('')
  const name = EMACS_KEY_NAMES[key] ?? (/^F\d+$/.test(key) ? `<${key.toLowerCase()}>` : key)
  return modifiers + name
}

/**
 * 押したキーを, CodeMirror の書き方にします (修飾キーを付ける).
 *
 * Shift は, Emacs と同じく, 名前のあるキー (`S-DEL`, `C-S-<up>`) と, Ctrl と一緒の英字 (`C-S-v`) にだけ付ける.
 * ほかの文字は, Shift で変わった文字 (`A`, `?`) にする.
 *
 * @param event - キーの事象.
 * @returns CodeMirror の書き方.
 */
export function eventKeyName(event: KeyboardEvent): string {
  const letter = event.ctrlKey && /^[A-Z]$/.test(event.key)
  const shift = event.shiftKey && (event.key.length > 1 || letter)
  const modifiers = [
    event.ctrlKey && 'Ctrl-',
    event.altKey && 'Alt-',
    shift && 'Shift-',
    event.metaKey && 'Meta-',
  ]
  return modifiers.filter(Boolean).join('') + (letter ? event.key.toLowerCase() : event.key)
}

/**
 * 押したキーを, Emacs の書き方にします.
 *
 * @param event - キーの事象.
 * @returns Emacs の書き方 (`C-x`, `a`, `RET`). 修飾キーだけなら `null`.
 */
export function emacsKey(event: KeyboardEvent): string | null {
  if (MODIFIER_KEYS.includes(event.key)) return null
  return strokeName(eventKeyName(event))
}
