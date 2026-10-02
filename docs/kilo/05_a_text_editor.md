# 5. テキストエディタ

## 通常の文字を挿入する

まず、指定された位置に `erow` に 1 文字を挿入する関数を作成することから始めましょう。

**ステップ 101** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/row-insert-char/kilo.c) / [row-insert-char](https://github.com/snaptoken/kilo-src/tree/row-insert-char))

```diff
 /*** 含まれる ***/
 …
 /*** 定義 ***/
 …
 /*** データ ***/
 …
 /*** ターミナル ***/
 …
 /*** 行操作 ***/
 
 int editorRowCxToRx(erow *row, int cx) {
   …
 }
 
 void editorUpdateRow(erow *row) {
   …
 }
 
 void editorAppendRow(char *s, size_t len) {
   …
 }
 
+void editorRowInsertChar(erow *row, int at, int c) {
+  if (at < 0 || at > row->size) at = row->size;
+  row->chars = realloc(row->chars, row->size + 2);
+  memmove(&row->chars[at + 1], &row->chars[at], row->size - at + 1);
+  row->size++;
+  row->chars[at] = c;
+  editorUpdateRow(row);
+}
+
 /*** ファイル入出力 ***/
 …
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

`memmove()`は`<string.h>`に由来する関数です。`memcpy()`に似ていますが、ソース配列と宛先配列が重複する場合でも安全に使用できます。

まず、文字を挿入したいインデックスである`at`を検証します。`at`は文字列の末尾から1文字先まで指定できることに注意してください。その場合、文字は文字列の末尾に挿入されます。

次に、`erow` の `chars` 用にさらに 1 バイトを割り当てます (ヌル バイトのための領域も確保する必要があるため、`2` を追加します)。そして、`memmove()` を使用して新しい文字のための領域を確保します。`chars` 配列の `size` をインクリメントし、実際に文字を配列内の位置に割り当てます。最後に、`editorUpdateRow()` を呼び出して、`render` フィールドと `rsize` フィールドが新しい行の内容で更新されるようにします。

次に、`/*** エディタ操作 ***/` という新しいセクションを作成します。このセクションには、キー入力をさまざまなテキスト編集操作にマッピングする際に `editorProcessKeypress()` から呼び出す関数が含まれます。このセクションに `editorInsertChar()` という関数を追加します。この関数は文字を受け取り、`editorRowInsertChar()` を使用してカーソルの位置にその文字を挿入します。

**ステップ 102** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/editor-insert-char/kilo.c) / [editor-insert-char](https://github.com/snaptoken/kilo-src/tree/editor-insert-char))

```diff
 /*** インクルード ***/
 …
 /*** 定義 ***/
 …
 /*** データ ***/
 …
 /*** ターミナル ***/
 …
 /*** 行操作 ***/
 
 int editorRowCxToRx(erow *row, int cx) {
   …
 }
 
 void editorUpdateRow(erow *row) {
   …
 }
 
 void editorAppendRow(char *s, size_t len) {
   …
 }
 
 void editorRowInsertChar(erow *row, int at, int c) {
   …
 }
 
+/*** エディタ操作 ***/
+
+void editorInsertChar(int c) {
+  if (E.cy == E.numrows) {
+    editorAppendRow("", 0);
+  }
+  editorRowInsertChar(&E.row[E.cy], E.cx, c);
+  E.cx++;
+}
+
 /*** ファイル入出力 ***/
 …
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

`E.cy == E.numrows` の場合、カーソルはファイルの末尾のチルダ行にあるため、そこに文字を挿入する前にファイルに新しい行を追加する必要があります。文字を挿入した後、カーソルを前方に移動して、ユーザーが次に挿入する文字が挿入された文字の後に来るようにします。

`editorInsertChar()` は `erow` の変更の詳細を気にする必要がなく、`editorRowInsertChar()` はカーソルの位置を気にする必要がないことに注意してください。これが、`/*** エディタ操作 ***/` セクションの関数と `/*** 行操作 ***/` セクションの関数の違いです。

`editorProcessKeypress()` 内の `switch` 文の `default:` ケースで `editorInsertChar()` を呼び出しましょう。これにより、他のエディタ関数にマッピングされていないキー入力が、編集中のテキストに直接挿入されるようになります。

**ステップ 103** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/key-insert-char/kilo.c) / [key-insert-char](https://github.com/snaptoken/kilo-src/tree/key-insert-char))

```diff
 /*** 含める ***/
 …
 /*** 定義する ***/
 …
 /*** データ ***/
 …
 /*** 端末 ***/
 …
 /*** 行操作 ***/
 …
 /*** エディタ操作 ***/
 …
 /*** ファイル入出力 ***/
 …
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 
 void editorMoveCursor(int key) {
   …
 }
 
 void editorProcessKeypress() {
   int c = editorReadKey();
 
   switch (c) {
     case CTRL_KEY('q'):
       write(STDOUT_FILENO, "\x1b[2J", 4);
       write(STDOUT_FILENO, "\x1b[H", 3);
       exit(0);
       break;
 
     case HOME_KEY:
       E.cx = 0;
       break;
 
     case END_KEY:
       if (E.cy < E.numrows)
         E.cx = E.row[E.cy].size;
       break;
 
     case PAGE_UP:
     case PAGE_DOWN:
       {
         if (c == PAGE_UP) {
           E.cy = E.rowoff;
         } else if (c == PAGE_DOWN) {
           E.cy = E.rowoff + E.screenrows - 1;
           if (E.cy > E.numrows) E.cy = E.numrows;
         }
 
         int times = E.screenrows;
         while (times--)
           editorMoveCursor(c == PAGE_UP ? ARROW_UP : ARROW_DOWN);
       }
       break;
 
     case ARROW_UP:
     case ARROW_DOWN:
     case ARROW_LEFT:
     case ARROW_RIGHT:
       editorMoveCursor(c);
       break;
+
+    default:
+      editorInsertChar(c);
+      break;
   }
 }
 
 /*** 初期化 ***/
```

*コンパイル*

この度、テキストビューアを正式にテキストエディタにアップグレードしました。

## 特殊文字の挿入を防止する

現在、バックスペースキーやエンターキーなどを押すと、それらの文字がテキストに直接挿入されてしまいますが、これは望ましくありません。そこで、これらの特殊キーを`editorProcessKeypress()`で処理し、`editorInsertChar()`が呼び出されるデフォルトのケースにならないようにしましょう。

**ステップ 104** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/block-special-chars/kilo.c) / [block-special-chars](https://github.com/snaptoken/kilo-src/tree/block-special-chars))

```diff
 /*** 含まれるもの ***/
 …
 /*** 定義するもの ***/
 
 #define KILO_VERSION "0.0.1"
 #define KILO_TAB_STOP 8
 
 #define CTRL_KEY(k) ((k) & 0x1f)
 
 enum editorKey {
+  BACKSPACE = 127,
   ARROW_LEFT = 1000,
   ARROW_RIGHT,
   ARROW_UP,
   ARROW_DOWN,
   DEL_KEY,
   HOME_KEY,
   END_KEY,
   PAGE_UP,
   PAGE_DOWN
 };
 
 /*** データ ***/
 …
 /*** 端末 ***/
 …
 /*** 行操作 ***/
 …
 /*** エディタ操作 ***/
 …
 /*** ファイル入出力 ***/
 …
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 
 void editorMoveCursor(int key) {
   …
 }
 
 void editorProcessKeypress() {
   int c = editorReadKey();
 
   switch (c) {
+    case '\r':
+      /* TODO */
+      break;
+
     case CTRL_KEY('q'):
       write(STDOUT_FILENO, "\x1b[2J", 4);
       write(STDOUT_FILENO, "\x1b[H", 3);
       exit(0);
       break;
 
     case HOME_KEY:
       E.cx = 0;
       break;
 
     case END_KEY:
       if (E.cy < E.numrows)
         E.cx = E.row[E.cy].size;
       break;
 
+    case BACKSPACE:
+    case CTRL_KEY('h'):
+    case DEL_KEY:
+      /* TODO */
+      break;
+
     case PAGE_UP:
     case PAGE_DOWN:
       {
         if (c == PAGE_UP) {
           E.cy = E.rowoff;
         } else if (c == PAGE_DOWN) {
           E.cy = E.rowoff + E.screenrows - 1;
           if (E.cy > E.numrows) E.cy = E.numrows;
         }
 
         int times = E.screenrows;
         while (times--)
           editorMoveCursor(c == PAGE_UP ? ARROW_UP : ARROW_DOWN);
       }
       break;
 
     case ARROW_UP:
     case ARROW_DOWN:
     case ARROW_LEFT:
     case ARROW_RIGHT:
       editorMoveCursor(c);
       break;
 
+    case CTRL_KEY('l'):
+    case '\x1b':
+      break;
+
     default:
       editorInsertChar(c);
       break;
   }
 }
 
 /*** 初期化 ***/
```

*コンパイル*

バックスペースには、C言語で人間が読みやすいバックスラッシュエスケープ表現（`\n`、`\r`など）がないため、`editorKey`列挙型の一部として追加し、ASCII値として`127`を割り当てます。

`editorProcessKeypress()` 関数では、`switch` 文に最初に追加する新しいキーは `'\r'` で、これは Enter キーです。今のところは無視しますが、後で何らかの処理を実行する予定なので、`TODO` コメントを付けておきます。

バックスペースとデリートは同様の方法で処理し、`TODO` でマークします。また、Ctrl-H キーの組み合わせも処理します。これは、かつてバックスペース文字が送信していた制御コード `8` を送信します。[ASCII テーブル](http://www.asciitable.com/) を見ると、ASCII コード `8` は「バックスペース」の `BS` という名前で、ASCII コード `127` は「削除」の `DEL` という名前であることがわかります。しかし、何らかの理由で、現代のコンピュータでは、バックスペース キーは `127` にマッピングされ、デリート キーはエスケープ シーケンス `<esc>[3~` にマッピングされています。これは、[第 3 章](https://viewsourcecode.org/snaptoken/kilo/03.rawInputAndOutput.html#the-delete-key) の最後で見たとおりです。

最後に、Ctrl-LとEscapeキーについては、これらのキーが押されても何も処理しないことで対応します。Ctrl-Lは従来、ターミナルプログラムで画面を更新するために使われてきました。私たちのテキストエディタでは、どのキーを押した場合でも画面が更新されるため、この機能を実装するために他に何もする必要はありません。Escapeキーを無視するのは、処理していないキーエスケープシーケンス（F1～F12キーなど）が多数あり、`editorReadKey()`の記述方法では、これらのキーを押すとEscapeキーを押したのと同じになってしまうためです。ユーザーが意図せずエスケープ文字`27`をテキストに挿入してしまうことを防ぐため、これらのキー入力は無視します。

## ディスクに保存

、ファイルに書き出す準備ができた単一の文字列に変換する関数を作成します。

**ステップ 105** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/rows-to-string/kilo.c) / [rows-to-string](https://github.com/snaptoken/kilo-src/tree/rows-to-string))

```diff
 /*** 含まれる ***/
 …
 /*** 定義 ***/
 …
 /*** データ ***/
 …
 /*** 端末 ***/
 …
 /*** 行操作 ***/
 …
 /*** エディタ操作 ***/
 …
 /*** ファイル入出力 ***/
 
+char *editorRowsToString(int *buflen) {
+  int totlen = 0;
+  int j;
+  for (j = 0; j < E.numrows; j++)
+    totlen += E.row[j].size + 1;
+  *buflen = totlen;
+
+  char *buf = malloc(totlen);
+  char *p = buf;
+  for (j = 0; j < E.numrows; j++) {
+    memcpy(p, E.row[j].chars, E.row[j].size);
+    p += E.row[j].size;
+    *p = '\n';
+    p++;
+  }
+
+  return buf;
+}
+
 void editorOpen(char *filename) {
   …
   }
 …
 /*** バッファを追加 ***/
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 …
       /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

まず、各行の長さを合計し、各行の末尾に追加する改行文字の分として、それぞれに「1」を加算します。合計の長さを「buflen」に保存し、呼び出し元に文字列の長さを通知します。

次に、必要なメモリを割り当てた後、行をループ処理し、各行の内容をバッファの末尾に `memcpy()` でコピーし、各行の後に改行文字を追加します。

呼び出し元がメモリを解放するために`free()`を実行することを期待して、`buf`を返します。

次に、`editorSave()` 関数を実装します。この関数は、`editorRowsToString()` によって返された文字列を実際にディスクに書き込みます。

**ステップ 106** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/save/kilo.c) / [save](https://github.com/snaptoken/kilo-src/tree/save))

```diff
 /*** 含まれるもの ***/
 
 #define _DEFAULT_SOURCE
 #define _BSD_SOURCE
 #define _GNU_SOURCE
 
 #include <ctype.h>
 #include <errno.h>
+#include <fcntl.h>
 #include <stdio.h>
 #include <stdarg.h>
 #include <stdlib.h>
 #include <string.h>
 #include <sys/ioctl.h>
 #include <sys/types.h>
 #include <termios.h>
 #include <time.h>
 #include <unistd.h>
 
 /*** 定義 ***/
 …
 /*** データ ***/
 …
 /*** 端末 ***/
 …
 /*** 行操作 ***/
 …
 /*** エディタ操作 ***/
 …
 /*** ファイル入出力 ***/
 
 char *editorRowsToString(int *buflen) {
   …
 }
 
 void editorOpen(char *filename) {
   …
 }
 
+void editorSave() {
+  if (E.filename == NULL) return;
+
+  int len;
+  char *buf = editorRowsToString(&len);
+
+  int fd = open(E.filename, O_RDWR | O_CREAT, 0644);
+  ftruncate(fd, len);
+  write(fd, buf, len);
+  close(fd);
+  free(buf);
+}
+
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 …
       /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

`open()`、`O_RDWR`、および`O_CREAT`は`<fcntl.h>`から取得されます。`ftruncate()`および`close()`は`<unistd.h>`から取得されます。

新規ファイルの場合は、`E.filename`は`NULL`となり、ファイルの保存場所がわからないため、今は何もせずに`return`します。後で、ユーザーにファイル名を入力させる方法を検討します。

それ以外の場合は、`editorRowsToString()` を呼び出し、`write()` で文字列を `E.filename` のパスに書き込みます。`open()` には、ファイルがまだ存在しない場合は新規に作成し (`O_CREAT`)、読み書き用にファイルを開く (`O_RDWR`) ように指示します。`O_CREAT` フラグを使用したため、新規ファイルに付与するモード (パーミッション) を含む追加の引数を渡す必要があります。`0644` は、テキストファイルに通常必要な標準パーミッションです。ファイルの所有者にはファイルの読み書き権限が付与され、他のすべてのユーザーにはファイルの読み取り権限のみが付与されます。

`ftruncate()` は、ファイルのサイズを指定された長さに設定します。ファイルサイズが指定された長さより大きい場合は、ファイルの末尾のデータを切り捨てて、指定された長さにします。ファイルサイズが指定された長さより短い場合は、末尾に `0` バイトを追加して、指定された長さにします。

ファイルを上書きする通常の方法は、`open()` に `O_TRUNC` フラグを渡すことです。これにより、新しいデータを書き込む前にファイルが完全に切り詰められ、空のファイルになります。書き込む予定のデータと同じ長さにファイルを切り詰めることで、`ftruncate()` 呼び出しが成功しても `write()` 呼び出しが失敗した場合に、上書き操作全体が少し安全になります。この場合、ファイルには以前のデータの大部分が残ります。しかし、`open()` 呼び出しによってファイルが完全に切り詰められた後に `write()` が失敗した場合、すべてのデータが失われてしまいます。

より高度なエディタは、まず新しい一時ファイルに書き込み、その後そのファイルをユーザーが上書きしたい実際のファイル名に変更し、処理全体を通してエラーがないか注意深くチェックします。

とにかく、あとはキーを`editorSave()`に割り当てるだけなので、やってみましょう！Ctrl-Sを使います。

**ステップ 107** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/ctrl-s/kilo.c) / [ctrl-s](https://github.com/snaptoken/kilo-src/tree/ctrl-s))

```diff
 /*** include ***/
 …
 /*** defined ***/
 …
 /*** data ***/
 …
 /*** terminal ***/
 …
 /*** row operations ***/
 …
 /*** editor operations ***/
 …
 /*** file i/o ***/
 …
 /*** append buffer ***/
 …
 /*** output ***/
 …
 /*** input ***/
 
 void editorMoveCursor(int key) {
   …
 }
 
 void editorProcessKeypress() {
   int c = editorReadKey();
 
   switch (c) {
     case '\r':
       /* TODO */
       break;
 
     case CTRL_KEY('q'):
       write(STDOUT_FILENO, "\x1b[2J", 4);
       write(STDOUT_FILENO, "\x1b[H", 3);
       exit(0);
       break;
 
+    case CTRL_KEY('s'):
+      editorSave();
+      break;
+
     case HOME_KEY:
       E.cx = 0;
       break;
 
     case END_KEY:
       if (E.cy < E.numrows)
         E.cx = E.row[E.cy].size;
       break;
 
     case BACKSPACE:
     case CTRL_KEY('h'):
     case DEL_KEY:
       /* TODO */
       break;
 
     case PAGE_UP:
     case PAGE_DOWN:
       {
         if (c == PAGE_UP) {
           E.cy = E.rowoff;
         } else if (c == PAGE_DOWN) {
           E.cy = E.rowoff + E.screenrows - 1;
           if (E.cy > E.numrows) E.cy = E.numrows;
         }
 
         int times = E.screenrows;
         while (times--)
           editorMoveCursor(c == PAGE_UP ? ARROW_UP : ARROW_DOWN);
       }
       break;
 
     case ARROW_UP:
     case ARROW_DOWN:
     case ARROW_LEFT:
     case ARROW_RIGHT:
       editorMoveCursor(c);
       break;
 
     case CTRL_KEY('l'):
     case '\x1b':
       break;
 
     default:
       editorInsertChar(c);
       break;
   }
 }
 
 /*** 初期化 ***/
```

*コンパイル*

エディタでファイルを開き、文字をいくつか挿入し、Ctrl+Sキーを押してからファイルを再度開くと、変更内容が保存されていることを確認できます。

`editorSave()`にエラー処理を追加しましょう。

**ステップ 108** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/save-errors/kilo.c) / [save-errors](https://github.com/snaptoken/kilo-src/tree/save-errors))

```diff
 /*** 含まれる ***/
 …
 /*** 定義 ***/
 …
 /*** データ ***/
 …
 /*** 端末 ***/
 …
 /*** 行操作 ***/
 …
 /*** エディタ操作 ***/
 …
 /*** ファイル入出力 ***/
 
 char *editorRowsToString(int *buflen) {
   …
 }
 
 void editorOpen(char *filename) {
   …
 }
 
 void editorSave() {
   if (E.filename == NULL) return;
 
   int len;
   char *buf = editorRowsToString(&len);
 
   int fd = open(E.filename, O_RDWR | O_CREAT, 0644);
+  if (fd != -1) {
+    if (ftruncate(fd, len) != -1) {
+      if (write(fd, buf, len) == len) {
+        close(fd);
+        free(buf);
+        return;
+      }
+    }
+    close(fd);
+  }
+
   free(buf);
 }
 
 /*** バッファを追加 ***/
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 …
       /*** 初期化 ***/
```

*コンパイル*

`open()`と`ftruncate()`は、エラーが発生した場合は両方とも`-1`を返します。`write()`は、書き込むように指示したバイト数を返すはずです。エラーが発生したかどうかに関わらず、ファイルが閉じられ、`buf`が指すメモリが解放されることを保証します。

`editorSetStatusMessage()` を使って、保存が成功したかどうかをユーザーに通知しましょう。ついでに、`main()` で設定されているヘルプメッセージに Ctrl+S キーのキーバインドも追加しておきます。

**ステップ 109** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/save-status-message/kilo.c) / [save-status-message](https://github.com/snaptoken/kilo-src/tree/save-status-message))

```diff
 /*** include ***/
 …
 /*** defined ***/
 …
 /*** data ***/
 …
 /*** terminal ***/
 …
 /*** row operations ***/
 …
 /*** editor operations ***/
 …
 /*** file i/o ***/
 
 char *editorRowsToString(int *buflen) {
   …
 }
 
 void editorOpen(char *filename) {
   …
 }
 
 void editorSave() {
   if (E.filename == NULL) return;
 
   int len;
   char *buf = editorRowsToString(&len);
 
   int fd = open(E.filename, O_RDWR | O_CREAT, 0644);
   if (fd != -1) {
     if (ftruncate(fd, len) != -1) {
       if (write(fd, buf, len) == len) {
         close(fd);
         free(buf);
+        editorSetStatusMessage("%d bytes written to disk", len);
         return;
       }
     }
     close(fd);
   }
 
   free(buf);
+  editorSetStatusMessage("Can't save! I/O error: %s", strerror(errno));
 }
 
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 …
 /*** 初期化 ***/
 
 void initEditor() {
   …
 }
 
 int main(int argc, char *argv[]) {
   enableRawMode();
   initEditor();
   if (argc >= 2) {
     editorOpen(argv[1]);
   }
 
+  editorSetStatusMessage("HELP: Ctrl-S = save | Ctrl-Q = quit");
 
   while (1) {
     editorRefreshScreen();
     editorProcessKeypress();
   }
 
   return 0;
 }
```

*コンパイルできません*

`strerror()` は `<string.h>` から来ています。

`strerror()` は `perror()` (`die()` で使用) に似ていますが、引数として `errno` の値を受け取り、そのエラーコードに対応する人間が読める文字列を返します。これにより、エラーをユーザーに表示するステータス メッセージの一部として含めることができます。

上記のコードは実際にはコンパイルされません。なぜなら、ファイル内で定義される前に `editorSetStatusMessage()` を呼び出そうとしているからです。C 言語では、このようなことはできません。C 言語は [シングルパス](https://en.wikipedia.org/wiki/One-pass_compiler) でコンパイルできる言語として設計されているため、プログラムの各部分を、プログラムの後の部分を知らなくてもコンパイルできるはずです。

C言語で関数を呼び出す際、コンパイラはその関数の引数と戻り値を知る必要があります。`editorSetStatusMessage()` 関数のプロトタイプをファイルの先頭付近に宣言することで、コンパイラにこの情報を伝えることができます。これにより、関数が定義される前に呼び出すことが可能になります。新しい `/*** prototypes ***/` セクションを追加し、その下に宣言を記述します。

**ステップ 110** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/prototypes/kilo.c) / [prototypes](https://github.com/snaptoken/kilo-src/tree/prototypes))

```diff
 /*** インクルード ***/
 …
 /*** 定義 ***/
 …
 /*** データ ***/
 
 typedef struct erow {
   …
 } erow;
 
 struct editorConfig {
   …
 };
 
 struct editorConfig E;
 
+/*** プロトタイプ ***/
+
+void editorSetStatusMessage(const char *fmt, ...);
+
 /*** ターミナル ***/
 …
 /*** 行操作 ***/
 …
 /*** エディタ操作 ***/
 …
 /*** ファイル入出力 ***/
 …
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 …
       /*** 初期化 ***/
```

*コンパイル*

## ダーティフラグ

エディタに読み込まれたテキストがファイルの内容と異なるかどうかを記録したいと考えています。そうすることで、ユーザーが終了しようとした際に、保存されていない変更が失われる可能性があることを警告できます。

テキストバッファは、ファイルを開いたり保存したりした後に変更された場合、「ダーティ」と呼ばれます。グローバルエディタの状態に`dirty`変数を追加し、初期値を`0`に設定しましょう。

**ステップ 111** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/dirty/kilo.c) / [dirty](https://github.com/snaptoken/kilo-src/tree/dirty))

```diff
 /*** 含める ***/
 …
 /*** 定義する ***/
 …
 /*** データ ***/
 
 typedef struct erow {
   …
 } erow;
 
 struct editorConfig {
   int cx, cy;
   int rx;
   int rowoff;
   int coloff;
   int screenrows;
   int screencols;
   int numrows;
   erow *row;
+  int dirty;
   char *filename;
   char statusmsg[80];
   time_t statusmsg_time;
   struct termios orig_termios;
 };
 
 struct editorConfig E;
 
 /*** プロトタイプ ***/
 …
 /*** 端末 ***/
 …
 /*** 行操作 ***/
 …
 /*** エディタ操作 ***/
 …
 /*** ファイル入出力 ***/
 …
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 …
 /*** 初期化 ***/
 
 void initEditor() {
   E.cx = 0;
   E.cy = 0;
   E.rx = 0;
   E.rowoff = 0;
   E.coloff = 0;
   E.numrows = 0;
   E.row = NULL;
+  E.dirty = 0;
   E.filename = NULL;
   E.statusmsg[0] = '\0';
   E.statusmsg_time = 0;
 
   if (getWindowSize(&E.screenrows, &E.screencols) == -1) die("getWindowSize");
   E.screenrows -= 2;
 }
 
 int main(int argc, char *argv[]) {
   …
   }
```

*コンパイルはされるが、目に見える効果はない*

ファイルが変更された場合は、ファイル名の後に「(modified)」と表示することで、ステータスバーに`E.dirty`の状態を表示しましょう。

**ステップ 112** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/show-dirty/kilo.c) / [show-dirty](https://github.com/snaptoken/kilo-src/tree/show-dirty))

```diff
 /*** include ***/
 …
 /*** defined ***/
 …
 /*** data ***/
 …
 /*** prototypes ***/
 …
 /*** terminal ***/
 …
 /*** row operations ***/
 …
 /*** editor operations ***/
 …
 /*** file i/o ***/
 …
 /*** append buffer ***/
 …
 /*** output ***/
 
 void editorScroll() {
   …
 }
 
 void editorDrawRows(struct abuf *ab) {
   …
 }
 
 void editorDrawStatusBar(struct abuf *ab) {
   abAppend(ab, "\x1b[7m", 4);
   char status[80], rstatus[80];
+  int len = snprintf(status, sizeof(status), "%.20s - %d lines %s",
+    E.filename ? E.filename : "[No Name]", E.numrows,
+    E.dirty ? "(modified)" : "");
   int rlen = snprintf(rstatus, sizeof(rstatus), "%d/%d",
     E.cy + 1, E.numrows);
   if (len > E.screencols) len = E.screencols;
   abAppend(ab, status, len);
   while (len < E.screencols) {
     if (E.screencols - len == rlen) {
       abAppend(ab, rstatus, rlen);
       break;
     } else {
       abAppend(ab, " ", 1);
       len++;
     }
   }
   abAppend(ab, "\x1b[m", 3);
   abAppend(ab, "\r\n", 2);
 }
 
 void editorDrawMessageBar(struct abuf *ab) {
   …
 }
 
 void editorRefreshScreen() {
   …
 }
 
 void editorSetStatusMessage(const char *fmt, ...) {
   …
 }
 
 /*** 入力 ***/
 …
       /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

それでは、テキストに変更を加える各行操作で、`E.dirty` をインクリメントしてみましょう。

**ステップ 113** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/increment-dirty/kilo.c) / [increment-dirty](https://github.com/snaptoken/kilo-src/tree/increment-dirty))

```diff
 /*** 含まれるもの ***/
 …
 /*** 定義 ***/
 …
 /*** データ ***/
 …
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 行操作 ***/
 
 int editorRowCxToRx(erow *row, int cx) {
   …
 }
 
 void editorUpdateRow(erow *row) {
   …
 }
 
 void editorAppendRow(char *s, size_t len) {
   E.row = realloc(E.row, sizeof(erow) * (E.numrows + 1));
 
   int at = E.numrows;
   E.row[at].size = len;
   E.row[at].chars = malloc(len + 1);
   memcpy(E.row[at].chars, s, len);
   E.row[at].chars[len] = '\0';
 
   E.row[at].rsize = 0;
   E.row[at].render = NULL;
   editorUpdateRow(&E.row[at]);
 
   E.numrows++;
+  E.dirty++;
 }
 
 void editorRowInsertChar(erow *row, int at, int c) {
   if (at < 0 || at > row->size) at = row->size;
   row->chars = realloc(row->chars, row->size + 2);
   memmove(&row->chars[at + 1], &row->chars[at], row->size - at + 1);
   row->size++;
   row->chars[at] = c;
   editorUpdateRow(row);
+  E.dirty++;
 }
 
 /*** エディタ操作 ***/
 …
 /*** ファイル入出力 ***/
 …
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 …
       /*** 初期化 ***/
```

*コンパイル*

`E.dirty++`の代わりに`E.dirty = 1`を使うこともできましたが、インクリメントすることでファイルの「汚れ具合」を把握できるため、これは便利な場合があります。（このチュートリアルでは`E.dirty`をブール値として扱うので、どちらを使っても問題ありません。）

この時点でファイルを開くと、変更を加える前にすぐに「(modified)」と表示されます。これは、`editorOpen()` が `editorAppendRow()` を呼び出し、`E.dirty` をインクリメントするためです。これを修正するには、`editorOpen()` の最後と `editorSave()` で `E.dirty` を `0` にリセットします。

**ステップ 114** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/reset-dirty/kilo.c) / [reset-dirty](https://github.com/snaptoken/kilo-src/tree/reset-dirty))

```diff
 /*** include ***/
 …
 /*** defined ***/
 …
 /*** data ***/
 …
 /*** prototypes ***/
 …
 /*** terminal ***/
 …
 /*** row operations ***/
 …
 /*** editor operations ***/
 …
 /*** file i/o ***/
 
 char *editorRowsToString(int *buflen) {
   …
 }
 
 void editorOpen(char *filename) {
   free(E.filename);
   E.filename = strdup(filename);
 
   FILE *fp = fopen(filename, "r");
   if (!fp) die("fopen");
 
   char *line = NULL;
   size_t linecap = 0;
   ssize_t linelen;
   while ((linelen = getline(&line, &linecap, fp)) != -1) {
     while (linelen > 0 && (line[linelen - 1] == '\n' ||
                            line[linelen - 1] == '\r'))
       linelen--;
     editorAppendRow(line, linelen);
   }
   free(line);
   fclose(fp);
+  E.dirty = 0;
 }
 
 void editorSave() {
   if (E.filename == NULL) return;
 
   int len;
   char *buf = editorRowsToString(&len);
 
   int fd = open(E.filename, O_RDWR | O_CREAT, 0644);
   if (fd != -1) {
     if (ftruncate(fd, len) != -1) {
       if (write(fd, buf, len) == len) {
         close(fd);
         free(buf);
+        E.dirty = 0;
         editorSetStatusMessage("%d bytes written to disk", len);
         return;
       }
     }
     close(fd);
   }
 
   free(buf);
   editorSetStatusMessage("Can't save! I/O error: %s", strerror(errno));
 }
 
 /*** バッファを追加 ***/
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 …
       /*** 初期化 ***/
```

*コンパイル*

これで、最初に文字を挿入したときにステータスバーに「(modified)」と表示され、ファイルをディスクに保存すると消えるはずです。

## 終了確認

これで、ユーザーが終了しようとしたときに、保存されていない変更について警告する準備が整いました。`E.dirty`が設定されている場合は、ステータスバーに警告を表示し、保存せずに終了するには、Ctrl+Qをさらに3回押すようにユーザーに要求します。

**ステップ 115** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/quit-confirmation/kilo.c) / [quit-confirmation](https://github.com/snaptoken/kilo-src/tree/quit-confirmation))

```diff
 /*** 含まれるもの ***/
 …
 /*** 定義 ***/
 
 #define KILO_VERSION "0.0.1"
 #define KILO_TAB_STOP 8
+#define KILO_QUIT_TIMES 3
 
 #define CTRL_KEY(k) ((k) & 0x1f)
 
 enum editorKey {
   …
 };
 
 /*** データ ***/
 …
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 行操作 ***/
 …
 /*** エディタ操作 ***/
 …
 /*** ファイル入出力 ***/
 …
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 
 void editorMoveCursor(int key) {
   …
 }
 
 void editorProcessKeypress() {
+  static int quit_times = KILO_QUIT_TIMES;
+
   int c = editorReadKey();
 
   switch (c) {
     case '\r':
       /* TODO */
       break;
 
     case CTRL_KEY('q'):
+      if (E.dirty && quit_times > 0) {
+        editorSetStatusMessage("WARNING!!! File has unsaved changes. "
+          "Press Ctrl-Q %d more times to quit.", quit_times);
+        quit_times--;
+        return;
+      }
       write(STDOUT_FILENO, "\x1b[2J", 4);
       write(STDOUT_FILENO, "\x1b[H", 3);
       exit(0);
       break;
 
     case CTRL_KEY('s'):
       editorSave();
       break;
 
     case HOME_KEY:
       E.cx = 0;
       break;
 
     case END_KEY:
       if (E.cy < E.numrows)
         E.cx = E.row[E.cy].size;
       break;
 
     case BACKSPACE:
     case CTRL_KEY('h'):
     case DEL_KEY:
       /* TODO */
       break;
 
     case PAGE_UP:
     case PAGE_DOWN:
       {
         if (c == PAGE_UP) {
           E.cy = E.rowoff;
         } else if (c == PAGE_DOWN) {
           E.cy = E.rowoff + E.screenrows - 1;
           if (E.cy > E.numrows) E.cy = E.numrows;
         }
 
         int times = E.screenrows;
         while (times--)
           editorMoveCursor(c == PAGE_UP ? ARROW_UP : ARROW_DOWN);
       }
       break;
 
     case ARROW_UP:
     case ARROW_DOWN:
     case ARROW_LEFT:
     case ARROW_RIGHT:
       editorMoveCursor(c);
       break;
 
     case CTRL_KEY('l'):
     case '\x1b':
       break;
 
     default:
       editorInsertChar(c);
       break;
   }
+
+  quit_times = KILO_QUIT_TIMES;
 }
 
 /*** init ***/
```

*コンパイル*

`editorProcessKeypress()` 関数では、静的変数を使用して、ユーザーが終了するために Ctrl-Q キーをあと何回押す必要があるかを追跡します。保存されていない変更がある状態で Ctrl-Q キーを押すたびに、ステータス メッセージを設定し、`quit_times` をデクリメントします。`quit_times` が `0` になると、プログラムの終了を許可します。Ctrl-Q 以外のキーを押すと、`editorProcessKeypress()` 関数の最後に `quit_times` が `3` にリセットされます。

## シンプルなバックスペース

次にバックスペースを実装しましょう。まず、`erow` 内の文字を削除する `editorRowDelChar()` 関数を作成します。

**ステップ 116** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/row-del-char/kilo.c) / [row-del-char](https://github.com/snaptoken/kilo-src/tree/row-del-char))

```diff
 /*** 含まれる ***/
 …
 /*** 定義 ***/
 …
 /*** データ ***/
 …
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 行操作 ***/
 
 int editorRowCxToRx(erow *row, int cx) {
   …
 }
 
 void editorUpdateRow(erow *row) {
   …
 }
 
 void editorAppendRow(char *s, size_t len) {
   …
 }
 
 void editorRowInsertChar(erow *row, int at, int c) {
   …
 }
 
+void editorRowDelChar(erow *row, int at) {
+  if (at < 0 || at >= row->size) return;
+  memmove(&row->chars[at], &row->chars[at + 1], row->size - at);
+  row->size--;
+  editorUpdateRow(row);
+  E.dirty++;
+}
+
 /*** エディタ操作 ***/
 …
 /*** ファイル入出力 ***/
 …
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 …
       /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

ご覧のとおり、`editorRowInsertChar()` と非常によく似ていますが、メモリ管理は一切必要ありません。`memmove()` を使用して、削除された文字をその後に続く文字で上書きするだけです（末尾のヌルバイトも移動に含まれることに注意してください）。次に、行の `size` をデクリメントし、`editorUpdateRow()` を呼び出し、`E.dirty` をインクリメントします。

それでは、カーソルの左側にある文字を削除するために `editorRowDelChar()` を使用する `editorDelChar()` を実装しましょう。

**ステップ 117** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/editor-del-char/kilo.c) / [editor-del-char](https://github.com/snaptoken/kilo-src/tree/editor-del-char))

```diff
 /*** インクルード ***/
 …
 /*** 定義 ***/
 …
 /*** データ ***/
 …
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 行操作 ***/
 …
 /*** エディタ操作 ***/
 
 void editorInsertChar(int c) {
   …
 }
 
+void editorDelChar() {
+  if (E.cy == E.numrows) return;
+
+  erow *row = &E.row[E.cy];
+  if (E.cx > 0) {
+    editorRowDelChar(row, E.cx - 1);
+    E.cx--;
+  }
+}
+
 /*** ファイル入出力 ***/
 …
 /***バッファの追加 ***/
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 …
       /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

カーソルがファイルの末尾を超えている場合は、削除するものがないため、すぐに処理を終了します。そうでない場合は、カーソルの位置にある行を取得し、カーソルの左側に文字があれば、その文字を削除してカーソルを左に1文字移動します。

バックスペース、Ctrl-H、およびDeleteキーを`editorDelChar()`にマッピングしましょう。

**ステップ 118** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/key-del-char/kilo.c) / [key-del-char](https://github.com/snaptoken/kilo-src/tree/key-del-char))

```diff
 /*** include ***/
 …
 /*** defined ***/
 …
 /*** data ***/
 …
 /*** prototypes ***/
 …
 /*** terminal ***/
 …
 /*** row operations ***/
 …
 /*** editor operations ***/
 …
 /*** file i/o ***/
 …
 /*** append buffer ***/
 …
 /*** output ***/
 …
 /*** input ***/
 
 void editorMoveCursor(int key) {
   …
 }
 
 void editorProcessKeypress() {
   static int quit_times = KILO_QUIT_TIMES;
 
   int c = editorReadKey();
 
   switch (c) {
     case '\r':
       /* TODO */
       break;
 
     case CTRL_KEY('q'):
       if (E.dirty && quit_times > 0) {
         editorSetStatusMessage("WARNING!!! File has unsaved changes. "
           "Press Ctrl-Q %d more times to quit.", quit_times);
         quit_times--;
         return;
       }
       write(STDOUT_FILENO, "\x1b[2J", 4);
       write(STDOUT_FILENO, "\x1b[H", 3);
       exit(0);
       break;
 
     case CTRL_KEY('s'):
       editorSave();
       break;
 
     case HOME_KEY:
       E.cx = 0;
       break;
 
     case END_KEY:
       if (E.cy < E.numrows)
         E.cx = E.row[E.cy].size;
       break;
 
     case BACKSPACE:
     case CTRL_KEY('h'):
     case DEL_KEY:
+      if (c == DEL_KEY) editorMoveCursor(ARROW_RIGHT);
+      editorDelChar();
       break;
 
     case PAGE_UP:
     case PAGE_DOWN:
       {
         if (c == PAGE_UP) {
           E.cy = E.rowoff;
         } else if (c == PAGE_DOWN) {
           E.cy = E.rowoff + E.screenrows - 1;
           if (E.cy > E.numrows) E.cy = E.numrows;
         }
 
         int times = E.screenrows;
         while (times--)
           editorMoveCursor(c == PAGE_UP ? ARROW_UP : ARROW_DOWN);
       }
       break;
 
     case ARROW_UP:
     case ARROW_DOWN:
     case ARROW_LEFT:
     case ARROW_RIGHT:
       editorMoveCursor(c);
       break;
 
     case CTRL_KEY('l'):
     case '\x1b':
       break;
 
     default:
       editorInsertChar(c);
       break;
   }
 
   quit_times = KILO_QUIT_TIMES;
 }
 
 /*** init ***/
```

*コンパイル*

偶然にも、このエディタでは、→キーを押してからバックスペースキーを押すと、テキストエディタでDeleteキーを押したときと同じ動作になります。つまり、カーソルの右側の文字が削除されます。そのため、上記のようにDeleteキーを実装しています。

## 行頭でのバックスペース

現在、`editorDelChar()` はカーソルが行頭にある場合は何も動作しません。ユーザーが行頭でバックスペースキーを押すと、その行の内容を前の行に追加し、現在の行を削除したいと考えています。これにより、2つの行の間にある暗黙の `\n` 文字がバックスペースで削除され、1つの行に結合されます。

そこで、行に文字列を追加する操作と、行を削除する操作という、2つの新しい行操作が必要になります。まずは`editorDelRow()`を実装しましょう。この関数には、削除する`erow`が所有するメモリを解放するための`editorFreeRow()`関数も必要になります。

**ステップ 119** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/del-row/kilo.c) / [del-row](https://github.com/snaptoken/kilo-src/tree/del-row))

```diff
 /*** 含まれる ***/
 …
 /*** 定義 ***/
 …
 /*** データ ***/
 …
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 行操作 ***/
 
 int editorRowCxToRx(erow *row, int cx) {
   …
 }
 
 void editorUpdateRow(erow *row) {
   …
 }
 
 void editorAppendRow(char *s, size_t len) {
   …
 }
 
+void editorFreeRow(erow *row) {
+  free(row->render);
+  free(row->chars);
+}
+
+void editorDelRow(int at) {
+  if (at < 0 || at >= E.numrows) return;
+  editorFreeRow(&E.row[at]);
+  memmove(&E.row[at], &E.row[at + 1], sizeof(erow) * (E.numrows - at - 1));
+  E.numrows--;
+  E.dirty++;
+}
+
 void editorRowInsertChar(erow *row, int at, int c) {
   …
 }
 
 void editorRowDelChar(erow *row, int at) {
   …
 }
 
 /*** エディタ操作 ***/
 …
 /*** ファイル入出力 ***/
 …
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 …
       /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

`editorDelRow()` は `editorRowDelChar()` とよく似ています。どちらの場合も、インデックスによって要素の配列から単一の要素を削除するからです。

まず、`at` インデックスを検証します。次に、`editorFreeRow()` を使用して、行が所有するメモリを解放します。その後、`memmove()` を使用して、削除された行構造体をその後に続く残りの行で上書きし、`numrows` をデクリメントします。最後に、`E.dirty` をインクリメントします。

それでは、行の末尾に文字列を追加する `editorRowAppendString()` を実装しましょう。

**ステップ 120** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/row-append-string/kilo.c) / [row-append-string](https://github.com/snaptoken/kilo-src/tree/row-append-string))

```diff
 /*** 含まれるもの ***/
 …
 /*** 定義 ***/
 …
 /*** データ ***/
 …
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 行操作 ***/
 
 int editorRowCxToRx(erow *row, int cx) {
   …
 }
 
 void editorUpdateRow(erow *row) {
   …
 }
 
 void editorAppendRow(char *s, size_t len) {
   …
 }
 
 void editorFreeRow(erow *row) {
   …
 }
 
 void editorDelRow(int at) {
   …
 }
 
 void editorRowInsertChar(erow *row, int at, int c) {
   …
 }
 
+void editorRowAppendString(erow *row, char *s, size_t len) {
+  row->chars = realloc(row->chars, row->size + len + 1);
+  memcpy(&row->chars[row->size], s, len);
+  row->size += len;
+  row->chars[row->size] = '\0';
+  editorUpdateRow(row);
+  E.dirty++;
+}
+
 void editorRowDelChar(erow *row, int at) {
   …
 }
 
 /*** エディタ操作 ***/
 …
 /*** ファイル入出力 ***/
 …
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 …
       /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

行の新しいサイズは `row->size + len + 1` (ヌルバイトを含む) なので、まず `row->chars` にその分のメモリを割り当てます。次に、指定された文字列を `memcpy()` で `row->chars` の内容の末尾にコピーします。その後、`row->size` を更新し、通常どおり `editorUpdateRow()` を呼び出し、通常どおり `E.dirty` をインクリメントします。

これで、カーソルが行頭にある場合を`editorDelChar()`で処理する準備が整いました。

**ステップ 121** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/del-char-row/kilo.c) / [del-char-row](https://github.com/snaptoken/kilo-src/tree/del-char-row))

```diff
 /*** include ***/
 …
 /*** defined ***/
 …
 /*** data ***/
 …
 /*** prototypes ***/
 …
 /*** terminal ***/
 …
 /*** row operations ***/
 …
 /*** editor operations ***/
 
 void editorInsertChar(int c) {
   …
 }
 
 void editorDelChar() {
   if (E.cy == E.numrows) return;
+  if (E.cx == 0 && E.cy == 0) return;
 
   erow *row = &E.row[E.cy];
   if (E.cx > 0) {
     editorRowDelChar(row, E.cx - 1);
     E.cx--;
+  } else {
+    E.cx = E.row[E.cy - 1].size;
+    editorRowAppendString(&E.row[E.cy - 1], row->chars, row->size);
+    editorDelRow(E.cy);
+    E.cy--;
   }
 }
 
 /*** ファイル入出力 ***/
 …
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 …
       /*** 初期化 ***/
```

*コンパイル*

カーソルが_最初の_行の先頭にある場合は、何もする必要がないので、すぐに`return`します。そうでない場合、`E.cx == 0`が見つかったら、計画どおり`editorRowAppendString()`を呼び出し、次に`editorDelRow()`を呼び出します。`row`は削除する行を指しているので、`row->chars`を前の行に追加し、次に`E.cy`がある行を削除します。前の行に追加する前に、`E.cx`を前の行の内容の末尾に設定します。そうすることで、カーソルは2行が結合した位置に移動します。

行末でDeleteキーを押すと、ユーザーの期待どおりに現在の行と次の行が結合されることに注意してください。これは、行末でカーソルを右に移動すると、カーソルが次の行の先頭に移動するからです。したがって、Deleteキーを→キーの後にBackspaceキーを押すエイリアスとして設定しても、正しく動作します。

## Enterキー

最後に実装する必要があるエディタ操作は、Enter キーです。Enter キーを使用すると、テキストに新しい行を挿入したり、1 行を 2 行に分割したりできます。まず、`editorAppendRow(...)` 関数の名前を `editorInsertRow(int at, ...)` に変更する必要があります。これにより、新しい引数 `at` で指定されたインデックスに行を挿入できるようになります。

**ステップ 122** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/append-to-insert/kilo.c) / [append-to-insert](https://github.com/snaptoken/kilo-src/tree/append-to-insert))

```diff
   row->rsize = idx;
 }
 
+void editorInsertRow(int at, char *s, size_t len) {
+  if (at < 0 || at > E.numrows) return;
+
+  E.row = realloc(E.row, sizeof(erow) * (E.numrows + 1));
+  memmove(&E.row[at + 1], &E.row[at], sizeof(erow) * (E.numrows - at));
 
   E.row[at].size = len;
   E.row[at].chars = malloc(len + 1);
```

*コンパイルできません*

`editorRowInsertChar()` とよく似ていて、まず `at` を検証し、次に `erow` をもう 1 つ割り当て、`memmove()` を使用して指定されたインデックスに新しい行のためのスペースを確保します。

また、`at` が引数として渡されるようになったため、古い `int at = ...` の行も削除します。

今後は、`editorAppendRow(...)` へのすべての呼び出しを `editorInsertRow(E.numrows, ...)` への呼び出しに置き換える必要があります。

**ステップ 123** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/use-insert-row/kilo.c) / [use-insert-row](https://github.com/snaptoken/kilo-src/tree/use-insert-row))

```diff
 /*** インクルード ***/
 …
 /*** 定義 ***/
 …
 /*** データ ***/
 …
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 行操作 ***/
 …
 /*** エディタ操作 ***/
 
 void editorInsertChar(int c) {
   if (E.cy == E.numrows) {
+    editorInsertRow(E.numrows, "", 0);
   }
   editorRowInsertChar(&E.row[E.cy], E.cx, c);
   E.cx++;
 }
 
 void editorDelChar() {
   …
   }
 …
 /*** ファイル入出力 ***/
 
 char *editorRowsToString(int *buflen) {
   …
 }
 
 void editorOpen(char *filename) {
   free(E.filename);
   E.filename = strdup(filename);
 
   FILE *fp = fopen(filename, "r");
   if (!fp) die("fopen");
 
   char *line = NULL;
   size_t linecap = 0;
   ssize_t linelen;
   while ((linelen = getline(&line, &linecap, fp)) != -1) {
     while (linelen > 0 && (line[linelen - 1] == '\n' ||
                            line[linelen - 1] == '\r'))
       linelen--;
+    editorInsertRow(E.numrows, line, linelen);
   }
   free(line);
   fclose(fp);
   E.dirty = 0;
 }
 
 void editorSave() {
   …
       }
 …
 /*** バッファを追加 ***/
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 …
       /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

`editorInsertRow()` が実装できたので、Enter キーの押下を処理する `editorInsertNewline()` を実装する準備が整いました。

**ステップ 124** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/insert-newline/kilo.c) / [insert-newline](https://github.com/snaptoken/kilo-src/tree/insert-newline))

```diff
 /*** include ***/
 …
 /*** defined ***/
 …
 /*** data ***/
 …
 /*** prototypes ***/
 …
 /*** terminal ***/
 …
 /*** row operations ***/
 …
 /*** editor operations ***/
 
 void editorInsertChar(int c) {
   …
 }
 
+void editorInsertNewline() {
+  if (E.cx == 0) {
+    editorInsertRow(E.cy, "", 0);
+  } else {
+    erow *row = &E.row[E.cy];
+    editorInsertRow(E.cy + 1, &row->chars[E.cx], row->size - E.cx);
+    row = &E.row[E.cy];
+    row->size = E.cx;
+    row->chars[row->size] = '\0';
+    editorUpdateRow(row);
+  }
+  E.cy++;
+  E.cx = 0;
+}
+
 void editorDelChar() {
   …
   }
 …
 /*** ファイル入出力 ***/
 …
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 …
       /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

行の先頭にいる場合は、現在の行の前に新しい空白行を挿入するだけで済みます。

そうでない場合は、現在処理中の行を2行に分割する必要があります。まず、`editorInsertRow()` を呼び出し、カーソルより右側の現在の行の文字を渡します。これにより、現在の行の後に正しい内容の新しい行が作成されます。次に、`row` ポインタを再割り当てします。`editorInsertRow()` は `realloc()` を呼び出すため、メモリが移動してポインタが無効になる可能性があるためです（これは危険です）。その後、現在の行のサイズをカーソルの位置に設定して内容を切り詰め、切り詰めた行に対して `editorUpdateRow()` を呼び出します。（`editorInsertRow()` は既に新しい行に対して `editorUpdateRow()` を呼び出しています。）

どちらの場合も、カーソルを行の先頭に移動するために、`E.cy`をインクリメントし、`E.cx`を`0`に設定します。

最後に、実際にEnterキーを`editorInsertNewline()`操作にマッピングしてみましょう。

**ステップ 125** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/enter-key/kilo.c) / [enter-key](https://github.com/snaptoken/kilo-src/tree/enter-key))

```diff
 /*** include ***/
 …
 /*** defined ***/
 …
 /*** data ***/
 …
 /*** prototypes ***/
 …
 /*** terminal ***/
 …
 /*** row operations ***/
 …
 /*** editor operations ***/
 …
 /*** file i/o ***/
 …
 /*** append buffer ***/
 …
 /*** output ***/
 …
 /*** input ***/
 
 void editorMoveCursor(int key) {
   …
 }
 
 void editorProcessKeypress() {
   static int quit_times = KILO_QUIT_TIMES;
 
   int c = editorReadKey();
 
   switch (c) {
     case '\r':
+      editorInsertNewline();
       break;
 
     case CTRL_KEY('q'):
       if (E.dirty && quit_times > 0) {
         editorSetStatusMessage("WARNING!!! File has unsaved changes. "
           "Press Ctrl-Q %d more times to quit.", quit_times);
         quit_times--;
         return;
       }
       write(STDOUT_FILENO, "\x1b[2J", 4);
       write(STDOUT_FILENO, "\x1b[H", 3);
       exit(0);
       break;
 
     case CTRL_KEY('s'):
       editorSave();
       break;
 
     case HOME_KEY:
       E.cx = 0;
       break;
 
     case END_KEY:
       if (E.cy < E.numrows)
         E.cx = E.row[E.cy].size;
       break;
 
     case BACKSPACE:
     case CTRL_KEY('h'):
     case DEL_KEY:
       if (c == DEL_KEY) editorMoveCursor(ARROW_RIGHT);
       editorDelChar();
       break;
 
     case PAGE_UP:
     case PAGE_DOWN:
       {
         if (c == PAGE_UP) {
           E.cy = E.rowoff;
         } else if (c == PAGE_DOWN) {
           E.cy = E.rowoff + E.screenrows - 1;
           if (E.cy > E.numrows) E.cy = E.numrows;
         }
 
         int times = E.screenrows;
         while (times--)
           editorMoveCursor(c == PAGE_UP ? ARROW_UP : ARROW_DOWN);
       }
       break;
 
     case ARROW_UP:
     case ARROW_DOWN:
     case ARROW_LEFT:
     case ARROW_RIGHT:
       editorMoveCursor(c);
       break;
 
     case CTRL_KEY('l'):
     case '\x1b':
       break;
 
     default:
       editorInsertChar(c);
       break;
   }
 
   quit_times = KILO_QUIT_TIMES;
 }
 
 /*** init ***/
```

*コンパイル*

これで、これから実装するテキスト編集操作はすべて完了です。もしご希望であれば、また勇気があれば、このチュートリアルの残りの部分でエディタのコードを修正してみても構いません。その場合は、エディタにバグが発生した場合に備えて、定期的に作業内容のバックアップ（`git`などのツールを使用）を作成することをお勧めします。

## 名前を付けて保存…

現在、ユーザーが引数なしで`./kilo`を実行すると、編集用の空白ファイルが作成されますが、保存する方法がありません。新しいファイルを保存する際に、ユーザーにファイル名の入力を促す方法が必要です。ステータスバーにプロンプトを表示し、プロンプトの後にユーザーがテキストを入力できるようにする`editorPrompt()`関数を作成しましょう。

**ステップ 126** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/prompt/kilo.c) / [prompt](https://github.com/snaptoken/kilo-src/tree/prompt))

```diff
 /*** include ***/
 …
 /*** defined ***/
 …
 /*** data ***/
 …
 /*** prototypes ***/
 
 void editorSetStatusMessage(const char *fmt, ...);
+void editorRefreshScreen();
 
 /*** terminal ***/
 …
 /*** row operations ***/
 …
 /*** editor operations ***/
 …
 /*** file i/o ***/
 …
 /*** append buffer ***/
 …
 /*** output ***/
 …
 /*** input ***/
 
+char *editorPrompt(char *prompt) {
+  size_t bufsize = 128;
+  char *buf = malloc(bufsize);
+
+  size_t buflen = 0;
+  buf[0] = '\0';
+
+  while (1) {
+    editorSetStatusMessage(prompt, buf);
+    editorRefreshScreen();
+
+    int c = editorReadKey();
+    if (c == '\r') {
+      if (buflen != 0) {
+        editorSetStatusMessage("");
+        return buf;
+      }
+    } else if (!iscntrl(c) && c < 128) {
+      if (buflen == bufsize - 1) {
+        bufsize *= 2;
+        buf = realloc(buf, bufsize);
+      }
+      buf[buflen++] = c;
+      buf[buflen] = '\0';
+    }
+  }
+}
+
 void editorMoveCursor(int key) {
   …
 }
 
 void editorProcessKeypress() {
   …
       }
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

ユーザーの入力は、動的に割り当てられる文字列である`buf`に格納されます。`buf`は、最初は空の文字列で初期化されます。その後、ステータスメッセージを繰り返し設定し、画面を更新し、処理するキー入力を待つ無限ループに入ります。`prompt`には、ユーザー入力が表示される場所である`%s`を含むフォーマット文字列が指定されます。

ユーザーが Enter キーを押し、入力内容が空でない場合、ステータス メッセージはクリアされ、入力内容が返されます。それ以外の場合、印刷可能な文字が入力されると、その文字が `buf` に追加されます。`buflen` が割り当てた最大容量 (`bufsize` に格納) に達した場合は、`bufsize` を 2 倍にして、その分のメモリを割り当ててから `buf` に追加します。また、`editorSetStatusMessage()` と `editorPrompt()` の呼び出し元はどちらも文字列の末尾を知るために `\0` 文字を使用するため、`buf` の末尾が `\0` であることを確認します。

入力キーが、`editorKey`列挙型に含まれる、大きな整数値を持つ特殊キーではないことを確認する必要があることに注意してください。そのためには、入力キーが`char`の範囲内にあるかどうかを、`128`未満であることを確認することでテストします。

それでは、`E.filename`が`NULL`の場合に、`editorSave()`でユーザーにファイル名の入力を促してみましょう。

**ステップ 127** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/save-as/kilo.c) / [save-as](https://github.com/snaptoken/kilo-src/tree/save-as))

```diff
 /*** インクルード ***/
 …
 /*** 定義 ***/
 …
 /*** データ ***/
 …
 /*** プロトタイプ ***/
 
 void editorSetStatusMessage(const char *fmt, ...);
 void editorRefreshScreen();
+char *editorPrompt(char *prompt);
 
 /*** ターミナル ***/
 …
 /*** 行操作 ***/
 …
 /*** エディタ操作 ***/
 …
 /*** ファイル入出力 ***/
 
 char *editorRowsToString(int *buflen) {
   …
 }
 
 void editorOpen(char *filename) {
   …
 }
 
 void editorSave() {
+  if (E.filename == NULL) {
+    E.filename = editorPrompt("Save as: %s");
+  }
 
   int len;
   char *buf = editorRowsToString(&len);
 
   int fd = open(E.filename, O_RDWR | O_CREAT, 0644);
   if (fd != -1) {
     if (ftruncate(fd, len) != -1) {
       if (write(fd, buf, len) == len) {
         close(fd);
         free(buf);
         E.dirty = 0;
         editorSetStatusMessage("%d bytes written to disk", len);
         return;
       }
     }
     close(fd);
   }
 
   free(buf);
   editorSetStatusMessage("Can't save! I/O error: %s", strerror(errno));
 }
 
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイル*

よし、これで基本的な「名前を付けて保存」機能が使えるようになった。次に、ユーザーがEscキーを押して入力プロンプトをキャンセルできるようにしよう。

**ステップ 128** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/prompt-escape/kilo.c) / [prompt-escape](https://github.com/snaptoken/kilo-src/tree/prompt-escape))

```diff
 /*** include ***/
 …
 /*** defined ***/
 …
 /*** data ***/
 …
 /*** prototypes ***/
 …
 /*** terminal ***/
 …
 /*** row operations ***/
 …
 /*** editor operations ***/
 …
 /*** file i/o ***/
 …
 /*** append buffer ***/
 …
 /*** output ***/
 …
 /*** input ***/
 
 char *editorPrompt(char *prompt) {
   size_t bufsize = 128;
   char *buf = malloc(bufsize);
 
   size_t buflen = 0;
   buf[0] = '\0';
 
   while (1) {
     editorSetStatusMessage(prompt, buf);
     editorRefreshScreen();
 
     int c = editorReadKey();
+    if (c == '\x1b') {
+      editorSetStatusMessage("");
+      free(buf);
+      return NULL;
+    } else if (c == '\r') {
       if (buflen != 0) {
         editorSetStatusMessage("");
         return buf;
       }
     } else if (!iscntrl(c) && c < 128) {
       if (buflen == bufsize - 1) {
         bufsize *= 2;
         buf = realloc(buf, bufsize);
       }
       buf[buflen++] = c;
       buf[buflen] = '\0';
     }
   }
 }
 
 void editorMoveCursor(int key) {
   …
 }
 
 void editorProcessKeypress() {
   …
       }
 …
 /*** 初期化 ***/
```

*コンパイル*

入力プロンプトがキャンセルされた場合、`buf` を `free()` して `NULL` を返します。そこで、`editorSave()` で戻り値が `NULL` の場合は、保存操作を中止し、「保存が中止されました」というメッセージをユーザーに表示することで対応しましょう。

**ステップ 129** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/abort-save/kilo.c) / [abort-save](https://github.com/snaptoken/kilo-src/tree/abort-save))

```diff
 /*** インクルード ***/
 …
 /*** 定義 ***/
 …
 /*** データ ***/
 …
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 行操作 ***/
 …
 /*** エディタ操作 ***/
 …
 /*** ファイル入出力 ***/
 
 char *editorRowsToString(int *buflen) {
   …
 }
 
 void editorOpen(char *filename) {
   …
 }
 
 void editorSave() {
   if (E.filename == NULL) {
+    E.filename = editorPrompt("Save as: %s (ESC to cancel)");
+    if (E.filename == NULL) {
+      editorSetStatusMessage("Save aborted");
+      return;
+    }
   }
 
   int len;
   char *buf = editorRowsToString(&len);
 
   int fd = open(E.filename, O_RDWR | O_CREAT, 0644);
   if (fd != -1) {
     if (ftruncate(fd, len) != -1) {
       if (write(fd, buf, len) == len) {
         close(fd);
         free(buf);
         E.dirty = 0;
         editorSetStatusMessage("%d bytes written to disk", len);
         return;
       }
     }
     close(fd);
   }
 
   free(buf);
   editorSetStatusMessage("Can't save! I/O error: %s", strerror(errno));
 }
 
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイル*

（注：**Windows版Bash**を使用している場合、プログラムでEscapeキーの押下を認識させるには、Escapeキーを3回押す必要があります。これは、`editorReadKey()`内の`read()`呼び出しがエスケープシーケンスを検出する際にタイムアウトが発生しないためです。）

それでは、入力プロンプトでユーザーがバックスペースキー（またはCtrl+H、またはDeleteキー）を押せるようにしましょう。

**ステップ 130** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/prompt-backspace/kilo.c) / [prompt-backspace](https://github.com/snaptoken/kilo-src/tree/prompt-backspace))

```diff
 /*** 含める ***/
 …
 /*** 定義する ***/
 …
 /*** データ ***/
 …
 /*** プロトタイプ ***/
 …
 /*** 端末 ***/
 …
 /*** 行操作 ***/
 …
 /*** エディタ操作 ***/
 …
 /*** ファイル入出力 ***/
 …
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 
 char *editorPrompt(char *prompt) {
   size_t bufsize = 128;
   char *buf = malloc(bufsize);
 
   size_t buflen = 0;
   buf[0] = '\0';
 
   while (1) {
     editorSetStatusMessage(prompt, buf);
     editorRefreshScreen();
 
     int c = editorReadKey();
+    if (c == DEL_KEY || c == CTRL_KEY('h') || c == BACKSPACE) {
+      if (buflen != 0) buf[--buflen] = '\0';
+    } else if (c == '\x1b') {
       editorSetStatusMessage("");
       free(buf);
       return NULL;
     } else if (c == '\r') {
       if (buflen != 0) {
         editorSetStatusMessage("");
         return buf;
       }
     } else if (!iscntrl(c) && c < 128) {
       if (buflen == bufsize - 1) {
         bufsize *= 2;
         buf = realloc(buf, bufsize);
       }
       buf[buflen++] = c;
       buf[buflen] = '\0';
     }
   }
 }
 
 void editorMoveCursor(int key) {
   …
 }
 
 void editorProcessKeypress() {
   …
       }
 …
 /*** 初期化 ***/
```

*コンパイル*

次の章では、`editorPrompt()` を使用してエディタにインクリメンタル検索機能を実装します。

[ページの先頭](https://viewsourcecode.org/snaptoken/kilo/05.aTextEditor.html#)
