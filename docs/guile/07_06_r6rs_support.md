### 7.6 R6RS サポート

R6RSライブラリの定義方法やGuileモジュールとの統合方法の詳細については、[R6RSライブラリ](06_18_modules.md#6186-r6rsライブラリ)を参照してください。

* [R6RSとの非互換性](#761-r6rsとの非互換性)
* [R6RS 標準ライブラリ](#762-r6rs-標準ライブラリ)

* * *

次へ: [R6RS 標準ライブラリ](#762-r6rs-標準ライブラリ)、上へ: [R6RS サポート](#76-r6rs-サポート) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.6.1 R6RSとの非互換性

GuileとR6RSの間にはいくつかの互換性の問題があります。意図的なものもあれば、バグによるもの、あるいは単に未実装の機能によるものもあります。このリストに記載されていない問題を発見した場合は、Guileの開発者にお知らせください。

* R6RSでは、準拠する実装が特定のエラーを通知しなければならない多くの状況が規定されています。Guileは実際にはそれほど気にしていません。正しいR6RSプログラムがそのエラーに遭遇しない場合は、そのエラーをチェックする必要はありません。
* 1つのファイル内で複数の`library`形式を使用する機能はまだサポートされていません。これは、`library`の展開によって現在のモジュールは設定されますが、復元されないためです。これはバグです。
* R6RSのUnicodeエスケープは、Guileの既存のエスケープと競合するため、デフォルトでは無効になっています。文字列内のエスケープされた改行のR6RSの処理についても同様です。
    
R6RS の動作は、リーダーオプションで有効にできます。詳細については、[文字列読み取り構文](06_06_05_strings.md#6651-文字列読み取り構文) を参照してください。
    
* Guile は、`H\x65;llo` (`Hello` と同じ) や `\x3BB;` (`λ` と同じ) などの記号の Unicode エスケープをまだサポートしていません。
* 変数トランスフォーマーへの `set!` は、元の `set!` 式が定義のコンテキストにあった場合でも、式にのみ展開され、定義には展開されません。
* R6RSの第10章で詳述されているアルゴリズムを使用する代わりに、トップレベル形式の展開は順次行われます。
    
例えば、以下のトップレベル定義の展開は正しく動作します。
    
（始める
（定義さえ？）
(ラムダ (x)
（または（= x 0）（奇数？（- x 1）））））
(define構文がおかしい？)
(構文規則()
（（奇数？×）（偶数？×ではない））））
（偶数？ 10）
⇒ #t
    
`begin`ラッパーの外側にある同じ定義では、以下のことは起こりません。
    
（定義さえ？）
(ラムダ (x)
（または（= x 0）（奇数？（- x 1）））））
(define構文がおかしい？)
(構文規則()
（（奇数？×）（偶数？×ではない））））
（偶数？ 10）
<名前のないポート>:4:18: プロシージャ内ですか?:
<名前のないポート>:4:18: 適用するタイプが間違っています: #<syntax-transformer odd?>
    
これは、`even?` の右辺を展開する際に、`odd?` への参照がまだ構文変換子としてマークされていないため、関数であると想定されるためです。
    
このバグはトップレベルプログラムのみに影響し、ライブラリフォーム内のコードには影響しません。トップレベルフォームの修正は可能と思われますが、後方互換性を維持したまま実装するのは困難です。ご提案やパッチをいただければ幸いです。
    
* `(rnrs io ports)` モジュールは未完成です。現在修正作業中です。
* Guile は、バイナリ ポートでのテキスト I/O プロシージャの使用、またはその逆を妨げません。Guileのすべてのポートは、バイナリ I/O とテキスト I/O の両方をサポートしています。詳細については、[Encoding](06_12_input_and_output.md#6123-エンコーディング) を参照してください。
* Guile の `equal?` の実装は、循環を含む引数に適用された場合、終了に失敗する可能性があります。

Guileは、ルートモジュール内に、Guileの従来のデフォルト設定よりもR6RSのデフォルト設定を選択するための手順を公開しています。

Scheme 手順: **install-r6rs!**

ガイルのデフォルト設定をR6RSにより適合するように変更します。

Guile のデフォルト設定は今後変更される可能性がありますが、この手順で実施する変更は、R6RS の規約をより適切にサポートするために、`.sls` と `.guile.sls` をサポートされている `%load-extensions` のセットに追加することです。[ロードパス](06_16_reading_and_evaluating_scheme_code.md#6168-ロードパス) を参照してください。また、文字列で R6RS の Unicode エスケープを有効にします。上記の説明を参照してください。

最後に、`--r6rs` コマンドライン引数は、ユーザーコードを呼び出す前に `install-r6rs!` を呼び出すことに注意してください。R6RS ユーザーは、この引数を Guile に渡したい場合が多いでしょう。

* * *

前へ: [R6RSとの非互換性](#761-r6rsとの非互換性)、上へ: [R6RSのサポート](#76-r6rs-サポート) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.6.2 R6RS 標準ライブラリ

改訂版レポートの以前のバージョンとは対照的に、R6RSでは、準拠する実装に必要な手順と構文形式を「標準ライブラリ」のセットに整理しており、ユーザープログラムやライブラリは必要に応じてこれらをインポートできます。ここでは、Guile用に実装されているライブラリを簡単に一覧表示します。

これらのライブラリの機能のほとんどはGuile自体に既に含まれているため、ここでは詳細な説明は行いません。Guileユーザーの多くは、既に広く知られ、十分に文書化されているGuileモジュールを使用することを想定しています。これらのR6RSライブラリは、主にコードを他のR6RSシステムに移植したいユーザーにとって有用です。

以下のセクションのドキュメントは、レポートのライブラリセクションの内容の一部を再現していますが、主にGuileによるR6RS標準ライブラリの実装に関する補足情報を提供することを目的としています。完全なドキュメント、設計の根拠、およびその他の例については、レポートの「標準ライブラリ」セクション（「アルゴリズム言語スキームに関する改訂版レポート」の[R6RS標準ライブラリ](https://doc.guix.gnu.org/guile/latest/en/r6rs.html#Standard-Libraries)を参照）を参照してください。

* [ライブラリの使用方法](#7621-ライブラリの使用方法)
* [rnrs base](#7622-rnrs-ベース)
* [rnrs unicode](#7623-rnrs-unicode)
* [rnrs bytevectors](#7624-rnrs-バイトベクター)
* [rnrs リスト](#7625-rnrs-リスト)
* [rnrsソート](#7626-rnrs-ソート)
* [rnrs コントロール](#7627-rnrs-コントロール)
* [R6RS レコード](#7628-r6rs-レコード)
* [rnrs レコード構文](#7629-rnrs-レコード構文)
* [rnrs レコード手続き型](#76210-rnrs-レコードの手続き型)
* [rnrs 記録検査](#76211-rnrs-レコードの検査)
* [rnrs 例外](#76212-rnrs-例外)
* [rnrs 条件](#76213-rnrs-条件)
* [I/O条件](#76214-入出力条件)
* [トランスコーダー](#76215-トランスコーダ)
* [rnrs io ポート](#76216-rnrs-io-ポート)
* [R6RS ファイルポート](#76217-r6rs-ファイルポート)
* [rnrs io simple](#76218-rnrs-io-simple)
* [rnrs ファイル](#76219-rnrs-ファイル)
* [rnrs プログラム](#76220-rnrs-プログラム)
* [rnrs 算術固定番号](#76221-rnrs-算術固定番号)
* [rnrs 算術フロナム](#76222-rnrs-算術-flonums)
* [rnrs ビット演算](#76223-rnrs-ビット演算)
* [rnrs syntax-case](#76224-rnrs-構文ケース)
* [rnrs ハッシュテーブル](#76225-rnrs-ハッシュテーブル)
* [rnrs 列挙型](#76226-rnrs-列挙型)
* [rnrs](#76227-rnrs)
* [rnrs eval](#76228-rnrs-eval)
* [rnrs mutable-pairs](#76229-rnrs-mutable-pairs)
* [rnrs mutable-strings](#76230-rnrs-mutable-strings)
* [rnrs r5rs](#76231-rnrs-r5rs)

* * *

次へ: [rnrs base](#7622-rnrs-ベース)、上へ: [R6RS Standard Libraries](#762-r6rs-標準ライブラリ) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.6.2.1 ライブラリの使用方法

Guile は、R6RS の「ライブラリ」形式を、ネイティブ Guile モジュール定義への変換として実装しています。このため、以下のサブセクションで説明するすべてのライブラリは、R6RS ライブラリやトップレベルプログラムで使用できるだけでなく、通常の Guile モジュールのように、例えば `use-modules` 形式を介してインポートすることもできます。例えば、R6RS の「コンポジット」ライブラリは次のようにインポートできます。

(import (rnrs (6)))

([use-modules](06_18_modules.md#6182-guileモジュールの使用) ((rnrs) :version (6)))

Guileのライブラリ実装の詳細については、[R6RSライブラリ](06_18_modules.md#6186-r6rsライブラリ)を参照してください。

* * *

次へ: [rnrs unicode](#7623-rnrs-unicode)、前: [ライブラリの使用法](#7621-ライブラリの使用方法)、上: [R6RS 標準ライブラリ](#762-r6rs-標準ライブラリ) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.6.2.2 rnrs ベース

`(rnrs base (6))`ライブラリは、アルゴリズム言語スキームに関する改訂版^6レポートのメインセクションで説明されている手順と構文形式をエクスポートします（[R6RSベースライブラリ](https://doc.guix.gnu.org/guile/latest/en/r6rs.html#Base-library)を参照）。これらは、対応する既存のマニュアルセクションごとに以下にグループ化されています。

スキームプロシージャ: **boolean?** obj

Scheme Procedure: **not** x

ドキュメントについては、[ブール値](06_06_01_booleans.md#661-ブール値)を参照してください。

Scheme Procedure: **symbol?** obj

Scheme手順: **symbol->string** sym

Scheme手順: **string->symbol** str

ドキュメントについては、[シンボルに関連する操作](06_06_06_symbols.md#6664-シンボルに関連する操作)を参照してください。

Scheme手順: **char?** obj

Scheme手順: **char=?**

Scheme手順: **char<?**

Scheme手順: **char>?**

Scheme手順: **char<=?**

Scheme手順: **char>=?**

スキーム手順: **integer->char** n

スキーム手順: **char->integer** chr

ドキュメントについては、[文字](06_06_03_characters.md#663-文字)を参照してください。

スキーム手順: **list?** x

Scheme Procedure: **null?** x

ドキュメントについては、[リスト述語](06_06_09_lists.md#6692-リスト述語)を参照してください。

スキーム手順: **pair?** x

スキーム手順: **cons** xy

スキーム手順: **car** ペア

スキーム手順: **cdr** ペア

スキーム手順: **caar** ペア

スキーム手順: **cadr** ペア

スキーム手順: **cdar** ペア

スキーム手順: **cddr** ペア

Scheme Procedure: **caaar** ペア

Scheme手順: **caadr**ペア

Scheme Procedure: **cadar** ペア

スキーム手順: **cdaar** ペア

Scheme手順: **caddr**ペア

Scheme手順: **cdadr**ペア

スキーム手順: **cddar** ペア

スキーム手順: **cdddr** ペア

スキーム手順: **caaar** ペア

Scheme Procedure: **caaadr** ペア

Scheme Procedure: **caadar** ペア

スキーム手順: **cadaar** ペア

Scheme Procedure: **cdaaar** ペア

スキーム手順: **cddaar** ペア

Scheme Procedure: **cdadar** ペア

Scheme手順: **cdaadr**ペア

スキーム手順: **cadadr** ペア

Scheme Procedure: **caaddr** ペア

スキーム手順: **caddar** ペア

スキーム手順: **cadddr** ペア

Scheme Procedure: **cdaddr** ペア

Scheme Procedure: **cddadr** ペア

スキーム手順: **cdddar** ペア

スキーム手順: **cddddr** ペア

ドキュメントについては、[Pairs](06_06_08_pairs.md#668-ペア)を参照してください。

スキーム手順: **number?** obj

ドキュメントについては、[Scheme の数値「タワー」](06_06_02_numerical_data_types.md#6621-scheme-の数値タワー) を参照してください。

Scheme プロシージャ: **string?** obj

ドキュメントについては、[String Predicates](06_06_05_strings.md#6652-文字列述語)を参照してください。

スキームプロシージャ: **procedure?** obj

ドキュメントについては、[プロシージャのプロパティとメタ情報](06_07_procedures.md#677-プロシージャのプロパティとメタ情報)を参照してください。

Scheme構文: **define** 名前 値

Scheme構文: **set!** 変数名 値

ドキュメントについては、[変数の定義と設定](03_hello_scheme.md#313-変数の定義と設定)を参照してください。

Scheme構文: **define-syntax**キーワード式

Scheme構文: **let構文** ((キーワード トランスフォーマー) …) exp1 exp2 …

Scheme構文: **letrec-syntax** ((キーワード トランスフォーマー) …) exp1 exp2 …

ドキュメントについては、[マクロの定義](06_08_macros.md#681-マクロの定義)を参照してください。

Scheme構文: **identifier-syntax** exp

ドキュメントについては、[識別子マクロ](06_08_macros.md#686-識別子マクロ)を参照してください。

Scheme構文: **構文規則**リテラル（パターンテンプレート）...

ドキュメントについては、[構文規則マクロ](06_08_macros.md#682-構文規則マクロ)を参照してください。

Scheme構文: **ラムダ** 形式体の本体

[Lambda: 基本プロシージャの作成](06_07_procedures.md#671-ラムダ-基本的なプロシージャの作成)を参照してください。

Scheme構文: **let**バインディング本体

Scheme構文: **let\*** バインディング本体

Scheme構文: **letrec** バインディング本体

Scheme構文: **letrec\*** バインディング本体

ドキュメントについては、[ローカル変数バインディング](06_10_definitions_and_variable_bindings.md#6102-ローカル変数バインディング)を参照してください。

Scheme構文: **let-values** バインディング本体

Scheme構文: **let\*-values** バインディング本体

ドキュメントについては、[SRFI-11 - let-values](07_05_10_srfi11_letvalues.md#7510-srfi-11---let-values)を参照してください。

Scheme構文: **begin** expr1 expr2 ...

ドキュメントについては、[シーケンスとスプライシング](06_11_controlling_the_flow_of_program_execution.md#6111-シーケンスとスプライシング)を参照してください。

Scheme構文: **quote** expr

Scheme構文: **quasiquote** expr

Scheme構文: **unquote** expr

Scheme構文: **unquote-splicing** expr

ドキュメントについては、[式構文](06_16_reading_and_evaluating_scheme_code.md#61611-式の構文)を参照してください。

Scheme構文: **if** テスト結果 \[代替案]

Scheme構文: **cond** clause1 clause2 ...

Scheme構文: **case** key clause1 clause2 ...

ドキュメントについては、[単純な条件評価](06_11_controlling_the_flow_of_program_execution.md#6112-単純な条件評価)を参照してください。

Scheme構文: **and** expr ...

Scheme構文: **or**式 ...

ドキュメントについては、[一連の式の条件付き評価](06_11_controlling_the_flow_of_program_execution.md#6113-式のシーケンスの条件付き評価)を参照してください。

スキーム手順: **eq?** xy

スキーム手順: **eqv?** xy

Scheme Procedure: **equal?** xy

スキーム手順: **symbol=?** symbol1 symbol2 ...

ドキュメントについては、[Equality](06_09_general_utility_functions.md#691-平等)を参照してください。

`symbol=?` は `eq?` と同じです。

スキーム手順: **complex?** z

ドキュメントについては、[複素数](06_06_02_numerical_data_types.md#6624-複素数)を参照してください。

スキーム手順: **real-part** z

スキーム手順: **imag-part** z

スキーム手順: **make-rectangular** 実部 虚部

Scheme 手順: **make-polar** xy

スキーム手順: **magnitude** z

スキーム手順: **角度** z

ドキュメントについては、[複素数演算](06_06_02_numerical_data_types.md#66210-複素数演算)を参照してください。

Scheme Procedure: **sqrt** z

Scheme Procedure: **exp** z

スキーム手順: **expt** z1 z2

スキーム手順: **log** z

スキーム手順: **sin** z

スキーム手順: **cos** z

スキーム手順: **tan** z

スキーム手順: **asin** z

スキーム手順: **acos** z

スキーム手順: **atan** z

ドキュメントについては、[科学関数](06_06_02_numerical_data_types.md#66212-科学関数)を参照してください。

スキーム手順: **real?** x

スキーム手順: **有理数?** x

スキーム手順: **numerator** x

スキーム手順: **分母** x

Scheme Procedure: **rationalize** x eps

ドキュメントについては、[実数と有理数](06_06_02_numerical_data_types.md#6623-実数と有理数)を参照してください。

Scheme Procedure: **exact?** x

Scheme Procedure: **inexact?** x

Scheme Procedure: **exact** z

Scheme Procedure: **inexact** z

ドキュメントについては、[正確な数値と不正確な数値](06_06_02_numerical_data_types.md#6625-正確な数と不正確な数)を参照してください。`exact`と`inexact`の手順は、Guileのコードライブラリが提供する`inexact->exact`と`exact->inexact`の手順と同一です。

スキームプロシージャ: **integer?** x

ドキュメントについては、[整数](06_06_02_numerical_data_types.md#6622-整数)を参照してください。

スキーム手順: **odd?** n

スキーム手順: **even?** n

スキーム手順: **gcd** x ...

スキーム手順: **lcm** x ...

スキーム手順: **exact-integer-sqrt** k

ドキュメントについては、[整数値の演算](06_06_02_numerical_data_types.md#6627-整数値に対する演算)を参照してください。

Scheme Procedure: **\=**

スキーム手順: **<** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- _003c-1)

スキーム手順: **\>**

Scheme Procedure: **<=**

Scheme Procedure: **\>=**

スキーム手順: **ゼロ?** x

スキーム手順: **肯定？** x

スキーム手順: **否定？** x

ドキュメントについては、[比較述語](07_05_49_srfi207_stringnotated_bytevectors.md#75496-比較)を参照してください。

スキーム手順: **for-each** f lst1 lst2 ...

ドキュメントについては、[Fold, Unfold & Map](07_05_03_srfi1_list_library.md#7535-折りたたみ展開マップ) を参照してください。

Scheme手順: **list** elem …

ドキュメントについては、[リストコンストラクタ](06_06_09_lists.md#6693-リストコンストラクタ)を参照してください。

Scheme Procedure: **length** lst

スキーム手順: **list-ref** lst k

Scheme Procedure: **list-tail** lst k

ドキュメントについては、[リスト選択](06_06_09_lists.md#6694-リスト選択)を参照してください。

Scheme Procedure: **append** lst … obj

Scheme Procedure: **append**

Scheme手順: **reverse** lst

ドキュメントについては、[Append and Reverse](06_06_09_lists.md#6695-追加と逆順)を参照してください。

Scheme手順: **number->string** n \[radix\]

Scheme手順: **string->number** str \[radix\]

ドキュメントについては、[数値と文字列の変換](07_05_49_srfi207_stringnotated_bytevectors.md#75493-変換)を参照してください。

Scheme手順: **string** char ...

Scheme手順: **make-string** k \[chr\]

Scheme手順: **list->string** lst

ドキュメントについては、[String Constructors](06_06_05_strings.md#6653-文字列コンストラクタ)を参照してください。

Scheme手順: **string->list** str \[start \[end\]\]

ドキュメントについては、[リスト/文字列変換](06_06_05_strings.md#6654-リスト文字列変換)を参照してください。

Scheme手順: **string-length** str

Scheme手順: **string-ref** str k

Scheme手順: **string-copy** str \[start \[end\]\]

Scheme手順: **substring** str start \[end\]

ドキュメントについては、[文字列選択](06_06_05_strings.md#6655-文字列の選択)を参照してください。

Scheme手順: **string=?** s1 s2 s3 …

Scheme手順: **string<?** s1 s2 s3 …

Scheme手順: **string>?** s1 s2 s3 …

Scheme手順: **string<=?** s1 s2 s3 …

Scheme手順: **string>=?** s1 s2 s3 …

ドキュメントについては、[文字列比較](06_06_05_strings.md#6657-文字列の比較)を参照してください。

Scheme プロシージャ: **string-append** arg …

ドキュメントについては、[文字列の反転と追加](06_06_05_strings.md#66510-文字列の反転と追加)を参照してください。

Scheme プロシージャ: **string-for-each** proc s \[start \[end\]\]

ドキュメントについては、[マッピング、折りたたみ、展開](06_06_05_strings.md#66511-マッピングフォールディングおよびアンフォールディング)を参照してください。

スキームプロシージャ: **+** z1 ...

スキーム手順: **\-** z1 z2 ...

スキームプロシージャ: **\*** z1 ...

スキーム手順: **/** z1 z2 ...

スキーム手順: **max** x1 x2 ...

スキーム手順: **min** x1 x2 ...

スキーム手順: **abs** x

Scheme手順: **truncate** x

スキーム手順: **floor** x

スキーム手順: **ceiling** x

スキーム手順: **ラウンド** x

ドキュメントについては、[算術関数](06_06_02_numerical_data_types.md#66211-算術関数)を参照してください。

Scheme Procedure: **div** xy

スキーム手順: **mod** xy

スキーム手順: **div-and-mod** xy

これらの手順では、2 つの実数 x と y を受け取ります。ただし、除数 y はゼロ以外でなければなりません。`div` は整数 q を返し、`mod` は実数 r を返します。ただし、_x = q\*y + r_ かつ _0 <= r < abs(y)_ を満たすものとします。`div-and-mod` は q と r の両方を返し、それぞれを個別に計算するよりも効率的です。なお、_y > 0_ の場合、`div` は _floor(x/y)_ を返し、それ以外の場合は _ceiling(x/y)_ を返します。

([div](#7622-rnrs-ベース) 123 10) ⇒ 12
([mod](#7622-rnrs-ベース) 123 10) ⇒ 3
([div-and-mod](#7622-rnrs-ベース) 123 10) ⇒ 12 と 3
([div-and-mod](#7622-rnrs-ベース) 123 \-10) ⇒ \-12 and 3
([div-and-mod](#7622-rnrs-ベース) \-123 10) ⇒ \-13 and 7
([div-and-mod](#7622-rnrs-ベース) \-123 \-10) ⇒ 13 と 7
([div-and-mod](#7622-rnrs-ベース) \-123.2 \-63.5) ⇒ 2.0 および 3.8
([div-and-mod](#7622-rnrs-ベース) 16/3 \-10/7) ⇒ \-3 および 22/21

スキーム手順: **div0** xy

スキーム手順: **mod0** xy

スキーム手順: **div0-and-mod0** xy

これらの手順では、2 つの実数 x と y を受け取ります。ただし、除数 y はゼロ以外でなければなりません。`div0` は整数 q を返し、`mod0` は _x = q\*y + r_ かつ _\-abs(y/2) <= r < abs(y/2)_ を満たす実数 r を返します。`div0-and-mod0` は q と r の両方を返し、それぞれを個別に計算するよりも効率的です。

`div0` は _x/y_ を最も近い整数に丸めた値を返すことに注意してください。_x/y_ が 2 つの整数のちょうど中間にある場合、y の符号に従って同点が解消されます。_y > 0_ の場合、同点は正の無限大に丸められ、そうでない場合は負の無限大に丸められます。これは、_\-abs(y/2) <= r < abs(y/2)_ という要件の結果です。

([div0](#7622-rnrs-ベース) 123 10) ⇒ 12
([mod0](#7622-rnrs-ベース) 123 10) ⇒ 3
([div0-and-mod0](#7622-rnrs-ベース) 123 10) ⇒ 12 と 3
([div0-and-mod0](#7622-rnrs-ベース) 123 \-10) ⇒ \-12 と 3
([div0-and-mod0](#7622-rnrs-ベース) \-123 10) ⇒ \-12 および \-3
([div0-and-mod0](#7622-rnrs-ベース) \-123 \-10) ⇒ 12 と \-3
([div0-and-mod0](#7622-rnrs-ベース) \-123.2 \-63.5) ⇒ 2.0 および 3.8
([div0-and-mod0](#7622-rnrs-ベース) 16/3 \-10/7) ⇒ \-4 および \-8/21

スキーム手順: **実数値?** obj

スキーム手順: **有理値?** obj

Scheme手順: **整数値?** obj

数値精度を損なうことなく実数、有理数、または整数値に変換できる場合に限り、`#t` を返します。

`real-valued?` は、虚数部がゼロの複素数に対して `#t` を返します。

スキーム手順: **nan?** x

Scheme Procedure: **infinite?** x

Scheme Procedure: **finite?** x

`nan?` は、x が NaN 値の場合は `#t` を返し、それ以外の場合は `#f` を返します。`infinite?` は、x が無限大の場合は `#t` を返し、それ以外の場合は `#f` を返します。`finite?` は、x が無限大でも NaN 値でもない場合は `#t` を返し、それ以外の場合は `#f` を返します。すべての実数は、これらの述語のうち 1 つだけを満たします。x が実数でない場合は例外が発生します。

Scheme構文: **assert** expr

expr が `#f` と評価された場合は `&assertion` 条件を発生させ、それ以外の場合は expr の値と評価します。

スキーム手順: **エラー** 誰がメッセージを irritant1 に送信しました...

スキーム手順: **assertion-violation** who message irritant1 ...

これらの手順は、引数に基づいて複合条件を生成します。who が `#f` でない場合、条件には `who` フィールドが who に設定された `&who` 条件が含まれます。`message` フィールドが message と等しい `&message` 条件が含まれます。`irritant1 ...` で指定された `irritants` リストを持つ `&irritants` 条件が含まれます。

`error` は、上記で説明した単純な条件と `&error` 条件を組み合わせた複合条件を生成します。`assertion-violation` は、`&assertion` 条件を含む複合条件を生成します。

Scheme手順: **vector-map** proc v

Scheme プロシージャ: **vector-for-each** proc v

これらの手順は、ベクトルに対する`map`および`for-each`契約を実装します。

Scheme手順: **vector** arg …

Scheme手順: **vector?** obj

Scheme手順: **make-vector** len

Scheme手順: **make-vector** len fill

Scheme手順: **list->vector** l

Scheme手順: **vector->list** v

ドキュメントについては、[動的ベクトルの作成と検証](06_06_10_vectors.md#66102-動的ベクトルの作成と検証)を参照してください。

Scheme手順: **vector-length** ベクトル

Scheme Procedure: **vector-ref** vector k

Scheme 手順: **vector-set!** vector k obj

Scheme 手順: **vector-fill!** v fill

ドキュメントについては、[ベクトルコンテンツへのアクセスと変更](06_06_10_vectors.md#66103-ベクトルコンテンツへのアクセスと変更)を参照してください。

Scheme Procedure: **call-with-current-continuation** proc

スキーム手順: **call/cc** proc

ドキュメントについては、[継続](06_11_controlling_the_flow_of_program_execution.md#6116-継続)を参照してください。

スキームプロシージャ: **values** arg …

スキーム手順: **call-with-values** プロデューサー コンシューマー

ドキュメントについては、[複数の値の返却と受け入れ](06_11_controlling_the_flow_of_program_execution.md#6117-複数の値の返却と受け入れ)を参照してください。

スキーム手順: **dynamic-wind** in\_guard thunk out\_guard

ドキュメントについては、[Dynamic Wind](06_11_controlling_the_flow_of_program_execution.md#61110-ダイナミックウィンド)を参照してください。

Scheme プロシージャ: **apply** proc arg … arglst

ドキュメントについては、[オンザフライ評価の手順](06_16_reading_and_evaluating_scheme_code.md#6165-オンザフライ評価の手順)を参照してください。

* * *

次へ: [rnrs バイトベクトル](#7624-rnrs-バイトベクター)、前: [rnrs ベース](#7622-rnrs-ベース)、上: [R6RS 標準ライブラリ](#762-r6rs-標準ライブラリ) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.6.2.3 rnrs unicode

`(rnrs unicode (6))`ライブラリは、Unicode文字と文字列を操作するための手順を提供します。

Scheme手順: **char-upcase** char

Scheme 手順: **char-downcase** char

Scheme 手順: **char-titlecase** char

Scheme手順: **char-foldcase** char

これらの手順は、引数をある Unicode 文字セットから別の Unicode 文字セットに変換します。`char-upcase`、`char-downcase`、および `char-titlecase` は、Guile コアライブラリの対応するものと同じです。詳細については、[Characters](06_06_03_characters.md#663-文字) を参照してください。

`char-foldcase` は、引数に `char-upcase` を適用した後、`char-downcase` を適用した結果を返します。ただし、トルコ語の文字 `U+0130` と `U+0131` の場合は、この手順は恒等関数として機能します。

スキーム手順: **char-ci=?** char1 char2 char3 ...

スキーム手順: **char-ci<?** char1 char2 char3 ...

スキーム手順: **char-ci>?** char1 char2 char3 ...

Scheme Procedure: **char-ci<=?** char1 char2 char3 ...

Scheme 手順: **char-ci>=?** char1 char2 char3 ...

これらの手順は、Unicode文字の大文字小文字を区別しない比較を容易にします。これらはGuileのコアライブラリが提供する手順と同一です。詳細については、[文字](06_06_03_characters.md#663-文字)を参照してください。

Scheme手順: **char-alphabetic?** char

Scheme 手順: **char-numeric?** char

Scheme 手順: **char-whitespace?** char

Scheme 手順: **char-upper-case?** char

Scheme 手順: **char-lower-case?** char

Scheme プロシージャ: **char-title-case?** char

これらのプロシージャは、さまざまなUnicode文字セット述語を実装しています。これらはGuileのコアライブラリが提供するプロシージャと同一です。詳細については、[Characters](06_06_03_characters.md#663-文字)を参照してください。

Scheme Procedure: **char-general-category** char

ドキュメントについては、[文字](06_06_03_characters.md#663-文字)を参照してください。

Scheme手順: **string-upcase** string

Scheme手順: **string-downcase** string

Scheme手順: **string-titlecase** string

Scheme手順: **string-foldcase** string

これらの手順は、入力に対してUnicodeの大文字小文字変換処理を実行します。詳細については、[Alphabetic Case Mapping](06_06_05_strings.md#6659-アルファベットの大文字小文字のマッピング)を参照してください。

Scheme Procedure: **string-ci=?** string1 string2 string3 ...

Scheme手順: **string-ci<?** string1 string2 string3 ...

Scheme Procedure: **string-ci>?** string1 string2 string3 ...

Scheme Procedure: **string-ci<=?** string1 string2 string3 ...

Scheme Procedure: **string-ci>=?** string1 string2 string3 ...

これらの手順では、入力に対して大文字小文字を区別しない比較を実行します。詳細については、[文字列比較](06_06_05_strings.md#6657-文字列の比較)を参照してください。

Scheme手順: **string-normalize-nfd** string

Scheme手順: **string-normalize-nfkd** string

Scheme手順: **string-normalize-nfc** string

Scheme手順: **string-normalize-nfkc** string

これらの手順は、入力に対してUnicode文字列の正規化操作を実行します。詳細については、[文字列比較](06_06_05_strings.md#6657-文字列の比較)を参照してください。

* * *

次へ: [rnrs リスト](#7625-rnrs-リスト)、前: [rnrs unicode](#7623-rnrs-unicode)、上: [R6RS 標準ライブラリ](#762-r6rs-標準ライブラリ) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.6.2.4 rnrs バイトベクター

`(rnrs bytevectors (6))` ライブラリは、バイナリデータのブロックを操作するための手順を提供します。この機能については、マニュアルの専用セクションで説明されています。[Bytevectors](06_06_12_bytevectors.md#6612-バイトベクトル) を参照してください。

* * *

次へ: [rnrs ソート](#7626-rnrs-ソート)、前: [rnrs バイトベクトル](#7624-rnrs-バイトベクター)、上: [R6RS 標準ライブラリ](#762-r6rs-標準ライブラリ) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.6.2.5 rnrs リスト

`(rnrs lists (6))`ライブラリは、リストを操作するための追加の手順を提供します。

Scheme手順: **find** proc list

この手順は、Guile の SRFI-1 実装で定義されている手順と同一です。詳細については、[Searching](07_05_03_srfi1_list_library.md#7537-検索) を参照してください。

Scheme Procedure: **for-all** proc list1 list2 ...

Scheme プロシージャ: **exists** proc list1 list2 ...

`for-all` プロシージャは、SRFI-1 で定義されている `every` プロシージャと同一です。`exists`プロシージャは、SRFI-1 の `any` と同一です。詳細については、[Searching](07_05_03_srfi1_list_library.md#7537-検索) を参照してください。

Scheme プロシージャ: **filter** proc list

Scheme Procedure: **partition** proc list

これらの手順は、SRFI-1 で提供される手順と同一です。`filter` の説明については、[リストの変更](06_06_09_lists.md#6696-リストの変更) を参照してください。`partition` については、[フィルタリングとパーティショニング](07_05_03_srfi1_list_library.md#7536-フィルタリングとパーティショニング) を参照してください。

Scheme Procedure: **fold-right** combine nil list1 list2 …

この手順は、SRFI-1 が提供する `fold-right` 手順と同一です。詳細については、[Fold, Unfold & Map](07_05_03_srfi1_list_library.md#7535-折りたたみ展開マップ) を参照してください。

Scheme Procedure: **fold-left** combine nil list1 list2 …

この手順は SRFI-1 の `fold` に似ていますが、combine はシードを最初の引数として呼び出されます。詳細については、[Fold, Unfold & Map](07_05_03_srfi1_list_library.md#7535-折りたたみ展開マップ) を参照してください。

Scheme手順: **remp** proc list

Scheme手順: オブジェクトリストの**削除**

Scheme手順: **remv** オブジェクトリスト

Scheme手順: **remq** オブジェクトリスト

`remove`、`remv`、および`remq`は、Guileのコアライブラリが提供する`delete`、`delv`、および`delq`プロシージャと同一です（[リストの変更](06_06_09_lists.md#6696-リストの変更)を参照）。`remp`は、SRFI-1が提供する代替の`remove`プロシージャと同一です（[削除](07_05_03_srfi1_list_library.md#7538-削除)を参照）。

Scheme手順: **memp** proc list

Scheme Procedure: **member** obj list

Scheme手順: **memv** オブジェクトリスト

Scheme手順: **memq** オブジェクトリスト

`member`、`memv`、および`memq`は、Guileのコアライブラリが提供するプロシージャと同じです。ドキュメントについては、[リスト検索](06_06_09_lists.md#6697-リスト検索)を参照してください。`memp`は、指定された述語関数`proc`を使用してリストlistの要素をテストします。`find`と同様の動作をしますが、`car`がprocを満たすlistの最初のサブリストを返します。

スキームプロシージャ: **assp** proc alist

スキーム手順: **assoc** オブジェクトリスト

スキームプロシージャ: **assv** オブジェクトリスト

Scheme Procedure: **assq** obj alist

`assoc`、`assv`、および`assq`は、Guileのコアライブラリが提供するプロシージャと同じです。詳細については、[Alist Key Equality](06_06_20_association_lists.md#66201-alistキーの等価性)を参照してください。`assp`は、指定された述語関数`proc`を使用して、関連付けリストalist内のキーをテストします。

Scheme Procedure: **cons\*** obj1 ... obj

Scheme Procedure: **cons\*** obj

この手順は、Guile のコアライブラリによってエクスポートされる手順と同一です。詳細については、[リストコンストラクタ](06_06_09_lists.md#6693-リストコンストラクタ) を参照してください。

* * *

次へ: [rnrs コントロール](#7627-rnrs-コントロール)、前: [rnrs リスト](#7625-rnrs-リスト)、上: [R6RS 標準ライブラリ](#762-r6rs-標準ライブラリ) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.6.2.6 rnrs ソート

`(rnrs sorting (6))`ライブラリは、リストとベクトルをソートするための手順を提供します。

Scheme プロシージャ: **list-sort** proc list

Scheme プロシージャ: **vector-sort** proc vector

これらのプロシージャは、元のデータを変更せずに、入力を昇順にソートして返します。proc は、入力リストまたはベクトルから 2 つの要素を引数として受け取り、最初の要素が 2 番目の要素より「小さい」場合は true を、それ以外の場合は `#f` を返すプロシージャでなければなりません。`list-sort` はリストを返し、`vector-sort` はベクトルを返します。

`list-sort`と`vector-sort`はどちらも、Guileのコアライブラリの`stable-sort`プロシージャに基づいて実装されています。このプロシージャの動作については、[Sorting](06_09_general_utility_functions.md#693-ソート)を参照してください。

Scheme プロシージャ: **vector-sort!** proc vector

`vector-sort!` は、上記のように proc を使用して要素の昇順を決定する、破壊的な「インプレース」ソートを実行します。`vector-sort!` は未指定の値を返します。

この手順は、Guile のコアライブラリの `sort!` 手順に基づいて実装されています。詳細については、[ソート](06_09_general_utility_functions.md#693-ソート) を参照してください。

* * *

次へ: [R6RS レコード](#7628-r6rs-レコード)、前: [rnrs ソート](#7626-rnrs-ソート)、上: [R6RS 標準ライブラリ](#762-r6rs-標準ライブラリ) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.6.2.7 rnrs コントロール

`(rnrs control (6))`ライブラリは、条件式を構築したり、実行の流れを制御したりするのに便利な構文形式を提供します。

Scheme構文: **when** test expression1 expression2 ...

Scheme構文: **unless** test expression1 expression2 ...

`when`形式は、指定されたテスト式を評価することによって評価されます。結果が真の値である場合、それに続く式が順番に評価され、最後の式の値が`when`式全体の値になります。

`unless` 形式も同様に動作しますが、指定された式は test の値が false の場合にのみ評価されるという点が異なります。

Scheme構文: **do** ((変数初期化ステップ) ...) (テスト式 ...) コマンド ...

この形式は、Guile のコアライブラリが提供する形式と同一です。詳細については、[反復メカニズム](06_11_controlling_the_flow_of_program_execution.md#6114-反復メカニズム) を参照してください。

Scheme構文: **case-lambda**節 ...

この形式は、Guile のコアライブラリが提供する形式と同一です。詳細については、[Case-lambda](06_07_procedures.md#675-case-lambda) を参照してください。

* * *

次へ: [rnrs レコード構文](#7629-rnrs-レコード構文)、前: [rnrs コントロール](#7627-rnrs-コントロール)、上: [R6RS 標準ライブラリ](#762-r6rs-標準ライブラリ) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.6.2.8 R6RS レコード

以下のマニュアルセクションでは、ユーザー定義データ型をサポートするGuileのR6RSレコードの実装について説明します。R6RSレコードAPIは、Guileの「ネイティブ」レコードとSRFI-9レコードAPIが提供する機能のスーパーセットを提供します。これらのインターフェースの説明については、[レコード](06_06_17_records.md#6617-レコード)および[SRFI-9レコード](06_06_16_srfi9_records.md#6616-srfi-9-レコード)を参照してください。

SRFI-9やGuileのネイティブレコードと同様に、R6RSレコードは、レコード名、フィールド、およびそれらのフィールドの変更可能性などの属性を指定するレコードタイプ記述子を使用して構築されます。

R6RSレコードは、このフレームワークを拡張し、定義時にレコード型の「親」型を指定することで、単一継承をサポートします。親型のフィールドに対するアクセサおよびミューテーター手続きは、この親型のサブタイプのレコードに適用できます。レコード型は_sealed_することができ、その場合は他のレコード型の親として使用することはできません。

レコード型の継承メカニズムは、レコードとその親のフィールドを初期化するプロセスにも影響を与えます。レコード型の新しいインスタンスを生成するコンストラクタ手続きは、レコードコンストラクタ記述子から取得されます。この記述子には、構築対象のレコードのレコード型記述子と、レコードサブタイプのコンストラクタが親タイプのコンストラクタに処理を委譲する方法を定義するプロトコル手続きがカプセル化されています。

プロトコルとは、レコードシステムが構築時に、構築中のレコードのフィールドに引数をバインドするために使用する手順です。プロトコル手順には、レコードの親型を構築するために必要な引数を受け取る手順nが渡されます。この手順nが呼び出されると、レコード型自体の新しいインスタンスを構築するために必要な引数を受け取り、レコード型の新しいインスタンスを返す手順pが返されます。

プロトコルは、n と p を使用してレコード型とその親型のフィールドを初期化するプロシージャを返す必要があります。このプロシージャは、によって返されるコンストラクタになります。

簡単な例として、画面上のxy座標をカプセル化する架空のレコード型`pixel`と、`pixel`を親型とし、追加の座標を格納する`voxel`を考えてみましょう。以下のプロトコルは、3つの座標すべてを受け取り、最初の2つの座標を使用して`pixel`のフィールドを初期化し、3つ目の座標を`voxel`の単一のフィールドにバインドするコンストラクタ手順を生成します。

(ラムダ(n)
(ラムダ (xyz)
(let ((p (nxy)))
(pz))))

プロトコルを、ヘルパープロシージャnによって結合された委譲コンストラクタのチェーンを生成する「コンストラクタファクトリー」と考えると役立つかもしれません。

R6RSレコードタイプは、一意に生成されたシンボルまたはユーザーが提供するシンボル（つまり、uid）を使用することで、_非生成_として宣言できます。これにより、同じuidと属性を持つ後続のレコードタイプ宣言は、以前に宣言されたレコードタイプ記述子を返します。

R6RSレコードタイプは_opaque_として宣言することもできます。その場合、`(rnrs records introspection)`で定義されているさまざまな述語と内省手順は、このタイプのレコードがレコードではないかのように動作します。

R6RSレコードAPIは、SRFI-9およびネイティブGuileレコードAPIと多くの名前空間を共有していますが、現時点ではどちらとも互換性がないことに注意してください。

* * *

次へ: [rnrs レコード手続き](#76210-rnrs-レコードの手続き型)、前: [R6RS レコード](#7628-r6rs-レコード)、上: [R6RS 標準ライブラリ](#762-r6rs-標準ライブラリ) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.6.2.9 rnrs レコード構文

`(rnrs records syntactic (6))`ライブラリは、R6RSレコードを操作するための構文APIをエクスポートします。

Scheme構文: **define-record-type** name-spec record-clause …

新しいレコード型を定義し、レコード型記述子、レコードコンストラクタ記述子、コンストラクタ手続き、レコード述語、および新しいレコード型のフィールドに対するアクセサ手続きとミューテータ手続きのバインディングを導入します。

name-spec は識別子であるか、または `(record-name constructor-name predicate-name)` の形式をとる必要があります。ここで、record-name、constructor-name、predicate-name はすべて識別子であり、それぞれレコード型記述子、コンストラクタ、述語プロシージャがバインドされる名前を指定します。name-spec が識別子のみの場合は、生成されたレコード型記述子がバインドされる名前を指定します。

各記録条項は、以下のいずれかでなければならない。

* `(fields field-spec*)`。ここで、各 field-spec は新しいレコードタイプのフィールドを指定し、以下のいずれかの形式をとります。
nameで指定された名前にそのフィールドのアクセサプロシージャをバインドします。
* `(mutable field-name accessor-name mutator-name)` は、field-name という名前の可変フィールドを指定し、accessor プロシージャと mutator プロシージャをそれぞれ accessor-name と mutator-name にバインドします。
* `(immutable field-name)` は、field-name という名前の不変フィールドを指定します。このフィールドのアクセサプロシージャが作成され、レコード名と field-name をハイフンで区切って追加した名前が付けられます。
* `(mutable field-name`) は、field-name という名前の可変フィールドを指定します。そのフィールドに対するアクセサプロシージャが作成され、上記のように命名されます。また、アクセサ名に `-set!` を追加することで、ミューテータープロシージャも作成され、命名されます。
* `field-name`は、field-nameという名前の不変フィールドを指定します。そのフィールドへのアクセス手順は、上記の説明に従って作成され、名前が付けられます。
* `(parent parent-name)`。ここでparent-nameは、新しいレコードタイプの親として使用されるレコードタイプの名前を示すシンボルです。
* `(プロトコル式)`。ここで、式は上記のように動作するプロトコル手順に評価され、新しいレコードタイプのレコードコンストラクタ記述子を作成するために使用されます。
* `(sealed sealed?)`、ここで sealed? は、新しいレコードタイプがシールされているかどうかを指定するブール値です。
* `(opaque opaque?)`。ここで opaque? は、新しいレコードタイプが不透明かどうかを指定するブール値です。
* `(nongenerative [uid])` は、オプションの uid を介してレコードタイプが非生成であることを指定します。uid が指定されていない場合、展開時に一意の uid が生成されます。
* `(parent-rtd parent-rtd parent-cd)`は、上記の`parent`形式のより明示的な形式です。parent-rtdとparent-cdは、それぞれレコード型記述子とレコード構築記述子に評価される必要があります。

Scheme構文: **record-type-descriptor** record-name

レコード名で指定された型に関連付けられたレコード型記述子に評価されます。

Scheme構文: **record-constructor-descriptor** record-name

レコード名で指定された型に関連付けられたレコードコンストラクタ記述子に評価されます。

* * *

次へ: [rnrs レコード検査](#76211-rnrs-レコードの検査)、前: [rnrs レコード構文](#7629-rnrs-レコード構文)、上: [R6RS 標準ライブラリ](#762-r6rs-標準ライブラリ) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.6.2.10 rnrs レコードの手続き型

`(rnrs records procedural (6))`ライブラリは、R6RSレコードを操作するための手続き型APIをエクスポートします。

スキーム手順: **make-record-type-descriptor** name parent uid sealed? opaque? fields

指定された特性を持つ新しいレコード型記述子を返します。name は、新しいレコード型の名前を示すシンボルである必要があります。parent は、返されるレコード型が拡張する `#f` または非シール レコード型記述子である必要があります。uid は、レコード型が生成型であることを示す `#f` または型の非生成型 uid を示すシンボルである必要があります。sealed? と opaque? は、レコード型のシール性と不透明性を指定するブール値である必要があります。fields は、`(可変名)` または `(不変名)` の形式の 0 個以上のフィールド指定子のベクトルである必要があります。ここで、name はフィールドの名前を示すシンボルです。

uidが`#f`でない場合、それはシンボルでなければなりません。

Scheme Procedure: **record-type-descriptor?** obj

objがレコード型記述子の場合は`#t`を返し、それ以外の場合は`#f`を返します。

Scheme Procedure: **make-record-constructor-descriptor** rtd parent-constructor-descriptor protocol

レコード型記述子rtdで指定されたレコード型のコンストラクタを生成するために使用できる、新しいレコードコンストラクタ記述子を返します。その委譲およびバインディング動作は、プロトコル手順プロトコルで指定されます。

parent-constructor-descriptor は、rtd の親型に対応するレコード コンストラクタ ディスクリプタを指定します（存在する場合）。rtd が基本型を表す場合、parent-constructor-descriptor は `#f` でなければなりません。rtd が別の型の拡張である場合、parent-constructor-descriptor は `#f` で構いませんが、この場合 protocol も `#f` でなければなりません。

Scheme Procedure: **record-constructor** rcd

レコードコンストラクタ記述子rcdで定義されたプロトコルを呼び出すことにより、レコードコンストラクタプロシージャを返します。

Scheme Procedure: **record-predicate** rtd

レコード型記述子rtdのレコード述語プロシージャを返します。

Scheme Procedure: **record-accessor** rtd k

レコード型記述子rtdのk番目のフィールドのレコードフィールドアクセサプロシージャを返します。

Scheme Procedure: **record-mutator** rtd k

レコード型記述子rtdのk番目のフィールドに対するレコードフィールド変更手順を返します。このフィールドが変更不可の場合、`&assertion`条件が発生します。

* * *

次へ: [rnrs 例外](https://doc.guix.gnu.org/guile/latest/en/guile.html#rnrs-exceptions)、前: [rnrs レコード手続き](#76210-rnrs-レコードの手続き型)、上: [R6RS 標準ライブラリ](#762-r6rs-標準ライブラリ) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.6.2.11 rnrs レコードの検査

`(rnrs レコード検査 (6))` ライブラリは、R6RS レコードに関するメタデータにアクセスするために役立つ手順を提供します。

Scheme Procedure: **record?** obj

指定されたオブジェクトが非不透明なR6RSレコードの場合は`#t`を返し、それ以外の場合は`#f`を返します。

スキーム手順: **record-rtd** レコード

レコードのレコード型記述子を返します。レコードが不透明な場合は、`&assertion` が発生します。

スキーム手順: **record-type-name** rtd

レコードタイプ記述子rtdの名前を返します。

スキーム手順: **record-type-parent** rtd

レコード型記述子rtdの親を返します。親がない場合は`#f`を返します。

スキーム手順: **record-type-uid** rtd

レコードタイプ記述子rtdのuidを返します。uidがない場合は`#f`を返します。

スキーム手順: **record-type-generative?** rtd

レコード型記述子rtdが生成型の場合は`#t`を返し、そうでない場合は`#f`を返します。

スキーム手順: **record-type-sealed?** rtd

レコード型記述子rtdがシールされている場合は`#t`を返し、そうでない場合は`#f`を返します。

スキーム手順: **record-type-opaque?** rtd

レコード型記述子rtdが不透明な場合は`#t`を返し、そうでない場合は`#f`を返します。

スキーム手順: **record-type-field-names** rtd [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- record_002dtype_002dfield_002dnames)

レコード型記述子rtdによって定義されたフィールドの名前を示すシンボルのベクトルを返します（そのサブタイプやスーパータイプは含まれません）。

Scheme Procedure: **record-field-mutable?** rtd k

レコード型記述子rtdのインデックスkにあるフィールド（そのサブタイプやスーパータイプではない）が可変である場合、`#t`を返します。

* * *

次へ: [rnrs 条件](#76213-rnrs-条件)、前: [rnrs レコード検査](#76211-rnrs-レコードの検査)、上: [R6RS 標準ライブラリ](#762-r6rs-標準ライブラリ) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.6.2.12 rnrs 例外

`(rnrs exceptions (6))` ライブラリは、例外的な状況の通知と処理に関連する機能を提供します。この機能は、Guile のコア例外処理プリミティブを再エクスポートします。詳細については、[Exceptions](06_11_controlling_the_flow_of_program_execution.md#6118-例外) を参照してください。同様の R6RS 以前の機能については、[SRFI-34 - プログラムの例外処理](07_05_23_srfi34_exception_handling_for_programs.md#7523-srfi-34---プログラムの例外処理) を参照してください。Guile では、SRFI-34、SRFI-35、および R6RS の例外処理はすべて同じコア機能に基づいて構築されているため、相互運用可能です。

Scheme プロシージャ: **with-exception-handler** ハンドラ サンク

`with-exception-handler` の詳細については、[例外の発生と処理](06_11_controlling_the_flow_of_program_execution.md#61182-例外の発生と処理) を参照してください。

Scheme構文: **guard** (変数 clause1 clause2 ...) body

body で指定された式を評価します。まず、発生した例外を変数にバインドするアドホック例外ハンドラを作成し、次に、指定された句を `cond` 式の一部であるかのように評価します。最初に一致した句の値が `guard` 式の値になります ([単純な条件付き評価](06_11_controlling_the_flow_of_program_execution.md#6112-単純な条件評価) を参照)。句のテスト式のいずれも `#t` に評価されない場合は、例外が再発生し、`guard` 形式の評価前に有効だった例外ハンドラが使用されます。

例えば、表現

([guard](07_05_23_srfi34_exception_handling_for_programs.md#7523-srfi-34---プログラムの例外処理) (ex (([eq?](06_09_general_utility_functions.md#691-平等) ex 'foo) 'bar) (([eq?](06_09_general_utility_functions.md#691-平等) ex 'bar) 'baz))
([raise](07_02_08_signals.md#728-シグナル) 'bar))

`baz` と評価されます。

Scheme手順: **raise** obj

Guile コアの `(raise-exception obj)` と同等です。[例外の発生と処理](06_11_controlling_the_flow_of_program_execution.md#61182-例外の発生と処理) を参照してください。（残念ながら、`raise` は Guile コアで既に別の関数に割り当てられています。[シグナル](07_02_08_signals.md#728-シグナル) を参照してください。）

Scheme Procedure: **raise-continuable** obj

Guile のコア `(raise-exception obj #:continuable? #t)` と同等です。[例外の発生と処理](06_11_controlling_the_flow_of_program_execution.md#61182-例外の発生と処理) を参照してください。

* * *

次へ: [I/O 条件](#76214-入出力条件)、前: [rnrs 例外](https://doc.guix.gnu.org/guile/latest/en/guile.html#rnrs-exceptions)、上: [R6RS 標準ライブラリ](#762-r6rs-標準ライブラリ) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.6.2.13 rnrs 条件

`(rnrs condition (6))` ライブラリは、新しい条件型を構築するためのフォームとプロシージャ、およびさまざまな一般的な例外状況を表す事前定義済みの条件型のライブラリを提供します。条件は、`&condition` レコード型のサブタイプのレコードであり、シールでも不透明でもありません。[R6RS レコード](#7628-r6rs-レコード) を参照してください。

条件は、_単純条件_として単独で操作することも、他の条件と組み合わせて_複合条件_を形成することもできます。複合条件は「入れ子」になりません。既存の複合条件から新しい複合条件を作成すると、それらは構成要素である単純条件に「平坦化」されます。たとえば、`&message`条件と、`&assertion`条件と別の`&message`条件を含む複合条件から新しい条件を作成すると、2つの`&message`条件と1つの`&assertion`条件を含む複合条件が生成されます。

以下に説明するレコード型述語とフィールドアクセサは、単純条件と複合条件のどちらにも適用できます。複合条件の場合、述語は、複合条件に適切な型の構成要素となる単純条件が含まれている場合に「#t」を返します。フィールドアクセサは、適切な型であると判断された最初の構成要素となる単純条件から必要なフィールドを返します。

Guile の R6RS レイヤーは、R6RS 条件システムの基盤として、`(ice-9 exceptions)` モジュールのコア例外タイプを使用します。Guile では、「条件」や「条件タイプ」よりも「例外オブジェクト」や「例外タイプ」という用語を使用することを推奨していますが、これは単なる命名規則の違いです。Guile では、条件階層内のタイプにも異なる名前が付けられています。詳細については、[例外オブジェクト](06_11_controlling_the_flow_of_program_execution.md#61181-例外オブジェクト) を参照してください。

このライブラリは、SRFI-35 条件モジュールと非常によく似ています ([SRFI-35 - 条件](07_05_24_srfi35_conditions.md#7524-srfi-35---条件) を参照)。その他の細かな違いとして、`(rnrs conditions)` ライブラリは条件フィールドアクセサに関するセマンティクスが若干異なり、定義済みの条件型がより多く含まれています。2 つの API は互換性があり、一方の API の `condition?` 述語をもう一方の API で作成された条件オブジェクトに適用すると `#t` が返されます。条件型も同じです。

条件タイプ: **&condition**

Scheme Procedure: **condition?** obj

条件の基本レコード型。Guile コアでは `&exception` と呼ばれます。

スキーム手順: **条件** condition1 ...

Scheme Procedure: **simple-conditions** condition

`condition` プロシージャは、条件引数から新しい複合条件を作成し、上記のように指定された複合条件を構成要素となる単純条件に平坦化します。

`simple-conditions` は、複合条件 `condition` の構成要素である単純条件のリストを、構築時に指定された順序で返します。

Scheme Procedure: **condition-predicate** rtd

Scheme Procedure: **condition-accessor** rtd proc

これらのプロシージャは、指定された条件レコードタイプ rtd の条件述語プロシージャとアクセサプロシージャを返します。

Scheme構文: **define-condition-type** condition-type supertype constructor predicate field-spec ...

条件型スーパータイプを親とする、condition-type という名前の条件型に対する新しいレコード型定義に評価されます。引数をこの型とその親型のフィールドにバインドするデフォルトコンストラクタは、識別子コンストラクタにバインドされます。条件述語は、predicate にバインドされます。新しい型のフィールドは不変であり、field-specs によって指定されます。field-specs はそれぞれ次の形式である必要があります。

（フィールドアクセサー）

ここで、field はフィールド名を示し、accessor はこのフィールド用に作成されたアクセサープロシージャへのバインディング名を示します。

条件タイプ: **&message**

Scheme Procedure: **make-message-condition** message

Scheme Procedure: **message-condition?** obj

Scheme Procedure: **condition-message** condition

発生した状況を説明するメッセージを含む型。

条件タイプ: **&warning**

Scheme 手順: **make-warning**

Scheme Procedure: **warning?** obj

実行中に発生する非致命的な状態を表すための基本型。

状態の種類: **&serious**

スキーム手順: **make-serious-condition**

スキーム手順: **serious-condition?** obj

無視できないほど深刻なエラーを表す条件の基本型。Guile コアでは `&error` と呼ばれます。

条件タイプ: **&error**

Scheme手順: **make-error**

Scheme 手順: **エラー?** obj

エラーを表す条件の基本型。Guile コアでは `&external-error` と呼ばれます。

条件タイプ: **&violation**

スキーム手順: **make-violation**

スキーム手順: **違反？**

言語またはライブラリの標準違反を表すために使用できる、`&serious` のサブタイプ。Guile コアでは `&programming-error` として知られています。

条件タイプ: **&assertion**

Scheme Procedure: **make-assertion-violation**

Scheme Procedure: **assertion-violation?** obj

プロシージャへの無効な呼び出しを示す`&violation`のサブタイプ。Guileコアでは`&assertion-failure`として知られています。

症状の種類: **&刺激物**

スキーム手順: **make-irritants-condition** 刺激物

スキーム手順: **irritants-condition?** obj

スキーム手順: **条件-刺激物** 条件

複合条件における別の条件の原因に関する情報を格納するために使用される基本型。

条件タイプ: **&who**

スキーム手順: **make-who-condition** who

スキーム手順: **who-condition?** obj

スキーム手順: **condition-who** 条件

複合条件において、別の条件の原因となるエンティティの識別情報（文字列または記号）を格納するために使用される基本型。

条件タイプ: **&non-continuable**

スキーム手順: **make-non-continuable-violation**

スキーム手順: **継続不可能な違反?** obj

`raise` によって呼び出された例外ハンドラがローカルで戻り値を返したことを示すために使用される `&violation` のサブタイプ。

条件タイプ: **&implementation-restriction**

Scheme Procedure: **make-implementation-restriction-violation**

スキーム手順: **実装制限違反?** obj

実装上の制約違反を示すために使用される`&violation`のサブタイプ。

条件タイプ: **&lexical**

Scheme手順: **make-lexical-violation**

Scheme 手順: **lexical-violation?** obj

`&violation` のサブタイプで、データム構文レベルでの構文違反を示すために使用されます。

条件タイプ: **&syntax**

Scheme Procedure: **make-syntax-violation** form subform

Scheme 手順: **構文違反?** obj

Scheme 手順: **syntax-violation-form** 条件

Scheme Procedure: **syntax-violation-subform** 条件

構文違反を示す`&violation`のサブタイプ。formフィールドとsubformフィールドはデータ値である必要があり、条件の原因となっている構文形式を示します。

条件タイプ: **&undefined**

Scheme Procedure: **make-undefined-violation**

Scheme Procedure: **undefined-violation?** obj

`&violation` のサブタイプで、未定義の識別子への参照を示します。Guileコアでは `&undefined-variable` と呼ばれます。

* * *

次へ: [トランスコーダー](#76215-トランスコーダ)、前: [rnrs条件](#76213-rnrs-条件)、上: [R6RS標準ライブラリ](#762-r6rs-標準ライブラリ) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.6.2.14 入出力条件

これらの条件タイプは、`(rnrs io ports (6))` と `(rnrs io simple (6))` ライブラリの両方によってエクスポートされます。

条件タイプ: **&i/o**

Scheme手順: **make-i/o-error**

Scheme手順: **i/oエラー?** obj

より具体的な入出力エラーを表す条件の上位概念。

条件タイプ: **&i/o-read**

Scheme手順: **make-i/o-read-error**

Scheme手順: **i/o-read-error?** obj

`&i/o`; のサブタイプは、読み取り関連の入出力エラーを表します。

条件タイプ: **&i/o-write**

Scheme手順: **make-i/o-write-error**

Scheme手順: **i/o書き込みエラー?** obj

`&i/o`; のサブタイプは、書き込み関連の入出力エラーを表します。

条件タイプ: **&i/o-invalid-position**

Scheme Procedure: **make-i/o-invalid-position-error** position

Scheme Procedure: **i/o-invalid-position-error?** obj

Scheme Procedure: **i/o-error-position** 条件 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- i_002fo_002derror_002dposition)

`&i/o`; のサブタイプは、ファイル位置を無効な位置に設定しようとしたことに関連するエラーを表します。

条件タイプ: **&i/o-filename**

Scheme Procedure: **make-io-filename-error** filename

Scheme手順: **i/o-filename-error?** obj

Scheme Procedure: **i/o-error-filename** 条件

`&i/o`; のサブタイプは、指定されたファイルに対する操作に関連するエラーを表します。

条件タイプ: **&i/o-file-protection**

Scheme Procedure: **make-i/o-file-protection-error** filename

Scheme Procedure: **i/o-file-protection-error?** obj

呼び出し元が十分な権限を持っていなかった名前付きファイルへのアクセス試行によって発生したエラーを表します。

条件タイプ: **&i/o-file-is-read-only**

Scheme Procedure: **make-i/o-file-is-read-only-error** filename

Scheme Procedure: **i/o-file-is-read-only-error?** obj

`&i/o-file-protection` のサブタイプは、読み取り専用ファイルへの書き込み試行に関連するエラーを表します。

条件タイプ: **&i/o-file-already-exists**

Scheme Procedure: **make-i/o-file-already-exists-error** filename

Scheme 手順: **i/o-file-already-exists-error?** obj

`&i/o-filename` のサブタイプは、存在しないと想定されていた既存のファイルに対する操作に関連するエラーを表します。

条件タイプ: **&i/o-file-does-not-exist**

Scheme 手順: **make-i/o-file-does-not-exist-error**

Scheme 手順: **i/o-file-does-not-exist-error?** obj

`&i/o-filename` のサブタイプは、存在すると想定された存在しないファイルに対する操作に関連するエラーを表します。

条件タイプ: **&i/o-port**

Scheme Procedure: **make-i/o-port-error** port

Scheme Procedure: **i/o-port-error?** obj

Scheme Procedure: **i/o-error-port** 条件

`&i/o`; のサブタイプは、ポート port に対する操作に関連するエラーを表します。

* * *

次へ: [rnrs io ポート](#76216-rnrs-io-ポート)、前: [I/O 条件](#76214-入出力条件)、上: [R6RS 標準ライブラリ](#762-r6rs-標準ライブラリ) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.6.2.15 トランスコーダ

トランスコーダー機能は`(rnrs io ports)`によってエクスポートされます。

Unicodeエンコーディング方式には、文字や文字列をバイトシーケンスとしてエンコードし、それらのシーケンスをデコードする標準的な方法がいくつか存在します。このドキュメントでは、_codec_はUnicodeまたは類似のエンコーディング方式を表す不変のSchemeオブジェクトです。

_行末スタイル_とは、`none`でない場合、テキストポートが行末の表現をどのように変換するかを記述する記号です。

_トランスコーダー_は、コーデックと行末スタイル、およびデコードエラーを処理するメソッドを組み合わせた、不変のSchemeオブジェクトです。各トランスコーダーは、バイトシーケンスとUnicode文字および文字列間の、双方向（必ずしもロスレスではない）、場合によってはステートフルな特定の変換を表します。すべてのトランスコーダーは、入力方向（バイトから文字）または出力方向（文字からバイト）で動作できます。トランスコーダーのパラメータ名は、対応する引数がトランスコーダーでなければならないことを意味します。

バイナリポートとは、バイナリ入出力をサポートし、関連付けられたトランスコーダを持たず、テキスト入出力もサポートしないポートです。テキストポートとは、テキスト入出力をサポートし、バイナリ入出力をサポートしないポートです。テキストポートには、関連付けられたトランスコーダがある場合とない場合があります。

スキーム手順: **latin-1-codec**

スキーム手順: **utf-8-codec**

スキーム手順: **utf-16-codec**

これらは、ISO 8859-1、UTF-8、およびUTF-16エンコーディング方式用に事前に定義されたコーデックです。

これらのプロシージャのいずれかを呼び出すと、`eqv?` の意味において、同じプロシージャへの他の呼び出しの結果と等しい値が返されます。

Scheme構文: **eol-style** eol-style-symbol

eol-style-symbol は、名前が `lf`、`cr`、`crlf`、`nel`、`crnel`、`ls`、および `none` のいずれかであるシンボルである必要があります。

この形式は対応するシンボルに評価されます。eol-style-symbol の名前がこれらのシンボルのいずれでもない場合、効果と結果は実装に依存します。特に、結果は `make-transcoder` の eol スタイルの引数として受け入れられる eol スタイルのシンボルになる可能性があります。それ以外の場合は、例外が発生します。

`none`を除くすべてのeolスタイルの記号は、特定の行末エンコーディングを表します。

`lf`

改行

`cr`

改行

`crlf`

キャリッジリターン、ラインフィード

`ネル`

次の行

`crnel`

改行、次の行

`ls`

行区切り線

トランスコーダを備えたテキストポートで、そのトランスコーダのeolスタイルシンボルが「none」の場合、変換は行われません。テキスト入力ポートの場合、「none」以外のeolスタイルシンボルは、上記のすべての行末エンコーディングが認識され、単一のラインフィードに変換されることを意味します。テキスト出力ポートの場合、「none」と「lf」は同等です。ラインフィード文字は指定されたeolスタイルシンボルに従ってエンコードされ、可能な行末に関係するその他の文字はそのままエンコードされます。

> **注:** eol-style-symbol の名前のみが重要です。

スキーム手順: **native-eol-style**

基となるプラットフォームのデフォルトの行末スタイルを返します。たとえば、Unix では `lf`、Windows では `crlf` です。

条件タイプ: **&i/o-decoding**

Scheme Procedure: **make-i/o-decoding-error** port

Scheme Procedure: **i/o-decoding-error?** obj

この条件タイプは以下のように定義できます。

(define-condition-type [&i/o- decoding](#76215-トランスコーダ) [&i/o-port](#76214-入出力条件)
[make-i/o-decoding-error](#76215-トランスコーダ) [i/o-decoding-error?](#76215-トランスコーダ))

このタイプの例外は、ポートからのテキスト入力に対する操作のいずれかが、ポートのトランスコーダの入力方向によって文字または文字列に変換できないバイト列に遭遇した場合に発生します。

このような例外が発生した場合、ポートの位置が無効なエンコーディングを超えています。

条件タイプ: **&i/o-encoding**

Scheme 手順: **make-i/o-encoding-error** ポート 文字

Scheme Procedure: **i/o-encoding-error?** obj

Scheme手順: **i/o-encoding-error-char**条件

この条件タイプは以下のように定義できます。

(define-condition-type [&i/o-encoding](#76215-トランスコーダ) [&i/o-port](#76214-入出力条件)
[make-i/o-encoding-error](#76215-トランスコーダ) [i/o-encoding-error?](#76215-トランスコーダ)
(char [i/o-encoding-error-char](#76215-トランスコーダ)))

このタイプの例外は、ポートへのテキスト出力操作のいずれかで、ポートのトランスコーダの出力方向によってバイトに変換できない文字に遭遇した場合に発生します。char はエンコードできなかった文字です。

Scheme構文: **error-handling-mode** error-handling-mode-symbol

error-handling-mode-symbol は、`ignore`、`raise`、`replace` のいずれかの名前を持つシンボルである必要があります。この形式は、対応するシンボルに評価されます。error-handling-mode-symbol がこれらの識別子のいずれでもない場合、効果と結果は実装に依存します。結果は、`make-transcoder` の handling-mode 引数として受け入れられる error-handling-mode シンボルになる場合があります。`make-transcoder` の handling-mode 引数として受け入れられない場合は、例外が発生します。

> **注:** エラー処理モードシンボルの名前のみが重要です。

トランスコーダのエラー処理モードは、エンコードまたはデコードエラーが発生した場合のテキスト入出力操作の動作を規定します。

テキスト入力操作で無効または不完全な文字エンコーディングが検出され、エラー処理モードが「無視」の場合、無効なエンコーディングの適切なバイト数が無視され、デコードは次のバイトから続行されます。

エラー処理モードが「replace」の場合、置換文字U+FFFDがデータストリームに挿入され、適切なバイト数が無視され、後続のバイトからデコードが続行されます。

エラー処理モードが`raise`の場合、条件タイプ`&i/o-decoding`の例外が発生します。

テキスト出力操作でエンコードできない文字に遭遇し、エラー処理モードが `ignore` の場合、その文字は無視され、エンコードは次の文字で続行されます。エラー処理モードが `replace` の場合、トランスコーダはコーデック固有の置換文字を出力し、エンコードは次の文字で続行されます。コーデックが Unicode エンコーディングのいずれかであるトランスコーダの場合、置換文字は U+FFFD ですが、Latin-1 エンコーディングの場合は `?` 文字です。エラー処理モードが `raise` の場合、条件タイプ `&i/o-encoding` の例外が発生します。

スキーム手順: **make-transcoder** codec

Scheme Procedure: **make-transcoder** codec eol-style

Scheme Procedure: **make-transcoder** codec eol-style handling-mode

codec はコーデックである必要があります。eol-style が存在する場合は、eol スタイルのシンボルである必要があります。handling-mode が存在する場合は、エラー処理モードのシンボルである必要があります。

eol-style は省略可能で、その場合は基盤となるプラットフォームのネイティブな改行スタイルがデフォルトとなります。handling-mode も省略可能で、その場合は `replace` がデフォルトとなります。結果として、引数で指定された動作を持つトランスコーダーが生成されます。

スキーム手順: **native-transcoder**

実装に依存するトランスコーダーを返します。このトランスコーダーは、場合によってはロケールに依存する「ネイティブ」トランスコーディングを表します。

スキーム手順: **transcoder-codec** トランスコーダー

スキーム手順: **transcoder-eol-style** トランスコーダー

スキーム手順: **トランスコーダーエラー処理モード** トランスコーダー

これらはトランスコーダーオブジェクトへのアクセサーです。`make-transcoder`によって返されたトランスコーダーに適用すると、それぞれコーデック、eol-style、およびhandling-modeの引数を返します。

Scheme手順: **bytevector->string** バイトベクタートランスコーダー

バイトベクトルをトランスコーダーの入力方向に従ってトランスコードした結果の文字列を返します。

Scheme手順: **string->bytevector** 文字列トランスコーダー

トランスコーダの出力方向に従って文字列をトランスコードした結果のバイトベクトルを返します。

* * *

次へ: [R6RS ファイルポート](#76217-r6rs-ファイルポート)、前: [トランスコーダー](#76215-トランスコーダ)、上: [R6RS 標準ライブラリ](#762-r6rs-標準ライブラリ) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.6.2.16 rnrs io ポート

Guile のバイナリおよびテキスト ポート インターフェイスは R6RS に大きく影響を受けているため、多くの R6RS ポート インターフェイスは他の場所で文書化されています。R6RS ポートは Guile のネイティブ ポートと分離されていないわけではないため、Guile 固有の手順は R6RS API を使用して作成されたポートで機能し、その逆も同様です。また、Guile ではすべてのポートがテキストとバイナリの両方であることにも注意してください。Guile のコア ポート API の詳細については、[入力と出力](06_12_input_and_output.md#612-入力と出力) を参照してください。R6RS ポート モジュールは、Guile の I/O ルーチンをヘルパーでラップし、ネイティブ Guile 例外を R6RS 条件に変換します。詳細については、[I/O 条件](#76214-入出力条件) を参照してください。 R6RS ファイルポート インターフェースに関するドキュメントについては、[R6RS ファイルポート](#76217-r6rs-ファイルポート) を参照してください。

_注_: このR6RS APIの実装はまだ完了していません。

Scheme 手順: **eof-object?** obj

ドキュメントについては、[バイナリ入出力](06_12_input_and_output.md#6122-バイナリ入出力)を参照してください。

Scheme Procedure: **eof-object**

ファイル終端（EOF）オブジェクトを返します。

([eof-object?](06_12_input_and_output.md#6122-バイナリ入出力) ([eof-object](#76216-rnrs-io-ポート)))
⇒ #t

Scheme Procedure: **port?** obj

Scheme Procedure: **input-port?** obj

Scheme Procedure: **output-port?** obj

Scheme Procedure: **call-with-port** port proc

ドキュメントについては、[Ports](06_12_input_and_output.md#6121-ポート)を参照してください。

スキーム手順: **port-transcoder** ポート

ポートのエンコーディングに関連付けられたトランスコーダーを返します。[エンコーディング](06_12_input_and_output.md#6123-エンコーディング)および[トランスコーダー](#76215-トランスコーダ)を参照してください。

スキーム手順: **binary-port?** port

ポートがバイナリ ポートであると思われる場合は `#t` を返し、そうでない場合は `#f` を返します。Guile は現在、バイナリ ポートとテキスト ポートを区別しないため、この述語はポートがバイナリ ポートとして作成されたかどうかの信頼できる指標ではありません。Guile はバイナリ ポートを作成する際にこのエンコーディングを使用するため、ポート エンコーディングが「ISO-8859-1」の場合に限り `#t` を返します。詳細については、[エンコーディング](06_12_input_and_output.md#6123-エンコーディング) を参照してください。

Scheme手順: **textual-port?** port

ポートがテキストポートであると思われる場合は `#t` を返し、そうでない場合は `#f` を返します。Guile は現在バイナリポートとテキストポートを区別しないため、この述語はポートがテキストポートとして作成されたかどうかの信頼できる指標ではありません。現在、Guile ではすべてのポートをテキスト入出力に使用できるため、常に `#t` を返します。詳細については、[Encoding](06_12_input_and_output.md#6123-エンコーディング) を参照してください。

スキーム手順: **transcoded-port** バイナリポートトランスコーダー

`transcoded-port` プロシージャは、指定されたトランスコーダを持つ新しいテキストポートを返します。それ以外の場合、新しいテキストポートの状態は、バイナリポートの状態とほぼ同じです。バイナリポートが入力ポートの場合、新しいテキストポートも入力ポートとなり、バイナリポートからまだ読み取られていないバイトをトランスコードします。バイナリポートが出力ポートの場合、新しいテキストポートも出力ポートとなり、出力文字をバイナリポートで表されるバイトシンクに書き込まれるバイトにトランスコードします。

しかし、副作用として、`transcoded-port` は、バイナリポート自体が閉じられてこの章で説明する入出力操作で使用できなくなっても、新しいテキストポートがバイナリポートによって表されるバイトソースまたはシンクを引き続き使用できるように、バイナリポートを特別な方法で閉じます。

スキーム手順: **port-position** ポート

`(seek port 0 SEEK_CUR)` と同等です。[ランダムアクセス](06_12_input_and_output.md#6127-ランダムアクセス) を参照してください。

スキーム手順: **port-has-port-position?** port

ポートが`port-position`をサポートしている場合は、`#t`を返します。

スキーム手順: **set-port-position!** ポートオフセット

`(seek port offset SEEK_SET)` と同等です。[ランダムアクセス](06_12_input_and_output.md#6127-ランダムアクセス) を参照してください。

スキーム手順: **port-has-set-port-position!?** port

`#t` は `set-port-position!` をサポートするポートです。

スキーム手順: **port-eof?** input-port

`(eof-object? (lookahead-u8 input-port))` と同等です。

スキーム手順: **標準入力ポート**

スキーム手順: **標準出力ポート**

Scheme Procedure: **standard-error-port**

標準入力に接続された新しいバイナリ入力ポート、または標準出力もしくは標準エラーに接続されたバイナリ出力ポートを返します。ポートが`port-position`および`set-port-position!`操作をサポートするかどうかは、実装に依存します。

スキーム手順: **current-input-port**

スキーム手順: **current-output-port**

スキーム手順: **current-error-port**

[入力、出力、エラーのデフォルトポート](06_12_input_and_output.md#6129-入力出力およびエラーのデフォルトポート)を参照してください。

スキーム手順: **open-bytevector-input-port** bv \[transcoder\]

スキーム手順: **open-bytevector-output-port** \[トランスコーダー\]

[バイトベクトルポート](06_12_input_and_output.md#612102-bytevector-ポート)を参照してください。

スキーム手順: **make-custom-binary-input-port** id read! get-position set-position! close

スキーム手順: **make-custom-binary-output-port** id write! get-position set-position! close

スキーム手順: **make-custom-binary-input/output-port** id read! write! get-position set-position! close

[カスタムポート](06_12_input_and_output.md#612104-カスタムポート)を参照してください。

Scheme Procedure: **make-custom-textual-input-port** id read! get-position set-position! close

Scheme Procedure: **make-custom-textual-output-port** id write! get-position set-position! close

Scheme Procedure: **make-custom-textual-input/output-port** id read! write! get-position set-position! close

[カスタムポート](06_12_input_and_output.md#612104-カスタムポート)を参照してください。

スキーム手順: **get-u8** ポート

スキーム手順: **lookahead-u8** ポート

スキーム手順: **get-bytevector-n** ポート数

スキームプロシージャ: **get-bytevector-n!** ポート bv 開始カウント

スキーム手順: **get-bytevector-some** ポート

スキーム手順: **get-bytevector-all** ポート

スキーム手順: **put-u8** ポートバイト

スキーム手順: **put-bytevector** port bv \[start \[count\]\]

[バイナリ入出力](06_12_input_and_output.md#6122-バイナリ入出力)を参照してください。

Scheme手順: **get-char** textual-input-port

Scheme 手順: **lookahead-char** textual-input-port

Scheme Procedure: **get-string-n** textual-input-port count

Scheme 手順: **get-string-n!** textual-input-port string start count

Scheme 手順: **get-string-all** textual-input-port

Scheme手順: **get-line** textual-input-port

Scheme手順: **put-char** port char

Scheme Procedure: **put-string** port string \[start \[count\]\]

[テキスト入出力](06_12_input_and_output.md#6124-テキスト入出力)を参照してください。

Scheme Procedure: **get-datum** textual-input-port count

テキスト入力ポートから外部表現を読み込み、それが表すデータを返します。`get-datum`プロシージャは、指定されたテキスト入力ポートから解析可能な次のデータを返します。このとき、テキスト入力ポートはオブジェクトの外部表現の末尾の直後を指すように更新されます。

入力中の_インターレクセムスペース_（コメントまたは空白文字。詳細は[Scheme Syntax: Standard and Guile Extensions](06_16_reading_and_evaluating_scheme_code.md#6161-scheme構文-標準とguile拡張機能)を参照）は、まずスキップされます。インターレクセムスペースの後にファイルの終端がある場合は、ファイルの終端オブジェクトが返されます。

入力データに外部表現と矛盾する文字が検出された場合、条件タイプが `&lexical` および `&i/o-read` の例外が発生します。また、外部表現の開始後にファイルの終端が検出されたものの、外部表現が不完全で解析できない場合も、条件タイプが `&lexical` および `&i/o-read` の例外が発生します。

Scheme Procedure: **put-datum** textual-output-port datum

datum は datum 値である必要があります。`put-datum` プロシージャは、datum の外部表現を textual-output-port に書き込みます。具体的な外部表現は実装に依存します。ただし、可能な限り、実装は `get-datum` がその表現を読み取った際に、datum と等しい (`equal?` の意味で) オブジェクトを返すような表現を生成する必要があります。

> **注:** すべてのデータムが、`get-datum` が元のデータムと等しいオブジェクトを生成するような外部表現を生成できるとは限りません。特に、データムに NaN が含まれている場合、これが不可能になることがあります。

> **注:** `put-datum` プロシージャは外部表現を書き込むだけで、末尾の区切り文字は書き込みません。`put-datum` を使用して複数の外部表現を連続して出力ポートに書き込む場合は、後続の `get-datum` 呼び出しで読み込めるように、適切に区切り文字を設定する必要があります。

スキーム手順: **flush-output-port** ポート

`force-output` のドキュメントについては、[Buffering](06_12_input_and_output.md#6126-バッファリング) を参照してください。

* * *

次へ: [rnrs io simple](#76218-rnrs-io-simple)、前: [rnrs io ports](#76216-rnrs-io-ポート)、上: [R6RS Standard Libraries](#762-r6rs-標準ライブラリ) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.6.2.17 R6RS ファイルポート

このセクションで説明する機能は、`(rnrs io ports)` モジュールによってエクスポートされます。

Scheme構文: **buffer-mode** buffer-mode-symbol

buffer-mode-symbol は、名前が `none`、`line`、`block` のいずれかであるシンボルでなければなりません。結果は対応するシンボルであり、関連付けられたバッファモードを指定します。これらの異なるバッファモードについては、[バッファリング](06_12_input_and_output.md#6126-バッファリング) を参照してください。バッファリング量を制御するには、代わりに `setvbuf` を使用してください。buffer-mode-symbol の名前のみが重要であることに注意してください。

ポートバッファリングについては、[バッファリング](06_12_input_and_output.md#6126-バッファリング)を参照してください。

スキームプロシージャ: **buffer-mode?** obj

引数が有効なバッファモードシンボルであれば`#t`を返し、そうでなければ`#f`を返します。

ファイルを開く際、各種プロシージャは、ファイルの開き方を指定するフラグをカプセル化した `file-options` オブジェクトを受け取ります。`file-options` オブジェクトは、有効なファイルオプションを構成するシンボルの列挙型セットです ([rnrs 列挙型](#76226-rnrs-列挙型) を参照)。

ファイルオプションのパラメータ名とは、対応する引数がファイルオプションオブジェクトでなければならないことを意味します。

Scheme構文: **file-options** file-options-symbol ...

各ファイルオプションシンボルはシンボルでなければなりません。

`file-options`構文は、指定されたオプションをカプセル化したfile-optionsオブジェクトを返します。

出力用にファイルを開く操作に渡された場合、`(file-options)` によって返される file-options オブジェクトは、ファイルが存在しない場合は作成し、存在する場合は条件タイプ `&i/o-file-already-exists` の例外を発生させることを指定します。デフォルトの動作を変更するには、以下の標準オプションを含めることができます。

`no-create`

ファイルがまだ存在しない場合は、ファイルは作成されず、代わりに条件タイプ `&i/o-file-does-not-exist` の例外が発生します。ファイルが既に存在する場合は、条件タイプ `&i/o-file-already-exists` の例外は発生せず、ファイルはゼロ長に切り詰められます。

失敗しない

ファイルが既に存在する場合、`no-create` が含まれていない場合でも、条件タイプ `&i/o-file-already-exists` の例外は発生せず、ファイルはゼロ長に切り詰められます。

`no-truncate`

ファイルが既に存在し、条件タイプ `&i/o-file-already-exists` の例外が `no-create` または `no-fail` のインクルードによって抑制されている場合、ファイルは切り詰められませんが、ポートの現在の位置はファイルの先頭に設定されます。

これらのオプションは、ファイルが入力専用に開かれている場合は効果がありません。上記以外のシンボルもファイルオプションシンボルとして使用できます。それらのシンボルには、実装固有の意味があります（もしあれば）。

> **注:** ファイルオプションシンボルの名前のみが重要です。

Scheme Procedure: **open-file-input-port** filename

Scheme Procedure: **open-file-input-port** filename file-options

Scheme Procedure: **open-file-input-port** filename file-options buffer-mode

Scheme Procedure: **open-file-input-port** filename file-options buffer-mode maybe-transcoder

maybe-transcoder はトランスコーダーか `#f` のいずれかでなければなりません。

`open-file-input-port`プロシージャは、指定されたファイルの入力ポートを返します。file-options引数とmaybe-transcoder引数は省略可能です。

file-options引数は、返されるポートのさまざまな側面を決定する可能性があり、デフォルト値は`(file-options)`です。

buffer-mode引数を指定する場合は、バッファモードを指定するシンボルのいずれかを指定する必要があります。buffer-mode引数のデフォルト値は`block`です。

maybe-transcoderがトランスコーダーである場合、返されたポートに関連付けられるトランスコーダーになります。

maybe-transcoder が `#f` または指定されていない場合、ポートはバイナリ ポートとなり、`port-position` および `set-port-position!` 操作をサポートします。それ以外の場合は、ポートはテキスト ポートとなり、`port-position` および `set-port-position!` 操作をサポートするかどうかは実装依存（場合によってはトランスコーダー依存）となります。

Scheme Procedure: **open-file-output-port** filename

Scheme Procedure: **open-file-output-port** filename file-options

Scheme Procedure: **open-file-output-port** filename file-options buffer-mode

Scheme Procedure: **open-file-output-port** filename file-options buffer-mode maybe-transcoder

maybe-transcoder はトランスコーダーか `#f` のいずれかでなければなりません。

`open-file-output-port` プロシージャは、指定されたファイルの出力ポートを返します。

file-options引数は、返されるポートのさまざまな側面を決定する可能性があり、デフォルト値は`(file-options)`です。

buffer-mode引数を指定する場合は、バッファモードを指定するシンボルのいずれかを指定する必要があります。buffer-mode引数のデフォルト値は`block`です。

maybe-transcoderがトランスコーダーである場合、それはポートに関連付けられたトランスコーダーになります。

maybe-transcoder が `#f` または指定されていない場合、ポートはバイナリ ポートとなり、 `port-position` および `set-port-position!` 操作をサポートします。それ以外の場合は、ポートはテキスト ポートとなり、`port-position` および `set-port-position!` 操作をサポートするかどうかは実装依存（場合によってはトランスコーダー依存）となります。

* * *

次へ: [rnrs ファイル](#76219-rnrs-ファイル)、前: [R6RS ファイル ポート](#76217-r6rs-ファイルポート)、上: [R6RS 標準ライブラリ](#762-r6rs-標準ライブラリ) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.6.2.18 rnrs io simple

`(rnrs io simple (6))` ライブラリは、ポートに対してテキスト入出力を実行するための便利な関数を提供します。このライブラリは、([I/O 条件](#76214-入出力条件) を参照) で説明されているすべての条件タイプと関連する手順もエクスポートします。このセクションのコンテキストでは、手順が Guile のコア ライブラリの対応する手順と「同一」に動作すると述べる場合、これは条件に関する動作を除いてのことです。このような手順は、エラーが発生した場合に適切な R6RS 条件を発生させますが、それ以外の場合は同一に動作します。

> **注:** 条件の正確性に関して既知の問題がまだあります。一部のエラーは、適切な R6RS 条件ではなく、ネイティブの Guile 例外としてスローされる場合があります。

Scheme Procedure: **eof-object**

Scheme 手順: **eof-object?** obj

これらの手順は、`(rnrs io ports (6))`ライブラリによって提供される手順と同一です。詳細については、[rnrs io ports](#76216-rnrs-io-ポート)を参照してください。

Scheme Procedure: **input-port?** obj

Scheme Procedure: **output-port?** obj

これらの手順は、Guile のコアライブラリが提供する手順と同一です。詳細については、[Ports](06_12_input_and_output.md#6121-ポート) を参照してください。

Scheme Procedure: **call-with-input-file** filename proc

Scheme Procedure: **call-with-output-file** filename proc

Scheme Procedure: **open-input-file** filename

Scheme手順: **open-output-file** filename

Scheme プロシージャ: **with-input-from-file** filename thunk

Scheme プロシージャ: **with-output-to-file** filename thunk

これらの手順は、Guile のコアライブラリが提供する手順と同一です。詳細については、[ファイルポート](06_12_input_and_output.md#612101-ファイルポート) を参照してください。

スキーム手順: **close-input-port** input-port

Scheme Procedure: **close-output-port** output-port

指定された入力ポートまたは出力ポートを閉じます。これらは旧式のインターフェースです。代わりに`close-port`を使用してください。

Scheme手順: **peek-char**

Scheme Procedure: **peek-char** textual-input-port

Scheme手順: **read-char**

Scheme手順: **read-char** textual-input-port

これらの手順は、Guile のコアライブラリが提供する手順と同一です。詳細については、[Venerable Port Interfaces](06_12_input_and_output.md#61211-ヴェネラブルポートインターフェース) を参照してください。

Scheme手順: **read**

Scheme手順: **read** テキスト入力ポート

この手順は、Guile のコアライブラリが提供する手順と同一です。詳細については、[スキームコードの読み込み](06_16_reading_and_evaluating_scheme_code.md#6162-リーディングスキームコード) を参照してください。

Scheme手順: **display** obj

Scheme 手順: **display** obj textual-output-port

Scheme Procedure: **newline**

Scheme Procedure: **newline** textual-output-port

Scheme手順: **write** obj

Scheme手順: **write** obj textual-output-port

Scheme手順: **write-char** char

Scheme手順: **write-char** char textual-output-port

これらの手順は、Guile のコアライブラリが提供する手順と同一です。詳細については、[Venerable Port Interfaces](06_12_input_and_output.md#61211-ヴェネラブルポートインターフェース) および [Writing Scheme Values](06_16_reading_and_evaluating_scheme_code.md#6164-scheme値の記述) を参照してください。

* * *

次へ: [rnrs プログラム](#76220-rnrs-プログラム)、前: [rnrs io simple](#76218-rnrs-io-simple)、上: [R6RS 標準ライブラリ](#762-r6rs-標準ライブラリ) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.6.2.19 rnrs ファイル

`(rnrs files (6))`ライブラリは、ファイルの存在をテストする`file-exists?`と、ファイルシステムからファイルを削除することを可能にする`delete-file`プロシージャを提供します。

これらの手順は、Guile のコアライブラリが提供する手順と同一です。詳細については、[ファイルシステム](07_02_03_file_system.md#723-ファイルシステム) を参照してください。

* * *

次へ: [rnrs 算術固定番号](#76221-rnrs-算術固定番号)、前: [rnrs ファイル](#76219-rnrs-ファイル)、上: [R6RS 標準ライブラリ](#762-r6rs-標準ライブラリ) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.6.2.20 rnrs プログラム

`(rnrs programs (6))`ライブラリは、プロセス管理と内省のための手順を提供します。

Scheme手順: **コマンドライン**

この手順は、Guile のコアライブラリが提供する手順と同一です。詳細については、[ランタイム環境](07_02_06_runtime_environment.md#726-ランタイム環境) を参照してください。

スキーム手順: **exit** \[status\]

この手順は、Guile のコアライブラリが提供する手順と同一です。詳細については、[プロセス](07_02_07_processes.md#727-プロセス) を参照してください。

* * *

次へ: [rnrs 算術 flonums](#76222-rnrs-算術-flonums)、前: [rnrs プログラム](#76220-rnrs-プログラム)、上: [R6RS 標準ライブラリ](#762-r6rs-標準ライブラリ) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.6.2.21 rnrs 算術固定番号

`(rnrs arithmetic fixnums (6))` ライブラリは、実装依存の正確な整数値の範囲に対して算術演算を実行するための手順を提供します。R6RS では、これを _fixnums_ と呼びます。Guile では、fixnum のサイズは `SCM` 型のサイズによって決まります。単一の SCM 構造体は、fixnum 全体を格納できることが保証されているため、fixnum の計算は特に効率的です ([The SCM Type](06_03_the_scm_type.md#63-scmタイプ) を参照)。32 ビット システムでは、fixnum の最も負の値と最も正の値は、それぞれ -536870912 と 536870911 です。

特に指定がない限り、以下のすべての手順は引数として固定番号を受け取ります。固定番号以外の引数が渡された場合は`&assertion`条件が発生し、結果が固定番号でない場合は`&implementation-restriction`条件が発生します。

スキームプロシージャ: **fixnum?** obj

objが固定数値の場合は`#t`を返し、それ以外の場合は`#f`を返します。

スキーム手順: **fixnum-width**

スキーム手順: **least-fixnum**

スキーム手順: **greatest-fixnum**

これらの手順は、それぞれ、Guile で固定数値を表すために必要な最大ビット数、最小固定数値、および最大固定数値を返します。

スキーム手順: **fx=?** fx1 fx2 fx3 ...

スキーム手順: **fx>?** fx1 fx2 fx3 ...

スキーム手順: **fx<?** fx1 fx2 fx3 ...

スキーム手順: **fx>=?** fx1 fx2 fx3 ...

スキーム手順: **fx<=?** fx1 fx2 fx3 ...

これらの手続きは、fixnum引数が（それぞれ）等しい、単調増加、単調減少、単調非減少、または単調非増加である場合に`#t`を返し、それ以外の場合は`#f`を返します。

Scheme Procedure: **fxzero?** fx

スキーム手順: **fxpositive?** fx

スキーム手順: **fxnegative?** fx

スキーム手順: **fxodd?** fx

スキーム手順: **fxeven?** fx

これらの数値述語は、fxがそれぞれ0、0より大きい、0より小さい、奇数、偶数の場合は`#t`を返し、それ以外の場合は`#f`を返します。

スキーム手順: **fxmax** fx1 fx2 ...

スキーム手順: **fxmin** fx1 fx2 ...

これらの手続きは、引数の最大値または最小値を返します。

スキーム手順: **fx+** fx1 fx2

スキーム手順: **fx\*** fx1 fx2

これらの手続きは、引数の合計または積を返します。

スキーム手順: **fx-** fx1 fx2

スキーム手順: **fx-** fx

単一の引数で呼び出された場合は、fx1とfx2の差、またはfxの否定を返します。

結果が固定数値でない場合、`&assertion` 条件が発生します。

Scheme Procedure: **fxdiv-and-mod** fx1 fx2

Scheme Procedure: **fxdiv** fx1 fx2

Scheme Procedure: **fxmod** fx1 fx2

スキーム手順: **fxdiv0-and-mod0** fx1 fx2

スキーム手順: **fxdiv0** fx1 fx2

スキーム手順: **fxmod0** fx1 fx2

これらの手順は、固定数に対する数論的除算を実装します。その意味論の説明については、[(rnrs base)](https://doc.guix.gnu.org/guile/latest/en/rnrs%20base.html#Top)を参照してください。

スキーム手順: **fx+/carry** fx1 fx2 fx3

以下の計算結果の 2 つの fixnum を返します。

(let\* ((s ([+](06_06_02_numerical_data_types.md#66211-算術関数) fx1 fx2 fx3))
(s0 ([mod0](#7622-rnrs-ベース) s ([expt](06_06_02_numerical_data_types.md#66212-科学関数) 2 ([fixnum-width](#76221-rnrs-算術固定番号)))))
(s1 ([div0](#7622-rnrs-ベース) s ([expt](06_06_02_numerical_data_types.md#66212-科学関数) 2 ([fixnum-width](#76221-rnrs-算術固定番号))))))
([values](06_11_controlling_the_flow_of_program_execution.md#6117-複数の値の返却と受け入れ) s0 s1))

スキーム手順: **fx-/carry** fx1 fx2 fx3

以下の計算結果の 2 つの fixnum を返します。

(let\* ((d ([\-](06_06_02_numerical_data_types.md#66211-算術関数) fx1 fx2 fx3))
(d0 ([mod0](#7622-rnrs-ベース) d ([expt](06_06_02_numerical_data_types.md#66212-科学関数) 2 ([fixnum-width](#76221-rnrs-算術固定番号)))))
(d1 ([div0](#7622-rnrs-ベース) d ([expt](06_06_02_numerical_data_types.md#66212-科学関数) 2 ([fixnum-width](#76221-rnrs-算術固定番号))))))
([values](06_11_controlling_the_flow_of_program_execution.md#6117-複数の値の返却と受け入れ) d0 d1))

スキーム手順: **fx\*/carry** fx1 fx2 fx3

以下の計算結果の 2 つの fixnum を返します。
(let\* ((s ([+](06_06_02_numerical_data_types.md#66211-算術関数) ([\*](06_06_02_numerical_data_types.md#66211-算術関数) fx1 fx2) fx3))
(s0 ([mod0](#7622-rnrs-ベース) s ([expt](06_06_02_numerical_data_types.md#66212-科学関数) 2 ([fixnum-width](#76221-rnrs-算術固定番号)))))
(s1 ([div0](#7622-rnrs-ベース) s ([expt](06_06_02_numerical_data_types.md#66212-科学関数) 2 ([fixnum-width](#76221-rnrs-算術固定番号))))))
([values](06_11_controlling_the_flow_of_program_execution.md#6117-複数の値の返却と受け入れ) s0 s1))

スキーム手順: **fxnot** fx

スキーム手順: **fxand** fx1 ...

Scheme Procedure: **fxior** fx1 ...

Scheme Procedure: **fxxor** fx1 ...

これらの手順は、Guile のコアライブラリが提供する `lognot`、`logand`、`logior`、および `logxor` 手順と同一です。詳細については、[ビット演算](06_06_02_numerical_data_types.md#66213-ビット演算) を参照してください。

スキーム手順: **fxif** fx1 fx2 fx3

fixnum引数のビットごとの「if」を返します。戻り値の`i`位置のビットは、fx1の`i`ビットが1の場合はfx2の`i`ビット、fx3の`i`ビットになります。

スキーム手順: **fxbit-count** fx

fx の 2 の補数表現における 1 ビットの数を返します。

スキーム手順: **fxlength** fx

fx を表現するために必要なビット数を返します。

スキーム手順: **fxfirst-bit-set** fx

fx の 2 の補数表現における最下位の 1 ビットのインデックスを返します。

スキーム手順: **fxbit-set?** fx1 fx2

fx1 の 2 の補数表現における fx2 番目のビットが 1 の場合は `#t` を返し、それ以外の場合は `#f` を返します。

スキーム手順: **fxcopy-bit** fx1 fx2 fx3

fx1のfx2ビット目をfx3のfx2ビット目に設定した結果を返します。

スキーム手順: **fxbit-field** fx1 fx2 fx3

fx1 内の、位置 fx2 (含む) から位置 fx3 (含まない) までの連続するビット列の整数表現を返します。

スキーム手順: **fxcopy-bit-field** fx1 fx2 fx3 fx4

fx1 の開始位置 fx2 と終了位置 fx3 のビットフィールドを、fx4 の対応するビットフィールドで置き換えた結果を返します。

Scheme Procedure: **fxarithmetic-shift** fx1 fx2

Scheme Procedure: **fxarithmetic-shift-left** fx1 fx2

Scheme Procedure: **fxarithmetic-shift-right** fx1 fx2

fx1 のビットを fx2 の位置分だけ右または左にシフトした結果を返します。`fxarithmetic-shift` は `fxarithmetic-shift-left` と同じです。

スキーム手順: **fxrotate-bit-field** fx1 fx2 fx3 fx4

fx1 のビットフィールドを、開始位置 fx2 と終了位置 fx3 から fx4 ビット分、上位ビットの方向に巡回的に置換した結果を返します。

スキーム手順: **fxreverse-bit-field** fx1 fx2 fx3

fx1 のビット列のうち、位置 fx2 (含む) と位置 fx3 (含まない) の間のビットの順序を反転させた結果を返します。

* * *

次へ: [rnrs 算術ビット演算](#76223-rnrs-ビット演算)、前: [rnrs 算術固定数値](#76221-rnrs-算術固定番号)、上: [R6RS 標準ライブラリ](#762-r6rs-標準ライブラリ) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.6.2.22 rnrs 算術 flonums

`(rnrs arithmetic flonums (6))`ライブラリは、R6RSが_flonums_と呼ぶ、実数の不正確な表現に対して算術演算を実行するための手順を提供します。

特に指定がない限り、以下のすべての手順は引数としてflonumを受け取り、flonum以外の引数が渡された場合は`&assertion`条件が発生します。

スキームプロシージャ: **flonum?** obj

objがflonumの場合は`#t`を返し、それ以外の場合は`#f`を返します。

スキーム手順: **real->flonum** x

実数 x に数値的に最も近い浮動小数点数を返します。

スキーム手順: **fl=?** fl1 fl2 fl3 ...

スキーム手順: **fl<?** fl1 fl2 fl3 ...

スキーム手順: **fl<=?** fl1 fl2 fl3 ...

スキーム手順: **fl>?** fl1 fl2 fl3 ...

スキーム手順: **fl>=?** fl1 fl2 fl3 ...

これらの手続きは、flonum引数が（それぞれ）等しい、単調増加、単調減少、単調非減少、または単調非増加である場合に`#t`を返し、それ以外の場合は`#f`を返します。

スキーム手順: **flinteger?** fl

Scheme Procedure: **flzero?** fl

スキーム手順: **flpositive?** fl

スキーム手順: **flnegative?** fl

スキーム手順: **flodd?** fl

スキーム手順: **fleven?** fl

これらの数値述語は、flがそれぞれ整数、ゼロ、ゼロより大きい、ゼロより小さい、奇数、偶数の場合は`#t`を返し、それ以外の場合は`#f`を返します。`flodd?`と`fleven?`の場合、flは整数値のflonumでなければなりません。

Scheme Procedure: **flfinite?** fl

Scheme Procedure: **flinfinite?** fl

スキームプロシージャ: **flnan?** fl

これらの数値述語は、fl がそれぞれ無限でない、無限である、または `NaN` 値である場合に `#t` を返します。

スキーム手順: **flmax** fl1 fl2 ...

スキーム手順: **flmin** fl1 fl2 ...

これらの手続きは、引数の最大値または最小値を返します。

スキーム手順: **fl+** fl1 ...

スキーム手順: **fl\*** fl ...

これらの手続きは、引数の合計または積を返します。

スキーム手順: **fl-** fl1 fl2 ...

スキーム手順: **fl-** fl

スキーム手順: **fl/** fl1 fl2 ...

スキーム手順: **fl/** fl

これらのプロシージャは、2 つの引数で呼び出された場合は、それぞれ引数の差または商を返します。1 つの引数で呼び出された場合は、fl の加法逆元または乗法逆元を返します。

Scheme Procedure: **flabs** fl

fl の絶対値を返します。

Scheme Procedure: **fldiv-and-mod** fl1 fl2

スキーム手順: **fldiv** fl1 fl2

Scheme Procedure: **fldmod** fl1 fl2

スキーム手順: **fldiv0-and-mod0** fl1 fl2

スキーム手順: **fldiv0** fl1 fl2

スキーム手順: **flmod0** fl1 fl2

これらの手順は、flonum 上での数論的除算を実装します。その意味論の説明については、[(rnrs base)](https://doc.guix.gnu.org/guile/latest/en/rnrs%20base.html#Top) を参照してください。

スキーム手順: **flnumerator** fl

スキーム手順: **fldenominator** fl

これらの手順は、fl の分子または分母を flonum として返します。

スキーム手順: **flfloor** fl1

スキーム手順: **flceiling** fl

Scheme手順: **fltruncate** fl

スキーム手順: **flround** fl

これらの手順は、Guile のコアライブラリが提供する `floor`、`ceiling`、`truncate`、および `round` 手順と同一です。詳細については、[算術関数](06_06_02_numerical_data_types.md#66211-算術関数) を参照してください。

Scheme Procedure: **flexp** fl

スキーム手順: **fllog** fl

スキーム手順: **fllog** fl1 fl2

Scheme Procedure: **flsin** fl

スキーム手順: **flcos** fl

Scheme Procedure: **fltan** fl

スキーム手順: **flasin** fl

Scheme Procedure: **flacos** fl

スキーム手順: **flatan** fl

スキーム手順: **flatan** fl1 fl2

これらの手順は、通常の超越関数を計算するもので、R6RS ベースライブラリによって提供される手順の flonum バリアントです ([(rnrs base)](https://doc.guix.gnu.org/guile/latest/en/rnrs%20base.html#Top) を参照)。

Scheme Procedure: **flsqrt** fl

fl の平方根を返します。fl が `-0.0` の場合は \-0.0 が返され、その他の負の値の場合は `NaN` が返されます。

スキーム手順: **flexpt** fl1 fl2

fl1の値をfl2乗した値を返します。

以下の条件タイプは、無限大やNaN値をサポートしていないScheme実装が、計算結果がそのような値になったことを示すために提供されています。Guileはこれら両方をサポートしているため、Guileの標準ライブラリ実装ではこれらの条件が発生することはありません。

条件タイプ: **&no-infinities**

スキーム手順: **make-no-infinities-violation** obj

スキーム手順: **無限大違反なし?**

無限大を表現できないScheme実装において、計算結果が無限大の値になったことを示す条件型。

条件タイプ: **&no-nans**

スキーム手順: **make-no-nans-violation** obj

スキーム手順: **no-nans-violation?** obj

`NaN`を表現できないScheme実装において、計算結果が`NaN`値になったことを示す条件型。

Scheme Procedure: **fixnum->flonum** fx

指定されたfixnum fxに数値的に最も近いflonumを返します。

* * *

次へ: [rnrs syntax-case](#76224-rnrs-構文ケース)、前: [rnrs arithmetic flonums](#76222-rnrs-算術-flonums)、上: [R6RS Standard Libraries](#762-r6rs-標準ライブラリ) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.6.2.23 rnrs ビット演算

`(rnrs arithmetic bitwise (6))` ライブラリは、fixnum の 2 の補数表現に対してビットごとの算術演算を実行するための手順を提供します。

このライブラリと、それがエクスポートするプロシージャは、整数のビット単位の操作をサポートする SRFI-60 と機能を共有しています ([SRFI-60 - ビットとしての整数](07_05_34_srfi60_integers_as_bits.md#7534-srfi-60---ビットとしての整数) を参照)。

スキーム手順: **ビットごとの否定** ei

スキーム手順: **ビットごとのAND** ei1 ...

スキーム手順: **bitwise-ior** ei1 ...

スキーム手順: **ビットごとのXOR** ei1 ...

これらの手順は、Guile のコアライブラリが提供する `lognot`、`logand`、`logior`、および `logxor` 手順と同一です。詳細については、[ビット演算](06_06_02_numerical_data_types.md#66213-ビット演算) を参照してください。

Scheme Procedure: **bitwise-if** ei1 ei2 ei3

引数のビットごとの「if」を返します。戻り値の `i` の位置にあるビットは、ei1 の `i` ビットが 1 の場合は ei2 の `i` ビット、ei3 の `i` ビットになります。

スキーム手順: **bitwise-bit-count** ei [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bitwise_002dbit_002dcount )

ei の 2 の補数表現における 1 ビットの数を返します。

Scheme Procedure: **bitwise-length** ei

ei を表すのに必要なビット数を返します。

スキーム手順: **bitwise-first-bit-set** ei

ei の 2 の補数表現における最下位の 1 ビットのインデックスを返します。

スキーム手順: **bitwise-bit-set?** ei1 ei2

ei1 の 2 の補数表現における ei2 番目のビットが 1 の場合は `#t` を返し、それ以外の場合は `#f` を返します。

スキーム手順: **bitwise-copy-bit** ei1 ei2 ei3

ei1のei2ビット目をei3のei2ビット目に設定した結果を返します。

スキーム手順: **bitwise-bit-field** ei1 ei2 ei3

ei1 内の、位置 ei2 (含む) から位置 ei3 (含まない) まで続く連続ビット列の整数表現を返します。

スキーム手順: **bitwise-copy-bit-field** ei1 ei2 ei3 ei4

ei1 の開始位置 ei2 と終了位置 ei3 のビットフィールドを、ei4 の対応するビットフィールドで置き換えた結果を返します。

スキーム手順: **ビット単位の算術シフト** ei1 ei2

Scheme Procedure: **bitwise-arithmetic-shift-left** ei1 ei2

スキーム手順: **ビット単位算術右シフト** ei1 ei2

ei1 のビットを ei2 の位置分だけ右または左にシフトした結果を返します。`bitwise-arithmetic-shift` は `bitwise-arithmetic-shift-left` と同じです。

スキーム手順: **bitwise-rotate-bit-field** ei1 ei2 ei3 ei4

ei1 のビットフィールドを、開始位置 ei2 と終了位置 ei3 で、ei4 ビット分だけ上位ビットの方向に巡回的に置換した結果を返します。

スキーム手順: **ビットごとのビットフィールドの反転** ei1 ei2 ei3

ei1 のビット列のうち、位置 ei2 (含む) と位置 ei3 (含まない) の間のビット列の順序を反転させた結果を返します。

* * *

次へ: [rnrs ハッシュテーブル](https://doc.guix.gnu.org/guile/latest/en/guile.html#rnrs-hashtables )、前: [rnrs ビット演算](#76223-rnrs-ビット演算)、上: [R6RS 標準ライブラリ](#762-r6rs-標準ライブラリ) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.6.2.24 rnrs 構文ケース

`(rnrs syntax-case (6))` ライブラリは、衛生的なマクロを作成するための `syntax-case` システムへのアクセスを提供します。1 つの例外を除き、このライブラリによってエクスポートされるすべてのフォームとプロシージャは、Guile の `syntax-case` に対するネイティブサポートの「再エクスポート」です。ドキュメント、例、および根拠については、[`syntax-case` システムのサポート](06_08_macros.md#683-syntax-case-システムのサポート) を参照してください。

Scheme プロシージャ: **make-variable-transformer** proc

構文オブジェクトを入力として受け取り、構文オブジェクトを返すプロシージャである proc から、新しい変数トランスフォーマーを作成します。このプロシージャの結果がバインドされる識別子が `set!` 式の左辺にある場合、`set!` 式全体を表す構文オブジェクトとともに proc が呼び出され、その戻り値が `set!` 式を置き換えます。

Scheme構文: **syntax-case** 式 (リテラル ...) 節 ...

構文大文字小文字を区別するパターンマッチング形式。

Scheme構文: **構文** テンプレート

Scheme構文: **quasisyntax**テンプレート

Scheme構文: **unsyntax**テンプレート

Scheme構文: **unsyntax-splicing**テンプレート

これらの形式を使用すると、構文ケース出力式のサブフォームの本体内で、データ値と非データ値を参照できます。これらは、Guile のコアライブラリが提供する形式と同一です。詳細については、「構文ケースシステムのサポート」(https://doc.guix.gnu.org/guile/latest/en/guile.html#Syntax-Case) を参照してください。

Scheme 手順: **identifier?** obj

スキーム手順: **bound-identifier=?** id1 id2

スキーム手順: **free-identifier=?** id1 id2

これらの述語手続きは、Scheme 識別子を表す構文オブジェクトに対して動作します。`identifier?` は、obj が識別子を表す場合は `#t` を返し、そうでない場合は `#f` を返します。`bound-identifier=?` は、id1 のバインディングがトランスフォーマーの出力で id2 への参照をキャプチャする場合、またはその逆の場合に限り `#t` を返します。`free-identifier=?` は、トランスフォーマーによって導入されたバインディングに関係なく、トランスフォーマーの出力で id1 と id2 が同じバインディングを参照する場合に限り `#t` を返します。

Scheme Procedure: **generate-temporaries** l

グローバルに一意なシンボルのリストを返します。このリストは、l と同じ長さで、l はリスト、またはリストを表す構文オブジェクトである必要があります。

スキーム手順: **syntax->datum** syntax-object

スキーム手順: **データ->構文** テンプレート ID データ

これらの手順は、ラップされた構文オブジェクトをSchemeデータ値に変換し、またSchemeデータ値からラップされた構文オブジェクトに変換します。`datum->syntax`によって返される構文オブジェクトは、構文オブジェクトのテンプレートIDとコンテキスト情報を共有します。

Scheme 手順: **syntax-violation** whom メッセージ形式

Scheme Procedure: **syntax-violation** whom message form subform

以下の単純条件を含む新しい複合条件を構築します。

* whomが`#f`でない場合、whomをフィールドとする`&who`条件
* 指定されたメッセージを含む `&message` 条件
* 指定されたフォームとオプションのサブフォームフィールドを含む `&syntax` 条件

* * *

次へ: [rnrs enums](#76226-rnrs-列挙型)、前: [rnrs syntax-case](#76224-rnrs-構文ケース)、上: [R6RS Standard Libraries](#762-r6rs-標準ライブラリ) \[[Contents](00_contents.md "目次")\]\[[Index](index_r5rs.md "索引")\]

#### 7.6.2.25 rnrs ハッシュテーブル

`(rnrs hashtables (6))` ライブラリは、ハッシュテーブルの作成とアクセスを行うための構造と手順を提供します。R6RS で定義されているハッシュテーブル API は、Guile のネイティブなハッシュテーブル実装と SRFI-69 が提供するハッシュテーブル実装の両方と実質的に同じです。それぞれ、[ハッシュテーブル](06_06_22_hash_tables.md#6622-ハッシュテーブル) および [SRFI-69 - 基本的なハッシュテーブル](07_05_39_srfi69_basic_hash_tables.md#7539-srfi-69---基本ハッシュテーブル) を参照してください。`(srfi :69)` ライブラリをインポートすることで、SRFI-69 ハッシュテーブルを操作する移植可能な R6RS ライブラリ コードを記述できますが、ある API で作成されたハッシュテーブルを別の API で使用することはできません。

SRFI-69ハッシュテーブルと同様に（Guileのネイティブハッシュテーブルとは異なり）、R6RSハッシュテーブルは作成時にハッシュ関数と等価関数をハッシュテーブルに関連付けます。さらに、R6RSでは（`hashtable-copy`コマンドを使用。詳細は後述）不変ハッシュテーブルの作成も可能です。

Scheme手順: **make-eq-hashtable**

Scheme 手順: **make-eq-hashtable** k

キーの比較に`eq?`を使用し、ハッシュ関数としてGuileの`hashq`プロシージャを使用する新しいハッシュテーブルを返します。kが指定されている場合は、ハッシュテーブルの初期容量を指定します。

Scheme手順: **make-eqv-hashtable**

Scheme 手順: **make-eqv-hashtable** k

キーの比較に`eqv?`を使用し、ハッシュ関数としてGuileの`hashv`プロシージャを使用する新しいハッシュテーブルを返します。kが指定されている場合は、ハッシュテーブルの初期容量を指定します。

Scheme手順: **make-hashtable** ハッシュ関数相当

Scheme 手順: **make-hashtable** hash-function equiv k

キーの比較にequiv、ハッシュ関数にhash-functionを使用する新しいハッシュテーブルを返します。equivは2つの引数を受け取り、それらが等しい場合はtrue、そうでない場合は#fを返すプロシージャである必要があります。hash-functionは1つの引数を受け取り、負でない整数を返すプロシージャである必要があります。

kが指定されている場合、それはハッシュテーブルの初期容量を指定します。

Scheme手順: **hashtable?** obj

objがR6RSハッシュテーブルの場合は`#t`を返し、それ以外の場合は`#f`を返します。

Scheme手順: **hashtable-size** ハッシュテーブル

現在ハッシュテーブル hashtable に存在するキーの数を返します。

スキーム手順: **hashtable-ref** ハッシュテーブルキーのデフォルト値

ハッシュテーブル hashtable 内のキーに関連付けられた値を返します。値が見つからない場合はデフォルト値を返します。

Scheme Procedure: **hashtable-set!** ハッシュテーブルキー obj

ハッシュテーブル hashtable 内のキー key を値 obj に関連付け、未指定の値を返します。ハッシュテーブルが不変である場合は、`&assertion` 条件が発生します。

Scheme Procedure: **hashtable-delete!** ハッシュテーブルキー

ハッシュテーブル hashtable 内でキー key に関連付けられているすべての関連付けを削除し、未指定の値を返します。ハッシュテーブルが不変である場合は、`&assertion` 条件が発生します。

Scheme手順: **hashtable-contains?** ハッシュテーブルキー

ハッシュテーブル hashtable にキー key に対応する関連付けが含まれている場合は `#t` を返し、そうでない場合は `#f` を返します。

Scheme Procedure: **hashtable-update!** hashtable key proc default

ハッシュテーブル hashtable 内のキーに、現在キーに関連付けられている値に対して proc を呼び出し、その結果を関連付けます。proc は引数を 1 つ取るプロシージャでなければなりません。関連付けが存在しない場合は、デフォルト値を使用します。ハッシュテーブルが不変である場合は、`&assertion` 条件が発生します。

Scheme手順: **hashtable-copy** ハッシュテーブル

Scheme 手順: **hashtable-copy** ハッシュテーブル可変

ハッシュテーブル hashtable のコピーを返します。オプション引数 mutable が指定され、それが true の場合、新しいハッシュテーブルは可変になります。

Scheme 手順: **hashtable-clear!** ハッシュテーブル

Scheme Procedure: **hashtable-clear!** hashtable k

ハッシュテーブル hashtable からすべての関連付けを削除します。ハッシュテーブルの新しい容量を指定するオプション引数 k は、Guile の `(rnrs hashtables)` 実装で受け入れられますが、無視されます。

Scheme Procedure: **hashtable-keys** ハッシュテーブル

ハッシュテーブル hashtable 内の関連付けを持つキーのベクトルを、指定なしの順序で返します。

Scheme Procedure: **hashtable-entries** hashtable

ハッシュテーブル内の関連付けを持つキーのベクトルと、これらのキーが対応する値のベクトル（順序は指定なし）の2つの値を返します。

Scheme Procedure: **hashtable-equivalence-function** hashtable

ハッシュテーブルによる等価性述語の使用を返します。このプロシージャは、`make-eq-hashtable` および `make-eqv-hashtable` で作成されたハッシュテーブルに対して、それぞれ `eq?` および `eqv?` を返します。

Scheme Procedure: **hashtable-hash-function** hashtable

hashtableで使用されるハッシュ関数を返します。`make-eq- hashtable`または`make-eqv-hashtable`で作成されたハッシュテーブルの場合は、`#f`が返されます。

Scheme手順: **hashtable-mutable?** ハッシュテーブル

ハッシュテーブルが可変の場合は`#t`を返し、そうでない場合は`#f`を返します。

利便性のために、いくつかのハッシュ関数が用意されています。

Scheme Procedure: **equal-hash** obj

objの構造と現在の内容に基づいて、objの整数ハッシュ値を返します。このハッシュ関数は、等価性関数として`equal?`と組み合わせて使用するのに適しています。

Scheme手順: **string-hash** string

スキーム手順: **symbol-hash** シンボル

これらの手順は、Guile のコアライブラリが提供する手順と同一です。詳細については、[ハッシュテーブルリファレンス](06_06_22_hash_tables.md#66222-ハッシュテーブルリファレンス) を参照してください。

Scheme Procedure: **string-ci-hash** string

文字列の内容に基づいて、大文字小文字を区別せずに整数ハッシュ値を返します。このハッシュ関数は、`string-ci=?` と併用して等価関数として使用するのに適しています。

* * *

次へ: [rnrs](#76227-rnrs)、前: [rnrs ハッシュテーブル](#76225-rnrs-ハッシュテーブル)、上: [R6RS 標準ライブラリ](#762-r6rs-標準ライブラリ) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.6.2.26 rnrs 列挙型

`(rnrs enums (6))`ライブラリは、列挙可能なシンボルの集合を操作するための構造と手順を提供します。Guileの実装では、有限個の異なるシンボルの集合である_universe_と、これらのシンボルのサブセット（列挙セットを定義する）をカプセル化する_enum-set_レコード型が定義されています。

SRFI-1 リストライブラリは、リストに対する集合演算を実行するための多数の手続きを提供しています。Guile の `(rnrs enums)` 実装では、それらのいくつかを使用しています。詳細については、[リストに対する集合演算](07_05_03_srfi1_list_library.md#75310-リストに対する集合演算) を参照してください。

Scheme手順: **make-enumeration** シンボルリスト

ユニバースと列挙セットの両方がシンボルのリストである symbol-list と等しい新しい enum-set を返します。

スキーム手順: **enum-set-universe** enum-set

enum-set の全体を表す enum-set を返します。

スキーム手順: **enum-set-indexer** enum-set

単一の引数を受け取り、その引数が列挙型セットの集合内でゼロから始まる位置を返す手続きを返します。引数がその集合の要素でない場合は `#f` を返します。

Scheme プロシージャ: **enum-set-constructor** enum-set

単一の引数、列挙型セットのユニバースからのシンボルのリスト、列挙型セットを受け取り、指定されたシンボルを含む部分集合を表す、同じユニバースを持つ新しい列挙型セットを返すプロシージャを返します。

Scheme手順: **enum-set->list** enum-set

enum-set で表されるセットのシンボルを、enum-set のユニバースに出現する順序で含むリストを返します。

スキームプロシージャ: **enum-set-member?** シンボル enum-set

スキーム手順: **enum-set-subset?** enum-set1 enum-set2

スキームプロシージャ: **enum-set=?** enum-set1 enum-set2

これらの手順では、シンボルと列挙型セットが他の列挙型セットに属しているかどうかをテストします。`enum-set-member?` は、シンボルが列挙型セットで指定された部分集合のメンバーである場合に限り `#t` を返します。`enum-set-subset?` は、列挙型セット 1 の全体集合が列挙型セット 2 の全体集合の部分集合であり、かつ列挙型セット 1 のすべてのシンボルが列挙型セット 2 に存在する場合に限り `#t` を返します。`enum-set=?` は、列挙型セット 1 が列挙型セット 2 の `enum-set-subset?` に従って部分集合である場合、およびその逆の場合に限り `#t` を返します。

Scheme Procedure: **enum-set-union** enum-set1 enum-set2

Scheme Procedure: **enum-set-intersection** enum-set1 enum-set2

Scheme Procedure: **enum-set-difference** enum-set1 enum-set2

これらの手続きは、それぞれ列挙型セットの引数の和集合、積集合、差集合を返します。

Scheme 手順: **enum-set-complement** enum-set

列挙型セットの補集合（列挙型セット）を、その集合に関して返します。

Scheme手順: **enum-set-projection** enum-set1 enum-set2

列挙型セット enum-set1 を列挙型セット enum-set2 の全体集合に投影したものを返します。

Scheme構文: **define-enumeration** type-name (symbol ...) constructor-syntax

2 つの新しい定義に評価されます。1 つは、上記の `enum-set-constructor` によって作成されたコンストラクタと同様に動作し、`(symbol ...)` で指定されたユニバースに新しい enum-set を作成する、コンストラクタ構文にバインドされたコンストラクタです。もう 1 つは、次の形式を持つ、型名にバインドされた「述語マクロ」です。

(型名 sym)

symが上記の記号で指定されたユニバースの要素である場合、この形式はsymと評価されます。そうでない場合は、`&syntax`条件が発生します。

* * *

次へ: [rnrs eval](#76228-rnrs-eval)、前: [rnrs enums](#76226-rnrs-列挙型)、上: [R6RS 標準ライブラリ](#762-r6rs-標準ライブラリ) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.6.2.27 rnrs

`(rnrs (6))`ライブラリは、他のすべてのR6RS標準ライブラリの複合体であり、以下のライブラリを除き、それらのライブラリからエクスポートされたすべての手続きと構文形式をインポートおよび再エクスポートします。

* `(rnrs eval (6))`
* `(rnrs 可変ペア (6))`
* `(rnrs 可変文字列 (6))`
* `(rnrs r5rs (6))`

* * *

次へ: [rnrs mutable-pairs](#76229-rnrs-mutable-pairs)、前: [rnrs](#76227-rnrs)、上: [R6RS 標準ライブラリ](#762-r6rs-標準ライブラリ) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.6.2.28 rnrs eval

`(rnrs eval (6)`ライブラリは、式の「オンザフライ」評価を実行するための手順を提供します。

Scheme手順: **eval**式環境

指定された環境（environment）で、有効な Scheme 式のデータ表現である式を評価します。この手順は Guile のコードライブラリが提供する手順と同一です。詳細については、「オンザフライ評価の手順」(https://doc.guix.gnu.org/guile/latest/en/guile.html#Fly-Evaluation) を参照してください。

Scheme 手順: **環境** import-spec ...

指定されたインポート仕様に基づいて新しい環境を構築して返します。インポート仕様は、`import` フォームで使用されるインポート仕様のデータ表現である必要があります。詳細については、[R6RS ライブラリ](06_18_modules.md#6186-r6rsライブラリ) を参照してください。

* * *

次へ: [rnrs mutable-strings](#76230-rnrs-mutable-strings)、前: [rnrs eval](#76228-rnrs-eval)、上: [R6RS 標準ライブラリ](#762-r6rs-標準ライブラリ) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.6.2.29 rnrs mutable-pairs

`(rnrs mutable-pairs (6))`ライブラリは、ペアの`car`フィールドと`cdr`フィールドを変更できる`set-car!`および`set-cdr!`プロシージャを提供します。

これらの手順は、Guile のコアライブラリが提供する手順と同一です。詳細については、[Pairs](06_06_08_pairs.md#668-ペア) を参照してください。Guile のすべてのペアは変更可能であるため、これらの手順では R6RS ライブラリ仕様で説明されている `&assertion` 条件がスローされることはありません。

* * *

次へ: [rnrs r5rs](#76231-rnrs-r5rs)、前: [rnrs mutable-pairs](#76229-rnrs-mutable-pairs)、上: [R6RS 標準ライブラリ](#762-r6rs-標準ライブラリ) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.6.2.30 rnrs mutable-strings

`(rnrs mutable-strings (6))`ライブラリは、文字列の内容を「その場で」変更できる`string-set!`と`string-fill!`の手順を提供します。

これらの手順は、Guile のコアライブラリが提供する手順と同一です。詳細については、[文字列の変更](06_06_05_strings.md#6656-文字列の変更) を参照してください。Guile のすべての文字列は変更可能であるため、これらの手順では、R6RS ライブラリ仕様で説明されている `&assertion` 条件がスローされることはありません。

* * *

前へ: [rnrs mutable-strings](#76230-rnrs-mutable-strings)、上へ: [R6RS 標準ライブラリ](#762-r6rs-標準ライブラリ) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.6.2.31 rnrs r5rs

`(rnrs r5rs (6))`ライブラリは、R5RSに存在するがR6RS基本ライブラリ仕様から省略された一部の手続きのバインディングをエクスポートします。

スキーム手順: **exact->inexact** z

Scheme手順: **inexact->exact** z

これらの手順は、Guile のコアライブラリが提供する手順と同一です。詳細については、[正確な数値と不正確な数値](06_06_02_numerical_data_types.md#6625-正確な数と不正確な数) を参照してください。

スキーム手順: **quotient** n1 n2

スキーム手順: **remainder** n1 n2

スキーム手順: **modulo** n1 n2

これらの手順は、Guile のコアライブラリが提供する手順と同一です。詳細については、「整数値の操作」を参照してください。

Scheme構文: **delay** expr

Scheme 手順: **force** promise

`delay` 形式と `force` 手順は、Guile のコアライブラリの対応するものと同一です。詳細については、[遅延評価](06_16_reading_and_evaluating_scheme_code.md#61610-遅延評価) を参照してください。

Scheme Procedure: **null-environment** n

Scheme 手順: **scheme-report-environment** n

これらの手順は、Guile モジュール `(ice-9 r5rs)` によって提供される手順と同一です。詳細については、[Environments](06_18_modules.md#61812-環境) を参照してください。

* * *

次へ: [パターンマッチング](07_08_pattern_matching.md#78-パターンマッチング)、前: [R6RS サポート](#76-r6rs-サポート)、上: [Guile モジュール](07_00_guile_modules.md#7つのguileモジュール) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
