import { describe, it, expect, vi, afterEach } from 'vitest'
import {
  isDarkColor,
  parseColor,
  setBackgroundColor,
  setFont,
  setForegroundColor,
} from '@/lib/theme'

describe('theme', () => {
  afterEach(() => {
    vi.restoreAllMocks()
    document.documentElement.removeAttribute('style')
    delete document.documentElement.dataset.backgroundMode
  })

  it('色を, 赤, 緑, 青にする', () => {
    expect(parseColor('#fff')).toEqual([255, 255, 255])
    expect(parseColor('#1E1e1e')).toEqual([30, 30, 30])
    expect(parseColor('rgb(1, 2, 3)')).toEqual([1, 2, 3])
    expect(parseColor('rgba(4,5,6,0.5)')).toEqual([4, 5, 6])
    expect(parseColor(' Black ')).toEqual([0, 0, 0])
    expect(parseColor('#12')).toBeNull()
    expect(parseColor('nocolor')).toBeNull()
  })

  it('表にない色の名前は, 画面に計算させる', () => {
    vi.spyOn(window, 'getComputedStyle').mockReturnValue({
      color: 'rgb(255, 0, 0)',
    } as CSSStyleDeclaration)
    expect(parseColor('red')).toEqual([255, 0, 0])
    vi.spyOn(window, 'getComputedStyle').mockReturnValue({ color: 'red' } as CSSStyleDeclaration)
    expect(parseColor('red')).toBeNull()
    // 計算に使った要素は, 残さない
    expect(document.body.children).toHaveLength(0)
  })

  it('背景が暗いかを, Emacs と同じく, 明るさで決める', () => {
    expect(isDarkColor('black')).toBe(true)
    expect(isDarkColor('#989898')).toBe(true)
    expect(isDarkColor('#999999')).toBe(false)
    expect(isDarkColor('white')).toBe(false)
    expect(isDarkColor('nocolor')).toBeNull()
  })

  it('色とフォントを, CSS の変数に置く', () => {
    const root = document.documentElement
    setBackgroundColor(root, 'black')
    expect(root.style.getPropertyValue('--background')).toBe('black')
    expect(root.dataset.backgroundMode).toBe('dark')
    setBackgroundColor(root, '#ffffff')
    expect(root.dataset.backgroundMode).toBe('light')
    expect(() => setBackgroundColor(root, 'nocolor')).toThrow('Undefined color: nocolor')
    setForegroundColor(root, 'white')
    expect(root.style.getPropertyValue('--foreground')).toBe('white')
    expect(() => setForegroundColor(root, 'nocolor')).toThrow('Undefined color: nocolor')
    setFont(root, 'DejaVu Sans Mono', 14)
    expect(root.style.getPropertyValue('--font-family')).toBe('"DejaVu Sans Mono", monospace')
    expect(root.style.getPropertyValue('--font-size')).toBe('14pt')
    // null の項目は, 変えない
    setFont(root, null, null)
    expect(root.style.getPropertyValue('--font-family')).toBe('"DejaVu Sans Mono", monospace')
    expect(root.style.getPropertyValue('--font-size')).toBe('14pt')
    expect(() => setFont(root, null, 0)).toThrow('Invalid font size: 0')
  })
})
