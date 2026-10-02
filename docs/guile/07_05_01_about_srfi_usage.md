#### 7.5.1 SRFI の使用方法について [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#About-SRFI-Usage-1)

GuileにおけるSRFIのサポートは、現在、コアライブラリとアドオンモジュールによって部分的に実装されています。つまり、一部のSRFIはインタープリタの起動時に自動的に利用可能になりますが、その他のSRFIは適切なサポートモジュールを明示的に使用する必要があります。

この不整合にはいくつかの理由があります。まず、機能チェック構文形式 `cond-expand` ([SRFI-0 - cond-expand](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d0) を参照) はすぐに利用できる必要があります。なぜなら、ユーザーが Scheme 実装をチェックしたいとき、つまり `use-modules` を使用して SRFI サポート モジュールをロードしても安全だとユーザーが知る前に、この機能が利用可能でなければならないからです。2 番目の理由は、開発者が SRFI 実装をモジュールとして追加し始める前に、SRFI で定義された機能の一部が Guile に実装されていたことです (たとえば SRFI-13 ([SRFI-13 - String Library](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d13) を参照))。将来的には、コアライブラリ内の SRFI が個別のモジュールに分離され、必要なときに明示的にモジュールをロードする必要が生じる可能性があります。そのため、将来的に SRFI-13 バインディングにアクセスするために `use-modules` を使用する必要が生じることを想定しておく必要があります。必要であれば、既にそのようにすることも可能です。配布パッケージには、現時点では何も機能しないものの、将来的に安全なコードを記述できるようにするためのモジュール `(srfi srfi-13)` が含まれています。

一般的に、特定の SRFI のサポートは、`(srfi srfi-number)` という名前のモジュールを使用することで提供されます。ここで、number は必要な SRFI の番号です。別の方法として、コマンドラインオプション `--use-srfi` を使用することもできます。このオプションを使用すると、必要なモジュールが自動的にロードされます ([Guile の呼び出し](https://doc.guix.gnu.org/guile/latest/en/guile.html#Invoking-Guile) を参照)。

* * *

次へ: [SRFI-1 - ライブラリ一覧](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1)、前: [SRFI の使用方法について](https://doc.guix.gnu.org/guile/latest/en/guile.html#About-SRFI-Usage)、上: [SRFI サポート モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI-Support) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
