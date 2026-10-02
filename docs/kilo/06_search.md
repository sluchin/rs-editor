# 6. 検索

`editorPrompt()` を使って、最小限の検索機能を実装してみましょう。ユーザーが検索クエリを入力して Enter キーを押すと、ファイルのすべての行をループ処理し、行にクエリ文字列が含まれている場合は、一致する箇所にカーソルを移動します。

**ステップ 131** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/basic-search/kilo.c) / [basic-search](https://github.com/snaptoken/kilo-src/tree/basic-search))

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
   …
     }
 …
+/*** 検索 ***/
+
+void editorFind() {
+  char *query = editorPrompt("Search: %s (ESC to cancel)");
+  if (query == NULL) return;
+
+  int i;
+  for (i = 0; i < E.numrows; i++) {
+    erow *row = &E.row[i];
+    char *match = strstr(row->render, query);
+    if (match) {
+      E.cy = i;
+      E.cx = match - row->render;
+      E.rowoff = E.numrows;
+      break;
+    }
+  }
+
+  free(query);
+}
+
 /*** バッファを追加 ***/
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

`strstr()` は `<string.h>` から来ています。

入力プロンプトをキャンセルするためにEscキーを押した場合、`editorPrompt()`は`NULL`を返し、検索を中止します。

それ以外の場合は、ファイルのすべての行をループします。`strstr()` を使用して、`query` が現在の行の部分文字列であるかどうかを確認します。一致するものがない場合は `NULL` を返し、一致する場合は一致する部分文字列へのポインタを返します。これを `E.cx` に設定できるインデックスに変換するには、`match` が `row->render` 文字列へのポインタであるため、`match` ポインタから `row->render` ポインタを減算します。最後に、`E.rowoff` を設定してファイルの一番下までスクロールします。これにより、次の画面更新時に `editorScroll()` が上方向にスクロールし、一致する行が画面の一番上に表示されます。このようにして、ユーザーはカーソルが移動した場所や一致する行を探すために画面全体を見渡す必要がなくなります。

ここに問題が1つあります。今、何が間違っていたかお気づきでしょうか？ `render` インデックスを `E.cx` に割り当てましたが、`E.cx` は `chars` のインデックスです。一致するものの左側にタブがある場合、カーソルは間違った位置になります。`render` インデックスを `chars` インデックスに変換してから `E.cx` に割り当てる必要があります。そこで、[第 4 章](04_a_text_viewer.md#タブとカーソル) で書いた `editorRowCxToRx()` 関数の逆の `editorRowRxToCx()` 関数を作成しましょう。この関数には、同じコードが多数含まれています。

**ステップ 132** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/rx-to-cx/kilo.c) / [rx-to-cx](https://github.com/snaptoken/kilo-src/tree/rx-to-cx))

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
 
+int editorRowRxToCx(erow *row, int rx) {
+  int cur_rx = 0;
+  int cx;
+  for (cx = 0; cx < row->size; cx++) {
+    if (row->chars[cx] == '\t')
+      cur_rx += (KILO_TAB_STOP - 1) - (cur_rx % KILO_TAB_STOP);
+    cur_rx++;
+
+    if (cur_rx > rx) return cx;
+  }
+  return cx;
+}
+
 void editorUpdateRow(erow *row) {
   …
 }
 
 void editorInsertRow(int at, char *s, size_t len) {
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
 
 void editorRowAppendString(erow *row, char *s, size_t len) {
   …
 }
 
 void editorRowDelChar(erow *row, int at) {
   …
 }
 
 /*** エディタ操作 ***/
 …
 /*** ファイル入出力 ***/
 …
 /*** 検索 ***/
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

`rx` を `cx` に変換するには、逆方向の変換とほぼ同じことを行います。つまり、`chars` 文字列をループ処理し、その都度現在の `rx` 値 (`cur_rx`) を計算します。ただし、特定の `cx` 値に到達したときに停止して `cur_rx` を返すのではなく、`cur_rx` が指定された `rx` 値に到達したときに停止して `cx` を返すようにします。

最後にある `return` 文は、呼び出し元が範囲外の `rx` を指定した場合に備えてのもので、そのような事態は発生しないはずです。`for` ループ内の `return` 文は、`render` への有効なインデックスであるすべての `rx` 値を処理する必要があります。

それでは、`editorRowRxToCx()` を呼び出して、一致したインデックスを `chars` インデックスに変換し、それを `E.cx` に割り当てましょう。

**ステップ 133** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/use-rx-to-cx/kilo.c) / [use-rx-to-cx](https://github.com/snaptoken/kilo-src/tree/use-rx-to-cx))

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
 /*** find ***/
 
 void editorFind() {
   char *query = editorPrompt("Search: %s (ESC to cancel)");
   if (query == NULL) return;
 
   int i;
   for (i = 0; i < E.numrows; i++) {
     erow *row = &E.row[i];
     char *match = strstr(row->render, query);
     if (match) {
       E.cy = i;
+      E.cx = editorRowRxToCx(row, match - row->render);
       E.rowoff = E.numrows;
       break;
     }
   }
 
   free(query);
 }
 
 /*** append buffer ***/
 …
 /*** output ***/
 …
 /*** input ***/
 …
 /*** init ***/
```

*コンパイルはされるが、目に見える効果はない*

最後に、Ctrl-F を `editorFind()` 関数に割り当て、`main()` で設定したヘルプメッセージに追加しましょう。

**ステップ 134** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/ctrl-f/kilo.c) / [ctrl-f](https://github.com/snaptoken/kilo-src/tree/ctrl-f))

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
 /*** find ***/
 …
 /*** append buffer ***/
 …
 /*** output ***/
 …
 /*** input ***/
 
 char *editorPrompt(char *prompt) {
   …
 }
 
 void editorMoveCursor(int key) {
   …
 }
 
 void editorProcessKeypress() {
   static int quit_times = KILO_QUIT_TIMES;
 
   int c = editorReadKey();
 
   switch (c) {
     case '\r':
       editorInsertNewline();
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
 
+    case CTRL_KEY('f'):
+      editorFind();
+      break;
+
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
 
 void initEditor() {
   …
 }
 
 int main(int argc, char *argv[]) {
   enableRawMode();
   initEditor();
   if (argc >= 2) {
     editorOpen(argv[1]);
   }
 
+  editorSetStatusMessage(
+    "HELP: Ctrl-S = save | Ctrl-Q = quit | Ctrl-F = find");
 
   while (1) {
     editorRefreshScreen();
     editorProcessKeypress();
   }
 
   return 0;
 }
```

*コンパイル*

## インクリメンタル検索

それでは、検索機能をさらに高度化しましょう。インクリメンタル検索に対応させたいと考えています。つまり、ユーザーが検索クエリを入力するたびに、キーを押すたびにファイルが検索される仕組みです。

これを実装するために、`editorPrompt()` 関数がコールバック関数を引数として受け取るようにします。キーが押されるたびにこの関数を呼び出し、ユーザーが入力した現在の検索クエリと最後に押したキーを渡します。

**ステップ 135** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/prompt-callback/kilo.c) / [prompt-callback](https://github.com/snaptoken/kilo-src/tree/prompt-callback))

```diff
 /*** 含める ***/
 …
 /*** 定義 ***/
 …
 /*** データ ***/
 …
 /*** プロトタイプ ***/
 
 void editorSetStatusMessage(const char *fmt, ...);
 void editorRefreshScreen();
+char *editorPrompt(char *prompt, void (*callback)(char *, int));
 
 /*** 端末 ***/
 …
 /*** 行操作 ***/
 …
 /*** エディタ操作 ***/
 …
 /*** ファイル入出力 ***/
 …
 /*** 検索 ***/
 …
 /*** バッファの追加 ***/
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 
+char *editorPrompt(char *prompt, void (*callback)(char *, int)) {
   size_t bufsize = 128;
   char *buf = malloc(bufsize);
 
   size_t buflen = 0;
   buf[0] = '\0';
 
   while (1) {
     editorSetStatusMessage(prompt, buf);
     editorRefreshScreen();
 
     int c = editorReadKey();
     if (c == DEL_KEY || c == CTRL_KEY('h') || c == BACKSPACE) {
       if (buflen != 0) buf[--buflen] = '\0';
     } else if (c == '\x1b') {
       editorSetStatusMessage("");
+      if (callback) callback(buf, c);
       free(buf);
       return NULL;
     } else if (c == '\r') {
       if (buflen != 0) {
         editorSetStatusMessage("");
+        if (callback) callback(buf, c);
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
+
+    if (callback) callback(buf, c);
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

*コンパイルできません*

`if` 文では、呼び出し元がコールバックを使用しない場合に `NULL` を渡すことができます。これは、ユーザーにファイル名の入力を求める場合に該当しますので、その際には `editorPrompt()` に `NULL` を渡します。また、コードをコンパイルするために、`editorFind()` の `editorPrompt()` にも `NULL` を渡します。

**ステップ 136** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/null-callback/kilo.c) / [null-callback](https://github.com/snaptoken/kilo-src/tree/null-callback))

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
+    E.filename = editorPrompt("Save as: %s (ESC to cancel)", NULL);
     if (E.filename == NULL) {
       editorSetStatusMessage("Save aborted");
       return;
     }
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
 
 /*** find ***/
 
 void editorFind() {
+  char *query = editorPrompt("Search: %s (ESC to cancel)", NULL);
   if (query == NULL) return;
 
   int i;
   for (i = 0; i < E.numrows; i++) {
     erow *row = &E.row[i];
     char *match = strstr(row->render, query);
     if (match) {
       E.cy = i;
       E.cx = editorRowRxToCx(row, match - row->render);
       E.rowoff = E.numrows;
       break;
     }
   }
 
   free(query);
 }
 
 /*** バッファを追加 ***/
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

それでは、実際の検索コードを`editorFind()`から`editorFindCallback()`という関数に移動しましょう。これは当然、`editorPrompt()`のコールバック関数になります。

**ステップ 137** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/incremental-search/kilo.c) / [incremental-search](https://github.com/snaptoken/kilo-src/tree/incremental-search))

```diff
 /*** 含める ***/
 …
 /*** 定義する ***/
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
 …
 /*** 検索 ***/
 
+void editorFindCallback(char *query, int key) {
+  if (key == '\r' || key == '\x1b') {
+    return;
+  }
 
   int i;
   for (i = 0; i < E.numrows; i++) {
     erow *row = &E.row[i];
     char *match = strstr(row->render, query);
     if (match) {
       E.cy = i;
       E.cx = editorRowRxToCx(row, match - row->render);
       E.rowoff = E.numrows;
       break;
     }
   }
+}
 
+void editorFind() {
+  char *query = editorPrompt("Search: %s (ESC to cancel)", editorFindCallback);
+
+  if (query) {
+    free(query);
+  }
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

コールバック関数では、ユーザーがEnterキーまたはEscapeキーを押したかどうかを確認します。EnterキーまたはEscapeキーが押された場合は検索モードが終了するため、再度検索を実行せずにすぐに処理を終了します。それ以外の場合は、他のキーが押された後に、現在のクエリ文字列に対して再度検索を実行します。

以上です。これでインクリメンタル検索が実現しました。

## 検索をキャンセルした際にカーソル位置を復元する

ユーザーがEscキーを押して検索をキャンセルした場合、カーソルを検索開始時の位置に戻したい。そのためには、カーソル位置とスクロール位置を保存し、検索キャンセル後にそれらの値を復元する必要がある。

**ステップ 138** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/restore-cursor/kilo.c) / [restore-cursor](https://github.com/snaptoken/kilo-src/tree/restore-cursor))

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
 /*** find ***/
 
 void editorFindCallback(char *query, int key) {
   …
 }
 
 void editorFind() {
+  int saved_cx = E.cx;
+  int saved_cy = E.cy;
+  int saved_coloff = E.coloff;
+  int saved_rowoff = E.rowoff;
+
   char *query = editorPrompt("Search: %s (ESC to cancel)", editorFindCallback);
 
   if (query) {
     free(query);
+  } else {
+    E.cx = saved_cx;
+    E.cy = saved_cy;
+    E.coloff = saved_coloff;
+    E.rowoff = saved_rowoff;
   }
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

`query`が`NULL`の場合は、Escキーが押されたことを意味するので、その場合は保存した値を復元します。

## 前方検索と後方検索

最後に追加したい機能は、矢印キーを使ってファイル内の次のマッチまたは前のマッチに移動できるようにすることです。↑キーと←キーで前のマッチに移動し、↓キーと→キーで次のマッチに移動します。

この機能は、コールバック関数内で2つの静的変数を使用して実装します。`last_match`には、最後に一致した行のインデックスが格納されます。最後に一致がなかった場合は`-1`が格納されます。`direction`には検索方向が格納されます。前方検索の場合は`1`、後方検索の場合は`-1`が格納されます。

**ステップ 139** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/callback-statics/kilo.c) / [callback-statics](https://github.com/snaptoken/kilo-src/tree/callback-statics))

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
 /*** find ***/
 
 void editorFindCallback(char *query, int key) {
+  static int last_match = -1;
+  static int direction = 1;
+
   if (key == '\r' || key == '\x1b') {
+    last_match = -1;
+    direction = 1;
     return;
+  } else if (key == ARROW_RIGHT || key == ARROW_DOWN) {
+    direction = 1;
+  } else if (key == ARROW_LEFT || key == ARROW_UP) {
+    direction = -1;
+  } else {
+    last_match = -1;
+    direction = 1;
   }
 
   int i;
   for (i = 0; i < E.numrows; i++) {
     erow *row = &E.row[i];
     char *match = strstr(row->render, query);
     if (match) {
       E.cy = i;
       E.cx = editorRowRxToCx(row, match - row->render);
       E.rowoff = E.numrows;
       break;
     }
   }
 }
 
 void editorFind() {
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

ご覧のとおり、矢印キーが押されない限り、`last_match` は常に `-1` にリセットされます。そのため、矢印キーが押された場合にのみ、次のマッチまたは前のマッチに進みます。また、← キーまたは ↑ キーが押されない限り、`direction` は常に `1` に設定されていることもわかります。そのため、ユーザーが最後のマッチから逆方向に検索するように明示的に指示しない限り、常に順方向に検索が行われます。

`key`が`'\r'`（Enterキー）または`'\x1b'`（Escapeキー）の場合、検索モードを終了しようとしていることを意味します。そのため、次の検索操作に備えるために、`last_match`と`direction`を初期値にリセットします。

これで全ての変数が設定されたので、実際に使ってみましょう。

**ステップ 140** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/search-arrows/kilo.c) / [search-arrows](https://github.com/snaptoken/kilo-src/tree/search-arrows))

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
 /*** find ***/
 
 void editorFindCallback(char *query, int key) {
   static int last_match = -1;
   static int direction = 1;
 
   if (key == '\r' || key == '\x1b') {
     last_match = -1;
     direction = 1;
     return;
   } else if (key == ARROW_RIGHT || key == ARROW_DOWN) {
     direction = 1;
   } else if (key == ARROW_LEFT || key == ARROW_UP) {
     direction = -1;
   } else {
     last_match = -1;
     direction = 1;
   }
 
+  if (last_match == -1) direction = 1;
+  int current = last_match;
   int i;
   for (i = 0; i < E.numrows; i++) {
+    current += direction;
+    if (current == -1) current = E.numrows - 1;
+    else if (current == E.numrows) current = 0;
+
+    erow *row = &E.row[current];
     char *match = strstr(row->render, query);
     if (match) {
+      last_match = current;
+      E.cy = current;
       E.cx = editorRowRxToCx(row, match - row->render);
       E.rowoff = E.numrows;
       break;
     }
   }
 }
 
 void editorFind() {
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

*コンパイル*

`current` は、現在検索している行のインデックスです。前回の一致があった場合は、次の行（または、逆方向に検索している場合は前の行）から検索を開始します。前回の一致がなかった場合は、ファイルの先頭から開始し、順方向に検索して最初の一致を見つけます。

`if ... else if` は、`current` をファイルの末尾からファイルの先頭へ、またはその逆へ移動させ、検索がファイルの末尾を「回り込んで」先頭（または末尾）から再開できるようにします。

一致する結果が見つかったら、`last_match`を`current`に設定します。そうすることで、ユーザーが矢印キーを押した場合、その時点から次の検索が開始されます。

最後に、ユーザーが矢印キーを使用できることを知らせるために、プロンプトテキストを更新することを忘れないでください。

**ステップ 141** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/search-arrows-help/kilo.c) / [search-arrows-help](https://github.com/snaptoken/kilo-src/tree/search-arrows-help))

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
 /*** find ***/
 
 void editorFindCallback(char *query, int key) {
   …
 }
 
 void editorFind() {
   int saved_cx = E.cx;
   int saved_cy = E.cy;
   int saved_coloff = E.coloff;
   int saved_rowoff = E.rowoff;
 
+  char *query = editorPrompt("Search: %s (Use ESC/Arrows/Enter)",
+                             editorFindCallback);
 
   if (query) {
     free(query);
   } else {
     E.cx = saved_cx;
     E.cy = saved_cy;
     E.coloff = saved_coloff;
     E.rowoff = saved_rowoff;
   }
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

次の章では、構文ハイライトとファイルタイプの検出を実装して、テキストエディタを完成させます。
