### 7.20 Statprof [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Statprof-1)

StatprofはGuile用の統計プロファイラーです。

statprofの簡単な使用例は次のようになります。

(use-modules (statprof))
(statprof (lambda ()
(マップ 1+ (イオタ 1000000))
#f))

これにより、統計プロファイリングを含むサンクが実行され、最終的に次のような統計情報のフラットな表が表示されます。

累積自己比率
時間 秒 秒 手順
57.14 39769.73 0.07 ice-9/boot-9.scm:249:5:map1
28.57 0.04 0.04 ice-9/boot-9.scm:1165:0:iota
14.29 0.02 0.02 1+
0.00 0.12 0.00 <現在の入力>:2:10
---
サンプル数：7
合計時間: 0.123490713秒 (GC時間: 0.201983993秒)

呼び出し回数列を除くすべての数値データは、統計的に近似値です。以下の列の説明、およびstatprof全体において、「時間」とは実行時間（ユーザー時間とシステム時間の両方）を指し、実時間ではありません。

「% time」列は、実行時間のうち、プロシージャ自体（子プロセスを除く）で費やされた時間の割合を示します。これは、「self seconds」（プロシージャ内で費やされた時間）を総実行時間で割った値として計算されます。

`cumulative seconds` は、関数の子要素内で費やされた時間もカウントします。再帰関数の場合、スタック上の各要素の起動が累積時間に加算されるため、上記の例のように、この値は合計時間を超える可能性があります。

最後に、GC時間はガベージコレクタで費やされた時間を計測します。マルチコアシステムでは、すべてのスレッドで費やされた時間がカウントされ、GCの「マーキング」フェーズが並列実行されるため、この時間は実行時間よりも長くなる可能性があります。GC時間が実行時間の大部分を占める場合、プログラムのほとんどの時間がオブジェクトの割り当てと、割り当て後のクリーンアップに費やされていることを意味します。プログラムの速度を向上させるには、まず割り当て率を下げる方法を検討するのが良いでしょう。

Statprofの主な動作モードは統計プロファイラとしてです。ただし、Statprofは「精密」モードでも実行できます。すべての呼び出しを記録するには、`#:count-calls? #t`というキーワード引数を`statprof`に渡します。

(use-modules (statprof))
(statprof (lambda ()
(マップ 1+ (イオタ 1000000))
#f)
#:count-calls? #t)

結果には「calls」という列が追加されます。

累積自己比率
時間 秒 秒 呼び出し手順
82.26 0.73 0.73 1000000 1+
11.29 420925.80 0.10 1000001 ice-9/boot-9.scm:249:5:map1
4.84 0.06 0.04 1 ice-9/boot-9.scm:1165:0:iota
[...]
---
サンプル数：62
合計時間: 0.893098065秒 (GC時間: 1.222796536秒)

ご覧のとおり、プロファイルが乱れています。以前のプロファイルではホットとしてマークされていなかった「1+」が最上位に表示されています。これは、呼び出し回数のオーバーヘッドが呼び出しに不当なペナルティを与えるためです。とはいえ、この正確なモードは、正確な呼び出し回数に基づいてアルゴリズムの最適化を行う際に役立つ場合があります。

### 実装に関する注記 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Implementation-notes)

プロファイラは、`statprof-reset` の呼び出しで定義した間隔後に Unix プロファイリングシグナル `ITIMER_PROF` がオフになるように設定することで動作します。シグナルがオンになると、サンプリングルーチンが実行され、スタックを遡ってすべての命令ポインタをバッファに記録します。サンプリングが完了すると、プロファイラはプロファイリングタイマーをリセットし、適切な間隔後に再びオンになるようにします。

その後、プロファイリングが停止すると、そのログバッファが分析され、「自己秒数」と「累積秒数」の統計情報が生成されます。スタックの最上位にあるプロシージャは「自己」サンプルとしてカウントされ、スタック上のすべてのプロシージャは「累積」サンプルとしてカウントされます。

プロファイラが実行中は、コードがプロファイラ内で実行されている間に経過したCPU時間（システム時間とユーザー時間。これは`ITIMER_PROF`が追跡する時間と同じです）を測定します。プロファイルにカウントされるのは実行時間のみで、実時間ではありません。たとえば、スリープ状態や入力/出力の待機状態では、タイマーのクロックは進みません。

### 使用方法 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Usage-13)

Scheme Procedure: **statprof** thunk \[#:loop loop=1\] \[#:hz hz=100\] \[#:port port=(current-output-port)\] \[#:count-calls? count-calls?=#f\] \[#:display-style display-style='flat\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-statprof)

サンクの実行をプロファイリングし、その戻り値を返します。

スタックは毎秒 Hz 回サンプリングされ、サンク自体は ループ 回呼び出されます。

count-calls? が true の場合、すべてのプロシージャ呼び出しが記録されます。この操作はややコストがかかります。

サンクのプロファイリングが完了したら、プロファイルをポートに出力します。display-styleが`flat`の場合は、結果はフラットプロファイルとして出力されます。display-styleが`tree`の場合は、結果はツリープロファイルとして出力されます。

`statprof` を使用するには、正常に動作するプロファイリングタイマーが必要です。一部のプラットフォームではプロファイリングタイマーがサポートされていません。`(provided? 'ITIMER_PROF)` を使用して、プロファイリングタイマーのサポート状況を確認できます。

プロファイリングは手動で有効化または無効化することもできます。

スキーム手順: **statprof-active?** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-statprof_002dactive_003f)

`statprof-start` の呼び出し回数が `statprof-stop` の呼び出し回数より多い場合は `#t` を返し、そうでない場合は `#f` を返します。

スキーム手順: **statprof-start** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-statprof_002dstart)

スキーム手順: **statprof-stop** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-statprof_002dstop)

プロファイラを開始または停止します。

スキーム手順: **statprof-reset** sample-seconds sample-microseconds count-calls? [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-statprof_002dreset)

プロファイリングのサンプリング間隔をサンプル秒とサンプルマイクロ秒にリセットします。count-calls? が true の場合、統計的プロファイリングデータの収集に加えて、プロシージャ呼び出しも計測するように設定してください。

`statprof-start`/`statprof-stop` という手動インターフェースを使用する場合、`statprof-reset` の最後の呼び出し、または `statprof-start` の最初の呼び出しから始まる暗黙的な statprof 状態が保持されます。この暗黙的な状態から統計情報を取得するためのアクセサがいくつか用意されています。

スキーム手順: **statprof-accumulated-time** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-statprof_002daccumulated_002dtime)

前回のstatprof実行中に蓄積された時間を返します。

スキーム手順: **statprof-sample-count** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-statprof_002dsample_002dcount)

前回のstatprof実行時に取得されたサンプル数を返します。

Scheme Procedure: **statprof-fold-call-data** proc init [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- statprof_002dfold_002dcall_002ddata)

statprofによって蓄積された呼び出しデータに基づいて、このプロシージャを折り畳みます。statprofがアクティブな間は、このプロシージャを呼び出すことはできません。

procは引数、呼び出しデータ、および以前の結果とともに呼び出されます。

Scheme Procedure: **statprof-proc-call-data** proc [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-statprof_002dproc_002dcall_002ddata)

procに関連付けられた呼び出しデータを返します。呼び出しデータがない場合は`#f`を返します。

スキーム手順: **statprof-call-data-name** cd [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-statprof_002dcall_002ddata_002dname)

スキーム手順: **statprof-call-data-calls** cd [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-statprof_002dcall_002ddata_002dcalls)

スキーム手順: **statprof-call-data-cum-samples** cd [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-statprof_002dcall_002ddata_002dcum_002dsamples)

スキーム手順: **statprof-call-data-self-samples** cd [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-statprof_002dcall_002ddata_002dself_002dsamples)

statprof の呼び出しデータオブジェクト内のフィールドへのアクセサー。

スキーム手順: **statprof-call-data->stats** call-data [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-statprof_002dcall_002ddata_002d_003estats)

`statprof-stats`型のオブジェクトを返します。

Scheme Procedure: **statprof-stats-proc-name** stats [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-statprof_002dstats_002dproc_002dname)

Scheme Procedure: **statprof-stats-%-time-in-proc** stats [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-statprof_002dstats_002d_0025_002dtime_002din_002dproc)

スキーム手順: **statprof-stats-cum-secs-in-proc** 統計 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-statprof_002dstats_002dcum_002dsecs_002din_002dproc)

スキーム手順: **statprof-stats-self-secs-in-proc** stats [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-statprof_002dstats_002dself_002dsecs_002din_002dproc)

Scheme Procedure: **statprof-stats-calls** stats [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-statprof_002dstats_002dcalls)

スキーム手順: **statprof-stats-self-secs-per-call** 統計 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-statprof_002dstats_002dself_002dsecs_002dper_002dcall)

スキーム手順: **statprof-stats-cum-secs-per-call** 統計 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-statprof_002dstats_002dcum_002dsecs_002dper_002dcall)

`statprof-stats`オブジェクトのフィールドへのアクセサー。

Scheme Procedure: **statprof-display** \[port=(current-output-port)\] \[#:style style=flat\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-statprof_002ddisplay)

収集した統計情報の概要を表示します。スタイルに指定できる値は次のとおりです。

`flat`

従来のgprofスタイルの平面プロファイルを表示します。

異常

データ中の統計的な異常値を見つける。

`tree`

樹木のプロファイルを表示します。

Scheme Procedure: **statprof-fetch-stacks** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-statprof_002dfetch_002dstacks)

`statprof-reset` の最後の呼び出し以降に取得されたスタックのリストを返します。

Scheme Procedure: **statprof-fetch-call-tree** \[#:precise precise?=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-statprof_002dfetch_002dcall_002dtree)

前回のstatprof実行時のコールツリーを返します。

戻り値はノードのリストです。ノードは次の形式のリストです。
@コード
ノード ::= (@var{proc} @var{count} . @var{nodes})
@end code

@var{proc} は、プロシージャの印刷可能な表現です。
文字列。@var{precise?} が false の場合（これがデフォルト）、ノード
手続き呼び出しに対応します。これが真であれば、ノード
プロシージャの戻り点に対応します。@code{#:precise? を渡していますか？
#t} を使用すると、ユーザーはプロシージャ内の異なるソース行を区別できます。
しかし、通常は詳細すぎるため、デフォルトではオフになっています。

Scheme Procedure: **gcprof** thunk \[#:loop\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-gcprof)

`statprof` 手順と同様ですが、CPU 時間をプロファイリングする代わりに、ガベージコレクションをプロファイリングします。

thunk の評価中、ガベージコレクションの直後にスタックがサンプリングされ、プログラム内でメモリ割り当ての原因となっているものについておおよその見当がつきます。

GC は頻繁には発生しないため、thunk がループ回数だけ呼び出されるようにするには、loop パラメータを使用する必要があるかもしれません。

* * *

次へ: [Texinfo Processing](https://doc.guix.gnu.org/guile/latest/en/guile.html#Texinfo-Processing)、前: [Statprof](https://doc.guix.gnu.org/guile/latest/en/guile.html#Statprof)、上: [Guile Modules](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guile-Modules) \[[Contents](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[Index](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "Index")\]
