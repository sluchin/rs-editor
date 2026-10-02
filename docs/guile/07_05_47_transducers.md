#### 7.5.47 トランスデューサ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Transducers)

Scheme言語で最もよく使われる操作のいくつかは、リストを変換する操作です。例えば、map、filter、takeなどです。これらはうまく機能し、よく理解されており、ほとんどのSchemeプログラマーが日常的に使用しています。しかし、これらはリストにしか作用しないため汎用的ではなく、N個の操作を組み合わせると`(-N 1)`個の中間リストが生成されるため、合成性もあまり高くありません。

トランスデューサは、どのようなプロセスで使用されるかを意識せず、中間的なコレクションを構築することなく構成可能です。つまり、すべての奇数を二乗するトランスデューサを作成できます。

(compose (tfilter odd?) (tmap (lambda (x) (\* xx))))

リストやベクトル、あるいはデータが一方向に流れるほぼあらゆる状況で再利用できます。非同期チャネルの処理ステップとして、イベントフレームワークの前処理ステップとして、あるいは遅延コレクションとトランスデューサを関数に渡して新しい遅延コレクションを取得するような遅延コンテキストでも使用できます。

コレクション固有の手続きを用いるという従来のSchemeのアプローチは変更されていません。代わりに、これらの手続きを補完する一般的な変換形式を規定します。その利点は明らかです。共通の変換を明確かつ分かりやすく記述できるため、コレクション固有の手続きを単に連結するよりも高速になります。特にGuileにおいては、GCパフォーマンスが大幅に向上します。

ただし、トランスデューサの初期化方法により、`(compose …)` はトランスデューサを左から右に合成することに注意してください。

* [SRFI-171 一般ディスカッション](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d171-General-Discussion)
* [トランスデューサの適用](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d171-Applying-Transducers)
* [Reducers](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d171-Reducers)
* [トランスデューサー](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d171-Transducers)
* [トランスデューサを書き込むためのヘルパー関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d171-Helpers)

* * *

次へ: [トランスデューサの適用](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d171-Applying-Transducers)、上へ: [トランスデューサ](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d171) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.5.47.1 SRFI-171 一般ディスカッション [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d171-General-Discussion-1)

#### リデューサーの概念 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-concept-of-reducers)

トランスデューサの中心的な部分は、3項削減手順である。

* 引数なし: リデューサーの識別子を生成します。
* (result-so-far): 完了。変換前または変換なしで `result-so-far` を返します。
* (result-so-far input) は、`result-so-far` と `input` を組み合わせて新しい `result-so-far` を生成します。

加算演算子 `+` を使用したリデューサーの場合、リデューサーは引数の順に `0`、`result-so-far`、`(+ result-so-far input)` を生成します。これは、通常の `+` 演算子が行う動作と全く同じです。

#### トランスデューサの概念 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-concept-of-transducers)

トランスデューサは、リデューサを受け取り、以下のように動作するリデューシング関数を生成する、引数1つの手続きです。

* 引数なし: 引数なしでリデューサーを呼び出す（その同一性を生成する）
* (result-so-far): これまでの結果を変換して、それを使ってリデューサーを呼び出すかもしれません。
* (result-so-far 入力) 入力に対して何らかの処理を行い、result-so-far と変換後の入力を引数としてリデューサーを呼び出す可能性があります。

簡単な例を挙げると次のようになります。

(list-transduce (tfilter odd?) + '(1 2 3 4 5))

これはまず、奇数要素のみをフィルタリングしたトランスデューサを返し、次に引数なしで`+`を実行してその識別子を取得します。その後、`(tfilter odd?)`によって返されたトランスデューサに`+`を渡して変換を開始し、還元関数を返します。これはSRFI 1のreduceと似たような動作をしますが、中間トランスデューサのいずれかが「還元済み」値（SRFI 9レコードとして実装）を返すかどうかもチェックします。これは、還元が早期に終了したことを意味します。

トランスデューサは合成され、最終的な還元は最後のステップでのみ実行されるため、合成されたトランスデューサは中間結果やコレクションを作成しません。合成関数の適用について考える通常の方法は右から左ですが、トランスダクションの構築方法により、左から右に適用されます。`(compose (tfilter odd?) (tmap sqrt))` は、最初に奇数値をフィルタリングし、次に残りの値の平方根を計算するトランスデューサを作成します。

#### 状態 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#State)

トランスデューサは`map`などの一般化のように見えるかもしれませんが、実際はそうではありません。トランスデューサはどのコンテキストで使用されているかを認識しないため、コレクション固有のトランスデューサとは異なり、状態を保持する必要があるトランスデューサもあります。状態を保持するトランスデューサは、隠された可変状態を使用して状態を保持するため、ミューテーション、並列処理、マルチショット継続に関するすべての注意点が適用されます。状態を保持する各トランスデューサは、ドキュメントでその旨が明確に説明されています。

#### 命名 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Naming)

トランスデューサモジュールからエクスポートされるリデューサは、SRFI-1の対応するものと同じ名前ですが、先頭に「r」が付きます。トランスデューサも同様の命名規則に従いますが、先頭に「t」が付きます。

* * *

次へ: [Reducers](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d171-Reducers)、前へ: [SRFI-171 一般ディスカッション](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d171-General-Discussion)、上へ: [トランスデューサー](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d171) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents)コンテンツ")\]\[[インデックス](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "インデックス")\]

#### 7.5.47.2 トランスデューサの適用 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Applying-Transducers)

Scheme Procedure: **list-transduce** xform f lst [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-list_002dtransduce)

Scheme Procedure: **list-transduce** xform f identity lst [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-list_002dtransduce-1)

トランスデューサ xform を初期化するには、リデューサ f を渡します。識別子が指定されていない場合、f は引数なしで実行され、リデューサの識別子を返します。その後、f はその識別子をシードとして使用して lst をリデュースします。

トランスデューサのいずれかが早期に終了した場合（`ttake`や`tdrop`など）、縮小値を返すことでその旨を通知します。Guileの実装では、縮小値は「reduced」という名前のSRFI 9レコード型でラップされた値です。トランスデューサがこのような値を返した場合、`list-transduce`は実行を停止し、直ちに縮小されていない値を返さなければなりません。

Scheme Procedure: **vector-transduce** xform f vec [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-vector_002dtransduce)

スキーム手順: **vector-transduce** xform f identity vec [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-vector_002dtransduce-1)

Scheme手順: **string-transduce** xform f str [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dtransduce)

Scheme手順: **string-transduce** xform f identity str [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dtransduce-1)

スキーム手順: **bytevector-u8-transduce** xform f bv [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002du8_002dtransduce)

Scheme Procedure: **bytevector-u8-transduce** xform f identity bv [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002du8_002dtransduce-1)

スキーム手順: **generator-transduce** xform f gen [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-generator_002dtransduce)

スキーム手順: **generator-transduce** xform f identity gen [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-generator_002dtransduce-1)

`list-transduce`と同様ですが、それぞれベクトル、文字列、u8バイトベクトル、SRFI-158形式のジェネレータに対応しています。

スキーム手順: **port-transduce** xform f reader [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-port_002dtransduce)

スキーム手順: **port-transduce** xform f リーダーポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-port_002dtransduce-1)

スキーム手順: **port-transduce** xform f identity reader port [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-port_002dtransduce-2)

`list-transduce` と同様ですが、ポート用です。ポートを指定せずに呼び出すと、`reader` を適用した結果に対して、EOF オブジェクトが返されるまで還元処理が行われます。これはおそらく `current-input-port` から読み込むためです。ポートを指定すると、引数なしの場合とは異なり、reader がポートに適用されます。識別子が指定されている場合は、それが還元処理の初期識別子として使用されます。

* * *

次へ: [トランスデューサ](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d171-Transducers)、前: [トランスデューサの適用](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d171-Applying-Transducers)、上: [トランスデューサ](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d171) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.5.47.3 リデューサー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Reducers)

Scheme Procedure: **rcons** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-rcons)

シンプルなconsingリデューサーです。値を指定せずに呼び出すと、自身の識別子である`'()`を返します。リストを1つ指定して呼び出すと、リストを反転します（`reverse!`を使用）。2つの値を指定して呼び出すと、2番目の値を1番目の値にconsingします。

(list-transduce (tmap (lambda (x) (+ x 1)) rcons (list 0 1 2 3))
⇒ (1 2 3 4)

Scheme Procedure: **reverse-rcons** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-reverse_002drcons)

rconsと同じですが、値の順序が逆になります。

(list-transduce (tmap (lambda (x) (+ x 1))) reverse-rcons (list 0 1 2 3))
⇒ (4 3 2 1)

Scheme Procedure: **rany** pred? [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-rany)

any のリデューサー バージョン。`(pred? value)` が #f 以外を返す場合、`(reduced (pred? value))` を返します。同一性は #f です。

(list-transduce (tmap (lambda (x) (+ x 1))) (rany odd?) (list 1 3 5))
⇒ #f

(list-transduce (tmap (lambda (x) (+ x 1))) (rany odd?) (list 1 3 4 5))
⇒ #t

スキーム手順: **revery** 予測? [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-revery)

すべてのリデューサー バージョン。いずれかの `(pred? value)` が #f を返す場合、変換を停止し、`(reduced #f)` を返します。すべての `(pred? value)` が true を返す場合、`(pred? value)` の最後の呼び出しの結果を返します。識別子は #t です。

(リスト変換)
(tmap (lambda (x) (+ x 1)))
(夢想 (ラムダ (v) (if (odd? v) v #f)))
（リスト2 4 6）
⇒ 7

(list-transduce (tmap (lambda (x) (+ x 1)) (revery odd?) (list 2 4 5 6))
⇒ #f

スキーム手順: **rcount** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-rcount)

シンプルな計数リデューサー。変換処理を通過する値をカウントします。

(list-transduce (tfilter odd?) rcount (list 1 2 3 4)) ⇒ 2.

* * *

次へ: [トランスデューサ書き込み用ヘルパー関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d171-Helpers)、前: [リデューサ](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d171-Reducers)、上: [トランスデューサ](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d171) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.5.47.4 トランスデューサ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Transducers-1)

Scheme手順: **tmap** proc [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-tmap)

すべての値にprocを適用するトランスデューサを返します。ステートレスです。

Scheme Procedure: **tfilter** pred? [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-tfilter)

pred? が #f を返す値を削除するトランスデューサーを返します。

ステートレス。

Scheme Procedure: **tremove** pred? [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-tremove)

pred? が非 #f を返す値を削除するトランスデューサーを返します。

ステートレス

Scheme プロシージャ: **tfilter-map** proc [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-tfilter_002dmap)

`(compose (tmap proc) (tfilter values))` と同じです。ステートレスです。

Scheme手順: **treplace**マッピング [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-treplace)

引数マッピングは、連想リスト（`equal?`を使用してキーを比較）、ハッシュテーブル、または1つの引数を受け取り、同じ引数または代替値を生成する1引数プロシージャです。

マッピングに渡された値が存在するかどうかをチェックするトランスデューサを返します。マッピングが見つかった場合は、そのマッピングの値が返され、見つからない場合は元の値が返されます。

内部状態は保持しませんが、treplace が使用中にマッピングを変更するとエラーになります。

スキーム手順: **tdrop** n [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-tdrop)

最初の n 個の値を破棄するトランスデューサを返します。

ステートフル。

スキーム手順: **ttake** n [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ttake)

最初の n 個の値を通過させた後、すべての値を破棄し、変換を停止するトランスデューサを返します。それ以降の値はすべて無視されます。

ステートフル。

Scheme Procedure: **tdrop-while** pred? [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-tdrop_002dwhile)

pred? が true を返す最初の値を破棄するトランスデューサーを返します。

ステートフル。

Scheme Procedure: **ttake-while** pred? [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ttake_002dwhile)

Scheme Procedure: **ttake-while** pred? retf [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ttake_002dwhile-1)

pred? が #f を返した後に変換を停止するトランスデューサを返します。以降の値はすべて無視され、最後に成功した値が返されます。retf は pred? が false を返すたびに呼び出される関数です。渡される引数は、これまでの結果と pred? が `#f` を返す入力です。デフォルトの関数は `(lambda (result input) result)` です。

ステートフル。

Scheme手順: **tconcatenate** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-tconcatenate)

tconcatenate は、各値 (リストである必要があります) の内容をリダクションに連結するトランスデューサです。

(list-transduce tconcatenate rcons '((1 2) (3 4 5) (6 (7 8) 9)))
⇒ (1 2 3 4 5 6 (7 8) 9)

スキームプロシージャ: **tappend-map** proc [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-tappend_002dmap)

`(compose (tmap proc) tconcatenate)` と同じです。

スキーム手順: **t flatten** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-t flatten)

tflatten は、リストで構成される入力を平坦化するトランスデューサです。

(list-transduce tflatten rcons '((1 2) 3 (4 (5 6) 7 8) 9)
⇒ (1 2 3 4 5 6 7 8 9)

スキーム手順: **tdelete-neighbor-duplicates** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-tdelete_002dneighbor_002dduplicates)

Scheme Procedure: **tdelete-neighbor-duplicates** equality-predicate [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-tdelete_002dneighbor_002dduplicates-1)

直後に続く重複要素を削除するトランスデューサを返します。デフォルトの等価性述語は `equal?` です。

ステートフル。

スキーム手順: **tdelete-duplicates** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-tdelete_002dduplicates)

Scheme Procedure: **tdelete-duplicates** equality-predicate [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-tdelete_002dduplicates-1)

等価述語を使用して比較された後続の重複要素を削除するトランスデューサを返します。デフォルトの等価述語は `equal?` です。

ステートフル。

スキーム手順: **tsegment** n [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-tsegment)

入力をn個の要素からなるリストにグループ化するトランスデューサを返します。変換処理が終了すると、要素数がn個未満であっても、残っているコレクションをすべて消去します。

ステートフル。

スキーム手順: **tpartition** pred? [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-tpartition)

`(pred? input)` の値が変化するたびに、入力をリストにグループ化するトランスデューサーを返します。

ステートフル。

スキーム手順: **tadd-between** 値 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-tadd_002dbetween)

各値と次の値の間に値を挿入するトランスデューサを返します。これは、`ttake` のようなトランスデューサとはうまく組み合わせることができません。なぜなら、変換処理が `value` で終了してしまう可能性があるからです。

ステートフル。

スキーム手順: **tenumerate** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-tenumerate)

Scheme手順: **tenumerate** 開始 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-tenumerate-1)

渡された値をインデックス付けするトランスデューサを返します。インデックスは、デフォルトで 0 に設定されている start から開始します。インデックス付けは、`(index . input)` のような cons ペアを使用して行います。

(list-transduce (tenumerate 1) rcons (list 'first 'second 'third))
⇒ ((1. 1番目) (2. 2番目) (3. 3番目))

ステートフル。

スキーム手順: **tlog** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-tlog)

スキーム手順: **tlog** ロガー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-tlog-1)

値や結果をログに記録したり出力したりするために使用できるトランスデューサを返します。ロガープロシージャの結果は破棄されます。デフォルトのロガーは `(lambda (result input) (write input) (newline))` です。

ステートレス。

#### Guile固有のトランスデューサ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guile_002dspecific-transducers)

これらのトランスデューサは`(srfi srfi-171 gnu)`ライブラリで利用可能であり、SRFI-171文書で説明されている標準規格の範囲外で提供されています。

Scheme手順: **tbatch**リデューサー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-tbatch)

スキーム手順: **tbatch** トランスデューサリデューサ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-tbatch-1)

リデューサーまたは`((トランスデューサー)リデューサー)`を使用して結果を蓄積し、最終的に縮小された値を返すバッチ処理トランスデューサー。これは、`tsegment`のようなものを一般化するために使用できます。

;; これは (tsegment 4) とまったく同じように動作します。
(list-transduce (tbatch (ttake 4) rcons) rcons (iota 10))
⇒ ((0 1 2 3) (4 5 6 7) (8 9))

Scheme手順: **tfold**リデューサー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-tfold)

Scheme 手順: **tfold** リデューサー シード [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-tfold-1)

`(リデューサーのシード値)` の結果を生成し、反復処理間でその結果を保存する折りたたみ式トランスデューサー。

(list-transduce (tfold +) rcons (iota 10))
⇒ (0 1 3 6 10 15 21 28 36 45)

* * *

前へ: [トランスデューサー](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d171-Transducers)、上へ: [トランスデューサー](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d171) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.5.47.5 トランスデューサを書き込むためのヘルパー関数 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Helper-functions-for-writing-transducers)

これらの関数は`(srfi srfi-171 meta)`モジュールに含まれており、独自のトランスデューサを作成する場合にのみ使用できます。

スキーム手順: **削減**値 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-reduced)

値を`<reduced>`コンテナで囲み、削減処理を停止する必要があることを示します。

スキーム手順: **削減された値？** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-reduced_003f)

値が `<reduced>` レコードの場合は #t を返します。

Scheme 手順: **unreduce** reduced-container [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-unreduce)

縮小コンテナ内の値を返します。

スキーム手順: **ensure-reduced** 値 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ensure_002dreduced)

値がまだ縮小されていない場合は、`<reduced>`コンテナで値をラップします。

Scheme 手順: **preserving-reduced** リデューサー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-preserving_002dreduced)

`reducer` を別のリデューサーでラップし、返された縮小値を別の縮小コンテナにカプセル化します。これは、`[collection]-reduce` でリデューサーを再利用する場合に便利です。リデューサーが縮小値を返すと、`[collection]-reduce` がそれをアンラップします。処理されない限り、縮小処理は継続されます。

Scheme Procedure: **list-reduce** f identity lst [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-list_002dreduce)

`list-transduce` が内部的に使用するリデューサー。f はトランスデューサーによって返されるリデューサーです。identity はリデュースの識別子（「シード」と呼ばれることもあります）。lst はリストです。f がリデュースされた値を返した場合、リデュースは直ちに停止し、リデュースされていない値が返されます。

Scheme Procedure: **vector-reduce** f identity vec [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-vector_002dreduce)

リストリデュースのベクトル版。

Scheme手順: **string-reduce** f identity str [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dreduce)

リスト削減の文字列版。

Scheme Procedure: **bytevector-u8-reduce** f identity bv [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002du8_002dreduce)

リストリデュースのバイトベクトルu8版。

スキーム手順: **port-reduce** f アイデンティティ リーダー ポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-port_002dreduce)

リストリデュースのポート版。リーダーを使用してポート上でリデュースを行い、リーダーがEOFオブジェクトを返すまで処理を続けます。

Scheme Procedure: **generator-reduce** f identity gen [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-generator_002dreduce)

リストリデュースのジェネレーター版。`gen` に対してリデュースを繰り返し、EOF オブジェクトを返します。

* * *

次へ: [SRFI-207 文字列表記バイトベクトル](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d207)、前: [トランスデューサ](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d171)、上: [SRFI サポート モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI-Support) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
