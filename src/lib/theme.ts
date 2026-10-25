/**
 * 画面の色とフォント (初期設定ファイルの `set-background-color`, `set-frame-font` など).
 *
 * 色とフォントは, 画面の一番外側の要素 (`<html>`) の CSS の変数 (`--background` など) に置きます.
 * 背景が暗いときは, `data-background-mode="dark"` を付けて, 暗い背景の色分けを使います
 * (Emacs の `frame-background-mode` と同じく, 背景の明るさで決める).
 */

/** 色の名前と, その色 (赤, 緑, 青. 0 から 255). CSS の色の名前のうち, よく使うもの. */
const NAMED_COLORS: Record<string, [number, number, number]> = {
  black: [0, 0, 0],
  white: [255, 255, 255],
  gray: [128, 128, 128],
  grey: [128, 128, 128],
  silver: [192, 192, 192],
  dimgray: [105, 105, 105],
  dimgrey: [105, 105, 105],
  darkgray: [169, 169, 169],
  darkgrey: [169, 169, 169],
  lightgray: [211, 211, 211],
  lightgrey: [211, 211, 211],
  darkslategray: [47, 79, 79],
  darkslategrey: [47, 79, 79],
  navy: [0, 0, 128],
  midnightblue: [25, 25, 112],
  maroon: [128, 0, 0],
  ivory: [255, 255, 240],
  linen: [250, 240, 230],
  wheat: [245, 222, 179],
}

/**
 * 色 (`#rgb`, `#rrggbb`, `rgb(r, g, b)`, 色の名前) を, 赤, 緑, 青にします.
 * 表にない色の名前は, 画面に色を置いて, 計算した色を読みます.
 *
 * @param color - 色.
 * @returns 赤, 緑, 青 (0 から 255). 読めなければ `null`.
 */
export function parseColor(color: string): [number, number, number] | null {
  const text = color.trim().toLowerCase()
  const short = /^#([0-9a-f])([0-9a-f])([0-9a-f])$/.exec(text)
  if (short)
    return [1, 2, 3].map((i) => parseInt(short[i] + short[i], 16)) as [number, number, number]
  const long = /^#([0-9a-f]{2})([0-9a-f]{2})([0-9a-f]{2})$/.exec(text)
  if (long) return [1, 2, 3].map((i) => parseInt(long[i], 16)) as [number, number, number]
  const rgb = /^rgba?\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)/.exec(text)
  if (rgb) return [1, 2, 3].map((i) => Number(rgb[i])) as [number, number, number]
  if (NAMED_COLORS[text]) return NAMED_COLORS[text]
  // 表にない名前は, 画面に計算させる (`rgb(...)` になる)
  const probe = document.createElement('div')
  probe.style.color = text
  if (probe.style.color === '') return null
  document.body.appendChild(probe)
  const computed = getComputedStyle(probe).color
  probe.remove()
  return /^rgba?\(/.test(computed) ? parseColor(computed) : null
}

/**
 * 背景が暗いかを返します. Emacs と同じく, 赤, 緑, 青の和が, 白の 6 割より小さければ, 暗い背景です.
 *
 * @param color - 背景の色.
 * @returns 暗ければ `true`. 色が読めなければ `null`.
 */
export function isDarkColor(color: string): boolean | null {
  const rgb = parseColor(color)
  if (!rgb) return null
  return rgb[0] + rgb[1] + rgb[2] < 255 * 3 * 0.6
}

/**
 * 背景の色を変えます. 色の明るさで, 色分け (明るい背景か, 暗い背景か) も変えます.
 *
 * @param root - 画面の一番外側の要素.
 * @param color - 色.
 * @throws 色が読めないとき.
 */
export function setBackgroundColor(root: HTMLElement, color: string) {
  const dark = isDarkColor(color)
  if (dark === null) throw new Error(`Undefined color: ${color}`)
  root.style.setProperty('--background', color)
  root.dataset.backgroundMode = dark ? 'dark' : 'light'
}

/**
 * 文字の色を変えます.
 *
 * @param root - 画面の一番外側の要素.
 * @param color - 色.
 * @throws 色が読めないとき.
 */
export function setForegroundColor(root: HTMLElement, color: string) {
  if (!parseColor(color)) throw new Error(`Undefined color: ${color}`)
  root.style.setProperty('--foreground', color)
}

/**
 * フォントを変えます.
 *
 * @param root - 画面の一番外側の要素.
 * @param family - ファミリー (`null` なら, 変えない).
 * @param size - 大きさ (ポイント. `null` なら, 変えない).
 * @throws 大きさが正の数でないとき.
 */
export function setFont(root: HTMLElement, family: string | null, size: number | null) {
  if (size !== null && !(size > 0)) throw new Error(`Invalid font size: ${size}`)
  // 見つからないときのために, 等幅のフォントを後ろに置く
  if (family !== null) root.style.setProperty('--font-family', `"${family}", monospace`)
  if (size !== null) root.style.setProperty('--font-size', `${size}pt`)
}
