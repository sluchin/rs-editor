#### 7.5.15 SRFI-18 - マルチスレッドのサポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d18-_002d-Multithreading-support)

これは、SRFI-18 スレッドおよび同期ライブラリの実装です。ここで説明する関数と変数は、以下のライブラリによって提供されています。

(use-modules (srfi srfi-18))

SRFI-18は、スレッド、ミューテックス、条件変数、時間、例外処理のための機能を定義します。これらの機能はGuileのプリミティブよりも高レベルであるため、Guileが提供する機能の上にレイヤーとして実装されます。具体的には、GuileのミューテックスはSRFI-18のミューテックスではなく、GuileのスレッドはSRFI-18のスレッドではない、といった具合です。Guileは一連のプリミティブを提供し、SRFI-18はそのプリミティブに基づいて構築されたシステムの1つです。

* [SRFI-18 スレッド](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d18-Threads)
* [SRFI-18 ミューテックス](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d18-Mutexes)
* [SRFI-18 条件変数](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d18-Condition-variables)
* [SRFI-18 時刻](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d18-Time)
* [SRFI-18 例外](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d18-Exceptions)

* * *

次へ: [SRFI-18 ミューテックス](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d18-Mutexes)、上へ: [SRFI-18 - マルチスレッドのサポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d18) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.5.15.1 SRFI-18 スレッド [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d18-Threads-1)

SRFI-18 で作成されたスレッドは、Guile の組み込みスレッド関数で作成されたスレッドとは 2 つの点で異なります。まず、SRFI-18 の `make-thread` で作成されたスレッドはブロック状態から開始され、`thread-start!` が呼び出されるまで実行が開始されません。次に、SRFI-18 のスレッドは、スレッド終了時に発生する例外を捕捉するトップレベルの例外ハンドラを備えて構築されます。

SRFI-18 スレッドは、Guile の基本的なスレッドとは別個のものです。Guile の基本的な機能の詳細については、[スレッド](https://doc.guix.gnu.org/guile/latest/en/guile.html#Threads) を参照してください。

機能: **current-thread** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-current_002dthread-1)

この関数を呼び出したスレッドを返します。これは、同名の組み込みプロシージャ`current-thread`と同じプロシージャです（[スレッド](https://doc.guix.gnu.org/guile/latest/en/guile.html#Threads)を参照）。

機能: **thread?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-thread_003f-1)

obj がスレッドの場合は `#t` を返し、それ以外の場合は `#f` を返します。これは、同名の組み込みプロシージャ `thread?` と同じプロシージャです ([Threads](https://doc.guix.gnu.org/guile/latest/en/guile.html#Threads) を参照)。

関数: **make-thread** thunk \[name\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dthread-1)

新しいスレッドで新しい動的状態を使用して `thunk` を呼び出し、新しいスレッドを返し、オプションでオブジェクト名 name を割り当てます。name は任意の Scheme オブジェクトです。

`make-thread`という名前は、`(ice-9 threads)`関数の`make-thread`と競合することに注意してください。これらの関数を両方使用したいアプリケーションは、それぞれ異なる名前で参照する必要があります。

機能: **thread-name** スレッド [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-thread_002dname)

スレッド作成時に割り当てられた名前を返します。名前が割り当てられていない場合は `#f` を返します。

機能: **スレッド固有** スレッド [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-thread_002dspecific)

機能: **スレッド固有のセット!** スレッドオブジェクト [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-thread_002dspecific_002dset_0021)

スレッドの「オブジェクト固有」プロパティを取得または設定します。GuileのSRFI-18実装では、この値はオブジェクトプロパティとして格納され、設定されていない場合は`#f`になります。

機能: **thread-start!** スレッド [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-thread_002dstart_0021)

スレッドのブロックを解除し、まだ実行が開始されていない場合は実行を開始できるようにします。

機能: **thread-yield!** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-thread_002dyield_0021)

1つ以上のスレッドが実行待ち状態にある場合、`thread-yield!`を呼び出すと、それらのスレッドのいずれかに即座にコンテキストスイッチが実行されます。それ以外の場合は、`thread-yield!`は効果がありません。`thread-yield!`は、Guileの組み込み関数`yield`と全く同じように動作します。

機能: **thread-sleep!** タイムアウト [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-thread_002dsleep_0021)

現在のスレッドは、時間オブジェクトのタイムアウトで指定された時点に達するまで待機します（[SRFI-18 Time](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d18-Time)を参照）。これは、タイムアウトが未来の時点を表す場合にのみスレッドをブロックします。タイムアウトが`#f`の場合はエラーです。

機能: **thread-terminate!** スレッド [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-thread_002dterminate_0021)

スレッドの異常終了を引き起こします。スレッドがまだ終了していない場合、スレッドが所有するすべてのミューテックスはロック解除/放棄されます。スレッドが現在のスレッドである場合、`thread-terminate!` は戻りません。それ以外の場合、`thread-terminate!` は未指定の値を返します。スレッドの終了は、`thread-terminate!` が戻る前に発生します。このスレッドで後続の参加を試みると、「終了したスレッド例外」が発生します。

`thread-terminate!` は、コア スレッド API のスレッドキャンセル手順 ([Threads](https://doc.guix.gnu.org/guile/latest/en/guile.html#Threads) を参照) と互換性があり、対象のスレッドにクリーンアップ ハンドラがインストールされている場合は、スレッドが終了する前にそれが呼び出され、その戻り値 (または例外がある場合は例外) が `thread-join!` の呼び出しによって後で取得できるように保存されます。

機能: **thread-join!** スレッド \[timeout \[timeout-val\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-thread_002djoin_0021)

スレッドが終了するまで待機し、終了値を返します。タイムアウト値が指定されている場合、待機を中止する時点を指定します。待機が中止されると、timeout-val が指定されていればそれが返されます。指定されていない場合は、`join-timeout-exception` 例外が発生します ([SRFI-18 例外](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d18-Exceptions) を参照)。スレッドが `thread-terminate!` の呼び出しによって終了した場合 (`terminated-thread-exception` が発生します)、またはスレッドがトップレベルの例外ハンドラによって処理された例外を発生させて終了した場合 (`uncaught-exception` が発生します。元の例外は `uncaught-exception-reason` を使用して取得できます) にも例外が発生する可能性があります。

* * *

次へ: [SRFI-18 条件変数](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d18-Condition-variables)、前: [SRFI-18 スレッド](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d18-Threads)、上: [SRFI-18 - マルチスレッドのサポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d18) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引"）\]

#### 7.5.15.2 SRFI-18 ミューテックス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d18-Mutexes-1)

SRFI-18 ミューテックスは、Guile のプリミティブ ミューテックスとは別個のものです。Guile のプリミティブ機能の詳細については、[ミューテックスと条件変数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Mutexes-and-Condition-Variables) を参照してください。

関数: **make-mutex** \[name\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dmutex-1)

新しいミューテックスを返します。オプションで、任意のSchemeオブジェクトであるオブジェクト名nameを割り当てることができます。返されるミューテックスは、上記の設定で作成されます。

機能: **mutex-name** mutex [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-mutex_002dname)

ミューテックスの作成時に割り当てられた名前を返します。名前が割り当てられていない場合は `#f` を返します。

機能: **mutex-specific** mutex [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-mutex_002dspecific)

mutex の「オブジェクト固有」プロパティを返します。プロパティが設定されていない場合は `#f` を返します。

機能: **mutex-specific-set!** mutex obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-mutex_002dspecific_002dset_0021)

ミューテックスの「オブジェクト固有」プロパティを設定します。

機能: **mutex-state** mutex [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-mutex_002dstate)

ミューテックスの状態に関する情報を返します。指定可能な値は次のとおりです。

* スレッド t: ミューテックスはロック/所有状態であり、スレッド t がミューテックスの所有者です
* シンボル `not-owned`: ミューテックスはロック/非所有状態です
* シンボル `abandoned`: ミューテックスはロック解除/放棄状態です
* シンボル `not-abandoned`: ミューテックスはロック解除/非放棄状態です

機能: **mutex-lock!** mutex \[timeout \[thread\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-mutex_002dlock_0021)

ミューテックスをロックします。オプションで、ロック試行を中止する時間オブジェクトのタイムアウトと、現在のスレッドとは異なる新しいミューテックスの所有者を与えるスレッドを指定できます。

機能: **mutex-unlock!** mutex \[condition-variable \[timeout\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-mutex_002dunlock_0021)

ミューテックスのロックを解除します。必要に応じて、シグナルを受け取るまで待機する条件変数 condition-variable を指定します。待機時間は無期限、または必要に応じて、時間オブジェクトのタイムアウトが経過するまでとします。

* * *

次へ: [SRFI-18 時間](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d18-Time)、前: [SRFI-18 ミューテックス](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d18-Mutexes)、上: [SRFI-18 - マルチスレッドのサポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d18) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.5.15.3 SRFI-18 条件変数 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d18-Condition-variables-1)

SRFI-18では、条件変数に対する「待機」関数は規定されていません。条件変数の待機は、前のセクションで説明したSRFI-18の`mutex-unlock!`関数を使用してシミュレートできます。

SRFI-18 条件変数は、Guile の基本条件変数とは互いに排他的です。Guile の基本機能の詳細については、[ミューテックスと条件変数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Mutexes-and-Condition-Variables) を参照してください。

機能: **条件変数?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-condition_002dvariable_003f-1)

objが条件変数の場合は`#t`を返し、それ以外の場合は`#f`を返します。

機能: **make-condition-variable** \[name\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dcondition_002dvariable-1)

新しい条件変数を返します。オプションで、任意の Scheme オブジェクトであるオブジェクト名 name を割り当てます。

機能: **条件変数名** 条件変数 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-condition_002dvariable_002dname)

条件変数が作成された際に割り当てられた名前を返します。名前が割り当てられていない場合は「#f」を返します。

機能: **条件変数固有** 条件変数 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-condition_002dvariable_002dspecific)

条件変数の「オブジェクト固有」プロパティを返します。設定されていない場合は `#f` を返します。

機能: **条件変数固有のセット!** 条件変数オブジェクト [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-condition_002dvariable_002dspecific_002dset_0021)

条件変数の「オブジェクト固有」プロパティを設定します。

機能: **condition-variable-signal!** condition-variable [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-condition_002dvariable_002dsignal_0021)

機能: **condition-variable-broadcast!** condition-variable [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-condition_002dvariable_002dbroadcast_0021)

`condition-variable-signal!` の場合は条件変数を待機しているスレッドを 1 つ起動し、`condition-variable-broadcast!` の場合はそれを待機しているすべてのスレッドを起動します。

* * *

次へ: [SRFI-18 例外](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d18-Exceptions)、前: [SRFI-18 条件変数](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d18-Condition-variables)、上: [SRFI-18 - マルチスレッドのサポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d18) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引"）\]

#### 7.5.15.4 SRFI-18 時刻 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d18-Time-1)

SRFI-18の時刻関数は、時刻を2つの形式で操作します。1つは、実装固有の方法で絶対的な時点を表す「時刻オブジェクト」型、もう1つは、指定されていない「エポック」からの秒数です。Guileの実装では、エポックはUnixエポック、つまり1970年1月1日00:00:00 UTCです。

関数: **current-time** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-current_002dtime-1)

現在時刻を時間オブジェクトとして返します。このプロシージャは、コアライブラリにある同名のプロシージャを置き換えるもので、コアライブラリのプロシージャはエポックからの秒数で現在時刻を返します。

関数: **time?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-time_003f)

objが時間オブジェクトの場合は`#t`を返し、それ以外の場合は`#f`を返します。

関数: **time->seconds** time [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-time_002d_003eseconds)

関数: **seconds->time** 秒 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-seconds_002d_003etime)

時間オブジェクトと、エポックからの秒数を表す数値との間で変換を行います。時間オブジェクトから秒数に変換する場合、戻り値は時間とエポック間の秒数になります。秒数から時間オブジェクトに変換する場合、戻り値はエポックから秒後の時間を表す時間オブジェクトになります。

* * *

前へ: [SRFI-18 時間](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d18-Time)、上へ: [SRFI-18 - マルチスレッドのサポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d18) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.5.15.5 SRFI-18 例外 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d18-Exceptions-1)

SRFI-18 の例外は、Guile の SRFI-34 実装で提供される例外と同一です。ただし、SRFI-18 関数からスローされた例外を処理するために呼び出される例外ハンドラの動作は、ハンドラの継続が関数呼び出しの継続と同じであるという点で、従来の SRFI-34 の動作とは異なります。ハンドラは末尾再帰的に呼び出され、例外は「バブルアップ」しません。

機能: **current-exception-handler** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-current_002dexception_002dhandler)

現在の例外ハンドラを返します。

機能: **with-exception-handler** ハンドラーサンク [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-with_002dexception_002dhandler-1)

ハンドラを現在の例外ハンドラとしてインストールし、引数なしでプロシージャサンクを呼び出し、その値を例外の値として返します。ハンドラは、単一の引数を受け入れるプロシージャである必要があります。このプロシージャが呼び出された時点での現在の例外ハンドラは、呼び出しが戻った後に復元されます。

関数: **raise** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-raise-1)

objを例外として発生させます。これは、SRFI 34で定義されている同名のプロシージャと同じです。

機能: **join-timeout-exception?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-join_002dtimeout_002dexception_003f)

指定されたタイムアウト時間内に終了しないスレッドで時間付き結合を実行した結果として発生した例外が obj である場合は `#t` を返し、それ以外の場合は `#f` を返します。

機能: **abandoned-mutex-exception?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-abandoned_002dmutex_002dexception_003f)

、所有スレッドによって放棄されたミューテックスをロックしようとした結果として発生した例外である場合は `#t` を返し、そうでない場合は `#f` を返します。

関数: **terminated-thread-exception?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-terminated_002dthread_002dexception_003f)

obj が `thread-terminate!` の呼び出しによって終了したスレッドへの参加の結果として発生した例外である場合、`#t` を返します。

関数: **uncaught-exception?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-uncaught_002dexception_003f)

機能: **uncaught-exception-reason** exc [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-uncaught_002dexception_002dreason)

`uncaught-exception?` は、obj が `make-thread` によってインストールされたトップレベルの例外ハンドラによって処理された例外を発生させて終了したスレッドに参加した結果としてスローされた例外である場合、`#t` を返します。この場合、元の例外は `thread-join!` によってスローされた例外の一部として保持され、その例外に対して `uncaught-exception-reason` を呼び出すことでアクセスできます。この例外保持メカニズムは `make-thread` の副作用であるため、上記のように終了したが他の方法で作成されたスレッドに参加しても、この `uncaught-exception` エラーは発生しないことに注意してください。

* * *

次へ: [SRFI-23 - エラー報告](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d23)、前: [SRFI-18 - マルチスレッドのサポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d18)、上: [SRFI サポート モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI-Support) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
