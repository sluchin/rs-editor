### 6.17 メモリ管理とガベージコレクション [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Memory-Management-and-Garbage-Collection)

Guileは、ほとんどのオブジェクトの管理にガベージコレクターを使用します。ガベージコレクターは基本的に目に見えないように設計されていますが、場合によっては明示的に操作する必要が生じます。

C 言語から Guile を使用する際にガベージ コレクションがどのように関係するかについての一般的な説明については、[ガベージ コレクション](https://doc.guix.gnu.org/guile/latest/en/guile.html#Garbage-Collection) を参照してください。

* [ガベージコレクションに関連する関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Garbage-Collection-Functions)
* [メモリブロック](https://doc.guix.gnu.org/guile/latest/en/guile.html#Memory-Blocks)
* [弱い参照](https://doc.guix.gnu.org/guile/latest/en/guile.html#Weak-References)
* [保護者](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guardians)

* * *

次へ: [メモリ ブロック](https://doc.guix.gnu.org/guile/latest/en/guile.html#Memory-Blocks)、上: [メモリ管理とガベージ コレクション](https://doc.guix.gnu.org/guile/latest/en/guile.html#Memory-Management) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.17.1 ガベージコレクションに関連する機能 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Function-related-to-Garbage-Collection )

Scheme手順: **gc** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-gc-1)

C 関数: **scm\_gc** () [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fgc)

`SCM` 内の有効なオブジェクトをすべて検索し、アクセスできなくなったオブジェクトを再利用のために解放します。通常、この関数を明示的に呼び出す必要はありません。必要に応じて自動的に実行されます。

C 関数: `SCM` **scm\_gc\_protect\_object** `(SCM obj)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fgc_005fprotect_005fobject)

ガベージ コレクタによって解放される可能性がある場合でも、オブジェクトが解放されないように保護します。オブジェクトの使用が終わったら、オブジェクトに対して `scm_gc_unprotect_object` を呼び出します。`scm_gc_protect_object`/`scm_gc_unprotect_object` の呼び出しはネストできます。オブジェクトは、保護された回数と同じ回数だけ保護が解除されるまで保護されたままになります。保護された回数よりも多くオブジェクトを保護解除するとエラーになります。渡された SCM オブジェクトを返します。

obj を C のグローバル変数に格納すると、同じ効果が得られることに注意してください [18](https://doc.guix.gnu.org/guile/latest/en/guile.html#FOOT18)。

C 関数: `SCM` **scm\_gc\_unprotect\_object** `(SCM obj)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fgc_005funprotect_005fobject)

`scm_gc_unprotect_object` によって保護されていたオブジェクトを、ガベージコレクタから保護解除します。渡された SCM オブジェクトを返します。

C 関数: `SCM` **scm\_permanent\_object** `(SCM obj)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fpermanent_005fobject)

`scm_gc_protect_object` と同様に、コレクターが常にオブジェクトをマークするようにしますが、ネストしてはならず (オブジェクトに対して `scm_permanent_object` を一度だけ呼び出す)、対応する非永続関数はありません。オブジェクトが永続として宣言されると、解放されることはありません。渡された SCM オブジェクトを返します。

C マクロ: `void` **scm\_remember\_upto\_here\_1** `(SCM obj)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fremember_005fupto_005fhere_005f1)

C マクロ: `void` **scm\_remember\_upto\_here\_2** `(SCM obj1, SCM obj2)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fremember_005fupto_005fhere_005f2)

指定されたオブジェクトへの参照を作成することで、それらがスタックまたはレジスタ上に確実に存在し、この時点より前にガベージコレクタによって解放されないようにします。

これらの関数は、通常のC言語のローカル変数（つまり「自動変数」）にのみ適用できることに注意してください。グローバル変数や静的変数に格納されているオブジェクト、あるいはmallocで確保されたブロックなどは、このメカニズムでは保護できません。

Scheme Procedure: **gc-stats** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-gc_002dstats)

C 関数: **scm\_gc\_stats** () [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fgc_005fstats)

Guileの現在のストレージ使用状況に関する統計情報の関連付けリストを返します。

Scheme手順: **gc-live-object-stats** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-gc_002dlive_002dobject_002dstats)

C 関数: **scm\_gc\_live\_object\_stats** () [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fgc_005flive_005fobject_005fstats)

現在アクティブなオブジェクトの統計情報のリストを返します。

関数: `void` **scm\_gc\_mark** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fgc_005fmark)

オブジェクト x をマークし、x が参照するすべてのオブジェクトに対して再帰処理を行います。x のマーク ビットが既に設定されている場合は、すぐに戻ります。この関数は、ガベージ コレクションのマーク フェーズ中にのみ呼び出す必要があり、通常は smob の _mark_ 関数から呼び出されます。

* * *

次へ: [弱い参照](https://doc.guix.gnu.org/guile/latest/en/guile.html#Weak-References)、前: [ガベージコレクションに関連する関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Garbage-Collection-Functions)、上: [メモリ管理とガベージコレクション](https://doc.guix.gnu.org/guile/latest/en/guile.html#Memory-Management) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.17.2 メモリブロック [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Memory-Blocks-1)

C言語プログラムでは、メモリブロックの動的な管理は通常、malloc、realloc、freeといった関数を用いて行われます。Guileには、ガベージコレクタとエラー報告システムに統合された、動的メモリ割り当てのための追加関数が用意されています。

Scheme オブジェクト (例えば外部オブジェクト) に関連付けられたメモリ ブロックは、`scm_gc_malloc` または `scm_gc_malloc_pointerless` を使用して割り当てる必要があります。これらの 2 つの関数は、有効なポインタを返すか、エラーを通知します。このように割り当てられたメモリ ブロックは明示的に解放できますが、厳密には必要なく、`scm_gc_free` を呼び出さないことを推奨します。`scm_gc_malloc` または `scm_gc_malloc_pointerless` を使用して割り当てられたすべてのメモリは、ガベージ コレクタがそれへの有効な参照を認識できなくなったときに自動的に解放されます [19](https://doc.guix.gnu.org/guile/latest/en/guile.html#FOOT19)。

ガベージコレクションが発生すると、Guile は `scm_gc_malloc` で割り当てられたメモリ内のワードを走査し、有効なポインタを探します。つまり、`scm_gc_malloc` で割り当てられたメモリにメモリ内の他の部分へのポインタが含まれている場合、ガベージコレクタはそれを検知し、メモリの解放を阻止します[20](https://doc.guix.gnu.org/guile/latest/en/guile.html#FOOT20)。逆に、`scm_gc_malloc_pointerless` で割り当てられたメモリは「ポインタなし」とみなされ、ポインタの走査は行われません。

Schemeオブジェクトに関連付けられていないメモリについては、`malloc`の代わりに`scm_malloc`を使用できます。`scm_gc_malloc`と同様に、有効なポインタを返すか、エラーを通知します。ただし、新しいメモリブロックがガベージコレクションによって解放されることは想定していません。メモリは`free`で明示的に解放する必要があります。

また、適切な場合には `realloc` の代わりに `scm_gc_realloc` と `scm_realloc` を使用し、適切な場合には `calloc` の代わりに `scm_gc_calloc` と `scm_calloc` を使用します。

関数 `scm_dynwind_free` は、dynwind コンテキストを抜けるときに libc の `free` を使用してメモリを解放する必要がある場合に役立ちます。[Dynamic Wind](https://doc.guix.gnu.org/guile/latest/en/guile.html#Dynamic-Wind) を参照してください。

C 関数: `void *` **scm\_malloc** `(size_t size)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fmalloc)

C 関数: `void *` **scm\_calloc** `(size_t size)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fcalloc)

sizeバイトのメモリを割り当て、そのメモリへのポインタを返します。sizeが0の場合はNULLを返します。メモリが不足している場合はエラーを通知します。この関数は、必要に応じてガベージコレクションを実行してメモリを解放します。

メモリはlibcの`malloc`関数によって割り当てられ、`free`関数で解放できます。異なるモジュール間でメモリを簡単に受け渡しできるように、`scm_malloc`に対応する`scm_free`関数は存在しません。

関数 `scm_calloc` は `scm_malloc` と似ていますが、メモリブロックをゼロで初期化する点も異なります。

これらの関数は（間接的に）`scm_gc_register_allocation`を呼び出します。

C言語関数: `void *` **scm\_realloc** `(void *mem, size_t new_size)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005frealloc)

mem にあるメモリブロックのサイズを new_size に変更し、その新しい位置を返します。new_size が 0 の場合、これは mem に対して `free` を呼び出すのと同じで、`NULL` が返されます。mem が `NULL` の場合、この関数は `scm_malloc` と同様に動作し、new_size サイズの新しいブロックを割り当てます。

メモリが不足している場合は、エラーを通知します。この関数は、必要に応じてガベージコレクション（GC）を実行してメモリを解放します。

この関数は`scm_gc_register_allocation`を呼び出します。

C 関数: `void *` **scm\_gc\_malloc** `(size_t size, const char *what)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fgc_005fmalloc)

C 関数: `void *` **scm\_gc\_malloc\_pointerless** `(size_t size, const char *what)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fgc_005fmalloc_005fpointerless)

C 関数: `void *` **scm\_gc\_realloc** `(void *mem, size_t old_size, size_t new_size, const char *what);` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fgc_005frealloc)

C 関数: `void *` **scm\_gc\_calloc** `(size_t size, const char *what)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fgc_005fcalloc)

指定したサイズのバイト数の自動管理メモリを割り当てます。このメモリは、どの有効なメモリブロックからも参照されなくなると、自動的に解放されます。

ガベージコレクションが発生すると、Guile は `scm_gc_malloc` または `scm_gc_calloc` で割り当てられたメモリ内のワードを走査し、GC によって管理されている他のメモリ割り当てへのポインタを探します。一方、`scm_gc_malloc_pointerless` で割り当てられたメモリは、ポインタの走査対象になりません。

`scm_gc_realloc` 関数は、mem が指すメモリ領域の「ポインタレス性」を維持します。再割り当てされたメモリブロックの元のサイズも渡す必要があることに注意してください。詳細は下記を参照してください。

C 関数: `void` **scm\_gc\_free** `(void *mem, size_t size, const char *what)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fgc_005ffree)

`scm_gc` 関数のいずれかによって以前に割り当てられた、`mem` が指すメモリブロックを明示的に解放します。この関数は、Guile 1.8 でコンパイルする必要のあるコードベースを除き、ほとんどの場合不要です。

サイズパラメータは明示的に渡す必要があることに注意してください。これは、通常、このパラメータ（GC制御対象オブジェクトに関連付けられたメモリ）を指定するのは容易であり、メモリ管理のオーバーヘッドを非常に低く抑えるのに役立つためです。ただし、Guile 2.xでは、サイズは常に無視されます。

C 関数: `void` **scm\_gc\_register\_allocation** `(size_t size)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fgc_005fregister_005fallocation)

指定されたバイト数が割り当てられたことをガベージコレクタに通知します。そうでなければ、コレクタはこの情報を知ることができません。

一般的に、Schemeは一定量のメモリが割り当てられた後で初めてガベージコレクションを開始します。この関数を呼び出すことで、Schemeのガベージコレクタはより多くのメモリ割り当てを認識し、必要に応じてより頻繁にガベージコレクションを実行します。

イメージなどの大きな非管理割り当てが、外部オブジェクトなどの小さなScheme割り当てによって解放される可能性がある場合、この関数を呼び出すことが特に重要です。

C 関数: `void` **scm\_dynwind\_free** `(void *mem)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fdynwind_005ffree-1)

`scm_dynwind_unwind_handler (free, mem, SCM_F_WIND_EXPLICITLY)` と同等です。つまり、現在の dynwind が終了すると、mem にあるメモリ ブロックが (C ライブラリの `free` を使用して) 解放されます。

Scheme手順: **malloc-stats** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-malloc_002dstats)

mallocされたオブジェクトの数を記述するalist（（what . n）...）を返します。whatは`scm_gc_malloc`の2番目の引数で、nは現在割り当てられているその型のオブジェクトの数です。

この関数は、Guileのコンパイル時に`GUILE_DEBUG_MALLOC`プリプロセッサマクロが定義されている場合にのみ使用できます。

* * *

次へ: [Guardians](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guardians)、前: [Memory Blocks](https://doc.guix.gnu.org/guile/latest/en/guile.html#Memory-Blocks)、上: [Memory Management and Garbage Collection](https://doc.guix.gnu.org/guile/latest/en/guile.html#Memory-Management) \[[Contents](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[Index](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "Index")\]

#### 6.17.3 弱い参照 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Weak-References-1)

[修正：この章は、マイケル・リヴシン氏の質問に対するミカエル・ジュルフェルト氏の回答に基づいています。もちろん、誤りがあったとしても、それは彼らの責任ではありません。]

弱い参照を使用すると、データに管理情報を付加できます。元のデータが使用されなくなり、ガベージコレクションされると、付加情報は自動的に消去されます。弱いキーハッシュでは、そのキーが他の場所から参照されなくなると、そのキーのハッシュエントリはすぐに消去されます。弱い値ハッシュの場合も同様で、値が使用されなくなるとすぐに消去されます。二重に弱いハッシュのエントリは、キーまたは値のいずれかが他の場所で使用されなくなると消去されます。

オブジェクトプロパティは、多くの状況において、脆弱なキーハッシュと同様の機能を提供します。（[オブジェクトプロパティ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Object-Properties)を参照）

以下に例を示します（少し無理があるかもしれませんが、例の一つは実際にGuileで使用されています）。

ファイル名とソースコード式の位置に関する情報を、式自体に関連付けたいデバッグシステムを実装していると仮定します。

ハッシュテーブルを使用すれば可能ですが、通常のハッシュテーブルを使用すると、例えばファイルが再読み込みされたときに、スキームインタープリタが古いソースを「忘れる」ことができなくなります。

ソースコード式から位置情報へのマッピングを実装するには、式がテーブルに存在するという理由だけで記憶されることを望まないため、弱キーテーブルを使用する必要があります。

ソースファイルの行番号からソースコード式へのマッピングを実装するには、弱値テーブルを使用します。

ソースコード式からそれらが構成するプロシージャへのマッピングを実装するには、二重弱テーブルを使用する必要があります。

* [弱いハッシュテーブル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Weak-hash-tables)
* [弱ベクトル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Weak-vectors)

* * *

次へ: [Weak vectors](https://doc.guix.gnu.org/guile/latest/en/guile.html#Weak-vectors)、上へ: [Weak References](https://doc.guix.gnu.org/guile/latest/en/guile.html#Weak-References ) \[[Contents](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[Index](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "Index")\]

#### 6.17.3.1 弱いハッシュテーブル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Weak-hash-tables-1)

Scheme Procedure: **make-weak-key-hash-table** \[size\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dweak_002dkey_002dhash_002dtable)

Scheme Procedure: **make-weak-value-hash-table** \[size\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dweak_002dvalue_002dhash_002dtable)

Scheme Procedure: **make-doubly-weak-hash-table** \[size\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002ddoubly_002dweak_002dhash_002dtable)

C 関数: **scm\_make\_weak\_key\_hash\_table** (size) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fmake_005fweak_005fkey_005fhash_005ftable)

C 関数: **scm\_make\_weak\_value\_hash\_table** (size) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fmake_005fweak_005fvalue_005fhash_005ftable)

C 関数: **scm\_make\_doubly\_weak\_hash\_table** (size) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fmake_005fdoubly_005fweak_005fhash_005ftable)

サイズバケット付きの弱ハッシュテーブルを返します。他のハッシュテーブルと同様に、テーブルの適切なサイズを選択するには注意が必要です。

弱いハッシュ テーブルは、ハンドルを操作するルーチンを除いて、通常のハッシュ テーブルを変更するのとまったく同じ方法で変更できます。弱いテーブルは、ハンドルを持たない別の実装を内部的に使用しています。`hashq-ref` などの詳細については、[ハッシュ テーブル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hash-Tables) を参照してください。

弱いキーのハッシュテーブルでは、値への参照は強いことに注意してください。つまり、値がキーを間接的にでも参照している場合、キーは決して収集されず、メモリリークにつながる可能性があります。弱い値のテーブルでは、その逆が当てはまります。

Scheme Procedure: **weak-key-hash-table?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-weak_002dkey_002dhash_002dtable_003f)

Scheme Procedure: **weak-value-hash-table?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-weak_002dvalue_002dhash_002dtable_003f)

Scheme Procedure: **doubly-weak-hash-table?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-doubly_002dweak_002dhash_002dtable_003f)

C 関数: **scm\_weak\_key\_hash\_table\_p** (obj) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fweak_005fkey_005fhash_005ftable_005fp)

C 関数: **scm\_weak\_value\_hash\_table\_p** (obj) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fweak_005fvalue_005fhash_005ftable_005fp)

C 関数: **scm\_doubly\_weak\_hash\_table\_p** (obj) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fdoubly_005fweak_005fhash_005ftable_005fp)

objが指定された弱ハッシュテーブルである場合は、`#t`を返します。なお、二重弱ハッシュテーブルは、弱キーハッシュテーブルでも弱値ハッシュテーブルでもありません。

* * *

前へ: [弱いハッシュテーブル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Weak-hash-tables)、上へ: [弱い参照](https://doc.guix.gnu.org/guile/latest/en/guile.html#Weak-References) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.17.3.2 弱ベクトル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Weak-vectors-1)

Scheme手順: **make-weak-vector** size \[fill\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dweak_002dvector)

C 関数: **scm\_make\_weak\_vector** (size, fill) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fmake_005fweak_005fvector)

サイズ個の要素を持つ弱ベクトルを返します。オプション引数fillが指定された場合、ベクトルのすべての要素がfillに設定されます。fillのデフォルト値は空のリストです。

Scheme Procedure: **weak-vector** elem … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-weak_002dvector)

Scheme手順: **list->weak-vector** l [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-list_002d_003eweak_002dvector)

C 関数: **scm\_weak\_vector** (l) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fweak_005fvector)

リストから弱いベクトルを構築します。`weak-vector` は引数のリストを使用し、`list->weak-vector` は唯一の引数 l (リスト) を使用して、`list- >vector` と同じ方法で弱いベクトルを構築します。

Scheme手順: **weak-vector?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-weak_002dvector_003f)

C 関数: **scm\_weak\_vector\_p** (obj) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fweak_005fvector_005fp)

objが弱ベクトルである場合は、`#t`を返します。

Scheme Procedure: **weak-vector-ref** wvect k [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-weak_002dvector_002dref)

C 関数: **scm\_weak\_vector\_ref** (wvect, k) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fweak_005fvector_005fref)

弱ベクトル wvect の k 番目の要素を返します。その要素が既に収集されている場合は `#f` を返します。

Scheme Procedure: **weak-vector-set!** wvect k elt [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-weak_002dvector_002dset_0021)

C 関数: **scm\_weak\_vector\_set\_x** (wvect, k, elt) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fweak_005fvector_005fset_005fx)

弱ベクトルwvectのk番目の要素をeltに設定します。

* * *

前へ: [弱い参照](https://doc.guix.gnu.org/guile/latest/en/guile.html#Weak-References)、上へ: [メモリ管理とガベージコレクション](https://doc.guix.gnu.org/guile/latest/en/guile.html#Memory-Management) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.17.4 ガーディアン [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guardians-1)

ガーディアン機能を使うと、本来ならゴミとして回収されてしまうようなオブジェクトについて通知を受け取ることができます。ガーディアンを設定することで、それらのオブジェクトが回収されるのを防ぎ、例えばクリーンアップ処理を実行できるようになります。

R. Kent Dybvig、Carl Bruggeman、および David Eby (1993)「世代ベースのガベージコレクタにおけるガーディアン」を参照。ACM SIGPLAN プログラミング言語設計および実装会議、1993 年 6 月。

Scheme 手順: **make-guardian** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dguardian)

C 関数: **scm\_make\_guardian** () [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fmake_005fguardian)

新しいガーディアンを作成します。ガーディアンは、ガベージコレクションから一連のオブジェクトを保護し、プログラムがクリーンアップなどの処理を実行できるようにします。

`make-guardian` は、ガーディアンを表すプロシージャを返します。ガーディアンプロシージャを引数付きで呼び出すと、引数がガーディアンの保護対象オブジェクトのセットに追加されます。ガーディアンプロシージャを引数なしで呼び出すと、ガベージコレクションの対象となる保護対象オブジェクトのいずれかが返されます。該当するオブジェクトがない場合は `#f` が返されます。このようにして返されたオブジェクトは、ガーディアンから削除されます。

1つのオブジェクトをガーディアンに複数回入れることも、1つのオブジェクトを複数のガーディアンに入れることもできます。その場合、オブジェクトはガーディアンのプロシージャによって複数回返されます。

オブジェクトは、どのガーディアンからも参照されなくなった時点で、ガーディアンから返される資格を得ます。

ガーディアンからオブジェクトが返される順序は保証されません。例えば、ファイナライズ処理に順序を付けたい場合は、他のオブジェクトのファイナライズに不要になるまで、オブジェクトを何らかのグローバルデータ構造に保持しておくことで実現できます。

弱いベクトルの要素であること、弱いキーを持つハッシュテーブルのキーであること、または弱い値を持つハッシュテーブルの値であることは、オブジェクトがガーディアンによって返されることを妨げるものではありません。しかし、オブジェクトがガーディアンから返される可能性がある限り、そのオブジェクトは弱いベクトルやハッシュテーブルから削除されることはありません。言い換えれば、弱いリンクはオブジェクトが収集可能とみなされることを妨げるものではありませんが、ガーディアン内にあることは弱いリンクが切断されることを妨げます。

弱いキーのハッシュテーブルにおけるキーは、そのキーがアクセス可能である限り、関連付けられた値への強い参照を持つと考えることができます。したがって、キーがガーディアン内からのみアクセス可能な場合、キーから値への参照もガーディアン内から来ているとみなされます。そのため、その値への他の参照が存在しない場合、ガーディアンから返される対象となります。

* * *

次へ: [外部関数インターフェース](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Function-Interface)、前: [メモリ管理とガベージコレクション](https://doc.guix.gnu.org/guile/latest/en/guile.html#Memory-Management)、上: [API リファレンス](https://doc.guix.gnu.org/guile/latest/en/guile.html#API-Reference) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
