# Scheme エディタプロジェクトガイド

## 概要
Tauri + React + Rust で構築された, Emacs のように使えるエディタで、Scheme の組み込みインタプリタを備えています (評価は `C-x C-e` と `M-:`)。

## 技術スタック
- **フロントエンド**: React 18 + TypeScript + Vite
- **バックエンド**: Rust + Tauri 2
- **ビルド**: Cargo + npm
- **開発**: フロントエンドと Rust バックエンド両方のホットリロード対応

## アーキテクチャ
フロントエンド（`src/` 内の React コンポーネント）は、Rust の核が描く内容 (frame) を出し、キーと文字を核に送ります (ミニバッファの入力と補完、サイドバーも持つ)。バックエンドは Cargo のワークスペース（ルートの `Cargo.toml`）で、画面に頼まない核のクレート `core/`（Scheme インタプリタ、パーサ、評価器、原子的な書き込み。ターミナルの ncurses でも使う）と、Tauri のアプリ `src-tauri/`（画面のエディタのセッション、検索、ログ。`core/` を再公開する）に分けています。計画は FUNCTIONS.md の「実装の計画」。ターミナルのアプリは `tui/`（ncurses。`cargo run -p rseditor-tui -- ファイル` で動く。キーと画面と問い（ミニバッファ）は `tui/src/`、基本のコマンドは `lisp/simple.scm`）。エディタの核（`Buffer`, `Editor`, 探す, 構文, 窓）は `core/src/`、評価器が核を直接動かす窓口は `core/src/scheme/editor_host.rs`。

- エディタの本体は Rust の核 (`core/`) にある。バッファ, ウィンドウ, キーの割り当て (`core/src/keymap.rs`), コマンドループ (`core/src/command_loop.rs`。画面とターミナルで共通), 編集のコマンド (Lisp の `lisp/*.scm` と, `core/src/` の組み込み) は全て核が持つ。画面 (TypeScript) は, 核の描く内容を出し, キーと文字を核に送るだけ (設計は `docs/core_migration.md`)。
- 画面のエディタは `src-tauri/src/session.rs` の `Session` (エディタ, 評価器, コマンドループ)。最初の入力 (Tauri のコマンド `session_input`, `src-tauri/src/front.rs`) で `session` のスレッドを起こし, 入力 (`Input`: キー, 確定した文字, クリックの位置, 高さ, コマンド) を順に処理する。処理のたびに, 描く内容 (`core/src/frame.rs` の `Frame`: ウィンドウの位置 (%), カーソル, マーク, エコー領域, モードライン, バッファの本文の変更) を `frame` のイベントで送る。本文は, 初めと読み直したときだけ全体, ほかは変更 (`TextChange`) だけ。入力が 30 秒 (`AUTO_SAVE_TIMEOUT`) 無ければ, 自動保存する。
- 画面にしかできないこと (ミニバッファで聞く, `read-key`, ウィンドウの見えている行, `recenter`, 開け閉め, 色とフォント, サイドバー) は, 核が `Bridge` (`core/src/scheme/host.rs`) で頼み (`editor-request` のイベント), 画面 (`src/App.tsx` の `perform`) が `editor_reply` で返事をする。キーの名前は Emacs の書き方 (`C-x`, `M-f`, `RET`。`src/lib/keys.ts` の `emacsKey`)。
- 画面の部品: `src/App.tsx` (frame を受けて, ウィンドウ, モードライン, ミニバッファを並べる), `src/components/WindowView.tsx` (ウィンドウ 1 つ。CodeMirror 6 で, 本文を出すだけ。編集は止め (`remote` の印の無い変更を捨てる), IME の確定した文字とクリックの位置を核に送る), `src/lib/docs.ts` (バッファの本文 (CodeMirror の `Text`) と, 前の frame からの変更 (`ChangeSet`)。ウィンドウは, 変更だけを当てる), `src/lib/session.ts` (frame の型, 入力を送る, 頼みを受ける), `src/components/Minibuffer.tsx` (ミニバッファの入力, 補完 (`src/lib/complete.ts`), 履歴 (`src/lib/history.ts`)), `src/components/ModeLine.tsx`。
- シンタックスハイライトは, CodeMirror の `StreamLanguage` で, Emacs 30 の scheme-mode と emacs-lisp-mode と同じ色分けにする (`src/lib/lispMode.ts`)。決まりは, Emacs の font-lock の変数 (`scheme-font-lock-keywords-2`, `lisp-el-font-lock-keywords-2`) から取り, `emacs -Q --batch` で付けた色と比べて確かめる。Emacs Lisp のマクロの名前は `src/lib/elispKeywords.ts` (`emacs -Q` から作る)。メジャーモードの名前と色付けは `src/lib/modes.ts`。括弧の強調 (show-paren-mode) は `src/lib/showParen.ts` (括弧を読むのは `src/lib/lispParser.ts`)。`highlight-regexp` の色付けは `src/lib/faces.ts`。
- 字下げ (`core/src/indent.rs`), S 式と構文 (`core/src/sexp.rs`, `core/src/syntax.rs`), 段落 (`core/src/paragraph.rs`), 段落を詰める (`core/src/fill.rs`), kill-ring (`core/src/kill_ring.rs`), キーボードマクロ (`core/src/kbd_macro.rs`) は核にある。`emacs -Q --batch` の結果 (`core/tests/*.json`) と比べて確かめる。
- ファイルのコマンド (`C-x C-f`, `C-x C-s`, `C-x C-v`, `C-x i`, `C-x s`, `revert-buffer`, `recover-this-file`) は Lisp (`lisp/files.scm`)。分け方は Emacs と同じ: 組み込み (Rust の `core/src/files.rs`。Emacs の C に当たる) は, 本文とファイルの間で読み書きする `insert-file-contents`, `write-region` (原子的に書く。`core/src/atomic.rs` の `write_atomic`) と, 自動保存 (`#名前#`) の `do-auto-save` だけ。ファイルを開くこと (`find-file-noselect`。方言は `set-auto-mode` と `auto-mode-alist`), 保存すること (`basic-save-buffer`), バックアップ (`backup-buffer`。訪れてから初めて保存する前に `名前~` に写す), 自動保存のファイルを消すこと, フック (`find-file-hook`, `after-save-hook`) は Lisp。dired (`C-x d`) は Lisp (`lisp/dired.scm`。一覧は `ls -al` の出力から作る)。ブックマークは `lisp/bookmark.scm` (保存は `~/.rseditor.d/bookmarks`)。
- Lisp の評価: 評価器は, エディタの操作を `Host` (`core/src/scheme/host.rs`) に頼む。核の操作は `EditorHost` (`core/src/scheme/editor_host.rs`) が核を直接動かし, 画面の要る操作だけを画面 (アプリは `Bridge`, ターミナルは `tui/src/ui.rs`, テストは偽の `Host`) に渡す。位置は Emacs と同じく 1 から数え, 文字 (Unicode のコードポイント) で数える (画面に送る frame の位置は UTF-16)。フック (`add-hook`, `run-hooks`) は `core/src/scheme/hooks.rs`。
- バッチモード (`rseditor-tui --batch`。`emacs --batch` と同じ: `-l`, `--eval`, `-f`, `--script`, ファイル。実装は `tui/src/batch.rs`) は、画面を使わずに引数の順に動かす。`princ` などは標準出力 (出力先は `EditorHost::set_output`)、`message` は標準エラー出力、`(kill-emacs N)` で終了コード、エラーは 255。Emacs との比較試験は `tests/batch/run.sh` (`npm run test:batch`。仕様は `docs/batch_test.md`)。
- `Mutex` は `.lock_recover()` (`core/src/sync.rs` の `LockExt`) で取る (`lock().unwrap()` は使わない)。評価は別のスレッドで動かし、panic は評価器が捕まえるので、ロックが poisoned のままだと、以後のキーが全て失敗するため。
- 自動保存は、入力が 30 秒止まったとき (画面のアプリ) と、画面から読んだキーが 300 個になったとき (`files::note_key`。画面とターミナルの両方) に行う。キーボードマクロの繰り返しは、`C-g` で止まる (画面のアプリは `Front::interrupted`、ターミナルは `Screen::interrupted`)。保存の前に、ファイルの更新時刻 (`verify-visited-file-modtime`) を確かめる。
- 初期設定ファイルは `~/.rseditor` (無ければ `~/.rseditor.d/init.scm`, それも無ければ `~/.config/rseditor/init.scm`。読んだファイルは `user-init-file`。画面のエディタは `Session::load_init` で読み, 失敗は `*Warnings*` とエコー領域に出す)。ファイル, 時刻, 正規表現の組み込みの手続き (`load`, `require`, `read-file-string`, `regexp-match-positions`, `make-temp-file`, `grep` など) は `core/src/scheme/system.rs`。`load` と `require` は `load-path` (始めは `~/.rseditor.d/lisp`, `~/.rseditor.d`) から探す。色とフォント (`set-background-color`, `set-frame-font`) は `src/lib/theme.ts` で, CSS の変数 (`src/index.css`) に置く (背景が暗いと, Emacs の暗い背景の face の色にする)。
- `M-x grep` は外のコマンドを使わず, Lisp (`lisp/compile.scm`) が `rust-grep` (core の `system.rs`) で探す。結果のバッファ (`*grep*`) もここで作り, `RET` は `local-set-key`, `M-g n`, `M-g p` は `next-error` など。Tauri も TUI も `loadup` で読む。
- サイドバーのプロジェクトのツリー (`F8`。Lisp のコマンド `project-tree` が画面に `toggle-project-tree` を頼む) は `src/components/ProjectTree.tsx`。根は `search.rs` の `project_root` (`.git` のある所)。押してもフォーカスを取らない。
- howm の Scheme 版は `lisp/howm.scm` (`(require 'howm)` で読む)。テストは `core/tests/test_howm.rs` と `tests/lisp/test_howm.scm` (どちらも偽のエディタ (`core/tests/common/mod.rs`) の上で動かす)。
- org-lint の Scheme 版は `lisp/org-lint.scm` (`(require 'org-lint)`)。構文木を作らず、行を種類に分けて (`org-lint--scan`)、その上でチェッカーを動かす。移していないチェッカーは `org-lint-unsupported-checkers`。Emacs の結果との比較は `tests/org-lint/run.sh` (`npm run test:org-lint`)、テストは `core/tests/test_org_lint.rs` と `tests/lisp/test_org_lint.scm`。
- シンタックスハイライト (C) は、核の `core/src/highlight/` (字句解析器 `mod.rs` と、言語の表 `lang.rs`)。`c-mode` は、方言を増やさず、`Buffer::mode_name` が `c-mode` で決める。結果は `Buffer::token_spans` (本文の版で覚える)、Lisp の `(token-spans)`、画面 (Tauri) は `BufferFrame.tokens` と `src/lib/faces.ts` の `tokenHighlighting`、TUI は `Cell.token` と `tui/src/screen.rs` の `token_style`。Emacs の `c-mode` との比較は `tests/c-highlight/run.sh` (`npm run test:c-highlight`)、設計は `docs/c_highlight_design.md`。
- Org mode の Scheme 版は `lisp/org.scm` (`(require 'org)` で読む。Emacs の org 9.7.11 の動きに合わせる)。テストは `core/tests/test_org.rs` と `tests/lisp/test_org.scm`。色は `highlight-regexp` の face (`src/styles/CodeEditor.css` の `cm-face-*`)、開け閉めは `outline-flag-region` (画面の CodeMirror の fold)。
- ERT (`ert-deftest`, `should` など) は `core/src/scheme/ert.rs`。特別な形は `ert::special_form`, `M-x ert` のコマンドは Scheme の `PRELUDE` で定義する。`tests/*.scm` は `core/tests/test_lisp_ert.rs` が走らせる (`npm run test:scheme`)。
- 起動のときに読む Lisp (Emacs の loadup.el に当たる) は `core/src/scheme/loadup.rs` に並べる (`lisp/subr.scm` (`y-or-n-p` など), `lisp/window.scm` (`switch-to-buffer`), `lisp/files.scm` (`find-file`, `save-buffer` など), `lisp/help-fns.scm` (`C-h f`))。ビルドのときに埋め込み, `Evaluator::new` で評価する。コマンドは, 引数を省くと (キーと M-x から呼ぶと), ミニバッファで聞く (`read-file-name` など)。
- docstring は英語で書く (関数は引数の後の文字列, 変数は `(define name value "doc")` の 3 つ目)。翻訳は `po/ja.po` (gettext の PO 形式. `msgctxt` は `function:名前` か `variable:名前`)。docstring を変えたら `npm run po:update` (`tools/po/update.py`) で `po/rseditor.pot` と各言語の PO を作り直し, 訳す (変わった翻訳は `fuzzy` になる)。`npm run check` の `po:check` が, 作り直し忘れを見つける。手順は `PO.md`。Rust は `core/src/scheme/po.rs` で PO を読み, `documentation` が変数 `help-language` の言語の翻訳を返す。ヘルプの組み込みの手続き (`documentation`, `help-function-arglist`, `mapatoms`, `symbol-file` など) は `core/src/scheme/help.rs`。
- `FUNCTIONS.md` の「Emacs の組み込みのコマンドと, エディタの基本のコマンド」は, `tools/emacs-commands/run.sh` で作る (`emacs -Q --batch` で, C で書いたコマンドと, エディタの基本の Lisp のコマンドと, 既定のキーを書き出し, rs-editor で実装済みかを書く)。印 (`<!-- emacs-commands:begin -->`) の間は, 手で直さない。
- `src/` 全体のカバレッジを 100% に保つ (`vite.config.ts` の `thresholds`)。Rust (`core/` と `src-tauri/`) も 100% に保つ (`npm run coverage` の `cargo tarpaulin --workspace --fail-under 100`。CI の `coverage` のジョブも同じ)。フロントエンドのテストは `tests/frontend/` に, `src/` と同じ構成で置く (Lisp の ERT のテストは `tests/lisp/`) (`src/lib/keys.ts` のテストは `tests/frontend/lib/keys.test.ts`。`src/` は `@/` で参照する)。Rust のテストは各クレートの `tests/` (`core/tests/`, `src-tauri/tests/`) に置き, `lib.rs` から公開した関数を呼ぶ。CodeMirror のテストは, jsdom の上で, 実際の `EditorView` を作って行う。

## 開発ワークフロー
- **開発サーバーを起動**: `npm run tauri:dev` — フロントエンドと Rust は両方とも変更時に自動リロード
- **テスト**: `npm run test`（`tests/frontend/` の Vitest）と `npm run test:scheme`（`tests/lisp/*.scm` の ERT）と `cargo test --workspace`
- **環境の準備**: Docker を使わないときに必要な道具とパッケージは `setup.md`。
- **Markdown のチェック**: `npm run format:markdown:check` (`markdownlint-cli2`。直すときは `npm run format:markdown`)。`npx markdownlint` は別のパッケージ名なので動かない。`npm install` をしていない環境では, `sudo npm install -g markdownlint-cli2` で入れて, `markdownlint-cli2 docs/functions.md` のようにファイルを指定して動かす。
- **フォーマット + リント**: `npm run check`（format:check、lint、test を実行 — CI ゲート）
- CI は `main` へのすべてのプッシュ/PR で同じ内容を実行

## 機能の追加

### Scheme 組み込み関数の追加
`core/src/scheme/builtins.rs` を編集:
1. `NAMES` に名前を追加 (評価器は, 起動のときに, この名前を一番外側の環境に定義する)
2. `call()` 関数内に新しい match 分岐を追加 (引数は `fixed`, `number`, `string` などで取り出す)
3. `Value` 結果またはエラー文字列を返す
4. 特別な形 (引数を評価しないもの) は, `core/src/scheme/evaluator.rs` の `special_form()` に追加

### UI の更新
`src/` 内のコンポーネントを編集:
1. コンポーネントは `src/components/` に配置
2. スタイルは `src/styles/` に配置
3. エディタのコマンドは画面に置かない (Lisp か核に置く)。画面にしかできない操作は, `App.tsx` の `perform` に足す

### Tauri コマンド
React から呼び出し可能な新しいバックエンド関数を追加:
1. `src-tauri/src/commands.rs` に `#[tauri::command]` 関数を追加 (本体は普通の関数にして, `src-tauri/tests/` から呼べるようにする)
2. `src-tauri/src/lib.rs` の `.invoke_handler(tauri::generate_handler![...])` リストに追加
3. React から呼び出し: `await invoke('function_name', { arg: value })` (`src/lib/fileOps.ts` や `src/lib/session.ts` のように, 関数で包む)

## 現在の実装状況

### 実装済み
- ✅ パーサ: リスト、シンボル、数値、文字列、ブール値、`'`
- ✅ 算術演算と比較: +, -, *, /, =, <, >, <=, >=, abs, min, max, quotient, remainder, modulo
- ✅ 制御フロー: if, cond, case, and, or, when, unless
- ✅ 変数バインディング: define, set!, let (名前付きも), let*, letrec
- ✅ lambda (残りの引数も)、末尾の呼び出し
- ✅ リスト操作: quote, list, car, cdr, cons, append, length, map など
- ✅ 型チェック: number?, string?, symbol? など
- ✅ エディタの操作と, Lisp でのカスタマイズ (define-command, global-set-key, local-set-key, 初期設定ファイル `~/.rseditor`, load)
- ✅ ファイル, 時刻, 正規表現, 文字列の組み込みの手続きと, 色とフォントの設定
- ✅ howm の Scheme 版 (`lisp/howm.scm`) と, ERT (Scheme のテスト)
- ✅ Org mode の Scheme 版 (`lisp/org.scm`)
- ✅ `M-x grep` (エディタが探す) と, サイドバーのプロジェクトのツリー
- ✅ 評価: カーソルの前の S 式 (`C-x C-e`) と, ミニバッファに入力した式 (`M-:`)。結果はミニバッファに出す
- ✅ 画面のエディタを Rust の核で動かす (画面は frame を描き, キーを送るだけ)
- ✅ エラーハンドリングと表示
- ✅ ダークテーマ UI
- ✅ シンタックスハイライト (Emacs の scheme-mode と emacs-lisp-mode と同じ色分け) と, ファイルの名前の補完
- ✅ 字下げ (Emacs の scheme-mode と emacs-lisp-mode と同じ) と, 括弧の強調

### TODO
- [ ] マクロ: define-syntax
- [ ] ドット対 (リストは, 今は要素の配列を共有する `List` (`value.rs`) で持つ。写すこと, `cdr`, `cons` (前の空きを使う) は, 配列を写さない)
- [ ] エディタ内のコメント（既にパースされているが）
- [ ] 行番号付きのより良いエラーメッセージ

## ビルドと配布

```bash
# 開発
npm run tauri:dev

# 現在のプラットフォーム用にビルド
npm run tauri:build

# ビルドされたアプリの場所
target/release/rseditor  # または Windows では .exe、macOS では .app
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
  - scope は、変更の場所: `editor` (`src/` のエディタの画面)、`scheme` (`core/src/scheme/` の Scheme の処理系)、`backend` (`src-tauri/` のそれ以外)、`deps` (依存パッケージの更新)。複数にまたがるときは、省略する。
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

## テストの規約

- **Rust のテストは, 各クレートの `tests/` (`core/tests/`, `tui/tests/`, `src-tauri/tests/`) にファイルを分けて置く** (本番のソースの中に `#[cfg(test)] mod tests` を書かない). ファイルの名前は `test_<対象のモジュール>.rs`. テストが呼ぶ関数は `pub` にする.

## 開発ルール

- Markdown (`*.md`) の日本語の文章は、句読点に「、」「。」を使う (「,」「.」は使わない)。インラインコード・コードブロック・URL・ファイル名・数値 (`1.2.0`、`1,000`) は対象外。見出しや番号付きリストの「1.」の形は変えない。`docs/` の生成物と、この `CLAUDE.md` は対象外。
- Markdown (`*.md`。この `CLAUDE.md` を除く) には、AI の目線の記述を書かない。読み手 (人) に向けて、事実、仕様、手順だけを書く。「承認されたら実装します」「教えてください」「提案します」のような、AI からの依頼や約束、作業の報告、「Claude が」のような書き手の記述は、会話で伝えて、ファイルには残さない。
