/**
 * キーボードマクロ (`C-x (`, `C-x )`, `C-x e`).
 *
 * エディタで押したキーを記録し, 再生するときは, 同じキーをエディタのキーの割り当てで動かします.
 * 割り当てのない文字のキーは, その文字を入れます. ミニバッファに入力した文字は, 記録しません.
 */

import { EditorView, runScopeHandlers } from '@codemirror/view'

/** 記録したキー. */
export interface RecordedKey {
  /** キー (`KeyboardEvent.key`). */
  key: string
  /** Ctrl を押していたら `true`. */
  ctrlKey: boolean
  /** Alt を押していたら `true`. */
  altKey: boolean
  /** Shift を押していたら `true`. */
  shiftKey: boolean
  /** Meta を押していたら `true`. */
  metaKey: boolean
}

/** 修飾キーだけを押したときの, キーの名前 (記録しない). */
const MODIFIER_KEYS = ['Control', 'Alt', 'Shift', 'Meta']

/** キーボードマクロ. 全てのバッファで共通です. */
export class KeyboardMacro {
  private recording: RecordedKey[] | null = null
  private last: RecordedKey[] = []
  private replaying = false
  // 直前に動いた割り当ての, キーの数 (終わりのキー `C-x )` を, 記録から除くため)
  private strokes = 0

  /** 記録しているなら `true`. */
  get isRecording(): boolean {
    return this.recording !== null
  }

  /** 再生しているなら `true`. */
  get isReplaying(): boolean {
    return this.replaying
  }

  /** 記録したマクロがあれば `true`. */
  get isDefined(): boolean {
    return this.last.length > 0
  }

  /**
   * 記録を始めます.
   *
   * @returns 始めたら `true` (記録か再生をしているあいだは `false`).
   */
  start(): boolean {
    if (this.recording || this.replaying) return false
    this.recording = []
    return true
  }

  /**
   * 記録を終えます. 終えるのに押したキー (直前の割り当て) は, 記録から除きます.
   *
   * @returns 終えたら `true` (記録していなければ `false`).
   */
  end(): boolean {
    if (!this.recording) return false
    this.last = this.recording.slice(0, Math.max(this.recording.length - this.strokes, 0))
    this.recording = null
    return true
  }

  /**
   * 直前の割り当てのキーを, 記録から除きます (記録中に, 使えないコマンドを押したとき).
   */
  dropLastBinding(): void {
    this.recording?.splice(Math.max(this.recording.length - this.strokes, 0))
  }

  /**
   * キーの割り当てが動いたことを知らせます.
   *
   * @param strokes - 割り当てのキーの数 (`C-x )` なら 2).
   */
  noteBinding(strokes: number): void {
    this.strokes = strokes
  }

  /**
   * 押したキーを記録します (記録しているときだけ. 修飾キーだけのものと, 再生したものは除く).
   *
   * @param event - キーの事象.
   */
  record(event: KeyboardEvent): void {
    if (!this.recording || this.replaying || MODIFIER_KEYS.includes(event.key)) return
    const { key, ctrlKey, altKey, shiftKey, metaKey } = event
    this.recording.push({ key, ctrlKey, altKey, shiftKey, metaKey })
  }

  /**
   * 記録したキーを, エディタで再生します.
   *
   * @param view - エディタ.
   */
  replay(view: EditorView): void {
    this.replaying = true
    try {
      for (const key of this.last) {
        const event = new KeyboardEvent('keydown', key)
        if (runScopeHandlers(view, event, 'editor')) continue
        if (key.key.length === 1 && !key.ctrlKey && !key.altKey && !key.metaKey) {
          view.dispatch(view.state.replaceSelection(key.key), {
            scrollIntoView: true,
            userEvent: 'input.type',
          })
        }
      }
    } finally {
      this.replaying = false
    }
  }
}
