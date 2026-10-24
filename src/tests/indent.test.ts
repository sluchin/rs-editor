import { describe, it, expect, vi, afterEach } from 'vitest'
import { EditorSelection, EditorState } from '@codemirror/state'
import { EditorView } from '@codemirror/view'
import { type Command, createCommands, emacsState } from '../lib/emacs'
import { KillRing } from '../lib/killRing'
import { KeyboardMacro } from '../lib/macro'
import {
  charWidth,
  columnAt,
  type IndentConfig,
  indentConfig,
  indentLines,
} from '../lib/lispIndent'

/** 作ったエディタ (テストの後に消す). */
const views: EditorView[] = []

afterEach(() => {
  while (views.length > 0) views.pop()?.destroy()
})

/**
 * エディタを作ります. 本文の `|` の位置にカーソルを, `^` の位置にマークを置きます (`^` は `|` より前).
 *
 * @param doc - 本文.
 * @param config - 字下げの設定 (省くと, Scheme).
 * @returns 本文を返す関数と, コマンドを実行する関数.
 */
function setup(doc: string, config: Partial<IndentConfig> = {}) {
  const mark = doc.replace('|', '').indexOf('^')
  const plain = doc.replace('^', '')
  const cursor = plain.indexOf('|')
  const message = vi.fn()
  const commands = createCommands({
    killRing: new KillRing(),
    message,
    macro: new KeyboardMacro(),
  })
  const view = new EditorView({
    parent: document.body,
    state: EditorState.create({
      doc: plain.replace('|', ''),
      selection: EditorSelection.cursor(Math.max(cursor, 0)),
      extensions: [
        emacsState(),
        indentConfig.of({ dialect: 'scheme', indentTabs: true, tabWidth: 8, ...config }),
      ],
    }),
  })
  views.push(view)
  /** コマンドを, 名前で実行します. */
  const run = (name: string) => {
    expect((commands[name] as Command)(view)).toBe(true)
  }
  if (mark >= 0) {
    view.dispatch({ selection: EditorSelection.cursor(mark) })
    run('set-mark-command')
    view.dispatch({ selection: EditorSelection.cursor(cursor) })
  }
  /** 本文を, カーソルの位置を `|` で示して返します. */
  const text = () => {
    const d = view.state.doc.toString()
    const head = view.state.selection.main.head
    return d.slice(0, head) + '|' + d.slice(head)
  }
  return { view, run, text, message }
}

/**
 * 本文全体を字下げします (Emacs の `indent-region` を全体に).
 *
 * @param doc - 本文.
 * @param config - 字下げの設定.
 * @returns 字下げした本文.
 */
function indentAll(doc: string, config: Partial<IndentConfig> = {}): string {
  const state = EditorState.create({
    doc,
    extensions: indentConfig.of({ dialect: 'scheme', indentTabs: false, tabWidth: 8, ...config }),
  })
  const result = indentLines(state, { from: 1, to: state.doc.lines })
  return state.update({ changes: result.changes }).state.doc.toString()
}

describe('桁', () => {
  it('タブはタブの幅の倍数まで, 全角の文字は 2 桁と数えること', () => {
    expect(columnAt('a\tb', 3, 8)).toBe(9)
    expect(columnAt('あい', 2, 8)).toBe(4)
    expect(columnAt('😀a', 3, 8)).toBe(3)
    const wide = [0x1100, 0x2e80, 0x3041, 0x3400, 0x4e00, 0xa000, 0xac00, 0xf900, 0xfe30]
    wide.push(0xff00, 0xffe0, 0x1f600, 0x1f900, 0x20000)
    for (const code of wide) expect(charWidth(code)).toBe(2)
    for (const code of [0x41, 0x1160, 0x3ffff]) expect(charWidth(code)).toBe(1)
  })
})

describe('RET (newline)', () => {
  it('前の行の字下げを直し, 終わりの空白を消して, 新しい行を字下げすること (Emacs と同じ)', () => {
    const cases: [string, string][] = [
      ['(define (f x)|', '(define (f x)\n  |'],
      ['(define (f x)   |  (g x))', '(define (f x)\n  |(g x))'],
      ['   (define (f x)|', '(define (f x)\n  |'],
      // 文字列の中の行は, 字下げしない
      ['(define (f x)\n  "abc|', '(define (f x)\n  "abc\n|'],
    ]
    for (const [doc, expected] of cases) {
      const { run, text } = setup(doc)
      run('newline')
      expect(text()).toBe(expected)
    }
  })

  it('Fundamental モードでは, 前の行の字下げにそろえること (前の行が空なら, 字下げしない)', () => {
    const cases: [string, string][] = [
      ['  foo  |', '  foo\n  |'],
      ['|x', '\n|x'],
      ['a\n\n|x', 'a\n\n\n|x'],
    ]
    for (const [doc, expected] of cases) {
      const { run, text } = setup(doc, { dialect: null })
      run('newline')
      expect(text()).toBe(expected)
    }
  })

  it('C-j は, 改行だけを入れること', () => {
    const { run, text } = setup('(define (f x)|')
    run('electric-newline-and-maybe-indent')
    expect(text()).toBe('(define (f x)\n|')
  })
})

describe('TAB (indent-for-tab-command)', () => {
  it('行の字下げを直し, 字下げの中のカーソルは字下げの後ろへ動かすこと', () => {
    const cases: [string, string][] = [
      ['(define (f x)\n(g| x))', '(define (f x)\n  (g| x))'],
      ['(define (f x)\n   | (g x))', '(define (f x)\n  |(g x))'],
      ['(define (f x)\n|(g x))', '(define (f x)\n  |(g x))'],
      // 変えなくても, カーソルは字下げの後ろへ
      ['(define (f x)\n | (g x))', '(define (f x)\n  |(g x))'],
      // 文字列の中の行は, 変えない
      ['"a\n|  b"', '"a\n|  b"'],
    ]
    for (const [doc, expected] of cases) {
      const { run, text } = setup(doc)
      run('indent-for-tab-command')
      expect(text()).toBe(expected)
    }
  })

  it('タブの幅と, タブを使うかに従うこと', () => {
    const doc = '(define (f)\n  (let ((a 1))\n    (when a\n|b)))'
    const tabs = setup(doc, { tabWidth: 4 })
    tabs.run('indent-for-tab-command')
    expect(tabs.text()).toBe('(define (f)\n  (let ((a 1))\n    (when a\n\t  |b)))')
    const spaces = setup(doc, { indentTabs: false })
    spaces.run('indent-for-tab-command')
    expect(spaces.text()).toBe('(define (f)\n  (let ((a 1))\n    (when a\n      |b)))')
  })

  it('Fundamental モードでは, 前の行の次の字下げの位置まで空白を入れること (indent-relative)', () => {
    const cases: [string, string][] = [
      ['    foo bar\n|x', '    foo bar\n    |x'],
      ['    foo bar\nab|x', '    foo bar\nab  |x'],
      // 次の位置が無ければ, 次のタブの位置へ (前の空白は消す)
      ['foo\n          |x', 'foo\n\t\t|x'],
      ['foo\n\n|', 'foo\n\n\t|'],
      ['|x', '\t|x'],
      // 前の行のタブの途中なら, タブの後ろへ
      ['\tfoo bar\n  |x', '\tfoo bar\n  \t|x'],
      ['a\tfoo\n   |x', 'a\tfoo\n   \t|x'],
    ]
    for (const [doc, expected] of cases) {
      const { run, text } = setup(doc, { dialect: null })
      run('indent-for-tab-command')
      expect(text()).toBe(expected)
    }
  })

  it('リージョンがあれば, リージョンを字下げすること', () => {
    const { run, text } = setup('^(define (f x)\n(g\n  (h|)))\n(i\nj)')
    run('indent-for-tab-command')
    expect(text()).toBe('(define (f x)\n  (g\n   (h|)))\n(i\nj)')
  })
})

describe('C-M-q (indent-sexp) と C-M-\\ (indent-region)', () => {
  const doc = '(define (f x)\n(g\n\n   \n  (h)))\n(i\nj)'

  it('C-M-q は, 後ろの式の 2 行目からを字下げすること (空の行と, 空白だけの行は除く)', () => {
    const { run, text } = setup('|' + doc)
    run('indent-sexp')
    expect(text()).toBe('|(define (f x)\n  (g\n\n   \n   (h)))\n(i\nj)')
  })

  it('C-M-q は, Fundamental モードでは何もしないこと', () => {
    const { run, text } = setup('a\n|(b\nc)', { dialect: null })
    run('indent-sexp')
    expect(text()).toBe('a\n|(b\nc)')
  })

  it('C-M-\\ は, リージョンの行を字下げすること (空の行は除く. 終わりが行の始めなら, その行は除く)', () => {
    const { run, text } = setup('^' + doc.replace('(i', '|(i'))
    run('indent-region')
    expect(text()).toBe('(define (f x)\n  (g\n\n   \n   (h)))\n|(i\nj)')
  })

  it('C-M-\\ は, マークが無ければ知らせること', () => {
    const { run, text, message } = setup('|(a\nb)')
    run('indent-region')
    expect(message).toHaveBeenCalledWith('The mark is not set now, so there is no region')
    expect(text()).toBe('|(a\nb)')
  })

  it('Emacs Lisp のコメントは, ;; は本体に, ; はコメントの桁にそろえること', () => {
    const elisp = '(defun f ()\n(let ((a 1))\n(when a\n;; c\n; d\n;;; e\n(message "x"))))'
    const { run, text } = setup('^' + elisp + '|', { dialect: 'elisp' })
    run('indent-region')
    expect(text()).toBe(
      '(defun f ()\n  (let ((a 1))\n    (when a\n      ;; c\n\t\t\t\t\t; d\n;;; e\n      (message "x"))))|',
    )
  })
})

describe('字下げ (全体)', () => {
  it('Scheme の字下げが, Emacs 30 の scheme-mode と同じこと', () => {
    const doc = [
      '(define (f x)',
      '(let loop ((i 0)',
      '(j 1))',
      '(loop i',
      'j))',
      '(let ((a 1)',
      '(b 2))',
      'a)',
      '(do ((i 0 (+ i 1)))',
      '((= i 3))',
      '(display i))',
      '(defmacro-ish foo',
      'bar)',
      '((lambda (x)',
      'x) 1',
      '2)',
      '(#;(ignored',
      'x) g',
      'h)',
      '#| block',
      '(not code',
      '|#',
      "(list #\\( 'a",
      '`(b ,c ,@d)',
      '#(1 2)',
      '"str',
      ' (ing" e)',
      '(case x',
      "((1) 'a)",
      "(else 'b)))",
      '(f',
      ')',
      '(  g)',
      '\t  (when\tx',
      '\t\ty)',
    ].join('\n')
    const expected = [
      '(define (f x)',
      '  (let loop ((i 0)',
      '\t     (j 1))',
      '    (loop i',
      '\t  j))',
      '  (let ((a 1)',
      '\t(b 2))',
      '    a)',
      '  (do ((i 0 (+ i 1)))',
      '      ((= i 3))',
      '    (display i))',
      '  (defmacro-ish foo',
      '    bar)',
      '  ((lambda (x)',
      '     x) 1',
      '     2)',
      '  (#;(ignored',
      '   x) g',
      '      h)',
      '  #| block',
      '  (not code',
      '  |#',
      "  (list #\\( 'a",
      '\t`(b ,c ,@d)',
      '\t#(1 2)',
      '\t"str',
      ' (ing" e)',
      '  (case x',
      "    ((1) 'a)",
      "    (else 'b)))",
      '(f',
      ' )',
      '(  g)',
      '(when\tx',
      '  y)',
    ].join('\n')
    expect(indentAll(doc, { indentTabs: true })).toBe(expected)
    // タブを使わなければ, 字下げのタブは空白
    const spaces = expected.replace(/^\t+/gm, (tabs) => ' '.repeat(8 * tabs.length))
    expect(indentAll(doc)).toBe(spaces)
  })

  it('Emacs Lisp の字下げが, Emacs 30 の emacs-lisp-mode と同じこと', () => {
    const doc = [
      '(defun f (x)',
      '"doc"',
      '(let* ((a ?\\()',
      '(b ?())',
      '(when a',
      'b)))',
      '(define-key map "a"',
      "'cmd)",
      '(defxyz foo',
      'bar)',
      '(cl-defmethod g ((x integer))',
      'x)',
      '(unwind-protect',
      '(foo)',
      '(bar))',
      '(condition-case err',
      '(foo)',
      '(error',
      '(bar)))',
      '[a',
      'b]',
      "(funcall #'car",
      'x)',
      '(setq a 1',
      'b 2)',
    ].join('\n')
    const expected = [
      '(defun f (x)',
      '  "doc"',
      '  (let* ((a ?\\()',
      '\t (b ?())',
      '\t (when a',
      '\t   b)))',
      '  (define-key map "a"',
      "\t      'cmd)",
      '  (defxyz foo',
      '\t  bar)',
      '  (cl-defmethod g ((x integer))',
      '    x)',
      '  (unwind-protect',
      '      (foo)',
      '    (bar))',
      '  (condition-case err',
      '      (foo)',
      '    (error',
      '     (bar)))',
      '  [a',
      '   b]',
      "  (funcall #'car",
      '\t   x)',
      '  (setq a 1',
      '\tb 2)',
    ].join('\n')
    expect(indentAll(doc, { dialect: 'elisp', indentTabs: true })).toBe(expected)
  })

  it('入れ子のコメント, #; の式, 特別な引数の数を, Emacs と同じく扱うこと', () => {
    const scheme = [
      '(a #| x #| y |# z |#',
      'b)',
      '(c #',
      'd)',
      '(e #;\'x #;#(1 2) #;"s',
      't" #;#\\a #;[a b] #;',
      'f',
      'g)',
      '(do x y',
      'z)',
      '(begin a',
      'b)',
    ].join('\n')
    expect(indentAll(scheme)).toBe(
      [
        '(a #| x #| y |# z |#',
        ' b)',
        '(c #',
        ' d)',
        '(e #;\'x #;#(1 2) #;"s',
        ' t" #;#\\a #;[a b] #;',
        ' f',
        ' g)',
        '(do x y',
        '  z)',
        '(begin a',
        '       b)',
      ].join('\n'),
    )
    const elisp = '(macroexp-let2 a b\nc)\n(progn a\nb)'
    expect(indentAll(elisp, { dialect: 'elisp' })).toBe(
      '(macroexp-let2 a b\n               c)\n(progn a\n       b)',
    )
  })
})
