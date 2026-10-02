# Kilo エディタ - アーキテクチャガイド

このドキュメントは、Kilo エディタの内部構造と実装パターンについて詳しく説明します。

## 全体的な設計哲学

Kilo は以下の原則に従って設計されています。

### 1. シンプリシティ (単純性)
- 最小限の抽象化。
- 理解しやすいコード。
- 必要な機能のみを実装。

### 2. 自己完結性
- 外部ライブラリに依存しない (C 標準ライブラリのみ)。
- すべての機能が単一ファイルに統合。

### 3. 教育的価値
- コードが学習教材として機能。
- 各機能が明確に分離。
- よくコメント化されたコード。

## モジュール構成

Kilo は大まかに以下のモジュールに分かれています。

### 1. ターミナル制御モジュール

#### Raw モードの設定
```c
void enableRawMode(int fd);
void disableRawMode(int fd);
```

**機能：**
- `enableRawMode()`：ターミナルを Raw モードに切り替え。
  - キャノニカルモード (行ごとのバッファリング) を無効化。
  - エコーバック (入力の自動表示) を無効化。
  - シグナル文字 (Ctrl+C など) の特別な処理を無効化。

- `disableRawMode()`：元のターミナル設定に復元。

**使用する termios フラグ：**
| フラグ | 説明 |
|--------|------|
| `BRKINT` | Break 条件を SIGINT に変換。 |
| `ICRNL` | CR を NL に変換 (無効化)。 |
| `INPCK` | パリティチェック (無効化)。 |
| `ISTRIP` | 入力の最上位ビットをクリア (無効化)。 |
| `IXON` | Ctrl+S/Q (フロー制御) を無効化。 |
| `OPOST` | 出力処理を無効化。 |
| `CS8` | キャラクタサイズを 8 ビットに設定。 |
| `ECHO` | 入力エコーを無効化。 |
| `ICANON` | キャノニカルモードを無効化。 |
| `ISIG` | シグナル生成を無効化。 |
| `IEXTEN` | 実装固有の入力処理を無効化。 |

#### ANSI エスケープシーケンス
Kilo は ANSI エスケープシーケンスを使用してカーソルと画面を制御します。

**主要なシーケンス：**
| シーケンス | 機能 |
|-----------|------|
| `\x1b[H` | カーソルをホーム (左上) に移動。 |
| `\x1b[2J` | 画面全体をクリア。 |
| `\x1b[K` | カーソルから行末まで削除 (EL)。 |
| `\x1b[{rows};{cols}H` | カーソルを指定位置に移動。 |
| `\x1b[7m` | 反転表示開始 (ステータスバー用)。 |
| `\x1b[m` | テキスト属性をリセット。 |
| `\x1b[{color}m` | 色を設定 (シンタックスハイライト用)。 |

### 2. エディタバッファモジュール

#### 行構造体
```c
typedef struct erow {
    int idx;           // この行のファイル内でのインデックス。
    int size;          // chars の実際の長さ。
    int rsize;         // render の実際の長さ (タブ展開後)。
    char *chars;       // 実際の行テキスト。
    char *render;      // 表示用の処理済みテキスト。
    unsigned char *hl; // シンタックスハイライト情報。
    int hlopen;        // マルチライン文字列/コメントの状態。
} erow;
```

**データ管理：**
- `chars`：ファイルの実際のテキスト (タブは 1 文字)。
- `render`：表示用のテキスト (タブはスペースに展開)。
- `hl`：各文字のハイライトタイプ。

#### 行の追加と削除
```c
void editorInsertRow(int at, char *s, size_t len);
void editorDelRow(int at);
void editorRowInsertChar(erow *row, int at, int c);
void editorRowAppendString(erow *row, char *s, size_t len);
void editorRowDelChar(erow *row, int at);
```

### 3. 画面レンダリングモジュール

#### スクロール管理
```c
void editorScroll(void);
```

**ロジック：**
- カーソルが画面の上に移動した場合、ビューをスクロールアップ。
- カーソルが画面の下に移動した場合、ビューをスクロールダウン。
- カーソルが左に移動した場合、ビューを左スクロール。
- カーソルが右に移動した場合、ビューを右スクロール。

#### 行のレンダリング
```c
void editorUpdateRow(erow *row);
```

**処理：**
1. 行テキストをスキャン。
2. タブをスペースに展開 (デフォルト 8 スペース)。
3. 非表示文字 (制御文字) を処理。
4. シンタックスハイライト情報を生成。

#### 画面の更新
```c
void editorRefreshScreen(void);
```

**処理ステップ：**
1. ANSI エスケープシーケンス "カーソルを隠す" を送信。
2. ビューポート内の各行をレンダリング。
3. ステータスバーを描画。
4. メッセージ行を描画 (存在する場合)。
5. カーソルを正しい位置に配置。
6. ANSI エスケープシーケンス "カーソルを表示" を送信。

### 4. ファイル I/O モジュール

#### ファイルの読み込み
```c
void editorOpen(char *filename);
```

**処理：**
1. ファイルをオープン。
2. 最初に各行の長さをスキャン (バッファサイズを決定)。
3. ファイルを行ごとに読み込み。
4. 各行をエディタバッファに追加。
5. ファイルをクローズ。

#### ファイルの保存
```c
char *editorRowsToString(int *buflen);
void editorSave(void);
```

**処理：**
1. すべての行をメモリ内にコンカテナした。
2. 一時ファイルに書き込み (原子的な操作)。
3. 一時ファイルを実際のファイル名にリネーム。
4. ダーティフラグをクリア。

### 5. キー入力処理モジュール

#### キー読み込み
```c
int editorReadKey(void);
```

**機能：**
- 単一キーを読み込み。
- エスケープシーケンス (矢印キーなど) を解析。
- 複合キー (Ctrl+X など) を処理。

#### キー処理
```c
void editorProcessKeypress(void);
```

**サポートするキー：**
- 通常文字：バッファに挿入。
- Ctrl+F：検索を開く。
- Ctrl+S：ファイルを保存。
- Ctrl+Q：エディタを終了。
- 矢印キー：カーソルを移動。
- Delete/Backspace：文字を削除。

### 6. シンタックスハイライトモジュール

#### ハイライトの種類
```c
#define HL_NORMAL 0       // デフォルト色。
#define HL_COMMENT 1      // コメント。
#define HL_MLCOMMENT 2    // マルチラインコメント。
#define HL_KEYWORD1 3     // 言語キーワード。
#define HL_KEYWORD2 4     // 言語キーワード (異なるグループ)。
#define HL_STRING 5       // 文字列リテラル。
#define HL_NUMBER 6       // 数字リテラル。
#define HL_MATCH 7        // 検索マッチハイライト。
```

#### 言語定義
```c
struct editorSyntax {
    char *filetype;           // 言語名。
    char **filematch;         // ファイルパターンマッチング。
    char **keywords;          // キーワードリスト。
    char *singleline_comment_start;  // 単一行コメント開始。
    char *multiline_comment_start;   // マルチラインコメント開始。
    char *multiline_comment_end;     // マルチラインコメント終了。
    int flags;                // オプションフラグ。
};
```

#### ハイライト処理
```c
void editorUpdateSyntax(erow *row);
```

**処理：**
1. 前の行の状態から開始 (マルチラインコメント対応)。
2. 行をスキャン。
3. キーワード、コメント、文字列を識別。
4. 各文字のハイライトタイプを設定。

## 実行フロー

### 初期化フェーズ
```
main()
  ├── termios 設定の保存
  ├── enableRawMode()
  ├── editorOpen(argv[1]) (ファイル名が指定された場合)
  ├── editorSelectSyntaxHighlight()
  └── エディタループへ
```

### メインループ
```
while (1) {
    editorRefreshScreen()      // 画面を再描画。
    editorProcessKeypress()    // キー入力を処理。
}
```

### 終了フェーズ
```
cleanup on exit:
  ├── disableRawMode()
  ├── メモリ解放
  └── ターミナル復元
```

## パフォーマンス考慮事項

### メモリ効率
- **バッファ管理**：各行を個別にメモリ割り当て。
- **遅延評価**：シンタックスハイライトは表示される行のみ処理。

### 画面更新の最適化
- **差分更新**：全画面クリアではなく、必要な部分のみ更新。
- **ANSI エスケープシーケンス**：効率的なカーソル制御。

### キー入力処理
- **ノンブロッキング**：入力がない場合でも画面更新。
- **タイムアウト**：ステータスメッセージの自動消去。

## 拡張例

### 新しいキーバインディングの追加
```c
// editorProcessKeypress() 内に追加。
case CTRL_H:  // Ctrl+H で検索。
    editorFind();
    break;
```

### 新しい言語のシンタックスハイライト追加
```c
struct editorSyntax HLDB[] = {
    {
        "c",
        {"*.c", "*.h", NULL},
        keywords,
        "//", "/*", "*/",
        HL_HIGHLIGHT_STRINGS | HL_HIGHLIGHT_NUMBERS
    },
    // … 他の言語定義 …
};
```

### Undo/Redo の実装
```c
// 編集履歴を保持する構造体。
typedef struct {
    char *before;      // 変更前のテキスト。
    char *after;       // 変更後のテキスト。
} UndoRecord;

// Undo/Redo スタック。
UndoRecord *undo_stack;
int undo_ptr = 0;
```

## デバッグのコツ

### ログ出力
```c
// エディタウィンドウ外の領域を使用してデバッグ情報を表示。
FILE *fp = fopen("/tmp/kilo_debug.log", "a");
fprintf(fp, "debug info\n");
fclose(fp);
```

### GDB での調査
```bash
gdb ./kilo
(gdb) break editorRefreshScreen
(gdb) run myfile.txt
(gdb) next
(gdb) print E.screenrows
```

## 結論

Kilo のアーキテクチャは、シンプルさと機能のバランスを取ることの例です。ターミナル制御、バッファ管理、画面レンダリングといった基本的な要素を理解することで、より高度なテキストエディタの実装を学習できます。
