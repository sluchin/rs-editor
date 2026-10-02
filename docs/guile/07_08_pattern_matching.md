### 7.8 パターンマッチング [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Pattern-Matching-1)

`(ice-9 match)`モジュールは、Alex Shinnによって作成されたパターンマッチング機能を提供し、多くのScheme実装で使用されているAndrew K. Wrightのパターンマッチング機能と互換性があります。

パターンマッチング機能は、オブジェクトを複数のパターンと照合し、そのオブジェクトを構成する要素を抽出します。パターンは、リスト、文字列、シンボル、レコードなど、あらゆるSchemeオブジェクトを表すことができます。また、パターン変数を含めることも可能です。一致するパターンが見つかると、そのパターンに関連付けられた式が評価されます。この際、必要に応じて、すべてのパターン変数がオブジェクトの対応する要素にバインドされます。

(let ((l '(hello (world))))
(マッチ l ;; <- 入力オブジェクト
(('こんにちは(誰)) ;; <- パターン
誰))) ;; <- 一致時に評価される式
⇒世界

この例では、リスト l はパターン `('hello (who))` に一致します。これは、リスト l が 2 つの要素を持つリストであり、最初の要素がシンボル `hello` で、2 番目の要素が 1 つの要素を持つリストであるためです。ここで who はパターン変数です。パターンマッチング関数 `match` は、この 1 つの要素を持つリストに含まれる値、つまりシンボル `world` に who をローカルにバインドします。l がパターンに一致しない場合は、エラーが発生します。

同じオブジェクトを、より単純なパターンと照合することもできます。

(let ((l '(hello (world))))
(マッチl)
((xy)
(値 xy))))
⇒こんにちは
⇒ （世界）

ここでパターン`(xy)`は、要素の型に関係なく、任意の2要素リストに一致します。パターン変数xとyは、それぞれリストlの最初の要素と2番目の要素にバインドされます。

パターンは組み合わせたり、ネストしたりできます。たとえば、`...`（省略記号）は、前のパターンがリスト内で0回以上一致する可能性があることを意味します。

(マッチリスト)
（（（表裏…）…）
頭))

この式は、lst 内の各リストの最初の要素を返します。適切なリストの適切なリストの場合、これは `(map car lst)` と同等です。ただし、パターンで規定されているように、lst およびその中のリストが適切なリストであることを確認するための追加チェックを実行し、そうでない場合はエラーを発生させます。

手書きのコードと比較すると、パターンマッチングは明瞭さと簡潔さを著しく向上させます。例えば、リストをマッチングする際に、`car`や`cdr`を何度も呼び出す必要はありません。また、入力がパターンに完全に一致することを保証することで、堅牢性も向上します。一方、手書きのコードは、簡潔さを優先するあまり、堅牢性を犠牲にしていることがよくあります。そしてもちろん、`match`はマクロであり、展開後のコードは同等の手書きコードと全く同じ効率性を備えています。

パターンマッチングは次のように定義されます。

Scheme構文: **match** exp clause1 clause2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-match)

オブジェクト exp を、clause1、clause2…のパターンと、出現順に照合します。最初に一致した句によって生成された値を返します。一致する句がない場合は、キー `match-error` を持つ例外をスローします。

各節は `(パターン body1 body2 …)` の形式をとります。各パターンは、以下に説明する構文に従う必要があります。各ボディは任意の Scheme 式であり、パターンの変数を参照する場合があります。

パターンの構文と解釈は以下のとおりです。

パターン: 一致:

pat ::= identifier anything、識別子をバインドします
| \_ 何でも
| () 空のリスト
| #t #t
| #f #f
| 文字列を文字列として
| 番号を数える
| キャラクター キャラクター
| 'sexp は S 式です
| 'シンボル シンボル（s-expr の特殊なケース）
| (pat\_1 ... pat\_n) n個の要素のリスト
| (pat\_1 ... pat\_n . pat\_{n+1}) n 個以上のリスト
| (pat\_1 ... pat\_n pat\_n+1 ooo) n 個以上のリスト、各要素
残りの部分はpat\_n+1と一致する必要があります
| #(pat\_1 ... pat\_n) n個の要素からなるベクトル
| #(pat\_1 ... pat\_n pat\_n+1 ooo) n 以上のベクトル、各要素
残りの部分はpat\_n+1と一致する必要があります
| #&pat box
| ($ レコード名 pat\_1 ... pat\_n) レコード
| (= フィールド pat) オブジェクトの \`\`フィールド
| (そして pat\_1 ... pat\_n) pat\_1 から pat\_n まで全て一致する場合
| (または pat\_1 ... pat\_n) pat\_1 から pat\_n のいずれかが一致する場合
| (pat\_1 ... pat\_n 以外) pat\_1 から pat\_n まで全て一致しない場合
| (? 述語 pat\_1 ... pat\_n) 述語が真で、すべての
pat_1 から pat_n まで一致
| (set! 識別子) 何でも、そしてセッターをバインドします
| (get! 識別子) 何でも、そしてゲッターをバインドします
| \`qp 準パターン
| (識別子 \*\*\* pat) はツリー内の pat に一致し、バインドします
パスへの識別子
パターンに一致するオブジェクトへ

ooo ::= ... 0 個以上
| \_\_\_ ゼロ以上
| ..1 1 個以上

準パターン: 一致:

qp ::= () 空のリスト
| #t #t
| #f #f
| 文字列を文字列として
| 番号を数える
| キャラクター キャラクター
| 識別子 シンボル
| (qp\_1 ... qp\_n) n個の要素のリスト
| (qp\_1 ... qp\_n . qp\_{n+1}) n 個以上のリスト
| (qp\_1 ... qp\_n qp\_n+1 ooo) n 個以上のリスト、各要素
残りの部分はqp\_n+1と一致する必要があります
| #(qp\_1 ... qp\_n) n個の要素からなるベクトル
| #(qp\_1 ... qp\_n qp\_n+1 ooo) n 以上のベクトル、各要素
残りの部分はqp\_n+1と一致する必要があります
| #&qp ボックス
パターンをなぞる
| 、@pat パターン

`quote`、`quasiquote`、`unquote`、`unquote-splicing`、`?`、`_`、`$`、`and`、`or`、`not`、`set!`、`get!`、`...`、および`___`は、パターン変数として使用できません。

より複雑な例を挙げましょう。

(use-modules (srfi srfi-9))

（させて （）
(レコードタイプ「人」の定義)
（友達の名前を作る）
人？
（名前-人名）
（友人、人、友人）

(letrec ((alice (make-person "Alice" (delay (list bob))))
(ボブ (make-person "Bob" (delay (list alice)))))
（アリスにマッチ）
(($ person name (= force (($ person "Bob"))))
(リスト「ボブの友人の名前」)
(\_ #f))))

⇒ （ボブの友人「アリス」）

ここでは、`$` パターンを使用して、2 つ以上のスロットを含む person 型の SRFI-9 レコードを照合します。最初のスロットの値は name にバインドされます。`=` パターンは、2 番目のスロットに `force` を適用し、その結果が指定されたパターンと一致するかどうかを確認するために使用されます。つまり、完全なパターンは、2 番目のスロットが、最初のスロットが `"Bob"` である person を含む 1 つの要素のリストに評価される Promise であるすべての person に一致します。

`(ice-9 match)` モジュールは、`match` をラップする以下の便利な構文糖衣マクロも提供します。

Scheme構文: **match-lambda** clause1 clause2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-match_002dlambda)

引数を1つだけ持ち、その引数を各句と照合し、対応する式の評価結果を返す手続きを作成します。

(match-lambda clause1 clause2 ...)
≍
(lambda (arg) (match arg clause1 clause2 ...))

((match-lambda
（（「こんにちは（誰）」）
誰が））
'（こんにちは世界）））
⇒世界

Scheme構文: **match-lambda\*** clause1 clause2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-match_002dlambda_002a)

引数の数を任意に指定できる手続きを作成し、その手続きが引数リストを各句と照合し、対応する式の評価結果を返すようにします。

(match-lambda\* clause1 clause2 ...)
≍
(ラムダ引数 (マッチ引数 clause1 clause2 ...))

((match-lambda\*
（（「こんにちは（誰）」）
誰が））
'こんにちは世界））
⇒世界

Scheme構文: **match-let** ((パターン式) …) body [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-match_002dlet)

各パターンを対応する式と照合し、一致したすべての変数をスコープ内に含めて本体を評価します。いずれかの式が一致しなかった場合はエラーを発生させます。`match-let` は名前付き let に類似しており、`match-lambda*` のように引数に基づいて照合する再帰関数にも使用できます。

(match-let (((xy) (list 1 2))
((ab) (リスト 3 4)))
(リスト abxy)
⇒
(3 4 1 2)

Scheme構文: **match-let** 変数 ((パターン初期化) …) 本体 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-match_002dlet-1)

`match-let` と同様ですが、_named let_ に類似しており、VARIABLE を INIT 式の数と同じ数の引数を受け入れる新しいプロシージャにローカルにバインドします。このプロシージャは、最初に INIT 式の評価結果に適用されます。呼び出されると、プロシージャは各引数を対応する PATTERN と照合し、BODY 式の評価結果を返します。_named let_ の詳細については、[Iteration](https://doc.guix.gnu.org/guile/latest/en/guile.html#while-do) を参照してください。

Scheme構文: **match-let\*** ((変数式) …) body [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-match_002dlet_002a)

`match-let` に似ていますが、`let*` と類似しており、スコープ内の先行するマッチ変数を含めて、変数を順番にマッチさせてバインドします。

(match-let\* (((xy) (list 1 2))
((ab) (リスト x 4)))
(リスト abxy)
≍
(match-let (((xy) (list 1 2)))
(match-let (((ab) (list x 4)))
(リスト abxy)))
⇒
(1 4 1 2)

Scheme構文: **match-letrec** ((変数式) …) body [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-match_002dletrec)

`match-let` に似ていますが、`letrec` と類似しており、スコープ内のすべてのマッチ変数と変数をマッチさせてバインドします。

Guileには、SXMLツリーに特化したパターンマッチング機能も付属しています。[`sxml-match`: SXMLのパターンマッチング](https://doc.guix.gnu.org/guile/latest/en/guile.html#sxml_002dmatch)を参照してください。

* * *

次へ: [Pretty Printing](https://doc.guix.gnu.org/guile/latest/en/guile.html#Pretty-Printing)、前: [Pattern Matching](https://doc.guix.gnu.org/guile/latest/en/guile.html#Pattern-Matching)、上: [Guile Modules](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guile-Modules) \[[Contents](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[Index](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "Index")\]
