#### 6.6.1 ブール値

2つのブール値は、真の場合は`#t`、偽の場合は`#f`です。R7RSに従って、`#true`と`#false`と表記することもできます。

ブール値は、一般的な等価述語である `eq?`、`eqv?`、`equal?` ([Equality](06_09_general_utility_functions.md#691-平等) を参照) や、`string=?` ([String Comparison](06_06_05_strings.md#6657-文字列の比較) および `<=` ([Comparison Predicates](07_05_49_srfi207_stringnotated_bytevectors.md#75496-比較) を参照) などの述語手続きによって返されます。

([<=](06_06_02_numerical_data_types.md#6628-比較述語) 3 8)
⇒ #t

([<=](06_06_02_numerical_data_types.md#6628-比較述語) 3 \-3)
⇒ #f

([equal?](06_09_general_utility_functions.md#691-平等) "house" "houses")
⇒ #f

([eq?](06_09_general_utility_functions.md#691-平等) #f #f)
⇒
#t

`if` や `cond` のようなテスト条件のコンテキスト ([単純な条件評価](06_11_controlling_the_flow_of_program_execution.md#6112-単純な条件評価) を参照) では、条件式が「true」と評価された場合にのみサブ式のグループが評価されます。「true」とは、`#f` 以外のすべての値を意味します。

(#tが「はい」「いいえ」の場合)
⇒ 「はい」

（0の場合は「はい」「いいえ」）
⇒ 「はい」

(#fが「はい」「いいえ」の場合)
⇒ 「いいえ」

この非対称性の結果として、典型的な Scheme ソースコードでは、`#t` よりも `#f` を明示的に使用する頻度が高くなります。`#f` は `if` または `cond` の偽値を表すために必要ですが、`#t` は `if` または `cond` の真値を表すために必要ではありません。

`#f` は他の Scheme の値と等価ではないことに注意することが重要です。特に、`#f` は (C や C++ のように) 数値の 0 とは異なり、(一部の Lisp 方言のように)「空のリスト」とも異なります。

C言語では、Schemeの2つのブール値は、定数`SCM_BOOL_T`（`#t`用）と`SCM_BOOL_F`（`#f`用）として利用できます。ただし、偽値`SCM_BOOL_F`には注意が必要です。C言語の条件式で使用すると、偽値として扱われません。これを判定するには、`scm_is_false`または`scm_is_true`を使用してください。

Scheme Procedure: **not** x

C 関数: **scm\_not** (x)

xが`#f`の場合は`#t`を返し、そうでない場合は`#f`を返します。

スキームプロシージャ: **boolean?** obj

C 関数: **scm\_boolean\_p** (obj)

objが`#t`または`#f`の場合は`#t`を返し、そうでない場合は`#f`を返します。

C マクロ: `SCM` **SCM\_BOOL\_T**

Schemeオブジェクト`#t`の`SCM`表現。

C マクロ: `SCM` **SCM\_BOOL\_F**

Schemeオブジェクト`#f`の`SCM`表現。

C 関数: `int` **scm\_is\_true** `(SCM obj)`

objが`#f`の場合は`0`を返し、そうでない場合は`1`を返します。

C 関数: `int` **scm\_is\_false** `(SCM obj)`

objが`#f`の場合は`1`を返し、そうでない場合は`0`を返します。

C 関数: `int` **scm\_is\_bool** `(SCM obj)`

objが`#t`または`#f`の場合は`1`を返し、それ以外の場合は`0`を返します。

C 関数: `SCM` **scm\_from\_bool** `(int val)`

valが0の場合は`#f`を返し、それ以外の場合は`#t`を返します。

C 関数: `int` **scm\_to\_bool** `(SCM 値)`

valが`SCM_BOOL_T`の場合は`1`を返し、valが`SCM_BOOL_F`の場合は`0`を返し、それ以外の場合は「型が間違っています」というエラーを通知します。

単に`SCM`の値が真偽を判定したい場合は、この関数ではなく`scm_is_true`を使用することをお勧めします。

* * *

次へ: [文字](06_06_03_characters.md#663-文字)、前: [ブール値](#661-ブール値)、上: [データ型](06_06_00_data_types.md#66-データ型) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
