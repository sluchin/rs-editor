### 8.2 クラス定義 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Class-Definition-1)

新しいクラスは、`define-class`構文を使用して定義されます。

(define-class [class](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-class-1) (superclass [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e))
スロットの説明 [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e)
クラスオプション [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e))

クラスは定義されるクラスです。スーパークラスのリストは、スロットとプロパティを継承する既存のクラス（存在する場合）を指定します。_スロット_は、そのクラスのインスタンスごとにデータを保持します[30](https://doc.guix.gnu.org/guile/latest/en/guile.html#FOOT30)。これは、他のオブジェクト指向システムにおける「フィールド」や「メンバ変数」のようなものです。各スロットの説明には、スロットの名前と、オプションでそのスロットの「プロパティ」がいくつか指定されます。たとえば、初期値、その値にアクセスする関数の名前などです。クラスのオプション、スロットの説明、および継承については、以下で詳しく説明します。

構文: **define-class** 名前 (スーパークラス …) スロット定義 … クラスオプション … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-define_002dclass-1)

`name` という名前のクラスを定義します。このクラスは `super` を継承し、`slot-definitions` と `class-options` によって直接スロットを定義します。新しく作成されたクラスは、現在の環境の変数 `name` にバインドされます。

各スロット定義は、スロットの名前を示すシンボルまたはリストのいずれかです。

(スロット名シンボル . スロットオプション)

ここで、slot-name-symbolはシンボルであり、slot-optionsは偶数個の要素を持つリストです。slot-optionsの偶数番目の要素（0から数えて）はスロットオプションのキーワードであり、奇数番目の要素はそれらのキーワードに対応する値です。

各クラスオプションは、オプションキーワードとそれに対応する値で構成されます。

例として、複素数を2つの実数で表現する型を定義してみましょう。[31](https://doc.guix.gnu.org/guile/latest/en/guile.html#FOOT31) これは、次のクラス定義で実現できます。

(define-class <my-complex> (<number>)
ri)

これにより、変数 `<my-complex>` が、インスタンスに 2 つのスロットが含まれる新しいクラスにバインドされます。これらのスロットは `r` と `i` と呼ばれ、複素数の実部と虚部を保持します。このクラスは、定義済みのクラスである `<number>` を継承していることに注意してください。[32](https://doc.guix.gnu.org/guile/latest/en/guile.html#FOOT32)

スロットのオプションについては、次のセクションで説明します。選択可能なクラスオプションは以下のとおりです。

クラスオプション: **#:metaclass** メタクラス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0023_003ametaclass)

`#:metaclass` クラスオプションは、定義するクラスのメタクラスを指定します。メタクラスは、`<class>` を継承するクラスである必要があります。メタクラスの使用方法については、[メタオブジェクトとメタオブジェクトプロトコル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Metaobjects-and-the-Metaobject-Protocol) および [メタクラス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Metaclasses) を参照してください。

`#:metaclass` オプションがない場合、GOOPS は `ensure-metaclass` を呼び出すことで新しいクラスのメタクラスを再利用または構築します ([ensure-metaclass](https://doc.guix.gnu.org/guile/latest/en/guile.html#Class-Definition-Protocol) を参照)。

クラスオプション: **#:name** name [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0023_003aname)

`#:name`クラスオプションは、新しいクラスの名前を指定します。この名前は、関連するオブジェクト（クラス自体、そのインスタンス、およびそのサブクラス）が出力される際に、クラスを識別するために使用されます。

`#:name` オプションが指定されていない場合、GOOPS は `define-class` の最初の引数をクラス名として使用します。

* * *

次へ: [スロットオプション](https://doc.guix.gnu.org/guile/latest/en/guile.html#Slot-Options)、前: [クラス定義](https://doc.guix.gnu.org/guile/latest/en/guile.html#Class-Definition)、上: [GOOPS](https://doc.guix.gnu.org/guile/latest/en/guile.html#GOOPS) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
