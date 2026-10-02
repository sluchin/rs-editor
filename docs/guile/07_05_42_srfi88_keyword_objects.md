#### 7.5.42 SRFI-88 キーワードオブジェクト

[SRFI-88](http://srfi.schemers.org/srfi-88/srfi-88.html) は、Guile のキーワードと同等の _キーワード オブジェクト_ を提供します ([キーワード](06_06_07_keywords.md#667-キーワード) を参照)。SRFI-88 のキーワードは、識別子の後に `:` が続く _postfix キーワード構文_ を使用して入力できます ([`postfix` キーワード構文](06_16_reading_and_evaluating_scheme_code.md#6162-リーディングスキームコード) を参照)。SRFI-88 は、以下の方法で利用できます。

(use-modules (srfi srfi-88))

これにより、`(read-set! keywords 'postfix)` を使用して、キーワード構文に適したリーダーオプションがインストールされます。また、以下に説明する手順も提供されます。

Scheme手順: **キーワード?** obj

objがキーワードの場合は`#t`を返します。これは、同名の組み込みプロシージャと同じ手順です（[`keyword?`](06_06_07_keywords.md#6674-キーワードプロシージャ)を参照）。

(キーワード? foo:) ⇒ #t
(キーワード? 'foo:) ⇒ #t
(キーワード? "foo") ⇒ #f

Scheme手順: **keyword->string** kw

kw の名前を文字列として返します。末尾のコロンは含めません。返された文字列は、例えば `string-set!` などで変更することはできません。

(キーワード→文字列 foo:) ⇒ "foo"

Scheme手順: **string->keyword** str

名前がstrであるキーワードオブジェクトを返します。

(キーワード→文字列 (文字列→キーワード "ab c")) ⇒ "ab c"

* * *

次へ: [SRFI-105 中括弧式](07_05_44_srfi105_curlyinfix_expressions.md#7544-srfi-105-中括弧式)、前: [SRFI-88 キーワードオブジェクト](#7542-srfi-88-キーワードオブジェクト)、上: [SRFI サポートモジュール](07_05_00_srfi_support_modules.md#75-srfi-サポート-モジュール) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
