手続き `integrate-system` は微分方程式の系
``` math
y_k^\prime = f_k(y_1, y_2, \ldots, y_n), \; k = 1, \ldots, n
```
を Runge-Kutta 法で積分する。

パラメタ `system-derivative` は系の状態 (状態変数 $`y_1, \ldots, 
y_n`$ に対する値からなるベクタ) を受け取って系の 微分係数 (値 $`y_1^\prime, \ldots, y_n^\prime`$) を算出する関数である。 パラメタ `initial-state` は系の初期状態を定める。 そして `h` は積分の刻み幅の初期推定値である。

`integrate-system` が返す値は，系の状態からなる無限ストリームである。

```
(define (integrate-system system-derivative
                          initial-state
                          h)
  (let ((next (runge-kutta-4 system-derivative h)))
    (letrec ((states
              (cons initial-state
                    (delay (map-streams next
                                        states)))))
      states)))
```

手続き `runge-kutta-4` は，系の状態から系の微分係数を算出する関数 `f` を 受け取る。 これは，系の状態を受け取って新しい系の状態を算出する関数を 算出する。

```
(define (runge-kutta-4 f h)
  (let ((*h (scale-vector h))
        (*2 (scale-vector 2))
        (*1/2 (scale-vector (/ 1 2)))
        (*1/6 (scale-vector (/ 1 6))))
    (lambda (y)
      ;; y is a system state
      (let* ((k0 (*h (f y)))
             (k1 (*h (f (add-vectors y (*1/2 k0)))))
             (k2 (*h (f (add-vectors y (*1/2 k1)))))
             (k3 (*h (f (add-vectors y k2)))))
        (add-vectors y
          (*1/6 (add-vectors k0
                             (*2 k1)
                             (*2 k2)
                             k3)))))))

(define (elementwise f)
  (lambda vectors
    (generate-vector
     (vector-length (car vectors))
     (lambda (i)
       (apply f
              (map (lambda (v) (vector-ref  v i))
                   vectors))))))

(define (generate-vector size proc)
  (let ((ans (make-vector size)))
    (letrec ((loop
              (lambda (i)
                (cond ((= i size) ans)
                      (else
                       (vector-set! ans i (proc i))
                       (loop (+ i 1)))))))
      (loop 0))))

(define add-vectors (elementwise +))

(define (scale-vector s)
  (elementwise (lambda (x) (* x s))))
```

`map-streams` 手続きは `map` に相当する: これはその第一引数 (手続き) を 第二引数 (ストリーム) のすべての要素に適用する。

```
(define (map-streams f s)
  (cons (f (head s))
        (delay (map-streams f (tail s)))))
```

無限ストリームは，そのストリームの最初の要素を car が保持し， そしてそのストリームの残りをかなえる約束を cdr が保持するペアとして 実装される。

```
(define head car)
(define (tail stream)
  (force (cdr stream)))
```

下記に示すのは減衰振動をモデル化した系
``` math
C {dv_C \over dt} = -i_L - {v_C \over R}
```
``` math
L {di_L \over dt} = v_C
```
の積分に `integrate-system` を応用した例である。

```
(define (damped-oscillator R L C)
  (lambda (state)
    (let ((Vc (vector-ref state 0))
          (Il (vector-ref state 1)))
      (vector (- 0 (+ (/ Vc (* R C)) (/ Il C)))
              (/ Vc L)))))

(define the-states
  (integrate-system
     (damped-oscillator 10000 1000 .001)
     '#(1 0)
     .01))
```
