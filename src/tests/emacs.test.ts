import { describe, it, expect, vi, afterEach } from 'vitest'
import { EditorSelection, EditorState } from '@codemirror/state'
import { EditorView, keymap } from '@codemirror/view'
import {
  type Command,
  createCommands,
  emacsKeymap,
  emacsState,
  gotoLine,
  KEY_BINDINGS,
  lastCommandField,
  markField,
  pageLines,
} from '../lib/emacs'
import { KillRing } from '../lib/killRing'

/** 作ったエディタ (テストの後に消す). */
const views: EditorView[] = []

afterEach(() => {
  while (views.length > 0) views.pop()?.destroy()
})

/**
 * エディタを作ります. 本文の `|` の位置にカーソルを置きます.
 *
 * @param doc - 本文 (`|` はカーソル).
 * @returns エディタと, コマンドと, 操作の関数.
 */
function setup(doc: string) {
  const cursor = doc.indexOf('|')
  const killRing = new KillRing()
  const message = vi.fn()
  const commands = createCommands({ killRing, message })
  const view = new EditorView({
    parent: document.body,
    state: EditorState.create({
      doc: doc.replace('|', ''),
      selection: EditorSelection.cursor(Math.max(cursor, 0)),
      extensions: [emacsState(), keymap.of(emacsKeymap((name) => commands[name]))],
    }),
  })
  views.push(view)
  /** コマンドを, 名前で実行します. */
  const run = (...names: string[]) => {
    for (const name of names) {
      expect((commands[name] as Command)(view)).toBe(true)
    }
  }
  /** 本文を, カーソルの位置を `|` で示して返します. */
  const text = () => {
    const doc = view.state.doc.toString()
    const head = view.state.selection.main.head
    return doc.slice(0, head) + '|' + doc.slice(head)
  }
  /** 選んでいる範囲の文字. */
  const selected = () => {
    const { from, to } = view.state.selection.main
    return view.state.sliceDoc(from, to)
  }
  /** 文字を入れます (コマンドでない書き換え). */
  const type = (insert: string) => {
    const pos = view.state.selection.main.head
    view.dispatch({ changes: { from: pos, insert }, selection: { anchor: pos + insert.length } })
  }
  return { view, run, text, selected, type, message, killRing }
}

describe('移動', () => {
  it('文字, 行の始めと終わり, 単語, バッファの始めと終わりへ動くこと', () => {
    const { run, text } = setup('ab|c de')
    run('forward-char')
    expect(text()).toBe('abc| de')
    run('forward-char', 'forward-char', 'forward-char', 'forward-char')
    expect(text()).toBe('abc de|')
    run('backward-char', 'move-beginning-of-line')
    expect(text()).toBe('|abc de')
    run('backward-char')
    expect(text()).toBe('|abc de')
    run('move-end-of-line', 'backward-word')
    expect(text()).toBe('abc |de')
    run('beginning-of-buffer', 'forward-word')
    expect(text()).toBe('abc| de')
    run('end-of-buffer')
    expect(text()).toBe('abc de|')
  })

  it('行を移るとき, 続けて移るあいだは始めの桁を保ち, 先頭と末尾の行から外へ出ないこと', () => {
    const { run, text, type } = setup('abcd|\nx\nabcdef')
    run('next-line')
    expect(text()).toBe('abcd\nx|\nabcdef')
    run('next-line')
    expect(text()).toBe('abcd\nx\nabcd|ef')
    run('next-line')
    expect(text()).toBe('abcd\nx\nabcd|ef')
    run('previous-line', 'previous-line', 'previous-line')
    expect(text()).toBe('abcd|\nx\nabcdef')
    // 入力すると, 桁を覚え直す
    type('!')
    run('next-line', 'next-line')
    expect(text()).toBe('abcd!\nx\nabcde|f')
  })

  it('S 式の単位で動くこと', () => {
    const { run, text } = setup('|(a (b)) c')
    run('forward-sexp')
    expect(text()).toBe('(a (b))| c')
    run('backward-sexp')
    expect(text()).toBe('|(a (b)) c')
  })

  it('1 から数えた行へ動き, 範囲の外は先頭か末尾の行にすること', () => {
    const { view, text } = setup('a\nb|\nc')
    gotoLine(view, 3)
    expect(text()).toBe('a\nb\n|c')
    gotoLine(view, 0)
    expect(text()).toBe('|a\nb\nc')
  })
})

describe('ページ送りと画面の位置', () => {
  it('画面の高さから, ページ送りの行の数を決めること (少なくとも 1 行)', () => {
    const { view } = setup('|a')
    expect(pageLines(view)).toBe(1)
    vi.spyOn(view.scrollDOM, 'clientHeight', 'get').mockReturnValue(120)
    vi.spyOn(view, 'defaultLineHeight', 'get').mockReturnValue(10)
    expect(pageLines(view)).toBe(10)
    vi.spyOn(view, 'defaultLineHeight', 'get').mockReturnValue(0)
    expect(pageLines(view)).toBe(1)
  })

  it('C-v と M-v で, ページの行の数だけ動くこと (マークがあれば範囲を広げる)', () => {
    const { view, run, text, selected } = setup('|0\n1\n2\n3\n4\n5')
    vi.spyOn(view.scrollDOM, 'clientHeight', 'get').mockReturnValue(40)
    vi.spyOn(view, 'defaultLineHeight', 'get').mockReturnValue(10)
    run('scroll-up-command')
    expect(text()).toBe('0\n1\n|2\n3\n4\n5')
    run('set-mark-command', 'scroll-up-command')
    expect(selected()).toBe('2\n3\n')
    run('keyboard-quit', 'scroll-down-command', 'scroll-down-command')
    expect(text()).toBe('|0\n1\n2\n3\n4\n5')
  })

  it('C-l を続けて押すと, 中央, 上, 下の順に置くこと', () => {
    const { view, run } = setup('a|')
    const steps = () => view.state.field(lastCommandField)?.step
    run('recenter-top-bottom')
    expect(steps()).toBe(0)
    run('recenter-top-bottom')
    expect(steps()).toBe(1)
    run('recenter-top-bottom', 'recenter-top-bottom')
    expect(steps()).toBe(0)
    run('forward-char', 'recenter-top-bottom')
    expect(steps()).toBe(0)
    // 書き換えも選択の変更もない変更 (設定の変更など) では, 直前のコマンドを覚えたまま
    view.dispatch({})
    run('recenter-top-bottom')
    expect(steps()).toBe(1)
  })
})

describe('削除とキル', () => {
  it('C-d で 1 文字消し, 末尾では何もしないこと', () => {
    const { run, text } = setup('a|bc')
    run('delete-char', 'delete-char', 'delete-char')
    expect(text()).toBe('a|')
  })

  it('C-k で行の終わりまでキルし, 行の終わりでは改行をキルし, 続けたキルは 1 つにつなげること', () => {
    const { run, text, killRing } = setup('|ab\ncd')
    run('kill-line', 'kill-line', 'kill-line', 'kill-line')
    expect(text()).toBe('|')
    expect(killRing.current()).toBe('ab\ncd')
  })

  it('単語を前と後ろへキルし, 後ろへのキルは, 項目の前につなげること', () => {
    const { run, text, killRing } = setup('foo bar| baz')
    run('kill-word')
    expect(text()).toBe('foo bar|')
    run('backward-kill-word', 'backward-kill-word')
    expect(text()).toBe('|')
    expect(killRing.current()).toBe('foo bar baz')
  })

  it('S 式をキルすること', () => {
    const { run, text, killRing } = setup('|(a b) c')
    run('kill-sexp')
    expect(text()).toBe('| c')
    expect(killRing.current()).toBe('(a b)')
  })

  it('リージョンをキルとコピーし, リージョンが無ければ知らせること', () => {
    const { run, text, killRing, message, view } = setup('|abc')
    run('kill-region', 'kill-ring-save')
    expect(message).toHaveBeenCalledTimes(2)
    expect(message).toHaveBeenCalledWith('The mark is not set now, so there is no region')
    run('set-mark-command', 'forward-char', 'kill-ring-save')
    expect(killRing.current()).toBe('a')
    expect(view.state.field(markField)).toBeNull()
    expect(text()).toBe('a|bc')
    run('set-mark-command', 'forward-char', 'kill-region')
    expect(text()).toBe('a|c')
    expect(killRing.current()).toBe('b')
  })

  it('コピーのすぐ後のキルは, 同じ項目につなげること', () => {
    const { run, killRing } = setup('|ab\ncd')
    run('set-mark-command', 'forward-char', 'kill-ring-save', 'kill-line')
    expect(killRing.current()).toBe('ab')
  })

  it('ヤンクし, 続けて M-y で古いものに置き換え, ヤンクの後でなければ知らせること', () => {
    const { run, text, message, killRing } = setup('|')
    run('yank')
    expect(message).toHaveBeenCalledWith('Kill ring is empty')
    killRing.push('old')
    killRing.push('new')
    run('yank-pop')
    expect(message).toHaveBeenCalledWith('Previous command was not a yank')
    run('yank')
    expect(text()).toBe('new|')
    run('yank-pop')
    expect(text()).toBe('old|')
    run('yank-pop')
    expect(text()).toBe('new|')
  })

  it('文字を入れ替えること (行の始めと終わり, 1 文字だけのときも)', () => {
    const middle = setup('a|bc')
    middle.run('transpose-chars')
    expect(middle.text()).toBe('ba|c')
    const start = setup('|abc')
    start.run('transpose-chars')
    expect(start.text()).toBe('ba|c')
    const end = setup('abc|')
    end.run('transpose-chars')
    expect(end.text()).toBe('acb|')
    const one = setup('a|')
    one.run('transpose-chars')
    expect(one.text()).toBe('a|')
  })

  it('単語の大文字・小文字を変えて, 単語の後へ動くこと', () => {
    const { run, text } = setup('|foo bar bAZ')
    run('upcase-word', 'downcase-word', 'capitalize-word')
    expect(text()).toBe('FOO bar Baz|')
  })

  it('C-o で, カーソルを動かさずに改行を入れること', () => {
    const { run, text } = setup('a|b')
    run('open-line')
    expect(text()).toBe('a|\nb')
  })

  it('元に戻し, やり直すこと (何もなければ, 何もしない)', () => {
    const { run, text, type } = setup('|')
    run('undo', 'redo')
    type('x')
    run('undo')
    expect(text()).toBe('|')
    run('redo')
    expect(text()).toBe('x|')
  })
})

describe('マークとリージョン', () => {
  it('マークを置くと, 移動で範囲が広がり, C-g で外れること', () => {
    const { run, selected, message, view } = setup('|abc')
    run('set-mark-command')
    expect(message).toHaveBeenCalledWith('Mark set')
    run('forward-char', 'forward-char')
    expect(selected()).toBe('ab')
    run('keyboard-quit')
    expect(message).toHaveBeenCalledWith('Quit')
    expect(selected()).toBe('')
    expect(view.state.field(markField)).toBeNull()
  })

  it('書き換えると, マークが外れること', () => {
    const { run, type, view } = setup('|abc')
    run('set-mark-command')
    type('x')
    expect(view.state.field(markField)).toBeNull()
  })

  it('C-x C-x でカーソルとマークを入れ替え, マークが無ければ知らせること', () => {
    const { run, text, message } = setup('|abc')
    run('exchange-point-and-mark')
    expect(message).toHaveBeenCalledWith('No mark set in this buffer')
    run('set-mark-command', 'forward-char', 'forward-char', 'exchange-point-and-mark')
    expect(text()).toBe('|abc')
    run('exchange-point-and-mark')
    expect(text()).toBe('ab|c')
  })

  it('C-x h で全てを選び, カーソルを先頭に, マークを末尾に置くこと', () => {
    const { run, text, selected, view } = setup('a|bc')
    run('mark-whole-buffer')
    expect(selected()).toBe('abc')
    expect(text()).toBe('|abc')
    expect(view.state.field(markField)).toBe(3)
  })
})

describe('キーの割り当て', () => {
  it('キーを押すと, 割り当てたコマンドを実行し, 無いコマンドは処理しないこと', () => {
    const { view, text } = setup('|ab')
    view.contentDOM.dispatchEvent(new KeyboardEvent('keydown', { key: 'f', ctrlKey: true }))
    expect(text()).toBe('a|b')
    view.contentDOM.dispatchEvent(new KeyboardEvent('keydown', { key: 'x', altKey: true }))
    expect(text()).toBe('a|b')
  })

  it('同じキーを, 二重に割り当てないこと', () => {
    const keys = KEY_BINDINGS.map(([key]) => key)
    expect(new Set(keys).size).toBe(keys.length)
  })
})
