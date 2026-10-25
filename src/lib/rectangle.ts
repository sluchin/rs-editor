/**
 * 矩形 (Emacs 30 の rect.el. `C-x r k`, `C-x r y`, `C-x r t`). マークとカーソルを角とする, 桁の範囲です.
 *
 * 桁は, Emacs と同じく, タブを `tab-width` の倍数まで, 全角の文字を 2 桁と数えます.
 * 桁がタブの途中なら, タブを分けます (`move-to-column` の `coerce`). 全角の文字の途中なら, 文字の後ろにします.
 */

import {
  type ChangeSpec,
  EditorSelection,
  type EditorState,
  type TransactionSpec,
} from '@codemirror/state'
import { type IndentConfig, charWidth, columnAt, indentConfig, whitespace } from './lispIndent'

/** 行の中の位置 ({@link moveToColumn} の結果). */
interface Column {
  /** 行 (タブを分けたり, 空白で埋めたりしたもの). */
  text: string
  /** 位置 (行の始めから). */
  index: number
  /** 位置の桁. */
  column: number
}

/**
 * 行の, 桁の位置へ動きます (Emacs の `move-to-column`).
 *
 * @param text - 行.
 * @param col - 桁.
 * @param force - 桁がタブの途中なら, タブを分けるか (`coerce`). `t` なら, 行が短ければ, 桁まで埋める.
 * @param config - 字下げの設定 (タブを使うなら, タブの前に空白を入れて分け, 使わないなら, タブを空白にする).
 * @returns 位置.
 */
function moveToColumn(
  text: string,
  col: number,
  force: 'coerce' | 't' | null,
  config: IndentConfig,
): Column {
  let index = 0
  let column = 0
  while (index < text.length && column < col) {
    const code = text.codePointAt(index) as number
    const size = code > 0xffff ? 2 : 1
    const next = code === 9 ? columnAt(text, index + 1, config.tabWidth) : column + charWidth(code)
    if (next > col && code === 9 && force) {
      if (config.indentTabs) {
        // タブの前に, 桁までの空白を入れる (タブは, その後ろから始まる)
        text = text.slice(0, index) + ' '.repeat(col - column) + text.slice(index)
        return { text, index: index + col - column, column: col }
      }
      text = text.slice(0, index) + ' '.repeat(next - column) + text.slice(index + 1)
      continue
    }
    index += size
    column = next
  }
  if (column < col && force === 't') {
    text += whitespace(column, col, config)
    return { text, index: text.length, column: col }
  }
  return { text, index, column }
}

/** 矩形の行と, 桁. */
interface Rectangle {
  /** 始めの行 (1 から). */
  first: number
  /** 終わりの行. */
  last: number
  /** 左の桁. */
  left: number
  /** 右の桁. */
  right: number
}

/**
 * マークとカーソルを角とする矩形を返します.
 *
 * @param state - エディタの状態.
 * @param mark - マーク.
 * @param head - カーソル.
 * @returns 矩形.
 */
function rectangleOf(state: EditorState, mark: number, head: number): Rectangle {
  const { doc } = state
  const { tabWidth } = state.facet(indentConfig)
  const start = doc.lineAt(Math.min(mark, head))
  const end = doc.lineAt(Math.max(mark, head))
  const startCol = columnAt(start.text, Math.min(mark, head) - start.from, tabWidth)
  const endCol = columnAt(end.text, Math.max(mark, head) - end.from, tabWidth)
  return {
    first: start.number,
    last: end.number,
    left: Math.min(startCol, endCol),
    right: Math.max(startCol, endCol),
  }
}

/**
 * 矩形を消して, その中身を返します (Emacs の `kill-rectangle`. `C-x r k`).
 * 行が短くて届かない所は, 空白とします. 消すときは, タブを使いません.
 *
 * @param state - エディタの状態.
 * @param mark - マーク.
 * @param head - カーソル.
 * @returns 変更とカーソル (カーソルの行の, 矩形の左), 消した中身 (行ごと).
 */
export function killRectangle(
  state: EditorState,
  mark: number,
  head: number,
): { spec: TransactionSpec; killed: string[] } {
  const { doc } = state
  const config = { ...state.facet(indentConfig), indentTabs: false }
  const { first, last, left, right } = rectangleOf(state, mark, head)
  const changes: ChangeSpec[] = []
  const killed: string[] = []
  const headLine = doc.lineAt(head)
  // カーソルの行の, 変更の後のカーソルの位置 (行の始めから)
  let cursor = head - headLine.from
  for (let n = first; n <= last; n++) {
    const line = doc.line(n)
    const start = moveToColumn(line.text, left, 'coerce', config)
    if (start.column < left) {
      killed.push(' '.repeat(right - left))
      continue
    }
    const end = moveToColumn(start.text, right, 't', config)
    killed.push(end.text.slice(start.index, end.index))
    const text = end.text.slice(0, start.index) + end.text.slice(end.index)
    changes.push({ from: line.from, to: line.to, insert: text })
    // カーソルは, 消した所の後ろなら, 消した所の始めへ (Emacs の save-excursion のマーカー)
    if (n === headLine.number) cursor = Math.min(cursor, start.index)
  }
  const set = state.changes(changes)
  // 前の行の変更で, カーソルの行の始めが動く
  const selection = EditorSelection.cursor(set.mapPos(headLine.from, -1) + cursor)
  return { spec: { changes: set, selection }, killed }
}

/**
 * 矩形を, カーソルを左上の角として入れます (Emacs の `yank-rectangle`. `C-x r y`).
 * 行が短ければ, 桁まで空白で埋め, 行が足りなければ, 足します.
 *
 * @param state - エディタの状態.
 * @param pos - カーソル.
 * @param rectangle - 矩形 (行ごと).
 * @returns 変更とカーソル (最後に入れた文字の後ろ).
 */
export function yankRectangle(
  state: EditorState,
  pos: number,
  rectangle: string[],
): TransactionSpec {
  const { doc } = state
  const config = state.facet(indentConfig)
  const top = doc.lineAt(pos)
  const col = columnAt(top.text, pos - top.from, config.tabWidth)
  const changes: ChangeSpec[] = [{ from: pos, insert: rectangle[0] }]
  // 最後に入れた行と, 入れた文字の後ろの位置 (行の始めから)
  let last = top
  let end = pos - top.from + rectangle[0].length
  let added = ''
  for (let i = 1; i < rectangle.length; i++) {
    const n = top.number + i
    const text = n > doc.lines ? '' : doc.line(n).text
    const at = moveToColumn(text, col, 't', config)
    const insert = at.text.slice(0, at.index) + rectangle[i] + at.text.slice(at.index)
    if (n > doc.lines) {
      // 行が足りなければ, 文書の終わりに足す
      added += '\n' + insert
    } else {
      last = doc.line(n)
      end = at.index + rectangle[i].length
      changes.push({ from: last.from, to: last.to, insert })
    }
  }
  if (added) changes.push({ from: doc.length, insert: added })
  const set = state.changes(changes)
  // 文書の終わりに足したなら, 文書の終わり. ほかは, 前の行の変更で動いた, 行の始めから
  const cursor = added ? set.newLength : set.mapPos(last.from, -1) + end
  return { changes: set, selection: EditorSelection.cursor(cursor) }
}

/**
 * 矩形の各行を, 文字で置き換えます (Emacs の `string-rectangle`. `C-x r t`).
 * 行が短ければ, 矩形の左の桁まで空白で埋めます.
 *
 * @param state - エディタの状態.
 * @param mark - マーク.
 * @param head - カーソル.
 * @param string - 置き換える文字.
 * @returns 変更とカーソル (最後の行の, 入れた文字の後ろ).
 */
export function stringRectangle(
  state: EditorState,
  mark: number,
  head: number,
  string: string,
): TransactionSpec {
  const { doc } = state
  const config = state.facet(indentConfig)
  const { first, last, left, right } = rectangleOf(state, mark, head)
  const changes: ChangeSpec[] = []
  let end = 0
  for (let n = first; n <= last; n++) {
    const line = doc.line(n)
    let { text } = moveToColumn(line.text, left, 't', config)
    // 桁まで埋めたので, 左の桁には届く
    const start = moveToColumn(text, left, 'coerce', config)
    const stop = moveToColumn(start.text, right, 'coerce', config)
    text = stop.text.slice(0, start.index) + string + stop.text.slice(stop.index)
    changes.push({ from: line.from, to: line.to, insert: text })
    end = start.index + string.length
  }
  const set = state.changes(changes)
  return {
    changes: set,
    selection: EditorSelection.cursor(set.mapPos(doc.line(last).from, -1) + end),
  }
}
