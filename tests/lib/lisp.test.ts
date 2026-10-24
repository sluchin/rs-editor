import { describe, it, expect, vi, beforeEach } from 'vitest'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import {
  type LispHost,
  evalLisp,
  listenEditorRequests,
  loadInitFile,
  performEditorRequest,
  runLispCommand,
} from '@/lib/lisp'

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }))
vi.mock('@tauri-apps/api/event', () => ({ listen: vi.fn() }))

const mockedInvoke = vi.mocked(invoke)
const mockedListen = vi.mocked(listen)

/**
 * 偽のエディタを作ります.
 *
 * @param text - 本文.
 * @returns エディタと, 今の本文とカーソル.
 */
function fakeHost(text: string) {
  const state = { text, point: 0, message: '' }
  const host: LispHost = {
    text: () => state.text,
    point: () => state.point,
    setPoint: (pos) => {
      state.point = pos
    },
    insert: (s) => {
      state.text = state.text.slice(0, state.point) + s + state.text.slice(state.point)
      state.point += s.length
    },
    deleteRegion: (from, to) => {
      state.text = state.text.slice(0, from) + state.text.slice(to)
    },
    bufferName: () => '*scratch*',
    message: (s) => {
      state.message = s
    },
    command: (name) => name === 'forward-char',
  }
  return { host, state }
}

describe('lisp', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('評価, コマンド, 初期設定ファイルを, バックエンドに頼む', async () => {
    mockedInvoke.mockResolvedValue({ result: '1', error: null, commands: [], keys: [] })
    await evalLisp('1')
    expect(mockedInvoke).toHaveBeenCalledWith('eval_scheme', { code: '1' })
    await runLispCommand('hi')
    expect(mockedInvoke).toHaveBeenCalledWith('run_lisp_command', { name: 'hi' })
    await loadInitFile()
    expect(mockedInvoke).toHaveBeenCalledWith('load_init')
  })

  it('エディタを, 1 から数えた位置で操作する', () => {
    const { host, state } = fakeHost('hello')
    const run = (op: string, ...args: unknown[]) => performEditorRequest(host, op, args)
    expect(run('point')).toBe(1)
    expect(run('point-min')).toBe(1)
    expect(run('point-max')).toBe(6)
    // 範囲の外は, 端へ寄せる
    expect(run('goto-char', 99)).toBe(6)
    expect(run('goto-char', -5)).toBe(1)
    expect(run('goto-char', 6.7)).toBe(6)
    expect(run('insert', ' world', '!')).toBeNull()
    expect(state.text).toBe('hello world!')
    expect(run('buffer-substring', 6, 1)).toBe('hello')
    expect(run('delete-region', 13, 6)).toBeNull()
    expect(run('buffer-string')).toBe('hello')
    expect(run('buffer-name')).toBe('*scratch*')
    expect(run('message', 'hi')).toBeNull()
    expect(state.message).toBe('hi')
    expect(run('command', 'forward-char')).toBe(true)
    expect(() => run('command', 'none')).toThrow('Undefined command: none')
    expect(() => run('goto-char', 'a')).toThrow('Expected a position: a')
    expect(() => run('goto-char', NaN)).toThrow('Expected a position: NaN')
    expect(() => run('fly')).toThrow('Unknown editor operation: fly')
  })

  it('頼みを受けて, 結果か失敗の内容を返事する', async () => {
    const unlisten = vi.fn()
    mockedListen.mockResolvedValue(unlisten)
    mockedInvoke.mockResolvedValue(undefined)
    const { host } = fakeHost('abc')
    expect(await listenEditorRequests(() => host)).toBe(unlisten)
    expect(mockedListen).toHaveBeenCalledWith('editor-request', expect.any(Function))
    const handler = mockedListen.mock.calls[0][1]
    const send = (id: number, op: string, args: unknown[] = []) =>
      handler({ event: 'editor-request', id: 0, payload: { id, op, args } })

    send(1, 'point-max')
    expect(mockedInvoke).toHaveBeenLastCalledWith('editor_reply', {
      id: 1,
      value: 4,
      error: null,
    })
    send(2, 'fly')
    expect(mockedInvoke).toHaveBeenLastCalledWith('editor_reply', {
      id: 2,
      value: null,
      error: 'Unknown editor operation: fly',
    })
    // Error でない失敗も, 文字にする
    const broken = {
      ...host,
      text: () => {
        throw 'broken'
      },
    }
    mockedListen.mockClear()
    await listenEditorRequests(() => broken)
    mockedListen.mock.calls[0][1]({
      event: 'editor-request',
      id: 0,
      payload: { id: 3, op: 'point', args: [] },
    })
    expect(mockedInvoke).toHaveBeenLastCalledWith('editor_reply', {
      id: 3,
      value: null,
      error: 'broken',
    })
  })
})
