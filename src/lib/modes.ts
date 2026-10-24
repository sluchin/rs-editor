/**
 * メジャーモード (バッファの種類). モードラインに出す名前と, シンタックスハイライトを決めます.
 */

/** メジャーモード. */
export interface MajorMode {
  /** モードラインに出す名前. */
  name: string
  /** シンタックスハイライトの言語 (無ければ, 色を付けない). */
  language: 'scheme' | null
}

/** Scheme のファイル (Emacs の `auto-mode-alist` と同じ拡張子). */
const SCHEME_FILE = /\.(scm|ss|sls|sld|stk|sch)$/

/**
 * バッファのメジャーモードを返します.
 *
 * @param name - バッファの名前.
 * @param filePath - バッファのファイル (無ければ `null`).
 * @returns メジャーモード.
 */
export function majorMode(name: string, filePath: string | null): MajorMode {
  // *scratch* は, Scheme を評価する (Emacs の lisp-interaction-mode)
  if (name === '*scratch*') return { name: 'Lisp Interaction', language: 'scheme' }
  if (name === '*Help*') return { name: 'Help', language: null }
  if (filePath && SCHEME_FILE.test(filePath)) return { name: 'Scheme', language: 'scheme' }
  return { name: 'Fundamental', language: null }
}
