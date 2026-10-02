#### 6.6.2 数値データ型 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Numerical-data-types)

Guileは、整数、有理数、実数、複素数といった豊富な数値型をサポートし、数値データを操作する広範な数学関数と科学関数を提供します。このマニュアルのこのセクションでは、それらの型と関数について説明します。

また、R5RS の Scheme における数値の表現方法を読むと、特に分かりやすく理解しやすいので参考になるかもしれません。R5RS の [Numbers](https://doc.guix.gnu.org/r5rs/latest/en/r5rs.html#Numbers) を参照してください。

* [Schemeの数値計算「タワー」](https://doc.guix.gnu.org/guile/latest/en/guile.html#Numerical-Tower)
* [整数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Integers)
* [実数と有理数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Reals-and-Rationals)
* [複素数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Complex-Numbers)
* [正確な数値と不正確な数値](https://doc.guix.gnu.org/guile/latest/en/guile.html#Exactness)
* [数値データの構文を読む](https://doc.guix.gnu.org/guile/latest/en/guile.html#Number-Syntax)
* [整数値に対する演算](https://doc.guix.gnu.org/guile/latest/en/guile.html#Integer-Operations)
* [比較述語](https://doc.guix.gnu.org/guile/latest/en/guile.html#Comparison)
* [数値と文字列の変換](https://doc.guix.gnu.org/guile/latest/en/guile.html#Conversion)
* [複素数演算](https://doc.guix.gnu.org/guile/latest/en/guile.html#Complex)
* [算術関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Arithmetic)
* [科学関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scientific)
* [ビット演算](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bitwise-Operations)
* [乱数生成](https://doc.guix.gnu.org/guile/latest/en/guile.html#Random)

* * *

次へ: [整数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Integers)、上: [数値データ型](https://doc.guix.gnu.org/guile/latest/en/guile.html#Numbers) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.2.1 Scheme の数値「タワー」[¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scheme_0027s-Numerical-_0060_0060Tower_0027_0027)

このスキームの数値「タワー」は、以下の種類の数値で構成されています。

整数

正負の整数。例：-5、0、18。

_根拠_

pとqが整数であるとき、_p/q_の形で表せる数の集合。例えば、_9/16_は表せるが、π（無理数）は表せない。これには整数（_n/1_）も含まれる。

_実数_

1次元の直線上のあらゆる位置を表す数の集合。これには有理数と無理数の両方が含まれる。

複素数

2次元空間におけるすべての可能な位置を表す数値の集合。これには実数と虚数（a+bi、ここでaは実部、bは虚部、iは-1の平方根）の両方が含まれます。

これは「塔」と呼ばれています。なぜなら、各カテゴリーは、その次のカテゴリーの上に「乗っている」ような関係にあるからです。つまり、すべての整数は有理数でもあり、すべての有理数は実数でもあり、すべての実数は複素数（ただし虚数部はゼロ）でもあるということです。

Schemeは、整数、有理数、実数、複素数への分類に加えて、数値が正確に表現されているか否かも区別します。例えば、_2\*sin(pi/4)_の結果は正確に_2^(1/2)_ですが、Guileは_pi/4_も_2^(1/2)_も正確に表現できません。代わりに、C言語の型`double`を使用して、不正確な近似値を格納します。

Guileは、任意の大きさの正確な有理数、C言語の`double`に収まる不正確な有理数、および`double`の実部と虚部を持つ不正確な複素数を表現できます。

`number?`述語は、任意のScheme値に適用して、その値がサポートされている数値型のいずれかであるかどうかを判定できます。

スキーム手順: **number?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-number_003f)

C 関数: **scm\_number\_p** (obj) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fnumber_005fp)

objが何らかの数値であれば`#t`を返し、そうでなければ`#f`を返します。

例えば：

([number?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-number_003f) 3)
⇒ #t

([number?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-number_003f) "こんにちは！")
⇒ #f

(円周率を3.141592654と定義)
([数値?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-number_003f) π)
⇒ #t

C 関数: `int` **scm\_is\_number** `(SCM obj)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fis_005fnumber)

これは `scm_is_true (scm_number_p (obj))` と同等です。

次のいくつかの小節では、Guileの各数値データ型について詳しく説明します。

* * *

次へ: [実数と有理数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Reals-and-Rationals)、前: [Scheme の数値「タワー」](https://doc.guix.gnu.org/guile/latest/en/guile.html#Numerical-Tower)、上: [数値データ型](https://doc.guix.gnu.org/guile/latest/en/guile.html#Numbers) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.2.2 整数 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Integers-1)

整数とは、小数部分を持たない数のことで、例えば2、83、 -3789などです。

Guileにおける整数は、次の例に示すように、任意に大きな値をとることができます。

(define (factorial n)
(let loop ((nn) (product 1))
(if ([\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) n 0)
製品
(ループ ([\-](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002d) n 1) ([\*](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002a) 積 n)))))

（3の階乗）
⇒ 6

（20の階乗）
⇒ 2432902008176640000

([\-](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002d) (階乗 45))
⇒ \-119622220865480194561963161495657715064383733760000000000

整数が4バイトまたは8バイトのメモリに収まるように制限されているプログラミング言語に慣れている読者は、これを意外に思ったり、Guileの整数表現が非効率的だと疑ったりするかもしれません。しかし実際には、Guileは可能な限りホストコンピュータのネイティブな整数表現を使用し、必要な数値がネイティブ形式に収まらない場合はより一般的な表現を使用することで、利便性と効率性のほぼ最適なバランスを実現しています。これら2つの表現間の変換は自動的に行われ、Schemeレベルのプログラマーには全く見えません。

C言語には様々な整数型があり、Guileはそれらと`SCM`表現との間で変換を行うための多数の関数を提供しています。例えば、C言語の`int`は`scm_to_int`や`scm_from_int`で扱うことができます。Guileはシステム間の違いに対応するため、独自のC言語の整数型もいくつか定義しています。

対象外の C 整数型は、符号付き型の場合は汎用関数 `scm_to_signed_integer` と `scm_from_signed_integer` を、符号なし型の場合は `scm_to_unsigned_integer` と `scm_from_unsigned_integer` を使用して処理できます。

Scheme の整数は、正確な整数と不正確な整数があります。たとえば、小数点を明示的に付けて `3.0` と表記された数値は不正確ですが、整数でもあります。関数 `integer?` と `scm_is_integer` は、このような数値に対して true を返しますが、関数 `exact-integer?`、`scm_is_exact_integer`、`scm_is_signed_integer`、および `scm_is_unsigned_integer` は正確な整数のみを受け入れるため、false を返します。同様に、`scm_to_signed_integer` のような変換関数も、正確な整数のみを受け入れます。

この動作の理由は、数値の不正確さが黙って失われるべきではないからです。不正確な整数を許可したい場合は、`inexact->exact` またはその C 版である `scm_inexact_to_exact` を明示的に呼び出すことができます。（この呼び出しによって正確な整数に変換されるのは不正確な整数のみであり、不正確な非整数は正確な分数になります。）

スキームプロシージャ: **integer?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-integer_003f)

C 関数: **scm\_integer\_p** (x) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005finteger_005fp)

xが正確な整数または不正確な整数である場合は`#t`を返し、そうでない場合は`#f`を返します。

([整数?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-integer_003f) 487)
⇒ #t

([整数?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-integer_003f) 3.0)
⇒ #t

([整数?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-integer_003f) \-3.4)
⇒ #f

([integer?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-integer_003f) +inf.0)
⇒ #f

C 関数: `int` **scm\_is\_integer** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fis_005finteger)

これは `scm_is_true (scm_integer_p (x))` と同等です。

スキームプロシージャ: **exact-integer?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-exact_002dinteger_003f)

C 関数: **scm\_exact\_integer\_p** (x) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fexact_005finteger_005fp)

xが正確な整数であれば`#t`を返し、そうでなければ`#f`を返します。

([正確な整数?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-exact_002dinteger_003f) 37)
⇒ #t

([exact-integer?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-exact_002dinteger_003f) 3.0)
⇒ #f

C 関数: `int` **scm\_is\_exact\_integer** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fis_005fexact_005finteger)

これは `scm_is_true (scm_exact_integer_p (x))` と同等です。

C 型: **scm\_t\_int8** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ft_005fint8)

C 型: **scm\_t\_uint8** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ft_005fuint8)

C 型: **scm\_t\_int16** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ft_005fint16)

C 型: **scm\_t\_uint16** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ft_005fuint16)

C 型: **scm\_t\_int32** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ft_005fint32)

C 型: **scm\_t\_uint32** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ft_005fuint32)

C 型: **scm\_t\_int64** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ft_005fint64)

C 型: **scm\_t\_uint64** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ft_005fuint64)

C 型: **scm\_t\_intmax** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ft_005fintmax)

C 型: **scm\_t\_uintmax** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ft_005fuintmax)

C 型は対応する ISO C 型と同等ですが、`scm_t_int64` と `scm_t_uint64` を除き、すべてのプラットフォームで定義されています。これらの型は 64 ビット型が利用可能な場合にのみ定義されます。たとえば、`scm_t_int8` は `int8_t` と同等です。

これらの定義は、すべてのプラットフォームがこれらの型を提供するようになるまでの暫定的な措置と考えてください。もし、関心のあるすべてのプラットフォームが既にこれらの型を提供していることがわかっている場合は、Guileが提供する型ではなく、それらの型を直接使用することをお勧めします。

C 関数: `int` **scm\_is\_signed\_integer** `(SCM x, scm_t_intmax min, scm_t_intmax max)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fis_005fsigned_005finteger)

C 関数: `int` **scm\_is\_unsigned\_integer** `(SCM x, scm_t_uintmax min, scm_t_uintmax max)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fis_005funsigned_005finteger)

xがminとmaxの間の正確な整数を表す場合は、`1`を返します。

これらの関数は、`SCM` 値が指定された範囲（例えば、特定の C 整数型の範囲）に収まるかどうかを確認するために使用できます。単に `SCM` 値を特定の C 整数型に変換したい場合は、変換関数のいずれかを直接使用してください。

C 関数: `scm_t_intmax` **scm\_to\_signed\_integer** `(SCM x, scm_t_intmax min, scm_t_intmax max)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005fsigned_005finteger)

C 関数: `scm_t_uintmax` **scm\_to\_unsigned\_integer** `(SCM x, scm_t_uintmax min, scm_t_uintmax max)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005funsigned_005finteger)

xがminとmaxの範囲内の正確な整数を表す場合は、その整数を返します。そうでない場合は、エラーを通知します。xが正確な整数でない場合は「型エラー」、指定された範囲に収まらない場合は「範囲外エラー」を返します。

C 関数: `SCM` **scm\_from\_signed\_integer** `(scm_t_intmax x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005fsigned_005finteger)

C 関数: `SCM` **scm\_from\_unsigned\_integer** `(scm_t_uintmax x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005funsigned_005finteger)

整数xを表す`SCM`値を返します。この関数は常に成功し、常に正確な数値を返します。

C 関数: `char` **scm\_to\_char** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005fchar)

C 関数: `signed char` **scm\_to\_schar** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005fschar)

C 関数: `unsigned char` **scm\_to\_uchar** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005fuchar)

C 関数: `short` **scm\_to\_short** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005fshort)

C 関数: `unsigned short` **scm\_to\_ushort** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005fushort)

C 関数: `int` **scm\_to\_int** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005fint)

C 関数: `unsigned int` **scm\_to\_uint** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005fuint)

C 関数: `long` **scm\_to\_long** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005flong)

C 関数: `unsigned long` **scm\_to\_ulong** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005fulong)

C 関数: `long long` **scm\_to\_long\_long** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005flong_005flong)

C 関数: `unsigned long long` **scm\_to\_ulong\_long** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005fulong_005flong)

C 関数: `size_t` **scm\_to\_size\_t** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005fsize_005ft)

C 関数: `ssize_t` **scm\_to\_ssize\_t** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005fssize_005ft)

C 関数: `scm_t_uintptr` **scm\_to\_uintptr\_t** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005fuintptr_005ft)

C 関数: `scm_t_ptrdiff` **scm\_to\_ptrdiff\_t** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005fptrdiff_005ft)

C 関数: `scm_t_int8` **scm\_to\_int8** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005fint8)

C 関数: `scm_t_uint8` **scm\_to\_uint8** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005fuint8)

C 関数: `scm_t_int16` **scm\_to\_int16** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005fint16)

C 関数: `scm_t_uint16` **scm\_to\_uint16** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005fuint16)

C 関数: `scm_t_int32` **scm\_to\_int32** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005fint32)

C 関数: `scm_t_uint32` **scm\_to\_uint32** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005fuint32)

C 関数: `scm_t_int64` **scm\_to\_int64** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005fint64)

C 関数: `scm_t_uint64` **scm\_to\_uint64** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005fuint64)

C 関数: `scm_t_intmax` **scm\_to\_intmax** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005fintmax)

C 関数: `scm_t_uintmax` **scm\_to\_uintmax** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005fuintmax)

C 関数: `scm_t_intptr` **scm\_to\_intptr\_t** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005fintptr_005ft)

C 関数: `scm_t_uintptr` **scm\_to\_uintptr\_t** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005fuintptr_005ft-1)

xが指定されたC型に収まる正確な整数を表す場合は、その整数を返します。そうでない場合は、エラーを通知します。xが正確な整数でない場合は「型間違い」エラー、指定された範囲に収まらない場合は「範囲外」エラーを返します。

関数 `scm_to_long_long`、`scm_to_ulong_long`、`scm_to_int64`、および `scm_to_uint64` は、対応する型が存在する場合にのみ使用できます。

C 関数: `SCM` **scm\_from\_char** `(char x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005fchar)

C 関数: `SCM` **scm\_from\_schar** `(signed char x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005fschar)

C 関数: `SCM` **scm\_from\_uchar** `(unsigned char x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005fuchar)

C 関数: `SCM` **scm\_from\_short** `(short x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005fshort)

C 関数: `SCM` **scm\_from\_ushort** `(unsigned short x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005fushort)

C 関数: `SCM` **scm\_from\_int** `(int x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005fint)

C 関数: `SCM` **scm\_from\_uint** `(unsigned int x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005fuint)

C 関数: `SCM` **scm\_from\_long** `(long x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005flong)

C 関数: `SCM` **scm\_from\_ulong** `(unsigned long x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005fulong)

C 関数: `SCM` **scm\_from\_long\_long** `(long long x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005flong_005flong)

C 関数: `SCM` **scm\_from\_ulong\_long** `(unsigned long long x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005fulong_005flong)

C 関数: `SCM` **scm\_from\_size\_t** `(size_t x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005fsize_005ft)

C 関数: `SCM` **scm\_from\_ssize\_t** `(ssize_t x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005fssize_005ft)

C 関数: `SCM` **scm\_from\_uintptr\_t** `(uintptr_t x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005fuintptr_005ft)

C 関数: `SCM` **scm\_from\_ptrdiff\_t** `(scm_t_ptrdiff x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005fptrdiff_005ft)

C 関数: `SCM` **scm\_from\_int8** `(scm_t_int8 x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005fint8)

C 関数: `SCM` **scm\_from\_uint8** `(scm_t_uint8 x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005fuint8)

C 関数: `SCM` **scm\_from\_int16** `(scm_t_int16 x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005fint16)

C 関数: `SCM` **scm\_from\_uint16** `(scm_t_uint16 x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005fuint16)

C 関数: `SCM` **scm\_from\_int32** `(scm_t_int32 x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005fint32)

C 関数: `SCM` **scm\_from\_uint32** `(scm_t_uint32 x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005fuint32)

C 関数: `SCM` **scm\_from\_int64** `(scm_t_int64 x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005fint64)

C 関数: `SCM` **scm\_from\_uint64** `(scm_t_uint64 x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005fuint64)

C 関数: `SCM` **scm\_from\_intmax** `(scm_t_intmax x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005fintmax)

C 関数: `SCM` **scm\_from\_uintmax** `(scm_t_uintmax x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005fuintmax)

C 関数: `SCM` **scm\_from\_intptr\_t** `(scm_t_intptr x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005fintptr_005ft)

C 関数: `SCM` **scm\_from\_uintptr\_t** `(scm_t_uintptr x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005fuintptr_005ft-1)

整数xを表す`SCM`値を返します。これらの関数は常に成功し、常に正確な数値を返します。

C 関数: `void` **scm\_to\_mpz** `(SCM val, mpz_t rop)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005fmpz)

val を多倍長整数 rop に代入します。val は正確な整数でなければならず、そうでない場合はエラーが通知されます。この関数を呼び出す前に、rop は `mpz_init` で初期化されている必要があります。rop が不要になった場合は、占有されている領域を `mpz_clear` で解放する必要があります。詳細については、GNU MP マニュアルの [Initializing Integers](https://www.gmplib.org/manual/Initializing-Integers.html#Initializing-Integers) を参照してください。

C 関数: `SCM` **scm\_from\_mpz** `(mpz_t val)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005fmpz)

val を表す `SCM` 値を返します。

* * *

次へ: [複素数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Complex-Numbers)、前: [整数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Integers)、上: [数値データ型](https://doc.guix.gnu.org/guile/latest/en/guile.html#Numbers) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.2.3 実数と有理数 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Real-and-Rational-Numbers)

数学的に、実数とは、連続的で無限な一次元直線上のあらゆる点を表す数の集合です。有理数とは、整数 p と q を用いて分数 p/q の形で表せるすべての数の集合です。すべての有理数は実数でもありますが、実数の中には有理数ではないものも存在します。例えば、√2 や π などです。

Guileは、正確な有理数と不正確な有理数の両方を表現できますが、正確な有限無理数は表現できません。正確な有理数は、分子と分母をそれぞれ正確な整数として格納することで表現されます。不正確な有理数は、C言語の型`double`を用いた浮動小数点数として格納されます。

正確な有理数は、整数の分数として表記します。スラッシュの前後には空白を入れてはいけません。

1/2
-22/7

不正確な有理数の実際の符号化は二進数ですが、小数点以下の桁数が限られている十進数と考えると分かりやすいかもしれません。これは、整数以外の数の標準的な表記法に対応しているからです。例えば、次のようになります。

0.34
-0.00000142857931198
-5648394822220000000000.0
4.0

Guileのエンコーディングの精度が限られているため、Guileの有限な「実数」は、10の十分なべき乗（実際には2のべき乗）を掛けてから割ることで、有理数形式で表すことができます。たとえば、「\-0.00000142857931198」は、-142857931198を100000000000000000で割ったものと同じです。したがって、現在のGuileのバージョンでは、有限数に対して「有理数？」と「実数？」の述語は同等です。

正確にゼロで割ると、予想通りエラーメッセージが表示されます。しかし、正確にゼロでないゼロで割ってもエラーは発生しません。代わりに、除算の結果は、割られる数の符号とゼロ除数の符号に応じて、プラスまたはマイナスの無限大になります（一部のプラットフォームでは、符号付きゼロ「\-0.0」と「+0.0」がサポートされています。「0.0」は「+0.0」と同じです）。

ゼロを不正確なゼロで割ると、NaN（「非数」）値が得られますが、Schemeでは実際には数値として扱われます。NaN値を任意の数値（NaN値自身を含む）と`=`、`<`、`>`、`<=`、`>=`を使用して比較しようとすると、常に`#f`が返されます。NaN値はNaN値自身とは`=`できませんが、NaN値自身および他のNaN値とは`eqv?`および`equal?`できます。ただし、NaN値をテストする推奨方法は`nan?`を使用することです。

実際の NaN 値と無限大は、'+nan.0'、'+inf.0'、'\-inf.0' と表記されます。この構文は、通常の Scheme 構文の拡張として `read` にも認識されます。これらの特殊な値は、Scheme では不正確な実数とみなされますが、有理数とはみなされません。非実数の複素数も、実部または虚部に無限大または NaN 値を含む場合があることに注意してください。実数が無限大、NaN 値、またはどちらでもないかをテストするには、それぞれ `inf?`、`nan?`、または `finite?` を使用します。Scheme のすべての実数は、これら 3 つのクラスのいずれかに正確に属します。

浮動小数点演算に IEEE 754 規格を採用しているプラットフォームでは、'+inf.0'、'\-inf.0'、および '+nan.0' の値は、対応する IEEE 754 規格の値を使用して実装されます。これらの値は、IEEE 754 規格で規定されているように、演算において `(= +nan.0 +nan.0)` ⇒ `#f` のように動作します。

Scheme Procedure: **real?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-real_003f)

C 関数: **scm\_real\_p** (obj) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005freal_005fp)

objが実数の場合は`#t`を返し、そうでない場合は`#f`を返します。整数と有理数の集合は実数の集合の部分集合を形成するため、objが整数または有理数の場合にも述語は満たされます。

スキーム手順: **有理数?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-rational_003f)

C 関数: **scm\_rational\_p** (x) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005frational_005fp)

xが有理数の場合は`#t`を、そうでない場合は`#f`を返します。整数値の集合は有理数の集合の部分集合を形成するため、xが整数であれば述語も満たされることに注意してください。

Scheme Procedure: **rationalize** x eps [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-rationalize)

C 関数: **scm\_rationalize** (x, eps) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005frationalize)

x との差が eps 以内の最も単純な有理数を返します。

R5RSの要件に従い、`rationalize`は引数が両方とも正確な場合にのみ正確な結果を返します。したがって、引数に対して`inexact->exact`を使用する必要があるかもしれません。

([rationalize](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-rationalize) ([inexact->exact](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-inexact_002d_003eexact) 1.2) 1/100)
⇒ 6/5

スキームプロシージャ: **inf?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-inf_003f)

C 関数: **scm\_inf\_p** (x) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005finf_005fp)

実数 x が '+inf.0' または '\-inf.0' の場合は `#t` を返します。それ以外の場合は `#f` を返します。

スキーム手順: **nan?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-nan_003f)

C 関数: **scm\_nan\_p** (x) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fnan_005fp)

実数 x が '+nan.0' の場合は `#t` を返し、それ以外の場合は `#f` を返します。

スキーム手順: **有限？** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-finite_003f)

C 関数: **scm\_finite\_p** (x) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffinite_005fp)

実数 x が無限大でも NaN でもない場合は `#t` を返し、それ以外の場合は `#f` を返します。

スキーム手順: **nan** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-nan)

C言語関数: **scm\_nan** () [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fnan)

NaN値である「+nan.0」を返します。

スキームプロシージャ: **inf** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-inf)

C 関数: **scm\_inf** () [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005finf)

'+inf.0'（正の無限大）を返します。

スキーム手順: **numerator** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-numerator)

C 関数: **scm\_numerator** (x) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fnumerator)

有理数 x の分子を返します。

スキーム手順: **分母** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-denominator)

C 関数: **scm\_denominator** (x) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fdenominator)

有理数 x の分母を返します。

C 関数: `int` **scm\_is\_real** `(SCM 値)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fis_005freal)

C 関数: `int` **scm\_is\_rational** `(SCM 値)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fis_005frational)

それぞれ `scm_is_true (scm_real_p (val))` および `scm_is_true (scm_rational_p (val))` と同等です。

C 関数: `double` **scm\_to\_double** `(SCM 値)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005fdouble)

`double`型で表現可能な値のうち、`val`に最も近い値を返します。`val`が大きすぎる場合は無限大を返します。引数`val`は実数でなければなりません。

C 関数: `SCM` **scm\_from\_double** `(double val)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005fdouble)

valを表す`SCM`値を返します。返される値は述語`inexact?`に従って不正確ですが、valと完全に等しくなります。

* * *

次へ: [正確な数と不正確な数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Exactness)、前: [実数と有理数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Reals-and-Rationals)、上: [数値データ型](https://doc.guix.gnu.org/guile/latest/en/guile.html#Numbers) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.2.4 複素数 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Complex-Numbers-1)

複素数とは、2次元空間におけるあらゆる点を表す数の集合です。この空間における特定の点の2つの座標は、その点を表す複素数の実部と虚部と呼ばれます。

Guileでは、複素数は実部と虚部の和として直交座標形式で表され、虚部は記号「i」で示されます。

3+4i
⇒
3.0+4.0i

([\*](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002a) 3-8i 2.3+0.3i)
⇒
9.3-17.5i

極座標形式も使用できます。大きさと角度の間に「@」を入れます。

1@3.141592 ⇒ -1.0 (概算)
-1@1.57079 ⇒ 0.0-1.0i (概算)

Guileは複素数を不正確な実数のペアとして表現するため、複素数の実部と虚部は、単一の不正確な実数と同様に、不正確さと限られた精度という性質を持つ。

複素数の各部分は、特殊値「+nan.0」、「+inf.0」、「\-inf.0」、および符号付きゼロ「0.0」または「\-0.0」を含む、任意の不正確な実数値を含む可能性があることに注意してください。

スキーム手順: **complex?** z [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-complex_003f)

C 関数: **scm\_complex\_p** (z) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fcomplex_005fp)

zが複素数の場合は`#t`を返し、それ以外の場合は`#f`を返します。実数、有理数、整数の集合は複素数の集合の部分集合を形成するため、zが実数、有理数、または整数である場合にも述語は満たされます。

C 関数: `int` **scm\_is\_complex** `(SCM 値)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fis_005fcomplex)

`scm_is_true (scm_complex_p (val))` と同等です。

* * *

次へ: [数値データの構文を読む](https://doc.guix.gnu.org/guile/latest/en/guile.html#Number-Syntax)、前: [複素数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Complex-Numbers)、上: [数値データ型](https://doc.guix.gnu.org/guile/latest/en/guile.html#Numbers) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.2.5 正確な数と不正確な数 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Exact-and-Inexact-Numbers)

R5RSでは、ごく一部の例外を除き、不正確な数値を含む計算結果は常に不正確な値になることが求められます。この要件を満たすため、Guileは「5」のような正確な整数値と、限られた精度では小数部を持たない対応する不正確な整数値（「5.0」と表示される）を区別します。Guileは、`inexact->exact`プロシージャの呼び出しによって強制された場合にのみ、後者の値を前者に変換します。

上記の要件に対する唯一の例外は、不正確な数値の値が結果に影響を与えない場合です。たとえば、`(expt n 0)` は `n` の任意の値に対して '1' となるため、`(expt 5.0 0)` は正確な '1' を返すことが許可されます。

Scheme Procedure: **exact?** z [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-exact_003f)

C 関数: **scm\_exact\_p** (z) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fexact_005fp)

数値zが正確な値であれば`#t`を返し、そうでなければ`#f`を返します。

([exact?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-exact_003f) 2)
⇒ #t

([exact?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-exact_003f) 0.5)
⇒ #f

([exact?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-exact_003f) ([/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002f) 2))
⇒ #t

C 関数: `int` **scm\_is\_exact** `(SCM z)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fis_005fexact)

数値 z が正確な値であれば 1 を返し、そうでなければ 0 を返します。これは `scm_is_true (scm_exact_p (z))` と同等です。

数値の正確さをテストする別の方法として、`scm_is_signed_integer` または `scm_is_unsigned_integer` を使用する方法があります。

Scheme Procedure: **inexact?** z [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-inexact_003f)

C 関数: **scm\_inexact\_p** (z) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005finexact_005fp)

数値zが不正確な場合は`#t`を返し、そうでない場合は`#f`を返します。

C 関数: `int` **scm\_is\_inexact** `(SCM z)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fis_005finexact)

数値zが不正確な場合は`1`を返し、そうでない場合は`0`を返します。これは`scm_is_true (scm_inexact_p (z))`と同等です。

Scheme手順: **inexact->exact** z [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-inexact_002d_003eexact)

C 関数: **scm\_inexact\_to\_exact** (z) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005finexact_005fto_005fexact)

zに最も近い正確な数が存在する場合は、それを返します。不正確な有理数の場合、Guileは不正確な有理数と数値的に等しい正確な有理数を返します。虚数部がゼロでない不正確な複素数は、正確な数に変換できません。

([inexact->exact](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-inexact_002d_003eexact) 0.5)
⇒ 1/2

12/10 は（ほとんどのプラットフォームで）`double` 型として正確に表現できないため、このような現象が発生します。しかし、「#e」という接頭辞で正確とマークされた小数値を読み取る場合、Guile はそれを正しく表現できます。

([inexact->exact](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-inexact_002d_003eexact) 1.2)
⇒ 5404319552844595/4503599627370496

#e1.2
⇒ 6/5

スキーム手順: **exact->inexact** z [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-exact_002d_003einexact)

C 関数: **scm\_exact\_to\_inexact** (z) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fexact_005fto_005finexact)

数値zをその不正確な表現に変換します。

* * *

次へ: [整数値の演算](https://doc.guix.gnu.org/guile/latest/en/guile.html#Integer-Operations)、前: [正確な数と不正確な数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Exactness)、上: [数値データ型](https://doc.guix.gnu.org/guile/latest/en/guile.html#Numbers) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.2.6 数値データの読み取り構文 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Read-Syntax-for-Numerical-Data)

整数の読み取り構文は、数字列、オプションでマイナスまたはプラス文字、整数がエンコードされている基数を示すコード、および数値が正確か不正確かを示すコードで構成されます。サポートされている基数コードは次のとおりです。

`#b`

`#B`

整数は2進数（基数2）で表記されます。

`#o`

`#O`

整数は8進数（基数8）で表記されます。

`#d`

`#D`

整数は10進数（基数10）で表記されます。

`#x`

`#X`

整数は16進数（基数16）で表記されます。

基数コードが省略されている場合、整数は10進数とみなされます。以下の例は、これらの基数コードの使用方法を示しています。

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

正確さを示すためのコード（ちなみに、これはすべての数値に適用できます）は以下のとおりです。

`#e`

`#E`

その数字は正確です

`#i`

`#I`

その数字は正確ではありません。

正確度指定子を省略した場合、小数点を含まない限り、数値は正確な値とみなされます。Guileは正確な複素数を表現できないため、複素数を要求した際にはエラーが発生します。

([exact?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-exact_003f) 1.2)
⇒ #f

([exact?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-exact_003f) #e1.2)
⇒ #t

([exact?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-exact_003f) #e+1i)
エラー: 引数の型が間違っています

Guile は、プラス無限大を表す構文 '+inf.0' とマイナス無限大を表す構文 '\-inf.0' も認識します。値は、示されているとおりに正確に記述する必要があります。つまり、常に符号があり、小数点以下に 1 桁のゼロがなければなりません。また、特殊な「非数値」値を表す構文 '+nan.0' と '\-nan.0' も認識します。「非数値」の場合、符号は無視され、値は常に '+nan.0' として出力されます。

* * *

次へ: [比較述語](https://doc.guix.gnu.org/guile/latest/en/guile.html#Comparison)、前: [数値データの構文の読み方](https://doc.guix.gnu.org/guile/latest/en/guile.html#Number-Syntax)、上: [数値データ型](https://doc.guix.gnu.org/guile/latest/en/guile.html#Numbers) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.2.7 整数値に対する演算 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Operations-on-Integer-Values)

スキーム手順: **odd?** n [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-odd_003f)

C 関数: **scm\_odd\_p** (n) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fodd_005fp)

nが奇数の場合は`#t`を返し、それ以外の場合は`#f`を返します。

スキーム手順: **even?** n [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-even_003f)

C 関数: **scm\_even\_p** (n) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005feven_005fp)

nが偶数の場合は`#t`を返し、それ以外の場合は`#f`を返します。

スキーム手順: **quotient** nd [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-quotient)

スキーム手順: **remainder** nd [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-remainder)

C 関数: **scm\_quotient** (n, d) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fquotient)

C 関数: **scm\_remainder** (n, d) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fremainder)

n を d で割った商または余りを返します。商はゼロ方向に丸められ、余りは n と同じ符号になります。すべての場合において、商と余りは _n = q\*d + r_ を満たします。

([remainder](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-remainder) 13 4) ⇒ 1
([remainder](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-remainder) \-13 4) ⇒ \-1

`truncate-quotient`、`truncate-remainder`、および関連する演算については、[算術関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Arithmetic)を参照してください。

スキーム手順: **modulo** nd [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-modulo)

C 関数: **scm\_modulo** (n, d) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fmodulo)

nをdで割った余りを、dと同じ符号で返します。

([モジュロ](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-modulo) 13 4) ⇒ 1
([modulo](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-modulo) \-13 4) ⇒ 3
([modulo](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-modulo) 13 \-4) ⇒ \-3
([modulo](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-modulo) \-13 \-4) ⇒ \-1

`floor-quotient`、`floor-remainder`、および関連する演算については、[算術関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Arithmetic)を参照してください。

スキーム手順: **gcd** x… [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-gcd)

C 関数: **scm\_gcd** (x, y) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fgcd)

すべての引数の最大公約数を返します。引数なしで呼び出した場合は、0が返されます。

C言語の関数`scm_gcd`は常に2つの引数を取りますが、Schemeの関数は任意の数の引数を取ることができます。

スキーム手順: **lcm** x… [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-lcm)

C 関数: **scm\_lcm** (x, y) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005flcm)

引数の最小公倍数を返します。引数なしで呼び出した場合は、1が返されます。

C言語の関数`scm_lcm`は常に2つの引数を取りますが、Schemeの関数は任意の数の引数を取ることができます。

スキーム手順: **modulo-expt** nkm [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-modulo_002dexpt)

C 関数: **scm\_modulo\_expt** (n, k, m) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fmodulo_005fexpt)

nを整数指数kで累乗し、mを法として返します。

([modulo-expt](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-modulo_002dexpt) 2 3 5)
⇒ 3

スキームプロシージャ: **exact-integer-sqrt** `k` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-exact_002dinteger_002dsqrt)

C 関数: `void` **scm\_exact\_integer\_sqrt** `(SCM k, SCM *s, SCM *r)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fexact_005finteger_005fsqrt)

_k = s^2 + r_ かつ _s^2 <= k < (s + 1)^2_ を満たす、正確な非負整数 s と r を返します。k が正確な非負整数でない場合はエラーが発生します。

([exact-integer-sqrt](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-exact_002dinteger_002dsqrt ) 10) ⇒ 3 と 1

* * *

次へ: [数値と文字列の変換](https://doc.guix.gnu.org/guile/latest/en/guile.html#Conversion)、前: [整数値の演算](https://doc.guix.gnu.org/guile/latest/en/guile.html#Integer-Operations)、上: [数値データ型](https://doc.guix.gnu.org/guile/latest/en/guile.html#Numbers) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.2.8 比較述語 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Comparison-Predicates)

以下のC言語の比較関数は常に2つの引数を取りますが、Schemeの関数は任意の数値を引数として取ることができます。また、C言語の関数はSchemeのブール値`SCM_BOOL_T`または`SCM_BOOL_F`のいずれかを返しますが、C言語の観点からはどちらも真です。したがって、例えば2つのScheme数値`x`と`y`の等価性をテストする場合は、常に`scm_is_true (scm_num_eq_p (x, y))`と記述してください。

Scheme Procedure: **\=** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d)

C 関数: **scm\_num\_eq\_p** (x, y) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fnum_005feq_005fp)

すべてのパラメータが数値的に等しい場合は、`#t`を返します。

スキーム手順: **<** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003c)

C 関数: **scm\_less\_p** (x, y) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fless_005fp)

パラメータのリストが単調増加している場合は、`#t` を返します。

スキーム手順: **\>** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e)

C 関数: **scm\_gr\_p** (x, y) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fgr_005fp)

パラメータのリストが単調減少している場合は、`#t` を返します。

Scheme Procedure: **<=** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003c_003d)

C 関数: **scm\_leq\_p** (x, y) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fleq_005fp)

パラメータのリストが単調非減少である場合は、`#t` を返します。

Scheme Procedure: **\>=** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e_003d)

C 関数: **scm\_geq\_p** (x, y) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fgeq_005fp)

パラメータのリストが単調非増加である場合は、`#t` を返します。

スキーム手順: **ゼロ？** z [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-zero_003f)

C 関数: **scm\_zero\_p** (z) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fzero_005fp)

zがゼロに等しい正確な数または不正確な数である場合は、`#t`を返します。

スキーム手順: **肯定？** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-positive_003f)

C 関数: **scm\_positive\_p** (x) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fpositive_005fp)

xが0より大きい正確な数または不正確な数である場合は、`#t`を返します。

スキーム手順: **否定？** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-negative_003f)

C 関数: **scm\_negative\_p** (x) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fnegative_005fp)

xが0未満の正確な数または不正確な数である場合は、`#t`を返します。

* * *

次へ: [複素数演算](https://doc.guix.gnu.org/guile/latest/en/guile.html#Complex)、前: [比較述語](https://doc.guix.gnu.org/guile/latest/en/guile.html#Comparison)、上: [数値データ型](https://doc.guix.gnu.org/guile/latest/en/guile.html#Numbers) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.2.9 数値と文字列の変換 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Converting-Numbers-To-and-From-Strings)

以下の手順では、R5RS で定義されている外部表現に従って数値を読み書きします (「アルゴリズム言語スキームに関する改訂版レポート」の [R5RS 字句構造](https://doc.guix.gnu.org/r5rs/latest/en/r5rs.html#Lexical-structure) を参照)。ロケールに依存する数値の解析については、[`(ice-9 i18n)` モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#Number-Input-and-Output) を参照してください。

Scheme手順: **number->string** n \[radix\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-number_002d_003estring)

C 関数: **scm\_number\_to\_string** (n, radix) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fnumber_005fto_005fstring)

指定された基数における数値 n の外部表現を保持する文字列を返します。n が正確な値でない場合は、基数 10 が使用されます。

Scheme手順: **string->number** string \[radix\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002d_003enumber)

C 関数: **scm\_string\_to\_number** (string, radix) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fto_005fnumber)

指定された文字列で表現される最も正確な数値を返します。 radix は、2、8、10、または 16 のいずれかの正確な整数である必要があります。 radix が指定されている場合、デフォルトの基数は文字列内の明示的な基数接頭辞 (例: "#o177") で上書きできます。 radix が指定されていない場合、デフォルトの基数は 10 です。 文字列が数値の構文的に有効な表記でない場合、`string->number` は `#f` を返します。

C 関数: `SCM` **scm\_c\_locale\_stringn\_to\_number** `(const char *string, size_t len, unsigned radix)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005flocale_005fstringn_005fto_005fnumber)

上記の `string->number` と同様ですが、ポインタと長さとして C 言語の文字列を受け取ります。文字列の文字は現在のロケールのエンコーディングである必要があります (名前の `locale` はそれだけを指し、ロケールに依存する解析は行われません)。

* * *

次へ: [算術関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Arithmetic)、前: [数値と文字列の変換](https://doc.guix.gnu.org/guile/latest/en/guile.html#Conversion)、上: [数値データ型](https://doc.guix.gnu.org/guile/latest/en/guile.html#Numbers) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.2.10 複素数演算 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Complex-Number-Operations)

スキーム手順: **make-rectangular** 実部 虚部 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002drectangular)

C 関数: **scm\_make\_rectangular** (実部、虚部) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fmake_005frectangular)

指定された実部と虚部から構成される複素数を返します。

Scheme Procedure: **make-polar** mag ang [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dpolar)

C 関数: **scm\_make\_polar** (mag, ang) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fmake_005fpolar)

複素数 mag \* e^(i \* ang) を返します。

スキーム手順: **real-part** z [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-real_002dpart)

C 関数: **scm\_real\_part** (z) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005freal_005fpart)

数値zの実数部を返します。

スキーム手順: **imag-part** z [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-imag_002dpart)

C 関数: **scm\_imag\_part** (z) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fimag_005fpart)

数値zの虚数部を返します。

スキーム手順: **magnitude** z [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-magnitude)

C 関数: **scm\_magnitude** (z) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fmagnitude)

数値zの絶対値を返します。これは実数引数に対する`abs`と同じですが、複素数も指定できます。

スキーム手順: **角度** z [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-angle)

C 関数: **scm\_angle** (z) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fangle)

複素数zの角度を返します。

C 関数: `SCM` **scm\_c\_make\_rectangular** `(double re, double im)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fmake_005frectangular)

C 関数: `SCM` **scm\_c\_make\_polar** `(double x, double y)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fmake_005fpolar)

それぞれ `scm_make_rectangular` や `scm_make_polar` と同様ですが、これらの関数は引数として `double` を受け取ります。

C 関数: `double` **scm\_c\_real\_part** `(z)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005freal_005fpart)

C 関数: `double` **scm\_c\_imag\_part** `(z)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fimag_005fpart)

zの実部または虚部を`double`型で返します。

C 関数: `double` **scm\_c\_magnitude** `(z)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fmagnitude)

C 関数: `double` **scm\_c\_angle** `(z)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fangle)

zの大きさまたは角度を`double`型で返します。

* * *

次へ: [科学関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scientific)、前: [複素数演算](https://doc.guix.gnu.org/guile/latest/en/guile.html#Complex)、上: [数値データ型](https://doc.guix.gnu.org/guile/latest/en/guile.html#Numbers) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.2.11 算術関数 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Arithmetic-Functions)

以下の C 言語の算術関数は常に 2 つの引数を取りますが、Scheme の関数は任意の数値を取ることができます。例えば、`(- x)` に相当する計算を行うなど、1 つの引数だけで呼び出す必要がある場合は、2 番目の引数として `SCM_UNDEFINED` を渡します。例: `scm_difference (x, SCM_UNDEFINED)`。

スキーム手順: **+** z1 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b)

C 関数: **scm\_sum** (z1, z2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsum)

すべてのパラメータ値の合計を返します。パラメータを指定せずに呼び出した場合は0を返します。

スキーム手順: **\-** z1 z2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002d)

C 関数: **scm\_difference** (z1, z2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fdifference)

引数z1を1つだけ指定して呼び出した場合、-z1が返されます。それ以外の場合は、最初の引数から最初の引数以外のすべての引数の合計が減算されます。

スキームプロシージャ: **\*** z1 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002a)

C 関数: **scm\_product** (z1, z2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fproduct)

すべての引数の積を返します。引数なしで呼び出した場合は、1が返されます。

スキーム手順: **/** z1 z2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002f)

C 関数: **scm\_divide** (z1, z2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fdivide)

最初の引数を、残りの引数の積で割ります。引数がz1のみの場合、1/z1が返されます。

スキーム手順: **1+** z [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-1_002b-1)

C 関数: **scm\_oneplus** (z) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005foneplus)

_z + 1_ を返します。

スキーム手順: **1-** z [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-1_002d-1)

C 関数: **scm\_oneminus** (z) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005foneminus)

_z - 1_ を返します。

スキーム手順: **abs** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-abs)

C 関数: **scm\_abs** (x) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fabs)

xの絶対値を返します。

xは虚数部がゼロの数値でなければなりません。複素数の大きさを計算するには、代わりに`magnitude`を使用してください。

スキーム手順: **max** x1 x2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-max)

C 関数: **scm\_max** (x1, x2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fmax)

すべてのパラメータ値のうち、最大値を返します。

スキーム手順: **min** x1 x2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-min)

C 関数: **scm\_min** (x1, x2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fmin)

すべてのパラメータ値のうち最小値を返します。

Scheme手順: **truncate** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-truncate)

C 関数: **scm\_truncate\_number** (x) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ftruncate_005fnumber)

不正確な数値xをゼロ方向に丸める。

スキーム手順: **round** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-round)

C 関数: **scm\_round\_number** (x) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fround_005fnumber)

不正確な数値 x を最も近い整数に丸めます。2 つの整数のちょうど中間値の場合は、偶数に丸めます。

スキーム手順: **floor** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-floor)

C 関数: **scm\_floor** (x) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffloor)

数値xをマイナス無限大の方向に四捨五入します。

スキーム手順: **ceiling** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ceiling)

C 関数: **scm\_ceiling** (x) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fceiling)

数値xを無限大の方向に丸める。

C 関数: `double` **scm\_c\_truncate** `(double x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005ftruncate)

C 関数: `double` **scm\_c\_round** `(double x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fround)

それぞれ `scm_truncate_number` や `scm_round_number` と同様ですが、これらの関数は `double` 型の値を引数として受け取り、戻り値として返します。

Scheme手順: **euclidean/** `xy` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-euclidean_002f)

Scheme手順: **euclidean-quotient** `xy` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-euclidean_002dquotient)

Scheme手順: **euclidean-remainder** `xy` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-euclidean_002dremainder)

C 関数: `void` **scm\_euclidean\_divide** `(SCM x, SCM y, SCM *q, SCM *r)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005feuclidean_005fdivide)

C 関数: `SCM` **scm\_euclidean\_quotient** `(SCM x, SCM y)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005feuclidean_005fquotient)

C 関数: `SCM` **scm\_euclidean\_remainder** `(SCM x, SCM y)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005feuclidean_005fremainder)

これらの手順では、2 つの実数 x と y を受け取ります。ただし、除数 y はゼロ以外でなければなりません。`euclidean-quotient` は整数 q を返し、`euclidean-remainder` は _x = q\*y + r_ かつ _0 <= r < |y|_ を満たす実数 r を返します。`euclidean/` は q と r の両方を返し、それぞれを個別に計算するよりも効率的です。_y > 0_ の場合、`euclidean-quotient` は _floor(x/y)_ を返し、それ以外の場合は _ceiling(x/y)_ を返すことに注意してください。

これらの演算子は、R6RSの演算子`div`、`mod`、および`div-and-mod`に相当することに注意してください。

([euclidean-quotient](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-euclidean_002dquotient) 123 10) ⇒ 12
([euclidean-remainder](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-euclidean_002dremainder) 123 10) ⇒ 3
([euclidean/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-euclidean_002f) 123 10) ⇒ 12 と 3
([euclidean/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-euclidean_002f) 123 \-10) ⇒ \-12 と 3
([euclidean/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-euclidean_002f) \-123 10) ⇒ \-13 および 7
([euclidean/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-euclidean_002f) \-123 \-10) ⇒ 13 と 7
([euclidean/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-euclidean_002f) \-123.2 \-63.5) ⇒ 2.0 および 3.8
([euclidean/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-euclidean_002f) 16/3 \-10/7) ⇒ \-3 および 22/21

スキーム手順: **floor/** `xy` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-floor_002f)

スキーム手順: **floor-quotient** `xy` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-floor_002dquotient)

スキーム手順: **floor-remainder** `xy` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-floor_002dremainder)

C 関数: `void` **scm\_floor\_divide** `(SCM x, SCM y, SCM *q, SCM *r)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffloor_005fdivide)

C 関数: `SCM` **scm\_floor\_quotient** `(x, y)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffloor_005fquotient)

C 関数: `SCM` **scm\_floor\_remainder** `(x, y)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffloor_005fremainder)

これらの手順は、2 つの実数 x と y を受け取ります。ただし、除数 y はゼロ以外でなければなりません。`floor-quotient` は整数 q を返し、`floor-remainder` は _q = floor(x/y)_ かつ _x = q\*y + r_ となる実数 r を返します。`floor/` は q と r の両方を返し、それぞれを個別に計算するよりも効率的です。なお、r がゼロ以外の場合は、y と同じ符号になります。

xとyが整数の場合、`floor-remainder`はR5RSの整数専用演算子`modulo`と同等です。

([floor-quotient](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-floor_002dquotient) 123 10) ⇒ 12
([floor-remainder](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-floor_002dremainder) 123 10) ⇒ 3
([floor/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-floor_002f) 123 10) ⇒ 12 と 3
([floor/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-floor_002f) 123 \-10) ⇒ \-13 および \-7
([floor/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-floor_002f) \-123 10) ⇒ \-13 および 7
([floor/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-floor_002f) \-123 \-10) ⇒ 12 と \-3
([floor/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-floor_002f) \-123.2 \-63.5) ⇒ 1.0 および \-59.7
([floor/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-floor_002f) 16/3 - 10/7) ⇒ -4 および -8/21

スキーム手順: **ceiling/** `xy` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ceiling_002f)

スキーム手順: **ceiling-quotient** `xy` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ceiling_002dquotient)

スキーム手順: **ceiling-remainder** `xy` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ceiling_002dremainder)

C 関数: `void` **scm\_ceiling\_divide** `(SCM x, SCM y, SCM *q, SCM *r)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fceiling_005fdivide)

C 関数: `SCM` **scm\_ceiling\_quotient** `(x, y)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fceiling_005fquotient)

C 関数: `SCM` **scm\_ceiling\_remainder** `(x, y)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fceiling_005fremainder)

これらの手順では、2 つの実数 x と y を受け取ります。ただし、除数 y はゼロ以外でなければなりません。`ceiling-quotient` は整数 q を返し、`ceiling-remainder` は _q = ceiling(x/y)_ かつ _x = q\*y + r_ となる実数 r を返します。`ceiling/` は q と r の両方を返し、それぞれを個別に計算するよりも効率的です。なお、r がゼロ以外の場合は、y とは逆の符号になります。

([ceiling-quotient](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ceiling_002dquotient) 123 10) ⇒ 13
([ceiling-remainder](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ceiling_002dremainder) 123 10) ⇒ \-7
([ceiling/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ceiling_002f) 123 10) ⇒ 13 および \-7
([ceiling/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ceiling_002f) 123 \- 10) ⇒ \-12 と 3
([ceiling/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ceiling_002f) \-123 10) ⇒ \-12 と \-3
([ceiling/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ceiling_002f) \-123 \-10) ⇒ 13 と 7
([ceiling/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ceiling_002f) \-123.2 \-63.5) ⇒ 2.0 および 3.8
([ceiling/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ceiling_002f) 16/3 \-10/7) ⇒ \-3 および 22/21

Scheme手順: **truncate/** `xy` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-truncate_002f)

Scheme Procedure: **truncate-quotient** `xy` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-truncate_002dquotient)

Scheme Procedure: **truncate-remainder** `xy` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-truncate_002dremainder)

C 関数: `void` **scm\_truncate\_divide** `(SCM x, SCM y, SCM *q, SCM *r)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ftruncate_005fdivide)

C 関数: `SCM` **scm\_truncate\_quotient** `(x, y)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ftruncate_005fquotient)

C 関数: `SCM` **scm\_truncate\_remainder** `(x, y)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ftruncate_005fremainder)

これらの手順では、2 つの実数 x と y を受け取ります。ただし、除数 y はゼロ以外でなければなりません。`truncate-quotient` は整数 q を返し、`truncate-remainder` は q を _x/y_ でゼロ方向に丸めた実数 r を返します。また、_x = q\*y + r_ となります。`truncate/` は q と r の両方を返し、それぞれを個別に計算するよりも効率的です。なお、r がゼロ以外の場合は、x と同じ符号になります。

xとyが整数の場合、これらの演算子はR5RSの整数専用演算子`quotient`と`remainder`と同等です。

([truncate-quotient](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-truncate_002dquotient) 123 10) ⇒ 12
([truncate-remainder](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-truncate_002dremainder) 123 10) ⇒ 3
([truncate/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-truncate_002f) 123 10) ⇒ 12 と 3
([truncate/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-truncate_002f) 123 \-10) ⇒ \-12 と 3
([truncate/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-truncate_002f) \-123 10) ⇒ \-12 と \-3
([truncate/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-truncate_002f) \-123 \-10) ⇒ 12 と \-3
([truncate/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-truncate_002f) \-123.2 \-63.5) ⇒ 1.0 および \-59.7
([truncate/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-truncate_002f) 16/3 \-10/7) ⇒ \-3 および 22/21

Scheme Procedure: **centered/** `xy` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-centered_002f)

スキーム手順: **centered-quotient** `xy` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-centered_002dquotient)

スキーム手順: **centered-remainder** `xy` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-centered_002dremainder)

C 関数: `void` **scm\_centered\_divide** `(SCM x, SCM y, SCM *q, SCM *r)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fcentered_005fdivide)

C 関数: `SCM` **scm\_centered\_quotient** `(SCM x, SCM y)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- scm_005fcentered_005fquotient)

C 関数: `SCM` **scm\_centered\_remainder** `(SCM x, SCM y)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fcentered_005fremainder)

これらの手順では、2 つの実数 x と y を受け取ります。ただし、除数 y はゼロ以外でなければなりません。`centered-quotient` は整数 q を返し、`centered-remainder` は _x = q\*y + r_ かつ _\-|y/2| <= r < |y/2|_ を満たす実数 r を返します。`centered/` は q と r の両方を返し、それぞれを個別に計算するよりも効率的です。

`centered-quotient` は、_x/y_ を最も近い整数に丸めた値を返すことに注意してください。_x/y_ が 2 つの整数のちょうど中間にある場合、y の符号に従って同点が解消されます。_y > 0_ の場合、同点は正の無限大に丸められ、そうでない場合は負の無限大に丸められます。これは、_\-|y/2| <= r < |y/2|_ という要件の結果です。

これらの演算子は、R6RSの演算子`div0`、`mod0`、および`div0-and-mod0`と同等であることに注意してください。

([centered-quotient](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-centered_002dquotient) 123 10) ⇒ 12
([centered-remainder](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-centered_002dremainder) 123 10) ⇒ 3
([centered/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-centered_002f) 123 10) ⇒ 12 と 3
([centered/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-centered_002f) 123 \-10) ⇒ \-12 と 3
([centered/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-centered_002f) \-123 10) ⇒ \-12 と \-3
([centered/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-centered_002f) \-123 \-10) ⇒ 12 と \-3
([centered/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-centered_002f) 125 10) ⇒ 13 と \-5
([centered/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-centered_002f) 127 10) ⇒ 13 と \-3
([centered/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-centered_002f) 135 10) ⇒ 14 と \-5
([centered/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-centered_002f) \-123.2 \-63.5) ⇒ 2.0 および 3.8
([centered/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-centered_002f) 16/3 \-10/7) ⇒ \-4 および \-8/21

スキーム手順: **round/** `xy` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-round_002f)

スキーム手順: **round-quotient** `xy` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-round_002dquotient)

スキーム手順: **round-remainder** `xy` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-round_002dremainder)

C 関数: `void` **scm\_round\_divide** `(SCM x, SCM y, SCM *q, SCM *r)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fround_005fdivide)

C 関数: `SCM` **scm\_round\_quotient** `(x, y)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fround_005fquotient)

C 関数: `SCM` **scm\_round\_remainder** `(x, y)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fround_005fremainder)

これらの手順では、2 つの実数 x と y を受け取ります。ただし、除数 y はゼロ以外でなければなりません。`round-quotient` は整数 q を返し、`round-remainder` は実数 r を返します。ただし、_x = q\*y + r_ であり、q は _x/y_ を最も近い整数に丸めた値で、同数の場合は最も近い偶数に丸められます。`round/` は q と r の両方を返し、それぞれを個別に計算するよりも効率的です。

`round/` と `centered/` はほぼ同等ですが、_x/y_ が 2 つの整数のちょうど中間にある場合の動作が異なります。この場合、`round/` は最も近い偶数を選択しますが、`centered/` は _\-|y/2| <= r < |y/2|_ という制約を満たすように選択します。この制約は、`round/` の対応する制約 _\-|y/2| <= r <= |y/2|_ よりも強い制約です。特に、x と y が整数の場合、`centered/` が返す可能性のある剰余の数は _|y|_ ですが、y が偶数の場合、`round/` が返す可能性のある剰余の数は _|y|+1_ です。

([round-quotient](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-round_002dquotient) 123 10) ⇒ 12
([round-remainder](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-round_002dremainder) 123 10) ⇒ 3
([round/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-round_002f) 123 10) ⇒ 12 と 3
([round/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-round_002f) 123 \-10) ⇒ \-12 と 3
([round/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-round_002f) \-123 10) ⇒ \-12 と \-3
([round/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-round_002f) \-123 \-10) ⇒ 12 と \-3
([round/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-round_002f) 125 10) ⇒ 12 と 5
([round/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-round_002f) 127 10) ⇒ 13 と \-3
([round/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-round_002f) 135 10) ⇒ 14 と \-5
([round/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-round_002f) \-123.2 \-63.5) ⇒ 2.0 および 3.8
([round/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-round_002f) 16/3 \-10/7) ⇒ \-4 および \-8/21

* * *

次へ: [ビット演算](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bitwise-Operations)、前: [算術関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Arithmetic)、上: [数値データ型](https://doc.guix.gnu.org/guile/latest/en/guile.html#Numbers) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.2.12 科学関数 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scientific-Functions)

以下の手順では、複素数を含むあらゆる種類の数値を引数として受け入れます。

Scheme Procedure: **sqrt** z [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sqrt)

z の平方根を返します。2 つの可能な根 (正と負) のうち、実部が正のものが返されます。実部がゼロの場合は、虚部が正のものが返されます。したがって、

(√9.0) ⇒ 3.0
(sqrt -9.0) ⇒ 0.0+3.0i
(sqrt 1.0+1.0i) ⇒ 1.09868411346781+0.455089860562227i
(sqrt -1.0-1.0i) ⇒ 0.455089860562227-1.09868411346781i

スキーム手順: **expt** z1 z2 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-expt)

z1をz2乗した値を返します。

スキーム手順: **sin** z [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sin)

zの正弦を返します。

スキーム手順: **cos** z [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-cos)

zのコサインを返します。

スキーム手順: **tan** z [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-tan)

zの正接を返します。

スキーム手順: **asin** z [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-asin)

zの逆正弦を返します。

スキーム手順: **acos** z [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-acos)

zの逆余弦を返します。

スキーム手順: **atan** z [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-atan)

スキーム手順: **atan** yx [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-atan-1)

zの逆正接、または_y/x_の逆正接を返します。

Scheme Procedure: **exp** z [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-exp)

e を z 乗に戻します。ここで e は自然対数の底です (2.71828…)。

スキーム手順: **log** z [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-log)

zの自然対数を返します。

スキーム手順: **log10** z [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-log10)

zの常用対数（底10）を返します。

スキーム手順: **sinh** z [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sinh)

zの双曲線正弦を返します。

Scheme Procedure: **cosh** z [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-cosh)

zの双曲線余弦を返します。

Scheme Procedure: **tanh** z [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-tanh)

zの双曲線正接を返します。

スキーム手順: **asinh** z [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-asinh)

zの双曲線アークサインを返します。

スキーム手順: **acosh** z [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-acosh)

zの双曲線逆余弦を返します。

Scheme Procedure: **atanh** z [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-atanh)

zの双曲線逆正接を返します。

* * *

次へ: [乱数生成](https://doc.guix.gnu.org/guile/latest/en/guile.html#Random)、前: [科学関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scientific)、上: [数値データ型](https://doc.guix.gnu.org/guile/latest/en/guile.html#Numbers) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.2.13 ビット演算 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bitwise-Operations-1)

以下のビット関数では、負の数は無限精度の2の補数として扱われます。例えば、_\-6_ はビット _...111010_ であり、左側に無限個の1があります。このようなビットパターンに6（2進数110）を加えると、すべて0になることがわかります。

スキーム手順: **logand** n1 n2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-logand)

C 関数: **scm\_logand** (n1, n2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005flogand)

整数引数のビットごとのAND演算結果を返します。

([logand](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-logand)) ⇒ \-1
([logand](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-logand) 7) ⇒ 7
([logand](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-logand) #b111 #b011 #b001) ⇒ 1

スキーム手順: **logior** n1 n2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-logior)

C 関数: **scm\_logior** (n1, n2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005flogior)

整数引数のビットごとのOR演算結果を返します。

([logior](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-logior)) ⇒ 0
([logior](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-logior) 7) ⇒ 7
([logior](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-logior) #b000 #b001 #b011) ⇒ 3

スキーム手順: **logxor** n1 n2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-logxor)

C 関数: **scm\_loxor** (n1, n2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005floxor)

整数引数のビットごとのXOR演算結果を返します。結果において、奇数個の引数で特定のビットが設定されている場合、そのビットは設定されます。

([logxor](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-logxor)) ⇒ 0
([logxor](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-logxor) 7) ⇒ 7
([logxor](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-logxor) #b000 #b001 #b011) ⇒ 2
([logxor](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-logxor) #b000 #b001 #b011 #b011) ⇒ 1

スキーム手順: **lognot** n [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-lognot)

C 関数: **scm\_lognot** (n) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005flognot)

整数引数の1の補数である整数を返します。つまり、各0ビットは1に、各1ビットは0に変更されます。

([number->string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-number_002d_003estring) ([lognot](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-lognot) #b10000000) 2)
⇒ "-10000001"
([number->string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-number_002d_003estring) ([lognot](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-lognot) #b0) 2)
⇒ "-1"

スキーム手順: **logtest** jk [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-logtest)

C 関数: **scm\_logtest** (j, k) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005flogtest)

jとkに共通する1ビットがあるかどうかをテストします。これは`(not (zero? (logand jk)))`と同等ですが、`logand`を実際に計算することなく、非ゼロかどうかをテストするだけです。

([logtest](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-logtest) #b0100 #b1011) ⇒ #f
([logtest](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-logtest) #b0100 #b0111) ⇒ #t

スキーム手順: **logbit?** インデックス j [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-logbit_003f)

C 関数: **scm\_logbit\_p** (index, j) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005flogbit_005fp)

j のビット番号インデックスがセットされているかどうかをテストします。インデックスは最下位ビットの場合、0 から始まります。

([logbit?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-logbit_003f) 0 #b1101) ⇒ #t
([logbit?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-logbit_003f) 1 #b1101) ⇒ #f
([logbit?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-logbit_003f) 2 #b1101) ⇒ #t
([logbit?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-logbit_003f) 3 #b1101) ⇒ #t
([logbit?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-logbit_003f) 4 #b1101) ⇒ #f

スキーム手順: **ash** n count [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ash)

C 関数: **scm\_ash** (n, count) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fash)

_floor(n \* 2^{count})_ を返します。n と count は正確な整数である必要があります。

nを無限精度の2の補数表現の整数とみなした場合、`ash`は、countが正の場合は左シフトでゼロビットを追加し、countが負の場合は右シフトでビットを削除することを意味します。これは「算術」シフトです。

([number->string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-number_002d_003estring) ([ash](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ash) #b1 3) 2) ⇒ "1000"
([number->string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-number_002d_003estring) ([ash](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ash) #b1010 \-1) 2) ⇒ "101"

;; -23 はビット ...11101001、-6 はビット ...111010
([ash](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ash) \-23 \-2) ⇒ \-6

スキーム手順: **round-ash** n カウント [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-round_002dash)

C 関数: **scm\_round\_ash** (n, count) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fround_005fash)

_round(n \* 2^count)_ を返します。n と count は正確な整数である必要があります。

nを無限精度の2の補数表現の整数とみなした場合、`round-ash`とは、countが正の場合はゼロビットを導入する左シフト、countが負の場合は最も近い整数に丸める右シフト（同点の場合は最も近い偶数に丸める）を意味します。これは丸められた「算術」シフトです。

([number->string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-number_002d_003estring) ([round-ash](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-round_002dash) #b1 3) 2) ⇒ \\"1000\\"
([number->string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-number_002d_003estring) ([round-ash](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-round_002dash) #b1010 \-1) 2) ⇒ \\"101\\"
([number->string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-number_002d_003estring) ([round-ash](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-round_002dash) #b1010 \-2) 2) ⇒ \\"10\\"
([number->string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-number_002d_003estring) ([round-ash](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-round_002dash) #b1011 \-2) 2) ⇒ \\"11\\"
([number->string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-number_002d_003estring) ([round-ash](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-round_002dash) #b1101 \-2) 2) ⇒ \\"11\\"
([number->string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-number_002d_003estring) ([round-ash](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-round_002dash) #b1110 \-2) 2) ⇒ \\"100\\"

スキーム手順: **logcount** n [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-logcount)

C 関数: **scm\_logcount** (n) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005flogcount)

整数nのビット数を返します。nが正の場合は、2進数表現における1ビットをカウントします。nが負の場合は、2の補数表現における0ビットをカウントします。nが0の場合は、0を返します。

([logcount](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-logcount) #b10101010)
⇒ 4
([logcount](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-logcount) 0)
⇒ 0
([logcount](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-logcount) \-2)
⇒ 1

Scheme Procedure: **integer-length** n [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-integer_002dlength)

C 関数: **scm\_integer\_length** (n) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005finteger_005flength)

nを表すのに必要なビット数を返します。

正のnの場合、これは最上位1ビットまでのビット数です。負のnの場合、これは2の補数表現における最上位0ビットまでのビット数です。

([integer-length](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-integer_002dlength) #b10101010) ⇒ 8
([整数の長さ](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-integer_002dlength) #b1111) ⇒ 4
([整数の長さ](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-integer_002dlength) 0) ⇒ 0
([integer-length](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-integer_002dlength) \-1) ⇒ 0
([integer-length](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-integer_002dlength) \-256) ⇒ 8
([integer-length](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-integer_002dlength) \-257) ⇒ 9

スキームプロシージャ: **integer-expt** nk [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-integer_002dexpt)

C 関数: **scm\_integer\_expt** (n, k) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005finteger_005fexpt)

nをk乗した値を返します。kは整数でなければならず、nは任意の数です。

負の k もサポートされており、通常の方法で _1/n^abs(k)_ となります。_n^0_ は通常どおり 1 であり、これには _0^0_ が 1 であることも含まれます。

([integer-expt](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-integer_002dexpt) 2 5) ⇒ 32
([integer-expt](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-integer_002dexpt) \-3 3) ⇒ \-27
([integer-expt](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-integer_002dexpt) 5 \-3) ⇒ 1/125
([integer-expt](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-integer_002dexpt) 0 0) ⇒ 1

スキーム手順: **bit-extract** n 開始 終了 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bit_002dextract)

C 関数: **scm\_bit\_extract** (n, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbit_005fextract)

n の開始ビット（含む）から終了ビット（含まない）までの整数を返します。開始ビットは結果の 0 番目のビットになります。

([number->string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-number_002d_003estring) ([bit-extract](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bit_002dextract) #b1101101010 0 4) 2)
⇒「1010」
([number->string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-number_002d_003estring) ([bit-extract](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bit_002dextract) #b1101101010 4 9) 2)
⇒「10110」

* * *

前へ: [ビット演算](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bitwise-Operations)、上へ: [数値データ型](https://doc.guix.gnu.org/guile/latest/en/guile.html#Numbers) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.2.14 乱数生成 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Random-Number-Generation)

擬似乱数は、`seed->random-state` または `datum->random-state` で作成できる乱数状態オブジェクトから生成されます。乱数状態オブジェクトの外部表現（つまり、`write` で書き込み、`read` で読み取れるもの）は、`random-state->datum` で取得できます。以下の各種関数の state パラメータはオプションで、デフォルトでは `*random-state*` 変数に格納されている状態オブジェクトが使用されます。

スキーム手順: **copy-random-state** \[state\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-copy_002drandom_002dstate)

C 関数: **scm\_copy\_random\_state** (state) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fcopy_005frandom_005fstate)

ランダムな状態のコピーを返します。

スキーム手順: **random** n \[state\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-random)

C 関数: **scm\_random** (n, state) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005frandom)

[0, n) の範囲の数値を返します。

正の整数または実数 n を受け取り、0 (含む) から n (含まない) までの範囲の同じ型の数値を返します。返される値は一様分布に従います。

スキーム手順: **random:exp** \[state\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-random_003aexp)

C 関数: **scm\_random\_exp** (状態) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005frandom_005fexp)

平均が 1 の指数分布に従う不正確な実数を返します。平均が u の指数分布の場合は、`(* u (random:exp))` を使用してください。

スキーム手順: **random:hollow-sphere!** vect \[state\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-random_003ahollow_002dsphere_0021)

C 関数: **scm\_random\_hollow\_sphere\_x** (vect, state) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005frandom_005fhollow_005fsphere_005fx)

不正確な実乱数でベクトルを埋めます。その二乗の合計は 1.0 になります。ベクトルを次元 n _\=_ `(ベクトル長ベクトル)` の空間の座標と考えると、座標は単位 n 球面の表面に均一に分布します。

スキーム手順: **random:normal** \[state\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-random_003anormal)

C 関数: **scm\_random\_normal** (状態) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005frandom_005fnormal)

正規分布に従う不正確な実数を返します。使用される分布は平均0、標準偏差1です。平均m、標準偏差dの正規分布の場合は、`(+ m (* d (random:normal)))` を使用してください。

スキーム手順: **random:normal-vector!** vect \[state\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-random_003anormal_002dvector_0021)

C 関数: **scm\_random\_normal\_vector\_x** (vect, state) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005frandom_005fnormal_005fvector_005fx)

ベクトルを、独立で標準正規分布（平均0、分散1）に従う不正確な実乱数で埋めます。

スキーム手順: **random:solid-sphere!** vect \[state\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-random_003asolid_002dsphere_0021)

C 関数: **scm\_random\_solid\_sphere\_x** (vect, state) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005frandom_005fsolid_005fsphere_005fx)

2乗の合計が 1.0 未満となる不正確な実乱数で vect を埋めます。vect を次元 n _\=_ `(vector-length vect)` の空間の座標と考えると、座標は単位 n\-球面内に均一に分布します。

スキーム手順: **random:uniform** \[state\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-random_003auniform)

C 関数: **scm\_random\_uniform** (状態) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005frandom_005funiform)

[0,1) の範囲で一様分布する不正確な実数乱数を返します。

スキーム手順: **シード→ランダム状態** シード [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-seed_002d_003erandom_002dstate)

C 関数: **scm\_seed\_to\_random\_state** (seed) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fseed_005fto_005frandom_005fstate)

シード値を使用して新しい乱数状態を返します。

スキーム手順: **datum->random-state** データ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-datum_002d_003erandom_002dstate)

C 関数: **scm\_datum\_to\_random\_state** (データ) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fdatum_005fto_005frandom_005fstate)

`random-state->datum`で取得したデータから、新しいランダムな状態を返します。

スキーム手順: **random-state->datum** 状態 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-random_002dstate_002d_003edatum)

C 関数: **scm\_random\_state\_to\_datum** (state) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005frandom_005fstate_005fto_005fdatum)

Schemeリーダーで書き出し、読み戻すことができる状態のデータ表現を返します。

スキーム手順: **random-state-from-platform** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-random_002dstate_002dfrom_002dplatform)

C 関数: **scm\_random\_state\_from\_platform** () [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005frandom_005fstate_005ffrom_005fplatform)

セキュリティが重要でないアプリケーションでの使用に適した、プラットフォーム固有のエントロピー源からシードされた新しい乱数状態を構築します。現在、最初に /dev/urandom が試行されます。それでも解決しない場合は、時刻、日付、プロセス ID、新しく割り当てられたヒープ セルのアドレス、ローカル スタック フレームのアドレス、および利用可能な場合は高解像度タイマーに基づいてシードが生成されます。

変数: **\*random-state\*** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002arandom_002dstate_002a)

上記の関数が状態パラメータが指定されていない場合に使用するグローバルな乱数状態。

`*random-state*` の初期値は、Guile の起動時に毎回同じであることに注意してください。したがって、上記の手順に状態パラメータを渡さず、`*random-state*` を `(seed->random-state your-seed)` に設定しない場合（ここで `your-seed` は毎回同じではない値です）、実行のたびに同じ「乱数」のシーケンスが生成されます。

例えば、関連するソースコードが変更されていない限り、Guileの起動後初めて乱数を使用する際に`(map random (cdr (iota 30)))`を実行すると、常に以下の結果が得られます。

(map [random](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-random) ([cdr](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-cdr) ([iota](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-iota) 19)))
⇒
(0 1 1 2 2 2 1 2 6 7 10 0 5 3 12 5 5 12)

セキュリティ上重要度の低いアプリケーションで乱数状態を適切にシードするには、プログラムの初期化時に以下を実行します。

([set!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-set_0021) [\*random-state\*](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002arandom_002dstate_002a) ([random-state-from-platform](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-random_002dstate_002dfrom_002dplatform)))

* * *

次へ: [文字セット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Character-Sets)、前: [数値データ型](https://doc.guix.gnu.org/guile/latest/en/guile.html#Numbers)、上: [データ型](https://doc.guix.gnu.org/guile/latest/en/guile.html#Data-Types) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
