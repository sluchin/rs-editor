import { describe, it, expect } from 'vitest'
import { countLines, countMessage, countSentences, countWords } from '@/lib/count'

/** 文字列と, `emacs -Q --batch` の `count-words--format` で, バッファ全体を数えた結果. */
const CASES: [string, string][] = [
  ['привет мир', 'Buffer has 1 line, 1 sentence, 2 words, and 10 characters'],
  ['abcабв', 'Buffer has 1 line, 1 sentence, 2 words, and 6 characters'],
  ['コーヒーを飲む', 'Buffer has 1 line, 1 sentence, 2 words, and 7 characters'],
  ['a', 'Buffer has 1 line, 1 sentence, 1 word, and 1 character'],
  ['a\n', 'Buffer has 1 line, 1 sentence, 1 word, and 2 characters'],
  ['a b\nc', 'Buffer has 2 lines, 1 sentence, 3 words, and 5 characters'],
  [
    "don't foo_bar foo-bar 12 3.5 x\n\n",
    'Buffer has 2 lines, 1 sentence, 10 words, and 32 characters',
  ],
  ["(define (foo-bar x) 'a) ; don't", 'Buffer has 1 line, 1 sentence, 7 words, and 31 characters'],
  [
    '日本語の文章です。カタカナと漢字abc',
    'Buffer has 1 line, 2 sentences, 5 words, and 19 characters',
  ],
  ['漢字かなカナ漢字', 'Buffer has 1 line, 1 sentence, 3 words, and 8 characters'],
  ['ｱｲｳ全角ＡＢＣ１２', 'Buffer has 1 line, 1 sentence, 3 words, and 10 characters'],
  ['한국어 단어', 'Buffer has 1 line, 1 sentence, 2 words, and 6 characters'],
  ['αβγ δ', 'Buffer has 1 line, 1 sentence, 2 words, and 5 characters'],
  ['a😀b ©x', 'Buffer has 1 line, 1 sentence, 4 words, and 6 characters'],
  ['x—y', 'Buffer has 1 line, 1 sentence, 2 words, and 3 characters'],
  ['1,000.5', 'Buffer has 1 line, 1 sentence, 3 words, and 7 characters'],
  ['café naïve', 'Buffer has 1 line, 1 sentence, 2 words, and 10 characters'],
  ['été', 'Buffer has 1 line, 1 sentence, 1 word, and 4 characters'],
  ['One. Two.  Three!  Four?\nFive', 'Buffer has 2 lines, 4 sentences, 5 words, and 29 characters'],
  ['Mr. Smith went.  Away', 'Buffer has 1 line, 2 sentences, 4 words, and 21 characters'],
  ['First para.\n\nSecond para.\n', 'Buffer has 3 lines, 2 sentences, 4 words, and 26 characters'],
  ['\n\n\nabc\n\n', 'Buffer has 5 lines, 1 sentence, 1 word, and 8 characters'],
  ['   ', 'Buffer has 1 line, 0 sentences, 0 words, and 3 characters'],
  ['a.  ', 'Buffer has 1 line, 1 sentence, 1 word, and 4 characters'],
  ['End.)  Next', 'Buffer has 1 line, 2 sentences, 2 words, and 11 characters'],
  ['これは文。これも文！最後？', 'Buffer has 1 line, 3 sentences, 5 words, and 13 characters'],
  [
    'Line one\nline two.\n\n\nNew para without end',
    'Buffer has 5 lines, 2 sentences, 8 words, and 41 characters',
  ],
  ['x\n\n\n', 'Buffer has 3 lines, 2 sentences, 1 word, and 4 characters'],
  ['a\n\nb\n\nc', 'Buffer has 5 lines, 3 sentences, 3 words, and 7 characters'],
  ['Hi!\tThere.', 'Buffer has 1 line, 2 sentences, 2 words, and 10 characters'],
  ['Q?\nA.\n', 'Buffer has 2 lines, 2 sentences, 2 words, and 6 characters'],
  [' word', 'Buffer has 1 line, 1 sentence, 1 word, and 5 characters'],
  ['1. item\n2. item\n', 'Buffer has 2 lines, 1 sentence, 4 words, and 16 characters'],
]

describe('count-words (M-=)', () => {
  it.each(CASES)('%j を, Emacs と同じに数えること', (text, expected) => {
    expect(countMessage('Buffer', text)).toBe(expected)
  })

  it('空の文字列は, どれも 0 と数えること (Emacs はエラーになる)', () => {
    expect(countLines('')).toBe(0)
    expect(countSentences('')).toBe(0)
    expect(countWords('')).toBe(0)
    expect(countMessage('Region', '')).toBe(
      'Region has 0 lines, 0 sentences, 0 words, and 0 characters',
    )
  })
})
