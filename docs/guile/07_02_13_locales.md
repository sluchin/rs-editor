#### 7.2.13 ロケール

Scheme Procedure: **setlocale** category \[locale\]

C 関数: **scm\_setlocale** (カテゴリ、ロケール)

各種国際化に使用される現在のロケールを取得または設定します。ロケールは「sv\_SE」のような文字列です。

locale が指定されている場合は、指定されたカテゴリのロケールが設定され、新しい値が返されます。locale が指定されていない場合は、現在の値が返されます。category には、次のいずれかの値を指定する必要があります (GNU C ライブラリ リファレンス マニュアルの [ロケールが影響するアクティビティのカテゴリ](https://doc.guix.gnu.org/libc/latest/en/libc.html#Locale-Categories) を参照)。

変数: **LC\_ALL**

変数: **LC\_COLLATE**

変数: **LC\_CTYPE*

変数: **LC\_MESSAGES**

変数: **LC\_MONETARY**

変数: **LC\_NUMERIC**

変数: **LC\_TIME**

一般的な使用例としては、`(setlocale LC\_ALL "")` があり、これは標準環境変数 (`LANG` など) に基づいてすべてのカテゴリを初期化します。カテゴリとロケール名の詳細については、GNU C ライブラリ リファレンス マニュアルの [ロケールと国際化](https://doc.guix.gnu.org/libc/latest/en/libc.html#Locales) を参照してください。

`setlocale` はプロセス全体のロケール設定に影響することに注意してください。スレッドセーフな代替手段については、[ロケールオブジェクトと `make-locale`](06_25_support_for_internationalization.md#6251-guile-による国際化) を参照してください。

`setlocale` は、locale-name がシステムにコンパイルされたロケールのいずれとも一致しない場合に、`system-error` 例外 ([エラーの処理方法](06_11_controlling_the_flow_of_program_execution.md#61113-エラーの処理方法) を発生させます。

* * *

前へ: [ロケール](#7213-ロケール)、上へ: [POSIX システムコールとネットワーク](07_02_00_posix_system_calls_and_networking.md#72-posix-システムコールとネットワーク) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
