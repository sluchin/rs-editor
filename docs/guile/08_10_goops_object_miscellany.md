### 8.10 GOOPS オブジェクト雑記 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#GOOPS-Object-Miscellany-1)

ここでは、GOOPSオブジェクトに関するいくつかのポイントについて説明します。ただし、それらは個別のセクションを設けるほど重要な内容ではありません。

#### オブジェクト等価性 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Object-Equality)

GOOPSがロードされると、`eqv?`、`equal?`、`=`は汎用関数となり、独自のクラスに特化したメソッドを定義することで、さまざまな種類の等価性がクラスにとって何を意味するかを制御できるようになります。

例えば、alist 内のエントリを検索するための `assoc` プロシージャは、alist 内のエントリの car が `assoc` 呼び出し時に渡されるキー パラメータと同じかどうかを判断するために `equal?` を使用するように指定されています。したがって、新しいクラスを定義し、そのクラスのインスタンスを alist のキーとして使用したい場合は、そのクラスに `equal?` メソッドを定義することで、`assoc` の検索を正確に制御できます。

#### オブジェクトのクローン作成 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Cloning-Objects)

汎用: **shallow-clone** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-shallow_002dclone)

メソッド: **shallow-clone** (self <object>) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-shallow_002dclone-1)

self の「浅い」クローンを返します。デフォルトのメソッドでは、新しいインスタンスを割り当て、self から新しいインスタンスにスロット値をコピーすることで浅いクローンを作成します。各スロット値は、即値として、または参照によってコピーされます。

汎用: **deep-clone** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-deep_002dclone)

メソッド: **deep-clone** (self <object>) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-deep_002dclone-1)

self の「ディープ」クローンを返します。デフォルトのメソッドでは、新しいインスタンスを割り当て、self からスロット値を新しいインスタンスにコピーまたはクローンすることで、ディープクローンを作成します。スロット値がインスタンスである場合 (`instance?` を満たす場合)、その値に対して `deep-clone` を呼び出すことでクローンが作成されます。その他のスロット値は、即値として、または参照によってコピーされます。

#### 書き込みと表示 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Write-and-Display)

プリミティブジェネリック: **write** オブジェクトポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-write-3)

プリミティブジェネリック: **display** オブジェクトポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display-3)

GOOPSがロードされると、`write`と`display`は、印刷用の特別なメソッドを持つ汎用関数になります。

* オブジェクト - クラス `<object>` のインスタンス
* 外部オブジェクト - `<foreign-object>` クラスのインスタンス
* クラス - クラス `<class>` のインスタンス
* 汎用関数 - クラス `<generic>` のインスタンス
* メソッド - クラス `<method>` のインスタンス。

`write`と`display`は、Guileのプリミティブ関数である`write`と`display`と同様に、GOOPS以外の値を出力します。

上記以外にも、もちろん独自のクラスに対して`write`メソッドと`display`メソッドを定義することで、それらのクラスのインスタンスがどのように出力されるかをカスタマイズできます。

* * *

次へ: [クラスの再定義](https://doc.guix.gnu.org/guile/latest/en/guile.html#Redefining-a-Class)、前: [GOOPS オブジェクト雑録](https://doc.guix.gnu.org/guile/latest/en/guile.html#GOOPS-Object-Miscellany)、上: [GOOPS](https://doc.guix.gnu.org/guile/latest/en/guile.html#GOOPS) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
