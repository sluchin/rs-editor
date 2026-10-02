#### 7.5.4 SRFI-2 - and-let\* [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d2-_002d-and_002dlet_002a)

以下の構文は、

([use-modules](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-use_002dmodules) (srfi srfi-2))

または別の方法として

([use-modules](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-use_002dmodules) (ice-9 and-let-star))

ライブラリ構文: **and-let\*** (句 …) 本体 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-and_002dlet_002a)

`and`と`let*`の組み合わせ。

各節は順番に評価され、`#f` が得られた場合は評価が停止し、`#f` が返されます。すべてが `#f` でない場合は、本体が評価され、最後の形式が戻り値になります。本体が空の場合は、結果は `#t` になります。各節は次のいずれかである必要があります。

`(シンボル式)`

式を評価し、`#f` をチェックして、それをシンボルにバインドします。`let*` と同様に、このバインドは後続の節で使用できます。

`(expr)`

exprを評価し、`#f`をチェックします。

`シンボル`

シンボルにバインドされた値を取得し、`#f` をチェックします。

`(expr)` には、例えば `((eq? xy))` のように「余分な」括弧のペアがあることに注意してください。これを覚える一つの方法は、`(symbol expr)` の `symbol` が省略されていると想像することです。

`and-let*` は、`#f` 値が終了を意味する計算に適していますが、後続の式では `#f` 以外の値が必要になります。

以下はこれを示しています。文字列内の角括弧 '\[...\]' で囲まれたテキストを返します。角括弧がない場合は `#f` を返します (つまり、`string-index` のどちらでも `#f` を返します)。

(define (extract-brackets str)
(and-let\* ((start (string-index str #\\\[))
(end (string-index str #\\\] start)))
(部分文字列 str (1+ 開始) 終了)))

以下では、通常の変数と式もテスト対象として示しています。`diagnostic-levels` は、診断タイプとレベルを関連付ける alist として扱われます。`str` は、タイプが既知であり、かつレベルが十分に高い場合にのみ出力されます。

(define (show-diagnostic type str)
(and-let\* (want-diagnostics
(レベル (assq-ref 診断レベルタイプ))
((>= 現在の診断レベル)))
(文字列を表示)))

`and-let*` の利点は、式とテストの拡張シーケンスが、個別の `and` と `let*`、または `cond` と `=>` から生じるような多くのネストを必要としないことです。

* * *

次へ: [SRFI-6 - 基本文字列ポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d6)、前: [SRFI-2 - and-let\*](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d2)、上: [SRFI サポート モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI-Support) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
