# Scheme エディタプロジェクトガイド

## 概要
Tauri + React + Rust で構築された, Emacs のように使えるエディタで、Scheme の組み込みインタプリタを備えています (評価は `C-x C-e` と `M-:`)。

## 技術スタック
- **フロントエンド**: React 18 + TypeScript + Vite
- **バックエンド**: Rust + Tauri 2
- **ビルド**: Cargo + npm
- **開発**: フロントエンドと Rust バックエンド両方のホットリロード対応

## アーキテクチャ
フロントエンド（`src/` 内の React コンポーネント）はミニバッファ、ファイル操作、コード表示を備えたエディタ UI を管理します。バックエンド（`src-tauri/` 内の Rust）は Scheme インタプリタ、パーサ、評価器、ファイル I/O を処理します。

- エディタは CodeMirror 6 (`src/components/CodeEditor.tsx`)。バッファごとに `EditorState` を持ち, 切り替えるときに差し替える。
- ウィンドウ (`C-x 2` など) は, 分け方を木 (`src/lib/windows.ts`) で持ち, 描くときは位置を % で計算して平らに並べる (分け方が変わっても, エディタを作り直さない)。ウィンドウごとに `CodeEditor` を置き, 同じバッファの書き換えは `sync` で別のウィンドウに写す。
- Emacs のコマンドとキーの割り当ては `src/lib/emacs.ts`。コマンドは Emacs の名前で引き, キーと `M-x` の両方から使う。画面のコマンド (ファイル, バッファ, 検索, 評価) は `App.tsx` の `actions` に置く。
- シンタックスハイライトは, CodeMirror の `StreamLanguage` で, Emacs 30 の scheme-mode と emacs-lisp-mode と同じ色分けにする (`src/lib/lispMode.ts`)。決まりは, Emacs の font-lock の変数 (`scheme-font-lock-keywords-2`, `lisp-el-font-lock-keywords-2`) から取り, `emacs -Q --batch` で付けた色と比べて確かめる。Emacs Lisp のマクロの名前は `src/lib/elispKeywords.ts` (`emacs -Q` から作る)。バッファのメジャーモード (色を付けるか, 字下げ, モードラインの名前) は `src/lib/modes.ts` で決める (ファイルの 1 行目の `-*- ... -*-` も読む)。
- 字下げは, Emacs 30 の `calculate-lisp-indent` と同じ計算 (`src/lib/lispIndent.ts`)。フォームごとの指定は `src/lib/lispIndentSpecs.ts` (`emacs -Q` から作る)。`emacs -Q --batch` の `indent-region` の結果と比べて確かめる。括弧の強調 (show-paren-mode) は `src/lib/showParen.ts`。矩形 (`C-x r k` など) は `src/lib/rectangle.ts`, ブックマーク (`C-x r m`, `C-x r b`) は `src/lib/bookmarks.ts` (保存は `~/.config/rseditor/bookmarks`)。段落を詰める (`M-q`) のは `src/lib/fill.ts` で, `emacs -Q --batch` の `fill-paragraph` の結果と比べて確かめる。
- 位置の計算 (単語, 行) は `src/lib/text.ts`, kill-ring は `src/lib/killRing.ts`, S 式の範囲は `src/lib/bracketMatch.ts`。
- Lisp からのエディタの操作: 評価器は, 操作を `Host` (`src-tauri/src/scheme/host.rs`) に頼む。アプリでは `Bridge` が `editor-request` のイベントで画面に頼み, 画面 (`src/lib/lisp.ts`) が操作をして `editor_reply` で返事をする。テストでは, 偽の `Host` を使う。位置は Emacs と同じく 1 から数える。
- 評価は, 大きなスタックのスレッドで行う (`Evaluator::run`)。Lisp で作ったコマンド (`define-command`) とキー (`global-set-key`) は評価器が覚え, 評価の結果 (`EvalResponse`) で画面に渡す。
- `src/` 全体のカバレッジを 100% に保つ (`vite.config.ts` の `thresholds`)。`src-tauri/` も 100% に保つ (`npm run coverage` の `cargo tarpaulin --fail-under 100`。CI の `coverage` のジョブも同じ)。フロントエンドのテストは `tests/` に, `src/` と同じ構成で置く (`src/lib/text.ts` のテストは `tests/lib/text.test.ts`。`src/` は `@/` で参照する)。Rust のテストは `src-tauri/tests/` に置き, `lib.rs` から公開した関数を呼ぶ。CodeMirror のテストは, jsdom の上で, 実際の `EditorView` を作って行う。

## 開発ワークフロー
- **開発サーバーを起動**: `npm run tauri:dev` — フロントエンドと Rust は両方とも変更時に自動リロード
- **テスト**: `npm run test`（`tests/` の Vitest）と `cargo test`（`src-tauri/` に対する）
- **フォーマット + リント**: `npm run check`（format:check、lint、test を実行 — CI ゲート）
- CI は `main` へのすべてのプッシュ/PR で同じ内容を実行

## 機能の追加

### Scheme 組み込み関数の追加
`src-tauri/src/scheme/builtins.rs` を編集:
1. `NAMES` に名前を追加 (評価器は, 起動のときに, この名前を一番外側の環境に定義する)
2. `call()` 関数内に新しい match 分岐を追加 (引数は `fixed`, `number`, `string` などで取り出す)
3. `Value` 結果またはエラー文字列を返す
4. 特別な形 (引数を評価しないもの) は, `src-tauri/src/scheme/evaluator.rs` の `special_form()` に追加

### UI の更新
`src/` 内のコンポーネントを編集:
1. コンポーネントは `src/components/` に配置
2. スタイルは `src/styles/` に配置
3. 状態/プロップの変更について `App.tsx` を更新

### Tauri コマンド
React から呼び出し可能な新しいバックエンド関数を追加:
1. `src-tauri/src/commands.rs` に `#[tauri::command]` 関数を追加 (本体は普通の関数にして, `src-tauri/tests/` から呼べるようにする)
2. `src-tauri/src/lib.rs` の `.invoke_handler(tauri::generate_handler![...])` リストに追加
3. React から呼び出し: `await invoke('function_name', { arg: value })` (`src/lib/fileOps.ts` や `src/lib/lisp.ts` のように, 関数で包む)

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
- ✅ 評価: カーソルの前の S 式 (`C-x C-e`) と, ミニバッファに入力した式 (`M-:`)。結果はミニバッファに出す
- ✅ エラーハンドリングと表示
- ✅ ダークテーマ UI
- ✅ シンタックスハイライト (Emacs の scheme-mode と emacs-lisp-mode と同じ色分け) と, ファイルの名前の補完
- ✅ 字下げ (Emacs の scheme-mode と emacs-lisp-mode と同じ) と, 括弧の強調

### TODO
- [ ] マクロ: define-syntax
- [ ] ドット対 (リストは, 今は Vec で持つ)
- [ ] エディタ内のコメント（既にパースされているが）
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
