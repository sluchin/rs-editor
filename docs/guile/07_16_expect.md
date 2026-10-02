### 7.16 Expect [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Expect-1)

このセクションのマクロは、以下の方法で利用できます。

([use-modules](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-use_002dmodules) (ice-9 [expect](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-expect)))

`expect` は、ポートからの出力に基づいてアクションを選択するためのマクロです。この名前は、Don Libes 氏が作成した同様の機能を持つツールに由来しています。特定の文字列が一致した場合、タイムアウトが発生した場合、またはポートでファイルの終端が検出された場合などにアクションを実行できます。`expect` マクロについては以下で説明します。`expect-strings` は、regexec をベースにした `expect` のフロントエンドです (正規表現のドキュメントを参照してください)。

マクロ: **expect-strings** 句 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-expect_002dstrings)

デフォルトでは、`expect-strings` は現在の入力ポートからデータを読み取ります。各句の最初の項は、文字列パターン（正規表現）に評価される式で構成されます。ポートから文字が1文字ずつ読み込まれると、バッファ文字列に蓄積され、各パターンと照合されます。パターンが一致すると、句内の残りの式が評価され、最後の式の値が返されます。例：

([with-input-from-file](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-with_002dinput_002dfrom_002dfile) "/etc/passwd"
(ラムダ()
([expect-strings](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-expect_002dstrings)
("^nobody" ([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) "nobodyユーザーが見つかりました。\n")
([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) "問題ありません。\n"))
("^daemon" ([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) "デーモンユーザーを取得しました。\n")))))

正規表現は`REG_NEWLINE`フラグ付きでコンパイルされるため、^と$のアンカーは文字列の先頭と末尾だけでなく、任意の改行位置で一致します。

節を記述する方法は他に2つあります。

評価する式は省略できます。その場合、パターンが一致すると、正規表現のマッチ結果（match-pickを"")に設定したregexecから取得した文字列に変換されたもの）が返されます。

記号 `=>` は、正規表現の一致が成功した結果を受け入れる手続きであることを示すために使用できます。例:

("^daemon" [\=>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d_003e) [write](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-write))
("^d(aemon)" [\=>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d_003e) (lambda args (for-each [write](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-write) args)))
("^da(em)on" [\=>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d_003e) (lambda (all [sub](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sub))
([write](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-write) all) ([newline](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-newline))
([write](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-write) [sub](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sub)) ([newline](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-newline))))

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

正規表現をコンパイルする際に使用するフラグ。これらは`make-regexp`に渡されます。[正規表現関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Regexp-Functions)を参照してください。デフォルト値は`regexp/newline`です。

`expect-strings-exec-flags`

正規表現を実行する際に使用するフラグ。regexp-exec に渡されます。[正規表現関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Regexp-Functions) を参照してください。デフォルト値は `regexp/noteol` で、文字列が蓄積されている間は `$` が文字列の末尾に一致しないようにしますが、改行後やファイルの末尾では一致させることができます。

すべての変数を使用した例を以下に示します。

(let ((expect-port ([open-input-file](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-open_002dinput_002dfile) "/etc/passwd"))
(expect-timeout 1)
(expect-timeout-proc
(lambda (s) ([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) "Times up!\n")))
(expect-eof-proc
(lambda (s) ([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) "ファイルの終わりに到達しました!\n")))
(expect-char-proc [display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display))
(expect-strings-compile-flags ([logior](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-logior) [regexp/newline](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-regexp_002fnewline) [regexp/icase](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-regexp_002ficase)))
(expect-strings-exec-flags 0))
([expect-strings](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-expect_002dstrings)
("^nobody" ([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) "nobodyユーザーが見つかりました\n"))))

マクロ: **expect** 句 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-expect)

`expect` は `expect-strings` と同じように使用されますが、テストはパターンではなくプロシージャとして指定されます。プロシージャは、ポートから文字が読み取られるたびに順番に呼び出され、2 つの引数が渡されます。1 つは蓄積された文字列の値、もう 1 つはファイルの終端に達したかどうかを示すフラグです。フラグは通常 `#f` ですが、ファイルの終端に達した場合は、プロシージャが最終的な蓄積文字列と `#t` を引数として追加で呼び出されます。

手順が偽以外の値を返した場合、テストは成功です。

`=>`構文が使用されている場合、テストが成功した場合は、対応する式に渡される引数を含むリストを返さなければなりません。

次の例では、文字列はファイルの先頭でのみ一致します。

(let ((expect-port ([open-input-file](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-open_002dinput_002dfile) "/etc/passwd")))
([expect](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-expect)
((lambda (s eof?) ([string=?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_003d_003f) s "fnord!"))
([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) "nobodyユーザーが見つかりました！\n"))))

`expect-strings` で説明されている制御変数は、`expect` の動作にも影響を与えますが、名前が `expect-strings-` で始まる変数は例外です。

* * *

次へ: [Scheme シェル (scsh)](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Scheme-shell-_0028scsh_0029)、前: [Expect](https://doc.guix.gnu.org/guile/latest/en/guile.html#Expect)、上: [Guile モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guile-Modules) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
