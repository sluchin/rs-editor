### 7.13 キュー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Queues-1)

このセクションの関数は以下によって提供されます

(use-modules (ice-9 q))

このモジュールは、任意のスキームオブジェクトを保持するキューを実装し、効率的な先入れ先出し（FIFO）操作を実現するように設計されています。

`make-q`はキューを作成し、オブジェクトは`enq!`と`deq!`で追加および削除されます。`q-push!`と`q-pop!`も使用でき、キューの先頭をスタックのように扱います。

  

Scheme手順: **make-q** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dq)

新しいキューを返します。

Scheme Procedure: **q?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-q_003f)

objがキューの場合は`#t`を返し、そうでない場合は`#f`を返します。

キューは独立したオブジェクトクラスではなく、consセルで実装されていることに注意してください。そのため、特定のリスト構造では`q?`から`#t`を取得できます。

Scheme Procedure: **enq!** q obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-enq_0021)

objをqの末尾に追加し、qを返す。

Scheme Procedure: **deq!** q [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-deq_0021)

スキーム手順: **q-pop!** q [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-q_002dpop_0021)

qから先頭の要素を削除して返します。qが空の場合は、`q-empty`例外がスローされます。

`deq!` と `q-pop!` は同じ操作です。この 2 つの名前は、アプリケーションが `enq!` と `deq!`、または `q-push!` と `q-pop!` を対応付けることができるようにするためのものです。

Scheme Procedure: **q-push!** q obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-q_002dpush_0021)

objをqの先頭に追加し、qを返す。

スキーム手順: **q-length** q [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-q_002dlength)

qに含まれる要素の数を返します。

スキーム手順: **q-empty?** q [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-q_002dempty_003f)

qが空の場合はtrueを返します。

スキーム手順: **q-empty-check** q [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-q_002dempty_002dcheck)

qが空の場合は、`q-empty`例外をスローします。

スキーム手順: **q-front** q [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-q_002dfront)

qの最初の要素を（削除せずに）返します。qが空の場合は、`q-empty`例外がスローされます。

スキーム手順: **q-rear** q [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-q_002drear)

q の最後の要素を（削除せずに）返します。q が空の場合は、`q-empty` 例外がスローされます。

Scheme Procedure: **q-remove!** q obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-q_002dremove_0021)

キューqからobjのすべての出現箇所を削除し、qを返します。objは`eq?`を使用してキューの要素と比較されます。

  

上記で説明した `q-empty` 例外は、`(throw 'q-empty)` と同様にスローされ、エラー スローのようなメッセージなどは表示されません。

キューはconsセルとして実装され、`car`にはキューに入れられた要素のリストが含まれ、`cdr`はそのリストの最後のセルになります（キューへの追加を容易にするため）。

(リスト.最後のセル)

キューが空の場合、list は空のリストとなり、last-cell は `#f` になります。

アプリケーションは、必要に応じてキューリストに直接アクセスできます。たとえば、要素を検索したり、特定の位置に挿入したりする場合などです。

Scheme Procedure: **sync-q!** q [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sync_002dq_0021)

q の最後のセルフィールドを再計算します。

上記の操作はすべて、説明どおりにlast-cellを維持するため、通常は`sync-q!`は必要ありません。ただし、アプリケーションがキューリストを変更する場合は、last-cellを同様に維持するか、`sync-q!`を呼び出して再計算する必要があります。

* * *

次へ: [Buffered Input](https://doc.guix.gnu.org/guile/latest/en/guile.html#Buffered-Input)、前: [Queues](https://doc.guix.gnu.org/guile/latest/en/guile.html#Queues)、上: [Guile Modules](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guile-Modules) \[[Contents](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[Index](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "Index")\]
