# 結合試験仕様書

rs-editor の画面 (Tauri のアプリ) を、人が実際に操作して確かめる試験の仕様です。
単体試験 (`npm run test`、`cargo test --workspace`) では、画面と Lisp (`lisp/*.scm`) をつないだ動きを確かめられないため、この試験で補います。

## 1. 目的と範囲

- 目的は、TypeScript のコマンドを Lisp に移した機能 (Phase 16、17) が、実際のアプリで動くことを確かめること。
- 範囲は、起動、バッファ、ヘルプ、`M-x grep`、`list-buffers`、dired、ブックマーク、ミニバッファの補完、`*Messages*` と `*Warnings*`。
- 範囲外は、字下げ、検索、矩形など、Emacs の結果と自動で比べている機能 (`tests/frontend/lib/`、`core/tests/` が確かめる)。

## 2. 試験の環境

| 項目 | 内容 |
| --- | --- |
| 対象 | `npm run tauri:dev` で起動したアプリ、または `npm run tauri:build` で作ったアプリ |
| OS | Linux (Ubuntu など)。macOS と Windows では、キーの違い (Meta キー) だけ確かめる |
| 初期設定ファイル | `~/.rseditor` (試験の前に、読み込みの失敗が無いことを確かめる) |
| ログ | `~/.local/share/rseditor/logs/rseditor.log` |
| 試験用のファイル | `~/work/a.txt` (中身は `note`)、`~/work/sub/` (空のディレクトリ)、`~/work/src/` (`.rs` を含む) |

試験の前に、次を用意する。

1. `mkdir -p ~/work/sub ~/work/src`
2. `printf 'note\n' > ~/work/a.txt`
3. `printf 'fn main() {}\nfn helper() {}\n' > ~/work/src/main.rs`
4. アプリを起動し直して、前の試験の状態 (バッファ、ブックマーク) を消す。ブックマークは `~/.rseditor.d/bookmarks` を消す。

キーは Emacs の書き方で書く (`C-x` は Ctrl を押しながら x、`M-x` は Alt を押しながら x)。
「RET」は Enter。

## 3. 判定の基準

- 「期待する結果」の通りなら、合格。
- 画面下に `Error:` が出たら、期待する結果に書いていない限り、不合格。
- 不合格のときは、番号、操作、実際の画面の文、ログの該当行を、結果の表に書く。

## 4. 試験項目

### 4.1 起動と初期設定

| 番号 | 事前の状態 | 操作 | 期待する結果 |
| --- | --- | --- | --- |
| IT-001 | 初期設定ファイルが正しい | アプリを起動する | `*scratch*` が出る。モードラインは `-:--- *scratch* [1/2]` で、`Lisp Interaction` と出る |
| IT-002 | 同上 | `C-x C-b` | 一覧に `*scratch*` と `*Messages*` が出る |
| IT-003 | 初期設定ファイルに、存在しないファイルの `load` を書く | アプリを起動する | 画面下に `Error:` が出る。`C-x b` `*Warnings*` RET で、`Error (initialization): An error occurred while loading the init file: …` が出る |
| IT-004 | `~/.rseditor` が無い | アプリを起動する | エラーなく起動する。`*Warnings*` のバッファは作られない (`C-x C-b` の一覧に無い) |
| IT-005 | 同上 | 初期設定ファイルに `(global-set-key "C-c h" 'hello)` と `hello` の `define-command` を書き、起動して `C-c h` | `hello` が動く |

### 4.2 バッファの切り替えと削除

| 番号 | 事前の状態 | 操作 | 期待する結果 |
| --- | --- | --- | --- |
| IT-010 | 起動直後 | `C-x C-f` `~/work/a.txt` RET | `note` が開く。モードラインは `-:--- a.txt [3/3]` |
| IT-011 | IT-010 の後 | `C-x b` `*scratch*` RET | `*scratch*` に戻る。モードラインは `[1/3]` |
| IT-012 | IT-010 の後 | `C-x b` `nothing` RET | 空の `nothing` バッファができる |
| IT-013 | `a.txt` を開いた (変更なし) | `C-x k` | `Kill buffer (default a.txt):` と聞く。RET で `a.txt` が消える (確認は出ない) |
| IT-014 | `a.txt` を開き、1 文字足した | `C-x k` RET | `Buffer a.txt modified; kill anyway? (yes or no)` と聞く |
| IT-015 | IT-014 の続き | `no` RET | 消さない。`a.txt` のまま。`yes` RET なら消える |
| IT-016 | `*scratch*` に文字を足した | `C-x k` RET | 確認なしで消え、別のバッファ (`*Messages*`) が出る |
| IT-017 | `C-x k` で `*scratch*` と `*Messages*` を全て消す | `C-x C-b` | `*scratch*` が作り直されている |
| IT-018 | 2 つ以上のバッファがある | `C-x k` で、名前に `*scratch*` と入力して RET | 指定したバッファが消える |

### 4.3 終了

| 番号 | 事前の状態 | 操作 | 期待する結果 |
| --- | --- | --- | --- |
| IT-020 | `*scratch*` に文字を足しただけ | `C-x C-c` | 確認なしで終了する |
| IT-021 | `a.txt` を開いて 1 文字足した | `C-x C-c` | `Modified buffers exist; exit anyway? (yes or no)` と聞く |
| IT-022 | IT-021 の続き | `no` RET | 終了しない。`yes` RET なら終了する |
| IT-023 | `a.txt` を開いて 1 文字足した | 窓の × ボタン | IT-021 と同じ確認が出る |
| IT-024 | `a.txt` を開いただけ (変更なし) | `C-x C-c` | 確認なしで終了する |

### 4.4 `*Messages*` と `*Warnings*`

| 番号 | 事前の状態 | 操作 | 期待する結果 |
| --- | --- | --- | --- |
| IT-030 | 起動直後 | `M-x` `bogus` RET、`C-x b` `*Messages*` RET | `Undefined command: bogus` が 1 行ある。モードラインは `-:%%- *Messages*` |
| IT-031 | IT-030 の後 | 文字を打つ | 入らない (読み取り専用) |
| IT-032 | `C-x 3` で窓を分け、右の窓に `*Messages*` を出し、左の窓に戻る | `M-x` `bogus2` RET | 右の窓の `*Messages*` に、すぐ `Undefined command: bogus2` が足される |
| IT-033 | `a.txt` を開いて変更した | `C-x C-s` | 画面下に `Wrote …`。`*Messages*` にも同じ行が足される |
| IT-034 | `*Messages*` を `C-x k` で消した | `M-x` `bogus` RET | `*Messages*` が作り直され、その行だけがある |
| IT-035 | 起動直後 | `M-:` `(display-warning 'test "hello" 'error)` RET | 別の窓に `*Warnings*` が出て、`Error (test): hello` がある。`C-x o` で移れる |
| IT-036 | IT-035 の後 | `M-:` `(display-warning 'test "quiet" 'debug)` RET | 窓は増えず、`*Warnings*` に `Debug (test): quiet` が足される |
| IT-037 | 起動直後 | `C-h e` | 別の窓に `*Messages*` が出る |

### 4.5 ヘルプ

| 番号 | 事前の状態 | 操作 | 期待する結果 |
| --- | --- | --- | --- |
| IT-040 | 起動直後 | `C-h k` `C-x C-f` | `*Help*` に `C-x C-f runs the command find-file` と、docstring が出る |
| IT-041 | 同上 | `C-h k` `C-f` | `C-f runs the command forward-char` と出る。docstring は、まだ `Not documented.` (Phase 17 で直す) |
| IT-043 | 同上 | `C-h b` | `*Help*` にキーの割り当ての一覧が出る。`C-h b` と、Lisp の `global-set-key` で足したキーを含む |
| IT-044 | 同上 | `C-h l` | 最近押した 300 キーまで出る。直前に押した `C-h` と `l` も含む |
| IT-045 | 同上 | `C-h f` `find-file` RET | 定義したファイル (`files.scm`)、キー (`C-x C-f`)、引数の形、docstring が出る |
| IT-046 | 同上 | `C-h f` `kill-buffer` RET | `kill-buffer is an interactive Lisp function in 'window.scm'.` と、`C-x k` が出る |
| IT-047 | 同上 | `C-h v` `help-language` RET | 変数の値と docstring が出る |
| IT-048 | 同上 | `C-h w` `find-file` RET | `find-file can be invoked by C-x C-f.` と出る |
| IT-049 | 同上 | `C-h ?` | ヘルプの一覧 (`You have typed C-h, the help character.`) が出る。`q` で閉じる |
| IT-050 | 同上 | `C-h c` `C-x C-f` | 画面下に `C-x C-f runs the command find-file` と出る |
| IT-051 | 同上 | `C-h x` `grep` RET | `grep` の docstring が出る |
| IT-052 | 同上 | `C-h s` | 構文表の説明が出る |
| IT-053 | `*Help*` が出ている | `q` | `*Help*` の窓が閉じる |

### 4.6 `M-x grep` と `next-error`

| 番号 | 事前の状態 | 操作 | 期待する結果 |
| --- | --- | --- | --- |
| IT-060 | `~/work/src/main.rs` がある | `M-x` `grep` RET | `In directory:` と聞く。既定は `~/` |
| IT-061 | IT-060 の続き | `~/w` と打って Tab | `~/work/` に補完される (`~/` で始まる名前の補完) |
| IT-062 | IT-061 の続き | `~/work/s` Tab | `~/work/s` から `~/work/sub/`、`~/work/src/` の候補になる |
| IT-063 | IT-060 の続き | `~/work/src/` RET、パターンに `fn` RET | `*grep*` が出て、`main.rs:1:fn main() {}` と `main.rs:2:fn helper() {}` が出る |
| IT-064 | IT-063 の後 | `*grep*` で 1 行目に移って RET | `main.rs` が開き、1 行目にカーソルがある |
| IT-065 | IT-063 の後 | `M-g n`、`M-g p` | 次と前の一致へ移る。最後の次は `Moved past last grep hit`、最初の前は `Moved back before first grep hit` と出る |
| IT-066 | 一致が無いパターンで `grep` | 実行 | `*grep*` に `no matches` が出る |
| IT-067 | 続けて `grep` を 2 回、3 回と実行する | 実行 | 毎回、`*grep*` が新しい結果で作り直される (2 回目以降も出る) |
| IT-068 | ディレクトリに `~/` を指定する (ホーム全体) | `grep` | `node_modules`、`target`、`.` で始まる名前のディレクトリは探さない。数秒以内に終わる |
| IT-069 | 起動直後 | `M-g n` | `No buffers contain error message locations` と出る (エラーで止まらない) |

### 4.7 `list-buffers` (`C-x C-b`)

| 番号 | 事前の状態 | 操作 | 期待する結果 |
| --- | --- | --- | --- |
| IT-070 | `a.txt` を開いて変更した | `C-x C-b` | `*Buffer List*` に、`CRM Buffer Size Mode File` の見出しと、各バッファの行が出る。`a.txt` の行に `*` が付く |
| IT-071 | IT-070 の続き | `n`、`p` | 次の行、前の行に移る |
| IT-072 | IT-070 の続き | 行を選んで RET (または `f`) | その行のバッファが出る |
| IT-073 | IT-070 の続き | `o` | その行のバッファを、別の窓に出す (窓の操作が足りないときは、`TODO.md` の通り) |
| IT-074 | IT-070 の続き | `q` | `*Buffer List*` の窓が閉じる |
| IT-075 | `*Messages*` がある | `C-x C-b` | `*Messages*` の行の 2 列目に、読み取り専用の印 (`%`) が出ることを確かめる (出なければ、`TODO.md` の Phase 16 の項目の通り) |

### 4.8 dired (`C-x d`)

| 番号 | 事前の状態 | 操作 | 期待する結果 |
| --- | --- | --- | --- |
| IT-080 | `~/work` がある | `C-x d` `~/work` RET | `ls -al` 形式の一覧が出る。モードラインは `Dired by name` |
| IT-081 | IT-080 の続き | `M-x` `revert-buffer` RET | 一覧が作り直される |
| IT-082 | IT-080 の続き | `C-x C-f` | ミニバッファの既定が `~/work/` になる (dired のディレクトリ)。`a.txt` RET で開く |
| IT-083 | IT-080 の続き | `C-x d` `~/work/sub` RET | `~/work/sub` の dired が別のバッファで開く。`C-x d` `~/work` RET で、元の dired に戻る |
| IT-084 | 同上 | `C-x C-b` | 一覧の `Mode` の列に `Dired by name` が出る |
| IT-085 | `a.txt` を開いている | `M-x` `dired-jump` RET | `a.txt` のあるディレクトリの dired が開く |

### 4.9 ブックマーク

| 番号 | 事前の状態 | 操作 | 期待する結果 |
| --- | --- | --- | --- |
| IT-090 | ファイルを開いていない (`*scratch*`) | `C-x r m` | `Error: Buffer not visiting a file` が出る |
| IT-091 | `a.txt` を開いて、カーソルを動かした | `C-x r m` RET | ブックマークの名前を聞く。既定は `a.txt`。RET で登録され、`~/.rseditor.d/bookmarks` ができる |
| IT-092 | IT-091 の後 | 別のバッファに移り、`C-x r b` `a.txt` RET | `a.txt` が開き、カーソルが登録した位置にある |
| IT-093 | IT-091 の後 | `M-x` `bookmark-bmenu-list` RET | `*Bookmark List*` に、`a.txt` の行が出る |
| IT-094 | ブックマークが 1 つも無い | `C-x r b` | `Error: No bookmarks` が出る |
| IT-095 | IT-091 の後、アプリを終了して起動し直す | `C-x r b` `a.txt` RET | 前の起動で登録したブックマークで、`a.txt` が開く |

### 4.10 ミニバッファの補完

| 番号 | 事前の状態 | 操作 | 期待する結果 |
| --- | --- | --- | --- |
| IT-100 | 起動直後 | `M-x` `find-fi` Tab | `find-file` に補完される |
| IT-101 | 同上 | `C-x C-f` `~/w` Tab | `~/work/` に補完される |
| IT-102 | 同上 | `C-x C-f` `/tm` Tab | `/tmp/` に補完される |
| IT-103 | 同上 | `C-x b` `*S` Tab | `*scratch*` に補完される |
| IT-104 | ミニバッファで入力中 | `C-g` | 入力をやめ、`Quit` と出る |

### 4.11 読み取り専用

| 番号 | 事前の状態 | 操作 | 期待する結果 |
| --- | --- | --- | --- |
| IT-110 | 起動直後 | `C-x b` `*Messages*` RET、文字を打つ | 入らない。モードラインの状態は `%%` |
| IT-111 | IT-110 の続き | `M-:` `(set-buffer-read-only #f)` RET、文字を打つ | 打てる。モードラインの状態は `%*` |
| IT-112 | IT-111 の続き | `M-:` `(set-buffer-read-only #t)` RET | また打てなくなる |
| IT-113 | `C-h f` で `*Help*` を出した | `*Help*` で文字を打つ | 入らない |

### 4.12 Lisp の実行

| 番号 | 事前の状態 | 操作 | 期待する結果 |
| --- | --- | --- | --- |
| IT-120 | `*scratch*` | `(+ 1 2)` と書いて `C-x C-e` | 画面下に `3` が出る。`*Messages*` にも `3` が足される |
| IT-121 | 同上 | `M-:` `(car 1)` RET | `Error:` で始まる文が出る。アプリは動き続ける |
| IT-122 | 同上 | `M-:` `(define-command (hi) (message "hi"))` RET、`M-x` `hi` RET | `hi` が出る |
| IT-123 | `*scratch*` | `(read-string "Name: ")` を `C-x C-e` で評価し、答えを入力 | ミニバッファで聞き、答えが返る。`C-g` で `Quit` |

## 5. 結果の記録

試験のたびに、次の表を `docs/integration_test_results/` に、日付 (`2026-11-02.md` など) の名前で作る。

| 番号 | 結果 (合格 / 不合格 / 対象外) | 備考 (不合格のときは、実際の画面の文とログの該当行) |
| --- | --- | --- |
| IT-001 | | |

## 6. 不合格のときの扱い

1. 不合格の番号と再現の手順を、`TODO.md` か Issue に書く。
2. 直したら、同じ番号をもう一度試し、合格を確かめる。
3. 単体試験 (`tests/frontend/`、`core/tests/`) で確かめられるものは、その試験を足す。足したら、この仕様書の該当の番号に、試験の名前を書く。

## 7. 既知の違い (不合格にしない)

- `C-h f`、`C-h k` で、画面の TypeScript のコマンド (`C-f` の `forward-char` など) は、docstring が `Not documented.` になる (`TODO.md` の Phase 17)。
- 読み取り専用のバッファでも、画面のコマンド (`C-k` など) は書き換えられる (`TODO.md` の Phase 16)。
- `kill-buffer` を Lisp から名前付きで呼ぶと、聞かずに消す (書き換えたファイルのバッファならエラー)。
- 古い JSON 形式のブックマークは、読み込まない。
- dired には、`RET`、`n`、`p`、`^`、`g` などの専用のキーがまだ無い (一覧と `dired-jump` だけ)。
