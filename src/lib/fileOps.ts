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

/** `M-x grep` で, 合った行. */
export interface GrepMatch {
  /** ファイル (探したディレクトリからのパス). */
  file: string
  /** 行の番号 (1 から). */
  line: number
  /** 行. */
  text: string
}

/**
 * ディレクトリの下のファイルから, 正規表現 (Rust の regex の書き方) に合う行を探します.
 * 正規表現に大文字が無ければ, 大文字と小文字を区別しません.
 *
 * @param pattern - 正規表現.
 * @param directory - ディレクトリ.
 * @returns 合った行 (ファイルの名前の順, 行の順).
 */
export async function grepDirectory(pattern: string, directory: string): Promise<GrepMatch[]> {
  return await invoke<GrepMatch[]>('grep_directory', { pattern, directory })
}

/**
 * プロジェクトの根 (`.git` のある, 上のディレクトリ. 無ければ, そのディレクトリ) を返します.
 *
 * @param directory - ディレクトリ.
 * @returns プロジェクトの根.
 */
export async function projectRoot(directory: string): Promise<string> {
  return await invoke<string>('project_root', { directory })
}
