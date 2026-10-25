# 未実装の Emacs デフォルト機能

rs-editor が、Emacs の初期状態 (`emacs -Q`) の機能のうち、実装しているものと、まだ実装していないものをまとめました。
キーの一覧は [KEYBINDINGS.md](./KEYBINDINGS.md)、dired との比較は [DIRED_VS_RS_FILER.md](./DIRED_VS_RS_FILER.md) にあります。

## ✅ 実装済み

**カーソル移動**
- C-f, C-b - 前後に移動
- C-n, C-p - 上下に移動 (続けて押すあいだ, 桁を保つ)
- C-a, C-e - 行の開始・末尾
- M-f, M-b - 単語単位の移動
- M-<, M-> - バッファの先頭・末尾
- C-v, M-v - ページ単位のスクロール
- C-l - 画面を中央にスクロール (続けて押すと, 上, 下)
- M-g g - 指定行へジャンプ
- C-M-f, C-M-b - S 式単位の移動

**削除・編集**
- C-d - 1 文字削除
- C-k - 行削除（kill-line。続けたキルは 1 つにつなげる）
- M-d, M-DEL - 単語削除（kill-word, backward-kill-word）
- C-M-k - S 式削除（kill-sexp）
- C-w, M-w - 選択範囲の削除とコピー（kill-region, kill-ring-save）
- C-y - 貼り付け（yank）
- M-y - 別の削除内容を貼り付け（yank-pop）
- C-t, M-t, C-x C-t - 文字、単語、行の入れ替え
- M-u, M-l, M-c - 単語の大文字・小文字
- C-x C-u, C-x C-l - リージョンの大文字・小文字（upcase-region, downcase-region）
- C-q - キーをそのまま入れる（quoted-insert）
- M-= - 行、文、単語、文字の数（count-words-region. M-x count-words）
- C-o - 改行を入れる（open-line）
- M-; - コメントの付け外し（comment-dwim）
- M-/ - 単語の補完（dabbrev-expand。今のバッファの中だけを探す）
- M-z - 文字までキル（zap-to-char）
- M-q - 段落を詰める（fill-paragraph。Lisp では、コメントを詰める。文字列は詰めない）
- M-SPC, M-\, M-^, C-x C-o - 空白と空の行を消す（just-one-space, delete-horizontal-space, delete-indentation, delete-blank-lines）
- C-/, C-x C-/ - Undo / Redo（バッファごと）

**改行と字下げ**
- RET, C-j - 改行（RET は字下げもする）
- TAB, C-M-q, C-M-\ - 字下げ（Emacs の scheme-mode と emacs-lisp-mode と同じ計算）
- 括弧の強調（show-paren-mode）、シンタックスハイライト（font-lock）

**リージョン選択**
- C-SPC - マーク設定（移動で範囲が広がる。transient-mark-mode）
- C-x C-x - カーソルとマークの入れ替え
- C-x h - バッファ全体選択

**検索と置換**
- C-s, C-r - 前方・後方検索（isearch）
- C-M-s, C-M-r - 正規表現の検索
- M-%, C-M-% - 置換（query-replace, query-replace-regexp）. M-x replace-string, replace-regexp

**ファイル操作**
- C-x C-f - ファイルを開く（find-file。Tab でファイルの名前を補完）
- C-x C-s - 保存（save-buffer）
- C-x C-w - 別のファイルに保存（write-file）

**バッファ管理**
- C-x b - バッファ切り替え
- C-x k - バッファを閉じる
- C-x C-b - バッファ一覧
- C-x C-c - 終了（保存していないバッファがあれば確認）

**ウィンドウ**
- C-x 2, C-x 3 - ウィンドウを分ける
- C-x o, C-x 0, C-x 1 - ウィンドウを移る, 消す, ほかを消す

**Lisp**
- C-x C-e, M-: - 式の評価（eval-last-sexp, eval-expression）
- 初期設定ファイル（`~/.config/rseditor/init.scm`）、Lisp で作るコマンド（define-command）とキー（global-set-key）

**ヘルプ**
- C-h k - キーの説明（describe-key）
- C-h b - キーの割り当ての一覧（describe-bindings）

**前置引数とマクロ**
- C-u, M-0 から M-9 - 前置引数
- C-x (, C-x ), C-x e - キーボードマクロ

**その他**
- M-x - コマンド実行（Tab で補完）
- C-g - キャンセル（keyboard-quit）

---

## ❌ 未実装

### 編集

| キー | コマンド | 説明 |
|------|---------|------|
| `C-x r k` / `C-x r y` / `C-x r t` など | kill-rectangle など | 矩形の操作 |
| `C-x r s` / `C-x r i` | copy-to-register / insert-register | レジスタ |
| `C-x r m` / `C-x r b` | bookmark-set / bookmark-jump | ブックマーク |
| `C-x n n` / `C-x n w` | narrow-to-region / widen | ナローイング |

### 移動

| キー | コマンド | 説明 |
|------|---------|------|
| `M-{` / `M-}` | backward-paragraph / forward-paragraph | 段落の移動 |
| `M-a` / `M-e` / `M-k` | backward-sentence / forward-sentence / kill-sentence | 文の移動とキル |
| `M-m` | back-to-indentation | 行の字下げの後へ |
| `C-M-u` / `C-M-d` | backward-up-list / down-list | リストの外と中へ |
| `C-M-a` / `C-M-e` | beginning-of-defun / end-of-defun | 定義の始めと終わり |
| `C-M-h` / `C-M-SPC` / `M-@` | mark-defun / mark-sexp / mark-word | 定義, S 式, 単語を選ぶ |
| `C-x C-n` | set-goal-column | 上下に移るときの桁を決める |

### ファイル・バッファ・ウィンドウ

| キー | コマンド | 説明 |
|------|---------|------|
| `C-x i` | insert-file | ファイルの中身を入れる |
| `C-x C-v` | find-alternate-file | 別のファイルを開き直す |
| `C-x s` | save-some-buffers | 変更したバッファを保存 |
| - | revert-buffer | ファイルから読み直す |
| - | auto-save / バックアップ（`file~`） | 自動保存と, 前の版の保存 |
| `C-x ^` / `C-x {` / `C-x }` / `C-x +` | enlarge-window など | ウィンドウの大きさ |
| `C-M-v` | scroll-other-window | ほかのウィンドウをスクロール |
| `C-x 4 ...` / `C-x 5 ...` | find-file-other-window など | ほかのウィンドウ, フレームで開く |
| `C-x d` | dired | ディレクトリの一覧 |

### 検索

| キー | コマンド | 説明 |
|------|---------|------|
| `M-s o` | occur | 合う行の一覧 |
| `M-s .` | isearch-forward-symbol-at-point | カーソルの記号を検索 |
| isearch 中の `C-w` / `M-y` / `M-p` / `M-n` | isearch-yank-word-or-char など | 検索の文字の取り込みと履歴 |
| ミニバッファの `M-p` / `M-n` | previous-history-element など | 入力の履歴 |

### ヘルプ

| キー | コマンド | 説明 |
|------|---------|------|
| `C-h f` / `C-h v` | describe-function / describe-variable | 関数, 変数の説明 |
| `C-h m` / `C-h a` / `C-h i` | describe-mode / apropos-command / info | モードの説明, コマンドの検索, マニュアル |

### モードとそのほか

| キー | コマンド | 説明 |
|------|---------|------|
| `<insert>` | overwrite-mode | 上書き |
| `C-x C-q` | read-only-mode | 読み込み専用 |
| - | auto-fill-mode / display-line-numbers-mode / visual-line-mode | 自動の折り返し, 行番号, 見た目の行 |
| `C-x C-+` / `C-x C--` | text-scale-adjust | 文字の大きさ |
| - | shell / eshell / compile / grep | 外部のプログラム |
| `C-x v ...` | vc-* | バージョン管理 |
| `M-$` | ispell-word | 綴りの確かめ |

---

## 削除された機能

括弧の自動補完は IDE 固有の機能で, Emacs の既定には無い。括弧の強調は, Emacs の既定 (show-paren-mode) なので, 作り直した。

**削除したもの:**
- ❌ **シンタックスハイライト** (`schemeTokenizer.ts`) - CodeMirror の Lisp のモード (`lispMode.ts`. Scheme と Emacs Lisp) で作り直した (Emacs の既定の font-lock と同じく, 色を付ける)
- ❌ **括弧マッチハイライト** (`findMatchingBracket`, `findUnmatchedClose` from `bracketMatch.ts`) - Emacs の show-paren-mode と同じく, `showParen.ts` で作り直した
- ❌ **括弧自動補完** (`bracketAutoClose.ts`) - IDE 固有機能
- ❌ **REPL コンポーネント** (`REPL.tsx`) - 評価は Emacs と同じく `C-x C-e` と `M-:` で行い、結果をミニバッファに出す

**保持したもの:**
- ✅ **S-expression ナビゲーション** (`forwardSexp`, `backwardSexp`, `killSexp`) - Emacs Lisp に対応する機能

---

## まとめ

rs-editor は、Emacs の基本的な編集（移動、kill-ring、Undo/Redo、検索と置換、ファイル、バッファ、ウィンドウ）と、Lisp のモード（字下げ、色、括弧の強調、評価）を実装しています。

次の候補は、よく使い、手間の小さいものから:
1. 編集: `M-m`
2. Lisp の移動: `C-M-u`, `C-M-d`, `C-M-a`, `C-M-e`
3. ヘルプ: `C-h f`, `C-h v`
4. Dired
