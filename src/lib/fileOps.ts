/**
 * ファイルの読み書き. バックエンド (Tauri のコマンド) に頼みます.
 */

import { invoke } from '@tauri-apps/api/core'

/**
 * ファイルを読みます.
 *
 * @param path - ファイルのパス.
 * @returns ファイルの内容.
 */
export async function readFile(path: string): Promise<string> {
  return await invoke<string>('read_file_content', { path })
}

/**
 * ファイルに書きます (あれば, 置き換えます).
 *
 * @param path - ファイルのパス.
 * @param content - 書く内容.
 */
export async function writeFile(path: string, content: string): Promise<void> {
  await invoke('write_file_content', { path, content })
}

/**
 * パスがあるかを返します.
 *
 * @param path - パス.
 * @returns あれば `true`.
 */
export async function pathExists(path: string): Promise<boolean> {
  return await invoke<boolean>('path_exists', { path })
}

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
