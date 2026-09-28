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

MIT
