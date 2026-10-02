# 6.6.5 文字列

> **原文**: [Guile Reference Manual - Strings](https://www.gnu.org/software/guile/manual/html_node/Strings.html)
>
> このドキュメントは GNU Free Documentation License の下で公開されている原文の翻訳です。

文字列は、固定長の文字の並びです。文字列はコンストラクタ手続きを呼び出すことで作成できますが、REPL や Scheme のソースファイルの中でリテラルとして入力することもできます。

文字列は、自分がいくつの文字で構成されているかという情報を常に携えているため、C のように特別な文字列の終端文字はありません。つまり、Scheme の文字列には、「`#\nul`」文字「`\0`」を含め、任意の文字を含めることができます。

文字列を効率的に使うには、Guile がそれらをどのように実装しているかを少し知っておく必要があります。Guile では、文字列はヘッドと、文字が格納されている実際のメモリの2つの部分で構成されます。文字列（またはその部分文字列）がコピーされるとき、作成されるのは新しいヘッドだけで、メモリは通常コピーされません。2つのヘッドは最初は同じメモリを指しています。

これら2つの文字列の一方が `string-set!` などで変更されると、それぞれの文字列が独自のメモリを持ち、一方を変更しても誤って他方まで変更されないように、共通のメモリがコピーされます。したがって、Guile の文字列は「コピーオンライト（copy on write）」です。メモリの実際のコピーは、一方の文字列に書き込みが行われるまで遅延されます。

この実装により、関係する文字列に変更が行われないという一般的な場合に、`substring` のような関数が非常に効率的になります。

文字列がすぐに変更されることが分かっている場合は、`substring` の代わりに `substring/copy` を使うことができます。この関数は作成時にすぐにコピーを行います。これは、特にマルチスレッドのプログラムにおいて、より効率的です。また、`substring/copy` は、短い部分文字列が、そうでなければ再利用できたはずの非常に大きな元の文字列のメモリを保持し続けてしまうという問題を避けることができます。

コピーをまったく避けて、一方の文字列の変更が他方にも現れるようにしたい場合は、`substring/shared` を使うことができます。この手続きによって作成された文字列は、部分文字列と元の文字列が互いに変更を共有するため、**変更共有部分文字列**（mutation sharing substring）と呼ばれます。

変更を防ぎたい場合は、`substring/read-only` を使ってください。

Guile は SRFI-13 のすべての手続きと、それ以外のいくつかの手続きを提供しています。

- 文字列の読み取り構文
- 文字列の述語
- 文字列のコンストラクタ
- リスト/文字列の変換
- 文字列の選択
- 文字列の変更
- 文字列の比較
- 文字列の検索
- アルファベットの大文字小文字の変換
- 文字列の反転と連結
- 写像、畳み込み、展開
- その他の文字列操作
- 文字列をバイトとして表現する
- C との相互変換
- 文字列の内部

## 6.6.5.1 文字列の読み取り構文

文字列の読み取り構文は、二重引用符（`"`）で囲まれた任意の長さの文字の並びです。

バックスラッシュはエスケープ文字であり、以下の特殊文字を挿入するために使用できます。`\"` と `\\` は R5RS 標準、`\|` は R7RS 標準、次の7つは R6RS 標準――C の構文に従っていることに注意――であり、残りの4つは Guile の拡張です。

`\\`
: バックスラッシュ文字。

`\"`
: 二重引用符文字（エスケープされていない `"` はそれ以外の場合、文字列の終わりです）。

`\|`
: 縦棒文字。

`\a`
: ベル文字（ASCII 7）。

`\f`
: 改ページ文字（ASCII 12）。

`\n`
: 改行文字（ASCII 10）。

`\r`
: 復帰文字（ASCII 13）。

`\t`
: タブ文字（ASCII 9）。

`\v`
: 垂直タブ文字（ASCII 11）。

`\b`
: バックスペース文字（ASCII 8）。

`\0`
: NUL 文字（ASCII 0）。

`\(`
: 開き括弧。これは、Emacs の Lisp モードを混乱させないように、複数行の文字列の行頭で使うことを意図しています。

`\` の後に改行（ASCII 10）
: 何もありません。この方法では、`\` が行の最後の文字である場合、文字列は改行なしで次の行の最初の文字から続きます。

  リーダオプション `hungry-eol-escapes` が有効になっている場合（デフォルトではそうではありません）、次の行の先頭の空白は捨てられます。

  ```scheme
  "foo\
    bar"
  ⇒ "foo  bar"
  (read-enable 'hungry-eol-escapes)
  "foo\
    bar"
  ⇒ "foobar"
  ```

`\xHH`
: 2桁の16進数で与えられる文字コード。たとえば ASCII の DEL（127）には `\x7f`。

`\uHHHH`
: 4桁の16進数で与えられる文字コード。たとえばマクロン付きの大文字 A（U+0100）には `Ā`。

`\UHHHHHH`
: 6桁の16進数で与えられる文字コード。たとえば `\U010402`。

以下は文字列リテラルの例です。

```scheme
"foo"
"bar plonk"
"Hello World"
"\"Hi\", he said."
```

3つのエスケープシーケンス `\xHH`、`\uHHHH`、`\UHHHHHH` は、以前のバージョンの Guile 向けに書かれたコードとの互換性を損なわないように選ばれました。R6RS の仕様は、16進エスケープについて別の互換性のない構文を提案しています。`\xHHHH;` ――文字コードの後に1桁から8桁の16進数を続け、セミコロンで終えるものです。代わりにこのエスケープ形式が望ましい場合は、リーダオプション `r6rs-hex-escapes` で有効にできます。

```scheme
(read-enable 'r6rs-hex-escapes)
```

リーダオプションの詳細については、「Scheme コードの読み込み」を参照してください。

## 6.6.5.2 文字列の述語

以下の手続きは、与えられた文字列が何らかの指定された性質を満たすかどうかを確認するために使用できます。

**Scheme 手続き: `string? obj`**<br>**C 関数: `scm_string_p (obj)`**
: `obj` が文字列であれば `#t` を、そうでなければ `#f` を返します。

**C 関数: `int scm_is_string (SCM obj)`**
: `obj` が文字列であれば 1 を、そうでなければ 0 を返します。

**Scheme 手続き: `string-null? str`**<br>**C 関数: `scm_string_null_p (str)`**
: `str` の長さがゼロであれば `#t` を、そうでなければ `#f` を返します。

  ```scheme
  (string-null? "")  ⇒ #t
  y                  ⇒ "foo"
  (string-null? y)   ⇒ #f
  ```

**Scheme 手続き: `string-any char_pred s [start [end]]`**<br>**C 関数: `scm_string_any (char_pred, s, start, end)`**
: 文字列 `s` のいずれかの文字に対して `char_pred` が真であるかどうかを確認します。

  `char_pred` には、それと等しい文字があるかどうかを確認するための文字、その集合に含まれる文字があるかどうかを確認するための文字集合（「文字集合」を参照）、あるいは呼び出す述語手続きを指定できます。

  手続きの場合、`start` から `end` までの文字に対して `(char_pred c)` の呼び出しが順に行われます。`char_pred` が真（つまり `#f` 以外）を返すと、`string-any` は停止し、その戻り値が `string-any` の戻り値になります。最後の文字（つまり `end`-1 の位置）に対する呼び出しは、その時点に達した場合、末尾呼び出しになります。

  `s` に文字がない場合（つまり `start` が `end` と等しい場合）、戻り値は `#f` です。

**Scheme 手続き: `string-every char_pred s [start [end]]`**<br>**C 関数: `scm_string_every (char_pred, s, start, end)`**
: 文字列 `s` のすべての文字に対して `char_pred` が真であるかどうかを確認します。

  `char_pred` には、すべての文字がそれと等しいかどうかを確認するための文字、すべての文字がその集合に含まれるかどうかを確認するための文字集合（「文字集合」を参照）、あるいは呼び出す述語手続きを指定できます。

  手続きの場合、`start` から `end` までの文字に対して `(char_pred c)` の呼び出しが順に行われます。`char_pred` が `#f` を返すと、`string-every` は停止して `#f` を返します。最後の文字（つまり `end`-1 の位置）に対する呼び出しは、その時点に達した場合、末尾呼び出しになり、その呼び出しからの戻り値が `string-every` の戻り値になります。

  `s` に文字がない場合（つまり `start` が `end` と等しい場合）、戻り値は `#t` です。

## 6.6.5.3 文字列のコンストラクタ

文字列のコンストラクタ手続きは、新しい文字列オブジェクトを作成し、場合によっては何らかの指定された文字データで初期化します。既存の文字列から文字列を作成する方法については、「文字列の選択」も参照してください。

**Scheme 手続き: `string char…`**
: 与えられた文字の引数から作られた、新しく割り当てられた文字列を返します。

  ```scheme
  (string #\x #\y #\z) ⇒ "xyz"
  (string)             ⇒ ""
  ```

**Scheme 手続き: `list->string lst`**<br>**C 関数: `scm_string (lst)`**
: 文字のリストから作られた、新しく割り当てられた文字列を返します。

  ```scheme
  (list->string '(#\a #\b #\c)) ⇒ "abc"
  ```

**Scheme 手続き: `reverse-list->string lst`**<br>**C 関数: `scm_reverse_list_to_string (lst)`**
: 文字のリストから逆順に作られた、新しく割り当てられた文字列を返します。

  ```scheme
  (reverse-list->string '(#\a #\B #\c)) ⇒ "cBa"
  ```

**Scheme 手続き: `make-string k [chr]`**<br>**C 関数: `scm_make_string (k, chr)`**
: 長さ `k` の新しく割り当てられた文字列を返します。`chr` が与えられた場合、文字列のすべての要素は `chr` に初期化されます。そうでなければ、文字列の内容は未規定です。

**C 関数: `SCM scm_c_make_string (size_t len, SCM chr)`**
: `scm_make_string` と同様ですが、長さを `size_t` として期待します。

**Scheme 手続き: `string-tabulate proc len`**<br>**C 関数: `scm_string_tabulate (proc, len)`**
: `proc` は整数から文字への手続きです。各インデックスに `proc` を適用して対応する文字列の要素を生成することで、サイズ `len` の文字列を構築します。`proc` がインデックスに適用される順序は規定されていません。

**Scheme 手続き: `string-join ls [delimiter [grammar]]`**<br>**C 関数: `scm_string_join (ls, delimiter, grammar)`**
: 文字列のリスト `ls` の中の文字列を、文字列 `delimiter` を `ls` の要素間の区切りとして使って連結します。`delimiter` のデフォルトは「` `」、つまり `ls` の文字列は間に空白文字を入れて連結されます。`grammar` は、区切りが文字列の間にどのように置かれるかを指定するシンボルであり、デフォルトはシンボル `infix` です。

  `infix`
  : リストの要素の間に区切りを挿入します。空の文字列は空のリストを生成します。

  `strict-infix`
  : `infix` と同様ですが、空のリストが与えられるとエラーを発生させます。

  `suffix`
  : リストのすべての要素の後に区切りを挿入します。

  `prefix`
  : リストの各要素の前に区切りを挿入します。

## 6.6.5.4 リスト/文字列の変換

文字列を処理するときは、まず手続き `string->list` を使ってリスト表現に変換し、結果のリストを操作してから文字列に戻すと便利なことがよくあります。これらの手続きは、同様の作業に役立ちます。

**Scheme 手続き: `string->list str [start [end]]`**<br>**C 関数: `scm_substring_to_list (str, start, end)`**<br>**C 関数: `scm_string_to_list (str)`**
: 文字列 `str` を文字のリストに変換します。

**Scheme 手続き: `string-split str char_pred`**<br>**C 関数: `scm_string_split (str, char_pred)`**
: 文字列 `str` を、次の文字が現れるところで区切られた部分文字列のリストに分割します。

  - `char_pred` が文字の場合は、それと等しい文字
  - `char_pred` が手続きの場合は、述語 `char_pred` を満たす文字
  - `char_pred` が文字集合の場合は、集合 `char_pred` に含まれる文字

  区切り文字の間の空の部分文字列は、結果のリストの中の空の文字列になることに注意してください。

  ```scheme
  (string-split "root:x:0:0:root:/root:/bin/bash" #\:)
  ⇒
  ("root" "x" "0" "0" "root" "/root" "/bin/bash")

  (string-split "::" #\:)
  ⇒
  ("" "" "")

  (string-split "" #\:)
  ⇒
  ("")
  ```

## 6.6.5.5 文字列の選択

これらの手続きで文字列の一部を取り出すことができます。`string-ref` は個々の文字を取り出し、`substring` はより長い文字列から部分文字列を取り出すために使用できます。

**Scheme 手続き: `string-length string`**<br>**C 関数: `scm_string_length (string)`**
: `string` の文字数を返します。

**C 関数: `size_t scm_c_string_length (SCM str)`**
: `str` の文字数を `size_t` として返します。

**Scheme 手続き: `string-ref str k`**<br>**C 関数: `scm_string_ref (str, k)`**
: ゼロ起点のインデックスを使って、`str` の文字 `k` を返します。`k` は `str` の有効なインデックスでなければなりません。

**C 関数: `SCM scm_c_string_ref (SCM str, size_t k)`**
: ゼロ起点のインデックスを使って、`str` の文字 `k` を返します。`k` は `str` の有効なインデックスでなければなりません。

**Scheme 手続き: `string-copy str [start [end]]`**<br>**C 関数: `scm_substring_copy (str, start, end)`**<br>**C 関数: `scm_string_copy (str)`**
: 与えられた文字列 `str` のコピーを返します。

  返される文字列は最初は `str` と記憶領域を共有しますが、2つの文字列のどちらかが変更されるとすぐにコピーされます。

**Scheme 手続き: `substring str start [end]`**<br>**C 関数: `scm_substring (str, start, end)`**
: `str` の文字から、インデックス `start`（これを含む）から始まりインデックス `end`（これを含まない）で終わる新しい文字列を作って返します。`str` は文字列でなければならず、`start` と `end` は次を満たす正確な整数でなければなりません。

  0 <= `start` <= `end` <= `(string-length str)`。

  返される文字列は最初は `str` と記憶領域を共有しますが、2つの文字列のどちらかが変更されるとすぐにコピーされます。

**Scheme 手続き: `substring/shared str start [end]`**<br>**C 関数: `scm_substring_shared (str, start, end)`**
: `substring` と同様ですが、文字列は変更されても記憶領域を共有し続けます。したがって、`str` への変更は新しい文字列に現れ、その逆も同様です。

**Scheme 手続き: `substring/copy str start [end]`**<br>**C 関数: `scm_substring_copy (str, start, end)`**
: `substring` と同様ですが、新しい文字列の記憶領域はすぐにコピーされます。

**Scheme 手続き: `substring/read-only str start [end]`**<br>**C 関数: `scm_substring_read_only (str, start, end)`**
: `substring` と同様ですが、結果の文字列は変更できません。

**C 関数: `SCM scm_c_substring (SCM str, size_t start, size_t end)`**<br>**C 関数: `SCM scm_c_substring_shared (SCM str, size_t start, size_t end)`**<br>**C 関数: `SCM scm_c_substring_copy (SCM str, size_t start, size_t end)`**<br>**C 関数: `SCM scm_c_substring_read_only (SCM str, size_t start, size_t end)`**
: `scm_substring` などと同様ですが、境界は `size_t` として与えられます。

**Scheme 手続き: `string-take s n`**<br>**C 関数: `scm_string_take (s, n)`**
: `s` の最初の `n` 文字を返します。

**Scheme 手続き: `string-drop s n`**<br>**C 関数: `scm_string_drop (s, n)`**
: `s` の最初の `n` 文字以外のすべてを返します。

**Scheme 手続き: `string-take-right s n`**<br>**C 関数: `scm_string_take_right (s, n)`**
: `s` の最後の `n` 文字を返します。

**Scheme 手続き: `string-drop-right s n`**<br>**C 関数: `scm_string_drop_right (s, n)`**
: `s` の最後の `n` 文字以外のすべてを返します。

**Scheme 手続き: `string-pad s len [chr [start [end]]]`**<br>**Scheme 手続き: `string-pad-right s len [chr [start [end]]]`**<br>**C 関数: `scm_string_pad (s, len, chr, start, end)`**<br>**C 関数: `scm_string_pad_right (s, len, chr, start, end)`**
: 文字列 `s` から文字 `start` から `end` までを取り出し、`chr` で埋めるか切り詰めるかして `len` 文字にします。

  `string-pad` は左側で埋めるか切り詰めるので、たとえば次のようになります。

  ```scheme
  (string-pad "x" 3)     ⇒ "  x"
  (string-pad "abcde" 3) ⇒ "cde"
  ```

  `string-pad-right` は右側で埋めるか切り詰めるので、たとえば次のようになります。

  ```scheme
  (string-pad-right "x" 3)     ⇒ "x  "
  (string-pad-right "abcde" 3) ⇒ "abc"
  ```

**Scheme 手続き: `string-trim s [char_pred [start [end]]]`**<br>**Scheme 手続き: `string-trim-right s [char_pred [start [end]]]`**<br>**Scheme 手続き: `string-trim-both s [char_pred [start [end]]]`**<br>**C 関数: `scm_string_trim (s, char_pred, start, end)`**<br>**C 関数: `scm_string_trim_right (s, char_pred, start, end)`**<br>**C 関数: `scm_string_trim_both (s, char_pred, start, end)`**
: `s` の端から `char_pred` の出現を取り除きます。

  `string-trim` は文字列の左側（先頭）から `char_pred` の文字を取り除き、`string-trim-right` は文字列の右側（末尾）からそれらを取り除き、`string-trim-both` は両端から取り除きます。

  `char_pred` には、文字、文字集合、または各文字に対して呼び出す述語手続きを指定できます。`char_pred` が与えられない場合、デフォルトは `char-set:whitespace` による空白です（「標準の文字集合」を参照）。

  ```scheme
  (string-trim " x ")              ⇒ "x "
  (string-trim-right "banana" #\a) ⇒ "banan"
  (string-trim-both ".,xy:;" char-set:punctuation)
                                   ⇒ "xy"
  (string-trim-both "xyzzy" (lambda (c)
                              (or (eqv? c #\x)
                                  (eqv? c #\y))))
                                   ⇒ "zz"
  ```

## 6.6.5.6 文字列の変更

これらの手続きは、文字列をその場で変更するためのものです。これは、操作の結果が新しい文字列ではなく、元の文字列のメモリ表現が変更されることを意味します。

**Scheme 手続き: `string-set! str k chr`**<br>**C 関数: `scm_string_set_x (str, k, chr)`**
: `str` の要素 `k` に `chr` を格納し、未規定の値を返します。`k` は `str` の有効なインデックスでなければなりません。

**C 関数: `void scm_c_string_set_x (SCM str, size_t k, SCM chr)`**
: `scm_string_set_x` と同様ですが、インデックスは `size_t` として与えられます。

**Scheme 手続き: `string-fill! str chr [start [end]]`**<br>**C 関数: `scm_substring_fill_x (str, chr, start, end)`**<br>**C 関数: `scm_string_fill_x (str, chr)`**
: 与えられた `str` のすべての要素に `chr` を格納し、未規定の値を返します。

**Scheme 手続き: `substring-fill! str start end fill`**<br>**C 関数: `scm_substring_fill_x (str, start, end, fill)`**
: `str` の `start` から `end` までのすべての文字を `fill` に変更します。

  ```scheme
  (define y (string-copy "abcdefg"))
  (substring-fill! y 1 3 #\r)
  y
  ⇒ "arrdefg"
  ```

**Scheme 手続き: `substring-move! str1 start1 end1 str2 start2`**<br>**C 関数: `scm_substring_move_x (str1, start1, end1, str2, start2)`**
: `start1` と `end1` で区切られた `str1` の部分文字列を、位置 `start2` から始まる `str2` にコピーします。`str1` と `str2` は同じ文字列でもかまいません。

**Scheme 手続き: `string-copy! target tstart s [start [end]]`**<br>**C 関数: `scm_string_copy_x (target, tstart, s, start, end)`**
: 文字列 `s` のインデックス範囲 [`start`, `end`) の文字の並びを、インデックス `tstart` から始まる文字列 `target` にコピーします。文字は必要に応じて左から右へ、または右から左へコピーされます――`target` と `s` が同じ文字列であっても、コピーは正しく機能することが保証されています。コピー操作が `target` 文字列の終わりを超えてしまう場合はエラーです。

## 6.6.5.7 文字列の比較

この節の手続きは、文字の順序付け述語（「文字」を参照）と似ていますが、文字の並びに対して定義されています。

最初のセットは R5RS で規定されており、名前は `?` で終わります。2番目のセットは SRFI-13 で規定されており、名前は `?` で終わりません。

`-ci` で終わる述語は、文字列を比較するときに文字の大文字と小文字を無視します。現時点では、大文字と小文字を区別しない比較は R5RS の規則を使って行われます。そこでは、単一の文字の大文字形を持つすべての小文字は、比較の前に大文字に変換されます。ロケールに依存した文字列比較については、`(ice-9 i18n)` モジュールを参照してください。

**Scheme 手続き: `string=? s1 s2 s3 …`**
: 辞書式の等価性の述語です。すべての文字列が同じ長さで、同じ位置に同じ文字を含んでいれば `#t` を返し、そうでなければ `#f` を返します。

  手続き `string-ci=?` は大文字と小文字を同じ文字であるかのように扱いますが、`string=?` は大文字と小文字を異なる文字として扱います。

**Scheme 手続き: `string<? s1 s2 s3 …`**
: 辞書式の順序付けの述語です。連続する文字列の引数 str_i と str_i+1 のすべての組について、str_i が辞書式に str_i+1 より小さければ `#t` を返します。

**Scheme 手続き: `string<=? s1 s2 s3 …`**
: 辞書式の順序付けの述語です。連続する文字列の引数 str_i と str_i+1 のすべての組について、str_i が辞書式に str_i+1 以下であれば `#t` を返します。

**Scheme 手続き: `string>? s1 s2 s3 …`**
: 辞書式の順序付けの述語です。連続する文字列の引数 str_i と str_i+1 のすべての組について、str_i が辞書式に str_i+1 より大きければ `#t` を返します。

**Scheme 手続き: `string>=? s1 s2 s3 …`**
: 辞書式の順序付けの述語です。連続する文字列の引数 str_i と str_i+1 のすべての組について、str_i が辞書式に str_i+1 以上であれば `#t` を返します。

**Scheme 手続き: `string-ci=? s1 s2 s3 …`**
: 大文字と小文字を区別しない文字列の等価性の述語です。すべての文字列が同じ長さで、その構成文字が各位置で（大文字と小文字を無視して）一致すれば `#t` を返し、そうでなければ `#f` を返します。

**Scheme 手続き: `string-ci<? s1 s2 s3 …`**
: 大文字と小文字を区別しない辞書式の順序付けの述語です。連続する文字列の引数 str_i と str_i+1 のすべての組について、大文字と小文字にかかわらず str_i が辞書式に str_i+1 より小さければ `#t` を返します。

**Scheme 手続き: `string-ci<=? s1 s2 s3 …`**
: 大文字と小文字を区別しない辞書式の順序付けの述語です。連続する文字列の引数 str_i と str_i+1 のすべての組について、大文字と小文字にかかわらず str_i が辞書式に str_i+1 以下であれば `#t` を返します。

**Scheme 手続き: `string-ci>? s1 s2 s3 …`**
: 大文字と小文字を区別しない辞書式の順序付けの述語です。連続する文字列の引数 str_i と str_i+1 のすべての組について、大文字と小文字にかかわらず str_i が辞書式に str_i+1 より大きければ `#t` を返します。

**Scheme 手続き: `string-ci>=? s1 s2 s3 …`**
: 大文字と小文字を区別しない辞書式の順序付けの述語です。連続する文字列の引数 str_i と str_i+1 のすべての組について、大文字と小文字にかかわらず str_i が辞書式に str_i+1 以上であれば `#t` を返します。

**Scheme 手続き: `string-compare s1 s2 proc_lt proc_eq proc_gt [start1 [end1 [start2 [end2]]]]`**<br>**C 関数: `scm_string_compare (s1, s2, proc_lt, proc_eq, proc_gt, start1, end1, start2, end2)`**
: `s1` が `s2` より小さいか、等しいか、大きいかに応じて、`proc_lt`、`proc_eq`、`proc_gt` を不一致インデックスに適用します。不一致インデックスとは、すべての 0 <= j < i について s1[j] = s2[j] となる最大のインデックス i です――つまり、i は一致しない最初の位置です。

**Scheme 手続き: `string-compare-ci s1 s2 proc_lt proc_eq proc_gt [start1 [end1 [start2 [end2]]]]`**<br>**C 関数: `scm_string_compare_ci (s1, s2, proc_lt, proc_eq, proc_gt, start1, end1, start2, end2)`**
: `s1` が `s2` より小さいか、等しいか、大きいかに応じて、`proc_lt`、`proc_eq`、`proc_gt` を不一致インデックスに適用します。不一致インデックスとは、すべての 0 <= j < i について s1[j] = s2[j] となる最大のインデックス i です――つまり、i は小文字化された文字が一致しない最初の位置です。

**Scheme 手続き: `string= s1 s2 [start1 [end1 [start2 [end2]]]]`**<br>**C 関数: `scm_string_eq (s1, s2, start1, end1, start2, end2)`**
: `s1` と `s2` が等しくなければ `#f` を、そうでなければ真の値を返します。

**Scheme 手続き: `string<> s1 s2 [start1 [end1 [start2 [end2]]]]`**<br>**C 関数: `scm_string_neq (s1, s2, start1, end1, start2, end2)`**
: `s1` と `s2` が等しければ `#f` を、そうでなければ真の値を返します。

**Scheme 手続き: `string< s1 s2 [start1 [end1 [start2 [end2]]]]`**<br>**C 関数: `scm_string_lt (s1, s2, start1, end1, start2, end2)`**
: `s1` が `s2` 以上であれば `#f` を、そうでなければ真の値を返します。

**Scheme 手続き: `string> s1 s2 [start1 [end1 [start2 [end2]]]]`**<br>**C 関数: `scm_string_gt (s1, s2, start1, end1, start2, end2)`**
: `s1` が `s2` 以下であれば `#f` を、そうでなければ真の値を返します。

**Scheme 手続き: `string<= s1 s2 [start1 [end1 [start2 [end2]]]]`**<br>**C 関数: `scm_string_le (s1, s2, start1, end1, start2, end2)`**
: `s1` が `s2` より大きければ `#f` を、そうでなければ真の値を返します。

**Scheme 手続き: `string>= s1 s2 [start1 [end1 [start2 [end2]]]]`**<br>**C 関数: `scm_string_ge (s1, s2, start1, end1, start2, end2)`**
: `s1` が `s2` より小さければ `#f` を、そうでなければ真の値を返します。

**Scheme 手続き: `string-ci= s1 s2 [start1 [end1 [start2 [end2]]]]`**<br>**C 関数: `scm_string_ci_eq (s1, s2, start1, end1, start2, end2)`**
: `s1` と `s2` が等しくなければ `#f` を、そうでなければ真の値を返します。文字の比較は大文字と小文字を区別せずに行われます。

**Scheme 手続き: `string-ci<> s1 s2 [start1 [end1 [start2 [end2]]]]`**<br>**C 関数: `scm_string_ci_neq (s1, s2, start1, end1, start2, end2)`**
: `s1` と `s2` が等しければ `#f` を、そうでなければ真の値を返します。文字の比較は大文字と小文字を区別せずに行われます。

**Scheme 手続き: `string-ci< s1 s2 [start1 [end1 [start2 [end2]]]]`**<br>**C 関数: `scm_string_ci_lt (s1, s2, start1, end1, start2, end2)`**
: `s1` が `s2` 以上であれば `#f` を、そうでなければ真の値を返します。文字の比較は大文字と小文字を区別せずに行われます。

**Scheme 手続き: `string-ci> s1 s2 [start1 [end1 [start2 [end2]]]]`**<br>**C 関数: `scm_string_ci_gt (s1, s2, start1, end1, start2, end2)`**
: `s1` が `s2` 以下であれば `#f` を、そうでなければ真の値を返します。文字の比較は大文字と小文字を区別せずに行われます。

**Scheme 手続き: `string-ci<= s1 s2 [start1 [end1 [start2 [end2]]]]`**<br>**C 関数: `scm_string_ci_le (s1, s2, start1, end1, start2, end2)`**
: `s1` が `s2` より大きければ `#f` を、そうでなければ真の値を返します。文字の比較は大文字と小文字を区別せずに行われます。

**Scheme 手続き: `string-ci>= s1 s2 [start1 [end1 [start2 [end2]]]]`**<br>**C 関数: `scm_string_ci_ge (s1, s2, start1, end1, start2, end2)`**
: `s1` が `s2` より小さければ `#f` を、そうでなければ真の値を返します。文字の比較は大文字と小文字を区別せずに行われます。

**Scheme 手続き: `string-hash s [bound [start [end]]]`**<br>**C 関数: `scm_substring_hash (s, bound, start, end)`**
: `s` のハッシュ値を計算します。オプション引数 `bound` は、ハッシュ関数の範囲を指定する非負の正確な整数です。正の値は戻り値を範囲 [0,`bound`) に制限します。

**Scheme 手続き: `string-hash-ci s [bound [start [end]]]`**<br>**C 関数: `scm_substring_hash_ci (s, bound, start, end)`**
: `s` のハッシュ値を計算します。オプション引数 `bound` は、ハッシュ関数の範囲を指定する非負の正確な整数です。正の値は戻り値を範囲 [0,`bound`) に制限します。

抽象的な Unicode 文字の同じ見た目は、複数の Unicode 文字の並びによって得られることがあるため、上で説明した大文字と小文字を区別しない文字列比較関数でさえ、同じ文字の異なる表現を含む文字列が与えられると `#f` を返すことがあります。たとえば、Unicode 文字「LATIN SMALL LETTER S WITH DOT BELOW AND DOT ABOVE」は、単一の文字（U+1E69）で表現することも、文字「LATIN SMALL LETTER S」（U+0073）の後に結合記号「COMBINING DOT BELOW」（U+0323）と「COMBINING DOT ABOVE」（U+0307）を続けて表現することもできます。

このため、比較する文字列がすべての文字について相互に一貫した表現を使っていることを保証することが、しばしば望ましくなります。Unicode 標準は、文字列の内容を正規化する2つの方法を定義しています。分解（decomposition）は、合成文字を Unicode 標準で定義された順序を持つ構成文字の集合に分解するものであり、合成（composition）はその逆を行うものです。

分解の操作は2つあります。「正準分解（canonical decomposition）」は元の文字と同じ見た目を共有する文字の並びを生成し、「互換分解（compatibility decomposition）」は見た目が元の文字と異なる場合があるものの、同じ抽象的な文字を表す文字の並びを生成します。

これらの操作は、次の正規化形式の集合にまとめられています。

NFD
: 文字はその正準形に分解されます。

NFKD
: 文字はその互換形に分解されます。

NFC
: 文字はその正準形に分解され、それから合成されます。

NFKC
: 文字はその互換形に分解され、それから合成されます。

以下の関数は、引数を上で説明した形式のいずれかにします。

**Scheme 手続き: `string-normalize-nfd s`**<br>**C 関数: `scm_string_normalize_nfd (s)`**
: `s` の NFD 正規化形式を返します。

**Scheme 手続き: `string-normalize-nfkd s`**<br>**C 関数: `scm_string_normalize_nfkd (s)`**
: `s` の NFKD 正規化形式を返します。

**Scheme 手続き: `string-normalize-nfc s`**<br>**C 関数: `scm_string_normalize_nfc (s)`**
: `s` の NFC 正規化形式を返します。

**Scheme 手続き: `string-normalize-nfkc s`**<br>**C 関数: `scm_string_normalize_nfkc (s)`**
: `s` の NFKC 正規化形式を返します。

## 6.6.5.8 文字列の検索

**Scheme 手続き: `string-index s char_pred [start [end]]`**<br>**C 関数: `scm_string_index (s, char_pred, start, end)`**
: 文字列 `s` を左から右へ検索し、次の条件を満たす文字が最初に現れるインデックスを返します。

  - `char_pred` が文字の場合は、それと等しい
  - `char_pred` が手続きの場合は、述語 `char_pred` を満たす
  - `char_pred` が文字集合の場合は、集合 `char_pred` に含まれる

  一致するものが見つからなければ `#f` を返します。

**Scheme 手続き: `string-rindex s char_pred [start [end]]`**<br>**C 関数: `scm_string_rindex (s, char_pred, start, end)`**
: 文字列 `s` を右から左へ検索し、次の条件を満たす文字が最後に現れるインデックスを返します。

  - `char_pred` が文字の場合は、それと等しい
  - `char_pred` が手続きの場合は、述語 `char_pred` を満たす
  - `char_pred` が文字集合の場合は、その集合に含まれる

  一致するものが見つからなければ `#f` を返します。

**Scheme 手続き: `string-prefix-length s1 s2 [start1 [end1 [start2 [end2]]]]`**<br>**C 関数: `scm_string_prefix_length (s1, s2, start1, end1, start2, end2)`**
: 2つの文字列の最長の共通接頭辞の長さを返します。

**Scheme 手続き: `string-prefix-length-ci s1 s2 [start1 [end1 [start2 [end2]]]]`**<br>**C 関数: `scm_string_prefix_length_ci (s1, s2, start1, end1, start2, end2)`**
: 文字の大文字と小文字を無視して、2つの文字列の最長の共通接頭辞の長さを返します。

**Scheme 手続き: `string-suffix-length s1 s2 [start1 [end1 [start2 [end2]]]]`**<br>**C 関数: `scm_string_suffix_length (s1, s2, start1, end1, start2, end2)`**
: 2つの文字列の最長の共通接尾辞の長さを返します。

**Scheme 手続き: `string-suffix-length-ci s1 s2 [start1 [end1 [start2 [end2]]]]`**<br>**C 関数: `scm_string_suffix_length_ci (s1, s2, start1, end1, start2, end2)`**
: 文字の大文字と小文字を無視して、2つの文字列の最長の共通接尾辞の長さを返します。

**Scheme 手続き: `string-prefix? s1 s2 [start1 [end1 [start2 [end2]]]]`**<br>**C 関数: `scm_string_prefix_p (s1, s2, start1, end1, start2, end2)`**
: `s1` は `s2` の接頭辞か？

**Scheme 手続き: `string-prefix-ci? s1 s2 [start1 [end1 [start2 [end2]]]]`**<br>**C 関数: `scm_string_prefix_ci_p (s1, s2, start1, end1, start2, end2)`**
: 文字の大文字と小文字を無視して、`s1` は `s2` の接頭辞か？

**Scheme 手続き: `string-suffix? s1 s2 [start1 [end1 [start2 [end2]]]]`**<br>**C 関数: `scm_string_suffix_p (s1, s2, start1, end1, start2, end2)`**
: `s1` は `s2` の接尾辞か？

**Scheme 手続き: `string-suffix-ci? s1 s2 [start1 [end1 [start2 [end2]]]]`**<br>**C 関数: `scm_string_suffix_ci_p (s1, s2, start1, end1, start2, end2)`**
: 文字の大文字と小文字を無視して、`s1` は `s2` の接尾辞か？

**Scheme 手続き: `string-index-right s char_pred [start [end]]`**<br>**C 関数: `scm_string_index_right (s, char_pred, start, end)`**
: 文字列 `s` を右から左へ検索し、次の条件を満たす文字が最後に現れるインデックスを返します。

  - `char_pred` が文字の場合は、それと等しい
  - `char_pred` が手続きの場合は、述語 `char_pred` を満たす
  - `char_pred` が文字集合の場合は、その集合に含まれる

  一致するものが見つからなければ `#f` を返します。

**Scheme 手続き: `string-skip s char_pred [start [end]]`**<br>**C 関数: `scm_string_skip (s, char_pred, start, end)`**
: 文字列 `s` を左から右へ検索し、次の条件を満たす文字が最初に現れるインデックスを返します。

  - `char_pred` が文字の場合は、それと等しくない
  - `char_pred` が手続きの場合は、述語 `char_pred` を満たさない
  - `char_pred` が文字集合の場合は、その集合に含まれない

**Scheme 手続き: `string-skip-right s char_pred [start [end]]`**<br>**C 関数: `scm_string_skip_right (s, char_pred, start, end)`**
: 文字列 `s` を右から左へ検索し、次の条件を満たす文字が最後に現れるインデックスを返します。

  - `char_pred` が文字の場合は、それと等しくない
  - `char_pred` が手続きの場合は、述語 `char_pred` を満たさない
  - `char_pred` が文字集合の場合は、その集合に含まれない

**Scheme 手続き: `string-count s char_pred [start [end]]`**<br>**C 関数: `scm_string_count (s, char_pred, start, end)`**
: 文字列 `s` の中で、次の条件を満たす文字の数を返します。

  - `char_pred` が文字の場合は、それと等しい
  - `char_pred` が手続きの場合は、述語 `char_pred` を満たす
  - `char_pred` が文字集合の場合は、集合 `char_pred` に含まれる

**Scheme 手続き: `string-contains s1 s2 [start1 [end1 [start2 [end2]]]]`**<br>**C 関数: `scm_string_contains (s1, s2, start1, end1, start2, end2)`**
: 文字列 `s1` は文字列 `s2` を含むか？ `s2` が部分文字列として現れる `s1` の中のインデックスを返し、なければ偽を返します。オプションの `start`/`end` インデックスは、操作を指示された部分文字列に制限します。

**Scheme 手続き: `string-contains-ci s1 s2 [start1 [end1 [start2 [end2]]]]`**<br>**C 関数: `scm_string_contains_ci (s1, s2, start1, end1, start2, end2)`**
: 文字列 `s1` は文字列 `s2` を含むか？ `s2` が部分文字列として現れる `s1` の中のインデックスを返し、なければ偽を返します。オプションの `start`/`end` インデックスは、操作を指示された部分文字列に制限します。文字の比較は大文字と小文字を区別せずに行われます。

## 6.6.5.9 アルファベットの大文字小文字の変換

これらは、文字列をそれぞれ大文字または小文字の同等物に変換したり、文字列の先頭を大文字にしたりするための手続きです。

これらは Unicode 文字に対する基本的な大文字小文字の変換規則を使います。特別な言語や文脈の規則は考慮されません。結果の文字列は、入力の文字列と同じ長さであることが保証されています。

ロケールに依存した大文字小文字の変換については、`(ice-9 i18n)` モジュールを参照してください。

**Scheme 手続き: `string-upcase str [start [end]]`**<br>**C 関数: `scm_substring_upcase (str, start, end)`**<br>**C 関数: `scm_string_upcase (str)`**
: `str` のすべての文字を大文字にします。

**Scheme 手続き: `string-upcase! str [start [end]]`**<br>**C 関数: `scm_substring_upcase_x (str, start, end)`**<br>**C 関数: `scm_string_upcase_x (str)`**
: `str` のすべての文字を破壊的に大文字にします。

  ```scheme
  (string-upcase! y)
  ⇒ "ARRDEFG"
  y
  ⇒ "ARRDEFG"
  ```

**Scheme 手続き: `string-downcase str [start [end]]`**<br>**C 関数: `scm_substring_downcase (str, start, end)`**<br>**C 関数: `scm_string_downcase (str)`**
: `str` のすべての文字を小文字にします。

**Scheme 手続き: `string-downcase! str [start [end]]`**<br>**C 関数: `scm_substring_downcase_x (str, start, end)`**<br>**C 関数: `scm_string_downcase_x (str)`**
: `str` のすべての文字を破壊的に小文字にします。

  ```scheme
  y
  ⇒ "ARRDEFG"
  (string-downcase! y)
  ⇒ "arrdefg"
  y
  ⇒ "arrdefg"
  ```

**Scheme 手続き: `string-capitalize str`**<br>**C 関数: `scm_string_capitalize (str)`**
: `str` の文字を持ち、すべての単語の最初の文字が大文字にされた、新しく割り当てられた文字列を返します。

**Scheme 手続き: `string-capitalize! str`**<br>**C 関数: `scm_string_capitalize_x (str)`**
: `str` のすべての単語の最初の文字を破壊的に大文字にし、`str` を返します。

  ```scheme
  y                      ⇒ "hello world"
  (string-capitalize! y) ⇒ "Hello World"
  y                      ⇒ "Hello World"
  ```

**Scheme 手続き: `string-titlecase str [start [end]]`**<br>**C 関数: `scm_string_titlecase (str, start, end)`**
: `str` の単語のすべての最初の文字をタイトルケースにします。

**Scheme 手続き: `string-titlecase! str [start [end]]`**<br>**C 関数: `scm_string_titlecase_x (str, start, end)`**
: `str` の単語のすべての最初の文字を破壊的にタイトルケースにします。

## 6.6.5.10 文字列の反転と連結

**Scheme 手続き: `string-reverse str [start [end]]`**<br>**C 関数: `scm_string_reverse (str, start, end)`**
: 文字列 `str` を反転します。オプション引数 `start` と `end` は、操作対象の `str` の領域を区切ります。

**Scheme 手続き: `string-reverse! str [start [end]]`**<br>**C 関数: `scm_string_reverse_x (str, start, end)`**
: 文字列 `str` をその場で反転します。オプション引数 `start` と `end` は、操作対象の `str` の領域を区切ります。戻り値は未規定です。

**Scheme 手続き: `string-append arg …`**<br>**C 関数: `scm_string_append (args)`**
: 与えられた文字列 `arg ...` を連結したものを文字とする、新しく割り当てられた文字列を返します。

  ```scheme
  (let ((h "hello "))
    (string-append h "world"))
  ⇒ "hello world"
  ```

**Scheme 手続き: `string-append/shared arg …`**<br>**C 関数: `scm_string_append_shared (args)`**
: `string-append` と同様ですが、結果は引数の文字列とメモリを共有する可能性があります。

**Scheme 手続き: `string-concatenate ls`**<br>**C 関数: `scm_string_concatenate (ls)`**
: `ls` の要素（文字列でなければならない）を連結して単一の文字列にします。新しく割り当てられた文字列を返すことが保証されています。

**Scheme 手続き: `string-concatenate-reverse ls [final_string [end]]`**<br>**C 関数: `scm_string_concatenate_reverse (ls, final_string, end)`**
: オプション引数がない場合、この手続きは次と同等です。

  ```scheme
  (string-concatenate (reverse ls))
  ```

  オプション引数 `final_string` が指定された場合、リストの反転と文字列の連結の操作を行う前に、それが `ls` の先頭に cons されます。`end` が与えられた場合、`final_string` のインデックス `end` までの文字だけが使われます。

  新しく割り当てられた文字列を返すことが保証されています。

**Scheme 手続き: `string-concatenate/shared ls`**<br>**C 関数: `scm_string_concatenate_shared (ls)`**
: `string-concatenate` と同様ですが、結果はリスト `ls` の中の文字列とメモリを共有する可能性があります。

**Scheme 手続き: `string-concatenate-reverse/shared ls [final_string [end]]`**<br>**C 関数: `scm_string_concatenate_reverse_shared (ls, final_string, end)`**
: `string-concatenate-reverse` と同様ですが、結果は `ls` 引数の中の文字列とメモリを共有する可能性があります。

## 6.6.5.11 写像、畳み込み、展開

**Scheme 手続き: `string-map proc s [start [end]]`**<br>**C 関数: `scm_string_map (proc, s, start, end)`**
: `proc` は文字から文字への手続きであり、`s` の上に写像されます。手続きが文字列の要素に適用される順序は規定されていません。

**Scheme 手続き: `string-map! proc s [start [end]]`**<br>**C 関数: `scm_string_map_x (proc, s, start, end)`**
: `proc` は文字から文字への手続きであり、`s` の上に写像されます。手続きが文字列の要素に適用される順序は規定されていません。文字列 `s` はその場で変更され、戻り値は規定されていません。

**Scheme 手続き: `string-for-each proc s [start [end]]`**<br>**C 関数: `scm_string_for_each (proc, s, start, end)`**
: `proc` が `s` の上に左から右の順に写像されます。戻り値は規定されていません。

**Scheme 手続き: `string-for-each-index proc s [start [end]]`**<br>**C 関数: `scm_string_for_each_index (proc, s, start, end)`**
: `s` の各インデックス `i` について、左から右へ `(proc i)` を呼び出します。

  たとえば、文字を交互に大文字と小文字に変えるには次のようにします。

  ```scheme
  (define str (string-copy "studly"))
  (string-for-each-index
    (lambda (i)
      (string-set! str i
        ((if (even? i) char-upcase char-downcase)
         (string-ref str i))))
    str)
  str ⇒ "StUdLy"
  ```

**Scheme 手続き: `string-fold kons knil s [start [end]]`**<br>**C 関数: `scm_string_fold (kons, knil, s, start, end)`**
: `knil` を終端要素として、`kons` を `s` の文字の上で左から右へ畳み込みます。`kons` は2つの引数を期待しなければなりません。実際の文字と、`kons` の適用の最後の結果です。

**Scheme 手続き: `string-fold-right kons knil s [start [end]]`**<br>**C 関数: `scm_string_fold_right (kons, knil, s, start, end)`**
: `knil` を終端要素として、`kons` を `s` の文字の上で右から左へ畳み込みます。`kons` は2つの引数を期待しなければなりません。実際の文字と、`kons` の適用の最後の結果です。

**Scheme 手続き: `string-unfold p f g seed [base [make_final]]`**<br>**C 関数: `scm_string_unfold (p, f, g, seed, base, make_final)`**
: - `g` は、初期シード `seed` からシード値の列を生成するために使われます: `seed`, (`g` `seed`), (`g`^2 `seed`), (`g`^3 `seed`), …
  - `p` はいつ停止するかを伝えます――これらのシード値の一つに適用されたときに真を返したときです。
  - `f` は各シード値を結果の文字列の対応する文字に写像します。これらの文字は左から右の順に文字列に組み立てられます。
  - `base` は、構築される文字列のオプションの最初/最も左の部分です。デフォルトは空の文字列です。
  - `make_final` は、終端のシード値（`p` が真を返すもの）に適用されて、構築される文字列の最後/最も右の部分を生成します。デフォルトでは何も追加されません。

**Scheme 手続き: `string-unfold-right p f g seed [base [make_final]]`**<br>**C 関数: `scm_string_unfold_right (p, f, g, seed, base, make_final)`**
: - `g` は、初期シード `seed` からシード値の列を生成するために使われます: `seed`, (`g` `seed`), (`g`^2 `seed`), (`g`^3 `seed`), …
  - `p` はいつ停止するかを伝えます――これらのシード値の一つに適用されたときに真を返したときです。
  - `f` は各シード値を結果の文字列の対応する文字に写像します。これらの文字は右から左の順に文字列に組み立てられます。
  - `base` は、構築される文字列のオプションの最初/最も右の部分です。デフォルトは空の文字列です。
  - `make_final` は、終端のシード値（`p` が真を返すもの）に適用されて、構築される文字列の最後/最も左の部分を生成します。デフォルトは `(lambda (x) )` です。

## 6.6.5.12 その他の文字列操作

**Scheme 手続き: `xsubstring s from [to [start [end]]]`**<br>**C 関数: `scm_xsubstring (s, from, to, start, end)`**
: これは、ある文字列の部分文字列の複製によるコピーを実装する、拡張された部分文字列の手続きです。

  `s` は文字列であり、`start` と `end` は `s` の部分文字列を区切るオプション引数で、デフォルトは 0 と `s` の長さです。この部分文字列を、インデックス空間の上下、正と負の両方向に複製します。`xsubstring` は、インデックス `from` から始まり `to` で終わる、この文字列の部分文字列を返します。`to` のデフォルトは `from` + (`end` - `start`) です。

**Scheme 手続き: `string-xcopy! target tstart s sfrom [sto [start [end]]]`**<br>**C 関数: `scm_string_xcopy_x (target, tstart, s, sfrom, sto, start, end)`**
: `xsubstring` とまったく同じですが、取り出されたテキストは、インデックス `tstart` から始まる文字列 `target` に書き込まれます。`(eq? target s)` である場合、またはこれらの引数が記憶領域を共有している場合、操作は定義されていません――文字列をそれ自身の上にコピーすることはできません。

**Scheme 手続き: `string-replace s1 s2 [start1 [end1 [start2 [end2]]]]`**<br>**C 関数: `scm_string_replace (s1, s2, start1, end1, start2, end2)`**
: 文字列 `s1` を返しますが、文字 `start1` … `end1` が `s2` の文字 `start2` … `end2` で置き換えられています。

**Scheme 手続き: `string-tokenize s [token_set [start [end]]]`**<br>**C 関数: `scm_string_tokenize (s, token_set, start, end)`**
: 文字列 `s` を部分文字列のリストに分割します。各部分文字列は、文字集合 `token_set`（デフォルトは `char-set:graphic`）の文字からなる、空でない連続した文字の最大の並びです。`start` または `end` のインデックスが与えられた場合、それらは `string-tokenize` が `s` の指示された部分文字列に対して操作するよう制限します。

**Scheme 手続き: `string-filter char_pred s [start [end]]`**<br>**C 関数: `scm_string_filter (char_pred, s, start, end)`**
: 文字列 `s` をフィルタリングし、`char_pred` を満たす文字だけを残します。

  `char_pred` が手続きの場合は各文字に述語として適用され、文字の場合は等価性がテストされ、文字集合の場合は所属がテストされます。

**Scheme 手続き: `string-delete char_pred s [start [end]]`**<br>**C 関数: `scm_string_delete (char_pred, s, start, end)`**
: `s` から `char_pred` を満たす文字を削除します。

  `char_pred` が手続きの場合は各文字に述語として適用され、文字の場合は等価性がテストされ、文字集合の場合は所属がテストされます。

以下の追加の関数は、モジュール `(ice-9 string-fun)` で利用できます。次のようにして使用できます。

```scheme
(use-modules (ice-9 string-fun))
```

**Scheme 手続き: `string-replace-substring str substring replacement`**
: 文字列 `str` の中の `substring` のすべての出現が `replacement` で置き換えられた新しい文字列を返します。例:

  ```scheme
  (string-replace-substring "a ring of strings" "ring" "rut")
  ⇒ "a rut of struts"
  ```

## 6.6.5.13 文字列をバイトとして表現する

Guile の外の冷たい世界では、すべての文字列が同じように扱われるわけではありません。そこにはバイトしかなく、文字列（文字の並び）をバイナリデータ（バイトの並び）として表現する方法はたくさんあります。

ユーザーとしては、通常これについてあまり考える必要はありません。キーボードで入力すると、システムはコンピュータに設定したロケールに従ってキーストロークをバイトとしてエンコードします。Guile はロケールを使ってそれらのバイトを文字にデコードし直します――願わくば、入力したのと同じ文字に。

ウェブサーバーのような、複数のユーザーがいるシステムを扱う場合は、それほど明確ではありません。ウェブサーバーは、あるユーザーから ISO-8859-1 文字集合でエンコードされたデータのリクエストを受け取り、次に別のユーザーから UTF-8 データのリクエストを受け取るかもしれません。

Guile は、文字列とバイトの並びの間で変換するための `iconv` モジュールを提供しています。Guile が生のバイトの並びをどのように表現するかの詳細については、「バイトベクタ」を参照してください。このモジュールの名前は、同じ名前の一般的な UNIX コマンドに由来しています。

多くの場合、これらの関数を使う代わりに、単にポートから文字列を読み書きするだけで十分であることに注意してください。そのためには、`set-port-encoding!` を使ってポートのエンコーディングを指定します。ポートと文字エンコーディングの詳細については、「ポート」を参照してください。

この節の他の手続きとは異なり、これらの手続きにアクセスする前に `iconv` モジュールを読み込む必要があります。

```scheme
(use-modules (ice-9 iconv))
```

**Scheme 手続き: `string->bytevector string encoding [conversion-strategy]`**
: `string` をバイトの並びとしてエンコードします。

  文字列は、`encoding` 文字列で指定された文字集合でエンコードされます。文字列にそのエンコーディングで表現できない文字がある場合、デフォルトではこの手続きは `encoding-error` を発生させます。他の動作を指定するには `conversion-strategy` 引数を渡してください。

  戻り値はバイトベクタです。バイトベクタの詳細については「バイトベクタ」を参照してください。文字エンコーディングと変換戦略の詳細については「ポート」を参照してください。

**Scheme 手続き: `bytevector->string bytevector encoding [conversion-strategy]`**
: `bytevector` を文字列にデコードします。

  バイトは `encoding` 文字列によって文字集合からデコードされます。バイトが有効なエンコーディングを形成していない場合、デフォルトではこの手続きは `decoding-error` を発生させます。`string->bytevector` と同様に、この動作を変更するにはオプションの `conversion-strategy` 引数を渡してください。文字エンコーディングと変換戦略の詳細については「ポート」を参照してください。

**Scheme 手続き: `call-with-output-encoded-string encoding proc [conversion-strategy]`**
: `call-with-output-string` と同様ですが、文字列を返す代わりに、`encoding` に従った文字列のエンコーディングをバイトベクタとして返します。この手続きは、文字列を集めてから `string->bytevector` で変換するよりも効率的な場合があります。

## 6.6.5.14 C との相互変換

C の文字列から Scheme の文字列を作成したり、Scheme の文字列を C の文字列に変換したりするときは、文字エンコーディングの概念が重要になります。

C では、文字列は単なるバイトの並びであり、文字エンコーディングはこれらのバイトと文字列を構成する実際の文字との関係を記述します。Scheme の文字列については、Scheme では通常、文字列をバイトの並びではなく文字の並びとして扱うため、文字エンコーディングは（ほとんどの場合）問題になりません。

C への変換と C からの変換には、それぞれ独自の課題があります。

C から Scheme に変換するときは、C の文字列のバイトの並びがそのエンコーディングに関して有効であることが重要です。たとえば、ASCII 文字列は 127 より大きいバイトを持つことができません。127 より大きい ASCII バイトは不正な形式と見なされ、Scheme の文字に変換することはできません。

逆の操作でも問題が発生することがあります。すべての文字エンコーディングがすべての可能な Scheme の文字を保持できるわけではありません。たとえば ASCII のような一部のエンコーディングは、すべての可能な文字のうちの小さな部分集合しか記述できません。そのため、C に変換するときは、まず C の文字列で表現できない Scheme の文字をどうするかを決めなければなりません。

Scheme の文字列を C の文字列に変換すると、結果を保持するために新しいメモリが割り当てられることがよくあります。このメモリが最終的に適切に解放されるよう注意しなければなりません。多くの場合、これは適切な dynwind コンテキストの中で `scm_dynwind_free` を使うことで実現できます。「Dynamic Wind」を参照してください。

**C 関数: `SCM scm_from_locale_string (const char *str)`**<br>**C 関数: `SCM scm_from_locale_stringn (const char *str, size_t len)`**
: 現在のロケールの文字エンコーディングで解釈したときに `str` と同じ内容を持つ、新しい Scheme の文字列を作成します。

  `scm_from_locale_string` では、`str` はヌル終端されていなければなりません。

  `scm_from_locale_stringn` では、`len` は `str` の長さをバイト単位で指定し、`str` はヌル終端されている必要はありません。`len` が `(size_t)-1` の場合は、`str` はヌル終端されている必要があり、実際の長さは `strlen` で求められます。

  C の文字列が不正な形式の場合、エラーが発生します。

  現在のロケールが、文字列と文字の定数に使われる実行文字集合のものと一致する保証はないため、これらの関数は C の文字列定数を変換するために使うべきではないことに注意してください。最近のほとんどの C コンパイラはデフォルトで UTF-8 を使うので、C の文字列定数を変換するには `scm_from_utf8_string` を推奨します。

**C 関数: `SCM scm_take_locale_string (char *str)`**<br>**C 関数: `SCM scm_take_locale_stringn (char *str, size_t len)`**
: それぞれ `scm_from_locale_string` および `scm_from_locale_stringn` と同様ですが、最終的に `str` を `free` で解放することも行います。したがって、Scheme の文字列を作成した直後にいずれにせよ `str` を解放する場合に、この関数を使うことができます。特定の場合には、Guile は `str` をその内部表現として直接使うことができます。

**C 関数: `char * scm_to_locale_string (SCM str)`**<br>**C 関数: `char * scm_to_locale_stringn (SCM str, size_t *lenp)`**
: 現在のロケールの文字エンコーディングで `str` と同じ内容を持つ C の文字列を返します。C の文字列は、最終的に `free` で解放しなければなりません。おそらく `scm_dynwind_free` を使ってください。「Dynamic Wind」を参照してください。

  `scm_to_locale_string` では、返される文字列はヌル終端されており、`str` が `#\nul` 文字を含んでいる場合はエラーが通知されます。

  `scm_to_locale_stringn` で `lenp` が `NULL` でない場合、`str` は `#\nul` 文字を含んでいてもよく、返される文字列のバイト単位の長さは `*lenp` に格納されます。この場合、返される文字列はヌル終端されません。`lenp` が `NULL` の場合、`scm_to_locale_stringn` は `scm_to_locale_string` と同様に振る舞います。

  `str` の文字が現在のロケールの文字エンコーディングで表現できない場合は、デフォルトのポート変換戦略が使われます。変換戦略の詳細については「ポート」を参照してください。

  変換戦略が `error` の場合、エラーが発生します。`substitute` の場合、疑問符のような置換文字がその場所に挿入されます。`escape` の場合、16進エスケープがその場所に挿入されます。

**C 関数: `size_t scm_to_locale_stringbuf (SCM str, char *buf, size_t max_len)`**
: `str` を現在のロケールのエンコーディングの C 文字列として、`buf` が指すメモリに置きます。`buf` のバッファには `max_len` バイトの余地があり、`scm_to_local_stringbuf` はそれを超えて格納することはありません。終端の `'\0'` は格納されません。

  `scm_to_locale_stringbuf` の戻り値は、`buf` がそれらを保持するのに十分な大きさであったかどうかにかかわらず、`str` のすべてに必要なバイト数です。したがって、戻り値が `max_len` より大きい場合は、`max_len` バイトしか格納されておらず、おそらくより大きなバッファでもう一度試す必要があります。

ほとんどの状況では、文字列の変換は上の関数のように現在のロケールを使って行うべきです。しかし、ロケールの文字エンコーディング以外の文字エンコーディングから文字列を変換したい場合もあるかもしれません。そのような場合のために、より低レベルの関数 `scm_to_stringn` と `scm_from_stringn` が提供されています。ロケールを適切に使っていれば、これらの関数が必要になることはめったにないはずです。

**C 型: `scm_t_string_failed_conversion_handler`**
: これは3つの値 `SCM_FAILED_CONVERSION_ERROR`、`SCM_FAILED_CONVERSION_QUESTION_MARK`、`SCM_FAILED_CONVERSION_ESCAPE_SEQUENCE` のいずれかを取ることができる列挙型です。これらは、与えられた文字エンコーディングへの、またはそれからの変換ができない文字を扱うための戦略を示すために使われます。`SCM_FAILED_CONVERSION_ERROR` は、一部の文字が変換できない場合に変換がエラーを投げるべきであることを示します。`SCM_FAILED_CONVERSION_QUESTION_MARK` は、変換が変換できない文字を疑問符の文字で置き換えるべきであることを示します。そして、`SCM_FAILED_CONVERSION_ESCAPE_SEQUENCE` は、変換が変換できない文字をエスケープシーケンスで置き換えることを要求します。

  3つの戦略はすべて Scheme の文字列を C に変換するときに適用されますが、C の文字列を Scheme に変換するときに使えるのは `SCM_FAILED_CONVERSION_ERROR` と `SCM_FAILED_CONVERSION_QUESTION_MARK` だけです。

**C 関数: `char *scm_to_stringn (SCM str, size_t *lenp, const char *encoding, scm_t_string_failed_conversion_handler handler)`**
: この関数は、Guile の文字列 `str` から新しく割り当てられた C の文字列を返します。返される文字列のバイト単位の長さは `lenp` に返されます。C の文字列の文字エンコーディングは、ASCII でヌル終端された C の文字列 `encoding` として渡されます。`handler` パラメータは、`encoding` に変換できない文字を扱うための戦略を与えます。

  `lenp` が `NULL` の場合、この関数はヌル終端された C の文字列を返します。文字列がヌル文字を含む場合はエラーを投げます。

  この関数への Scheme のインターフェースは、`ice-9 iconv` モジュールの `string->bytevector` です。「文字列をバイトとして表現する」を参照してください。

**C 関数: `SCM scm_from_stringn (const char *str, size_t len, const char *encoding, scm_t_string_failed_conversion_handler handler)`**
: この関数は、C の文字列 `str` から Scheme の文字列を返します。C の文字列のバイト単位の長さは `len` として入力されます。C の文字列のエンコーディングは、ASCII でヌル終端された C の文字列 `encoding` として渡されます。`handler` パラメータは、変換できない文字を扱うための戦略を示唆します。

  この関数への Scheme のインターフェースは `bytevector->string` です。「文字列をバイトとして表現する」を参照してください。

以下の変換関数は、最もよく使われるエンコーディングのための便宜として提供されています。

**C 関数: `SCM scm_from_latin1_string (const char *str)`**<br>**C 関数: `SCM scm_from_utf8_string (const char *str)`**<br>**C 関数: `SCM scm_from_utf32_string (const scm_t_wchar *str)`**
: ISO-8859-1、UTF-8、または UTF-32 でエンコードされた、ヌル終端された C の文字列 `str` から Scheme の文字列を返します。これらの関数は、ハードコードされた C の文字列定数を Scheme の文字列に変換するために使うべきです。

**C 関数: `SCM scm_from_latin1_stringn (const char *str, size_t len)`**<br>**C 関数: `SCM scm_from_utf8_stringn (const char *str, size_t len)`**<br>**C 関数: `SCM scm_from_utf32_stringn (const scm_t_wchar *str, size_t len)`**
: ISO-8859-1、UTF-8、または UTF-32 でエンコードされた、長さ `len` の C の文字列 `str` から Scheme の文字列を返します。`len` は、`scm_from_latin1_stringn` と `scm_from_utf8_stringn` では `str` が指すバイト数であり、`scm_from_utf32_stringn` の場合は `str` の要素（コードポイント）の数です。

**C 関数: `char *scm_to_latin1_stringn (SCM str, size_t *lenp)`**<br>**C 関数: `char *scm_to_utf8_stringn (SCM str, size_t *lenp)`**<br>**C 関数: `scm_t_wchar *scm_to_utf32_stringn (SCM str, size_t *lenp)`**
: Scheme の文字列 `str` から、新しく割り当てられた、ISO-8859-1、UTF-8、または UTF-32 でエンコードされた C の文字列を返します。`str` を指定されたエンコーディングに変換できない場合はエラーが投げられます。`lenp` が `NULL` の場合、返される C の文字列はヌル終端され、そうしないと C の文字列にヌル文字が含まれてしまう場合はエラーが投げられます。`lenp` が `NULL` でない場合、文字列はヌル終端されず、返される文字列の長さが `lenp` に返されます。返される長さは、`scm_to_latin1_stringn` と `scm_to_utf8_stringn` ではバイト数であり、`scm_to_utf32_stringn` では要素（コードポイント）の数です。

よくあることではありませんが、ポートの実装の詳細を扱っているときに、ポートのエンコーディングと変換戦略に従って文字列をエンコードおよびデコードする必要がある場合があります。その目的のための便利な関数もいくつかあります。

**C 関数: `SCM scm_from_port_string (const char *str, SCM port)`**<br>**C 関数: `SCM scm_from_port_stringn (const char *str, size_t len, SCM port)`**<br>**C 関数: `char* scm_to_port_string (SCM str, SCM port)`**<br>**C 関数: `char* scm_to_port_stringn (SCM str, size_t *lenp, SCM port)`**
: `scm_from_stringn` とその仲間と同様ですが、エンコーディングと変換戦略を与えられたポートオブジェクトから取ります。

## 6.6.5.15 文字列の内部

Guile は、各文字列をメモリ上に、関連する属性の集合とともに Unicode のコードポイントの連続した配列として格納します。文字列のすべてのコードポイントが 0 から 255 まで（両端を含む）の整数の範囲にある場合、コードポイントの配列はコードポイントごとに1バイトとして格納されます。つまり、ISO-8859-1（別名 Latin-1）の文字列として格納されます。文字列のいずれかのコードポイントが 255 より大きい整数値を持つ場合、コードポイントの配列はコードポイントごとに4バイトとして格納されます。つまり、UTF-32 の文字列として格納されます。

コードポイントごとに1バイトの表現と4バイトの表現の間の変換は、必要に応じて自動的に行われます。

文字列の内部表現を設定するための API は提供されていませんが、それを照会するための手続きの組が利用できます。これらはデバッグ用の手続きです。Guile の文字列の内部表現の詳細はリリースごとに変わる可能性があるため、本番のコードでそれらを使うことは推奨されません。

**Scheme 手続き: `string-bytes-per-char str`**<br>**C 関数: `scm_string_bytes_per_char (str)`**
: 文字列 `str` の中で Unicode のコードポイントをエンコードするのに使われるバイト数を返します。結果は1または4です。

**Scheme 手続き: `%string-dump str`**<br>**C 関数: `scm_sys_string_dump (str)`**
: `str` のデバッグ情報を含む連想リストを返します。連想リストには次のエントリがあります。

  `string`
  : 文字列そのもの。

  `start`
  : その stringbuf の中での文字列の開始インデックス

  `length`
  : 文字列の長さ

  `shared`
  : この文字列が部分文字列であれば、その親の文字列を返します。そうでなければ `#f` を返します。

  `read-only`
  : 文字列が読み取り専用であれば `#t`

  `stringbuf-chars`
  : この文字列の stringbuf の文字を含む新しい文字列

  `stringbuf-length`
  : この stringbuf の文字数

  `stringbuf-shared`
  : この stringbuf が共有されていれば `#t`

  `stringbuf-wide`
  : この stringbuf の文字が32ビットのバッファに格納されていれば `#t`、8ビットのバッファに格納されていれば `#f`

---

> **ライセンス**: この翻訳は GNU Free Documentation License v1.3 以降に基づいて作成されています。
> 原文の著作権: Copyright (C) 1996-2023 Free Software Foundation, Inc.
