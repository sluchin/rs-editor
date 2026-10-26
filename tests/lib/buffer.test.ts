import { describe, it, expect } from 'vitest'
import { type Buffer, headLines, toText, uniquifyBufferNames } from '@/lib/buffer'

/**
 * バッファを作ります.
 *
 * @param name - 名前.
 * @param fields - ファイルか, dired のディレクトリ.
 * @returns バッファ.
 */
function buffer(name: string, fields: Partial<Buffer> = {}): Buffer {
  return { id: name, name, filePath: null, doc: toText(''), modified: false, ...fields }
}

/**
 * ファイルのバッファを作ります (名前は, ファイルの名前).
 *
 * @param path - ファイルのパス.
 * @returns バッファ.
 */
function file(path: string): Buffer {
  return buffer(path.slice(path.lastIndexOf('/') + 1), { id: path, filePath: path })
}

/**
 * 名前を付け直した, バッファの名前を返します.
 *
 * @param buffers - バッファ.
 * @returns 名前.
 */
const names = (buffers: Buffer[]) => uniquifyBufferNames(buffers).map((b) => b.name)

describe('uniquifyBufferNames', () => {
  it('同じ名前が無ければ, 同じ配列を返す', () => {
    const buffers = [buffer('*scratch*'), file('/a/index.ts'), file('/a/main.ts')]
    expect(uniquifyBufferNames(buffers)).toBe(buffers)
  })

  it('同じ名前のファイルは, 区別できるまで, 親のディレクトリを足す', () => {
    expect(names([file('/p/src/index.ts'), file('/p/lib/index.ts')])).toEqual([
      'index.ts<src>',
      'index.ts<lib>',
    ])
    expect(
      names([file('/x/a/index.ts'), file('/y/src/index.ts'), file('/z/src/index.ts')]),
    ).toEqual(['index.ts<a>', 'index.ts<y/src>', 'index.ts<z/src>'])
    // 一番上のディレクトリのファイル
    expect(names([file('/index.ts'), file('/a/index.ts')])).toEqual(['index.ts</>', 'index.ts<a>'])
  })

  it('重なりが無くなれば, もとの名前に戻し, 変わらないバッファは同じものを使う', () => {
    const other = file('/b/main.ts')
    const result = uniquifyBufferNames([{ ...file('/a/index.ts'), name: 'index.ts<a>' }, other])
    expect(result.map((b) => b.name)).toEqual(['index.ts', 'main.ts'])
    expect(result[1]).toBe(other)
  })

  it('dired のバッファも, ファイルと同じく区別する', () => {
    expect(
      names([
        buffer('src', { directory: '/p/src' }),
        buffer('src', { directory: '/q/src/' }),
        buffer('/', { directory: '/' }),
      ]),
    ).toEqual(['src<p>', 'src<q>', '/'])
  })

  it('区別できない名前 (同じファイル, ファイルのないバッファ) には, 番号を付ける', () => {
    expect(names([file('/a/x'), file('/a/x'), file('/a/x')])).toEqual([
      'x<a>',
      'x<a><2>',
      'x<a><3>',
    ])
    // ファイルのないバッファの名前は, 変えない
    expect(names([buffer('notes'), file('/a/notes')])).toEqual(['notes', 'notes<2>'])
  })
})

describe('toText と headLines', () => {
  it('行の区切りを CodeMirror と同じく読み, 始めの 2 行だけを返すこと', () => {
    expect(toText('a\r\nb\rc\nd').toString()).toBe('a\nb\nc\nd')
    expect(headLines(toText('-*- mode: scheme -*-\nsecond\nthird'))).toBe(
      '-*- mode: scheme -*-\nsecond',
    )
    expect(headLines(toText('only'))).toBe('only')
    expect(headLines(toText(''))).toBe('')
  })
})
