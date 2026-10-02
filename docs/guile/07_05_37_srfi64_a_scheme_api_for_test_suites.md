#### 7.5.37 SRFI-64: テストスイート用の Scheme API [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d64_003a-A-Scheme-API-for-Test-Suites)

* [SRFI-64 抄録](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d64-Abstract)
* [SRFI-64 理論的根拠](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d64-Rationale)
* [SRFI-64 基本的なテストスイートの作成](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d64-Writing-Basic-Test-Suites)
* [SRFI-64 条件付きテストスイートおよびその他の高度な機能](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d64-Conditonal-Test-Suites-and-Other-Advanced-Features)
* [SRFI-64 テストランナー](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d64-Test-Runner)
* [SRFI-64 テスト結果](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d64-Test-Results)
* [SRFI-64 新しいテストランナーの作成](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d64-Writing-a-New-Test-Runner)

* * *

次へ: [SRFI-64 の根拠](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d64-Rationale)、上へ: [SRFI-64: テスト スイート用の Scheme API](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d64) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.5.37.1 SRFI-64 要約 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d64-Abstract-1)

これは、Scheme API、ライブラリ、アプリケーション、および実装を移植性の高い方法で簡単にテストできるように、_テストスイート_を作成するためのAPIを定義します。テストスイートは、_テストランナー_のコンテキストで実行される_テストケース_の集合です。この仕様は、テストスイートの実行結果のレポートと処理をカスタマイズできるように、新しいテストランナーの作成もサポートしています。

* * *

次へ: [SRFI-64 基本的なテストスイートの作成](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d64-Writing-Basic-Test-Suites)、前: [SRFI-64 概要](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d64-Abstract)、上: [SRFI-64: テストスイートのための Scheme API](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d64) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次内容")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.5.37.2 SRFI-64 理論的根拠 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d64-Rationale-1)

Schemeコミュニティは、テストスイートを作成するための標準規格を必要としています。すべてのSRFIやその他のライブラリには、テストスイートが付属しているべきです。このようなテストスイートは、モジュールなどの非標準機能を必要とせず、移植性を備えている必要があります。テストスイートの実装、つまり「ランナー」自体は移植性を持つ必要はありませんが、移植性の高い基本的な実装を作成できることが望ましいです。

Schemeで書かれたテストフレームワークは他にもあり、[RackUnit](https://docs.racket-lang.org/rackunit/)などがその例です。しかし、RackUnitは移植性に欠け、やや冗長な部分もあります。このフレームワークとRackUnitの間にブリッジがあれば、RackUnitのテストをこのフレームワークで実行したり、その逆も可能になるでしょう。また、Javaの「標準」[JUnit](https://www.junit.org/) APIへのSchemeインターフェースを提供するSchemeラッパーも少なくとも1つ存在します。このフレームワークで書かれたテストをJUnitランナーで実行できるようにブリッジがあれば便利でしょう。これらの機能はいずれもこの仕様には含まれていません。

このAPIは、暗黙的な「テストランナー」を含む、暗黙的な動的状態を利用します。これにより、APIは便利で簡潔に使用できますが、JUnitスタイルのフレームワークのような明示的なテストオブジェクトを使用する場合と比べて、やや洗練さや「構成性」に欠けるかもしれません。オブジェクト指向設計や関数型設計の原則に従うことを意図したものではありませんが、使いやすく、拡張しやすいものとなることを願っています。

この提案では、いくつかのマクロを追加するだけで、Schemeソースファイルをテストスイートに変換できます。ファイル全体を新しい形式で書き直す必要がないため、インデントをやり直す必要もありません。

APIで定義されているすべての名前は、接頭辞「test-」で始まります。関数のような形式はすべて構文として定義されています。これらは関数、マクロ、または組み込み関数として実装できます。構文として指定する理由は、部分式を評価せずに特定のテストをスキップしたり、実装で行番号の表示や例外の捕捉などの機能を追加できるようにするためです。

* * *

次へ: [SRFI-64 条件付きテスト スイートとその他の高度な機能](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d64-Conditonal-Test-Suites-and-Other-Advanced-Features)、前: [SRFI-64 の根拠](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d64-Rationale)、上: [SRFI-64: テスト スイート用の Scheme API](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d64) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次内容")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.5.37.3 SRFI-64 基本的なテストスイートの作成 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d64-Writing-Basic-Test-Suites-1)

まずは簡単な例から始めましょう。これは完全に自己完結型のテストスイートです。

;; シンプルなテストスイートを初期化して名前を付けます。
([test-begin](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dbegin) "vec-test")
(define v ([make-vector](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dvector) 5 99))
式が真と評価されることを要求します。
([test-assert](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dassert) ([vector?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-vector_003f) v))
;; ある式が他の式と等価であるかどうかをテストします。
([test-eqv](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002deqv) 99 ([vector-ref](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-vector_002dref) v 2))
([vector-set!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-vector_002dset_0021) v 2 7)
([test-eqv](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002deqv) 7 ([vector-ref](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-vector_002dref) v 2))
;; テストスイートを完了し、結果を報告します。
([test-end](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dend) "vec-test")

このテストスイートは、専用のソースファイルに保存できます。他に何も必要ありません。トップレベルのフォームは不要なので、インデントを追加することなく、既存のプログラムやテストをこのフォームに簡単にラップできます。また、個々のテストに名前を付ける必要もなく（ただし、名前を付けるのは任意です）、新しいテストを簡単に追加できます。

テストケースは、テスト結果を蓄積して報告するオブジェクトである_テストランナー_のコンテキストで実行されます。この仕様では、カスタムテストランナーの作成方法と使用方法を定義していますが、実装ではデフォルトのテストランナーも提供する必要があります。上記のファイルをトップレベル環境で読み込むと、実装で指定されたデフォルトのテストランナーを使用してテストが実行され、`test-end`を実行すると、実装で指定された方法でサマリーが表示されることが推奨されます（必須ではありません）。Guileで使用されているSRFI 64実装では、このようなデフォルトのテストランナーが提供されています。上記のコードスニペットをREPLで実行すると、次の出力が得られます。

**テストグループに参加: vec-test **
$1 = #t
\* 合格：
$2 = ((pass . 1))
\* 合格：
$3 = ((pass . 2))
\* 合格：
$4 = ((pass . 3))
**テストグループを離脱します: vec-test **
**テストスイートが完了しました。**
*** 予想されるパス数：3

また、`<test-runner>`オブジェクトも返します。

#### シンプルなテストケース [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Simple-test_002dcases)

基本的なテストケースは、特定の条件が真であることをテストします。テストケースには名前が付けられる場合があります。基本的なテストケースの形式は `test-assert` です。

Scheme構文: **test-assert** \[test-name\] 式 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dassert)

これは式を評価します。結果が真であればテストは合格です。結果が偽であれば、テスト失敗が報告されます。例外が発生した場合も、実装に例外をキャッチする方法があれば、テストは失敗します。失敗の報告方法は、テスト ランナーの環境によって異なります。test-name は、テスト ケースの名前を表す文字列です。（例では test-name は文字列リテラルですが、式です。一度だけ評価されます。）これは、エラーを報告するとき、および後述するようにテストをスキップするときに使用されます。現在のテスト ランナーがない場合に `test-assert` を呼び出すとエラーになります。

以下の形式は、`test-assert`を直接使用するよりも便利な場合があります。

Scheme構文: **test-eqv** \[test-name\] は test-expr を必要とします [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002deqv)

これは以下と同等です。

([test-assert](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dassert) \[test-name\] ([eqv?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-eqv_003f) 期待されるテスト式))

同様に、`test-equal`と`test-eq`は、それぞれ`test-assert`と`equal?`または`eq?`を組み合わせたものの省略形です。

Scheme構文: **test-equal** \[test-name\] 期待されるtest-expr [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dequal)

Scheme構文: **test-eq** \[test-name\] 期待されるtest-expr [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002deq)

簡単な例を挙げましょう。

(define (mean xy) ([/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002f) ([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) xy) 2.0))
([test-eqv](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002deqv) 4 (平均 3 5))

不正確な実数の近似等価性をテストするには、`test-approximate`を使用できます。

Scheme構文: **test-approximate** \[test-name\] は test-expr を必要とします エラー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dapproximate)

これは以下と同等です（ただし、各引数は一度だけ評価されます）。

([test-assert](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dassert) \[test-name\]
(および ([\>=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e_003d) test-expr ([\-](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002d) は [error](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-error)))
([<=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003c_003d) test-expr ([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) expected [error](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-error)))))

以下に例を示します。

([test-approximate](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dapproximate) "22/7 は π の 1% 以内ですか?
3.1415926535
22/7
1/100)

#### エラー検出のためのテスト [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Tests-for-catching-errors)

評価が失敗するべきであることを明示的に指定する方法が必要です。これにより、必要な場合にエラーが検出されることが確認できます。

Scheme構文: **test-error** \[\[test-name\] error-type\] test-expr [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002derror)

test-expr の評価ではエラーが発生することが想定されます。エラーの種類は error-type で示されます。

エラータイプが省略されている場合、または`#t`の場合は、「何らかの未指定のエラーを通知する必要がある」ことを意味します。例：

([test-error](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002derror) #t ([vector-ref](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-vector_002dref) '#(1 2) 9))

この仕様では、テストエラーの形式は実装依存（または将来の仕様で規定）としていますが、すべての実装で `#t` を許可する必要があります。一部の実装では [SRFI-35 の条件](https://srfi.schemers.org/srfi-35/srfi-35.html) をサポートする場合がありますが、これらは [SRFI-36 の I/O 条件](https://srfi.schemers.org/srfi-36/srfi-36.html) に対してのみ標準化されており、テストスイートではほとんど役に立ちません。実装によっては、実装固有の「例外タイプ」を許可することもできます。たとえば、Java ベースの実装では、Java 例外クラスの名前を許可できます。

;; カワ固有の例
([test-error](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002derror) <java.lang.IndexOutOfBoundsException> ([vector-ref](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-vector_002dref) '#(1 2) 9))

例外を捕捉できない実装では、`test-error` フォームをスキップする必要があります。

GuileのSRFI-64実装では、エラータイプを以下のように指定できます。

* `#f`は、テストでエラーが発生することが想定されていないことを意味します。
* `#t`は、テストで何らかのエラーが発生することが予想されることを意味します。
* SRFI-35 の `make-exception-type` または `make-condition-type` で作成されたネイティブ例外タイプ
* キャッチされた例外に適用され、それが正しい型であるかどうかを判断する述語
* 例外タイプのレガシーな `make-exception-from-throw` スタイルの例外を表すシンボル。

以下は、Guileで有効な例です。

([test-error](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002derror) "旧式の例外タイプを期待する"
数値オーバーフロー
([/](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002f) 1 0))

([use-modules](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-use_002dmodules) (ice-9 例外)) ;標準例外タイプの場合
([test-error](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002derror) "ネイティブ例外タイプを期待する"
[&warning](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0026warning)
([raise-exception](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-raise_002dexception) ([make-warning](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dwarning))))

([test-error](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002derror) "述語を使用してネイティブ例外を期待する"
[警告?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-warning_003f)
([raise-exception](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-raise_002dexception) ([make-warning](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dwarning))))

([use-modules](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-use_002dmodules) (srfi srfi-35))

([test-error](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002derror) "深刻なSRFI 35条件タイプを想定"
[&serious](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0026serious)
([raise-Exception](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-raise_002dException) ([条件](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-condition) ([&serious](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0026serious))))

([test-error](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002derror) "述語を使用して、深刻な SRFI 35 条件タイプを想定"
[serious-condition?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-serious_002dcondition_003f)
([raise-Exception](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-raise_002dException) ([条件](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-condition) ([&serious](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0026serious))))

#### 構文のテスト [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Testing-syntax)

構文のテストは難しいものです。特に、無効な構文がエラーの原因となっているかどうかを確認したい場合はなおさらです。以下のユーティリティ関数が役立ちます。

Scheme手順: **test-read-eval-string** string [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dread_002deval_002dstring)

この関数は文字列を解析し（`read`を使用）、結果を評価します。評価結果は`test-read-eval-string`から返されます。`read`の後に未読の文字がある場合はエラーが通知されます。例：`(test-read-eval-string "(+ 3 4)")`は`7`に評価されます。`(test-read-eval-string "(+ 3 4")`はエラーを通知します。`(test-read-eval-string "(+ 3 4) ")`は、リストの読み取り後に余分な「ジャンク」（つまりスペース）があるため、エラーを通知します。

テストで使用される`test-read-eval-string`：

([test-equal](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dequal) 7 ([test-read-eval-string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- test_002dread_002deval_002dstring) "(+ 3 4)"))
([test-error](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002derror) ([test-read-eval-string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dread_002deval_002dstring) "(+ 3"))
([test-equal](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dequal) #\\newline ([test-read-eval-string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dread_002deval_002dstring) "#\\\\newline"))
([test-error](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002derror) ([test-read-eval-string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dread_002deval_002dstring) "#\\\\newlin"))
;; srfi-62 が利用可能でない限り、次の 2 つのテストをスキップします。
([test-skip](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dskip) ([cond-expand](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-cond_002dexpand) (srfi-62 0) (else 2)))
([test-equal](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dequal) 5 ([test-read-eval-string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dread_002deval_002dstring) "(+ 1 #;(\* 2 3) 4)"))
([test-equal](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dequal) '(xz) (test-read-string "(list 'x #;'y 'z)"))

#### テストグループとパス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Test-groups-and-paths)

テストグループとは、テストケース、式、定義を含むフォームの名前付きシーケンスです。グループに入るとテストグループ名が設定され、グループから出ると以前のグループ名に戻ります。これらは動的（実行時）操作であり、グループにはその他の効果や識別情報はありません。テストグループは非公式なグループ分けであり、Scheme の値でも構文形式でもありません。テストグループには、ネストされた内部テストグループを含めることができます。テストグループパスは、現在アクティブな（入っている）テストグループ名のリストであり、最も古い（最も外側の）ものから順に並んでいます。

Scheme構文: **test-begin** suite-name \[count\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dbegin)

`test-begin`を実行すると、新しいテストグループが開始されます。suite-nameは現在のテストグループ名となり、テストグループパスの末尾に追加されます。移植性の高いテストスイートでは、 suite-nameに文字列リテラルを使用する必要があります。式やその他の種類のリテラルを使用した場合の効果は規定されていません。

理由：記号を使う方が望ましい場合もあるでしょう。しかし、人間が読みやすい名前が必要であり、標準のSchemeでは、リテラル記号にスペースや大文字小文字を混在させる方法がありません。

オプションのカウントは、このグループによって実行されるテストケースの数と一致する必要があります。（ネストされたテストグループは、このカウントでは1つのテストケースとしてカウントされます。）この追加テストは、予期しないエラーのためにテストが実行されなかった場合を検出するのに役立つ場合があります。

さらに、現在実行中のテストランナーが存在しない場合は、実装定義の方法でインストールされます。

Scheme構文: **test-end** \[suite-name\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dend)

`test-end`を実行すると、現在のテストグループから離脱します。スイート名が現在のテストグループ名と一致しない場合は、エラーが報告されます。

さらに、一致する`test-begin`が新しいテストランナーをインストールした場合、`test-end`は実装定義の方法で蓄積されたテスト結果を報告した後、それをアンインストールします。

Scheme構文: **test-group** suite-name decl-or-expr … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dgroup)

同等の項目:

(if ([not](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-not) (test-to-skip% (var suite-name)))
([dynamic-wind](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-dynamic_002dwind)
(lambda () ([test-begin](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dbegin) (var suite-name)))
(lambda () (var decl-or-expr) [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e))
(lambda () ([test-end](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dend) (var suite-name)))))

これは通常、指定されたテストグループ内で宣言または式を実行することと同等です。ただし、アクティブな `test-skip` に一致した場合は、グループ全体がスキップされます（後述）。また、例外が発生した場合は `test-end` が実行されます。

#### セットアップとクリーンアップの処理 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Handling-set_002dup-and-cleanup)

Scheme構文: **test-group-with-cleanup** suite-name decl-or-expr … cleanup-form [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dgroup_002dwith_002dcleanup)

宣言式または式の各形式を（<body> のように）順番に実行し、その後、クリーンアップ形式を実行します。宣言式または式のいずれかで例外が発生した場合でも、クリーンアップ形式は実行される必要があります（実装に例外を捕捉する方法がある場合）。

例えば：

(let ((f ([open-output-file](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-open_002doutput_002dfile) "log")))
([test-group-with-cleanup](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dgroup_002dwith_002dcleanup) "test-file"
（たくさんのテストを実行する f）
([close-output-port](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-close_002doutput_002dport) f)))

* * *

次へ: [SRFI-64 テスト ランナー](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d64-Test-Runner)、前: [SRFI-64 基本的なテスト スイートの作成](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d64-Writing-Basic-Test-Suites)、上: [SRFI-64: テストスイート用の Scheme API](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d64) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次内容")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.5.37.4 SRFI-64 条件付きテスト スイートとその他の高度な機能 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d64-Conditonal-Test-Suites-and-Other-Advanced-Features-1)

以下では、実行するテストを制御する機能、または一部のテストが失敗すると想定される機能について説明します。

#### テスト指定子 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Test-specifiers)

特定のテストのみを実行したい場合や、特定のテストが失敗することがわかっている場合があります。テスト指定子は、テストランナーを受け取り、ブール値を返す引数1つの関数です。指定子はテスト実行前に実行でき、その結果によってテストの実行を制御できます。便宜上、指定子は非プロシージャ値にすることもできます。その場合、countとnameについて後述するように、指定子プロシージャに強制変換されます。

簡単な例を挙げると次のようになります。

(if (var some-condition) ([test-skip](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dskip) 2)) ;; 次の 2 つのテストをスキップする

スキーム手順: **test-match-name** name [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dmatch_002dname)

結果として得られる指定子は、現在のテスト名（`test-runner-test-name` によって返されるもの）が name と `equal?` である場合に一致します。

Scheme構文: **test-match-nth** n \[count\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dmatch_002dnth)

これは_状態を持つ_述語に評価されます。カウンターは、呼び出された回数を追跡します。述語は、n回目の呼び出し（`1`が最初の呼び出し）と、次の'(- count 1)'回の呼び出しに一致します。ここで、countのデフォルト値は`1`です。

Scheme構文: **test-match-any**指定子 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dmatch_002dany)

結果として得られる指定子は、いずれかの指定子が一致した場合に一致します。各指定子は順番に適用されるため、前の指定子が真であっても、後の指定子による副作用が発生します。

Scheme構文: **test-match-all**指定子 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dmatch_002dall)

結果として得られる指定子は、各指定子が一致する場合に一致します。各指定子は順番に適用されるため、前の指定子が偽であっても、後の指定子による副作用が発生します。

count _(つまり整数)_ '(test-match-nth 1 count)' の便利な省略形。

name _(文字列)_ '(test-match-name name)' の便利な省略形。

#### 選択したテストをスキップする [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Skipping-selected-tests)

場合によっては、テストをスキップしたい場合もあるでしょう。

Scheme構文: **test-skip**指定子 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dskip)

`test-skip` を評価すると、結果として得られる指定子が現在有効なスキップ指定子のセットに追加されます。各テスト（または `test-group`）の前に、有効なスキップ指定子のセットが有効なテストランナーに適用されます。いずれかの指定子が一致する場合、テストはスキップされます。

便宜上、指定子が `(test-match-name specifier)` の構文糖衣である文字列である場合。例:

([test-skip](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dskip) "test-b")
([test-assert](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dassert) "test-a") ;; 実行済み
([test-assert](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dassert) "test-b") ;; スキップ

`test-skip` によって導入されたスキップ指定子は、後続のネストされていない `test-end` によって削除されます。

([test-begin](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dbegin) "group1")
([test-skip](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dskip) "test-a")
([test-assert](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dassert) "test-a") ;; スキップ
([test-end](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dend) "group1") ;; 前の test-skip を取り消す
([test-assert](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dassert) "test-a") ;; 実行済み

#### 想定される障害 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Expected-failures)

テストケースが失敗すると分かっていても、修正する時間がない、あるいは修正できない場合があります。例えば、特定の機能が特定のプラットフォームでしか動作しない場合などです。しかし、修正を忘れないように、テストケースを残しておきたいと思うでしょう。そのようなテストは失敗することが想定されている、ということを記録しておきたいのです。

Scheme構文: **test-expect-fail**指定子 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dexpect_002dfail)

一致するテスト（一致の定義は`test-skip`で定義されている）は失敗することが想定されています。これはテストの実行ではなく、テストレポートにのみ影響します。例：

([test-expect-fail](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dexpect_002dfail) 2)
([test-eqv](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002deqv) [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e)) ;; 失敗すると予想される
([test-eqv](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002deqv) [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e)) ;; 失敗すると予想される
([test-eqv](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002deqv) [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e)) ;; 合格が期待される

* * *

次へ: [SRFI-64 テスト結果](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d64-Test-Results)、前: [SRFI-64 条件付きテスト スイートとその他の高度な機能](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d64-Conditonal-Test-Suites-and-Other-Advanced-Features)、上: [SRFI-64: テスト スイート用の Scheme API](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d64) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次内容")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.5.37.5 SRFI-64 テストランナー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d64-Test-Runner-1)

テストランナーは、テストスイートを実行し、状態を管理するオブジェクトです。テストグループパス、スキップおよび期待失敗指定子のセットは、テストランナーの一部です。テストランナーは通常、実行されたテストに関する統計情報も収集します。

Scheme Procedure: **test-runner?** value [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_003f)

値がテストランナーオブジェクトである場合に限り、真となります。

スキームパラメータ: **test-runner-current** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- test_002drunner_002dcurrent)

スキームパラメータ: **test-runner-current** ランナー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_002dcurrent-1)

現在のテストランナーを取得または設定します。

Scheme Procedure: **test-runner-get** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_002dget)

`(test-runner-current)` と同じですが、現在のテストランナーがない場合は例外をスローします。

Scheme手順: **test-runner-simple** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_002dsimple)

標準出力ポートにエラーと概要を出力する、新しいシンプルなテストランナーを作成します。

Scheme Procedure: **test-runner-null** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_002dnull)

テスト結果を一切処理しない新しいテストランナーを作成します。これは主に、カスタムランナーを作成する際に拡張することを目的としています。

Scheme手順: **test-runner-create** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_002dcreate)

新しいテストランナーを作成します。'((test-runner-factory))' と同等です。

スキームパラメータ: **test-runner-factory** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_002dfactory)

スキームパラメータ: **test-runner-factory** ファクトリ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_002dfactory-1)

現在のテストランナーファクトリを取得または設定します。ファクトリとは、新しいテストランナーを作成する引数なしの関数です。デフォルト値は`test-runner-simple`です。

#### 特定のランナーを使用して特定のテストを実行する [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Running-specific-tests-with-a-specified-runner)

Scheme プロシージャ: **test-apply** \[runner\] 指定子 … プロシージャ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dapply)

指定されたランナーを現在のテストランナーとして使用し、引数なしでプロシージャを呼び出します。ランナーが省略された場合は、`(test-runner-current)` が使用されます。（現在のランナーがない場合は、`test-begin` と同様に作成されます。） 1 つ以上の指定子がリストされている場合は、指定子に一致するテストのみが実行されます。指定子は、`test-skip` で使用されるものと同じ形式です。テストは、`test-apply` のいずれかの指定子に一致し、かつアクティブな `test-skip` 指定子に一致しない場合に実行されます。

Scheme構文: **test-with-runner** runner decl-or-expr … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dwith_002drunner)

現在のテストランナーが runner であるコンテキストで、各宣言または式を順番に実行します。

* * *

次へ: [SRFI-64 新しいテスト ランナーの作成](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d64-Writing-a-New-Test-Runner)、前: [SRFI-64 テスト ランナー](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d64-Test-Runner)、上: [SRFI-64: テスト スイート用の Scheme API](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d64) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次内容")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.5.37.6 SRFI-64 テスト結果 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d64-Test-Results-1)

テストを実行すると、現在のテストランナーにさまざまなステータスプロパティが設定されます。これは、カスタムテストランナー、または（まれに）テストスイートで確認できます。

#### 結果の種類 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Result-Kind)

テストを実行すると、以下のいずれかのステータスシンボルが表示される場合があります。

`'パス`

試験は予想通り合格した。

`'失敗`

試験は失敗した（そして、失敗するとは予想されていなかった）。

`'xfail`

試験は失敗したが、それは予想通りだった。

`'xpass`

試験は合格したが、不合格になると予想されていた。

`'スキップ`

そのテストはスキップされた。

スキーム手順: **test-result-kind** \[runner\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dresult_002dkind)

最新のテストから上記のいずれかの結果コードを返します。テストがまだ実行されていない場合は `#f` を返します。新しいテストを開始したが、まだ結果がない場合、テストが失敗すると予想される場合は `'xfail`、テストがスキップされるべき場合は `'skip`、それ以外の場合は `#f` を返します。

Scheme 手順: **test-passed?** \[runner\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dpassed_003f)

'(test-result-kind \[runner\])' の値が `'pass` または `'xpass` のいずれかである場合に真となります。これは、テストスイートにおいて、前のテストが合格した場合にのみ特定のテストを実行する場合に便利な省略記法です。

#### テスト結果のプロパティ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Test-result-properties)

テストランナーは、現在または最新のテストに関連付けられた、より詳細な「結果プロパティ」のセットも保持します。（つまり、新しいテストが開始されていない限り、最新のテストのプロパティが利用可能です。）各プロパティには、名前（シンボル）と値（任意の値）があります。一部のプロパティは標準で定義されているか、実装によって設定されます。実装によっては、プロパティを追加することもできます。

Scheme Procedure: **test-result-ref** runner pname \[default\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dresult_002dref)

pname プロパティ名（シンボル）に関連付けられたプロパティ値を返します。pname に関連付けられた値がない場合はデフォルト値を返し、デフォルト値が指定されていない場合は `#f` を返します。

Scheme構文: **test-result-set!** runner pname value [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dresult_002dset_0021)

pnameプロパティ名に関連付けられたプロパティ値を値に設定します。通常は実装コードからこの関数を呼び出す必要がありますが、カスタムテストランナーが追加のプロパティを追加する場合にも役立つ場合があります。

Scheme Procedure: **test-result-remove** runner pname [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- test_002dresult_002dremove)

pnameという名前のプロパティを削除します。

Scheme Procedure: **test-result-clear** runner [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dresult_002dclear)

結果プロパティをすべて削除します。実装では、`test-assert` および同様の手順の開始時に `test-result-clear` が自動的に呼び出されます。

Scheme Procedure: **test-result-alist** runner [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dresult_002dalist)

現在の結果プロパティの関連付けリストを返します。結果がテストランナーと状態を共有するかどうかは指定されていません。結果は変更すべきではありませんが、将来の `test-result-set!` または `test-result-remove` 呼び出しによって暗黙的に変更される可能性があります。ただし、`test-result-clear` は返される alist を変更しません。したがって、以前の実行結果オブジェクトを「アーカイブ」することができます。

#### 標準結果プロパティ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Standard-result-properties)

利用可能な結果プロパティのセットは実装によって異なります。ただし、以下のプロパティが提供される可能性があることが推奨されます。

`'result-kind`

前述のとおり、結果の種類を指定します。これは必須の結果プロパティです。`(test-result-kind runner)` は、`(test-result-ref runner 'result-kind)` と同等です。

`'ソースファイル`

`'ソース行`

テストスイートのソースコード内におけるテストステートメント（`test-assert`など）の位置（既知の場合）。

`'source-form`

意味があり、かつ既知である場合は、そのソース形式。

`'期待値`

意味があり既知の場合、期待されるエラーのない結果。

`'expected-error`

`test-error`で指定されたエラータイプ（意味があり既知の場合）。

`'実際値`

意味があり既知の場合、実際のエラーではない結果値。

`'実際エラー`

エラーが通知され、かつそのエラーが既知の場合のエラー値。実際のエラー値は実装依存です。

* * *

前へ: [SRFI-64 テスト結果](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d64-Test-Results)、上へ: [SRFI-64: テストスイート用の Scheme API](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d64) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.5.37.7 SRFI-64 新しいテストランナーの作成 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d64-Writing-a-New-Test-Runner-1)

このセクションでは、テストランナーの書き方について説明します。テストケースだけを記述したい場合は、このセクションは無視しても構いません。

#### コールバック関数 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Call_002dback-Functions)

これらのコールバック関数は、テストランナーの「メソッド」（オブジェクト指向的な意味で）です。`test-runner-on-event` メソッドは、イベントが発生したときに実装によって呼び出されます。

イベントのコールバック関数を定義（設定）するには、次の式を使用します。（これは通常、テストランナーを初期化する際に行います。）

`(test-runner-on-event! runner event-function)`

イベント関数は、テストランナー引数を受け取り、イベントによってはその他の引数を受け取る場合もあります。

イベントのコールバック関数を抽出（取得）するには、次のようにします。`(test-runner-on-event runner)`

イベントのコールバック関数を呼び出すには、次の式を使用します。（これは通常、実装コアによって行われます。） '((test-runner-on-event runner) runner other-args …)'。

以下のコールバックフックが利用可能です。

Scheme Procedure: **test-runner-on-test-begin** runner [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_002don_002dtest_002dbegin)

Scheme Procedure: **test-runner-on-test-begin!** runner on-test-begin-function [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_002don_002dtest_002dbegin_0021)

スキームプロシージャ: **on-test-begin-function** ランナー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-on_002dtest_002dbegin_002dfunction)

on-test-begin関数は、個々のテストケースの開始時に、テスト式（および期待値）が評価される前に呼び出されます。

Scheme Procedure: **test-runner-on-test-end** runner [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_002don_002dtest_002dend)

Scheme Procedure: **test-runner-on-test-end!** runner on-test-end-function [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_002don_002dtest_002dend_0021)

スキーム手順: **on-test-end-function** ランナー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-on_002dtest_002dend_002dfunction)

on-test-end関数は、個々のテストケースの最後に、テスト結果が利用可能になった時点で呼び出されます。

Scheme Procedure: **test-runner-on-group-begin** runner [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_002don_002dgroup_002dbegin)

Scheme Procedure: **test-runner-on-group-begin!** runner on-group-begin-function [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_002don_002dgroup_002dbegin_0021)

Scheme Procedure: **on-group-begin-function** runner suite-name count [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-on_002dgroup_002dbegin_002dfunction)

on-group-begin-function は、`test-begin` によって呼び出されます。これには、`test-group` の開始時も含まれます。suite-name は Scheme 文字列で、count は整数または `#f` です。

Scheme Procedure: **test-runner-on-group-end** runner [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_002don_002dgroup_002dend)

Scheme Procedure: **test-runner-on-group-end!** runner on-group-end-function [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_002don_002dgroup_002dend_0021)

Scheme プロシージャ: **on-group-end-function** ランナー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-on_002dgroup_002dend_002dfunction)

on-group-end-function は、`test-end` によって呼び出されます。これには、`test-group` の末尾での呼び出しも含まれます。

スキーム手順: **test-runner-on-bad-count** ランナー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_002don_002dbad_002dcount)

Scheme Procedure: **test-runner-on-bad-count!** runner on-bad-count-function [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_002don_002dbad_002dcount_0021)

Scheme Procedure: **on-bad-count-function** runner actual-count expected-count [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-on_002dbad_002dcount_002dfunction)

`test-end` から呼び出されます (on-group-end-function が呼び出される前)。これは、一致する `test-begin` で期待されるカウントが指定されており、その期待されるカウントが実際に実行またはスキップされたテストの実際のカウントと一致しない場合に呼び出されます。

スキーム手順: **test-runner-on-bad-end-name** ランナー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_002don_002dbad_002dend_002dname)

Scheme Procedure: **test-runner-on-bad-end-name!** runner on-bad-end-name-function [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_002don_002dbad_002dend_002dname_0021)

Scheme Procedure: **on-bad-end-name-function** runner begin-name end-name [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- on_002dbad_002dend_002dname_002dfunction)

`test-end` から呼び出されます (on-group-end-function が呼び出される前)。スイート名が指定されており、一致する `test-begin` の名前がなかった場合に呼び出されます。

Scheme Procedure: **test-runner-on-final** runner [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_002don_002dfinal)

Scheme Procedure: **test-runner-on-final!** runner on-final-function [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_002don_002dfinal_0021)

スキームプロシージャ: **on-final-function** ランナー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-on_002dfinal_002dfunction)

on-final関数は1つのパラメータ（テストランナー）を受け取り、通常はテストの概要（カウント）を表示します。on-final関数は、最も外側の`test-end`に対応するon-group-end関数が呼び出された後に呼び出されます。デフォルト値は`test-on-final-simple`で、これは各種テストの数を標準出力ポートに出力します。

`test-runner-simple` が返すデフォルトのテストランナーは、以下のコールバック関数を使用します。

Scheme Procedure: **test-on-test-begin-simple** runner [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002don_002dtest_002dbegin_002dsimple)

Scheme Procedure: **test-on-test-end-simple** runner [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002don_002dtest_002dend_002dsimple)

Scheme Procedure: **test-on-group-begin-simple** runner suite-name count [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002don_002dgroup_002dbegin_002dsimple)

Scheme 手順: **test-on-group-end-simple** ランナー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002don_002dgroup_002dend_002dsimple)

スキーム手順: **test-on-bad-count-simple** runner actual-count expected-count [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002don_002dbad_002dcount_002dsimple)

Scheme Procedure: **test-on-bad-end-name-simple** runner begin-name end-name [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002don_002dbad_002dend_002dname_002dsimple)

独自のテストランナーを作成したい場合は、それらを呼び出すことができます。

#### テストランナーのコンポーネント [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Test_002drunner-components)

以下の関数は、テストランナーの他のコンポーネントにアクセスするためのものです。これらは通常、新しいテストランナーまたはマッチ述語を作成する場合にのみ使用されます。

スキーム手順: **test-runner-pass-count** ランナー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_002dpass_002dcount)

合格したテストの数と、合格すると予想されていたテストの数を返します。

スキーム手順: **test-runner-fail-count** ランナー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_002dfail_002dcount)

合格するはずだったが、実際には失敗したテストの数を返します。

スキーム手順: **test-runner-xpass-count** ランナー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_002dxpass_002dcount)

合格したが、不合格になると予想されていたテストの数を返します。

スキーム手順: **test-runner-xfail-count** ランナー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_002dxfail_002dcount)

合格するはずだったテストのうち、失敗したテストの数を返します。

スキーム手順: **test-runner-skip-count** ランナー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_002dskip_002dcount)

スキップされたテストまたはテストグループの数を返します。

Scheme Procedure: **test-runner-test-name** runner [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_002dtest_002dname)

現在のテストまたはテストグループの名前を文字列として返します。`test-begin` の実行中はテストグループの名前、実際のテストの実行中はテストケースの名前になります。名前が指定されていない場合は、空の文字列が返されます。

Scheme Procedure: **test-runner-group-path** runner [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_002dgroup_002dpath)

私たちが属しているグループの名前のリスト。一番外側のグループが最初に表示されます。

スキーム手順: **test-runner-group-stack** ランナー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_002dgroup_002dstack)

ネストされているグループ名のリスト。一番外側のグループが最後に表示されます。（コピー処理が不要なため、`test-runner-group-path`よりも効率的です。）

スキーム手順: **test-runner-aux-value** ランナー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_002daux_002dvalue)

スキーム手順: **test-runner-aux-value!** テスト時のランナー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_002daux_002dvalue_0021)

テストランナーの`aux-value`フィールドを取得または設定します。このフィールドは、このAPIや`test-runner-simple`テストランナーでは使用されませんが、カスタムテストランナーが追加の状態を保存するために使用できます。

Scheme Procedure: **test-runner-reset** runner [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_002dreset)

ランナーの状態を初期状態にリセットします。

#### 例 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Example)

これはシンプルなカスタムテストランナーの例です。テストスイートを実行する前にこのプログラムを読み込むと、デフォルトのテストランナーとしてインストールされます。

(define (my-simple-runner filename)
(let ((runner ([test-runner-null](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_002dnull)))
	(ポート ([open-output-file](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-open_002doutput_002dfile) filename))
(渡された数値 0)
(失敗数 0)
([test-runner-on-test-end!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_002don_002dtest_002dend_0021) runner
(ラムダ (ランナー)
(case ([test-result-kind](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002dresult_002dkind) runner)
((pass xpass) ([set!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-set_0021) num-passed ([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) num-passed 1)))
((fail xfail) ([set!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-set_0021) num-failed ([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) num-failed 1)))
(それ以外の場合 #t))))
([test-runner-on-final!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_002don_002dfinal_0021) runner
(ラムダ (ランナー)
([format](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-format) port "テスト合格: ~d.~% テスト不合格: ~d.~%"
合格数 不合格数
	([close-output-port](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-close_002doutput_002dport) port)))
ランナー））
([test-runner-factory](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-test_002drunner_002dfactory)
(lambda () (my-simple-runner "/tmp/my-test.log")))

* * *

次へ: [SRFI-69 - 基本的なハッシュテーブル](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d69)、前: [SRFI-64: テストスイート用の Scheme API](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d64)、上: [SRFI サポートモジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI-Support) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
