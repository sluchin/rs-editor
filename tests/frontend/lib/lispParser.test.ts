import { describe, it, expect } from 'vitest'
import { LispParser } from '@/lib/lispParser'

/**
 * 本文を読んで, 知らせた括弧を返します.
 *
 * @param text - 本文.
 * @param scheme - Scheme なら `true`.
 * @returns 括弧 (行, 位置, 文字).
 */
function parens(text: string, scheme = true): string[] {
  const found: string[] = []
  const parser = new LispParser(scheme, (line, index, ch) => found.push(`${line}:${index}${ch}`))
  text.split('\n').forEach((line, n) => parser.scanLine(line, n))
  return found
}

describe('LispParser', () => {
  it('文字列とコメントの中の括弧を除くこと', () => {
    expect(parens('(a "(\\")") ; (x\n[b]')).toEqual(['0:0(', '0:9)', '1:0[', '1:2]'])
    // 行をまたぐ文字列
    expect(parens('"(\n)" ()')).toEqual(['1:3(', '1:4)'])
  })

  it('Scheme の #| |# (入れ子も) と, #; の式と, 文字の括弧を除くこと', () => {
    expect(parens('#| ( #| ) |# ( |# (a)')).toEqual(['0:18(', '0:20)'])
    expect(parens('#; \'(a (b) "(" #\\( c) (d)')).toEqual(['0:22(', '0:24)'])
    expect(parens('#; #\\( (e)')).toEqual(['0:7(', '0:9)'])
    expect(parens('#;\n\tsym (f)')).toEqual(['1:5(', '1:7)'])
    expect(parens('(#\\( #\\))')).toEqual(['0:0(', '0:8)'])
  })

  it('Emacs Lisp では, #| と #; はコメントでなく, ?\\( は文字であること', () => {
    expect(parens('#|(a)', false)).toEqual(['0:2(', '0:4)'])
    expect(parens('(?\\( ?()', false)).toEqual(['0:0(', '0:6(', '0:7)'])
  })
})
