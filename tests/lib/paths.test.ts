import { describe, it, expect } from 'vitest'
import { autoSaveFileName, directoryFileName, fileDirectory } from '@/lib/paths'

describe('パス', () => {
  it('ディレクトリを返すこと', () => {
    expect(fileDirectory('/home/u/a.scm')).toBe('/home/u/')
    expect(fileDirectory('/a')).toBe('/')
    expect(fileDirectory('a')).toBe('')
  })

  it('ディレクトリの後ろの / を除くこと (/ は, そのまま)', () => {
    expect(directoryFileName('/home/u/')).toBe('/home/u')
    expect(directoryFileName('/home/u')).toBe('/home/u')
    expect(directoryFileName('/')).toBe('/')
    expect(directoryFileName('//')).toBe('/')
    expect(directoryFileName('')).toBe('')
  })

  it('自動保存のファイルは, 同じディレクトリの #名前# であること', () => {
    expect(autoSaveFileName('/home/u/a.scm')).toBe('/home/u/#a.scm#')
    expect(autoSaveFileName('a.scm')).toBe('#a.scm#')
  })
})
