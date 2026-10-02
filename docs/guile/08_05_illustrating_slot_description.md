### 8.5 スロットの説明の図解 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Illustrating-Slot-Description)

スロットの説明を説明するために、先ほど見た`<my-complex>`クラスを再定義してみましょう。定義例は以下のとおりです。

(define-class <my-complex> (<number>)
(r #:init-value 0 #:getter get-r #:setter set-r! #:init-keyword #:r)
(i #:init-value 0 #:getter get-i #:setter set-i! #:init-keyword #:i))

この定義では、`r` と `i` スロットはデフォルトで 0 に設定され、`make` を `#:r` と `#:i` キーワード付きで呼び出すことで他の値に初期化できます。また、汎用関数 `get-r`、`set-r!`、`get-i`、`set-i!` が自動的に定義され、スロットの読み書きができるようになります。

(define c1 ([make](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make-1) <my-complex> #:r 1 #:i 2))
(get-r c1) ⇒ 1
(set-r! c1 12)
(get-r c1) ⇒ 12
(define c2 ([make](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make-1) <my-complex> #:r 2))
(get-r c2) ⇒ 2
(get-i c2) ⇒ 0

アクセサーはスロットの読み取りと書き込みの両方を行うことができます。したがって、`#:accessor` オプションを使用した `<my-complex>` クラスの別の定義は次のようになります。

(define-class <my-complex> (<number>)
(r #:init-value 0 #:accessor [real- part](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-real_002dpart) #:init-keyword #:r)
(i #:init-value 0 #:accessor [imag-part](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-imag_002dpart) #:init-keyword #:i))

この定義では、`r`スロットは次のように読み取ることができます。

([real-part](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-real_002dpart) c)

そして以下のように設定します。

([set!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-set_0021) ([real-part](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-real_002dpart) c) new-value)

ここで、直交座標と極座標の両方で複素数を操作したいとしましょう。一つの解決策としては、特定の表現方法を用いる複素数の定義と、一方の表現から他方の表現へ変換するための関数を用意することが考えられます。より良い解決策は、次のような仮想スロットを使用することです。

(define-class <my-complex> (<number>)
;; 真のスロットは直交座標を使用します
(r #:init-value 0 #:accessor [real-part](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-real_002dpart) #:init-keyword #:r)
(i #:init-value 0 #:accessor [imag-part](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-imag_002dpart) #:init-keyword #:i)
;; 仮想スロットへのアクセスで変換を実行します
(m #:accessor [magnitude](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-magnitude) #:init-keyword #:magn
#:割り当て #:仮想
#:スロット参照 (ラムダ (o)
(let ((r ([slot-ref](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002dref-1) o 'r)) (i ([slot-ref](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002dref-1) o 'i)))
([sqrt](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sqrt) ([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) ([\*](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002a) rr) ([\*](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002a) ii)))))
#:スロットセット! (ラムダ (om)
(let ((a ([slot-ref](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002dref-1) o 'a)))
([slot-set!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002dset_0021-1) o 'r ([\*](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002a) m ([cos](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-cos) a)))
([slot-set!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002dset_0021-1) o 'i ([\*](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002a) m ([sin](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sin) a))))))
(a #:accessor [angle](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-angle) #:init-keyword #:angle
#:割り当て #:仮想
#:スロット参照 (ラムダ (o)
([atan](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-atan) ([slot-ref](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002dref-1) o 'i) ([slot-ref](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002dref-1) o 'r)))
#:スロットセット! (ラムダ(oa)
(let ((m ([slot-ref](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002dref-1) o 'm)))
([slot-set!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002dset_0021-1) o 'r ([\*](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002a) m ([cos](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-cos) a)))
([slot-set!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002dset_0021-1) o 'i ([\*](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002a) m ([sin](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sin) a)))))))

このクラス定義では、大きさ `m` と角度 `a` のスロットは仮想スロットであり、参照されると、通常の（つまり `#:allocation #:instance`）スロット `r` と `i` から、関連する `#:slot-ref` オプションで定義された関数を呼び出すことによって計算されます。同様に、`m` または `a` を書き込むと、`#:slot-set!` オプションで定義された関数が呼び出されます。したがって、次の式が成り立ちます。

([slot-set!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002dset_0021-1) c 'a 3)

複素数 `c` の角度を設定することを可能にします。

(define c ([make](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make-1) <my-complex> #:r 12 #:i 20))
([real-part](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-real_002dpart) c) ⇒ 12
([angle](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-angle) c) ⇒ 1.03037682652431
([slot-set!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002dset_0021-1) c 'i 10)
([set!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-set_0021) ([real-part](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-real_002dpart) c) 1)
([describe](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-describe) c)
⊣
#<<my-complex> 401e9b58> は [class](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-class-1) <my-complex> のインスタンスです
スロットは以下の通りです。
r [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) 1
i [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) 10
m [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) 10.0498756211209
a [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) 1.47112767430373

4つのスロットに対して初期化キーワードが定義されたので、標準のSchemeプリミティブである`make-rectangular`と`make-polar`を定義できるようになりました。

(define [make-rectangular](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002drectangular)
(lambda (xy) ([make](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make-1) <my-complex> #:rx #:iy)))

(define [make-polar](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dpolar)
(lambda (xy) ([make](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make-1) <my-complex> #:magn x #:angle y)))

* * *

次へ: [継承](https://doc.guix.gnu.org/guile/latest/en/guile.html#Inheritance)、前: [スロットの説明の例](https://doc.guix.gnu.org/guile/latest/en/guile.html#Slot-Description-Example)、上: [GOOPS](https://doc.guix.gnu.org/guile/latest/en/guile.html#GOOPS) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
