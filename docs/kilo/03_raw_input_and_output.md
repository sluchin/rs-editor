# 3. 生の入出力

## Ctrl+Q を押して終了

前章では、Ctrlキーとアルファベットキーの組み合わせがバイト1～26に対応することが分かりました。これを利用してCtrlキーの組み合わせを検出し、エディタ内のさまざまな操作に割り当ててみましょう。まずはCtrl+Qを終了操作に割り当ててみます。

**ステップ 20** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/ctrl-q/kilo.c) / [ctrl-q](https://github.com/snaptoken/kilo-src/tree/ctrl-q))

```diff
 /*** インクルード ***/
 
 #include <ctype.h>
 #include <errno.h>
 #include <stdio.h>
 #include <stdlib.h>
 #include <termios.h>
 #include <unistd.h>
 
+/*** 定義 ***/
+
+#define CTRL_KEY(k) ((k) & 0x1f)
+
 /*** データ ***/
 …
 /*** 端末 ***/
 …
 /*** 初期化 ***/
 
 int main() {
   enableRawMode();
 
   while (1) {
     char c = '\0';
     if (read(STDIN_FILENO, &c, 1) == -1 && errno != EAGAIN) die("read");
     if (iscntrl(c)) {
       printf("%d\r\n", c);
     } else {
       printf("%d ('%c')\r\n", c, c);
     }
+    if (c == CTRL_KEY('q')) break;
   }
 
   return 0;
 }
```

*コンパイル*

`CTRL_KEY`マクロは、バイナリ値`00011111`と文字をビット単位でAND演算します。 （C言語では、バイナリリテラルがないため、ビットマスクは通常16進数で指定します。16進数の方が、慣れれば簡潔で読みやすいからです。）言い換えれば、文字の上位3ビットを`0`に設定します。これは、ターミナルでCtrlキーを押す動作と同じです。Ctrlキーと組み合わせたキーのビット5と6を取り出し、それを送信します。（慣例として、ビット番号は0から始まります。）ASCII文字セットは、意図的にこのように設計されているようです。（同様に、ビット5を設定およびクリアすることで、小文字と大文字を切り替えることができるように設計されています。）

## キーボード入力のリファクタリング

低レベルのキー入力読み取り関数と、キー入力をエディタ操作にマッピングする関数を作成しましょう。また、この時点でキー入力の出力は停止します。

**ステップ 21** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/refactor-input/kilo.c) / [refactor-input](https://github.com/snaptoken/kilo-src/tree/refactor-input))

```diff
 /*** インクルード ***/
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
 
+char editorReadKey() {
+  int nread;
+  char c;
+  while ((nread = read(STDIN_FILENO, &c, 1)) != 1) {
+    if (nread == -1 && errno != EAGAIN) die("read");
+  }
+  return c;
+}
+
+/*** 入力 ***/
+
+void editorProcessKeypress() {
+  char c = editorReadKey();
+
+  switch (c) {
+    case CTRL_KEY('q'):
+      exit(0);
+      break;
+  }
+}
+
 /*** 初期化 ***/
 
 int main() {
   enableRawMode();
 
   while (1) {
+    editorProcessKeypress();
   }
 
   return 0;
 }
```

*コンパイル*

`editorReadKey()` の役割は、キーが1回押されるまで待機し、そのキーの値を返すことです。後ほど、この関数を拡張してエスケープシーケンスを処理できるようにします。エスケープシーケンスでは、矢印キーのように、1回のキー押下を表す複数のバイトを読み取る必要があります。

`editorProcessKeypress()` はキー入力を待ち、その後処理を行います。処理後、さまざまな Ctrl キーの組み合わせやその他の特殊キーを異なるエディタ機能にマッピングし、英数字やその他の印刷可能なキーの文字を編集中のテキストに挿入します。

`editorReadKey()` は低レベルの端末入力を扱うため `/*** terminal ***/` セクションに属しますが、`editorProcessKeypress()` ははるかに高レベルでキーをエディタ関数にマッピングするため、新しい `/*** input ***/` セクションに属します。

これで`main()`関数は大幅に簡略化されましたが、今後もこの状態を維持していきたいと思います。

## 画面をクリアする

キーを押すたびに、エディタのユーザーインターフェースを画面に表示します。まずは画面をクリアすることから始めましょう。

**ステップ 22** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/clear-screen/kilo.c) / [clear-screen](https://github.com/snaptoken/kilo-src/tree/clear-screen))

```diff
 /*** インクルード ***/
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
 
 char editorReadKey() {
   …
   }
 …
+/*** 出力 ***/
+
+void editorRefreshScreen() {
+  write(STDOUT_FILENO, "\x1b[2J", 4);
+}
+
 /*** 入力 ***/
 …
 /*** 初期化 ***/
 
 int main() {
   enableRawMode();
 
   while (1) {
+    editorRefreshScreen();
     editorProcessKeypress();
   }
 
   return 0;
 }
```

*コンパイル*

`write()`と`STDOUT_FILENO`は`<unistd.h>`から来ています。

`write()` 呼び出しの `4` は、端末に `4` バイトを書き込むことを意味します。最初のバイトは `\x1b` で、これはエスケープ文字、つまり 10 進数で `27` です。(`\x1b` を覚えておいてください。たくさん使います。) 残りの 3 バイトは `[2J` です。

端末にエスケープシーケンスを書き込みます。エスケープシーケンスは常にエスケープ文字（`27`）で始まり、その後に`[`文字が続きます。エスケープシーケンスは、テキストの色付け、カーソルの移動、画面の一部のクリアなど、さまざまなテキスト書式設定タスクを端末に実行するように指示します。

画面をクリアするために、`J` コマンド ([画面内消去](http://vt100.net/docs/vt100-ug/chapter3.html#ED)) を使用します。エスケープシーケンスコマンドは引数を取り、引数はコマンドの前に指定します。この場合、引数は `2` で、画面全体をクリアすることを意味します。`<esc>[1J` はカーソル位置まで画面をクリアし、`<esc>[0J` はカーソル位置から画面の末尾まで画面をクリアします。また、`0` は `J` のデフォルト引数なので、`<esc>[J` だけでもカーソル位置から画面の末尾まで画面をクリアします。

テキストエディタでは、主に[VT100](https://en.wikipedia.org/wiki/VT100)エスケープシーケンスを使用します。これは、最新のターミナルエミュレータで広くサポートされています。各エスケープシーケンスの詳細については、[VT100ユーザーガイド](http://vt100.net/docs/vt100-ug/chapter3.html)を参照してください。

可能な限り多くの端末をサポートしたい場合は、[ncurses](https://en.wikipedia.org/wiki/Ncurses)ライブラリを使用できます。このライブラリは、[terminfo](https://en.wikipedia.org/wiki/Terminfo)データベースを使用して、端末の機能と、その特定の端末で使用するエスケープシーケンスを特定します。

## カーソルの位置を変更する

`<esc>[2J` コマンドを実行すると、カーソルが画面の下部に移動することにお気づきかもしれません。エディタのインターフェースを上から下へ描画できるように、カーソルを左上隅に移動しましょう。

**ステップ 23** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/cursor-home/kilo.c) / [cursor-home](https://github.com/snaptoken/kilo-src/tree/cursor-home))

```diff
 /*** インクルード ***/
 …
 /*** 定義 ***/
 …
 /*** データ ***/
 …
 /*** ターミナル ***/
 …
 /*** 出力 ***/
 
 void editorRefreshScreen() {
   write(STDOUT_FILENO, "\x1b[2J", 4);
+  write(STDOUT_FILENO, "\x1b[H", 3);
 }
 
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイル*

このエスケープシーケンスはわずか `3` バイトの長さで、カーソルの位置を指定するために `H` コマンド ([カーソル位置](http://vt100.net/docs/vt100-ug/chapter3.html#CUP)) を使用します。`H` コマンドは実際には 2 つの引数を取ります。カーソルを配置する行番号と列番号です。したがって、80×24 サイズの端末でカーソルを画面の中央に配置したい場合は、`<esc>[12;40H` コマンドを使用できます。(複数の引数は `;` 文字で区切られます。) `H` のデフォルトの引数はどちらも `1` なので、両方の引数を省略すると、`<esc>[1;1H` コマンドを送信した場合と同様に、カーソルが最初の行と最初の列に配置されます。(行と列の番号は `0` ではなく `1` から始まります。)

## 終了時に画面をクリアする

プログラム終了時に画面をクリアし、カーソルの位置を再設定しましょう。画面のレンダリング中にエラーが発生した場合、画面上に不要なデータが残ってしまうのを避けたいですし、カーソルの位置に関係なくエラーメッセージが表示されるのも避けたいからです。

**ステップ 24** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/clean-exit/kilo.c) / [clean-exit](https://github.com/snaptoken/kilo-src/tree/clean-exit))

```diff
 /*** インクルード ***/
 …
 /*** 定義 ***/
 …
 /*** データ ***/
 …
 /*** ターミナル ***/
 
 void die(const char *s) {
+  write(STDOUT_FILENO, "\x1b[2J", 4);
+  write(STDOUT_FILENO, "\x1b[H", 3);
+
   perror(s);
   exit(1);
 }
 
 void disableRawMode() {
   …
 }
 
 void enableRawMode() {
   …
 }
 
 char editorReadKey() {
   …
   }
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 
 void editorProcessKeypress() {
   char c = editorReadKey();
 
   switch (c) {
     case CTRL_KEY('q'):
+      write(STDOUT_FILENO, "\x1b[2J", 4);
+      write(STDOUT_FILENO, "\x1b[H", 3);
       exit(0);
       break;
   }
 }
 
 /*** 初期化 ***/
```

*コンパイル*

画面をクリアしたい終了ポイントは 2 つあります。1 つは `die()` を実行するとき、もう 1 つはユーザーが Ctrl-Q を押して終了するときです。

プログラム終了時に画面をクリアするために `atexit()` を使用することもできますが、そうすると `die()` によって出力されたエラーメッセージが出力直後に消去されてしまいます。

## チルダ

さあ、描画を始めましょう。[vim](http://www.vim.org/)のように、画面の左側にチルダ（`~`）の列を描画します。テキストエディタでは、編集中のファイルの末尾以降に続く行の先頭にチルダを描画します。

**ステップ 25** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/tildes/kilo.c) / [tildes](https://github.com/snaptoken/kilo-src/tree/tildes))

```diff
 /*** インクルード ***/
 …
 /*** 定義 ***/
 …
 /*** データ ***/
 …
 /*** ターミナル ***/
 …
 /*** 出力 ***/
 
+void editorDrawRows() {
+  int y;
+  for (y = 0; y < 24; y++) {
+    write(STDOUT_FILENO, "~\r\n", 3);
+  }
+}
+
 void editorRefreshScreen() {
   write(STDOUT_FILENO, "\x1b[2J", 4);
   write(STDOUT_FILENO, "\x1b[H", 3);
+
+  editorDrawRows();
+
+  write(STDOUT_FILENO, "\x1b[H", 3);
 }
 
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイル*

`editorDrawRows()` は、編集中のテキストバッファの各行を描画します。現時点では、各行にチルダが描画されます。これは、その行がファイルの一部ではなく、テキストを含めることができないことを意味します。

端末のサイズがまだわからないため、何行描画すればよいかもわかりません。とりあえず、24行描画します。

描画が終わったら、もう一度 `<esc>[H` エスケープシーケンスを実行して、カーソルを左上隅に戻します。

## グローバル状態

次の目標はターミナルのサイズを取得し、`editorDrawRows()` で描画する行数を把握することです。しかしその前に、エディタの状態を格納するグローバル構造体を設定しましょう。この構造体は、ターミナルの幅と高さを格納するために使用します。とりあえず、`orig_termios` グローバル変数を構造体に追加しておきましょう。

**ステップ 26** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/global-state/kilo.c) / [global-state](https://github.com/snaptoken/kilo-src/tree/global-state))

```diff
 /*** インクルード ***/
 …
 /*** 定義 ***/
 …
 /*** データ ***/
 
+struct editorConfig {
+  struct termios orig_termios;
+};
+
+struct editorConfig E;
 
 /*** ターミナル ***/
 
 void die(const char *s) {
   …
 }
 
 void disableRawMode() {
+  if (tcsetattr(STDIN_FILENO, TCSAFLUSH, &E.orig_termios) == -1)
     die("tcsetattr");
 }
 
 void enableRawMode() {
+  if (tcgetattr(STDIN_FILENO, &E.orig_termios) == -1) die("tcgetattr");
   atexit(disableRawMode);
 
+  struct termios raw = E.orig_termios;
   raw.c_iflag &= ~(BRKINT | ICRNL | INPCK | ISTRIP | IXON);
   raw.c_oflag &= ~(OPOST);
   raw.c_cflag |= (CS8);
   raw.c_lflag &= ~(ECHO | ICANON | IEXTEN | ISIG);
   raw.c_cc[VMIN] = 0;
   raw.c_cc[VTIME] = 1;
 
   if (tcsetattr(STDIN_FILENO, TCSAFLUSH, &raw) == -1) die("tcsetattr");
 }
 
 char editorReadKey() {
   …
   }
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

エディタの状態を格納するグローバル変数は「E」という名前です。「orig_termios」という文字列をすべて「E.orig_termios」に置き換える必要があります。

## ウィンドウサイズを簡単に設定する方法

ほとんどのシステムでは、`ioctl()` を `TIOCGWINSZ` リクエストとともに呼び出すだけで、端末のサイズを取得できます。（私の知る限り、これは **T**erminal **IOC**tl (それ自体は **I**nput/**Output **C**on**t**ro**l**) **G**et **WIN**dow **S**i**Z**e の略です。）

**ステップ 27** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/ioctl/kilo.c) / [ioctl](https://github.com/snaptoken/kilo-src/tree/ioctl))

```diff
 /*** インクルード ***/
 
 #include <ctype.h>
 #include <errno.h>
 #include <stdio.h>
 #include <stdlib.h>
+#include <sys/ioctl.h>
 #include <termios.h>
 #include <unistd.h>
 
 /*** 定義 ***/
 …
 /*** データ ***/
 …
 /*** 端末 ***/
 
 void die(const char *s) {
   …
 }
 
 void disableRawMode() {
   …
 }
 
 void enableRawMode() {
   …
 }
 
 char editorReadKey() {
   …
 }
 
+int getWindowSize(int *rows, int *cols) {
+  struct winsize ws;
+
+  if (ioctl(STDOUT_FILENO, TIOCGWINSZ, &ws) == -1 || ws.ws_col == 0) {
+    return -1;
+  } else {
+    *cols = ws.ws_col;
+    *rows = ws.ws_row;
+    return 0;
+  }
+}
+
 /*** 出力 ***/
 …
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

`ioctl()`、`TIOCGWINSZ`、および`struct winsize`は`<sys/ioctl.h>`から取得されます。

成功した場合、`ioctl()` は端末の幅の列数と高さの行数を、指定された `winsize` 構造体に格納します。失敗した場合、`ioctl()` は `-1` を返します。また、返された値が `0` でないことを確認します。これは、誤った結果となる可能性があるためです。`ioctl()` がどちらの方法で失敗した場合でも、`getWindowSize()` は `-1` を返して失敗を報告します。成功した場合は、関数に渡された `int` 参照を設定することで値を返します。（これは、C 言語で関数が複数の値を返す一般的な方法です。また、戻り値を使用して成功または失敗を示すこともできます。）

それでは、`screenrows`と`screencols`をグローバルエディタ状態に追加し、`getWindowSize()`を呼び出してこれらの値を設定しましょう。

**ステップ 28** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/init-editor/kilo.c) / [init-editor](https://github.com/snaptoken/kilo-src/tree/init-editor))

```diff
 /*** インクルード ***/
 …
 /*** 定義 ***/
 …
 /*** データ ***/
 
 struct editorConfig {
+  int screenrows;
+  int screencols;
   struct termios orig_termios;
 };
 
 struct editorConfig E;
 
 /*** ターミナル ***/
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 …
 /*** 初期化 ***/
 
+void initEditor() {
+  if (getWindowSize(&E.screenrows, &E.screencols) == -1) die("getWindowSize");
+}
+
 int main() {
   enableRawMode();
+  initEditor();
 
   while (1) {
     editorRefreshScreen();
     editorProcessKeypress();
   }
 
   return 0;
 }
```

*コンパイルはされるが、目に見える効果はない*

`initEditor()` の役割は、`E` 構造体内のすべてのフィールドを初期化することです。

これで、画面に適切な数のチルダを表示する準備が整いました。

**ステップ 29** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/screenrows/kilo.c) / [screenrows](https://github.com/snaptoken/kilo-src/tree/screenrows))

```diff
 /*** インクルード ***/
 …
 /*** 定義 ***/
 …
 /*** データ ***/
 …
 /*** ターミナル ***/
 …
 /*** 出力 ***/
 
 void editorDrawRows() {
   int y;
+  for (y = 0; y < E.screenrows; y++) {
     write(STDOUT_FILENO, "~\r\n", 3);
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

## ウィンドウサイズ、難しい方法

`ioctl()` はすべてのシステムでウィンドウサイズを要求できるとは限らないため、ウィンドウサイズを取得するための代替手段を提供します。

この戦略は、カーソルを画面の右下隅に配置し、エスケープシーケンスを使用してカーソルの位置を照会することです。これにより、画面上の行数と列数がわかります。

まず、カーソルを右下隅に移動させましょう。

**ステップ 30** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/bottom-right/kilo.c) / [bottom-right](https://github.com/snaptoken/kilo-src/tree/bottom-right))

```diff
 /*** 含める ***/
 …
 /*** 定義する ***/
 …
 /*** データ ***/
 …
 /*** 端末 ***/
 
 void die(const char *s) {
   …
 }
 
 void disableRawMode() {
   …
 }
 
 void enableRawMode() {
   …
 }
 
 char editorReadKey() {
   …
 }
 
 int getWindowSize(int *rows, int *cols) {
   struct winsize ws;
 
+  if (1 || ioctl(STDOUT_FILENO, TIOCGWINSZ, &ws) == -1 || ws.ws_col == 0) {
+    if (write(STDOUT_FILENO, "\x1b[999C\x1b[999B", 12) != 12) return -1;
+    editorReadKey();
     return -1;
   } else {
     *cols = ws.ws_col;
     *rows = ws.ws_row;
     return 0;
   }
 }
 
 /*** 出力 ***/
 …
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイル*

コードからお分かりいただけるように、「カーソルを右下隅に移動する」といった単純なコマンドは存在しません。

ここでは、2つのエスケープシーケンスを連続して送信しています。`C`コマンド（[カーソル前進](http://vt100.net/docs/vt100-ug/chapter3.html#CUF)）はカーソルを右に移動し、`B`コマンド（[カーソル下降](http://vt100.net/docs/vt100-ug/chapter3.html#CUD)）はカーソルを下に移動します。引数は、右または下方向にどれだけ移動するかを指定します。ここでは、カーソルが画面の右端と下端に到達するように、非常に大きな値である`999`を使用しています。

`C` と `B` コマンドは、カーソルが画面の端を超えないようにするために特別に [ドキュメント](http://vt100.net/docs/vt100-ug/chapter3.html#CUD) 作成されています。`<esc>[999;999H` コマンドを使用しない理由は、[ドキュメント](http://vt100.net/docs/vt100-ug/chapter3.html#CUP) にカーソルを画面外に移動しようとしたときに何が起こるかが明記されていないためです。

開発中のフォールバック分岐をテストできるように、一時的に`if`条件の先頭に`1 ||`を追加していることに注意してください。

この時点では `getWindowSize()` から常に `-1` (エラーが発生したことを意味します) が返されるため、プログラムが `die()` を呼び出して画面をクリアする前に、エスケープシーケンスの結果を確認できるように `editorReadKey()` を呼び出します。プログラムを実行すると、カーソルが画面の右下隅に移動し、キーを押すと、画面がクリアされた後に `die()` によって表示されるエラーメッセージが表示されます。

次に、カーソル位置を取得する必要があります。`n` コマンド ([デバイスステータスレポート](http://vt100.net/docs/vt100-ug/chapter3.html#DSR)) を使用して、端末にステータス情報を問い合わせることができます。カーソル位置を問い合わせるために、引数として `6` を指定します。その後、標準入力から応答を読み取ることができます。応答がどのように表示されるかを確認するために、標準入力から各文字を出力してみましょう。

**ステップ 31** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/cursor-query/kilo.c) / [cursor-query](https://github.com/snaptoken/kilo-src/tree/cursor-query))

```diff
 /*** include ***/
 …
 /*** defined ***/
 …
 /*** data ***/
 …
 /*** terminal ***/
 
 void die(const char *s) {
   …
 }
 
 void disableRawMode() {
   …
 }
 
 void enableRawMode() {
   …
 }
 
 char editorReadKey() {
   …
 }
 
+int getCursorPosition(int *rows, int *cols) {
+  if (write(STDOUT_FILENO, "\x1b[6n", 4) != 4) return -1;
+
+  printf("\r\n");
+  char c;
+  while (read(STDIN_FILENO, &c, 1) == 1) {
+    if (iscntrl(c)) {
+      printf("%d\r\n", c);
+    } else {
+      printf("%d ('%c')\r\n", c, c);
+    }
+  }
+
+  editorReadKey();
+
+  return -1;
+}
+
 int getWindowSize(int *rows, int *cols) {
   struct winsize ws;
 
   if (1 || ioctl(STDOUT_FILENO, TIOCGWINSZ, &ws) == -1 || ws.ws_col == 0) {
     if (write(STDOUT_FILENO, "\x1b[999C\x1b[999B", 12) != 12) return -1;
+    return getCursorPosition(rows, cols);
   } else {
     *cols = ws.ws_col;
     *rows = ws.ws_row;
     return 0;
   }
 }
 
 /*** 出力 ***/
 …
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイル*

応答はエスケープシーケンスです！エスケープ文字（`27`）に続いて`[`文字、そして実際の応答である`24;80R`などが続きます。（このエスケープシーケンスについては、[カーソル位置レポート](http://vt100.net/docs/vt100-ug/chapter3.html#CPR)で説明されています。）

前回と同様に、終了時に画面がクリアされる前にデバッグ出力を確認できるように、一時的に `editorReadKey()` を呼び出す処理を挿入しました。

（注：**Windows版Bash**を使用している場合、`read()`はタイムアウトしないため、無限ループに陥ります。その場合は、外部からプロセスを強制終了するか、コマンドプロンプトウィンドウを終了して再度開く必要があります。）

このレスポンスを解析する必要があります。まずは、バッファに読み込みましょう。文字「R」に到達するまで、文字を読み込み続けます。

**ステップ 32** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/response-buffer/kilo.c) / [response-buffer](https://github.com/snaptoken/kilo-src/tree/response-buffer))

```diff
 /*** include ***/
 …
 /*** defined ***/
 …
 /*** data ***/
 …
 /*** terminal ***/
 
 void die(const char *s) {
   …
 }
 
 void disableRawMode() {
   …
 }
 
 void enableRawMode() {
   …
 }
 
 char editorReadKey() {
   …
 }
 
 int getCursorPosition(int *rows, int *cols) {
+  char buf[32];
+  unsigned int i = 0;
+
   if (write(STDOUT_FILENO, "\x1b[6n", 4) != 4) return -1;
 
+  while (i < sizeof(buf) - 1) {
+    if (read(STDIN_FILENO, &buf[i], 1) != 1) break;
+    if (buf[i] == 'R') break;
+    i++;
   }
+  buf[i] = '\0';
+
+  printf("\r\n&buf[1]: '%s'\r\n", &buf[1]);
 
   editorReadKey();
 
   return -1;
 }
 
 int getWindowSize(int *rows, int *cols) {
   …
   }
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイル*

バッファを出力する際、`'\x1b'` 文字は出力したくありません。なぜなら、端末がこれをエスケープシーケンスとして解釈し、表示しないからです。そこで、`&buf[1]` を `printf()` に渡すことで、`buf` の最初の文字をスキップします。`printf()` は文字列が `0` バイトで終わることを想定しているため、`buf` の最後のバイトに `'\0'` を代入します。

プログラムを実行すると、`buf` に `<esc>[24;80` という形式の応答が格納されていることがわかります。`sscanf()` を使用して、そこから 2 つの数値を解析してみましょう。

**ステップ 33** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/parse-response/kilo.c) / [parse-response](https://github.com/snaptoken/kilo-src/tree/parse-response))

```diff
 /*** include ***/
 …
 /*** defined ***/
 …
 /*** data ***/
 …
 /*** terminal ***/
 
 void die(const char *s) {
   …
 }
 
 void disableRawMode() {
   …
 }
 
 void enableRawMode() {
   …
 }
 
 char editorReadKey() {
   …
 }
 
 int getCursorPosition(int *rows, int *cols) {
   char buf[32];
   unsigned int i = 0;
 
   if (write(STDOUT_FILENO, "\x1b[6n", 4) != 4) return -1;
 
   while (i < sizeof(buf) - 1) {
     if (read(STDIN_FILENO, &buf[i], 1) != 1) break;
     if (buf[i] == 'R') break;
     i++;
   }
   buf[i] = '\0';
 
+  if (buf[0] != '\x1b' || buf[1] != '[') return -1;
+  if (sscanf(&buf[2], "%d;%d", rows, cols) != 2) return -1;
 
+  return 0;
 }
 
 int getWindowSize(int *rows, int *cols) {
   …
   }
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイル*

`sscanf()` は `<stdio.h>` から来ています。

まず、エスケープシーケンスで応答したことを確認します。次に、`buf` の 3 番目の文字へのポインタを `sscanf()` に渡しますが、`'\x1b'` と `'['` の文字はスキップします。つまり、`24;80` という形式の文字列を `sscanf()` に渡します。また、文字列 `%d;%d` も渡します。これは、セミコロンで区切られた 2 つの整数を解析し、その値を `rows` と `cols` 変数に格納するように指示するものです。

ウィンドウサイズを取得するための代替手段が完成しました。`editorDrawRows()`を実行すると、端末の高さに応じた正しい数のチルダが表示されるはずです。

これでうまくいくことがわかったので、if 条件に入れた `1 ||` を一時的に削除しましょう。

**ステップ 34** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/back-to-ioctl/kilo.c) / [back-to-ioctl](https://github.com/snaptoken/kilo-src/tree/back-to-ioctl))

```diff
 /*** include ***/
 …
 /*** defined ***/
 …
 /*** data ***/
 …
 /*** terminal ***/
 
 void die(const char *s) {
   …
 }
 
 void disableRawMode() {
   …
 }
 
 void enableRawMode() {
   …
 }
 
 char editorReadKey() {
   …
 }
 
 int getCursorPosition(int *rows, int *cols) {
   …
 }
 
 int getWindowSize(int *rows, int *cols) {
   struct winsize ws;
 
+  if (ioctl(STDOUT_FILENO, TIOCGWINSZ, &ws) == -1 || ws.ws_col == 0) {
     if (write(STDOUT_FILENO, "\x1b[999C\x1b[999B", 12) != 12) return -1;
     return getCursorPosition(rows, cols);
   } else {
     *cols = ws.ws_col;
     *rows = ws.ws_row;
     return 0;
   }
 }
 
 /*** 出力 ***/
 …
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

## 最後の行

画面の最後の行にチルダがないことにお気づきかもしれません。これは、コードの小さなバグが原因です。最後のチルダを出力すると、他の行と同様に「\r\n」を出力しますが、これによりターミナルがスクロールして新しい空白行のためのスペースが確保されます。最後の行は例外として「\r\n」を出力するようにしましょう。

**ステップ 35** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/last-line/kilo.c) / [last-line](https://github.com/snaptoken/kilo-src/tree/last-line))

```diff
 /*** インクルード ***/
 …
 /*** 定義 ***/
 …
 /*** データ ***/
 …
 /*** ターミナル ***/
 …
 /*** 出力 ***/
 
 void editorDrawRows() {
   int y;
   for (y = 0; y < E.screenrows; y++) {
+    write(STDOUT_FILENO, "~", 1);
+
+    if (y < E.screenrows - 1) {
+      write(STDOUT_FILENO, "\r\n", 2);
+    }
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

## バッファの追加

画面を更新するたびに小さな `write()` を何度も実行するのは良い方法ではありません。画面全体が一度に更新されるように、大きな `write()` を 1 回実行する方が良いでしょう。そうしないと、`write()` の間に予測不能な小さな間隔が生じ、不快なちらつき効果が発生する可能性があります。

すべての `write()` 呼び出しを、文字列をバッファに追加し、最後にそのバッファを `write()` で出力するコードに置き換えたいと考えています。残念ながら、C 言語には動的文字列がないため、追加操作のみをサポートする独自の動的文字列型を作成します。

まず、新しい `/*** append buffer ***/` セクションを作成し、その下に `abuf` 構造体を定義します。

**ステップ 36** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/abuf-struct/kilo.c) / [abuf-struct](https://github.com/snaptoken/kilo-src/tree/abuf-struct))

```diff
 /*** 含める ***/
 …
 /*** 定義する ***/
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
 
 char editorReadKey() {
   …
 }
 
 int getCursorPosition(int *rows, int *cols) {
   …
 }
 
 int getWindowSize(int *rows, int *cols) {
   …
   }
 …
+/*** バッファを追加する ***/
+
+struct abuf {
+  char *b;
+  int len;
+};
+
+#define ABUF_INIT {NULL, 0}
+
 /*** 出力 ***/
 …
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

追加バッファは、メモリ内のバッファへのポインタと長さで構成されます。空のバッファを表す定数`ABUF_INIT`を定義します。これは、`abuf`型のコンストラクタとして機能します。

次に、`abAppend()` 操作と `abFree()` デストラクタを定義しましょう。

**ステップ 37** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/abuf-append/kilo.c) / [abuf-append](https://github.com/snaptoken/kilo-src/tree/abuf-append))

```diff
 /*** インクルード ***/
 
 #include <ctype.h>
 #include <errno.h>
 #include <stdio.h>
 #include <stdlib.h>
+#include <string.h>
 #include <sys/ioctl.h>
 #include <termios.h>
 #include <unistd.h>
 
 /*** 定義 ***/
 …
 /*** データ ***/
 …
 /*** 端末 ***/
 …
 /*** 追加バッファ ***/
 
 struct abuf {
   …
 };
 
 #define ABUF_INIT {NULL, 0}
 
+void abAppend(struct abuf *ab, const char *s, int len) {
+  char *new = realloc(ab->b, ab->len + len);
+
+  if (new == NULL) return;
+  memcpy(&new[ab->len], s, len);
+  ab->b = new;
+  ab->len += len;
+}
+
+void abFree(struct abuf *ab) {
+  free(ab->b);
+}
+
 /*** 出力 ***/
 …
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

`realloc()` と `free()` は `<stdlib.h>` から来ています。`memcpy()` は `<string.h>` から来ています。

`abuf` に文字列 `s` を追加するには、まず新しい文字列を格納するのに十分なメモリを確保する必要があります。`realloc()` 関数に、現在の文字列のサイズに、追加する文字列のサイズを加えたサイズのメモリブロックを割り当てるように指示します。`realloc()` 関数は、既に割り当て済みのメモリブロックのサイズを拡張するか、現在のメモリブロックを `free()` して、新しい文字列を格納するのに十分な大きさの新しいメモリブロックを別の場所に割り当てます。

次に、`memcpy()` を使用して、バッファ内の現在のデータの末尾の後に文字列 `s` をコピーし、`abuf` のポインタと長さを新しい値に更新します。

`abFree()` は、`abuf` が使用する動的メモリを解放するデストラクタです。

さて、これで`abuf`型を使用する準備が整いました。

**ステップ 38** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/use-abuf/kilo.c) / [use-abuf](https://github.com/snaptoken/kilo-src/tree/use-abuf))

```diff
 /*** include ***/
 …
 /*** defined ***/
 …
 /*** data ***/
 …
 /*** terminal ***/
 …
 /*** append buffer ***/
 …
 /*** output ***/
 
+void editorDrawRows(struct abuf *ab) {
   int y;
   for (y = 0; y < E.screenrows; y++) {
+    abAppend(ab, "~", 1);
 
     if (y < E.screenrows - 1) {
+      abAppend(ab, "\r\n", 2);
     }
   }
 }
 
 void editorRefreshScreen() {
+  struct abuf ab = ABUF_INIT;
 
+  abAppend(&ab, "\x1b[2J", 4);
+  abAppend(&ab, "\x1b[H", 3);
 
+  editorDrawRows(&ab);
+
+  abAppend(&ab, "\x1b[H", 3);
+
+  write(STDOUT_FILENO, ab.b, ab.len);
+  abFree(&ab);
 }
 
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

`editorRefreshScreen()` では、まず `ABUF_INIT` を代入して `ab` という名前の新しい `abuf` を初期化します。次に、`write(STDOUT_FILENO, ...)` の各出現箇所を `abAppend(&ab, ...)` に置き換えます。また、`ab` を `editorDrawRows()` にも渡すことで、`editorDrawRows()` でも `abAppend()` が使用できるようになります。最後に、`write()` でバッファの内容を標準出力に出力し、`abuf` が使用していたメモリを解放します。

## 再描画時にカーソルを非表示にする

画面のちらつきの原因として考えられるもう一つの要因があります。それは、端末が画面に描画している間、カーソルが画面の中央付近に一瞬表示されてしまうことです。これを防ぐため、画面を更新する前にカーソルを非表示にし、更新が完了したらすぐに再び表示するようにします。

**ステップ 39** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/hide-cursor/kilo.c) / [hide-cursor](https://github.com/snaptoken/kilo-src/tree/hide-cursor))

```diff
 /*** インクルード ***/
 …
 /*** 定義 ***/
 …
 /*** データ ***/
 …
 /*** ターミナル ***/
 …
 /*** 追加バッファ ***/
 …
 /*** 出力 ***/
 
 void editorDrawRows(struct abuf *ab) {
   …
 }
 
 void editorRefreshScreen() {
   struct abuf ab = ABUF_INIT;
 
+  abAppend(&ab, "\x1b[?25l", 6);
   abAppend(&ab, "\x1b[2J", 4);
   abAppend(&ab, "\x1b[H", 3);
 
   editorDrawRows(&ab);
 
   abAppend(&ab, "\x1b[H", 3);
+  abAppend(&ab, "\x1b[?25h", 6);
 
   write(STDOUT_FILENO, ab.b, ab.len);
   abFree(&ab);
 }
 
 /*** 入力 ***/
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

エスケープシーケンスを使用して、端末にカーソルを非表示/表示するように指示します。`h` および `l` コマンド ([Set Mode](http://vt100.net/docs/vt100-ug/chapter3.html#SM)、[Reset Mode](http://vt100.net/docs/vt100-ug/chapter3.html#RM)) は、さまざまな端末機能または [“モード”](http://vt100.net/docs/vt100-ug/chapter3.html#S3.3.4) をオン/オフするために使用されます。先ほどリンクした VT100 ユーザーガイドには、上記で使用した引数 `?25` は記載されていません。カーソルの非表示/表示機能は、[後の VT モデル](http://vt100.net/docs/vt510-rm/DECTCEM.html) に登場したようです。そのため、一部の端末ではカーソルの表示/非表示をサポートしていない場合がありますが、サポートしていない場合は、エスケープシーケンスは無視されます。この場合、それは大きな問題ではありません。

## 一度に1行ずつクリアする

画面を更新するたびに画面全体をクリアするのではなく、各行を再描画する際に各行をクリアする方が最適のようです。`<esc>[2J` (画面全体をクリア) エスケープシーケンスを削除し、代わりに描画する各行の最後に `<esc>[K` シーケンスを追加しましょう。

**ステップ 40** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/clear-line/kilo.c) / [clear-line](https://github.com/snaptoken/kilo-src/tree/clear-line))

```diff
   for (y = 0; y < E.screenrows; y++) {
     abAppend(ab, "~", 1);
 
+    abAppend(ab, "\x1b[K", 3);
     if (y < E.screenrows - 1) {
       abAppend(ab, "\r\n", 2);
     }
```

*コンパイルはされるが、目に見える効果はない*

`K` コマンド ([行内消去](http://vt100.net/docs/vt100-ug/chapter3.html#EL)) は、現在の行の一部を消去します。その引数は `J` コマンドの引数に似ています。`2` は行全体を消去し、`1` はカーソルの左側の行を消去し、`0` はカーソルの右側の行を消去します。`0` はデフォルトの引数であり、それが目的なので、引数を省略して `<esc>[K` を使用します。

## ようこそメッセージ

そろそろウェルカムメッセージを表示する頃合いかもしれません。画面の下から3分の1くらいのところに、エディター名とバージョン番号を表示してみましょう。

**ステップ 41** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/welcome/kilo.c) / [welcome](https://github.com/snaptoken/kilo-src/tree/welcome))

```diff
 /*** インクルード ***/
 …
 /*** 定義 ***/
 
+#define KILO_VERSION "0.0.1"
+
 #define CTRL_KEY(k) ((k) & 0x1f)
 
 /*** データ ***/
 …
 /*** ターミナル ***/
 …
 /*** 追加バッファ ***/
 …
 /*** 出力 ***/
 
 void editorDrawRows(struct abuf *ab) {
   int y;
   for (y = 0; y < E.screenrows; y++) {
+    if (y == E.screenrows / 3) {
+      char welcome[80];
+      int welcomelen = snprintf(welcome, sizeof(welcome),
+        "Kilo editor -- version %s", KILO_VERSION);
+      if (welcomelen > E.screencols) welcomelen = E.screencols;
+      abAppend(ab, welcome, welcomelen);
+    } else {
+      abAppend(ab, "~", 1);
+    }
 
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

`snprintf()` は `<stdio.h>` から来ています。

`welcome`バッファと`snprintf()`を使用して、`KILO_VERSION`文字列をウェルカムメッセージに挿入します。また、端末が小さすぎてウェルカムメッセージが収まらない場合に備えて、文字列の長さを切り詰めます。

それでは、中央に配置しましょう。

**ステップ 42** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/center/kilo.c) / [center](https://github.com/snaptoken/kilo-src/tree/center))

```diff
 /*** include ***/
 …
 /*** defined ***/
 …
 /*** data ***/
 …
 /*** terminal ***/
 …
 /*** append buffer ***/
 …
 /*** output ***/
 
 void editorDrawRows(struct abuf *ab) {
   int y;
   for (y = 0; y < E.screenrows; y++) {
     if (y == E.screenrows / 3) {
       char welcome[80];
       int welcomelen = snprintf(welcome, sizeof(welcome),
         "Kilo editor -- version %s", KILO_VERSION);
       if (welcomelen > E.screencols) welcomelen = E.screencols;
+      int padding = (E.screencols - welcomelen) / 2;
+      if (padding) {
+        abAppend(ab, "~", 1);
+        padding--;
+      }
+      while (padding--) abAppend(ab, " ", 1);
       abAppend(ab, welcome, welcomelen);
     } else {
       abAppend(ab, "~", 1);
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

文字列を中央揃えにするには、画面の幅を 2 で割り、そこから文字列の長さの半分を引きます。つまり、`E.screencols/2 - welcomelen/2` となり、簡略化すると `(E.screencols - welcomelen) / 2` となります。これにより、画面の左端から文字列の表示を開始する位置がわかります。最初の文字はチルダなので、その位置を空白文字で埋めます。

## カーソルを移動する

それでは、入力に焦点を当てましょう。ユーザーがカーソルを移動できるようにしたいと考えています。最初のステップは、カーソルの `x` 座標と `y` 座標をグローバルエディタの状態として追跡することです。

**ステップ 43** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/cx-cy/kilo.c) / [cx-cy](https://github.com/snaptoken/kilo-src/tree/cx-cy))

```diff
 /*** インクルード ***/
 …
 /*** 定義 ***/
 …
 /*** データ ***/
 
 struct editorConfig {
+  int cx, cy;
   int screenrows;
   int screencols;
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
+  E.cx = 0;
+  E.cy = 0;
+
   if (getWindowSize(&E.screenrows, &E.screencols) == -1) die("getWindowSize");
 }
 
 int main() {
   …
   }
```

*コンパイルはされるが、目に見える効果はない*

`E.cx`はカーソルの水平座標（列）、`E.cy`は垂直座標（行）を表します。カーソルを画面の左上から開始させたいので、両方とも`0`に初期化します。（C言語ではインデックスが`0`から始まるため、可能な限り0から始まるインデックス値を使用します。）

それでは、`editorRefreshScreen()` にコードを追加して、カーソルを `E.cx` と `E.cy` に格納されている位置に移動させましょう。

**ステップ 44** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/set-cursor-position/kilo.c) / [set-cursor-position](https://github.com/snaptoken/kilo-src/tree/set-cursor-position))

```diff
 /*** include ***/
 …
 /*** defined ***/
 …
 /*** data ***/
 …
 /*** terminal ***/
 …
 /*** append buffer ***/
 …
 /*** output ***/
 
 void editorDrawRows(struct abuf *ab) {
   …
 }
 
 void editorRefreshScreen() {
   struct abuf ab = ABUF_INIT;
 
   abAppend(&ab, "\x1b[?25l", 6);
   abAppend(&ab, "\x1b[H", 3);
 
   editorDrawRows(&ab);
 
+  char buf[32];
+  snprintf(buf, sizeof(buf), "\x1b[%d;%dH", E.cy + 1, E.cx + 1);
+  abAppend(&ab, buf, strlen(buf));
+
   abAppend(&ab, "\x1b[?25h", 6);
 
   write(STDOUT_FILENO, ab.b, ab.len);
   abFree(&ab);
 }
 
 /*** 入力***/
 …
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

`strlen()` は `<string.h>` から来ています。

以前の `H` コマンドを、カーソルを移動させたい正確な位置を指定できる引数付きの `H` コマンドに変更しました。（上記の差分を見ると古い `H` コマンドが削除されていないか確認しておいてください。）

端末で使用される0から始まるインデックス値から1から始まるインデックス値に変換するために、`E.cy`と`E.cx`に`1`を加算します。

この時点で、`E.cx`を`10`などの値で初期化したり、`E.cx++`をメインループに挿入したりして、これまでのコードが意図どおりに動作していることを確認してみてください。

次に、ユーザーがwasdキーを使ってカーソルを移動できるようにします。（これらのキーを矢印キーとして使用することに慣れていない場合は、wが上矢印、sが下矢印、aが左矢印、dが右矢印です。）

**ステップ 45** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/move-cursor/kilo.c) / [move-cursor](https://github.com/snaptoken/kilo-src/tree/move-cursor))

```diff
 /*** include ***/
 …
 /*** defined ***/
 …
 /*** data ***/
 …
 /*** terminal ***/
 …
 /*** append buffer ***/
 …
 /*** output ***/
 …
 /*** input ***/
 
+void editorMoveCursor(char key) {
+  switch (key) {
+    case 'a':
+      E.cx--;
+      break;
+    case 'd':
+      E.cx++;
+      break;
+    case 'w':
+      E.cy--;
+      break;
+    case 's':
+      E.cy++;
+      break;
+  }
+}
+
 void editorProcessKeypress() {
   char c = editorReadKey();
 
   switch (c) {
     case CTRL_KEY('q'):
       write(STDOUT_FILENO, "\x1b[2J", 4);
       write(STDOUT_FILENO, "\x1b[H", 3);
       exit(0);
       break;
+
+    case 'w':
+    case 's':
+    case 'a':
+    case 'd':
+      editorMoveCursor(c);
+      break;
   }
 }
 
 /*** init ***/
```

*コンパイル*

これで、それらのキーを使ってカーソルを移動できるはずです。

## 矢印キー

カーソルを移動するためのキー操作のマッピング方法がわかったので、wasd キーを矢印キーに置き換えてみましょう。前の章で、矢印キーを押すと複数のバイトがプログラムに入力として送信されることを確認しました。これらのバイトは、`'\x1b'`、`'['` で始まり、押された 4 つの矢印キーのどれかに応じて `'A'`、`'B'`、`'C'`、または `'D'` が続くエスケープシーケンスの形式です。`editorReadKey()` を変更して、この形式のエスケープシーケンスを単一のキー操作として読み取るようにしましょう。

**ステップ 46** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/detect-arrow-keys/kilo.c) / [detect-arrow-keys](https://github.com/snaptoken/kilo-src/tree/detect-arrow-keys))

```diff
 /*** 含まれる ***/
 …
 /*** 定義する ***/
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
 
 char editorReadKey() {
   int nread;
   char c;
   while ((nread = read(STDIN_FILENO, &c, 1)) != 1) {
     if (nread == -1 && errno != EAGAIN) die("read");
   }
+
+  if (c == '\x1b') {
+    char seq[3];
+
+    if (read(STDIN_FILENO, &seq[0], 1) != 1) return '\x1b';
+    if (read(STDIN_FILENO, &seq[1], 1) != 1) return '\x1b';
+
+    if (seq[0] == '[') {
+      switch (seq[1]) {
+        case 'A': return 'w';
+        case 'B': return 's';
+        case 'C': return 'd';
+        case 'D': return 'a';
+      }
+    }
+
+    return '\x1b';
+  } else {
+    return c;
+  }
 }
 
 int getCursorPosition(int *rows, int *cols) {
   …
 }
 
 int getWindowSize(int *rows, int *cols) {
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

エスケープ文字を読み取った場合、すぐに `seq` バッファにさらに 2 バイトを読み込みます。これらの読み込みのいずれかがタイムアウト (0.1 秒後) した場合、ユーザーがEscape キーを押したと判断し、その値を返します。そうでない場合は、エスケープ シーケンスが矢印キーのエスケープ シーケンスかどうかを確認します。矢印キーのエスケープ シーケンスであれば、とりあえず対応する wasd 文字を返します。認識できないエスケープ シーケンスであれば、エスケープ文字を返します。

将来的に、より長いエスケープシーケンスを処理する予定があるため、`seq`バッファを3バイトの長さにしています。

基本的に、矢印キーをwasdキーにエイリアス設定しました。これにより矢印キーはすぐに動作するようになりますが、wasdキーは依然として`editorMoveCursor()`関数にマッピングされたままです。私たちが望むのは、`editorReadKey()`が各矢印キーに対して特別な値を返すことで、特定の矢印キーが押されたことを識別できるようにすることです。

まず、wasd 文字の各インスタンスを定数 `ARROW_UP`、`ARROW_LEFT`、`ARROW_DOWN`、および `ARROW_RIGHT` に置き換えることから始めましょう。

**ステップ 47** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/arrow-keys-enum/kilo.c) / [arrow-keys-enum](https://github.com/snaptoken/kilo-src/tree/arrow-keys-enum))

```diff
 /*** 含まれる ***/
 …
 /*** 定義 ***/
 
 #define KILO_VERSION "0.0.1"
 
 #define CTRL_KEY(k) ((k) & 0x1f)
 
+enum editorKey {
+  ARROW_LEFT = 'a',
+  ARROW_RIGHT = 'd',
+  ARROW_UP = 'w',
+  ARROW_DOWN = 's'
+};
+
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
 
 char editorReadKey() {
   int nread;
   char c;
   while ((nread = read(STDIN_FILENO, &c, 1)) != 1) {
     if (nread == -1 && errno != EAGAIN) die("read");
   }
 
   if (c == '\x1b') {
     char seq[3];
 
     if (read(STDIN_FILENO, &seq[0], 1) != 1) return '\x1b';
     if (read(STDIN_FILENO, &seq[1], 1) != 1) return '\x1b';
 
     if (seq[0] == '[') {
       switch (seq[1]) {
+        case 'A': return ARROW_UP;
+        case 'B': return ARROW_DOWN;
+        case 'C': return ARROW_RIGHT;
+        case 'D': return ARROW_LEFT;
       }
     }
 
     return '\x1b';
   } else {
     return c;
   }
 }
 
 int getCursorPosition(int *rows, int *cols) {
   …
 }
 
 int getWindowSize(int *rows, int *cols) {
   …
   }
 …
 /*** バッファを追加 ***/
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 
 void editorMoveCursor(char key) {
   switch (key) {
+    case ARROW_LEFT:
       E.cx--;
       break;
+    case ARROW_RIGHT:
       E.cx++;
       break;
+    case ARROW_UP:
       E.cy--;
       break;
+    case ARROW_DOWN:
       E.cy++;
       break;
   }
 }
 
 void editorProcessKeypress() {
   char c = editorReadKey();
 
   switch (c) {
     case CTRL_KEY('q'):
       write(STDOUT_FILENO, "\x1b[2J", 4);
       write(STDOUT_FILENO, "\x1b[H", 3);
       exit(0);
       break;
 
+    case ARROW_UP:
+    case ARROW_DOWN:
+    case ARROW_LEFT:
+    case ARROW_RIGHT:
       editorMoveCursor(c);
       break;
   }
 }
 
 /*** 初期化 ***/
```

*コンパイルはされるが、目に見える効果はない*

次に、`editorKey`列挙型で、wasdキーと競合しない矢印キーの表現を選択する必要があります。通常のキー入力と競合しないように、`char`型の範囲外の大きな整数値を割り当てます。また、キー入力を格納するすべての変数を`char`型ではなく`int`型に変更する必要があります。

**ステップ 48** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/arrow-keys-int/kilo.c) / [arrow-keys-int](https://github.com/snaptoken/kilo-src/tree/arrow-keys-int))

```diff
 /*** 含まれるもの ***/
 …
 /*** 定義 ***/
 
 #define KILO_VERSION "0.0.1"
 
 #define CTRL_KEY(k) ((k) & 0x1f)
 
 enum editorKey {
+  ARROW_LEFT = 1000,
+  ARROW_RIGHT,
+  ARROW_UP,
+  ARROW_DOWN
 };
 
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
 
+int editorReadKey() {
   int nread;
   char c;
   while ((nread = read(STDIN_FILENO, &c, 1)) != 1) {
     if (nread == -1 && errno != EAGAIN) die("read");
   }
 
   if (c == '\x1b') {
     char seq[3];
 
     if (read(STDIN_FILENO, &seq[0], 1) != 1) return '\x1b';
     if (read(STDIN_FILENO, &seq[1], 1) != 1) return '\x1b';
 
     if (seq[0] == '[') {
       switch (seq[1]) {
         case 'A': return ARROW_UP;
         case 'B': return ARROW_DOWN;
         case 'C': return ARROW_RIGHT;
         case 'D': return ARROW_LEFT;
       }
     }
 
     return '\x1b';
   } else {
     return c;
   }
 }
 
 int getCursorPosition(int *rows, int *cols) {
   …
 }
 
 int getWindowSize(int *rows, int *cols) {
   …
   }
 …
 /*** バッファを追加 ***/
 …
 /*** 出力 ***/
 …
 /*** 入力 ***/
 
+void editorMoveCursor(int key) {
   switch (key) {
     case ARROW_LEFT:
       E.cx--;
       break;
     case ARROW_RIGHT:
       E.cx++;
       break;
     case ARROW_UP:
       E.cy--;
       break;
     case ARROW_DOWN:
       E.cy++;
       break;
   }
 }
 
 void editorProcessKeypress() {
+  int c = editorReadKey();
 
   switch (c) {
     case CTRL_KEY('q'):
       write(STDOUT_FILENO, "\x1b[2J", 4);
       write(STDOUT_FILENO, "\x1b[H", 3);
       exit(0);
       break;
 
     case ARROW_UP:
     case ARROW_DOWN:
     case ARROW_LEFT:
     case ARROW_RIGHT:
       editorMoveCursor(c);
       break;
   }
 }
 
 /*** 初期化 ***/
```

*コンパイル*

列挙型の最初の定数を`1000`に設定することで、残りの定数には`1001`、`1002`、`1003`といったように、増加する値が割り当てられます。

これで矢印キーの処理コードは完了です。ここで、プログラムの実行中にエスケープシーケンスを手動で入力してみるのも面白いでしょう。Escapeキー、\[キー、Shift+Cを順番に素早く押してみてください。すると、キー入力が右矢印キーとして解釈されるかもしれません。かなり素早く操作する必要があるので、操作を容易にするために、一時的に`enableRawMode()`の`VTIME`値を調整することをお勧めします。（Ctrl+\[を押すことはEscapeキーを押すことと同じであるという点も覚えておくと便利です。これは、Ctrl+Mを押すことがEnterキーを押すことと同じであるのと同じ理由です。Ctrlキーは、同時に入力した文字の6ビット目と7ビット目をクリアします。）

## カーソルが画面外に移動しないようにする

現在、`E.cx`と`E.cy`の値が負の値になったり、画面の右端や下端を超えたりする可能性があります。そこで、`editorMoveCursor()`関数内で境界チェックを行うことで、これを防ぎましょう。

**ステップ 49** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/off-screen/kilo.c) / [off-screen](https://github.com/snaptoken/kilo-src/tree/off-screen))

```diff
 /*** include ***/
 …
 /*** defined ***/
 …
 /*** data ***/
 …
 /*** terminal ***/
 …
 /*** append buffer ***/
 …
 /*** output ***/
 …
 /*** input ***/
 
 void editorMoveCursor(int key) {
   switch (key) {
     case ARROW_LEFT:
+      if (E.cx != 0) {
+        E.cx--;
+      }
       break;
     case ARROW_RIGHT:
+      if (E.cx != E.screencols - 1) {
+        E.cx++;
+      }
       break;
     case ARROW_UP:
+      if (E.cy != 0) {
+        E.cy--;
+      }
       break;
     case ARROW_DOWN:
+      if (E.cy != E.screenrows - 1) {
+        E.cy++;
+      }
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

## ページアップキーとページダウンキー

低レベルの端末コードを完成させるには、矢印キーのようにエスケープシーケンスを使用する特殊なキー入力をいくつか検出する必要があります。まずは Page Up キーと Page Down キーから始めましょう。Page Up は `<esc>[5~` として送信され、Page Down は `<esc>[6~` として送信されます。

**ステップ 50** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/detect-page-up-down/kilo.c) / [detect-page-up-down](https://github.com/snaptoken/kilo-src/tree/detect-page-up-down))

```diff
 /*** 含まれる ***/
 …
 /*** 定義 ***/
 
 #define KILO_VERSION "0.0.1"
 
 #define CTRL_KEY(k) ((k) & 0x1f)
 
 enum editorKey {
   ARROW_LEFT = 1000,
   ARROW_RIGHT,
   ARROW_UP,
+  ARROW_DOWN,
+  PAGE_UP,
+  PAGE_DOWN
 };
 
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
   int nread;
   char c;
   while ((nread = read(STDIN_FILENO, &c, 1)) != 1) {
     if (nread == -1 && errno != EAGAIN) die("read");
   }
 
   if (c == '\x1b') {
     char seq[3];
 
     if (read(STDIN_FILENO, &seq[0], 1) != 1) return '\x1b';
     if (read(STDIN_FILENO, &seq[1], 1) != 1) return '\x1b';
 
     if (seq[0] == '[') {
+      if (seq[1] >= '0' && seq[1] <= '9') {
+        if (read(STDIN_FILENO, &seq[2], 1) != 1) return '\x1b';
+        if (seq[2] == '~') {
+          switch (seq[1]) {
+            case '5': return PAGE_UP;
+            case '6': return PAGE_DOWN;
+          }
+        }
+      } else {
+        switch (seq[1]) {
+          case 'A': return ARROW_UP;
+          case 'B': return ARROW_DOWN;
+          case 'C': return ARROW_RIGHT;
+          case 'D': return ARROW_LEFT;
+        }
       }
     }
 
     return '\x1b';
   } else {
     return c;
   }
 }
 
 int getCursorPosition(int *rows, int *cols) {
   …
 }
 
 int getWindowSize(int *rows, int *cols) {
   …
   }
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

これで、`seq` が 3 バイトを格納できるように宣言した理由がわかったでしょう。`[` の後のバイトが数字の場合、`~` であることを期待して別のバイトを読み取ります。次に、数字のバイトが `5` か `6` かをテストします。

Page UpキーとPage Downキーに何らかの動作をさせてみましょう。とりあえず、カーソルを画面の上部または下部に移動させるようにします。

**ステップ 51** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/page-up-down-simple/kilo.c) / [page-up-down-simple](https://github.com/snaptoken/kilo-src/tree/page-up-down-simple))

```diff
 /*** include ***/
 …
 /*** defined ***/
 …
 /*** data ***/
 …
 /*** terminal ***/
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
     case CTRL_KEY('q'):
       write(STDOUT_FILENO, "\x1b[2J", 4);
       write(STDOUT_FILENO, "\x1b[H", 3);
       exit(0);
       break;
 
+    case PAGE_UP:
+    case PAGE_DOWN:
+      {
+        int times = E.screenrows;
+        while (times--)
+          editorMoveCursor(c == PAGE_UP ? ARROW_UP : ARROW_DOWN);
+      }
+      break;
+
     case ARROW_UP:
     case ARROW_DOWN:
     case ARROW_LEFT:
     case ARROW_RIGHT:
       editorMoveCursor(c);
       break;
   }
 }
 
 /*** 初期化 ***/
```

*コンパイル*

`times`変数を宣言できるように、その括弧で囲んだコードブロックを作成します。（`switch`文の中では変数を直接宣言することはできません。）ユーザーが画面の一番上または一番下に移動するために、↑キーまたは↓キーを十分な回数押す動作をシミュレートします。このようにPage UpとPage Downを実装することで、後でスクロールを実装する際に非常に楽になります。

Fnキーを搭載したノートパソコンを使用している場合は、Fn+↑とFn+↓を押すことで、Page UpキーとPage Downキーを押した動作をシミュレートできる場合があります。

## Home キーと End キー

次に、Home キーと End キーを実装しましょう。前のキーと同様に、これらのキーもエスケープシーケンスを送信します。前のキーとは異なり、これらのキーによって送信されるエスケープシーケンスは、OS またはターミナルエミュレータによって異なる場合があります。Home キーは、`<esc>[1~`、`<esc>[7~`、`<esc>[H`、または `<esc>OH` (文字 O の後に H が続く) として送信される可能性があります。同様に、End キーは、`<esc>[4~`、`<esc>[8~`、`<esc>[F`、または `<esc>OF` (文字 O の後に F が続く) として送信される可能性があります。これらのすべてのケースを処理しましょう。

**ステップ 52** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/detect-home-end/kilo.c) / [detect-home-end](https://github.com/snaptoken/kilo-src/tree/detect-home-end))

```diff
 /*** 含まれる ***/
 …
 /*** 定義 ***/
 
 #define KILO_VERSION "0.0.1"
 
 #define CTRL_KEY(k) ((k) & 0x1f)
 
 enum editorKey {
   ARROW_LEFT = 1000,
   ARROW_RIGHT,
   ARROW_UP,
   ARROW_DOWN,
+  HOME_KEY,
+  END_KEY,
   PAGE_UP,
   PAGE_DOWN
 };
 
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
   int nread;
   char c;
   while ((nread = read(STDIN_FILENO, &c, 1)) != 1) {
     if (nread == -1 && errno != EAGAIN) die("read");
   }
 
   if (c == '\x1b') {
     char seq[3];
 
     if (read(STDIN_FILENO, &seq[0], 1) != 1) return '\x1b';
     if (read(STDIN_FILENO, &seq[1], 1) != 1) return '\x1b';
 
     if (seq[0] == '[') {
       if (seq[1] >= '0' && seq[1] <= '9') {
         if (read(STDIN_FILENO, &seq[2], 1) != 1) return '\x1b';
         if (seq[2] == '~') {
           switch (seq[1]) {
+            case '1': return HOME_KEY;
+            case '4': return END_KEY;
             case '5': return PAGE_UP;
             case '6': return PAGE_DOWN;
+            case '7': return HOME_KEY;
+            case '8': return END_KEY;
           }
         }
       } else {
         switch (seq[1]) {
           case 'A': return ARROW_UP;
           case 'B': return ARROW_DOWN;
           case 'C': return ARROW_RIGHT;
           case 'D': return ARROW_LEFT;
+          case 'H': return HOME_KEY;
+          case 'F': return END_KEY;
         }
       }
+    } else if (seq[0] == 'O') {
+      switch (seq[1]) {
+        case 'H': return HOME_KEY;
+        case 'F': return END_KEY;
+      }
     }
 
     return '\x1b';
   } else {
     return c;
   }
 }
 
 int getCursorPosition(int *rows, int *cols) {
   …
 }
 
 int getWindowSize(int *rows, int *cols) {
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

それでは、HomeキーとEndキーに何らかの動作をさせてみましょう。とりあえず、カーソルを画面の左端または右端に移動させるようにします。

**ステップ 53** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/home-end-simple/kilo.c) / [home-end-simple](https://github.com/snaptoken/kilo-src/tree/home-end-simple))

```diff
 /*** include ***/
 …
 /*** defined ***/
 …
 /*** data ***/
 …
 /*** terminal ***/
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
     case CTRL_KEY('q'):
       write(STDOUT_FILENO, "\x1b[2J", 4);
       write(STDOUT_FILENO, "\x1b[H", 3);
       exit(0);
       break;
 
+    case HOME_KEY:
+      E.cx = 0;
+      break;
+
+    case END_KEY:
+      E.cx = E.screencols - 1;
+      break;
+
     case PAGE_UP:
     case PAGE_DOWN:
       {
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
 
 /*** 初期化 ***/
```

*コンパイル*

Fnキーを搭載したノートパソコンを使用している場合は、Fn+←とFn+→を押すことで、HomeキーとEndキーを押した動作をシミュレートできる場合があります。

## 削除キー

最後に、Deleteキーが押されたことを検出しましょう。Deleteキーはエスケープシーケンス`<esc>[3~`を送信するだけなので、switch文に簡単に追加できます。今のところ、このキーには何も動作させません。

**ステップ 54** ([kilo.c](https://github.com/snaptoken/kilo-src/blob/detect-delete-key/kilo.c) / [detect-delete-key](https://github.com/snaptoken/kilo-src/tree/detect-delete-key))

```diff
 /*** 含まれる ***/
 …
 /*** 定義 ***/
 
 #define KILO_VERSION "0.0.1"
 
 #define CTRL_KEY(k) ((k) & 0x1f)
 
 enum editorKey {
   ARROW_LEFT = 1000,
   ARROW_RIGHT,
   ARROW_UP,
   ARROW_DOWN,
+  DEL_KEY,
   HOME_KEY,
   END_KEY,
   PAGE_UP,
   PAGE_DOWN
 };
 
 /*** データ ***/
 …
 /*** 端末 ***/
 
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
   int nread;
   char c;
   while ((nread = read(STDIN_FILENO, &c, 1)) != 1) {
     if (nread == -1 && errno != EAGAIN) die("read");
   }
 
   if (c == '\x1b') {
     char seq[3];
 
     if (read(STDIN_FILENO, &seq[0], 1) != 1) return '\x1b';
     if (read(STDIN_FILENO, &seq[1], 1) != 1) return '\x1b';
 
     if (seq[0] == '[') {
       if (seq[1] >= '0' && seq[1] <= '9') {
         if (read(STDIN_FILENO, &seq[2], 1) != 1) return '\x1b';
         if (seq[2] == '~') {
           switch (seq[1]) {
             case '1': return HOME_KEY;
+            case '3': return DEL_KEY;
             case '4': return END_KEY;
             case '5': return PAGE_UP;
             case '6': return PAGE_DOWN;
             case '7': return HOME_KEY;
             case '8': return END_KEY;
           }
         }
       } else {
         switch (seq[1]) {
           case 'A': return ARROW_UP;
           case 'B': return ARROW_DOWN;
           case 'C': return ARROW_RIGHT;
           case 'D': return ARROW_LEFT;
           case 'H': return HOME_KEY;
           case 'F': return END_KEY;
         }
       }
     } else if (seq[0] == 'O') {
       switch (seq[1]) {
         case 'H': return HOME_KEY;
         case 'F': return END_KEY;
       }
     }
 
     return '\x1b';
   } else {
     return c;
   }
 }
 
 int getCursorPosition(int *rows, int *cols) {
   …
 }
 
 int getWindowSize(int *rows, int *cols) {
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

Fnキーを搭載したノートパソコンを使用している場合は、FnキーとBackspaceキーを同時に押すことで、Deleteキーを押したのと同じ動作をシミュレートできる場合があります。

次の章では、縦横スクロールとステータスバーを備えたテキストファイルを表示するプログラムを作成します。

[ページの先頭](03_raw_input_and_output.md)
