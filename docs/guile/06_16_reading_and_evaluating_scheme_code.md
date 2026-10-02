### 6.16 Scheme コードの読み取りと評価 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Reading-and-Evaluating-Scheme-Code)

この章では、実行時にSchemeコードを読み込み、ロードし、評価し、コンパイルすることに関わるGuileの関数について説明します。

* [Scheme構文：標準とGuile拡張機能](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scheme-Syntax)
* [スキームコードの読み込み](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scheme-Read)
* [コンパイラのための Scheme コードの読み方](https://doc.guix.gnu.org/guile/latest/en/guile.html#Annotated-Scheme-Read)
* [Scheme値の書き込み](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scheme-Write)
* [オンザフライ評価の手順](https://doc.guix.gnu.org/guile/latest/en/guile.html#Fly-Evaluation)
* [Schemeコードのコンパイル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compilation)
* [ファイルからのスキームコードの読み込み](https://doc.guix.gnu.org/guile/latest/en/guile.html#Loading)
* [ロードパス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Load-Paths)
* [ソースファイルの文字エンコーディング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Character-Encoding-of-Source-Files)
* [遅延評価](https://doc.guix.gnu.org/guile/latest/en/guile.html#Delayed-Evaluation)
* [ローカル評価](https://doc.guix.gnu.org/guile/latest/en/guile.html#Local-Evaluation)
* [ローカル インクルージョン](https://doc.guix.gnu.org/guile/latest/en/guile.html#Local-Inclusion)
* [サンドボックス評価](https://doc.guix.gnu.org/guile/latest/en/guile.html#Sandboxed-Evaluation)
* [REPLサーバー](https://doc.guix.gnu.org/guile/latest/en/guile.html#REPL-Servers)
* [協調型REPLサーバー](https://doc.guix.gnu.org/guile/latest/en/guile.html#Cooperative-REPL-Servers)

* * *

次へ: [スキームコードの読み込み](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scheme-Read)、上へ: [スキームコードの読み込みと評価](https://doc.guix.gnu.org/guile/latest/en/guile.html#Read_002fLoad_002fEval_002fCompile) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.16.1 Scheme構文: 標準とGuile拡張機能 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scheme-Syntax_003a-Standard-and-Guile-Extensions)

* [式構文](https://doc.guix.gnu.org/guile/latest/en/guile.html#Expression-Syntax)
* [コメント](https://doc.guix.gnu.org/guile/latest/en/guile.html#Comments)
* [ブロックコメント](https://doc.guix.gnu.org/guile/latest/en/guile.html#Block-Comments)
* [大文字小文字の区別](https://doc.guix.gnu.org/guile/latest/en/guile.html#Case-Sensitivity)
* [キーワード構文](https://doc.guix.gnu.org/guile/latest/en/guile.html#Keyword-Syntax)
* [リーダー拡張機能](https://doc.guix.gnu.org/guile/latest/en/guile.html#Reader-Extensions)

* * *

次へ: [コメント](https://doc.guix.gnu.org/guile/latest/en/guile.html#Comments)、上へ: [Scheme構文: 標準とGuile拡張](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scheme-Syntax) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.16.1.1 式の構文 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Expression-Syntax-1)

評価される式は、以下のいずれかの形式をとります。

`シンボル`

シンボルは逆参照によって評価されます。そのシンボルのバインディングが検索され、その値が使用されます。たとえば、

(define x 123)
x ⇒ 123

`(proc args…)`

括弧で囲まれた式は関数呼び出しです。procと各引数が評価され、その後、(procが評価された結果の)関数がそれらの引数とともに呼び出されます。

procと引数が評価される順序は規定されていないため、副作用のある式を使用する際は注意が必要です。

(最大 1 2 3) ⇒ 3

(define (get-some-proc) min)
((get-some-proc) 1 2 3) ⇒ 1

マクロ呼び出しでも同様の括弧形式が使用されますが、その場合は引数は評価されません。詳細については、マクロの説明を参照してください（ [マクロ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Macros)および[構文規則マクロ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Syntax-Rules)を参照）。

定数

数値、文字列、文字、ブール定数は「それ自身に対して」評価されるため、リテラルとして表現できます。

123 ⇒ 123
99.9 ⇒ 99.9
「こんにちは」⇒「こんにちは」
#\\z ⇒ #\\z
#t ⇒ #t

アプリケーションは、リテラル文字列を変更しようとしてはならないことに注意してください。リテラル文字列は読み取り専用メモリに格納されている可能性があるためです。

`(引用データ)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-quote)

`'データ`

引用符は、リテラルシンボル（変数参照の代わりに）、リテラルリスト（関数呼び出しの代わりに）、またはリテラルベクトルを取得するために使用されます。`'` は単に `quote` 形式の省略形です。たとえば、

'x ⇒ x
'(1 2 3) ⇒ (1 2 3)
'#(1 (2 3) 4) ⇒ #(1 (2 3) 4)
(引用 x) ⇒ x
(引用 (1 2 3)) ⇒ (1 2 3)
(引用 #(1 (2 3) 4)) ⇒ #(1 (2 3) 4)

アプリケーションは、`quote`形式から取得したリテラルリストやベクトルを変更しようとしてはならないことに注意してください。これらは読み取り専用メモリに格納されている可能性があるためです。

`(準引用データ)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-quasiquote )

`` `データ ``

バッククォートによる擬似引用は`quote`に似ていますが、選択された部分式のみが評価されます。これは、大部分が定数であるものの、特定の箇所で式を置換する必要があるリストやベクトル構造を構築するのに便利な方法です。

適切な `list`、`cons`、または `vector` 呼び出しを使用すれば常に同じ効果が得られますが、準クォートの方が簡単な場合が多いです。

`(unquote expr)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-unquote)

`,expr`

準引用データ内では、`unquote` または `,` は評価して挿入する式を示します。カンマ構文 `,` は単に `unquote` 形式の省略形です。たとえば、

\`(1 2 (\* 9 9) 3 4) ⇒ (1 2 (\* 9 9) 3 4)
\`(1 2 ,(\* 9 9) 3 4) ⇒ (1 2 81 3 4)
\`(1 (unquote (+ 1 1)) 3) ⇒ (1 2 3)
\`#(1 ,(/ 12 2)) ⇒ #(1 6)

`(unquote-splicing expr)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-unquote_002dsplicing)

`,@expr`

準引用データ内では、`unquote-splicing` または `,@` は、評価される式と、返されたリストの要素を挿入することを示します。式はリストに評価されなければなりません。「カンマアット」構文 `,@` は、単に `unquote-splicing` 形式の省略形です。

(define x '(2 3))
\`(1 ,x 4) ⇒ (1 (2 3) 4)
\`(1 ,@x 4) ⇒ (1 2 3 4)
\`(1 (unquote-splicing (map 1+ x))) ⇒ (1 3 4)
\`#(9 ,@x 9) ⇒ #(9 2 3 9)

`,@` は通常の `,` とは異なり、ネストされた階層が 1 レベル分削除されることに注意してください。`,@` では返されたリストの要素が挿入されますが、`,` ではリスト自体が挿入されます。

* * *

次へ: [ブロックコメント](https://doc.guix.gnu.org/guile/latest/en/guile.html#Block-Comments)、前: [式構文](https://doc.guix.gnu.org/guile/latest/en/guile.html#Expression-Syntax)、上: [Scheme構文: 標準とGuile拡張](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scheme-Syntax) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.16.1.2 コメント [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Comments-1)

Schemeソースファイル内のコメントは、セミコロン（;）で開始して記述します。コメントは行末まで続きます。コメントは任意の列から開始でき、Schemeコードと同じ行に挿入することも可能です。

; コメント
;; コメントも
(define x 1) ; 式の後のコメント
(let ((y 1))
;; 何かを表示します。
([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) y)
;;; 左余白にコメントがあります。
([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) ([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) y 1)))

行内の式の後に続くコメントにはセミコロンを1つ、コードのようにインデントされたコメントにはセミコロンを2つ、インデントされたコードブロック内であっても0列目から始まるコメントにはセミコロンを3つ使用するのが一般的です。この慣例は、EmacsのSchemeモードでコードをインデントする際に使用されます。

* * *

次へ: [大文字小文字の区別](https://doc.guix.gnu.org/guile/latest/en/guile.html#Case-Sensitivity)、前: [コメント](https://doc.guix.gnu.org/guile/latest/en/guile.html#Comments)、上: [Scheme構文: 標準とGuile拡張](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scheme-Syntax) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.16.1.3 ブロックコメント [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Block-Comments-1)

R5RSで定義されている標準的な行コメントに加えて、Guileには複数行コメント用の別のコメントタイプがあり、これを「ブロックコメント」と呼びます。このタイプのコメントは文字シーケンス「#!」で始まり、文字「!#」で終わります。

これらのコメントは、Scheme シェル scsh のブロック コメントと互換性があります ([Scheme シェル (scsh)](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Scheme-shell-_0028scsh_0029) を参照)。文字 `#!` は、シェル スクリプトで、スクリプトを実行するプログラム名が同じ行に続くことを示すために使用される特殊な文字であるため選択されました。

したがって、Guileのスクリプトはしばしば次のように始まります。

#! /usr/local/bin/guile \-s
!#

Guile スクリプトの詳細については、スクリプトのセクションを参照してください ([Guile スクリプト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guile-Scripting))。

同様に、Guile（バージョン2.0以降）は、R6RSおよび[SRFI-30](http://srfi.schemers.org/srfi-30/srfi-30.html)で規定されているネストされたブロックコメントをサポートしています。

([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) 1 #| これはネストされたブロックコメントです |# 2)
⇒ 3

後方互換性のために、この構文は `read-hash-extend` で上書きできます ([`read-hash-extend`](https://doc.guix.gnu.org/guile/latest/en/guile.html#Reader-Extensions) を参照)。

コメントの内容がコードの解釈に影響を与える特殊なケースが1つあります。ソースファイルの最初の数行に`coding: utf-8`のような文字エンコーディング宣言があると、Guileのデフォルトのリーダーに対して、このソースコードファイルがASCIIではないことを示します。詳細については、[ソースファイルの文字エンコーディング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Character-Encoding-of-Source-Files)を参照してください。

* * *

次へ: [キーワード構文](https://doc.guix.gnu.org/guile/latest/en/guile.html#Keyword-Syntax)、前: [ブロックコメント](https://doc.guix.gnu.org/guile/latest/en/guile.html#Block-Comments)、上: [Scheme構文: 標準とGuile拡張](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scheme-Syntax) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.16.1.4 大文字小文字の区別 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Case-Sensitivity-1)

R5RSで定義されているSchemeは、シンボルを読み込む際に大文字小文字を区別しません。一方、Guileはデフォルトで大文字小文字を区別するため、識別子は

狡猾な
ガイル・ワジー

R5RSスキームでは同じだが、ガイルでは異なる。

Guileでは、リーダーオプション`case-insensitive`を設定することで、大文字小文字の区別を無効にすることができます。リーダーオプションの詳細については、[スキームコードの読み取り](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scheme-Read)を参照してください。

([read-enable](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002denable) 'case-insensitive)

また、ファイル内にリーダーディレクティブ`#!fold-case`（または`#!no-fold-case`）を配置することで、単一ファイル内での大文字小文字の区別を無効（または有効）にすることも可能です。

* * *

次へ: [リーダー拡張機能](https://doc.guix.gnu.org/guile/latest/en/guile.html#Reader-Extensions)、前: [大文字小文字の区別](https://doc.guix.gnu.org/guile/latest/en/guile.html#Case-Sensitivity)、上: [Scheme構文: 標準とGuile拡張機能](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scheme-Syntax) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.16.1.5 キーワード構文 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Keyword-Syntax-1)

* * *

前へ: [キーワード構文](https://doc.guix.gnu.org/guile/latest/en/guile.html#Keyword-Syntax)、上へ: [Scheme構文: 標準とGuile拡張](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scheme-Syntax) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Content s "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.16.1.6 リーダー拡張機能 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Reader-Extensions-1)

Scheme Procedure: **read-hash-extend** chr proc [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002dhash_002dextend)

C 関数: **scm\_read\_hash\_extend** (chr, proc) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fread_005fhash_005fextend)

文字シーケンス「#」と「chr」で始まる式を読み込むためのプロシージャ「proc」をインストールします。procは、文字「chr」と、さらにデータを読み取るポートの2つの引数で呼び出されます。返されるオブジェクトは、「read」の戻り値になります。procに「#f」を渡すと、以前の設定が削除されます。

* * *

次へ: [コンパイラのための Scheme コードの読み取り](https://doc.guix.gnu.org/guile/latest/en/guile.html#Annotated-Scheme-Read)、前: [Scheme 構文: 標準と Guile 拡張](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scheme-Syntax)、上: [Scheme コードの読み取りと評価](https://doc.guix.gnu.org/guile/latest/en/guile.html#Read_002fLoad_002fEval_002fCompile) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次内容")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.16.2 リーディングスキームコード [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Reading-Scheme-Code)

Scheme手順: **read** \[port\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read-1)

C 関数: **scm\_read** (ポート) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fread)

入力ポート port から S 式を読み取ります。port が指定されていない場合は、現在の入力ポートから読み取ります。次のトークンの前の空白はすべて破棄されます。

GuileのSchemeリーダーの動作は、読み取りオプションを操作することで変更できます。

Scheme手順: **read-options** \[setting\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002doptions)

グローバル読み取りオプションの現在の設定を表示します。設定が省略された場合は、現在の読み取りオプションの短縮形のみが表示されます。設定が「help」記号の場合は、オプションの完全な説明が表示されます。

利用可能なオプションとそのデフォルト値は、プロンプトで`read-options`を実行することで確認できます。

scheme@(guile-user)[\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e) ([read-options](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002doptions))
（角括弧内のキーワード #f ポジション）
scheme@(guile-user)[\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e) ([read-options](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002doptions) 'help)
位置 はい ソースコード式の位置を記録します。
大文字小文字を区別しない いいえ 記号を小文字に変換します。
キーワード #f キーワード認識のスタイル: #f、'prefix または 'postfix。
r6rs-hex-escapes no R6RS 可変長文字と [string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string) の 16 進エスケープを使用します。
角括弧 はい R6RS との互換性のために、\`\[' と \`\]' を括弧として扱います。
hungry-eol-escapes no 文字列では、先頭の空白文字を消費します
行末から脱出しました。
curly-infix いいえ SRFI-105 の curly infix 式をサポートしていません。
r7rs-symbols は R7RS |...| [symbol](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-symbol) 表記法をサポートしていません。
バイト文字列 SRFI-207 をサポートしていません #u8"\\xce;\\xbb; calculus" バイト文字列

Guileには、ポートごとに読み取りオプションを設定するための暫定的なメカニズムも含まれていることに注意してください。たとえば、リーダーが`#!fold-case`または`#!no-fold-case`リーダーディレクティブに遭遇すると、ポートで`case-insensitive`読み取りオプションが設定（または解除）されます。同様に、`#!curly-infix`リーダーディレクティブはポートで`curly-infix`読み取りオプションを設定し、`#!curly-infix-and-bracket-lists`はポートで`curly-infix`を設定し、`square-brackets`を解除します（[SRFI-105 Curly-infix expressions.](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d105)を参照）。現在、ポートごとの読み取りオプションにアクセスしたり設定したりする他の方法はありません。

ブール値のオプションは、`read-enable`と`read-disable`で切り替えることができます。ブール値ではない`keywords`オプションは、`read-set!`を使用して設定する必要があります。

Scheme Procedure: **read-enable** option-name [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002denable)

Scheme Procedure: **read-disable** option-name [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002ddisable)

Scheme構文: **read-set!** オプション名 値 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002dset_0021)

読み取りオプションを変更します。`read-enable`はブール値オプションと組み合わせて使用し、オプションを有効にします。`read-disable`はオプションを無効にします。

`read-set!` は、オプションを特定の値に設定するために使用できます。歴史的な経緯から、このマクロは引用符で囲まれていないオプション名を想定しています。

例えば、`read` で全てのシンボルを小文字に変換する（おそらく古い Scheme コードとの互換性のため）には、次のように入力します。

([read-enable](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002denable ) 'case-insensitive)

`r6rs-hex-escapes` および `hungry-eol-escapes` オプションの影響の詳細については、[String Read Syntax](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Syntax) を参照してください。

`r7rs-symbols` オプションの詳細については、[シンボルの拡張読み取り構文](https://doc.guix.gnu.org/guile/latest/en/guile.html#Symbol-Read-Syntax) を参照してください。

* * *

次へ: [Scheme 値の書き込み](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scheme-Write)、前: [Scheme コードの読み取り](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scheme-Read)、上: [Scheme コードの読み取りと評価](https://doc.guix.gnu.org/guile/latest/en/guile.html#Read_002fLoad_002fEval_002fCompile) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.16.3 コンパイラのためのスキームコードの読み方 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Reading-Scheme-Code_002c-For-the-Compiler)

Schemeプログラムで何らかの問題が発生した場合、ユーザーはそれを修正する方法を知りたいと思うでしょう。そのためには、まずエラーが発生した場所を特定する必要があります。ソースコードの各構成要素にソースコードの場所を関連付け、そのソースコードの場所情報をコンパイラやインタプリタに伝達する必要があります。

そのため、Guileは`read-syntax`を提供しています。

Scheme手順: **read-syntax** \[port\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002dsyntax)

入力ポート port から S 式を読み取ります。port が指定されていない場合は、現在の入力ポートから読み取ります。

空白とコメントをスキップした後、ポートから利用可能なバイトがなくなった場合は、ファイルの終端オブジェクトを返します。[バイナリ入出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Binary-I_002fO)を参照してください。それ以外の場合は、注釈付きデータを返します。注釈付きデータとは、ソース位置をデータに関連付ける構文オブジェクトです。例:

(入力文字列「foo」を引数とする呼び出しの構文)
; ⇒ #<syntax:unknown file:1:2 foo>
(入力文字列「(foo)」を引数として呼び出し、構文を読み取る)
; ⇒
; #<構文:不明なファイル:1:0
; (#<構文不明ファイル:1:1 foo>)>

2番目の例が示すように、ペアとベクトルのすべてのフィールドも再帰的に注釈が付けられます。

ほとんどのユーザーは、構文オブジェクトをマクロのコンテキストで使用してスコープ情報を識別子に関連付ける構文オブジェクトに慣れています。[マクロ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Macros)を参照してください。ここでは、構文オブジェクトを使用してソース位置情報を任意のデータムに関連付けますが、スコープ情報は添付しません。Schemeコンパイラ（`compile`）とインタプリタ（`eval`）は、構文オブジェクトを直接入力として受け入れることができ、ソース情報を結果のコードに関連付けることができます。[Schemeコードのコンパイル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compilation)および[実行時評価の手順](https://doc.guix.gnu.org/guile/latest/en/guile.html#Fly-Evaluation)を参照してください。

Schemeコンパイラまたはインタプリタにソース位置を取得するための従来のインターフェースとして、`read-syntax`のようにデータを直接ラップするのではなく、`read`によって返される各サブデータに「ソースプロパティ」を関連付けるサイドテーブルを使用する方法があることに注意してください。この方法の欠点は、あらゆる種類のデータに注釈を付けることができないことです。詳細については、[ソースプロパティ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Source-Properties)を参照してください。

* * *

次へ: [オンザフライ評価の手順](https://doc.guix.gnu.org/guile/latest/en/guile.html#Fly-Evaluation)、前: [コンパイラのための Scheme コードの読み込み](https://doc.guix.gnu.org/guile/latest/en/guile.html#Annotated-Scheme-Read)、上: [Scheme コードの読み込みと評価](https://doc.guix.gnu.org/guile/latest/en/guile.html#Read_002fLoad_002fEval_002fCompile) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引"）\]

#### 6.16.4 Scheme値の記述 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Writing-Scheme-Values)

スキームの値はどれでもポートに書き込むことができます。ただし、すべての値を読み戻せるわけではありません（[スキームコードの読み取り](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scheme-Read)を参照）。

Scheme手順: **write** obj \[port\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-write)

obj の表現をポートに送信するか、指定されていない場合は現在の出力ポートに送信する。

出力は機械可読な形式で、`read` コマンドで読み戻すことができます（[スキームコードの読み取り](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scheme-Read) を参照）。文字列は二重引用符で囲まれ、必要に応じてエスケープ処理が施され、文字は '#\\' 表記で出力されます。

Scheme手順: **display** obj \[port\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display)

obj の表現をポートに送信するか、指定されていない場合は現在の出力ポートに送信する。

出力は人間が読みやすいように設計されており、`write` とは異なり、文字列は二重引用符やエスケープなしで出力され、文字は'#\\' 形式ではなく `write-char` に従って出力されます。

Schemeリーダーの場合と同様に、Schemeプリンターの動作に影響を与えるオプションがいくつかあります。

Scheme手順: **print-options** \[setting\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-print_002doptions)

読み取りオプションの現在の設定を表示します。設定が省略された場合は、現在の読み取りオプションの短縮形のみが表示されます。設定が「help」記号の場合は、オプションの完全な説明が表示されます。

利用可能なオプションとそのデフォルト値は、プロンプトで`print-options`を実行することで確認できます。

scheme@(guile-user)[\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e) ([print-options](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-print_002doptions))
(quote-keywordish-symbols reader highlight-suffix "}" highlight-prefix "{")
scheme@(guile-user)[\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e) ([print-options](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-print_002doptions) 'help)
highlight-prefix { 強調表示された値の前に表示する [文字列](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string)。
highlight-suffix } 強調表示された値の後に表示する [文字列](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string)。
引用キーワード記号リーダー コロンを含む記号を印刷する方法
[first](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-first) または [last](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-last) 文字として使用されます。
値 '#f' はコロンを [引用](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-not) [引用](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-quote-1) しません。
「#t」は引用し、「reader」は引用する
[when](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-when-1) 読者 [option](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-option) 'keywords' は
[not](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-not) '#f'。
escape-newlines yes 改行を \\n としてレンダリングします [when](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-when-1) 印刷時
`write` を使用します。
r7rs-symbols エスケープ記号なし R7RS を使用 |...| [symbol](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-symbol)
表記法。
バイト文字列 バイトベクトルをバイト文字列としてレンダリングしない (SRFI-207)

これらのオプションは、print-set!構文を使用して変更できます。

Scheme構文: **print-set!** オプション名 値 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-print_002dset_0021)

印刷オプションを変更します。歴史的な経緯により、`print-set!` は引用符で囲まれていないオプション名を想定するマクロです。

* * *

次へ: [Scheme コードのコンパイル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compilation)、前: [Scheme 値の書き込み](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scheme-Write)、上: [Scheme コードの読み込みと評価](https://doc.guix.gnu.org/guile/latest/en/guile.html#Read_002fLoad_002fEval_002fCompile) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.16.5 オンザフライ評価の手順 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Procedures-for-On-the-Fly-Evaluation)

Schemeには、その式をデータとして表現できるという素晴らしい特性があります。`eval`プロシージャは、Schemeのデータを受け取り、それをコードとして評価します。

Scheme手順: **eval** exp module\_or\_state [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-eval)

C 関数: **scm\_eval** (exp, module\_or\_state) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005feval)

Scheme式を表すリストexpを、module_or_stateで指定されたトップレベル環境で評価します。expが評価されている間（`primitive-eval`を使用）、module_or_stateが現在のモジュールになります。`eval`が戻ると、現在のモジュールは以前の値にリセットされます。XXX - 動的状態。例：(eval '(+ 1 2) (interaction-environment))

Scheme Procedure: **interaction-environment** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-interaction_002denvironment)

C 関数: **scm\_interaction\_environment** () [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005finteraction_005fenvironment)

実装定義のバインディングを含む環境の指定子を返します。通常、レポートに記載されているバインディングのスーパーセットが含まれます。このプロシージャの目的は、ユーザーが動的に型付けした式を実装が評価する環境を返すことです。

その他の環境については、[環境](https://doc.guix.gnu.org/guile/latest/en/guile.html#Environments)を参照してください。

もちろん、コードが常にSchemeデータとして渡されるとは限りません。これは特にGuileの他の言語実装の場合に当てはまります（[他の言語のサポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Other-Languages)を参照）。文字列しかない場合は、`eval-string`を使用できます。デフォルト環境にはこのプロシージャの旧バージョンがありますが、実際には`(ice-9 eval- string)`のバージョンが必要なので、それをロードしてください。

(use-modules (ice-9 eval-string))

Scheme 手順: **eval-string** string \[#:module=#f\] \[#:file=#f\] \[#:line=#f\] \[#:column=#f\] \[#:lang=(current-language)\] \[#:compile?=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-eval_002dstring)

文字列を現在の言語（通常はScheme）に従って解析します。文字列に含まれる式を順番に評価またはコンパイルし、最後の式を返します。

module キーワード引数が設定されている場合は、モジュールの探索を保存し ([モジュール システム リフレクション](https://doc.guix.gnu.org/guile/latest/en/guile.html#Module-System-Reflection) を参照)、評価前に現在のモジュールを module に設定します。

ファイル、行、列のキーワード引数を使用すると、ソース文字列が特定のソース位置から始まることを指定できます。

最後に、lang は言語であり、デフォルトでは現在の言語が使用されます。また、compile? が true の場合、または指定された言語の評価器がない場合、式はコンパイルされます。

C 関数: **scm\_eval\_string** (string) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005feval_005fstring)

C 関数: **scm\_eval\_string\_in\_module** (string, module) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005feval_005fstring_005fin_005fmodule)

これらのCバインディングは、`(ice-9 eval-string)`から`eval-string`を呼び出し、モジュール内または現在のモジュール内で評価を行います。

C 関数: `SCM` **scm\_c\_eval\_string** `(const char *string)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005feval_005fstring)

`scm_eval_string` ですが、`SCM` の代わりにロケールエンコーディングの C 文字列を受け取ります。

Scheme プロシージャ: **apply** proc arg … arglst [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-apply)

C 関数: **scm\_apply\_0** (proc, arglst) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fapply_005f0)

C 関数: **scm\_apply\_1** (proc, arg1, arglst) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fapply_005f1)

C 関数: **scm\_apply\_2** (proc, arg1, arg2, arglst) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fapply_005f2)

C 関数: **scm\_apply\_3** (proc, arg1, arg2, arg3, arglst) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fapply_005f3)

C 関数: **scm\_apply** (proc, arg, rest) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fapply)

引数 arg … と arglst リストの要素を使用して proc を呼び出します。

`scm_apply` は、Scheme レベルの `(lambda (proc arg1 . rest) ...)` に対応するパラメータを受け取ります。したがって、arg1 と rest リストの最後の要素を除くすべての要素が arg … を構成し、rest の最後の要素が arglst リストになります。または、rest が空のリスト `SCM_EOL` の場合は arg … はなく、(arg1) が arglst になります。

arglst は変更されませんが、`scm_apply` に渡される残りのリストは変更されます。

C 関数: **scm\_call\_0** (proc) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fcall_005f0)

C 関数: **scm\_call\_1** (proc, arg1) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fcall_005f1)

C 関数: **scm\_call\_2** (proc, arg1, arg2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fcall_005f2)

C 関数: **scm\_call\_3** (proc, arg1, arg2, arg3) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fcall_005f3)

C 関数: **scm\_call\_4** (proc, arg1, arg2, arg3, arg4) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fcall_005f4)

C 関数: **scm\_call\_5** (proc, arg1, arg2, arg3, arg4, arg5) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fcall_005f5)

C 関数: **scm\_call\_6** (proc, arg1, arg2, arg3, arg4, arg5, arg6) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fcall_005f6)

C 関数: **scm\_call\_7** (proc, arg1, arg2, arg3, arg4, arg5, arg6, arg7) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fcall_005f7)

C 関数: **scm\_call\_8** (proc, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fcall_005f8)

C 関数: **scm\_call\_9** (proc, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fcall_005f9)

指定された引数でprocを呼び出します。

C 関数: **scm\_call** (proc, ...) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fcall)

任意の数の引数を指定してprocを呼び出します。引数リストは`SCM_UNDEFINED`で終了する必要があります。例：

scm_call (scm_c_public_ref ("guile", "+"),
scm_from_int (1)
scm_from_int (2)
SCM_UNDEFINIDE);

C 関数: **scm\_call\_n** (proc、argv、nargs) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fcall_005fn)

引数配列argvを`SCM*`型として渡してprocを呼び出します。引数の長さは`size_t`型としてnargsに渡されます。

スキームプロシージャ: **primitive-eval** exp [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-primitive_002deval)

C 関数: **scm\_primitive\_eval** (exp) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fprimitive_005feval)

現在のモジュールで指定された最上位環境でexpを評価します。

* * *

次へ: [ファイルからのスキームコードの読み込み](https://doc.guix.gnu.org/guile/latest/en/guile.html#Loading)、前: [オンザフライ評価の手順](https://doc.guix.gnu.org/guile/latest/en/guile.html#Fly-Evaluation)、上: [スキームコードの読み込みと評価](https://doc.guix.gnu.org/guile/latest/en/guile.html#Read_002fLoad_002fEval_002fCompile) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.16.6 Scheme コードのコンパイル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compiling-Scheme-Code)

`eval` プロシージャは、Scheme の S 式表現を直接解釈します。評価のための別の戦略として、式を評価するために必要な計算を事前に決定し、その手順を使用して目的の結果を生成する方法があります。これは `コンパイル` と呼ばれます。

`(+ 2 2)` や `"Hello world!"` のような単純な Scheme 式をコンパイルすることは可能ですが、コンパイルが最も興味深いのは手続きのコンテキストにおいてです。ラムダ式をコンパイルすると、コンパイル済みの手続きが生成されます。これは通常の手続きとよく似ていますが、汎用インタプリタをバイパスできるため、通常ははるかに高速です。

Guileのインストール環境におけるシステムモジュールの関数は通常既にコンパイルされているため、高速にロードされ、実行されます。

適切に記述された Scheme プログラムでは、通常、このセクションのプロシージャは呼び出されません。これは、`eval` を使用することがしばしば好ましくないのと同じ理由です。デフォルトでは、Guile はまだコンパイルされていないファイルを見つけると自動的にコンパイルします ([`--auto-compile`](https://doc.guix.gnu.org/guile/latest/en/guile.html#Invoking-Guile) を参照)。コンパイラは、シェルから `guild compile foo.scm` のように明示的に呼び出すこともできます。

（`eval`や`compile`の呼び出しが一般的に好ましくないとされるのはなぜでしょうか？それは、これらの呼び出しがトップレベルの式にしか適用できないという制約があるからです。また、「コンパイル時」の計算に関するほとんどのニーズは、マクロやクロージャによって満たされます。もちろん、REPL自体や、ポートから式を読み込むコードは、良い反例と言えるでしょう。）

自動コンパイルは通常、ユーザーの介入を必要とせずに透過的に動作します。ただし、Guile はまだ適切な依存関係追跡を行っていないため、ファイル a.scm が b.scm のマクロを使用している場合、b.scm が変更されても、`a.scm` は自動的に再コンパイルされません。自動コンパイル キャッシュを強制的に無効にするには、Guile に `--fresh-auto-compile` オプションを渡すか、環境変数 `GUILE_AUTO_COMPILE` を `fresh` に設定してください (`0` または `1` ではなく)。

コンパイラ自体の詳細については、[仮想マシンへのコンパイル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compiling-to-the-Virtual-Machine) を参照してください。仮想マシンの詳細については、[Guile 用仮想マシン](https://doc.guix.gnu.org/guile/latest/en/guile.html#A-Virtual-Machine-for-Guile) を参照してください。

Guileのコンパイラへのコマンドラインインターフェースは、`guild compile`コマンドです。

コマンド: **guild compile** \[オプション...\] ファイル... [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-guild-compile)

ソースファイルであるコンパイルファイルをコンパイルし、バイトコードをコンパイルキャッシュまたは-oオプションで指定されたファイルに保存します。以下のオプションが利用可能です。

-L dir

--load-path=dir

モジュールのロードパスの先頭に dir を追加します。

-o ファイル

--output=ofile

出力バイトコードをファイル（ofile）に書き込みます。慣例として、バイトコードファイル名は「.go」で終わります。-o オプションを省略した場合、出力ファイル名は「compile-file」の場合と同様になります（下記参照）。

-x 拡張機能

拡張子を有効なソースファイル名拡張子として認識する。

例えば、R6RS コードをコンパイルする場合、拡張子が .sls のファイルを見つけるために、`-x .sls` を渡すと良いでしょう。

-W 警告 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-warnings_002c-compiler)

--warn=警告

特定の警告メッセージを有効にするには、`-Whelp` を使用してください。利用可能なオプションの一覧が表示されます。デフォルトは `-W1` で、一般的な警告メッセージをいくつか有効にします。すべての警告を無効にするには、`-W0` を指定してください。

-O opt [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-optimizations_002c-compiler)

--optimize=opt

特定のコンパイラ最適化を有効または無効にします。使用可能なオプションの一覧については、`-Ohelp` を使用してください。デフォルトは `-O2` で、ほとんどの最適化が有効になります。コンパイル速度がコンパイル済みコードの速度よりも重要な場合は、`-O0` をお勧めします。特定のコンパイラパスを無効にするには、`-Ono-opt` を渡します。コンパイラには任意の数の `-O` オプションを渡すことができ、後から渡されたものが優先されます。

--r6rs

--r7rs

デフォルトのバインディング、リーダーオプション、およびロードパスが特定の Scheme 標準に合わせて調整された環境でコンパイルしてください。[R6RS サポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#R6RS-Support) および [R7RS サポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#R7RS-Support) を参照してください。

-f lang

--from=lang

ファイルのソース言語としてlangを使用します。このオプションを省略した場合、`scheme`が使用されます。

-t lang

--to=lang

ファイルの出力言語としてlangを指定します。このオプションを省略した場合、`rtl`が使用されます。

-Tターゲット

--target=target

%host-type の代わりに target 用のコードを生成します ([%host-type](https://doc.guix.gnu.org/guile/latest/en/guile.html#Build-Config) を参照)。ターゲットは、`armv5tel-unknown-linux-gnueabi` のような有効な GNU トリプレットである必要があります (GNU Autoconf マニュアルの [Specifying Target Triplets](https://www.gnu.org/software/autoconf/manual/autoconf.html#Specifying-Target-Triplets) を参照)。

コーディング宣言が含まれていない限り、各ファイルは UTF-8 エンコードされていると想定されます([ソースファイルの文字エンコーディング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Character-Encoding-of-Source-Files) を参照)。

コンパイラはSchemeコードから直接呼び出すこともできます。これらのインターフェースは独自のモジュールに格納されています。

(use-modules (system base compile))

Scheme 手順: **compile** exp \[#:env=#f\] \[#:from=(current-language)\] \[#:to=value\] \[#:opts='()\] \[#:optimization-level=(default-optimization-level)\] \[#:warning-level=(default-warning-level)\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-compile-1)

環境 env 内で式 exp をコンパイルします。exp がプロシージャの場合、コンパイルされたプロシージャが出力されます。それ以外の場合は、`compile` は `eval` とほぼ同等です。

言語とコンパイラオプションの説明については、[仮想マシンへのコンパイル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compiling-to-the-Virtual-Machine)を参照してください。

Scheme 手順: **compile-file** file \[#:output-file=#f\] \[#:from=(current-language)\] \[#:to='rtl\] \[#:env=(default-environment from)\] \[#:opts='()\] \[#:optimization-level=(default-optimization-level)\] \[#:warning-level=(default-warning-level)\] \[#:canonicalization='relative\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-compile_002dfile-1)

file という名前のファイルをコンパイルします。

出力は出力ファイルに書き込まれます。出力ファイル名を指定しない場合、出力は`(compiled-file-name file)`で計算されたキャッシュディレクトリ内のファイルに書き込まれます。

from と to は、ソース言語とターゲット言語を指定します。これらのオプション、および env と opts の詳細については、[仮想マシンへのコンパイル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compiling-to-the-Virtual-Machine) を参照してください。

`guild compile`と同様に、ファイルにエンコーディング宣言が含まれていない限り、ファイルはUTF-8エンコードされているとみなされます。

スキームパラメータ: **default-optimization-level** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-default_002doptimization_002dlevel)

デフォルトの最適化レベル。0から9までの整数値で指定します。デフォルト値は2です。

スキームパラメータ: **default-warning-level** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-default_002dwarning_002dlevel)

デフォルトの警告レベル。0から9までの整数値で指定します。デフォルト値は1です。

パラメータの設定方法の詳細については、[パラメータ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Parameters)を参照してください。

Scheme手順: **compiled-file-name** ファイル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-compiled_002dfile_002dname)

コンパイル済みの Scheme ファイル file のキャッシュされた場所を計算します。

このファイルは通常、環境変数 `XDG_CACHE_HOME` の値に応じて、$HOME/.cache/guile/ccache ディレクトリの下に作成されます。`compiled-file-name` は、自動コンパイルされたファイルのキャッシュのフォールバック場所を提供することを目的としています。コンパイル ファイルを `%load-compiled-path` に配置したい場合は、`compile-file` に output-file オプションを明示的に渡す必要があります。

Scheme変数: **%auto-compilation-options** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0025auto_002dcompilation_002doptions)

この変数には、ソースファイルを自動コンパイルする際に `compile-file` プロシージャに渡されるオプションが格納されます。デフォルトでは、有用なコンパイル警告が有効になっています。この変数は~/.guile からカスタマイズできます。

* * *

次へ: [ロードパス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Load-Paths)、前: [Scheme コードのコンパイル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compilation)、上: [Scheme コードの読み込みと評価](https://doc.guix.gnu.org/guile/latest/en/guile.html#Read_002fLoad_002fEval_002fCompile) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.16.7 ファイルからのスキームコードの読み込み [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Loading-Scheme-Code-from-File)

Scheme手順: **load** filename \[reader\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-load-2)

ファイル名を読み込み、最上位環境でその内容を評価する。

リーダーが指定されている場合は、`#f` またはシグネチャ `(lambda (port) …)` を持つプロシージャで、port から次の式を読み込む必要があります。リーダーが `#f` であるか、指定されていない場合は、Guile の組み込みプロシージャ `read` が使用されます ([Reading Scheme Code](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scheme-Read) を参照)。

リーダー引数は、ファイルの読み込み前に `current-reader` 流体の値を設定することで効果を発揮し（下記参照）、読み込み完了時にその値を元に戻します。ファイル名内の Scheme コードは、`current-reader` 流体を設定することで、実行時に現在のリーダー手順を変更できます。

変数 `%load-hook` が定義されている場合、それはコードがロードされる前に呼び出されるプロシージャにバインドされている必要があります。`%load-hook` の詳細については、このセクションの後半にあるドキュメントを参照してください。

Scheme手順: **load-compiled**ファイル名 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-load_002dcompiled)

コンパイル済みのファイル（ファイル名：filename）を読み込みます。

ソースファイルをコンパイルし（[Scheme コードの読み取りと評価](https://doc.guix.gnu.org/guile/latest/en/guile.html#Read_002fLoad_002fEval_002fCompile)を参照）、その結果のファイルに対して `load-compiled` を呼び出すことは、ソースファイルに対して `load` を呼び出すことと同じです。

Scheme手順: **primitive-load** filename [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-primitive_002dload-1)

C 関数: **scm\_primitive\_load** (ファイル名) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fprimitive_005fload)

ファイル名 filename を読み込み、その内容を最上位環境で評価します。filename は、完全なパス名、または現在のディレクトリからの相対パス名のいずれかである必要があります。変数 `%load-hook` が定義されている場合は、コードが読み込まれる前に呼び出されるプロシージャにバインドする必要があります。`%load-hook` の詳細については、このセクションの後半を参照してください。

C 関数: `SCM` **scm\_c\_primitive\_load** `(const char *filename)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fprimitive_005fload)

`scm_primitive_load` ですが、`SCM` の代わりに C 文字列を受け取ります。

変数: **current-reader** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-current_002dreader)

`current-reader` は、上記の読み込み手順が式を読み込むために現在使用している読み込み手順を保持します（読み込み対象のファイルから）。`current-reader` は流体であるため、各動的ルートに独立した値があり、`fluid-ref` および `fluid-set!` を使用して読み書きする必要があります（[流体と動的状態](https://doc.guix.gnu.org/guile/latest/en/guile.html#Fluids-and-Dynamic-States) を参照）。

`current-reader` を変更することは、通常、ローカルな構文変更を導入する際に役立ちます。例えば、`fluid-set!` 呼び出しに続くコードが、新しくインストールされたリーダーを使用して読み込まれるようにする場合などです。`current-reader` の変更は、コードが評価される評価時、またはコードがコンパイルされるコンパイル時に行う必要があります。

(eval-when (compile eval)
(fluid-set! current-reader my-own-reader))

上記の`eval-when`形式は、`current-reader`の変更が適切なタイミングで行われることを保証します。

変数: **%load-hook** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0025load_002dhook)

ファイルがロードされるたびに呼び出されるプロシージャ `(%load-hook filename)`、またはそのような呼び出しが行われない場合は `#f` を指定します。`%load-hook` は、すべてのロード関数 (`load` および `primitive-load`、次のセクションで説明する `load-from-path` および `primitive-load-path`) で使用されます。

例えば、アプリケーションはこれを設定して、読み込まれたものを表示できます。

(set! %load-hook (lambda (filename)
(フォーマット #t "~a ...\n" ファイル名を読み込んでいます)))
(パス「foo.scm」から読み込み)
⊣ /usr/local/share/guile/site/foo.scm を読み込んでいます...

スキーム手順: **current-load-port** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-current_002dload_002dport)

C 関数: **scm\_current\_load\_port** () [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fcurrent_005fload_005fport)

現在のロードポートを返します。ロードポートは`primitive-load`によって内部的に使用されます。

* * *

次へ: [ソースファイルの文字エンコーディング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Character-Encoding-of-Source-Files)、前: [ファイルからのスキームコードの読み込み](https://doc.guix.gnu.org/guile/latest/en/guile.html#Loading)、上: [スキームコードの読み込みと評価](https://doc.guix.gnu.org/guile/latest/en/guile.html#Read_002fLoad_002fEval_002fCompile) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.16.8 ロードパス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Load-Paths-1)

前のセクションで説明した手順では、ファイルシステム内の特定の場所にあるSchemeコードを検索します。Guileにも、ロードパス内でコードを検索する手順がいくつかあります。

変数: **%load-path** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0025load_002dpath)

Scheme モジュールとライブラリを検索するディレクトリのリスト。Guile の起動時に、`%load-path` はデフォルトのロード パス `(list (%library-dir) (%site-dir) (%global-site-dir) (%package-data-dir))` に初期化されます。環境変数 `GUILE_LOAD_PATH` を使用して、追加のディレクトリを先頭または末尾に追加できます ([環境変数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Environment-Variables) を参照)。

`%site-dir` および関連する手順の詳細については、[構成、ビルド、およびインストール](https://doc.guix.gnu.org/guile/latest/en/guile.html#Build-Config) を参照してください。

Scheme手順: **load-from-path** filename [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-load_002dfrom_002dpath)

`load` と同様ですが、読み込みパス内でファイル名を検索します。コンパイル済みのファイルが利用可能で最新の状態であれば、優先的にそれを読み込みます。

ユーザーは`add-to-load-path`を呼び出すことで、ロードパスを拡張できます。

Scheme構文: **add-to-load-path** dir [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-add_002dto_002dload_002dpath)

ロードパスにディレクトリを追加します。

例えば、スクリプトには、自身が存在するディレクトリをロードパスに追加するための以下の形式を含めることができます。

(add-to-load-path (dirname (current-filename)))

`%load-path` を直接変更するよりも、`add-to-load-path` を使用する方が良いでしょう。なぜなら、`add-to-load-path` はコンパイル時と実行時の両方でパスの変更を処理してくれるからです。

Scheme手順: **primitive-load-path** ファイル名 \[exception-on-not-found\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-primitive_002dload_002dpath)

C 関数: **scm\_primitive\_load\_path** (ファイル名) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fprimitive_005fload_005fpath)

`%load-path` で指定されたパス名 filename を検索し、最上位環境に読み込みます。filename が相対パス名で、検索パスのリストに見つからない場合は、エラーが通知されます。コンパイル済みのファイルが利用可能で最新の状態であれば、優先的に読み込みます。

ファイル名が相対パス名であり、検索パスのリストに見つからない場合、オプションの2番目の引数exception-on-not-foundに応じて、次の3つのいずれかが発生します。引数が`#f`の場合は、`#f`が返されます。引数がプロシージャの場合は、引数なしで呼び出されます。（これにより、ファイルの読み込みによって発生する例外と、ローダー自体に関連する例外を区別できます。）それ以外の場合は、エラーが通知されます。

Guile 1.8以前のバージョンとの互換性を保つため、C関数は引数を1つだけ取ります。引数は文字列（ファイル名）または引数リストのいずれかです。

Scheme Procedure: **%search-load-path** filename [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0025search_002dload_002dpath)

C 関数: **scm\_sys\_search\_load\_path** (ファイル名) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsys_005fsearch_005fload_005fpath)

`%load-path` で指定されたパスリストにある filename という名前のファイルを検索します。filename は現在のユーザーが読み取り可能なファイルである必要があります。filename が検索対象パスのリストに含まれているか、絶対パス名である場合は、その完全なパス名を返します。それ以外の場合は、`#f` を返します。ファイル名には、`%load-extensions` リストにある任意の拡張子を含めることができます。`%search-load-path` は、各拡張子を自動的に試行します。

変数: **%load-extensions** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0025load_002dextensions)

Schemeコードを含むファイルのデフォルトのファイル拡張子のリスト。`%search-load-path`は、読み込むファイルを探す際に、これらの拡張子をそれぞれ試します。デフォルトでは、`%load-extensions`はリスト`("" ".scm")`にバインドされています。

前述のとおり、Guile はソース ファイルを探す際に `%load-path` を参照すると同時に、対応するコンパイル済みファイルを探すために `%load-compiled-path` も参照します。コンパイル済みファイルがソース ファイルと同じかそれより新しい場合は、ソース ファイルの代わりに `load-compiled` を使用してコンパイル済みファイルが読み込まれます。

変数: **%load-compiled-path** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0025load_002dcompiled_002dpath)

`%load-path` と同様ですが、コンパイル済みファイル用です。デフォルトでは、このパスには 2 つのエントリがあります。1 つは Guile 自体からコンパイルされたファイル用、もう 1 つはサイト パッケージ用です。環境変数 `GUILE_LOAD_COMPILED_PATH` を使用して、追加のディレクトリを先頭または末尾に追加できます ([環境変数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Environment-Variables) を参照)。

`primitive-load-path` が相対パスに対応するコンパイル済みファイルを `%load-compiled-path` 内で検索する場合、相対パスに `.go` を付加します。たとえば、`ice-9/popen` を検索すると、`/usr/lib/guile/3.0/ccache/ice-9/popen.go` が見つかり、`/usr/share/guile/3.0/ice-9/popen.scm` の代わりにそれが使用されます。

`primitive-load-path` が `%load-compiled-path` 内に対応する `.go` ファイルを見つけられない場合、または `.go` ファイルが古い場合は、フォールバック パス内で対応する自動コンパイル済みファイルを検索し、存在しない場合は作成します。

サイト パッケージを正しくインストールする方法については、[サイト パッケージのインストール](https://doc.guix.gnu.org/guile/latest/en/guile.html#Installing-Site-Packages) を参照してください。ロード パスとモジュールの関係については、[モジュールとファイルシステム](https://doc.guix.gnu.org/guile/latest/en/guile.html#Modules-and-the-File-System) を参照してください。フォールバック パスと自動コンパイルについては、[Scheme コードのコンパイル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compilation) を参照してください。

最後に、一般的なパス操作のためのヘルパープロシージャがいくつか用意されています。

Scheme手順: **parse-path** path \[tail\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-parse_002dpath)

C 関数: **scm\_parse\_path** (path, tail) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fparse_005fpath)

コロン区切りの文字列であるパスを解析してリストを作成し、末尾を追加したリストを返します。パスが`#f`の場合は、末尾が返されます。

Scheme手順: **parse-path-with-ellipsis** パスベース [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-parse_002dpath_002dwith_002dellipsis)

C 関数: **scm\_parse\_path\_with\_ellipsis** (path, base) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fparse_005fpath_005fwith_005fellipsis)

コロンで区切られた文字列であるパスを解析してリストに変換し、結果として得られるリストを返します。パスの「...」部分が存在する場合は、その部分にベース（リスト）を挿入します。存在しない場合は、ベースを末尾に追加します。パスが「#f」の場合は、ベースが返されます。

Scheme 手順: **search-path** path filename \[extensions \[require-exts?\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-search_002dpath)

C 関数: **scm\_search\_path** (path, filename, rest) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsearch_005fpath)

パス内から filename という名前のファイルを含むディレクトリを検索します。ファイルは読み取り可能でなければならず、ディレクトリであってはなりません。見つかった場合は、その完全なファイル名を返します。見つからなかった場合は、`#f` を返します。filename が絶対パスの場合は、変更せずに返します。extensions が指定されている場合、それは文字列のリストです。パス内の各ディレクトリに対して、filename に各拡張子を連結したものを検索します。require-exts? が true の場合、返されるファイル名に指定された拡張子のいずれかが含まれている必要があります。require-exts? が指定されていない場合は、デフォルトで `#f` になります。

Guile 1.8以前のバージョンとの互換性を保つため、C関数は3つの引数のみを受け取ります。

* * *

次へ: [遅延評価](https://doc.guix.gnu.org/guile/latest/en/guile.html#Delayed-Evaluation)、前: [ロードパス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Load-Paths)、上: [スキームコードの読み込みと評価](https://doc.guix.gnu.org/guile/latest/en/guile.html#Read_002fLoad_002fEval_002fCompile) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.16.9 ソースファイルの文字エンコーディング [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Character-Encoding-of-Source- Files-1)

Schemeのソースコードファイルは通常ASCIIまたはUTF-8でエンコードされますが、組み込みのリーダーは他の文字エンコーディングも解釈できます。GuileがSchemeのソースコードをロードする際、ファイルのエンコーディングを推測するために、後述する`file-encoding`プロシージャを使用します。ヒントがない場合は、UTF-8が想定されます。ソースファイルのエンコーディングに関するヒントを提供する1つの方法は、ファイルの先頭500文字以内にエンコーディング宣言を配置することです。

コーディング宣言は `coding: XXXXXX` の形式で記述します。ここで `XXXXXX` は、ソースコードファイルがエンコードされている文字エンコーディングの名前です。コーディング宣言は、スキームコメント内に記述する必要があります。セミコロンで始まるコメント、またはファイル内の最初のブロックコメント `#!` のいずれかになります。

コーディング宣言における文字エンコーディング名は、通常、小文字で、文字、数字、ハイフンのみを含み、`set-port-encoding!` で認識されます（[`set-port-encoding!`](https://doc.guix.gnu.org/guile/latest/en/guile.html#Ports) を参照）。文字エンコーディング名の一般的な例としては、IANA で定義されている `utf-8` や `iso-8859-1` などがあります。したがって、コーディング宣言は Emacs とほぼ互換性があります。

ただし、Emacsが認識するエンコーディング名とIANAが定義するエンコーディング名にはいくつかの違いがあり、後者は基本的に前者のサブセットです。たとえば、`latin-1`はEmacsでは有効なエンコーディング名ですが、Guileが準拠するIANA標準には準拠していません。代わりに、Emacsが認識し、IANAが命名した`iso-8859-1`を使用する必要があります（IANAは大文字で表記しますが、Emacsは小文字を要求し、Guileは大文字小文字を区別しません）。

ソースコードの場合、組み込みのソースコードリーダーで解釈できる文字エンコーディングは、すべての文字エンコーディングのうち一部のみとなります。ASCIIテキストが変更されずにそのまま表示される文字エンコーディングのみが使用可能です。これには、`UTF-8`および`ISO-8859-1`から`ISO-8859-15`が含まれます。マルチバイト文字エンコーディングである`UTF-16`および`UTF-32`はASCIIと互換性がないため、使用できません。

ポートから非ASCIIコードを読み込む場合、`load`関数ではなく`read`関数を使用するシナリオが考えられます。ポートの文字エンコーディングが、ポートが読み込むコードのエンコーディングと同じであれば、特別な処理は必要ありません。ポートが自動的に文字エンコーディング変換を行います。ポートのエンコーディングを設定するには、`setlocale`関数または`set-port-encoding!`関数を使用します（[Ports](https://doc.guix.gnu.org/guile/latest/en/guile.html#Ports)を参照）。

未知の文字エンコーディングのコードを読み取るためにポートを使用する場合、次の 3 つの手順で実行できます。まず、`set-port-encoding!` を使用してポートの文字エンコーディングを ISO-8859-1 に設定します。次に、後述する `file-encoding` プロシージャを使用して、ポートから読み取る際にエンコーディング宣言をスキャンします。副作用として、スキャンが完了するとポートが巻き戻されます。その後、`set-port-encoding!` を使用して、`file-encoding` が返すエンコーディングがあれば、ポートの文字エンコーディングをそのエンコーディングに設定します。これで、コードを通常どおり読み取ることができます。

あるいは、`open-file` および関連する手順の `#:guess-encoding` キーワード引数を使用することもできます。[ファイルポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#File-Ports) を参照してください。

Scheme Procedure: **file-encoding** port [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-file_002dencoding)

C 関数: **scm\_file\_encoding** (ポート) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffile_005fencoding)

ポートの最初の数百バイトをスキャンして、文字エンコーディングに関する手がかりを探します。エンコーディング名を含む文字列を返すか、エンコーディングが判別できない場合は「#f」を返します。ポートは巻き戻されます。

現在サポートされている方法は、Emacs のような文字コード宣言を探すことのみです (GNU Emacs リファレンス マニュアルの [Emacs がファイル エンコーディングを認識する方法](https://www.gnu.org/software/emacs/manual/html_mono/emacs.html#Recognize-Coding) を参照)。コード宣言は `coding: XXXXX` の形式で、Scheme コメント内に記述する必要があります。今後、追加のヒューリスティックが追加される可能性があります。

* * *

次へ: [ローカル評価](https://doc.guix.gnu.org/guile/latest/en/guile.html#Local-Evaluation)、前: [ソースファイルの文字エンコーディング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Character-Encoding-of-Source-Files)、上: [スキームコードの読み込みと評価](https://doc.guix.gnu.org/guile/latest/en/guile.html#Read_002fLoad_002fEval_002fCompile) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.16.10 遅延評価 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Delayed-Evaluation-1)

Promise は、計算結果が実際に必要になるまで計算を延期し、その計算を一度だけ実行するための便利な方法です。詳細については、[SRFI-45 - 反復遅延アルゴリズムを表現するためのプリミティブ](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d45) も参照してください。

構文: **delay** expr [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-delay)

指定された式（expr）を保持し、後で`force`によって評価される準備が整ったPromiseオブジェクトを返します。

Scheme 手順: **promise?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-promise_003f)

C 関数: **scm\_promise\_p** (obj) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fpromise_005fp)

objがPromiseであればtrueを返します。

Scheme Procedure: **force** p [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-force)

C 関数: **scm\_force** (p) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fforce)

指定されたプロミス p 内の式を評価して得られた値を返します。p が以前に強制実行されている場合、その式は再度評価されず、その時点で得られた値がそのまま返されます。

`force` 実行中、式は自身のプロミスに対して再度 `force` を呼び出すことができ、その結果、その式が再帰的に評価されます。最初に返される評価結果がプロミスの値となります。それ以降の評価は通常どおり完了しますが、その結果は無視され、`force` は常に最初の値を返します。

* * *

次へ: [ローカルインクルージョン](https://doc.guix.gnu.org/guile/latest/en/guile.html#Local-Inclusion)、前: [遅延評価](https://doc.guix.gnu.org/guile/latest/en/guile.html#Delayed-Evaluation)、上: [スキームコードの読み込みと評価](https://doc.guix.gnu.org/guile/latest/en/guile.html#Read_002fLoad_002fEval_002fCompile) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.16.11 ローカル評価 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Local-Evaluation-1)

Guileには、字句環境をキャプチャし、その環境内で新しい式を評価する機能が備わっています。このコードはモジュールとして実装されています。

(use-modules (ice-9 local-eval))

構文: **the-environment** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-the_002denvironment)

`local-eval` または `local-compile` で使用するための字句環境をキャプチャして返します。

Scheme手順: **local-eval** exp env [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-local_002deval)

C 関数: **scm\_local\_eval** (exp, env) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005flocal_005feval)

Scheme手順: **local-compile** exp env \[opts=()\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-local_002dcompile)

式 exp を字句環境 env で評価またはコンパイルします。

以下に簡単な例を示します。これは、ある時点での値だけでなく、変数そのものが捕捉されることを示しています。

(define e (let ((x 100)) (the-environment)))
(define fetch-x (local-eval '(lambda () x) e))
(フェッチ-x)
⇒ 100
(local-eval '(set! x 42) e)
(フェッチ-x)
⇒ 42

exp は `(the-environment)` の字句環境内で評価されますが、`local-eval` の呼び出しの動的な環境を持ちます。

`local-eval`と`local-compile`は式のみを評価し、定義は評価できません。

(local-eval '(define foo 42)
(let ((x 100)) (the-environment)))
⇒構文エラー: 式コンテキストでの定義

`(the-environment)` の現在の実装では、「通常の」字句バインディングと `syntax-case` でバインドされたパターン変数のみがキャプチャされることに注意してください。`let-syntax`、`letrec-syntax`、またはトップレベル以外の `define-syntax` 形式でバインドされたローカル構文トランスフォーマーは、現時点ではキャプチャされません。キャプチャされた構文キーワードを `local-eval` または `local-compile` で参照しようとすると、エラーが発生します。

* * *

次へ: [サンドボックス評価](https://doc.guix.gnu.org/guile/latest/en/guile.html#Sandboxed-Evaluation)、前: [ローカル評価](https://doc.guix.gnu.org/guile/latest/en/guile.html#Local-Evaluation)、上: [スキームコードの読み込みと評価](https://doc.guix.gnu.org/guile/latest/en/guile.html#Read_002fLoad_002fEval_002fCompile) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.16.12 ローカルインクルージョン [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Local-Inclusion-1)

このセクションでは、Scheme コードをリンクするさまざまな方法について説明しました。基本的には、`load` と `load-compiled` を使用して実行時にファイルを読み込む方法です。Guile では、実行時ではなく展開時にプログラムの一部を組み合わせる別の方法も提供されています。

Scheme構文: **include** ファイル名 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-include)

展開時にファイル名を開き、その中に含まれる Scheme フォームを読み込み、`begin` 内の `include` の場所にそれらを挿入します。

ファイル名が相対パスの場合、`include` フォームが出現するファイルを含むパスからの相対パスで検索されます。

C言語プログラマーの方であれば、Schemeの`load`がC言語の`dlopen`に相当するとすれば、`include`はC言語のプリプロセッサの`#include`に相当すると考えてください。`include`を使用すると、`include`形式ではなく、インクルードするファイルの内容を直接入力したのと同じ効果が得られます。

コードはコンパイル時にインクルードされるため、マクロエクスパンダから利用可能です。インクルードされたファイル内の構文定義は、`eval-when` を使用する必要なく、`include` が出現する形式で後続のコードから利用できます。（[Eval-when](https://doc.guix.gnu.org/guile/latest/en/guile.html#Eval-When) を参照してください。）

同様の理由で、`include` を使用するフォームをコンパイルすると、複数のファイルで構成される 1 つのコンパイル単位が生成されます。コンパイル済みファイルの読み込みは、コンパイル単位に対して 1 つの `stat` 操作で済みます。これは、`load` の場合の `2*n` 操作 (最良の場合、読み込まれたソース ファイルごとに 1 回、対応するコンパイル済みファイルごとに 1 回) とは異なります。

`load` とは異なり、`include` はネストされたレキシカル コンテキスト内でも動作します。偶然にも、オプティマイザはレキシカル コンテキスト内で最も効果的に動作します。これは、レキシカル コンテキスト内のバインディングの使用箇所がすべて可視化されるため、`(let () ...)` 内にファイルを含めることで、場合によっては大幅な速度向上につながるからです。

一方で、`include`には早期バインディングの欠点もすべてあります。つまり、`include`を含むコードがコンパイルされると、インクルードされたファイルへの変更は、インクルードフォームの今後の動作には反映されません。

また、コンパイル時に絶対パスまたは現在のディレクトリからの相対パスを必要とする`include`の特定の形式は、ソースをある場所でコンパイルしてから別の場所にインストールする場合にはあまり適していません。そのため、Guileはロードパス内からインクルードするソースファイルを探す`include-from-path`という別の形式を提供しています。

Scheme構文: **include-from-path** file-name [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-include_002dfrom_002dpath)

`include` と同様ですが、`file-name` が絶対ファイル名であることを期待するのではなく、`%load-path` 内で検索するための相対パスであることが期待されます。

`include-from-path` は、パッケージのすべてのソースファイルをインストールしたい場合（そうすべきです！）に特に役立ちます。これにより、`.go` ファイルが最新であるかどうかに依存するのではなく、インストール済みのファイルをソースから評価することが可能になります。

* * *

次へ: [REPL サーバー](https://doc.guix.gnu.org/guile/latest/en/guile.html#REPL-Servers)、前: [ローカル インクルージョン](https://doc.guix.gnu.org/guile/latest/en/guile.html#Local-Inclusion )、上: [Scheme コードの読み込みと評価](https://doc.guix.gnu.org/guile/latest/en/guile.html#Read_002fLoad_002fEval_002fCompile) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.16.13 サンドボックス評価 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Sandboxed-Evaluation-1)

信頼できない相手から提供されたコードを評価したい場合もあるでしょう。最も安全な方法は、新しいコンピュータを購入し、そのコンピュータ上でコードを評価し、その後コンピュータを廃棄することです。しかし、この簡単な方法を取りたくない場合は、Guileには信頼できないコードをある程度の安心感を持って評価できる限定的な「サンドボックス」機能が用意されています。

サンドボックス化された評価ツールを使用するには、そのモジュールをロードします。

(use-modules (ice-9 sandbox))

ガイルのサンドボックス機能は、コードが使用する時間と空間を制限する機能から始まります。

Scheme Procedure: **call-with-time-limit** limit thunk limit-reached [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-call_002dwith_002dtime_002dlimit)

thunk を呼び出しますが、壁時計の制限秒が経過した場合はキャンセルします。計算がキャンセルされた場合は、末尾で limit-reached を呼び出します。thunk は割り込みを無効にしたり、`dynamic-wind` アンワインド ハンドラによるアボートを妨げたりしてはなりません。

Scheme Procedure: **call-with-allocation-limit** limit thunk limit-reached [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-call_002dwith_002dallocation_002dlimit)

thunk を呼び出しますが、制限バイトが割り当てられている場合はキャンセルします。計算がキャンセルされた場合は、末尾で limit-reached を呼び出します。thunk は割り込みを無効にしたり、`dynamic-wind` アンワインド ハンドラによるアボートを妨げたりしてはなりません。

この制限はスタックとヒープの両方の割り当てに適用されます。制限バイト数が割り当てられるまで計算は中断されませんが、ヒープ割り当ての制限については、次のガベージコレクションまでチェックが延期される場合があります。

なお、現在の欠点として、ヒープサイズの制限はすべてのスレッドに適用され、他の無関係なスレッドによる同時割り当ても割り当て制限にカウントされます。

Scheme Procedure: **call-with-time-and-allocation-limits** time-limit allocation-limit thunk [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-call_002dwith_002dtime_002dand_002dallocation_002dlimits)

thunk を動的に呼び出します。この動的範囲では、実行時間は時間制限秒、メモリ割り当ては割り当て制限バイトに制限されます。thunk は割り込みを無効にしたり、`dynamic-wind` アンワインド ハンドラによるアボートを妨げたりしてはなりません。

成功した場合、thunk の呼び出しによって生成されたすべての値を返します。thunk によってスローされた捕捉されない例外は伝播されます。時間または割り当て制限を超過した場合、`limit-exceeded` キーに例外がスローされます。

時間制限とスタック制限はどちらも非常に厳密ですが、ヒープ制限はガベージコレクション後に非同期的にチェックされます。特に、ヒープが既に非常に大きい場合、ガベージコレクション間の割り当てバイト数が大きくなるため、チェックの精度が低下します。

さらに、割り当て制限で使用されるメカニズム（`after-gc-hook`）のため、`(make-vector #e1e7)`のような大きな単一の割り当ては、割り当て自体がガベージコレクションを引き起こす場合でも、割り当てが完了した後にのみ検出されます。そのため、ユーザーコードが設定された割り当て制限を超えるだけでなく、利用可能なすべてのメモリを使い果たし、任意の割り当て箇所でメモリ不足状態を引き起こす可能性があります。Guile自体でのメモリ割り当ての失敗は安全であり、例外がスローされるはずですが、ほとんどのシステムは`malloc`の失敗を処理するように設計されていません。そのため、割り当ての失敗はシステム内で予期しないコードパスを実行する可能性があり、これはサンドボックスの弱点（したがって興味深い攻撃ポイント）となります。

メインのサンドボックスインターフェースは`eval-in-sandbox`です。

Scheme Procedure: **eval-in-sandbox** exp \[#:time-limit 0.1\] \[#:allocation-limit #e10e6\] \[#:bindings all-pure-bindings\] \[#:module (make-sandbox-module bindings)\] \[#:sever-module? #t\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-eval_002din_002dsandbox)

Scheme式expを隔離された「サンドボックス」内で評価します。実行時間を実時間でtimelimit秒に制限し、メモリ割り当てをallocationlimitバイトに制限します。

評価はモジュール内で行われます。デフォルトでは、バインディングに対して`make-sandbox-module`を実行した結果が使用され、バインディング自体もデフォルトでは`all-pure-bindings`になります。これがサンドボックスの中核であり、式に対して_安全な_スコープを作成することです。

安全なサンドボックスモジュールには2つの特徴があります。まず、評価中の式が時間制限やメモリ割り当て制限によってキャンセルされるのを許容しません。これにより、式が適切なタイミングで終了することが保証されます。

第二に、安全なサンドボックスモジュールは、評価が以前の評価から情報を受け取ることや、将来の評価に影響を与えることを防ぎます。`(ice-9 sandbox)` によってエクスポートされるバインディングセットのすべての組み合わせは、安全なサンドボックスモジュールを形成します。

バインディングはインポート セットのリストとして指定する必要があります。1 つのインポート セットは、car が `(ice-9 q)` のようなインターフェース名を表し、cdr がインポートのリストであるリストです。インポートは、ベア シンボルまたは `(out . in)` のペアのいずれかです。ここで、out と in はどちらもシンボルであり、それぞれバインディングがモジュールからエクスポートされる名前と、バインディングを使用可能にする名前を示します。bindings は、モジュール引数のデフォルト初期化子への入力としてのみ使用されることに注意してください。`#:module` を渡すと、bindings は使用されません。sever-module? が true (デフォルト) の場合、評価が戻った後、モジュールはグローバル モジュール ツリーからリンク解除され、mod がガベージ コレクションされるようになります。

成功した場合、exp によって生成されたすべての値を返します。式によってキャッチされなかった例外は伝播されます。時間または割り当て制限を超えた場合は、`limit-exceeded` キーに例外がスローされます。

安全なサンドボックスモジュールを構築するのは、一般的に難しい作業です。Guileは、定義済みのバインディングセットから安全なモジュールを簡単に構築する方法を提供しています。そのインターフェースについて説明する前に、安全性に関する一般的な注意事項をいくつかご紹介します。

1. 時間制限と割り当て制限は、計算の中断とキャンセルが可能であることを前提としています。そのため、サンドボックスモジュールに含まれるバインディングは、割り込み処理を無期限に延期したり、中止を防止したりすることができてはなりません。実際には、この2つ目の考慮事項から、`dynamic-wind` はどのバインディングセットにも含めるべきではないということになります。
2. 時間およびリソース割り当ての制限は、`eval-in-sandbox` 呼び出しにのみ適用されます。呼び出しによって返されたプロシージャが後で呼び出される場合、制限は「自動的に」適用されません。`eval-in-sandbox` のユーザーは、サンドボックスから脱出するプロシージャを呼び出す際に、制限を再適用するよう細心の注意を払う必要があります。
3. 同様に、サンドボックスから脱出したプロシージャが後で呼び出される場合、`eval-in-sandbox`呼び出しの動的な環境が必ずしも存在するとは限りません。
    
この詳細により、2 つの理由から `primitive-eval` をサンドボックスに公開することができなくなります。1 つ目の理由は、`allow-legacy-syntax-objects?` パラメータが true の場合、レガシー コードが任意のバインディングへの参照を偽造する可能性があるためです。このパラメータのデフォルト値は true です。詳細は [構文変換ヘルパー](https://doc.guix.gnu.org/guile/latest/en/guile.html#Syntax-Transformer-Helpers) を参照してください。このパラメータは `eval-in-sandbox` 呼び出し自体の間は `#f` にバインドされますが、エスケープされたプロシージャの呼び出し中はバインドされません。
    
`primitive-eval` を公開しない2つ目の理由は、`primitive-eval` は暗黙的に現在のモジュール内で動作するためです。エスケープされたプロシージャの場合、現在のモジュールは `eval-in-sandbox` 呼び出し自体の現在のモジュールとは異なる可能性があります。
    
ここで共通しているのは、サンドボックスに公開されるインターフェースが動的な環境に依存している場合、サンドボックス内のプロシージャがアクセスすべきでないバインディングという形で、誤って追加の機能を与えてしまう可能性があるということです。そのため、デフォルトの定義済みバインディングセットは、動的にスコープが設定される値に依存しません。
    
4. ミューテーションによって、サンドボックス化された評価が、提供されたデータの利用者の不変条件を破る可能性があります。多くのコードは一般的にミューテーションを想定していませんが、可変データをサンドボックス化された評価に渡し、かつその評価にミューテーション機能も付与した場合、サンドボックス化されたコードは実際にそのデータを変更する可能性があります。サンドボックスへのデフォルトのバインディングセットには、ミューテーションを行うプリミティブは含まれていません。
    
関連して、`set!` はサンドボックスがプリミティブを変更できるようにし、システム全体の多くの不変条件を無効にする可能性があります。Guile は現在、インポートされたバインディングと可変性に関して非常に寛容です。モジュールローカル変数またはレキシカルにバインドされた変数への `set!` は問題ありませんが、インポートされたバインディングへの `set!` を簡単に禁止する方法が現在ないため、現時点ではどのバインディング セットにも `set!` は含まれていません。
    
5. ミューテーションによって、サンドボックス化された評価が状態を保持したり、他のコードとの通信メカニズムを作成したりすることが可能になる場合があります。これは一見魅力的に聞こえますが、一方で、脅威モデルの一部となる可能性もあります。繰り返しになりますが、デフォルトのバインディングセットにはミューテーションプリミティブが含まれていないため、サンドボックス化された評価が状態を保持することはできません。
6. サンドボックスは、ネットワーク接続を開いたり、ファイルに書き込んだり、ディスクからファイルを開いたりすることができないはずです。デフォルトのバインディングセットには、オペレーティングシステムとのやり取りは含まれていません。

読者の皆様、もし上記の議論に興味を持たれたなら、ジョナサン・リース氏の博士論文「ラムダ計算に基づくセキュリティカーネル」もきっとお楽しみいただけるでしょう。

Scheme変数: **all-pure-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-all_002dpure_002dbindings)

Guileユーザーコードがデフォルトで利用できるバインディングの安全なサブセットを構成する、すべての「純粋な」バインディング。

Scheme変数: **all-pure-and-impure-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-all_002dpure_002dand_002dimpure_002dbindings)

`all-pure-bindings`と同様ですが、`vector-set!`のような変更可能なプリミティブも含まれています。このセットは、変更に関する注意点はあるものの、上記で述べた意味では依然として安全です。

これらの複合セットの構成要素は以下のとおりです。

Scheme変数: **alist-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-alist_002dbindings)

Scheme変数: **array-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-array_002dbindings)

Scheme変数: **bit-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bit_002dbindings)

Scheme変数: **bitvector-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bitvector_002dbindings)

Scheme変数: **char-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dbindings)

Scheme変数: **char-set-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dset_002dbindings)

Scheme変数: **clock-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-clock_002dbindings)

Scheme変数: **core-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-core_002dbindings)

Scheme変数: **error-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-error_002dbindings)

スキーム変数: **fluid-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-fluid_002dbindings)

スキーム変数: **hash-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hash_002dbindings)

Scheme変数: **iteration-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-iteration_002dbindings)

Scheme変数: **keyword-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-keyword_002dbindings)

Scheme変数: **list-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-list_002dbindings)

Scheme変数: **macro-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-macro_002dbindings)

Scheme変数: **nil-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-nil_002dbindings)

Scheme変数: **number-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-number_002dbindings)

Scheme変数: **pair-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-pair_002dbindings)

Scheme変数: **predicate-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-predicate_002dbindings)

Scheme変数: **procedure-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-procedure_002dbindings)

Scheme変数: **promise-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-promise_002dbindings)

Scheme変数: **prompt-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-prompt_002dbindings)

Scheme変数: **regexp-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-regexp_002dbindings)

Scheme変数: **sort-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sort_002dbindings)

Scheme変数: **srfi-4-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-srfi_002d4_002dbindings)

Scheme変数: **string-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dbindings)

Scheme変数: **symbol-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-symbol_002dbindings)

Scheme変数: **unspecified-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-unspecified_002dbindings)

Scheme変数: **variable-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-variable_002dbindings)

Scheme変数: **vector-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-vector_002dbindings)

Scheme変数: **version-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-version_002dbindings)

`all-pure-bindings`の構成要素。

Scheme変数: **mutating-alist-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-mutating_002dalist_002dbindings)

Scheme変数: **mutating-array-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-mutating_002darray_002dbindings)

Scheme変数: **mutating-bitvector-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-mutating_002dbitvector_002dbindings)

スキーム変数: **mutating-fluid-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-mutating_002dfluid_002dbindings)

Scheme変数: **mutating-hash-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-mutating_002dhash_002dbindings)

Scheme変数: **mutating-list-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-mutating_002dlist_002dbindings)

Scheme変数: **mutating-pair-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-mutating_002dpair_002dbindings)

Scheme変数: **mutating-sort-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-mutating_002dsort_002dbindings)

スキーム変数: **mutating-srfi-4-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-mutating_002dsrfi_002d4_002dbindings)

Scheme変数: **mutating-string-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-mutating_002dstring_002dbindings)

Scheme変数: **mutating-variable-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-mutating_002dvariable_002dbindings)

Scheme変数: **mutating-vector-bindings** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-mutating_002dvector_002dbindings)

`all-pure-and-impure-bindings` の追加コンポーネント。

最後に、バインディングセットをどう活用すれば良いのでしょうか？そもそもバインディングセットとは何でしょうか？そんな疑問に答えるのが`make-sandbox-module`です。

Scheme 手順: **make-sandbox-module** バインディング [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dsandbox_002dmodule)

バインディングのみを含む新しいモジュールを返します。

バインディングはインポートセットのリストとして指定する必要があります。1つのインポートセットは、carが`(ice-9 q)`のようなインターフェース名を表し、cdrがインポートのリストであるリストです。インポートは、ベアシンボルまたは`(out . in)`のペアのいずれかです。ここで、outとinはどちらもシンボルであり、それぞれバインディングがモジュールからエクスポートされる名前と、バインディングを利用可能にする名前を表します。

つまり、バインディングセットは単なるリストであり、`all-pure-and-impure-bindings`は実際にはすべてのコンポーネントバインディングセットを連結した結果に過ぎないということです。

* * *

次へ: [協調型 REPL サーバー](https://doc.guix.gnu.org/guile/latest/en/guile.html#Cooperative-REPL-Servers)、前: [サンドボックス化された評価](https://doc.guix.gnu.org/guile/latest/en/guile.html#Sandboxed-Evaluation)、上: [Scheme コードの読み取りと評価](https://doc.guix.gnu.org/guile/latest/en/guile.html#Read_002fLoad_002fEval_002fCompile) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.16.14 REPLサーバー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#REPL-Servers-1)

このセクションの手順は以下によって提供されます

([use-modules](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-use_002dmodules) ([system](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-system) REPLサーバー))

Guileでアプリケーションを作成する場合、REPLでScheme式を評価することでユーザーがアプリケーションと対話できるようにすると便利な場合が多い。

このモジュールの手順を使用すると、ローカル接続または TCP 接続を介して対話できる _REPL サーバー_ を起動できます。Guile 自体も内部的にこれらを使用して \--listen スイッチを実装しています。[コマンドライン オプション](https://doc.guix.gnu.org/guile/latest/en/guile.html#Command_002dline-Options)。

スキーム手順: **make-tcp-server-socket** \[#:host=#f\] \[#:addr\] \[#:port=37146\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dtcp_002dserver_002dsocket)

指定されたアドレス addr とポート番号 port にバインドされたストリームソケットを返します。ホスト名が指定されていて、アドレス addr が指定されていない場合は、ホスト名文字列がアドレスに変換されます。どちらも指定されていない場合は、ループバックアドレスが使用されます。

Scheme Procedure: **make-unix-domain-server-socket** \[#:path="/tmp/guile-socket"\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dunix_002ddomain_002dserver_002dsocket)

指定されたパスにバインドされたUNIXドメインソケットを返します。

Scheme Procedure: **run-server** \[server-socket\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-run_002dserver)

Scheme Procedure: **spawn-server** \[server-socket\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-spawn_002dserver)

REPLを作成して実行し、指定されたサーバーソケット経由で利用できるようにします。サーバーソケットが指定されていない場合は、引数なしで`make-tcp-server-socket`を呼び出して作成されたソケットがデフォルトで使用されます。

`run-server`は現在のスレッドでサーバーを実行しますが、`spawn-server`は新しいスレッドでサーバーを実行します。

スキーム手順: **サーバーとクライアントを停止します!** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-stop_002dserver_002dand_002dclients_0021)

実行中のすべてのサーバーソケットの接続を閉じます。

現在の実装では、REPLスレッドはスタックを巻き戻さずにキャンセルされることにご注意ください。いずれかのスレッドがミューテックスを保持している場合、またはクリティカルセクション内にある場合、結果は未定義です。

* * *

前へ: [REPLサーバー](https://doc.guix.gnu.org/guile/latest/en/guile.html#REPL-Servers)、上へ: [Schemeコードの読み込みと評価](https://doc.guix.gnu.org/guile/latest/en/guile.html#Read_002fLoad_002fEval_002fCompile) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.16.15 協調型 REPL サーバー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Cooperative-REPL-Servers-1)

このセクションの手順は以下によって提供されます

([use-modules](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-use_002dmodules) ([system](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-system) repl coop-server))

通常の REPL サーバーは独自のスレッドで実行されますが ([REPL サーバー](https://doc.guix.gnu.org/guile/latest/en/guile.html#REPL-Servers) を参照)、イベント ループを使用するプログラムやシングル スレッド プログラムなどでは、既存のスレッド内で指定された時間に実行される REPL を提供する方が便利な場合があります。これにより、スレッドの同期を気にすることなく、REPL からプログラムのデータ構造に安全にアクセスしたり変更したりすることが可能になります。

REPLは`spawn-coop-repl-server`と`poll-coop-repl-server`を呼び出すスレッドで実行されますが、呼び出し元のスレッドがブロックされないように専用のスレッドが生成されます。生成されたスレッドはREPLへの入力を読み取り、新しい接続を待ち受けます。

協調型REPLサーバーは、`spawn-coop-repl-server`から返されたオブジェクトを引数として`poll-coop-repl-server`を呼び出すことで、保留中の式を評価するために定期的にポーリングする必要があります。`poll-coop-repl-server`を呼び出したスレッドは、式の評価が完了するまで、またはデバッガが起動されるまでブロックされます。

Scheme Procedure: **spawn-coop-repl-server** \[server-socket\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-spawn_002dcoop_002drepl_002dserver)

新しい協調型REPLサーバーオブジェクトを作成して返し、サーバーソケットへの接続をリッスンする新しいスレッドを生成します。REPLサーバーが正しく機能するためには、返されたサーバーオブジェクトに対して`poll-coop-repl-server`を定期的に呼び出す必要があります。

スキーム手順: **poll-coop-repl-server** coop-server [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-poll_002dcoop_002drepl_002dserver)

協調型REPLサーバーcoop-serverをポーリングし、保留中の操作があれば、REPLプロンプトに入力された式の評価などを実行します。この手順は、`spawn-coop-repl-server`を呼び出したスレッドと同じスレッドから呼び出す必要があります。

* * *

次へ: [モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#Modules)、前: [スキームコードの読み込みと評価](https://doc.guix.gnu.org/guile/latest/en/guile.html#Read_002fLoad_002fEval_002fCompile)、上: [API リファレンス](https://doc.guix.gnu.org/guile/latest/en/guile.html#API-Reference ) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
