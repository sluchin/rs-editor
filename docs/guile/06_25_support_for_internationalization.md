### 6.25 国際化のサポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Support-for-Internationalization)

Guile は、Scheme プログラムの国際化 [23](https://doc.guix.gnu.org/guile/latest/en/guile.html#FOOT23) を 2 つの方法でサポートします。まず、特定の文化的慣習に準拠した方法 (つまり、「ロケール依存」の方法) でテキストとデータを操作する手順が `(ice-9 i18n)` で提供されます。次に、Guile は GNU `gettext` を使用してプログラム メッセージ ストリングを翻訳することを許可します。

* [Guile による国際化](https://doc.guix.gnu.org/guile/latest/en/guile.html#i18n-Introduction)
* [テキスト照合](https://doc.guix.gnu.org/guile/latest/en/guile.html#Text-Collation)
* [文字の大文字・小文字のマッピング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Character-Case-Mapping)
* [数値の入出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Number-Input-and-Output)
* [ロケール情報へのアクセス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Accessing-Locale-Information)
* [Gettext サポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Gettext-Support)

* * *

次へ: [テキスト照合](https://doc.guix.gnu.org/guile/latest/en/guile.html#Text-Collation)、前: [国際化のサポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Internationalization)、上: [国際化のサポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Internationalization) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.25.1 Guile による国際化 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Internationalization-with-Guile)

以下に説明する機能を利用するには、`(ice-9 i18n)`モジュールを通常の方法でインポートする必要があります。

(use-modules (ice-9 i18n))

`(ice-9 i18n)`モジュールは、ユーザーが選択した文化的慣習に準拠した方法でテキストやその他のデータを操作する手順を提供します。世界の各地域や言語には、例えば実数の表現方法、文字の分類方法、テキストの照合方法など、独自の慣習があります。これらの側面すべてが、その地域や言語のいわゆる「文化的慣習」を構成します。

コンピュータシステムでは、通常、一連の文化的慣習を「ロケール」と呼びます。これらの文化的慣習を構成する各側面ごとに、「ロケールカテゴリ」が定義されます。たとえば、文字の分類方法は「LC_CTYPE」カテゴリで定義され、プログラムメッセージがユーザーに表示される言語は「LC_MESSAGES」カテゴリで定義されます（詳細は「ロケールに関する一般情報」を参照してください）。

このモジュールが提供する手順を使用すると、あらゆるロケール設定に自動的に適応するプログラムを開発できます。後述するように、これらの手順の多くは、オプションで _locale オブジェクト_ 引数を受け取ることができます。この追加引数は、呼び出された手順が従うべきロケール設定を定義します。この引数を省略すると、プロセスの現在のロケール設定が適用されます ( [`setlocale`](https://doc.guix.gnu.org/guile/latest/en/guile.html#Locales) を参照)。

以下の手順により、そのようなロケールオブジェクトを操作することができます。

スキーム手順: **make-locale** category-list locale-name \[base-locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dlocale)

C 関数: **scm\_make\_locale** (category\_list, locale\_name, base\_locale) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fmake_005flocale)

ロケールデータセットのセットを表すデータ構造への参照を返します。locale-name は特定のロケールを示す文字列 (例: `"aa_DJ"`) で、category-list はロケールカテゴリのリスト、または `setlocale` で使用される単一のカテゴリのいずれかである必要があります ([`setlocale`](https://doc.guix.gnu.org/guile/latest/en/guile.html#Locales) を参照)。オプションで `base-locale` が渡された場合は、category-list にリストされていないカテゴリの設定を示すロケールオブジェクトである必要があります。

以下の呼び出しは、メッセージと文字分類にスウェーデン語を使用することと、その他のカテゴリのデフォルト設定（つまり、通常米国で使用されている慣習を表すデフォルトの「C」ロケールの設定）を組み合わせたロケールオブジェクトを作成します。

(make-locale (list LC\_MESSAGES LC\_CTYPE) "sv\_SE")

以下の例は、エスペラント語のメッセージと慣習をクロアチアの通貨慣習と組み合わせたものです。

(make-locale LC\_MONETARY "hr\_HR"
(ロケール LC\_ALL "eo\_EO"))

`make-locale` は、locale-name がシステム上でコンパイルされたロケールのいずれとも一致しない場合に、`system-error` 例外 ([エラーの処理方法](https://doc.guix.gnu.org/guile/latest/en/guile.html#Handling-Errors) を発生させます。なお、GNU 以外のシステムでは、このエラーはロケールオブジェクトが実際に使用される際に発生する場合があります。

スキームプロシージャ: **locale?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locale_003f)

C 関数: **scm\_locale\_p** (obj) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005flocale_005fp)

objがロケールオブジェクトであればtrueを返します。

スキーム変数: **%global-locale** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0025global_002dlocale)

C 変数: **scm\_global\_locale** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fglobal_005flocale)

この変数は、`setlocale()` を使用してインストールされた現在のプロセスロケールを示すロケールオブジェクトにバインドされています（[ロケール](https://doc.guix.gnu.org/guile/latest/en/guile.html#Locales)を参照）。他のロケールオブジェクトと同様に使用でき、たとえば `make-locale` の 3 番目の引数として使用できます。

* * *

次へ: [文字の大文字小文字のマッピング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Character-Case-Mapping)、前: [Guile による国際化](https://doc.guix.gnu.org/guile/latest/en/guile.html#i18n-Introduction)、上: [国際化のサポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Internationalization) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.25.2 テキスト照合 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Text-Collation-1)

以下の手順は、テキスト照合、すなわちロケールに依存した文字列および文字のソートをサポートします。

Scheme 手順: **string-locale<?** s1 s2 \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dlocale_003c_003f)

C 関数: **scm\_string\_locale\_lt** (s1, s2, locale) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005flocale_005flt)

スキーム手順: **string-locale>?** s1 s2 \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dlocale_003e_003f)

C 関数: **scm\_string\_locale\_gt** (s1、s2、ロケール) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005flocale_005fgt)

スキーム手順: **string-locale-ci<?** s1 s2 \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dlocale_002dci_003c_003f)

C 関数: **scm\_string\_locale\_ci\_lt** (s1、s2、ロケール) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005flocale_005fci_005flt)

スキーム手順: **string-locale-ci>?** s1 s2 \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dlocale_002dci_003e_003f)

C 関数: **scm\_string\_locale\_ci\_gt** (s1、s2、ロケール) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005flocale_005fci_005fgt)

文字列s1とs2をロケールに応じて比較します。ロケールが指定されている場合は、`make-locale`によって返されるロケールオブジェクトが比較に使用されます。指定されていない場合は、現在のシステムロケールが使用されます。`-ci`オプションの場合、大文字と小文字を区別せずに比較が行われます。

スキーム手順: **string-locale-ci=?** s1 s2 \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dlocale_002dci_003d_003f)

C 関数: **scm\_string\_locale\_ci\_eq** (s1、s2、ロケール) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005flocale_005fci_005feq)

文字列s1とs2を、大文字小文字を区別せず、ロケールに応じて比較します。ロケールが指定されている場合は、`make-locale`コマンドで返されるロケールオブジェクトが比較に使用されます。指定されていない場合は、現在のシステムロケールが使用されます。

Scheme Procedure: **char-locale<?** c1 c2 \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dlocale_003c_003f)

C 関数: **scm\_char\_locale\_lt** (c1, c2, locale) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005flocale_005flt)

Scheme Procedure: **char-locale>?** c1 c2 \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dlocale_003e_003f);

C 関数: **scm\_char\_locale\_gt** (c1, c2, locale) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005flocale_005fgt);

スキーム手順: **char-locale-ci<?** c1 c2 \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dlocale_002dci_003c_003f);

C 関数: **scm\_char\_locale\_ci\_lt** (c1, c2, locale) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005flocale_005fci_005flt);

スキーム手順: **char-locale-ci>?** c1 c2 \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dlocale_002dci_003e_003f);

C 関数: **scm\_char\_locale\_ci\_gt** (c1, c2, locale) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005flocale_005fci_005fgt)

文字 c1 と c2 を、ロケール (`make-locale` によって返されるロケールオブジェクト) または現在のロケールに基づいて比較します。`-ci` オプションの場合、比較は大文字小文字を区別せずに行われます。

Scheme Procedure: **char-locale-ci=?** c1 c2 \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dlocale_002dci_003d_003f)

C 関数: **scm\_char\_locale\_ci\_eq** (c1, c2, locale) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005flocale_005fci_005feq)

文字 c1 が c2 と等しい場合、ロケールまたは現在のロケールに応じて大文字小文字を区別せずに true を返します。

* * *

次へ: [数値の入出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Number-Input-and-Output)、前: [テキスト照合](https://doc.guix.gnu.org/guile/latest/en/guile.html#Text-Collation)、上: [国際化のサポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Internationalization) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.25.3 文字の大文字小文字のマッピング [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Character-Case-Mapping-1)

以下の手順は、「文字の大文字小文字のマッピング」、つまり文字や文字列を大文字または小文字に変換する機能を提供します。SRFI-13 にも同様の手順が用意されています ([アルファベットの大文字小文字のマッピング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Alphabetic-Case-Mapping) を参照)。ただし、SRFI-13 の手順はロケールに依存しません。そのため、特定の言語や地域で使用されている慣習の特殊性は考慮されていません。たとえば、ラテン文字を使用するほとんどの言語では小文字の「i」が大文字の「I」にマッピングされますが、トルコ語では小文字の「i」が「上にドットが付いたラテン文字の大文字の I」にマッピングされます。以下の手順を使用すると、プログラマは慣用的な文字マッピングを提供できます。

Scheme Procedure: **char-locale-downcase** chr \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dlocale_002ddowncase)

C 関数: **scm\_char\_locale\_upcase** (chr, ロケール) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005flocale_005fupcase);

ロケールまたは現在のロケールに応じて、chr に対応する小文字を返します。

スキーム手順: **char-locale-upcase** chr \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dlocale_002dupcase);

C 関数: **scm\_char\_locale\_downcase** (chr, ロケール) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005flocale_005fdowncase);

ロケールまたは現在のロケールに応じて、chr に対応する大文字を返します。

Scheme Procedure: **char-locale-titlecase** chr \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dlocale_002dtitlecase);

C 関数: **scm\_char\_locale\_titlecase** (chr, ロケール) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchar_005flocale_005ftitlecase)

ロケールまたは現在のロケールに応じて、chr に対応するタイトルケース文字を返します。

Scheme手順: **string-locale-upcase** str \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dlocale_002dupcase)

C 関数: **scm\_string\_locale\_upcase** (str, locale) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005flocale_005fupcase)

str を locale または現在のロケールに従って大文字にした新しい文字列を返します。

Scheme手順: **string-locale-downcase** str \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dlocale_002ddowncase)

C 関数: **scm\_string\_locale\_downcase** (str, locale) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005flocale_005fdowncase)

str を小文字に変換した新しい文字列を、ロケールまたは現在のロケールに従って返します。

Scheme 手順: **string-locale-titlecase** str \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dlocale_002dtitlecase)

C 関数: **scm\_string\_locale\_titlecase** (str, locale) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005flocale_005ftitlecase)

str をタイトルケースにした新しい文字列を、ロケールまたは現在のロケールに従って返します。

* * *

次へ: [ロケール情報へのアクセス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Accessing-Locale-Information)、前: [文字の大文字小文字のマッピング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Character-Case-Mapping)、上: [国際化のサポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Internationalization) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.25.4 数値の入出力 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Number-Input-and-Output-1)

以下の手順により、プログラムは特定の地域設定に従って記述された数値を読み書きできるようになります。例えば、英語では「10,000.5」は通常「10,000.5」と表記されますが、フランス語では「10 000,5」と表記されます。これらの手順は、このような表記の違いを考慮に入れることを可能にします。

スキーム手順: **locale-string->integer** str \[base \[locale\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locale_002dstring_002d_003einteger)

C 関数: **scm\_locale\_string\_to\_integer** (str, base, locale) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005flocale_005fstring_005fto_005finteger)

文字列 str を、ロケール (`make-locale` によって返されるロケール オブジェクト) または現在のプロセスのロケールに従って整数に変換します。base が指定されている場合、読み取られる整数の基数 (たとえば、16 進数の場合は `16`、10 進数の場合は `10`) を決定します。デフォルトでは、10 進数が読み取られます。2 つの値を返します ([複数の値の返と受け入れ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Multiple-Values) を参照)。整数 (成功時) または `#f`、および str から読み取られた文字数 (失敗時は `0`)。

この関数は、Cライブラリの`strtol`関数に基づいています（GNU Cライブラリリファレンスマニュアルの[`strtol`](https://doc.guix.gnu.org/libc/latest/en/libc.html#Parsing-of-Integers)を参照）。

スキーム手順: **locale-string->inexact** str \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locale_002dstring_002d_003einexact)

C 関数: **scm\_locale\_string\_to\_inexact** (str, ロケール) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005flocale_005fstring_005fto_005finexact)

文字列 str を、ロケール (`make-locale` によって返されるロケール オブジェクト) または現在のプロセスのロケールに基づいて、不正確な数値に変換します。2 つの値を返します ([複数の値の返と受け入れ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Multiple-Values) を参照)。成功した場合は不正確な数値、失敗した場合は `#f`、str から読み取った文字数 (失敗した場合は `0`) です。

この関数は、Cライブラリの`strtod`関数に基づいています（GNU Cライブラリリファレンスマニュアルの[`strtod`](https://doc.guix.gnu.org/libc/latest/en/libc.html#Parsing-of-Floats)を参照）。

Scheme Procedure: **number->locale-string** number \[fraction-digits \[locale\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-number_002d_003elocale_002dstring)

数値（不正確な値）を、ロケール（ロケールオブジェクト）または現在のロケールの文化的な慣習に従って文字列に変換します。デフォルトでは、上限まで必要な数の小数部を表示します。オプションで、表示する小数部の数を指定する整数に小数部をバインドすることもできます。

スキーム手順: **monetary-amount->locale-string** amount intl? \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-monetary_002damount_002d_003elocale_002dstring)

amount（金額を表す不正確な値）を、locale（localeオブジェクト）または現在のロケールの文化的な慣習に従って文字列に変換します。intl?がtrueの場合、指定されたロケールの国際通貨形式が使用されます（GNU Cライブラリリファレンスマニュアルの[国際通貨形式とロケール通貨形式](https://doc.guix.gnu.org/libc/latest/en/libc.html#Currency-Symbol)を参照）。

* * *

次へ: [Gettext サポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Gettext-Support)、前: [数値入力と出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Number-Input-and-Output)、上: [国際化のサポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Internationalization) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.25.5 ロケール情報へのアクセス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Accessing-Locale-Information-1)

ロケールに関する非常に具体的な情報、例えば曜日や月を表す単語、浮動小数点数の表現形式などを取得すると便利な場合があります。`(ice-9 i18n)` モジュールは、libc の関数 `nl_langinfo()` および `localeconv()` と同様の方法で、この機能をサポートしています（GNU Cライブラリ リファレンス マニュアルの [C 言語からのロケール情報へのアクセス](https://doc.guix.gnu.org/libc/latest/en/libc.html#Locale-Information) を参照）。利用可能な関数は以下のとおりです。

Scheme Procedure: **locale-encoding** \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locale_002dencoding)

ロケールまたは現在のロケールのエンコーディング名（解釈がシステムに依存する文字列）を返します。

以下の関数は日付と時刻を扱います。

スキーム手順: **locale-day** day \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locale_002dday)

スキーム手順: **locale-day-short** day \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locale_002dday_002dshort)

スキーム手順: **locale-month** 月 \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locale_002dmonth)

スキーム手順: **locale-month-short** 月 \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locale_002dmonth_002dshort)

指定されたロケールまたは現在のロケールで、day（またはmonth）で示される日（または月）を表すために使用される単語（文字列）を返します。day（またはmonth）は、1～7（または1～12）の整数です。`-short` オプションを指定すると、完全な名前ではなく略語が表示されます。

スキーム手順: **locale-am-string** \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locale_002dam_002dstring)

スキーム手順: **locale-pm-string** \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locale_002dpm_002dstring)

午前（または午後）を12時間形式で表すために使用される（空の場合もある）文字列を返します。

スキーム手順: **locale-date+time-format** \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locale_002ddate_002btime_002dformat)

スキーム手順: **locale-date-format** \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locale_002ddate_002dformat)

スキーム手順: **locale-time-format** \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locale_002dtime_002dformat)

スキーム手順: **locale-time+am/pm-format** \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locale_002dtime_002bam_002fpm_002dformat)

スキーム手順: **locale-era-date-format** \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locale_002dera_002ddate_002dformat)

スキーム手順: **locale-era-date+time-format** \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locale_002dera_002ddate_002btime_002dformat)

スキーム手順: **locale-era-time-format** \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locale_002dera_002dtime_002dformat)

これらの手順は、特定の制約とロケールまたは現在のロケールの慣例に従って日付/時刻の一部を表示するために使用できる、`strftime` に適したフォーマット文字列を返します ([Time](https://doc.guix.gnu.org/guile/latest/en/guile.html#Time) を参照) (GNU C ライブラリ リファレンス マニュアルの [`nl_langinfo ()` の項目](https://doc.guix.gnu.org/libc/latest/en/libc.html#The-Elegant-and-Fast-Way) を参照)。

スキーム手順: **locale-era** \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locale_002dera)

スキーム手順: **locale-era-year** \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locale_002dera_002dyear)

これらの関数は、それぞれ、ロケールまたは現在のロケールで使用されている該当する時代の紀元と年を返します。ほとんどのロケールではこの値は定義されていません。その場合は、空の文字列が返されます。この値が定義されているロケールの例としては、日本語ロケールがあります。

以下の手順は、数値表現に関する情報を提供する。

スキーム手順: **locale-小数点** \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locale_002ddecmal_002dpoint)

スキーム手順: **locale-thousands-separator** \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locale_002dthousands_002dseparator)

これらの関数は、ロケールまたは現在のロケールにおける小数点または千の位の区切り記号の表現を示す文字列を返します。

スキーム手順: **locale-digit-grouping** \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locale_002ddigit_002dgrouping)

数値の整数部分の桁を小数点から左に向かってどのようにグループ化するかを示す整数のリスト（循環リストになる可能性あり）を返します。リストには、右から左に向かって、連続するグループのサイズを示す整数が含まれます。リストが循環リストでない場合、最後のグループ以降の桁はグループ化されません。

例えば、返されたリストが「3」のみを含む循環リストで、千の位の区切り文字が「","」である場合（英語のロケールの場合と同様）、数値「12345678」は「12,345,678」と表示されるはずです。

以下の手順は、金額の表現に関するものです。これらの手順の中には、指定された地域における国際通貨制度と現地通貨制度のどちらを使用するかを示す追加の引数 intl? (ブール値) を取るものがあります。

スキーム手順: **locale-monetary-decimal-point** \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locale_002dmonetary_002ddecimal_002dpoint)

スキーム手順: **locale-monetary-thousands-separator** \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locale_002dmonetary_002dthousands_002dseparator)

スキーム手順: **locale-monetary-grouping** \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locale_002dmonetary_002dgrouping)

これらは、上記の手続きに対応する金銭的な手続きです。これらの手続きは、金銭的な金額に適用されます。

スキーム手順: **locale-currency-symbol** intl? \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locale_002dcurrency_002dsymbol)

指定されたロケール、または現在のロケールの通貨記号（文字列）を返します。

以下の例は、国内通貨と国際通貨の形式の違いを示しています。

(define us (make-locale LC\_MONETARY "en\_US"))
(locale-currency-symbol #f us)
⇒ "-$"
(locale-currency-symbol #t us)
⇒ 「USD」

スキーム手順: **locale-monetary-fractional-digits** intl? \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- locale_002dmonetary_002dfractional_002ddigits)

金額を表示する際に使用する小数部の桁数を、ロケールまたは現在のロケールに応じて返します。ロケールで指定されていない場合は、`#f` が返されます。

スキーム手順: **locale-currency-symbol-precedes-positive?** intl? \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locale_002dcurrency_002dsymbol_002dprecedes_002dpositive_003f)

スキーム手順: **locale-currency-symbol-precedes-negative?** intl? \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locale_002dcurrency_002dsymbol_002dprecedes_002dnegative_003f)

Scheme Procedure: **locale-positive-separated-by-space?** intl? \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locale_002dpositive_002dseparated_002dby_002dspace_003f)

Scheme Procedure: **locale-negative-separated-by-space?** intl? \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locale_002dnegative_002dseparated_002dby_002dspace_003f)

これらの手順は、通貨記号を正または負の数値の前に付けるかどうか、および通貨記号と正または負の数値の間に空白を挿入するかどうかを示すブール値を返します。

スキーム手順: **locale-monetary-positive-sign** \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locale_002dmonetary_002dpositive_002dsign)

スキーム手順: **locale-monetary-negative-sign** \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locale_002dmonetary_002dnegative_002dsign)

金額を印刷する際に使用すべき正（または負）の符号を示す文字列を返します。

スキーム手順: **locale-positive-sign-position** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locale_002dpositive_002dsign_002dposition)

スキーム手順: **locale-negative-sign-position** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locale_002dnegative_002dsign_002dposition)

これらの関数は、金額を印刷する際に正負の符号を表示する位置を示す記号を返します。指定可能な値は次のとおりです。

`括弧サイズ`

通貨記号と数量は括弧で囲む必要があります。

`sign-before`

数量記号と通貨記号の前に符号文字列を出力してください。

`sign-after`

数量と通貨記号の後に符号文字列を出力してください。

通貨記号の前に符号を付けます

通貨記号の直前に符号文字列を出力してください。

通貨記号の後の符号

通貨記号の直後に符号文字列を出力してください。

`未指定`

指定なし。通貨記号の後に符号を印刷することをお勧めします。

最後に、ユーザーインターフェースをプログラミングする際には、以下の2つの手順が役立つ場合があります。

スキーム手順: **locale-yes-regexp** \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locale_002dyes_002dregexp)

Scheme Procedure: **locale-no-regexp** \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locale_002dno_002dregexp)

はい/いいえの質問に対する肯定的な（または否定的な）回答を認識するための正規表現として使用できる文字列を返します。Cロケールの場合、デフォルト値は通常、それぞれ`"^[yY]"`と`"^[nN]"`です。

以下に例を示します。

(use-modules (ice-9 rdelim))
(フォーマット #t "ガイルは最高？~%")
(let lp ((answer (read-line)))
(cond ((string-match (locale-yes-regexp) answer)
(フォーマット #t "ハイタッチ！~%"))
((string-match (locale-no-regexp) answer)
(フォーマット #t "今はどうですか? もう最高ですか?~%")
(lp (read-line)))
（それ以外
(フォーマット #t "どういう意味ですか?~%")
(lp (read-line)))))

国際化されたはい/いいえの文字列出力には、`gettext`を使用する必要があります（[Gettextサポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Gettext-Support)を参照）。

これらの関数の使用例としては、`number->locale-string` および `monetary-amount->locale-string` プロシージャの実装 ([Number Input and Output](https://doc.guix.gnu.org/guile/latest/en/guile.html#Number-Input-and-Output) を参照)、および SRFI-19 日付と時刻の文字列への変換 ([SRFI-19 - Time/Date Library](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d19) を参照) などがあります。

* * *

前へ: [ロケール情報へのアクセス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Accessing-Locale-Information)、上へ: [国際化のサポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Internationalization) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.25.6 Gettext サポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Gettext-Support-1)

Guileは、メッセージ文字列を変換するためのGNU `gettext`へのインターフェースを提供します（GNU `gettext`ユーティリティの[概要](https://www.gnu.org/software/gettext/manual/gettext.html#Introduction)を参照）。

メッセージはドメインごとに収集されるため、ライブラリやプログラムによってメッセージカタログが異なります。以下の関数におけるドメインパラメータは文字列です（メッセージカタログファイル名の一部となります）。

`gettext` が利用できない場合、または Guile が '\--without-nls' に設定されている場合、翻訳を行わないダミー関数が提供されます。Guile で `gettext` がサポートされている場合は、`i18n` 機能が提供されます ([機能追跡](https://doc.guix.gnu.org/guile/latest/en/guile.html#Feature-Tracking) を参照)。

Scheme Procedure: **gettext** msg \[domain \[category\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-gettext)

C 関数: **scm\_gettext** (msg, domain, category) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fgettext)

ドメイン内の msg の翻訳を返します。domain はオプションで、デフォルトでは以下の `textdomain` で設定されたドメインが使用されます。category はオプションで、デフォルトでは `LC_MESSAGES` が使用されます ([Locales](https://doc.guix.gnu.org/guile/latest/en/guile.html#Locales) を参照)。

通常、msg はリテラル文字列です。`xgettext` は、ソースからそれらを抽出して、翻訳者が使用できるメッセージカタログを作成できます (GNU `gettext` ユーティリティの [`xgettext` プログラムの呼び出し](https://www.gnu.org/software/gettext/manual/gettext.html#xgettext-Invocation) を参照)。

(display (gettext "あなたは曲がりくねった通路の迷路の中にいます。"))

`gettext` の省略形として `G_` を使用するのが一般的です。[24](https://doc.guix.gnu.org/guile/latest/en/guile.html#FOOT24) ライブラリは、特定のドメインを使用して翻訳を検索するように `G_` を定義することができ、プログラムの異なる部分で異なる翻訳ソースを持つことができます。

(define (G\_ msg) (gettext msg "mylibrary"))
(display (G\_ "ファイルが見つかりません。"))

`G_` は、メッセージ文字列から曖昧さを解消するための余分なテキストを削除するのに適した場所でもあります。たとえば、GNU `gettext` ユーティリティの [GUI プログラムでの `gettext` の使用方法](https://www.gnu.org/software/gettext/manual/gettext.html#GUI-program-problems) を参照してください。

スキーム手順: **ngettext** msg msgplural n \[domain \[category\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ngettext)

C 関数: **scm\_ngettext** (msg、msgplural、n、ドメイン、カテゴリ) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fngettext)

msg/msgplural のドメインへの翻訳を返します。n の数に応じて適切な複数形が選択されます。domain はオプションで、デフォルトでは以下の `textdomain` で設定されたドメインが使用されます。category はオプションで、デフォルトでは `LC_MESSAGES` が使用されます ([Locales](https://doc.guix.gnu.org/guile/latest/en/guile.html#Locales) を参照)。

msgは単数形、msgpluralは複数形です。翻訳が利用できない場合、_n = 1_の場合はmsgが、それ以外の場合はmsgpluralが使用されます。翻訳されたメッセージカタログは、異なるルールを持つことができ、2つ以上の形式を持つことができます。

上記の `gettext` の通り、通常の用途では msg と msgplural はリテラル文字列です。これは `xgettext` がソースからそれらを抽出してメッセージカタログを構築できるためです。例:

(define (done n)
(format #t (ngettext "~処理されたファイル\n")
"~a個のファイルが処理されました\n" n)
n))

（完了 1） ⊣ 1 ファイル処理済み
（完了 3） ⊣ 3つのファイルを処理しました

英語の単数形と複数形の規則は他の言語とは異なるため、複数形には通常の `gettext` ではなく `ngettext` を使用することが重要です。翻訳者が正しい形式を入力できるのは `ngettext` のみです（GNU `gettext` ユーティリティの [複数形のための追加機能](https://www.gnu.org/software/gettext/manual/gettext.html#Plural-forms) を参照してください）。

Scheme Procedure: **textdomain** \[domain\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-textdomain)

C 関数: **scm\_textdomain** (ドメイン) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ftextdomain)

gettext のデフォルトドメインを取得または設定します。パラメーターなしで呼び出すと、現在のドメインが返されます。パラメーターを指定して呼び出すと、ドメインが現在のドメインとして設定され、その新しい値が返されます。例:

(テキストドメイン "myprog")
⇒ 「マイプログ」

スキーム手順: **bindtextdomain** ドメイン \[directory\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bindtextdomain)

C 関数: **scm\_bindtextdomain** (ドメイン、ディレクトリ) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbindtextdomain)

ドメインのメッセージ ファイルを検索するディレクトリを取得または設定します。ディレクトリを指定せずに呼び出すと、現在の設定が返されます。ディレクトリを指定して呼び出すと、ドメインのディレクトリが設定され、その新しい設定が返されます。例:

(bindtextdomain "myprog" "/my/tree/share/locale")
⇒ "/my/tree/share/locale"

Autoconf/Automakeを使用する場合、アプリケーションは設定済みの`localedir`をプログラムに取り込む（置換するか、設定ファイルを生成するなどして）ようにし、それをドメインに設定する必要があります。これにより、カタログが標準以外の場所にインストールされている場合でも、カタログを見つけることができるようになります。

スキーム手順: **bind-textdomain-codeset** ドメイン \[encoding\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bind_002dtextdomain_002dcodeset)

C 関数: **scm\_bind\_textdomain\_codeset** (ドメイン、エンコーディング) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbind_005ftextdomain_005fcodeset)

`gettext` がドメインからのメッセージに使用するテキストエンコーディングを取得または設定します。encoding は文字列で、エンコーディングシステムの名前を指定します。例えば、`"8859_1"` のように指定します。（Unix/POSIX システムでは、`iconv` プログラムで使用可能なすべてのエンコーディングを一覧表示できます。）

エンコードを指定せずに呼び出すと、現在の設定が返されます。まだ設定されていない場合は `#f` が返されます。エンコードを指定して呼び出すと、ドメインに対してエンコードが設定され、その新しい設定が返されます。たとえば、

(バインドテキストドメインコードセット「myprog」)
⇒ #f
(bind-textdomain-codeset "myprog" "latin-9")
⇒ 「ラテン9」

要求されたエンコーディングは、翻訳されたデータファイルと異なる場合があります。メッセージは必要に応じて再エンコードされます。ただし、翻訳がない場合、`gettext` はメッセージを変更せずに返します。つまり、再エンコードは行われません。そのため、ソースメッセージ文字列はプレーンなASCII形式が最適です。

現在、Guileはマルチバイト文字を認識できず、文字列関数はマルチバイト文字列内の文字境界を認識しません。ただし、アプリケーションは少なくともそのような文字列を何らかの出力に渡すことはできます。将来的には変更される可能性があります。

* * *

次へ: [コードカバレッジレポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Code-Coverage)、前: [国際化のサポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Internationalization)、上: [APIリファレンス](https://doc.guix.gnu.org/guile/latest/en/guile.html#API-Reference) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
