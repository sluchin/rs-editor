#### 7.2.13 ロケール [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Locales-1)

Scheme Procedure: **setlocale** category \[locale\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-setlocale)

C 関数: **scm\_setlocale** (カテゴリ、ロケール) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsetlocale)

各種国際化に使用される現在のロケールを取得または設定します。ロケールは「sv\_SE」のような文字列です。

locale が指定されている場合は、指定されたカテゴリのロケールが設定され、新しい値が返されます。locale が指定されていない場合は、現在の値が返されます。category には、次のいずれかの値を指定する必要があります (GNU C ライブラリ リファレンス マニュアルの [ロケールが影響するアクティビティのカテゴリ](https://doc.guix.gnu.org/libc/latest/en/libc.html#Locale-Categories) を参照)。

変数: **LC\_ALL** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-LC_005fALL)

変数: **LC\_COLLATE** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-LC_005fCOLLATE)

変数: **LC\_CTYPE* [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-LC_005fCTYPE)

変数: **LC\_MESSAGES** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-LC_005fMESSAGES)

変数: **LC\_MONETARY** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-LC_005fMONETARY)

変数: **LC\_NUMERIC** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-LC_005fNUMERIC)

変数: **LC\_TIME** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-LC_005fTIME)

一般的な使用例としては、`(setlocale LC\_ALL "")` があり、これは標準環境変数 (`LANG` など) に基づいてすべてのカテゴリを初期化します。カテゴリとロケール名の詳細については、GNU C ライブラリ リファレンス マニュアルの [ロケールと国際化](https://doc.guix.gnu.org/libc/latest/en/libc.html#Locales) を参照してください。

`setlocale` はプロセス全体のロケール設定に影響することに注意してください。スレッドセーフな代替手段については、[ロケールオブジェクトと `make-locale`](https://doc.guix.gnu.org/guile/latest/en/guile.html#i18n-Introduction) を参照してください。

`setlocale` は、locale-name がシステムにコンパイルされたロケールのいずれとも一致しない場合に、`system-error` 例外 ([エラーの処理方法](https://doc.guix.gnu.org/guile/latest/en/guile.html#Handling-Errors) を発生させます。

* * *

前へ: [ロケール](https://doc.guix.gnu.org/guile/latest/en/guile.html#Locales)、上へ: [POSIX システムコールとネットワーク](https://doc.guix.gnu.org/guile/latest/en/guile.html#POSIX) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
