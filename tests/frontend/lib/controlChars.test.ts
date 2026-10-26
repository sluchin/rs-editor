import { describe, it, expect, afterEach } from 'vitest'
import { EditorState } from '@codemirror/state'
import { EditorView } from '@codemirror/view'
import { controlChars, renderControlChar } from '@/lib/controlChars'

/** 作ったエディタ (テストの後に消す). */
const views: EditorView[] = []

afterEach(() => {
  while (views.length > 0) views.pop()?.destroy()
})

describe('制御文字の表示', () => {
  it('制御文字を, ^A のように出すこと (タブは除く)', () => {
    const view = new EditorView({
      parent: document.body,
      state: EditorState.create({ doc: 'a\x01b\x7f\tc', extensions: [controlChars()] }),
    })
    views.push(view)
    const marks = [...view.contentDOM.querySelectorAll('.cm-control-char')]
    expect(marks.map((mark) => mark.textContent)).toEqual(['^A', '^?'])
  })

  it('ESC を ^[ にすること', () => {
    expect(renderControlChar(27).textContent).toBe('^[')
  })
})
