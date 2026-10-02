### 6.27 コードカバレッジレポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Code-Coverage-Reports)

プログラムやライブラリのテストスイートを作成する際には、コードのどの部分がテストスイートによってカバーされているかを知ることが望ましいです。`(system vm coverage)`モジュールは、コードカバレッジデータを収集し、それを表示するためのツールを提供します。詳細は以下をご覧ください。

Scheme 手順: **with-code-coverage** thunk [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-with_002dcode_002dcoverage)

Guileの仮想マシンを計測してコードカバレッジデータを収集しながら、引数なしのプロシージャであるthunkを実行します。コードカバレッジデータとthunkが返す値を返します。

スキーム手順: **coverage-data?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-coverage_002ddata_003f)

objが`with-code-coverage`によって返される_coverageデータ_オブジェクトである場合は、`#t`を返します。

スキーム手順: **coverage-data->lcov** データポート番号:キーモジュール [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-coverage_002ddata_002d_003elcov)

`with-code-coverage`で取得したコードカバレッジ情報データを走査し、[LCOV](http://ltp.sourceforge.net/coverage/lcov.php)で使用される`.info`形式でカバレッジ情報をポートに書き込みます。レポートには、コードが実行されなかった場合でも、すべてのモジュール（デフォルトでは現在ロードされているすべてのモジュール）が含まれます。

生成されたデータは、LCOVの`genhtml`コマンドに渡すことでHTMLレポートを生成でき、これによりカバレッジデータの視覚化に役立ちます。

使用例を以下に示します。

(use-modules (system vm coverage)
(システム VM VM)

(call-with-values (lambda ()
(コードカバレッジ付き)
(ラムダ()
(何か巧妙なことをする))))
(ラムダ式 (データ結果)
(let ((port (open-output-file "lcov.info")))
（カバレッジデータ→lcovデータポート）
(ポートを閉じる))))

さらに、このモジュールは、カバレッジデータに対する他のユーザーインターフェースを作成できるようにする低レベルの手順も提供します。

Scheme 手順: **instrumented-source-files** データ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-instrumented_002dsource_002dfiles)

データ収集時にコードがロードされていたソースファイル、つまり「計測対象」のソースファイルのリストを返します。

Scheme手順: **line-execution-counts**データファイル[¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-line_002dexecution_002dcounts)

ファイルの行番号と実行回数のペアのリストを返します。ファイルがデータ対象ファイルに含まれていない場合は、`#f`を返します。これには、実行回数がゼロの行も含まれます。

Scheme手順: **instrumented/executed-lines**データファイル[¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-instrumented_002fexecuted_002dlines)

データに基づいて、ファイル内の計測対象となったソース行数と実行されたソース行数を返します。

Scheme プロシージャ: **procedure-execution-count** data proc [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-procedure_002dexecution_002dcount)

データに基づいて、proc のコードが実行された回数を返します。proc が実行されなかった場合は `#f` を返します。proc がクロージャの場合、返されるのはクロージャ自体のコードが実行された回数であり、この特定のクロージャに関連付けられたコードが実行された回数ではありません。

* * *

次へ: [GOOPS](https://doc.guix.gnu.org/guile/latest/en/guile.html#GOOPS)、前: [API リファレンス](https://doc.guix.gnu.org/guile/latest/en/guile.html#API-Reference)、上: [Guile リファレンス マニュアル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Top) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
