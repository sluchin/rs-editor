/**
 * 段落を詰める (Emacs 30 の `fill-paragraph`. `M-q`). 行を `fill-column` (70 桁) までに詰め直します.
 *
 * - Lisp では, カーソルの行のコメントの段落を詰めます (コードと文字列は, 変えません).
 *   続きの行には, コメントの `;` の並びを付けます.
 * - Lisp でなければ, 空の行で区切った段落を詰めます (Emacs の text-mode).
 *   続きの行の始め (fill-prefix) は, 2 行目の始めの空白や `>` などから取ります (`adaptive-fill-mode`).
 * - 語の間の空白は 1 つにし, 文の終わり (`.` `?` `!`) の後に 2 つあった空白は, 2 つのままにします
 *   (`sentence-end-double-space`). 行をつないだ所が, 文の終わりなら, 空白 2 つにします.
 * - 日本語 (漢字, かな) は, 空白の無い所でも折り, 行をつなぐときは空白を入れません.
 *   行の始めに置けない文字 (`。` `」` など) と, 行の終わりに置けない文字 (`「` など) は, 禁則で避けます.
 */

import { EditorSelection, type EditorState, type TransactionSpec } from '@codemirror/state'
import { commentStart } from './comment'
import { charWidth, columnAt, indentConfig, whitespace } from './lispIndent'

/** 行の長さの上限の桁 (Emacs の `fill-column`). */
export const FILL_COLUMN = 70

/** 禁則で, 行を越えてよい桁の数 (Emacs の `kinsoku-limit`). */
const KINSOKU_LIMIT = 4

/** 行の始めの, fill-prefix になる文字 (Emacs の `adaptive-fill-regexp`). */
const ADAPTIVE_PREFIX = /^[-–!|#%;>*·•‣⁃◦ \t]*/

/** Lisp のコメントの始め (字下げと, `;` の並びと, 後ろの空白). */
const COMMENT_PREFIX = /^([ \t]*)(;+)([ \t]*)/

/** 空白だけの行. */
const BLANK = /^[ \t]*$/

/** 日本語と中国語の文字 (空白の無い所でも折り, 行をつなぐときは空白を入れない). */
const CJK = /[\p{Script=Han}\p{Script=Hiragana}\p{Script=Katakana}\u3000-\u303f\uff00-\uffef]/u

/** 行の始めに置けない文字. */
const NO_LINE_START = new Set(
  '、。，．・：；？！゛゜ヽヾゝゞ々ー’”）〕］｝〉》」』】ぁぃぅぇぉっゃゅょゎァィゥェォッャュョヮヵヶ',
)

/** 行の終わりに置けない文字. */
const NO_LINE_END = new Set('‘“（〔［｛〈《「『【')

/** 文の終わり (`.` `?` `!` と, 後ろの閉じ括弧と引用符). */
const SENTENCE_END = /[.?!][\])}"'”’]*$/

/** 詰める段落. */
interface Paragraph {
  /** 始めの行 (1 から). */
  first: number
  /** 終わりの行. */
  last: number
  /** 始めの行の, 本文の前 (そのまま残す). */
  head: string
  /** 続きの行の始めに付けるもの. */
  prefix: string
  /** 行ごとの, 本文の始めの位置 (行の始めから). */
  starts: number[]
}

/**
 * 文字が日本語か中国語なら `true`.
 *
 * @param ch - 文字 (無ければ `undefined`).
 * @returns 日本語か中国語なら `true`.
 */
function isCjk(ch: string | undefined): boolean {
  return ch !== undefined && CJK.test(ch)
}

/**
 * 1 行だけの段落の, 続きの行の始めを返します. 空白だけならそのまま, ほかは同じ幅の空白にします.
 *
 * @param head - 始めの行の, 本文の前.
 * @param tabWidth - タブの幅.
 * @returns 続きの行の始め.
 */
function prefixFromHead(head: string, tabWidth: number): string {
  return BLANK.test(head) ? head : ' '.repeat(columnAt(head, head.length, tabWidth))
}

/**
 * Lisp のコメントの段落を探します.
 *
 * @param state - エディタの状態.
 * @param lineNumber - カーソルの行.
 * @returns 段落. コメントでなければ `null`.
 */
function commentParagraph(state: EditorState, lineNumber: number): Paragraph | null {
  const { doc } = state
  const config = state.facet(indentConfig)
  /** 行のコメント (文字列の中の `;` を除く). 本文が無ければ `null`. */
  const commentAt = (n: number) => {
    if (n < 1 || n > doc.lines) return null
    const text = doc.line(n).text
    const start = commentStart(state, n)
    if (start === null) return null
    const match = COMMENT_PREFIX.exec(text.slice(start)) as RegExpExecArray
    const body = start + match[0].length
    if (body === text.length) return null
    return {
      body,
      col: columnAt(text, start, config.tabWidth),
      semicolons: match[2],
      spaces: match[3],
      code: !BLANK.test(text.slice(0, start)),
    }
  }
  const current = commentAt(lineNumber)
  if (!current) return null
  // 同じ桁で, 同じ数の `;` のコメントなら, 同じ段落 (コードの後ろのコメントは, 段落の始めの行だけ)
  const same = (comment: ReturnType<typeof commentAt>, code: boolean) =>
    comment !== null &&
    comment.code === code &&
    comment.col === current.col &&
    comment.semicolons === current.semicolons
  let first = lineNumber
  if (!current.code) {
    while (same(commentAt(first - 1), false)) first--
    if (same(commentAt(first - 1), true)) first--
  }
  let last = lineNumber
  while (same(commentAt(last + 1), false)) last++
  const starts = []
  for (let n = first; n <= last; n++) starts.push((commentAt(n) as { body: number }).body)
  const head = doc.line(first).text.slice(0, starts[0])
  // 続きの行の始めは, 2 行目から取る. 1 行だけなら, コメントの桁まで空白で埋める
  const prefix =
    last > first
      ? doc.line(first + 1).text.slice(0, starts[1])
      : whitespace(0, current.col, config) + current.semicolons + current.spaces
  return { first, last, head, prefix, starts }
}

/**
 * 空の行で区切った段落を探します (Lisp でないとき).
 *
 * @param state - エディタの状態.
 * @param lineNumber - カーソルの行. 空の行なら, 次の段落.
 * @returns 段落. 無ければ `null`.
 */
function textParagraph(state: EditorState, lineNumber: number): Paragraph | null {
  const { doc } = state
  const { tabWidth } = state.facet(indentConfig)
  const blank = (n: number) => BLANK.test(doc.line(n).text)
  let first = lineNumber
  while (first <= doc.lines && blank(first)) first++
  if (first > doc.lines) return null
  while (first > 1 && !blank(first - 1)) first--
  let last = first
  while (last < doc.lines && !blank(last + 1)) last++
  const head = (ADAPTIVE_PREFIX.exec(doc.line(first).text) as RegExpExecArray)[0]
  const prefix =
    last > first
      ? (ADAPTIVE_PREFIX.exec(doc.line(first + 1).text) as RegExpExecArray)[0]
      : prefixFromHead(head, tabWidth)
  const starts = [head.length]
  for (let n = first + 1; n <= last; n++) {
    const text = doc.line(n).text
    // fill-prefix があれば除き, 残りの始めの空白も除く
    const rest = text.startsWith(prefix) ? prefix.length : 0
    starts.push(rest + (/^[ \t]*/.exec(text.slice(rest)) as RegExpExecArray)[0].length)
  }
  return { first, last, head, prefix, starts }
}

/** 段落の本文の 1 文字. */
interface Char {
  /** 文字. */
  ch: string
  /** 元の位置 (文書の始めから). */
  pos: number
}

/** 本文の, 語の間の空白. */
interface Gap {
  /** 空白の後の文字の番号 (本文の中). */
  index: number
  /** 空白 (行の終わりで折ったなら, 使わない). 空なら, 空白の無い所で折れる. */
  text: string
}

/**
 * 段落の本文を, 文字と, 語の間の空白に分けます. 空白は, 1 つ (文の終わりの後は 2 つ) にします.
 *
 * @param state - エディタの状態.
 * @param paragraph - 段落.
 * @returns 本文の文字と, 折れる所.
 */
function splitWords(state: EditorState, paragraph: Paragraph) {
  const chars: Char[] = []
  // 次の文字の前の空白 (`null` は, 空白でない所)
  const spaces: (string | null)[] = []
  let pending: string | null = null
  for (let n = paragraph.first; n <= paragraph.last; n++) {
    const line = state.doc.line(n)
    const body = line.text.slice(paragraph.starts[n - paragraph.first]).replace(/[ \t]+$/, '')
    if (n > paragraph.first) pending = '\n'
    let i = 0
    for (const ch of body) {
      const pos = line.from + paragraph.starts[n - paragraph.first] + i
      i += ch.length
      if (ch === ' ' || ch === '\t') {
        pending = (pending ?? '') + ch
        continue
      }
      spaces.push(pending)
      chars.push({ ch, pos })
      pending = null
    }
  }
  const gaps: Gap[] = []
  for (let index = 1; index < chars.length; index++) {
    const space = spaces[index]
    const before = chars[index - 1].ch
    const after = chars[index].ch
    if (space === null) {
      // 日本語の前後は, 空白が無くても折れる (禁則の文字の前後を除く)
      if (
        (isCjk(before) || isCjk(after)) &&
        !NO_LINE_END.has(before) &&
        !NO_LINE_START.has(after)
      ) {
        gaps.push({ index, text: '' })
      }
      continue
    }
    const text = chars
      .slice(Math.max(0, index - 4), index)
      .map((c) => c.ch)
      .join('')
    if (space === '\n' && (isCjk(before) || isCjk(after))) {
      gaps.push({ index, text: '' })
    } else if (SENTENCE_END.test(text) && (space.includes('\n') || space.length >= 2)) {
      gaps.push({ index, text: '  ' })
    } else {
      gaps.push({ index, text: ' ' })
    }
  }
  return { chars, gaps }
}

/** 行の終わりで折れない所を含む, 語. */
interface Word {
  /** 文字 (本文の中の番号) と, 語の中の空白. */
  parts: (number | string)[]
  /** 幅. */
  width: number
  /** 後ろの, 行の始めに置けない文字の幅 (禁則で, 行を越えて置ける). */
  tail: number
}

/**
 * 本文を, 折れる所で, 語に分けます. 文の終わりでない `.` の後の空白 1 つでは, 折りません (`Mr. Smith`).
 *
 * @param chars - 本文の文字.
 * @param gaps - 語の間の空白.
 * @returns 語と, 語の前の空白 (始めの語は空).
 */
function toWords(chars: Char[], gaps: Gap[]) {
  const words: Word[] = []
  const spaces: string[] = []
  let word: Word = { parts: [], width: 0, tail: 0 }
  let g = 0
  for (let index = 0; index < chars.length; index++) {
    if (g < gaps.length && gaps[g].index === index) {
      const { text } = gaps[g++]
      if (text === ' ' && chars[index - 1].ch === '.') {
        word.parts.push(text)
        word.width += text.length
        word.tail = 0
      } else {
        words.push(word)
        spaces.push(text)
        word = { parts: [], width: 0, tail: 0 }
      }
    }
    const width = charWidth(chars[index].ch.codePointAt(0) as number)
    word.parts.push(index)
    word.width += width
    word.tail = NO_LINE_START.has(chars[index].ch) ? word.tail + width : 0
  }
  words.push(word)
  return { words, spaces: ['', ...spaces] }
}

/**
 * 段落を詰めて, 行に分けます.
 *
 * @param chars - 本文の文字.
 * @param gaps - 語の間の空白.
 * @param paragraph - 段落.
 * @param tabWidth - タブの幅.
 * @returns 新しい本文と, 本文の文字ごとの新しい位置 (段落の始めから).
 */
function breakLines(chars: Char[], gaps: Gap[], paragraph: Paragraph, tabWidth: number) {
  const { words, spaces } = toWords(chars, gaps)
  let out = paragraph.head
  const positions: number[] = []
  let col = columnAt(out, out.length, tabWidth)
  words.forEach((word, i) => {
    const space = spaces[i]
    const end = col + space.length + word.width
    // 行の始めに置けない文字は, 少しなら, 行を越えて置く (禁則)
    const fits =
      end <= FILL_COLUMN ||
      (word.tail > 0 && word.tail <= KINSOKU_LIMIT && end - word.tail <= FILL_COLUMN)
    if (i > 0 && !fits) {
      out += '\n' + paragraph.prefix
      col = columnAt(paragraph.prefix, paragraph.prefix.length, tabWidth)
    } else {
      out += space
      col += space.length
    }
    for (const part of word.parts) {
      if (typeof part === 'number') positions.push(out.length)
      const text = typeof part === 'number' ? chars[part].ch : part
      out += text
      col += typeof part === 'number' ? charWidth(text.codePointAt(0) as number) : text.length
    }
  })
  return { text: out, positions }
}

/**
 * カーソルの段落を詰めます (Emacs の `fill-paragraph`).
 *
 * @param state - エディタの状態.
 * @param pos - カーソルの位置.
 * @returns 変更とカーソル. 詰める段落が無ければ `null`.
 */
export function fillParagraph(state: EditorState, pos: number): TransactionSpec | null {
  const config = state.facet(indentConfig)
  const lineNumber = state.doc.lineAt(pos).number
  const paragraph = config.dialect
    ? commentParagraph(state, lineNumber)
    : textParagraph(state, lineNumber)
  if (!paragraph) return null
  const { chars, gaps } = splitWords(state, paragraph)
  const { text, positions } = breakLines(chars, gaps, paragraph, config.tabWidth)
  const from = state.doc.line(paragraph.first).from
  const to = state.doc.line(paragraph.last).to
  // カーソルは, 同じ文字の前に置く (段落の始めの行の, 本文の前なら, そのまま)
  const next = chars.findIndex((c) => c.pos >= pos)
  const cursor =
    pos < from + paragraph.head.length ? pos : from + (next < 0 ? text.length : positions[next])
  return {
    changes: { from, to, insert: text },
    selection: EditorSelection.cursor(cursor),
  }
}
