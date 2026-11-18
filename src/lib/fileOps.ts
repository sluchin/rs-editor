/**
 * ファイルとディレクトリ (ホームディレクトリ, ディレクトリの中, プロジェクトの根). バックエンド (Tauri のコマンド) に頼みます.
 */

import { invoke } from '@tauri-apps/api/core'

/**
 * ホームディレクトリを返します.
 *
 * @returns ホームディレクトリのパス.
 */
export async function getHomeDir(): Promise<string> {
  return await invoke<string>('home_dir')
}

/**
 * ディレクトリの中の名前を返します (ディレクトリには, 後ろに `/` が付く).
 *
 * @param path - ディレクトリのパス.
 * @returns 名前の順の, 名前.
 */
export async function listDirectory(path: string): Promise<string[]> {
  return await invoke<string[]>('list_directory', { path })
}

/**
 * プロジェクトの根 (`.git` のある, 上のディレクトリ. 無ければそのディレクトリ) を返します.
 *
 * @param directory - ディレクトリ.
 * @returns プロジェクトの根.
 */
export async function projectRoot(directory: string): Promise<string> {
  return await invoke<string>('project_root', { directory })
}

/**
 * ファイルかディレクトリを作ります (同じ名前があれば失敗する).
 *
 * @param path - 作るパス.
 * @param directory - ディレクトリを作るなら真, 空のファイルを作るなら偽.
 */
export async function createPath(path: string, directory: boolean): Promise<void> {
  await invoke('create_path', { path, directory })
}

/**
 * ファイルかディレクトリの名前を変えます (変え先に同じ名前があれば失敗する).
 *
 * @param from - 今のパス.
 * @param to - 変えたあとのパス.
 */
export async function renamePath(from: string, to: string): Promise<void> {
  await invoke('rename_path', { from, to })
}

/**
 * ファイルか, ディレクトリ (中身ごと) を消します.
 *
 * @param path - 消すパス.
 */
export async function deletePath(path: string): Promise<void> {
  await invoke('delete_path', { path })
}
