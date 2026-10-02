#### 6.6.1 ブール値 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Booleans-1)

2つのブール値は、真の場合は`#t`、偽の場合は`#f`です。R7RSに従って、`#true`と`#false`と表記することもできます。

ブール値は、一般的な等価述語である `eq?`、`eqv?`、`equal?` ([Equality](https://doc.guix.gnu.org/guile/latest/en/guile.html#Equality) を参照) や、`string=?` ([String Comparison](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Comparison) および `<=` ([Comparison Predicates](https://doc.guix.gnu.org/guile/latest/en/guile.html#Comparison) を参照) などの述語手続きによって返されます。

([<=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003c_003d) 3 8)
⇒ #t

([<=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003c_003d) 3 \-3)
⇒ #f

([equal?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-equal_003f) "house" "houses")
⇒ #f

([eq?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-eq_003f) #f #f)
⇒
#t

`if` や `cond` のようなテスト条件のコンテキスト ([単純な条件評価](https://doc.guix.gnu.org/guile/latest/en/guile.html#Conditionals) を参照) では、条件式が「true」と評価された場合にのみサブ式のグループが評価されます。「true」とは、`#f` 以外のすべての値を意味します。

(#tが「はい」「いいえ」の場合)
⇒ 「はい」

（0の場合は「はい」「いいえ」）
⇒ 「はい」

(#fが「はい」「いいえ」の場合)
⇒ 「いいえ」

この非対称性の結果として、典型的な Scheme ソースコードでは、`#t` よりも `#f` を明示的に使用する頻度が高くなります。`#f` は `if` または `cond` の偽値を表すために必要ですが、`#t` は `if` または `cond` の真値を表すために必要ではありません。

`#f` は他の Scheme の値と等価ではないことに注意することが重要です。特に、`#f` は (C や C++ のように) 数値の 0 とは異なり、(一部の Lisp 方言のように)「空のリスト」とも異なります。

C言語では、Schemeの2つのブール値は、定数`SCM_BOOL_T`（`#t`用）と`SCM_BOOL_F`（`#f`用）として利用できます。ただし、偽値`SCM_BOOL_F`には注意が必要です。C言語の条件式で使用すると、偽値として扱われません。これを判定するには、`scm_is_false`または`scm_is_true`を使用してください。

Scheme Procedure: **not** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-not)

C 関数: **scm\_not** (x) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fnot)

xが`#f`の場合は`#t`を返し、そうでない場合は`#f`を返します。

スキームプロシージャ: **boolean?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-boolean_003f)

C 関数: **scm\_boolean\_p** (obj) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fboolean_005fp)

objが`#t`または`#f`の場合は`#t`を返し、そうでない場合は`#f`を返します。

C マクロ: `SCM` **SCM\_BOOL\_T** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fBOOL_005fT)

Schemeオブジェクト`#t`の`SCM`表現。

C マクロ: `SCM` **SCM\_BOOL\_F** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fBOOL_005fF)

Schemeオブジェクト`#f`の`SCM`表現。

C 関数: `int` **scm\_is\_true** `(SCM obj)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fis_005ftrue)

objが`#f`の場合は`0`を返し、そうでない場合は`1`を返します。

C 関数: `int` **scm\_is\_false** `(SCM obj)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fis_005ffalse)

objが`#f`の場合は`1`を返し、そうでない場合は`0`を返します。

C 関数: `int` **scm\_is\_bool** `(SCM obj)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fis_005fbool)

objが`#t`または`#f`の場合は`1`を返し、それ以外の場合は`0`を返します。

C 関数: `SCM` **scm\_from\_bool** `(int val)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005fbool)

valが0の場合は`#f`を返し、それ以外の場合は`#t`を返します。

C 関数: `int` **scm\_to\_bool** `(SCM 値)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005fbool)

valが`SCM_BOOL_T`の場合は`1`を返し、valが`SCM_BOOL_F`の場合は`0`を返し、それ以外の場合は「型が間違っています」というエラーを通知します。

単に`SCM`の値が真偽を判定したい場合は、この関数ではなく`scm_is_true`を使用することをお勧めします。

* * *

次へ: [文字](https://doc.guix.gnu.org/guile/latest/en/guile.html#Characters)、前: [ブール値](https://doc.guix.gnu.org/guile/latest/en/guile.html#Booleans)、上: [データ型](https://doc.guix.gnu.org/guile/latest/en/guile.html#Data-Types) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
