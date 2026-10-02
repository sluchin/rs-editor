#### 7.5.14 SRFI-17 - 一般化セット! [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d17-_002d-Generalized-set_0021)

この SRFI は汎用的な `set!` を実装しており、いくつかの「参照」関数を `set!` のターゲット位置として使用できます。この機能は以下から利用できます。

(use-modules (srfi srfi-17))

例えば、`vector-ref`は以下のように拡張されます。

(set! (vector-ref vec idx) new-value)

と同等

(ベクトルセット! vec idx new-value)

`vector-ref`式は、取得または格納可能な場所を識別するという考え方です。どちらの場合も同じ形式が使用されるため、視覚的な明瞭さが保たれます。これは、C言語の「lvalue」の概念に似ています。

この種の `set!` のメカニズムは Guile コアにあります ([Procedures with Setters](https://doc.guix.gnu.org/guile/latest/en/guile.html#Procedures-with-Setters) を参照)。このモジュールは、以下の関数をセッター付きプロシージャとして定義し、`set!` のターゲットにできるようにします。

> `car`, `cdr`, `caar`, `cadr`, `cdar`, `cddr`, `caaar`, `caadr`, `cadar`, `caddr`, `cdaar`, `cdadr`, `cddar`, `cdddr`, `caaaar`, `caaadr`, `caadar`, `caaddr`, `cadaar`、`cadadr`、`caddar`、`cadddr`、`cdaaar`、`cdaadr`、`cdadar`、`cdaddr`、`cddaar`、`cddadr`、`cdddar`、`cddddr`
>
> `string-ref`、`vector-ref`

SRFI では、`setter` ([Procedures with Setters](https://doc.guix.gnu.org/guile/latest/en/guile.html#Procedures-with-Setters) を参照) をセッター付きプロシージャとして指定しており、プロシージャのセッターを変更できるようにしています。例: `(set! (setter foo) my-new-setter-handler)`。現在 Guile ではこの機能は実装されておらず、セッターは作成時にのみ指定できます (下記の `getter-with-setter`)。

機能: **getter-with-setter** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-getter_002dwith_002dsetter)

Guile コアの `make-procedure-with-setter` と同じです ([Procedures with Setters](https://doc.guix.gnu.org/guile/latest/en/guile.html#Procedures-with-Setters) を参照)。

* * *

次へ: [SRFI-19 - 時刻/日付ライブラリ](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d19)、前: [SRFI-17 - 一般化セット!](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d17)、上: [SRFI サポート モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI-Support) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
