#### 7.5.43 SRFI-98 環境変数へのアクセス。[¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d98-Accessing-environment-variables_002e)

これは、現在の環境とやり取りするための Guile の組み込みサポートをラップしたポータブルなライブラリです。[ランタイム環境](https://doc.guix.gnu.org/guile/latest/en/guile.html#Runtime-Environment) を参照してください。

Scheme手順: **get-environment-variable** name [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-get_002denvironment_002dvariable)

文字列 `name` で指定された環境変数の値を含む文字列を返します。指定された環境変数が見つからない場合は `#f` を返します。これは `(getenv name)` と同等です。

Scheme手順: **get-environment-variables** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-get_002denvironment_002dvariables)

すべての環境変数の名前と値を、キーと値の両方が文字列である連想リストとして返します。

* * *

次へ: [SRFI-111 ボックス](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d111)、前: [SRFI-98 環境変数へのアクセス](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d98)、上: [SRFI サポート モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI-Support) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
