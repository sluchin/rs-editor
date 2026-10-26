import { describe, it, expect, vi, beforeEach } from 'vitest'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import {
  type LispHost,
  charCount,
  evalLisp,
  listenEditorRequests,
  loadInitFile,
  performEditorRequest,
  runLispCommand,
  utf16Offset,
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
  const state = {
    text,
    point: 0,
    message: '',
    calls: [] as string[],
    modified: true,
  }
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
    findFile: async (path) => {
      state.calls.push(`find-file ${path}`)
    },
    switchToBuffer: async (name) => {
      state.calls.push(`switch-to-buffer ${name}`)
    },
    bufferFileName: (name) => (name ? `/${name}` : null),
    bufferModified: (name) => name === 'mod',
    bufferList: () => ['*scratch*', 'a'],
    killBuffer: async (name) => {
      state.calls.push(`kill-buffer ${name}`)
    },
    killNew: (text) => {
      state.calls.push(`kill-new ${text}`)
    },
    completingRead: async (prompt, candidates, initial) =>
      `${prompt}${candidates.join(',')}${initial}`,
    displayBuffer: async (name, line) => {
      state.calls.push(`display-buffer ${name} ${line}`)
    },
    findFileNoselect: async (path) => `buffer of ${path}`,
    highlightRegexp: (regexp, face, group) => {
      state.calls.push(`highlight ${regexp} ${face} ${group}`)
    },
    outlineFlagRegion: (from, to, hide) => {
      state.calls.push(`outline ${from} ${to} ${hide}`)
    },
    invisible: (pos) => pos === 1,
    unhighlightRegexp: (regexp) => {
      state.calls.push(`unhighlight ${regexp}`)
    },
    setModified: (modified) => {
      state.modified = modified
    },
    localSetKey: (key, command) => {
      state.calls.push(`local-set-key ${key} ${command}`)
    },
    readString: async (prompt, initial) => `${prompt}${initial}`,
    setBackgroundColor: (color) => {
      state.calls.push(`background ${color}`)
    },
    setForegroundColor: (color) => {
      state.calls.push(`foreground ${color}`)
    },
    setFont: (family, size) => {
      state.calls.push(`font ${family} ${size}`)
    },
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

  it('位置は, 文字 (コードポイント) で数える (絵文字は 1 文字)', () => {
    // 😀 は UTF-16 で 2 つ. 日本語は 1 つ
    const { host, state } = fakeHost('a😀b日c')
    const run = (op: string, ...args: unknown[]) => performEditorRequest(host, op, args)
    expect(run('point-max')).toBe(6)
    expect(run('goto-char', 3)).toBe(3)
    expect(state.point).toBe(3)
    expect(run('point')).toBe(3)
    expect(run('buffer-substring', 2, 4)).toBe('😀b')
    expect(run('goto-char', 99)).toBe(6)
    expect(state.point).toBe(6)
    expect(run('delete-region', 2, 3)).toBeNull()
    expect(state.text).toBe('ab日c')
  })

  it('UTF-16 の位置と, 文字の数を変える (対になっていない半分は 1 文字)', () => {
    expect(charCount('a😀b')).toBe(3)
    expect(charCount('a😀b', 3)).toBe(2)
    expect(utf16Offset('a😀b', 2)).toBe(3)
    expect(utf16Offset('a😀b', 3)).toBe(4)
    // 対になっていない半分
    expect(charCount('\ud83d\ud83dx')).toBe(3)
    expect(charCount('\ude00\ude00')).toBe(2)
    expect(utf16Offset('\ud83dx', 2)).toBe(2)
  })

  it('ファイルとバッファを出し, 入力を受け, 色とフォントを変える', async () => {
    const { host, state } = fakeHost('')
    const run = (op: string, ...args: unknown[]) => performEditorRequest(host, op, args)
    expect(await run('find-file', '/a.txt')).toBeNull()
    expect(await run('switch-to-buffer', '*howm*')).toBeNull()
    expect(run('buffer-file-name')).toBeNull()
    expect(run('set-buffer-modified-p', false)).toBeNull()
    expect(state.modified).toBe(false)
    expect(run('set-buffer-modified-p', true)).toBeNull()
    expect(state.modified).toBe(true)
    expect(run('local-set-key', 'c', 'howm-create')).toBeNull()
    expect(await run('read-string', 'Search: ', 'x')).toBe('Search: x')
    expect(await run('read-string', 'Name: ')).toBe('Name: ')
    expect(run('set-background-color', 'black')).toBeNull()
    expect(run('set-foreground-color', 'white')).toBeNull()
    expect(run('set-font', 'Mono', 14)).toBeNull()
    expect(run('set-font', null, null)).toBeNull()
    expect(state.calls).toEqual([
      'find-file /a.txt',
      'switch-to-buffer *howm*',
      'local-set-key c howm-create',
      'background black',
      'foreground white',
      'font Mono 14',
      'font null null',
    ])
  })

  it('バッファの一覧, 消す, kill ring, 補完, 別のウィンドウ, 下線', async () => {
    const { host, state } = fakeHost('')
    const run = (op: string, ...args: unknown[]) => performEditorRequest(host, op, args)
    expect(run('buffer-file-name', 'a')).toBe('/a')
    expect(run('buffer-modified-p')).toBe(false)
    expect(run('buffer-modified-p', 'mod')).toBe(true)
    expect(run('buffer-list')).toEqual(['*scratch*', 'a'])
    expect(await run('kill-buffer', 'a')).toBe(true)
    expect(await run('kill-buffer')).toBe(true)
    expect(run('kill-new', 'k')).toBeNull()
    expect(await run('completing-read', 'P: ', ['a', 'b'], 'i')).toBe('P: a,bi')
    expect(await run('completing-read', 'P: ', [])).toBe('P: ')
    expect(await run('display-buffer', 'a', 3)).toBeNull()
    expect(await run('display-buffer', 'a')).toBeNull()
    expect(await run('find-file-noselect', '/x')).toBe('buffer of /x')
    expect(run('highlight-regexp', 'a+')).toBeNull()
    expect(run('highlight-regexp', 'a(b)', 'org-todo', 1)).toBeNull()
    expect(run('unhighlight-regexp', 'a+')).toBeNull()
    expect(run('unhighlight-regexp', true)).toBeNull()
    expect(state.calls).toEqual([
      'kill-buffer a',
      'kill-buffer undefined',
      'kill-new k',
      'display-buffer a 3',
      'display-buffer a null',
      'highlight a+ null null',
      'highlight a(b) org-todo 1',
      'unhighlight a+',
      'unhighlight true',
    ])
  })

  it('範囲を隠し, 隠したかを返す (位置は文字で数える)', () => {
    const { host, state } = fakeHost('a😀bc')
    const run = (op: string, ...args: unknown[]) => performEditorRequest(host, op, args)
    expect(run('outline-flag-region', 3, 2, true)).toBeNull()
    expect(run('outline-flag-region', 1, 5, 'x')).toBeNull()
    expect(state.calls).toEqual(['outline 1 3 true', 'outline 0 5 false'])
    expect(run('outline-invisible-p', 2)).toBe(true)
    expect(run('outline-invisible-p', 1)).toBe(false)
    // 終わりの位置 (文字が無い) は, 隠していない
    expect(run('outline-invisible-p', 5)).toBe(false)
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
    await vi.waitFor(() =>
      expect(mockedInvoke).toHaveBeenLastCalledWith('editor_reply', {
        id: 1,
        value: 4,
        error: null,
      }),
    )
    send(2, 'fly')
    await vi.waitFor(() =>
      expect(mockedInvoke).toHaveBeenLastCalledWith('editor_reply', {
        id: 2,
        value: null,
        error: 'Unknown editor operation: fly',
      }),
    )
    // 終わるのを待つ操作は, 終わってから返事する
    send(4, 'read-string', ['Name: ', 'a'])
    await vi.waitFor(() =>
      expect(mockedInvoke).toHaveBeenLastCalledWith('editor_reply', {
        id: 4,
        value: 'Name: a',
        error: null,
      }),
    )
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
    await vi.waitFor(() =>
      expect(mockedInvoke).toHaveBeenLastCalledWith('editor_reply', {
        id: 3,
        value: null,
        error: 'broken',
      }),
    )
  })
})
