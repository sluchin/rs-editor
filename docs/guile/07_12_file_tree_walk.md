### 7.12 ファイルツリーウォーク

このセクションの関数は、ファイルとディレクトリのツリーを走査します。関数には2種類あります。1つ目は高レベルの関数型インターフェースで、2つ目はC言語の`ftw`および`nftw`ルーチンに似ています（GNU Cライブラリリファレンスマニュアルの[ディレクトリツリーの操作](https://doc.guix.gnu.org/libc/latest/en/libc.html#Working-with-Directory-Trees)を参照）。

(use-modules (ice-9 最高))

  

Scheme Procedure: **file-system-tree** file-name \[enter? \[stat\]\]

`(file-name stat children ...)` の形式のツリーを返します。ここで、`stat` は `(stat file-name)` の結果であり、`children` は、`file-name` がディレクトリを指定する場合に、`file-name` に含まれる各ファイルに対応する同様の構造です。

オプションのenter?述語は`(enter? name stat)`として呼び出され、ディレクトリ名への再帰を可能にするにはtrueを返す必要があります。デフォルト値は常に`#t`を返すプロシージャです。ディレクトリがenter?に一致しない場合でも、結果として得られるツリーには表示されますが、子ディレクトリはゼロです。

stat引数はオプションであり、`file-system-fold`と同様に、デフォルト値は`lstat`です（下記参照）。

以下の例は、Guileソースツリー内のmodule/languageディレクトリ以下のファイルの階層リストを取得し、それらの`stat`情報を破棄する方法を示しています。

(use-modules (ice-9 match))

(define remove-stat
;; `file-system-tree` が提供する `stat` オブジェクトを削除します。
;; ツリー内の各ファイルに対して。
(マッチラムダ)
((名前統計) ; フラットファイル
名前）
((名前 ステータス 子 ...) ; ディレクトリ
(リスト名 (マップ削除統計子)))))

(let ((dir (string-append (assq-ref %guile-build-info 'top\_srcdir)
"/module/language")))
(remove-stat (file-system-tree dir)))

⇒
（"言語"
(("value" ("spec.go" "spec.scm"))
（"スキーム"
("spec.go"
「spec.scm」
「compile-tree-il.scm」
「decompile-tree-il.scm」
「decompile-tree-il.go」
"compile-tree-il.go"))
("tree-il"
("spec.go"
「fix-letrec.go」
「inline.go」
「fix-letrec.scm」
「compile-glil.go」
「spec.scm」
「optimize.scm」
「primitives.scm」
...))
...))

`file-system-tree` のようにメモリ上にエントリのツリーを構築するのではなく、ディレクトリのエントリを直接処理することが望ましい場合がよくあります。以下の手順（コンビネータ）は、ディレクトリツリーを走査しながらディレクトリのエントリを直接処理できるように設計されています。実際、`file-system-tree` はこのコンビネータに基づいて実装されています。

Scheme Procedure: **file-system-fold** enter? leaf down up skip error init file-name \[stat\]

指定されたファイル名のディレクトリを再帰的に走査し、以下に説明するリーフ、ダウン、アップ、スキップの各手順を順次適用した結果を返します。

`(enter? path stat result)` が true を返す場合にのみサブディレクトリに入ります。サブディレクトリに入ったときは、`(down path stat result)` を呼び出します。ここで、path はサブディレクトリのパス、stat は `(false-if-exception (stat path))` の結果です。サブディレクトリを出るときは、`(up path stat result)` を呼び出します。

ディレクトリ内の各ファイルに対して、`(leaf path stat result)` を呼び出します。

enter? が `#f` を返す場合、または読み取り不可能なディレクトリに遭遇した場合は、`(skip path stat result)` を呼び出します。

ファイル名がフラットファイルを指定する場合、`(leaf path stat init)` が返されます。

`opendir` または stat 呼び出しが失敗した場合、`(error path stat errno result)` を呼び出します。ここで errno は発生したオペレーティングシステムのエラー番号 (例: `EACCES`) です。そして、`#f` または、利用可能な場合はそのエントリに対する stat 呼び出しの結果を stat します。

特殊な「.」と「..」のエントリは、これらのプロシージャには渡されません。プロシージャへのパス引数は完全なファイル名です（例：`"../foo/bar/gnu"`）。ファイル名が絶対ファイル名の場合、パスも絶対ファイル名になります。デバイス/inode番号のペアで識別されるファイルとディレクトリは、一度だけ走査されます。

オプションの stat 引数のデフォルト値は `lstat` で、これはシンボリックリンクをたどらないことを意味します。シンボリックリンクをたどる場合は、代わりに `stat` プロシージャを使用できます ([stat](07_02_03_file_system.md#723-ファイルシステム) を参照)。

以下の例は、`file-system-fold` の使用方法を示しています。

(define (total-file-size file-name)
「FILE-NAME 配下のファイルのサイズをバイト単位で返します (類似の
（GNU Coreutilsで`du --apparent-size`を実行する場合）

(定義 (入力? 名前 統計 結果)
;; バージョン管理ディレクトリをスキップします。
(not (member (basename name) '(".git" ".svn" "CVS"))))
(define (leaf name stat result)
;; RESULT と NAME にあるファイルのサイズを返します。
(+結果(統計:サイズ統計)))

;; ディレクトリの場合はゼロバイトをカウントします。
(define (down name stat result) result)
(define (up name stat result) result)

;; スキップされたディレクトリについても同様です。
(define (skip name stat result) result)

;; 読み取り不可能なファイル/ディレクトリは無視しますが、ユーザーに警告します。
(define (error name stat errno result)
(format (current-error-port) "warning: ~a: ~a~%"
名前（strerror errno）
結果）

(ファイルシステムフォールド エンター? リーフダウンアップ エラーをスキップ)
0 ; 初期カウンタはゼロバイトです
ファイル名））

（合計ファイルサイズ "."）
⇒ 8217554

(合計ファイルサイズ "/dev/null")
⇒ 0

代替となるC言語風関数については、以下で説明します。

Scheme Procedure: **scandir** name \[select? \[entry<?\]\]

ディレクトリ名に含まれるファイルのうち、述語 select? に一致するファイルの名前のリストを返します (デフォルトではすべてのファイル)。返されるファイル名のリストは、entry<? に従ってソートされます。entry<? のデフォルトは `string-locale<?` で、ファイル名はロケールのアルファベット順にソートされます ([テキスト照合](06_25_support_for_internationalization.md#6252-テキスト照合) を参照)。name が判読不能な場合、またはディレクトリでない場合は `#f` を返します。

この手順は、同名のCライブラリ関数をモデルにしています（GNU Cライブラリリファレンスマニュアルの[ディレクトリコンテンツのスキャン](https://doc.guix.gnu.org/libc/latest/en/libc.html#Scanning-Directory-Content)を参照）。

スキームプロシージャ: **ftw** startname proc \['hash-size n\]

開始名から下へファイルシステムツリーを走査し、各ファイルとディレクトリに対してprocを呼び出す。

ハードリンクとシンボリックリンクの両方がたどられます。ファイルまたはディレクトリは一度だけ処理されたと報告され、別の場所で再び見つかった場合はスキップされます。この結果、`ftw`は循環リンクされたディレクトリ構造に対しても安全です。

各プロシージャ呼び出しは `(proc filename statinfo flag)` であり、続行するには `#t` を返し、停止するにはその他の値を返さなければなりません。

filename は訪問した項目で、startname にパスと項目名を加えたものです。statinfo は filename に対する `stat` ([ファイルシステム](07_02_03_file_system.md#723-ファイルシステム) を参照) の戻り値です。flag は次のいずれかのシンボルです。

`レギュラー`

filenameはファイルであり、これにはデバイスや名前付きパイプなどの特殊ファイルも含まれます。

`ディレクトリ`

filenameはディレクトリです。

`invalid-stat`

`stat` の呼び出し時にエラーが発生したため、何もわかりません。この場合、statinfo は `#f` です。

`ディレクトリが読み取れません`

filenameはディレクトリですが、読み取り不可能なディレクトリであるため、再帰的に処理されることはありません。

`symlink`

ファイル名は、無効なシンボリックリンクです。シンボリックリンクは通常、たどられ、リンク先が報告されます。リンク先が存在しない場合は、リンク自体が報告されます。

`ftw` の戻り値は、処理が完了した場合は `#t` となり、それ以外の場合は、停止の原因となった proc からの `#t` 以外の値となります。

オプション引数シンボル`hash-size`と整数を指定することで、既に訪問した項目を追跡するために使用されるハッシュテーブルのサイズを設定できます。（[ハッシュテーブルリファレンス](06_06_22_hash_tables.md#66222-ハッシュテーブルリファレンス)を参照）

現在の実装では、proc から `#t` 以外の値を返すことだけが `ftw` を終了させる有効な方法です。proc は `throw` やそれに類する手段を使用してエスケープしてはなりません。

スキームプロシージャ: **nftw** startname proc \['chdir\] \[' Depth\] \['hash-size n\] \['mount\] \['physical\]

開始名から始まるファイルシステムツリーを走査し、各ファイルとディレクトリに対してprocを呼び出します。`nftw`は、上記で説明した基本的な`ftw`に加えて、追加機能を備えています。

`ftw`と同様に、ハードリンクとシンボリックリンクの両方がたどられます。ファイルまたはディレクトリは一度だけ処理対象として報告され、別の場所で再び見つかった場合はスキップされます。このため、`nftw`は循環リンクされたディレクトリ構造に対しても安全です。

各プロシージャ呼び出しは `(proc filename statinfo flag base level)` であり、続行するには `#t` を返し、停止するにはその他の値を返さなければなりません。

filename は、startname にパスと項目名を加えた、訪問した項目です。 statinfo は、filename に対する `stat` の戻り値です ([ファイルシステム](07_02_03_file_system.md#723-ファイルシステム) を参照)。 base は、filename 内の整数オフセットで、この項目の basename の開始位置です。 level は、ディレクトリのネストレベルを示す整数で、startname の内容 (またはファイルの場合はその項目自体) に対して 0 から始まります。 flag は、次の記号のいずれかです。

`レギュラー`

filename はファイルであり、デバイスや名前付きパイプなどの特殊ファイルも含まれます。

`ディレクトリ`

filenameはディレクトリです。

`ディレクトリ処理済み`

filenameはディレクトリであり、その内容はすべて閲覧済みです。以下の`depth`オプションを使用する場合、`directory`の代わりにこのフラグを指定します。

`invalid-stat`

ファイル名に`stat`を適用する際にエラーが発生したため、ファイル名に関する情報は何も得られませんでした。この場合、statinfoは`#f`です。

`ディレクトリが読み取れません`

filenameはディレクトリですが、読み取り不可能なディレクトリであるため、再帰的に処理されることはありません。

`stale-symlink`

ファイル名は、無効なシンボリックリンクです。リンクは通常たどられ、リンク先が報告されます。リンク先が存在しない場合は、リンク自体が報告されます。

`symlink`

以下に説明する `physical` オプションを使用すると、ファイル名がシンボリックリンクであり、そのリンク先が存在し（かつ、リンクがたどられていない）ことを示します。

`nftw` の動作を変更するために、以下のオプション引数を指定できます。各引数はシンボルとして渡されます（`hash-size` にはそれに続く整数値を指定します）。

`chdir`

proc を呼び出す前に、対象アイテムを含むディレクトリに移動してください。`nftw` が戻ると、元の現在のディレクトリが復元されます。

このオプションでは、通常、各プロシージャ呼び出しのベースパラメータを使用してファイル名のベース部分を選択します。ファイル名はパスですが、ディレクトリが変更されると無効になります（開始ディレクトリが絶対パスでない限り）。

`depth`

ファイルを「深さ優先」で処理します。つまり、ディレクトリ自体を処理する前に、各ディレクトリの内容に対してprocが呼び出されます。通常は、まずディレクトリが報告され、次にその内容が報告されます。

このオプションでは、ディレクトリに対して proc を実行するフラグは `directory` ではなく `directory-processed` になります。

`hash-size n`

既に訪問した項目を追跡するために使用するハッシュテーブルのサイズを設定します。（[ハッシュテーブルリファレンス](06_06_22_hash_tables.md#66222-ハッシュテーブルリファレンス)を参照）

`mount`

マウントポイントを越えないようにする、つまり、startname と同じファイルシステム上の項目（つまり、同じ `stat:dev`）のみにアクセスするようにします。

物理的

シンボリックリンクをたどらず、代わりに`symlink`としてprocに報告してください。ダングリングリンク（リンク先が存在しないリンク）は、引き続き`stale-symlink`として報告されます。

`nftw` の戻り値は、処理が完了した場合は `#t` となり、それ以外の場合は、停止の原因となった proc からの `#t` 以外の値となります。

現在の実装では、proc から `#t` 以外の値を返すことだけが `ftw` を終了させる有効な方法です。proc は `throw` やそれに類する手段を使用してエスケープしてはなりません。

* * *

次へ: [Streams](07_14_streams.md#714-ストリーム)、前: [File Tree Walk](#712-ファイルツリーウォーク)、上: [Guile Modules](07_00_guile_modules.md#7つのguileモジュール) \[[Contents](00_contents.md "目次")\]\[[Index](index_r5rs.md "Index")\]
