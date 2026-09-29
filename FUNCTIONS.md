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

## 実装予定の関数（TODO）

以下の関数は CLAUDE.md の TODO リストで実装予定です：

### 比較演算子
- `(> num1 num2 ...)`
- `(< num1 num2 ...)`
- `(= num1 num2 ...)`
- `(>= num1 num2 ...)`
- `(<= num1 num2 ...)`

### 制御フロー
- `(if condition then-expr else-expr)`
- `(cond (test1 expr1) (test2 expr2) ...)`
- `(case expr (key1 expr1) (key2 expr2) ...)`

### 変数バインディング
- `(define name value)`
- `(let ((var1 val1) (var2 val2)) body)`
- `(let* ((var1 val1) (var2 val2)) body)`
- `(letrec ((var1 val1) (var2 val2)) body)`

### ラムダ / 関数定義
- `(lambda (args) body)`
- `(define-syntax name transformer)`

### 基本的なリスト操作
- `(car lst)` - リストの先頭要素を返す
- `(cdr lst)` - リストの残りを返す
- `(cons elem lst)` - 要素をリストの先頭に追加
- `(append lst1 lst2 ...)` - リストを結合
- `(length lst)` - リストの要素数を返す
- `(map func lst)` - リストの各要素に関数を適用

### 型チェック
- `(integer? expr)`
- `(number? expr)`
- `(string? expr)`
- `(symbol? expr)`
- `(list? expr)`
- `(boolean? expr)`

## 型システム

rs-editor の Scheme インタプリタがサポートしている型：

- **Number**: 浮動小数点数（内部表現は f64）
- **String**: ダブルクォートで囲まれたテキスト
- **Symbol**: クォートなしの識別子
- **Boolean**: `#t` (true) / `#f` (false)
- **List**: S式形式のリスト `(elem1 elem2 ...)`

## 今後の拡張予定

詳細は `CLAUDE.md` の TODO と `TODO.md` のフェーズ分けを参照してください。

次のステップ：
1. 比較演算子と制御フロー（`if`, `cond`）の実装
2. 変数バインディングと定義（`define`, `let`）
3. より高度なリスト操作
4. ラムダ式とクロージャのサポート
