/**
 * 行, 文, 単語, 文字を数える (Emacs 30 の `count-words` と `count-words-region`. `M-=`).
 * エディタ (CodeMirror) に依存しないので, 単体でテストできます.
 */

/** 単語の文字 (文字, 数字, 結合文字, 絵文字). */
const WORD_CHAR = /[\p{L}\p{N}\p{M}\p{Emoji_Presentation}]/u

/** 文の終わり (Emacs の `sentence-end`. `sentence-end-double-space` が `t`). */
const SENTENCE_END =
  /([.?!…‽][\]"'”’)}»›]*($|[ \u00a0]$|\t|[ \u00a0][ \u00a0])|[。．？！]+)[ \u00a0\t\n]*/gmu

/** 空白だけの行 (Emacs の `paragraph-separate`). */
const BLANK = /^[ \t\f]*$/

/**
 * 単語の文字の, 文字の種類 (Emacs の `char-script-table`) を返します.
 *
 * @param ch - 文字.
 * @returns 種類.
 */
function script(ch: string): string {
  if (/\p{M}/u.test(ch)) return 'mark'
  if (/\p{Script=Han}/u.test(ch)) return 'han'
  if (/\p{Script=Hiragana}/u.test(ch)) return 'hiragana'
  if (/[\p{Script=Katakana}ー]/u.test(ch)) return 'katakana'
  if (/\p{Script=Hangul}/u.test(ch)) return 'hangul'
  if (/\p{Script=Greek}/u.test(ch)) return 'greek'
  if (/\p{Script=Cyrillic}/u.test(ch)) return 'cyrillic'
  if (/\p{Emoji_Presentation}/u.test(ch)) return 'emoji'
  return 'latin'
}

/**
 * 隣り合う単語の文字の間で, 単語が分かれるなら `true` (Emacs の `word-combining-categories` と
 * `word-separating-categories`). 漢字の後のかな (送りがな) と, カタカナの後のひらがなは, 分けません.
 *
 * @param before - 前の文字.
 * @param after - 後の文字.
 * @returns 分かれるなら `true`.
 */
function separates(before: string, after: string): boolean {
  const a = script(before)
  const b = script(after)
  if (a === 'mark' || b === 'mark') return false
  const kana = (s: string) => s === 'hiragana' || s === 'katakana'
  if (kana(a) && kana(b)) return a === 'hiragana' && b === 'katakana'
  if (a === 'han' && kana(b)) return false
  return a !== b
}

/**
 * 単語の数を返します (Emacs の `forward-word` で動ける回数).
 *
 * @param text - 文字列.
 * @returns 単語の数.
 */
export function countWords(text: string): number {
  let words = 0
  let previous: string | null = null
  for (const ch of text) {
    const word = WORD_CHAR.test(ch)
    if (word && (previous === null || separates(previous, ch))) words++
    previous = word ? ch : null
  }
  return words
}

/**
 * 行の数を返します (Emacs の `count-lines`. 改行で終わらなければ, 最後の行も数える).
 *
 * @param text - 文字列.
 * @returns 行の数.
 */
export function countLines(text: string): number {
  const newlines = text.split('\n').length - 1
  return text === '' || text.endsWith('\n') ? newlines : newlines + 1
}

/**
 * 段落の本文の終わりを返します (Emacs の `end-of-paragraph-text`. 空白だけの行で, 段落を分ける).
 *
 * @param text - 文字列.
 * @param pos - 位置.
 * @returns 段落の, 最後の行の終わり.
 */
function endOfParagraphText(text: string, pos: number): number {
  const lines = text.split('\n')
  // 位置の行 (行の始めの位置と, 番号)
  let n = text.slice(0, pos).split('\n').length - 1
  let start = text.lastIndexOf('\n', pos - 1) + 1
  // forward-paragraph: 空の行を飛ばし, 次の空の行の始め (無ければ, 文字列の終わり) へ
  while (n < lines.length && BLANK.test(lines[n])) start += lines[n++].length + 1
  while (n < lines.length && !BLANK.test(lines[n])) start += lines[n++].length + 1
  let end = Math.min(start, text.length)
  if (text[end - 1] === '\n') end--
  if (end > pos) return end
  return pos + 1 < text.length ? endOfParagraphText(text, pos + 1) : pos + 1
}

/**
 * 次の文の終わりを返します (Emacs の `forward-sentence`. 段落の終わりも, 文の終わり).
 *
 * @param text - 文字列.
 * @param pos - 位置.
 * @returns 文の終わり (後ろの空白の前).
 */
function forwardSentence(text: string, pos: number): number {
  const end = endOfParagraphText(text, pos)
  const pattern = new RegExp(SENTENCE_END)
  pattern.lastIndex = pos
  const match = pattern.exec(text.slice(0, end))
  if (!match) return end
  let next = match.index + match[0].length
  while (next > pos && /[ \t\n]/.test(text[next - 1])) next--
  return next
}

/**
 * 文の数を返します (Emacs の `count-sentences`).
 *
 * @param text - 文字列.
 * @returns 文の数.
 */
export function countSentences(text: string): number {
  let sentences = 0
  let pos = 0
  while (pos < text.length) {
    // 段落の終わりも文の終わりなので, 必ず先へ進む
    pos = forwardSentence(text, pos)
    sentences++
  }
  // 最後の, 空白だけの文は除く
  return /[ \t\n]$/.test(text) ? sentences - 1 : sentences
}

/**
 * 数えた結果を, Emacs と同じ文にします (`Buffer has 2 lines, 1 sentence, 3 words, and 5 characters`).
 *
 * @param label - 数えた範囲 (`Buffer` か `Region`).
 * @param text - 文字列.
 * @returns 文.
 */
export function countMessage(label: string, text: string): string {
  const count = (n: number, unit: string) => `${n} ${unit}${n === 1 ? '' : 's'}`
  return (
    `${label} has ${count(countLines(text), 'line')}, ${count(countSentences(text), 'sentence')}, ` +
    `${count(countWords(text), 'word')}, and ${count([...text].length, 'character')}`
  )
}
