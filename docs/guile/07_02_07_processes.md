#### 7.2.7 プロセス

スキームプロシージャ: **chdir** str

C 関数: **scm\_chdir** (str)

現在の作業ディレクトリをstrに変更します。strにはファイル名を含む文字列、またはシステムがサポートしている場合はポート番号を指定できます。`(provided? 'chdir-port)`はポートがサポートされているかどうかを示します。戻り値は未指定です。

Scheme手順: **getcwd**

C言語関数: **scm\_getcwd** ()

現在の作業ディレクトリの名前を返します。

スキーム手順: **umask** \[mode\]

C 関数: **scm\_umask** (モード)

mode を省略した場合、現在のファイル作成マスクを表す 10 進数値を返します。それ以外の場合は、ファイル作成マスクは mode に設定され、以前の値が返されます。umask の使用方法の詳細については、『GNU C ライブラリ リファレンス マニュアル』の [ファイル パーミッションの割り当て](https://doc.guix.gnu.org/libc/latest/en/libc.html#Setting-Permissions) を参照してください。

例えば、`(umask #o022)` はマスクを8進数22/10進数18に設定します。

スキーム手順: **chroot** パス

C 関数: **scm\_chroot** (パス)

ルートディレクトリをpathで指定されたディレクトリに変更します。このディレクトリは、パス名が/で始まる場合に使用されます。ルートディレクトリは、現在のプロセスのすべての子プロセスに継承されます。ルートディレクトリを変更できるのはスーパーユーザーのみです。

Scheme手順: **getpid**

C言語関数: **scm\_getpid** ()

現在のプロセスIDを表す整数を返します。

スキーム手順: **getgroups**

C言語関数: **scm\_getgroups** ()

現在の補助グループIDを表す整数のベクトルを返します。

Scheme手順: **getppid**

C 関数: **scm\_getppid** ()

親プロセスのプロセスIDを表す整数を返します。

スキーム手順: **getuid**

C 関数: **scm\_getuid** ()

現在の実際のユーザーIDを表す整数を返します。

スキーム手順: **getgid**

C 関数: **scm\_getgid** ()

現在の実際のグループIDを表す整数を返します。

スキーム手順: **geteuid**

C 関数: **scm\_geteuid** ()

現在の有効なユーザーIDを表す整数を返します。システムが有効なIDをサポートしていない場合は、実際のIDが返されます。`(provided? 'EIDs)` は、システムが有効なIDをサポートしているかどうかを示します。

スキーム手順: **getegid**

C 関数: **scm\_getegid** ()

現在の有効なグループ ID を表す整数を返します。システムが有効な ID をサポートしていない場合は、実際の ID が返されます。`(provided? 'EIDs)` は、システムが有効な ID をサポートしているかどうかを示します。

Scheme手順: **setgroups** vec

C 関数: **scm\_setgroups** (vec)

指定されたベクトルvecに含まれる整数値を、現在の補助グループIDセットに設定します。戻り値は未定義です。

一般的に、プロセスグループIDを設定できるのはスーパーユーザーのみです（GNU Cライブラリリファレンスマニュアルの[グループIDの設定](https://doc.guix.gnu.org/libc/latest/en/libc.html#Setting-Groups)を参照してください）。

スキームプロシージャ: **setuid** id

C 関数: **scm\_setuid** (id)

プロセスに適切な権限が付与されている場合、実効ユーザーIDと有効ユーザーIDの両方を整数値のIDに設定します。戻り値は未指定です。

スキームプロシージャ: **setgid** id

C 関数: **scm\_setgid** (id)

プロセスに適切な権限がある場合、実効グループIDと有効グループIDの両方を整数値のIDに設定します。戻り値は未指定です。

スキームプロシージャ: **seteuid** id

C 関数: **scm\_seteuid** (id)

プロセスに適切な権限がある場合、実効ユーザーIDを整数値のIDに設定します。実効IDがサポートされていない場合は、代わりに実IDが設定されます。`(provided? 'EIDs)` は、システムが実効IDをサポートしているかどうかを示します。戻り値は未定義です。

スキーム手順: **setegid** id

C 関数: **scm\_setegid** (id)

プロセスに適切な権限がある場合、有効グループIDを整数値のIDに設定します。有効IDがサポートされていない場合は、代わりに実IDが設定されます。`(provided? 'EIDs)` は、システムが有効IDをサポートしているかどうかを示します。戻り値は未定義です。

スキームプロシージャ: **getpgrp**

C 関数: **scm\_getpgrp** ()

現在のプロセスグループIDを表す整数を返します。これはPOSIXの定義であり、BSDの定義ではありません。

スキーム手順: **setpgid** pid pgid

C 関数: **scm\_setpgid** (pid, pgid)

プロセス pid をプロセス グループ pgid に移動します。pid または pgid は整数である必要があります。現在のプロセスの ID を示すために、0 を指定することもできます。ジョブ制御をサポートしていないシステムでは失敗します。戻り値は未定義です。

スキーム手順: **setsid**

C 関数: **scm\_setsid** ()

新しいセッションを作成します。現在のプロセスがセッションリーダーとなり、新しいプロセスグループに追加されます。プロセスが制御端末を持っている場合は、その端末から切り離されます。戻り値は、新しいプロセスグループIDを表す整数です。

スキーム手順: **getsid** pid

C 関数: **scm\_getsid** (pid)

プロセスのプロセスID（pid）に対応するセッションIDを返します。（プロセスのセッションIDは、そのセッションリーダーのプロセスグループIDです。）

スキーム手順: **waitpid** pid \[オプション\]

C言語関数: **scm\_waitpid** (pid、オプション)

この手順は、終了した、または（オプションで）停止した子プロセスからステータス情報を収集します。通常、この処理が完了するまで呼び出し元のプロセスを一時停止します。対象となる子プロセスが複数ある場合は、オペレーティングシステムによって1つが選択されます。

pidの値によって動作が決まります。

pidが0より大きい

指定された子プロセスからステータス情報を要求します。

pid が -1 または `WAIT_ANY` に等しい場合

任意の子プロセスのステータス情報を要求します。

pid が 0 または `WAIT_MYPGRP` に等しい

現在のプロセスグループ内の任意の子プロセスのステータス情報を要求します。

pidが-1未満

プロセスグループIDが-pidである子プロセスのステータス情報を要求します。

オプション引数が指定されている場合、それは以下の変数のうち0個以上の値のビットごとのOR演算結果である必要があります。

変数: **WNOHANG** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- WNOHANG)

収集すべき子プロセスがない場合でも、すぐに処理を終了します。

変数: **WUNTRACED**

停止したプロセスと終了したプロセスの両方について、ステータス情報を報告してください。

戻り値は以下の要素を含むペアです。

1. 子プロセスのプロセスID。`WNOHANG`が指定され、プロセスが収集されなかった場合は0。
2. 整数型のステータス値（GNU Cライブラリリファレンスマニュアルの[プロセス完了ステータス](https://doc.guix.gnu.org/libc/latest/en/libc.html#Process-Completion-Status)を参照）。

以下の3つの関数は、`waitpid`によって返される整数型のステータス値をデコードするために使用できます。

スキームプロシージャ: **status:exit-val** ステータス

C 関数: **scm\_status\_exit\_val** (status)

プロセスが `exit` または `_exit` の呼び出しによって正常に終了した場合に設定される終了ステータス値を返します。それ以外の場合は `#f` を返します。

スキーム手順: **status:term-sig** ステータス

C 関数: **scm\_status\_term\_sig** (ステータス)

プロセスを終了させたシグナル番号があれば返します。なければ`#f`を返します。

スキーム手順: **status:stop-sig** ステータス

C 関数: **scm\_status\_stop\_sig** (ステータス)

プロセスを停止させたシグナル番号があれば返します。なければ`#f`を返します。

Scheme手順: **system** \[cmd\]

C 関数: **scm\_system** (cmd)

オペレーティングシステムの「コマンドプロセッサ」を使用してcmdを実行します。Unixでは通常、デフォルトのシェルである`sh`が使用されます。返される値は、`waitpid`によって返されるcmdの終了ステータスであり、上記の関数を使用して解釈できます。

`system`が引数なしで呼び出された場合、コマンドプロセッサが利用可能かどうかを示すブール値を返します。

Scheme Procedure: **system\*** arg1 arg2 …

C 関数: **scm\_system\_star** (args)

arg1 arg2 ... で指定されたコマンドを実行します。最初の要素は実行するコマンドを示す文字列でなければならず、残りの要素はそのコマンドの各引数を表す文字列でなければなりません。

この関数は、`waitpid`によって提供されるコマンドの終了ステータスを返します。この値は、 `status:exit-val`および関連関数で処理できます。

`system*`は`system`と似ていますが、引数ごとに1つの文字列しか受け付けず、シェル解釈は行いません。コマンドはforkとexeclpを使用して実行されます。したがって、シェル解釈が不要な状況では、この関数は`system`よりも安全である可能性があります。

例: (system\* "echo" "foo" "bar")

Scheme Procedure: **quit** \[status\]

スキーム手順: **exit** \[status\]

Schemeスタックを適切にアンワインドして、現在のプロセスを終了します。ステータスが指定されていない場合は、終了ステータスはゼロです。ステータスが指定され、それが整数である場合は、その整数が終了ステータスとして使用されます。ステータスが`#t`または`#f`の場合、終了ステータスはそれぞれEXIT_SUCCESSまたはEXIT_FAILUREになります。

プロシージャ`exit`は`quit`の別名です。両者は同じ機能を持ちます。

スキーム変数: **EXIT\_SUCCESS**

スキーム変数: **EXIT\_FAILURE**

これらの定数は、成功（ゼロ）または失敗（イチ）を示す標準的な終了コードを表します。

スキームプロシージャ: **primitive-exit** \[status\]

スキームプロシージャ: **primitive-\_exit** \[status\]

C 関数: **scm\_primitive\_exit** (ステータス)

C 関数: **scm\_primitive\_\_exit** (ステータス)

Schemeスタックを巻き戻さずに現在のプロセスを終了します。終了ステータスが指定されている場合はステータス、指定されていない場合はゼロを返します。

`primitive-exit` は C の `exit` 関数を使用するため、通常の C レベルのクリーンアップ (出力ストリームのフラッシュ、`atexit` 関数の呼び出しなど、GNU C ライブラリ リファレンス マニュアルの [Normal Termination](https://doc.guix.gnu.org/libc/latest/en/libc.html#Normal-Termination) を参照) を実行します。

`primitive-_exit` は `_exit` システムコールです（GNU C ライブラリ リファレンス マニュアルの [終了処理の内部構造](https://doc.guix.gnu.org/libc/latest/en/libc.html#Termination-Internals) を参照）。これは、Scheme レベルおよび C レベルのクリーンアップを行わずに、プログラムを即座に終了します。

`primitive-_exit` の典型的な使用例は、`primitive-fork` で作成された子プロセスから実行する場合です。例えば、Gdk プログラムでは、子プロセスは X サーバー接続と、その接続を閉じる C レベルの `atexit` クリーンアップを継承します。しかし、子プロセスで接続を閉じると親プロセスのプロトコルが壊れてしまうため、それを回避して終了するには `primitive-_exit` を使用する必要があります。

Scheme手順: **execl** filename arg …

C言語関数: **scm\_execl** (ファイル名、引数)

filenameで指定されたファイルを新しいプロセスイメージとして実行します。残りの引数はプロセスに渡されます。Cプログラムからは、`main`関数の`argv`引数としてアクセスできます。慣例として、最初の引数はfilenameと同じです。すべての引数は文字列である必要があります。

arg が指定されていない場合、filename は引数リストが空のまま実行されます。これはシステムに依存する副作用を引き起こす可能性があります。

この手順は現在、`execv`システムコールを使用して実装されていますが、Scheme呼び出しインターフェースを使用しているため、`execl`と呼んでいます。

Scheme手順: **execlp** filename arg …

C 関数: **scm\_execlp** (ファイル名、引数)

`execl` と同様ですが、ファイル名にスラッシュが含まれていない場合は、`PATH` 環境変数にリストされているディレクトリを検索して実行するファイルを見つけます。

この手順は現在、`execvp`システムコールを使用して実装されていますが、Scheme呼び出しインターフェースを使用しているため、`execlp`と呼んでいます。

Scheme手順: **execle** filename env arg …

C 関数: **scm\_execle** (ファイル名、環境、引数)

`execl` と似ていますが、新しいプロセスの環境は env で指定され、env は `environ` プロシージャによって返される文字列のリストである必要があります。

この手順は現在、`execve`システムコールを使用して実装されていますが、Scheme呼び出しインターフェースを使用しているため、`execle`と呼んでいます。

スキーム手順: **primitive-fork**

C 関数: **scm\_fork** ()

現在の「親」プロセスを複製して、新しい「子」プロセスを作成します。子プロセスでは戻り値は0です。親プロセスでは戻り値は子プロセスのプロセスID（整数値）です。

複数のスレッドが実行されているプロセスをフォークするのは安全ではないことに注意してください。子プロセスでは、`primitive-fork` を呼び出したスレッドのみが保持されます。ロックされたミューテックスや開いているファイルディスクリプタなど、他のスレッドが保持していたリソースはすべて失われます。実際、POSIX では、マルチスレッドのフォーク後に安全に呼び出せるのは非同期シグナルセーフなプロシージャのみと規定されていますが、これは非常に限られたセットです。Guile は、マルチスレッドプログラムからのフォークを検出すると警告を発します。

> **注:** パイプが設定されたプロセスを生成する場合は、以下で説明する `spawn` 手順を使用すると、`primitive-fork` と `execl` の組み合わせよりも堅牢性が高く (特にマルチスレッドのコンテキストで)、移植性が高く、通常は効率的になります。

この手順は、scshのforkとの名前の競合を避けるため、`fork`から名前が変更されました。

Scheme手順: **spawn** プログラム引数 \[#:environment=(environ)\] \[#:input=(current-input-port)\] \[#:output=(current-output-port)\] \[#:error=(current-error-port)\] \[#:search-path?=#t\]

指定された引数（1つ以上の文字列のリスト。慣例として、最初の引数は通常「program」）を使用してプログラムを実行する新しい子プロセスを生成し、そのPIDを返します。プログラムが見つからない場合、または実行できない場合は、`system-error`例外を発生させます。

キーワード引数 `#:search-path?` が true の場合、プログラムを検索するために `PATH` 環境変数を調べるかどうかを選択します。デフォルトでは true です。

`#:environment` キーワードパラメータは、子プロセスの環境変数のリストを指定します。デフォルト値は `(environ)` です。

キーワード引数 `#:input`、`#:output`、および `#:error` は、子プロセスが標準入力、標準出力、および標準エラーとして使用するポートまたはファイルディスクリプタを指定します。親プロセスから継承されるファイルディスクリプタは他にありません。

以下の例は、`uname`プログラムを`-o`オプション付きで起動し（ GNU Coreutilsの[unameの呼び出し](https://www.gnu.org/software/coreutils/manual/coreutils.html#uname-invocation)を参照）、その標準出力をパイプにリダイレクトして、そこから読み込む方法を示しています。

([use-modules](06_18_modules.md#6182-guileモジュールの使用) (rnrs io ports))

(let\* ((input+output ([pipe](07_02_02_ports_and_file_descriptors.md#722-ポートとファイルディスクリプタ)))
(pid ([spawn](#727-プロセス) "uname" '("uname" "-o")
#:output ([cdr](06_06_08_pairs.md#668-ペア) input+output))))
([close-port](06_12_input_and_output.md#6121-ポート) ([cdr](06_06_08_pairs.md#668-ペア) 入力+出力))
([format](07_05_20_srfi28_basic_format_strings.md#7520-srfi-28---基本フォーマット文字列) #t "read ~s~%" ([get-string-all](06_12_input_and_output.md#6124-テキスト入出力) ([car](06_06_08_pairs.md#668-ペア) input+output)))
([close-port](06_12_input_and_output.md#6121-ポート) ([car](06_06_08_pairs.md#668-ペア) 入力+出力))
([waitpid](#727-プロセス) pid))

⊣ [read](06_16_reading_and_evaluating_scheme_code.md#6162-リーディングスキームコード) "GNU/Linux\n"
⇒ (1234 . 0)

Scheme Procedure: **nice** incr

C 関数: **scm\_nice** (incr)

現在のプロセスの優先度を `incr` だけ上げます。優先度の値が大きいほど、プロセスの実行頻度は低くなります。戻り値は未定義です。

スキーム手順: **setpriority** which who prio

C 関数: **scm\_setpriority** (which, who, prio)

which と who で指定されるプロセス、プロセス グループ、またはユーザーのスケジューリング優先度を設定します。 which は変数 `PRIO_PROCESS`、`PRIO_PGRP`、または `PRIO_USER` のいずれかであり、 who は which に対して相対的に解釈されます (`PRIO_PROCESS` の場合はプロセス 識別子、`PRIO_PGRP` の場合はプロセス グループ識別子、`PRIO_USER` の場合はユーザー識別子)。 who の値がゼロの場合は、現在のプロセス、プロセス グループ、またはユーザーを示します。 prio は範囲 \[−20,20\] の値です。デフォルトの優先度は 0 です。優先度が低い (数値で) ほど、スケジューリングが有利になります。 指定されたすべてのプロセスの優先度を設定します。優先度を下げることができるのはスーパー ユーザーのみです。戻り値は指定されていません。

スキーム手順: **getpriority** which who

C 関数: **scm\_getpriority** (which, who)

which と who で示されるプロセス、プロセス グループ、またはユーザーのスケジューリング優先度を返します。 which は変数 `PRIO_PROCESS`、`PRIO_PGRP`、または `PRIO_USER` のいずれかであり、 who は which に応じて解釈されます (`PRIO_PROCESS` の場合はプロセス 識別子、`PRIO_PGRP` の場合はプロセス グループ識別子、`PRIO_USER` の場合はユーザー識別子)。 who の値がゼロの場合は、現在のプロセス、プロセス グループ、またはユーザーを示します。 指定されたプロセスの中で最も優先度の高い (数値が最も低い) プロセスを返します。

スキーム手順: **getaffinity** pid

C 関数: **scm\_getaffinity** (pid)

プロセスPIDに対応するCPUアフィニティマスクを表すビットベクトルを返します。プロセスがアフィニティを持つ各CPUに対応するビットが、返されるビットベクトル内で設定されます。設定されているビット数は、Guileが他のプロセスと競合することなく使用できるCPUの数の目安となります。

現在、この手順はGNU系のライブラリでのみ定義されています（GNU Cライブラリリファレンスマニュアルの[`sched_getaffinity`](https://doc.guix.gnu.org/libc/latest/en/libc.html#CPU-Affinity)を参照）。

スキーム手順: **setaffinity** pid マスク

C言語関数: **scm\_setaffinity** (pid, mask)

プロセスまたはスレッドID（pid）に対して、CPUアフィニティマスク（ビットベクトル）であるmaskをインストールします。戻り値は未定義です。

現在、この手順はGNU系のライブラリでのみ定義されています（GNU Cライブラリリファレンスマニュアルの[`sched_setaffinity`](https://doc.guix.gnu.org/libc/latest/en/libc.html#CPU-Affinity)を参照）。

システムで使用可能なプロセッサ数を取得する方法については、[スレッド](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6221-スレッド)を参照してください。

* * *

次へ: [端末とPty](07_02_09_terminals_and_ptys.md#729-端末とpty)、前: [プロセス](#727-プロセス)、上: [POSIXシステムコールとネットワーク](07_02_00_posix_system_calls_and_networking.md#72-posix-システムコールとネットワーク) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
