#### 7.5.37 SRFI-64: テストスイート用の Scheme API

* [SRFI-64 抄録](#75371-srfi-64-要約)
* [SRFI-64 理論的根拠](#75372-srfi-64-理論的根拠)
* [SRFI-64 基本的なテストスイートの作成](#75373-srfi-64-基本的なテストスイートの作成)
* [SRFI-64 条件付きテストスイートおよびその他の高度な機能](#75374-srfi-64-条件付きテスト-スイートとその他の高度な機能)
* [SRFI-64 テストランナー](#75375-srfi-64-テストランナー)
* [SRFI-64 テスト結果](#75376-srfi-64-テスト結果)
* [SRFI-64 新しいテストランナーの作成](#75377-srfi-64-新しいテストランナーの作成)

* * *

次へ: [SRFI-64 の根拠](#75372-srfi-64-理論的根拠)、上へ: [SRFI-64: テスト スイート用の Scheme API](#7537-srfi-64-テストスイート用の-scheme-api) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.5.37.1 SRFI-64 要約

これは、Scheme API、ライブラリ、アプリケーション、および実装を移植性の高い方法で簡単にテストできるように、_テストスイート_を作成するためのAPIを定義します。テストスイートは、_テストランナー_のコンテキストで実行される_テストケース_の集合です。この仕様は、テストスイートの実行結果のレポートと処理をカスタマイズできるように、新しいテストランナーの作成もサポートしています。

* * *

次へ: [SRFI-64 基本的なテストスイートの作成](#75373-srfi-64-基本的なテストスイートの作成)、前: [SRFI-64 概要](#75371-srfi-64-要約)、上: [SRFI-64: テストスイートのための Scheme API](#7537-srfi-64-テストスイート用の-scheme-api) \[[目次](00_contents.md "目次内容")\]\[[索引](index_r5rs.md "索引")\]

#### 7.5.37.2 SRFI-64 理論的根拠

Schemeコミュニティは、テストスイートを作成するための標準規格を必要としています。すべてのSRFIやその他のライブラリには、テストスイートが付属しているべきです。このようなテストスイートは、モジュールなどの非標準機能を必要とせず、移植性を備えている必要があります。テストスイートの実装、つまり「ランナー」自体は移植性を持つ必要はありませんが、移植性の高い基本的な実装を作成できることが望ましいです。

Schemeで書かれたテストフレームワークは他にもあり、[RackUnit](https://docs.racket-lang.org/rackunit/)などがその例です。しかし、RackUnitは移植性に欠け、やや冗長な部分もあります。このフレームワークとRackUnitの間にブリッジがあれば、RackUnitのテストをこのフレームワークで実行したり、その逆も可能になるでしょう。また、Javaの「標準」[JUnit](https://www.junit.org/) APIへのSchemeインターフェースを提供するSchemeラッパーも少なくとも1つ存在します。このフレームワークで書かれたテストをJUnitランナーで実行できるようにブリッジがあれば便利でしょう。これらの機能はいずれもこの仕様には含まれていません。

このAPIは、暗黙的な「テストランナー」を含む、暗黙的な動的状態を利用します。これにより、APIは便利で簡潔に使用できますが、JUnitスタイルのフレームワークのような明示的なテストオブジェクトを使用する場合と比べて、やや洗練さや「構成性」に欠けるかもしれません。オブジェクト指向設計や関数型設計の原則に従うことを意図したものではありませんが、使いやすく、拡張しやすいものとなることを願っています。

この提案では、いくつかのマクロを追加するだけで、Schemeソースファイルをテストスイートに変換できます。ファイル全体を新しい形式で書き直す必要がないため、インデントをやり直す必要もありません。

APIで定義されているすべての名前は、接頭辞「test-」で始まります。関数のような形式はすべて構文として定義されています。これらは関数、マクロ、または組み込み関数として実装できます。構文として指定する理由は、部分式を評価せずに特定のテストをスキップしたり、実装で行番号の表示や例外の捕捉などの機能を追加できるようにするためです。

* * *

次へ: [SRFI-64 条件付きテスト スイートとその他の高度な機能](#75374-srfi-64-条件付きテスト-スイートとその他の高度な機能)、前: [SRFI-64 の根拠](#75372-srfi-64-理論的根拠)、上: [SRFI-64: テスト スイート用の Scheme API](#7537-srfi-64-テストスイート用の-scheme-api) \[[目次](00_contents.md "目次内容")\]\[[索引](index_r5rs.md "索引")\]

#### 7.5.37.3 SRFI-64 基本的なテストスイートの作成

まずは簡単な例から始めましょう。これは完全に自己完結型のテストスイートです。

;; シンプルなテストスイートを初期化して名前を付けます。
([test-begin](#テストグループとパス) "vec-test")
(define v ([make-vector](06_06_10_vectors.md#66102-動的ベクトルの作成と検証) 5 99))
式が真と評価されることを要求します。
([test-assert](#シンプルなテストケース) ([vector?](06_06_10_vectors.md#66102-動的ベクトルの作成と検証) v))
;; ある式が他の式と等価であるかどうかをテストします。
([test-eqv](#シンプルなテストケース) 99 ([vector-ref](06_06_10_vectors.md#66103-ベクトルコンテンツへのアクセスと変更) v 2))
([vector-set!](06_06_10_vectors.md#66103-ベクトルコンテンツへのアクセスと変更) v 2 7)
([test-eqv](#シンプルなテストケース) 7 ([vector-ref](06_06_10_vectors.md#66103-ベクトルコンテンツへのアクセスと変更) v 2))
;; テストスイートを完了し、結果を報告します。
([test-end](#テストグループとパス) "vec-test")

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

#### シンプルなテストケース

基本的なテストケースは、特定の条件が真であることをテストします。テストケースには名前が付けられる場合があります。基本的なテストケースの形式は `test-assert` です。

Scheme構文: **test-assert** \[test-name\] 式

これは式を評価します。結果が真であればテストは合格です。結果が偽であれば、テスト失敗が報告されます。例外が発生した場合も、実装に例外をキャッチする方法があれば、テストは失敗します。失敗の報告方法は、テスト ランナーの環境によって異なります。test-name は、テスト ケースの名前を表す文字列です。（例では test-name は文字列リテラルですが、式です。一度だけ評価されます。）これは、エラーを報告するとき、および後述するようにテストをスキップするときに使用されます。現在のテスト ランナーがない場合に `test-assert` を呼び出すとエラーになります。

以下の形式は、`test-assert`を直接使用するよりも便利な場合があります。

Scheme構文: **test-eqv** \[test-name\] は test-expr を必要とします

これは以下と同等です。

([test-assert](#シンプルなテストケース) \[test-name\] ([eqv?](06_09_general_utility_functions.md#691-平等) 期待されるテスト式))

同様に、`test-equal`と`test-eq`は、それぞれ`test-assert`と`equal?`または`eq?`を組み合わせたものの省略形です。

Scheme構文: **test-equal** \[test-name\] 期待されるtest-expr

Scheme構文: **test-eq** \[test-name\] 期待されるtest-expr

簡単な例を挙げましょう。

(define (mean xy) ([/](06_06_02_numerical_data_types.md#66211-算術関数) ([+](06_06_02_numerical_data_types.md#66211-算術関数) xy) 2.0))
([test-eqv](#シンプルなテストケース) 4 (平均 3 5))

不正確な実数の近似等価性をテストするには、`test-approximate`を使用できます。

Scheme構文: **test-approximate** \[test-name\] は test-expr を必要とします エラー

これは以下と同等です（ただし、各引数は一度だけ評価されます）。

([test-assert](#シンプルなテストケース) \[test-name\]
(および ([\>=](06_06_02_numerical_data_types.md#6628-比較述語) test-expr ([\-](06_06_02_numerical_data_types.md#66211-算術関数) は [error](04_programming_in_scheme.md#4446-デバッグコマンド)))
([<=](06_06_02_numerical_data_types.md#6628-比較述語) test-expr ([+](06_06_02_numerical_data_types.md#66211-算術関数) expected [error](04_programming_in_scheme.md#4446-デバッグコマンド)))))

以下に例を示します。

([test-approximate](#シンプルなテストケース) "22/7 は π の 1% 以内ですか?
3.1415926535
22/7
1/100)

#### エラー検出のためのテスト

評価が失敗するべきであることを明示的に指定する方法が必要です。これにより、必要な場合にエラーが検出されることが確認できます。

Scheme構文: **test-error** \[\[test-name\] error-type\] test-expr

test-expr の評価ではエラーが発生することが想定されます。エラーの種類は error-type で示されます。

エラータイプが省略されている場合、または`#t`の場合は、「何らかの未指定のエラーを通知する必要がある」ことを意味します。例：

([test-error](#エラー検出のためのテスト) #t ([vector-ref](06_06_10_vectors.md#66103-ベクトルコンテンツへのアクセスと変更) '#(1 2) 9))

この仕様では、テストエラーの形式は実装依存（または将来の仕様で規定）としていますが、すべての実装で `#t` を許可する必要があります。一部の実装では [SRFI-35 の条件](https://srfi.schemers.org/srfi-35/srfi-35.html) をサポートする場合がありますが、これらは [SRFI-36 の I/O 条件](https://srfi.schemers.org/srfi-36/srfi-36.html) に対してのみ標準化されており、テストスイートではほとんど役に立ちません。実装によっては、実装固有の「例外タイプ」を許可することもできます。たとえば、Java ベースの実装では、Java 例外クラスの名前を許可できます。

;; カワ固有の例
([test-error](#エラー検出のためのテスト) <java.lang.IndexOutOfBoundsException> ([vector-ref](06_06_10_vectors.md#66103-ベクトルコンテンツへのアクセスと変更) '#(1 2) 9))

例外を捕捉できない実装では、`test-error` フォームをスキップする必要があります。

GuileのSRFI-64実装では、エラータイプを以下のように指定できます。

* `#f`は、テストでエラーが発生することが想定されていないことを意味します。
* `#t`は、テストで何らかのエラーが発生することが予想されることを意味します。
* SRFI-35 の `make-exception-type` または `make-condition-type` で作成されたネイティブ例外タイプ
* キャッチされた例外に適用され、それが正しい型であるかどうかを判断する述語
* 例外タイプのレガシーな `make-exception-from-throw` スタイルの例外を表すシンボル。

以下は、Guileで有効な例です。

([test-error](#エラー検出のためのテスト) "旧式の例外タイプを期待する"
数値オーバーフロー
([/](06_06_02_numerical_data_types.md#66211-算術関数) 1 0))

([use-modules](06_18_modules.md#6182-guileモジュールの使用) (ice-9 例外)) ;標準例外タイプの場合
([test-error](#エラー検出のためのテスト) "ネイティブ例外タイプを期待する"
[&warning](07_06_r6rs_support.md#76213-rnrs-条件)
([raise-exception](06_11_controlling_the_flow_of_program_execution.md#61182-例外の発生と処理) ([make-warning](06_11_controlling_the_flow_of_program_execution.md#61181-例外オブジェクト))))

([test-error](#エラー検出のためのテスト) "述語を使用してネイティブ例外を期待する"
[警告?](06_11_controlling_the_flow_of_program_execution.md#61181-例外オブジェクト)
([raise-exception](06_11_controlling_the_flow_of_program_execution.md#61182-例外の発生と処理) ([make-warning](06_11_controlling_the_flow_of_program_execution.md#61181-例外オブジェクト))))

([use-modules](06_18_modules.md#6182-guileモジュールの使用) (srfi srfi-35))

([test-error](#エラー検出のためのテスト) "深刻なSRFI 35条件タイプを想定"
[&serious](07_06_r6rs_support.md#76213-rnrs-条件)
([raise-Exception](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-raise_002dException) ([条件](07_05_24_srfi35_conditions.md#7524-srfi-35---条件) ([&serious](07_06_r6rs_support.md#76213-rnrs-条件))))

([test-error](#エラー検出のためのテスト) "述語を使用して、深刻な SRFI 35 条件タイプを想定"
[serious-condition?](07_05_24_srfi35_conditions.md#7524-srfi-35---条件)
([raise-Exception](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-raise_002dException) ([条件](07_05_24_srfi35_conditions.md#7524-srfi-35---条件) ([&serious](07_06_r6rs_support.md#76213-rnrs-条件))))

#### 構文のテスト

構文のテストは難しいものです。特に、無効な構文がエラーの原因となっているかどうかを確認したい場合はなおさらです。以下のユーティリティ関数が役立ちます。

Scheme手順: **test-read-eval-string** string

この関数は文字列を解析し（`read`を使用）、結果を評価します。評価結果は`test-read-eval-string`から返されます。`read`の後に未読の文字がある場合はエラーが通知されます。例：`(test-read-eval-string "(+ 3 4)")`は`7`に評価されます。`(test-read-eval-string "(+ 3 4")`はエラーを通知します。`(test-read-eval-string "(+ 3 4) ")`は、リストの読み取り後に余分な「ジャンク」（つまりスペース）があるため、エラーを通知します。

テストで使用される`test-read-eval-string`：

([test-equal](#シンプルなテストケース) 7 ([test-read-eval-string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- test_002dread_002deval_002dstring) "(+ 3 4)"))
([test-error](#エラー検出のためのテスト) ([test-read-eval-string](#構文のテスト) "(+ 3"))
([test-equal](#シンプルなテストケース) #\\newline ([test-read-eval-string](#構文のテスト) "#\\\\newline"))
([test-error](#エラー検出のためのテスト) ([test-read-eval-string](#構文のテスト) "#\\\\newlin"))
;; srfi-62 が利用可能でない限り、次の 2 つのテストをスキップします。
([test-skip](#選択したテストをスキップする) ([cond-expand](07_05_02_subsection_2.md#752-srfi-0---cond-expand-) (srfi-62 0) (else 2)))
([test-equal](#シンプルなテストケース) 5 ([test-read-eval-string](#構文のテスト) "(+ 1 #;(\* 2 3) 4)"))
([test-equal](#シンプルなテストケース) '(xz) (test-read-string "(list 'x #;'y 'z)"))

#### テストグループとパス

テストグループとは、テストケース、式、定義を含むフォームの名前付きシーケンスです。グループに入るとテストグループ名が設定され、グループから出ると以前のグループ名に戻ります。これらは動的（実行時）操作であり、グループにはその他の効果や識別情報はありません。テストグループは非公式なグループ分けであり、Scheme の値でも構文形式でもありません。テストグループには、ネストされた内部テストグループを含めることができます。テストグループパスは、現在アクティブな（入っている）テストグループ名のリストであり、最も古い（最も外側の）ものから順に並んでいます。

Scheme構文: **test-begin** suite-name \[count\]

`test-begin`を実行すると、新しいテストグループが開始されます。suite-nameは現在のテストグループ名となり、テストグループパスの末尾に追加されます。移植性の高いテストスイートでは、 suite-nameに文字列リテラルを使用する必要があります。式やその他の種類のリテラルを使用した場合の効果は規定されていません。

理由：記号を使う方が望ましい場合もあるでしょう。しかし、人間が読みやすい名前が必要であり、標準のSchemeでは、リテラル記号にスペースや大文字小文字を混在させる方法がありません。

オプションのカウントは、このグループによって実行されるテストケースの数と一致する必要があります。（ネストされたテストグループは、このカウントでは1つのテストケースとしてカウントされます。）この追加テストは、予期しないエラーのためにテストが実行されなかった場合を検出するのに役立つ場合があります。

さらに、現在実行中のテストランナーが存在しない場合は、実装定義の方法でインストールされます。

Scheme構文: **test-end** \[suite-name\]

`test-end`を実行すると、現在のテストグループから離脱します。スイート名が現在のテストグループ名と一致しない場合は、エラーが報告されます。

さらに、一致する`test-begin`が新しいテストランナーをインストールした場合、`test-end`は実装定義の方法で蓄積されたテスト結果を報告した後、それをアンインストールします。

Scheme構文: **test-group** suite-name decl-or-expr …

同等の項目:

(if ([not](06_06_01_booleans.md#661-ブール値) (test-to-skip% (var suite-name)))
([dynamic-wind](06_11_controlling_the_flow_of_program_execution.md#61110-ダイナミックウィンド)
(lambda () ([test-begin](#テストグループとパス) (var suite-name)))
(lambda () (var decl-or-expr) [...](06_08_macros.md#6821-パターン))
(lambda () ([test-end](#テストグループとパス) (var suite-name)))))

これは通常、指定されたテストグループ内で宣言または式を実行することと同等です。ただし、アクティブな `test-skip` に一致した場合は、グループ全体がスキップされます（後述）。また、例外が発生した場合は `test-end` が実行されます。

#### セットアップとクリーンアップの処理

Scheme構文: **test-group-with-cleanup** suite-name decl-or-expr … cleanup-form

宣言式または式の各形式を（<body> のように）順番に実行し、その後、クリーンアップ形式を実行します。宣言式または式のいずれかで例外が発生した場合でも、クリーンアップ形式は実行される必要があります（実装に例外を捕捉する方法がある場合）。

例えば：

(let ((f ([open-output-file](06_12_input_and_output.md#612101-ファイルポート) "log")))
([test-group-with-cleanup](#セットアップとクリーンアップの処理) "test-file"
（たくさんのテストを実行する f）
([close-output-port](07_06_r6rs_support.md#76218-rnrs-io-simple) f)))

* * *

次へ: [SRFI-64 テスト ランナー](#75375-srfi-64-テストランナー)、前: [SRFI-64 基本的なテスト スイートの作成](#75373-srfi-64-基本的なテストスイートの作成)、上: [SRFI-64: テストスイート用の Scheme API](#7537-srfi-64-テストスイート用の-scheme-api) \[[目次](00_contents.md "目次内容")\]\[[索引](index_r5rs.md "索引")\]

#### 7.5.37.4 SRFI-64 条件付きテスト スイートとその他の高度な機能

以下では、実行するテストを制御する機能、または一部のテストが失敗すると想定される機能について説明します。

#### テスト指定子

特定のテストのみを実行したい場合や、特定のテストが失敗することがわかっている場合があります。テスト指定子は、テストランナーを受け取り、ブール値を返す引数1つの関数です。指定子はテスト実行前に実行でき、その結果によってテストの実行を制御できます。便宜上、指定子は非プロシージャ値にすることもできます。その場合、countとnameについて後述するように、指定子プロシージャに強制変換されます。

簡単な例を挙げると次のようになります。

(if (var some-condition) ([test-skip](#選択したテストをスキップする) 2)) ;; 次の 2 つのテストをスキップする

スキーム手順: **test-match-name** name

結果として得られる指定子は、現在のテスト名（`test-runner-test-name` によって返されるもの）が name と `equal?` である場合に一致します。

Scheme構文: **test-match-nth** n \[count\]

これは_状態を持つ_述語に評価されます。カウンターは、呼び出された回数を追跡します。述語は、n回目の呼び出し（`1`が最初の呼び出し）と、次の'(- count 1)'回の呼び出しに一致します。ここで、countのデフォルト値は`1`です。

Scheme構文: **test-match-any**指定子 …

結果として得られる指定子は、いずれかの指定子が一致した場合に一致します。各指定子は順番に適用されるため、前の指定子が真であっても、後の指定子による副作用が発生します。

Scheme構文: **test-match-all**指定子 …

結果として得られる指定子は、各指定子が一致する場合に一致します。各指定子は順番に適用されるため、前の指定子が偽であっても、後の指定子による副作用が発生します。

count _(つまり整数)_ '(test-match-nth 1 count)' の便利な省略形。

name _(文字列)_ '(test-match-name name)' の便利な省略形。

#### 選択したテストをスキップする

場合によっては、テストをスキップしたい場合もあるでしょう。

Scheme構文: **test-skip**指定子

`test-skip` を評価すると、結果として得られる指定子が現在有効なスキップ指定子のセットに追加されます。各テスト（または `test-group`）の前に、有効なスキップ指定子のセットが有効なテストランナーに適用されます。いずれかの指定子が一致する場合、テストはスキップされます。

便宜上、指定子が `(test-match-name specifier)` の構文糖衣である文字列である場合。例:

([test-skip](#選択したテストをスキップする) "test-b")
([test-assert](#シンプルなテストケース) "test-a") ;; 実行済み
([test-assert](#シンプルなテストケース) "test-b") ;; スキップ

`test-skip` によって導入されたスキップ指定子は、後続のネストされていない `test-end` によって削除されます。

([test-begin](#テストグループとパス) "group1")
([test-skip](#選択したテストをスキップする) "test-a")
([test-assert](#シンプルなテストケース) "test-a") ;; スキップ
([test-end](#テストグループとパス) "group1") ;; 前の test-skip を取り消す
([test-assert](#シンプルなテストケース) "test-a") ;; 実行済み

#### 想定される障害

テストケースが失敗すると分かっていても、修正する時間がない、あるいは修正できない場合があります。例えば、特定の機能が特定のプラットフォームでしか動作しない場合などです。しかし、修正を忘れないように、テストケースを残しておきたいと思うでしょう。そのようなテストは失敗することが想定されている、ということを記録しておきたいのです。

Scheme構文: **test-expect-fail**指定子

一致するテスト（一致の定義は`test-skip`で定義されている）は失敗することが想定されています。これはテストの実行ではなく、テストレポートにのみ影響します。例：

([test-expect-fail](#想定される障害) 2)
([test-eqv](#シンプルなテストケース) [...](06_08_macros.md#6821-パターン)) ;; 失敗すると予想される
([test-eqv](#シンプルなテストケース) [...](06_08_macros.md#6821-パターン)) ;; 失敗すると予想される
([test-eqv](#シンプルなテストケース) [...](06_08_macros.md#6821-パターン)) ;; 合格が期待される

* * *

次へ: [SRFI-64 テスト結果](#75376-srfi-64-テスト結果)、前: [SRFI-64 条件付きテスト スイートとその他の高度な機能](#75374-srfi-64-条件付きテスト-スイートとその他の高度な機能)、上: [SRFI-64: テスト スイート用の Scheme API](#7537-srfi-64-テストスイート用の-scheme-api) \[[目次](00_contents.md "目次内容")\]\[[索引](index_r5rs.md "索引")\]

#### 7.5.37.5 SRFI-64 テストランナー

テストランナーは、テストスイートを実行し、状態を管理するオブジェクトです。テストグループパス、スキップおよび期待失敗指定子のセットは、テストランナーの一部です。テストランナーは通常、実行されたテストに関する統計情報も収集します。

Scheme Procedure: **test-runner?** value

値がテストランナーオブジェクトである場合に限り、真となります。

スキームパラメータ: **test-runner-current** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- test_002drunner_002dcurrent)

スキームパラメータ: **test-runner-current** ランナー

現在のテストランナーを取得または設定します。

Scheme Procedure: **test-runner-get**

`(test-runner-current)` と同じですが、現在のテストランナーがない場合は例外をスローします。

Scheme手順: **test-runner-simple**

標準出力ポートにエラーと概要を出力する、新しいシンプルなテストランナーを作成します。

Scheme Procedure: **test-runner-null**

テスト結果を一切処理しない新しいテストランナーを作成します。これは主に、カスタムランナーを作成する際に拡張することを目的としています。

Scheme手順: **test-runner-create**

新しいテストランナーを作成します。'((test-runner-factory))' と同等です。

スキームパラメータ: **test-runner-factory**

スキームパラメータ: **test-runner-factory** ファクトリ

現在のテストランナーファクトリを取得または設定します。ファクトリとは、新しいテストランナーを作成する引数なしの関数です。デフォルト値は`test-runner-simple`です。

#### 特定のランナーを使用して特定のテストを実行する

Scheme プロシージャ: **test-apply** \[runner\] 指定子 … プロシージャ

指定されたランナーを現在のテストランナーとして使用し、引数なしでプロシージャを呼び出します。ランナーが省略された場合は、`(test-runner-current)` が使用されます。（現在のランナーがない場合は、`test-begin` と同様に作成されます。） 1 つ以上の指定子がリストされている場合は、指定子に一致するテストのみが実行されます。指定子は、`test-skip` で使用されるものと同じ形式です。テストは、`test-apply` のいずれかの指定子に一致し、かつアクティブな `test-skip` 指定子に一致しない場合に実行されます。

Scheme構文: **test-with-runner** runner decl-or-expr …

現在のテストランナーが runner であるコンテキストで、各宣言または式を順番に実行します。

* * *

次へ: [SRFI-64 新しいテスト ランナーの作成](#75377-srfi-64-新しいテストランナーの作成)、前: [SRFI-64 テスト ランナー](#75375-srfi-64-テストランナー)、上: [SRFI-64: テスト スイート用の Scheme API](#7537-srfi-64-テストスイート用の-scheme-api) \[[目次](00_contents.md "目次内容")\]\[[索引](index_r5rs.md "索引")\]

#### 7.5.37.6 SRFI-64 テスト結果

テストを実行すると、現在のテストランナーにさまざまなステータスプロパティが設定されます。これは、カスタムテストランナー、または（まれに）テストスイートで確認できます。

#### 結果の種類

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

スキーム手順: **test-result-kind** \[runner\]

最新のテストから上記のいずれかの結果コードを返します。テストがまだ実行されていない場合は `#f` を返します。新しいテストを開始したが、まだ結果がない場合、テストが失敗すると予想される場合は `'xfail`、テストがスキップされるべき場合は `'skip`、それ以外の場合は `#f` を返します。

Scheme 手順: **test-passed?** \[runner\]

'(test-result-kind \[runner\])' の値が `'pass` または `'xpass` のいずれかである場合に真となります。これは、テストスイートにおいて、前のテストが合格した場合にのみ特定のテストを実行する場合に便利な省略記法です。

#### テスト結果のプロパティ

テストランナーは、現在または最新のテストに関連付けられた、より詳細な「結果プロパティ」のセットも保持します。（つまり、新しいテストが開始されていない限り、最新のテストのプロパティが利用可能です。）各プロパティには、名前（シンボル）と値（任意の値）があります。一部のプロパティは標準で定義されているか、実装によって設定されます。実装によっては、プロパティを追加することもできます。

Scheme Procedure: **test-result-ref** runner pname \[default\]

pname プロパティ名（シンボル）に関連付けられたプロパティ値を返します。pname に関連付けられた値がない場合はデフォルト値を返し、デフォルト値が指定されていない場合は `#f` を返します。

Scheme構文: **test-result-set!** runner pname value

pnameプロパティ名に関連付けられたプロパティ値を値に設定します。通常は実装コードからこの関数を呼び出す必要がありますが、カスタムテストランナーが追加のプロパティを追加する場合にも役立つ場合があります。

Scheme Procedure: **test-result-remove** runner pname [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- test_002dresult_002dremove)

pnameという名前のプロパティを削除します。

Scheme Procedure: **test-result-clear** runner

結果プロパティをすべて削除します。実装では、`test-assert` および同様の手順の開始時に `test-result-clear` が自動的に呼び出されます。

Scheme Procedure: **test-result-alist** runner

現在の結果プロパティの関連付けリストを返します。結果がテストランナーと状態を共有するかどうかは指定されていません。結果は変更すべきではありませんが、将来の `test-result-set!` または `test-result-remove` 呼び出しによって暗黙的に変更される可能性があります。ただし、`test-result-clear` は返される alist を変更しません。したがって、以前の実行結果オブジェクトを「アーカイブ」することができます。

#### 標準結果プロパティ

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

前へ: [SRFI-64 テスト結果](#75376-srfi-64-テスト結果)、上へ: [SRFI-64: テストスイート用の Scheme API](#7537-srfi-64-テストスイート用の-scheme-api) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.5.37.7 SRFI-64 新しいテストランナーの作成

このセクションでは、テストランナーの書き方について説明します。テストケースだけを記述したい場合は、このセクションは無視しても構いません。

#### コールバック関数

これらのコールバック関数は、テストランナーの「メソッド」（オブジェクト指向的な意味で）です。`test-runner-on-event` メソッドは、イベントが発生したときに実装によって呼び出されます。

イベントのコールバック関数を定義（設定）するには、次の式を使用します。（これは通常、テストランナーを初期化する際に行います。）

`(test-runner-on-event! runner event-function)`

イベント関数は、テストランナー引数を受け取り、イベントによってはその他の引数を受け取る場合もあります。

イベントのコールバック関数を抽出（取得）するには、次のようにします。`(test-runner-on-event runner)`

イベントのコールバック関数を呼び出すには、次の式を使用します。（これは通常、実装コアによって行われます。） '((test-runner-on-event runner) runner other-args …)'。

以下のコールバックフックが利用可能です。

Scheme Procedure: **test-runner-on-test-begin** runner

Scheme Procedure: **test-runner-on-test-begin!** runner on-test-begin-function

スキームプロシージャ: **on-test-begin-function** ランナー

on-test-begin関数は、個々のテストケースの開始時に、テスト式（および期待値）が評価される前に呼び出されます。

Scheme Procedure: **test-runner-on-test-end** runner

Scheme Procedure: **test-runner-on-test-end!** runner on-test-end-function

スキーム手順: **on-test-end-function** ランナー

on-test-end関数は、個々のテストケースの最後に、テスト結果が利用可能になった時点で呼び出されます。

Scheme Procedure: **test-runner-on-group-begin** runner

Scheme Procedure: **test-runner-on-group-begin!** runner on-group-begin-function

Scheme Procedure: **on-group-begin-function** runner suite-name count

on-group-begin-function は、`test-begin` によって呼び出されます。これには、`test-group` の開始時も含まれます。suite-name は Scheme 文字列で、count は整数または `#f` です。

Scheme Procedure: **test-runner-on-group-end** runner

Scheme Procedure: **test-runner-on-group-end!** runner on-group-end-function

Scheme プロシージャ: **on-group-end-function** ランナー

on-group-end-function は、`test-end` によって呼び出されます。これには、`test-group` の末尾での呼び出しも含まれます。

スキーム手順: **test-runner-on-bad-count** ランナー

Scheme Procedure: **test-runner-on-bad-count!** runner on-bad-count-function

Scheme Procedure: **on-bad-count-function** runner actual-count expected-count

`test-end` から呼び出されます (on-group-end-function が呼び出される前)。これは、一致する `test-begin` で期待されるカウントが指定されており、その期待されるカウントが実際に実行またはスキップされたテストの実際のカウントと一致しない場合に呼び出されます。

スキーム手順: **test-runner-on-bad-end-name** ランナー

Scheme Procedure: **test-runner-on-bad-end-name!** runner on-bad-end-name-function

Scheme Procedure: **on-bad-end-name-function** runner begin-name end-name [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- on_002dbad_002dend_002dname_002dfunction)

`test-end` から呼び出されます (on-group-end-function が呼び出される前)。スイート名が指定されており、一致する `test-begin` の名前がなかった場合に呼び出されます。

Scheme Procedure: **test-runner-on-final** runner

Scheme Procedure: **test-runner-on-final!** runner on-final-function

スキームプロシージャ: **on-final-function** ランナー

on-final関数は1つのパラメータ（テストランナー）を受け取り、通常はテストの概要（カウント）を表示します。on-final関数は、最も外側の`test-end`に対応するon-group-end関数が呼び出された後に呼び出されます。デフォルト値は`test-on-final-simple`で、これは各種テストの数を標準出力ポートに出力します。

`test-runner-simple` が返すデフォルトのテストランナーは、以下のコールバック関数を使用します。

Scheme Procedure: **test-on-test-begin-simple** runner

Scheme Procedure: **test-on-test-end-simple** runner

Scheme Procedure: **test-on-group-begin-simple** runner suite-name count

Scheme 手順: **test-on-group-end-simple** ランナー

スキーム手順: **test-on-bad-count-simple** runner actual-count expected-count

Scheme Procedure: **test-on-bad-end-name-simple** runner begin-name end-name

独自のテストランナーを作成したい場合は、それらを呼び出すことができます。

#### テストランナーのコンポーネント

以下の関数は、テストランナーの他のコンポーネントにアクセスするためのものです。これらは通常、新しいテストランナーまたはマッチ述語を作成する場合にのみ使用されます。

スキーム手順: **test-runner-pass-count** ランナー

合格したテストの数と、合格すると予想されていたテストの数を返します。

スキーム手順: **test-runner-fail-count** ランナー

合格するはずだったが、実際には失敗したテストの数を返します。

スキーム手順: **test-runner-xpass-count** ランナー

合格したが、不合格になると予想されていたテストの数を返します。

スキーム手順: **test-runner-xfail-count** ランナー

合格するはずだったテストのうち、失敗したテストの数を返します。

スキーム手順: **test-runner-skip-count** ランナー

スキップされたテストまたはテストグループの数を返します。

Scheme Procedure: **test-runner-test-name** runner

現在のテストまたはテストグループの名前を文字列として返します。`test-begin` の実行中はテストグループの名前、実際のテストの実行中はテストケースの名前になります。名前が指定されていない場合は、空の文字列が返されます。

Scheme Procedure: **test-runner-group-path** runner

私たちが属しているグループの名前のリスト。一番外側のグループが最初に表示されます。

スキーム手順: **test-runner-group-stack** ランナー

ネストされているグループ名のリスト。一番外側のグループが最後に表示されます。（コピー処理が不要なため、`test-runner-group-path`よりも効率的です。）

スキーム手順: **test-runner-aux-value** ランナー

スキーム手順: **test-runner-aux-value!** テスト時のランナー

テストランナーの`aux-value`フィールドを取得または設定します。このフィールドは、このAPIや`test-runner-simple`テストランナーでは使用されませんが、カスタムテストランナーが追加の状態を保存するために使用できます。

Scheme Procedure: **test-runner-reset** runner

ランナーの状態を初期状態にリセットします。

#### 例

これはシンプルなカスタムテストランナーの例です。テストスイートを実行する前にこのプログラムを読み込むと、デフォルトのテストランナーとしてインストールされます。

(define (my-simple-runner filename)
(let ((runner ([test-runner-null](#75375-srfi-64-テストランナー)))
	(ポート ([open-output-file](06_12_input_and_output.md#612101-ファイルポート) filename))
(渡された数値 0)
(失敗数 0)
([test-runner-on-test-end!](#コールバック関数) runner
(ラムダ (ランナー)
(case ([test-result-kind](#結果の種類) runner)
((pass xpass) ([set!](07_06_r6rs_support.md#7622-rnrs-ベース) num-passed ([+](06_06_02_numerical_data_types.md#66211-算術関数) num-passed 1)))
((fail xfail) ([set!](07_06_r6rs_support.md#7622-rnrs-ベース) num-failed ([+](06_06_02_numerical_data_types.md#66211-算術関数) num-failed 1)))
(それ以外の場合 #t))))
([test-runner-on-final!](#コールバック関数) runner
(ラムダ (ランナー)
([format](07_05_20_srfi28_basic_format_strings.md#7520-srfi-28---基本フォーマット文字列) port "テスト合格: ~d.~% テスト不合格: ~d.~%"
合格数 不合格数
	([close-output-port](07_06_r6rs_support.md#76218-rnrs-io-simple) port)))
ランナー））
([test-runner-factory](#75375-srfi-64-テストランナー)
(lambda () (my-simple-runner "/tmp/my-test.log")))

* * *

次へ: [SRFI-69 - 基本的なハッシュテーブル](07_05_39_srfi69_basic_hash_tables.md#7539-srfi-69---基本ハッシュテーブル)、前: [SRFI-64: テストスイート用の Scheme API](#7537-srfi-64-テストスイート用の-scheme-api)、上: [SRFI サポートモジュール](07_05_00_srfi_support_modules.md#75-srfi-サポート-モジュール) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
