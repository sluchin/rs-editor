/**
 * モードライン (ウィンドウの下の, バッファの状態の行).
 */

import '../styles/ModeLine.css'

/** カーソルの位置. */
export interface CursorInfo {
  /** 行 (1 から数える). */
  line: number
  /** 桁 (1 から数える). */
  column: number
}

/** {@link ModeLine} のプロパティ. */
interface ModeLineProps {
  /** バッファの名前. */
  bufferName: string
  /** 保存してから書き換えたなら `true` (`**` を出す). */
  modified: boolean
  /** 読み取り専用なら `true` (`%%` か `%*` を出す). 省くと `false`. */
  readOnly?: boolean
  /** カーソルの位置 (行と桁を出す). */
  cursor: CursorInfo
  /** メジャーモードの名前. 省くと `Scheme`. */
  mode?: string
  /** 行を出すなら `true` (`line-number-mode`. 省くと `true`). */
  lineNumber?: boolean
  /** 桁を出すなら `true` (`column-number-mode`. 省くと `true`). */
  columnNumber?: boolean
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
  readOnly = false,
  cursor,
  mode = 'Scheme',
  lineNumber = true,
  columnNumber = true,
  bufferCount = 1,
  currentBufferIndex = 1,
  active = true,
}: ModeLineProps) {
  const status = readOnly ? (modified ? '%*' : '%%') : modified ? '**' : '--'
  const bufferIndicator = bufferCount > 1 ? ` [${currentBufferIndex}/${bufferCount}]` : ''
  const position = [lineNumber ? `L${cursor.line}` : '', columnNumber ? `C${cursor.column}` : '']
    .filter(Boolean)
    .join(' ')

  return (
    <div className={active ? 'mode-line' : 'mode-line mode-line-inactive'}>
      -:{status}- {bufferName}
      {bufferIndicator} All {position ? `${position} ` : ''}({mode})
    </div>
  )
}
