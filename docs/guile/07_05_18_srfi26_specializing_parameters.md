#### 7.5.18 SRFI-26 - パラメータの特殊化 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d26-_002d-specializing-parameters)

この SRFI は、関数の選択されたパラメータを簡単に特殊化するための構文を提供します。これは、

(use-modules (srfi srfi-26))

ライブラリ構文: **cut** slot1 slot2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-cut)

ライブラリ構文: **cute** slot1 slot2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-cute)

指定された式に特化した選択されたパラメータを使用して、(slot1 slot2 …) を呼び出す新しいプロシージャを返します。

例を挙げて説明します。以下は、`write` の特殊化で、出力を `my-output-port` に送信します。

(cut write <> my-output-port)
⇒
(lambda (obj) (write obj my-output-port))

特殊記号 `<>` は、新しいプロシージャへの引数が配置されるスロットを示します。一方、`my-output-port` は評価されて渡される式であり、つまり `write` の動作を特殊化します。

`<>`

作成されたプロシージャからの引数が配置されるスロット。引数は、`cut` 形式で出現する順序で `<>` スロットに割り当てられます。引数の順序を変更することはできません。

`cut` の最初の引数は通常、プロシージャ (またはプロシージャを指定する式) ですが、`<>` も使用できます。たとえば、

(カット <> 1 2 3)
⇒
(ラムダ式 (proc) (proc 1 2 3))

`<...>`

新しい手続きから残りの引数をすべて格納するスロット。これは`cut`形式の末尾でのみ発生します。

例えば、`max` のように可変数の引数を取るが、さらに下限を強制する手続き、

(define my-lower-bound 123)

(最大値 my-lower-bound <...> を切り出す)
⇒
(lambda arglist (apply max my-lower-bound arglist))

`cut` の場合、特殊化式は新しいプロシージャが呼び出されるたびに評価されます。`cute` の場合、特殊化式は新しいプロシージャが作成されるときに一度だけ評価されます。`cute` という名前は「評価された引数を持つ `cut`」を意味します。いずれの場合も、評価の順序は指定されていません。

以下は`cut`と`cute`の違いを示しています。

（カットフォーマット <> "時刻は~s"（現在時刻））
⇒
(lambda (port) (format port "時刻は~sです" (current-time)))

（かわいい形式 <> "時刻は ~s" (現在時刻)）
⇒
(let ((val (current-time)))
(lambda (port) (format port "時刻は~s" val))

（`cut`と`cute`を混在させて、一部の式は毎回評価され、他の式は一度だけ評価されるようなケースは想定されていません。）

`cut` は、上記の例で示したような `lambda` 形式の省略形にすぎません。ただし、`cut` を使うと、特殊化されていないパラメータに名前を付ける必要がなくなり、より簡潔になります。関数型プログラミングスタイルでの使用や、`map`、`for-each` などとの組み合わせが一般的です。

(map (cut \* 2 <>) '(1 2 3 4))

(for-each (cut write <> my-port) my-list)

* * *

次へ: [SRFI-28 - 基本フォーマット文字列](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d28)、前: [SRFI-26 - パラメータの特殊化](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d26)、上: [SRFI サポート モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI-Support) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
