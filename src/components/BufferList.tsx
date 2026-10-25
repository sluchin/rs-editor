/**
 * バッファの一覧 (C-x C-b). 画面の上に重ねて出し, 選んだバッファに切り替えます.
 */

import { useEffect } from 'react'
import { Buffer } from '../App'
import { majorMode } from '../lib/modes'
import '../styles/BufferList.css'

/** {@link BufferList} のプロパティ. */
interface BufferListProps {
  /** 一覧に出すバッファ. */
  buffers: Buffer[]
  /** 選んでいるウィンドウのバッファ (印を付ける). */
  currentBufferId: string
  /** バッファを選んだときに呼びます. */
  onSelectBuffer: (bufferId: string) => void
  /** 一覧を閉じるときに呼びます (外側か, 閉じるボタンを押したとき, `q` を押したとき). */
  onClose: () => void
}

/**
 * バッファの一覧を出します.
 *
 * @param props - プロパティ.
 * @returns バッファの一覧.
 */
export default function BufferList({
  buffers,
  currentBufferId,
  onSelectBuffer,
  onClose,
}: BufferListProps) {
  // `q` で閉じる (Emacs の Buffer Menu と同じ). 修飾キーと一緒なら, ほかのコマンドなので閉じない
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'q' && !e.ctrlKey && !e.altKey && !e.metaKey) {
        e.preventDefault()
        onClose()
      }
    }
    window.addEventListener('keydown', handleKeyDown)
    return () => window.removeEventListener('keydown', handleKeyDown)
  }, [onClose])

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
                  <td className="buffer-mode">
                    {majorMode(buffer.name, buffer.filePath, buffer.content).name}
                  </td>
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
