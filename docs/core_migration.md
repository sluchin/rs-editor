# 画面 (Tauri) を核につなぎ替える設計

画面 (Tauri) の TypeScript のコマンドを Lisp に移すために、本文の持ち主を CodeMirror から Rust の核 (`core/`) に移す設計です。
`docs/functions.md` の「実装の計画」の 1.11 に当たります。
移し終えたら、結合試験 (`docs/integration_test.md`) を行います。

## 1. 目的

- 画面のコマンドを、ターミナルのアプリ (`tui/`) と同じ Lisp (`lisp/*.scm`) で動かす。TypeScript のコマンド (`src/lib/emacs.ts` など) は消す。
- 画面とターミナルのアプリで、コマンドの動き (前置引数、`last-command`、マーク、undo、kill-ring) を同じにする。
- `mirror.rs` (評価のたびに本文の全体を写す仕組み) をやめ、Lisp が核の上で直接動くようにする。

## 2. 今の作り

| 部分 | 今の持ち主 | 内容 |
| --- | --- | --- |
| 本文、カーソル、マーク、undo | CodeMirror (`src/components/CodeEditor.tsx`) | バッファごとに `EditorState`。`Buffer.doc` は CodeMirror の `Text` |
| バッファの一覧、ウィンドウ | `src/hooks/useBuffers.ts`、`src/lib/windows.ts` | React の状態 |
| キーの割り当て、コマンド | `src/lib/emacs.ts` (約 125 個) | CodeMirror の keymap と、`createCommands` |
| kill-ring、キーボードマクロ | `src/lib/killRing.ts`、`src/lib/emacs.ts` | 全てのバッファで共通 |
| ミニバッファ | `src/components/Minibuffer.tsx`、`src/hooks/useMinibuffer.ts` | 入力、補完、履歴 |
| Lisp | `core/` の評価器。`mirror.rs` で、評価のあいだだけ本文を核に写す | 画面の操作は `editor-request` のイベントで画面に頼む |

ターミナルのアプリは、すでに核が本文を持ち、キーの割り当て (`tui/src/keymap.rs`) とコマンドループ (`tui/src/app.rs`) を Rust で持つ。
Lisp のコマンド (`lisp/simple.scm` など) は、このコマンドループの上で、Emacs の結果と全ての位置で一致する。

## 3. 目標の作り

```text
┌──────────── 画面 (TypeScript) ─────────────┐          ┌──────────────── Rust (src-tauri) ────────────────┐
│ キーと文字の入力 (IME を含む)                │ ──key──▶ │ コマンドループ (core::command_loop)                │
│ CodeMirror: 表示、色分け、スクロール          │          │   キーの割り当て (core::keymap)                   │
│ ミニバッファ: 入力の欄、補完の候補             │ ◀─frame─ │   評価器 + EditorHost (核の Editor を直接動かす)   │
│ モードライン、サイドバー、色とフォント          │ ◀─ask─── │   画面に頼む操作 (TauriUi: 問い、メッセージ、表示) │
└──────────────────────────────────────────────┘          └──────────────────────────────────────────────────┘
```

- 本文、カーソル、マーク、undo、kill-ring、バッファ、ウィンドウ、前置引数は、全て核 (`core::editor::Editor`) が持つ。
- キーは、全て Rust のコマンドループに送る。コマンドループは、キーを引いて (Lisp の `global-set-key`、`local-set-key` も含む)、コマンドを Lisp で動かす。
- 動かした後、Rust は「描く内容」(frame) を画面に送る。画面は、それを CodeMirror とモードラインに映すだけにする。
- 問い (`read-string`、`completing-read`、`read-key`、`yes-or-no-p`) とメッセージは、これまでどおり画面のミニバッファで行う (`UiHost` の代わりの `TauriUi`)。

## 4. 部品

### 4.1 Rust に新しく作るもの

| 部品 | 場所 | 内容 |
| --- | --- | --- |
| キーの割り当て | `core/src/keymap.rs` | `tui/src/keymap.rs` を移す。既定の割り当ては、`src/lib/emacs.ts` の `KEY_BINDINGS` と同じにする (1 つの表から作る) |
| コマンドループ | `core/src/command_loop.rs` | `tui/src/app.rs` の、キーを読む所、前置引数、`this-command`、`last-command`、`undo-boundary`、マークの解除、キーボードマクロを移す。画面の違いは `trait Ui` で受ける |
| 描く内容 | `core/src/frame.rs` | ウィンドウの分け方、各ウィンドウのバッファ、カーソル、リージョン、色付け (`highlight-regexp`、`outline`)、モードラインの文字、エコー領域。本文は、前に送った版からの変更 (位置、消す長さ、入れる文字) だけを送る |
| 本文の変更の記録 | `core/src/buffer.rs` | バッファに版の番号と、前の frame からの変更の一覧を持たせる (undo の記録とは別) |
| Tauri のつなぎ | `src-tauri/src/session.rs` | アプリに 1 つの `Session` (Editor、評価器、コマンドループ)。Tauri のコマンド `key_event`、`insert_text`、`mouse_point`、`resize`、`frame` |
| 画面に頼む操作 | `src-tauri/src/tauri_ui.rs` | `trait Ui` の画面版。問いとメッセージは、今の `Bridge` (`editor-request`、`editor_reply`) を使う |

### 4.2 画面 (TypeScript) に残すもの

- CodeMirror: 表示、色分け (`src/lib/lispMode.ts`)、括弧の強調 (`src/lib/showParen.ts`)、スクロール。CodeMirror の編集のコマンドと keymap は外す。
- 入力: キーを Emacs の書き方 (`toEmacsKey`) にして `key_event` で送る。文字は `insert_text` で送る。日本語の IME は、変換を確定したとき (`compositionend`) に送る。
- ミニバッファ (`Minibuffer.tsx`): 入力の欄と補完の候補。補完の候補は、Rust から問いと一緒に受け取る。
- モードライン、サイドバー (`ProjectTree.tsx`)、色とフォント (`src/lib/theme.ts`)。
- ファイルの選択など、OS の画面を使うもの。

### 4.3 消すもの (TypeScript)

`src/lib/emacs.ts` のコマンド、`killRing.ts`、`text.ts`、`sexp.ts`、`paragraph.ts`、`syntax.ts`、`bracketMatch.ts`、`rectangle.ts`、`fill.ts`、`lispIndent.ts`、`history.ts` の isearch の部分、
`useBuffers.ts`、`useSearch.ts`、`useFiles.ts` の自動保存とバックアップ (Rust に移す)、`useLisp.ts` の大部分、`src/lib/lisp.ts` の `performEditorRequest` の本文の操作、`core/src/scheme/mirror.rs`。
それぞれの単体試験 (`tests/frontend/lib/*.test.ts` など) も消すか、Rust の試験 (`core/tests/`) に移す。
Emacs の結果と全ての位置で比べている試験 (`tests/frontend/lib/motion.json` など) は、すでに Rust の試験 (`core/tests/test_motion.rs` など) にある。

## 5. 細かい決めごと

### 5.1 描く内容 (frame)

- 1 回のコマンドの後に 1 つ送る。コマンドの途中で問いを出すとき (`read-string` など) は、問いの前にも送る (問いの前に、変えた本文を見せるため)。
- 本文は、バッファごとに版の番号を付け、画面が持っている版からの変更だけを送る。画面が版を持っていないバッファ (初めて出すバッファ) は、全体を送る。
- 位置は、画面に合わせて UTF-16 の位置にして送る (核は文字で数える)。変換は Rust で行う。
- CodeMirror の `EditorState` は、バッファごとに画面に残す (色分けの状態を作り直さないため)。undo の記録は CodeMirror に持たせない (核が持つ)。

### 5.2 スクロールとウィンドウの大きさ

- スクロールは CodeMirror に任せ、カーソルが見えるようにする。`recenter` (`C-l`)、`scroll-up-command` (`C-v`)、`move-to-window-line` (`M-r`) は、画面に頼む操作にする (`TauriUi`)。
- 画面は、各ウィンドウの見えている行の数を `resize` で Rust に送る (`window-body-height` に使う)。

### 5.3 マウス

- クリックした位置は `mouse_point` で送り、核がカーソルを動かす。ドラッグの選択は、マークとカーソルにする。

### 5.4 速さ

- キー 1 つで、Tauri のコマンドの呼び出し 1 回と、frame 1 つにする。目安は、キーを押してから描くまで 16 ms 以内。
- 文字を続けて打つときは、`insert_text` にまとめる。IME の確定も、1 回で送る。
- 大きなファイル (10 MB) で、`C-f` と文字の入力が遅れないことを測る (5.6 の段階 2 で測る)。遅ければ、frame をまとめる (次の描画まで待つ) ことを考える。

### 5.5 ファイル

- 読み書きは、Rust の核で行う (`find-file-noselect`、`files--write-buffer`)。原子的な書き込み (`core/src/atomic.rs`) を使う。
- 自動保存 (`#名前#`) とバックアップ (`名前~`) は、Rust に移す (今は `src/hooks/useFiles.ts`)。

## 6. 進め方

各段階の終わりに、`npm run check` を通してコミットする。途中の段階では、画面の一部が動かなくてもよい (アプリの不安定さは許す)。

| 段階 | 内容 | 終わりの確かめ |
| --- | --- | --- |
| 1 | キーの割り当てとコマンドループを `core/` に移す (`trait Ui`)。ターミナルのアプリは、それを使う | `cargo test --workspace` (ターミナルのアプリの動きは変わらない) |
| 2 | frame と、本文の変更の記録を作る。Tauri に `Session` と `key_event`、`insert_text`、`frame` を作る | Rust の試験で、キーから frame までを確かめる。大きなファイルで速さを測る |
| 3 | 画面を frame で描くようにする。キーと文字を Rust に送る。CodeMirror の編集のコマンドを外す | 文字を打つ、移動する、`C-x C-f`、`C-x C-s` が画面で動く |
| 4 | ミニバッファ、問い、メッセージ、補完を `TauriUi` につなぐ | `M-x`、`C-x b`、`yes-or-no-p` が動く |
| 5 | ウィンドウ、モードライン、スクロール、マウス、色付け、IME | `C-x 2`、`C-x o`、`C-l`、`C-v`、日本語の入力が動く |
| 6 | 自動保存、バックアップ、初期設定ファイル、フック、サイドバー | `~/.rseditor` が読める。`#名前#` と `名前~` ができる |
| 7 | TypeScript のコマンドと、使わなくなった部品と試験を消す。`mirror.rs` と `RSEDITOR_NO_CORE` を消す | `npm run check`。カバレッジは TypeScript も Rust も 100% |
| 8 | 文書を直す (`.claude/CLAUDE.md`、`docs/functions.md`、`TODO.md`)。画面のコマンドの docstring の計画 (`TODO.md` の Phase 17) を、Rust の組み込みだけに直す | markdownlint |
| 9 | 結合試験の仕様書を新しい作りに合わせて直し、結合試験をする | `docs/integration_test.md` |

## 7. 危ないところと、その対策

| 危ないところ | 対策 |
| --- | --- |
| 日本語の IME の入力が、キーの送り先を変えると崩れる | 変換中 (`isComposing`) のキーは Rust に送らず、確定した文字だけを送る。段階 5 で、実際の画面で確かめる |
| キーを押してから描くまでが遅い | 段階 2 で測る。遅ければ frame を小さくする (変えたウィンドウだけ) |
| 色分けが、本文を全て置き換えると作り直しになり遅い | 変更だけを CodeMirror に渡す (5.1) |
| 画面とターミナルで、キーの割り当てがずれる | 既定の割り当てを 1 つの表にする (4.1) |
| 途中の段階で、画面が使えない | 段階 3 から 6 は、続けて進める。途中では、ターミナルのアプリで編集する |
| 試験の書き直しが多い | 画面の試験は、frame を受けて描くことと、キーを送ることだけにする。コマンドの動きは Rust の試験で確かめる |
