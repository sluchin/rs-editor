/**
 * メジャーモード (バッファの種類). モードラインに出す名前と, シンタックスハイライトと字下げを決めます.
 */

/** メジャーモード. */
export interface MajorMode {
  /** モードラインに出す名前. */
  name: string
  /** シンタックスハイライトと字下げの言語 (無ければ, 色を付けず, Fundamental モードの字下げ). */
  language: 'scheme' | 'elisp' | null
  /** 字下げにタブを使うなら `true` (Emacs の `indent-tabs-mode`). */
  indentTabs: boolean
  /** タブの幅 (Emacs の `tab-width`). */
  tabWidth: number
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

/** ファイルの変数 (Emacs の, 1 行目の `-*- ... -*-`). */
export interface FileVariables {
  /** メジャーモード (`mode: scheme` など). */
  mode?: string
  /** 字句の束縛 (`lexical-binding: t`). */
  lexicalBinding?: boolean
  /** 字下げにタブを使うか (`indent-tabs-mode: nil`). */
  indentTabsMode?: boolean
  /** タブの幅 (`tab-width: 4`). */
  tabWidth?: number
}

/**
 * 1 行目 (`#!` で始まれば 2 行目) の `-*- ... -*-` から, ファイルの変数を読みます.
 * `-*- scheme -*-` のように, 名前だけならメジャーモードです.
 *
 * @param content - 本文.
 * @returns ファイルの変数.
 */
export function fileVariables(content: string): FileVariables {
  const [first, second = ''] = content.split('\n', 2)
  const m = /-\*-(.*?)-\*-/.exec(first.startsWith('#!') ? second : first)
  if (!m) return {}
  const body = m[1].trim()
  if (!body.includes(':')) return { mode: body }
  const vars: FileVariables = {}
  for (const pair of body.split(';')) {
    const [key, value] = pair.split(':').map((part) => part.trim())
    if (key === 'mode') vars.mode = value
    else if (key === 'lexical-binding') vars.lexicalBinding = value !== 'nil'
    else if (key === 'indent-tabs-mode') vars.indentTabsMode = value !== 'nil'
    else if (key === 'tab-width' && /^[1-9][0-9]*$/.test(value)) vars.tabWidth = Number(value)
  }
  return vars
}

/** `mode:` の値と, メジャーモード. */
const MODE_NAMES: Record<string, (typeof AUTO_MODES)[number][1] | 'fundamental'> = {
  'emacs-lisp': 'elisp',
  'lisp-data': 'lisp-data',
  scheme: 'scheme',
  fundamental: 'fundamental',
}

/** dired のバッファのメジャーモード (`C-x d`). */
export const DIRED_MODE: MajorMode = {
  name: 'Dired by name',
  language: null,
  indentTabs: true,
  tabWidth: 8,
}

/** `M-x grep` の結果のバッファのモード. */
export const GREP_MODE: MajorMode = {
  name: 'Grep',
  language: null,
  indentTabs: true,
  tabWidth: 8,
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
  // Emacs の既定は, タブで字下げし, タブの幅は 8. ファイルの変数で変えられる
  const vars: FileVariables = filePath ? fileVariables(content) : {}
  const indent = { indentTabs: vars.indentTabsMode ?? true, tabWidth: vars.tabWidth ?? 8 }
  // *scratch* は, Scheme を評価する (Emacs の lisp-interaction-mode)
  if (name === '*scratch*') return { name: 'Lisp Interaction', language: 'scheme', ...indent }
  if (name === '*Help*') return { name: 'Help', language: null, ...indent }
  const mode =
    MODE_NAMES[vars.mode ?? ''] ??
    (filePath ? AUTO_MODES.find(([re]) => re.test(filePath))?.[1] : undefined)
  if (mode === 'scheme') return { name: 'Scheme', language: 'scheme', ...indent }
  if (mode === 'lisp-data') return { name: 'Lisp-Data', language: 'elisp', ...indent }
  // Emacs と同じく, 字句の束縛なら /l, 動的な束縛なら /d を付ける
  if (mode === 'elisp') {
    return { name: vars.lexicalBinding ? 'ELisp/l' : 'ELisp/d', language: 'elisp', ...indent }
  }
  return { name: 'Fundamental', language: null, ...indent }
}
