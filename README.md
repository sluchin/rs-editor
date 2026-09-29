# Scheme エディタ (rseditor)

**Tauri**、**React**、**Rust** で構築されたモダンな Scheme プログラミング言語エディタです。

## 機能

- 📝 **コードエディタ** - Scheme コードを記述し、キーボードショートカット (Ctrl+Enter) で実行
- ⚙️ **Scheme インタプリタ** - Rust ベースの Scheme 処理系:
  - 算術演算: `+`, `-`, `*`, `/`
  - リスト操作: `list`, `quote`
  - 基本値の型: 数値、シンボル、文字列、ブール値、リスト
- 💬 **REPL** - 対話的 Read-Eval-Print ループとコマンド履歴
- 📊 **出力表示** - 評価結果とエラーメッセージを表示

## プロジェクト構成

```
.
├── src/                      # React フロントエンド
│   ├── components/          # エディタと REPL コンポーネント
│   ├── styles/              # コンポーネントスタイルシート
│   ├── App.tsx              # メインアプリケーション
│   └── main.tsx             # エントリーポイント
├── src-tauri/               # Tauri バックエンド
│   ├── src/
│   │   ├── main.rs          # Tauri アプリのエントリーポイント
│   │   └── scheme/          # Scheme インタプリタ
│   │       ├── parser.rs    # Scheme パーサー
│   │       ├── evaluator.rs # Scheme 評価器
│   │       ├── value.rs     # 値型定義
│   │       └── mod.rs       # モジュール定義
│   └── Cargo.toml           # Rust 依存関係
├── package.json             # Node 依存関係
├── vite.config.ts           # Vite ビルド設定
└── tsconfig.json            # TypeScript 設定
```

## セットアップと開発

### 前提条件
- Node.js 18 以上
- Rust 1.70 以上
- Tauri CLI 2

### インストール

```bash
# リポジトリをクローン
git clone <repo>
cd rseditor

# Node 依存関係をインストール
npm install

# Tauri CLI をインストール（オプション、すでに devDependencies に含まれています）
npm install -g @tauri-apps/cli@2
```

### 開発モードで実行

```bash
# 開発サーバーを起動（Tauri アプリをビルドして実行）
npm run tauri:dev
```

このコマンドは以下の処理を実行します:
1. Vite dev サーバーを `http://localhost:5173` で起動
2. Rust バックエンドをコンパイル
3. Tauri デスクトップアプリケーションを起動

### 本番向けビルド

```bash
# 最適化された本番ビルドを作成
npm run tauri:build
```

ビルドされたアプリケーションは `src-tauri/target/release/` に出力されます。

### 開発ワークフロー（Lint / Format / Test）

```bash
# コードの静的解析（ESLint + cargo clippy）
npm run lint

# ESLint の自動修正
npm run lint:fix

# コード整形（Prettier + cargo fmt）
npm run format

# 整形済みかどうかチェックのみ（変更しない）
npm run format:check

# テストを実行（Vitest + cargo test）
npm run test

# テストをウォッチモードで実行（フロントエンドのみ）
npm run test:watch

# format:check → lint → test をまとめて実行
npm run check
```

`npm run check` は GitHub Actions の CI (`.github/workflows/ci.yml`) でも実行され、`main` ブランチへの push・PR ごとに自動でフォーマット・lint・テストが確認されます。

## 使用例

### 基本的な算術演算
```scheme
(+ 1 2 3)      ; => 6
(- 10 3)       ; => 7
(* 2 5)        ; => 10
(/ 20 4)       ; => 5
```

### リスト
```scheme
(list 1 2 3)   ; => (1 2 3)
(quote (a b c)) ; => (a b c)
```

### Scheme 式の評価
```scheme
; カーソルを式の直後に置いて C-x C-e で評価
(+ 2 3)        ; カーソル↑: C-x C-e → ミニバッファに「5」と表示

; または M-: でミニバッファで直接入力
M-:            ; ミニバッファが開く
(* 3 4)        ; 式を入力して Enter → ミニバッファに「12」と表示
```

## キーバインド

rs-editor は Emacs スタイルのキーバインドをサポートしています。詳細は [KEYBINDINGS.md](./KEYBINDINGS.md) を参照してください。

### よく使うショートカット

**ファイル操作**
- `C-x C-f` - ファイルを開く
- `C-x C-s` - ファイルを保存

**カーソル移動**
- `C-f` / `C-b` - 前後に移動
- `C-n` / `C-p` - 上下に移動
- `C-a` / `C-e` - 行の開始/末尾へ

**編集**
- `C-d` - 文字を削除
- `C-k` - 行を削除
- `C-y` - 貼り付け
- `C-/` - 取り消し（undo）
- `C-x C-/` - やり直し（redo）

**Scheme 評価**
- `C-x C-e` - カーソル直前の S 式を評価
- `M-:` - ミニバッファで式を直接入力して評価

**検索**
- `C-s` - 前方に検索
- `C-r` - 後方に検索

**バッファ管理**
- `C-x b` - バッファを切り替え
- `C-x k` - バッファを閉じる
- `C-x C-b` - バッファ一覧を表示

## 開発ロードマップ

- [ ] 拡張算術演算 (mod, abs, sqrt など)
- [ ] 制御フロー (if, cond)
- [ ] 変数バインディング (define, let)
- [ ] ラムダ関数
- [ ] 条件と比較 (>, <, =)
- [ ] 文字列操作
- [ ] リスト操作 (car, cdr, cons)
- [ ] Monaco Editor を使ったシンタックスハイライト
- [ ] コード補完と自動フォーマット
- [ ] デバッグサポート

## トラブルシューティング

### アプリが起動しない

ターミナルで以下を確認してください：

```bash
npm run tauri:dev
```

実行時のエラーメッセージを確認:

1. **`cargo` がない** → Rust をインストール: https://rustup.rs/
2. **`node_modules` がない** → `npm install` を実行
3. **ポート 5173 が使用中** → 別のターミナルで `npm run dev` が実行されていないか確認

### テキストエリアに入力できない

1. アプリが完全に起動するまで待つ（Rust コンパイルに時間がかかることがあります）
2. テキストエリアを**クリック**してフォーカスを当てる
3. テキストを入力

### デバッグ情報を見る

Tauri アプリ内で **Ctrl+Shift+I** を押すとデベロッパーツールが開き、エラーメッセージを確認できます。

## アーキテクチャ

### フロントエンド (React + TypeScript)
- **Editor.tsx** - コード入力エリアと実行ボタン
- **REPL.tsx** - 対話的出力、履歴、インラインREPL
- **App.tsx** - 状態管理と Tauri 通信を行うメインアプリコンポーネント

### バックエンド (Rust)
- **Parser** - Scheme 式をトークン化して AST にパース
- **Evaluator** - シンプルな環境/スコープシステムを使用して AST ノードを評価
- **Value** - Scheme 値を表現し、表示フォーマットを処理

### 通信
Tauri のコマンドシステムを使用して React から `eval_scheme` 関数を呼び出し、コード文字列を送信して結果/エラーを受け取ります。

## ライセンス

GPL v3.0 - 詳細は [LICENSE](./LICENSE) ファイルを参照してください。
