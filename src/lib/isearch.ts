export interface IsearchMatch {
  start: number
  end: number
}

export function findNextMatch(
  text: string,
  query: string,
  fromPos: number,
  direction: 'forward' | 'backward',
): IsearchMatch | null {
  if (query === '') return null

  if (direction === 'forward') {
    const idx = text.indexOf(query, fromPos)
    return idx === -1 ? null : { start: idx, end: idx + query.length }
  }

  const idx = text.lastIndexOf(query, Math.max(0, fromPos))
  return idx === -1 ? null : { start: idx, end: idx + query.length }
}
