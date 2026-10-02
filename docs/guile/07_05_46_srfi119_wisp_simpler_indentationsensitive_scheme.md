#### 7.5.46 SRFI-119 Wisp: よりシンプルなインデント対応スキーム。[¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d119-Wisp_003a-simpler-indentation_002dsensitive-Scheme_002e)

Guileに同梱されている言語には、_Wisp_（「Whitespace to Lisp」の略）とも呼ばれるSRFI-119が含まれています。これは、括弧を同等のインデントとインラインコロンに置き換えることができるSchemeのエンコーディングです。[SRFI-119の仕様](http://srfi.schemers.org/srfi-119/srfi-119.html)を参照してください。いくつかの例を以下に示します。

display "Hello World!" ⇒ (display "Hello World!")

define : factorial n ⇒ (define (factorial n)
if : ゼロ? n ⇒ (if (ゼロ? n)
. 1 ⇒ 1
\* n : factorial {n - 1} ⇒ (\* n (factorial {n - 1}))))

Wispコードを含むファイルを実行するには、`guile --language=wisp -x .w` のように言語とファイル名拡張子 `.w` を選択します。

Wisp を使用するファイルでは、[SRFI-105 Curly-infix expressions.](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d105) を参照してください。(Curly Infix) は常に有効になります。

* * *

次へ: [SRFI-197: パイプライン演算子](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d197)、前: [SRFI-119 Wisp: よりシンプルなインデント対応スキーム](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d119)、上: [SRFI サポート モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI-Support) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
