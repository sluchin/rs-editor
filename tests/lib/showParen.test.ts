import { describe, it, expect, afterEach } from 'vitest'
import { Compartment, EditorSelection, EditorState, Text } from '@codemirror/state'
import { EditorView } from '@codemirror/view'
import { indentConfig } from '@/lib/lispIndent'
import { findParen, showParen } from '@/lib/showParen'

/** 作ったエディタ (テストの後に消す). */
const views: EditorView[] = []

afterEach(() => {
  while (views.length > 0) views.pop()?.destroy()
})

type Dialect = 'scheme' | 'elisp' | null

/**
 * カーソルの近くの括弧を探します. 本文の `|` の位置にカーソルを置きます.
 *
 * @param doc - 本文 (`|` はカーソル).
 * @param dialect - Lisp の方言.
 * @returns 括弧と対の位置と, 合わないか. 無ければ `null`.
 */
function find(doc: string, dialect: Dialect = 'scheme') {
  const found = findParen(Text.of(doc.replace('|', '').split('\n')), doc.indexOf('|'), dialect)
  return found && [found.here, found.paren.match, found.paren.mismatch]
}

describe('findParen', () => {
  it('前の閉じる括弧を先に, 無ければ後ろの開く括弧を見ること (Emacs と同じ)', () => {
    expect(find('(a)|(b)')).toEqual([2, 0, false])
    expect(find('(a) |')).toBeNull()
    expect(find('|(a [b] c)')).toEqual([0, 8, false])
    expect(find('(a [b]|)')).toEqual([5, 3, false])
    expect(find('|a')).toBeNull()
    expect(find('|')).toBeNull()
  })

  it('対が無いか, 括弧の種類が違えば, 合わないとすること', () => {
    expect(find('|(a')).toEqual([0, null, true])
    expect(find('a)|')).toEqual([1, null, true])
    expect(find('|(a]')).toEqual([0, 2, true])
    expect(find('(a]|')).toEqual([2, 0, true])
  })

  it('文字列とコメントと文字の括弧は, 対にしないこと', () => {
    expect(find('|(a "(" ; )\n #\\) b)')).toEqual([0, 17, false])
    expect(find('|(a #| ) |# b)')).toEqual([0, 12, false])
    expect(find('|(a #;(b) c)')).toEqual([0, 10, false])
    // `\(` は括弧でない
    expect(find('a\\|(b)', null)).toBeNull()
    expect(find('(?\\( a)|', 'elisp')).toEqual([6, 0, false])
  })

  it('文字列とコメントの中の括弧は, 同じ行の中で対を探すこと', () => {
    expect(find('"foo |(bar) baz"')).toEqual([5, 9, false])
    expect(find('"foo (bar)| baz"')).toEqual([9, 5, false])
    expect(find('"foo ([b\\)a] r)|"')).toEqual([14, 5, false])
    expect(find('"foo |([bar)]"')).toEqual([5, 11, true])
    expect(find('"foo [(bar])|"')).toEqual([11, 5, true])
    // `"` か行の終わりで止める
    expect(find('"|(" (a)')).toEqual([1, null, true])
    expect(find('; |(a\nb)')).toEqual([2, null, true])
    expect(find('; (a b)|', 'elisp')).toEqual([6, 2, false])
  })

  it('Fundamental モードは, {} も括弧にし, 文字列の外の括弧を対にすること', () => {
    expect(find('|{a "}" \\} }', null)).toEqual([0, 10, false])
    expect(find('|{a}', 'elisp')).toBeNull()
    expect(find('(a ; )|\nb)', null)).toEqual([5, 0, false])
  })

  it('本文が同じなら, 読み直さないこと', () => {
    const doc = Text.of(['(a (b))'])
    expect(findParen(doc, 0, 'elisp')?.paren.match).toBe(6)
    expect(findParen(doc, 0, 'elisp')?.paren.match).toBe(6)
    expect(findParen(doc, 3, null)?.paren.match).toBe(5)
  })
})

describe('showParen', () => {
  /**
   * 括弧の強調を付けたエディタを作ります.
   *
   * @param doc - 本文 (`|` はカーソル).
   * @param dialect - Lisp の方言.
   * @returns エディタ.
   */
  function setup(doc: string, dialect: Dialect = 'scheme') {
    const view = new EditorView({
      parent: document.body,
      state: EditorState.create({
        doc: doc.replace('|', ''),
        selection: EditorSelection.cursor(doc.indexOf('|')),
        extensions: [showParen(), indentConfig.of({ dialect, indentTabs: true, tabWidth: 8 })],
      }),
    })
    views.push(view)
    return view
  }

  /**
   * フォーカスを移して, エディタが知るまで待ちます (CodeMirror は, 少し後で知らせる).
   *
   * @param view - エディタ.
   */
  const focus = async (view: EditorView) => {
    view.focus()
    await new Promise((resolve) => setTimeout(resolve, 50))
  }

  /** 強調した文字 (色の名前と, 文字). */
  const marks = (view: EditorView) =>
    Array.from(view.contentDOM.querySelectorAll('[class^="cm-show-paren"]')).map((e) => [
      e.className,
      e.textContent,
    ])

  it('フォーカスがあれば, カーソルの近くの括弧と対に色を付けること', async () => {
    const view = setup('(a [b] c)|')
    expect(marks(view)).toEqual([])
    await focus(view)
    expect(marks(view)).toEqual([
      ['cm-show-paren-match', '('],
      ['cm-show-paren-match', ')'],
    ])
    view.dispatch({ selection: EditorSelection.cursor(3) })
    expect(marks(view)).toEqual([
      ['cm-show-paren-match', '['],
      ['cm-show-paren-match', ']'],
    ])
    view.dispatch({ selection: EditorSelection.cursor(2) })
    expect(marks(view)).toEqual([])
    // 本文もカーソルも変わらなければ, そのまま
    view.dispatch({ scrollIntoView: true })
    expect(marks(view)).toEqual([])
    view.dispatch({ changes: { from: 0, insert: ')' }, selection: EditorSelection.cursor(1) })
    expect(marks(view)).toEqual([['cm-show-paren-mismatch', ')']])
    view.contentDOM.blur()
    await new Promise((resolve) => setTimeout(resolve, 50))
    expect(marks(view)).toEqual([])
  })

  it('メジャーモードが変わったら, 付け直すこと', async () => {
    const mode = new Compartment()
    const view = new EditorView({
      parent: document.body,
      state: EditorState.create({
        doc: '{a}',
        selection: EditorSelection.cursor(3),
        extensions: [
          showParen(),
          mode.of(indentConfig.of({ dialect: 'elisp', indentTabs: true, tabWidth: 8 })),
        ],
      }),
    })
    views.push(view)
    await focus(view)
    expect(marks(view)).toEqual([])
    view.dispatch({
      effects: mode.reconfigure(indentConfig.of({ dialect: null, indentTabs: true, tabWidth: 8 })),
    })
    expect(marks(view)).toEqual([
      ['cm-show-paren-match', '{'],
      ['cm-show-paren-match', '}'],
    ])
  })
})
