/**
 * 段落と文の移動 (Emacs 30 の paragraphs.el. `M-{`, `M-}`, `M-a`, `M-e`, `M-k`).
 * エディタ (CodeMirror) に依存しないので, 単体でテストできます.
 *
 * 段落を分ける行は, モードの `paragraph-start` と `paragraph-separate` で決めます. Fundamental モードと
 * emacs-lisp-mode では, 空白だけの行で, scheme-mode では, 空の行だけです (空白だけの行は, 段落を分けない).
 */

import { type Dialect, UserError } from './syntax'
import { lineStartAt } from './text'

/** 行の終わり (Emacs の正規表現の `$`). */
const EOL = '(?=\\n|$)'
/** 行の始め (Emacs の正規表現の `^`). */
const BOL = '(?<![^\\n])'

/** 段落の正規表現. */
interface ParagraphRegexps {
  /** 段落の始めか, 段落を分ける行 (Emacs の `paragraph-start`). */
  start: RegExp
  /** 段落を分ける行 (Emacs の `paragraph-separate`). */
  separate: RegExp
  /** 段落の始めか, 段落を分ける行を探す (Emacs の `forward-paragraph` の `sp-parstart`). */
  search: RegExp
}

/**
 * モードの段落の正規表現を返します.
 *
 * @param dialect - Lisp の方言.
 * @returns 正規表現.
 */
function paragraphRegexps(dialect: Dialect): ParagraphRegexps {
  const start = dialect === 'scheme' ? `${EOL}|${BOL}\\f` : `\\f|[ \\t]*${EOL}`
  const separate = dialect === 'scheme' ? start : `[ \\t\\f]*${EOL}`
  return {
    start: new RegExp(start, 'y'),
    separate: new RegExp(separate, 'y'),
    search: new RegExp(`${BOL}[ \\t]*(?:${start}|${separate})`, 'g'),
  }
}

/** 文の終わり (Emacs の `(sentence-end)`. `sentence-end-double-space` が `t`). */
const SENTENCE_END = new RegExp(
  `(?:[.?!…‽][\\]"'”’)}»›]*(?:${EOL}|[ \\u00a0]${EOL}|\\t|[ \\u00a0][ \\u00a0])|[。．？！]+)[ \\u00a0\\t\\n]*`,
  'gu',
)

/**
 * 位置で, 正規表現が合えば `true` (Emacs の `looking-at`).
 *
 * @param re - 正規表現 (`y` の).
 * @param text - 本文.
 * @param pos - 位置.
 * @returns 合えば `true`.
 */
function lookingAt(re: RegExp, text: string, pos: number): boolean {
  re.lastIndex = pos
  return re.test(text)
}

/**
 * 本文を, 位置で切ります. 正規表現の `$` は, 位置が本当の行の終わりのときだけ合います (Emacs の検索の範囲).
 *
 * @param text - 本文.
 * @param stop - 位置.
 * @returns 切った本文.
 */
function bounded(text: string, stop: number): string {
  // 行の終わりでなければ, どの正規表現にも合わない文字を足して, `$` を合わなくする
  return stop >= text.length || text[stop] === '\n'
    ? text.slice(0, stop)
    : text.slice(0, stop) + '\u0000'
}

/**
 * 位置から前へ, 正規表現が合う所を探します (Emacs の `re-search-backward`. 合う文字は, 位置の前で終わる).
 *
 * @param re - 正規表現 (`g` の).
 * @param text - 本文.
 * @param pos - 位置.
 * @param bound - 合う所の始めの限り.
 * @returns 合った所. 無ければ `null`.
 */
function searchBackward(
  re: RegExp,
  text: string,
  pos: number,
  bound: number,
): { start: number; end: number } | null {
  const subject = bounded(text, pos)
  const sticky = new RegExp(re.source, re.flags.replace('g', '') + 'y')
  for (let start = pos; start >= bound; start--) {
    sticky.lastIndex = start
    const m = sticky.exec(subject)
    if (m && start + m[0].length <= pos) return { start, end: start + m[0].length }
  }
  return null
}

/**
 * 段落を越えます (Emacs の `forward-paragraph`. `fill-prefix` は無い).
 *
 * 後ろへは, 段落の後ろの, 段落を分ける行の始めへ, 前へは, 段落の前の, 段落を分ける行の始めへ動きます.
 *
 * @param text - 本文.
 * @param pos - 位置.
 * @param count - 越える数 (負なら, 前へ).
 * @param dialect - Lisp の方言.
 * @returns 越えた後の位置.
 */
export function forwardParagraph(
  text: string,
  pos: number,
  count: number,
  dialect: Dialect,
): number {
  const { start, separate, search } = paragraphRegexps(dialect)
  const max = text.length
  const bol = (p: number) => lineStartAt(text, p)
  const nextLine = (p: number) => {
    const i = text.indexOf('\n', p)
    return i < 0 ? max : i + 1
  }
  let arg = count
  let p = pos
  while (arg < 0 && p > 0) {
    // 前が空の行なら, そこへ
    if (
      !lookingAt(separate, text, p) &&
      text[p - 1] === '\n' &&
      bol(p - 1) === p - 1 &&
      lookingAt(separate, text, p - 1)
    ) {
      p--
      arg++
      continue
    }
    // 前の, 段落を分ける行を越える
    p = bol(p - 1)
    while (p > 0 && lookingAt(separate, text, p)) p = bol(p - 1)
    if (p === 0) break
    arg++
    p = text.indexOf('\n', p) < 0 ? max : text.indexOf('\n', p)
    // 前の, 段落の始めか, 段落を分ける行を探す
    let found = false
    for (;;) {
      const m = searchBackward(search, text, p, 0)
      if (!m) {
        p = 0
        break
      }
      p = m.start
      if (lookingAt(separate, text, p) || lookingAt(start, text, p)) {
        found = true
        break
      }
    }
    if (!found) {
      p = 0
      continue
    }
    // 段落を分ける行を越える
    while (p < max && lookingAt(separate, text, p)) p = nextLine(p)
    // 前の行が空なら, その行へ. 空でなければ, 段落の始めへ
    const lineStart = bol(p)
    const previousEnd = lineStart === 0 ? 0 : lineStart - 1
    p = previousEnd > bol(previousEnd) ? previousEnd + 1 : previousEnd
  }
  while (arg > 0 && p < max) {
    // 段落を分ける行を越える
    p = bol(p)
    while (p < max && lookingAt(separate, text, p)) p = nextLine(p)
    if (p < max) arg--
    p = nextLine(p)
    // 次の, 段落の始めか, 段落を分ける行を探す
    let found = p
    for (;;) {
      search.lastIndex = p
      const m = search.exec(text)
      if (!m) {
        p = max
        break
      }
      found = p = m.index
      if (p >= max || lookingAt(separate, text, p) || lookingAt(start, text, p)) break
      p++
    }
    if (p < max) p = found
  }
  return p
}

/**
 * 段落の本文の始めへ動きます (Emacs の `start-of-paragraph-text`).
 *
 * @param text - 本文.
 * @param pos - 位置.
 * @param dialect - Lisp の方言.
 * @returns 段落の, 始めの空白でない文字.
 */
function startOfParagraphText(text: string, pos: number, dialect: Dialect): number {
  const start = forwardParagraph(text, pos, -1, dialect)
  let p = start
  while (p < text.length && ' \t\n'.includes(text[p])) p++
  // 越えた空の行が, 始めの位置を含むなら, その前から
  if (p >= pos && start < pos) return start > 0 ? startOfParagraphText(text, start, dialect) : start
  return p
}

/**
 * 段落の本文の終わりへ動きます (Emacs の `end-of-paragraph-text`).
 *
 * @param text - 本文.
 * @param pos - 位置.
 * @param dialect - Lisp の方言.
 * @returns 段落の, 最後の行の終わり.
 * @throws {UserError} 本文の終わりから動こうとしたとき (`End of buffer`).
 */
function endOfParagraphText(text: string, pos: number, dialect: Dialect): number {
  let p = forwardParagraph(text, pos, 1, dialect)
  if (text[p - 1] === '\n') p--
  if (p > pos) return p
  if (p >= text.length) throw new UserError('End of buffer')
  p++
  return p < text.length ? endOfParagraphText(text, p, dialect) : p
}

/**
 * 文を越えます (Emacs の `forward-sentence`). 段落の終わりも, 文の終わりです.
 *
 * 後ろへは, 文の終わり (後ろの空白の前) へ, 前へは, 文の始めへ動きます.
 *
 * @param text - 本文.
 * @param pos - 位置.
 * @param count - 越える数 (負なら, 前へ).
 * @param dialect - Lisp の方言.
 * @returns 越えた後の位置.
 * @throws {UserError} 本文の終わりから後ろへ動こうとしたとき (`End of buffer`).
 */
export function forwardSentence(
  text: string,
  pos: number,
  count: number,
  dialect: Dialect,
): number {
  let p = pos
  for (let arg = count; arg < 0; arg++) {
    const textStart = startOfParagraphText(text, p, dialect)
    const lineStart = lineStartAt(text, textStart)
    let m = searchBackward(SENTENCE_END, text, p, lineStart)
    // 前の文の終わりが, 位置で終われば (位置が, 文の始め), もう 1 つ前
    if (m && m.end >= p) m = searchBackward(SENTENCE_END, text, m.start, lineStart)
    p = m ? m.end : textStart
  }
  for (let arg = count; arg > 0; arg--) {
    const end = endOfParagraphText(text, p, dialect)
    SENTENCE_END.lastIndex = p
    const m = SENTENCE_END.exec(bounded(text, end))
    if (m) {
      p = m.index + m[0].length
      while (p > 0 && ' \t\n'.includes(text[p - 1])) p--
    } else {
      p = end
    }
  }
  return p
}
