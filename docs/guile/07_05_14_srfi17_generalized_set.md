#### 7.5.14 SRFI-17 - 一般化セット!

この SRFI は汎用的な `set!` を実装しており、いくつかの「参照」関数を `set!` のターゲット位置として使用できます。この機能は以下から利用できます。

(use-modules (srfi srfi-17))

例えば、`vector-ref`は以下のように拡張されます。

(set! (vector-ref vec idx) new-value)

と同等

(ベクトルセット! vec idx new-value)

`vector-ref`式は、取得または格納可能な場所を識別するという考え方です。どちらの場合も同じ形式が使用されるため、視覚的な明瞭さが保たれます。これは、C言語の「lvalue」の概念に似ています。

この種の `set!` のメカニズムは Guile コアにあります ([Procedures with Setters](06_07_procedures.md#678-セッター付きプロシージャ) を参照)。このモジュールは、以下の関数をセッター付きプロシージャとして定義し、`set!` のターゲットにできるようにします。

> `car`, `cdr`, `caar`, `cadr`, `cdar`, `cddr`, `caaar`, `caadr`, `cadar`, `caddr`, `cdaar`, `cdadr`, `cddar`, `cdddr`, `caaaar`, `caaadr`, `caadar`, `caaddr`, `cadaar`、`cadadr`、`caddar`、`cadddr`、`cdaaar`、`cdaadr`、`cdadar`、`cdaddr`、`cddaar`、`cddadr`、`cdddar`、`cddddr`
>
> `string-ref`、`vector-ref`

SRFI では、`setter` ([Procedures with Setters](06_07_procedures.md#678-セッター付きプロシージャ) を参照) をセッター付きプロシージャとして指定しており、プロシージャのセッターを変更できるようにしています。例: `(set! (setter foo) my-new-setter-handler)`。現在 Guile ではこの機能は実装されておらず、セッターは作成時にのみ指定できます (下記の `getter-with-setter`)。

機能: **getter-with-setter**

Guile コアの `make-procedure-with-setter` と同じです ([Procedures with Setters](06_07_procedures.md#678-セッター付きプロシージャ) を参照)。

* * *

次へ: [SRFI-19 - 時刻/日付ライブラリ](07_05_16_srfi19_timedate_library.md#7516-srfi-19---時刻日付ライブラリ)、前: [SRFI-17 - 一般化セット!](#7514-srfi-17---一般化セット)、上: [SRFI サポート モジュール](07_05_00_srfi_support_modules.md#75-srfi-サポート-モジュール) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
