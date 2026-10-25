/**
 * ブックマークのコマンド (C-x r m, C-x r b). ブックマークの一覧を, 始めに読み, 付けるたびに保存します.
 */

import { useEffect, useRef, useState } from 'react'
import type { EditorHandle } from '../components/CodeEditor'
import {
  type Bookmark,
  bookmarkPosition,
  loadBookmarks,
  makeBookmark,
  saveBookmarks,
  storeBookmark,
} from '../lib/bookmarks'
import { bufferNameFromPath } from '../lib/buffer'
import { readFile } from '../lib/fileOps'
import type { AppContext } from './useBuffers'
import type { CommandSet } from './useMinibuffer'

/**
 * ブックマークのコマンドを作ります.
 *
 * @param ctx - ミニバッファと, バッファとウィンドウ.
 * @returns コマンド.
 */
export function useBookmarks(ctx: AppContext): CommandSet {
  const {
    buffers,
    currentBuffer,
    currentBufferId,
    editor,
    setPendingPoint,
    setCurrentBufferId,
    createNewBuffer,
    setPendingAction,
    setMinibufferState,
    showMessage,
  } = ctx
  // ブックマークの一覧 (新しい順) と, 最後に使ったブックマークの名前 (C-x r b の既定)
  const [bookmarks, setBookmarks] = useState<Bookmark[]>([])
  const lastBookmark = useRef<string | null>(null)

  // 保存したブックマークを, 始めに 1 回だけ読む
  useEffect(() => {
    loadBookmarks()
      .then(setBookmarks)
      .catch((err) => setMinibufferState({ mode: 'message', text: `Bookmark file error: ${err}` }))
  }, [setMinibufferState])

  /**
   * ブックマークの名前を, ミニバッファで聞きます (C-x r m). 既定は, ファイルの名前です.
   */
  const handleBookmarkSet = () => {
    if (!currentBuffer.filePath) {
      showMessage('Buffer not visiting a file or directory')
      return
    }
    setPendingAction('bookmark-set')
    setMinibufferState({
      mode: 'input',
      prompt: `Set bookmark named (default ${bufferNameFromPath(currentBuffer.filePath)}): `,
      input: '',
      completions: bookmarks.map((b) => b.name),
    })
  }

  /**
   * 移るブックマークの名前を, ミニバッファで聞きます (C-x r b). 既定は, 最後に使ったブックマークです.
   */
  const handleBookmarkJump = () => {
    const last = lastBookmark.current
    setPendingAction('bookmark-jump')
    setMinibufferState({
      mode: 'input',
      prompt: last === null ? 'Jump to bookmark: ' : `Jump to bookmark (default ${last}): `,
      input: '',
      completions: bookmarks.map((b) => b.name),
    })
  }

  /**
   * ブックマークを付けて, 保存します.
   *
   * @param value - 入力した名前 (空なら, ファイルの名前).
   */
  const setBookmark = async (value: string) => {
    const filePath = currentBuffer.filePath as string
    const name = value === '' ? bufferNameFromPath(filePath) : value
    const handle = editor.current as EditorHandle
    const next = storeBookmark(
      bookmarks,
      makeBookmark(name, filePath, handle.text(), handle.point()),
    )
    setBookmarks(next)
    lastBookmark.current = name
    showMessage('')
    try {
      await saveBookmarks(next)
    } catch (err) {
      showMessage(`Error: ${err}`)
    }
  }

  /**
   * ブックマークのファイルを出して, 位置へ動きます. ファイルを開いたバッファがあれば, それを使います.
   *
   * @param value - 入力した名前 (空なら, 最後に使ったブックマーク).
   */
  const jumpToBookmark = async (value: string) => {
    const name = value === '' ? lastBookmark.current : value
    const bookmark = bookmarks.find((b) => b.name === name)
    if (!bookmark) {
      showMessage(name ? `Invalid bookmark ${name}` : 'No bookmark specified')
      return
    }
    lastBookmark.current = bookmark.name
    showMessage('')
    const open = buffers.find((b) => b.filePath === bookmark.filename)
    if (open) {
      const pos = bookmarkPosition(bookmark, open.content)
      if (open.id === currentBufferId) {
        editor.current?.setCursor(pos)
      } else {
        setPendingPoint(pos)
        setCurrentBufferId(open.id)
      }
      return
    }
    try {
      const content = await readFile(bookmark.filename)
      setPendingPoint(bookmarkPosition(bookmark, content))
      createNewBuffer(bufferNameFromPath(bookmark.filename), bookmark.filename, content)
    } catch (err) {
      showMessage(`Error: ${err}`)
    }
  }

  return {
    actions: {
      'bookmark-set': handleBookmarkSet,
      'bookmark-jump': handleBookmarkJump,
    },
    submit: {
      'bookmark-set': setBookmark,
      'bookmark-jump': jumpToBookmark,
    },
  }
}
