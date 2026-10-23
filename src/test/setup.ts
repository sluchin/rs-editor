import '@testing-library/jest-dom/vitest'

// jsdom の Range には, 位置を測る関数が無い (CodeMirror が, 画面を動かすときに使う)
Range.prototype.getBoundingClientRect = () => new DOMRect()
Range.prototype.getClientRects = () =>
  ({
    length: 0,
    item: () => null,
    [Symbol.iterator]: [][Symbol.iterator],
  }) as unknown as DOMRectList
