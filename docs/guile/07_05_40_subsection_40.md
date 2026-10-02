#### 7.5.40 SRFI-71 - 複数値のための拡張let構文 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d71-_002d-Extended-let_002dsyntax-for-multiple-values )

このSRFIは、`let`、`let*`、および`letrec`の形式をシャドウイングし、複数の値を受け入れることができるようにします。例：

(use-modules (srfi srfi-71))

(let\* ((xy (values 1 2))
(z (+ xy)))
(* z 2))
⇒ 6

[SRFI-71の仕様書](http://srfi.schemers.org/srfi-71/srfi-71.html)を参照してください。

* * *

次へ: [SRFI-88 キーワードオブジェクト](07_05_42_srfi88_keyword_objects.md#7542-srfi-88-キーワードオブジェクト)、前: [SRFI-71 - 複数値のための拡張 let 構文](#7540-srfi-71---複数値のための拡張let構文-)、上: [SRFI サポート モジュール](07_05_00_srfi_support_modules.md#75-srfi-サポート-モジュール) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
