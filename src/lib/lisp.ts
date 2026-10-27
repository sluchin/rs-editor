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
  /** 手続きのある, エディタが実行するフック (`find-file-hook`, `after-save-hook`). */
  hooks?: string[]
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
  /** 名前のバッファ (無ければ, 作る) を出します. 出したら, 終わります. */
  switchToBuffer: (name: string) => Promise<void>
  /** バッファ (省くと, 今のバッファ) のファイルのパス (ファイルのないバッファは `null`). */
  bufferFileName: (name?: string) => string | null
  /** バッファ (省くと, 今のバッファ) を, 書き換えていれば `true`. */
  bufferModified: (name?: string) => boolean
  /** バッファの名前 (開いた順). */
  bufferList: () => string[]
  /**
   * バッファ (省くと, 今のバッファ) を消します. 消したら, 終わります (出していたウィンドウには, 別のバッファを出す).
   * 書き換えていれば, 消さずに失敗します.
   */
  killBuffer: (name?: string) => Promise<void>
  /** 文字を, kill-ring に入れます (C-y で貼れる). */
  killNew: (text: string) => void
  /** ミニバッファで, 候補を Tab で補完しながら, 入力してもらいます (C-g でやめたら, 失敗). */
  completingRead: (prompt: string, candidates: string[], initial: string) => Promise<string>
  /**
   * バッファを, ほかのウィンドウに出します (選ばない. ウィンドウが 1 つなら, 下に分ける). 出したら, 終わります.
   * 行 (1 から) を渡すと, そのウィンドウのカーソルを, その行に置きます.
   */
  displayBuffer: (name: string, line: number | null) => Promise<void>
  /** ファイルを開いて (開いていれば, そのバッファ), 出さずに, バッファの名前を返します. */
  findFileNoselect: (path: string) => Promise<string>
  /**
   * 今のバッファの, 正規表現 (JavaScript の書き方) に合う所を, face の色にします (face が無ければ, 下線).
   * 括弧の番号を渡すと, その括弧に合った所だけ.
   */
  highlightRegexp: (regexp: string, face: string | null, group: number | null) => void
  /** 今のバッファの下線を消します (`true` なら, 全て). */
  unhighlightRegexp: (regexp: string | true) => void
  /** 範囲を隠すか (`true`), 出します (`false`) (Emacs の `outline-flag-region`). */
  outlineFlagRegion: (from: number, to: number, hide: boolean) => void
  /** 位置の文字を, 隠していれば `true` (Emacs の `outline-invisible-p`). */
  invisible: (pos: number) => boolean
  /** バッファを, 書き換えたことにします (`false` なら, 書き換えていないことにする). 画面に出したら, 終わります. */
  setModified: (modified: boolean) => Promise<void>
  /** バッファだけのキーを割り当てます (Lisp の割り当てより先に使う). */
  localSetKey: (key: string, command: string) => void
  /** コマンドに割り当てたキー (Emacs の書き方. `C-x C-f` など). */
  whereIs: (command: string) => string[]
  /** 画面 (TypeScript) で書いたコマンドの名前. */
  editorCommandNames: () => string[]
  /** ミニバッファで, 文字を入力してもらいます (C-g でやめたら, 失敗). */
  readString: (prompt: string, initial: string) => Promise<string>
  /**
   * ミニバッファで, ファイルの名前を, Tab で補完しながら入力してもらいます (C-g でやめたら, 失敗).
   * 入力の始めは, ディレクトリ (空なら, 今のバッファのディレクトリ).
   */
  readFileName: (prompt: string, dir: string) => Promise<string>
  /** ミニバッファに問いを出して, キーを 1 つ押してもらいます (C-g でやめたら, 失敗). */
  readKey: (prompt: string) => Promise<string>
  /**
   * バッファ (省くと, 今のバッファ) を, ファイルに書いて, そのファイルのバッファにします. 書いたら, 終わります.
   * 書けなければ, 失敗します.
   */
  writeBuffer: (path: string, name?: string) => Promise<void>
  /** 今のバッファの本文を置き換えます (カーソルは同じ文字の所に残り, 元に戻せる). `modified` は置き換えた後の変更の有無. */
  setBufferText: (text: string, modified: boolean) => void
  /**
   * 今のバッファが dired なら, ディレクトリの一覧を作り直します. 作り直したら, 終わります.
   *
   * @returns dired なら `true`.
   */
  revertDired: () => Promise<boolean>
  /**
   * 今のバッファの状態 (Lisp の核に写す). 位置は UTF-16 の単位 (0 から).
   *
   * @returns バッファの名前, 本文, カーソル, マーク (無ければ `null`), ファイルのパス (無ければ `null`).
   */
  snapshot: () => {
    name: string
    text: string
    point: number
    mark: number | null
    file: string | null
  }
  /**
   * Lisp の核での変更を, 今のバッファに返します (本文が違えば置き換え, カーソルとマークを置く). 位置は UTF-16 の単位 (0 から).
   */
  applyBuffer: (text: string, point: number, mark: number | null) => void
  /** 背景の色を変えます. */
  setBackgroundColor: (color: string) => void
  /** 文字の色を変えます. */
  setForegroundColor: (color: string) => void
  /** フォントを変えます (`null` の項目は, 変えない. 大きさはポイント). */
  setFont: (family: string | null, size: number | null) => void
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
 * フック (`find-file-hook` など) を, 今のバッファで実行します.
 *
 * @param hook - フックの名前.
 * @returns 結果.
 */
export function runHooks(hook: string): Promise<EvalResponse> {
  return evalLisp(`(run-hooks '${hook})`)
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
 * 初期設定ファイル (`~/.rseditor`. 無ければ `~/.config/rseditor/init.scm`) を読んで, 評価します.
 *
 * @returns 結果.
 */
export function loadInitFile(): Promise<EvalResponse> {
  return invoke<EvalResponse>('load_init')
}

/**
 * UTF-16 の単位の位置 (エディタの位置) までの, 文字 (Unicode のコードポイント) の数を返します.
 * Lisp の位置と文字列の長さは, Emacs と同じく文字で数えるため, 絵文字などの BMP の外の文字を 1 つと数えます.
 *
 * @param text - 本文.
 * @param end - UTF-16 の単位の位置.
 * @returns 文字の数.
 */
export function charCount(text: string, end: number = text.length): number {
  let count = end
  for (let i = 1; i < end; i++) {
    if (isLowSurrogate(text.charCodeAt(i)) && isHighSurrogate(text.charCodeAt(i - 1))) count--
  }
  return count
}

/**
 * 文字 (Unicode のコードポイント) の数で数えた位置を, UTF-16 の単位の位置 (エディタの位置) にします.
 *
 * @param text - 本文.
 * @param chars - 文字の数で数えた位置 (本文の文字の数を超えない).
 * @returns UTF-16 の単位の位置.
 */
export function utf16Offset(text: string, chars: number): number {
  let i = 0
  for (let n = 0; n < chars; n++) {
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
 * 省ける, バッファの名前の引数を返します.
 *
 * @param args - 引数.
 * @returns バッファの名前. 省いたら `undefined`.
 */
function optionalName(args: unknown[]): string | undefined {
  return args.length > 0 ? String(args[0]) : undefined
}

/**
 * エディタの操作をします. Lisp との位置のやり取りは, 文字 (Unicode のコードポイント) で数え, 1 から数えます.
 * エディタの位置 (UTF-16 の単位) とは, ここで変えます.
 *
 * @param host - エディタ.
 * @param op - 操作の名前.
 * @param args - 引数.
 * @returns 結果 (値がなければ `null`). 画面が変わるのを待つ操作 (`find-file` など) は, 結果の `Promise`.
 * @throws 操作が失敗したとき.
 */
export function performEditorRequest(host: LispHost, op: string, args: unknown[]): unknown {
  const text = host.text()
  const length = charCount(text)
  const range = (): [number, number] => {
    const [a, b] = [offset(args[0], length), offset(args[1], length)]
    return [utf16Offset(text, Math.min(a, b)), utf16Offset(text, Math.max(a, b))]
  }
  switch (op) {
    case 'point':
      return charCount(text, host.point()) + 1
    case 'point-min':
      return 1
    case 'point-max':
      return length + 1
    case 'goto-char': {
      const pos = offset(args[0], length)
      host.setPoint(utf16Offset(text, pos))
      return pos + 1
    }
    case 'insert':
      host.insert(args.join(''))
      return null
    case 'delete-region':
      host.deleteRegion(...range())
      return null
    case 'buffer-string':
      return text
    case 'buffer-substring':
      return text.slice(...range())
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
    case 'pop-to-buffer-same-window':
      return host.switchToBuffer(String(args[0])).then(() => null)
    case 'buffer-file-name':
      return host.bufferFileName(optionalName(args))
    case 'buffer-modified-p':
      return host.bufferModified(optionalName(args))
    case 'buffer-list':
      return host.bufferList()
    case 'kill-buffer':
      return host.killBuffer(optionalName(args)).then(() => true)
    case 'kill-new':
      host.killNew(String(args[0]))
      return null
    case 'completing-read':
      return host.completingRead(
        String(args[0]),
        (args[1] as unknown[]).map(String),
        String(args[2] ?? ''),
      )
    case 'display-buffer':
      return host
        .displayBuffer(String(args[0]), typeof args[1] === 'number' ? args[1] : null)
        .then(() => null)
    case 'find-file-noselect':
      return host.findFileNoselect(String(args[0]))
    case 'highlight-regexp':
      host.highlightRegexp(
        String(args[0]),
        typeof args[1] === 'string' ? args[1] : null,
        typeof args[2] === 'number' ? args[2] : null,
      )
      return null
    case 'outline-flag-region':
      host.outlineFlagRegion(...range(), args[2] === true)
      return null
    case 'outline-invisible-p': {
      const pos = offset(args[0], length)
      return pos < length && host.invisible(utf16Offset(text, pos))
    }
    case 'unhighlight-regexp':
      host.unhighlightRegexp(args[0] === true ? true : String(args[0]))
      return null
    case 'set-buffer-modified-p':
      return host.setModified(args[0] === true).then(() => null)
    case 'local-set-key':
      host.localSetKey(String(args[0]), String(args[1]))
      return null
    case 'where-is-internal':
      return host.whereIs(String(args[0]))
    case 'editor-command-names':
      return host.editorCommandNames()
    case 'read-string':
      return host.readString(String(args[0]), String(args[1] ?? ''))
    case 'read-file-name':
      return host.readFileName(String(args[0]), String(args[1] ?? ''))
    case 'read-key':
      return host.readKey(String(args[0] ?? ''))
    case 'files--write-buffer':
      return host.writeBuffer(String(args[0]), optionalName(args.slice(1))).then(() => null)
    case 'files--set-buffer-text':
      host.setBufferText(String(args[0]), args[1] === true)
      return null
    case 'files--revert-dired':
      return host.revertDired()
    case 'editor-snapshot': {
      // Lisp の核に写す: 名前, 本文, カーソル, マーク (位置は文字で数え, 1 から), ファイル
      const snapshot = host.snapshot()
      const place = (pos: number | null) =>
        pos === null ? false : charCount(snapshot.text, pos) + 1
      return [
        snapshot.name,
        snapshot.text,
        place(snapshot.point),
        place(snapshot.mark),
        snapshot.file ?? false,
      ]
    }
    case 'editor-apply': {
      // 核での変更を返す: 本文, カーソル, マーク (位置は文字で数え, 1 から)
      const next = String(args[0])
      const nextLength = charCount(next)
      const place = (value: unknown) => utf16Offset(next, offset(value, nextLength))
      host.applyBuffer(next, place(args[1]), typeof args[2] === 'number' ? place(args[2]) : null)
      return null
    }
    case 'current-buffer':
      return host.bufferName()
    case 'set-buffer':
      // バッファの切り替えは、画面のウィンドウが受け持つ. 今のバッファへの切り替えだけ、受ける
      if (String(args[0]) !== host.bufferName()) {
        throw new Error('set-buffer to another buffer is not supported')
      }
      return host.bufferName()
    case 'set-background-color':
      host.setBackgroundColor(String(args[0]))
      return null
    case 'set-foreground-color':
      host.setForegroundColor(String(args[0]))
      return null
    case 'set-font':
      host.setFont(
        typeof args[0] === 'string' ? args[0] : null,
        typeof args[1] === 'number' ? args[1] : null,
      )
      return null
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
    // 結果が Promise の操作は, 終わるのを待って返事をする
    void new Promise((resolve) => resolve(performEditorRequest(host(), op, args))).then(
      (value) => invoke('editor_reply', { id, value, error: null }),
      (err) =>
        invoke('editor_reply', {
          id,
          value: null,
          error: err instanceof Error ? err.message : String(err),
        }),
    )
  })
}
