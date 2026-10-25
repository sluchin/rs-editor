/**
 * モードライン (ウィンドウの下の, バッファの状態の行).
 */

import '../styles/ModeLine.css'
import { CursorInfo } from './CodeEditor'

/** {@link ModeLine} のプロパティ. */
interface ModeLineProps {
  /** バッファの名前. */
  bufferName: string
  /** 保存してから書き換えたなら `true` (`**` を出す). */
  modified: boolean
  /** カーソルの位置 (行と桁を出す). */
  cursor: CursorInfo
  /** メジャーモードの名前. 省くと `Scheme`. */
  mode?: string
  /** バッファの数. 2 つ以上なら, 何番目のバッファかを出す. 省くと 1. */
  bufferCount?: number
  /** 何番目のバッファか (1 から数える). 省くと 1. */
  currentBufferIndex?: number
  /** 選んでいるウィンドウのモードラインなら `true` (省くと `true`). */
  active?: boolean
}

/**
 * モードラインを出します (`-:**- 名前  All L1 C1 (モード)` のように, Emacs と同じ形).
 *
 * @param props - プロパティ.
 * @returns モードライン.
 */
export default function ModeLine({
  bufferName,
  modified,
  cursor,
  mode = 'Scheme',
  bufferCount = 1,
  currentBufferIndex = 1,
  active = true,
}: ModeLineProps) {
  const status = modified ? '**' : '--'
  const bufferIndicator = bufferCount > 1 ? ` [${currentBufferIndex}/${bufferCount}]` : ''

  return (
    <div className={active ? 'mode-line' : 'mode-line mode-line-inactive'}>
      -:{status}- {bufferName}
      {bufferIndicator} All L{cursor.line} C{cursor.column} ({mode})
    </div>
  )
}
