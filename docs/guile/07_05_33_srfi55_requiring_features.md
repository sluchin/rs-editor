#### 7.5.33 SRFI-55 - 必須機能

SRFI-55は、選択したSRFIモジュールをロードするための移植性の高いメカニズムである`require-extension`を提供します。これはGuileコアに実装されているため、SRFI-55自体を使用するためにモジュールを追加する必要はありません。

ライブラリ構文: **require-extension** clause1 clause2 …

条項1、条項2…の機能を必須とし、いずれかの機能が利用できない場合はエラーを発生させる。

句は `(識別子 引数...)` の形式です。現在サポートされている識別子は `srfi` のみで、引数は SRFI 番号です。たとえば、SRFI-1 と SRFI-6 を取得するには、

(require-extension (srfi 1 6))

`require-extension`は最上位レベルでのみ使用できます。

Guile固有のプログラムは、コアにまだ含まれていないSRFIをロードするために`use-modules`を使用するだけで済みます。`require-extension`は、他のScheme実装に移植できるように設計されたプログラム用です。

* * *

次へ: [SRFI-61 - より一般的な `cond` 句](07_05_35_srfi61_a_more_general_cond_clause.md#7535-srfi-61---より一般的な-cond-節)、前: [SRFI-55 - 必須機能](#7533-srfi-55---必須機能)、上: [SRFI サポート モジュール](07_05_00_srfi_support_modules.md#75-srfi-サポート-モジュール) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
