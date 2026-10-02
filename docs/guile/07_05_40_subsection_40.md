#### 7.5.40 SRFI-71 - 複数値のための拡張let構文 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d71-_002d-Extended-let_002dsyntax-for-multiple-values )

このSRFIは、`let`、`let*`、および`letrec`の形式をシャドウイングし、複数の値を受け入れることができるようにします。例：

(use-modules (srfi srfi-71))

(let\* ((xy (values 1 2))
(z (+ xy)))
(* z 2))
⇒ 6

[SRFI-71の仕様書](http://srfi.schemers.org/srfi-71/srfi-71.html)を参照してください。

* * *

次へ: [SRFI-88 キーワードオブジェクト](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d88)、前: [SRFI-71 - 複数値のための拡張 let 構文](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d71)、上: [SRFI サポート モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI-Support) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
