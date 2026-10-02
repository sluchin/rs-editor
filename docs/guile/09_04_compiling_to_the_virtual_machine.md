### 9.4 仮想マシンへのコンパイル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compiling-to-the-Virtual-Machine-1)

コンパイラ！その言葉自体が、熟練したプログラマーでさえも興奮と畏敬の念を抱かせる。しかし、コンパイラは単なるプログラムであり、容易にハッキングできるものだ。このセクションでは、Guileのコンパイラを、興味のあるSchemeハッカーが安心して読み、拡張できるように説明することを目的とする。

.scm ファイルのコンパイル方法が分からず困っている場合は、[Scheme コードの読み取りと評価](https://doc.guix.gnu.org/guile/latest/en/guile.html#Read_002fLoad_002fEval_002fCompile) を参照してください。

* [コンパイラタワー](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compiler-Tower)
* [Schemeコンパイラ](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Scheme-Compiler)
* [Tree-IL](https://doc.guix.gnu.org/guile/latest/en/guile.html#Tree_002dIL)
* [継続パススタイル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Continuation_002dPassing-Style )
* [バイトコード](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytecode)
* [新しい高水準言語の作成](https://doc.guix.gnu.org/guile/latest/en/guile.html#Writing-New-High_002dLevel-Languages)
* [コンパイラの拡張](https://doc.guix.gnu.org/guile/latest/en/guile.html#Extending-the-Compiler)

* * *

次へ: [Scheme コンパイラ](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Scheme-Compiler)、上: [仮想マシンへのコンパイル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compiling-to-the-Virtual-Machine) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.4.1 コンパイラタワー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compiler-Tower-1)

Guileのコンパイラは非常にシンプルです。より正確に言うと、コンパイラ群です。Guileは、Schemeから始まり、VM命令セットに似た言語へと段階的に簡略化していく言語群を定義しています（[命令セット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instruction-Set)を参照）。

各言語は次の言語へのコンパイル方法を認識しているため、各ステップはシンプルで理解しやすいものとなっています。さらに、これらの言語はGuileにハードコーディングされていないため、ユーザーは新しい高水準言語、新しいパス、あるいは異なるコンパイルターゲットを追加することも可能です。

言語はモジュール「(システム基本言語)」に登録されます。

(use-modules (システム基本言語))

これらは「define-language」フォームで登録されます。

Scheme構文: **define-language** \[#:name\] \[#:title\] \[#:reader\] \[#:printer\] \[#:parser=#f\] \[#:compilers='()\] \[#:decompilers='()\] \[#:evaluator=#f\] \[#:joiner=#f\] \[#:for-humans?=#t\] \[#:make-default-environment=make-fresh-user-module\] \[#:lowerer=#f\] \[#:analyzer=#f\] \[#:compiler-chooser=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-define_002dlanguage)

言語を定義する。

この構文は、現在の環境の name にバインドされた `<language>` オブジェクトを定義します。さらに、この言語はグローバル言語セットに追加されます。たとえば、Scheme の言語定義は次のようになります。

(言語定義スキーム)
#:title 	"スキーム"
#:reader (lambda (port env) ...)
#:compilers \`((tree-il . ,compile-tree-il))
#:decompilers \`((tree-il . ,decompile-tree-il))
#:evaluator 	(lambda (x module) (primitive-eval x))
#:プリンター	書き込み
#:make-default-environment (lambda () ...))

このように言語を定義することの興味深い点は、読み込み・評価・出力ループに対して統一されたインターフェースを提供することです。これにより、ユーザーはREPLの現在の言語を変更できます。

scheme@(guile-user)> 、言語ツリー-il
Tree中間言語でハッキングを楽しんでください！元に戻すには、「\`,L scheme」と入力してください。
tree-il@(guile-user)> 、Lスキーム
Scheme でハッキングを楽しんでください！元に戻すには、`,L tree-il` と入力してください。
scheme@(guile-user)>

言語は、上記のように名前で検索できます。

Scheme 手順: **lookup-language** name [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-lookup_002dlanguage)

指定された名前の言語を検索し、必要に応じて自動的に読み込みます。

言語は、`(言語名指定)` という名前のモジュール内で、name という名前の変数を探すことによって自動的にロードされます。

言語オブジェクトが返されます。その名前の言語が存在しない場合は「#f」が返されます。

Guile が Scheme をバイトコードにコンパイルする際、Scheme 言語に対して、Scheme からバイトコードへのパス上の次の言語へのコンパイラを選択するように要求します。この計算を再帰的に実行することで、柔軟なコンパイラの連鎖から変換が構築されます。次のリンクは、言語のコンパイラセレクタを呼び出すか、存在しない場合は言語のコンパイラフィールドから取得されます。

言語はアナライザーを指定でき、そのアナライザーは、その言語の項が下位レベルに展開されコンパイルされる前に実行されます。コンパイラの警告はここで発行されます。

言語にローワー（低位化）が指定されている場合、コンパイル前に式に対してその手続きが呼び出されます。最適化や正規化はここで行われます。

最後に、言語のコンパイラは、ある言語で下降した項を、連鎖内の次の言語に翻訳します。

コアモジュール `(guile)` で定義されている `current-language` パラメータには、「現在の言語」という概念があります。この言語は通常 Scheme ですが、ユーザーが再バインドすることも可能です。実行時コンパイルインターフェース ([Scheme コードの読み込みと評価](https://doc.guix.gnu.org/guile/latest/en/guile.html#Read_002fLoad_002fEval_002fCompile) を参照) を使用すると、他のソース言語とターゲット言語を選択することもできます。

Schemeをコンパイルする際の一般的な言語の階層構造は次のようになります。

* スキーム
* ツリー中間言語 (Tree-IL)
* 継続パス型スタイル（CPS）
* バイトコード

前述のとおり（[オブジェクトファイル形式](https://doc.guix.gnu.org/guile/latest/en/guile.html#Object-File-Format)を参照）、バイトコードはELF形式で、ディスクへのシリアル化に対応しています。しかし、実行時にSchemeをコンパイルする場合、Schemeの値、例えばコンパイル済みのプロシージャが必要になります。このため、抽象化を損なわないように、Guileはタワーの底部に擬似言語を定義しています。

* 価値

`value`にコンパイルすると、バイトコードがプロシージャにロードされ、コールドバイトがウォームコードに変換されます。

おそらくこの奇妙さは例を挙げて説明できるだろう。`compile-file`はデフォルトでバイトコードにコンパイルされる。なぜなら、Guileランタイムの外にある不毛な世界で存在しなければならないオブジェクトコードを生成するからだ。一方、`compile`はデフォルトで`value`にコンパイルされる。なぜなら、その生成物はGuileの世界に再び入るからだ。

実際、コンパイルのプロセスは、次のクワインで示されるように、これらの異なる世界を無限に循環することができる。

((lambda (x) ((compile x) x)) '(lambda (x) ((compile x) x)))

* * *

次へ: [Tree-IL](https://doc.guix.gnu.org/guile/latest/en/guile.html#Tree_002dIL)、前: [Compiler Tower](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compiler-Tower)、上: [Compiling to the Virtual Machine](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compiling-to-the-Virtual-Machine) \[[Contents](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[Index](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.4.2 Schemeコンパイラ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Scheme-Compiler-1)

Schemeコンパイラの役割は、すべてのマクロとSchemeのすべての要素を最も基本的な式に展開することです。「基本的な式」の定義は、Schemeコンパイラのターゲット言語であるTree-ILが提供する構成要素（手続き呼び出し、条件分岐、字句参照など）によって決まります。これについては、次のセクションで詳しく説明します。

Scheme-to-Tree-ILコンパイラの厄介で面白い点は、マクロ展開器によって完全に実装されていることです。マクロ展開器はマクロを展開するために既にソースコード全体を走査する必要があるため、同時に解析も行い、Tree-IL式を直接生成してしまう可能性があります。

このコンパイラは実際にはマクロ展開器であるため、拡張可能です。ユーザーが記述したマクロはすべてコンパイラの一部となります。

Scheme-to-Tree-ILエキスパンダーは、汎用的な`compile`プロシージャを使用して呼び出すことができます。

([compile](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-compile) '([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) 1 2) #:from 'scheme #:to 'tree-il)
⇒
#<tree-il ([call](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-call) (toplevel [+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b)) ([const](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-const) 1) ([const](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-const) 2))[\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e)

`(compile foo #:from 'scheme #:to 'tree-il)` は、マクロ展開器を `(macroexpand foo 'c '(compile load eval))` のように呼び出すことと完全に同等です。[マクロ展開](https://doc.guix.gnu.org/guile/latest/en/guile.html#Macro-Expansion) を参照してください。`compile` によって `'tree-il` にディスパッチされるプロシージャ `compile-tree-il` は、`macroexpand` をラップした小さなもので、Guile の言語タワーにおけるコンパイラプロシージャの一般的な形式に準拠するようにしています。

コンパイラプロシージャは、式、環境、およびオプションのキーワードリストという3つの引数を取ります。そして、コンパイルされた式、対象言語に対応する環境、および「継続環境」という3つの値を返します。コンパイルされた式と環境は、次の言語のコンパイラへの入力として使用されます。「継続環境」は、同じモジュール内で同じソース言語から別の式をコンパイルするために使用できます。

例えば、`(define-module (foo))`という式をコンパイルするとします。これにより、Tree-IL式と環境が生成されます。しかし、2つ目の式をコンパイルする場合、前の式をコンパイルした際のコンパイル時の影響、つまりユーザーが`(foo)`モジュールに入るという効果を考慮する必要があります。これが「継続環境」の目的です。継続環境は、後続の式をコンパイルする際に環境として渡されます。

Schemeでは、環境とはモジュールのことです。デフォルトでは、`compile`と`compile-file`の手順は新しいモジュール内でコンパイルされるため、コンパイル対象の式によって導入されたバインディングとマクロは分離されます。

(eq? (current-module) (compile '(current-module)))
⇒ #f

(compile '(define hello 'world))
（定義済み？「こんにちは」）
⇒ #f

（定義する / \*）
(eq? (compile '/) /)
⇒ #f

同様に、`current-reader` 流体への変更 ([`current-reader`](https://doc.guix.gnu.org/guile/latest/en/guile.html#Loading) を参照) は分離されています。

(compile '(fluid-set! current-reader (lambda args 'fail)))
（流体参照電流読み取り装置）
⇒ #f

しかしながら、コンパイラとコンパイル対象が同じ名前空間を共有するようにするには、コンパイル環境として明示的に`(current-module)`を渡すことで実現できます。

(define hello 'world)
(compile 'hello #:env (current-module))
⇒世界

* * *

次へ: [継続渡しスタイル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Continuation_002dPassing-Style)、前: [Scheme コンパイラ](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Scheme-Compiler)、上: [仮想マシンへのコンパイル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compiling-to-the-Virtual-Machine) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.4.3 Tree-IL [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Tree_002dIL-1)

ツリー中間言語（Tree-IL）は、Schemeに表現力の点で近い構造化された中間言語です。これは、拡張され、事前に解析されたSchemeと言えます。

Tree-ILは、その表現がS式ではなくレコードに基づいているという意味で「構造化」されています。これにより、言語に柔軟性がもたらされ、より低レベルの言語へのコンパイルに必要な変換が限定的になります。たとえば、Tree-ILの型`<const>`は、`src`と`exp`という2つのフィールドを持つレコード型です。この型のインスタンスは`make-const`で作成されます。この型のフィールドには、`const-src`と`const-exp`プロシージャでアクセスします。また、述語`const?`もあります。レコードの詳細については、[レコード](https://doc.guix.gnu.org/guile/latest/en/guile.html#Records)を参照してください。

すべての Tree-IL 型には、式のソース位置情報を保持する `src` スロットがあります。この情報が存在する場合、コンパイルされたオブジェクト コードに残余化され、バックトレースでソース情報を表示できるようになります。`src` の形式は、 Guile の `source-properties` 関数が返す形式と同じです。詳細については、[ソース プロパティ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Source-Properties) を参照してください。

Tree-ILオブジェクトは内部的にはレコードを使用して表現されますが、各種類のTree-ILには同等のS式による外部表現も存在します。たとえば、`#<const src: #f exp: 3>`式のS式表現は次のようになります。

(定数 3)

ユーザーはREPL上でこの形式を直接使用してプログラミングできます。

scheme@(guile-user)> 、言語ツリー-il
Tree中間言語でハッキングを楽しんでください！元に戻すには、「\`,L scheme」と入力してください。
tree-il@(guile-user)> (call (primitive +) (const 32) (const 10))
⇒ 42

`src` フィールドは外部表現から除外されます。

Tree-ILオブジェクトは、Tree-ILリーダーである`parse-tree-il`を呼び出すことで、外部表現から作成できます。入力S式にソース情報が付加されている場合、その情報は生成されるTree-IL式にも反映されます。おそらくこれがTree-ILへのコンパイルの最も簡単な方法でしょう。適切な外部表現をS式形式で作成し、残りの処理は`parse-tree-il`に任せるだけです。

Scheme変数: **<void>** src [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003cvoid_003e)

外部表現: **(void)** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0028void_0029)

空の式。実際には、Scheme の `(if #f #f)` と同等です。

Scheme変数: **<const>** src exp [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003cconst_003e)

外部表現: **(const** exp) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0028const)

不変の定数。

Scheme 変数: **<primitive-ref>** src name [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003cprimitive_002dref_003e)

外部表現: **(プリミティブ**名) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0028primitive)

「プリミティブ」への参照。プリミティブとは、コンパイル時にオープンコード化される可能性のある手続きのことです。例えば、`cons` は通常プリミティブとして認識されるため、単一の命令にコンパイルされます。

Tree-ILのコンパイルは通常、`<module-ref>`式と`<toplevel-ref>`式を`<primitive-ref>`式に解決するパスから始まります。実際のコンパイルパスでは、`apply`や`cons`などの特定のプリミティブへの呼び出しに対して特別な処理が行われます。

Scheme 変数: **<lexical-ref>** src name gensym [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003clexical_002dref_003e)

外部表現: **(語彙名 gensym) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0028lexical)

字句的に束縛された変数への参照。nameはソースプログラムにおける変数の元の名前です。gensymはこの変数の一意の識別子です。

Scheme 変数: **<lexical-set>** src name gensym exp [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003clexical_002dset_003e)

外部表現: **(set!** (lexical name gensym) exp) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0028set_0021)

字句的に束縛された変数を設定します。

Scheme 変数: **<module-ref>** src mod name public? [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003cmodule_002dref_003e)

外部表現: **(@** mod name) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0028_0040)

外部表現: **(@@** mod name) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0028_0040_0040)

特定のモジュール内の変数への参照。mod にはモジュール名を指定します。例: `(guile-user)`。

public? が true の場合、name という名前の変数は mod のパブリック インターフェースで検索され、`@` でシリアル化されます。そうでない場合は、モジュールのプライベート バインディングで検索され、`@@` でシリアル化されます。

Scheme 変数: **<module-set>** src mod name public? exp [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003cmodule_002dset_003e)

外部表現: **(set!** (@ mod name) exp) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0028set_0021-1)

外部表現: **(set!** (@@ mod name) exp) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0028set_0021-2)

特定のモジュール内の変数を設定します。

スキーム変数: **<toplevel-ref>** src name [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003ctoplevel_002dref_003e)

外部表現: **(トップレベル** 名前) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0028toplevel)

現在のプロシージャのモジュール内の変数を参照します。

スキーム変数: **<toplevel-set>** src name exp [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003ctoplevel_002dset_003e)

外部表現: **(set!** (トップレベル名) exp) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0028set_0021-3)

現在のプロシージャのモジュール内の変数を設定します。

スキーム変数: **<toplevel-define>** src name exp [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003ctoplevel_002ddefine_003e)

外部表現: **(define** name exp) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0028define)

現在のプロシージャのモジュール内に、新しいトップレベル変数を定義します。

Scheme 変数: **<conditional>** src test then else [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003cconditional_003e)

外部表現: **(if** test then else) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0028if)

条件文です。else は省略できません。

スキーム変数: **<call>** src proc args [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003ccall_003e)

外部表現: **(call** proc . args) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0028call)

プロシージャコール。

スキーム変数: **<primcall>** src 名 args [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003cprimcall_003e)

外部表現: **(primcall** name . args) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0028primcall)

プリミティブへの呼び出し。`(call (プリミティブ名) . args)` と同等です。この構文は、`<call>` よりも生成や分析が便利な場合が多いです。

コンパイル処理の一環として、`(call (primitive name) . args)` のインスタンスは primcall に変換されます。

スキーム変数: **<seq>** src head tail [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003cseq_003e)

外部表現: **(seq** ヘッド テール) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0028seq)

シーケンス。意味としては、まず先頭が評価され、その結果得られた値は無視されます。次に末尾が末尾の位置で評価されます。

Scheme 変数: **<lambda>** src meta body [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003clambda_003e)

外部表現: **(ラムダ** メタボディ) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0028lambda)

クロージャ。meta はプロシージャのプロパティの関連付けリストです。body は、型 `<lambda-case>` の単一の Tree-IL 式です。`<lambda-case>` 句は別の句にチェーンできるため、Tree-IL の `<lambda>` は Scheme の `case-lambda` と同等の表現力を持ちます。

スキーム変数: **<lambda-case>** req opt rest kw inits gensyms body alternate [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003clambda_002dcase_003e)

外部表現: **(lambda-case** ((req opt rest kw inits gensyms) body) \[alternate\]) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0028lambda_002dcase)

`case-lambda` の 1 つの節。Scheme の `lambda` 式は、1 つの節を持つ `case-lambda` として扱われます。

req は、プロシージャの必須引数をシンボルで表したリストです。opt は、オプション引数のリスト、またはオプション引数がない場合は `#f` です。rest は、残りの引数の名前、または `#f` です。

kw は、`(allow-other-keys? (keyword name var) ...)` の形式のリストです。ここで、keyword は name という名前の引数に対応するキーワードであり、対応する gensym は var です。キーワード引数がない場合は `#f` になります。inits は、すべてのオプション引数とキーワード引数に対応する tree-il 式で、プロシージャ呼び出し元によって値が提供されない変数をバインドするために評価されます。各 init 式は、左から右に、以前にバインドされた変数の字句コンテキストで評価されます。

gensyms は、すべての引数に対応する gensyms のリストです。まず必須引数、次にオプション引数（存在する場合）、次に残りの引数（存在する場合）、最後にすべてのキーワード引数が続きます。

body は節の本体です。プロシージャが適切な数の引数で呼び出された場合、body は末尾で評価されます。そうでない場合、代替式が存在する場合は、次に試行する節を表す `<lambda-case>` 式である必要があります。代替式が存在しない場合は、引数の数が間違っているというエラーが通知されます。

Scheme 変数: **<let>** src names gensyms vals exp [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003clet_003e)

外部表現: **(let** names gensyms vals exp) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0028let)

Scheme の `let` のような字句バインディング。names は元のバインディング名、gensyms は名前に対応する gensym、vals は値を表す Tree-IL 式です。exp は単一の Tree-IL 式です。

Scheme 変数: **<letrec>** 順序通り? src names gensyms vals exp [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003cletrec_003e)

外部表現: **(letrec** names gensYSms vals exp) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0028letrec)

外部表現: **(letrec\*** names gensYSms vals exp) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0028letrec_002a)

Scheme の `letrec` や in-order? が true の場合は `letrec*` のように再帰的なバインディングを作成する `<let>` のバージョン。

Scheme変数: **<prompt>** エスケープのみ? タグ本体ハンドラ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003cprompt_003e)

外部表現: **(prompt** エスケープのみ? タグ本体ハンドラ) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0028prompt)

動的プロンプト。`body`（これも式）の実行の動的範囲内で、`tag`（式）という名前のプロンプトを設定します。このプロンプトで中止が発生した場合、制御は`handler`（これも式で、プロシージャである必要があります）に渡されます。ハンドラプロシージャの最初の引数はキャプチャされた継続で、その後に中止時に渡されたすべての値が続きます。`escape-only?`がtrueの場合、ハンドラは、オプション引数やキーワード引数、代替引数がなく、最初の引数が参照されない単一の`<lambda-case>`本体式を持つ`<lambda>`である必要があります。詳細については、[プロンプト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Prompts)を参照してください。

Scheme 変数: **<abort>** tag args tail [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003cabort_003e)

外部表現: **(abort** タグ args tail) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0028abort)

名前タグと式を持つ最も近いプロンプトに強制終了します。args はプロンプトのハンドラに渡す式のリスト、tail は追加の引数のリストに評価される式である必要があります。強制終了すると部分的な継続が保存され、後で復元される可能性があります。その結果、`<abort>` 式はいくつかの値に評価されます。

通常、上位レベルのコンパイラでは生成されないものの、Tree-ILコンパイラが行うソースコード間の最適化および解析処理中に生成されるTree-IL構文が2つあります。デフォルトの解析処理で必要に応じて生成されるため、ユーザーはよほど高度な知識がない限り、これらの式を直接生成すべきではありません。

Scheme 変数: **<let-values>** src names gensyms exp body [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003clet_002dvalues_003e)

外部表現: **(let-values** names gensyms exp body) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0028let_002dvalues)

Schemeの`receive`と同様に、`exp`の評価によって返される値を、gensymsで記述される`lambda`のようなバインディングにバインドします。つまり、gensymsは不適切なリストである可能性があります。

`<let-values>` は、`<call>` をプリミティブである `call-with-values` に最適化したものです。

Scheme 変数: **<fix>** src names gensyms vals body [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003cfix_003e)

外部表現: **(修正** 名前 gensyms vals body) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0028fix)

`<letrec>`と同様ですが、未設定のラムダ式である値にのみ適用されます。

`fix`は`letrec`（および`let`）の最適化です。

Tree-ILは、ソース言語からのコンパイルターゲットとして便利です。最適化の手段としても便利ですが、通常はCPSの方が優れています。Tree-ILの強みは、評価順序を固定しないため、コードの移動が多少容易になる点です。

Tree-ILに対して現在実行されている最適化パスは以下のとおりです。

* オープンコーディング（トップレベル参照をプリミティブ参照に変換し、プリミティブへの呼び出しをプリミティブ呼び出しに変換する）
* 部分評価（インライン化、コピー伝播、定数畳み込みを含む）

* * *

次へ: [バイトコード](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytecode)、前: [ツリーIL](https://doc.guix.gnu.org/guile/latest/en/guile.html#Tree_002dIL)、上: [仮想マシンへのコンパイル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compiling-to-the-Virtual-Machine) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.4.4 継続パススタイル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Continuation_002dPassing-Style-1)

継続渡しスタイル（CPS）は、Guileの主要な中間言語であり、人間が使う言語と機械が使う言語の間のギャップを埋める役割を果たします。CPSはプログラムのあらゆる部分、つまりすべての制御点とすべての中間値に名前を付けます。そのため、コンパイラの主要な役割であるプログラムの推論を行うための優れた手段となります。

* [CPS入門](https://doc.guix.gnu.org/guile/latest/en/guile.html#An-Introduction-to-CPS)
* [GuileにおけるCPS](https://doc.guix.gnu.org/guile/latest/en/guile.html#CPS-in-Guile)
* [CPSの構築](https://doc.guix.gnu.org/guile/latest/en/guile.html#Building-CPS)
* [CPS Soup](https://doc.guix.gnu.org/guile/latest/en/guile.html#CPS-Soup)
* [CPSのコンパイル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compiling-CPS)

* * *

次へ: [Guile の CPS](https://doc.guix.gnu.org/guile/latest/en/guile.html#CPS-in-Guile)、上: [Continuation-Passing Style](https://doc.guix.gnu.org/guile/latest/en/guile.html#Continuation_002dPassing-Style) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.4.4.1 CPS の概要 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#An-Introduction-to-CPS-1)

次のScheme式を考えてみましょう。

（始める
([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) "32と10の合計は：")
([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) 42)
([newline](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-newline)))

この式に含まれるすべての部分式を特定し、それぞれに固有のラベルを付けて注釈を付けましょう。

（始める
([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) "32と10の合計は：")
|k1 k2
k0
([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) 42)
|k4 k5
k3
([newline](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-newline)))
|k7
k6

これらのラベルはそれぞれ、プログラム内の特定の箇所を示します。あるラベルが別のラベルの続きとなる場合もあります。例えば、`k7` の続きは `k6` です。これは、`k7` というラベルの式によって `newline` の値が評価された後、その値を `k6` で引き続き適用するためです。

どちらの式が `k0` を継続として持ちますか？それは `k1` とラベル付けされた式か `k2` とラベル付けされた式のどちらかです。Scheme では引数の評価順序は固定されていませんが、何らかの順序で評価されることは保証されています。一般的な Scheme とは異なり、継続渡しスタイルでは評価順序が明示されます。Guile では、この選択は上位言語コンパイラによって行われます。

左から右への評価順序を仮定しましょう。この場合、`k1` の継続は `k2` であり、`k2` の継続は `k0` です。

この例を踏まえて、SchemeにおけるCPSの例を示しましょう。

(ラムダ (ktail)
(let ((k1 (lambda ()
(let ((k2 (lambda (proc)
(let ((k0 (lambda (arg0)
(proc k4 arg0))))
(k0 "32と10の合計は: ")))))
(k2 [display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display)))))
(k4 (lambda [\_](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_005f)
(let ((k5 (lambda (proc)
(let ((k3 (lambda (arg0)
(proc k7 arg0))))
(k3 42)))))
(k5 [display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display)))))
(k7 (lambda [\_](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_005f)
(let ((k6 (lambda (proc)
(proc ktail))))
(k6 [newline](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-newline))))))
(k1))

なんてこった、コードが爆発的に増えたぞ！ラムダ式がこんなに多いのはどういうことだ？確かに、CPSはTree-ILのような「直接型」の中間言語よりも本質的に冗長だ。同時に、CPSはより明示的な表現を用いるため、完全なSchemeよりもシンプルだ。

元のプログラムでは、`k0` というラベルの付いた式は実質的にコンテキストにあります。この式が返す値はすべて無視されます。Scheme では、この事実は暗黙的に示されています。CPS では、その継続である `k4` が任意の数の値を受け取り、それらを無視することに注目することで、この事実を明示的に確認できます。単一の値を受け取る `k2` と比較すると、`k1` は「値」コンテキストにあると言えます。同様に、`k6` は式全体に関して末尾コンテキストにあります。なぜなら、その継続は末尾継続である `ktail` だからです。CPS はこれらの詳細を明確にし、名前を付けています。

* * *

次へ: [CPS の構築](https://doc.guix.gnu.org/guile/latest/en/guile.html#Building-CPS)、前: [CPS 入門](https://doc.guix.gnu.org/guile/latest/en/guile.html#An-Introduction-to-CPS)、上: [継続 - パススタイル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Continuation_002dPassing-Style) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.4.4.2 Guile の CPS [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#CPS-in-Guile-1)

GuileのCPS言語は、_継続_で構成されています。継続とは、ラベル付きのプログラムポイントのことです。従来のコンパイラに慣れている方は、継続を基本的なブロックだと考えてください。プログラムは、ラベルから継続へのマップとして表現される、継続の「スープ」です。

基本ブロックと同様に、各継続は 1 つの関数にのみ属します。関数のエントリ ポイントに対応する継続や、関数の末尾を表す継続など、特殊な継続もあります。その他の継続には _項_ が含まれます。項には _式_ が含まれ、これは 0 個以上の値に評価されます。項はまた、その値を渡す継続も記録します。条件分岐などの一部の項は、複数の継続のうちの 1 つに継続する場合があります。

継続ラベルは小さな整数です。そのため、簡単にソートしたり、グループ分けしたりできます。ある項が継続を参照する場合、その項は継続のラベルを記録するだけで、名前で参照します。継続ラベルは、プログラム内のラベルセットの中で一意です。

変数には小さな整数が付けられます。変数名は、プログラム内の変数セットの中で一意である必要があります。

例えば、2つの値を受け取ってそれらを加算する単純な継続は、`(ice-9 match)`の`match`形式を使用して、次のようにマッチングできます。

（試合継続）
(($ $kargs (x 名 y 名) (x 変数 y 変数)
($ $Continue k src ($ $primcall '+ #f (x-var y-var))))
(書式 #t "~a と ~a を加算し、結果をラベル ~a に渡す"
x変数 y変数 k)))

ここでは、最も一般的な継続の形式である `$kargs` が見られます。これは、いくつかの値を変数にバインドしてから、項を評価します。

CPS の続き: **$kargs** は変数名を用語として指定します [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024kargs)

入力された値を、元の名前が names の変数 vars にバインドし、その後 term を評価します。

`$kargs` の名前はデバッグ専用であり、最終的にはデバッガーで使用するためにオブジェクトファイルに残されます。

`$kargs` 内の項は常に `$continue` であり、これは式を評価し、継続へと続きます。

CPS 用語: **$continue** k src exp [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024continue)

式 exp を評価し、結果の値 (存在する場合) を継続処理 k に渡します。式に関連付けられたソース情報は src に格納されており、src は `source-properties` のような alist であるか、関連付けられたソースがない場合は `#f` となります。

式にはいくつかの種類があります。上記は`$primcall`の例です。

CPS式: **$primcall** 名前 パラメータ 引数 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024primcall)

既知のシンボルである `name` で識別される基本操作を引数 args を渡して実行し、結果として得られたすべての値を継続に渡します。

param は定数パラメータであり、その解釈は該当する primcall に依存します。通常は `#f` ですが、定数を値に加算する `add/immediate` など、コンパイル時に定数情報が必要となる primcall の場合は、このパラメータにその情報が格納されます。

利用可能なプリミティブのセットには、Tree-ILで認識されている多くのプリミティブに加え、さらに多くのプリミティブが含まれています。詳細はソースコードを参照してください。なお、Tree-ILのプリミティブ呼び出しの中には、より低レベルのCPSプリミティブ呼び出しのシーケンスに変換する必要があるものがあります。詳細については、`(language tree-il compile-cps)`を参照してください。

`$primcall`、あるいは実際にはあらゆる式で使用される変数は、式が評価される前に定義されていなければなりません。言い換えれば、式で使用される変数を束縛する先行継続`$kargs`は、その式を使用する継続を_支配_しなければなりません。つまり、定義は使用を支配するということです。この条件は上記の例では自明に満たされていますが、一般に、特定の項の「スコープ」にある変数のセットを決定するには、どの継続が項を支配しているかを調べるためにフロー分析を行う必要があります。スコープにある変数は、項を支配する継続によって定義された変数です。

以下に、既に説明した `$primcall` 以外の、Guile の CPS 言語における式の種類を一覧で示します。すべての式は、その継続を指定する `$continue` 項で囲まれていることを思い出してください。

CPS式: **$const** val [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024const)

定数値 val で続行します。

CPS式: **$prim** name [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024prim)

指定された名前のプリミティブ操作を実行する手順を続行します。

CPS 式: **$call** proc args [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024call)

引数argsを指定してprocを呼び出し、すべての値を継続に渡します。procとargsリストの要素はすべて変数名である必要があります。項kで識別される継続は、`$kreceive`または`$ktail`インスタンスである必要があります。

CPS式: **$values** 引数 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024values)

リスト引数で指定された値を継続に渡します。

CPS式: **$prompt** エスケープ? タグハンドラ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024prompt)

CPSには、_高階CPS_と_一階CPS_という2つのサブ言語があります。違いは、高階CPSでは、`$fun`式と`$rec`式があり、これらによって関数または相互再帰関数が、使用箇所の暗黙的なスコープに束縛される点です。Guileは、_クロージャ変換_によって高階CPSを一階CPSに変換します。この変換では、すべてのクロージャの表現を選択し、すべての関数呼び出しに渡される暗黙的なクロージャパラメータを介して自由変数にアクセスできるようにします。

CPS式: **$fun**本体 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024fun)

手続きを続行します。body は関数のエントリ ポイントを指定します。エントリ ポイントは `$kfun` である必要があります。この式の種類は、クロージャ変換前の CPS 言語である高階 CPS でのみ有効です。

CPS式: **$rec** は変数と関数の名前を指定します [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024rec)

相互再帰的な一連の手順を、名前、変数、関数で示して続行します。名前はシンボルのリスト、変数は変数名（一意の整数）のリスト、関数は`$fun`値のリストです。`$kargs`継続では、名前と変数のバインディングも定義する必要があることに注意してください。

継続処理パスでは、`$rec` で宣言された関数をローカル継続に変換しようとします。残った `$fun` インスタンスは、クロージャ変換パスによって後で削除されます。関数に自由変数がない場合は、定数として割り当てられます。

CPS式: **$const-fun**ラベル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024const_002dfun)

エントリポイントがラベルである関数を表す定数。定数であるため、同じラベルを持つ`$const-fun`のインスタンスはメモリを割り当てません。関数のメモリ領域はコンパイル単位の一部として割り当てられます。

実際には、コンパイル単位内で呼び出し箇所がすべて可視ではなく、かつ自由変数を持たない関数については、`$const-fun` 式は CPS 変換によって具体化されます。この式の種類は、一階 CPS の一部です。

それ以外の場合、クロージャに自由変数がある場合は、定義箇所で`allocate-words` primcallによって割り当てられ、そこで自由変数が初期化されます。クロージャ内のコードポインタは`$code`式から初期化されます。

CPS式: **$code** ラベル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024code)

ラベルの値を使用して処理を続行します。この値は、プログラム内の `$kfun` 継続を示す必要があります。クロージャオブジェクトのコードポインタを初期化する際に使用されます。

ただし、クロージャがそのスコープから決して外れないことが証明できる場合は、より軽量な表現方法を選択できます。さらに、すべての呼び出し箇所が既知であれば、クロージャ変換によって`$call`を`$callk`に下げることで呼び出しがハードワイヤリングされます。

CPS 式: **$callk** ラベル proc args [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024callk)

`$call`と同様ですが、呼び出し先が同じコンパイル単位内にあることがわかっている場合に使用します。labelは、プログラム内の何らかの`$kfun`継続を示す必要があります。この場合、procは実行時に呼び出し先を決定するために使用されないため、単なる追加引数となります。

要約すると、`$continue` は単一のラベルに続く CPS 用語です。しかし、`$branch`、`$switch`、`$throw`、`$prompt` など、異なる数のラベルに続くことができる他の種類の CPS 用語もあります。

CPS 用語: **$branch** kf kt src op param args [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024branch)

分岐関数 primcall を引数 args と定数パラメータ param で評価し、テストが真であればゼロ値で kt に進みます。そうでなければ kf に進みます。

`$branch` 項は、`$primcall` 式を持つ `$continue` 項に似ていますが、値をバインドして単一のラベルに継続する代わりに、テストの結果はバインドされず、代わりに継続ラベルを選択するために使用されます。

`$branch` で有効な操作（op値に対応）のセットは限られています。一般的には、テスト式の結果を変数にバインドし、その変数を参照する `true?` op に対して `$branch` を作成します。オプティマイザは、可能であればこの分岐をインライン化します。

CPS用語: **$switch** kf kt\* src arg [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024switch)

インデックス引数 arg に従ってリスト k* 内のラベルに処理を続行するか、arg が k* の長さ以上の場合はデフォルトの継続 kf に処理を続行します。インデックス変数 arg は、ボックス化されていない符号なし 64 ビット値です。

`$switch` 項は、C言語の `switch` 文に相当します。CPS コンパイラは、ソース言語にそのような概念があれば、`$switch` 項を直接生成できます。あるいは、CPS オプティマイザに適切な `$branch` 文の連鎖を `$switch` インスタンスに変換するように依頼することもできます。Scheme コンパイラは後者の方法を採用しています。

CPS用語: **$throw** src op param args [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024throw)

再開不可能な例外をスローします。例外をスローすると、処理は一切続行されません。op の通常の値は `throw` で、引数 key と args の 2 つを持ちます。また、VM の `throw/value` および `throw/value+data` 命令にコンパイルされる特定の primcall もいくつかあります。詳細はコードを参照してください。

`$throw` を項として持つ利点は、それが継続しないため、オプティマイザが型述語からより多くの情報を収集できることです。たとえば、述語が `char?` で、kf が throw に継続する場合、kt によって支配されるラベルの集合は、throw が表記上、到達することのないラベルに継続する場合よりも大きくなります。

CPS 用語: **$prompt** k kh src エスケープ? タグ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024prompt-1)

変数名タグで識別されるプロンプトをスタックにプッシュします。escape? が true の場合、プロンプトはエスケープ専用になります。その後、ゼロ値で kh へ処理を続行します。本体がこのプロンプトで異常終了した場合、制御は kh というラベルの付いた継続（`$kreceive` 継続）に進みます。プロンプトは後で `pop-prompt` プライマリ呼び出しによってポップされます。

ここまでで、用語、式、そして最も一般的な継続の種類である `$kargs` について説明しました。`$kargs` は、継続の先行プロセスに、継続が望む場所に値を渡すように指示できる場合に使用します。たとえば、`$kargs` 継続 k が変数 v をバインドし、コンパイラが v をスロット 6 に割り当てると決定した場合、k のすべての先行プロセスは、k にジャンプする前に、v の値をスロット 6 に格納する必要があります。これが不可能な状況の 1 つは、関数呼び出しから値を受け取る場合です。Guile には関数呼び出し規約があり、現在、戻り値はスタックに配置されます。呼び出しの継続では、関数から返される値の数が期待される値の数と一致するかどうかを確認し、それらの値を名前付き変数にシャッフルまたは収集する必要があります。`$kreceive` はこの種の継続を表します。

CPS の継続: **$kreceive** arity k [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024kreceive)

スタック上の値を受け取ります。引数の数に応じて値を解析し、解析された値をラベルkの`$kargs`継続に渡します。`$kreceive`固有の制限として、引数の数には必須引数と残りの引数のみを含めることができます。

`$arity` は、`$kreceive` および `$kclause` で使用されるヘルパーデータ構造であり、以下で説明します。

CPSデータ: **$arity** req opt rest kw allow-other-keys? [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024arity)

引数の数を宣言するデータ型。req と opt は、それぞれ必須引数とオプション引数のソース名のリストです。rest は、残りの変数のソース名、またはこの引数数が追加の値を受け入れない場合は `#f` です。kw は、キーワード引数を記述する `((キーワード名 変数) ...)` の形式のリストです。allow-other-keys? は、他のキーワード引数が許可されている場合は true、そうでない場合は false です。

なお、kwリスト内のvarsを除き、これらの名前はすべてソース名であり、固有の変数名ではありません。

さらに、関数エントリでのみ使用される特定の種類の継続が3つあります。

CPS 継続: **$kfun** src meta self tail clause [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024kfun)

関数エントリを宣言します。src はプロシージャ宣言のソース情報であり、meta は Tree-IL の `<lambda>` で上記で説明したメタデータ alist です。self は呼び出されるプロシージャにバインドされた変数であり、自己参照に使用できます。tail はこの関数の `$ktail` のラベルであり、関数の末尾継続に対応します。clause は関数内の最初の `case-lambda` 句の最初の `$kclause` のラベル、またはそれ以外の場合は `#f` です。

CPS の続き: **$ktail** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024ktail)

尾部の続き。

CPS 継続: **$kclause** arity cont alternate [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0024kclause)

指定された引数数を持つ関数の節。互換性のある実引数を持つ関数の適用は、節本体を表す `$kargs` インスタンスである `cont` というラベルの付いた継続に続きます。引数に互換性がない場合、制御は alternative に進みます。alternate は次の節を表す `$kclause` 、次の節がない場合は `#f` になります。

* * *

次へ: [CPS Soup](https://doc.guix.gnu.org/guile/latest/en/guile.html#CPS-Soup)、前: [Guile の CPS](https://doc.guix.gnu.org/guile/latest/en/guile.html#CPS-in-Guile)、上: [Continuation-Passing Style](https://doc.guix.gnu.org/guile/latest/en/guile.html#Continuation_002dPassing-Style) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.4.4.3 CPSの構築 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Building-CPS-1)

Tree-ILとは異なり、CPS言語は、手続き型コンストラクタやアクセサ、あるいはS式マッチングではなく、抽象マクロを用いて構築および分解されるように設計されています。

分解とマッチングは、`(ice-9 match)` の `match` フォームで適切に処理されます。[パターンマッチング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Pattern-Matching) を参照してください。構築は、相互に連携するビルダーマクロのセット、`build-term`、`build-cont`、および `build-exp` で処理されます。

以下のインターフェース定義において、`term`と`exp`はそれぞれ`build-term`または`build-exp`によって構築されるものとします。その他の名前はすべてScheme式として評価されるものとします。これらの形式の多くは、特定のコンテキストにおいて`unquote`を認識し、既に構築された値を挿入します。詳細については、以下の仕様を参照してください。

Scheme構文: **build-term** ,val [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-build_002dterm)

Scheme構文: **build-term** ($continue k src exp) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-build_002dterm-1)

Scheme構文: **build-exp** ,val [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-build_002dexp)

Scheme構文: **build-exp** ($const val) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-build_002dexp-1)

Scheme構文: **build-exp** ($prim name) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-build_002dexp-2)

Scheme構文: **build-exp** ($fun kentry) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-build_002dexp-3)

Scheme構文: **build-exp** ($const-fun kentry) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-build_002dexp-4)

Scheme構文: **build-exp** ($code kentry) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-build_002dexp-5)

Scheme構文: **build-exp** ($rec names syms funs) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-build_002dexp-6)

Scheme構文: **build-exp** ($call proc (arg ...)) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-build_002dexp-7)

Scheme構文: **build-exp** ($call proc args) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-build_002dexp-8)

Scheme構文: **build-exp** ($callk k proc (arg ...)) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-build_002dexp-9)

Scheme構文: **build-exp** ($callk k proc args) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-build_002dexp-10)

Scheme構文: **build-exp** ($primcall name param (arg ...)) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-build_002dexp-11)

Scheme構文: **build-exp** ($primcall name param args) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-build_002dexp-12)

Scheme構文: **build-exp** ($values (arg ...)) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-build_002dexp-13)

Scheme構文: **build-exp** ($values args) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-build_002dexp-14)

Scheme構文: **build-exp** ($prompt escape? タグハンドラ) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-build_002dexp-15)

Scheme構文: **build-term** ($branch kf kt src op param (arg ...)) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-build_002dterm-2)

Scheme構文: **build-term** ($branch kf kt src op param args) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-build_002dterm-3)

Scheme構文: **build-term** ($switch kf kt\* src arg) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-build_002dterm-4)

Scheme構文: **build-term** ($throw src op param (arg ...)) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-build_002dterm-5)

Scheme構文: **build-term** ($throw src op param args) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-build_002dterm-6)

Scheme構文: **build-term** ($prompt k kh src escape? tag) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-build_002dterm-7)

Scheme構文: **build-cont** ,val [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-build_002dcont)

Scheme構文: **build-cont** ($kargs (name ...) (sym ...) term) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-build_002dcont-1)

Scheme構文: **build-cont** ($kargs names syms term) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-build_002dcont-2)

Scheme構文: **build-cont** ($kreceive req rest kargs) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-build_002dcont-3)

Scheme構文: **build-cont** ($kfun src meta self ktail kclause) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-build_002dcont-4)

Scheme構文: **build-cont** ($kclause ,arity kbody kalt) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-build_002dcont-5)

Scheme構文: **build-cont** ($kclause (req opt rest kw aok?) kbody) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-build_002dcont-6)

CPSの項、式、または継続を構築します。

その他にも、いくつか雑多なインターフェースがあります。

スキーム手順: **make-arity** req opt rest kw allow-other-keywords? [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002darity)

`$arity` 個のオブジェクトのための手続き型コンストラクタ。

Scheme構文: **rewrite-term** val (pat term) ... [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-rewrite_002dterm)

Scheme構文: **rewrite-exp** val (pat exp) ... [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-rewrite_002dexp)

Scheme構文: **rewrite-cont** val (pat cont) ... [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-rewrite_002dcont)

`match` を使用して、val をパターン pat... の系列と照合します。照合句の本体は、それぞれ `build-term`、`build-exp`、または `build-cont` の構文のテンプレートである必要があります。

* * *

次へ: [CPS のコンパイル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compiling-CPS)、前: [CPS の構築](https://doc.guix.gnu.org/guile/latest/en/guile.html#Building-CPS)、上: [継続パス スタイル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Continuation_002dPassing-Style) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.4.4.4 CPS Soup [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#CPS-Soup-1)

GuileのCPS言語におけるプログラムは、いわば「スープ」のようなものだと表現できます。なぜなら、プログラム内のすべての継続は、どの関数やスコープに属するかを示す明示的なマーカーなしに、同じ「鍋」に混ぜ合わされるからです。CPSにおけるプログラムは、継続ラベルから継続値へのマップです。序論で述べたように、継続ラベルは整数です。ラベルは負の値をとることはできません。

慣例として、ラベル 0 はプログラムのエントリの `$kfun` 継続にマッピングされるべきであり、これは引数のない関数であるべきです。関数の本体は、関数のエントリから到達可能なラベル付き継続で構成されます。プログラムは、高階 CPS では `$fun` と `$rec` を介して、または一階 CPS では `$const-fun`、`$callk`、および割り当てられたクロージャを介して、他の関数を参照できます。プログラムは論理的に、エントリ関数から到達可能なすべての関数のすべての継続を含みます。コンパイラ パスでは、到達不可能な継続がプログラムに残る場合があります。後続のコンパイラ パスでは、変換と解析が到達可能な継続のみを考慮するようにする必要があります。ただし、到達不可能な継続を含めることが有効な継続の変換に影響を与えない場合は、変換がすべての継続に対して実行されても問題ありません。

「soup」自体は、整数キーに特化した関数型配列マップトライ木である_intmap_として実装されています。intmapは、整数を任意の値に関連付けます。現在、intmapはコンパイラのCPSフェーズでのみ使用されるプライベートデータ構造です。intmapを操作するには、`(language cps intmap)`モジュールをロードしてください。

(use-modules (language cps intmap))

Intmapは関数型データ構造なので、コンストラクタのようなものはありません。空のIntmapから始めて、そこにエントリを追加していくだけです。

(intmap? empty-intmap) ⇒ #t
(define x (intmap-add empty-intmap 42 "hi"))
(intmap? x) ⇒ #t
(intmap-ref x 42) ⇒ "hi"
(intmap-ref x 43) ⇒ エラー: 43 が存在しません
(intmap-ref x 43 (lambda (k) "yo!")) ⇒ "yo"
(intmap-add x 42 "hej") ⇒ エラー: 42 は既に存在します

`intmap-ref`と`intmap-add`はintmapインターフェースの中核となる機能です。その他にも、指定されたキーに関連付けられた値を置き換える`intmap-replace`（ただし、そのキーが既に存在していることが前提）と、intmapからキーを削除する`intmap-remove`があります。

Intmapは、和集合や積集合などの集合演算に適したツリー状の構造を持つため、二項演算の`intmap-union`と`intmap-intersect`も用意されています。結果がどちらかの引数と等しい場合、その引数はそのまま返されます。そのため、`eq?`で確認するだけで、集合演算によって新しい結果が生成されたかどうかを検出できます。この特性により、Intmapは不動点の計算に役立ちます。

キーが両方の intmap に存在し、関連付けられた値が `eq?` の意味で同じでない場合、結果の値は「meet」プロシージャによって決定されます。これは、`intmap-union`、`intmap-intersect`、および `intmap-add`、`intmap-replace` などの関数のオプションの最後の引数です。meet プロシージャは 2 つの値とともに呼び出され、ドメイン固有の方法で交差または結合された値を返す必要があります。meet プロシージャが指定されていない場合、デフォルトの meet プロシージャはエラーを発生させます。

intmap の値のセットを走査するには、`intmap-next` と `intmap-prev` という手順があります。たとえば、intmap x に 42 を何らかの値にマッピングするエントリが 1 つある場合、次のようになります。

(intmap-next x) ⇒ 42
(intmap-next x 0) ⇒ 42
(intmap-next x 42) ⇒ 42
(intmap-next x 43) ⇒ #f
(intmap-prev x) ⇒ 42
(intmap-prev x 42) ⇒ 42
(intmap-prev x 41) ⇒ #f

また、intmap内のキーと値を最小値から最大値の順に折り畳む`intmap-fold`プロシージャと、その逆の順序で折り畳む`intmap-fold-right`プロシージャもあります。これらのプロシージャは最大3つのシード値を受け取ることができます。折り畳みプロシージャが返す値の数は、シード値の数と同じです。

(define q (intmap-add (intmap-add empty-intmap 1 2) 3 4))
(intmap-fold acons q '()) ⇒ ((3 . 4) (1 . 2))
(intmap-fold-right acons q '()) ⇒ ((1 . 2) (3 . 4))

intmap のエントリが更新 (削除、追加、または変更) されると、元の intmap と同じ構造を持つ新しい intmap が作成されます。この操作により、既存の計算結果が将来の計算によって影響を受けないことが保証されます。つまり、ユーザー コードから変更が見えることはありません。これはコンパイラ データ構造にとって非常に優れた特性であり、変換前のプログラムのコピーを保持し、変換後のプログラムを構築する際にそれを使用することができます。intmap の更新は、intmap のサイズに対して O(log n) の時間で実行できます。

しかし、O(log n) の割り当てコストは、特に intmap をその場で更新できることがわかっている場合には、大きすぎる場合があります。例として、整数 1 から 100 を整数 42 から 141 にマッピングする intmap があるとします。このマップの各値に 1 を加算して変換したいとします。`(language cps utils)` モジュールには既に効率的な `intmap-map` プロシージャがありますが、それを知らなかった場合は、次のようにするかもしれません。

(define (intmap-increment map)
(let lp ((k 0) (map map))
(let ((k (intmap-next map k)))
(k の場合)
(let ((v (intmap-ref map k)))
(lp (1+ k) (intmap-replace map k (1+ v))))
地図））））

`intmap-replace` によって生成される中間値はプログラムから完全に見えないことに注意してください。必要なのは `intmap-replace` の最終結果の値だけです。残りの値は最終値と状態を共有しても構わないので、その場で更新できます。Guile は、Clojure の transient インターフェース ([http://clojure.org/transients](http://clojure.org/transients)) に触発された _transient intmaps_ を介して、このようなインターフェースを提供します。

インプレースプロシージャ `intmap-add!` および `intmap-replace!` は、一時的な intmap を返します。これらのインプレースプロシージャのいずれかが通常の永続 intmap に対して呼び出されると、新しい一時的な intmap が作成されます。これは O(1) の操作です。その他の点では、インターフェースは永続版の `intmap-add` および `intmap-replace` と同様です。インプレースプロシージャが一時的な intmap に対して呼び出されると、intmap はその場で変更され、同じ値が返されます。

`intmap-add`のような永続的な操作が一時的なintmapに対して呼び出されると、その一時的なintmapの可変サブ構造が永続的としてマークされ、`intmap-add`は、元の一時的なintmapと構造を共有するものの状態を共有しない新しい永続的なintmapに対して実行されます。一時的なintmapを変更すると、変更を反映させるのに十分なコピーが発生しますが、そのサブ構造の一部が既に一時的なintmapによって「所有」されている場合は、それ以上のコピーは必要ありません。

`intmap-increment` をより効率的にするために、トランジェントを使用できます。変更された 2 つの要素は、**このように**マークされています。

(define (intmap-increment map)
(let lp ((k 0) (map map))
(let ((k (intmap-next map k)))
(k の場合)
(let ((v (intmap-ref map k)))
(lp (1+ k) (intmap-replace! map k (1+ v))))
(persistent-intmap マップ)))))

プログラムの他の部分に可変性が漏れないように、`persistent-intmap` プロシージャを使用して結果を永続的にタグ付けしてください。さらに念を入れるなら、入力マップに対して `persistent-intmap` を呼び出すことで、入力マップが既に一時的である場合、`intmap-increment` 本体での変更が入力値に影響を与えないようにすることができます。

要約すると、CPSにおけるプログラムは、値が継続であるintmapです。CPS値を扱うための便利な機能については、`(language cps utils)`のソースコードを参照してください。

* * *

前へ: [CPS Soup](https://doc.guix.gnu.org/guile/latest/en/guile.html#CPS-Soup)、上へ: [Continuation-Passing Style](https://doc.guix.gnu.org/guile/latest/en/guile.html#Continuation_002dPassing-Style) \[[Contents](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[Index](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "Index")\]

#### 9.4.4.5 CPS のコンパイル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compiling-CPS-1)

GuileでCPSをコンパイルするには、変換、最適化、コード生成の3つの段階があります。

CPS変換とは、高水準言語をCPSにコンパイルするプロセスです。ソース言語は直接この変換を行うことも、Tree-ILに変換して（おそらくこちらの方が簡単でしょう）、後でTree-ILにCPSへの変換を任せることもできます。Tree-ILを経由する利点は、部分評価などのTree-IL最適化パスを実行できることです。また、Tree-ILからCPSへのコンパイラは、代入変換を処理します。代入されたローカル変数（Tree-ILでは`<lexical-set>`であるローカル変数）は、ヒープ上のボックス化された値に変換されます。[変数とVM](https://doc.guix.gnu.org/guile/latest/en/guile.html#Variables-and-the-VM)を参照してください。

CPS変換後、GuileはCPSに対していくつかの最適化処理を実行します。Guileにおける最適化の大部分はCPS言語上で行われます。唯一の大きな例外は部分評価で、これは歴史的な理由からTree-IL上で行われます。

CPSで行われる主な最適化は継続化です。これは、常に同じ継続で呼び出される関数を関数本体に直接組み込むものです。これにより、さらなる最適化の余地が生まれ、プロシージャ呼び出しが`goto`に変換されます。また、再帰関数ネストからループを作成することもできます。Guileは、デッドコード除去、共通部分式除去、ループの剥離と不変コードの移動、範囲推論と型推論も行います。

残りの最適化パスは、実際にはクリーンアップと正規化です。CPSは高水準言語と低水準バイトコードの間のギャップを埋め、コンパイルプロセスの大部分をソース間変換として表現できるようにします。クロージャ変換もその一例で、関数内で自由変数への参照はクロージャ参照に変換され、関数はクロージャに変換されます。さらにいくつかのパスがあり、項に残るプリムコールが仮想マシンに対応する命令を持つものだけであり、それらの継続が正しい数の値を受け取ることを保証しています。

最後に、CPSコンパイラのバックエンドは、各関数ごとにバイトコードを1つずつ出力します。そのためには、関数内のすべての箇所で生存変数のセットを特定します。この生存情報を使用して、各変数にスタックスロットを割り当て、変数がその生存期間中、シャッフルされることなく1つのスロットに存在できるようにします。（もちろん、生存期間が異なる変数はスロットを共有できます。）最後に、バックエンドは、関数内の各継続に対して、通常は1つのVM命令であるコードを生成します。

* * *

次へ: [新しい高水準言語の作成](https://doc.guix.gnu.org/guile/latest/en/guile.html#Writing-New-High_002dLevel-Languages)、前: [継続渡しスタイル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Continuation_002dPassing-Style)、上: [仮想マシンへのコンパイル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compiling-to-the-Virtual-Machine) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次内容")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.4.5 バイトコード [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytecode-1)

前述のとおり、Guile はすべてのコードをバイトコードにコンパイルし、そのバイトコードは ELF イメージに格納されます。Guile における ELF の使用方法の詳細については、[オブジェクト ファイル フォーマット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Object-File-Format) を参照してください。

バイトコードイメージを生成するために、Guileはアセンブラとリンカを提供します。

`(system vm assembler)`モジュールで定義されているアセンブラは、比較的単純な命令型インターフェースを備えています。アセンブラをインスタンス化するための`make-assembler`関数と、各種類の命令を出力するための`emit-inst`プロシージャ群を提供します。

`emit-inst`プロシージャは、実際にはVMの機械可読な記述からコンパイル時に生成されます。特定のオペランド型に関するいくつかの例外を除き、emitプロシージャの各オペランドは、対応する命令のオペランドに対応します。

`allocate-words` については、[メモリアクセス命令](https://doc.guix.gnu.org/guile/latest/en/guile.html#Memory-Access-Instructions) を参照してください。ドキュメントには次のように記載されています。

指示: **allocate-words** `s12:dst s12:nwords` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-allocate_002dwords-1)

したがって、emit手順は次の形式になります。

Scheme Procedure: **emit-allocate-words** asm dst nwords [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-emit_002dallocate_002dwords)

すべてのemitプロシージャは、最初の引数としてアセンブラを受け取り、有用な値を返しません。

引数の型はオペランドの型に依存します。[命令セット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instruction-Set)を参照してください。ほとんどは制限された範囲内の整数ですが、ラベルは一般的に不透明なシンボルで表現されます。命令に対応するエミッタの他に、アセンブラモジュールにはいくつかの補助関数が定義されています。

スキーム手順: **emit-label** asm ラベル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-emit_002dlabel)

現在のプログラムポイントにラベルを定義します。

スキーム手順: **emit-source** asm ソース [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-emit_002dsource)

ソースを現在のプログラムポイントに関連付けます。

スキーム手順: **emit-cache-ref** asm dst キー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-emit_002dcache_002dref)

スキーム手順: **emit-cache-set!** asm key val [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-emit_002dcache_002dset_0021)

コンパイル単位キャッシュを実装するためのマクロ命令。コンパイル単位ごとに、キーに対応する単一のキャッシュセルが割り当てられます。

Scheme Procedure: **emit-load-constant** asm dst constant [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- emit_002dload_002dconstant)

Schemeデータ定数をdstにロードします。

スキーム手順: **emit-begin-program** asm ラベルのプロパティ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-emit_002dbegin_002dprogram)

スキーム手順: **emit-end-program** asm [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-emit_002dend_002dprogram)

指定されたラベルとメタデータプロパティを使用して、プロシージャの範囲を区切ります。

Scheme Procedure: **emit-load-static-procedure** asm dst label [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-emit_002dload_002dstatic_002dprocedure)

指定されたラベルを持つプロシージャをローカル変数dstにロードします。このマクロ命令は、自由変数を持たないプロシージャ（クロージャではないプロシージャ）でのみ使用してください。

Scheme Procedure: **emit-begin-standard-arity** asm req nlocals alternate [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-emit_002dbegin_002dstandard_002darity)

Scheme Procedure: **emit-begin-opt-arity** asm req opt rest nlocals alternate [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-emit_002dbegin_002dopt_002darity)

Scheme Procedure: **emit-begin-kw-arity** asm req opt rest kw-indices allow-other-keys? nlocals alternate [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-emit_002dbegin_002dkw_002darity)

Scheme Procedure: **emit-end-arity** asm [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-emit_002dend_002darity)

手続きの条項を区切る。

リンカーは複雑なものです。その仕組みに興味のあるハッカーは、イアン・ランス・テイラーのリンカーに関する一連の記事を読むと良いでしょう。インターネットで検索すれば簡単に見つかるはずです。ユーザーの視点から見ると、制御できるのは、結果として得られるイメージをファイルに書き出すかどうかという1つのつまみだけです。ユーザーがコンパイラオプションの一部として `#:to-file? #t` を渡すと ([The Scheme Compiler](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Scheme-Compiler) を参照)、リンカーは結果として得られるセグメントをページ境界に揃えますが、そうでない場合は揃えません。

Scheme Procedure: **link-assembly** asm #:page-aligned?=#t [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-link_002dassembly)

ELFイメージをリンクし、バイトベクトルを返します。page-aligned?がtrueの場合、Guileは異なるプロセス間でのコード共有を最大化するために、異なるパーミッションを持つセグメントをページサイズの境界に整列します。それ以外の場合は、アドレス空間の消費を最小限に抑えるためにパディングが最小限に抑えられます。

イメージをディスクに書き込むには、`(ice-9 binary-ports)` の `put-bytevector` を使用するだけです。

オブジェクトコードを擬似言語`value`にコンパイルするには、objcodeをプログラムにロードし、コンパイル環境に応じてそのサンクを実行します。通常、環境はコンパイラを通じて透過的に伝播されますが、ユーザーはモジュールとしてコンパイル環境を手動で指定することもできます。イメージをロードする手順は、`(system vm loader)`モジュールにあります。

([use-modules](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-use_002dmodules) ([system](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-system) vm loader))

Scheme変数: **load-thunk-from-file** ファイル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-load_002dthunk_002dfrom_002dfile)

C 関数: **scm\_load\_thunk\_from\_file** (ファイル) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- scm_005fload_005fthunk_005ffrom_005ffile)

指定されたファイル名のファイルからオブジェクトコードを読み込みます。ファイルは`mmap`関数によってメモリにマッピングされるため、非常に高速な操作です。

Scheme 変数: **load-thunk-from-memory** bv [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-load_002dthunk_002dfrom_002dmemory)

C 関数: **scm\_load\_thunk\_from\_memory** (bv) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fload_005fthunk_005ffrom_005fmemory)

バイトベクターからオブジェクトコードをロードします。埋め込まれたScheme値の適切なアライメントを確保するため、データはバイトベクターからコピーされます。

さらに、指定されたポインタに対応するELFイメージを検索したり、マッピングされたすべてのELFイメージを一覧表示したりする手順も用意されています。

Scheme 変数: **find-mapped-elf-image** ptr [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-find_002dmapped_002delf_002dimage)

整数値ptrが与えられた場合、そのポインタを含むELFイメージを検索し、バイトベクトルとして返します。イメージが見つからない場合は、`#f`を返します。このルーチンは主にデバッガやその他のイントロスペクティブツールで使用されます。

Scheme変数: **all-mapped-elf-images** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-all_002dmapped_002delf_002dimages)

マップされたすべてのELFイメージをバイトベクトルのリストとして返します。

* * *

次へ: [コンパイラの拡張](https://doc.guix.gnu.org/guile/latest/en/guile.html#Extending-the-Compiler)、前: [バイトコード](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytecode)、上: [仮想マシンへのコンパイル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compiling-to-the-Virtual-Machine) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.4.6 新しい高水準言語の作成 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Writing-New-High_002dLevel-Languages-1)

Guileのコンパイラシステムに新しい言語langを統合するには、言語定義と、それを処理するパーサー、コンパイラ、その他のルーチンを参照するモジュール`(language lang spec)`を作成する必要があります。`(language brainfuck)`のモジュール階層は、この方法を分かりやすく示すための非常に基本的なBrainfuckの実装を定義しています。Brainfuck言語自体の詳細については、例えば[http://en.wikipedia.org/wiki/Brainfuck](http://en.wikipedia.org/wiki/Brainfuck)を参照してください。

* * *

前へ: [新しい高水準言語の作成](https://doc.guix.gnu.org/guile/latest/en/guile.html#Writing-New-High_002dLevel-Languages)、上へ: [仮想マシンへのコンパイル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compiling-to-the-Virtual-Machine) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.4.7 コンパイラの拡張 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Extending-the-Compiler-1)

ここからは、マニュアルのこれまでの無味乾燥なトーンから少し外れてみましょう。正直に言ってください。コンパイラの内部構造マニュアルをここまで読み進めてきたあなたは、間違いなく熱狂的なファンです。大学の授業で物足りなさを感じたのかもしれませんし、あるいはコンピュータサイエンスの聖域とも言えるコンパイラをハッキングしたいという願望をずっと抱いていたのかもしれません。いずれにせよ、あなたは素晴らしい仲間たちに囲まれ、絶好の機会に恵まれています。Guileのコンパイラは、あなたの助けを必要としているのです。

Guile コンパイラの改良には多くの方法があります。速度面で最も重要な改良はおそらく、グローバルレジスタ割り当てによる最適化された事前ネイティブコンパイルでしょう。最初の試みとしては、コンパイラを拡張してバイトコードに加えてマシンコードも出力し、`instrument-entry` バイトコードで参照される対応する JIT データ構造を事前に埋める方法が考えられます。[計測手順](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instrumentation-Instructions) を参照してください。

コンパイラは上位レベルの機能も必要としており、新しい高水準コンパイラを追加する必要があります。JavaScriptとEmacs Lispはほぼ完成していますが、さらに改良の余地があります。Luaもあれば嬉しいですが、皆さんが興味を持った言語であれば何でも歓迎します。

コンパイラはハッキングするためのものであって、賞賛したり文句を言ったりするためのものではない。さあ、始めよう！

* * *

次へ: [概念インデックス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Concept-Index)、前: [Guile 実装](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guile-Implementation)、上: [Guile リファレンス マニュアル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Top) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
