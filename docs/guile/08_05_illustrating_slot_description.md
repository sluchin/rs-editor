### 8.5 スロットの説明の図解

スロットの説明を説明するために、先ほど見た`<my-complex>`クラスを再定義してみましょう。定義例は以下のとおりです。

(define-class <my-complex> (<number>)
(r #:init-value 0 #:getter get-r #:setter set-r! #:init-keyword #:r)
(i #:init-value 0 #:getter get-i #:setter set-i! #:init-keyword #:i))

この定義では、`r` と `i` スロットはデフォルトで 0 に設定され、`make` を `#:r` と `#:i` キーワード付きで呼び出すことで他の値に初期化できます。また、汎用関数 `get-r`、`set-r!`、`get-i`、`set-i!` が自動的に定義され、スロットの読み書きができるようになります。

(define c1 ([make](08_03_instance_creation_and_slot_access.md#83-インスタンスの作成とスロットへのアクセス) <my-complex> #:r 1 #:i 2))
(get-r c1) ⇒ 1
(set-r! c1 12)
(get-r c1) ⇒ 12
(define c2 ([make](08_03_instance_creation_and_slot_access.md#83-インスタンスの作成とスロットへのアクセス) <my-complex> #:r 2))
(get-r c2) ⇒ 2
(get-i c2) ⇒ 0

アクセサーはスロットの読み取りと書き込みの両方を行うことができます。したがって、`#:accessor` オプションを使用した `<my-complex>` クラスの別の定義は次のようになります。

(define-class <my-complex> (<number>)
(r #:init-value 0 #:accessor [real- part](06_06_02_numerical_data_types.md#66210-複素数演算) #:init-keyword #:r)
(i #:init-value 0 #:accessor [imag-part](06_06_02_numerical_data_types.md#66210-複素数演算) #:init-keyword #:i))

この定義では、`r`スロットは次のように読み取ることができます。

([real-part](06_06_02_numerical_data_types.md#66210-複素数演算) c)

そして以下のように設定します。

([set!](07_06_r6rs_support.md#7622-rnrs-ベース) ([real-part](06_06_02_numerical_data_types.md#66210-複素数演算) c) new-value)

ここで、直交座標と極座標の両方で複素数を操作したいとしましょう。一つの解決策としては、特定の表現方法を用いる複素数の定義と、一方の表現から他方の表現へ変換するための関数を用意することが考えられます。より良い解決策は、次のような仮想スロットを使用することです。

(define-class <my-complex> (<number>)
;; 真のスロットは直交座標を使用します
(r #:init-value 0 #:accessor [real-part](06_06_02_numerical_data_types.md#66210-複素数演算) #:init-keyword #:r)
(i #:init-value 0 #:accessor [imag-part](06_06_02_numerical_data_types.md#66210-複素数演算) #:init-keyword #:i)
;; 仮想スロットへのアクセスで変換を実行します
(m #:accessor [magnitude](06_06_02_numerical_data_types.md#66210-複素数演算) #:init-keyword #:magn
#:割り当て #:仮想
#:スロット参照 (ラムダ (o)
(let ((r ([slot-ref](08_08_introspection.md#885-スロットへのアクセス) o 'r)) (i ([slot-ref](08_08_introspection.md#885-スロットへのアクセス) o 'i)))
([sqrt](06_06_02_numerical_data_types.md#66212-科学関数) ([+](06_06_02_numerical_data_types.md#66211-算術関数) ([\*](06_06_02_numerical_data_types.md#66211-算術関数) rr) ([\*](06_06_02_numerical_data_types.md#66211-算術関数) ii)))))
#:スロットセット! (ラムダ (om)
(let ((a ([slot-ref](08_08_introspection.md#885-スロットへのアクセス) o 'a)))
([slot-set!](08_08_introspection.md#885-スロットへのアクセス) o 'r ([\*](06_06_02_numerical_data_types.md#66211-算術関数) m ([cos](06_06_02_numerical_data_types.md#66212-科学関数) a)))
([slot-set!](08_08_introspection.md#885-スロットへのアクセス) o 'i ([\*](06_06_02_numerical_data_types.md#66211-算術関数) m ([sin](06_06_02_numerical_data_types.md#66212-科学関数) a))))))
(a #:accessor [angle](06_06_02_numerical_data_types.md#66210-複素数演算) #:init-keyword #:angle
#:割り当て #:仮想
#:スロット参照 (ラムダ (o)
([atan](06_06_02_numerical_data_types.md#66212-科学関数) ([slot-ref](08_08_introspection.md#885-スロットへのアクセス) o 'i) ([slot-ref](08_08_introspection.md#885-スロットへのアクセス) o 'r)))
#:スロットセット! (ラムダ(oa)
(let ((m ([slot-ref](08_08_introspection.md#885-スロットへのアクセス) o 'm)))
([slot-set!](08_08_introspection.md#885-スロットへのアクセス) o 'r ([\*](06_06_02_numerical_data_types.md#66211-算術関数) m ([cos](06_06_02_numerical_data_types.md#66212-科学関数) a)))
([slot-set!](08_08_introspection.md#885-スロットへのアクセス) o 'i ([\*](06_06_02_numerical_data_types.md#66211-算術関数) m ([sin](06_06_02_numerical_data_types.md#66212-科学関数) a)))))))

このクラス定義では、大きさ `m` と角度 `a` のスロットは仮想スロットであり、参照されると、通常の（つまり `#:allocation #:instance`）スロット `r` と `i` から、関連する `#:slot-ref` オプションで定義された関数を呼び出すことによって計算されます。同様に、`m` または `a` を書き込むと、`#:slot-set!` オプションで定義された関数が呼び出されます。したがって、次の式が成り立ちます。

([slot-set!](08_08_introspection.md#885-スロットへのアクセス) c 'a 3)

複素数 `c` の角度を設定することを可能にします。

(define c ([make](08_03_instance_creation_and_slot_access.md#83-インスタンスの作成とスロットへのアクセス) <my-complex> #:r 12 #:i 20))
([real-part](06_06_02_numerical_data_types.md#66210-複素数演算) c) ⇒ 12
([angle](06_06_02_numerical_data_types.md#66210-複素数演算) c) ⇒ 1.03037682652431
([slot-set!](08_08_introspection.md#885-スロットへのアクセス) c 'i 10)
([set!](07_06_r6rs_support.md#7622-rnrs-ベース) ([real-part](06_06_02_numerical_data_types.md#66210-複素数演算) c) 1)
([describe](04_programming_in_scheme.md#4441-ヘルプコマンド) c)
⊣
#<<my-complex> 401e9b58> は [class](08_11_the_metaobject_protocol.md#8115-クラス定義プロトコル) <my-complex> のインスタンスです
スロットは以下の通りです。
r [\=](06_06_02_numerical_data_types.md#6628-比較述語) 1
i [\=](06_06_02_numerical_data_types.md#6628-比較述語) 10
m [\=](06_06_02_numerical_data_types.md#6628-比較述語) 10.0498756211209
a [\=](06_06_02_numerical_data_types.md#6628-比較述語) 1.47112767430373

4つのスロットに対して初期化キーワードが定義されたので、標準のSchemeプリミティブである`make-rectangular`と`make-polar`を定義できるようになりました。

(define [make-rectangular](06_06_02_numerical_data_types.md#66210-複素数演算)
(lambda (xy) ([make](08_03_instance_creation_and_slot_access.md#83-インスタンスの作成とスロットへのアクセス) <my-complex> #:rx #:iy)))

(define [make-polar](06_06_02_numerical_data_types.md#66210-複素数演算)
(lambda (xy) ([make](08_03_instance_creation_and_slot_access.md#83-インスタンスの作成とスロットへのアクセス) <my-complex> #:magn x #:angle y)))

* * *

次へ: [継承](08_07_inheritance.md#87-継承)、前: [スロットの説明の例](#85-スロットの説明の図解)、上: [GOOPS](08_00_goops.md#8-goops) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
