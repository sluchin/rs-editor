/**
 * ミニバッファの状態と, 入力を待っているコマンドと, 入力の履歴. コマンドの組 (`CommandSet`) の型も置きます.
 */

import { type SetStateAction, useCallback, useRef, useState } from 'react'
import type { MinibufferState } from '../components/Minibuffer'
import { completeFileName } from '../lib/complete'
import { listDirectory } from '../lib/fileOps'
import { HISTORY_LENGTH, addToHistory } from '../lib/history'

/** ミニバッファの入力を待っているコマンド (入力を確定したときに行う). 待っていなければ `null`. */
export type PendingAction =
  | 'find-file'
  | 'write-file'
  | 'execute-command'
  | 'switch-buffer'
  | 'kill-buffer'
  | 'eval-expression'
  | 'goto-line'
  | 'confirm-exit'
  | 'replace-from'
  | 'replace-to'
  | 'string-rectangle'
  | 'bookmark-set'
  | 'bookmark-jump'
  | 'insert-file'
  | 'find-alternate-file'
  | 'confirm-alternate'
  | 'confirm-revert'
  | 'confirm-recover'
  | 'dired'
  | 'read-string'
  | 'grep'
  | 'grep-directory'
  | null

/** 入力を待っているコマンドと, 入力の履歴の名前 (同じ名前のコマンドは, 履歴を分け合う). */
const HISTORY_NAMES: Partial<Record<NonNullable<PendingAction>, string>> = {
  'find-file': 'file-name',
  'write-file': 'file-name',
  'insert-file': 'file-name',
  'find-alternate-file': 'file-name',
  dired: 'file-name',
  'execute-command': 'extended-command',
  'switch-buffer': 'buffer-name',
  'eval-expression': 'read-expression',
  'goto-line': 'goto-line',
  'replace-from': 'query-replace',
  'replace-to': 'query-replace',
  'string-rectangle': 'string-rectangle',
  'bookmark-set': 'bookmark',
  'bookmark-jump': 'bookmark',
  'kill-buffer': 'yes-or-no-p',
  'confirm-exit': 'yes-or-no-p',
  'confirm-alternate': 'yes-or-no-p',
  'confirm-revert': 'yes-or-no-p',
  'confirm-recover': 'yes-or-no-p',
  'read-string': 'minibuffer',
  grep: 'grep',
  'grep-directory': 'file-name',
}

/** ミニバッファの入力を確定したときに, 入力を待っていたコマンドごとに行う関数. */
export type Submitters = Partial<
  Record<NonNullable<PendingAction>, (value: string) => void | Promise<void>>
>

/** 画面のコマンドの組 (機能ごとのフックが返す). */
export interface CommandSet {
  /** コマンド (キーと M-x から, 名前で呼ぶ). */
  actions: Record<string, () => void>
  /** ミニバッファの入力を確定したときに, 入力を待っていたコマンドごとに行う関数. */
  submit?: Submitters
}

/**
 * ファイルの名前を補完します (`C-x C-f`, 保存するファイル).
 *
 * @param input - 入力 (パス).
 * @returns 補完した入力と, 候補.
 */
export const completeFile = (input: string) => completeFileName(input, listDirectory)

/**
 * ミニバッファの状態と, 入力を待っているコマンドと, 入力の履歴を持ちます.
 *
 * @returns 状態と, 変える関数.
 */
export function useMinibuffer() {
  // ミニバッファの入力を待っているコマンドと, ミニバッファの状態
  const [pendingAction, setPendingAction] = useState<PendingAction>(null)
  const [minibufferState, setState] = useState<MinibufferState>({
    mode: 'message',
    text: '',
  })
  // ミニバッファを使っているか (状態を変えたら, 描き直す前でも, すぐに変わる.
  // M-x から呼んだコマンドが, M-x の入力を閉じたことを知れるように)
  const active = useRef(false)

  /**
   * ミニバッファの状態を変えます. 関数で変えるときは, 使っているかを変えないこと.
   *
   * @param state - 新しい状態 (か, 前の状態から作る関数).
   */
  const setMinibufferState = useCallback((state: SetStateAction<MinibufferState>) => {
    if (typeof state !== 'function') active.current = state.mode !== 'message'
    setState(state)
  }, [])

  /**
   * ミニバッファを使っているか (メッセージでなく, 入力や問いを出しているか) を返します.
   *
   * @returns 使っていれば `true`.
   */
  const isMinibufferActive = () => active.current
  // ミニバッファの入力の履歴 (履歴の名前ごと. 新しい順)
  const [histories, setHistories] = useState<Record<string, string[]>>({})

  /**
   * ミニバッファに, メッセージを出します.
   *
   * @param text - メッセージ.
   */
  const showMessage = (text: string) => {
    setMinibufferState({ mode: 'message', text })
  }

  /**
   * 入力を待っているコマンドの, 入力の履歴に加えます.
   *
   * @param action - 入力を待っていたコマンド.
   * @param value - 入力.
   */
  const addHistory = (action: PendingAction, value: string) => {
    const historyName = action && HISTORY_NAMES[action]
    if (historyName) {
      setHistories((prev) => ({
        ...prev,
        [historyName]: addToHistory(prev[historyName] ?? [], value, HISTORY_LENGTH),
      }))
    }
  }

  // 入力を待っているコマンドの, 入力の履歴
  const historyName = pendingAction ? HISTORY_NAMES[pendingAction] : undefined
  const history = histories[historyName as string]

  return {
    pendingAction,
    setPendingAction,
    minibufferState,
    setMinibufferState,
    isMinibufferActive,
    showMessage,
    addHistory,
    history,
  }
}

/** ミニバッファの状態と, 変える関数. */
export type MinibufferContext = ReturnType<typeof useMinibuffer>
