### 6.4 Guile の初期化

Guile API の機能を使用する各スレッドは、`scm_with_guile` または `scm_init_guile` を使用して Guile モードに移行する必要があります。最初のスレッドが Guile モードに入ると、Guile のグローバル状態が自動的に初期化されます。

スレッドが Guile API 関数の外でブロックしたい場合は、`scm_without_guile` を使用して一時的に Guile モードを終了する必要があります。[Guile モードでのブロッキング](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6226-guile-モードでのブロッキング) を参照してください。

`call-with-new-thread` または `scm_spawn_thread` によって作成されたスレッドは、Guile モードで開始されるため、初期化する必要はありません。

C 関数: `void *` **scm\_with\_guile** `(void *(*func)(void *), void *data)`

関数を呼び出し、データを渡して、関数が返す値を返します。関数が実行されている間、現在のスレッドはGuileモードになり、Guile APIを使用できます。

`scm_with_guile` が guile モードから呼び出された場合、`scm_with_guile` が戻った後もスレッドは guile モードのままです。

そうでない場合は、現在のスレッドをGuileモードにし、必要に応じて、たとえば`all-threads`によって返されるリストに含まれるScheme表現を割り当てます。このScheme表現は`scm_with_guile`が返されても削除されないため、特定のスレッドは、その存続期間中、常に同じScheme値で表現されます（そもそも表現されない場合もあります）。

これがGuileモードに入る最初のスレッドである場合、`func`を呼び出す前にGuileのグローバル状態が初期化されます。

関数 func は `scm_with_continuation_barrier` を介して呼び出されるため、`scm_with_guile` は正確に 1 回だけ戻ります。

`scm_with_guile` が戻ると、スレッドはもはや Guile モードではなくなります (ただし、`scm_with_guile` が Guile モードから呼び出された場合は除きます。上記を参照してください)。したがって、`SCM` 変数をスタックに格納し、ガベージ コレクタから保護されることを保証できるのは `func` だけです。この制限のない Guile の初期化方法については、`scm_init_guile` を参照してください。

スレッドが `scm_without_guile` によって一時的に Guile モードから抜け出している状態でも、`scm_with_guile` を呼び出すことは問題ありません。その場合、スレッドは一時的に再び Guile モードに入ります。

C 関数: `void` **scm\_init\_guile** `()`

現在のスレッド内のすべてのコードが、`scm_with_guile` の呼び出し内から実行されるかのように動作するように設定してください。つまり、現在のスレッドから呼び出されるすべての関数は、スタック フレーム上の `SCM` 値がガベージ コレクタから保護されていると想定できます (もちろん、スレッドが明示的に guile モードを終了した場合を除く)。

`scm_init_guile` が、既に一度 Guile モードになったスレッドから呼び出された場合、何も起こりません。この動作は、スレッドが一時的に Guile モードから抜けた状態で `scm_init_guile` を呼び出す場合に問題となります。この場合、`scm_init_guile` が戻った後、スレッドは Guile モードになりません。したがって、このような状況では `scm_init_guile` を使用しないでください。

`scm_init_guile` によって Guile モードに設定されたスレッドで捕捉されない例外が発生した場合、現在のエラー ポートに短いメッセージが出力され、`scm_pthread_exit (NULL)` によってスレッドが終了します。継続処理には制限はありません。

関数 `scm_init_guile` は、スタック境界を見つけるための特殊な処理を必要とするため、すべてのプラットフォームで利用できるとは限りません。この処理は、Guile が動作するすべてのプラットフォームに移植されているとは限らないためです。したがって、可能であれば、この関数の代わりに `scm_with_guile` またはその派生版である `scm_boot_guile` を使用することをお勧めします。

C 関数: `void` **scm\_boot\_guile** `(int argc, char **argv, void (*main_func) (void *data, int argc, char **argv), void *data)`

`scm_with_guile` と同様に Guile モードに入り、指定されたデータ、argc、argv を渡して main\_func を呼び出します。main\_func が戻ると、`scm_boot_guile` は `exit (0)` を呼び出します。`scm_boot_guile` は決して戻りません。別の終了値が必要な場合は、main\_func 自身で `exit` を呼び出すようにしてください。まったく終了したくない場合は、`scm_boot_guile` の代わりに `scm_with_guile` を使用してください。

関数 `scm_boot_guile` は、Scheme の `command-line` 関数が argc と argv で指定された文字列を返すように設定しています。main\_func が argc または argv を変更する場合は、最終的なリストを使用して `scm_set_program_arguments` を呼び出す必要があります。そうすることで、Scheme コードはどの引数が処理されたかを認識できます ([ランタイム環境](07_02_06_runtime_environment.md#726-ランタイム環境) を参照)。

C 関数: `void` **scm\_shell** `(int argc, char **argv)`

コマンドライン引数を`guile`実行ファイルと同様の方法で処理します。これには、通常のGuile初期化ファイルの読み込み、ユーザーとの対話、`-s`または`-e`オプションで指定されたスクリプトや式の実行、そして終了が含まれます。詳細については、[Guileの起動](04_programming_in_scheme.md#42-guile-の呼び出し)を参照してください。

この関数は値を返さないため、この関数を呼び出す前に、アプリケーション固有の初期化処理をすべて完了しておく必要があります。

* * *

次へ: [データ型](06_06_00_data_types.md#66-データ型)、前: [ Guile の初期化](#64-guile-の初期化)、上: [API リファレンス](06_00_api_reference.md#6-apiリファレンス) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
