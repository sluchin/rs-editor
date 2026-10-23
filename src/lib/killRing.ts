/**
 * kill-ring (キルした文字の履歴). 全てのバッファで共通に使います.
 */

/** 覚えておく数. 超えたら, 古い方から捨てます. */
export const KILL_RING_MAX = 60

/**
 * キルした文字を覚えて, ヤンクで取り出します. 新しいものが先頭です.
 */
export class KillRing {
  /** キルした文字 (新しい順). */
  private entries: string[] = []
  /** ヤンクで取り出す位置 (M-y で, 古い方へ進む). */
  private index = 0

  /**
   * キルした文字を加えます. 続けてキルしたときは, 先頭の項目につなげます
   * (前へ消したときは前に, 後ろへ消したときは後ろに).
   *
   * @param text - キルした文字.
   * @param join - 先頭の項目につなげるなら, つなげる側. 新しい項目にするなら, `null`.
   */
  push(text: string, join: 'append' | 'prepend' | null = null): void {
    if (join && this.entries.length > 0) {
      this.entries[0] = join === 'append' ? this.entries[0] + text : text + this.entries[0]
    } else {
      this.entries.unshift(text)
      if (this.entries.length > KILL_RING_MAX) this.entries.pop()
    }
    this.index = 0
  }

  /**
   * 次にヤンクする文字を返します.
   *
   * @returns 文字. 何もキルしていなければ, `undefined`.
   */
  current(): string | undefined {
    return this.entries[this.index]
  }

  /**
   * 取り出す位置を, 1 つ古い方へ進めます (最後の次は, 最新に戻ります).
   *
   * @returns 進めた先の文字. 何もキルしていなければ, `undefined`.
   */
  rotate(): string | undefined {
    if (this.entries.length === 0) return undefined
    this.index = (this.index + 1) % this.entries.length
    return this.entries[this.index]
  }

  /** 取り出す位置を, 最新に戻します (ヤンクの始め). */
  reset(): void {
    this.index = 0
  }
}
