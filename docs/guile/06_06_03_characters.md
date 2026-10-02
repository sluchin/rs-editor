#### 6.6.3 文字 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Characters-1)

Schemeには、単一の文字を表すデータ型があります。

文字が一体何であるかを正確に定義するのは、見た目以上に複雑な場合があります。GuileはR6RSのアドバイスに従い、Unicode標準を使用して文字の定義を定めています。つまり、Guileにとって文字とは、Unicode文字データベースに含まれるすべてのものを指します。

Unicode文字データベースは、基本的に「コードポイント」と呼ばれる整数を使用してインデックス付けされた文字のテーブルです。有効なコードポイントは、0から`#xD7FF`までの範囲、または`#xE000`から`#x10FFFF`までの範囲で、約110万個のコードポイントがあります。

文字に割り当てられた、あるいはUnicodeによって何らかの意味が与えられたコードポイントは、「指定コードポイント」と呼ばれます。約20万個ある指定コードポイントのほとんどは、他の文字、記号、空白、制御文字を修飾する文字、アクセント記号、その他の結合記号を示します。中には文字ではなく、隣接する文字の書式設定や表示方法を示す指示記号もあります。

コードポイントが指定コードポイントでない場合、つまりUnicode規格によって文字に割り当てられていない場合は、「予約コードポイント」と呼ばれます。これは、将来の使用のために予約されていることを意味します。コードポイントの大部分、約80万個は「予約コードポイント」です。

慣例として、Unicodeコードポイントは「U+XXXX」と表記されます。ここで「XXXX」は16進数です。ただし、この表記法は有効なコードではありません。Guileは「U+XXXX」を文字として解釈しません。

Schemeでは、文字リテラルは`#\name`と記述します。ここでnameは、使用したい文字の名前です。印刷可能な文字は、通常の1文字の名前を持ちます。たとえば、`#\a`は小文字の`a`です。

コードポイントの中には、「結合文字」と呼ばれるものがあり、これらは単独で印刷されることを意図したものではなく、前の文字の外観を変更することを目的としています。結合文字の場合、文字リテラルの別の形式として、`#\` の後に U+25CC (小さな点線の円) が続き、その後に結合文字が続きます。これにより、結合文字は `#\` のバックスラッシュではなく、円の上に描画されます。

空白文字や制御文字など、印刷されない文字の多くにも名前が付けられています。

最も一般的に使用される非印刷文字は、以下の表に示すように、長い文字名を持っています。

キャラクター名

コードポイント

`#\nul`

U+0000

`#\alarm`

U+0007

`#\backspace`

U+0008

`#\tab`

U+0009

`#\linefeed`

U+000A

`#\newline`

U+000A

`#\vtab`

U+000B

`#\page`

U+000C

`#\return`

U+000D

`#\esc`

U+001B

`#\space`

U+0020

`#\delete`

U+007F

「C0制御文字」（コードポイントが32未満の文字）には、それぞれ略称があります。以下の表に、各文字の略称を示します。

0 = `#\nul`

1 = `#\soh`

2 = `#\stx`

3 = `#\etx`

4 = `#\eot`

5 = `#\enq`

6 = `#\ack`

7 = `#\bel`

8 = `#\bs`

9 = `#\ht`

10 = `#\lf`

11 = `#\vt`

12 = `#\ff`

13 = `#\cr`

14 = `#\so`

15 = `#\si`

16 = `#\dle`

17 = `#\dc1`

18 = `#\dc2`

19 = `#\dc3`

20 = `#\dc4`

21 = `#\nak`

22 = `#\syn`

23 = `#\etb`

24 = `#\can`

25 = `#\em`

26 = `#\sub`

27 = `#\esc`

28 = `#\fs`

29 = `#\gs`

30 = `#\rs`

31 = `#\us`

32 = `#\sp`

「削除」文字（コードポイントU+007F）の略称は`#\del`です。

R7RSにおける「エスケープ」文字（コードポイントU+001B）の名前は`#\escape`です。

Guileとの互換性を保つために、いくつかの代替名も残されています。

代替

標準

`#\nl`

`#\newline`

`#\np`

`#\page`

`#\null`

`#\nul`

文字は、コードポイント値を使用して記述することもできます。例えば、`#\bs` の場合は `#\10`、`#\del` の場合は `#\177` のように、8 進数で記述できます。

8進数よりも16進数を好む場合は、文字エスケープのための追加の構文があります。`#\xHHHH` – 文字「x」の後に1～8桁の16進数が続きます。

Scheme手順: **char?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_003f)

C 関数: **scm\_char\_p** (x) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fp)

xが文字の場合は`#t`を返し、そうでない場合は`#f`を返します。

基本的に、以下の文字比較演算は、文字のコードポイントの数値比較です。

Scheme手順: **char=?** xy [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_003d_003f)

xのコードポイントがyのコードポイントと等しい場合は`#t`を返し、そうでない場合は`#f`を返します。

Scheme手順: **char<?** xy [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_003c_003f)

xのコードポイントがyのコードポイントより小さい場合は`#t`を返し、そうでない場合は`#f`を返します。

Scheme手順: **char<=?** xy [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_003c_003d_003f)

xのコードポイントがyのコードポイント以下の場合、`#t`を返し、そうでない場合は`#f`を返します。

Scheme手順: **char>?** xy [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_003e_003f)

xのコードポイントがyのコードポイントより大きい場合は`#t`を返し、そうでない場合は`#f`を返します。

Scheme手順: **char>=?** xy [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_003e_003d_003f)

xのコードポイントがyのコードポイント以上であれば`#t`を返し、そうでなければ`#f`を返す。

大文字小文字を区別しない文字比較では、Unicode の大文字小文字変換が使用されます。大文字小文字変換による比較では、小文字の文字が、1 文字で表現できる大文字の形を持つ場合、比較前に大文字に変換されます。その他の文字は、比較前に変換されません。これには、ドイツ語のシャープ S (エスツェット) も含まれます。この文字は、大文字の形が 2 文字であるため、変換前に大文字に変換されません。Unicode の大文字小文字変換は言語に依存しません。一般的に正しい規則を使用しますが、すべての言語のすべてのケースを網羅できるわけではありません。

Scheme手順: **char-ci=?** xy [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dci_003d_003f)

x の大文字小文字を区別したコードポイントが y の大文字小文字を区別したコードポイントと同じ場合は `#t` を返し、そうでない場合は `#f` を返します。

Scheme手順: **char-ci<?** xy [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dci_003c_003f)

x の大文字小文字を区別したコードポイントが y の大文字小文字を区別したコードポイントより小さい場合は `#t` を返し、そうでない場合は `#f` を返します。

Scheme手順: **char-ci<=?** xy [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dci_003c_003d_003f)

x の大文字小文字を区別したコードポイントが y の大文字小文字を区別したコードポイント以下の場合、`#t` を返し、そうでない場合は `#f` を返します。

Scheme手順: **char-ci>?** xy [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dci_003e_003f)

x の大文字小文字を区別したコードポイントが y の大文字小文字を区別したコードポイントより大きい場合は `#t` を返し、そうでない場合は `#f` を返します。

Scheme手順: **char-ci>=?** xy [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dci_003e_003d_003f)

x の大文字小文字を区別したコードポイントが y の大文字小文字を区別したコードポイント以上であれば `#t` を返し、そうでなければ `#f` を返します。

スキーム手順: **char-alphabetic?** chr [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dalphabetic_003f)

C 関数: **scm\_char\_alphabetic\_p** (chr) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005falphabetic_005fp)

文字がアルファベットの場合は「#t」を返し、そうでない場合は「#f」を返します。

Scheme Procedure: **char-numeric?** chr [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dnumeric_003f)

C 関数: **scm\_char\_numeric\_p** (chr) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fnumeric_005fp)

文字が数値の場合は「#t」を返し、そうでない場合は「#f」を返します。

Scheme 手順: **char-whitespace?** chr [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dwhitespace_003f)

C 関数: **scm\_char\_whitespace\_p** (chr) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fwhitespace_005fp)

文字が空白の場合は「#t」を返し、そうでない場合は「#f」を返します。

Scheme 手順: **char-upper-case?** chr [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dupper_002dcase_003f)

C 関数: **scm\_char\_upper\_case\_p** (chr) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fupper_005fcase_005fp)

文字が大文字の場合は「#t」を返し、そうでない場合は「#f」を返します。

Scheme 手順: **char-lower-case?** chr [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dlower_002dcase_003f)

C 関数: **scm\_char\_lower\_case\_p** (chr) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005flower_005fcase_005fp )

文字が小文字の場合は「#t」を返し、そうでない場合は「#f」を返します。

Scheme 手順: **char-is-both?** chr [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dis_002dboth_003f)

C 関数: **scm\_char\_is\_both\_p** (chr) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fis_005fboth_005fp)

文字が大文字または小文字の場合は「#t」を返し、そうでない場合は「#f」を返します。

Scheme Procedure: **char-general-category** chr [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dgeneral_002dcategory)

C 関数: **scm\_char\_general\_category** (chr) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fgeneral_005fcategory)

chr に割り当てられている Unicode 一般カテゴリの 2 文字の名前を表す記号を返します。名前付きカテゴリが割り当てられていない場合は `#f` を返します。以下の表は、カテゴリ名とその意味の一覧です。

ル

大文字

Pf

最後の引用句読点

Ll

小文字

ポ

その他の句読点

中尉

タイトルケースの文字

スモール

数学記号

Lm

修飾文字

Sc

通貨記号

ロ

その他の手紙

Sk

修飾記号

マンガン

非間隔マーク

それで

その他のシンボル

Mc

結合間隔マーク

Zs

スペースセパレーター

自分

囲みマーク

Zl

区切り線

Nd

10進数

Zp

段落区切り

オランダ

文字番号

cc

コントロール

いいえ

その他の番号

参照

形式

PC

接続詞句読点

Cs

代理母

Pd

ダッシュ句読点

株式会社

私的使用

追伸

句読点を開く

中国

未割り当て

ペ

句読点を閉じる

円周率

引用符の先頭の句読点

スキーム手順: **char->integer** chr [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002d_003einteger)

C 関数: **scm\_char\_to\_integer** (chr) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fto_005finteger)

chrのコードポイントを返します。

スキーム手順: **integer->char** n [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-integer_002d_003echar)

C 関数: **scm\_integer\_to\_char** (n) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005finteger_005fto_005fchar)

コードポイントnを持つ文字を返します。整数nは有効なコードポイントである必要があります。有効なコードポイントは、0から`#xD7FF`までの範囲、または`#xE000`から`#x10FFFF`までの範囲です。

Scheme手順: **char-upcase** chr [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dupcase)

C 関数: **scm\_char\_upcase** (chr) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fupcase)

chr の大文字バージョンを返します。

Scheme 手順: **char-downcase** chr [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002ddowncase)

C 関数: **scm\_char\_downcase** (chr) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005fdowncase)

chr の小文字バージョンを返します。

Scheme 手順: **char-titlecase** chr [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dtitlecase)

C 関数: **scm\_char\_titlecase** (chr) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005ftitlecase)

chr のタイトルケース文字バージョンが存在する場合はそれを返し、存在しない場合は大文字バージョンを返します。

ほとんどの文字ではこれらは同じですが、Unicode 標準には、`U+01F3` “dz” のような特定の二重音字互換性文字が含まれており、その大文字とタイトルケースの文字は異なります (この場合はそれぞれ `U+01F1` “DZ” と `U+01F2` “Dz” です)。

C 関数: `scm_t_wchar` **scm\_c\_upcase** `(scm_t_wchar c)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fupcase)

C 関数: `scm_t_wchar` **scm\_c\_downcase** `(scm_t_wchar c)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fdowncase)

C 関数: `scm_t_wchar` **scm\_c\_titlecase** `(scm_t_wchar c)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005ftitlecase)

これらのC関数は、Unicodeコードポイントの整数表現を受け取り、それぞれ大文字、小文字、タイトルケースに対応するコードポイントを返します。型`scm_t_wchar`は、符号付き32ビット整数です。

文字にはUnicodeで定義された「正式名称」もあります。これらの名称は、Guileでは`(ice-9 unicode)`モジュールからアクセスできます。

(use-modules (ice-9 unicode))

Scheme手順: **char->formal-name** chr [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002d_003eformal_002dname)

文字「ch」の正式なすべて大文字のUnicode名を文字列として返します。文字に名前がない場合は「#f」を返します。

Scheme手順: **formal-name->char** name [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-formal_002dname_002d_003echar)

正式なUnicode名がすべて大文字でnameである文字を返します。そのような文字が見つからない場合は`#f`を返します。

* * *

次へ: [文字列](https://doc.guix.gnu.org/guile/latest/en/guile.html#Strings)、前: [文字](https://doc.guix.gnu.org/guile/latest/en/guile.html#Characters)、上: [データ型](https://doc.guix.gnu.org/guile/latest/en/guile.html#Data-Types) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
