### 7.9 Readline サポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Readline-Support-1)

Guileにはreadlineライブラリへのインターフェースモジュールが付属しています（[GNU Readlineライブラリ](https://tiswww.cwru.edu/php/chet/readline/readline.html#Top)を参照）。readlineのコマンドライン編集機能のおかげで、対話型の使用がはるかに便利になります。`(ice-9 readline)`を使用すると、カーソルキーで現在の入力行を移動したり、入力履歴から以前のコマンドラインを取得したり、履歴エントリを検索したりできます。

* [Readline サポートの読み込み](https://doc.guix.gnu.org/guile/latest/en/guile.html#Loading-Readline-Support)
* [Readline オプション](https://doc.guix.gnu.org/guile/latest/en/guile.html#Readline-Options)
* [Readline 関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Readline-Functions)

* * *

次へ: [Readline オプション](https://doc.guix.gnu.org/guile/latest/en/guile.html#Readline-Options)、上へ: [Readline サポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Readline-Support) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.9.1 Readline サポートの読み込み [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Loading-Readline-Support-1)

このモジュールはデフォルトではロードされないため、明示的にロードして有効化する必要があります。これは、以下の2行の簡単なコードで実現できます。

([use-modules](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-use_002dmodules) (ice-9 [readline](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-readline-1)))
([activate-readline](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-activate_002dreadline))

最初の行は必要なコードを読み込み、2行目はREPL用のreadlineの機能を有効にします。このモジュールを頻繁に使用する予定がある場合は、これらの2行を.guileの個人用起動ファイルに保存することをお勧めします。

readlineモジュールをロードすると、REPLの動作が少し変わることに気づくでしょう。たとえば、リストの閉じ括弧を入力する前にEnterキーを押すと、_継続_プロンプト（3つのドット：`...`）が表示されます。これにより、括弧を一致させようとする際に、視覚的に分かりやすいフィードバックが得られます。さらに簡単にするために、_バウンス括弧_が実装されています。つまり、閉じ括弧を入力すると、カーソルが対応する開き括弧に短時間ジャンプするため、簡単に一致させることができます。

readlineモジュールが有効化されると、対話的に入力されたすべての行が履歴に保存され、後でカーソルキー（上矢印と下矢印）を使って呼び出すことができます。readlineは、コマンドラインと履歴をナビゲートするためのEmacsのキー操作にも対応しています。

`(quit)` を評価するか Ctrl+D を押して Guile セッションを終了すると、履歴は .guile\_history ファイルに保存され、次回 Guile を起動する際に読み込まれます。そのため、新しい Guile セッションを開始しても、（おそらく長々とした）定義式を引き続き利用できます。

環境変数 `GUILE_HISTORY` を設定することで、別の履歴ファイルを指定できます。また、アプリケーション 'Guile' をテストすることで、.inputrc に Guile 固有のカスタマイズを加えることができます (GNU Readline ライブラリの [Conditional Init Constructs](https://tiswww.cwru.edu/php/chet/readline/readline.html#Conditional-Init-Constructs) を参照)。たとえば、対応する括弧のペアを挿入するキーを定義するには、

$if ガイル
"\\Co": "()\\Cb"
$endif

* * *

次へ: [Readline 関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Readline-Functions)、前: [Readline サポートの読み込み](https://doc.guix.gnu.org/guile/latest/en/guile.html#Loading-Readline-Support)、上: [Readline サポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Readline-Support) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.9.2 Readline オプション [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Readline-Options-1)

readlineインターフェースモジュールは、ユーザーのニーズに合わせていくつかの方法で調整できます。設定は、評価オプションやデバッグオプションと同様に、readlineモジュールのオプションインターフェースを介して行います（[ランタイムオプション](https://doc.guix.gnu.org/guile/latest/en/guile.html#Runtime-Options)を参照）。

Scheme Procedure: **readline-options** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-readline_002doptions)

Scheme Procedure: **readline-enable** option-name [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-readline_002denable)

スキーム手順: **readline-disable** オプション名 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-readline_002ddisable)

Scheme構文: **readline-set!** オプション名 値 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-readline_002dset_0021)

readlineオプションのアクセサ。有効化/無効化の手順とは異なり、`readline-set!`は構文であり、引用符で囲まれていないオプション名を想定していることに注意してください。

以下は、Guileで`(readline-options 'help)`と入力して生成されたreadlineオプションの一覧です。デフォルト値も確認できます。

history-file yes 履歴ファイルを使用します。
history-length 200 履歴の長さ。
括弧を表示するまでの時間 (ms)
(0 [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) off)
括弧付き貼り付け はい 制御文字の解釈を無効にする
[in](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-in) は貼り付けます。

readline オプション インターフェースは、readline モジュールをロードした後でのみ使用できます。なぜなら、このインターフェースはそのモジュール内で定義されているからです。

* * *

前へ: [Readline オプション](https://doc.guix.gnu.org/guile/latest/en/guile.html#Readline-Options)、上へ: [Readline サポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Readline-Support) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.9.3 Readline 関数 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Readline-Functions-1)

以下の機能は以下によって提供されます

(use-modules (ice-9 readline))

Schemeコードからreadlineを使用する方法は2つあります。1つは`readline`を直接呼び出して1行ずつ入力を取得する方法、もう1つは通常の読み取り機能をすべて備えた以下のreadlineポートを使用する方法です。

関数: **readline** \[prompt\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-readline-1)

ユーザーからの入力を1行読み込み、文字列として返します（末尾に改行は含まれません）。promptは表示するプロンプトです。デフォルトでは、以下の`set-readline-prompt!`で設定された文字列が使用されます。

(readline "何か入力してください: ") ⇒ "こんにちは"

機能: **set-readline-input-port!** ポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-set_002dreadline_002dinput_002dport_0021)

機能: **set-readline-output-port!** ポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-set_002dreadline_002doutput_002dport_0021)

readline 関数が読み書きする入力ポートと出力ポートを設定します。port はファイルポートである必要があります ([ファイルポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#File-Ports) を参照)。通常は端末ポートである必要があります。

デフォルトでは、`(ice-9 readline)` がロードされたときに `current-input-port` と `current-output-port` ([入力、出力、エラーのデフォルトポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Default-Ports) を参照) が使用されます。これは、対話型ユーザーセッションでは Unix の「標準入力」と「標準出力」を意味します。

* [Readlineポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Readline-Port)
* [完了](https://doc.guix.gnu.org/guile/latest/en/guile.html#Completion)

#### 7.9.3.1 Readline ポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Readline-Port)

機能: **readline-port** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-readline_002dport)

入力を取得するために上記の `readline` 関数を呼び出すバッファリングされた入力ポートを返します ([バッファリングされた入力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Buffered-Input) を参照)。このポートは、通常の読み取り関数 (`read`、`read-char` など) すべてで使用でき、ユーザーは readline の対話型編集機能を利用できます。

作成されるreadlineポートは1つだけです。`readline-port`は最初に呼び出されたときにポートを作成し、それ以降の呼び出しでは以前に作成したポートを返します。

機能: **activate-readline** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-activate_002dreadline)

`current-input-port` が端末である場合 ([`isatty?`](https://doc.guix.gnu.org/guile/latest/en/guile.html#Terminals-and-Ptys) を参照)、`current-input-port` からのすべての読み取りに対して readline を有効にし ([Default Ports for Input, Output and Errors](https://doc.guix.gnu.org/guile/latest/en/guile.html#Default-Ports) を参照)、対話型 REPL で readline 機能を有効にします ([Using the Guile REPL](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-REPL) を参照)。

(activate-readline)
(文字読み込み)

`activate-readline` は、上記の `set-current-input-port` で `readline-port` を指定するだけで、`current-input-port` で readline を有効にします。`activate-readline` によって追加される REPL の機能が不要な場合は、アプリケーション側で直接この操作を行うこともできます。

関数: **set-readline-prompt!** prompt1 \[prompt2\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-set_002dreadline_002dprompt_0021)

入力読み込み時に表示されるプロンプト文字列を設定します。これは`readline-port`を介して読み込む際に使用され、上記の`readline`関数のデフォルトのプロンプトでもあります。

prompt1 は最初に表示されるプロンプトです。ユーザーが複数行にわたって式を入力する可能性がある場合、prompt2 は追加の入力が必要であることを示す別のプロンプトです。たとえば、Guile REPL では、これは省略記号 ('...') です。

論理式の境界を示すアプリケーションについては、`set-buffered-input-continuation?!`（[Buffered Input](https://doc.guix.gnu.org/guile/latest/en/guile.html#Buffered-Input)を参照）を参照してください（もちろん、アプリケーションにそのような概念があることを前提としています）。

#### 7.9.3.2 完了 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Completion)

機能: **with-readline-completion-function** 補完サンク [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-with_002dreadline_002dcompletion_002dfunction)

`(thunk)` を呼び出す際に、`completer` に readline タブ補完関数を指定します。この補完関数は、その thunk 内の readline 呼び出しで使用されます。補完を行わない場合は、`#f` を指定できます。

completer は、GNU Readline ライブラリの [How Completing Works](https://tiswww.cwru.edu/php/chet/readline/readline.html#How-Completing-Works) で説明されているように、`(completer text state)` として呼び出されます。text は補完される部分的な単語であり、各 completer 呼び出しは、可能な補完文字列を返すか、それ以上ない場合は `#f` を返す必要があります。state は、新しいテキストについて尋ねる最初の呼び出しでは `#f` であり、そのテキストのさらなる補完を取得する際には `#t` になります。

パスワードファイルからユーザーログイン名を補完する例を以下に示します（[ユーザー情報](https://doc.guix.gnu.org/guile/latest/en/guile.html#User-Information)を参照）。これはreadlineの`rl_username_completion_function`とよく似ています。

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

機能: **ファイル名補完機能** テキスト状態 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-filename_002dcompletion_002dfunction)

ファイル名の補完機能を提供する補完関数。これは、readline の `rl_filename_completion_function` です（GNU Readline ライブラリの [補完関数](https://tiswww.cwru.edu/php/chet/readline/readline.html#Completion-Functions) を参照）。

機能: **make-completion-function** string-list [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dcompletion_002dfunction)

文字列リストに含まれる候補の中から補完候補を提示する補完関数を返します。大文字と小文字は区別されます。

* * *

次へ: [フォーマット出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Formatted-Output)、前: [Readline サポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Readline-Support)、上: [Guile モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guile-Modules) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
