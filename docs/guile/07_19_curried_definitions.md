### 7.19 カリー定義 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Curried-Definitions-1)

このセクションのマクロは以下によって提供されています

([use-modules](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-use_002dmodules) (ice-9 curried-definitions))

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

Scheme構文: **define** (… (name args …) …) body … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-define-2)

Scheme構文: **define\*** (… (name args …) …) body … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-define_002a-1)

Scheme構文: **define-public** (… (name args …) …) body … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-define_002dpublic-1)

パラメータリストargsを持つプロシージャにバインドされたトップレベル変数nameを作成します。name自体が仮パラメータリストである場合、その仮パラメータリストを使用して高階プロシージャが作成され、パラメータリストargsを持つプロシージャが返されます。このネストは任意の深さまで可能です。

`define*` も同様ですが、仮引数リストは [lambda\* および define\* ](https://doc.guix.gnu.org/guile/latest/en/guile.html#lambda_002a-and-define_002a) で説明されているように追加のオプションを受け取ります。たとえば、

(define\* ((foo #:keys (bar 'baz) (quux 'zot)) frotz #:rest rest)
(リストバー quux frotz レスト)

((foo #:quux 'foo) 1 2 3 4 5)
⇒ (baz foo 1 (2 3 4 5))

`define-public` は `define` と似ていますが、現在のモジュールのエクスポートされたバインディングのリストに名前を追加します。

* * *

次へ: [SXML](https://doc.guix.gnu.org/guile/latest/en/guile.html#SXML)、前: [カリー定義](https://doc.guix.gnu.org/guile/latest/en/guile.html#Curried-Definitions)、上: [Guile モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guile-Modules) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
