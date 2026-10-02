### 7.10 整形印刷 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Pretty-Printing-1)

モジュール`(ice-9 pretty-print)`は、 Schemeオブジェクトをきれいに整形して出力するプロシージャ`pretty-print`を提供します。これは、リストやベクトルなどの深くネストされたデータ構造や複雑なデータ構造に特に役立ちます。

モジュールは、以下のコマンドを入力することでロードされます。

([use-modules](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-use_002dmodules) (ice-9 [pretty-print](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-pretty_002dprint)))

これにより、`pretty-print` プロシージャが利用可能になります。`pretty-print` がどのように出力をフォーマットするかの例を以下に示します。

([pretty-print](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-pretty_002dprint) '(define (foo) (lambda (x)
(cond (([zero?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-zero_003f) x) #t) (([negative?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-negative_003f) x) \-x) (else
(if ([\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) x 1) 2 ([\*](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002a) xxx)))))))
⊣
(define (foo)
(ラムダ (x)
(cond (([zero?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-zero_003f) x) #t)
(([negative?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-negative_003f) x) \-x)
(else (if ([\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) x 1) 2 ([\*](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002a) xxx))))))

Scheme 手順: **pretty-print** obj \[port\] \[keyword-options\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-pretty_002dprint-1)

Schemeオブジェクトobjのテキスト表現をportに出力します。portが指定されていない場合は、現在の出力ポートがデフォルトで使用されます。

その他のキーワードオプションは、以下のキーワードとパラメータです。

`#:display?`フラグ

flagがtrueの場合は、`display`を使用して出力します。デフォルトは`#f`で、これは`write`スタイルを使用することを意味します。[スキーム値の書き込み](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scheme-Write)を参照してください。

`#:per-line-prefix`文字列

指定された文字列を各行の先頭に付加して表示します。デフォルトでは付加されません。

`#:width`列

指定された列数内に印刷します。デフォルト値は79です。

`#:max-expr-width`列

式の最大幅。デフォルト値は50です。

`(ice-9 pretty-print)`モジュールによってエクスポートされるもう一つの関数は`truncated-print`です。これは、Schemeのデータを出力する際に、出力文字数を一定数に切り詰める処理です。これは、ユーザーに任意のデータを提示する必要があるものの、表示できる行が1行しかない場合に便利です。

(define [exp](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-exp) '(ab #(cde) f . g))
([truncated-print](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-truncated_002dprint) [exp](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-exp) #:width 10) ([newline](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-newline))
⊣ (ab . #)
([truncated-print](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-truncated_002dprint) [exp](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-exp) #:width 15) ([newline](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-newline))
⊣ (ab # f . g)
([truncated-print](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-truncated_002dprint) [exp](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-exp) #:width 18) ([newline](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-newline))
⊣ (ab #(c [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e)) . #)
([truncated-print](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-truncated_002dprint) [exp](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-exp) #:width 20) ([newline](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-newline))
⊣ (ab #(cde) f . g)
([truncated-print](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-truncated_002dprint) "素早い茶色の狐" #:width 20) ([newline](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-newline))
⊣ 「素早く茶色に…」
([truncated-print](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-truncated_002dprint) ([current-module](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-current_002dmodule)) #:width 20) ([newline](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-newline))
⊣ #<ディレクトリ (gui...>

`truncated-print` は末尾の改行を出力しません。式が指定された幅に収まらない場合、式は切り詰められます。場合によっては省略形[27](https://doc.guix.gnu.org/guile/latest/en/guile.html#FOOT27)になるか、最悪の場合は `#` として表示されます。

Scheme Procedure: **truncated-print** obj \[port\] \[keyword-options\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-truncated_002dprint)

obj を出力します。必要に応じて出力を切り詰め、幅の文字数に収まるようにします。デフォルトでは、obj は `write` を使用して出力されますが、この動作は `display?` キーワード引数で上書きできます。

デフォルトの動作は深さ優先で出力されます。つまり、残りの幅全体が、 obj の各部分式に割り当てられます。たとえば、obj がベクトルの場合、obj の各要素に割り当てられます。幅優先? キーワード引数を使用することで、利用可能な幅を「配分」し、各部分式に均等に割り当てることができます。

その他のキーワードオプションは、以下のキーワードとパラメータです。

`#:display?`フラグ

flag が true の場合、`display` を使用して出力します。デフォルトは `#f` で、これは `write` スタイルを使用することを意味します。[スキーム値の書き込み](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scheme-Write) を参照してください。

`#:width`列

指定された列数内に印刷します。デフォルト値は79です。

`#:幅優先?`フラグ

flagがtrueの場合、複合データ構造（リスト、ベクトル、ペアなど）の要素間で、利用可能な幅を幅優先で割り当てます。デフォルトは`#f`で、これはどの要素でも利用可能な幅をすべて使用できることを意味します。

* * *

次へ: [ファイルツリーウォーク](https://doc.guix.gnu.org/guile/latest/en/guile.html#File-Tree-Walk)、前: [整形印刷](https://doc.guix.gnu.org/guile/latest/en/guile.html#Pretty-Printing)、上: [Guile モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guile-Modules) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
