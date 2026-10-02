### 9.1 Guile の簡単な歴史 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#A-Brief-History-of-Guile)

Guileは、コードとしてもハッカーコミュニティとしても、歴史的な過程を経て生まれた産物です。ソースコードをハッキングする際には、過去の決定事項や将来の方向性を知るために、こうした歴史を理解しておくことが役立つ場合があります。

もちろん、Guileの真の歴史は、ハッカーたちがハッキングすることによって書かれるものであり、ライターが書くことによって書かれるものではありません。そこで、現状と今後の方向性について触れて、このセクションを締めくくりたいと思います。

* [Emacs Thesis](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Emacs-Thesis)
* [初期の頃](https://doc.guix.gnu.org/guile/latest/en/guile.html#Early-Days)
* [多数のメンテナーによるスキーム](https://doc.guix.gnu.org/guile/latest/en/guile.html#A-Scheme-of-Many-Maintainers)
* [Guileの主要リリース一覧](https://doc.guix.gnu.org/guile/latest/en/guile.html#A-Timeline-of-Selected-Guile-Releases)
* [ステータス、または：あなたの助けが必要です](https://doc.guix.gnu.org/guile/latest/en/guile.html#Status)

* * *

次へ: [初期](https://doc.guix.gnu.org/guile/latest/en/guile.html#Early-Days)、上へ: [Guile の簡単な歴史](https://doc.guix.gnu.org/guile/latest/en/guile.html#History) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.1.1 Emacs テーゼ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Emacs-Thesis-1)

Guileの物語は、Emacsの開発経験をGNUシステム上の膨大なプログラムにもたらす物語である。

Emacsは、1984年にGNU版として初めて開発された当時、「プログラムの作成方法」という問題に対する斬新なアプローチでした。Emacsの基本理念は、低レベル言語で記述された直交カーネルと、強力な高レベル拡張言語を組み合わせることで、複合的なプログラムを作成することが楽しい、というものです。

拡張言語は、拡張性の高いプログラム、つまり様々なユーザーや時代の変化に容易に適応できるプログラムを育成します。その証拠として、Emacsが25年以上にわたって存続し続けていることが挙げられます。

拡張言語は、他者によるプログラムの変更を可能にするだけでなく、_インテンション_にも適しています。「Emacs流」で構築されたプログラムは、作成者にとって快適で、必要な機能を簡単に追加できます。

Emacsの使い勝手が広く評価されるようになると、多くのハッカーがこの使い心地をGNUシステムの他の部分にも広げる方法を検討し始めた。プログラムをEmacs化する最も簡単な方法は、共通言語の実装をプログラムに組み込むことだと明らかになった。

* * *

次へ: [多数のメンテナーによるスキーム](https://doc.guix.gnu.org/guile/latest/en/guile.html#A-Scheme-of-Many-Maintainers )、前: [Emacs の論文](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Emacs-Thesis)、上: [Guile の簡単な歴史](https://doc.guix.gnu.org/guile/latest/en/guile.html#History) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.1.2 初期段階 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Early-Days-1)

トム・ロードは、組み込み可能な言語ランタイムの開発に全力を注いだ最初の人物であり、それを「GEL」（GNU拡張言語）と名付けた。

GELは、オーブリー・ジャファーによるSchemeの実装であるSCMを、ライブラリとして組み込むのに適した形に変換した結果として生まれた。（SCM自体は、ジョージ・カレットによるSIODの実装に基づいていた。）

ロードはリチャード・ストールマンを説得し、GELをGNUプロジェクトの公式拡張言語にすることに成功した。SchemeはEmacs Lispよりも洗練された、より現代的なLispであったため、これは自然な流れだった。また、GELの機能が向上しれば、他の言語、特にEmacs Lispを実行できるようになるという見込みも、説得の根拠の一つだった。

他のプログラミング言語との命名上の衝突を避けるため、リー・トーマスはGELに「Guile」という新しい名前を提案した。「Guile」は再帰的な頭字語であるだけでなく、その祖先である「Planner」「Conniver」「Schemer」の命名規則を巧みに踏襲している。（後者は、古いオペレーティングシステムのファイル名の6文字制限のため、「Scheme」に短縮された。）さらに、「Guile」は「guy-ell」、つまり「Guy L. Steele」を連想させる。彼はジェラルド・サスマンと共に、Schemeを最初に発見した人物である。

Guile（当時はGEL）が一般公開に向けて準備を進めていた頃、別の拡張言語であるTclの人気が高まっていました。多くの開発者は、Tclのシェルライクな構文と、高度に開発されたグラフィカルウィジェットライブラリであるTkに利点を見出しました。また、当時はTclを「汎用拡張言語」として宣伝する大規模なマーケティング活動も行われていました。

GNU Emacsの主要開発者であるリチャード・ストールマンは、拡張言語のあるべき姿について独自のビジョンを持っており、TclはEmacs Lispほど有能ではないと考えていた。彼はcomp.lang.tclニュースグループに批判的な投稿をし、インターネット史上屈指の論争を引き起こした。後に「Tcl戦争」と呼ばれることになるこの議論の中で、彼はフリーソフトウェア財団がGNUプロジェクトの拡張言語としてGuileを推進する意向であることを発表した。

GuileはTclへの反発として開発されたという誤解がよくありますが、これは間違いです。確かにGuileの発表は「Tcl戦争」と同時期に行われましたが、Guileはそうした論争とは無関係の状況から生まれたものです。実際、既存アプリケーションの拡張と、より完全な動的プログラミング環境との間のギャップを埋める強力な言語の必要性は、今日でも依然として存在しています。

* * *

次へ: [Guile の厳選リリースのタイムライン](https://doc.guix.gnu.org/guile/latest/en/guile.html#A-Timeline-of-Selected-Guile-Releases)、前へ: [初期の頃](https://doc.guix.gnu.org/guile/latest/en/guile.html#Early-Days)、上へ: [Guile の簡単な歴史](https://doc.guix.gnu.org/guile/latest/en/guile.html#History) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.1.3 多数のメンテナーによるスキーム [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#A-Scheme-of-Many-Maintainers-1)

現状を概観すると、Schemeの実装とメンテナーはN対1の関係にあるように思われる。つまり、Schemeを実装する人は複数回実装する可能性があるが、特定のSchemeの存続期間は一人のメンテナーの活動期間に左右されるということである。

この点において、ガイルは異例の存在だ。

Tom Lordは、Guileの最初の1年半ほど、つまり1994年末から1996年半ばまでメンテナンスを担当しました。この期間にリリースされたバージョンは、スタンドアロンプログラムとしてのSCMから、再利用可能で組み込み可能なライブラリとしてのGuileへと進化する過程を示していますが、その過程でTclとTkの組み込み、Javaのコンパイルと逆アセンブルのためのツールチェーン、C言語のような構文の追加、モジュールシステムの作成、そして豊富なPOSIXインターフェースの開発開始など、機能の爆発的な増加が見られました。

Guileには、そうした機能の一部しか残っていません。小型で組み込み可能な言語を提供することと、現代のEmacsが必要とするであろうすべての機能（例えばグラフィカルツールキット）を備えた言語を提供することの間で、常に葛藤がありました。最終的に、Guileの普及が進むにつれて、開発チームは機能の幅広さよりも、深み、ドキュメント、そして直交性に重点を置くことを決定しました。Guileには幅広いサードパーティライブラリが存在するものの、これはそれ以来Guileの中心的な焦点となっています。

ジム・ブランディは、1999年末までの3年間、この安定化期間を統括し、その後、彼自身も他のプロジェクトに移りました。それ以来、Guileはグループによるメンテナンス体制となっています。最初のグループは、マチェイ・スタホヴィアク、ミカエル・ジュルフェルト、マリウス・フォルマーで構成され、フォルマーが最も長く在籍しました。2007年末までに、マリウスはほとんど他のプロジェクトに移ったため、ニール・ジェラムとルドヴィック・クルテスが主要なメンテナンス責任を引き継ぎました。2009年末にアンディ・ウィンゴがニールとルドヴィックに加わり、ニールは退任し、その後すぐにマーク・ウィーバーが加わりました。マークも5年以上その役割を務めた後、退任し、2020年1月現在、ルドヴィックとアンディがGuileの共同メンテナンス担当者となっています。

もちろん、Guileの開発作業の大部分は、ここではすべてを挙げきれないほど多くの貢献者によるものであり、彼らがいなければ世界はもっと貧しい場所になっていただろう。

* * *

次へ: [ステータス、または: ご協力のお願い](https://doc.guix.gnu.org/guile/latest/en/guile.html#Status)、前: [多数のメンテナーによるスキーム](https://doc.guix.gnu.org/guile/latest/en/guile.html#A-Scheme-of-Many-Maintainers)、上: [Guile の簡単な歴史](https://doc.guix.gnu.org/guile/latest/en/guile.html#History) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.1.4 Guile の主要リリースのタイムライン [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#A-Timeline-of-Selected-Guile-Releases-1)

guile-i — 1995年2月4日

SCMは図書館へと変貌を遂げた。

guile-ii — 1995年4月6日

低レベルモジュールシステムが追加されました。Tcl/Tkのサポートが追加され、SchemeをTclで拡張したり、その逆も可能になりました。POSIXサポートが改善され、Javaとの統合も試験的に試みられました。

guile-iii — 1995年8月18日

C言語に似た構文であるctaxが改良されたが、今回のリリースでは主にGuileを細分化する作業に着手したことが主な特徴だった。

1.0 — 1997年1月5日

`#f` は `'()` と区別されるようになりました。ユーザーレベルの協調型マルチスレッド処理が追加されました。ソースレベルのデバッグがより便利になり、プログラマーマニュアルとユーザーマニュアルの作成が始まりました。モジュールシステムには高レベルのインターフェースが採用され、それは現在でもほぼ同じ形で使用されています。

1.1 — 1997年5月16日

1.2 — 1997年6月24日

Tcl/Tkとctaxのサポートは別々のパッケージとして分離され、現在もその状態が維持されています。GuileはSCSHとの互換性が向上し、UNIXスクリプト言語としての利便性も高まりました。Libguileは共有ライブラリとしてビルドできるようになり、C言語で書かれたサードパーティ製の拡張機能も動的リンクによってロード可能になりました。

1.3.0 — 1998年10月19日

readlineライブラリの登場により、コマンドライン編集が格段に快適になった。マルチバイト文字列による国際化の初期サポートは削除され、本格的な国際化が再び実現するまでには10年の歳月を要した。Emacs Lispの初期サポートが実装され、ポートはファイルディスクリプタのサポートが向上し、Fluidsが追加された。

1.3.2 — 1999年8月20日

1.3.4 — 1999年9月25日

1.4 — 2000年6月21日

Lisp特有の機能が多数追加されました。フック、Common Lispの`format`、オプション引数とキーワード引数、`getopt-long`、ソート、乱数生成、その他多くの修正と機能強化が含まれています。Guileには、対話型デバッガ、対話型ヘルプ、より優れたバックトレース機能も追加されました。

1.6 — 2002年9月6日

GuileはR5RS規格に対応し、多数のSRFIモジュールを追加しました。モジュールシステムは、識別子の選択と名前変更のためのプログラムによるサポートによって拡張されました。GOOPSオブジェクトシステムはGuileコアに統合されました。

1.8 — 2006年2月20日

Guileの任意精度演算はGMPライブラリを使用するように変更され、正確な有理数のサポートが追加されました。Guileに組み込まれていたユーザー空間スレッドは削除され、POSIXプリエンプティブスレッドが採用され、真のマルチプロセッシングが実現しました。Gettextのサポートが追加され、GuileのC APIは大幅に整理され、直交化されました。

2.0 — 2010年2月16日

Guile に仮想マシンが追加され、関連するコンパイラとツールチェーンも追加されました。国際化のサポートが、Unicode、ロケール、 libunistring の面でついに再実装されました。実行中の Guile インスタンスは、Geiser を介して Emacs 内から制御およびデバッグできるようになりました。Guile は、他の多くの Schemes に見られる機能に追いつきました。SRFI-18 スレッド、モジュール衛生的なマクロ、プロファイラ、トレーサ、デバッガ、SSAX XML 統合、バイトベクトル、動的 FFI、区切り継続、モジュール バージョン、および R6RS の部分的なサポートです。

2.2 — 2017年3月15日

2.0で導入された仮想マシンは、コンパイラやツールチェーンの大部分とともに完全に書き直されました。これにより、多くのGuileプログラムの実行速度が向上し、起動時間とメモリ使用量も削減されました。GuileのPOSIXマルチスレッド機能が改善され、スタックは動的に拡張可能になり、ポート機能はノンブロッキングI/Oをサポートするようになりました。

3.0 – 2020年1月

Guileは、シンプルなジャストインタイム（JIT）コンパイラによるネイティブコード生成のサポートを獲得し、仮想マシンの速度をさらに向上させた。コンパイラ自体にも、トップレベルバインディングのインライン化、クロージャの最適化の改善、整数値と浮動小数点値のアンボックス化の改善など、多くの新しい最適化が加えられた。R7RSのサポートが追加され、R6RSのサポートも改善された。例外処理機能（throwとcatch）は、SRFI-34例外ハンドラに基づいて書き直された。

* * *

前へ: [Guile の厳選リリースのタイムライン](https://doc.guix.gnu.org/guile/latest/en/guile.html#A-Timeline-of-Selected-Guile-Releases)、上へ: [Guile の簡単な歴史](https://doc.guix.gnu.org/guile/latest/en/guile.html#History) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.1.5 ステータス、または: ヘルプが必要です [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Status_002c-or_003a-Your-Help-Needed)

Guileは当初の目標の多くを達成したが、まだやるべきことはたくさんある。

既存のアプリケーションをよりEmacsライクな操作感にするという、依然として根強い課題が残っている。Guileはこの点で一定の成果を上げているものの、GNUシステム内のほとんどのアプリケーションは依然としてGuileとの統合に対応していない。

Guileをこれらのアプリケーションに導入するには、投資、つまりGuileをプログラムに組み込むために必要な「ハッキングへの情熱」が必要であり、新しい種類の動作を可能にするのに十分なレベルに達したときに初めて報われます。これは、新しいハッカーが貢献する絶好の機会となるでしょう。自分がよく使い慣れているアプリケーションを選び、まだできないことを考え、Guileを統合してそのタスクをGuileで実装する方法を考えてみてください。

時が経つにつれ、この状況は逆転し、プログラムがGuile上で動作するようになり、最終的にはGNUシステム全体がEmacs化される可能性もある。実際、多くのGuileモジュールが`ice-9`名前空間に存在するのは、カート・ヴォネガットの小説『猫のゆりかご』に登場する架空の物質、ソフトウェアの塊を結晶化させる種結晶として機能する「ice-9」にちなんだ名前である。

この議論全体には、動的言語はC言語のような言語よりも優れているという考え方が暗黙のうちに含まれている。C言語のような言語にもそれなりの役割はあるが、Guileの見解は、SchemeはC言語よりも表現力に優れ、書くのも楽しいというものだ。この認識は、他の言語ではなく、できる限りSchemeでコードを書くべきだという強い意志を伴っている。

近年では、バイトコードとネイティブコンパイル、基盤となるハードウェアの高速化、高水準言語の外部呼び出しインターフェースなどにより、拡張可能なアプリケーションをほぼ完全に高水準言語から記述することが可能になっています。Smalltalk システムや Common Lisp ベースのシステムはその一例です。すでに多くの純粋な Guile アプリケーションが存在しますが、過去には、Guile の事前構築済みインターフェースを持たないシステムライブラリとのインターフェースや、高いパフォーマンスを必要とするタスクなど、一部のタスクでは C 言語を使用する必要がありました。Guile 3.0 で JIT コンパイラによるネイティブコード生成が導入されたことで、これらの古いアプリケーションのほとんどは、より多くの C コードを Scheme に移行できるように更新できるようになりました。

とはいえ、Guileのみで構成されたアプリケーションであっても、C言語やPythonに近い構文を持つ言語からユーザーがプログラムを拡張できる機会を提供したい場合もあるでしょう。例えば、PythonをGuileにコンパイルするというアイデアも興味深いものです。これはそれほど突飛なアイデアではありません。IronPythonやJRubyなどを参考にしてみてください。

また、Emacs自体にも注目すべき点があります。GuileのEmacs Lispサポートは、正確性、堅牢性、速度において優れたレベルに達しています。しかし、Emacs本体への統合を完了するには、まだやるべきことが残っています。統合が完了すれば、ネイティブスレッド、本格的なオブジェクトシステム、より洗練された型、より簡潔な構文、そしてGuileの拡張機能すべてへのアクセスなど、Emacsに多くの魅力的な機能がもたらされるでしょう。

最後に、世界の計算処理の大部分がウェブブラウザで行われている現状を踏まえると、ウェブクライアントにおけるGuileの可能性について改めて考えてみる価値は十分にあるでしょう。WebAssemblyの登場により、ほぼすべてのユーザー向けデバイスで利用可能な、妥当なコンパイルターゲットがようやく実現するかもしれません。特に、末尾呼び出し、区切り継続、GC管理オブジェクトを可能にするための今後の提案を考慮すると、Schemeは再びウェブブラウザで活躍の場を得る可能性を秘めています。さあ、始めましょう！

* * *

次へ: [Guile 用仮想マシン](https://doc.guix.gnu.org/guile/latest/en/guile.html#A-Virtual-Machine-for-Guile)、前: [Guile の簡単な歴史](https://doc.guix.gnu.org/guile/latest/en/guile.html#History)、上: [Guile の実装](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guile-Implementation) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
