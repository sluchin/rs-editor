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

  it('モードの列に, バッファのメジャーモードを出す', () => {
    const { container } = render(
      <BufferList
        buffers={[
          ...buffers,
          { id: '3', name: 'a.el', filePath: '/tmp/a.el', content: '', modified: false },
          { id: '4', name: 'a.txt', filePath: '/tmp/a.txt', content: '', modified: false },
        ]}
        currentBufferId="1"
        onSelectBuffer={vi.fn()}
        onClose={vi.fn()}
      />,
    )
    const modes = [...container.querySelectorAll('.buffer-mode')].map((td) => td.textContent)

    expect(modes).toEqual(['Scheme', 'Lisp Interaction', 'ELisp/d', 'Fundamental'])
  })

  it('q で閉じる (修飾キーと一緒なら, 閉じない). 閉じた後は, キーを受けない', () => {
    const onClose = vi.fn()
    const { unmount } = render(
      <BufferList
        buffers={buffers}
        currentBufferId="1"
        onSelectBuffer={vi.fn()}
        onClose={onClose}
      />,
    )

    fireEvent.keyDown(window, { key: 'q', ctrlKey: true })
    fireEvent.keyDown(window, { key: 'q', altKey: true })
    fireEvent.keyDown(window, { key: 'q', metaKey: true })
    fireEvent.keyDown(window, { key: 'a' })
    expect(onClose).not.toHaveBeenCalled()

    fireEvent.keyDown(window, { key: 'q' })
    expect(onClose).toHaveBeenCalledTimes(1)

    unmount()
    fireEvent.keyDown(window, { key: 'q' })
    expect(onClose).toHaveBeenCalledTimes(1)
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
