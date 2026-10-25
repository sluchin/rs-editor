# キーバインド一覧

rs-editor のキーバインド（ショートカット）一覧です。Emacs スタイルのキーバインドに準拠しています。

## 記号について

- `C-x` : Control キーを押しながら x キー
- `M-x` : Alt/Meta キーを押しながら x キー
- `C-M-x` : Control と Meta を同時に押しながら x キー
- `'` : クォート記号（`'(+ 1 2)` = `(quote (+ 1 2))` と等価）

## カーソル移動

| キー | コマンド | 説明 |
|------|---------|------|
| `C-f` | forward-char | 1 文字進む |
| `C-b` | backward-char | 1 文字戻る |
| `C-n` | next-line | 次の行へ |
| `C-p` | previous-line | 前の行へ |
| `C-a` | beginning-of-line | 行の開始へ |
| `C-e` | end-of-line | 行の末尾へ |
| `M-f` | forward-word | 1 単語進む |
| `M-b` | backward-word | 1 単語戻る |
| `M-<` | beginning-of-buffer | バッファの先頭へ |
| `M->` | end-of-buffer | バッファの末尾へ |
| `C-v` | scroll-up-command | 1 画面先へ (カーソルの行を画面の上に) |
| `M-v` | scroll-down-command | 1 画面前へ (カーソルの行を画面の下に) |
| `C-l` | recenter-top-bottom | カーソルの行を画面の中央へ (続けて押すと, 上, 下の順) |
| `M-g g` / `M-g M-g` | goto-line | 入れた行へ移動 |
| `M-m` | back-to-indentation | 行の字下げの後ろへ |
| `M-}` / `M-{` | forward-paragraph / backward-paragraph | 次の段落の後ろ・前の段落の前へ（段落は, 空白だけの行で分ける。scheme-mode では空の行だけ） |
| `M-e` / `M-a` | forward-sentence / backward-sentence | 文の終わり・始めへ（文の終わりは, `.` などの後に空白 2 つか行の終わり。`。` などは, そのまま。段落の終わりも文の終わり） |
| `C-x C-n` | set-goal-column | 今の桁を, 上下に移るときの桁にする（`C-u C-x C-n` で止める） |

`C-n` / `C-p` は, 続けて押すあいだ, 始めの桁を保ちます.

## S式移動（Scheme 特化）

| キー | コマンド | 説明 |
|------|---------|------|
| `C-M-f` | forward-sexp | 次の S 式へ移動 |
| `C-M-b` | backward-sexp | 前の S 式へ移動 |
| `C-M-u` | backward-up-list | 外のリストの始めへ（文字列の中では, 文字列の中のリストを見て, 無ければ文字列の始めへ）。`M-x up-list` は外のリストの終わりへ |
| `C-M-d` | down-list | 次のリストの中へ |
| `C-M-a` / `C-M-e` | beginning-of-defun / end-of-defun | defun（行の始めの開く括弧から始まる式）の始め・終わりへ |

S 式は, Emacs 30 の構文表（Fundamental モード, emacs-lisp-mode, scheme-mode）で読みます。文字列とコメント（`;`, Scheme の `#| |#`）の中の括弧は数えません。括弧が合わなければ, 動かずに知らせます（`No next sexp` など）。

## 削除・編集

| キー | コマンド | 説明 |
|------|---------|------|
| `C-d` | delete-char | カーソル位置の文字を削除 |
| `C-k` | kill-line | 行を削除（kill-ring に保存。行末では改行を削除） |
| `M-d` | kill-word | 次の単語を削除（kill-ring に保存） |
| `M-DEL` | backward-kill-word | 前の単語を削除（kill-ring に保存） |
| `C-w` | kill-region | 選択範囲を削除（kill-ring に保存） |
| `M-w` | kill-ring-save | 選択範囲をコピー（kill-ring に保存） |
| `C-y` | yank | kill-ring から貼り付け |
| `M-y` | yank-pop | kill-ring の別の内容を貼り付け |
| `C-t` | transpose-chars | 隣同士の文字を入れ替え |
| `M-t` | transpose-words | 前の単語と後ろの単語を入れ替え |
| `C-x C-t` | transpose-lines | 今の行と前の行を入れ替え |
| `M-;` | comment-dwim | コメントを付ける（リージョンがあれば、行をコメントにする・外す。コメントがあれば、コメントへ動く） |
| `M-/` | dabbrev-expand | 前の単語を、バッファの中の単語で補完（続けて押すと、次の候補） |
| `C-x C-u` / `C-x C-l` | upcase-region / downcase-region | リージョンを大文字・小文字にする |
| `C-q` | quoted-insert | 次に押すキーを、そのまま入れる（`C-q C-a` で制御文字。`C-q 1 0 1` のように、8 進数で文字のコードも入れられる。制御文字は `^A` のように出す） |
| `M-=` | count-words-region | リージョンの行、文、単語、文字の数を出す（`M-x count-words` は、リージョンが無ければバッファ全体） |
| `M-z` | zap-to-char | 次に押す文字までをキル（その文字を含む。前置引数があれば、その数だけ先の文字まで） |
| `M-q` | fill-paragraph | 段落を 70 桁に詰める（Lisp では、コメントの段落） |
| `M-SPC` | just-one-space | カーソルの前後の空白を 1 つにする |
| `M-\` | delete-horizontal-space | カーソルの前後の空白を消す |
| `M-^` | delete-indentation | 今の行を前の行につなげる（つないだ所の空白は 1 つ。括弧の内側は空白なし） |
| `C-x C-o` | delete-blank-lines | 空の行を消す（続く空の行は 1 行にする。1 行だけなら消す。空でない行では、後ろの空の行を消す） |
| `M-u` | upcase-word | 次の単語を大文字に |
| `M-l` | downcase-word | 次の単語を小文字に |
| `M-c` | capitalize-word | 次の単語の先頭を大文字に |
| `C-o` | open-line | カーソルを動かさずに改行を入れる |
| `C-/` / `C-_` / `C-x u` | undo | 操作を取り消す |
| `C-x C-/` | redo | 操作をやり直す |
| `C-M-k` | kill-sexp | 次の S 式を削除 |
| `M-k` | kill-sentence | 文の終わりまでを削除（kill-ring に保存） |

## 改行と字下げ

| キー | コマンド | 説明 |
|------|---------|------|
| `RET` / `C-m` | newline | 改行して, 新しい行を字下げする（前の行の字下げも直し, 前の行の終わりの空白を消す. Emacs の electric-indent-mode） |
| `C-j` | electric-newline-and-maybe-indent | 改行だけを入れる（字下げしない） |
| `TAB` / `C-i` | indent-for-tab-command | 行を字下げする（リージョンがあれば, リージョンの行を）. Fundamental モードでは, 前の行の次の字下げの位置まで空白を入れる |
| `C-M-q` | indent-sexp | カーソルの後ろの S 式の, 2 行目からを字下げする（Lisp だけ） |
| `C-M-\` | indent-region | リージョンの行を字下げする |

字下げは, Emacs 30 の scheme-mode と emacs-lisp-mode と同じです（`lisp-indent-function` の指定, コメントの `;` と `;;` と `;;;`）。Emacs の既定のとおり, タブを使い, タブの幅は 8 です。ファイルの 1 行目の `-*- indent-tabs-mode: nil; tab-width: 4 -*-` で変えられます。

カーソルの前の閉じる括弧か, 後ろの開く括弧と, 対になる括弧に色を付けます（Emacs の show-paren-mode）。対が無いか, 括弧の種類が違えば, 紫にします。

続けてキルすると（`C-k C-k` など）, kill-ring の 1 つの項目につなげます。kill-ring は, 全てのバッファで共通です。

## リージョン選択

| キー | コマンド | 説明 |
|------|---------|------|
| `C-SPC` | set-mark-command | マークを置く（移動すると, マークからカーソルまでが選ばれる） |
| `C-x C-x` | exchange-point-and-mark | カーソルとマークを入れ替え |
| `C-x h` | mark-whole-buffer | バッファ全体を選択 |
| `M-@` | mark-word | 次の単語の終わりにマークを置く（続けると, リージョンを延ばす） |
| `C-M-SPC` | mark-sexp | 次の S 式の終わりにマークを置く（続けると, リージョンを延ばす） |
| `C-M-h` | mark-defun | defun を選ぶ（前のコメントと空の行も。続けると, 次の defun まで延ばす） |
| `C-g` | keyboard-quit | マークを外す |

本文を書き換えると, マークは外れます（Emacs の transient-mark-mode）。

## 矩形・レジスタ・ブックマーク

| キー | コマンド | 説明 |
|------|---------|------|
| `C-x r k` | kill-rectangle | マークとカーソルを角とする矩形を消して, 覚える（kill-ring には入れない） |
| `C-x r y` | yank-rectangle | 最後に消した矩形を, カーソルを左上の角として入れる |
| `C-x r t` | string-rectangle | 矩形の各行を, ミニバッファで入力した文字で置き換える |
| `C-x r s` / `C-x r x` | copy-to-register | リージョンを, 次に押す文字のレジスタに入れる |
| `C-x r i` / `C-x r g` | insert-register | 次に押す文字のレジスタの中身を入れる（カーソルは, 入れた文字の前） |
| `C-x r m` | bookmark-set | ファイルの今の位置に, 名前を付けて覚える（既定の名前は, ファイルの名前） |
| `C-x r b` | bookmark-jump | ブックマークのファイルを開いて, 位置へ動く（Tab で名前を補完） |

矩形の桁は, タブを `tab-width` の倍数まで, 全角の文字を 2 桁と数えます。ブックマークは `~/.config/rseditor/bookmarks` に保存し, 次に起動したときも使えます。

## 検索

| キー | コマンド | 説明 |
|------|---------|------|
| `C-s` | isearch-forward | 前方に検索 |
| `C-r` | isearch-backward | 後方に検索 |
| `C-M-s` | isearch-forward-regexp | 正規表現で前方に検索 |
| `C-M-r` | isearch-backward-regexp | 正規表現で後方に検索 |
| `C-s` （検索中） | 次のマッチへ | 検索結果の次へ（探す文字が空なら, 最後に探した文字で探す） |
| `C-w` （検索中） | isearch-yank-word-or-char | カーソルの後ろの単語（単語でなければ 1 文字）を, 探す文字に足す |
| `M-y` （検索中） | isearch-yank-pop-only | 最後にキルした文字を, 探す文字に足す（続けると, 1 つ古いキルした文字に替える） |
| `M-p` / `M-n` （検索中） | isearch-ring-retreat / isearch-ring-advance | 検索の履歴の, 古い方・新しい方の文字で探す |
| `C-g` （検索中） | quit | 検索をキャンセル |

正規表現は JavaScript の書き方です（`^` と `$` は行の始めと終わり）。`C-w` と `M-y` で足した文字は, 正規表現の検索でも, そのまま合います。
検索の履歴は, 文字と正規表現で別です（16 個まで. `RET` で確定した文字を覚え, `C-g` でやめた文字は覚えない）。

### grep（ファイルから探す）

| キー | コマンド | 説明 |
|------|---------|------|
| `M-x grep` | grep | 正規表現とディレクトリを聞き, ディレクトリの下（深くまで）のファイルから探して, 結果を `*grep*` バッファに出す（外のコマンドは使わない） |
| `RET` （`*grep*`） | compile-goto-error | その行のファイルを, ほかのウィンドウに開く |
| `M-g n` / `M-g M-n` / `` C-x ` `` | next-error | 次の合った行へ |
| `M-g p` / `M-g M-p` | previous-error | 前の合った行へ |

正規表現は Rust の regex の書き方です。大文字が無ければ, 大文字と小文字を区別しません。`.` で始まるディレクトリ, `node_modules`, `target` と, 文字のファイルでないものは探しません。

## 置換

| キー | コマンド | 説明 |
|------|---------|------|
| `M-%` | query-replace | カーソルから後ろを, 1 つずつ聞いて置き換える |
| `C-M-%` | query-replace-regexp | 正規表現で, 1 つずつ聞いて置き換える |
| （`M-x`） | replace-string | カーソルから後ろを, 全て置き換える |
| （`M-x`） | replace-regexp | 正規表現で, 全て置き換える |

1 つずつ聞くときは, `y` / `SPC` で置き換え, `n` / `DEL` で飛ばし, `!` で残りを全て置き換え, `.` で置き換えて終え, `q` / `RET` で終えます（`C-g` で取り消し）。
置き換える文字では, `\&`（合った全体）と `\1` から `\9`（グループ）を使えます。

## 前置引数とキーボードマクロ

| キー | コマンド | 説明 |
|------|---------|------|
| `C-u` | universal-argument | 次のコマンドを 4 回（続けて押すと 16 回, 後に数字を押すとその回数）繰り返す |
| `M-0` から `M-9` | digit-argument | 次のコマンドを, その数だけ繰り返す |
| `C-x (` | start-kbd-macro | キーボードマクロの記録を始める |
| `C-x )` | end-kbd-macro | 記録を終える |
| `C-x e` | call-last-kbd-macro | 記録したマクロを実行する（`C-u 3 C-x e` で 3 回） |

前置引数の後の, 割り当てのない文字は, その数だけ入れます（`C-u 3 a` で `aaa`）。ファイルやバッファなど, 画面のコマンドは, 繰り返しません。
キーボードマクロは, エディタで押したキーを記録します。ミニバッファに入力した文字は, 記録しません。

## ファイル操作

| キー | コマンド | 説明 |
|------|---------|------|
| `C-x C-f` | find-file | ファイルを開く |
| `C-x C-s` | save-buffer | ファイルを保存 |
| `C-x C-w` | write-file | 別のファイルに保存（ほかのファイルを置き換えるときは、聞く） |
| `C-x C-v` | find-alternate-file | 選んでいるバッファを, 別のファイルを開いたバッファに置き換える（書き換えていれば, 聞く） |
| `C-x i` | insert-file | ファイルの中身を, カーソルの位置に入れる |
| `C-x s` | save-some-buffers | 変更したファイルのバッファを, 1 つずつ保存するか聞く（`y` / `SPC`, `n` / `DEL`, `!` で残りを全て, `.` で保存して終える, `q` / `RET` で終える） |
| （`M-x`） | revert-buffer | ファイルから読み直す（聞いてから。dired のバッファは, 聞かずに一覧を作り直す） |
| （`M-x`） | recover-this-file | 自動保存のファイルから読み直す |

ファイルのバッファは, 書き換えが 30 秒止まると, 同じディレクトリの `#名前#` に自動保存します（保存すると消す）。
開いたファイルの自動保存のファイルの方が新しければ, 開いたときに知らせます。
開いてから始めて保存するときは, 前の版を `名前~` に写します（バックアップ）。
## バッファ管理

| キー | コマンド | 説明 |
|------|---------|------|
| `C-x b` | switch-to-buffer | バッファを切り替え |
| `C-x k` | kill-buffer | 現在のバッファを閉じる |
| `C-x C-b` | list-buffers | バッファ一覧を表示 |
| `C-x C-c` | save-buffers-kill-terminal | 終了（保存していないバッファがあれば, 終わってよいか聞く） |

バッファごとに, カーソルの位置と, 元に戻す履歴を覚えています。

## ウィンドウ

| キー | コマンド | 説明 |
|------|---------|------|
| `C-x 2` | split-window-below | ウィンドウを上下に分ける |
| `C-x 3` | split-window-right | ウィンドウを左右に分ける |
| `C-x o` | other-window | 次のウィンドウへ移る（クリックでも選べる） |
| `C-x 0` | delete-window | 選んでいるウィンドウを消す |
| `C-x 1` | delete-other-windows | ほかのウィンドウを消す |

同じバッファを 2 つのウィンドウに出すと, 一方での書き換えは, もう一方にも写ります（カーソルと, 元に戻す履歴は, ウィンドウごと）。

## Scheme 評価

| キー | コマンド | 説明 |
|------|---------|------|
| `C-x C-e` | eval-last-sexp | カーソル直前の S 式を評価 |
| `M-:` | eval-expression | ミニバッファで式を入力して評価 |

## コマンド実行

| キー | コマンド | 説明 |
|------|---------|------|
| `M-x` | execute-extended-command | コマンドを名前で実行（この一覧のコマンドの名前は, 全て使える） |
| `TAB` （ミニバッファ） | 補完 | `M-x` のコマンドの名前, `C-x b` のバッファの名前, `C-x C-f` と保存先のファイルの名前を補完する（候補が複数なら一覧を出す） |
| `M-p` / `M-n` （ミニバッファ） | previous-history-element / next-history-element | 入力の履歴の, 古い方・新しい方を出す（履歴は, コマンドの種類ごと。ファイルの名前は, `C-x C-f`, `C-x C-w`, `C-x d` などで共通） |

## ヘルプ

| キー | コマンド | 説明 |
|------|---------|------|
| `C-h k` | describe-key | 次に押すキーのコマンドを, 実行せずにミニバッファに出す |
| `C-h b` | describe-bindings | キーの割り当ての一覧を `*Help*` バッファに出す（Lisp の割り当てを含む） |

## その他

| キー | コマンド | 説明 |
|------|---------|------|
| `C-g` | keyboard-quit | 操作をキャンセル（Quit メッセージ表示） |
| `F8` | project-tree | 左のサイドバーに, プロジェクト（`.git` のある所）のファイルのツリーを出すか消す（ディレクトリを押すと開き, ファイルを押すと開く） |

## Dired（ディレクトリの編集）

Emacs 30 の dired の既定のキーです（`emacs -Q` の `dired-mode-map` から取り出した）。**rs-editor の dired は, `C-x d` でディレクトリの一覧（`ls -al`）を出すだけで, dired の中のキーは, 全て未実装です。**
一覧は `M-x revert-buffer` で作り直せます。
rs-filer のキーとの違いは [DIRED_VS_RS_FILER.md](DIRED_VS_RS_FILER.md) にまとめています。

### Dired を開く

| キー | コマンド | 説明 | 状況 |
|------|---------|------|------|
| `C-x d` | dired | ディレクトリを開く | 実装済み（一覧を出すだけ） |
| `C-x 4 d` | dired-other-window | 別のウィンドウでディレクトリを開く | 未実装 |
| `C-x 5 d` | dired-other-frame | 別のフレームでディレクトリを開く | 未実装 |
| `C-x C-j` | dired-jump | 今のファイルのディレクトリを開き, そのファイルにカーソルを合わせる | 未実装 |
| `C-x 4 C-j` | dired-jump-other-window | `C-x C-j` を別のウィンドウで | 未実装 |

### 移動

| キー | コマンド | 説明 | 状況 |
|------|---------|------|------|
| `n` / `SPC` / `C-n` | dired-next-line | 次の行へ | 未実装 |
| `p` / `S-SPC` / `C-p` | dired-previous-line | 前の行へ | 未実装 |
| `>` | dired-next-dirline | 次のディレクトリの行へ | 未実装 |
| `<` | dired-prev-dirline | 前のディレクトリの行へ | 未実装 |
| `j` | dired-goto-file | 名前を入れて, そのファイルの行へ | 未実装 |
| `^` | dired-up-directory | 親ディレクトリへ | 未実装 |
| `M-}` / `* C-n` | dired-next-marked-file | 次のマークしたファイルへ | 未実装 |
| `M-{` / `* C-p` | dired-prev-marked-file | 前のマークしたファイルへ | 未実装 |
| `C-M-n` | dired-next-subdir | 次のサブディレクトリの見出しへ | 未実装 |
| `C-M-p` | dired-prev-subdir | 前のサブディレクトリの見出しへ | 未実装 |
| `C-M-u` | dired-tree-up | 上のサブディレクトリの見出しへ | 未実装 |
| `C-M-d` | dired-tree-down | 下のサブディレクトリの見出しへ | 未実装 |
| `M-G` | dired-goto-subdir | 名前を入れて, そのサブディレクトリの見出しへ | 未実装 |

### 開く・見る

| キー | コマンド | 説明 | 状況 |
|------|---------|------|------|
| `RET` / `e` / `f` | dired-find-file | ファイルを開く（ディレクトリなら, その dired） | 未実装 |
| `o` | dired-find-file-other-window | 別のウィンドウで開く | 未実装 |
| `C-o` | dired-display-file | 別のウィンドウに出す（カーソルは dired に残る） | 未実装 |
| `v` | dired-view-file | 読むだけのモードで開く | 未実装 |
| `a` | dired-find-alternate-file | dired のバッファを, 開いたファイルに置き換える（既定では無効） | 未実装 |
| `E` | dired-do-open | 外部のアプリで開く | 未実装 |
| `W` | browse-url-of-dired-file | ブラウザで開く | 未実装 |
| `y` | dired-show-file-type | ファイルの種類を表示（`file` コマンド） | 未実装 |
| `i` | dired-maybe-insert-subdir | サブディレクトリの中身を, 同じバッファの下に足す | 未実装 |
| `$` | dired-hide-subdir | サブディレクトリを畳む / 開く | 未実装 |
| `M-$` | dired-hide-all | 全てのサブディレクトリを畳む / 開く | 未実装 |
| `k` | dired-do-kill-lines | マークした行を表示から消す（ファイルは消さない） | 未実装 |
| `l` | dired-do-redisplay | マークした行を読み直す | 未実装 |
| `g` | revert-buffer | 一覧を読み直す | 未実装（`M-x revert-buffer` で作り直せる） |
| `s` | dired-sort-toggle-or-edit | 名前順と日付順を切り替える | 未実装 |
| `(` | dired-hide-details-mode | 詳細（権限, サイズ, 日付）を隠す / 出す | 未実装 |
| `w` | dired-copy-filename-as-kill | ファイル名を kill-ring にコピー（`C-u 0 w` でフルパス） | 未実装 |

### マーク

| キー | コマンド | 説明 | 状況 |
|------|---------|------|------|
| `m` / `* m` | dired-mark | マークする | 未実装 |
| `u` / `* u` | dired-unmark | マークを外す | 未実装 |
| `DEL` / `* DEL` | dired-unmark-backward | 前の行のマークを外す | 未実装 |
| `U` / `* !` | dired-unmark-all-marks | 全てのマークを外す | 未実装 |
| `M-DEL` / `* ?` | dired-unmark-all-files | 指定した文字のマークを全て外す | 未実装 |
| `t` / `* t` | dired-toggle-marks | マークを反転 | 未実装 |
| `* *` | dired-mark-executables | 実行できるファイルをマーク | 未実装 |
| `* /` | dired-mark-directories | ディレクトリをマーク | 未実装 |
| `* @` | dired-mark-symlinks | シンボリックリンクをマーク | 未実装 |
| `* s` | dired-mark-subdir-files | 今のサブディレクトリのファイルを全てマーク | 未実装 |
| `* %` / `% m` | dired-mark-files-regexp | 名前が正規表現に合うファイルをマーク | 未実装 |
| `% g` | dired-mark-files-containing-regexp | 中身が正規表現に合うファイルをマーク | 未実装 |
| `* c` | dired-change-marks | マークの文字を変える | 未実装 |
| `* N` | dired-number-of-marked-files | マークした数とサイズを表示 | 未実装 |

### 削除のフラグ

| キー | コマンド | 説明 | 状況 |
|------|---------|------|------|
| `d` | dired-flag-file-deletion | 削除のフラグ（`D`）を付ける | 未実装 |
| `x` | dired-do-flagged-delete | フラグを付けたファイルを削除 | 未実装 |
| `#` | dired-flag-auto-save-files | 自動保存のファイルにフラグ | 未実装 |
| `~` | dired-flag-backup-files | バックアップのファイルにフラグ | 未実装 |
| `.` | dired-clean-directory | 古い番号付きのバックアップにフラグ | 未実装 |
| `% d` | dired-flag-files-regexp | 名前が正規表現に合うファイルにフラグ | 未実装 |
| `% &` | dired-flag-garbage-files | 要らないファイル（`.o` など）にフラグ | 未実装 |

### ファイルの操作（マークしたファイル, 無ければカーソルの行）

| キー | コマンド | 説明 | 状況 |
|------|---------|------|------|
| `C` | dired-do-copy | コピー | 未実装 |
| `R` | dired-do-rename | 名前の変更・移動 | 未実装 |
| `D` | dired-do-delete | 削除（確認あり） | 未実装 |
| `+` | dired-create-directory | ディレクトリを作る | 未実装 |
| `H` | dired-do-hardlink | ハードリンクを作る | 未実装 |
| `S` | dired-do-symlink | シンボリックリンクを作る | 未実装 |
| `Y` | dired-do-relsymlink | 相対パスのシンボリックリンクを作る | 未実装 |
| `M` | dired-do-chmod | 権限を変える | 未実装 |
| `G` | dired-do-chgrp | グループを変える | 未実装 |
| `O` | dired-do-chown | 所有者を変える | 未実装 |
| `T` | dired-do-touch | 更新日時を変える | 未実装 |
| `Z` | dired-do-compress | 圧縮 / 展開 | 未実装 |
| `c` | dired-do-compress-to | 1 つのアーカイブにまとめる | 未実装 |
| `=` | dired-diff | 別のファイルと diff | 未実装 |
| `!` / `X` | dired-do-shell-command | シェルのコマンドを実行 | 未実装 |
| `&` | dired-do-async-shell-command | シェルのコマンドを非同期で実行 | 未実装 |
| `A` | dired-do-find-regexp | 中身を正規表現で検索 | 未実装 |
| `Q` | dired-do-find-regexp-and-replace | 中身を正規表現で置換 | 未実装 |
| `B` | dired-do-byte-compile | Emacs Lisp をバイトコンパイル | 未実装 |
| `L` | dired-do-load | Emacs Lisp を読み込む | 未実装 |
| `I` | dired-do-info | Info のファイルとして開く | 未実装 |
| `N` | dired-do-man | man のページとして開く | 未実装 |
| `P` | dired-do-print | 印刷 | 未実装 |
| `% C` | dired-do-copy-regexp | 正規表現で名前を変えてコピー | 未実装 |
| `% R` / `% r` | dired-do-rename-regexp | 正規表現で名前を変える | 未実装 |
| `% H` / `% S` / `% Y` | dired-do-hardlink-regexp ほか | 正規表現で名前を変えてリンクを作る | 未実装 |
| `% u` / `% l` | dired-upcase / dired-downcase | 名前を大文字 / 小文字にする | 未実装 |
| `: e` / `: d` / `: s` / `: v` | epa-dired-do-encrypt ほか | GnuPG で暗号化 / 復号 / 署名 / 検証 | 未実装 |
| `C-t` で始まるキー | image-dired-* | 画像のサムネイル（`C-t d` で表示など） | 未実装 |

### そのほか

| キー | コマンド | 説明 | 状況 |
|------|---------|------|------|
| `C-x C-q` | dired-toggle-read-only | 一覧を編集して名前を変える（wdired） | 未実装 |
| `C-/` / `C-x u` | dired-undo | マークなどを元に戻す | 未実装 |
| `M-s f C-s` / `M-s f C-M-s` | dired-isearch-filenames(-regexp) | ファイル名だけを isearch | 未実装 |
| `M-s a C-s` / `M-s a C-M-s` | dired-do-isearch(-regexp) | マークしたファイルの中身を isearch | 未実装 |
| `?` | dired-summary | 主なキーを 1 行で表示 | 未実装 |
| `h` | describe-mode | dired の説明を表示 | 未実装 |
| `q` | quit-window | dired のウィンドウを閉じる | 未実装 |

## 実装状況

### 完全実装済み
- カーソル移動（C-f, C-b, C-n, C-p, C-a, C-e, M-f, M-b, M-<, M->, C-v, M-v, C-l, M-g g, M-m, M-{, M-}, M-a, M-e, C-x C-n）
- S 式移動（C-M-f, C-M-b, C-M-u, C-M-d, C-M-a, C-M-e）
- 削除・編集（C-d, C-k, M-d, M-DEL, C-w, M-w, C-y, M-y, C-t, M-t, C-x C-t, M-u, M-l, M-c, C-o, C-/, C-x C-/, C-M-k, M-k, M-;, M-/, M-z, M-q, M-SPC, M-\, M-^, C-x C-o, C-x C-u, C-x C-l, C-q, M-=）
- リージョン選択（C-SPC, C-x C-x, C-x h, M-@, C-M-SPC, C-M-h）
- 矩形・レジスタ・ブックマーク（C-x r k, C-x r y, C-x r t, C-x r s, C-x r i, C-x r m, C-x r b）
- 改行と字下げ（RET, C-j, TAB, C-M-q, C-M-\）, 括弧の強調
- 検索（C-s, C-r, C-M-s, C-M-r. 検索中の C-w, M-y, M-p, M-n）
- 置換（M-%, C-M-%, replace-string, replace-regexp）
- 前置引数（C-u, M-0 から M-9）, キーボードマクロ（C-x (, C-x ), C-x e）
- ファイル操作（C-x C-f, C-x C-s, C-x C-w, C-x C-v, C-x i, C-x s, revert-buffer, recover-this-file）, 自動保存とバックアップ
- バッファ管理（C-x b, C-x k, C-x C-b, C-x C-c）
- ウィンドウ（C-x 2, C-x 3, C-x o, C-x 0, C-x 1）
- Scheme 評価（C-x C-e, M-:）
- コマンド実行（M-x）, ミニバッファの入力の履歴（M-p, M-n）
- ヘルプ（C-h k, C-h b）
- その他（C-g）

### 未実装
- そのほかのヘルプ（C-h f, C-h v など）
- Dired の中の全てのキー（C-x d は, 一覧を出すだけ）

## howm (`lisp/howm.sl` を読み込んだとき)

| キー | コマンド | 説明 |
|------|----------|------|
| `M-x howm`, `C-c , ,` | `howm-menu` | メニューを出す |
| `C-c , c` | `howm-create` | メモを作る |
| `C-c , a` / `C-c , l` | `howm-list-all` / `howm-list-recent` | 一覧 / 最近のメモ |
| `C-c , s` / `C-c , g` | `howm-search-fixed` / `howm-search-regexp` | 文字 / 正規表現で探す |
| `C-c , .` / `C-c , :` | `howm-today` / `howm-yesterday` | 今日 / 昨日のメモ |
| `C-c , y` / `C-c , t` | `howm-list-schedule` / `howm-list-todo` | 予定 / todo |
| `C-c , RET` | `howm-action` | ボタン, 項目, リンクを動かす |

メニューでは, ボタンの前の文字 (`c`, `s`, `g`, `:`, `.`, `a`, `l`, `y`, `t`, `R`) と, ボタンや項目の上の `RET` が使えます。
一覧 (`*howmS*`) では, `RET` でその行のメモを開き, `n` と `p` で行を動きます。
howm から開いたメモでは, `>>> 言葉` と `<<< 言葉` の上の `RET` で, その言葉を探します (ほかの所では, 改行)。

## よくある操作パターン

### S 式の評価
```
1. カーソルをS式の直後に移動: C-f / C-b などで移動
2. C-x C-e で評価
3. 結果がミニバッファに表示される
```

### テキスト選択と削除
```
1. C-SPC でマーク設定
2. C-f / C-b / M-f / M-b などでカーソルを移動
3. C-w で選択範囲を削除
```

### 検索と置換
```
1. C-s で検索開始
2. 検索語を入力
3. C-s で次のマッチへ（または Esc でキャンセル）
```

### ファイルを開いて編集
```
1. C-x C-f でファイル選択
2. ファイルが新しいバッファで開く
3. 編集後 C-x C-s で保存
```

## Scheme インタプリタの使用

### 単一の式を評価
```
(+ 1 2 3)
⌨ C-x C-e
→ ミニバッファに「6」が表示
```

### 複数の式を入力して評価
```
M-: でプロンプト「Eval: 」を表示
(* 2 3 4) を入力して Enter
→ 結果「24」がミニバッファに表示
```

## トラブルシューティング

### キーが効かない
- 別のダイアログ（バッファ一覧など）が開いていないか確認
- ミニバッファが入力受付中でないか確認

### 検索中にキーが効かない
- Esc または C-g で検索をキャンセル
- 検索中は検索専用のキーバインド（C-s, C-w, M-y, M-p, M-n など）のみが有効

### バッファが閉じられない
- 未保存の変更がある場合、先に C-x C-s で保存が必要
- または完全に削除する場合、別の方法を検討
