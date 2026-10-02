#### 7.2.1 POSIX インターフェース規約 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#POSIX-Interface-Conventions)

これらのインターフェースは、オペレーティングシステムの機能へのアクセスを提供します。これらは、基盤となるCインターフェースをシンプルにラップすることで、Schemeからの利用をより便利にします。また、scshのGuileポートを実装するためにも使用されます（[Schemeシェル（scsh）](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Scheme-shell-_0028scsh_0029)を参照）。

一般的に、対応するUnix機能ごとに1つの手順が存在します。ただし、例外もあり、例えば、Unixに同等の基本的な機能がないSchemeの速度と利便性のために実装された手順（例：`copy-file`）などがあります。

インターフェースは、可能な限り異なるバージョンのUnix間で移植性を確保するように設計されています。特定のシステムで実装できない手順は、何もしないか、限定的な動作しか行わない場合があります。また、エラーが発生する場合もあります。

一般的な命名規則は以下のとおりです。

* スキーム名は、基盤となるUnix機能の名前と同一であることが多い。
* Unixのプロシージャ名に含まれるアンダースコアはハイフンに変換されます。
* Scheme データを破壊的に変更する手順には、感嘆符が付加されます。例: `recv!`。
* 述語 (`#t` または `#f` のみを返す) には疑問符が付加されます (例: `access?`)。
* scsh で定義されている異なるインターフェースとの競合を避けるために、一部の名前が変更されています (例: `primitive-fork`)。
* `EPERM` や `R_OK` などの Unix プリプロセッサ名は、同じ名前の Scheme 変数に変換されます (アンダースコアはハイフンに置き換えられません)。

予期しない状況は、通常、例外を発生させることで処理されます。一部のプロシージャは、処理が成功しなかった場合に特別な値を返します。例えば、`getenv` は、要求された文字列が環境に見つからない場合に `#f` を返します。これらのケースについては、ドキュメントに記載されています。

例外の処理方法については、[例外](https://doc.guix.gnu.org/guile/latest/en/guile.html#Exceptions)を参照してください。

Cライブラリがヌルポインタを返すか、その他の方法で報告するエラーは、`scm-error`を使用して`system-error`例外を発生させることで報告されます（[エラー通知の手順](https://doc.guix.gnu.org/guile/latest/en/guile.html#Error-Reporting)を参照）。データパラメータは、Unixの`errno`値（整数）を含むリストです。たとえば、

(define (my-handler key func fmt fmtargs data)
（表示キー）（改行）
(表示機能) (改行)
(フォーマット#t fmt fmtargsを適用) (改行)
（データ表示）（改行）

(catch 'system-error
(lambda () (dup2 -123 -456))
マイハンドラー)

⊣
システムエラー
重複2
不正なファイルディスクリプタ
（9）

  

関数: **system-error-errno** 引数リスト[¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-system_002derror_002derrno)

例外ハンドラの引数であるリストから `errno` 値を返します。例外が `system-error` でない場合は、`#f` が返されます。例:

（キャッチ
システムエラー
(ラムダ()
(mkdir "/this-ought-to-fail-if-I'm-not-root"))
(ラムダ関連)
(let ((errno (system-error-errno stuff)))
（条件）
((= errno EACCES)
（「それは許可されていません。」と表示する）
((= errno EEXIST)
(「既に存在します。」と表示)
(#t
(display (strerror errno))))
(改行))))

* * *

次へ: [ファイルシステム](https://doc.guix.gnu.org/guile/latest/en/guile.html#File-System)、前: [POSIX インターフェース規約](https://doc.guix.gnu.org/guile/latest/en/guile.html#Conventions)、上: [POSIX システムコールとネットワーク](https://doc.guix.gnu.org/guile/latest/en/guile.html#POSIX) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
