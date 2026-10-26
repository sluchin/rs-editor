/**
 * 正規表現に合う所の色付け (Lisp の `highlight-regexp`). face を渡すと, その face の色にします
 * (org-mode の見出しの色など). face が無ければ, 下線を引きます.
 */

import {
  Decoration,
  type DecorationSet,
  type EditorView,
  MatchDecorator,
  ViewPlugin,
  type ViewUpdate,
} from '@codemirror/view'

/** 色付けする所 (Lisp の `(highlight-regexp REGEXP FACE SUBEXP)`). */
export interface Highlight {
  /** 正規表現 (JavaScript の書き方. 行ごとに合わせる). */
  regexp: string
  /** face の名前 (`org-level-1` など. CSS の `cm-face-<名前>`). 無ければ, 下線. */
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
 * @param face - face の名前 (無ければ, 下線).
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
 * 色付けの正規表現を確かめます (空の文字に合う正規表現は, 行ごとに探すのが終わらないので, 使えない).
 *
 * @param regexp - 正規表現.
 * @throws 正規表現が誤っているか, 空の文字に合うとき.
 */
export function checkHighlightRegexp(regexp: string): void {
  if (new RegExp(regexp).test('')) throw new Error(`Regexp matches the empty string: ${regexp}`)
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
 * @returns 拡張. 無ければ, 空.
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
