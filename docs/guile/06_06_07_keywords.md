#### 6.6.7 キーワード [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Keywords-1)

キーワードは、入力しやすい便利な読み書き構文を備えた自己評価オブジェクトです。

GuileのキーワードサポートはR5RSに準拠しており、キーワードが`:`や`#:`で始まること、または`:`で終わることを可能にする(切り替え可能な)読み取り構文拡張を追加しています。

* [キーワードを使用する理由](https://doc.guix.gnu.org/guile/latest/en/guile.html#Why-Use-Keywords_003f)
* [キーワードを使ったコーディング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Coding-With-Keywords)
* [キーワード読み取り構文](https://doc.guix.gnu.org/guile/latest/en/guile.html#Keyword-Read-Syntax)
* [キーワードプロシージャ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Keyword-Procedures)

* * *

次へ: [キーワードによるコーディング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Coding-With-Keywords)、上へ: [キーワード](https://doc.guix.gnu.org/guile/latest/en/guile.html#Keywords) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.7.1 キーワードを使用する理由 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Why-Use-Keywords_003f-1)

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
[...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e)) ;; その他のmake-window引数

一方、キーワードを使用すると、デフォルト値の引数は省略され、デフォルト値以外の引数は適切なキーワードによって明確に識別されます。その結果、呼び出しがはるかに分かりやすくなります。

(make-window #:width 800 #:height 100)

一方、引数の少ない単純な手続きの場合、キーワードの使用は助けになるどころか妨げになるだろう。例えば、プリミティブ手続き `cons` は、次のように呼び出さなければならないとしても改善されないだろう。

([cons](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-cons) #:car x #:cdr y)

したがって、キーワードを使用するかどうかの決定は、純粋に実用的なものです。呼び出し時にプロシージャの呼び出しを明確にするのであれば、キーワードを使用すべきです。

* * *

次へ: [キーワード読み取り構文](https://doc.guix.gnu.org/guile/latest/en/guile.html#Keyword-Read-Syntax)、前: [キーワードを使用する理由](https://doc.guix.gnu.org/guile/latest/en/guile.html#Why-Use-Keywords_003f)、上: [キーワード](https://doc.guix.gnu.org/guile/latest/en/guile.html#Keywords) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.7.2 キーワードを使用したコーディング [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Coding-With-Keywords-1)

プロシージャがキーワードをサポートしたい場合は、レスト引数を受け取り、そのレスト引数の内容からキーワードとその対応する引数を抽出するのに都合の良い手段を用いるべきです。

次の例は、その原理を示しています。`make-window` のコードは、`get-keyword-value` というヘルパープロシージャを使用して、残りの引数から個々のキーワード引数を抽出します。

(define (get-keyword-value args keyword default)
(let ((kv ([memq](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-memq) キーワード引数)))
(if (and kv ([\>=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e_003d) ([length](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-length) kv) 2))
([cadr](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-cadr) kv)
デフォルト）））

(define (make-window . args)
(let ((depth (get-keyword-value args #:depth screen-depth))
(bg (get-keyword-value args #:bg "white"))
([width](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-width) (get-keyword-value args #:width 800))
(height (get-keyword-value args #:height 100))
[...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- _002e_002e_002e))
[...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e)))

しかし、`get-keyword-value` を記述する必要はありません。`(ice-9 optargs)` モジュールには、次のようなキーワードをサポートする手順を実装するために使用できる強力なマクロのセットが用意されています。

([use-modules](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-use_002dmodules) (ice-9 optargs))

(define (make-window . args)
([let-keywords](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-let_002dkeywords) args #f ((depth screen-depth)
(背景「白」)
([width](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-width) 800)
(高さ100)
[...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e)))

あるいは、もっと経済的な言い方をすれば、こうなります。

([use-modules](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-use_002dmodules) (ice-9 optargs))

(define\* (make-window #:key (depth screen-depth)
(背景「白」)
([width](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-width) 800)
(高さ100)
[...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e))

`let-keywords`、`define*`、および`(ice-9 optargs)`モジュールによって提供されるその他の機能の詳細については、[Optional Arguments](https://doc.guix.gnu.org/guile/latest/en/guile.html#Optional-Arguments)を参照してください。

C言語で実装されたプロシージャからのキーワード引数を処理するには、 `scm_c_bind_keyword_arguments`を使用します（[キーワードプロシージャ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Keyword-Procedures)を参照）。

* * *

次へ: [キーワード手順](https://doc.guix.gnu.org/guile/latest/en/guile.html#Keyword-Procedures)、前: [キーワードを使用したコーディング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Coding-With-Keywords)、上: [キーワード](https://doc.guix.gnu.org/guile/latest/en/guile.html#Keywords) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.7.3 キーワード読み取り構文 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Keyword-Read-Syntax-1)

Guile はデフォルトでは R5RS と互換性のあるキーワード構文のみを認識します。`#:NAME` の形式のトークン（`NAME` は Scheme シンボルと同じ構文を持ちます ([シンボルの拡張読み取り構文](https://doc.guix.gnu.org/guile/latest/en/guile.html#Symbol-Read-Syntax) を参照)）は、`NAME` という名前のキーワードの外部表現です。キーワードオブジェクトもこの構文を使用して出力されるため、キーワードオブジェクトを含む値を Guile に読み戻すことができます。式内で使用される場合、キーワードは自己引用オブジェクトになります。

`keywords`の読み取りオプションが`'prefix`に設定されている場合、Guileは代替の読み取り構文`:NAME`も認識します。それ以外の場合は、R5RSの要件に従って、`:NAME`形式のトークンはシンボルとして読み取られます。

`keywords` の読み取りオプションが `'postfix` に設定されている場合、Guile は SRFI-88 の読み取り構文 `NAME:` を認識します ([SRFI-88 キーワード オブジェクト](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d88) を参照)。それ以外の場合は、この形式のトークンはシンボルとして読み込まれます。

代替の非R5RSキーワード構文を有効または無効にするには、 [Reading Scheme Code](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scheme-Read)に記載されている`read-set!`プロシージャを使用します。`prefix`構文と`postfix`構文は相互に排他的であることに注意してください。

([read-set!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002dset_0021) キーワード 'prefix)

＃：タイプ
⇒
＃：タイプ

：タイプ
⇒
＃：タイプ

([read-set!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002dset_0021) キーワード 'postfix)

タイプ：
⇒
＃：タイプ

：タイプ
⇒
：タイプ

([read-set!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002dset_0021) キーワード #f)

＃：タイプ
⇒
＃：タイプ

：タイプ
⊣
エラー: 式 :type:
エラー: 未定義の変数: :type
中止: (未定義変数)

* * *

前へ: [キーワード読み取り構文](https://doc.guix.gnu.org/guile/latest/en/guile.html#Keyword-Read-Syntax)、上へ: [キーワード](https://doc.guix.gnu.org/guile/latest/en/guile.html#Keywords) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.7.4 キーワードプロシージャ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Keyword-Procedures-1)

Scheme手順: **キーワード?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-keyword_003f)

C 関数: **scm\_keyword\_p** (obj) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fkeyword_005fp)

引数 obj がキーワードの場合は `#t` を返し、そうでない場合は `#f` を返します。

スキーム手順: **キーワード→シンボル** キーワード [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-keyword_002d_003esymbol)

C 関数: **scm\_keyword\_to\_symbol** (keyword) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fkeyword_005fto_005fsymbol)

キーワードと同じ名前のシンボルを返します。

Scheme手順: **symbol->keyword** symbol [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-symbol_002d_003ekeyword)

C 関数: **scm\_symbol\_to\_keyword** (symbol) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsymbol_005fto_005fkeyword)

シンボルと同じ名前のキーワードを返します。

C 関数: `int` **scm\_is\_keyword** `(SCM obj)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fis_005fkeyword)

`scm_is_true (scm_keyword_p (obj))` と同等です。

C 関数: `SCM` **scm\_from\_locale\_keyword** `(const char *name)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005flocale_005fkeyword)

C 関数: `SCM` **scm\_from\_locale\_keywordn** `(const char *name, size_t len)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005flocale_005fkeywordn)

それぞれ `scm_symbol_to_keyword (scm_from_locale_symbol (name))` および `scm_symbol_to_keyword (scm_from_locale_symboln (name, len))` と同等です。

これらの関数は、name が C 文字列定数である場合は使用しないでください。現在のロケールが、文字列定数や文字定数に使用される実行文字セットと一致する保証がないためです。最新の C コンパイラのほとんどはデフォルトで UTF-8 を使用するため、そのような場合は `scm_from_utf8_keyword` の使用をお勧めします。

C 関数: `SCM` **scm\_from\_latin1\_keyword** `(const char *name)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005flatin1_005fkeyword)

C 関数: `SCM` **scm\_from\_utf8\_keyword** `(const char *name)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005futf8_005fkeyword)

それぞれ `scm_symbol_to_keyword (scm_from_latin1_symbol (name))` および `scm_symbol_to_keyword (scm_from_utf8_symbol (name))` と同等です。

C 関数: `void` **scm\_c\_bind\_keyword\_arguments** ``(const char *subr, SCM rest, scm_t_keyword_arguments_flags flags, SCM keyword1, SCM *argp1, …, SCM keywordN, SCM *argpN, `SCM_UNDEFINED`)`` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fbind_005fkeyword_005farguments)

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

次へ: [リスト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Lists)、前: [キーワード](https://doc.guix.gnu.org/guile/latest/en/guile.html#Keywords)、上: [データ型](https://doc.guix.gnu.org/guile/latest/en/guile.html#Data-Types) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
