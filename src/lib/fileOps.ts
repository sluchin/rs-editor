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

/**
 * ファイルのバックアップ (`<path>~`) を作ります. ファイルが無ければ, 何もしません.
 *
 * @param path - ファイルのパス.
 */
export async function backupFile(path: string): Promise<void> {
  await invoke('backup_file', { path })
}

/**
 * ファイルを消します. 無ければ, 何もしません.
 *
 * @param path - ファイルのパス.
 */
export async function deleteFile(path: string): Promise<void> {
  await invoke('delete_file', { path })
}

/**
 * 1 つ目のファイルが, 2 つ目のファイルより新しいかを返します (1 つ目だけがあるときも `true`).
 *
 * @param file1 - 1 つ目のファイル.
 * @param file2 - 2 つ目のファイル.
 * @returns 新しければ `true`.
 */
export async function fileNewerThan(file1: string, file2: string): Promise<boolean> {
  return await invoke<boolean>('file_newer_than', { file1, file2 })
}

/**
 * ディレクトリの一覧を, `ls -al` で作ります.
 *
 * @param path - ディレクトリのパス.
 * @returns `ls` の出力.
 */
export async function listDirectoryLong(path: string): Promise<string> {
  return await invoke<string>('list_directory_long', { path })
}
