### 6.15 PEG解析

構文解析式文法（PEG）は、テキスト処理のための形式言語を定義する方法です。PEGは、マッチング（正規表現など）にも、再帰下降構文解析器（lex/yaccなど）の構築にも使用できます。Guileは、構文解析中に保持する情報をより細かく制御できる、PEG構文の上位セットを使用しています。

構文に慣れたい場合は、Wikipedia に PEG に関する明確で簡潔な入門記事があります。 [http://en.wikipedia.org/wiki/Parsing\_expression\_grammar](http://en.wikipedia.org/wiki/Parsing_expression_grammar)

PEGを紹介した論文には、PEGの動作原理に関するより詳細な説明と、その構文に関する詳細な説明が記載されています。[https://bford.info/pub/lang/peg.pdf](https://bford.info/pub/lang/peg.pdf)

`(ice-9 peg)`モジュールは、PEGをラムダ式にコンパイルすることで機能します。これらのラムダ式は、コンパイル時にdefineマクロ（`define-peg-pattern`および`define-peg-string-patterns`）を使用して変数に格納することも、コンパイル関数（`compile-peg-pattern`および`peg-string-compile`）を使用して実行時に明示的に計算することもできます。

これらは、解析（`match-pattern`）または検索（`search-for-pattern`）に使用できます。便宜上、`search-for-pattern`はパターンリテラルも受け付けるので、簡単な検索をインラインで記述することも可能です（正規表現はこのように使用されることがよくあります）。

このドキュメントの残りの部分は、構文リファレンス、APIリファレンス、およびチュートリアルで構成されています。

* [PEG構文リファレンス](#6151-peg構文リファレンス)
* [PEG API リファレンス](#6152-peg-api-リファレンス)
* [PEGチュートリアル](#6153-pegチュートリアル)
* [PEG Internals](#6154-peg-内部)

* * *

次へ: [PEG API リファレンス](#6152-peg-api-リファレンス)、上へ: [PEG 解析](#615-peg解析) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 6.15.1 PEG構文リファレンス

#### 標準PEG構文:

PEG パターン: **シーケンス** ab

aを解析します。解析が成功した場合、aとして解析されたテキストの末尾からbの解析を続行します。aとbの両方が成功すれば成功です。

`"a b"`

（そしてab）

PEGパターン: **順序付き選択** ab

a を解析します。これが失敗した場合は、バックトラックして b を解析します。a または b のいずれかが成功すれば成功です。

`"a/b"`

（またはab）

PEG パターン: **ゼロ以上**

前の a によって解析されたテキストの末尾から各 a を開始し、可能な限り連続して a を解析します。必ず成功します。

`"a*"`

`(* a)`

PEG パターン: **1 つ以上**

前の a によって解析されたテキストの末尾から各 a を開始し、可能な限り連続して a を解析します。少なくとも 1 つの a が解析されれば成功です。

`"a+"`

`(+ a)`

PEG パターン: **オプション** a

a を解析しようとします。a が成功すれば成功です。

「え？」

`(? a)`

PEG パターン: が ** ** に続きます

解析が可能かどうかを確認しますが、実際に解析は行いません。解析が成功する場合に成功します。

`"&a"`

（aが続く）

PEG パターン: が ** 後に続かない **

解析が不可能であることを確認しますが、実際には解析しません。解析が失敗する場合に成功します。

`"!a"`

（aが後に続かない）

PEG パターン: **文字列リテラル** “abc”

文字列「abc」を解析します。解析が成功すれば成功となります。

`"'abc'"`

`"abc"`

PEGパターン: **任意の文字**

任意の1文字を解析します。解析対象のテキストがなくなるまで、処理は成功します。

`"."`

`peg-any`

PEG パターン: **文字クラス** ab

aとbが文字の場合の「順序付き選択 ab」の別の構文。

`"[ab]"`

（または「a」「b」）

PEG パターン: **文字範囲** az

aからzまでの間の任意の文字を解析します。

`"[az]"`

`(範囲 #\a #\z)`

PEG パターン: **文字の逆範囲** az

aとzの間に含まれない文字を解析します。

`"[^az]"`

`(範囲外 #\a #\z)`

例：

"(a !b / c &d\*) 'e'+"

こうなります：

（そして
（または
(そして ([not-followed-by](#標準peg構文) b))
(および c ([followed-by](#標準peg構文) ([\*](06_06_02_numerical_data_types.md#66211-算術関数) d))))
([+](06_06_02_numerical_data_types.md#66211-算術関数) "e"))

#### 拡張構文

S式には、いくつかの追加構文があります。

PEG パターン: を **無視**

に一致するテキストを無視します

PEG パターン:をキャプチャします

a に一致するテキストを取得します。

PEG パターン: **peg** a

文字列構文を使用してPEGパターンaを埋め込みます。

例：

"!a / 'b'"

同等です

（または（[peg](#拡張構文) "!a") "b")

そして

(または ([not-followed-by](#標準peg構文) a) "b")

* * *

次へ: [PEG チュートリアル](#6153-pegチュートリアル)、前: [PEG 構文リファレンス](#6151-peg構文リファレンス)、上: [PEG 解析](#615-peg解析) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 6.15.2 PEG API リファレンス

#### マクロの定義

PEGを定義する最も簡単な方法は、defineマクロのいずれかを使用することです（これらのマクロはどちらも`define`式に展開されます）。これらのマクロは、解析関数を変数にバインドします。これらの解析関数は、`match-pattern`または`search-for-pattern`によって呼び出され、PEGマッチレコードが返されます。このレコードから、PEGマッチデコンストラクタ関数を使用して生データを取得できます。より複雑な（そしておそらくより分かりやすい）例は、チュートリアルに記載されています。

Scheme マクロ: **define-peg-string-patterns** peg-string

PEG ペグ文字列内のすべての非終端記号を定義します。より正確には、`define-peg-string-patterns` は PEG のスーパーセットを受け取ります。通常の PEG では、非終端記号とパターンの間に `<-` があります。`define-peg-string-patterns` はこの記号を使用して、構文木を伝播させるべき情報を決定します。通常の `<-` は一致したテキストを構文木を伝播させ、`<--` は一致したテキストに非終端記号の名前をタグ付けして構文木を伝播させ、`<` は一致したテキストを破棄して構文木を伝播させません。また、非終端記号には「-」文字を含めることができますが、通常の PEG では許可されていません。

例えば、私たちが：

(ペグストリングパターンの定義)
"as <- 'a'+
bs <- 'b'+
as-or-bs <- as/bs")
(ペグストリングパターンの定義)
"as-tag <-- 'a'+
bsタグ <-- 'b'+
as-or-bs-tag <-- as-tag/bs-tag")

それから：

([match-pattern](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-match_002dpattern ) as-or-bs "aabbcc") ⇒
#<peg 開始: 0 終了: 2 文字列: aabbcc ツリー: aa>
([match-pattern](#構文解析とマッチング関数) as-or-bs-tag "aabbcc") ⇒
#<peg 開始: 0 終了: 2 文字列: aabbcc ツリー: (as-or-bs-tag (as-tag aa))[\>](06_06_02_numerical_data_types.md#6628-比較述語)

なお、この操作を行うことで、トップレベルで 6 つの変数 (as、bs、as-or-bs、as-tag、bs-tag、as-or-bs-tag) をバインドしています。

Scheme マクロ: **define-peg-pattern** name capture-type peg-sexp

単一の非終端名を定義します。capture-type は、構文解析ツリーに渡される情報の量を決定します。peg-sexp は、S 式形式の PEG です。

capture-type の指定可能な値:

すべて

一致したテキストに非終端記号の名前をタグ付けして、構文解析ツリーを上位に渡します。

`本文`

一致したテキストを構文解析ツリーの上位に渡します。

`なし`

構文解析ツリーを上位に何も渡さない。

例えば、私たちが：

(define-peg-pattern as body ([+](06_06_02_numerical_data_types.md#66211-算術関数) "a"))
(define-peg-pattern bs body ([+](06_06_02_numerical_data_types.md#66211-算術関数) "b"))
(define-peg-pattern as-or-bs body (or as bs))
(define-peg-pattern as-tag all ([+](06_06_02_numerical_data_types.md#66211-算術関数) "a"))
(define-peg-pattern bs-tag all ([+](06_06_02_numerical_data_types.md#66211-算術関数) "b"))
(define-peg-pattern as-or-bs-tag all (or as-tag bs-tag))

それから：

([match-pattern](#構文解析とマッチング関数) as-or-bs "aabbcc") ⇒
#<peg 開始: 0 終了: 2 文字列: aabbcc ツリー: aa>
([match-pattern](#構文解析とマッチング関数) as-or-bs-tag "aabbcc") ⇒
#<peg 開始: 0 終了: 2 文字列: aabbcc ツリー: (as-or-bs-tag (as-tag aa))[\>](06_06_02_numerical_data_types.md#6628-比較述語)

なお、この操作を行うことで、トップレベルで 6 つの変数 (as、bs、as-or-bs、as-tag、bs-tag、as-or-bs-tag) をバインドしています。

#### コンパイル関数

実行時に匿名PEGパターンをコンパイルできると便利な場合があります。これらの関数を使用すると、どちらの構文でもコンパイルできます。

Scheme手順: **peg-string-compile** peg-stringキャプチャタイプ

キャプチャタイプに従って伝播するペグストリングにPEGパターンをコンパイルします（キャプチャタイプは`define-peg-pattern`のいずれかの値になります）。

Scheme Procedure: **compile-peg-pattern** peg-sexp capture-type [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-compile_002dpeg_002dpattern )

キャプチャタイプに従って伝播するpeg-sexp内のPEGパターンをコンパイルします（キャプチャタイプは`define-peg-pattern`のいずれかの値になります）。

これらの関数は構文オブジェクトを返します。構文オブジェクトはマクロで使用する場合に便利です。新しい非終端記号を定義するだけであれば、次のようにします。

(define [exp](06_06_02_numerical_data_types.md#66212-科学関数) '([+](06_06_02_numerical_data_types.md#66211-算術関数) "a"))
(define as ([compile](04_programming_in_scheme.md#4444-コンパイルコマンド) ([compile-peg-pattern](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-compile_002dpeg_002dpattern) [exp](06_06_02_numerical_data_types.md#66212-科学関数) 'body)))

この非終端記号は、通常のPEG関数すべてで使用できます。

([match-pattern](#構文解析とマッチング関数) as "aaaa") ⇒
#<ペグ開始: 0 終了: 5 文字列: aaaaa ツリー: aaaaa>

#### 構文解析とマッチング関数

ここでいう「解析」とは、文字列を最初の文字から始まるツリー構造に解析することを意味し、「マッチング」とは、文字列の中から部分文字列を検索することを意味します。実際には、この2つの関数の違いは、`match-pattern`はインデックス0から始まる有効な部分文字列が見つからない場合に処理を中止するのに対し、`search-for-pattern`は検索を続ける点のみです。これらの制約条件の下では、どちらの関数も「解析」と「マッチング」を同等に実行できます。

Scheme 手順: **match-pattern** 非用語文字列

nontermに格納されているPEGを使用して文字列を解析します。一致するパターンが見つからない場合、`match-pattern`はfalseを返します。一致するパターンが見つかった場合は、PEG一致レコードが返されます。

`define-peg-pattern` の `capture-type` 引数を使用すると、解析中に保持する情報を選択できます。オプションは次のとおりです。

すべて

一致したテキストに非終端記号を付ける

`本文`

一致したテキストのみ

`なし`

何もない

(define-peg-pattern as all ([+](06_06_02_numerical_data_types.md#66211-算術関数) "a"))
([match-pattern](#構文解析とマッチング関数) as "aabbcc") ⇒
#<peg 開始: 0 終了: 2 文字列: aabbcc ツリー: (as aa)[\>](06_06_02_numerical_data_types.md#6628-比較述語)

(define-peg-pattern as body ([+](06_06_02_numerical_data_types.md#66211-算術関数) "a"))
([match-pattern](#構文解析とマッチング関数) as "aabbcc") ⇒
#<peg 開始: 0 終了: 2 文字列: aabbcc ツリー: aa>

(define-peg-pattern as none ([+](06_06_02_numerical_data_types.md#66211-算術関数) "a"))
([match-pattern](#構文解析とマッチング関数) as "aabbcc") ⇒
#<peg 開始: 0 終了: 2 文字列: aabbcc ツリー: ()[\>](06_06_02_numerical_data_types.md#6628-比較述語)

(define-peg-pattern bs body ([+](06_06_02_numerical_data_types.md#66211-算術関数) "b"))
([match-pattern](#構文解析とマッチング関数) bs "aabbcc") ⇒
#f

Scheme マクロ: **search-for-pattern** nonterm-or-peg string

文字列内を検索し、一致する部分式を探します。nonterm-or-peg には、非終端記号またはリテラル PEG パターンを指定できます。リテラル PEG パターンが指定された場合、`search-for-pattern` は、多くのハッカーが慣れ親しんでいる正規表現検索と非常によく似た動作をします。一致するものが見つからなかった場合、`search-for-pattern` は false を返します。一致するものが見つかった場合は、PEG 一致レコードが返されます。

(define-peg-pattern as body ([+](06_06_02_numerical_data_types.md#66211-算術関数) "a"))
([search-for-pattern](#構文解析とマッチング関数) as "aabbcc") ⇒
#<peg 開始: 0 終了: 2 文字列: aabbcc ツリー: aa>
([search-for-pattern](#構文解析とマッチング関数) ([+](06_06_02_numerical_data_types.md#66211-算術関数) "a") "aabbcc") ⇒
#<peg 開始: 0 終了: 2 文字列: aabbcc ツリー: aa>
([search-for-pattern](#構文解析とマッチング関数) "'a'+" "aabbcc") ⇒
#<peg 開始: 0 終了: 2 文字列: aabbcc ツリー: aa>

(define-peg-pattern as all ([+](06_06_02_numerical_data_types.md#66211-算術関数) "a"))
([search-for-pattern](#構文解析とマッチング関数) as "aabbcc") ⇒
#<peg 開始: 0 終了: 2 文字列: aabbcc ツリー: (as aa)[\>](06_06_02_numerical_data_types.md#6628-比較述語)

(define-peg-pattern bs body ([+](06_06_02_numerical_data_types.md#66211-算術関数) "b"))
([search-for-pattern](#構文解析とマッチング関数) bs "aabbcc") ⇒
#<ペグ開始: 2 終了: 4 文字列: aabbcc ツリー: bb>
([search-for-pattern](#構文解析とマッチング関数) ([+](06_06_02_numerical_data_types.md#66211-算術関数) "b") "aabbcc") ⇒
#<ペグ開始: 2 終了: 4 文字列: aabbcc ツリー: bb>
([search-for-pattern](#構文解析とマッチング関数) "'b'+" "aabbcc") ⇒
#<ペグ開始: 2 終了: 4 文字列: aabbcc ツリー: bb>

(define-peg-pattern zs body ([+](06_06_02_numerical_data_types.md#66211-算術関数) "z"))
([search-for-pattern](#構文解析とマッチング関数) zs "aabbcc") ⇒
#f
([search-for-pattern](#構文解析とマッチング関数) ([+](06_06_02_numerical_data_types.md#66211-算術関数) "z") "aabbcc") ⇒
#f
([search-for-pattern](#構文解析とマッチング関数) "'z'+" "aabbcc") ⇒
#f

#### PEG マッチ レコード

`match-pattern`関数と`search-for-pattern`関数はどちらもPEGマッチレコードを返します。これらのレコードから実際の情報を抽出するには、以下の関数を使用します。

Scheme Procedure: **peg:string** match-record

`match-record` の作成時に解析された元の文字列を返します。

スキーム手順: **peg:start** match-record

元の文字列（`peg:string`から取得）の中で、解析された最初の文字のインデックスを返します。これが`peg:end`と同じ場合、何も解析されませんでした。

スキーム手順: **peg:end** match-record

元の文字列（`peg:string`から取得）の中で最後に解析された文字のインデックスより1大きい値を返します。これが`peg:start`と同じ場合、何も解析されませんでした。

Scheme Procedure: **peg:substring** match-record

`match-record` によって解析された部分文字列を返します。これは `(substring (peg:string match-record) (peg:start match-record) (peg:end match-record))` と同等です。

スキーム手順: **peg:tree** match-record

`match-record`によって解析されたツリーを返します。

スキーム手順: **peg-record?** match-record

`match-record`がPEGマッチレコードであればtrueを返し、そうでなければfalseを返します。

例：

(define-peg-pattern bs all ([peg](#拡張構文) "'b'+"))

([search-for-pattern](#構文解析とマッチング関数) bs "aabbcc") ⇒
#<peg 開始: 2 終了: 4 文字列: aabbcc ツリー: (bs bb)[\>](06_06_02_numerical_data_types.md#6628-比較述語)

(let ((pm ([search-for-pattern](#構文解析とマッチング関数) bs "aabbcc")))
\`(([string](06_06_05_strings.md#6653-文字列コンストラクタ) ,([peg:string](#peg-マッチ-レコード) pm))
(開始、([peg:開始](#peg-マッチ-レコード) pm))
(end ,([peg:end](#peg-マッチ-レコード) pm))
([substring](06_06_05_strings.md#6655-文字列の選択) ,([peg:substring](#peg-マッチ-レコード) pm))
(ツリー、([peg:tree](#peg-マッチ-レコード) pm))
([record?](06_06_17_records.md#6617-レコード) ,([peg-record?](#peg-マッチ-レコード) pm)))) ⇒
(([string](06_06_05_strings.md#6653-文字列コンストラクタ) "aabbcc")
（開始2）
（終了4）
([substring](06_06_05_strings.md#6655-文字列の選択) "bb")
(ツリー (bs "bb"))
([record?](06_06_17_records.md#6617-レコード) #t))

#### その他

Scheme手順: **context-flatten** tst lst

述語 tst とリスト lst を受け取ります。すべての要素がアトムであるか、または tst を満たすまで、lst を平坦化します。lst 自体が tst を満たす場合、`(list lst)` が返されます (これは、唯一の要素が tst を満たす平坦なリストです)。

([context-flatten](#その他) (lambda (x) (and ([number?](06_06_02_numerical_data_types.md#6621-scheme-の数値タワー) ([car](06_06_08_pairs.md#668-ペア) x)) ([\=](06_06_02_numerical_data_types.md#6628-比較述語) ([car](06_06_08_pairs.md#668-ペア) x) 1))) '(2 2 (1 1 (2 2)) (2 2 (1 1)))) ⇒
(2 2 (1 1 (2 2)) 2 2 (1 1))
([context-flatten](#その他) (lambda (x) (and ([number?](06_06_02_numerical_data_types.md#6621-scheme-の数値タワー) ([car](06_06_08_pairs.md#668-ペア) x)) ([\=](06_06_02_numerical_data_types.md#6628-比較述語) ([car](06_06_08_pairs.md#668-ペア) x) 1))) '(1 1 (1 1 (2 2)) (2 2 (1 1)))) ⇒
((1 1 (1 1 (2 2)) (2 2 (1 1))))

なぜこれがここにあるのか疑問に思うなら、チュートリアルを見てください。

スキーム手順: **keyword-flatten** terms lst

`context-flatten` のより限定的な形式。終端アトムのリスト `terms` を受け取り、すべての要素がアトムであるか、`terms` のアトムを最初の要素として持つリストになるまで、lst を平坦化します。

([keyword-flatten](#その他) '(ab) '(cab (ac) (bc) (c (ba) (ca)))) ⇒
(cab (ac) (bc) c (ba) ca)

なぜこれがここにあるのか疑問に思うなら、チュートリアルを見てください。

* * *

次へ: [PEG の内部構造](https://doc.guix.gnu.org/guile/latest/en/guile.html#PEG-Internals )、前: [PEG API リファレンス](#6152-peg-api-リファレンス)、上: [PEG の解析](#615-peg解析) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 6.15.3 PEGチュートリアル

#### /etc/passwd の解析

この例では、PEGを使用して/etc/passwdを解析する方法を示します。

まず、/etc/passwd ファイルの例を定義します。

(define \*etc-passwd\*
"root:x:0:0:root:/root:/bin/bash
デーモン:x:1:1:デーモン:/usr/sbin:/bin/sh
bin:x:2:2:bin:/bin:/bin/sh
sys:x:3:3:sys:/dev:/bin/sh
nobody:x:65534:65534:nobody:/nonexistent:/bin/sh
messagebus:x:103:107::/var/run/dbus:/bin/false
")

まず手始めに、/etc/passwd にあるすべてのエントリをリストにまとめてみましょう。

文字列ベースのPEG構文でこれを行うと、次のようになります。

(ペグストリングパターンの定義)
"passwd <- entry\* !.
エントリ <-- (! NL .)\* NL\*
NL < '\\n'")

`passwd` ファイルは、ファイルの末尾 (`!.` (`.` は任意の文字なので、`!.` は「何もない」という意味)) まで 0 個以上のエントリ (`entry*`) で構成されます。非端末 `passwd` のデータをキャプチャしますが、名前でタグ付けしないようにするには、`<-` を使用します。

エントリとは、改行文字ではない 0 文字以上の文字列 (`(! NL .)*`) と、それに続く 0 文字以上の改行文字 (`NL*`) のことです。すべてのエントリに `entry` というタグを付けたいので、`<--` を使用します。

改行は文字通りの改行（`'\n'`）です。出力に改行が大量に含まれて煩雑になるのは避けたいので、`<`を使ってキャプチャしたデータを削除します。

以下は、S式を用いて定義された同じPEGです。

(define-peg-pattern passwd body (and ([\*](06_06_02_numerical_data_types.md#66211-算術関数) entry) ([not-followed-by](#標準peg構文) peg-any)))
(define-peg-pattern entry all (and ([\*](06_06_02_numerical_data_types.md#66211-算術関数) (and ([not-followed-by](#標準peg構文) NL) peg-any))
			([\*](06_06_02_numerical_data_types.md#66211-算術関数) NL)))
(define-peg-pattern NL none "\\n")

明らかにこれははるかに冗長です。一方で、より明示的であるため、自動的に構築しやすくなります。ただし、場合によってはS式を使いやすくするいくつかのテクニックがあります。1つは`ignore`キーワードです。文字列構文では、テキストを別の非終端記号に分割する以外に「このテキストを破棄する」方法がありません。たとえば、改行を破棄するには`NL`を定義する必要がありました。S式構文では、単に`(ignore "\n")`と書くだけで済みます。また、文字列構文の方がはるかに簡潔な場合は、`peg`キーワードを使用して文字列構文をS式構文に埋め込むことができます。たとえば、次のように書くことができます。

(define-peg-pattern passwd body ([peg](#拡張構文) "entry\* !."))

どのように定義しても、`*etc-passwd*`を非終端記号`passwd`で解析すると、同じ結果が得られます。

([peg:tree](#peg-マッチ-レコード) ([match-pattern](#構文解析とマッチング関数) passwd \*etc-passwd\*)) ⇒
((エントリ "root:x:0:0:root:/root:/bin/bash")
(エントリ「デーモン:x:1:1:デーモン:/usr/sbin:/bin/sh」)
(エントリ "bin:x:2:2:bin:/bin:/bin/sh")
(エントリ "sys:x:3:3:sys:/dev:/bin/sh")
(エントリ "nobody:x:65534:65534:nobody:/nonexistent:/bin/sh")
(エントリ "messagebus:x:103:107::/var/run/dbus:/bin/false"))

しかし、注意すべき点があります。

([peg:tree](#peg-マッチ-レコード) ([match-pattern](#構文解析とマッチング関数) passwd "one entry")) ⇒
（エントリー「1件」）

デフォルトでは、PEG によって生成される構文木は、情報を失うことなく可能な限り圧縮されます。最初はこれが望ましいものではないように見えるかもしれませんが、圧縮されていない構文木は非常に厄介です (特定のリストがどの程度深くネストされるかを簡単に予測する方法がなく、空のリストが至る所に散らばっているなど)。ただし、このことによる副作用として、圧縮が過剰になる場合があります。`((entry "one entry"))` が `(entry "one entry")` に圧縮されても情報は破棄されませんが、この特定のケースではおそらく望ましい結果ではありません。

この処理を容易に行うための関数が 2 つあります。`keyword-flatten` と `context-flatten` です。`keyword-flatten` 関数は、キーワードのリストとフラット化するリストを受け取り、すべてのサブリストの最初の要素がキーワードのいずれかになるようにリストを変換します。`context-flatten` 関数も同様ですが、キーワードのリストの代わりに、指定されたサブリストが適切かどうかを示す述語を受け取ります（詳細は API リファレンスを参照してください）。

ここで必要なのは`keyword-flatten`です。

([keyword-flatten](#その他) '(entry) ([peg:tree](#peg-マッチ-レコード) ([match-pattern](#構文解析とマッチング関数) passwd \*etc-passwd\*))) ⇒
((エントリ "root:x:0:0:root:/root:/bin/bash")
(エントリ「デーモン:x:1:1:デーモン:/usr/sbin:/bin/sh」)
(エントリ "bin:x:2:2:bin:/bin:/bin/sh")
(エントリ "sys:x:3:3:sys:/dev:/bin/sh")
(エントリ "nobody:x:65534:65534:nobody:/nonexistent:/bin/sh")
(エントリ "messagebus:x:103:107::/var/run/dbus:/bin/false"))
([keyword-flatten](#その他) '(entry) ([peg:tree](#peg-マッチ-レコード) ([match-pattern](#構文解析とマッチング関数) passwd "one entry"))) ⇒
（（エントリー「1エントリー」））

もちろん、これはやや作為的な例です。実際には、曖昧さを解消するために、非終端記号`passwd`にタグを付けるだけでしょう（S式の場合は`all`キーワード、文字列の場合は`<--`記号を使用します）。

(define-peg-pattern tag-passwd all ([peg](#拡張構文) "entry\* !."))
([peg:tree](#peg-マッチ-レコード) ([match-pattern](#構文解析とマッチング関数) tag-passwd \*etc-passwd\*)) ⇒
(タグパスワード)
(エントリ "root:x:0:0:root:/root:/bin/bash")
(エントリ「デーモン:x:1:1:デーモン:/usr/sbin:/bin/sh」)
(エントリ "bin:x:2:2:bin:/bin:/bin/sh")
(エントリ "sys:x:3:3:sys:/dev:/bin/sh")
(エントリ "nobody:x:65534:65534:nobody:/nonexistent:/bin/sh")
(エントリ "messagebus:x:103:107::/var/run/dbus:/bin/false"))
([peg:tree](#peg-マッチ-レコード) ([match-pattern](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-match_002dpattern ) tag-passwd "one entry"))
(タグパスワード)
（エントリー「1エントリー」）

何かを解析した結果がどうなるか不安な場合は、次の2つの絶対的なルールを覚えておいてください。

1. 解析情報は決して破棄されません。
2. 要素数が2つ未満のリストは決して存在しません。

（１）の目的において、「解析情報」とは、`any`キーワードまたは`<--`記号でタグ付けされたものを指します。プレーンな文字列は連結されます。

この例をもう少し拡張して、実際にpasswdファイルから有用な情報を抽出してみましょう。

(ペグストリングパターンの定義)
"passwd <-- エントリ\* !.
エントリ <-- ログイン C パスワード C uid C gid C nameORcomment C homedir C shell NL\*
ログイン <-- テキスト
パス <-- テキスト
uid <-- \[0-9\]\*
gid <-- \[0-9\]\*
名前またはコメント <-- テキスト
ホームディレクトリ <-- パス
shell <-- path
パス <-- (スラッシュ パス要素)\*
pathELEMENT <-- (!NL !C !'/' .)\*
テキスト <- (!NL !C .)\*
C < ':'
NL < '\\n'
スラッシュ < '/'")

これはかなりきれいな構文解析ツリーを生成します。

(パスワード)
(エントリ (ログイン "root")
（「x」を渡してください）
(uid "0")
(gid "0")
(名前またはコメント "root")
(homedir (path (pathELEMENT "root")))
(shell (path (pathELEMENT "bin") (pathELEMENT "bash"))))
(エントリ (ログイン "daemon")
（「x」を渡してください）
(uid "1")
(gid "1")
(名前またはコメント "デーモン")
(ホームディレクトリ)
(path (pathELEMENT "usr") (pathELEMENT "sbin")))
(shell (path (pathELEMENT "bin") (pathELEMENT "sh"))))
(エントリ (ログイン "bin")
（「x」を渡してください）
(uid "2")
(gid "2")
(名前またはコメント "bin")
(homedir (path (pathELEMENT "bin")))
(shell (path (pathELEMENT "bin") (pathELEMENT "sh"))))
(エントリ (ログイン "sys")
（「x」を渡してください）
(uid "3")
(gid "3")
(名前またはコメント "sys")
(homedir (path (pathELEMENT "dev")))
(shell (path (pathELEMENT "bin") (pathELEMENT "sh"))))
(エントリ (ログイン "nobody")
（「x」を渡してください）
(uid "65534")
(gid "65534")
(名前またはコメント「誰も」)
(homedir (path (pathELEMENT "nonexistent")))
(shell (path (pathELEMENT "bin") (pathELEMENT "sh"))))
(エントリ (ログイン "messagebus")
（「x」を渡してください）
(uid "103")
(gid "107")
名前またはコメント
(ホームディレクトリ)
(パス (pathELEMENT "var")
(pathELEMENT "run")
(pathELEMENT "dbus")))
(shell (path (pathELEMENT "bin") (pathELEMENT "false")))))

フィールドにエントリがない場合（例えば、messagebus の `nameORcomment` など）、シンボルが挿入されることに注意してください。これは「情報を無駄にしない」というルールです。定義時に `*` を使用したため、0 文字の `nameORcomment` に正しくマッチしました。これは通常望ましい動作です。なぜなら、例えば `list-ref` を使用して要素を取り出すことができるからです（すべての要素に既知のオフセットがあるため）。

空の一致を示す記号を表示したくない場合は、`entry` の `nameORcomment` の後に `*` を `+` に置き換え、`?` を追加してください。そうすると、1 文字以上の解析が試みられ、失敗（解析ツリーに何も挿入されない）しますが、nameORcomment に一致しなくても処理が続行されるため、処理は継続されます。

#### 算術式の埋め込み

以下のPEGを使用して、単純な数式を解析できます。

(ペグストリングパターンの定義)
"expr <- sum
sum <-- (product ('+' / '-') sum) / product
product <-- (value ('\*' / '/') product) / value
value <-- number / '(' expr ')'
数字 <-- \[0-9\]+")

それから：

([peg:tree](#peg-マッチ-レコード) ([match-pattern](#構文解析とマッチング関数) expr "1+1/2\*3+(1+1)/2")) ⇒
（合計（積（値（数値 "1")）））
「＋」
(合計 (積)
（値（数値「1」））
"/"
（製品
（値（数値「2」））
「\*」
（積（値（数値「3」））））
「＋」
(合計 (積)
（価値 "（"
（合計（積（値（数値 "1")）））
「＋」
(合計 (積 (値 (数値 "1")))))
")""
"/"
(積 (値 (数値 "2")))))))

このPEGでは無駄な作業はほとんどありません。非終端記号「number」にはタグを付ける必要があります。そうしないと、構文木圧縮の文字列連結段階で数値が算術式と混ざってしまう可能性があるからです（パーサーは「1」の後に「/」が続くのを見て「 1/」と認識してしまう）。迷ったらタグを付けましょう。

これらの構文解析ツリーをLisp式に変換するのは非常に簡単です。

(define (parse-sum sum left . rest)
(if ([null?](06_06_09_lists.md#6692-リスト述語) rest)
([apply](06_16_reading_and_evaluating_scheme_code.md#6165-オンザフライ評価の手順) parse-product left)
([list](06_06_09_lists.md#6693-リストコンストラクタ) ([string->symbol](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002d_003esymbol ) ([car](06_06_08_pairs.md#668-ペア) rest))
	([apply](06_16_reading_and_evaluating_scheme_code.md#6165-オンザフライ評価の手順) parse-product left)
	([apply](06_16_reading_and_evaluating_scheme_code.md#6165-オンザフライ評価の手順) parse-sum ([cadr](06_06_08_pairs.md#668-ペア) rest)))))

(define (parse-product product left . rest)
(if ([null?](06_06_09_lists.md#6692-リスト述語) rest)
([apply](06_16_reading_and_evaluating_scheme_code.md#6165-オンザフライ評価の手順) parse-value left)
([list](06_06_09_lists.md#6693-リストコンストラクタ) ([string->symbol](06_06_06_symbols.md#6664-シンボルに関連する操作) ([car](06_06_08_pairs.md#668-ペア) rest))
	([apply](06_16_reading_and_evaluating_scheme_code.md#6165-オンザフライ評価の手順) parse-value left)
	([apply](06_16_reading_and_evaluating_scheme_code.md#6165-オンザフライ評価の手順) parse-product ([cadr](06_06_08_pairs.md#668-ペア) rest)))))

(define (parse-value value [first](07_05_03_srfi1_list_library.md#7533-セレクタ) . rest)
(if ([null?](06_06_09_lists.md#6692-リスト述語) rest)
([string->number](06_06_02_numerical_data_types.md#6629-数値と文字列の変換) ([cadr](06_06_08_pairs.md#668-ペア) [first](07_05_03_srfi1_list_library.md#7533-セレクタ)))
([apply](06_16_reading_and_evaluating_scheme_code.md#6165-オンザフライ評価の手順) parse-sum ([car](06_06_08_pairs.md#668-ペア) rest))))

(parse-expr parse-sum を定義)

（これらの関数はすべて非常によく似ていることに注意してください。より複雑なPEGの場合は、抽象化する価値があります。）

それから：

([apply](06_16_reading_and_evaluating_scheme_code.md#6165-オンザフライ評価の手順) parse-expr ([peg:tree](#peg-マッチ-レコード) ([match-pattern](#構文解析とマッチング関数) expr "1+1/2\*3+(1+1)/2"))) ⇒
([+](06_06_02_numerical_data_types.md#66211-算術関数) 1 ([+](06_06_02_numerical_data_types.md#66211-算術関数) ([/](06_06_02_numerical_data_types.md#66211-算術関数) 1 ([\*](06_06_02_numerical_data_types.md#66211-算術関数) 2 3)) ([/](06_06_02_numerical_data_types.md#66211-算術関数) ([+](06_06_02_numerical_data_types.md#66211-算術関数) 1 1) 2)))

しかし待ってください！結合法則が間違っています！`(/ 1 (* 2 3))` とあるところは、`(* (/ 1 2) 3)` とあるべきです。

例えば、`"sum <-- (product ('+' / '-') sum) / product"` を `"sum <-- (sum ('+' / '-') product) / product"` に置き換えたくなるかもしれませんが、これは良くない考えです。PEG は左再帰をサポートしていません。その理由を理解するには、パーサーがここで何をするかを想像してみてください。`sum` を解析しようとすると、まず `sum` を解析する必要があります。しかし、そのためには、まず `sum` を解析する必要があります。これがスタックがオーバーフローするまで続きます。

では、PEGを使って左結合二項演算子をどのように解析すればよいのでしょうか？正直なところ、これはPEGの大きな欠点の1つです。汎用的な方法はありませんが、ここでは繰り返し演算子が適切な選択肢となります。

([use-modules](06_18_modules.md#6182-guileモジュールの使用) (srfi srfi-1))

(ペグストリングパターンの定義)
"expr <- sum
sum <-- (product ('+' / '-'))\* product
product <-- (value ('\*' / '/'))\* value
value <-- number / '(' expr ')'
数字 <-- \[0-9\]+")

深呼吸して…
(define (make-left-parser next-func)
(lambda (sum [first](07_05_03_srfi1_list_library.md#7533-セレクタ) . rest) ;; 一般的な形式、以下のコメントは、
;; 合計式を扱っている
(if ([null?](06_06_09_lists.md#6692-リスト述語) rest) ;; form (sum (product ...))
([apply](06_16_reading_and_evaluating_scheme_code.md#6165-オンザフライ評価の手順) next-func [first](07_05_03_srfi1_list_library.md#7533-セレクタ))
(if ([string?](06_06_05_strings.md#6652-文字列述語) ([cadr](06_06_08_pairs.md#668-ペア) [first](07_05_03_srfi1_list_library.md#7533-セレクタ)));; form (sum ((product ...) "+") (product ...))
	([list](06_06_09_lists.md#6693-リストコンストラクタ) ([string->symbol](06_06_06_symbols.md#6664-シンボルに関連する操作) ([cadr](06_06_08_pairs.md#668-ペア) [first](07_05_03_srfi1_list_library.md#7533-セレクタ)))
		([apply](06_16_reading_and_evaluating_scheme_code.md#6165-オンザフライ評価の手順) next-func ([car](06_06_08_pairs.md#668-ペア) [first](07_05_03_srfi1_list_library.md#7533-セレクタ)))
		([apply](06_16_reading_and_evaluating_scheme_code.md#6165-オンザフライ評価の手順) next-func ([car](06_06_08_pairs.md#668-ペア) rest)))
;; 形式 (合計 (((積 ...) "+") ((積 ...) "+")) (積 ...))
	([car](06_06_08_pairs.md#668-ペア)
	([reduce](07_05_03_srfi1_list_library.md#7535-折りたたみ展開マップ) ;; リストを走査して左結合木を構築する
	(ラムダ (lr)
	([list](06_06_09_lists.md#6693-リストコンストラクタ) ([list](06_06_09_lists.md#6693-リストコンストラクタ) ([cadr](06_06_08_pairs.md#668-ペア) r) ([car](06_06_08_pairs.md#668-ペア) r) ([apply](06_16_reading_and_evaluating_scheme_code.md#6165-オンザフライ評価の手順) next-func ([car](06_06_08_pairs.md#668-ペア) l)))
		([文字列->シンボル](06_06_06_symbols.md#6664-シンボルに関連する操作) ([cadr](06_06_08_pairs.md#668-ペア) l)))
	'無視する
	([append](06_06_09_lists.md#6695-追加と逆順) ;; すべての製品のリストを作成します
;; 最初のものは事前に解析しておく必要があります
	([list](06_06_09_lists.md#6693-リストコンストラクタ) ([list](06_06_09_lists.md#6693-リストコンストラクタ) ([apply](06_16_reading_and_evaluating_scheme_code.md#6165-オンザフライ評価の手順) next-func ([caar](06_06_08_pairs.md#668-ペア) [first](07_05_03_srfi1_list_library.md#7533-セレクタ)))
			([string->symbol](06_06_06_symbols.md#6664-シンボルに関連する操作) ([cadar](06_06_08_pairs.md#668-ペア) [first](07_05_03_srfi1_list_library.md#7533-セレクタ)))))
	([cdr](06_06_08_pairs.md#668-ペア) [first](07_05_03_srfi1_list_library.md#7533-セレクタ))
;; 最後の一つは追加する必要があります
	([list](06_06_09_lists.md#6693-リストコンストラクタ) ([append](06_06_09_lists.md#6695-追加と逆順) rest '("done"))))))))))

(define (parse-value value [first](07_05_03_srfi1_list_library.md#7533-セレクタ) . rest)
(if ([null?](06_06_09_lists.md#6692-リスト述語) rest)
([string->number](06_06_02_numerical_data_types.md#6629-数値と文字列の変換) ([cadr](06_06_08_pairs.md#668-ペア) [first](07_05_03_srfi1_list_library.md#7533-セレクタ)))
([apply](06_16_reading_and_evaluating_scheme_code.md#6165-オンザフライ評価の手順) parse-sum ([car](06_06_08_pairs.md#668-ペア) rest))))
(define parse-product (make-left-parser parse-value))
(define parse-sum (make-left-parser parse-product))
(parse-expr parse-sum を定義)

それから：

([apply](06_16_reading_and_evaluating_scheme_code.md#6165-オンザフライ評価の手順) parse-expr ([peg:tree](#peg-マッチ-レコード) ([match-pattern](#構文解析とマッチング関数) expr "1+1/2\*3+(1+1)/2"))) ⇒
([+](06_06_02_numerical_data_types.md#66211-算術関数) ([+](06_06_02_numerical_data_types.md#66211-算術関数) 1 ([\*](06_06_02_numerical_data_types.md#66211-算術関数) ([/](06_06_02_numerical_data_types.md#66211-算術関数) 1 2) 3)) ([/](06_06_02_numerical_data_types.md#66211-算術関数) ([+](06_06_02_numerical_data_types.md#66211-算術関数) 1 1) 2))

ご覧のとおり、これはかなり見苦しいです（`context-flatten` を使用すればもっと見やすくできますが、上記のように記述することで、0 個以上の `*` 式が解析される 3 つの方法をどのように処理するかが明確になります）。幸いなことに、ほとんどの場合は右結合性のみを使用すれば問題ありません。

#### 簡略化された関数

さらに興味深い例として、(非常に)簡略化されたC言語関数を解析する以下の文法を考えてみましょう。

(ペグストリングパターンの定義)
"cfunc <-- cSP ctype cSP cname cSP cargs cLB cSP cbody cRB
ctype <-- cidentifier
cname <-- cidentifier
cargs <-- cLP (! (cSP cRP) carg cSP (cCOMMA / cRP) cSP)\* cSP
carg <-- cSP ctype cSP cname
cbody <-- cstatement \*
識別子 <- \[a-zA-z\]\[a-zA-Z0-9\_\]\*
cstatement <-- (!';'.)\*cSC cSP
cSC < ';'
cCOMMA < ','
cLP < '('
cRP < ')'
cLB < '{'
cRB < '}'
cSP < \[ \\t\\n\]\*")

それから：

([match-pattern](#構文解析とマッチング関数) cfunc "int square(int a) { return a\*a;}") ⇒
（32）
(cfunc (ctype "int")
(cname "square")
(cargs (carg (ctype "int") (cname "a")))
(cbody (cstatement "return a\*a"))))

そして：

([match-pattern](#構文解析とマッチング関数) cfunc "int mod(int a, int b) { int c = a/b;return ab\*c; }") ⇒
（52）
(cfunc (ctype "int")
(cname "mod")
(cargs (carg (ctype "int") (cname "a"))
(carg (ctype "int") (cname "b")))
(cbody (cstatement "int c = a/b")
(cstatement "return a- b\*c"))))

`carg` 非終端記号をすべて `cargs` 非終端記号で囲むことで、構文解析構造の曖昧さを解消し、`match-pattern` の出力に対して `context-flatten` を呼び出す必要がなくなりました。`cstatement` 非終端記号についても同様の手法を用い、`cbody` 非終端記号で囲みました。

ここで使用されている空白非終端記号`cSP`は、構文的に無関係な情報をマッチングするための一般的なパターンの（非常に）便利な例です。`<`でタグ付けされ、 `*`で終わるため、構文解析ツリーが煩雑になることはなく（圧縮ステップですべての空リストが破棄されます）、構文解析が失敗することもありません。

* * *

前へ: [PEG チュートリアル](#6153-pegチュートリアル)、上へ: [PEG 解析](#615-peg解析) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 6.15.4 PEG 内部

PEGパーサーは、文字列を入力として受け取り、それを指定された非終端記号として解析しようとします。PEG実装の重要な考え方は、すべての非終端記号が、文字列を引数として受け取り、その文字列を非終端記号として解析しようとする関数であるということです。関数は常に先頭から開始しますが、解析の最後に何らかの素材が残っていれば、解析は成功したとみなされます。

これにより、さまざまなPEG構文解析操作を簡単にモデル化できます。たとえば、PEG文法`"ab"`を考えてみましょう。これは`(and "a" "b")`とも書けます。これは文字列「ab」に一致します。PEGスタイルでこれを実装するには、次のようになります。

(define (match-and-ab str)
(文字列に一致)
(match-b str))

ご覧のとおり、関数を使うことでシーケンスを簡単にモデル化できます。同様に、`(またはab)` も次のようにモデル化できます。

(define (match-or-ab str)
(または (match-a str) (match-b str)))

ここでは、PEG の `or` 式の意味が Scheme の `or` 演算子に自然にマッピングされます。この関数は `(match-a str)` を実行しようとし、成功した場合はその結果を返します。そうでない場合は `(match-b str)` を実行します。

もちろん、上記のコードだけではうまく動作しません。解析関数同士が通信するための何らかの方法が必要です。実際に使用するインターフェースは以下のとおりです。

#### 構文解析関数インターフェース

構文解析関数は、文字列、その文字列の長さ、および解析を開始する文字列内の位置という3つの引数を取ります。実際には、構文解析関数は部分文字列を分割して渡します。最初の引数は文字のバッファであり、後の2つの引数は、構文解析関数が参照すべきバッファ内の範囲を指定します。

解析関数は、非終端記号に一致しなかった場合は #f を返し、一致した文字列の最終位置を表す整数を最初の要素とするリストを返します。cdr には関数が返したいその他のデータを指定できます。データが他にない場合は '() を返します。

ただし、返される追加データがリストの場合、`match-pattern` はそのリスト内の隣接する文字列を追加します。たとえば、解析関数が `(13 ("a" "b" "c"))` を返す場合、`match-pattern` はその値として `(13 ("abc"))` を使用します。

例えば、実際のインターフェースを使用して「ab」に一致する関数を以下に示します。

(define (match-ab str len pos)
(and ([<=](06_06_02_numerical_data_types.md#6628-比較述語) ([+](06_06_02_numerical_data_types.md#66211-算術関数) pos 2) len)
([string=](06_06_05_strings.md#6657-文字列の比較) str "ab" pos ([+](06_06_02_numerical_data_types.md#66211-算術関数) pos 2))
([list](06_06_09_lists.md#6693-リストコンストラクタ) ([+](06_06_02_numerical_data_types.md#66211-算術関数) pos 2) '()))) ; 追加情報は返されません

"ab")` を実行することで文字列を照合するために使用できます。

#### コードジェネレータと拡張可能な構文

`define-peg-pattern`形式などのPEG式は、内部的に2つのステップで解釈されます。

まず、任意の文字列PEGは、`(ice-9 peg string-peg)`モジュール内のコードによってS式PEGに展開されます。

次に、生成されたS式PEGは、`(ice-9 peg codegen)`モジュールによって解析関数にコンパイルされます。具体的には、S式に対して`compile-peg-pattern`関数が呼び出されます。そして、渡された形式に基づいて処理内容が決定されます。

PEG構文は、`compile-peg-pattern`にその形式に対して行う処理に関するオプションを追加することで拡張できます。拡張された構文は、例えば`my-parsing-form`のようなシンボルに関連付けられ、その形式のすべてのPEG式に対して呼び出されます。

(my-parsing-form [...](06_08_macros.md#6821-パターン))

解析関数は2つの引数を取る必要があります。1つ目は、フォームのすべての引数（ただしフォーム名は含まない）のリストを含む構文オブジェクト、2つ目は、`define-peg-pattern`に渡される`capture-type`引数です。

新しい関数は、`(add-peg-compiler! symbol function)` を呼び出すことで登録できます。ここで、`symbol` はこのタイプの形式を示すシンボルであり、`function` は上記で説明したコード生成関数です。関数 `add-peg-compiler!` は、`(ice-9 peg codegen)` モジュールからエクスポートされます。

* * *

次へ: [メモリ管理とガベージコレクション](06_17_memory_management_and_garbage_collection.md#617-メモリ管理とガベージコレクション)、前: [PEG解析](#615-peg解析)、上: [APIリファレンス](06_00_api_reference.md#6-apiリファレンス) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
