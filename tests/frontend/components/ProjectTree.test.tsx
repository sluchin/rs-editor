import { describe, it, expect, vi } from 'vitest'
import { act, fireEvent, render } from '@testing-library/react'
import ProjectTree from '@/components/ProjectTree'

/** ディレクトリの中の偽物. */
const tree: Record<string, string[]> = {
  '/p/': ['.git/', 'src/', 'README.md'],
  '/p/src/': ['main.rs'],
}

describe('ProjectTree', () => {
  it('根の中を出し, ディレクトリを開け閉めし, ファイルを選ぶと知らせること', async () => {
    const listDirectory = vi.fn(async (path: string) => {
      const names = tree[path]
      if (!names) throw new Error(`cannot read ${path}`)
      return names
    })
    const onOpenFile = vi.fn()
    const view = render(
      <ProjectTree
        root="/p"
        currentFile="/p/README.md"
        listDirectory={listDirectory}
        onOpenFile={onOpenFile}
      />,
    )
    const item = (name: string) =>
      [...view.container.querySelectorAll('.project-tree-item')].find((e) =>
        e.textContent?.endsWith(name),
      ) as HTMLElement
    await vi.waitFor(() => expect(item('README.md')).toBeDefined())
    expect(view.container.querySelector('.project-tree-title')?.textContent).toBe('P')
    // .git は出さない. 開いているファイルは強調する
    expect(item('.git')).toBeUndefined()
    expect(item('README.md').classList).toContain('project-tree-current')
    expect(item('src').textContent).toBe('▸src')
    // 開く (読む), 閉じる
    await act(async () => fireEvent.click(item('src')))
    expect(item('src').textContent).toBe('▾src')
    expect(item('main.rs')).toBeDefined()
    fireEvent.click(item('main.rs'))
    expect(onOpenFile).toHaveBeenCalledWith('/p/src/main.rs')
    fireEvent.click(item('src'))
    expect(item('main.rs')).toBeUndefined()
    // 読み直す (開いているディレクトリも)
    await act(async () => fireEvent.click(item('src')))
    const calls = listDirectory.mock.calls.length
    await act(async () =>
      fireEvent.click(view.container.querySelector('.project-tree-refresh') as HTMLElement),
    )
    expect(listDirectory.mock.calls.length).toBe(calls + 2)
    // 押してもフォーカスを取らない
    expect(fireEvent.mouseDown(view.container.querySelector('.project-tree') as HTMLElement)).toBe(
      false,
    )
    // 読めないディレクトリは, 理由を出す
    view.rerender(
      <ProjectTree
        root="/none/"
        currentFile={null}
        listDirectory={listDirectory}
        onOpenFile={onOpenFile}
      />,
    )
    await vi.waitFor(() =>
      expect(view.container.querySelector('.project-tree-error')?.textContent).toContain(
        'cannot read /none/',
      ),
    )
  })
})
