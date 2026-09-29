import { describe, it, expect, vi, beforeEach } from 'vitest'
import { readFile, writeFile, pathExists, getHomeDir } from '../lib/fileOps'
import * as tauri from '@tauri-apps/api/core'

vi.mock('@tauri-apps/api/core')

describe('fileOps', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  describe('readFile', () => {
    it('reads file content from Tauri', async () => {
      const mockContent = 'file content'
      vi.mocked(tauri.invoke).mockResolvedValueOnce(mockContent)

      const result = await readFile('/path/to/file.txt')

      expect(tauri.invoke).toHaveBeenCalledWith('read_file_content', { path: '/path/to/file.txt' })
      expect(result).toBe(mockContent)
    })

    it('handles file read errors', async () => {
      const error = new Error('File not found')
      vi.mocked(tauri.invoke).mockRejectedValueOnce(error)

      await expect(readFile('/nonexistent.txt')).rejects.toThrow('File not found')
    })
  })

  describe('writeFile', () => {
    it('writes file content via Tauri', async () => {
      vi.mocked(tauri.invoke).mockResolvedValueOnce(undefined)

      await writeFile('/path/to/file.txt', 'new content')

      expect(tauri.invoke).toHaveBeenCalledWith('write_file_content', {
        path: '/path/to/file.txt',
        content: 'new content',
      })
    })

    it('handles write errors', async () => {
      const error = new Error('Permission denied')
      vi.mocked(tauri.invoke).mockRejectedValueOnce(error)

      await expect(writeFile('/protected.txt', 'content')).rejects.toThrow('Permission denied')
    })

    it('writes empty content', async () => {
      vi.mocked(tauri.invoke).mockResolvedValueOnce(undefined)

      await writeFile('/file.txt', '')

      expect(tauri.invoke).toHaveBeenCalledWith('write_file_content', {
        path: '/file.txt',
        content: '',
      })
    })
  })

  describe('pathExists', () => {
    it('checks if path exists', async () => {
      vi.mocked(tauri.invoke).mockResolvedValueOnce(true)

      const result = await pathExists('/some/path')

      expect(tauri.invoke).toHaveBeenCalledWith('path_exists', { path: '/some/path' })
      expect(result).toBe(true)
    })

    it('returns false for nonexistent paths', async () => {
      vi.mocked(tauri.invoke).mockResolvedValueOnce(false)

      const result = await pathExists('/nonexistent/path')

      expect(result).toBe(false)
    })

    it('handles errors checking path existence', async () => {
      const error = new Error('Permission denied')
      vi.mocked(tauri.invoke).mockRejectedValueOnce(error)

      await expect(pathExists('/restricted/path')).rejects.toThrow('Permission denied')
    })
  })

  describe('getHomeDir', () => {
    it('gets home directory path', async () => {
      vi.mocked(tauri.invoke).mockResolvedValueOnce('/home/user')

      const result = await getHomeDir()

      expect(tauri.invoke).toHaveBeenCalledWith('home_dir')
      expect(result).toBe('/home/user')
    })

    it('handles home dir lookup errors', async () => {
      const error = new Error('Failed to get home directory')
      vi.mocked(tauri.invoke).mockRejectedValueOnce(error)

      await expect(getHomeDir()).rejects.toThrow('Failed to get home directory')
    })
  })
})
