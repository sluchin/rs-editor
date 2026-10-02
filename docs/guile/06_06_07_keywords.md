# 6.6.7 キーワード

> **原文**: [Guile Reference Manual - Keywords](https://www.gnu.org/software/guile/manual/html_node/Keywords.html)
>
> このドキュメントは GNU Free Documentation License の下で公開されている原文の翻訳です。

キーワードは、入力しやすい便利な読み取り構文を持つ、自己評価的なオブジェクトです。

Guile のキーワードのサポートは R5RS に準拠しており、キーワードを `#:` だけでなく `:` で始めたり、`:` で終えたりできるようにする（切り替え可能な）読み取り構文の拡張を追加しています。

- なぜキーワードを使うのか？
- キーワードを使ったコーディング
- キーワードの読み取り構文
- キーワードの手続き

## 6.6.7.1 なぜキーワードを使うのか？

キーワードは、プログラムや手続きが、インターフェースを扱いにくくすることなく、多数のオプション引数を受け付けられるようにしたい文脈で役立ちます。

これを説明するために、何らかのグラフィカルツールキットを使って描画するための新しいウィンドウを画面上に作成する、仮想的な `make-window` 手続きを考えてみましょう。呼び出し側が指定したいかもしれないが、適切にデフォルト値を設定することもできるパラメータがたくさんあります。たとえば次のようなものです。

- 色深度 ― デフォルト: 画面の色深度
- 背景色 ― デフォルト: 白
- 幅 ― デフォルト: 600
- 高さ ― デフォルト: 400

`make-window` がキーワードを使わなかった場合、呼び出し側は可能な各引数に値を渡さなければならず、正しい引数の順序を覚え、その引数のデフォルト値を示すために特別な値を使わなければなりません。

```scheme
(make-window 'default              ;; 色深度
             'default              ;; 背景色
             800                   ;; 幅
             100                   ;; 高さ
             …)                    ;; make-window のさらなる引数
```

一方、キーワードを使えば、デフォルトの引数は省略され、デフォルトでない引数は適切なキーワードで明確にタグ付けされます。その結果、呼び出しははるかに明確になります。

```scheme
(make-window #:width 800 #:height 100)
```

一方、引数が少ないより単純な手続きでは、キーワードの使用は助けになるどころか妨げになるでしょう。たとえば、プリミティブ手続き `cons` は、次のように呼び出さなければならないとしたら改善されたことにはならないでしょう。

```scheme
(cons #:car x #:cdr y)
```

したがって、キーワードを使うかどうかの決定は純粋に実用的なものです。呼び出し地点で手続きの呼び出しが明確になるなら使ってください。

## 6.6.7.2 キーワードを使ったコーディング

手続きがキーワードをサポートしたい場合は、残余引数を取り、その残余引数の内容からキーワードとそれに対応する引数を取り出すのに便利な手段を何でも使うべきです。

次の例はその原理を示しています。`make-window` のコードは、`get-keyword-value` という補助手続きを使って、残余引数から個々のキーワード引数を取り出しています。

```scheme
(define (get-keyword-value args keyword default)
  (let ((kv (memq keyword args)))
    (if (and kv (>= (length kv) 2))
        (cadr kv)
        default)))

(define (make-window . args)
  (let ((depth  (get-keyword-value args #:depth  screen-depth))
        (bg     (get-keyword-value args #:bg     "white"))
        (width  (get-keyword-value args #:width  800))
        (height (get-keyword-value args #:height 100))
        …)
    …))
```

しかし、`get-keyword-value` を書く必要はありません。`(ice-9 optargs)` モジュールは、このようなキーワードをサポートする手続きを実装するために使える、強力なマクロの集合を提供しています。

```scheme
(use-modules (ice-9 optargs))

(define (make-window . args)
  (let-keywords args #f ((depth  screen-depth)
                         (bg     "white")
                         (width  800)
                         (height 100))
    ...))
```

あるいは、さらに簡潔に、次のようにします。

```scheme
(use-modules (ice-9 optargs))

(define* (make-window #:key (depth  screen-depth)
                            (bg     "white")
                            (width  800)
                            (height 100))
  ...)
```

`let-keywords`、`define*`、その他 `(ice-9 optargs)` モジュールが提供する機能の詳細については、「オプション引数」を参照してください。

C で実装された手続きのキーワード引数を扱うには、`scm_c_bind_keyword_arguments` を使ってください（「キーワードの手続き」を参照）。

## 6.6.7.3 キーワードの読み取り構文

Guile は、デフォルトでは R5RS と互換性のあるキーワードの構文のみを認識します。`NAME` が Scheme のシンボルと同じ構文（「シンボルの拡張読み取り構文」を参照）を持つ `#:NAME` という形式のトークンは、`NAME` という名前のキーワードの外部表現です。キーワードオブジェクトもこの構文を使って表示されるので、キーワードオブジェクトを含む値は Guile に読み戻すことができます。式の中で使われるとき、キーワードは自己クォートするオブジェクトです。

`keywords` 読み取りオプションが `'prefix` に設定されている場合、Guile は代替の読み取り構文 `:NAME` も認識します。そうでなければ、R5RS で要求されているとおり、`:NAME` という形式のトークンはシンボルとして読み込まれます。

`keywords` 読み取りオプションが `'postfix` に設定されている場合、Guile は SRFI-88 の読み取り構文 `NAME:` を認識します（「SRFI-88 キーワードオブジェクト」を参照）。そうでなければ、この形式のトークンはシンボルとして読み込まれます。

代替の非 R5RS のキーワード構文を有効化および無効化するには、「Scheme コードの読み込み」で文書化されている `read-set!` 手続きを使います。`prefix` と `postfix` の構文は互いに排他的であることに注意してください。

```scheme
(read-set! keywords 'prefix)

#:type
⇒
#:type

:type
⇒
#:type

(read-set! keywords 'postfix)

type:
⇒
#:type

:type
⇒
:type

(read-set! keywords #f)

#:type
⇒
#:type

:type
⊣
ERROR: In expression :type:
ERROR: Unbound variable: :type
ABORT: (unbound-variable)
```

## 6.6.7.4 キーワードの手続き

**Scheme 手続き: `keyword? obj`**<br>**C 関数: `scm_keyword_p (obj)`**
: 引数 `obj` がキーワードであれば `#t` を、そうでなければ `#f` を返します。

**Scheme 手続き: `keyword->symbol keyword`**<br>**C 関数: `scm_keyword_to_symbol (keyword)`**
: `keyword` と同じ名前を持つシンボルを返します。

**Scheme 手続き: `symbol->keyword symbol`**<br>**C 関数: `scm_symbol_to_keyword (symbol)`**
: `symbol` と同じ名前を持つキーワードを返します。

**C 関数: `int scm_is_keyword (SCM obj)`**
: `scm_is_true (scm_keyword_p (obj))` と同等です。

**C 関数: `SCM scm_from_locale_keyword (const char *name)`**<br>**C 関数: `SCM scm_from_locale_keywordn (const char *name, size_t len)`**
: それぞれ `scm_symbol_to_keyword (scm_from_locale_symbol (name))` および `scm_symbol_to_keyword (scm_from_locale_symboln (name, len))` と同等です。

  現在のロケールが、文字列と文字の定数に使われる実行文字集合のものと一致する保証はないため、`name` が C の文字列定数である場合にはこれらの関数を使うべきではないことに注意してください。最近のほとんどの C コンパイラはデフォルトで UTF-8 を使うので、そのような場合には `scm_from_utf8_keyword` を推奨します。

**C 関数: `SCM scm_from_latin1_keyword (const char *name)`**<br>**C 関数: `SCM scm_from_utf8_keyword (const char *name)`**
: それぞれ `scm_symbol_to_keyword (scm_from_latin1_symbol (name))` および `scm_symbol_to_keyword (scm_from_utf8_symbol (name))` と同等です。

**C 関数: `void scm_c_bind_keyword_arguments (const char *subr, SCM rest, scm_t_keyword_arguments_flags flags, SCM keyword1, SCM *argp1, …, SCM keywordN, SCM *argpN, SCM_UNDEFINED)`**
: 指定されたキーワード引数を `rest` から取り出します。`rest` は変更されません。キーワード引数 `keyword1` が関連付けられた値とともに `rest` に存在する場合、その値は `argp1` が指す変数に格納され、そうでなければ変数は変更されないままになります。`keywordN` と `argpN` までの他のキーワードと引数のポインタについても同様です。`scm_c_bind_keyword_arguments` への引数のリストは `SCM_UNDEFINED` で終わらなければなりません。

  関連するキーワード引数が存在しない場合、`argp1` から `argpN` が指す変数は変更されないままなので、`scm_c_bind_keyword_arguments` を呼び出す前にそれらをデフォルト値に初期化しておくべきであることに注意してください。あるいは、呼び出しの前にそれらを `SCM_UNDEFINED` に初期化しておき、呼び出しの後で `SCM_UNBNDP` を使ってどれが与えられたかを確認することもできます。

  認識されないキーワード引数が `rest` に存在し、`flags` が `SCM_ALLOW_OTHER_KEYS` を含んでいない場合、またはキーワードでない引数が存在し、`flags` が `SCM_ALLOW_NON_KEYWORD_ARGUMENTS` を含んでいない場合は、例外が発生します。`subr` は、エラー報告のために、キーワード引数を受け取る手続きの名前であるべきです。

  例:

  ```c
  SCM k_delimiter;
  SCM k_grammar;
  SCM sym_infix;

  SCM my_string_join (SCM strings, SCM rest)
  {
    SCM delimiter = SCM_UNDEFINED;
    SCM grammar   = sym_infix;

    scm_c_bind_keyword_arguments ("my-string-join", rest, 0,
                                  k_delimiter, &delimiter,
                                  k_grammar, &grammar,
                                  SCM_UNDEFINED);

    if (SCM_UNBNDP (delimiter))
      delimiter = scm_from_utf8_string (" ");

    return scm_string_join (strings, delimiter, grammar);
  }

  void my_init ()
  {
    k_delimiter = scm_from_utf8_keyword ("delimiter");
    k_grammar   = scm_from_utf8_keyword ("grammar");
    sym_infix   = scm_from_utf8_symbol ("infix");
    scm_c_define_gsubr ("my-string-join", 1, 0, 1, my_string_join);
  }
  ```

---

> **ライセンス**: この翻訳は GNU Free Documentation License v1.3 以降に基づいて作成されています。
> 原文の著作権: Copyright (C) 1996-2023 Free Software Foundation, Inc.
