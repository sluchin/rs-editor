import { describe, it, expect } from 'vitest'
import { grepBufferText, grepMatchIndex, lineStart, timeString } from '@/lib/grep'

describe('grep', () => {
  const date = new Date(2026, 9, 5, 9, 3, 7)

  it('時刻を, Emacs の current-time-string の形にする', () => {
    expect(timeString(date)).toBe('Mon Oct  5 09:03:07 2026')
  })

  it('結果のバッファを, Emacs の grep-mode の形で作る', () => {
    const matches = [
      { file: 'a.txt', line: 3, text: 'foo' },
      { file: 'src/b.ts', line: 10, text: 'x: foo' },
    ]
    expect(grepBufferText('/home/u/p/', '/home/u/', 'foo', matches, date)).toBe(
      [
        '-*- mode: grep; default-directory: "~/p/" -*-',
        'Grep started at Mon Oct  5 09:03:07 2026',
        '',
        'grep (built-in) -r -e foo .',
        'a.txt:3:foo',
        'src/b.ts:10:x: foo',
        '',
        'Grep finished with 2 matches found at Mon Oct  5 09:03:07 2026',
        '',
      ].join('\n'),
    )
    expect(grepBufferText('/tmp/', '/home/u/', 'x', [matches[0]], date)).toContain(
      'default-directory: "/tmp/"',
    )
    expect(grepBufferText('/tmp/', '/home/u/', 'x', [matches[0]], date)).toContain(
      'with 1 match found',
    )
    expect(grepBufferText('/tmp/', '/home/u/', 'x', [], date)).toContain('with no matches found')
  })

  it('バッファの行の番号から, 合った行を返す', () => {
    const matches = [
      { file: 'a.txt', line: 3, text: 'foo' },
      { file: 'b.txt', line: 1, text: 'foo' },
    ]
    expect(grepMatchIndex(matches, 4)).toBeNull()
    expect(grepMatchIndex(matches, 5)).toBe(0)
    expect(grepMatchIndex(matches, 6)).toBe(1)
    expect(grepMatchIndex(matches, 7)).toBeNull()
  })

  it('行の始めの位置を返す', () => {
    expect(lineStart('ab\ncd\nef', 1)).toBe(0)
    expect(lineStart('ab\ncd\nef', 3)).toBe(6)
    expect(lineStart('ab\ncd\nef', 9)).toBe(6)
  })
})
