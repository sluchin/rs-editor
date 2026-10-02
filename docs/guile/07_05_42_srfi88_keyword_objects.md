#### 7.5.42 SRFI-88 キーワードオブジェクト [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d88-Keyword-Objects)

[SRFI-88](http://srfi.schemers.org/srfi-88/srfi-88.html) は、Guile のキーワードと同等の _キーワード オブジェクト_ を提供します ([キーワード](https://doc.guix.gnu.org/guile/latest/en/guile.html#Keywords) を参照)。SRFI-88 のキーワードは、識別子の後に `:` が続く _postfix キーワード構文_ を使用して入力できます ([`postfix` キーワード構文](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scheme-Read) を参照)。SRFI-88 は、以下の方法で利用できます。

(use-modules (srfi srfi-88))

これにより、`(read-set! keywords 'postfix)` を使用して、キーワード構文に適したリーダーオプションがインストールされます。また、以下に説明する手順も提供されます。

Scheme手順: **キーワード?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-keyword_003f-1)

objがキーワードの場合は`#t`を返します。これは、同名の組み込みプロシージャと同じ手順です（[`keyword?`](https://doc.guix.gnu.org/guile/latest/en/guile.html#Keyword-Procedures)を参照）。

(キーワード? foo:) ⇒ #t
(キーワード? 'foo:) ⇒ #t
(キーワード? "foo") ⇒ #f

Scheme手順: **keyword->string** kw [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-keyword_002d_003estring)

kw の名前を文字列として返します。末尾のコロンは含めません。返された文字列は、例えば `string-set!` などで変更することはできません。

(キーワード→文字列 foo:) ⇒ "foo"

Scheme手順: **string->keyword** str [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002d_003ekeyword)

名前がstrであるキーワードオブジェクトを返します。

(キーワード→文字列 (文字列→キーワード "ab c")) ⇒ "ab c"

* * *

次へ: [SRFI-105 中括弧式](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d105)、前: [SRFI-88 キーワードオブジェクト](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d88)、上: [SRFI サポートモジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI-Support) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
