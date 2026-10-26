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
- 🔧 **Lisp でのカスタマイズ** - Lisp からエディタを動かし、コマンドを作り、キーを割り当てる。起動のときに `~/.rseditor` (または `~/.rseditor.d/init.scm`) を読む (「Lisp でのカスタマイズ」)
- 📝 **howm** - メモ (howm) の Scheme 版 (`lisp/howm.scm`)。`(require 'howm)` で読み, `M-x howm` でメニューを出す (「howm」)
- 🗂️ **Org mode** - Emacs の Org mode (org 9.7.11) の Scheme 版 (`lisp/org.scm`)。`(require 'org)` で読み, `.org` のファイルを開くと Org mode になる (「Org mode」)
- 🔍 **M-x grep** - 外のコマンドを使わず、エディタ (Rust) がディレクトリの下を探す。結果は `*grep*` バッファに出し、`RET` でその行を開く。`M-g n` / `M-g p` で次・前へ
- 🌲 **プロジェクトのツリー** - VS Code のように、左のサイドバーにプロジェクト (`.git` のある所) のファイルのツリーを出す。`F8` (`M-x project-tree`) で出すか消す
- 💬 **評価** - カーソルの前の S 式 (`C-x C-e`) や、ミニバッファに入力した式 (`M-:`) を評価する
- 📊 **出力表示** - 評価結果とエラーメッセージを表示

## プロジェクト構成

```
.
├── src/                      # React フロントエンド
│   ├── components/          # エディタ、ミニバッファ、モードライン、バッファの一覧、プロジェクトのツリー
│   ├── hooks/               # 機能ごとの状態とコマンド (ファイル、dired と grep、検索と置換、ブックマーク、Lisp)
│   ├── lib/                 # Emacs のコマンド (emacs.ts)、Lisp とのやり取り (lisp.ts) など
│   ├── styles/              # コンポーネントスタイルシート
│   ├── lib/theme.ts         # 色とフォント (set-background-color, set-frame-font)
│   ├── lib/grep.ts          # M-x grep の結果のバッファ (*grep*)
│   ├── App.tsx              # メインアプリケーション (フックのコマンドを、キーとミニバッファにつなぐ)
│   └── main.tsx             # エントリーポイント
├── src-tauri/               # Tauri バックエンド
│   ├── src/
│   │   ├── main.rs          # Tauri アプリのエントリーポイント
│   │   ├── lib.rs           # アプリの起動と、コマンドの登録
│   │   ├── commands.rs      # 画面から呼ぶコマンド (評価、ファイルの読み書き)
│   │   ├── logging.rs       # ログ
│   │   ├── search.rs        # M-x grep (ディレクトリの下を探す) と、プロジェクトの根を探す
│   │   └── scheme/          # Scheme インタプリタ
│   │       ├── parser.rs    # Scheme パーサー
│   │       ├── evaluator.rs # Scheme 評価器 (特別な形)
│   │       ├── builtins.rs  # 組み込みの手続き (エディタの操作を含む)
│   │       ├── system.rs    # 組み込みの手続き (ファイル、時刻、正規表現、load、require)
│   │       ├── ert.rs       # テスト (Emacs の ERT と同じ書き方)
│   │       ├── env.rs       # 変数の環境
│   │       ├── host.rs      # エディタ (画面) に操作を頼む仕組み
│   │       ├── keys.rs      # キーの書き方の変換 (Emacs → CodeMirror)
│   │       ├── value.rs     # 値型定義
│   │       └── mod.rs       # モジュール定義
│   └── Cargo.toml           # Rust 依存関係
├── lisp/howm.scm             # howm の Scheme 版
├── lisp/org.scm              # Org mode の Scheme 版
├── tools/emacs-commands/     # Emacs の組み込みと基本のコマンドと, 既定のキーの一覧 (FUNCTIONS.md) を作る
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
| `(find-file "path")` | ファイルを開く (開いていれば、そのバッファを出す) |
| `(switch-to-buffer "name")` | バッファを出す (無ければ作る) |
| `(buffer-file-name)` | バッファのファイル (無ければ `#f`) |
| `(set-buffer-modified-p #f)` | バッファを、書き換えていないことにする |
| `(read-string "Prompt: " "初め" hist "既定")` | ミニバッファで入力してもらう (Emacs と同じ引数。空で確定すると、既定の値。`C-g` でやめると `Quit`) |
| `(completing-read "Prompt: " '("a" "b") pred require-match "初め" hist "既定")` | 候補を `Tab` で補完しながら入力してもらう (Emacs と同じ引数。候補は文字列、記号、alist。`require-match` が non-nil なら、候補に無い入力は聞き直す) |
| `(buffer-list)` / `(kill-buffer "name")` / `(buffer-modified-p)` | バッファの名前 / バッファを消す / 書き換えたか |
| `(display-buffer "name" 行)` / `(find-file-noselect "path")` | ほかのウィンドウに出す / 出さずに開く |
| `(highlight-regexp "re")` / `(unhighlight-regexp "re")` | 下線を引く / 消す (`#t` で全て。正規表現は JavaScript の書き方) |
| `(kill-new "text")` | kill ring に入れる |
| `(local-set-key "c" 'name)` | バッファだけのキー (Lisp のキーより先に使う) |
| `(set-background-color "black")` / `(set-foreground-color "white")` | 背景と文字の色 (背景が暗いと、色分けも暗い背景のものにする) |
| `(set-frame-font "DejaVu Sans Mono-14")` | フォント (ファミリーと、大きさ (ポイント)) |
| `(set-face-attribute 'default #f :family "Mono" :height 140)` | フォント (`:height` は 1/10 ポイント)、`:foreground`、`:background` |

### ファイル、時刻、文字列の関数

| 関数 | 内容 |
|------|------|
| `(load "path")` | ファイルを読んで評価する (相対パスは、読み込んでいるファイルのディレクトリから。見つからなければ、`load-path` のディレクトリから探す。`.scm` は省ける) |
| `(require 'name)` / `(provide 'name)` / `(featurep 'name)` | `load-path` から `name.scm` を 1 度だけ読む / 読んだことにする / 読んだか |
| `load-path` | ファイルを探すディレクトリのリスト (始めは `("~/.rseditor.d/lisp" "~/.rseditor.d")`) |
| `(make-temp-file "prefix" #t)` / `(delete-file "path")` / `(delete-directory "path" #t)` | 一時ファイル (`#t` ならディレクトリ) を作る / ファイルを消す / ディレクトリを消す (`#t` なら中身も) |
| `(expand-file-name "~/x")` | 絶対パスにする (`~` はホーム) |
| `(file-exists? "path")` / `(file-directory? "path")` | あるか / ディレクトリか |
| `(read-file-string "path")` / `(write-file-string "path" "text")` | ファイルを読む / 書く (ディレクトリも作る) |
| `(make-directory "path")` | ディレクトリを作る (親も) |
| `(directory-files-recursively "dir" "\\.txt$")` | 名前が正規表現に合うファイル (深くまで) |
| `(file-modification-time "path")` / `(current-time)` | 時刻 (1970 年からの秒数) |
| `(format-time-string "%Y-%m-%d" time)` / `(date-to-time "2026-10-25")` | 時刻と日付 (その土地の時刻) |
| `(regexp-match "a(b)" "xab")` / `(regexp-quote "a.b")` | 正規表現 (Rust の regex の書き方) |
| `(regexp-match-positions "b" "abcb")` | 合った所の全て (`((始め 終わり) ...)`。位置は文字で、0 から) |
| `(grep "regexp" files)` | 合う行 (`(パス 行の番号 行)` のリスト) |
| `(string-search "b" "abc")`, `string-prefix?`, `string-suffix?`, `string-trim`, `string-join`, `split-string` | 文字列 |
| `(sort list less?)`, `cadr` など, `(random n)` | リスト、乱数 |

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

### フック

Emacs と同じく、`add-hook`、`remove-hook`、`run-hooks`、`run-hook-with-args` が使えます。エディタは、ファイルを開いたバッファを初めて出したときに `find-file-hook` を、今のバッファを保存したときに `after-save-hook` を、そのバッファで実行します。

```scheme
(add-hook 'after-save-hook
          (lambda () (message "Saved %s" (buffer-file-name))))
```

### 初期設定ファイル

起動のときに、`~/.rseditor` を読んで評価します。無ければ `~/.rseditor.d/init.scm` (Emacs の `~/.emacs.d/init.el` と同じ)、それも無ければ `~/.config/rseditor/init.scm` (OS の設定のディレクトリの `rseditor/init.scm`) を読みます。読んだファイルは `user-init-file` に入ります。自分の Lisp のファイルは `~/.rseditor.d/lisp/` (または `~/.rseditor.d/`) に置くと、`(require 'name)` で読めます。どちらもなければ、何もしません。誤りがあっても起動は続け、ミニバッファに知らせます (誤りの前の式は、評価済みです)。

```scheme
;; ~/.rseditor.d/init.scm
(set-background-color "black")
(set-foreground-color "white")
(set-frame-font "Noto Sans Mono CJK JP-12")
(require 'howm)                   ; ~/.rseditor.d/lisp/howm.scm を読む
(global-set-key "C-c h" 'howm-menu)
```

### howm

`lisp/howm.scm` は、Emacs の howm (メモ) を Scheme で書き直したものです。`~/.rseditor.d/lisp/` に写して (`cp lisp/howm.scm ~/.rseditor.d/lisp/`)、初期設定ファイルに `(require 'howm)` と書くと、`M-x howm` (または `C-c , ,`) でメニューを出します。メモは `~/howm` に置きます (`(set! howm-directory "~/memo")` で変える)。

- メニュー: howm の日本語のメニューと同じです。ボタン (`[新規]` など) の上で `RET`、またはボタンの前の文字 (`c` など) で動かします。`%here%` (予定、todo、最近、ランダムなど)、`%eval%`、`%call%`、`%|...%|` (隠す)、`%"x"` (キー)、`>>> %名前%` (別のメニュー) が使えます。メニューは `~/howm/0000-00-00-000000.txt` から作ります (無ければ、既定のメニューを書く。`[menu 編集]` で直せる)。
- メモ: 作る (`C-c , c`)、速記 (`C-c , e`。`C-c C-c` でメモにする)、複製 (`C-c , D`)、題と名前を kill ring へ (`C-c , K`)。
- 一覧と検索: 一覧 (`a`)、最近 (`l`)、前後 (`A`)、検索 (`s` はキーワードを補完、`g` は正規表現、`m` はローマ字、`o` は今のバッファ)、履歴 (`h`)、酔歩 (`w`)、今日 (`.`)、昨日 (`:`)、予定 (`y`)、todo (`t`)。一覧のバッファ (`*howmS*`) は、`n` と `p` で動くと、別のウィンドウにメモの中身を出し、`RET` でその行のメモを開きます。
- リンク: `>>> 言葉`、`<<< 言葉`、`[[言葉]]` (wiki)、キーワード、`file://` に下線を引き、その上の `RET` で探す (開く) ことができます。`<<< 言葉` の行を、一覧の先に出します。
- 予定と todo: 書き方は howm と同じです (`[2026-10-25]@ 予定`、`+` todo、`!` 〆切、`-` 覚書、`~` 保留、`.` 済み)。項目の上の `RET` で、済み (`.`)、取り消し、種類、怠け具合を変えます。日付の上の `RET` で、日付を変える (`+1` など)、繰り返して写す (`every` など)、その日付で探すことができます。日付と日時を入れる (`C-c , d`、`C-c , T`)、キーワードを入れる (`C-c , i`)。
- そのほか: howm のバッファを全て消す (`C-c , Q`)、バッファの一覧 (`C-c , b`)、出し直す (`C-c , r`)。
- フック: `C-x C-f` で開いたメモも、メモのバッファにします (`howm-set-mode-on-find-file`)。メモを保存すると、キーワードを集め直して下線を引き直し、メニューを出していれば出し直します (`howm-refresh-after-save`、`howm-menu-refresh-after-save`、`howm-after-save-hook`)。

### Org mode

`lisp/org.scm` は、Emacs の Org mode (org 9.7.11) を Scheme で書き直したものです。`~/.rseditor.d/lisp/` に写して (`cp lisp/org.scm ~/.rseditor.d/lisp/`)、初期設定ファイルに `(require 'org)` と書くと、`.org` のファイルを開いたとき Org mode になります (`M-x org-mode` でも)。変数の名前と既定の値は Emacs と同じです (`(set! org-directory "~/org")`、`(set! org-agenda-files '("~/org"))`、`(set! org-todo-keywords '((sequence "TODO" "WAIT" "|" "DONE")))` など)。キーは「KEYBINDINGS.md」の「Org mode」にあります。

- 見出し: `TAB` (見出しごとに FOLDED → CHILDREN → SUBTREE)、`S-TAB` (全体)、`#+STARTUP: overview` など。作る (`M-RET`、`C-RET`)、深さ (`M-左右`、`M-S-左右`)、動かす (`M-上下`)。
- TODO と優先度とタグ: `C-c C-t` (`(t)` などのキーがあれば、キーで選ぶ)、`S-左右`、`S-上下`、`C-c ,`、`C-c C-q`。`org-log-done` で CLOSED を書き、繰り返し (`+1w`、`++1m`、`.+1d`) の予定は日付を進めます。
- 日付と予定: `C-c .`、`C-c !`、`C-c C-s`、`C-c C-d`。日付は Emacs と同じく `+3`、`-1w`、`fri`、`10/30`、`3pm` などで入れられ、`S-上下左右` でカーソルの所 (年、月、日、時、分) を変えます。
- リストとチェックボックス: `M-RET`、`C-c C-c`、`C-c -`、`[2/3]` と `[66%]` を数え直す。
- 表: `|` で始まる行。`TAB`、`RET`、`C-c C-c` で揃え (数の多い列は右に。全角は 2 桁)、`M-左右上下`、`M-S-左右上下` で列と行を動かす、消す、足す。`C-c |` で作る。
- リンク: `[[file:a.org::*見出し][説明]]`、`C-c C-l` (入れる)、`C-c C-o` (開く)、`C-c l` (覚える)。
- アジェンダ (`C-c a`): 週の予定 (`a`)、TODO の一覧 (`t`)、タグ (`m`)、言葉 (`s`)。書き方は Emacs と同じです (`  work:        9:00...... 会議`、`Sched. 2x:`、`In   3 d.:` など)。
- キャプチャ (`C-c c`): `org-capture-templates` (Emacs と同じ書き方。`entry`、`item`、`checkitem`、`plain` と、`file`、`file+headline`) で書き、`C-c C-c` でファイルに足して保存する。
- 色: 見出し、TODO、タグ、日付、リンク、強調 (`*太字*`、`/斜体/` など) に、Emacs と同じ色を付けます。

#### テスト (ERT)

Emacs の ERT と同じく、`ert-deftest`、`should`、`should-not`、`should-error`、`skip-unless` でテストを書けます。`M-x ert` で、テストを走らせて、結果を `*ert*` に出します (`(ert-run-tests-batch)` は、結果を文字列で返す)。

```scheme
(ert-deftest my-test ()
  "足し算."
  (should (= (+ 1 2) 3))
  (should-error (car 1)))
```

howm のテストは `tests/test_howm.scm` です。`npm run test:scheme` (`cargo test --test lisp_ert_test`) で、`tests/*.scm` を全て走らせます。

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

- **CodeEditor.tsx** - エディタ (CodeMirror 6。Emacs のキーとコマンドで動かす)
- **Minibuffer.tsx** - ミニバッファ (メッセージ、入力と補完、isearch、query-replace の問い)
- **ModeLine.tsx** - モードライン (バッファの名前、変更、カーソルの位置、メジャーモード)
- **BufferList.tsx** - バッファの一覧 (`C-x C-b`)
- **ProjectTree.tsx** - 左のサイドバーの、プロジェクトのファイルのツリー (`F8`。ディレクトリは、開いたときに読む)
- **App.tsx** - 画面を出し、機能ごとのフックのコマンドを、キーと `M-x` とミニバッファにつなぐメインアプリコンポーネント
- **hooks/** - 機能ごとの状態とコマンド: ミニバッファ (`useMinibuffer`)、バッファとウィンドウ (`useBuffers`)、ファイル (`useFiles`)、dired と grep とプロジェクトのツリー (`useDirectories`)、検索と置換 (`useSearch`)、ブックマーク (`useBookmarks`)、評価と Lisp のコマンド (`useLisp`)
- **lib/theme.ts** - 色とフォント (CSS の変数。背景が暗いと `data-background-mode="dark"` にして、暗い背景の色分けを使う)

### バックエンド (Rust)

- **Parser** - Scheme 式をトークン化して AST にパース
- **Evaluator** - シンプルな環境/スコープシステムを使用して AST ノードを評価
- **search.rs** - `M-x grep` (正規表現に大文字が無ければ、大文字と小文字を区別しない。`.` で始まるディレクトリ、`node_modules`、`target` と、文字のファイルでないものは探さない) と、プロジェクトの根 (`.git` のある所)
- **Value** - Scheme 値を表現し、表示フォーマットを処理

### 通信

Tauri のコマンドシステムを使用して React から `eval_scheme` 関数を呼び出し、コード文字列を送信して結果/エラーを受け取ります。結果には、Lisp で作ったコマンドと、割り当てたキーも含みます。

評価の途中のエディタの操作 (`insert` など) は、評価器が `editor-request` のイベントで画面に頼み、画面が操作をして `editor_reply` で返事をします。評価は別のスレッドで行い、返事を待ちます (5 秒で失敗にする。`read-string` は 1 時間)。バッファを出す操作 (`find-file`、`switch-to-buffer`) は、画面を描き直してから返事をします (続きの操作を、出したバッファで行うため)。Lisp で作ったコマンドは `run_lisp_command`、初期設定ファイルは `load_init` で評価します。

## ライセンス

GPL v3.0 - 詳細は [LICENSE](./LICENSE) ファイルを参照してください。
