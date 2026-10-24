import { describe, it, expect, afterEach } from 'vitest'
import { EditorState } from '@codemirror/state'
import { EditorView, keymap } from '@codemirror/view'
import { KeyboardMacro } from '@/lib/macro'

const views: EditorView[] = []

afterEach(() => {
  while (views.length > 0) views.pop()?.destroy()
})

describe('KeyboardMacro', () => {
  it('修飾キーだけのものと, 再生したキーは記録せず, 再生のあいだは記録を始めないこと', () => {
    const macro = new KeyboardMacro()
    const started: boolean[] = []
    const view = new EditorView({
      parent: document.body,
      state: EditorState.create({
        extensions: keymap.of([
          {
            key: 'Ctrl-q',
            run: () => {
              started.push(macro.start())
              macro.record(new KeyboardEvent('keydown', { key: 'a' }))
              return true
            },
          },
        ]),
      }),
    })
    views.push(view)
    expect(macro.start()).toBe(true)
    macro.record(new KeyboardEvent('keydown', { key: 'Control', ctrlKey: true }))
    macro.record(new KeyboardEvent('keydown', { key: 'q', ctrlKey: true }))
    macro.record(new KeyboardEvent('keydown', { key: 'x' }))
    macro.noteBinding(0)
    expect(macro.end()).toBe(true)
    macro.replay(view)
    expect(started).toEqual([false])
    expect(macro.isReplaying).toBe(false)
    expect(view.state.doc.toString()).toBe('x')
  })
})
