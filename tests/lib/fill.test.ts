import { describe, it, expect } from 'vitest'
import { EditorSelection, EditorState } from '@codemirror/state'
import { fillParagraph } from '@/lib/fill'
import { type IndentConfig, indentConfig } from '@/lib/lispIndent'

/** モードの名前と, 字下げの設定の方言. */
const DIALECTS: Record<string, IndentConfig['dialect']> = {
  text: null,
  scheme: 'scheme',
  elisp: 'elisp',
}

/**
 * M-q を押した後の本文を返します.
 *
 * @param mode - モード (`text`, `scheme`, `elisp`).
 * @param doc - 本文 (`|` はカーソル).
 * @returns 本文 (`|` はカーソル).
 */
function fill(mode: string, doc: string): string {
  const pos = doc.indexOf('|')
  const state = EditorState.create({
    doc: doc.replace('|', ''),
    selection: EditorSelection.cursor(pos),
    extensions: [indentConfig.of({ dialect: DIALECTS[mode], indentTabs: true, tabWidth: 8 })],
  })
  const spec = fillParagraph(state, pos)
  const next = spec ? state.update(spec).state : state
  const head = next.selection.main.head
  const text = next.doc.toString()
  return text.slice(0, head) + '|' + text.slice(head)
}

/**
 * モード, M-q を押す前の本文, 押した後の本文. 押した後の本文は, `emacs -Q --batch` で,
 * text-mode, scheme-mode, emacs-lisp-mode の `fill-paragraph` を動かした結果.
 */
const CASES: [string, string, string][] = [
  [
    'text',
    'aaa |bbb ccc ddd eee fff ggg hhh iii jjj kkk lll mmm nnn ooo ppp qqq rrr sss ttt uuu vvv www xxx\nyyy zzz.\nNext sentence.  Mr. Smith  went   away.\n\nsecond para',
    'aaa |bbb ccc ddd eee fff ggg hhh iii jjj kkk lll mmm nnn ooo ppp qqq\nrrr sss ttt uuu vvv www xxx yyy zzz.  Next sentence.  Mr. Smith went\naway.\n\nsecond para',
  ],
  [
    'text',
    '  |indented first line words words words words words words words words words words words words\n  more',
    '  |indented first line words words words words words words words words\n  words words words words more',
  ],
  [
    'text',
    'x23456789 x23456789 x23456789 x23456789 x23456789 x23456789 x23456789\nab|',
    'x23456789 x23456789 x23456789 x23456789 x23456789 x23456789 x23456789\nab|',
  ],
  [
    'text',
    'x23456789 x23456789 x23456789 x23456789 x23456789 x23456789 x23456789 y\nab|',
    'x23456789 x23456789 x23456789 x23456789 x23456789 x23456789 x23456789\ny ab|',
  ],
  [
    'text',
    '|x234567890123456789012345678901234567890123456789012345678901234567890123456789 y z\nab',
    '|x234567890123456789012345678901234567890123456789012345678901234567890123456789\ny z ab',
  ],
  [
    'text',
    '|- aa bb cc dd ee ff gg hh ii jj kk ll mm nn oo pp qq rr ss tt uu vv ww xx yy zz',
    '|- aa bb cc dd ee ff gg hh ii jj kk ll mm nn oo pp qq rr ss tt uu vv ww\n  xx yy zz',
  ],
  [
    'text',
    '|> aa bb cc dd ee ff gg hh ii jj kk ll mm nn oo pp qq rr ss tt uu vv ww xx yy zz',
    '|> aa bb cc dd ee ff gg hh ii jj kk ll mm nn oo pp qq rr ss tt uu vv ww\n  xx yy zz',
  ],
  [
    'text',
    '|\taa bb cc dd ee ff gg hh ii jj kk ll mm nn oo pp qq rr ss tt uu vv ww xx yy zz',
    '|\taa bb cc dd ee ff gg hh ii jj kk ll mm nn oo pp qq rr ss tt uu\n\tvv ww xx yy zz',
  ],
  [
    'text',
    'a\n|\nb aa bb cc dd ee ff gg hh ii jj kk ll mm nn oo pp qq rr ss tt uu vv ww xx yy zz\n',
    'a\n|\nb aa bb cc dd ee ff gg hh ii jj kk ll mm nn oo pp qq rr ss tt uu vv ww\nxx yy zz\n',
  ],
  ['text', 'a\n|\n\n', 'a\n|\n\n'],
  [
    'text',
    '|  aa bb\n\tcc aa bb cc dd ee ff gg hh ii jj kk ll mm nn oo pp qq rr ss tt uu vv ww xx yy zz',
    '|  aa bb cc aa bb cc dd ee ff gg hh ii jj kk ll mm nn oo pp qq rr ss tt\n\tuu vv ww xx yy zz',
  ],
  [
    'text',
    '|> aa bb\ncc aa bb cc dd ee ff gg hh ii jj kk ll mm nn oo pp qq rr ss tt uu vv ww xx yy zz',
    '|> aa bb cc aa bb cc dd ee ff gg hh ii jj kk ll mm nn oo pp qq rr ss tt\nuu vv ww xx yy zz',
  ],
  [
    'text',
    '|aa bb\n  cc dd\n  aa bb cc dd ee ff gg hh ii jj kk ll mm nn oo pp qq rr ss tt uu vv ww xx yy zz',
    '|aa bb cc dd aa bb cc dd ee ff gg hh ii jj kk ll mm nn oo pp qq rr ss\n  tt uu vv ww xx yy zz',
  ],
  [
    'text',
    '|> aa\n> bb\n  cc dd ee ff gg hh ii jj kk ll mm nn oo pp qq rr ss tt uu vv ww xx yy zz',
    '|> aa bb cc dd ee ff gg hh ii jj kk ll mm nn oo pp qq rr ss tt uu vv ww\n> xx yy zz',
  ],
  [
    'text',
    '|> quoted aa bb cc dd ee ff gg hh ii jj kk ll mm nn oo pp qq rr ss tt uu vv ww xx yy zz\n> ww',
    '|> quoted aa bb cc dd ee ff gg hh ii jj kk ll mm nn oo pp qq rr ss tt\n> uu vv ww xx yy zz ww',
  ],
  ['text', '|aa\tbb  cc  \n', '|aa bb cc\n'],
  ['text', '|end?  Yes! No.)  Ok: done\nx', '|end?  Yes! No.)  Ok: done x'],
  [
    'text',
    '|xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx Mr. Smith',
    '|xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\nMr. Smith',
  ],
  [
    'text',
    'これ|は日本語の文章です。これは日本語の文章です。これは日本語の文章です。これは日本語の文章です。\n続きの行です。English words here.',
    'これ|は日本語の文章です。これは日本語の文章です。これは日本語の文章です。\nこれは日本語の文章です。続きの行です。English words here.',
  ],
  [
    'text',
    '|ああああああああああああああああああああああああああああああああああ「い」\n',
    '|ああああああああああああああああああああああああああああああああああ\n「い」\n',
  ],
  ['text', '|日本\nEnglish\n日本\n。\nabc。\nx\n', '|日本English日本。abc。x\n'],
  [
    'text',
    '|aaaa bbbb cccc dddd eeee ffff gggg hhhh iiii jjjj kkkk llll mmmm nnnnnn日本語の文章です\n',
    '|aaaa bbbb cccc dddd eeee ffff gggg hhhh iiii jjjj kkkk llll mmmm\nnnnnnn日本語の文章です\n',
  ],
  [
    'text',
    '|あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへほまみむめもabc def\n',
    '|あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへほまみむめも\nabc def\n',
  ],
  ['text', '|日本語 日本語 日本語\n日本語\n', '|日本語 日本語 日本語日本語\n'],
  [
    'text',
    '|これは日本語の文章です（括弧の中）これは日本語の文章です「かぎ」これは日本語の文章ですこれは\n',
    '|これは日本語の文章です（括弧の中）これは日本語の文章です「かぎ」これは\n日本語の文章ですこれは\n',
  ],
  [
    'text',
    'aaa bbb ccc ddd eee fff ggg hhh iii jjj kkk lll mmm nnn ooo ppp qqq rrr|',
    'aaa bbb ccc ddd eee fff ggg hhh iii jjj kkk lll mmm nnn ooo ppp qqq\nrrr|',
  ],
  [
    'elisp',
    ';;| aaa bbb ccc ddd eee fff ggg hhh iii jjj kkk lll mmm nnn ooo ppp qqq rrr sss ttt uuu vvv\n;; www xxx\n(defun f ()\n  1)',
    ';;| aaa bbb ccc ddd eee fff ggg hhh iii jjj kkk lll mmm nnn ooo ppp qqq\n;; rrr sss ttt uuu vvv www xxx\n(defun f ()\n  1)',
  ],
  ['elisp', '(def|un f (x)\n  (+ x\n     1))', '(def|un f (x)\n  (+ x\n     1))'],
  [
    'elisp',
    '(foo) ; a|aa bbb ccc ddd eee fff ggg hhh iii jjj kkk lll mmm nnn ooo ppp qqq rrr sss ttt uuu\n      ; vvv',
    '(foo) ; a|aa bbb ccc ddd eee fff ggg hhh iii jjj kkk lll mmm nnn ooo\n      ; ppp qqq rrr sss ttt uuu vvv',
  ],
  [
    'scheme',
    '  ;; a|aa bbb ccc ddd eee fff ggg hhh iii jjj kkk lll mmm nnn ooo ppp qqq rrr sss ttt uuu vvv\n  ;; www',
    '  ;; a|aa bbb ccc ddd eee fff ggg hhh iii jjj kkk lll mmm nnn ooo ppp\n  ;; qqq rrr sss ttt uuu vvv www',
  ],
  [
    'scheme',
    '|;;; aa bb cc dd ee ff gg hh ii jj kk ll mm nn oo pp qq rr ss tt uu vv ww xx yy zz',
    '|;;; aa bb cc dd ee ff gg hh ii jj kk ll mm nn oo pp qq rr ss tt uu vv\n;;; ww xx yy zz',
  ],
  ['scheme', '  ;; aa\n  ;; bb\n(|a)', '  ;; aa\n  ;; bb\n(|a)'],
  [
    'scheme',
    '|(a) ; aa bb cc dd ee ff gg hh ii jj kk ll mm nn oo pp qq rr ss tt uu vv ww xx yy zz\n(b) ; cc\n',
    '|(a) ; aa bb cc dd ee ff gg hh ii jj kk ll mm nn oo pp qq rr ss tt uu\n    ; vv ww xx yy zz\n(b) ; cc\n',
  ],
  ['scheme', '";| aa" \n', '";| aa" \n'],
  ['scheme', ';;  aa\n;;|  bb\n', ';;  aa |bb\n'],
  ['scheme', ';; aa\n;;; bb\n|;; cc\n', ';; aa\n;;; bb\n|;; cc\n'],
  ['scheme', '|;; aa\n;;\n;; cc\n', '|;; aa\n;;\n;; cc\n'],
  ['scheme', ';; aa\n;; b|b cc\n;; dd\n\n;; ee', ';; aa b|b cc dd\n\n;; ee'],
  ['scheme', '(a)\n\n|', '(a)\n\n|'],
  ['scheme', '(a) ; aa\n    ; b|b\n', '(a) ; aa b|b\n'],
]

describe('fillParagraph (M-q)', () => {
  it.each(CASES)('%s: %j を, Emacs と同じに詰めること', (mode, doc, expected) => {
    expect(fill(mode, doc)).toBe(expected)
  })

  it('詰める段落が無ければ, null を返すこと', () => {
    const state = EditorState.create({ doc: '\n\n' })
    expect(fillParagraph(state, 0)).toBeNull()
  })
})
