### 7.3 HTTP、Web、その他すべて [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#HTTP_002c-the-Web_002c-and-All-That)

コンピュータ同士を接続して情報を共有することは以前から可能だったが、ここ数十年のワールドワイドウェブの普及により、それがはるかに容易になった。その結果、高度に接続されたコンピューティングネットワークが構築され、Guileはその一部を担っている。

「ウェブ」とは、サーバー、クライアント、プロキシ、キャッシュによって処理されるHTTPプロトコル[26](https://doc.guix.gnu.org/guile/latest/en/guile.html#FOOT26)、およびそのプロトコルによって送受信できるさまざまな種類のメッセージとメッセージコンポーネント、特にHTMLを指します。

ある意味では、ウェブは移動するテキストと言えるでしょう。プロトコル自体はテキストベースであり（ペイロードはバイナリの場合もありますが）、ソケットを作成してウェブにテキストを送信することも可能です。しかし、このようなアプローチは明らかに原始的です。このセクションでは、Guileが提供するより高レベルのデータ型と操作、すなわちURI、HTTPリクエストおよびレスポンスレコード、そして従来のウェブサーバーの実装について詳しく説明します。

このセクションの内容は、後の概念が前の概念に基づいて構築されるという昇順で構成されています。最も高度な視点から始めたい場合は、[Web の例](https://doc.guix.gnu.org/guile/latest/en/guile.html#Web-Examples) を参照し、そこから遡って学習を進めてください。

* [型とウェブ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Types-and-the-Web)
* [ユニバーサルリソース識別子](https://doc.guix.gnu.org/guile/latest/en/guile.html#URIs)
* [ハイパーテキスト転送プロトコル](https://doc.guix.gnu.org/guile/latest/en/guile.html#HTTP)
* [HTTP ヘッダー](https://doc.guix.gnu.org/guile/latest/en/guile.html#HTTP-Headers)
* [転送符号化](https://doc.guix.gnu.org/guile/latest/en/guile.html#Transfer-Codings)
* [HTTPリクエスト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Requests)
* [HTTPレスポンス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Responses)
* [Webクライアント](https://doc.guix.gnu.org/guile/latest/en/guile.html#Web-Client)
* [Webサーバー](https://doc.guix.gnu.org/guile/latest/en/guile.html#Web-Server)
* [Web サンプル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Web-Examples)

* * *

次へ: [Universal Resource Identifiers](https://doc.guix.gnu.org/guile/latest/en/guile.html#URIs)、上へ: [HTTP、Web、その他](https://doc.guix.gnu.org/guile/latest/en/guile.html#Web) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.3.1 型とウェブ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Types-and-the-Web-1)

データ型を適切に活用したプログラムは、多くの一般的なバグから解放されるというのは、誰もが認める真理である。しかし残念ながら、Webプログラミングの一般的な慣習では、この原則が無視されているように思われる。本節では、Webプログラミングにおける表現力豊かなデータ型の重要性を論じる。

「表現力豊かなデータ型」とは、データ型がプログラムが問題を解決する方法について何らかの情報を示すことを意味します。例えば、日付をSRFI 19日付レコードで表現する場合（[SRFI-19 - 時刻/日付ライブラリ](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d19)を参照）、プログラムの一部で常に有効な日付が扱われることになります。無効な日付などの基本的なケースに対するエラー処理は、文字列などの他の型からSRFI 19日付レコードを生成する境界で行われます。

ウェブに関して言えば、データ型はHTTPメッセージの2つの主要な段階、すなわち解析と生成において役立ちます。

リクエストを解析してレスポンスを生成する必要があるサーバーを考えてみましょう。Guile はリクエストを HTTP リクエスト オブジェクトに解析します ( [HTTP リクエスト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Requests) を参照)。各ヘッダーは適切な Scheme データ型に解析されます。入力される文字ストリームから型付きデータへのこの遷移は、プログラムの状態変化です。文字列は解析される場合もあれば、されない場合もあり、解析されない場合は何らかの処理が必要になります (この場合、Guile はエラーをスローします)。しかし、解析されたリクエストを取得した後は、「クライアント」コード (Guile Web フレームワーク上に構築されたコード) は構文の妥当性をチェックする必要はありません。型によって既にこの情報が明示されているからです。

解析境界におけるこの状態変化により、プログラムはより堅牢になります。なぜなら、プログラム自体が多くの一般的なエラーチェックを行う必要がなくなり、アドホックな文字列パーサーの代わりに通常のSchemeプロシージャを使用してリクエストを処理できるようになるからです。

レスポンス生成側（サーバー側）における型の必要性は、重要性は劣らないものの、より微妙な問題です。ユーザーがフォームから送信したテキストを出力するPOSTハンドラーを例に考えてみましょう。このようなハンドラーには、次のような手順が含まれる可能性があります。

;; まず、ヘルパープロシージャ
(define (para . contents)
(string-append "<p>" (string-concatenate contents) "</p>"))

;; さて、ここからがシンプルなウェブアプリケーションの本題です。
(定義 (あなたが言ったテキスト)
(段落「あなたはこう言いました:」テキスト)

(display (you-sai "Hi!"))
⊣ <p>あなたは言いました: こんにちは！</p>

これは、入力テキストにHTMLの特殊文字「<」、「\>」、「&」が含まれていない限り、完全に有効な実装です。しかし、この文字セットの制限に関する規定はプログラム自体には反映されていません。プログラマーがこの点を理解し、別の場所でチェックを行っていると想定する必要があります。

残念ながら、プログラミングの歴史は短いながらも、この仮定を裏付けていません。クロスサイトスクリプティング（XSS）の脆弱性は、フィルタリングされていないユーザー入力が出力に反映されてしまう、よくあるエラーの一例です。ユーザーが細工されたコメントをウェブサイトに送信すると、訪問者がドメインのセキュリティコンテキスト内で悪意のあるJavaScriptを実行してしまう可能性があります。

(display (you-said "<script src=\\"http://bad.com/nasty.js\\" />"))
⊣ <p>あなたは次のように言いました: <script src="http://bad.com/nasty.js" /></p>

ここでの根本的な問題は、ユーザーデータとプログラムテンプレートの両方が文字列で表現されていることです。この同一性のため、型を使ってもプログラマーは両者を区別できず、混乱が生じます。

解決策はいくつか考えられますが、おそらく最良の方法は、HTML を文字列としてではなく、ネイティブの S 式、つまり SXML として扱うことです。基本的な考え方は、HTML は文字列で表されるテキストか、タグ付きリストで表される要素のいずれかであるということです。したがって、「foo」は「"foo"」になり、「<b>foo</b>」は「(b "foo")」になります。属性が存在する場合は、「@」で始まるタグ付きリストに入れます。例：「(img (@ (src "http://example.com/foo.png")))」。詳細については、[SXML](https://doc.guix.gnu.org/guile/latest/en/guile.html#SXML) を参照してください。

SXMLの良い点は、HTML要素とテキストを混同することがない点です。では、`para`の新しい定義を作成してみましょう。

(define (para . contents)
\`(p ,@contents))

(use-modules (sxml simple))
(sxml->xml (you-said "Hi!"))
⊣ <p>あなたは言いました: こんにちは！</p>

(sxml->xml (you-said "<i>Rats, foiled again!</i>"))
⊣ <p>あなたはこう言いました: &lt;i&gt;またしても失敗！&lt;/i&gt;</p>

2番目の例からわかるように、HTML要素が意図せず出力に混入することはありません。しかし、SXMLを`you-said`に渡すことは全く問題ありません。実際、それがSXMLがすべてを文字列として扱う場合よりも優れている大きな利点です。

(sxml->xml (you-said (you-said "<Hi!>")))
⊣ <p>あなたは言いました: <p>あなたは言いました: &lt;こんにちは!&gt;</p></p>

SXML型を使用すると、プロシージャを_構成_できます。型によって、どの部分がHTML要素で、どの部分がテキストであるかが明確になります。そのため、ユーザー入力のエスケープ処理について心配する必要はありません。文字列への型変換が自動的に処理します。XSS脆弱性は過去のものとなりました。

なるほど。それはそれで結構な意見や意見ですが、一体どうやって使うのでしょうか？続きを読んでください！

* * *

次へ: [ハイパーテキスト転送プロトコル](https://doc.guix.gnu.org/guile/latest/en/guile.html#HTTP)、前: [型とWeb](https://doc.guix.gnu.org/guile/latest/en/guile.html#Types-and-the-Web)、上: [HTTP、Web、その他](https://doc.guix.gnu.org/guile/latest/en/guile.html#Web) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.3.2 ユニバーサルリソース識別子 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Universal-Resource-Identifiers)

Guileは、RFC 3986で定義されているユニバーサルリソース識別子（URI）の標準データ型を提供します。

一般的なURI構文は以下のとおりです。

URI参照:= \[スキーム ":"\] \["//" \[userinfo "@"\] ホスト \[":" ポート\]\] パス \\
\[ "?" クエリ \] \[ "#" フラグメント \]

例えば、URI '`http://www.gnu.org/help/`' では、スキームは `http`、ホストは `www.gnu.org`、パスは `/help/` であり、userinfo、port、query、fragment はありません。

Userinfo は一種の抽象化であり、従来の URI スキームでは `username:passwd` という形式の userinfo が許容されていました。しかし、パスワードは URI に含めるべきではないため、RFC はこの慣行を容認せず、`@` 記号より前のすべてを _userinfo_ と呼んでいます。

(use-modules (web uri))

以下の手順は、`(web uri)`モジュールに記載されています。上記のようなフォームを使用して、このモジュールをGuileにロードすることで、これらの手順にアクセスできるようになります。

SchemeからURIを構築する最も一般的な方法は、`build-uri`関数を使用することです。

スキーム手順: **build-uri** scheme \[#:userinfo=`#f`\] \[#:host=`#f`\] \[#:port=`#f`\] \[#:path=`""`\] \[#:query=`#f`\] \[#:fragment=`#f`\] \[#:validate?=`#t`\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-build_002duri)

URIを構築します。schemeはシンボル、portは正の整数または`#f`、その他のフィールドは文字列または`#f`である必要があります。validate?がtrueの場合、構築されたURIが有効であることを確認するために、いくつかの整合性チェックも実行します。

スキームプロシージャ: **uri?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-uri_003f)

objがURIの場合は`#t`を返します。

Guileでは、URIはURIレコードとして表現され、多数の関連アクセサが存在します。

スキーム手順: **uri-scheme** uri [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-uri_002dscheme)

スキーム手順: **uri-userinfo** uri [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-uri_002duserinfo)

スキーム手順: **uri-host** uri [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-uri_002dhost)

スキーム手順: **uri-port** uri [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-uri_002dport)

スキーム手順: **uri-path** uri [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-uri_002dpath)

スキーム手順: **uri-query** uri [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-uri_002dquery)

スキーム手順: **uri-fragment** uri [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-uri_002dfragment)

URIレコード型のフィールドアクセサー。URIスキームはシンボル、またはオブジェクトが相対参照の場合は`#f`になります（下記参照）。ポートは正の整数または`#f`になり、その他のフィールドは文字列、または存在しない場合は`#f`になります。

Scheme手順: **string->uri** string [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002d_003euri)

文字列をURIオブジェクトに解析します。文字列を解析できなかった場合は`#f`を返します。

Scheme手順: **uri->string** uri \[#:include-fragment?=`#t`\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-uri_002d_003estring)

URIを文字列にシリアル化します。URIにスキームのデフォルトポートが含まれている場合、そのポートはシリアル化に含まれません。include-fragment? が false の場合、結果の文字列にはフラグメント（存在する場合）は含まれません。

Scheme 手順: **declare-default-port!** scheme port [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-declare_002ddefault_002dport_0021)

指定されたURIスキームに対して、デフォルトのポートを宣言します。

スキームプロシージャ: **uri-decode** str \[#:encoding=`"utf-8"`\] \[#:decode-plus-to-space? #t\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-uri_002ddecode)

指定された文字列を、文字エンコーディングの名前であるエンコーディングに従ってパーセントデコードします。

この関数は一般的に完全なURI文字列には適用しないでください。パスの場合は、代わりに`split-and-decode-uri-path`を使用してください。クエリ文字列の場合は、クエリを`&`と`=`で分割し、各コンポーネントを個別にデコードしてください。

また、パーセントエンコードされた文字列は文字ではなくバイトをエンコードすることに注意してください。指定されたバイトシーケンスが有効な文字列エンコーディングであるという保証はありません。そのため、デコードされたバイトが指定されたエンコーディングに対して有効でない場合、このルーチンはエラーを通知する可能性があります。デコードされたバイトを直接バイトベクトルとして取得する場合は、エンコーディングに`#f`を渡してください。文字エンコーディングの詳細については、[`set-port-encoding!`](https://doc.guix.gnu.org/guile/latest/en/guile.html#Ports)を参照してください。

decode-plus-to-space? が true の場合（これがデフォルトです）、プラス文字 '+' をスペース文字に置き換えます。これは、`application/x-www-form- urlencoded` データを解析する際に必要です。

デコードされた文字の文字列を返します。エンコードが `#f` の場合はバイトベクトルを返します。

Scheme Procedure: **uri-encode** str \[#:encoding=`"utf-8"`\] \[#:unescaped-chars\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-uri_002dencode)

文字セットに含まれない文字、エスケープされていない文字をパーセントエンコードします。

デフォルトの文字セットには、ASCII の英数字に加え、特殊文字「\-」、「.」、「\_」、「~」が含まれます。その他の文字はパーセントエンコードされ、指定されたエンコーディングのバイトベクトルに書き出された後、各バイトが `%HH` としてエンコードされます。ここで、HH はそのバイトの 16 進数表現です。

スキーム手順: **split-and-decode-uri-path** パス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-split_002dand_002ddecode_002duri_002dpath)

パスを構成要素に分割し、各構成要素をデコードして、空の構成要素を削除します。

例えば、`"/foo/bar%20baz/"` は、2 つの要素からなるリスト `("foo" "bar baz")` にデコードされます。

Scheme手順: **encode-and-join-uri-path**部分 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-encode_002dand_002djoin_002duri_002dpath)

文字列のリストであるはずの各要素をURIエンコードし、区切り文字として「/」を使用して各要素を結合します。

例えば、リスト `("scrambled eggs" "biscuits&gravy")` は `"scrambled%20eggs/biscuits%26gravy"` とエンコードされます。

#### URI のサブタイプ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Subtypes-of-URI)

前述のとおり、すべての URI オブジェクトにスキームがあるわけではありません。「汎用URI 構文」の例では、文法定義の左辺が URI ではなく URI 参照であったことに気づかれたかもしれません。URI 参照は、スキームがオプションである URI の一般化です。スキームが指定されていない場合は、他の関連する URI に対する相対的なものとみなされます。URI 参照の一般的な用途は、HTTP と HTTPS の選択を曖昧にしたい場合です。たとえば、`/foo.css` を参照する Web ページを提供すると、HTTPS 経由で読み込まれた場合は HTTPS が使用され、それ以外の場合は HTTP が使用されます。

Scheme 手順: **build-uri-reference** \[#:scheme=`#f`\] \[#:userinfo=`#f`\] \[#:host=`#f`\] \[#:port=`#f`\] \[#:path=`""`\] \[#:query=`#f`\] \[#:fragment=`#f`\] \[#:validate?=`#t`\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-build_002duri_002dreference)

`build-uri`と同様だが、スキームをオプションで指定できる。

スキームプロシージャ: **uri-reference?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-uri_002dreference_003f)

objがURI参照である場合は`#t`を返します。これは最も一般的なURI述語であり、スキームを持つ完全なURI（`uri?`に一致するもの）だけでなく、スキームを持たないURIも含まれます。

スキームを明示的に欠いたURI参照である_relative-ref_を作成することも可能です。

Scheme 手順: **build-relative-ref** \[#:userinfo=`#f`\] \[#:host=`#f`\] \[#:port=`#f`\] \[#:path=`""`\] \[#:query=`#f`\] \[#:fragment=`#f`\] \[#:validate?=`#t`\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-build_002drelative_002dref)

`build-uri`と同様だが、スキームは含まれていない。

Scheme Procedure: **relative-ref?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-relative_002dref_003f)

objが「relative-ref」（スキームを持たないURI参照）である場合は、`#t`を返します。すべてのURI参照は、`uri?`または`relative-ref?`のいずれかに一致します（両方には一致しません）。

上記から分かりにくいかもしれませんが、これらのURIタイプの中で最も一般的なのはURI参照であり、`build-uri-reference`が最も一般的なコンストラクタです。`build-uri`と`build-relative-ref`は、URI参照に特定の制約を適用します。最も汎用的なURIパーサーは`string->uri-reference`であり、相対参照が必要な場合のためのパーサーも用意されています。

`uri?` はスキームを持つ URI オブジェクトに対してのみ `#t` を返します。つまり、相対参照は拒否されます。

Scheme手順: **string->uri-reference** string [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002d_003euri_002dreference)

文字列をURIオブジェクトに解析します。スキームは不要です。文字列を解析できなかった場合は`#f`を返します。

Scheme手順: **string->relative-ref** string [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002d_003erelative_002dref)

文字列をURIオブジェクトに解析し、スキームが存在しないことを確認します。文字列を解析できなかった場合は`#f`を返します。

* * *

次へ: [HTTP ヘッダー](https://doc.guix.gnu.org/guile/latest/en/guile.html#HTTP-Headers)、前: [ユニバーサル リソース 識別子](https://doc.guix.gnu.org/guile/latest/en/guile.html#URIs)、上: [HTTP、Web、その他](https://doc.guix.gnu.org/guile/latest/en/guile.html#Web) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.3.3 ハイパーテキスト転送プロトコル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Hyper_002dText-Transfer-Protocol )

GuileにWeb機能を組み込むという当初の動機は、外部パッケージに頼るのではなく、人々がコードを共有できる標準的な基盤を確立することでした。そのため、HTTPプロトコルの要素に対応する低レベルのパーサーとアンパーサーを多数提供することで、データ型への注力を継続しています。

現時点で低レベルの詳細をスキップしてWebページに進む場合は、[Webクライアント](https://doc.guix.gnu.org/guile/latest/en/guile.html#Web-Client)および[Webサーバー](https://doc.guix.gnu.org/guile/latest/en/guile.html#Web-Server)を参照してください。そうでない場合は、HTTPモジュールをロードして読み進めてください。

(use-modules (web http))

`(web http)` モジュールの主な目的は、標準 HTTP ヘッダーを解析および逆解析し、Guile に対してネイティブなデータ構造として表現することです。たとえば、`Date:` ヘッダーは文字列ではなく、SRFI-19 日付レコード ([SRFI-19 - Time/Date Library](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d19) を参照) として表現されます。

GuileはRFCにかなり厳密に従おうとしているが（互換性ハックは破滅への道である）、あまりかけ離れていないテキストについては多少の許容範囲が設けられている。

ヘッダー名は小文字の記号で表されます。

Scheme手順: **string->header** name [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002d_003eheader)

名前を解析して、シンボリックなヘッダー名に変換します。

Scheme 手順: **header->string** sym [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-header_002d_003estring)

sym という名前のヘッダーの文字列形式を返します。

例えば：

(文字列->ヘッダー "Content-Length")
⇒コンテンツの長さ
(header->string 'content-length)
⇒ 「コンテンツの長さ」

(string->header "FOO")
⇒フー
(ヘッダー->文字列 'foo')
⇒ 「フー」

Guileは、既知のヘッダー、その文字列名、およびいくつかの解析・シリアル化手順のレジストリを保持しています。ヘッダーが未知の場合、その文字列名は単にシンボル名をタイトルケースで表記したものです。

Scheme Procedure: **known-header?** sym [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-known_002dheader_003f)

symが既知のヘッダーで、関連するパーサーとシリアル化手順が含まれている場合は`#t`を返し、そうでない場合は`#f`を返します。

Scheme 手順: **header-parser** sym [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-header_002dparser)

sym という名前のヘッダーの値パーサーを返します。結果は、文字列を引数として受け取り、解析された値を返すプロシージャです。Guile がヘッダーを認識していない場合は、文字列をそのまま渡すデフォルトのパーサーが返されます。

Scheme Procedure: **header-validator** sym [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-header_002dvalidator)

指定された値が sym という名前のヘッダーに対して有効な場合、`#t` を返す述語を返します。不明なヘッダーのデフォルトのバリデーターは `string?` です。

Scheme 手順: **header-writer** sym [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-header_002dwriter)

sym という名前のヘッダーの値をポートに書き込むプロシージャを返します。このプロシージャは、値とポートの 2 つの引数を取ります。デフォルトの書き込み先は `display` です。

Guileが標準で認識するヘッダーセットの詳細については、[HTTPヘッダー](https://doc.guix.gnu.org/guile/latest/en/guile.html#HTTP-Headers)を参照してください。独自のヘッダーを追加するには、`declare-header!`プロシージャを使用します。

Scheme 手順: **declare-header!** name parser validator writer \[#:multiple?=`#f`\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-declare_002dheader_0021)

指定されたヘッダーに対して、パーサー、バリデーター、およびライターを宣言します。

例えば、何らかのプロキシの背後でWebサーバーを運用していて、プロキシが元のクライアントのIPv4アドレスを示す`X-Client-Address`ヘッダーを追加するとします。HTTPリクエストレコードでこのヘッダーを文字列として残すのではなく、Scheme値に解析したい場合、次のようにGuileのHTTPスタックにこのヘッダーを登録できます。

(declare-header! "X-Client-Address"
(ラムダ (str)
(inet-pton AF\_INET str))
(ラムダ (ip)
(and (整数? ip) (正確な? ip) (<= 0 ip #xffffffff)))
(ラムダ (ip port)
(ディスプレイ (inet-ntop AF\_INET ip) ポート)))

Scheme Procedure: **declare-opaque-header!** name [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-declare_002dopaque_002dheader_0021)

ヘッダーの値を「そのまま」返したり書き込んだりしたい場合のための、`declare-header!` の特殊バージョンです。

Scheme Procedure: **valid-header?** sym val [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-valid_002dheader_003f)

val が名前 sym のヘッダーに対する有効な Scheme 値であれば true を返し、そうでなければ `#f` を返します。

ヘッダーの読み書きのための汎用インターフェースができたので、まさにそれを行います。

スキーム手順: **read-header** ポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002dheader)

ポートからHTTPヘッダーを1つ読み込みます。ヘッダー名と解析されたスキーム値の2つの値を返します。ヘッダーは既知だが値が無効な場合は例外が発生する場合があります。

メッセージ本文の末尾（つまり、空白行）に達した場合、両方の値に対してファイル終端オブジェクトを返します。

Scheme 手順: **parse-header** name val [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-parse_002dheader)

文字列valを、nameという名前のヘッダーのパーサーで解析します。解析された値を返します。

Scheme Procedure: **write-header** name val port [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-write_002dheader)

指定されたヘッダー名と値を、`header-writer`のライターを使用してポートに書き込みます。

スキーム手順: **read-headers** ポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002dheaders)

指定されたポートからHTTPメッセージのヘッダーを読み込み、順序付きリストとして返します。

Scheme Procedure: **write-headers** headers port [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-write_002dheaders)

指定されたヘッダーリストをポートに書き込みます。ユーザーが別のヘッダーを追加したい場合があるため、末尾の「\\r\\n」は書きません。

`(web http)`モジュールには、リクエスト行とレスポンス行を読み書きするためのユーティリティプロシージャもいくつか含まれています。

Scheme Procedure: **parse-http-method** str \[start\] \[end\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-parse_002dhttp_002dmethod)

文字列からHTTPメソッドを解析します。結果は`GET`のような大文字の記号になります。

Scheme Procedure: **parse-http-version** str \[start\] \[end\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-parse_002dhttp_002dversion)

文字列からHTTPバージョンを解析し、メジャーバージョンとマイナーバージョンのペアとして返します。例えば、`HTTP/1.1`は整数のペア`(1 . 1)`として解析されます。

Scheme Procedure: **parse-request-uri** str \[start\] \[end\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-parse_002drequest_002duri)

HTTPリクエスト行からURIを解析します。リクエスト内のURIには、スキームやホスト名は必須ではありません。結果はURIオブジェクトになります。

スキーム手順: **read-request-line** ポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002drequest_002dline)

ポートからのHTTPリクエストの最初の行を読み込み、メソッド、URI、バージョンの3つの値を返します。

Scheme Procedure: **write-request-line** method uri version port [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-write_002drequest_002dline)

ポートへのHTTPリクエストの最初の行を記述します。

スキーム手順: **read-response-line** ポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002dresponse_002dline)

ポートからHTTPレスポンスの最初の行を読み取り、HTTPバージョン、レスポンスコード、および「理由フレーズ」の3つの値を返します。

スキーム手順: **write-response-line** バージョン コード 理由フレーズ ポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-write_002dresponse_002dline)

ポートへのHTTPレスポンスの最初の行を記述します。

* * *

次へ: [転送符号化](https://doc.guix.gnu.org/guile/latest/en/guile.html#Transfer-Codings)、前: [ハイパーテキスト転送プロトコル](https://doc.guix.gnu.org/guile/latest/en/guile.html#HTTP)、上: [HTTP、Web、その他](https://doc.guix.gnu.org/guile/latest/en/guile.html#Web) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.3.4 HTTP ヘッダー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#HTTP-Headers-1)

`(web http)` モジュールは、ヘッダーを解析するためのインフラストラクチャを定義するだけでなく、HTTP/1.1 標準で定義されているすべてのヘッダーに対して、特定のパーサーとアンパーサーを定義します。

例えば、「Accept-Language」という名前のヘッダーで値が「en, es;q=0.8」の場合、Guile はそれを品質リスト (以下に定義) として解析します。

(解析ヘッダー '受け入れ言語 "en, es;q=0.8")
⇒ ((1000 . "en") (800 . "es"))

「Accept-Language」ヘッダーの値の形式は、HTTP標準で定義されている他のすべてのヘッダーの値と同様に、以下に定義されています。（ヘッダーが不明な場合は、値は文字列として返されます。）

簡潔にするため、以下のヘッダー定義は「型 `name`」の形式で示されており、ヘッダー `name` の値は指定された型になることを示しています。Guile は内部的にヘッダー名を小文字で処理するため、このドキュメントでは型名をタイトルケースで表記します。各ヘッダーの目的と例を簡単に説明します。

これらのヘッダーの意味の詳細については、HTTP 1.1 標準規格である RFC 2616 を参照してください。

* [HTTPヘッダーの種類](https://doc.guix.gnu.org/guile/latest/en/guile.html#HTTP-Header-Types)
* [一般ヘッダー](https://doc.guix.gnu.org/guile/latest/en/guile.html#General-Headers)
* [エンティティヘッダー](https://doc.guix.gnu.org/guile/latest/en/guile.html#Entity-Headers)
* [リクエストヘッダー](https://doc.guix.gnu.org/guile/latest/en/guile.html#Request-Headers)
* [レスポンスヘッダー](https://doc.guix.gnu.org/guile/latest/en/guile.html#Response-Headers)

#### 7.3.4.1 HTTPヘッダータイプ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#HTTP-Header-Types)

ここでは、ヘッダーを定義する際に後述する型を定義します。

HTTP ヘッダータイプ: **日付** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-Date)

SRFI-19の日付。

HTTP ヘッダータイプ: **KVList** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-KVList)

要素がキーまたはキーと値のペアであるリスト。キーはシンボルに解析されます。値はデフォルトでは文字列です。文字列以外の値は例外であり、必要に応じて以下に明示的に記載されています。

HTTP ヘッダータイプ: **SList** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SList)

文字列のリスト。

HTTP ヘッダータイプ: **Quality** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-Quality)

0から1000までの正確な整数。品質は、複数の選択肢がある場合の好みを表すために使用されます。たとえば、品質が870の選択肢は、品質が500の選択肢よりも好ましいとされます。

（特性値は通信上では0.0から1.0までの数値で表記されるが、規格では小数点以下3桁までしか使用できないため、0から1000までの整数に相当する。そのため、ガイルはこの数値を使用している。）

HTTP ヘッダータイプ: **QList** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-QList)

品質リスト：ペアのリストで、carは品質、cdrは文字列です。オプションとその品質のリストを表すために使用されます。

HTTP ヘッダータイプ: **ETag** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ETag)

エンティティタグはペアとして表されます。ペアのcarは不透明な文字列で、cdrはエンティティタグが「強い」エンティティタグの場合は`#t`、そうでない場合は`#f`になります。

#### 7.3.4.2 一般的なヘッダー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#General-Headers)

一般的なHTTPヘッダーは、あらゆるHTTPメッセージに含まれる可能性があります。

HTTP ヘッダー: `KVList` **cache-control** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-cache_002dcontrol)

キャッシュ制御ディレクティブのキーと値のリスト。詳細はRFC 2616を参照してください。

`max-age`、`max-stale`、`min-fresh`、および`s-maxage`のパラメータが存在する場合、それらはすべて非負の整数として解析されます。

`private` および `no-cache` のパラメータが存在する場合、それらはヘッダー名のリストとして、シンボルとして解析されます。

(parse-header 'cache-control "no-cache,no-store"
⇒ （キャッシュなし、ストアなし）
(parse-header 'cache-control "no-cache=\\"Authorization,Date\\",no-store"
⇒ ((no-cache . (認証日)) no-store)
(parse-header 'cache-control "no-cache=\\"Authorization,Date\\",max-age=10"
⇒ ((no-cache . (認証日)) (max-age . 10))

HTTP ヘッダー: `List` **connection** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-connection)

このHTTP接続にのみ適用されるヘッダー名のリスト（記号形式）。さらに、「close」記号が存在する場合があり、これはサーバーがリクエストへの応答後に接続を閉じる必要があることを示します。

(parse-header 'connection "close")
⇒ （閉じる）

HTTP ヘッダー: `Date` **date** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-date-2)

特定のHTTPメッセージが発信された日付。

(parse-header 'date "Tue, 15 Nov 1994 08:12:31 GMT")
⇒ #<日付 ...>

HTTP ヘッダー: `KVList` **pragma** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-pragma)

実装固有の指示のキーと値のリスト。

(parse-header 'pragma "no-cache, broccoli=tasty")
⇒ (no-cache (broccoli . "tasty"))

HTTP ヘッダー: `List` **trailer** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-trailer)

メッセージヘッダーではなく、メッセージ本文の後に表示されるヘッダー名のリスト。

(解析ヘッダー 'トレーラー "ETag")
⇒ （etag）

HTTP ヘッダー: `List` **transfer-encoding** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-transfer_002dencoding)

キーと値のリストとして表現される転送コードの一覧。仕様で定義されている転送コードは「チャンク化」のみです。

(parse-header 'transfer-encoding "chunked")
⇒ ((チャンク化))

HTTP ヘッダー: `List` **upgrade** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-upgrade)

サーバーがリクエストへの応答として使用できる追加プロトコルを示す文字列のリスト。

(parse-header 'upgrade "WebSocket")
⇒ （「WebSocket」）

FIXME: より完全に解析する？

HTTP ヘッダー: `List` **via** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-via)

中間サーバーおよびプロキシのプロトコルバージョンとホストを示す文字列のリスト。1つのメッセージに複数の`via`ヘッダーが含まれる場合があります。

(parse-header 'via "1.0 venus, 1.1 mars")
⇒ （「1.0 金星」「1.1 火星」）

HTTP ヘッダー: `List` **警告** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-warning)

サーバーまたは中間プロキシによって発行された警告のリスト。各警告は、0～1000の正確な整数であるコード、文字列としてのホスト、文字列としての警告テキスト、および`#f`またはSRFI-19日付の4つの要素のリストです。

1つのメッセージに複数の`warning`ヘッダーが含まれる場合があります。

(parse-header 'warning "123 foo \\"コア侵害が差し迫っています\\"")
⇒ ((123 "foo" "core-breach imminent" #f))

#### 7.3.4.3 エンティティヘッダー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Entity-Headers)

エンティティヘッダーは、あらゆるHTTPメッセージに含まれる可能性があり、HTTPリクエストまたはレスポンスで参照されるリソースを示します。

HTTP ヘッダー: `List` **allow** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-allow)

特定のリソースで許可されているメソッドのリスト（シンボル形式）。

(parse-header 'allow "GET, HEAD")
⇒ （ヘッドゲット）

HTTP ヘッダー: `List` **content-encoding** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-content_002dencoding)

コンテンツのコード一覧（記号形式）。

(parse-header 'content-encoding "gzip")
⇒ (gzip)

HTTP ヘッダー: `List` **content-language** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-content_002dlanguage)

リソースが使用されている言語を文字列として表したもの。

(parse-header 'content-language "en")
⇒ （"en")

HTTP ヘッダー: `UInt` **content-length** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-content_002dlength)

リソース内のバイト数を、正確な非負の整数で表したもの。

(parse-header 'content-length "300")
⇒ 300

HTTP ヘッダー: `URI` **content-location** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-content_002dlocation)

別のURIからもアクセス可能なリソースの場合、そのリソースの正規URIを指定します。

(parse-header 'content-location "http://example.com/foo")
⇒ #<<uri> ...>

HTTP ヘッダー: `String` **content-md5** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-content_002dmd5)

リソースのMD5ダイジェスト。

(parse-header 'content-md5 "ffaea1a79810785575e29e2bd45e2fa5")
⇒ 「ffaea1a79810785575e29e2bd45e2fa5」

HTTP ヘッダー: `List` **content-range** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-content_002drange)

範囲指定は、3つの要素のリストとして表されます。シンボル「bytes」、バイト範囲を示すシンボル「*」または整数のペア、インスタンス長を示す「*」または整数です。レスポンスにリソースの一部のみが含まれることを示すために使用されます。

(parse-header 'content-range "bytes 10-20/\*")
⇒ (バイト (10 . 20) \*)

HTTP ヘッダー: `List` **content-type** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-content_002dtype)

リソースのMIMEタイプ（シンボル形式）と、関連するパラメータ。

(parse-header 'content-type "text/plain")
⇒ (text/plain)
(parse-header 'content-type "text/plain;charset=utf-8")
⇒ (text/plain (charset . "utf-8"))

`charset` パラメータはやや不適切な名称であり、HTTP 仕様でもその点は認められています。これは文字セットではなく、文字のエンコーディングを指定するものです。

HTTP ヘッダー: `Date` **expires** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-expires)

応答で提供されたリソースが古くなったとみなされる日時。

(parse-header 'expires "Tue, 15 Nov 1994 08:12:31 GMT")
⇒ #<日付 ...>

HTTP ヘッダー: `Date` **last-modified** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-last_002dmodified)

応答で示されたリソースが最後に更新された日時。

(parse-header 'expires "Tue, 15 Nov 1994 08:12:31 GMT")
⇒ #<日付 ...>

#### 7.3.4.4 リクエストヘッダー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Request-Headers)

リクエストヘッダーはHTTPリクエストにのみ含まれ、レスポンスには含まれません。

HTTP ヘッダー: `List` **accept** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-accept-1)

レスポンスに使用するメディアタイプの優先順位リスト。リストの各要素は、`content-type` と同じ形式のリストです。

(parse-header 'accept "text/html,text/plain;charset=utf-8")
⇒ ((text/html) (text/plain (charset . "utf-8")))

好みは品質値で表現されます。

(parse-header 'accept "text/html;q=0.8,text/plain;q=0.6")
⇒ ((text/html (q . 800)) (text/plain (q . 600)))

HTTP ヘッダー: `QList` **accept-charset** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-accept_002dcharset)

許容される文字セットの良質なリスト。HTTP が「文字セット」と呼ぶものは、Guile が「文字エンコーディング」と呼ぶものであることを改めて注意してください。

(parse-header 'accept-charset "iso-8859-5, unicode-1-1;q=0.8")
⇒ ((1000 . "iso-8859-5") (800 . "unicode-1-1"))

HTTP ヘッダー: `QList` **accept-encoding** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-accept_002dencoding)

許容されるコンテンツコーディングの質の高いリスト。

(parse-header 'accept-encoding "gzip,identity=0.8")
⇒ ((1000 . "gzip") (800 . "identity"))

HTTP ヘッダー: `QList` **accept-language** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-accept_002dlanguage)

使用可能な言語の質の高いリスト。

(parse-header 'accept-language "cn,en=0.75")
⇒ ((1000 . "cn") (750 . "en"))

HTTP ヘッダー: `Pair` **authorization** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-authorization)

認証情報。ペアのcarは、`basic`などの認証方式を示します。基本認証の場合、ペアのcdrはbase64エンコードされた「user:pass」文字列になります。`digest`などの他の認証方式の場合、cdrは認証情報のキーと値のリストになります。

(parse-header 'authorization "Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ=="
⇒ (基本 . "QWxhZGRpbjpvcGVuIHNlc2FtZQ==")

HTTP ヘッダー: `List` **expect** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-expect-1)

クライアントがサーバーに対して抱く期待事項のリスト。これらの期待事項はキーと値のペアのリストです。

(parse-header 'expect "100-continue")
⇒ ((100-続き))

HTTP ヘッダー: `String` **from** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-from)

HTTPリクエストを行ったユーザーのメールアドレス。

(parse-header 'from "bob@example.com")
⇒ "bob@example.com"

HTTP ヘッダー: `ペア` **host** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-host)

要求されたリソースのホストをホスト名とポート番号のペアで指定します。ポート番号が指定されていない場合は、ポート番号は「#f」になります。

(parse-header 'host "gnu.org:80")
⇒ ("gnu.org" . 80)
(parse-header 'host "gnu.org")
⇒ ("gnu.org" . #f)

HTTP ヘッダー: `*|List` **if-match** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-if_002dmatch)

リソースのetagがそのセットに含まれている場合にのみリクエストを続行することを示すetagのセット。任意のetagを示す記号「*」、またはエンティティタグのリストのいずれか。

(parse-header 'if-match "\*")
⇒ \*
(parse-header 'if-match "asdfadf")
⇒ (("asdfadf" . #t))
(parse-header 'if-match W/"asdfadf")
⇒ (("asdfadf" . #f))

HTTP ヘッダー: `Date` **if-modified-since** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-if_002dmodified_002dsince)

指定された日付以降にリソースが変更された場合にのみ、応答を実行する必要があることを示します。

(parse-header 'if-modified-since "Tue, 15 Nov 1994 08:12:31 GMT")
⇒ #<日付 ...>

HTTP ヘッダー: `*|List` **if-none-match** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-if_002dnone_002dmatch)

リソースのetagがセットに含まれていない場合にのみリクエストを続行することを示すetagのセット。任意のetagを示す記号「*」、またはエンティティタグのリストのいずれか。

(parse-header 'if-none-match "\*")
⇒ \*

HTTP ヘッダー: `ETag|Date` **if-range** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-if_002drange)

リソースが変更日またはetagに一致する場合にのみ、範囲要求を続行することを示します。エンティティタグ、またはSRFI-19日付のいずれかです。

(parse-header 'if-range "\\"original-etag\\"")
⇒ ("original-etag" . #t)

HTTP ヘッダー: `Date` **if-unmodified-since** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-if_002dunmodified_002dsince)

指定された日付以降にリソースが変更されていない場合に限り、応答を実行する必要があることを示します。

(parse-header 'if-not-modified-since "Tue, 15 Nov 1994 08:12:31 GMT")
⇒ #<日付 ...>

HTTP ヘッダー: `UInt` **max-forwards** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-max_002dforwards)

リクエストが経由するプロキシまたはゲートウェイの最大ホップ数。

(parse-header 'max-forwards "10")
⇒ 10

HTTP ヘッダー: `Pair` **proxy-authorization** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-proxy_002dauthorization)

プロキシ接続の認証情報。フォーマットの詳細については、上記の「authorization」に関するドキュメントを参照してください。

(parse-header 'proxy-authorization "Digest foo=bar,baz=qux"
⇒ (ダイジェスト (foo . "bar") (baz . "qux"))

HTTP ヘッダー: `Pair` **range** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-range)

範囲指定リクエスト。クライアントがリソースの一部のみを要求していることを示します。ペアのcarはシンボル「bytes」、cdrはペアのリストです。cdrの各要素は範囲を示し、carは最初のバイト位置、cdrは最後のバイト位置を整数で表します。指定しない場合は「#f」となります。

(parse-header 'range "bytes=10-30,50-")
⇒ (バイト (10 . 30) (50 . #f))

HTTP ヘッダー: `URI` **referer** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-referer)

ユーザーをこのリソースに誘導したリソースのURI。ヘッダー名はスペルミスですが、これを使用するしかありません。

(解析ヘッダーのリファラー「http://www.gnu.org/」)
⇒ #<uri ...>

HTTP ヘッダー: `List` **te** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-te)

キーと値のリストとして表現される転送コードの一覧。一般的な転送コードは「トレーラー」です。

(parse-header 'te "trailers")
⇒ （（予告編））

HTTP ヘッダー: `String` **user-agent** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-user_002dagent)

リクエストを行ったユーザーエージェントを示す文字列。仕様ではこのヘッダーの構造化されたフォーマットが定義されていますが、広く無視されているため、Guile は厳密な解析を試みません。

(parse-header 'user-agent "Mozilla/5.0")
⇒ "Mozilla/5.0"

#### 7.3.4.5 レスポンスヘッダー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Response-Headers)

HTTP ヘッダー: `List` **accept-ranges** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-accept_002dranges)

サーバーがサポートする射程単位のリスト（記号形式）。

(parse-header 'accept-ranges "bytes")
⇒ （バイト）

HTTP ヘッダー: `UInt` **age** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-age)

キャッシュされたレスポンスの経過時間（秒単位）。

(parse-header 'age "3600")
⇒ 3600

HTTP ヘッダー: `ETag` **etag** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-etag)

リソースのエンティティタグ。

(parse-header 'etag "\\"foo\\"")
⇒ ("foo" . #t)

HTTP ヘッダー: `URI-reference` **location** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-location-1)

リクエストが完了する可能性のあるURI参照。クライアント側のリダイレクトを実行するために、リダイレクトステータスコードと組み合わせて使用されます。

(parse-header 'location "http://example.com/other")
⇒ #<uri ...>

HTTP ヘッダー: `List` **proxy-authenticate** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-proxy_002dauthenticate)

プロキシに対する一連の課題。認証が必要であることを示す。

(parse-header 'proxy-authenticate "Basic realm=\\"foo\\"")
⇒ ((basic (realm . "foo")))

HTTP ヘッダー: `UInt|Date` **retry-after** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-retry_002dafter)

サーバーがビジー状態であることを示すステータスコード（503など）と組み合わせて使用され、クライアントが後で再試行する必要があることを示します。秒数、または日付を指定します。

(parse-header 'retry-after "60")
⇒ 60

HTTP ヘッダー: `String` **server** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-server)

サーバーを識別する文字列。

(parse-header 'server "私の最初のウェブサーバー")
⇒ 「私の最初のウェブサーバー」

HTTP ヘッダー: `*|List` **vary** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-vary)

このレスポンスの計算に使用されたリクエストヘッダーのセット。たとえば、`accept-language` ヘッダーへの応答として、サーバー側でコンテンツネゴシエーションが実行されたことを示すために使用されます。また、すべてのヘッダーが考慮されたことを示す記号 `*` を使用することもできます。

(parse-header 'vary "Accept-Language, Accept")
⇒ (accept-language accept)

HTTP ヘッダー: `List` **www-authenticate** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-www_002dauthenticate)

ユーザーに対する一連の課題。認証が必要であることを示す。

(parse-header 'www-authenticate "Basic realm=\\"foo\\"")
⇒ ((basic (realm . "foo")))

* * *

次へ: [HTTP リクエスト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Requests)、前: [HTTP ヘッダー](https://doc.guix.gnu.org/guile/latest/en/guile.html#HTTP-Headers)、上: [HTTP、Web、その他](https://doc.guix.gnu.org/guile/latest/en/guile.html#Web) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.3.5 転送符号化 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Transfer-Codings-1)

HTTP 1.1では、メッセージ本文に様々な転送符号化方式を適用できます。これには、様々な種類の圧縮方式やHTTPチャンク符号化が含まれます。現在、Guileはチャンク符号化のみをサポートしています。

チャンク符号化は、メッセージ本文に適用できるオプションの符号化方式であり、事前に長さが不明なメッセージでも返送できるようにするものです。このようなメッセージはチャンクに分割され、最後のチャンクは長さがゼロで終了します。

エンコーディングの処理をより簡単にするために、Guileは既存のポートを「ラップ」するポートを作成する手順を提供し、内部で透過的に変換を適用します。

これらの手順は`(web http)`モジュールに含まれています。

(use-modules (web http))

Scheme Procedure: **make-chunked-input-port** port \[#:keep-alive?=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dchunked_002dinput_002dport)

ポートからチャンクエンコードされたデータを透過的に読み取り、デコードする新しいポートを返します。チャンクエンコードされたデータがなくなったら、ファイルの終端オブジェクトを返します。ポートが閉じられると、keep-alive? が true でない限り、ポートも閉じられます。

チャンク化された入力が途中で終了した場合、`&chunked-input-ended-promaturely` 例外が発生します。

(use-modules (ice-9 rdelim))

(define s "5\\r\\nFirst\\r\\nA\\r\\n line\\n Sec\\r\\n8\\r\\nond line\\r\\n0\\r\\n")
(define p (make-chunked-input-port (open-input-string s)))
(read-line s)
⇒ 「最初の行」
(read-line s)
⇒ 「2行目」

Scheme Procedure: **make-chunked-output-port** port \[#:keep-alive?=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dchunked_002doutput_002dport)

新しいポートを返します。このポートは、データをチャンクエンコードしてからポートに書き込みます。このポートに書き込みが行われるたびに、ポートがフラッシュされるまでデータをバッファリングし、フラッシュされた時点で、それまでに書き込まれたすべてのデータを含むチャンクを書き込みます。ポートが閉じられると、残りのデータと終端のゼロチャンクがポートに書き込まれます。また、keep-alive? が true でない限り、ポートが閉じられます。

注：バッファリングされたデータがない場合にチャンク出力ポートを強制的に使用しても、ゼロチャンクは書き込まれません。これは、クライアントによるデータの解釈が誤ってしまうためです。

（出力文字列付き呼び出し）
(ラムダ (出力)
(define out\* (make-chunked-output-port out #:keep-alive? #t))
(最初のチャンクを表示する*)
(強制出力*)
(force-output out\*) ; これはゼロチャンクを書き込まないことに注意してください
(「2番目のチャンク」を表示する*)
(ポートを閉じる*)))
⇒ "b\\r\\n最初のチャンク\\r\\nc\\r\\n2番目のチャンク\\r\\n0\\r\\n"

* * *

次へ: [HTTP レスポンス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Responses)、前: [転送符号化](https://doc.guix.gnu.org/guile/latest/en/guile.html#Transfer-Codings)、上: [HTTP、Web、その他](https://doc.guix.gnu.org/guile/latest/en/guile.html#Web) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.3.6 HTTPリクエスト [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#HTTP-Requests)

(use-modules (Webリクエスト))

requestモジュールには、HTTPリクエスト用のデータ型が含まれています。

* [文字セットに関する重要な注意事項](https://doc.guix.gnu.org/guile/latest/en/guile.html#An-Important-Note-on-Character-Sets)
* [リクエストAPI](https://doc.guix.gnu.org/guile/latest/en/guile.html#Request-API)

#### 7.3.6.1 文字セットに関する重要な注意事項 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#An-Important-Note-on-Character-Sets)

HTTPリクエストは、リクエスト本文とヘッダーからなるリクエスト本体と、（オプションで）ボディの2つの部分から構成されます。ボディはバイナリ形式のコンテンツタイプを持つ場合があり、テキスト形式の場合でも、その長さは文字数ではなくバイト単位で指定されます。

したがって、HTTPは基本的にバイナリプロトコルです。ただし、リクエスト行とヘッダーはASCIIのサブセットで指定されているため、ポートのエンコーディングがASCII互換の1バイト/文字のエンコーディングに設定されていれば、テキストとして扱うことができます。ISO-8859-1（latin-1）はまさにそのようなエンコーディングであり、Guileにとって非常に効率的です。

つまり、Guileはネットワークからリクエストを読み取ったり書き出したりする際に、ポートのエンコーディングをlatin-1に設定し、リクエストヘッダーをテキストとして扱うのです。

リクエストボディはまた別の問題です。バイナリデータの場合、データはおそらくバイトベクターに格納されているため、R6RSのバイナリ出力手順を使用してバイナリペイロードを書き出します。テキストデータは通常、何らかの文字エンコーディング（通常はUTF-8）に書き出す必要があり、その後、結果として得られるバイトベクターがポートに書き出されます。

要約すると、Guileは汎用性を損なうことなく、latin-1ソケットを介してHTTPの読み書きを行います。

#### 7.3.6.2 リクエスト API [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Request-API)

Scheme Procedure: **request?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_003f)

スキーム手順: **request-method** request [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002dmethod)

スキーム手順: **request-uri** request [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002duri)

スキーム手順: **request-version** request [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002dversion)

スキーム手順: **request-headers** request [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002dheaders)

スキーム手順: **request-meta** リクエスト [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002dmeta)

スキーム手順: **request-port** リクエスト [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002dport)

リクエストタイプに対応する述語とフィールドアクセサー。フィールドは以下のとおりです。

`メソッド`

HTTPメソッド、例えば`GET`。

`uri`

URIをURIレコードとして表したもの。

`バージョン`

HTTP バージョンのペア、例えば `(1 . 1)`。

`ヘッダー`

リクエストヘッダーは、解析された値のリストとして返されます。

`メタ`

他のデータの任意のリスト。たとえば、`accept` から `sockaddr` で返される情報など（[ネットワークソケットと通信](https://doc.guix.gnu.org/guile/latest/en/guile.html#Network-Sockets-and-Communication) を参照）。

`ポート`

リクエストボディの読み書きに使用するポート番号（存在する場合）。

スキーム手順: **read-request** ポート \[meta='()\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002drequest)

ポートからHTTPリクエストを読み取り、必要に応じて指定されたメタデータmetaを添付します。

副作用として、ポートのエンコーディングがISO-8859-1（latin-1）に設定されるため、1文字を読み取ると1バイトが読み取られます。詳細については、上記の文字セットに関する説明を参照してください。

リクエスト本文はリクエストの一部ではないことに注意してください。リクエストを読み込んだ後は、本文を別途読み込むことができます。リクエストの書き込みについても同様です。

スキーム手順: **build-request** uri \[#:method='GET\] \[#:version='(1 . 1)\] \[#:headers='()\] \[#:port=#f\] \[#:meta='()\] \[#:validate-headers?=#t\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-build_002drequest)

HTTPリクエストオブジェクトを構築します。validate-headers?がtrueの場合、各ヘッダーはそれぞれのバリデーターを通して検証されます。

スキーム手順: **write-request** r port [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-write_002drequest)

指定されたHTTPリクエストをポートに書き込みます。

新しいリクエストを返します。そのリクエストの`request-port`は、おそらく何らかの転送エンコーディングを使用して、ポートへの書き込みを継続します。

Scheme Procedure: **read-request-body** r [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002drequest_002dbody)

リクエストボディをバイトベクトルとしてrから読み込みます。リクエストボディがない場合は`#f`を返します。

Scheme Procedure: **write-request-body** r bv [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-write_002drequest_002dbody)

バイトベクトルbvを、HTTPリクエストrに対応するポートに書き込みます。

HTTPリクエストに通常関連付けられる各種ヘッダーは、これらの専用アクセサーを使用してアクセスできます。解析されたヘッダーのフォーマットの詳細については、[HTTPヘッダー](https://doc.guix.gnu.org/guile/latest/en/guile.html#HTTP-Headers)を参照してください。

スキーム手順: **request-accept** request \[default='()\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002daccept)

スキーム手順: **request-accept-charset** request \[default='()\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002daccept_002dcharset)

スキーム手順: **request-accept-encoding** request \[default='()\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002daccept_002dencoding)

スキーム手順: **request-accept-language** request \[default='()\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002daccept_002dlanguage)

スキーム手順: **request-allow** request \[default='()\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002dallow)

スキーム手順: **request-authorization** request \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002dauthorization)

スキーム手順: **request-cache-control** request \[default='()\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002dcache_002dcontrol)

スキーム手順: **request-connection** request \[default='()\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002dconnection)

スキーム手順: **request-content-encoding** request \[default='()\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002dcontent_002dencoding)

スキーム手順: **request-content-language** request \[default='()\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002dcontent_002dlanguage)

スキーム手順: **request-content-length** リクエスト \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002dcontent_002dlength)

スキーム手順: **request-content-location** request \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002dcontent_002dlocation)

スキーム手順: **request-content-md5** リクエスト \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002dcontent_002dmd5)

スキーム手順: **request-content-range** request \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- request_002dcontent_002drange)

スキーム手順: **request-content-type** request \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002dcontent_002dtype)

スキーム手順: **request-date** リクエスト \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002ddate)

Scheme Procedure: **request-expect** request \[default='()\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002dexpect)

スキーム手順: **request-expires** request \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002dexpires)

スキーム手順: **request-from** request \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002dfrom)

スキーム手順: **request-host** request \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002dhost)

スキーム手順: **request-if-match** request \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002dif_002dmatch)

スキーム手順: **request-if-modified-since** request \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002dif_002dmodified_002dsince)

スキーム手順: **request-if-none-match** request \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002dif_002dnone_002dmatch)

Scheme Procedure: **request-if-range** request \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002dif_002drange)

スキーム手順: **request-if-unmodified-since** request \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002dif_002dunmodified_002dsince)

スキーム手順: **request-last-modified** リクエスト \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002dlast_002dmodified)

スキーム手順: **request-max-forwards** request \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002dmax_002dforwards)

Scheme Procedure: **request-pragma** request \[default='()\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002dpragma)

スキーム手順: **request-proxy-authorization** request \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002dproxy_002dauthorization)

スキーム手順: **request-range** request \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002drange)

スキーム手順: **request-referer** request \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002dreferer)

スキーム手順: **request-te** request \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002dte)

スキーム手順: **request-trailer** request \[default='()\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002dtrailer)

スキーム手順: **request-transfer-encoding** request \[default='()\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002dtransfer_002dencoding)

スキーム手順: **request-upgrade** request \[default='()\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002dupgrade)

スキーム手順: **request-user-agent** request \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002duser_002dagent)

スキーム手順: **request-via** request \[default='()\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002dvia)

スキーム手順: **request-warning** request \[default='()\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002dwarning)

指定されたリクエストヘッダーを返します。ヘッダーが存在しない場合はデフォルト値を返します。

スキーム手順: **request-absolute-uri** r \[default-host=#f\] \[default-port=#f\] \[default-scheme=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-request_002dabsolute_002duri)

`host`ヘッダーとデフォルトのスキーム、ホスト、ポートを使用して、リクエストの絶対URIを決定するヘルパールーチン。デフォルトのスキームがなく、URI自体が絶対URIでない場合は、エラーが通知されます。

* * *

次へ: [Webクライアント](https://doc.guix.gnu.org/guile/latest/en/guile.html#Web-Client)、前: [HTTPリクエスト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Requests)、上: [HTTP、Web、その他](https://doc.guix.gnu.org/guile/latest/en/guile.html#Web) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.3.7 HTTPレスポンス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#HTTP-Responses)

(use-modules (web response))

リクエストと同様に（[HTTPリクエスト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Requests)を参照）、GuileはHTTPレスポンス用のデータ型を提供します。ここでも、ボディはリクエストとは別に表現されます。

Scheme Procedure: **response?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-response_003f)

スキーム手順: **response-version** レスポンス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-response_002dversion)

スキーム手順: **response-code** レスポンス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-response_002dcode)

スキーム手順: **response-reason-phrase** レスポンス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-response_002dreason_002dphrase)

スキーム手順: **response-headers** レスポンス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-response_002dheaders)

スキーム手順: **response-port** レスポンス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-response_002dport)

レスポンスタイプの述語とフィールドアクセサー。フィールドは以下のとおりです。

`バージョン`

HTTP バージョンのペア、例えば `(1 . 1)`。

`コード`

HTTPレスポンスコード（例：`200`）。

理由フレーズ

理由を示すフレーズ、または応答コードに対応する標準的な理由を示すフレーズ。

`ヘッダー`

レスポンスヘッダーは、解析された値のリストとして返されます。

`ポート`

レスポンスボディの読み書きに使用するポート（存在する場合）。

スキーム手順: **read-response** ポート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002dresponse)

ポートからHTTPレスポンスを読み取ります。

副作用として、ポートのエンコーディングがISO-8859-1（latin-1）に設定されるため、1文字を読み取ると1バイトが読み取られます。詳細については、[HTTPレスポンス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Responses)の文字セットに関する説明を参照してください。

スキーム手順: **build-response** \[#:version='(1 . 1)\] \[#:code=200\] \[#:reason-phrase=#f\] \[#:headers='()\] \[#:port=#f\] \[#:validate-headers?=#t\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-build_002dresponse)

HTTPレスポンスオブジェクトを構築します。validate-headers?がtrueの場合、各ヘッダーはそれぞれのバリデーターを通して検証されます。

スキーム手順: **adapt-response-version** レスポンスバージョン [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-adapt_002dresponse_002dversion)

指定されたレスポンスを別のHTTPバージョンに合わせて調整し、新しいHTTPレスポンスを返します。

このアイデアは、多くのアプリケーションがデフォルトのHTTPバージョンに対して応答を生成するだけで済む場合を想定しており、このメソッドは古いHTTPバージョン（0.9および1.0）に対応するために、プログラムによる様々な変換を処理できるというものです。しかし現状では、この機能はバージョンフィールドを更新するだけで、やや大雑把な処理になっています。

Scheme Procedure: **write-response** r port [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-write_002dresponse)

指定されたHTTPレスポンスをポートに書き込みます。

新しいレスポンスを返します。そのレスポンスの`response-port`は、おそらく何らかの転送エンコーディングを使用して、ポートへの書き込みを継続します。

Scheme Procedure: **response-must-not-include-body?** r [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-response_002dmust_002dnot_002dinclude_002dbody_003f)

ステータスコード304などの一部のレスポンスは、ボディを持たないように指定されています。この述語は、そのようなレスポンスに対して`#t`を返します。

ただし、`HEAD`リクエストに対するレスポンスにもボディを含めてはならないことに注意してください。

スキーム手順: **response-body-port** r \[#:decode?=#t\] \[#:keep-alive?=#t\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-response_002dbody_002dport)

r の本体を読み取ることができる入力ポートを返します。返されるポートのエンコードは、r の `content-type` ヘッダーに従って設定されます。ただし、`decode?` が `#f` の場合は例外です。本体が利用できない場合は `#f` を返します。

keep-alive? が `#f` の場合、返されたポートを閉じると、r の応答ポートも閉じられます。

Scheme Procedure: **read-response-body** r [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002dresponse_002dbody)

レスポンスボディをバイトベクトルとしてrから読み込みます。レスポンスボディがない場合は`#f`を返します。

スキーム手順: **write-response-body** r bv [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-write_002dresponse_002dbody)

HTTPレスポンスrに対応するポートに、バイトベクトルbvを書き込みます。

リクエストと同様に、HTTPレスポンスに通常関連付けられるさまざまなヘッダーは、これらの専用アクセサーを使用してアクセスできます。解析されたヘッダーのフォーマットの詳細については、[HTTPヘッダー](https://doc.guix.gnu.org/guile/latest/en/guile.html#HTTP-Headers)を参照してください。

スキーム手順: **response-accept-ranges** レスポンス \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-response_002daccept_002dranges)

スキーム手順: **response-age** レスポンス \[default='()\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-response_002dage)

スキーム手順: **response-allow** レスポンス \[default='()\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-response_002dallow)

スキーム手順: **response-cache-control** response \[default='()\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-response_002dcache_002dcontrol)

スキーム手順: **response-connection** レスポンス \[default='()\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-response_002dconnection)

スキーム手順: **response-content-encoding** response \[default='()\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-response_002dcontent_002dencoding)

Scheme Procedure: **response-content-language** response \[default='()\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-response_002dcontent_002dlanguage)

スキーム手順: **response-content-length** レスポンス \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-response_002dcontent_002dlength)

スキーム手順: **response-content-location** response \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-response_002dcontent_002dlocation)

スキーム手順: **response-content-md5** レスポンス \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-response_002dcontent_002dmd5)

スキーム手順: **response-content-range** レスポンス \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-response_002dcontent_002drange)

スキーム手順: **response-content-type** response \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-response_002dcontent_002dtype)

スキーム手順: **response-date** レスポンス \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-response_002ddate)

スキーム手順: **response-etag** レスポンス \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-response_002detag)

スキーム手順: **response-expires** response \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-response_002dexpires)

スキーム手順: **response-last-modified** レスポンス \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-response_002dlast_002dmodified)

スキーム手順: **response-location** response \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-response_002dlocation)

Scheme Procedure: **response-pragma** response \[default='()\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-response_002dpragma)

スキーム手順: **response-proxy-authenticate** レスポンス \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-response_002dproxy_002dauthenticate)

スキーム手順: **response-retry-after** レスポンス \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-response_002dretry_002dafter)

スキーム手順: **response-server** レスポンス \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-response_002dserver)

スキーム手順: **response-trailer** レスポンス \[default='()\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-response_002dtrailer)

スキーム手順: **response-transfer-encoding** response \[default='()\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-response_002dtransfer_002dencoding)

スキーム手順: **response-upgrade** レスポンス \[default='()\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-response_002dupgrade)

スキーム手順: **response-vary** response \[default='()\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-response_002dvary)

スキーム手順: **response-via** response \[default='()\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-response_002dvia)

スキーム手順: **response-warning** レスポンス \[default='()\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-response_002dwarning)

スキーム手順: **response-www-authenticate** レスポンス \[default=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-response_002dwww_002dauthenticate)

指定されたレスポンスヘッダーを返します。ヘッダーが存在しない場合はデフォルト値を返します。

Scheme Procedure: **text-content-type?** type [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-text_002dcontent_002dtype_003f)

`response-content-type` によって返されるシンボルである type が、`text/plain` などのテキスト型を表す場合は、`#t` を返します。

* * *

次へ: [Webサーバー](https://doc.guix.gnu.org/guile/latest/en/guile.html#Web-Server)、前: [HTTPレスポンス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Responses)、上: [HTTP、Web、その他](https://doc.guix.gnu.org/guile/latest/en/guile.html#Web) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.3.8 Webクライアント [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Web-Client-1)

`(Webクライアント)`は、下位レベルのHTTP、リクエスト、レスポンスモジュールに基づいて構築された、シンプルで同期的なHTTPクライアントを提供します。

(use-modules (web client))

スキーム手順: **open-socket-for-uri** uri \[#:verify-certificate? #t\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-open_002dsocket_002dfor_002duri)

URIへの接続用に開いている入出力ポートを返します。GuileはHTTPSをサポートするためにGuile-GnuTLSを動的にロードします。

詳細については、[Guile-GnuTLS の Web サイト](https://gitlab.com/gnutls/guile/) および GnuTLS-Guile の [Guile 用の GnuTLS バインディングのインストール方法](https://doc.guix.gnu.org/guile/latest/en/gnutls-guile.html#Guile-Preparations) を参照してください。

verify-certificate? が true の場合、サーバーの X.509 証明書を `x509-certificate-directory` から読み取った証明書と照合して検証します。エラーが発生した場合（たとえば、サーバーの証明書の有効期限が切れている、ホスト名が一致しないなど）、`tls-certificate-error` 例外を発生させます。`tls-certificate-error` 例外の引数は次のとおりです。

1. 失敗の原因を示す記号。証明書のホスト名がサーバーのホスト名と一致しない場合は「host-mismatch」、その他の原因の場合は「invalid-certificate」。
2. サーバーの X.509 証明書 (GnuTLS-Guile の [GnuTLS Guile リファレンス](https://doc.guix.gnu.org/guile/latest/en/gnutls-guile.html#Guile-Reference) を参照)。
3. サーバーのホスト名（文字列）
4. `invalid-certificate` エラーの場合、GnuTLS 証明書ステータス値のリスト（`certificate-status/` 定数のいずれか、例えば `certificate-status/signer-not-found` または `certificate-status/revoked`）。

スキーム手順: **http-request** uri arg… [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-http_002drequest)

URIに対応するサーバーに接続し、HTTPメソッド（`GET`、`HEAD`、`POST`など）を使用してリクエストを行います。

以下のキーワード引数を使用すると、リクエストに本文を追加したり、特定のヘッダーを設定したりするなど、さまざまな方法でリクエストを変更できます。以下の表に、キーワード引数とそのデフォルト値を示します。

`#:method 'GET`

`#:body #f`

`#:verify-certificate? #t`

`#:port (open-socket-for-uri uri #:verify-certificate? verify-certificate?)`

`#:version '(1 . 1)`

`#:keep-alive? #f`

`#:headers '()`

`#:decode-body? #t`

`#:streaming? #f`

既にポートが開いている場合は、それをポート番号として渡してください。そうでない場合は、URIに対応するサーバーへの接続が開かれます。alistヘッダーに含まれる追加のヘッダーは、リクエストに追加されます。

bodyが`#f`でない場合、HTTPリクエストとともにメッセージ本文も送信されます。bodyが文字列の場合、ヘッダーのcontent-typeに従ってエンコードされ、デフォルトはUTF-8です。それ以外の場合は、bodyはバイトベクトルであるか、本文がない場合は`#f`である必要があります。メッセージ本文はどのリクエストにも送信できますが、通常は`POST`リクエストと`PUT`リクエストのみに本文が含まれます。

デフォルト値である「decode-body?」がtrueの場合、レスポンスの本文がテキストコンテンツタイプであれば、文字列にデコードされます。そうでない場合は、バイトベクトルとして返されます。

ただし、streaming? が true の場合、この関数はサーバーからのレスポンスボディを即座に読み込むのではなく、ヘッダーのみを読み取ります。レスポンスボディは、データを読み取ることができるポートとして返されます。

keep-alive? が true でない限り、応答本文全体が読み込まれた後にポートは閉じられます。

port が false で、uri が HTTPS URL を示し、verify-certificate? が true の場合、`x509-certificate-directory` にある証明書と X.509 証明書を照合します。

サーバーから読み取ったレスポンスと、レスポンスボディを文字列、バイトベクター、#f 値、またはポート (streaming? が true の場合) として返します。

スキーム手順: **http-get** uri arg… [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-http_002dget)

スキーム手順: **http-head** uri arg… [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-http_002dhead)

スキーム手順: **http-post** uri arg… [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-http_002dpost)

スキーム手順: **http-put** uri arg… [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-http_002dput)

Scheme Procedure: **http-delete** uri arg… [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-http_002ddelete)

スキーム手順: **http-trace** uri arg… [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-http_002dtrace)

スキーム手順: **http-options** uri arg… [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-http_002doptions)

URIに対応するサーバーに接続し、適切なメソッド（`GET`、`HEAD`、`POST`など）を使用してHTTP経由でリクエストを行います。

これらの手順は、特定のメソッド引数で特殊化された `http-request` のバリアントであり、同じプロトタイプを持ちます。つまり、URI の後にオプションのキーワード引数のシーケンスが続きます。さまざまなキーワード引数の詳細については、[http-request](https://doc.guix.gnu.org/guile/latest/en/guile.html#http_002drequest) を参照してください。

スキームパラメータ: **x509-certificate-directory** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-x509_002dcertificate_002ddirectory)

このパラメータは、HTTPS接続用のX.509証明書を検索するディレクトリの名前を指定します。

デフォルト値は以下のいずれかです。

* 環境変数 `GUILE_TLS_CERTIFICATE_DIRECTORY` の値。
* または、環境変数 `SSL_CERT_DIR` の値 (OpenSSL ライブラリでも有効)。
* または、最終手段として `"/etc/ssl/certs"`。

X.509証明書は、`open-socket-for-uri`、`http-request`、または関連する手順の`#:verify-certificate?`引数がtrueの場合に、リモートサイトのIDを認証する際に使用されます。

`http-get` は、Web サイトへの単発のリクエストを行う際に便利です。Web スパイダーや、多数のリクエストを並行して処理する必要のある他のクライアントを作成する場合は、Web サーバーと同様の構造を持つイベント駆動型の URL フェッチャーを構築する方が良いでしょう ([Web サーバー](https://doc.guix.gnu.org/guile/latest/en/guile.html#Web-Server) を参照)。

別の選択肢としては、スレッドを使用する方法があり、これは良い方法ではあるが、パフォーマンスはそれほど高くない。おそらくpar-mapやfuturesを介して行うことになるだろう。

スキームパラメータ: **current-http-proxy** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-current_002dhttp_002dproxy)

スキームパラメータ: **current-https-proxy** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-current_002dhttps_002dproxy)

`#f` または、`open-socket-for-uri` を含む `(web client)` モジュール内のプロシージャで使用される HTTP または HTTPS プロキシ サーバーの URL を含む空でない文字列を指定します。初期値は、環境変数 `http_proxy` および `https_proxy` に基づいて決定されます。

(current-http-proxy) ⇒ "http://localhost:8123/"
(parameterize ((current-http-proxy #f))
(http-get "http://example.com/")) ; 一時的にプロキシをバイパスする
(current-http-proxy) ⇒ "http://localhost:8123/"

* * *

次へ: [Web の例](https://doc.guix.gnu.org/guile/latest/en/guile.html#Web-Examples)、前: [Web クライアント](https://doc.guix.gnu.org/guile/latest/en/guile.html#Web-Client)、上: [HTTP、Web、その他](https://doc.guix.gnu.org/guile/latest/en/guile.html#Web) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.3.9 Webサーバー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Web-Server-1)

`(web server)` は汎用的なウェブサーバーインターフェースであり、Guile によって制御されるウェブサーバーのためのメインループ実装も含まれています。

(use-modules (Webサーバー))

最下層は`<server-impl>`オブジェクトで、サーバーの起動、クライアントからのリクエストの読み取り、クライアントへのレスポンスの書き込み、サーバーのクローズを行うためのフックセットを定義します。これらのフック（それぞれ`open`、`read`、`write`、`close`）は、`<server-impl>`オブジェクトにまとめられています。このモジュールのプロシージャは、必要に応じて`<server-impl>`オブジェクトを受け取ります。

`<server-impl>` は名前で検索することもできます。`run-server` に `http` シンボルを渡すと、Guile は `(web server http)` モジュール内で `http` という名前の変数を検索します。この変数は `<server-impl>` オブジェクトにバインドされている必要があります。このようなバインドは、`define-server-impl` 構文をインスタンス化することで行われます。このようにして、run-server ループは利用可能な他のバックエンドを自動的にロードできます。

サーバーのライフサイクルは以下のとおりです。

1. サーバーを開くために、`open`フックが呼び出されます。`open`は、バックエンドに応じて0個以上の引数を取り、不透明なサーバーソケットオブジェクトを返すか、エラーを通知します。
2. `read`フックが呼び出され、新しいクライアントからのリクエストが読み取られます。`read`フックは、サーバーソケットを引数として1つ取ります。不透明なクライアントソケット、リクエスト、リクエストボディの3つの値を返す必要があります。リクエストは、`(web request)`からの`<request>`オブジェクトである必要があります。ボディは文字列またはバイトベクターである必要があります。ボディがない場合は`#f`になります。
    
読み取りが失敗した場合、`read`フックはクライアントソケット、リクエスト、およびボディに対して#fを返す可能性があります。
    
3. リクエストとボディを引数として、ユーザーが提供するハンドラプロシージャが呼び出されます。ハンドラは、`(web response)` からの `<response>` レコードとしてのレスポンスと、バイトベクターとしてのレスポンスボディ（存在しない場合は `#f`）の 2 つの値を返す必要があります。
    
レスポンスとレスポンスボディは、以下で説明する `sanitize-response` によって処理されます。これにより、ハンドラー作成者はいくつかの便利なショートカットを使用できます。たとえば、`<response>` の代わりに、ヘッダーのリストを返すだけで、デフォルトのレスポンスオブジェクトがそれらのヘッダーで構築されます。ボディのバイトベクターの代わりに、ハンドラーは文字列を返すことができ、これは適切なエンコーディングにシリアル化されます。または、プロシージャを返すこともでき、これはポートで呼び出されてデータを書き出します。詳細については、`sanitize-response` のドキュメントを参照してください。
    
4. `write`フックは、クライアントソケット、レスポンス、およびボディの3つの引数で呼び出されます。`write`フックは値を返しません。
5. この時点でリクエストの処理は完了です。ループの場合は、ループを戻って新しいリクエストを読み取ろうとします。
6. ユーザーがループを中断した場合、サーバーソケットの `close` フックが呼び出されます。

ユーザーは、以下の形式でサーバー実装を定義できます。

Scheme構文: **define-server-impl** name open read write close [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-define_002dserver_002dimpl)

open、read、write、close のフックを持つ `<server-impl>` オブジェクトを作成し、それを現在のモジュールのシンボル名にバインドします。

Scheme Procedure: **lookup-server-impl** impl [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-lookup_002dserver_002dimpl)

サーバー実装を検索します。`impl` が既にサーバー実装である場合は、それが直接返されます。シンボルである場合は、`(web server impl)` モジュール内の `impl` という名前のバインディングが検索されます。それ以外の場合は、エラーが通知されます。

現在、サーバーの実装はやや不透明な型であり、`read-client` のようなこのモジュール内の他のプロシージャに渡す場合にのみ役立ちます。

`(web server)`モジュールは、`<server-impl>`オブジェクトを使用してWebサーバーの一部を実装するルーチンをいくつか定義しています。`<server-impl>`のさまざまなフィールドへのアクセサを公開していないため、実際にはこれらのルーチンがimplオブジェクトにアクセスできる唯一のプロシージャとなります。

Scheme Procedure: **open-server** impl open-params [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-open_002dserver)

指定された実装に対してサーバーを開きます。新しいサーバーオブジェクトを1つの値として返します。実装の`open`プロシージャがopen-params（リストである必要があります）に適用されます。

Scheme 手順: **read-client** 実装サーバー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002dclient)

サーバーから新しいクライアントを読み込むには、実装の `read` プロシージャをサーバーに適用します。成功した場合は、クライアントに対応するオブジェクト、リクエストオブジェクト、およびリクエストボディの 3 つの値を返します。例外が発生した場合は、3 つの値すべてに対して `#f` を返します。

Scheme Procedure: **handle-request** ハンドラーリクエストボディの状態 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-handle_002drequest)

指定されたリクエストを処理し、レスポンスとリクエストボディを返します。

リクエストとレスポンスボディを引数として指定されたハンドラを呼び出すことで、レスポンスとレスポンスボディが生成されます。

状態の要素は引数としてハンドラに渡され、追加の値として返される場合もあります。ハンドラの戻り値から収集された新しい状態は、リストとして返されます。この仕組みは、サーバーループがユーザーからハンドラと、ユーザーが関心を持つ状態値を受け取り、ユーザーのハンドラが自身の状態を明示的に管理できるようにするというものです。

スキーム手順: **sanitize-response** リクエストレスポンスボディ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sanitize_002dresponse)

提示された応答と本文を「サニタイズ」し、提示された要求にふさわしいものにする。

ウェブハンドラーの作成者の便宜を図るため、レスポンスはヘッダーのリストとして指定できます。この場合、デフォルトのレスポンスの構築に使用されます。レスポンスのバージョンがリクエストのバージョンと一致することを保証します。本文が文字列の場合、レスポンスに適したエンコーディングで文字列をバイトベクトルにエンコードします。必要に応じて、`content-length` および `content-type` ヘッダーを追加します。

bodyがプロシージャの場合、ポートを引数として呼び出し、出力はバイトベクトルとして収集されます。将来的には、圧縮されたチャンクエンコードポートを使用し、このプロシージャを書き込みクライアントプロシージャ内で後から呼び出すことを検討しています。プロシージャが特定のタイミングで呼び出されることを前提としないよう、開発者は注意してください。

Scheme 手順: **write-client** 実装サーバークライアント応答ボディ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-write_002dclient)

クライアントに対してHTTPレスポンスとボディを書き込む。サーバーとクライアントが永続接続をサポートしている場合、その後は実装側がクライアントを追跡する責任を負い、おそらく何らかの方法でサーバー引数にクライアントを付加することになるだろう。

Scheme Procedure: **close-server** impl server [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-close_002dserver)

`open-server`の以前の呼び出しによって割り当てられたリソースを解放します。

上記の手順に従えば、ウェブサーバーを作成するのは簡単なことです。

Scheme Procedure: **serve-one-client** ハンドラ実装サーバー状態 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-serve_002done_002dclient)

サーバーからリクエストを1件読み込み、リクエストとボディに対してハンドラを呼び出し、レスポンスをクライアントに書き込みます。ハンドラ処理によって生成された新しい状態を返します。

Scheme Procedure: **run-server** handler \[impl='http\] \[open-params='()\] arg … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-run_002dserver-1)

Guileの内蔵ウェブサーバーを実行します。

ハンドラーは、HTTPリクエストとリクエストボディという2つ以上の引数を受け取り、レスポンスとレスポンスボディという2つ以上の値を返すプロシージャである必要があります。

例については、次のセクション [Web の例](https://doc.guix.gnu.org/guile/latest/en/guile.html#Web-Examples) を参照してください。

レスポンスとレスポンスボディは、クライアントに返送される前に`sanitize-response`によって処理されます。

ハンドラへの追加引数は arg ... から取得されます。これらの引数は _state_ を構成します。追加の戻り値は新しい状態に蓄積され、後続のリクエストに使用されます。このようにして、ハンドラは自身の状態を明示的に管理できます。

デフォルトのWebサーバー実装は`http`であり、ソケットにバインドして、そのポートでリクエストを待ち受けます。

HTTP 実装: **http** \[#:host=#f\] \[#:family=AF\_INET\] \[#:addr=INADDR\_LOOPBACK\] \[#:port 8080\] \[#:socket\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-http)

デフォルトのHTTP実装です。キーワード引数を持つ関数としてドキュメント化していますが、これはまさにその通りだからです。`run-server`へのすべてのopen-paramsは、この実装のopen関数に渡されます。

;; デフォルト: localhost:8080
(サーバー実行ハンドラ)
;; 同じこと
(run-server handler 'http '())
;; 別のポートで
(run-server handler 'http '(#:port 8081))
IPv6
(run-server handler 'http '(#:family AF\_INET6 #:port 8081))
;; カスタムソケット
(run-server handler 'http \`(#:socket ,(sudo-make-me-a-socket)))

* * *

前へ: [Webサーバー](https://doc.guix.gnu.org/guile/latest/en/guile.html#Web-Server)、上へ: [HTTP、Web、その他](https://doc.guix.gnu.org/guile/latest/en/guile.html#Web) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.3.10 Web の例 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Web-Examples-1)

さて、面倒な内部構造の話はこれくらいにして、ウェブアプリケーションを作ってみましょう！

* [Hello, World!](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hello_002c-World_0021)
* [リクエストの検査](https://doc.guix.gnu.org/guile/latest/en/guile.html#Inspecting-the-Request)
* [高レベルインターフェース](https://doc.guix.gnu.org/guile/latest/en/guile.html#Higher_002dLevel-Interfaces)
* [結論](https://doc.guix.gnu.org/guile/latest/en/guile.html#結論)

#### 7.3.10.1 こんにちは、世界！ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hello_002c-World_0021)

まず最初に書くべきプログラムは、もちろん「Hello, World!」です。つまり、私たちが望む動作をするウェブハンドラーを実装する必要があるということです。

次に、2つの引数と2つの戻り値を持つ関数であるハンドラを定義します。

(define (handler request request-body)
(値、応答、応答本文)

最初の例では、ショートカットを利用して、適切なレスポンスオブジェクトではなくヘッダーのリストを返します。レスポンスボディはペイロードです。

(define (hello-world-handler request request-body)
(値 '((content-type . (text/plain)))
"こんにちは世界！"））

それでは、このハンドラを使ってサーバーを実行してテストしてみましょう。まだWebサーバーモジュールをロードしていない場合はロードし、このハンドラを使ってサーバーを実行してください。

(use-modules (Webサーバー))
(run-server hello-world-handler)

デフォルトでは、Webサーバーは`localhost:8080`でリクエストを待ち受けます。Webブラウザでそのアドレスにアクセスしてテストしてください。`Hello World!`という文字列が表示されれば成功です！

#### 7.3.10.2 リクエストの検査 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Inspecting-the-Request)

上記の「Hello World」プログラムは、あらゆるURIに応答する汎用的なグリーターです。より限定的なグリーターを作成するには、リクエストオブジェクトを検査し、条件に応じて異なる結果を生成する必要があります。そこで、リクエスト、レスポンス、URIモジュールをロードして、まさにそれを行いましょう。

(use-modules (web server)) ; おそらく既に実行済みでしょう
(use-modules (Webリクエスト)
（ウェブからの応答）
(ウェブURI)

(define (request-path-components request)
(分割およびデコードURIパス(URIパス(リクエストURIリクエスト))))

(define (hello-hacker-handler リクエストボディ)
(if (equal? (request-path-components request)
'（"ハッカー"））
(値 '((content-type . (text/plain)))
「やあ、ハッカー！」
(リクエストが見つかりませんでした)))

(run-server hello-hacker-handler)

ここでは、URIパスの構成要素を文字列のリストとして返すヘルパー関数を定義し、それを使って`/hacker/`へのリクエストをチェックしていることがわかります。成功した場合の手順は以前と同じです。ブラウザで`http://localhost:8080/hacker/`にアクセスして確認してください。

常に`split-and-decode-uri-path`でデコードされたURIパスコンポーネントと照合する必要があります。上記の例は、`/hacker/`、`//hacker///`、および`/h%61ck%65r`で機能します。

しかし、`not-found` を定義するのを忘れていました！これらの例を REPL に貼り付けて、Web ブラウザで他の URI にアクセスすると、Guile コンソールがデバッガに切り替わります。

<名前のないポート>:38:7: プロシージャ module-lookup 内:
<名前のないポート>:38:7: バインドされていない変数: 見つかりません

新しいプロンプトが表示されます。バックトレースを表示するには「,bt」と入力し、続行するには「,q」と入力してください。
scheme@(guile-user) \[1\]>

それでは、デバッガー上で関数を定義してみましょう。ご存じのとおり、404エラーを返すようにします。

;; REPLにこれを貼り付けてください
(define (not-found request)
(値 (ビルドレスポンス #:コード 404)
(string-append "リソースが見つかりません: "
(uri->string (request-uri request)))))

;; ウェブサーバーを稼働させ続けるには、これを貼り付けてください。
、続く

ここで`http://localhost/foo/`にアクセスすると、このエラーメッセージが表示されます。（ただし、一部の一般的なWebブラウザは、404メッセージ本文が十分に長くない限り、サーバーが生成した404メッセージを表示せず、独自のメッセージを表示することに注意してください。）

#### 7.3.10.3 高レベルインターフェース [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Higher_002dLevel-Interfaces)

Web ハンドラー インターフェースは、あらゆる種類の Guile Web アプリケーションで使用できる共通のベースラインです。ただし、特に HTML を生成する場合は、通常はその上に何かを構築する必要があります。以下は、SXML を使用して HTML 出力を構築する簡単な例です ([SXML](https://doc.guix.gnu.org/guile/latest/en/guile.html#SXML) を参照)。

まず、モジュールをロードします。

(use-modules (Webサーバー))
（ウェブリクエスト）
（ウェブからの応答）
(sxml シンプル)

次に、HTML本文要素のリストをSXML形式で受け取り、それをスーパーテンプレートに配置するシンプルなテンプレート関数を定義します。

(define (templateize title body)
\`(html (head (title ,title))
(body ,@body)))

例えば、最もシンプルな「Hello」HTMLは次のように生成できます。

(sxml->xml (templatize "Hello!" '((b "Hi!"))))
⊣
<html><head><title>こんにちは！</title></head><body><b>やあ！</b></body></html>

HTMLを文字列として扱うよりも、Schemeのデータ型を扱う方がはるかに良い。それでは、簡単なレスポンスヘルパーを定義しよう。

(define\* (respond #:optional body #:key
（ステータス200）
（タイトル「こんにちは、こんにちは！」）
(doctype "<!DOCTYPE html>\n")
(content-type-params '((charset . "utf-8")))
(コンテンツタイプ 'text/html')
(追加ヘッダー '())
(sxml (and body (templateize title body))))
(値 (ビルドレスポンス)
#:コードステータス
#:headers \`((content-type
. (,content-type ,@content-type-params))
、@extra-headers))
(ラムダ (ポート)
(sxmlの場合)
（始める
(ドキュメントタイプが指定されている場合、ドキュメントタイプのポートを表示します)
(sxml→xml sxmlポート))))))

ここでは、デフォルト初期化子を備えたキーワード引数の威力を見ることができます。引数が完全に解析される頃には、ローカル変数`sxml`にはテンプレート化されたSXMLが格納され、クライアントへの送信準備が整います。

また、`respond` はレスポンスボディを文字列として返す代わりに、Webサーバーが呼び出してクライアントにレスポンスを書き出すためのプロシージャを提供します。

それでは、このレスポンダーを使った簡単な例を見てみましょう。受信したヘッダーをHTMLテーブルにレイアウトします。

(define (debug-page request body)
（応答する
\`((h1 "こんにちは世界！")
（テーブル
(tr (th "ヘッダー") (th "値"))
,@(map (lambda (pair)
\`(tr (td (tt ,(with-output-to-string
(lambda () (display (car pair))))))
(td (tt ,(with-output-to-string
(ラムダ()
(書き込み (cdr ペア))))))))
(リクエストヘッダーリクエスト))))))

(run-server debug-page)

これで、ウェブブラウザで任意のローカルアドレスにアクセスすると、ようやくHTMLが表示されるようになりました。

#### 7.3.10.4 結論 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#結論)

さて、現時点では、Guileの組み込みWebサポートはここまでです。Webアプリケーションを作成する方法は数多くありますが、最も基本的なデータ型を標準化することで、ユーザーは自分に最適なアプローチを選択できるだけでなく、サーバーの実装を切り替えることも可能になるはずです。これはGuileの比較的新しい機能なので、ご意見やご感想がありましたらぜひお聞かせください。今後の開発に役立てさせていただきます。Webハッキングをお楽しみください！

* * *

次へ: [SRFI サポート モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI-Support)、前: [HTTP、Web、その他](https://doc.guix.gnu.org/guile/latest/en/guile.html#Web)、上: [Guile モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guile-Modules) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
