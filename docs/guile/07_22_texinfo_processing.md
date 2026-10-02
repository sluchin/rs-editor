### 7.22 Texinfo処理 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Texinfo-Processing-1)

* [(texinfo)](https://doc.guix.gnu.org/guile/latest/en/guile.html#texinfo)
* [(texinfo docbook)](https://doc.guix.gnu.org/guile/latest/en/guile.html#texinfo-docbook)
* [(texinfo html)](https://doc.guix.gnu.org/guile/latest/en/guile.html#texinfo-html)
* [(texinfo インデックス作成)](https://doc.guix.gnu.org/guile/latest/en/guile.html#texinfo-indexing)
* [(texinfo string-utils)](https://doc.guix.gnu.org/guile/latest/en/guile.html#texinfo-string_002dutils)
* [(texinfo plain-text)](https://doc.guix.gnu.org/guile/latest/en/guile.html#texinfo-plain_002dtext)
* [(texinfo serialize)](https://doc.guix.gnu.org/guile/latest/en/guile.html#texinfo-serialize)
* [(texinfo リフレクション)](https://doc.guix.gnu.org/guile/latest/en/guile.html#texinfo-reflection)

* * *

次へ: [(texinfo docbook)](https://doc.guix.gnu.org/guile/latest/en/guile.html#texinfo-docbook)、上へ: [Texinfo Processing](https://doc.guix.gnu.org/guile/latest/en/guile.html#Texinfo-Processing) \[[Contents](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[Index](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.22.1 (texinfo) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#g_t_0028texinfo_0029)

* [概要](https://doc.guix.gnu.org/guile/latest/en/guile.html#Overview-5)
* [使用方法](https://doc.guix.gnu.org/guile/latest/en/guile.html#Usage-5)

#### 7.22.1.1 概要 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Overview-5)

#### Scheme での texinfo 処理 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Texinfo-processing-in-scheme)

このモジュールは、texinfoをSXMLに解析します。もちろん、印刷出力にはTeXが常に最適なプロセッサです。しかし、`makeinfo`はinfoに対してはうまく機能しますが、他のフォーマットへの出力はあまりカスタマイズできず、プログラム全体としても拡張性に欠けます。このモジュールは、texinfoをSXML処理ツール群に統合する、texinfo処理のための拡張可能なフレームワークを提供することを目的としています。

#### SXML語彙に関する注記 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Notes-on-the-SXML-vocabulary)

次のtexinfoフラグメントを考えてみましょう。

@deffn プリミティブセットカー！ペア値
この機能...
@end defn

論理的には、カテゴリ（Primitive）、名前（set-car!）、引数（ペア値）はdeffnの「属性」であり、説明はその内容です。しかし、texinfoでは`@deffn`のように環境の引数内に@コマンドを使用できるため、texinfoの「属性」はPCDATAとなります。一方、XMLの属性はCDATAです。このため、texinfoの@コマンドの「属性」は「引数」と呼ばれ、特殊要素「%」の下にグループ化されます。

'%' は有効な NCName ではないため、stexinfo は SXML の上位互換です。相互運用性を確保するため、このモジュールは '%' を 'texinfo-arguments' に置き換える変換関数を提供します。

#### 7.22.1.2 使用法 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Usage-5)

関数: **call-with-file-and-dir** filename proc [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-call_002dwith_002dfile_002dand_002ddir)

引数1つのプロシージャprocを、filenameから読み込む入力ポートを指定して呼び出します。procの実行中は、現在のディレクトリは`(dirname filename)`となります。これは、相対パス名でファイルを含むドキュメントを解析する場合に便利です。

変数: **texi-command-specs** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-texi_002dcommand_002dspecs)

関数: **texi-command- Depth** コマンド max- Depth [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-texi_002dcommand_002d Depth)

指定されたtexinfoコマンドcommandに対して、そのネストレベルを返します。ネストがmax-depthを超えている場合は`#f`を返します。

例：

(texi-command-depth 'chapter 4) ⇒ 1
(texi-command-depth 'top 4) ⇒ 0
(texi-command-depth 'subsection 4) ⇒ 3
(texi-command- Depth '付録サブ秒 4) ⇒ 3
(texi-command-depth 'subsection 2) ⇒ #f

機能: **texi-fragment->stexi** 文字列またはポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-texi_002dfragment_002d_003estexi)

文字列またはポートで指定されたtexinfoコマンドを解析し、結果として得られるstexiツリーを返します。ツリーの先頭は特殊コマンド`*fragment*`になります。

関数: **texi->stexi** ポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-texi_002d_003estexi)

ポートから完全なtexinfoドキュメントを読み込み、解析済みのstexiツリーを返します。解析は`@settitle`から始まり、`@bye`またはEOFで終了します。

機能: **stexi->sxml** ツリー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-stexi_002d_003esxml)

stexi ツリーを sxml に変換します。これには、texinfo 引数を保持する `%` 要素を、各引数に対応する要素に置き換える作業が含まれます。

FIXME: 現状では単に % を `texinfo-arguments` に変更しているだけですが、これは将来的に DTD を作成するというアイデアとは合致しません。

* * *

次へ: [(texinfo html)](https://doc.guix.gnu.org/guile/latest/en/guile.html#texinfo-html)、前へ: [(texinfo)](https://doc.guix.gnu.org/guile/latest/en/guile.html#texinfo)、上へ: [Texinfo Processing](https://doc.guix.gnu.org/guile/latest/en/guile.html#Texinfo-Processing) \[[Contents](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[Index](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.22.2 (texinfo docbook) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#g_t_0028texinfo-docbook_0029)

* [概要](https://doc.guix.gnu.org/guile/latest/en/guile.html#Overview-6)
* [使用方法](https://doc.guix.gnu.org/guile/latest/en/guile.html#Usage-6)

#### 7.22.2.1 概要 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Overview-6)

このモジュールは、docbookのSXML表現のごく一部をstexiに変換する手順をエクスポートします。これは決して完全なものではありません。目的は、外部モジュールがdocbookの特定の部分（例えば、特定のツールによって生成された部分）を解析できるように、多数のルーチンとスタイルシートを収集することです。

#### 7.22.2.2 使用法 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Usage-6)

変数: **\*sdocbook->stexi-rules\*** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002asdocbook_002d_003estexi_002drules_002a)

変数: **\*sdocbook-block-commands\*** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002asdocbook_002dblock_002dcommands_002a)

機能: **sdocbook-flatten** sdocbook [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sdocbook_002dflatten)

sdocbook の断片を「フラット化」して、ブロック要素が互いにネストされないようにします。

Docbookはネスト構造になっており、例えば`refsect2`は通常`refsect1`の中に含まれます。ドキュメント内の論理的な区分はツリー構造で表現され、`refsect2`要素はそのセクション内のすべての要素を_含みます_。

それとは対照的に、texinfoはフラットなフォーマットであり、セクションは`@subsection`のような独立したセクションヘッダーによって区切られ、ブロック要素は互いにネストされません。

この関数はネストされたsdocbookフラグメントsdocbookを受け取り、すべてのセクションをフラット化します。

(refsect1 (refsect2 (パラ "Hello")))

になる

((refsect1) (refsect2) (パラ "Hello"))

多くの場合（常に？）、セクション要素の最初の子要素として `<title>` があります。`refsect*` 要素を `chapter` のような適切なセクション要素に処理することに関心のあるユーザーは、`replace-titles` と `filter-empty-elements` に関心を持つかもしれません。[replace-titles](https://doc.guix.gnu.org/guile/latest/en/guile.html#texinfo-docbook-replace_002dtitles) および [filter-empty-elements](https://doc.guix.gnu.org/guile/latest/en/guile.html#texinfo-docbook-filter_002dempty_002delements) を参照してください。

ノードセット、つまりタグなしのstexi要素のリストを返します。ノードセットの定義については、[SXPath](https://doc.guix.gnu.org/guile/latest/en/guile.html#SXPath)を参照してください。

機能: **filter-empty-elements** sdocbook [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-filter_002dempty_002delements)

sdocbookのノードセットから空の要素を除外します。主に`sdocbook-flatten`を実行した後に有効です。

機能: **replace-titles** sdocbook-fragment [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-replace_002dtitles)

sdocbookノードセットsdocbook-fragmentを反復処理し、連続する`refsect`要素と`title`要素を適切なtexinfoセクション分けコマンドに変換します。`sdocbook-flatten`を実行した後に行うと最も効果的です。

例えば：

(replace-titles '((refsect1) (タイトル "Foo") (パラ "Bar.")))
⇒ '((章「フー」) (パラ「バー」))

* * *

次へ: [(texinfo インデックス作成)](https://doc.guix.gnu.org/guile/latest/en/guile.html#texinfo-indexing)、前: [(texinfo ドキュメントブック)](https://doc.guix.gnu.org/guile/latest/en/guile.html#texinfo-docbook)、上: [Texinfo 処理](https://doc.guix.gnu.org/guile/latest/en/guile.html#Texinfo-Processing) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.22.3 (texinfo html) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#g_t_0028texinfo-html_0029)

* [概要](https://doc.guix.gnu.org/guile/latest/en/guile.html#Overview-7)
* [使用方法](https://doc.guix.gnu.org/guile/latest/en/guile.html#Usage-7)

#### 7.22.3.1 概要 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Overview-7)

このモジュールは、`stexi` から HTML への変換を実装します。`stexi->shtml` の出力は、実際には HTML 語彙を含む SXML であることに注意してください。つまり、出力はさらに処理することができ、最終的には `sxml->xml` によってシリアル化する必要があります。[XML の読み書き](https://doc.guix.gnu.org/guile/latest/en/guile.html#Reading-and-Writing-XML) を参照してください。

参照（つまり、`@ref` コマンド群）は、_ref-resolver_ によって解決されます。[add-ref-resolver!](https://doc.guix.gnu.org/guile/latest/en/guile.html#texinfo-html-add_002dref_002dresolver_0021) を参照してください。

#### 7.22.3.2 使用法 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Usage-7)

関数: **add-ref-resolver!** proc [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-add_002dref_002dresolver_0021)

ref-resolver のリストの先頭に proc を追加します。proc は、ノード名とマニュアル名を受け取り、参照先の URL を返すか、`#f` を指定してリスト内の次の ref-resolver に制御を渡すことが期待されます。

デフォルトのref-resolverは、手動で指定された名前、`#`、およびノード名を連結したものを返します。

機能: **stexi->shtml** ツリー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-stexi_002d_003eshtml)

StexiツリーをSHTMLに変換し、ref-resolverを使用して参照を解決します。詳細については、モジュールの解説を参照してください。

関数: **urlify** str [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-urlify)

* * *

次へ: [(texinfo string-utils)](https://doc.guix.gnu.org/guile/latest/en/guile.html#texinfo-string_002dutils)、前: [(texinfo html)](https://doc.guix.gnu.org/guile/latest/en/guile.html#texinfo-html)、上: [Texinfo Processing](https://doc.guix.gnu.org/guile/latest/en/guile.html#Texinfo-Processing) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.22.4 (texinfo インデックス作成) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#g_t_0028texinfo-indexing_0029)

* [概要](https://doc.guix.gnu.org/guile/latest/en/guile.html#Overview-8)
* [使用方法](https://doc.guix.gnu.org/guile/latest/en/guile.html#Usage-8)

#### 7.22.4.1 概要 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Overview-8)

与えられたステキシの断片に対して、指定された種類のインデックスを返す。

なお、現状では`stexi-extract-index`はインデックスエントリの種類を区別していません。これはバグです ;)

#### 7.22.4.2 使用法 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Usage-8)

機能: **stexi-extract-index** ツリー マニュアル名 種類 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-stexi_002dextract_002dindex)

stexi ツリー tree が与えられた場合、kind 型のエントリをすべてインデックス化します。kind は、定義済みの texinfo インデックス (`concept`、`variable`、`function`、`key`、`program`、`type`) のいずれか、または特殊記号 `auto` または `all` のいずれかです。`auto` は stext をスキャンして `(printindex)` ステートメントを探し、`all` は型に関係なくすべてのエントリからインデックスを生成します。

返されるインデックスはペアのリストであり、CARはエントリ（文字列）、CDRはノード名（文字列）です。

* * *

次へ: [(texinfo plain-text)](https://doc.guix.gnu.org/guile/latest/en/guile.html#texinfo-plain_002dtext)、前: [(texinfo indexing)](https://doc.guix.gnu.org/guile/latest/en/guile.html#texinfo-indexing)、上: [Texinfo Processing](https://doc.guix.gnu.org/guile/latest/en/guile.html#Texinfo-Processing) \[[Contents](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[Index](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.22.5 (texinfo string-utils) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#g_t_0028texinfo-string_002dutils_0029)

* [概要](https://doc.guix.gnu.org/guile/latest/en/guile.html#Overview-9)
* [使用方法](https://doc.guix.gnu.org/guile/latest/en/guile.html#Usage-9)

#### 7.22.5.1 概要 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Overview-9)

モジュール「(texinfo string-utils)」は、Guileのtexinfoサポートに役立つ様々な文字列関連関数を提供します。

#### 7.22.5.2 使用法 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Usage-9)

機能: **escape-special-chars** str special-chars escape-char [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-escape_002dspecial_002dchars)

指定されたエスケープ文字を先頭に、指定されたすべての特殊文字を含む文字列のコピーを返します。

特殊文字は、単一の文字でも、すべての特殊文字で構成される文字列でも構いません。

;; 文字列を正規表現で安全にする...
([escape-special-chars](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-escape_002dspecial_002dchars) "\*\*\*(例の文字列)\*\*\*"
"\[\]()/\*."
#\\\\)
[\=>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d_003e) "\\\\\*\\\\\*\\\\\*\\\\(例の文字列\\\\)\\\\\*\\\\\*\\\\\*"

;; 1文字だけエスケープすることもできます...
([escape-special-chars](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-escape_002dspecial_002dchars) "richardt@vzavenue.net"
#\\@
#\\@)
[\=>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d_003e) "richardt@@vzavenue.net"

機能: **transform-string** str 一致? \[start\] \[end\] を置換 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-transform_002dstring)

str の各文字に対して match? を使用し、一致する文字が見つかった場合は、その文字を置換します。

match? は、関数、文字、文字列、または `#t` のいずれかです。match? が関数の場合、入力として 1 文字を受け取り、一致する場合は '#t' を返します。match? が文字の場合、`char=?` を使用して各文字列文字と比較されます。match? が文字列の場合、その文字列内の任意の文字が一致とみなされます。`#t` を指定すると、すべての文字が一致とみなされます。

replaceが関数の場合、一致した文字を引数として関数が呼び出され、戻り値が'display'を介して出力文字列に渡されます。replaceがそれ以外の場合は、'display'を介して出力文字列に渡されます。

一致した文字の置換は、必ずしも単一の文字である必要はありません。これがこの関数を「string-map」と区別する点であり、Webページのテキストで「#\\&」を「&amp;」に変換するなどの用途に役立つ理由です。このモジュールに含まれる他の関数の中には、「transform-string」の一般的な使用方法をラップしただけのものもあります。この関数では不可能な変換は、正規表現を使用して行うべきでしょう。

startとendが指定されている場合、文字列のどの部分が変換されるかを制御します。ただし、入力文字列全体が出力されます。したがって、startが「5」の場合、strの最初の5文字が返される文字列に表示されます。

この2つは同等です。
([transform-string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-transform_002dstring) str #\\space #\\-) ; すべてのスペースをハイフンに変更します
([transform-string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-transform_002dstring) str (lambda (c) ([char=?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_003d_003f) #\\space c)) #\\-)

関数: **expand-tabs** str \[tab-size\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-expand_002dtabs)

すべてのタブをスペースに展開した文字列のコピーを返します。タブサイズのデフォルト値は8です。

タブサイズを8と仮定すると、これは以下と同等です。

([transform-string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-transform_002dstring) str #\\tab " ")

関数: **center-string** str \[width\] \[chr\] \[rchr\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-center_002dstring)

str のコピーを、幅文字のフィールドの中央に配置して返します。必要なパディングは文字 chr によって行われ、デフォルトは '#\\space' です。rchr が指定されている場合は、代わりに右側のパディングに rchr が使用されます。以下の例を参照してください。左側に rchr が、右側に rchr があります。デフォルトの幅は80 です。デフォルトの chr と rchr は '#\\space' です。文字列は切り捨てられません。

([center-string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-center_002dstring) "リチャード・トッド" 24)
[\=>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d_003e) "リチャード・トッド"

([center-string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-center_002dstring) " Richard Todd " 24 #\\=)
[\=>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d_003e) "===== Richard Todd ====="

([center-string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-center_002dstring) " Richard Todd " 24 #\\< #\\>)
[\=>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d_003e) "<<<<< リチャード・トッド >>>>>"

関数: **left-justify-string** str \[width\] \[chr\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-left_002djustify_002dstring)

`left-justify-string str [width chr]`。文字列strをchrで埋めて、幅の文字数で左揃えになるようにしたコピーを返します。デフォルトの幅は80です。srfi-13の'string-pad'とは異なり、文字列が切り詰められることはありません。

関数: **right-justify-string** str \[width\] \[chr\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-right_002djustify_002dstring)

文字列を文字で埋め、指定された幅のフィールド内で右寄せになるようにしたコピーを返します。デフォルトの幅は80です。デフォルトの文字は「#\\space」です。srfi-13の「string-pad」とは異なり、文字列が切り詰められることはありません。

関数: **collapse-repeated-chars** str \[chr\] \[num\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-collapse_002drepeated_002dchars)

文字列 str のコピーを返します。このコピーでは、文字列 chr の重複するすべてのインスタンスを最大 num 個に削減します。chr のデフォルト値は '#\\space'、num のデフォルト値は 1 です。

([collapse-repeated-chars](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-collapse_002drepeated_002dchars) "H ell o")
[\=>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d_003e) "こんにちは"
([collapse-repeated-chars](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-collapse_002drepeated_002dchars) "H--e--l--l--o" #\\-)
[\=>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d_003e) "こんにちは"
([collapse-repeated-chars](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-collapse_002drepeated_002dchars) "He--l---l----o" #\\- 2)
[\=>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d_003e) "彼--l--l--o"

機能: **make-text-wrapper** \[#:line-width\] \[#:expand-tabs?\] \[#:tab-width\] \[#:collapse-whitespace?\] \[#:subsequent-indent\] \[#:initial-indent\] \[#:break-long-words?\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dtext_002dwrapper)

指定されたパラメータに従って文字列を複数の行に分割するプロシージャを返します。

`#:line-width`

これは、行を折り返す位置を決定する際に使用される目標長さです。デフォルト値は80です。

`#:expand-tabs?`

入力内のタブを展開するかどうかを示すブール値。デフォルトは #t です。

`#:tab-width`

タブが展開された場合、展開されるスペースの数を指定します。デフォルト値は8です。

`#:collapse-whitespace?`

既存のテキスト内の空白文字を削除するかどうかを示すブール値。デフォルトは #t です。

テキストが既に適切にフォーマットされており、単に異なる幅に合わせて折り返される場合は、この設定を「#f」にしてください。こうすることで、元のテキストに共通する多くのテキストの慣例（文と文の間に2つのスペースを入れるなど）が保持されます。入力テキストのスペースが信頼できない場合は、この設定をデフォルトのままにしておくと、繰り返される空白はすべて1つのスペースにまとめられます。

`#:initial-indent`

折り返されたテキストの最初の行の前に挿入される文字列を定義します。デフォルトは空文字列「」です。

`#:subsequent-indent`

折り返されたテキストのすべての行の先頭に挿入される文字列を定義します（最初の行を除く）。デフォルトは空文字列「」です。

`#:break-long-words?`

単語が長すぎて1行に収まらない場合、この設定はラッパーに処理方法を指示します。デフォルトは#tで、長い単語を分割します。#fに設定すると、定義された`#:line-width`よりも長い行でも、行がそのまま表示されます。

戻り値は、入力文字列を引数として受け取るプロシージャであり、文字列のリストを返します。リストの各要素は1行です。

関数: **fill-string** str . kwargs [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-fill_002dstring)

文字列 str で指定されたテキストを、kwargs で指定されたパラメータに従ってラップします。パラメータが指定されていない場合は、デフォルト設定が適用されます。ラップされたテキストを含む単一の文字列を返します。有効なキーワード引数については、`make-text-wrapper` を参照してください。

関数: **string->wrapped-lines** str . kwargs [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002d_003ewrapped_002dlines)

`string->wrapped-lines str keywds ...`。文字列 str で指定されたテキストを、keywds で指定されたパラメータに従って折り返します。パラメータが指定されていない場合は、デフォルト設定が適用されます。折り返された行を表す文字列のリストを返します。有効なキーワード引数については、`make-text-wrapper` を参照してください。

* * *

次へ: [(texinfo serialize)](https://doc.guix.gnu.org/guile/latest/en/guile.html#texinfo-serialize)、前: [(texinfo string-utils)](https://doc.guix.gnu.org/guile/latest/en/guile.html#texinfo-string_002dutils)、上: [Texinfo Processing](https://doc.guix.gnu.org/guile/latest/en/guile.html#Texinfo-Processing) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.22.6 (texinfo プレーンテキスト) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#g_t_0028texinfo-plain_002dtext_0029)

* [概要](https://doc.guix.gnu.org/guile/latest/en/guile.html#Overview-10)
* [使用方法](https://doc.guix.gnu.org/guile/latest/en/guile.html#Usage-10)

#### 7.22.6.1 概要 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Overview-10)

stexi形式からプレーンテキストへの変換。`info`の出力を再現しようと試み、かなり近い結果を得ています。

#### 7.22.6.2 使用法 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Usage-10)

機能: **stexi->plain-text** ツリー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-stexi_002d_003eplain_002dtext )

ツリーをプレーンテキストに変換します。文字列を返します。

スキーム変数: **\*line-width\*** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002aline_002dwidth_002a)

この流体（[流体と動的状態](https://doc.guix.gnu.org/guile/latest/en/guile.html#Fluids-and-Dynamic-States)を参照）は、`stexi->plain-text`変換における行折り返しの目的で、行の長さを指定します。

* * *

次へ: [(texinfo reflection)](https://doc.guix.gnu.org/guile/latest/en/guile.html#texinfo-reflection), 前へ: [(texinfo plain-text)](https://doc.guix.gnu.org/guile/latest/en/guile.html#texinfo-plain_002dtext), 上へ: [Texinfo Processing](https://doc.guix.gnu.org/guile/latest/en/guile.html#Texinfo-Processing) \[[Contents](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[Index](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.22.7 (texinfo シリアル化) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#g_t_0028texinfo-serialize_0029)

* [概要](https://doc.guix.gnu.org/guile/latest/en/guile.html#Overview-11)
* [使用方法](https://doc.guix.gnu.org/guile/latest/en/guile.html#Usage-11)

#### 7.22.7.1 概要 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Overview-11)

`stexi`をプレーンなtexinfoにシリアライズします。

#### 7.22.7.2 使用法 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Usage-11)

機能: **stexi->texi** ツリー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-stexi_002d_003etexi)

Stexiツリーをプレーンなtexinfoにシリアル化する。

* * *

前へ: [(texinfo serialize)](https://doc.guix.gnu.org/guile/latest/en/guile.html#texinfo-serialize)、上へ: [Texinfo Processing](https://doc.guix.gnu.org/guile/latest/en/guile.html#Texinfo-Processing) \[[Contents](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[Index](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.22.8 (texinfo リフレクション) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#g_t_0028texinfo-reflection_0029)

* [概要](https://doc.guix.gnu.org/guile/latest/en/guile.html#Overview-12)
* [使用方法](https://doc.guix.gnu.org/guile/latest/en/guile.html#Usage-12)

#### 7.22.8.1 概要 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Overview-12)

オブジェクトとモジュールの`stexi`ドキュメントを生成するルーチン。

なお、この文脈における「オブジェクト」とは、単に場所に関連付けられた値のことです。GOOPSとは何の関係もありません。

#### 7.22.8.2 使用法 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Usage-12)

関数: **module-stexi-documentation** sym-name \[%docs-resolver\] \[#:docs-resolver\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-module_002dstexi_002ddocumentation)

sym-name という名前のモジュールのドキュメントを返します。ドキュメントは `stexi` 形式で出力されます ([texinfo](https://doc.guix.gnu.org/guile/latest/en/guile.html#texinfo) を参照)。

機能: **script-stexi-documentation** スクリプトパス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-script_002dstexi_002ddocumentation)

指定されたスクリプトのドキュメントを返します。ドキュメントはスクリプトのコメントから取得され、`stexi`形式で返されます（[texinfo](https://doc.guix.gnu.org/guile/latest/en/guile.html#texinfo)を参照）。

機能: **object-stexi-documentation** \_ \[\_\] \[#:force\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-object_002dstexi_002ddocumentation)

機能: **package-stexi-standard-copying** 名前 バージョン 更新年 著作権所有者 許可 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-package_002dstexi_002dstandard_002dcopying)

標準的なtexinfoの`copying`セクションを作成します。

yearsは、ドキュメント化対象のモジュールがリリースされた年（整数）のリストです。その他の引数はすべて文字列です。

機能: **package-stexi-standard-titlepage** 名前 バージョン 更新者 著者 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-package_002dstexi_002dstandard_002dtitlepage)

標準的なGNUタイトルページを作成します。

authorsは`(名前.メールアドレス)`のペアのリストです。その他の引数はすべて文字列です。

この手順の使用例を以下に示します。

(パッケージ-stexi-standard-タイトルページ
「フーリブ」
「3.2」
「2006年9月26日」
'(("Alyssa P Hacker" . "alyssa@example.com"))
（2004年 2005年 2006年）
「フリーソフトウェア財団」
「標準的なGPLライセンスの利用規約がここに記載されます」

機能: **package-stexi-generic-menu** 名前エントリ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-package_002dstexi_002dgeneric_002dmenu)

エントリの汎用的な alist からメニューを作成します。car はノード名、cdr は説明となります。例外として、エントリが `#f` の場合は区切り文字が生成されます。

機能: **package-stexi-standard-menu** 名前 モジュール モジュールの説明 追加エントリ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-package_002dstexi_002dstandard_002dmenu)

makeinfoによる処理に適した、標準的なトップノードとメニューを作成します。

機能: **package-stexi-extended-menu** 名前 モジュールペア スクリプトペア 追加エントリ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-package_002dstexi_002dextended_002dmenu)

標準メニューと同様の「拡張」メニューを作成します。ただし、スクリプト用のセクションを追加してください。

機能: **package-stexi-standard-prologue** 名前 ファイル名 カテゴリ 説明 コピー タイトルページ メニュー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-package_002dstexi_002dstandard_002dprologue)

後でtexinfoにシリアル化したり、makeinfoを使用して.infoファイルを作成したりするのに適した、標準的なプロローグを作成します。

`package-stexi-documentation` のプロローグとして渡すのに適した stexinfo フォームのリストを返します。[texinfo reflection package-stexi- documentation](https://doc.guix.gnu.org/guile/latest/en/guile.html#texinfo-reflection-package_002dstexi_002ddocumentation)、[package-stexi-standard-titlepage](https://doc.guix.gnu.org/guile/latest/en/guile.html#texinfo-reflection-package_002dstexi_002dstandard_002dtitlepage)、[package-stexi-standard-copying](https://doc.guix.gnu.org/guile/latest/en/guile.html#texinfo-reflection-package_002dstexi_002dstandard_002dcopying) を参照してください。 [package-stexi-standard-menu](https://doc.guix.gnu.org/guile/latest/en/guile.html#texinfo-reflection-package_002dstexi_002dstandard_002dmenu).

機能: **package-stexi-documentation** モジュール名 ファイル名 プロローグ エピローグ \[#:module-stexi-documentation-args\] \[#:scripts\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-package_002dstexi_002ddocumentation)

パッケージ（まとめてリリースされるモジュールの集合）のstexiドキュメントを作成します。

modulesにはモジュール名のリストが渡されることが想定されており、モジュール名は記号のリストです。返されるstexiには、nameというタイトルとfilenameというtexinfoファイル名が付けられます。

プロローグとエピローグは、それぞれ生成されたモジュールのドキュメントの前後に出力ドキュメントに挿入されるstexi形式のリストです。標準的なGNU texinfoプロローグを作成するには、[texinfo reflection package-stexi-standard-prologue](https://doc.guix.gnu.org/guile/latest/en/guile.html#texinfo-reflection-package_002dstexi_002dstandard_002dprologue)を参照してください。

module-stexi-documentation-args はオプションの引数で、指定すると `module-texi-documentation` が呼び出されたときの引数リストに追加されます。たとえば、`#:docs-resolver` 引数を定義すると便利です。

機能: **package-stexi-documentation-for-include** モジュール モジュールの説明 \[#:module-stexi-documentation-args\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-package_002dstexi_002ddocumentation_002dfor_002dinclude)

パッケージ（まとめてリリースされるモジュールの集合）のstexiドキュメントを作成します。

modulesにはモジュール名のリストが渡されることが想定されています。モジュール名はシンボルのリストです。stexinfoフラグメントを返します。

`package-stexi-documentation`とは異なり、この関数は完全なtexinfoドキュメントを生成するのではなく、メニューとモジュールドキュメントのみを生成します。マニュアルの一部を手書きで作成し、自動生成された部分を`@include`で読み込む場合に便利です。

module-stexi-documentation-args はオプションの引数で、指定すると `module-texi-documentation` が呼び出されたときの引数リストに追加されます。たとえば、`#:docs-resolver` 引数を定義すると便利です。

* * *

次へ: [Guile の実装](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guile-Implementation)、前へ: [Guile モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guile-Modules)、上へ: [Guile リファレンス マニュアル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Top) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
