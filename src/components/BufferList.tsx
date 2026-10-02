import { Buffer } from '../App'
import '../styles/BufferList.css'

interface BufferListProps {
  buffers: Buffer[]
  currentBufferId: string
  onSelectBuffer: (bufferId: string) => void
  onClose: () => void
}

export default function BufferList({
  buffers,
  currentBufferId,
  onSelectBuffer,
  onClose,
}: BufferListProps) {
  return (
    <div className="buffer-list-overlay" onClick={onClose}>
      <div className="buffer-list-container" onClick={(e) => e.stopPropagation()}>
        <div className="buffer-list-header">
          <h2>Buffer List</h2>
          <button onClick={onClose} className="close-button">
            ×
          </button>
        </div>
        <div className="buffer-list-content">
          <table className="buffer-table">
            <thead>
              <tr>
                <th className="buffer-status"></th>
                <th>Buffer</th>
                <th>Size</th>
                <th>Mode</th>
                <th>File</th>
              </tr>
            </thead>
            <tbody>
              {buffers.map((buffer) => (
                <tr
                  key={buffer.id}
                  className={`buffer-row ${buffer.id === currentBufferId ? 'current' : ''}`}
                  onClick={() => onSelectBuffer(buffer.id)}
                >
                  <td className="buffer-status">
                    <span className="status-indicator">
                      {buffer.id === currentBufferId ? '>' : ' '}
                    </span>
                    {buffer.modified ? '*' : ' '}
                  </td>
                  <td className="buffer-name">{buffer.name}</td>
                  <td className="buffer-size">{buffer.content.length}</td>
                  <td className="buffer-mode">Lisp</td>
                  <td className="buffer-file">{buffer.filePath || '--'}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
        <div className="buffer-list-footer">
          <p>Click to select buffer, or press q to close</p>
        </div>
      </div>
    </div>
  )
}
