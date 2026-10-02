import '../styles/ModeLine.css'
import { CursorInfo } from './CodeEditor'

interface ModeLineProps {
  bufferName: string
  modified: boolean
  cursor: CursorInfo
  mode?: string
  bufferCount?: number
  currentBufferIndex?: number
}

export default function ModeLine({
  bufferName,
  modified,
  cursor,
  mode = 'Scheme',
  bufferCount = 1,
  currentBufferIndex = 1,
}: ModeLineProps) {
  const status = modified ? '**' : '--'
  const bufferIndicator = bufferCount > 1 ? ` [${currentBufferIndex}/${bufferCount}]` : ''

  return (
    <div className="mode-line">
      -:{status}- {bufferName}
      {bufferIndicator} All L{cursor.line} C{cursor.column} ({mode})
    </div>
  )
}
