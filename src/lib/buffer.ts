/**
 * バッファ (ファイルを開いたもの, または *scratch* などの, ファイルのないもの) と, バッファを作る関数.
 */

import { Text } from '@codemirror/state'
import { diredBufferName } from './dired'
import type { GrepMatch } from './fileOps'
import { fileNewerThan, pathExists, readFile } from './fileOps'
import type { KeyBinding } from './lisp'
import { autoSaveFileName, directoryFileName } from './paths'

/** *scratch* バッファの始めの本文 (Emacs と同じ). */
const INITIAL_TEXT = `;; This buffer is for text that is not saved, and for Lisp evaluation.
;; To create a file, visit it with C-x C-f and enter text in its buffer.
`

/** バッファ (ファイルを開いたもの, または *scratch* などの, ファイルのないもの). */
export interface Buffer {
  /** バッファを区別する ID. */
  id: string
  /** バッファの名前 (ファイルの名前, または `*scratch*` など). */
  name: string
  /** バッファのファイルのパス. ファイルのないバッファは `null`. */
  filePath: string | null
  /**
   * 本文 (CodeMirror の `Text`. 変えられないので, エディタの本文を写さずに共有する).
   * 文字列が要るときだけ, `toString()` で作る.
   */
  doc: Text
  /** 保存してから, 書き換えたら `true`. */
  modified: boolean
  /** 開いてから, ファイルのバックアップ (`<file>~`) を作ったら `true` (Emacs の `buffer-backed-up`). */
  backedUp?: boolean
  /** dired のバッファなら, そのディレクトリ (後ろに `/` は付けない). */
  directory?: string
  /** バッファだけのキー (Lisp の `local-set-key`. Lisp の割り当てより先に使う). */
  keys?: KeyBinding[]
  /** `M-x grep` の結果のバッファなら, 探したディレクトリ (`/` で終わる) と, 合った行. */
  grep?: { directory: string; matches: GrepMatch[] }
}

/**
 * 文字列から, 本文 (`Text`) を作ります. 行の区切りは, CodeMirror と同じく `\r\n`, `\r`, `\n` です.
 *
 * @param content - 文字列.
 * @returns 本文.
 */
export function toText(content: string): Text {
  return Text.of(content.split(/\r\n?|\n/))
}

/**
 * 本文の始めの 2 行を返します (ファイルの変数 `-*- ... -*-` を読むため. 本文の全体は作らない).
 *
 * @param doc - 本文.
 * @returns 始めの 2 行 (`\n` でつなぐ).
 */
export function headLines(doc: Text): string {
  return doc.lines < 2 ? doc.line(1).text : `${doc.line(1).text}\n${doc.line(2).text}`
}

/**
 * バッファの ID を作ります.
 *
 * @returns ID.
 */
export function generateBufferId(): string {
  return `buffer-${Date.now()}-${Math.random().toString(36).slice(2, 11)}`
}

/**
 * ファイルのパスから, バッファの名前 (ファイルの名前) を作ります.
 *
 * @param path - ファイルのパス.
 * @returns バッファの名前. パスが `/` で終わる場合は, パスのまま.
 */
export function bufferNameFromPath(path: string): string {
  const parts = path.split('/')
  return parts[parts.length - 1] || path
}

/**
 * *scratch* バッファを作ります.
 *
 * @returns *scratch* バッファ.
 */
export function createScratchBuffer(): Buffer {
  return {
    id: generateBufferId(),
    name: '*scratch*',
    filePath: null,
    doc: toText(INITIAL_TEXT),
    modified: false,
  }
}

/**
 * ファイルを読んで, バッファを作ります (まだ出さない). 自動保存のファイルの方が新しければ, そう知らせます.
 *
 * @param path - ファイルのパス.
 * @returns バッファと, 出すメッセージ. ファイルが無ければ, 空のバッファ.
 */
export async function visitFile(path: string): Promise<{ buffer: Buffer; message: string }> {
  const exists = await pathExists(path)
  const content = exists ? await readFile(path) : ''
  const name = bufferNameFromPath(path)
  const buffer: Buffer = {
    id: generateBufferId(),
    name,
    filePath: path,
    doc: toText(content),
    modified: false,
  }
  if (await fileNewerThan(autoSaveFileName(path), path)) {
    return { buffer, message: `${name} has auto save data; consider M-x recover-this-file` }
  }
  return { buffer, message: exists ? '' : '(New file)' }
}

/** 名前を, ディレクトリで区別するバッファ (ファイルか, dired のバッファ). */
interface NamedByPath {
  /** バッファの添え字. */
  index: number
  /** もとの名前 (ファイルの名前, または dired のディレクトリの名前). */
  base: string
  /** 親のディレクトリの名前 (近い順). */
  parents: string[]
}

/**
 * 同じ名前のファイル (dired のディレクトリも) のバッファの名前を, 親のディレクトリで区別します
 * (Emacs の uniquify の `post-forward-angle-brackets`. `index.ts<src>`, `index.ts<lib>`).
 * 区別できるまで, 親のディレクトリを近い方から足します. 重なりが無くなれば, もとの名前に戻します.
 * それでも重なる名前 (同じファイルや, ファイルのないバッファと重なる名前) には, `<2>` から番号を付けます.
 *
 * @param buffers - バッファ.
 * @returns 名前を付け直したバッファ. 名前が変わらなければ, 同じ配列 (変わらないバッファは, 同じもの).
 */
export function uniquifyBufferNames(buffers: Buffer[]): Buffer[] {
  const names = buffers.map((b) => b.name)
  const named: NamedByPath[] = []
  buffers.forEach((b, index) => {
    const path = b.filePath ?? (b.directory !== undefined ? directoryFileName(b.directory) : null)
    if (path === null) return
    const base = b.filePath ? bufferNameFromPath(path) : diredBufferName(path)
    const parents = path.split('/').slice(0, -1).filter(Boolean).reverse()
    named.push({ index, base, parents })
  })
  for (const group of groupBy(named, (n) => n.base)) {
    if (group.length === 1) names[group[0].index] = group[0].base
    else distinguish(group, 1, names)
  }
  // ファイルのないバッファは, 名前を変えない
  const used = new Set(
    buffers.filter((b, i) => !named.some((n) => n.index === i)).map((b) => b.name),
  )
  for (const { index } of named) {
    let name = names[index]
    for (let n = 2; used.has(name); n++) name = `${names[index]}<${n}>`
    used.add(name)
    names[index] = name
  }
  if (names.every((name, i) => name === buffers[i].name)) return buffers
  return buffers.map((b, i) => (b.name === names[i] ? b : { ...b, name: names[i] }))
}

/**
 * 同じ名前のバッファを, 親のディレクトリを `depth` 個まで付けて区別します. 区別できないものは, 1 つ深くします.
 *
 * @param group - 同じ名前のバッファ.
 * @param depth - 付ける親のディレクトリの数.
 * @param names - 決めた名前を入れる先 (バッファの添え字ごと).
 */
function distinguish(group: NamedByPath[], depth: number, names: string[]) {
  const suffix = (n: NamedByPath) => n.parents.slice(0, depth).reverse().join('/')
  for (const same of groupBy(group, suffix)) {
    // 親のディレクトリを全て付けても同じ (同じファイル) なら, それ以上は深くしない
    if (same.length > 1 && same.some((n) => n.parents.length > depth)) {
      distinguish(same, depth + 1, names)
    } else {
      for (const n of same) names[n.index] = `${n.base}<${suffix(n) || '/'}>`
    }
  }
}

/**
 * 鍵ごとに分けます (始めに現れた順).
 *
 * @param items - 分けるもの.
 * @param key - 鍵を返す関数.
 * @returns 鍵ごとの組.
 */
function groupBy<T>(items: T[], key: (item: T) => string): T[][] {
  const groups = new Map<string, T[]>()
  for (const item of items) {
    const k = key(item)
    groups.set(k, [...(groups.get(k) ?? []), item])
  }
  return [...groups.values()]
}
