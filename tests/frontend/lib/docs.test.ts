import { describe, it, expect } from 'vitest'
import { DocStore, toText } from '@/lib/docs'
import type { BufferFrame } from '@/lib/session'

/**
 * バッファの描く内容を作ります.
 *
 * @param name - 名前.
 * @param fields - 変える項目.
 * @returns 描く内容.
 */
const buffer = (name: string, fields: Partial<BufferFrame> = {}): BufferFrame => ({
  name,
  version: 0,
  text: null,
  changes: [],
  modified: false,
  read_only: false,
  file: null,
  directory: null,
  highlights: [],
  ...fields,
})

describe('DocStore', () => {
  it('全体を受けたら置き換え, 変更を受けたら当て, 消したバッファは忘れること', () => {
    const store = new DocStore()
    // 知らないバッファと, 本文をまだ受けていないバッファは, 空
    expect(store.get('a').doc.toString()).toBe('')
    store.apply([buffer('a', { text: 'hello\nworld' }), buffer('b')])
    expect(store.get('a').doc.toString()).toBe('hello\nworld')
    expect(store.get('a').changes).toBeNull()
    expect(store.get('b').doc.length).toBe(0)
    const first = store.get('a').doc
    store.apply([
      buffer('a', {
        changes: [
          { from: 0, to: 0, insert: '>' },
          { from: 6, to: 12, insert: '!' },
        ],
      }),
    ])
    const entry = store.get('a')
    expect(entry.doc.toString()).toBe('>hello!')
    expect(entry.prev).toBe(first)
    expect(entry.changes?.apply(first).toString()).toBe('>hello!')
    // 変えていなければ, 変更は無い
    store.apply([buffer('a')])
    expect(store.get('a').changes).toBeNull()
    expect(store.get('a').doc).toBe(entry.doc)
    // 全体を受け直す
    store.apply([buffer('a', { text: 'new' })])
    expect(store.get('a').prev).toBe(entry.doc)
    // 描く内容に無いバッファは, 忘れる
    store.apply([])
    expect(store.get('a').doc.toString()).toBe('')
  })

  it('文字列を, 行に分けた本文にすること', () => {
    expect(toText('a\nb').lines).toBe(2)
  })
})
