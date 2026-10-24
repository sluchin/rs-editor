import { describe, it, expect } from 'vitest'
import { StringStream } from '@codemirror/language'
import { majorMode } from '../lib/modes'
import { definitionKind, schemeParser } from '../lib/schemeMode'

/**
 * 行を, 字句に分けます (色を付けない字句と, 空白は除く).
 *
 * @param lines - 行.
 * @returns 字句の文字と種類.
 */
function tokenize(...lines: string[]): [string, string][] {
  const state = schemeParser.startState?.(2) as ReturnType<
    NonNullable<typeof schemeParser.startState>
  >
  const tokens: [string, string][] = []
  for (const line of lines) {
    const stream = new StringStream(line, 2, 2)
    while (!stream.eol()) {
      stream.start = stream.pos
      const type = schemeParser.token(stream, state)
      if (type && type !== 'paren') tokens.push([stream.current(), type])
    }
  }
  return tokens
}

describe('majorMode', () => {
  it('バッファの名前とファイルから, メジャーモードを決めること', () => {
    expect(majorMode('*scratch*', null)).toEqual({ name: 'Lisp Interaction', language: 'scheme' })
    expect(majorMode('*Help*', null)).toEqual({ name: 'Help', language: null })
    expect(majorMode('a.scm', '/tmp/a.scm')).toEqual({ name: 'Scheme', language: 'scheme' })
    expect(majorMode('a.sld', '/tmp/a.sld').language).toBe('scheme')
    expect(majorMode('a.txt', '/tmp/a.txt')).toEqual({ name: 'Fundamental', language: null })
    expect(majorMode('a.scm', null)).toEqual({ name: 'Fundamental', language: null })
  })
})

describe('schemeParser', () => {
  it('define の仲間を見分けること (Emacs の scheme-mode と同じ色)', () => {
    expect(definitionKind('define')).toBe('function')
    expect(definitionKind('define*')).toBe('function')
    expect(definitionKind('define-public')).toBe('function')
    expect(definitionKind('define-syntax')).toBe('variable')
    expect(definitionKind('define-macro')).toBe('variable')
    expect(definitionKind('define-command')).toBe('type')
    expect(definitionKind('define-record-type')).toBe('type')
    expect(definitionKind('defined')).toBeNull()
    expect(definitionKind('car')).toBeNull()
  })

  it('( の直後の特別な形と, 定義する名前に色を付けること', () => {
    expect(tokenize('(define (square x) (if (car x) x 1))')).toEqual([
      ['define', 'keyword'],
      ['square', 'definedFunction'],
      ['if', 'keyword'],
    ])
    expect(tokenize('(define-syntax swap! (syntax-rules () ((_ a b) b)))')).toEqual([
      ['define-syntax', 'keyword'],
      ['swap!', 'definedVariable'],
      ['syntax-rules', 'keyword'],
    ])
    expect(tokenize('(define-command hello', '  (lambda () 1))')).toEqual([
      ['define-command', 'keyword'],
      ['hello', 'definedType'],
      ['lambda', 'keyword'],
    ])
    // ( の直後でなければ, キーワードでも色を付けない
    expect(tokenize("'(if) (list if 'define)")).toEqual([['if', 'keyword']])
    // 定義する名前は, 次の字句だけ
    expect(tokenize('(define "x" y)')).toEqual([
      ['define', 'keyword'],
      ['"x"', 'string'],
    ])
    // [ の後は, キーワードにしない
    expect(tokenize('[if]')).toEqual([])
  })

  it('コメントと文字列は, 行をまたいで色を付けること', () => {
    expect(tokenize('(+ 1 2) ; sum (if')).toEqual([['; sum (if', 'comment']])
    expect(tokenize('"a \\" (if', 'b" (when)')).toEqual([
      ['"a \\" (if', 'string'],
      ['b"', 'string'],
      ['when', 'keyword'],
    ])
    expect(tokenize('#| a #| b |# (if', 'c |# (or)')).toEqual([
      ['#| a #| b |# (if', 'comment'],
      ['c |#', 'comment'],
      ['or', 'keyword'],
    ])
    expect(tokenize('#|', '|#')).toEqual([
      ['#|', 'comment'],
      ['|#', 'comment'],
    ])
  })

  it('文字の括弧と, 名前でない文字を, 名前と分けること', () => {
    // `#\(` は文字なので, 次の `(if` は ( の直後
    expect(tokenize('(list #\\( `(if ,x))')).toEqual([['if', 'keyword']])
    expect(tokenize('#\\')).toEqual([])
  })
})
