/**
 * 置換 (query-replace, replace-string と, その正規表現の版).
 *
 * 正規表現は JavaScript の書き方です. 置き換える文字では, Emacs と同じく `\&` (合った全体), `\1` から `\9` (グループ),
 * `\\` (`\` そのもの) を使えます.
 */

import { compileRegexp } from './isearch'

/** 置き換える所. */
export interface ReplaceMatch {
  /** 始まりの位置. */
  start: number
  /** 終わりの位置 (この位置の文字は含まない). */
  end: number
  /** 置き換える文字 (`\&` などを展開した後). */
  replacement: string
}

/** 置換の指定. */
export interface ReplaceSpec {
  /** 探す文字 (`regexp` なら正規表現). */
  from: string
  /** 置き換える文字. */
  to: string
  /** 正規表現で探すなら `true`. */
  regexp: boolean
}

/**
 * 置き換える文字の `\&`, `\1` などを, 合った文字にします.
 *
 * @param template - 置き換える文字.
 * @param groups - 合った文字 (0 が全体, 1 からグループ).
 * @returns 展開した文字.
 */
export function expandReplacement(
  template: string,
  groups: readonly (string | undefined)[],
): string {
  return template.replace(/\\(&|\d|\\)/g, (_, c: string) => {
    if (c === '\\') return '\\'
    return groups[c === '&' ? 0 : Number(c)] ?? ''
  })
}

/**
 * 位置から後ろで, 次に置き換える所を探します. 空の所 (正規表現) は, 飛ばします.
 *
 * @param text - 本文.
 * @param spec - 置換の指定.
 * @param pos - 探し始める位置.
 * @returns 置き換える所. 無ければ `null`.
 */
export function findReplaceMatch(
  text: string,
  spec: ReplaceSpec,
  pos: number,
): ReplaceMatch | null {
  if (spec.from === '') return null
  if (!spec.regexp) {
    const start = text.indexOf(spec.from, pos)
    return start === -1 ? null : { start, end: start + spec.from.length, replacement: spec.to }
  }
  const re = compileRegexp(spec.from)
  if (!re) return null
  re.lastIndex = pos
  for (let m = re.exec(text); m; m = re.exec(text)) {
    if (m[0] !== '') {
      return {
        start: m.index,
        end: m.index + m[0].length,
        replacement: expandReplacement(spec.to, m),
      }
    }
    re.lastIndex = m.index + 1
  }
  return null
}

/** 置き換える本文 (エディタ). */
export interface ReplaceTarget {
  /** 本文. */
  text: () => string
  /** 範囲を置き換えます. */
  replace: (from: number, to: number, text: string) => void
}

/**
 * 位置から後ろを, 全て置き換えます.
 *
 * @param target - 置き換える本文.
 * @param spec - 置換の指定.
 * @param pos - 置き換え始める位置.
 * @returns 置き換えた数と, 最後に置き換えた文字の後の位置 (置き換えなければ `pos`).
 */
export function replaceRemaining(
  target: ReplaceTarget,
  spec: ReplaceSpec,
  pos: number,
): { count: number; pos: number } {
  let count = 0
  for (
    let m = findReplaceMatch(target.text(), spec, pos);
    m;
    m = findReplaceMatch(target.text(), spec, pos)
  ) {
    target.replace(m.start, m.end, m.replacement)
    pos = m.start + m.replacement.length
    count++
  }
  return { count, pos }
}

/**
 * 置き換えた数のメッセージを作ります.
 *
 * @param count - 置き換えた数.
 * @returns メッセージ.
 */
export function replacedMessage(count: number): string {
  return `Replaced ${count} occurrence${count === 1 ? '' : 's'}`
}
