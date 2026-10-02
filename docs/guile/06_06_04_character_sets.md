#### 6.6.4 文字セット

本節で説明する機能は、SRFI-14に直接対応するものです。

データ型 _charset_ は文字セットを実装します（[文字](06_06_03_characters.md#663-文字)を参照）。文字セットの内部表現はユーザーには見えないため、それらを処理するための多くの手順が提供されています。

文字セットは作成、拡張、文字のメンバーシップのテスト、および他の文字セットとの比較が可能です。

* [文字セット述語/比較](#6641-文字セット述語比較)
* [文字セットの反復処理](#6642-文字セットの反復処理)
* [文字セットの作成](#6643-文字セットの作成)
* [文字セットのクエリ](#6644-文字セットのクエリ)
* [文字セット代数](#6645-文字セット代数)
* [標準文字セット](#6646-標準文字セット)

* * *

次へ: [文字セットの反復処理](#6642-文字セットの反復処理)、上へ: [文字セット](#664-文字セット) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 6.6.4.1 文字セット述語/比較

これらの手順は、オブジェクトが文字セットであるかどうか、または複数の文字セットが互いに等しいか部分集合であるかをテストするために使用します。`char-set-hash` はハッシュ値を計算するために使用でき、高速検索手順などで利用できます。

Scheme手順: **char-set?** obj

C 関数: **scm\_char\_set\_p** (obj)

objが文字セットの場合は`#t`を返し、そうでない場合は`#f`を返します。

Scheme手順: **char-set=** char\_set …

C 関数: **scm\_char\_set\_eq** (char\_sets)

指定された文字セットがすべて等しい場合は、`#t` を返します。

Scheme手順: **char-set<=** char\_set …

C 関数: **scm\_char\_set\_leq** (char\_sets)

すべての文字セット char\_seti が文字セット char\_seti+1 の部分集合である場合は、`#t` を返します。

Scheme Procedure: **char-set-hash** cs \[bound\]

C 関数: **scm\_char\_set\_hash** (cs、bound)

文字セット cs のハッシュ値を計算します。bound が指定され、それがゼロ以外の場合、返される値は 0 … bound - 1 の範囲に制限されます。

* * *

次へ: [文字セットの作成](#6643-文字セットの作成)、前: [文字セット述語/比較](#6641-文字セット述語比較)、上: [文字セット](#664-文字セット) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 6.6.4.2 文字セットの反復処理

文字セットカーソルは、文字セットの要素を順に処理するための手段です。`char-set-cursor`で文字セットカーソルを作成した後、`char-set-ref`でカーソルを逆参照したり、`char-set-cursor-next`で次の要素に進んだりできます。カーソルがセットの最後の要素を通過したかどうかは、`end-of-char-set?`で確認できます。

さらに、文字セットのマッピングおよび（展開）手順も提供されます。

Scheme手順: **char-set-cursor** cs

C 関数: **scm\_char\_set\_cursor** (cs)

カーソルを文字セットcsに戻します。

Scheme手順: **char-set-ref** csカーソル

C 関数: **scm\_char\_set\_ref** (cs, cursor)

文字セットcs内の現在のカーソル位置cursorにある文字を返します。`end-of-char-set?`がtrueを返すカーソルを渡すとエラーになります。

Scheme プロシージャ: **char-set-cursor-next** cs カーソル

C 関数: **scm\_char\_set\_cursor\_next** (cs, cursor)

文字セットカーソルを文字セットcs内の次の文字まで進めます。指定されたカーソルが`end-of-char-set?`条件を満たす場合はエラーとなります。

Scheme手順: **end-of-char-set?**カーソル

C 関数: **scm\_end\_of\_char\_set\_p** (カーソル)

カーソルが文字セットの末尾に達した場合は「#t」を返し、そうでない場合は「#f」を返します。

Scheme Procedure: **char-set-fold** kons knil cs

C 関数: **scm\_char\_set\_fold** (kons, knil, cs)

プロシージャkonsを文字セットcsに折り畳み、knilで初期化します。

Scheme手順: **char-set-unfold** pfg seed \[base\_cs\]

C 関数: **scm\_char\_set\_unfold** (p, f, g, seed, base\_cs)

これは文字セットの基本的な構成要素です。

* g は、初期シードから一連の「シード」値を生成するために使用されます: シード、(g シード)、(g^2 シード)、(g^3 シード)、…
* p は停止するタイミングを示します。つまり、シード値のいずれかに適用したときに true が返されたときです。
* f は各シード値を文字にマッピングします。これらの文字は基本文字セット base\_cs に追加され、結果が生成されます。base\_cs のデフォルト値は空セットです。

Scheme手順: **char-set-unfold!** pfg seed base\_cs

C 関数: **scm\_char\_set\_unfold\_x** (p, f, g, seed, base\_cs)

これは文字セットの基本的な構成要素です。

* g は、初期シードから一連の「シード」値を生成するために使用されます: シード、(g シード)、(g^2 シード)、(g^3 シード)、…
* p は停止するタイミングを示します。つまり、シード値のいずれかに適用したときに true が返されたときです。
* f は各シード値を文字にマッピングします。これらの文字は基本文字セット base\_cs に追加され、結果が生成されます。base\_cs のデフォルト値は空セットです。

Scheme プロシージャ: **char-set-for-each** proc cs

C 関数: **scm\_char\_set\_for\_each** (proc, cs)

文字セットcs内のすべての文字にprocを適用します。戻り値は指定されていません。

Scheme プロシージャ: **char-set-map** proc cs

C 関数: **scm\_char\_set\_map** (proc, cs)

プロシージャ proc を cs 内のすべての文字にマッピングします。proc は文字から文字へのプロシージャである必要があります。

* * *

次へ: [文字セットのクエリ](#6644-文字セットのクエリ)、前: [文字セットの反復処理](#6642-文字セットの反復処理)、上: [文字セット](#664-文字セット) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 6.6.4.3 文字セットの作成

これらの手順によって新しい文字セットが生成されます。

Scheme手順: **char-set-copy** cs

C 関数: **scm\_char\_set\_copy** (cs)

csに含まれるすべての文字を含む、新たに割り当てられた文字セットを返します。

Scheme手順: **char-set** chr …

C 関数: **scm\_char\_set** (chrs)

指定されたすべての文字を含む文字セットを返します。

Scheme手順: **list->char-set** list \[base\_cs\]

C 関数: **scm\_list\_to\_char\_set** (list, base\_cs)

文字リストを文字セットに変換します。文字セット base\_cs が指定されている場合、このセットに含まれる文字も結果に含まれます。

Scheme手順: **list->char-set!** list base\_cs

C 関数: **scm\_list\_to\_char\_set\_x** (list, base\_cs)

文字リストを文字セットに変換します。文字はbase_csに追加され、base_csが返されます。

Scheme手順: **string->char-set** str \[base\_cs\]

C 関数: **scm\_string\_to\_char\_set** (str, base\_cs)

文字列strを文字セットに変換します。文字セットbase_csが指定された場合、そのセットに含まれる文字も結果に含まれます。

Scheme手順: **string->char-set!** str base\_cs

C 関数: **scm\_string\_to\_char\_set\_x** (str, base\_cs)

文字列strを文字セットに変換します。文字列から文字を抽出し、base_csに追加して、base_csを返します。

Scheme 手順: **char-set-filter** pred cs \[base\_cs\]

C 関数: **scm\_char\_set\_filter** (pred, cs, base\_cs)

csに含まれるすべての文字を含む文字セットを返します。この文字セットはpredを満たす必要があります。base\_csの文字が指定されている場合は、結果に追加されます。

Scheme 手順: **char-set-filter!** pred cs base\_cs

C 関数: **scm\_char\_set\_filter\_x** (pred, cs, base\_cs)

csに含まれるすべての文字を含む文字セットを返し、predを満たすようにします。文字はbase_csに追加され、base_csが返されます。

Scheme 手順: **ucs-range->char-set** lower upper \[error \[base\_cs\]\]

C 関数: **scm\_ucs\_range\_to\_char\_set** (lower、upper、error、base\_cs)

文字コードが半開区間 \[lower,upper] 内にあるすべての文字を含む文字セットを返します。

errorが真の値の場合、指定された範囲に実装された文字範囲に含まれていない文字が含まれていると、エラーが通知されます。errorが`#f`の場合、これらの文字は結果の文字セットから黙って除外されます。

base\_cs に含まれる文字が指定されている場合は、結果に追加されます。

Scheme 手順: **ucs-range->char-set!** lower upper error base\_cs

C 関数: **scm\_ucs\_range\_to\_char\_set\_x** (lower、upper、error、base\_cs)

文字コードが半開区間 \[lower,upper] 内にあるすべての文字を含む文字セットを返します。

errorが真の値の場合、指定された範囲に実装された文字範囲に含まれていない文字が含まれていると、エラーが通知されます。errorが`#f`の場合、これらの文字は結果の文字セットから黙って除外されます。

文字はbase_csに追加され、base_csが返されます。

Scheme手順: **\->char-set** x

C 関数: **scm\_to\_char\_set** (x)

x を文字セットに変換します。x は文字列、文字、または文字セットのいずれかです。文字列は構成文字のセットに変換され、文字は単一要素のセットに変換され、文字セットはそのまま返されます。

* * *

次へ: [文字セット代数](#6645-文字セット代数)、前: [文字セットの作成](#6643-文字セットの作成)、上: [文字セット](#664-文字セット) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 6.6.4.4 文字セットのクエリ

これらの手順を使用して、文字セットの要素やその他の情報にアクセスします。

Scheme手順: **%char-set-dump** cs

cs のデバッグ情報を含む関連付けリストを返します。関連付けリストには、次のエントリが含まれます。

`char-set`

文字セット自体

`len`

文字セットに含まれる連続するコードポイントのグループの数

`範囲`

各サブリストがコードポイントの範囲とそれに関連付けられた文字であるリストのリスト

この関数の戻り値は、Guile のバージョン間で一貫性が保証されないため、コード内で使用しないでください。

Scheme手順: **char-set-size** cs

C 関数: **scm\_char\_set\_size** (cs)

文字セットcsに含まれる要素の数を返します。

Scheme Procedure: **char-set-count** pred cs

C 関数: **scm\_char\_set\_count** (pred, cs)

文字セット cs 内の、述語 pred を満たす要素の数を返します。

Scheme手順: **char-set->list** cs

C 関数: **scm\_char\_set\_to\_list** (cs)

文字セット cs の要素を含むリストを返します。

Scheme手順: **char-set->string** cs

C 関数: **scm\_char\_set\_to\_string** (cs)

文字セット cs の要素を含む文字列を返します。文字列内の文字の順序は定義されていません。

Scheme 手順: **char-set-contains?** cs ch

C 関数: **scm\_char\_set\_contains\_p** (cs, ch)

文字chが文字セットcsに含まれている場合は`#t`を返し、そうでない場合は`#f`を返します。

Scheme Procedure: **char-set-every** pred cs

C 関数: **scm\_char\_set\_every** (pred, cs)

文字セット cs 内のすべての文字が述語 pred を満たす場合は、真の値を返します。

Scheme 手順: **char-set-any** pred cs

C 関数: **scm\_char\_set\_any** (pred, cs)

文字セット cs 内のいずれかの文字が述語 pred を満たす場合は、真の値を返します。

* * *

次へ: [標準文字セット](#6646-標準文字セット)、前: [文字セットのクエリ](#6644-文字セットのクエリ)、上: [文字セット](#664-文字セット) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 6.6.4.5 文字セット代数

文字セットは、和集合、補集合、積集合などの一般的な集合代数演算を用いて操作できます。これらの操作はすべて、文字セット引数を変更する副作用のあるバリアントを提供します。

スキーム手順: **char-set-adjoin** cs chr …

C 関数: **scm\_char\_set\_adjoin** (cs, chrs)

すべての文字引数を、文字セットである最初の引数に追加します。

Scheme手順: **char-set-delete** cs chr …

C 関数: **scm\_char\_set\_delete** (cs, chrs)

最初の引数（文字セットである必要があります）から、すべての文字引数を削除します。

スキーム手順: **char-set-adjoin!** cs chr …

C 関数: **scm\_char\_set\_adjoin\_x** (cs, chrs)

すべての文字引数を、文字セットである最初の引数に追加します。

Scheme手順: **char-set-delete!** cs chr …

C 関数: **scm\_char\_set\_delete\_x** (cs, chrs)

最初の引数（文字セットである必要があります）から、すべての文字引数を削除します。

Scheme手順: **char-set-complement** cs

C 関数: **scm\_char\_set\_complement** (cs)

文字セットcsの補数を返します。

文字セットの補集合には、予約済みコードポイント（文字に関連付けられていないコードポイント）が多数含まれる可能性があることに注意してください。`char-set-complement` の出力と指定コードポイントの集合 `char-set:designated` との共通部分を計算することで、出力結果を修正すると役立つ場合があります。

Scheme手順: **char-set-union** cs …

C 関数: **scm\_char\_set\_union** (char\_sets)

引数として渡されたすべての文字セットの和集合を返します。

Scheme手順: **char-set-intersection** cs …

C 関数: **scm\_char\_set\_intersection** (char\_sets)

引数として指定されたすべての文字セットの共通部分を返します。

Scheme手順: **char-set-difference** cs1 cs …

C 関数: **scm\_char\_set\_difference** (cs1, char\_sets)

引数として指定されたすべての文字セットの差を返します。

Scheme手順: **char-set-xor** cs …

C 関数: **scm\_char\_set\_xor** (char\_sets)

引数として指定されたすべての文字セットの排他的論理和を返します。

Scheme手順: **char-set-diff+intersection** cs1 cs …

C 関数: **scm\_char\_set\_diff\_plus\_intersection** (cs1, char\_sets)

引数として指定されたすべての文字セットの差と共通部分を返します。

Scheme 手順: **char-set-complement!** cs

C 関数: **scm\_char\_set\_complement\_x** (cs)

文字セットcsの補数を返します。

Scheme プロシージャ: **char-set-union!** cs1 cs …

C 関数: **scm\_char\_set\_union\_x** (cs1, char\_sets)

引数として渡されたすべての文字セットの和集合を返します。

Scheme手順: **char-set-intersection!** cs1 cs …

C 関数: **scm\_char\_set\_intersection\_x** (cs1, char\_sets)

引数として指定されたすべての文字セットの共通部分を返します。

Scheme手順: **文字セットの差分!** cs1 cs …

C 関数: **scm\_char\_set\_difference\_x** (cs1, char\_sets)

引数として指定されたすべての文字セットの差を返します。

Scheme手順: **char-set-xor!** cs1 cs …

C 関数: **scm\_char\_set\_xor\_x** (cs1, char\_sets)

引数として指定されたすべての文字セットの排他的論理和を返します。

Scheme手順: **char-set-diff+intersection!** cs1 cs2 cs …

C 関数: **scm\_char\_set\_diff\_plus\_intersection\_x** (cs1, cs2, char\_sets)

引数として指定されたすべての文字セットの差と共通部分を返します。

* * *

前へ: [文字集合代数](#6645-文字セット代数)、上へ: [文字集合](#664-文字セット) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 6.6.4.6 標準文字セット

文字セットデータ型とプロシージャを有効活用するために、いくつかの事前定義された文字セット変数が存在する。

これらの文字セットはロケールに依存せず、`setlocale` 呼び出しによって再計算されることはありません。Unicode コードポイントの全範囲の文字が含まれています。たとえば、`char-set:letter` には約 10 万文字が含まれています。

Scheme変数: **char-set:lower-case**

C 変数: **scm\_char\_set\_lower\_case** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005flower_005fcase )

すべて小文字。

Scheme変数: **char-set:upper-case**

C 変数: **scm\_char\_set\_upper\_case**

すべて大文字。

Scheme変数: **char-set:title-case**

C 変数: **scm\_char\_set\_title\_case**

大文字の後に小文字が続くように機能する、すべての単一文字。

Scheme変数: **char-set:letter**

C 変数: **scm\_char\_set\_letter**

すべての文字。これには、`char-set:lower-case`、`char-set:upper-case`、`char-set:title-case`、および大文字と小文字の区別がない多くの文字が含まれます。たとえば、中国語や日本語の文字には、通常、大文字と小文字の概念がありません。

スキーム変数: **char-set:digit**

C 変数: **scm\_char\_set\_digit**

すべて数字です。

スキーム変数: **char-set:letter+digit**

C 変数: **scm\_char\_set\_letter\_and\_digit**

`char-set:letter`と`char-set:digit`の和集合。

Scheme変数: **char-set:graphic**

C 変数: **scm\_char\_set\_graphic**

紙にインクを付けるすべての文字。

Scheme変数: **char-set:printing**

C 変数: **scm\_char\_set\_printing**

`char-set:graphic`と`char-set:whitespace`の和集合。

Scheme変数: **char-set:whitespace**

C 変数: **scm\_char\_set\_whitespace**

すべて空白文字。

Scheme変数: **char-set:blank**

C 変数: **scm\_char\_set\_blank**

すべての水平方向の空白文字。特に`#\space`と`#\tab`が含まれます。

スキーム変数: **char-set:iso-control**

C 変数: **scm\_char\_set\_iso\_control**

ISO制御文字は、C0制御文字（U+0000～U+001F）、削除文字（U+007F）、およびC1制御文字（U+0080～U+009F）です。

Scheme変数: **char-set:punctuation**

C 変数: **scm\_char\_set\_punctuation**

`!"#%&'()*,-./:;?@[\\]_{}` などの句読点文字すべて

Scheme変数: **char-set:symbol**

C 変数: **scm\_char\_set\_symbol**

``$+<=>^`|~`` のようなすべての記号文字。

スキーム変数: **char-set:hex-digit**

C 変数: **scm\_char\_set\_hex\_digit**

16進数で「0123456789abcdefABCDEF」です。

Scheme変数: **char-set:ascii**

C 変数: **scm\_char\_set\_ascii**

すべてASCII文字。

Scheme変数: **char-set:empty**

C 変数: **scm\_char\_set\_empty**

空の文字セット。

スキーム変数: **char-set:designated**

C 変数: **scm\_char\_set\_designated**

この文字セットには、指定されたすべてのコードポイントが含まれています。これには、Unicodeによって文字またはその他の意味が割り当てられたすべてのコードポイントが含まれます。

Scheme変数: **char-set:full**

C 変数: **scm\_char\_set\_full**

この文字セットには、使用可能なすべてのコードポイントが含まれています。これには、指定コードポイントと予約コードポイントの両方が含まれます。

* * *

次へ: [記号](06_06_06_symbols.md#666-シンボル)、前: [文字セット](#664-文字セット)、上: [データ型](06_06_00_data_types.md#66-データ型) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
