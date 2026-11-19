import { describe, it, expect, vi, beforeEach } from 'vitest'
import { act, fireEvent, render } from '@testing-library/react'
import ProjectTree from '@/components/ProjectTree'

/** ディレクトリの中の偽物. */
const tree: Record<string, string[]> = {
  '/p/': ['.git/', 'src/', 'README.md'],
  '/p/src/': ['main.rs'],
}

/** ファイルの作成, 名前の変更, 削除の偽物. */
const ops = {
  createPath: vi.fn(async () => {}),
  renamePath: vi.fn(async () => {}),
  deletePath: vi.fn(async () => {}),
}

describe('ProjectTree', () => {
  beforeEach(() => localStorage.clear())

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
        {...ops}
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
        {...ops}
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
        {...ops}
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
        {...ops}
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
        {...ops}
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
        {...ops}
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
        {...ops}
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
        {...ops}
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

  /** 根に README.md と src/ (中に main.rs) がある, 木を出す. */
  const setup = async () => {
    const listDirectory = vi.fn(async (path: string) => tree[path] ?? [])
    tree['/p/'] = ['.git/', 'src/', 'README.md']
    const onOpenFile = vi.fn()
    const view = render(
      <ProjectTree
        {...ops}
        root="/p"
        currentFile={null}
        listDirectory={listDirectory}
        onOpenFile={onOpenFile}
        focusSignal={1}
      />,
    )
    await vi.waitFor(() =>
      expect(view.container.querySelectorAll('.project-tree-item').length).toBe(2),
    )
    const box = view.container.querySelector('.project-tree') as HTMLElement
    const press = (key: string) => fireEvent.keyDown(box, { key })
    const input = () =>
      view.container.querySelector('.project-tree-prompt input') as HTMLInputElement
    const type = (text: string) => fireEvent.change(input(), { target: { value: text } })
    const notice = () => view.container.querySelector('.project-tree-error')?.textContent
    return { view, box, press, input, type, notice, onOpenFile, listDirectory }
  }

  it('幅を, 右の端を引くか { と } で変え, 限りの中に収めること', async () => {
    const { view, press, box } = await setup()
    expect(box.style.flexBasis).toBe('240px')
    press('{')
    expect(box.style.flexBasis).toBe('220px')
    press('}')
    press('}')
    expect(box.style.flexBasis).toBe('260px')
    const resizer = view.container.querySelector('.project-tree-resizer') as HTMLElement
    fireEvent.mouseDown(resizer, { clientX: 260 })
    fireEvent.mouseMove(document, { clientX: 300 })
    expect(box.style.flexBasis).toBe('300px')
    fireEvent.mouseMove(document, { clientX: 5000 })
    expect(box.style.flexBasis).toBe('600px')
    fireEvent.mouseMove(document, { clientX: -5000 })
    expect(box.style.flexBasis).toBe('120px')
    // 離したあとは, 動かない
    fireEvent.mouseUp(document)
    fireEvent.mouseMove(document, { clientX: 400 })
    expect(box.style.flexBasis).toBe('120px')
  })

  it('a と A で, 選んだ行の所に, ファイルとディレクトリを作ること', async () => {
    const { view, press, input, type, onOpenFile, notice, box } = await setup()
    // 行が無ければ, 根に作る. 作ったファイルは, 開く
    press('a')
    expect(view.container.querySelector('.project-tree-prompt')?.textContent).toContain('New file')
    type(' new.txt ')
    fireEvent.keyDown(input(), { key: 'Enter' })
    await vi.waitFor(() => expect(ops.createPath).toHaveBeenCalledWith('/p/new.txt', false))
    await vi.waitFor(() => expect(onOpenFile).toHaveBeenCalledWith('/p/new.txt'))
    expect(document.activeElement).toBe(box)
    // ディレクトリの行では, その中に作る. 開いて見せる
    press('<')
    press('A')
    expect(view.container.querySelector('.project-tree-prompt')?.textContent).toContain(
      'New directory',
    )
    type('sub')
    fireEvent.keyDown(input(), { key: 'Enter' })
    await vi.waitFor(() => expect(ops.createPath).toHaveBeenCalledWith('/p/src/sub', true))
    await vi.waitFor(() => expect(view.container.textContent).toContain('main.rs'))
    expect(onOpenFile).toHaveBeenCalledTimes(1)
    // ファイルの行では, その親に作る
    await act(async () => {})
    press('<')
    press('n')
    press('a')
    type('x.rs')
    fireEvent.keyDown(input(), { key: 'Enter' })
    await vi.waitFor(() => expect(ops.createPath).toHaveBeenCalledWith('/p/src/x.rs', false))
    // 名前が空, `/` を含むときは, 作らずに知らせる. ほかのキーは, 欄に任せる
    const before = ops.createPath.mock.calls.length
    press('a')
    fireEvent.keyDown(input(), { key: 'x' })
    fireEvent.keyDown(input(), { key: 'Enter' })
    expect(notice()).toBeUndefined()
    press('a')
    type('a/b')
    fireEvent.keyDown(input(), { key: 'Enter' })
    expect(notice()).toBe('Invalid name: a/b')
    expect(ops.createPath).toHaveBeenCalledTimes(before)
    // 失敗は, 知らせに出す. 次のキーで消える
    ops.createPath.mockRejectedValueOnce(new Error('exists'))
    press('a')
    type('dup')
    fireEvent.keyDown(input(), { key: 'Enter' })
    await vi.waitFor(() => expect(notice()).toBe('Error: exists'))
    press('n')
    expect(notice()).toBeUndefined()
    // ヘッダーの + でも, 根に作れる. Escape と C-g でやめる
    fireEvent.click(view.container.querySelector('[title="New File"]') as HTMLElement)
    expect(input()).not.toBeNull()
    fireEvent.keyDown(input(), { key: 'Escape' })
    expect(view.container.querySelector('.project-tree-prompt')).toBeNull()
    press('a')
    fireEvent.keyDown(input(), { key: 'g', ctrlKey: true })
    expect(view.container.querySelector('.project-tree-prompt')).toBeNull()
  })

  it('r で名前を変え, d と y で消し, ほかのキーでやめること', async () => {
    const { view, press, input, type, notice, listDirectory } = await setup()
    // 行が無いときは, 何も起きない
    press('r')
    press('d')
    expect(view.container.querySelector('.project-tree-prompt')).toBeNull()
    press('<')
    press('n')
    // 今の名前から始める
    press('r')
    expect(input().value).toBe('README.md')
    type('NOTES.md')
    fireEvent.keyDown(input(), { key: 'Enter' })
    await vi.waitFor(() =>
      expect(ops.renamePath).toHaveBeenCalledWith('/p/README.md', '/p/NOTES.md'),
    )
    await vi.waitFor(() => expect(listDirectory).toHaveBeenCalledTimes(2))
    await act(async () => {})
    // 子のファイルは, 親の中で変える
    press('<')
    press('f')
    await vi.waitFor(() => expect(view.container.textContent).toContain('main.rs'))
    press('n')
    press('r')
    type('lib.rs')
    fireEvent.keyDown(input(), { key: 'Enter' })
    await vi.waitFor(() =>
      expect(ops.renamePath).toHaveBeenCalledWith('/p/src/main.rs', '/p/src/lib.rs'),
    )
    // 失敗は知らせに出す (名前を変えた行は, 偽の一覧に無いので, 選び直す)
    await act(async () => {})
    press('<')
    press('n')
    ops.renamePath.mockRejectedValueOnce(new Error('exists'))
    press('r')
    fireEvent.keyDown(input(), { key: 'Enter' })
    await vi.waitFor(() => expect(notice()).toBe('Error: exists'))
    // 消す: y だけが消す
    press('d')
    expect(view.container.querySelector('.project-tree-prompt')?.textContent).toBe(
      'Delete main.rs? (y/n)',
    )
    press('n')
    expect(ops.deletePath).not.toHaveBeenCalled()
    expect(view.container.querySelector('.project-tree-prompt')).toBeNull()
    press('d')
    press('y')
    await vi.waitFor(() => expect(ops.deletePath).toHaveBeenCalledWith('/p/src/main.rs'))
    // 根の直下を消すと, 行は無くなる (偽の一覧は変わらないので, README.md まで動かす)
    await act(async () => {})
    press('<')
    press('n')
    press('n')
    press('d')
    press('y')
    await vi.waitFor(() => expect(ops.deletePath).toHaveBeenCalledWith('/p/README.md'))
  })

  it('根が / のときも, 直下のファイルの名前を変え, 欄を押せること', async () => {
    const listDirectory = vi.fn(async () => ['a.txt'])
    const view = render(
      <ProjectTree
        {...ops}
        root="/"
        currentFile={null}
        listDirectory={listDirectory}
        onOpenFile={vi.fn()}
        focusSignal={1}
      />,
    )
    await vi.waitFor(() => expect(view.container.textContent).toContain('a.txt'))
    const box = view.container.querySelector('.project-tree') as HTMLElement
    fireEvent.keyDown(box, { key: '<' })
    fireEvent.keyDown(box, { key: 'r' })
    const input = view.container.querySelector('.project-tree-prompt input') as HTMLElement
    // 欄を押すときは, フォーカスを奪わない (押した動きを止めない)
    expect(fireEvent.mouseDown(input)).toBe(true)
    fireEvent.change(input, { target: { value: 'b.txt' } })
    fireEvent.keyDown(input, { key: 'Enter' })
    await vi.waitFor(() => expect(ops.renamePath).toHaveBeenCalledWith('/a.txt', '/b.txt'))
    await vi.waitFor(() => expect(listDirectory).toHaveBeenCalledTimes(2))
  })

  it('幅を覚えて, 次に出すときに使うこと', async () => {
    const { press, view } = await setup()
    press('}')
    expect(localStorage.getItem('rseditor.projectTree.width')).toBe('260')
    view.unmount()
    const again = await setup()
    expect(again.box.style.flexBasis).toBe('260px')
  })

  it('窓にフォーカスが戻ったら, 読み直すこと', async () => {
    const listDirectory = vi.fn(async (path: string) => tree[path] ?? [])
    render(
      <ProjectTree
        {...ops}
        root="/p"
        currentFile={null}
        listDirectory={listDirectory}
        onOpenFile={vi.fn()}
      />,
    )
    await vi.waitFor(() => expect(listDirectory).toHaveBeenCalledTimes(1))
    act(() => {
      window.dispatchEvent(new Event('focus'))
    })
    await vi.waitFor(() => expect(listDirectory).toHaveBeenCalledTimes(2))
  })
})
