#### 6.6.7 キーワード

キーワードは、入力しやすい便利な読み書き構文を備えた自己評価オブジェクトです。

GuileのキーワードサポートはR5RSに準拠しており、キーワードが`:`や`#:`で始まること、または`:`で終わることを可能にする(切り替え可能な)読み取り構文拡張を追加しています。

* [キーワードを使用する理由](#6671-キーワードを使用する理由)
* [キーワードを使ったコーディング](#6672-キーワードを使用したコーディング)
* [キーワード読み取り構文](#6673-キーワード読み取り構文)
* [キーワードプロシージャ](#6674-キーワードプロシージャ)

* * *

次へ: [キーワードによるコーディング](#6672-キーワードを使用したコーディング)、上へ: [キーワード](#667-キーワード) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 6.6.7.1 キーワードを使用する理由

キーワードは、プログラムやプロシージャが、インターフェースを管理不能にすることなく、多数のオプション引数を受け入れることができるようにしたい場合に役立ちます。

これを説明するために、グラフィカルツールキットを使用して画面上に描画するための新しいウィンドウを作成する、架空の`make-window`プロシージャを考えてみましょう。呼び出し元が指定したいパラメータは多数ありますが、それらは適切にデフォルト値を設定することも可能です。例えば、次のようになります。

* 色深度 – デフォルト: 画面の色深度
* 背景色 – デフォルト: 白
* 幅 – デフォルト: 600
* 高さ – デフォルト: 400

`make-window`がキーワードを使用しなかった場合、呼び出し元は考えられる引数ごとに値を渡す必要があり、正しい引数の順序を記憶し、その引数のデフォルト値を示す特別な値を使用する必要があったでしょう。

(make-window 'default ;; 色深度
'default ;; 背景色
800 ;; 幅
100 ;; 高さ
[...](06_08_macros.md#6821-パターン)) ;; その他のmake-window引数

一方、キーワードを使用すると、デフォルト値の引数は省略され、デフォルト値以外の引数は適切なキーワードによって明確に識別されます。その結果、呼び出しがはるかに分かりやすくなります。

(make-window #:width 800 #:height 100)

一方、引数の少ない単純な手続きの場合、キーワードの使用は助けになるどころか妨げになるだろう。例えば、プリミティブ手続き `cons` は、次のように呼び出さなければならないとしても改善されないだろう。

([cons](06_06_08_pairs.md#668-ペア) #:car x #:cdr y)

したがって、キーワードを使用するかどうかの決定は、純粋に実用的なものです。呼び出し時にプロシージャの呼び出しを明確にするのであれば、キーワードを使用すべきです。

* * *

次へ: [キーワード読み取り構文](#6673-キーワード読み取り構文)、前: [キーワードを使用する理由](#6671-キーワードを使用する理由)、上: [キーワード](#667-キーワード) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 6.6.7.2 キーワードを使用したコーディング

プロシージャがキーワードをサポートしたい場合は、レスト引数を受け取り、そのレスト引数の内容からキーワードとその対応する引数を抽出するのに都合の良い手段を用いるべきです。

次の例は、その原理を示しています。`make-window` のコードは、`get-keyword-value` というヘルパープロシージャを使用して、残りの引数から個々のキーワード引数を抽出します。

(define (get-keyword-value args keyword default)
(let ((kv ([memq](06_06_09_lists.md#6697-リスト検索) キーワード引数)))
(if (and kv ([\>=](06_06_02_numerical_data_types.md#6628-比較述語) ([length](06_06_09_lists.md#6694-リスト選択) kv) 2))
([cadr](06_06_08_pairs.md#668-ペア) kv)
デフォルト）））

(define (make-window . args)
(let ((depth (get-keyword-value args #:depth screen-depth))
(bg (get-keyword-value args #:bg "white"))
([width](04_programming_in_scheme.md#4446-デバッグコマンド) (get-keyword-value args #:width 800))
(height (get-keyword-value args #:height 100))
[...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- _002e_002e_002e))
[...](06_08_macros.md#6821-パターン)))

しかし、`get-keyword-value` を記述する必要はありません。`(ice-9 optargs)` モジュールには、次のようなキーワードをサポートする手順を実装するために使用できる強力なマクロのセットが用意されています。

([use-modules](06_18_modules.md#6182-guileモジュールの使用) (ice-9 optargs))

(define (make-window . args)
([let-keywords](06_07_procedures.md#6742-ice-9-optargs) args #f ((depth screen-depth)
(背景「白」)
([width](04_programming_in_scheme.md#4446-デバッグコマンド) 800)
(高さ100)
[...](06_08_macros.md#6821-パターン)))

あるいは、もっと経済的な言い方をすれば、こうなります。

([use-modules](06_18_modules.md#6182-guileモジュールの使用) (ice-9 optargs))

(define\* (make-window #:key (depth screen-depth)
(背景「白」)
([width](04_programming_in_scheme.md#4446-デバッグコマンド) 800)
(高さ100)
[...](06_08_macros.md#6821-パターン))

`let-keywords`、`define*`、および`(ice-9 optargs)`モジュールによって提供されるその他の機能の詳細については、[Optional Arguments](06_07_procedures.md#674-オプションの引数)を参照してください。

C言語で実装されたプロシージャからのキーワード引数を処理するには、 `scm_c_bind_keyword_arguments`を使用します（[キーワードプロシージャ](#6674-キーワードプロシージャ)を参照）。

* * *

次へ: [キーワード手順](#6674-キーワードプロシージャ)、前: [キーワードを使用したコーディング](#6672-キーワードを使用したコーディング)、上: [キーワード](#667-キーワード) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 6.6.7.3 キーワード読み取り構文

Guile はデフォルトでは R5RS と互換性のあるキーワード構文のみを認識します。`#:NAME` の形式のトークン（`NAME` は Scheme シンボルと同じ構文を持ちます ([シンボルの拡張読み取り構文](06_06_06_symbols.md#6665-シンボルの拡張読み取り構文) を参照)）は、`NAME` という名前のキーワードの外部表現です。キーワードオブジェクトもこの構文を使用して出力されるため、キーワードオブジェクトを含む値を Guile に読み戻すことができます。式内で使用される場合、キーワードは自己引用オブジェクトになります。

`keywords`の読み取りオプションが`'prefix`に設定されている場合、Guileは代替の読み取り構文`:NAME`も認識します。それ以外の場合は、R5RSの要件に従って、`:NAME`形式のトークンはシンボルとして読み取られます。

`keywords` の読み取りオプションが `'postfix` に設定されている場合、Guile は SRFI-88 の読み取り構文 `NAME:` を認識します ([SRFI-88 キーワード オブジェクト](07_05_42_srfi88_keyword_objects.md#7542-srfi-88-キーワードオブジェクト) を参照)。それ以外の場合は、この形式のトークンはシンボルとして読み込まれます。

代替の非R5RSキーワード構文を有効または無効にするには、 [Reading Scheme Code](06_16_reading_and_evaluating_scheme_code.md#6162-リーディングスキームコード)に記載されている`read-set!`プロシージャを使用します。`prefix`構文と`postfix`構文は相互に排他的であることに注意してください。

([read-set!](06_16_reading_and_evaluating_scheme_code.md#6162-リーディングスキームコード) キーワード 'prefix)

＃：タイプ
⇒
＃：タイプ

：タイプ
⇒
＃：タイプ

([read-set!](06_16_reading_and_evaluating_scheme_code.md#6162-リーディングスキームコード) キーワード 'postfix)

タイプ：
⇒
＃：タイプ

：タイプ
⇒
：タイプ

([read-set!](06_16_reading_and_evaluating_scheme_code.md#6162-リーディングスキームコード) キーワード #f)

＃：タイプ
⇒
＃：タイプ

：タイプ
⊣
エラー: 式 :type:
エラー: 未定義の変数: :type
中止: (未定義変数)

* * *

前へ: [キーワード読み取り構文](#6673-キーワード読み取り構文)、上へ: [キーワード](#667-キーワード) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 6.6.7.4 キーワードプロシージャ

Scheme手順: **キーワード?** obj

C 関数: **scm\_keyword\_p** (obj)

引数 obj がキーワードの場合は `#t` を返し、そうでない場合は `#f` を返します。

スキーム手順: **キーワード→シンボル** キーワード

C 関数: **scm\_keyword\_to\_symbol** (keyword)

キーワードと同じ名前のシンボルを返します。

Scheme手順: **symbol->keyword** symbol

C 関数: **scm\_symbol\_to\_keyword** (symbol)

シンボルと同じ名前のキーワードを返します。

C 関数: `int` **scm\_is\_keyword** `(SCM obj)`

`scm_is_true (scm_keyword_p (obj))` と同等です。

C 関数: `SCM` **scm\_from\_locale\_keyword** `(const char *name)`

C 関数: `SCM` **scm\_from\_locale\_keywordn** `(const char *name, size_t len)`

それぞれ `scm_symbol_to_keyword (scm_from_locale_symbol (name))` および `scm_symbol_to_keyword (scm_from_locale_symboln (name, len))` と同等です。

これらの関数は、name が C 文字列定数である場合は使用しないでください。現在のロケールが、文字列定数や文字定数に使用される実行文字セットと一致する保証がないためです。最新の C コンパイラのほとんどはデフォルトで UTF-8 を使用するため、そのような場合は `scm_from_utf8_keyword` の使用をお勧めします。

C 関数: `SCM` **scm\_from\_latin1\_keyword** `(const char *name)`

C 関数: `SCM` **scm\_from\_utf8\_keyword** `(const char *name)`

それぞれ `scm_symbol_to_keyword (scm_from_latin1_symbol (name))` および `scm_symbol_to_keyword (scm_from_utf8_symbol (name))` と同等です。

C 関数: `void` **scm\_c\_bind\_keyword\_arguments** ``(const char *subr, SCM rest, scm_t_keyword_arguments_flags flags, SCM keyword1, SCM *argp1, …, SCM keywordN, SCM *argpN, `SCM_UNDEFINED`)``

変更されないrestから、指定されたキーワード引数を抽出します。キーワード引数keyword1が関連値とともにrestに存在する場合、その値はargp1が指す変数に格納されます。そうでない場合、変数は変更されません。同様に、keywordNおよびargpNまでの他のキーワードと引数ポインタについても処理を行います。`scm_c_bind_keyword_arguments`への引数リストは、`SCM_UNDEFINED`で終了する必要があります。

argp1 から argpN が指す変数は、関連付けられたキーワード引数が存在しない場合、変更されないため、`scm_c_bind_keyword_arguments` を呼び出す前にデフォルト値に初期化する必要があります。あるいは、呼び出し前に `SCM_UNDEFINED` に初期化し、呼び出し後に `SCM_UNBNDP` を使用して、どの変数が提供されたかを確認することもできます。

認識されないキーワード引数が rest に存在し、flags に `SCM_ALLOW_OTHER_KEYS` が含まれていない場合、またはキーワード以外の引数が存在し、flags に `SCM_ALLOW_NON_KEYWORD_ARGUMENTS` が含まれていない場合、例外が発生します。エラー報告のため、subr にはキーワード引数を受け取るプロシージャの名前を指定する必要があります。

例えば：

SCM k_delimiter;
SCM k_grammar;
SCM sym\_infix;

SCM my_string_join (SCM strings, SCM rest)
{
SCM区切り文字 = SCM\_UNDEFINED;
SCM文法 = sym\_infix;

scm\_c\_bind\_keyword\_arguments ("my-string-join", rest, 0,
k_delimiter、&delimiter、
k_grammar、&grammar、
SCM_UNDEFINIDE);

if (SCM\_UNBNDP (区切り文字))
delimiter = scm\_from\_utf8\_string (" ");

return scm_string_join (strings, delimiter, grammar);
}

void my_init()
{
k_delimiter = scm_from_utf8_keyword ("delimiter");
k_grammar = scm_from_utf8_keyword ("grammar");
sym\_infix = scm\_from\_utf8\_symbol ("infix");
scm\_c\_define\_gsubr ("my-string-join", 1, 0, 1, my\_string\_join);
}

* * *

次へ: [リスト](06_06_09_lists.md#669-リスト)、前: [キーワード](#667-キーワード)、上: [データ型](06_06_00_data_types.md#66-データ型) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
