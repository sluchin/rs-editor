/**
 * 字下げ (Emacs 30 の `lisp-indent-line` と, Fundamental モードの `indent-relative`).
 *
 * Lisp の字下げは, Emacs の `calculate-lisp-indent`, `lisp-indent-function` (`scheme-indent-function`),
 * `lisp-indent-specform`, `lisp-indent-defform` を, 開いているリストの要素の位置から計算し直したものです.
 * 桁は, Emacs と同じく, タブを `tab-width` の倍数まで, 全角の文字を 2 桁と数えます.
 */

import { EditorSelection, type EditorState, Facet, type ChangeSpec } from '@codemirror/state'
import { ELISP_INDENT_SPECS, type IndentSpec, SCHEME_INDENT_SPECS } from './lispIndentSpecs'

/** 字下げの設定 (バッファごと). */
export interface IndentConfig {
  /** Lisp の方言 (`null` なら, Fundamental モードの字下げ). */
  dialect: 'scheme' | 'elisp' | null
  /** 字下げにタブを使うなら `true` (Emacs の `indent-tabs-mode`). */
  indentTabs: boolean
  /** タブの幅 (Emacs の `tab-width`). */
  tabWidth: number
}

/** Emacs の既定の字下げの設定. */
const DEFAULT_CONFIG: IndentConfig = { dialect: null, indentTabs: true, tabWidth: 8 }

/** 字下げの設定. */
export const indentConfig = Facet.define<IndentConfig, IndentConfig>({
  combine: (values) => values[0] ?? DEFAULT_CONFIG,
})

/** `;` 1 つのコメントの行の桁 (Emacs の `comment-column`). */
const COMMENT_COLUMN = 40

/** 本体の字下げの幅 (Emacs の `lisp-body-indent`). */
const BODY_INDENT = 2

/**
 * 文字の幅を返します (全角の文字は 2).
 *
 * @param code - 文字のコード.
 * @returns 幅.
 */
export function charWidth(code: number): number {
  const wide =
    (code >= 0x1100 && code <= 0x115f) ||
    (code >= 0x2e80 && code <= 0x303e) ||
    (code >= 0x3041 && code <= 0x33ff) ||
    (code >= 0x3400 && code <= 0x4dbf) ||
    (code >= 0x4e00 && code <= 0x9fff) ||
    (code >= 0xa000 && code <= 0xa4cf) ||
    (code >= 0xac00 && code <= 0xd7a3) ||
    (code >= 0xf900 && code <= 0xfaff) ||
    (code >= 0xfe30 && code <= 0xfe4f) ||
    (code >= 0xff00 && code <= 0xff60) ||
    (code >= 0xffe0 && code <= 0xffe6) ||
    (code >= 0x1f300 && code <= 0x1f64f) ||
    (code >= 0x1f900 && code <= 0x1f9ff) ||
    (code >= 0x20000 && code <= 0x3fffd)
  return wide ? 2 : 1
}

/**
 * 行の, 位置の桁を返します.
 *
 * @param text - 行.
 * @param index - 位置 (行の中).
 * @param tabWidth - タブの幅.
 * @returns 桁.
 */
export function columnAt(text: string, index: number, tabWidth: number): number {
  let col = 0
  for (let i = 0; i < index;) {
    const code = text.codePointAt(i) as number
    col = code === 9 ? (Math.floor(col / tabWidth) + 1) * tabWidth : col + charWidth(code)
    i += code > 0xffff ? 2 : 1
  }
  return col
}

/**
 * 桁から桁までを埋める空白を返します (Emacs の `indent-to`).
 *
 * @param from - 始めの桁.
 * @param to - 終わりの桁.
 * @param config - 字下げの設定.
 * @returns 空白.
 */
function whitespace(from: number, to: number, config: IndentConfig): string {
  const { indentTabs, tabWidth } = config
  const tabs = indentTabs ? Math.floor(to / tabWidth) - Math.floor(from / tabWidth) : 0
  if (tabs <= 0) return ' '.repeat(Math.max(0, to - from))
  return '\t'.repeat(tabs) + ' '.repeat(to - Math.floor(to / tabWidth) * tabWidth)
}

/**
 * 字下げの空白を, 桁まで変えます (Emacs の `indent-line-to`. 今の空白は, できるだけ残す).
 *
 * @param old - 今の字下げの空白.
 * @param col - 桁.
 * @param config - 字下げの設定.
 * @returns 新しい字下げの空白.
 */
function indentTo(old: string, col: number, config: IndentConfig): string {
  const { tabWidth } = config
  const current = columnAt(old, old.length, tabWidth)
  if (current > col) {
    // 桁までを残す (タブの途中なら, タブを空白にする)
    let i = 0
    while (columnAt(old, i + 1, tabWidth) <= col) i++
    return old.slice(0, i) + ' '.repeat(col - columnAt(old, i, tabWidth))
  }
  // タブを入れられるなら, 後ろの空白を消してから入れる
  const base =
    col - Math.floor(current / tabWidth) * tabWidth >= tabWidth ? old.replace(/ +$/, '') : old
  return base + whitespace(columnAt(base, base.length, tabWidth), col, config)
}

/** 行の始めの空白の長さを返します. */
const indentLength = (text: string) => (/^[ \t]*/.exec(text) as RegExpExecArray)[0].length

/** リストの要素. */
interface Element {
  /** 桁 (前の `'` などを含む). */
  col: number
  /** 行 (0 から). */
  line: number
  /** 種類 (`'` などの後ろで見る). */
  kind: 'list' | 'symbol' | 'other'
  /** 名前 (シンボルなら). */
  text: string
}

/** 開いているリスト. */
interface Frame {
  /** `(` の桁. */
  col: number
  /** `(` の行. */
  line: number
  /** これまでに読んだ, リストの要素 (前から順). */
  elements: Element[]
}

/**
 * Lisp の本文を, 行ごとに読みます (Emacs の `parse-partial-sexp`). 開いているリストと, 文字列の中かを覚えます.
 */
export class LispParser {
  stack: Frame[] = []
  inString = false
  /** 文字列が, `#;` でコメントにした式なら `true` (Emacs は, コメントの中と見る). */
  commentString = false
  /** `#| |#` のコメントの深さ (Scheme). */
  blockComment = 0
  /** `#;` でコメントにした式の, 括弧の深さ (Scheme. コメントでなければ `null`). */
  datum: number | null = null
  /** 行ごとの, 始めの式の桁 (行の始めから読んで, 始めに式が始まる所. 前の `'` などを含む). */
  firstSexpCol: number[] = []
  /** 括弧 (文字列, コメント, 文字の中のものを除く) を読んだときに呼びます (括弧の強調で使う). */
  onParen?: (line: number, index: number, ch: string) => void

  constructor(
    private readonly scheme: boolean,
    private readonly tabWidth: number,
  ) {}

  /**
   * 1 行を読みます.
   *
   * @param text - 行.
   * @param line - 行 (0 から).
   * @param start - 読み始める位置.
   * @param end - 読み終える位置.
   */
  scanLine(text: string, line: number, start = 0, end = text.length) {
    let i = start
    let prefix: number | null = null
    const col = () => columnAt(text, i, this.tabWidth)
    /** 式を, 開いているリストの要素にします. */
    const element = (kind: Element['kind'], at: number, name = '') => {
      const c = prefix ?? columnAt(text, at, this.tabWidth)
      prefix = null
      this.firstSexpCol[line] ??= c
      this.stack[this.stack.length - 1]?.elements.push({ col: c, line, kind, text: name })
    }
    while (i < end) {
      const ch = text[i]
      if (this.inString) {
        // 行が文字列の中で始まると, Emacs は閉じる `"` を, 式の始めと見る
        if (ch === '"') {
          if (!this.commentString) this.firstSexpCol[line] ??= col()
          this.inString = false
          this.commentString = false
        }
        i += ch === '\\' ? 2 : 1
      } else if (this.blockComment > 0) {
        const close = text.startsWith('|#', i)
        const open = text.startsWith('#|', i)
        this.blockComment += close ? -1 : open ? 1 : 0
        i += close || open ? 2 : 1
      } else if (ch === ' ' || ch === '\t') {
        i++
      } else if (ch === ';') {
        break
      } else if (this.datum !== null) {
        i = this.skipDatum(text, i)
      } else if (this.scheme && (text.startsWith('#|', i) || text.startsWith('#;', i))) {
        // `#;` の後の式は, コメント (Emacs の scheme-mode)
        if (text[i + 1] === '|') this.blockComment = 1
        else this.datum = 0
        i += 2
      } else if (ch === '"') {
        element('other', i)
        this.inString = true
        i++
      } else if (ch === '(' || ch === '[') {
        this.onParen?.(line, i, ch)
        element('list', i)
        this.stack.push({ col: col(), line, elements: [] })
        i++
      } else if (ch === ')' || ch === ']') {
        this.onParen?.(line, i, ch)
        this.stack.pop()
        prefix = null
        i++
      } else if (
        ch === "'" ||
        ch === '`' ||
        ch === ',' ||
        (ch === '#' && !(this.scheme && text[i + 1] === '\\'))
      ) {
        // 式の前の印 (`'`, `` ` ``, `,`, `,@`, `#'`, `#(`. Emacs の構文表では, `#` はいつも前の印)
        prefix ??= col()
        i += (ch === ',' && text[i + 1] === '@') || (ch === '#' && text[i + 1] === "'") ? 2 : 1
      } else {
        const from = i
        // Scheme の文字 (`#\(`) は, 括弧も名前の文字. Emacs Lisp の `?\(` は `\` で括弧を名前の文字にする
        // (`?(` の `(` は, Emacs と同じく開く括弧)
        if (this.scheme && text.startsWith('#\\', i)) i += 3
        while (i < text.length && !/[\s()[\]";'`,]/.test(text[i])) i += text[i] === '\\' ? 2 : 1
        element('symbol', from, text.slice(from, i))
      }
    }
  }

  /**
   * `#;` でコメントにした式の, 字句を 1 つ読み飛ばします. 式が終わったら, コメントを終えます.
   *
   * @param text - 行.
   * @param i - 位置 (空白でない).
   * @returns 次の位置.
   */
  private skipDatum(text: string, i: number): number {
    const ch = text[i]
    // 式の前の印 (`'`, `#(` など. `#\` で始まる文字を除く) は, 後ろの式と合わせて 1 つ
    if ("'`,".includes(ch) || (ch === '#' && text[i + 1] !== '\\')) return i + 1
    const depth = this.datum as number
    let next = i + 1
    if (ch === '(' || ch === '[') this.datum = depth + 1
    else if (ch === ')' || ch === ']') this.datum = depth - 1
    else if (ch === '"') this.inString = this.commentString = true
    else {
      if (text.startsWith('#\\', i)) next = i + 3
      while (next < text.length && !/[\s()[\]";'`,]/.test(text[next])) next++
    }
    if ((this.datum as number) <= 0) this.datum = null
    return next
  }
}

/**
 * Emacs の `lisp-indent-specform` (特別な引数が `count` 個のフォーム).
 *
 * @param count - 特別な引数の数.
 * @param frame - 開いているリスト.
 * @param normal - ふつうの字下げ.
 * @returns 字下げの桁.
 */
function specform(count: number, frame: Frame, normal: number): number {
  // 行の前にある引数の数 (フォームの名前を除く)
  const args = frame.elements.length - 1
  const left = count - args
  const body = frame.col + BODY_INDENT
  // 特別な引数の行は, 始めの 2 つまでは, 本体の倍の字下げ
  if (left > 0) return args <= 1 ? frame.col + 2 * BODY_INDENT : normal
  if ((count === 0 && left === 0) || (left === 0 && body <= normal)) return body
  return normal
}

/**
 * 行の, Lisp の字下げの桁を計算します (Emacs の `calculate-lisp-indent`).
 *
 * @param parser - 行の始めまで読んだ状態.
 * @param text - 行.
 * @param scheme - Scheme なら `true`.
 * @returns 桁. 変えないなら `null`.
 */
function lispIndent(parser: LispParser, text: string, scheme: boolean): number | null {
  const rest = text.slice(indentLength(text))
  // 文字列の中で始まる行と, `;;;` のコメントは, 変えない. `;` 1 つのコメントは, コメントの桁
  if ((parser.inString && !parser.commentString) || rest.startsWith(';;;')) return null
  if (/^;(?!;)/.test(rest)) return COMMENT_COLUMN
  const frame = parser.stack[parser.stack.length - 1]
  if (!frame) return 0
  const { elements } = frame
  if (elements.length === 0) return frame.col + 1
  const first = elements[0]
  const last = elements[elements.length - 1]
  // 最後の式が始まる行の, 始めの式の桁
  const lastLineCol = parser.firstSexpCol[last.line]
  // フォームの名前がシンボルでなければ, 名前か, 最後の式の行の始めの式にそろえる
  if (first.kind !== 'symbol') return last.line === first.line ? first.col : lastLineCol
  // ふつうの字下げ: 名前と同じ行に最後の式があれば, 1 つ目の引数 (無ければ名前) にそろえる
  let normal = lastLineCol
  if (last.line === first.line) normal = last === first ? first.col : elements[1].col
  const name = first.text
  const spec: IndentSpec | undefined = (scheme ? SCHEME_INDENT_SPECS : ELISP_INDENT_SPECS).get(name)
  // 名前が def で始まるフォームを定義の形にするのは, Scheme だけ (Emacs 30 の Emacs Lisp は, 指定だけを見る)
  const defName = scheme && spec === undefined && name.length > 3 && name.startsWith('def')
  if (spec === 'defun' || defName) {
    // 定義の形 (Emacs の `lisp-indent-defform`): `(` の行に最後の式があれば, 本体の字下げ
    return last.line === frame.line ? frame.col + BODY_INDENT : normal
  }
  if (spec === 'let') {
    // 名前付きの let (名前が, let と同じ行にある) は, 特別な引数が 2 つ
    const second = elements[1] as Element | undefined
    const named =
      second?.line === first.line &&
      second.kind === 'symbol' &&
      /^[-a-zA-Z0-9+*/?!@$%^&_:~]/.test(second.text)
    return specform(named ? 2 : 1, frame, normal)
  }
  return typeof spec === 'number' ? specform(spec, frame, normal) : normal
}

/**
 * 行の, Fundamental モードの字下げの桁を計算します (Emacs の `indent-according-to-mode` の,
 * `indent-relative` の代わり): 前の行が空白だけなら 0, そうでなければ前の行の字下げ.
 *
 * @param previous - 前の行 (無ければ `null`).
 * @param tabWidth - タブの幅.
 * @returns 桁.
 */
function fundamentalIndent(previous: string | null, tabWidth: number): number {
  if (previous === null || /^[ \t]*$/.test(previous)) return 0
  return columnAt(previous, indentLength(previous), tabWidth)
}

/** 字下げを変える行. */
export interface IndentLinesOptions {
  /** 始めの行 (1 から). */
  from: number
  /** 終わりの行 (1 から). */
  to: number
  /** 空白だけの行を変えないなら `true` (Emacs の `indent-sexp`). */
  skipBlank?: boolean
  /** 空の行も字下げするなら `true` (`RET` の新しい行). 省くと, 空の行は変えない. */
  emptyLines?: boolean
  /** 字下げを続ける間 (`indent-sexp`. 行の始めで, リストの深さを見る). */
  whileDeeper?: { line: number; index: number }
}

/** 行の字下げを変えた結果. */
export interface IndentResult {
  /** 変更 (元の本文の位置). */
  changes: ChangeSpec[]
  /** 行ごとの, 字下げを変えた後の本文 (字下げを変えた行だけ). */
  lines: Map<number, string>
  /** 字下げを計算した行 (変えなかった行も. 文字列の中の行などは除く). */
  computed: Set<number>
}

/**
 * 行の字下げを, 上から順に変えます (変えた行の後の行は, 変えた後の行を見て計算する).
 *
 * @param state - エディタの状態.
 * @param options - 字下げを変える行.
 * @returns 変更.
 */
export function indentLines(state: EditorState, options: IndentLinesOptions): IndentResult {
  const config = state.facet(indentConfig)
  const { doc } = state
  const changes: ChangeSpec[] = []
  const lines = new Map<number, string>()
  const computed = new Set<number>()
  const parser = config.dialect && new LispParser(config.dialect === 'scheme', config.tabWidth)
  let previous: string | null = options.from > 1 ? doc.line(options.from - 1).text : null
  for (let n = 1; n < options.from; n++) parser?.scanLine(doc.line(n).text, n - 1)
  // indent-sexp: カーソルの行の, カーソルの所の深さより深い間だけ
  let depth = 0
  const deeper = options.whileDeeper
  if (deeper && parser) {
    const text = doc.line(deeper.line).text
    parser.scanLine(text, deeper.line - 1, 0, deeper.index)
    depth = parser.stack.length
    parser.scanLine(text, deeper.line - 1, deeper.index)
  }
  for (let n = options.from; n <= Math.min(options.to, doc.lines); n++) {
    const line = doc.line(n)
    let text = line.text
    if (deeper && parser && parser.stack.length <= depth) break
    const blank = /^[ \t]*$/.test(text)
    if ((text !== '' || options.emptyLines) && !(blank && options.skipBlank)) {
      const col = parser
        ? lispIndent(parser, text, config.dialect === 'scheme')
        : fundamentalIndent(previous, config.tabWidth)
      const length = indentLength(text)
      if (col !== null) computed.add(n)
      if (col !== null && columnAt(text, length, config.tabWidth) !== col) {
        // `;` 1 つのコメントは, 空白を入れ直す (Emacs の `comment-indent`)
        const comment = /^;(?!;)/.test(text.slice(length))
        const insert = comment
          ? whitespace(0, col, config)
          : indentTo(text.slice(0, length), col, config)
        changes.push({ from: line.from, to: line.from + length, insert })
        text = insert + text.slice(length)
        lines.set(n, text)
      }
    }
    parser?.scanLine(text, n - 1)
    previous = text
  }
  return { changes, lines, computed }
}

/**
 * 字下げを変えた後の, カーソルの位置を返します (字下げの中にあれば, 字下げの後ろへ動かす).
 *
 * @param state - エディタの状態.
 * @param pos - 元のカーソルの位置.
 * @param result - 字下げを変えた結果.
 * @returns 新しい位置 (変えた後の本文の位置).
 */
function cursorAfterIndent(state: EditorState, pos: number, result: IndentResult): number {
  const line = state.doc.lineAt(pos)
  const changes = state.changes(result.changes)
  if (!result.computed.has(line.number)) return changes.mapPos(pos, 1)
  const offset = pos - line.from
  const oldLength = indentLength(line.text)
  const newLength = indentLength(result.lines.get(line.number) ?? line.text)
  const newFrom = changes.mapPos(line.from, -1)
  return offset <= oldLength ? newFrom + newLength : newFrom + offset - oldLength + newLength
}

/**
 * `TAB` (Emacs の `indent-for-tab-command`). Lisp では行の字下げを直し, Fundamental モードでは,
 * 前の行の次の字下げの位置まで, カーソルの所に空白を入れます (`indent-relative`).
 *
 * @param state - エディタの状態.
 * @param pos - カーソルの位置.
 * @returns 変更とカーソル.
 */
export function indentForTab(state: EditorState, pos: number) {
  const config = state.facet(indentConfig)
  const line = state.doc.lineAt(pos)
  if (config.dialect) {
    const result = indentLines(state, { from: line.number, to: line.number })
    return {
      changes: result.changes,
      selection: EditorSelection.cursor(cursorAfterIndent(state, pos, result)),
    }
  }
  const col = columnAt(line.text, pos - line.from, config.tabWidth)
  const target = relativeIndent(state, line.number, col, config.tabWidth)
  // 次の字下げの位置が無ければ, 次のタブの位置へ (Emacs の `tab-to-tab-stop`. 前の空白は消す)
  if (target === null) {
    const next = (Math.floor(col / config.tabWidth) + 1) * config.tabWidth
    const before = /[ \t]*$/.exec(line.text.slice(0, pos - line.from)) as RegExpExecArray
    const from = pos - before[0].length
    const insert = whitespace(columnAt(line.text, from - line.from, config.tabWidth), next, config)
    return {
      changes: { from, to: pos, insert },
      selection: EditorSelection.cursor(from + insert.length),
    }
  }
  const insert = whitespace(col, target, config)
  return { changes: { from: pos, insert }, selection: EditorSelection.cursor(pos + insert.length) }
}

/**
 * 前の空でない行の, 桁より後ろの, 次の字下げの位置 (空白の後の, 文字の始め) を返します
 * (Emacs の `indent-relative`).
 *
 * @param state - エディタの状態.
 * @param lineNumber - 行 (1 から).
 * @param col - 桁.
 * @param tabWidth - タブの幅.
 * @returns 桁. 無ければ `null`.
 */
function relativeIndent(
  state: EditorState,
  lineNumber: number,
  col: number,
  tabWidth: number,
): number | null {
  let n = lineNumber - 1
  while (n >= 1 && state.doc.line(n).text === '') n--
  if (n < 1) return null
  const text = state.doc.line(n).text
  // 桁の所へ (タブの途中なら, タブの始め)
  let i = 0
  while (i < text.length && columnAt(text, i + 1, tabWidth) <= col) i++
  if (i < text.length && columnAt(text, i, tabWidth) < col) i++
  if (columnAt(text, i, tabWidth) > col) i--
  // 文字の途中なら, その語を飛ばし, 空白を飛ばす
  if (!/[ \t]/.test(text[i] ?? '')) while (i < text.length && !/[ \t]/.test(text[i])) i++
  while (i < text.length && /[ \t]/.test(text[i])) i++
  return i === text.length ? null : columnAt(text, i, tabWidth)
}

/**
 * `RET` (Emacs の `newline` と `electric-indent-mode`). 改行を入れ, 前の行の字下げを直して (Lisp だけ),
 * 前の行の終わりの空白を消し, 新しい行を字下げします.
 *
 * @param state - エディタの状態.
 * @param pos - カーソルの位置.
 * @returns 変更とカーソル.
 */
export function newlineAndIndent(state: EditorState, pos: number) {
  const config = state.facet(indentConfig)
  const line = state.doc.lineAt(pos)
  const after = (/^[ \t]*/.exec(line.text.slice(pos - line.from)) as RegExpExecArray)[0]
  // 前の行は, カーソルまで. 新しい行は, カーソルの後ろ
  const head = line.text.slice(0, pos - line.from)
  const tail = line.text.slice(pos - line.from + after.length)
  const lineStart = line.from
  const simulated = state.update({
    changes: { from: lineStart, to: line.to, insert: head + '\n' + after + tail },
  }).state
  // 前の行の字下げを直す (Fundamental モードは, 直さない)
  let prev = head
  if (config.dialect) {
    const result = indentLines(simulated, { from: line.number, to: line.number })
    prev = result.lines.get(line.number) ?? head
  }
  // 前の行の終わりの空白を消す
  prev = prev.replace(/[ \t]+$/, '')
  const withPrev = state.update({
    changes: { from: lineStart, to: line.to, insert: prev + '\n' + after + tail },
  }).state
  const next = indentLines(withPrev, {
    from: line.number + 1,
    to: line.number + 1,
    emptyLines: true,
  })
  const indented = next.lines.get(line.number + 1)
  const newLine = indented === undefined ? after : indented.slice(0, indentLength(indented))
  const insert = prev + '\n' + newLine
  // 字下げを計算しなかった (文字列の中) なら, カーソルは新しい行の始め
  const cursor = next.computed.has(line.number + 1)
    ? lineStart + insert.length
    : lineStart + prev.length + 1
  return {
    changes: { from: lineStart, to: pos + after.length, insert },
    selection: EditorSelection.cursor(cursor),
  }
}
