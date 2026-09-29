import { describe, it, expect, vi, beforeEach } from 'vitest'
import { readFile, writeFile, pathExists, getHomeDir } from '../lib/fileOps'
import * as tauri from '@tauri-apps/api/core'

vi.mock('@tauri-apps/api/core')

/**
 * ファイル操作ユーティリティ関数のテストスイート.
 * Tauri API 経由のファイルI/O操作をテストします.
 */
describe('fileOps', () => {
  beforeEach(() => {
    // 各テストの前にモックをリセット.
    vi.clearAllMocks()
  })

  describe('readFile', () => {
    // ファイル読み込み機能をテストします.

    it('reads file content from Tauri', async () => {
      // Tauri の read_file_content コマンドでファイルを読むことを確認.
      const mockContent = 'file content'
      vi.mocked(tauri.invoke).mockResolvedValueOnce(mockContent)

      const result = await readFile('/path/to/file.txt')

      expect(tauri.invoke).toHaveBeenCalledWith('read_file_content', { path: '/path/to/file.txt' })
      expect(result).toBe(mockContent)
    })

    it('handles file read errors', async () => {
      // ファイル読み込みエラーが適切に処理されることを確認.
      const error = new Error('File not found')
      vi.mocked(tauri.invoke).mockRejectedValueOnce(error)

      await expect(readFile('/nonexistent.txt')).rejects.toThrow('File not found')
    })
  })

  describe('writeFile', () => {
    // ファイル書き込み機能をテストします.

    it('writes file content via Tauri', async () => {
      // Tauri の write_file_content コマンドでファイルを書くことを確認.
      vi.mocked(tauri.invoke).mockResolvedValueOnce(undefined)

      await writeFile('/path/to/file.txt', 'new content')

      expect(tauri.invoke).toHaveBeenCalledWith('write_file_content', {
        path: '/path/to/file.txt',
        content: 'new content',
      })
    })

    it('handles write errors', async () => {
      // ファイル書き込みエラーが適切に処理されることを確認.
      const error = new Error('Permission denied')
      vi.mocked(tauri.invoke).mockRejectedValueOnce(error)

      await expect(writeFile('/protected.txt', 'content')).rejects.toThrow('Permission denied')
    })

    it('writes empty content', async () => {
      // 空の内容を書き込むことができることを確認.
      vi.mocked(tauri.invoke).mockResolvedValueOnce(undefined)

      await writeFile('/file.txt', '')

      expect(tauri.invoke).toHaveBeenCalledWith('write_file_content', {
        path: '/file.txt',
        content: '',
      })
    })
  })

  describe('pathExists', () => {
    // パス存在確認機能をテストします.

    it('checks if path exists', async () => {
      // Tauri の path_exists コマンドでパス存在を確認することを確認.
      vi.mocked(tauri.invoke).mockResolvedValueOnce(true)

      const result = await pathExists('/some/path')

      expect(tauri.invoke).toHaveBeenCalledWith('path_exists', { path: '/some/path' })
      expect(result).toBe(true)
    })

    it('returns false for nonexistent paths', async () => {
      // 存在しないパスに対して false を返すことを確認.
      vi.mocked(tauri.invoke).mockResolvedValueOnce(false)

      const result = await pathExists('/nonexistent/path')

      expect(result).toBe(false)
    })

    it('handles errors checking path existence', async () => {
      // パス存在確認エラーが適切に処理されることを確認.
      const error = new Error('Permission denied')
      vi.mocked(tauri.invoke).mockRejectedValueOnce(error)

      await expect(pathExists('/restricted/path')).rejects.toThrow('Permission denied')
    })
  })

  describe('getHomeDir', () => {
    // ホームディレクトリ取得機能をテストします.

    it('gets home directory path', async () => {
      // Tauri の home_dir コマンドでホームディレクトリを取得することを確認.
      vi.mocked(tauri.invoke).mockResolvedValueOnce('/home/user')

      const result = await getHomeDir()

      expect(tauri.invoke).toHaveBeenCalledWith('home_dir')
      expect(result).toBe('/home/user')
    })

    it('handles home dir lookup errors', async () => {
      // ホームディレクトリ取得エラーが適切に処理されることを確認.
      const error = new Error('Failed to get home directory')
      vi.mocked(tauri.invoke).mockRejectedValueOnce(error)

      await expect(getHomeDir()).rejects.toThrow('Failed to get home directory')
    })
  })
})
