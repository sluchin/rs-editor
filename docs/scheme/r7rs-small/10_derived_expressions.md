# 派生式型

本節は `quasiquote` を除く原始式型 (リテラル，変数，呼出し，`lambda`， `if`, `set!`) による派生式型の構文定義を与える。

条件付き派生構文型:

```scheme
(define-syntax cond
  (syntax-rules (else =>)
    ((cond (else result1 result2 ...))
     (begin result1 result2 ...))
    ((cond (test => result))
     (let ((temp test))
       (if temp (result temp))))
    ((cond (test => result) clause1 clause2 ...)
     (let ((temp test))
       (if temp
           (result temp)
           (cond clause1 clause2 ...))))
    ((cond (test)) test)
    ((cond (test) clause1 clause2 ...)
     (let ((temp test))
       (if temp
           temp
           (cond clause1 clause2 ...))))
    ((cond (test result1 result2 ...))
     (if test (begin result1 result2 ...)))
    ((cond (test result1 result2 ...)
           clause1 clause2 ...)
     (if test
         (begin result1 result2 ...)
         (cond clause1 clause2 ...)))))
```

```scheme
(define-syntax case
  (syntax-rules (else =>)
    ((case (key ...)
       clauses ...)
     (let ((atom-key (key ...)))
       (case atom-key clauses ...)))
    ((case key
       (else => result))
     (result key))
    ((case key
       (else result1 result2 ...))
     (begin result1 result2 ...))
    ((case key
       ((atoms ...) result1 result2 ...))
     (if (memv key '(atoms ...))
         (begin result1 result2 ...)))
    ((case key
       ((atoms ...) => result))
     (if (memv key '(atoms ...))
         (result key)))
    ((case key
       ((atoms ...) => result)
       clause clauses ...)
     (if (memv key '(atoms ...))
         (result key)
         (case key clause clauses ...)))
    ((case key
       ((atoms ...) result1 result2 ...)
       clause clauses ...)
     (if (memv key '(atoms ...))
         (begin result1 result2 ...)
         (case key clause clauses ...)))))
```

```scheme
(define-syntax and
  (syntax-rules ()
    ((and) #t)
    ((and test) test)
    ((and test1 test2 ...)
     (if test1 (and test2 ...) #f))))
```

```scheme
(define-syntax or
  (syntax-rules ()
    ((or) #f)
    ((or test) test)
    ((or test1 test2 ...)
     (let ((x test1))
       (if x x (or test2 ...))))))
```

```scheme
(define-syntax when
  (syntax-rules ()
    ((when test result1 result2 ...)
     (if test
         (begin result1 result2 ...)))))
```

```scheme
(define-syntax unless
  (syntax-rules ()
    ((unless test result1 result2 ...)
     (if (not test)
         (begin result1 result2 ...)))))
```

束縛コンストラクト:

```scheme
(define-syntax let
  (syntax-rules ()
    ((let ((name val) ...) body1 body2 ...)
     ((lambda (name ...) body1 body2 ...)
      val ...))
    ((let tag ((name val) ...) body1 body2 ...)
     ((letrec ((tag (lambda (name ...)
                      body1 body2 ...)))
        tag)
      val ...))))
```

```scheme
(define-syntax let*
  (syntax-rules ()
    ((let* () body1 body2 ...)
     (let () body1 body2 ...))
    ((let* ((name1 val1) (name2 val2) ...)
       body1 body2 ...)
     (let ((name1 val1))
       (let* ((name2 val2) ...)
         body1 body2 ...)))))
```

下記の `letrec` マクロは，なにか場所に格納され，その場所に 格納された値を取得しようとするとエラーになってしまうものを返す式の代わりとして， シンボル `<undefined>` を使っている。 (このような式は Scheme では定義されない。) 値が評価される順序を規定することを避けるために必要な一時変数を生成するため， あるトリックが使われている。 これは補助的なマクロを使用することによっても達成できただろう。

```scheme
(define-syntax letrec
  (syntax-rules ()
    ((letrec ((var1 init1) ...) body ...)
     (letrec "generate_temp_names"
       (var1 ...)
       ()
       ((var1 init1) ...)
       body ...))
    ((letrec "generate_temp_names"
       ()
       (temp1 ...)
       ((var1 init1) ...)
       body ...)
     (let ((var1 <undefined>) ...)
       (let ((temp1 init1) ...)
         (set! var1 temp1)
         ...
         body ...)))
    ((letrec "generate_temp_names"
       (x y ...)
       (temp ...)
       ((var1 init1) ...)
       body ...)
     (letrec "generate_temp_names"
       (y ...)
       (newtemp temp ...)
       ((var1 init1) ...)
       body ...))))
```

```scheme
(define-syntax letrec*
  (syntax-rules ()
    ((letrec* ((var1 init1) ...) body1 body2 ...)
     (let ((var1 <undefined>) ...)
       (set! var1 init1)
       ...
       (let () body1 body2 ...)))))
```

```scheme
(define-syntax let-values
  (syntax-rules ()
    ((let-values (binding ...) body0 body1 ...)
     (let-values "bind"
         (binding ...) () (begin body0 body1 ...)))

    ((let-values "bind" () tmps body)
     (let tmps body))

    ((let-values "bind" ((b0 e0)
         binding ...) tmps body)
     (let-values "mktmp" b0 e0 ()
         (binding ...) tmps body))

    ((let-values "mktmp" () e0 args
         bindings tmps body)
     (call-with-values
       (lambda () e0)
       (lambda args
         (let-values "bind"
             bindings tmps body))))

    ((let-values "mktmp" (a . b) e0 (arg ...)
         bindings (tmp ...) body)
     (let-values "mktmp" b e0 (arg ... x)
         bindings (tmp ... (a x)) body))

    ((let-values "mktmp" a e0 (arg ...)
        bindings (tmp ...) body)
     (call-with-values
       (lambda () e0)
       (lambda (arg ... . x)
         (let-values "bind"
             bindings (tmp ... (a x)) body))))))
```

```scheme
(define-syntax let*-values
  (syntax-rules ()
    ((let*-values () body0 body1 ...)
     (let () body0 body1 ...))

    ((let*-values (binding0 binding1 ...)
         body0 body1 ...)
     (let-values (binding0)
       (let*-values (binding1 ...)
         body0 body1 ...)))))
```

```scheme
(define-syntax define-values
  (syntax-rules ()
    ((define-values () expr)
     (define dummy
       (call-with-values (lambda () expr)
                         (lambda args #f))))
    ((define-values (var) expr)
     (define var expr))
    ((define-values (var0 var1 ... varn) expr)
     (begin
       (define var0
         (call-with-values (lambda () expr)
                           list))
       (define var1
         (let ((v (cadr var0)))
           (set-cdr! var0 (cddr var0))
           v)) ...
       (define varn
         (let ((v (cadr var0)))
           (set! var0 (car var0))
           v))))
    ((define-values (var0 var1 ... . varn) expr)
     (begin
       (define var0
         (call-with-values (lambda () expr)
                           list))
       (define var1
         (let ((v (cadr var0)))
           (set-cdr! var0 (cddr var0))
           v)) ...
       (define varn
         (let ((v (cdr var0)))
           (set! var0 (car var0))
           v))))
    ((define-values var expr)
     (define var
       (call-with-values (lambda () expr)
                         list)))))
```

```scheme
(define-syntax begin
  (syntax-rules ()
    ((begin exp ...)
     ((lambda () exp ...)))))
```

下記の代替的な `begin` の展開は，ラムダ式の本体に複数の式を書けるという能力を 利用していない。 いずれにせよ，これらの規則が適用できるのは， `begin` の本体に 定義が全く含まれていないときだけであることに注意せよ。

```scheme
(define-syntax begin
  (syntax-rules ()
    ((begin exp)
     exp)
    ((begin exp1 exp2 ...)
     (call-with-values
         (lambda () exp1)
       (lambda args
         (begin exp2 ...))))))
```

下記の `do` の構文定義は変数節を展開するためにあるトリックを使っている。 上記の `letrec` と同じく，補助マクロでもよかっただろう。 式 `(if #f #f)` は未規定の値を得るために使われている。

```scheme
(define-syntax do
  (syntax-rules ()
    ((do ((var init step ...) ...)
         (test expr ...)
         command ...)
     (letrec
       ((loop
         (lambda (var ...)
           (if test
               (begin
                 (if #f #f)
                 expr ...)
               (begin
                 command
                 ...
                 (loop (do "step" var step ...)
                       ...))))))
       (loop init ...)))
    ((do "step" x)
     x)
    ((do "step" x y)
     y)))
```

ここで `delay`, `force` ならびに `delay-force` の可能な実装がある。 我々は式

```scheme
(delay-force <式>)
```

を次の手続き呼出しと同じ意味をもつものとして定義する:

```scheme
(make-promise #f (lambda () <式>))
```

すなわち，次のように定義する:

```scheme
(define-syntax delay-force
  (syntax-rules ()
    ((delay-force expression)
     (make-promise #f (lambda () expression)))))
```

そして式

```scheme
(delay <式>)
```

を次と同じ意味をもつものとして定義する:

```scheme
(delay-force (make-promise #t <式>))
```

すなわち，次のように定義する:

```scheme
(define-syntax delay
  (syntax-rules ()
    ((delay expression)
     (delay-force (make-promise #t expression)))))
```

ここで `make-promise` は次のように定義される:

```scheme
(define make-promise
  (lambda (done? proc)
    (list (cons done? proc))))
```

最後に，非遅延結果 (すなわち， `delay-force` の代わりに `delay` によって生成された値) が返されるまで次のトランポリン技術 を用いて 約束の中の手続き式を繰り返し呼び出すために， `force` を次のように定義する:

```scheme
(define (force promise)
  (if (promise-done? promise)
      (promise-value promise)
      (let ((promise* ((promise-value promise))))
        (unless (promise-done? promise)
          (promise-update! promise* promise))
        (force promise))))
```

次の約束アクセサによって:

```scheme
(define promise-done?
  (lambda (x) (car (car x))))
(define promise-value
  (lambda (x) (cdr (car x))))
(define promise-update!
  (lambda (new old)
    (set-car! (car old) (promise-done? new))
    (set-cdr! (car old) (promise-value new))
    (set-car! new (car old))))
```

次の `make-parameter` および `parameterize` の実装は スレッドなしの実装にとって適切である。 パラメタオブジェクトは， 二つの任意の異なるオブジェクト`<param-set!>` と `<param-convert>`を用いて ここでは手続きとして実装される:

```scheme
(define (make-parameter init . o)
  (let* ((converter
          (if (pair? o) (car o) (lambda (x) x)))
         (value (converter init)))
    (lambda args
      (cond
       ((null? args)
        value)
       ((eq? (car args) <param-set!>)
        (set! value (cadr args)))
       ((eq? (car args) <param-convert>)
        converter)
       (else
        (error "bad parameter syntax"))))))
```

このとき `parameterize` は， 関連付けられた値を動的に再束縛するために `dynamic-wind` を用いる:

```scheme
(define-syntax parameterize
  (syntax-rules ()
    ((parameterize ("step")
                   ((param value p old new) ...)
                   ()
                   body)
     (let ((p param) ...)
       (let ((old (p)) ...
             (new ((p <param-convert>) value)) ...)
         (dynamic-wind
          (lambda () (p <param-set!> new) ...)
          (lambda () . body)
          (lambda () (p <param-set!> old) ...)))))
    ((parameterize ("step")
                   args
                   ((param value) . rest)
                   body)
     (parameterize ("step")
                   ((param value p old new) . args)
                   rest
                   body))
    ((parameterize ((param value) ...) . body)
     (parameterize ("step")
                   ()
                   ((param value) ...)
                   body))))
```

`guard` の次の実装は，ここで呼ばれる補助的なマクロ `guard-aux` に依存している。

```scheme
(define-syntax guard
  (syntax-rules ()
    ((guard (var clause ...) e1 e2 ...)
     ((call/cc
       (lambda (guard-k)
         (with-exception-handler
          (lambda (condition)
            ((call/cc
               (lambda (handler-k)
                 (guard-k
                  (lambda ()
                    (let ((var condition))
                      (guard-aux
                        (handler-k
                          (lambda ()
                            (raise-continuable condition)))
                        clause ...))))))))
          (lambda ()
            (call-with-values
             (lambda () e1 e2 ...)
             (lambda args
               (guard-k
                 (lambda ()
                   (apply values args)))))))))))))

(define-syntax guard-aux
  (syntax-rules (else =>)
    ((guard-aux reraise (else result1 result2 ...))
     (begin result1 result2 ...))
    ((guard-aux reraise (test => result))
     (let ((temp test))
       (if temp
           (result temp)
           reraise)))
    ((guard-aux reraise (test => result)
                clause1 clause2 ...)
     (let ((temp test))
       (if temp
           (result temp)
           (guard-aux reraise clause1 clause2 ...))))
    ((guard-aux reraise (test))
     (or test reraise))
    ((guard-aux reraise (test) clause1 clause2 ...)
     (let ((temp test))
       (if temp
           temp
           (guard-aux reraise clause1 clause2 ...))))
    ((guard-aux reraise (test result1 result2 ...))
     (if test
         (begin result1 result2 ...)
         reraise))
    ((guard-aux reraise
                (test result1 result2 ...)
                clause1 clause2 ...)
     (if test
         (begin result1 result2 ...)
         (guard-aux reraise clause1 clause2 ...)))))
```

```scheme
(define-syntax case-lambda
  (syntax-rules ()
    ((case-lambda (params body0 ...) ...)
     (lambda args
       (let ((len (length args)))
         (let-syntax
             ((cl (syntax-rules ::: ()
                    ((cl)
                     (error "no matching clause"))
                    ((cl ((p :::) . body) . rest)
                     (if (= len (length '(p :::)))
                         (apply (lambda (p :::)
                                  . body)
                                args)
                         (cl . rest)))
                    ((cl ((p ::: . tail) . body)
                         . rest)
                     (if (>= len (length '(p :::)))
                         (apply
                          (lambda (p ::: . tail)
                            . body)
                          args)
                         (cl . rest))))))
           (cl (params body0 ...) ...)))))))
```

`cond-expand` の定義は， `features` 手続きと対話しない。 それは実装で提供される各機能識別子が明示的に言及されることを要求する。

```scheme
(define-syntax cond-expand
  ;; すべての機能 ID とライブラリを記載するため，これを拡張する
  (syntax-rules (and or not else r7rs library scheme base)
    ((cond-expand)
     (syntax-error "Unfulfilled cond-expand"))
    ((cond-expand (else body ...))
     (begin body ...))
    ((cond-expand ((and) body ...) more-clauses ...)
     (begin body ...))
    ((cond-expand ((and req1 req2 ...) body ...)
                  more-clauses ...)
     (cond-expand
       (req1
         (cond-expand
           ((and req2 ...) body ...)
           more-clauses ...))
       more-clauses ...))
    ((cond-expand ((or) body ...) more-clauses ...)
     (cond-expand more-clauses ...))
    ((cond-expand ((or req1 req2 ...) body ...)
                  more-clauses ...)
     (cond-expand
       (req1
        (begin body ...))
       (else
        (cond-expand
           ((or req2 ...) body ...)
           more-clauses ...))))
    ((cond-expand ((not req) body ...)
                  more-clauses ...)
     (cond-expand
       (req
         (cond-expand more-clauses ...))
       (else body ...)))
    ((cond-expand (r7rs body ...)
                  more-clauses ...)
       (begin body ...))
    ;; サポートされている各機能識別子のため，
    ;; ここで句を追加する。
    ;; 例:
    ;; ((cond-expand (exact-closed body ...)
    ;;               more-clauses ...)
    ;;   (begin body ...))
    ;; ((cond-expand (ieee-float body ...)
    ;;               more-clauses ...)
    ;;   (begin body ...))
    ((cond-expand ((library (scheme base))
                   body ...)
                  more-clauses ...)
      (begin body ...))
    ;; 各ライブラリのため，ここで句を追加する
    ((cond-expand (feature-id body ...)
                  more-clauses ...)
       (cond-expand more-clauses ...))
    ((cond-expand ((library (name ...))
                   body ...)
                  more-clauses ...)
       (cond-expand more-clauses ...))))
```
