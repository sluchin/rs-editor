#### 7.2.14 暗号化

このセクションの手順は強力な暗号化には適しておらず、同名のよく知られた一般的なシステムライブラリ関数へのインターフェースに過ぎないことにご注意ください。これらの手順は、基となる関数と同等の性能（または不備）を持つため、使用する前にシステムのドキュメントを参照してください（GNU Cライブラリリファレンスマニュアルの[パスワードの暗号化](https://doc.guix.gnu.org/libc/latest/en/libc.html#crypt)を参照）。

スキーム手順: **crypt** キーソルト

C 関数: **scm\_crypt** (key, salt)

`crypt` Cライブラリ関数を使用して、鍵とソルト（両方とも文字列）を暗号化します。

`getpass`は厳密には暗号化手順ではありませんが、`crypt`と組み合わせて使用されることが多いため、ここに記載しています。

Scheme Procedure: **getpass** prompt

C 関数: **scm\_getpass** (プロンプト)

標準エラー出力にプロンプトを表示し、/dev/tty からパスワードを読み込みます。このファイルにアクセスできない場合は、標準入力から読み込みます。パスワードは最大 127 文字までです。余分な文字と末尾の改行文字は破棄されます。パスワードの読み込み中は、エコーと特殊文字によるシグナルの生成は無効になります。

* * *

次へ: [The (ice-9 getopt-long) Module](07_04_the_ice9_getoptlong_module.md#74-ice-9-getopt-long-モジュール)、前: [POSIX システムコールとネットワーク](07_02_00_posix_system_calls_and_networking.md#72-posix-システムコールとネットワーク)、上: [Guile Modules](07_00_guile_modules.md#7つのguileモジュール) \[[Contents](00_contents.md "目次")\]\[[Index](index_r5rs.md "Index")\]
