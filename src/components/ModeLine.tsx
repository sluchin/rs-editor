import '../styles/ModeLine.css'
import { CursorInfo } from './CodeEditor'

interface ModeLineProps {
  bufferName: string
  modified: boolean
  cursor: CursorInfo
  mode?: string
  bufferCount?: number
  currentBufferIndex?: number
  /** 選んでいるウィンドウのモードラインなら `true` (省くと `true`). */
  active?: boolean
}

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
