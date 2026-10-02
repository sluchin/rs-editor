### 6.20 外部オブジェクト

この章では、外部オブジェクトの定義と操作に関する参考情報を提供します。外部オブジェクトに関するチュートリアル形式の入門については、[新しい外部オブジェクトタイプの定義](05_programming_in_c.md#55-新しい外部オブジェクト型の定義)を参照してください。

C 型: **scm\_t\_struct\_finalize**

この関数型は `void` を返し、1 つの `SCM` 引数を取ります。

C 関数: `SCM` **scm\_make\_foreign\_object\_type** `(SCM 名、SCM スロット、scm_t_struct_finalize ファイナライザ)`

新しい外部オブジェクト型を作成します。name は型の名前を表すシンボルです。slots はシンボルのリストで、それぞれが外部オブジェクト型のフィールドの名前を表します。finalizer はファイナライザーを示し、NULL にすることもできます。

ファイナライザは可能な限り使用しないことをお勧めします。[外部オブジェクトメモリ管理](05_programming_in_c.md#554-外部オブジェクトメモリ管理)を参照してください。ファイナライザは非同期セーフかつスレッドセーフである必要があります。これも[外部オブジェクトメモリ管理](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Object-Memory-Management )を参照してください。スレッドセーフではないアプリケーションに Guile を組み込んでいて、ファイナライズが必要な外部オブジェクト型を定義している場合は、自動ファイナライズを無効にして、`scm_manually_run_finalizers()` を自分で呼び出すように設定することをお勧めします。

C 関数: `int` **scm\_set\_automatic\_finalization\_enabled** `(int enabled_p)`

自動ファイナライゼーションを有効または無効にします。デフォルトでは、Guile はオブジェクトファイナライザを自動的に呼び出し、可能であれば別のスレッドで実行します。enabled_p にゼロ値を渡すと、Guile 全体の自動ファイナライゼーションが無効になります。自動ファイナライゼーションを無効にした場合は、定期的に `scm_run_finalizers()` を呼び出す必要があります。

他のほとんどのGuile関数とは異なり、`scm_set_automatic_finalization_enabled`はGuileが初期化される前に呼び出すことができます。

自動ファイナライズの以前の状態を返します。

C 関数: `int` **scm\_run\_finalizers** `(void)`

保留中のファイナライザをすべて呼び出します。呼び出されたファイナライザの数を返します。この関数は、自動ファイナライゼーションが無効になっている場合に呼び出す必要がありますが、有効になっている場合でも呼び出される可能性があります。

C 関数: `void` **scm\_assert\_foreign\_object\_type** `(SCM type, SCM val)`

valが指定された型の外部オブジェクトである場合は、何も行わない。そうでない場合は、エラーを通知する。

C 関数: `SCM` **scm\_make\_foreign\_object\_0** `(SCM タイプ)`

C 関数: `SCM` **scm\_make\_foreign\_object\_1** `(SCM 型、void *val0)`

C 関数: `SCM` **scm\_make\_foreign\_object\_2** `(SCM 型、void *val0、void *val1)`

C 関数: `SCM` **scm\_make\_foreign\_object\_3** `(SCM 型、void *val0、void *val1、void *val2)`

C 関数: `SCM` **scm\_make\_foreign\_object\_n** `(SCM 型、size_t n、void *vals[])`

指定された型の新しい外部オブジェクトを作成し、最初の n 個のフィールドを適切な値で初期化します。

特定の型のオブジェクトのフィールド数は、型が作成される時点で固定されます。値のフィールド数よりも多くの初期化子を指定するとエラーになります。必要な数よりも少ない初期化子を指定することは全く問題ありません。これは、一部のフィールドがポインタ型ではない場合に便利で、後述するセッターを使用して初期化しやすくなります。

C 関数: `void*` **scm\_foreign\_object\_ref** `(SCM obj, size_t n);`

C 関数: `scm_t_bits` **scm\_foreign\_object\_unsigned\_ref** `(SCM obj, size_t n);`

C 関数: `scm_t_signed_bits` **scm\_foreign\_object\_signed\_ref** `(SCM obj, size_t n);`

外部オブジェクト obj の n 番目のフィールドの値を返します。フィールドのバッキングストアの幅は `scm_t_bits` 値と同じで、これはポインタと同じ幅以上です。さまざまなバリアントは、移植性の高い方法で型変換を処理します。

C 関数: `void` **scm\_foreign\_object\_set\_x** `(SCM obj, size_t n, void *val);`

C 関数: `void` **scm\_foreign\_object\_unsigned\_set\_x** `(SCM obj, size_t n, scm_t_bits val);`

C 関数: `void` **scm\_foreign\_object\_signed\_set\_x** `(SCM obj, size_t n, scm_t_signed_bits val);`

必要に応じて、移植可能な方法で`scm_t_bits`値に変換した後、外部オブジェクトobjのn番目のフィールドの値をvalに設定します。

Scheme から外部オブジェクトにアクセスすることもできます。例については、[Foreign Objects and Scheme](05_programming_in_c.md#555-外部オブジェクトとスキーム) を参照してください。

(use-modules (system foreign-object))

Scheme 手順: **make-foreign-object-type** name slots \[#:finalizer=#f\] \[#:supers='()\]

新しい外部オブジェクト型を作成します。`scm_make_foreign_object_type` については上記のドキュメントを参照してください。これらの関数は、ファイナライザがインスタンスにアタッチされる方法（内部の詳細）と、この関数がオプションでスーパークラスのリストを受け取り、それが `make-class` に渡されるという点を除いて、まったく同じです。

結果として得られる値はGOOPSクラスです。Guileのクラスの詳細については、[GOOPS](08_00_goops.md#8-goops)を参照してください。

Scheme構文: **define-foreign-object-type** 名前 コンストラクタ (スロット ...) \[#:finalizer=#f\]

`make-foreign-object-type` を使用して型を定義し、それを名前にバインドするための便利なマクロです。コンストラクタはコンストラクタにバインドされ、ゲッターは各スロットにバインドされます。

* * *

次へ: [スレッド、ミューテックス、非同期処理、動的ルート](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#622-スレッドミューテックス非同期処理および動的ルート)、前: [外部オブジェクト](#620-外部オブジェクト)、上: [API リファレンス](06_00_api_reference.md#6-apiリファレンス) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
