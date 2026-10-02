### 7.16 Expect

このセクションのマクロは、以下の方法で利用できます。

([use-modules](06_18_modules.md#6182-guileモジュールの使用) (ice-9 [expect](#716-expect)))

`expect` は、ポートからの出力に基づいてアクションを選択するためのマクロです。この名前は、Don Libes 氏が作成した同様の機能を持つツールに由来しています。特定の文字列が一致した場合、タイムアウトが発生した場合、またはポートでファイルの終端が検出された場合などにアクションを実行できます。`expect` マクロについては以下で説明します。`expect-strings` は、regexec をベースにした `expect` のフロントエンドです (正規表現のドキュメントを参照してください)。

マクロ: **expect-strings** 句 …

デフォルトでは、`expect-strings` は現在の入力ポートからデータを読み取ります。各句の最初の項は、文字列パターン（正規表現）に評価される式で構成されます。ポートから文字が1文字ずつ読み込まれると、バッファ文字列に蓄積され、各パターンと照合されます。パターンが一致すると、句内の残りの式が評価され、最後の式の値が返されます。例：

([with-input-from-file](06_12_input_and_output.md#612101-ファイルポート) "/etc/passwd"
(ラムダ()
([expect-strings](#716-expect)
("^nobody" ([display](06_16_reading_and_evaluating_scheme_code.md#6164-scheme値の記述) "nobodyユーザーが見つかりました。\n")
([display](06_16_reading_and_evaluating_scheme_code.md#6164-scheme値の記述) "問題ありません。\n"))
("^daemon" ([display](06_16_reading_and_evaluating_scheme_code.md#6164-scheme値の記述) "デーモンユーザーを取得しました。\n")))))

正規表現は`REG_NEWLINE`フラグ付きでコンパイルされるため、^と$のアンカーは文字列の先頭と末尾だけでなく、任意の改行位置で一致します。

節を記述する方法は他に2つあります。

評価する式は省略できます。その場合、パターンが一致すると、正規表現のマッチ結果（match-pickを"")に設定したregexecから取得した文字列に変換されたもの）が返されます。

記号 `=>` は、正規表現の一致が成功した結果を受け入れる手続きであることを示すために使用できます。例:

("^daemon" [\=>](06_08_macros.md#6821-パターン) [write](06_16_reading_and_evaluating_scheme_code.md#6164-scheme値の記述))
("^d(aemon)" [\=>](06_08_macros.md#6821-パターン) (lambda args (for-each [write](06_16_reading_and_evaluating_scheme_code.md#6164-scheme値の記述) args)))
("^da(em)on" [\=>](06_08_macros.md#6821-パターン) (lambda (all [sub](09_03_a_virtual_machine_for_guile.md#9377-組み込み関数呼び出し命令))
([write](06_16_reading_and_evaluating_scheme_code.md#6164-scheme値の記述) all) ([newline](06_12_input_and_output.md#61211-ヴェネラブルポートインターフェース))
([write](06_16_reading_and_evaluating_scheme_code.md#6164-scheme値の記述) [sub](09_03_a_virtual_machine_for_guile.md#9377-組み込み関数呼び出し命令)) ([newline](06_12_input_and_output.md#61211-ヴェネラブルポートインターフェース))))

部分文字列の順序は、開き括弧が現れる順序に対応します。

`expect`（および`expect-strings`）の動作を制御するために、いくつかの変数を使用できます。ほとんどの変数は、デフォルトの動作を生成する値`#f`にデフォルトでトップレベルでバインドされています。これらの変数は、トップレベルで再定義することも、expect式を囲む形式でローカルにバインドすることもできます。

`expect-port`

現在の入力ポートの代わりに、文字を読み取るためのポートを指定します。

`expect-timeout`

`expect` はこの秒数後に終了し、`#f` または expect-timeout-proc によって返される値を返します。

`expect-timeout-proc`

タイムアウトが発生した場合に呼び出されるプロシージャ。このプロシージャは、蓄積された文字列を引数として1つだけ取ります。

`expect-eof-proc`

入力ポートでファイル終端が検出された場合に呼び出されるプロシージャ。このプロシージャは、蓄積された文字列を引数として1つだけ受け取ります。

`expect-char-proc`

ポートから文字が読み込まれるたびに呼び出されるプロシージャ。このプロシージャは、読み込まれた文字を引数として1つだけ受け取ります。

`expect-strings-compile-flags`

正規表現をコンパイルする際に使用するフラグ。これらは`make-regexp`に渡されます。[正規表現関数](06_13_regular_expressions.md#6131-正規表現関数)を参照してください。デフォルト値は`regexp/newline`です。

`expect-strings-exec-flags`

正規表現を実行する際に使用するフラグ。regexp-exec に渡されます。[正規表現関数](06_13_regular_expressions.md#6131-正規表現関数) を参照してください。デフォルト値は `regexp/noteol` で、文字列が蓄積されている間は `$` が文字列の末尾に一致しないようにしますが、改行後やファイルの末尾では一致させることができます。

すべての変数を使用した例を以下に示します。

(let ((expect-port ([open-input-file](06_12_input_and_output.md#612101-ファイルポート) "/etc/passwd"))
(expect-timeout 1)
(expect-timeout-proc
(lambda (s) ([display](06_16_reading_and_evaluating_scheme_code.md#6164-scheme値の記述) "Times up!\n")))
(expect-eof-proc
(lambda (s) ([display](06_16_reading_and_evaluating_scheme_code.md#6164-scheme値の記述) "ファイルの終わりに到達しました!\n")))
(expect-char-proc [display](06_16_reading_and_evaluating_scheme_code.md#6164-scheme値の記述))
(expect-strings-compile-flags ([logior](06_06_02_numerical_data_types.md#66213-ビット演算) [regexp/newline](06_13_regular_expressions.md#6131-正規表現関数) [regexp/icase](06_13_regular_expressions.md#6131-正規表現関数)))
(expect-strings-exec-flags 0))
([expect-strings](#716-expect)
("^nobody" ([display](06_16_reading_and_evaluating_scheme_code.md#6164-scheme値の記述) "nobodyユーザーが見つかりました\n"))))

マクロ: **expect** 句 …

`expect` は `expect-strings` と同じように使用されますが、テストはパターンではなくプロシージャとして指定されます。プロシージャは、ポートから文字が読み取られるたびに順番に呼び出され、2 つの引数が渡されます。1 つは蓄積された文字列の値、もう 1 つはファイルの終端に達したかどうかを示すフラグです。フラグは通常 `#f` ですが、ファイルの終端に達した場合は、プロシージャが最終的な蓄積文字列と `#t` を引数として追加で呼び出されます。

手順が偽以外の値を返した場合、テストは成功です。

`=>`構文が使用されている場合、テストが成功した場合は、対応する式に渡される引数を含むリストを返さなければなりません。

次の例では、文字列はファイルの先頭でのみ一致します。

(let ((expect-port ([open-input-file](06_12_input_and_output.md#612101-ファイルポート) "/etc/passwd")))
([expect](#716-expect)
((lambda (s eof?) ([string=?](06_06_05_strings.md#6657-文字列の比較) s "fnord!"))
([display](06_16_reading_and_evaluating_scheme_code.md#6164-scheme値の記述) "nobodyユーザーが見つかりました！\n"))))

`expect-strings` で説明されている制御変数は、`expect` の動作にも影響を与えますが、名前が `expect-strings-` で始まる変数は例外です。

* * *

次へ: [Scheme シェル (scsh)](07_18_the_scheme_shell_scsh.md#718-schemeシェル-scsh)、前: [Expect](#716-expect)、上: [Guile モジュール](07_00_guile_modules.md#7つのguileモジュール) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
