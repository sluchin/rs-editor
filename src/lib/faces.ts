/**
 * 正規表現に合う所の色付け (Lisp の `highlight-regexp`). face を渡すと, その face の色にします
 * (org-mode の見出しの色など). face が無ければ下線を引きます.
 */

import { type Range } from '@codemirror/state'
import {
  Decoration,
  type DecorationSet,
  EditorView,
  MatchDecorator,
  ViewPlugin,
  type ViewUpdate,
} from '@codemirror/view'

/** 色付けする所 (Lisp の `(highlight-regexp REGEXP FACE SUBEXP)`). */
export interface Highlight {
  /** 正規表現 (JavaScript の書き方. 行ごとに合わせる). */
  regexp: string
  /** face の名前 (`org-level-1` など. CSS の `cm-face-<名前>`). 無ければ下線. */
  face?: string
  /** 色付けする括弧の番号 (0 は, 合った所の全体). */
  group?: number
}

/** 下線 (face が無いとき. howm のリンクなど). */
const underlineMark = Decoration.mark({ class: 'cm-lisp-highlight' })

/** face の印 (同じ face は, 同じ印を使う). */
const faceMarks = new Map<string, Decoration>()

/**
 * face の印を返します.
 *
 * @param face - face の名前 (無ければ下線).
 * @returns 印.
 */
function faceMark(face: string | undefined): Decoration {
  if (!face) return underlineMark
  let mark = faceMarks.get(face)
  if (!mark) {
    mark = Decoration.mark({ class: `cm-face-${face}` })
    faceMarks.set(face, mark)
  }
  return mark
}

/**
 * 1 つの色付けの, 印を付ける部品を作ります.
 *
 * @param highlight - 色付けする所.
 * @returns 部品.
 */
function decorator({ regexp, face, group = 0 }: Highlight): MatchDecorator {
  const mark = faceMark(face)
  return new MatchDecorator({
    regexp: new RegExp(regexp, 'gd'),
    decorate: (add, from, _to, match) => {
      // 括弧が合っていなければ (省ける括弧), 付けない
      // 'd' のフラグで, 括弧ごとの位置 (行の中) が入る
      const range = (match as RegExpExecArray & { indices: ([number, number] | undefined)[] })
        .indices[group]
      if (range && range[1] > range[0]) {
        add(from - match.index + range[0], from - match.index + range[1], mark)
      }
    },
  })
}

/**
 * 正規表現に合う所を色付けする拡張を作ります.
 *
 * @param highlights - 色付けする所 (後のものを上に重ねる).
 * @returns 拡張. 無ければ空.
 */
export function regexpHighlighting(highlights: readonly Highlight[]) {
  return highlights.map((highlight) => {
    const deco = decorator(highlight)
    return ViewPlugin.fromClass(
      class {
        decorations: DecorationSet
        constructor(view: EditorView) {
          this.decorations = deco.createDeco(view)
        }
        update(update: ViewUpdate) {
          this.decorations = deco.updateDeco(update, this.decorations)
        }
      },
      { decorations: (plugin) => plugin.decorations },
    )
  })
}

/** シンタックスハイライトの字句 (核が返す. 位置は UTF-16). */
export interface Token {
  /** 始まり. */
  from: number
  /** 終わり (この位置の文字は含まない). */
  to: number
  /** 種類の名前 (CSS のクラスは `cm-tok-<名前>`). */
  kind: string
}

/** 種類の印 (同じ種類は, 同じ印を使う). */
const tokenMarks = new Map<string, Decoration>()

/**
 * 種類の印を返します.
 *
 * @param kind - 種類の名前.
 * @returns 印.
 */
function tokenMark(kind: string): Decoration {
  let mark = tokenMarks.get(kind)
  if (!mark) {
    mark = Decoration.mark({ class: `cm-tok-${kind}` })
    tokenMarks.set(kind, mark)
  }
  return mark
}

/**
 * 核が返したシンタックスハイライトの字句を, 色にする拡張を作ります (C, C++, Scheme, Emacs Lisp).
 *
 * 本文より先へ出る範囲は, 飛ばします (本文と字句は同じ描く内容で届くが, 反映の順で一瞬ずれるため).
 *
 * @param tokens - 字句.
 * @returns 拡張. 字句が無ければ空.
 */
export function tokenHighlighting(tokens: readonly Token[]) {
  if (tokens.length === 0) return []
  let built: { length: number; set: DecorationSet } | null = null
  return EditorView.decorations.of((view) => {
    const length = view.state.doc.length
    if (built?.length !== length) {
      const ranges: Range<Decoration>[] = tokens
        .filter((t) => t.from < t.to && t.to <= length)
        .map((t) => tokenMark(t.kind).range(t.from, t.to))
      built = { length, set: Decoration.set(ranges, true) }
    }
    return built.set
  })
}
