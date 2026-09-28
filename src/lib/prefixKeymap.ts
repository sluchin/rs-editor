import { useRef, KeyboardEvent } from 'react'

interface PrefixKeymapOptions {
  onFindFile: () => void
  onSaveBuffer: () => void
}

/**
 * Handles the Emacs C-x prefix key sequences relevant to file operations:
 * C-x C-f (find-file) and C-x C-s (save-buffer).
 */
export function usePrefixKeymap({ onFindFile, onSaveBuffer }: PrefixKeymapOptions) {
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
    }

    return false
  }

  return { handleKeyDown }
}
