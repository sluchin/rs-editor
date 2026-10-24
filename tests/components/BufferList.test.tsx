import { describe, it, expect, vi } from 'vitest'
import { render, screen, fireEvent } from '@testing-library/react'
import BufferList from '@/components/BufferList'

describe('BufferList Component', () => {
  const buffers = [
    { id: '1', name: 'a.scm', filePath: '/tmp/a.scm', content: '(+ 1 2)', modified: true },
    { id: '2', name: '*scratch*', filePath: null, content: '', modified: false },
  ]

  it('バッファの一覧 (今のバッファ, 変更, 大きさ, ファイル) を表示する', () => {
    const { container } = render(
      <BufferList
        buffers={buffers}
        currentBufferId="1"
        onSelectBuffer={vi.fn()}
        onClose={vi.fn()}
      />,
    )
    const rows = container.querySelectorAll('tbody tr')

    expect(rows).toHaveLength(2)
    expect(rows[0].className).toContain('current')
    expect(rows[0].textContent).toContain('>*')
    expect(rows[0].textContent).toContain('7')
    expect(rows[0].textContent).toContain('/tmp/a.scm')
    expect(rows[1].className).not.toContain('current')
    expect(rows[1].textContent).toContain('--')
  })

  it('行を押すと選び, × と外側で閉じる (中を押しても閉じない)', () => {
    const onSelectBuffer = vi.fn()
    const onClose = vi.fn()
    const { container } = render(
      <BufferList
        buffers={buffers}
        currentBufferId="1"
        onSelectBuffer={onSelectBuffer}
        onClose={onClose}
      />,
    )

    fireEvent.click(screen.getByText('*scratch*'))
    expect(onSelectBuffer).toHaveBeenCalledWith('2')

    fireEvent.click(container.querySelector('.buffer-list-container') as Element)
    expect(onClose).not.toHaveBeenCalled()

    fireEvent.click(screen.getByText('×'))
    fireEvent.click(container.querySelector('.buffer-list-overlay') as Element)
    expect(onClose).toHaveBeenCalledTimes(2)
  })
})
