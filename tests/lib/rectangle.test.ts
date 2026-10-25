import { describe, it, expect } from 'vitest'
import { EditorSelection, EditorState } from '@codemirror/state'
import { killRectangle, stringRectangle, yankRectangle } from '@/lib/rectangle'
import { indentConfig } from '@/lib/lispIndent'

/**
 * 矩形のコマンドを動かした後の本文を返します.
 *
 * @param command - コマンド (`kill`, `yank`, `string`).
 * @param doc - 本文 (`|` はカーソル, `^` はマーク).
 * @param arg - 入れる矩形 (`yank`) か, 置き換える文字 (`string`).
 * @returns 本文 (`|` はカーソル) と, 消した中身 (`kill`).
 */
function run(command: string, doc: string, arg: string[] | string | null) {
  const plain = doc.replace(/[|^]/g, '')
  const marked = doc.replace('^', '')
  const head = marked.indexOf('|')
  const mark = doc.replace('|', '').indexOf('^')
  const state = EditorState.create({
    doc: plain,
    selection: EditorSelection.cursor(head),
    // Emacs の既定 (タブを使い, 幅は 8)
    extensions: [indentConfig.of({ dialect: null, indentTabs: true, tabWidth: 8 })],
  })
  let killed: string[] = []
  let spec
  if (command === 'kill') {
    const result = killRectangle(state, mark, head)
    spec = result.spec
    killed = result.killed
  } else if (command === 'yank') {
    spec = yankRectangle(state, head, arg as string[])
  } else {
    spec = stringRectangle(state, mark, head, arg as string)
  }
  const next = state.update(spec).state
  const text = next.doc.toString()
  const at = next.selection.main.head
  return [text.slice(0, at) + '|' + text.slice(at), killed]
}

/**
 * コマンド, 前の本文, 引数, 後の本文, 消した中身. 後の本文と, 消した中身は, `emacs -Q --batch` で,
 * `kill-rectangle`, `yank-rectangle`, `string-rectangle` を動かした結果.
 */
const CASES: [string, string, string[] | string | null, string, string[]][] = [
  ['kill', 'a^bc\nde\nfgh|i', null, 'a\nd\nf|i', ['bc', 'e ', 'gh']],
  ['kill', 'a|bc\nde\nfgh^i', null, 'a|\nd\nfi', ['bc', 'e ', 'gh']],
  [
    'kill',
    'ab^c\nd\tefgh\nijklmnopq|r',
    null,
    'ab\nd fgh\nij|r',
    ['c      ', '      e', 'klmnopq'],
  ],
  [
    'kill',
    'abcdefghij^k\nx\tyz\nabcdefghijkl|m',
    null,
    'abcdefghij\nx\tyz\nabcdefghij|m',
    ['k ', '  ', 'kl'],
  ],
  ['kill', 'ab^c\n日本語\nabcd|e', null, 'ab\n日語\nab|e', ['c ', '本', 'cd']],
  ['kill', 'ab^c\n日本語\nabc|e', null, 'ab\n日語\nab|e', ['c', '本', 'c']],
  ['kill', 'a^bc\n日本語\nabc|e', null, 'a\n日語\na|e', ['bc', '本', 'bc']],
  ['kill', '^abc\na\tb\nabc|e', null, '\n     b\n|e', ['abc', 'a  ', 'abc']],
  ['kill', 'a^bc\na\tb\nabc|e', null, 'a\na     b\na|e', ['bc', '  ', 'bc']],
  ['kill', '^ab|c', null, '|c', ['ab']],
  ['kill', 'abcd^ef\ngh\nijklm|nop', null, 'abcdf\ngh\nijkl|nop', ['e', ' ', 'm']],
  ['kill', 'x\nab|cd\nef^gh\ny', null, 'x\nab|cd\nefgh\ny', ['', '']],
  ['kill', 'ab^cd\nef|gh', null, 'abcd\nef|gh', ['', '']],
  ['yank', '^xy\nz|w', ['12', '34', '56'], 'xy\nz12w\n 34\n 56|', []],
  ['yank', 'x|\nabcdef', ['12', '34'], 'x12\na34|bcdef', []],
  ['yank', 'ab|c\nd\nefg', ['1', '2', '3'], 'ab1c\nd 2\nef3|g', []],
  ['yank', 'a|\tb\nx', ['1', '2'], 'a1\tb\nx2|', []],
  ['yank', 'x\tyz|', ['1', '2'], 'x\tyz1\n\t  2|', []],
  ['yank', 'abc|\n', ['1', '2'], 'abc1\n   2|', []],
  ['yank', '日本|語\na', ['1', '2'], '日本1語\na   2|', []],
  ['yank', 'a|b', ['1', '', '3'], 'a1b\n \n 3|', []],
  ['yank', 'a\tb|\nxy', ['1', '2'], 'a\tb1\nxy\t 2|', []],
  ['yank', 'ab|\na\tb', ['1', '2'], 'ab1\na 2|\tb', []],
  ['yank', 'a|b\n日本', ['1', '2'], 'a1b\n日2|本', []],
  ['yank', 'a|b\ncd\nef', ['XYZ'], 'aXYZ|b\ncd\nef', []],
  ['string', 'ab^cdef\nghijkl\nmn|opqr', 'XY', 'abXYcdef\nghXYijkl\nmnXY|opqr', []],
  ['string', 'ab|cdef\ng\nmn^opqr', 'XY', 'abXYcdef\ng XY\nmnXY|opqr', []],
  ['string', 'ab^cdef\nghijkl|', '', 'ab\ngh|', []],
  ['string', 'a\tb^c\nxyzwvutsr|qp', 'Z', 'a\tbZc\nxyzwvutsrZ|qp', []],
  ['string', 'ab^c\n日本語\nabc|e', 'Z', 'abZ\n日Z語\nabZ|e', []],
  ['string', 'a^bc\n日本語\nabc|e', 'Z', 'aZ\n日Z語\naZ|e', []],
  ['string', '^abc\na\tb\nabc|e', 'Z', 'Z\nZ\tb\nZ|e', []],
  ['string', 'a^bc\na\tb\nabc|e', 'Z', 'aZ\naZ\tb\naZ|e', []],
  ['string', 'abcdefghij^k\nx\tyz\nabcdefghijkl|m', 'Z', 'abcdefghijZ\nx\tyzZ\nabcdefghijZ|m', []],
  ['string', 'abcdefghijkl^\nx\nabcdefghijkl|', 'Z', 'abcdefghijklZ\nx\t    Z\nabcdefghijklZ|', []],
  ['kill', '😀^ab\nxyz|w', null, '😀b\nxy|w', ['a', 'z']],
  ['yank', '😀|a\nb', ['1', '2'], '😀1a\nb 2|', []],
  ['string', 'a😀^b\ncdef|g', 'Z', 'a😀Z\ncdeZ|g', []],
]

describe('矩形', () => {
  it.each(CASES)('%s: %j を, Emacs と同じに変えること', (command, doc, arg, expected, killed) => {
    expect(run(command, doc, arg)).toEqual([expected, killed])
  })
})
