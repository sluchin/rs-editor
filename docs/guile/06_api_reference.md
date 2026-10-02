# 6 API リファレンス

> **原文**: [Guile Reference Manual - API Reference](https://www.gnu.org/software/guile/manual/html_node/API-Reference.html)
>
> このドキュメントは GNU Free Documentation License の下で公開されている原文の翻訳です。

Guile は、開発者に対して2つのコア言語、Scheme と C でアプリケーションプログラミングインターフェース（API）を提供しています。マニュアルのこの部分には、Scheme と C の両方のインターフェースを通じて利用可能なすべての機能のリファレンスドキュメントが含まれています。

第6章は分量が多いため、節ごとにファイルを分けています。

| 節 | ファイル |
|---|---|
| 6.1 Guile API の概要 | このファイル |
| 6.2 非推奨 | このファイル |
| 6.3 SCM 型 | このファイル |
| 6.4 Guile の初期化 | このファイル |
| 6.5 スナーフィングマクロ | このファイル |
| 6.6 データ型 | `06_06_data_types*.md` |
| 6.7 手続き | `06_07_procedures.md` |
| 6.8 マクロ | `06_08_macros.md` |
| 6.9 汎用ユーティリティ関数 | `06_09_utility_functions.md` |
| 6.10 定義と変数の束縛 | `06_10_definitions.md` |
| 6.11 プログラム実行の流れの制御 | `06_11_control_flow.md` |
| 6.12 入出力 | `06_12_input_output.md` |
| 6.13 正規表現 | `06_13_regular_expressions.md` |
| 6.14 LALR(1) 構文解析 | `06_14_lalr_parsing.md` |
| 6.15 PEG 構文解析 | `06_15_peg_parsing.md` |
| 6.16 Scheme コードの読み込みと評価 | `06_16_read_eval.md` |
| 6.17 メモリ管理とガベージコレクション | `06_17_memory_management.md` |
| 6.18 モジュール | `06_18_modules.md` |
| 6.19 外部関数インターフェース | `06_19_ffi.md` |
| 6.20 外部オブジェクト | `06_20_foreign_objects.md` |
| 6.21 Smob | `06_21_smobs.md` |
| 6.22 スレッド、ミューテックス、async、動的ルート | `06_22_threads.md` |
| 6.23 設定、機能、実行時オプション | `06_23_configuration.md` |
| 6.24 他の言語のサポート | `06_24_other_languages.md` |
| 6.25 国際化のサポート | `06_25_i18n.md` |
| 6.26 デバッグ基盤 | `06_26_debugging.md` |
| 6.27 コードカバレッジレポート | `06_27_code_coverage.md` |

## 6.1 Guile API の概要

Guile のアプリケーションプログラミングインターフェース（API）は、アプリケーション開発者が C または Scheme のどちらのプログラミングでも使用できる機能を提供します。インターフェースは、C ではマクロ、関数、変数であり、Scheme では手続き、変数、構文、その他の種類のオブジェクトである要素で構成されます。

多くの要素は、それぞれに適した形で、Scheme と C の両方で利用できます。たとえば、Scheme の手続き `assq` は、C コードからも `scm_assq` として利用できます。これらの要素は、Scheme と C の両方の側面を扱いながら、一度だけ文書化されています。

要素の Scheme での名前は、規則的な方法でその C での名前と関係しています。また、C 関数は体系的な方法でパラメータを受け取ります。

通常、C 関数の名前は、いくつかの単純なテキスト変換を使って、その Scheme での名前から導き出すことができます。

- `-`（ハイフン）を `_`（アンダースコア）に置き換える。
- `?`（疑問符）を `_p` に置き換える。
- `!`（感嘆符）を `_x` に置き換える。
- 内部の `->` を `_to_` に置き換える。
- `<=`（以下）を `_leq` に置き換える。
- `>=`（以上）を `_geq` に置き換える。
- `<`（より小さい）を `_less` に置き換える。
- `>`（より大きい）を `_gr` に置き換える。
- 先頭に `scm_` を付ける。

C 関数は、対応する Scheme 関数が可変個の引数を取る場合でも、常に `SCM` 型の固定個の引数を取ります。

一部の Scheme 関数では、最後のいくつかの引数がオプションです。対応する C 関数は、常にすべてのオプション引数を指定して呼び出さなければなりません。引数が指定されなかったかのような効果を得るには、その値として `SCM_UNDEFINED` を渡します。途中の引数に対してこれを行うことはできません。ある引数が `SCM_UNDEFINED` であれば、それに続くすべての引数も `SCM_UNDEFINED` でなければなりません。

一部の Scheme 関数は、任意の個数の残余引数を取ります。対応する C 関数は、これらすべての引数のリストを付けて呼び出さなければなりません。このリストは常に C 関数の最後の引数です。

これら2つの変形は組み合わせることもできます。

Scheme 関数に対応する C 関数の戻り値の型は常に `SCM` です。そのため、以下の説明では、戻り値と引数を除いて型はしばしば省略されています。

## 6.2 非推奨

ときどき、Guile の関数やその他の機能は時代遅れになります。Guile の非推奨（deprecation）は、これに対処するのを助ける仕組みです。

非推奨の機能を使うと、実行時に警告メッセージが表示される可能性が高いです。また、十分に新しいツールチェーンがあれば、libguile の非推奨の関数を使うとリンク時に警告が出ます。

特定のリリースでどのインターフェースが非推奨になっているかについての主要な情報源は、ファイル `NEWS` です。このファイルには、時代遅れになったものの代わりに何を使うべきかも記載されています。

ファイル `README` には、Guile の公開 API から非推奨の機能を含めるか除外するかを制御する方法と、非推奨の警告メッセージを制御する方法についての説明が含まれています。

この仕組みの背後にある考えは、通常はすべての非推奨のインターフェースが利用可能でありながら、それらを使うコードをコンパイルしたり実行したりするときにフィードバックが得られるので、自分のペースで新しい API に移行できる、というものです。

## 6.3 SCM 型

Guile は、すべての Scheme 値を単一の C 型 `SCM` で表現します。このトピックの入門については、「動的型」を参照してください。

**C 型: `SCM`**
: `SCM` は、Scheme オブジェクトの型が何であれ、Guile のすべての Scheme オブジェクトを表現するために使われる、ユーザーレベルの抽象的な C 型です。`SCM` 型の変数に対しては、代入以外のいかなる C の操作も動作することが保証されていないため、`SCM` 値を扱うにはマクロと関数だけを使うべきです。値は、ユーティリティ関数とマクロを使って C のデータ型と `SCM` 型の間で変換されます。

**C 型: `scm_t_bits`**
: `scm_t_bits` は、任意の Scheme オブジェクトを表現するのに必要なすべての情報を保持するのに十分な大きさであることが保証されている、符号なし整数のデータ型です。このデータ型は主に Guile の内部を実装するために使われますが、Guile に対するある種の拡張を書くためにもこの型を使う必要があります。

**C 型: `scm_t_signed_bits`**
: これは `scm_t_bits` と同じサイズの符号付き整数型です。

**C マクロ: `scm_t_bits SCM_UNPACK (SCM x)`**
: `SCM` 値 `x` を、整数型としての表現に変換します。`SCM_UNPACK` を適用した後でのみ、`SCM` 値のビットと内容にアクセスできます。

**C マクロ: `SCM SCM_PACK (scm_t_bits x)`**
: Scheme オブジェクトの有効な整数表現を受け取り、それを `SCM` 値としての表現に変換します。

## 6.4 Guile の初期化

Guile API の関数を使いたい各スレッドは、`scm_with_guile` または `scm_init_guile` のどちらかで自分自身を guile モードにする必要があります。Guile のグローバルな状態は、最初のスレッドが guile モードに入ったときに自動的に初期化されます。

スレッドが Guile API 関数の外でブロックしたい場合は、`scm_without_guile` で一時的に guile モードを離れるべきです。「Guile モードでのブロッキング」を参照してください。

`call-with-new-thread` または `scm_spawn_thread` によって作成されたスレッドは guile モードで開始するので、それらを初期化する必要はありません。

**C 関数: `void * scm_with_guile (void *(*func)(void *), void *data)`**
: `func` を呼び出して `data` を渡し、`func` が返したものを返します。`func` の実行中、現在のスレッドは guile モードにあり、したがって Guile API を使うことができます。

  `scm_with_guile` が guile モードから呼び出された場合、`scm_with_guile` が戻ったときもスレッドは guile モードのままです。

  そうでなければ、現在のスレッドを guile モードにし、必要であれば、たとえば `all-threads` が返すリストに含まれる Scheme での表現をスレッドに与えます。この Scheme での表現は `scm_with_guile` が戻っても削除されないので、あるスレッドは、表現されるとすれば、その生存期間中は常に同じ Scheme 値によって表現されます。

  これが guile モードに入る最初のスレッドである場合、`func` を呼び出す前に Guile のグローバルな状態が初期化されます。

  関数 `func` は `scm_with_continuation_barrier` を介して呼び出されます。したがって、`scm_with_guile` はちょうど1回だけ戻ります。

  `scm_with_guile` が戻ると、スレッドはもはや guile モードではありません（`scm_with_guile` が guile モードから呼び出された場合を除く。上記参照）。したがって、スタック上に `SCM` 変数を格納して、それがガベージコレクタから保護されていると確信できるのは `func` だけです。この制限のない、Guile を初期化する別のアプローチについては `scm_init_guile` を参照してください。

  スレッドが `scm_without_guile` によって一時的に guile モードを離れている間に `scm_with_guile` を呼び出してもかまいません。その場合は、単に一時的に再び guile モードに入ります。

**C 関数: `void scm_init_guile ()`**
: 現在のスレッド内のすべてのコードが、`scm_with_guile` の呼び出しの内側からであるかのように実行されるよう手配します。つまり、現在のスレッドによって呼び出されるすべての関数は、自分のスタックフレーム上の `SCM` 値がガベージコレクタから保護されていると想定できます（もちろん、スレッドが明示的に guile モードを離れている場合を除く）。

  すでに一度 guile モードになったことのあるスレッドから `scm_init_guile` が呼び出された場合は、何も起こりません。この動作は、スレッドが一時的に guile モードを離れているだけのときに `scm_init_guile` を呼び出す場合に問題になります。その場合、`scm_init_guile` が戻った後、スレッドは guile モードになっていません。したがって、そのようなシナリオでは `scm_init_guile` を使うべきではありません。

  `scm_init_guile` によって guile モードにされたスレッドで捕捉されない throw が起こると、現在のエラーポートに短いメッセージが表示され、スレッドは `scm_pthread_exit (NULL)` によって終了します。継続には何の制限も課されません。

  関数 `scm_init_guile` は、Guile が動作するすべてのプラットフォームに移植されているとは限らない、スタックの境界を見つけるための魔法を必要とするため、すべてのプラットフォームで利用できるとは限りません。したがって、可能であれば、この関数の代わりに `scm_with_guile` またはその変形である `scm_boot_guile` を使うほうがよいでしょう。

**C 関数: `void scm_boot_guile (int argc, char **argv, void (*main_func) (void *data, int argc, char **argv), void *data)`**
: `scm_with_guile` と同様に guile モードに入り、示されたとおりに `data`、`argc`、`argv` を渡して `main_func` を呼び出します。`main_func` が戻ると、`scm_boot_guile` は `exit (0)` を呼び出します。`scm_boot_guile` は決して戻りません。別の終了値が必要な場合は、`main_func` 自身に `exit` を呼び出させてください。まったく終了したくない場合は、`scm_boot_guile` の代わりに `scm_with_guile` を使ってください。

  関数 `scm_boot_guile` は、Scheme の `command-line` 関数が `argc` と `argv` で与えられた文字列を返すよう手配します。`main_func` が `argc` または `argv` を変更する場合は、最終的なリストを付けて `scm_set_program_arguments` を呼び出し、どの引数が処理されたかを Scheme コードが分かるようにすべきです（「実行時環境」を参照）。

**C 関数: `void scm_shell (int argc, char **argv)`**
: `guile` 実行ファイルと同じ方法でコマンドライン引数を処理します。これには、通常の Guile 初期化ファイルの読み込み、ユーザーとの対話、あるいは `-s` または `-e` オプションで指定されたスクリプトや式の実行、そしてその後の終了が含まれます。詳細については「Guile の起動」を参照してください。

  この関数は戻らないので、アプリケーション固有の初期化はすべてこの関数を呼び出す前に行わなければなりません。

## 6.5 スナーフィングマクロ

以下のマクロは2つの異なることを行います。通常どおりコンパイルされたときはある方法で展開され、スナーフィング中に処理されたときは `guile-snarf` プログラムに何らかの初期化コードを拾わせます。「関数のスナーフィング」を参照してください。

以下の説明では、コードが通常どおりコンパイルされる場合を指して「通常は」という用語を、コードが `guile-snarf` によって処理される場合を指して「スナーフィング中は」という用語を使います。

**C マクロ: `SCM_SNARF_INIT (code)`**
: 通常は、`SCM_SNARF_INIT` は何にも展開されません。スナーフィング中は、`code` を、その後にセミコロンを付けて、初期化処理ファイルに含めさせます。

  これは初期化処理をスナーフィングするための基本的なマクロです。以下のより特化したマクロは、内部でこれを使っています。

**C マクロ: `SCM_DEFINE (c_name, scheme_name, req, opt, var, arglist, docstring)`**
: 通常は、このマクロは次のように展開されます。

  ```c
  static const char s_c_name[] = scheme_name;
  SCM
  c_name arglist
  ```

  スナーフィング中は、次のものを初期化処理に追加させます。

  ```c
  scm_c_define_gsubr (s_c_name, req, opt, var,
                      c_name);
  ```

  したがって、これを使って、`scheme_name` という名前で Scheme から利用可能になる `c_name` という名前の C 関数を宣言できます。

  `arglist` 引数は括弧で囲まなければならないことに注意してください。

**C マクロ: `SCM_SYMBOL (c_name, scheme_name)`**<br>**C マクロ: `SCM_GLOBAL_SYMBOL (c_name, scheme_name)`**
: 通常は、これらのマクロはそれぞれ次のように展開されます。

  ```c
  static SCM c_name
  ```

  または

  ```c
  SCM c_name
  ```

  スナーフィング中は、どちらも次の初期化コードに展開されます。

  ```c
  c_name = scm_permanent_object (scm_from_locale_symbol (scheme_name));
  ```

  したがって、これらを使って、`scheme_name` という名前のシンボルに初期化される `SCM` 型の静的変数またはグローバル変数を宣言できます。

**C マクロ: `SCM_KEYWORD (c_name, scheme_name)`**<br>**C マクロ: `SCM_GLOBAL_KEYWORD (c_name, scheme_name)`**
: 通常は、これらのマクロはそれぞれ次のように展開されます。

  ```c
  static SCM c_name
  ```

  または

  ```c
  SCM c_name
  ```

  スナーフィング中は、どちらも次の初期化コードに展開されます。

  ```c
  c_name = scm_permanent_object (scm_c_make_keyword (scheme_name));
  ```

  したがって、これらを使って、`scheme_name` という名前のキーワードに初期化される `SCM` 型の静的変数またはグローバル変数を宣言できます。

**C マクロ: `SCM_VARIABLE (c_name, scheme_name)`**<br>**C マクロ: `SCM_GLOBAL_VARIABLE (c_name, scheme_name)`**
: これらのマクロは、値を `SCM_BOOL_F` とした `SCM_VARIABLE_INIT` および `SCM_GLOBAL_VARIABLE_INIT` とそれぞれ同等です。

**C マクロ: `SCM_VARIABLE_INIT (c_name, scheme_name, value)`**<br>**C マクロ: `SCM_GLOBAL_VARIABLE_INIT (c_name, scheme_name, value)`**
: 通常は、これらのマクロはそれぞれ次のように展開されます。

  ```c
  static SCM c_name
  ```

  または

  ```c
  SCM c_name
  ```

  スナーフィング中は、どちらも次の初期化コードに展開されます。

  ```c
  c_name = scm_permanent_object (scm_c_define (scheme_name, value));
  ```

  したがって、これらを使って、現在のモジュールにある `scheme_name` という名前の Scheme 変数を表すオブジェクトに初期化される、`SCM` 型の静的またはグローバルな C 変数を宣言できます。その変数は、まだ存在しない場合は定義されます。常に `value` に設定されます。

---

> **ライセンス**: この翻訳は GNU Free Documentation License v1.3 以降に基づいて作成されています。
> 原文の著作権: Copyright (C) 1996-2023 Free Software Foundation, Inc.
