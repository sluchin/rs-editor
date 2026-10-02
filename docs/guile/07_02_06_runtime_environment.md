#### 7.2.6 ランタイム環境 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Runtime-Environment-1)

Scheme手順: **プログラム引数** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-program_002darguments)

Scheme手順: **コマンドライン** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-command_002dline)

Scheme手順: **set-program-arguments** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-set_002dprogram_002darguments)

C 関数: **scm\_program\_arguments** () [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fprogram_005farguments)

C 関数: **scm\_set\_program\_arguments\_scm** (lst) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fset_005fprogram_005farguments_005fscm)

Guileに渡されるコマンドライン引数を取得するか、新しい引数を設定します。

引数は文字列のリストで、最初の文字列は呼び出されるプログラム名です。対話的に実行する場合は単に「guile」（または実行可能ファイルのパス）となり、-s オプションでスクリプトを実行する場合はスクリプト名となります（[Guile の呼び出し](https://doc.guix.gnu.org/guile/latest/en/guile.html#Invoking-Guile) を参照）。

guile -L /my/extra/dir -s foo.scm abc def

(プログラム引数) ⇒ ("foo.scm" "abc" "def")

`set-program-arguments` を使用すると、ライブラリモジュールなどが引数を変更できます。たとえば、認識するオプションを削除し、残りをメインラインに残すことができます。

引数リストは流動的な構造で保持されるため、各スレッドごとに独立しています。リスト自体も、リスト内の文字列も、いかなる時点でもコピーされることはなく、通常は変更されるべきではありません。

`program-arguments`と`command-line`という2つの名前は、歴史的な偶然によるもので、どちらも全く同じ機能を持っています。`scm_set_program_arguments_scm`という名前には、後述のC言語関数との衝突を避けるため、末尾に`_scm`が追加されています。

C 関数: `void` **scm\_set\_program\_arguments** `(int argc, char **argv, char *first)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fset_005fprogram_005farguments)

上記で、`program-arguments`と`command-line`にコマンドライン引数のリストを設定してください。

argv は、C 言語の `main` 関数のように、ヌル終端文字列の配列です。argc は argv に含まれる文字列の数です。もし argc が負の値の場合は、argv 内の `NULL` が文字列の末尾を示します。

first は引数の先頭に追加される文字列、またはそのような追加がない場合は `NULL` です。これは、オプション引数を削除するために argv を進めた後にプログラム名を渡す便利な方法です。例:

{
char *progname = argv[0];
for (argv++; argv\[0\] != NULL && argv\[0\]\[0\] == '-'; argv++)
{
/\* マンチオプション... \*/
}
/\* スキームレベルで使用するための残りの引数 \*/
scm_set_program_arguments (\-1, argv, progname);
}

このような処理は、起動時に`scm_boot_guile`で行われることが多く、Cレベルで処理されるオプションは削除されます。指定された文字列はすべてコピーされるため、`scm_set_program_arguments`が戻るとCデータは再度アクセスされません。

スキームプロシージャ: **getenv** 名前 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-getenv)

C 関数: **scm\_getenv** (名前) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fgetenv)

現在の環境内で文字列名を検索します。戻り値は`#f`ですが、`NAME=VALUE`の形式の文字列が見つかった場合は、文字列`VALUE`が返されます。

Scheme手順: **setenv** 名前 値 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-setenv)

現在のプロセスの環境を変更します。この環境は、子プロセスが継承するデフォルトの環境でもあります。

値が`#f`の場合、環境からnameが削除されます。それ以外の場合は、文字列name\=valueが環境に追加され、nameに一致する既存の文字列はすべて置き換えられます。

戻り値は指定されていません。

Scheme手順: **unsetenv** name [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-unsetenv)

環境変数から変数名を削除してください。変数名には「\=」文字を含めることはできません。

Scheme手順: **environ** \[env\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-environ)

C 関数: **scm\_environ** (env) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fenviron)

env が省略された場合は、現在の環境 (Unix の意味で) を文字列のリストとして返します。それ以外の場合は、現在の環境 (子プロセスのデフォルト環境でもある) を、指定された文字列のリストに設定します。env の各メンバーは、name\=value の形式である必要があり、name の値は重複してはなりません。env が指定された場合、戻り値は未指定です。

Scheme手順: **putenv** str [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-putenv)

C 関数: **scm\_putenv** (str) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fputenv)

現在のプロセスの環境を変更します。この環境は、子プロセスが継承するデフォルトの環境でもあります。

strが`NAME=VALUE`の形式の場合、strは環境に直接書き込まれ、既存の環境文字列で`NAME`に一致する名前を持つものはすべて置き換えられます。strに等号が含まれていない場合、strに一致する名前を持つ既存の文字列はすべて削除されます。

戻り値は指定されていません。

* * *

次へ: [シグナル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Signals)、前: [ランタイム環境](https://doc.guix.gnu.org/guile/latest/en/guile.html#Runtime-Environment)、上: [POSIX システムコールとネットワーク](https://doc.guix.gnu.org/guile/latest/en/guile.html#POSIX) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
