# Kilo エディタ

## 概要

Kilo は、Antirez によって作成された小さくて単純なテキストエディタです。C 言語で実装され、Unix ターミナル上で動作します。Kilo は教育的な目的で設計されており、"Build Your Own Text Editor" チュートリアルの基礎となっています。

このドキュメントは、Kilo エディタの機能とアーキテクチャについて説明します。Kilo は Scheme エディタ (RS-Editor) の開発に影響を与えており、特にターミナルベースのエディタの実装パターンに関して参考になります。

## 主な特徴

### コア機能
- **行編集**：複数行テキストの編集。
- **カーソル移動**：矢印キー、Ctrl+F/B/N/P による移動。
- **テキスト挿入**：キャラクタの入力と削除。
- **行番号表示**：左側に行番号を表示 (オプション)。
- **構文ハイライト**：C言語とその他の言語の基本的なシンタックスハイライト。
- **ファイル操作**：ファイルの読み込みと保存 (Ctrl+S)。
- **検索機能**：正規表現による検索と置換。
- **Quit**：Ctrl+Q でエディタを終了。

### シンプル設計
- **小さなコード**：約 1000 行の C コード。
- **最小限の依存**：標準ライブラリのみを使用。
- **モダンな機能**：行ごとの処理、ダーティフラグ、Undo/Redo。

## インストール

### 前提条件
- GCC または Clang C コンパイラ。
- POSIX 準拠の Unix ライク OS (Linux、macOS、BSD など)。
- termios ライブラリ (ほとんどの Unix システムに含まれます)。

### ビルド方法

```bash
git clone https://github.com/antirez/kilo.git
cd kilo
make
```

### 実行方法

```bash
# 新しいファイルを作成。
./kilo myfile.txt

# 既存ファイルを開く。
./kilo path/to/existing/file.txt
```

## キーバインディング

### ナビゲーション
| キー | 説明 |
|------|------|
| `Ctrl+F` | カーソル前進 (→) |
| `Ctrl+B` | カーソル後退 (←) |
| `Ctrl+N` | 次行へ移動 (↓) |
| `Ctrl+P` | 前行へ移動 (↑) |
| `Ctrl+A` | 行頭へ移動 |
| `Ctrl+E` | 行末へ移動 |
| `Ctrl+Home` | ファイルの先頭へ移動 |
| `Ctrl+End` | ファイルの末尾へ移動 |

### 編集
| キー | 説明 |
|------|------|
| `Ctrl+D` | カーソル位置の文字を削除 |
| `Backspace` | カーソル前の文字を削除 |
| `Ctrl+K` | 行末まで削除 (Kill Line) |
| `Ctrl+U` | 行頭から削除 (Kill to Line Start) |

### ファイル操作
| キー | 説明 |
|------|------|
| `Ctrl+S` | ファイルを保存 |
| `Ctrl+Q` | エディタを終了 |

### 検索
| キー | 説明 |
|------|------|
| `Ctrl+F` | 検索を開く (コンテキストに応じて) |
| `Enter` | 次のマッチへ移動 |
| `Escape` | 検索を閉じる |

## アーキテクチャ概要

### 主要コンポーネント

#### 1. ターミナル管理 (`enableRawMode()`、`disableRawMode()`)
- Raw モード：1 文字ずつ入力を取得するためにターミナルを設定。
- Canonical モード：デフォルトのバッファ入力モード。
- ANSI エスケープシーケンス：カーソル移動と表示制御に使用。

#### 2. エディタバッファ (`erow` 構造体)
- **行データ**：各行のテキストを保持。
- **行長**：行の実際の長さ。
- **表示用データ**：タブ展開やハイライト用の処理済みテキスト。

#### 3. 画面レンダリング (`editorRefreshScreen()`)
- スクロール機能：ビューポート内での行の表示。
- 行番号：左側に行番号を表示。
- ステータスバー：ファイル名とカーソル位置の表示。

#### 4. ファイルI/O (`editorOpen()`、`editorSave()`)
- ファイル読み込み：mmap() による効率的な読み込み。
- ファイル保存：部分的な保存とアトミックな書き込み。

#### 5. キー入力処理 (`editorProcessKeypress()`)
- キーシーケンス解析：矢印キーやコントロールシーケンスの処理。
- Emacs ライクなキーバインディング。

### データ構造

```c
// エディタ行の構造体。
typedef struct erow {
    int idx;           // 行のインデックス。
    int size;          // 行の長さ。
    int rsize;         // レンダリング用の長さ (タブ展開後)。
    char *chars;       // 行のテキスト。
    char *render;      // レンダリング用テキスト (タブ展開済み)。
    unsigned char *hl; // シンタックスハイライト情報。
    int hlopen;        // マルチライン文字列またはコメント状態。
} erow;

// エディタの状態。
struct editorConfig {
    int cx, cy;               // カーソル位置 (x, y)。
    int rowoff, coloff;       // スクロール位置。
    int screenrows, screencols; // 画面サイズ。
    int numrows;              // バッファ内の行数。
    erow *row;                // 行データの配列。
    int dirty;                // ファイル変更フラグ。
    char *filename;           // 現在のファイル名。
    char statusmsg[80];       // ステータスメッセージ。
    time_t statusmsg_time;    // ステータスメッセージの表示時間。
    struct editorSyntax *syntax; // 使用中のシンタックスハイライト。
};
```

## シンタックスハイライト

Kilo は複数の言語をサポートするシンタックスハイライトシステムを提供します。

### サポート言語
- **C**：キーワード、コメント、文字列。
- **Makefile**：キーワード、コメント。
- **その他**：拡張可能なシステム。

### ハイライトの仕組み
1. ファイル拡張子によって言語を判定。
2. 各行をスキャンして、キーワード、コメント、文字列を識別。
3. 識別されたトークンに対応する色を適用。

## 拡張ポイント

Kilo は以下の点で拡張が可能です。

### シンタックスハイライトの追加
- `editorSyntax` 配列に言語定義を追加。
- キーワード、ファイル拡張子、コメント記号を定義。

### キーバインディングのカスタマイズ
- `editorProcessKeypress()` 関数を修正。
- 新しいキーシーケンスを追加。

### ファイル保存形式の変更
- `editorSave()` 関数を修正。
- 異なるエンコーディングや形式に対応。

## 参考資料

### オリジナルリポジトリ
- GitHub：https://github.com/antirez/kilo

### チュートリアル
- Build Your Own Text Editor：https://viewsourcecode.org/snaptoken/kilo/

### 関連資料
- ANSI エスケープシーケンス：https://en.wikipedia.org/wiki/ANSI_escape_code
- termios API：https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/termios.h.html
- 正規表現ライブラリ：POSIX regex 関数 (regex.h)。

## ライセンス

Kilo は BSD 2 条項ライセンスの下で提供されています。

## 関連プロジェクト

### RS-Editor
このプロジェクトの Rust ベースの Scheme エディタは、Kilo の設計パターンに影響を受けています。特に：
- ターミナルベースの UI。
- Emacs ライクなキーバインディング。
- シンプルで拡張可能なアーキテクチャ。

### 他のテキストエディタ
- Vim：モダンで機能豊富なエディタ。
- GNU Emacs：高度にカスタマイズ可能。
- nano：初心者向けのシンプルなエディタ。
