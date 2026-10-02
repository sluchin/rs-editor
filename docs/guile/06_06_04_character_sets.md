# 6.6.4 文字集合

> **原文**: [Guile Reference Manual - Character Sets](https://www.gnu.org/software/guile/manual/html_node/Character-Sets.html)
>
> このドキュメントは GNU Free Documentation License の下で公開されている原文の翻訳です。

この節で説明する機能は、SRFI-14 に直接対応しています。

データ型 `charset` は文字の集合を実装します（「文字」を参照）。文字集合の内部表現はユーザーからは見えないため、それらを扱うための多くの手続きが提供されています。

文字集合は、作成したり、拡張したり、文字の所属をテストしたり、他の文字集合と比較したりできます。

- 文字集合の述語/比較
- 文字集合の反復
- 文字集合の作成
- 文字集合の照会
- 文字集合の代数
- 標準の文字集合

## 6.6.4.1 文字集合の述語/比較

これらの手続きは、オブジェクトが文字集合であるかどうか、あるいは複数の文字集合が等しいか互いの部分集合であるかどうかをテストするために使います。`char-set-hash` は、高速な検索手続きで使うためなどに、ハッシュ値を計算するために使用できます。

**Scheme 手続き: `char-set? obj`**<br>**C 関数: `scm_char_set_p (obj)`**
: `obj` が文字集合であれば `#t` を、そうでなければ `#f` を返します。

**Scheme 手続き: `char-set= char_set …`**<br>**C 関数: `scm_char_set_eq (char_sets)`**
: 与えられたすべての文字集合が等しければ `#t` を返します。

**Scheme 手続き: `char-set<= char_set …`**<br>**C 関数: `scm_char_set_leq (char_sets)`**
: すべての文字集合 char_set_i が文字集合 char_set_i+1 の部分集合であれば `#t` を返します。

**Scheme 手続き: `char-set-hash cs [bound]`**<br>**C 関数: `scm_char_set_hash (cs, bound)`**
: 文字集合 `cs` のハッシュ値を計算します。`bound` が与えられてゼロでない場合、返される値を 0 … `bound` - 1 の範囲に制限します。

## 6.6.4.2 文字集合の反復

文字集合カーソルは、文字集合のメンバーを反復するための手段です。`char-set-cursor` で文字集合カーソルを作成した後、カーソルは `char-set-ref` で参照外しでき、`char-set-cursor-next` で次のメンバーに進めることができます。カーソルが集合の最後の要素を過ぎたかどうかは、`end-of-char-set?` で確認できます。

さらに、文字集合のための写像と畳み込み（およびその逆）の手続きも提供されています。

**Scheme 手続き: `char-set-cursor cs`**<br>**C 関数: `scm_char_set_cursor (cs)`**
: 文字集合 `cs` へのカーソルを返します。

**Scheme 手続き: `char-set-ref cs cursor`**<br>**C 関数: `scm_char_set_ref (cs, cursor)`**
: 文字集合 `cs` の中の現在のカーソル位置 `cursor` にある文字を返します。`end-of-char-set?` が真を返すカーソルを渡すのはエラーです。

**Scheme 手続き: `char-set-cursor-next cs cursor`**<br>**C 関数: `scm_char_set_cursor_next (cs, cursor)`**
: 文字集合カーソル `cursor` を、文字集合 `cs` の次の文字に進めます。与えられたカーソルが `end-of-char-set?` を満たす場合はエラーです。

**Scheme 手続き: `end-of-char-set? cursor`**<br>**C 関数: `scm_end_of_char_set_p (cursor)`**
: `cursor` が文字集合の終わりに達していれば `#t` を、そうでなければ `#f` を返します。

**Scheme 手続き: `char-set-fold kons knil cs`**<br>**C 関数: `scm_char_set_fold (kons, knil, cs)`**
: 手続き `kons` を文字集合 `cs` の上で畳み込みます。初期値は `knil` です。

**Scheme 手続き: `char-set-unfold p f g seed [base_cs]`**<br>**C 関数: `scm_char_set_unfold (p, f, g, seed, base_cs)`**
: これは文字集合の基本的なコンストラクタです。

  - `g` は、初期シード `seed` から「シード」値の列を生成するために使われます: `seed`, (`g` `seed`), (`g`^2 `seed`), (`g`^3 `seed`), …
  - `p` はいつ停止するかを伝えます――シード値の一つに適用されたときに真を返したときです。
  - `f` は各シード値を文字に写像します。これらの文字は基本の文字集合 `base_cs` に追加されて結果を形成します。`base_cs` のデフォルトは空集合です。

**Scheme 手続き: `char-set-unfold! p f g seed base_cs`**<br>**C 関数: `scm_char_set_unfold_x (p, f, g, seed, base_cs)`**
: これは文字集合の基本的なコンストラクタです。

  - `g` は、初期シード `seed` から「シード」値の列を生成するために使われます: `seed`, (`g` `seed`), (`g`^2 `seed`), (`g`^3 `seed`), …
  - `p` はいつ停止するかを伝えます――シード値の一つに適用されたときに真を返したときです。
  - `f` は各シード値を文字に写像します。これらの文字は基本の文字集合 `base_cs` に追加されて結果を形成します。`base_cs` のデフォルトは空集合です。

**Scheme 手続き: `char-set-for-each proc cs`**<br>**C 関数: `scm_char_set_for_each (proc, cs)`**
: 文字集合 `cs` のすべての文字に `proc` を適用します。戻り値は規定されていません。

**Scheme 手続き: `char-set-map proc cs`**<br>**C 関数: `scm_char_set_map (proc, cs)`**
: 手続き `proc` を `cs` のすべての文字に写像します。`proc` は文字から文字への手続きでなければなりません。

## 6.6.4.3 文字集合の作成

新しい文字集合はこれらの手続きで生成されます。

**Scheme 手続き: `char-set-copy cs`**<br>**C 関数: `scm_char_set_copy (cs)`**
: `cs` のすべての文字を含む、新しく割り当てられた文字集合を返します。

**Scheme 手続き: `char-set chr …`**<br>**C 関数: `scm_char_set (chrs)`**
: 与えられたすべての文字を含む文字集合を返します。

**Scheme 手続き: `list->char-set list [base_cs]`**<br>**C 関数: `scm_list_to_char_set (list, base_cs)`**
: 文字のリスト `list` を文字集合に変換します。文字集合 `base_cs` が与えられた場合、この集合の文字も結果に含まれます。

**Scheme 手続き: `list->char-set! list base_cs`**<br>**C 関数: `scm_list_to_char_set_x (list, base_cs)`**
: 文字のリスト `list` を文字集合に変換します。文字は `base_cs` に追加され、`base_cs` が返されます。

**Scheme 手続き: `string->char-set str [base_cs]`**<br>**C 関数: `scm_string_to_char_set (str, base_cs)`**
: 文字列 `str` を文字集合に変換します。文字集合 `base_cs` が与えられた場合、この集合の文字も結果に含まれます。

**Scheme 手続き: `string->char-set! str base_cs`**<br>**C 関数: `scm_string_to_char_set_x (str, base_cs)`**
: 文字列 `str` を文字集合に変換します。文字列の文字は `base_cs` に追加され、`base_cs` が返されます。

**Scheme 手続き: `char-set-filter pred cs [base_cs]`**<br>**C 関数: `scm_char_set_filter (pred, cs, base_cs)`**
: `cs` の文字のうち `pred` を満たすすべての文字を含む文字集合を返します。与えられた場合、`base_cs` の文字が結果に追加されます。

**Scheme 手続き: `char-set-filter! pred cs base_cs`**<br>**C 関数: `scm_char_set_filter_x (pred, cs, base_cs)`**
: `cs` の文字のうち `pred` を満たすすべての文字を含む文字集合を返します。文字は `base_cs` に追加され、`base_cs` が返されます。

**Scheme 手続き: `ucs-range->char-set lower upper [error [base_cs]]`**<br>**C 関数: `scm_ucs_range_to_char_set (lower, upper, error, base_cs)`**
: 文字コードが半開区間 [`lower`,`upper`) にあるすべての文字を含む文字集合を返します。

  `error` が真の値の場合、指定された範囲に実装されている文字の範囲に含まれない文字が含まれていれば、エラーが通知されます。`error` が `#f` の場合、これらの文字は結果の文字集合から黙って除外されます。

  与えられた場合、`base_cs` の文字が結果に追加されます。

**Scheme 手続き: `ucs-range->char-set! lower upper error base_cs`**<br>**C 関数: `scm_ucs_range_to_char_set_x (lower, upper, error, base_cs)`**
: 文字コードが半開区間 [`lower`,`upper`) にあるすべての文字を含む文字集合を返します。

  `error` が真の値の場合、指定された範囲に実装されている文字の範囲に含まれない文字が含まれていれば、エラーが通知されます。`error` が `#f` の場合、これらの文字は結果の文字集合から黙って除外されます。

  文字は `base_cs` に追加され、`base_cs` が返されます。

**Scheme 手続き: `->char-set x`**<br>**C 関数: `scm_to_char_set (x)`**
: `x` を文字集合に強制変換します。`x` は文字列、文字、または文字集合でかまいません。文字列はそれを構成する文字の集合に変換され、文字は要素が1つの集合に変換され、文字集合はそのまま返されます。

## 6.6.4.4 文字集合の照会

これらの手続きで、文字集合の要素やその他の情報にアクセスします。

**Scheme 手続き: `%char-set-dump cs`**
: `cs` のデバッグ情報を含む連想リストを返します。連想リストには次のエントリがあります。

  `char-set`
  : 文字集合そのもの

  `len`
  : 文字集合が含む、連続したコードポイントのグループの数

  `ranges`
  : リストのリストで、各サブリストはコードポイントの範囲とそれに関連する文字

  この関数の戻り値は、Guile のバージョン間で一貫していることを当てにすることはできず、コードの中で使うべきではありません。

**Scheme 手続き: `char-set-size cs`**<br>**C 関数: `scm_char_set_size (cs)`**
: 文字集合 `cs` の要素数を返します。

**Scheme 手続き: `char-set-count pred cs`**<br>**C 関数: `scm_char_set_count (pred, cs)`**
: 文字集合 `cs` の要素のうち、述語 `pred` を満たすものの数を返します。

**Scheme 手続き: `char-set->list cs`**<br>**C 関数: `scm_char_set_to_list (cs)`**
: 文字集合 `cs` の要素を含むリストを返します。

**Scheme 手続き: `char-set->string cs`**<br>**C 関数: `scm_char_set_to_string (cs)`**
: 文字集合 `cs` の要素を含む文字列を返します。文字が文字列に置かれる順序は定義されていません。

**Scheme 手続き: `char-set-contains? cs ch`**<br>**C 関数: `scm_char_set_contains_p (cs, ch)`**
: 文字 `ch` が文字集合 `cs` に含まれていれば `#t` を、そうでなければ `#f` を返します。

**Scheme 手続き: `char-set-every pred cs`**<br>**C 関数: `scm_char_set_every (pred, cs)`**
: 文字集合 `cs` のすべての文字が述語 `pred` を満たせば、真の値を返します。

**Scheme 手続き: `char-set-any pred cs`**<br>**C 関数: `scm_char_set_any (pred, cs)`**
: 文字集合 `cs` のいずれかの文字が述語 `pred` を満たせば、真の値を返します。

## 6.6.4.5 文字集合の代数

文字集合は、和集合、補集合、共通部分などの一般的な集合代数の操作で操作できます。これらの手続きはすべて、文字集合の引数を変更する副作用のある変形を提供しています。

**Scheme 手続き: `char-set-adjoin cs chr …`**<br>**C 関数: `scm_char_set_adjoin (cs, chrs)`**
: すべての文字の引数を最初の引数に追加します。最初の引数は文字集合でなければなりません。

**Scheme 手続き: `char-set-delete cs chr …`**<br>**C 関数: `scm_char_set_delete (cs, chrs)`**
: すべての文字の引数を最初の引数から削除します。最初の引数は文字集合でなければなりません。

**Scheme 手続き: `char-set-adjoin! cs chr …`**<br>**C 関数: `scm_char_set_adjoin_x (cs, chrs)`**
: すべての文字の引数を最初の引数に追加します。最初の引数は文字集合でなければなりません。

**Scheme 手続き: `char-set-delete! cs chr …`**<br>**C 関数: `scm_char_set_delete_x (cs, chrs)`**
: すべての文字の引数を最初の引数から削除します。最初の引数は文字集合でなければなりません。

**Scheme 手続き: `char-set-complement cs`**<br>**C 関数: `scm_char_set_complement (cs)`**
: 文字集合 `cs` の補集合を返します。

文字集合の補集合には、多くの予約済みコードポイント（文字に関連付けられていないコードポイント）が含まれる可能性が高いことに注意してください。`char-set-complement` の出力を、指定済みコードポイントの集合 `char-set:designated` との共通部分を計算することで修正すると役立つかもしれません。

**Scheme 手続き: `char-set-union cs …`**<br>**C 関数: `scm_char_set_union (char_sets)`**
: すべての引数の文字集合の和集合を返します。

**Scheme 手続き: `char-set-intersection cs …`**<br>**C 関数: `scm_char_set_intersection (char_sets)`**
: すべての引数の文字集合の共通部分を返します。

**Scheme 手続き: `char-set-difference cs1 cs …`**<br>**C 関数: `scm_char_set_difference (cs1, char_sets)`**
: すべての引数の文字集合の差を返します。

**Scheme 手続き: `char-set-xor cs …`**<br>**C 関数: `scm_char_set_xor (char_sets)`**
: すべての引数の文字集合の排他的論理和を返します。

**Scheme 手続き: `char-set-diff+intersection cs1 cs …`**<br>**C 関数: `scm_char_set_diff_plus_intersection (cs1, char_sets)`**
: すべての引数の文字集合の差と共通部分を返します。

**Scheme 手続き: `char-set-complement! cs`**<br>**C 関数: `scm_char_set_complement_x (cs)`**
: 文字集合 `cs` の補集合を返します。

**Scheme 手続き: `char-set-union! cs1 cs …`**<br>**C 関数: `scm_char_set_union_x (cs1, char_sets)`**
: すべての引数の文字集合の和集合を返します。

**Scheme 手続き: `char-set-intersection! cs1 cs …`**<br>**C 関数: `scm_char_set_intersection_x (cs1, char_sets)`**
: すべての引数の文字集合の共通部分を返します。

**Scheme 手続き: `char-set-difference! cs1 cs …`**<br>**C 関数: `scm_char_set_difference_x (cs1, char_sets)`**
: すべての引数の文字集合の差を返します。

**Scheme 手続き: `char-set-xor! cs1 cs …`**<br>**C 関数: `scm_char_set_xor_x (cs1, char_sets)`**
: すべての引数の文字集合の排他的論理和を返します。

**Scheme 手続き: `char-set-diff+intersection! cs1 cs2 cs …`**<br>**C 関数: `scm_char_set_diff_plus_intersection_x (cs1, cs2, char_sets)`**
: すべての引数の文字集合の差と共通部分を返します。

## 6.6.4.6 標準の文字集合

文字集合のデータ型と手続きの使用を有用なものにするために、定義済みの文字集合の変数がいくつか存在します。

これらの文字集合はロケールに依存せず、`setlocale` の呼び出しによって再計算されることはありません。これらは Unicode のコードポイントの全範囲からの文字を含んでいます。たとえば、`char-set:letter` は約10万個の文字を含んでいます。

**Scheme 変数: `char-set:lower-case`**<br>**C 変数: `scm_char_set_lower_case`**
: すべての小文字。

**Scheme 変数: `char-set:upper-case`**<br>**C 変数: `scm_char_set_upper_case`**
: すべての大文字。

**Scheme 変数: `char-set:title-case`**<br>**C 変数: `scm_char_set_title_case`**
: 大文字の後に小文字が続いているかのように機能する、すべての単一の文字。

**Scheme 変数: `char-set:letter`**<br>**C 変数: `scm_char_set_letter`**
: すべての文字（letter）。これには `char-set:lower-case`、`char-set:upper-case`、`char-set:title-case`、そして大文字と小文字の区別をまったく持たない多くの文字が含まれます。たとえば、中国語や日本語の文字には通常、大文字と小文字の概念がありません。

**Scheme 変数: `char-set:digit`**<br>**C 変数: `scm_char_set_digit`**
: すべての数字。

**Scheme 変数: `char-set:letter+digit`**<br>**C 変数: `scm_char_set_letter_and_digit`**
: `char-set:letter` と `char-set:digit` の和集合。

**Scheme 変数: `char-set:graphic`**<br>**C 変数: `scm_char_set_graphic`**
: 紙にインクを付けるすべての文字。

**Scheme 変数: `char-set:printing`**<br>**C 変数: `scm_char_set_printing`**
: `char-set:graphic` と `char-set:whitespace` の和集合。

**Scheme 変数: `char-set:whitespace`**<br>**C 変数: `scm_char_set_whitespace`**
: すべての空白文字。

**Scheme 変数: `char-set:blank`**<br>**C 変数: `scm_char_set_blank`**
: すべての水平方向の空白文字。特に `#\space` と `#\tab` を含みます。

**Scheme 変数: `char-set:iso-control`**<br>**C 変数: `scm_char_set_iso_control`**
: ISO 制御文字とは、C0 制御文字（U+0000 から U+001F）、delete（U+007F）、そして C1 制御文字（U+0080 から U+009F）です。

**Scheme 変数: `char-set:punctuation`**<br>**C 変数: `scm_char_set_punctuation`**
: `!"#%&'()*,-./:;?@[\\]_{}` のような、すべての句読点文字。

**Scheme 変数: `char-set:symbol`**<br>**C 変数: `scm_char_set_symbol`**
: ``$+<=>^`|~`` のような、すべての記号文字。

**Scheme 変数: `char-set:hex-digit`**<br>**C 変数: `scm_char_set_hex_digit`**
: 16進数字 `0123456789abcdefABCDEF`。

**Scheme 変数: `char-set:ascii`**<br>**C 変数: `scm_char_set_ascii`**
: すべての ASCII 文字。

**Scheme 変数: `char-set:empty`**<br>**C 変数: `scm_char_set_empty`**
: 空の文字集合。

**Scheme 変数: `char-set:designated`**<br>**C 変数: `scm_char_set_designated`**
: この文字集合はすべての指定済みコードポイントを含みます。これには、Unicode が文字やその他の意味を割り当てたすべてのコードポイントが含まれます。

**Scheme 変数: `char-set:full`**<br>**C 変数: `scm_char_set_full`**
: この文字集合はすべての可能なコードポイントを含みます。これには指定済みと予約済みの両方のコードポイントが含まれます。

---

> **ライセンス**: この翻訳は GNU Free Documentation License v1.3 以降に基づいて作成されています。
> 原文の著作権: Copyright (C) 1996-2023 Free Software Foundation, Inc.
