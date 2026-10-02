### 6.2 非推奨 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Deprecation-1)

Guileの機能やその他の特徴は、時として時代遅れになることがあります。Guileの「非推奨化」機能は、こうした問題に対処するための仕組みです。

非推奨の機能を使用すると、実行時に警告メッセージが表示される可能性があります。また、ツールチェーンが十分に新しい場合、`libguile` の非推奨関数を使用すると、リンク時に警告が発生します。

特定のリリースでどのインターフェースが非推奨になったかを知るための主要な情報源は、NEWS ファイルです。このファイルには、非推奨になったものの代わりに何を使用すべきかも記載されています。

READMEファイルには、Guileの公開APIから非推奨機能を追加または削除する方法、および非推奨警告メッセージを制御する方法に関する説明が記載されています。

この仕組みの根底にある考え方は、通常は非推奨のインターフェースもすべて利用可能だが、それらを使用するコードをコンパイルおよび実行する際にフィードバックが得られるため、ユーザーは都合の良いときに新しいAPIに移行できるというものだ。

* * *

次へ: [Guile の初期化](https://doc.guix.gnu.org/guile/latest/en/guile.html#Initialization)、前: [非推奨](https://doc.guix.gnu.org/guile/latest/en/guile.html#Deprecation)、上: [API リファレンス](https://doc.guix.gnu.org/guile/latest/en/guile.html#API-Reference) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
