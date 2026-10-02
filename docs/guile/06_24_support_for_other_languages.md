### 6.24 他の言語のサポート

Schemeに加えて、ユーザーはますます多くの言語でGuileプログラムを作成できるようになっています。現在サポートされている言語には、Emacs LispとECMAScriptが含まれます。

Guileは依然として基本的にはScheme言語ですが、多様な言語構成要素をサポートするように設計されているため、Guile上に他の言語を実装できます。これにより、ユーザーはScheme以外の言語でアプリケーションを作成したり、拡張したりすることも可能です。このセクションでは、実装されている言語について説明します。

（言語の実装方法の詳細については、[仮想マシンへのコンパイル](09_04_compiling_to_the_virtual_machine.md#94-仮想マシンへのコンパイル)を参照してください。）

* [他の言語の使用](#6241-他の言語の使用)
* [Emacs Lisp](#6242-emacs-lisp)
* [ECMAScript](#6243-ecmascript)

* * *

次へ: [Emacs Lisp](#6242-emacs-lisp)、上へ: [その他の言語のサポート](#624-他の言語のサポート) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 6.24.1 他の言語の使用

現在、Guile 内から他の言語にアクセスする方法は 2 つしかありません。REPL を使用する方法と、`compile`、`read-and-compile`、`compile-file` を介してプログラム的にアクセスする方法です。

REPLはGuileのコマンドプロンプトです（[Guileを対話的に使用する](04_programming_in_scheme.md#44-guile-を対話的に使用する)を参照）。REPLには「現在の言語」という概念があり、デフォルトではSchemeになっています。ユーザーはメタコマンド`,language`を使用して言語を変更できます。

例えば、以下のメタコマンドはEmacs Lisp入力を有効にします。

scheme@(guile-user)> 、言語 elisp
Emacs Lispでハッキングを楽しんでください！元に戻すには、「\`,L scheme'」と入力してください。
elisp@(guile-user)> (eq 1 2)
$1 = #nil

各言語には略称があります。例えば、Elisp は `elisp` です。同じ略称は、`compile` コマンドを使用してソースコードをプログラム的にコンパイルする際にも使用できます。

elisp@(guile-user)> 、Lスキーム
Guile Scheme でハッキングを楽しんでください！元に戻すには、\`,L elisp' と入力してください。
scheme@(guile-user)> (compile '(eq 1 2) #:from 'elisp)
$2 = #nil

確かに、`compile`への入力はデータであるため、これはデータ表現が単純なLispy言語に最適です。より複雑な解析が必要な他の言語は、文字列として扱う方が適しています。

構文が複雑な言語を扱う最も簡単な方法は、`compile-file`などのコマンドを使ってファイルを扱うことです。ただし、ポート上で言語のリーダーを呼び出し、結果として得られる式（その時点ではデータ）をコンパイルすることも可能です。詳細については、[Schemeコードのコンパイル](06_16_reading_and_evaluating_scheme_code.md#6166-scheme-コードのコンパイル)を参照してください。

さまざまな言語のアスペクトを内省する方法の詳細については、[コンパイラタワー](09_04_compiling_to_the_virtual_machine.md#941-コンパイラタワー)を参照してください。

* * *

次へ: [ECMAScript](#6243-ecmascript)、前: [他の言語の使用](#6241-他の言語の使用)、上: [他の言語のサポート](#624-他の言語のサポート) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 6.24.2 Emacs Lisp

Emacs Lisp（Elisp）は、Emacsエディタで使用される動的スコープのLisp方言です。Emacs Lispの詳細については、Emacs Lispの[概要](https://www.gnu.org/software/emacs/manual/html_mono/elisp.html#Top)を参照してください。

最終的には、GuileによるElispの実装がEmacs独自のElisp実装に取って代わるほど優れたものになることを期待しています。そのため、Elispの様々な機能を高性能かつ互換性のある方法でサポートする方法について、長年にわたり真剣に検討を重ねてきました。

Emacs Lispに精通している読者は、Guileでこれらの様々なElisp機能が具体的にどのようにサポートされているのかに興味を持つかもしれません。このセクションの残りの部分では、Elisp愛好家のこうした疑問に答えることに焦点を当てます。

* [Nil](#62421-nil)
* [動的バインディング](#62422-動的バインディング)
* [その他のElisp機能](#62423-その他のelisp機能)

* * *

次へ: [Dynamic Binding](#62422-動的バインディング)、上へ: [Emacs Lisp](#6242-emacs-lisp) \[[Contents](00_contents.md "目次")\]\[[Index](index_r5rs.md "Index")\]

#### 6.24.2.1 Nil

ELispにおける`nil`は、Schemeの`#f`と`'()`を組み合わせたものです。偽であり、リストの末尾を表します。つまり、ブール値であると同時にリストでもあります。

Guile は、`nil` を `#f` や `'()` とは別の値としてサポートすることを選択しました。これにより、既存の Scheme および Elisp コードは、現在のセマンティクスを維持できます。Elisp では単に `nil` と記述および読み取られる `nil` は、Scheme では外部表現 `#nil` を持ちます。

Elisp コードでは、`#nil`、`#f`、および `'()` は `nil` と同じように動作します。つまり、Elisp の `if`、`cond`、`when`、`not`、`null` などによってすべて `nil` として解釈されます。Scheme コード内から Elisp がオブジェクトを `nil` として解釈するかどうかをテストするには、`nil?` を使用します。

Scheme Procedure: **nil?** obj

objがEmacs Lispコードによって`nil`と解釈される場合は`#t`を返し、そうでない場合は`#f`を返します。

([nil?](#62421-nil) #nil) ⇒ #t
([nil?](#62421-nil) #f) ⇒ #t
([nil?](#62421-nil) '()) ⇒ #t
([nil?](#62421-nil) 3) ⇒ #f

`nil`を低レベルの明確な値として扱うというこの決定は、2つの言語間の相互運用性を容易にします。Guileは、Schemeが`nil`を次のように扱うように選択しました。

(ブール値? #nil) ⇒ #t
(nilではない) ⇒ #t
(null? #nil) ⇒ #t

C言語では、次のようになります。

scm_is_bool (SCM_ELISP_NIL) ⇒ 1
scm_is_false (SCM_ELISP_NIL) ⇒ 1
scm_is_null (SCM_ELISP_NIL) ⇒ 1

このように、Schemeで書かれた`fold`関数は、Elisp（または他の言語）で書かれた関数を、Elispが行うように、ヌル終端リストに対して正しく畳み込むことができます。逆もまた同様です。Elispで書かれた`fold`関数は、Schemeが行うように、`'()`で終端されたリストに対して畳み込むことができます。

低レベルでは、`#f`、`#t`、`nil`、および`'()`のビット表現は、1ビットだけ異なるように作成されているため、例えば`#f`\-or-`nil`のテストを非常に効率的に行うことができます。詳細については、`libguile/boolean.h`を参照してください。

#### 平等

Scheme の `equal?` は推移的でなければならず、`'()` は `#f` と `equal?` ではないため、Scheme では `nil` は `#f` または `'()` と `equal?` ではありません。

(eq? #f '()) ⇒ #f
(eq? #nil '()) ⇒ #f
(eq? #nil #f) ⇒ #f
(eqv? #f '()) ⇒ #f
(eqv? #nil '()) ⇒ #f
(eqv? #nil #f) ⇒ #f
(等しい? #f '()) ⇒ #f
(等しい? #nil '()) ⇒ #f
(等しい? #nil #f) ⇒ #f

しかし、Elispでは、`'()`、`#f`、`nil`はすべて`equal`です（ただし`eq`ではありません）。

(defvar f (make-scheme-false))
(defvar eol (make-scheme-null))
(eq f eol) ⇒ nil
(eq nil eol) ⇒ nil
(eq nil f) ⇒ nil
(等しい f eol) ⇒ t
(等号 nile eol) ⇒ t
(等しい nil f) ⇒ t

これらの選択肢はElispとSchemeコード間の相互運用性を容易にしますが、完璧ではありません。標準Schemeでは正しいコードでも、2つ目のfalse値とnull値が存在する場合には正しくない場合があります。例えば、次のようになります。

(真偽値 x を定義)
(もし (eq? x #f) が等しい場合)
#f
#t))

このコードは値の真偽を判定するためのもののようですが、偽値である`#f`と`nil`が2つ存在するため、もはや正しくありません。

同様に、ループも存在する。

(define (my-length l)
(let lp ((ll) (len 0))
(もし (eq? l '()) と等しい場合)
レン
(lp (cdr l) (1+ len)))))

ここで、`my-length` は、l が `nil` で終端されたリストである場合にエラーを発生させます。

これらの例はどちらも標準Schemeとしては正しいですが、実際に何を行いたいかによっては、Guile Schemeとしては正しくありません。正しく記述すると、偽または空集合の_プロパティ_をテストし、その集合の個々の要素をテストしません。つまり、偽または空集合をテストするには、`eq?`や`memv`などではなく、`not`や`null?`を使用する必要があります。

幸いなことに、`not` と `null?` の使用は良いスタイルなので、適切に記述された標準的な Scheme プログラムはすべて Guile Scheme でも正しく動作します。

上記例の正しいバージョンは以下のとおりです。

(define (truthiness\* x)
(x でない場合)
#f
#t))
;; または: (define (t\* x) (not (not x)))
;; または: (define (t\*\* x) x)

(define (my-length\* l)
(let lp ((ll) (len 0))
(もし (null? l) ならば
レン
(lp (cdr l) (1+ len)))))

この問題には、Elisp における鏡像ケースが存在します。

(defun my-falsep (x)
(もし (eq x nil) の場合)
t
なし))

Guile は、`#f`、`'()`、または `nil` との等価比較を含むコードをコンパイルする際に警告を発することがあります。詳細については、[Scheme コードのコンパイル](06_16_reading_and_evaluating_scheme_code.md#6166-scheme-コードのコンパイル) を参照してください。

* * *

次へ: [その他のElisp機能](#62423-その他のelisp機能)、前: [なし](#62421-nil)、上: [Emacs Lisp](#6242-emacs-lisp) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 6.24.2.2 動的バインディング

「レキシカルスコープ」を使用するSchemeとは対照的に、Emacs Lispは変数のスコープを動的に制御します。Guileは「fluids」機能で動的スコープをサポートしています。詳細については、「Fluids and Dynamic States」を参照してください。

* * *

前へ: [Dynamic Binding](#62422-動的バインディング)、上へ: [Emacs Lisp](#6242-emacs-lisp) \[[Contents](00_contents.md "目次")\]\[[Index](index_r5rs.md "Index")\]

#### 6.24.2.3 その他のElisp機能

バッファローカル変数とモードローカル変数、文字のバッキービット、Emacsのプリミティブデータ型、ElispのLisp-2的な性質などについて、ここで触れておくべきでしょう。ドキュメントへの貢献は大歓迎です！

* * *

前へ: [Emacs Lisp](#6242-emacs-lisp)、上へ: [その他の言語のサポート](#624-他の言語のサポート) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 6.24.3 ECMAScript

[ECMAScript](http://www.ecma-international.org/publications/files/ECMA-ST/Ecma-262.pdf)は、Guileが実装した最初の非Scheme言語ではありませんでしたが、Guileのバイトコードコンパイラ向けに実装された最初の言語でした。目標は、比較的小規模な言語であるECMAScriptバージョン3.1をサポートすることでしたが、実装者は全く無責任で、標準ライブラリを完成させる前に、さらには構文の一部さえも完成させる前に、他のことに気を取られてしまいました。そのため、ECMAScriptはマニュアルに記載されるべきですが、実装が完了するまでは、おそらくもっと責任感のあるハッカーによって実装されるまでは、推奨されるべきではありません。

その間、親切なユーザーは、`,L ecmascript` や `cat test-suite/tests/ecmascript.test` のような呼び出しを調査するかもしれません。

* * *

次へ: [デバッグインフラストラクチャ](06_26_debugging_infrastructure.md#626-デバッグインフラストラクチャ)、前: [他の言語のサポート](#624-他の言語のサポート)、上: [API リファレンス](06_00_api_reference.md#6-apiリファレンス) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
