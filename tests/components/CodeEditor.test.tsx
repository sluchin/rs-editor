import { describe, it, expect, vi } from 'vitest'
import { createRef } from 'react'
import { act, render, fireEvent } from '@testing-library/react'
import { EditorView } from '@codemirror/view'
import { language } from '@codemirror/language'
import CodeEditor, { EditorHandle } from '@/components/CodeEditor'

/**
 * CodeEditor を表示します.
 *
 * @param props - 変える引数.
 * @returns 表示と, エディタ (CodeMirror) と, 呼ばれた関数.
 */
function renderEditor(props: Partial<Parameters<typeof CodeEditor>[0]> = {}) {
  const handlers = {
    onChange: vi.fn(),
    onCursorChange: vi.fn(),
    onMessage: vi.fn(),
    actions: { 'find-file': vi.fn() },
  }
  const handle = createRef<EditorHandle>()
  const element = (extra: Partial<Parameters<typeof CodeEditor>[0]> = {}) => (
    <CodeEditor
      bufferId="a"
      initialContent="(+ 1 2)"
      keyBindings={[]}
      disabled={false}
      handle={handle}
      {...handlers}
      {...props}
      {...extra}
    />
  )
  const view = render(element())
  const editor = EditorView.findFromDOM(
    view.container.querySelector('.cm-editor') as HTMLElement,
  ) as EditorView
  /** キーを押します. */
  const key = (k: string, mods: { ctrl?: boolean; alt?: boolean } = {}) =>
    fireEvent.keyDown(editor.contentDOM, {
      key: k,
      ctrlKey: mods.ctrl ?? false,
      altKey: mods.alt ?? false,
    })
  return { ...view, editor, handle, key, element, ...handlers }
}

describe('CodeEditor Component', () => {
  it('前置引数で, エディタのコマンドは繰り返し, 画面のコマンドは 1 回だけ動かすこと', () => {
    const { editor, key, actions, onMessage } = renderEditor()
    key('u', { ctrl: true })
    expect(onMessage).toHaveBeenCalledWith('C-u 4-')
    key('f', { ctrl: true })
    expect(editor.state.selection.main.head).toBe(4)
    key('u', { ctrl: true })
    key('x', { ctrl: true })
    key('f', { ctrl: true })
    expect(actions['find-file']).toHaveBeenCalledTimes(1)
  })

  it('別のウィンドウの書き換えを写し, 元に戻す履歴には入れないこと', () => {
    const { editor, handle, onChange } = renderEditor()
    act(() => handle.current?.sync('(+ 1 2 3)'))
    expect(editor.state.doc.toString()).toBe('(+ 1 2 3)')
    act(() => handle.current?.sync('(+ 1 2 3)'))
    act(() => handle.current?.sync('x'))
    expect(editor.state.doc.toString()).toBe('x')
    expect(onChange).not.toHaveBeenCalled()
    expect(handle.current?.run('undo')).toBe(true)
    expect(editor.state.doc.toString()).toBe('x')
  })

  it('言語を渡すと色を付け, 言語やバッファが変わると付け直すこと', () => {
    const { editor, rerender, element } = renderEditor({ language: 'scheme' })
    expect(editor.state.facet(language)?.name).toBe('scheme')
    rerender(element({ language: null }))
    expect(editor.state.facet(language)).toBeNull()
    // 色を付けないバッファから, 色を付けるバッファに切り替える
    rerender(element({ bufferId: 'b', language: 'scheme' }))
    expect(editor.state.facet(language)?.name).toBe('scheme')
    rerender(element({ bufferId: 'b', language: 'elisp' }))
    expect(editor.state.facet(language)?.name).toBe('elisp')
  })

  it('選んでいないウィンドウのエディタは, フォーカスを取らず, 選ぶと取ること', () => {
    const { editor, rerender, element } = renderEditor({ active: false })
    expect(editor.hasFocus).toBe(false)
    rerender(element({ active: false, disabled: true }))
    rerender(element({ active: false, disabled: false }))
    expect(editor.hasFocus).toBe(false)
    rerender(element({ active: true, disabled: true }))
    expect(editor.hasFocus).toBe(false)
    rerender(element({ active: true, disabled: false }))
    expect(editor.hasFocus).toBe(true)
  })

  it('Lisp から使う操作 (本文, 位置, 入れる, 消す) をすること', () => {
    const { editor, handle } = renderEditor()
    act(() => handle.current?.setCursor(3))
    expect(handle.current?.point()).toBe(3)
    act(() => handle.current?.insert('xy'))
    expect(handle.current?.text()).toBe('(+ xy1 2)')
    expect(editor.state.selection.main.head).toBe(5)
    act(() => handle.current?.deleteRegion(0, 3))
    expect(handle.current?.text()).toBe('xy1 2)')
  })

  it('Lisp で割り当てたキーを, 既定のキーより先に使い, バッファを切り替えても使うこと', () => {
    const run = vi.fn()
    const { editor, key, element, rerender } = renderEditor({ actions: { mine: run } })
    // 画面のコマンドは, 同じ名前のエディタのコマンドより先に使う
    const ownEnd = vi.fn()
    rerender(element({ actions: { 'end-of-buffer': ownEnd } }))
    key('>', { alt: true })
    expect(ownEnd).toHaveBeenCalled()
    expect(editor.state.selection.main.head).toBe(0)

    const keys = [{ key: 'Ctrl-e', command: 'mine' }]
    rerender(element({ actions: { mine: run }, keyBindings: keys }))
    key('e', { ctrl: true })
    expect(run).toHaveBeenCalledTimes(1)
    // 同じ割り当てなら, 作り直さない
    rerender(element({ actions: { mine: run }, keyBindings: keys }))
    rerender(element({ actions: { mine: run }, keyBindings: keys, bufferId: 'b' }))
    key('e', { ctrl: true })
    expect(run).toHaveBeenCalledTimes(2)
  })

  it('始めの本文を表示し, カーソルの位置を知らせること', () => {
    const { editor, onCursorChange } = renderEditor()
    expect(editor.state.doc.toString()).toBe('(+ 1 2)')
    expect(onCursorChange).toHaveBeenCalledWith({ line: 1, column: 1, offset: 0 })
  })

  it('書き換えと, カーソルの移動を知らせること', () => {
    const { editor, onChange, onCursorChange } = renderEditor()
    editor.dispatch({ changes: { from: 0, insert: 'a\n' }, selection: { anchor: 3 } })
    expect(onChange).toHaveBeenCalledWith('a\n(+ 1 2)')
    expect(onCursorChange).toHaveBeenLastCalledWith({ line: 2, column: 2, offset: 3 })
    editor.dispatch({ selection: { anchor: 0 } })
    expect(onCursorChange).toHaveBeenLastCalledWith({ line: 1, column: 1, offset: 0 })
  })

  it('Emacs のキーでエディタを動かし, 画面のコマンドも呼ぶこと', () => {
    const { editor, key, actions } = renderEditor()
    key('e', { ctrl: true })
    expect(editor.state.selection.main.head).toBe(7)
    key('x', { ctrl: true })
    key('f', { ctrl: true })
    expect(actions['find-file']).toHaveBeenCalled()
  })

  it('コマンドを名前で実行し, 無いコマンドは false を返すこと', () => {
    const { editor, handle, actions } = renderEditor()
    expect(handle.current?.run('end-of-buffer')).toBe(true)
    expect(editor.state.selection.main.head).toBe(7)
    expect(handle.current?.run('find-file')).toBe(true)
    expect(actions['find-file']).toHaveBeenCalled()
    expect(handle.current?.run('no-such-command')).toBe(false)
  })

  it('範囲を選び, カーソルを置き, 行へ動くこと', () => {
    const { editor, handle } = renderEditor({ initialContent: 'ab\ncd' })
    act(() => handle.current?.select(1, 2))
    expect(editor.state.selection.main).toMatchObject({ from: 1, to: 2 })
    act(() => handle.current?.setCursor(4))
    expect(editor.state.selection.main.head).toBe(4)
    act(() => handle.current?.gotoLine(1))
    expect(editor.state.selection.main.head).toBe(0)
  })

  it('コマンドの知らせを, ミニバッファへ渡すこと', () => {
    const { key, onMessage } = renderEditor()
    key('g', { ctrl: true })
    expect(onMessage).toHaveBeenCalledWith('Quit')
  })

  it('書き換えさせないあいだは読み取り専用にし, 終わったら戻すこと', () => {
    const { editor, rerender, element } = renderEditor({ disabled: true })
    expect(editor.state.readOnly).toBe(true)
    rerender(element({ disabled: false }))
    expect(editor.state.readOnly).toBe(false)
  })

  it('バッファを切り替えると, バッファごとの本文とカーソルに切り替え, 消したバッファは作り直すこと', () => {
    const { editor, rerender, element, handle, onChange } = renderEditor()
    editor.dispatch({ selection: { anchor: 3 } })
    rerender(element({ bufferId: 'b', initialContent: 'other' }))
    expect(editor.state.doc.toString()).toBe('other')
    rerender(element({ bufferId: 'a', initialContent: '(+ 1 2)' }))
    expect(editor.state.doc.toString()).toBe('(+ 1 2)')
    expect(editor.state.selection.main.head).toBe(3)
    // 隠していたあいだに (別のウィンドウで) 書き換えた本文は, 写す (書き換えとしては知らせない)
    rerender(element({ bufferId: 'b', initialContent: 'other' }))
    onChange.mockClear()
    rerender(element({ bufferId: 'a', initialContent: '(+ 10 2) ;' }))
    expect(editor.state.doc.toString()).toBe('(+ 10 2) ;')
    expect(editor.state.selection.main.head).toBe(3)
    expect(onChange).not.toHaveBeenCalled()
    // 同じバッファのまま描き直しても, 状態を変えない
    rerender(element({ bufferId: 'a', initialContent: 'ignored' }))
    expect(editor.state.doc.toString()).toBe('(+ 10 2) ;')
    act(() => handle.current?.forget('b'))
    rerender(element({ bufferId: 'b', initialContent: 'new' }))
    expect(editor.state.doc.toString()).toBe('new')
  })
})
