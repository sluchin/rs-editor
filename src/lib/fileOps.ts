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
