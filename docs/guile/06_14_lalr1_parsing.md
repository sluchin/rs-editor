### 6.14 LALR(1) 構文解析

`(system base lalr)` モジュールは、Dominique Boucher 氏による LALR(1) パーサージェネレータである [`lalr-scm` を提供します](https://github.com/schemeway/lalr-scm/)。`lalr-scm` は、GNU Bison と同じアルゴリズムを使用します (Bison、Yacc 互換パーサージェネレータの [Bison 入門](https://www.gnu.org/software/bison/manual/bison.html#Introduction) を参照)。パーサーは `lalr-parser` マクロを使用して定義されます。

Scheme構文: **lalr-parser** \[options\] トークンルール...

LALR(1)構文解析器を生成します。tokensは文法の終端記号を表す記号のリストです。rulesは文法生成規則です。

各ルールは `(非終端記号 (右辺 ...) : アクション ...)` という形式をとります。ここで、非終端記号はルールの名前、右辺は生成ルール、アクションはルールに関連付けられた意味アクションです。

生成されるパーサーは、_トークナイザー_と_構文エラー処理_という2つの引数を取る手続きです。トークナイザーは、`make-lexical-token`によって生成された字句トークンを返すサンクである必要があります。構文エラー処理は、少なくともエラーメッセージ（文字列）と、オプションでエラーの原因となった字句トークンを指定して呼び出すことができます。

詳細については、`lalr-scm`のドキュメントを参照してください。

* * *

次へ: [スキームコードの読み込みと評価](06_16_reading_and_evaluating_scheme_code.md#616-scheme-コードの読み取りと評価)、前: [LALR(1) 解析](#614-lalr1-構文解析)、上: [API リファレンス](06_00_api_reference.md#6-apiリファレンス) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
