/**
 * Lisp のコメントの付け外し (Emacs の `comment-dwim`. `M-;`). コメントは `;` で始めます.
 *
 * - リージョンがあれば, リージョンの行をコメントにします. 全ての行がコメントなら, コメントを外します.
 * - リージョンが無ければ, カーソルの行の, コメントへ動きます. コメントが無ければ, 作ります
 *   (空白だけの行は, 式と同じ字下げで `;; ` を, 式の後ろは `comment-column` (40 桁) に `; ` を).
 */

import {
  ChangeSet,
  EditorSelection,
  type EditorState,
  type TransactionSpec,
} from '@codemirror/state'
import {
  COMMENT_COLUMN,
  LispParser,
  columnAt,
  indentConfig,
  indentLines,
  whitespace,
} from './lispIndent'

/** 行の始めのコメント (字下げと, `;` の並びと, 後ろの空白 1 つ). */
const LINE_COMMENT = /^([ \t]*);+ ?/

/** リージョンをコメントにするときに, 行に付ける文字 (Emacs の `comment-add` が 1 なので, `;` 2 つ). */
const REGION_COMMENT = ';; '

/** 空白だけの行. */
const BLANK = /^[ \t]*$/

/**
 * 行の, コメントの `;` の位置を返します (文字列の中の `;` を除く).
 *
 * @param state - エディタの状態.
 * @param lineNumber - 行 (1 から).
 * @returns 行の始めからの位置. コメントが無ければ `null`.
 */
export function commentStart(state: EditorState, lineNumber: number): number | null {
  const config = state.facet(indentConfig)
  // 文字列は, 行をまたぐので, 始めから読む
  const parser = new LispParser(config.dialect === 'scheme', config.tabWidth)
  for (let n = 1; n <= lineNumber; n++) parser.scanLine(state.doc.line(n).text, n - 1)
  return parser.comment
}

/**
 * 行の, 桁の位置を返します (字下げの中で, 桁に届く始めの位置).
 *
 * @param text - 行.
 * @param col - 桁.
 * @param tabWidth - タブの幅.
 * @returns 行の始めからの位置.
 */
function indexAtColumn(text: string, col: number, tabWidth: number): number {
  let i = 0
  while (columnAt(text, i, tabWidth) < col) i++
  return i
}

/**
 * リージョンの行をコメントにするか, 全ての行がコメントなら, コメントを外します
 * (Emacs の `comment-or-uncomment-region`). 空白だけの行は, 変えません.
 *
 * @param state - エディタの状態.
 * @param from - リージョンの始め.
 * @param to - リージョンの終わり.
 * @param pos - カーソルの位置.
 * @returns 変更とカーソル.
 */
function commentRegion(state: EditorState, from: number, to: number, pos: number): TransactionSpec {
  const { doc } = state
  const { tabWidth } = state.facet(indentConfig)
  // 終わりが行の始めなら, その行は含めない (indent-region と同じ)
  const end = doc.lineAt(to)
  const last = to === end.from ? end.number - 1 : end.number
  const lines = []
  for (let n = doc.lineAt(from).number; n <= last; n++) {
    const line = doc.line(n)
    if (!BLANK.test(line.text)) lines.push(line)
  }
  let changes
  if (lines.length > 0 && lines.every((line) => LINE_COMMENT.test(line.text))) {
    changes = lines.map((line) => {
      const match = LINE_COMMENT.exec(line.text) as RegExpExecArray
      return { from: line.from + match[1].length, to: line.from + match[0].length }
    })
  } else {
    // コメントの `;` は, 一番浅い字下げの桁にそろえる
    const col = Math.min(
      ...lines.map((line) =>
        columnAt(line.text, (/^[ \t]*/.exec(line.text) as RegExpExecArray)[0].length, tabWidth),
      ),
    )
    changes = lines.map((line) => ({
      from: line.from + indexAtColumn(line.text, col, tabWidth),
      insert: REGION_COMMENT,
    }))
  }
  const set = state.changes(changes)
  return { changes: set, selection: EditorSelection.cursor(set.mapPos(pos)) }
}

/**
 * コメントを付けるか, コメントへ動きます (Emacs の `comment-dwim`).
 *
 * @param state - エディタの状態.
 * @param pos - カーソルの位置.
 * @param mark - マークの位置 (置いていなければ `null`). カーソルと違えば, リージョンを変えます.
 * @returns 変更とカーソル. コメントの書き方が無い (Lisp でない) 場合は, 知らせる文字.
 */
export function commentDwim(
  state: EditorState,
  pos: number,
  mark: number | null,
): TransactionSpec | string {
  const config = state.facet(indentConfig)
  if (!config.dialect) return 'No comment syntax is defined'
  if (mark !== null && mark !== pos) {
    return commentRegion(state, Math.min(mark, pos), Math.max(mark, pos), pos)
  }
  const line = state.doc.lineAt(pos)
  const comment = commentStart(state, line.number)
  if (comment !== null) {
    // コメントの文字の始め (`;` の並びと, 空白 1 つの後) へ動く
    const starter = /^;+ ?/.exec(line.text.slice(comment)) as RegExpExecArray
    return { selection: EditorSelection.cursor(line.from + comment + starter[0].length) }
  }
  if (BLANK.test(line.text)) {
    // 空白だけの行は, 式と同じ字下げにして, `;; ` を入れる
    const indent = state.changes(
      indentLines(state, { from: line.number, to: line.number, emptyLines: true }).changes,
    )
    const at = indent.mapPos(line.to, 1)
    const changes = indent.compose(
      ChangeSet.of({ from: at, insert: REGION_COMMENT }, indent.newLength),
    )
    return { changes, selection: EditorSelection.cursor(at + REGION_COMMENT.length) }
  }
  // 式の後ろは, 後ろの空白を除いて, comment-column (式がそれより長ければ, 空白 1 つの後) に `; ` を入れる
  const code = line.text.replace(/[ \t]+$/, '')
  const col = columnAt(code, code.length, config.tabWidth)
  const insert = whitespace(col, Math.max(COMMENT_COLUMN, col + 1), config) + '; '
  const from = line.from + code.length
  return {
    changes: { from, to: line.to, insert },
    selection: EditorSelection.cursor(from + insert.length),
  }
}
