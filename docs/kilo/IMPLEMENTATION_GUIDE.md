# Kilo - 実装ガイド

このドキュメントは, Kilo エディタを学習し, 同様のエディタを実装するための実践的なガイドです.

## 段階的な実装戦略

### フェーズ 1: 基本的なターミナル制御 (ステップ 1-10)

#### 1. ターミナルの Raw モード設定
```c
#include <unistd.h>
#include <termios.h>

struct termios orig_termios;

void enableRawMode() {
    tcgetattr(STDIN_FILENO, &orig_termios);
    struct termios raw = orig_termios;
    raw.c_lflag &= ~(ECHO | ICANON);
    tcsetattr(STDIN_FILENO, TCSAFLUSH, &raw);
}

void disableRawMode() {
    tcsetattr(STDIN_FILENO, TCSAFLUSH, &orig_termios);
}
```

#### 2. キー入力の読み込み
```c
int main() {
    enableRawMode();
    atexit(disableRawMode);
    
    while (1) {
        char c;
        if (read(STDIN_FILENO, &c, 1) == -1 && errno != EAGAIN)
            perror("read");
        
        if (c == 'q') break;
        
        printf("char: %d\n", c);
    }
}
```

#### 3. 画面クリアと ANSI シーケンス
```c
void refreshScreen() {
    write(STDOUT_FILENO, "\x1b[2J", 4);      // 画面クリア.
    write(STDOUT_FILENO, "\x1b[H", 3);       // カーソルをホームに.
}
```

### フェーズ 2: テキスト編集 (ステップ 11-30)

#### 1. 行データ構造
```c
typedef struct {
    int size;
    char *chars;
} erow;

typedef struct {
    int numrows;
    erow *row;
} EditorConfig;
```

#### 2. 行の挿入・削除
```c
void insertRow(int at, char *s, size_t len) {
    E.row = realloc(E.row, sizeof(erow) * (E.numrows + 1));
    E.row[at].chars = malloc(len + 1);
    memcpy(E.row[at].chars, s, len);
    E.row[at].size = len;
    E.numrows++;
}

void deleteRow(int at) {
    free(E.row[at].chars);
    memmove(&E.row[at], &E.row[at + 1], 
            sizeof(erow) * (E.numrows - at - 1));
    E.numrows--;
}
```

#### 3. キャラクタの挿入・削除
```c
void insertChar(int c) {
    if (E.cy == E.numrows) {
        insertRow(E.numrows, "", 0);
    }
    
    erow *row = &E.row[E.cy];
    row->chars = realloc(row->chars, row->size + 2);
    memmove(&row->chars[E.cx + 1], &row->chars[E.cx],
            row->size - E.cx + 1);
    row->chars[E.cx] = c;
    row->size++;
    E.cx++;
}
```

### フェーズ 3: ファイル I/O (ステップ 31-45)

#### 1. ファイルの読み込み
```c
void openFile(char *filename) {
    E.filename = strdup(filename);
    
    FILE *fp = fopen(filename, "r");
    if (!fp) {
        if (errno != ENOENT) perror("fopen");
        return;
    }
    
    char *line = NULL;
    size_t linecap = 0;
    ssize_t linelen;
    
    while ((linelen = getline(&line, &linecap, fp)) != -1) {
        while (linelen > 0 && 
               (line[linelen - 1] == '\n' || line[linelen - 1] == '\r'))
            linelen--;
        
        insertRow(E.numrows, line, linelen);
    }
    
    free(line);
    fclose(fp);
}
```

#### 2. ファイルの保存
```c
char *rowsToString(int *buflen) {
    int totlen = 0;
    int j;
    
    for (j = 0; j < E.numrows; j++)
        totlen += E.row[j].size + 1;  // +1 改行用.
    
    *buflen = totlen;
    char *buf = malloc(totlen);
    char *p = buf;
    
    for (j = 0; j < E.numrows; j++) {
        memcpy(p, E.row[j].chars, E.row[j].size);
        p += E.row[j].size;
        *p = '\n';
        p++;
    }
    
    return buf;
}

void saveFile() {
    if (!E.filename) return;
    
    int len;
    char *buf = rowsToString(&len);
    
    // 一時ファイルに書き込み.
    int fd = open(E.filename, O_RDWR | O_CREAT, 0644);
    if (fd != -1) {
        if (ftruncate(fd, len) != -1) {
            if (write(fd, buf, len) == len) {
                close(fd);
                free(buf);
                E.dirty = 0;
                return;
            }
        }
        close(fd);
    }
    
    free(buf);
}
```

### フェーズ 4: 検索とハイライト (ステップ 46-60)

#### 1. 基本的な検索
```c
void searchFile() {
    char *query = (char *)malloc(256);
    if (!query) return;
    
    // ユーザーが検索文字列を入力.
    if (readInput(query, 256) == -1 || !query[0]) {
        free(query);
        return;
    }
    
    // 全行をスキャン.
    for (int i = 0; i < E.numrows; i++) {
        erow *row = &E.row[i];
        char *match = strstr(row->chars, query);
        
        if (match) {
            E.cy = i;
            E.cx = match - row->chars;
            E.rowoff = 0;
            break;
        }
    }
    
    free(query);
}
```

#### 2. 簡単なシンタックスハイライト
```c
void updateSyntax(erow *row) {
    row->hl = realloc(row->hl, row->size);
    memset(row->hl, HL_NORMAL, row->size);
    
    if (E.syntax == NULL) return;
    
    char **keywords = E.syntax->keywords;
    char *scs = E.syntax->singleline_comment_start;
    
    for (int i = 0; i < row->size; i++) {
        char c = row->chars[i];
        
        // コメント処理.
        if (!strncmp(&row->chars[i], scs, strlen(scs))) {
            memset(&row->hl[i], HL_COMMENT, row->size - i);
            break;
        }
        
        // 文字列処理.
        if (c == '"' || c == '\'') {
            // 文字列の終わりまで HL_STRING でマーク.
            row->hl[i] = HL_STRING;
            i++;
            while (i < row->size && row->chars[i] != c) {
                row->hl[i] = HL_STRING;
                i++;
            }
            if (i < row->size) row->hl[i] = HL_STRING;
        }
    }
}
```

### フェーズ 5: 高度な機能 (ステップ 61-75)

#### 1. Undo/Redo スタック
```c
typedef struct {
    char *before;
    char *after;
} UndoRecord;

#define UNDO_MAX 100
UndoRecord undo_stack[UNDO_MAX];
int undo_ptr = -1;

void pushUndo(char *before, char *after) {
    undo_ptr++;
    if (undo_ptr >= UNDO_MAX) undo_ptr = UNDO_MAX - 1;
    
    free(undo_stack[undo_ptr].before);
    free(undo_stack[undo_ptr].after);
    
    undo_stack[undo_ptr].before = strdup(before);
    undo_stack[undo_ptr].after = strdup(after);
}

void undo() {
    if (undo_ptr < 0) return;
    
    // 状態を復元.
    UndoRecord *rec = &undo_stack[undo_ptr];
    // ... restore from rec->before ...
    
    undo_ptr--;
}
```

#### 2. マルチラインコメント処理
```c
void updateSyntax(erow *row) {
    // マルチラインコメント状態を追跡.
    int in_comment = (row->idx > 0) ? E.row[row->idx - 1].hlopen : 0;
    
    for (int i = 0; i < row->size; i++) {
        if (in_comment) {
            row->hl[i] = HL_MLCOMMENT;
            if (!strncmp(&row->chars[i], mce, mce_len)) {
                memset(&row->hl[i], HL_MLCOMMENT, mce_len);
                i += mce_len - 1;
                in_comment = 0;
            }
            continue;
        }
        
        // マルチラインコメント開始.
        if (!strncmp(&row->chars[i], mcs, mcs_len)) {
            in_comment = 1;
            memset(&row->hl[i], HL_MLCOMMENT, mcs_len);
            i += mcs_len - 1;
            continue;
        }
    }
    
    row->hlopen = in_comment;
}
```

## ベストプラクティス

### 1. メモリ管理
- **realloc の安全性**: `realloc()` の戻り値をチェック.
  ```c
  char *new = realloc(ptr, size);
  if (!new) {
      perror("realloc");
      return;  // 元のポインタは変更しない.
  }
  ptr = new;
  ```

- **リークの防止**: 終了時にすべてのメモリを解放.
  ```c
  void cleanup() {
      for (int i = 0; i < E.numrows; i++) {
          free(E.row[i].chars);
          free(E.row[i].hl);
      }
      free(E.row);
  }
  ```

### 2. エラーハンドリング
- **関数の失敗をチェック**:
  ```c
  FILE *fp = fopen(filename, "r");
  if (!fp) {
      perror("fopen");
      return -1;
  }
  ```

- **ファイルディスクリプタのクローズ忘れを防ぐ**:
  ```c
  int fd = open(filename, O_RDWR);
  if (fd != -1) {
      // 処理.
      close(fd);
  }
  ```

### 3. スクリーン更新の最適化
- **全画面クリアを避ける**: 変更行のみ再描画.
  ```c
  void refreshScreen() {
      // カーソルを隠す.
      fputs("\x1b[?25l", stdout);
      
      // 変更行のみ描画.
      for (int y = 0; y < E.screenrows; y++) {
          // 変更フラグをチェック.
          if (row_modified[y]) {
              drawRow(y);
          }
      }
      
      // カーソルを表示.
      fputs("\x1b[?25h", stdout);
      fflush(stdout);
  }
  ```

### 4. パフォーマンス改善
- **バッファリング**: write() 呼び出しの回数を最小化.
  ```c
  char buf[1024];
  char *p = buf;
  
  for (int i = 0; i < 100; i++) {
      p += sprintf(p, "text %d\n", i);
  }
  
  write(STDOUT_FILENO, buf, p - buf);
  ```

- **遅延更新**: 画面の変更をまとめて更新.
  ```c
  int dirty = 0;  // 変更フラグ.
  
  void markDirty() { dirty = 1; }
  
  // メインループ.
  if (dirty) {
      refreshScreen();
      dirty = 0;
  }
  ```

### 5. コード構造化
- **関数の責任分離**: 各関数は 1 つの役割を持つ.
  ```c
  // 悪い例.
  void processKey() {
      // キー処理, ファイルI/O, 画面更新...多すぎる.
  }
  
  // 良い例.
  void handleKey(int key) { /* キー処理のみ */ }
  void saveIfNeeded() { /* ファイル保存のみ */ }
  void refreshScreen() { /* 画面更新のみ */ }
  ```

- **構造体の活用**:
  ```c
  struct EditorState {
      int cx, cy;
      int screenrows, screencols;
      int numrows;
      erow *row;
      int dirty;
      char *filename;
  } E;
  ```

## テスト戦略

### ユニットテスト例
```c
// テストするのは難しい (GUI アプリケーションのため),
// しかし一部のロジックはテスト可能.

// テスト: 行の挿入.
void test_insertRow() {
    EditorConfig test_E = {0};
    E = test_E;
    
    insertRow(0, "Hello", 5);
    assert(E.numrows == 1);
    assert(strncmp(E.row[0].chars, "Hello", 5) == 0);
    
    // クリーンアップ.
    free(E.row[0].chars);
    free(E.row);
}

// テスト: 行の削除.
void test_deleteRow() {
    EditorConfig test_E = {0};
    E = test_E;
    
    insertRow(0, "Hello", 5);
    insertRow(1, "World", 5);
    deleteRow(0);
    
    assert(E.numrows == 1);
    assert(strncmp(E.row[0].chars, "World", 5) == 0);
}
```

### 統合テスト
- **手動テスト**: エディタの実装段階でコマンドラインから動作確認.
- **スクリーンショット比較**: 出力の正確性を検証.

## デバッグのコツ

### GDB での調査
```bash
# GDB を起動.
gdb ./kilo

# ブレークポイント設定.
(gdb) break editorRefreshScreen

# 実行.
(gdb) run myfile.txt

# ステップ実行.
(gdb) next

# 変数を表示.
(gdb) print E.cx
(gdb) print E.cy

# ウォッチポイント.
(gdb) watch E.dirty
```

### ログ出力
```c
// debug.log にログを出力.
FILE *debug_log = fopen("/tmp/kilo_debug.log", "a");
fprintf(debug_log, "cx=%d, cy=%d, key=%d\n", E.cx, E.cy, key);
fclose(debug_log);
```

## コンパイルとビルド

### Makefile の例
```makefile
CFLAGS = -std=c99 -Wall -Wextra -pedantic -O2
LDFLAGS = -lm

kilo: kilo.c
	$(CC) $(CFLAGS) -o kilo kilo.c $(LDFLAGS)

clean:
	rm -f kilo

test: kilo
	./kilo test.txt
```

## 次のステップ

### さらに学ぶべき機能
1. **複数バッファ**: 複数のファイルを同時に編集.
2. **マクロ**: キーシーケンスをマクロとして記録・再生.
3. **プラグインシステム**: 拡張可能なアーキテクチャ.
4. **高度な検索**: 正規表現による検索・置換.
5. **折り返しと折りたたみ**: 長い行の処理.

### 参考実装
- Vim: モダンで機能豊富.
- Emacs: 超拡張可能.
- nano: シンプルで初心者向け.

## まとめ

Kilo の実装を学ぶことで, テキストエディタの基本原理を理解できます. 次のステップは:
1. Kilo を完全に理解する.
2. 独自の機能を追加する.
3. 別の言語で実装してみる (Rust, Python など).
4. より高度なテキストエディタを構築する.
