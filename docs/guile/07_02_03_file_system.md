#### 7.2.3 ファイルシステム [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#File-System-1)

これらの手順により、ファイルシステム属性（所有者、アクセス許可、ファイルサイズ、ファイルの種類など）の照会と設定、ファイルの削除、コピー、名前変更、リンク、ディレクトリの作成と削除、およびその内容の照会、ファイルシステムの同期、特殊ファイルの作成が可能になります。

Scheme Procedure: **access?** path how [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-access_003f)

C 関数: **scm\_access** (パス、方法) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005faccess)

呼び出し元のプロセスの実際のUIDとGIDでファイルのアクセス可能性をテストします。パスが存在し、howが要求するすべての権限が許可されている場合は`#t`、そうでない場合は`#f`が返されます。

how は、以下の値のいずれかである整数、または複数の値のビットごとの OR (`logior`) です。

変数: **R\_OK** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-R_005fOK)

読み取り権限をテストします。

変数: **W\_OK** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-W_005fOK)

書き込み権限をテストします。

変数: **X\_OK** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-X_005fOK)

実行権限をテストします。

変数: **F\_OK** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-F_005fOK)

ファイルの存在を確認します。これは他の各テストで暗黙的に前提とされているため、それらと組み合わせる必要はありません。

`access?` は、ファイルの読み書きを試みた際に何が起こるかを単純に示すものではないことに注意することが重要です。通常の状況ではそうなりますが、set-UID または set-GID プログラムではそうではありません。なぜなら、`access?` は実IDをテストするのに対し、ファイルを開く、または実行する試行では実効IDが使用されるからです。

set-UID/GID を実行しないプログラムは、実 ID と実効 ID の違いを無視できますが、特にライブラリ関数では、最大限の汎用性を確保するために、 `access?` を使用して open または execute の結果を予測するのではなく、単純にそれを試みて例外をキャッチするのが最善です。

`access?` の主な用途は、set-UID/GID プログラムが、実効 ID によって付与されるより大きな (あるいはより小さな) 権限を持たずに、呼び出し元のユーザーが実行できたであろう操作を判断できるようにすることです。詳細については、GNU C ライブラリ リファレンス マニュアルの [Testing File Access](https://doc.guix.gnu.org/libc/latest/en/libc.html#Testing-File-Access) を参照してください。

Scheme手順: **stat**オブジェクト \[exception-on-error?\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-stat)

C 関数: **scm_stat** (オブジェクト、例外_on_error) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstat)

object によって指定されたファイルに関するさまざまな情報を含むオブジェクトを返します。object は、ファイル名、ポート、またはファイル上で開いている整数ファイルディスクリプタを含む文字列です (この場合、基盤となるシステムコールとして `fstat` が使用されます)。

オプションの exception\_on\_error 引数が true の場合（デフォルト値）、基となるシステムコールがエラーを返した場合（例えば、ファイルが見つからない、または読み取り不可能な場合）、例外が発生します。それ以外の場合は、エラーによって `stat` は `#f` を返します。

`stat` によって返されるオブジェクトは、以下のプロシージャに単一のパラメータとして渡すことができ、これらのプロシージャはすべて整数を返します。

Scheme Procedure: **stat:dev** st [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-stat_003adev)

ファイルが格納されているデバイスの番号。

Scheme Procedure: **stat:ino** st [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-stat_003aino)

ファイルシリアル番号は、このファイルを同じデバイス上の他のすべてのファイルと区別するためのものです。

スキーム手順: **stat:mode** st [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-stat_003amode)

ファイルのモード。これは、ファイルの種類情報とファイル権限ビットを組み込んだ整数値です。詳しくは、下記の`stat:type`および`stat:perms`を参照してください。

スキーム手順: **stat:nlink** st [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-stat_003anlink)

ファイルへのハードリンクの数。

Scheme Procedure: **stat:uid** st [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-stat_003auid)

ファイルの所有者のユーザーID。

スキームプロシージャ: **stat:gid** st [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-stat_003agid)

ファイルのグループID。

Scheme Procedure: **stat:rdev** st [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-stat_003ardev)

デバイスID。この項目は、文字ファイルまたはブロック特殊ファイルに対してのみ定義されます。システムによってはこのフィールドが全く利用できない場合があり、その場合は`stat:rdev`は`#f`を返します。

Scheme Procedure: **stat:size** st [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-stat_003asize)

通常のファイルのサイズ（バイト単位）。

スキーム手順: **stat:atime** st [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-stat_003aatime)

ファイルへの最終アクセス時刻（秒単位）。

Scheme Procedure: **stat:mtime** st [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-stat_003amtime)

ファイルの最終更新時刻（秒単位）。

スキーム手順: **stat:ctime** st [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-stat_003actime)

ファイルの属性の最終更新時刻（秒単位）。

スキーム手順: **stat:atimensec** st [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-stat_003atimensec)

スキーム手順: **stat:mtimensec** st [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-stat_003amtimensec)

スキーム手順: **stat:ctimensec** st [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-stat_003actimensec)

ファイルのアクセス、変更、または属性変更時刻の小数部分（ナノ秒単位）。ナノ秒単位のタイムスタンプは、一部のオペレーティングシステムとファイルシステムでのみ利用可能です。Guileがファイルのナノ秒単位のタイムスタンプを取得できない場合、これらのフィールドは0に設定されます。

スキーム手順: **stat:blksize** st [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-stat_003ablksize)

ファイルの読み書きに最適なブロックサイズ（バイト単位）。システムによってはこのフィールドが利用できない場合があり、その場合は`stat:blksize`が適切な推奨ブロックサイズを返します。

Scheme Procedure: **stat:blocks** st [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-stat_003ablocks)

ファイルが占めるディスク容量を512バイトブロック単位で計測します。システムによってはこのフィールドが利用できない場合があり、その場合は`stat:blocks`は`#f`を返します。

さらに、以下の手順では、`stat:mode`からの情報をより便利な形式で返します。

Scheme Procedure: **stat:type** st [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-stat_003atype)

ファイルの種類を表すシンボル。指定可能な値は、「regular」、「directory」、「symlink」、「block-special」、「char-special」、「fifo」、「socket」、「unknown」です。

スキーム手順: **stat:perms** st [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-stat_003aperms)

アクセス権限ビットを表す整数。

Scheme手順: **lstat** パス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-lstat)

C 関数: **scm\_lstat** (パス) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005flstat)

`stat` と似ていますが、シンボリックリンクをたどりません。つまり、シンボリックリンク自体に関する情報が返され、リンク先のファイルに関する情報は返されません。path は文字列である必要があります。

スキーム手順: **statat** dir ファイル名 \[flags\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-statat)

C 関数: **scm\_statat** dir filename flags [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstatat)

`stat`と同様ですが、ファイル名をファイルポートdirで参照されるディレクトリからの相対パスで解決します。オプション引数flagsは`AT_SYMLINK_NOFOLLOW`にすることができ、その場合、ファイル名がシンボリックリンクであっても参照解除されません。

Scheme Procedure: **readlink** path [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-readlink)

C 関数: **scm\_readlink** (パス) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005freadlink)

パス（文字列、またはシステムがサポートしている場合はポート）で指定されたシンボリックリンクの値、つまりリンクが指すファイルを返します。

ポートで表されるシンボリックリンクを読み取るには、シンボリックリンクが`O_NOFOLLOW`および`O_PATH`フラグ付きで開かれている必要があります。`(provided? 'readlink-port)`はポートがサポートされているかどうかを示します。

スキーム手順: **chown** オブジェクト所有者グループ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-chown)

C言語関数: **scm\_chown** (オブジェクト、所有者、グループ) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchown)

オブジェクトが参照するファイルの所有権とグループを、整数値の owner と group に変更します。オブジェクトは、ファイル名を含む文字列、またはプラットフォームが `fchown` をサポートしている場合（GNU C ライブラリ リファレンス マニュアルの [ファイル所有者](https://doc.guix.gnu.org/libc/latest/en/libc.html#File-Owner) を参照）、ファイル上で開いているポートまたは整数ファイル記述子です。戻り値は未定義です。

オブジェクトがシンボリックリンクの場合、オペレーティングシステムに応じて、リンクの所有権または参照ファイルの所有権が変更されます（lchown は現在サポートされていません）。所有者またはグループに `-1` が指定されている場合は、その ID は変更されません。

スキーム手順: **chownat** dir name owner group \[flags\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-chownat)

C 関数: **scm\_chownat** (dir, name, owner, group, flags) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchownat)

`chown` と同様ですが、ファイルポート dir で参照されるディレクトリ内の name という名前のファイルの所有者および/またはグループを変更します。オプション引数 flags はビットマスクです。`AT_SYMLINK_NOFOLLOW` が存在する場合、name がシンボリックリンクであれば、参照解除されません。

スキーム手順: **chmod** オブジェクトモード [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-chmod)

C 関数: **scm\_chmod** (オブジェクト、モード) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fchmod)

object で参照されるファイルのパーミッションを変更します。object は、ファイル名、ポート、またはファイルに対して開いている整数ファイルディスクリプタを含む文字列です (この場合、基盤となるシステムコールとして `fchmod` が使用されます)。mode は、新しいパーミッションを 10 進数で指定します (例: `(chmod "foo" #o755)`)。戻り値は未定義です。

Scheme 手順: **utime** オブジェクト \[actime \[modtime \[actimens \[modtimens \[flags\]\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-utime)

C 関数: **scm\_utime** (object, actime, modtime, actimens, modtimens, flags) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005futime)

`utime` は、オブジェクトで指定されたファイルのアクセス時刻と変更時刻を設定します。actime または modtime が指定されていない場合は、現在時刻が使用されます。actime と modtime は、`current-time` プロシージャによって返される整数値の時刻である必要があります。

オブジェクトはファイル名、または（システムがサポートしている場合）ポート番号である必要があります。

オプションのactimensとmodtimensは、actimeとmodtimeを加算するためのナノ秒単位の値です。ナノ秒単位の精度は、一部のファイルシステムとオペレーティングシステムの組み合わせでのみサポートされています。

([utime](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-utime) "foo" ([\-](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002d) ([current-time](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-current_002dtime)) 3600))

アクセス時刻を1時間前に、変更時刻を現在時刻に設定します。

最後に、フラグは「0」または定数「AT_SYMLINK_NOFOLLOW」のいずれかになります。これは、シンボリックリンクであってもオブジェクトの時間を設定します。

GNU/Linuxシステムでは、少なくともLinuxカーネル5.10.46を使用している場合、オブジェクトがポートである場合、`AT_SYMLINK_NOFOLLOW`が設定されていても、シンボリックリンクにならない可能性があります。これはLinuxのバグかGuileのラッパーのバグのどちらかです。正確な原因は不明です。

Scheme手順: **delete-file** str [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-delete_002dfile)

C 関数: **scm\_delete\_file** (str) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fdelete_005ffile)

str で指定されたパスのファイルを削除（または「リンク解除」）します。

Scheme Procedure: **delete-file-at** dir str \[flags\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-delete_002dfile_002dat)

C 関数: **scm\_delete\_file\_at** (dir, str, flags) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fdelete_005ffile_005fat)

`unlink`と同様ですが、ファイルポートdirで参照されるディレクトリを基準としてstrを解決します。

オプションの flags 引数には `AT_REMOVEDIR` を指定できます。この場合、`delete-file-at` は `delete-file` ではなく `rmdir` のように動作します。なぜ POSIX には代わりに `rmdirat` 関数がないのでしょうか？ 全く分かりません！

Scheme Procedure: **copy-file** oldfile newfile \[#:copy-on-write='auto\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-copy_002dfile)

C 関数: **scm\_copy\_file** (oldfile, newfile) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fcopy_005ffile)

oldfileで指定されたファイルをnewfileにコピーします。戻り値は未定義です。

`#:copy-on-write` キーワード引数は、コピーオンライトコピーを試行するかどうか、および失敗した場合の動作を決定します。指定可能な値は、`'always` (コピーオンライトを試行し、失敗した場合はエラーを返す)、`'auto` (コピーオンライトを試行し、失敗した場合は通常のコピーにフォールバックする)、および `'never` (通常のコピーを実行する) です。

Scheme Procedure: **sendfile** out in count \[offset\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sendfile)

C 関数: **scm\_sendfile** (out, in, count, offset) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsendfile)

入力から出力へ、指定されたバイト数を送信します。入力と出力は、いずれも開いているファイルポートまたはファイルディスクリプタである必要があります。オフセットが省略された場合は、入力の現在位置から読み取りを開始します。オフセットが指定されている場合は、指定されたオフセット位置から読み取りを開始します。実際に送信されたバイト数を返します。

in がポートである場合、in のポートとしてのオフセットは、基となるファイルディスクリプタのオフセットとは異なる可能性があるため、オフセットを指定する方が望ましい場合が多い。

GNU/Linuxなど、この機能をサポートするシステムでは、この手順は通常システムコールに対応するlibcの`sendfile`関数を使用します。これは、一連の`read`および`write`システムコールを実行するよりも高速です。典型的な用途としては、ソケット経由でファイルを送信することが挙げられます。

場合によっては、libc の `sendfile` 関数が `EINVAL` または `ENOSYS` を返すことがあります。その場合、Guile の `sendfile` プロシージャは自動的に `read` と `write` の一連の呼び出しを実行するようにフォールバックします。

場合によっては、libc関数がcountよりも少ないバイト数を送信することがあります。例えば、outがパイプなどの低速または制限のあるデバイスである場合などです。そのような場合、Guileの`sendfile`は、countバイトが正確に送信されるか、エラーが発生するまで自動的に再試行します。

Scheme 手順: **rename-file** oldname newname [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-rename_002dfile)

C 関数: **scm\_rename** (oldname, newname) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005frename)

oldnameで指定されたファイルの名前をnewnameに変更します。戻り値は未定義です。

スキーム手順: **rename-file-at** olddir oldname newdir newname [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-rename_002dfile_002dat)

C 関数: **scm\_renameat** (olddir, oldname, newdir, newname) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005frenameat)

`rename-file`と同様ですが、olddirまたはnewdirがtrueの場合、現在の作業ディレクトリではなく、ファイルポートolddirまたはnewdirで指定されたディレクトリを基準としてoldnameまたはnewnameを解決します。

スキーム手順: **link** oldpath newpath [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-link)

C 関数: **scm\_link** (oldpath, newpath) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005flink)

ファイルシステム内に、oldpathで指定されたファイル名に対してnewpathという新しい名前を作成します。oldpathがシンボリックリンクの場合、システムによってはリンクがたどられる場合とたどられない場合があります。

スキーム手順: **symlink** oldpath newpath [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-symlink)

C 関数: **scm\_symlink** (oldpath, newpath) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsymlink)

newpath という名前のシンボリックリンクを作成し、その値（つまり、oldpath を指す）を設定します。戻り値は未指定です。

スキーム手順: **symlinkat** dir oldpath newpath [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-symlinkat)

C 関数: **scm\_symlinkat** (dir, oldpath, newpath) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsymlinkat)

`symlink` と同様ですが、ファイルポート dir が参照するディレクトリを基準として newpath を解決します。

Scheme手順: **mkdir** path \[mode\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-mkdir)

C言語関数: **scm\_mkdir** (path, mode) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fmkdir)

パスで指定された名前の新しいディレクトリを作成します。mode が省略された場合、ディレクトリのパーミッションは現在の umask でマスクされた `#o777` に設定されます ([`umask`](https://doc.guix.gnu.org/guile/latest/en/guile.html#Processes) を参照)。それ以外の場合は、mode で指定された値に現在の umask でマスクされた値が設定されます。戻り値は未定義です。

Scheme Procedure: **mkdirat** dir path \[mode\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-mkdirat)

C 関数: **scm\_mkdirat** (dir, path, mode) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fmkdirat)

`mkdir`と同様ですが、ファイルポートdirで参照されるディレクトリからの相対パスでパスを解決します。

Scheme手順: **rmdir** パス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-rmdir)

C 関数: **scm\_rmdir** (path) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005frmdir)

指定されたパスで指定された既存のディレクトリを削除します。この操作を成功させるには、ディレクトリが空である必要があります。戻り値は未指定です。

スキーム手順: **opendir** ディレクトリ名 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-opendir)

C 関数: **scm\_opendir** (dirname) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fopendir)

dirnameで指定されたディレクトリを開き、ディレクトリストリームを返します。

これと以下の手順を使用する前に、利用可能なディレクトリ走査の上位レベルの手順を確認してください（[ファイルツリーウォーク](https://doc.guix.gnu.org/guile/latest/en/guile.html#File-Tree-Walk)を参照）。

Scheme Procedure: **directory-stream?** オブジェクト [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-directory_002dstream_003f)

C 関数: **scm\_directory\_stream\_p** (オブジェクト) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fdirectory_005fstream_005fp)

`opendir` によって返されるディレクトリ ストリームであるかどうかを示すブール値を返します。

Scheme Procedure: **readdir** stream [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-readdir)

C 関数: **scm\_readdir** (stream) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005freaddir)

ディレクトリストリームから次のディレクトリエントリを文字列として返します。読み込むエントリが残っていない場合は、ファイルの末尾オブジェクトが返されます。

Scheme手順: **rewinddir**ストリーム [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-rewinddir)

C 関数: **scm\_rewinddir** (ストリーム) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005frewinddir)

ディレクトリポートストリームをリセットして、次回の `readdir` 呼び出しで最初のディレクトリエントリが返されるようにします。

Scheme Procedure: **closedir** ストリーム [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-closedir)

C 関数: **scm\_closedir** (stream) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fclosedir)

ディレクトリストリームを閉じます。戻り値は未指定です。

ディレクトリ内のすべてのエントリを表示する方法を示す例を以下に示します。

(ディレクトリを定義します ([opendir](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-opendir) "/usr/lib"))
(do ((entry ([readdir](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-readdir) dir) ([readdir](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-readdir) dir)))
(([eof-object?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-eof_002dobject_003f) エントリ))
([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) エントリ)([newline](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-newline)))
([closedir](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-closedir) ディレクトリ)

Scheme 手順: **sync** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sync)

C 関数: **scm\_sync** () [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsync)

オペレーティングシステムのディスクバッファをフラッシュします。戻り値は未指定です。

Scheme Procedure: **mknod** path type perms dev [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-mknod)

C 関数: **scm\_mknod** (path, type, perms, dev) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fmknod)

デバイスに対応するファイルなど、新しい特殊ファイルを作成します。path はファイル名を指定します。type には、'regular'、'directory'、'symlink'、'block- special'、'char-special'、'fifo'、'socket' のいずれかのシンボルを指定する必要があります。perms (整数) はファイルのアクセス許可を指定します。dev (整数) は、特殊ファイルが参照するデバイスを指定します。その正確な解釈は、作成される特殊ファイルの種類によって異なります。

例えば、

([mknod](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-mknod) "/dev/fd0" 'block-special #o660 ([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) ([\*](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002a) 2 256) 2))

戻り値は指定されていません。

スキーム手順: **tmpnam** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-tmpnam)

C 関数: **scm\_tmpnam** () [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ftmpnam)

既に存在しない一時ファイルの自動生成名を返します。名前にはパスが含まれます。通常は /tmp ディレクトリですが、これはシステムによって異なります。

`tmpnam`を使用する際は注意が必要です。ファイル名を選択してからファイルを作成するまでの間に、別のプログラムがその名前を使用したり、攻撃者が重要なファイルへのシンボリックリンクを作成して、そのファイルを上書きさせてしまう可能性があります。

安全な方法は、上書きを防ぐために `open` 関数で `O_EXCL` を指定してファイルを作成することです。ファイルが存在する場合 (エラー `EEXIST` が発生した場合) は、ループ内で別の名前で再試行できます。以下の `mkstemp` 関数はその処理を行います。

Scheme手順: **mkstemp** tmpl \[mode\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-mkstemp)

ファイルシステム内に新しい固有のファイルを作成し、そのファイルへの読み書き用に新しいバッファ付きポートを開いた状態で返します。

tmpl は、ファイルを作成する場所を指定する文字列です。末尾は「XXXXXX」でなければなりません。新しく作成されるファイルの名前は tmpl と同じですが、「X」の部分が置き換えられます。ファイル名は、返されたポートに対して `port-filename` を呼び出すことで確認できます。

新しく作成されたファイルはGuileによって自動的に削除されないことに注意してください。おそらく、呼び出し元はファイルが不要になったときに`delete-file`を呼び出すように設定する必要があります。

POSIX ではファイルのパーミッション モードは指定されていません。GNU およびほとんどのシステムでは `#o600` ですが、必要に応じてアプリケーションは `chmod` を使用してそれを緩和できます。たとえば、通常のファイル作成でよく使用される `#o666` から `umask` を引いたものなどです。

(let ((port (mkstemp "/tmp/myfile-XXXXXX")))
(chmod port (logand #o666 (lognot (umask))))
...)

オプションの mode 引数は、新しいファイルを開くモードを、`open-file` と同じ形式の文字列として指定します。デフォルト値は `"w+"` です。

Scheme手順: **tmpfile** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-tmpfile)

C 関数: **scm\_tmpfile** () [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ftmpfile)

入力/出力ポートを、stdio.hで定義されているパス接頭辞`P_tmpdir`を使用して命名された一意の一時ファイルに格納します。このファイルは、ポートが閉じられるかプログラムが終了すると自動的に削除されます。

スキーム手順: **mkdtemp** tmpl [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-mkdtemp)

C 関数: **scm\_mkdtemp** (tmpl) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fmkdtemp)

テンプレート文字列 tmpl に従って、新しいディレクトリを作成します。

tmplはディレクトリ名を指定する文字列です。tmplの最後の6文字は「XXXXXX」でなければなりません。実行が成功すると、新しいディレクトリ名が返されます。この名前はtmplと同じ形式ですが、「XXXXXX」の部分が変更され、ディレクトリ名が一意になるように調整されています。

作成されるディレクトリのパーミッションはOSに依存しますが、通常は`#o700`です。

テンプレートの形式が間違っている場合、またはディレクトリを作成できない場合は、エラーが発生する可能性があります。

Scheme手順: **dirname** filename [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-dirname)

C 関数: **scm\_dirname** (ファイル名) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fdirname)

ファイル名 filename のディレクトリ名部分を返します。filename にディレクトリ名部分が含まれていない場合は、「.」を返します。

Scheme手順: **basename** filename \[suffix\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-basename)

C 関数: **scm\_basename** (ファイル名、接尾辞) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbasename)

ファイル名 filename のベース名を返します。ベース名とは、ディレクトリ要素を含まないファイル名のことです。接尾辞が指定されており、それがベース名の末尾と等しい場合は、接尾辞も削除されます。

([basename](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-basename) "/tmp/test.xml" ".xml")
⇒ 「テスト」

Scheme手順: **canonicalize-path** path [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-canonicalize_002dpath)

C 関数: **scm\_canonicalize\_path** (path) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fcanonicalize_005fpath)

パスの正規（絶対）パスを返します。正規パスには、`.` や `..` の要素、パス区切り文字の繰り返し（`/`）、シンボリックリンクは含まれません。

パスの構成要素のいずれかが存在しない場合はエラーが発生します。

([canonicalize-path](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-canonicalize_002dpath) "test.xml")
⇒ "/tmp/test.xml"

Scheme手順: **file-exists?** filename [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-file_002dexists_003f)

ファイル名 filename が存在する場合は `#t` を返し、存在しない場合は `#f` を返します。

GNUなどの多くのオペレーティングシステムでは、ファイル名の構成要素を区切るために`/`（スラッシュ）を使用します。`/`で始まるファイル名は、_絶対ファイル名_とみなされます。これらの規則はPOSIX基本定義で規定されており、準拠するファイル名は「パス名」と呼ばれます。一部のオペレーティングシステムでは異なる規則が使用されています。特にWindowsでは、ファイル名の区切り文字として`\`（バックスラッシュ）を使用し、絶対ファイル名として`C:\`のような_ボリューム名_の概念も持っています。以下の手順と変数は、移植性の高いファイル名操作をサポートします。

スキーム手順: **システムファイル名規則** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-system_002dfile_002dname_002dconvention)

このGuileが実行されているシステムの種類に応じて、`posix`または`windows`のいずれかを返します。

Scheme Procedure: **ファイル名区切り文字?** c [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-file_002dname_002dseparator_003f)

ホストプラットフォーム上で文字「c」がファイル名の区切り文字である場合は、trueを返します。

Scheme手順: **絶対ファイル名?** ファイル名 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-absolute_002dfile_002dname_003f)

ファイル名がホストプラットフォーム上の絶対ファイル名を表す場合は、trueを返します。

Scheme変数: **ファイル名区切り文字列** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-file_002dname_002dseparator_002dstring)

推奨されるファイル名区切り文字。

Windows 用の MinGW ビルドでは、`/` と `\` の両方が有効な区切り文字であることに注意してください。したがって、ファイル名の構成要素を抽出する場合など、プログラムは `file-name-separator-string` が唯一のファイル名区切り文字であると想定すべきではありません。

スキーム手順: **in-vicinity** ディレクトリ ファイル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-in_002dvicinity)

ディレクトリとファイルを連結し、ファイル名区切り文字列（デフォルトではスラッシュ）がまだ存在しない場合は、間に挿入します。これにより、ファイル名の作成が容易になります。

* * *

次へ: [時間](https://doc.guix.gnu.org/guile/latest/en/guile.html#Time)、前: [ファイルシステム](https://doc.guix.gnu.org/guile/latest/en/guile.html#File-System)、上: [POSIX システムコールとネットワーク](https://doc.guix.gnu.org/guile/latest/en/guile.html#POSIX) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
