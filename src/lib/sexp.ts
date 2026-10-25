/**
 * S 式, リスト, defun の移動 (Emacs 30 の lisp.el. `C-M-f`, `C-M-u`, `C-M-d`, `C-M-a`, `C-M-e`, `C-M-h`).
 * エディタ (CodeMirror) に依存しないので, 単体でテストできます.
 *
 * defun は, 行の始め (0 桁) の開く括弧から始まる式です (Emacs の `open-paren-in-column-0-is-defun-start`).
 */

import {
  type ScanError,
  type Source,
  UserError,
  backwardPrefixChars,
  forwardComment,
  isCommentStart,
  isOpenParen,
  isWhitespace,
  parseState,
  scanLists,
} from './syntax'
import { lineStartAt as lineStart } from './text'

/**
 * 行の終わりを返します.
 *
 * @param text - 本文.
 * @param pos - 位置.
 * @returns 行の終わり (改行の前).
 */
function lineEnd(text: string, pos: number): number {
  const i = text.indexOf('\n', pos)
  return i < 0 ? text.length : i
}

/**
 * 次の行の始めを返します (Emacs の `(forward-line 1)`).
 *
 * @param text - 本文.
 * @param pos - 位置.
 * @returns 次の行の始め. 最後の行なら, 本文の終わり.
 */
function nextLine(text: string, pos: number): number {
  const i = text.indexOf('\n', pos)
  return i < 0 ? text.length : i + 1
}

/**
 * S 式を越えます (Emacs の `forward-sexp`. 前へ越えたときは, 式の前の印 `'` なども越える).
 *
 * @param source - 本文.
 * @param pos - 位置.
 * @param count - 越える数 (負なら, 前へ).
 * @returns 越えた後の位置. 式が無ければ, 本文の端.
 * @throws {ScanError} 括弧が合わないとき.
 */
export function forwardSexp(source: Source, pos: number, count: number): number {
  const end = scanLists(source, pos, count, 0, true)
  if (count > 0) return end ?? source.max ?? source.text.length
  return backwardPrefixChars(source, end ?? source.min ?? 0)
}

/**
 * 外のリストへ出ます (Emacs の, キーから動かしたときの `up-list` と `backward-up-list`).
 * 文字列の中では, 文字列の中のリストだけを見て, 無ければ文字列の外へ出ます.
 *
 * @param source - 本文.
 * @param pos - 位置.
 * @param count - 出る数 (正なら後ろへ, 負なら前へ).
 * @returns 出た後の位置.
 * @throws {UserError} 外にリストが無いとき (`At top level`).
 */
export function upList(source: Source, pos: number, count: number): number {
  const { text } = source
  const inc = count > 0 ? 1 : -1
  let arg = count
  while (arg !== 0) {
    const state = parseState(source, pos)
    try {
      // 文字列かコメントの中なら, その中だけを見る
      let inner = source
      if (state.start !== null) {
        let end: number
        if (state.inString) {
          try {
            end = forwardSexp(source, state.start, 1)
          } catch {
            end = text.length
          }
        } else {
          end = forwardComment(source, state.start, 1)
        }
        inner = { ...source, min: state.start, max: end }
      }
      pos = scanLists(inner, pos, inc, 1, false) as number
    } catch {
      const start = state.start as number
      // コメントの後ろ (閉じていなければ, まだコメントの中)
      const after = state.inComment && inc > 0 ? forwardComment(source, start, 1) : start
      if (state.inString) {
        // 文字列の外へ (後ろへは, 文字列が閉じていなければ, 出られない)
        try {
          pos = inc > 0 ? forwardSexp(source, start, 1) : start
        } catch (error) {
          throw new UserError((error as ScanError).display)
        }
      } else if (state.inComment && !parseState(source, after).inComment) {
        // コメントの外から, もう一度
        pos = after
        arg += inc
      } else {
        throw new UserError('At top level')
      }
    }
    arg -= inc
  }
  return pos
}

/**
 * 中のリストへ入ります (Emacs の `down-list`. 後ろの開く括弧の後ろへ).
 *
 * @param source - 本文.
 * @param pos - 位置.
 * @returns 入った後の位置.
 * @throws {UserError} 文字列かコメントの中のときと, 入るリストが無いとき.
 */
export function downList(source: Source, pos: number): number {
  if (parseState(source, pos).start !== null) {
    throw new UserError('This command doesn’t work in strings or comments')
  }
  try {
    return scanLists(source, pos, 1, -1, false) as number
  } catch {
    throw new UserError('At bottom level')
  }
}

/**
 * defun の始め (行の始めの開く括弧) を探します (Emacs の `beginning-of-defun-raw`).
 * 文字列とコメントの中の括弧は除きます.
 *
 * @param source - 本文.
 * @param pos - 位置.
 * @param direction - 向き (`1` なら前へ, `-1` なら後ろへ).
 * @returns 見つけた括弧の位置と, 見つけたか. 見つからなければ, 本文の端.
 */
function beginningOfDefunRaw(
  source: Source,
  pos: number,
  direction: 1 | -1,
): { pos: number; found: boolean } {
  const { text } = source
  /** 行の始めの開く括弧なら `true`. */
  const start = (at: number) => lineStart(text, at) === at && isOpenParen(source, at)
  let p = direction < 0 && pos < text.length ? pos + 1 : pos
  for (;;) {
    let found = -1
    if (direction > 0) {
      for (let at = p - 1; at >= 0 && found < 0; at--) if (start(at)) found = at
      if (found < 0) return { pos: 0, found: false }
      p = found
    } else {
      for (let at = p; at < text.length && found < 0; at++) if (start(at)) found = at
      if (found < 0) return { pos: text.length, found: false }
      p = found + 1
    }
    // 文字列とコメントの中なら, その先から, もう一度
    if (parseState(source, p).start === null) return { pos: found, found: true }
  }
}

/**
 * defun の始めへ動きます (Emacs の `beginning-of-defun`).
 *
 * @param source - 本文.
 * @param pos - 位置.
 * @returns 動いた後の位置. defun が無ければ, 本文の始め.
 */
export function beginningOfDefun(source: Source, pos: number): number {
  return beginningOfDefunRaw(source, pos, 1).pos
}

/**
 * defun の終わりへ動きます (Emacs の `end-of-defun`). 終わりの後ろが, 空白かコメントだけなら, 次の行の始めへ.
 *
 * @param source - 本文.
 * @param pos - 位置.
 * @returns 動いた後の位置.
 * @throws {ScanError} 括弧が合わないとき.
 */
export function endOfDefun(source: Source, pos: number): number {
  const { text } = source
  /** defun の後ろの空白と, 改行かコメントまでを飛ばします. */
  const skip = (p: number) => {
    if (lineStart(text, p) === p) return p
    while (p < text.length && (text[p] === ' ' || text[p] === '\t')) p++
    return isCommentStart(source, p) || text[p] === '\n' ? nextLine(text, p) : p
  }
  const beg = beginningOfDefunRaw(source, lineEnd(text, pos), 1).pos
  let p = skip(forwardSexp(source, beg, 1))
  // defun の間から始めたら, 次の defun の終わりへ
  if (p <= pos) {
    const raw = beginningOfDefunRaw(source, pos, -1)
    p = raw.found ? forwardSexp(source, raw.pos, 1) : raw.pos
  }
  return skip(p)
}

/**
 * 行が, 空白とコメントだけなら `true` (Emacs の `beginning-of-defun--in-emptyish-line-p`).
 *
 * @param source - 本文.
 * @param pos - 行の中の位置.
 * @returns 空白とコメントだけなら `true`.
 */
function emptyishLine(source: Source, pos: number): boolean {
  const start = lineStart(source.text, pos)
  const state = parseState(source, start)
  if (state.inString) return false
  const from = state.inComment ? (state.start as number) : start
  return lineEnd(source.text, pos) < forwardComment(source, from, Infinity)
}

/**
 * defun の始めへ動き, 前に続くコメントの行の始めへ動きます (Emacs の `beginning-of-defun-comments`).
 *
 * @param source - 本文.
 * @param pos - 位置.
 * @returns 動いた後の位置.
 */
function beginningOfDefunComments(source: Source, pos: number): number {
  const { text } = source
  let p = beginningOfDefun(source, pos)
  let firstLine: boolean
  for (;;) {
    // 前の行へ (Emacs の `(forward-line -1)`)
    const start = lineStart(text, p)
    firstLine = start === 0
    p = firstLine ? 0 : lineStart(text, start - 1)
    // 行の終わりが, 前の行から続くコメントの中なら, コメントの始めの行へ
    p = lineEnd(text, p)
    let state = parseState(source, p)
    while (state.inComment && (state.start as number) < lineStart(text, p)) {
      p = lineEnd(text, state.start as number)
      state = parseState(source, p)
    }
    if (firstLine) break
    while (p > lineStart(text, p) && isWhitespace(source, p - 1)) p--
    // 空の行か, コメントでない文字がある行なら, 止める
    if (p === lineStart(text, p) || !emptyishLine(source, p)) break
  }
  return firstLine ? lineStart(text, p) : nextLine(text, p)
}

/**
 * defun を選ぶ範囲を返します (Emacs の `mark-defun`). 前のコメントと, 前の空の行も含めます.
 * リージョンがあれば, マークを次の defun の終わりへ延ばします.
 *
 * @param source - 本文.
 * @param pos - カーソル.
 * @param mark - リージョンのマーク (リージョンが無ければ `null`).
 * @returns カーソルとマーク.
 * @throws {ScanError} 括弧が合わないとき.
 */
export function markDefun(
  source: Source,
  pos: number,
  mark: number | null,
): { head: number; mark: number } {
  const { text } = source
  let head: number
  if (mark !== null && mark !== pos) {
    mark = endOfDefun(source, mark)
    head = pos
  } else {
    let beg = beginningOfDefunComments(source, pos)
    let end = endOfDefun(source, beg)
    if (end <= pos || beg === 0) {
      // 前の defun を見つけたので, 後ろの defun にする
      end = endOfDefun(source, pos)
      beg = beginningOfDefunComments(source, end)
    }
    mark = endOfDefun(source, beg)
    head = beg
  }
  // 前の空の行も含める
  while (head > 0 && (text[head - 1] === '\n' || isWhitespace(source, head - 1))) head--
  if (head > 0) head = nextLine(text, head)
  return { head, mark }
}
