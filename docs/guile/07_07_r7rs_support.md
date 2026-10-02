### 7.7 R7RS サポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#R7RS-Support-1)

[R7RS](https://small.r7rs.org/) 標準は、基本的に R5RS (Guile によって直接サポートされています) にモジュール機能とバインディングを標準モジュールセットに整理したもので、

幸いなことに、R7RS モジュールの構文は R6RS と互換性があるように設計されているため、Guile のドキュメントはそのまま適用されます。R6RS ライブラリの定義方法と Guile モジュールとの統合方法の詳細については、[R6RS ライブラリ](https://doc.guix.gnu.org/guile/latest/en/guile.html#R6RS-Libraries) を参照してください。また、[ライブラリの使用方法](https://doc.guix.gnu.org/guile/latest/en/guile.html#Library-Usage) も参照してください。

* [R7RSとの非互換性](https://doc.guix.gnu.org/guile/latest/en/guile.html#R7RS-Incompatibilities)
* [R7RS 標準ライブラリ](https://doc.guix.gnu.org/guile/latest/en/guile.html#R7RS-Standard-Libraries)

* * *

次へ: [R7RS 標準ライブラリ](https://doc.guix.gnu.org/guile/latest/en/guile.html#R7RS-Standard-Libraries)、上へ: [R7RS サポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#R7RS-Support) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.7.1 R7RSとの非互換性 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Incompatibilities-with-the-R7RS)

R7RSはR6RSに比べてはるかに野心的な標準ではないため（[GuileとScheme](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guile-and-Scheme)を参照）、Guileでのサポートは非常に容易です。そのため、Guileは時折発生するバグといくつかの未実装機能を除けば、R7RSに完全に準拠した実装となっています。

* R7RS では、`#0=(1 2 3 . #0#)` のような _データラベル_ を使用して循環データ構造を読み取る構文が指定されています。Guile のリーダーは現在この構文をサポートしていません。[https://bugs.gnu.org/38236](https://bugs.gnu.org/38236)。
* R6RSと同様に、R7RSの多くの字句機能はGuileの従来の構文と競合します。`r6rs-hex-escapes`と`hungry-eol-escapes`（[R6RSとの非互換性](https://doc.guix.gnu.org/guile/latest/en/guile.html#R6RS-Incompatibilities)を参照）に加えて、`r7rs-symbols`リーダー機能を明示的に有効にする必要があります。

Guileは、ルートモジュール内に、Guileの従来のデフォルト設定よりもR7RSのデフォルト設定を選択するための手順を公開しています。

Scheme 手順: **install-r7rs!** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-install_002dr7rs_0021)

ガイルのデフォルト設定をR7RSにより適合するように変更する。

Guile のデフォルト設定は今後変更される可能性がありますが、この手順で実施する変更は、R7RS の規約をより適切にサポートするために、`.sls` と `.guile.sls` をサポートされている `%load-extensions` のセットに追加することです。[ロード パス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Load-Paths) を参照してください。`install-r7rs!` を実行すると、上記のリーダー オプションも有効になります。

最後に、`--r7rs` コマンドライン引数は、ユーザーコードを呼び出す前に `install-r7rs!` を呼び出すことに注意してください。R7RS ユーザーは、この引数を Guile に渡したい場合が多いでしょう。

* * *

前へ: [R7RSとの非互換性](https://doc.guix.gnu.org/guile/latest/en/guile.html#R7RS-Incompatibilities)、上へ: [R7RSのサポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#R7RS-Support) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.7.2 R7RS 標準ライブラリ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#R7RS-Standard-Libraries-1)

R7RSは、R5RSの定義をモジュールごとに整理し、さらにいくつかの新しい定義を追加しています。

R6RSとは異なり、R7RSにはR5RSに比べて新しい定義がほとんどないため、ここではこれらのライブラリを完全に文書化することは試みません。これらのライブラリの機能のほとんどは、すでにGuileの標準環境に組み込まれています。繰り返しになりますが、ほとんどのGuileユーザーは、よく知られ、十分に文書化されたGuileモジュールを使用することが想定されています。これらのR7RSライブラリは、主にコードを他のR7RSシステムに移植したいユーザーにとって有用です。

簡単に概説すると、R7RSで定義されているライブラリは以下のとおりです。

（スキームベース）

コア機能は、主に R5RS に対応していますが、以下に個別にリストされている要素を除き、SRFI-34 エラー処理 ([SRFI-34 - プログラムの例外処理](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d34) を参照)、バイトベクトルとバイトベクトルポート ([Bytevectors](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevectors) を参照)、およびその他のいくつかの新しい手順が追加されています。

`(scheme case-lambda)`

`case-lambda`。

`(スキーム文字)`

文字列や文字を大文字または小文字に変換する、文字が数値かどうかを判定する述語など。

（スキーム複合体）

複素数のためのコンストラクタとアクセサ。

`(スキーム cxr)`

`cddr`、`cadadr`、その他諸々。

`(scheme eval)`

`eval`だけでなく、モジュールインポートセットを使用してユーザーが環境を指定できる`environment`ルーチンも含まれています。

（スキームファイル）

`call-with-input-file` など。

（スキームが不正確）

不正確な数値を扱うルーチン: `sin`、`finite?`など。

`(scheme lazy)`

約束。

（スキームの読み込み）

`load` 手順。

`(scheme process-context)`

環境変数。[SRFI-98 環境変数へのアクセス](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d98) を参照してください。また、`command-line`、`emergency-exit` (Guile の `primitive-_exit` と同様)、および `exit` も参照してください。

`(スキーム r5rs)`

`r5rs` によってエクスポートされる正確なバインディングのセットですが、`transcript-off` / `transcript-on` は含まれておらず、`_` や `else` などの補助構文定義も含まれています。補助構文の詳細については、[構文ルール マクロ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Syntax-Rules) を参照してください。

（スキーム読み込み）

`read`プロシージャ。

（スキームの複製）

`interaction-environment`手順。

（スキーム時間）

`current-second`、`current-jiffy`、`jiffies-per-second`。Guileは、R7RSが「jiffies」と呼ぶものを「内部時間単位」と呼んでいます。

（スキーム書き込み）

`display`、`write`、`write-shared`、`write-simple`。

完全なドキュメントについては、関心のあるユーザーはR7RSを直接参照することをお勧めします（アルゴリズム言語スキームに関する改訂版^7レポートの[R7RS](https://doc.guix.gnu.org/guile/latest/en/r7rs.html#R7RS)を参照）。

* * *

次へ: [Readline サポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Readline-Support)、前: [R7RS サポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#R7RS-Support)、上: [Guile モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guile-Modules) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
