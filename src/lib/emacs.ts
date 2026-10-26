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
  type TransactionSpec,
} from '@codemirror/state'
import { EditorView, type KeyBinding } from '@codemirror/view'
import { commentDwim } from './comment'
import { findExpansions } from './dabbrev'
import { countMessage } from './count'
import { fillParagraph } from './fill'
import { killRectangle, stringRectangle, yankRectangle } from './rectangle'
import {
  columnAt,
  indentConfig,
  indentForTab,
  indentLines,
  indexAtColumn,
  newlineAndIndent,
} from './lispIndent'
import type { KillRing } from './killRing'
import type { KeyboardMacro } from './macro'
import { forwardParagraph, forwardSentence } from './paragraph'
import { beginningOfDefun, downList, endOfDefun, forwardSexp, markDefun, upList } from './sexp'
import { ScanError, type Source, isWhitespace } from './syntax'
import { deleteBlankLines, deleteHorizontalSpace, deleteIndentation, justOneSpace } from './spaces'
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
  /** 画面のどこにカーソルの行を置いたか (`recenter`. 0: 中央, 1: 上, 2: 下). 何番目の候補か (`dabbrev`). */
  step?: number
  /** 補完の略語 (`dabbrev`). */
  abbrev?: string
  /** 補完の候補 (`dabbrev`). */
  candidates?: string[]
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
  // マークを置くか外す変更があれば, その値. 本文を書き換えたら, 外す
  update(mark, tr) {
    for (const effect of tr.effects) {
      if (effect.is(setMarkEffect)) return effect.value
    }
    return tr.docChanged ? null : mark
  },
})

/** 行を移るときの桁を決める (止めるときは `null`). */
const goalColumnEffect = StateEffect.define<number | null>()

/** 行を移るときに, いつも使う桁 (`C-x C-n` の `goal-column`). 決めていなければ `null`. */
export const goalColumnField = StateField.define<number | null>({
  create: () => null,
  // 変える変更があれば, その値
  update(value, tr) {
    for (const effect of tr.effects) {
      if (effect.is(goalColumnEffect)) return effect.value
    }
    return value
  },
})

/** 次に押すキーを説明するか (describe-key) を変える. */
const describeKeyEffect = StateEffect.define<boolean>()

/** 次に押すキーを, 実行せずに説明するなら `true` (`C-h k`). */
export const describeKeyField = StateField.define<boolean>({
  create: () => false,
  // 変える変更があれば, その値
  update(value, tr) {
    for (const effect of tr.effects) {
      if (effect.is(describeKeyEffect)) return effect.value
    }
    return value
  },
})

/**
 * 次に押す文字を待つコマンド (`M-z` の文字, `C-x r s` のレジスタ) の, 名前と, 回数と, 押した文字.
 */
export interface ReadCharState {
  /** 文字を待つコマンドの名前. */
  command: string
  /** 前置引数 (`M-z` では, 何番目の文字までキルするか). */
  count: number
  /** 押した文字 (まだ押していなければ `undefined`). */
  char?: string
}

/** 文字を待つ (止めるときは `null`). */
const readCharEffect = StateEffect.define<ReadCharState | null>()

/** 次に押す文字を待っていれば, そのコマンドと文字. 待っていなければ `null`. */
export const readCharField = StateField.define<ReadCharState | null>({
  create: () => null,
  // 変える変更があれば, その値
  update(value, tr) {
    for (const effect of tr.effects) {
      if (effect.is(readCharEffect)) return effect.value
    }
    return value
  },
})

/** 次に押すキーを, そのまま入れる (`quoted-insert`) のを待っているときの, 回数と, 押したキー. */
export interface QuotedState {
  /** 入れる回数 (前置引数). */
  count: number
  /** 8 進数の文字のコード (`C-q 1 0 1` で `A`. 数字を押していなければ 0). */
  code: number
  /** 押した 8 進数の数字の数. */
  digits: number
  /** 入れる文字 (まだ押していなければ `undefined`). */
  char?: string
}

/** キーをそのまま入れるのを待つ (止めるときは `null`). */
const quotedEffect = StateEffect.define<QuotedState | null>()

/** キーをそのまま入れる (`C-q`) のを待っていれば, その状態. 待っていなければ `null`. */
export const quotedField = StateField.define<QuotedState | null>({
  create: () => null,
  // 変える変更があれば, その値
  update(value, tr) {
    for (const effect of tr.effects) {
      if (effect.is(quotedEffect)) return effect.value
    }
    return value
  },
})

/**
 * Emacs のコマンドに要るエディタの拡張 (マーク, 直前のコマンド, 元に戻す履歴, キーの説明, `M-z`, `C-q`).
 *
 * @returns 拡張.
 */
export function emacsState(): Extension {
  return [
    markField,
    lastCommandField,
    history(),
    describeKeyField,
    readCharField,
    quotedField,
    goalColumnField,
  ]
}

/** CodeMirror のキーの名前と, Emacs のキーの名前. */
const EMACS_KEY_NAMES: Record<string, string> = {
  Space: 'SPC',
  ' ': 'SPC',
  Enter: 'RET',
  Tab: 'TAB',
  Backspace: 'DEL',
  Escape: 'ESC',
  Delete: '<delete>',
  Insert: '<insert>',
  Home: '<home>',
  End: '<end>',
  PageUp: '<prior>',
  PageDown: '<next>',
  ArrowLeft: '<left>',
  ArrowRight: '<right>',
  ArrowUp: '<up>',
  ArrowDown: '<down>',
}

/** CodeMirror の修飾キーと, Emacs の修飾キー. */
const EMACS_MODIFIERS: Record<string, string> = {
  Ctrl: 'C-',
  Alt: 'M-',
  Shift: 'S-',
  Meta: 's-',
}

/**
 * CodeMirror のキーの書き方 (`Ctrl-x Ctrl-f`) を, Emacs の書き方 (`C-x C-f`) にします.
 *
 * @param keys - CodeMirror の書き方 (空白で区切ると, 続けて押すキー).
 * @returns Emacs の書き方.
 */
export function toEmacsKey(keys: string): string {
  return keys
    .split(' ')
    .map((stroke) => {
      const parts = stroke.split(/-(?!$)/)
      const key = parts.pop() as string
      const modifiers = parts.map((m) => EMACS_MODIFIERS[m] ?? `${m}-`).join('')
      const name = EMACS_KEY_NAMES[key] ?? (/^F\d+$/.test(key) ? `<${key.toLowerCase()}>` : key)
      return modifiers + name
    })
    .join(' ')
}

/**
 * 押したキーを, CodeMirror の書き方にします (修飾キーを付ける).
 *
 * @param event - キーの事象.
 * @returns CodeMirror の書き方.
 */
function eventKeyName(event: KeyboardEvent): string {
  const modifiers = [event.ctrlKey && 'Ctrl-', event.altKey && 'Alt-', event.metaKey && 'Meta-']
  return modifiers.filter(Boolean).join('') + event.key
}

/**
 * コマンド (エディタを受け取って, 処理したら `true`). 前置引数を引数として受け取るコマンド
 * ({@link ARGUMENT_COMMANDS}) には, 前置引数の数を渡します.
 */
export type Command = (view: EditorView, prefix?: number) => boolean

/** 前置引数の数の上限 (大きな数で, 繰り返しが終わらなくならないように). */
export const MAX_PREFIX_COUNT = 10000

/** 前置引数を, 繰り返しでなく, 引数として受け取るコマンド. */
const ARGUMENT_COMMANDS = new Set(['set-goal-column'])

/** コマンドを作るのに要るもの. */
export interface CommandDeps {
  /** kill-ring. */
  killRing: KillRing
  /** ミニバッファに出す. */
  message: (text: string) => void
  /** キーボードマクロ. */
  macro: KeyboardMacro
  /** レジスタ (名前の文字と, 中身). 省くと, コマンドの組ごとに作る. */
  registers?: Map<string, string>
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
 * `C-x C-n` で桁を決めていれば, その桁へ動きます.
 *
 * @param view - エディタ.
 * @param delta - 移る行の数 (下が正).
 * @returns `true`.
 */
function lineMove(view: EditorView, delta: number): boolean {
  const { doc } = view.state
  const line = doc.lineAt(head(view))
  const column = view.state.field(goalColumnField, false) ?? null
  if (column !== null) {
    const target = doc.line(clampLine(line.number + delta, doc.lines))
    const { tabWidth } = view.state.facet(indentConfig)
    return moveTo(view, target.from + indexAtColumn(target.text, column, tabWidth))
  }
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
export function createCommands({
  killRing,
  message,
  macro,
  registers = new Map(),
}: CommandDeps): Record<string, Command> {
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
   * リージョンの行を字下げします (Emacs の `indent-region`. 空の行は除く).
   *
   * @param view - エディタ.
   * @returns `true`.
   */
  const indentRegion = (view: EditorView): boolean => {
    const mark = view.state.field(markField)
    if (mark === null) {
      message('The mark is not set now, so there is no region')
      return true
    }
    const { doc } = view.state
    const from = Math.min(mark, head(view))
    const to = Math.max(mark, head(view))
    // 終わりが行の始めなら, その行は含めない
    const last = doc.lineAt(to)
    const end = to === last.from && to > from ? last.number - 1 : last.number
    const result = indentLines(view.state, { from: doc.lineAt(from).number, to: end })
    const changes = view.state.changes(result.changes)
    view.dispatch({
      changes,
      selection: EditorSelection.cursor(changes.mapPos(head(view), 1)),
      annotations: commandAnnotation.of({ name: 'indent' }),
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
   * リージョンの大文字・小文字を変えます. カーソルは, リージョンの同じ端に置きます.
   *
   * @param view - エディタ.
   * @param kind - 変え方.
   * @returns `true`.
   */
  const caseRegion = (view: EditorView, kind: CaseKind): boolean => {
    const mark = view.state.field(markField)
    if (mark === null) {
      message('The mark is not set now, so there is no region')
      return true
    }
    const from = Math.min(mark, head(view))
    const to = Math.max(mark, head(view))
    // `ß` の大文字は `SS` なので, 長さが変わることがある
    const insert = changeCase(view.state.sliceDoc(from, to), kind)
    view.dispatch({
      changes: { from, to, insert },
      selection: EditorSelection.cursor(head(view) === from ? from : from + insert.length),
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

  /**
   * 本文と方言を返します (式の走査に渡す).
   *
   * @param view - エディタ.
   * @returns 本文と方言.
   */
  const source = (view: EditorView): Source => ({
    text: text(view),
    dialect: view.state.facet(indentConfig).dialect,
  })

  /**
   * 式を走査して, することをします. 走査に失敗したら, 理由をミニバッファに出します (Emacs のエラー).
   *
   * @param action - すること.
   * @param failure - 括弧が合わないときに出す理由 (省くと, 走査の理由).
   * @returns `true`.
   */
  const attempt = (action: () => boolean, failure?: string): boolean => {
    try {
      return action()
    } catch (error) {
      message(error instanceof ScanError ? (failure ?? error.message) : (error as Error).message)
      return true
    }
  }

  /**
   * マークを置いて, リージョンを選びます.
   *
   * @param view - エディタ.
   * @param mark - マーク.
   * @param pos - カーソル.
   * @param fresh - 新しく置いたなら `true` (`Mark set` と出す. リージョンを延ばしたときは出さない).
   * @returns `true`.
   */
  const activateMark = (view: EditorView, mark: number, pos: number, fresh: boolean): boolean => {
    view.dispatch({
      selection: EditorSelection.range(mark, pos),
      effects: setMarkEffect.of(mark),
      scrollIntoView: true,
      annotations: commandAnnotation.of({ name: 'mark' }),
    })
    if (fresh) message('Mark set')
    return true
  }

  /**
   * マークから, 式や単語を越えた所へ, マークを動かします (`C-M-SPC`, `M-@`). リージョンが無ければ,
   * カーソルから越えた所に, マークを置きます. 続けると, リージョンを延ばします.
   *
   * @param view - エディタ.
   * @param move - 位置と向きから, 越えた所を返す関数.
   * @returns `true`.
   */
  const markForward = (view: EditorView, move: (from: number, count: number) => number) => {
    const mark = view.state.field(markField)
    // リージョンがあれば, カーソルから離れる向きに延ばす
    const count = mark !== null && mark < head(view) ? -1 : 1
    return activateMark(view, move(mark ?? head(view), count), head(view), mark === null)
  }

  /** 最後に消した矩形 (Emacs の `killed-rectangle`). */
  let killedRectangle: string[] = []

  /**
   * 次に押す文字を待ってから, コマンドを動かします. 押す前は, 待つことにして, 聞く文字を出します.
   * 待っているあいだに, 同じコマンドを前置引数で繰り返したときは, 回数を足します.
   *
   * @param view - エディタ.
   * @param name - コマンドの名前.
   * @param prompt - 聞く文字.
   * @param action - 押した文字と回数で, することを.
   * @returns `true`.
   */
  const readChar = (
    view: EditorView,
    name: string,
    prompt: string,
    action: (char: string, count: number) => boolean,
  ): boolean => {
    const pending = view.state.field(readCharField)
    if (pending?.command !== name || pending.char === undefined) {
      const count = pending?.command === name ? pending.count + 1 : 1
      view.dispatch({ effects: readCharEffect.of({ command: name, count }) })
      message(prompt)
      return true
    }
    view.dispatch({ effects: readCharEffect.of(null) })
    message('')
    return action(pending.char, pending.count)
  }

  /**
   * 空白を変える変更をします.
   *
   * @param view - エディタ.
   * @param spec - 変更とカーソル (無ければ, 何もしない).
   * @returns `true`.
   */
  const changeSpaces = (view: EditorView, spec: TransactionSpec | null): boolean => {
    if (spec) {
      view.dispatch({
        ...spec,
        scrollIntoView: true,
        userEvent: 'delete',
        annotations: commandAnnotation.of({ name: 'spaces' }),
      })
    }
    return true
  }

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
    'forward-sexp': (view) =>
      attempt(() => moveTo(view, forwardSexp(source(view), head(view), 1)), 'No next sexp'),
    'backward-sexp': (view) =>
      attempt(() => moveTo(view, forwardSexp(source(view), head(view), -1)), 'No previous sexp'),
    'backward-up-list': (view) => attempt(() => moveTo(view, upList(source(view), head(view), -1))),
    'up-list': (view) => attempt(() => moveTo(view, upList(source(view), head(view), 1))),
    'down-list': (view) => attempt(() => moveTo(view, downList(source(view), head(view)))),
    'beginning-of-defun': (view) => moveTo(view, beginningOfDefun(source(view), head(view))),
    'end-of-defun': (view) => attempt(() => moveTo(view, endOfDefun(source(view), head(view)))),
    'forward-paragraph': (view) =>
      moveTo(view, forwardParagraph(text(view), head(view), 1, source(view).dialect)),
    'backward-paragraph': (view) =>
      moveTo(view, forwardParagraph(text(view), head(view), -1, source(view).dialect)),
    'forward-sentence': (view) =>
      attempt(() => moveTo(view, forwardSentence(text(view), head(view), 1, source(view).dialect))),
    'backward-sentence': (view) =>
      moveTo(view, forwardSentence(text(view), head(view), -1, source(view).dialect)),
    // 行の, 字下げの後ろへ
    'back-to-indentation': (view) => {
      const line = view.state.doc.lineAt(head(view))
      const src = source(view)
      let pos = line.from
      while (pos < line.to && isWhitespace(src, pos)) pos++
      return moveTo(view, pos)
    },
    // 次の行から, 行を移るときに, いつも今の桁へ動く (前置引数があれば, 止める)
    'set-goal-column': (view, prefix) => {
      if (prefix !== undefined) {
        view.dispatch({ effects: goalColumnEffect.of(null) })
        message('No goal column')
        return true
      }
      const line = view.state.doc.lineAt(head(view))
      const { tabWidth } = view.state.facet(indentConfig)
      const column = columnAt(line.text, head(view) - line.from, tabWidth)
      view.dispatch({ effects: goalColumnEffect.of(column) })
      message(`Goal column ${column} (use C-x C-n with an arg to unset it)`)
      return true
    },

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
    'kill-sexp': (view) =>
      attempt(
        () => kill(view, head(view), forwardSexp(source(view), head(view), 1)),
        'No next sexp',
      ),
    'kill-sentence': (view) =>
      attempt(() =>
        kill(view, head(view), forwardSentence(text(view), head(view), 1, source(view).dialect)),
      ),
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
    // カーソルの前の単語 (単語の中なら, その単語) と, 後ろの単語を入れ替えて, 後ろの単語の後へ動く
    'transpose-words': (view) => {
      const doc = text(view)
      const firstFrom = wordBackward(doc, head(view))
      const firstTo = wordForward(doc, firstFrom)
      const secondTo = wordForward(doc, firstTo)
      const secondFrom = wordBackward(doc, secondTo)
      if (firstFrom === firstTo || secondFrom < firstTo) {
        message("Don't have two things to transpose")
        return true
      }
      view.dispatch({
        changes: {
          from: firstFrom,
          to: secondTo,
          insert:
            doc.slice(secondFrom, secondTo) +
            doc.slice(firstTo, secondFrom) +
            doc.slice(firstFrom, firstTo),
        },
        selection: EditorSelection.cursor(secondTo),
        scrollIntoView: true,
        annotations: commandAnnotation.of({ name: 'transpose' }),
      })
      return true
    },
    // カーソルの行と前の行を入れ替えて, 次の行の始めへ動く (先頭の行では, 次の行と入れ替える)
    'transpose-lines': (view) => {
      const { doc } = view.state
      if (doc.lines < 2) {
        message("Don't have two things to transpose")
        return true
      }
      const second = doc.line(Math.max(doc.lineAt(head(view)).number, 2))
      const first = doc.line(second.number - 1)
      // 最後の行は, 後ろに改行を足す (Emacs と同じ)
      const newline = second.number === doc.lines ? '\n' : ''
      view.dispatch({
        changes: {
          from: first.from,
          to: second.to,
          insert: `${second.text}\n${first.text}${newline}`,
        },
        selection: EditorSelection.cursor(second.to + 1),
        scrollIntoView: true,
        annotations: commandAnnotation.of({ name: 'transpose' }),
      })
      return true
    },
    // 次に押す文字を待ち, カーソルから, その文字 (前置引数があれば, その数だけ先の文字) までをキルする
    'zap-to-char': (view) =>
      readChar(view, 'zap-to-char', 'Zap to char: ', (char, count) => {
        // 大文字なら, 大文字と小文字を区別する (Emacs の `char-uppercase-p`)
        const fold = char === char.toLowerCase()
        const same = (c: string) => (fold ? c.toLowerCase() === char : c === char)
        const doc = text(view)
        let found = 0
        for (let i = head(view); i < doc.length; i++) {
          if (same(doc[i]) && ++found === count) return kill(view, head(view), i + 1)
        }
        message(`Search failed: "${char}"`)
        return true
      }),
    // リージョンを, 次に押す文字のレジスタに入れる
    'copy-to-register': (view) =>
      readChar(view, 'copy-to-register', 'Copy to register: ', (char) => {
        const mark = view.state.field(markField)
        if (mark === null) {
          message('The mark is not set now, so there is no region')
          return true
        }
        registers.set(
          char,
          view.state.sliceDoc(Math.min(mark, head(view)), Math.max(mark, head(view))),
        )
        view.dispatch({
          selection: EditorSelection.cursor(head(view)),
          effects: setMarkEffect.of(null),
          annotations: commandAnnotation.of({ name: 'register' }),
        })
        return true
      }),
    // 次に押す文字のレジスタの中身を入れる. カーソルは, 入れた文字の前に置く
    'insert-register': (view) =>
      readChar(view, 'insert-register', 'Insert register: ', (char) => {
        const content = registers.get(char)
        if (content === undefined) {
          message('Register does not contain text')
          return true
        }
        const { from, to } = view.state.selection.main
        view.dispatch({
          changes: { from, to, insert: content },
          selection: EditorSelection.cursor(from),
          scrollIntoView: true,
          userEvent: 'input.paste',
          annotations: commandAnnotation.of({ name: 'register' }),
        })
        return true
      }),
    // 矩形を消して, 覚える (kill-ring には入れない)
    'kill-rectangle': (view) => {
      const mark = view.state.field(markField)
      if (mark === null) {
        message('The mark is not set now, so there is no region')
        return true
      }
      const { spec, killed } = killRectangle(view.state, mark, head(view))
      killedRectangle = killed
      view.dispatch(spec, {
        effects: setMarkEffect.of(null),
        scrollIntoView: true,
        userEvent: 'delete',
        annotations: commandAnnotation.of({ name: 'kill-rectangle' }),
      })
      return true
    },
    // 最後に消した矩形を, カーソルを左上の角として入れる
    'yank-rectangle': (view) => {
      if (killedRectangle.length === 0) return true
      view.dispatch(yankRectangle(view.state, head(view), killedRectangle), {
        scrollIntoView: true,
        userEvent: 'input.paste',
        annotations: commandAnnotation.of({ name: 'yank-rectangle' }),
      })
      return true
    },
    // 次に押すキーを, そのまま入れる (制御文字も. 8 進数で, 文字のコードも入れられる)
    'quoted-insert': (view) => {
      const quoted = view.state.field(quotedField)
      if (quoted?.char === undefined) {
        // 前置引数で繰り返したときは, 回数を足す
        view.dispatch({
          effects: quotedEffect.of({ count: (quoted?.count ?? 0) + 1, code: 0, digits: 0 }),
        })
        message('C-q-')
        return true
      }
      view.dispatch(view.state.replaceSelection(quoted.char.repeat(quoted.count)), {
        effects: quotedEffect.of(null),
        scrollIntoView: true,
        userEvent: 'input.type',
        annotations: commandAnnotation.of({ name: 'quoted-insert' }),
      })
      message('')
      return true
    },
    // リージョンがあれば, リージョンを数え, 無ければ, バッファ全体を数える
    'count-words': (view) => {
      const { from, to } = view.state.selection.main
      const region = view.state.field(markField) !== null && from !== to
      message(
        region
          ? countMessage('Region', view.state.sliceDoc(from, to))
          : countMessage('Buffer', text(view)),
      )
      return true
    },
    'count-words-region': (view) => {
      if (view.state.field(markField) === null) {
        message('The mark is not set now, so there is no region')
        return true
      }
      const { from, to } = view.state.selection.main
      message(countMessage('Region', view.state.sliceDoc(from, to)))
      return true
    },
    'fill-paragraph': (view) => {
      const result = fillParagraph(view.state, head(view))
      if (result) {
        view.dispatch({
          ...result,
          scrollIntoView: true,
          userEvent: 'input',
          annotations: commandAnnotation.of({ name: 'fill' }),
        })
      }
      return true
    },
    'just-one-space': (view) => changeSpaces(view, justOneSpace(view.state, head(view))),
    'delete-horizontal-space': (view) =>
      changeSpaces(view, deleteHorizontalSpace(view.state, head(view))),
    'delete-indentation': (view) => changeSpaces(view, deleteIndentation(view.state, head(view))),
    'delete-blank-lines': (view) => changeSpaces(view, deleteBlankLines(view.state, head(view))),
    'upcase-word': (view) => caseWord(view, 'upcase'),
    'downcase-word': (view) => caseWord(view, 'downcase'),
    'capitalize-word': (view) => caseWord(view, 'capitalize'),
    'upcase-region': (view) => caseRegion(view, 'upcase'),
    'downcase-region': (view) => caseRegion(view, 'downcase'),
    'open-line': (view) => {
      const pos = head(view)
      view.dispatch({
        changes: { from: pos, insert: '\n' },
        selection: EditorSelection.cursor(pos),
        annotations: commandAnnotation.of({ name: 'open-line' }),
      })
      return true
    },

    // 改行と字下げ
    newline: (view) => {
      view.dispatch({
        ...newlineAndIndent(view.state, head(view)),
        scrollIntoView: true,
        userEvent: 'input',
        annotations: commandAnnotation.of({ name: 'newline' }),
      })
      return true
    },
    // electric-indent-mode のときは, 字下げしない
    'electric-newline-and-maybe-indent': (view) => {
      const pos = head(view)
      view.dispatch({
        changes: { from: pos, insert: '\n' },
        selection: EditorSelection.cursor(pos + 1),
        scrollIntoView: true,
        userEvent: 'input',
        annotations: commandAnnotation.of({ name: 'newline' }),
      })
      return true
    },
    // リージョンがあれば, リージョンを字下げする
    'indent-for-tab-command': (view) => {
      const mark = view.state.field(markField)
      if (mark !== null && mark !== head(view)) return indentRegion(view)
      view.dispatch({
        ...indentForTab(view.state, head(view)),
        scrollIntoView: true,
        annotations: commandAnnotation.of({ name: 'indent' }),
      })
      return true
    },
    'indent-region': (view) => indentRegion(view),
    // カーソルの後ろの式の, 2 行目からを字下げする (Lisp だけ)
    'indent-sexp': (view) => {
      if (!view.state.facet(indentConfig).dialect) return true
      const pos = head(view)
      const line = view.state.doc.lineAt(pos)
      const result = indentLines(view.state, {
        from: line.number + 1,
        to: view.state.doc.lines,
        skipBlank: true,
        whileDeeper: { line: line.number, index: pos - line.from },
      })
      view.dispatch({
        changes: result.changes,
        annotations: commandAnnotation.of({ name: 'indent' }),
      })
      return true
    },
    'comment-dwim': (view) => {
      const mark = view.state.field(markField)
      const result = commentDwim(view.state, head(view), mark)
      if (typeof result === 'string') {
        message(result)
        return true
      }
      view.dispatch({
        ...result,
        // リージョンを変えたら, マークを外す
        effects: mark !== null && mark !== head(view) ? setMarkEffect.of(null) : [],
        scrollIntoView: true,
        userEvent: 'input',
        annotations: commandAnnotation.of({ name: 'comment' }),
      })
      return true
    },
    // 続けて押すと, 次の候補にする. 候補が尽きたら, 略語に戻す
    'dabbrev-expand': (view) => {
      const last = view.state.field(lastCommandField)
      /**
       * 略語か候補を入れます.
       *
       * @param from - 略語の始め.
       * @param to - 置き換える範囲の終わり.
       * @param insert - 入れる文字.
       * @param next - 直前のコマンドとして覚えること.
       * @returns `true`.
       */
      const expand = (from: number, to: number, insert: string, next: LastCommand) => {
        view.dispatch({
          changes: { from, to, insert },
          selection: EditorSelection.cursor(from + insert.length),
          scrollIntoView: true,
          userEvent: 'input.complete',
          annotations: commandAnnotation.of(next),
        })
        return true
      }
      if (last?.name === 'dabbrev') {
        const { from, to, abbrev, candidates } = last as Required<LastCommand>
        const step = (last.step as number) + 1
        if (step >= candidates.length) {
          message(`No further dynamic expansion for ‘${abbrev}’ found`)
          return expand(from, to, abbrev, { name: 'move' })
        }
        const word = candidates[step]
        return expand(from, to, word, { ...last, to: from + word.length, step })
      }
      const pos = head(view)
      const found = findExpansions(text(view), pos, view.state.facet(indentConfig).dialect !== null)
      if (!found) {
        message('No possible abbreviation preceding point')
        return true
      }
      const { from, abbrev, candidates } = found
      if (candidates.length === 0) {
        message(`No dynamic expansion for ‘${abbrev}’ found`)
        return true
      }
      return expand(from, pos, candidates[0], {
        name: 'dabbrev',
        from,
        to: from + candidates[0].length,
        step: 0,
        abbrev,
        candidates,
      })
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
    'mark-word': (view) =>
      markForward(view, (from, count) =>
        count > 0 ? wordForward(text(view), from) : wordBackward(text(view), from),
      ),
    'mark-sexp': (view) => {
      const extend = view.state.field(markField) !== null
      try {
        return markForward(view, (from, count) => forwardSexp(source(view), from, count))
      } catch (error) {
        const { message: reason } = error as Error
        const premature = reason === 'Containing expression ends prematurely'
        message(premature ? (extend ? 'No more sexp to select' : 'No sexp to select') : reason)
        return true
      }
    },
    'mark-defun': (view) =>
      attempt(() => {
        const mark = view.state.field(markField)
        const region = markDefun(source(view), head(view), mark)
        return activateMark(view, region.mark, region.head, mark === null || mark === head(view))
      }),
    'mark-whole-buffer': (view) => {
      const length = view.state.doc.length
      view.dispatch({
        selection: EditorSelection.range(length, 0),
        effects: setMarkEffect.of(length),
        annotations: commandAnnotation.of({ name: 'move' }),
      })
      return true
    },

    // キーボードマクロ
    'start-kbd-macro': () => {
      if (macro.start()) {
        message('Defining kbd macro...')
      } else {
        // 記録には残さない
        macro.dropLastBinding()
        message('Already defining kbd macro')
      }
      return true
    },
    'end-kbd-macro': () => {
      message(macro.end() ? 'Keyboard macro defined' : 'Not defining kbd macro')
      return true
    },
    'call-last-kbd-macro': (view) => {
      if (macro.isRecording || macro.isReplaying) {
        // 記録には残さない
        macro.dropLastBinding()
        message('Cannot execute a keyboard macro while defining or executing one')
      } else if (!macro.isDefined) {
        message('No kbd macro has been defined')
      } else {
        macro.replay(view)
      }
      return true
    },

    // ヘルプ
    'describe-key': (view) => {
      view.dispatch({ effects: describeKeyEffect.of(true) })
      message('Describe the following key: ')
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
 * 矩形の各行を, 文字で置き換えます (`C-x r t`. 文字は, 画面がミニバッファで聞く).
 *
 * @param view - エディタ.
 * @param string - 置き換える文字.
 * @returns マークが無ければ `false`.
 */
export function replaceRectangle(view: EditorView, string: string): boolean {
  const mark = view.state.field(markField)
  if (mark === null) return false
  view.dispatch(stringRectangle(view.state, mark, view.state.selection.main.head, string), {
    effects: setMarkEffect.of(null),
    scrollIntoView: true,
    userEvent: 'input',
    annotations: commandAnnotation.of({ name: 'string-rectangle' }),
  })
  return true
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
  ['Ctrl-Alt-u', 'backward-up-list'],
  ['Ctrl-Alt-d', 'down-list'],
  ['Ctrl-Alt-a', 'beginning-of-defun'],
  ['Ctrl-Alt-e', 'end-of-defun'],
  ['Alt-{', 'backward-paragraph'],
  ['Alt-}', 'forward-paragraph'],
  ['Alt-a', 'backward-sentence'],
  ['Alt-e', 'forward-sentence'],
  ['Alt-m', 'back-to-indentation'],
  ['Ctrl-x Ctrl-n', 'set-goal-column'],
  // 削除と編集
  ['Ctrl-d', 'delete-char'],
  ['Ctrl-k', 'kill-line'],
  ['Alt-d', 'kill-word'],
  ['Alt-Backspace', 'backward-kill-word'],
  ['Ctrl-Alt-k', 'kill-sexp'],
  ['Alt-k', 'kill-sentence'],
  ['Ctrl-w', 'kill-region'],
  ['Alt-w', 'kill-ring-save'],
  ['Ctrl-y', 'yank'],
  ['Alt-y', 'yank-pop'],
  ['Ctrl-t', 'transpose-chars'],
  ['Alt-t', 'transpose-words'],
  ['Ctrl-x Ctrl-t', 'transpose-lines'],
  ['Alt-;', 'comment-dwim'],
  ['Alt-/', 'dabbrev-expand'],
  ['Alt-z', 'zap-to-char'],
  ['Alt-q', 'fill-paragraph'],
  ['Ctrl-q', 'quoted-insert'],
  ['Alt-=', 'count-words-region'],
  // 矩形, レジスタ, ブックマーク
  ['Ctrl-x r k', 'kill-rectangle'],
  ['Ctrl-x r y', 'yank-rectangle'],
  ['Ctrl-x r t', 'string-rectangle'],
  ['Ctrl-x r s', 'copy-to-register'],
  ['Ctrl-x r x', 'copy-to-register'],
  ['Ctrl-x r i', 'insert-register'],
  ['Ctrl-x r g', 'insert-register'],
  ['Ctrl-x r m', 'bookmark-set'],
  ['Ctrl-x r b', 'bookmark-jump'],
  ['Alt-Space', 'just-one-space'],
  ['Alt-\\', 'delete-horizontal-space'],
  ['Alt-^', 'delete-indentation'],
  ['Ctrl-x Ctrl-o', 'delete-blank-lines'],
  ['Alt-u', 'upcase-word'],
  ['Alt-l', 'downcase-word'],
  ['Alt-c', 'capitalize-word'],
  ['Ctrl-x Ctrl-u', 'upcase-region'],
  ['Ctrl-x Ctrl-l', 'downcase-region'],
  ['Ctrl-o', 'open-line'],
  // 改行と字下げ
  ['Enter', 'newline'],
  ['Ctrl-m', 'newline'],
  ['Ctrl-j', 'electric-newline-and-maybe-indent'],
  ['Tab', 'indent-for-tab-command'],
  ['Ctrl-i', 'indent-for-tab-command'],
  ['Ctrl-Alt-q', 'indent-sexp'],
  ['Ctrl-Alt-\\', 'indent-region'],
  ['Ctrl-/', 'undo'],
  ['Ctrl-_', 'undo'],
  ['Ctrl-x u', 'undo'],
  ['Ctrl-x Ctrl-/', 'redo'],
  // マークとリージョン
  ['Ctrl-Space', 'set-mark-command'],
  ['Ctrl-x Ctrl-x', 'exchange-point-and-mark'],
  ['Ctrl-x h', 'mark-whole-buffer'],
  ['Alt-@', 'mark-word'],
  ['Ctrl-Alt-Space', 'mark-sexp'],
  ['Ctrl-Alt-h', 'mark-defun'],
  ['Ctrl-g', 'keyboard-quit'],
  ['Ctrl-]', 'abort-recursive-edit'],
  // ヘルプ
  ['Ctrl-h k', 'describe-key'],
  ['Ctrl-h b', 'describe-bindings'],
  // 前置引数
  ['Ctrl-u', 'universal-argument'],
  ...Array.from({ length: 10 }, (_, d) => [`Alt-${d}`, 'digit-argument'] as const),
  // キーボードマクロ
  ['Ctrl-x (', 'start-kbd-macro'],
  ['Ctrl-x )', 'end-kbd-macro'],
  ['Ctrl-x e', 'call-last-kbd-macro'],
  // 検索と置換
  ['Ctrl-s', 'isearch-forward'],
  ['Ctrl-r', 'isearch-backward'],
  ['Ctrl-Alt-s', 'isearch-forward-regexp'],
  ['Ctrl-Alt-r', 'isearch-backward-regexp'],
  ['Alt-%', 'query-replace'],
  ['Ctrl-Alt-%', 'query-replace-regexp'],
  // ファイルとバッファ
  ['Ctrl-x Ctrl-f', 'find-file'],
  ['Ctrl-x Ctrl-s', 'save-buffer'],
  ['Ctrl-x Ctrl-w', 'write-file'],
  ['Ctrl-x Ctrl-v', 'find-alternate-file'],
  ['Ctrl-x i', 'insert-file'],
  ['Ctrl-x s', 'save-some-buffers'],
  ['Ctrl-x d', 'dired'],
  ['Alt-g n', 'next-error'],
  ['Alt-g Alt-n', 'next-error'],
  ['Ctrl-x `', 'next-error'],
  ['Alt-g p', 'previous-error'],
  ['Alt-g Alt-p', 'previous-error'],
  ['F8', 'project-tree'],
  ['Ctrl-x b', 'switch-to-buffer'],
  ['Ctrl-x k', 'kill-buffer'],
  ['Ctrl-x Ctrl-b', 'list-buffers'],
  ['Ctrl-x Ctrl-c', 'save-buffers-kill-terminal'],
  // ウィンドウ
  ['Ctrl-x 2', 'split-window-below'],
  ['Ctrl-x 3', 'split-window-right'],
  ['Ctrl-x o', 'other-window'],
  ['Ctrl-x 0', 'delete-window'],
  ['Ctrl-x 1', 'delete-other-windows'],
  // コマンドと評価
  ['Alt-x', 'execute-extended-command'],
  ['Ctrl-x Ctrl-e', 'eval-last-sexp'],
  ['Alt-:', 'eval-expression'],
]

/** 修飾キーだけを押したときの, キーの名前. */
const MODIFIER_KEYS = ['Control', 'Alt', 'Shift', 'Meta']

/** `C-q` の後に押すと, 文字として入れる, 文字でないキー (Emacs の RET は `^M` だが, 改行にする). */
const QUOTED_KEYS: Record<string, string> = {
  Enter: '\n',
  Tab: '\t',
  Backspace: '\x7f',
  Escape: '\x1b',
}

/** `C-` を付けた記号と, 制御文字のコード. */
const CONTROL_SYMBOLS: Record<string, number> = {
  '@': 0,
  ' ': 0,
  '[': 27,
  '\\': 28,
  ']': 29,
  '^': 30,
  _: 31,
  '?': 127,
}

/**
 * `C-q` の後に押したキーの, 入れる文字を返します.
 *
 * @param event - キーの事象.
 * @returns 文字. 入れられないキー (`M-` を付けたもの, 矢印など) なら `null`.
 */
export function quotedChar(event: KeyboardEvent): string | null {
  if (event.altKey || event.metaKey) return null
  if (event.ctrlKey) {
    const key = event.key.toUpperCase()
    // C-m は, RET と同じく, 改行にする
    if (key === 'M') return '\n'
    if (/^[A-Z]$/.test(key)) return String.fromCharCode(key.charCodeAt(0) - 64)
    const code = CONTROL_SYMBOLS[event.key]
    return code === undefined ? null : String.fromCharCode(code)
  }
  if (event.key.length === 1 || /^[\ud800-\udbff]/.test(event.key)) return event.key
  return QUOTED_KEYS[event.key] ?? null
}

/** 割り当てのあるキーのうち, `M-z` や `C-x r s` の後に押すと, 文字として使うもの. */
const READ_CHAR_KEYS: Record<string, string> = {
  Enter: '\n',
  'Ctrl-m': '\n',
  'Ctrl-j': '\n',
  Tab: '\t',
  'Ctrl-i': '\t',
}

/** キーの割り当ての作り方. */
export interface KeymapOptions {
  /** 足す割り当て (Lisp の `global-set-key`). 既定の割り当てより先に使います. */
  extra?: readonly { key: string; command: string }[]
  /** 前置引数 (`C-u 4`) の数だけ繰り返すコマンドなら `true` (既定は全て). */
  repeatable?: (name: string) => boolean
  /** キーボードマクロ (割り当てが動いたことを知らせる). */
  macro?: KeyboardMacro
  /** ミニバッファに出す (前置引数の入力中の表示). */
  message?: (text: string) => void
}

/**
 * キーの割り当てを, CodeMirror のキーの割り当てにします. コマンドは, 押したときに `lookup` で引きます.
 *
 * 前置引数: `C-u` は 4 (続けて押すと 4 倍), `C-u` の後の数字と `M-0` から `M-9` は, その数です.
 * 次のコマンドを, その数だけ繰り返します. 割り当てのない文字は, その数だけ入れます.
 * 数は {@link MAX_PREFIX_COUNT} までです.
 *
 * @param lookup - コマンドの名前から, コマンドを返す関数 (無ければ `undefined`).
 * @param options - 作り方.
 * @returns キーの割り当て.
 */
export function emacsKeymap(
  lookup: (name: string) => Command | undefined,
  { extra = [], repeatable = () => true, macro, message = () => {} }: KeymapOptions = {},
): KeyBinding[] {
  // 入力中の前置引数 (無ければ count は null)
  const prefix: { count: number | null; digits: string } = { count: null, digits: '' }
  const clearPrefix = () => {
    prefix.count = null
    prefix.digits = ''
  }
  const addDigit = (digit: string) => {
    prefix.digits += digit
    prefix.count = Math.min(Number(prefix.digits), MAX_PREFIX_COUNT)
    message(`C-u ${prefix.count}-`)
  }

  /**
   * 文字を待つコマンド (`M-z` など) に, 押した文字を渡して動かします.
   *
   * @param view - エディタ.
   * @param char - 文字.
   * @returns `true`.
   */
  const readCharTo = (view: EditorView, char: string): boolean => {
    const pending = view.state.field(readCharField) as ReadCharState
    view.dispatch({ effects: readCharEffect.of({ ...pending, char }) })
    return (lookup(pending.command) as Command)(view)
  }

  /**
   * 割り当てたコマンドを動かします.
   *
   * @param view - エディタ.
   * @param key - キー.
   * @param name - コマンドの名前.
   * @returns 処理したら `true`.
   */
  const run = (view: EditorView, key: string, name: string): boolean => {
    if (view.state.field(readCharField, false)) {
      if (READ_CHAR_KEYS[key] !== undefined) return readCharTo(view, READ_CHAR_KEYS[key])
      // 文字でないキーなら, 待つのを止めて, そのキーのコマンドを動かす
      view.dispatch({ effects: readCharEffect.of(null) })
    }
    if (view.state.field(describeKeyField, false)) {
      view.dispatch({ effects: describeKeyEffect.of(false) })
      message(`${toEmacsKey(key)} runs the command ${name}`)
      return true
    }
    if (name === 'universal-argument') {
      // 数字を入れた後は, 数を変えない
      if (prefix.digits === '') prefix.count = Math.min((prefix.count ?? 1) * 4, MAX_PREFIX_COUNT)
      message(`C-u ${prefix.count}-`)
      return true
    }
    if (name === 'digit-argument') {
      addDigit(key.slice(-1))
      return true
    }
    const command = lookup(name)
    const count = prefix.count
    if (count !== null) {
      clearPrefix()
      message('')
    }
    if (!command) return false
    macro?.noteBinding(key.split(' ').length)
    if (count !== null && ARGUMENT_COMMANDS.has(name)) return command(view, count)
    if (count === null || !repeatable(name)) return command(view)
    for (let i = 0; i < count; i++) command(view)
    return true
  }

  /**
   * `C-q` の後に押したキーを, 入れます. 8 進数の数字は, 3 つまで続けて押せます. 数字の後に,
   * 数字でないキーを押すと, そのコードの文字を入れ, そのキーも動かします (RET は, 入れて終わり).
   *
   * @param view - エディタ.
   * @param event - キーの事象.
   * @returns 処理したら `true` (後のキーの割り当てを動かさない).
   */
  const quotedKey = (view: EditorView, event: KeyboardEvent): boolean => {
    const quoted = view.state.field(quotedField, false)
    if (!quoted || MODIFIER_KEYS.includes(event.key)) return false
    const plain = !event.ctrlKey && !event.altKey && !event.metaKey
    /** 文字を入れます. */
    const insert = (char: string) => {
      view.dispatch({ effects: quotedEffect.of({ ...quoted, char }) })
      ;(lookup('quoted-insert') as Command)(view)
    }
    if (plain && /^[0-7]$/.test(event.key)) {
      const next = {
        ...quoted,
        code: quoted.code * 8 + Number(event.key),
        digits: quoted.digits + 1,
      }
      if (next.digits === 3) {
        insert(String.fromCodePoint(next.code))
      } else {
        view.dispatch({ effects: quotedEffect.of(next) })
        message(`C-q ${next.code.toString(8)}-`)
      }
      return true
    }
    if (quoted.digits > 0) {
      insert(String.fromCodePoint(quoted.code))
      return plain && event.key === 'Enter'
    }
    const char = quotedChar(event)
    if (char === null) {
      // 入れられないキーなら, 待つのを止めて, そのキーのコマンドを動かす
      view.dispatch({ effects: quotedEffect.of(null) })
      message('')
      return false
    }
    insert(char)
    return true
  }

  const bindings = [...extra.map(({ key, command }) => [key, command] as const), ...KEY_BINDINGS]
  return [
    // `C-q` の後のキー. CodeMirror は, `any` を, 後から足す割り当ての先に動かすので, 始めに置く
    // (C-x などの前置きのキーは, CodeMirror が先に受けるので, そのまま入れられない)
    { any: quotedKey },
    ...bindings.map(([key, name]) => ({
      key,
      run: (view: EditorView) => run(view, key, name),
      preventDefault: true,
    })),
    // 前置引数の入力中の, 数字と, 割り当てのない文字. CodeMirror は, `any` を, 後から足す割り当ての先に動かすので,
    // 最後に置く (割り当てのあるキーでは, コマンドを先に動かす)
    {
      any: (view, event) => {
        if (view.state.field(describeKeyField, false)) {
          // 修飾キーだけのときは, 次のキーを待つ
          if (MODIFIER_KEYS.includes(event.key)) return false
          view.dispatch({ effects: describeKeyEffect.of(false) })
          const name = toEmacsKey(eventKeyName(event))
          const plain = event.key.length === 1 && !event.ctrlKey && !event.altKey && !event.metaKey
          message(plain ? `${name} runs the command self-insert-command` : `${name} is undefined`)
          return true
        }
        if (view.state.field(readCharField, false)) {
          // 修飾キーだけのときは, 次のキーを待つ. 文字でなければ, 待つのを止める
          if (MODIFIER_KEYS.includes(event.key)) return false
          if (event.key.length === 1 && !event.ctrlKey && !event.altKey && !event.metaKey) {
            return readCharTo(view, event.key)
          }
          view.dispatch({ effects: readCharEffect.of(null) })
          return false
        }
        if (prefix.count === null || event.ctrlKey || event.altKey || event.metaKey) return false
        if (/^[0-9]$/.test(event.key)) {
          addDigit(event.key)
          return true
        }
        if (event.key.length !== 1) return false
        const text = event.key.repeat(prefix.count)
        clearPrefix()
        message('')
        view.dispatch(view.state.replaceSelection(text), {
          scrollIntoView: true,
          userEvent: 'input.type',
        })
        return true
      },
    },
  ]
}
