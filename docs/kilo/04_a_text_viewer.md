# 4. テキストビューア

## 行ビューア

エディタにテキストの行を格納するためのデータ型を作成しましょう。

**ステップ 55** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/erow/kilo.c) / [erow](https://github.com/snaptoken/kilo-src/tree/erow))

```diff
 /*** インクルード ***/
 …
 /*** 定義 ***/
 …
 /*** データ ***/
 
+typedef struct erow {
+  int size;
+  char *chars;
+} erow;
+
 struct editorConfig {
   int cx, cy;
   int screenrows;
   int screencols;
+  int numrows;
+  erow row;
   struct termios orig_termios;
 };
 
 struct editorConfig E;
 
 /*** ターミナル ***/
 …
 /*** 追加バッファ ***/
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 …
 /*** 初期化 ***/
 
 void initEditor() {
   E.cx = 0;
   E.cy = 0;
+  E.numrows = 0;
 
   if (getWindowSize(&E.screenrows, &E.screencols) == -1) die("getWindowSize");
 }
 
 int main() {
   …
   }
```

*コンパイルはされるが、目に見える効果はない*

`erow`は「エディタ行」の略で、テキスト行を動的に割り当てられた文字データへのポインタと長さとして格納します。`typedef`を使用すると、型を`struct erow`ではなく`erow`として参照できます。

エディタのグローバル状態に`erow`値と`numrows`変数を追加します。現時点では、エディタは1行のテキストしか表示しないため、`numrows`は`0`または`1`のいずれかになります。`initEditor ()`で`0`に初期化します。

それでは、`erow`にテキストを入力してみましょう。ファイルからの読み込みは今は気にしません。代わりに、「Hello, world」という文字列をハードコーディングします。

**ステップ 56** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/hello-world/kilo.c) / [hello-world](https://github.com/snaptoken/kilo-src/tree/hello-world))

```diff
 /*** インクルード ***/
 
 #include <ctype.h>
 #include <errno.h>
 #include <stdio.h>
 #include <stdlib.h>
 #include <string.h>
 #include <sys/ioctl.h>
+#include <sys/types.h>
 #include <termios.h>
 #include <unistd.h>
 
 /*** 定義 ***/
 …
 /*** データ ***/
 …
 /*** ターミナル ***/
 
 void die(const char *s) {
   …
 }
 
 void disableRawMode() {
   …
 }
 
 void enableRawMode() {
   …
 }
 
 int editorReadKey() {
   …
 }
 
 int getCursorPosition(int *rows, int *cols) {
   …
 }
 
 int getWindowSize(int *rows, int *cols) {
   …
   }
 …
+/*** ファイル入出力 ***/
+
+void editorOpen() {
+  char *line = "Hello, world!";
+  ssize_t linelen = 13;
+
+  E.row.size = linelen;
+  E.row.chars = malloc(linelen + 1);
+  memcpy(E.row.chars, line, linelen);
+  E.row.chars[linelen] = '\0';
+  E.numrows = 1;
+}
+
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
 
 int main() {
   enableRawMode();
   initEditor();
+  editorOpen();
 
   while (1) {
     editorRefreshScreen();
     editorProcessKeypress();
   }
 
   return 0;
 }
```

*コンパイルはされるが、目に見える効果はない*

`malloc()` は `<stdlib.h>` から、`ssize_t` は `<sys/types.h>` から来ています。

`editorOpen()` は最終的にディスクからファイルを開いて読み込むためのものとなるため、新しい `/*** file i/o ***/` セクションに配置します。エディタの `erow` 構造体に「Hello, world」メッセージを読み込むには、`size` フィールドをメッセージの長さに設定し、`malloc()` で必要なメモリを解放し、`memcpy()` でメッセージを、割り当てたメモリを指す `chars` フィールドに格納します。最後に、`E.numrows` 変数を `1` に設定して、`erow` に表示すべき行が含まれることを示します。

それでは、表示してみましょう。

**ステップ 57** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/draw-erow/kilo.c) / [draw-erow](https://github.com/snaptoken/kilo-src/tree/draw-erow))

```diff
 /*** include ***/
 …
 /*** defined ***/
 …
 /*** data ***/
 …
 /*** terminal ***/
 …
 /*** file i/o ***/
 …
 /*** append buffer ***/
 …
 /*** output ***/
 
 void editorDrawRows(struct abuf *ab) {
   int y;
   for (y = 0; y < E.screenrows; y++) {
+    if (y >= E.numrows) {
+      if (y == E.screenrows / 3) {
+        char welcome[80];
+        int welcomelen = snprintf(welcome, sizeof(welcome),
+          "Kilo editor -- version %s", KILO_VERSION);
+        if (welcomelen > E.screencols) welcomelen = E.screencols;
+        int padding = (E.screencols - welcomelen) / 2;
+        if (padding) {
+          abAppend(ab, "~", 1);
+          padding--;
+        }
+        while (padding--) abAppend(ab, " ", 1);
+        abAppend(ab, welcome, welcomelen);
+      } else {
         abAppend(ab, "~", 1);
       }
     } else {
+      int len = E.row.size;
+      if (len > E.screencols) len = E.screencols;
+      abAppend(ab, E.row.chars, len);
     }
 
     abAppend(ab, "\x1b[K", 3);
     if (y < E.screenrows - 1) {
       abAppend(ab, "\r\n", 2);
     }
   }
 }
 
 void editorRefreshScreen() {
   …
 }
 
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイル*

以前に行った行描画コードを、現在描画している行がテキストバッファの一部であるか、テキストバッファの末尾の後に続く行であるかをチェックする `if` 文で囲みます。

テキストバッファの一部である行を描画するには、`erow`の`chars`フィールドの内容を書き出すだけです。ただし、描画された行が画面の端からはみ出す場合は、その行を切り詰めるように注意します。

次に、ユーザーが実際にファイルを開けるようにしましょう。ファイルの最初の行を読み込んで表示します。

**ステップ 58** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/open-file/kilo.c) / [open-file](https://github.com/snaptoken/kilo-src/tree/open-file))

```diff
 /*** include ***/
 …
 /*** defined ***/
 …
 /*** data ***/
 …
 /*** terminal ***/
 …
 /*** file i/o ***/
 
+void editorOpen(char *filename) {
+  FILE *fp = fopen(filename, "r");
+  if (!fp) die("fopen");
 
+  char *line = NULL;
+  size_t linecap = 0;
+  ssize_t linelen;
+  linelen = getline(&line, &linecap, fp);
+  if (linelen != -1) {
+    while (linelen > 0 && (line[linelen - 1] == '\n' ||
+                           line[linelen - 1] == '\r'))
+      linelen--;
+    E.row.size = linelen;
+    E.row.chars = malloc(linelen + 1);
+    memcpy(E.row.chars, line, linelen);
+    E.row.chars[linelen] = '\0';
+    E.numrows = 1;
+  }
+  free(line);
+  fclose(fp);
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
 
+int main(int argc, char *argv[]) {
   enableRawMode();
   initEditor();
+  if (argc >= 2) {
+    editorOpen(argv[1]);
+  }
 
   while (1) {
     editorRefreshScreen();
     editorProcessKeypress();
   }
 
   return 0;
 }
```

*コンパイルできる場合とできない場合があります*

`FILE`、`fopen()`、および`getline()`は`<stdio.h>`から来ています。

`editorOpen()` のコア部分は同じですが、ハードコードされた値ではなく、`getline()` から `line` と `linelen` の値を取得するようになりました。

`editorOpen()` はファイル名を受け取り、`fopen()` を使用して読み取り用にファイルを開くようになりました。ユーザーがコマンドライン引数としてファイル名を渡したかどうかを確認することで、開くファイルを選択できます。ファイル名が渡された場合は、`editorOpen()` を呼び出し、ファイル名を渡します。引数なしで `./kilo` を実行した場合、`editorOpen()` は呼び出されず、空のファイルから開始されます。

`getline()` は、各行に割り当てるメモリ量がわからない場合に、ファイルから行を読み込む際に便利です。メモリ管理は自動的に行われます。まず、null の `line` ポインタと `linecap` (行容量) に `0` を渡します。これにより、次に読み込む行用に新しいメモリが割り当てられ、`line` がそのメモリを指すように設定され、`linecap` が割り当てられたメモリ量を示すように設定されます。戻り値は読み込んだ行の長さ、またはファイルの末尾で読み込む行がない場合は `-1` です。後で `editorOpen()` でファイルの複数行を読み込む場合、新しい `line` と `linecap` の値を `getline()` に繰り返し渡すことができます。`linecap` が次に読み込む行を収めるのに十分な大きさである限り、`line` が指すメモリを再利用しようとします。今のところは、読み込んだ1行を`E.row.chars`にコピーし、 `getline()`が割り当てた`line`を`free()`で解放します。

また、行末の改行文字（キャリッジリターン）を削除してから、`erow`にコピーします。各`erow`が1行のテキストを表していることがわかっているので、各行末に改行文字を保存する必要はありません。

コンパイラが `getline()` についてエラーを出す場合は、[機能テストマクロ](https://www.gnu.org/software/libc/manual/html_node/Feature-Test-Macros.html)を定義する必要があるかもしれません。マクロがなくてもお使いのマシンで問題なくコンパイルできる場合でも、コードの移植性を高めるためにマクロを追加しましょう。

**ステップ 59** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/feature-test-macros/kilo.c) / [feature-test-macros](https://github.com/snaptoken/kilo-src/tree/feature-test-macros))

```diff
 /*** インクルード ***/
 
+#define _DEFAULT_SOURCE
+#define _BSD_SOURCE
+#define _GNU_SOURCE
+
 #include <ctype.h>
 #include <errno.h>
 #include <stdio.h>
 #include <stdlib.h>
 #include <string.h>
 #include <sys/ioctl.h>
 #include <sys/types.h>
 #include <termios.h>
 #include <unistd.h>
 
 /*** 定義 ***/
 …
 /*** データ ***/
 …
 /*** 端末 ***/
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

これらのマクロをインクルードファイルよりも上に追加する理由は、インクルードするヘッダーファイルが、どの機能を公開するかを決定するためにマクロを使用するからです。

それでは、簡単なバグを修正しましょう。ウェルカムメッセージは、ユーザーが引数なしでプログラムを起動した場合にのみ表示され、ファイルを開いた際には表示されないようにしたいのです。ウェルカムメッセージがファイルの表示を妨げる可能性があるからです。

**ステップ 60** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/hide-welcome/kilo.c) / [hide-welcome](https://github.com/snaptoken/kilo-src/tree/hide-welcome))

```diff
 /*** include ***/
 …
 /*** defined ***/
 …
 /*** data ***/
 …
 /*** terminal ***/
 …
 /*** file i/o ***/
 …
 /*** append buffer ***/
 …
 /*** output ***/
 
 void editorDrawRows(struct abuf *ab) {
   int y;
   for (y = 0; y < E.screenrows; y++) {
     if (y >= E.numrows) {
+      if (E.numrows == 0 && y == E.screenrows / 3) {
         char welcome[80];
         int welcomelen = snprintf(welcome, sizeof(welcome),
           "Kilo editor -- version %s", KILO_VERSION);
         if (welcomelen > E.screencols) welcomelen = E.screencols;
         int padding = (E.screencols - welcomelen) / 2;
         if (padding) {
           abAppend(ab, "~", 1);
           padding--;
         }
         while (padding--) abAppend(ab, " ", 1);
         abAppend(ab, welcome, welcomelen);
       } else {
         abAppend(ab, "~", 1);
       }
     } else {
       int len = E.row.size;
       if (len > E.screencols) len = E.screencols;
       abAppend(ab, E.row.chars, len);
     }
 
     abAppend(ab, "\x1b[K", 3);
     if (y < E.screenrows - 1) {
       abAppend(ab, "\r\n", 2);
     }
   }
 }
 
 void editorRefreshScreen() {
   …
 }
 
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイル*

これで、テキストバッファが完全に空の場合にのみ、ウェルカムメッセージが表示されるようになりました。

## 複数行

複数行を格納するために、`E.row` を `erow` 構造体の配列にしましょう。これは動的に割り当てられる配列なので、`erow` へのポインタとして定義し、ポインタを `NULL` で初期化します。（これにより、`E.row` がポインタであることを想定していないコードの多くが動作しなくなるため、以降のいくつかのステップでコンパイルエラーが発生します。）

**ステップ 61** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/erow-array/kilo.c) / [erow-array](https://github.com/snaptoken/kilo-src/tree/erow-array))

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
   int cx, cy;
   int screenrows;
   int screencols;
   int numrows;
+  erow *row;
   struct termios orig_termios;
 };
 
 struct editorConfig E;
 
 /*** ターミナル ***/
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
   E.numrows = 0;
+  E.row = NULL;
 
   if (getWindowSize(&E.screenrows, &E.screencols) == -1) die("getWindowSize");
 }
 
 int main(int argc, char *argv[]) {
   …
   }
```

*コンパイルできません*

次に、`editorOpen()` 関数内の `E.row` を初期化するコードを、`editorAppendRow()` という新しい関数に移動しましょう。また、このコードを `/*** 行操作 ***/` という新しいセクションの下に配置します。

**ステップ 62** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/append-row/kilo.c) / [append-row](https://github.com/snaptoken/kilo-src/tree/append-row))

```diff
 /*** 含まれる ***/
 …
 /*** 定義 ***/
 …
 /*** データ ***/
 …
 /*** ターミナル ***/
 
 void die(const char *s) {
   …
 }
 
 void disableRawMode() {
   …
 }
 
 void enableRawMode() {
   …
 }
 
 int editorReadKey() {
   …
 }
 
 int getCursorPosition(int *rows, int *cols) {
   …
 }
 
 int getWindowSize(int *rows, int *cols) {
   …
   }
 …
+/*** 行操作 ***/
+
+void editorAppendRow(char *s, size_t len) {
+  E.row.size = len;
+  E.row.chars = malloc(len + 1);
+  memcpy(E.row.chars, s, len);
+  E.row.chars[len] = '\0';
+  E.numrows = 1;
+}
+
 /*** ファイル入出力 ***/
 
 void editorOpen(char *filename) {
   FILE *fp = fopen(filename, "r");
   if (!fp) die("fopen");
 
   char *line = NULL;
   size_t linecap = 0;
   ssize_t linelen;
   linelen = getline(&line, &linecap, fp);
   if (linelen != -1) {
     while (linelen > 0 && (line[linelen - 1] == '\n' ||
                            line[linelen - 1] == '\r'))
       linelen--;
+    editorAppendRow(line, linelen);
   }
   free(line);
   fclose(fp);
 }
 
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイルできません*

`line`と`linelen`変数の名前を`s`と`len`に変更したことに注意してください。これらは現在、`editorAppendRow()`の引数になっています。

`editorAppendRow()` では、新しい `erow` のための領域を確保し、指定された文字列を `E.row` 配列の末尾にある新しい `erow` にコピーするようにします。それでは、早速実行してみましょう。

**ステップ 63** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/fix-append-row/kilo.c) / [fix-append-row](https://github.com/snaptoken/kilo-src/tree/fix-append-row))

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
 
 void editorAppendRow(char *s, size_t len) {
+  E.row = realloc(E.row, sizeof(erow) * (E.numrows + 1));
+
+  int at = E.numrows;
+  E.row[at].size = len;
+  E.row[at].chars = malloc(len + 1);
+  memcpy(E.row[at].chars, s, len);
+  E.row[at].chars[len] = '\0';
+  E.numrows++;
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

*コンパイルできません*

`realloc()` に割り当てたいバイト数を伝える必要があるので、各 `erow` が占めるバイト数 (`sizeof(erow)`) に、必要な行数を掛けます。次に、`at` を初期化したい新しい行のインデックスに設定し、`E.row` の各出現箇所を `E.row[at]` に置き換えます。最後に、`E.numrows = 1` を `E.numrows++` に変更します。

次に、現在の行を出力する際に、`E.row` の代わりに `E.row[y]` を使用するように `editorDrawRows()` を更新しましょう。

**ステップ 64** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/draw-multiple-erows/kilo.c) / [draw-multiple-erows](https://github.com/snaptoken/kilo-src/tree/draw-multiple-erows))

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
 /*** file i/o ***/
 …
 /*** append buffer ***/
 …
 /*** output ***/
 
 void editorDrawRows(struct abuf *ab) {
   int y;
   for (y = 0; y < E.screenrows; y++) {
     if (y >= E.numrows) {
       if (E.numrows == 0 && y == E.screenrows / 3) {
         char welcome[80];
         int welcomelen = snprintf(welcome, sizeof(welcome),
           "Kilo editor -- version %s", KILO_VERSION);
         if (welcomelen > E.screencols) welcomelen = E.screencols;
         int padding = (E.screencols - welcomelen) / 2;
         if (padding) {
           abAppend(ab, "~", 1);
           padding--;
         }
         while (padding--) abAppend(ab, " ", 1);
         abAppend(ab, welcome, welcomelen);
       } else {
         abAppend(ab, "~", 1);
       }
     } else {
+      int len = E.row[y].size;
       if (len > E.screencols) len = E.screencols;
+      abAppend(ab, E.row[y].chars, len);
     }
 
     abAppend(ab, "\x1b[K", 3);
     if (y < E.screenrows - 1) {
       abAppend(ab, "\r\n", 2);
     }
   }
 }
 
 void editorRefreshScreen() {
   …
 }
 
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

この時点ではコードはコンパイルされるはずですが、ファイルから読み込まれるのは1行だけです。`editorOpen()`に`while`ループを追加して、ファイル全体を`E.row`に読み込むようにしましょう。

**ステップ 65** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/read-multiple-lines/kilo.c) / [read-multiple-lines](https://github.com/snaptoken/kilo-src/tree/read-multiple-lines))

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
 /*** file i/o ***/
 
 void editorOpen(char *filename) {
   FILE *fp = fopen(filename, "r");
   if (!fp) die("fopen");
 
   char *line = NULL;
   size_t linecap = 0;
   ssize_t linelen;
+  while ((linelen = getline(&line, &linecap, fp)) != -1) {
     while (linelen > 0 && (line[linelen - 1] == '\n' ||
                            line[linelen - 1] == '\r'))
       linelen--;
     editorAppendRow(line, linelen);
   }
   free(line);
   fclose(fp);
 }
 
 /*** append buffer ***/
 …
 /*** output ***/
 …
 /*** input ***/
 …
 /*** init ***/
```

*コンパイル*

`while` ループが機能するのは、`getline()` がファイルの末尾に到達し、読み込む行がなくなったときに `-1` を返すためです。

例えば、`./kilo kilo.c` を実行すると、画面にテキスト行が多数表示されるはずです。

## 垂直スクロール

次に、ファイルの先頭数行しか表示されないのではなく、ファイル全体をスクロールできるようにしたいと思います。そこで、グローバルエディタの状態に`rowoff`（行オフセット）変数を追加し、ユーザーが現在スクロールしているファイルの行を追跡できるようにします。

**ステップ 66** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/rowoff/kilo.c) / [rowoff](https://github.com/snaptoken/kilo-src/tree/rowoff))

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
   int cx, cy;
+  int rowoff;
   int screenrows;
   int screencols;
   int numrows;
   erow *row;
   struct termios orig_termios;
 };
 
 struct editorConfig E;
 
 /*** ターミナル ***/
 …
 /*** 行操作 ***/
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
+  E.rowoff = 0;
   E.numrows = 0;
   E.row = NULL;
 
   if (getWindowSize(&E.screenrows, &E.screencols) == -1) die("getWindowSize");
 }
 
 int main(int argc, char *argv[]) {
   …
   }
```

*コンパイルはされるが、目に見える効果はない*

初期値を「0」に設定すると、デフォルトではファイルの先頭までスクロールされます。

それでは、`editorDrawRows()` が `rowoff` の値に応じてファイルの正しい行範囲を表示するようにしましょう。

**ステップ 67** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/filerow/kilo.c) / [filerow](https://github.com/snaptoken/kilo-src/tree/filerow))

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
 /*** ファイル入出力 ***/
 …
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 
 void editorDrawRows(struct abuf *ab) {
   int y;
   for (y = 0; y < E.screenrows; y++) {
+    int filerow = y + E.rowoff;
+    if (filerow >= E.numrows) {
       if (E.numrows == 0 && y == E.screenrows / 3) {
         char welcome[80];
         int welcomelen = snprintf(welcome, sizeof(welcome),
           "Kilo editor -- version %s", KILO_VERSION);
         if (welcomelen > E.screencols) welcomelen = E.screencols;
         int padding = (E.screencols - welcomelen) / 2;
         if (padding) {
           abAppend(ab, "~", 1);
           padding--;
         }
         while (padding--) abAppend(ab, " ", 1);
         abAppend(ab, welcome, welcomelen);
       } else {
         abAppend(ab, "~", 1);
       }
     } else {
+      int len = E.row[filerow].size;
       if (len > E.screencols) len = E.screencols;
+      abAppend(ab, E.row[filerow].chars, len);
     }
 
     abAppend(ab, "\x1b[K", 3);
     if (y < E.screenrows - 1) {
       abAppend(ab, "\r\n", 2);
     }
   }
 }
 
 void editorRefreshScreen() {
   …
 }
 
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

各 `y` 位置に表示したいファイルの行を取得するには、`y` 位置に `E.rowoff` を加算します。そのため、その値を格納する新しい変数 `filerow` を定義し、それを `E.row` のインデックスとして使用します。

では、`E.rowoff` の値はどこで設定すればよいでしょうか？私たちの戦略は、カーソルが可視ウィンドウの外に移動したかどうかを確認し、移動した場合は、カーソルが可視ウィンドウのすぐ内側になるように `E.rowoff` を調整することです。このロジックを `editorScroll()` という関数に記述し、画面を更新する直前に呼び出します。

**ステップ 68** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/editor-scroll/kilo.c) / [editor-scroll](https://github.com/snaptoken/kilo-src/tree/editor-scroll))

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
 /*** ファイル入出力 ***/
 …
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 
+void editorScroll() {
+  if (E.cy < E.rowoff) {
+    E.rowoff = E.cy;
+  }
+  if (E.cy >= E.rowoff + E.screenrows) {
+    E.rowoff = E.cy - E.screenrows + 1;
+  }
+}
+
 void editorDrawRows(struct abuf *ab) {
   …
 }
 
 void editorRefreshScreen() {
+  editorScroll();
+
   struct abuf ab = ABUF_INIT;
 
   abAppend(&ab, "\x1b[?25l", 6);
   abAppend(&ab, "\x1b[H", 3);
 
   editorDrawRows(&ab);
 
   char buf[32];
   snprintf(buf, sizeof(buf), "\x1b[%d;%dH", E.cy + 1, E.cx + 1);
   abAppend(&ab, buf, strlen(buf));
 
   abAppend(&ab, "\x1b[?25h", 6);
 
   write(STDOUT_FILENO, ab.b, ab.len);
   abFree(&ab);
 }
 
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

最初の `if` 文は、カーソルが表示されているウィンドウの上にあるかどうかをチェックし、上にある場合はカーソルの位置までスクロールします。2 番目の `if` 文は、カーソルが表示されているウィンドウの下端を超えているかどうかをチェックし、`E.rowoff` は画面の _上_の位置を参照するため、画面の _下_の位置については `E.screenrows` を使用する必要があるため、少し複雑な計算が含まれています。

それでは、カーソルを画面の下部を超えて移動させてみましょう（ただし、ファイルの下部を超えては移動させません）。

**ステップ 69** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/enable-vertical-scroll/kilo.c) / [enable-vertical-scroll](https://github.com/snaptoken/kilo-src/tree/enable-vertical-scroll))

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
 /*** file i/o ***/
 …
 /*** append buffer ***/
 …
 /*** output ***/
 …
 /*** input ***/
 
 void editorMoveCursor(int key) {
   switch (key) {
     case ARROW_LEFT:
       if (E.cx != 0) {
         E.cx--;
       }
       break;
     case ARROW_RIGHT:
       if (E.cx != E.screencols - 1) {
         E.cx++;
       }
       break;
     case ARROW_UP:
       if (E.cy != 0) {
         E.cy--;
       }
       break;
     case ARROW_DOWN:
+      if (E.cy < E.numrows) {
         E.cy++;
       }
       break;
   }
 }
 
 void editorProcessKeypress() {
   …
       }
 …
 /*** init ***/
```

*コンパイル*

`./kilo kilo.c`を実行すると、ファイル全体をスクロールできるようになります。（ファイルにタブ文字が含まれている場合、画面に描画する際にタブ文字が正しく消去されないことがあります。この問題は近日中に修正予定です。それまでの間は、タブ文字の少ないファイルでテストしてみてください。）

上にスクロールしようとすると、カーソルが正しく配置されないことに気づくかもしれません。これは、`E.cy` が画面上のカーソルの位置ではなく、テキストファイル内のカーソルの位置を参照するようになったためです。画面上にカーソルを配置するには、`E.cy` の値から `E.rowoff` を減算する必要があります。

**ステップ 70** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/fix-cursor-scrolling/kilo.c) / [fix-cursor-scrolling](https://github.com/snaptoken/kilo-src/tree/fix-cursor-scrolling))

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
 /*** ファイル入出力 ***/
 …
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 
 void editorScroll() {
   …
 }
 
 void editorDrawRows(struct abuf *ab) {
   …
 }
 
 void editorRefreshScreen() {
   editorScroll();
 
   struct abuf ab = ABUF_INIT;
 
   abAppend(&ab, "\x1b[?25l", 6);
   abAppend(&ab, "\x1b[H", 3);
 
   editorDrawRows(&ab);
 
   char buf[32];
+  snprintf(buf, sizeof(buf), "\x1b[%d;%dH", (E.cy - E.rowoff) + 1, E.cx + 1);
   abAppend(&ab, buf, strlen(buf));
 
   abAppend(&ab, "\x1b[?25h", 6);
 
   write(STDOUT_FILENO, ab.b, ab.len);
   abFree(&ab);
 }
 
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイル*

## 水平スクロール

それでは、水平スクロールに取り掛かりましょう。垂直スクロールを実装したのとほぼ同じ方法で実装します。まず、`coloff`（列オフセット）変数をグローバルエディタ状態に追加します。

**ステップ 71** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/coloff/kilo.c) / [coloff](https://github.com/snaptoken/kilo-src/tree/coloff))

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
   int rowoff;
+  int coloff;
   int screenrows;
   int screencols;
   int numrows;
   erow *row;
   struct termios orig_termios;
 };
 
 struct editorConfig E;
 
 /*** 端末 ***/
 …
 /*** 行操作 ***/
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
   E.rowoff = 0;
+  E.coloff = 0;
   E.numrows = 0;
   E.row = NULL;
 
   if (getWindowSize(&E.screenrows, &E.screencols) == -1) die("getWindowSize");
 }
 
 int main(int argc, char *argv[]) {
   …
   }
```

*コンパイルはされるが、目に見える効果はない*

列オフセットに各行を表示するには、表示する各 `erow` の `chars` へのインデックスとして `E.coloff` を使用し、オフセットの左側にある文字数を行の長さから減算します。

**ステップ 72** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/use-coloff/kilo.c) / [use-coloff](https://github.com/snaptoken/kilo-src/tree/use-coloff))

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
 /*** file i/o ***/
 …
 /*** append buffer ***/
 …
 /*** output ***/
 
 void editorScroll() {
   …
 }
 
 void editorDrawRows(struct abuf *ab) {
   int y;
   for (y = 0; y < E.screenrows; y++) {
     int filerow = y + E.rowoff;
     if (filerow >= E.numrows) {
       if (E.numrows == 0 && y == E.screenrows / 3) {
         char welcome[80];
         int welcomelen = snprintf(welcome, sizeof(welcome),
           "Kilo editor -- version %s", KILO_VERSION);
         if (welcomelen > E.screencols) welcomelen = E.screencols;
         int padding = (E.screencols - welcomelen) / 2;
         if (padding) {
           abAppend(ab, "~", 1);
           padding--;
         }
         while (padding--) abAppend(ab, " ", 1);
         abAppend(ab, welcome, welcomelen);
       } else {
         abAppend(ab, "~", 1);
       }
     } else {
+      int len = E.row[filerow].size - E.coloff;
+      if (len < 0) len = 0;
       if (len > E.screencols) len = E.screencols;
+      abAppend(ab, &E.row[filerow].chars[E.coloff], len);
     }
 
     abAppend(ab, "\x1b[K", 3);
     if (y < E.screenrows - 1) {
       abAppend(ab, "\r\n", 2);
     }
   }
 }
 
 void editorRefreshScreen() {
   …
 }
 
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

長さから`E.coloff`を減算すると、`len`が負の値になる場合があることに注意してください。これは、ユーザーが行末を超えて水平方向にスクロールしたことを意味します。その場合、`len`を`0`に設定して、その行には何も表示されないようにします。

それでは、水平スクロールに対応するように`editorScroll()`を更新しましょう。

**ステップ 73** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/editor-scroll-horizontal/kilo.c) / [editor-scroll-horizontal](https://github.com/snaptoken/kilo-src/tree/editor-scroll-horizontal))

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
 /*** ファイル入出力 ***/
 …
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 
 void editorScroll() {
   if (E.cy < E.rowoff) {
     E.rowoff = E.cy;
   }
   if (E.cy >= E.rowoff + E.screenrows) {
     E.rowoff = E.cy - E.screenrows + 1;
   }
+  if (E.cx < E.coloff) {
+    E.coloff = E.cx;
+  }
+  if (E.cx >= E.coloff + E.screencols) {
+    E.coloff = E.cx - E.screencols + 1;
+  }
 }
 
 void editorDrawRows(struct abuf *ab) {
   …
 }
 
 void editorRefreshScreen() {
   …
 }
 
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

ご覧のとおり、これは垂直スクロールのコードと全く同じです。`E.cy`を`E.cx`に、`E.rowoff`を`E.coloff`に、`E.screenrows`を`E.screencols`に置き換えるだけです。

それでは、ユーザーが画面の右端を超えてスクロールできるようにしてみましょう。

**ステップ 74** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/enable-horizontal-scroll/kilo.c) / [enable-horizontal-scroll](https://github.com/snaptoken/kilo-src/tree/enable-horizontal-scroll))

```diff
       }
       break;
     case ARROW_RIGHT:
+      E.cx++;
       break;
     case ARROW_UP:
       if (E.cy != 0) {
```

*コンパイル*

水平スクロールが正常に動作するようになったことを確認できるはずです。

次に、垂直スクロールの場合と同様に、カーソルの位置を修正しましょう。

**ステップ 75** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/fix-cursor-scrolling-horizontal/kilo.c) / [fix-cursor-scrolling-horizontal](https://github.com/snaptoken/kilo-src/tree/fix-cursor-scrolling-horizontal))

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
 /*** ファイル入出力 ***/
 …
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 
 void editorScroll() {
   …
 }
 
 void editorDrawRows(struct abuf *ab) {
   …
 }
 
 void editorRefreshScreen() {
   editorScroll();
 
   struct abuf ab = ABUF_INIT;
 
   abAppend(&ab, "\x1b[?25l", 6);
   abAppend(&ab, "\x1b[H", 3);
 
   editorDrawRows(&ab);
 
   char buf[32];
+  snprintf(buf, sizeof(buf), "\x1b[%d;%dH", (E.cy - E.rowoff) + 1,
+                                            (E.cx - E.coloff) + 1);
   abAppend(&ab, buf, strlen(buf));
 
   abAppend(&ab, "\x1b[?25h", 6);
 
   write(STDOUT_FILENO, ab.b, ab.len);
   abFree(&ab);
 }
 
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイル*

## 右方向へのスクロールを制限する

`E.cx`と`E.cy`はどちらも、画面上のカーソル位置ではなく、ファイル内のカーソル位置を参照します。そのため、次の手順では、`E.cx`と`E.cy`の値がファイル内の有効な位置のみを指すように制限します。そうしないと、ユーザーがカーソルを行の右端まで移動させてそこにテキストを挿入できてしまい、意味がなくなってしまいます。（このルールの唯一の例外は、`E.cx`が行末から1文字先を指すことで、行末に文字を挿入できること、そして`E.cy`がファイル末尾から1行先を指すことで、ファイル末尾に新しい行を簡単に追加できることです。）

まず、ユーザーが現在の行の末尾を超えてスクロールできないように設定しましょう。

**ステップ 76** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/scroll-limits/kilo.c) / [scroll-limits](https://github.com/snaptoken/kilo-src/tree/scroll-limits))

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
 /*** ファイル入出力 ***/
 …
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 
 void editorMoveCursor(int key) {
+  erow *row = (E.cy >= E.numrows) ? NULL : &E.row[E.cy];
+
   switch (key) {
     case ARROW_LEFT:
       if (E.cx != 0) {
         E.cx--;
       }
       break;
     case ARROW_RIGHT:
+      if (row && E.cx < row->size) {
+        E.cx++;
+      }
       break;
     case ARROW_UP:
       if (E.cy != 0) {
         E.cy--;
       }
       break;
     case ARROW_DOWN:
       if (E.cy < E.numrows) {
         E.cy++;
       }
       break;
   }
 }
 
 void editorProcessKeypress() {
   …
       }
 …
 /*** 初期化 ***/
```

*コンパイル*

`E.cy` はファイルの最終行の1行先でも構わないので、三項演算子を使ってカーソルが実際の行上にあるかどうかを確認します。もしそうであれば、`row` 変数はカーソルがある `erow` を指し、カーソルが右に移動する前に `E.cx` がその行の末尾の左側にあるかどうかを確認します。

## カーソルを行末にスナップする

ただし、ユーザーはカーソルを行末より先に移動させることは可能です。長い行の末尾にカーソルを移動させ、次に短い行に移動させることでそれが可能です。この場合、`E.cx`の値は変更されず、カーソルは現在位置する行の末尾より右側に移動します。

`editorMoveCursor()` に、`E.cx` が現在の行の末尾を超えて移動した場合に修正するコードを追加しましょう。

**ステップ 77** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/snap-cursor/kilo.c) / [snap-cursor](https://github.com/snaptoken/kilo-src/tree/snap-cursor))

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
 /*** ファイル入出力 ***/
 …
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 
 void editorMoveCursor(int key) {
   erow *row = (E.cy >= E.numrows) ? NULL : &E.row[E.cy];
 
   switch (key) {
     case ARROW_LEFT:
       if (E.cx != 0) {
         E.cx--;
       }
       break;
     case ARROW_RIGHT:
       if (row && E.cx < row->size) {
         E.cx++;
       }
       break;
     case ARROW_UP:
       if (E.cy != 0) {
         E.cy--;
       }
       break;
     case ARROW_DOWN:
       if (E.cy < E.numrows) {
         E.cy++;
       }
       break;
   }
+
+  row = (E.cy >= E.numrows) ? NULL : &E.row[E.cy];
+  int rowlen = row ? row->size : 0;
+  if (E.cx > rowlen) {
+    E.cx = rowlen;
+  }
 }
 
 void editorProcessKeypress() {
   …
       }
 …
 /*** 初期化 ***/
```

*コンパイル*

`E.cy`が以前とは異なる行を指している可能性があるため、`row`を再度設定する必要があります。次に、`E.cx`がその行の末尾より右側にある場合は、`E.cx`をその行の末尾に設定します。また、ここでは`NULL`行の長さを`0`とみなしますが、これは今回の目的には適しています。

## 行頭で左に移動する

ユーザーが行頭で←キーを押すと、前の行の末尾に移動できるようにしましょう。

**ステップ 78** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/moving-left/kilo.c) / [moving-left](https://github.com/snaptoken/kilo-src/tree/moving-left))

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
 /*** ファイル入出力 ***/
 …
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 
 void editorMoveCursor(int key) {
   erow *row = (E.cy >= E.numrows) ? NULL : &E.row[E.cy];
 
   switch (key) {
     case ARROW_LEFT:
       if (E.cx != 0) {
         E.cx--;
+      } else if (E.cy > 0) {
+        E.cy--;
+        E.cx = E.row[E.cy].size;
       }
       break;
     case ARROW_RIGHT:
       if (row && E.cx < row->size) {
         E.cx++;
       }
       break;
     case ARROW_UP:
       if (E.cy != 0) {
         E.cy--;
       }
       break;
     case ARROW_DOWN:
       if (E.cy < E.numrows) {
         E.cy++;
       }
       break;
   }
 
   row = (E.cy >= E.numrows) ? NULL : &E.row[E.cy];
   int rowlen = row ? row->size : 0;
   if (E.cx > rowlen) {
     E.cx = rowlen;
   }
 }
 
 void editorProcessKeypress() {
   …
       }
 …
 /*** 初期化 ***/
```

*コンパイル*

一番上の行にいないことを確認してから、一列上に移動させます。

## 行末で右に移動する

同様に、ユーザーが行末で「→」を押すと次の行の先頭に移動できるようにしましょう。

**ステップ 79** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/moving-right/kilo.c) / [moving-right](https://github.com/snaptoken/kilo-src/tree/moving-right))

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
 /*** ファイル入出力 ***/
 …
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 
 void editorMoveCursor(int key) {
   erow *row = (E.cy >= E.numrows) ? NULL : &E.row[E.cy];
 
   switch (key) {
     case ARROW_LEFT:
       if (E.cx != 0) {
         E.cx--;
       } else if (E.cy > 0) {
         E.cy--;
         E.cx = E.row[E.cy].size;
       }
       break;
     case ARROW_RIGHT:
       if (row && E.cx < row->size) {
         E.cx++;
+      } else if (row && E.cx == row->size) {
+        E.cy++;
+        E.cx = 0;
       }
       break;
     case ARROW_UP:
       if (E.cy != 0) {
         E.cy--;
       }
       break;
     case ARROW_DOWN:
       if (E.cy < E.numrows) {
         E.cy++;
       }
       break;
   }
 
   row = (E.cy >= E.numrows) ? NULL : &E.row[E.cy];
   int rowlen = row ? row->size : 0;
   if (E.cx > rowlen) {
     E.cx = rowlen;
   }
 }
 
 void editorProcessKeypress() {
   …
       }
 …
 /*** 初期化 ***/
```

*コンパイル*

ここでは、次の行に進む前に、それらがファイルの末尾にないことを確認する必要があります。

## タブのレンダリング

`./kilo Makefile` を使用して `Makefile` を開いてみると、Makefile の 2 行目のタブ文字が 8 列ほどの幅を占めていることに気づくでしょう。タブの長さは、使用するターミナルとその設定によって異なります。各タブの長さを把握し、タブのレンダリング方法も制御したいので、`erow` 構造体に `render` という 2 番目の文字列を追加します。この文字列には、その行のテキストに対して実際に画面に描画する文字が含まれます。今のところ `render` はタブにのみ使用しますが、将来的には、印刷不可能な制御文字を `^` 文字の後に別の文字が続く形でレンダリングするために使用できます。たとえば、Ctrl-A 文字の場合は `^A` とします (これは、ターミナルで制御文字を表示する一般的な方法です)。

また、ターミナルで`Makefile`内のタブ文字が表示されても、そのタブ内の画面上の文字は消去されないことに気づくかもしれません。タブは、改行やキャリッジリターンと同様に、カーソルを次のタブ位置まで移動させるだけです。これが、タブを複数のスペースとして表示したいもう一つの理由です。スペースは、その前にあった文字を消去するからです。

それでは、まず `render` と `rsize` (`render` の内容のサイズを含む) を `erow` 構造体に追加し、新しい `erow` が構築および初期化される `editorAppendRow()` で初期化することから始めましょう。

**ステップ 80** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/render/kilo.c) / [render](https://github.com/snaptoken/kilo-src/tree/render))

```diff
 /*** 含める ***/
 …
 /*** 定義する ***/
 …
 /*** データ ***/
 
 typedef struct erow {
   int size;
+  int rsize;
   char *chars;
+  char *render;
 } erow;
 
 struct editorConfig {
   …
 };
 
 struct editorConfig E;
 
 /*** ターミナル ***/
 …
 /*** 行操作 ***/
 
 void editorAppendRow(char *s, size_t len) {
   E.row = realloc(E.row, sizeof(erow) * (E.numrows + 1));
 
   int at = E.numrows;
   E.row[at].size = len;
   E.row[at].chars = malloc(len + 1);
   memcpy(E.row[at].chars, s, len);
   E.row[at].chars[len] = '\0';
+
+  E.row[at].rsize = 0;
+  E.row[at].render = NULL;
+
   E.numrows++;
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

*コンパイルはされるが、目に見える効果はない*

次に、`erow` の `chars` 文字列を使用して `render` 文字列の内容を埋める `editorUpdateRow()` 関数を作成しましょう。`chars` から `render` へ各文字をコピーします。タブのレンダリング方法については、今はまだ考慮しません。

**ステップ 81** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/editor-update-row/kilo.c) / [editor-update-row](https://github.com/snaptoken/kilo-src/tree/editor-update-row))

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
 
+void editorUpdateRow(erow *row) {
+  free(row->render);
+  row->render = malloc(row->size + 1);
+
+  int j;
+  int idx = 0;
+  for (j = 0; j < row->size; j++) {
+    row->render[idx++] = row->chars[j];
+  }
+  row->render[idx] = '\0';
+  row->rsize = idx;
+}
+
 void editorAppendRow(char *s, size_t len) {
   E.row = realloc(E.row, sizeof(erow) * (E.numrows + 1));
 
   int at = E.numrows;
   E.row[at].size = len;
   E.row[at].chars = malloc(len + 1);
   memcpy(E.row[at].chars, s, len);
   E.row[at].chars[len] = '\0';
 
   E.row[at].rsize = 0;
   E.row[at].render = NULL;
+  editorUpdateRow(&E.row[at]);
 
   E.numrows++;
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

*コンパイルはされるが、目に見える効果はない*

`for`ループの後、`idx`には`row->render`にコピーした文字数が格納されるので、それを`row->rsize`に代入します。

それでは、各 `erow` を表示する際に、`editorDrawRows()` 内の `chars` と `size` を `render` と `rsize` に置き換えてみましょう。

**ステップ 82** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/use-render/kilo.c) / [use-render](https://github.com/snaptoken/kilo-src/tree/use-render))

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
 /*** file i/o ***/
 …
 /*** append buffer ***/
 …
 /*** output ***/
 
 void editorScroll() {
   …
 }
 
 void editorDrawRows(struct abuf *ab) {
   int y;
   for (y = 0; y < E.screenrows; y++) {
     int filerow = y + E.rowoff;
     if (filerow >= E.numrows) {
       if (E.numrows == 0 && y == E.screenrows / 3) {
         char welcome[80];
         int welcomelen = snprintf(welcome, sizeof(welcome),
           "Kilo editor -- version %s", KILO_VERSION);
         if (welcomelen > E.screencols) welcomelen = E.screencols;
         int padding = (E.screencols - welcomelen) / 2;
         if (padding) {
           abAppend(ab, "~", 1);
           padding--;
         }
         while (padding--) abAppend(ab, " ", 1);
         abAppend(ab, welcome, welcomelen);
       } else {
         abAppend(ab, "~", 1);
       }
     } else {
+      int len = E.row[filerow].rsize - E.coloff;
       if (len < 0) len = 0;
       if (len > E.screencols) len = E.screencols;
+      abAppend(ab, &E.row[filerow].render[E.coloff], len);
     }
 
     abAppend(ab, "\x1b[K", 3);
     if (y < E.screenrows - 1) {
       abAppend(ab, "\r\n", 2);
     }
   }
 }
 
 void editorRefreshScreen() {
   …
 }
 
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

これでテキストビューアは`render`内の文字を表示するようになりました。次に、タブを複数のスペース文字としてレンダリングするコードを`editorUpdateRow()`に追加しましょう。

**ステップ 83** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/tabs/kilo.c) / [tabs](https://github.com/snaptoken/kilo-src/tree/tabs))

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
 
 void editorUpdateRow(erow *row) {
+  int tabs = 0;
   int j;
+  for (j = 0; j < row->size; j++)
+    if (row->chars[j] == '\t') tabs++;
+
+  free(row->render);
+  row->render = malloc(row->size + tabs*7 + 1);
+
   int idx = 0;
   for (j = 0; j < row->size; j++) {
+    if (row->chars[j] == '\t') {
+      row->render[idx++] = ' ';
+      while (idx % 8 != 0) row->render[idx++] = ' ';
+    } else {
+      row->render[idx++] = row->chars[j];
+    }
   }
   row->render[idx] = '\0';
   row->rsize = idx;
 }
 
 void editorAppendRow(char *s, size_t len) {
   …
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

まず、行の `chars` をループしてタブの数を数え、`render` に割り当てるメモリ量を把握する必要があります。各タブに必要な最大文字数は 8 です。`row->size` は既に各タブに 1 をカウントしているので、タブの数に 7 を掛けて `row->size` に加算し、レンダリングされた行に必要な最大メモリ量を取得します。

メモリを割り当てた後、`for`ループを修正して、現在の文字がタブ文字かどうかを確認します。タブ文字の場合は、スペースを1つ追加し（各タブ文字はカーソルを少なくとも1列進める必要があるため）、タブストップ（8で割り切れる列）に到達するまでスペースを追加し続けます。

この時点で、タブストップの長さを定数にするべきだろう。

**ステップ 84** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/tab-stop/kilo.c) / [tab-stop](https://github.com/snaptoken/kilo-src/tree/tab-stop))

```diff
 /*** 含まれるもの ***/
 …
 /*** 定義 ***/
 
 #define KILO_VERSION "0.0.1"
+#define KILO_TAB_STOP 8
 
 #define CTRL_KEY(k) ((k) & 0x1f)
 
 enum editorKey {
   …
 };
 
 /*** データ ***/
 …
 /*** ターミナル ***/
 …
 /*** 行操作 ***/
 
 void editorUpdateRow(erow *row) {
   int tabs = 0;
   int j;
   for (j = 0; j < row->size; j++)
     if (row->chars[j] == '\t') tabs++;
 
   free(row->render);
+  row->render = malloc(row->size + tabs*(KILO_TAB_STOP - 1) + 1);
 
   int idx = 0;
   for (j = 0; j < row->size; j++) {
     if (row->chars[j] == '\t') {
       row->render[idx++] = ' ';
+      while (idx % KILO_TAB_STOP != 0) row->render[idx++] = ' ';
     } else {
       row->render[idx++] = row->chars[j];
     }
   }
   row->render[idx] = '\0';
   row->rsize = idx;
 }
 
 void editorAppendRow(char *s, size_t len) {
   …
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

*コンパイルはされるが、目に見える効果はない*

これによりコードがより分かりやすくなり、タブストップの長さも設定可能になります。

## タブとカーソル

カーソルは現在、タブとうまく連携しません。カーソルを画面上に配置する際、各文字が画面上の1列のみを占めると想定しているためです。これを修正するために、新しい水平座標変数`E.rx`を導入します。`E.cx`は`erow`の`chars`フィールドへのインデックスですが、`E.rx`変数は`render`フィールドへのインデックスになります。現在の行にタブがない場合、`E.rx`は`E.cx`と同じになります。タブがある場合、`E.rx`は、レンダリング時にタブが占める余分なスペース分だけ`E.cx`よりも大きくなります。

まず、グローバル状態構造体に`rx`を追加し、それを`0`で初期化します。

**ステップ 85** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/rx/kilo.c) / [rx](https://github.com/snaptoken/kilo-src/tree/rx))

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
+  int rx;
   int rowoff;
   int coloff;
   int screenrows;
   int screencols;
   int numrows;
   erow *row;
   struct termios orig_termios;
 };
 
 struct editorConfig E;
 
 /*** 端末 ***/
 …
 /*** 行操作 ***/
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
+  E.rx = 0;
   E.rowoff = 0;
   E.coloff = 0;
   E.numrows = 0;
   E.row = NULL;
 
   if (getWindowSize(&E.screenrows, &E.screencols) == -1) die("getWindowSize");
 }
 
 int main(int argc, char *argv[]) {
   …
   }
```

*コンパイルはされるが、目に見える効果はない*

`editorScroll()` 関数の先頭で `E.rx` の値を設定します。今のところは `E.cx` と同じ値に設定します。その後、スクロール処理では実際に画面に表示される文字とカーソルの位置を考慮する必要があるため、`editorScroll()` 関数内の `E.cx` をすべて `E.rx` に置き換えます。

**ステップ 86** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/rx-scroll/kilo.c) / [rx-scroll](https://github.com/snaptoken/kilo-src/tree/rx-scroll))

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
 /*** ファイル入出力 ***/
 …
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 
 void editorScroll() {
+  E.rx = E.cx;
+
   if (E.cy < E.rowoff) {
     E.rowoff = E.cy;
   }
   if (E.cy >= E.rowoff + E.screenrows) {
     E.rowoff = E.cy - E.screenrows + 1;
   }
+  if (E.rx < E.coloff) {
+    E.coloff = E.rx;
   }
+  if (E.rx >= E.coloff + E.screencols) {
+    E.coloff = E.rx - E.screencols + 1;
   }
 }
 
 void editorDrawRows(struct abuf *ab) {
   …
 }
 
 void editorRefreshScreen() {
   …
 }
 
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

カーソル位置を設定する `editorRefreshScreen()` 関数内で、`E.cx` を `E.rx` に変更してください。

**ステップ 87** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/use-rx/kilo.c) / [use-rx](https://github.com/snaptoken/kilo-src/tree/use-rx))

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
 /*** ファイル入出力 ***/
 …
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 
 void editorScroll() {
   …
 }
 
 void editorDrawRows(struct abuf *ab) {
   …
 }
 
 void editorRefreshScreen() {
   editorScroll();
 
   struct abuf ab = ABUF_INIT;
 
   abAppend(&ab, "\x1b[?25l", 6);
   abAppend(&ab, "\x1b[H", 3);
 
   editorDrawRows(&ab);
 
   char buf[32];
   snprintf(buf, sizeof(buf), "\x1b[%d;%dH", (E.cy - E.rowoff) + 1,
+                                            (E.rx - E.coloff) + 1);
   abAppend(&ab, buf, strlen(buf));
 
   abAppend(&ab, "\x1b[?25h", 6);
 
   write(STDOUT_FILENO, ab.b, ab.len);
   abFree(&ab);
 }
 
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

あとは、`editorScroll()` で `E.rx` の値を正しく計算するだけです。`chars` インデックスを `render` インデックスに変換する `editorRowCxToRx()` 関数を作成しましょう。`cx` の左側にあるすべての文字をループ処理し、各タブが占めるスペースの数を計算する必要があります。

**ステップ 88** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/cx-to-rx/kilo.c) / [cx-to-rx](https://github.com/snaptoken/kilo-src/tree/cx-to-rx))

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
 
+int editorRowCxToRx(erow *row, int cx) {
+  int rx = 0;
+  int j;
+  for (j = 0; j < cx; j++) {
+    if (row->chars[j] == '\t')
+      rx += (KILO_TAB_STOP - 1) - (rx % KILO_TAB_STOP);
+    rx++;
+  }
+  return rx;
+}
+
 void editorUpdateRow(erow *row) {
   …
 }
 
 void editorAppendRow(char *s, size_t len) {
   …
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

*コンパイルはされるが、目に見える効果はない*

各文字について、それがタブ文字の場合は、`rx % KILO_TAB_STOP` を使用して最後のタブストップから右に何列離れているかを調べ、次に `KILO_TAB_STOP - 1` からその値を減算して、次のタブストップから左に何列離れているかを調べます。その値を `rx` に加算して次のタブストップのすぐ左に移動し、無条件の `rx++` ステートメントによって次のタブストップに正確に移動します。現在タブストップ上にいる場合でも、この処理が機能することに注目してください。

最終的に `E.rx` を適切な値に設定するために、`editorScroll()` の先頭で `editorRowCxToRx()` を呼び出しましょう。

**ステップ 89** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/set-rx/kilo.c) / [set-rx](https://github.com/snaptoken/kilo-src/tree/set-rx))

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
 /*** ファイル入出力 ***/
 …
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 
 void editorScroll() {
+  E.rx = 0;
+  if (E.cy < E.numrows) {
+    E.rx = editorRowCxToRx(&E.row[E.cy], E.cx);
+  }
 
   if (E.cy < E.rowoff) {
     E.rowoff = E.cy;
   }
   if (E.cy >= E.rowoff + E.screenrows) {
     E.rowoff = E.cy - E.screenrows + 1;
   }
   if (E.rx < E.coloff) {
     E.coloff = E.rx;
   }
   if (E.rx >= E.coloff + E.screencols) {
     E.coloff = E.rx - E.screencols + 1;
   }
 }
 
 void editorDrawRows(struct abuf *ab) {
   …
 }
 
 void editorRefreshScreen() {
   …
 }
 
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイル*

これで、タブを含む行内でカーソルが正しく移動することを確認できるはずです。

## Page Up キーと Page Down キーによるスクロール

スクロール機能が実装できたので、次はPage UpキーとPage Downキーでページ全体を上下にスクロールできるようにしましょう。

**ステップ 90** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/page-up-down/kilo.c) / [page-up-down](https://github.com/snaptoken/kilo-src/tree/page-up-down))

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
       E.cx = E.screencols - 1;
       break;
 
     case PAGE_UP:
     case PAGE_DOWN:
       {
+        if (c == PAGE_UP) {
+          E.cy = E.rowoff;
+        } else if (c == PAGE_DOWN) {
+          E.cy = E.rowoff + E.screenrows - 1;
+          if (E.cy > E.numrows) E.cy = E.numrows;
+        }
+
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
   }
 }
 
 /*** init ***/
```

*コンパイル*

ページを上下にスクロールするには、カーソルを画面の上部または下部に配置し、画面全体分の↑キーまたは↓キーの押下をシミュレートします。`editorMoveCursor()`に処理を委譲することで、カーソル移動時に必要な境界チェックとカーソル位置の修正がすべて自動的に行われます。

## End を使用して行末に移動する

それでは、Endキーを押すとカーソルが現在の行の末尾に移動するように設定しましょう。（Homeキーは既にカーソルを行の先頭に移動するように設定されています。これは、`E.cx`を画面ではなくファイルからの相対パスとして作成したためです。）

**ステップ 91** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/end-key/kilo.c) / [end-key](https://github.com/snaptoken/kilo-src/tree/end-key))

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
+      if (E.cy < E.numrows)
+        E.cx = E.row[E.cy].size;
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
   }
 }
 
 /*** init ***/
```

*コンパイル*

Endキーを押すと、カーソルが現在の行の末尾に移動します。現在の行がない場合は、`E.cx`は`0`である必要があり、`0`のままなので、何もする必要はありません。

## ステータスバー

最後にテキスト編集機能を追加する前に、ステータスバーを実装します。このステータスバーには、ファイル名、ファイル内の行数、現在表示している行など、便利な情報が表示されます。後ほど、ファイルが最後に保存されてから変更されたかどうかを示すマーカーを追加し、構文ハイライトを実装する際にはファイルの種類も表示するようにします。

まず、画面下部に1行のステータスバーを表示するスペースを確保します。

**ステップ 92** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/status-bar-make-room/kilo.c) / [status-bar-make-room](https://github.com/snaptoken/kilo-src/tree/status-bar-make-room))

```diff
     }
 
     abAppend(ab, "\x1b[K", 3);
+    abAppend(ab, "\r\n", 2);
   }
 }
 
 …
   E.row = NULL;
 
   if (getWindowSize(&E.screenrows, &E.screencols) == -1) die("getWindowSize");
+  E.screenrows -= 1;
 }
 
 int main(int argc, char *argv[]) {
```

*コンパイル*

`E.screenrows`をデクリメントすることで、`editorDrawRows()`が画面下部にテキスト行を描画しようとしないようにします。また、ステータスバーが画面に描画される最後の行となるため、`editorDrawRows()`が描画した最後の行の後に改行を出力するようにします。

これら2つの変更を加えることで、テキストビューアがスクロールやカーソル移動を含めて正常に動作し、ステータスバーが表示される最後の行は、表示コードの残りの部分から変更されないことに注目してください。

ステータスバーを目立たせるために、反転色で表示します。つまり、白の背景に黒のテキストを表示します。エスケープシーケンス`<esc>[7m`で反転色に切り替わり、`<esc>[m`で通常の書式に戻ります。反転したスペース文字で構成された、空白の白いステータスバーを描画してみましょう。

**ステップ 93** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/blank-status-bar/kilo.c) / [blank-status-bar](https://github.com/snaptoken/kilo-src/tree/blank-status-bar))

```diff
 /*** 含まれるもの ***/
 …
 /*** 定義するもの ***/
 …
 /*** データ ***/
 …
 /*** 端末 ***/
 …
 /*** 行操作 ***/
 …
 /*** ファイル入出力 ***/
 …
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 
 void editorScroll() {
   …
 }
 
 void editorDrawRows(struct abuf *ab) {
   …
 }
 
+void editorDrawStatusBar(struct abuf *ab) {
+  abAppend(ab, "\x1b[7m", 4);
+  int len = 0;
+  while (len < E.screencols) {
+    abAppend(ab, " ", 1);
+    len++;
+  }
+  abAppend(ab, "\x1b[m", 3);
+}
+
 void editorRefreshScreen() {
   editorScroll();
 
   struct abuf ab = ABUF_INIT;
 
   abAppend(&ab, "\x1b[?25l", 6);
   abAppend(&ab, "\x1b[H", 3);
 
   editorDrawRows(&ab);
+  editorDrawStatusBar(&ab);
 
   char buf[32];
   snprintf(buf, sizeof(buf), "\x1b[%d;%dH", (E.cy - E.rowoff) + 1,
                                             (E.rx - E.coloff) + 1);
   abAppend(&ab, buf, strlen(buf));
 
   abAppend(&ab, "\x1b[?25h", 6);
 
   write(STDOUT_FILENO, ab.b, ab.len);
   abFree(&ab);
 }
 
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイル*

`m` コマンド ([Select Graphic Rendition](http://vt100.net/docs/vt100-ug/chapter3.html#SGR)) は、その後に印刷されるテキストに、太字 (`1`)、アンダースコア (`4`)、点滅 (`5`)、反転色 (`7`) など、さまざまな属性を適用します。たとえば、`<esc>[1;4;5;7m` コマンドを使用すると、これらの属性をすべて指定できます。引数に `0` を指定すると、すべての属性がクリアされ、これがデフォルトの引数となるため、通常のテキスト書式に戻すには `<esc>[m` を使用します。

ステータスバーにファイル名を表示したいので、`filename`という文字列をグローバルエディタの状態に追加し、ファイルが開かれたときにファイル名のコピーをそこに保存しましょう。

**ステップ 94** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/filename/kilo.c) / [filename](https://github.com/snaptoken/kilo-src/tree/filename))

```diff
 /*** 含まれる ***/
 …
 /*** 定義される ***/
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
+  char *filename;
   struct termios orig_termios;
 };
 
 struct editorConfig E;
 
 /*** 端末 ***/
 …
 /*** 行操作 ***/
 …
 /*** ファイル入出力 ***/
 
 void editorOpen(char *filename) {
+  free(E.filename);
+  E.filename = strdup(filename);
+
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
 }
 
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
+  E.filename = NULL;
 
   if (getWindowSize(&E.screenrows, &E.screencols) == -1) die("getWindowSize");
   E.screenrows -= 1;
 }
 
 int main(int argc, char *argv[]) {
   …
   }
```

*コンパイルはされるが、目に見える効果はない*

`strdup()` は `<string.h>` から派生した関数です。指定された文字列のコピーを作成し、必要なメモリを割り当て、そのメモリを `free()` で解放することを前提としています。

`E.filename` は `NULL` ポインタで初期化され、ファイルが開かれない場合（引数なしでプログラムを実行した場合に発生する状況）は `NULL` のままになります。

これでステータスバーに情報を表示する準備が整いました。ファイル名を最大20文字表示し、その後にファイルの行数を表示します。ファイル名がない場合は、「[ファイル名なし]」と表示します。

**ステップ 95** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/status-bar-left/kilo.c) / [status-bar-left](https://github.com/snaptoken/kilo-src/tree/status-bar-left))

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
 /*** ファイル入出力 ***/
 …
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 
 void editorScroll() {
   …
 }
 
 void editorDrawRows(struct abuf *ab) {
   …
 }
 
 void editorDrawStatusBar(struct abuf *ab) {
   abAppend(ab, "\x1b[7m", 4);
+  char status[80];
+  int len = snprintf(status, sizeof(status), "%.20s - %d lines",
+    E.filename ? E.filename : "[No Name]", E.numrows);
+  if (len > E.screencols) len = E.screencols;
+  abAppend(ab, status, len);
   while (len < E.screencols) {
     abAppend(ab, " ", 1);
     len++;
   }
   abAppend(ab, "\x1b[m", 3);
 }
 
 void editorRefreshScreen() {
   …
 }
 
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイル*

ウィンドウの幅に収まらない場合に備えて、ステータス文字列を短く切り詰めるようにしています。また、画面の端まで余白を描画するコードも引き続き使用しているため、ステータスバー全体が白い背景になります。

それでは、現在の行番号を表示し、画面の右端に揃えましょう。

**ステップ 96** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/status-bar-right/kilo.c) / [status-bar-right](https://github.com/snaptoken/kilo-src/tree/status-bar-right))

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
 /*** ファイル入出力 ***/
 …
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 
 void editorScroll() {
   …
 }
 
 void editorDrawRows(struct abuf *ab) {
   …
 }
 
 void editorDrawStatusBar(struct abuf *ab) {
   abAppend(ab, "\x1b[7m", 4);
+  char status[80], rstatus[80];
   int len = snprintf(status, sizeof(status), "%.20s - %d lines",
     E.filename ? E.filename : "[No Name]", E.numrows);
+  int rlen = snprintf(rstatus, sizeof(rstatus), "%d/%d",
+    E.cy + 1, E.numrows);
   if (len > E.screencols) len = E.screencols;
   abAppend(ab, status, len);
   while (len < E.screencols) {
+    if (E.screencols - len == rlen) {
+      abAppend(ab, rstatus, rlen);
+      break;
+    } else {
+      abAppend(ab, " ", 1);
+      len++;
+    }
   }
   abAppend(ab, "\x1b[m", 3);
 }
 
 void editorRefreshScreen() {
   …
 }
 
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイル*

現在の行は`E.cy`に格納されます。`E.cy`は0から始まるインデックスなので、この値に`1`を加算します。最初のステータス文字列を出力した後、2番目のステータス文字列を出力した場合に画面の右端に当たるまで、スペースを出力し続けます。これは、`E.screencols - len`が2番目のステータス文字列の長さと等しくなったときに発生します。その時点でステータス文字列を出力し、ループを抜けます。これでステータスバー全体が出力されたことになります。

## ステータスメッセージ

ステータスバーの下に、もう1行追加します。これは、ユーザーにメッセージを表示したり、検索時などにユーザーに入力を促したりするために使用します。現在のメッセージは、`statusmsg`という文字列に格納し、グローバルエディタの状態として保存します。また、メッセージのタイムスタンプも保存し、表示後数秒で削除できるようにします。

**ステップ 97** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/status-message/kilo.c) / [status-message](https://github.com/snaptoken/kilo-src/tree/status-message))

```diff
 /*** インクルード ***/
 
 #define _DEFAULT_SOURCE
 #define _BSD_SOURCE
 #define _GNU_SOURCE
 
 #include <ctype.h>
 #include <errno.h>
 #include <stdio.h>
 #include <stdlib.h>
 #include <string.h>
 #include <sys/ioctl.h>
 #include <sys/types.h>
 #include <termios.h>
+#include <time.h>
 #include <unistd.h>
 
 /*** 定義 ***/
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
   char *filename;
+  char statusmsg[80];
+  time_t statusmsg_time;
   struct termios orig_termios;
 };
 
 struct editorConfig E;
 
 /*** ターミナル ***/
 …
 /*** 行操作 ***/
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
   E.filename = NULL;
+  E.statusmsg[0] = '\0';
+  E.statusmsg_time = 0;
 
   if (getWindowSize(&E.screenrows, &E.screencols) == -1) die("getWindowSize");
   E.screenrows -= 1;
 }
 
 int main(int argc, char *argv[]) {
   …
   }
```

*コンパイルはされるが、目に見える効果はない*

`time_t` は `<time.h>` から取得されます。

`E.statusmsg`は空の文字列で初期化されるため、デフォルトではメッセージは表示されません。`E.statusmsg_time`には、ステータスメッセージを設定した時点のタイムスタンプが格納されます。

`editorSetStatusMessage()` 関数を定義しましょう。この関数は、 `printf()` 関数群と同様に、フォーマット文字列と可変個の引数を受け取ります。

**ステップ 98** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/set-status-message/kilo.c) / [set-status-message](https://github.com/snaptoken/kilo-src/tree/set-status-message))

```diff
 /*** インクルード ***/
 
 #define _DEFAULT_SOURCE
 #define _BSD_SOURCE
 #define _GNU_SOURCE
 
 #include <ctype.h>
 #include <errno.h>
 #include <stdio.h>
+#include <stdarg.h>
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
 /*** ファイル入出力 ***/
 …
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 
 void editorScroll() {
   …
 }
 
 void editorDrawRows(struct abuf *ab) {
   …
 }
 
 void editorDrawStatusBar(struct abuf *ab) {
   …
 }
 
 void editorRefreshScreen() {
   …
 }
 
+void editorSetStatusMessage(const char *fmt, ...) {
+  va_list ap;
+  va_start(ap, fmt);
+  vsnprintf(E.statusmsg, sizeof(E.statusmsg), fmt, ap);
+  va_end(ap);
+  E.statusmsg_time = time(NULL);
+}
+
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
 
+  editorSetStatusMessage("HELP: Ctrl-Q = quit");
+
   while (1) {
     editorRefreshScreen();
     editorProcessKeypress();
   }
 
   return 0;
 }
```

*コンパイルはされるが、目に見える効果はない*

`va_list`、`va_start()`、`va_end()`は`<stdarg.h>`から取得されます。`vsnprintf()`は`<stdio.h>`から取得されます。`time()`は`<time.h>`から取得されます。

`main()` 関数では、初期ステータスメッセージを、テキストエディタが使用するキーバインディング（現在は Ctrl+Q で終了）を含むヘルプメッセージに設定します。

`vsnprintf()` を使うと、独自の `printf()` スタイルの関数を作成できます。結果の文字列を `E.statusmsg` に格納し、`E.statusmsg_time` には現在の時刻を設定します。現在の時刻は `time()` に `NULL` を渡すことで取得できます。（この関数は、[1970 年 1 月 1 日午前 1 時](https://en.wikipedia.org/wiki/Unix_time) からの経過秒数を整数として返します。）

`...` 引数により、`editorSetStatusMessage()` は [可変引数関数](https://en.wikipedia.org/wiki/Variadic_function) となり、任意の数の引数を取ることができます。C 言語では、これらの引数を扱うために、型 `va_list` の値に対して `va_start()` と `va_end()` を呼び出します。`...` の前の最後の引数 (この場合は `fmt`) を `va_start()` に渡すことで、次の引数のアドレスがわかります。次に、`va_start()` と `va_end()` の呼び出しの間に `va_arg()` を呼び出し、次の引数の型 (通常は指定されたフォーマット文字列から取得) を渡すと、その引数の値が返されます。この場合、`fmt`と`ap`を`vsnprintf()`に渡すと、フォーマット文字列の読み取りと、各引数を取得するための`va_arg()`の呼び出しが処理されます。

表示するステータスメッセージができたので、ステータスバーの下に2行目のスペースを確保して、そこにメッセージを表示させましょう。

**ステップ 99** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/message-bar-make-room/kilo.c) / [message-bar-make-room](https://github.com/snaptoken/kilo-src/tree/message-bar-make-room))

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
 /*** ファイル入出力 ***/
 …
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 
 void editorScroll() {
   …
 }
 
 void editorDrawRows(struct abuf *ab) {
   …
 }
 
 void editorDrawStatusBar(struct abuf *ab) {
   abAppend(ab, "\x1b[7m", 4);
   char status[80], rstatus[80];
   int len = snprintf(status, sizeof(status), "%.20s - %d lines",
     E.filename ? E.filename : "[No Name]", E.numrows);
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
+  abAppend(ab, "\r\n", 2);
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
 
 void initEditor() {
   E.cx = 0;
   E.cy = 0;
   E.rx = 0;
   E.rowoff = 0;
   E.coloff = 0;
   E.numrows = 0;
   E.row = NULL;
   E.filename = NULL;
   E.statusmsg[0] = '\0';
   E.statusmsg_time = 0;
 
   if (getWindowSize(&E.screenrows, &E.screencols) == -1) die("getWindowSize");
+  E.screenrows -= 2;
 }
 
 int main(int argc, char *argv[]) {
   …
   }
```

*コンパイル*

`E.screenrows`を再度デクリメントし、最初のステータスバーの後に改行を出力します。これで、最後の行が再び空白になります。

新しい関数 `editorDrawMessageBar()` でメッセージバーを描画しましょう。

**ステップ 100** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/draw-message-bar/kilo.c) / [draw-message-bar](https://github.com/snaptoken/kilo-src/tree/draw-message-bar))

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
 /*** ファイル入出力 ***/
 …
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 
 void editorScroll() {
   …
 }
 
 void editorDrawRows(struct abuf *ab) {
   …
 }
 
 void editorDrawStatusBar(struct abuf *ab) {
   …
 }
 
+void editorDrawMessageBar(struct abuf *ab) {
+  abAppend(ab, "\x1b[K", 3);
+  int msglen = strlen(E.statusmsg);
+  if (msglen > E.screencols) msglen = E.screencols;
+  if (msglen && time(NULL) - E.statusmsg_time < 5)
+    abAppend(ab, E.statusmsg, msglen);
+}
+
 void editorRefreshScreen() {
   editorScroll();
 
   struct abuf ab = ABUF_INIT;
 
   abAppend(&ab, "\x1b[?25l", 6);
   abAppend(&ab, "\x1b[H", 3);
 
   editorDrawRows(&ab);
   editorDrawStatusBar(&ab);
+  editorDrawMessageBar(&ab);
 
   char buf[32];
   snprintf(buf, sizeof(buf), "\x1b[%d;%dH", (E.cy - E.rowoff) + 1,
                                             (E.rx - E.coloff) + 1);
   abAppend(&ab, buf, strlen(buf));
 
   abAppend(&ab, "\x1b[?25h", 6);
 
   write(STDOUT_FILENO, ab.b, ab.len);
   abFree(&ab);
 }
 
 void editorSetStatusMessage(const char *fmt, ...) {
   …
 }
 
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイル*

まず、エスケープシーケンス「<esc>[K」を使ってメッセージバーをクリアします。次に、メッセージが画面の幅に収まることを確認してから、メッセージを表示します。ただし、メッセージは5秒以内に送信されたものに限ります。

プログラムを起動すると、画面下部にヘルプメッセージが表示されます。5秒後にキーを押すと、このメッセージは消えます。なお、画面の更新はキーを押すたびに行われますのでご注意ください。

次の章では、テキストビューアをテキストエディタに変え、ユーザーが文字を挿入したり削除したり、変更内容をディスクに保存できるようにします。

[ページの先頭](04_a_text_viewer.md)
