#### 6.6.5 文字列 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Strings-1)

文字列は、固定長の文字シーケンスです。コンストラクタプロシージャを呼び出すことで作成できますが、REPLやSchemeソースファイルに直接入力することもできます。

Schemeの文字列は常に、それが何文字で構成されているかという情報を含んでいるため、C言語のように特別な文字列終端文字はありません。つまり、Schemeの文字列には、ヌル文字「\\0」を含むあらゆる文字を含めることができます。

文字列を効率的に使用するには、Guile が文字列をどのように実装しているかを少し知っておく必要があります。Guile では、文字列はヘッドと、文字が格納されている実際のメモリの 2 つの部分から構成されます。文字列（またはその部分文字列）をコピーする場合、新しいヘッドのみが作成され、メモリは通常コピーされません。2 つのヘッドは最初は同じメモリを指しています。

`string-set!` のように、これら2つの文字列のうちいずれかが変更されると、共通のメモリがコピーされるため、各文字列はそれぞれ独自のメモリを持つことができ、一方を変更しても他方が誤って変更されることはありません。つまり、Guileの文字列は「コピーオンライト」方式を採用しており、メモリの実際のコピーは、いずれかの文字列に書き込みが行われるまで遅延されます。

この実装により、`substring`のような関数は、対象となる文字列に変更が加えられない一般的なケースにおいて非常に効率的になります。

文字列がすぐに変更されることがわかっている場合は、`substring` の代わりに `substring/copy` を使用できます。この関数は、作成時にすぐにコピーを実行します。これは、特にマルチスレッドプログラムにおいて、より効率的です。また、`substring/copy` を使用すると、短い部分文字列が、本来再利用できるはずの非常に大きな元の文字列のメモリを保持してしまうという問題を回避できます。

コピーを完全に回避し、一方の文字列の変更がもう一方の文字列にも反映されるようにしたい場合は、`substring/shared`を使用できます。この手順で作成される文字列は、部分文字列と元の文字列が互いに変更を共有するため、「ミューテーション共有部分文字列」と呼ばれます。

変更を防止したい場合は、`substring/read-only`を使用してください。

GuileはSRFI-13のすべての手順に加え、さらにいくつかの手順を提供します。

* [文字列読み取り構文](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Syntax)
* [文字列述語](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Predicates)
* [文字列コンストラクタ](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Constructors)
* [リスト/文字列変換](https://doc.guix.gnu.org/guile/latest/en/guile.html#List_002fString-Conversion)
* [文字列選択](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Selection)
* [文字列の変更](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Modification)
* [文字列比較](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Comparison)
* [文字列検索](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Searching)
* [アルファベットの大文字小文字のマッピング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Alphabetic-Case-Mapping)
* [文字列の反転と追加](https://doc.guix.gnu.org/guile/latest/en/guile.html#Reversing-and-Appending-Strings)
* [マッピング、折りたたみ、展開](https://doc.guix.gnu.org/guile/latest/en/guile.html#Mapping-Folding-and-Unfolding)
* [その他の文字列操作](https://doc.guix.gnu.org/guile/latest/en/guile.html#Miscellaneous-String-Operations)
* [文字列をバイトとして表現する](https://doc.guix.gnu.org/guile/latest/en/guile.html#Representing-Strings-as-Bytes)
* [C言語との変換/C言語からの変換](https://doc.guix.gnu.org/guile/latest/en/guile.html#Conversion-to_002ffrom-C)
* [String の内部構造](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Internals )

* * *

次へ: [文字列述語](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Predicates)、上へ: [文字列](https://doc.guix.gnu.org/guile/latest/en/guile.html#Strings) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.5.1 文字列読み取り構文 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Read-Syntax)

文字列の読み取り構文は、二重引用符（`"`）で囲まれた任意の長さの文字のシーケンスです。

バックスラッシュはエスケープ文字であり、次の特殊文字を挿入するために使用できます。`\"` と `\\` は R5RS 標準、`\|` は R7RS 標準、次の 7 つは R6RS 標準 (C 構文に従っていることに注意してください)、残りの 4 つは Guile 拡張機能です。

`\\`

バックスラッシュ文字。

`\"`

二重引用符文字（エスケープされていない「"」は文字列の終わりを意味します）。

`\|`

縦棒文字。

`\a`

ベル文字（ASCII 7）。

`\f`

フォームフィード文字（ASCII 12）。

`\n`

改行文字（ASCII 10）。

`\r`

キャリッジリターン文字（ASCII 13）。

`\t`

タブ文字（ASCII 9）。

`\v`

垂直タブ文字（ASCII 11）。

`\b`

バックスペース文字（ASCII 8）。

`\0`

ヌル文字（ASCII 0）。

`\(`

開き括弧。これは、複数行文字列の行頭で使用することで、Emacs Lisp モードの混同を避けることを目的としています。

`\` の後に改行文字 (ASCII 10) が続く

何も変わりません。この方法だと、行末に「\」がある場合、改行なしで次の行の最初の文字から文字列が続きます。

デフォルトでは無効になっているが、`hungry-eol-escapes` リーダーオプションが有効になっている場合、次の行の先頭の空白文字は破棄される。

"foo\\
バー"
⇒ 「フーバー」
([read-enable](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002denable) 'hungry-eol-escapes)
"foo\\
バー"
⇒ 「フーバー」

`\xHH`

2桁の16進数で表される文字コード。例えば、ASCIIのDEL（127）の場合は`\x7f`となります。

`\uHHHH`

4桁の16進数で表される文字コード。例えば、マクロン付きの大文字Aは`\u0100`（U+0100）で表されます。

`\UHHHHHH`

6桁の16進数で表される文字コード。例：`\U010402`。

以下は文字列リテラルの例です。

「フー」
「バーで売っている安酒」
"こんにちは世界"
「こんにちは」と彼は言った。

`\xHH`、`\uHHHH`、`\UHHHHHH`の3つのエスケープシーケンスは、以前のバージョンのGuile用に書かれたコードとの互換性を損なわないように選択されました。R6RS仕様では、16進エスケープに異なる、互換性のない構文が推奨されています。それは、文字コードの後に1～8桁の16進数が続き、最後にセミコロンで終わる形式です。このエスケープ形式を使用したい場合は、リーダーオプション`r6rs-hex-escapes`で有効にできます。

([read-enable](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002denable) 'r6rs-hex-escapes)

リーダーオプションの詳細については、[Reading Scheme Code](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scheme-Read) を参照してください。

* * *

次へ: [文字列コンストラクタ](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Constructors)、前: [文字列読み取り構文](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Syntax)、上: [文字列](https://doc.guix.gnu.org/guile/latest/en/guile.html#Strings) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.5.2 文字列述語 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Predicates-1)

以下の手順を用いることで、与えられた文字列が特定の特性を満たしているかどうかを確認できます。

Scheme手順: **string?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_003f)

C 関数: **scm\_string\_p** (obj) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fp)

objが文字列の場合は`#t`を返し、そうでない場合は`#f`を返します。

C 関数: `int` **scm\_is\_string** `(SCM obj)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fis_005fstring)

objが文字列の場合は`1`を返し、それ以外の場合は`0`を返します。

Scheme手順: **string-null?** str [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dnull_003f)

C 関数: **scm\_string\_null\_p** (str) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fnull_005fp)

文字列の長さがゼロの場合は「#t」を返し、それ以外の場合は「#f」を返します。

([string-null?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dnull_003f) "") ⇒ #t
y ⇒ "foo"
([string-null?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dnull_003f) y) ⇒ #f

Scheme プロシージャ: **string-any** char\_pred s \[start \[end\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dany)

C 関数: **scm\_string\_any** (char\_pred, s, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fany)

文字列 s 内のいずれかの文字に対して char\_pred が true であるかどうかを確認します。

char\_pred には、それと等しいかどうかをチェックする文字、またはそのセットに含まれるかどうかをチェックする文字セット ([文字セット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Character-Sets) を参照)、または呼び出す述語プロシージャを指定できます。

プロシージャでは、開始文字から終了文字まで、文字に対して `(char_pred c)` が順次呼び出されます。char_pred が true (つまり `#f` 以外) を返した場合、`string-any` は停止し、その戻り値が `string-any` の戻り値となります。最後の文字 (つまり _end\-1_) に到達した場合、その呼び出しは末尾呼び出しとなります。

s に文字がない場合 (つまり、開始位置と終了位置が等しい場合)、戻り値は `#f` です。

Scheme Procedure: **string-every** char\_pred s \[start \[end\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002devery)

C 関数: **scm\_string\_every** (char\_pred, s, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fevery)

文字列sの各文字について、char_predがtrueであるかどうかを確認します。

char\_pred には、その文字と等しいすべての文字をチェックする文字、またはそのセットに含まれるすべての文字をチェックする文字セット ([文字セット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Character-Sets) を参照)、または呼び出す述語プロシージャを指定できます。

プロシージャでは、開始文字から終了文字まで、文字に対して `(char_pred c)` が順次呼び出されます。char_pred が `#f` を返すと、`string-every` は停止し、`#f` を返します。最後の文字 (つまり _end\-1_) に到達した場合、その呼び出しは末尾呼び出しとなり、その呼び出しからの戻り値は `string-every` からの戻り値となります。

s に文字がない場合 (つまり、開始位置と終了位置が等しい場合)、戻り値は `#t` になります。

* * *

次へ: [リスト/文字列変換](https://doc.guix.gnu.org/guile/latest/en/guile.html#List_002fString-Conversion)、前: [文字列述語](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Predicates)、上: [文字列](https://doc.guix.gnu.org/guile/latest/en/guile.html#Strings) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.5.3 文字列コンストラクタ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Constructors-1)

文字列コンストラクタの手順では、新しい文字列オブジェクトを作成し、必要に応じて指定された文字データで初期化します。既存の文字列から文字列を作成する方法については、[文字列の選択](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Selection)も参照してください。

Scheme手順: **string** char… [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string)

指定された文字引数から作成された、新たに割り当てられた文字列を返します。

(文字列 #\\x #\\y #\\z) ⇒ "xyz"
(文字列) ⇒ ""

Scheme手順: **list->string** lst [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-list_002d_003estring)

C 関数: **scm\_string** (lst) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring)

文字リストから作成された、新たに割り当てられた文字列を返します。

(list->string '(#\\a #\\b #\\c)) ⇒ "abc"

Scheme手順: **reverse-list->string** lst [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-reverse_002dlist_002d_003estring)

C 関数: **scm\_reverse\_list\_to\_string** (lst) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005freverse_005flist_005fto_005fstring)

文字のリストから作成された、新しく割り当てられた文字列を逆順に返します。

(逆順リスト→文字列 '(#\\a #\\B #\\c)) ⇒ "cBa"

Scheme手順: **make-string** k \[chr\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dstring)

C 関数: **scm\_make\_string** (k, chr) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fmake_005fstring)

長さkの新規に割り当てられた文字列を返します。chrが指定されている場合は、文字列のすべての要素がchrで初期化されます。それ以外の場合は、文字列の内容は未指定です。

C 関数: `SCM` **scm\_c\_make\_string** `(size_t len, SCM chr)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fmake_005fstring)

`scm_make_string`と同様ですが、長さは`size_t`型で指定します。

Scheme プロシージャ: **string-tabulate** proc len [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dtabulate)

C 関数: **scm\_string\_tabulate** (proc, len) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005ftabulate)

procは整数から文字への変換を行うプロシージャです。procを各インデックスに適用して対応する文字列要素を生成することで、サイズlenの文字列を構築します。procをインデックスに適用する順序は指定されていません。

Scheme手順: **string-join** ls \[delimiter \[grammar\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002djoin)

C 関数: **scm\_string\_join** (ls、区切り文字、文法) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fjoin)

文字列リスト ls に文字列を追加します。ls の要素間の区切り文字として文字列区切り文字を使用します。区切り文字のデフォルト値は ' ' です。つまり、ls 内の文字列はスペース文字で区切られて追加されます。文法は、区切り文字を文字列間にどのように配置するかを指定する記号で、デフォルト値は `infix` です。

`infix`

リスト要素間に区切り文字を挿入してください。空の文字列を挿入すると、空のリストが生成されます。

`strict-infix`

`infix`と同様ですが、空のリストが与えられた場合はエラーが発生します。

`接尾辞`

リストの各要素の後に区切り文字を挿入してください。

`prefix`

各リスト要素の前に区切り文字を挿入してください。

* * *

次へ: [文字列の選択](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Selection)、前へ: [文字列コンストラクタ](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Constructors)、上へ: [文字列](https://doc.guix.gnu.org/guile/latest/en/guile.html#Strings) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.5.4 リスト/文字列変換 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#List_002fString-conversion)

文字列を処理する際、まず `string->list` という手順を使って文字列をリスト形式に変換し、その結果得られたリストを操作してから、再び文字列に変換するという方法が便利な場合が多い。これらの手順は、同様のタスクに有効である。

Scheme手順: **string->list** str \[start \[end\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002d_003elist)

C 関数: **scm\_substring\_to\_list** (str, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsubstring_005fto_005flist)

C 関数: **scm\_string\_to\_list** (str) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fto_005flist)

文字列strを文字のリストに変換します。

Scheme手順: **string-split** str char\_pred [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dsplit)

C 関数: **scm\_string\_split** (str, char\_pred) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fsplit)

文字列strを、特定の文字の出現によって区切られた部分文字列のリストに分割します。

* 文字の場合、char_pred と等しい。
* プロシージャである場合、述語 char_pred を満たします。
* は、文字セット char\_pred に含まれている。

区切り文字の間に空のサブストリングが存在する場合、結果リストには空の文字列が返されることに注意してください。

([string-split](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dsplit) "root:x:0:0:root:/root:/bin/bash" #\\:)
⇒
("root" "x" "0" "0" "root" "/root" "/bin/bash")

([string-split](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dsplit) "::" #\\:)
⇒
（"" "" "")

([string-split](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dsplit) "" #\\:)
⇒
（"")

* * *

次へ: [文字列の変更](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Modification)、前: [リスト/文字列の変換](https://doc.guix.gnu.org/guile/latest/en/guile.html#List_002fString-Conversion)、上: [文字列](https://doc.guix.gnu.org/guile/latest/en/guile.html#Strings) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.5.5 文字列の選択 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Selection-1)

これらの手順により、文字列の一部を抽出できます。`string-ref`は個々の文字を出力し、`substring`はより長い文字列から部分文字列を抽出するために使用できます。

Scheme手順: **string-length** string [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dlength)

C 関数: **scm\_string\_length** (string) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005flength)

文字列に含まれる文字数を返します。

C 関数: `size_t` **scm\_c\_string\_length** `(SCM str)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fstring_005flength)

文字列strに含まれる文字数を`size_t`型で返します。

Scheme手順: **string-ref** str k [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dref)

C 関数: **scm\_string\_ref** (str, k) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fref)

ゼロ起点インデックスを使用して、文字列strのk番目の文字を返します。kはstrの有効なインデックスである必要があります。

C 関数: `SCM` **scm\_c\_string\_ref** `(SCM str, size_t k)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fstring_005fref)

ゼロ起点インデックスを使用して、文字列strのk番目の文字を返します。kはstrの有効なインデックスである必要があります。

Scheme手順: **string-copy** str \[start \[end\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dcopy)

C 関数: **scm\_substring\_copy** (str, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsubstring_005fcopy)

C 関数: **scm\_string\_copy** (str) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fcopy)

指定された文字列strのコピーを返します。

返される文字列は、最初はstrと同じストレージを共有しますが、どちらかの文字列が変更されるとすぐにコピーされます。

Scheme手順: **substring** str start \[end\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-substring)

C 関数: **scm\_substring** (str, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsubstring)

str の文字から、インデックス start (含む) からインデックス end (含まない) までの文字で構成される新しい文字列を返します。str は文字列である必要があり、start と end は以下の条件を満たす正確な整数である必要があります。

0 <= 開始 <= 終了 <= `(文字列の長さ str)`。

返される文字列は、最初はstrと同じストレージを共有しますが、どちらかの文字列が変更されるとすぐにコピーされます。

Scheme手順: **substring/shared** str start \[end\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-substring_002fshared)

C 関数: **scm\_substring\_shared** (str, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsubstring_005fshared)

`substring` と同様ですが、文字列が変更されてもストレージは共有されます。そのため、str への変更は新しい文字列に反映され、その逆も同様です。

Scheme手順: **substring/copy** str start \[end\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-substring_002fcopy)

C 関数: **scm\_substring\_copy** (str, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsubstring_005fcopy-1)

`substring`と同様ですが、新しい文字列の格納場所は即座にコピーされます。

Scheme手順: **substring/read-only** str start \[end\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-substring_002fread_002donly)

C 関数: **scm\_substring\_read\_only** (str, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsubstring_005fread_005fonly)

`substring`と同様ですが、結果として得られる文字列は変更できません。

C 関数: `SCM` **scm\_c\_substring** `(SCM str, size_t start, size_t end)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fsubstring)

C 関数: `SCM` **scm\_c\_substring\_shared** `(SCM str, size_t start, size_t end)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fsubstring_005fshared)

C 関数: `SCM` **scm\_c\_substring\_copy** `(SCM str, size_t start, size_t end)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fsubstring_005fcopy)

C 関数: `SCM` **scm\_c\_substring\_read\_only** `(SCM str, size_t start, size_t end)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fsubstring_005fread_005fonly)

`scm_substring`などと同様ですが、境界は`size_t`として指定されます。

Scheme 手順: **string-take** sn [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dtake)

C 関数: **scm\_string\_take** (s, n) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005ftake)

sの最初のn文字を返します。

Scheme手順: **string-drop** sn [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002ddrop)

C 関数: **scm\_string\_drop** (s, n) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fdrop)

sの最初のn文字を除くすべての文字を返します。

Scheme Procedure: **string-take-right** sn [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dtake_002dright)

C 関数: **scm\_string\_take\_right** (s, n) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005ftake_005fright)

s の最後の n 文字を返します。

Scheme Procedure: **string-drop-right** sn [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002ddrop_002dright)

C 関数: **scm\_string\_drop\_right** (s, n) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fdrop_005fright)

s の最後の n 文字を除くすべての文字を返します。

Scheme手順: **string-pad** s len \[chr \[start \[end\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dpad)

Scheme 手順: **string-pad-right** s len \[chr \[start \[end\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dpad_002dright)

C 関数: **scm\_string\_pad** (s, len, chr, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fpad)

C 関数: **scm\_string\_pad\_right** (s, len, chr, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fpad_005fright)

文字列 s の先頭から末尾までの文字を取得し、chr で埋めるか、切り捨てて len 文字にします。

`string-pad` は左側をパディングまたは切り捨てます。例えば

(string-pad "x" 3) ⇒ " x"
(string-pad "abcde" 3) ⇒ "cde"

`string-pad-right` は右側をパディングまたは切り捨てます。例えば

(string-pad-right "x" 3) ⇒ "x "
(string-pad-right "abcde" 3) ⇒ "abc"

Scheme手順: **string-trim** s \[char\_pred \[start \[end\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dtrim)

Scheme 手順: **string-trim-right** s \[char\_pred \[start \[end\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dtrim_002dright)

Scheme 手順: **string-trim-both** s \[char\_pred \[start \[end\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dtrim_002dboth)

C 関数: **scm\_string\_trim** (s, char\_pred, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005ftrim)

C 関数: **scm\_string\_trim\_right** (s, char\_pred, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005ftrim_005fright)

C 関数: **scm\_string\_trim\_both** (s, char\_pred, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005ftrim_005fboth)

s の末尾から char\_pred の出現箇所を削除します。

`string-trim` は文字列の左端 (先頭) から char_pred 文字を削除し、`string-trim-right` は文字列の右端 (末尾) から削除し、`string-trim-both` は両端から削除します。

char\_pred には、文字、文字セット、または各文字に対して呼び出す述語プロシージャを指定できます。char\_pred が指定されていない場合、デフォルトは `char-set:whitespace` に従って空白文字になります ([標準文字セット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Standard-Character-Sets) を参照)。

(string-trim " x ") ⇒ "x "
(string-trim-right "banana" #\\a) ⇒ "banan"
(string-trim-both ".,xy:;" char-set:punctuation)
⇒「xy」
(string-trim-both "xyzzy" (lambda (c)
(または (eqv? c #\\x)
(eqv? c #\\y))))
⇒ "zz"

* * *

次へ: [文字列比較](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Comparison)、前へ: [文字列選択](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Selection)、上へ: [文字列](https://doc.guix.gnu.org/guile/latest/en/guile.html#Strings) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.5.6 文字列の変更 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Modification-1)

これらの手順は、文字列をその場で変更するためのものです。つまり、操作の結果は新しい文字列ではなく、元の文字列のメモリ上の表現が変更されます。

Scheme 手順: **string-set!** str k chr [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dset_0021)

C 関数: **scm\_string\_set\_x** (str, k, chr) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fset_005fx)

文字列 str の要素 k に chr を格納し、未指定の値を返します。k は str の有効なインデックスである必要があります。

C 関数: `void` **scm\_c\_string\_set\_x** `(SCM str, size_t k, SCM chr)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fstring_005fset_005fx)

`scm_string_set_x`と同様ですが、インデックスは`size_t`として指定されます。

Scheme手順: **string-fill!** str chr \[start \[end\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dfill_0021)

C 関数: **scm\_substring\_fill\_x** (str, chr, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsubstring_005ffill_005fx)

C 関数: **scm\_string\_fill\_x** (str, chr) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005ffill_005fx)

指定された文字列の各要素に文字を格納し、未指定の値を返します。

Scheme手順: **substring-fill!** str start end fill [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-substring_002dfill_0021)

C 関数: **scm\_substring\_fill\_x** (str, start, end, fill) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsubstring_005ffill_005fx-1)

str の開始位置と終了位置の間のすべての文字を fill に変更します。

(define y ([string-copy](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dcopy) "abcdefg"))
([substring-fill!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-substring_002dfill_0021 ) y 1 3 #\\r)
y
⇒ "arrdefg"

Scheme手順: **substring-move!** str1 start1 end1 str2 start2 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-substring_002dmove_0021)

C 関数: **scm\_substring\_move\_x** (str1, start1, end1, str2, start2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsubstring_005fmove_005fx)

str1 の start1 と end1 で囲まれた部分文字列を、位置 start2 から str2 にコピーします。str1 と str2 は同じ文字列でも構いません。

Scheme 手順: **string-copy!** target tstart s \[start \[end\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dcopy_0021)

C 関数: **scm\_string\_copy\_x** (target, tstart, s, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fcopy_005fx)

文字列 s のインデックス範囲 [start, end) から文字列 target へ、インデックス tstart から始まる一連の文字をコピーします。文字は必要に応じて左から右、または右から左にコピーされます。target と s が同じ文字列であっても、コピーは必ず実行されます。コピー操作が target 文字列の末尾を超えて実行されるとエラーになります。

* * *

次へ: [文字列検索](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Searching)、前: [文字列変更](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Modification)、上: [文字列](https://doc.guix.gnu.org/guile/latest/en/guile.html#Strings) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.5.7 文字列の比較 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Comparison-1)

このセクションの手順は文字順序述語（[文字](https://doc.guix.gnu.org/guile/latest/en/guile.html#Characters)を参照）に似ていますが、文字シーケンス上で定義されます。

最初のセットはR5RSで規定されており、名前の末尾は「?」です。2番目のセットはSRFI-13で規定されており、名前の末尾は「?」ではありません。

`-ci` で終わる述語は、文字列を比較する際に文字の大文字/小文字を区別しません。現時点では、大文字/小文字を区別しない比較は R5RS ルールを使用して行われ、1 文字の大文字形式を持つ小文字はすべて比較前に大文字に変換されます。ロケールに依存する文字列比較については、[`(ice-9 i18n)` モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#Text-Collation) を参照してください。

Scheme手順: **string=?** s1 s2 s3 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_003d_003f)

辞書式等価述語。すべての文字列の長さが同じで、同じ位置に同じ文字が含まれている場合は `#t` を返し、そうでない場合は `#f` を返します。

プロシージャ `string-ci=?` は大文字と小文字を同じ文字として扱いますが、`string=?` は大文字と小文字を別々の文字として扱います。

Scheme手順: **string<?** s1 s2 s3 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_003c_003f)

辞書順述語。連続する文字列引数 str\_i と str\_i+1 の任意のペアについて、str\_i が辞書順で str\_i+1 より小さい場合は `#t` を返します。

Scheme手順: **string<=?** s1 s2 s3 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_003c_003d_003f)

辞書順述語。連続する文字列引数 str\_i と str\_i+1 の任意のペアについて、str\_i が辞書順で str\_i+1 以下である場合、`#t` を返します。

Scheme手順: **string>?** s1 s2 s3 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_003e_003f)

辞書順述語。連続する文字列引数 str\_i と str\_i+1 の任意のペアについて、str\_i が str\_i+1 より辞書順で大きい場合、`#t` を返します。

Scheme手順: **string>=?** s1 s2 s3 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_003e_003d_003f)

辞書順述語。連続する文字列引数 str\_i と str\_i+1 の任意のペアについて、str\_i が辞書順で str\_i+1 以上である場合、`#t` を返します。

Scheme Procedure: **string-ci=?** s1 s2 s3 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dci_003d_003f)

大文字小文字を区別しない文字列等価述語。すべての文字列の長さが同じで、各位置で構成要素の文字が一致する場合（大文字小文字を区別しない）は「#t」を返し、そうでない場合は「#f」を返します。

Scheme手順: **string-ci<?** s1 s2 s3 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dci_003c_003f)

大文字小文字を区別しない辞書式順序付け述語。連続する文字列引数 str\_i と str\_i+1 の任意のペアについて、str\_i が大文字小文字に関係なく str\_i+1 より辞書式的に小さい場合、`#t` を返します。

Scheme Procedure: **string-ci<=?** s1 s2 s3 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dci_003c_003d_003f)

大文字小文字を区別しない辞書式順序付け述語。連続する文字列引数 str\_i と str\_i+1 の任意のペアについて、str\_i が大文字小文字に関係なく辞書式順序で str\_i+1 以下である場合、`#t` を返します。

Scheme Procedure: **string-ci>?** s1 s2 s3 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dci_003e_003f)

大文字小文字を区別しない辞書式順序付け述語。連続する文字列引数 str\_i と str\_i+1 の任意のペアについて、str\_i が大文字小文字に関係なく str\_i+1 より辞書式的に大きい場合、`#t` を返します。

Scheme Procedure: **string-ci>=?** s1 s2 s3 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dci_003e_003d_003f)

大文字小文字を区別しない辞書式順序付け述語。連続する文字列引数 str\_i と str\_i+1 の任意のペアについて、str\_i が大文字小文字に関係なく str\_i+1 以上である場合、`#t` を返します。

Scheme プロシージャ: **string-compare** s1 s2 proc\_lt proc\_eq proc\_gt \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dcompare)

C 関数: **scm\_string\_compare** (s1, s2, proc\_lt, proc\_eq, proc\_gt, start1, end1, start2, end2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fcompare)

s1 が s2 より小さいか、等しいか、大きいかに応じて、不一致インデックスに proc_lt、proc_eq、proc_gt を適用します。不一致インデックスは、0 <= j < i のすべてに対して s1\[j\] = s2\[j\] となる最大のインデックス i です。つまり、i は一致しない最初の位置です。

Scheme プロシージャ: **string-compare-ci** s1 s2 proc\_lt proc\_eq proc\_gt \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dcompare_002dci)

C 関数: **scm\_string\_compare\_ci** (s1, s2, proc\_lt, proc\_eq, proc\_gt, start1, end1, start2, end2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fcompare_005fci)

s1 が s2 より小さいか、等しいか、大きいかに応じて、不一致インデックスに proc\_lt、proc\_eq、proc\_gt を適用します。不一致インデックスは、0 <= j < i のすべてに対して s1\[j\] = s2\[j\] となる最大のインデックス i です。つまり、i は小文字が一致しない最初の位置です。

Scheme手順: **string=** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_003d)

C 関数: **scm\_string\_eq** (s1, s2, start1, end1, start2, end2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005feq)

s1とs2が等しくない場合は`#f`を返し、そうでない場合は真の値を返します。

Scheme 手順: **string<>** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_003c_003e)

C 関数: **scm\_string\_neq** (s1, s2, start1, end1, start2, end2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fneq)

s1とs2が等しい場合は`#f`を返し、そうでない場合はtrueを返します。

Scheme 手順: **string<** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_003c)

C 関数: **scm\_string\_lt** (s1, s2, start1, end1, start2, end2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005flt)

s1がs2以上の場合、`#f`を返し、そうでない場合はtrueを返します。

Scheme 手順: **string>** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_003e)

C 関数: **scm\_string\_gt** (s1, s2, start1, end1, start2, end2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fgt)

s1がs2以下の場合、`#f`を返し、それ以外の場合はtrueを返します。

Scheme 手順: **string<=** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_003c_003d)

C 関数: **scm\_string\_le** (s1, s2, start1, end1, start2, end2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fle)

s1がs2より大きい場合は`#f`を返し、そうでない場合はtrueを返します。

Scheme 手順: **string>=** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_003e_003d)

C 関数: **scm\_string\_ge** (s1, s2, start1, end1, start2, end2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fge)

s1がs2より小さい場合は`#f`を返し、そうでない場合はtrueを返します。

Scheme Procedure: **string-ci=** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dci_003d)

C 関数: **scm\_string\_ci\_eq** (s1, s2, start1, end1, start2, end2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fci_005feq)

s1とs2が等しくない場合は`#f`を返し、それ以外の場合はtrueを返します。文字比較は大文字小文字を区別せずに行われます。

Scheme Procedure: **string-ci<>** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dci_003c_003e)

C 関数: **scm\_string\_ci\_neq** (s1, s2, start1, end1, start2, end2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fci_005fneq)

s1とs2が等しい場合は`#f`を返し、そうでない場合はtrueを返します。文字比較は大文字小文字を区別せずに行われます。

Scheme 手順: **string-ci<** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dci_003c)

C 関数: **scm\_string\_ci\_lt** (s1, s2, start1, end1, start2, end2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fci_005flt)

s1がs2以上の場合、`#f`を返し、それ以外の場合はtrueを返します。文字比較は大文字と小文字を区別せずに行われます。

Scheme Procedure: **string-ci>** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dci_003e)

C 関数: **scm\_string\_ci\_gt** (s1, s2, start1, end1, start2, end2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fci_005fgt)

s1がs2以下の場合、`#f`を返し、それ以外の場合はtrueを返します。文字比較は大文字と小文字を区別せずに行われます。

Scheme 手順: **string-ci<=** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dci_003c_003d)

C 関数: **scm\_string\_ci\_le** (s1, s2, start1, end1, start2, end2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fci_005fle)

s1がs2より大きい場合は`#f`を返し、そうでない場合はtrueを返します。文字比較は大文字と小文字を区別せずに行われます。

Scheme 手順: **string-ci>=** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dci_003e_003d)

C 関数: **scm\_string\_ci\_ge** (s1, s2, start1, end1, start2, end2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- scm_005fstring_005fci_005fge)

s1がs2より小さい場合は`#f`を返し、そうでない場合はtrueを返します。文字比較は大文字小文字を区別せずに行われます。

Scheme Procedure: **string-hash** s \[bound \[start \[end\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dhash)

C 関数: **scm\_substring\_hash** (s, bound, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsubstring_005fhash)

s のハッシュ値を計算します。オプション引数 bound は、ハッシュ関数の範囲を指定する非負の整数です。正の値は、戻り値を [0, bound) の範囲に制限します。

Scheme Procedure: **string-hash-ci** s \[bound \[start \[end\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dhash_002dci)

C 関数: **scm\_substring\_hash\_ci** (s, bound, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsubstring_005fhash_005fci)

s のハッシュ値を計算します。オプション引数 bound は、ハッシュ関数の範囲を指定する非負の整数です。正の値は、戻り値を [0, bound) の範囲に制限します。

抽象的な Unicode 文字の同じ視覚的外観は、複数の Unicode 文字のシーケンスによって得られるため、上記で説明した大文字小文字を区別しない文字列比較関数でも、同じ文字の異なる表現を含む文字列が与えられた場合、`#f` を返すことがあります。たとえば、Unicode 文字「LATIN SMALL LETTER S WITH DOT BELOW AND DOT ABOVE」は、1 つの文字 (U+1E69) で表すことも、文字「LATIN SMALL LETTER S」(U+0073) に結合記号「COMBINING DOT BELOW」(U+0323) と「COMBINING DOT ABOVE」(U+0307) を付けて表すこともできます。

このため、比較対象となる文字列において、各文字の表現が相互に一貫していることを確認することが望ましい場合が多い。Unicode規格では、文字列の内容を正規化する2つの方法が定義されている。1つは、複合文字をUnicode規格で定義された順序で構成文字のセットに分解する分解法、もう1つはその逆を行う合成法である。

分解操作には2種類あります。「正規分解」は、元の文字と同じ視覚的外観を持つ文字シーケンスを生成するのに対し、「互換性分解」は、視覚的外観は元の文字と異なる場合があるものの、同じ抽象的な文字を表す文字シーケンスを生成します。

これらの操作は、以下の正規化形式にカプセル化されています。

_NFD_

キャラクターは、その正統的な形態に分解される。

_NFKD_

文字は互換性のある形式に分解されます。

_NFC_

キャラクターは、その正統的な形態に分解され、その後合成される。

_NFKC_

文字は互換性のある形式に分解され、その後合成される。

以下の関数は、引数を上述のいずれかの形式に変換します。

Scheme 手順: **string-normalize-nfd** s [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dnormalize_002dnfd)

C 関数: **scm\_string\_normalize\_nfd** (s) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fnormalize_005fnfd)

s の `NFD` 正規化形式を返します。

Scheme 手順: **string-normalize-nfkd** s [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dnormalize_002dnfkd)

C 関数: **scm\_string\_normalize\_nfkd** (s) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fnormalize_005fnfkd)

s の `NFKD` 正規化形式を返します。

Scheme Procedure: **string-normalize-nfc** s [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dnormalize_002dnfc)

C 関数: **scm\_string\_normalize\_nfc** (s) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fnormalize_005fnfc)

s の `NFC` 正規化形式を返します。

Scheme 手順: **string-normalize-nfkc** s [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dnormalize_002dnfkc)

C 関数: **scm\_string\_normalize\_nfkc** (s) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fnormalize_005fnfkc)

s の `NFKC` 正規化形式を返します。

* * *

次へ: [アルファベットの大文字小文字のマッピング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Alphabetic-Case-Mapping)、前: [文字列の比較](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Comparison)、上: [文字列](https://doc.guix.gnu.org/guile/latest/en/guile.html#Strings) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.5.8 文字列検索 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Searching-1)

Scheme手順: **string-index** s char\_pred \[start \[end\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dindex)

C 関数: **scm\_string\_index** (s, char\_pred, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005findex)

文字列sを左から右に検索し、文字が最初に出現するインデックスを返します。

* 文字の場合は char\_pred と等しい。
* プロシージャである場合、述語 char_pred を満たします。
* は、文字セット char\_pred に含まれている。

一致するものが見つからない場合は、`#f`を返します。

Scheme手順: **string-rindex** s char\_pred \[start \[end\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002drindex)

C 関数: **scm\_string\_rindex** (s, char\_pred, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005frindex)

文字の最後の出現位置のインデックスを返します。

* 文字の場合は char\_pred と等しい。
* プロシージャである場合、述語 char_pred を満たします。
char\_pred が文字セットの場合、* はそのセットに含まれます。

一致するものが見つからない場合は、`#f`を返します。

Scheme 手順: **string-prefix-length** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dprefix_002dlength)

C 関数: **scm\_string\_prefix\_length** (s1, s2, start1, end1, start2, end2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fprefix_005flength)

2つの文字列に共通する最長の接頭辞の長さを返します。

Scheme Procedure: **string-prefix-length-ci** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dprefix_002dlength_002dci)

C 関数: **scm\_string\_prefix\_length\_ci** (s1, s2, start1, end1, start2, end2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fprefix_005flength_005fci)

2つの文字列の中で、共通する最長の接頭辞の長さを返します。文字の大文字・小文字は区別しません。

Scheme Procedure: **string-suffix-length** s1 s2 \[start1\[end1\[start2\[end2\]\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dsuffix_002dlength);

C 関数: **scm\_string\_suffix\_length** (s1, s2, start1, end1, start2, end2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fsuffix_005flength);

2つの文字列に共通する最長の接尾辞の長さを返します。

Scheme Procedure: **string-suffix-length-ci** s1 s2 \[start1\[end1\[start2\[end2\]\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dsuffix_002dlength_002dci);

C 関数: **scm\_string\_suffix\_length\_ci** (s1, s2, start1, end1, start2, end2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fsuffix_005flength_005fci);

2つの文字列に共通する最長の接尾辞の長さを返します。文字の大文字・小文字は区別しません。

Scheme 手順: **string-prefix?** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dprefix_003f)

C 関数: **scm\_string\_prefix\_p** (s1, s2, start1, end1, start2, end2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fprefix_005fp)

s1はs2の接頭辞ですか？

Scheme 手順: **string-prefix-ci?** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dprefix_002dci_003f)

C 関数: **scm\_string\_prefix\_ci\_p** (s1, s2, start1, end1, start2, end2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fprefix_005fci_005fp)

文字の大文字・小文字を区別しない場合、s1はs2の接頭辞ですか？

Scheme Procedure: **string-suffix?** s1 s2 \[start1\[end1\[start2\[end2\]\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dsuffix_003f);

C 関数: **scm\_string\_suffix\_p** (s1, s2, start1, end1, start2, end2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- scm_005fstring_005fsuffix_005fp);

s1はs2の接尾辞ですか？

スキーム手順: **string-suffix-ci?** s1 s2 \[start1\[end1\[start2\[end2\]\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dsuffix_002dci_003f);

C 関数: **scm\_string\_suffix\_ci\_p** (s1, s2, start1, end1, start2, end2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fsuffix_005fci_005fp);

文字の大文字・小文字を区別しない場合、s1はs2の接尾辞ですか？

Scheme 手順: **string-index-right** s char\_pred \[start \[end\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dindex_002dright)

C 関数: **scm\_string\_index\_right** (s, char\_pred, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005findex_005fright)

文字列sを右から左に検索し、文字の最後の出現位置のインデックスを返します。

* 文字の場合は char\_pred と等しい。
* プロシージャである場合、述語 char_pred を満たします。
char\_pred が文字セットの場合、* はそのセットに含まれます。

一致するものが見つからない場合は、`#f`を返します。

Scheme 手順: **string-skip** s char\_pred \[start \[end\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dskip)

C 関数: **scm\_string\_skip** (s, char\_pred, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fskip)

文字列sを左から右に検索し、文字が最初に出現するインデックスを返します。

* 文字の場合、char_pred と等しくありません。
* プロシージャの場合、述語 char_pred を満たしません。
char\_pred が文字セットの場合、* はセットに含まれません。

Scheme 手順: **string-skip-right** s char\_pred \[start \[end\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dskip_002dright)

C 関数: **scm\_string\_skip\_right** (s, char\_pred, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fskip_005fright)

文字列sを右から左に検索し、文字の最後の出現位置のインデックスを返します。

* 文字の場合、char_pred と等しくありません。
* プロシージャの場合、述語 char_pred を満たしません。
char\_pred が文字セットの場合、* はセットに含まれません。

Scheme手順: **string-count** s char\_pred \[start \[end\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dcount)

C 関数: **scm_string_count** (s, char_pred, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fcount)

文字列sに含まれる文字数を返します。

* 文字の場合は char\_pred と等しい。
* プロシージャである場合、述語 char_pred を満たします。
* は、文字セット char\_pred に含まれている。

Scheme 手順: **string-contains** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dcontains)

C 関数: **scm\_string\_contains** (s1, s2, start1, end1, start2, end2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fcontains)

文字列s1に文字列s2が含まれているかどうか。s2が部分文字列として出現するs1内のインデックスを返します。含まれていない場合はfalseを返します。オプションの開始/終了インデックスを指定すると、操作対象が指定された部分文字列に限定されます。

Scheme 手順: **string-contains-ci** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dcontains_002dci)

C 関数: **scm\_string\_contains\_ci** (s1, s2, start1, end1, start2, end2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fcontains_005fci)

文字列s1に文字列s2が含まれているかどうかを確認します。s2が部分文字列として出現するs1内のインデックスを返します。含まれていない場合はfalseを返します。オプションの開始/終了インデックスを指定すると、操作対象を指定された部分文字列に限定できます。文字比較は大文字と小文字を区別せずに行われます。

* * *

次へ: [文字列の反転と追加](https://doc.guix.gnu.org/guile/latest/en/guile.html#Reversing-and-Appending-Strings)、前: [文字列の検索](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Searching)、上: [文字列](https://doc.guix.gnu.org/guile/latest/en/guile.html#Strings) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.5.9 アルファベットの大文字小文字のマッピング [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Alphabetic-Case-Mapping-1)

これらは、文字列をそれぞれ大文字または小文字にマッピングしたり、文字列を大文字にしたりするための手順です。

Unicode文字の基本的な大文字小文字の対応付け規則を使用します。特別な言語規則や文脈規則は考慮されません。結果として得られる文字列は、入力文字列と同じ長さであることが保証されます。

ロケールに依存する大文字小文字の変換については、[`(ice-9 i18n)` モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#Character-Case-Mapping) を参照してください。

Scheme手順: **string-upcase** str \[start \[end\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dupcase)

C 関数: **scm\_substring\_upcase** (str, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsubstring_005fupcase)

C 関数: **scm\_string\_upcase** (str) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fupcase)

`str`内のすべての文字を大文字にします。

Scheme 手順: **string-upcase!** str \[start \[end\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dupcase_0021)

C 関数: **scm\_substring\_upcase\_x** (str, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsubstring_005fupcase_005fx)

C 関数: **scm\_string\_upcase\_x** (str) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fupcase_005fx)

`str`内のすべての文字を破壊的に大文字化します。

([string-upcase!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dupcase_0021) y)
⇒ "ARRDEFG"
y
⇒ "ARRDEFG"

Scheme手順: **string-downcase** str \[start \[end\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002ddowncase)

C 関数: **scm\_substring\_downcase** (str, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsubstring_005fdowncase)

C 関数: **scm\_string\_downcase** (str) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fdowncase)

str のすべての文字を小文字にします。

Scheme 手順: **string-downcase!** str \[start \[end\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002ddowncase_0021)

C 関数: **scm\_substring\_downcase\_x** (str, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsubstring_005fdowncase_005fx)

C 関数: **scm\_string\_downcase\_x** (str) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fdowncase_005fx)

str のすべての文字を破壊的に小文字にします。

y
⇒ "ARRDEFG"
([string-downcase!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002ddowncase_0021 )y)
⇒ "arrdefg"
y
⇒ "arrdefg"

Scheme 手順: **string-capitalize** str [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dcapitalize)

C 関数: **scm\_string\_capitalize** (str) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fcapitalize)

strに含まれる文字を含む、新たに割り当てられた文字列を返します。ただし、各単語の最初の文字は大文字になります。

Scheme 手順: **string-capitalize!** str [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dcapitalize_0021)

C 関数: **scm\_string\_capitalize\_x** (str) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fcapitalize_005fx)

str の各単語の最初の文字を破壊的に大文字に変換し、str を返します。

y ⇒ 「ハローワールド」
([string-capitalize!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dcapitalize_0021) y) ⇒ "Hello World"
y ⇒ 「ハローワールド」

Scheme手順: **string-titlecase** str \[start \[end\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dtitlecase)

C 関数: **scm\_string\_titlecase** (str, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005ftitlecase)

str 内の単語の最初の文字をすべてタイトルケースにします。

Scheme 手順: **string-titlecase!** str \[start \[end\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dtitlecase_0021)

C 関数: **scm\_string\_titlecase\_x** (str, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005ftitlecase_005fx)

str 内の単語の最初の文字をすべて破壊的にタイトルケースにします。

* * *

次へ: [マッピング、折りたたみ、展開](https://doc.guix.gnu.org/guile/latest/en/guile.html#Mapping-Folding-and-Unfolding)、前: [アルファベットケースのマッピング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Alphabetic-Case-Mapping)、上: [文字列](https://doc.guix.gnu.org/guile/latest/en/guile.html#Strings) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.5.10 文字列の反転と追加 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Reversing-and-Appending-Strings-1)

Scheme手順: **string-reverse** str \[start \[end\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dreverse)

C 関数: **scm\_string\_reverse** (str, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005freverse)

文字列strを反転します。オプション引数startとendは、strの操作対象領域を指定します。

Scheme 手順: **string-reverse!** str \[start \[end\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dreverse_0021)

C 関数: **scm\_string\_reverse\_x** (str, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005freverse_005fx)

文字列strをその場で反転します。オプション引数startとendは、strの操作対象領域を指定します。戻り値は未定義です。

Scheme プロシージャ: **string-append** arg … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dappend)

C 関数: **scm\_string\_append** (args) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fappend)

指定された文字列arg...を連結した文字で構成される、新しく割り当てられた文字列を返します。

(let ((h "hello "))
(文字列に h "world") を追加)
⇒ 「ハローワールド」

Scheme プロシージャ: **string-append/shared** arg … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dappend_002fshared)

C 関数: **scm\_string\_append\_shared** (args) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fappend_005fshared)

`string-append`と同様ですが、結果が引数文字列とメモリを共有する場合があります。

Scheme手順: **string-concatenate** ls [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dconcatenate)

C 関数: **scm\_string\_concatenate** (ls) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fconcatenate)

ls の各要素（文字列である必要があります）を連結して、1 つの文字列を作成します。必ず新しく割り当てられた文字列を返します。

Scheme手順: **string-concatenate-reverse** ls \[final\_string \[end\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dconcatenate_002dreverse)

C 関数: **scm\_string\_concatenate\_reverse** (ls, final\_string, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fconcatenate_005freverse)

オプション引数がない場合、この手順は以下と同等です。

([string-concatenate](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dconcatenate) ([reverse](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-reverse) ls))

オプション引数 final\_string が指定されている場合、リストの反転および文字列連結操作を実行する前に、final\_string が ls の先頭に追加されます。end が指定されている場合は、final\_string の end までの文字のみが使用されます。

新しく割り当てられた文字列を返すことを保証します。

Scheme手順: **string-concatenate/shared** ls [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dconcatenate_002fshared)

C 関数: **scm\_string\_concatenate\_shared** (ls) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- scm_005fstring_005fconcatenate_005fshared)

`string-concatenate` と同様ですが、結果はリスト ls 内の文字列とメモリを共有する場合があります。

Scheme手順: **string-concatenate-reverse/shared** ls \[final\_string \[end\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dconcatenate_002dreverse_002fshared)

C 関数: **scm\_string\_concatenate\_reverse\_shared** (ls, final\_string, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fconcatenate_005freverse_005fshared)

`string-concatenate-reverse`と同様ですが、結果はls引数内の文字列とメモリを共有する場合があります。

* * *

次へ: [その他の文字列操作](https://doc.guix.gnu.org/guile/latest/en/guile.html#Miscellaneous-String-Operations)、前: [文字列の反転と追加](https://doc.guix.gnu.org/guile/latest/en/guile.html#Reversing-and-Appending-Strings)、上: [文字列](https://doc.guix.gnu.org/guile/latest/en/guile.html#Strings) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.5.11 マッピング、フォールディング、およびアンフォールディング [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Mapping_002c-Folding_002c-and-Unfolding)

Scheme プロシージャ: **string-map** proc s \[start \[end\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dmap)

C 関数: **scm\_string\_map** (proc, s, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fmap)

procはchar->char型のプロシージャであり、sにマッピングされます。プロシージャが文字列要素に適用される順序は指定されていません。

Scheme プロシージャ: **string-map!** proc s \[start \[end\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dmap_0021)

C 関数: **scm\_string\_map\_x** (proc, s, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fmap_005fx)

procはchar->char型のプロシージャであり、sに対してマップされます。プロシージャが文字列要素に適用される順序は指定されていません。文字列sはインプレースで変更され、戻り値は指定されていません。

Scheme プロシージャ: **string-for-each** proc s \[start \[end\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dfor_002deach)

C 関数: **scm\_string\_for\_each** (proc, s, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005ffor_005feach)

procは、sに対して左から右の順にマッピングされます。戻り値は指定されていません。

Scheme プロシージャ: **string-for-each-index** proc s \[start \[end\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dfor_002deach_002dindex)

C 関数: **scm\_string\_for\_each\_index** (proc, s, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005ffor_005feach_005findex)

s の各インデックス i に対して、左から右へ `(proc i)` を呼び出します。

例えば、文字を交互に大文字と小文字に変更するには、

(define str (string-copy "studly"))
(各インデックスの文字列)
(ラムダ (i)
(文字列セット! str i
((if (even? i) char-upcase char-downcase)
(文字列参照 str i))))
str)
str ⇒ "StUdLy"

Scheme Procedure: **string-fold** kons knil s \[start \[end\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dfold)

C 関数: **scm\_string\_fold** (kons, knil, s, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005ffold)

kons を、終端要素 knil をとして、左から右へ s の文字の上に折りたたみます。kons は、実際の文字と kons の適用結果の 2 つの引数を受け取る必要があります。

Scheme Procedure: **string-fold-right** kons knil s \[start \[end\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dfold_002dright)

C 関数: **scm\_string\_fold\_right** (kons, knil, s, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005ffold_005fright)

kons を、終端要素 knil をとして、右から左へ s の文字の上に折りたたみます。kons は、実際の文字と kons の適用結果の 2 つの引数を受け取る必要があります。

Scheme手順: **string-unfold** pfg seed \[base \[make\_final\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dunfold)

C 関数: **scm\_string\_unfold** (p, f, g, seed, base, make\_final) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005funfold)

* g は、初期シードから一連の _seed_ 値を生成するために使用されます: seed、(g シード)、(g^2 シード)、(g^3 シード)、…
* p は停止するタイミングを示します。つまり、これらのシード値のいずれかに適用したときに true が返されたときです。
* f は、各シード値を結果文字列内の対応する文字にマッピングします。これらの文字は、左から右の順序で文字列に組み立てられます。
* base は、構築される文字列のオプションの先頭/左端の部分です。デフォルトは空文字列です。
* make_final は、終端シード値 (p が true を返す値) に適用され、構築された文字列の最終部分 (右端部分) を生成します。デフォルトでは、追加の処理は行われません。

Scheme Procedure: **string-unfold-right** pfg seed \[base \[make\_final\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dunfold_002dright)

C 関数: **scm\_string\_unfold\_right** (p, f, g, seed, base, make\_final) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005funfold_005fright)

* g は、初期シードから一連の _seed_ 値を生成するために使用されます: seed、(g シード)、(g^2 シード)、(g^3 シード)、…
* p は停止するタイミングを示します。つまり、これらのシード値のいずれかに適用したときに true が返されたときです。
* f は、各シード値を結果文字列内の対応する文字にマッピングします。これらの文字は、右から左の順序で文字列に組み立てられます。
* base は、構築される文字列のオプションの先頭/右端の部分です。デフォルトは空文字列です。
* make_final は、ターミナル シード値 (p が true を返す値) に適用され、構築された文字列の最後の左端部分を生成します。デフォルトは `(lambda (x) )` です。

* * *

次へ: [文字列をバイトとして表現する](https://doc.guix.gnu.org/guile/latest/en/guile.html#Representing-Strings-as-Bytes)、前: [マッピング、折りたたみ、展開](https://doc.guix.gnu.org/guile/latest/en/guile.html#Mapping-Folding-and-Unfolding)、上: [文字列](https://doc.guix.gnu.org/guile/latest/en/guile.html#Strings) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.5.12 その他の文字列操作 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Miscellaneous-String-Operations-1)

Scheme Procedure: **xsubstring** s from \[to \[start \[end\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-xsubstring)

C 関数: **scm\_xsubstring** (s, from, to, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fxsubstring)

これは、ある文字列の部分文字列を複製コピーする処理を実装する拡張部分文字列処理手順です。

s は文字列です。start と end は、s の部分文字列を区切るオプションの引数で、デフォルト値は 0 と s の長さです。この部分文字列をインデックス空間の上下に、正負両方向に複製します。`xsubstring` は、この文字列のインデックス from から始まり、to で終わる部分文字列を返します。to のデフォルト値は from + (end - start) です。

Scheme 手順: **string-xcopy!** target tstart s sfrom \[sto \[start \[end\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dxcopy_0021)

C 関数: **scm\_string\_xcopy\_x** (target, tstart, s, sfrom, sto, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fxcopy_005fx)

`xsubstring` と全く同じですが、抽出されたテキストはインデックス tstart から始まる文字列 target に書き込まれます。`(eq? target s)` またはこれらの引数がストレージを共有している場合、この操作は定義されません。文字列をそれ自身にコピーすることはできません。

Scheme手順: **string-replace** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dreplace)

C 関数: **scm\_string\_replace** (s1, s2, start1, end1, start2, end2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005freplace)

文字列s1を返しますが、s2のstart1～end1の文字をstart2～end2の文字に置き換えます。

スキーム手順: **string-tokenize** s \[token\_set \[start \[end\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dtokenize)

C 関数: **scm\_string\_tokenize** (s, token\_set, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005ftokenize)

文字列 s を部分文字列のリストに分割します。各部分文字列は、文字セット token\_set (デフォルトは `char-set:graphic`) から抽出された、空でない連続した文字の最大シーケンスです。開始インデックスまたは終了インデックスが指定されている場合、`string-tokenize` は s の指定された部分文字列のみを処理するように制限されます。

Scheme手順: **string-filter** char\_pred s \[start \[end\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dfilter)

C 関数: **scm_string_filter** (char_pred, s, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005ffilter)

文字列 s をフィルタリングし、char_pred を満たす文字のみを残します。

char_pred がプロシージャの場合、各文字に対して述語として適用されます。文字の場合は、等価性がテストされ、文字セットの場合は、メンバーシップがテストされます。

Scheme プロシージャ: **string-delete** char\_pred s \[start \[end\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002ddelete)

C 関数: **scm\_string\_delete** (char\_pred, s, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- scm_005fstring_005fdelete)

s から char\_pred を満たす文字を削除します。

char_pred がプロシージャの場合、各文字に対して述語として適用されます。文字の場合は、等価性がテストされ、文字セットの場合は、メンバーシップがテストされます。

モジュール`(ice-9 string-fun)`には、以下の追加機能が利用可能です。これらは以下と組み合わせて使用できます。

(use-modules (ice-9 string-fun))

Scheme手順: **string-replace-substring** str 部分文字列置換 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dreplace_002dsubstring)

文字列str内の部分文字列のすべてのインスタンスが置換文字列に置き換えられた新しい文字列を返します。例：

([string-replace-substring](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dreplace_002dsubstring) "a ring of strings" "ring" "rut")
⇒ 「支柱の轍」

* * *

次へ: [C への変換/C からの変換](https://doc.guix.gnu.org/guile/latest/en/guile.html#Conversion-to_002ffrom-C)、前: [その他の文字列操作](https://doc.guix.gnu.org/guile/latest/en/guile.html#Miscellaneous-String-Operations)、上: [文字列](https://doc.guix.gnu.org/guile/latest/en/guile.html#Strings) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.5.13 文字列をバイトとして表現する[¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Representing-Strings-as-Bytes-1)

ガイルの世界の外にある冷徹な世界では、すべての文字列が同じように扱われるわけではない。そこにはバイトしか存在せず、文字列（文字の並び）をバイナリデータ（バイトの並び）として表現する方法は数多く存在する。

ユーザーとして、通常はこのことをあまり意識する必要はありません。キーボードで文字を入力すると、システムはコンピューターに設定されているロケールに従って、入力されたキーストロークをバイト列としてエンコードします。Guileはロケールを使用して、これらのバイト列を文字にデコードします。うまくいけば、入力した文字と同じ文字になります。

ウェブサーバーのような、複数のユーザーが利用するシステムを扱う場合、すべてが明確とは限りません。ウェブサーバーは、あるユーザーからISO-8859-1文字セットでエンコードされたデータの要求を受け取り、その後、別のユーザーからUTF-8データの要求を受け取る可能性があります。

Guile には、文字列とバイト列間の変換を行うための _iconv_ モジュールが用意されています。Guile が生のバイト列をどのように表現するかについては、[Bytevectors](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevectors) を参照してください。このモジュールの名前は、同名の一般的な UNIX コマンドに由来しています。

これらの関数を使用する代わりに、ポートから文字列を読み書きするだけで十分な場合が多いことに注意してください。これを行うには、`set-port-encoding!` を使用してポートのエンコーディングを指定します。ポートと文字エンコーディングの詳細については、[Ports](https://doc.guix.gnu.org/guile/latest/en/guile.html#Ports) を参照してください。

このセクションの他の手順とは異なり、これらの手順を実行するには、まず`iconv`モジュールをロードする必要があります。

(use-modules (ice-9 iconv))

Scheme手順: **string->bytevector** 文字列エンコーディング \[変換戦略\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002d_003ebytevector)

文字列をバイト列としてエンコードします。

文字列は、エンコード文字列で指定された文字セットでエンコードされます。文字列にエンコードで表現できない文字が含まれている場合、デフォルトではこのプロシージャは `encoding-error` を発生させます。別の動作を指定するには、変換戦略引数を渡してください。

戻り値はバイトベクトルです。バイトベクトルの詳細については、[Bytevectors](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevectors)を参照してください。文字エンコーディングと変換戦略の詳細については、[Ports](https://doc.guix.gnu.org/guile/latest/en/guile.html#Ports)を参照してください。

Scheme手順: **bytevector->string** bytevectorエンコーディング \[conversion-strategy\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002d_003estring)

バイトベクトルを文字列にデコードします。

バイト列は、エンコード文字列によって指定された文字セットからデコードされます。バイト列が有効なエンコードを構成しない場合、デフォルトではこの処理は `decoding-error` を発生させます。`string->bytevector` と同様に、オプションの変換戦略引数を渡すことで、この動作を変更できます。文字エンコードと変換戦略の詳細については、[Ports](https://doc.guix.gnu.org/guile/latest/en/guile.html#Ports) を参照してください。

Scheme Procedure: **call-with-output-encoded-string** encoding proc \[conversion-strategy\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-call_002dwith_002doutput_002dencoded_002dstring)

`call-with-output-string` と同様ですが、文字列を返す代わりに、指定されたエンコーディングに従って文字列をエンコードしたバイトベクトルを返します。この手順は、文字列を収集してから `string->bytevector` で変換するよりも効率的な場合があります。

* * *

次へ: [文字列の内部構造](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Internals)、前: [文字列をバイトとして表現する](https://doc.guix.gnu.org/guile/latest/en/guile.html#Representing-Strings-as-Bytes)、上: [文字列](https://doc.guix.gnu.org/guile/latest/en/guile.html#Strings) \[[目次]( https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.5.14 C言語への変換/C言語からの変換 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Conversion-to_002ffrom-C-1)

C言語の文字列からSchemeの文字列を作成する場合、またはSchemeの文字列をC言語の文字列に変換する場合、文字エンコーディングの概念が重要になります。

C言語では、文字列は単なるバイト列であり、文字エンコーディングはこれらのバイトと文字列を構成する実際の文字との関係を記述します。Schemeの文字列の場合、文字エンコーディングは（ほとんどの場合）問題になりません。なぜなら、Schemeでは通常、文字列をバイト列ではなく文字列として扱うからです。

C言語への変換とC言語からの変換には、それぞれ特有の課題がある。

C言語からScheme言語に変換する場合、C文字列のバイト列がそのエンコーディングに対して有効であることが重要です。例えば、ASCII文字列には127より大きいバイトを含めることはできません。127より大きいASCIIバイトは不正な形式とみなされ、Scheme文字に変換することはできません。

逆方向の操作でも問題が発生する可能性があります。すべての文字エンコーディングが、Schemeのすべての文字を格納できるわけではありません。例えばASCIIのようなエンコーディングは、すべての文字のごく一部しか表現できません。そのため、C言語に変換する際には、C文字列で表現できないScheme文字をどう処理するかをまず決める必要があります。

Scheme 文字列を C 文字列に変換すると、多くの場合、結果を保持するために新しいメモリが割り当てられます。このメモリが最終的に適切に解放されるように注意する必要があります。多くの場合、これは適切な dynwind コンテキスト内で `scm_dynwind_free` を使用することで実現できます。[Dynamic Wind](https://doc.guix.gnu.org/guile/latest/en/guile.html#Dynamic-Wind) を参照してください。

C 関数: `SCM` **scm\_from\_locale\_string** `(const char *str)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005flocale_005fstring)

C 関数: `SCM` **scm\_from\_locale\_stringn** `(const char *str, size_t len)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005flocale_005fstringn)

現在のロケールの文字エンコーディングで解釈した場合にstrと同じ内容を持つ新しいScheme文字列を作成します。

`scm_from_locale_string`の場合、strはヌル終端されている必要があります。

`scm_from_locale_stringn` の場合、len は str の長さをバイト単位で指定します。str はヌル終端されている必要はありません。len が `(size_t)-1` の場合は、str はヌル終端されている必要があり、実際の長さは `strlen` で取得されます。

C言語の文字列が不正な形式である場合、エラーが発生します。

これらの関数は、C言語の文字列定数の変換には使用しないでください。現在のロケールが、文字列定数や文字定数に使用される実行文字セットと一致する保証がないためです。最新のCコンパイラのほとんどはデフォルトでUTF-8を使用するため、C言語の文字列定数を変換するには`scm_from_utf8_string`を使用することをお勧めします。

C 関数: `SCM` **scm\_take\_locale\_string** `(char *str)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ftake_005flocale_005fstring)

C 関数: `SCM` **scm\_take\_locale\_stringn** `(char *str, size_t len)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ftake_005flocale_005fstringn)

`scm_from_locale_string` および `scm_from_locale_stringn` と同様ですが、最終的には `free` で str を解放します。したがって、Scheme 文字列を作成した直後に str を解放する場合にこの関数を使用できます。場合によっては、Guile は str を内部表現として直接使用できます。

C 関数: `char *` **scm\_to\_locale\_string** `(SCM str)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005flocale_005fstring)

C 関数: `char *` **scm\_to\_locale\_stringn** `(SCM str, size_t *lenp)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005flocale_005fstringn)

現在のロケールの文字エンコーディングで、str と同じ内容の C 文字列を返します。C 文字列は最終的に `free` で解放する必要があります。おそらく `scm_dynwind_free` を使用するでしょう。[Dynamic Wind](https://doc.guix.gnu.org/guile/latest/en/guile.html#Dynamic-Wind) を参照してください。

`scm_to_locale_string` の場合、返される文字列はヌル終端されており、str に `#\nul` 文字が含まれている場合はエラーが通知されます。

`scm_to_locale_stringn` かつ lenp が `NULL` でない場合、str には `#\nul` 文字が含まれる可能性があり、返される文字列のバイト単位の長さが `*lenp` に格納されます。この場合、返される文字列はヌル終端されません。lenp が `NULL` の場合、`scm_to_locale_stringn` は `scm_to_locale_string` と同じように動作します。

str 内の文字が現在のロケールの文字エンコーディングで表現できない場合、デフォルトのポート変換戦略が使用されます。変換戦略の詳細については、[Ports](https://doc.guix.gnu.org/guile/latest/en/guile.html#Ports) を参照してください。

変換戦略が「error」の場合はエラーが発生します。「substitute」の場合は、疑問符などの置換文字が挿入されます。「escape」の場合は、16進エスケープ文字が挿入されます。

C 関数: `size_t` **scm\_to\_locale\_stringbuf** `(SCM str, char *buf, size_t max_len)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005flocale_005fstringbuf)

str を現在のロケールエンコーディングの C 文字列として、buf が指すメモリに格納します。buf のバッファには max\_len バイトの容量があり、`scm_to_local_stringbuf` はそれ以上のバイトを格納することはありません。終端文字 `'\0'` は格納されません。

`scm_to_locale_stringbuf` の戻り値は、文字列 str 全体に必要なバイト数です。これは、バッファ buf が十分な大きさであったかどうかに関係なく示されます。したがって、戻り値が max\_len より大きい場合、格納されたのは max\_len バイトのみであり、より大きなバッファを使用して再度試行する必要があるでしょう。

ほとんどの場合、文字列変換は上記の関数のように現在のロケールを使用して行われます。しかし、ロケールの文字エンコーディングとは異なる文字エンコーディングから文字列を変換したい場合もあります。そのような場合のために、低レベル関数`scm_to_stringn`と`scm_from_stringn`が用意されています。ロケールを適切に使用していれば、これらの関数が必要になることはほとんどありません。

C 型: **scm\_t\_string\_failed\_conversion\_handler** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ft_005fstring_005ffailed_005fconversion_005fhandler)

これは列挙型であり、`SCM_FAILED_CONVERSION_ERROR`、`SCM_FAILED_CONVERSION_QUESTION_MARK`、および`SCM_FAILED_CONVERSION_ESCAPE_SEQUENCE`の3つの値のいずれかを取ることができます。これらは、指定された文字エンコーディングとの間で変換できない文字を処理する戦略を示すために使用されます。`SCM_FAILED_CONVERSION_ERROR`は、一部の文字が変換できない場合に、変換時にエラーをスローする必要があることを示します。`SCM_FAILED_CONVERSION_QUESTION_MARK`は、変換できない文字を疑問符文字に置き換える必要があることを示します。また、`SCM_FAILED_CONVERSION_ESCAPE_SEQUENCE`は、変換できない文字をエスケープシーケンスに置き換える必要があることを示します。

Scheme文字列をCに変換する際には3つの戦略すべてが適用されますが、C文字列をSchemeに変換する際には`SCM_FAILED_CONVERSION_ERROR`と`SCM_FAILED_CONVERSION_QUESTION_MARK`のみを使用できます。

C 関数: `char` **\*scm\_to\_stringn** `(SCM str, size_t *lenp, const char *encoding, scm_t_string_failed_conversion_handler handler)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002ascm_005fto_005fstringn)

この関数は、Guileの文字列strから新たに割り当てられたC文字列を返します。返される文字列の長さ（バイト単位）はlenpに格納されます。C文字列の文字エンコーディングは、ASCII形式のヌル終端C文字列エンコーディングとして渡されます。handlerパラメータは、エンコーディングに変換できない文字を処理するための戦略を指定します。

lenpが`NULL`の場合、この関数はヌル終端されたC言語の文字列を返します。文字列にヌル文字が含まれている場合はエラーが発生します。

この関数の Scheme インターフェースは、`ice-9 iconv` モジュールの `string->bytevector` です。[文字列をバイトとして表現する](https://doc.guix.gnu.org/guile/latest/en/guile.html#Representing-Strings-as-Bytes) を参照してください。

C 関数: `SCM` **scm\_from\_stringn** `(const char *str, size_t len, const char *encoding, scm_t_string_failed_conversion_handler handler)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005fstringn)

この関数は、C言語文字列strからスキーム文字列を返します。C言語文字列の長さ（バイト単位）はlenとして入力されます。C言語文字列のエンコーディングは、ASCII形式のヌル終端C言語文字列`encoding`として渡されます。ハンドラパラメータは、変換不可能な文字の処理方法を示します。

この関数の Scheme インターフェースは `bytevector->string` です。[文字列をバイトとして表現する](https://doc.guix.gnu.org/guile/latest/en/guile.html#Representing-Strings-as-Bytes) を参照してください。

以下の変換関数は、最も一般的に使用されるエンコーディングの利便性を考慮して提供されています。

C 関数: `SCM` **scm\_from\_latin1\_string** `(const char *str)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005flatin1_005fstring)

C 関数: `SCM` **scm\_from\_utf8\_string** `(const char *str)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- scm_005ffrom_005futf8_005fstring)

C 関数: `SCM` **scm\_from\_utf32\_string** `(const scm_t_wchar *str)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005futf32_005fstring)

ヌル終端されたC文字列strから、ISO-8859-1、UTF-8、またはUTF-32でエンコードされたScheme文字列を返します。これらの関数は、ハードコードされたC文字列定数をScheme文字列に変換するために使用されます。

C 関数: `SCM` **scm\_from\_latin1\_stringn** `(const char *str, size_t len)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005flatin1_005fstringn)

C 関数: `SCM` **scm\_from\_utf8\_stringn** `(const char *str, size_t len)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005futf8_005fstringn)

C 関数: `SCM` **scm\_from\_utf32\_stringn** `(const scm_t_wchar *str, size_t len)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005futf32_005fstringn)

C言語の文字列strから、ISO-8859-1、UTF-8、またはUTF-32でエンコードされた長さlenのスキーム文字列を返します。`scm_from_latin1_stringn`および`scm_from_utf8_stringn`の場合、lenはstrが指すバイト数です。`scm_from_utf32_stringn`の場合、lenはstr内の要素数（コードポイント数）です。

C 関数: `char` **\*scm\_to\_latin1\_stringn** `(SCM str, size_t *lenp)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002ascm_005fto_005flatin1_005fstringn)

C 関数: `char` **\*scm\_to\_utf8\_stringn** `(SCM str, size_t *lenp)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002ascm_005fto_005futf8_005fstringn)

C 関数: `scm_t_wchar` **\*scm\_to\_utf32\_stringn** `(SCM str, size_t *lenp)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002ascm_005fto_005futf32_005fstringn)

Scheme 文字列 str から、新しく割り当てられた ISO-8859-1、UTF-8、または UTF-32 エンコードされた C 文字列を返します。str を指定されたエンコードに変換できない場合はエラーがスローされます。lenp が NULL の場合、返される C 文字列はヌル終端され、それ以外の場合に C 文字列にヌル文字が含まれる場合はエラーがスローされます。lenp が NULL でない場合、文字列はヌル終端されず、返される文字列の長さが lenp に格納されます。返される長さは、`scm_to_latin1_stringn` および `scm_to_utf8_stringn` の場合はバイト数、`scm_to_utf32_stringn` の場合は要素数 (コードポイント数) です。

頻繁ではありませんが、ポートの実装の詳細を扱う場合、ポートのエンコードおよび変換戦略に従って文字列をエンコードおよびデコードする必要が生じる場合があります。そのような場合のために、便利な関数もいくつか用意されています。

C 関数: `SCM` **scm\_from\_port\_string** `(const char *str, SCM ポート)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005fport_005fstring)

C 関数: `SCM` **scm\_from\_port\_stringn** `(const char *str, size_t len, SCM port)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005fport_005fstringn)

C 関数: `char*` **scm\_to\_port\_string** `(SCM str, SCM port)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005fport_005fstring)

C 関数: `char*` **scm\_to\_port\_stringn** `(SCM str, size_t *lenp, SCM port)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005fport_005fstringn)

`scm_from_stringn` や類似の関数と同様だが、指定されたポートオブジェクトからエンコードおよび変換戦略を取得する点が異なる。

* * *

前へ: [C言語への変換/C言語からの変換](https://doc.guix.gnu.org/guile/latest/en/guile.html#Conversion-to_002ffrom-C)、上へ: [文字列](https://doc.guix.gnu.org/guile/latest/en/guile.html#Strings) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.5.15 文字列の内部構造 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Internals-1)

Guileは、各文字列をUnicodeコードポイントの連続配列と、それに関連付けられた属性セットとしてメモリに格納します。文字列のすべてのコードポイントが0から255までの整数値である場合、コードポイント配列はコードポイントごとに1バイトとして格納されます。つまり、ISO-8859-1（別名Latin-1）文字列として格納されます。文字列のいずれかのコードポイントが255より大きい整数値を持つ場合、コードポイント配列はコードポイントごとに4バイトとして格納されます。つまり、UTF-32文字列として格納されます。

1バイト/コードポイント表現と4バイト/コードポイント表現間の変換は、必要に応じて自動的に行われます。

文字列の内部表現を設定するためのAPIは提供されていませんが、それを照会するためのプロシージャが2つ用意されています。これらはデバッグ用のプロシージャです。Guileの文字列の内部表現の詳細はリリースごとに変更される可能性があるため、本番コードでの使用は推奨されません。

Scheme手順: **string-bytes-per-char** str [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dbytes_002dper_002dchar)

C 関数: **scm\_string\_bytes\_per\_char** (str) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fbytes_005fper_005fchar)

文字列str内のUnicodeコードポイントをエンコードするために使用されるバイト数を返します。結果は1または4です。

Scheme手順: **%string-dump** str [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0025string_002ddump)

C 関数: **scm\_sys\_string\_dump** (str) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsys_005fstring_005fdump)

str のデバッグ情報を含む連想リストを返します。連想リストには次のエントリが含まれます。

`string`

弦そのもの。

`開始`

文字列バッファ内の文字列の開始インデックス

`長さ`

紐の長さ

`共有`

この文字列が部分文字列である場合、その親文字列を返します。そうでない場合は、`#f` を返します。

読み取り専用

文字列が読み取り専用の場合は `#t` と表示されます。

`stringbuf-chars`

この文字列のstringbufの文字を含む新しい文字列

`stringbuf-length`

この文字列バッファの文字数

`stringbuf-shared`

この文字列バッファが共有されている場合は `#t` とします。

`stringbuf-wide`

この文字列バッファの文字が32ビットバッファに格納されている場合は`#t`、8ビットバッファに格納されている場合は`#f`と表示されます。

* * *

次へ: [キーワード](https://doc.guix.gnu.org/guile/latest/en/guile.html#Keywords)、前: [文字列](https://doc.guix.gnu.org/guile/latest/en/guile.html#Strings)、上: [データ型](https://doc.guix.gnu.org/guile/latest/en/guile.html#Data-Types) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
