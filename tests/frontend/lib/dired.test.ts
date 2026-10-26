import { describe, it, expect } from 'vitest'
import { diredBufferName, diredListing } from '@/lib/dired'

describe('dired', () => {
  it('バッファの名前は, ディレクトリの名前であること', () => {
    expect(diredBufferName('/home/u/src/')).toBe('src')
    expect(diredBufferName('/home/u/src')).toBe('src')
    expect(diredBufferName('/')).toBe('/')
  })

  it('一覧は, ディレクトリの行と, total を除いた ls の行を, 2 文字の字下げで並べること', () => {
    const listing = [
      'total 4',
      'drwxr-xr-x  3 u u  100 Oct 25 16:42 .',
      '-rw-r--r--  1 u u    3 Oct 25 16:42 a.txt',
      '',
    ].join('\n')
    expect(diredListing('/home/u/d/', listing)).toBe(
      [
        '  /home/u/d:',
        '  drwxr-xr-x  3 u u  100 Oct 25 16:42 .',
        '  -rw-r--r--  1 u u    3 Oct 25 16:42 a.txt',
        '',
      ].join('\n'),
    )
  })
})
