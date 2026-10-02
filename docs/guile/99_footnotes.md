##### [(1)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF1)

「 ice-9」は、カート・ヴォネガットの小説『猫のゆりかご』に登場する架空の物質を指しています（[ステータス、または：あなたの助けが必要です](https://doc.guix.gnu.org/guile/latest/en/guile.html#Status)を参照）。

##### [(2)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF2)

これらの定義は概算です。完全かつ詳細な情報については、アルゴリズム言語スキームの改訂版(5)レポートの[R5RS構文](https://doc.guix.gnu.org/r5rs/latest/en/r5rs.html#Formal-syntax-and-semantics)を参照してください。

##### [(3)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF3)

`GUILE_INSTALL_LOCALE`環境変数は、Guileのバージョン2.0.9より前のバージョンでは無視されていました。

##### [(4)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF4)

`guile` および `guild` 変数は、Guile バージョン 2.0.12 以降で定義されています。

##### [(5)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF5)

Guile は C のチープをスキャンして参照を検索しないため、`malloc` で割り当てられたメモリ セグメントから `SCM` オブジェクトへの参照を取得するには、`SCM` オブジェクトを生存させるために別の手段を使用する必要があります。[ガベージ コレクションに関連する関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Garbage-Collection-Functions) を参照してください。

##### [(6)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF6)

Guile 1.8では、Guileモードでスレッドがブロックされると、ガベージコレクションが実行されませんでした。そのため、スレッドはブロックできる場合は常にGuileモードを解除する必要がありました。Guile 2.xでは、この操作は不要になりました。

##### [(7)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF7)

ホワイトボックステスト計画とは、テスト対象アプリケーションの内部設計に関する知識を組み込んだテスト計画のことです。

##### [(8)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF8)

もちろん、フリーソフトウェアの世界では、アプリケーションのソースコードを自分のニーズに合わせて自由に修正することができます。しかし、ここでは、ソースコードを修正することなく利用できる、アプリケーションが提供する拡張機能について考察します。

##### [(9)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF9)

厳密に言えば、Schemeには実際のデータ型「リスト」は存在しません。リストは「連鎖したペア」で構成されており、定義上のみ存在します。つまり、リストとは、リストのように見えるペアの連鎖のことです。

##### [(10)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF10)

リストの要素間には、カンマやセミコロンなどの区切り文字がないことに注意してください。

##### [(11)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF11)

ビッグエンディアンとリトルエンディアンは最も一般的な「エンディアン方式」ですが、他にも存在します。例えば、GNU MPライブラリでは、バイト順とは独立してワード順を指定できます（GNU多倍長演算ライブラリマニュアルの「整数のインポートとエクスポート」を参照）。

##### [(12)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF12)

R6RS では `(bytevector-fill! bv fill)` のみが定義されています。引数 start と end は Guile の拡張機能です (cf. [`vector-fill!`](https://doc.guix.gnu.org/guile/latest/en/guile.html#x_002dvector_002dfill_0021)、[`string-fill!`](https://doc.guix.gnu.org/guile/latest/en/guile.html#x_002dstring_002dfill_0021))。

##### [(13)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF13)

作業上の定義は以下のとおりです。

(define foo-ref [vector-ref](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-vector_002dref))
(define foo-set! [vector-set!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-vector_002dset_0021))
(define f ([make-vector](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- make_002dvector) 2 #f))

##### [(14)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF14)

近年では、このような組み込み言語はしばしば「組み込みドメイン固有言語」、略してEDSLと呼ばれる。

##### [(15)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF15)

言語専門家は、ここでは `free-identifier=?` ではなく `literal-identifier=?` を使用する必要性を認識している可能性が高く、おそらくその通りでしょう。修正案は受け付けます。

##### [(16)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF16)

2013年5月3日に[オリジナル](http://sites.google.com/site/evalapply/eccentric.txt)からアーカイブされました。

##### [(17)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF17)

Barzilay、Culpepper、Flattによる論文「構文パラメータでクリーンな状態を保つ」で説明されている。

##### [(18)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF18)

Guileのバージョン1.8までは、C言語のグローバル変数はマークフェーズでガベージコレクタによって参照されませんでした。そのため、C言語でSchemeオブジェクトの解放を防ぐ唯一の方法は`scm_gc_protect_object`を使用することでした。

##### [(19)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF19)

Guile のバージョン 1.8 までは、`scm_gc_malloc` で割り当てられたメモリは `scm_gc_free` で解放する必要がありました。

##### [(20)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF20)

Guile 1.8 までは、`scm_gc_malloc` で割り当てられたメモリは、マークフェーズでコレクタによってアクセスされませんでした。そのため、メモリブロックに含まれる生存オブジェクトへのポインタについては、例えば SMOB マーク関数 ([`scm_set_smob_mark`](https://doc.guix.gnu.org/guile/latest/en/guile.html#Smobs) などを使用して、GC に明示的に通知する必要がありました。

##### [(21)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF21)

Guile 2.2以前のバージョンでは、_すべての_モジュールバインディングが利用可能になっていました。symbol-listは、最初にロードをトリガーするバインディングのリストにすぎませんでした。

##### [(22)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF22)

Guileへの高レベルFFIの貢献は大歓迎です。

##### [(23)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF23)

簡潔さとスタイルを保つため、プログラマーは国際化を「i18n」と呼ぶことが多い。

##### [(24)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF24)

`gettext` のユーザーの中には、`gettext` の一般的な略記が `G_` であることに少し驚く方もいるかもしれません。他のほとんどの言語では、一般的な略記は `_` です。Guile では `G_` を使用していますが、これは `_` が既に `syntax-rules`、`match`、その他のマクロで使用される構文キーワードに割り当てられているためです。

##### [(25)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF25)

このモジュールは、`popen`機能が提供されるシステムでのみ利用可能です（[共通機能シンボル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Common-Feature-Symbols)を参照）。

##### [(26)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF26)

はい、Pはプロトコルの略ですが、このフレーズはRFC 2616に繰り返し登場します。

##### [(27)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF27)

Unicode対応ポートでは、省略記号は「水平省略記号」（U+2026）という文字で表されますが、そうでない場合は3つの点で表されます。

##### [(28)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF28)

`~h` フォーマット指定子は、Guile バージョン 2.0.6 で初めて登場しました。

##### [(29)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF29)

この例は、Krishnamurthi らによる論文から引用したものです。彼らの論文は、XML の変換において構文規則に基づくパターンマッチングが有効であることを初めて示したもので、記述されている言語である XT3D は XML 言語です。

##### [(30)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF30)

通常はそうですが、`#:allocation` スロットオプションも参照してください。

##### [(31)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF31)

もちろん、Guileは既に複素数を提供しており、`<complex>`は実際にはGOOPSの事前定義されたクラスですが、ここでの定義は例として依然として有用です。

##### [(32)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF32)

`<number>` は定義済みのクラス `<complex>` の直接のスーパークラスです。`<complex>` は `<real>` のスーパークラスであり、`<real>` は `<integer>` のスーパークラスです。

##### [(33)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF33)

ただし、`(math 2D-vectors)` の `x` は `(math 3D-vectors)` の `x` とメソッドを共有しないため、モジュール性は維持されます。

##### [(34)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF34)

`define-method` のパラメータリストは、Scheme プロシージャで使用される規則に従います。特に、任意の数のパラメータを表すために、ドット表記またはシンボルを使用できます。

##### [(35)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF35)

このセクションは、ジェフ・ダルトン氏（J.Dalton@ed.ac.uk）によるCLOSの簡単な紹介資料を基に作成されています。

##### [(36)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF36)

PAIPは、Paradigms of Artificial Intelligence Programmingの略で、Lispに関する古いながらも今でも役立つテキストです。Norvigによる回顧録はPAIPの教訓をまとめたもので、[http://norvig.com/Lisp-retro.html](http://norvig.com/Lisp-retro.html)で読むことができます。

##### [(37)](https://doc.guix.gnu.org/guile/latest/en/guile.html#DOCF37)

最も低レベルの機械語コードでさえ、CPUによって解釈されるものと考えることができ、実際、多くの場合、機械語命令を「マイクロオペレーション」にコンパイルすることによって実装されます。
