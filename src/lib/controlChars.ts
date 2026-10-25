/**
 * 制御文字の表示 (Emacs と同じく, `C-a` (1) は `^A`, DEL (127) は `^?` と出す).
 * タブと改行は, 除きます.
 */

import { highlightSpecialChars } from '@codemirror/view'

/** `^` を付けて出す文字. */
// 制御文字を探すための正規表現なので, 制御文字を書く
// eslint-disable-next-line no-control-regex
const CONTROL_CHARS = /[\u0000-\u0008\u000b-\u001f\u007f]/g

/**
 * 制御文字の表示の要素を作ります.
 *
 * @param code - 文字のコード.
 * @returns 要素 (`^A` など).
 */
export function renderControlChar(code: number): HTMLElement {
  const span = document.createElement('span')
  span.className = 'cm-control-char'
  span.textContent = `^${String.fromCharCode(code ^ 64)}`
  return span
}

/**
 * 制御文字を `^A` のように出す拡張を返します.
 *
 * @returns 拡張.
 */
export function controlChars() {
  return highlightSpecialChars({
    specialChars: CONTROL_CHARS,
    addSpecialChars: null,
    render: renderControlChar,
  })
}
