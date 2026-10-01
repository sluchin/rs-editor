# Scheme 組み込み関数リスト

このファイルは、rs-editor に実装されている Scheme インタプリタの組み込み関数一覧をまとめたものです。

## 算術演算

### `(+ num1 num2 ...)`
複数の数値を加算します。

**例:**
```scheme
(+ 1 2 3)    ; => 6
(+ 10.5 0.5) ; => 11
(+)          ; => 0
```

### `(- num1 num2 ...)`
最初の数値から残りの数値を減算します。単一引数の場合は負数化します。

**例:**
```scheme
(- 10 3)     ; => 7
(- 5)        ; => -5
(- 100 20 10) ; => 70
```

### `(* num1 num2 ...)`
複数の数値を乗算します。

**例:**
```scheme
(* 2 3 4)    ; => 24
(* 5 0)      ; => 0
(*)          ; => 1
```

### `(/ num1 num2 ...)`
最初の数値を残りの数値で順番に除算します。ゼロ除算はエラーになります。

**例:**
```scheme
(/ 20 4)     ; => 5
(/ 100 2 5)  ; => 10
(/ 1 0)      ; => エラー: 0で除算することはできません
```

## リスト操作

### `(quote expr)`
式を評価せずにそのまま返します。リテラルとしてリストやシンボルを取得するのに使用します。

**例:**
```scheme
(quote (+ 1 2))  ; => (+ 1 2)
(quote hello)    ; => hello
'(1 2 3)         ; => (1 2 3) [クォート記法]
```

### `(list elem1 elem2 ...)`
与えられた要素からリストを構築します。各要素は評価されます。

**例:**
```scheme
(list 1 2 3)              ; => (1 2 3)
(list (+ 1 1) (+ 2 1))   ; => (2 3)
(list)                    ; => ()
```

## 比較演算

`(= a b ...)`, `(< a b ...)`, `(> a b ...)`, `(<= a b ...)`, `(>= a b ...)` — 隣り合う引数すべてが条件を満たすと `#t`。

## その他の数値関数

`quotient`, `remainder`, `modulo`, `abs`, `min`, `max`, `zero?`, `even?`, `odd?`

## 論理・等価

`not`, `eq?`, `eqv?`, `equal?`（いずれも値の構造的な等価比較）

## 型述語

`number?`, `integer?`, `string?`, `symbol?`, `boolean?`, `list?`, `pair?`, `null?`, `procedure?`

## リスト操作（追加分）

`car`, `cdr`, `cons`（第 2 引数はリスト）, `append`, `length`, `reverse`, `list-ref`,
`map`, `for-each`, `filter`, `fold`（`(fold f init lst)`、`f` は `(elem acc)`）, `reduce`, `apply`

## 文字列・入出力

`string-append`, `string-length`, `number->string`, `display`（引用符なしで出力）, `newline`

## 特殊形式

`quote`（`'x`）, `if`, `cond`（`else`）, `case`, `and`, `or`, `when`, `unless`,
`define`（`(define (f . args) ...)` 形式を含む）, `set!`, `lambda`, `begin`,
`let`, `let*`, `letrec`, 名前付き `let`。クロージャと末尾呼び出し最適化に対応。

## 未実装

`define-syntax`, quasiquote, `do`, ドット対, 文字型, ベクタなど。一覧は `CLAUDE.md` の TODO を参照。

## 型システム

rs-editor の Scheme インタプリタがサポートしている型：

- **Number**: 浮動小数点数（内部表現は f64）
- **String**: ダブルクォートで囲まれたテキスト
- **Symbol**: クォートなしの識別子
- **Boolean**: `#t` (true) / `#f` (false)
- **List**: S式形式のリスト `(elem1 elem2 ...)`（ドット対は未対応）
- **Procedure**: 組み込み関数とクロージャ

## 今後の拡張予定

詳細は `CLAUDE.md` の TODO と `TODO.md` のフェーズ分けを参照してください。
