#### 7.5.46 SRFI-119 Wisp: よりシンプルなインデント対応スキーム。

Guileに同梱されている言語には、_Wisp_（「Whitespace to Lisp」の略）とも呼ばれるSRFI-119が含まれています。これは、括弧を同等のインデントとインラインコロンに置き換えることができるSchemeのエンコーディングです。[SRFI-119の仕様](http://srfi.schemers.org/srfi-119/srfi-119.html)を参照してください。いくつかの例を以下に示します。

display "Hello World!" ⇒ (display "Hello World!")

define : factorial n ⇒ (define (factorial n)
if : ゼロ? n ⇒ (if (ゼロ? n)
. 1 ⇒ 1
\* n : factorial {n - 1} ⇒ (\* n (factorial {n - 1}))))

Wispコードを含むファイルを実行するには、`guile --language=wisp -x .w` のように言語とファイル名拡張子 `.w` を選択します。

Wisp を使用するファイルでは、[SRFI-105 Curly-infix expressions.](07_05_44_srfi105_curlyinfix_expressions.md#7544-srfi-105-中括弧式) を参照してください。(Curly Infix) は常に有効になります。

* * *

次へ: [SRFI-197: パイプライン演算子](07_05_48_srfi197_pipeline_operators.md#7548-srfi-197-パイプライン演算子)、前: [SRFI-119 Wisp: よりシンプルなインデント対応スキーム](#7546-srfi-119-wisp-よりシンプルなインデント対応スキーム)、上: [SRFI サポート モジュール](07_05_00_srfi_support_modules.md#75-srfi-サポート-モジュール) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
