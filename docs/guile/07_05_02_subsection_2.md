#### 7.5.2 SRFI-0 - cond-expand [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d0-_002d-cond_002dexpand )

このSRFIを使用すると、移植可能なSchemeプログラムが特定の機能の有無をテストし、異なるコードブロックを使用して自身を適応させるか、必要な機能が利用できない場合はエラーを発生させることができます。ロードするモジュールはなく、これはGuileコアに組み込まれています。

Guile専用に設計されたプログラムであれば、通常はこの仕組みは必要なく、もちろんGuileの各種ドキュメント化された部分を直接利用できます。

構文: **cond-expand** (機能本体…) …

機能仕様が満たされている最初の節の本体まで展開します。どの機能も満たされていない場合はエラーとなります。

機能は`srfi-1`などのシンボルで表され、機能仕様では`and`、`or`、`not`の形式を使用して組み合わせをテストできます。最後の句は`else`で、他の条件が満たされない場合に使用されます。

例えば、SRFI-1が利用できない場合は、`alist-cons`のプライベートバージョンを定義します。

(cond-expand (srfi-1
）
（それ以外
(定義 (alist-cons key val alist)
(cons (cons key val) alist))))

または、特定の一連の SRFI (リスト操作、文字列ポート、`receive` および文字列操作) を要求し、それらが利用できない場合は失敗します。

(cond-expand ((and srfi-1 srfi-6 srfi-8 srfi-13)
))

Guileコアには以下の機能があります。

狡猾さ
guile-2 ;; Guile 2.x 以降
guile-2.2 ;; Guile 2.2 から開始
guile-3 ;; Guile 3.x 以降
guile-3.0 ;; Guile 3.0 から開始
r5rs
r6rs
r7rs
正確なクローズド ieee-float フル Unicode 比率 ;; R7RS 機能
srfi-0
srfi-4
srfi-6
srfi-13
srfi-14
srfi-16
srfi-23
srfi-30
srfi-39
srfi-46
srfi-55
srfi-61
srfi-62
srfi-87
srfi-105

その他のSRFI機能シンボルは、`use-modules`でコードがロードされた後に定義されます。なぜなら、その時点で初めてバインディングが利用可能になるからです。

'\--use-srfi' コマンドラインオプション ([Guile の呼び出し](04_programming_in_scheme.md#42-guile-の呼び出し) を参照) は、移植可能なプログラムを実行する際に `cond-expand` を満たす SRFI をロードする良い方法です。

`guile`機能をテストすることで、プログラムはGuileモジュールシステムに適応しつつ、他のSchemeシステムでも動作させることができます。例えば、以下のコードはSRFI-8（`receive`）を必要としますが、Guileメカニズムを使ってそれをロードする方法も知っています。

(cond-expand (srfi-8
）
（策略）
(use-modules (srfi srfi-8))))

同様に、`guile-2` 機能をテストすることで、Guile 2.x と以前のバージョンの Guile 間でコードの移植性を確保できます。例えば、Guile 2.x のコンパイラに対応したコードを記述しても、1.8 以前のバージョンで正しく解釈されるようになります。

(cond-expand (guile-2 (eval-when (compile)
;; これはコンパイル時に評価する必要があります。
(fluid-set! current-reader my-reader)))
（策略）
;; 以前のバージョンのGuileには
;; コンパイルフェーズを分離します。
(fluid-set! current-reader my-reader)))

`cond-expand` は `*features*` メカニズムとは別物であることに注意してください ([Feature Tracking](06_23_configuration_features_and_runtime_options.md#6232-機能追跡) を参照)。一方の機能シンボルは他方の機能シンボルとは無関係です。

* * *

次へ: [SRFI-2 - and-let\*](07_05_04_srfi2_andlet.md#754-srfi-2---and-let)、前: [SRFI-0 - cond-expand](#752-srfi-0---cond-expand-)、上: [SRFI サポート モジュール](07_05_00_srfi_support_modules.md#75-srfi-サポート-モジュール) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
