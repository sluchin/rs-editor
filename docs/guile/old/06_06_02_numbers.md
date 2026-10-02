# 6.6.2 数値データ型

> **原文**: [Guile Reference Manual - Numbers](https://www.gnu.org/software/guile/manual/html_node/Numbers.html)
>
> このドキュメントは GNU Free Documentation License の下で公開されている原文の翻訳です。

Guile は、整数、有理数、実数、複素数という豊かな数値型の「塔」をサポートしており、数値データを操作するための数学関数と科学関数の広範な集合を提供しています。マニュアルのこの節では、それらの型と関数について文書化します。

R5RS における Scheme の数の説明を読むのも啓発的かもしれません。それは特に明快で分かりやすいものです。『R5RS』の「Numbers」を参照してください。

- Scheme の数値の「塔」
- 整数
- 実数と有理数
- 複素数
- 正確数と非正確数
- 数値データの読み取り構文
- 整数値に対する操作
- 比較述語
- 数値と文字列の相互変換
- 複素数の操作
- 算術関数
- 科学関数
- ビット演算
- 乱数生成

## 6.6.2.1 Scheme の数値の「塔」

Scheme の数値の「塔」は、次の数のカテゴリで構成されています。

整数（integers）
: 正または負の整数。例: –5、0、18。

有理数（rationals）
: p と q を整数として p/q と表現できる数の集合。例: 9/16 は該当しますが、pi（無理数）は該当しません。これには整数（n/1）が含まれます。

実数（real numbers）
: 1次元の直線上のあらゆる可能な位置を記述する数の集合。これには有理数と無理数が含まれます。

複素数（complex numbers）
: 2次元空間のあらゆる可能な位置を記述する数の集合。これには実数と虚数（a+bi、ここで a は実部、b は虚部、i は −1 の平方根）が含まれます。

これが塔と呼ばれるのは、各カテゴリがその次のカテゴリの「上に乗っている」からです。つまり、すべての整数は有理数でもあり、すべての有理数は実数でもあり、すべての実数は（虚部がゼロの）複素数でもある、という意味においてです。

整数、有理数、実数、複素数への分類に加えて、Scheme は数が正確に表現されているかどうかも区別します。たとえば、2*sin(pi/4) の結果は正確には 2^(1/2) ですが、Guile は pi/4 も 2^(1/2) も正確には表現できません。代わりに、C の型 `double` を使って、非正確な近似値を格納します。

Guile は、任意の大きさの正確な有理数、C の `double` に収まる非正確な有理数、そして `double` の実部と虚部を持つ非正確な複素数を表現できます。

`number?` 述語は、任意の Scheme 値に適用して、その値がサポートされている数値型のいずれかであるかどうかを調べることができます。

**Scheme 手続き: `number? obj`**<br>**C 関数: `scm_number_p (obj)`**
: `obj` が何らかの種類の数であれば `#t` を、そうでなければ `#f` を返します。

例:

```scheme
(number? 3)
⇒ #t

(number? "hello there!")
⇒ #f

(define pi 3.141592654)
(number? pi)
⇒ #t
```

**C 関数: `int scm_is_number (SCM obj)`**
: これは `scm_is_true (scm_number_p (obj))` と同等です。

以降のいくつかの小節では、Guile の数値データ型それぞれについて詳しく文書化します。

## 6.6.2.2 整数

整数とは、2、83、−3789 のように、小数部分を持たない数のことです。

次の例が示すように、Guile の整数は任意に大きくすることができます。

```scheme
(define (factorial n)
  (let loop ((n n) (product 1))
    (if (= n 0)
        product
        (loop (- n 1) (* product n)))))

(factorial 3)
⇒ 6

(factorial 20)
⇒ 2432902008176640000

(- (factorial 45))
⇒ -119622220865480194561963161495657715064383733760000000000
```

整数がわずか4バイトまたは8バイトのメモリに収まる必要があるために制限されているプログラミング言語の経験がある読者は、これを驚くべきことと感じたり、Guile の整数表現が非効率なのではないかと疑ったりするかもしれません。実際には、Guile は可能な場合にはホストコンピュータのネイティブな整数表現を使い、必要な数がネイティブな形式に収まらない場合にはより一般的な表現を使うことで、利便性と効率のほぼ最適なバランスを実現しています。これら2つの表現の間の変換は自動的であり、Scheme レベルのプログラマからは完全に見えません。

C には多数の異なる整数型があり、Guile はそれらと `SCM` 表現との間で変換するための多数の関数を提供しています。たとえば、C の `int` は `scm_to_int` と `scm_from_int` で扱うことができます。Guile はまた、システム間の違いに対処するのを助けるために、独自の C 整数型もいくつか定義しています。

対応していない C の整数型は、符号付きの型については汎用の `scm_to_signed_integer` と `scm_from_signed_integer` で、符号なしの型については `scm_to_unsigned_integer` と `scm_from_unsigned_integer` で扱うことができます。

Scheme の整数は正確にも非正確にもなりえます。たとえば、明示的な小数点を付けて `3.0` と書かれた数は非正確ですが、整数でもあります。関数 `integer?` と `scm_is_integer` はそのような数に対して真を報告しますが、関数 `exact-integer?`、`scm_is_exact_integer`、`scm_is_signed_integer`、`scm_is_unsigned_integer` は正確な整数のみを許可するので、偽を報告します。同様に、`scm_to_signed_integer` のような変換関数は正確な整数のみを受け付けます。

この動作の動機は、数の非正確性が黙って失われるべきではないということです。非正確な整数を許可したい場合は、`inexact->exact` またはそれに相当する C の `scm_inexact_to_exact` の呼び出しを明示的に挿入できます。（この呼び出しによって正確な整数に変換されるのは非正確な整数だけです。非正確な非整数は正確な分数になります。）

**Scheme 手続き: `integer? x`**<br>**C 関数: `scm_integer_p (x)`**
: `x` が正確または非正確な整数であれば `#t` を返し、そうでなければ `#f` を返します。

  ```scheme
  (integer? 487)
  ⇒ #t

  (integer? 3.0)
  ⇒ #t

  (integer? -3.4)
  ⇒ #f

  (integer? +inf.0)
  ⇒ #f
  ```

**C 関数: `int scm_is_integer (SCM x)`**
: これは `scm_is_true (scm_integer_p (x))` と同等です。

**Scheme 手続き: `exact-integer? x`**<br>**C 関数: `scm_exact_integer_p (x)`**
: `x` が正確な整数であれば `#t` を返し、そうでなければ `#f` を返します。

  ```scheme
  (exact-integer? 37)
  ⇒ #t

  (exact-integer? 3.0)
  ⇒ #f
  ```

**C 関数: `int scm_is_exact_integer (SCM x)`**
: これは `scm_is_true (scm_exact_integer_p (x))` と同等です。

**C 型: `scm_t_int8`**<br>**C 型: `scm_t_uint8`**<br>**C 型: `scm_t_int16`**<br>**C 型: `scm_t_uint16`**<br>**C 型: `scm_t_int32`**<br>**C 型: `scm_t_uint32`**<br>**C 型: `scm_t_int64`**<br>**C 型: `scm_t_uint64`**<br>**C 型: `scm_t_intmax`**<br>**C 型: `scm_t_uintmax`**
: これらの C の型は対応する ISO C の型と同等ですが、すべてのプラットフォームで定義されています。ただし `scm_t_int64` と `scm_t_uint64` は例外で、これらは64ビットの型が利用可能な場合にのみ定義されます。たとえば、`scm_t_int8` は `int8_t` と同等です。

  これらの定義は、すべてのプラットフォームがこれらの型を提供するまでの暫定措置と見なすことができます。関心のあるすべてのプラットフォームがすでにこれらの型を提供していることが分かっているなら、Guile が提供する型の代わりにそれらを直接使うほうがよいでしょう。

**C 関数: `int scm_is_signed_integer (SCM x, scm_t_intmax min, scm_t_intmax max)`**<br>**C 関数: `int scm_is_unsigned_integer (SCM x, scm_t_uintmax min, scm_t_uintmax max)`**
: `x` が `min` 以上 `max` 以下の正確な整数を表している場合に `1` を返します。

  これらの関数は、`SCM` 値が、特定の C の整数型の範囲のような、与えられた範囲に収まるかどうかを確認するために使用できます。単に `SCM` 値を特定の C の整数型に変換したいだけなら、変換関数のいずれかを直接使ってください。

**C 関数: `scm_t_intmax scm_to_signed_integer (SCM x, scm_t_intmax min, scm_t_intmax max)`**<br>**C 関数: `scm_t_uintmax scm_to_unsigned_integer (SCM x, scm_t_uintmax min, scm_t_uintmax max)`**
: `x` が `min` 以上 `max` 以下の正確な整数を表している場合、その整数を返します。そうでなければエラーを通知します。`x` が正確な整数でない場合は「wrong-type」エラー、与えられた範囲に収まらない場合は「out-of-range」エラーです。

**C 関数: `SCM scm_from_signed_integer (scm_t_intmax x)`**<br>**C 関数: `SCM scm_from_unsigned_integer (scm_t_uintmax x)`**
: 整数 `x` を表す `SCM` 値を返します。この関数は常に成功し、常に正確な数を返します。

**C 関数: `char scm_to_char (SCM x)`**<br>**C 関数: `signed char scm_to_schar (SCM x)`**<br>**C 関数: `unsigned char scm_to_uchar (SCM x)`**<br>**C 関数: `short scm_to_short (SCM x)`**<br>**C 関数: `unsigned short scm_to_ushort (SCM x)`**<br>**C 関数: `int scm_to_int (SCM x)`**<br>**C 関数: `unsigned int scm_to_uint (SCM x)`**<br>**C 関数: `long scm_to_long (SCM x)`**<br>**C 関数: `unsigned long scm_to_ulong (SCM x)`**<br>**C 関数: `long long scm_to_long_long (SCM x)`**<br>**C 関数: `unsigned long long scm_to_ulong_long (SCM x)`**<br>**C 関数: `size_t scm_to_size_t (SCM x)`**<br>**C 関数: `ssize_t scm_to_ssize_t (SCM x)`**<br>**C 関数: `scm_t_uintptr scm_to_uintptr_t (SCM x)`**<br>**C 関数: `scm_t_ptrdiff scm_to_ptrdiff_t (SCM x)`**<br>**C 関数: `scm_t_int8 scm_to_int8 (SCM x)`**<br>**C 関数: `scm_t_uint8 scm_to_uint8 (SCM x)`**<br>**C 関数: `scm_t_int16 scm_to_int16 (SCM x)`**<br>**C 関数: `scm_t_uint16 scm_to_uint16 (SCM x)`**<br>**C 関数: `scm_t_int32 scm_to_int32 (SCM x)`**<br>**C 関数: `scm_t_uint32 scm_to_uint32 (SCM x)`**<br>**C 関数: `scm_t_int64 scm_to_int64 (SCM x)`**<br>**C 関数: `scm_t_uint64 scm_to_uint64 (SCM x)`**<br>**C 関数: `scm_t_intmax scm_to_intmax (SCM x)`**<br>**C 関数: `scm_t_uintmax scm_to_uintmax (SCM x)`**<br>**C 関数: `scm_t_intptr scm_to_intptr_t (SCM x)`**<br>**C 関数: `scm_t_uintptr scm_to_uintptr_t (SCM x)`**
: `x` が示された C の型に収まる正確な整数を表している場合、その整数を返します。そうでなければエラーを通知します。`x` が正確な整数でない場合は「wrong-type」エラー、与えられた範囲に収まらない場合は「out-of-range」エラーです。

  関数 `scm_to_long_long`、`scm_to_ulong_long`、`scm_to_int64`、`scm_to_uint64` は、対応する型が利用可能な場合にのみ利用できます。

**C 関数: `SCM scm_from_char (char x)`**<br>**C 関数: `SCM scm_from_schar (signed char x)`**<br>**C 関数: `SCM scm_from_uchar (unsigned char x)`**<br>**C 関数: `SCM scm_from_short (short x)`**<br>**C 関数: `SCM scm_from_ushort (unsigned short x)`**<br>**C 関数: `SCM scm_from_int (int x)`**<br>**C 関数: `SCM scm_from_uint (unsigned int x)`**<br>**C 関数: `SCM scm_from_long (long x)`**<br>**C 関数: `SCM scm_from_ulong (unsigned long x)`**<br>**C 関数: `SCM scm_from_long_long (long long x)`**<br>**C 関数: `SCM scm_from_ulong_long (unsigned long long x)`**<br>**C 関数: `SCM scm_from_size_t (size_t x)`**<br>**C 関数: `SCM scm_from_ssize_t (ssize_t x)`**<br>**C 関数: `SCM scm_from_uintptr_t (uintptr_t x)`**<br>**C 関数: `SCM scm_from_ptrdiff_t (scm_t_ptrdiff x)`**<br>**C 関数: `SCM scm_from_int8 (scm_t_int8 x)`**<br>**C 関数: `SCM scm_from_uint8 (scm_t_uint8 x)`**<br>**C 関数: `SCM scm_from_int16 (scm_t_int16 x)`**<br>**C 関数: `SCM scm_from_uint16 (scm_t_uint16 x)`**<br>**C 関数: `SCM scm_from_int32 (scm_t_int32 x)`**<br>**C 関数: `SCM scm_from_uint32 (scm_t_uint32 x)`**<br>**C 関数: `SCM scm_from_int64 (scm_t_int64 x)`**<br>**C 関数: `SCM scm_from_uint64 (scm_t_uint64 x)`**<br>**C 関数: `SCM scm_from_intmax (scm_t_intmax x)`**<br>**C 関数: `SCM scm_from_uintmax (scm_t_uintmax x)`**<br>**C 関数: `SCM scm_from_intptr_t (scm_t_intptr x)`**<br>**C 関数: `SCM scm_from_uintptr_t (scm_t_uintptr x)`**
: 整数 `x` を表す `SCM` 値を返します。これらの関数は常に成功し、常に正確な数を返します。

**C 関数: `void scm_to_mpz (SCM val, mpz_t rop)`**
: `val` を多倍長整数 `rop` に代入します。`val` は正確な整数でなければならず、そうでなければエラーが通知されます。`rop` は、この関数が呼び出される前に `mpz_init` で初期化されていなければなりません。`rop` が不要になったら、占有されている領域を `mpz_clear` で解放しなければなりません。詳細については『GNU MP Manual』の「Initializing Integers」を参照してください。

**C 関数: `SCM scm_from_mpz (mpz_t val)`**
: `val` を表す `SCM` 値を返します。

## 6.6.2.3 実数と有理数

数学的には、実数とは、連続した無限の1次元の直線上のあらゆる可能な点を記述する数の集合です。有理数とは、p と q を整数として分数 p/q と書けるすべての数の集合です。すべての有理数は実数でもありますが、有理数でない実数も存在します。たとえば 2 の平方根や pi です。

Guile は正確な有理数と非正確な有理数の両方を表現できますが、精密な有限の無理数を表現することはできません。正確な有理数は、分子と分母を2つの正確な整数として格納することで表現されます。非正確な有理数は、C の型 `double` を使って浮動小数点数として格納されます。

正確な有理数は、整数の分数として書かれます。スラッシュの周りに空白があってはなりません。

```scheme
1/2
-22/7
```

非正確な有理数の実際のエンコーディングは2進数ですが、それを有効数字の桁数が限られ、どこかに小数点がある10進数として考えると分かりやすいかもしれません。これは整数でない数の標準的な表記に対応しているからです。たとえば次のようになります。

```scheme
0.34
-0.00000142857931198
-5648394822220000000000.0
4.0
```

Guile のエンコーディングの精度は限られているため、Guile における有限の「実数」はどれも、十分な 10（実際には 2）の累乗を掛けてから割ることで、有理数の形で書くことができます。たとえば、「`-0.00000142857931198`」は −142857931198 を 100000000000000000 で割ったものと同じです。したがって、Guile の現在の実装では、有限の数に対して `rational?` 述語と `real?` 述語は等価です。

正確なゼロで割ると、予想どおりエラーメッセージが出ます。しかし、非正確なゼロで割ってもエラーは生じません。代わりに、割られる数の符号とゼロの除数の符号に応じて、除算の結果は正または負の無限大になります（一部のプラットフォームは符号付きゼロ「`-0.0`」と「`+0.0`」をサポートしています。「`0.0`」は「`+0.0`」と同じです）。

ゼロを非正確なゼロで割ると NaN（「非数」）の値が得られますが、Scheme では実際には数と見なされます。`=`、`<`、`>`、`<=`、`>=` を使って NaN 値を（それ自身を含む）任意の数と比較しようとすると、常に `#f` が返されます。NaN 値はそれ自身と `=` ではありませんが、それ自身や他の NaN 値と `eqv?` かつ `equal?` です。しかし、NaN をテストする好ましい方法は `nan?` を使うことです。

実数の NaN 値と無限大は、「`+nan.0`」、「`+inf.0`」、「`-inf.0`」と書かれます。この構文は、通常の Scheme の構文の拡張として `read` によっても認識されます。これらの特別な値は、Scheme では非正確な実数と見なされますが、有理数ではありません。非実数の複素数も、実部や虚部に無限大や NaN 値を含むことがあることに注意してください。

実数が無限大か、NaN 値か、どちらでもないかをテストするには、それぞれ `inf?`、`nan?`、`finite?` を使います。Scheme のすべての実数は、これら3つのクラスのちょうど1つに属します。

浮動小数点演算について IEEE 754 に従うプラットフォームでは、「`+inf.0`」、「`-inf.0`」、「`+nan.0`」の値は、対応する IEEE 754 の値を使って実装されています。それらは算術演算において IEEE 754 が記述するとおりに振る舞います。つまり、`(= +nan.0 +nan.0) ⇒ #f` です。

**Scheme 手続き: `real? obj`**<br>**C 関数: `scm_real_p (obj)`**
: `obj` が実数であれば `#t` を、そうでなければ `#f` を返します。整数値と有理数値の集合は実数の集合の部分集合を形成しているので、`obj` が整数または有理数であってもこの述語は満たされることに注意してください。

**Scheme 手続き: `rational? x`**<br>**C 関数: `scm_rational_p (x)`**
: `x` が有理数であれば `#t` を、そうでなければ `#f` を返します。整数値の集合は有理数の集合の部分集合を形成していること、つまり `x` が整数であってもこの述語は満たされることに注意してください。

**Scheme 手続き: `rationalize x eps`**<br>**C 関数: `scm_rationalize (x, eps)`**
: `x` との差が `eps` 以下である最も単純な有理数を返します。

  R5RS で要求されているように、`rationalize` は両方の引数が正確な場合にのみ正確な結果を返します。したがって、引数に `inexact->exact` を使う必要があるかもしれません。

  ```scheme
  (rationalize (inexact->exact 1.2) 1/100)
  ⇒ 6/5
  ```

**Scheme 手続き: `inf? x`**<br>**C 関数: `scm_inf_p (x)`**
: 実数 `x` が「`+inf.0`」または「`-inf.0`」であれば `#t` を返します。そうでなければ `#f` を返します。

**Scheme 手続き: `nan? x`**<br>**C 関数: `scm_nan_p (x)`**
: 実数 `x` が「`+nan.0`」であれば `#t` を、そうでなければ `#f` を返します。

**Scheme 手続き: `finite? x`**<br>**C 関数: `scm_finite_p (x)`**
: 実数 `x` が無限大でも NaN でもなければ `#t` を、そうでなければ `#f` を返します。

**Scheme 手続き: `nan`**<br>**C 関数: `scm_nan ()`**
: NaN 値である「`+nan.0`」を返します。

**Scheme 手続き: `inf`**<br>**C 関数: `scm_inf ()`**
: 正の無限大である「`+inf.0`」を返します。

**Scheme 手続き: `numerator x`**<br>**C 関数: `scm_numerator (x)`**
: 有理数 `x` の分子を返します。

**Scheme 手続き: `denominator x`**<br>**C 関数: `scm_denominator (x)`**
: 有理数 `x` の分母を返します。

**C 関数: `int scm_is_real (SCM val)`**<br>**C 関数: `int scm_is_rational (SCM val)`**
: それぞれ `scm_is_true (scm_real_p (val))` および `scm_is_true (scm_rational_p (val))` と同等です。

**C 関数: `double scm_to_double (SCM val)`**
: `val` に最も近い、`double` として表現可能な数を返します。絶対値が大きすぎる `val` に対しては無限大を返します。引数 `val` は実数でなければなりません。

**C 関数: `SCM scm_from_double (double val)`**
: `val` を表す `SCM` 値を返します。返される値は述語 `inexact?` によれば非正確ですが、`val` と正確に等しくなります。

## 6.6.2.4 複素数

複素数とは、2次元空間のあらゆる可能な点を記述する数の集合です。この空間内の特定の点の2つの座標は、その点を記述する複素数の実部と虚部として知られています。

Guile では、複素数は、虚部を示すために記号 `i` を使い、実部と虚部の和として直交形式で書かれます。

```scheme
3+4i
⇒
3.0+4.0i

(* 3-8i 2.3+0.3i)
⇒
9.3-17.5i
```

極形式も使うことができ、大きさと角度の間に「`@`」を置きます。

```scheme
1@3.141592 ⇒ -1.0      (approx)
-1@1.57079 ⇒ 0.0-1.0i  (approx)
```

Guile は複素数を非正確な実数のペアとして表現するため、複素数の実部と虚部は、単一の非正確な実数と同じ非正確さと限られた精度の性質を持ちます。

複素数の各部分は、特別な値「`+nan.0`」、「`+inf.0`」、「`-inf.0`」、および符号付きゼロ「`0.0`」または「`-0.0`」のいずれかを含め、任意の非正確な実数値を含むことができることに注意してください。

**Scheme 手続き: `complex? z`**<br>**C 関数: `scm_complex_p (z)`**
: `z` が複素数であれば `#t` を、そうでなければ `#f` を返します。実数値、有理数値、整数値の集合は複素数の集合の部分集合を形成していること、つまり `z` が実数、有理数、整数であってもこの述語は満たされることに注意してください。

**C 関数: `int scm_is_complex (SCM val)`**
: `scm_is_true (scm_complex_p (val))` と同等です。

## 6.6.2.5 正確数と非正確数

R5RS は、わずかな例外を除いて、非正確な数を含む計算は常に非正確な結果を生成することを要求しています。この要件を満たすために、Guile は「`5`」のような正確な整数値と、利用可能な限られた精度において小数部分を持たず「`5.0`」と表示される、対応する非正確な整数値とを区別します。Guile は、`inexact->exact` 手続きの呼び出しによって強制された場合にのみ、後者の値を前者に変換します。

上記の要件の唯一の例外は、非正確な数の値が結果に影響しない場合です。たとえば `(expt n 0)` は `n` の任意の値に対して「`1`」なので、`(expt 5.0 0)` は正確な「`1`」を返すことが許されています。

**Scheme 手続き: `exact? z`**<br>**C 関数: `scm_exact_p (z)`**
: 数 `z` が正確であれば `#t` を、そうでなければ `#f` を返します。

  ```scheme
  (exact? 2)
  ⇒ #t

  (exact? 0.5)
  ⇒ #f

  (exact? (/ 2))
  ⇒ #t
  ```

**C 関数: `int scm_is_exact (SCM z)`**
: 数 `z` が正確であれば `1` を、そうでなければ `0` を返します。これは `scm_is_true (scm_exact_p (z))` と同等です。

  数の正確さをテストする別のアプローチは、`scm_is_signed_integer` または `scm_is_unsigned_integer` を使うことです。

**Scheme 手続き: `inexact? z`**<br>**C 関数: `scm_inexact_p (z)`**
: 数 `z` が非正確であれば `#t` を、そうでなければ `#f` を返します。

**C 関数: `int scm_is_inexact (SCM z)`**
: 数 `z` が非正確であれば `1` を、そうでなければ `0` を返します。これは `scm_is_true (scm_inexact_p (z))` と同等です。

**Scheme 手続き: `inexact->exact z`**<br>**C 関数: `scm_inexact_to_exact (z)`**
: `z` に数値的に最も近い正確な数があれば、それを返します。非正確な有理数については、Guile は非正確な有理数と数値的に等しい正確な有理数を返します。虚部がゼロでない非正確な複素数は正確にすることができません。

  ```scheme
  (inexact->exact 0.5)
  ⇒ 1/2
  ```

  次のことが起こるのは、12/10 が（ほとんどのプラットフォームで）`double` として正確に表現できないからです。しかし、「#e」接頭辞で正確であると印付けられた10進数を読み込むとき、Guile はそれを正しく表現できます。

  ```scheme
  (inexact->exact 1.2)
  ⇒ 5404319552844595/4503599627370496

  #e1.2
  ⇒ 6/5
  ```

**Scheme 手続き: `exact->inexact z`**<br>**C 関数: `scm_exact_to_inexact (z)`**
: 数 `z` をその非正確な表現に変換します。

## 6.6.2.6 数値データの読み取り構文

整数の読み取り構文は数字の並びであり、その前にオプションで、マイナスまたはプラスの文字、整数がエンコードされている基数を示すコード、そして数が正確か非正確かを示すコードを置くことができます。サポートされている基数のコードは次のとおりです。

`#b`<br>`#B`
: 整数は2進数（基数2）で書かれています。

`#o`<br>`#O`
: 整数は8進数（基数8）で書かれています。

`#d`<br>`#D`
: 整数は10進数（基数10）で書かれています。

`#x`<br>`#X`
: 整数は16進数（基数16）で書かれています。

基数のコードが省略された場合、整数は10進数であると見なされます。次の例は、これらの基数のコードの使い方を示しています。

```scheme
-13
⇒ -13

#d-13
⇒ -13

#x-13
⇒ -19

#b+1101
⇒ 13

#o377
⇒ 255
```

正確さを示すためのコード（ちなみに、これはすべての数値に適用できます）は次のとおりです。

`#e`<br>`#E`
: 数は正確です。

`#i`<br>`#I`
: 数は非正確です。

正確さの指示子が省略された場合、数は小数点を含まない限り正確です。Guile は正確な複素数を表現できないので、それを要求するとエラーが通知されます。

```scheme
(exact? 1.2)
⇒ #f

(exact? #e1.2)
⇒ #t

(exact? #e+1i)
ERROR: Wrong type argument
```

Guile は、正の無限大と負の無限大を表す構文「`+inf.0`」と「`-inf.0`」もそれぞれ理解します。値は示されたとおりに正確に書かなければなりません。つまり、常に符号を持ち、小数点の後にちょうど1つのゼロの数字がなければなりません。また、特別な「非数」の値を表す「`+nan.0`」と「`-nan.0`」も理解します。「非数」については符号は無視され、値は常に「`+nan.0`」と表示されます。

## 6.6.2.7 整数値に対する操作

**Scheme 手続き: `odd? n`**<br>**C 関数: `scm_odd_p (n)`**
: `n` が奇数であれば `#t` を、そうでなければ `#f` を返します。

**Scheme 手続き: `even? n`**<br>**C 関数: `scm_even_p (n)`**
: `n` が偶数であれば `#t` を、そうでなければ `#f` を返します。

**Scheme 手続き: `quotient n d`**<br>**Scheme 手続き: `remainder n d`**<br>**C 関数: `scm_quotient (n, d)`**<br>**C 関数: `scm_remainder (n, d)`**
: `n` を `d` で割った商または余りを返します。商はゼロに向かって丸められ、余りは `n` と同じ符号を持ちます。いずれの場合も、商と余りは n = q*d + r を満たします。

  ```scheme
  (remainder 13 4) ⇒ 1
  (remainder -13 4) ⇒ -1
  ```

  「算術関数」の `truncate-quotient`、`truncate-remainder` と関連する操作も参照してください。

**Scheme 手続き: `modulo n d`**<br>**C 関数: `scm_modulo (n, d)`**
: `n` を `d` で割った余りを、`d` と同じ符号で返します。

  ```scheme
  (modulo 13 4) ⇒ 1
  (modulo -13 4) ⇒ 3
  (modulo 13 -4) ⇒ -3
  (modulo -13 -4) ⇒ -1
  ```

  「算術関数」の `floor-quotient`、`floor-remainder` と関連する操作も参照してください。

**Scheme 手続き: `gcd x…`**<br>**C 関数: `scm_gcd (x, y)`**
: すべての引数の最大公約数を返します。引数なしで呼び出された場合は 0 が返されます。

  C 関数 `scm_gcd` は常に2つの引数を取りますが、Scheme 関数は任意の個数を取ることができます。

**Scheme 手続き: `lcm x…`**<br>**C 関数: `scm_lcm (x, y)`**
: 引数の最小公倍数を返します。引数なしで呼び出された場合は 1 が返されます。

  C 関数 `scm_lcm` は常に2つの引数を取りますが、Scheme 関数は任意の個数を取ることができます。

**Scheme 手続き: `modulo-expt n k m`**<br>**C 関数: `scm_modulo_expt (n, k, m)`**
: `n` を整数の指数 `k` 乗したものを、`m` を法として返します。

  ```scheme
  (modulo-expt 2 3 5)
  ⇒ 3
  ```

**Scheme 手続き: `exact-integer-sqrt k`**<br>**C 関数: `void scm_exact_integer_sqrt (SCM k, SCM *s, SCM *r)`**
: k = s^2 + r かつ s^2 <= k < (s + 1)^2 となる2つの正確な非負整数 `s` と `r` を返します。`k` が正確な非負整数でなければエラーが発生します。

  ```scheme
  (exact-integer-sqrt 10) ⇒ 3 and 1
  ```

## 6.6.2.8 比較述語

以下の C の比較関数は常に2つの引数を取りますが、Scheme 関数は任意の個数を取ることができます。また、C 関数は Scheme の真偽値 `SCM_BOOL_T` または `SCM_BOOL_F` のいずれかを返し、これらは C から見るとどちらも真であることに注意してください。したがって、たとえば2つの Scheme の数 `x` と `y` が等しいかどうかをテストするときは、常に `scm_is_true (scm_num_eq_p (x, y))` と書いてください。

**Scheme 手続き: `=`**<br>**C 関数: `scm_num_eq_p (x, y)`**
: すべてのパラメータが数値的に等しければ `#t` を返します。

**Scheme 手続き: `<`**<br>**C 関数: `scm_less_p (x, y)`**
: パラメータのリストが単調増加していれば `#t` を返します。

**Scheme 手続き: `>`**<br>**C 関数: `scm_gr_p (x, y)`**
: パラメータのリストが単調減少していれば `#t` を返します。

**Scheme 手続き: `<=`**<br>**C 関数: `scm_leq_p (x, y)`**
: パラメータのリストが単調非減少であれば `#t` を返します。

**Scheme 手続き: `>=`**<br>**C 関数: `scm_geq_p (x, y)`**
: パラメータのリストが単調非増加であれば `#t` を返します。

**Scheme 手続き: `zero? z`**<br>**C 関数: `scm_zero_p (z)`**
: `z` がゼロに等しい正確または非正確な数であれば `#t` を返します。

**Scheme 手続き: `positive? x`**<br>**C 関数: `scm_positive_p (x)`**
: `x` がゼロより大きい正確または非正確な数であれば `#t` を返します。

**Scheme 手続き: `negative? x`**<br>**C 関数: `scm_negative_p (x)`**
: `x` がゼロより小さい正確または非正確な数であれば `#t` を返します。

## 6.6.2.9 数値と文字列の相互変換

以下の手続きは、R5RS で定義された外部表現に従って数を読み書きします（『The Revised^5 Report on the Algorithmic Language Scheme』の「R5RS Lexical Structure」を参照）。ロケールに依存した数の解析については、`(ice-9 i18n)` モジュールを参照してください。

**Scheme 手続き: `number->string n [radix]`**<br>**C 関数: `scm_number_to_string (n, radix)`**
: 与えられた基数 `radix` での数 `n` の外部表現を保持する文字列を返します。`n` が非正確な場合は、基数 10 が使用されます。

**Scheme 手続き: `string->number string [radix]`**<br>**C 関数: `scm_string_to_number (string, radix)`**
: 与えられた文字列 `string` によって表される、最も精密な表現の数を返します。`radix` は 2、8、10、16 のいずれかの正確な整数でなければなりません。与えられた場合、`radix` は、`string` 内の明示的な基数の接頭辞（例: "#o177"）によって上書きされる可能性のあるデフォルトの基数です。`radix` が与えられない場合、デフォルトの基数は 10 です。`string` が数の構文的に正しい表記でない場合、`string->number` は `#f` を返します。

**C 関数: `SCM scm_c_locale_stringn_to_number (const char *string, size_t len, unsigned radix)`**
: 上の `string->number` と同様ですが、ポインタと長さとして C の文字列を受け取ります。文字列の文字は現在のロケールのエンコーディングでなければなりません（名前の `locale` はそれだけを指しており、ロケールに依存した解析はありません）。

## 6.6.2.10 複素数の操作

**Scheme 手続き: `make-rectangular real_part imaginary_part`**<br>**C 関数: `scm_make_rectangular (real_part, imaginary_part)`**
: 与えられた実部 `real-part` と虚部 `imaginary-part` から構成される複素数を返します。

**Scheme 手続き: `make-polar mag ang`**<br>**C 関数: `scm_make_polar (mag, ang)`**
: 複素数 mag * e^(i * ang) を返します。

**Scheme 手続き: `real-part z`**<br>**C 関数: `scm_real_part (z)`**
: 数 `z` の実部を返します。

**Scheme 手続き: `imag-part z`**<br>**C 関数: `scm_imag_part (z)`**
: 数 `z` の虚部を返します。

**Scheme 手続き: `magnitude z`**<br>**C 関数: `scm_magnitude (z)`**
: 数 `z` の大きさを返します。これは実数の引数に対しては `abs` と同じですが、複素数も受け付けます。

**Scheme 手続き: `angle z`**<br>**C 関数: `scm_angle (z)`**
: 複素数 `z` の角度を返します。

**C 関数: `SCM scm_c_make_rectangular (double re, double im)`**<br>**C 関数: `SCM scm_c_make_polar (double x, double y)`**
: それぞれ `scm_make_rectangular` または `scm_make_polar` と同様ですが、これらの関数は引数として `double` を受け取ります。

**C 関数: `double scm_c_real_part (z)`**<br>**C 関数: `double scm_c_imag_part (z)`**
: `z` の実部または虚部を `double` として返します。

**C 関数: `double scm_c_magnitude (z)`**<br>**C 関数: `double scm_c_angle (z)`**
: `z` の大きさまたは角度を `double` として返します。

## 6.6.2.11 算術関数

以下の C の算術関数は常に2つの引数を取りますが、Scheme 関数は任意の個数を取ることができます。たとえば `(- x)` に相当するものを計算するために1つの引数だけで呼び出す必要がある場合は、2番目の引数として `SCM_UNDEFINED` を渡してください: `scm_difference (x, SCM_UNDEFINED)`。

**Scheme 手続き: `+ z1 …`**<br>**C 関数: `scm_sum (z1, z2)`**
: すべてのパラメータの値の和を返します。パラメータなしで呼び出された場合は 0 を返します。

**Scheme 手続き: `- z1 z2 …`**<br>**C 関数: `scm_difference (z1, z2)`**
: 1つの引数 `z1` で呼び出された場合は、`-z1` が返されます。そうでなければ、最初の引数以外のすべての引数の和が最初の引数から引かれます。

**Scheme 手続き: `* z1 …`**<br>**C 関数: `scm_product (z1, z2)`**
: すべての引数の積を返します。引数なしで呼び出された場合は 1 が返されます。

**Scheme 手続き: `/ z1 z2 …`**<br>**C 関数: `scm_divide (z1, z2)`**
: 最初の引数を残りの引数の積で割ります。1つの引数 `z1` で呼び出された場合は、`1/z1` が返されます。

**Scheme 手続き: `1+ z`**<br>**C 関数: `scm_oneplus (z)`**
: z + 1 を返します。

**Scheme 手続き: `1- z`**<br>**C 関数: `scm_oneminus (z)`**
: z - 1 を返します。

**Scheme 手続き: `abs x`**<br>**C 関数: `scm_abs (x)`**
: `x` の絶対値を返します。

  `x` は虚部がゼロの数でなければなりません。複素数の大きさを計算するには、代わりに `magnitude` を使ってください。

**Scheme 手続き: `max x1 x2 …`**<br>**C 関数: `scm_max (x1, x2)`**
: すべてのパラメータの値の最大値を返します。

**Scheme 手続き: `min x1 x2 …`**<br>**C 関数: `scm_min (x1, x2)`**
: すべてのパラメータの値の最小値を返します。

**Scheme 手続き: `truncate x`**<br>**C 関数: `scm_truncate_number (x)`**
: 非正確な数 `x` をゼロに向かって丸めます。

**Scheme 手続き: `round x`**<br>**C 関数: `scm_round_number (x)`**
: 非正確な数 `x` を最も近い整数に丸めます。2つの整数のちょうど中間にある場合は、偶数のほうに丸めます。

**Scheme 手続き: `floor x`**<br>**C 関数: `scm_floor (x)`**
: 数 `x` を負の無限大に向かって丸めます。

**Scheme 手続き: `ceiling x`**<br>**C 関数: `scm_ceiling (x)`**
: 数 `x` を無限大に向かって丸めます。

**C 関数: `double scm_c_truncate (double x)`**<br>**C 関数: `double scm_c_round (double x)`**
: それぞれ `scm_truncate_number` または `scm_round_number` と同様ですが、これらの関数は `double` の値を受け取って返します。

**Scheme 手続き: `euclidean/ x y`**<br>**Scheme 手続き: `euclidean-quotient x y`**<br>**Scheme 手続き: `euclidean-remainder x y`**<br>**C 関数: `void scm_euclidean_divide (SCM x, SCM y, SCM *q, SCM *r)`**<br>**C 関数: `SCM scm_euclidean_quotient (SCM x, SCM y)`**<br>**C 関数: `SCM scm_euclidean_remainder (SCM x, SCM y)`**
: これらの手続きは2つの実数 `x` と `y` を受け付けます。ここで除数 `y` はゼロであってはなりません。`euclidean-quotient` は x = q*y + r かつ 0 <= r < |y| となる整数 `q` を返し、`euclidean-remainder` はそのような実数 `r` を返します。`euclidean/` は `q` と `r` の両方を返し、それぞれを別々に計算するよりも効率的です。y > 0 のとき、`euclidean-quotient` は floor(x/y) を返し、そうでなければ ceiling(x/y) を返すことに注意してください。

  これらの演算子は R6RS の演算子 `div`、`mod`、`div-and-mod` と同等であることに注意してください。

  ```scheme
  (euclidean-quotient 123 10) ⇒ 12
  (euclidean-remainder 123 10) ⇒ 3
  (euclidean/ 123 10) ⇒ 12 and 3
  (euclidean/ 123 -10) ⇒ -12 and 3
  (euclidean/ -123 10) ⇒ -13 and 7
  (euclidean/ -123 -10) ⇒ 13 and 7
  (euclidean/ -123.2 -63.5) ⇒ 2.0 and 3.8
  (euclidean/ 16/3 -10/7) ⇒ -3 and 22/21
  ```

**Scheme 手続き: `floor/ x y`**<br>**Scheme 手続き: `floor-quotient x y`**<br>**Scheme 手続き: `floor-remainder x y`**<br>**C 関数: `void scm_floor_divide (SCM x, SCM y, SCM *q, SCM *r)`**<br>**C 関数: `SCM scm_floor_quotient (x, y)`**<br>**C 関数: `SCM scm_floor_remainder (x, y)`**
: これらの手続きは2つの実数 `x` と `y` を受け付けます。ここで除数 `y` はゼロであってはなりません。`floor-quotient` は q = floor(x/y) かつ x = q*y + r となる整数 `q` を返し、`floor-remainder` はそのような実数 `r` を返します。`floor/` は `q` と `r` の両方を返し、それぞれを別々に計算するよりも効率的です。`r` がゼロでない場合、`y` と同じ符号を持つことに注意してください。

  `x` と `y` が整数のとき、`floor-remainder` は R5RS の整数専用の演算子 `modulo` と同等です。

  ```scheme
  (floor-quotient 123 10) ⇒ 12
  (floor-remainder 123 10) ⇒ 3
  (floor/ 123 10) ⇒ 12 and 3
  (floor/ 123 -10) ⇒ -13 and -7
  (floor/ -123 10) ⇒ -13 and 7
  (floor/ -123 -10) ⇒ 12 and -3
  (floor/ -123.2 -63.5) ⇒ 1.0 and -59.7
  (floor/ 16/3 -10/7) ⇒ -4 and -8/21
  ```

**Scheme 手続き: `ceiling/ x y`**<br>**Scheme 手続き: `ceiling-quotient x y`**<br>**Scheme 手続き: `ceiling-remainder x y`**<br>**C 関数: `void scm_ceiling_divide (SCM x, SCM y, SCM *q, SCM *r)`**<br>**C 関数: `SCM scm_ceiling_quotient (x, y)`**<br>**C 関数: `SCM scm_ceiling_remainder (x, y)`**
: これらの手続きは2つの実数 `x` と `y` を受け付けます。ここで除数 `y` はゼロであってはなりません。`ceiling-quotient` は q = ceiling(x/y) かつ x = q*y + r となる整数 `q` を返し、`ceiling-remainder` はそのような実数 `r` を返します。`ceiling/` は `q` と `r` の両方を返し、それぞれを別々に計算するよりも効率的です。`r` がゼロでない場合、`y` と逆の符号を持つことに注意してください。

  ```scheme
  (ceiling-quotient 123 10) ⇒ 13
  (ceiling-remainder 123 10) ⇒ -7
  (ceiling/ 123 10) ⇒ 13 and -7
  (ceiling/ 123 -10) ⇒ -12 and 3
  (ceiling/ -123 10) ⇒ -12 and -3
  (ceiling/ -123 -10) ⇒ 13 and 7
  (ceiling/ -123.2 -63.5) ⇒ 2.0 and 3.8
  (ceiling/ 16/3 -10/7) ⇒ -3 and 22/21
  ```

**Scheme 手続き: `truncate/ x y`**<br>**Scheme 手続き: `truncate-quotient x y`**<br>**Scheme 手続き: `truncate-remainder x y`**<br>**C 関数: `void scm_truncate_divide (SCM x, SCM y, SCM *q, SCM *r)`**<br>**C 関数: `SCM scm_truncate_quotient (x, y)`**<br>**C 関数: `SCM scm_truncate_remainder (x, y)`**
: これらの手続きは2つの実数 `x` と `y` を受け付けます。ここで除数 `y` はゼロであってはなりません。`truncate-quotient` は、q が x/y をゼロに向かって丸めたものであり、かつ x = q*y + r となる整数 `q` を返し、`truncate-remainder` はそのような実数 `r` を返します。`truncate/` は `q` と `r` の両方を返し、それぞれを別々に計算するよりも効率的です。`r` がゼロでない場合、`x` と同じ符号を持つことに注意してください。

  `x` と `y` が整数のとき、これらの演算子は R5RS の整数専用の演算子 `quotient` と `remainder` と同等です。

  ```scheme
  (truncate-quotient 123 10) ⇒ 12
  (truncate-remainder 123 10) ⇒ 3
  (truncate/ 123 10) ⇒ 12 and 3
  (truncate/ 123 -10) ⇒ -12 and 3
  (truncate/ -123 10) ⇒ -12 and -3
  (truncate/ -123 -10) ⇒ 12 and -3
  (truncate/ -123.2 -63.5) ⇒ 1.0 and -59.7
  (truncate/ 16/3 -10/7) ⇒ -3 and 22/21
  ```

**Scheme 手続き: `centered/ x y`**<br>**Scheme 手続き: `centered-quotient x y`**<br>**Scheme 手続き: `centered-remainder x y`**<br>**C 関数: `void scm_centered_divide (SCM x, SCM y, SCM *q, SCM *r)`**<br>**C 関数: `SCM scm_centered_quotient (SCM x, SCM y)`**<br>**C 関数: `SCM scm_centered_remainder (SCM x, SCM y)`**
: これらの手続きは2つの実数 `x` と `y` を受け付けます。ここで除数 `y` はゼロであってはなりません。`centered-quotient` は x = q*y + r かつ -|y/2| <= r < |y/2| となる整数 `q` を返し、`centered-remainder` はそのような実数 `r` を返します。`centered/` は `q` と `r` の両方を返し、それぞれを別々に計算するよりも効率的です。

  `centered-quotient` は x/y を最も近い整数に丸めたものを返すことに注意してください。x/y が2つの整数のちょうど中間にあるとき、引き分けは `y` の符号に従って解決されます。y > 0 であれば、引き分けは正の無限大に向かって丸められ、そうでなければ負の無限大に向かって丸められます。これは -|y/2| <= r < |y/2| という要件の帰結です。

  これらの演算子は R6RS の演算子 `div0`、`mod0`、`div0-and-mod0` と同等であることに注意してください。

  ```scheme
  (centered-quotient 123 10) ⇒ 12
  (centered-remainder 123 10) ⇒ 3
  (centered/ 123 10) ⇒ 12 and 3
  (centered/ 123 -10) ⇒ -12 and 3
  (centered/ -123 10) ⇒ -12 and -3
  (centered/ -123 -10) ⇒ 12 and -3
  (centered/ 125 10) ⇒ 13 and -5
  (centered/ 127 10) ⇒ 13 and -3
  (centered/ 135 10) ⇒ 14 and -5
  (centered/ -123.2 -63.5) ⇒ 2.0 and 3.8
  (centered/ 16/3 -10/7) ⇒ -4 and -8/21
  ```

**Scheme 手続き: `round/ x y`**<br>**Scheme 手続き: `round-quotient x y`**<br>**Scheme 手続き: `round-remainder x y`**<br>**C 関数: `void scm_round_divide (SCM x, SCM y, SCM *q, SCM *r)`**<br>**C 関数: `SCM scm_round_quotient (x, y)`**<br>**C 関数: `SCM scm_round_remainder (x, y)`**
: これらの手続きは2つの実数 `x` と `y` を受け付けます。ここで除数 `y` はゼロであってはなりません。`round-quotient` は、x = q*y + r であり、q が x/y を最も近い整数に丸めたもの（引き分けは最も近い偶数に向かう）となる整数 `q` を返し、`round-remainder` はそのような実数 `r` を返します。`round/` は `q` と `r` の両方を返し、それぞれを別々に計算するよりも効率的です。

  `round/` と `centered/` はほぼ同等ですが、x/y が2つの整数のちょうど中間にあるときに動作が異なることに注意してください。この場合、`round/` は最も近い偶数を選びますが、`centered/` は -|y/2| <= r < |y/2| という制約を満たすように選びます。これは `round/` に対応する制約 -|y/2| <= r <= |y/2| よりも強いものです。特に、`x` と `y` が整数のとき、`centered/` が返しうる余りの数は |y| ですが、`round/` が返しうる余りの数は `y` が偶数のとき |y|+1 です。

  ```scheme
  (round-quotient 123 10) ⇒ 12
  (round-remainder 123 10) ⇒ 3
  (round/ 123 10) ⇒ 12 and 3
  (round/ 123 -10) ⇒ -12 and 3
  (round/ -123 10) ⇒ -12 and -3
  (round/ -123 -10) ⇒ 12 and -3
  (round/ 125 10) ⇒ 12 and 5
  (round/ 127 10) ⇒ 13 and -3
  (round/ 135 10) ⇒ 14 and -5
  (round/ -123.2 -63.5) ⇒ 2.0 and 3.8
  (round/ 16/3 -10/7) ⇒ -4 and -8/21
  ```

## 6.6.2.12 科学関数

以下の手続きは、複素数を含め、あらゆる種類の数を引数として受け付けます。

**Scheme 手続き: `sqrt z`**
: `z` の平方根を返します。2つの可能な根（正と負）のうち、実部が正のものが返されます。実部がゼロの場合は、虚部が正のものが返されます。したがって、次のようになります。

  ```scheme
  (sqrt 9.0)       ⇒ 3.0
  (sqrt -9.0)      ⇒ 0.0+3.0i
  (sqrt 1.0+1.0i)  ⇒ 1.09868411346781+0.455089860562227i
  (sqrt -1.0-1.0i) ⇒ 0.455089860562227-1.09868411346781i
  ```

**Scheme 手続き: `expt z1 z2`**
: `z1` の `z2` 乗を返します。

**Scheme 手続き: `sin z`**
: `z` の正弦を返します。

**Scheme 手続き: `cos z`**
: `z` の余弦を返します。

**Scheme 手続き: `tan z`**
: `z` の正接を返します。

**Scheme 手続き: `asin z`**
: `z` の逆正弦を返します。

**Scheme 手続き: `acos z`**
: `z` の逆余弦を返します。

**Scheme 手続き: `atan z`**<br>**Scheme 手続き: `atan y x`**
: `z` の、または y/x の逆正接を返します。

**Scheme 手続き: `exp z`**
: e の `z` 乗を返します。ここで e は自然対数の底（2.71828…）です。

**Scheme 手続き: `log z`**
: `z` の自然対数を返します。

**Scheme 手続き: `log10 z`**
: `z` の底 10 の対数を返します。

**Scheme 手続き: `sinh z`**
: `z` の双曲線正弦を返します。

**Scheme 手続き: `cosh z`**
: `z` の双曲線余弦を返します。

**Scheme 手続き: `tanh z`**
: `z` の双曲線正接を返します。

**Scheme 手続き: `asinh z`**
: `z` の逆双曲線正弦を返します。

**Scheme 手続き: `acosh z`**
: `z` の逆双曲線余弦を返します。

**Scheme 手続き: `atanh z`**
: `z` の逆双曲線正接を返します。

## 6.6.2.13 ビット演算

以下のビット演算関数では、負の数は無限精度の2の補数として扱われます。たとえば -6 はビット ...111010 であり、左側に無限に多くの1があります。このようなビットパターンに 6（2進数で 110）を加えるとすべてゼロになることが分かります。

**Scheme 手続き: `logand n1 n2 …`**<br>**C 関数: `scm_logand (n1, n2)`**
: 整数の引数のビットごとの AND を返します。

  ```scheme
  (logand) ⇒ -1
  (logand 7) ⇒ 7
  (logand #b111 #b011 #b001) ⇒ 1
  ```

**Scheme 手続き: `logior n1 n2 …`**<br>**C 関数: `scm_logior (n1, n2)`**
: 整数の引数のビットごとの OR を返します。

  ```scheme
  (logior) ⇒ 0
  (logior 7) ⇒ 7
  (logior #b000 #b001 #b011) ⇒ 3
  ```

**Scheme 手続き: `logxor n1 n2 …`**<br>**C 関数: `scm_loxor (n1, n2)`**
: 整数の引数のビットごとの XOR を返します。奇数個の引数でセットされているビットが、結果でセットされます。

  ```scheme
  (logxor) ⇒ 0
  (logxor 7) ⇒ 7
  (logxor #b000 #b001 #b011) ⇒ 2
  (logxor #b000 #b001 #b011 #b011) ⇒ 1
  ```

**Scheme 手続き: `lognot n`**<br>**C 関数: `scm_lognot (n)`**
: 整数の引数の1の補数である整数、つまり各 0 ビットを 1 に、各 1 ビットを 0 に変えた整数を返します。

  ```scheme
  (number->string (lognot #b10000000) 2)
  ⇒ "-10000001"
  (number->string (lognot #b0) 2)
  ⇒ "-1"
  ```

**Scheme 手続き: `logtest j k`**<br>**C 関数: `scm_logtest (j, k)`**
: `j` と `k` に共通する 1 ビットがあるかどうかをテストします。これは `(not (zero? (logand j k)))` と同等ですが、実際に `logand` を計算せず、ゼロでないかどうかをテストするだけです。

  ```scheme
  (logtest #b0100 #b1011) ⇒ #f
  (logtest #b0100 #b0111) ⇒ #t
  ```

**Scheme 手続き: `logbit? index j`**<br>**C 関数: `scm_logbit_p (index, j)`**
: `j` のビット番号 `index` がセットされているかどうかをテストします。`index` は最下位ビットを 0 として始まります。

  ```scheme
  (logbit? 0 #b1101) ⇒ #t
  (logbit? 1 #b1101) ⇒ #f
  (logbit? 2 #b1101) ⇒ #t
  (logbit? 3 #b1101) ⇒ #t
  (logbit? 4 #b1101) ⇒ #f
  ```

**Scheme 手続き: `ash n count`**<br>**C 関数: `scm_ash (n, count)`**
: floor(n * 2^{count}) を返します。`n` と `count` は正確な整数でなければなりません。

  `n` を無限精度の2の補数の整数と見なすと、`ash` は `count` が正のときはゼロのビットを導入する左シフトを、`count` が負のときはビットを捨てる右シフトを意味します。これは「算術」シフトです。

  ```scheme
  (number->string (ash #b1 3) 2)     ⇒ "1000"
  (number->string (ash #b1010 -1) 2) ⇒ "101"

  ;; -23 is bits ...11101001, -6 is bits ...111010
  (ash -23 -2) ⇒ -6
  ```

**Scheme 手続き: `round-ash n count`**<br>**C 関数: `scm_round_ash (n, count)`**
: round(n * 2^count) を返します。`n` と `count` は正確な整数でなければなりません。

  `n` を無限精度の2の補数の整数と見なすと、`round-ash` は `count` が正のときはゼロのビットを導入する左シフトを、`count` が負のときは最も近い整数に丸める（引き分けは最も近い偶数に向かう）右シフトを意味します。これは丸めを伴う「算術」シフトです。

  ```scheme
  (number->string (round-ash #b1 3) 2)     ⇒ \"1000\"
  (number->string (round-ash #b1010 -1) 2) ⇒ \"101\"
  (number->string (round-ash #b1010 -2) 2) ⇒ \"10\"
  (number->string (round-ash #b1011 -2) 2) ⇒ \"11\"
  (number->string (round-ash #b1101 -2) 2) ⇒ \"11\"
  (number->string (round-ash #b1110 -2) 2) ⇒ \"100\"
  ```

**Scheme 手続き: `logcount n`**<br>**C 関数: `scm_logcount (n)`**
: 整数 `n` のビットの数を返します。`n` が正の場合は、その2進表現の 1 ビットが数えられます。負の場合は、その2の補数の2進表現の 0 ビットが数えられます。ゼロの場合は 0 が返されます。

  ```scheme
  (logcount #b10101010)
  ⇒ 4
  (logcount 0)
  ⇒ 0
  (logcount -2)
  ⇒ 1
  ```

**Scheme 手続き: `integer-length n`**<br>**C 関数: `scm_integer_length (n)`**
: `n` を表現するのに必要なビットの数を返します。

  正の `n` の場合、これは最上位の 1 ビットまでのビット数です。負の `n` の場合、それは2の補数の形での最上位の 0 ビットまでのビット数です。

  ```scheme
  (integer-length #b10101010) ⇒ 8
  (integer-length #b1111)     ⇒ 4
  (integer-length 0)          ⇒ 0
  (integer-length -1)         ⇒ 0
  (integer-length -256)       ⇒ 8
  (integer-length -257)       ⇒ 9
  ```

**Scheme 手続き: `integer-expt n k`**<br>**C 関数: `scm_integer_expt (n, k)`**
: `n` の `k` 乗を返します。`k` は正確な整数でなければなりませんが、`n` は任意の数でかまいません。

  負の `k` もサポートされており、通常どおり 1/n^abs(k) になります。n^0 は通常どおり 1 であり、0^0 も 1 です。

  ```scheme
  (integer-expt 2 5)   ⇒ 32
  (integer-expt -3 3)  ⇒ -27
  (integer-expt 5 -3)  ⇒ 1/125
  (integer-expt 0 0)   ⇒ 1
  ```

**Scheme 手続き: `bit-extract n start end`**<br>**C 関数: `scm_bit_extract (n, start, end)`**
: `n` の `start`（これを含む）から `end`（これを含まない）までのビットで構成される整数を返します。`start` 番目のビットが結果の 0 番目のビットになります。

  ```scheme
  (number->string (bit-extract #b1101101010 0 4) 2)
  ⇒ "1010"
  (number->string (bit-extract #b1101101010 4 9) 2)
  ⇒ "10110"
  ```

## 6.6.2.14 乱数生成

擬似乱数は乱数状態オブジェクトから生成されます。乱数状態オブジェクトは `seed->random-state` または `datum->random-state` で作成できます。乱数状態オブジェクトの外部表現（つまり `write` で書き出して `read` で読み込めるもの）は、`random-state->datum` によって得られます。以下のさまざまな関数の `state` パラメータはオプションであり、デフォルトは `*random-state*` 変数にある状態オブジェクトです。

**Scheme 手続き: `copy-random-state [state]`**<br>**C 関数: `scm_copy_random_state (state)`**
: 乱数状態 `state` のコピーを返します。

**Scheme 手続き: `random n [state]`**<br>**C 関数: `scm_random (n, state)`**
: [0, n) の範囲の数を返します。

  正の整数または実数 `n` を受け付け、ゼロ（これを含む）から `n`（これを含まない）までの同じ型の数を返します。返される値は一様分布を持ちます。

**Scheme 手続き: `random:exp [state]`**<br>**C 関数: `scm_random_exp (state)`**
: 平均 1 の指数分布に従う非正確な実数を返します。平均 `u` の指数分布には `(* u (random:exp))` を使ってください。

**Scheme 手続き: `random:hollow-sphere! vect [state]`**<br>**C 関数: `scm_random_hollow_sphere_x (vect, state)`**
: `vect` を、その2乗の和が 1.0 に等しくなる非正確な実数の乱数で埋めます。`vect` を次元 n = `(vector-length vect)` の空間の座標と考えると、座標は単位 n 球の表面上に一様に分布します。

**Scheme 手続き: `random:normal [state]`**<br>**C 関数: `scm_random_normal (state)`**
: 正規分布に従う非正確な実数を返します。使われる分布は平均 0、標準偏差 1 です。平均 `m`、標準偏差 `d` の正規分布には `(+ m (* d (random:normal)))` を使ってください。

**Scheme 手続き: `random:normal-vector! vect [state]`**<br>**C 関数: `scm_random_normal_vector_x (vect, state)`**
: `vect` を、独立で標準正規分布に従う（つまり平均 0、分散 1 の）非正確な実数の乱数で埋めます。

**Scheme 手続き: `random:solid-sphere! vect [state]`**<br>**C 関数: `scm_random_solid_sphere_x (vect, state)`**
: `vect` を、その2乗の和が 1.0 未満になる非正確な実数の乱数で埋めます。`vect` を次元 n = `(vector-length vect)` の空間の座標と考えると、座標は単位 n 球の内部に一様に分布します。

**Scheme 手続き: `random:uniform [state]`**<br>**C 関数: `scm_random_uniform (state)`**
: [0,1) の範囲で一様に分布する非正確な実数の乱数を返します。

**Scheme 手続き: `seed->random-state seed`**<br>**C 関数: `scm_seed_to_random_state (seed)`**
: `seed` を使った新しい乱数状態を返します。

**Scheme 手続き: `datum->random-state datum`**<br>**C 関数: `scm_datum_to_random_state (datum)`**
: `datum` から新しい乱数状態を返します。`datum` は `random-state->datum` によって得られたものであるべきです。

**Scheme 手続き: `random-state->datum state`**<br>**C 関数: `scm_random_state_to_datum (state)`**
: Scheme のリーダで書き出して読み戻すことができる、`state` のデータ表現を返します。

**Scheme 手続き: `random-state-from-platform`**<br>**C 関数: `scm_random_state_from_platform ()`**
: セキュリティが重要でないアプリケーションでの使用に適した、プラットフォーム固有のエントロピー源からシードされた新しい乱数状態を構築します。現在は、まず `/dev/urandom` が試され、そうでなければシードは、時刻、日付、プロセス ID、新しく割り当てられたヒープセルのアドレス、局所スタックフレームのアドレス、そして利用可能であれば高分解能タイマーに基づいて作られます。

**変数: `*random-state*`**
: `state` パラメータが与えられないときに上記の関数で使われる、グローバルな乱数状態です。

`*random-state*` の初期値は、Guile が起動するたびに同じであることに注意してください。したがって、上記の手続きに `state` パラメータを渡さず、`*random-state*` を `(seed->random-state your-seed)`（ここで `your-seed` は毎回同じではない何か）に設定しなければ、毎回の実行で同じ「乱数」の並びが得られます。

たとえば、関連するソースコードが変更されていない限り、Guile の起動以降で乱数を最初に使うのが `(map random (cdr (iota 30)))` であれば、常に次のようになります。

```scheme
(map random (cdr (iota 19)))
⇒
(0 1 1 2 2 2 1 2 6 7 10 0 5 3 12 5 5 12)
```

セキュリティが重要でないアプリケーションのために乱数状態を適切な方法でシードするには、プログラムの初期化中に次のようにします。

```scheme
(set! *random-state* (random-state-from-platform))
```

---

> **ライセンス**: この翻訳は GNU Free Documentation License v1.3 以降に基づいて作成されています。
> 原文の著作権: Copyright (C) 1996-2023 Free Software Foundation, Inc.
