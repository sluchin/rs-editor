# 6.6 データ型

> **原文**: [Guile Reference Manual - Data Types](https://www.gnu.org/software/guile/manual/html_node/Data-Types.html)
>
> このドキュメントは GNU Free Documentation License の下で公開されている原文の翻訳です。

Guile のデータ型は、問題領域に適用できる表現と機能の強力な組み込みライブラリを形成しています。この章では、Guile に組み込まれているデータ型を、単純なものから複雑なものまで概観します。

- 真偽値
- 数値データ型
- 文字
- 文字集合
- 文字列
- シンボル
- キーワード
- ペア
- リスト
- ベクタ
- ビットベクタ
- バイトベクタ
- 配列
- VList
- レコードの概要
- SRFI-9 レコード
- レコード
- 構造体
- 辞書型
- 連想リスト
- VList ベースのハッシュリスト（VHash）
- ハッシュテーブル
- その他の型

## 6.6.1 真偽値

2つの真偽値は、真を表す `#t` と偽を表す `#f` です。R7RS に従って、`#true` と `#false` と書くこともできます。

真偽値は、一般的な等価性の述語 `eq?`、`eqv?`、`equal?`（「等価性」を参照）や、`string=?`（「文字列の比較」を参照）や `<=`（「比較述語」を参照）のような数値や文字列の比較演算子などの述語手続きによって返されます。

```scheme
(<= 3 8)
⇒ #t

(<= 3 -3)
⇒ #f

(equal? "house" "houses")
⇒ #f

(eq? #f #f)
⇒
#t
```

`if` や `cond`（「単純な条件付き評価」を参照）のような、条件式が「真」に評価された場合にのみ部分式のグループが評価されるテスト条件の文脈では、「真」は `#f` 以外のあらゆる値を意味します。

```scheme
(if #t "yes" "no")
⇒ "yes"

(if 0 "yes" "no")
⇒ "yes"

(if #f "yes" "no")
⇒ "no"
```

この非対称性の結果として、典型的な Scheme のソースコードでは `#t` よりも `#f` を明示的に使うことが多くなります。`#f` は `if` や `cond` の偽の値を表すために必要ですが、`#t` は `if` や `cond` の真の値を表すために必要ではないからです。

`#f` は他のいかなる Scheme 値とも等価ではないことに注意することが重要です。特に、`#f` は（C や C++ のように）数値 0 と同じではなく、また（一部の Lisp 方言のように）「空リスト」とも同じではありません。

C では、2つの Scheme の真偽値は、`#t` を表す `SCM_BOOL_T` と `#f` を表す `SCM_BOOL_F` という2つの定数として利用できます。偽の値 `SCM_BOOL_F` には注意が必要です。C の条件文で使ったとき、それは偽ではありません。それをテストするには、`scm_is_false` または `scm_is_true` を使用してください。

**Scheme 手続き: `not x`**<br>**C 関数: `scm_not (x)`**
: `x` が `#f` であれば `#t` を返し、そうでなければ `#f` を返します。

**Scheme 手続き: `boolean? obj`**<br>**C 関数: `scm_boolean_p (obj)`**
: `obj` が `#t` または `#f` のいずれかであれば `#t` を返し、そうでなければ `#f` を返します。

**C マクロ: `SCM SCM_BOOL_T`**
: Scheme オブジェクト `#t` の `SCM` 表現です。

**C マクロ: `SCM SCM_BOOL_F`**
: Scheme オブジェクト `#f` の `SCM` 表現です。

**C 関数: `int scm_is_true (SCM obj)`**
: `obj` が `#f` であれば `0` を返し、そうでなければ `1` を返します。

**C 関数: `int scm_is_false (SCM obj)`**
: `obj` が `#f` であれば `1` を返し、そうでなければ `0` を返します。

**C 関数: `int scm_is_bool (SCM obj)`**
: `obj` が `#t` または `#f` のいずれかであれば `1` を返し、そうでなければ `0` を返します。

**C 関数: `SCM scm_from_bool (int val)`**
: `val` が `0` であれば `#f` を返し、そうでなければ `#t` を返します。

**C 関数: `int scm_to_bool (SCM val)`**
: `val` が `SCM_BOOL_T` であれば `1` を返し、`val` が `SCM_BOOL_F` であれば `0` を返し、そうでなければ「wrong type」エラーを通知します。

  単に `SCM` 値の真偽をテストしたいだけの場合は、この関数の代わりにおそらく `scm_is_true` を使うべきです。

---

> **ライセンス**: この翻訳は GNU Free Documentation License v1.3 以降に基づいて作成されています。
> 原文の著作権: Copyright (C) 1996-2023 Free Software Foundation, Inc.
