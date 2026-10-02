#### 7.5.48 SRFI-197: パイプライン演算子 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d197_003a-Pipeline-Operators)

[SRFI-197](http://srfi.schemers.org/srfi-197/srfi-197.html) は、Clojure の `->` や OCaml の `|>` のような関数型パイプライン (スレッド) 演算子を提供します。パイプラインは、深くネストされた式を記述するためのシンプルで簡潔かつ読みやすい方法です。この SRFI は、`(ab (cd (efg)))` のようなネストされた式を `(chain g (ef _) (cd _) (ab _))` という一連の操作に書き換えることができる、チェーンおよびネスト パイプライン演算子のファミリーを定義します。

`let*` と同様に、`chain` も評価順序を保証することに注意してください。実際、`(chain a (b _) (c _))` は `(let* ((x (ba)) (x (cx))) x)` のように展開され、`(c (ba))` のようには展開されません。そのため、`chain` は `if` や `let` のような構文を含むパイプラインには適していません。

複雑な構文を含むパイプラインの場合、`nest` および `nest-reverse` 演算子は `chain` のように見えますが、`let*` 形式ではなく、ネストされた形式に展開されることが保証されています。`nest` は `chain` とは逆方向にネストされるため、`(nest (a _) (b _) c)` は `(a (bc))` に展開されます。

Scheme構文: **chain** 初期値 \[プレースホルダー \[省略記号\]\] ステップ … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-chain)

各部分を小さな整数のシーケンスに変換し、対応するバイトのバイトベクトルを以下のように返します。

初期値は式です。

プレースホルダーと省略記号はリテラル記号です。これらはプレースホルダー記号と省略記号です。プレースホルダーまたは省略記号が存在しない場合、それぞれデフォルトで「_」と「...」になります。

ステップの構文は (データ …) で、各データはプレースホルダー記号、省略記号、または式のいずれかです。ステップには少なくとも 1 つのデータを含める必要があります。省略記号はステップの末尾でのみ使用でき、プレースホルダー記号の直後に配置する必要があります。

意味: `chain` は、各ステップを左から右へ順番に評価し、各ステップの結果を次のステップに渡します。

各ステップはアプリケーションとして評価され、そのアプリケーションの戻り値が次のステップのパイプライン値として渡されます。initial-value は最初のステップのパイプライン値です。`chain` の戻り値は最後のステップの戻り値です。

各ステップ内のプレースホルダー記号は、そのステップのパイプライン値に、出現順に置き換えられます。ステップのプレースホルダーの数とパイプライン値の数が異なる場合はエラーとなります。ただし、ステップにプレースホルダーが含まれていない場合は、パイプライン値は無視されます。

([chain](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-chain) x (ab [\_](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_005f)))
⇒（abx）
([chain](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-chain) (ab) (c [\_](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_005f) d) (ef [\_](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_005f)))
⇒ (let\* ((x (ab)) (x (cxd))) (efx))
([chain](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-chain) (a) (b [\_](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_005f) [\_](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_005f)) (c [\_](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_005f)))
⇒ (let\*-values (((x1 x2) (a)) ((x) (b x1 x2))) (cx))

ステップがプレースホルダー記号の後に省略記号が続く形で終了する場合、そのプレースホルダーシーケンスは、対応するプレースホルダーを持たない残りのすべてのパイプライン値に置き換えられます。

([chain](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-chain) (a) (b [\_](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_005f) c [\_](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_005f) [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e)) (d [\_](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_005f)))
⇒ (let\*-values (((x1 . x2) (a)) ((x) ([apply](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-apply) b x1 c x2))) (dx))

`chain` およびその他のすべての SRFI 197 マクロはカスタムプレースホルダー記号をサポートしており、構文定義の本体で `_` または `....` を挿入する場合に、衛生状態を維持するのに役立ちます。

([chain](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-chain) (ab) <> (c <> d) (ef <>))
⇒ (let\* ((x (ab)) (x (cxd))) (efx))
([chain](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-chain) (a) [\-](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002d) \--- (b [\-](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002d) c [\-](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002d) \---) (d [\-](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002d)))
⇒ (let\*-values (((x1 . x2) (a)) ((x) ([apply](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-apply) b x1 c x2))) (dx))

スキーム構文: **chain-and** initial-value \[placeholder\] step … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-chain_002dand)

initial-value は式です。placeholder はリテラル記号です。これはプレースホルダー記号です。placeholder が指定されていない場合、プレースホルダー記号は `_` です。step の構文は (datum … \[placeholder datum …\]) です。

意味: `chain` のバリアントで、いずれかのステップが `#f` を返す場合に処理を中断し、`#f` を返します。`chain-and` は `chain` に対して SRFI 2 と同じであり、`and-let*` は `let*` に対して同じです。

各ステップはアプリケーションとして評価されます。ステップが `#f` と評価された場合、残りのステップは評価されず、`chain-and` は `#f` を返します。それ以外の場合は、ステップの戻り値が次のステップのパイプライン値として渡されます。initial-value は最初のステップのパイプライン値です。どのステップも `#f` と評価されなかった場合、`chain-and` の戻り値は最後のステップの戻り値になります。

各ステップ内のプレースホルダーは、そのステップのパイプライン値に置き換えられます。ステップにプレースホルダーが含まれていない場合、そのステップのパイプライン値は無視されますが、`chain-and` は、そのパイプライン値が `#f` であるかどうかをチェックします。

`chain-and`は各ステップの戻り値をチェックするため、複数の戻り値を持つステップはサポートしていません。ステップが複数の値を返す場合はエラーとなります。

Scheme構文: **chain-when** initial-value \[placeholder\] (\[guard\] step) … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-chain_002dwhen)

initial-value と guard は式です。placeholder はリテラル記号です。これはプレースホルダー記号です。placeholder が存在しない場合、プレースホルダー記号は `_` です。step の構文は (datum … \[placeholder datum …\]) です。

セマンティクス: 各ステップにガード式があり、ガード式が #f と評価された場合にスキップされる `chain` のバリアント。

(定義 (数値 n を記述)
([chain-when](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-chain_002dwhen) '()
(([odd?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-odd_003f) n) ([cons](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-cons) "odd" [\_](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_005f)))
(([even?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-even_003f) n) ([cons](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-cons) "even" [\_](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_005f)))
(([zero?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-zero_003f) n) ([cons](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-cons) "zero" [\_](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_005f)))
(([positive?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-positive_003f) n) ([cons](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-cons) "positive" [\_](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_005f)))))

(describe-number 3) ; => '("positive" "odd")
(describe-number 4) ; => '("positive" "even")

各ステップはアプリケーションとして評価されます。ステップの戻り値は、次のステップのパイプライン値として渡されます。initial-valueは、最初のステップのパイプライン値です。

各ステップ内のプレースホルダーは、そのステップのパイプライン値に置き換えられます。ステップにプレースホルダーが含まれていない場合、そのステップのパイプライン値は無視されます。

ステップのガードが存在し、それが `#f` と評価される場合、そのステップはスキップされ、そのパイプライン値が次のステップのパイプライン値として再利用されます。`chain-when` の戻り値は、スキップされなかった最後のステップの戻り値、またはすべてのステップがスキップされた場合は初期値です。

`chain-when`はステップをスキップする可能性があるため、複数の戻り値を持つステップをサポートしていません。ステップが複数の値を返す場合はエラーとなります。

Scheme構文: **chain-lambda** \[placeholder \[ellipsis\]\] ステップ … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-chain_002dlambda)

プレースホルダーと省略記号はリテラル記号です。これらはプレースホルダー記号と省略記号です。プレースホルダーまたは省略記号が存在しない場合、それぞれデフォルトで「_」と「...」になります。

ステップの構文は (データ …) で、各データはプレースホルダー記号、省略記号、または式のいずれかです。ステップには少なくとも 1 つのデータを含める必要があります。省略記号はステップの末尾でのみ使用でき、プレースホルダー記号の直後に配置する必要があります。

セマンティクス：`chain`ステップのシーケンスからプロシージャを作成します。`chain-lambda`プロシージャが呼び出されると、各ステップが左から右の順に評価され、各ステップの結果が次のステップに渡されます。

([chain-lambda](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-chain_002dlambda) (a [\_](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_005f)) (b [\_](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_005f)))
⇒ (lambda (x) (let\* ((x (ax))) (bx)))
([chain-lambda](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-chain_002dlambda) (a [\_](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_005f) [\_](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_005f)) (bc [\_](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_005f)))
⇒ (lambda (x1 x2) (let\* ((x (a x1 x2))) (bcx)))

各ステップはアプリケーションとして評価され、そのアプリケーションの戻り値が次のステップのパイプライン値として渡されます。プロシージャの引数は、最初のステップのパイプライン値です。プロシージャの戻り値は、最後のステップの戻り値です。

各ステップ内のプレースホルダー記号は、そのステップのパイプライン値に、出現順に置き換えられます。ステップのプレースホルダーの数とパイプライン値の数が異なる場合はエラーとなります。ただし、ステップにプレースホルダーが含まれていない場合は、パイプライン値は無視されます。

ステップがプレースホルダー記号とそれに続く省略記号で終了する場合、そのプレースホルダーシーケンスは、対応するプレースホルダーを持たない残りのすべてのパイプライン値に置き換えられます。

最初のステップにおけるプレースホルダーの数によって、プロシージャの引数の個数が決まります。最初のステップが省略記号で終わる場合、そのプロシージャは可変引数プロシージャです。

Scheme構文: **ネスト** \[プレースホルダー\] <ステップ> … 初期値 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-nest)

プレースホルダーはリテラル記号です。これがプレースホルダー記号です。プレースホルダーが存在しない場合、プレースホルダー記号は `_` です。ステップの構文は (データ … プレースホルダー データ …) です。初期値は式です。

意味論: `nest` は `chain` と似ていますが、手順の順序が逆です。`chain` とは異なり、`nest` は式を文字通りネストします。そのため、`chain` のような厳密な評価順序の保証は提供しません。

([nest](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-nest) (ab [\_](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_005f)) (cd [\_](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_005f)) e) ; => (ab (cde))

`nest` 式は、最後のステップのプレースホルダーを初期値で字句的に置換し、次にその置換値で最後から2番目のステップのプレースホルダーを置換するという手順で評価され、最初のステップのプレースホルダーが置換されるまでこの処理が繰り返されます。最終的な置換結果が式でない場合はエラーとなり、式が評価されてその値が返されます。

`nest`は実際にネストされた形式を生成するため、`chain`では構築できない式を構築できます。たとえば、`nest`は引用符で囲まれたデータ構造を構築できます。

([nest](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-nest) '\_ (1 2 [\_](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_005f)) (3 [\_](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_005f) 5) ([\_](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_005f)) 4) ; => '(1 2 (3 (4) 5))

`nest` は、if、let、lambda、parameterize などの特殊な形式をパイプラインに安全に含めることもできます。

カスタムプレースホルダーを使用すると、`nest` 式を安全にネストできます。

([nest](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-nest) ([nest](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-nest) \_2 '\_2 (1 2 3 \_2) [\_](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_005f) 6)
([\_](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_005f) 5 \_2)
4)
⇒ '(1 2 3 (4 5 6))

Scheme構文: **nest-reverse** initial-value \[placeholder\] step … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-nest_002dreverse)

構文: initial-value は式です。 placeholder はリテラル記号です。これはプレースホルダー記号です。 placeholder が存在しない場合、プレースホルダー記号は `_` です。 step の構文は (datum … placeholder datum …) です。

意味: `nest-reverse` は `nest` のバリアントで、逆順でネストします。これは `chain` と同じ順序です。

([nest-reverse](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-nest_002dreverse) e (cd [\_](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_005f)) (ab [\_](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_005f))) ; => (ab (cde))

`nest-reverse`式は、まず最初のステップのプレースホルダーを初期値で字句的に置換し、次にその置換値で2番目のステップのプレースホルダーを置換し、最後のステップのプレースホルダーが置換されるまでこの処理を繰り返して評価されます。最終的な置換値が式でない場合はエラーとなり、式が評価されてその値が返されます。

* [謝辞](https://doc.guix.gnu.org/guile/latest/en/guile.html#Acknowledgements)

#### 7.5.48.1 謝辞 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Acknowledgements)

アダム・ネルソンがこのSRFIを改良するのを手伝ってくれたSRFI 197メーリングリストの参加者、特にマーク・ニーパー＝ヴィスキルヒェン、リヌス・ビョルンスタム、河合史郎、ラッシ・コルテラ、ジョン・コーワンに感謝します。

Marcが提供した段落は、（わずかな変更を加えただけで）ネストおよびネストリバースマクロのセマンティクスセクションに組み込まれています。

[Clojure](https://clojure.org/)とClojureスレッドマクロのオリジナル実装を提供してくれたRich Hickey氏、そして（EPLライセンスの）スレッドマクロのドキュメントページを提供してくれたPaulus Esterhazy氏に感謝します。このドキュメントは、インスピレーションの源であり、このドキュメントの例の一部にもなっています。

* * *

次へ: [SRFI-244 - 複数値定義](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d244)、前: [SRFI-197: パイプライン演算子](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d197)、上: [SRFI サポート モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI-Support) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
