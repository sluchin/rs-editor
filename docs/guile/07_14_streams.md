### 7.14 ストリーム

このセクションでは、Guile の従来のストリーム モジュールについて説明します。より完全で移植性の高いストリーム ライブラリについては、[SRFI-41 - Streams](07_05_28_srfi41_streams.md#7528-srfi-41---ストリーム) を参照してください。

ストリームは値のシーケンスを表し、各値は必要なときにのみ計算されます。これにより、大きなシーケンスや無限のシーケンスを、「car」、「cdr」、「map」、「fold」などの馴染みのある操作で表現および操作できます。このような操作では、必要な分だけが一度にメモリに保持されます。このセクションの関数は以下から入手できます。

(use-modules (ice-9 streams))

ストリームはプロミスを使用して実装されます（[遅延評価](06_16_reading_and_evaluating_scheme_code.md#61610-遅延評価)を参照）。これにより、基となる値の計算は必要なときにのみ行われ、計算が繰り返されないように値が保持されます。

以下は、すべて奇数のストリームを生成する簡単な例です。

(オッズを定義する (make-stream (lambda (state)
（状態（＋状態2）））
1))
（ストリームカーオッズ）⇒ 1
(stream-car (stream-cdr odds)) ⇒ 3

`stream-map` を使用すると、奇数個の正方形のストリームを生成できます。

(define (square n) (* nn))
(define oddsquares (stream-map square odds))

これらは無限シーケンスなのでリストに変換することはできませんが、例えば以下のように（無限に）出力することができます。

(stream-for-each (lambda (n sq)
(書式 #t "~a の二乗は ~a\\n" n の二乗です))
オッズ（オッズスクエア）
⊣
1の2乗は1
3の2乗は9です
5の2乗は25です
7の2乗は49です
...

  

Scheme 手順: **make-stream** proc initial-state

procを連続して呼び出すことによって形成された新しいストリームを返します。

各呼び出しは `(proc state)` であり、`car` がストリームの値、`cdr` が次の呼び出しの新しい状態となるペアを返す必要があります。最初の呼び出しでは、状態は指定された初期状態です。ストリームの最後に、proc はペアではないオブジェクトを返す必要があります。

スキーム手順: **stream-car** ストリーム

ストリームから最初の要素を返します。ストリームは空であってはなりません。

スキーム手順: **stream-cdr** ストリーム

ストリームの2番目以降の要素を返すストリーム。ストリームは空であってはなりません。

Scheme Procedure: **stream-null?** stream

ストリームが空の場合はtrueを返します。

Scheme Procedure: **list->stream** list

Scheme手順: **vector->stream** vector

リストまたはベクターの内容を含むストリームを返します。

リストやベクターは、その後変更してはなりません。なぜなら、そこで行われた変更が返されるストリームに反映されるかどうかは規定されていないからです。

Scheme Procedure: **port->stream** port readproc

readproc を使用してポートから読み取った値であるストリームを返します。各読み取り呼び出しは `(readproc port)` であり、入力の最後に EOF オブジェクト ([バイナリ I/O](06_12_input_and_output.md#6122-バイナリ入出力)) を返す必要があります。

例えば、ファイルからの文字ストリーム、

(ポート->ストリーム (入力ファイルを開く "/foo/bar.txt") 文字を読み込む)

Scheme Procedure: **stream->list** stream

ストリームの内容全体を表すリストを返します。

スキーム手順: **stream->reversed-list** ストリーム

ストリームの内容全体を逆順で格納したリストを返します。

Scheme Procedure: **stream->list&length** stream

2 つの値を返します ([複数の値の返と受け入れ](06_11_controlling_the_flow_of_program_execution.md#6117-複数の値の返却と受け入れ) を参照)。1 つ目はストリームの内容全体を表すリスト、2 つ目はそのリスト内の要素の数です。

スキーム手順: **stream->reversed-list&length** stream

2 つの値を返します ([複数の値の返却と受け入れ](06_11_controlling_the_flow_of_program_execution.md#6117-複数の値の返却と受け入れ) を参照)。1 つ目は、ストリームの内容全体を逆順にしたリスト、2 つ目は、そのリスト内の要素の数です。

Scheme手順: **stream->vector** stream

ストリームの内容全体を表すベクトルを返します。

関数: **stream-fold** proc init stream1 stream2 …

指定されたストリームの要素に対して、最短のストリームの末尾に到達するまで、最初から最後まで順にprocを適用します。最後に実行したprocの結果を返します。

各呼び出しは `(proc elem1 elem2 … prev)` の形式で行われ、各 elem は対応するストリームからのものです。prev は前の proc 呼び出しからの戻り値、または最初の呼び出しで指定された init です。

関数: **stream-for-each** proc stream1 stream2 …

指定されたストリームの要素に対してproc関数を呼び出します。戻り値は未定義です。

各呼び出しは `(proc elem1 elem2 …)` の形式で行われ、各要素は対応するストリームから取得されます。`stream-for-each` は最短のストリームの末尾に到達すると停止します。

関数: **stream-map** proc stream1 stream2 …

指定されたストリームの要素にprocを適用した結果である新しいストリームを返します。

各呼び出しは `(proc elem1 elem2 …)` の形式で行われ、各要素は対応するストリームから取得されます。新しいストリームは、指定された最短のストリームの末尾に達した時点で終了します。

* * *

次へ: [Expect](07_16_expect.md#716-expect)、前: [Streams](#714-ストリーム)、上: [Guile Modules](07_00_guile_modules.md#7つのguileモジュール) \[[Contents](00_contents.md "目次")\]\[[Index](index_r5rs.md "Index")\]
