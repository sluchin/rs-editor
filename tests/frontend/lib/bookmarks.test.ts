import { describe, it, expect, vi, beforeEach } from 'vitest'
import { invoke } from '@tauri-apps/api/core'
import {
  bookmarkPosition,
  loadBookmarks,
  makeBookmark,
  saveBookmarks,
  storeBookmark,
} from '@/lib/bookmarks'
import * as fileOps from '@/lib/fileOps'

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }))
vi.mock('@/lib/fileOps', () => ({
  readFile: vi.fn(),
  writeFile: vi.fn(),
  pathExists: vi.fn(),
}))

const mocked = vi.mocked(fileOps)
const mockedInvoke = vi.mocked(invoke)

/** 本文 (Emacs で, 20 文字目に bookmark-set をして確かめたもの). */
const TEXT = 'abcdefghijklmnopqrstuvwxyz0123456789\nline2\n'

describe('ブックマーク', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    mockedInvoke.mockResolvedValue('/home/u/.config/rseditor/bookmarks')
  })

  it('位置の前後の 16 文字を覚えること (Emacs と同じ)', () => {
    expect(makeBookmark('foo.scm', '/tmp/foo.scm', TEXT, 19)).toEqual({
      name: 'foo.scm',
      filename: '/tmp/foo.scm',
      position: 19,
      front: 'tuvwxyz012345678',
      rear: 'defghijklmnopqrs',
    })
    expect(makeBookmark('a', '/a', 'xy', 1)).toMatchObject({ front: 'y', rear: 'x' })
  })

  it('新しい名前は始めに足し, 同じ名前は, その場で置き換えること', () => {
    const a = makeBookmark('a', '/a', '', 0)
    const b = makeBookmark('b', '/b', '', 0)
    const list = storeBookmark(storeBookmark([], a), b)
    expect(list.map((x) => x.name)).toEqual(['b', 'a'])
    const moved = { ...a, position: 5 }
    expect(storeBookmark(list, moved)).toEqual([b, moved])
  })

  it('本文を書き換えていても, 前後の文字を探して, 位置を直すこと', () => {
    const bookmark = makeBookmark('m', '/m', TEXT, 19)
    expect(bookmarkPosition(bookmark, TEXT)).toBe(19)
    // 前に足した
    expect(bookmarkPosition(bookmark, 'NEW TEXT\n' + TEXT)).toBe(28)
    // 後ろの文字が無ければ, 前の文字だけで探す (前を消した)
    const changed = TEXT.replace('tuv', 'TUV').slice(3)
    expect(bookmarkPosition(bookmark, changed)).toBe(16)
    // どちらも無ければ, 覚えた位置 (本文より長ければ, 終わり)
    expect(bookmarkPosition(bookmark, 'short')).toBe(5)
    expect(bookmarkPosition(bookmark, 'z'.repeat(30))).toBe(19)
    // 位置が, 前の文字の長さより前なら, 前の文字は探さない
    expect(bookmarkPosition({ ...bookmark, position: 0, front: '' }, TEXT)).toBe(0)
    // 前後の文字が空 (空のファイル)
    expect(bookmarkPosition(makeBookmark('e', '/e', '', 0), 'abc')).toBe(0)
  })

  it('保存したファイルが無ければ, 空の一覧を読むこと', async () => {
    mocked.pathExists.mockResolvedValue(false)
    expect(await loadBookmarks()).toEqual([])
    expect(mockedInvoke).toHaveBeenCalledWith('bookmark_file_path')
  })

  it('一覧を JSON で保存し, 読むこと', async () => {
    const list = [makeBookmark('a', '/a', 'abc', 1)]
    await saveBookmarks(list)
    const [path, content] = mocked.writeFile.mock.calls[0]
    expect(path).toBe('/home/u/.config/rseditor/bookmarks')
    mocked.pathExists.mockResolvedValue(true)
    mocked.readFile.mockResolvedValue(content)
    expect(await loadBookmarks()).toEqual(list)
  })
})
