#### 7.5.25 SRFI-37 - args-fold [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d37-_002d-args_002dfold)

これは、GNU `getopt_long` スタイルのプログラム引数用のプロセッサです。`(ice-9 getopt-long)` の `getopt-long` よりも宣言性の低い代替インターフェースを提供します ([The (ice-9 getopt-long) Module](https://doc.guix.gnu.org/guile/latest/en/guile.html#getopt_002dlong) を参照)。`getopt-long` とは異なり、オプションの繰り返しと、オプションごとに任意の数の短い名前と長い名前をサポートします。アクセスするには、次のようにします。

([use-modules](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-use_002dmodules) (srfi srfi-37))

SRFI-37は主に`option`型と`args-fold`関数を提供します。このライブラリを使用するには、`option`でオプションのセットを作成し、それを`args-fold`を呼び出すための仕様として使用します。

以下は、一般的な「--version」および「--help」オプション用のシンプルな引数プロセッサの例です。このプロセッサは、コマンドラインで指定されたファイルのリストを逆順に返します。

([args-fold](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-args_002dfold) ([cdr](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-cdr) ([プログラム引数](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-program_002darguments)))
(let ((display-and-exit-proc
(ラムダ (msg)
(lambda (opt name arg loads)
([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) msg) ([quit](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-quit))))))
([list](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-list-1) ([option](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-option) '(#\\v "version") #f #f
(display-and-exit-proc "Foo version 42.0\n"))
([オプション](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-option) '(#\\h "help") #f #f
(表示および終了プロシージャ)
"使用方法: foo scheme-file ..."))))
(lambda (opt name arg loads)
([error](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-error) "認識されないオプション \`~A'" 名))
(lambda (op loads) ([cons](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-cons) op loads))
'())

Scheme Procedure: **option** names required-arg? optional-arg? processor [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-option-1)

単一の種類のプログラムオプションを指定するオブジェクトを返します。

names はコマンドラインオプション名のリストであり、従来の `getopt` の短いオプションの場合は文字、`getopt_long` スタイルの長いオプションの場合は文字列で構成される必要があります。

required-arg? と optional-arg? は相互に排他的です。どちらか一方、または両方が `#f` である必要があります。required-arg? の場合、長いオプションの場合は '\--opt=value' のように、コマンドラインでオプションの後に引数を指定する必要があります。指定しない場合はエラーが発生します。optional-arg? の場合、引数が利用可能な場合は引数が使用されます。

processor は、少なくとも 3 つの引数を取るプロシージャで、`args-fold` がオプションを検出したときに呼び出されます。引数は、オプションを含むオブジェクト、コマンドラインで使用される名前、およびオプションに指定された引数 (引数がない場合は `#f`) です。残りの引数は `args-fold` の「シード」であり、processor はシードも返す必要があります。

スキーム手順: **option-names** opt [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-option_002dnames)

Scheme Procedure: **option-required-arg?** opt [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-option_002drequired_002darg_003f)

スキームプロシージャ: **option-optional-arg?** opt [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-option_002doptional_002darg_003f)

スキーム手順: **option-processor** opt [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-option_002dprocessor)

上記で説明した `option` と同様に、オプションオブジェクトである opt の指定されたフィールドを返します。

Scheme Procedure: **args-fold** args options unrecognized-option-proc operand-proc seed … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-args_002dfold)

`(cdr (program-arguments))` によって返されるようなプログラム引数のリストである args を、上記のようにオプションオブジェクトのリストである options に対して順番に処理します。呼び出されるすべての関数は、「seeds」、つまり seed … から始まる最後の複数値を複数引数として受け取り、新しい seeds を返さなければなりません。最終的な seeds を返します。

オプションに見つからないオプションについては、オプションオブジェクトのプロセッサのような`unrecognized-option-proc`を呼び出してください。

コマンドラインで名前付きオプション以外の項目を指定して `operand-proc` を呼び出します。これには、'\--' の後の引数も含まれます。呼び出し時には、対象の引数とシード値が渡されます。

* * *

次へ: [SRFI-39 - パラメータ](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d39)、前: [SRFI-37 - args-fold](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d37)、上: [SRFI サポート モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI-Support) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
