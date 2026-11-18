import { describe, it, expect, vi, beforeEach } from 'vitest'
import {
  createPath,
  deletePath,
  getHomeDir,
  listDirectory,
  projectRoot,
  renamePath,
} from '@/lib/fileOps'
import * as tauri from '@tauri-apps/api/core'

vi.mock('@tauri-apps/api/core')

describe('fileOps', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('ホームディレクトリを Tauri に聞き, 失敗を返すこと', async () => {
    vi.mocked(tauri.invoke).mockResolvedValueOnce('/home/user')
    expect(await getHomeDir()).toBe('/home/user')
    expect(tauri.invoke).toHaveBeenCalledWith('home_dir')
    vi.mocked(tauri.invoke).mockRejectedValueOnce(new Error('No home directory'))
    await expect(getHomeDir()).rejects.toThrow('No home directory')
  })

  it('ディレクトリの一覧を Tauri に頼むこと', async () => {
    vi.mocked(tauri.invoke).mockResolvedValueOnce(['a', 'b/'])
    expect(await listDirectory('/tmp')).toEqual(['a', 'b/'])
    expect(tauri.invoke).toHaveBeenCalledWith('list_directory', { path: '/tmp' })
  })

  it('プロジェクトの根を聞くこと', async () => {
    vi.mocked(tauri.invoke).mockResolvedValueOnce('/proj')
    expect(await projectRoot('/proj/src')).toBe('/proj')
    expect(tauri.invoke).toHaveBeenCalledWith('project_root', { directory: '/proj/src' })
  })

  it('ファイルの作成, 名前の変更, 削除を Tauri に頼むこと', async () => {
    vi.mocked(tauri.invoke).mockResolvedValue(undefined)
    await createPath('/p/a.txt', false)
    expect(tauri.invoke).toHaveBeenCalledWith('create_path', { path: '/p/a.txt', directory: false })
    await renamePath('/p/a.txt', '/p/b.txt')
    expect(tauri.invoke).toHaveBeenCalledWith('rename_path', { from: '/p/a.txt', to: '/p/b.txt' })
    await deletePath('/p/b.txt')
    expect(tauri.invoke).toHaveBeenCalledWith('delete_path', { path: '/p/b.txt' })
  })
})
