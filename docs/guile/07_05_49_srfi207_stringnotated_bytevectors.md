#### 7.5.49 SRFI-207 文字列表記バイトベクトル

[SRFI-207](http://srfi.schemers.org/srfi-207/srfi-207.html) は、バイナリデータをより人間が理解しやすい形で表現するためのASCIIテキスト表記法を提供します。詳細は[Bytevectors](06_06_12_bytevectors.md#6612-バイトベクトル)を参照してください。また、整数、文字、文字列、その他のバイトベクトルのシーケンスからバイトベクトルを構築するためのバイト文字列指向の手順、およびバイトベクトルを文字列のように操作するための手順も提供します。

バイナリファイル形式は通常、自己記述型ではなく、自己記述型であっても、記述部分自体がバイナリ形式であるため、人間が解釈するのは困難です。この問題を解決するために、ファイルの先頭、あるいは場合によってはファイルの各セクションの先頭に、人間が読みやすいセクションを設けるのが一般的です。歴史的な経緯やテキストエンコーディングの複雑さを避けるため、この人間が読みやすいセクションは通常ASCIIテキストで表現されます。

例えば、ZIPファイルは16進数バイト「50 4B」で始まりますが、これはZIPフォーマットの発明者であるフィル・カッツのイニシャル「PK」のASCIIエンコーディングです。別の例として、GIF画像フォーマットは「47 49 46 38 39 61」で始まりますが、これは「GIF89a」のASCIIエンコーディングで、「89a」はフォーマットのバージョンです。3つ目の例はPNG画像フォーマットで、ファイルヘッダーは「89 50 4E 47」で始まります。最初のバイトは意図的に非ASCIIですが、次の3バイトは「PNG」です。さらに、PNGファイルはチャンクに分割され、各チャンクには4バイトの「チャンクタイプ」コードが含まれています。チャンクタイプの文字は、パレットを表す「PLTE」、デフォルトの背景色を表す「bKGD」、UTF-8で記述されたテキストを表す「iTXt」など、その用途を表すニーモニックです。

バイトベクトルにこのような文字列データが含まれる場合、人間プログラマーにとっては、`#u8(0x89 0x50 0x4E 0x47 0x0D 0x0A 0x1A 0x0A)` よりも `#u8"\x89;PNG\r\n\x1A;\n"` を扱う方がはるかに扱いやすい。

さらに、このSRFIは、バイトベクトルに対して、文字列に対して提供されるものとよく似た追加の手順を提供します。たとえば、バイトベクトルは、パディングやトリミング、大文字小文字を区別してまたは区別せずに比較、検索、結合、分割することができます。

このSRFIのほとんどのプロシージャは、他のバイトベクタープロシージャと区別するために`bytestring-`で始まります。これは、これらのプロシージャが別のバイトストリング型を受け入れたり返したりすることを意味するものではありません。バイトストリングとバイトベクターは全く同じ型です。

* [外部表記法](#75491-外部表記法)
* [コンストラクター](#75492-コンストラクタ)
* [変換](#75493-変換)
* [選択](#75494-選択)
* [置換](#75495-置換)
* [比較](#75496-比較)
* [検索中](#75497-検索)
* [結合と分割](#75498-結合と分割)
* [I/O](#75499-io)
* [例外](#754910-例外)
* [謝辞](#754911-謝辞)

* * *

次へ: [コンストラクタ](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d207-Contructors)、上: [SRFI-207 文字列表記バイトベクトル](#7549-srfi-207-文字列表記バイトベクトル) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.5.49.1 外部表記法

文字列表記のバイトベクターの基本形式は `#u8"CONTENT"` です。Scheme のリーダーは、`(read-enable 'bytestrings)` でバイト文字列が有効になっている場合にそれを読み取り、Scheme のライターは、`(print-enable 'bytestrings)` でバイト文字列が有効になっている場合にそれを書き込みます。

文字列表記のバイトベクター内で文字エンコーディングの問題を回避するため、文字列表記のバイトベクターの CONTENT 内では、印刷可能な ASCII 文字 (つまり、Unicode コードポイント U+0020 から U+007E までの範囲) のみを使用できます。その他の文字はすべて、ニーモニックまたはインラインの 16 進エスケープで表現する必要があり、`"` と `\` も通常の Scheme 文字列と同様にエスケープする必要があります。

文字列表記のバイトベクターの内容内:

* `\a` ⇒ 7
* `\b` ⇒ 8
* `\t` ⇒ 9
* `\n` ⇒ 10
* `\r` ⇒ 13
* `\"` ⇒ 34
* `\\` ⇒ 92
* `\|` ⇒ 124
* `\x` の後に 0 文字以上、1 または 2 桁の 16 進数、そして `;` が続くシーケンスは、16 進数で指定された整数を表します。
* `\` の後に 0 個以上の行内空白文字、改行、さらに 0 個以上の行内空白文字が続くシーケンスは無視され、結果として得られるバイトベクトルにはエントリがありません。
* その他の印刷可能なASCII文字は、ASCII/Unicodeコード表におけるその文字の文字番号を表します。
* 文字列表記のバイトベクトル内で、`\` で始まる他の文字またはシーケンスを使用するとエラーになります。

注: `\|` シーケンスは、文字列解析、シンボル解析、および文字列表記のバイトベクトル解析で同じシーケンスを使用できるようにするために提供されています。ただし、この SRFI では、文字列のネイティブ構文を継承するのではなく、有効な字句構文の完全な定義を提供しているため、`#u8"&iota;"` と `#u8"\xE000;"` は無効であることが明確です。

Scheme リーダーが文字列表記のバイトベクトルに遭遇すると、そのバイトベクトルが完全に書き出されたかのようにデータを生成します。つまり、`#u8"A"` は `#u8(65)` と完全に等価です。

* * *

次へ: [変換](#75493-変換)、前: [外部表記](#75491-外部表記法)、上: [SRFI-207 文字列表記バイトベクトル](#7549-srfi-207-文字列表記バイトベクトル) \[[目次](00_contents.md "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引"）\]

#### 7.5.49.2 コンストラクタ

Scheme手順: **バイト文字列**部分…

各部分を小さな整数のシーケンスに変換し、対応するバイトのバイトベクトルを以下のように返します。

* 部分値が0～255の範囲内の正確な整数である場合、結果に加算されます。
* 一部がASCII文字（つまり、コードポイントが0～127の範囲内）である場合、そのコードポイントに変換され、結果に追加されます。
* partがバイトベクトルの場合、その要素が結果に追加されます。
* 一部がASCII文字の文字列である場合、それはコードポイントのシーケンスに変換され、結果に追加されます。

それ以外の場合は、`bytestring-error?`を満たすエラーが通知されます。例:

(バイト列 "lo" #\\r #x65 #u8(#x6d)) ⇒ #u8"lorem"

(バイト文字列 "η" #\\space #u8(#x65 #x71 #x75 #x69 #x76))
⇒バイト文字列エラーが発生しました

Scheme手順: **make-bytestring** 部分

指定されたパーツが`bytestring`の引数として適切であれば、それらに`bytestring`を適用した結果得られるバイトベクトルを返します。そうでない場合は、`bytestring-error?`を満たすエラーが発生します。

Scheme手順: **make-bytestring!** バイトベクター (パーツ)

パーツが`bytestring`の適切な引数である場合、`make-bytestring`を呼び出した場合に得られるバイトベクターのバイトを、インデックスatから始まるバイトベクターに書き込みます。例：

(define bv (make-bytevector 10 #x20))
(make-bytestring! bv 2 '(#\\s #\\c "he" #u8(#x6d #x65))) bv)
⇒ #u8" スキーム "

* * *

次へ: [選択](#75494-選択)、前: [コンストラクタ](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d207-Contructors)、上: [SRFI-207 文字列表記バイトベクトル](#7549-srfi-207-文字列表記バイトベクトル) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.5.49.3 変換

Scheme手順: **bytevector->hex-string** bytevector

Scheme手順: **16進数文字列→バイトベクトル**文字列

バイトベクトルと、16進数のペアを含む文字列との間で変換を行います。文字列が16進数のペアでない場合、`bytestring-error?` を満たすエラーが発生します。

(バイトベクトル→16進数文字列 #u8"Ford") ⇒ "467f7264"
(16進数文字列→バイトベクトル "5a6170686f64") ⇒ #u8"Zaphod")

Scheme Procedure: **bytevector->base64** bytevector \[digits\]

Scheme手順: **base64->bytevecto** 文字列 \[digits\]

バイトベクトルとそのBase64エンコーディング（文字列）との間で変換を行います。64桁は、文字0～9、AZ、az、および記号+と/で表されます。ただし、Base64エンコーディングには、62桁目と63桁目の表現が異なるさまざまなバリアントが存在します。オプション引数digits（2文字の文字列）が指定された場合、その2文字が62桁目と63桁目として使用されます。詳細は[RFC 4648](https://tools.ietf.org/html/rfc4648)を参照してください。

文字列がbase64形式でない場合、`bytestring-error?`を満たすエラーが発生します。ただし、`char-whitespace?`を満たす文字は黙って無視されます。

(bytevector->base64 #u8(1 2 3 4 5 6)) ⇒ ⇒ "AQIDBAUG"
(bytevector->base64 #u8"Arthur Dent") ⇒ "QXJ0aHVyIERlbnQ="
(base64->bytevector "+/ /+") ⇒ #u8(#xfb #xff #xfe)

Scheme手順: **bytestring->list** bytevector \[start \[end\]\]

バイトベクトルの全部または一部を、同じ長さのリストに変換します。このリストには、32～127の範囲の要素には文字が、その他の要素には正確な整数が含まれます。</p>

(bytestring->list #u8(#x41 #x42 1 2) 1 3) ⇒ (#\\B 1)

Scheme手順: **make-bytestring-generator** arg …

呼び出されると、`bytestring` が引数に適用された場合に作成されるバイトベクターの連続バイトを返すジェネレーターを返します。ただし、バイトベクター自体は作成されません。バイトが生成される前に引数が検証されます。引数の形式が不正な場合は、`bytestring-error?` を満たすエラーが発生します。

(ジェネレーター->リスト (make-bytestring-generator "lorem"))
⇒ (#x6c #x6f #x72 #x65 #x6d)

* * *

次へ: [置換](#75495-置換)、前: [変換](#75493-変換)、上: [SRFI-207 文字列表記バイトベクトル](#7549-srfi-207-文字列表記バイトベクトル) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.5.49.4 選択

Scheme手順: **bytestring-pad** bytevector len char-or-u8

Scheme手順: **bytestring-pad-right** bytevector len char-or-u8

バイトベクターの内容に加えて、先頭または末尾に char-or-u8 (ASCII 文字または 0～255 の範囲の正確な整数) を含む十分な追加バイトを含む、新たに割り当てられたバイトベクターを返します。結果の長さは少なくとも len になります。

(bytestring-pad #u8"Zaphod" 10 #\\\_) ⇒ #u8"\_\_\_\_Zaphod"
(bytestring-pad-right #u8(#x80 #x7f) 8 0) ⇒ #u8(#x80 #x7f 0 0 0 0 0 0)

Scheme手順: **bytestring-trim** bytevector pred

Scheme 手順: **bytestring-trim-right** bytevector pred

Scheme 手順: **bytestring-trim-both** bytevector pred

bytevector の内容を含む、新しく割り当てられた bytevector を返します。ただし、pred を満たす先頭/末尾/先頭と末尾の両方の連続するバイトは含まれません。

(bytestring-trim #u8" Trillian" (lambda (b) (= b #x20)))
⇒ #u8"トリリアン"
(bytestring-trim-both #u8(0 0 #x80 #x7f 0 0 0) zero?) ⇒ #u8(#x80 #x7f)

* * *

次へ: [比較](#75496-比較)、前へ: [選択](#75494-選択)、上へ: [SRFI-207 文字列表記バイトベクトル](#7549-srfi-207-文字列表記バイトベクトル) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.5.49.5 置換

Scheme手順: **bytestring-replace** bytevector1 bytevector2 start1 end1 \[start2 end2\]

bytevector1 の内容を含む、新しく割り当てられた bytevector を返します。ただし、start1 と end1 でインデックス付けされたバイトは含まれず、代わりに bytevector2 の start2 と end2 でインデックス付けされたバイトが代入されます。

(bytestring-replace #u8"Vogon torture" #u8"poetry" 6 13)
⇒ #u8「ヴォゴン詩」

* * *

次へ: [検索中](#75497-検索)、前: [置換](#75495-置換)、上: [SRFI-207 文字列表記バイトベクトル](#7549-srfi-207-文字列表記バイトベクトル) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.5.49.6 比較

バイトベクトルの等価性を比較するには、`(rnrs bytevectors)` の `bytevector=?` プロシージャ ([Bytevectors](06_06_12_bytevectors.md#6612-バイトベクトル) を参照) または `equal?` を使用します。

Scheme手順: **bytestring<?** bytevector1 bytevector2

Scheme手順: **bytestring>?** bytevector1 bytevector2

Scheme手順: **bytestring<=?** bytevector1 bytevector2

Scheme手順: **bytestring>=?** bytevector1 bytevector2

bytevector1がbytevector2より小さい、大きい、以下、以上である場合に`#t`を返します。比較は辞書式順序で行われ、短いbytevectorが長いbytevectorより先に比較され、すべての要素が等しい場合に比較されます。

(bytestring<? #u8"Heart Of Gold" #u8"Heart of Gold") ⇒ #t
(バイト文字列<=? #u8(#x81 #x95) #u8(#x80 #xa0)) ⇒ #f
(バイト文字列>? #u8(1 2 3) #u8(1 2)) ⇒ #t

* * *

次へ: [結合と分割](#75498-結合と分割)、前: [比較](#75496-比較)、上: [SRFI-207 文字列表記バイトベクトル](#7549-srfi-207-文字列表記バイトベクトル) \[[目次](00_contents.md "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引"）\]

#### 7.5.49.7 検索

Scheme手順: **bytestring-index** bytevector pred \[start \[end\]\]

Scheme プロシージャ: **bytestring-index-right** bytevector pred \[start \[end\]\]

バイトベクターを先頭から末尾、または末尾から先頭の順に検索し、predを満たす最初のバイトを見つけ、そのバイトを含むバイトベクター内のインデックスを返します。どちらの方向でも、先頭は含まれ、末尾は含まれません。そのようなバイトが見つからない場合は、`#f`を返します。

(バイト文字列インデックス #u8(#x65 #x72 #x83 #x6f) (λ (b) (> b #x7f))) ⇒ 2
(バイト文字列インデックス #u8"Beeblebrox" (λ (b) (> b #x7f))) ⇒ #f
(bytestring-index-right #u8"Zaphod" は奇数か?) ⇒ 4

Scheme プロシージャ: **bytestring-break** bytevector pred

Scheme手順: **bytestring-span** bytevector pred

2 つの値を返します。1 つは、pred を満たさない/満たす文字の最大シーケンス (bytevector の先頭から末尾まで検索) を含む bytevector であり、もう 1 つは残りの文字を含む bytevector です。

(バイト文字列の区切り #u8(#x50 #x4b 0 0 #x1 #x5) ゼロ?)
⇒ #u8(#x50 #x4b) #u8(0 0 #x1 #x5)
(bytestring-span #u8"ABCDefg" (lambda (b) (and (> b 40) (< b 91))))
⇒ #u8"ABCD" #u8"efg"

* * *

次へ: [I/O](#75499-io)、前へ: [検索中](#75497-検索)、上へ: [SRFI-207 文字列表記バイトベクトル](#7549-srfi-207-文字列表記バイトベクトル) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.5.49.8 結合と分割

Scheme手順: **bytestring-join** バイトベクトルリスト区切り文字

`bytevector-list` 内のバイトベクターを、区切り文字を使用して結合します。区切り文字は、`bytestring` の引数として適切なものであれば何でも構いません。`grammar` 引数は、区切り文字の使用方法を決定するシンボルで、デフォルトは `infix` です。`grammar` に次の 4 つのシンボル以外を指定するとエラーになります。

`infix`

中置記法または区切り文字文法を意味します。リスト要素間に区切り文字を挿入します。空のリストは空のバイトベクトルを生成します。

`strict-infix`

リストが空でない場合は `infix` と同じ意味になりますが、空のリストが与えられた場合は `bytestring-error?` を満たすエラーを通知します。

`接尾辞`

接尾辞または終端文字の文法を意味します。リストの各要素の後に区切り文字を挿入します。

`prefix`

これは接頭辞文法を意味します。リストの各要素の前に区切り文字を挿入します。

例えば：

(bytestring-join '(#u8"Heart" #u8"of" #u8"Gold") #x20)
⇒ #u8「黄金の心」
(bytestring-join '(#u8(#xef #xbb) #u8(#xbf)) 0 'prefix)
⇒ #u8(0 #xef #xbb 0 #xbf)
(バイト文字列結合 '() 0 '厳密な中置)
⇒ ⇒ バイト文字列エラーが発生しました

Scheme手順: **bytestring-split** バイトベクトル区切り文字 \[grammar\]

バイトベクターの要素を分割し、区切り文字（ASCII文字または0～255の範囲の整数）を使用して新しく割り当てられたバイトベクターのリストを返します。区切り文字のバイトは結果のバイトベクターには含まれません。

grammar引数は、バイトベクターの分割方法を制御するために使用されます。デフォルト値と意味は`bytestring-join`と同じですが、`infix`と`strict-infix`は同じ意味になります。つまり、grammarが`prefix`または`suffix`の場合、それぞれバイトベクターの最初または最後の位置にある区切り文字は無視されます。

(バイト文字列分割 #u8"Beeblebrox" #x62)
⇒ (#u8"Bee" #u8"le" #u8"rox")
(bytestring-split #u8(1 0 2 0) 0 'suffix)
⇒ (#u8(1) #u8(2))

* * *

次へ: [例外](#754910-例外)、前: [結合と分割](#75498-結合と分割)、上: [SRFI-207 文字列表記バイトベクトル](#7549-srfi-207-文字列表記バイトベクトル) \[[目次](00_contents.md "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引"）\]

#### 7.5.49.9 I/O

Scheme Procedure: **read-textual-bytestring** prefix \[port\]

このSRFIで説明されている外部形式の文字列をポートから読み込み、バイトベクトルとして返します。prefix引数がfalseの場合、このプロシージャは「#u8」が既にポートから読み込まれているとみなします。ポートが省略された場合、デフォルト値として「(current-input-port)」の値が使用されます。読み込まれた文字が外部形式でない場合、「bytestring-error?」を満たすエラーが発生します。

(ポート番号付き呼び出し)
(open-input-string "#u8\\"AB\\\\xad;\\\\xf0;\\\\x0d;CD\\"")
(lambda (port) (read-textual-bytestring #t port)))
⇒ #u8(#x41 #x42 #xad #xf0 #x0d #x43 #x44)

Scheme プロシージャ: **write-textual-bytestring** bytevector \[port\]

この SRFI で説明されている外部フォーマットのバイトベクトルをポートに書き込みます。非グラフィック ASCII 文字を表すバイトはエンコードされません。その他のすべてのバイトは、可能な場合は単一の文字でエンコードされ、そうでない場合は `\x` エスケープでエンコードされます。ポートが省略された場合、デフォルト値は `(current-output-port)` になります。

(ポート番号付き呼び出し)
(open-output-string)
(ラムダ (ポート)
(テキストバイト文字列を書き込む)
#u8(#x9 #x41 #x72 #x74 #x68 #x75 #x72 #xa)
ポート）
(get-output-string port)))
⇒ 「#u8\\"\\\\tアーサー\\\\n\\"」

Scheme Procedure: **write-binary-bytestring** port arg …

各引数を、`bytestring` と同じ解釈でバイナリ出力ポート port に出力しますが、バイトベクトルは作成しません。引数は、ポートにバイトを書き込む前に検証されます。引数の形式が不正な場合は、`bytestring-error?` を満たすエラーが発生します。

(ポート番号付き呼び出し)
(open-output-bytevector)
(ラムダ (ポート)
(write-binary-bytestring port #\\Z #x61 #x70 "hod")
(get-output-bytevector port)))
⇒ #u8"ザフォド"

* * *

次へ: [謝辞](#754911-謝辞)、前: [I/O](#75499-io)、上: [SRFI-207 文字列表記バイトベクトル](#7549-srfi-207-文字列表記バイトベクトル) \[[目次](00_contents.md "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引"）\]

#### 7.5.49.10 例外

Scheme手順: **bytestring-error?** obj

objが以下のいずれかの手続きによって通知された`&bytestring-error`である場合、かつそれらが説明する状況下では、`#t`を返します。

* `バイト文字列`
* `16進数文字列→バイト文字列`
* `base64->bytestring`
* `make-bytestring`
* `make-bytestring!`
* `バイト文字列結合`
* `read-textual-bytestring`
* `write-binary-bytestring`
* `make-bytestring-generator`

* * *

前へ: [例外](#754910-例外)、上へ: [SRFI-207 文字列表記バイトベクトル](#7549-srfi-207-文字列表記バイトベクトル) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.5.49.11 謝辞

ダフネ・プレストン＝ケンダルはバイトベクトルの文字列表記法を考案し、ジョン・コーワンは手続きライブラリを作成し、ヴォルフガング・コーコラン＝マテは手続きのオリジナルのサンプル実装を作成した。

この表記法は、Python バージョン 2.6 以降で使用されている `bytes` オブジェクト用の表記法に着想を得ており、これは Scheme のバイトベクトル、特に R7RS におけるバイトベクトルと根本的に目的が類似しています。さらに、多くの手順は [SRFI 152](https://srfi.schemers.org/srfi-152/srfi-152.html) の手順と非常に類似しています。

SRFIメーリングリストの参加者の皆様にも感謝申し上げます。特に、Lassi Kortela氏は恥ずかしい技術的誤りを訂正してくださり、Marc Nieper-Wißkirchen氏は`write`プロシージャでこの表記法をデフォルトで使用すべきではない理由を説明してくださいました。

* * *

前へ: [SRFI-207 文字列表記バイトベクトル](#7549-srfi-207-文字列表記バイトベクトル)、上へ: [SRFI サポート モジュール](07_05_00_srfi_support_modules.md#75-srfi-サポート-モジュール) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
