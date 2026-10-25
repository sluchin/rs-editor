/**
 * Lisp (Rust の Scheme の評価器) とのやり取り.
 *
 * - 評価: `eval_scheme` (コード), `run_lisp_command` (Lisp で作ったコマンド), `load_init` (初期設定ファイル).
 * - エディタの操作: 評価器は, `editor-request` のイベントで, 操作 (`insert`, `point` など) を頼みます.
 *   画面は, 操作をして, `editor_reply` で返事をします. 位置は, Emacs と同じく 1 から数えます.
 */

import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'

/** Lisp で割り当てたキー (`global-set-key`). キーは CodeMirror の書き方. */
export interface KeyBinding {
  /** キー (CodeMirror の書き方. `Ctrl-c a` など). */
  key: string
  /** コマンドの名前. */
  command: string
}

/** 評価の結果. */
export interface EvalResponse {
  /** 結果 (失敗したときは, 空). */
  result: string
  /** 失敗の内容 (成功したときは `null`). */
  error: string | null
  /** Lisp で作ったコマンドの名前 (`define-command`). */
  commands: string[]
  /** Lisp で割り当てたキー. */
  keys: KeyBinding[]
}

/** 評価器からの, エディタの操作の頼み. */
export interface EditorRequest {
  /** 返事 (`editor_reply`) と組にする番号. */
  id: number
  /** 操作の名前 (`insert`, `point` など). */
  op: string
  /** 引数. */
  args: unknown[]
}

/** Lisp から動かすエディタ (位置は 0 から数える). */
export interface LispHost {
  /** 本文. */
  text: () => string
  /** カーソルの位置. */
  point: () => number
  /** カーソルを置きます. */
  setPoint: (pos: number) => void
  /** カーソルの位置に入れて, カーソルを入れた文字の後へ動かします. */
  insert: (text: string) => void
  /** 範囲を消します. */
  deleteRegion: (from: number, to: number) => void
  /** バッファの名前. */
  bufferName: () => string
  /** ミニバッファに出します. */
  message: (text: string) => void
  /**
   * コマンドを名前で実行します.
   *
   * @returns コマンドがあれば `true`.
   */
  command: (name: string) => boolean
}

/**
 * コードを評価します.
 *
 * @param code - コード.
 * @returns 結果.
 */
export function evalLisp(code: string): Promise<EvalResponse> {
  return invoke<EvalResponse>('eval_scheme', { code })
}

/**
 * Lisp で作ったコマンドを実行します.
 *
 * @param name - コマンドの名前.
 * @returns 結果.
 */
export function runLispCommand(name: string): Promise<EvalResponse> {
  return invoke<EvalResponse>('run_lisp_command', { name })
}

/**
 * 初期設定ファイル (`~/.config/rseditor/init.scm`) を読んで, 評価します.
 *
 * @returns 結果.
 */
export function loadInitFile(): Promise<EvalResponse> {
  return invoke<EvalResponse>('load_init')
}

/**
 * 1 から数えた位置を, 0 から数えた位置 (本文の中) にします.
 *
 * @param value - 位置 (数でなければ, 失敗).
 * @param length - 本文の長さ.
 * @returns 0 から数えた位置.
 */
function offset(value: unknown, length: number): number {
  if (typeof value !== 'number' || Number.isNaN(value)) {
    throw new Error(`Expected a position: ${String(value)}`)
  }
  return Math.min(Math.max(Math.trunc(value) - 1, 0), length)
}

/**
 * エディタの操作をします.
 *
 * @param host - エディタ.
 * @param op - 操作の名前.
 * @param args - 引数.
 * @returns 結果 (値がなければ `null`).
 * @throws 操作が失敗したとき.
 */
export function performEditorRequest(host: LispHost, op: string, args: unknown[]): unknown {
  const length = host.text().length
  const range = (): [number, number] => {
    const [a, b] = [offset(args[0], length), offset(args[1], length)]
    return [Math.min(a, b), Math.max(a, b)]
  }
  switch (op) {
    case 'point':
      return host.point() + 1
    case 'point-min':
      return 1
    case 'point-max':
      return length + 1
    case 'goto-char': {
      const pos = offset(args[0], length)
      host.setPoint(pos)
      return pos + 1
    }
    case 'insert':
      host.insert(args.join(''))
      return null
    case 'delete-region':
      host.deleteRegion(...range())
      return null
    case 'buffer-string':
      return host.text()
    case 'buffer-substring':
      return host.text().slice(...range())
    case 'buffer-name':
      return host.bufferName()
    case 'message':
      host.message(String(args[0]))
      return null
    case 'command': {
      const name = String(args[0])
      if (!host.command(name)) throw new Error(`Undefined command: ${name}`)
      return true
    }
    default:
      throw new Error(`Unknown editor operation: ${op}`)
  }
}

/**
 * 評価器からの頼みを受けて, エディタを操作し, 返事をします.
 *
 * @param host - エディタを返す関数 (頼みのたびに呼ぶ. 画面の今の状態を使うため).
 * @returns 受けるのをやめる関数.
 */
export function listenEditorRequests(host: () => LispHost): Promise<UnlistenFn> {
  return listen<EditorRequest>('editor-request', ({ payload: { id, op, args } }) => {
    let reply: { id: number; value: unknown; error: string | null }
    try {
      reply = { id, value: performEditorRequest(host(), op, args), error: null }
    } catch (err) {
      reply = { id, value: null, error: err instanceof Error ? err.message : String(err) }
    }
    void invoke('editor_reply', reply)
  })
}
