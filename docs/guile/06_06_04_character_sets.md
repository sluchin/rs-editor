#### 6.6.4 文字セット [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Character-Sets-1)

本節で説明する機能は、SRFI-14に直接対応するものです。

データ型 _charset_ は文字セットを実装します（[文字](https://doc.guix.gnu.org/guile/latest/en/guile.html#Characters)を参照）。文字セットの内部表現はユーザーには見えないため、それらを処理するための多くの手順が提供されています。

文字セットは作成、拡張、文字のメンバーシップのテスト、および他の文字セットとの比較が可能です。

* [文字セット述語/比較](https://doc.guix.gnu.org/guile/latest/en/guile.html#Character-Set-Predicates_002fComparison)
* [文字セットの反復処理](https://doc.guix.gnu.org/guile/latest/en/guile.html#Iterating-Over-Character-Sets)
* [文字セットの作成](https://doc.guix.gnu.org/guile/latest/en/guile.html#Creating-Character-Sets)
* [文字セットのクエリ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Querying-Character-Sets)
* [文字セット代数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Character_002dSet-Algebra)
* [標準文字セット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Standard-Character-Sets)

* * *

次へ: [文字セットの反復処理](https://doc.guix.gnu.org/guile/latest/en/guile.html#Iterating-Over-Character-Sets)、上へ: [文字セット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Character-Sets) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.4.1 文字セット述語/比較 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Character-Set-Predicates_002fComparison-1)

これらの手順は、オブジェクトが文字セットであるかどうか、または複数の文字セットが互いに等しいか部分集合であるかをテストするために使用します。`char-set-hash` はハッシュ値を計算するために使用でき、高速検索手順などで利用できます。

Scheme手順: **char-set?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_003f)

C 関数: **scm\_char\_set\_p** (obj) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fp)

objが文字セットの場合は`#t`を返し、そうでない場合は`#f`を返します。

Scheme手順: **char-set=** char\_set … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_003d)

C 関数: **scm\_char\_set\_eq** (char\_sets) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005feq)

指定された文字セットがすべて等しい場合は、`#t` を返します。

Scheme手順: **char-set<=** char\_set … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_003c_003d)

C 関数: **scm\_char\_set\_leq** (char\_sets) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fleq)

すべての文字セット char\_seti が文字セット char\_seti+1 の部分集合である場合は、`#t` を返します。

Scheme Procedure: **char-set-hash** cs \[bound\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_002dhash)

C 関数: **scm\_char\_set\_hash** (cs、bound) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fhash)

文字セット cs のハッシュ値を計算します。bound が指定され、それがゼロ以外の場合、返される値は 0 … bound - 1 の範囲に制限されます。

* * *

次へ: [文字セットの作成](https://doc.guix.gnu.org/guile/latest/en/guile.html#Creating-Character-Sets)、前: [文字セット述語/比較](https://doc.guix.gnu.org/guile/latest/en/guile.html#Character-Set-Predicates_002fComparison)、上: [文字セット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Character-Sets) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.4.2 文字セットの反復処理 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Iterating-Over-Character-Sets-1)

文字セットカーソルは、文字セットの要素を順に処理するための手段です。`char-set-cursor`で文字セットカーソルを作成した後、`char-set-ref`でカーソルを逆参照したり、`char-set-cursor-next`で次の要素に進んだりできます。カーソルがセットの最後の要素を通過したかどうかは、`end-of-char-set?`で確認できます。

さらに、文字セットのマッピングおよび（展開）手順も提供されます。

Scheme手順: **char-set-cursor** cs [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_002dcursor)

C 関数: **scm\_char\_set\_cursor** (cs) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fcursor)

カーソルを文字セットcsに戻します。

Scheme手順: **char-set-ref** csカーソル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_002dref)

C 関数: **scm\_char\_set\_ref** (cs, cursor) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fref)

文字セットcs内の現在のカーソル位置cursorにある文字を返します。`end-of-char-set?`がtrueを返すカーソルを渡すとエラーになります。

Scheme プロシージャ: **char-set-cursor-next** cs カーソル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_002dcursor_002dnext)

C 関数: **scm\_char\_set\_cursor\_next** (cs, cursor) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fcursor_005fnext)

文字セットカーソルを文字セットcs内の次の文字まで進めます。指定されたカーソルが`end-of-char-set?`条件を満たす場合はエラーとなります。

Scheme手順: **end-of-char-set?**カーソル[¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-end_002dof_002dchar_002dset_003f)

C 関数: **scm\_end\_of\_char\_set\_p** (カーソル) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fend_005fof_005fchar_005fset_005fp)

カーソルが文字セットの末尾に達した場合は「#t」を返し、そうでない場合は「#f」を返します。

Scheme Procedure: **char-set-fold** kons knil cs [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_002dfold)

C 関数: **scm\_char\_set\_fold** (kons, knil, cs) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005ffold)

プロシージャkonsを文字セットcsに折り畳み、knilで初期化します。

Scheme手順: **char-set-unfold** pfg seed \[base\_cs\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_002dunfold)

C 関数: **scm\_char\_set\_unfold** (p, f, g, seed, base\_cs) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005funfold)

これは文字セットの基本的な構成要素です。

* g は、初期シードから一連の「シード」値を生成するために使用されます: シード、(g シード)、(g^2 シード)、(g^3 シード)、…
* p は停止するタイミングを示します。つまり、シード値のいずれかに適用したときに true が返されたときです。
* f は各シード値を文字にマッピングします。これらの文字は基本文字セット base\_cs に追加され、結果が生成されます。base\_cs のデフォルト値は空セットです。

Scheme手順: **char-set-unfold!** pfg seed base\_cs [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_002dunfold_0021)

C 関数: **scm\_char\_set\_unfold\_x** (p, f, g, seed, base\_cs) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005funfold_005fx)

これは文字セットの基本的な構成要素です。

* g は、初期シードから一連の「シード」値を生成するために使用されます: シード、(g シード)、(g^2 シード)、(g^3 シード)、…
* p は停止するタイミングを示します。つまり、シード値のいずれかに適用したときに true が返されたときです。
* f は各シード値を文字にマッピングします。これらの文字は基本文字セット base\_cs に追加され、結果が生成されます。base\_cs のデフォルト値は空セットです。

Scheme プロシージャ: **char-set-for-each** proc cs [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_002dfor_002deach)

C 関数: **scm\_char\_set\_for\_each** (proc, cs) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005ffor_005feach)

文字セットcs内のすべての文字にprocを適用します。戻り値は指定されていません。

Scheme プロシージャ: **char-set-map** proc cs [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_002dmap)

C 関数: **scm\_char\_set\_map** (proc, cs) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fmap)

プロシージャ proc を cs 内のすべての文字にマッピングします。proc は文字から文字へのプロシージャである必要があります。

* * *

次へ: [文字セットのクエリ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Querying-Character-Sets)、前: [文字セットの反復処理](https://doc.guix.gnu.org/guile/latest/en/guile.html#Iterating-Over-Character-Sets)、上: [文字セット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Character-Sets) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.4.3 文字セットの作成 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Creating-Character-Sets-1)

これらの手順によって新しい文字セットが生成されます。

Scheme手順: **char-set-copy** cs [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_002dcopy)

C 関数: **scm\_char\_set\_copy** (cs) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fcopy)

csに含まれるすべての文字を含む、新たに割り当てられた文字セットを返します。

Scheme手順: **char-set** chr … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset)

C 関数: **scm\_char\_set** (chrs) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset)

指定されたすべての文字を含む文字セットを返します。

Scheme手順: **list->char-set** list \[base\_cs\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-list_002d_003echar_002dset)

C 関数: **scm\_list\_to\_char\_set** (list, base\_cs) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005flist_005fto_005fchar_005fset)

文字リストを文字セットに変換します。文字セット base\_cs が指定されている場合、このセットに含まれる文字も結果に含まれます。

Scheme手順: **list->char-set!** list base\_cs [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-list_002d_003echar_002dset_0021)

C 関数: **scm\_list\_to\_char\_set\_x** (list, base\_cs) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005flist_005fto_005fchar_005fset_005fx)

文字リストを文字セットに変換します。文字はbase_csに追加され、base_csが返されます。

Scheme手順: **string->char-set** str \[base\_cs\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002d_003echar_002dset)

C 関数: **scm\_string\_to\_char\_set** (str, base\_cs) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fto_005fchar_005fset)

文字列strを文字セットに変換します。文字セットbase_csが指定された場合、そのセットに含まれる文字も結果に含まれます。

Scheme手順: **string->char-set!** str base\_cs [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002d_003echar_002dset_0021)

C 関数: **scm\_string\_to\_char\_set\_x** (str, base\_cs) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fto_005fchar_005fset_005fx)

文字列strを文字セットに変換します。文字列から文字を抽出し、base_csに追加して、base_csを返します。

Scheme 手順: **char-set-filter** pred cs \[base\_cs\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_002dfilter)

C 関数: **scm\_char\_set\_filter** (pred, cs, base\_cs) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005ffilter)

csに含まれるすべての文字を含む文字セットを返します。この文字セットはpredを満たす必要があります。base\_csの文字が指定されている場合は、結果に追加されます。

Scheme 手順: **char-set-filter!** pred cs base\_cs [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_002dfilter_0021)

C 関数: **scm\_char\_set\_filter\_x** (pred, cs, base\_cs) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005ffilter_005fx)

csに含まれるすべての文字を含む文字セットを返し、predを満たすようにします。文字はbase_csに追加され、base_csが返されます。

Scheme 手順: **ucs-range->char-set** lower upper \[error \[base\_cs\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ucs_002drange_002d_003echar_002dset)

C 関数: **scm\_ucs\_range\_to\_char\_set** (lower、upper、error、base\_cs) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fucs_005frange_005fto_005fchar_005fset)

文字コードが半開区間 \[lower,upper] 内にあるすべての文字を含む文字セットを返します。

errorが真の値の場合、指定された範囲に実装された文字範囲に含まれていない文字が含まれていると、エラーが通知されます。errorが`#f`の場合、これらの文字は結果の文字セットから黙って除外されます。

base\_cs に含まれる文字が指定されている場合は、結果に追加されます。

Scheme 手順: **ucs-range->char-set!** lower upper error base\_cs [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ucs_002drange_002d_003echar_002dset_0021)

C 関数: **scm\_ucs\_range\_to\_char\_set\_x** (lower、upper、error、base\_cs) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fucs_005frange_005fto_005fchar_005fset_005fx)

文字コードが半開区間 \[lower,upper] 内にあるすべての文字を含む文字セットを返します。

errorが真の値の場合、指定された範囲に実装された文字範囲に含まれていない文字が含まれていると、エラーが通知されます。errorが`#f`の場合、これらの文字は結果の文字セットから黙って除外されます。

文字はbase_csに追加され、base_csが返されます。

Scheme手順: **\->char-set** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002d_003echar_002dset)

C 関数: **scm\_to\_char\_set** (x) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005fchar_005fset)

x を文字セットに変換します。x は文字列、文字、または文字セットのいずれかです。文字列は構成文字のセットに変換され、文字は単一要素のセットに変換され、文字セットはそのまま返されます。

* * *

次へ: [文字セット代数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Character_002dSet-Algebra)、前: [文字セットの作成](https://doc.guix.gnu.org/guile/latest/en/guile.html#Creating-Character-Sets)、上: [文字セット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Character-Sets) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.4.4 文字セットのクエリ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Querying-Character-Sets-1)

これらの手順を使用して、文字セットの要素やその他の情報にアクセスします。

Scheme手順: **%char-set-dump** cs [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0025char_002dset_002ddump)

cs のデバッグ情報を含む関連付けリストを返します。関連付けリストには、次のエントリが含まれます。

`char-set`

文字セット自体

`len`

文字セットに含まれる連続するコードポイントのグループの数

`範囲`

各サブリストがコードポイントの範囲とそれに関連付けられた文字であるリストのリスト

この関数の戻り値は、Guile のバージョン間で一貫性が保証されないため、コード内で使用しないでください。

Scheme手順: **char-set-size** cs [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_002dsize)

C 関数: **scm\_char\_set\_size** (cs) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fsize)

文字セットcsに含まれる要素の数を返します。

Scheme Procedure: **char-set-count** pred cs [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_002dcount)

C 関数: **scm\_char\_set\_count** (pred, cs) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fcount)

文字セット cs 内の、述語 pred を満たす要素の数を返します。

Scheme手順: **char-set->list** cs [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_002d_003elist)

C 関数: **scm\_char\_set\_to\_list** (cs) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fto_005flist)

文字セット cs の要素を含むリストを返します。

Scheme手順: **char-set->string** cs [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_002d_003estring)

C 関数: **scm\_char\_set\_to\_string** (cs) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fto_005fstring)

文字セット cs の要素を含む文字列を返します。文字列内の文字の順序は定義されていません。

Scheme 手順: **char-set-contains?** cs ch [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_002dcontains_003f)

C 関数: **scm\_char\_set\_contains\_p** (cs, ch) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fcontains_005fp)

文字chが文字セットcsに含まれている場合は`#t`を返し、そうでない場合は`#f`を返します。

Scheme Procedure: **char-set-every** pred cs [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_002devery)

C 関数: **scm\_char\_set\_every** (pred, cs) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fevery)

文字セット cs 内のすべての文字が述語 pred を満たす場合は、真の値を返します。

Scheme 手順: **char-set-any** pred cs [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_002dany)

C 関数: **scm\_char\_set\_any** (pred, cs) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fany)

文字セット cs 内のいずれかの文字が述語 pred を満たす場合は、真の値を返します。

* * *

次へ: [標準文字セット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Standard-Character-Sets)、前: [文字セットのクエリ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Querying-Character-Sets)、上: [文字セット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Character-Sets) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.4.5 文字セット代数 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Character_002dSet-Algebra-1)

文字セットは、和集合、補集合、積集合などの一般的な集合代数演算を用いて操作できます。これらの操作はすべて、文字セット引数を変更する副作用のあるバリアントを提供します。

スキーム手順: **char-set-adjoin** cs chr … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_002dadjoin)

C 関数: **scm\_char\_set\_adjoin** (cs, chrs) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fadjoin)

すべての文字引数を、文字セットである最初の引数に追加します。

Scheme手順: **char-set-delete** cs chr … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_002ddelete)

C 関数: **scm\_char\_set\_delete** (cs, chrs) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fdelete)

最初の引数（文字セットである必要があります）から、すべての文字引数を削除します。

スキーム手順: **char-set-adjoin!** cs chr … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_002dadjoin_0021)

C 関数: **scm\_char\_set\_adjoin\_x** (cs, chrs) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fadjoin_005fx)

すべての文字引数を、文字セットである最初の引数に追加します。

Scheme手順: **char-set-delete!** cs chr … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_002ddelete_0021)

C 関数: **scm\_char\_set\_delete\_x** (cs, chrs) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fdelete_005fx)

最初の引数（文字セットである必要があります）から、すべての文字引数を削除します。

Scheme手順: **char-set-complement** cs [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_002dcomplement)

C 関数: **scm\_char\_set\_complement** (cs) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fcomplement)

文字セットcsの補数を返します。

文字セットの補集合には、予約済みコードポイント（文字に関連付けられていないコードポイント）が多数含まれる可能性があることに注意してください。`char-set-complement` の出力と指定コードポイントの集合 `char-set:designated` との共通部分を計算することで、出力結果を修正すると役立つ場合があります。

Scheme手順: **char-set-union** cs … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_002dunion)

C 関数: **scm\_char\_set\_union** (char\_sets) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005funion)

引数として渡されたすべての文字セットの和集合を返します。

Scheme手順: **char-set-intersection** cs … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_002dintersection)

C 関数: **scm\_char\_set\_intersection** (char\_sets) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fintersection)

引数として指定されたすべての文字セットの共通部分を返します。

Scheme手順: **char-set-difference** cs1 cs … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_002ddifference)

C 関数: **scm\_char\_set\_difference** (cs1, char\_sets) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fdifference)

引数として指定されたすべての文字セットの差を返します。

Scheme手順: **char-set-xor** cs … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_002dxor)

C 関数: **scm\_char\_set\_xor** (char\_sets) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fxor)

引数として指定されたすべての文字セットの排他的論理和を返します。

Scheme手順: **char-set-diff+intersection** cs1 cs … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_002ddiff_002bintersection)

C 関数: **scm\_char\_set\_diff\_plus\_intersection** (cs1, char\_sets) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fdiff_005fplus_005fintersection)

引数として指定されたすべての文字セットの差と共通部分を返します。

Scheme 手順: **char-set-complement!** cs [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_002dcomplement_0021)

C 関数: **scm\_char\_set\_complement\_x** (cs) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fcomplement_005fx)

文字セットcsの補数を返します。

Scheme プロシージャ: **char-set-union!** cs1 cs … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_002dunion_0021)

C 関数: **scm\_char\_set\_union\_x** (cs1, char\_sets) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005funion_005fx)

引数として渡されたすべての文字セットの和集合を返します。

Scheme手順: **char-set-intersection!** cs1 cs … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_002dintersection_0021)

C 関数: **scm\_char\_set\_intersection\_x** (cs1, char\_sets) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fintersection_005fx)

引数として指定されたすべての文字セットの共通部分を返します。

Scheme手順: **文字セットの差分!** cs1 cs … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_002ddifference_0021)

C 関数: **scm\_char\_set\_difference\_x** (cs1, char\_sets) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fdifference_005fx)

引数として指定されたすべての文字セットの差を返します。

Scheme手順: **char-set-xor!** cs1 cs … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_002dxor_0021)

C 関数: **scm\_char\_set\_xor\_x** (cs1, char\_sets) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fxor_005fx)

引数として指定されたすべての文字セットの排他的論理和を返します。

Scheme手順: **char-set-diff+intersection!** cs1 cs2 cs … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_002ddiff_002bintersection_0021)

C 関数: **scm\_char\_set\_diff\_plus\_intersection\_x** (cs1, cs2, char\_sets) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fdiff_005fplus_005fintersection_005fx)

引数として指定されたすべての文字セットの差と共通部分を返します。

* * *

前へ: [文字集合代数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Character_002dSet-Algebra)、上へ: [文字集合](https://doc.guix.gnu.org/guile/latest/en/guile.html#Character-Sets) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.4.6 標準文字セット [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Standard-Character-Sets-1)

文字セットデータ型とプロシージャを有効活用するために、いくつかの事前定義された文字セット変数が存在する。

これらの文字セットはロケールに依存せず、`setlocale` 呼び出しによって再計算されることはありません。Unicode コードポイントの全範囲の文字が含まれています。たとえば、`char-set:letter` には約 10 万文字が含まれています。

Scheme変数: **char-set:lower-case** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_003alower_002dcase)

C 変数: **scm\_char\_set\_lower\_case** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005flower_005fcase )

すべて小文字。

Scheme変数: **char-set:upper-case** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_003aupper_002dcase)

C 変数: **scm\_char\_set\_upper\_case** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fupper_005fcase)

すべて大文字。

Scheme変数: **char-set:title-case** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_003atitle_002dcase)

C 変数: **scm\_char\_set\_title\_case** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005ftitle_005fcase)

大文字の後に小文字が続くように機能する、すべての単一文字。

Scheme変数: **char-set:letter** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_003aletter)

C 変数: **scm\_char\_set\_letter** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fletter)

すべての文字。これには、`char-set:lower-case`、`char-set:upper-case`、`char-set:title-case`、および大文字と小文字の区別がない多くの文字が含まれます。たとえば、中国語や日本語の文字には、通常、大文字と小文字の概念がありません。

スキーム変数: **char-set:digit** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_003adigit)

C 変数: **scm\_char\_set\_digit** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fdigit)

すべて数字です。

スキーム変数: **char-set:letter+digit** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_003aletter_002bdigit)

C 変数: **scm\_char\_set\_letter\_and\_digit** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fletter_005fand_005fdigit)

`char-set:letter`と`char-set:digit`の和集合。

Scheme変数: **char-set:graphic** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_003agraphic)

C 変数: **scm\_char\_set\_graphic** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fgraphic)

紙にインクを付けるすべての文字。

Scheme変数: **char-set:printing** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_003aprinting)

C 変数: **scm\_char\_set\_printing** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fprinting)

`char-set:graphic`と`char-set:whitespace`の和集合。

Scheme変数: **char-set:whitespace** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_003awhitespace)

C 変数: **scm\_char\_set\_whitespace** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fwhitespace)

すべて空白文字。

Scheme変数: **char-set:blank** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_003ablank)

C 変数: **scm\_char\_set\_blank** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fblank)

すべての水平方向の空白文字。特に`#\space`と`#\tab`が含まれます。

スキーム変数: **char-set:iso-control** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_003aiso_002dcontrol)

C 変数: **scm\_char\_set\_iso\_control** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fiso_005fcontrol)

ISO制御文字は、C0制御文字（U+0000～U+001F）、削除文字（U+007F）、およびC1制御文字（U+0080～U+009F）です。

Scheme変数: **char-set:punctuation** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_003apunctuation)

C 変数: **scm\_char\_set\_punctuation** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fpunctuation)

`!"#%&'()*,-./:;?@[\\]_{}` などの句読点文字すべて

Scheme変数: **char-set:symbol** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_003asymbol)

C 変数: **scm\_char\_set\_symbol** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fsymbol)

``$+<=>^`|~`` のようなすべての記号文字。

スキーム変数: **char-set:hex-digit** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_003ahex_002ddigit)

C 変数: **scm\_char\_set\_hex\_digit** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fhex_005fdigit)

16進数で「0123456789abcdefABCDEF」です。

Scheme変数: **char-set:ascii** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_003aascii)

C 変数: **scm\_char\_set\_ascii** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fascii)

すべてASCII文字。

Scheme変数: **char-set:empty** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_003aempty)

C 変数: **scm\_char\_set\_empty** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fempty)

空の文字セット。

スキーム変数: **char-set:designated** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_003adesignated)

C 変数: **scm\_char\_set\_designated** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005fdesignated)

この文字セットには、指定されたすべてのコードポイントが含まれています。これには、Unicodeによって文字またはその他の意味が割り当てられたすべてのコードポイントが含まれます。

Scheme変数: **char-set:full** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_003afull)

C 変数: **scm\_char\_set\_full** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fset_005ffull)

この文字セットには、使用可能なすべてのコードポイントが含まれています。これには、指定コードポイントと予約コードポイントの両方が含まれます。

* * *

次へ: [記号](https://doc.guix.gnu.org/guile/latest/en/guile.html#Symbols)、前: [文字セット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Character-Sets)、上: [データ型](https://doc.guix.gnu.org/guile/latest/en/guile.html#Data-Types) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
