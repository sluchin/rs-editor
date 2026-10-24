/**
 * メジャーモード (バッファの種類). モードラインに出す名前と, シンタックスハイライトを決めます.
 */

/** メジャーモード. */
export interface MajorMode {
  /** モードラインに出す名前. */
  name: string
  /** シンタックスハイライトの言語 (無ければ, 色を付けない). */
  language: 'scheme' | 'elisp' | null
}

/**
 * ファイルの名前とメジャーモード (Emacs 30 の `auto-mode-alist` の, Lisp の分). 先に合ったものを使います.
 */
const AUTO_MODES: [RegExp, 'lisp-data' | 'elisp' | 'scheme'][] = [
  [/\.dir-locals(?:-2)?\.el$/, 'lisp-data'],
  [/\.eld$/, 'lisp-data'],
  [/eww-bookmarks$/, 'lisp-data'],
  [/tramp$/, 'lisp-data'],
  [/\/archive-contents$/, 'lisp-data'],
  [/places$/, 'lisp-data'],
  [/\.emacs-places$/, 'lisp-data'],
  [/\.el$/, 'elisp'],
  [/Project\.ede$/, 'elisp'],
  [/(?:\.(?:scm|sls|sld|stk|ss|sch)|\/\.guile)$/, 'scheme'],
  [/\.scm\.[0-9]*$/, 'scheme'],
  [/\.oak$/, 'scheme'],
  [/[:/\\]\..*(?:emacs|gnus|viper)$/, 'elisp'],
  [/[:/]_emacs$/, 'elisp'],
]

/**
 * Emacs Lisp のファイルが, 1 行目で `lexical-binding: t` にしているかを返します.
 *
 * @param content - 本文.
 * @returns していれば `true`.
 */
function lexicalBinding(content: string): boolean {
  const firstLine = content.split('\n', 1)[0]
  return /-\*-.*\blexical-binding:\s*t\b/.test(firstLine)
}

/**
 * バッファのメジャーモードを返します.
 *
 * @param name - バッファの名前.
 * @param filePath - バッファのファイル (無ければ `null`).
 * @param content - バッファの本文.
 * @returns メジャーモード.
 */
export function majorMode(name: string, filePath: string | null, content: string): MajorMode {
  // *scratch* は, Scheme を評価する (Emacs の lisp-interaction-mode)
  if (name === '*scratch*') return { name: 'Lisp Interaction', language: 'scheme' }
  if (name === '*Help*') return { name: 'Help', language: null }
  const mode = filePath ? AUTO_MODES.find(([re]) => re.test(filePath))?.[1] : undefined
  if (mode === 'scheme') return { name: 'Scheme', language: 'scheme' }
  if (mode === 'lisp-data') return { name: 'Lisp-Data', language: 'elisp' }
  // Emacs と同じく, 字句の束縛なら /l, 動的な束縛なら /d を付ける
  if (mode === 'elisp') {
    return { name: lexicalBinding(content) ? 'ELisp/l' : 'ELisp/d', language: 'elisp' }
  }
  return { name: 'Fundamental', language: null }
}
