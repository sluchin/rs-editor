import { describe, it, expect } from 'vitest'
import { fileDirectory } from '@/lib/paths'

describe('パス', () => {
  it('ディレクトリを返すこと', () => {
    expect(fileDirectory('/home/u/a.scm')).toBe('/home/u/')
    expect(fileDirectory('/a')).toBe('/')
    expect(fileDirectory('a')).toBe('')
  })
})
