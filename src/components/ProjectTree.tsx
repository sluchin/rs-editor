/**
 * サイドバーの, プロジェクトのファイルのツリー (VS Code のエクスプローラーのように, 左に出す).
 *
 * ディレクトリは, 開いたときに読みます (開くたびに読み直す). `.git` は出しません.
 * 右の端を引くか, `{` と `}` で, 幅を変えます. `a` (ファイル), `A` (ディレクトリ), `r`, `d` で, 作る, 名前を変える, 消す.
 */

import { useCallback, useEffect, useRef, useState } from 'react'
import '../styles/ProjectTree.css'

/** 出さない名前. */
const HIDDEN_NAMES = ['.git/']

/** 幅の初めの値と, 小さい方と大きい方の限り, `{` と `}` で変える量 (px). */
const DEFAULT_WIDTH = 240
const MIN_WIDTH = 120
const MAX_WIDTH = 600
const WIDTH_STEP = 20

/** 名前を聞いている (または, 消すかを確かめている) とき. */
interface Prompt {
  /** 作るファイル, 作るディレクトリ, 名前の変更, 削除. */
  kind: 'file' | 'directory' | 'rename' | 'delete'
  /** 作る先のディレクトリ (作るとき), 対象のパス (名前の変更と削除). */
  path: string
  /** 入力している名前. */
  text: string
}

/** プロジェクトのツリーの引数. */
interface ProjectTreeProps {
  /** プロジェクトの根 (後ろに `/` は付けない). */
  root: string
  /** 選んでいるウィンドウのバッファのファイル (強調する). */
  currentFile: string | null
  /** ディレクトリの中の名前を返す関数 (ディレクトリには, 後ろに `/` が付く). */
  listDirectory: (path: string) => Promise<string[]>
  /** ファイルを選んだときに呼びます. */
  onOpenFile: (path: string) => void
  /** ファイルかディレクトリを作る (同じ名前があれば失敗する). */
  createPath: (path: string, directory: boolean) => Promise<void>
  /** 名前を変える (変え先に同じ名前があれば失敗する). */
  renamePath: (from: string, to: string) => Promise<void>
  /** ファイルか, ディレクトリ (中身ごと) を消す. */
  deletePath: (path: string) => Promise<void>
  /** 増えるたびに, キーで動かすために, フォーカスをツリーへ移す. */
  focusSignal?: number
}

/** 見えている行 (キーで動かす). */
interface VisibleRow {
  /** パス. */
  path: string
  /** ディレクトリか. */
  isDirectory: boolean
  /** 1 つ上のディレクトリ. */
  parent: string
}

/**
 * ディレクトリと名前をつなげます.
 *
 * @param dir - ディレクトリ (`/` で終わってもよい).
 * @param name - 名前.
 * @returns パス.
 */
function joinPath(dir: string, name: string): string {
  return dir.endsWith('/') ? dir + name : `${dir}/${name}`
}

/**
 * パスの 1 つ上のディレクトリを返します (`/` の直下なら `/`).
 *
 * @param path - パス.
 * @returns 親のディレクトリ.
 */
function parentOf(path: string): string {
  return path.slice(0, path.lastIndexOf('/')) || '/'
}

/**
 * パスの最後の名前を返します.
 *
 * @param path - パス.
 * @returns 名前 (`/` だけなら `/`).
 */
function baseName(path: string): string {
  return path.split('/').filter(Boolean).pop() ?? '/'
}

/**
 * プロジェクトのツリー.
 *
 * @param props - 引数.
 * @returns 画面.
 */
export default function ProjectTree({
  root,
  currentFile,
  listDirectory,
  onOpenFile,
  createPath,
  renamePath,
  deletePath,
  focusSignal = 0,
}: ProjectTreeProps) {
  // 読んだディレクトリの中の名前 (読めなければ理由)
  const [entries, setEntries] = useState<Record<string, string[] | { error: string }>>({})
  // 開いているディレクトリ (根は, いつも開く)
  const [expanded, setExpanded] = useState<Set<string>>(new Set())
  // 幅 (px)
  const [width, setWidth] = useState(DEFAULT_WIDTH)
  // 名前を聞いているとき, 失敗の知らせ
  const [prompt, setPrompt] = useState<Prompt | null>(null)
  const [notice, setNotice] = useState('')

  /**
   * ディレクトリを読みます.
   *
   * @param dir - ディレクトリ.
   */
  const load = useCallback(
    async (dir: string) => {
      try {
        const names = (await listDirectory(joinPath(dir, ''))).filter(
          (n) => !HIDDEN_NAMES.includes(n),
        )
        setEntries((prev) => ({ ...prev, [dir]: names }))
      } catch (err) {
        setEntries((prev) => ({ ...prev, [dir]: { error: String(err) } }))
      }
    },
    [listDirectory],
  )

  // 根が変わったら, 読み直す
  useEffect(() => {
    setEntries({})
    setExpanded(new Set())
    void load(root)
  }, [root, load])

  // 今のファイルまでのディレクトリを開く (VS Code の Reveal. ファイルを切り替えるたびに行う)
  useEffect(() => {
    if (!currentFile || !currentFile.startsWith(joinPath(root, ''))) return
    const parts = currentFile.slice(joinPath(root, '').length).split('/')
    parts.pop()
    const dirs: string[] = []
    let dir = root
    for (const part of parts) {
      dir = joinPath(dir, part)
      dirs.push(dir)
    }
    if (dirs.length === 0) return
    setExpanded((prev) => new Set([...prev, ...dirs]))
    for (const d of dirs) void load(d)
  }, [root, currentFile, load])

  // 今のファイルが見える所へ動かす
  const body = useRef<HTMLDivElement>(null)
  useEffect(() => {
    body.current?.querySelector('.project-tree-current')?.scrollIntoView?.({ block: 'nearest' })
  }, [currentFile, entries, expanded])

  // キーで動かす行 (パス)
  const [cursor, setCursor] = useState<string | null>(null)
  const container = useRef<HTMLDivElement>(null)

  // 頼まれたら, フォーカスを移す (行は, 今のファイルから始める)
  useEffect(() => {
    if (focusSignal === 0) return
    container.current?.focus()
    setCursor((prev) => prev ?? currentFile)
  }, [focusSignal]) // eslint-disable-line react-hooks/exhaustive-deps

  /**
   * 見えている行を, 上から順に並べます.
   *
   * @param dir - ディレクトリ.
   * @returns 行.
   */
  const visibleRows = (dir: string): VisibleRow[] => {
    const names = entries[dir]
    if (!Array.isArray(names)) return []
    return names.flatMap((name) => {
      const isDirectory = name.endsWith('/')
      const path = joinPath(dir, isDirectory ? name.slice(0, -1) : name)
      const row = { path, isDirectory, parent: dir }
      return expanded.has(path) && isDirectory ? [row, ...visibleRows(path)] : [row]
    })
  }

  /**
   * ツリーにフォーカスがあるときのキー (n, p, RET, f, b, g, q).
   *
   * @param event - キー.
   */
  const handleKeyDown = (event: React.KeyboardEvent<HTMLDivElement>) => {
    // 名前を入力しているあいだは, 入力の欄が受ける
    if (event.target instanceof HTMLInputElement) return
    // エディタへ渡すキー (C-x など) は, ここで止めない
    const rows = visibleRows(root)
    const index = rows.findIndex((r) => r.path === cursor)
    const row = rows[index] as VisibleRow | undefined
    if (event.altKey || event.metaKey) return
    // 消すかの確かめ: `y` だけが消す. ほかのキーは, やめる
    if (prompt?.kind === 'delete') {
      if (event.key === 'y') void remove(prompt.path)
      setPrompt(null)
      event.preventDefault()
      return
    }
    setNotice('')
    const key = event.ctrlKey ? `C-${event.key}` : event.key
    const move = (to: number) => {
      if (rows.length > 0) setCursor(rows[Math.max(0, Math.min(rows.length - 1, to))].path)
    }
    switch (key) {
      case 'n':
      case 'C-n':
      case 'ArrowDown':
        move(index + 1)
        break
      case 'p':
      case 'C-p':
      case 'ArrowUp':
        move(index < 0 ? rows.length : index - 1)
        break
      case '<':
      case 'Home':
        move(0)
        break
      case '>':
      case 'End':
        move(rows.length)
        break
      case 'Enter':
      case ' ':
        if (!row) break
        if (row.isDirectory) toggle(row.path)
        else {
          onOpenFile(row.path)
          container.current?.blur()
        }
        break
      case 'f':
      case 'ArrowRight':
        if (!row?.isDirectory) break
        if (!expanded.has(row.path)) toggle(row.path)
        else move(index + 1)
        break
      case 'b':
      case 'ArrowLeft':
        if (!row) break
        if (row.isDirectory && expanded.has(row.path)) toggle(row.path)
        else if (row.parent !== root) setCursor(row.parent)
        break
      case 'g':
        refresh()
        break
      case 'a':
      case 'A':
        startCreate(key === 'A', row)
        break
      case 'r':
        if (row) setPrompt({ kind: 'rename', path: row.path, text: baseName(row.path) })
        break
      case 'd':
        if (row) setPrompt({ kind: 'delete', path: row.path, text: '' })
        break
      case '{':
        resize(width - WIDTH_STEP)
        break
      case '}':
        resize(width + WIDTH_STEP)
        break
      case 'q':
      case 'Escape':
      case 'C-g':
        container.current?.blur()
        break
      default:
        return
    }
    event.preventDefault()
  }

  /**
   * 幅を変えます (限りの中で).
   *
   * @param next - 新しい幅 (px).
   */
  const resize = (next: number) => setWidth(Math.max(MIN_WIDTH, Math.min(MAX_WIDTH, next)))

  /**
   * 右の端を引いて, 幅を変えます.
   *
   * @param event - 押したマウス.
   */
  const startResize = (event: React.MouseEvent) => {
    event.preventDefault()
    const startX = event.clientX
    const startWidth = width
    const move = (e: MouseEvent) => resize(startWidth + e.clientX - startX)
    const stop = () => {
      document.removeEventListener('mousemove', move)
      document.removeEventListener('mouseup', stop)
    }
    document.addEventListener('mousemove', move)
    document.addEventListener('mouseup', stop)
  }

  /**
   * 作る名前を聞き始めます. 作る先は, 行がディレクトリならその中, ファイルならその親, 行が無ければ根.
   *
   * @param directory - ディレクトリを作るなら真.
   * @param row - カーソルの行.
   */
  const startCreate = (directory: boolean, row: VisibleRow | undefined) => {
    const dir = row ? (row.isDirectory ? row.path : row.parent) : root
    setNotice('')
    setPrompt({ kind: directory ? 'directory' : 'file', path: dir, text: '' })
  }

  /**
   * 失敗を知らせに出して, フォーカスをツリーへ戻します.
   *
   * @param action - 頼むこと.
   */
  const attempt = async (action: () => Promise<void>) => {
    try {
      await action()
    } catch (err) {
      setNotice(String(err))
    }
    container.current?.focus()
  }

  /**
   * 聞いた名前で, 作るか, 名前を変えます.
   */
  const submit = (prompt: Prompt) => {
    const name = prompt.text.trim()
    setPrompt(null)
    if (name === '' || name.includes('/')) {
      setNotice(name === '' ? '' : `Invalid name: ${name}`)
      container.current?.focus()
      return
    }
    void attempt(async () => {
      if (prompt.kind === 'rename') {
        const parent = parentOf(prompt.path)
        const to = joinPath(parent, name)
        await renamePath(prompt.path, to)
        await load(parent)
        setCursor(to)
        return
      }
      const path = joinPath(prompt.path, name)
      await createPath(path, prompt.kind === 'directory')
      setExpanded((prev) => new Set([...prev, prompt.path]))
      await load(prompt.path)
      setCursor(path)
      if (prompt.kind === 'file') onOpenFile(path)
    })
  }

  /**
   * 消します.
   *
   * @param path - 消すパス.
   */
  const remove = (path: string) =>
    attempt(async () => {
      const parent = parentOf(path)
      await deletePath(path)
      await load(parent)
      setCursor(parent === root ? null : parent)
    })

  /**
   * ディレクトリを開くか, 閉じます. 開くときは, 読み直します.
   *
   * @param dir - ディレクトリ.
   */
  const toggle = (dir: string) => {
    const next = new Set(expanded)
    if (next.has(dir)) {
      next.delete(dir)
    } else {
      next.add(dir)
      void load(dir)
    }
    setExpanded(next)
  }

  /**
   * 根と, 開いているディレクトリを, 読み直します.
   */
  const refresh = () => {
    for (const dir of [root, ...expanded]) void load(dir)
  }

  /**
   * ディレクトリの中を描きます.
   *
   * @param dir - ディレクトリ.
   * @param depth - 深さ (字下げ).
   * @returns 画面.
   */
  const renderDirectory = (dir: string, depth: number) => {
    const names = entries[dir]
    if (!names) return null
    if (!Array.isArray(names)) {
      return (
        <div className="project-tree-error" style={{ paddingLeft: depth * 12 + 8 }}>
          {names.error}
        </div>
      )
    }
    return names.map((name) => {
      const isDirectory = name.endsWith('/')
      const path = joinPath(dir, isDirectory ? name.slice(0, -1) : name)
      const open = expanded.has(path)
      const classes = [
        'project-tree-item',
        isDirectory ? 'project-tree-directory' : 'project-tree-file',
      ]
      if (path === currentFile) classes.push('project-tree-current')
      if (path === cursor) classes.push('project-tree-cursor')
      return (
        <div key={path}>
          <div
            className={classes.join(' ')}
            style={{ paddingLeft: depth * 12 + 8 }}
            title={path}
            onClick={() => (isDirectory ? toggle(path) : onOpenFile(path))}
          >
            <span className="project-tree-twisty">{isDirectory ? (open ? '▾' : '▸') : ''}</span>
            {isDirectory ? name.slice(0, -1) : name}
          </div>
          {isDirectory && open && renderDirectory(path, depth + 1)}
        </div>
      )
    })
  }

  return (
    // 押してもフォーカスを取らない (キーは, エディタのまま)
    <div
      className="project-tree"
      aria-label="Project"
      tabIndex={-1}
      ref={container}
      style={{ flexBasis: width }}
      onKeyDown={handleKeyDown}
      onMouseDown={(e) => {
        if (!(e.target instanceof HTMLInputElement)) e.preventDefault()
      }}
    >
      <div className="project-tree-header">
        <span className="project-tree-title" title={root}>
          {baseName(root).toUpperCase()}
        </span>
        <span>
          <button
            className="project-tree-add"
            title="New File"
            onClick={() => startCreate(false, undefined)}
          >
            +
          </button>
          <button className="project-tree-refresh" title="Refresh" onClick={refresh}>
            ↻
          </button>
        </span>
      </div>
      <div className="project-tree-body" ref={body}>
        {renderDirectory(root, 0)}
      </div>
      {prompt?.kind === 'delete' && (
        <div className="project-tree-prompt">Delete {baseName(prompt.path)}? (y/n)</div>
      )}
      {prompt && prompt.kind !== 'delete' && (
        <label className="project-tree-prompt">
          {prompt.kind === 'rename'
            ? 'Rename to: '
            : prompt.kind === 'directory'
              ? 'New directory: '
              : 'New file: '}
          <input
            autoFocus
            value={prompt.text}
            onChange={(e) => setPrompt({ ...prompt, text: e.target.value })}
            onKeyDown={(e) => {
              if (e.key === 'Enter') submit(prompt)
              else if (e.key === 'Escape' || (e.ctrlKey && e.key === 'g')) {
                setPrompt(null)
                container.current?.focus()
              } else return
              e.preventDefault()
            }}
          />
        </label>
      )}
      {notice && <div className="project-tree-error">{notice}</div>}
      <div className="project-tree-resizer" onMouseDown={startResize} />
    </div>
  )
}
