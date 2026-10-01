# Scheme エディタプロジェクトガイド

## 概要
Tauri + React + Rust で構築された Scheme 言語エディタで、組み込みインタプリタと REPL を備えています。

## 技術スタック
- **フロントエンド**: React 18 + TypeScript + Vite
- **バックエンド**: Rust + Tauri 2
- **ビルド**: Cargo + npm
- **開発**: フロントエンドと Rust バックエンド両方のホットリロード対応

## アーキテクチャ
フロントエンド（`src/` 内の React コンポーネント）はミニバッファ、ファイル操作、コード表示を備えたエディタ UI を管理します。バックエンド（`src-tauri/` 内の Rust）は Scheme インタプリタ、パーサ、評価器、ファイル I/O を処理します。`src/lib/` のユーティリティモジュールはキーマッピング、構文ハイライト、括弧マッチング機能を提供します（未統合）。

## 開発ワークフロー
- **開発サーバーを起動**: `npm run tauri:dev` — フロントエンドと Rust は両方とも変更時に自動リロード
- **テスト**: `npm run test`（`src/` に対する Vitest）と `cargo test`（`src-tauri/` に対する）
- **フォーマット + リント**: `npm run check`（format:check、lint、test を実行 — CI ゲート）
- CI は `main` へのすべてのプッシュ/PR で同じ内容を実行

## 機能の追加

### Scheme 組み込み関数の追加
`src-tauri/src/scheme/evaluator.rs` を編集:
1. `register_builtins()` 内に `reg("名前", |interp, args| ...)` を追加（特殊形式は `eval_inner()` の match に追加）
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
- ✅ パーサ: リスト、シンボル、数値、文字列、ブール値、クォート（`'x`）、コメント
- ✅ 評価器: 環境（レキシカルスコープ）、クロージャ、末尾呼び出し最適化
- ✅ 特殊形式: quote, if, cond, case, and, or, when, unless, define, set!, lambda, begin, let, let*, letrec, 名前付き let
- ✅ 組み込み関数: 算術・比較、リスト操作（car, cdr, cons, append, length, reverse, list-ref, map, for-each, filter, fold, reduce, apply）、型述語、文字列、display/newline
- ✅ REPL と履歴機能（最後の 10 項目）
- ✅ エラーハンドリングと表示
- ✅ ダークテーマ UI

### TODO
インタプリタ（`src-tauri/src/scheme/`）の未実装項目:
- [ ] マクロ: define-syntax / syntax-rules
- [ ] 準クォート: quasiquote（`` ` ``）, unquote（`,`）, unquote-splicing（`,@`）
- [ ] 制御構文: do, delay / force, call/cc, dynamic-wind, let-values, case-lambda
- [ ] データ型: ドット対（改良リスト, `set-car!` / `set-cdr!`）、文字型（`#\a`）、ベクタ、ハッシュテーブル
- [ ] 数値: 整数と実数の区別（現状はすべて f64）、sqrt, expt, floor, round, exp など数学関数、`string->number`
- [ ] リスト関数: member, assoc, assq, list-tail, cadr 系, last, iota, sort, delete, 複数リストを取る map / for-each
- [ ] 文字列関数: substring, string=?, string<?, string-upcase, string->symbol, string->list など
- [ ] エラー処理: error, assert, guard / with-exception-handler
- [ ] 入出力: write, read, ファイルの load、`display` を逐次 REPL に流す（現状は評価完了後にまとめて返す）
- [ ] 行番号付きのより良いエラーメッセージ（パーサ・評価器とも位置情報なし）
- [ ] `src-tauri/tests/evaluator_edge_cases.rs` が評価器を呼ばないプレースホルダのままなので、`Evaluator` を使うテストに置き換える

R7RS-small 準拠に向けた項目（現状は R7RS の小さなサブセットで、準拠していない。上記の個別項目も含め、多くは再設計を伴う）:
- [ ] 値表現をペア（cons セル）ベースに変更する（ドット対、O(1) の `cons`、`set-car!` / `set-cdr!`、`eq?` の同一性判定の前提）
- [ ] 数値塔: 正確数 / 不正確数、有理数（`1/3`）、`exact` / `inexact`、`exact->inexact` など
- [ ] `eq?` / `eqv?` / `equal?` の区別（現状はすべて構造的比較）
- [ ] 特殊形式をシンボル名で判定している点を改め、再束縛やマクロ展開と矛盾しないようにする
- [ ] 構文: `syntax-rules`, `case-lambda`, `parameterize`, `define-record-type`, `cond` / `case` の `=>`, トップレベルの `begin` による `define` 展開
- [ ] ライブラリ: `import` / `define-library`
- [ ] 継続・多値: `call/cc`, `dynamic-wind`, `values` / `call-with-values`
- [ ] 例外: `raise`, `raise-continuable`, `error`, `with-exception-handler`, `guard`
- [ ] 字句: `#true` / `#false`, `#\` 文字, `#(` ベクタ, `|sym|`, `#;` データコメント, `#| |#` ブロックコメント, `1e3` / `#x10` / `1/2` などの数値リテラル, 文字列の `\x41;` などのエスケープ
- [ ] 入出力ポート: `current-output-port`, 文字列ポート, `write` / `write-string` / `read` と、`write` と `display` の出力の区別
- [ ] 再帰の深さ上限（現在 10000）に頼らない評価（継続や明示的なスタックの導入と合わせて検討）
- [ ] 適合性テスト（R7RS の例を使ったテストスイート）を追加し、準拠状況を `FUNCTIONS.md` に記載する

エディタ側の未実装項目は `TODO.md` を参照。

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
- メッセージ本体はダッシュ/ハイフンで始まる（例: `- Fix bug in parser`）
- コミットメッセージに `Co-Authored-By` または Claude 帰属行を含めない

### Git 設定

```bash
git config user.name "Tetsuya Higashi"
git config user.email "996846+sluchin@users.noreply.github.com"
```
