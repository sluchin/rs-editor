import '../styles/ModeLine.css'
import { CursorInfo } from './CodeEditor'

interface ModeLineProps {
  bufferName: string
  modified: boolean
  cursor: CursorInfo
  mode?: string
}

export default function ModeLine({ bufferName, modified, cursor, mode = 'Scheme' }: ModeLineProps) {
  const status = modified ? '**' : '--'
  return (
    <div className="mode-line">
      -:{status}-  {bufferName}      All L{cursor.line}     ({mode})
    </div>
  )
}
