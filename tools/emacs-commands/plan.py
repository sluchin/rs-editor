"""「エディタの基本のコマンド」を, Lisp (`lisp/`) で実装する計画. generate.py が読み, docs/functions.md に書く.

段階は番号で分ける. 前の段階を終えてから, 次の段階に進む (1 は土台で, 2 以降の全ての前提).
コマンドは, ライブラリごとに段階を決め, 例外だけ COMMAND_STEPS に書く. 段階が決まっていないコマンドが
あれば, generate.py が失敗する (Emacs が増やしたコマンドに, 段階を付け忘れないため).
"""

# 段階 (番号, 題, 内容). 内容は, 箇条書きの文.
STEPS = [
    (1, '土台: Rust のエディタの核 (ターミナルの ncurses でも動かすため)',
     ['目標は, 同じエディタを, 今の画面 (Tauri) と, ターミナル (ncurses) の両方で動かすこと。そのため, バッファ, カーソル, マーク, undo, '
      'kill-ring, 窓を, 画面 (TypeScript) でなく Rust の核に持つ。画面は, 核の状態を描いて, キーを核に渡すだけにする。'
      'コマンドは入れない (Emacs の C の関数に当たるものを作る)。',
      '【済】1.1 クレートを分ける: 画面に頼らない核のクレート (`core/`. 評価器 `scheme` も移す) と, それを使うアプリ (`src-tauri/`), ターミナルのアプリ (`tui/`)。'
      'Cargo のワークスペースにする。テストのカバレッジ 100% は, 核にも保つ。',
      '【Rust の部品は済. Lisp につなぐのは 1.9】1.2 バッファ (`core/src/buffer.rs`): 本文 (rope), カーソル, 変更の有無, ファイルの名前。`point`, `goto-char`, `insert`, `delete-region`, `char-after`, `char-before`, '
      '`forward-char`, `backward-char`, `forward-line`, `line-beginning-position`, `line-end-position`, `bobp`, `eobp`, `bolp`, `eolp`, `buffer-size`, '
      '`current-column`, `move-to-column`。位置は文字で数える (今の Lisp と同じ)。',
      '【Rust の部品は済. Lisp につなぐのは 1.9】1.3 マークとリージョン (`Buffer`): `mark`, `set-mark`, `push-mark`, `pop-mark`, `region-beginning`, `region-end`, `use-region-p`。',
      '【Rust の部品は済. Lisp につなぐのは 1.9】1.4 元に戻す (`Buffer`): 変更の履歴 (`undo-boundary`, `primitive-undo`)。今の CodeMirror の履歴と同じ動き (まとめ方, 重ねた undo) にする。',
      '【Rust の部品は済. Lisp につなぐのは 1.9】1.5 kill-ring (`core/src/kill_ring.rs`): `kill-region`, `current-kill`, `kill-append`, `insert-for-yank`。',
      '【Rust の部品は済 (`core/src/search.rs`, `core/src/syntax.rs`). Lisp につなぐのは 1.9】1.6 探す: `search-forward`, `search-backward`, `re-search-forward`, `re-search-backward`, `looking-at`, `match-beginning`, `match-end` '
      '(Emacs の正規表現の書き方に合わせる)。構文: `scan-lists`, `scan-sexps`, `syntax-ppss`, `char-syntax` (`src/lib/syntax.ts` を Rust に移した)。どちらも `emacs -Q --batch` の結果と全ての位置で比べる (`tools/syntax/run.sh`)。',
      '【Rust の部品は済 (`core/src/window.rs`, `core/src/editor.rs`). Lisp につなぐのは 1.9】1.7 窓とバッファの一覧: `selected-window`, `split-window`, `delete-window`, `window-list`, `set-window-buffer`, `set-buffer`, `get-buffer`, '
      '`generate-new-buffer`, `with-current-buffer`, バッファごとの変数 (`make-local-variable`)。',
      '【前置引数の部品は済 (`Editor::prefix_arg`, `prefix_numeric_value`). `interactive` は 1.9】1.8 前置引数: `current-prefix-arg`, `prefix-numeric-value`。`define-command` に, 引数の読み方 (Emacs の `interactive`) を足す。',
      '【Rust は済 (`core/src/scheme/editor_host.rs`: `Editor` を直接動かす `EditorHost`. 63 個の組み込み). 今の画面 (Tauri) をつなぐのは 1.11】1.9 評価器を核につなぐ: 今の `Host` (画面に頼んで返事を待つ) をやめ, 評価器が核を直接動かす。画面に頼むのは, 描くこと, ミニバッファの入力, '
      'ファイルの選択だけにする (`Host` を小さくする)。',
      '【済 (`tui/`. 動いている: 移動, 編集, kill, yank, undo, 窓の分割, ファイルを開く, 保存, M-x, 終了。基本のコマンドは `lisp/simple.scm`)】1.10 ターミナルのアプリの骨組み (`tui/`. ncurses): 核の状態を描き, キーを核に渡す。カーソルを動かし, 文字を入れ, 保存できるところまで。',
      '【段階 2 から 11 の後】1.11 今の画面 (Tauri) を核につなぎ替える: 本文の持ち主を CodeMirror から核に移す。CodeMirror は表示と入力にする。'
      '今の画面のコマンド (`src/lib/emacs.ts` など. 約 85 個) は, CodeMirror の状態を直接書き換えているので, 段階 2 から 11 で Lisp に移し終えるまでは, 持ち主を移せない。'
      '移す前の動きは, `tests/frontend/` のテストで保つ。',
      '【つないだ (`core/src/scheme/mirror.rs`. 環境変数 `RSEDITOR_NO_CORE` で、これまでの動きに戻せる)】1.11 の前につなぐ案 (Tauri の画面を, 先に核の組み込みで動かす): 評価のあいだだけ, 画面のバッファを核に写し (スナップショット), 終わったら変更を画面に返す '
      '(`EditorHost` を `Bridge` の上で動かし, 本文の操作は核で, バッファの出し入れとウィンドウは画面に頼む). 探す, 構文, マークなどの組み込みが, 今の画面の Lisp (howm, org) でも使える。'
      '画面の往復が減る代わりに, 画面と核の本文の同期が要る。実際の画面での確かめが要るので, 段階 2 以降の Lisp が画面で要るようになったときに行う。']),
    (2, '動き',
     ['【Lisp 版は済 (`lisp/simple.scm`). `emacs -Q --batch` の結果と, 本文の全ての位置で一致 (`core/tests/test_motion.rs`. 392 通り). '
      '画面 (Tauri) の TypeScript 版を消すのは, 画面をつないだ後 (段階 1.11 の前の案)】'
      '行, 単語, バッファの端, 左右の移動。核の上の Lisp で書く。カーソルを動かすので, 速さを比べてから Lisp に移す。']),
    (3, '消す, 切る, 貼る, 元に戻す',
     ['【Lisp 版は済 (`lisp/simple.scm`). `emacs -Q --batch` の結果と一致 (`core/tests/test_edit.rs`: 57 通りのコマンドを 7 種類の本文の全ての位置から, '
      '`core/tests/test_undo.rs`: undo と undo-redo の 10 通りの場面). 画面 (Tauri) の TypeScript 版を消すのは, 画面をつないだ後】'
      '違いが残るのは, `default-indent-new-line` (字下げは段階 6), `quoted-insert` (1 文字を表すキーだけ), `cycle-spacing` (最初の呼び出しだけを Emacs と比べた), '
      '`undo` が保存した状態に戻っても「変更なし」にならないこと。'
      'kill-ring を使うコマンドと, 空白と改行を整えるコマンド, 入れ替え, `undo`。']),
    (4, 'マーク, 前置引数, 実行',
     ['【Lisp 版とターミナルのアプリは済 (`lisp/simple.scm`, `tui/`). マークは `emacs -Q --batch` の結果と一致 (`core/tests/test_edit.rs`). '
      '`next-error` と `previous-error` は, `*grep*` の画面 (TypeScript) のままで, 段階 9 で移す. 画面 (Tauri) の TypeScript 版を消すのは, 画面をつないだ後】'
      'マークとリージョン, 前置引数 (`C-u`, `M-数字`), `M-x`, `M-X`, `M-:`, `C-x M-:`, `C-g`, 表示の切り替え (`truncate-lines`, `selective-display`)。']),
    (5, 'S 式, 段落, ページ',
     ['【式, defun, 段落, 文は済 (`lisp/simple.scm`, 計算は `core/src/sexp.rs` と `core/src/paragraph.rs`). `emacs -Q --batch` の結果と一致 '
      '(`core/tests/test_lisp.rs`: 3 つのモードで, 50 通りのコマンドを全ての位置から, 対話的に). ページ (`forward-page`, `mark-page`) と入れ替え (`transpose-sexps`, `transpose-sentences`, `transpose-paragraphs`) も済 (`transpose-sexps` は, バックスラッシュで quote した文字と `##` `,@` の前後だけ Emacs と違う)。'
      '`insert-parentheses`、`move-past-close-and-reindent`、`count-lines-page` も済 (`move-past-close-and-reindent` は、scheme-mode の `{}` と `|sym (|` を含む本文の字下げだけ Emacs と違う)。'
      '絞り込みは、核に作った (`Buffer::narrow_to_region`、`widen`、`save-restriction`、`buffer-narrowed-p`。点、移動、検索、構文の走査、`buffer-string`、`erase-buffer`、窓の描画が、見える範囲だけを扱う。`goto-line` は、絞り込みの外の行なら絞り込みをやめる)。'
      '`narrow-to-region`、`widen`、`narrow-to-page`、`narrow-to-defun` は、Emacs と一致 (`core/tests/test_lisp.rs`、`core/tests/test_narrow.rs`)。'
      '違いが残るのは、段落、文、字下げ、整形の計算 (`forward-paragraph` など) が、絞り込みを見ずに本文の全体を読むこと、画面 (Tauri) がまだ絞り込めないこと。'
      '画面 (Tauri) の TypeScript 版 (`src/lib/sexp.ts`, `src/lib/paragraph.ts`) を消すのは, 画面をつないだ後】'
      '括弧の移動と消去 (`C-M-f` など), defun, 文と段落, ページ。構文表 (段階 1.6) を使う。']),
    (6, '字下げ, 整形, コメント',
     ['【Lisp 版は済 (`lisp/simple.scm`, 計算は字下げが `core/src/indent.rs`, 整形が `core/src/fill.rs`, コメントが `core/src/comment.rs`). '
      '`emacs -Q --batch` の結果と一致 (`core/tests/test_lisp.rs`: `indent-for-tab-command`, `indent-region`, `indent-sexp`, `newline`, `indent-rigidly`, `tab-to-tab-stop`, '
      '`set-fill-prefix`, `fill-paragraph`, `comment-dwim`, `comment-line` を, 本文の全ての位置から. 日本語の折り (禁則) は Emacs と別の決まりで, `core/tests/test_fill.rs` で試す). '
      '違いが残るのは, Lisp の `fill-paragraph` が文字列と `#| |#` の中の段落を詰めないこと, `comment-dwim` が `#| |#` に対応しないこと, '
      '`comment-dwim` と `comment-line` のリージョンが行の途中で始まるか終わるとき (Emacs は途中から入れる. こちらは行ごと), `indent-rigidly` が前置引数で動かすだけで, 矢印キーの続きの入力は無いこと, '
      '`comment-set-column` が桁を決めるだけで, コメントの置き直しをしないこと。'
      '画面 (Tauri) の TypeScript 版 (`src/lib/lispIndent.ts`, `src/lib/fill.ts`, `src/lib/comment.ts`) を消すのは, 画面をつないだ後】'
      '`TAB`, `C-M-\\`, `M-q`, `M-;`。字下げの決まりと整形は, ターミナルでも使うため, Rust に移した。'
      'エディタと整形の道具 (TODO の Phase 18) で, 決まりを 1 つにする。']),
    (7, '検索, 置換, 強調',
     ['【Lisp 版は済 (`lisp/isearch.scm`, `lisp/replace.scm`. ターミナルのアプリが読む). `emacs -Q --batch` の結果と一致: '
      '`replace-string`, `replace-regexp`, `query-replace`, `query-replace-regexp` (`core/tests/test_replace.rs`), `occur` (`core/tests/test_occur.rs`), '
      '`isearch-forward` と `isearch-backward` と正規表現の版 (`core/tests/test_isearch.rs`: Emacs のコマンドを, キーの順に呼んだ結果と, 全ての位置から比べる)、`isearch-forward-thing-at-point` と `isearch-forward-symbol-at-point` (カーソルの所の物を初めの文字列にする。物は `region`、`symbol`、`sexp` で、`url` は無い。変数 `isearch-forward-thing-at-point` は、同じ名前の命令があるので `isearch-forward-thing-at-point-things`)。'
      '強調 (`highlight-regexp`, `unhighlight-regexp`, `highlight-phrase`, `highlight-symbol-at-point`, `highlight-lines-matching-regexp`) は, 核が正規表現を持ち (`Buffer::highlights`), '
      'ターミナルは下線で描く (`core/tests/test_highlight.rs`). 違いが残るのは, 記号の検索 (`isearch-forward-symbol`) で空白や正規表現の特別な文字を打つこと, 空白を打つと, 続く空白の全てに合う (lax whitespace) 動きが無いこと, isearch が, 別のキーで終えたときに, そのキーのコマンドを動かさないこと, '
      '`\\#` `\\,` `\\?` の置き換えと, 置き換えの取り消し (`u`, `^`) と, 編集 (`e`) が無いこと, `occur` が *Occur* の色と, 複数のバッファを扱わないこと, '
      '強調の色の名前を聞かないこと。画面 (Tauri) の TypeScript 版 (`src/hooks/useSearch.ts`) を消すのは, 画面をつないだ後】'
      'isearch, `query-replace`, `occur`, `highlight-regexp`。検索の状態を核に持つ (描き方は, 画面ごと)。']),
    (8, '矩形, レジスタ, キーボードマクロ, 略語',
     ['矩形 (`C-x r`), レジスタ, キーボードマクロ (`F3`, `F4`, `C-x (`), 略語と `M-/`。',
      '【矩形とレジスタとキーボードマクロは Lisp 版が済 (`lisp/rect.scm`, `lisp/register.scm`, `lisp/simple.scm`). '
      '`emacs -Q --batch` の結果と一致: 矩形 (`clear-rectangle`, `delete-rectangle`, `open-rectangle`, `rectangle-number-lines`, `kill-rectangle`, `copy-rectangle-as-kill` と `yank-rectangle`) と, '
      'レジスタ (`copy-rectangle-to-register`, `number-to-register`, `increment-register`, `append-to-register`, `point-to-register`, `jump-to-register`) (`core/tests/test_lisp.rs`, `core/tests/test_edit.rs`). '
      'ウィンドウ構成のレジスタ (`window-configuration-to-register`) は, バッチの Emacs に窓が無いので Rust のテストだけ (`core/tests/test_register.rs`, `core/tests/test_editor.rs`). '
      'キーボードマクロの `kmacro-*` (カウンター, `F3`, `F4`, 名前を付ける) も, 同じ理由で `tui/tests/test_app.rs` で試す. '
      '違いが残るのは, 幅の違う文字 (全角, 曖昧幅) を含む行の矩形の桁の数え方, '
      '`kmacro-end-call-mouse` (マウス) と, キーボードマクロのリング (`kmacro-cycle-ring-next` など) が無いこと。'
      '略語 (`lisp/abbrev.scm`: `expand-abbrev`, `abbrev-prefix-mark`, `add-global-abbrev`, `add-mode-abbrev`, `inverse-add-global-abbrev`, `inverse-add-mode-abbrev`) も済で, '
      '`emacs -Q --batch` の結果と一致 (`core/tests/test_edit.rs`: 大文字と小文字の展開, 前置引数, 取り消し, 作り直しの問い, 接頭辞の印を, 本文の全ての位置から). '
      '違いが残るのは, 略語の表がメジャーモードごとではなく, 全てのバッファで 1 つのローカルの表と, グローバルの表だけなこと, '
      '`abbrev-mode` (打つと自動で展開する) と, 略語の保存 (`write-abbrev-file`) と, 表の親 (`:parents`), 展開のフックの `no-self-insert` が無いこと。'
      '`dabbrev-completion` (`lisp/dabbrev.scm`) と, 補完の土台 (`completion-at-point`, `lisp/minibuffer.scm`) も済で, `dabbrev-completion` は `emacs -Q --batch` の結果と一致 (`core/tests/test_edit.rs`: 本文の全ての位置から, 補完した本文と, カーソルと, 出したメッセージ). '
      '`complete-symbol` は, 補完する名前が Emacs と別 (Emacs は Emacs Lisp の記号, こちらは評価器に定義された Scheme の名前) なので, `core/tests/test_minibuffer.rs` で試す。'
      '補完の違いは, 候補の選び方が先頭の一致だけ (partial-completion などの方式が無い), `*Completions*` が 1 行ずつの簡単な一覧 (列に並べず, 選んで確定もできない), '
      '`dabbrev-completion` が他のバッファを探さない (前置引数を付けても今のバッファだけ) こと。'
      '`dabbrev-expand` の違いは, 展開の後に空白を入れて続けると次の語を取る機能と, 他のバッファを探す機能が無いこと。'
      '略語の違いを無くすには, (1) メジャーモードごとの略語の表と `abbrev-mode`: エディタにメジャーモードの仕組み (バッファごとのモードと, 変数 `local-abbrev-table`) と, 文字を打つたびに略語を展開するか決める処理 (Emacs の `self-insert-command` の後の `abbrev-mode` の判定) を足す必要があり, エディタ側の変更が大きい。 '
      '(2) 略語の保存 (`write-abbrev-file`, `read-abbrev-file`) と表の親 (`:parents`), 展開のフックの `no-self-insert` は, Lisp だけで書ける。】']),
    (9, 'ファイル, バッファ, dired, ブックマーク, シェル',
     ['ファイルとバッファの残りのコマンド, `list-buffers`, dired, ブックマーク, `M-!`, `M-|`, `M-&`, バッファの複製。',
      '`save-buffers-kill-terminal` は, アプリを終える頼みを `Host` に足してから移す。',
      '【シェルとバッファの命令は Lisp 版が済 (`lisp/simple.scm`, `lisp/files.scm`). `emacs -Q --batch` の結果と一致: '
      '`shell-command` と `shell-command-on-region` (出力のバッファ, 今のバッファに入れる, 領域の置き換え, エコー領域か別のバッファか, 終了コードのメッセージ), '
      '`clone-buffer` (本文, 点, マーク, 絞り込み), `not-modified`, 読み取り専用のバッファでの編集の失敗 (`insert`, `delete-char`, `kill-word`, `upcase-region` など) (`core/tests/test_edit.rs`). '
      '`async-shell-command`, `rename-buffer`, `rename-uniquely`, `read-only-mode`, `find-file-read-only`, `revert-buffer-quick`, `list-directory`, `clone-indirect-buffer-other-window` は '
      '名前, ファイル, 問いが絡むので Rust で試す (`core/tests/test_simple_buffers.rs`, `core/tests/test_editor.rs`, `core/tests/test_buffer.rs`). '
      '違いが残るのは, 非同期のシェルのコマンド (`&` で終わる命令と `async-shell-command`) を終わりまで動かしてから次を読むこと (プロセスのバッファ, 途中の出力, 止めることが無い), '
      '標準エラーを別のバッファに分けられないこと (`error-buffer` は使わず, 出力に混ぜる), 領域が不連続のとき (`region-noncontiguous-p`) と `shell-command-dont-erase-buffer` が無いこと, '
      '`clone-indirect-buffer-other-window` が本文を共有する間接バッファではなく, 本文を写した双子のバッファになること, '
      '`clone-buffer` がメジャーモードとバッファごとの変数を写さないこと (モードが無い), '
      '`read-only-mode` がマイナーモードではなく命令で, 文字ごとの `read-only` の属性と `inhibit-read-only` と `view-mode` が無いこと, '
      '`list-directory` の一覧が `ls` の出力のままで (`ls -CF` の列の幅は端末の幅で決まる), `*Directory*` が help-mode でなく, 取り消しの記録を止めないこと, '
      '`revert-buffer-quick` が自動保存のファイルからの読み直し (前置引数) と `revert-buffer-quick-short-answers` に対応しないこと, '
      '`find-file-read-only` がワイルドカードを展開しないことと, `rename-buffer` が空白で始まる名前に乱数を付けないこと。'
      '画面 (Tauri) の TypeScript 版を消すのは, 画面をつないだ後】']),
    (10, 'ウィンドウ',
     ['窓の分割, 削除, 移動, 大きさ, スクロール, 別の窓に出すコマンド。窓の木 (`src/lib/windows.ts`) は, 段階 1.7 で核に移したものを使う。'
      '【Lisp 版とターミナルのアプリは済 (`lisp/window.scm`, `lisp/simple.scm`, `core/src/window.rs`, `tui/`). '
      'バッチの Emacs には窓が 1 つしか無いので, `emacs -Q --batch` との比べは無く, Rust のテストで試す (`core/tests/test_window_commands.rs`, `core/tests/test_window.rs`, `core/tests/test_editor.rs`, `tui/tests/test_app.rs`). '
      '窓の木に分ける割合を持たせ (`enlarge-window`, `shrink-window`, `balance-windows`, `fit-window-to-buffer` などが変える), 画面の大きさを核に知らせて (`Editor::set_screen_size`), 窓の大きさを行と桁で数える。'
      '違いが残るのは, 窓ごとの点 (`window-point`) と先頭の行 (`window-start`) が無く, 点はバッファごとなので, 同じバッファを出した窓では `scroll-other-window` が選んだ窓の点も動かすこと, '
      '`next-buffer` と `previous-buffer` が, 窓ごとの前のバッファの履歴ではなく, バッファの一覧の順に替えること, `quit-window` と `delete-windows-on` がバッファを後ろに回さないこと (`bury-buffer` が無い), '
      '大きさを変えるときに, 隣の窓の最小の高さを守らず, 割合を 5% から 95% に収めるだけなこと, `fit-window-to-buffer` が高さだけを合わせること (幅, `fit-window-to-buffer-horizontally`, 最大の高さの変数が無い), '
      '`enlarge-window` が行の数で言った量にぴったり合わないことがある (割合から丸めるため) こと, '
      '`same-window-prefix`, `other-window-prefix`, `toggle-window-dedicated`, `window-toggle-side-windows` (窓を固定する, 横の窓) が無いこと, '
      '`move-to-window-line-top-bottom` が `scroll-margin` を見ないこと。'
      '画面 (Tauri) の TypeScript 版を消すのは, 画面をつないだ後】']),
    (11, 'ヘルプ, xref, info',
     ['`C-h` のコマンドと, 定義へ移るコマンド (`M-.`), info。ヘルプは Phase 17 (docstring と多言語化) と一緒に進める。']),
    (12, '入れない',
     ['rs-editor に当たる機能が無いので, 入れない。メール, フレーム (複数の窓枠), 日本語入力以外のテキスト変換, Emacs の案内の文書。']),
]

# ライブラリごとの, 既定の段階.
LIBRARY_STEPS = {
    'simple': 3,
    'files': 9,
    'window': 10,
    'bindings': 2,
    'lisp': 5,
    'indent': 6,
    'paragraphs': 5,
    'page': 5,
    'fill': 6,
    'newcomment': 6,
    'replace': 7,
    'isearch': 7,
    'rect': 8,
    'register': 8,
    'kmacro': 8,
    'abbrev': 8,
    'dabbrev': 8,
    'buff-menu': 9,
    'dired': 9,
    'bookmark': 9,
    'xref': 11,
    'loaddefs': 7,
    'help': 11,
    'help-fns': 11,
    'info': 11,
}

# ライブラリの既定と違うコマンドの段階.
COMMAND_STEPS = {
    # simple: 動き
    **dict.fromkeys([
        'back-to-indentation', 'backward-word', 'beginning-of-buffer', 'end-of-buffer', 'move-beginning-of-line',
        'move-end-of-line', 'next-line', 'previous-line', 'goto-line', 'goto-line-relative', 'set-goal-column',
    ], 2),
    # simple: マーク, 前置引数, 実行, 表示
    **dict.fromkeys([
        'set-mark-command', 'exchange-point-and-mark', 'mark-whole-buffer', 'mark-word', 'pop-global-mark',
        'digit-argument', 'negative-argument', 'universal-argument', 'keyboard-quit', 'keyboard-escape-quit',
        'what-cursor-position', 'count-words-region', 'set-fill-column', 'toggle-truncate-lines',
        'set-selective-display', 'repeat-complex-command', 'execute-extended-command',
        'execute-extended-command-for-buffer', 'eval-expression', 'next-error', 'previous-error',
    ], 4),
    # simple: S 式の入れ替え
    'transpose-sexps': 5,
    # simple: シェルとバッファの複製
    **dict.fromkeys([
        'shell-command', 'shell-command-on-region', 'async-shell-command', 'clone-buffer',
        'clone-indirect-buffer-other-window',
    ], 9),
    # simple: メールとテキスト変換
    **dict.fromkeys([
        'compose-mail', 'compose-mail-other-frame', 'compose-mail-other-window', 'analyze-text-conversion',
    ], 12),
    # bindings: 補完
    'complete-symbol': 8,
    # bindings: kill-ring
    'ignore-preserving-kill-region': 3,
    # help: Emacs の案内の文書
    **dict.fromkeys([
        'describe-copying', 'describe-distribution', 'describe-gnu-project', 'describe-no-warranty',
        'view-emacs-FAQ', 'view-emacs-debugging', 'view-emacs-news', 'view-emacs-problems', 'view-emacs-todo',
        'view-external-packages', 'view-order-manuals',
    ], 12),
    # window: フレーム
    'handle-select-window': 12,
}


def step_of(command, library):
    """コマンドの段階 (番号). フレームとタブに出すコマンドは 12. 決まっていなければ None.

    Args:
        command: コマンドの名前.
        library: Emacs でコマンドがあるライブラリ.
    """
    if command in COMMAND_STEPS:
        return COMMAND_STEPS[command]
    if 'frame' in command or command.endswith('-other-tab'):
        return 12
    return LIBRARY_STEPS.get(library)
