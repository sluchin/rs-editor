### 6.23 設定、機能、およびランタイムオプション

私の策略があなたの策略と異なるのはなぜですか？考えられるバリエーションは3種類あります。

* ビルドの違い — Guile ソースコードのバージョン、インストールディレクトリ、含まれる機能と含まれない機能を制御する構成フラグなど。
* 動的にロードされるコードの違い — 実行中の Guile に動的にロードできるモジュールによって提供される動作と機能
* 異なる実行時オプション — Guile の動作を制御するために提供されるオプションの一部は、異なる設定になっている場合があります。

Guileは、実行時にこれらの可能なすべてのバリエーションを照会するための「内省的」変数とプロシージャを提供します。実行時オプションについては、オプションの設定を変更したり、オプションの意味に関するドキュメントを取得したりするためのプロシージャも提供します。

* [設定、ビルド、インストール](#6231-設定ビルドおよびインストール)
* [機能追跡](https://doc.guix.gnu.org/guile/latest/en/guile.html#Feature-Tracking )
* [ランタイムオプション](#6233-ランタイムオプション)

* * *

次へ: [機能追跡](#6232-機能追跡)、上: [構成、機能、およびランタイム オプション](#623-設定機能およびランタイムオプション) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 6.23.1 設定、ビルド、およびインストール

以下の手順と変数は、Guileがシステム上でどのように構成、構築、インストールされたかに関する情報を提供します。

Scheme手順: **バージョン**

Scheme Procedure: **effective-version**

Scheme Procedure: **major-version**

スキーム手順: **マイナーバージョン**

Scheme手順: **マイクロバージョン**

C 関数: **scm\_version** ()

C 関数: **scm\_effective\_version** ()

C 関数: **scm\_major\_version** ()

C 関数: **scm\_minor\_version** ()

C 関数: **scm\_micro\_version** ()

Guile の完全なバージョン番号、実効バージョン番号、メジャーバージョン番号、マイナーバージョン番号、マイクロバージョン番号をそれぞれ表す文字列を返します。`effective-version` 関数は、安定版シリーズ中に変更されないバージョン名を返します。現在、これはマイクロバージョンを省略することを意味します。実効バージョンは、バージョン付き共有ディレクトリ名 (例: /usr/share/guile/3.0/) などの項目に使用されます。

([バージョン](#6231-設定ビルドおよびインストール)) ⇒ 「3.0.0」
([effective-version](#6231-設定ビルドおよびインストール)) ⇒ "3.0"
([メジャーバージョン](#6231-設定ビルドおよびインストール)) ⇒ "3"
([マイナーバージョン](#6231-設定ビルドおよびインストール)) ⇒ "0"
([マイクロバージョン](#6231-設定ビルドおよびインストール)) ⇒ "0"

Scheme手順: **%package-data-dir**

C 関数: **scm\_sys\_package\_data\_dir** ()

Guile Schemeファイルが一般的に保存されているディレクトリの名前を返します。Unix系システムでは、通常は/usr/local/share/guileまたは/usr/share/guileです。

Scheme手順: **%library-dir**

C 関数: **scm\_sys\_library\_dir** ()

Guile コアインストールに含まれる Guile Scheme ファイル (サードパーティ パッケージのファイルとは異なります) がインストールされているディレクトリの名前を返します。Unix 系システムでは、通常は /usr/local/share/guile/GUILE\_EFFECTIVE\_VERSION または /usr/share/guile/GUILE\_EFFECTIVE\_VERSION です。

例えば /usr/local/share/guile/3.0。

Scheme Procedure: **%site-dir**

C 関数: **scm\_sys\_site\_dir** ()

サイト固有のGuile Schemeファイルをインストールするディレクトリ名を返します。Unix系システムでは、通常は/usr/local/share/guile/siteまたは/usr/share/guile/siteです。

スキーム手順: **%site-ccache-dir**

C 関数: **scm\_sys\_site\_ccache\_dir** ()

このバージョンの Guile で使用するために、コンパイル済みの `.go` ファイルをインストールするディレクトリを返します。例えば、/usr/lib/guile/3.0/site-ccache のようなディレクトリです。

変数: **%guile-build-info**

特定のGuileの構築中に収集された情報の一覧。エントリは、ディレクトリ、環境変数、バージョン情報など、いくつかのカテゴリに分類できます。

簡単に言うと、`%guile-build-info`に含まれるキーをグループ別に以下に示します。

ディレクトリ

srcdir、top\_srcdir、プレフィックス、exec\_prefix、bindir、sbindir、libexecdir、datadir、sysconfdir、sharedstatedir、localstatedir、libdir、infodir、mandir、includer、pkgdatadir、pkglibdir、pkgincludeir

環境変数

LIBS

バージョン情報

guileversion、libguileinterface、buildstamp

値はすべて文字列です。`LIBS` の値は通常、`pkg-config --libs guile-3.0` の出力の一部としても見つかります。`guileversion` の値は XYZ の形式で、`(version)` によって返される値と同じである必要があります。`libguileinterface` の値は libtool と互換性があり、 CURRENT:REVISION:AGE の形式です (GNU Libtool の [ライブラリ インターフェース バージョン](https://www.gnu.org/software/libtool/manual/libtool.html#Versioning) を参照)。`buildstamp` の値は、コマンド 'date -u +'%Y-%m-%d %T'' (UTC) の出力です。

ソースコードでは、`%guile-build-info` は libguile/libpath.h から初期化されますが、これは完全に生成されるため、ビルド前にこのファイルを削除することで、そのビルドの最新の値が保証されます。

変数: **%host-type**

ホスト Guile の正規ホストタイプ (GNU トリプレット) は、例えば `"x86_64-unknown-linux-gnu"` に設定されました (GNU Autoconf マニュアルの [正規化](https://www.gnu.org/software/autoconf/manual/autoconf.html#Canonicalizing) を参照)。

* * *

次へ: [ランタイム オプション](#6233-ランタイムオプション)、前: [構成、ビルド、インストール](#6231-設定ビルドおよびインストール)、上: [構成、機能、ランタイム オプション](#623-設定機能およびランタイムオプション) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 6.23.2 機能追跡

Guileには、実行中のGuileで利用可能な機能をある程度追跡するSchemeレベルの変数`*features*`があります。`*features*`は、例えば`threads`のようなシンボルのリストであり、それぞれのシンボルは実行中のGuileプロセスの機能を記述します。

変数: **\*features\*** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- _002afeatures_002a)

Guileプロセスの利用可能な機能を説明する記号の一覧。

`set!` を使用して `*features*` 変数を直接変更しないでください。代わりに、次のサブセクションで説明されている手順を参照してください。

* [機能操作](#62321-機能操作)
* [共通機能シンボル](#62322-共通機能シンボル)

* * *

次へ: [共通機能シンボル](#62322-共通機能シンボル)、上へ: [機能追跡](#6232-機能追跡) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 6.23.2.1 機能操作

特定の機能が利用可能かどうかを確認するには、`provided?` プロシージャを使用します。

スキーム手順: **provided?**機能

非推奨のスキーム手順: **feature?** feature

指定された機能が利用可能な場合は「#t」を返し、そうでない場合は「#f」を返します。

独自の Scheme コードから機能を宣伝するには、`provide` プロシージャを使用できます。

スキーム手順: **provide** 機能

このGuileプロセスで使用可能な機能のリストに機能を追加します。

C言語の場合、便宜上、同等の関数は機能名を`char *`型の引数として受け取ります。

C 関数: `void` **scm\_add\_feature** `(const char *str)`

このGuileプロセスで使用可能な機能のリストに、strという名前のシンボルを追加します。

* * *

前へ: [フィーチャ操作](#62321-機能操作)、上へ: [フィーチャ追跡](#6232-機能追跡) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 6.23.2.2 共通機能シンボル

一般的に、特定の機能が利用できる理由は2つ考えられます。1つは、Guileライブラリがその機能を有効にした状態で構成およびコンパイルされている場合（つまり、その機能がシステム上のライブラリに組み込まれている場合）です。もう1つは、Guileによって動的にロードされたC言語またはScheme言語のコードによって、その機能がリストに追加されている場合です。

最初のカテゴリーでは、現在のバージョンのGuileが定義する可能性のある機能（構築方法によって異なります）と、それらが意味するものを以下に示します。

`配列`

配列のサポートを示します（[配列](06_06_13_arrays.md#6613-配列)を参照）。

`array-for-each`

`array-for-each` およびその他の配列マッピング手順が利用可能であることを示します ([配列](06_06_13_arrays.md#6613-配列) を参照)。

文字準備完了？

`char-ready?` 関数が利用可能であることを示します（[Venerable Port Interfaces](06_12_input_and_output.md#61211-ヴェネラブルポートインターフェース)を参照）。

`複雑な`

複素数をサポートしていることを示します。

`現在時刻`

時間関連の関数（`times`、`get-internal-run-time`など）が利用可能であることを示します（[Time](07_02_05_time.md#725-時間)を参照）。

`デバッグ拡張機能`

デバッグ評価ツールが利用可能であることを示し、その制御オプションも併せて表示します。

`遅延`

プロミスのサポートを示します（[遅延評価](06_16_reading_and_evaluating_scheme_code.md#61610-遅延評価)を参照）。

`EIDs`

`geteuid`と`getegid`は実際に有効なユーザーIDとグループIDを返すことを示しています（[プロセス](07_02_07_processes.md#727-プロセス)を参照）。

不正確

不正確な数値に対するサポートを示します。

`入出力拡張機能`

以下の拡張I/O手順が利用可能であることを示します: `ftell`、`redirect-port`、`dup->fdes`、`dup2`、`fileno`、`isatty?`、`fdopen`、`primitive-move->fdes`、`fdes->ports` ([ポートとファイルディスクリプタ](07_02_02_ports_and_file_descriptors.md#722-ポートとファイルディスクリプタ)を参照)。

`net-db`

ネットワークデータベース関数 `scm_gethost`、`scm_getnet`、`scm_getproto`、`scm_getserv`、`scm_sethost`、`scm_setnet`、`scm_setproto`、`scm_setserv`、およびそれらの 'byXXX' バリアントが利用可能であることを示します ([ネットワークデータベース](07_02_11_networking.md#72112-ネットワークデータベース) を参照)。

`posix`

POSIX 関数 (`pipe`、`getgroups`、`kill`、`execl` など) のサポートを示します ([POSIX システムコールとネットワーク](07_02_00_posix_system_calls_and_networking.md#72-posix-システムコールとネットワーク) を参照)。

`fork`

POSIX の `fork` 関数をサポートしていることを示します ([`primitive-fork`](07_02_07_processes.md#727-プロセス) を参照)。

`popen`

`(ice-9 popen)`モジュールにおける`open-pipe`のサポートを示します（ [Pipes](07_02_10_pipes.md#7210-パイプ)を参照）。

`ランダム`

乱数生成関数（`random`、`copy-random-state`、`random-uniform`など）が利用可能であることを示します（[乱数生成](06_06_02_numerical_data_types.md#66214-乱数生成)を参照）。

「無謀な」

Guileが重要なチェックを省略してビルドされたことを示しています。これは決して表示されるべきではありません！

`regex`

`make-regexp`、`regexp-exec`などのPOSIX正規表現をサポートすることを示します（[正規表現関数](06_13_regular_expressions.md#6131-正規表現関数)を参照）。

`ソケット`

ソケット関連の関数（`socket`、`bind`、`connect`など）が利用可能であることを示します（[ネットワークソケットと通信](07_02_11_networking.md#72114-ネットワークソケットと通信)を参照）。

`ソート`

ソートおよびマージ機能が利用可能であることを示します（[ソート](06_09_general_utility_functions.md#693-ソート)を参照）。

`システム`

`system`関数が利用可能であることを示します（[プロセス](07_02_07_processes.md#727-プロセス)を参照）。

`スレッド`

マルチスレッドをサポートしていることを示します（[スレッド](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6221-スレッド)を参照）。

`値`

`values` および `call-with-values` を使用した複数の戻り値のサポートを示します ([複数の値の返却と受け入れ](06_11_controlling_the_flow_of_program_execution.md#6117-複数の値の返却と受け入れ) を参照)。

第2のカテゴリで利用可能な機能は、当然ながら、Guileプロセスに読み込まれた追加コードによって異なります。以下の表は、この理由で遭遇する可能性のある機能の一覧です。

`defmacro`

`defmacro`マクロが使用可能であることを示します（[マクロ](06_08_macros.md#68-マクロ)を参照）。

`describe`

`(oop goops describe)`モジュールがロードされたことを示します。このモジュールは、GOOPSインスタンスの内容を記述するための手順を提供します。

`readline`

Guile がコマンドライン編集用の Readline サポートをロードしたことを示します ([Readline サポート](07_09_readline_support.md#79-readline-サポート) を参照)。

`記録`

`make-record-type` および関連コマンドを使用したレコード定義のサポートを示します（[レコード](06_06_17_records.md#6617-レコード) を参照）。

これらの表は網羅的に見えるかもしれませんが、機能シンボルと利用可能な手順／動作との対応関係は厳密には定義されていないため、実際にはこれらに頼るのは賢明ではありません。何らかの手順の存在を確認する必要があるコードを書く場合は、`provided?` を使用して対応する機能をテストするよりも、`defined?` 手順を直接使用する方が安全でしょう。

* * *

前へ: [機能追跡](#6232-機能追跡)、上へ: [構成、機能、およびランタイム オプション](#623-設定機能およびランタイムオプション) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 6.23.3 ランタイムオプション

`read` のような組み込みプロシージャや、捕捉されないエラーが発生した場合の動作など、組み込みの動作をパラメータ化するためのランタイムオプションが多数用意されています。

リーダーオプションの詳細については、[Reading Scheme Code](06_16_reading_and_evaluating_scheme_code.md#6162-リーディングスキームコード) を参照してください。

印刷オプションの詳細については、[スキーム値の書き込み](06_16_reading_and_evaluating_scheme_code.md#6164-scheme値の記述)を参照してください。

最後に、デバッガーオプションの詳細については、[デバッグオプション](06_26_debugging_infrastructure.md#62645-デバッグオプション)を参照してください。

* [オプションの使用例](#62331-オプションの使用例)

#### 6.23.3.1 オプションの使用例

以下は、読み取りおよびデバッグオプションの処理手順が使用されるセッションの例です。この例では、ユーザーは

1. 記号 `abc` と `aBc` は同じではないことに気づく
2. `read-options` を調べ、`case-insensitive` が「no」に設定されていることを確認します。
3. 大文字小文字を区別しない設定を有効にする
4. 再帰プロンプトを終了します
5. `aBc`と`abc`が同じであることを確認します。

scheme@(guile-user)[\>](06_06_02_numerical_data_types.md#6628-比較述語) (define abc "hello")
scheme@(guile-user)[\>](06_06_02_numerical_data_types.md#6628-比較述語) abc
$1 [\=](06_06_02_numerical_data_types.md#6628-比較述語) "こんにちは"
scheme@(guile-user)[\>](06_06_02_numerical_data_types.md#6628-比較述語) aBc
<unknown-location>: 警告: バインドされていない変数 \`aBc' の可能性があります
エラー: [プロシージャ](06_07_procedures.md#678-セッター付きプロシージャ) module-lookup 内:
エラー: 未定義変数: aBc
新しいプロンプトが表示されます。バックトレースを表示するには「,bt」と入力し、続行するには「,q」と入力してください。
scheme@(guile-user) \[1\][\>](06_06_02_numerical_data_types.md#6628-比較述語) ([read-options](06_16_reading_and_evaluating_scheme_code.md#6162-リーディングスキームコード) 'help)
ソースコード式をコピーしないでください。
位置 はい ソースコード式の位置を記録します。
大文字小文字を区別しない いいえ 記号を小文字に変換します。
キーワード #f キーワード認識のスタイル: #f、'prefix または 'postfix。
r6rs-hex-escapes no R6RS 可変長文字と [string](06_06_05_strings.md#6653-文字列コンストラクタ) の 16 進エスケープを使用します。
角括弧 はい R6RS との互換性のために、\`\[' と \`\]' を括弧として扱います。
hungry-eol-escapes no 文字列では、先頭の空白文字を消費します
行末から脱出しました。
curly-infix いいえ SRFI-105 の curly infix 式をサポートしていません。
scheme@(guile-user) \[1\][\>](06_06_02_numerical_data_types.md#6628-比較述語) ([read-enable](06_16_reading_and_evaluating_scheme_code.md#6162-リーディングスキームコード) 'case-insensitive)
$2 [\=](06_06_02_numerical_data_types.md#6628-比較述語) (角括弧キーワード#f大文字小文字を区別しない位置)
scheme@(guile-user) \[1\][\>](06_06_02_numerical_data_types.md#6628-比較述語) ,q
scheme@(guile-user)[\>](06_06_02_numerical_data_types.md#6628-比較述語) aBc
$3 [\=](06_06_02_numerical_data_types.md#6628-比較述語) "こんにちは"

* * *

次へ: [国際化のサポート](06_25_support_for_internationalization.md#625-国際化のサポート)、前: [設定、機能、およびランタイム オプション](#623-設定機能およびランタイムオプション)、上: [API リファレンス](06_00_api_reference.md#6-apiリファレンス) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
