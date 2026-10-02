#### 7.5.20 SRFI-28 - 基本フォーマット文字列[¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d28-_002d-Basic-Format-Strings)

SRFI-28 は、`~a`、`~s`、`~%`、および `~~` フォーマット指定子のみを提供する基本的な `format` プロシージャを提供します。このプロシージャは、次のようにインポートできます。

([use-modules](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-use_002dmodules) (srfi srfi-28))

Scheme手順: **format** メッセージ引数 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-format)

指定されたメッセージをフォーマット文字列として使用し、フォーマットされたメッセージを返します。フォーマット文字列には、以下のフォーマット指定子を含めることができます。

`~a`

次の引数のテキスト表現を、`display` で表示されるかのように挿入します。

`~s`

次の引数のテキスト表現を、`write` で出力したかのように挿入します。

`~%`

改行を挿入してください。

`~~`

チルダを挿入してください。

この手順は、出力先として `#f` を指定して `simple-format` を呼び出すのと同じです ([Simple Textual Output](https://doc.guix.gnu.org/guile/latest/en/guile.html#Simple-Output) を参照)。

* * *

次へ: [SRFI-31 - 再帰評価のための特殊形式 'rec'](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d31)、前: [SRFI-28 - 基本フォーマット文字列](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d28)、上: [SRFI サポート モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI-Support) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
