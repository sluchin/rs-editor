/**
 * エディタ (Rust の `src-tauri/src/session.rs`) とのやり取り.
 *
 * - 入力 (キー, 文字, クリック) は, `session_input` で送ります.
 * - エディタは, コマンドの後に, 描く内容 (`frame` のイベント) を送ります.
 * - 問い (`read-string` など) と, 画面だけの操作 (色, スクロール) は, `editor-request` のイベントで頼まれます.
 *   画面は, 答えを `editor_reply` で返します.
 */

import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'

/** 本文の変更 (UTF-16 の位置). */
export interface TextChange {
  /** 始め. */
  from: number
  /** 終わり (消す所の終わり). */
  to: number
  /** 入れる文字. */
  insert: string
}

/** ウィンドウの描く内容. */
export interface WindowFrame {
  /** ウィンドウの番号. */
  id: number
  /** 左の位置 (%). */
  left: number
  /** 上の位置 (%). */
  top: number
  /** 幅 (%). */
  width: number
  /** 高さ (%). */
  height: number
  /** 出しているバッファの名前. */
  buffer: string
  /** カーソル (UTF-16 の位置). */
  point: number
  /** リージョンの端 (マークを使っているときだけ). */
  mark: number | null
}

/** 強調する正規表現 (Lisp の `highlight-regexp`). */
export interface HighlightFrame {
  /** 正規表現 (JavaScript の書き方. 行ごとに合わせる). */
  regexp: string
  /** face の名前. */
  face: string
  /** 色付けする括弧の番号. */
  group: number
}

/** バッファの描く内容. */
export interface BufferFrame {
  /** 名前. */
  name: string
  /** 本文の版. */
  version: number
  /** 本文の全体 (初めて送るときと, 読み直したとき. ほかは `null`). */
  text: string | null
  /** 前に送ってからの変更 (順に当てる). */
  changes: TextChange[]
  /** 書き換えたか. */
  modified: boolean
  /** 読み取り専用か. */
  read_only: boolean
  /** ファイルのパス. */
  file: string | null
  /** dired のディレクトリ. */
  directory: string | null
  /** 強調する正規表現. */
  highlights: HighlightFrame[]
}

/** 描く内容. */
export interface Frame {
  /** ウィンドウ. */
  windows: WindowFrame[]
  /** 選んでいるウィンドウの番号. */
  selected: number
  /** バッファ (開いた順). */
  buffers: BufferFrame[]
  /** エコー領域. */
  message: string
}

/** 画面からの入力. */
export type Input =
  | { kind: 'key'; key: string }
  | { kind: 'text'; text: string }
  | { kind: 'point'; window: number; pos: number; anchor: number | null }
  | { kind: 'command'; name: string }
  | { kind: 'find-file'; path: string }
  | { kind: 'resize'; heights: [number, number][] }
  | { kind: 'redraw' }

/** エディタからの頼み. */
export interface EditorRequest {
  /** 返事と組にする番号. */
  id: number
  /** 操作の名前. */
  op: string
  /** 引数. */
  args: unknown[]
}

/**
 * 入力をエディタに送ります.
 *
 * @param input - 入力.
 * @returns 送ったら終わる Promise.
 */
export function sendInput(input: Input): Promise<void> {
  return invoke('session_input', { input })
}

/**
 * 描く内容を受けます.
 *
 * @param handler - 受け取る関数.
 * @returns 受けるのをやめる関数.
 */
export function listenFrames(handler: (frame: Frame) => void): Promise<UnlistenFn> {
  return listen<Frame>('frame', ({ payload }) => handler(payload))
}

/**
 * エディタからの頼みを受けて, 答えます. 答えが Promise なら, 終わるのを待って答えます.
 *
 * @param handler - 頼みに答える関数 (失敗したら, 投げる).
 * @returns 受けるのをやめる関数.
 */
export function listenRequests(
  handler: (op: string, args: unknown[]) => unknown,
): Promise<UnlistenFn> {
  return listen<EditorRequest>('editor-request', ({ payload: { id, op, args } }) => {
    void new Promise((resolve) => resolve(handler(op, args))).then(
      (value) => invoke('editor_reply', { id, value: value ?? null, error: null }),
      (err) =>
        invoke('editor_reply', {
          id,
          value: null,
          error: err instanceof Error ? err.message : String(err),
        }),
    )
  })
}

/**
 * 文字 (Unicode のコードポイント) の数で数えた位置を, UTF-16 の単位の位置にします.
 *
 * @param text - 本文.
 * @param chars - 文字の数で数えた位置.
 * @returns UTF-16 の単位の位置 (本文の長さを超えない).
 */
export function utf16Offset(text: string, chars: number): number {
  let i = 0
  for (let n = 0; n < chars && i < text.length; n++) {
    i += isHighSurrogate(text.charCodeAt(i)) && isLowSurrogate(text.charCodeAt(i + 1)) ? 2 : 1
  }
  return i
}

/**
 * サロゲートペアの前の半分かを返します.
 *
 * @param code - UTF-16 の単位.
 * @returns 前の半分なら `true`.
 */
function isHighSurrogate(code: number): boolean {
  return code >= 0xd800 && code <= 0xdbff
}

/**
 * サロゲートペアの後ろの半分かを返します.
 *
 * @param code - UTF-16 の単位.
 * @returns 後ろの半分なら `true`.
 */
function isLowSurrogate(code: number): boolean {
  return code >= 0xdc00 && code <= 0xdfff
}
