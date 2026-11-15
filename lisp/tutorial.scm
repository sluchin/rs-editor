;;; tutorial.scm --- the learn-by-doing tutorial -*- scheme -*-

;; Emacs の tutorial.el に当たるもの (`C-h t`). 起動のときに読む (core/src/scheme/loadup.rs).
;; 本文は, Emacs の TUTORIAL ではなく, このエディタのキーに合わせて書いたもの.

(define tutorial--text-en
  "RSEDITOR TUTORIAL

This tutorial teaches the keys you use every day.  Read it with the cursor,
and try each key on the text.  C-v means: hold Control and press v.
M-v means: hold Meta (Alt) and press v, or press ESC and then v.

Type C-v to move to the next screen.  Type M-v to go back.

>> Try C-v and M-v a few times.

* QUITTING

C-g stops the command you are typing.  Use it whenever you are lost.
C-x C-c leaves the editor.

* MOVING THE CURSOR

        Previous line, C-p
                  :
                  :
 Back, C-b .... Current cursor position .... Forward, C-f
                  :
                  :
        Next line, C-n

>> Move the cursor to the line in the middle with C-n and C-p.
   Then press C-l.  It puts the cursor line in the center of the screen.

 Word                 M-f (forward)      M-b (back)
 Beginning of line    C-a
 End of line          C-e
 Sentence             M-a (beginning)    M-e (end)
 Beginning of buffer  M-<
 End of buffer        M->

>> Try each key on this line.

* REPEATING A COMMAND

C-u and a number before a command repeats the command that many times.
M-5 C-f moves 5 characters forward.

>> Type C-u 8 * to insert ******** into the buffer.

* DELETING AND INSERTING

 DEL      Delete the character before the cursor
 C-d      Delete the character after the cursor
 M-d      Kill the word after the cursor
 C-k      Kill from the cursor to the end of the line

Killed text is not lost.  C-y yanks (pastes) the last killed text.
M-y after C-y replaces it with an older killed text.

>> Move to the next line, press C-k, then C-y.

* MARKING A REGION

C-SPC sets the mark.  The text between the mark and the cursor is the region.
 C-w   Kill the region          M-w   Copy the region
 C-y   Paste it again           C-x h Mark the whole buffer

>> Set the mark here with C-SPC, move down a few lines, and press M-w.
   Then press C-y to paste the copy.

* UNDO

C-/ (or C-x u) undoes the last change.  Press it again to undo more.
C-? redoes.  Undoing a kill brings the text back.

>> Press C-k to kill this line, then C-/ to get it back.

* SEARCHING

C-s searches forward as you type.  Press C-s again for the next match, and
C-r to search backward.  RET stops at the match.  C-g goes back to where you began.

>> Type C-s and the word  region , and press C-s again.

M-% replaces text.  It asks for the old and the new text, then asks at each match:
y replaces, n skips, ! replaces all the rest, q quits.

* FILES

 C-x C-f   Visit a file (type the name; TAB completes it)
 C-x C-s   Save the buffer
 C-x s     Save all the buffers
 C-x C-w   Save with another name

* BUFFERS AND WINDOWS

 C-x b     Switch to another buffer
 C-x C-b   List the buffers
 C-x k     Kill a buffer
 C-x 2     Split the window into two, one above the other
 C-x 3     Split the window into two, side by side
 C-x o     Move to the other window
 C-x 1     Keep only this window
 C-x 0     Delete this window

>> Try C-x 2, then C-x o, then C-x 0.

* COMMANDS BY NAME

M-x runs a command by its name.  Type a part of the name and press TAB.
M-: evaluates a Scheme expression.  C-x C-e evaluates the expression before the cursor.

>> Try M-: and (+ 1 2) RET.

* GETTING HELP

 C-h k   Describe a key
 C-h f   Describe a function
 C-h v   Describe a variable
 C-h m   Describe the current mode
 C-h a   Search the commands for a word
 C-h b   List all the key bindings
 C-h ?   List the help commands

* THE END

You now know the basic keys.  Many more commands exist.  See the README and
the key list (docs/keybindings.md) of rs-editor, and use C-h a to look for a command.

This buffer is not a file.  You can edit it freely; closing it with C-x k throws it away.
Start the tutorial again with C-h t.
")

(define tutorial--text-ja
  "RSEDITOR チュートリアル

このチュートリアルは、毎日使うキーを教えます。カーソルで読み進めながら、
本文に、それぞれのキーを試してください。C-v は、Control を押しながら v を押すことです。
M-v は、Meta (Alt) を押しながら v を押すか、ESC を押してから v を押すことです。

C-v で次の画面に進みます。M-v で戻ります。

>> C-v と M-v を、何回か試してください。

* 終わる

C-g は、入力しているコマンドを止めます。迷ったら、いつでも使います。
C-x C-c は、エディタを終わります。

* カーソルを動かす

        前の行、C-p
                  :
                  :
 後ろ、C-b .... 今のカーソルの位置 .... 前、C-f
                  :
                  :
        次の行、C-n

>> C-n と C-p で、真ん中の行に動いてください。
   次に C-l を押します。カーソルの行が、画面の中央に来ます。

 単語                 M-f (前へ)         M-b (後ろへ)
 行の先頭             C-a
 行の末尾             C-e
 文                   M-a (先頭)         M-e (末尾)
 バッファの先頭       M-<
 バッファの末尾       M->

>> この行で、それぞれのキーを試してください。

* コマンドを繰り返す

コマンドの前に C-u と数字を打つと、その回数だけ繰り返します。
M-5 C-f は、5 文字前へ動きます。

>> C-u 8 * と打つと、******** が入ります。

* 消す、入れる

 DEL      カーソルの前の 1 文字を消す
 C-d      カーソルの後ろの 1 文字を消す
 M-d      カーソルの後ろの単語を kill する
 C-k      カーソルから行末までを kill する

kill したテキストは、なくなりません。C-y で、最後に kill したテキストを貼ります (yank)。
C-y のあとの M-y は、もっと前に kill したテキストに替えます。

>> 次の行に動いて、C-k を押し、次に C-y を押してください。

* リージョンを選ぶ

C-SPC は、マークを置きます。マークとカーソルの間のテキストが、リージョンです。
 C-w   リージョンを kill する      M-w   リージョンをコピーする
 C-y   貼る                        C-x h バッファ全体を選ぶ

>> ここで C-SPC を押してマークを置き、数行下に動いて M-w を押してください。
   次に C-y で、コピーを貼ります。

* 元に戻す

C-/ (または C-x u) は、最後の変更を元に戻します。続けて押すと、もっと戻ります。
C-? は、やり直します。kill を元に戻すと、テキストが戻ります。

>> C-k でこの行を kill し、C-/ で戻してください。

* 探す

C-s は、打ちながら前へ探します。もう一度 C-s で次の一致、C-r で後ろへ探します。
RET で、一致した所で止まります。C-g は、始めた所に戻ります。

>> C-s を押して、リージョン と打ち、もう一度 C-s を押してください。

M-% は、置き換えます。古いテキストと新しいテキストを聞き、一致のたびに聞きます。
y は置き換え、n は飛ばし、! は残りを全て置き換え、q はやめます。

* ファイル

 C-x C-f   ファイルを開く (名前を打つ。TAB で補完する)
 C-x C-s   バッファを保存する
 C-x s     全てのバッファを保存する
 C-x C-w   別の名前で保存する

* バッファとウィンドウ

 C-x b     別のバッファに切り替える
 C-x C-b   バッファの一覧を出す
 C-x k     バッファを閉じる
 C-x 2     ウィンドウを上下に 2 つに分ける
 C-x 3     ウィンドウを左右に 2 つに分ける
 C-x o     もう一方のウィンドウに移る
 C-x 1     このウィンドウだけにする
 C-x 0     このウィンドウを閉じる

>> C-x 2、C-x o、C-x 0 を、順に試してください。

* 名前でコマンドを動かす

M-x は、名前でコマンドを動かします。名前の一部を打って、TAB を押します。
M-: は、Scheme の式を評価します。C-x C-e は、カーソルの前の式を評価します。

>> M-: を押して、(+ 1 2) RET と打ってください。

* ヘルプ

 C-h k   キーの説明
 C-h f   関数の説明
 C-h v   変数の説明
 C-h m   今のモードの説明
 C-h a   単語でコマンドを探す
 C-h b   キーの割り当ての一覧
 C-h ?   ヘルプのコマンドの一覧

* おわりに

基本のキーは、以上です。コマンドは、ほかにも、たくさんあります。rs-editor の README と、
キーの一覧 (docs/keybindings.md) を見てください。コマンドを探すには、C-h a を使います。

このバッファは、ファイルではありません。自由に書き換えてよく、C-x k で閉じると、捨てられます。
もう一度始めるには、C-h t を押します。
")

(define tutorial--languages '("English" "Japanese"))

(define (tutorial--read-language)
  "Read the language of the tutorial in the minibuffer; return \"en\" or \"ja\"."
  (if (equal? (completing-read "Language: " tutorial--languages #f #t) "Japanese") "ja" "en"))

(define (tutorial--text language)
  "Return the text of the tutorial in LANGUAGE (\"ja\", or English for any other)."
  (if (equal? language "ja") tutorial--text-ja tutorial--text-en))

(define (tutorial--buffer-name language)
  "Return the name of the tutorial buffer for LANGUAGE."
  (if (equal? language "ja") "TUTORIAL.ja" "TUTORIAL"))

(define-command (help-with-tutorial . args)
  "Select the learn-by-doing tutorial.
The tutorial is in the language of `help-language' if it has one (\"ja\"), otherwise English.
With a prefix argument, ask which language to use.
If the tutorial buffer is already there, select it so that you can go on."
  (let* ((ask (if (null? args) (current-prefix-arg) (car args)))
         (language (if ask (tutorial--read-language) help-language))
         (name (tutorial--buffer-name language)))
    (if (get-buffer name)
        (switch-to-buffer name)
        (begin
          (switch-to-buffer name)
          (insert (tutorial--text language))
          (goto-char (point-min))
          (set-buffer-modified-p #f)))
    #f))

(provide 'tutorial)

;;; tutorial.scm ends here
