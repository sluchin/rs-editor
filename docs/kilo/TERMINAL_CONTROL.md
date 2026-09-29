# Kilo - ターミナル制御ガイド

このドキュメントは, Kilo がどのようにターミナルを制御しているかについて詳しく説明します.

## ターミナルの基礎

### キャノニカルモードと Raw モード

#### キャノニカルモード (デフォルト)
キャノニカルモードは, ターミナルが行単位でテキストをバッファリングする標準的なモードです.

**特徴:**
- ユーザーが Enter を押すまで, キーボード入力をバッファリング.
- Backspace, Ctrl+U などの編集機能がターミナルに統合.
- Ctrl+C でシグナル SIGINT を送信.
- Ctrl+Z でシグナル SIGTSTP を送信 (プロセスを一時停止).
- エコーバック (入力が自動的に画面に表示).

**問題:**
- リアルタイムのキー入力処理が不可能.
- テキストエディタのような即座の反応が実現できない.

#### Raw モード
Raw モードは, ターミナルがキーボード入力をほぼそのまま渡すモードです.

**特徴:**
- 入力が即座にアプリケーションに渡される.
- キャノニカルモードの特殊な処理がない.
- エコーバックがない.
- すべての表示制御をアプリケーションが行う.

**利点:**
- 文字ごとのリアルタイムな処理.
- 画面制御の完全な制御.
- ゲームやエディタなどのインタラクティブなアプリケーションに最適.

## termios API

### termios 構造体
```c
struct termios {
    tcflag_t c_iflag;      // 入力フラグ.
    tcflag_t c_oflag;      // 出力フラグ.
    tcflag_t c_cflag;      // 制御フラグ.
    tcflag_t c_lflag;      // ローカルフラグ.
    cc_t c_cc[NCCS];       // 特殊文字.
};
```

### Raw モード設定の詳細

#### 入力フラグ (c_iflag)
| フラグ | 説明 | Kilo での使用 |
|--------|------|---------------|
| `BRKINT` | Break 条件を SIGINT に | 無効化 (BRKINT &= ~flag) |
| `ICRNL` | CR を NL に変換 | 無効化 (手入力で CR を認識) |
| `INPCK` | パリティチェック実施 | 無効化 (不要) |
| `ISTRIP` | 最上位ビットをクリア | 無効化 (8 ビット対応) |
| `IXON` | Ctrl+S/Q (フロー制御) | 無効化 (エディタが制御) |

#### 出力フラグ (c_oflag)
| フラグ | 説明 | Kilo での使用 |
|--------|------|---------------|
| `OPOST` | 出力処理 (LF を CR+LF に) | 無効化 (生の出力) |

#### 制御フラグ (c_cflag)
| フラグ | 説明 | Kilo での使用 |
|--------|------|---------------|
| `CS8` | キャラクタサイズ 8 ビット | 設定 (標準) |

#### ローカルフラグ (c_lflag)
| フラグ | 説明 | Kilo での使用 |
|--------|------|---------------|
| `ECHO` | 入力をエコーバック | 無効化 |
| `ICANON` | キャノニカルモード有効 | 無効化 |
| `IEXTEN` | 実装依存の入力処理 | 無効化 |
| `ISIG` | Ctrl+C, Ctrl+Z 処理 | 無効化 |

### Raw モード有効化のコード例
```c
void enableRawMode(int fd) {
    struct termios raw;
    
    // 現在の termios 設定を取得.
    if (tcgetattr(fd, &raw) == -1) die("tcgetattr");
    
    // フラグをクリア.
    raw.c_iflag &= ~(BRKINT | ICRNL | INPCK | ISTRIP | IXON);
    raw.c_oflag &= ~(OPOST);
    raw.c_cflag |= (CS8);
    raw.c_lflag &= ~(ECHO | ICANON | IEXTEN | ISIG);
    
    // 最小読み込み文字数と読み込みタイムアウト.
    raw.c_cc[VMIN] = 0;       // ノンブロッキング.
    raw.c_cc[VTIME] = 1;      // 100ms タイムアウト.
    
    // 新しい設定を適用.
    if (tcsetattr(fd, TCSAFLUSH, &raw) == -1) die("tcsetattr");
}
```

## ANSI エスケープシーケンス

ANSI エスケープシーケンスは, ターミナルを制御するための標準化されたシーケンスです. すべてのシーケンスは ESC (0x1b) で始まります.

### 基本的な形式
```
ESC [ {param} {command}
```

例:
- `ESC[2J` : 画面全体をクリア.
- `ESC[10;20H` : カーソルを行 10, 列 20 に移動.
- `ESC[31m` : テキストを赤色に設定.

### Kilo で使用されるシーケンス

#### カーソル制御
```c
// ホーム位置に移動 (左上).
printf("\x1b[H");

// 指定位置に移動 (行, 列).
printf("\x1b[%d;%dH", row, col);

// カーソルを隠す.
printf("\x1b[?25l");

// カーソルを表示.
printf("\x1b[?25h");
```

| シーケンス | 説明 |
|-----------|------|
| `\x1b[H` | ホーム (0, 0) に移動 |
| `\x1b[{row};{col}H` | ({row}, {col}) に移動 |
| `\x1b[{n}A` | n 行上に移動 |
| `\x1b[{n}B` | n 行下に移動 |
| `\x1b[{n}C` | n 列右に移動 |
| `\x1b[{n}D` | n 列左に移動 |
| `\x1b[?25l` | カーソルを隠す (l = low) |
| `\x1b[?25h` | カーソルを表示 (h = high) |

#### 画面クリア
```c
// 画面全体をクリア.
printf("\x1b[2J");

// カーソル位置から行末まで削除.
printf("\x1b[K");

// カーソル位置から画面末までクリア.
printf("\x1b[0J");
```

| シーケンス | 説明 |
|-----------|------|
| `\x1b[2J` | 画面全体クリア (ED) |
| `\x1b[K` | 行末まで削除 (EL) |
| `\x1b[0J` | カーソルから画面末までクリア |
| `\x1b[1J` | 画面開始からカーソルまでクリア |

#### 表示属性
```c
// リセット.
printf("\x1b[m");

// 太字.
printf("\x1b[1m");

// 反転 (背景色と前景色を入れ替え).
printf("\x1b[7m");

// リセット (上記の属性をすべてクリア).
printf("\x1b[0m");
```

| シーケンス | 説明 |
|-----------|------|
| `\x1b[0m` | デフォルトに リセット |
| `\x1b[1m` | 太字 |
| `\x1b[4m` | 下線 |
| `\x1b[7m` | 反転表示 |

#### 色
```c
// 前景色 (文字色).
printf("\x1b[30m");  // 黒.
printf("\x1b[31m");  // 赤.
printf("\x1b[32m");  // 緑.
printf("\x1b[33m");  // 黄.
printf("\x1b[34m");  // 青.
printf("\x1b[35m");  // マゼンタ.
printf("\x1b[36m");  // シアン.
printf("\x1b[37m");  // 白.

// 背景色.
printf("\x1b[40m");  // 黒.
printf("\x1b[41m");  // 赤.
// ... 同様に 42-47 ...
```

| コード | 説明 |
|--------|------|
| 30-37 | 前景色 (黒, 赤, 緑, 黄, 青, マゼンタ, シアン, 白) |
| 40-47 | 背景色 (同じ順序) |
| 90-97 | 明るい前景色 |
| 100-107 | 明るい背景色 |

### Kilo のシンタックスハイライト色設定
```c
int editorSyntaxToColor(int hl) {
    switch(hl) {
        case HL_COMMENT:
        case HL_MLCOMMENT:
            return 36;  // シアン (コメント).
        case HL_KEYWORD1:
            return 33;  // 黄 (キーワード).
        case HL_KEYWORD2:
            return 32;  // 緑 (型キーワード).
        case HL_STRING:
            return 31;  // 赤 (文字列).
        case HL_NUMBER:
            return 35;  // マゼンタ (数字).
        case HL_MATCH:
            return 34;  // 青 (検索マッチ).
        default:
            return 37;  // 白 (デフォルト).
    }
}
```

## キー入力処理

### エスケープシーケンスの読み込み
矢印キーや機能キーは複数の文字からなるシーケンスで送信されます.

**例:**
- 上矢印: `ESC [ A` = `\x1b[A`
- 下矢印: `ESC [ B` = `\x1b[B`
- 右矢印: `ESC [ C` = `\x1b[C`
- 左矢印: `ESC [ D` = `\x1b[D`

### キー読み込み関数
```c
int editorReadKey(void) {
    int nread;
    unsigned char c;
    
    // 1 文字を読み込み.
    nread = read(STDIN_FILENO, &c, 1);
    if (nread == -1 && errno != EAGAIN) die("read");
    
    // ESC シーケンスの場合.
    if (c == '\x1b') {
        unsigned char seq[3];
        
        // シーケンスの次の 2 文字を読み込み.
        if (read(STDIN_FILENO, &seq[0], 1) == -1) return '\x1b';
        if (read(STDIN_FILENO, &seq[1], 1) == -1) return '\x1b';
        
        // 矢印キーの処理.
        if (seq[0] == '[') {
            switch(seq[1]) {
                case 'A': return ARROW_UP;
                case 'B': return ARROW_DOWN;
                case 'C': return ARROW_RIGHT;
                case 'D': return ARROW_LEFT;
            }
        }
    }
    
    return c;
}
```

## ターミナルサイズの取得

```c
int getWindowSize(int *rows, int *cols) {
    struct winsize ws;
    
    // ioctl TIOCGWINSZ でターミナルサイズを取得.
    if (ioctl(STDOUT_FILENO, TIOCGWINSZ, &ws) == -1 || ws.ws_col == 0) {
        // フォールバック: カーソルを右下に移動し, 位置を読む.
        // ...
        return -1;
    } else {
        *cols = ws.ws_col;
        *rows = ws.ws_row;
        return 0;
    }
}
```

## ウィンドウリサイズの処理

ターミナルがリサイズされた場合, OS は `SIGWINCH` シグナルを送信します.

```c
// シグナルハンドラ.
void handleSigwinch(int sig) {
    signal(SIGWINCH, handleSigwinch);
    
    // 新しいサイズを取得.
    getWindowSize(&E.screenrows, &E.screencols);
    E.screenrows--;  // ステータスバー用.
}

// main() 内でハンドラを設定.
signal(SIGWINCH, handleSigwinch);
```

## 実装上の注意点

### タイムアウト設定
```c
// c_cc[VTIME] は 1/10 秒単位で設定.
raw.c_cc[VTIME] = 1;  // 100ms.
```

ステータスメッセージは短時間表示された後, 自動的に消えます. これはタイムアウトで実現されます.

### エラーハンドリング
```c
// ファイルディスクリプタが無効の場合.
if (tcgetattr(STDIN_FILENO, &orig_termios) == -1) {
    die("tcgetattr");  // プログラムを終了.
}

// 終了時に必ずクリーンアップ.
// atexit() を使用して登録.
atexit(disableRawMode);
```

## パフォーマンス最適化

### バッファリング
```c
// 1 つの write() 呼び出しで複数のシーケンスを送信.
char buf[1024];
sprintf(buf, "\x1b[H\x1b[2J");  // ホーム + 画面クリア.
write(STDOUT_FILENO, buf, strlen(buf));
```

### 差分更新
全画面を毎回クリアするのではなく, 変更された行のみを再描画することで, ちらつきを減らし, パフォーマンスを向上させます.

## トラブルシューティング

### "tcgetattr failed" エラー
- ターミナルがサポートされていない環境.
- SSH 接続の標準入出力がリダイレクトされている.

### 文字が見えない/おかしく表示される
- 端末の文字エンコーディング設定を確認.
- LANG 環境変数を確認.

### 色が正しく表示されない
- ターミナルが色をサポートしているか確認.
- TERM 環境変数を確認 (`xterm-256color` など).

## 参考資料

- ANSI エスケープシーケンス: https://en.wikipedia.org/wiki/ANSI_escape_code
- termios マニュアル: `man termios`
- ioctl マニュアル: `man ioctl_tty`
