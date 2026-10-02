### 9.3 Guile 用仮想マシン [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#A-Virtual-Machine-for-Guile-1)

データの話はこれくらいにして、Guileはどのようにコードを実行するのか？

コードは言語の文法的な表現です。これらの言語は、インタプリタ（解釈対象のプログラムと並行して実行され、高水準コードを低水準コードに動的に変換するプログラム）を用いて実装される場合もあれば、コンパイラ（高水準プログラムを同等の低水準コードに変換し、その低水準コードを他の言語実装に渡すプログラム）を用いて実装される場合もあります。これらの言語はそれぞれ仮想マシンと考えることができます。つまり、プログラムが実行するための抽象的なマシンを提供するのです。

Guile は、さまざまな言語レベルで多数のインタプリタとコンパイラを実装しています。たとえば、Scheme 言語用のインタプリタがあり、これは Guile に同梱されている低レベル仮想マシン用のバイトコードにコンパイルされた Scheme プログラムとして実装されています。この仮想マシンは、バイトコードを解釈する C プログラムであるインタプリタと、バイトコード プログラムをネイティブ マシン コードに動的に変換する C プログラムであるコンパイラの両方によって実装されています [37](https://doc.guix.gnu.org/guile/latest/en/guile.html#FOOT37)。

このセクションでは、Guileのバイトコード仮想マシンで実装されている言語について説明するとともに、SchemeプログラムをGuileのVMに変換した例をいくつか紹介します。

* [なぜVMを使うのか？](https://doc.guix.gnu.org/guile/latest/en/guile.html#Why-a-VM_003f)
* [VM の概念](https://doc.guix.gnu.org/guile/latest/en/guile.html#VM-Concepts)
* [スタックレイアウト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Stack-Layout)
* [変数とVM](https://doc.guix.gnu.org/guile/latest/en/guile.html#Variables-and-the-VM)
* [コンパイル済みプロシージャはVMプログラムです](https://doc.guix.gnu.org/guile/latest/en/guile.html#VM-Programs)
* [オブジェクトファイル形式](https://doc.guix.gnu.org/guile/latest/en/guile.html#Object-File-Format)
* [命令セット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instruction-Set)
* [ジャストインタイムネイティブコード](https://doc.guix.gnu.org/guile/latest/en/guile.html#Just_002dIn_002dTime-Native-Code)

* * *

次へ: [VM の概念](https://doc.guix.gnu.org/guile/latest/en/guile.html#VM-Concepts)、上へ: [Guile 用仮想マシン](https://doc.guix.gnu.org/guile/latest/en/guile.html#A-Virtual-Machine-for-Guile) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.3.1 VMを使う理由 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Why-a-VM_003f-1)

長らくGuileにはC言語で実装されたSchemeインタープリタしか存在しなかった。Guileのインタープリタは、SchemeソースコードのS式表現を直接操作していた。

しかし、このインタプリタは高度に最適化され、手作業で調整されていたものの、Scheme式の評価中に多くの不要な計算を実行していました。例えば、関数を引数に適用すると、引数がリストに不必要に蓄積されてしまいます。`(fxy)`のような式の評価では、fが手続きなのか、`if`のような特殊形式なのか、それとも他のものなのかを常に判断する必要がありました。インタプリタは字句環境をヒープデータ構造として表現していたため、評価のたびにメモリ割り当てが発生し、当然ながら処理速度が遅くなりました。その他にも様々な問題がありました。

低速なインタプリタの問題に対する解決策は、高水準言語であるSchemeを、すべてのチェックとディスパッチ処理が既に完了している低水準言語にコンパイルすることだった。つまり、コードは「仕事をこなす」ために必要な最小限の要素にまで削ぎ落とされる。

そこで問題となるのは、どの低レベル言語を選択するかということだ。選択肢はたくさんある。ネイティブコードに直接コンパイルすることもできるが、Guileはクロスプラットフォーム対応のプロジェクトであるため、移植性の問題が生じる。

コンパイルによるパフォーマンス向上は望ましいものの、単一のコードパスによる移植性も維持したい。そのための明白な解決策は、すべてのGuileインストール環境に存在する仮想マシンにコンパイルすることである。

仮想マシンに依存する最も簡単で楽しい方法は、Guile自体の中に仮想マシンを実装することです。Guileには、バイトコードインタープリタ（C言語で記述）とSchemeからバイトコードへのコンパイラ（Schemeで記述）が含まれています。この方法により、仮想マシンはSchemeが必要とする機能（末尾呼び出し、複数値、`call/cc`）を提供するだけでなく、Guile向けに最適化されたインライン命令（GC管理によるメモリ割り当て、型チェックなど）も提供できます。

Guileには、バイトコードをネイティブコードに変換するジャストインタイム（JIT）コンパイラも含まれています。Guileは移植性の高いコード生成ライブラリ（[https://gitlab.com/wingo/lightening](https://gitlab.com/wingo/lightening)）を組み込んでいるため、移植性の利点を維持しつつ、高速なネイティブコードのメリットも享受できます。JITコンパイラ自体の処理時間を過度に消費しないように、Guileは頻繁に呼び出されるバイトコードに対してのみマシンコードを出力するように調整されています。

このセクションの残りの部分では、Guileが実装するVMと、その上で実行されるコンパイル済みプロシージャについて説明します。

先に進む前に、過去形でインタープリタについて述べましたが、Guileには依然としてインタープリタが存在することを指摘しておくべきでしょう。違いは、以前はそれがGuileの主要なScheme実装であり、高度に最適化されたC言語で実装されていたのに対し、現在は他のプログラムと同様に、実際にSchemeで実装され、VMバイトコードにコンパイルされるようになった点です。（コンパイラの起動に使用されるC言語インタープリタもまだ存在しますが、通常は実行時には使用されません。）

Schemeでインタプリタを実装する利点は、インタプリタ化されたコードとコンパイルされたコードの間で末尾呼び出しと多値処理が維持されることであり、Guile 3.0でJITコンパイラが登場したことで、従来の手作業で調整されたC言語実装と同等の速度を実現できるようになったことです。まさに両方の長所を兼ね備えています。

また、バイトコードコンパイラを実装するというこの決定は、事前ネイティブコンパイルを排除するものではないことに注意してください。その他の可能性については、[コンパイラの拡張](https://doc.guix.gnu.org/guile/latest/en/guile.html#Extending-the-Compiler)で説明されています。

* * *

次へ: [スタックレイアウト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Stack-Layout)、前: [VMを使う理由](https://doc.guix.gnu.org/guile/latest/en/guile.html#Why-a-VM_003f)、上: [Guile 用仮想マシン](https://doc.guix.gnu.org/guile/latest/en/guile.html#A-Virtual-Machine-for-Guile) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.3.2 VM の概念 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#VM-Concepts-1)

Schemeプロシージャ内のバイトコードは、仮想マシン（VM）によって解釈されます。各スレッドは、それぞれ独自のVMインスタンスを持ちます。仮想マシンは、プロシージャ内の命令シーケンスを実行します。

各VM命令は、まずそれがどの操作であるかを示し、次にそのソースオペランドとデスティネーションオペランドをエンコードします。各プロシージャは、関数引数を含むいくつかのローカル変数を持つことを宣言します。これらのローカル変数は、プロシージャで使用可能なオペランドを構成し、インデックスによってアクセスされます。

プロシージャのローカル変数はスタックに格納されます。プロシージャを呼び出すと通常スタックが拡張され、プロシージャから戻るとスタックが縮小されます。スタックメモリは、それを所有する仮想マシン専用です。

仮想マシンは、スタックに加えて、他の仮想マシンを含むGuileの他の部分と共有されるグローバルメモリ（モジュール、グローバルバインディングなど）にもアクセスできます。

VMが持つレジスタは以下のとおりです。

* ip - 命令ポインタ
* sp - スタックポインタ
* fp - フレームポインタ

他のアーキテクチャでは、命令ポインタは「プログラムカウンタ」（pc）と呼ばれることもあります。これらのレジスタ群は仮想マシンではごく一般的なものであり、GuileのVMにおける正確な意味については次のセクションで説明します。

* * *

次へ: [変数と VM](https://doc.guix.gnu.org/guile/latest/en/guile.html#Variables-and-the-VM)、前: [VM の概念](https://doc.guix.gnu.org/guile/latest/en/guile.html#VM-Concepts)、上: [Guile 用仮想マシン](https://doc.guix.gnu.org/guile/latest/en/guile.html#A-Virtual-Machine-for-Guile) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.3.3 スタックレイアウト [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Stack-Layout-1)

Guileの仮想マシンのスタックはフレームで構成されています。各フレームはコンパイル済みのプロシージャの適用に対応し、引数、ローカル変数、および（フレームの終了後に実行すべき処理など）いくつかの管理情報を格納する領域を含んでいます。

コンパイラは計算のセマンティクスが維持される限り、どのような処理を行っても構いませんが、実際には関数を呼び出すたびに新しいフレームが作成されます。（もちろん、末尾呼び出しの場合は例外です。詳細は[末尾呼び出し](https://doc.guix.gnu.org/guile/latest/en/guile.html#Tail-Calls)を参照してください。）

上部スタックフレームの構造は以下のとおりです。

| ...前のフレームのローカル... |
+===============================+ <- fp + 3
| 動的リンク |
+------------------------------+
| 仮想返送先住所 (vRA) |
+------------------------------+
| 機械返送先住所 (mRA) |
+===============================+ <- fp
| ローカル 0 |
+------------------------------+
| ローカル1 |
+------------------------------+
| ... |
+------------------------------+
| ローカル N-1 |
\\------------------------------/ <- sp

上記の図では、スタックは下方向に成長します。関数呼び出しの開始時には、適用されるプロシージャがローカル変数0にあり、続いてローカル変数1からの引数が続きます。プロシージャは、渡された引数が互換性のあるものであることを確認した後、関数にローカルな変数を保持するために、フレーム内に追加の領域を割り当てます。

ローカル変数スロットの値が不要になった場合、Guile はそのスロットを自由に再利用できることに注意してください。これは、呼び出し先と引数に最初に使用されたスロットにも当てはまります。そのため、Guile のバックトレースでは、すべての引数が常に表示されるとは限りません。引数に対応するスロットが他の変数によって再利用されている可能性があるためです。

_仮想リターンアドレス_は、このプログラムが適用される前に有効だった`ip`です。このアクティベーションフレームから戻ると、この`ip`にジャンプします。同様に、_動的リンク_は、現在の`fp`に対する、このプログラムが適用される前に有効だった`fp`のオフセットです。

戻りアドレスには、仮想戻りアドレス（vRA）とマシン戻りアドレス（mRA）の2種類があります。vRAは常に存在し、バイトコードアドレスを示します。mRAは、マシンコードを含む関数（例えば、JITコンパイルされた関数）から呼び出しが行われた場合にのみ存在します。

非末尾アプリケーションに備えるため、GuileのVMは、適用する関数とその引数を適切なスタックスロットにシャッフルし、その下に3つの空きスロットを確保するコードを出力します。次に、呼び出しはこれらの空きスロットを初期化して、マシンリターンアドレス（またはNULL）、仮想リターンアドレス、および前のフレームポインタ（`fp`）へのオフセットを保持します。その後、呼び出される関数の`ip`を取得し、`fp`を新しい呼び出しフレームを指すように調整します。

このようにして、動的リンクは現在のフレームを前のフレームにリンクします。スタックトレースを計算するには、これらのフレームを順にたどる必要があります。

Guile の各スタック ローカル変数は、32 ビット アーキテクチャでも 64 ビット幅です。これにより、Guile はスタック ローカル変数の均一な扱いを維持しながら、64 ビット整数と浮動小数点数に対するアンボックス演算を可能にします。アンボックス演算の詳細については、[命令セット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instruction-Set) を参照してください。

実装の詳細として、動的リンクは絶対値ではなくオフセットとして格納します。これは、スタックが実行時に拡張されたり、部分継続呼び出し中に移動したりする可能性があるためです。絶対値として格納すると、フレームを走査してフレームポインタを再配置する必要が生じます。

* * *

次へ: [コンパイル済みプロシージャは VM プログラムです](https://doc.guix.gnu.org/guile/latest/en/guile.html#VM-Programs)、前: [スタック レイアウト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Stack-Layout)、上: [Guile 用仮想マシン](https://doc.guix.gnu.org/guile/latest/en/guile.html#A-Virtual-Machine-for-Guile) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.3.4 変数と VM [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Variables-and-the-VM-1)

以下のSchemeコードを例として考えてみましょう。

(define (foo a)
(lambda (b) (vector foo ab)))

ラムダ式の中では、`foo`はトップレベル変数、`a`は字句的にキャプチャされた変数、`b`はローカル変数です。

`a`と`b`を表す別の言い方としては、`a`はラムダ式の中で定義されていないため「自由」変数であり、`b`は「束縛」変数であると言うことができます。これらは、関数を記述するための数学的表記法であるラムダ計算で使用される用語です。ラムダ計算は、関数と変数について正確に推論できる言語であるため有用です。特にスコープ関係を記述するのに優れており、そのためここで言及しています。

Guileはすべての変数をスタック上に割り当てます。自由変数を持つ字句的に囲まれたプロシージャ（クロージャ）が作成されると、それらの変数を自由変数ベクトルにコピーします。その後、自由変数への参照は自由変数ベクトルを介してリダイレクトされます。

ただし、変数が `set!` される場合は、スタック割り当てではなくヒープ割り当てを行う必要があります。そうすることで、同じ変数をキャプチャする異なるクロージャが同じ値を参照できるようになります。また、これにより、継続は特定の時点での値ではなく、変数への参照をキャプチャできるようになります。これらの理由から、`set!` 変数は「ボックス」、つまり変数セルに割り当てられます。詳細については、[変数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Variables) を参照してください。`set!` 変数への参照は、ボックスを介して間接的に行われます。

したがって、直感に反するかもしれませんが、「よりハードウェアに近い」ように見える`set!`は、実際には余分なメモリ割り当てと間接参照を強制します。Guileのオプティマイザは、この割り当てを削除できる場合もありますが、常にできるとは限りません。

先の例に戻ると、`b`は決して変更されないため、スタック上に割り当てられる可能性があります。

`a`もスタック上に割り当てられる可能性があり、これも変更されることはありません。囲まれたラムダ式の中では、その値は自由変数ベクトルにコピーされ、そこから参照されます。

`foo` はトップレベル変数です。なぜなら、この例では `foo` は字句的に束縛されていないからです。

* * *

次へ: [オブジェクト ファイル形式](https://doc.guix.gnu.org/guile/latest/en/guile.html#Object-File-Format)、前: [変数と VM](https://doc.guix.gnu.org/guile/latest/en/guile.html#Variables-and-the-VM)、上: [Guile 用仮想マシン](https://doc.guix.gnu.org/guile/latest/en/guile.html#A-Virtual- Machine-for-Guile) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.3.5 コンパイル済みプロシージャは VM プログラムです [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compiled-Procedures-are-VM-Programs)

デフォルトでは、GuileのREPLで式を入力すると、まずバイトコードにコンパイルされます。次に、そのバイトコードが実行されて値が生成されます。式がプロシージャに評価される場合、この処理の結果はコンパイル済みのプロシージャになります。

コンパイル済みプロシージャは、バイトコードとキャプチャされたレキシカル変数への参照からなる複合オブジェクトです。さらに、プロシージャがコンパイルされると、行番号マッピングやドキュメント文字列などの関連メタデータがサイドテーブルに書き込まれます。これらの要素は、`(system vm program)` のアクセサを使用して個別に取得できます。完全な API リファレンスについては、[コンパイル済みプロシージャ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compiled-Procedures) を参照してください。

プロシージャは、コンパイル時に静的に割り当てられたデータを参照することができます。たとえば、一対の即時オブジェクト（[即時オブジェクト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Immediate-Objects)を参照）は、コンパイルされたバイトコードを含むメモリセグメントに直接割り当てられ、バイトコードから直接アクセスできます。

静的に割り当てられたデータのもう一つの用途は、バイトコードのキャッシュとして機能することです。トップレベル変数のルックアップはこの方法で処理されます。トップレベルバインディングが最初に参照されると、解決された変数がキャッシュに格納されます。その後、変数へのアクセスはすべてキャッシュセルを経由します。変数の値は将来変更される可能性がありますが、変数自体は変更されません。

これらの概念がどのように結びついているかを知るには、先に定義した`foo`関数を分解して、何が起こっているのかを見てみましょう。

scheme@(guile-user)> (define (foo a) (lambda (b) (vector foo ab)))
scheme@(guile-user)> ,x foo
#xf1da30 の #<procedure foo (a)> の逆アセンブル:

0 (インストゥルメントエントリ 164) (不明なファイル):5:0
2 (assert-nargs-ee/locals 2 1) ;; 3 スロット (1 引数)
3 (allocate-words/immediate 2 3) (不明なファイル):5:16
4 (load-u64 0 0 65605)
7 (単語セット!/即時 2 0 0)
8 (load-label 0 7) ;; #xf1da6c の匿名プロシージャ
10 (単語セット!/即時 2 1 0)
11 (scm-set!/immediate 2 2 1)
12 (リセットフレーム 1) ;; 1 スロット
13（割り込み処理）
14（戻り値）

----------------------------------------
#xf1da6c における匿名プロシージャの逆アセンブル:

0 (インストゥルメントエントリ 183) (不明なファイル):5:16
2 (assert-nargs-ee/locals 2 3) ;; 5 スロット (1 引数)
3 (static-ref 2 152) ;; #<変数 112e530 の値: #<プロシージャ foo (a)>>
5 (immediate-tag=? 2 7 0) ;; ヒープオブジェクト?
7 (je 19) ;; -> L2
8 (static-ref 2 119) ;; #<directory (guile-user) ca9750>
10 (static-ref 1 127) ;; foo
12 (call-scm<-scm-scm 2 2 1 40)
14 (immediate-tag=? 2 7 0) ;; ヒープオブジェクト?
16 (jne 8) ;; -> L1
17 (scm-ref/immediate 0 2 1)
18 (immediate-tag=? 0 4095 2308) ;; 未定義?
20 (je 4) ;; -> L1
21 (static-set! 2 134) ;; #<変数 112e530 の値: #<プロシージャ foo (a)>>
23 (j 3) ;; -> L2
L1:
24 (throw/value 1 151) ;; #(unbound-variable #f "Unbound variable: ~S")
L2:
26 (scm-ref/immediate 2 2 1)
27 (allocate-words/immediate 1 4) (不明なファイル):5:28
28 (load-u64 0 0 781)
31 (単語セット!/即時 1 0 0)
32 (scm-set!/immediate 1 1 2)
33 (scm-ref/immediate 4 4 2)
34 (scm-set!/immediate 1 2 4)
35 (scm-set!/immediate 1 3 3)
36 (ムーブ 4 1)
37 (リセットフレーム 1) ;; 1 スロット
38（割り込み処理）
39（戻り値）

まず注目すべき点は、バイトコードがかなり低レベルであるということです。Schemeからバイトコードにコンパイルされるプログラムでは、より基本的な演算で表現されます。そのため、予想以上に多くの命令が含まれる場合があります。

最初の命令群は、外側の `foo` プロシージャです。それに続いて、内部のクロージャのコードが記述されています。コードは一見すると難解に見えるかもしれませんが、練習を重ねればすぐに理解できるようになります。実際、バイトコードを読み解く能力は、Guile プログラムの低レベルなパフォーマンスを理解する上で重要なステップです。

`foo` 関数は前奏から始まります。`instrument-entry` バイトコードは、関数に関連付けられたカウンタをインクリメントします。カウンタが一定のしきい値に達すると、Guile は `foo` のマシンコード（JIT コンパイル）を出力します。マシンコードの出力は比較的低コストですが、時間がかかるため、すべての関数に対して実行するのは望ましくありません。関数ごとのカウンタとグローバルなしきい値を使用することで、Guile は「ホット」な関数のみを JIT コンパイルすることに時間を費やすことができます。

プレリュードの次の部分は引数チェック命令で、呼び出し時に引数が 1 つだけであったこと (呼び出し先の関数自体が 2 つになるため) をチェックし、さらに 1 つのローカル変数のためにスタック領域を確保します。

次に、`ip` 3 から 11 まで、3 ワードのオブジェクトを割り当てることで新しいクロージャを割り当てます。最初のワードを初期化して型タグを格納し、2 番目のワードをコード ポインタに設定し、最後に `ip` 11 で、ローカル値 1 (`a` 引数) を 3 番目のワード (最初の自由変数) に格納します。

`foo` は戻る前に「フレームをリセット」してローカル変数 (戻り値) を 1 つだけ保持し、保留中の割り込み ([非同期割り込み](https://doc.guix.gnu.org/guile/latest/en/guile.html#Asyncs) を参照) を実行してから戻ります。

Guileの仮想マシンにおけるローカル変数は通常、スタックポインタを基準としてアドレス指定されるため、`sp[n]`へのアクセスは非常に効率的です。しかし、関数実行中に`sp`が変化する可能性があり、また入力引数は`sp`ではなく`fp`を基準とするため、逆アセンブリコードが読みにくくなる場合があります。

`sp` 相対参照に対応する `fp` 相対スロットを知るには、逆アセンブリを上にスキャンして「n slots」という注釈を探します。この例では 3 となっており、フレームに 3 つのスロット分のスペースがあることを示しています。したがって、0 インデックスの `sp` 相対スロット 2 は、呼び出し中のクロージャの値が最初に保持されていた `fp` 相対スロット 0 に対応します。これは、Guile が結果を計算するためにクロージャの値を必要としないため、スロット 0 は再利用可能であり、この場合は新しいクロージャを作成する結果に使用されます。

クロージャとは、データを含むコードのことです。ご覧のとおり、クロージャを作成するには、オブジェクトを作成し（`ip` 3）、コードポインタをそのオブジェクトに配置し（`ip` 8と10）、クロージャの自由変数を配置します（`ip` 11）。

2 番目のスタンザでは、クロージャのコードを逆アセンブルします。前奏の後、`ip` 5 から 24 までのすべてのコードは、トップレベル変数 `foo` をスロット 1 にロードすることに関連しています。このルックアップは 1 回のみ実行され、キャッシュに関連付けられています。最初の実行後、キャッシュ内の値はバインドされた変数になり、コードは `ip` 7 から 26 にジャンプします。最初の実行では、Guile は関数に関連付けられたモジュールを取得し、実行時ルーチンを呼び出して変数をルックアップし、キャッシュを初期化する前に変数がバインドされていることを確認します。いずれにしても、`ip` 26 は変数をローカル 2 に逆参照します。

以下に、ベクター戻り値の割り当てと初期化を示します。`Ip` 27 で割り当てを行い、続く 2 つの命令でオブジェクトの最初のワードの型と長さのタグを初期化します。`Ip` 32 でオブジェクトのワード 1 (最初のベクター スロット) を`foo` の値に設定します。`ip` 33 で `a` のクロージャ変数を取得し、`ip` 34 でそれを 2 番目のベクター スロットに格納します。最後に、`ip` 35 でローカル変数 `b` を 3 番目のベクター スロットに格納します。その後、戻りシーケンスが続きます。

* * *

次へ: [命令セット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instruction-Set)、前: [コンパイル済みプロシージャはVMプログラムです](https://doc.guix.gnu.org/guile/latest/en/guile.html#VM-Programs)、上: [Guile用仮想マシン](https://doc.guix.gnu.org/guile/latest/en/guile.html#A-Virtual-Machine-for-Guile) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.3.6 オブジェクトファイル形式 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Object-File-Format-1)

ファイルをディスクにコンパイルするには、コンパイルされたコードをディスクに書き込み、後でGuileにロードするためのフォーマットが必要です。優れたオブジェクトファイルフォーマットには、次のような特徴があります。

何よりもまず、コンパイル済みファイルの読み込みは非常に安価であるべきです。
* ファイル内で定数を静的に割り当てることが可能であるべきです。例えば、ソースコード内のバイトベクトルリテラルをオブジェクトファイルに直接出力することができます。
コンパイルされたファイルは、異なるプロセス間でのコードとデータの共有を最大限に可能にするものであるべきです。
コンパイルされたファイルには、行番号などのデバッグ情報を含める必要がありますが、その情報はコード自体とは分離されている必要があります。容量が限られている場合は、デバッグ情報を削除できるようにする必要があります。

これらの特性はSchemeに特有のものではありません。実際、CやC++といった主流言語は、過去に何度もこの問題を解決してきました。Guileは、GNUやその他のUnix系システムのオブジェクトファイル形式であるELFをオブジェクトファイル形式として採用することで、これらの言語の成果を基盤としています。GuileはすべてのプラットフォームでELFを使用していますが、プラットフォームのELFサポートは利用していません。Guileは独自のリンカとローダを実装しています。ELFを使用する利点は、コードを共有することではなく、アイデアを共有することです。ELFは、単に優れた設計のオブジェクトファイル形式なのです。

ELFファイルには、その内容を記述する2つのメタテーブルがあります。1つ目のメタテーブルはローダー用で、_プログラムテーブル_、または_セグメントテーブル_と呼ばれることもあります。プログラムテーブルはファイルを大きなチャンクに分割し、ローダーはこれらのチャンクをそれぞれ異なる方法で処理します。これらの_セグメント_間の違いは、主にアクセス権限です。

通常、ELFファイルのすべてのセグメントは読み取り専用としてマークされますが、変更可能な静的データ、またはロード時に初期化が必要な静的データを表す部分は例外です。ELFファイルのロードは、読み取り専用権限でメモリにmmapし、セグメントテーブルを使用してファイルの小さなサブ領域を書き込み可能としてマークするだけで済みます。この書き込み可能なセクションは、通常、ガベージコレクタのルートセットにも追加されます。

1 つの ELF セグメントは「動的」としてマークされており、これはローダーにとって重要なデータが含まれていることを意味します。Guile はこのセグメントを使用して、このファイルに対応する Guile バージョンを記録します。また、動的セグメントには、必要なリンク時初期化を実行するために実行される初期化サンクのアドレスを指すエントリがあります。（これは、通常の ELF 共有オブジェクトの動的再配置に似ていますが、ローダーが再配置のテーブルを解釈するのではなく、再配置をプロシージャとしてコンパイルする点が異なります。）最後に、動的セグメントはオブジェクト ファイルの「エントリ サンク」の場所を示します。このサンクは、`load-thunk-from-memory` または `load-thunk-from-file` の呼び出し元に返されます。呼び出されると、コンパイルされた式の「本体」が実行されます。

ELFファイル内のもう一つのメタテーブルは、セクションテーブルです。プログラムテーブルはローダーのためにELFファイルを大きなチャンクに分割しますが、セクションテーブルはデバッガーなどのイントロスペクティブツールで使用するための小さなセクションを指定します。1つのセグメント（プログラムテーブルのエントリ）には通常、複数のセクションが含まれます。また、どのセグメントにも属さないセクションが存在する場合があります。

Guileの`.go`ファイルに含まれる典型的なセクションは以下のとおりです。

`.rtl-text`

バイトコード。

`.data`

初期化が必要なデータ、または実行時に変更される可能性のあるデータ。

`.rodata`

実行時の初期化を必要としない静的に割り当てられたデータであり、そのためプロセス間で共有することができる。

`.dynamic`

上記で説明した動的セクション。

`.symtab`

`.strtab`

`.rtl-text`内のアドレスをプロシージャ名にマッピングするテーブル。`.strtab`は`.symtab`で使用されます。

`.guile.procprops`

`.guile.arities`

`.guile.arities.strtab`

`.guile.docstrs`

`.guile.docstrs.strtab`

手続きのプロパティ、引数の数、およびドキュメント文字列をまとめたサイドテーブル。

`.guile.docstrs.strtab`

プログラムテキスト内の各リターンポイントに対応する生存スロットのセットと、それらのスロットがポインタであるか否かを記述したフレームマップのサイドテーブル。ガベージコレクタによって使用されます。

`.debug_info`

`.debug_abbrev`

`.debug_str`

`.debug_loc`

`.debug_line`

デバッグ情報はDWARF形式で出力されます。詳細については、DWARF仕様書を参照してください。

`.shstrtab`

セクション名文字列テーブル。

詳細については、[elf(5) のマニュアルページ](http://linux.die.net/man/5/elf) を参照してください。DWARF デバッグ形式の詳細については、[DWARF 仕様](http://dwarfstd.org/) を参照してください。あるいは、冒険好きな探検家なら、コンパイル済みの `.go` ファイルに対して `readelf` または `objdump` を実行してみてください。きっと楽しいですよ！

* * *

次へ: [Just-In-Time Native Code](https://doc.guix.gnu.org/guile/latest/en/guile.html#Just_002dIn_002dTime-Native-Code)、前: [Object File Format](https://doc.guix.gnu.org/guile/latest/en/guile.html#Object-File-Format)、上: [Guile 用仮想マシン](https://doc.guix.gnu.org/guile/latest/en/guile.html#A-Virtual-Machine-for-Guile) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.3.7 命令セット [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instruction-Set-1)

現在、Guileの仮想マシンには約150個の命令が存在します。これらの命令は、プログラム実行における最小単位を表します。理想的には、条件分岐なしで1つのタスクを実行し、その後、ストリーム内の次の命令へと処理を移します。

命令自体は1つ以上の32ビット単位から構成されます。最初のワードの下位8ビットはオペコードを示し、残りの命令はオペランドを表します。オペランドのエンコード方法にはいくつかの種類があります。

`sn`

ローカル変数の `sp` 相対インデックスを示す、符号なし n ビット整数。

`fn`

ローカル変数の `fp` 相対インデックスを示す、符号なし n ビット整数。継続処理が可変個の値を受け入れる場合に、受信した値をフレーム内の既知の位置にシャッフルするために使用されます。

`cn`

符号なしnビット整数。定数値を表します。

`l24`

現在の `ip` からのオフセットを、32 ビット単位で符号付き 24 ビット値として表します。相対ジャンプのためのバイトコード アドレスを示します。

`zi16`

`i16`

`i32`

、即値Scheme値（[Immediate Objects](https://doc.guix.gnu.org/guile/latest/en/guile.html#Immediate-Objects)を参照）。`zi16`は符号拡張、その他はゼロ拡張です。

`a32`

`b32`

Schemeの即値で、32ビットワードのペアとしてエンコードされます。`a32`と`b32`の値は常に同じオペコードにセットされ、それぞれ上位ビットと下位ビットを示します。通常は64ビットシステムでのみ使用されます。

`n32`

静的に割り当てられた非即値。非即値のアドレスは符号付き32ビット整数としてエンコードされ、32ビット単位での相対オフセットを示します。`SCM x = ip + offset` と考えてください。

`r32`

間接的なスキーム値。`n32`と同様だが、間接的に指定する。`SCM *x = ip + offset`と考えてください。

`l32`

`lo32`

符号付き32ビット整数として表されるIP相対アドレス。`make-closure`のようにバイトコードアドレスを示す場合もあれば、`static-patch!`のように非即値アドレスを示す場合もある。

仮想マシンの観点から見ると、`l32` と `lo32` は同じものです。違いは、アセンブラによっては、静的に割り当てられたオブジェクトのフィールドをパッチする場合などに、ラベルとして `lo32` アドレスを指定し、そのラベルから一定数のワードをオフセットして指定できるようにする場合がある点です。

`v32:x8-l24`

ほぼすべてのVM命令は固定サイズです。最適化された`case`分岐を実行するために使用される`jtable`命令は例外で、命令内の追加ワード数を示すために末尾に`v32`ワードを使用し、そのワード自体は`x8-l24`値としてエンコードされます。

`b1`

ブール値：真の場合は1、それ以外の場合は0。

`xn`

無視されるnビットのシーケンス。

命令は、まずその名前を記述し、次にオペランドを記述することで指定されます。オペランドは32ビットのワードにパックされ、先に指定されたオペランドは下位ビットに配置されます。

例えば、以下の命令仕様を考えてみましょう。

指示: **call** `f24:proc x8:_ c24:nlocals` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-call)

命令の最初のワードは、下位ビットに呼び出しオペコードに対応する8ビット値が格納され、その後に24ビット値としてprocが続きます。2番目のワードは、8ビットのデッドビットで始まり、その後に24ビットの即値としてインデックスが格納されます。

スタックへの参照をエンコードするオペランドを持つ命令の場合、それらのスタック値の解釈は命令自体に委ねられます。ほとんどの命令は、オペランドがタグ付きSCM値（`scm`表現）であることを想定していますが、一部の命令は、ボックス化されていない整数（`u64`および`s64`表現）または浮動小数点数（`f64`表現）を想定しています。`u64`値のビットは`s64`値のビットと同じであり、`s64`値は2の補数で格納されることが前提となっています。

命令には静的型があります。つまり、命令は想定した形式でオペランドを受け取る必要があります。コンパイラは、このことを保証しなければなりません。

特に明記されていない限り、すべてのオペランドと結果は`scm`表現で表されます。

* [呼び出しと戻りの手順](https://doc.guix.gnu.org/guile/latest/en/guile.html#Call-and-Return-Instructions)
* [関数プロローグの説明](https://doc.guix.gnu.org/guile/latest/en/guile.html#Function-Prologue-Instructions)
* [シャッフル手順](https://doc.guix.gnu.org/guile/latest/en/guile.html#Shuffling-Instructions)
* [トランポリンの使い方](https://doc.guix.gnu.org/guile/latest/en/guile.html#Trampoline-Instructions)
* [非ローカル制御フロー命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Non_002dLocal-Control-Flow-Instructions)
* [計測手順](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instrumentation-Instructions)
* [組み込み関数呼び出し手順](https://doc.guix.gnu.org/guile/latest/en/guile.html#Intrinsic-Call-Instructions)
* [定数命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Constant-Instructions)
* [メモリアクセス命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Memory-Access-Instructions)
* [アトミックメモリアクセス命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Atomic-Memory-Access-Instructions)
* [タグ付けとタグ解除の手順](https://doc.guix.gnu.org/guile/latest/en/guile.html#Tagging-and-Untagging-Instructions)
* [整数演算命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Integer-Arithmetic-Instructions)
* [浮動小数点演算命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Floating_002dPoint-Arithmetic-Instructions)
* [比較手順](https://doc.guix.gnu.org/guile/latest/en/guile.html#Comparison-命令)
* [ブランチ手順](https://doc.guix.gnu.org/guile/latest/en/guile.html#Branch-Instructions)
* [生メモリへのアクセス手順](https://doc.guix.gnu.org/guile/latest/en/guile.html#Raw-Memory-Access-Instructions)

* * *

次へ: [関数プロローグ命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Function-Prologue-Instructions)、上: [命令セット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instruction-Set) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.3.7.1 呼び出しと戻りの手順 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Call-and-Return-Instructions-1)

前述のとおり（[スタックレイアウト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Stack-Layout)を参照）、Guile の呼び出し規約では、引数はスタックに渡され、値はスタックに返されます。

末尾位置と非末尾位置の両方の呼び出しにおいて、プロシージャと引数は呼び出し命令の前に所定の位置に配置されている必要があります。末尾呼び出しの場合、「所定の位置」とは、プロシージャが `fp` を基準としたスロット 0 にあり、引数がそれに続くことを意味します。非末尾呼び出しの場合、プロシージャが `fp` を基準としたスロット n にある場合、引数はスロット n+1 から続く必要があり、n-1 から n-3 の間に mRA、vRA、および `fp` を保存するための空きスロットが 3 つ必要です。

戻り値も同様です。複数の値を返す場合、戻り値を出力する前に、値は`fp`相対スロット0から始まるようにシャッフルされている必要があります。

呼び出しと戻りの両方において、`sp`は呼び出し先または呼び出し元に対して、それぞれ引数の数または戻り値の数を示すために使用されます。戻り値を受け取った後、呼び出し元は`sp`を元の値にリセットすることでフレームを復元する責任があります。

指示: **call** `f24:proc x8:_ c24:nlocals` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-call-1)

プロシージャを呼び出します。proc はプロシージャに対応するローカル変数です。proc の下にある 3 つの値は、保存された呼び出しフレームデータによって上書きされます。新しいフレームには nlocals 個のローカル変数を格納するスペースがあります。1 つはプロシージャ用、残りは既にプッシュされているはずの引数用です。

呼び出しが戻ると、次の命令の実行が開始されます。戻りスタックには任意の数の値が存在する可能性があります。正確な数は、呼び出し後の `sp` から proc\-1 のアドレスを減算することで得られます。

指示: **call-label** `f24:proc x8:_ c24:nlocals l32:label` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-call_002dlabel)

同じコンパイル単位内のプロシージャを呼び出す。

この命令は `call` とほぼ同じですが、呼び出し先を見つけるために proc を逆参照する代わりに、呼び出し先は label にあることがわかっています。label は、現在の `ip` から 32 ビット単位で符号付き 32 ビットオフセットされた値です。proc は逆参照されないため、クロージャの別の表現である可能性があります。

指示: **tail-call** `x24:_` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-tail_002dcall)

プロシージャを末尾呼び出しします。プロシージャとすべての引数が既に適切な位置にシャッフルされていること、およびフレームが呼び出しの引数の数にリセットされている必要があります。

指示: **tail-call-label** `x24:_ l32:label` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-tail_002dcall_002dlabel)

既知のプロシージャを末尾呼び出しします。`call` が `call-label` に対応するように、`tail-call` は `tail-call-label` に対応します。

指示: **return-values** `x24:_` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-return_002dvalues)

呼び出しフレームから複数の値を返します。返される値は、既にスロット0から始まる連続した配列にシャッフルされており、フレームは既にリセットされている必要があります。

指示: **receive** `f12:dst f12:proc x8:_ c24:nlocals` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-receive-2)

プロシージャが proc 内にある呼び出しから単一の戻り値を受け取り、呼び出しが実際に少なくとも 1 つの値を返したことを確認します。その後、フレームを nlocals ローカルにリセットします。

指示: **receive-values** `f24:proc b1:allow-extra? x7:_ c24:nvalues` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-receive_002dvalues)

プロシージャが proc 内にある呼び出しから、複数の値を受け取ります。返された値が nvalues より少ない場合は、エラーを通知します。allow-extra? が true でない限り、返される値の数は nvalues と正確に一致する必要があります。`receive-values` の実行後、値は `mov` でコピーするか、そのまま使用できます。

* * *

次へ: [シャッフル命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Shuffling-Instructions)、前: [呼び出しと戻り命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Call-and-Return-Instructions)、上: [命令セット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instruction-Set) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.3.7.2 関数プロローグの説明 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Function-Prologue-Instructions-1)

Guileにおける関数呼び出しは非常に低コストです。VMは単にプロシージャに制御を渡すだけです。プロシージャ自体は、適切な数の引数が渡されたことをアサートする責任を負います。この戦略により、一般的なケースを損なうことなく、任意の複雑な引数解析イディオムを開発することが可能になります。

例えば、キーワード引数プロシージャの呼び出しのみが、キーワード引数の解析コストを「負担」する。（本稿執筆時点では、キーワード引数を持つプロシージャの呼び出しは、固定引数を持つプロシージャの呼び出しに比べて、通常2～4倍のコストがかかる。）

指示: **assert-nargs-ee** `c24:expected` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-assert_002dnargs_002dee)

指示: **assert-nargs-ge** `c24:expected` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-assert_002dnargs_002dge)

指示: **assert-nargs-le** `c24:expected` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-assert_002dnargs_002dle)

実際の引数の数がそれぞれ `==`、`>=`、または `<=` で想定される数でない場合は、エラーを通知します。

引数の数は、フレームポインタからスタックポインタを減算することによって決定されます (`fp - sp`)。スタックフレームの詳細については、[スタックレイアウト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Stack-Layout)を参照してください。なお、期待される引数の数には、プロシージャ自体が含まれます。

指示: **arguments<=?** `c24:expected` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-arguments_003c_003d_003f)

引数の数が想定よりも少ない場合、等しい場合、多い場合、それぞれ比較結果の値として`LESS_THAN`、`EQUAL`、または`NONE`を設定します。

指示: **positional-arguments<=?** `c24:nreq x8:_ c24:expected` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-positional_002darguments_003c_003d_003f)

位置引数の数がそれぞれ想定よりも少ない場合、等しい場合、多い場合、比較結果の値として `LESS_THAN`、`EQUAL`、または `NONE` を設定します。最初の nreq 引数は位置引数であり、キーワードではない後続の引数も位置引数です。

`arguments<=?` および `positional-arguments<=?` 命令は、`case-lambda` のように複数の引数を実装するために使用されます。詳細については、[Case-lambda](https://doc.guix.gnu.org/guile/latest/en/guile.html#Case_002dlambda) を参照してください。比較結果の詳細については、[Branch Instructions](https://doc.guix.gnu.org/guile/latest/en/guile.html#Branch-Instructions) を参照してください。

指示: **bind-kwargs** `c24:nreq c8:flags c24:nreq-and-opt x8:_ c24:ntotal n32:kw-offset` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bind_002dkwargs)

flagsはビットフィールドであり、最下位ビットはallow-other-keys、2番目のビットはhas-rest、そして続く6ビットは未使用です。

最後の位置引数を見つけ、残りの引数をすべて ntotal より上にシャッフルします。途中のローカル変数を `SCM_UNDEFINED` に初期化します。次に、現在の ip から kw-offset ワードの定数をロードし、それと allow-other-keys フラグを使用してキーワード引数をバインドします。has-rest の場合、シャッフルされた引数をすべてリストに収集し、nreq-and-opt に格納します。最後に、シャッフルした引数をクリアします。

構文解析は、kw-offset を使用して検索されるキーワード引数の関連付けリストによって行われます。alist は、キーワード引数をローカルスロットのインデックスにマッピングする `(kw . index)` 形式のペアのリストです。`allow-other-keys` が設定されていない限り、未知のキーが見つかった場合、パーサーはエラーを通知します。

マクロメガ命令。

指示: **bind-optionals** `f24:nlocals` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bind_002doptionals)

現在のフレームを拡張して、少なくとも nlocals 個のローカル変数を持つようにします。新しい値はすべて `SCM_UNDEFINED` で埋めます。フレームに nlocals 個を超えるローカル変数がある場合は、そのままにしておきます。

指示: **bind-rest** `f24:dst` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bind_002drest)

dst または dst より上の引数をすべてリストに収集し、そのリストを dst に格納します。

指示: **alloc-frame** `c24:nlocals` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-alloc_002dframe)

スタック上にnlocals個のローカル変数を格納するための十分なスペースがあることを確認してください。新しいローカル変数の値は未定義です。

指示: **reset-frame** `c24:nlocals` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-reset_002dframe)

`alloc-frame` と同様ですが、スタックが十分な大きさであるかどうかのチェックは行わず、値を `SCM_UNDEFINED` に初期化しません。フレームサイズを、以前 alloc-frame で設定したサイズよりも小さい値にリセットするために使用されます。

指示: **assert-nargs-ee/locals** `c12:expected c12:nlocals` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-assert_002dnargs_002dee_002flocals)

`assert-nargs-ee`と`alloc-frame`のシーケンスに相当します。予約されるローカル変数の数は、nlocals + nlocals となります。

* * *

次へ: [トランポリン命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Trampoline-Instructions)、前: [関数プロローグ命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Function-Prologue-Instructions)、上: [命令セット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instruction-Set) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.3.7.3 シャッフル手順 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Shuffling-Instructions-1)

これらの命令は、スタック上の値を移動するために使用されます。

命令: **mov** `s12:dst s12:src` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-mov)

命令: **long-mov** `s24:dst x8:_ s24:src` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-long_002dmov)

あるローカルスロットから別のローカルスロットへ値をコピーします。

前述のとおり、手続き引数とローカル変数はローカルスロットに割り当てられます。Guileのコンパイラは、変数を異なるスロットに移動させることを避けるように設計されているため、多くの場合`mov`命令は不要になります。しかし、変数の移動が必要なケースもあり、そのような場合は`mov`命令を使用するのが適切です。

指示: **long-fmov** `f24:dst x8:_ f24:src` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-long_002dfmov)

あるローカルスロットから別のローカルスロットに値をコピーしますが、スロットのアドレス指定は`sp`ではなく`fp`を基準とします。これは、複数の値が返された後に値をシャッフルして適切な位置に配置する場合に使用されます。

手順: **push** `s24:src` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-push)

スタックポインタを1ワード増やし、スロットsrcの値で埋めます。スタックポインタを調整する前に、srcへのオフセットを計算します。

`push`命令は、オペランドが24ビット未満でエンコードされているため、他の命令がオペランドにアドレス指定できない場合に使用されます。この場合、Guileのアセンブラは、必要なオペランドを一時的にスタックにプッシュし、それらの近くに配置した変数にアドレス指定するための元の命令を出力し、結果（存在する場合）を元の場所に戻すコードを透過的に出力します。

命令: **pop** `s24:dst` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-pop)

スタックポインタをポップし、スロットdstに格納されていた値を格納します。dstへのオフセットは、スタックポインタの調整後に計算されます。

手順: **drop** `c24:count` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-drop-1)

スタックポインタをカウントワード分だけポップし、そこに格納されていた値はすべて破棄します。

指示: **shuffle-down** `f12:from f12:to` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-shuffle_002ddown)

から までの値を入れ替えて、フレームサイズを FROM\-TO スロット分だけ縮小します。これは、`call-with-values`、`values`、および `apply` の内部実装の一部です。

指示: **expand-apply-argument** `x24:_` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-expand_002dapply_002dargument)

フレーム内の最後のローカル変数を取得し、それをスタック上に展開します。これは、`apply` の最後の引数と同様です。

* * *

次へ: [非ローカル制御フロー命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Non_002dLocal- Control-Flow-Instructions)、前: [シャッフリング命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Shuffling-Instructions)、上: [命令セット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instruction-Set) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.3.7.4 トランポリンの説明 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Trampoline-Instructions-1)

Guileで使用可能なオブジェクトのほとんどはバイトコードで実装されたプロシージャですが、すべてがそうではありません。プリミティブ、継続、その他のプロシージャに似たオブジェクトには、独自の呼び出し規約があります。Guileは、`call`命令に特別なケースを追加する代わりに、これらの使用可能なオブジェクトをVMトランポリンプロシージャでラップし、バイトコードでこれらのオブジェクトに対する特別なサポートを提供します。

トランポリン手順は通常、Guileによって実行時に生成されます。例えば、`scm_c_make_gsubr`の呼び出しに応じて生成されます。そのため、コンパイラはこれらの命令を含むコードを出力すべきではありません。しかし、これらの仕組みを知ることは興味深いので、ここではトランポリン命令について解説します。

指示: **subr-call** `c24:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-subr_002dcall)

サブルーチンを呼び出し、このフレーム内のすべてのローカル変数を引数として渡し、結果をスタックに格納して、返却準備を整えます。

指示: **foreign-call** `c12:cif-idx c12:ptr-idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-foreign_002dcall)

外部関数を呼び出します。呼び出し先の関数の cif-idx および ptr-idx クロージャスロットから cif と外部ポインタを取得します。引数はスタックから取得され、結果はスタックに配置され、返される準備が整います。

指示: **builtin-ref** `s12:dst c12:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-builtin_002dref)

インデックスを指定して組み込みスタブをdstにロードします。

* * *

次へ: [計測命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instrumentation-Instructions)、前: [トランポリン命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Trampoline-Instructions)、上: [命令セット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instruction-Set) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.3.7.5 非ローカル制御フロー命令 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Non_002dLocal-Control-Flow-Instructions-1)

指示: **capture-continuation** `s24:dst` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-capture_002dcontinuation)

現在の継続をキャプチャし、dstに書き込みます。`call/cc`の実装の一部です。

命令: **continuation-call** `c24:contregs` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-continuation_002dcall)

非局所的に継続に戻ります。継続への引数はスタックから取得されます。contregs は、具体化された継続を含む自由変数です。

指示: **abort** `x24:_` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-abort-1)

プロンプトハンドラに処理を中断します。タグはスロット1に配置され、フレーム内の残りの値はプロンプトハンドラに返されます。これは、`abort-to-prompt`の末尾適用に対応します。

指定されたタグを持つプロンプトが動的環境内に見つからない場合は、エラーが通知されます。それ以外の場合は、すべての引数がプロンプトのハンドラに渡され、必要に応じてキャプチャされた継続も渡されます。

プロンプトのハンドラがキャプチャされた継続を参照していないことが証明できれば、継続は割り当てられません。この決定は実行時に動的に行われます。一般的には、継続がキャプチャされ、再開される可能性があります。再開された継続は、複数値の戻り値の場合と同様に、スロット0からスタックに引数がプッシュされ、呼び出し元で制御が再開されます。したがって、呼び出し元の関数にとって、`abort-to-prompt`への呼び出しは、他の関数呼び出しと何ら変わりません。

指示: **compose-continuation** `c24:cont` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-compose_002dcontinuation)

現在の継続と部分継続を合成します。継続への引数はスタックから取得されます。cont は、具体化された継続を含む自由変数です。

指示: **prompt** `s24:tag b1:escape-only? x7:_ f24:proc-slot x8:_ l24:handler-offset` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-prompt)

動的スタックに新しいプロンプトをプッシュします。タグは tag から、ハンドラーは現在の ip から handler-offset ワードの位置にあります。

このプロンプトに対して中止命令が出された場合、制御はハンドラにジャンプします。ハンドラは、proc-slot にあるプロシージャ呼び出しから返されるかのように、複数の値を返すことを期待します。最初の引数は具象化された部分継続であり、その後にハンドラに返される値が続きます。制御がハンドラに戻った場合、プロンプトは既に中止メカニズムによってポップされています。（Guile の `prompt` は Felleisen の _–F–_ 演算子を実装しています。）

escape-only? がゼロ以外の場合、プロンプトは escape-only としてマークされ、継続を具体化しないようにこのプロンプトを中止することができます。

プロンプトの詳細については、[プロンプト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Prompts)を参照してください。

指示: **throw** `s12:key s12:args` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-throw-1)

keyとargsに対して例外をスローしてエラーを発生させます。argsはリストである必要があります。

指示: **throw/value** `s24:value n32:key-subr-and-message` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-throw_002fvalue)

指示: **throw/value+data** `s24:value n32:key-subr-and-message` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-throw_002fvalue_002bdata)

エラーを発生させ、val が不正な値であることを示します。key-subr-and-message はベクトルで、最初の要素はスローするシンボル、2 番目の要素はエラーを通知する手順 (文字列) または `#f`、3 番目の要素はメッセージのフォーマット文字列で、テンプレートが 1 つあります。これらの命令はフォールスルーしません。

これらの命令はどちらも、4 つの引数を持つキーに例外をスローします。引数は、エラーを示すプロシージャ (または `#f`)、フォーマット文字列、値のリスト、そして最後の引数として `#f` または値のリストのいずれかです。

指示: **到達不能** `x24:_` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-unreachable)

処理を中止します。この命令は決して実行されるべきではなく、継続してはなりません。これは無意味に思えるかもしれませんが、そうではありません。これは`raise-exception`へのプライマリ呼び出しの後に挿入され、この制御フローの分岐がグラフに再接続されないことをコンパイラに知らせます。

* * *

次へ: [組み込み呼び出し命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Intrinsic-Call-Instructions)、前: [非ローカル制御フロー命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Non_002dLocal-Control-Flow-Instructions)、上: [命令セット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instruction-Set) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.3.7.6 計測手順 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instrumentation-Instructions-1)

指示: **instrument-entry** `x24:_ n32:data` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-instrument_002dentry)

命令: **instrument-loop** `x24:_ n32:data` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-instrument_002dloop)

この関数の実行カウンタを増やし、場合によっては次の JIT レベルに階層化します。データは、実行回数とこの関数に対応する次のレベルの JIT コードを記録する構造体へのオフセットです。現在の増分値は、`instrument-entry` で 30、`instrument-loop` で 2 です。

VMフックが有効になっている場合、`instrument-entry`は適用フックも実行します。

指示: **handle-interrupts** `x24:_` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-handle_002dinterrupts)

非同期割り込み (asyncs) を処理します。[非同期割り込み](https://doc.guix.gnu.org/guile/latest/en/guile.html#Asyncs) を参照してください。コンパイラは、呼び出し、戻り、またはループの終了前に `handle-interrupts` 命令を挿入します。

命令: **return-from-interrupt** `x24:_` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- return_002dfrom_002dinterrupt)

呼び出しから復帰し、同時にスタックフレームを呼び出し元からポップするための特別な命令。非同期割り込みから復帰する際に使用されます。

* * *

次へ: [定数命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Constant-Instructions)、前: [計測命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instrumentation-Instructions)、上: [命令セット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instruction-Set) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.3.7.7 組み込み関数呼び出し命令 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Intrinsic-Call-Instructions-1)

Guileの命令セットは低レベルです。これは、例えば`vector-ref`操作の個々の構成要素を最適化して削除できるため、実行時に実行する必要のある操作だけを残すことができるという利点があります。

しかし、マクロ操作の中には、あらゆる例外ケースに対応するために実行時に大量の計算が必要となるものがあり、そのマイクロ操作コンポーネントは最適化に適さない場合があります。マクロ操作全体のコードを残余化すると、メリットもなくコードが肥大化するだけです。

このような場合、GuileのVMは_intrinsics_（ホスト言語で記述されたランタイムルーチン。現在はC言語のみだが、GuileがWebAssemblyなどのランタイムターゲットに対応すれば将来的にはC言語も含まれる可能性がある）を呼び出します。各intrinsicsプロトタイプには1つの命令があり、命令内のインデックスによってintrinsicsが指定されます。

指示: **call-thread** `x24:_ c32:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-call_002dthread)

インデックス idx を指定して `void` を返す組み込み関数を呼び出し、引数として現在の `scm_thread*` を渡します。

指示: **call-thread-scm** `s24:a c32:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-call_002dthread_002dscm)

インデックス idx を指定して `void` を返す組み込み関数を呼び出し、引数として現在の `scm_thread*` と `scm` ローカル変数 a を渡します。

指示: **call-thread-scm-scm** `s12:a s12:b c32:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-call_002dthread_002dscm_002dscm)

インデックス idx を指定して `void` を返す組み込み関数を呼び出し、引数として現在の `scm_thread*` と `scm` ローカル変数 a および b を渡します。

指示: **call-scm-sz-u32** `s12:a s12:b c32:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-call_002dscm_002dsz_002du32)

インデックス idx を指定して `void` を返す組み込み関数を呼び出し、ローカル変数 a、b、c を引数として渡します。a は `scm` 型の値であり、b と c はそれぞれ `size_t` 型と `uint32_t` 型に収まる生の `u64` 型の値です。

指示: **call-scm<-thread** `s24:dst c32:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-call_002dscm_003c_002dthread)

インデックス idx を指定して、現在の `scm_thread*` を引数として渡して、`SCM` を返す組み込み関数を呼び出します。結果を dst に格納します。

指示: **call-scm<-u64** `s12:dst s12:a c32:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-call_002dscm_003c_002du64)

インデックス idx を指定して `SCM` を返す組み込み関数を呼び出し、引数として `u64` ローカル変数 a を渡します。結果を dst に格納します。

指示: **call-scm<-s64** `s12:dst s12:a c32:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-call_002dscm_003c_002ds64)

インデックス idx を指定して `SCM` を返す組み込み関数を呼び出し、引数としてローカル変数 `s64` の a を渡します。結果を dst に格納します。

指示: **call-scm<-scm** `s12:dst s12:a c32:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-call_002dscm_003c_002dscm)

インデックス idx を指定して `SCM` を返す組み込み関数を呼び出し、引数として `scm` ローカル変数 a を渡します。結果を dst に格納します。

指示: **call-u64<-scm** `s12:dst s12:a c32:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-call_002du64_003c_002dscm)

インデックス idx を指定して `uint64_t` を返す組み込み関数を呼び出し、引数として `scm` ローカル変数 a を渡します。`u64` 型の結果を dst に格納します。

指示: **call-s64<-scm** `s12:dst s12:a c32:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-call_002ds64_003c_002dscm)

インデックス idx を指定して `int64_t` を返す組み込み関数を呼び出し、引数としてローカル変数 `scm` の a を渡します。`s64` の結果を dst に格納します。

命令: **call-f64<-scm** `s12:dst s12:a c32:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-call_002df64_003c_002dscm)

インデックス idx を指定して `double` を返す組み込み関数を呼び出し、引数としてローカル変数 `scm` の a を渡します。`f64` の結果を dst に格納します。

指示: **call-scm<-scm-scm** `s8:dst s8:a s8:b c32:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-call_002dscm_003c_002dscm_002dscm)

インデックス idx を指定して、`SCM` を返す組み込み関数を呼び出し、引数として `scm` のローカル変数 a と b を渡します。`scm` の結果を dst に格納します。

指示: **call-scm<-scm-uimm** `s8:dst s8:a c8:b c32:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-call_002dscm_003c_002dscm_002duimm)

インデックス idx を指定して、`scm` ローカル変数 a と `uint8_t` 即値変数 b を引数として渡して、`SCM` を返す組み込み関数を呼び出します。`scm` の結果を dst に格納します。

命令: **call-scm<-thread-scm** `s12:dst s12:a c32:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-call_002dscm_003c_002dthread_002dscm)

インデックス idx を指定して、現在の `scm_thread*` と `scm` ローカル変数 a を引数として渡して、`SCM` を返す組み込み関数を呼び出します。`scm` の結果を dst に格納します。

指示: **call-scm<-scm-u64** `s8:dst s8:a s8:b c32:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-call_002dscm_003c_002dscm_002du64)

インデックス idx を指定して、`SCM` を返す組み込み関数を呼び出し、引数として `scm` ローカル変数 a と `u64` ローカル変数 b を渡します。`scm` の結果を dst に格納します。

指示: **call-scm-scm** `s12:a s12:b c32:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-call_002dscm_002dscm)

インデックス idx を指定して `void` を返す組み込み関数を呼び出し、引数として `scm` ローカル変数 a と b を渡します。

指示: **call-scm-scm-scm** `s8:a s8:b s8:c c32:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-call_002dscm_002dscm_002dscm)

インデックス idx を指定して `void` を返す組み込み関数を呼び出し、引数として `scm` ローカル変数 a、b、c を渡します。

指示: **call-scm-uimm-scm** `s8:a c8:b s8:c c32:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-call_002dscm_002duimm_002dscm)

インデックス idx を持つ `void` を返す組み込み関数を呼び出し、引数として `scm` ローカル a、`uint8_t` 即値 b、および `scm` ローカル c を渡します。

特定の組み込み関数に対応するマクロ命令があります。これらは、適切な組み込み関数IDx引数を指定した`call-instrinsic-kind`命令と同等です。

マクロ命令: **add** dst ab [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-add)

マクロ命令: **add/immediate** dst ab/imm [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-add_002fimmediate)

`SCM`の値aとbを加算し、結果をdstに格納します。

マクロ命令: **sub** dst ab [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sub)

マクロ命令: **sub/immediate** dst ab/imm [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sub_002fimmediate)

aから`SCM`値bを減算し、その結果をdstに格納します。

マクロ命令: **mul** dst ab [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-mul)

`SCM`の値aとbを乗算し、その結果をdstに格納します。

マクロ命令: **div** dst ab [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-div-1)

`SCM`の値aをbで割り、その結果をdstに格納します。

マクロ命令: **quo** dst ab [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-quo)

`SCM`の値aとbの商を計算し、その結果をdstに格納します。

マクロ命令: **rem** dst ab [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-rem)

`SCM`の値aとbの残差を計算し、その結果をdstに格納します。

マクロ命令: **mod** dst ab [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-mod-1)

`SCM`値aをbで割った剰余を計算し、その結果をdstに格納します。

マクロ命令: **logand** dst ab [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-logand-1)

`SCM` の値 a と b のビットごとの `and` を計算し、結果を dst に格納します。

マクロ命令: **logior** dst ab [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-logior-1)

`SCM` の値 a と b のビットごとの包含的 `OR` を計算し、結果を dst に格納します。

マクロ命令: **logxor** dst ab [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-logxor-1)

`SCM` の値 a と b のビットごとの排他的 `OR` を計算し、結果を dst に格納します。

マクロ命令: **logsub** dst ab [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-logsub)

`SCM` 値 a と b のビットごとの `AND` を計算し、結果を dst に格納します。

マクロ命令: **lsh** dst ab [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-lsh)

マクロ命令: **lsh/immediate** ab/imm [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-lsh_002fimmediate)

`SCM` 値 a を `u64` 値 b ビット左にシフトし、結果を dst に格納します。

マクロ命令: **rsh** dst ab [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-rsh)

マクロ命令: **rsh/immediate** dst ab/imm [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-rsh_002fimmediate)

`SCM` 値 a を `u64` 値 b ビットだけ右シフトし、結果を dst に格納します。

マクロ命令: **scm->f64** dst src [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_002d_003ef64)

src をボックス化されていない `f64` に変換し、結果を dst に格納します。src が実数でない場合はエラーが発生します。

マクロ命令: **scm->u64** dst src [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_002d_003eu64)

src をボックス化されていない `u64` に変換し、結果を dst に格納します。src が範囲内の整数でない場合はエラーが発生します。

マクロ命令: **scm->u64/truncate** dst src [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_002d_003eu64_002ftruncate)

src をボックス化されていない `u64` に変換し、結果を dst に格納します。下位 64 ビットに切り捨てます。src が整数でない場合はエラーが発生します。

マクロ命令: **scm->s64** dst src [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_002d_003es64)

src をボックス化されていない `s64` に変換し、結果を dst に格納します。src が範囲内の整数でない場合はエラーが発生します。

マクロ命令: **u64->scm** dst src [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-u64_002d_003escm)

u64 値 src を Scheme 整数に変換して dst に格納します。

マクロ命令: **s64->scm** scm<-s64 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-s64_002d_003escm)

s64 値 src を Scheme 整数に変換して dst に格納します。

マクロ命令: **string-set!** str idx ch [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dset_0021-1)

文字列strの文字idx（`u64`）をch（有効な文字値である`u64`）に設定します。

マクロ命令: **string->number** dst src [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002d_003enumber-2)

srcに対して`string->number`を呼び出し、結果をdstに格納します。

マクロ命令: **string->symbol** dst src [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002d_003esymbol- 2)

srcに対して`string->symbol`を呼び出し、結果をdstに格納します。

マクロ命令: **symbol->keyword** dst src [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-symbol_002d_003ekeyword-1)

srcに対して`symbol->keyword`を呼び出し、結果をdstに格納します。

マクロ命令: **class-of** dst src [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-class_002dof-1)

dstを`src`のGOOPSクラスに設定します。

マクロ命令: **wind** ワインダー アンワインダー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-wind)

wind および unwind プロシージャを動的スタックにプッシュします。実際にはどちらも呼び出されません。コンパイラは、通常の動的風化制御フローのために winder および unwinder への呼び出しを生成する必要があります。また、コンパイラが winder および unwinder がサンクであることを証明できない場合は、そのチェックを挿入する必要があることにも注意してください。[Dynamic Wind](https://doc.guix.gnu.org/guile/latest/en/guile.html#Dynamic-Wind) を参照してください。

マクロ命令: **unwind** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-unwind)

式の動的範囲から抜け出し、動的スタックから最上位のエントリを削除します。

マクロ命令: **push-fluid** 流体値 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-push_002dfluid)

with-fluids オブジェクトを作成し、そのオブジェクトを動的スタックにプッシュすることで、値を流体に動的にバインドします。[流体と動的状態](https://doc.guix.gnu.org/guile/latest/en/guile.html#Fluids-and-Dynamic-States) を参照してください。

マクロ命令: **pop-fluid** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-pop_002dfluid)

`with-fluid*`式の動的な範囲をそのままにして、流体を以前の値に戻します。`push-fluid`は常に`pop-fluid`とバランスを取る必要があります。

マクロ命令: **fluid-ref** dst fluid [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-fluid_002dref-1)

流体流体に関連付けられた値をdstに配置します。

マクロ命令: **fluid-set!** 流体値 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-fluid_002dset_0021-1)

流体の値を値に設定します。

マクロ命令: **push-dynamic-state** state [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-push_002ddynamic_002dstate)

現在の流体バインディングのセットを動的スタックに保存し、代わりに状態からバインディングをインステートします。[流体と動的状態](https://doc.guix.gnu.org/guile/latest/en/guile.html#Fluids-and-Dynamic-States)を参照してください。

マクロ命令: **pop-dynamic-state** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-pop_002ddynamic_002dstate)

動的スタックから保存済みの流体バインディングセットを復元します。`push-dynamic-state` は常に `pop-dynamic-state` とバランスが取れている必要があります。

マクロ命令: **resolve-module** dst name public? [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-resolve_002dmodule-1)

名前のモジュールを検索し、直接オペランドpublic? が true の場合にそのパブリック インターフェースを解決し、結果を dst に格納します。

マクロ命令: **lookup** dst mod sym [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-lookup)

モジュール mod で sym を検索し、結果として得られた変数 (見つからない場合は `#f`) を dst に配置します。

マクロ命令: **define!** dst mod sym [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-define_0021)

モジュール mod で sym を検索し、結果として得られる変数を dst に配置し、必要に応じて変数を作成します。

マクロ命令: **current-module** dst [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-current_002dmodule-1)

dstを現在のモジュールに設定します。

マクロ命令: **$car** dst src [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024car)

マクロ命令: **$cdr** dst src [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024cdr)

マクロ命令: **$set-car!** x val [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024set_002dcar_0021)

マクロ命令: **$set-cdr!** x val [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024set_002dcdr_0021)

マクロ命令: **$variable-ref** dst src [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024variable_002dref)

マクロ命令: **$variable-set!** x val [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024variable_002dset_0021)

マクロ命令: **$vector-length** dst x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024vector_002dlength)

マクロ命令: **$vector-ref** dst x idx [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024vector_002dref)

マクロ命令: **$vector-ref/immediate** dst x idx/imm [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024vector_002dref_002fimmediate)

マクロ命令: **$vector-set!** x idx v [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024vector_002dset_0021)

マクロ命令: **$vector-set!/immediate** x idx/imm v [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024vector_002dset_0021_002fimmediate)

マクロ命令: **$allocate-struct** dst vtable nwords [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024allocate_002dstruct)

マクロ命令: **$struct-vtable** dst src [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024struct_002dvtable)

マクロ命令: **$struct-ref** dst src idx [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024struct_002dref)

マクロ命令: **$struct-ref/immediate** dst src idx/imm [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024struct_002dref_002fimmediate)

マクロ命令: **$struct-set!** x idx v [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024struct_002dset_0021)

マクロ命令: **$struct-set!/immediate** x idx/imm v [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024struct_002dset_0021_002fimmediate)

ベースラインコンパイラで使用する組み込み関数。CPSコンパイルの一般的な戦略は、例えば`vector-ref`などの構成要素を公開し、コンパイラがそれらから学習して不要な部分を削除することです。しかし、最適化を行わないベースラインコンパイラでは、それは単なるオーバーヘッドとなるため、通常の型チェックをすべてカプセル化する組み込み関数を用意しました。

* * *

次へ: [メモリアクセス命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Memory-Access-Instructions)、前: [組み込み関数呼び出し命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Intrinsic-Call-Instructions)、上: [命令セット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instruction-Set) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.3.7.8 定数命令 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Constant-Instructions-1)

以下の命令は、リテラルデータをプログラムに読み込むためのものです。リテラルデータには2種類あります。

最初の命令セットは即値をロードします。これらの命令は即値を命令ストリームに直接エンコードします。

指示: **make-immediate** `s8:dst zi16:low-bits` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dimmediate)

下位ビットが符号拡張された下位ビットである即値を作成します。

命令: **make-short-immediate** `s8:dst i16:low-bits` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dshort_002dimmediate)

下位ビットが下位ビットで、上位ビットが0である即値を作成します。

命令: **make-long-immediate** `s24:dst i32:low-bits` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dlong_002dimmediate)

下位ビットが下位ビットで、上位ビットが0である即値を作成します。

命令: **make-long-long-immediate** `s24:dst a32:high-bits b32:low-bits` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dlong_002dlong_002dimmediate)

上位ビットと下位ビットを使って即値を作成します。

非即時定数リテラルは、直接または間接的に参照されます。たとえば、Guile はコンパイル時に文字列のレイアウトを把握し、そのオブジェクトをコンパイル済みイメージに直接埋め込むように調整します。文字列への参照は、`make-non-immediate` を使用して、コンパイル単位へのポインタを `scm` 値として直接扱います。

指示: **make-non-immediate** `s24:dst n32:offset` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dnon_002dimmediate)

静的に割り当てられたメモリへのポインタをdstにロードします。オブジェクトのメモリは、現在の命令ポインタから32ビットワード離れたオフセット位置にあります。オブジェクトが可変か不変かは、コンパイラによって割り当てられた場所と、ローダーによってロードされた場所によって決まります。

レジスタにコードポインタをロードする必要がある場合があります。そのためには、`load-label`を使用します。

指示: **load-label** `s24:dst l32:offset` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-load_002dlabel)

現在の `ip` からオフセットワード離れたラベルをロードし、dst に書き込みます。offset は符号付き 32ビット整数です。

最後に、Guileは、関連する定数ローダーを備えた、多数の非ボックス型データ型をサポートしています。

命令: **load-f64** `s24:dst au32:high-bits au32:low-bits` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-load_002df64)

上位ビットと下位ビットを結合して生成された倍精度浮動小数点値をロードし、dstに書き込みます。

命令: **load-u64** `s24:dst au32:high-bits au32:low-bits` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-load_002du64)

上位ビットと下位ビットを結合して生成された符号なし64ビット整数をロードし、dstに書き込みます。

命令: **load-s64** `s24:dst au32:high-bits au32:low-bits` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-load_002ds64)

上位ビットと下位ビットを結合して生成された符号付き64ビット整数をロードし、dstに書き込みます。

システム全体で一意である必要があるオブジェクトがいくつかあります。シンボルとキーワードがこれに該当します。Guileはこれらのオブジェクトについて、コンパイルユニットのロード時に初期化を行い、イメージ内のスロットに格納します。参照は、このスロットを介して間接的に行われます。この場合、`static-ref`が使用されます。

指示: **static-ref** `s24:dst r32:offset` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-static_002dref)

scm 値を dst にロードします。scm 値は、現在の命令ポインタから 32 ビットワード離れたメモリからフェッチされます。offset は符号付き値です。

非即値オブジェクトのフィールドは、ロード時に修正する必要がある場合があります。これは、それらがどのアドレスにロードされるか事前にわからないためです。たとえば、フィールドの1つに非即値オブジェクトを含むペアの場合がこれに該当します。このような状況では、`static-set!` と `static-patch!` が使用されます。

指示: **static-set!** `s24:src lo32:offset` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-static_002dset_0021)

現在の命令ポインタから32ビットワード離れた位置に、scm値をメモリに格納します。オフセットは符号付き値です。

指示: **static-patch!** `x24:_ lo32:dst-offset l32:src-offset` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-static_002dpatch_0021)

dst-offset にあるポインタを src-offset を指すようにパッチします。どちらのオフセットも符号付き 32 ビット値であり、現在の命令ポインタから 32 ビットワード離れたメモリ アドレスを示します。

* * *

次へ: [アトミックメモリアクセス命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Atomic-Memory-Access-Instructions)、前: [定数命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Constant-Instructions)、上: [命令セット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instruction-Set) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.3.7.9 メモリアクセス命令 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Memory-Access-Instructions-1)

これらの指示では、`/immediate` バリアントは、インデックスまたはカウントを即時値として表します。それ以外の場合は、これらの値はボックス化されていない u64 ローカル変数です。

指示: **allocate-words** `s12:dst s12:count` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-allocate_002dwords)

指示: **allocate-words/immediate** `s12:dst c12:count` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-allocate_002dwords_002fimmediate)

カウントワードで構成される新しいGCトレース済みオブジェクトを割り当て、それをdstに格納します。

指示: **scm-ref** `s8:dst s8:obj s8:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_002dref)

指示: **scm-ref/immediate** `s8:dst s8:obj c8:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_002dref_002fimmediate)

ローカルオブジェクト obj からワードオフセット idx にある `SCM` オブジェクトをロードし、dst に格納します。

指示: **scm-set!** `s8:dst s8:idx s8:obj` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_002dset_0021)

指示: **scm-set!/immediate** `s8:dst c8:idx s8:obj` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_002dset_0021_002fimmediate)

`scm` ローカル値をオブジェクト obj のワードオフセット idx に格納します。

指示: **scm-ref/tag** `s8:dst s8:obj c8:tag` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_002dref_002ftag)

obj の最初の単語を読み込み、その直後のタグを減算し、結果として得られた `SCM` を dst に格納します。

指示: **scm-set!/tag** `s8:obj c8:tag s8:val` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_002dset_0021_002ftag)

obj の最初の単語を、`scm` 値 val の展開されたビットと即時値タグに置き換えた値に設定します。

指示: **word-ref** `s8:dst s8:obj s8:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-word_002dref)

指示: **word-ref/immediate** `s8:dst s8:obj c8:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-word_002dref_002fimmediate)

ローカルオブジェクトからオフセットidxにあるワードをロードし、ローカルdstの`u64`に格納します。

指示: **word-set!** `s8:dst s8:idx s8:obj` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-word_002dset_0021)

指示: **word-set!/immediate** `s8:dst c8:idx s8:obj` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-word_002dset_0021_002fimmediate)

`u64` ローカル値をオブジェクト obj のワードオフセット idx に格納します。

指示: **pointer-ref/immediate** `s8:dst s8:obj c8:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-pointer_002dref_002fimmediate)

ローカルオブジェクトからオフセットidxにあるポインタをロードし、それをアンボックス化されたポインタlocal dstに格納します。

命令: **pointer-set!/immediate** `s8:dst c8:idx s8:obj` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-pointer_002dset_0021_002fimmediate)

ローカル変数valのアンボックスポインタを、オブジェクトobjのワードオフセットidxに格納します。

指示: **tail-pointer-ref/immediate** `s8:dst s8:obj c8:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-tail_002dpointer_002dref_002fimmediate)

ローカルオブジェクトobjからワードオフセットidxのアドレスを計算し、それをdstに格納します。

* * *

次へ: [タグ付けとタグ解除命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Tagging-and-Untagging-Instructions)、前: [メモリアクセス命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Memory-Access-Instructions)、上: [命令セット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instruction-Set) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.3.7.10 アトミックメモリアクセス命令 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Atomic-Memory-Access-Instructions-1)

指示: **current-thread** `s24:dst` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-current_002dthread-2)

現在のスレッドをdstに書き込む。

指示: **atomic-scm-ref/immediate** `s8:dst s8:obj c8:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-atomic_002dscm_002dref_002fimmediate)

シーケンシャル一貫性メモリモデルを使用して、ローカルオブジェクト obj からワードオフセット idx にある `SCM` オブジェクトをアトミックにロードします。結果を dst に格納します。

指示: **atomic-scm-set!/immediate** `s8:obj c8:idx s8:val` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-atomic_002dscm_002dset_0021_002fimmediate)

シーケンシャル一貫性メモリモデルを使用して、ローカルオブジェクト obj からワードオフセット idx にある `SCM` オブジェクトを val にアトミックに設定します。

命令: **atomic-scm-swap!/immediate** `s24:dst x8:_ s24:obj c8:idx s24:val` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-atomic_002dscm_002dswap_0021_002fimmediate)

逐次一貫性メモリモデルを使用して、オブジェクト obj のワードオフセット idx に格納されている `SCM` 値を val とアトミックに交換します。交換後の値は dst に格納します。

指示: **atomic-scm-compare-and-swap!/immediate** `s24:dst x8:_ s24:obj c8:idx s24:expected x8:_ s24:desired` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-atomic_002dscm_002dcompare_002dand_002dswap_0021_002fimmediate)

逐次一貫性メモリモデルを使用して、オブジェクト obj のワードオフセット idx に格納されている `SCM` 値を、期待される値とアトミックに交換します。交換は、元の値が期待どおりであった場合に限ります。交換後の値は、obj の idx に格納されていた値を dst に格納します。

* * *

次へ: [整数演算命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Integer-Arithmetic-Instructions)、前: [アトミックメモリアクセス命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Atomic-Memory-Access-Instructions)、上: [命令セット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instruction-Set) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.3.7.11 タグ付けとタグ解除の手順 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Tagging-and-Untagging-Instructions-1)

指示: **tag-char** `s12:dst s12:src` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-tag_002dchar)

src の `u64` を整数値とする `SCM` 文字を作成し、それを dst に格納します。

指示: **untag-char** `s12:dst s12:src` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-untag_002dchar)

`SCM`文字srcから整数値を抽出し、結果として得られた`u64`をdstに格納します。

指示: **tag-fixnum** `s12:dst s12:src` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-tag_002dfixnum)

src 内の `s64` を値とする `SCM` 整数を作成し、それを dst に格納します。

指示: **untag-fixnum** `s12:dst s12:src` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-untag_002dfixnum)

`SCM`整数srcから整数値を抽出し、結果として得られた`s64`をdstに格納します。

* * *

次へ: [浮動小数点演算命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Floating_002dPoint-Arithmetic-Instructions)、前: [タグ付けおよびタグ解除命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Tagging-and-Untagging-Instructions)、上: [命令セット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instruction-Set) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.3.7.12 整数演算命令 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Integer-Arithmetic-Instructions-1)

命令: **uadd** `s8:dst s8:a s8:b` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-uadd)

命令: **uadd/immediate** `s8:dst s8:a c8:b` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-uadd_002fimmediate)

`u64`値aとbを加算し、その結果を`u64`形式でdstに格納します。オーバーフローが発生すると、値は折り返されます。

命令: **usub** `s8:dst s8:a s8:b` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-usub)

指示: **usub/immediate** `s8:dst s8:a c8:b` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-usub_002fimmediate)

aから`u64`値bを減算し、その結果の`u64`値をdstに格納します。アンダーフローが発生すると、オーバーフローが発生します。

指示: **umul** `s8:dst s8:a s8:b` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-umul)

指示: **umul/immediate** `s8:dst s8:a c8:b` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-umul_002fimmediate)

`u64`値aとbを乗算し、その結果を`u64`形式でdstに格納します。オーバーフローが発生すると、値が折り返されます。

命令: **ulogand** `s8:dst s8:a s8:b` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ulogand)

`u64`値aとbのビットごとのAND演算結果を`u64`ローカルdstに格納します。

指示: **ulogior** `s8:dst s8:a s8:b` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ulogior)

`u64` 値 a と b のビットごとの `or` を `u64` ローカル dst に格納します。

指示: **ulogxor** `s8:dst s8:a s8:b` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ulogxor)

`u64`値aとbのビットごとの排他的論理和を`u64`ローカルdstに格納します。

指示: **ulogsub** `s8:dst s8:a s8:b` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ulogsub)

`u64` 値 a のビットごとの `and` と b のビットごとの `not` を `u64` ローカル変数 dst に格納します。

指示: **ulsh** `s8:dst s8:a s8:b` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ulsh)

指示: **ulsh/immediate** `s8:dst s8:a c8:b` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ulsh_002fimmediate)

a に格納されている非ボックス化符号なし 64 ビット整数を b ビット左シフトし、同じく非ボックス化符号なし 64 ビット整数を生成します。64 ビットに切り捨て、非ボックス化値として dst に書き込みます。b の下位 6 ビットのみが使用されます。

命令: **ursh** `s8:dst s8:a s8:b` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ursh)

指示: **ursh/immediate** `s8:dst s8:a c8:b` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ursh_002fimmediate)

a に格納されている非ボックス化された符号なし 64 ビット整数を、同じく非ボックス化された符号なし 64 ビット整数である b ビットだけ右シフトします。64 ビットに切り捨て、非ボックス化された値として dst に書き込みます。b の下位 6 ビットのみが使用されます。

命令: **srsh** `s8:dst s8:a s8:b` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-srsh)

指示: **srsh/immediate** `s8:dst s8:a c8:b` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-srsh_002fimmediate)

a に格納されている符号なし 64 ビット整数を、同じく符号なし 64 ビット整数である b ビットだけ右シフトします。64 ビットに切り捨て、dst に符号なし値として書き込みます。b の下位 6 ビットのみが使用されます。

* * *

次へ: [比較命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Comparison-Instructions)、前: [整数演算命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Integer-Arithmetic-Instructions)、上: [命令セット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instruction-Set) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.3.7.13 浮動小数点演算命令 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Floating_002dPoint-Arithmetic-Instructions-1)

指示: **fadd** `s8:dst s8:a s8:b` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-fadd)

`f64`の値aとbを加算し、その結果を`f64`としてdstに格納します。

指示: **fsub** `s8:dst s8:a s8:b` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-fsub)

aから`f64`値bを減算し、その結果の`f64`をdstに格納します。

指示: **fmul** `s8:dst s8:a s8:b` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-fmul)

`f64` の値 a と b を乗算し、その結果を `f64` として dst に格納します。

指示: **fdiv** `s8:dst s8:a s8:b` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-fdiv)

`f64` の値 a を b で割り、その結果の `f64` を dst に格納します。

* * *

次へ: [分岐命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Branch-Instructions)、前: [浮動小数点演算命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Floating_002dPoint-Arithmetic-Instructions)、上: [命令セット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instruction-Set) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.3.7.14 比較命令 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Comparison-命令-1)

比較命令は比較結果を設定し、その結果は[分岐命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Branch-Instructions)で使用できます。

指示: **u64=?** `s12:a s12:b` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-u64_003d_003f)

`u64`値aとbが同じ場合は比較結果をEQUALに設定し、そうでない場合は`NONE`に設定します。

指示: **u64<?** `s12:a s12:b` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-u64_003c_003f)

`u64` 値 a が `u64` 値 b より小さいか同じである場合は比較結果を `LESS_THAN` に設定し、そうでない場合は `NONE` に設定します。

指示: **s64<?** `s12:a s12:b` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-s64_003c_003f)

`s64` の値 a が `s64` の値 b より小さいか同じである場合は比較結果を `LESS_THAN` に設定し、そうでない場合は `NONE` に設定します。

指示: **s64-imm=?** `s12:a z12:b` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-s64_002dimm_003d_003f)

`s64` の値 a が直近の `s64` の値 b と等しい場合は比較結果を EQUAL に設定し、そうでない場合は `NONE` に設定します。

指示: **u64-imm<?** `s12:a c12:b` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-u64_002dimm_003c_003f)

`u64` 値 a が直近の `u64` 値 b より小さい場合は比較結果を `LESS_THAN` に設定し、そうでない場合は `NONE` に設定します。

指示: **imm-u64<?** `s12:a s12:b` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-imm_002du64_003c_003f)

`u64` の即値 b が `u64` の値 a より小さい場合は比較結果を `LESS_THAN` に設定し、そうでない場合は `NONE` に設定します。

指示: **s64-imm<?** `s12:a z12:b` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-s64_002dimm_003c_003f)

`s64` の値 a が直近の `s64` の値 b より小さい場合は比較結果を `LESS_THAN` に設定し、そうでない場合は `NONE` に設定します。

指示: **imm-s64<?** `s12:a z12:b` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-imm_002ds64_003c_003f)

`s64` の即値 b が `s64` の値 a より小さい場合は比較結果を `LESS_THAN` に設定し、そうでない場合は `NONE` に設定します。

指示: **f64=?** `s12:a s12:b` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-f64_003d_003f)

f64値aとf64値bが等しい場合は比較結果をEQUALに設定し、そうでない場合はNONEに設定します。

指示: **f64<?** `s12:a s12:b` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-f64_003c_003f)

f64値aがf64値bより小さい場合は比較結果を`LESS_THAN`、aがb以上の場合は`NONE`、それ以外の場合は`INVALID`に設定します。

指示: **\=?** `s12:a s12:b` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d_003f)

SCM値aとbが、スキームの演算子「=」の意味において数値的に等しい場合、比較結果を「EQUAL」に設定します。そうでない場合は「NONE」に設定します。

指示: **heap-numbers-equal?** `s12:a s12:b` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-heap_002dnumbers_002dequal_003f)

SCM値aとbがスキーム「=」の意味で数値的に等しい場合、比較結果をEQUALに設定します。そうでない場合は「NONE」に設定します。aとbはどちらもヒープ数であることがわかっています。

指示: **<?** `s12:a s12:b` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003c_003f)

SCM値aがSCM値bより小さい場合は比較結果を`LESS_THAN`、aがb以上の場合は`NONE`、それ以外の場合は`INVALID`に設定します。

指示: **immediate-tag=?** `s24:obj c16:mask c16:tag` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-immediate_002dtag_003d_003f)

`scm` 値 obj のビットと即値マスクのビット間のビットごとの `and` の結果が tag である場合は比較結果を EQUAL に設定し、そうでない場合は `NONE` に設定します。

指示: **heap-tag=?** `s24:obj c16:mask c16:tag` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-heap_002dtag_003d_003f)

`scm` 値 obj の最初のワードと即値マスクとのビットごとの `and` の結果が tag である場合は比較結果を EQUAL に設定し、そうでない場合は `NONE` に設定します。

指示: **eq?** `s12:a s12:b` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-eq_003f-2)

SCM値aとbが`eq?`の場合は比較結果をEQUALに設定し、そうでない場合は`NONE`に設定します。

指示: **eq-immediate?** `s8:a zi16:b` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-eq_002dimmediate_003f)

SCM値aが直近のSCM値b（符号拡張）と等しい場合は比較結果をEQUALに設定し、そうでない場合はNONEに設定します。

`immediate-tag=?` および `heap-tag=?` には、正確な型タグ値を抽象化するマクロ命令のセットも用意されています。[Guile の SCM 型](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-SCM-Type-in-Guile) を参照してください。

マクロ命令: **fixnum?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-fixnum_003f-1)

マクロ命令: **heap-object?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-heap_002dobject_003f)

マクロ命令: **char?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_003f-2)

マクロ命令: **eq-false?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-eq_002dfalse_003f)

マクロ命令: **eq-nil?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-eq_002dnil_003f)

マクロ命令: **eq-null?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-eq_002dnull_003f)

マクロ命令: **eq-true?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-eq_002dtrue_003f)

マクロ命令: **未指定?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-unspecified_003f)

マクロ命令: **未定義?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-undefined_003f)

マクロ命令: **eof-object?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-eof_002dobject_003f-3)

マクロ命令: **null?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-null_003f-2)

マクロ命令: **false?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-false_003f)

マクロ命令: **nil?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-nil_003f-1)

xが対応する述語（例：`null?`）を通過する場合は比較結果を`EQUAL`に設定し、そうでない場合は`NONE`に設定する`immediate-tag=?`命令を出力します。

マクロ命令: **pair?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-pair_003f-2)

マクロ命令: **struct?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-struct_003f-1)

マクロ命令: **symbol?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-symbol_003f-2)

マクロ命令: **変数?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-variable_003f-1)

マクロ命令: **vector?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-vector_003f-3)

マクロ命令: **immutable-vector?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-immutable_002dvector_003f)

マクロ命令: **mutable-vector?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-mutable_002dvector_003f)

マクロ命令: **weak-vector?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-weak_002dvector_003f-1)

マクロ命令: **string?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_003f-2)

マクロ命令: **heap-number?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-heap_002dnumber_003f)

マクロ命令: **hash-table?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hash_002dtable_003f-1)

マクロ命令: **ポインタ?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-pointer_003f-1)

マクロ命令: **fluid?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-fluid_003f-1)

マクロ命令: **stringbuf?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-stringbuf_003f)

マクロ命令: **dynamic-state?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-dynamic_002dstate_003f-1)

マクロ命令: **frame?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-frame_003f-1)

マクロ命令: **キーワード?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-keyword_003f-2)

マクロ命令: **atomic-box?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-atomic_002dbox_003f-1)

マクロ命令: **構文?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-syntax_003f)

マクロ命令: **プログラム?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-program_003f-1)

マクロ命令: **vm-continuation?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-vm_002dcontinuation_003f)

マクロ命令: **bytevector?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_003f-1)

マクロ命令: **weak-set?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-weak_002dset_003f)

マクロ命令: **weak-table?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-weak_002dtable_003f)

マクロ命令: **array?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-array_003f-1)

マクロ命令: **bitvector?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bitvector_003f-1)

マクロ命令: **smob?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-smob_003f)

マクロ命令: **port?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-port_003f-2)

マクロ命令: **bignum?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bignum_003f)

マクロ命令: **flonum?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-flonum_003f-1)

マクロ命令: **compnum?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-compnum_003f)

マクロ命令: **fracnum?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-fracnum_003f)

xが対応する述語（例：`null?`）を通過する場合は比較結果を`EQUAL`に設定し、そうでない場合は`NONE`に設定する`heap-tag=?`命令を出力します。

* * *

次へ: [生メモリアクセス命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Raw-Memory-Access-Instructions)、前: [比較命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Comparison-Instructions)、上: [命令セット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instruction-Set) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.3.7.15 分岐手順[¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Branch-Instructions-1)

分岐命令へのオフセットはすべて24ビット符号付き数値であり、32ビット単位でカウントされます。これにより、Guileは相対ジャンプに対して実質的に26ビットのアドレス範囲を持つことになります。

指示: **j** `l24:offset` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-j)

現在の命令ポインタにオフセットを加算します。

指示: **jl** `l24:offset` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-jl)

最後の比較結果が「LESS_THAN」の場合、符号付き24ビット数値であるオフセットを現在の命令ポインタに加算します。

指示: **je** `l24:offset` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-je)

最後の比較結果が「EQUAL」の場合、符号付き24ビット数であるオフセットを現在の命令ポインタに加算します。

指示: **jnl** `l24:offset` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-jnl)

最後の比較結果が`LESS_THAN`でない場合、符号付き24ビット数であるオフセットを現在の命令ポインタに加算します。

命令: **jne** `l24:offset` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-jne)

最後の比較結果が「EQUAL」でない場合、符号付き24ビット数であるオフセットを現在の命令ポインタに加算します。

指示: **jge** `l24:offset` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-jge)

最後の比較結果が「NONE」の場合、符号付き24ビット数値であるオフセットを現在の命令ポインタに加算します。

これは`<?`比較の後に使用することを想定しており、非数（NaN）値の扱い方が`jnl`とは異なります。`<?`は、どちらかの値がNaNの場合、`NONE`ではなく`INVALID`を設定します。正確な数値の場合、`jge`は`jnl`と同じです。

指示: **jnge** `l24:offset` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-jnge)

最後の比較結果が「NONE」でない場合、符号付き24ビット数値であるオフセットを現在の命令ポインタに加算します。

これは`<?`比較の後に使用することを想定しており、非数（NaN）値の扱い方が`jl`とは異なります。`<?`は、どちらかの値がNaNの場合、`NONE`ではなく`INVALID`を設定します。正確な数値の場合、`jnge`は`jl`と同じです。

指示: **jtable** `s24:idx v32:length [x8:_ l24:offset]...` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-jtable)

C言語の`switch`文のように、テーブルのエントリに分岐します。idxは、分岐先のエントリを示す`u64`ローカル変数です。即値lenはテーブルのエントリ数を示し、1以上である必要があります。テーブルの最後のエントリは「キャッチオール」エントリです。offset...の値は符号付き24ビット即値（`l24`エンコーディング）で、現在の命令ポインタから32ビットワード離れたメモリ アドレスを示します。

* * *

前へ: [分岐命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Branch-Instructions)、上へ: [命令セット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instruction-Set) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.3.7.16 生メモリアクセス命令 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Raw-Memory-Access-Instructions-1)

バイトベクトル演算は現在のハードウェアの機能にほぼ対応しているため、VM命令にインライン化することで、最終的なネイティブコンパイルへの明確な道筋が確保されます。これがなければ、Schemeプログラムは生バイトにアクセスするために他のプリミティブを必要としますが、これらのプリミティブは他のプリミティブと遜色ありません。

指示: **u8-ref** `s8:dst s8:ptr s8:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-u8_002dref)

指示: **s8-ref** `s8:dst s8:ptr s8:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-s8_002dref)

指示: **u16-ref** `s8:dst s8:ptr s8:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-u16_002dref)

指示: **s16-ref** `s8:dst s8:ptr s8:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-s16_002dref)

指示: **u32-ref** `s8:dst s8:ptr s8:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-u32_002dref)

指示: **s32-ref** `s8:dst s8:ptr s8:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-s32_002dref)

命令: **u64-ref** `s8:dst s8:ptr s8:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-u64_002dref)

命令: **s64-ref** `s8:dst s8:ptr s8:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-s64_002dref)

命令: **f32-ref** `s8:dst s8:ptr s8:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-f32_002dref)

命令: **f64-ref** `s8:dst s8:ptr s8:idx` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-f64_002dref)

生ポインタ local ptr からバイトオフセット idx の位置にあるアイテムを取得し、dst に格納します。すべてのアクセスはネイティブエンディアンを使用します。

idxの値は、ボックス化されていない符号なし64ビット整数である必要があります。

結果はすべて、符号付き64ビット整数、符号なし64ビット整数、またはIEEE倍精度浮動小数点数のいずれかとして、ボックス化されていない値としてスタックに書き込まれます。

指示: **u8-set!** `s8:ptr s8:idx s8:val` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-u8_002dset_0021)

指示: **s8-set!** `s8:ptr s8:idx s8:val` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-s8_002dset_0021)

指示: **u16-set!** `s8:ptr s8:idx s8:val` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-u16_002dset_0021)

指示: **s16-set!** `s8:ptr s8:idx s8:val` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-s16_002dset_0021)

命令: **u32-set!** `s8:ptr s8:idx s8:val` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-u32_002dset_0021)

指示: **s32-set!** `s8:ptr s8:idx s8:val` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-s32_002dset_0021)

命令: **u64-set!** `s8:ptr s8:idx s8:val` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-u64_002dset_0021)

指示: **s64-set!** `s8:ptr s8:idx s8:val` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-s64_002dset_0021)

指示: **f32-set!** `s8:ptr s8:idx s8:val` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-f32_002dset_0021)

指示: **f64-set!** `s8:ptr s8:idx s8:val` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-f64_002dset_0021)

valを、生ポインタlocal ptrが指すメモリ上のバイトオフセットidxに格納します。マルチバイト値は、ネイティブエンディアンを使用して書き込まれます。

idxの値は、ボックス化されていない符号なし64ビット整数である必要があります。

valの値はすべて、符号付き64ビット整数、符号なし64ビット整数、またはIEEE倍精度浮動小数点数のいずれかとして、ボックス化されていません。

* * *

前へ: [命令セット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instruction-Set)、上へ: [Guile 用仮想マシン](https://doc.guix.gnu.org/guile/latest/en/guile.html#A-Virtual-Machine-for-Guile) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.3.8 ジャストインタイムネイティブコード [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Just_002dIn_002dTime-Native-Code-1)

Guileの仮想マシンの最後の構成要素は、バイトコード命令をネイティブコードに変換するジャストインタイム（JIT）コンパイラです。関数を実行する際、バイトコード命令をネイティブコードにコンパイルする方が、仮想マシンが命令を解釈するよりも高速です。

JITコンパイラは、各関数に関連付けられたカウンタによって自動的に実行されます。カウンタは、関数が呼び出されたとき、および各ループの反復処理中にインクリメントされます。関数のカウンタが特定の値を超えると、その関数はJITコンパイルされます。詳細については、[計測手順](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instrumentation-Instructions)を参照してください。

GuileのJITコンパイラは、いわゆる「テンプレートJIT」と呼ばれるものです。このタイプのJITは非常にシンプルで、関数内の各命令に対して、JITコンパイラはその命令の種類に対応する汎用的な機械語のシーケンスを出力し、コンパイル対象の命令の特定のオペランドを参照するように汎用テンプレートを特殊化します。

テンプレートJITの最大の強みは、コード生成速度が非常に速い点にある。コンパイル対象のバイトコードに対して、時間のかかる分析を行う必要がないため、効率的にコードを生成することができる。

テンプレートJITは非常に予測しやすいという利点もあります。テンプレートJITが出力するネイティブコードは、対応するバイトコードと同じパフォーマンス特性を持ちながら、より高速に動作します。理論的には、テンプレートJITのマシンコードは実行時に参照される値に依存しないため、事前に生成しておくことも可能です。

この予測可能性により、テンプレートJITによって生成されるネイティブコードに結論が適用されることが分かっている上で、バイトコードの観点からシステムのパフォーマンスについて推論することが可能になります。

命令に対応するマシンコードは常に、その命令に対してインタプリタが行うのと同じタスクを実行するため、バイトコードとテンプレートJITを使用することで、Guileプログラマはバイトコードモデルに基づいてプログラムをデバッグできます。Guileプログラマがブレークポイントを設定すると、Guileはデバッグ対象のスレッドのJITを無効にし、インタプリタ（フックを実行するための対応するコードを持つ）にフォールバックします。[VMフック](https://doc.guix.gnu.org/guile/latest/en/guile.html#VM-Hooks)を参照してください。

Guile はネイティブ コードを生成するために、GNU Lightning のフォーク版を使用しています。この「Lightening」プロジェクトは、GNU Lightning のバックエンド サポートをベースに、Guile のニーズに合わせてライブラリの API と動作を調整することを目的としており、独立したプロジェクトとして開発されました。このコードは Guile のソース ディストリビューションに含まれています。詳細については、[https://gitlab.com/wingo/lightening](https://gitlab.com/wingo/lightening) を参照してください。2019 年半ば現在、Lightening は x86-64、ia32、ARMv7、および AArch64 アーキテクチャのコード生成をサポートしています。

テンプレートJITの弱点は2つあります。まず、高速に動作する必要のあるシンプルなバックエンドであるため、テンプレートJITは、より良いコードを生成するのに役立つ分析、特にグローバルレジスタの割り当てや命令の選択を行う時間がありません。

しかし、これは重要な投機的プログラム変換を実行できないという欠点に比べれば些細な問題です。例えば、Guileは式`(fx)`において、実際にはfが常に同じ関数を参照していることを認識できます。高度なJITコンパイラであれば、fを投機的に呼び出し箇所にインライン化し、さらに動的なチェックによってアサーションが依然として成り立つことを確認します。しかし、テンプレートJITは実行時にのみ判明する値には注意を払わないため、このような変換を行うことはできません。

この制限は、Guileの堅牢な事前最適化コンパイラによって部分的に緩和されます。このコンパイラは、最適化が常に有効であることが証明できる場合に、重要な最適化を事前に実行できます。また、低レベルのバイトコードによって、これらの最適化の効果（省略された型チェックなど）を表現できます。Guileのコンパイラの詳細については、[仮想マシンへのコンパイル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compiling-to-the-Virtual-Machine)を参照してください。

テンプレートJITで補完された、Schemeからバイトコードへの事前コンパイル戦略は、Schemeのやや静的な性質に特に適しています。Schemeプログラマは、自由変数参照の同一性を字句的に明確にするようなコードを書くことがよくあります。たとえば、`(fx)`式は`(let ((f (lambda (x) (1+ x)))) ...)`式の中に現れる可能性があり、また、`f`が特定のモジュールからインポートされ、そのバインディングがわかっていることがわかります。事前コンパイル技術は、多態性が少なく、一階述語論理プログラミングが多いSchemeのような言語にはうまく機能します。しかし、実行時に非常に変更可能で、メソッド呼び出し（実質的には高階呼び出し）のために解析が難しいJavaScriptのような言語にはあまり適していません。

とはいえ、現時点ではテンプレートJITはGuileにとってうまく機能しています。保守しやすいコードはわずか数千行で済み、Schemeプログラムの実行速度が向上し、GuileのScheme実装の大部分はScheme自体で記述されています。次のステップはおそらく、Schemeで記述されたコンパイラのバックエンドに、グローバルレジスタ割り当てと命令選択を行う機会を活用するために、ネイティブコードの事前生成機能を追加することでしょう。これが機能するようになれば、GuileはSchemeでの投機的最適化も試すことができるようになります。今後の方向性については、[Extending the Compiler](https://doc.guix.gnu.org/guile/latest/en/guile.html#Extending-the-Compiler)を参照してください。

最後に、JITコンパイルを早めたり、遅らせたり、あるいは実行しないように調整できる環境変数がいくつかあることに注意してください。詳しくは、[環境変数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Environment-Variables)を参照してください。

* * *

前へ: [Guile 用仮想マシン](https://doc.guix.gnu.org/guile/latest/en/guile.html#A-Virtual-Machine-for-Guile)、上へ: [Guile 実装](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guile-Implementation) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
