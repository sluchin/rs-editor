/**
 * 空白と空の行を消すコマンド (Emacs 30 の `just-one-space`, `delete-horizontal-space`,
 * `delete-indentation`, `delete-blank-lines`).
 */

import { EditorSelection, type EditorState, type TransactionSpec } from '@codemirror/state'
import { indentConfig } from './lispIndent'

/** 空白だけの行. */
const BLANK = /^[ \t]*$/

/** 開き括弧 (Emacs の構文表の `(`. 全角の括弧も含む. Emacs Lisp では, `{` を除く). */
const OPEN = '([{（「『【〔［｛〈《'

/** 閉じ括弧 (Emacs の構文表の `)`. Emacs Lisp では, `}` を除く). */
const CLOSE = ')]}）」』】〕］｝〉》'

/** モードごとの, 式の前に付ける文字 (Emacs の構文表の `'`). */
const PREFIX: Record<string, string> = { scheme: "#',@`", elisp: "#',`" }

/**
 * カーソルの前後の, 空白とタブの範囲を返します.
 *
 * @param state - エディタの状態.
 * @param pos - カーソルの位置.
 * @returns 範囲.
 */
function spacesAround(state: EditorState, pos: number): { from: number; to: number } {
  const line = state.doc.lineAt(pos)
  let from = pos - line.from
  let to = from
  while (from > 0 && /[ \t]/.test(line.text[from - 1])) from--
  while (to < line.length && /[ \t]/.test(line.text[to])) to++
  return { from: line.from + from, to: line.from + to }
}

/**
 * カーソルの前後の空白とタブを, 空白 1 つにします (Emacs の `just-one-space`. `M-SPC`).
 *
 * @param state - エディタの状態.
 * @param pos - カーソルの位置.
 * @returns 変更とカーソル.
 */
export function justOneSpace(state: EditorState, pos: number): TransactionSpec {
  const { from, to } = spacesAround(state, pos)
  return {
    changes: { from, to, insert: ' ' },
    selection: EditorSelection.cursor(from + 1),
  }
}

/**
 * カーソルの前後の空白とタブを消します (Emacs の `delete-horizontal-space`. `M-\`).
 *
 * @param state - エディタの状態.
 * @param pos - カーソルの位置.
 * @returns 変更とカーソル.
 */
export function deleteHorizontalSpace(state: EditorState, pos: number): TransactionSpec {
  const { from, to } = spacesAround(state, pos)
  return { changes: { from, to }, selection: EditorSelection.cursor(from) }
}

/**
 * カーソルの行を, 前の行につなげます (Emacs の `delete-indentation`. `M-^`).
 * つないだ所の空白は 1 つにします. 閉じ括弧の前と, 開き括弧 (Lisp では `'` なども) の後と,
 * 空の行とつないだときは, 空白を入れません (Emacs の `fixup-whitespace`).
 *
 * @param state - エディタの状態.
 * @param pos - カーソルの位置.
 * @returns 変更とカーソル (つないだ所). 先頭の行なら, 行の始めへ動くだけ.
 */
export function deleteIndentation(state: EditorState, pos: number): TransactionSpec {
  const { doc } = state
  const line = doc.lineAt(pos)
  if (line.number === 1) return { selection: EditorSelection.cursor(line.from) }
  const previous = doc.line(line.number - 1)
  const code = previous.text.replace(/[ \t]+$/, '')
  const rest = line.text.replace(/^[ \t]+/, '')
  const from = previous.from + code.length
  const to = line.to - rest.length
  const { dialect } = state.facet(indentConfig)
  const before = code.slice(-1)
  const after = rest.slice(0, 1)
  const tight =
    code === '' ||
    rest === '' ||
    (CLOSE.includes(after) && !(dialect === 'elisp' && after === '}')) ||
    (OPEN.includes(before) && !(dialect === 'elisp' && before === '{')) ||
    (dialect !== null && PREFIX[dialect].includes(before))
  const insert = tight ? '' : ' '
  return { changes: { from, to, insert }, selection: EditorSelection.cursor(from) }
}

/**
 * 空の行を消します (Emacs の `delete-blank-lines`. `C-x C-o`).
 *
 * - 空の行が続く所では, 1 行だけを残します.
 * - 1 行だけの空の行は, 消します.
 * - 空でない行では, 後ろに続く空の行を消します.
 *
 * 空白とタブだけの行も, 空の行とします.
 *
 * @param state - エディタの状態.
 * @param pos - カーソルの位置.
 * @returns 変更とカーソル. 消すものが無ければ `null`.
 */
export function deleteBlankLines(state: EditorState, pos: number): TransactionSpec | null {
  const { doc } = state
  const blank = (n: number) => n >= 1 && n <= doc.lines && BLANK.test(doc.line(n).text)
  const current = doc.lineAt(pos)
  if (!blank(current.number)) {
    let last = current.number
    while (blank(last + 1)) last++
    if (last === current.number) return null
    // 後ろの空でない行の始めまで (最後の行まで空なら, 文書の終わりまで) を消す
    const from = doc.line(current.number + 1).from
    const to = last < doc.lines ? doc.line(last + 1).from : doc.length
    return { changes: { from, to }, selection: EditorSelection.cursor(pos) }
  }
  let first = current.number
  while (blank(first - 1)) first--
  let last = current.number
  while (blank(last + 1)) last++
  const from = doc.line(first).from
  if (first === last) {
    // 1 行だけなら, 行を消す (最後の行なら, 空白だけを消す)
    const to = last < doc.lines ? doc.line(last + 1).from : doc.line(last).to
    return { changes: { from, to }, selection: EditorSelection.cursor(from) }
  }
  // 続く空の行は, 空の 1 行にする
  return { changes: { from, to: doc.line(last).to }, selection: EditorSelection.cursor(from) }
}
