# Scheme エディタ (rseditor)

**Tauri**、**React**、**Rust** で構築されたモダンな Scheme プログラミング言語エディタです。

## 機能

- 📝 **コードエディタ** - CodeMirror 6 の上で、Emacs のキー操作で Scheme コードを書く (キーの一覧は `KEYBINDINGS.md`)
- ⚙️ **Scheme インタプリタ** - Rust ベースの Scheme 処理系:
  - 特別な形: `quote` (`'`)、`if`、`define`、`set!`、`lambda`、`begin`、`let` (名前付きも)、`let*`、`letrec`、`cond`、`case`、`and`、`or`、`when`、`unless`
  - 数: `+`、`-`、`*`、`/`、`=`、`<`、`>`、`<=`、`>=`、`abs`、`min`、`max`、`quotient`、`remainder`、`modulo` など
  - リスト: `car`、`cdr`、`cons`、`list`、`length`、`append`、`reverse`、`map`、`for-each`、`filter`、`apply`、`assoc`、`member` など
  - 文字列: `string-append`、`substring`、`string-length`、`format` など
  - 末尾の呼び出しは、深さを使わない。それ以外の入れ子は 10000 まで
- 🔧 **Lisp でのカスタマイズ** - Lisp からエディタを動かし、コマンドを作り、キーを割り当てる。起動のときに `~/.config/rseditor/init.scm` を読む (「Lisp でのカスタマイズ」)
- 💬 **REPL** - 対話的 Read-Eval-Print ループとコマンド履歴
- 📊 **出力表示** - 評価結果とエラーメッセージを表示

## プロジェクト構成

```
.
├── src/                      # React フロントエンド
│   ├── components/          # エディタと REPL コンポーネント
│   ├── lib/                 # Emacs のコマンド (emacs.ts)、Lisp とのやり取り (lisp.ts) など
│   ├── styles/              # コンポーネントスタイルシート
│   ├── App.tsx              # メインアプリケーション
│   └── main.tsx             # エントリーポイント
├── src-tauri/               # Tauri バックエンド
│   ├── src/
│   │   ├── main.rs          # Tauri アプリのエントリーポイント
│   │   └── scheme/          # Scheme インタプリタ
│   │       ├── parser.rs    # Scheme パーサー
│   │       ├── evaluator.rs # Scheme 評価器 (特別な形)
│   │       ├── builtins.rs  # 組み込みの手続き (エディタの操作を含む)
│   │       ├── env.rs       # 変数の環境
│   │       ├── host.rs      # エディタ (画面) に操作を頼む仕組み
│   │       ├── keys.rs      # キーの書き方の変換 (Emacs → CodeMirror)
│   │       ├── value.rs     # 値型定義
│   │       └── mod.rs       # モジュール定義
│   └── Cargo.toml           # Rust 依存関係
├── package.json             # Node 依存関係
├── vite.config.ts           # Vite ビルド設定
└── tsconfig.json            # TypeScript 設定
```

## ログ

ログは、標準エラー出力と、ログファイルに出ます。フロントエンド (`loglevel`) のログも、バックエンドに転送して (`log_frontend_message`)、同じファイルに残します。

- ログファイル: `<データディレクトリ>/rseditor/logs/rseditor.log` (Linux は `~/.local/share/rseditor/logs/`、Windows は `%LOCALAPPDATA%\rseditor\logs\`、macOS は `~/Library/Application Support/rseditor/logs/`)。5 MB を超えると世代送りして、3 世代 (`.1` から `.3`) を残します。
- レベル: デバッグ版 (`npm run tauri:dev`) はデバッグ以上、リリース版 (`npm run tauri:build`) は情報 (最低限)、警告、エラーを出します。判定は、ログの初期化の中でします (Rust は `src-tauri/src/logging.rs` の `default_level`、フロントエンドは `src/lib/logForwarder.ts` の `setupLogging`)。
- レベルを変えるとき: `RUST_LOG=debug npm run tauri:dev` のように、環境変数 `RUST_LOG` で上書きします (リリース版でも効きます)。
- ログは、Rust は `log::debug!` / `info!` / `warn!` / `error!`、フロントエンドは `loglevel` の `log.debug` / `info` / `warn` / `error` を使います。パスワードなどの秘密は書きません。

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

**ローカル開発（TypeScript のみ）:**

```bash
# TypeScript 静的解析（ESLint）
npm run lint

# Vitest でテスト実行
npm run test

# カバレッジ (フロントエンドは @vitest/coverage-v8、バックエンドは cargo tarpaulin。どちらも 100% でなければ失敗する。結果は coverage/index.html と src-tauri/tarpaulin-report.html)
# Rust 側は、初回のみ `cargo install cargo-tarpaulin` が必要
npm run coverage

# テストをウォッチモードで実行
npm run test:watch

# ESLint の自動修正
npm run lint:fix

# コード整形（Prettier）
npm run format

# 整形済みかどうかチェック
npm run format:check
```

**Rust も含めて完全にチェック（CI 相当）:**

```bash
# TypeScript + Rust の static analysis（ESLint + cargo clippy）
npm run lint:all

# TypeScript + Rust テスト
npm run test:all

# format:check → lint:all → test:all をまとめて実行
npm run check
```

**注:**

- `npm run lint` はローカル環境で実行可能（TypeScript のみ）
- `npm run lint:all` と `npm run test:all` は Rust のビルド環境が必要
- GitHub Actions の CI (`.github/workflows/ci.yml`) は `npm run check` を実行し、`main` ブランチへの push・PR ごとに全てのチェックが自動実行されます

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

### 定義と手続き

```scheme
(define (fact n) (if (= n 0) 1 (* n (fact (- n 1)))))
(fact 10)                         ; => 3628800
(map (lambda (x) (* x x)) '(1 2 3)) ; => (1 4 9)
(let loop ((i 0) (acc '()))       ; 名前付きの let で繰り返す
  (if (= i 3) acc (loop (+ i 1) (cons i acc)))) ; => (2 1 0)
```

定義は、アプリを閉じるまで覚えています (`C-x C-e`、`M-:`、初期設定ファイルで共通)。

## Lisp でのカスタマイズ

### エディタを動かす関数

位置は、Emacs と同じく 1 から数えます。

| 関数 | 内容 |
|------|------|
| `(point)` / `(point-min)` / `(point-max)` | カーソルの位置 / 先頭 / 末尾 |
| `(goto-char pos)` | カーソルを動かす |
| `(insert "文字" ...)` | カーソルの位置に入れる |
| `(delete-region from to)` | 範囲を消す |
| `(buffer-string)` / `(buffer-substring from to)` | 本文 / 範囲の文字 |
| `(buffer-name)` | バッファの名前 |
| `(message "形式" 引数 ...)` | ミニバッファに出す (`%s`、`%S`、`%d`、`%%`) |
| `(call-interactively 'name)` / `(command-execute 'name)` | コマンドを名前で実行する (`M-x` と同じ) |

### コマンドとキー

```scheme
;; コマンドを作る (M-x insert-hello で実行できる)
(define-command (insert-hello)
  (insert "Hello, " (buffer-name)))

;; キーを割り当てる (Emacs の書き方: C- は Ctrl、M- は Alt、空白で区切ると続けて押すキー)
(global-set-key "C-c h" 'insert-hello)
(global-set-key "<f5>" 'save-buffer)
```

- `define-command` は `(define-command (名前) 本体 ...)` か、`(define-command 名前 手続き)`。同じ名前の既にあるコマンドより、先に使います。
- `global-set-key` の割り当ては、既定の割り当てより先に使います。名前の付いたキーは `SPC`、`RET`、`TAB`、`DEL`、`ESC`、`<f1>` から `<f12>`、`<left>` などです。

### 初期設定ファイル

起動のときに、`~/.config/rseditor/init.scm` (OS の設定のディレクトリの `rseditor/init.scm`) を読んで評価します。ファイルがなければ、何もしません。誤りがあっても起動は続け、ミニバッファに知らせます (誤りの前の式は、評価済みです)。

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
- `C-v` / `M-v` - 1 画面先へ/前へ
- `C-l` - カーソルの行を画面の中央へ
- `M-g g` - 入れた行へ移動

**編集**

- `C-d` - 文字を削除
- `C-k` - 行を削除
- `M-d` / `M-DEL` - 単語を削除
- `M-u` / `M-l` / `M-c` - 単語を大文字/小文字/先頭だけ大文字に
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

- [x] 拡張算術演算 (modulo, abs など)
- [ ] sqrt などの数学の関数
- [x] 制御フロー (if, cond)
- [x] 変数バインディング (define, let)
- [x] ラムダ関数
- [x] 条件と比較 (>, <, =)
- [x] 文字列操作
- [x] リスト操作 (car, cdr, cons)
- [x] Lisp でのカスタマイズ (コマンド、キー、初期設定ファイル)
- [x] シンタックスハイライト (Emacs の scheme-mode と emacs-lisp-mode と同じ色分け)
- [x] 字下げ (Emacs の scheme-mode と emacs-lisp-mode と同じ) と, 括弧の強調 (show-paren-mode)
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

Tauri のコマンドシステムを使用して React から `eval_scheme` 関数を呼び出し、コード文字列を送信して結果/エラーを受け取ります。結果には、Lisp で作ったコマンドと、割り当てたキーも含みます。

評価の途中のエディタの操作 (`insert` など) は、評価器が `editor-request` のイベントで画面に頼み、画面が操作をして `editor_reply` で返事をします。評価は別のスレッドで行い、返事を待ちます (5 秒で失敗にする)。Lisp で作ったコマンドは `run_lisp_command`、初期設定ファイルは `load_init` で評価します。

## ライセンス

GPL v3.0 - 詳細は [LICENSE](./LICENSE) ファイルを参照してください。
