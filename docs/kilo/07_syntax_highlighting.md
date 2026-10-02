# 7. 構文ハイライト

## カラフルな数字

まずは、できるだけシンプルな方法で画面に色を付けてみましょう。数字を赤色で強調表示してみます。

**ステップ 142** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/syntax-digits/kilo.c) / [syntax-digits](https://github.com/snaptoken/kilo-src/tree/syntax-digits))

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
       int len = E.row[filerow].rsize - E.coloff;
       if (len < 0) len = 0;
       if (len > E.screencols) len = E.screencols;
+      char *c = &E.row[filerow].render[E.coloff];
+      int j;
+      for (j = 0; j < len; j++) {
+        if (isdigit(c[j])) {
+          abAppend(ab, "\x1b[31m", 5);
+          abAppend(ab, &c[j], 1);
+          abAppend(ab, "\x1b[39m", 5);
+        } else {
+          abAppend(ab, &c[j], 1);
+        }
+      }
     }
 
     abAppend(ab, "\x1b[K", 3);
     abAppend(ab, "\r\n", 2);
   }
 }
 
 void editorDrawStatusBar(struct abuf *ab) {
   …
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

*コンパイル*

今後は、印刷したい `render` のサブストリングをそのまま `abAppend()` に渡すことはできません。今後は文字ごとに処理する必要があります。そこで、文字をループして、各文字に対して `isdigit()` を使用して、それが数字文字かどうかをテストします。数字文字の場合は、その前に `<esc>[31m` エスケープシーケンスを、後に `<esc>[39m` シーケンスを記述します。

以前は、`m` コマンド ([グラフィック表示の選択](http://vt100.net/docs/vt100-ug/chapter3.html#SGR)) を使用して、反転色でステータス バーを描画していました。今回は、テキストの色を設定するために使用します。[VT100 ユーザー ガイド](http://vt100.net/docs/vt100-ug/chapter3.html) には色に関する説明がないため、[ANSI エスケープ コード](https://en.wikipedia.org/wiki/ANSI_escape_code) に関する Wikipedia の記事を参照します。この記事には、さまざまな端末で `m` コマンドに使用できるさまざまな引数コードをすべて含む大きな表が含まれています。また、使用可能な 8 つの前景色/背景色を含む ANSI カラー テーブルも含まれています。

最初の表によると、テキストの色はコード「30」から「37」で設定でき、「39」でデフォルトの色にリセットできます。色の表では、「0」が黒、「1」が赤、といった具合に「7」まで続き、「7」が白です。これらを組み合わせると、「m」コマンドの引数として「31」を使用することで、テキストの色を赤に設定できます。数字を印刷した後、「m」コマンドの引数として「39」を使用することで、テキストの色を元の色に戻すことができます。

## リファクタリング構文のハイライト

これでテキストの色付け方法はわかりましたが、文字列全体、キーワード、コメントなどを実際に強調表示するには、さらに多くの作業が必要になります。現在数字に対して行っているように、各文字のクラスに基づいて使用する色を決定するだけではいけません。私たちがやりたいことは、テキストの各行を表示する前に強調表示を決定し、変更があった場合はその行を再強調表示することです。そのためには、各行の強調表示を配列に格納する必要があります。`erow`構造体に`hl`という名前の配列を追加しましょう。`hl`は「highlight」の略です。

**ステップ 143** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/syntax-refactoring/kilo.c) / [syntax-refactoring](https://github.com/snaptoken/kilo-src/tree/syntax-refactoring))

```diff
 /*** インクルード ***/
 …
 /*** 定義 ***/
 …
 /*** データ ***/
 
 typedef struct erow {
   int size;
   int rsize;
   char *chars;
   char *render;
+  unsigned char *hl;
 } erow;
 
 struct editorConfig {
   …
 };
 
 struct editorConfig E;
 
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 行操作 ***/
 
 int editorRowCxToRx(erow *row, int cx) {
   …
 }
 
 int editorRowRxToCx(erow *row, int rx) {
   …
 }
 
 void editorUpdateRow(erow *row) {
   …
 }
 
 void editorInsertRow(int at, char *s, size_t len) {
   if (at < 0 || at > E.numrows) return;
 
   E.row = realloc(E.row, sizeof(erow) * (E.numrows + 1));
   memmove(&E.row[at + 1], &E.row[at], sizeof(erow) * (E.numrows - at));
 
   E.row[at].size = len;
   E.row[at].chars = malloc(len + 1);
   memcpy(E.row[at].chars, s, len);
   E.row[at].chars[len] = '\0';
 
   E.row[at].rsize = 0;
   E.row[at].render = NULL;
+  E.row[at].hl = NULL;
   editorUpdateRow(&E.row[at]);
 
   E.numrows++;
   E.dirty++;
 }
 
 void editorFreeRow(erow *row) {
   free(row->render);
   free(row->chars);
+  free(row->hl);
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

`hl` は `unsigned char` 型の値の配列です。つまり、`0` から `255` の範囲の整数です。配列内の各値は `render` 内の文字に対応し、その文字が文字列の一部なのか、コメントなのか、数値なのかなどを示します。それでは、 `hl` 配列が取り得る値を含む `enum` を作成してみましょう。

**ステップ 144** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/highlight-enum/kilo.c) / [highlight-enum](https://github.com/snaptoken/kilo-src/tree/highlight-enum))

```diff
 /*** インクルード ***/
 …
 /*** 定義 ***/
 
 #define KILO_VERSION "0.0.1"
 #define KILO_TAB_STOP 8
 #define KILO_QUIT_TIMES 3
 
 #define CTRL_KEY(k) ((k) & 0x1f)
 
 enum editorKey {
   …
 };
 
+enum editorHighlight {
+  HL_NORMAL = 0,
+  HL_NUMBER
+};
+
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

今のところは、数字のみを強調表示することに焦点を当てます。つまり、数字の一部であるすべての文字には、`hl`配列内の対応する`HL_NUMBER`値を持たせ、`hl`内のその他の値はすべて`HL_NORMAL`にしたいと考えています。

新しい `/*** 構文ハイライト ***/` セクションを作成し、その中に `editorUpdateSyntax()` 関数を作成します。この関数は `erow` の文字を順に処理し、`hl` 配列の各値を設定することで文字をハイライト表示します。

**ステップ 145** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/editor-update-syntax/kilo.c) / [editor-update-syntax](https://github.com/snaptoken/kilo-src/tree/editor-update-syntax))

```diff
 /*** 含まれるもの ***/
 …
 /*** 定義するもの ***/
 …
 /*** データ ***/
 …
 /*** プロトタイプ ***/
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
+/*** 構文ハイライト ***/
+
+void editorUpdateSyntax(erow *row) {
+  row->hl = realloc(row->hl, row->rsize);
+  memset(row->hl, HL_NORMAL, row->rsize);
+
+  int i;
+  for (i = 0; i < row->rsize; i++) {
+    if (isdigit(row->render[i])) {
+      row->hl[i] = HL_NUMBER;
+    }
+  }
+}
+
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
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

`memset()` は `<string.h>` から来ています。

まず、必要なメモリを `realloc()` で割り当てます。これは新しい行である可能性や、前回ハイライトした時よりも行のサイズが大きくなっている可能性があるためです。`hl` 配列のサイズは `render` 配列と同じなので、`rsize` を `hl` に割り当てるメモリ量として使用します。

次に、`memset()` を使用して、デフォルトですべての文字を `HL_NORMAL` に設定してから、文字をループ処理して数字を `HL_NUMBER` に設定します。（ご安心ください。近いうちに数字を認識するためのより良い方法を実装する予定ですが、今はリファクタリングに注力しています。）

それでは実際に`editorUpdateSyntax()`を呼び出してみましょう。

**ステップ 146** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/call-update-syntax/kilo.c) / [call-update-syntax](https://github.com/snaptoken/kilo-src/tree/call-update-syntax))

```diff
 /*** 含める ***/
 …
 /*** 定義 ***/
 …
 /*** データ ***/
 …
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 構文強調表示 ***/
 …
 /*** 行操作 ***/
 
 int editorRowCxToRx(erow *row, int cx) {
   …
 }
 
 int editorRowRxToCx(erow *row, int rx) {
   …
 }
 
 void editorUpdateRow(erow *row) {
   int tabs = 0;
   int j;
   for (j = 0; j < row->size; j++)
     if (row->chars[j] == '\t') tabs++;
 
   free(row->render);
   row->render = malloc(row->size + tabs*(KILO_TAB_STOP - 1) + 1);
 
   int idx = 0;
   for (j = 0; j < row->size; j++) {
     if (row->chars[j] == '\t') {
       row->render[idx++] = ' ';
       while (idx % KILO_TAB_STOP != 0) row->render[idx++] = ' ';
     } else {
       row->render[idx++] = row->chars[j];
     }
   }
   row->render[idx] = '\0';
   row->rsize = idx;
+
+  editorUpdateSyntax(row);
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

`editorUpdateRow()` は、行のテキストが変更されるたびに `render` 配列を更新する役割を既に担っているため、`hl` 配列を更新するのもその場所で行うのが理にかなっています。したがって、`render` を更新した後、最後に `editorUpdateSyntax()` を呼び出します。

次に、`hl` の値を、実際に描画に使用したい ANSI カラーコードにマッピングする `editorSyntaxToColor()` 関数を作成しましょう。

**ステップ 147** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/map-colors/kilo.c) / [map-colors](https://github.com/snaptoken/kilo-src/tree/map-colors))

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
 /*** 構文ハイライト ***/
 
 void editorUpdateSyntax(erow *row) {
   …
 }
 
+int editorSyntaxToColor(int hl) {
+  switch (hl) {
+    case HL_NUMBER: return 31;
+    default: return 37;
+  }
+}
+
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
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

数値については「前景色赤」のANSIコード、それ以外の文字については「前景色白」のANSIコードを返します。（`HL_NORMAL`は別途処理するため、`editorSyntaxToColor()`で処理する必要はありません。）

それでは、最後にハイライトされたテキストを画面に描画してみましょう！

**ステップ 148** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/use-hl/kilo.c) / [use-hl](https://github.com/snaptoken/kilo-src/tree/use-hl))

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
 /*** syntax highlighting ***/
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
       int len = E.row[filerow].rsize - E.coloff;
       if (len < 0) len = 0;
       if (len > E.screencols) len = E.screencols;
       char *c = &E.row[filerow].render[E.coloff];
+      unsigned char *hl = &E.row[filerow].hl[E.coloff];
       int j;
       for (j = 0; j < len; j++) {
+        if (hl[j] == HL_NORMAL) {
           abAppend(ab, "\x1b[39m", 5);
+          abAppend(ab, &c[j], 1);
         } else {
+          int color = editorSyntaxToColor(hl[j]);
+          char buf[16];
+          int clen = snprintf(buf, sizeof(buf), "\x1b[%dm", color);
+          abAppend(ab, buf, clen);
           abAppend(ab, &c[j], 1);
         }
       }
+      abAppend(ab, "\x1b[39m", 5);
     }
 
     abAppend(ab, "\x1b[K", 3);
     abAppend(ab, "\r\n", 2);
   }
 }
 
 void editorDrawStatusBar(struct abuf *ab) {
   …
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

まず、`hl`配列のスライスのうち、印刷する`render`のスライスに対応するスライスへのポインタ`hl`を取得します。次に、各文字について、それが`HL_NORMAL`文字であれば、`<esc>[39m`を使用して、印刷前にデフォルトのテキストカラーが使用されていることを確認します。`HL_NORMAL`でない場合は、`snprintf()`を使用してエスケープシーケンスをバッファに書き込み、それを`abAppend()`に渡してから実際の文字を追加します。最後に、すべての文字をループして表示した後、テキストカラーがデフォルトにリセットされていることを確認するために、最後の`<esc>[39m`エスケープシーケンスを印刷します。

これは機能しますが、本当にすべての文字の前にエスケープシーケンスを記述する必要があるのでしょうか？実際には、ほとんどの文字は前の文字と同じ色なので、ほとんどのエスケープシーケンスは冗長です。文字をループ処理する際に現在のテキストの色を記録しておき、色が変わったときだけエスケープシーケンスを出力するようにしましょう。

**ステップ 149** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/current-color/kilo.c) / [current-color](https://github.com/snaptoken/kilo-src/tree/current-color))

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
 /*** syntax highlighting ***/
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
       int len = E.row[filerow].rsize - E.coloff;
       if (len < 0) len = 0;
       if (len > E.screencols) len = E.screencols;
       char *c = &E.row[filerow].render[E.coloff];
       unsigned char *hl = &E.row[filerow].hl[E.coloff];
+      int current_color = -1;
       int j;
       for (j = 0; j < len; j++) {
         if (hl[j] == HL_NORMAL) {
+          if (current_color != -1) {
+            abAppend(ab, "\x1b[39m", 5);
+            current_color = -1;
+          }
           abAppend(ab, &c[j], 1);
         } else {
           int color = editorSyntaxToColor(hl[j]);
+          if (color != current_color) {
+            current_color = color;
+            char buf[16];
+            int clen = snprintf(buf, sizeof(buf), "\x1b[%dm", color);
+            abAppend(ab, buf, clen);
+          }
           abAppend(ab, &c[j], 1);
         }
       }
       abAppend(ab, "\x1b[39m", 5);
     }
 
     abAppend(ab, "\x1b[K", 3);
     abAppend(ab, "\r\n", 2);
   }
 }
 
 void editorDrawStatusBar(struct abuf *ab) {
   …
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

`current_color` は、デフォルトのテキストカラーが必要な場合は `-1` になります。それ以外の場合は、`editorSyntaxToColor()` が最後に返した値に設定されます。色が変更されると、その色のエスケープシーケンスを出力し、`current_color` を新しい色に設定します。ハイライトされたテキストから `HL_NORMAL` テキストに戻ると、`<esc>[39m` エスケープシーケンスを出力し、`current_color` を `-1` に設定します。

これで構文強調表示システムの再設計は完了です。

## カラフルな検索結果

文字列やキーワードなどをハイライト表示する前に、ハイライト表示システムを使って検索結果をハイライト表示してみましょう。まず、`editorHighlight` 列挙型に `HL_MATCH` を追加し、`editorSyntaxToColor()` で青色 (`34`) にマッピングします。

**ステップ 150** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/hl-match/kilo.c) / [hl-match](https://github.com/snaptoken/kilo-src/tree/hl-match))

```diff
 /*** 含まれるもの ***/
 …
 /*** 定義 ***/
 
 #define KILO_VERSION "0.0.1"
 #define KILO_TAB_STOP 8
 #define KILO_QUIT_TIMES 3
 
 #define CTRL_KEY(k) ((k) & 0x1f)
 
 enum editorKey {
   …
 };
 
 enum editorHighlight {
   HL_NORMAL = 0,
+  HL_NUMBER,
+  HL_MATCH
 };
 
 /*** データ ***/
 …
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 構文ハイライト***/
 
 void editorUpdateSyntax(erow *row) {
   …
 }
 
 int editorSyntaxToColor(int hl) {
   switch (hl) {
     case HL_NUMBER: return 31;
+    case HL_MATCH: return 34;
     default: return 37;
   }
 }
 
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
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

あとは、検索コード内の `HL_MATCH` に一致した部分文字列を `memset()` するだけです。

**ステップ 151** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/search-highlighting/kilo.c) / [search-highlighting](https://github.com/snaptoken/kilo-src/tree/search-highlighting))

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
 /*** syntax highlighting ***/
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
 
   if (last_match == -1) direction = 1;
   int current = last_match;
   int i;
   for (i = 0; i < E.numrows; i++) {
     current += direction;
     if (current == -1) current = E.numrows - 1;
     else if (current == E.numrows) current = 0;
 
     erow *row = &E.row[current];
     char *match = strstr(row->render, query);
     if (match) {
       last_match = current;
       E.cy = current;
       E.cx = editorRowRxToCx(row, match - row->render);
       E.rowoff = E.numrows;
+
+      memset(&row->hl[match - row->render], HL_MATCH, strlen(query));
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

`match - row->render` はマッチの `render` へのインデックスなので、それを `hl` へのインデックスとして使用します。

## 検索後に構文ハイライトを復元する

現在、検索結果はユーザーが検索機能の使用を終えた後も青色でハイライト表示されたままになっています。そこで、検索後に`hl`を以前の値に戻したいと考えています。そのため、`editorFindCallback()`内で`hl`の元の内容を`saved_hl`という静的変数に保存し、コールバックの先頭で`hl`を`saved_hl`の内容に復元します。

**ステップ 152** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/restore-hl/kilo.c) / [restore-hl](https://github.com/snaptoken/kilo-src/tree/restore-hl))

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
 /*** 構文強調表示 ***/
 …
 /*** 行操作 ***/
 …
 /*** エディタ操作 ***/
 …
 /*** ファイル入出力 ***/
 …
 /*** 検索 ***/
 
 void editorFindCallback(char *query, int key) {
   static int last_match = -1;
   static int direction = 1;
 
+  static int saved_hl_line;
+  static char *saved_hl = NULL;
+
+  if (saved_hl) {
+    memcpy(E.row[saved_hl_line].hl, saved_hl, E.row[saved_hl_line].rsize);
+    free(saved_hl);
+    saved_hl = NULL;
+  }
+
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
 
   if (last_match == -1) direction = 1;
   int current = last_match;
   int i;
   for (i = 0; i < E.numrows; i++) {
     current += direction;
     if (current == -1) current = E.numrows - 1;
     else if (current == E.numrows) current = 0;
 
     erow *row = &E.row[current];
     char *match = strstr(row->render, query);
     if (match) {
       last_match = current;
       E.cy = current;
       E.cx = editorRowRxToCx(row, match - row->render);
       E.rowoff = E.numrows;
 
+      saved_hl_line = current;
+      saved_hl = malloc(row->rsize);
+      memcpy(saved_hl, row->hl, row->rsize);
       memset(&row->hl[match - row->render], HL_MATCH, strlen(query));
       break;
     }
   }
 }
 
 void editorFind() {
   …
   }
 …
 /***バッファを追加 ***/
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイル*

`saved_hl_line` という別の静的変数を使用して、どの行の `hl` を復元する必要があるかを特定します。`saved_hl` は動的に割り当てられる配列で、復元するものがない場合は `NULL` を指します。復元するものがある場合は、それを `memcpy()` で保存対象の行の `hl` にコピーし、その後 `saved_hl` を解放して `NULL` に戻します。

`malloc()`で確保されたメモリは必ず`free()`で解放されることに注意してください。ユーザーがEnterキーまたはEscapeキーを押して検索プロンプトを閉じると、`editorPrompt()`がコールバック関数を呼び出し、`editorPrompt()`が最終的に戻る前に`hl`を復元する機会が与えられるからです。また、`saved_hl`は関数の先頭で必ず`free()`で解放されるため、古い値が解放される前に`malloc()`で確保されることはあり得ません。最後に、`hl`の保存と復元の間にユーザーがファイルを編集することは不可能なので、`saved_hl_line`を`E.row`のインデックスとして安全に使用できます。（これらの点を考慮することが重要です。）

## カラフルな数字

それでは、数字を正しくハイライト表示する作業に取り掛かりましょう。まず、`editorUpdateSyntax()` 内の `for` ループを `while` ループに変更し、各イテレーションで複数の文字を処理できるようにします。（数字の場合は一度に 1 文字しか処理しませんが、これは後で役立ちます。）

**ステップ 153** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/syntax-while/kilo.c) / [syntax-while](https://github.com/snaptoken/kilo-src/tree/syntax-while))

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
 /*** 構文ハイライト ***/
 
 void editorUpdateSyntax(erow *row) {
   row->hl = realloc(row->hl, row->rsize);
   memset(row->hl, HL_NORMAL, row->rsize);
 
+  int i = 0;
+  while (i < row->rsize) {
+    char c = row->render[i];
+
+    if (isdigit(c)) {
       row->hl[i] = HL_NUMBER;
     }
+
+    i++;
   }
 }
 
 int editorSyntaxToColor(int hl) {
   …
   }
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
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

それでは、文字を受け取り、それが区切り文字とみなされる場合に true を返す `is_separator()` 関数を定義しましょう。

**ステップ 154** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/is-separator/kilo.c) / [is-separator](https://github.com/snaptoken/kilo-src/tree/is-separator))

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
 /*** 構文ハイライト ***/
 
+int is_separator(int c) {
+  return isspace(c) || c == '\0' || strchr(",.()+-/*=~%<>[];", c) != NULL;
+}
+
 void editorUpdateSyntax(erow *row) {
   …
 }
 
 int editorSyntaxToColor(int hl) {
   …
   }
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
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

`strchr()` は `<string.h>` から派生した関数です。文字列内で指定された文字が最初に出現する箇所を探し、その文字へのポインタを返します。文字列内にその文字が含まれていない場合、`strchr()` は `NULL` を返します。

現状では、`int32_t` の `32` のように、識別子の一部である数値も強調表示されてしまいます。これを修正するため、数値の前に区切り文字（空白文字や句読点など）を付けることを必須とします。また、ヌルバイト（`'\0'`）も区切り文字として扱うことで、各行末のヌルバイトを区切り文字としてカウントできるようにし、将来的にコードを簡略化できるようにします。

`editorUpdateSyntax()` 関数に、前の文字が区切り文字だったかどうかを記録する `prev_sep` 変数を追加しましょう。そして、それを使って数字を正しく認識し、強調表示します。

**ステップ 155** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/prev-sep/kilo.c) / [prev-sep](https://github.com/snaptoken/kilo-src/tree/prev-sep))

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
 /*** syntax highlighting ***/
 
 int is_separator(int c) {
   …
 }
 
 void editorUpdateSyntax(erow *row) {
   row->hl = realloc(row->hl, row->rsize);
   memset(row->hl, HL_NORMAL, row->rsize);
 
+  int prev_sep = 1;
+
   int i = 0;
   while (i < row->rsize) {
     char c = row->render[i];
+    unsigned char prev_hl = (i > 0) ? row->hl[i - 1] : HL_NORMAL;
 
+    if (isdigit(c) && (prev_sep || prev_hl == HL_NUMBER)) {
       row->hl[i] = HL_NUMBER;
+      i++;
+      prev_sep = 0;
+      continue;
     }
 
+    prev_sep = is_separator(c);
     i++;
   }
 }
 
 int editorSyntaxToColor(int hl) {
   …
   }
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
 …
 /*** 初期化 ***/
```

*コンパイル*

行頭を区切り文字とみなすため、`prev_sep`を`1`（つまりtrue）に初期化します。（そうしないと、行頭の数字が強調表示されません。）

`prev_hl`は、前の文字の強調表示タイプに設定されます。数字を`HL_NUMBER`で強調表示するには、前の文字が区切り文字であるか、または`HL_NUMBER`で強調表示されている必要があります。

現在の文字を特定の方法で強調表示することに決めたら（この場合は`HL_NUMBER`）、`i`をインクリメントしてその文字を「消費」し、`prev_sep`を`0`に設定して強調表示の途中であることを示し、ループを`continue`します。強調表示する各要素に対して、このパターンを使用します。

現在の文字をハイライトしない場合は、`while` ループの最後に戻り、現在の文字が区切り文字かどうかに応じて `prev_sep` を設定し、文字を消費するために `i` をインクリメントします。関数の先頭で行った `memset()` は、ハイライトされていない文字の `hl` の値が `HL_NORMAL` になることを意味します。

次に、小数点を含む数値を強調表示する機能をサポートしましょう。

**ステップ 156** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/decimal-point/kilo.c) / [decimal-point](https://github.com/snaptoken/kilo-src/tree/decimal-point))

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
 /*** 構文ハイライト ***/
 
 int is_separator(int c) {
   …
 }
 
 void editorUpdateSyntax(erow *row) {
   row->hl = realloc(row->hl, row->rsize);
   memset(row->hl, HL_NORMAL, row->rsize);
 
   int prev_sep = 1;
 
   int i = 0;
   while (i < row->rsize) {
     char c = row->render[i];
     unsigned char prev_hl = (i > 0) ? row->hl[i - 1] : HL_NORMAL;
 
+    if ((isdigit(c) && (prev_sep || prev_hl == HL_NUMBER)) ||
+        (c == '.' && prev_hl == HL_NUMBER)) {
       row->hl[i] = HL_NUMBER;
       i++;
       prev_sep = 0;
       continue;
     }
 
     prev_sep = is_separator(c);
     i++;
   }
 }
 
 int editorSyntaxToColor(int hl) {
   …
   }
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
 …
 /*** 初期化 ***/
```

*コンパイル*

先ほど数字としてハイライトした文字の後に続く「.」文字は、その数字の一部とみなされます。

## ファイルタイプの検出

他の点について説明する前に、エディタにファイルタイプの検出機能を追加します。これにより、ファイルの種類ごとに異なるハイライト表示ルールを設定できるようになります。例えば、テキストファイルはハイライト表示せず、Cファイルでは数値、文字列、C/C++スタイルのコメント、そしてC言語特有の様々なキーワードをハイライト表示するようにします。

特定のファイルタイプの構文強調表示に関するすべての情報を含む `editorSyntax` 構造体を作成しましょう。

**ステップ 157** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/editor-syntax/kilo.c) / [editor-syntax](https://github.com/snaptoken/kilo-src/tree/editor-syntax))

```diff
 /*** 含まれるもの ***/
 …
 /*** 定義 ***/
 
 #define KILO_VERSION "0.0.1"
 #define KILO_TAB_STOP 8
 #define KILO_QUIT_TIMES 3
 
 #define CTRL_KEY(k) ((k) & 0x1f)
 
 enum editorKey {
   …
 };
 
 enum editorHighlight {
   …
 };
 
+#define HL_HIGHLIGHT_NUMBERS (1<<0)
+
 /*** データ ***/
 
+struct editorSyntax {
+  char *filetype;
+  char **filematch;
+  int flags;
+};
+
 typedef struct erow {
   …
 } erow;
 
 struct editorConfig {
   …
 };
 
 struct editorConfig E;
 
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 構文ハイライト ***/
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
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

`filetype` フィールドは、ステータスバーにユーザーに表示されるファイルタイプの名前です。`filematch` は文字列の配列で、各文字列にはファイル名と照合するパターンが含まれています。ファイル名が一致すると、そのファイルは指定されたファイルタイプとして認識されます。最後に、`flags` はビットフィールドで、そのファイルタイプの数値を強調表示するかどうか、および文字列を強調表示するかどうかを示すフラグが含まれます。ここでは、`HL_HIGHLIGHT_NUMBERS` フラグビットのみを定義します。

それでは、組み込みの`editorSyntax`構造体の配列を作成し、そこにC言語用の構造体を追加しましょう。

**ステップ 158** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/hldb/kilo.c) / [hldb](https://github.com/snaptoken/kilo-src/tree/hldb))

```diff
 /*** インクルード ***/
 …
 /*** 定義 ***/
 …
 /*** データ ***/
 
 struct editorSyntax {
   …
 };
 
 typedef struct erow {
   …
 } erow;
 
 struct editorConfig {
   …
 };
 
 struct editorConfig E;
 
+/*** ファイルタイプ ***/
+
+char *C_HL_extensions[] = { ".c", ".h", ".cpp", NULL };
+
+struct editorSyntax HLDB[] = {
+  {
+    "c",
+    C_HL_extensions,
+    HL_HIGHLIGHT_NUMBERS
+  },
+};
+
+#define HLDB_ENTRIES (sizeof(HLDB) / sizeof(HLDB[0]))
+
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 構文ハイライト ***/
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
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

`HLDB` は「ハイライトデータベース」の略です。C 言語用の `editorSyntax` 構造体には、`filetype` フィールドに文字列 `"c"`、`filematch` フィールドに拡張子 `".c"`、`".h"`、`".cpp"` (配列は `NULL` で終了する必要があります)、`flags` フィールドに `HL_HIGHLIGHT_NUMBERS` フラグがオンになっています。

次に、`HLDB`配列の長さを格納するための定数`HLDB_ENTRIES`を定義します。

それでは、現在の `editorSyntax` 構造体へのポインタをグローバルエディタ状態に追加し、それを `NULL` で初期化しましょう。

**ステップ 159** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/e-syntax/kilo.c) / [e-syntax](https://github.com/snaptoken/kilo-src/tree/e-syntax))

```diff
 /*** 含まれる ***/
 …
 /*** 定義される ***/
 …
 /*** データ ***/
 
 struct editorSyntax {
   …
 };
 
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
   int dirty;
   char *filename;
   char statusmsg[80];
   time_t statusmsg_time;
+  struct editorSyntax *syntax;
   struct termios orig_termios;
 };
 
 struct editorConfig E;
 
 /*** ファイルタイプ ***/
 …
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 構文ハイライト ***/
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
   E.dirty = 0;
   E.filename = NULL;
   E.statusmsg[0] = '\0';
   E.statusmsg_time = 0;
+  E.syntax = NULL;
 
   if (getWindowSize(&E.screenrows, &E.screencols) == -1) die("getWindowSize");
   E.screenrows -= 2;
 }
 
 int main(int argc, char *argv[]) {
   …
   }
```

*コンパイルはされるが、目に見える効果はない*

`E.syntax`が`NULL`の場合、現在のファイルに対応するファイルタイプが存在しないため、構文ハイライトは行われません。

ステータスバーに現在のファイルタイプを表示しましょう。`E.syntax`が`NULL`の場合は、代わりに`no ft`（「ファイルタイプなし」）を表示します。

**ステップ 160** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/show-filetype/kilo.c) / [show-filetype](https://github.com/snaptoken/kilo-src/tree/show-filetype))

```diff
 /*** 含める ***/
 …
 /*** 定義する ***/
 …
 /*** データ ***/
 …
 /*** ファイルタイプ ***/
 …
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 構文ハイライト ***/
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
 
 void editorScroll() {
   …
 }
 
 void editorDrawRows(struct abuf *ab) {
   …
 }
 
 void editorDrawStatusBar(struct abuf *ab) {
   abAppend(ab, "\x1b[7m", 4);
   char status[80], rstatus[80];
   int len = snprintf(status, sizeof(status), "%.20s - %d lines %s",
     E.filename ? E.filename : "[No Name]", E.numrows,
     E.dirty ? "(modified)" : "");
+  int rlen = snprintf(rstatus, sizeof(rstatus), "%s | %d/%d",
+    E.syntax ? E.syntax->filetype : "no ft", E.cy + 1, E.numrows);
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

*コンパイル*

それでは、`editorUpdateSyntax()` を変更して、現在の `E.syntax` の値を考慮に入れるようにしましょう。

**ステップ 161** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/use-filetype/kilo.c) / [use-filetype](https://github.com/snaptoken/kilo-src/tree/use-filetype))

```diff
 /*** 含まれるもの ***/
 …
 /*** 定義するもの ***/
 …
 /*** データ ***/
 …
 /*** ファイルタイプ ***/
 …
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 構文ハイライト ***/
 
 int is_separator(int c) {
   …
 }
 
 void editorUpdateSyntax(erow *row) {
   row->hl = realloc(row->hl, row->rsize);
   memset(row->hl, HL_NORMAL, row->rsize);
 
+  if (E.syntax == NULL) return;
+
   int prev_sep = 1;
 
   int i = 0;
   while (i < row->rsize) {
     char c = row->render[i];
     unsigned char prev_hl = (i > 0) ? row->hl[i - 1] : HL_NORMAL;
 
+    if (E.syntax->flags & HL_HIGHLIGHT_NUMBERS) {
+      if ((isdigit(c) && (prev_sep || prev_hl == HL_NUMBER)) ||
+          (c == '.' && prev_hl == HL_NUMBER)) {
+        row->hl[i] = HL_NUMBER;
+        i++;
+        prev_sep = 0;
+        continue;
+      }
     }
 
     prev_sep = is_separator(c);
     i++;
   }
 }
 
 int editorSyntaxToColor(int hl) {
   …
   }
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
 …
 /*** 初期化 ***/
```

*コンパイル*

ファイルタイプが設定されていない場合は、行全体を `HL_NORMAL` に `memset()` した後、すぐに `return` します。また、数値強調表示のコードは、現在のファイルタイプに対して数値を強調表示する必要があるかどうかをチェックする `if` 文で囲みます。

次に、現在のファイル名を HLDB の `filematch` フィールドのいずれかと照合する `editorSelectSyntaxHighlight()` 関数を作成します。一致するものがあれば、`E.syntax` にそのファイルタイプを設定します。

**ステップ 162** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/select-syntax/kilo.c) / [select-syntax](https://github.com/snaptoken/kilo-src/tree/select-syntax))

```diff
 /*** 含まれるもの ***/
 …
 /*** 定義するもの ***/
 …
 /*** データ ***/
 …
 /*** ファイルタイプ ***/
 …
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 構文ハイライト ***/
 
 int is_separator(int c) {
   …
 }
 
 void editorUpdateSyntax(erow *row) {
   …
 }
 
 int editorSyntaxToColor(int hl) {
   …
 }
 
+void editorSelectSyntaxHighlight() {
+  E.syntax = NULL;
+  if (E.filename == NULL) return;
+
+  char *ext = strrchr(E.filename, '.');
+
+  for (unsigned int j = 0; j < HLDB_ENTRIES; j++) {
+    struct editorSyntax *s = &HLDB[j];
+    unsigned int i = 0;
+    while (s->filematch[i]) {
+      int is_ext = (s->filematch[i][0] == '.');
+      if ((is_ext && ext && !strcmp(ext, s->filematch[i])) ||
+          (!is_ext && strstr(E.filename, s->filematch[i]))) {
+        E.syntax = s;
+        return;
+      }
+      i++;
+    }
+  }
+}
+
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
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

`strrchr()` と `strcmp()` は `<string.h>` から取得されます。`strrchr()` は文字列内の文字の最後の出現位置へのポインタを返し、`strcmp()` は指定された 2 つの文字列が等しい場合に `0` を返します。

まず、`E.syntax` を `NULL` に設定します。これにより、一致するものが何もない場合、またはファイル名がない場合は、ファイルタイプが存在しないことになります。

次に、`strrchr()` を使用してファイル名の末尾にある `.` 文字を探し、拡張子部分へのポインタを取得します。拡張子がない場合は、`ext` は `NULL` になります。

最後に、`HLDB`配列内の各`editorSyntax`構造体をループ処理し、それぞれの構造体について、`filematch`配列内の各パターンをループ処理します。パターンが`.`で始まる場合は、ファイル拡張子パターンなので、`strcmp()`を使用してファイル名がその拡張子で終わるかどうかを確認します。ファイル拡張子パターンでない場合は、`strstr()`を使用してファイル名のどこかにパターンが存在するかどうかを確認します。ファイル名がこれらのルールに従って一致した場合は、`E.syntax`に現在の`editorSyntax`構造体を設定し、`return`します。

`E.filename`が変更される箇所すべてで`editorSelectSyntaxHighlight()`を呼び出したい。これは`editorOpen()`と`editorSave()`で行われる。

**ステップ 163** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/detect-filetype/kilo.c) / [detect-filetype](https://github.com/snaptoken/kilo-src/tree/detect-filetype))

```diff
 /*** 含める ***/
 …
 /*** 定義する ***/
 …
 /*** データ ***/
 …
 /*** ファイルタイプ ***/
 …
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 構文強調表示 ***/
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
   free(E.filename);
   E.filename = strdup(filename);
 
+  editorSelectSyntaxHighlight();
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
     editorInsertRow(E.numrows, line, linelen);
   }
   free(line);
   fclose(fp);
   E.dirty = 0;
 }
 
 void editorSave() {
   if (E.filename == NULL) {
     E.filename = editorPrompt("Save as: %s (ESC to cancel)", NULL);
     if (E.filename == NULL) {
       editorSetStatusMessage("Save aborted");
       return;
     }
+    editorSelectSyntaxHighlight();
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
 …
 /*** append buffer ***/
 …
 /*** output ***/
 …
 /*** input ***/
 …
 /*** init ***/
```

*コンパイル*

この時点で、エディタで C ファイルを開くと、数字がハイライト表示され、ファイルの種類を表示するステータス バーに「c」と表示されるはずです。引数なしでエディタを起動し、ファイル名が「.c」で終わるファイルを保存すると、ステータス バーのファイルの種類が「no ft」から「c」に正しく変化するはずです。しかし、ファイル内の数字はハイライト表示されません。これは非常に残念です。

`editorSelectSyntaxHighlight()`で`E.syntax`を設定した後、ファイル全体を再度ハイライト表示してみましょう。

**ステップ 164** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/rehighlight/kilo.c) / [rehighlight](https://github.com/snaptoken/kilo-src/tree/rehighlight))

```diff
 /*** 含まれるもの ***/
 …
 /*** 定義するもの ***/
 …
 /*** データ ***/
 …
 /*** ファイルタイプ ***/
 …
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 構文ハイライト ***/
 
 int is_separator(int c) {
   …
 }
 
 void editorUpdateSyntax(erow *row) {
   …
 }
 
 int editorSyntaxToColor(int hl) {
   …
 }
 
 void editorSelectSyntaxHighlight() {
   E.syntax = NULL;
   if (E.filename == NULL) return;
 
   char *ext = strrchr(E.filename, '.');
 
   for (unsigned int j = 0; j < HLDB_ENTRIES; j++) {
     struct editorSyntax *s = &HLDB[j];
     unsigned int i = 0;
     while (s->filematch[i]) {
       int is_ext = (s->filematch[i][0] == '.');
       if ((is_ext && ext && !strcmp(ext, s->filematch[i])) ||
           (!is_ext && strstr(E.filename, s->filematch[i]))) {
         E.syntax = s;
+
+        int filerow;
+        for (filerow = 0; filerow < E.numrows; filerow++) {
+          editorUpdateSyntax(&E.row[filerow]);
+        }
+
         return;
       }
       i++;
     }
   }
 }
 
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
 …
 /*** 初期化 ***/
```

*コンパイル*

ファイル内の各行をループ処理し、`editorUpdateSyntax()` を呼び出すだけです。これで、ファイルの種類が変わるとすぐにハイライト表示が変わります。

## カラフルな文字列

さて、前置きはこれくらいにして、いよいよ他の要素にも注目していきましょう！まずは弦から。

**ステップ 165** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/hl-string/kilo.c) / [hl-string](https://github.com/snaptoken/kilo-src/tree/hl-string))

```diff
 /*** 含まれるもの ***/
 …
 /*** 定義するもの ***/
 
 #define KILO_VERSION "0.0.1"
 #define KILO_TAB_STOP 8
 #define KILO_QUIT_TIMES 3
 
 #define CTRL_KEY(k) ((k) & 0x1f)
 
 enum editorKey {
   …
 };
 
 enum editorHighlight {
   HL_NORMAL = 0,
+  HL_STRING,
   HL_NUMBER,
   HL_MATCH
 };
 
 #define HL_HIGHLIGHT_NUMBERS (1<<0)
 
 /*** データ ***/
 …
 /*** ファイルタイプ ***/
 …
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 構文ハイライト ***/
 
 int is_separator(int c) {
   …
 }
 
 void editorUpdateSyntax(erow *row) {
   …
 }
 
 int editorSyntaxToColor(int hl) {
   switch (hl) {
+    case HL_STRING: return 35;
     case HL_NUMBER: return 31;
     case HL_MATCH: return 34;
     default: return 37;
   }
 }
 
 void editorSelectSyntaxHighlight() {
   …
         }
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
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

文字列をマゼンタ色（`35`）に着色します。

それでは、`editorSyntax`構造体の`flags`フィールドに`HL_HIGHLIGHT_STRINGS`ビットフラグを追加し、Cファイルのハイライト表示時にこのフラグをオンにしましょう。

**ステップ 166** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/string-flag/kilo.c) / [string-flag](https://github.com/snaptoken/kilo-src/tree/string-flag))

```diff
 /*** 含まれるもの ***/
 …
 /*** 定義 ***/
 
 #define KILO_VERSION "0.0.1"
 #define KILO_TAB_STOP 8
 #define KILO_QUIT_TIMES 3
 
 #define CTRL_KEY(k) ((k) & 0x1f)
 
 enum editorKey {
   …
 };
 
 enum editorHighlight {
   …
 };
 
 #define HL_HIGHLIGHT_NUMBERS (1<<0)
+#define HL_HIGHLIGHT_STRINGS (1<<1)
 
 /*** データ ***/
 …
 /*** ファイルタイプ ***/
 
 char *C_HL_extensions[] = { ".c", ".h", ".cpp", NULL };
 
 struct editorSyntax HLDB[] = {
   {
     "c",
     C_HL_extensions,
+    HL_HIGHLIGHT_NUMBERS | HL_HIGHLIGHT_STRINGS
   },
 };
 
 #define HLDB_ENTRIES (sizeof(HLDB) / sizeof(HLDB[0]))
 
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 構文ハイライト ***/
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
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

それでは、実際のハイライトコードについて説明します。`in_string` 変数を使用して、現在文字列の中にいるかどうかを追跡します。文字列の中にいる場合は、閉じ引用符に到達するまで、現在の文字を文字列としてハイライトし続けます。

**ステップ 167** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/syntax-strings/kilo.c) / [syntax-strings](https://github.com/snaptoken/kilo-src/tree/syntax-strings))

```diff
 /*** 含まれるもの ***/
 …
 /*** 定義するもの ***/
 …
 /*** データ ***/
 …
 /*** ファイルタイプ ***/
 …
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 構文ハイライト ***/
 
 int is_separator(int c) {
   …
 }
 
 void editorUpdateSyntax(erow *row) {
   row->hl = realloc(row->hl, row->rsize);
   memset(row->hl, HL_NORMAL, row->rsize);
 
   if (E.syntax == NULL) return;
 
   int prev_sep = 1;
+  int in_string = 0;
 
   int i = 0;
   while (i < row->rsize) {
     char c = row->render[i];
     unsigned char prev_hl = (i > 0) ? row->hl[i - 1] : HL_NORMAL;
 
+    if (E.syntax->flags & HL_HIGHLIGHT_STRINGS) {
+      if (in_string) {
+        row->hl[i] = HL_STRING;
+        if (c == in_string) in_string = 0;
+        i++;
+        prev_sep = 1;
+        continue;
+      } else {
+        if (c == '"' || c == '\'') {
+          in_string = c;
+          row->hl[i] = HL_STRING;
+          i++;
+          continue;
+        }
+      }
+    }
+
     if (E.syntax->flags & HL_HIGHLIGHT_NUMBERS) {
       if ((isdigit(c) && (prev_sep || prev_hl == HL_NUMBER)) ||
           (c == '.' && prev_hl == HL_NUMBER)) {
         row->hl[i] = HL_NUMBER;
         i++;
         prev_sep = 0;
         continue;
       }
     }
 
     prev_sep = is_separator(c);
     i++;
   }
 }
 
 int editorSyntaxToColor(int hl) {
   …
 }
 
 void editorSelectSyntaxHighlight() {
   …
         }
 …
 /*** 行操作 ***/
 …
 /*** エディタ操作 ***/
 …
 /*** ファイル入出力 ***/
 …
 /*** 検索***/
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

ご覧のとおり、二重引用符で囲まれた文字列と単一引用符で囲まれた文字列の両方を強調表示しています（Lisper/Rustユーザーの皆さん、ごめんなさい）。実際には、文字列を閉じる文字が二重引用符（`"`）か単一引用符（`'`）のどちらかであることを判別するために、`in_string` の値としてそのどちらかを格納しています。

では、コードを上から順に見ていきましょう。`in_string`が設定されている場合、現在の文字は`HL_STRING`で強調表示できることがわかります。次に、現在の文字が閉じ引用符（`c == in_string`）であるかどうかを確認し、そうであれば`in_string`を`0`にリセットします。そして、現在の文字を強調表示したので、`i`をインクリメントして現在のループの反復処理を`continue`して終了することで、その文字を消費する必要があります。また、文字列の強調表示が完了したら、閉じ引用符が区切り文字として扱われるように、`prev_sep`を`1`に設定します。

現在文字列の中にいない場合は、二重引用符または単一引用符の有無をチェックして、文字列の先頭にいるかどうかを確認する必要があります。先頭にいる場合は、引用符を`in_string`に格納し、`HL_STRING`で強調表示して、それを処理します。

文字列をハイライト表示する際には、エスケープされた引用符を考慮に入れるべきでしょう。文字列内に「\'」または「\"」というシーケンスが含まれている場合、ほとんどの言語ではエスケープされた引用符は文字列を閉じません。

**ステップ 168** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/string-escapes/kilo.c) / [string-escapes](https://github.com/snaptoken/kilo-src/tree/string-escapes))

```diff
 /*** 含まれるもの ***/
 …
 /*** 定義するもの ***/
 …
 /*** データ ***/
 …
 /*** ファイルタイプ ***/
 …
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 構文ハイライト ***/
 
 int is_separator(int c) {
   …
 }
 
 void editorUpdateSyntax(erow *row) {
   row->hl = realloc(row->hl, row->rsize);
   memset(row->hl, HL_NORMAL, row->rsize);
 
   if (E.syntax == NULL) return;
 
   int prev_sep = 1;
   int in_string = 0;
 
   int i = 0;
   while (i < row->rsize) {
     char c = row->render[i];
     unsigned char prev_hl = (i > 0) ? row->hl[i - 1] : HL_NORMAL;
 
     if (E.syntax->flags & HL_HIGHLIGHT_STRINGS) {
       if (in_string) {
         row->hl[i] = HL_STRING;
+        if (c == '\\' && i + 1 < row->rsize) {
+          row->hl[i + 1] = HL_STRING;
+          i += 2;
+          continue;
+        }
         if (c == in_string) in_string = 0;
         i++;
         prev_sep = 1;
         continue;
       } else {
         if (c == '"' || c == '\'') {
           in_string = c;
           row->hl[i] = HL_STRING;
           i++;
           continue;
         }
       }
     }
 
     if (E.syntax->flags & HL_HIGHLIGHT_NUMBERS) {
       if ((isdigit(c) && (prev_sep || prev_hl == HL_NUMBER)) ||
           (c == '.' && prev_hl == HL_NUMBER)) {
         row->hl[i] = HL_NUMBER;
         i++;
         prev_sep = 0;
         continue;
       }
     }
 
     prev_sep = is_separator(c);
     i++;
   }
 }
 
 int editorSyntaxToColor(int hl) {
   …
 }
 
 void editorSelectSyntaxHighlight() {
   …
         }
 …
 /*** 行操作 ***/
 …
 /*** エディタ操作 ***/
 …
 /*** ファイル入出力 ***/
 …
 /*** 検索***/
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

文字列の中にいて、現在の文字がバックスラッシュ（`\`）であり、かつ、その行にバックスラッシュの後に少なくとももう1文字ある場合、バックスラッシュの後に続く文字を`HL_STRING`で強調表示して消費します。両方の文字を一度に消費するために、`i`を`2`増やします。

## カラフルな一行コメント

次に、1行コメントについて説明しましょう。（複数行コメントは複雑なので、最後に説明します。）

**ステップ 169** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/hl-comment/kilo.c) / [hl-comment](https://github.com/snaptoken/kilo-src/tree/hl-comment))

```diff
 /*** 含まれるもの ***/
 …
 /*** 定義 ***/
 
 #define KILO_VERSION "0.0.1"
 #define KILO_TAB_STOP 8
 #define KILO_QUIT_TIMES 3
 
 #define CTRL_KEY(k) ((k) & 0x1f)
 
 enum editorKey {
   …
 };
 
 enum editorHighlight {
   HL_NORMAL = 0,
+  HL_COMMENT,
   HL_STRING,
   HL_NUMBER,
   HL_MATCH
 };
 
 #define HL_HIGHLIGHT_NUMBERS (1<<0)
 #define HL_HIGHLIGHT_STRINGS (1<<1)
 
 /***データ ***/
 …
 /*** ファイルタイプ ***/
 …
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 構文ハイライト ***/
 
 int is_separator(int c) {
   …
 }
 
 void editorUpdateSyntax(erow *row) {
   …
 }
 
 int editorSyntaxToColor(int hl) {
   switch (hl) {
+    case HL_COMMENT: return 36;
     case HL_STRING: return 35;
     case HL_NUMBER: return 31;
     case HL_MATCH: return 34;
     default: return 37;
   }
 }
 
 void editorSelectSyntaxHighlight() {
   …
         }
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
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

コメントはシアン色（`36`）で強調表示されます。

言語によって単一行コメントのパターンが大きく異なるため、各言語で独自のパターンを指定できるようにします。`editorSyntax`構造体に`singleline_comment_start`文字列を追加し、Cファイルタイプの場合は`"//"`に設定します。

**ステップ 170** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/scs/kilo.c) / [scs](https://github.com/snaptoken/kilo-src/tree/scs))

```diff
 /*** 含まれるもの ***/
 …
 /*** 定義するもの ***/
 …
 /*** データ ***/
 
 struct editorSyntax {
   char *filetype;
   char **filematch;
+  char *singleline_comment_start;
   int flags;
 };
 
 typedef struct erow {
   …
 } erow;
 
 struct editorConfig {
   …
 };
 
 struct editorConfig E;
 
 /*** ファイルタイプ ***/
 
 char *C_HL_extensions[] = { ".c", ".h", ".cpp", NULL };
 
 struct editorSyntax HLDB[] = {
   {
     "c",
     C_HL_extensions,
+    "//",
     HL_HIGHLIGHT_NUMBERS | HL_HIGHLIGHT_STRINGS
   },
 };
 
 #define HLDB_ENTRIES (sizeof(HLDB) / sizeof(HLDB[0]))
 
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 構文ハイライト ***/
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
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

さて、次はハイライト表示のコードです。

**ステップ 171** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/syntax-comments/kilo.c) / [syntax-comments](https://github.com/snaptoken/kilo-src/tree/syntax-comments))

```diff
 /*** 含まれるもの ***/
 …
 /*** 定義するもの ***/
 …
 /*** データ ***/
 …
 /*** ファイルタイプ ***/
 …
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 構文ハイライト ***/
 
 int is_separator(int c) {
   …
 }
 
 void editorUpdateSyntax(erow *row) {
   row->hl = realloc(row->hl, row->rsize);
   memset(row->hl, HL_NORMAL, row->rsize);
 
   if (E.syntax == NULL) return;
 
+  char *scs = E.syntax->singleline_comment_start;
+  int scs_len = scs ? strlen(scs) : 0;
+
   int prev_sep = 1;
   int in_string = 0;
 
   int i = 0;
   while (i < row->rsize) {
     char c = row->render[i];
     unsigned char prev_hl = (i > 0) ? row->hl[i - 1] : HL_NORMAL;
 
+    if (scs_len && !in_string) {
+      if (!strncmp(&row->render[i], scs, scs_len)) {
+        memset(&row->hl[i], HL_COMMENT, row->rsize - i);
+        break;
+      }
+    }
+
     if (E.syntax->flags & HL_HIGHLIGHT_STRINGS) {
       if (in_string) {
         row->hl[i] = HL_STRING;
         if (c == '\\' && i + 1 < row->rsize) {
           row->hl[i + 1] = HL_STRING;
           i += 2;
           continue;
         }
         if (c == in_string) in_string = 0;
         i++;
         prev_sep = 1;
         continue;
       } else {
         if (c == '"' || c == '\'') {
           in_string = c;
           row->hl[i] = HL_STRING;
           i++;
           continue;
         }
       }
     }
 
     if (E.syntax->flags & HL_HIGHLIGHT_NUMBERS) {
       if ((isdigit(c) && (prev_sep || prev_hl == HL_NUMBER)) ||
           (c == '.' && prev_hl == HL_NUMBER)) {
         row->hl[i] = HL_NUMBER;
         i++;
         prev_sep = 0;
         continue;
       }
     }
 
     prev_sep = is_separator(c);
     i++;
   }
 }
 
 int editorSyntaxToColor(int hl) {
   …
 }
 
 void editorSelectSyntaxHighlight() {
   …
         }
 …
 /*** 行操作 ***/
 …
 /*** エディタ操作 ***/
 …
 /*** ファイル入出力 ***/
 …
 /*** 検索***/
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

`strncmp()` は `<string.h>` から来ています。

特定のファイルタイプで単一行コメントのハイライト表示を不要にする場合は、`singleline_comment_start` を `NULL` または空文字列 (`""`) に設定できます。入力のしやすさ (および可読性の向上) のため、`scs` を `E.syntax->singleline_comment_start` のエイリアスとしています。次に、`scs_len` を文字列の長さに設定します。文字列が `NULL` の場合は `0` に設定します。これにより、`scs_len` をブール値として使用して、単一行コメントをハイライト表示するかどうかを判断できます。

そこで、コメント強調表示コードを `if` 文で囲み、`scs_len` をチェックするとともに、文字列強調表示コードの上にこのコードを配置するため、文字列内にないことも確認します（この関数では順序が非常に重要です）。

これらのチェックに合格した場合、`strncmp()` を使用して、この文字が単一行コメントの開始文字であるかどうかを確認します。開始文字であれば、行の残りの部分全体を `HL_COMMENT` で `memset()` し、構文強調表示ループを `break` で抜け出します。これで、行の強調表示は完了です。

## カラフルなキーワード

次に、キーワードの強調表示について見ていきましょう。ここでは、言語によって2種類のキーワードを指定し、それぞれ異なる色で強調表示できるようにします。（C言語では、実際のキーワードを1つの色で強調表示し、一般的な型名を別の色で強調表示します。）

**ステップ 172** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/hl-keywords/kilo.c) / [hl-keywords](https://github.com/snaptoken/kilo-src/tree/hl-keywords))

```diff
 /*** インクルード ***/
 …
 /*** 定義 ***/
 
 #define KILO_VERSION "0.0.1"
 #define KILO_TAB_STOP 8
 #define KILO_QUIT_TIMES 3
 
 #define CTRL_KEY(k) ((k) & 0x1f)
 
 enum editorKey {
   …
 };
 
 enum editorHighlight {
   HL_NORMAL = 0,
   HL_COMMENT,
+  HL_KEYWORD1,
+  HL_KEYWORD2,
   HL_STRING,
   HL_NUMBER,
   HL_MATCH
 };
 
 #define HL_HIGHLIGHT_NUMBERS (1<<0)
 #define HL_HIGHLIGHT_STRINGS (1<<1)
 
 /*** データ ***/
 …
 /*** ファイルタイプ ***/
 …
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 構文ハイライト ***/
 
 int is_separator(int c) {
   …
 }
 
 void editorUpdateSyntax(erow *row) {
   …
 }
 
 int editorSyntaxToColor(int hl) {
   switch (hl) {
     case HL_COMMENT: return 36;
+    case HL_KEYWORD1: return 33;
+    case HL_KEYWORD2: return 32;
     case HL_STRING: return 35;
     case HL_NUMBER: return 31;
     case HL_MATCH: return 34;
     default: return 37;
   }
 }
 
 void editorSelectSyntaxHighlight() {
   …
         }
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
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

キーワードに使用する2つの色は、黄色（`33`）と緑（`32`）です。

`editorSyntax`構造体に`keywords`配列を追加しましょう。これはNULLで終端される文字列配列で、各文字列にはキーワードが含まれます。2種類のキーワードを区別するために、2番目の種類のキーワードはパイプ（`|`）文字（縦棒とも呼ばれます）で終端します。

**ステップ 173** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/c-keywords/kilo.c) / [c-keywords](https://github.com/snaptoken/kilo-src/tree/c-keywords))

```diff
 /*** 含まれる ***/
 …
 /*** 定義される ***/
 …
 /*** データ ***/
 
 struct editorSyntax {
   char *filetype;
   char **filematch;
+  char **keywords;
   char *singleline_comment_start;
   int flags;
 };
 
 typedef struct erow {
   …
 } erow;
 
 struct editorConfig {
   …
 };
 
 struct editorConfig E;
 
 /*** filetypes ***/
 
 char *C_HL_extensions[] = { ".c", ".h", ".cpp", NULL };
+char *C_HL_keywords[] = {
+  "switch", "if", "while", "for", "break", "continue", "return", "else",
+  "struct", "union", "typedef", "static", "enum", "class", "case",
+
+  "int|", "long|", "double|", "float|", "char|", "unsigned|", "signed|",
+  "void|", NULL
+};
 
 struct editorSyntax HLDB[] = {
   {
     "c",
     C_HL_extensions,
+    C_HL_keywords,
     "//",
     HL_HIGHLIGHT_NUMBERS | HL_HIGHLIGHT_STRINGS
   },
 };
 
 #define HLDB_ENTRIES (sizeof(HLDB) / sizeof(HLDB[0]))
 
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 構文ハイライト ***/
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
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

前述のとおり、一般的なC言語の型を補助キーワードとして強調表示するため、それぞれの末尾に「|」文字を付けます。

それでは、それらを詳しく見ていきましょう。

**ステップ 174** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/syntax-keywords/kilo.c) / [syntax-keywords](https://github.com/snaptoken/kilo-src/tree/syntax-keywords))

```diff
 /*** 含まれるもの ***/
 …
 /*** 定義するもの ***/
 …
 /*** データ ***/
 …
 /*** ファイルタイプ ***/
 …
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 構文ハイライト ***/
 
 int is_separator(int c) {
   …
 }
 
 void editorUpdateSyntax(erow *row) {
   row->hl = realloc(row->hl, row->rsize);
   memset(row->hl, HL_NORMAL, row->rsize);
 
   if (E.syntax == NULL) return;
 
+  char **keywords = E.syntax->keywords;
+
   char *scs = E.syntax->singleline_comment_start;
   int scs_len = scs ? strlen(scs) : 0;
 
   int prev_sep = 1;
   int in_string = 0;
 
   int i = 0;
   while (i < row->rsize) {
     char c = row->render[i];
     unsigned char prev_hl = (i > 0) ? row->hl[i - 1] : HL_NORMAL;
 
     if (scs_len && !in_string) {
       if (!strncmp(&row->render[i], scs, scs_len)) {
         memset(&row->hl[i], HL_COMMENT, row->rsize - i);
         break;
       }
     }
 
     if (E.syntax->flags & HL_HIGHLIGHT_STRINGS) {
       if (in_string) {
         row->hl[i] = HL_STRING;
         if (c == '\\' && i + 1 < row->rsize) {
           row->hl[i + 1] = HL_STRING;
           i += 2;
           continue;
         }
         if (c == in_string) in_string = 0;
         i++;
         prev_sep = 1;
         continue;
       } else {
         if (c == '"' || c == '\'') {
           in_string = c;
           row->hl[i] = HL_STRING;
           i++;
           continue;
         }
       }
     }
 
     if (E.syntax->flags & HL_HIGHLIGHT_NUMBERS) {
       if ((isdigit(c) && (prev_sep || prev_hl == HL_NUMBER)) ||
           (c == '.' && prev_hl == HL_NUMBER)) {
         row->hl[i] = HL_NUMBER;
         i++;
         prev_sep = 0;
         continue;
       }
     }
 
+    if (prev_sep) {
+      int j;
+      for (j = 0; keywords[j]; j++) {
+        int klen = strlen(keywords[j]);
+        int kw2 = keywords[j][klen - 1] == '|';
+        if (kw2) klen--;
+
+        if (!strncmp(&row->render[i], keywords[j], klen) &&
+            is_separator(row->render[i + klen])) {
+          memset(&row->hl[i], kw2 ? HL_KEYWORD2 : HL_KEYWORD1, klen);
+          i += klen;
+          break;
+        }
+      }
+      if (keywords[j] != NULL) {
+        prev_sep = 0;
+        continue;
+      }
+    }
+
     prev_sep = is_separator(c);
     i++;
   }
 }
 
 int editorSyntaxToColor(int hl) {
   …
 }
 
 void editorSelectSyntaxHighlight() {
   …
         }
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
 …
 /*** 初期化 ***/
```

*コンパイル*

まず、関数の先頭で、`keywords` を `E.syntax->keywords` のエイリアスにします。これは、かなり複雑なコードの中で頻繁に使用するためです。

キーワードには、キーワードの前後両方に区切り文字が必要です。そうしないと、「avoid」、「voided」、「avoidable」の「void」がキーワードとして強調表示されてしまい、これは絶対に回避したい問題です。

そこで、各キーワードをループ処理する前に、`prev_sep` をチェックして、区切り文字がキーワードの前に来ていることを確認します。各キーワードについて、長さを `klen` に、それがセカンダリキーワードであるかどうかを `kw2` に格納します。セカンダリキーワードの場合は、余分な `|` 文字を考慮して `klen` をデクリメントします。

次に、`strncmp()` を使用して、テキスト内の現在の位置にキーワードが存在するかどうかを確認し、さらにキーワードの後に区切り文字があるかどうかを確認します。`\0` は区切り文字とみなされるため、キーワードが行末にある場合でもこの方法は有効です。

これらすべてが通過した場合、強調表示するキーワードが見つかります。`memset()` を使用してキーワード全体を一度に強調表示し、`kw2` の値に応じて `HL_KEYWORD1` または `HL_KEYWORD2` で強調表示します。次に、キーワードの長さだけ `i` をインクリメントして、キーワード全体を消費します。その後、内部ループに入っているため、外部ループを `continue` する前に内部ループを抜ける必要があるため、`continue` ではなく `break` します。そのため、`for` ループの後、終了値 `NULL` に到達したかどうかを確認してループが抜けたかどうかをチェックし、抜けた場合は `continue` します。

[印刷不可能な文字](https://viewsourcecode.org/snaptoken/kilo/07.syntaxHighlighting.html#nonprintable-characters)
------------------------------------------------------------------------------------------------------- ----------------

複数行コメントのハイライト表示に取り組む前に、`editorUpdateSyntax()` から少し離れてみましょう。

印刷不可能な文字をもっとユーザーフレンドリーな方法で表示するようにします。現状では、印刷不可能な文字はエディタのレンダリングを完全に台無しにしてしまいます。`kilo` を実行して、引数として `kilo` 自体を渡してみてください。つまり、`kilo` を使用して `kilo` 実行可能ファイルを開きます。カーソルを移動したり、文字を入力したりしてみてください。見栄えが良くありません。キーを押すたびにターミナルが鳴ります。これは、可聴ベル文字 (`7`) が出力されるためです。コード内のターミナルエスケープシーケンスを含む文字列は、実際のエスケープシーケンスとして出力されます。これは、生の実行可能ファイルではそのように格納されているためです。

こうした事態を防ぐため、印刷不可能な文字を印刷可能な文字に変換します。アルファベットの制御文字（Ctrl-A = `1`、Ctrl-B = `2`、…、Ctrl-Z = `26`）は、大文字の `A` から `Z` までで表示します。また、バイト `0` も制御文字として表示します。Ctrl-@ = `0` なので、`@` 記号で表示します。最後に、その他の印刷不可能な文字はすべて疑問符（`?`）で表示します。これらの文字を印刷可能な文字と区別するために、反転色（白地に黒）で表示します。

**ステップ 175** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/nonprintables/kilo.c) / [nonprintables](https://github.com/snaptoken/kilo-src/tree/nonprintables))

```diff
 /*** 含める ***/
 …
 /*** 定義する ***/
 …
 /*** データ ***/
 …
 /*** ファイルタイプ ***/
 …
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 構文ハイライト ***/
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
       int len = E.row[filerow].rsize - E.coloff;
       if (len < 0) len = 0;
       if (len > E.screencols) len = E.screencols;
       char *c = &E.row[filerow].render[E.coloff];
       unsigned char *hl = &E.row[filerow].hl[E.coloff];
       int current_color = -1;
       int j;
       for (j = 0; j < len; j++) {
+        if (iscntrl(c[j])) {
+          char sym = (c[j] <= 26) ? '@' + c[j] : '?';
+          abAppend(ab, "\x1b[7m", 4);
+          abAppend(ab, &sym, 1);
+          abAppend(ab, "\x1b[m", 3);
+        } else if (hl[j] == HL_NORMAL) {
           if (current_color != -1) {
             abAppend(ab, "\x1b[39m", 5);
             current_color = -1;
           }
           abAppend(ab, &c[j], 1);
         } else {
           int color = editorSyntaxToColor(hl[j]);
           if (color != current_color) {
             current_color = color;
             char buf[16];
             int clen = snprintf(buf, sizeof(buf), "\x1b[%dm", color);
             abAppend(ab, buf, clen);
           }
           abAppend(ab, &c[j], 1);
         }
       }
       abAppend(ab, "\x1b[39m", 5);
     }
 
     abAppend(ab, "\x1b[K", 3);
     abAppend(ab, "\r\n", 2);
   }
 }
 
 void editorDrawStatusBar(struct abuf *ab) {
   …
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

*コンパイル*

`iscntrl()` 関数を使用して、現在の文字が制御文字かどうかを確認します。制御文字の場合は、その値を `'@'` に加算して印刷可能な文字に変換します（ASCII では、アルファベットの大文字は `@` 文字の後に続きます）。アルファベットの範囲外の場合は、`'?'` 文字を使用します。

次に、`<esc>[7m` エスケープシーケンスを使用して反転色に切り替えてから、変換された記号を出力します。`<esc>[m` を使用して反転色を再びオフにします。

残念ながら、`<esc>[m` を押すと、色を含むすべてのテキスト書式設定が無効になります。そこで、現在の色のエスケープシーケンスを後で表示してみましょう。

**ステップ 176** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/nonprintables-fix-color/kilo.c) / [nonprintables-fix-color](https://github.com/snaptoken/kilo-src/tree/nonprintables-fix-color))

```diff
 /*** 含める ***/
 …
 /*** 定義する ***/
 …
 /*** データ ***/
 …
 /*** ファイルタイプ ***/
 …
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 構文ハイライト ***/
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
       int len = E.row[filerow].rsize - E.coloff;
       if (len < 0) len = 0;
       if (len > E.screencols) len = E.screencols;
       char *c = &E.row[filerow].render[E.coloff];
       unsigned char *hl = &E.row[filerow].hl[E.coloff];
       int current_color = -1;
       int j;
       for (j = 0; j < len; j++) {
         if (iscntrl(c[j])) {
           char sym = (c[j] <= 26) ? '@' + c[j] : '?';
           abAppend(ab, "\x1b[7m", 4);
           abAppend(ab, &sym, 1);
           abAppend(ab, "\x1b[m", 3);
+          if (current_color != -1) {
+            char buf[16];
+            int clen = snprintf(buf, sizeof(buf), "\x1b[%dm", current_color);
+            abAppend(ab, buf, clen);
+          }
         } else if (hl[j] == HL_NORMAL) {
           if (current_color != -1) {
             abAppend(ab, "\x1b[39m", 5);
             current_color = -1;
           }
           abAppend(ab, &c[j], 1);
         } else {
           int color = editorSyntaxToColor(hl[j]);
           if (color != current_color) {
             current_color = color;
             char buf[16];
             int clen = snprintf(buf, sizeof(buf), "\x1b[%dm", color);
             abAppend(ab, buf, clen);
           }
           abAppend(ab, &c[j], 1);
         }
       }
       abAppend(ab, "\x1b[39m", 5);
     }
 
     abAppend(ab, "\x1b[K", 3);
     abAppend(ab, "\r\n", 2);
   }
 }
 
 void editorDrawStatusBar(struct abuf *ab) {
   …
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

*コンパイル*

Ctrl+A、Ctrl+Bなどを押して、これらの制御文字を文字列やコメントに挿入することで、印刷不可能な文字の色付けをテストできます。すると、それらの文字は周囲の文字と同じ色になりますが、色が反転していることがわかります。

## カラフルな複数行コメント

さて、最後に実装すべき機能が一つ残っています。それは複数行コメントのハイライト表示です。まずは、`editorHighlight`列挙型に`HL_MLCOMMENT`を追加することから始めましょう。

**ステップ 177** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/hl-multiline-comments/kilo.c) / [hl-multiline-comments](https://github.com/snaptoken/kilo-src/tree/hl-multiline-comments))

```diff
 /*** インクルード ***/
 …
 /*** 定義 ***/
 
 #define KILO_VERSION "0.0.1"
 #define KILO_TAB_STOP 8
 #define KILO_QUIT_TIMES 3
 
 #define CTRL_KEY(k) ((k) & 0x1f)
 
 enum editorKey {
   …
 };
 
 enum editorHighlight {
   HL_NORMAL = 0,
   HL_COMMENT,
+  HL_MLCOMMENT,
   HL_KEYWORD1,
   HL_KEYWORD2,
   HL_STRING,
   HL_NUMBER,
   HL_MATCH
 };
 
 #define HL_HIGHLIGHT_NUMBERS (1<<0)
 #define HL_HIGHLIGHT_STRINGS (1<<1)
 
 /*** データ ***/
 …
 /*** ファイルタイプ ***/
 …
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 構文ハイライト ***/
 
 int is_separator(int c) {
   …
 }
 
 void editorUpdateSyntax(erow *row) {
   …
 }
 
 int editorSyntaxToColor(int hl) {
   switch (hl) {
+    case HL_COMMENT:
+    case HL_MLCOMMENT: return 36;
     case HL_KEYWORD1: return 33;
     case HL_KEYWORD2: return 32;
     case HL_STRING: return 35;
     case HL_NUMBER: return 31;
     case HL_MATCH: return 34;
     default: return 37;
   }
 }
 
 void editorSelectSyntaxHighlight() {
   …
         }
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
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

複数行のコメントは、単一行のコメントと同じ色（シアン）で強調表示します。

次に、`editorSyntax` に `multiline_comment_start` と `multiline_comment_end` という 2 つの文字列を追加します。C 言語では、これらは `"/*"` と `"*/"` になります。

**ステップ 178** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/mcs-mce/kilo.c) / [mcs-mce](https://github.com/snaptoken/kilo-src/tree/mcs-mce))

```diff
 /*** 含まれる ***/
 …
 /*** 定義される ***/
 …
 /*** データ ***/
 
 struct editorSyntax {
   char *filetype;
   char **filematch;
   char **keywords;
   char *singleline_comment_start;
+  char *multiline_comment_start;
+  char *multiline_comment_end;
   int flags;
 };
 
 typedef struct erow {
   …
 } erow;
 
 struct editorConfig {
   …
 };
 
 struct editorConfig E;
 
 /*** ファイルタイプ ***/
 
 char *C_HL_extensions[] = { ".c", ".h", ".cpp", NULL };
 char *C_HL_keywords[] = {
   …
 };
 
 struct editorSyntax HLDB[] = {
   {
     "c",
     C_HL_extensions,
     C_HL_keywords,
+    "//", "/*", "*/",
     HL_HIGHLIGHT_NUMBERS | HL_HIGHLIGHT_STRINGS
   },
 };
 
 #define HLDB_ENTRIES (sizeof(HLDB) / sizeof(HLDB[0]))
 
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 構文ハイライト ***/
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
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

それでは、`editorUpdateSyntax()` をもう一度開いてみましょう。単一行コメント用に既に用意されている `scs` エイリアスと同様の `mcs` と `mce` エイリアスを追加します。さらに `mcs_len` と `mce_len` も追加します。

**ステップ 179** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/mcs-mce-len/kilo.c) / [mcs-mce-len](https://github.com/snaptoken/kilo-src/tree/mcs-mce-len))

```diff
 /*** 含まれるもの ***/
 …
 /*** 定義するもの ***/
 …
 /*** データ ***/
 …
 /*** ファイルタイプ *** /
 …
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 構文ハイライト ***/
 
 int is_separator(int c) {
   …
 }
 
 void editorUpdateSyntax(erow *row) {
   row->hl = realloc(row->hl, row->rsize);
   memset(row->hl, HL_NORMAL, row->rsize);
 
   if (E.syntax == NULL) return;
 
   char **keywords = E.syntax->keywords;
 
   char *scs = E.syntax->singleline_comment_start;
+  char *mcs = E.syntax->multiline_comment_start;
+  char *mce = E.syntax->multiline_comment_end;
+
   int scs_len = scs ? strlen(scs) : 0;
+  int mcs_len = mcs ? strlen(mcs) : 0;
+  int mce_len = mce ? strlen(mce) : 0;
 
   int prev_sep = 1;
   int in_string = 0;
 
   int i = 0;
   while (i < row->rsize) {
     char c = row->render[i];
     unsigned char prev_hl = (i > 0) ? row->hl[i - 1] : HL_NORMAL;
 
     if (scs_len && !in_string) {
       if (!strncmp(&row->render[i], scs, scs_len)) {
         memset(&row->hl[i], HL_COMMENT, row->rsize - i);
         break;
       }
     }
 
     if (E.syntax->flags & HL_HIGHLIGHT_STRINGS) {
       if (in_string) {
         row->hl[i] = HL_STRING;
         if (c == '\\' && i + 1 < row->rsize) {
           row->hl[i + 1] = HL_STRING;
           i += 2;
           continue;
         }
         if (c == in_string) in_string = 0;
         i++;
         prev_sep = 1;
         continue;
       } else {
         if (c == '"' || c == '\'') {
           in_string = c;
           row->hl[i] = HL_STRING;
           i++;
           continue;
         }
       }
     }
 
     if (E.syntax->flags & HL_HIGHLIGHT_NUMBERS) {
       if ((isdigit(c) && (prev_sep || prev_hl == HL_NUMBER)) ||
           (c == '.' && prev_hl == HL_NUMBER)) {
         row->hl[i] = HL_NUMBER;
         i++;
         prev_sep = 0;
         continue;
       }
     }
 
     if (prev_sep) {
       int j;
       for (j = 0; keywords[j]; j++) {
         int klen = strlen(keywords[j]);
         int kw2 = keywords[j][klen - 1] == '|';
         if (kw2) klen--;
 
         if (!strncmp(&row->render[i], keywords[j], klen) &&
             is_separator(row->render[i + klen])) {
           memset(&row->hl[i], kw2 ? HL_KEYWORD2 : HL_KEYWORD1, klen);
           i += klen;
           break;
         }
       }
       if (keywords[j] != NULL) {
         prev_sep = 0;
         continue;
       }
     }
 
     prev_sep = is_separator(c);
     i++;
   }
 }
 
 int editorSyntaxToColor(int hl) {
   …
 }
 
 void editorSelectSyntaxHighlight() {
   …
         }
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
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

次に、ハイライト表示のコードについて説明します。複数行については、今のところ気にしなくて構いません。

**ステップ 180** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/syntax-mlcomment/kilo.c) / [syntax-mlcomment](https://github.com/snaptoken/kilo-src/tree/syntax-mlcomment))

```diff
 /*** 含まれるもの ***/
 …
 /*** 定義するもの ***/
 …
 /*** データ ***/
 …
 /*** ファイルタイプ ***/
 …
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 構文ハイライト ***/
 
 int is_separator(int c) {
   …
 }
 
 void editorUpdateSyntax(erow *row) {
   row->hl = realloc(row->hl, row->rsize);
   memset(row->hl, HL_NORMAL, row->rsize);
 
   if (E.syntax == NULL) return;
 
   char **keywords = E.syntax->keywords;
 
   char *scs = E.syntax->singleline_comment_start;
   char *mcs = E.syntax->multiline_comment_start;
   char *mce = E.syntax->multiline_comment_end;
 
   int scs_len = scs ? strlen(scs) : 0;
   int mcs_len = mcs ? strlen(mcs) : 0;
   int mce_len = mce ? strlen(mce) : 0;
 
   int prev_sep = 1;
   int in_string = 0;
+  int in_comment = 0;
 
   int i = 0;
   while (i < row->rsize) {
     char c = row->render[i];
     unsigned char prev_hl = (i > 0) ? row->hl[i - 1] : HL_NORMAL;
 
     if (scs_len && !in_string) {
       if (!strncmp(&row->render[i], scs, scs_len)) {
         memset(&row->hl[i], HL_COMMENT, row->rsize - i);
         break;
       }
     }
 
+    if (mcs_len && mce_len && !in_string) {
+      if (in_comment) {
+        row->hl[i] = HL_MLCOMMENT;
+        if (!strncmp(&row->render[i], mce, mce_len)) {
+          memset(&row->hl[i], HL_MLCOMMENT, mce_len);
+          i += mce_len;
+          in_comment = 0;
+          prev_sep = 1;
+          continue;
+        } else {
+          i++;
+          continue;
+        }
+      } else if (!strncmp(&row->render[i], mcs, mcs_len)) {
+        memset(&row->hl[i], HL_MLCOMMENT, mcs_len);
+        i += mcs_len;
+        in_comment = 1;
+        continue;
+      }
+    }
+
     if (E.syntax->flags & HL_HIGHLIGHT_STRINGS) {
       if (in_string) {
         row->hl[i] = HL_STRING;
         if (c == '\\' && i + 1 < row->rsize) {
           row->hl[i + 1] = HL_STRING;
           i += 2;
           continue;
         }
         if (c == in_string) in_string = 0;
         i++;
         prev_sep = 1;
         continue;
       } else {
         if (c == '"' || c == '\'') {
           in_string = c;
           row->hl[i] = HL_STRING;
           i++;
           continue;
         }
       }
     }
 
     if (E.syntax->flags & HL_HIGHLIGHT_NUMBERS) {
       if ((isdigit(c) && (prev_sep || prev_hl == HL_NUMBER)) ||
           (c == '.' && prev_hl == HL_NUMBER)) {
         row->hl[i] = HL_NUMBER;
         i++;
         prev_sep = 0;
         continue;
       }
     }
 
     if (prev_sep) {
       int j;
       for (j = 0; keywords[j]; j++) {
         int klen = strlen(keywords[j]);
         int kw2 = keywords[j][klen - 1] == '|';
         if (kw2) klen--;
 
         if (!strncmp(&row->render[i], keywords[j], klen) &&
             is_separator(row->render[i + klen])) {
           memset(&row->hl[i], kw2 ? HL_KEYWORD2 : HL_KEYWORD1, klen);
           i += klen;
           break;
         }
       }
       if (keywords[j] != NULL) {
         prev_sep = 0;
         continue;
       }
     }
 
     prev_sep = is_separator(c);
     i++;
   }
 }
 
 int editorSyntaxToColor(int hl) {
   …
 }
 
 void editorSelectSyntaxHighlight() {
   …
         }
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
 …
 /*** 初期化 ***/
```

*コンパイル*

まず、現在複数行のコメントの中にいるかどうかを追跡するためのブール変数`in_comment`を追加します（この変数は単一行のコメントには使用されません）。

`while` ループに入ると、複数行コメントのハイライト表示を有効にするために、`mcs` と `mce` の両方が `NULL` ではなく、長さが `0` より大きい文字列である必要があります。また、文字列の中に `/*` がないことも確認します。ほとんどの言語では、文字列の中に `/*` があってもコメントは開始されないからです。 よし、はっきり言おう。_すべての_言語だ。

現在複数行コメントの中にいる場合は、`HL_MLCOMMENT` を使用して現在の文字を安全にハイライト表示できます。次に、`strncmp()` と `mce` を使用して、複数行コメントの末尾にいるかどうかを確認します。末尾にいる場合は、`memset()` を使用して `mce` 文字列全体を `HL_MLCOMMENT` でハイライト表示し、それを消費します。コメントの末尾にいない場合は、既にハイライト表示されている現在の文字をそのまま消費します。

現在複数行コメントの中にいない場合は、`strncmp()` と `mcs` を使用して、複数行コメントの先頭にいるかどうかを確認します。先頭にいる場合は、`memset()` を使用して `mcs` 文字列全体を `HL_MLCOMMENT` でハイライト表示し、`in_comment` を true に設定して、`mcs` 文字列全体を処理します。

さて、複数行コメントによって生じるちょっとした問題点を修正しましょう。それは、複数行コメントの中に単一行コメントがあってはならないということです。

**ステップ 181** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/slcomment-within-mlcomment/kilo.c) / [slcomment-within-mlcomment](https://github.com/snaptoken/kilo-src/tree/slcomment-within-mlcomment))

```diff
 /*** 含まれるもの ***/
 …
 /*** 定義するもの ***/
 …
 /*** データ ***/
 …
 /*** ファイルタイプ ***/
 …
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 構文ハイライト ***/
 
 int is_separator(int c) {
   …
 }
 
 void editorUpdateSyntax(erow *row) {
   row->hl = realloc(row->hl, row->rsize);
   memset(row->hl, HL_NORMAL, row->rsize);
 
   if (E.syntax == NULL) return;
 
   char **keywords = E.syntax->keywords;
 
   char *scs = E.syntax->singleline_comment_start;
   char *mcs = E.syntax->multiline_comment_start;
   char *mce = E.syntax->multiline_comment_end;
 
   int scs_len = scs ? strlen(scs) : 0;
   int mcs_len = mcs ? strlen(mcs) : 0;
   int mce_len = mce ? strlen(mce) : 0;
 
   int prev_sep = 1;
   int in_string = 0;
   int in_comment = 0;
 
   int i = 0;
   while (i < row->rsize) {
     char c = row->render[i];
     unsigned char prev_hl = (i > 0) ? row->hl[i - 1] : HL_NORMAL;
 
+    if (scs_len && !in_string && !in_comment) {
       if (!strncmp(&row->render[i], scs, scs_len)) {
         memset(&row->hl[i], HL_COMMENT, row->rsize - i);
         break;
       }
     }
 
     if (mcs_len && mce_len && !in_string) {
       if (in_comment) {
         row->hl[i] = HL_MLCOMMENT;
         if (!strncmp(&row->render[i], mce, mce_len)) {
           memset(&row->hl[i], HL_MLCOMMENT, mce_len);
           i += mce_len;
           in_comment = 0;
           prev_sep = 1;
           continue;
         } else {
           i++;
           continue;
         }
       } else if (!strncmp(&row->render[i], mcs, mcs_len)) {
         memset(&row->hl[i], HL_MLCOMMENT, mcs_len);
         i += mcs_len;
         in_comment = 1;
         continue;
       }
     }
 
     if (E.syntax->flags & HL_HIGHLIGHT_STRINGS) {
       if (in_string) {
         row->hl[i] = HL_STRING;
         if (c == '\\' && i + 1 < row->rsize) {
           row->hl[i + 1] = HL_STRING;
           i += 2;
           continue;
         }
         if (c == in_string) in_string = 0;
         i++;
         prev_sep = 1;
         continue;
       } else {
         if (c == '"' || c == '\'') {
           in_string = c;
           row->hl[i] = HL_STRING;
           i++;
           continue;
         }
       }
     }
 
     if (E.syntax->flags & HL_HIGHLIGHT_NUMBERS) {
       if ((isdigit(c) && (prev_sep || prev_hl == HL_NUMBER)) ||
           (c == '.' && prev_hl == HL_NUMBER)) {
         row->hl[i] = HL_NUMBER;
         i++;
         prev_sep = 0;
         continue;
       }
     }
 
     if (prev_sep) {
       int j;
       for (j = 0; keywords[j]; j++) {
         int klen = strlen(keywords[j]);
         int kw2 = keywords[j][klen - 1] == '|';
         if (kw2) klen--;
 
         if (!strncmp(&row->render[i], keywords[j], klen) &&
             is_separator(row->render[i + klen])) {
           memset(&row->hl[i], kw2 ? HL_KEYWORD2 : HL_KEYWORD1, klen);
           i += klen;
           break;
         }
       }
       if (keywords[j] != NULL) {
         prev_sep = 0;
         continue;
       }
     }
 
     prev_sep = is_separator(c);
     i++;
   }
 }
 
 int editorSyntaxToColor(int hl) {
   …
 }
 
 void editorSelectSyntaxHighlight() {
   …
         }
 …
 /***行操作 ***/
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
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

さて、それでは、実際に複数行にまたがる複数行コメントをハイライト表示する作業に移りましょう。そのためには、前の行が閉じられていない複数行コメントの一部であるかどうかを知る必要があります。`erow`構造体に`hl_open_comment`というブール変数を追加しましょう。また、各`erow`がファイル内の自身のインデックスを認識できるように、`idx`という整数変数も追加します。これにより、各行が前の行の`hl_open_comment`値を調べることができるようになります。

**ステップ 182** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/idx-and-hloc/kilo.c) / [idx-and-hloc](https://github.com/snaptoken/kilo-src/tree/idx-and-hloc))

```diff
 /*** 含まれるもの ***/
 …
 /*** 定義するもの ***/
 …
 /*** データ ***/
 
 struct editorSyntax {
   …
 };
 
 typedef struct erow {
+  int idx;
   int size;
   int rsize;
   char *chars;
   char *render;
   unsigned char *hl;
+  int hl_open_comment;
 } erow;
 
 struct editorConfig {
   …
 };
 
 struct editorConfig E;
 
 /*** ファイルタイプ ***/
 …
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 構文ハイライト ***/
 …
 /*** 行操作 ***/
 
 int editorRowCxToRx(erow *row, int cx) {
   …
 }
 
 int editorRowRxToCx(erow *row, int rx) {
   …
 }
 
 void editorUpdateRow(erow *row) {
   …
 }
 
 void editorInsertRow(int at, char *s, size_t len) {
   if (at < 0 || at > E.numrows) return;
 
   E.row = realloc(E.row, sizeof(erow) * (E.numrows + 1));
   memmove(&E.row[at + 1], &E.row[at], sizeof(erow) * (E.numrows - at));
 
+  E.row[at].idx = at;
+
   E.row[at].size = len;
   E.row[at].chars = malloc(len + 1);
   memcpy(E.row[at].chars, s, len);
   E.row[at].chars[len] = '\0';
 
   E.row[at].rsize = 0;
   E.row[at].render = NULL;
   E.row[at].hl = NULL;
+  E.row[at].hl_open_comment = 0;
   editorUpdateRow(&E.row[at]);
 
   E.numrows++;
   E.dirty++;
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

`idx` は、ファイルが挿入された時点での行のインデックスで初期化されます。ファイルに行が挿入または削除されるたびに、各行の `idx` が必ず更新されるようにしましょう。

**ステップ 183** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/update-idx/kilo.c) / [update-idx](https://github.com/snaptoken/kilo-src/tree/update-idx))

```diff
 /*** 含まれるもの ***/
 …
 /*** 定義 ***/
 …
 /*** データ ***/
 …
 /*** ファイルタイプ ***/
 …
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 構文ハイライト ***/
 …
 /*** 行操作 ***/
 
 int editorRowCxToRx(erow *row, int cx) {
   …
 }
 
 int editorRowRxToCx(erow *row, int rx) {
   …
 }
 
 void editorUpdateRow(erow *row) {
   …
 }
 
 void editorInsertRow(int at, char *s, size_t len) {
   if (at < 0 || at > E.numrows) return;
 
   E.row = realloc(E.row, sizeof(erow) * (E.numrows + 1));
   memmove(&E.row[at + 1], &E.row[at], sizeof(erow) * (E.numrows - at));
+  for (int j = at + 1; j <= E.numrows; j++) E.row[j].idx++;
 
   E.row[at].idx = at;
 
   E.row[at].size = len;
   E.row[at].chars = malloc(len + 1);
   memcpy(E.row[at].chars, s, len);
   E.row[at].chars[len] = '\0';
 
   E.row[at].rsize = 0;
   E.row[at].render = NULL;
   E.row[at].hl = NULL;
   E.row[at].hl_open_comment = 0;
   editorUpdateRow(&E.row[at]);
 
   E.numrows++;
   E.dirty++;
 }
 
 void editorFreeRow(erow *row) {
   …
 }
 
 void editorDelRow(int at) {
   if (at < 0 || at >= E.numrows) return;
   editorFreeRow(&E.row[at]);
   memmove(&E.row[at], &E.row[at + 1], sizeof(erow) * (E.numrows - at - 1));
+  for (int j = at; j < E.numrows - 1; j++) E.row[j].idx--;
   E.numrows--;
   E.dirty++;
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

`for`ループは、挿入または削除操作によって移動された各行のインデックスを更新します。

さて、最後のステップです。

**ステップ 184** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/propagate-highlight/kilo.c) / [propagate-highlight](https://github.com/snaptoken/kilo-src/tree/propagate-highlight))

```diff
 /*** 含まれるもの ***/
 …
 /*** 定義するもの ***/
 …
 /*** データ ***/
 …
 /*** ファイルタイプ ***/
 …
 /*** プロトタイプ ***/
 …
 /*** ターミナル ***/
 …
 /*** 構文ハイライト ***/
 
 int is_separator(int c) {
   …
 }
 
 void editorUpdateSyntax(erow *row) {
   row->hl = realloc(row->hl, row->rsize);
   memset(row->hl, HL_NORMAL, row->rsize);
 
   if (E.syntax == NULL) return;
 
   char **keywords = E.syntax->keywords;
 
   char *scs = E.syntax->singleline_comment_start;
   char *mcs = E.syntax->multiline_comment_start;
   char *mce = E.syntax->multiline_comment_end;
 
   int scs_len = scs ? strlen(scs) : 0;
   int mcs_len = mcs ? strlen(mcs) : 0;
   int mce_len = mce ? strlen(mce) : 0;
 
   int prev_sep = 1;
   int in_string = 0;
+  int in_comment = (row->idx > 0 && E.row[row->idx - 1].hl_open_comment);
 
   int i = 0;
   while (i < row->rsize) {
     char c = row->render[i];
     unsigned char prev_hl = (i > 0) ? row->hl[i - 1] : HL_NORMAL;
 
     if (scs_len && !in_string && !in_comment) {
       if (!strncmp(&row->render[i], scs, scs_len)) {
         memset(&row->hl[i], HL_COMMENT, row->rsize - i);
         break;
       }
     }
 
     if (mcs_len && mce_len && !in_string) {
       if (in_comment) {
         row->hl[i] = HL_MLCOMMENT;
         if (!strncmp(&row->render[i], mce, mce_len)) {
           memset(&row->hl[i], HL_MLCOMMENT, mce_len);
           i += mce_len;
           in_comment = 0;
           prev_sep = 1;
           continue;
         } else {
           i++;
           continue;
         }
       } else if (!strncmp(&row->render[i], mcs, mcs_len)) {
         memset(&row->hl[i], HL_MLCOMMENT, mcs_len);
         i += mcs_len;
         in_comment = 1;
         continue;
       }
     }
 
     if (E.syntax->flags & HL_HIGHLIGHT_STRINGS) {
       if (in_string) {
         row->hl[i] = HL_STRING;
         if (c == '\\' && i + 1 < row->rsize) {
           row->hl[i + 1] = HL_STRING;
           i += 2;
           continue;
         }
         if (c == in_string) in_string = 0;
         i++;
         prev_sep = 1;
         continue;
       } else {
         if (c == '"' || c == '\'') {
           in_string = c;
           row->hl[i] = HL_STRING;
           i++;
           continue;
         }
       }
     }
 
     if (E.syntax->flags & HL_HIGHLIGHT_NUMBERS) {
       if ((isdigit(c) && (prev_sep || prev_hl == HL_NUMBER)) ||
           (c == '.' && prev_hl == HL_NUMBER)) {
         row->hl[i] = HL_NUMBER;
         i++;
         prev_sep = 0;
         continue;
       }
     }
 
     if (prev_sep) {
       int j;
       for (j = 0; keywords[j]; j++) {
         int klen = strlen(keywords[j]);
         int kw2 = keywords[j][klen - 1] == '|';
         if (kw2) klen--;
 
         if (!strncmp(&row->render[i], keywords[j], klen) &&
             is_separator(row->render[i + klen])) {
           memset(&row->hl[i], kw2 ? HL_KEYWORD2 : HL_KEYWORD1, klen);
           i += klen;
           break;
         }
       }
       if (keywords[j] != NULL) {
         prev_sep = 0;
         continue;
       }
     }
 
     prev_sep = is_separator(c);
     i++;
   }
+
+  int changed = (row->hl_open_comment != in_comment);
+  row->hl_open_comment = in_comment;
+  if (changed && row->idx + 1 < E.numrows)
+    editorUpdateSyntax(&E.row[row->idx + 1]);
 }
 
 int editorSyntaxToColor(int hl) {
   …
 }
 
 void editorSelectSyntaxHighlight() {
   …
         }
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
 …
 /*** 初期化 ***/
```

*コンパイル*

`editorUpdateSyntax()` 関数の冒頭付近で、前の行に閉じられていない複数行コメントがある場合、`in_comment` を true に初期化します。そうすると、現在の行は最初から複数行コメントとしてハイライト表示されます。

`editorUpdateSyntax()` の最後に、現在の行の `hl_open_comment` の値を、行全体を処理した後に `in_comment` が残した状態に設定します。これにより、行が閉じられていない複数行コメントとして終了したかどうかがわかります。

次に、ファイル内の次の行の構文を更新することを検討する必要があります。これまでは、ユーザーが特定の行を変更した場合にのみ、その行の構文を更新してきました。しかし、複数行コメントの場合、ユーザーは1行を変更するだけでファイル全体をコメントアウトできます。そのため、現在の行に続くすべての行の構文を更新する必要があるようです。ただし、この行の`hl_open_comment`の値が変更されていない場合、次の行のハイライト表示は変更されないことがわかっています。そこで、`hl_open_comment`が変更されたかどうかを確認し、変更があった場合（かつファイルに次の行がある場合）にのみ、次の行で`editorUpdateSyntax()`を呼び出します。`editorUpdateSyntax()`は次の行で自身を呼び出し続けるため、変更はますます多くの行に伝播し、いずれかの行が変更されなくなるまで続きます。その時点で、それ以降のすべての行も変更されていないことがわかります。

## 完了しました

これでテキストエディタは完成です。[付録](https://viewsourcecode.org/snaptoken/kilo/08.appendices.html)には、エディタを拡張する際に役立つ機能のアイデアがいくつか掲載されています。

[ページの先頭](https://viewsourcecode.org/snaptoken/kilo/07.syntaxHighlighting.html#)
