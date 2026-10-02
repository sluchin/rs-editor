### 7.17 `sxml-match`: SXML のパターンマッチング [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#sxml_002dmatch_003a-Pattern-Matching-of-SXML)

`(sxml match)` モジュールは、`syntax-rules` および `syntax-case` マクロシステムのパターンマッチングを彷彿とさせる「例による」スタイルで、SXML ツリーのパターンマッチングのための構文形式を提供します。SXML の詳細については、[SXML](https://doc.guix.gnu.org/guile/latest/en/guile.html#SXML) を参照してください。

次の例[29](https://doc.guix.gnu.org/guile/latest/en/guile.html#FOOT29)は、音楽アルバムカタログ言語をHTMLに変換する簡単な例を示しています。

(define (album->html x)
([sxml-match](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sxml_002dmatch) x
((アルバム ([@](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0040) (タイトル ,t)) (カタログ (番号 ,n) (フォーマット ,f)) [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e))
\`(ul (li ,t)
(li (b ,n) (i ,f)) [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e))))

3つのマクロが提供されています：`sxml-match`、`sxml-match-let`、および`sxml-match-let*`。

標準的なS式パターンマッチング（[パターンマッチング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Pattern-Matching)を参照）と比較して、`sxml-match`には以下の利点があります。

* SXML要素のマッチングは、SXMLの正規化の程度に依存しません。
* SXML属性（要素内）のマッチングは順序が下がっています。パターン内で指定された属性の順序は、マッチング対象の要素の順序と一致する必要はありません。
* パターンで指定されたすべての属性は、一致させる要素に存在しなければなりません。XMLは「拡張可能」であるという考え方に基づき、一致させる要素には、パターンで指定されていない追加の属性が含まれる場合があります。

本モジュールはWebIt!の後継であり、インディアナ大学のエリック・ヒルズデール、ダン・フリードマン、ケント・ディブヴィグによって開発されたS式パターンマッチングに触発されたものである。

* [構文](https://doc.guix.gnu.org/guile/latest/en/guile.html#Syntax)
* [XML要素のマッチング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Matching-XML-Elements)
* [パターン内の楕円](https://doc.guix.gnu.org/guile/latest/en/guile.html#Ellipses-in-Patterns)
* [準引用符付き出力における省略記号](https://doc.guix.gnu.org/guile/latest/en/guile.html#Ellipses-in-Quasiquote_0027d-Output)
* [マッチングノードセット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Matching-Nodesets)
* [ノードセットの「残りの部分」のマッチング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Matching-the-_0060_0060Rest_0027_0027-of-a-Nodeset)
* [一致しない属性のマッチング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Matching-the-Unmatched-Attributes)
* [属性パターンのデフォルト値](https://doc.guix.gnu.org/guile/latest/en/guile.html#Default-Values-in-Attribute-Patterns)
* [パターンにおけるガード](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guards-in-Patterns)
* [カタモーフィズム](https://doc.guix.gnu.org/guile/latest/en/guile.html#Catamorphisms)
* [名前付きカタモルフィズム](https://doc.guix.gnu.org/guile/latest/en/guile.html#Named_002dCatamorphisms)
* [`sxml-match-let` および `sxml-match-let*`](https://doc.guix.gnu.org/guile/latest/en/guile.html#sxml_002dmatch_002dlet-and-sxml_002dmatch_002dlet_002a)

#### 構文 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Syntax)

`sxml-match`は、XMLノードのパターンマッチングに`case`のような形式を提供します。

Scheme構文: **sxml-match** input-expression clause1 clause2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sxml_002dmatch)

入力式（SXMLツリー）を、指定された句（1つ以上）に従って照合します。各句は、パターンと、パターンが一致した場合に評価される1つ以上の式で構成されます。オプションとして、`sxml-match`内の各句には、ガード式を含めることができます。

パターン表記は、Schemeの`syntax-rules`および`syntax-case`マクロシステムに基づいています。`sxml-match`構文の文法は以下のとおりです。

マッチフォーム ::= (sxml-match 入力式
節+）

句 ::= \[ノードパターン アクション式+\]
| \[ノードパターン（ガード式*）アクション式+\]

ノードパターン ::= リテラルパターン
| pat-var-or-cata
| 要素パターン
| リストパターン

リテラルパターン ::= 文字列
| キャラクター
| 番号
| #t
| #f

attr-list-pattern ::= (@ attribute-pattern\*)
| (@ 属性パターン\* . pat-var-or-cata)

属性パターン ::= (タグシンボル 属性値パターン)

属性値パターン ::= リテラルパターン
| pat-var-or-cata
| (pat-var-or-cata デフォルト値式)

要素パターン ::= (タグシンボル属性リストパターン?)
| (タグシンボル属性リストパターン?ノードセットパターン)
| (タグシンボル属性リストパターン?
ノードセットパターン? 。 pat-var-or-cata)

リストパターン ::= (リストノードセットパターン)
| (リストノードセットパターン? . pat-var-or-cata)
| (リスト)

ノードセットパターン ::= ノードパターン
| ノードパターン...
| ノードパターン ノードセットパターン
| ノードパターン ... ノードセットパターン

pat-var-or-cata ::= (var-symbol を引用解除)
| (unquote \[var-symbol\*\])
| (unquote \[cata-expression -> var-symbol\*\])

リストまたは要素本体パターン内では、省略記号は一度しか出現できませんが、その後に0個以上のノードパターンが続く場合があります。

ガード式は、カタモルフィズムの戻り値を参照することはできません。

出力式中の省略記号は、式の文脈内でのみ使用可能であり、構文形式では使用できません。

以下のセクションでは、`sxml-match` パターンマッチングの具体的な側面について説明します。

#### XML要素のマッチング [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Matching-XML-Elements)

以下の例は、XML要素のパターンマッチングを示しています。

([sxml-match](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sxml_002dmatch) '(e ([@](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0040) (i 1)) 3 4 5)
((e ([@](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0040) (i ,d)) ,a ,b ,c) ([list](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-list-1) dabc))
（、そうでなければ#f）

`sxml-match` の各句は、パターンと、パターンが正しく一致した場合に評価される 1 つ以上の式の 2 つの部分から構成されます。上記の例では、属性 `i` と 3 つの子要素を持つ要素 `e` に一致しています。

パターン変数はパターン内で「引用符なし」で記述する必要があります。上記の式では、d は `1`、a は `3`、b は `4`、c は `5` にバインドされます。

#### パターン内の楕円 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Ellipses-in-Patterns)

`syntax-rules`と同様に、省略記号（…）を使用して繰り返しパターンを指定できます。パターン`item ...`は、パターン`item`が0回以上一致することを指定することに注意してください。

パターンにおける省略記号の使用例は、以下のコード断片に示されています。ここでは、要素 `d` 内の `a` 要素の繰り返しインスタンスの子要素に一致させるために、入れ子になった省略記号が使用されています。

(define x '(d (a 1 2 3) (a 4 5) (a 6 7 8) (a 9 10)))

([sxml-match](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sxml_002dmatch) x
((d (a ,b [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e)) [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e))
([list](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-list-1) ([list](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-list-1) b [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e)) [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e))))

上記の式は、`((1 2 3) (4 5) (6 7 8) (9 10))` という値を返します。

#### 準引用符付き出力における省略記号 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Ellipses-in-Quasiquote_0027d-Output)

`sxml-match`フォームの本文内では、省略記号（…）を使用できる、少し拡張されたquasiquoteが提供されています。以下の例でその例を示します。

([sxml-match](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sxml_002dmatch) '(e 3 4 5 6 7)
((e ,i [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e) 6 7) \`("start" ,([list](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-list-1) 'wrap i) [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e) "end"))
（、そうでなければ#f）

一般的なパターンは、``(something ,i ...)` が ``(something ,@i)` に書き換えられることです。

#### ノードセットのマッチング [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Matching-Nodesets)

ノードセットパターンは、パターン内の識別子リストで始まるリストによって指定されます。以下の例は、ノードセットのマッチングを示しています。

([sxml-match](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sxml_002dmatch) '("i" "j" "k" "l" "m")
(([list](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-list-1) ,a ,b ,c ,d ,e)
\`((p ,a) (p ,b) (p ,c) (p ,d) (p ,e))))

この例では、各ノードセット項目をHTMLの段落要素で囲んでいます。この例は、省略記号（…）を使用することで、書き換えて簡略化できます。

([sxml-match](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sxml_002dmatch) '("i" "j" "k" "l" "m")
(([list](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-list-1) ,i [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e))
\`((p ,i) [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e))))

このバージョンでは、任意の長さのノードセットに一致し、ノードセット内の各項目をHTMLの段落要素で囲みます。

#### ノードセットの「残りの部分」のマッチング [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Matching-the-_0060_0060Rest_0027_0027-of-a-Nodeset)

ノードセットの「残りの部分」に一致させるには、要素またはノードセットパターンの末尾に`. rest)`パターンを使用します。

これは以下の例で示されています。

([sxml-match](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sxml_002dmatch) '(e 3 (f 4 5 6) 7)
((e ,a (f . ,y) ,d)
([list](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-list-1) ayd)))

上記の式は `(3 (4 5 6) 7)` を返します。

#### 一致しない属性のマッチング [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Matching-the-Unmatched-Attributes)

マッチング対象の要素に存在するものの、パターンには含まれていない属性のリストをバインドすると便利な場合があります。これは、属性リストパターンの最後に`.rest)`パターンを使用することで実現できます。以下の例でその例を示します。

([sxml-match](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sxml_002dmatch) '(a ([@](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- _0040) (z 1) (y 2) (x 3)) 4 5 6)
((a ([@](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0040) (y ,www) . ,qqq) ,t ,u ,v)
([list](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-list-1) www qqq tuv)))

上記の式は属性 `y` に一致し、残りの属性のリストを変数 qqq にバインドします。上記の式の結果は `(2 ((z 1) (x 3)) 4 5 6)` です。

このタイプのパターンでは、すべての属性をバインドすることも可能です。

([sxml-match](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sxml_002dmatch) '(a ([@](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0040) (z 1) (y 2) (x 3)))
((a ([@](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0040) . ,qqq))
qqq))

#### 属性パターンのデフォルト値 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Default-Values-in-Attribute-Patterns)

属性にデフォルト値を指定することが可能です。このデフォルト値は、一致対象の要素にその属性が存在しない場合に適用されます。以下の例でその例を示します。

([sxml-match](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sxml_002dmatch) '(e 3 4 5)
((e ([@](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0040) (z (,d 1))) ,a ,b ,c) ([list](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-list-1) dabc)))

属性「z」が要素「e」に存在しない場合、値「1」が使用されます。

#### パターンにおけるガード [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guards-in-Patterns)

ガードは、`guard` キーワードを使用してパターン句に追加できます。ガード式には、パターンが一致した場合にのみ評価される式を 0 個以上含めることができます。ガード式が `#t` と評価された場合にのみ、句の本体が評価されます。

ガード式の使用例を以下に示します。

([sxml-match](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sxml_002dmatch) '(a 2 3)
((a ,n) ([guard](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-guard) ([number?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-number_003f) n)) n)
((a ,m ,n) ([guard](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-guard) ([number?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-number_003f) m) ([number?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-number_003f) n)) ([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) mn)))

#### カタモーフィズム [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Catamorphisms)

以下の例は、`sxml-match` フォーム内で明示的な再帰を使用する方法を示しています。この例では、XML 要素 `plus`、`minus`、`times`、および `div` で表される基本的な算術演算を行うためのシンプルな計算機を実装しています。

(simple-eval を定義)
(ラムダ (x)
([sxml-match](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sxml_002dmatch) x
(,i ([guard](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-guard) ([integer?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-integer_003f) i)) i)
((plus ,x ,y) ([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) (simple-eval x) (simple-eval y)))
(([times](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-times) ,x ,y) ([\*](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002a) (simple-eval x) (simple-eval y)))
((マイナス ,x ,y) ([\-](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002d) (simple-eval x) (simple-eval y)))
(([div](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-div) ,x ,y) ([/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002f) (simple-eval x) (simple-eval y)))
(、そうでなければ ([error](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-error) "simple-eval: 無効な式" x)))))

`sxml-match` のカタモルフィズム機能を使用すると、`simple-eval` のより簡潔なバージョンを作成できます。パターン `,(x)` は、この位置にバインドされた値に対してパターンマッチャーを再帰的に呼び出します。

(simple-eval を定義)
(ラムダ (x)
([sxml-match](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sxml_002dmatch) x
(,i ([guard](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-guard) ([integer?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-integer_003f) i)) i)
((プラス、(x)、(y)) ([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) xy))
(([times](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-times) ,(x) ,(y)) ([\*](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002a) xy))
((マイナス ,(x) ,(y)) ([\-](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002d) xy))
(([div](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-div) ,(x) ,(y)) ([/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002f) xy))
(、そうでなければ ([error](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-error) "simple-eval: 無効な式" x)))))

#### 名前付きカタモルフィズム [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Named_002dCatamorphisms)

「cata」の位置で演算子を明示的に指定することも可能です。`,(id*)` は現在の `sxml-match` の先頭に再帰し、`,(cata -> id*)` は `cata` に再帰します。`cata` は、引数を 1 つ取り、`->` の後に続く識別子の数と同じ数の値を返すプロシージャに評価される必要があります。

名前付き変換パターンを使用すると、処理を複数の相互再帰的な手順に分割できます。以下の例は、テレビ番組表をHTML形式に変換する例です。

(define (tv-guide->html g)
(define (cast-list cl)
([sxml-match](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sxml_002dmatch) cl
((CastList (CastMember (Character (Name ,ch)) (Actor (Name ,a))) [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e))
\`([div](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-div) (ul (li ,ch ": " ,a) [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e)))))
(define (prog p)
([sxml-match](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sxml_002dmatch) p
((プログラム (開始 ,開始時刻) (期間 ,期間) (シリーズ ,シリーズタイトル)
(説明 ,desc [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e)))
\`([div](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-div) (p ,start-time
（br）シリーズタイトル
(br) ,desc [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e))))
((プログラム (開始 ,開始時刻) (期間 ,期間) (シリーズ ,シリーズタイトル)
(説明 ,desc [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e))
,(キャストリスト -> cl))
\`([div](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-div) (p ,start-time
（br）シリーズタイトル
(br) ,desc [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e))
、cl))))
([sxml-match](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sxml_002dmatch) g
((TVGuide ([@](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0040) (start ,start-date)
(終了、終了日)
(Channel (Name ,nm) ,(prog \-> p) [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e)) [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e))
`(html (head (title "TVガイド"))
(body (h1 "TVガイド"))
([div](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-div) (h2 ,nm) ,p [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e)) [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e))))))

#### `sxml-match-let` および `sxml-match-let*` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#sxml_002dmatch_002dlet-and-sxml_002dmatch_002dlet_002a)

Scheme構文: **sxml-match-let** ((pat expr) ...) expression0 expression ... [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sxml_002dmatch_002dlet)

Scheme構文: **sxml-match-let\*** ((pat expr) ...) expression0 expression ... [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sxml_002dmatch_002dlet_002a)

これらの形式は、Scheme の `let` および `let*` 形式を一般化し、単純な変数ではなく XML パターンをバインディング位置に使用できるようにします。

例えば、以下の式：

([sxml-match-let](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sxml_002dmatch_002dlet) (((a ,i ,j) '(a 1 2)))
([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) i [j](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-j)))

指定されたXML値内の変数iとjをそれぞれ`1`と`2`にバインドします。

* * *

次へ: [カリー定義](https://doc.guix.gnu.org/guile/latest/en/guile.html#Curried-Definitions)、前: [`sxml-match`: SXML のパターンマッチング](https://doc.guix.gnu.org/guile/latest/en/guile.html#sxml_002dmatch)、上: [Guile モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guile-Modules) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
