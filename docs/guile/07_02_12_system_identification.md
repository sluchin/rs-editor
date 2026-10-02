#### 7.2.12 システム識別

このセクションでは、Guileが動作しているシステムに関する情報にアクセスするために提供するさまざまな手順を一覧表示します。

Scheme手順: **uname**

C 関数: **scm\_uname** ()

プログラムが実行されているコンピュータシステムに関する情報を含むオブジェクトを返します。

以下の手順は、`uname` によって返されるオブジェクトを受け取り、選択されたコンポーネント（すべて文字列）を返します。

スキーム手順: **utsname:sysname** un

オペレーティングシステムの名称。

スキーム手順: **utsname:nodename** un

コンピュータのネットワーク名。

スキーム手順: **utsname:release** un

オペレーティングシステムの実装における現在のリリースレベル。

スキーム手順: **utsname:version** un

オペレーティングシステムのリリースにおける現在のバージョンレベル。

スキーム手順: **utsname:machine** un

ハードウェアの説明。

Scheme Procedure: **gethostname**

C 関数: **scm\_gethostname** ()

現在のプロセッサのホスト名を返します。

Scheme Procedure: **sethostname** name

C 関数: **scm\_sethostname** (name)

現在のプロセッサのホスト名をnameに設定します。スーパーユーザーのみが使用できます。戻り値は指定されていません。

* * *

次へ: [暗号化](07_02_14_encryption.md#7214-暗号化)、前: [システム識別](#7212-システム識別)、上: [POSIX システムコールとネットワーク](07_02_00_posix_system_calls_and_networking.md#72-posix-システムコールとネットワーク) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
