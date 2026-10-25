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
  describeKeyField,
  lastCommandField,
  markField,
  zapField,
  pageLines,
  toEmacsKey,
} from '@/lib/emacs'
import { KillRing } from '@/lib/killRing'
import { KeyboardMacro } from '@/lib/macro'

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
  const macro = new KeyboardMacro()
  const commands = createCommands({ killRing, message, macro })
  const view = new EditorView({
    parent: document.body,
    state: EditorState.create({
      doc: doc.replace('|', ''),
      selection: EditorSelection.cursor(Math.max(cursor, 0)),
      extensions: [
        emacsState(),
        EditorView.domEventHandlers({
          keydown: (event) => {
            macro.record(event)
            return false
          },
        }),
        keymap.of(
          emacsKeymap((name) => commands[name], {
            macro,
            message,
            repeatable: (name) => name !== 'transpose-chars',
          }),
        ),
      ],
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
  /** キーを押します (`C-x (` のように, 空白で区切ると続けて押す). */
  const press = (keys: string) => {
    for (const stroke of keys.split(' ')) {
      const ctrlKey = stroke.startsWith('C-')
      const altKey = stroke.startsWith('M-')
      const key = ctrlKey || altKey ? stroke.slice(2) : stroke
      view.contentDOM.dispatchEvent(new KeyboardEvent('keydown', { key, ctrlKey, altKey }))
    }
  }
  return { view, run, text, selected, type, message, killRing, press, macro }
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

  it('M-t で, 前の単語 (単語の中なら, その単語) と後ろの単語を入れ替えること', () => {
    const middle = setup('foo b|ar, baz')
    middle.run('transpose-words')
    expect(middle.text()).toBe('foo baz, bar|')
    const between = setup('foo |bar')
    between.run('transpose-words')
    expect(between.text()).toBe('bar foo|')
    // 後ろに単語が無いときと, 単語が無いときは, 知らせる
    for (const doc of ['foo bar|', '|']) {
      const none = setup(doc)
      none.run('transpose-words')
      expect(none.text()).toBe(doc)
      expect(none.message).toHaveBeenCalledWith("Don't have two things to transpose")
    }
  })

  it('C-x C-t で, 前の行と入れ替えて, 次の行へ動くこと (先頭の行は次の行と, 最後の行は改行を足す)', () => {
    const middle = setup('a\nb|b\nc')
    middle.run('transpose-lines')
    expect(middle.text()).toBe('bb\na\n|c')
    const first = setup('|a\nb')
    first.run('transpose-lines')
    expect(first.text()).toBe('b\na\n|')
    const one = setup('a|')
    one.run('transpose-lines')
    expect(one.text()).toBe('a|')
    expect(one.message).toHaveBeenCalledWith("Don't have two things to transpose")
  })

  it('M-/ で, 単語を補完し, 続けて押すと次の候補に, 尽きたら略語に戻すこと', () => {
    const { run, text, message } = setup('apple apricot\nap|')
    run('dabbrev-expand')
    expect(text()).toBe('apple apricot\napricot|')
    run('dabbrev-expand')
    expect(text()).toBe('apple apricot\napple|')
    run('dabbrev-expand')
    expect(text()).toBe('apple apricot\nap|')
    expect(message).toHaveBeenCalledWith('No further dynamic expansion for ‘ap’ found')
    // 尽きた後は, 始めから探し直す
    run('dabbrev-expand')
    expect(text()).toBe('apple apricot\napricot|')
  })

  it('M-/ で, 略語が無いときと, 候補が無いときは, 知らせること', () => {
    const blank = setup('a |')
    blank.run('dabbrev-expand')
    expect(blank.message).toHaveBeenCalledWith('No possible abbreviation preceding point')
    const none = setup('zz|')
    none.run('dabbrev-expand')
    expect(none.text()).toBe('zz|')
    expect(none.message).toHaveBeenCalledWith('No dynamic expansion for ‘zz’ found')
  })

  it('M-z で, 次に押した文字までをキルすること (前置引数があれば, その数だけ先の文字まで)', () => {
    const { press, text, message, killRing } = setup('|abcabcx')
    press('M-z')
    expect(message).toHaveBeenCalledWith('Zap to char: ')
    press('Control')
    press('b')
    expect(text()).toBe('|cabcx')
    expect(killRing.current()).toBe('ab')
    const repeated = setup('|abcabcx')
    repeated.press('C-u 2 M-z c')
    expect(repeated.text()).toBe('|x')
  })

  it('M-z の文字が小文字なら, 大文字と小文字を区別せず, 大文字なら, 区別すること', () => {
    const lower = setup('|aXbx')
    lower.press('M-z x')
    expect(lower.text()).toBe('|bx')
    const upper = setup('|xaXb')
    upper.press('M-z X')
    expect(upper.text()).toBe('|b')
  })

  it('M-z の文字が無ければ, 知らせること', () => {
    const { press, text, message, view } = setup('|abc')
    press('M-z q')
    expect(text()).toBe('|abc')
    expect(message).toHaveBeenLastCalledWith('Search failed: "q"')
    expect(view.state.field(zapField)).toBeNull()
  })

  it('M-z の後の RET と TAB は, 改行とタブの文字として使うこと', () => {
    const newline = setup('|ab\ncd')
    newline.press('M-z Enter')
    expect(newline.text()).toBe('|cd')
    const tab = setup('|a\tb')
    tab.press('M-z Tab')
    expect(tab.text()).toBe('|b')
  })

  it('M-z の後の, 文字でないキーは, 待つのを止めて, そのキーのコマンドを動かすこと', () => {
    const { press, text, view } = setup('|abc')
    press('M-z C-f')
    expect(text()).toBe('a|bc')
    expect(view.state.field(zapField)).toBeNull()
    // 割り当てのないキーは, 待つのを止めるだけ
    press('M-z C-z')
    expect(view.state.field(zapField)).toBeNull()
    press('b')
    expect(text()).toBe('a|bc')
  })

  it('M-q で, 段落を詰めること (詰める段落が無ければ, 何もしない)', () => {
    const words = Array.from({ length: 15 }, (_, i) => `word${i}`).join(' ')
    const { run, text } = setup(`|${words}`)
    run('fill-paragraph')
    expect(text()).toBe(
      '|word0 word1 word2 word3 word4 word5 word6 word7 word8 word9 word10\nword11 word12 word13 word14',
    )
    const empty = setup('|\n')
    empty.run('fill-paragraph')
    expect(empty.text()).toBe('|\n')
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

describe('前置引数', () => {
  it('作り方を省くと, 全てのコマンドを繰り返すこと', () => {
    const commands = createCommands({
      killRing: new KillRing(),
      message: vi.fn(),
      macro: new KeyboardMacro(),
    })
    const view = new EditorView({
      parent: document.body,
      state: EditorState.create({
        doc: 'abcdef',
        extensions: [emacsState(), keymap.of(emacsKeymap((name) => commands[name]))],
      }),
    })
    views.push(view)
    view.contentDOM.dispatchEvent(new KeyboardEvent('keydown', { key: 'u', ctrlKey: true }))
    view.contentDOM.dispatchEvent(new KeyboardEvent('keydown', { key: 'f', ctrlKey: true }))
    expect(view.state.selection.main.head).toBe(4)
  })

  it('C-u は 4 回, 続けると 16 回, 数字はその数だけ繰り返すこと', () => {
    const { press, text, message } = setup('|' + 'x'.repeat(40))
    press('C-u C-f')
    expect(text()).toBe('xxxx|' + 'x'.repeat(36))
    expect(message).toHaveBeenCalledWith('C-u 4-')
    press('C-u C-u C-f')
    expect(text().indexOf('|')).toBe(20)
    expect(message).toHaveBeenCalledWith('C-u 16-')
    press('C-u 1 2 C-u C-b')
    expect(text().indexOf('|')).toBe(8)
    expect(message).toHaveBeenCalledWith('C-u 12-')
    press('M-3 C-b')
    expect(text().indexOf('|')).toBe(5)
  })

  it('割り当てのない文字は, その数だけ入れ, 繰り返さないコマンドは 1 回だけ動かすこと', () => {
    const { press, text, view } = setup('ab|')
    press('C-u a')
    expect(text()).toBe('abaaaa|')
    press('C-u C-t')
    expect(text()).toBe('abaaaa|')
    view.dispatch({ selection: { anchor: 1 } })
    press('M-2 C-t')
    expect(text()).toBe('ba|aaaa')
    // 文字でないキーと, 修飾キー付きのキーは, 前置引数を残したまま, 処理しない
    press('C-u ArrowRight C-z')
    press('b')
    expect(text()).toBe('babbbb|aaaa')
  })

  it('前置引数の後の, コマンドのないキーは, 前置引数を捨てること', () => {
    const { press, text } = setup('|ab')
    press('C-u M-x')
    press('C-f')
    expect(text()).toBe('a|b')
  })
})

describe('キーボードマクロ', () => {
  it('C-x ( から C-x ) までのキーを記録し, C-x e で再生すること', () => {
    const { press, text, message } = setup('|')
    press('C-x (')
    expect(message).toHaveBeenCalledWith('Defining kbd macro...')
    press('C-x (')
    expect(message).toHaveBeenCalledWith('Already defining kbd macro')
    press('C-x e')
    expect(message).toHaveBeenCalledWith(
      'Cannot execute a keyboard macro while defining or executing one',
    )
    // jsdom では, キーを押しても文字は入らない (再生では入れる)
    press('Shift h i C-z C-b')
    press('C-x )')
    expect(message).toHaveBeenCalledWith('Keyboard macro defined')
    press('C-x e')
    expect(text()).toBe('h|i')
    // 2 回: hi| → hih|i → hihh|ii
    press('C-e C-u 2 C-x e')
    expect(text()).toBe('hihh|ii')
  })

  it('M-z と, その後に押した文字を記録し, 再生すること', () => {
    const { press, text } = setup('|a,b,c,d')
    press('C-x ( M-z , C-x )')
    expect(text()).toBe('|b,c,d')
    press('C-x e')
    expect(text()).toBe('|c,d')
  })

  it('記録していないときと, 記録がないときは, 知らせること', () => {
    const { press, message, macro, run } = setup('|')
    press('C-x )')
    expect(message).toHaveBeenCalledWith('Not defining kbd macro')
    press('C-x e')
    expect(message).toHaveBeenCalledWith('No kbd macro has been defined')
    // 割り当てより多いキーを除いても, 空になるだけ
    run('start-kbd-macro')
    macro.noteBinding(5)
    run('end-kbd-macro')
    expect(macro.isDefined).toBe(false)
  })
})

describe('キーの説明', () => {
  it('C-h k の次のキーを, 実行せずに説明すること', () => {
    const { press, text, message, view, run } = setup('|ab')
    press('C-h k')
    expect(message).toHaveBeenCalledWith('Describe the following key: ')
    press('C-f')
    expect(message).toHaveBeenLastCalledWith('C-f runs the command forward-char')
    expect(text()).toBe('|ab')
    press('C-h k C-x C-f')
    expect(message).toHaveBeenLastCalledWith('C-x C-f runs the command find-file')
    // 修飾キーだけは, 次のキーを待つ
    run('describe-key')
    press('Control')
    expect(view.state.field(describeKeyField)).toBe(true)
    press('a')
    expect(message).toHaveBeenLastCalledWith('a runs the command self-insert-command')
    press('C-h k C-z')
    expect(message).toHaveBeenLastCalledWith('C-z is undefined')
    press('C-h k M-k')
    expect(message).toHaveBeenLastCalledWith('M-k is undefined')
    press('C-h k')
    view.contentDOM.dispatchEvent(new KeyboardEvent('keydown', { key: 'q', metaKey: true }))
    expect(message).toHaveBeenLastCalledWith('s-q is undefined')
    press('C-f')
    expect(text()).toBe('a|b')
  })

  it('CodeMirror のキーの書き方を, Emacs の書き方にすること', () => {
    expect(toEmacsKey('Ctrl-Alt-f')).toBe('C-M-f')
    expect(toEmacsKey('Ctrl-x Ctrl-f')).toBe('C-x C-f')
    expect(toEmacsKey('Shift-ArrowLeft')).toBe('S-<left>')
    expect(toEmacsKey('Meta-a')).toBe('s-a')
    expect(toEmacsKey('F5')).toBe('<f5>')
    expect(toEmacsKey('Ctrl--')).toBe('C--')
    expect(toEmacsKey('Ctrl-Space')).toBe('C-SPC')
    expect(toEmacsKey('Cmd-a')).toBe('Cmd-a')
  })
})
