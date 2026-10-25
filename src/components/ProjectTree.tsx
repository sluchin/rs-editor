/**
 * サイドバーの, プロジェクトのファイルのツリー (VS Code のエクスプローラーのように, 左に出す).
 *
 * ディレクトリは, 開いたときに読みます (開くたびに読み直す). `.git` は出しません.
 */

import { useCallback, useEffect, useState } from 'react'
import '../styles/ProjectTree.css'

/** 出さない名前. */
const HIDDEN_NAMES = ['.git/']

interface ProjectTreeProps {
  /** プロジェクトの根 (後ろに `/` は付けない). */
  root: string
  /** 選んでいるウィンドウのバッファのファイル (強調する). */
  currentFile: string | null
  /** ディレクトリの中の名前を返す関数 (ディレクトリには, 後ろに `/` が付く). */
  listDirectory: (path: string) => Promise<string[]>
  /** ファイルを選んだときに呼びます. */
  onOpenFile: (path: string) => void
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
}: ProjectTreeProps) {
  // 読んだディレクトリの中の名前 (読めなければ, 理由)
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
    <div className="project-tree" aria-label="Project" onMouseDown={(e) => e.preventDefault()}>
      <div className="project-tree-header">
        <span className="project-tree-title" title={root}>
          {baseName(root).toUpperCase()}
        </span>
        <button className="project-tree-refresh" title="Refresh" onClick={refresh}>
          ↻
        </button>
      </div>
      <div className="project-tree-body">{renderDirectory(root, 0)}</div>
    </div>
  )
}
