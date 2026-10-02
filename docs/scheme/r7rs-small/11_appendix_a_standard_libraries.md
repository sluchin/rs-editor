# 標準ライブラリ

本節では，標準ライブラリで提供されるエクスポートを一覧表示する。 ライブラリは，すべての実装でサポートされないかもしれない， あるいは負荷が高いかもしれない機能を分離するように考慮されている。

`scheme` ライブラリ接頭辞にはすべての標準ライブラリで使用され， 将来の規格で使用するために予約されている。

**base ライブラリ**

`(scheme base)`ライブラリは， 伝統的に Scheme と関連付けられた多くの手続きと構文束縛をエクスポートする。 base ライブラリと他の標準ライブラリの境界は，構造上ではなく，使用上に基づく。 特に，他の標準手続きまたは構文の点で コンパイラや実行システムによって通常プリミティブとして実装されている一部の手段は， base ライブラリの一部としてではなく，分離したライブラリとして定義されている。 同様に， base ライブラリの一部のエクスポートは，他のエクスポートの点で実装可能である。 それらは厳密な意味で冗長ではあるが， 使い方の共通パターンを取り込んでいるので，便利な略語として提供されている。

```scheme
 *                        +
 -                        ...
 /                        <
 <=                       =
 =>                       >
 >=                       _
 abs                      and
 append                   apply
 assoc                    assq
 assv                     begin
 binary-port?             boolean=?
 boolean?                 bytevector
 bytevector-append        bytevector-copy
 bytevector-copy!         bytevector-length
 bytevector-u8-ref        bytevector-u8-set!
 bytevector?              caar
 cadr
 call-with-current-continuation
 call-with-port           call-with-values
 call/cc                  car
 case                     cdar
 cddr                     cdr
 ceiling                  char->integer
 char-ready?              char<=?
 char<?                   char=?
 char>=?                  char>?
 char?                    close-input-port
 close-output-port        close-port
 complex?                 cond
 cond-expand              cons
 current-error-port       current-input-port
 current-output-port      define
 define-record-type       define-syntax
 define-values            denominator
 do                       dynamic-wind
 else                     eof-object
 eof-object?              eq?
 equal?                   eqv?
 error                    error-object-irritants
 error-object-message     error-object?
 even?                    exact
 exact-integer-sqrt       exact-integer?
 exact?                   expt
 features                 file-error?
 floor                    floor-quotient
 floor-remainder          floor/
 flush-output-port        for-each
 gcd                      get-output-bytevector
 get-output-string        guard
 if                       include
 include-ci               inexact
 inexact?                 input-port-open?
 input-port?              integer->char
 integer?                 lambda
 lcm                      length
 let                      let*
 let*-values              let-syntax
 let-values               letrec
 letrec*                  letrec-syntax
 list                     list->string
 list->vector             list-copy
 list-ref                 list-set!
 list-tail                list?
 make-bytevector          make-list
 make-parameter           make-string
 make-vector              map
 max                      member
 memq                     memv
 min                      modulo
 negative?                newline
 not                      null?
 number->string           number?
 numerator                odd?
 open-input-bytevector    open-input-string
 open-output-bytevector   open-output-string
 or                       output-port-open?
 output-port?             pair?
 parameterize             peek-char
 peek-u8                  port?
 positive?                procedure?
 quasiquote               quote
 quotient                 raise
 raise-continuable        rational?
 rationalize              read-bytevector
 read-bytevector!         read-char
 read-error?              read-line
 read-string              read-u8
 real?                    remainder
 reverse                  round
 set!                     set-car!
 set-cdr!                 square
 string                   string->list
 string->number           string->symbol
 string->utf8             string->vector
 string-append            string-copy
 string-copy!             string-fill!
 string-for-each          string-length
 string-map               string-ref
 string-set!              string<=?
 string<?                 string=?
 string>=?                string>?
 string?                  substring
 symbol->string           symbol=?
 symbol?                  syntax-error
 syntax-rules             textual-port?
 truncate                 truncate-quotient
 truncate-remainder       truncate/
 u8-ready?                unless
 unquote                  unquote-splicing
 utf8->string             values
 vector                   vector->list
 vector->string           vector-append
 vector-copy              vector-copy!
 vector-fill!             vector-for-each
 vector-length            vector-map
 vector-ref               vector-set!
 vector?                  when
 with-exception-handler   write-bytevector
 write-char               write-string
 write-u8                 zero?
```

**case-lambda ライブラリ**

`(scheme case-lambda)` ライブラリは， `case-lambda` 構文をエクスポートする。

```scheme
 case-lambda
```

**char ライブラリ**

`(scheme char)` ライブラリは， すべてのUnicode文字をサポートする場合，潜在的に大きなテーブルを伴う 文字を扱う手続きを提供する。

```scheme
 char-alphabetic?         char-ci<=?
 char-ci<?                char-ci=?
 char-ci>=?               char-ci>?
 char-downcase            char-foldcase
 char-lower-case?         char-numeric?
 char-upcase              char-upper-case?
 char-whitespace?         digit-value
 string-ci<=?             string-ci<?
 string-ci=?              string-ci>=?
 string-ci>?              string-downcase
 string-foldcase          string-upcase
```

**complex ライブラリ**

`(scheme complex)` ライブラリは， 通常非実数のみ有用な手続きをエクスポートする。

```scheme
 angle                    imag-part
 magnitude                make-polar
 make-rectangular         real-part
```

**CxR ライブラリ**

`(scheme cxr)` ライブラリは， 3から4つの `car` と `cdr` 操作の合成からなる 24個の手続きをエクスポートする。

```scheme
(define caddar
  (lambda (x) (car (cdr (cdr (car x)))))).
```

`car` と `cdr` 自身，およびこれら2つの合成からなる4個の手続きは， base ライブラリに含まれる。6.4節参照。

```scheme
 caaaar                   caaadr
 caaar                    caadar
 caaddr                   caadr
 cadaar                   cadadr
 cadar                    caddar
 cadddr                   caddr
 cdaaar                   cdaadr
 cdaar                    cdadar
 cdaddr                   cdadr
 cddaar                   cddadr
 cddar                    cdddar
 cddddr                   cdddr
```

**eval ライブラリ**

`(scheme eval)` ライブラリは， プログラムとして Scheme データを評価する手続きをエクスポートする。

```scheme
 environment              eval
```

**file ライブラリ**

`(scheme file)` ライブラリは， ファイルにアクセスするための手続きを提供する。

```scheme
 call-with-input-file     call-with-output-file
 delete-file              file-exists?
 open-binary-input-file   open-binary-output-file
 open-input-file          open-output-file
 with-input-from-file     with-output-to-file
```

**inexact ライブラリ**

`(scheme inexact)` ライブラリは， 通常不正確値のみ有用な手続きをエクスポートする。

```scheme
 acos                     asin
 atan                     cos
 exp                      finite?
 infinite?                log
 nan?                     sin
 sqrt                     tan
```

**lazy ライブラリ**

`(scheme lazy)` ライブラリは， 遅延評価のための手続きと構文キーワードをエクスポートする。

```scheme
 delay                    delay-force
 force                    make-promise
 promise?
```

**load ライブラリ**

`(scheme load)` ライブラリは， Scheme 式をファイルからロードする手続きをエクスポートする。

```scheme
 load
```

**process-context ライブラリ**

`(scheme process-context)` ライブラリは， プログラムの呼び出しコンテキストにアクセスする手続きをエクスポートする。

```scheme
 command-line             emergency-exit
 exit
 get-environment-variable
 get-environment-variables
```

**read ライブラリ**

`(scheme read)` ライブラリは， Scheme オブジェクトを読み込むための手続きを提供する。

```scheme
 read
```

**REPL ライブラリ**

`(scheme repl)` ライブラリは， `interaction-environment` 手続きをエクスポートする。

```scheme
 interaction-environment
```

**time ライブラリ**

`(scheme time)` ライブラリは， 時間に関連した値へのアクセスを提供する。

```scheme
 current-jiffy            current-second
 jiffies-per-second
```

**write ライブラリ**

`(scheme write)` ライブラリは， Scheme オブジェクトを書き込むための手続きを提供する。

```scheme
 display                  write
 write-shared             write-simple
```

**R5RS ライブラリ**

`(scheme r5rs)` ライブラリは， `transcript-on` と `transcript-off` が存在しないことを除いて， R<sup>5</sup>RSで定義された識別子を提供する。 `exact` および `inexact` 手続きは， それらは R<sup>5</sup>RSのもとでは，それぞれ 名前 `inexact->exact` および `exact->inexact` で現れる ことに注意せよ。 しかし，もし実装が複素数ライブラリのような特定のライブラリを提供しなければ， 対応する識別子はこのライブラリに現れなくなる。

```scheme
 *                        +
 -                        /
 <                        <=
 =                        >
 >=                       abs
 acos                     and
 angle                    append
 apply                    asin
 assoc                    assq
 assv                     atan
 begin                    boolean?
 caaaar                   caaadr
 caaar                    caadar
 caaddr                   caadr
 caar                     cadaar
 cadadr                   cadar
 caddar                   cadddr
 caddr                    cadr
 call-with-current-continuation
 call-with-input-file     call-with-output-file
 call-with-values         car
 case                     cdaaar
 cdaadr                   cdaar
 cdadar                   cdaddr
 cdadr                    cdar
 cddaar                   cddadr
 cddar                    cdddar
 cddddr                   cdddr
 cddr                     cdr
 ceiling                  char->integer
 char-alphabetic?         char-ci<=?
 char-ci<?                char-ci=?
 char-ci>=?               char-ci>?
 char-downcase            char-lower-case?
 char-numeric?            char-ready?
 char-upcase              char-upper-case?
 char-whitespace?         char<=?
 char<?                   char=?
 char>=?                  char>?
 char?                    close-input-port
 close-output-port        complex?
 cond                     cons
 cos                      current-input-port
 current-output-port      define
 define-syntax            delay
 denominator              display
 do                       dynamic-wind
 eof-object?              eq?
 equal?                   eqv?
 eval                     even?
 exact->inexact           exact?
 exp                      expt
 floor                    for-each
 force                    gcd
 if                       imag-part
 inexact->exact           inexact?
 input-port?              integer->char
 integer?                 interaction-environment
 lambda                   lcm
 length                   let
 let*                     let-syntax
 letrec                   letrec-syntax
 list                     list->string
 list->vector             list-ref
 list-tail                list?
 load                     log
 magnitude                make-polar
 make-rectangular         make-string
 make-vector              map
 max                      member
 memq                     memv
 min                      modulo
 negative?                newline
 not                      null-environment
 null?                    number->string
 number?                  numerator
 odd?                     open-input-file
 open-output-file         or
 output-port?             pair?
 peek-char                positive?
 procedure?               quasiquote
 quote                    quotient
 rational?                rationalize
 read                     read-char
 real-part                real?
 remainder                reverse
 round
 scheme-report-environment
 set!                     set-car!
 set-cdr!                 sin
 sqrt                     string
 string->list             string->number
 string->symbol           string-append
 string-ci<=?             string-ci<?
 string-ci=?              string-ci>=?
 string-ci>?              string-copy
 string-fill!             string-length
 string-ref               string-set!
 string<=?                string<?
 string=?                 string>=?
 string>?                 string?
 substring                symbol->string
 symbol?                  tan
 truncate                 values
 vector                   vector->list
 vector-fill!             vector-length
 vector-ref               vector-set!
 vector?                  with-input-from-file
 with-output-to-file      write
 write-char               zero?
```
