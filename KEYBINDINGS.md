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

`C-n` / `C-p` は, 続けて押すあいだ, 始めの桁を保ちます.

## S式移動（Scheme 特化）

| キー | コマンド | 説明 |
|------|---------|------|
| `C-M-f` | forward-sexp | 次の S 式へ移動 |
| `C-M-b` | backward-sexp | 前の S 式へ移動 |

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
| `M-u` | upcase-word | 次の単語を大文字に |
| `M-l` | downcase-word | 次の単語を小文字に |
| `M-c` | capitalize-word | 次の単語の先頭を大文字に |
| `C-o` | open-line | カーソルを動かさずに改行を入れる |
| `C-/` / `C-_` / `C-x u` | undo | 操作を取り消す |
| `C-x C-/` | redo | 操作をやり直す |
| `C-M-k` | kill-sexp | 次の S 式を削除 |

続けてキルすると（`C-k C-k` など）, kill-ring の 1 つの項目につなげます。kill-ring は, 全てのバッファで共通です。

## リージョン選択

| キー | コマンド | 説明 |
|------|---------|------|
| `C-SPC` | set-mark-command | マークを置く（移動すると, マークからカーソルまでが選ばれる） |
| `C-x C-x` | exchange-point-and-mark | カーソルとマークを入れ替え |
| `C-x h` | mark-whole-buffer | バッファ全体を選択 |
| `C-g` | keyboard-quit | マークを外す |

本文を書き換えると, マークは外れます（Emacs の transient-mark-mode）。

## 検索

| キー | コマンド | 説明 |
|------|---------|------|
| `C-s` | isearch-forward | 前方に検索 |
| `C-r` | isearch-backward | 後方に検索 |
| `C-M-s` | isearch-forward-regexp | 正規表現で前方に検索 |
| `C-M-r` | isearch-backward-regexp | 正規表現で後方に検索 |
| `C-s` （検索中） | 次のマッチへ | 検索結果の次へ |
| `C-g` （検索中） | quit | 検索をキャンセル |

正規表現は JavaScript の書き方です（`^` と `$` は行の始めと終わり）。

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

## バッファ管理

| キー | コマンド | 説明 |
|------|---------|------|
| `C-x b` | switch-to-buffer | バッファを切り替え |
| `C-x k` | kill-buffer | 現在のバッファを閉じる |
| `C-x C-b` | list-buffers | バッファ一覧を表示 |
| `C-x C-c` | save-buffers-kill-terminal | 終了（保存していないバッファがあれば, 終わってよいか聞く） |

バッファごとに, カーソルの位置と, 元に戻す履歴を覚えています。

## Scheme 評価

| キー | コマンド | 説明 |
|------|---------|------|
| `C-x C-e` | eval-last-sexp | カーソル直前の S 式を評価 |
| `M-:` | eval-expression | ミニバッファで式を入力して評価 |

## コマンド実行

| キー | コマンド | 説明 |
|------|---------|------|
| `M-x` | execute-extended-command | コマンドを名前で実行（この一覧のコマンドの名前は, 全て使える） |

## ヘルプ

| キー | コマンド | 説明 |
|------|---------|------|
| `C-h k` | describe-key | 次に押すキーのコマンドを, 実行せずにミニバッファに出す |
| `C-h b` | describe-bindings | キーの割り当ての一覧を `*Help*` バッファに出す（Lisp の割り当てを含む） |

## その他

| キー | コマンド | 説明 |
|------|---------|------|
| `C-g` | keyboard-quit | 操作をキャンセル（Quit メッセージ表示） |

## Dired（ディレクトリの編集）

Emacs 30 の dired の既定のキーです（`emacs -Q` の `dired-mode-map` から取り出した）。**rs-editor には dired が無いので, 全て未実装です。**
rs-filer のキーとの違いは [DIRED_VS_RS_FILER.md](DIRED_VS_RS_FILER.md) にまとめています。

### Dired を開く

| キー | コマンド | 説明 | 状況 |
|------|---------|------|------|
| `C-x d` | dired | ディレクトリを開く | 未実装 |
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
| `g` | revert-buffer | 一覧を読み直す | 未実装 |
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
- カーソル移動（C-f, C-b, C-n, C-p, C-a, C-e, M-f, M-b, M-<, M->, C-v, M-v, C-l, M-g g）
- S 式移動（C-M-f, C-M-b）
- 削除・編集（C-d, C-k, M-d, M-DEL, C-w, M-w, C-y, M-y, C-t, M-u, M-l, M-c, C-o, C-/, C-x C-/, C-M-k）
- リージョン選択（C-SPC, C-x C-x, C-x h）
- 検索（C-s, C-r, C-M-s, C-M-r）
- 置換（M-%, C-M-%, replace-string, replace-regexp）
- 前置引数（C-u, M-0 から M-9）, キーボードマクロ（C-x (, C-x ), C-x e）
- ファイル操作（C-x C-f, C-x C-s）
- バッファ管理（C-x b, C-x k, C-x C-b, C-x C-c）
- Scheme 評価（C-x C-e, M-:）
- コマンド実行（M-x）
- ヘルプ（C-h k, C-h b）
- その他（C-g）

### 未実装
- ウィンドウ分割（C-x 2, C-x 3, C-x o, C-x 0, C-x 1）
- そのほかのヘルプ（C-h f, C-h v など）
- Dired（C-x d と, dired の中の全てのキー）

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
- 検索中は検索専用のキーバインド（C-s など）のみが有効

### バッファが閉じられない
- 未保存の変更がある場合、先に C-x C-s で保存が必要
- または完全に削除する場合、別の方法を検討
