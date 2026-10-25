/**
 * 検索と置換のコマンド. isearch (C-s, C-r, C-M-s, C-M-r) と, 置換 (M-%, C-M-%, `replace-string`,
 * `replace-regexp`) を受け持ちます.
 */

import { useRef, useState } from 'react'
import type { EditorHandle } from '../components/CodeEditor'
import type {
  IsearchCommand,
  MinibufferIsearchState,
  MinibufferState,
} from '../components/Minibuffer'
import { SEARCH_RING_MAX, addToHistory } from '../lib/history'
import { compileRegexp, findNextMatch, quoteRegexp, yankWordOrCharEnd } from '../lib/isearch'
import {
  type ReplaceMatch,
  type ReplaceSpec,
  findReplaceMatch,
  replacedMessage,
  replaceRemaining,
} from '../lib/replace'
import type { AppContext } from './useBuffers'
import type { CommandSet } from './useMinibuffer'

/** 置換の種類 (query-replace, replace-string と, その正規表現の版). */
interface ReplaceKind {
  /** 1 つずつ聞くなら `true`. */
  query: boolean
  /** 正規表現で探すなら `true`. */
  regexp: boolean
}

/** 進めている置換. */
interface ReplaceState {
  /** 置換の種類. */
  kind: ReplaceKind
  /** 探す文字 (入力したら). */
  from?: string
  /** 置換の指定と, 聞いている所 (query-replace で, 聞いているあいだ). */
  spec?: ReplaceSpec
  /** query-replace で, 聞いている所. */
  match?: ReplaceMatch
  /** 置き換えた数. */
  count: number
}

/**
 * 置換の名前 (プロンプトに出す) を返します.
 *
 * @param kind - 置換の種類.
 * @returns 名前.
 */
function replaceTitle({ query, regexp }: ReplaceKind): string {
  if (query) return regexp ? 'Query replace regexp' : 'Query replace'
  return regexp ? 'Replace regexp' : 'Replace string'
}

/**
 * 検索と置換のコマンドを作ります.
 *
 * @param ctx - ミニバッファと, バッファとウィンドウ.
 * @returns コマンドと, isearch の操作と, 1 つのキーで答える問いと, やめるときの関数.
 */
export function useSearch(ctx: AppContext) {
  const { currentBuffer, cursor, editor, setPendingAction, setMinibufferState, showMessage } = ctx
  // 進めている置換
  const [replace, setReplace] = useState<ReplaceState | null>(null)
  // isearch の検索の履歴 (文字と, 正規表現. 新しい順)
  const searchRing = useRef({ plain: [] as string[], regexp: [] as string[] })

  /**
   * 見つかったところを選びます. カーソルは, 前へ探すときは終わりに, 後ろへ探すときは始めに置きます.
   *
   * @param direction - 探す向き.
   * @param match - 見つかったところ.
   */
  const selectMatch = (
    direction: 'forward' | 'backward',
    match: { start: number; end: number },
  ) => {
    if (direction === 'forward') {
      editor.current?.select(match.start, match.end)
    } else {
      editor.current?.select(match.end, match.start)
    }
  }

  /**
   * isearch を始めます.
   *
   * @param direction - 探す向き.
   * @param regexp - 正規表現で探すなら `true`.
   */
  const handleIsearch = (direction: 'forward' | 'backward', regexp: boolean) => {
    setMinibufferState({
      mode: 'isearch',
      direction,
      regexp,
      query: '',
      originalCursor: cursor.offset,
      matchStart: null,
      matchEnd: null,
    })
  }

  /**
   * isearch で, 同じ文字の次 (または前) に合う所へ進みます (isearch 中の C-s / C-r).
   * 前へ探していて, 本文の始めで合っている場合は, 失敗にします (もう 1 度押すと, 現在位置から探し直す).
   *
   * @param direction - 探す向き.
   * @param current - isearch の状態.
   */
  const handleIsearchRepeat = (
    direction: 'forward' | 'backward',
    current: MinibufferIsearchState,
  ) => {
    // 探す文字が空なら, 最後に探した文字で探す
    if (current.query === '' && direction === current.direction) {
      const ring = searchRing.current[current.regexp ? 'regexp' : 'plain']
      if (ring.length > 0) searchIsearch(current, ring[0], { ringIndex: 0 })
      return
    }
    if (direction === 'backward' && current.matchStart !== null && current.matchStart <= 0) {
      setMinibufferState({ ...current, matchStart: null, matchEnd: null, yankBase: undefined })
      return
    }

    const fromPos =
      current.matchStart === null
        ? current.originalCursor
        : direction === 'forward'
          ? current.matchStart + 1
          : current.matchStart - 1

    const match = findNextMatch(
      currentBuffer.content,
      current.query,
      fromPos,
      direction,
      current.regexp,
    )

    if (match) {
      selectMatch(direction, match)
      setMinibufferState({
        ...current,
        direction,
        matchStart: match.start,
        matchEnd: match.end,
        yankBase: undefined,
      })
    } else {
      setMinibufferState({ ...current, matchStart: null, matchEnd: null, yankBase: undefined })
    }
  }

  /**
   * isearch の探す文字を変えて, 探し始めた位置から探し直します.
   *
   * @param current - isearch の状態.
   * @param query - 探す文字.
   * @param changes - ほかに変える項目 (続けた M-y の文字などは, 変えなければ捨てる).
   */
  const searchIsearch = (
    current: MinibufferIsearchState,
    query: string,
    changes: Partial<MinibufferIsearchState> = {},
  ) => {
    const match = findNextMatch(
      currentBuffer.content,
      query,
      current.originalCursor,
      current.direction,
      current.regexp,
    )
    if (match) selectMatch(current.direction, match)
    setMinibufferState({
      ...current,
      yankBase: undefined,
      ...changes,
      query,
      matchStart: match ? match.start : null,
      matchEnd: match ? match.end : null,
    })
  }

  /**
   * isearch の中のコマンドを行います. C-w でカーソルの後ろの単語か文字を, M-y でキルした文字を取り込み,
   * M-p と M-n で, 検索の履歴を出します.
   *
   * @param command - コマンド.
   * @param current - isearch の状態.
   */
  const handleIsearchCommand = (command: IsearchCommand, current: MinibufferIsearchState) => {
    const quote = (text: string) => (current.regexp ? quoteRegexp(text) : text)
    const handle = editor.current as EditorHandle
    if (command === 'yank-word-or-char') {
      // 合っていれば, 合った所の終わりから (前へ探していても)
      const text = currentBuffer.content
      const from = current.matchEnd ?? current.yankPoint ?? current.originalCursor
      const end = yankWordOrCharEnd(text, from)
      if (end === from) return
      const failing = current.matchStart === null && current.query !== ''
      searchIsearch(current, current.query + quote(text.slice(from, end)), {
        yankPoint: failing ? end : null,
      })
    } else if (command === 'yank-pop') {
      // 続けた M-y は, 取り込んだ文字を, 1 つ古いキルした文字に置き換える
      const base = current.yankBase ?? current.query
      const kill = current.yankBase === undefined ? handle.currentKill() : handle.rotateKill()
      if (kill === undefined) return
      searchIsearch(current, base + quote(kill), { yankBase: base })
    } else {
      const ring = searchRing.current[current.regexp ? 'regexp' : 'plain']
      if (ring.length === 0) return
      const advance = command === 'ring-advance'
      const index =
        ((current.ringIndex ?? (advance ? 0 : -1)) + (advance ? -1 : 1) + ring.length) % ring.length
      searchIsearch(current, ring[index], { ringIndex: index })
    }
  }

  /**
   * isearch を確定します. 見つかったところの, 探した向きの側にカーソルを置き, 探した文字を検索の履歴に加えます.
   *
   * @param current - isearch の状態.
   */
  const finishIsearch = (current: MinibufferIsearchState) => {
    const kind = current.regexp ? 'regexp' : 'plain'
    searchRing.current[kind] = addToHistory(
      searchRing.current[kind],
      current.query,
      SEARCH_RING_MAX,
      true,
    )
    if (current.matchStart !== null) {
      editor.current?.setCursor(
        current.direction === 'forward' ? (current.matchEnd as number) : current.matchStart,
      )
    }
    setMinibufferState({ mode: 'message', text: '' })
  }

  /**
   * 置換を始めます (探す文字を聞く).
   *
   * @param kind - 置換の種類.
   */
  const handleReplace = (kind: ReplaceKind) => {
    setReplace({ kind, count: 0 })
    setPendingAction('replace-from')
    setMinibufferState({ mode: 'input', prompt: `${replaceTitle(kind)}: `, input: '' })
  }

  /**
   * 探す文字を覚えて, 置き換える文字を聞きます. 空か, 正しくない正規表現なら, やめます.
   *
   * @param value - 探す文字.
   */
  const askReplaceTo = (value: string) => {
    const state = replace as ReplaceState
    if (value === '') {
      setReplace(null)
      showMessage('')
    } else if (state.kind.regexp && !compileRegexp(value)) {
      setReplace(null)
      showMessage(`Invalid regexp: ${value}`)
    } else {
      setReplace({ ...state, from: value })
      setPendingAction('replace-to')
      setMinibufferState({
        mode: 'input',
        prompt: `${replaceTitle(state.kind)} ${value} with: `,
        input: '',
      })
    }
  }

  /**
   * 置き換える文字で, 置換を始めます (query-replace なら, 1 つずつ聞く).
   *
   * @param value - 置き換える文字.
   */
  const startReplace = (value: string) => {
    const state = replace as ReplaceState
    const spec = { from: state.from as string, to: value, regexp: state.kind.regexp }
    const pos = (editor.current as EditorHandle).point()
    if (state.kind.query) {
      askReplace({ ...state, spec }, pos)
    } else {
      const done = replaceRemaining(editor.current as EditorHandle, spec, pos)
      finishReplace(done.count, done.pos)
    }
  }

  /**
   * query-replace で, 置き換える所を選んで聞きます. 無ければ, 終えます.
   *
   * @param state - 進めている置換 (spec がある).
   * @param pos - 探し始める位置.
   */
  const askReplace = (state: ReplaceState, pos: number) => {
    const handle = editor.current as EditorHandle
    const spec = state.spec as ReplaceSpec
    const match = findReplaceMatch(handle.text(), spec, pos)
    if (!match) {
      finishReplace(state.count, pos)
      return
    }
    handle.select(match.start, match.end)
    setReplace({ ...state, match })
    setMinibufferState({
      mode: 'query',
      prompt: `Query replacing ${spec.from} with ${spec.to}: (y, n, !, ., q) `,
    })
  }

  /**
   * 置換を終えて, 置き換えた数を出します.
   *
   * @param count - 置き換えた数.
   * @param pos - カーソルを置く位置.
   */
  const finishReplace = (count: number, pos: number) => {
    editor.current?.setCursor(pos)
    setReplace(null)
    showMessage(replacedMessage(count))
  }

  /**
   * query-replace の問いに答えます.
   *
   * @param key - 押したキー.
   */
  const answerQuery = (key: string) => {
    const state = replace as ReplaceState
    const match = state.match as ReplaceMatch
    const handle = editor.current as EditorHandle
    const replaceMatch = () => {
      handle.replace(match.start, match.end, match.replacement)
      return match.start + match.replacement.length
    }
    if (key === 'y' || key === ' ') {
      const pos = replaceMatch()
      askReplace({ ...state, count: state.count + 1 }, pos)
    } else if (key === 'n' || key === 'Backspace' || key === 'Delete') {
      askReplace(state, match.end)
    } else if (key === '!') {
      const rest = replaceRemaining(handle, state.spec as ReplaceSpec, replaceMatch())
      finishReplace(state.count + 1 + rest.count, rest.pos)
    } else if (key === '.') {
      finishReplace(state.count + 1, replaceMatch())
    } else if (key === 'q' || key === 'Enter') {
      finishReplace(state.count, match.start)
    }
  }

  /**
   * 置換をやめます. isearch なら, カーソルを探す前の位置に戻します.
   *
   * @param current - ミニバッファの状態.
   */
  const cancel = (current: MinibufferState) => {
    setReplace(null)
    if (current.mode === 'isearch') {
      editor.current?.setCursor(current.originalCursor)
    }
  }

  const commands: CommandSet = {
    actions: {
      'isearch-forward': () => handleIsearch('forward', false),
      'isearch-backward': () => handleIsearch('backward', false),
      'isearch-forward-regexp': () => handleIsearch('forward', true),
      'isearch-backward-regexp': () => handleIsearch('backward', true),
      'query-replace': () => handleReplace({ query: true, regexp: false }),
      'query-replace-regexp': () => handleReplace({ query: true, regexp: true }),
      'replace-string': () => handleReplace({ query: false, regexp: false }),
      'replace-regexp': () => handleReplace({ query: false, regexp: true }),
    },
    submit: {
      'replace-from': askReplaceTo,
      'replace-to': startReplace,
    },
  }

  return {
    commands,
    handleIsearchRepeat,
    handleIsearchCommand,
    searchIsearch,
    finishIsearch,
    answerQuery,
    cancel,
  }
}
