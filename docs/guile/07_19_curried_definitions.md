### 7.19 カリー定義

このセクションのマクロは以下によって提供されています

([use-modules](06_18_modules.md#6182-guileモジュールの使用) (ice-9 curried-definitions))

そして、デフォルトで提供されているものを置き換えます。

Guile 2.0以前のGuileでは、「カリー化定義」と呼ばれる定義形式が提供されていました。これは、`define`の構文を拡張し、任意の深さまでプロシージャを返すプロシージャを簡単に定義できるようにするものです。

例えば、

(define ((foo x) y)
(リストxy)

は便利な形式です

(define foo
(ラムダ (x)
(ラムダ (y)
(リスト xy))))

Scheme構文: **define** (… (name args …) …) body …

Scheme構文: **define\*** (… (name args …) …) body …

Scheme構文: **define-public** (… (name args …) …) body …

パラメータリストargsを持つプロシージャにバインドされたトップレベル変数nameを作成します。name自体が仮パラメータリストである場合、その仮パラメータリストを使用して高階プロシージャが作成され、パラメータリストargsを持つプロシージャが返されます。このネストは任意の深さまで可能です。

`define*` も同様ですが、仮引数リストは [lambda\* および define\* ](06_07_procedures.md#6741-lambda-と-define) で説明されているように追加のオプションを受け取ります。たとえば、

(define\* ((foo #:keys (bar 'baz) (quux 'zot)) frotz #:rest rest)
(リストバー quux frotz レスト)

((foo #:quux 'foo) 1 2 3 4 5)
⇒ (baz foo 1 (2 3 4 5))

`define-public` は `define` と似ていますが、現在のモジュールのエクスポートされたバインディングのリストに名前を追加します。

* * *

次へ: [SXML](07_21_sxml.md#721-sxml)、前: [カリー定義](#719-カリー定義)、上: [Guile モジュール](07_00_guile_modules.md#7つのguileモジュール) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
