/**
 * Emacs のコマンド (CodeMirror の上で動かす) と, キーの割り当て.
 *
 * - マーク: `C-SPC` で置くと, 移動で範囲 (リージョン) が広がります (transient-mark-mode). 書き換えると外れます.
 * - 直前のコマンド: 続けて押したときに動きが変わるコマンド (`M-y`, `C-l`, 続けたキル, `C-n` の桁) のために, 覚えておきます.
 * - kill-ring: 全てのバッファで共通です ({@link KillRing}).
 */

import { history, redo, undo } from '@codemirror/commands'
import {
  Annotation,
  EditorSelection,
  type Extension,
  StateEffect,
  StateField,
} from '@codemirror/state'
import { EditorView, type KeyBinding } from '@codemirror/view'
import { backwardSexp, forwardSexp } from './bracketMatch'
import type { KillRing } from './killRing'
import { type CaseKind, changeCase, clampLine, wordBackward, wordForward } from './text'

/** 直前のコマンド. */
export interface LastCommand {
  /** 種類 (`kill`, `yank`, `line-move`, `recenter`, `move` など). */
  name: string
  /** ヤンクした範囲の始め (`yank`). */
  from?: number
  /** ヤンクした範囲の終わり (`yank`). */
  to?: number
  /** 行を移るときの桁 (`line-move`). */
  goal?: number
  /** 画面のどこにカーソルの行を置いたか (`recenter`. 0: 中央, 1: 上, 2: 下). */
  step?: number
}

/** コマンドが起こした変更に付ける, 直前のコマンド. */
const commandAnnotation = Annotation.define<LastCommand>()

/**
 * 直前のコマンド. コマンドでない変更 (文字の入力, マウスでの選択) があれば, 消えます.
 */
export const lastCommandField = StateField.define<LastCommand | null>({
  create: () => null,
  update: (value, tr) =>
    tr.annotation(commandAnnotation) ?? (tr.docChanged || tr.selection ? null : value),
})

/** マークを置く (外すときは `null`). */
const setMarkEffect = StateEffect.define<number | null>()

/**
 * マークの位置. 置いていなければ `null`. 書き換えると外れます (Emacs の transient-mark-mode).
 */
export const markField = StateField.define<number | null>({
  create: () => null,
  update(mark, tr) {
    for (const effect of tr.effects) {
      if (effect.is(setMarkEffect)) return effect.value
    }
    return tr.docChanged ? null : mark
  },
})

/**
 * Emacs のコマンドに要るエディタの拡張 (マーク, 直前のコマンド, 元に戻す履歴).
 *
 * @returns 拡張.
 */
export function emacsState(): Extension {
  return [markField, lastCommandField, history()]
}

/** コマンド (エディタを受け取って, 処理したら `true`). */
export type Command = (view: EditorView) => boolean

/** コマンドを作るのに要るもの. */
export interface CommandDeps {
  /** kill-ring. */
  killRing: KillRing
  /** ミニバッファに出す. */
  message: (text: string) => void
}

/**
 * カーソルを動かします. マークを置いていれば, マークからカーソルまでを選びます.
 *
 * @param view - エディタ.
 * @param head - 動かす先.
 * @param last - 直前のコマンドとして覚えること.
 * @returns `true`.
 */
function moveTo(view: EditorView, head: number, last: LastCommand = { name: 'move' }): boolean {
  const mark = view.state.field(markField)
  view.dispatch({
    selection: mark === null ? EditorSelection.cursor(head) : EditorSelection.range(mark, head),
    scrollIntoView: true,
    annotations: commandAnnotation.of(last),
  })
  return true
}

/**
 * カーソルの位置を返します.
 *
 * @param view - エディタ.
 * @returns 位置.
 */
function head(view: EditorView): number {
  return view.state.selection.main.head
}

/**
 * 行を移ります (論理行). 続けて移るあいだは, 始めの桁を保ちます. 先頭と末尾の行からは, 外へ出ません.
 *
 * @param view - エディタ.
 * @param delta - 移る行の数 (下が正).
 * @returns `true`.
 */
function lineMove(view: EditorView, delta: number): boolean {
  const { doc } = view.state
  const line = doc.lineAt(head(view))
  const last = view.state.field(lastCommandField)
  const goal = last?.name === 'line-move' ? (last.goal as number) : head(view) - line.from
  const target = doc.line(clampLine(line.number + delta, doc.lines))
  return moveTo(view, target.from + Math.min(goal, target.length), { name: 'line-move', goal })
}

/**
 * 1 画面に収まる行の数から, ページ送りで動く行の数を返します (重なりとして 2 行を残す).
 *
 * @param view - エディタ.
 * @returns 行の数 (少なくとも 1).
 */
export function pageLines(view: EditorView): number {
  const lines = Math.floor(view.scrollDOM.clientHeight / view.defaultLineHeight) - 2
  return Number.isFinite(lines) && lines > 1 ? lines : 1
}

/**
 * ページ送りをします. カーソルを動かして, その行が画面の上 (前へ戻るときは下) に来るようにします.
 *
 * @param view - エディタ.
 * @param direction - 1 で先へ (C-v), -1 で前へ (M-v).
 * @returns `true`.
 */
function scrollPage(view: EditorView, direction: 1 | -1): boolean {
  const { doc } = view.state
  const line = doc.lineAt(head(view))
  const target = doc.line(clampLine(line.number + direction * pageLines(view), doc.lines))
  const mark = view.state.field(markField)
  view.dispatch({
    selection:
      mark === null
        ? EditorSelection.cursor(target.from)
        : EditorSelection.range(mark, target.from),
    effects: EditorView.scrollIntoView(target.from, { y: direction === 1 ? 'start' : 'end' }),
    annotations: commandAnnotation.of({ name: 'move' }),
  })
  return true
}

/** `C-l` で, カーソルの行を置く場所 (押すたびに, 中央, 上, 下の順). */
const RECENTER = ['center', 'start', 'end'] as const

/**
 * カーソルの行を, 画面の中央に置きます. 続けて押すと, 上, 下の順に置きます (recenter-top-bottom).
 *
 * @param view - エディタ.
 * @returns `true`.
 */
function recenter(view: EditorView): boolean {
  const last = view.state.field(lastCommandField)
  const step = last?.name === 'recenter' ? ((last.step as number) + 1) % RECENTER.length : 0
  view.dispatch({
    effects: EditorView.scrollIntoView(head(view), { y: RECENTER[step] }),
    annotations: commandAnnotation.of({ name: 'recenter', step }),
  })
  return true
}

/**
 * Emacs のエディタのコマンドの組を作ります.
 *
 * @param deps - コマンドに要るもの.
 * @returns コマンド (Emacs のコマンドの名前で引く).
 */
export function createCommands({ killRing, message }: CommandDeps): Record<string, Command> {
  /**
   * 範囲を消して, kill-ring に入れます. 続けてキルしたときは, 1 つの項目につなげます.
   *
   * @param view - エディタ.
   * @param from - 始め.
   * @param to - 終わり.
   * @param direction - 消す向き (前へ消したときは, 項目の前につなげる).
   * @returns `true`.
   */
  const kill = (
    view: EditorView,
    from: number,
    to: number,
    direction: 'forward' | 'backward' = 'forward',
  ): boolean => {
    if (from === to) return true
    const joined = view.state.field(lastCommandField)?.name === 'kill'
    killRing.push(
      view.state.sliceDoc(from, to),
      joined ? (direction === 'forward' ? 'append' : 'prepend') : null,
    )
    view.dispatch({
      changes: { from, to },
      selection: EditorSelection.cursor(from),
      scrollIntoView: true,
      userEvent: 'delete',
      annotations: commandAnnotation.of({ name: 'kill' }),
    })
    return true
  }

  /**
   * カーソルからの単語の大文字・小文字を変えて, 単語の後へ動きます.
   *
   * @param view - エディタ.
   * @param kind - 変え方.
   * @returns `true`.
   */
  const caseWord = (view: EditorView, kind: CaseKind): boolean => {
    const from = head(view)
    const to = wordForward(view.state.doc.toString(), from)
    view.dispatch({
      changes: { from, to, insert: changeCase(view.state.sliceDoc(from, to), kind) },
      selection: EditorSelection.cursor(to),
      scrollIntoView: true,
      annotations: commandAnnotation.of({ name: 'case' }),
    })
    return true
  }

  /**
   * 文字を入れて, 入れた範囲を, 直前のヤンクとして覚えます.
   *
   * @param view - エディタ.
   * @param from - 始め.
   * @param to - 置き換える範囲の終わり.
   * @param text - 入れる文字.
   * @returns `true`.
   */
  const insertYank = (view: EditorView, from: number, to: number, text: string): boolean => {
    view.dispatch({
      changes: { from, to, insert: text },
      selection: EditorSelection.cursor(from + text.length),
      scrollIntoView: true,
      userEvent: 'input.paste',
      annotations: commandAnnotation.of({ name: 'yank', from, to: from + text.length }),
    })
    return true
  }

  const text = (view: EditorView) => view.state.doc.toString()

  return {
    // 移動
    'forward-char': (view) => moveTo(view, Math.min(head(view) + 1, view.state.doc.length)),
    'backward-char': (view) => moveTo(view, Math.max(head(view) - 1, 0)),
    'next-line': (view) => lineMove(view, 1),
    'previous-line': (view) => lineMove(view, -1),
    'move-beginning-of-line': (view) => moveTo(view, view.state.doc.lineAt(head(view)).from),
    'move-end-of-line': (view) => moveTo(view, view.state.doc.lineAt(head(view)).to),
    'forward-word': (view) => moveTo(view, wordForward(text(view), head(view))),
    'backward-word': (view) => moveTo(view, wordBackward(text(view), head(view))),
    'beginning-of-buffer': (view) => moveTo(view, 0),
    'end-of-buffer': (view) => moveTo(view, view.state.doc.length),
    'scroll-up-command': (view) => scrollPage(view, 1),
    'scroll-down-command': (view) => scrollPage(view, -1),
    'recenter-top-bottom': recenter,
    'forward-sexp': (view) => moveTo(view, forwardSexp(text(view), head(view))),
    'backward-sexp': (view) => moveTo(view, backwardSexp(text(view), head(view))),

    // 削除と編集
    'delete-char': (view) => {
      const from = head(view)
      if (from < view.state.doc.length) {
        view.dispatch({
          changes: { from, to: from + 1 },
          userEvent: 'delete',
          annotations: commandAnnotation.of({ name: 'delete' }),
        })
      }
      return true
    },
    'kill-line': (view) => {
      const from = head(view)
      const line = view.state.doc.lineAt(from)
      // 行末では, 改行を消す
      const to = from === line.to ? Math.min(line.to + 1, view.state.doc.length) : line.to
      return kill(view, from, to)
    },
    'kill-word': (view) => kill(view, head(view), wordForward(text(view), head(view))),
    'backward-kill-word': (view) =>
      kill(view, wordBackward(text(view), head(view)), head(view), 'backward'),
    'kill-sexp': (view) => kill(view, head(view), forwardSexp(text(view), head(view))),
    'kill-region': (view) => {
      const { from, to } = view.state.selection.main
      if (from === to) {
        message('The mark is not set now, so there is no region')
        return true
      }
      return kill(view, from, to)
    },
    'kill-ring-save': (view) => {
      const { from, to } = view.state.selection.main
      if (from === to) {
        message('The mark is not set now, so there is no region')
        return true
      }
      killRing.push(view.state.sliceDoc(from, to))
      view.dispatch({
        selection: EditorSelection.cursor(head(view)),
        effects: setMarkEffect.of(null),
        annotations: commandAnnotation.of({ name: 'kill' }),
      })
      return true
    },
    yank: (view) => {
      killRing.reset()
      const yanked = killRing.current()
      if (yanked === undefined) {
        message('Kill ring is empty')
        return true
      }
      const { from, to } = view.state.selection.main
      return insertYank(view, from, to, yanked)
    },
    'yank-pop': (view) => {
      const last = view.state.field(lastCommandField)
      if (last?.name !== 'yank') {
        message('Previous command was not a yank')
        return true
      }
      // ヤンクした後なので, kill-ring は空でない
      return insertYank(view, last.from as number, last.to as number, killRing.rotate() as string)
    },
    'transpose-chars': (view) => {
      const length = view.state.doc.length
      if (length < 2) return true
      const pos = head(view)
      // 入れ替える 2 文字の始め (行の始めでは次の 2 文字, 終わりでは前の 2 文字)
      const from = pos === 0 ? 0 : pos === length ? length - 2 : pos - 1
      const pair = view.state.sliceDoc(from, from + 2)
      view.dispatch({
        changes: { from, to: from + 2, insert: pair[1] + pair[0] },
        selection: EditorSelection.cursor(from + 2),
        annotations: commandAnnotation.of({ name: 'transpose' }),
      })
      return true
    },
    'upcase-word': (view) => caseWord(view, 'upcase'),
    'downcase-word': (view) => caseWord(view, 'downcase'),
    'capitalize-word': (view) => caseWord(view, 'capitalize'),
    'open-line': (view) => {
      const pos = head(view)
      view.dispatch({
        changes: { from: pos, insert: '\n' },
        selection: EditorSelection.cursor(pos),
        annotations: commandAnnotation.of({ name: 'open-line' }),
      })
      return true
    },
    undo: (view) => undo(view) || true,
    redo: (view) => redo(view) || true,

    // マークとリージョン
    'set-mark-command': (view) => {
      view.dispatch({
        selection: EditorSelection.cursor(head(view)),
        effects: setMarkEffect.of(head(view)),
        annotations: commandAnnotation.of({ name: 'set-mark' }),
      })
      message('Mark set')
      return true
    },
    'exchange-point-and-mark': (view) => {
      const mark = view.state.field(markField)
      if (mark === null) {
        message('No mark set in this buffer')
        return true
      }
      view.dispatch({
        selection: EditorSelection.range(head(view), mark),
        effects: setMarkEffect.of(head(view)),
        scrollIntoView: true,
        annotations: commandAnnotation.of({ name: 'move' }),
      })
      return true
    },
    'mark-whole-buffer': (view) => {
      const length = view.state.doc.length
      view.dispatch({
        selection: EditorSelection.range(length, 0),
        effects: setMarkEffect.of(length),
        annotations: commandAnnotation.of({ name: 'move' }),
      })
      return true
    },
    'keyboard-quit': (view) => {
      view.dispatch({
        selection: EditorSelection.cursor(head(view)),
        effects: setMarkEffect.of(null),
        annotations: commandAnnotation.of({ name: 'quit' }),
      })
      message('Quit')
      return true
    },
  }
}

/**
 * 1 から数えた行の始めへ動きます (goto-line).
 *
 * @param view - エディタ.
 * @param line - 行の番号 (範囲の外なら, 先頭か末尾の行).
 */
export function gotoLine(view: EditorView, line: number): void {
  const { doc } = view.state
  moveTo(view, doc.line(clampLine(line, doc.lines)).from)
}

/**
 * キーと, コマンドの名前の割り当て. キーは CodeMirror の書き方 (`Ctrl-x Ctrl-f` は, 続けて押すキー).
 * エディタのコマンド ({@link createCommands}) のほかに, 画面のコマンド (ファイル, バッファ, 評価など) も含みます.
 */
export const KEY_BINDINGS: readonly (readonly [key: string, command: string])[] = [
  // 移動
  ['Ctrl-f', 'forward-char'],
  ['Ctrl-b', 'backward-char'],
  ['Ctrl-n', 'next-line'],
  ['Ctrl-p', 'previous-line'],
  ['Ctrl-a', 'move-beginning-of-line'],
  ['Ctrl-e', 'move-end-of-line'],
  ['Alt-f', 'forward-word'],
  ['Alt-b', 'backward-word'],
  ['Alt-<', 'beginning-of-buffer'],
  ['Alt->', 'end-of-buffer'],
  ['Ctrl-v', 'scroll-up-command'],
  ['Alt-v', 'scroll-down-command'],
  ['Ctrl-l', 'recenter-top-bottom'],
  ['Alt-g g', 'goto-line'],
  ['Alt-g Alt-g', 'goto-line'],
  ['Ctrl-Alt-f', 'forward-sexp'],
  ['Ctrl-Alt-b', 'backward-sexp'],
  // 削除と編集
  ['Ctrl-d', 'delete-char'],
  ['Ctrl-k', 'kill-line'],
  ['Alt-d', 'kill-word'],
  ['Alt-Backspace', 'backward-kill-word'],
  ['Ctrl-Alt-k', 'kill-sexp'],
  ['Ctrl-w', 'kill-region'],
  ['Alt-w', 'kill-ring-save'],
  ['Ctrl-y', 'yank'],
  ['Alt-y', 'yank-pop'],
  ['Ctrl-t', 'transpose-chars'],
  ['Alt-u', 'upcase-word'],
  ['Alt-l', 'downcase-word'],
  ['Alt-c', 'capitalize-word'],
  ['Ctrl-o', 'open-line'],
  ['Ctrl-/', 'undo'],
  ['Ctrl-_', 'undo'],
  ['Ctrl-x u', 'undo'],
  ['Ctrl-x Ctrl-/', 'redo'],
  // マークとリージョン
  ['Ctrl-Space', 'set-mark-command'],
  ['Ctrl-x Ctrl-x', 'exchange-point-and-mark'],
  ['Ctrl-x h', 'mark-whole-buffer'],
  ['Ctrl-g', 'keyboard-quit'],
  // 検索
  ['Ctrl-s', 'isearch-forward'],
  ['Ctrl-r', 'isearch-backward'],
  // ファイルとバッファ
  ['Ctrl-x Ctrl-f', 'find-file'],
  ['Ctrl-x Ctrl-s', 'save-buffer'],
  ['Ctrl-x b', 'switch-to-buffer'],
  ['Ctrl-x k', 'kill-buffer'],
  ['Ctrl-x Ctrl-b', 'list-buffers'],
  ['Ctrl-x Ctrl-c', 'save-buffers-kill-terminal'],
  // コマンドと評価
  ['Alt-x', 'execute-extended-command'],
  ['Ctrl-x Ctrl-e', 'eval-last-sexp'],
  ['Alt-:', 'eval-expression'],
]

/**
 * キーの割り当てを, CodeMirror のキーの割り当てにします. コマンドは, 押したときに `lookup` で引きます.
 *
 * @param lookup - コマンドの名前から, コマンドを返す関数 (無ければ `undefined`).
 * @param extra - 足す割り当て (Lisp の `global-set-key`). 既定の割り当てより先に使います.
 * @returns キーの割り当て.
 */
export function emacsKeymap(
  lookup: (name: string) => Command | undefined,
  extra: readonly { key: string; command: string }[] = [],
): KeyBinding[] {
  const bindings = [...extra.map(({ key, command }) => [key, command] as const), ...KEY_BINDINGS]
  return bindings.map(([key, name]) => ({
    key,
    run: (view) => lookup(name)?.(view) ?? false,
    preventDefault: true,
  }))
}
