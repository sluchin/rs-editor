### 7.15 バッファリングされた入力 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Buffered-Input-1)

以下の機能は以下によって提供されます

(use-modules (ice-9 buffered-input))

バッファリングされた入力ポートを使用すると、読み取り関数はポートを読み取る際に、文字のチャンクをまとめて返すことができます。また、アプリケーションレベルの論理式に対する追加入力の概念も保持され、読み取り関数に渡されます。

Scheme Procedure: **make-buffered-input-port** リーダー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dbuffered_002dinput_002dport)

指定されたリーダー関数から取得した文字を返す入力ポートを作成します。リーダーは（リーダーcont）と呼ばれ、文字列またはEOFオブジェクトを返す必要があります。

新しいポートは、リーダーが返す文字をそのまま取得し、何も追加しません。したがって、改行文字やその他の区切り文字が必要な場合は、リーダー関数から取得する必要があります。

リーダーへのcontパラメータは、初期入力の場合は`#f`、式の続きを入力する場合は`#t`です。これはアプリケーションレベルの概念であり、以下の`set-buffered-input-continuation?!`で設定します。ユーザーが部分的な式を入力した場合、リーダーは例えば、続きの入力が必要であることを示す別のプロンプトを表示することができます。

Scheme Procedure: **make-line-buffered-input-port** リーダー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dline_002dbuffered_002dinput_002dport)

指定されたリーダー関数から取得した文字を返す入力ポートを作成します。これは上記の`make-buffered-input-port`と同様ですが、リーダーは行指向であることが想定されています。

リーダーが呼び出され（リーダー続き）、上記のように文字列またはEOFオブジェクトを返します。各文字列は改行文字を含まない入力行であり、ポートコードは各文字列の後に改行を挿入します。

Scheme Procedure: **set-buffered-input-continuation?!** port cont [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-set_002dbuffered_002dinput_002dcontinuation_003f_0021)

指定されたバッファ付き入力ポートの入力継続フラグを設定します。

アプリケーションは、新しい論理式の読み込みを開始する際に、cont フラグを `#f` として呼び出すことでこれを使用します。たとえば、Scheme の `read` 関数 ([Reading Scheme Code](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scheme-Read) を参照) では、

(define my-port (make-buffered-input-port my-reader))

(set-buffered-input-continuation?! my-port #f)
(let ((obj (read my-port)))
...

* * *

次へ: [`sxml-match`: SXML のパターン マッチング](https://doc.guix.gnu.org/guile/latest/en/guile.html#sxml_002dmatch)、前: [Buffered Input](https://doc.guix.gnu.org/guile/latest/en/guile.html#Buffered-Input)、上: [Guile モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guile-Modules) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
