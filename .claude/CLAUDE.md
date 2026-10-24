# Scheme エディタプロジェクトガイド

## 概要
Tauri + React + Rust で構築された Scheme 言語エディタで、組み込みインタプリタと REPL を備えています。

## 技術スタック
- **フロントエンド**: React 18 + TypeScript + Vite
- **バックエンド**: Rust + Tauri 2
- **ビルド**: Cargo + npm
- **開発**: フロントエンドと Rust バックエンド両方のホットリロード対応

## アーキテクチャ
フロントエンド（`src/` 内の React コンポーネント）はミニバッファ、ファイル操作、コード表示を備えたエディタ UI を管理します。バックエンド（`src-tauri/` 内の Rust）は Scheme インタプリタ、パーサ、評価器、ファイル I/O を処理します。

- エディタは CodeMirror 6 (`src/components/CodeEditor.tsx`)。バッファごとに `EditorState` を持ち, 切り替えるときに差し替える。
- Emacs のコマンドとキーの割り当ては `src/lib/emacs.ts`。コマンドは Emacs の名前で引き, キーと `M-x` の両方から使う。画面のコマンド (ファイル, バッファ, 検索, 評価) は `App.tsx` の `actions` に置く。
- 位置の計算 (単語, 行) は `src/lib/text.ts`, kill-ring は `src/lib/killRing.ts`, S 式の範囲は `src/lib/bracketMatch.ts`。
- Lisp からのエディタの操作: 評価器は, 操作を `Host` (`src-tauri/src/scheme/host.rs`) に頼む。アプリでは `Bridge` が `editor-request` のイベントで画面に頼み, 画面 (`src/lib/lisp.ts`) が操作をして `editor_reply` で返事をする。テストでは, 偽の `Host` を使う。位置は Emacs と同じく 1 から数える。
- 評価は, 大きなスタックのスレッドで行う (`Evaluator::run`)。Lisp で作ったコマンド (`define-command`) とキー (`global-set-key`) は評価器が覚え, 評価の結果 (`EvalResponse`) で画面に渡す。
- `src/` 全体のカバレッジを 100% に保つ (`vite.config.ts` の `thresholds`)。`src-tauri/` も 100% に保つ (`npm run coverage` の `cargo tarpaulin --fail-under 100`。CI の `coverage` のジョブも同じ)。Rust のテストは `src-tauri/tests/` に置き, `lib.rs` から公開した関数を呼ぶ。CodeMirror のテストは, jsdom の上で, 実際の `EditorView` を作って行う。

## 開発ワークフロー
- **開発サーバーを起動**: `npm run tauri:dev` — フロントエンドと Rust は両方とも変更時に自動リロード
- **テスト**: `npm run test`（`src/` に対する Vitest）と `cargo test`（`src-tauri/` に対する）
- **フォーマット + リント**: `npm run check`（format:check、lint、test を実行 — CI ゲート）
- CI は `main` へのすべてのプッシュ/PR で同じ内容を実行

## 機能の追加

### Scheme 組み込み関数の追加
`src-tauri/src/scheme/evaluator.rs` を編集:
1. `eval_builtin()` 関数内に新しい match 分岐を追加
2. `Value` 型のパターンマッチングを使用してロジックを実装
3. `Value` 結果またはエラー文字列を返す

### UI の更新
`src/` 内のコンポーネントを編集:
1. コンポーネントは `src/components/` に配置
2. スタイルは `src/styles/` に配置
3. 状態/プロップの変更について `App.tsx` を更新

### Tauri コマンド
React から呼び出し可能な新しいバックエンド関数を追加:
1. `src-tauri/src/main.rs` に `#[tauri::command]` 関数を追加
2. `.invoke_handler(tauri::generate_handler![...])` リストに追加
3. React から呼び出し: `await invoke('function_name', { arg: value })`

## 現在の実装状況

### 実装済み
- ✅ パーサ: リスト、シンボル、数値、文字列、ブール値、`'`
- ✅ 算術演算と比較: +, -, *, /, =, <, >, <=, >=, abs, min, max, quotient, remainder, modulo
- ✅ 制御フロー: if, cond, case, and, or, when, unless
- ✅ 変数バインディング: define, set!, let (名前付きも), let*, letrec
- ✅ lambda (残りの引数も)、末尾の呼び出し
- ✅ リスト操作: quote, list, car, cdr, cons, append, length, map など
- ✅ 型チェック: number?, string?, symbol? など
- ✅ エディタの操作と, Lisp でのカスタマイズ (define-command, global-set-key, 初期設定ファイル)
- ✅ REPL と履歴機能（最後の 10 項目）
- ✅ エラーハンドリングと表示
- ✅ ダークテーマ UI

### TODO
- [ ] マクロ: define-syntax
- [ ] ドット対 (リストは, 今は Vec で持つ)
- [ ] エディタ内のコメント（既にパースされているが）
- [ ] 構文ハイライト（Monaco Editor の検討）
- [ ] 行番号付きのより良いエラーメッセージ

## ビルドと配布

```bash
# 開発
npm run tauri:dev

# 現在のプラットフォーム用にビルド
npm run tauri:build

# ビルドされたアプリの場所
src-tauri/target/release/rseditor  # または Windows では .exe、macOS では .app
```

## ヒント

- フロントエンドのホットリロードは Vite 開発サーバーを経由して機能
- Rust コードの変更はアプリのリロードが必要（自動的に発生）
- エラーをチェック: Tauri アプリで Ctrl+Shift+I
- Rust のコンパイルは最初のビルドでは遅くなる可能性がある

## コミットガイドライン

- コミットメッセージは英語で記述
- **件名は、Conventional Commits の形式で書く**: `type(scope): description`。scope (括弧を含む) は、省略できる。description は、英語で、命令形 (例: `feat(scheme): Add the string functions to the evaluator`)。
  - type は、次のどれか。
    - `feat`: 機能の追加
    - `fix`: バグの修正
    - `docs`: 文書とコメントだけの変更 (`*.md`、`docs/`、TSDoc、Rust の `///` コメント)
    - `style`: 整形、規約の適用 (動作を変えない)
    - `refactor`: 動作を変えない、書き換え
    - `test`: テストだけの変更
    - `build`: ビルドと、開発環境 (`package.json`、`vite.config.ts`、`tsconfig*.json`、`src-tauri/Cargo.toml`、`src-tauri/tauri.conf.json`) の変更
    - `ci`: CI (`.github/`) の変更
    - `chore`: そのほかの雑務 (最初のコミット、`.gitignore`、依存パッケージの更新 (`package-lock.json`) など)
  - scope は、変更の場所: `editor` (`src/` のエディタの画面)、`scheme` (`src-tauri/src/scheme/` の Scheme の処理系)、`backend` (`src-tauri/` のそれ以外)、`deps` (依存パッケージの更新)。複数にまたがるときは、省略する。
  - 互換性を壊す変更は、type (または scope) の直後に、感嘆符を付ける (Conventional Commits の仕様のとおり)。
  - 複数の type にまたがるときは、変更の中心になる type にする。
- 件名のあとに空行を 1 行入れて、本文を書く。本文はダッシュ/ハイフンで始まる箇条書き (例: `- Fix bug in parser`)。箇条書きの各行の間には、空行を入れない
- コミットメッセージに `Co-Authored-By` または Claude 帰属行を含めない
- **rebase したあとの push は、`git push --force-with-lease --force-if-includes origin <ブランチ名>` で行う** (`--force` は使わない。リモートに、手元に取り込んでいないコミットがあれば、push は失敗する)。ユーザーに依頼されたときだけ、push する。

### Git 設定

```bash
git config user.name "Tetsuya Higashi"
git config user.email "996846+sluchin@users.noreply.github.com"
```
