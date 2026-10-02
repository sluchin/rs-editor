8 GOOPS [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#GOOPS-1)
------------------------------------------------------------------------

GOOPSは「Guileオブジェクト指向プログラミングシステム」の略です。その実装は、Erick Gallesio氏によるSTk-3.99.3と、Gregor Kiczales氏によるTiny-Closのバージョン1.3を基にしています。Common LispオブジェクトシステムであるCLOSと精神的に非常に近いものですが、Scheme言語向けに改良されています。

GOOPSは、クラス、オブジェクト、多重継承、マルチメソッドディスパッチを備えた汎用関数など、完全なオブジェクト指向システムです。さらに、その実装はメタオブジェクトプロトコルに依存しており、GOOPSの中核となる操作自体が関連クラスのメソッドとして定義され、それらのメソッドをオーバーライドまたは再定義することでカスタマイズできます。

GOOPSを使い始めるには、まず`(oop goops)`モジュールをインポートする必要があります。これはGuile REPLで以下のように評価することで行えます。

([use-modules](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-use_002dmodules) (oop goops))

* [著作権表示](https://doc.guix.gnu.org/guile/latest/en/guile.html#Copyright-Notice)
* [クラス定義](https://doc.guix.gnu.org/guile/latest/en/guile.html#Class-Definition)
* [インスタンスの作成とスロットへのアクセス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instance-Creation)
* [スロットオプション](https://doc.guix.gnu.org/guile/latest/en/guile.html#Slot-Options)
* [スロットの説明例](https://doc.guix.gnu.org/guile/latest/en/guile.html#Slot-Description-Example)
* [メソッドと汎用関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Methods-and-Generic-Functions)
* [継承](https://doc.guix.gnu.org/guile/latest/en/guile.html#Inheritance)
* [イントロスペクション](https://doc.guix.gnu.org/guile/latest/en/guile.html#Introspection)
* [エラー処理](https://doc.guix.gnu.org/guile/latest/en/guile.html#GOOPS-Error-Handling)
* [GOOPS オブジェクト雑録](https://doc.guix.gnu.org/guile/latest/en/guile.html#GOOPS-Object-Miscellany)
* [メタオブジェクトプロトコル](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Metaobject-Protocol)
* [クラスの再定義](https://doc.guix.gnu.org/guile/latest/en/guile.html#Redefining-a-Class)
* [インスタンスのクラスの変更](https://doc.guix.gnu.org/guile/latest/en/guile.html#Changing-the-Class-of-an-Instance )

* * *

次へ: [クラス定義](https://doc.guix.gnu.org/guile/latest/en/guile.html#Class-Definition)、上へ: [GOOPS](https://doc.guix.gnu.org/guile/latest/en/guile.html#GOOPS) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
