import { describe, it, expect, vi } from 'vitest'
import { act, fireEvent, render } from '@testing-library/react'
import ProjectTree from '@/components/ProjectTree'

/** ディレクトリの中の偽物. */
const tree: Record<string, string[]> = {
  '/p/': ['.git/', 'src/', 'README.md'],
  '/p/src/': ['main.rs'],
}

describe('ProjectTree', () => {
  it('今のファイルが深くにあれば, そこまでのディレクトリを開き, 見える所へ動かすこと', async () => {
    const deep: Record<string, string[]> = {
      '/p/': ['a/', 'z.txt'],
      '/p/a/': ['b/'],
      '/p/a/b/': ['f.rs'],
    }
    const listDirectory = vi.fn(async (path: string) => deep[path] ?? [])
    const scrolled = vi.fn()
    Element.prototype.scrollIntoView = scrolled
    const view = render(
      <ProjectTree
        root="/p"
        currentFile="/p/z.txt"
        listDirectory={listDirectory}
        onOpenFile={vi.fn()}
      />,
    )
    const current = () => view.container.querySelector('.project-tree-current')?.textContent
    await vi.waitFor(() => expect(current()).toBe('z.txt'))
    expect(listDirectory).not.toHaveBeenCalledWith('/p/a/')
    // 別の場所のファイルに切り替わると, そこまで開く
    view.rerender(
      <ProjectTree
        root="/p"
        currentFile="/p/a/b/f.rs"
        listDirectory={listDirectory}
        onOpenFile={vi.fn()}
      />,
    )
    await vi.waitFor(() => expect(current()).toBe('f.rs'))
    expect(scrolled).toHaveBeenCalled()
    // 根の外のファイルでは, 何もしない
    view.rerender(
      <ProjectTree
        root="/p"
        currentFile="/q/x.rs"
        listDirectory={listDirectory}
        onOpenFile={vi.fn()}
      />,
    )
    expect(current()).toBeUndefined()
    // scrollIntoView の無い環境でも, 落ちない
    // @ts-expect-error jsdom の差し替え
    delete Element.prototype.scrollIntoView
    view.rerender(
      <ProjectTree
        root="/p"
        currentFile="/p/a/b/f.rs"
        listDirectory={listDirectory}
        onOpenFile={vi.fn()}
      />,
    )
    await vi.waitFor(() => expect(current()).toBe('f.rs'))
  })

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

  it('フォーカスを移すと, n, p, RET, f, b, g, q で動かせること', async () => {
    const listDirectory = vi.fn(async (path: string) => {
      if (path === '/p/bad/') throw new Error('denied')
      return tree[path] ?? []
    })
    tree['/p/'] = ['.git/', 'bad/', 'src/', 'README.md']
    const onOpenFile = vi.fn()
    const view = render(
      <ProjectTree
        root="/p"
        currentFile={null}
        listDirectory={listDirectory}
        onOpenFile={onOpenFile}
        focusSignal={0}
      />,
    )
    await vi.waitFor(() =>
      expect(view.container.querySelectorAll('.project-tree-item').length).toBe(3),
    )
    const box = view.container.querySelector('.project-tree') as HTMLElement
    const cursor = () =>
      view.container.querySelector('.project-tree-cursor')?.textContent?.replace(/^[▸▾]/, '')
    const press = (key: string, ctrl = false) => fireEvent.keyDown(box, { key, ctrlKey: ctrl })
    // 動かす前に押しても, 何も起きない
    press('RET')
    press('Enter')
    press('f')
    press('b')
    view.rerender(
      <ProjectTree
        root="/p"
        currentFile={null}
        listDirectory={listDirectory}
        onOpenFile={onOpenFile}
        focusSignal={1}
      />,
    )
    expect(document.activeElement).toBe(box)
    expect(cursor()).toBeUndefined()
    press('p')
    expect(cursor()).toBe('README.md')
    press('<')
    expect(cursor()).toBe('bad')
    press('n')
    expect(cursor()).toBe('src')
    fireEvent.keyDown(box, { key: 'n', ctrlKey: true })
    expect(cursor()).toBe('README.md')
    fireEvent.keyDown(box, { key: 'n', ctrlKey: true })
    expect(cursor()).toBe('README.md')
    press('ArrowUp')
    expect(cursor()).toBe('src')
    // 開く (右, f, RET)
    press('f')
    await vi.waitFor(() => expect(view.container.textContent).toContain('main.rs'))
    press('f')
    expect(cursor()).toBe('main.rs')
    press('f')
    expect(cursor()).toBe('main.rs')
    // 子から親へ, 親を閉じる, 一番上では動かない
    press('b')
    expect(cursor()).toBe('src')
    press('ArrowLeft')
    expect(view.container.textContent).not.toContain('main.rs')
    press('b')
    expect(cursor()).toBe('src')
    press('Enter')
    await vi.waitFor(() => expect(view.container.textContent).toContain('main.rs'))
    press(' ')
    expect(view.container.textContent).not.toContain('main.rs')
    // 読めないディレクトリを開いても動ける
    press('<')
    press('RET')
    press('Enter')
    await vi.waitFor(() => expect(view.container.textContent).toContain('denied'))
    press('n')
    expect(cursor()).toBe('src')
    // ファイルを開くと, フォーカスを戻す
    press('End')
    press('Enter')
    expect(onOpenFile).toHaveBeenCalledWith('/p/README.md')
    expect(document.activeElement).not.toBe(box)
    // 読み直し, 戻る, 知らないキーは通す
    const before = listDirectory.mock.calls.length
    press('g')
    expect(listDirectory.mock.calls.length).toBeGreaterThan(before)
    for (const key of ['q', 'Escape']) {
      box.focus()
      press(key)
      expect(document.activeElement).not.toBe(box)
    }
    box.focus()
    press('g', true)
    expect(document.activeElement).not.toBe(box)
    expect(fireEvent.keyDown(box, { key: 'x' })).toBe(true)
    // 別名のキーも, 同じように動く
    box.focus()
    for (const key of ['Home', '>', 'ArrowDown', 'ArrowRight', 'ArrowLeft', 'End', ' ']) press(key)
    press('p', true)
    press('n', true)
    // 空のツリーでも壊れない
    // Meta や Alt の付いたキーは通す
    expect(fireEvent.keyDown(box, { key: 'n', ctrlKey: true, altKey: true })).toBe(true)
    expect(fireEvent.keyDown(box, { key: 'n', ctrlKey: true, metaKey: true })).toBe(true)
    tree['/p/'] = []
    press('g')
    await vi.waitFor(() =>
      expect(view.container.querySelectorAll('.project-tree-item').length).toBe(0),
    )
    press('n')
  })
})
