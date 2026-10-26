/// <reference types="vitest/config" />
/**
 * Vite (開発サーバーとビルド) と Vitest (テスト) の設定.
 */
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  // src を `@/` で参照する (テストから `@/lib/...` のように使う)
  resolve: {
    alias: { '@': '/src' },
  },
  // エディタ (CodeMirror) で, 1 つのファイルが大きくなる (アプリの中から読むので, 分けない)
  build: { chunkSizeWarningLimit: 1000 },
  server: {
    port: 5173,
    strictPort: false,
  },
  test: {
    environment: 'jsdom',
    globals: true,
    setupFiles: './tests/frontend/setup.ts',
    // 画面のテストは tests/frontend/ に, src/ と同じ構成で置く (Lisp のテストは tests/lisp/)
    include: ['tests/frontend/**/*.{test,spec}.{ts,tsx}'],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'html'],
      // 測るのは src のソース (テスト, 型の宣言, 入口は除く)
      include: ['src/**/*.{ts,tsx}'],
      exclude: ['src/main.tsx', 'src/vite-env.d.ts'],
      // 行, 分岐, 関数, 文の全てを 100% に保つ (下回れば失敗する)
      thresholds: {
        lines: 100,
        branches: 100,
        functions: 100,
        statements: 100,
      },
    },
  },
})
