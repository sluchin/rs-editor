# Scheme マニュアル

rs-editor に組み込まれた Scheme インタプリタのマニュアルです。

## 目次

1. [はじめに](introduction.md)
   - Scheme について
   - 基本的な概念

2. [構文](syntax.md)
   - リスト記法
   - クォート
   - コメント
   - 数値・文字列・シンボル

3. [データ型](data-types.md)
   - 数値型
   - 文字列
   - シンボル
   - ブール値
   - リスト

4. [組み込み手続き](builtin-procedures.md)
   - 算術演算
   - リスト操作
   - その他のユーティリティ

5. [実例とチュートリアル](examples.md)
   - 基本的な計算
   - リスト処理
   - より複雑な例

## クイックスタート

rs-editor の REPL で Scheme コードを評価できます。

### 基本的な計算

```scheme
(+ 1 2 3)      ; => 6
(* 10 20)      ; => 200
(- 100 30)     ; => 70
```

### リスト処理

```scheme
(quote (1 2 3))  ; => (1 2 3)
(list 1 2 3)     ; => (1 2 3)
```

## 注記

このマニュアルは rs-editor に実装された Scheme インタプリタの使用方法を説明します。
完全な R5RS / R6RS 標準仕様ではなく、限定された機能セットを提供しています。

詳細な標準仕様は以下を参照してください：
- [R6RS: Scheme Language](https://www.r6rs.org/)
- [R5RS: Revised^5 Report on the Algorithmic Language Scheme](https://schemers.org/Documents/Standards/R5RS/)
