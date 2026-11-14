import { describe, it, expect, vi } from 'vitest'
import { createRef } from 'react'
import { act, fireEvent, render } from '@testing-library/react'
import { ChangeSet, Text } from '@codemirror/state'
import { EditorView } from '@codemirror/view'
import WindowView, { type WindowHandle } from '@/components/WindowView'
import type { DocEntry } from '@/lib/docs'
import type { Highlight } from '@/lib/faces'
import type { MajorMode } from '@/lib/session'

const SCHEME: MajorMode = { name: 'Scheme', language: 'scheme', indent_tabs: true, tab_width: 8 }
const FUNDAMENTAL: MajorMode = {
  name: 'Fundamental',
  language: null,
  indent_tabs: true,
  tab_width: 4,
}
const NONE: Highlight[] = []

/**
 * 本文を作ります.
 *
 * @param text - 文字列.
 * @returns 本文 (変更なし).
 */
const entryOf = (text: string): DocEntry => {
  const doc = Text.of(text.split('\n'))
  return { doc, prev: doc, changes: null }
}

/**
 * ウィンドウを出します.
 *
 * @param props - 変えるプロパティ.
 * @returns 出したもの, 送った入力, エディタ, 描き直す関数.
 */
function setup(props: Partial<Parameters<typeof WindowView>[0]> = {}) {
  const onInput = vi.fn()
  const handle = createRef<WindowHandle>()
  const base = {
    windowId: 1,
    bufferName: 'a',
    entry: entryOf('(foo bar)\nline 2'),
    point: 0,
    mark: null,
    active: true,
    asking: false,
    mode: SCHEME,
    highlights: NONE,
    tokens: [],
    onInput,
    handle,
    ...props,
  }
  const view = render(<WindowView {...base} />)
  const editor = () =>
    EditorView.findFromDOM(view.container.querySelector('.cm-editor') as HTMLElement) as EditorView
  const rerender = (next: Partial<Parameters<typeof WindowView>[0]>) => {
    Object.assign(base, next)
    view.rerender(<WindowView {...base} />)
  }
  return { view, onInput, handle, editor, rerender }
}

describe('WindowView', () => {
  it('本文とカーソルとリージョンを映すこと', () => {
    const { editor, rerender } = setup({ point: 3 })
    expect(editor().state.doc.toString()).toBe('(foo bar)\nline 2')
    expect(editor().state.selection.main.head).toBe(3)
    // リージョン (マークからカーソルまで)
    rerender({ point: 5, mark: 1 })
    expect(editor().state.selection.main.from).toBe(1)
    expect(editor().state.selection.main.to).toBe(5)
    // 本文より先の位置は, 終わりにする
    rerender({ point: 99, mark: null })
    expect(editor().state.selection.main.head).toBe(16)
  })

  it('前の本文からの変更は, そのまま当て, ほかの本文は, 違う所だけ置き換えること', () => {
    const first = entryOf('abc')
    const { editor, rerender } = setup({ entry: first })
    const changes = ChangeSet.of({ from: 3, insert: 'd' }, 3)
    const second: DocEntry = { doc: changes.apply(first.doc), prev: first.doc, changes }
    rerender({ entry: second })
    expect(editor().state.doc).toBe(editor().state.doc)
    expect(editor().state.doc.toString()).toBe('abcd')
    // 前の本文が違えば, 比べて置き換える
    rerender({ entry: entryOf('xbcd') })
    expect(editor().state.doc.toString()).toBe('xbcd')
    // 同じ本文なら, 何もしない
    const doc = editor().state.doc
    rerender({ entry: { doc, prev: doc, changes: null } })
    expect(editor().state.doc).toBe(doc)
  })

  it('バッファを切り替えると, そのバッファの状態にし, 戻すと前の状態にすること', () => {
    const { editor, rerender, handle } = setup()
    act(() => handle.current?.foldRegion(1, 8, true))
    expect(handle.current?.invisible(2)).toBe(true)
    // 隠した所の端は, 隠していない. 端に触れるだけの範囲は, 隠した所を変えない
    expect(handle.current?.invisible(8)).toBe(false)
    act(() => handle.current?.foldRegion(8, 10, false))
    expect(handle.current?.invisible(2)).toBe(true)
    rerender({ bufferName: 'b', entry: entryOf('other'), mode: FUNDAMENTAL })
    expect(editor().state.doc.toString()).toBe('other')
    expect(handle.current?.invisible(2)).toBe(false)
    rerender({ bufferName: 'a', entry: entryOf('(foo bar)\nline 2'), mode: SCHEME })
    expect(handle.current?.invisible(2)).toBe(true)
    // 出す (隠した所に掛かる範囲)
    act(() => handle.current?.foldRegion(0, 3, false))
    expect(handle.current?.invisible(2)).toBe(false)
    act(() => handle.current?.foldRegion(12, 3, true))
    expect(handle.current?.invisible(2)).toBe(false)
  })

  it('自分では本文を書き換えないこと (日本語の変換中の文字は除く)', () => {
    const { editor } = setup()
    act(() => editor().dispatch({ changes: { from: 0, insert: 'x' } }))
    expect(editor().state.doc.toString()).toBe('(foo bar)\nline 2')
    act(() =>
      editor().dispatch({ changes: { from: 0, insert: 'あ' }, userEvent: 'input.type.compose' }),
    )
    expect(editor().state.doc.toString()).toBe('あ(foo bar)\nline 2')
    // カーソルだけなら動かせる
    act(() => editor().dispatch({ selection: { anchor: 2 } }))
    expect(editor().state.selection.main.head).toBe(2)
  })

  it('キーと, 確定した文字と, 貼り付けと, クリックを送ること', async () => {
    const { editor, onInput } = setup()
    const content = editor().contentDOM
    fireEvent.keyDown(content, { key: 'f', ctrlKey: true })
    expect(onInput).toHaveBeenLastCalledWith({ kind: 'key', key: 'C-f' })
    // 変換中のキーと, 修飾キーだけは送らない
    fireEvent.keyDown(content, { key: 'a', isComposing: true })
    fireEvent.keyDown(content, { key: 'a', keyCode: 229 })
    fireEvent.keyDown(content, { key: 'Shift' })
    expect(onInput).toHaveBeenCalledTimes(1)
    // 変換を確定したら, 入れた文字を戻して, 確定した文字を送る
    act(() =>
      editor().dispatch({ changes: { from: 0, insert: 'に' }, userEvent: 'input.type.compose' }),
    )
    fireEvent(content, new CompositionEvent('compositionend', { data: '日本' }))
    expect(onInput).toHaveBeenLastCalledWith({ kind: 'text', text: '日本' })
    await vi.waitFor(() => expect(editor().state.doc.toString()).toBe('(foo bar)\nline 2'))
    fireEvent(content, new CompositionEvent('compositionend', { data: '' }))
    expect(onInput).toHaveBeenCalledTimes(2)
    // 貼り付け (空なら送らない). 切り取りと落とすことは, させない
    const paste = (text: string | null) => {
      const event = new Event('paste', { bubbles: true, cancelable: true })
      Object.assign(event, {
        clipboardData: text === null ? null : { getData: () => text },
      })
      fireEvent(content, event)
    }
    paste('pasted')
    expect(onInput).toHaveBeenLastCalledWith({ kind: 'text', text: 'pasted' })
    paste('')
    paste(null)
    expect(onInput).toHaveBeenCalledTimes(3)
    expect(fireEvent(content, new Event('cut', { bubbles: true, cancelable: true }))).toBe(false)
    expect(fireEvent(content, new Event('drop', { bubbles: true, cancelable: true }))).toBe(false)
    // クリック: カーソルの位置. ドラッグで選んだら, 始めも
    act(() => editor().dispatch({ selection: { anchor: 4 } }))
    fireEvent.mouseUp(content)
    expect(onInput).toHaveBeenLastCalledWith({ kind: 'point', window: 1, pos: 4, anchor: null })
    act(() => editor().dispatch({ selection: { anchor: 1, head: 4 } }))
    fireEvent.mouseUp(content)
    expect(onInput).toHaveBeenLastCalledWith({ kind: 'point', window: 1, pos: 4, anchor: 1 })
  })

  it('画面からの操作 (真ん中に出す, 一番上の行, 見えている行の数, 本文) に答えること', () => {
    const { editor, handle } = setup()
    act(() => handle.current?.recenter())
    expect(handle.current?.topLine()).toBe(0)
    expect(handle.current?.bodyHeight()).toBeGreaterThanOrEqual(1)
    expect(handle.current?.doc()).toBe(editor().state.doc)
  })

  it('色付けを引き直し, 選んでいるウィンドウにフォーカスを移すこと', () => {
    const { editor, rerender, view } = setup({ active: false })
    expect(editor().hasFocus).toBe(false)
    rerender({ highlights: [{ regexp: 'foo' }] })
    expect(view.container.querySelector('.cm-lisp-highlight')?.textContent).toBe('foo')
    // 同じ face は同じ印. 合わなかった括弧には付けない
    rerender({
      highlights: [
        { regexp: '(x)?foo', face: 'bold', group: 1 },
        { regexp: 'bar', face: 'bold' },
        { regexp: 'f(o)o', face: 'italic', group: 1 },
      ],
    })
    expect(view.container.querySelector('.cm-face-bold')?.textContent).toBe('bar')
    expect(view.container.querySelector('.cm-face-italic')?.textContent).toBe('o')
    rerender({ highlights: [{ regexp: 'foo' }] })
    // 本文が変わったら, 引き直す
    rerender({ entry: entryOf('(foo foo)') })
    expect(view.container.querySelectorAll('.cm-lisp-highlight').length).toBe(2)
    // 聞いているあいだは, 移さない
    rerender({ active: true, asking: true })
    expect(editor().hasFocus).toBe(false)
    rerender({ asking: false })
    expect(editor().hasFocus).toBe(true)
    view.unmount()
  })

  it('核が返した字句 (種類と範囲) を、種類ごとのクラスで色付けすること', () => {
    const { rerender, view } = setup()
    // 字句が無ければ, 印は付かない
    expect(view.container.querySelector('[class*="cm-tok-"]')).toBeNull()
    // 本文 `(foo bar)\nline 2` (長さ 16). 空の範囲と, 本文より先へ出る範囲は, 飛ばす
    rerender({
      tokens: [
        { from: 1, to: 4, kind: 'keyword' },
        { from: 5, to: 8, kind: 'keyword' },
        { from: 10, to: 14, kind: 'comment' },
        { from: 3, to: 3, kind: 'string' },
        { from: 12, to: 30, kind: 'type' },
      ],
    })
    const keywords = view.container.querySelectorAll('.cm-tok-keyword')
    expect([...keywords].map((e) => e.textContent)).toEqual(['foo', 'bar'])
    expect(view.container.querySelector('.cm-tok-comment')?.textContent).toBe('line')
    expect(view.container.querySelector('.cm-tok-string')).toBeNull()
    expect(view.container.querySelector('.cm-tok-type')).toBeNull()
    // 本文の長さが変わったら, 作り直す (さっき飛ばした範囲が, 収まる)
    rerender({ entry: entryOf('(foo bar)\nline 2 and more text') })
    expect(view.container.querySelector('.cm-tok-type')?.textContent).toBe('ne 2 and more text')
    view.unmount()
  })
})
