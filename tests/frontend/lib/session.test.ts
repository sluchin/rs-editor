import { describe, it, expect, vi, beforeEach } from 'vitest'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { listenFrames, listenRequests, sendInput, utf16Offset } from '@/lib/session'

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }))
vi.mock('@tauri-apps/api/event', () => ({ listen: vi.fn() }))

const mockedInvoke = vi.mocked(invoke)
const mockedListen = vi.mocked(listen)

/**
 * 受けたイベントの関数を返します.
 *
 * @param name - イベントの名前.
 * @returns 関数.
 */
const handlerOf = (name: string) =>
  mockedListen.mock.calls.find(([event]) => event === name)?.[1] as (event: {
    payload: unknown
  }) => void

beforeEach(() => {
  vi.clearAllMocks()
  mockedInvoke.mockResolvedValue(undefined)
  mockedListen.mockResolvedValue(() => {})
})

describe('session', () => {
  it('入力を session_input で送ること', async () => {
    await sendInput({ kind: 'key', key: 'C-f' })
    expect(mockedInvoke).toHaveBeenCalledWith('session_input', {
      input: { kind: 'key', key: 'C-f' },
    })
  })

  it('描く内容を受けること', async () => {
    const handler = vi.fn()
    await listenFrames(handler)
    const frame = { windows: [], selected: 1, buffers: [], message: 'hi' }
    handlerOf('frame')({ payload: frame })
    expect(handler).toHaveBeenCalledWith(frame)
  })

  it('頼みに答え, 失敗したら, 失敗の内容を返すこと', async () => {
    await listenRequests((op, args) => {
      if (op === 'fail') throw new Error('boom')
      if (op === 'throw-string') throw 'bad'
      if (op === 'nothing') return undefined
      return Promise.resolve([op, ...args])
    })
    const handler = handlerOf('editor-request')
    const replies = () => mockedInvoke.mock.calls.map(([, arg]) => arg)
    handler({ payload: { id: 1, op: 'ok', args: [2] } })
    handler({ payload: { id: 2, op: 'fail', args: [] } })
    handler({ payload: { id: 3, op: 'throw-string', args: [] } })
    handler({ payload: { id: 4, op: 'nothing', args: [] } })
    await vi.waitFor(() => expect(mockedInvoke).toHaveBeenCalledTimes(4))
    expect(replies()).toEqual(
      expect.arrayContaining([
        { id: 1, value: ['ok', 2], error: null },
        { id: 2, value: null, error: 'boom' },
        { id: 3, value: null, error: 'bad' },
        { id: 4, value: null, error: null },
      ]),
    )
  })

  it('文字の数の位置を, UTF-16 の位置にすること', () => {
    expect(utf16Offset('a😀b', 2)).toBe(3)
    expect(utf16Offset('a😀b', 9)).toBe(4)
    expect(utf16Offset('\ud800x', 1)).toBe(1)
  })
})
