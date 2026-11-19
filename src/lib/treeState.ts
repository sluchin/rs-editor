/**
 * サイドバーのツリーの状態 (出しているか, 根, 幅) を, 次の起動まで覚えます.
 * 保存先 (localStorage) が使えないときは, 覚えないだけで, 失敗しません.
 */

const ROOT_KEY = 'rseditor.projectTree.root'
const WIDTH_KEY = 'rseditor.projectTree.width'

/**
 * 保存した根を返します.
 *
 * @returns 出していたときの根. 出していなかった (または, 保存が無い) ときは null.
 */
export function loadTreeRoot(): string | null {
  try {
    return localStorage.getItem(ROOT_KEY) || null
  } catch {
    return null
  }
}

/**
 * 根を保存します (null は, 出していないこと).
 *
 * @param root - 根.
 */
export function saveTreeRoot(root: string | null): void {
  try {
    if (root === null) localStorage.removeItem(ROOT_KEY)
    else localStorage.setItem(ROOT_KEY, root)
  } catch {
    // 保存できなくても, 動きは変わらない
  }
}

/**
 * 保存した幅を返します.
 *
 * @returns 幅 (px). 無いときは null.
 */
export function loadTreeWidth(): number | null {
  try {
    const value = Number(localStorage.getItem(WIDTH_KEY))
    return Number.isFinite(value) && value > 0 ? value : null
  } catch {
    return null
  }
}

/**
 * 幅を保存します.
 *
 * @param width - 幅 (px).
 */
export function saveTreeWidth(width: number): void {
  try {
    localStorage.setItem(WIDTH_KEY, String(width))
  } catch {
    // 保存できなくても, 動きは変わらない
  }
}
