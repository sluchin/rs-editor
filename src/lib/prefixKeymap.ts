import { useRef, KeyboardEvent } from 'react'

interface PrefixKeymapOptions {
  onFindFile: () => void
  onSaveBuffer: () => void
  onRedo?: () => void
  onSwitchBuffer?: () => void
  onKillBuffer?: () => void
  onListBuffers?: () => void
}

/**
 * Handles the Emacs C-x prefix key sequences:
 * C-x C-f (find-file), C-x C-s (save-buffer), C-x C-/ (redo), C-x h (mark-whole-buffer),
 * C-x b (switch-to-buffer), C-x k (kill-buffer)
 */
export function usePrefixKeymap({
  onFindFile,
  onSaveBuffer,
  onRedo,
  onSwitchBuffer,
  onKillBuffer,
  onListBuffers,
}: PrefixKeymapOptions) {
  const awaitingX = useRef(false)

  const handleKeyDown = (e: KeyboardEvent<HTMLTextAreaElement>): boolean => {
    if (e.ctrlKey && e.key === 'x') {
      e.preventDefault()
      awaitingX.current = true
      return true
    }

    if (awaitingX.current) {
      awaitingX.current = false
      if (e.ctrlKey && e.key === 'f') {
        e.preventDefault()
        onFindFile()
        return true
      }
      if (e.ctrlKey && e.key === 's') {
        e.preventDefault()
        onSaveBuffer()
        return true
      }
      if (e.ctrlKey && e.key === '/') {
        e.preventDefault()
        onRedo?.()
        return true
      }
      if (e.ctrlKey && e.key === 'b') {
        e.preventDefault()
        onListBuffers?.()
        return true
      }
      if (e.key === 'h') {
        e.preventDefault()
        const el = e.currentTarget as HTMLTextAreaElement
        el.setSelectionRange(0, el.value.length)
        return true
      }
      if (e.key === 'b') {
        e.preventDefault()
        onSwitchBuffer?.()
        return true
      }
      if (e.key === 'k') {
        e.preventDefault()
        onKillBuffer?.()
        return true
      }
    }

    return false
  }

  return { handleKeyDown }
}
