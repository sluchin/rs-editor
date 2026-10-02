#### 7.2.8 シグナル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Signals-1)

以下の手順は、シグナルの発生、処理、および待機を行います。

Scheme コードのシグナル ハンドラは非同期処理 ([非同期割り込み](https://doc.guix.gnu.org/guile/latest/en/guile.html#Asyncs) を参照) で実行されるため、安全なタイミングでハンドラのスレッド内で呼び出されます。通常、これは現在実行中のプリミティブ プロシージャがすべて終了した後になります (外部イベントを待機するプリミティブの場合は、かなり時間がかかる場合があります)。

スキーム手順: **kill** pid sig [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-kill)

C言語関数: **scm\_kill** (pid, sig) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fkill)

指定されたプロセスまたはプロセス群に信号を送信します。

pidは、シグナルが送信されるプロセスを指定します。

pidが0より大きい

識別子がpidであるプロセス。

pidが0に等しい

現在のプロセスグループ内のすべてのプロセス。

pidが-1未満

識別子が -pid であるプロセス グループ

pidが-1に等しい

プロセスが特権を持つ場合、一部の特殊なシステムプロセスを除くすべてのプロセスが対象となります。そうでない場合は、現在の有効ユーザーIDを持つすべてのプロセスが対象となります。

sig は、Unix シンボル名に対応する変数を使用して指定する必要があります。例:

変数: **SIGHUP** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SIGHUP)

切断信号。

変数: **SIGINT** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SIGINT)

割り込み信号。

GNU システム上のシグナルの完全なリストは、GNU C ライブラリ リファレンス マニュアルの [標準シグナル](https://doc.guix.gnu.org/libc/latest/en/libc.html#Standard-Signals) に記載されています。

スキーム手順: **raise** sig [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-raise)

C 関数: **scm\_raise** (sig) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fraise)

指定されたシグナルsigを現在のプロセスに送信します。sigは`kill`手順で説明されているとおりです。

Scheme Procedure: **sigaction** signum \[handler \[flags \[thread\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sigaction)

C 関数: **scm\_sigaction** (signum、handler、flags) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsigaction)

C 関数: **scm\_sigaction\_for\_thread** (signum、handler、flags、thread) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsigaction_005ffor_005fthread)

指定されたシグナルのシグナルハンドラをインストールまたは報告します。

`SIGINT`などの変数の値を使用して指定できます。

ハンドラが省略された場合、`sigaction` はペアを返します。CAR は現在のシグナル ハンドラで、`SIG_DFL` (デフォルト アクション) または `SIG_IGN` (無視) の値を持つ整数、あるいはシグナルを処理する Scheme プロシージャ、または Scheme 以外のプロシージャがシグナルを処理する場合は `#f` のいずれかになります。CDR には、ハンドラの現在の `sigaction` フラグが含まれます。

ハンドラが指定されている場合、それはsignumの新しいハンドラとしてインストールされます。ハンドラは、引数を1つ取るSchemeプロシージャ、または`SIG_DFL`（デフォルトアクション）もしくは`SIG_IGN`（無視）の値、または`#f`（`sigaction`が最初に使用される前にインストールされていたシグナルハンドラを復元する）のいずれかです。Schemeプロシージャが指定されている場合、そのプロシージャは指定されたスレッドで実行されます。スレッドが指定されていない場合は、`sigaction`を呼び出したときのスレッドが使用されます。

flags は、以下のいずれかの `logior` ([ビット演算](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bitwise-Operations) を参照) です (システムによって提供されている場合)。何も提供されていない場合は `0` です。

変数: **SA\_NOCLDSTOP** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SA_005fNOCLDSTOP)

デフォルトでは、子プロセスが停止したとき（つまり、`SIGSTOP`シグナルを受け取ったとき）、および子プロセスが終了したときに`SIGCHLD`シグナルが送信されます。`SA_NOCLDSTOP`フラグを指定すると、停止時ではなく終了時のみ`SIGCHLD`シグナルが送信されます。

`SA_NOCLDSTOP`は、`SIGCHLD`以外のシグナルには影響を与えません。

変数: **SA\_RESTART** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SA_005fRESTART)

システムコール中にシグナルが発生した場合は、シグナルを送信してからシステムコールを再開します（そのコールから`EINTR`エラーを返すのではなく）。

Guile はシグナルを非同期的に処理します。シグナルを受信すると、同期シグナルハンドラはシグナルが受信されたという事実を記録し、関連する Guile スレッドに保留中のシグナルがあることを通知するフラグを設定します。Guile スレッドは保留中の割り込みフラグを確認すると、シグナルハンドラの非同期部分（ `sigaction` でアタッチされたハンドラ）を実行するように手配します。

ただし、この戦略には、`SA_RESTART` フラグとの間で予期せぬ相互作用が生じる可能性があります。同期ハンドラはほとんど何も処理を行わず、特に Guile ハンドラを実行しないため、`SA_RESTART` フラグが設定されたシグナルハンドラを使用して、長時間実行されるシステムコールで停止したスレッドを中断することは不可能です。同期ハンドラは保留中の割り込みを記録するだけで、その後システムコールが再開され、Guile は実際にフラグを確認して非同期ハンドラを実行する機会がありません。これは仕様です。

戻り値は、上記で説明した古いハンドラに関する情報を含むペアです。

このインターフェースでは、「シグナルブロッキング」機能へのアクセスは提供されていません。スレッドサポートによってデータ構造への一貫したアクセスという問題が解決される可能性があるため、この機能は必ずしも必要ではないかもしれません。

スキーム手順: **restore-signals** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-restore_002dsignals)

C 関数: **scm\_restore\_signals** () [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005frestore_005fsignals)

`sigaction` が呼び出される前の値に、すべてのシグナルハンドラを戻します。戻り値は未指定です。

スキーム手順: **alarm** i [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-alarm)

C 関数: **scm\_alarm** (i) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005falarm)

指定した秒数（整数）が経過した後に`SIGALRM`シグナルを発生させるタイマーを設定します。デフォルトの動作はプロセスを終了させることであるため、事前に`SIGALRM`用のシグナルハンドラをインストールしておくことをお勧めします。

戻り値は、前回の警報が鳴った場合の残り時間を示します。新しい値は前回の警報に置き換わります。前回の警報が鳴らなかった場合、戻り値はゼロになります。

Scheme Procedure: **pause** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-pause)

C言語関数: **scm\_pause** () [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fpause)

現在のプロセス（スレッド？）を一時停止し、現在のプロセスを終了するか、ハンドラプロシージャを呼び出すシグナルが到着するまで待機します。戻り値は未定義です。

Scheme Procedure: **sleep** 秒 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sleep)

スキーム手順: **usleep** マイクロ秒 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-usleep)

C言語関数: **scm\_sleep** (秒) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsleep)

C言語関数: **scm\_usleep** (マイクロ秒) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fusleep)

指定された期間（秒またはマイクロ秒、いずれも整数）待機します。信号が到着すると待機は停止し、戻り値は残り時間（秒またはマイクロ秒）となります。指定された期間が経過しても信号がない場合は、戻り値はゼロとなります。

ほとんどのシステムでは、プロセススケジューラはマイクロ秒単位の精度ではないため、`usleep` によって実際にスリープされる期間は、システムクロックのティック境界（たとえば 10 ミリ秒）に丸められる可能性があります。

C レベルでの同等の機能については、`scm_std_sleep` および `scm_std_usleep` を参照してください ([Guile モードのブロッキング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Blocking) を参照)。

Scheme手順: **getitimer** which\_timer [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-getitimer)

Scheme Procedure: **setitimer** which\_timer interval\_seconds interval\_microseconds value\_seconds value\_microseconds [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-setitimer)

C 関数: **scm\_getitimer** (which\_timer) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fgetitimer)

C 関数: **scm_setitimer** (which_timer, interval_seconds, interval_microseconds, value_seconds, value_microseconds) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsetitimer)

特定のシステムタイマーに設定されている期間を取得または設定します。

これらのタイマーには2つの設定があります。1つ目の設定である間隔は、現在のタイマーが期限切れになったときにタイマーがリセットされる値です。2つ目の設定はタイマーの現在の値で、次の期限切れが通知されるタイミングを示します。

which_timer は、以下のいずれかの値です。

変数: **ITIMER\_REAL** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ITIMER_005fREAL)

リアルタイムタイマー。経過時間をリアルタイムでカウントダウンします。ゼロになると`SIGALRM`シグナルを発信します。これは上記の`alarm`と同様ですが、より高解像度の期間でカウントダウンします。

変数: **ITIMER\_VIRTUAL** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ITIMER_005fVIRTUAL)

仮想時間タイマー。現在のプロセスが実際にCPUを使用している間、カウントダウンします。ゼロになると`SIGVTALRM`シグナルが発生します。

変数: **ITIMER\_PROF** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ITIMER_005fPROF)

プロセス実行中（`ITIMER_VIRTUAL`と同様）およびプロセスに代わってシステムコールが実行されている間、カウントダウンするプロファイリングタイマーです。ゼロになると`SIGPROF`シグナルが発生します。

このタイマーは、プログラムがどこで時間を費やしているかをプロファイリングすることを目的としています（タイマーが作動した時点でプログラムがどこにいるかを調べることによって）。

`getitimer` は、再起動タイマーの値とその現在の値を、2 つのペアを含むリストとして返します。各ペアは、秒とマイクロ秒単位の時間です。`((interval_secs . interval_usecs) (value_secs . value_usecs))`。

`setitimer` は、タイマーの値を秒単位とマイクロ秒単位（整数値）で同様に設定します。間隔値をゼロにすると、タイマーは一度だけ実行されます。戻り値は、`getitimer` と同じ形式で、タイマーの前回の設定値です。

(setitimer ITIMER\_REAL
5 500000 ;; 5.5秒ごとにSIGALRMシグナルを発生させる
2 0) ;; 2秒以内に最初のSIGALRMを受信

タイマーはマイクロ秒単位でプログラムされているが、実際の精度はそれほど高くないかもしれない。

`ITIMER_PROF` と `ITIMER_VIRTUAL` はすべてのプラットフォームで機能するわけではなく、呼び出し時に常にエラーが発生する可能性があることに注意してください。`(provided? 'ITIMER_PROF)` と `(provided? 'ITIMER_VIRTUAL)` を使用すると、指定されたホストでこれらのイタイマーがサポートされているかどうかをテストできます。`ITIMER_REAL` は、`setitimer` をサポートするすべてのプラットフォームでサポートされています。

* * *

次へ: [パイプ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Pipes)、前: [シグナル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Signals)、上: [POSIX システムコールとネットワーク](https://doc.guix.gnu.org/guile/latest/en/guile.html#POSIX) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
