#### 7.2.9 端末とPty [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Terminals-and-Ptys-1)

Scheme Procedure: **isatty?** port [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-isatty_003f)

C 関数: **scm\_isatty\_p** (ポート) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fisatty_005fp)

ポートがシリアル非ファイルデバイスを使用している場合は`#t`を返し、そうでない場合は`#f`を返します。

スキーム手順: **ttyname** ポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ttyname)

C 関数: **scm\_ttyname** (ポート) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fttyname)

基となるシリアル端末デバイスポートの名前を文字列で返します。

スキーム手順: **ctermid** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ctermid)

C 関数: **scm\_ctermid** () [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fctermid)

現在のプロセスを制御している端末のファイル名を含む文字列を返します。

スキーム手順: **tcgetpgrp** ポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-tcgetpgrp)

C 関数: **scm\_tcgetpgrp** (ポート) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ftcgetpgrp)

ファイルディスクリプタの基となるポートで開いている端末に関連付けられているフォアグラウンドプロセスグループのプロセスグループIDを返します。

フォアグラウンドプロセスグループが存在しない場合、戻り値は1より大きい数値で、既存のどのプロセスグループのプロセスグループIDとも一致しません。これは、以前フォアグラウンドジョブだったジョブ内のすべてのプロセスが終了し、他のジョブがまだフォアグラウンドに移動されていない場合に発生する可能性があります。

スキーム手順: **tcsetpgrp** ポート pgid [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-tcsetpgrp)

C 関数: **scm\_tcsetpgrp** (ポート、pgid) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ftcsetpgrp)

ファイルディスクリプタの基となるポートで使用される端末のフォアグラウンドプロセスグループIDを、整数値のpgidに設定します。呼び出し元のプロセスは、pgidと同じセッションに属し、同じ制御端末を持っている必要があります。戻り値は未定義です。

* * *

次へ: [ネットワーク](https://doc.guix.gnu.org/guile/latest/en/guile.html#Networking)、前: [端末とPty](https://doc.guix.gnu.org/guile/latest/en/guile.html#Terminals-and-Ptys)、上: [POSIXシステムコールとネットワーク](https://doc.guix.gnu.org/guile/latest/en/guile.html#POSIX) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
