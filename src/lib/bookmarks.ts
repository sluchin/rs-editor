/**
 * ブックマーク (Emacs の bookmark.el. `C-x r m`, `C-x r b`). ファイルの名前と位置に, 名前を付けて覚えます.
 *
 * 位置の前後の文字も覚え, 移るときは, ファイルを書き換えていても, その文字を探して位置を直します
 * (Emacs の `bookmark-default-handler`). ブックマークは, 設定のディレクトリのファイルに, JSON で保存します.
 */

import { invoke } from '@tauri-apps/api/core'
import { pathExists, readFile, writeFile } from './fileOps'

/** 位置の前後の, 覚える文字の数 (Emacs の `bookmark-search-size`). */
const CONTEXT_SIZE = 16

/** ブックマーク. */
export interface Bookmark {
  /** 名前. */
  name: string
  /** ファイルのパス. */
  filename: string
  /** 位置 (本文の始めから. 0 から数える). */
  position: number
  /** 位置の後ろの文字 (Emacs の `front-context-string`). */
  front: string
  /** 位置の前の文字 (Emacs の `rear-context-string`). */
  rear: string
}

/**
 * ブックマークを作ります (Emacs の `bookmark-make-record-default`).
 *
 * @param name - 名前.
 * @param filename - ファイルのパス.
 * @param text - 本文.
 * @param position - 位置.
 * @returns ブックマーク.
 */
export function makeBookmark(
  name: string,
  filename: string,
  text: string,
  position: number,
): Bookmark {
  return {
    name,
    filename,
    position,
    front: text.slice(position, position + CONTEXT_SIZE),
    rear: text.slice(Math.max(0, position - CONTEXT_SIZE), position),
  }
}

/**
 * ブックマークを足します. 同じ名前があれば, 置き換えます (Emacs の `bookmark-set`).
 * 新しいものは, 一覧の始めに置きます.
 *
 * @param bookmarks - 一覧.
 * @param bookmark - 足すブックマーク.
 * @returns 新しい一覧.
 */
export function storeBookmark(bookmarks: Bookmark[], bookmark: Bookmark): Bookmark[] {
  const index = bookmarks.findIndex((b) => b.name === bookmark.name)
  if (index < 0) return [bookmark, ...bookmarks]
  return bookmarks.map((b, i) => (i === index ? bookmark : b))
}

/**
 * ブックマークの位置を, 本文に合わせて直します. 覚えた位置から, 後ろの文字を後ろへ探し,
 * 見つかれば, 前の文字を前へ探します (Emacs の `bookmark-default-handler`).
 *
 * @param bookmark - ブックマーク.
 * @param text - 本文.
 * @returns 位置.
 */
export function bookmarkPosition(bookmark: Bookmark, text: string): number {
  let pos = Math.min(bookmark.position, text.length)
  if (bookmark.front) {
    const found = text.indexOf(bookmark.front, pos)
    if (found >= 0) pos = found
  }
  if (bookmark.rear && pos >= bookmark.rear.length) {
    const found = text.lastIndexOf(bookmark.rear, pos - bookmark.rear.length)
    if (found >= 0) pos = found + bookmark.rear.length
  }
  return pos
}

/**
 * 保存したブックマークを読みます. ファイルが無ければ, 空です.
 *
 * @returns 一覧.
 */
export async function loadBookmarks(): Promise<Bookmark[]> {
  const path = await invoke<string>('bookmark_file_path')
  if (!(await pathExists(path))) return []
  return JSON.parse(await readFile(path)) as Bookmark[]
}

/**
 * ブックマークを保存します.
 *
 * @param bookmarks - 一覧.
 */
export async function saveBookmarks(bookmarks: Bookmark[]): Promise<void> {
  const path = await invoke<string>('bookmark_file_path')
  await writeFile(path, JSON.stringify(bookmarks, null, 2) + '\n')
}
