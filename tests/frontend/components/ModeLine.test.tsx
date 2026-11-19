import { describe, it, expect } from 'vitest'
import { render } from '@testing-library/react'
import type { CursorInfo } from '@/components/ModeLine'
import ModeLine from '@/components/ModeLine'

/**
 * ModeLine コンポーネント のテストスイート.
 * 行番号とファイル名を表示するモードラインをテストします.
 */
describe('ModeLine Component', () => {
  it('renders with cursor info', () => {
    // カーソル情報を含めてレンダリングされることを確認.
    const cursorInfo: CursorInfo = { line: 5, column: 10 }
    const { container } = render(
      <ModeLine bufferName="test.scm" modified={false} cursor={cursorInfo} />,
    )

    expect(container.textContent).toContain('L5')
  })

  it('displays filename when provided', () => {
    // ファイル名が表示されることを確認.
    const cursorInfo: CursorInfo = { line: 1, column: 1 }
    const { container } = render(
      <ModeLine bufferName="test.scm" modified={false} cursor={cursorInfo} />,
    )

    expect(container.textContent).toContain('test.scm')
  })

  it('shows the line and the column only when their modes are on', () => {
    const cursorInfo: CursorInfo = { line: 3, column: 4 }
    const { container, rerender } = render(
      <ModeLine bufferName="m" modified={false} cursor={cursorInfo} />,
    )
    expect(container.textContent).toContain('All L3 C4 (')
    rerender(<ModeLine bufferName="m" modified={false} cursor={cursorInfo} columnNumber={false} />)
    expect(container.textContent).toContain('All L3 (')
    rerender(<ModeLine bufferName="m" modified={false} cursor={cursorInfo} lineNumber={false} />)
    expect(container.textContent).toContain('All C4 (')
    rerender(
      <ModeLine
        bufferName="m"
        modified={false}
        cursor={cursorInfo}
        lineNumber={false}
        columnNumber={false}
      />,
    )
    expect(container.textContent).toContain('All (')
  })

  it('displays the read-only indicator', () => {
    const cursorInfo: CursorInfo = { line: 1, column: 0 }
    const { container, rerender } = render(
      <ModeLine bufferName="r" modified={false} readOnly cursor={cursorInfo} />,
    )
    expect(container.textContent).toContain('-:%%- r')
    rerender(<ModeLine bufferName="r" modified readOnly cursor={cursorInfo} />)
    expect(container.textContent).toContain('-:%*- r')
  })

  it('displays modified indicator', () => {
    // 変更されたことを示すインジケーター (**) が表示されることを確認.
    const cursorInfo: CursorInfo = { line: 1, column: 1 }
    const { container } = render(
      <ModeLine bufferName="test.scm" modified={true} cursor={cursorInfo} />,
    )

    expect(container.textContent).toContain('**')
  })
})

describe('ModeLine Component (バッファの数)', () => {
  it('バッファが複数なら, 何番目かを表示する', () => {
    const { container } = render(
      <ModeLine
        bufferName="a.scm"
        modified={false}
        cursor={{ line: 1, column: 0 }}
        bufferCount={3}
        currentBufferIndex={2}
      />,
    )
    expect(container.textContent).toContain('[2/3]')
  })
})
