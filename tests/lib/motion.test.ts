import { describe, it, expect, vi } from 'vitest'
import { EditorSelection, EditorState } from '@codemirror/state'
import { EditorView } from '@codemirror/view'
import { createCommands, emacsState, markField } from '@/lib/emacs'
import { KillRing } from '@/lib/killRing'
import { KeyboardMacro } from '@/lib/macro'
import { indentConfig } from '@/lib/lispIndent'
import table from './motion.json'

/** 結果: カーソル, カーソルとマーク (マークのコマンド), または, ミニバッファに出した理由. */
type Result = number | [number, number | null] | string

/**
 * Emacs 30 (`emacs -Q --batch`) で, 本文の全ての位置からコマンドを動かした結果.
 * モード, 本文, コマンド, 位置ごとの結果 (0 から数える).
 */
const cases = table as [mode: string, text: string, command: string, results: Result[]][]

/** Emacs のメジャーモードと, Lisp の方言. */
const DIALECTS: Record<string, 'scheme' | 'elisp' | null> = {
  'fundamental-mode': null,
  'emacs-lisp-mode': 'elisp',
  'scheme-mode': 'scheme',
}

/**
 * 位置からコマンドを動かして, 結果を返します.
 *
 * @param mode - Emacs のメジャーモード.
 * @param text - 本文.
 * @param command - コマンド.
 * @param pos - 位置.
 * @returns 結果.
 */
function run(mode: string, text: string, command: string, pos: number): Result {
  const message = vi.fn()
  const commands = createCommands({ killRing: new KillRing(), message, macro: new KeyboardMacro() })
  const view = new EditorView({
    state: EditorState.create({
      doc: text,
      selection: EditorSelection.cursor(pos),
      extensions: [
        emacsState(),
        indentConfig.of({ dialect: DIALECTS[mode], indentTabs: true, tabWidth: 8 }),
      ],
    }),
  })
  commands[command](view)
  const head = view.state.selection.main.head
  const mark = view.state.field(markField)
  view.destroy()
  const shown = message.mock.calls.length ? (message.mock.lastCall as string[])[0] : null
  if (shown !== null && shown !== 'Mark set') return shown
  return command.startsWith('mark-') ? [head, mark] : head
}

describe('段落, 文, リスト, defun の移動 (Emacs の結果と比べる)', () => {
  it.each(cases)('%s: %j の %s', (mode, text, command, results) => {
    const actual = results.map((_, pos) => run(mode, text, command, pos))
    expect(actual).toEqual(results)
  })
})
