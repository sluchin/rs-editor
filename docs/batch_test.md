# バッチモード比較試験仕様書

Emacs のバッチモード (`emacs --batch`) と rs-editor のバッチモード (`rseditor-tui --batch`) で同じ試験を動かし、結果を比べる試験の仕様です。
画面を使わずに、核の動き (バッファ、検索、ファイル、フック、終了コード) を Emacs の動きと突き合わせます。

## 1. 目的と範囲

- 目的は、rs-editor の核が、Emacs と同じ結果を返すことを、自動で確かめること。
- 範囲は、バッファの編集、検索、kill と narrowing、バッファの操作、ファイルを開くこと、書くこと、保存、バックアップ、自動保存、フック、ファイル名と文字列の関数、終了コード、コマンドラインの引数。
- 範囲外は、画面に出ること (`docs/integration_test.md` が確かめる)、Emacs の正規表現の細かい違い (rs-editor は Rust の正規表現)、メジャーモード (`major-mode` の名前が違う)。

## 2. 試験の方法

試験は、`tests/batch/cases/` に、Emacs Lisp (`NAME.el`) と Scheme (`NAME.scm`) の対で置く。言語が違うので、同じ動きを 2 つの言語で書く。

1. 試験ごとに、空のディレクトリを 2 つ (Emacs 用と rs-editor 用) 作り、同じ初期ファイルを置く。
2. Emacs は `emacs -Q --batch --eval '(setq create-lockfiles nil)' -l NAME.el`、rs-editor は `rseditor-tui --batch -l NAME.scm` で動かす。
3. 次の 3 つを比べる。1 つでも違えば失敗。
   - 標準出力。
   - 終了コード。
   - 動かした後のディレクトリのファイル (名前と中身)。バックアップ (`名前~`) と自動保存 (`#名前#`) もここで比べる。
4. 作業用のディレクトリは、リポジトリの `target/` の下に作る。Emacs は `/tmp` の下のファイルをバックアップしないため。ディレクトリの名前は、比べる前に `@T@` に置き換える。
5. `-e` を付けると、標準エラー出力 (`message` の出力など) も比べる。標準出力とは別に比べる (Emacs は標準出力を溜めてから出すので、混ぜると順序が合わない)。

### 2.1 初期ファイル

| ファイル | 中身 |
| --- | --- |
| `a.txt` | `old` と改行 |
| `multi.txt` | `line1`、`line2`、`line3` (各行に改行) |
| `sub/` | 空のディレクトリ |

### 2.2 実行

```sh
tests/batch/run.sh              # 全部
tests/batch/run.sh save write   # 名前に save か write を含むものだけ
tests/batch/run.sh -l           # 試験の一覧
tests/batch/run.sh -e           # 標準エラー出力も比べる
tests/batch/run.sh -k           # 作業用のディレクトリを残す (中の emacs.out と rs.out を見る)
tests/batch/run.sh -v           # 成功した試験の出力も出す
EMACS=/opt/emacs/bin/emacs RSEDITOR=./target/release/rseditor-tui tests/batch/run.sh
```

失敗した試験は、Emacs の出力 (`-`) と rs-editor の出力 (`+`) の差分を出す。終了コードは、全部成功なら 0、1 つでも失敗があれば 1。

## 3. rs-editor のバッチモードの仕様

試験が前提にする、rs-editor のバッチモード (`rseditor-tui --batch`。実装は `tui/src/batch.rs`) の仕様です。Emacs と同じ動きにする。

| 項目 | 仕様 |
| --- | --- |
| 引数の処理 | 左から順に処理する |
| `-l FILE`、`--load FILE` | FILE を読む |
| `--eval EXPR`、`--eval=EXPR` | EXPR を評価する |
| `-f FUNC`、`--funcall FUNC` | 引数の無いコマンドか関数 FUNC を呼ぶ |
| `--script FILE` | `--batch -l FILE` と同じ |
| 引数 `FILE` | FILE を訪れ、現在のバッファにする (`find-file`) |
| 初期設定ファイル | 読まない |
| 出力 | `princ`、`prin1`、`print`、`terpri` は標準出力。`message` は標準エラー出力 (改行を付ける) |
| 入力 | `read-string` などは、打ったことにするキー (`execute-kbd-macro`) が先、無ければ標準入力から読む (試験では空) |
| キーのコマンド | `execute-kbd-macro` で打ったキーを、その場でコマンドループ (`CommandLoop`) に渡して動かす。動かしている間、`executing-kbd-macro` が真。コマンドが失敗したら、そこで止めてエラーにする。コマンドは、現在のバッファでなく、`switch-to-buffer` したバッファで動く |
| 終了 | `(kill-emacs N)` で終了コード N。全部評価し終えたら 0 |
| エラー | 捕まえなかったエラーは標準エラー出力に出し、終了コード 255 |
| 変数 | `noninteractive` が真 |
| 相対パス | 起動したディレクトリを基にする (Emacs の `default-directory`) |
| `write-region` | `Wrote ...` のメッセージを出さない (Emacs の `noninteractive` と同じ) |

## 4. 試験項目

### 4.1 編集と検索

| ID | 試験 | 確かめること |
| --- | --- | --- |
| B-01 | `01-edit-basic` | `insert`、`point`、`point-min`、`point-max`、`goto-char`、`buffer-substring`、`delete-region`、`erase-buffer`、`buffer-size` |
| B-02 | `02-search` | `search-forward` (位置と `point`、見つからないときの `noerror`)、`re-search-forward`、`search-backward`、失敗のエラーメッセージ |
| B-03 | `03-kill-narrow` | `kill-region` と `yank`、`narrow-to-region` と `widen`、`save-excursion` が `point` を保つこと |

### 4.2 バッファ

| ID | 試験 | 確かめること |
| --- | --- | --- |
| B-04 | `04-buffers` | `generate-new-buffer` と `generate-new-buffer-name` の名前 (`foo`、`foo<2>`)、`rename-buffer`、`buffer-list`、`kill-buffer`、同じ名前に変えるとエラーになること |

### 4.3 ファイル

| ID | 試験 | 確かめること |
| --- | --- | --- |
| B-05 | `05-find-file-existing` | `find-file-noselect` が、バッファの名前、ファイルの名前、中身、`point`、書き換えていないこと、2 度目に同じバッファを返すことを満たす |
| B-06 | `06-find-file-new` | 無いファイルを開くと、空のバッファがそのファイルを訪れ、書き換えていないこと |
| B-07 | `07-insert-file-contents` | 返す値 (ファイルの名前と文字数)、`point` が入れた文字の前に残ること、書き換えたことになること、無いファイルのエラー |
| B-08 | `08-write-region` | 全体、範囲、文字列、足す (APPEND)、VISIT (書き換えていないことになり、バッファがファイルを訪れる) |
| B-09 | `09-save-backup` | `save-buffer` で書き換えていないことになる。初めの保存でだけ `名前~` を作る。新しいファイルは、初めの保存では作らず、2 度目の保存で作る。最後が改行でないバッファは、保存するときに改行を足す (`mode-require-final-newline`) |
| B-10 | `10-write-file` | `write-file` でバッファの名前が変わる。`set-visited-file-name` で名前が変わり、書き換えたことになる。`basic-save-buffer` で書く |
| B-11 | `11-copy-file` | 写す。既にあるとエラー (メッセージも比べる)。`OK-IF-ALREADY-EXISTS` で置き換える。元が無いとエラー |
| B-12 | `12-auto-save` | `do-auto-save` で `#名前#` ができる。`save-buffer` で消える |
| B-13 | `13-hooks` | `find-file-hook` と `after-save-hook` が、動く順と、そのときの現在のバッファで動く。既に開いたファイルを開き直しても、`find-file-hook` が動かない |
| B-17 | `17-find-file-directory` | ディレクトリを `find-file-noselect` すると、dired のバッファを返す |

### 4.4 名前と文字列

| ID | 試験 | 確かめること |
| --- | --- | --- |
| B-14 | `14-names-strings` | `file-name-nondirectory`、`file-name-directory`、`expand-file-name`、`string-trim`、`split-string`、`format` (`%5d`、`%-3s`、リスト)、正規表現の位置 |

### 4.5 終了とエラー

| ID | 試験 | 確かめること |
| --- | --- | --- |
| B-15 | `15-exit-code` | `(kill-emacs 3)` で終了コードが 3 になり、後ろを評価しない |
| B-16 | `16-error-exit` | 捕まえなかったエラーで終了コードが 255 になり、後ろを評価しない。それまでの出力は残る |

### 4.6 コマンドラインの引数

引数の試験は、`tests/batch/run.sh` の中に書いている。

| ID | 試験 | 確かめること |
| --- | --- | --- |
| C-01 | `cli-eval` | `--eval` を 2 つ、順に評価する |
| C-02 | `cli-load-order` | `-l`、`--eval`、`-l` を、書いた順に処理する |
| C-03 | `cli-funcall` | `-l` で定義した関数を `-f` で呼ぶ |
| C-04 | `cli-visit-file` | `FILE` を訪れ、バッファの名前と中身が分かる |
| C-05 | `cli-script` | `--script` で読む |

### 4.7 編集のコマンド

Emacs と rs-editor で、コマンドと関数の結果 (本文、カーソル、マーク、キルリング、レジスタ) が同じことを確かめる。

| ID | 試験 | 確かめること |
| --- | --- | --- |
| E-20 | `20-motion` | `forward-char`、`forward-line` の戻り値、`forward-word`、`beginning-of-line`、`end-of-line` (引数つき)、`line-beginning-position`、`count-lines`、`move-to-column`、`bobp` と `eobp` |
| E-21 | `21-delete-insert` | `delete-horizontal-space`、`just-one-space`、`delete-char`、`newline`、`open-line`、`delete-blank-lines`、`delete-indentation`、`insert-char` |
| E-22 | `22-case-words` | `upcase-region`、`capitalize-region`、`upcase-word`、`kill-word`、`backward-kill-word`、`transpose-words`、`transpose-chars` |
| E-23 | `23-kill-lines` | `kill-line` (引数つき)、`kill-whole-line`、`yank`、`current-kill`、`copy-region-as-kill`、`zap-to-char` |
| E-24 | `24-marks` | `set-mark`、`region-beginning`、`exchange-point-and-mark`、`push-mark`、`pop-mark`、`mark-whole-buffer`、`mark-word`、`kill-ring-save` (範囲の引数)、マーカーの移動 |
| E-25 | `25-registers` | `copy-to-register`、`insert-register`、`point-to-register`、`number-to-register`、`increment-register`、`append-to-register`、`prepend-to-register` |
| E-26 | `26-rectangles` | `extract-rectangle`、`delete-rectangle`、`kill-rectangle`、`yank-rectangle`、`string-rectangle`、`open-rectangle`、`clear-rectangle` |
| E-27 | `27-paragraphs-fill` | `forward-paragraph`、`backward-paragraph`、`fill-paragraph`、`fill-region`、`set-fill-column`、`mark-paragraph` |
| E-28 | `28-sort-lines` | `sort-lines` (昇順と降順)、`reverse-region`、`flush-lines`、`keep-lines`、`how-many`、`delete-duplicate-lines` |
| E-29 | `29-replace-match` | `match-beginning`、`match-end`、`match-string`、`replace-match`、`looking-at`、`looking-back`、`replace-string`、`replace-regexp`、`replace-regexp-in-string`、`regexp-quote` |
| E-30 | `30-undo` | `undo-boundary` と `undo` の組 |
| E-31 | `31-transpose-indent` | `transpose-lines`、`current-indentation`、`back-to-indentation`、`indent-rigidly`、`indent-to`、`indent-line-to`、`indent-region` (COLUMN つき)、`delete-trailing-whitespace` |
| E-32 | `32-sexp` | `forward-sexp`、`backward-sexp`、`up-list`、`down-list`、`backward-up-list`、`kill-sexp`、`transpose-sexps`、行き過ぎのエラー |
| E-33 | `33-narrow-widen` | `narrow-to-region` 中の `point-min` と `point-max`、範囲外へのエラー、`save-restriction`、`widen` |
| E-34 | `34-yank-pop` | `yank` の後の `yank-pop` (キルリングを回す) |
| E-35 | `35-bookmarks` | `bookmark-set`、`bookmark-jump` (別のバッファから)、未登録の名前のエラー、`bookmark-set-no-overwrite` |
| E-36 | `36-kmacro` | `kmacro-insert-counter` (カウンターと知らせ)、`kmacro-start-macro`、`kmacro-end-macro` |
| E-37 | `37-dired` | `dired-noselect` (バッファの名前、`..`、ファイルの一覧、同じ場所なら同じバッファ) |
| E-38 | `38-search-funcs` | `search-forward` と `search-backward` (大文字小文字、COUNT、BOUND、NOERROR)、`re-search-*`、`search-forward-regexp`、`word-search-forward`、`line-number-at-pos` |
| E-39 | `39-kbd-macro` | `execute-kbd-macro` (`kbd` のキー、回数)、文字キーの挿入、`M-f` などの編集キー、マクロの中の `C-x (` と `C-x )` (マクロを動かしている間は終えない)、`last-kbd-macro` |
| E-40 | `40-kbd-macro-errors` | マクロの中のキーが失敗したとき (エラーの文、そこで止まる)、回数 0 (失敗するまで繰り返す)、空のマクロ |

## 5. 比べない違い

| 項目 | 理由 |
| --- | --- |
| 言語 | Emacs は Emacs Lisp、rs-editor は Scheme。試験は言語ごとに書く (`nil` と `#f`、`defun` と `define` など)。真偽は `"t"` や `"nil"` の文字にして出す |
| 正規表現 | rs-editor は Rust の正規表現。試験は、両方で同じ結果になる式だけ使う |
| メジャーモード | `major-mode` の名前が違う (`emacs-lisp-mode` と `elisp`)。試験しない |
| `Error: ...` の行 | Emacs 30 が標準出力に出す、エラーの 1 行とバックトレース。比べる前に除く |
| 標準エラー出力 | 既定では比べない (`-e` で比べる) |
| メッセージの言語 | `LC_ALL=C` で動かす (Emacs はロケールでエラーの文を訳すため) |
| `yank-pop` (前が `yank` でないとき) | Emacs はプロンプト「Yank from kill-ring: 」を標準出力に出して読む。rs-editor のバッチモードは、プロンプトを標準出力に出さない (`read-string` などすべて同じ)。入力を標準入力から読むところは同じ |
| エラーの種類 | Emacs Lisp は型付きのエラー (`(car e)` が `end-of-file` など)。Scheme の `condition-case` は型を持たず, `error-message-string` で文を比べる |
| `while` | Scheme に `while` が無いため、繰り返しの試験 (29 番) は `.scm` を手で書く |
| `isearch` | 対話のコマンド (キーを読む) なので、バッチモードでは動かせない。同じ検索の関数 (38 番) で比べる |
| dired の操作 | 印をつける、消す、名前を変えるなどは、キーを読むコマンド。一覧のバッファの中身だけ比べる (日付や大きさは違うので比べない) |
| ロックファイル | Emacs は `create-lockfiles` を `nil` にして動かす。rs-editor はロックファイルを作らない |

## 6. 試験の足し方

1. `tests/batch/cases/NN-name.el` と `NN-name.scm` を、同じ出力になるように書く。20 番より後ろは、`.el` を書いて `tests/batch/tools/gen.py NN-name` で `.scm` を作る (`nil` と `t`、`setq`、`progn`、文字の記法 `?a` を変える)。Scheme に無い書き方 (`while` など) は、`.scm` を手で直し、先頭に `;;; hand-written` と書くと、作り直さない。
2. 出力は `(princ (format "...\n" ...))` で出す。真偽は文字にする。エラーは `condition-case` で捕まえ、メッセージを出す。
3. ファイルの中身は、動かした後のディレクトリの比較で見る。わざわざ出さなくてよい。
4. この文書の 4 章の表に、行を足す。
