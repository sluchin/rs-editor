import { describe, it, expect } from 'vitest'
import { findExpansions } from '@/lib/dabbrev'

describe('findExpansions', () => {
  it('カーソルの前を近い順に, 次に後ろを近い順に探すこと (同じ候補と, 略語そのものは除く)', () => {
    const text = 'foobar fooqux fo fooqux\nfo| fozz foobar'
    const pos = text.indexOf('|')
    expect(findExpansions(text.replace('|', ''), pos, false)).toEqual({
      from: pos - 2,
      abbrev: 'fo',
      candidates: ['fooqux', 'foobar', 'fozz'],
    })
  })

  it('カーソルが単語の中なら, カーソルまでを略語にし, その単語は候補にしないこと', () => {
    expect(findExpansions('foobar fox', 9, false)).toEqual({
      from: 7,
      abbrev: 'fo',
      candidates: ['foobar'],
    })
  })

  it('Lisp では, 記号 (`-` などを含む) を 1 つの単語にすること', () => {
    const text = '(string-append str'
    expect(findExpansions(text, text.length, true)?.candidates).toEqual(['string-append'])
    expect(findExpansions(text, text.length, false)?.candidates).toEqual(['string'])
  })

  it('カーソルの前が単語でなければ, null を返すこと', () => {
    expect(findExpansions('foo ', 4, false)).toBeNull()
    expect(findExpansions('foo', 0, false)).toBeNull()
  })
})
