# 6.6.6 シンボル

> **原文**: [Guile Reference Manual - Symbols](https://www.gnu.org/software/guile/manual/html_node/Symbols.html)
>
> このドキュメントは GNU Free Documentation License の下で公開されている原文の翻訳です。

Scheme のシンボルは、3つの方法で広く使われています。離散的なデータの項目として、連想リストやハッシュテーブルの検索キーとして、そして変数の参照を表すためにです。

シンボルは、文字の並びによって定義されるという点で文字列に似ています。その文字の並びはシンボルの**名前**として知られています。通常の場合――つまり、シンボルの名前に Scheme の構文の他の要素と混同される可能性のある文字が含まれていない場合――シンボルは Scheme プログラムの中で、引用符やその他の特別な構文を付けずに、名前を構成する文字の並びを書くことで書かれます。たとえば、名前が「multiply-by-2」であるシンボルは、単に次のように書かれます。

```scheme
multiply-by-2
```

これが、内容が「multiply-by-2」である文字列とどのように異なるかに注目してください。文字列は次のように二重引用符を付けて書かれます。

```scheme
"multiply-by-2"
```

書き方を超えて見ると、シンボルは2つの重要な点で文字列と異なります。

最初の重要な違いは一意性です。同じに見える文字列がプログラムの2つの異なる場所から2回読み込まれると、その結果は、内容がたまたま同じである2つの異なる文字列オブジェクトになります。一方、同じに見えるシンボルがプログラムの2つの異なる場所から2回読み込まれると、その結果は2回とも同じシンボルオブジェクトになります。

読み込まれた2つのシンボルが与えられれば、`eq?` を使ってそれらが同じであるかどうか（つまり同じ名前を持つかどうか）をテストできます。`eq?` は Scheme で最も効率的な比較演算子であり、このように2つのシンボルを比較するのは、たとえば2つの数を比較するのと同じくらい高速です。一方、2つの文字列が与えられた場合、それらの文字列が同じ内容を持つかどうかを判断するには、はるかに遅い比較演算子である `equal?` または `string=?` を使わなければなりません。

```scheme
(define sym1 (quote hello))
(define sym2 (quote hello))
(eq? sym1 sym2) ⇒ #t

(define str1 "hello")
(define str2 "hello")
(eq? str1 str2) ⇒ #f
(equal? str1 str2) ⇒ #t
```

2番目の重要な違いは、シンボルは文字列と異なり、自己評価的ではないということです。上の例で `(quote …)` が必要なのはこのためです。`(quote hello)` は "hello" という名前のシンボル自体に評価されますが、クォートされていない `hello` は "hello" という名前のシンボルとして読み込まれ、変数参照として評価されます…これについては後述します（「変数を表すシンボル」を参照）。

- 離散的なデータとしてのシンボル
- 検索キーとしてのシンボル
- 変数を表すシンボル
- シンボルに関連する操作
- シンボルの拡張読み取り構文
- インターンされていないシンボル

## 6.6.6.1 離散的なデータとしてのシンボル

数とシンボルは、どちらも `eq?` による比較に適しているという点で似ています。しかし、シンボルの名前はそのシンボルが表す概念を直接記述するために使えるので、シンボルは数よりも記述的です。

たとえば、コンピュータプログラムでいくつかの色を表現する必要があると想像してください。数を使うと、数と色の間の何らかの対応付けを恣意的に選び、その対応付けを一貫して使うよう注意しなければなりません。

```scheme
;; 1=red, 2=green, 3=purple

(if (eq? (color-of vehicle) 1)
    ...)
```

定数を定義することで、対応付けをより明示的にし、コードを読みやすくすることができます。

```scheme
(define red 1)
(define green 2)
(define purple 3)

(if (eq? (color-of vehicle) red)
    ...)
```

しかし最も単純で明確なアプローチは、数をまったく使わず、名前が参照する色を指定するシンボルを使うことです。

```scheme
(if (eq? (color-of vehicle) 'red)
    ...)
```

数に対するシンボルの記述的な利点は、記述したい概念の集合が大きくなるにつれて増していきます。車のオブジェクトが、次のようなものを持っているか使っているかなど、他のプロパティも持てるとしましょう。

- オートマチックまたはマニュアルのトランスミッション
- 有鉛または無鉛の燃料
- パワーステアリング（またはそうでない）

すると、車の組み合わされたプロパティの集合は、シンボルのリストとして自然に表現して操作できます。

```scheme
(properties-of vehicle1)
⇒
(red manual unleaded power-steering)

(if (memq 'power-steering (properties-of vehicle1))
    (display "Unfit people can drive this vehicle.\n")
    (display "You'll need strong arms to drive this vehicle!\n"))
⊣
Unfit people can drive this vehicle.
```

ここで頼りにしているシンボルの基本的な性質は、プログラムのある部分に出現する `'red` が、プログラムの別の部分に出現する `'red` と区別できないシンボルであるということを思い出してください。これは、シンボルを `eq?` で有用に比較できることを意味します。同時に、シンボルは自然に記述的な名前を持っています。この効率性と記述力の組み合わせにより、シンボルは離散的なデータとして使うのに理想的です。

## 6.6.6.2 検索キーとしてのシンボル

その効率性と記述力を考えると、連想リストやハッシュテーブルのキーとしてシンボルを使うのは自然なことです。

これを説明するために、前の小節の車のプロパティの例の、より構造化された表現を考えてみましょう。すべてのプロパティを平坦なリストに混ぜ合わせる代わりに、次のような連想リストを使うことができます。

```scheme
(define car1-properties '((color . red)
                          (transmission . manual)
                          (fuel . unleaded)
                          (steering . power-assisted)))
```

この構造が平坦なリストよりもいかに明示的で拡張可能であるかに注目してください。たとえば、`manual` が、たとえば車の窓やロックではなく、トランスミッションを指していることが明確になります。また、さらなるプロパティが、あいまいになることなく、可能な値の中で同じシンボルを使うこともできます。

```scheme
(define car1-properties '((color . red)
                          (transmission . manual)
                          (fuel . unleaded)
                          (steering . power-assisted)
                          (seat-color . red)
                          (locking . manual)))
```

このような表現を使えば、効率的な `assq-XXX` 系の手続き（「連想リスト」を参照）を使って、個々の情報を取り出したり変更したりするのは簡単です。

```scheme
(assq-ref car1-properties 'fuel) ⇒ unleaded
(assq-ref car1-properties 'transmission) ⇒ manual

(assq-set! car1-properties 'seat-color 'black)
⇒
((color . red)
 (transmission . manual)
 (fuel . unleaded)
 (steering . power-assisted)
 (seat-color . black)
 (locking . manual)))
```

ハッシュテーブルにもキーがあり、ハッシュテーブルでのシンボルの使用にも、連想リストの場合とまったく同じ議論が当てはまります。シンボルをキーとするエントリをハッシュテーブルのどこに追加するかを決めるために Guile が使うハッシュ値は、`symbol-hash` 手続きを呼び出すことで得られます。

**Scheme 手続き: `symbol-hash symbol`**<br>**C 関数: `scm_symbol_hash (symbol)`**
: `symbol` のハッシュ値を返します。

ハッシュテーブル一般についての情報と、連想リストではなくハッシュテーブルを使うことを選ぶ理由については、「ハッシュテーブル」を参照してください。

## 6.6.6.3 変数を表すシンボル

Scheme プログラムの中のクォートされていないシンボルが評価されると、それは変数参照として解釈され、評価の結果は適切な変数の値になります。

たとえば、式 `(string-length "abcd")` が読み込まれて評価されると、文字の並び `string-length` は名前が "string-length" であるシンボルとして読み込まれます。このシンボルは、値が文字列の長さの計算を実装する手続きである変数に関連付けられています。したがって、`string-length` シンボルの評価はその手続きになります。

クォートされていないシンボルとそれが参照する変数との結び付きの詳細は、別の場所で説明されています。シンボルと変数の間の関連付けがどのように作成されるかについては「定義と変数の束縛」を、それらの関連付けが Guile のモジュールシステムによってどのように影響を受けるかについては「モジュール」を参照してください。

## 6.6.6.4 シンボルに関連する操作

任意の Scheme 値が与えられたとき、`symbol?` プリミティブを使ってそれがシンボルかどうかを判断できます。

**Scheme 手続き: `symbol? obj`**<br>**C 関数: `scm_symbol_p (obj)`**
: `obj` がシンボルであれば `#t` を、そうでなければ `#f` を返します。

**C 関数: `int scm_is_symbol (SCM val)`**
: `scm_is_true (scm_symbol_p (val))` と同等です。

シンボルを持っていることが分かったら、`symbol->string` を呼び出すことでその名前を文字列として得ることができます。Guile は、大文字と小文字の区別に関して、`symbol->string` の詳細においてデフォルトで R5RS とは異なることに注意してください。

**Scheme 手続き: `symbol->string s`**<br>**C 関数: `scm_symbol_to_string (s)`**
: シンボル `s` の名前を文字列として返します。デフォルトでは、Guile はシンボルを大文字と小文字を区別して読み込むので、返される文字列は、`s` を作成させた文字の並びと同じ大文字小文字の変化を持ちます。

  Guile がシンボルを大文字と小文字を区別せずに読み込むように（R5RS で規定されているとおりに）設定されており、`s` がリテラル式（『The Revised^5 Report on Scheme』の「Literal expressions」を参照）の一部として、または `read` や `string-ci->symbol` 手続きの呼び出しによって生まれた場合、Guile はシンボルオブジェクトを作成する前にシンボルの名前のアルファベット文字を小文字に変換するので、ここで返される文字列は小文字になります。

  `s` が `string->symbol` によって作成された場合、返される文字列の文字の大文字小文字は、`s` が作成された時点での Guile の大文字小文字の区別の設定にかかわらず、`string->symbol` に渡された文字列のものと同じになります。

  この手続きが返す文字列に `string-set!` のような変更手続きを適用するのはエラーです。

ほとんどのシンボルはコードの中にリテラルとして書くことで作成されます。しかし、次の手続きを使ってプログラム的にシンボルを作成することも可能です。

**Scheme 手続き: `symbol char…`**
: 与えられた文字の引数から作られた、新しく割り当てられたシンボルを返します。

  ```scheme
  (symbol #\x #\y #\z) ⇒ xyz
  ```

**Scheme 手続き: `list->symbol lst`**
: 文字のリストから作られた、新しく割り当てられたシンボルを返します。

  ```scheme
  (list->symbol '(#\a #\b #\c)) ⇒ abc
  ```

**Scheme 手続き: `symbol-append arg …`**
: 与えられたシンボル `arg ...` を連結したものを文字とする、新しく割り当てられたシンボルを返します。

  ```scheme
  (let ((h 'hello))
    (symbol-append h 'world))
  ⇒ helloworld
  ```

**Scheme 手続き: `string->symbol string`**<br>**C 関数: `scm_string_to_symbol (string)`**
: 名前が `string` であるシンボルを返します。この手続きは、特殊文字や標準でない大文字小文字の文字を含む名前を持つシンボルを作成できますが、Scheme の一部の実装ではそのようなシンボルはそれ自身として読み込むことができないため、そのようなシンボルを作成するのは通常良い考えではありません。

**Scheme 手続き: `string-ci->symbol str`**<br>**C 関数: `scm_string_ci_to_symbol (str)`**
: 名前が `str` であるシンボルを返します。Guile が現在シンボルを大文字と小文字を区別せずに読み込んでいる場合、返されるシンボルが検索または作成される前に、`str` は小文字に変換されます。

次の例は、シンボルの大文字と小文字の区別に関する Guile の詳細な動作を示しています。

```scheme
(read-enable 'case-insensitive)   ; R5RS compliant behavior

(symbol->string 'flying-fish)    ⇒ "flying-fish"
(symbol->string 'Martin)         ⇒ "martin"
(symbol->string
   (string->symbol "Malvina"))   ⇒ "Malvina"

(eq? 'mISSISSIppi 'mississippi)  ⇒ #t
(string->symbol "mISSISSIppi")   ⇒ mISSISSIppi
(eq? 'bitBlt (string->symbol "bitBlt")) ⇒ #f
(eq? 'LolliPop
  (string->symbol (symbol->string 'LolliPop))) ⇒ #t
(string=? "K. Harper, M.D."
  (symbol->string
    (string->symbol "K. Harper, M.D."))) ⇒ #t

(read-disable 'case-insensitive)   ; Guile default behavior

(symbol->string 'flying-fish)    ⇒ "flying-fish"
(symbol->string 'Martin)         ⇒ "Martin"
(symbol->string
   (string->symbol "Malvina"))   ⇒ "Malvina"

(eq? 'mISSISSIppi 'mississippi)  ⇒ #f
(string->symbol "mISSISSIppi")   ⇒ mISSISSIppi
(eq? 'bitBlt (string->symbol "bitBlt")) ⇒ #t
(eq? 'LolliPop
  (string->symbol (symbol->string 'LolliPop))) ⇒ #t
(string=? "K. Harper, M.D."
  (symbol->string
    (string->symbol "K. Harper, M.D."))) ⇒ #t
```

C からは、現在のロケールのエンコーディングの C 文字列から Scheme のシンボルを構築する、より低レベルの関数があります。

C からそれ以上のことをしたい場合は、`scm_symbol_to_string` と `scm_string_to_symbol` を使ってシンボルと文字列の間で変換し、文字列を操作するべきです。

**C 関数: `SCM scm_from_latin1_symbol (const char *name)`**<br>**C 関数: `SCM scm_from_utf8_symbol (const char *name)`**
: ヌル終端された C の文字列 `name` によって名前が指定される Scheme のシンボルを構築して返します。これらは、C の文字列がソースコードにハードコードされている場合に適しています。

**C 関数: `SCM scm_from_locale_symbol (const char *name)`**<br>**C 関数: `SCM scm_from_locale_symboln (const char *name, size_t len)`**
: `name` によって名前が指定される Scheme のシンボルを構築して返します。`scm_from_locale_symbol` では `name` はヌル終端されていなければなりません。`scm_from_locale_symboln` では、`name` の長さは `len` によって明示的に指定されます。

  現在のロケールが、文字列と文字の定数に使われる実行文字集合のものと一致する保証はないため、`name` が C の文字列定数である場合にはこれらの関数を使うべきではないことに注意してください。最近のほとんどの C コンパイラはデフォルトで UTF-8 を使うので、そのような場合には `scm_from_utf8_symbol` を推奨します。

**C 関数: `SCM scm_take_locale_symbol (char *str)`**<br>**C 関数: `SCM scm_take_locale_symboln (char *str, size_t len)`**
: それぞれ `scm_from_locale_symbol` および `scm_from_locale_symboln` と同様ですが、最終的に `str` を `free` で解放することも行います。したがって、Scheme の文字列を作成した直後にいずれにせよ `str` を解放する場合に、この関数を使うことができます。特定の場合には、Guile は `str` をその内部表現として直接使うことができます。

シンボルのサイズも C から得ることができます。

**C 関数: `size_t scm_c_symbol_length (SCM sym)`**
: `sym` の文字数を返します。

最後に、一部のアプリケーション、特に新しい Scheme コードを動的に生成するものは、生成されたコードで使うためのシンボルを生成する必要があります。`gensym` プリミティブはこの必要を満たします。

**Scheme 手続き: `gensym [prefix]`**<br>**C 関数: `scm_gensym (prefix)`**
: 接頭辞とカウンタの値から構築された名前を持つ新しいシンボルを作成します。文字列 `prefix` はオプション引数として指定できます。デフォルトの接頭辞は「` g`」です。カウンタは呼び出しごとに1ずつ増やされます。カウンタをリセットする手段は用意されていません。

`gensym` によって生成されるシンボルは、その名前が空白で始まるため、おそらく一意です。そうでなければ、そのようなシンボルはプログラマがわざわざそうしない限り生成できないからです。代わりにインターンされていないシンボル（「インターンされていないシンボル」を参照）を使えば一意性を保証できますが、それらは有用な形で書き出して読み戻すことができません。

## 6.6.6.5 シンボルの拡張読み取り構文

シンボルの読み取り構文は、文字、数字、拡張アルファベット文字の並びであり、数を始めることのできない文字で始まります。さらに、特別なケースとして `+`、`-`、`...` は、数が `+`、`-`、`.` で始まることがあるにもかかわらず、シンボルとして読み込まれます。

拡張アルファベット文字は、識別子の中で文字（letter）であるかのように使うことができます。拡張アルファベット文字の集合は次のとおりです。

```
! $ % & * + - . / : < = > ? @ ^ _ ~
```

上で定義された標準の読み取り構文（これは R5RS から取られています。『The Revised^5 Report on Scheme』の「Formal syntax」を参照）に加えて、Guile は、空白文字、改行、括弧などの珍しい文字を含めることができる、拡張されたシンボルの読み取り構文を提供しています。（何らかの理由で）上で言及されていない文字を含むシンボルを書く必要がある場合は、次のようにして書くことができます。

- シンボルを文字 `#{` で始め、
- シンボルの文字を書き、
- シンボルを文字 `}#` で終えます。

この形式の読み取り構文のいくつかの例を示します。最初のシンボルは空白文字を含むため、2番目は改行を含むため、最後は数のように見えるため、拡張構文を使う必要があります。

```scheme
#{foo bar}#

#{what
ever}#

#{4242}#
```

Guile はシンボルのこの拡張読み取り構文を提供していますが、移植性がなく、あまり読みやすくないため、広く使うことは推奨されません。

あるいは、`r7rs-symbols` 読み取りオプション（「Scheme コードの読み込み」を参照）を有効にすると、二重引用符の代わりに縦棒で区切る点を除いて、文字列に使われるのと同じ記法を使って任意のシンボルを書くことができます。

```scheme
|foo bar|
|\x3BB; is a greek lambda|
|\| is a vertical bar|
```

`r7rs-symbols` 表示オプション（「Scheme 値の書き出し」を参照）もあることに注意してください。この記法の使用を有効にするには、次の式の一方または両方を評価します。

```scheme
(read-enable  'r7rs-symbols)
(print-enable 'r7rs-symbols)
```

## 6.6.6.6 インターンされていないシンボル

シンボルを有用なものにしているのは、それらが自動的に一意に保たれることです。異なるオブジェクトでありながら同じ名前を持つ2つのシンボルは存在しません。しかし、もちろん例外のない規則はありません。これまで論じてきた通常のシンボルに加えて、少し異なる振る舞いをする特別な**インターンされていない**（uninterned）シンボルを作成することもできます。

それらの何が異なるのか、そしてなぜ有用なのかを理解するために、通常のシンボルが実際にどのように一意に保たれているかを見てみましょう。

Guile が特定の名前を持つシンボルを見つけたいときはいつでも、たとえば `read` の間や `string->symbol` を実行するときに、まずすべての既存のシンボルの表を調べて、与えられた名前を持つシンボルがすでに存在するかどうかを調べます。存在する場合、Guile は単にそのシンボルを返します。存在しない場合、その名前を持つ新しいシンボルが作成され、後で見つけられるように表に登録されます。

ときには、「新鮮」であることが保証されたシンボル、つまり以前に存在しなかったシンボルを作成したいことがあるかもしれません。また、将来、他の誰もが意図せずにあなたのシンボルに出くわすことがないことを何らかの方法で保証したいかもしれません。シンボルのこれらの性質は、マクロ展開の間にコードを生成するときにしばしば必要になります。新しい一時変数を導入するとき、それらが他の人のコードの変数と衝突しないことを保証したいのです。

これを手配する最も簡単な方法は、新しいシンボルを作成するが、すべてのシンボルのグローバルな表には登録しないことです。そうすれば、誰も偶然にあなたのシンボルにアクセスすることはありません。表にないシンボルは**インターンされていない**と呼ばれます。もちろん、表にあるシンボルは**インターンされている**（interned）と呼ばれます。

新しいインターンされていないシンボルは、関数 `make-symbol` で作成します。シンボルがインターンされているかどうかは、`symbol-interned?` でテストできます。

インターンされていないシンボルは、シンボルの名前がシンボルオブジェクトを一意に識別するという規則を破ります。このため、それらはインターンされたシンボルのように書き出して読み戻すことができません。現在、Guile はインターンされていないシンボルの読み込みをサポートしていません。この理由から、関数 `gensym` はインターンされていないシンボルを返さないことに注意してください。

**Scheme 手続き: `make-symbol name`**<br>**C 関数: `scm_make_symbol (name)`**
: 名前 `name` を持つ新しいインターンされていないシンボルを返します。返されるシンボルは一意であることが保証されており、将来の `string->symbol` の呼び出しがそれを返すことはありません。

**Scheme 手続き: `symbol-interned? symbol`**<br>**C 関数: `scm_symbol_interned_p (symbol)`**
: `symbol` がインターンされていれば `#t` を、そうでなければ `#f` を返します。

例:

```scheme
(define foo-1 (string->symbol "foo"))
(define foo-2 (string->symbol "foo"))
(define foo-3 (make-symbol "foo"))
(define foo-4 (make-symbol "foo"))

(eq? foo-1 foo-2)
⇒ #t
; 同じ名前を持つ2つのインターンされたシンボルは同じオブジェクトだが、

(eq? foo-1 foo-3)
⇒ #f
; 同じ名前で make-symbol を呼び出すと別のオブジェクトが返される。

(eq? foo-3 foo-4)
⇒ #f
; make-symbol の呼び出しは、同じ名前であっても常に新しいオブジェクトを返す。

foo-3
⇒ #<uninterned-symbol foo 8085290>
; インターンされていないシンボルはインターンされたシンボルとは異なる形で表示されるが、

(symbol? foo-3)
⇒ #t
; それでもシンボルであり、

(symbol-interned? foo-3)
⇒ #f
; ただインターンされていないだけである。
```

---

> **ライセンス**: この翻訳は GNU Free Documentation License v1.3 以降に基づいて作成されています。
> 原文の著作権: Copyright (C) 1996-2023 Free Software Foundation, Inc.
