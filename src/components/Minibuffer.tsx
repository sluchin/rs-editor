import '../styles/Minibuffer.css'

interface MinibufferProps {
  message?: string
}

const DEFAULT_MESSAGE = 'For information about this editor, see the README.'

export default function Minibuffer({ message }: MinibufferProps) {
  return <div className="minibuffer">{message || DEFAULT_MESSAGE}</div>
}
