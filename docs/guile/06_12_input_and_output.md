### 6.12 入力と出力 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Input-and-Output-1)

* [ポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Ports)
* [バイナリ入出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Binary-I_002fO)
* [エンコーディング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Encoding)
* [テキスト入出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Textual-I_002fO)
* [シンプルなテキスト出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Simple-Output)
* [バッファリング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Buffering)
* [ランダムアクセス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Random-Access)
* [行指向および区切りテキスト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Line_002fDelimited)
* [入力、出力、エラーのデフォルトポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Default-Ports)
* [ポートの種類](https://doc.guix.gnu.org/guile/latest/en/guile.html#Port-Types)
* [Venerable Port Interfaces](https://doc.guix.gnu.org/guile/latest/en/guile.html#Venerable-Port-Interfaces)
* [C言語からのポートの使用](https://doc.guix.gnu.org/guile/latest/en/guile.html#Using-Ports-from-C)
* [ノンブロッキングI/O](https://doc.guix.gnu.org/guile/latest/en/guile.html#Non_002dBlocking-I_002fO)
* [Unicodeバイトオーダーマークの処理](https://doc.guix.gnu.org/guile/latest/en/guile.html#BOM-Handling)

* * *

次へ: [バイナリ入出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Binary-I_002fO)、上: [入出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Input-and-Output) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.12.1 ポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Ports-1)

ポートは、Guileが入出力を行うための手段です。Guileは入力ポートから文字やバイトを読み込むことも、出力ポートに書き出すこともできます。一部のポートは両方のインターフェースをサポートしています。

Guileには、さまざまな種類のポートが実装されています。ファイルポートは、その名の通り、ファイルを介して入出力を行います。例えば、次のような文字列をファイルに表示することができます。

(let ((port (open-output-file "foo.txt")))
(「こんにちは、世界！\n」ポートを表示)
（クローズポートポート）

文字列から入力を受け取ったり、文字列に出力を収集したりするための文字列ポート、同じことを行うがバイトベクトルをデータのソースまたはシンクとして使用するバイトベクトルポート、入力を提供したり出力を処理したりするためにScheme関数を呼び出すように設定するためのカスタムポートもあります。[ポートの種類](https://doc.guix.gnu.org/guile/latest/en/guile.html#Port-Types)を参照してください。

ポートは、不要になったら、上記の例のように `close-port` を呼び出して閉じる必要があります。これにより、保留中の出力が、ファイルポートの場合はディスクに、それ以外の場合はポートがサポートする可変ストアに確実に書き込まれるようになります。バッファリングされたデータの書き出し中に発生したエラーは、ガベージコレクタによってポートが閉じられる後ではなく、`close-port` で速やかに発生します。バッファリングされた出力の詳細については、[バッファリング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Buffering) を参照してください。

ポートを閉じると、ファイルが保持している貴重なリソースも解放されます。通常、Scheme ではプログラマーはデータ構造のクリーンアップを行う必要はありません ([メモリ管理とガベージ コレクション](https://doc.guix.gnu.org/guile/latest/en/guile.html#Memory-Management) を参照)。しかし、ほとんどのシステムでは、プロセスごと、システム全体で同時に開けるファイルの数に厳しい制限があります。多くのファイルを使用するプログラムは、これらの制限に達しないように注意する必要があります。パイプやソケットなどの同様のシステム リソースについても同じことが言えます。

実際、これらの理由から、上記の例はポートを使用する最も慣用的な方法ではありません。`call-with-output-file`のようなプロシージャを介してポートを取得する方が一般的で、これらのプロシージャは`close-port`を自動的に処理します。

(出力ファイル "foo.txt" を指定して呼び出し)
(ラムダ (ポート)
(「こんにちは、世界！\n」を表示するポート)))

最後に、すべてのポートには、必要に応じて入力バッファと出力バッファが関連付けられています。バッファリングは、小さな読み書きのオーバーヘッドを制限するための一般的な戦略です。バッファリングがない場合、ファイルからフェッチされる各文字には、少なくとも 1 回、文字とエンコーディングによってはそれ以上のカーネル呼び出しが必要になります。代わりに、Guile は読み書きを内部バッファにバッチ処理します。ただし、ポートの出力をすぐに表示させたい場合もあります。ポートのバッファリングを制御するインターフェイスの詳細については、[バッファリング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Buffering) を参照してください。

Scheme Procedure: **port?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-port_003f)

C 関数: **scm\_port\_p** (x) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fport_005fp)

xがポートであるかどうかを示すブール値を返します。`(or (input-port? x) (output- port? x))`と同等です。

スキーム手順: **input-port?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-input_002dport_003f)

C 関数: **scm\_input\_port\_p** (x) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005finput_005fport_005fp)

xが入力ポートであれば`#t`を返し、そうでなければ`#f`を返します。この述語を満たすオブジェクトはすべて`port?`も満たします。

スキーム手順: **出力ポート?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-output_002dport_003f)

C 関数: **scm\_output\_port\_p** (x) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005foutput_005fport_005fp)

xが出力ポートであれば`#t`を返し、そうでなければ`#f`を返します。この述語を満たすオブジェクトはすべて`port?`も満たします。

スキーム手順: **close-port** ポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-close_002dport)

C 関数: **scm\_close\_port** (port) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fclose_005fport)

指定されたポート オブジェクトを閉じます。ポートが正常に閉じられた場合は `#t` を、既に閉じられていた場合は `#f` を返します。バッファリングされた出力のフラッシュ時など、エラーが発生した場合は例外が発生する場合があります。バッファリングされた出力の詳細については、[Buffering](https://doc.guix.gnu.org/guile/latest/en/guile.html#Buffering) を参照してください。ファイル ディスクリプタを閉じる手順については、[close](https://doc.guix.gnu.org/guile/latest/en/guile.html#Ports-and-File-Descriptors) を参照してください。

スキーム手順: **port-closed?** ポート[¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-port_002dclosed_003f)

C 関数: **scm\_port\_closed\_p** (ポート) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fport_005fclosed_005fp)

ポートが閉じている場合は「#t」を、開いている場合は「#f」を返します。

Scheme Procedure: **call-with-port** port proc [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-call_002dwith_002dport)

procを呼び出し、ポート番号を渡して、procの終了時にポート番号を閉じます。procの戻り値を返します。

* * *

次へ: [エンコーディング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Encoding)、前へ: [ポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Ports)、上へ: [入力と出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Input-and-Output) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.12.2 バイナリ入出力 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Binary-I_002fO-1)

Guile のポートは基本的にバイナリであり、最も低いレベルではバイト単位で動作します。このセクションでは、Guile のコアとなるバイナリ I/O 操作について説明します。文字列や文字の入出力については、[Textual I/O](https://doc.guix.gnu.org/guile/latest/en/guile.html#Textual-I_002fO) を参照してください。

これらのルーチンを使用するには、まずバイナリI/Oモジュールをインクルードしてください。

(use-modules (ice-9 binary-ports))

このモジュールの名前からバイナリポートが他のポートとは異なる種類のポートであるかのように思われるかもしれませんが、実際はそうではありません。Guile のすべてのポートは、バイナリポートとテキストポートの両方の性質を持っています。

スキーム手順: **get-u8** ポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-get_002du8)

C 関数: **scm\_get\_u8** (ポート) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fget_005fu8)

ポートから読み取ったオクテット、入力ポート、必要に応じてブロックするオブジェクト、またはファイル終端オブジェクトを返します。

スキーム手順: **lookahead-u8** ポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-lookahead_002du8)

C 関数: **scm\_lookahead\_u8** (ポート) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005flookahead_005fu8)

`get-u8` と同様ですが、ポートの位置をオクテットを超えて更新しません。

ファイル終端オブジェクトは、他のどの種類のオブジェクトとも異なります。ペアでも、シンボルでも、その他のオブジェクトでもありません。値がファイル終端オブジェクトかどうかを確認するには、`eof-object?`述語を使用します。

Scheme Procedure: **eof-object?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-eof_002dobject_003f)

C 関数: **scm\_eof\_object\_p** (x) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005feof_005fobject_005fp)

xがファイル終端オブジェクトの場合は`#t`を返し、それ以外の場合は`#f`を返します。

このモジュール内の他のプロシージャとは異なり、`eof-object?` はデフォルト環境で定義されていることに注意してください。

スキーム手順: **get-bytevector-n** ポート数 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-get_002dbytevector_002dn)

C 関数: **scm\_get\_bytevector\_n** (port, count) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fget_005fbytevector_005fn)

ポートからcount個のオクテットを読み込み、必要に応じてブロックし、読み取ったオクテットを含むバイトベクトルを返します。利用可能なバイト数がcount個より少ない場合は、count個より小さいバイトベクトルが返されます。

スキームプロシージャ: **get-bytevector-n!** ポート bv 開始カウント [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-get_002dbytevector_002dn_0021)

C 関数: **scm\_get\_bytevector\_n\_x** (port, bv, start, count) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fget_005fbytevector_005fn_005fx)

ポートからcountバイトを読み込み、インデックスstartからbvに格納します。実際に読み取ったバイト数、またはファイル終端オブジェクトを返します。

スキーム手順: **get-bytevector-some** ポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-get_002dbytevector_002dsome)

C 関数: **scm\_get\_bytevector\_some** (ポート) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fget_005fbytevector_005fsome)

ポートからデータを読み込み、必要に応じてブロックしながら、バイトが利用可能になるかファイルの終端に達するまで処理を続けます。ファイルの終端オブジェクト、または利用可能なバイトの一部（少なくとも1バイト）を含む新しいバイトベクトルを返し、ポートの位置をこれらのバイトのすぐ後を指すように更新します。

スキームプロシージャ: **get-bytevector-some!** port bv start count [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-get_002dbytevector_002dsome_0021 )

C 関数: **scm\_get\_bytevector\_some\_x** (port, bv, start, count) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fget_005fbytevector_005fsome_005fx)

ポートから指定されたバイト数を読み込み、少なくとも1バイトが利用可能になるかファイルの終端に達するまで必要に応じてブロックします。読み取ったバイト数は、インデックスの先頭からbvに格納します。実際に読み取ったバイト数、またはファイルの終端オブジェクトを返します。

スキーム手順: **get-bytevector-all** ポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-get_002dbytevector_002dall)

C 関数: **scm\_get\_bytevector\_all** (ポート) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fget_005fbytevector_005fall)

ファイルの終端に達するまで、必要に応じてブロックしながらポートからデータを読み込みます。読み取ったデータを含む新しいバイトベクトル、または（データがない場合は）ファイルの終端オブジェクトを返します。

スキーム手順: **unget-bytevector** ポート bv \[start \[count\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-unget_002dbytevector)

C 関数: **scm\_unget\_bytevector** (ポート、bv、開始、カウント) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005funget_005fbytevector)

bv の内容を port に格納します。必要に応じてインデックス start から開始し、オクテット数に制限することで、後続の読み取り操作時に port から次のバイトとして bv のバイトが左から右に読み込まれるようにします。複数回呼び出された場合、読み取られなかったバイトは後入れ先出しの順序で再度読み込まれます。

ポートにバイナリ出力を行うには、`put-u8` または `put-bytevector` を使用します。

スキーム手順: **put-u8** ポートオクテット[¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-put_002du8)

C 関数: **scm\_put\_u8** (ポート、オクテット) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fput_005fu8)

0～255の範囲の整数であるオクテットを、バイナリ出力ポートであるポートに書き込みます。

スキーム手順: **put-bytevector** port bv \[start \[count\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-put_002dbytevector)

C 関数: **scm\_put\_bytevector** (port, bv, start, count) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fput_005fbytevector)

bvの内容をポートに書き込みます。必要に応じてインデックス開始位置から開始し、オクテット数に制限します。

#### R7RS におけるバイナリ I/O [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Binary-I_002fO-in-R7RS)

[R7RS](https://doc.guix.gnu.org/guile/latest/en/guile.html#R7RS-Standard-Libraries) は、以下のバイナリ I/O プロシージャを定義します。これらにアクセスするには、

(use-modules (scheme base))

Scheme手順: **open-output-bytevector** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-open_002doutput_002dbytevector)

[`get-output-bytevector`](https://doc.guix.gnu.org/guile/latest/en/guile.html#x_002dget_002doutput_002dbytevector) で取得するためのバイトを蓄積するバイナリ出力ポートを返します。

Scheme手順: **write-u8** byte \[out\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-write_002du8)

指定されたバイナリ出力ポート out に 1 バイトを書き込み、未指定の値を返します。out のデフォルト値は `(現在の出力ポート)` です。

[`put-u8`](https://doc.guix.gnu.org/guile/latest/en/guile.html#x_002dput_002du8)も参照してください。

Scheme Procedure: **read-u8** \[in\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002du8)

バイナリ入力ポートから使用可能な次のバイトを返し、ポートを次のバイトを指すように更新します。使用可能なバイトがなくなった場合は、ファイルの終端オブジェクトが返されます。in のデフォルト値は `(current-input-port)` です。

[`get-u8`](https://doc.guix.gnu.org/guile/latest/en/guile.html#x_002dget_002du8)も参照してください。

スキーム手順: **peek-u8** \[in\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-peek_002du8)

バイナリ入力ポートから使用可能な次のバイトを返しますが、ポートを次のバイトを指すように更新しません。使用可能なバイトがなくなったら、ファイルの終端オブジェクトが返されます。 のデフォルト値は `(current-input-port)` です。

[`lookahead-u8`](https://doc.guix.gnu.org/guile/latest/en/guile.html#x_002dlookahead_002du8)も参照してください。

Scheme Procedure: **get-output-bytevector** port [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-get_002doutput_002dbytevector)

これまでにポートに出力されたバイトを、出力された順序で格納したバイトベクトルを返します。ポートが [`open-output-bytevector`](https://doc.guix.gnu.org/guile/latest/en/guile.html#x_002dopen_002doutput_002dbytevector) を使用して作成されていない場合はエラーになります。

(define out (open-output-bytevector))
(write-u8 1 出力)
(write-u8 2 出力)
(write-u8 3 出力)
(get-output-bytevector out) ⇒ #vu8(1 2 3)

スキーム手順: **open-input-bytevector** bv [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-open_002dinput_002dbytevector)

バイトベクトルbvを受け取り、bvからバイトを出力するバイナリ入力ポートを返します。

(define in (open-input-bytevector #vu8(1 2 3)))
(read-u8 in) ⇒ 1
(覗き見) ⇒ 2
(read-u8 in) ⇒ 2
(read-u8 in) ⇒ 3
(read-u8 in) ⇒ #<eof>

Scheme Procedure: **read-bytevector!** bv \[port \[start \[end\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002dbytevector_0021)

バイナリ入力ポートから、次のend - startバイト、またはファイルの末尾までに利用可能なバイト数を、start位置から左から右の順にバイトベクタbvに読み込みます。endが指定されていない場合は、bvの末尾に達するまで読み込みます。startが指定されていない場合は、位置0から読み込みます。

読み取ったバイト数を返します。バイトがない場合は、ファイル終端オブジェクトが返されます。

(define in (open-input-bytevector #vu8(1 2 3)))
(define bv (make-bytevector 5 0))
(read-bytevector! bv in 1 3) ⇒ 2
bv ⇒ #vu8(0 1 2 0 0 0)

スキーム手順: **read-bytevector** k in [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002dbytevector)

バイナリ入力ポートから次のkバイト（ファイル末尾がkバイト未満の場合はファイル末尾までの利用可能なバイト数）を左から右の順に読み込み、新しく割り当てられたバイトベクトルに格納し、そのバイトベクトルを返します。ファイル末尾までの利用可能なバイト数がない場合は、ファイル末尾オブジェクトが返されます。

(define bv #vu8(1 2 3))
(read-bytevector 2 (open-input-bytevector bv)) ⇒ #vu8(1 2)
(read-bytevector 10 (open-input-bytevector bv)) ⇒ #vu8(1 2 3)

Scheme Procedure: **write-bytevector** bv \[port \[start \[end\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-write_002dbytevector)

バイトベクトルbvのバイトを、開始位置から終了位置まで左から右の順にバイナリ出力ポートに書き込みます。開始位置のデフォルト値は0、終了位置のデフォルト値はbvの長さです。

(define out (open-output-bytevector))
(write-bytevector #vu8(0 1 2 3 4) out 2 4)
(get-output-bytevector out) ⇒ #vu8(2 3)

* * *

次へ: [テキスト入出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Textual-I_002fO)、前: [バイナリ入出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Binary-I_002fO)、上: [入出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Input-and-Output) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.12.3 エンコーディング [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Encoding-1)

Guileポートにおけるテキスト入出力は、バイナリ演算の上に構築されています。そのため、各ポートには関連付けられた文字エンコーディングがあり、ポートから読み取られたバイトがどのように文字に変換されるか、またポートに書き込まれた文字がどのようにバイトに変換されるかを制御します。

スキーム手順: **port-encoding** port [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-port_002dencoding)

C 関数: **scm\_port\_encoding** (ポート) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fport_005fencoding)

ポートが入力と出力を解釈するために使用する文字エンコーディングを文字列として返します。

Scheme Procedure: **set-port-encoding!** port enc [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-set_002dport_002dencoding_0021)

C 関数: **scm\_set\_port\_encoding\_x** (port, enc) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fset_005fport_005fencoding_005fx)

ポートへの入出力の解釈に使用される文字エンコーディングを設定します。enc はエンコーディング名を含む文字列です。有効なエンコーディング名は、IANA で定義されているもの (http://www.iana.org/assignments/character-sets) で、たとえば `"UTF-8"` や `"ISO-8859-1"` などです。

ポートが作成されると、エンコーディングが割り当てられます。ポートの初期エンコーディングを決定する通常の手順は、`%default-port-encoding` の値を取得することです。

スキーム変数: **%default-port-encoding** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0025default_002dport_002dencoding)

新しく作成されたポートにデフォルトで使用されるエンコーディングの名前を含む流体（[流体と動的状態](https://doc.guix.gnu.org/guile/latest/en/guile.html#Fluids-and-Dynamic-States)を参照）。特殊なケースとして、値`#f`は`"ISO-8859-1"`と同等です。

`%default-port-encoding` 自体は、`setlocale` が呼び出されている場合、現在のロケールに適したエンコーディングにデフォルトで設定されます。ロケールの詳細や、`setlocale` を明示的に呼び出す必要がある場合については、[ロケール](https://doc.guix.gnu.org/guile/latest/en/guile.html#Locales) を参照してください。

ポートの種類によっては、初期ロケールを決定する別の方法があります。たとえば、文字列ポートは、現在のロケールに関係なくすべての文字を表現できるように、デフォルトで UTF-8 エンコーディングを使用します。ファイルポートは、オプションでファイル内の `coding:` 宣言を検出できます。詳細については、[ファイルポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#File-Ports) を参照してください。バイナリポートは、0 から 255 までの各コードポイントがその値を持つバイトに対応する ISO-8859-1 エンコーディングで初期化される場合があります。

現在、これらのポートは非モーダルエンコーディングのみに対応しています。ほとんどのエンコーディングは非モーダルであり、バイト列から文字列への変換はコンテキストに依存しません。つまり、同じバイト列は常に同じ文字列を返します。ISO-2022-JPやISO-2022-KRなど、モーダルエンコーディングは広く使用されていますが、これらはまだサポートされていません。

各ポートには、関連する変換戦略も設定されています。この戦略は、Guileの文字をポートのエンコードされた文字表現に変換できない場合にどうするかを決定します。可能な戦略は3つあります。エラーを発生させる、文字を16進エスケープシーケンスで置き換える、または文字を代替文字で置き換える、のいずれかです。ポート変換戦略は、入力ポートから文字をデコードする際にも使用されます。

スキーム手順: **port-conversion-strategy** port [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-port_002dconversion_002dstrategy)

C 関数: **scm\_port\_conversion\_strategy** (port) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fport_005fconversion_005fstrategy)

ポートの現在のエンコーディングでは表現できない文字を出力したときの、ポートの動作を返します。

ポートが`#f`の場合、現在のデフォルトの動作が返されます。新しいポートが作成されると、このデフォルトの動作が適用されます。

スキーム手順: **set-port-conversion-strategy!** port sym [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-set_002dport_002dconversion_002dstrategy_0021)

C 関数: **scm\_set\_port\_conversion\_strategy\_x** (port, sym) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fset_005fport_005fconversion_005fstrategy_005fx)

ポートの現在のエンコーディングで表現できない文字を出力する場合、または文字を読み取ろうとしたときに Guile がデコードエラーに遭遇した場合の Guile の動作を設定します。sym には、`error`、`substitute`、または `escape` のいずれかを指定できます。

ポートがオープンポートの場合、そのポートに対して変換エラーの動作が設定されます。`#f` の場合は、このスレッドで今後作成されるすべてのポートのデフォルト動作として設定されます。

ポートエンコーディングと同様に、ポートの初期変換戦略を決定する要素が存在します。

スキーム変数: **%default-port-conversion-strategy** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0025default_002dport_002dconversion_002dstrategy)

新しく作成されたポートの変換戦略、および`scm_to_stringn`、`scm_from_stringn`、`string->pointer`、`pointer->string`などの他の変換ルーチンを定義する流体。

その値は、上記で説明した記号のいずれかで、意味も同じでなければなりません。つまり、`error`、`substitute`、または`escape`です。

Guileが起動したとき、その値は「substitute」です。

`(set-port-conversion-strategy! #f sym)` は `(fluid-set! %default-port-conversion-strategy sym)` と同等であることに注意してください。

前述のとおり、出力ポートには3つのポート変換戦略があります。`error`戦略では、変換できない文字が検出されるとエラーが発生します。`substitute`戦略では、変換できない文字を疑問符（'?'）に置き換えます。最後に、`escape`戦略では、Guileの文字列構文で認識されるエスケープ処理を使用して、変換できない文字を16進エスケープとして出力します。ポートのエンコーディングが`UTF-8`などのUnicodeエンコーディングの場合、エンコーディングエラーは発生しないことに注意してください。

入力ポートの場合、`error` ストラテジーでは、`ISO-8859-1` を `UTF-8` として読み込もうとした場合など、無効なエンコーディングに遭遇すると Guile はエラーをスローします。エラーは、読み取り位置を進める前にスローされます。`substitute` ストラテジーでは、Unicode の推奨事項に従って、不正なバイトを U+FFFD 置換文字に置き換えます。入力ポートから読み取る場合、`escape` ストラテジーは `error` と同様に扱われます。

* * *

次へ: [シンプルなテキスト出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Simple-Output)、前: [エンコーディング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Encoding)、上: [入力と出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Input-and-Output) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.12.4 テキスト入出力 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Textual-I_002fO-1)

このセクションでは、Guile の文字と文字列に対するコアとなるテキスト入出力操作について説明します。バイトとバイトベクトルの入出力については、[バイナリ入出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Binary-I_002fO) を参照してください。文字とバイトの関係については、[エンコーディング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Encoding) を参照してください。ポートから一般的な S 式を読み取るには、[Scheme コードの読み取り](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scheme-Read) を参照してください。一般的な Scheme データを書き込むインターフェイスについては、[Scheme 値の書き込み](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scheme-Write) を参照してください。

これらのルーチンを使用するには、まずテキスト入出力モジュールをインクルードしてください。

(use-modules (ice-9 textual-ports))

このモジュールの名前から、テキストポートは他のポートとは異なる種類のポートであるかのように思われるかもしれませんが、実際はそうではありません。Guile のすべてのポートは、バイナリポートとテキストポートの両方の性質を持っています。

Scheme手順: **get-char** 入力ポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-get_002dchar)

入力ポートから文字全体を読み込むか、ファイルの終端に達するまで、必要に応じてブロックしながら入力ポートからデータを読み込みます。

ファイルの末尾に到達する前に完全な文字が利用可能な場合、`get-char` はその文字を返し、入力ポートをその文字より先を指すように更新します。文字が読み込まれる前にファイルの末尾に達した場合、`get-char` はファイルの末尾オブジェクトを返します。

Scheme Procedure: **lookahead-char** input-port [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-lookahead_002dchar)

`lookahead-char` プロシージャは `get-char` に似ていますが、入力ポートを文字の先を指すように更新しません。

1バイトまたは複数のバイトを「アンゲット」できるのと同様に、エンコードされた文字に対応するバイトを「アンゲット」することも可能です。

スキーム手順: **unget-char** ポート文字 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-unget_002dchar)

文字charをポートに配置し、次の読み取り操作で読み込まれるようにします。複数回呼び出された場合、読み込まれていない文字は後入れ先出しの順序で再度読み込まれます。

スキーム手順: **unget-string** port str [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-unget_002dstring)

文字列strをportに配置すると、後続の読み取り操作でportから次の文字としてstrの文字が左から右に読み込まれます。複数回呼び出された場合、読み込まれていない文字は後入れ先出しの順序で再度読み込まれます。

文字を1文字ずつ読み込むのは非効率的です。文字列を介して複数の文字を一度に入出力できるのであれば、そちらの方が高速になる可能性があります。

Scheme Procedure: **get-string-n** input-port count [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-get_002dstring_002dn)

`get-string-n` プロシージャは、入力ポートからデータを読み込み、必要に応じてブロックしながら、count 文字が利用可能になるか、ファイルの終端に達するまで処理を続けます。count は、読み込む文字数を表す、正確な非負の整数でなければなりません。

ファイル終端までにcount文字が利用可能な場合、`get-string-n`はそれらのcount文字からなる文字列を返します。ファイル終端までに利用可能な文字数が少ない場合でも、1文字以上読み取れる場合は、それらの文字を含む文字列を返します。いずれの場合も、入力ポートは読み取った文字の直後を指すように更新されます。ファイル終端までに読み取れない文字がある場合は、ファイル終端オブジェクトが返されます。

Scheme Procedure: **get-string-n!** input-port string start count [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-get_002dstring_002dn_0021)

`get-string-n!` プロシージャは、`get-string-n` と同様の方法で input-port からデータを読み取ります。start と count は、正確に指定された非負の整数オブジェクトである必要があり、count は読み取る文字数を表します。string は、少なくとも $start + count$ 文字を含む文字列である必要があります。

ファイル終端までにcount文字が利用可能な場合、それらの文字はインデックス開始位置から始まる文字列に書き込まれ、countが返されます。ファイル終端までに利用可能な文字数がcountより少ない場合でも、1文字以上読み取れる場合は、それらの文字がインデックス開始位置から始まる文字列に書き込まれ、実際に読み取られた文字数が正確な整数オブジェクトとして返されます。ファイル終端までに読み取れない文字がある場合は、ファイル終端オブジェクトが返されます。

Scheme 手順: **get-string-all** 入力ポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-get_002dstring_002dall)

入力ポートからファイルの末尾まで読み込み、`get-string-n` および `get-string-n!` と同様の方法で文字をデコードします。

ファイル末尾より前に文字が存在する場合は、そのデータからデコードされたすべての文字を含む文字列が返されます。ファイル末尾より前に文字が存在しない場合は、ファイル末尾オブジェクトが返されます。

Scheme Procedure: **get-line** input-port [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-get_002dline)

入力ポートから改行文字またはファイルの末尾までを読み取り、`get-string-n` および `get-string-n!` と同様の方法で文字をデコードします。

改行文字が読み込まれた場合、改行文字までのテキスト全体（改行文字自体は含まない）を含む文字列が返され、ポートは改行文字の直後を指すように更新されます。改行文字が読み込まれる前にファイルの終端に達したが、一部の文字が読み込まれて文字としてデコードされている場合は、それらの文字を含む文字列が返されます。文字が読み込まれる前にファイルの終端に達した場合は、ファイルの終端オブジェクトが返されます。

最後に、ポートに文字を書き込むための基本的な手順は2つだけです。

Scheme手順: **put-char** port char [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-put_002dchar)

ポートに文字を書き込みます。`put-char` プロシージャは未指定の値を返します。

Scheme Procedure: **put-string** ポート文字列 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-put_002dstring)

Scheme Procedure: **put-string** port string start [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-put_002dstring-1)

Scheme Procedure: **put-string** port string start count [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-put_002dstring-2)

インデックス start から始まる文字列の文字数をポートに書き込みます。

start と count は、負でない正確な整数オブジェクトである必要があります。文字列の長さは、少なくとも _start + count_ である必要があります。start のデフォルト値は 0 です。count のデフォルト値は _`(文字列の長さの文字列)` - start_$ です。

`put-string` を呼び出すことは、関連する文字シーケンスに対して `put-char` を呼び出すこととあらゆる点で同等ですが、ポートがバッファリングされていない場合でも、一度に複数の文字をポートに書き込もうとします。

`put-string` プロシージャは未指定の値を返します。

テキストポートには、行と列というテキスト上の位置が関連付けられています。文字を読み込んだり書き出したりすると、行と列がそれに応じて移動します。

スキーム手順: **port-column** ポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-port_002dcolumn)

スキーム手順: **port-line** ポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-port_002dline)

C 関数: **scm\_port\_column** (ポート) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fport_005fcolumn)

C 関数: **scm\_port\_line** (ポート) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fport_005fline)

ポートの現在の列番号または行番号を返します。

ポートの行と位置は、0を起点とする整数で表されます。つまり、最初の行の最初の文字は、0行目、0列目となります。ただし、エラーメッセージなどで行番号を表示する場合は、1を加算して1を起点とする整数にすることをお勧めします。これは、行番号は従来1から始まるため、プログラマー以外のユーザーにとって最も自然な表記となるからです。

Scheme Procedure: **set-port-column!** ポート列 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- set_002dport_002dcolumn_0021)

Scheme Procedure: **set-port-line!** ポートライン [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-set_002dport_002dline_0021)

C 関数: **scm\_set\_port\_column\_x** (ポート、列) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fset_005fport_005fcolumn_005fx)

C 関数: **scm\_set\_port\_line\_x** (port, line) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fset_005fport_005fline_005fx)

ポートの現在の列番号または行番号を設定します。

* * *

次へ: [バッファリング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Buffering)、前: [テキスト入出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Textual-I_002fO)、上: [入出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Input-and-Output) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.12.5 シンプルなテキスト出力 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Simple-Textual-Output)

Guile は、シンプルな書式付き出力関数 `simple-format` を提供しています。より高度な書式付き出力機能については、[書式付き出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Formatted-Output) を参照してください。

Scheme Procedure: **simple-format** destination message . args [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-simple_002dformat)

C 関数: **scm\_simple\_format** (宛先、メッセージ、引数) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsimple_005fformat)

メッセージを宛先に書き込みます。デフォルトは現在の出力ポートです。メッセージには `~A` および `~S` エスケープ文字を含めることができます。印刷時には、エスケープ文字は args の対応するメンバーに置き換えられます。`~A` は `display` を使用してフォーマットし、`~S` は `write` を使用してフォーマットします。宛先が `#t` の場合は現在の出力ポートを使用し、宛先が `#f` の場合はフォーマットされたテキストを含む文字列を返します。末尾に改行は追加されません。

やや紛らわしいことに、Guile は起動時に `format` 識別子を `simple-format` にバインドします。`(ice-9 format)` がロードされると、実際にはコアの `format` バインディングが置き換えられるため、あなた自身または使用しているモジュールが `(ice-9 format)` をロードしているかどうかによって、シンプルなバージョンまたはより高機能なバージョンを使用することになります。

* * *

次へ: [ランダムアクセス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Random-Access)、前: [シンプルなテキスト出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Simple-Output)、上: [入力と出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Input-and-Output) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.12.6 バッファリング [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Buffering-1)

すべてのポートには、入力バッファと出力バッファが関連付けられています。ポートは可変ストアによって支えられていると考えることができ、そのストアは遠く離れている可能性があります。たとえば、ファイルディスクリプタによって支えられているポートは、データの読み書きのためにカーネルまでアクセスする必要があります。この往復コストを回避するために、Guileは通常、可変ストアからデータをチャンク単位で読み込み、その中間バッファから`get-char`のような小さな要求を処理します。同様に、`write-char`のような小さな書き込みは、まずバッファに書き込まれ、バッファがいっぱいになったとき（またはポートがフラッシュされたとき）にストアに送信されます。バッファ付きポートは、可変ストアへの往復回数を減らすことでプログラムの速度を向上させ、その仕組みはユーザーにはほとんど透過的です。

しかし、バッファリングがプログラムの意味論に影響を与える主な方法は2つあります。正しく、かつ高性能なプログラムを構築するには、これらの状況を理解することが不可欠です。

最初のケースは、ランダムアクセス読み書きポートです（[ランダムアクセス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Random-Access)を参照）。これらのポートは通常、ファイルによってバックアップされ、読み書きの両方で論理的に同じ可変ストアを操作します。したがって、文字を読み込んでバッファがいっぱいになり、次に文字を書き込むと、読み取ったバッファに書き込まれたバイトは無効になります。読み書きを切り替えるたびに、Guile は保留中のバッファをフラッシュする必要があります。これが頻繁に発生すると、コストが高くなる可能性があります。その場合は、両方向のバッファの量を減らす必要があります。同様に、Guile はシークする前にバッファをフラッシュする必要があります。これらの考慮事項は、論理的に同じ可変ストアから読み書きせず、シークもできないソケットには適用されません。また、ソケットはデフォルトではバッファリングされていないことに注意してください。 [ネットワークソケットと通信](https://doc.guix.gnu.org/guile/latest/en/guile.html#Network-Sockets-and-Communication)を参照してください。

2番目のケースの方がより厄介です。バッファ付きポートにデータを書き込むと、おそらく直接可変ストアには出力されません。（この「おそらく」という表現は、プログラムに不確定性をもたらします。ストアに何がいつ書き込まれるかは、バッファがどれだけいっぱいかによって決まります。これはユーザーが明示的に認識しておく必要がある点です。）データは後でストアに書き込まれます。別の書き込みによってバッファがいっぱいになったとき、`force-output`が呼び出されたとき、`close-port`が呼び出されたとき、プログラムが終了したとき、あるいはガベージコレクタが実行されたときなどです。重要な点は、_その時にもエラーが通知される_ということです。バッファ付き書き込みはエラー検出を遅延させ（そして可変ストアへの副作用も遅延させます）、ポートタイプがGCで閉じる必要がない場合は、おそらく無期限に遅延します。

テキストポートでよく用いられるヒューリスティックの一つに、改行文字（`\n`）が書き込まれたときに出力をフラッシュするという方法があります。この行バッファリングモードは、TTYポートではデフォルトで有効になっています。その他のほとんどのポートはブロックバッファリング方式を採用しており、出力バッファがポートとその構成によって決まるブロックサイズに達すると、ブロックの内容に関係なく、出力はブロック単位でフラッシュされます。同様に、読み込みもブロックサイズで行われますが、読み取れるバイト数が少ない場合は、バッファが完全に満たされない可能性があります。

バッファサイズよりも大きいバイナリの読み書きは、バッファを経由せずに直接可変ストアに書き込まれることに注意してください。アクセスパターンに多くの大きな読み書きが含まれる場合は、バッファリングはそれほど重要ではないかもしれません。

ポートのバッファリング動作を制御するには、`setvbuf`を使用します。

Scheme Procedure: **setvbuf** port mode \[size\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-setvbuf)

C 関数: **scm\_setvbuf** (ポート、モード、サイズ) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsetvbuf)

ポートのバッファリングモードを設定します。modeには、以下のいずれかの記号を指定できます。

`なし`

非バッファリング

`line`

ラインバッファリング

`ブロック`

ブロックバッファリング方式を採用し、新しく割り当てられたサイズバイトのバッファを使用します。サイズを省略した場合、デフォルトのサイズが使用されます。

ファイルポートのバッファリングを設定する別の方法として、モード文字列の一部として「0」または「l」を指定してファイルを開く方法があります。これは、それぞれバッファなしポートまたはラインバッファ付きポートに対応します。詳細については、[ファイルポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#File-Ports)を参照してください。

バッファリングされた出力データは、ポートが閉じられると書き出されます。プログラムの特定の箇所で確実にフラッシュするには、`force-output`を使用してください。

Scheme Procedure: **force-output** \[port\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-force_002doutput)

C 関数: **scm\_force\_output** (ポート) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fforce_005foutput)

指定された出力ポート、またはポートが省略された場合は現在の出力ポートをフラッシュします。現在の出力バッファの内容（存在する場合）は、基となるポート実装に渡されます。

戻り値は指定されていません。

スキーム手順: **flush-all-ports** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-flush_002dall_002dports)

C 関数: **scm\_flush\_all\_ports** () [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fflush_005fall_005fports)

開いているすべての出力ポートに対して`force-output`を呼び出すのと同等です。戻り値は未定義です。

同様に、Guileのポートを使用する代わりに、ファイルディスクリプタを直接操作したい場合もあるでしょう。その場合は、入力ポートに対して`drain-input`コマンドを使用して、そのポートからバッファリングされた入力を取得します。

スキーム手順: **drain-input** ポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-drain_002dinput)

C 関数: **scm\_drain\_input** (ポート) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fdrain_005finput)

この手順は、force-output が出力バッファをクリアするのと同様に、ポートの入力バッファをクリアします。バッファの内容は単一の文字列として返されます。例:

(define p ([open-input-file](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-open_002dinput_002dfile) [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e)))
([drain-input](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-drain_002dinput) p) [\=>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d_003e) 空の文字列、まだバッファリングされていません。
([unread-char](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-unread_002dchar) ([read-char](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002dchar) p) p)
([drain-input](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-drain_002dinput) p) [\=>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d_003e) p から [up](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-up) までの最初の文字をバッファサイズまで書き込みます。

これらの考慮事項はすべて、Cライブラリのストリームに関するものと非常によく似ていますが、Guileの移植版はCストリームをベースに構築されているわけではありません。それでも、他のシステムがどのように処理しているかを知ることは有益です。Cストリームの詳細については、『GNU Cライブラリリファレンスマニュアル』の[Streams](https://doc.guix.gnu.org/libc/latest/en/libc.html#Streams)を参照してください。

* * *

次へ: [行指向区切りテキスト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Line_002fDelimited)、前: [バッファリング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Buffering)、上: [入力と出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Input-and-Output) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.12.7 ランダムアクセス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Random-Access-1)

Scheme Procedure: **seek** fd\_port offset whence [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-seek)

C 関数: **scm\_seek** (fd\_port, offset, whence) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fseek)

fd\_port の現在の位置を整数オフセットに設定します。ファイルポートの場合、オフセットはバイト数で表されます。文字列ポートなどの他のタイプのポートの場合、オフセットはポートのデータ内の位置の抽象的な表現であり、必ずしもバイト数で表されるとは限りません。オフセットは whence の値に従って解釈されます。

whereceには、以下のいずれかの変数を指定する必要があります。

変数: **SEEK\_SET** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SEEK_005fSET)

ファイルの先頭からシークします。

変数: **SEEK\_CUR** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SEEK_005fCUR)

現在位置から探索する。

変数: **SEEK\_END** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SEEK_005fEND)

ファイルの末尾からシークします。

GNU/Linuxなどの対応システムでは、スパースファイル内の「穴」をナビゲートするために、以下の定数を使用できます。

変数: **SEEK\_DATA** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SEEK_005fDATA)

データを含む、オフセット以上のファイル内の次の位置にシークします。オフセットがデータの位置を指している場合は、ファイルオフセットがオフセットに設定されます。

変数: **SEEK\_HOLE** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SEEK_005fHOLE)

ファイル内のオフセット以上の次の穴にシークします。オフセットが穴の中央を指している場合は、ファイルオフセットがオフセットに設定されます。オフセットより先に穴がない場合は、ファイルオフセットがファイルの末尾に調整されます。つまり、どのファイルにも末尾に暗黙の穴が存在します。

fd\_port がファイルディスクリプタの場合、基となるシステムコールは `lseek` です (GNU C ライブラリ リファレンス マニュアルの [ファイル位置プリミティブ](https://doc.guix.gnu.org/libc/latest/en/libc.html#File-Position-Primitive) を参照)。port は文字列 port にすることができます。

返される値は、fd\_port 内の新しい位置です。つまり、ポートの現在の位置は次のようにして取得できます。

([seek](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-seek) ポート 0 [SEEK\_CUR](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SEEK_005fCUR))

Scheme Procedure: **ftell** fd\_port [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ftell)

C 関数: **scm\_ftell** (fd\_port) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fftell)

fd_port の現在位置を表す整数値を返します（開始位置からの計測値）。以下と同等です。

([seek](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-seek) ポート 0 [SEEK\_CUR](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SEEK_005fCUR))

Scheme手順: **truncate-file** file \[length\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-truncate_002dfile)

C 関数: **scm\_truncate\_file** (ファイル、長さ) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ftruncate_005ffile)

ファイルを指定されたバイト数に切り詰めます。fileには、ファイル名文字列、ポートオブジェクト、または整数型のファイルディスクリプタを指定できます。戻り値は未定義です。

ポートまたはファイルディスクリプタの長さは省略できます。その場合、ファイルは現在の位置で切り詰められます（上記の`ftell`を参照）。

ほとんどのシステムでは、現在のサイズよりも大きい長さを指定することでファイルを拡張できますが、これはPOSIX規格では必須ではありません。

* * *

次へ: [入力、出力、エラーのデフォルトポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Default-Ports)、前: [ランダムアクセス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Random-Access)、上: [入力と出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Input-and-Output) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.12.8 行指向テキストと区切りテキスト [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Line-Oriented-and-Delimited-Text)

区切り文字付きI/Oモジュールには、以下の方法でアクセスできます。

([use-modules](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-use_002dmodules) (ice-9 rdelim))

これは、テキスト行の読み書き、または指定された文字セットで区切られたテキストの読み取りに使用できます。

Scheme手順: **read-line** \[port\] \[handle-delim\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002dline)

指定されたポートからテキスト行を返します。指定されていない場合は、`(current-input-port)` によって返される値からテキスト行を返します。Unix では、テキスト行は最初の行末文字またはファイル末尾で終了します。

handle-delimを指定する場合は、以下のいずれかの記号を指定してください。

`trim`

終端区切り文字を破棄します。これはデフォルトの動作ですが、読み取りが区切り文字で終了したのか、ファイルの終端で終了したのかを判別できなくなります。

`concat`

返された文字列に、終端区切り文字（存在する場合）を追加します。

覗き見る

終端区切り文字（存在する場合）をポートに戻します。

`split`

ポートから読み取った文字列と、終端を示す区切り文字またはファイル終端オブジェクトを含むペアを返します。

Scheme Procedure: **read-line!** buf \[port\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002dline_0021)

指定された文字列 buf にテキスト 1 行を読み込み、buf に追加された文字数を返します。buf が満杯の場合は、`#f` を返します。ポートが指定されている場合はそこから読み込み、指定されていない場合は `(current-input-port)` によって返される値から読み込みます。

Scheme Procedure: **read-delimited** delims \[port\] \[handle-delim\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002ddelimited)

文字列delims内のいずれかの文字が見つかるか、ファイルの終端に達するまでテキストを読み込みます。ポートが指定されている場合はそのポートから読み込み、指定されていない場合は`(current-input-port)`によって返される値から読み込みます。handle-delimは`read-line`で説明されている値と同じ値を取ります。

Scheme Procedure: **read-delimited!** delims buf \[port\] \[handle-delim\] \[start\] \[end\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002ddelimited_0021)

指定された文字列バッファにテキストを読み込みます。

区切り文字が見つかった場合は、書き込まれた文字数を返します。ただし、handle-delim が `split` の場合は、上記のように戻り値はペアになります。

特殊なケースとして、ポートが既にストリームの終端に達している場合は、EOFオブジェクトが返されます。また、バッファが満杯で文字が書き込まれなかった場合は、`#f`が返されます。

正直言って、ちょっと変わったインターフェースですね。

Scheme Procedure: **%read-delimited!** delims str gobble \[port \[start \[end\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0025read_002ddelimited_0021)

C 関数: **scm\_read\_delimited\_x** (delims, str, gobble, port, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fread_005fdelimited_005fx)

指定された区切り文字が見つかるまで、port から str に文字を読み込みます。gobble が true の場合、区切り文字を破棄します。そうでない場合は、次の読み取りのために入力ストリームに残します。port が指定されていない場合は、`(current-input-port)` の値を使用します。start または end が指定されている場合は、start と end で囲まれた str の部分文字列にのみデータを格納します (デフォルト値はそれぞれ文字列の先頭と末尾です)。

文字列の終端を示す区切り文字と読み取った文字数のペアを返します。ファイルの末尾で読み取りが停止した場合、返される区切り文字は eof オブジェクトです。区切り文字に遭遇せずに文字列が読み込まれた場合、この値は `#f` です。

Scheme手順: **%read-line** \[port\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0025read_002dline)

C 関数: **scm\_read\_line** (ポート) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fread_005fline)

ポートから改行で終端された行を読み込み、必要に応じてストレージを割り当てます。文字列から改行終端文字（存在する場合）が削除され、行とその区切り文字からなるペアが返されます。区切り文字は改行文字またはeofオブジェクトのいずれかです。ファイルの末尾で`%read-line`が呼び出された場合、`(#<eof> . #<eof>)`というペアが返されます。

Scheme手順: **write-line** obj \[port\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-write_002dline)

C 関数: **scm\_write\_line** (obj, port) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fwrite_005fline)

objと改行文字をポートに表示します。ポートが指定されていない場合は、`(現在の出力ポート)`が使用されます。この手順は以下と同等です。

([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) obj \[port\])
([newline](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-newline) \[port\])

スキーム手順: **for-rdelim-from-port** port proc rdelim-proc \[#:stop-pred=eof-object?\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-for_002drdelim_002dfrom_002dport)

`(rdelim-proc port)` によって提供される各ユニットに対して、このユニット (rdelim) を処理対象の proc に渡します。これは、stop-pred が `#t` を返すまで、port 全体で継続されます。stop-pred はデフォルトで `eof-object?` です。rdelim-proc は、呼び出されるたびに port 内を進む必要があります。

スキーム手順: **for-delimited-from-port** port proc \[#:delims=”\\n”\] \[#:handle-delim='trim\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-for_002ddelimited_002dfrom_002dport)

ポートからdelimsで区切られた各行に対してprocを呼び出します。

Scheme Procedure: **for-line-in-file** file proc \[#:encoding=#f\] \[#:guess-encoding=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-for_002dline_002din_002dfile)

ファイル内の各行に対してプロシージャを呼び出します。ファイルはファイル名文字列である必要があります。

procに渡される行は、必ず文字列である必要があります。

* * *

次へ: [ポートの種類](https://doc.guix.gnu.org/guile/latest/en/guile.html#Port-Types)、前: [行指向および区切りテキスト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Line_002fDelimited)、上: [入力と出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Input-and-Output) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.12.9 入力、出力、およびエラーのデフォルトポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Default-Ports-for-Input_002c-Output-and-Errors)

スキーム手順: **current-input-port** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-current_002dinput_002dport)

C 関数: **scm\_current\_input\_port** () [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fcurrent_005finput_005fport)

現在の入力ポートを返します。これは、多くの入力処理で使用されるデフォルトのポートです。

最初は、これはUnixおよびC言語の用語でいうところの「標準入力」です。標準入力がTTYの場合、ポートはバッファリングされませんが、それ以外の場合は完全にバッファリングされます。

アプリケーションが対話型サブプロセスを実行する場合、バッファリングなしの入力は好ましい。なぜなら、タイプアヘッド入力がGuileのバッファに入り込まず、サブプロセスから利用できなくなることがないからである。

Guileのバッファリングは、TTYの「行制御」とは完全に独立していることに注意してください。TTYの通常のクックモードでは、ユーザーがReturnキーを押したときに初めてGuileは入力行を認識します。

スキーム手順: **current-output-port** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-current_002doutput_002dport)

C 関数: **scm\_current\_output\_port** () [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fcurrent_005foutput_005fport)

現在の出力ポートを返します。これは、多くの出力処理で使用されるデフォルトのポートです。

初期状態では、これはUnixおよびC言語の用語でいうところの「標準出力」です。標準出力がTTYの場合、このポートはバッファリングされませんが、それ以外の場合は完全にバッファリングされます。

TTY へのバッファなし出力は、進捗状況の出力やプロンプトが確実に表示されるようにするのに適しています。しかし、常に全行を出力するアプリケーションは行バッファリングに変更したり、出力が多いアプリケーションは完全にバッファリングし、特定の箇所で明示的に `force-output` 呼び出しを行うように変更することもできます ([バッファリング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Buffering) を参照)。

スキーム手順: **current-error-port** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-current_002derror_002dport)

C 関数: **scm\_current\_error\_port** () [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fcurrent_005ferror_005fport)

エラーや警告を送信するポート番号を返します。

元々は、UnixおよびC言語の用語でいうところの「標準エラー」です。標準エラーがTTYの場合、このポートはバッファリングされませんが、そうでない場合は完全にバッファリングされます。

Scheme Procedure: **set-current-input-port** port [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-set_002dcurrent_002dinput_002dport)

スキーム手順: **set-current-output-port** port [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-set_002dcurrent_002doutput_002dport)

Scheme Procedure: **set-current-error-port** port [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-set_002dcurrent_002derror_002dport)

C 関数: **scm\_set\_current\_input\_port** (port) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fset_005fcurrent_005finput_005fport)

C 関数: **scm\_set\_current\_output\_port** (port) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fset_005fcurrent_005foutput_005fport)

C 関数: **scm\_set\_current\_error\_port** (port) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fset_005fcurrent_005ferror_005fport)

`current-input-port`、`current-output-port`、`current-error-port`がそれぞれ返すポートを、入力または出力に指定されたポートを使用するように変更します。

Scheme Procedure: **with-input-from-port** port thunk [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-with_002dinput_002dfrom_002dport)

Scheme 手順: **with-output-to-port** ポートサンク [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-with_002doutput_002dto_002dport)

Scheme 手順: **with-error-to-port** ポートサンク [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-with_002derror_002dto_002dport)

`current-input-port`、`current-output-port`、または`current-error-port`が指定されたポートに再バインドされる動的な環境でthunkを呼び出します。

C 関数: `void` **scm\_dynwind\_current\_input\_port** `(SCM ポート)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fdynwind_005fcurrent_005finput_005fport)

C 関数: `void` **scm\_dynwind\_current\_output\_port** `(SCM ポート)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fdynwind_005fcurrent_005foutput_005fport)

C 関数: `void` **scm\_dynwind\_current\_error\_port** `(SCM ポート)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fdynwind_005fcurrent_005ferror_005fport)

これらの関数は、`scm_dynwind_begin` と `scm_dynwind_end` の呼び出しのペア内で使用する必要があります ([Dynamic Wind](https://doc.guix.gnu.org/guile/latest/en/guile.html#Dynamic-Wind) を参照)。dynwind コンテキスト中は、指定されたポートが port に設定されます。

より正確には、dynwindコンテキストに入るとき、または抜けるときに、現在のポートが「バックアップ」値と交換されます。バックアップ値は、ポート引数で初期化されます。

* * *

次へ: [Venerable Port Interfaces](https://doc.guix.gnu.org/guile/latest/en/guile.html#Venerable-Port-Interfaces)、前: [Default Ports for Input, Output and Errors](https://doc.guix.gnu.org/guile/latest/en/guile.html#Default-Ports)、上: [Input and Output](https://doc.guix.gnu.org/guile/latest/en/guile.html#Input-and-Output) \[[Contents](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "Table of contents")\]\[[Index](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "Index")\]

#### 6.12.10 ポートの種類[¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Types-of-Port)

* [ファイルポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#File-Ports)
* [Bytevector ポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevector-Ports)
* [String Ports](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Ports)
* [カスタムポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Custom-Ports)
* [ソフトポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Soft-Ports)
* [Void Ports](https://doc.guix.gnu.org/guile/latest/en/guile.html#Void-Ports)
* [低レベルカスタムポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Low_002dLevel-Custom-Ports)
* [C言語における低レベルカスタムポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Low_002dLevel-Custom-Ports-in-C)

* * *

次へ: [バイトベクトルポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevector-Ports)、上へ: [ポートの種類](https://doc.guix.gnu.org/guile/latest/en/guile.html#Port-Types) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.12.10.1 ファイルポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#File-Ports-1)

ファイルポートを開くには、以下の手順を使用します。Unixの`open`システムコールへのインターフェースについては、[open](https://doc.guix.gnu.org/guile/latest/en/guile.html#Ports-and-File-Descriptors)も参照してください。

すべてのファイルアクセスは、利用可能な場合は「LFS」（ラージファイルサポート機能）を使用するため、2ギビバイト（2^31バイト）を超えるファイルも32ビットシステムで読み書きできます。

ほとんどのシステムでは、同時に開くことができるファイルの数に制限があるため、不要になったファイルポートは明示的に閉じることを強くお勧めします（ [Ports](https://doc.guix.gnu.org/guile/latest/en/guile.html#Ports)を参照）。

Scheme Procedure: **open-file** filename mode \[#:guess-encoding=#f\] \[#:encoding=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-open_002dfile)

C 関数: **scm\_open\_file\_with\_encoding** (filename, mode, guess\_encoding, encoding) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fopen_005ffile_005fwith_005fencoding)

C 関数: **scm\_open\_file** (ファイル名、モード) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fopen_005ffile)

ファイル名が filename のファイルを開き、そのファイルを表すポートを返します。ポートの属性は mode 文字列によって決定されます。この解釈方法は C 言語の stdio と同様です。最初の文字は次のいずれかでなければなりません。

' r'

入力用に既存のファイルを開きます。

「 w」

出力用のファイルを開き、存在しない場合は作成し、存在する場合はその内容を削除します。

「 a」

出力用のファイルを開きます。ファイルが存在しない場合は作成します。ポートへの書き込みはすべてファイルの末尾に行われます。ポート使用中は「追記モード」をオフにすることができます。詳細は[fcntl](https://doc.guix.gnu.org/guile/latest/en/guile.html#Ports-and-File-Descriptors)を参照してください。

以下の追加文字を付加できます。

「 b」

システムがバイナリモードをサポートしている場合は、基となるファイルをバイナリモードで開きます。また、デフォルトのポートエンコーディングを無視し、バイナリ互換の文字エンコーディング「ISO-8859-1」を使用してファイルを開きます。

' +'

入力と出力の両方にポートを開きます。例：`r+`：既存のファイルを入力と出力の両方に開きます。

「 e」

`O_CLOEXEC`フラグに従って、基となるファイルディスクリプタをclose-on-execとしてマークします。

「 0」

「バッファなし」ポートを作成します。この場合、入出力操作は追加のバッファリングなしで、基盤となるポート実装に直接渡されます。これにより、I/O操作が遅くなる可能性があります。バッファリングモードは、ポートの使用中に変更できます（[バッファリング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Buffering)を参照）。

「 l」

ポートにラインバッファリング機能を追加します。ポートの出力バッファは、改行文字が書き込まれるたびに自動的にフラッシュされます。

「 b」

バイナリモードを使用することで、ファイル内の各バイトが1つのScheme文字として読み込まれるようになります。

このプロパティを提供するために、ファイルはデフォルトのポートエンコーディングを無視して、8ビット文字エンコーディング「ISO-8859-1」で開かれます。ポートエンコーディングの詳細については、[Ports](https://doc.guix.gnu.org/guile/latest/en/guile.html#Ports)を参照してください。

バイナリデータを文字または文字列として読み書きすることも可能ですが、通常はバイトをオクテットとして、バイトシーケンスをバイトベクトルとして扱う方が望ましいことに注意してください。詳細については、[バイナリ入出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Binary-I_002fO)を参照してください。

このオプションには、DOSとの互換性という歴史的な意味合いもありました。デフォルトの（テキスト）モードでは、DOSはCR-LFシーケンスを1バイトのLFとして読み取ります。`b`フラグは、基となる`open`呼び出しに`O_BINARY`を追加することで、この読み取りを防止します。とはいえ、このフラグはポートエンコーディングへの影響があるため、一般的に有用です。

バイナリ モードが要求されていない限り、新しいポートの文字エンコーディングは次のように決定されます。まず、guess-encoding が true の場合、`file-encoding` プロシージャを使用してファイルのエンコーディングを推測します ([ソース ファイルの文字エンコーディング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Character-Encoding-of-Source-Files) を参照)。guess-encoding が false の場合、または `file-encoding` が失敗した場合、encoding も false でない限り、encoding が使用されます。最終手段として、デフォルトのポート エンコーディングが使用されます。ポート エンコーディングの詳細については、[ポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Ports) を参照してください。バイナリ モードが要求されているにもかかわらず、guess-encoding または encoding が false でない場合、エラーとなります。

要求されたアクセス権限でファイルを開くことができない場合、`open-file` は例外をスローします。

Scheme Procedure: **open-input-file** filename \[#:guess-encoding=#f\] \[#:encoding=#f\] \[#:binary=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-open_002dinput_002dfile)

入力用にファイル名を開きます。binary が true の場合、ポートをバイナリモードで開きます。それ以外の場合は、テキストモードを使用します。encoding と guess-encoding は、上記の `open-file` の説明に従って文字エンコーディングを決定します。以下と同等です。

([open-file](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-open_002dfile) ファイル名
(バイナリ「rb」「r」の場合)
#:guess-encoding guess-encoding
#:エンコーディングエンコーディング)

Scheme Procedure: **open-output-file** filename \[#:encoding=#f\] \[#:binary=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-open_002doutput_002dfile)

出力用にファイル名を開きます。binary が true の場合、ポートをバイナリモードで開きます。それ以外の場合は、テキストモードを使用します。encoding は、上記の `open-file` の説明に従って文字エンコーディングを指定します。以下と同等です。

([open-file](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-open_002dfile) ファイル名
(バイナリ「wb」「w」の場合)
#:エンコーディングエンコーディング)

Scheme Procedure: **call-with-input-file** filename proc \[#:guess-encoding=#f\] \[#:encoding=#f\] \[#:binary=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-call_002dwith_002dinput_002dfile)

Scheme Procedure: **call-with-output-file** filename proc \[#:encoding=#f\] \[#:binary=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-call_002dwith_002doutput_002dfile)

入力または出力用にファイル名を開き、結果として得られたポート番号で `(proc port)` を呼び出します。proc が返した値を返します。ファイル名はそれぞれ `open-input-file` または `open-output-file` に従って開かれ、開けない場合はエラーが通知されます。

proc が戻ると、ポートは閉じられます。proc が戻らない場合（例えば、エラーをスローした場合）、ポートは自動的に閉じられない可能性がありますが、他に参照されない限り、通常の方法でガベージコレクションされます。

Scheme Procedure: **with-input-from-file** filename thunk \[#:guess-encoding=#f\] \[#:encoding=#f\] \[#:binary=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-with_002dinput_002dfrom_002dfile)

Scheme Procedure: **with-output-to-file** filename thunk \[#:encoding=#f\] \[#:binary=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-with_002doutput_002dto_002dfile)

Scheme Procedure: **with-error-to-file** filename thunk \[#:encoding=#f\] \[#:binary=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-with_002derror_002dto_002dfile)

filename を開き、新しいポートをそれぞれ `current-input-port`、`current-output-port`、または `current-error-port` として設定して `(thunk)` を呼び出します。thunk によって返された値を返します。filename はそれぞれ `open-input-file` または `open-output-file` に従って開かれ、開けない場合はエラーが通知されます。

thunk が戻ると、ポートは閉じられ、対応する現在のポートの以前の設定が復元されます。

現在のポート設定は `dynamic-wind` で管理されるため、サンクがどのように終了しても (例外など) 以前の値が復元され、サンクが (キャプチャされた継続を介して) 再び開始された場合は、ファイル名ポートに再度設定されます。

ポートは、サンクが正常に戻ったときに閉じられますが、例外や新しい継続によって終了した場合は閉じられません。これにより、キャプチャされた継続によってサンクに再度入った場合でも、ポートが使用可能な状態であることが保証されます。もちろん、ポートはどこからも参照されなくなった時点で、通常の方法でガベージコレクションされ、閉じられます。

スキーム手順: **port-mode** ポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-port_002dmode)

C 関数: **scm\_port\_mode** (ポート) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fport_005fmode)

開いているポートに関連付けられているポートモードを返します。ポート作成時にのみ使用される「append」などのモードは保持されないため、これらのモードはポートが開かれたときに使用されたモードと必ずしも同じではありません。

スキーム手順: **port-filename** port [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-port_002dfilename)

C 関数: **scm\_port\_filename** (ポート) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fport_005ffilename )

ポートに関連付けられたファイル名を返します。ポートに関連付けられたファイル名がない場合は「#f」を返します。

ポートは開いている必要があります。ポートが閉じられると、`port-filename` は使用できません。

Scheme Procedure: **set-port-filename!** port filename [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-set_002dport_002dfilename_0021)

C 関数: **scm\_set\_port\_filename\_x** (ポート、ファイル名) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fset_005fport_005ffilename_005fx)

ポートに関連付けられたファイル名を変更します。入力ポートが指定されていない場合は、現在の入力ポートを使用します。なお、これはポートのデータソースを変更するものではなく、`port-filename` によって返され、診断出力に表示される値のみを変更するものです。

Scheme手順: **file-port?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-file_002dport_003f)

C 関数: **scm\_file\_port\_p** (obj) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffile_005fport_005fp)

objがファイルに関連付けられたポートであるかどうかを判定します。

* * *

次へ: [文字列ポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Ports)、前: [ファイルポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#File-Ports)、上: [ポートの種類](https://doc.guix.gnu.org/guile/latest/en/guile.html#Port-Types) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.12.10.2 Bytevector ポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevector-Ports-1)

スキーム手順: **open-bytevector-input-port** bv \[transcoder\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-open_002dbytevector_002dinput_002dport)

C 関数: **scm\_open\_bytevector\_input\_port** (bv、トランスコーダー) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fopen_005fbytevector_005finput_005fport)

バイトベクトル bv から内容が取得される入力ポートを返します ([バイトベクトル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevectors) を参照)。

トランスコーダー引数は現在サポートされていません。

スキーム手順: **open-bytevector-output-port** \[トランスコーダー\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-open_002dbytevector_002doutput_002dport)

C 関数: **scm\_open\_bytevector\_output\_port** (トランスコーダー) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fopen_005fbytevector_005foutput_005fport)

バイナリ出力ポートとプロシージャの2つの値を返します。後者のプロシージャは、以下の図に示すように、ポートによって蓄積されたデータを含むバイトベクトルを取得するために、引数なしで呼び出す必要があります。

([call-with-values](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-call_002dwith_002dvalues)
(ラムダ()
([open-bytevector-output-port](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-open_002dbytevector_002doutput_002dport)))
(lambda (port get-bytevector)
([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) "hello" ポート)
(get-bytevector)))

⇒ #vu8(104 101 108 108 111)

トランスコーダー引数は現在サポートされていません。

Scheme プロシージャ: **call-with-output-bytevector** proc [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-call_002dwith_002doutput_002dbytevector)

新しく作成したバイトベクター出力ポートを使用して、引数1つのプロシージャprocを呼び出します。関数が戻ると、ポートに書き込まれた文字で構成されるバイトベクターが返されます。procはポートを閉じてはいけません。

Scheme プロシージャ: **call-with-input-bytevector** bytevector proc [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-call_002dwith_002dinput_002dbytevector)

バイトベクターの内容を読み取るための、新しく作成した入力ポートを使用して、引数1つのプロシージャprocを呼び出します。プロシージャによって生成された値が返されます。

* * *

次へ: [カスタムポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Custom-Ports)、前: [バイトベクトルポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevector-Ports)、上: [ポートの種類](https://doc.guix.gnu.org/guile/latest/en/guile.html#Port-Types) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.12.10.3 String Ports [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Ports-1)

Scheme プロシージャ: **call-with-output-string** proc [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-call_002dwith_002doutput_002dstring)

C 関数: **scm\_call\_with\_output\_string** (proc) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fcall_005fwith_005foutput_005fstring)

新しく作成された出力ポートを使用して、引数1つのプロシージャprocを呼び出します。関数が戻ると、ポートに書き込まれた文字で構成される文字列が返されます。procはポートを閉じてはいけません。

Scheme プロシージャ: **call-with-input-string** string proc [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-call_002dwith_002dinput_002dstring)

C 関数: **scm\_call\_with\_input\_string** (string, proc) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fcall_005fwith_005finput_005fstring)

文字列の内容を読み取ることができる、新しく作成された入力ポートを使用して、引数1つのプロシージャprocを呼び出します。procによって生成された値が返されます。

Scheme プロシージャ: **with-output-to-string** thunk [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-with_002doutput_002dto_002dstring)

現在の出力ポートを一時的に新しい文字列ポートに設定した状態で、引数なしのプロシージャサンクを呼び出します。現在の出力に書き込まれた文字で構成される文字列を返します。

Scheme プロシージャ: **with-input-from-string** string thunk [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-with_002dinput_002dfrom_002dstring)

引数なしのプロシージャ thunk を呼び出します。現在の入力ポートは、指定された文字列で開かれた文字列ポートに一時的に設定されます。thunk によって生成された値が返されます。

Scheme手順: **open-input-string** str [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-open_002dinput_002dstring)

C 関数: **scm\_open\_input\_string** (str) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fopen_005finput_005fstring)

文字列を受け取り、その文字列から文字を出力する入力ポートを返します。ポートは`close-input-port`で閉じることができますが、アクセスできなくなった場合はガベージコレクタによってストレージが解放されます。

Scheme手順: **open-output-string** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-open_002doutput_002dstring)

C 関数: **scm\_open\_output\_string** () [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fopen_005foutput_005fstring)

`get-output-string` で取得するための文字を蓄積する出力ポートを返します。このポートは `close-output-port` プロシージャで閉じることができますが、アクセスできなくなった場合はガベージコレクタによってストレージが解放されます。

Scheme Procedure: **get-output-string** port [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-get_002doutput_002dstring)

C 関数: **scm\_get\_output\_string** (ポート) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fget_005foutput_005fstring)

`open-output-string`で作成された出力ポートが与えられた場合、そのポートにこれまでに出力された文字で構成される文字列を返します。

`get-output-string`はポートを閉じる前に使用する必要があります。ポートを閉じると文字列を取得できなくなります。

文字列ポートの場合、ポートエンコーディングは他のタイプのポートとは異なる扱いを受けます。文字列ポートを作成する際、現在のロケールから文字エンコーディングを継承しません。すべての有効な文字列文字を処理できるデフォルトのロケールが割り当てられます。通常、文字列ポートの文字エンコーディングをデフォルトから変更すべきではありません。[エンコーディング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Encoding)を参照してください。

* * *

次へ: [ソフトポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Soft-Ports)、前: [文字列ポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Ports)、上: [ポートの種類](https://doc.guix.gnu.org/guile/latest/en/guile.html#Port-Types) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.12.10.4 カスタムポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Custom-Ports-1)

カスタムポートを使用すると、ユーザーは独自のプロシージャを介して入力と出力を処理できます。最も基本的なカスタムポートはバイトレベルで動作し、ユーザーが提供する関数を呼び出して入力用のバイトを供給し、出力用のバイトを受け取ります。Guileでは、テキストポートはバイナリポートの上に構築され、バイトからコードポイントシーケンスをエンコードおよびデコードします。カスタムポートの上位レベルのテキストレイヤーを使用すると、ユーザーはバイトではなく文字を扱うことができます。

これらの手順を実行する前に、適切なモジュールをインポートしてください。

(use-modules (ice-9 binary-ports))
(use-modules (ice-9 textual-ports))

スキーム手順: **make-custom-binary-input-port** id read! get-position set-position! close [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dcustom_002dbinary_002dinput_002dport)

`read!` 関数を呼び出し、バイトベクトル、バイトを書き込むインデックス、および読み込むバイト数を渡して入力を読み込むことで、id (文字列) という名前の新しいカスタムバイナリ入力ポートを返します。`read!` プロシージャは、読み込んだバイト数を示す整数、またはファイルの終端を示す `0` を返す必要があります。

オプションとして、get-positionが`#f`でない場合、カスタムバイナリポートで`port-position`が呼び出されたときに呼び出されるサンクである必要があり、基となるデータストリーム内の位置を示す整数を返す必要があります。get-positionが指定されていない場合、返されるポートは`port-position`をサポートしていません。

同様に、set-position! が `#f` でない場合は、引数が 1 つのプロシージャである必要があります。カスタムバイナリ入力ポートで `set-port-position!` が呼び出されると、set-position! には、次に読み取るバイトの位置を示す整数が渡されます。

最後に、close が `#f` でない場合は、サンクである必要があります。これは、カスタムバイナリ入力ポートが閉じられたときに呼び出されます。

返されるポートはデフォルトでは完全にバッファリングされていますが、バッファリングモードは`setvbuf`を使用して変更できます（[バッファリング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Buffering)を参照）。

カスタムバイナリ入力ポートを使用すると、`open-bytevector-input-port` プロシージャ ([Bytevector Ports](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevector-Ports) を参照) は次のように実装できます。

(define ([open-bytevector-input-port](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-open_002dbytevector_002dinput_002dport) source)
（位置0を定義する）
(define [length](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-length) ([bytevector-length](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002dlength) source))

(define (read! bv start [count](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-count))
(let (([count](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-count) ([min](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-min) [count](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-count) ([\-](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002d) [length](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-length) position))))
([bytevector-copy!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002dcopy_0021) ソース位置
bv 開始 [カウント](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-count))
([set!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-set_0021) 位置 ([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) 位置 [count](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-count)))
[カウント](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-count)))

(define (get-position) position)

(define (set-position! new-position)
([set!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-set_0021) position new-position))

([make-custom-binary-input-port](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dcustom_002dbinary_002dinput_002dport) "ポート" を読み込みました!
位置を取得、位置を設定！
#f))

([read](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read-1) ([open-bytevector-input-port](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-open_002dbytevector_002dinput_002dport) ([string->utf8](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002d_003eutf8) "hello")))
⇒こんにちは

スキーム手順: **make-custom-binary-output-port** id write! get-position set-position! close [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- make_002dcustom_002dbinary_002doutput_002dport)

`write!` 関数を呼び出し、バイトベクトル、このバイトベクトルからバイトを読み取るインデックス、および「書き込む」バイト数を渡して出力をシンクする、id (文字列) という名前の新しいカスタムバイナリ出力ポートを返します。`write!` プロシージャは、実際に書き込まれたバイト数を示す整数を返す必要があります。書き込むバイト数として `0` が渡された場合、バイトシンクにファイルの終端が送信されたかのように動作する必要があります。

その他の引数は、`make-custom-binary-input-port` と同じです。

スキーム手順: **make-custom-binary-input/output-port** id read! write! get-position set-position! close [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dcustom_002dbinary_002dinput_002foutput_002dport)

id (文字列) という名前の新しいカスタムバイナリ入出力ポートを返します。各種引数は と同じです。その他の引数は `make-custom-binary-input-port` および `make-custom-binary-output-port` と同じです。ポートでバッファリングが有効になっている場合 (デフォルトでは有効です)、入力は双方向でバッファリングされます。[バッファリング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Buffering) を参照してください。set-position! 関数が指定され、`#f` でない場合、ポートはランダムアクセスとしてもマークされ、読み取りと書き込みの間にバッファがフラッシュされます。

Scheme Procedure: **make-custom-textual-input-port** id read! get-position set-position! close [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dcustom_002dtextual_002dinput_002dport)

Scheme Procedure: **make-custom-textual-output-port** id write! get-position set-position! close [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dcustom_002dtextual_002doutput_002dport)

Scheme Procedure: **make-custom-textual-input/output-port** id read! write! get-position set-position! close [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dcustom_002dtextual_002dinput_002foutput_002dport)

カスタムバイナリポートと同様ですが、テキストポート用です。具体的には、読み取り関数にはバイトベクトルではなく、書き込み用の可変文字列が渡され、書き込み用のバッファも同様です。ただし、ポートの位置は依然としてバイト単位で表されます。

Guileに文字列ポートが付属していなかった場合、カスタムテキストポートを使用して実装できます。

(define (open-string-input-port source)
（位置0を定義する）
(長さの定義 (文字列の長さソース))

(define (read! dst 開始カウント)
(let ((count (min count (- length position))))
(文字列コピー! 宛先開始ソース位置 (+ 位置数))
(位置を設定 (+ 位置数))
カウント））

(make-custom-textual-input-port "strport" read! #f #f #f))

(read (open-string-input-port "hello"))

* * *

次へ: [Void Ports](https://doc.guix.gnu.org/guile/latest/en/guile.html#Void-Ports)、前: [Custom Ports](https://doc.guix.gnu.org/guile/latest/en/guile.html#Custom-Ports)、上: [Types of Port](https://doc.guix.gnu.org/guile/latest/en/guile.html#Port-Types) \[[Contents](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[Index](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "Index")\]

#### 6.12.10.5 ソフトポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Soft-Ports-1)

ソフトポートとは、Guileがカスタムバイナリポートやテキストポートを実装する以前に備えていた機能で、カスタマイズ可能なテキスト入出力を可能にするものです。

R6RSのカスタムテキストポートよりもソフトポートの方が使いやすく、表現力も高いため、ソフトポートをお勧めします。R6RSのカスタムテキストポートは、ポートが可変の文字列バッファを持つという原則に基づいて動作し、これはバッファ、オフセット、長さを引数にとる`read`および`write`プロシージャに反映されています。しかし、Guileでは、一部のポートが文字列バッファを持つのではなく、すべてのポートがバイトバッファを持つため、R6RSインターフェースはオーバーヘッドと複雑さを伴います。

さらに、R6RS インターフェースとは異なり、`(ice-9 soft-ports)` モジュールの `make-soft-port` はキーワード引数を受け入れるため、時間の経過とともに機能を拡張することができます。

より多くのパワー、特にシーク機能が必要な場合は、低レベルのカスタムポートを使用することを検討してください。[低レベルのカスタムポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Low_002dLevel-Custom-Ports)を参照してください。

(use-modules (ice-9 soft-ports))

Scheme 手順: **make-soft-port** \[#:id\] \[#:read-string\] \[#:write-string\] \[#:input-waiting?\] \[#:close\] \[#:close-on-gc?\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dsoft_002dport)

新しいポートを返します。read-stringキーワード引数が指定されている場合、そのポートは入力ポートになります。write-stringが指定されている場合、そのポートは出力ポートになります。両方が指定されている場合、そのポートは入出力両方に使用できます。

ポートの内部バッファが空の場合、read-string は引数なしで呼び出され、文字列を返すか、ストリームの終了を示す `#f` を返します。同様に、ポートが書き込みバッファをフラッシュすると、そのバッファ内の文字が唯一の引数として write-string プロシージャに渡されます。write-string は未指定の値を返します。

指定された場合、ソフトポートにread-stringによって直接返される入力がある場合は、input-waiting?は`#t`を返す必要があります。

指定されている場合、ポートが閉じられたときに、引数なしで close が呼び出されます。close-on-gc? が `#t` の場合、保留中の書き込みバッファをすべてフラッシュした後、ポートに到達できなくなったときにも close が呼び出されます。

ソフトポートを使用すると、前のセクションの`open-string-input-port`の例はよりシンプルになります。

(define (open-string-input-port source)
(既に読んだかどうかを定義する? #f)

(define (read-string)
（条件）
（既に読んだ？ "")
（それ以外
(設定済み! 既読? #t)
ソース）））

(make-soft-port #:id "strport" #:read-string read-string))

なお、Guileのデフォルト環境には、以前のバージョンの`make-soft-port`が存在し、現在も残っています。そのインターフェースはやや使いにくく、ユーザーは従来、バッファリングされていない入力を想定していました。このインターフェースは今後非推奨となりますが、ここではその概要を説明します。

スキーム手順: **deprecated-make-soft-port** pv モード [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-deprecated_002dmake_002dsoft_002dport)

モード文字列で指定された文字を受信または送信できるポートを返します（[open-file](https://doc.guix.gnu.org/guile/latest/en/guile.html#File-Ports)を参照）。pvは長さ5または6のベクトルでなければなりません。その構成要素は次のとおりです。

0. 出力用に1文字を受け取る手続き
1. 文字列を出力として受け取る手続き
2. 出力のフラッシュのためのサンク
3. 1文字取得のために考える
4. ポートを閉じるためのサンク（ガベージコレクションによるものではない）
5. (存在し、かつ `#f` でない場合) ポートからブロックせずに読み取ることができる文字数を計算するためのサンク。

出力専用ポートの場合、要素0、1、2、および4のみがプロシージャである必要があります。入力専用ポートの場合、要素3および4のみがプロシージャである必要があります。サンク2および4は、実行すべき有用な操作がない場合は、代わりに`#f`にすることができます。

thunk 3 が `#f` または `eof-object` (「The Revised^5 Report on Scheme」の [eof-object?](https://doc.guix.gnu.org/r5rs/latest/en/r5rs.html#Input) を参照) を返す場合、ポートがファイルの終端に達したことを示します。例:

(define stdout ([current-output-port](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-current_002doutput_002dport)))
(define p ([deprecated-make-soft-port](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-deprecated_002dmake_002dsoft_002dport)
([vector](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-vector)
(lambda (c) ([write](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-write) c stdout))
(lambda (s) ([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) s stdout))
(lambda () ([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) "." stdout))
(lambda () ([char-upcase](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dupcase) ([read-char](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002dchar))))
(lambda () ([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) "@" stdout)))
"rw"))

([write](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-write) pp) ⇒ #<input-output: soft 8081e20>

* * *

次へ: [低レベルカスタムポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Low_002dLevel-Custom-Ports )、前: [ソフトポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Soft-Ports)、上: [ポートの種類](https://doc.guix.gnu.org/guile/latest/en/guile.html#Port-Types) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.12.10.6 ボイドポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Void-Ports-1)

この種のポートは、書き込み時にデータをすべて破棄し、読み取り時には常にファイル終端オブジェクトを返します。

Scheme手順: **%make-void-port**モード [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0025make_002dvoid_002dport)

C 関数: **scm\_sys\_make\_void\_port** (モード) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsys_005fmake_005fvoid_005fport)

新しいvoidポートを作成して返します。voidポートは/dev/nullのように動作します。mode引数は、このポートの入出力モードを指定します。詳細については、[ファイルポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#File-Ports)の`open-file`のドキュメントを参照してください。

* * *

次へ: [C 言語の低レベルカスタムポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Low_002dLevel-Custom-Ports-in-C)、前: [ボイドポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Void-Ports)、上: [ポートの種類](https://doc.guix.gnu.org/guile/latest/en/guile.html#Port-Types) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.12.10.7 低レベルカスタムポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Low_002dLevel-Custom-Ports-1)

このセクションでは、Guile の最も低レベルで基本的なインターフェースを使用して新しいタイプのポートを実装する方法について説明します。まず、`(ice-9 custom-ports)` モジュールをロードします。

(use-modules (ice-9 custom-ports))

次に、新しいポートを作成するには、`make-custom-port` を呼び出します。

Scheme Procedure: **make-custom-port** \[#:read\] \[#:write\] \[#:read-wait-fd\] \[#:write-wait-fd\] \[#:input-waiting?\] \[#:seek\] \[#:random-access?\] \[#:get-natural-buffer-sizes\] \[#:id\] \[#:print\] \[#:close\] \[#:close-on-gc?\] \[#:truncate\] \[#:encoding\] \[#:conversion-strategy\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dcustom_002dport)

新しいカスタムポートを作成します。

`#:encoding` および `#:conversion-strategy` の詳細については、[Encoding](https://doc.guix.gnu.org/guile/latest/en/guile.html#Encoding) を参照してください。

ポートには、その動作を実装する一連の関連プロシージャとプロパティがあります。新しいカスタムポートを作成するには、主にこれらのプロシージャを記述し、それをキーワード引数として`make-custom-port`に渡します。

スキームポートメソッド: **#:read** port dst start count [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0023_003aread)

ポートの`#:read`実装は、読み取りバッファを埋めます。指定されたバイトベクトルdstに、オフセットstartからcountバイト分をコピーし、読み取ったバイト数を返すか、または、バイトの読み取りがブロックされることを示す`#f`を返します。

Scheme Port メソッド: **#:write** port src start count [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0023_003awrite)

ポートの`#:write`実装は、書き込みバッファを可変ストアにフラッシュします。指定されたバイトベクターsrcからオフセットstartからcountバイト分を書き込み、書き込まれたバイト数を返すか、書き込み処理がブロックされることを示すために`#f`を返す必要があります。

`make-custom-port`に`#:read`引数を渡すと、ポートは入力ポートになります。`#:write`引数を渡すと出力ポートになり、両方を渡すと入出力ポートになります。

Scheme ポート方式: **#:read-wait-fd** ポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0023_003aread_002dwait_002dfd)

Scheme ポート方法: **#:write-wait-fd** ポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0023_003awrite_002dwait_002dfd)

ポートの `#:read` メソッドまたは `#:write` メソッドが `#f` を返す場合、それは読み取りまたは書き込みがブロックされることを示しており、Guile は代わりに、操作が完了するまで、ポートの `#:read-wait-fd` メソッドまたは `#:write-wait-fd` メソッドによって返されるファイルディスクリプタに対して `poll` を実行する必要があります。詳細については、[Non-Blocking I/O](https://doc.guix.gnu.org/guile/latest/en/guile.html#Non_002dBlocking-I_002fO) を参照してください。

これらのメソッドは、`#:read` メソッドまたは `#:write` メソッドが `#f` を返すことができる場合に実装する必要があり、負でない整数のファイルディスクリプタを返す必要があります。ただし、ポートが最終的に読み取り可能または書き込み可能になるかどうかを判断するために、ユーザーが明示的に呼び出すこともできます。ポートに関連付けられたファイルディスクリプタがない場合は、`#f` を返す必要があります。デフォルトの実装では `#f` を返します。

Scheme ポートメソッド: **#:input-waiting?** ポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0023_003ainput_002dwaiting_003f)

まれに、ポートからデータを読み取れるかどうかを知ることが役立つ場合があります。たとえば、ユーザーが対話型コンソールで「1 2 3」と入力した場合、コンソールは「1」を読み込んで評価した後、「2」を読み込んで評価する前に別のプロンプトを表示すべきではありません。これは、既に入力が待機しているためです。ポートが先読みできる場合は、入力が利用可能な場合は「#t」を返し、次のバイトの読み取りがブロックされる場合は「#f」を返す「#:input-waiting?」メソッドを実装する必要があります。デフォルトの実装では「#t」が返されます。

Scheme Port メソッド: **#:seek** ポートオフセット whence [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0023_003aseek)

ポートの現在のバイト位置を設定または取得します。Guile は、必要に応じて、シークの前に読み取りバッファや書き込みバッファをフラッシュします。オフセットと whence パラメータは、`seek` プロシージャと同じです。[ランダムアクセス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Random-Access) を参照してください。

`#:seek` メソッドは、シーク後のバイト位置を返します。現在の位置を照会するには、オフセットを 0、whence を `SEEK_CUR` として `#:seek` メソッドを呼び出します。オフセットやwhence に他の値を指定すると、実際にシークが実行されます。ポートがシーク可能でない場合は、`#:seek` メソッドはエラーをスローする必要があります。これは、デフォルトの実装で行われている動作です。

Scheme ポート方法: **#:truncate** ポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0023_003atruncate)

ポートデータを指定された長さに切り詰めます。Guileは必要に応じて事前にバッファをフラッシュします。デフォルトの実装では、このポートでは切り詰めがサポートされていないことを示すエラーが発生します。

Scheme ポート方式: **#:random-access?** ポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0023_003arandom_002daccess_003f)

ポートがランダムアクセス用に開いている場合は`#t`を返し、そうでない場合は`#f`を返します。

バッファリングされた入力を持つランダムアクセスポートでシークを実行したり、読み取り後に書き込みに切り替えたりすると、バッファリングされた入力は破棄され、Guile はポートをバッファリングされたバイト数だけシークします。同様に、バッファリングされた出力を持つランダムアクセスポートでシークを実行したり、書き込み後に読み取りに切り替えたりすると、保留中のバイトが `write` プロシージャの呼び出しによってフラッシュされます。[バッファリング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Buffering) を参照してください。

`#:random-access?` メソッドから true を返すことで、ポートにこの動作が必要であることを Guile に示します。この関数のデフォルトの実装では、ポートに `#:seek` 実装がある場合、`#t` が返されます。

Scheme ポートメソッド: **#:get-natural-buffer-sizes** read-buf-size write-buf-size [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0023_003aget_002dnatural_002dbuffer_002dsizes)

Guile は内部的にポートにバッファをアタッチします。入力ポートには必ず読み取りバッファがあり、出力ポートには必ず書き込みバッファがあります。[バッファリング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Buffering) を参照してください。ポートバッファは、バイトベクターと、そのバイトベクター内のデータの取得場所と書き込み場所を示すカーソルで構成されます。

ポートの実装では、通常バッファリングについて考慮する必要はありません。ポートの `#:read` メソッドまたは `#:write` メソッドは、バッファのバイトベクトルを引数として受け取り、そのバイトベクトル内のオフセットと長さも受け取ります。そして、そのバイトベクトルを埋めるか空にします。ただし、場合によっては、ポートの実装が Guile に適切なデフォルトのバッファサイズを提供できることがあります。たとえば、ファイルポートは `#:get-natural-buffer-sizes` を実装して、オペレーティングシステムがポートによって開かれた特定のファイルに適したバッファサイズを Guile に通知できるようにします。

このメソッドは、ポートの自然な読み取りバッファサイズと書き込みバッファサイズに対応する2つの値を返します。2つのパラメータread-buf-sizeとwrite-buf-sizeは、Guileが適切なサイズとして推測したものです。カスタムの`#:get-natural-buffer-sizes`メソッドを使用すれば、Guileの選択を上書きすることも、デフォルトの実装のようにそのまま渡すこともできます。

Scheme Port メソッド: **#:print** port out [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0023_003aprint)

ポートが出力に書き込まれるとき（例：`(write port out)` 経由）に呼び出されます。

`#:print` が明示的に指定されていない場合、デフォルトの実装では `#<mode:id address>` のようなものが出力されます。ここで、mode は `input`、`output`、または `input-output` のいずれかであり、id は `#:id` キーワード引数から取得され (デフォルトは `"custom-port"`)、address はポートに関連付けられた一意の整数です。

Scheme ポート方法: **#:close** port [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0023_003aclose)

ポートが閉じられたときに呼び出されます。ポートが使用する明示的に管理されたリソースを解放する必要があります。

デフォルトでは、ガベージコレクションされるポートは、バッファリングされた出力を閉じたりフラッシュしたりすることなく、そのまま終了します。ポートがファイルディスクリプタなどの外部リソースを解放する必要がある場合、またはポートが開いている間にガベージコレクションされた場合でも内部バッファがフラッシュされるようにする必要がある場合は、`make-custom-port` に `#:close-on-gc? #t` を渡してください。この場合、`#:close` メソッドは別のスレッドで呼び出される可能性があることに注意してください。

これらのメソッドへの呼び出しはすべて、ポートが閉じられるまで、どのスレッドからでも並列かつ同時に実行できることに注意してください。`close` メソッドは、他のメソッドが実行されていないときに呼び出され、`close` メソッドが呼び出された後は、他のメソッドは呼び出されません。ポートの実装で同時実行を防ぐために相互排他が必要な場合は、適切なロック処理を行う責任があります。

* * *

前へ: [低レベルカスタムポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Low_002dLevel-Custom-Ports)、上へ: [ポートの種類](https://doc.guix.gnu.org/guile/latest/en/guile.html#Port-Types) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.12.10.8 C言語における低レベルカスタムポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Low_002dLevel-Custom-Ports-in-C-1)

前のセクションで説明した `make-custom-port` 手順は、C レベルでは同様の機能を持っていますが、構成が少し異なります。

C言語では、新しいポート型オブジェクトを作成することでこの仕組みが実現します。メソッドはポート自体ではなく、このポート型オブジェクトに関連付けられます。ポート型オブジェクトは、ポート型を定義する際に割り当てられる不透明なポインタであり、ポートAPIへのキーとして機能します。

ポート自体には、関連付けられた _stream_ 値があります。ストリームはユーザーが制御するポインタであり、ポートの作成時に設定されます。`SCM_STREAM` マクロは、ポートを指定すると、関連付けられたストリーム値を `scm_t_bits` として返します。ポート メソッドは常に自分の型のポートで呼び出されるため、ポート メソッドはこの値を期待される型に安全にキャストできます。これに対し、Scheme では `make-custom-port` メソッドはポート固有のデータを直接共有するクロージャにできるため、ストリームへのアクセスは必要ありません。

ポートタイプは、`scm_make_port_type` を呼び出すことによって作成されます。

関数: `scm_t_port_type*` **scm\_make\_port\_type** `(char *name, size_t (*read) (SCM port, SCM dst, size_t start, size_t count), size_t (*write) (SCM port, SCM src, size_t start, size_t count))` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fmake_005fport_005ftype)

新しいポートタイプを定義します。name パラメータは `make-custom-port` の `#:id` パラメータに似ています。read と write は `make-custom-port` の `#:read` と `#:write` に似ていますが、read または write 操作がブロックする場合は `#f` ではなく `(size_t)-1` を返す必要があります。

関数: `void` **scm\_set\_port\_read\_wait\_fd** `(scm_t_port_type *type, int (*wait_fd) (SCM port))` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fset_005fport_005fread_005fwait_005ffd)

関数: `void` **scm\_set\_port\_write\_wait\_fd** `(scm_t_port_type *type, int (*wait_fd) (SCM port))` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fset_005fport_005fwrite_005fwait_005ffd)

関数: `void` **scm\_set\_port\_print** `(scm_t_port_type *type, int (*print) (SCM port, SCM dest_port, scm_print_state *pstate))` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fset_005fport_005fprint)

関数: `void` **scm\_set\_port\_close** `(scm_t_port_type *type, void (*close) (SCMポート))` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fset_005fport_005fclose)

関数: `void` **scm\_set\_port\_needs\_close\_on\_gc** `(scm_t_port_type *type, int needs_close_p)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fset_005fport_005fneeds_005fclose_005fon_005fgc)

関数: `void` **scm\_set\_port\_seek** `(scm_t_port_type *type, scm_t_off (*seek) (SCMポート、scm_t_offオフセット、int whence))` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fset_005fport_005fseek)

関数: `void` **scm\_set\_port\_truncate** `(scm_t_port_type *type, void (*truncate) (SCM port, scm_t_off length))` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fset_005fport_005ftruncate)

関数: `void` **scm\_set\_port\_random\_access\_p** `(scm_t_port_type *type, int (*random_access_p) (SCM port));` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fset_005fport_005frandom_005faccess_005fp)

関数: `void` **scm\_set\_port\_input\_waiting** `(scm_t_port_type *type, int (*input_waiting) (SCM port));` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fset_005fport_005finput_005fwaiting)

関数: `void` **scm\_set\_port\_get\_natural\_buffer\_sizes** `(scm_t_port_type *type, void (*get_natural_buffer_sizes) (SCM, size_t *read_buf_size, size_t *write_buf_size))` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fset_005fport_005fget_005fnatural_005fbuffer_005fsizes)

各メソッドの詳細については、 [低レベルカスタムポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Low_002dLevel-Custom-Ports)を参照してください。

ポートの種類が決まったら、`scm_c_make_port` または `scm_c_make_port_with_encoding` を使用してポートを作成できます。

関数: `SCM` **scm\_c\_make\_port\_with\_encoding** `(scm_t_port_type *type, unsigned long mode_bits, SCM encoding, SCM conversion_strategy, scm_t_bits stream)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fmake_005fport_005fwith_005fencoding)

関数: `SCM` **scm\_c\_make\_port** `(scm_t_port_type *type, unsigned long mode_bits, scm_t_bits stream)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fmake_005fport)

指定されたタイプのポートを作成します。ストリームは、ポートに関連付けられたプライベートデータを示し、ポートの実装では後で `SCM_STREAM` を使用して取得できます。モード ビットには、ポートが入力ポートまたは出力ポートであることを示すフラグ `SCM_RDNG` または `SCM_WRTNG` のいずれか、または複数を含める必要があります。モード ビットには、ポートがバッファリングされないかライン バッファリングされるかを示す `SCM_BUF0` または `SCM_BUFLINE` を含めることもできます。デフォルトでは、ポートはブロック バッファリングされます。[バッファリング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Buffering) を参照してください。

ご想像のとおり、encodingとconversion_strategyは、ポートの初期テキストエンコードと変換戦略を指定します。どちらもシンボルです。`scm_c_make_port`は、デフォルトのポートエンコードと変換戦略を使用する点を除いて、`scm_c_make_port_with_encoding`と同じです。

この時点で、カスタムポートタイプをC言語で実装するかSchemeで実装するか迷うかもしれません。おそらくSchemeの`make-custom-port`を使うのが良いでしょう。C言語とSchemeの速度はほぼ同じで、C言語で実装されたポートはサスペンドできないという欠点があります。[非ブロッキングI/O](https://doc.guix.gnu.org/guile/latest/en/guile.html#Non_002dBlocking-I_002fO)を参照してください。

* * *

次へ: [C言語からのポートの使用](https://doc.guix.gnu.org/guile/latest/en/guile.html#Using-Ports-from-C)、前: [ポートの種類](https://doc.guix.gnu.org/guile/latest/en/guile.html#Port-Types)、上: [入力と出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Input-and-Output) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.12.11 ヴェネラブルポートインターフェース [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Venerable-Port-Interfaces-1)

Guileが登場してから約25年の間に、そのポートシステムは進化を遂げ、多くの便利な機能が追加されました。同時に、この25年の間に4つの主要なScheme標準がリリースされ、ポートインターフェースのあるべき姿に関するSchemeの共通認識も進化しました。しかし、これらの進化の枝すべてが一貫していることを期待するのは無理があります。Guileの初期インターフェースの中には、後のScheme標準と互換性のないものもありますが、Guileは古いインターフェースを単純に削除することはできません。残念なことに、R6RSとR7RSの標準はどちらもR5RSをベースとしていますが、最終的には異なった、やや互換性のない設計になっています。

Guile のアプローチは、互いに意味のある一連のポート プリミティブを選択することです。私たちはそのプリミティブのセットを文書化し、それらに基づいて内部インターフェースを設計し、ユーザーに推奨します。R6RS I/O システムは、この分野で Scheme がこれまでに作成した中で最も優れた標準であるため、私たちは主に、`(ice-9 binary-ports)` と `(ice-9 textual-ports)` は完全に `(rnrs io ports)` をモデルにしていることを推奨します。ただし、Guile は R6RS を完全にコピーしているわけではありません。[R6RS との非互換性](https://doc.guix.gnu.org/guile/latest/en/guile.html#R6RS-Incompatibilities) を参照してください。

同時に、ハッカーの先人たちから受け継がれてきた、由緒あるポートインターフェースも数多く存在します。これらのインターフェースのほとんどは、Schemeにモジュールが必要とされるようになる以前から存在していたため、デフォルトの環境に組み込まれています。Guileでもこれらのインターフェースをサポートしており、削除する予定はありませんが、新規ユーザーにはお勧めしません。

Scheme手順: **char-ready?** \[port\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-char_002dready_003f)

入力ポートに文字が準備できている場合は `#t` を返し、そうでない場合は `#f` を返します。`char-ready?` が `#t` を返す場合、そのポートでの次の `read-char` 操作はハングアップしないことが保証されます。ポートがファイルの末尾にあるファイルポートの場合、`char-ready?` は `#t` を返します。

`char-ready?` は、プログラムが入力待ちで停止することなく、対話型ポートから文字を受け取れるようにするために存在します。このようなポートに関連付けられた入力エディタは、`char-ready?` によって存在が確認された文字が消去されないようにする必要があります。`char-ready?` がファイルの末尾で `#f` を返す場合、ファイルの末尾にあるポートは、準備完了文字がない対話型ポートと区別がつかなくなります。

`char-ready?` は、1 バイト エンコーディングの端末とソケットでのみ確実に動作することに注意してください。内部的には、ポートにバッファリングされた入力がある場合、またはポートの背後にあるファイル ディスクリプタが読み取り可能としてポーリングされている場合、Guile がカーネルからさらにバイトを取得できることを示す `#t` を返します。ただし、1 バイトを取得できるからといって、完全な文字が利用可能であるとは限りません。[エンコーディング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Encoding) を参照してください。また、多くのシステムでは、ファイル ディスクリプタが読み取り可能としてポーリングされていても、バイトを読み取るときにブロックされる可能性があります。Linux カーネルでは、ファイルによってバックアップされているすべてのファイル ポートは常に読み取り可能としてポーリングされることにも注意してください。ファイル以外のポートの場合、この手順は常に `#t` を返しますが、`char-ready?` ハンドラを持つソフト ポートは例外です。 [Soft Ports](https://doc.guix.gnu.org/guile/latest/en/guile.html#Soft-Ports)を参照してください。

要するに、これは意味論的な説明が難しいレガシーな手順です。しかし、入力がバッファリングされているかどうかを確認するのに役立ちます。[非ブロッキングI/O](https://doc.guix.gnu.org/guile/latest/en/guile.html#Non_002dBlocking-I_002fO)を参照してください。

Scheme手順: **read-char** \[port\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002dchar)

`get-char` と同じですが、ポートはデフォルトで現在の入力ポートになります。[テキスト入出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Textual-I_002fO) を参照してください。

Scheme Procedure: **peek-char** \[port\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-peek_002dchar)

`lookahead-char` と同じですが、ポートはデフォルトで現在の入力ポートになります。[テキスト入出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Textual-I_002fO) を参照してください。

Scheme手順: **unread-char** cobj \[port\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-unread_002dchar)

`unget-char` と同じですが、port のデフォルト値が現在の入力ポートになり、引数が入れ替わります。[テキスト入出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Textual-I_002fO) を参照してください。

Scheme手順: **unread-string** str \[port\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-unread_002dstring)

C 関数: **scm\_unread\_string** (str, port) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005funread_005fstring)

`unget-string` と同じですが、port のデフォルト値が現在の入力ポートになり、引数が入れ替わります。[テキスト入出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Textual-I_002fO) を参照してください。

Scheme Procedure: **newline** \[port\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-newline)

指定されたポートに改行文字を送信します。ポートを省略した場合は、現在の出力ポートに送信します。`(put-char port #\newline)` と同等です。

Scheme手順: **write-char** chr \[port\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-write_002dchar)

`put-char` と同じですが、port はデフォルトで現在の入力ポートになり、引数が入れ替わります。[テキスト入出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Textual-I_002fO) を参照してください。

* * *

次へ: [ノンブロッキング I/O](https://doc.guix.gnu.org/guile/latest/en/guile.html#Non_002dBlocking-I_002fO)、前: [Venerable Port Interfaces](https://doc.guix.gnu.org/guile/latest/en/guile.html#Venerable-Port-Interfaces)、上: [入力と出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Input-and-Output) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.12.12 C言語からのポートの使用 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Using-Ports-from-C-1)

GuileのCインターフェースは、バイトや文字の送受信をC言語との連携をより良くするための便利な機能を提供します。

C 関数: `size_t` **scm\_c\_read** `(SCM ポート、void *buffer、size_t サイズ)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fread)

ポートから指定されたサイズバイトまで読み込み、バッファに格納します。戻り値は実際に読み込まれたバイト数です。ファイルの終端に達した場合は、サイズよりも少ない値になることがあります。

これはバイナリ入力手順であるため、この関数は`port-line`と`port-column`を更新しないことに注意してください（[Textual I/O](https://doc.guix.gnu.org/guile/latest/en/guile.html#Textual-I_002fO)を参照）。

C 関数: `void` **scm\_c\_write** `(SCM ポート、const void *buffer、size_t サイズ)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fwrite)

バッファ内の指定されたバイト数をポートに書き込みます。

これはバイナリ出力手順であるため、この関数は`port-line`と`port-column`を更新しないことに注意してください（[Textual I/O](https://doc.guix.gnu.org/guile/latest/en/guile.html#Textual-I_002fO)を参照）。

C 関数: `size_t` **scm\_c\_read\_bytes** `(SCM ポート、SCM bv、size_t 開始サイズ、size_t カウント)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fread_005fbytes)

C 関数: `void` **scm\_c\_write\_bytes** `(SCM port, SCM bv, size_t start, size_t count)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fwrite_005fbytes)

`scm_c_read` や `scm_c_write` と同様ですが、バイトベクター bv への読み書きを行います。count は、バイトベクター内で開始するバイトインデックスを示し、読み書きは count バイト分続きます。

C 関数: `void` **scm\_unget\_bytes** `(const unsigned char *buf, size_t len, SCM port)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005funget_005fbytes)

C 関数: `void` **scm\_unget\_byte** `(int c, SCM port)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005funget_005fbyte)

C 関数: `void` **scm\_ungetc** `(scm_t_wchar c, SCM port)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fungetc)

それぞれ「unget-bytevector」、「unget-byte」、「unget-char」と同様です。 [テキスト I/O](https://doc.guix.gnu.org/guile/latest/en/guile.html#Textual-I_002fO) を参照してください。

C 関数: `void` **scm\_c\_put\_latin1\_chars** `(SCM ポート、const scm_t_uint8 *buf、size_t len)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fput_005flatin1_005fchars)

C 関数: `void` **scm\_c\_put\_utf32\_chars** `(SCM port, const scm_t_uint32 *buf, size_t len);` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fput_005futf32_005fchars)

ポートに文字列を書き込みます。最初のケースでは、`scm_t_uint8*` バッファはlatin-1エンコーディングの文字列です。2番目のケースでは、`scm_t_uint32*` バッファはUTF-32エンコーディングの文字列です。これらのルーチンは、ポートの行と列を更新します。

* * *

次へ: [Unicodeバイトオーダーマークの処理](https://doc.guix.gnu.org/guile/latest/en/guile.html#BOM-Handling)、前: [C言語からのポートの使用](https://doc.guix.gnu.org/guile/latest/en/guile.html#Using-Ports-from-C)、上: [入力と出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Input-and-Output) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.12.13 ノンブロッキング I/O [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Non_002dBlocking-I_002fO-1)

Guileのほとんどのポートはブロッキング方式です。ポートから文字を読み取ろうとすると、文字が準備できるまで、またはストリームの終端が検出されるまで、Guileは読み取り処理をブロックします。同様に、Guileが（バッファリングされた可能性のある）データを出力ポートに書き込む場合も、すべてのデータが書き込まれるまでブロックします。

ブロッキングモードでポートを操作するのは非常に便利です。データの流れを反映したコードの流れを持つ、シンプルで逐次的なアルゴリズムを作成できます。ただし、ブロッキングI/Oには主に2つの制限があります。

まず、コードがデータ待ち状態に陥りやすいという問題があります。コードが他の処理を実行できるはずの時間をデータ待ちに費やすのは無駄であり、プログラムの処理能力を最大限に引き出すことを妨げます。クライアントからのリクエストを順次処理するWebサーバーを実装する場合、サーバーがクライアントのHTTPリクエストの完了やレスポンスの受信を待つ状態に陥りやすくなります。その結果、1秒あたりに処理できるリクエスト数が、本来処理したい数よりも少なくなってしまうのです。

2つ目の制限はこれに関連しています。ユーザーが制御する入力に対してブロッキングパーサーを使用すると、サービス拒否攻撃の脆弱性が生じます。実際、2010年代初頭に流行したいわゆる「スローロリス攻撃」はまさにその典型例です。一般的なWebサーバーに対して、HTTPリクエストを1文字ずつ少しずつ送信する攻撃でした。ほんの数件のスローロリス接続だけで、Webサーバー全体を占有してしまうことができたのです。

Guileでは、あらゆる種類のブロッキング型ネットワークプロセスを記述できる機能を維持しつつ、内部的には、それらのプロセスがブロックする可能性のあるリクエストを一時停止できるようにしたいと考えています。

これを実現するには、まずGuileポートが自身を非ブロッキングポートとして宣言できるようにする必要があります。これは現在、ファイルポートのみでサポートされており、ソケット、端末、またはファイルディスクリプタによってサポートされるその他のポートも含まれます。そのためには、難解なUNIXコマンドを使用します。

(let ((flags (fcntl socket F\_GETFL)))
(fcntl socket F\_SETFL (logior O\_NONBLOCK flags)))

これでファイルディスクリプタは非ブロッキングモードで開かれます。Guile がこのファイルから読み書きしようとして、ブロッキング読み書きを行わないとデータを取得できないことを示す結果が返された場合、Guile はソケットの `read-wait-fd` または `write-wait-fd` をポーリングしてブロックし、ブロッキング読み書きの錯覚を維持します。これらの内部インターフェイスの詳細については、[低レベルカスタムポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Low_002dLevel-Custom-Ports) を参照してください。

これまでのところ、現状を再現しただけです。ファイルディスクリプタは非ブロッキングですが、ポートに対する操作はブロッキングします。さらに進むには、区切り継続を使用して「スレッド」を一時停止し、ファイルディスクリプタが読み取り可能または書き込み可能になったときにのみスレッドを再開できると良いでしょう。（[プロンプト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Prompts)を参照）。

しかし、ここで問題が生じます。ポートコードはC言語で実装されているため、計算を外部プロンプトに中断することはできますが、GuileはCスタックをキャプチャする区切り継続を再開できないため、計算を再開することはできません。

この問題を解決するために、互換性がありながら完全に並列処理が可能なポート操作の実装を作成しました。この実装を使用するには、以下の手順に従ってください。

(use-modules (ice-9 suspendable-ports))
(install-suspendable-ports!)

これにより、`get-char` や `put-bytevector` などのコア I/O プリミティブが、標準ライブラリのものとまったく同じですが、2 つの違いがある新しいバージョンに置き換えられます。 1 つは、読み取りまたは書き込みがブロックする場合、サスペンド可能なポート操作が、必要に応じて `current-read-waiter` または `current-write-waiter` パラメータの値を呼び出すことです。 [パラメータ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Parameters) を参照してください。 デフォルトの読み取りおよび書き込みウェイターは、C の読み取りおよび書き込みウェイターと同じこと、つまりポーリングを行います。ただし、ユーザー コードでウェイターをパラメータ化することで、計算をサスペンドして、プログラムが他の I/O 操作を処理できるようにします。 新しいサスペンド可能なポートの実装は Scheme で記述されているため、サスペンドされた計算は、後で進捗が可能になったときに再開できます。 成功です。

もう一つの大きな違いは、新しいポート実装がSchemeで書かれているため、C言語よりも遅いことです。現状では3～4倍遅いですが、これは多くの要因によって異なります。そのため、C言語による実装をデフォルトとして維持する必要があります。Guileのコンパイラが改善されれば、この差を縮め、ポート操作の実装を再び1つに絞り込むことができるでしょう。

Guileには現在、現在のスレッドを一時停止し、その間に他のスレッドをスケジュールする機能は実装されていません。このような機能を追加する前に、スケジューラやその他のユーザー空間並行処理パターンを構築するために使用できる適切なプリミティブを提供していること、そして採用するパターンが適切なものであることを確認したいと考えています。それまでの間、非同期I/Oと並行処理機能のプロトタイプとして、8sync（[https://gnu.org/software/8sync](https://gnu.org/software/8sync)）を参照してください。

スキーム手順: **install-suspendable-ports!** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-install_002dsuspendable_002dports_0021)

上記の説明に従って、コアポートの実装をサスペンド可能なポートに置き換えてください。これにより、`get-char`、`put-u8`などのバインディングの値がインプレースで変更されます。

スキーム手順: **uninstall-suspendable-ports!** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-uninstall_002dsuspendable_002dports_0021)

`install-suspendable-ports!` の効果を元に戻し、元のコアポートの実装を復元します。

スキームパラメータ: **current-read-waiter** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-current_002dread_002dwaiter)

スキームパラメータ: **current-write-waiter** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-current_002dwrite_002dwaiter)

値が引数1つのプロシージャであるパラメータは、サスペンド可能なポート操作が読み取りまたは書き込み中にポート上でブロックする場合に呼び出されます。これらのパラメータのデフォルト値は、ポートのファイルディスクリプタに対してブロッキング`poll`を実行します。プロシージャには、対象のポートが引数として渡されます。

* * *

前へ: [ノンブロッキングI/O](https://doc.guix.gnu.org/guile/latest/en/guile.html#Non_002dBlocking-I_002fO)、上へ: [入力と出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Input-and-Output) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.12.14 Unicodeバイトオーダーマークの処理 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Handling-of-Unicode-Byte-Order-Marks)

このセクションでは、Guile における Unicode バイトオーダーマーク (BOM) の処理に関する詳細について説明します。バイトオーダーマーク (U+FEFF) は通常、UTF-16 または UTF-32 ストリームの先頭に配置され、読み取り側がバイト順序を確実に判断できるようにします。まれに、UTF-8 ストリームの先頭に BOM が配置されることもありますが、これは非常にまれであり、一般的には推奨されません。

Guileは、ポートのエンコーディングが`UTF-8`、`UTF-16`、または`UTF-32`に設定されている場合、Unicode標準の推奨事項に従ってBOMを自動的に処理しようとします。簡単に言うと、GuileはUTF-16またはUTF-32ストリームの先頭にBOMを自動的に書き込み、UTF-8、UTF-16、またはUTF-32ストリームの先頭からBOMを自動的に消費します。

Unicode標準で規定されているとおり、BOMはストリームの開始時にのみ特別に処理され、ポートエンコーディングが`UTF-8`、`UTF-16`、または`UTF-32`に設定されている場合に限ります。ポートエンコーディングが`UTF-16BE`、`UTF-16LE`、`UTF-32BE`、または`UTF-32LE`に設定されている場合は、BOMは特別に処理されず、このセクションで説明されている特別な処理は一切適用されません。

* Guile が UTF-16 または UTF-32 ストリームのバイト順序を正しく検出できるようにするには、書き込み、シーク、またはバイナリ I/O の前にテキスト読み取りを実行する必要があります。ストリームの開始時に明示的に読み取りが要求されない限り、Guile は BOM の読み取りを試みません。
* 最初の読み取りの前にテキスト書き込みが実行された場合、任意のバイト順序が選択されます。現在、すべてのプラットフォームでビッグエンディアンがデフォルトですが、将来変更される可能性があります。出力ストリームのバイト順序を明示的に制御する場合は、ポートエンコーディングを`UTF-16BE`、`UTF-16LE`、`UTF-32BE`、または`UTF-32LE`に設定し、必要に応じてBOM（`#\xFEFF`）を明示的に書き込みます。
* ストリームの途中で `set-port-encoding!` が呼び出された場合、Guile はこれを BOM 処理の目的で新しい論理的な「ストリームの開始」として扱い、以前に検出された BOM をすべて無視します。そのため、以前に使用されていたものとは異なるバイト順序が選択される場合があります。これは、より大きなバイナリ ストリーム内に埋め込まれた複数の論理テキスト ストリームをサポートすることを目的としています。
* バイナリI/O操作は、ポートが「ストリームの開始位置」にあるかどうかというGuileの概念を更新することを保証せず、またBOMを生成または消費することも保証しません。
* シークをサポートするポート（通常のファイルなど）の場合、入力ストリームと出力ストリームはリンクされているとみなされます。ユーザーが最初に読み取った場合、BOMが消費されます（適切な場合）。しかし、その後の書き込みではBOMは生成されません。同様に、ユーザーが最初に書き込んだ場合、その後の読み取りではBOMは消費されません。
* ランダムアクセスではないポート（パイプ、ソケット、端末など）の場合、BOM処理の目的においては、入力ストリームと出力ストリームは_独立_とみなされます。最初の読み取りでは（必要に応じて）BOMが消費され、最初の書き込みでも（必要に応じて）BOMが生成されます。ただし、入力ストリームと出力ストリームは常に同じバイト順を使用します。
* ファイルの先頭にシークすると、「ストリームの開始」フラグが設定されます。そのため、後続のテキストの読み書きでは、BOM が消費または生成されます。ただし、`set- port-encoding!` とは異なり、ポートにバイト順が既に選択されている場合は、シーク後もそのバイト順は有効のままで、BOM の存在によって変更されることはありません。ファイルの先頭以外の場所にシークすると、「ストリームの開始」フラグがクリアされます。

* * *

次へ: [LALR(1) 解析](https://doc.guix.gnu.org/guile/latest/en/guile.html#LALR_00281_0029-Parsing)、前: [入力と出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Input-and-Output)、上: [API リファレンス](https://doc.guix.gnu.org/guile/latest/en/guile.html#API-Reference) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
