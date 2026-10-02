#### 7.5.43 SRFI-98 環境変数へのアクセス。

これは、現在の環境とやり取りするための Guile の組み込みサポートをラップしたポータブルなライブラリです。[ランタイム環境](07_02_06_runtime_environment.md#726-ランタイム環境) を参照してください。

Scheme手順: **get-environment-variable** name

文字列 `name` で指定された環境変数の値を含む文字列を返します。指定された環境変数が見つからない場合は `#f` を返します。これは `(getenv name)` と同等です。

Scheme手順: **get-environment-variables**

すべての環境変数の名前と値を、キーと値の両方が文字列である連想リストとして返します。

* * *

次へ: [SRFI-111 ボックス](07_05_45_srfi111_boxes.md#7545-srfi-111-ボックス)、前: [SRFI-98 環境変数へのアクセス](#7543-srfi-98-環境変数へのアクセス)、上: [SRFI サポート モジュール](07_05_00_srfi_support_modules.md#75-srfi-サポート-モジュール) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
