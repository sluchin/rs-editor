#### 7.2.12 システム識別 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#System-Identification-1)

このセクションでは、Guileが動作しているシステムに関する情報にアクセスするために提供するさまざまな手順を一覧表示します。

Scheme手順: **uname** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-uname)

C 関数: **scm\_uname** () [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005funame)

プログラムが実行されているコンピュータシステムに関する情報を含むオブジェクトを返します。

以下の手順は、`uname` によって返されるオブジェクトを受け取り、選択されたコンポーネント（すべて文字列）を返します。

スキーム手順: **utsname:sysname** un [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-utsname_003asysname)

オペレーティングシステムの名称。

スキーム手順: **utsname:nodename** un [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-utsname_003anodename)

コンピュータのネットワーク名。

スキーム手順: **utsname:release** un [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-utsname_003arelease)

オペレーティングシステムの実装における現在のリリースレベル。

スキーム手順: **utsname:version** un [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-utsname_003aversion)

オペレーティングシステムのリリースにおける現在のバージョンレベル。

スキーム手順: **utsname:machine** un [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-utsname_003amachine)

ハードウェアの説明。

Scheme Procedure: **gethostname** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-gethostname)

C 関数: **scm\_gethostname** () [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fgethostname)

現在のプロセッサのホスト名を返します。

Scheme Procedure: **sethostname** name [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sethostname)

C 関数: **scm\_sethostname** (name) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsethostname)

現在のプロセッサのホスト名をnameに設定します。スーパーユーザーのみが使用できます。戻り値は指定されていません。

* * *

次へ: [暗号化](https://doc.guix.gnu.org/guile/latest/en/guile.html#Encryption)、前: [システム識別](https://doc.guix.gnu.org/guile/latest/en/guile.html#System-Identification)、上: [POSIX システムコールとネットワーク](https://doc.guix.gnu.org/guile/latest/en/guile.html#POSIX) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
