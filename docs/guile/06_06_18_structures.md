#### 6.6.18 構造体 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Structures-1)

_structure_ は、0 から始まる番号のフィールドに Scheme 値または C ワードを保持する第一級データ型です。_vtable_ は、構造体型を表す構造体であり、フィールド型と権限、および `write` などのオプションの print 関数を提供します。

構造体はレコードよりも低レベルです（[レコード](https://doc.guix.gnu.org/guile/latest/en/guile.html#Records)を参照）。通常、構造化データを表現する必要がある場合は、レコードを使用します。しかし、新しい種類の構造化データ抽象化を実装する必要がある場合もあり、その目的には構造体が役立ちます。実際、Guile のレコードは構造体を使用して実装されています。

* [Vtables](https://doc.guix.gnu.org/guile/latest/en/guile.html#Vtables)
* [構造体の基本](https://doc.guix.gnu.org/guile/latest/en/guile.html#Structure-Basics)
* [Vtable の内容](https://doc.guix.gnu.org/guile/latest/en/guile.html#Vtable-Contents)
* [Meta-Vtables](https://doc.guix.gnu.org/guile/latest/en/guile.html#Meta_002dVtables)
* [Vtable の例](https://doc.guix.gnu.org/guile/latest/en/guile.html#Vtable-Example)

* * *

次へ: [構造の基本](https://doc.guix.gnu.org/guile/latest/en/guile.html#Structure-Basics)、上へ: [構造](https://doc.guix.gnu.org/guile/latest/en/guile.html#Structures) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.18.1 Vtables [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Vtables-1)

vtableは構造体型であり、そのレイアウトやその他の情報を指定します。vtable自体も実際には構造体ですが、最初はそれを気にする必要はありません（[Vtable Contents](https://doc.guix.gnu.org/guile/latest/en/guile.html#Vtable-Contents)を参照）。

Scheme手順: **make-vtable** フィールド \[print\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dvtable)

新しいvtableを作成します。

fields は、作成する構造体内のフィールドを記述する文字列です。各フィールドは、タイプ文字とパーミッション文字の 2 文字で表されます。たとえば、「pw」のようになります。タイプは次のとおりです。

* `p` – Scheme の値。「p」は「protected」の略で、ガベージコレクションから保護されていることを意味します。
* `u` – 任意のデータワード（`scm_t_bits`）。Scheme レベルでは、符号なし整数として読み書きされます。「u」は「unboxed」の略で、追加の型注釈なしで生の値として格納されます。

以前は、各フィールドの2文字目がパーミッションコードで、例えば書き込み可能を表す「w」や読み取り専用を表す「r」などでした。しかし、構造体は時間の経過とともに、より低レベルの生データ処理ツールへと進化し、アクセス制御は上位レイヤーとして実装する方が適切になりました。実際、「struct-set!」は上位レベルのレコード処理ツールによる抽象化をバイパスできる横断的な演算子であり、フィールドが書き込み可能であっても、「信頼できない」コードに「struct-set!」を公開することは、一般的に（抽象化を維持するという意味で）安全ではありません。さらに、パーミッションチェックは、最適化によって削減できないオーバーヘッドを構造体へのアクセスごとに追加し、構造体が低レベルの構成要素として機能する能力を阻害していました。これらの理由から、Guile構造体のすべてのフィールドは書き込み可能になりました。読み取り専用フィールドを作成しようとすると、非推奨の警告が表示され、フィールドは書き込み可能になります。

(make-vtable "pw") ;; 1つのスキームフィールド
(make-vtable "pwuwuw") ;; 1つのスキームと2つのアンボックスフィールド

オプションの print 引数は、`display` や `write` (など) によって呼び出される関数で、この vtable から作成された構造体の印刷表現を提供します。この関数は `(print struct port)` と呼ばれ、struct を参照して port に書き込みます。デフォルトの print では、マシン アドレスのペアを含む '#<struct ADDR:ADDR>' のような形式が出力されます。

例えば、以下のprint関数は、その構造体の2つのフィールドを表示します。

(make-vtable "pwpw"
(ラムダ式 (struct port)
(フォーマットポート "#<~a と ~a>")
(構造体参照構造体 0)
(構造体参照構造体 1))))

* * *

次へ: [Vtable Contents](https://doc.guix.gnu.org/guile/latest/en/guile.html#Vtable-Contents)、前: [Vtables](https://doc.guix.gnu.org/guile/latest/en/guile.html#Vtables)、上: [Structures](https://doc.guix.gnu.org/guile/latest/en/guile.html#Structures) \[[Contents](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[Index](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.18.2 構造体の基本 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Structure-Basics-1)

このセクションでは、構造体を操作するための基本的な手順について説明します。`make-struct/no-tail` は構造体を作成し、`struct-ref` と `struct-set!` はそのフィールドにアクセスします。

Scheme手順: **make-struct/no-tail** vtable init … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dstruct_002fno_002dtail)

指定されたvtableに従ってレイアウトされた新しい構造を作成します（[Vtables](https://doc.guix.gnu.org/guile/latest/en/guile.html#Vtables)を参照）。

オプションの init… 引数は、構造体のフィールドの初期値です。読み取り専用フィールドに値を設定するには、この方法しかありません。init 引数の数がフィールドの数より少ない場合、デフォルト値は Scheme フィールド (型 `p`) の場合は `#f`、ボックス化されていないフィールド (型 `u`) の場合は 0 になります。

名前が少し奇妙であることは認めます。その理由は、Guileには以前、追加の引数を取る`make-struct`という関数があったためです。この古いインターフェースは非推奨となり、この機能の新しい名前は`make-struct/no-tail`となりました。

例えば、

(define v (make-vtable "pwpwpw"))
(define s (make-struct/no-tail v 123 "abc" 456))
(struct-ref s 0) ⇒ 123
(struct-ref s 1) ⇒ "abc"

C 関数: `SCM` **scm\_make\_struct** `(SCM vtable, SCM tail_size, SCM init_list)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fmake_005fstruct)

C 関数: `SCM` **scm\_c\_make\_struct** `(SCM vtable、SCM tail_size、SCM init、...)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fmake_005fstruct)

C 関数: `SCM` **scm\_c\_make\_structv** `(SCM vtable, SCM tail_size, size_t n_inits, scm_t_bits init[])` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fmake_005fstructv)

C言語で構造体を作成する方法はいくつかあります。`scm_make_struct`はリストを受け取り、`scm_c_make_struct`はSCM_UNDEFINEDで終端された可変引数を受け取り、`scm_c_make_structv`はパックされた配列を受け取ります。

これらすべてにおいて、tail_size はゼロである必要があります (SCM 値として)。

Scheme手順: **struct?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-struct_003f)

C 関数: **scm\_struct\_p** (obj) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstruct_005fp)

objが構造体の場合は`#t`を返し、そうでない場合は`#f`を返します。

Scheme手順: **struct-ref** struct n [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-struct_002dref)

C 関数: **scm\_struct\_ref** (struct, n) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstruct_005fref)

構造体内のn番目のフィールドの内容を返します。最初のフィールドは0番目のフィールドです。

nが範囲外の場合はエラーが発生します。

Scheme手順: **struct-set!** struct n value [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-struct_002dset_0021)

C 関数: **scm\_struct\_set\_x** (struct, n, value) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstruct_005fset_005fx)

構造体内のフィールド番号nに値を設定します。最初のフィールドは番号0です。

nが範囲外の場合、またはフィールドが読み取り専用（r）のため書き込みができない場合は、エラーが発生します。

ボックス化されていないフィールド（型が「u」のフィールド）には、特別な手順でアクセスする必要があります。

Scheme手順: **struct-ref/unboxed** struct n [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-struct_002dref_002funboxed)

Scheme手順: **struct-set!/unboxed** struct n value [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-struct_002dset_0021_002funboxed)

C 関数: **scm\_struct\_ref\_unboxed** (struct, n) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstruct_005fref_005funboxed)

C 関数: **scm\_struct\_set\_x\_unboxed** (struct, n, value) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstruct_005fset_005fx_005funboxed)

`struct-ref` や `struct-set!` と同様ですが、これらはボックス化されていないフィールドにのみ使用できます。`struct-ref/unboxed` は常に正の整数を返します。同様に、`struct-set!/unboxed` は値引数として符号なし整数を受け取り、それ以外の場合はエラーを通知します。

Scheme手順: **struct-vtable** struct [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-struct_002dvtable)

C 関数: **scm\_struct\_vtable** (struct) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstruct_005fvtable)

構造体を記述するvtableを返します。

vtableは実質的に構造体の型です。vtableの詳細については、[Vtable Contents](https://doc.guix.gnu.org/guile/latest/en/guile.html#Vtable-Contents)を参照してください。

* * *

次へ: [Meta-Vtables](https://doc.guix.gnu.org/guile/latest/en/guile.html#Meta_002dVtables)、前: [Structure Basics](https://doc.guix.gnu.org/guile/latest/en/guile.html#Structure-Basics)、上: [Structures](https://doc.guix.gnu.org/guile/latest/en/guile.html#Structures) \[[Contents](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[Index](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "Index")\]

#### 6.6.18.3 Vtable の内容[¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Vtable-Contents-1)

vtable自体が構造体です。vtableには、そのインスタンス（vtableから作成される構造体）のさまざまな側面を記述する特定のフィールドセットがあります。これらのフィールドの中にはGuile内部のものと、パブリックインターフェースの一部となっているものがあり、ユーザーが追加できるフィールドもあります。

各vtableには、インスタンスのレイアウトを表すフィールド、インスタンスの印刷に使用されるプロシージャを表すフィールド、およびvtable自体の名前を表すフィールドがあります。レイアウトとプリンタへのアクセスは、フィールドインデックスを介して直接行われます。vtable名へのアクセスは、アクセサプロシージャを介して行われます。

Scheme変数: **vtable-index-layout** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-vtable_002dindex_002dlayout)

C マクロ: **scm\_vtable\_index\_layout** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fvtable_005findex_005flayout)

vtable 内のレイアウト仕様のフィールド番号。レイアウト仕様は、`make-vtable` に渡されるフィールド文字列から形成される `pwpw` のようなシンボル、または `make-struct-layout` によって作成されるシンボルです ([Meta-Vtables](https://doc.guix.gnu.org/guile/latest/en/guile.html#Meta_002dVtables) を参照)。

(define v (make-vtable "pwpw" 0))
(struct-ref v vtable-index-layout) ⇒ pwpw

vtableを使用する構造体のレイアウトは変更できないため、このフィールドは読み取り専用です。

Scheme変数: **vtable-index-printer** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-vtable_002dindex_002dprinter)

C マクロ: **scm\_vtable\_index\_printer** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fvtable_005findex_005fprinter )

プリンタ機能のフィールド番号。デフォルトの印刷機能を使用する場合は、このフィールドに「#f」を指定します。

(define (my-print-func struct port)
...)
(define v (make-vtable "pwpw" my-print-func))
(struct-ref v vtable-index-printer) ⇒ my-print-func

このフィールドは書き込み可能であり、印刷機能を動的に変更できます。

Scheme手順: **struct-vtable-name** vtable [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-struct_002dvtable_002dname)

Scheme手順: **set-struct-vtable-name!** vtable名 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-set_002dstruct_002dvtable_002dname_0021)

C 関数: **scm\_struct\_vtable\_name** (vtable) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstruct_005fvtable_005fname)

C 関数: **scm\_set\_struct\_vtable\_name\_x** (vtable, name) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fset_005fstruct_005fvtable_005fname_005fx)

vtable の名前を取得または設定します。name はシンボルであり、vtable から作成された構造体を印刷する際のデフォルトの印刷関数で使用されます。

(define v (make-vtable "pw"))
(set-struct-vtable-name! v 'my-name')

(define s (make-struct v 0))
(表示s) ⊣ #<my-name b7ab3ae0:b7ab3730>

* * *

次へ: [Vtable の例](https://doc.guix.gnu.org/guile/latest/en/guile.html#Vtable-Example)、前: [Vtable の内容](https://doc.guix.gnu.org/guile/latest/en/guile.html#Vtable-Contents)、上: [構造](https://doc.guix.gnu.org/guile/latest/en/guile.html#Structures) \[[内容](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.18.4 Meta-Vtables [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Meta_002dVtables-1)

構造体であるvtable自体もvtableを持ち、vtableもまた構造体です。構造体、そのvtable、vtableのvtableなどによって構造体のツリーが形成されます。新しい構造体を作成すると、ツリーに葉が追加されます。その構造体がvtableである場合、その構造体を使用して他の葉を作成することができます。

`struct-vtable` を呼び出して vtable ツリーを辿っていくと、最終的にルートに到達します。ルートは、それ自身の vtable です。

scheme@(guile-user)> (current-module)
$1 = #<ディレクトリ (guile-user) 221b090>
scheme@(guile-user)> (struct-vtable $1)
$2 = #<レコードタイプモジュール>
scheme@(guile-user)> (struct-vtable $2)
$3 = #<<standard-vtable> 12c30a0>
scheme@(guile-user)> (struct-vtable $3)
$4 = #<<standard-vtable> 12c3fa0>
scheme@(guile-user)> (struct-vtable $4)
$5 = #<<standard-vtable> 12c3fa0>
scheme@(guile-user)> <standard-vtable>
$6 = #<<standard-vtable> 12c3fa0>

この例では、`$1` は `$2` のインスタンスであり、`$2` は `$3` のインスタンスであり、`$3` は `$4` のインスタンスであり、そして不思議なことに `$4` はそれ自身のインスタンスであると言えます。このコンソール セッションで `$4` にバインドされている値は、デフォルト 環境の `<standard-vtable>` にもバインドされています。

スキーム変数: **<standard-vtable>** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003cstandard_002dvtable_003e)

新しいvtableを作成する際に便利なメタvtable。

これらの値はすべて構造体です。`$1` を除くすべてが vtable です。`$2` は `$3` のインスタンスであり、`$3` は vtable であるため、`$3` は _meta-vtable_、つまり vtable を作成できる vtable であると言えます。

この定義により、vtable が何であるかをより正確に指定できます。vtable は、メタ vtable から作成される構造体です。メタ vtable から構造体を作成する際には、構造体の最初のフィールドが有効なレイアウトであることを確認するための特別なチェックが実行されます。さらに、これらのチェックによって、子 vtable のレイアウトに vtable に必要なすべてのフィールドが正しい順序で含まれていることが確認された場合、子 vtable もメタ テーブルとなり、親から特別なビットを継承します。

Scheme手順: **struct-vtable?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-struct_002dvtable_003f)

C 関数: **scm\_struct\_vtable\_p** (obj) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstruct_005fvtable_005fp)

objがvtable構造体（メタvtableのインスタンス）である場合は、`#t`を返します。

`<standard-vtable>`はvtableツリーのルートです。（通常、Guileプロセスにはルートは1つしかありませんが、一部のレガシーインターフェースの関係で複数存在する場合があります。）

vtable の必須フィールドのセットは、`<standard-vtable>` のフィールドのセットであり、デフォルト環境では `standard-vtable-fields` にバインドされています。レイアウトにフィールドを追加したメタ vtable を作成することも可能で、これを使用して追加データを含む vtable を作成できます。

scheme@(guile-user)> (struct-ref $3 vtable-index-layout)
$6 = pwuhuhpwphuhuhpwpwpw
scheme@(guile-user)> (struct-ref $4 vtable-index-layout)
$7 = pwuhuhpwphuhuh
scheme@(guile-user)> standard-vtable-fields
8ドル＝「プワフワフワフ」
scheme@(guile-user)> (struct-ref $2 vtable-offset-user)
$9 = モジュール

先の例の続きとして、`$2` は追加フィールドを持つ vtable です。これは、その vtable である `$3` が、拡張レイアウトを持つメタ vtable から作成されたためです。`vtable-offset-user` は、`standard-vtable-fields` のフィールド数を示す便利な定義です。

スキーム変数: **standard-vtable-fields** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-standard_002dvtable_002dfields)

vtableが持つべきフィールドの順序付きセットを含む文字列。

スキーム変数: **vtable-offset-user** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-vtable_002doffset_002duser)

ユーザーが利用できるvtable内の最初のインデックス。

Scheme手順: **make-struct-layout**フィールド[¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dstruct_002dlayout)

C 関数: **scm\_make\_struct\_layout** (フィールド) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fmake_005fstruct_005flayout)

フィールド文字列から構造体レイアウトシンボルを返します。フィールドは`make-vtable`で説明されているとおりです（[Vtables](https://doc.guix.gnu.org/guile/latest/en/guile.html#Vtables)を参照）。無効なフィールド文字列はエラーとなります。

これらの定義を用いると、`make-vtable`は次のように定義できる。

(define\* (make-vtable fields #:optional printer)
(make-struct/no-tail <standard-vtable>
(make-struct-layout フィールド)
プリンター))

* * *

前へ: [Meta-Vtables](https://doc.guix.gnu.org/guile/latest/en/guile.html#Meta_002dVtables)、上へ: [Structures](https://doc.guix.gnu.org/guile/latest/en/guile.html#Structures) \[[Contents](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[Index](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.18.5 Vtable の例 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Vtable-Example-1)

これらの点を例を用いてまとめてみましょう。単一継承のシンプルなオブジェクトシステムを考えます。オブジェクトは通常の構造体であり、クラスは、クラス名、親クラス、フィールドのリストという3つの追加クラスフィールドを持つvtableになります。

そこでまず、これらの追加クラスフィールドを持つインスタンスを割り当てるメタ仮想テーブルが必要になります。

(define <class>
(make-vtable
(string-append standard-vtable-fields "pwpwpw")
(ラムダ (x ポート)
(フォーマットポート "<<class> ~a>" (クラス名 x)))))

(define (class? x)
(そして (struct? x)
(eq? (struct-vtable x) <class>)))

特定のメタvtableを持つ構造体を作成するには、`make-struct/no-tail`を使用し、`make-vtable`と同様に計算されたインスタンスレイアウトとプリンタを渡します。さらに、3つの追加のクラスフィールドも渡します。

(定義 (make-class name 親フィールド)
(let\* ((fields (compute-fields parent fields))
(レイアウト (compute-layout フィールド)))
(make-struct/no-tail <class>
レイアウト
(ラムダ (x ポート)
(print-instance x port)
名前
親
フィールド)))

インスタンスは、構造体内のスロットに関連データを格納します。スロットの数はフィールドの数と同じです。以下の `compute-layout` プロシージャはレイアウトを計算し、`field-index` はフィールドに対応するスロットを返します。

(define-syntax-rule (define-accessor name n)
(define (name obj)
(構造体参照オブジェクト n)))

;; クラス用アクセサ
(define-accessor class-name (+ vtable-offset-user 0))
(define-accessor class-parent (+ vtable-offset-user 1))
(define-accessor class-fields (+ vtable-offset-user 2))

(define (compute-fields parent fields)
(親の場合)
(append (class-fields parent) fields)
フィールド))

(define (compute-layout fields)
(make-struct-layout
(文字列連結 (make-list (length fields) "pw"))))

(define (field-index class field)
(リストインデックス (クラスフィールド クラス) フィールド)

(define (print-instance x port)
(フォーマットポート "<~a" (クラス名 (struct-vtable x)))
(for-each (lambda (field idx)
(フォーマットポート " ~a: ~a" フィールド (構造体参照 x idx)))
(クラスフィールド (構造体vtable x))
(iota (length (class-fields (struct-vtable x)))))
(フォーマットポート ">"))

では、この時点で実際にいくつかのクラスを作成してみましょう。

(define-syntax-rule (define-class name parent field ...)
(define name (make-class 'name parent '(field ...))))

(define-class <surface> #f
幅 高さ)

(define-class <window> <surface>
xy)

最後に、インスタンスを作成します。

(make-struct/no-tail <window> 400 300 10 20)
⇒ <<window> 幅: 400 高さ: 300 x: 10 y: 20>

以上です。このオブジェクトシステムには多くの最適化や機能強化が可能であり、付属のGOOPSシステムはそのほとんどを既に実装しています。よりシンプルなユースケースでは、レコード機能で十分でしょう。しかし、時には新しい種類のデータ抽象化が必要になる場合があり、そのために構造体が用意されています。

* * *

次へ: [関連リスト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Association-Lists )、前: [構造体](https://doc.guix.gnu.org/guile/latest/en/guile.html#Structures)、上: [データ型](https://doc.guix.gnu.org/guile/latest/en/guile.html#Data-Types) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
