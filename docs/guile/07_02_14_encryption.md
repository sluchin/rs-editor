#### 7.2.14 暗号化 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Encryption-1)

このセクションの手順は強力な暗号化には適しておらず、同名のよく知られた一般的なシステムライブラリ関数へのインターフェースに過ぎないことにご注意ください。これらの手順は、基となる関数と同等の性能（または不備）を持つため、使用する前にシステムのドキュメントを参照してください（GNU Cライブラリリファレンスマニュアルの[パスワードの暗号化](https://doc.guix.gnu.org/libc/latest/en/libc.html#crypt)を参照）。

スキーム手順: **crypt** キーソルト [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-crypt)

C 関数: **scm\_crypt** (key, salt) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fcrypt)

`crypt` Cライブラリ関数を使用して、鍵とソルト（両方とも文字列）を暗号化します。

`getpass`は厳密には暗号化手順ではありませんが、`crypt`と組み合わせて使用されることが多いため、ここに記載しています。

Scheme Procedure: **getpass** prompt [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-getpass)

C 関数: **scm\_getpass** (プロンプト) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fgetpass)

標準エラー出力にプロンプトを表示し、/dev/tty からパスワードを読み込みます。このファイルにアクセスできない場合は、標準入力から読み込みます。パスワードは最大 127 文字までです。余分な文字と末尾の改行文字は破棄されます。パスワードの読み込み中は、エコーと特殊文字によるシグナルの生成は無効になります。

* * *

次へ: [The (ice-9 getopt-long) Module](https://doc.guix.gnu.org/guile/latest/en/guile.html#getopt_002dlong)、前: [POSIX システムコールとネットワーク](https://doc.guix.gnu.org/guile/latest/en/guile.html#POSIX)、上: [Guile Modules](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guile-Modules) \[[Contents](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[Index](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "Index")\]
