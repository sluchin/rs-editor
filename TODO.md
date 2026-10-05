# TODO: Emacs 相当機能の実装手順

このファイルは、rseditor を Emacs ライクなエディタに近づけるための実装ステップをまとめたもの。
`.claude/CLAUDE.md` に記載の TODO（演算子追加、制御構文など Scheme インタプリタ側）とは別に、
**エディタ UI / キーバインド側**の作業をフェーズ分けして記載する。

## 現状整理

- `src/lib/prefixKeymap.ts` … `C-x C-f` / `C-x C-s` のみ実装。`CodeEditor.tsx` から実際に呼ばれている。
- `src/lib/emacsKeymap.ts` … `C-f/b/n/p/a/e/d/k/y/w`, `M-f/b/w`, `C-space`（mark）, `C-g` を実装済みだが
  **`CodeEditor.tsx` に未接続**。また `C-x C-s` を独自に処理しており `prefixKeymap.ts` と役割が重複している。
- `src/lib/bracketAutoClose.ts` … 括弧・文字列の自動補完/skip-over/ペア削除を実装済みだが未接続。
- `src/lib/bracketMatch.ts` / `src/lib/schemeTokenizer.ts` … 括弧対応検出・字句解析を実装済みだが
  どこからも呼ばれておらず、シンタックスハイライトやハイライト表示に未使用。
- `src/components/REPL.tsx` … 実装済みだが `App.tsx` からマウントされていない（コメントで「currently unused」）。
- ミニバッファは `find-file` / `write-file` の2種類の `input` モードのみ対応。`M-x` のようなコマンド実行や
  `isearch` は未対応。

これを踏まえ、まず「実装済みだが配線されていないコード」を統合するところから着手し、
その後に新規機能を積み上げる。

---

## Phase 1: 既存キーマップの統合と配線

- [x] `prefixKeymap.ts` と `emacsKeymap.ts` の `C-x` プレフィックス処理が重複しているため統合する。
      方針: `emacsKeymap.ts` を「単一キー・M- キーバインド」専用にし、`C-x` 系プレフィックスは
      `prefixKeymap.ts` に一本化する（`onFindFile` / `onSaveBuffer` に加えて、Phase 4 で追加する
      `C-x b` / `C-x k` / `C-x 2` などもここに集約）。
- [x] `CodeEditor.tsx` に `useEmacsKeymap` を接続し、`onKeyDown` で
      `prefixKeyDown(e) || bracketAutoCloseKeyDown(e) || emacsKeyDown(e)` の優先順位で処理する。
      （プレフィックスキー処理 → 括弧自動補完 → Emacs カーソル/編集コマンド、の順で `preventDefault` の競合を避ける）
- [x] `useBracketAutoClose` を `CodeEditor.tsx` に接続する。
- [x] `App.tsx` の `handleEditorChange` は `(value: string) => void` だが、`useEmacsKeymap` /
      `useBracketAutoClose` はカーソル位置も一緒に返す `(value, cursorPos) => void` を要求しているため、
      `CodeEditor` 側でカーソル位置を `setSelectionRange` してから `onChange` を呼ぶよう調整する
      （React の再レンダー後に `selectionStart` が飛ばないよう `useLayoutEffect` 等で反映）。
- [x] 上記3ファイルの単体テストを追加・拡充する（`bracketMatch.test.ts` / `schemeTokenizer.test.ts` に倣う）。

## Phase 2: シンタックスハイライト

- [x] `CodeEditor.tsx` を「透明な `<textarea>` を実入力に使い、背後に色付き `<pre>` を重ねる」構成に変更する
      （スクロール位置を `onScroll` で同期）。既存の `bracketAutoClose` / `emacsKeymap` はそのまま `<textarea>` に効かせる。
- [x] `schemeTokenizer.tokenize()` の結果を使い、トークン種別ごとに `<span>` でラップして色付けする
      （`keyword` / `string` / `comment` / `number` / `boolean` / `paren` / `symbol`）。
- [x] `bracketMatch.findMatchingBracket()` を使い、カーソルに隣接する括弧とその対応括弧をハイライトする
      （Emacs の `show-paren-mode` 相当）。
- [x] `bracketMatch.findUnmatchedClose()` を使い、閉じ括弧の対応が取れていない箇所をエラー表示する。
- [x] パフォーマンス確認: 大きめのバッファでも `tokenize()` が入力毎に重くならないか（必要なら debounce）。

## Phase 3: ミニバッファ / コマンド実行の拡張

- [ ] `Minibuffer.tsx` の `MinibufferState` に `mode: 'isearch'` を追加し、`C-s` / `C-r` によるインクリメンタル検索を実装する。
      - マッチ箇所へのカーソル移動、`C-s` 連打で次のマッチへ、`C-g` でキャンセルして元の位置に戻る。
- [ ] `M-x` (execute-extended-command) を実装する。
      - コマンド名を文字列で受け取り、内部の「コマンドレジストリ」（`{name, run}` のマップ）から実行する。
      - 最初は `find-file` / `save-buffer` / `save-buffers-kill-terminal` 程度から始め、Phase 4/5 のバッファ/REPL 系コマンドを順次登録する。
- [ ] `M-x` の補完（前方一致で候補を絞り込み、Tab で補完）はあると Emacs らしさが出るが優先度は低め。
- [ ] `keyboard-quit` (`C-g`) 実行時に `Quit` メッセージをミニバッファに表示する（現状 `emacsKeymap.ts` は mark を消すのみ）。

## Phase 4: 編集コマンドの強化

- [x] Undo / Redo: `C-/` (undo), `C-x C-/`または `M-_` 相当の redo。
      `<textarea>` のネイティブ undo に頼らず、独自の undo スタックを持たせるか検討する
      （kill-ring や mark と同様、バッファごとの状態として `App.tsx` か新規 hook で管理）。
- [x] Kill-ring の多段化: 現在 `emacsKeymap.ts` の `killRing` は単一の `useRef<string>`。
      Emacs の `M-y` (yank-pop) を実装するには配列化してリングにする必要がある。
- [x] `C-t` (transpose-chars) を追加する。
- [x] `M-<` / `M->` (buffer-start / buffer-end) を追加する。
- [x] `C-x h` (mark-whole-buffer) を追加する。
- [x] Scheme 向けの構造編集コマンド（あると差別化になる）:
      - `C-M-f` / `C-M-b` (forward-sexp / backward-sexp) … `bracketMatch` のロジックを流用して S 式単位の移動を実装。
      - `C-M-k` (kill-sexp)

## Phase 5: バッファ / ウィンドウ管理

- [x] 複数バッファをサポートする。`App.tsx` の `code` / `filePath` / `modified` を単一バッファの状態として
      `{ id, name, filePath, content, modified }[]` の配列に置き換える。
- [x] `C-x b` (switch-to-buffer): ミニバッファでバッファ名を入力して切り替え。
- [x] `C-x k` (kill-buffer): 現在のバッファを閉じる（未保存なら確認を挟む）。
- [x] `C-x C-b` (list-buffers): バッファ一覧を表示するビュー。
- [ ] ウィンドウ分割（優先度は低いが Emacs らしさの要）: `C-x 2` (縦分割) / `C-x 3` (横分割) / `C-x o` (他ウィンドウへ移動) / `C-x 0` / `C-x 1`。
      React 側は「ペインのレイアウトツリー」を持たせ、各ペインに独立した `CodeEditor` インスタンス（同一バッファ参照可）を描画する。

## Phase 6: REPL / Scheme 統合

- [x] `REPL.tsx` を `App.tsx` にマウントする（下ペイン、もしくは `C-x C-e` 実行結果をミニバッファ/エコーエリアに出す方式でも可）。
- [x] `C-x C-e` (eval-last-sexp): カーソル直前の S 式を `bracketMatch` で特定し、`eval_scheme` コマンド（`src-tauri/src/main.rs`）
      に渡して評価、結果をミニバッファに表示する。
- [x] `M-:` (eval-expression): ミニバッファに Scheme 式を直接入力して評価するモードを追加（Phase 3 の `M-x` 基盤を流用）。
- [x] REPL 履歴とエディタの kill-ring / undo 履歴が競合しないよう、フォーカス管理（テキストエリア vs REPL 入力欄）を整理する。

## Phase 7: モードライン / 仕上げ

- [x] `ModeLine.tsx` に現在の mark 状態（リージョン選択中かどうか）、isearch 状態などを表示する。
- [ ] `C-h k` (describe-key) 相当の簡易ヘルプ（実装済みキーバインド一覧をミニバッファかダイアログで表示）。
- [x] キーバインドの一覧をドキュメント化し `README.md` に追記する。
- [x] 各フェーズの hook ごとに Vitest でのテストを追加し、`npm run check` を通すこと。

---

## 実装順序の目安

1. Phase 1（配線の統合）→ 2. Phase 2（ハイライト）→ 3. Phase 3（ミニバッファ拡張）
4. Phase 4（編集コマンド強化）→ 5. Phase 6（REPL 統合、Scheme エディタとしての核心機能）
6. Phase 5（複数バッファ/ウィンドウ、UI 的に大掛かりなので後回し）→ 7. Phase 7（仕上げ）

Phase 1 は既存コードの接続のみで大きな新規実装が不要なため最優先。
Phase 6（REPL 統合）は「Scheme エディタ」としての価値に直結するため、Phase 5 より先に着手する方が良い。
