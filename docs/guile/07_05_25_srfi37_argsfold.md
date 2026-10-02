#### 7.5.25 SRFI-37 - args-fold

これは、GNU `getopt_long` スタイルのプログラム引数用のプロセッサです。`(ice-9 getopt-long)` の `getopt-long` よりも宣言性の低い代替インターフェースを提供します ([The (ice-9 getopt-long) Module](07_04_the_ice9_getoptlong_module.md#74-ice-9-getopt-long-モジュール) を参照)。`getopt-long` とは異なり、オプションの繰り返しと、オプションごとに任意の数の短い名前と長い名前をサポートします。アクセスするには、次のようにします。

([use-modules](06_18_modules.md#6182-guileモジュールの使用) (srfi srfi-37))

SRFI-37は主に`option`型と`args-fold`関数を提供します。このライブラリを使用するには、`option`でオプションのセットを作成し、それを`args-fold`を呼び出すための仕様として使用します。

以下は、一般的な「--version」および「--help」オプション用のシンプルな引数プロセッサの例です。このプロセッサは、コマンドラインで指定されたファイルのリストを逆順に返します。

([args-fold](#7525-srfi-37---args-fold) ([cdr](06_06_08_pairs.md#668-ペア) ([プログラム引数](07_02_06_runtime_environment.md#726-ランタイム環境)))
(let ((display-and-exit-proc
(ラムダ (msg)
(lambda (opt name arg loads)
([display](06_16_reading_and_evaluating_scheme_code.md#6164-scheme値の記述) msg) ([quit](04_programming_in_scheme.md#4448-システムコマンド))))))
([list](06_06_09_lists.md#6693-リストコンストラクタ) ([option](04_programming_in_scheme.md#4448-システムコマンド) '(#\\v "version") #f #f
(display-and-exit-proc "Foo version 42.0\n"))
([オプション](04_programming_in_scheme.md#4448-システムコマンド) '(#\\h "help") #f #f
(表示および終了プロシージャ)
"使用方法: foo scheme-file ..."))))
(lambda (opt name arg loads)
([error](04_programming_in_scheme.md#4446-デバッグコマンド) "認識されないオプション \`~A'" 名))
(lambda (op loads) ([cons](06_06_08_pairs.md#668-ペア) op loads))
'())

Scheme Procedure: **option** names required-arg? optional-arg? processor

単一の種類のプログラムオプションを指定するオブジェクトを返します。

names はコマンドラインオプション名のリストであり、従来の `getopt` の短いオプションの場合は文字、`getopt_long` スタイルの長いオプションの場合は文字列で構成される必要があります。

required-arg? と optional-arg? は相互に排他的です。どちらか一方、または両方が `#f` である必要があります。required-arg? の場合、長いオプションの場合は '\--opt=value' のように、コマンドラインでオプションの後に引数を指定する必要があります。指定しない場合はエラーが発生します。optional-arg? の場合、引数が利用可能な場合は引数が使用されます。

processor は、少なくとも 3 つの引数を取るプロシージャで、`args-fold` がオプションを検出したときに呼び出されます。引数は、オプションを含むオブジェクト、コマンドラインで使用される名前、およびオプションに指定された引数 (引数がない場合は `#f`) です。残りの引数は `args-fold` の「シード」であり、processor はシードも返す必要があります。

スキーム手順: **option-names** opt

Scheme Procedure: **option-required-arg?** opt

スキームプロシージャ: **option-optional-arg?** opt

スキーム手順: **option-processor** opt

上記で説明した `option` と同様に、オプションオブジェクトである opt の指定されたフィールドを返します。

Scheme Procedure: **args-fold** args options unrecognized-option-proc operand-proc seed …

`(cdr (program-arguments))` によって返されるようなプログラム引数のリストである args を、上記のようにオプションオブジェクトのリストである options に対して順番に処理します。呼び出されるすべての関数は、「seeds」、つまり seed … から始まる最後の複数値を複数引数として受け取り、新しい seeds を返さなければなりません。最終的な seeds を返します。

オプションに見つからないオプションについては、オプションオブジェクトのプロセッサのような`unrecognized-option-proc`を呼び出してください。

コマンドラインで名前付きオプション以外の項目を指定して `operand-proc` を呼び出します。これには、'\--' の後の引数も含まれます。呼び出し時には、対象の引数とシード値が渡されます。

* * *

次へ: [SRFI-39 - パラメータ](07_05_27_srfi39_parameters.md#7527-srfi-39---パラメータ)、前: [SRFI-37 - args-fold](#7525-srfi-37---args-fold)、上: [SRFI サポート モジュール](07_05_00_srfi_support_modules.md#75-srfi-サポート-モジュール) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
