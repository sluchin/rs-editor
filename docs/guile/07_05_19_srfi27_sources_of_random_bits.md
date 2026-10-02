#### 7.5.19 SRFI-27 - ランダムビットのソース [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d27-_002d-Sources-of-Random-Bits)

このサブセクションは、Sebastian Egner によって書かれた [SRFI-27 の仕様](http://srfi.schemers.org/srfi-27/srfi-27.html) に基づいています。

この SRFI は、（擬似）乱数生成器へのアクセスを提供します。SRFI-27 が実装されている Guile の組み込み乱数機能については、[乱数生成](https://doc.guix.gnu.org/guile/latest/en/guile.html#Random) を参照してください。SRFI-27 では、乱数は乱数生成アルゴリズムとその状態をカプセル化した _乱数ソース_ から取得されます。

* [デフォルトの乱数生成元](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d27-Default-Random-Source)
* [ランダムソース](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d27-Random-Sources)
* [乱数生成器の取得手順](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d27-Random-Number-Generators)

* * *

次へ: [ランダムソース](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d27-Random-Sources)、上へ: [SRFI-27 - ランダムビットのソース](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d27) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.5.19.1 デフォルトの乱数生成器 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Default-Random-Source)

関数: **ランダム整数** n [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-random_002dinteger)

デフォルトの乱数生成器を使用して、0（含む）からn（含まない）までの乱数を返します。返される乱数は一様分布に従います。

関数: **random-real** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-random_002dreal )

デフォルトの乱数生成器を使用して、0から1の範囲の乱数を返します。返される乱数は一様分布に従います。

機能: **default-random-source** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-default_002drandom_002dsource)

`random-integer` と `random-real` が `random-source-make-integers` と `random-source-make-reals` を使用して生成された乱数ソース (これらの手順については、[乱数生成器の取得手順](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d27-Random-Number-Generators) を参照してください)。`default-random-source` への代入は `random-integer` または `random-real` を変更しないことに注意してください。また、新しい値を代入しないことを強くお勧めします。

* * *

次へ: [乱数生成器の取得手順](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d27-Random-Number-Generators)、前: [デフォルトの乱数ソース](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d27-Default-Random-Source)、上: [SRFI-27 - 乱数ビットのソース](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d27) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次内容")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.5.19.2 ランダムソース [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Random-Sources)

関数: **make-random-source** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002drandom_002dsource)

新しい乱数源を作成します。この手順で作成された各乱数源から得られる乱数ストリームは、以下のいずれかの手順によって状態が変更されない限り、同一になります。

関数: **random-source?** オブジェクト [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-random_002dsource_003f)

オブジェクトがランダムソースであるかどうかをテストします。ランダムソースは互いに素な型です。

関数: **random-source-randomize!** source [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-random_002dsource_002drandomize_0021)

乱数発生器の状態を真にランダムな値に設定しようと試みます。現在の実装では、現在のシステム時刻に基づいたシード値を使用しています。

関数: **random-source-pseudo-randomize!** source ij [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-random_002dsource_002dpseudo_002drandomize_0021)

ランダムソース s の状態を、(i, j) 番目の独立したランダムソースの初期状態に変更します。ここで、i と j は非負の整数です。この手順により、2 つの整数でインデックス付けされた多数の独立したランダムソース (通常はすべて同じバックボーンジェネレータから生成されます) を取得できます。`random-source-randomize!` とは対照的に、この手順は完全に決定論的です。

ランダムな状態に関連付けられた状態は、以下の手順で取得および復元できます。

関数: **random-source-state-ref** ソース [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-random_002dsource_002dstate_002dref)

機能: **random-source-state-set!** ソース状態 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-random_002dsource_002dstate_002dset_0021)

ランダムソースの状態を取得および設定します。状態オブジェクトの性質については、外部表現を持つこと（つまり、`write`に渡して、その後`read`で読み戻すことができること）以外に、いかなる仮定も置かないでください。

* * *

前へ: [ランダムソース](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d27-Random-Sources)、上へ: [SRFI-27 - ランダムビットのソース](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d27) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.5.19.3 乱数生成器の取得手順 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Obtaining-random-number-generator-procedures)

関数: **random-source-make-integers** source [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-random_002dsource_002dmake_002dintegers)

ランダムソース source を使用してランダムな整数を生成する手順を取得します。返される手順は、正の整数 n を引数として受け取り、ソースの状態を進めることで、区間 {0, ..., n\-1} から次の均一分布のランダムな整数を返します。

アプリケーションが同じ乱数源に対して複数の乱数生成器を取得して使用する場合、いずれかの生成器を呼び出すと、乱数源の状態が進みます。したがって、生成器はそれぞれ同じ乱数列を生成するのではなく、状態を共有します。これは、固定された乱数源から派生する他のすべてのタイプの生成器にも当てはまります。

SRFIの規定では「並行処理をサポートする実装では、ジェネレータの状態が適切に進行するようにする」と明記されていますが、GuileのSRFI-27実装では、パフォーマンスに深刻な悪影響を与えるため、現状ではこの規定は適用されていません。そのため、マルチスレッドプログラムでは、スレッド間で共有される乱数ソースに対して自身でロックを行うか、複数のスレッドで異なる乱数ソースを使用する必要があります。

関数: **random-source-make-reals** ソース [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-random_002dsource_002dmake_002dreals)

機能: **random-source-make-reals** ソースユニット [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-random_002dsource_002dmake_002dreals-1)

ランダムソース source を使用して、0 < x < 1 の実数を生成するプロシージャを取得します。プロシージャ rand は引数なしで呼び出されます。

オプションパラメータ unit は、返されるプロシージャによって生成される数値の型と出力の量子化を決定します。unit は _0 < unit < 1_ を満たす数値でなければなりません。返されるプロシージャによって生成される数値は unit と同じ数値型であり、出力値の間隔は最大で unit です。rand は、{1, ..., floor(1/unit)-1} のランダムな整数 x * unit として数値を生成すると考えることができます。ただし、実際の値の生成方法は必ずしもこの方法である必要はなく、rand の実際の解像度は unit よりはるかに高くなる可能性があることに注意してください。unit が指定されていない場合は、効率的な数値形式の仮数部の幅に関連する、妥当な小さな値がデフォルト値として使用されます。

* * *

次へ: [SRFI-30 - ネストされた複数行コメント](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d30)、前: [SRFI-27 - ランダムビットのソース](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d27)、上: [SRFI サポート モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI-Support) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
