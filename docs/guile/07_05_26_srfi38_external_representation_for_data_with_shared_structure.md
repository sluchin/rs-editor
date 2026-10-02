#### 7.5.26 SRFI-38 - 共有構造を持つデータの外部表現

このサブセクションは、Ray Dillinger によって書かれた [SRFI-38 の仕様](http://srfi.schemers.org/srfi-38/srfi-38.html) に基づいています。

この SRFI は、`write-with-shared-structure` および `read-with-shared-structure` を使用して書き込まれたデータと読み込まれたデータの代替外部表現を作成します。これは、R5RS のセクション 7 で示されている `write` および `read` を使用して書き込まれたデータと読み込まれたデータの外部表現の文法と同一ですが、単一の生成規則が異なります。

<データ> --> <単純データ> | <複合データ>

以下の5つの作品に置き換えられます。

<データム> --> <データムの定義> | <非定義データム> | <定義されたデータム>
<定義データ> --> #<インデックス番号>=<非定義データ>
<定義されたデータ> --> #<インデックス番号>#
<非定義データ> --> <単純データ> | <複合データ>
<indexnum> --> <数字10>+

Scheme プロシージャ: **write-with-shared-structure** obj

Scheme 手順: **write-with-shared-structure** obj port

Scheme プロシージャ: **write-with-shared-structure** obj port optarg

指定されたポートに、obj の外部表現を書き込みます。書き込まれる表現に含まれる文字列は二重引用符で囲まれ、文字列内のバックスラッシュと二重引用符はバックスラッシュでエスケープされます。文字オブジェクトは `#\` 表記を使用して書き込まれます。

値ではなく位置を示すオブジェクト（R5RS スキームの cons セル、ベクトル、および長さがゼロでない文字列。また、Guile の構造体、バイトベクトル、ポート、ハッシュテーブル）が、書き込まれるデータ内で複数箇所に現れる場合、最初に書き込まれるときは '#N\=' が前に付き、それ以降は '#N#' に置き換えられます。ここで、N はその特定のオブジェクトを識別するために使用される自然数です。位置を示すオブジェクトが構造体内で一度しか出現しない場合、`write-with-shared-structure` は、それらのオブジェクトに対して `write` と同じ外部表現を生成する必要があります。

`write-with-shared-structure`は有限時間内に終了し、有限データを書き込む際に有限表現を生成します。

`write-with-shared-structure` は未指定の値を返します。port 引数は省略可能で、その場合は `(current-output-port)` が返す値がデフォルト値として使用されます。optarg 引数も省略可能です。optarg 引数が存在する場合、出力と戻り値への影響は未指定ですが、`write-with-shared-structure` は `read-with-shared-structure` で読み取れる表現を書き込む必要があります。実装によっては、optarg を使用して書式規則、数値基数、または戻り値を指定する場合があります。Guile の実装では optarg は無視されます。

例えば、コード

(begin (define a ([cons](06_06_08_pairs.md#668-ペア) 'val1 'val2))
([set-cdr!](06_06_08_pairs.md#668-ペア) aa)
([write-with-shared-structure](#7526-srfi-38---共有構造を持つデータの外部表現) a))

出力は `#1=(val1 . #1#)` となるはずです。これは、`cdr` が自身を含む cons セルを示しています。

Scheme手順: **read-with-shared-structure**

Scheme 手順: **read-with-shared-structure** ポート

`read-with-shared-structure` は、`write-with-shared-structure` によって生成された Scheme オブジェクトの外部表現を Scheme オブジェクトに変換します。つまり、これは、上記で定義された拡張外部表現文法における非終端記号 '<datum>' のパーサーです。`read-with-shared-structure` は、指定された入力ポートから解析可能な次のオブジェクトを返し、port を更新して、オブジェクトの外部表現の末尾の次の最初の文字を指すようにします。

入力で、オブジェクトを開始できる文字が見つかる前にファイルの終端が検出された場合、ファイルの終端オブジェクトが返されます。ポートは開いたままになり、(`read-with-shared-structure` または `read` による) 以降の読み取り試行もファイルの終端オブジェクトを返します。オブジェクトの外部表現の開始後にファイルの終端が検出されたものの、外部表現が不完全で解析できない場合は、エラーが通知されます。

ポート引数は省略可能で、その場合は`(current-input-port)`によって返される値がデフォルト値として使用されます。閉じているポートから読み取ろうとするとエラーになります。

* * *

次へ: [SRFI-41 - ストリーム](07_05_28_srfi41_streams.md#7528-srfi-41---ストリーム)、前: [SRFI-38 - 共有構造を持つデータの外部表現](#7526-srfi-38---共有構造を持つデータの外部表現)、上: [SRFI サポート モジュール](07_05_00_srfi_support_modules.md#75-srfi-サポート-モジュール) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
