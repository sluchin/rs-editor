import { describe, it, expect, vi } from 'vitest'
import { fireEvent, render } from '@testing-library/react'
import TabBar from '@/components/TabBar'

describe('TabBar', () => {
  it('タブが 2 つ未満なら何も出さないこと', () => {
    const view = render(<TabBar tabs={['a']} current={0} onSelect={vi.fn()} onNew={vi.fn()} />)
    expect(view.container.innerHTML).toBe('')
  })

  it('タブを並べ、選んでいるタブを示し、押すと番号 (1 から) を知らせること', () => {
    const onSelect = vi.fn()
    const onNew = vi.fn()
    const view = render(<TabBar tabs={['a', 'b']} current={1} onSelect={onSelect} onNew={onNew} />)
    const tabs = view.getAllByRole('tab')
    expect(tabs.map((t) => t.textContent)).toEqual(['a', 'b'])
    expect(tabs[1].getAttribute('aria-selected')).toBe('true')
    expect(tabs[1].classList).toContain('tab-bar-current')
    expect(tabs[0].getAttribute('aria-selected')).toBe('false')
    fireEvent.click(tabs[0])
    expect(onSelect).toHaveBeenCalledWith(1)
    fireEvent.click(view.getByTitle('New tab'))
    expect(onNew).toHaveBeenCalled()
    // 押してもフォーカスを取らない
    expect(fireEvent.mouseDown(view.getByRole('tablist'))).toBe(false)
  })
})
