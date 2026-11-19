import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { loadTreeRoot, loadTreeWidth, saveTreeRoot, saveTreeWidth } from '@/lib/treeState'

describe('treeState', () => {
  beforeEach(() => localStorage.clear())
  afterEach(() => vi.restoreAllMocks())

  it('根を保存して読み, null で消すこと', () => {
    expect(loadTreeRoot()).toBeNull()
    saveTreeRoot('/p')
    expect(loadTreeRoot()).toBe('/p')
    saveTreeRoot(null)
    expect(loadTreeRoot()).toBeNull()
  })

  it('幅を保存して読み, 数でない値は無いものとすること', () => {
    expect(loadTreeWidth()).toBeNull()
    saveTreeWidth(300)
    expect(loadTreeWidth()).toBe(300)
    localStorage.setItem('rseditor.projectTree.width', 'abc')
    expect(loadTreeWidth()).toBeNull()
  })

  it('保存先が使えなくても失敗しないこと', () => {
    vi.spyOn(Storage.prototype, 'getItem').mockImplementation(() => {
      throw new Error('denied')
    })
    vi.spyOn(Storage.prototype, 'setItem').mockImplementation(() => {
      throw new Error('denied')
    })
    vi.spyOn(Storage.prototype, 'removeItem').mockImplementation(() => {
      throw new Error('denied')
    })
    expect(loadTreeRoot()).toBeNull()
    expect(loadTreeWidth()).toBeNull()
    expect(() => saveTreeRoot('/p')).not.toThrow()
    expect(() => saveTreeRoot(null)).not.toThrow()
    expect(() => saveTreeWidth(300)).not.toThrow()
  })
})
