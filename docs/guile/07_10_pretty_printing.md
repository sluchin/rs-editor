### 7.10 整形印刷

モジュール`(ice-9 pretty-print)`は、 Schemeオブジェクトをきれいに整形して出力するプロシージャ`pretty-print`を提供します。これは、リストやベクトルなどの深くネストされたデータ構造や複雑なデータ構造に特に役立ちます。

モジュールは、以下のコマンドを入力することでロードされます。

([use-modules](06_18_modules.md#6182-guileモジュールの使用) (ice-9 [pretty-print](04_programming_in_scheme.md#4447-コマンドの検査)))

これにより、`pretty-print` プロシージャが利用可能になります。`pretty-print` がどのように出力をフォーマットするかの例を以下に示します。

([pretty-print](04_programming_in_scheme.md#4447-コマンドの検査) '(define (foo) (lambda (x)
(cond (([zero?](06_06_02_numerical_data_types.md#6628-比較述語) x) #t) (([negative?](06_06_02_numerical_data_types.md#6628-比較述語) x) \-x) (else
(if ([\=](06_06_02_numerical_data_types.md#6628-比較述語) x 1) 2 ([\*](06_06_02_numerical_data_types.md#66211-算術関数) xxx)))))))
⊣
(define (foo)
(ラムダ (x)
(cond (([zero?](06_06_02_numerical_data_types.md#6628-比較述語) x) #t)
(([negative?](06_06_02_numerical_data_types.md#6628-比較述語) x) \-x)
(else (if ([\=](06_06_02_numerical_data_types.md#6628-比較述語) x 1) 2 ([\*](06_06_02_numerical_data_types.md#66211-算術関数) xxx))))))

Scheme 手順: **pretty-print** obj \[port\] \[keyword-options\]

Schemeオブジェクトobjのテキスト表現をportに出力します。portが指定されていない場合は、現在の出力ポートがデフォルトで使用されます。

その他のキーワードオプションは、以下のキーワードとパラメータです。

`#:display?`フラグ

flagがtrueの場合は、`display`を使用して出力します。デフォルトは`#f`で、これは`write`スタイルを使用することを意味します。[スキーム値の書き込み](06_16_reading_and_evaluating_scheme_code.md#6164-scheme値の記述)を参照してください。

`#:per-line-prefix`文字列

指定された文字列を各行の先頭に付加して表示します。デフォルトでは付加されません。

`#:width`列

指定された列数内に印刷します。デフォルト値は79です。

`#:max-expr-width`列

式の最大幅。デフォルト値は50です。

`(ice-9 pretty-print)`モジュールによってエクスポートされるもう一つの関数は`truncated-print`です。これは、Schemeのデータを出力する際に、出力文字数を一定数に切り詰める処理です。これは、ユーザーに任意のデータを提示する必要があるものの、表示できる行が1行しかない場合に便利です。

(define [exp](06_06_02_numerical_data_types.md#66212-科学関数) '(ab #(cde) f . g))
([truncated-print](#710-整形印刷) [exp](06_06_02_numerical_data_types.md#66212-科学関数) #:width 10) ([newline](06_12_input_and_output.md#61211-ヴェネラブルポートインターフェース))
⊣ (ab . #)
([truncated-print](#710-整形印刷) [exp](06_06_02_numerical_data_types.md#66212-科学関数) #:width 15) ([newline](06_12_input_and_output.md#61211-ヴェネラブルポートインターフェース))
⊣ (ab # f . g)
([truncated-print](#710-整形印刷) [exp](06_06_02_numerical_data_types.md#66212-科学関数) #:width 18) ([newline](06_12_input_and_output.md#61211-ヴェネラブルポートインターフェース))
⊣ (ab #(c [...](06_08_macros.md#6821-パターン)) . #)
([truncated-print](#710-整形印刷) [exp](06_06_02_numerical_data_types.md#66212-科学関数) #:width 20) ([newline](06_12_input_and_output.md#61211-ヴェネラブルポートインターフェース))
⊣ (ab #(cde) f . g)
([truncated-print](#710-整形印刷) "素早い茶色の狐" #:width 20) ([newline](06_12_input_and_output.md#61211-ヴェネラブルポートインターフェース))
⊣ 「素早く茶色に…」
([truncated-print](#710-整形印刷) ([current-module](06_18_modules.md#6188-モジュールシステムリフレクション)) #:width 20) ([newline](06_12_input_and_output.md#61211-ヴェネラブルポートインターフェース))
⊣ #<ディレクトリ (gui...>

`truncated-print` は末尾の改行を出力しません。式が指定された幅に収まらない場合、式は切り詰められます。場合によっては省略形[27](99_footnotes.md#27)になるか、最悪の場合は `#` として表示されます。

Scheme Procedure: **truncated-print** obj \[port\] \[keyword-options\]

obj を出力します。必要に応じて出力を切り詰め、幅の文字数に収まるようにします。デフォルトでは、obj は `write` を使用して出力されますが、この動作は `display?` キーワード引数で上書きできます。

デフォルトの動作は深さ優先で出力されます。つまり、残りの幅全体が、 obj の各部分式に割り当てられます。たとえば、obj がベクトルの場合、obj の各要素に割り当てられます。幅優先? キーワード引数を使用することで、利用可能な幅を「配分」し、各部分式に均等に割り当てることができます。

その他のキーワードオプションは、以下のキーワードとパラメータです。

`#:display?`フラグ

flag が true の場合、`display` を使用して出力します。デフォルトは `#f` で、これは `write` スタイルを使用することを意味します。[スキーム値の書き込み](06_16_reading_and_evaluating_scheme_code.md#6164-scheme値の記述) を参照してください。

`#:width`列

指定された列数内に印刷します。デフォルト値は79です。

`#:幅優先?`フラグ

flagがtrueの場合、複合データ構造（リスト、ベクトル、ペアなど）の要素間で、利用可能な幅を幅優先で割り当てます。デフォルトは`#f`で、これはどの要素でも利用可能な幅をすべて使用できることを意味します。

* * *

次へ: [ファイルツリーウォーク](07_12_file_tree_walk.md#712-ファイルツリーウォーク)、前: [整形印刷](#710-整形印刷)、上: [Guile モジュール](07_00_guile_modules.md#7つのguileモジュール) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
