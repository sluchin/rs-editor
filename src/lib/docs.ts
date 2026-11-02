/**
 * バッファの本文 (CodeMirror の `Text`). エディタから受けた描く内容 (全体か, 変更) を当てて, 本文を持ちます.
 * ウィンドウは, 前の本文からの変更 (`ChangeSet`) をそのまま当てて, 色分けの状態を作り直さないようにします.
 */

import { ChangeSet, Text } from '@codemirror/state'
import type { BufferFrame } from './session'

/** バッファの本文. */
export interface DocEntry {
  /** 今の本文. */
  doc: Text
  /** 前の描く内容での本文. */
  prev: Text
  /** 前の本文からの変更 (全体を受けたときと, 変えていないときは `null`). */
  changes: ChangeSet | null
}

/**
 * 文字列を, CodeMirror の本文にします.
 *
 * @param text - 文字列.
 * @returns 本文.
 */
export function toText(text: string): Text {
  return Text.of(text.split('\n'))
}

/** バッファの本文の集まり. */
export class DocStore {
  /** バッファの名前と, 本文. */
  private entries = new Map<string, DocEntry>()

  /**
   * 描く内容のバッファを当てます. 描く内容に無いバッファ (消したバッファ) は, 忘れます.
   *
   * @param buffers - バッファ.
   */
  apply(buffers: readonly BufferFrame[]): void {
    const names = new Set(buffers.map((b) => b.name))
    for (const name of [...this.entries.keys()]) {
      if (!names.has(name)) this.entries.delete(name)
    }
    for (const buffer of buffers) {
      const old = this.entries.get(buffer.name)
      if (buffer.text !== null) {
        const doc = toText(buffer.text)
        this.entries.set(buffer.name, { doc, prev: old?.doc ?? doc, changes: null })
      } else if (old) {
        let doc = old.doc
        let changes: ChangeSet | null = null
        for (const change of buffer.changes) {
          const set = ChangeSet.of(change, doc.length)
          changes = changes ? changes.compose(set) : set
          doc = set.apply(doc)
        }
        this.entries.set(buffer.name, { doc, prev: old.doc, changes })
      } else {
        // 本文をまだ受けていない (出していない) バッファ
        this.entries.set(buffer.name, { doc: Text.empty, prev: Text.empty, changes: null })
      }
    }
  }

  /**
   * バッファの本文を返します.
   *
   * @param name - バッファの名前.
   * @returns 本文. 知らないバッファなら, 空の本文.
   */
  get(name: string): DocEntry {
    return this.entries.get(name) ?? { doc: Text.empty, prev: Text.empty, changes: null }
  }
}
