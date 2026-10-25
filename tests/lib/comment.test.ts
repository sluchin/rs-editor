import { describe, it, expect, vi, afterEach } from 'vitest'
import { EditorSelection, EditorState } from '@codemirror/state'
import { EditorView } from '@codemirror/view'
import { type Command, createCommands, emacsState, markField } from '@/lib/emacs'
import { KillRing } from '@/lib/killRing'
import { KeyboardMacro } from '@/lib/macro'
import { type IndentConfig, indentConfig } from '@/lib/lispIndent'

/** 作ったエディタ (テストの後に消す). */
const views: EditorView[] = []

afterEach(() => {
  while (views.length > 0) views.pop()?.destroy()
})

/**
 * エディタを作ります. 本文の `|` の位置にカーソルを, `^` の位置にマークを置きます (`^` は `|` より前).
 *
 * @param doc - 本文.
 * @param config - 字下げの設定 (省くと, Scheme で, 字下げに空白を使う).
 * @returns エディタと, 本文を返す関数と, コマンドを実行する関数.
 */
function setup(doc: string, config: Partial<IndentConfig> = {}) {
  const mark = doc.replace('|', '').indexOf('^')
  const plain = doc.replace('^', '')
  const cursor = plain.indexOf('|')
  const message = vi.fn()
  const commands = createCommands({ killRing: new KillRing(), message, macro: new KeyboardMacro() })
  const view = new EditorView({
    parent: document.body,
    state: EditorState.create({
      doc: plain.replace('|', ''),
      selection: EditorSelection.cursor(cursor),
      extensions: [
        emacsState(),
        indentConfig.of({ dialect: 'scheme', indentTabs: false, tabWidth: 8, ...config }),
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
 * M-; を押した後の本文を返します.
 *
 * @param doc - 本文 (`|` はカーソル, `^` はマーク).
 * @param config - 字下げの設定.
 * @returns 本文 (`|` はカーソル).
 */
function commentDwim(doc: string, config: Partial<IndentConfig> = {}): string {
  const { run, text } = setup(doc, config)
  run('comment-dwim')
  return text()
}

describe('comment-dwim (M-;)', () => {
  it('Lisp でなければ, コメントの書き方が無いと知らせること', () => {
    const { run, text, message } = setup('a|', { dialect: null })
    run('comment-dwim')
    expect(text()).toBe('a|')
    expect(message).toHaveBeenCalledWith('No comment syntax is defined')
  })

  it('式の後ろは, 40 桁に `; ` を入れること (式が長ければ, 空白 1 つの後. 後ろの空白は除く)', () => {
    expect(commentDwim('(foo)|')).toBe(`(foo)${' '.repeat(35)}; |`)
    expect(commentDwim('|(foo)   ')).toBe(`(foo)${' '.repeat(35)}; |`)
    // 字下げにタブを使うときは, タブで埋める
    expect(commentDwim('(foo)|', { indentTabs: true })).toBe('(foo)\t\t\t\t\t; |')
    const long = 'x'.repeat(45)
    expect(commentDwim(`${long}|`)).toBe(`${long} ; |`)
    expect(commentDwim(`${long}|`, { indentTabs: true })).toBe(`${long} ; |`)
  })

  it('コメントがあれば, コメントの文字の始めへ動くこと', () => {
    expect(commentDwim('|(foo) ; bar')).toBe('(foo) ; |bar')
    expect(commentDwim(';;;no space|')).toBe(';;;|no space')
    expect(commentDwim('(a)\n|  ;; b', { dialect: 'elisp' })).toBe('(a)\n  ;; |b')
  })

  it('文字列の中の `;` は, コメントにしないこと (行をまたぐ文字列も)', () => {
    expect(commentDwim('"a;b"|')).toBe(`"a;b"${' '.repeat(35)}; |`)
    expect(commentDwim('"a\nb;c"|')).toBe(`"a\nb;c"${' '.repeat(36)}; |`)
  })

  it('空白だけの行は, 式と同じ字下げで `;; ` を入れること', () => {
    expect(commentDwim('(define (f)\n|\n  1)')).toBe('(define (f)\n  ;; |\n  1)')
    expect(commentDwim('   |')).toBe(';; |')
  })

  it('リージョンの行をコメントにし, 一番浅い字下げにそろえ, マークを外すこと', () => {
    const { run, text, view } = setup('^(a)\n  (b)\n|')
    run('comment-dwim')
    expect(text()).toBe(';; (a)\n;;   (b)\n|')
    expect(view.state.field(markField)).toBeNull()
    // 空白だけの行は変えない
    expect(commentDwim('^  (a)\n\n    (b)|')).toBe('  ;; (a)\n\n  ;;   (b)|')
    // タブの字下げは, 桁でそろえる
    expect(commentDwim('^\t(a)\n (b)|', { indentTabs: true })).toBe('\t;; (a)\n ;; (b)|')
  })

  it('リージョンの全ての行がコメントなら, コメントを外し, 一部だけなら, 全てをコメントにすること', () => {
    expect(commentDwim('^;; (a)\n  ;;(b)|')).toBe('(a)\n  (b)|')
    expect(commentDwim('^;; (a)\n(b)|')).toBe(';; ;; (a)\n;; (b)|')
    // 空白だけの行しか無ければ, 何もしない
    expect(commentDwim('^\n  \n|')).toBe('\n  \n|')
  })
})
