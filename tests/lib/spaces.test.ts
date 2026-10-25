import { describe, it, expect, vi, afterEach } from 'vitest'
import { EditorSelection, EditorState } from '@codemirror/state'
import { EditorView, keymap } from '@codemirror/view'
import { type Command, createCommands, emacsKeymap, emacsState } from '@/lib/emacs'
import { KillRing } from '@/lib/killRing'
import { KeyboardMacro } from '@/lib/macro'
import { type IndentConfig, indentConfig } from '@/lib/lispIndent'

/** 作ったエディタ (テストの後に消す). */
const views: EditorView[] = []

afterEach(() => {
  while (views.length > 0) views.pop()?.destroy()
})

/** モードの名前と, 字下げの設定の方言. */
const DIALECTS: Record<string, IndentConfig['dialect']> = {
  text: null,
  scheme: 'scheme',
  elisp: 'elisp',
}

/**
 * エディタを作ります.
 *
 * @param mode - モード (`text`, `scheme`, `elisp`).
 * @param doc - 本文 (`|` はカーソル).
 * @returns エディタと, コマンドと, 本文を返す関数.
 */
function setup(mode: string, doc: string) {
  const commands = createCommands({
    killRing: new KillRing(),
    message: vi.fn(),
    macro: new KeyboardMacro(),
  })
  const view = new EditorView({
    parent: document.body,
    state: EditorState.create({
      doc: doc.replace('|', ''),
      selection: EditorSelection.cursor(doc.indexOf('|')),
      extensions: [
        emacsState(),
        indentConfig.of({ dialect: DIALECTS[mode], indentTabs: true, tabWidth: 8 }),
        keymap.of(emacsKeymap((name) => commands[name])),
      ],
    }),
  })
  views.push(view)
  /** 本文を, カーソルの位置を `|` で示して返します. */
  const text = () => {
    const d = view.state.doc.toString()
    const head = view.state.selection.main.head
    return d.slice(0, head) + '|' + d.slice(head)
  }
  return { view, commands, text }
}

/**
 * モード, コマンド, 前の本文, 後の本文. 後の本文は, `emacs -Q --batch` で,
 * text-mode, scheme-mode, emacs-lisp-mode のコマンドを動かした結果.
 */
const CASES: [string, string, string, string][] = [
  ['text', 'just-one-space', 'a  \t|  b', 'a |b'],
  ['text', 'just-one-space', 'a|b', 'a |b'],
  ['text', 'just-one-space', 'a \n | \n b', 'a \n |\n b'],
  ['text', 'just-one-space', '|  b', ' |b'],
  ['text', 'just-one-space', 'a  |', 'a |'],
  ['text', 'delete-horizontal-space', 'a  \t|  b', 'a|b'],
  ['text', 'delete-horizontal-space', 'a \n | \n b', 'a \n|\n b'],
  ['text', 'delete-horizontal-space', 'a|b', 'a|b'],
  ['text', 'delete-indentation', 'a  \n   b|c', 'a| bc'],
  ['text', 'delete-indentation', '|a', '|a'],
  ['text', 'delete-indentation', 'ab|c', '|abc'],
  ['text', 'delete-indentation', 'a\n|', 'a|'],
  ['text', 'delete-indentation', '\n  |b', '|b'],
  ['text', 'delete-indentation', 'a\n\n|b', 'a\n|b'],
  ['text', 'delete-indentation', 'a\n  b|c  ', 'a| bc  '],
  ['text', 'delete-indentation', "(foo '\n   |(a))", "(foo '| (a))"],
  ['text', 'delete-indentation', 'x（\n|b）\n', 'x（|b）\n'],
  ['text', 'delete-indentation', '日本\n|語', '日本| 語'],
  ['text', 'delete-indentation', 'a\n|]', 'a|]'],
  ['scheme', 'delete-indentation', '(foo\n   |bar)', '(foo| bar)'],
  ['scheme', 'delete-indentation', '(foo\n   |)', '(foo|)'],
  ['scheme', 'delete-indentation', "(foo '\n   |(a))", "(foo '|(a))"],
  ['scheme', 'delete-indentation', '(foo #\n   |(a))', '(foo #|(a))'],
  ['scheme', 'delete-indentation', '(foo ,@\n   |a)', '(foo ,@|a)'],
  ['scheme', 'delete-indentation', '{\n|b', '{|b'],
  ['elisp', 'delete-indentation', '{\n|b', '{| b'],
  ['elisp', 'delete-indentation', 'a\n|}', 'a| }'],
  ['elisp', 'delete-indentation', '(a `\n|b)', '(a `|b)'],
  ['text', 'delete-blank-lines', 'a\n|\nb', 'a\n|b'],
  ['text', 'delete-blank-lines', 'a\n\n|\n  \n\nb', 'a\n|\nb'],
  ['text', 'delete-blank-lines', 'a|\n\n \n\nb', 'a|\nb'],
  ['text', 'delete-blank-lines', 'a|\nb', 'a|\nb'],
  ['text', 'delete-blank-lines', 'a\n\n|\n', 'a\n|'],
  ['text', 'delete-blank-lines', '|\n\na', '|\na'],
  ['text', 'delete-blank-lines', 'a\n  |  \nb', 'a\n|b'],
  ['text', 'delete-blank-lines', 'a|\n\n', 'a|\n'],
  ['text', 'delete-blank-lines', '|', '|'],
  ['text', 'delete-blank-lines', 'a\n|', 'a\n|'],
  ['text', 'delete-blank-lines', 'a\n|  ', 'a\n|'],
]

describe('空白と空の行を消すコマンド', () => {
  it.each(CASES)('%s: %s で, %j を, Emacs と同じに変えること', (mode, name, doc, expected) => {
    const { view, commands, text } = setup(mode, doc)
    expect((commands[name] as Command)(view)).toBe(true)
    expect(text()).toBe(expected)
  })

  it.each([
    ['M-SPC', [{ key: ' ', altKey: true }], 'a  |  b', 'a |b'],
    ['M-\\', [{ key: '\\', altKey: true }], 'a  |  b', 'a|b'],
    ['M-^', [{ key: '^', altKey: true }], 'a\n  |b', 'a| b'],
    [
      'C-x C-o',
      [
        { key: 'x', ctrlKey: true },
        { key: 'o', ctrlKey: true },
      ],
      'a\n|\nb',
      'a\n|b',
    ],
  ])('%s を割り当てること', (_, keys, doc, expected) => {
    const { view, text } = setup('text', doc)
    for (const key of keys) view.contentDOM.dispatchEvent(new KeyboardEvent('keydown', key))
    expect(text()).toBe(expected)
  })
})
