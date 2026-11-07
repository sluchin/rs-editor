# TODO: Emacs 相当機能の実装手順

このファイルは、rseditor を Emacs ライクなエディタに近づけるための実装ステップをまとめたもの。
`.claude/CLAUDE.md` に記載の TODO（演算子追加、制御構文など Scheme インタプリタ側）とは別に、
**エディタ UI / キーバインド側**の作業をフェーズ分けして記載する。

## 現状整理

- Phase 1 から Phase 13 までと、Phase 15 の必須の項目は、実装した ([x])。Phase 13 は、全て完了。Phase 12 は、実装したコマンドの、Emacs 30 と違うところ。
- エディタは CodeMirror 6 (Phase 8)。Phase 1 から Phase 7 の、`<textarea>` と `prefixKeymap.ts`、`emacsKeymap.ts` などの記述は、当時のもの
  (今は `src/components/CodeEditor.tsx` と `src/lib/emacs.ts`)。
- まだ無い Emacs の標準機能は、Phase 14 に並べた。

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

(この Phase の `<textarea>` と `<pre>` の作りは、Phase 8 で CodeMirror 6 に置き換え、色分けは Phase 10 で Emacs と同じにした。)

- [x] `CodeEditor.tsx` を「透明な `<textarea>` を実入力に使い、背後に色付き `<pre>` を重ねる」構成に変更する
      （スクロール位置を `onScroll` で同期）。既存の `bracketAutoClose` / `emacsKeymap` はそのまま `<textarea>` に効かせる。
- [x] `schemeTokenizer.tokenize()` の結果を使い、トークン種別ごとに `<span>` でラップして色付けする
      （`keyword` / `string` / `comment` / `number` / `boolean` / `paren` / `symbol`）。
- [x] `bracketMatch.findMatchingBracket()` を使い、カーソルに隣接する括弧とその対応括弧をハイライトする
      （Emacs の `show-paren-mode` 相当）。
- [x] `bracketMatch.findUnmatchedClose()` を使い、閉じ括弧の対応が取れていない箇所をエラー表示する。
- [x] パフォーマンス確認: 大きめのバッファでも `tokenize()` が入力毎に重くならないか（必要なら debounce）。

## Phase 3: ミニバッファ / コマンド実行の拡張

- [x] `Minibuffer.tsx` の `MinibufferState` に `mode: 'isearch'` を追加し、`C-s` / `C-r` によるインクリメンタル検索を実装する。
      - マッチ箇所へのカーソル移動、`C-s` 連打で次のマッチへ、`C-g` でキャンセルして元の位置に戻る。
- [x] `M-x` (execute-extended-command) を実装する。
      - コマンド名を文字列で受け取り、内部の「コマンドレジストリ」（`{name, run}` のマップ）から実行する。
      - 最初は `find-file` / `save-buffer` / `save-buffers-kill-terminal` 程度から始め、Phase 4/5 のバッファ/REPL 系コマンドを順次登録する。
- [x] `M-x` と `C-x b` の補完（前方一致する候補の、共通する先頭の部分まで Tab で補完し、候補が複数なら一覧を出す）。
- [x] `keyboard-quit` (`C-g`) 実行時に `Quit` メッセージをミニバッファに表示する。

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
- [x] ウィンドウ分割: `C-x 2` (上下に分ける) / `C-x 3` (左右に分ける) / `C-x o` (他ウィンドウへ移動) / `C-x 0` / `C-x 1`。同じバッファを出すウィンドウには、書き換えを写す。
      React 側は「ペインのレイアウトツリー」を持たせ、各ペインに独立した `CodeEditor` インスタンス（同一バッファ参照可）を描画する。

## Phase 6: REPL / Scheme 統合

- [x] `REPL.tsx` を `App.tsx` にマウントする（下ペイン、もしくは `C-x C-e` 実行結果をミニバッファ/エコーエリアに出す方式でも可）。
- [x] `C-x C-e` (eval-last-sexp): カーソル直前の S 式を `bracketMatch` で特定し、`eval_scheme` コマンド（`src-tauri/src/main.rs`）
      に渡して評価、結果をミニバッファに表示する。
- [x] `M-:` (eval-expression): ミニバッファに Scheme 式を直接入力して評価するモードを追加（Phase 3 の `M-x` 基盤を流用）。
- [x] REPL 履歴とエディタの kill-ring / undo 履歴が競合しないよう、フォーカス管理（テキストエリア vs REPL 入力欄）を整理する。

## Phase 7: モードライン / 仕上げ

- [x] `ModeLine.tsx` に現在の mark 状態（リージョン選択中かどうか）、isearch 状態などを表示する。
- [x] `C-h k` (describe-key) でキーの説明をミニバッファに出し、`C-h b` (describe-bindings) でキーの割り当ての一覧を `*Help*` バッファに出す。
- [x] キーバインドの一覧をドキュメント化し `README.md` に追記する。
- [x] 各フェーズの hook ごとに Vitest でのテストを追加し、`npm run check` を通すこと。

## Phase 8: CodeMirror 6 への置き換えと、足りない Emacs のキー

- [x] エディタを `<textarea>` から CodeMirror 6 に置き換える (`CodeEditor.tsx`)。バッファごとにエディタの状態 (カーソル、元に戻す履歴) を持つ。
- [x] Emacs のコマンドを `src/lib/emacs.ts` にまとめる (マーク、直前のコマンド、kill-ring)。`C-x` 系のキーは、CodeMirror のキーの並びで割り当て、`prefixKeymap.ts` をなくす。
- [x] `C-v` / `M-v`、`C-l`、`M-g g`、`M-d` / `M-DEL`、`M-u` / `M-l` / `M-c`、`C-o`、`C-x C-x`、`C-x C-c` を足す。
- [x] 続けたキルを 1 つにつなげる。`C-SPC` の後の移動で、範囲を広げる (transient-mark-mode)。
- [x] 置換 (`M-%`、`C-M-%`、`replace-string`、`replace-regexp`) と正規表現の検索 (`C-M-s`、`C-M-r`)。
- [x] 前置引数 (`C-u`、`M-0` から `M-9`) とキーボードマクロ (`C-x (`、`C-x )`、`C-x e`)。

## Phase 9: Lisp でのカスタマイズ

- [x] Scheme の処理系に、言語の核を足す: `define`、`lambda`、`let` (名前付きも), `let*`、`letrec`、`if` / `cond` / `case`、`and` / `or` / `when` / `unless`、比較、文字列とリストの操作。
      定義は、アプリが持ち続ける処理系 (`Evaluator`) の環境に覚える。末尾の呼び出しは、深さを使わない。
- [x] Lisp からエディタを操作する関数を用意する: `point`、`point-min`、`point-max`、`goto-char`、`insert`、`delete-region`、`buffer-string`、`buffer-substring`、`buffer-name`、`message`。
      評価は Rust で行い、エディタの操作は、画面に頼んで返事を待つ (`core/src/scheme/host.rs` と `src/lib/lisp.ts`)。
- [x] Lisp から、既にあるコマンドを名前で呼べるようにする (`call-interactively`、`command-execute`)。
- [x] Lisp で定義した関数を、コマンドとして登録する (`define-command`。`M-x` から呼べる)。
- [x] キーの割り当てを Lisp から変える (`global-set-key`)。
- [x] 起動のときに、初期設定ファイル (`~/.rseditor`。無ければ `~/.config/rseditor/init.scm`) を読んで評価する。誤りがあっても起動は続け、ミニバッファに知らせる。

## Phase 10: ファイルの名前の補完と、シンタックスハイライト

- [x] `C-x C-f` と保存先 (`C-x C-s` のファイルのないバッファ) で、Tab でファイルの名前を補完する。
      最後の `/` の後ろを、そのディレクトリの中の名前 (Rust の `list_directory`) で補完し、ディレクトリには `/` を付ける。
- [x] シンタックスハイライト (Emacs の font-lock)。Emacs 30 の scheme-mode と emacs-lisp-mode と同じ所に、Emacs の既定の色を付ける
      (今は核の `core/src/highlight/lisp.rs`。以前は `src/lib/lispMode.ts`)。コメント、文字列、`(` の直後の特別な形、定義する名前、`:key` など。Emacs Lisp では、      文字列の中の正規表現のグループと要らない `\`、閉じた括弧の後ろに続く引数の警告も。`emacs -Q --batch` で付けた色と、1 文字ずつ比べて確かめた。
- [x] メジャーモード (核の `lisp/files.scm` の `auto-mode-alist` と `core/src/mode.rs`): Emacs の `auto-mode-alist` と同じく、`*scratch*` は Lisp Interaction, `.scm` などは Scheme,
      `.el` と `.emacs` などは ELisp/l か ELisp/d, `.dir-locals.el` などは Lisp-Data (この 4 つは色を付ける), ほかは Fundamental。
      モードラインに名前を出す。

## Phase 11: 字下げと、括弧の強調

- [x] 字下げ (`src/lib/lispIndent.ts`): Emacs 30 の `calculate-lisp-indent` と `lisp-indent-function` (`scheme-indent-function`) と同じ。
      フォームごとの指定は、Emacs の `lisp-indent-function` と `scheme-indent-function` の値から作る (`src/lib/lispIndentSpecs.ts`)。
      `RET` (electric-indent-mode), `C-j`、`TAB` (Fundamental モードでは indent-relative), `C-M-q`、`C-M-\`。
      `emacs -Q --batch` の `indent-region` と、1 バイトずつ比べて確かめた。
- [x] タブを使うかと、タブの幅 (Emacs の既定は、タブを使い、幅は 8)。ファイルの 1 行目の `-*- ... -*-` の
      `mode`、`lexical-binding`、`indent-tabs-mode`、`tab-width` を読む。
- [x] 括弧の強調 (`src/lib/showParen.ts`. Emacs の show-paren-mode): カーソルの前の閉じる括弧か、後ろの開く括弧と、対に色を付ける。
      対が無いか、種類が違えば、合わない色にする。

## Phase 12: Emacs と違うところ

実装したコマンドのうち、Emacs 30 と動きが違うところ。

- [x] `M-q` (fill-paragraph): Lisp の文字列 (docstring) の中を詰める。
- [x] `M-q` (fill-paragraph): リージョンがあれば、リージョンの全ての段落を詰める (Emacs の `fill-region`)。
- [x] 前置引数を、繰り返しでなく、引数として渡す。
  - [x] `M-SPC` (just-one-space): `C-u 3 M-SPC` で空白を 3 つ残す。負の数なら、改行も消す。
  - [x] `M-\` (delete-horizontal-space): 前置引数があれば、カーソルの前の空白だけを消す。
  - [x] `M-^` (delete-indentation): 前置引数があれば、次の行とつなげる。リージョンがあれば、リージョンの行をつなげる。
- [x] `C-x C-u` (upcase-region) と `C-x C-l` (downcase-region): Emacs と同じく、既定で使えない (disabled) コマンドで、初めて押すと使うかを確かめる (`n` `y` `SPC` `!`)。確かめる画面は、`*Disabled Command*` のバッファでなく、エコー領域の問い。`y` で永続的に有効にする (初期設定ファイルへの書き込み) はしない。
- [x] `C-q` (quoted-insert): `C-q` の後の、`C-x`、`C-h`、`M-g` などの前置きのキーを、制御文字としてそのまま入れる。
- [x] `C-q RET` と `C-q C-m`: Emacs と同じく `^M` (CR) を入れる。
- [x] `C-q` の後の `M-a` などの、Meta を付けたキー: Emacs と同じく、8 ビット目を立てた文字を入れる。
- [ ] `C-u M-=` (count-words-region): 前置引数があれば、バッファ全体を数える。
- [x] `M-f`、`M-b` などの単語: Emacs と同じく、書記体系で単語を切る (日本語は、漢字と送りがなを 1 つの単語にする、など。`tests/batch/cases/56-word-motion`)。
- [x] kill-ring, 消した矩形 (`C-x r k`), レジスタ: Emacs と同じく、全てのウィンドウで共通。
- [x] `C-x r y` (yank-rectangle) と `C-x r i` (insert-register): Emacs は、入れた範囲の端にマークを置く (リージョンは選ばない)。
- [ ] 矩形とレジスタの前置引数: `C-u C-x r k` (短い行も空白で埋める), `C-u C-x r s` (リージョンを消す),
      `C-u C-x r i` (カーソルを入れた文字の後ろに置く)。
- [ ] `C-x r t` (string-rectangle): Emacs は、入力しているあいだ、置き換えた結果を見せる (`rectangle-preview`)。
- [ ] ブックマーク: Emacs は、`~/.emacs.d/bookmarks` に、Lisp の形で、終わるときに保存する。
      今は、`~/.config/rseditor/bookmarks` に、JSON で、付けるたびに保存する。位置は、Emacs と違い、0 から数える (UTF-16 の位置)。
- [ ] `C-x r b` (bookmark-jump): ファイルが無いとき、Emacs は、新しい場所を聞く。今は、読めないと知らせるだけ。
- [x] `C-M-a` (beginning-of-defun) と `C-M-e` (end-of-defun): Emacs は、動く前の位置にマークを置く (リージョンは選ばない)。
- [x] `C-x C-n` (set-goal-column): Emacs と同じく、既定で使えない (disabled) コマンド。
- [x] `C-n`、`C-p` (`C-x C-n` で桁を決めていないとき): Emacs と同じく、表示の桁 (タブと全角の文字の幅) を保つ (`tests/batch/cases/55-line-column`)。
- [ ] Scheme の `#;` でコメントにした式と、`#/.../` (正規表現): Emacs の scheme-mode は、式の移動 (`C-M-f`、`C-M-u` など) でも、      コメントと文字列として読む。今は、字下げと括弧の強調だけで、そう読む。
- [x] `M-@`、`C-M-SPC`、`C-M-h` の負の前置引数 (前へ選ぶ)。
- [ ] `M-SPC`: Linux のデスクトップ (GNOME など) は、Alt+Space をウィンドウのメニューに使うので、エディタにキーが届かないことがある。
      Emacs と同じく、`ESC SPC` でも動くようにするか、届かないことを README に書く。
- [x] `C-x i` (insert-file): Emacs は、入れた文字の終わりにマークを置く (リージョンは選ばない)。
- [ ] `C-x s` (save-some-buffers): Emacs の問いは、`C-r` (バッファを見る), `C-f`、`d` (違いを見る), `C-h` も受ける。
      バッファの名前とファイルの名前が違えば、両方を出す。
- [x] 自動保存: Emacs は、300 回キーを押すごとにも保存する (`auto-save-interval`)。今は、書き換えが 30 秒止まったときだけ。
- [x] ターミナルのアプリも、入力が 30 秒止まったときに自動保存する (`Screen::wait_key`)。
      バッファを消すときと、Emacs を終えるときの扱い (`delete-auto-save-files`) も、まだ。
- [ ] バックアップ: Emacs は、前の版の名前を変えて `名前~` にする (`backup-by-copying` が `nil`)。今は、写す。
- [ ] `recover-this-file`: Emacs の問いは、`diff` (違いを見る) も受け、自動保存のファイルの一覧を見せる。
- [ ] `C-x d` (dired): 一覧を出すだけで、dired の中のキー (`RET`、`^`、`g`、`m`、`d` など) と、読み込み専用は、まだ。
      Emacs 30 は、始めの行に空きの量 (`available`) を出す。今は、出さない。
- [x] isearch の `M-p`、`M-n`: Emacs (`search-ring-update` が `nil`) と同じく、履歴の文字を、ミニバッファで直させる (`tests/batch/cases/59-isearch-edit`)。
- [x] ミニバッファの `M-n`: Emacs は、履歴の先で、既定の値 (`C-x C-f` なら、今のファイルの名前など) を出す。今は、出さない。
- [ ] `M-x grep`: Emacs は、grep のコマンドの行 (`grep --color=auto -nH --null -e`) を聞き、外の grep を動かす。今は、正規表現とディレクトリを聞き、      エディタが、ディレクトリの下を深くまで探す (`rgrep` に近い)。正規表現は Rust の regex の書き方。大文字が無ければ、大文字と小文字を区別しない
      (Emacs の `rgrep` の `case-fold-search` と同じ)。`.` で始まるディレクトリ、`node_modules`、`target` と、文字のファイルでないものは、探さない。
      既定の正規表現 (カーソルの所の言葉) は、まだ。

## Phase 13: 初期設定ファイルと howm

- [x] 初期設定ファイルを `~/.rseditor` にする (無ければ、前の `~/.config/rseditor/init.scm`)。
- [x] 色とフォントを、初期設定ファイルで変える: `set-background-color`、`set-foreground-color`、`set-frame-font`、`set-face-attribute` (`default` の `:family`、`:height`、`:foreground`、`:background`)。
      背景が暗いと、色分けを Emacs の暗い背景の face の色にする (`src/lib/theme.ts`、`src/index.css`)。
- [x] Lisp の手続きを足す: `load`、ファイル (`read-file-string`、`write-file-string`、`directory-files-recursively` など), 時刻 (`format-time-string`、`date-to-time`),
      正規表現 (`regexp-match`、`grep`), 文字列 (`string-search`、`split-string` など), `sort`、`cadr` など、`random`。
      エディタの操作: `find-file`、`switch-to-buffer`、`buffer-file-name`、`set-buffer-modified-p`、`read-string`、`local-set-key`。
- [x] howm を Scheme で書き直す (`lisp/howm.scm`)。`M-x howm` でメニュー、メモを作る、一覧、検索、今日と昨日、予定と todo, リンク (`>>>`、`<<<`)。
- [x] howm の、まだ移していない機能: 下線 (action-lock の色付け), 一覧の窓 (中身の表示), `[複製]`、`[速記]`、ローマ字検索 (migemo), `[前後]`、`[履歴]`、`[酔歩]`、      キーワードの補完、日付の入力 (`[日↓]`、`[時↓]`), タイトルとファイル名のコピー、`[全消]`、メニューの `%eval%`、`%call%`、別のメニュー (`%○○%`),
      保存したときのメニューの更新 (`after-save-hook`), `C-x C-f` で開いたメモの howm-mode (`find-file-hook`)。
      フック (`add-hook`、`remove-hook`、`run-hooks`、`run-hook-with-args`) を足し、エディタが、ファイルを開いたバッファを初めて出したときと、保存したときに実行する。
- [x] `read-string` の補完と、既定の値 (Emacs の `completing-read`)。`read-string` と `completing-read` は、Emacs と同じ引数
      (`completing-read` の PREDICATE, REQUIRE-MATCH (候補に無ければ `[No match]` で聞き直す), INITIAL-INPUT, DEF。候補は文字列、記号、alist)。
- [x] Lisp の文字列の位置は文字の数、画面の位置は UTF-16 の数で数える (絵文字など、BMP の外の文字があると、ずれる)。
      `performEditorRequest` (`src/lib/lisp.ts`) で、文字の数と UTF-16 の位置を変える (`charCount`、`utf16Offset`)。

## Phase 14: Emacs の標準機能で、まだ無いもの

`emacs -Q` (Emacs 30.2) の既定のキーの割り当て (`global-map`) と比べて、rs-editor に無いもの (2026-10-25 に洗い出した)。
マウス、メニュー、ファンクションキー、入力メソッド、文字コード、フレーム、タブは、除いた。
キーの無いコマンドと、isearch, ミニバッファ、query-replace の中のキーも加えた。

### 移動

- [x] `C-M-n` / `C-M-p` (forward-list / backward-list): リストの単位の移動。
- [x] `C-x [` / `C-x ]` (backward-page / forward-page), `C-x C-p` (mark-page), `C-x l` (count-lines-page): ページ (`^L`) の単位。
- [x] `M-r` (move-to-window-line-top-bottom): カーソルを、画面の中、上、下の行へ。
- [x] `M-g c` (goto-char), `M-g TAB` (move-to-column): 位置、桁へ (`tests/batch/cases/60-key-commands`)。
- [x] `M-g n` / `M-g p` / `` C-x ` `` (next-error / previous-error): `M-x grep` の結果の次・前へ (Phase 15)。
- [ ] `next-error` で、`compile` と `occur` の結果も使う。`*grep*` の中の、今の行の印 (`=>`) と強調。
- [ ] `M-g i` (imenu): 定義の一覧から選んで移る。
- [ ] `C-M-l` (reposition-window), `C-x <` / `C-x >` (scroll-left / scroll-right): 画面の位置。
- [x] `C-x C-@` (pop-global-mark)。
- [x] `C-u C-SPC` (前のマークへ戻る。 mark-ring): マークの履歴 (`tests/batch/cases/61-repeat`)。

### 削除・編集

- [x] `C-M-w` (append-next-kill): 次のキルを、kill-ring の先頭につなげる。
- [x] `C-S-DEL` (kill-whole-line) にキーを割り当てる。画面のアプリは、名前のあるキーと Ctrl と一緒の英字に、Emacs と同じく Shift (`S-`) を付けて送り、割り当ての無い Shift 付きのキーは、Shift を外して引く (shift-translation)。ターミナルは、Shift を区別できないことが多い。
- [x] `C-M-t` (transpose-sexps)。
- [x] `M-(` (insert-parentheses), `M-)` (move-past-close-and-reindent)。
- [x] `C-M-o` (split-line), `C-M-j` (default-indent-new-line), `M-i` (tab-to-tab-stop)。
- [x] `C-x TAB` (indent-rigidly): リージョンを、桁の数だけ字下げする。
- [x] `C-x C-;` (comment-line), `C-x ;` (comment-set-column)。
- [x] `C-x 8 RET` (insert-char): 文字を番号 (16 進、`#x`、`#o`、`#10r` など) で入れる。
- [ ] `C-x 8 RET` で、Unicode の名前で入れる (Emacs は、名前を補完する。今は、`Invalid character`)。`C-x 8` で始まる、そのほかの文字。
- [ ] `read-number`: 既定の値が無く、`M-n` の値だけがあるとき、Emacs は空の入力を聞き直す。今は、`M-n` の最初の値を返す。
- [x] `C-x =` (what-cursor-position): カーソルの文字と位置を出す。
- [x] `C-x z` (repeat), `C-x ESC ESC` (repeat-complex-command): 前のコマンドの繰り返し。続けて `z` を押すと、また繰り返す。
- [ ] `C-x z`: Emacs は、ミニバッファで引数を読んだコマンドを、`command-history` に残した同じ引数で繰り返す。今は、引数を聞き直す (`command-history` は、`M-x` のコマンドの名前だけを残す)。
- [x] `C--` / `M--` (negative-argument): 負の前置引数。
- [x] `C-?` (undo-redo)。
- [ ] リージョンの中だけの undo (`C-u C-/` ではなく、リージョンがあるときの `C-/`)。
- [x] `M-~` (not-modified): バッファを、変更なしにする。
- [x] `C-x f` (set-fill-column) にキーを割り当てる (前置引数が無ければ、Emacs 30 と同じく、桁を聞く)。
- [x] `M-SPC` (cycle-spacing): Emacs 30 の既定は、続けて押すと、空白を 1 つ、無し、元に戻す。今は `just-one-space`。
- [x] `M-'` (abbrev-prefix-mark), `C-x a ...` (add-global-abbrev など), `C-x '` (expand-abbrev): 略語 (abbrev)。
- [x] `C-M-i` (complete-symbol), `C-M-/` (dabbrev-completion): 補完の候補の一覧。
- [x] キーの無いコマンド: `untabify`、`tabify` (`tests/batch/cases/62-tabify`。タブの幅は 8)。
- [ ] キーの無いコマンド: `align`、`align-regexp`、`center-line`、`whitespace-mode` (ほかの `sort-lines` などは完了)。

### リージョン・矩形・レジスタ・ブックマーク

- [x] `M-h` (mark-paragraph)。
- [ ] `C-x SPC` (rectangle-mark-mode): 矩形を選ぶ。
- [x] `C-x r c` (clear-rectangle), `C-x r d` (delete-rectangle), `C-x r o` (open-rectangle),
      `C-x r M-w` (copy-rectangle-as-kill), `C-x r N` (rectangle-number-lines)。
- [x] `C-x r SPC` / `C-x r j` (point-to-register / jump-to-register): 位置のレジスタ。
- [ ] `C-x C-k x` (kmacro-to-register)。
- [ ] `bookmark-delete`、`bookmark-rename`。
- [x] `C-x n g` (goto-line-relative): キーを割り当てる。

### 検索・置換

- [x] isearch の大文字と小文字: 探す文字が小文字だけなら、大文字と小文字を区別しない (`case-fold-search`、`search-upper-case`)。`M-c` で区別を切り替える。
- [x] isearch の中の `M-c` (大文字と小文字), `M-r` (正規表現との切り替え)。
- [x] isearch の中の `M-e` (文字をミニバッファで直す。ミニバッファの `M-p` は、検索の履歴)。
- [ ] isearch の中の `C-q`、`M-%` (置換へ),
      `M-s o` (occur へ), `C-M-w` / `C-M-y` (1 文字削る・足す), `DEL` で 1 つ前の状態に戻る (今は、文字を消すだけ),
      合わないときの `C-g` (合った所までに戻す), 本文の端で折り返す (`Wrapped I-search`)。
- [ ] isearch の、合った所の全てを強調する (`isearch-lazy-highlight`) と、合った数を出す (`isearch-lazy-count`)。
- [x] `M-s w` (isearch-forward-word), `M-s _` (isearch-forward-symbol), `M-s .` (isearch-forward-symbol-at-point), `M-s M-.` (isearch-forward-thing-at-point)。
- [ ] `multi-occur`。
- [x] `grep` (Phase 15. 外のコマンドを使わず、エディタが探す)。
- [ ] `rgrep` と `lgrep` (探すファイルの名前の形 (`*.ts` など) を聞く), `zrgrep`、`find-grep`。
- [x] `M-s h r` / `M-s h l` / `M-s h p` / `M-s h .` / `M-s h u` (highlight-regexp など): 強調。
- [ ] query-replace の中の `^` (前へ戻る), `,` (置き換えて、止まる), `E` (置き換える文字を直す), `C-r` / `C-w` (再帰編集),
      `u` / `U` (置き換えを戻す), `?` (説明)。大文字と小文字を合わせる置換 (`case-replace`)。
- [ ] 置き換える文字の `\,(...)` (Lisp の式) と `\#` (置き換えた数)。
- [ ] `C-M-%` などで、リージョンがあれば、リージョンの中だけを置き換える。

### ファイル

- [x] `C-x C-r` (find-file-read-only), `C-x C-q` (read-only-mode)。
- [x] `C-x C-d` (list-directory), `C-x C-j` (dired-jump), `C-x 4 d` (dired-other-window)。
- [ ] dired の中のキー (docs/keybindings.md の Dired の表。 `RET`、`^`、`g`、`m`、`u`、`d`、`x`、`C`、`R`、`D`、`+` など)。
- [ ] `auto-revert-mode` (ファイルが書き換わったら読み直す)。
- [x] 保存するときに、ファイルが外で書き換わっていないか確かめる (`verify-visited-file-modtime`、`set-visited-file-modtime`。Emacs の「X has changed since visited or saved.  Save anyway?」)。
- [x] 外で書き換わったファイルのバッファを、初めて書き換えるときに確かめる (`ask-user-about-supersession-threat`。「X changed on disk; really edit the buffer? (y, n, r or C-h)」。書き換えの入口 `ensure_writable` が `file-supersession-threat` のエラーを返し、`EditorHost` が聞いてやり直す)。
- [ ] ファイルの錠 (`.#名前`。`create-lockfiles`)。
- [ ] `recover-session` (前のセッションの自動保存から、まとめて読み直す), `recentf-open` (最近のファイル)。
- [x] `append-to-file`, `rename-visited-file`。
- [ ] `find-file` のワイルドカード (`*.scm` で、全てを開く) と、ディレクトリを開いたら dired にする。

### バッファ・ウィンドウ

- [x] `C-x x t` (toggle-truncate-lines): キーを割り当てる。
- [x] 同じ名前のファイルのバッファの名前を分ける (`a.scm<src>` など。 `uniquify`。`lisp/uniquify.scm`)。dired のバッファは分けない (Emacs は分ける)。
- [ ] `C-x C-b` の一覧 (Buffer Menu) の中のキー (`d`、`s`、`x`、`u`、`~` など)。今は、`RET`、`f`、`o`、`q`、`n`、`p`、`SPC` だけ (`lisp/buff-menu.scm`)。
- [ ] `ibuffer`。
- [ ] `C-x 4 4` (other-window-prefix)。
- [x] `C-x ^` / `C-x {` / `C-x }` / `C-x -` / `C-x +` (enlarge-window, shrink-window-horizontally, enlarge-window-horizontally,
      shrink-window-if-larger-than-buffer, balance-windows): ウィンドウの大きさ。
- [x] `C-M-S-v` (scroll-other-window-down、`M-<prior>` も) / `C-M-S-l`: キーを割り当てる。
- [ ] `C-x w ...` のうち `toggle-window-dedicated`、`winner-mode`、`windmove`。
- [ ] `C-x 5 ...` (フレーム) と `C-x t ...` (タブ)。

### 前置引数とキーボードマクロ

- [ ] `C-x C-k ...` (kmacro): マクロの環 (`C-x C-k C-n` など), 数え上げ (`C-x C-k TAB`、`C-x C-k C-c`),
      キーに割り当てる (`C-x C-k b`), 直す (`C-x C-k RET`、`C-x C-k e`), 1 つずつ動かす (`C-x C-k SPC`)。
- [ ] `C-x C-k r` (apply-macro-to-region-lines), `C-x q` (kbd-macro-query)。

### ヘルプ

- [~] `C-h f` / `C-h v` / `C-h o` / `C-h x` (describe-function / describe-variable / describe-symbol / describe-command) (Phase 18)。`C-h f` は完了。
- [ ] `C-h m` (describe-mode)。
- [ ] `C-h a` / `C-h d` (apropos-command / apropos-documentation)。
- [x] `C-h l` (view-lossage. 押したキーの履歴), `C-h e` (view-echo-area-messages. `*Messages*` バッファ)。
- [ ] `C-h t` (チュートリアル), `C-h q` (help-quick-toggle。今は `help-quit`)。
- [ ] `*Help*` バッファの中のキー (`q`、`l`、`r`、リンクを開く)。

### ミニバッファと補完

- [x] `M-r` / `M-s` (previous-matching-history-element など): 入力の履歴を正規表現で探す (ターミナルと画面)。
- [x] `M-n` の既定の値 (future history)。`read-string` と `read-file-name` の既定の値を出す。
- [ ] `*Completions*` バッファで選ぶ (`M-v`、`M-<up>`、`M-<down>`、`RET`)。補完のスタイル (`partial-completion` の `f-f` など)。
- [x] `yes-or-no-p` で、`yes` と `no` のほかの答えには、聞き直す (「Please answer yes or no.」)。今は、`no` とする。
- [ ] `M-ESC ESC` (keyboard-escape-quit), 再帰編集 (`C-M-c`)。
- [ ] `M-X` (execute-extended-command-for-buffer), `M-x` で、コマンドのキーを知らせる (「You can run the command ... with C-x ...」)。
- [ ] `C-u M-x` などの、前置引数を付けたコマンドの実行。
- [ ] ミニバッファの中での、エディタのキー (`C-a`、`C-k`、`M-d`、`C-y` など)。今は、入力欄 (`input`) のキーだけ。

### Lisp

- [x] `M-.` / `M-,` / `M-?` / `C-M-.` (xref-find-definitions など): 定義へ移る。
- [ ] `C-M-x` (eval-defun), `eval-buffer`、`eval-region`、`load-file`、`C-u C-x C-e` (結果を本文に入れる)。
- [ ] `ielm` (Lisp の対話の画面), `*scratch*` での `C-j` (eval-print-last-sexp)。
- [ ] `eldoc-mode` (関数の引数をミニバッファに出す), `check-parens`。

### モードとそのほか

- [ ] `<insert>` (overwrite-mode), `auto-fill-mode`, `display-line-numbers-mode`, `visual-line-mode`, `column-number-mode`, `hl-line-mode`。
- [ ] `C-x C-+` / `C-x C--` / `C-x C-0` (text-scale-adjust), `C-x C-M-+` (global-text-scale-adjust): 文字の大きさ。
- [ ] `delete-selection-mode`、`electric-pair-mode`、`electric-indent-mode` (既定で有効。 `RET` で字下げする、は実装済み), `show-paren-mode` の設定。
- [ ] `prog-mode`。
- [ ] `shell`, `eshell`, `term`。
- [ ] `compile`、`recompile`、`C-x p ...` (project. `C-x p f` (プロジェクトのファイルを開く), `C-x p g` (プロジェクトの中を探す) など)。
      プロジェクトの根を探すのは、サイドバーのツリーで実装した (`.git` のある所)。
- [ ] `C-x v ...` (vc. バージョン管理。 `C-x v v`、`C-x v =`、`C-x v l` など), `diff`、`ediff`。
- [ ] `M-$` (ispell-word), `flyspell-mode`: 綴り。
- [ ] `C-x *` (calc), `calendar`, `C-x m` (compose-mail), `eww`, `M-s M-w` (eww-search-words)。
- [ ] `C-z` (suspend-frame), `C-x C-z`。
- [ ] 2 段の表示 (`C-x 6 ...`), `C-x $` (set-selective-display), `C-x 4 a` (add-change-log-entry-other-window)。
- [ ] `M-x customize` と、設定の保存 (`custom-file`)。`desktop-save-mode` (開いていたバッファを、次に起動したときに開く)。

## dotfiles/.emacs.el の global-map の関数

`~/dotfiles/.emacs.el` で `define-key global-map` にしているキーと関数 (103 個。同じキーを後で付け直したものも、並べた)。
「状態」は、rs-editor に、同じ働きのコマンドがあるか (キーは、rs-editor では違うことがある)。[x] は有る、[~] は一部、[ ] は無い。行は `.emacs.el` の行の番号。

### 編集と移動

| 状態 | キー | 関数 | 行 | 備考 |
|---|---|---|---|---|
| [x] | (tty の erase) | backward-delete-char-untabify | 473 | タブを空白に崩して消す (キーは無い) |
| [~] | `C-x %` | (括弧の対へ移る lambda) | 1269 | `forward-list` / `backward-list`。rs-editor は `C-M-f` / `C-M-b` (forward-sexp) で代わりになる |
| [x] | `C-;` / `C-x ;` | hippie-expand | 1367, 1368 | Emacs と同じく、既定のキーは無い (初期設定ファイルで `(global-set-key "C-;" 'hippie-expand)`) |
| [x] | `RET` | newline-and-indent | 1382 | rs-editor の `RET` は字下げする (electric-indent-mode) |
| [x] | `<C-insert>` | clipboard-kill-ring-save | 1406 | 既定の `C-<insert>` は kill-ring-save (Emacs と同じ) |
| [x] | `S-DEL` | clipboard-kill-region | 1409 | 既定の `S-<delete>` は kill-region (Emacs と同じ) |
| [x] | `<S-insert>` | clipboard-yank | 1412 | 既定の `S-<insert>` は yank (Emacs と同じ) |
| [x] | `C-S-k` | just-one-space | 1424 | rs-editor は `M-SPC` |
| [x] | `C-S-t` | transpose-chars | 1429 | `C-t` は nil にしている |
| [x] | `C-i` | self-insert-command | 1432 | タブを入れる |
| [x] | `M-g` | goto-line | 1443 | |
| [~] | `C-c ;` | comment-or-uncomment-region | 1446 | `M-;` (comment-dwim) のリージョンの働きとして有る。コマンドとしては無い |
| [~] | `C-<return>` / `C-x j` | cua-set-rectangle-mark | 2666, 2667 | 矩形の選択は無い。`C-x r k` などは有る |
| [ ] | `C-c C-m` | mark-symbol | 2978 | |
| [~] | `C-c C-w` | mark-word* | 2979 | `M-@` (mark-word) は有る |
| [ ] | `C-c C-s` | mark-string | 2980 | |
| [x] | `C-?` | redo | 3117 | `C-x C-/` |
| [ ] | `M-3` | delete-mark-ring | 1221 | |
| [ ] | `<f7>` / `<S-f7>` | point-undo / point-redo | 3229, 3230 | |
| [ ] | `<f8>` / `<S-f8>` | goto-last-change / goto-last-change-reverse | 3235, 3236 | rs-editor の `F8` は project-tree |
| [x] | `<up>` `<down>` `<right>` `<left>` | previous-line / next-line / forward-char / backward-char | 6520-6523 | |
| [x] | `<C-up>` `<C-down>` | backward-paragraph / forward-paragraph | 6524, 6525 | |
| [x] | `<C-right>` `<C-left>` | forward-word / backward-word | 6526, 6527 | |

### 表示とモード

| 状態 | キー | 関数 | 行 | 備考 |
|---|---|---|---|---|
| [ ] | `M-1` | toggle-which-func-mode | 631 | |
| [ ] | `<f11>` | resize-frame-interactively | 763 | |
| [ ] | `<f2>` | toggle-debug-on-error | 1262 | |
| [ ] | `C-^` | global-whitespace-mode | 1403 | |
| [ ] | `C-\` | skk-mode | 1415, 3784 | 先に nil にして、後で skk-mode |
| [ ] | `C-c C-l` | toggle-truncate-lines | 1418 | |
| [x] | `C-c C-g` | (message nil) | 1421 | エコー領域を消す。`C-g` で消える |
| [ ] | `<f10>` | other-window-or-split | 1440 | `C-x o` と `C-x 2` は有る |
| [ ] | `C-c t` | tabbar-mode | 3520 | |
| [ ] | `<f4>` | toggle-auto-complete-mode | 5256 | |
| [ ] | `<f5>` | toggle-yas-minor-mode | 5401 | |
| [ ] | `M-2` | se/make-summary-buffer | 5221 | |
| [ ] | `C-x t` | (nil) | 3785 | 外しているだけ |

### ファイル、ディレクトリ、バッファ

| 状態 | キー | 関数 | 行 | 備考 |
|---|---|---|---|---|
| [~] | `C-c r` | bookmark-choice | 1063 | `C-x r m` / `C-x r b` は有る |
| [~] | `C-x C-o` | occur-by-moccur | 1123 | howm の `C-c , o` (howm-occur) は有る。occur, moccur は無い |
| [ ] | `<f9>` / `<S-f9>` | desktop-recover / desktop-save* | 1346, 1353 | |
| [ ] | `C-x M-l` | find-file-at-point-goto-line | 1398 | |
| [ ] | `C-x M-f` | find-file-at-point | 1399 | |
| [ ] | `C-x M-d` | dired-at-point | 1400 | |
| [ ] | `C-x C-j` | dired-jump | 1564 | |
| [ ] | `C-x j` | dired-jump-other-window | 1565 | 後で cua-set-rectangle-mark に付け直している |
| [~] | `C-c x` / `<f6>` | speedbar-get-focus / sr-speedbar-toggle | 1842, 1843, 1957, 1958 | `F8` (project-tree) が近い |
| [ ] | `C-c C-x` / `<f12>` | recentf-open-files | 2651, 2652 | |
| [ ] | `C-c C-f` | file-cache-choice | 2720 | |
| [~] | `C-c C-j` | direx:jump-to-directory-other-window | 2992 | `F8` (project-tree) が近い |
| [ ] | `C-c a` | anything-choice (eshell では anything-eshell) | 3408 | |
| [ ] | `C-x a` | anything-filelist+ | 3415 | |
| [ ] | `C-c s` | helm-choice (eshell では helm-choice-for-eshell) | 3490 | |
| [ ] | `C-c n` | helm-mini | 3508 | |
| [ ] | `C-x C-z` | open-junk-file | 3981 | |

### 調べる、外のツール

| 状態 | キー | 関数 | 行 | 備考 |
|---|---|---|---|---|
| [ ] | `C-c C-i` | info-apropos-region-or-word | 779 | |
| [ ] | `C-h C-i` | info-lookup-symbol | 780 | |
| [ ] | `<f1>` | (manual-entry (current-word)) | 826 | |
| [ ] | `<S-f1>` | (woman (current-word)) | 871 | |
| [ ] | `<M-f1>` | iman | 2954 | |
| [ ] | `C-c m` | w3m-gtk-lookup | 923 | |
| [ ] | `C-c 3` | w3m-choice | 4585 | |
| [ ] | `C-c f` | default-browser-or-vlc-choice | 1339 | |
| [ ] | `C-c w` | sdcv-search-input | 4222 | |
| [ ] | `C-c i` | sdcv-search-pointer+ | 4224 | |
| [ ] | `C-c C-h` | hyperspec-lookup | 5458 | |
| [ ] | `C-c v` | vc-choice | 4873 | |
| [ ] | `C-c 2` | navi2ch | 3614 | |
| [ ] | `C-c e` | evernote-choice | 4659 | |
| [ ] | `C-c p` | pomodoro-choice | 4089 | |
| [ ] | `C-c d` | calendar-datetime-choice | 2156 | howm の `C-c , d` (日付を入れる) は有る |
| [x] | `C-c l` | org-store-link | 2521 | `lisp/org.scm` を `(require 'org)` で読む |
| [ ] | `C-c o` | org-choice | 2550 | |
| [x] | `C-c h` | howm-menu | 3673 | `lisp/howm.scm` を `(require 'howm)` で読む |
| [ ] | `M-\` / `M-[` / `M-]` | bm-toggle / bm-previous / bm-next | 3198-3200 | |

### cscope

| 状態 | キー | 関数 | 行 |
|---|---|---|---|
| [ ] | `C-<f3>` | cscope-set-initial-directory | 5133 |
| [ ] | `C-<f4>` | cscope-unset-initial-directory | 5134 |
| [ ] | `C-<f5>` | cscope-find-this-symbol | 5135 |
| [ ] | `C-<f6>` | cscope-find-global-definition | 5136 |
| [ ] | `C-<f7>` | cscope-find-global-definition-no-prompting | 5137 |
| [ ] | `C-<f8>` | cscope-pop-mark | 5138 |
| [ ] | `C-<f9>` | cscope-history-forward-line | 5139 |
| [ ] | `C-<f10>` | cscope-history-forward-file | 5140 |
| [ ] | `C-<f11>` | cscope-history-backward-line | 5141 |
| [ ] | `C-<f12>` | cscope-history-backward-file | 5142 |
| [ ] | `M-<f9>` | cscope-display-buffer | 5143 |
| [ ] | `M-<f10>` | cscope-display-buffer-toggle | 5144 |

## Phase 15: M-x grep と、サイドバーのプロジェクトのツリー

- [x] `M-x grep`: 外のコマンドを使わず、エディタ (Lisp の `lisp/compile.scm` が `rust-grep` で) が、ディレクトリの下のファイルから探す。
      正規表現とディレクトリを聞き、結果を `*grep*` バッファ (Emacs の grep-mode の形。 `ファイル:行:行の文字`) に、ほかのウィンドウに出す。
      `RET` (compile-goto-error) で、その行のファイルを、ほかのウィンドウに開く。`M-g n`、`M-g p`、`` C-x ` `` で、次・前の合った行へ。
- [x] サイドバーのプロジェクトのツリー (VS Code のエクスプローラー。 `src/components/ProjectTree.tsx`)。`F8` か `M-x project-tree` で、出すか消す。
      根は、バッファのディレクトリから上へたどって `.git` のある所 (無ければ、バッファのディレクトリ)。ディレクトリを押すと開き (開くたびに読み直す),
      ファイルを押すと、選んでいるウィンドウに開く。今のバッファのファイルを強調する。`.git` は出さない。↻ で読み直す。
- [ ] ツリー: キーで動かす (フォーカスを移して、`n`、`p`、`RET` など), 幅を変える、今のファイルまで開く (VS Code の Reveal),
      ファイルを作る・名前を変える・消す、ファイルが変わったら読み直す、根を選ぶコマンド、起動のときに出す設定 (今は、初期設定ファイルで `(call-interactively 'project-tree)`)。
- [ ] `*grep*` の中のキー: `n` / `p` (next-error-no-select など), `g` (探し直す), `TAB` / `S-TAB` (次・前のファイル), `C-c C-k` (止める)。
- [ ] grep: 探しているあいだに、見つけた行を順に出す (今は、終わってから出す)。止める (`C-c C-k`)。

## Phase 16: 起動の引数 (Emacs の Initial options)

Emacs 30.2 の `emacs --help` の「Initialization options」と同じことを、`rseditor` の起動の引数でできるようにする。
今は、起動の引数を読んでいない (初期設定ファイルは、必ず `~/.rseditor` か `~/.rseditor.d/init.scm` を読む)。

- [ ] 起動の引数を読む仕組み (Rust の `std::env::args` か `tauri-plugin-cli`)。知らない引数は、Emacs と同じく、開くファイルとして扱う (Action options は別の項目)。
      引数の値は、`--chdir DIR` と `--chdir=DIR` の両方の書き方を受ける。
- [ ] `--no-init-file`、`-q`: 初期設定ファイル (`~/.rseditor`、`~/.rseditor.d/init.scm`) を読まない。
- [ ] `--quick`、`-Q`: `-q --no-site-file --no-site-lisp --no-splash --no-x-resources` と同じ。
- [ ] `--no-site-file`: 全ての利用者の初期設定 (Emacs の `site-start.el`) を読まない。先に、rs-editor の `site-start.scm` (置く場所を決める) を読むようにする。
- [ ] `--no-site-lisp`、`-nsl`: 全ての利用者の Lisp のディレクトリ (Emacs の `site-lisp`) を `load-path` に入れない。先に、その場所を決めて、`load-path` に入れるようにする。
- [ ] `--init-directory=DIR`: 初期設定ファイルを、`~/.rseditor.d` の代わりに DIR から探す (`user-emacs-directory` に当たる変数も足す)。
- [ ] `--user USER`、`-u USER`: ほかの利用者の初期設定ファイル (`~USER/.rseditor`) を読む。
- [ ] `--debug-init`: 初期設定ファイルの失敗で、止まった所 (式と、呼び出しの跡) を出す (`debug-on-error` に当たる変数も足す)。
- [ ] `--chdir DIR`: 始めのディレクトリ (`default-directory`) を DIR にする。
- [ ] `--no-splash`: 始めの画面 (Emacs の `*GNU Emacs*`) を出さない。先に、始めの画面 (`inhibit-startup-screen` で消せる) を作る。
- [ ] `--no-desktop`: 前のバッファと窓 (Emacs の `desktop-save-mode`) を戻さない。先に、戻す機能を作る。
- [x] `--batch`: 画面を出さずに動かす (`-q` を含む)。Lisp の `message` は標準エラーに出し、終わったら終える。`noninteractive` に当たる変数を足す。
- [x] `--script FILE`: 画面を出さずに、Scheme のファイルを動かして終える (`--batch -l FILE` に当たる)。`-x` は、`#!/usr/bin/rseditor -x` で使う `-Q --script`。
- [ ] `--daemon`、`--bg-daemon[=NAME]`、`--fg-daemon[=NAME]`: 窓を出さずに、後ろで動かす (Emacs の server)。先に、`emacsclient` に当たるもの (動いている rs-editor でファイルを開く) を作る。
- [ ] `--no-window-system`、`-nw`: 端末の中で動かす。画面が Tauri (WebView) なので、作れるかを調べてから決める。
- [ ] `--display DISPLAY`、`-d DISPLAY`: 窓を出す X のディスプレイを変える (Tauri に環境変数 `DISPLAY` を渡す)。
- [ ] `--terminal DEVICE`、`-t DEVICE`: 端末の装置を変える (`-nw` を作ってから)。
- [ ] `--no-x-resources`: X のリソース (`~/.Xresources` の `Emacs.font` など) を読まない。先に、X のリソースを読むかを決める (読まないなら、受けて何もしない)。
- [ ] Emacs の作り方のための引数 (`--dump-file`、`--no-loadup` / `-nl`、`--fingerprint`、`--seccomp`、`--no-build-details`、`--module-assertions`):
      rs-editor には、当たるもの (Lisp の状態を書き出したファイル、loadup.el, 動的なモジュール) が無い。受けて何もしないか、`--fingerprint` などは版を出すかを決める。

## Phase 17: TypeScript のコマンドを、Lisp に移す

今は、エディタのコマンド (`find-file`、`forward-word` など) を、ほとんど TypeScript (画面) で書いている。
Emacs と同じく、組み込み (C に当たる Rust と、画面の TypeScript) は小さくし、コマンドは Lisp (`lisp/`) で書くようにする。
どのコマンドを、どこで書いているかは、docs/functions.md の「実装」の列 (`tools/emacs-commands/run.sh` で作り直す) で分かる。

- [x] 起動のときに読む、既定の Lisp のファイル (Emacs の `loadup.el` で読む `simple.el`、`files.el` などに当たる) を決める (`lisp/` に置き、アプリに入れる)。
      `core/src/scheme/loadup.rs` に並べたファイル (`lisp/subr.scm`、`lisp/window.scm`、`lisp/files.scm`、`lisp/help-fns.scm`) を、ビルドのときに埋め込み、評価器を作るときに評価する。
      コマンドは、引数を省くと (キーと M-x から呼ぶと), ミニバッファで聞く (Emacs の `interactive` に当たる)。
- [x] `find-file` (`C-x C-f`) を Lisp に移した (`lisp/files.scm`。`read-file-name`、`find-file-noselect`、`pop-to-buffer-same-window` を使う)。
- [x] よく使うファイルとバッファのコマンドを Lisp に移した: `save-buffer` (`C-x C-s`), `write-file` (`C-x C-w`), `insert-file` (`C-x i`),
      `find-alternate-file` (`C-x C-v`), `save-some-buffers` (`C-x s`) (`lisp/files.scm`), `switch-to-buffer` (`C-x b`. `lisp/window.scm`)。
      問いの `y-or-n-p`、`yes-or-no-p` と `file-name-directory` は `lisp/subr.scm`。組み込みに `read-key` (1 つのキーを読む) と、      ファイルの読み書きの組み込み (Emacs の C と同じ `insert-file-contents`、`write-region`、`do-auto-save`。`core/src/files.rs`) を足した。開くこと、保存すること、バックアップは、Emacs と同じく Lisp (`find-file-noselect`、`basic-save-buffer`、`backup-buffer`)。
      バッファを出す組み込みは、Emacs と同じく `pop-to-buffer-same-window` にした (`switch-to-buffer` はコマンド)。
- [x] `revert-buffer` (dired のときは `files--revert-dired` (`lisp/dired.scm`) で一覧を作り直す), `recover-this-file` (`make-auto-save-file-name`、`file-newer-than-file-p`) を Lisp に移した (`lisp/files.scm`)。
      組み込みに `files--set-buffer-text` (本文を置き換える。カーソルは同じ文字の所に残り、元に戻せる) を足した。
- [ ] 続けて移すもの: `save-buffers-kill-terminal` など。
      `kill-buffer` は、Emacs で C の関数なので、組み込みのまま残す。
- [ ] Lisp から使う組み込みの関数を足す (カーソル、文字、行、マーク、窓、バッファなど。 Emacs の C の関数に当たるもの)。
- [ ] Emacs で Lisp で書いたコマンド (docs/functions.md の「エディタの基本のコマンド」) を、Lisp に移す。移したら、TypeScript の方は消す。
      進め方は、docs/functions.md の「実装の計画」(段階 1 から 12。表の「計画」の列の番号。計画は `tools/emacs-commands/plan.py`)。
      目標は、今の画面 (Tauri) と、ターミナル (ncurses) の両方で動かすこと。そのため、段階 1 で、エディタの核 (バッファ、マーク、undo, kill-ring, 窓) を Rust に作る
      (1.1 から 1.11. 進み方は、docs/functions.md の「実装の計画」)。
      1.1 (クレートの分割: `core/`) と、1.2 から 1.8 の Rust の部品 (`Buffer`、`KillRing`、`search`、`syntax`、`window`、`Editor`), 1.9 の `EditorHost` (評価器が核を直接動かす), 1.10 のターミナルのアプリ (`tui/`) は完了。1.11 (今の画面 (Tauri) を核につなぐ) は完了 (`docs/core_migration.md`。画面の TypeScript のコマンドは全て消し、画面は核の frame を描いてキーを送るだけにした)。段階 2 (動き) と段階 3 (消す、切る、貼る、元に戻す) の Lisp 版 (`lisp/simple.scm`) は完了で、Emacs の結果と全位置で一致する (`core/tests/test_motion.rs`、`test_edit.rs`、`test_undo.rs`)。段階 4 (マーク、前置引数、実行) は、マークと前置引数と情報のコマンドの Lisp 版 (`push-mark-command`、`set-mark-command`、`exchange-point-and-mark`、`mark-word`、`universal-argument`、`digit-argument`、`what-cursor-position`、`count-words-region` など) と、バッファをまたぐマークの履歴 (`global-mark-pop`) が完了 (Emacs の結果と一致する)。ターミナルのアプリは Lisp の前置引数 (`prefix-arg`、`universal-argument-map`) とコマンドの終わりのマークの解除につないだ。`truncate-lines` と `selective-display` の描画、`repeat-complex-command`、`execute-extended-command-for-buffer` も完了 (段階 4 の画面 (Tauri) 側は、画面をつないだ後)。`next-error` と `previous-error` は TypeScript のまま。段階 5 (S 式、段落、ページ) は、式、defun, 段落、文の Lisp 版 (計算は `core/src/sexp.rs` と `core/src/paragraph.rs`) が完了で、Emacs の結果と全位置で一致する (`core/tests/test_lisp.rs`)。ページ (`forward-page`、`mark-page`) と `transpose-sexps`、`transpose-sentences`、`transpose-paragraphs` も完了 (`transpose-sexps` は、バックスラッシュで quote した文字と `##` `,@` の前後だけ Emacs と違う)。`narrow-to-page` と `count-lines-page` は残り (ナローイングを核に入れてから)。`condition-case` を評価器に足した。段階 6 (字下げ、整形、コメント) は、字下げ (`TAB`、`indent-region`、`indent-sexp`、`RET` の字下げ)、`indent-rigidly`、`tab-to-tab-stop`、整形 (`M-q`、`set-fill-prefix`)、コメント (`M-;`、`C-x C-;`、`C-x ;`) の Lisp 版が完了 (計算は `core/src/indent.rs`、`core/src/fill.rs`、`core/src/comment.rs`。字下げ、整形、`comment-dwim`、`comment-line` は Emacs の結果と全位置で一致する (`core/tests/test_lisp.rs`)。違いは docs/functions.md の「実装の計画」)。ターミナルのアプリのキーにつないだ。段階 7 (検索、置換、強調) は、isearch (`lisp/isearch.scm`)、`replace-string`、`replace-regexp`、`query-replace`、`query-replace-regexp`、`occur` (`lisp/replace.scm`)、強調 (hi-lock のコマンド。核の `Buffer::highlights`) の Lisp 版が完了で、Emacs の結果と一致する (`core/tests/test_isearch.rs`、`test_replace.rs`、`test_occur.rs`)。ターミナルのアプリは、`C-s`、`M-%`、`M-s o` などにつないだ。
      速さが要るもの (`self-insert-command`、カーソルを動かすもの) は、動かして比べてから決める。
- [x] 読み出し専用のバッファ (`buffer-read-only`、`read-only-mode`、`toggle-read-only`) を核に入れる (`Buffer::read_only`。書き換えると `Buffer is read-only` のエラー)。
- [ ] Emacs で C で書いたコマンド (docs/functions.md の「組み込みのコマンド」) は、組み込みのまま残す (Emacs と同じ)。

## Phase 18: ヘルプ (docstring と C-h f) と、ヘルプの多言語化

Emacs の `C-h f` は、関数の前の `;;` のコメントでなく、docstring (引数の後の文字列) を出す。rs-editor も同じにする。
今は、`define` と `define-command` の docstring を覚えておらず、`lisp/*.scm` の説明は、全て `;;` のコメントで書いている。

- [x] `define`、`define-command` の docstring (引数の後の文字列。本体が文字列だけなら、docstring でなく値) を覚える。
      `(documentation 'name)` と、変数の docstring (Emacs の `defvar` の 3 つ目。 `documentation-property`) も。
- [ ] `define-syntax` の docstring。
- [ ] 組み込みの関数 (Rust) の docstring を、1 か所に書く (Emacs の C の `doc: /* ... */` と DOC ファイルに当たる)。
      画面のコマンド (TypeScript) は、核に移した (`docs/core_migration.md`) ので、コマンドは全て Lisp の docstring を持つ。
      残るのは、Rust の組み込みの手続き (`car`、`insert`、`forward-char` など)。`C-h f` で調べると、`Not documented.` と出る。
      次の順で直す。
  1. 組み込みの手続きの docstring (英語) を、1 つの表 `core/src/scheme/builtin_docs.rs` (名前 → 文) に置く。
     Emacs にある関数は、Emacs の docstring を `emacs -Q --batch` で書き出して作る (`tools/emacs-commands/` に足す)。rs-editor で動きが違うものは、違いを 1 行足す。
  2. Lisp の `documentation` が、Lisp の docstring が無ければ、表の文を返す。`lisp/help-fns.scm` の `Not documented.` は、本当に docstring の無いものだけになる。
  3. `tools/po/update.py` が表も集め (`msgctxt` は `function:名前`)、`po/ja.po` に訳を足す。表の文も、Lisp の docstring と同じく `help-language` の訳で出る。
  4. テスト: 組み込みの名前 (`builtins::NAMES` など) が全て表にあること (`core/tests/`)。無い名前を足すと失敗する。
- [x] `C-h f` (describe-function. `lisp/help-fns.scm`): `*Help*` を、ほかのウィンドウに出す。定義したファイル (`symbol-file`), 割り当てたキー (`where-is-internal`),
      引数の形 (`help-function-arglist`。`(find-file &rest ARGS)`), docstring (無ければ `Not documented.`)。
      候補は、Lisp と Rust の関数 (`mapatoms`、`fboundp`)。
- [ ] `C-h v` / `C-h o` / `C-h x`。`*Help*` の、定義したファイルへのリンク、`q` で閉じる。カーソルの所の名前を既定にする。
      docstring の中の `\\[command]` (キー) と `` `name' `` (リンク) は、Emacs と同じく置き換える。
      引数の形は、Emacs の書き方 (`&optional`) にする (今は、残りの引数で受けて `&rest ARGS`)。
- [x] `lisp/howm.scm` の `;;` の説明を、docstring (英語) に書き直す (元の日本語は、`po/ja.po` の翻訳にした)。
- [x] `lisp/org.scm` の `;;` の説明を、docstring (英語) に書き直す (訳は `po/ja.po`)。
- [ ] 多言語化の仕組み (Emacs には無い。 rs-editor で足す). 翻訳のファイルは gettext の PO 形式にし、読むのは Rust で行う
      (gettext の実行時の仕組み (libintl, `setlocale`) は使わない。エディタの中で言語を変えるため、C のライブラリに頼らないため):
  - docstring は、Emacs と同じく英語で書く (元の文。翻訳が無ければ、これを出す)。
  - 翻訳は `po/ja.po` のように、言語ごとの PO のファイルに置く。`msgctxt` は名前 (`org-cycle` など。関数と変数は `function:` と `variable:` を前に付ける),
    `msgid` は英語の docstring, `msgstr` は翻訳。
  - [x] Lisp の docstring を集めて `po/rseditor.pot` を作り、各言語の PO に混ぜる道具 (`tools/po/update.py`。`npm run po:update`)。
    元の docstring が変わった翻訳は、`fuzzy` の印と、前の msgid (`#|`) を付ける。fuzzy の翻訳は出さず、英語を出す。
    `npm run po:check` (`npm run check` で動く) で、作り直していない PO を見つける。
  - [ ] Rust の組み込みの docstring も集める (上の「1 か所に書く」の後)。
  - [x] Rust で PO のファイルを読み (`core/src/scheme/po.rs`。ビルドのときに埋め込む), 名前と元の文から翻訳を引く。
    Lisp の `documentation` と `documentation-property` が、翻訳を返す (2 つ目 (3 つ目) の引数 RAW が真なら、元の英語)。
  - [x] 出す言語は、変数 `help-language` (既定は `LANG` から。 `"en"` なら元の文) で選ぶ。
  - [ ] `*Help*` に、元の文 (英語) を見るボタンを付ける。
  - 翻訳の無いものと、fuzzy の翻訳を、一覧にする (Phase 20 の lint の 1 つ)。Poedit などの翻訳の道具で直せることを、README に書く。
  - Rust のエラーの文 (`Cannot move column further` など) は、Emacs と同じく訳さない。
    画面の文 (メニュー、`message`) を訳すことになったら、複数形や語順を扱える Fluent (`fluent-rs`) と比べて決める。

## Phase 19: Scheme の整形 (format)

`lisp/*.scm` と `tests/lisp/*.scm` を、決まった形に揃える道具。エディタの `TAB` / `C-M-q` と同じ字下げにする。

- [ ] 字下げの決まりを、エディタ (`src/lib/lispIndent.ts`、`lispIndentSpecs.ts`) と 1 つにする (Emacs の `lisp-indent-function` と `scheme-indent-function` の値)。
      Rust で書くなら、決まりの表をファイル (JSON か Scheme) にして、両方から読む。
- [ ] 整形の道具 (例: `cargo run --bin scmfmt -- FILE...`): 字下げを直し、行の終わりの空白、閉じ括弧だけの行、空行の重なりを消す。
      コメント (`;`、`;;`、`;;;` の位置。 Emacs の決まり) と、文字列の中は変えない。長い行を分けるかは、はじめは変えない。
- [ ] `--check` (直す所があれば、失敗にする) を付け、`npm run format` と `npm run check` に入れる。
- [ ] エディタの `M-x indent-region` と結果が同じことを、テストで確かめる。

## Phase 20: Scheme の lint

`lisp/*.scm` と `tests/lisp/*.scm` の誤りを、動かす前に見つける道具 (Emacs の byte-compile の警告と `checkdoc` に当たる)。`npm run check` に入れる。

- [ ] 定義していない名前 (変数と関数), 引数の数の違い (定義した関数と、組み込みの関数)。
- [ ] 使っていない変数 (`let` と引数), 同じ名前の定義の重なり。
- [ ] 変数と関数の名前のぶつかり (名前の場所が 1 つなので、同じ名前の変数と関数は、後のものが前のものを消す。 org.scm の `org-agenda-files` で起きた)。
- [ ] rs-editor の Scheme に無いものを使った所: ドット対 (`(a . b)`、`cons` の 2 つ目がリストでない), `cond` の `=>`、      `format` の `%s`、`%S`、`%d`、`%%` 以外 (`%5d` など), ベクタ、文字 (`#\a`)。
- [ ] `'()` を偽として使った所 (`(if '() ...)` は真)。
- [ ] docstring (Phase 18): コマンド (`define-command`) と、外から使う関数の docstring が無い所。Emacs の `checkdoc` の決まり (1 行目は文で終える、引数は大文字で書くなど)。
- [ ] 翻訳 (Phase 18): `.pot` が docstring と合っているか、PO の翻訳の無いものと fuzzy (元の docstring が変わった) の翻訳。
- [ ] 警告ごとに、止める書き方 (コメントか、ファイルの変数) を決める。

## Phase 21: そのほかの機能

上の Phase に無い項目。Scheme の整形 (Phase 19)、Scheme の lint (Phase 20)、ヘルプの多言語化 (Phase 18) は、それぞれの Phase で進める。

### 編集のモード

- [ ] paredit: 括弧の対応を崩さずに S 式を編集するマイナーモード (`(` で `()` を入れる、`C-)` で S 式を取り込む、など)。
- [ ] skk-mode: 日本語入力 (SKK)。dotfiles では `C-\` に割り当てている (下の「dotfiles/.emacs.el の global-map の関数」)。
- [ ] markdown-mode: Markdown の色分けと、見出しの移動など。

### Org mode

- [x] org-lint: Org の文書の書き方の誤りを一覧にする (`lisp/org-lint.scm`。`M-x org-lint` で `*Org Lint*` に出し、`RET` でその行に移る)。Phase 20 (Scheme のコードの lint) とは別のもの。行を種類に分けて調べ、`tests/org-lint/run.sh` で Emacs の org-lint の結果と比べる。移したチェッカー: misplaced-heading、duplicate-custom-id、duplicate-name、duplicate-target、duplicate-footnote-definition、orphaned-affiliated-keywords、obsolete-affiliated-keywords、deprecated-export-blocks、missing-language-in-src-block、missing-backend-in-export-block、deprecated-category-setup、special-property-in-properties-drawer、obsolete-properties-drawer、invalid-effort-property、invalid-id-property、undefined-footnote-reference、unreferenced-footnote-definition、invalid-keyword-syntax、invalid-block、misplaced-planning-info、incomplete-drawer、indented-diary-sexp、quote-section、spurious-colons、invalid-custom-id-link、invalid-fuzzy-link、trailing-bracket-after-link、unclosed-brackets-in-link-description、link-to-local-file、file-application、percent-encoding-link-escape、deprecated-header-syntax、non-existent-setupfile-parameter、wrong-include-link-parameter (INCLUDE の `::` の後の検索の部分は確かめない)、obsolete-include-markup、planning-inactive (計 36)。
- [ ] org-lint の残りのチェッカー 23 個 (`org-lint-unsupported-checkers`): 日時 (`timestamp-syntax` (曜日が言語で変わる)、`mismatched-planning-repeaters`)、項目 (`item-number`)、`invalid-id-link` (org-id)、Babel (`wrong-header-argument`、`wrong-header-value`、`empty-header-argument`、`named-result`、`invalid-babel-call-block`、`suspicious-language-in-src-block`)、引用 (`org-cite`)、書き出しの設定 (`unknown-options-item`、`misspelled-export-option`、`invalid-macro-argument-and-template`)、ほか。構文木 (日時、項目) や、Babel と ox の情報を作ってから。
- [x] C のシンタックスハイライト (核の Rust で字句に分け、画面 (Tauri) と TUI (ncurses) の両方に色を付ける。設計は `docs/c_highlight_design.md`)。`core/src/highlight/` が本文を字句 (コメント、コメントの区切り、`/** */`、文字列、キーワード、型、定数、プリプロセッサ、数値) に分け、`c-mode` (`.c` と `.h`) のバッファは `Buffer::token_spans` で取れる。Emacs 30 の `c-mode` の font-lock と、`tests/c-highlight/run.sh` (`npm run test:c-highlight`) で比べる。
- [x] C++ のシンタックスハイライト (`c++-mode`。`.cc` `.cpp` `.hpp` など。キーワードと型の表、生の文字列 `R"x(...)x"`、数値の区切り `1'000`。Emacs 30 の `c++-mode` と、`tests/c-highlight/run.sh` で比べる。`docs/c_highlight_design.md` の段階 2)。
- [x] C と C++ のハイライトの、宣言の名前 (関数の名前、変数の名前、型の名前、ラベル、`#define` の名前。Emacs の cc-mode の宣言の推測の、よく使う形。`core/src/highlight/decl.rs`。Emacs と `tests/c-highlight/run.sh` で比べる。`docs/c_highlight_design.md` の段階 3)。
- [x] C++ の宣言の名前の、残りの形 (演算子の関数、`typename T::x`、配列へのポインタと参照、`if` `while` `switch` の条件の中の宣言。ファイルの先頭の型の無い関数の呼び出しは、Emacs も色を付けないので同じ)。`docs/c_highlight_design.md` の 3.3。
- [ ] シンタックスハイライトの、再解析の効率化 (1.2 MB、3.5 万行の C のファイルの全体の解析が、デバッグビルドで 0.4 秒弱なので、遅くなってから) (今は、本文を書き換えるたびに、全体を読み直す。行ごとの状態を覚えて、変わった行から先だけを読み直す) (同じ文書の段階 4)。
- [x] Elisp と Scheme の色付けを、画面の CodeMirror の `StreamLanguage` から、核の `core/src/highlight/lisp.rs` に移した (Tauri と TUI の両方に色が付く。`emacs -Q --batch` との比較は `tests/lisp-highlight/`。`src/lib/lispMode.ts` と `elispKeywords.ts` は消した。`docs/c_highlight_design.md` の 3.4)。
- [x] Rust のシンタックスハイライト (`rust-mode`。`.rs`)。核の `core/src/highlight/rust.rs` が、rust-lang/rust-mode の font-lock (正規表現の並びと、先に付いた色を上書きしない決まり) を移し、Tauri と TUI の両方に色を付ける。コメント、ドキュメントのコメント、入れ子のブロックコメント、文字列、生の文字列、文字とライフタイム、キーワード、組み込みの型、`unsafe`、マクロ、属性、`fn` `struct` `enum` `mod` `use` の次の名前、`?` が、rust-mode と一致する。比較は `tests/rust-highlight/run.sh` (`npm run test:rust-highlight`。`RUST_MODE_DIR` に rust-mode の clone を指定する)。
- [x] TypeScript のシンタックスハイライト (`typescript-mode`。`.ts` `.tsx` `.mts` `.cts`)。核の `core/src/highlight/typescript.rs` が、emacs-typescript/typescript-mode の font-lock (正規表現と、括弧を読む規則。コメント、文字列、テンプレートリテラルの `${}`、正規表現のリテラル、キーワード、型、定数、`class` `interface` `type` `enum` の名前、`const` の名前、引数、関数の呼び出し、デコレータ、JSDoc) を同じ順で移し、Tauri と TUI の両方に色を付ける。比較は `tests/ts-highlight/run.sh` (`npm run test:ts-highlight`。`TS_MODE_DIR` に typescript-mode の clone を指定する。Emacs の結果は `expected/` に入れてある)。`.js` と `.jsx` は、Emacs が組み込みの `js-mode` (規則が違う) で開くので含めない。
- [x] メジャーモードの判定を、核に一本化した (`auto-mode-alist` と、1 行目の `-*- mode: ... -*-` は `lisp/files.scm`。モードラインの名前、括弧の読み方、字下げは `core/src/mode.rs` の `Buffer::mode_info`。画面は `BufferFrame.mode` で受け取り、`src/lib/modes.ts` は消した。TUI のモードラインも同じ名前 (`Lisp Interaction`、`ELisp/d`、`Text` など) を出す)。
- [x] TUI の字句の色を、256 色の端末で、Emacs に近い色にする。

### 作りと画面

- [ ] Scheme のインタプリタを独立させる: `core/src/scheme/` を、エディタの核 (`Editor`, `Buffer`) に頼らない別のクレートに分け、エディタ無しでも使えるようにする。
- [ ] 画面 (Tauri) でも `*Messages*` バッファでメッセージの記録を見られるようにするか検討する。ターミナルのアプリは `view-echo-area-messages` (`C-h e`) で見られる。
- [ ] w3m のように、Web のページをエディタの中で見る機能を、Tauri の webview で作れるか検討する。

### バッチモードの Emacs との違い (レビューから)

- [x] キーボードマクロを `C-g` で止める: `execute-kbd-macro` の回数 0 (と負の数) は、失敗するまで繰り返す。繰り返している間に `C-g` を押すと、止める (画面のアプリは、`session_input` が `C-g` を旗で知らせる。ターミナルのアプリは、`Screen::interrupted` で、待たずに確かめる)。
- [ ] 引数の数の違いを、`wrong-number-of-arguments` (データは手続きと個数) にする。今は種類が `error` で、メッセージは `car expects 1 argument, given 2`。
- [ ] `list-ref` など、Emacs に無い手続きの範囲外を、`args-out-of-range` にする (今は種類が `error`)。
- [x] ロックの `lock().unwrap()` を、panic のあとでも続けられるようにする (`core/src/sync.rs` の `LockExt::lock_recover`)。評価の panic で poisoned になったロックで、以後のキーが全て失敗することは無くなった。
- [ ] 開発用の依存の `npm audit` の警告 (`minimatch`、`katex`) は、直る版が出たら上げる (今は `markdownlint-cli2@0.23.3` と、`markdown-it` の `overrides` で、high は 0 件)。

### 完了済み

- [x] エラーを、種類とデータを持つ型にする (`core/src/error.rs`)。`signal`、`define-error`、`user-error`、`condition-case` が受ける条件 (`error-conditions`)。`(error "End of buffer")` などが、文の書き出しで種類を変えない。
- [x] `string-match` の結果を、バッファごとでなく 1 つにする。`string-match` の負の START を、文字列の終わりから数える。
- [x] ERT (Emacs Lisp のテストの仕組み) と同じことを Scheme でできるようにする (`ert-deftest`, `should`, `should-not`)。
- [x] Org mode (`lisp/org.scm`)。
- [x] ミニバッファを `C-g` で抜ける。
- [x] Rust のテストのファイル名を `test_<対象>.rs` にそろえる (`core/tests/test_evaluator_edge_cases.rs`, `src-tauri/tests/test_integration.rs` に改名)。

## Phase 22: 優先度つきの進め方

各 Phase の未完了 (`- [ ]`) は 120 件ある。下に、優先度ごとにまとめた (この節の項目の数は含まない)。詳しい内容は、各 Phase の項目を見る (ここでは名前だけ)。
決め方は、(1) 固まる・データを失う・落ちるものを先に、(2) 毎日の編集で Emacs と違って困るもの (`dotfiles/.emacs.el` で使っているキーを含む) を次に、(3) 使う場面が限られるものや、大きな機能を後に、とした。
各項目の大きさは、小 (1 コミット)、中 (数コミット)、大 (Phase 単位) の目安。

### P0: 固まる、データを失う、落ちる

- [x] キーボードマクロを `C-g` で止める。
- [x] 保存前に、外の書き換えを確かめる。
- [x] 300 回のキーごとの自動保存。
- [x] `lock().unwrap()` の panic 対策。
- [x] `yes-or-no-p` の聞き直し。
- [x] 編集のときの確かめ (`ask-user-about-supersession-threat`)。

### P1: 毎日の編集で、Emacs と違って困るもの (次にやる)

編集の動き (小が多い。まとめて 1 つのまとまりで進める):

- [x] 負の前置引数 `C--` / `M--` (Phase 14)。
- [x] 前置引数を、繰り返しでなく、引数として渡す (Phase 12)。多くのコマンドの動きに関わるので、先にやる。中。
- [x] `M-@`、`C-M-SPC`、`C-M-h` の負の前置引数 (前へ選ぶ) (Phase 12)。小。
- [x] kill-ring、消した矩形、レジスタを、全てのウィンドウで共通にする (Phase 12)。中。
- [x] isearch の大文字と小文字 (`case-fold-search`)、`M-c` / `M-r` / `M-e` (Phase 14「検索・置換」)。小〜中。
- [x] `M-n` の既定の値 (ミニバッファと isearch) と、`M-r` / `M-s` の履歴の検索 (Phase 12, 14)。小。
- [x] `C-q` の細かい違い (前置きのキー、`C-q RET`、Meta 付き) (Phase 12)。小。
- [x] 使えない (disabled) コマンドの確かめ: `C-x C-u`、`C-x C-l`、`C-x C-n` (Phase 12)。小。
- [x] `C-n` / `C-p` が、表示の桁 (タブと全角の幅) を保つ (Phase 12)。中。
- [x] 単語の文字を Emacs の構文表に合わせる (`M-f`、`M-b`) (Phase 12)。小〜中。
- [x] `M-q` を、docstring の中と、リージョンの全ての段落でも使えるようにする (Phase 12)。小〜中。
- [x] マークを置く違い: `C-M-a` / `C-M-e`、`C-x i`、`C-x r y`、`C-x r i` (Phase 12)。小。

足りないコマンド (Phase 14 と dotfiles の表から。よく使うものを先に):

- [x] キー割り当てだけが残るもの (コマンドは有る): `C-S-DEL` (kill-whole-line)、`C-x f` (set-fill-column)、`C-x =` (what-cursor-position)、`C-x 8 RET` (insert-char)、`C-?` (undo-redo)、`M-g c` / `M-g TAB`、`C-x n g`、`C-x x t`、`C-M-S-v` / `C-M-S-l`、`C-x C-@` (pop-global-mark)。小。
- [x] コマンドごと足すもの: `C-x z` (repeat)、`C-x ESC ESC`、`C-u C-SPC` (mark-ring)。小。
- [x] キーの無いコマンドのうち `untabify`、`tabify` (Phase 14「削除・編集」)。小。
- [x] クリップボードのキー: `<C-insert>`、`S-DEL`、`<S-insert>` と、`hippie-expand` (`C-;`)、`backward-delete-char-untabify` (dotfiles の表)。小。クリップボード (`select-enable-clipboard`) は、画面のアプリは、kill をクリップボードにも書き、yank はほかのアプリが書いた文字を使う。ターミナルとバッチは、Emacs の端末と同じく、使わない。`hippie-expand` (`lisp/hippie-exp.scm`) は、Emacs の既定の `hippie-expand-try-functions-list` の関数を全て持つ。違い: `try-expand-dabbrev-visible` と `try-expand-whole-kill` は無い。ファイルの名前の補完は `completion-ignored-extensions` を使わない。`try-expand-dabbrev-from-kill` は、kill-ring を `yank-pop` で回した後は、回した位置から読む。
- [x] ファイル・バッファ: ターミナルのアプリの 30 秒の自動保存、同名のファイルのバッファの名前を分ける (`uniquify`。dired のバッファは分けない)、`rename-visited-file`、`append-to-file`。小〜中。
- [ ] dired の中のキー (`RET`、`^`、`g`、`m`、`u`、`d`、`x`、`C`、`R`、`D`、`+`) と、ディレクトリを開いたら dired にする (Phase 12, 14)。大。毎日使うなら、P1 の最後に進める。
- [ ] Buffer Menu の中のキー (`d`、`s`、`x`、`u`、`~`) (Phase 14)。小。
- [ ] Lisp の作業: `C-M-x` (eval-defun)、`eval-buffer`、`eval-region`、`load-file`、`*scratch*` の `C-j`、`check-parens` (Phase 14「Lisp」)。小〜中。
- [ ] 外のツール: `compile`、`recompile`、`C-x p ...` (project) (Phase 14「モードとそのほか」)。中。
- [ ] ウィンドウ: `C-x 4 4`、`C-x w ...` の残り (`toggle-window-dedicated`、`winner-mode`、`windmove`) (Phase 14)。小〜中。
- [ ] `next-error` で `compile` と `occur` の結果も使う、`multi-occur` (Phase 14)。中。
- [ ] 置換: query-replace の `^`、`,`、`E`、`C-r` / `C-w`、`\,(...)`、`\#`、リージョンの中だけ (Phase 14)。中。

### P2: 便利だが、なくても困らない (P1 のあとに、よく使うものから)

- [ ] ヘルプ: `C-h m` / `a` / `d` / `t`、`*Help*` の中のキー (Phase 14, 18)。中。
- [ ] 起動の引数: `--no-init-file` / `-q`、`--quick` / `-Q`、`--chdir`、`--init-directory`、`--debug-init`、`--no-splash` (Phase 16)。小。引数を読む仕組みを先に (`--batch` と `--script` は完了)。
- [ ] エラーの違い: `wrong-number-of-arguments`、`args-out-of-range` (Phase 21)。小。
- [ ] grep: `rgrep` / `lgrep`、`*grep*` の中のキー、探しながら出す、止める (Phase 12, 14, 15)。中。
- [ ] ツリー (サイドバー): キーで動かす、今のファイルまで開く (Phase 15)。中。
- [ ] `C-x SPC` (rectangle-mark-mode)、`bookmark-delete` / `bookmark-rename`、`C-x C-k x` (kmacro-to-register) (Phase 14)。小〜中。
- [ ] `*Completions*` で選ぶ、補完のスタイル (Phase 14)。中。
- [ ] キーボードマクロ: `C-x C-k ...` (環、数え上げ)、`C-x C-k r`、`C-x q` (Phase 14)。中。
- [ ] `imenu` (`M-g i`)、`eldoc-mode`、`ielm`、`eval-defun` などの評価 (Phase 14)。中〜大。
- [ ] モード: `auto-fill-mode`、`visual-line-mode`、`display-line-numbers-mode`、`hl-line-mode`、`electric-pair-mode`、`delete-selection-mode`、ほかのメジャーモードの切り替え (Phase 14)。中。
- [ ] Scheme の整形 (`scmfmt`) と lint (Phase 19, 20)。大。コードを書くときの助けになる。先に、字下げの決まりをエディタと 1 つにする。
- [ ] 日本語入力 (skk-mode)、Markdown (markdown-mode)、paredit (Phase 21)。`dotfiles` で使っているので、使う頻度で上げる。大。
- [ ] ヘルプの多言語化の続き (Rust の組み込みの docstring を 1 か所に、`*Help*` の元の文を見るボタン) (Phase 18)。中。
- [ ] Phase 17 の続き (`save-buffers-kill-terminal` など、Emacs で Lisp のコマンドを Lisp に移す)。中〜大。

### P3: 使う場面が限られる、または大きい (後回し)

- [ ] バージョン管理 (`C-x v ...`)、`diff`、`ediff`、綴り (`ispell`、`flyspell`)、`calc`、`calendar`、メール、`eww` / w3m、`shell`、`eshell`、`term` (Phase 14, 21)。大。外のツールで足りる。
- [ ] フレームとタブ (`C-x 5 ...`、`C-x t ...`)、`C-z`、2 段の表示 (`C-x 6 ...`)、`customize`、`desktop-save-mode`、`recover-session`、`recentf`、ファイルの錠 (Phase 14)。中〜大。
- [ ] 起動の引数のうち、画面や作り方に関わるもの: `--daemon`、`-nw`、`--display`、`--terminal`、`--no-x-resources`、`--user`、`--no-desktop`、`--dump-file` など (Phase 16)。先の機能 (server、`-nw`) を決めてから。
- [ ] Scheme のインタプリタを、独立したクレートにする (Phase 21)。大。
- [ ] 画面 (Tauri) の `*Messages*`、Web のページを見る機能 (Phase 21)。調べてから決める。
- [ ] `npm audit` の警告 (`minimatch`、`katex`) は、直る版が出たら上げる (Phase 21)。待つ。

### 整理 (進める前に直す)

- Phase 16 の `--batch` と `--script FILE` は、実装済み (`tui/src/batch.rs`、`tests/batch/` の `cli-script`)。Phase 14 の完了項目は、`fboundp` とキーの割り当てで確かめて、チェックを付けてある。Phase 12 の「Emacs と違うところ」は、動きを一つずつ確かめていない。
- 同じ内容が、Phase 12 と Phase 14 の両方にある項目 (`M-n` の既定の値、`C-x C-n` など) は、片方に寄せる。
- この節の項目は、チェックボックスで進み具合を示す。項目を完了したら、各 Phase の項目と、この節の項目を、同じ変更で `[x]` にする (消さない)。この節の項目は、全て完了してから、最後にまとめて削除する。この節の項目は、未完了の数 (上の件数) には数えない。

## 実装順序の目安

Phase 1 から Phase 12 は、完了。これからは、上の「Phase 22: 優先度つきの進め方」の順 (P0 から P3) で進める。
