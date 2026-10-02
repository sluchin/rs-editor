### 6.21 Smobs [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Smobs-1)

_smob_ は「小さなオブジェクト」です。Guile 2.0.12 で外部オブジェクトが導入される前は ([Foreign Objects](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Objects) を参照)、C コードで新しい種類の Scheme オブジェクトを定義するには smob が推奨されていました。後述するいわゆる「適用可能な SMOBs」を除き、smob は現在レガシー インターフェースとなっており、いずれ非推奨になる予定です。[Deprecation](https://doc.guix.gnu.org/guile/latest/en/guile.html#Deprecation) を参照してください。新しいコードでは、外部オブジェクト インターフェースを使用する必要があります。

このセクションには、スモブの定義と操作に関する参考情報が含まれています。スモブのチュートリアル形式の入門については、このマニュアルの以前のバージョンの「新しい型の定義（スモブ）」を参照してください。

関数: `scm_t_bits` **scm\_make\_smob\_type** `(const char *name, size_t size)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- scm_005fmake_005fsmob_005ftype)

この関数は、name という名前の新しい smob タイプをインスタンスサイズ size を指定してシステムに追加します。戻り値は、そのタイプのインスタンスを作成する際に使用されるタグです。

サイズが0の場合、デフォルトの_free_関数は何も行いません。

sizeが0でない場合、デフォルトの_free_関数は、`scm_gc_free`を使用して`SCM_SMOB_DATA`が指すメモリブロックを解放します。`scm_gc_free`呼び出しのwhatパラメータはnameになります。

_mark_、_free_、_print_、および_equalp_関数にはデフォルト値が用意されています。これらの関数をカスタマイズする場合は、`scm_make_smob_type`の呼び出しの直後に、`scm_set_smob_mark`、`scm_set_smob_free`、`scm_set_smob_print`、および/または`scm_set_smob_equalp`のいずれか、または複数を呼び出す必要があります。

C 関数: `void` **scm\_set\_smob\_free** `(scm_t_bits tc, size_t (*free) (SCM obj))` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fset_005fsmob_005ffree)

この関数は、タグ tc で指定された smob タイプに対して、smob 解放手順 (_finalizer_ と呼ばれることもあります) を設定します。tc は、`scm_make_smob_type` によって返されるタグです。

解放処理では、smobインスタンスobjに直接関連付けられているすべてのリソースを解放する必要があります。また、参照するすべての`SCM`値は既に解放済みであり、無効であると想定する必要があります。

また、`scm_gc_free`、`SCM_SMOB_FLAGS`、`SCM_SMOB_DATA`、`SCM_SMOB_DATA_2`、および`SCM_SMOB_DATA_3`以外のlibguile関数またはマクロを呼び出してはなりません。

フリープロシージャは0を返さなければならない。

obj に関連付けられたリソースが `scm_gc_malloc` または `scm_gc_malloc_pointerless` で割り当てられたメモリのみで構成されている場合は、解放手順を定義する必要はありません。これは、このメモリが不要になったときにガベージコレクタによって自動的に回収されるためです([`scm_gc_malloc`](https://doc.guix.gnu.org/guile/latest/en/guile.html#Memory-Blocks) を参照)。

smob フリー関数はスレッドセーフである必要があります。ファイナライザと並行処理については、[外部オブジェクトメモリ管理](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Object-Memory-Management) を参照してください。スレッドセーフではないアプリケーションに Guile を組み込んでいて、ファイナライズが必要な smob 型を定義している場合は、自動ファイナライズを無効にして、`scm_manually_run_finalizers()` を自分で呼び出すように設定することをお勧めします。[外部オブジェクト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Objects) を参照してください。

C 関数: `void` **scm\_set\_smob\_mark** `(scm_t_bits tc, SCM (*mark) (SCM obj))` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fset_005fsmob_005fmark)

この関数は、タグtcで指定されたsmobタイプのsmobマーキング手順を設定します。tcは`scm_make_smob_type`によって返されるタグです。

マーキング手順を定義することは、ほとんどの場合、間違った方法です。`scm_gc_malloc`関数と`scm_gc_malloc_pointerless`関数を使用してsmobデータを割り当て、GCがポインタを自動的にトレースできるようにする方がはるかに望ましいです。

現在見られるマーク処理は、ほぼ間違いなくGuile 1.8の頃、つまりBoehm-Demers-Weiserガベージコレクタへの移行以前の時代のものである。このようなsmob実装は、`scm_gc_malloc`などの関数のみを使用するように変更し、マーク機能を削除すべきである。

マーク関数を残す場合は、フリーリストにあるオブジェクトに対しても呼び出される可能性があることに注意してください。BDW GCの`gc/gc_mark.h`ヘッダーファイルのコメントをよく読んで理解してください。

マーク処理では、smobインスタンスobjが直接参照するすべての`SCM`値に対して`scm_gc_mark`が呼び出される必要があります。これらの`SCM`値のうち1つが処理から返され、Guileはその値に対して`scm_gc_mark`を呼び出します。これは、リストを形成するsmobインスタンスの深い再帰を回避するために使用できます。

`scm_gc_mark`、`SCM_SMOB_FLAGS`、`SCM_SMOB_DATA`、`SCM_SMOB_DATA_2`、および`SCM_SMOB_DATA_3`以外のlibguile関数またはマクロを呼び出してはならない。

C 関数: `void` **scm\_set\_smob\_print** `(scm_t_bits tc, int (*print) (SCM obj, SCM port, scm_print_state* pstate))` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fset_005fsmob_005fprint)

この関数は、タグtcで指定されたsmobタイプのsmob印刷手順を設定します。tcは`scm_make_smob_type`によって返されるタグです。

印刷手順では、pstate の情報を使用して、smob インスタンス obj のテキスト表現をポートに出力する必要があります。

テキスト表現は`#<名前 ...>`の形式である必要があります。これにより、`read`がそれを他のScheme値として解釈しないことが保証されます。

pstate を無視して、`scm_display`、`scm_write`、`scm_simple_format`、および `scm_puts` を使用してポートに出力するのが最も良い場合が多いです。

C 関数: `void` **scm\_set\_smob\_equalp** `(scm_t_bits tc, SCM (*equalp) (SCM obj1, SCM obj2))` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fset_005fsmob_005fequalp)

この関数は、タグ tc で指定された smob タイプに対して、smob 等価性テスト述語を設定します。tc は、`scm_make_smob_type` によって返されるタグです。

equalpプロシージャは、obj1がobj2と等しい場合、`SCM_BOOL_T`を返す必要があります。そうでない場合は、`SCM_BOOL_F`を返す必要があります。obj1とobj2はどちらもsmob型tcのインスタンスです。

C 関数: `void` **scm\_assert\_smob\_type** `(scm_t_bits tag, SCM val)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fassert_005fsmob_005ftype)

valがtagで示される型のsmobである場合は、何もしない。そうでない場合は、エラーを通知する。

C マクロ: `int` **SCM\_SMOB\_PREDICATE** `(scm_t_bits タグ、SCM 式)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fSMOB_005fPREDICATE)

expがtagで指定された型のsmobインスタンスであればtrueを返し、そうでなければfalseを返します。式expは複数回評価される可能性があるため、副作用があってはなりません。

C 関数: `SCM` **scm\_new\_smob** `(scm_t_bits tag, void *data)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fnew_005fsmob)

C 関数: `SCM` **scm\_new\_double\_smob** `(scm_t_bits tag, void *data, void *data2, void *data3)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fnew_005fdouble_005fsmob)

タグ tag と smob データ data、data2、data3 を適切に指定して、新しい smob を作成します。

タグは、`scm_make_smob_type` によって返されたものです。初期値 data、data2、および data3 は `scm_t_bits` 型です。これらの値を `SCM` 値として使用する場合は、まず `SCM_UNPACK` を使用して `scm_t_bits` 型に変換する必要があります。

smobインスタンスのフラグは最初はゼロから始まります。

C マクロ: `scm_t_bits` **SCM\_SMOB\_FLAGS** `(SCM オブジェクト)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fSMOB_005fFLAGS)

smobオブジェクトの追加ビット16個を返します。これらのビットには特に意味は定義されていませんので、自由にお使いください。

C マクロ: `scm_t_bits` **SCM\_SET\_SMOB\_FLAGS** `(SCM オブジェクト、scm_t_bits フラグ)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fSET_005fSMOB_005fFLAGS)

smob objの16ビット余剰部分をフラグに設定します。これらのビットには特に意味は定義されていませんので、自由に使用できます。

C マクロ: `scm_t_bits` **SCM\_SMOB\_DATA** `(SCM オブジェクト)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fSMOB_005fDATA)

C マクロ: `scm_t_bits` **SCM\_SMOB\_DATA\_2** `(SCM オブジェクト)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fSMOB_005fDATA_005f2)

C マクロ: `scm_t_bits` **SCM\_SMOB\_DATA\_3** `(SCM オブジェクト)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fSMOB_005fDATA_005f3)

smob obj の最初の (2 番目、3 番目) の直近のワードを `scm_t_bits` 値として返します。ワードに `SCM` 値が含まれている場合は、代わりに `SCM_SMOB_OBJECT` (など) を使用します。

C マクロ: `void` **SCM\_SET\_SMOB\_DATA** `(SCM obj, scm_t_bits val)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fSET_005fSMOB_005fDATA)

C マクロ: `void` **SCM\_SET\_SMOB\_DATA\_2** `(SCM obj, scm_t_bits val)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fSET_005fSMOB_005fDATA_005f2)

C マクロ: `void` **SCM\_SET\_SMOB\_DATA\_3** `(SCM obj, scm_t_bits val)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fSET_005fSMOB_005fDATA_005f3)

smob obj の最初の (2 番目、3 番目) の直近のワードを val に設定します。ワードを `SCM` 値に設定する必要がある場合は、代わりに `SCM_SMOB_SET_OBJECT` (など) を使用してください。

C マクロ: `SCM` **SCM\_SMOB\_OBJECT** `(SCM obj)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fSMOB_005fOBJECT)

C マクロ: `SCM` **SCM\_SMOB\_OBJECT\_2** `(SCM obj)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- SCM_005fSMOB_005fOBJECT_005f2)

C マクロ: `SCM` **SCM\_SMOB\_OBJECT\_3** `(SCM obj)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fSMOB_005fOBJECT_005f3)

smob obj の最初の (2 番目、3 番目) の直近のワードを `SCM` 値として返します。ワードに `scm_t_bits` 値が含まれている場合は、代わりに `SCM_SMOB_DATA` (など) を使用します。

C マクロ: `void` **SCM\_SET\_SMOB\_OBJECT** `(SCM obj, SCM val)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fSET_005fSMOB_005fOBJECT)

C マクロ: `void` **SCM\_SET\_SMOB\_OBJECT\_2** `(SCM obj, SCM val)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fSET_005fSMOB_005fOBJECT_005f2)

C マクロ: `void` **SCM\_SET\_SMOB\_OBJECT\_3** `(SCM obj, SCM val)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fSET_005fSMOB_005fOBJECT_005f3)

smob obj の最初の (2 番目、3 番目) 即値ワードを val に設定します。ワードを `scm_t_bits` 値に設定する必要がある場合は、代わりに `SCM_SMOB_SET_DATA` (など) を使用してください。

C マクロ: `SCM *` **SCM\_SMOB\_OBJECT\_LOC** `(SCM obj)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fSMOB_005fOBJECT_005fLOC)

C マクロ: `SCM *` **SCM\_SMOB\_OBJECT\_2\_LOC** `(SCM obj)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fSMOB_005fOBJECT_005f2_005fLOC)

C マクロ: `SCM *` **SCM\_SMOB\_OBJECT\_3\_LOC** `(SCM obj)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fSMOB_005fOBJECT_005f3_005fLOC)

smob obj の最初の (2 番目、3 番目) 即値ワードへのポインタを返します。これは `SCM` へのポインタであることに注意してください。`scm_t_bits` の値を使用する必要がある場合は、必要に応じて `SCM_PACK` および `SCM_UNPACK` を使用してください。

機能: `SCM` **scm\_markcdr** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fmarkcdr)

smob x 内の参照をマークします。ただし、x の最初のデータワードには通常の Scheme オブジェクトが含まれており、x は他のオブジェクトを参照していないものとします。この関数は、x の最初のデータワードを単純に返します。

* * *

次へ: [設定、機能、およびランタイム オプション](https://doc.guix.gnu.org/guile/latest/en/guile.html#Options-and-Config)、前: [Smobs](https://doc.guix.gnu.org/guile/latest/en/guile.html#Smobs)、上: [API リファレンス](https://doc.guix.gnu.org/guile/latest/en/guile.html#API-Reference) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
