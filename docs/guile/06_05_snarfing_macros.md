### 6.5 マクロのスナーフィング [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Snarfing-Macros-1)

以下のマクロは 2 つの異なる動作をします。通常コンパイル時には、ある方法で展開されます。スナーフィング中に処理されると、`guile-snarf` プログラムが初期化コードを取得するようになります。[関数スナーフィング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Function-Snarfing) を参照してください。

以下の説明では、「通常」という用語はコードが通常どおりコンパイルされる場合を指し、「スナーフ中」という用語はコードが`guile-snarf`によって処理される場合を指します。

C マクロ: **SCM\_SNARF\_INIT** (コード) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fSNARF_005fINIT)

通常、`SCM_SNARF_INIT` は何も展開されませんが、スナーフィング中は、初期化アクションファイルにコードが挿入され、その後にセミコロンが続きます。

これは初期化アクションを捕捉するための基本マクロです。以下のより特殊なマクロは、内部的にこのマクロを使用します。

C マクロ: **SCM_DEFINE** (c_name, scheme_name, req, opt, var, arglist, docstring) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fDEFINE)

通常、このマクロは次のように展開されます。

static const char s\_c\_name\[\] = scheme\_name;
SCM
c_name 引数リスト

むさぼり食うと、

scm\_c\_define\_gsubr (s\_c\_name、req、opt、var、
c_name);

初期化アクションに追加されます。したがって、これを使用して、c_name という名前の C 関数を宣言し、scheme から scheme_name という名前で利用できるようになります。

引数リストは必ず括弧で囲む必要があることに注意してください。

C マクロ: **SCM\_SYMBOL** (c\_name, scheme\_name) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fSYMBOL)

C マクロ: **SCM\_GLOBAL\_SYMBOL** (c\_name, scheme\_name) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fGLOBAL_005fSYMBOL)

通常、これらのマクロは次のように展開されます。

static SCM c_name

または

SCM c_name

それぞれ。スナーフィング中、両方とも初期化コードに展開されます。

c_name = scm_permanent_object (scm_from_locale_symbol (scheme_name));

したがって、これらを使用して、scheme\_name という名前のシンボルで初期化される `SCM` 型の静的またはグローバル変数を宣言できます。

C マクロ: **SCM_KEYWORD** (c_name, scheme_name) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fKEYWORD)

C マクロ: **SCM\_GLOBAL\_KEYWORD** (c\_name, scheme\_name) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fGLOBAL_005fKEYWORD)

通常、これらのマクロは次のように展開されます。

static SCM c_name

または

SCM c_name

それぞれ。スナーフィング中、両方とも初期化コードに展開されます。

c_name = scm_permanent_object (scm_c_make_keyword (scheme_name));

したがって、これらを使用して、scheme\_name という名前のキーワードで初期化される `SCM` 型の静的またはグローバル変数を宣言できます。

C マクロ: **SCM\_VARIABLE** (c\_name, scheme\_name) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fVARIABLE)

C マクロ: **SCM\_GLOBAL\_VARIABLE** (c\_name, scheme\_name) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fGLOBAL_005fVARIABLE)

これらのマクロは、それぞれ`SCM_VARIABLE_INIT`と`SCM_GLOBAL_VARIABLE_INIT`に相当し、値は`SCM_BOOL_F`です。

C マクロ: **SCM\_VARIABLE\_INIT** (c\_name, scheme\_name, value) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fVARIABLE_005fINIT)

C マクロ: **SCM\_GLOBAL\_VARIABLE\_INIT** (c\_name, scheme\_name, value) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fGLOBAL_005fVARIABLE_005fINIT)

通常、これらのマクロは次のように展開されます。

static SCM c_name

または

SCM c_name

それぞれ。スナーフィング中、両方とも初期化コードに展開されます。

c_name = scm_permanent_object (scm_c_define (scheme_name, value));

したがって、これらを使用して、現在のモジュール内の scheme\_name という名前の Scheme 変数を表すオブジェクトで初期化される、型 `SCM` の静的またはグローバルな C 変数を宣言できます。変数は、まだ存在しない場合に定義されます。常に値に設定されます。

* * *

次へ: [手順](https://doc.guix.gnu.org/guile/latest/en/guile.html#Procedures)、前: [マクロのスナーフィング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Snarfing-Macros)、上: [API リファレンス](https://doc.guix.gnu.org/guile/latest/en/guile.html#API-Reference) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
