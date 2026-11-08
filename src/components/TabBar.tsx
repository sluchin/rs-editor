/**
 * タブの行 (Emacs の tab-bar). タブが 2 つ以上あるときだけ出す. 押すと, そのタブに切り替える.
 */

import type { ReactElement } from 'react'
import '../styles/TabBar.css'

/** タブの行の引数. */
interface TabBarProps {
  /** タブの名前. */
  tabs: string[]
  /** 選んでいるタブの番号 (0 から). */
  current: number
  /** タブを押したときに呼ぶ関数 (番号は 1 から). */
  onSelect: (number: number) => void
  /** 「+」を押したときに呼びます (新しいタブ). */
  onNew: () => void
}

/**
 * タブの行.
 *
 * @param props - 引数.
 * @returns 画面 (タブが 2 つ未満なら何も出さない).
 */
export default function TabBar({
  tabs,
  current,
  onSelect,
  onNew,
}: TabBarProps): ReactElement | null {
  if (tabs.length < 2) return null
  return (
    // 押してもフォーカスを取らない (キーは, エディタのまま)
    <div className="tab-bar" role="tablist" onMouseDown={(e) => e.preventDefault()}>
      {tabs.map((name, index) => (
        <button
          key={index}
          role="tab"
          aria-selected={index === current}
          className={`tab-bar-tab${index === current ? ' tab-bar-current' : ''}`}
          title={name}
          onClick={() => onSelect(index + 1)}
        >
          {name}
        </button>
      ))}
      <button className="tab-bar-new" title="New tab" onClick={onNew}>
        +
      </button>
    </div>
  )
}
