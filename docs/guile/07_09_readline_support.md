### 7.9 Readline サポート

Guileにはreadlineライブラリへのインターフェースモジュールが付属しています（[GNU Readlineライブラリ](https://tiswww.cwru.edu/php/chet/readline/readline.html#Top)を参照）。readlineのコマンドライン編集機能のおかげで、対話型の使用がはるかに便利になります。`(ice-9 readline)`を使用すると、カーソルキーで現在の入力行を移動したり、入力履歴から以前のコマンドラインを取得したり、履歴エントリを検索したりできます。

* [Readline サポートの読み込み](#791-readline-サポートの読み込み)
* [Readline オプション](#792-readline-オプション)
* [Readline 関数](#793-readline-関数)

* * *

次へ: [Readline オプション](#792-readline-オプション)、上へ: [Readline サポート](#79-readline-サポート) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.9.1 Readline サポートの読み込み

このモジュールはデフォルトではロードされないため、明示的にロードして有効化する必要があります。これは、以下の2行の簡単なコードで実現できます。

([use-modules](06_18_modules.md#6182-guileモジュールの使用) (ice-9 [readline](#793-readline-関数)))
([activate-readline](#7931-readline-ポート))

最初の行は必要なコードを読み込み、2行目はREPL用のreadlineの機能を有効にします。このモジュールを頻繁に使用する予定がある場合は、これらの2行を.guileの個人用起動ファイルに保存することをお勧めします。

readlineモジュールをロードすると、REPLの動作が少し変わることに気づくでしょう。たとえば、リストの閉じ括弧を入力する前にEnterキーを押すと、_継続_プロンプト（3つのドット：`...`）が表示されます。これにより、括弧を一致させようとする際に、視覚的に分かりやすいフィードバックが得られます。さらに簡単にするために、_バウンス括弧_が実装されています。つまり、閉じ括弧を入力すると、カーソルが対応する開き括弧に短時間ジャンプするため、簡単に一致させることができます。

readlineモジュールが有効化されると、対話的に入力されたすべての行が履歴に保存され、後でカーソルキー（上矢印と下矢印）を使って呼び出すことができます。readlineは、コマンドラインと履歴をナビゲートするためのEmacsのキー操作にも対応しています。

`(quit)` を評価するか Ctrl+D を押して Guile セッションを終了すると、履歴は .guile\_history ファイルに保存され、次回 Guile を起動する際に読み込まれます。そのため、新しい Guile セッションを開始しても、（おそらく長々とした）定義式を引き続き利用できます。

環境変数 `GUILE_HISTORY` を設定することで、別の履歴ファイルを指定できます。また、アプリケーション 'Guile' をテストすることで、.inputrc に Guile 固有のカスタマイズを加えることができます (GNU Readline ライブラリの [Conditional Init Constructs](https://tiswww.cwru.edu/php/chet/readline/readline.html#Conditional-Init-Constructs) を参照)。たとえば、対応する括弧のペアを挿入するキーを定義するには、

$if ガイル
"\\Co": "()\\Cb"
$endif

* * *

次へ: [Readline 関数](#793-readline-関数)、前: [Readline サポートの読み込み](#791-readline-サポートの読み込み)、上: [Readline サポート](#79-readline-サポート) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.9.2 Readline オプション

readlineインターフェースモジュールは、ユーザーのニーズに合わせていくつかの方法で調整できます。設定は、評価オプションやデバッグオプションと同様に、readlineモジュールのオプションインターフェースを介して行います（[ランタイムオプション](06_23_configuration_features_and_runtime_options.md#6233-ランタイムオプション)を参照）。

Scheme Procedure: **readline-options**

Scheme Procedure: **readline-enable** option-name

スキーム手順: **readline-disable** オプション名

Scheme構文: **readline-set!** オプション名 値

readlineオプションのアクセサ。有効化/無効化の手順とは異なり、`readline-set!`は構文であり、引用符で囲まれていないオプション名を想定していることに注意してください。

以下は、Guileで`(readline-options 'help)`と入力して生成されたreadlineオプションの一覧です。デフォルト値も確認できます。

history-file yes 履歴ファイルを使用します。
history-length 200 履歴の長さ。
括弧を表示するまでの時間 (ms)
(0 [\=](06_06_02_numerical_data_types.md#6628-比較述語) off)
括弧付き貼り付け はい 制御文字の解釈を無効にする
[in](04_programming_in_scheme.md#4442-モジュールコマンド) は貼り付けます。

readline オプション インターフェースは、readline モジュールをロードした後でのみ使用できます。なぜなら、このインターフェースはそのモジュール内で定義されているからです。

* * *

前へ: [Readline オプション](#792-readline-オプション)、上へ: [Readline サポート](#79-readline-サポート) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.9.3 Readline 関数

以下の機能は以下によって提供されます

(use-modules (ice-9 readline))

Schemeコードからreadlineを使用する方法は2つあります。1つは`readline`を直接呼び出して1行ずつ入力を取得する方法、もう1つは通常の読み取り機能をすべて備えた以下のreadlineポートを使用する方法です。

関数: **readline** \[prompt\]

ユーザーからの入力を1行読み込み、文字列として返します（末尾に改行は含まれません）。promptは表示するプロンプトです。デフォルトでは、以下の`set-readline-prompt!`で設定された文字列が使用されます。

(readline "何か入力してください: ") ⇒ "こんにちは"

機能: **set-readline-input-port!** ポート

機能: **set-readline-output-port!** ポート

readline 関数が読み書きする入力ポートと出力ポートを設定します。port はファイルポートである必要があります ([ファイルポート](06_12_input_and_output.md#612101-ファイルポート) を参照)。通常は端末ポートである必要があります。

デフォルトでは、`(ice-9 readline)` がロードされたときに `current-input-port` と `current-output-port` ([入力、出力、エラーのデフォルトポート](06_12_input_and_output.md#6129-入力出力およびエラーのデフォルトポート) を参照) が使用されます。これは、対話型ユーザーセッションでは Unix の「標準入力」と「標準出力」を意味します。

* [Readlineポート](#7931-readline-ポート)
* [完了](#7932-完了)

#### 7.9.3.1 Readline ポート

機能: **readline-port**

入力を取得するために上記の `readline` 関数を呼び出すバッファリングされた入力ポートを返します ([バッファリングされた入力](07_15_buffered_input.md#715-バッファリングされた入力) を参照)。このポートは、通常の読み取り関数 (`read`、`read-char` など) すべてで使用でき、ユーザーは readline の対話型編集機能を利用できます。

作成されるreadlineポートは1つだけです。`readline-port`は最初に呼び出されたときにポートを作成し、それ以降の呼び出しでは以前に作成したポートを返します。

機能: **activate-readline**

`current-input-port` が端末である場合 ([`isatty?`](07_02_09_terminals_and_ptys.md#729-端末とpty) を参照)、`current-input-port` からのすべての読み取りに対して readline を有効にし ([Default Ports for Input, Output and Errors](06_12_input_and_output.md#6129-入力出力およびエラーのデフォルトポート) を参照)、対話型 REPL で readline 機能を有効にします ([Using the Guile REPL](03_hello_scheme.md#333-guile-repl-の使用) を参照)。

(activate-readline)
(文字読み込み)

`activate-readline` は、上記の `set-current-input-port` で `readline-port` を指定するだけで、`current-input-port` で readline を有効にします。`activate-readline` によって追加される REPL の機能が不要な場合は、アプリケーション側で直接この操作を行うこともできます。

関数: **set-readline-prompt!** prompt1 \[prompt2\]

入力読み込み時に表示されるプロンプト文字列を設定します。これは`readline-port`を介して読み込む際に使用され、上記の`readline`関数のデフォルトのプロンプトでもあります。

prompt1 は最初に表示されるプロンプトです。ユーザーが複数行にわたって式を入力する可能性がある場合、prompt2 は追加の入力が必要であることを示す別のプロンプトです。たとえば、Guile REPL では、これは省略記号 ('...') です。

論理式の境界を示すアプリケーションについては、`set-buffered-input-continuation?!`（[Buffered Input](07_15_buffered_input.md#715-バッファリングされた入力)を参照）を参照してください（もちろん、アプリケーションにそのような概念があることを前提としています）。

#### 7.9.3.2 完了

機能: **with-readline-completion-function** 補完サンク

`(thunk)` を呼び出す際に、`completer` に readline タブ補完関数を指定します。この補完関数は、その thunk 内の readline 呼び出しで使用されます。補完を行わない場合は、`#f` を指定できます。

completer は、GNU Readline ライブラリの [How Completing Works](https://tiswww.cwru.edu/php/chet/readline/readline.html#How-Completing-Works) で説明されているように、`(completer text state)` として呼び出されます。text は補完される部分的な単語であり、各 completer 呼び出しは、可能な補完文字列を返すか、それ以上ない場合は `#f` を返す必要があります。state は、新しいテキストについて尋ねる最初の呼び出しでは `#f` であり、そのテキストのさらなる補完を取得する際には `#t` になります。

パスワードファイルからユーザーログイン名を補完する例を以下に示します（[ユーザー情報](07_02_04_user_information.md#724-ユーザー情報)を参照）。これはreadlineの`rl_username_completion_function`とよく似ています。

(define (username-completer-function text state)
(状態ではない場合)
(setpwent)) ;; 新規、データベースの先頭へ移動
(let more ((pw (getpwent)))
(pw の場合)
(if (string-prefix? text (passwd:name pw))
(passwd:name pw) ;; この名前が一致する場合は、それを返します
(more (getpwent))) ;; 一致しません、次を見てください
（始める
;; データベースの終了、閉じて#fを返す
(終了)
#f))))

機能: **apropos-completion-function** テキスト状態 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- apropos_002dcompletion_002dfunction)

Guileの関数と変数（すべての`define`）の補完機能を提供する補完関数です。これはデフォルトの補完関数です。

機能: **ファイル名補完機能** テキスト状態

ファイル名の補完機能を提供する補完関数。これは、readline の `rl_filename_completion_function` です（GNU Readline ライブラリの [補完関数](https://tiswww.cwru.edu/php/chet/readline/readline.html#Completion-Functions) を参照）。

機能: **make-completion-function** string-list

文字列リストに含まれる候補の中から補完候補を提示する補完関数を返します。大文字と小文字は区別されます。

* * *

次へ: [フォーマット出力](07_11_formatted_output.md#711-フォーマットされた出力)、前: [Readline サポート](#79-readline-サポート)、上: [Guile モジュール](07_00_guile_modules.md#7つのguileモジュール) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
