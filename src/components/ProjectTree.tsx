/**
 * サイドバーの, プロジェクトのファイルのツリー (VS Code のエクスプローラーのように, 左に出す).
 *
 * ディレクトリは, 開いたときに読みます (開くたびに読み直す). `.git` は出しません.
 */

import { useCallback, useEffect, useRef, useState } from 'react'
import '../styles/ProjectTree.css'

/** 出さない名前. */
const HIDDEN_NAMES = ['.git/']

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
  focusSignal = 0,
}: ProjectTreeProps) {
  // 読んだディレクトリの中の名前 (読めなければ理由)
  const [entries, setEntries] = useState<Record<string, string[] | { error: string }>>({})
  // 開いているディレクトリ (根は, いつも開く)
  const [expanded, setExpanded] = useState<Set<string>>(new Set())

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
    // エディタへ渡すキー (C-x など) は, ここで止めない
    const rows = visibleRows(root)
    const index = rows.findIndex((r) => r.path === cursor)
    const row = rows[index] as VisibleRow | undefined
    if (event.altKey || event.metaKey) return
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
      onKeyDown={handleKeyDown}
      onMouseDown={(e) => e.preventDefault()}
    >
      <div className="project-tree-header">
        <span className="project-tree-title" title={root}>
          {baseName(root).toUpperCase()}
        </span>
        <button className="project-tree-refresh" title="Refresh" onClick={refresh}>
          ↻
        </button>
      </div>
      <div className="project-tree-body" ref={body}>
        {renderDirectory(root, 0)}
      </div>
    </div>
  )
}
