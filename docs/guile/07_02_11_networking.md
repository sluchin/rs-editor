#### 7.2.11 ネットワーク [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Networking-1)

* [ネットワークアドレス変換](https://doc.guix.gnu.org/guile/latest/en/guile.html#Network-Address-Conversion)
* [ネットワークデータベース](https://doc.guix.gnu.org/guile/latest/en/guile.html#Network-Databases)
* [ネットワークソケットアドレス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Network-Socket-Address)
* [ネットワークソケットと通信](https://doc.guix.gnu.org/guile/latest/en/guile.html#Network-Sockets-and-Communication)
* [ネットワークソケットの例](https://doc.guix.gnu.org/guile/latest/en/guile.html#Internet-Socket-Examples)

* * *

次へ: [ネットワークデータベース](https://doc.guix.gnu.org/guile/latest/en/guile.html#Network-Databases)、上へ: [ネットワーキング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Networking) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.2.11.1 ネットワークアドレス変換 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Network-Address-Conversion-1)

このセクションでは、インターネットアドレスを数値形式と文字列形式の間で変換する手順について説明します。

#### IPv4アドレス変換 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#IPv4-Address-Conversion)

IPv4インターネットアドレスは4バイトの値であり、Guileではホストバイトオーダーの整数として表現されます。例えば、「0.0.0.1」は1、「1.0.0.0」は16777216となります。

一部の基盤となるC関数はアドレスにネットワークバイトオーダーを使用しますが、Guileは必要に応じて変換を行い、Schemeレベルではホストバイトオーダーがどこでも使用されるようにします。

変数: **INADDR\_ANY** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-INADDR_005fANY)

サーバーの場合、これは `bind` ([ネットワークソケットと通信](https://doc.guix.gnu.org/guile/latest/en/guile.html#Network-Sockets-and-Communication) を参照) と組み合わせて使用することで、マシン上の任意のインターフェースからの接続を許可できます。

変数: **INADDR\_BROADCAST** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-INADDR_005fBROADCAST)

ローカルネットワーク上のブロードキャストアドレス。

変数: **INADDR\_LOOPBACK** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-INADDR_005fLOOPBACK)

ループバックデバイスを使用するローカルホストのアドレス、つまり「127.0.0.1」。

スキーム手順: **inet-netof** アドレス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-inet_002dnetof)

C 関数: **scm\_inet\_netof** (アドレス) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005finet_005fnetof)

指定された IPv4 インターネット アドレスのネットワーク番号部分を返します。例:

([inet-netof](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-inet_002dnetof) 2130706433) ⇒ 127

スキーム手順: **inet-lnaof** アドレス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-inet_002dlnaof)

C 関数: **scm\_lnaof** (アドレス) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005flnaof)

指定された IPv4 インターネット アドレスのローカル アドレスとネットワーク パートを、廃止されたクラス A/B/C システムを使用して返します。例:

([inet-lnaof](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-inet_002dlnaof) 2130706433) ⇒ 1

スキーム手順: **inet-makeaddr** net lna [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-inet_002dmakeaddr)

C 関数: **scm\_inet\_makeaddr** (net, lna) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005finet_005fmakeaddr)

IPv4インターネットアドレスは、ネットワーク番号netとネットワーク内ローカルアドレス番号lnaを組み合わせて作成します。例：

([inet-makeaddr](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-inet_002dmakeaddr) 127 1) ⇒ 2130706433

#### IPv6アドレス変換 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#IPv6-Address-Conversion)

IPv6インターネットアドレスは16バイトの値であり、Guileではホストバイトオーダーの整数として表現されます。例えば「::1」は1を表します。便宜上、以下の定数が定義されています。

変数: **IN6ADDR\_ANY** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-IN6ADDR_005fANY)

サーバーの場合、これは `bind` ([ネットワークソケットと通信](https://doc.guix.gnu.org/guile/latest/en/guile.html#Network-Sockets-and-Communication) を参照) と組み合わせて使用することで、マシン上の任意の IPv6 インターフェイスからの接続を許可できます。

変数: **IN6ADDR\_LOOPBACK** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-IN6ADDR_005fLOOPBACK)

ループバックデバイスを使用するローカルホストのアドレス、つまり「::1」。

以下の手順は、IPv6アドレスまたはIPv4アドレスをテキスト表現に変換し、またその逆の変換を行います。

スキーム手順: **inet-ntop** ファミリーアドレス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-inet_002dntop)

C 関数: **scm\_inet\_ntop** (family, address) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005finet_005fntop)

ネットワーク アドレスを整数から印刷可能な文字列に変換します。family は `AF_INET` または `AF_INET6` です。例:

([inet-ntop](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-inet_002dntop) AF\_INET 2130706433) ⇒ "127.0.0.1"
([inet-ntop](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-inet_002dntop) AF\_INET6 ([\-](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002d) ([expt](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-expt) 2 128) 1))
⇒ 「ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff」

スキーム手順: **inet-pton** ファミリーアドレス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-inet_002dpton)

C 関数: **scm\_inet\_pton** (family, address) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005finet_005fpton)

印刷可能なネットワーク アドレスを含む文字列を整数アドレスに変換します。family は `AF_INET` または `AF_INET6` です。例:

([inet-pton](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-inet_002dpton) AF\_INET "127.0.0.1") ⇒ 2130706433
([inet-pton](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-inet_002dpton) AF\_INET6 "::1") ⇒ 1

* * *

次へ: [ネットワークソケットアドレス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Network-Socket-Address)、前: [ネットワークアドレス変換](https://doc.guix.gnu.org/guile/latest/en/guile.html#Network-Address-Conversion)、上: [ネットワーク](https://doc.guix.gnu.org/guile/latest/en/guile.html#Networking) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.2.11.2 ネットワークデータベース [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Network-Databases-1)

このセクションでは、さまざまなネットワークデータベースに問い合わせを行う手順について説明します。データベースルーチンは再入可能ではないため、使用する際は注意が必要です。

#### `getaddrinfo` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#getaddrinfo)

`getaddrinfo`プロシージャは、ホスト名とサービス名をソケットアドレスおよび関連情報に、プロトコルに依存しない方法でマッピングします。

スキーム手順: **getaddrinfo** name service \[hint\_flags \[hint\_family \[hint\_socktype \[hint\_protocol\]\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-getaddrinfo)

C 関数: **scm\_getaddrinfo** (name, service, hint\_flags, hint\_family, hint\_socktype, hint\_protocol) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fgetaddrinfo)

指定されたサービスにアクセスするためのソケットを作成する際に使用する、ホスト名および/またはサービスに関するソケットアドレスと関連情報を含む`addrinfo`構造体のリストを返します。

(let\* ((ai (car (getaddrinfo "www.gnu.org" "http")))
(s (ソケット (addrinfo:fam ai) (addrinfo:socktype ai)
(addrinfo:protocol ai))))
(接続 s (addrinfo:addr ai))
s)

サービスが省略されている場合、または`#f`の場合は、nameに対応するネットワークレベルのアドレスを返します。nameが`#f`の場合は、サービスが提供されなければならず、呼び出し元にローカルなサービスロケーションが返されます。

追加のヒントを提供できます。指定する場合、hint_flags は、以下の定数の中から 0 個以上の定数をビット単位で論理和した結果である必要があります。

`AI_PASSIVE`

ソケットアドレスは`bind`に使用することを想定しています。

`AI_CANONNAME`

正規ホスト名のリクエスト。`addrinfo:canonname`で取得できます。これは主にDNSルックアップが関係する場合に有効です。

`AI_NUMERICHOST`

名前が数値のホストアドレス文字列（例：「127.0.0.1」）であることを指定します。これは、名前解決が使用されないことを意味します。

`AI_NUMERICSERV`

同様に、サービスが数値ポート文字列（例：「80」）であることを指定します。

`AI_ADDRCONFIG`

ローカルシステムで設定されているアドレスのみを返します。返されたソケットアドレスを使用して接続を行う場合は、このフラグを指定することを強くお勧めします。そうしないと、返されたアドレスの一部が到達不能であったり、サポートされていないプロトコルを使用している可能性があります。

`AI_V4MAPPED`

IPv6アドレスを検索する際に、利用可能なIPv6アドレスがまったくない場合は、マッピングされたIPv4アドレスを返す。

`AI_ALL`

IPv6 アドレスを検索する際に、このフラグが `AI_V4MAPPED` と共に設定されている場合、すべての IPv4 アドレスに加えてすべての IPv6 アドレスが返され、後者は IPv6 形式にマッピングされます。

hint\_family が指定されている場合は、要求されたアドレス ファミリ (例: `AF_INET6`) を指定する必要があります。同様に、hint\_socktype は要求されたソケット タイプ (例: `SOCK_DGRAM`) を指定し、hint\_protocol は要求されたプロトコルを指定する必要があります (その値は `socket` 呼び出しの場合と同様に解釈されます)。

エラーが発生した場合、キー「getaddrinfo-error」を持つ例外がスローされ、その引数としてエラーコード（整数）が渡されます。

(catch 'getaddrinfo-error
(ラムダ()
(getaddrinfo "www.gnu.org" "gopher"))
(lambda (key errcode)
(条件 ((= エラーコード EAI\_SERVICE)
	(「Gopherについて知りません！\n」と表示)
	((= エラーコード EAI\_NONAME)
	(「www.gnu.org が見つかりません\n」と表示)
	（それ以外
	(フォーマット #t "何かが間違っています: ~a\\n"
		(gai-strerror エラーコード))))))

エラーコードは以下のとおりです。

`EAI_AGAIN`

現時点では、名称またはサービスを特定できませんでした。今後の試行で解決する可能性があります。

`EAI_BADFLAGS`

hint_flags に無効な値が含まれています。

`EAI_FAIL`

名前解決中に回復不能なエラーが発生しました。

`EAI_FAMILY`

ヒント_family は認識されませんでした。

`EAI_NONAME`

指定されたパラメータに対して名前が解決されないか、名前もサービスも指定されていません。

`EAI_NODATA`

この非POSIXエラーコードは、一部のシステム（少なくともGNUとDarwin）で返される可能性があります。例えば、名前はわかっているものの、要求したデータが得られなかった場合などです。エラー処理コードは、このエラーコードが定義された時点で、適切に処理できるように準備しておく必要があります。

`EAI_SERVICE`

指定されたソケットタイプに対してサービスが認識されませんでした。

`EAI_SOCKTYPE`

hint_socktype が認識されませんでした。

`EAI_SYSTEM`

システムエラーが発生しました。C言語では、エラーコードは`errno`に格納されています。この値はSchemeからはアクセスできませんが、実際には実際のエラー原因に関する情報はほとんど得られません。

ユーザーは、詳細について["POSIX仕様"](http://www.opengroup.org/onlinepubs/9699919799/functions/getaddrinfo.html)を読むことをお勧めします。

以下の手順では、`getaddrinfo`によって返される`addrinfo`オブジェクトを使用します。

スキーム手順: **addrinfo:flags** ai [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-addrinfo_003aflags)

ai のフラグを `AI_` 値のビットごとの OR として返します (上記参照)。

スキーム手順: **addrinfo:fam** ai [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-addrinfo_003afam)

ai のアドレスファミリー (`AF_` 値) を返します。

スキーム手順: **addrinfo:socktype** ai [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-addrinfo_003asocktype)

ai のソケットタイプ (`SOCK_` 値) を返します。

スキーム手順: **addrinfo:protocol** ai [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-addrinfo_003aprotocol)

AIのプロトコルを返します。

スキーム手順: **addrinfo:addr** ai [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-addrinfo_003aaddr)

ai に関連付けられたソケット アドレスを `sockaddr` オブジェクトとして返します ([ネットワーク ソケット アドレス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Network-Socket-Address) を参照)。

スキーム手順: **addrinfo:canonname** ai [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-addrinfo_003acanonname)

`AI_CANONNAME`フラグが指定されている場合は、aiに関連付けられた正規名を表す文字列を返します。

#### ホストデータベース [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Host-Database)

ホストオブジェクトとは、ネットワークホストについて知られている情報を表す構造体であり、ソフトウェア内でシステムのネットワークIDを表す一般的な方法です。

以下の関数はホストオブジェクトを受け取り、選択されたコンポーネントを返します。

スキーム手順: **hostent:name** ホスト [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hostent_003aname)

ホストの「公式」ホスト名。

スキーム手順: **hostent:aliases** ホスト [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hostent_003aaliases)

ホストのエイリアス一覧。

スキーム手順: **hostent:addrtype** host [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hostent_003aaddrtype)

ホストアドレスのタイプ。`AF`定数のいずれか、例えば`AF_INET`や`AF_INET6`など。

スキーム手順: **hostent:length** host [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hostent_003alength)

ホストの各アドレスの長さ（バイト単位）。

スキーム手順: **hostent:addr-list** host [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hostent_003aaddr_002dlist)

ホストに関連付けられたネットワーク アドレスのリスト。`AF_INET` の場合、これらは整数型の IPv4 アドレスです ([ネットワーク アドレス変換](https://doc.guix.gnu.org/guile/latest/en/guile.html#Network-Address-Conversion) を参照)。

ホストデータベースを検索するには、以下の手順を使用できます。ただし、`getaddrinfo` の方が汎用性が高く、スレッドセーフであるため、そちらを使用することをお勧めします。

Scheme Procedure: **gethost** \[host\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-gethost)

Scheme Procedure: **gethostbyname** hostname [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-gethostbyname)

スキーム手順: **gethostbyaddr** アドレス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-gethostbyaddr)

C 関数: **scm\_gethost** (host) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fgethost)

名前またはアドレスでホストを検索し、ホストオブジェクトを返します。`gethost` プロシージャは、文字列の名前または整数のアドレスのいずれかを受け入れます。引数が指定されていない場合は、`gethostent` (下記参照) と同様の動作をします。名前またはアドレスが指定されているにもかかわらずアドレスが見つからない場合は、対応する `h_error` 値に対応する `host-not-found`、`try-again`、`no-recovery`、または `no-data` のいずれかのキーにエラーがスローされます。異常な状況では、`system-error` または `misc_error` キーにエラーがスローされる場合があります。

([gethost](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-gethost) "www.gnu.org")
⇒ #("www.gnu.org" () 2 4 (3353880842))

([gethostbyname](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-gethostbyname) "www.emacs.org")
⇒ #("emacs.org" ("www.emacs.org") 2 4 (1073448978))

以下の手順を使用すると、ホストデータベースを最初から最後まで順に処理できます。

スキーム手順: **sethostent** \[stayopen\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sethostent)

ホストオブジェクトを読み取ることができる内部ストリームを初期化します。このプロシージャは、`gethostent` を呼び出す前に必ず呼び出す必要があり、また、呼び出し後にホストエントリストリームをリセットするために呼び出すこともできます。stayopen が指定され、それが `#f` でない場合、後続の `gethostbyname` または `gethostbyaddr` 呼び出しによってデータベースが閉じられないため、効率が向上する可能性があります。

Scheme Procedure: **gethostent** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-gethostent)

ホストデータベースから次のホストオブジェクトを返します。ホストが見つからない場合（またはエラーが発生した場合）は、`#f`を返します。このプロシージャは、`sethostent`が呼び出される前に使用することはできません。

スキーム手順: **endhostent** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-endhostent)

`gethostent`で使用されるストリームを閉じます。戻り値は未指定です。

Scheme手順: **sethost** \[stayopen\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sethost)

C 関数: **scm\_sethost** (stayopen) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsethost)

stayopenを省略した場合、これは`endhostent`と同等です。それ以外の場合は、`sethostent stayopen`と同等です。

#### ネットワークデータベース [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Network-Database)

以下の関数は、ネットワークを表すオブジェクトを受け取り、選択されたコンポーネントを返します。

スキーム手順: **netent:name** net [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-netent_003aname)

「公式」ネットワーク名。

スキーム手順: **netent:aliases** net [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-netent_003aaliases)

ネットワークのエイリアスの一覧。

スキーム手順: **netent:addrtype** net [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-netent_003aaddrtype)

ネットワーク番号の型。現在、これは`AF_INET`のみを返します。

スキーム手順: **netent:net** net [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-netent_003anet)

ネットワーク番号。

ネットワークデータベースを検索するには、以下の手順を使用します。

Scheme Procedure: **getnet** \[net\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-getnet)

スキームプロシージャ: **getnetbyname** net-name [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-getnetbyname)

スキーム手順: **getnetbyaddr** ネット番号 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-getnetbyaddr)

C 関数: **scm\_getnet** (net) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fgetnet)

ネットワークデータベースで、ネットワーク名またはネットワーク番号を指定してネットワークを検索します。net-name引数は文字列、net-number引数は整数である必要があります。引数が指定されていない場合は、`getnet`はどちらのタイプの引数も受け入れ、`getnetent`（下記参照）と同様の動作をします。

以下の手順を用いることで、ネットワークデータベースを最初から最後まで順に調べることができます。

Scheme Procedure: **setnetent** \[stayopen\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-setnetent)

ネットワークオブジェクトを読み取ることができる内部ストリームを初期化します。このプロシージャは、`getnetent` を呼び出す前に必ず呼び出す必要があり、また、呼び出し後にネットワークエントリストリームをリセットするために呼び出すこともできます。stayopen が指定され、それが `#f` でない場合、後続の `getnetbyname` または `getnetbyaddr` 呼び出しによってデータベースが閉じられないため、効率が向上する可能性があります。

スキーム手順: **getnetent** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-getnetent)

ネットワークデータベースから次のエントリを返します。

スキーム手順: **endnetent** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-endnetent)

`getnetent`で使用されているストリームを閉じます。戻り値は未指定です。

スキーム手順: **setnet** \[stayopen\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-setnet)

C 関数: **scm\_setnet** (stayopen) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsetnet)

stayopenが省略された場合、これは`endnetent`と同等です。それ以外の場合は、`setnetent stayopen`と同等です。

#### プロトコルデータベース [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Protocol-Database)

以下の関数は、プロトコルを表すオブジェクトを受け取り、選択されたコンポーネントを返します。

スキーム手順: **protoent:name** プロトコル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-protoent_003aname)

「公式」プロトコル名。

スキーム手順: **protoent:aliases** プロトコル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-protoent_003aaliases)

プロトコルの別名の一覧。

スキーム手順: **protoent:proto** プロトコル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-protoent_003aproto)

プロトコル番号。

プロトコルデータベースを検索するには、以下の手順を使用します。

Scheme手順: **getproto** \[protocol\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-getproto)

Scheme手順: **getprotobyname** name [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-getprotobyname)

Scheme Procedure: **getprotobynumber** number [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-getprotobynumber)

C 関数: **scm\_getproto** (プロトコル) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fgetproto)

ネットワークプロトコルを名前または番号で検索します。`getprotobyname`は文字列引数を、`getprotobynumber`は整数引数を取ります。引数が指定されていない場合、`getproto`はどちらの型も受け入れ、`getprotoent`（下記参照）と同様の動作をします。

以下の手順を用いることで、プロトコルデータベースを最初から最後まで順に調べることができます。

Scheme手順: **setprotoent** \[stayopen\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-setprotoent)

プロトコルオブジェクトを読み取ることができる内部ストリームを初期化します。このプロシージャは、`getprotoent` を呼び出す前に必ず呼び出す必要があり、プロトコルエントリストリームをリセットするために呼び出し後にも呼び出すことができます。stayopen が指定され、それが `#f` でない場合、後続の `getprotobyname` または `getprotobynumber` 呼び出しによってデータベースが閉じられないため、効率が向上する可能性があります。

Scheme手順: **getprotoent** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-getprotoent)

プロトコルデータベースから次のエントリを返します。

スキーム手順: **endprotoent** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-endprotoent)

`getprotoent`で使用されるストリームを閉じます。戻り値は未指定です。

Scheme手順: **setproto** \[stayopen\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-setproto)

C 関数: **scm\_setproto** (stayopen) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsetproto)

stayopenが省略された場合、これは`endprotoent`と同等です。それ以外の場合は、`setprotoent stayopen`と同等です。

#### サービスデータベース [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Service-Database)

以下の関数は、サービスを表すオブジェクトを受け取り、選択されたコンポーネントを返します。

スキーム手順: **servent:name** serv [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-servent_003aname)

ネットワークサービスの「正式名称」。

スキーム手順: **servent:aliases** serv [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-servent_003aaliases)

ネットワークサービスの別名の一覧。

スキーム手順: **servent:port** serv [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-servent_003aport)

サービスが使用するインターネットポート。

Scheme Procedure: **servent:proto** serv [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-servent_003aproto)

サービスで使用されるプロトコル。データベースには、異なるプロトコル名で同じサービスが複数回登録される場合があります。

サービスデータベースを検索するには、以下の手順を使用します。

Scheme Procedure: **getserv** \[name \[protocol\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-getserv)

スキーム手順: **getservbyname** 名前プロトコル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-getservbyname)

スキーム手順: **getservbyport** ポートプロトコル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-getservbyport)

C 関数: **scm\_getserv** (名前、プロトコル) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fgetserv)

ネットワークサービスを名前またはサービス番号で検索し、ネットワークサービスオブジェクトを返します。protocol引数は、目的のプロトコル名を指定します。ネットワークサービスデータベースで見つかったプロトコルがこの名前と一致しない場合、システムエラーが通知されます。

`getserv`プロシージャは、最初の引数としてサービス名またはサービス番号を受け取ります。引数が指定されていない場合は、`getservent`と同様の動作をします（下記参照）。

([getserv](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-getserv) "imap" "tcp")
⇒ #("imap2" ("imap") 143 "tcp")

([getservbyport](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-getservbyport) 88 "udp")
⇒ #("kerberos" ("kerberos5" "krb5") 88 "udp")

以下の手順を使用すると、サービスデータベースを最初から最後まで順に調べることができます。

Scheme手順: **setservent** \[stayopen\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-setservent)

サービスオブジェクトを読み取ることができる内部ストリームを初期化します。このプロシージャは、`getservent` を呼び出す前に必ず呼び出す必要があり、サービスエントリストリームをリセットするために呼び出し後にも呼び出すことができます。`stayopen` が指定され、それが `#f` でない場合、後続の `getservbyname` または `getservbyport` 呼び出しによってデータベースが閉じられないため、効率が向上する可能性があります。

スキーム手順: **getservent** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-getservent)

サービスデータベースから次のエントリを返します。

スキーム手順: **endservent** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-endservent)

`getservent`で使用されるストリームを閉じます。戻り値は指定されていません。

Scheme手順: **setserv** \[stayopen\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-setserv)

C 関数: **scm\_setserv** (stayopen) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsetserv)

stayopenを省略した場合、これは`endservent`と同等です。それ以外の場合は、`setservent stayopen`と同等です。

* * *

次へ: [ネットワークソケットと通信](https://doc.guix.gnu.org/guile/latest/en/guile.html#Network-Sockets-and-Communication)、前: [ネットワークデータベース](https://doc.guix.gnu.org/guile/latest/en/guile.html#Network-Databases)、上: [ネットワーキング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Networking) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.2.11.3 ネットワークソケットアドレス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Network-Socket-Address-1)

_socket address_ オブジェクトは、通信用のソケットエンドポイントを識別します。たとえば `AF_INET` の場合、ソケットアドレスオブジェクトは、ホストアドレス (またはホスト上のインターフェース) と、実行中のクライアントまたはサーバープロセス内の特定の開いているソケットを指定するポート番号で構成されます。ソケットアドレスオブジェクトは、次のように作成できます。

スキーム手順: **make-socket-address** AF\_INET ipv4addr port [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dsocket_002daddress)

スキーム手順: **make-socket-address** AF\_INET6 ipv6addr port \[flowinfo \[scopeid\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dsocket_002daddress-1)

スキーム手順: **make-socket-address** AF\_UNIX パス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dsocket_002daddress-2)

C 関数: **scm\_make\_socket\_address** (family, address, arglist) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- scm_005fmake_005fsocket_005faddress)

新しいソケットアドレスオブジェクトを返します。最初の引数はアドレスファミリ（`AF`定数のいずれか）で、その後の引数はファミリによって異なります。

`AF_INET` の引数は、IPv4 ネットワーク アドレス番号 ([ネットワーク アドレス変換](https://doc.guix.gnu.org/guile/latest/en/guile.html#Network-Address-Conversion) を参照) とポート番号です。

`AF_INET6` の引数は、IPv6 ネットワークアドレス番号とポート番号です。オプションで、flowinfo と scopeid の引数も指定できます（いずれも整数値、デフォルト値は 0）。

`AF_UNIX` の場合、引数はファイル名（文字列）です。

C 関数 `scm_make_socket_address` は、family と address 引数を直接受け取り、arglist は、IPv4 の場合はポート、IPv6 の場合はポートとオプションの flowinfo および scopeid、Unix ドメインの場合は空のリスト `SCM_EOL` となる追加の引数のリストです。

以下の関数はソケットアドレスオブジェクトのフィールドにアクセスします。

スキーム手順: **sockaddr:fam** sa [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sockaddr_003afam)

ソケットアドレスオブジェクトsaからアドレスファミリを返します。これは`AF`定数の1つです（例：`AF_INET`）。

Scheme Procedure: **sockaddr:path** sa [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sockaddr_003apath)

`AF_UNIX`ソケットアドレスオブジェクトsaの場合、ファイル名を返します。

スキーム手順: **sockaddr:addr** sa [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sockaddr_003aaddr)

`AF_INET` または `AF_INET6` ソケットアドレスオブジェクト sa の場合、ネットワークアドレス番号を返します。

スキーム手順: **sockaddr:port** sa [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sockaddr_003aport)

`AF_INET` または `AF_INET6` ソケットアドレスオブジェクト sa の場合、ポート番号を返します。

スキーム手順: **sockaddr:flowinfo** sa [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sockaddr_003aflowinfo)

`AF_INET6`ソケットアドレスオブジェクトsaの場合、flowinfo値を返します。

スキーム手順: **sockaddr:scopeid** sa [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sockaddr_003ascopeid)

`AF_INET6`ソケットアドレスオブジェクトsaの場合、スコープID値を返します。

以下の関数は、C言語の`struct sockaddr`構造体との間で変換を行います（GNU Cライブラリリファレンスマニュアルの[アドレスフォーマット](https://doc.guix.gnu.org/libc/latest/en/libc.html#Address-Formats)を参照）。この構造体は汎用型であり、アプリケーションはアドレスファミリに応じて`struct sockaddr_in`、`struct sockaddr_in6`、または`struct sockaddr_un`との間でキャストできます。

取得または返される `struct sockaddr` では、フィールドのバイト順序は C の慣例に従います (GNU C ライブラリ リファレンス マニュアルの [バイト順序変換](https://doc.guix.gnu.org/libc/latest/en/libc.html#Byte-Order) を参照)。これは、`AF_INET` ホスト アドレス (`sin_addr.s_addr`) とポート番号 (`sin_port`)、および `AF_INET6` ポート番号 (`sin6_port`) がネットワーク バイト順序であることを意味します。しかし、Scheme レベルでは、これらの値はホスト バイト順序で取得または返されるため、ポートは通常の整数であり、ホスト アドレスも同様に通常の整数です ([ネットワーク アドレス変換](https://doc.guix.gnu.org/guile/latest/en/guile.html#Network-Address-Conversion) で説明されているとおりです)。

C 関数: `struct sockaddr *` **scm\_c\_make\_socket\_address** `(SCM ファミリ、SCM アドレス、SCM 引数、size_t *outsize)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- scm_005fc_005fmake_005fsocket_005faddress)

上記の`scm_make_socket_address`が受け取るような引数から作成された、新しく`malloc`された`struct sockaddr`を返します。

`struct sockaddr` の戻り値のサイズ（バイト単位）は `*outsize` に格納されます。アプリケーションは、不要になった戻り値の構造体を解放するために `free` を呼び出す必要があります。

C 関数: `SCM` **scm\_from\_sockaddr** `(const struct sockaddr *address, unsigned address_size)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005fsockaddr)

C言語のアドレス構造体からSchemeソケットアドレスオブジェクトを返します。address_sizeはアドレスのバイト単位のサイズです。

C 関数: `struct sockaddr *` **scm\_to\_sockaddr** `(SCM アドレス、size_t *address_size)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005fsockaddr)

Schemeレベルのソケットアドレスオブジェクトから、新しく`malloc`された`struct sockaddr`を返します。

`struct sockaddr` の戻り値のサイズ（バイト単位）は `*outsize` に格納されます。アプリケーションは、不要になった戻り値の構造体を解放するために `free` を呼び出す必要があります。

* * *

次へ: [ネットワークソケットの例](https://doc.guix.gnu.org/guile/latest/en/guile.html#Internet-Socket-Examples)、前: [ネットワークソケットアドレス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Network-Socket-Address)、上: [ネットワーク](https://doc.guix.gnu.org/guile/latest/en/guile.html#Networking) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.2.11.4 ネットワークソケットと通信 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Network-Sockets-and-Communication-1)

ソケットポートは、`socket` および `socketpair` を使用して作成できます。ポートは、同じポートへの読み書きの信頼性を高めるため、初期状態ではバッファリングされていません。`setvbuf` を使用してポートにバッファを追加できます（[バッファリング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Buffering) を参照）。

ほとんどのシステムでは、同時に開くことができるファイルとソケットの数に制限があるため、不要になったソケットポートは明示的に閉じることを強くお勧めします（[Ports](https://doc.guix.gnu.org/guile/latest/en/guile.html#Ports)を参照）。

基となるC言語の関数の中には、ネットワークバイトオーダーの値を受け取るものもありますが、Guileの慣例として、Schemeレベルではすべてが通常のホストバイトオーダーであり、必要に応じて自動的に変換が行われます。

Scheme Procedure: **socket** ファミリースタイルのプロトコル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-socket-1)

C 関数: **scm\_socket** (family, style, proto) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsocket)

family、style、protoで指定されたタイプの新しいソケットポートを返します。3つのパラメータはすべて整数です。familyの可能な値は、システムでサポートされている場合、次のとおりです。

変数: **PF\_UNIX** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-PF_005fUNIX)

変数: **PF\_INET** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-PF_005fINET)

変数: **PF\_INET6** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-PF_005fINET6)

スタイルに指定できる値は以下のとおりです（システムがサポートしている場合）。

変数: **SOCK\_STREAM** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SOCK_005fSTREAM)

変数: **SOCK\_DGRAM** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SOCK_005fDGRAM)

変数: **SOCK\_RAW** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SOCK_005fRAW)

変数: **SOCK\_RDM** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SOCK_005fRDM)

変数: **SOCK\_SEQPACKET** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SOCK_005fSEQPACKET)

proto は、`getprotobyname` を使用してプロトコル名から取得できます（[ネットワークデータベース](https://doc.guix.gnu.org/guile/latest/en/guile.html#Network-Databases)を参照）。値がゼロの場合はデフォルトのプロトコルを意味し、通常はこれで問題ありません。

ソケットは、通常は以下の `connect` または `accept` を使用してどこかに接続されるまで、通信に使用することはできません。

Scheme Procedure: **socketpair** ファミリー スタイル proto [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-socketpair)

C 関数: **scm\_socketpair** (family, style, proto) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsocketpair)

`car`と`cdr`が互いに接続された2つの無名ソケットポートであるペアを返します。接続は全二重通信なので、2つのポート間でデータは双方向に転送できます。

family、style、proto は上記の `socket` と同様です。ただし、多くのシステムは `PF_UNIX` ファミリーのソケットペアのみをサポートしています。proto にはゼロのみが有効な値となる可能性が高いです。

スキーム手順: **getsockopt** ソケットレベル optname [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-getsockopt)

スキーム手順: **setsockopt** ソケットレベル optname 値 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-setsockopt)

C 関数: **scm\_getsockopt** (sock, level, optname) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fgetsockopt)

C 関数: **scm\_setsockopt** (sock, level, optname, value) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsetsockopt)

ソケットポート sock のオプションを取得または設定します。`getsockopt` は現在の値を返します。`setsockopt` は値を設定しますが、戻り値は未指定です。

level はプロトコル レイヤーを指定する整数で、ソケット レベル オプションの場合は `SOL_SOCKET`、または `IPPROTO` 定数または `getprotoent` からのプロトコル番号です ([ネットワーク データベース](https://doc.guix.gnu.org/guile/latest/en/guile.html#Network-Databases) を参照)。

変数: **SOL\_SOCKET** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SOL_005fSOCKET)

変数: **IPPROTO\_IP** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-IPPROTO_005fIP)

変数: **IPPROTO\_IPV6** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-IPPROTO_005fIPV6)

変数: **IPPROTO\_TCP** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-IPPROTO_005fTCP)

変数: **IPPROTO\_UDP** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-IPPROTO_005fUDP)

optnameは、プロトコル層内のオプションを指定する整数です。

`SOL_SOCKET` レベルでは、以下のオプション名が定義されています（システムによって提供される場合）。これらの意味については、GNU C ライブラリ リファレンス マニュアルの [ソケット レベル オプション](https://doc.guix.gnu.org/libc/latest/en/libc.html#Socket_002dLevel-Options) または `man 7 socket` を参照してください。

変数: **SO\_DEBUG** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SO_005fDEBUG)

変数: **SO\_REUSEADDR** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SO_005fREUSEADDR)

変数: **SO\_STYLE** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SO_005fSTYLE)

変数: **SO_TYPE* [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SO_005fTYPE)

変数: **SO\_ERROR** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SO_005fERROR)

変数: **SO\_DONTROUTE** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SO_005fDONTROUTE)

変数: **SO\_BROADCAST** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SO_005fBROADCAST)

変数: **SO\_SNDBUF** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SO_005fSNDBUF)

変数: **SO\_RCVBUF** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SO_005fRCVBUF)

変数: **SO\_KEEPALIVE** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SO_005fKEEPALIVE)

変数: **SO\_OOBINLINE** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SO_005fOOBINLINE)

変数: **SO\_NO\_CHECK** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SO_005fNO_005fCHECK)

変数: **SO\_PRIORITY** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SO_005fPRIORITY)

変数: **SO\_REUSEPORT** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SO_005fREUSEPORT)

変数: **SO\_RCVTIMEO** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SO_005fRCVTIMEO)

変数: **SO\_SNDTIMEO** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SO_005fSNDTIMEO)

取得または返される値は整数です。

変数: **SO\_LINGER** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SO_005fLINGER)

取得または返される値は、整数のペア `(ENABLE . TIMEOUT)` です。タイムアウトをサポートしていない古いシステム (つまり、`struct linger` がないシステム) では、ENABLE のみが有効ですが、Guile の値は常にペアになります。

IPレベル（`IPPROTO_IP`）については、以下のオプション名が定義されています（システムによって提供される場合）。これらの意味については、`man ip`を参照してください。

変数: **IP\_MULTICAST\_IF** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-IP_005fMULTICAST_005fIF)

これは、マルチキャストトラフィックで使用される送信元インターフェースを設定します。

変数: **IP\_MULTICAST\_TTL** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-IP_005fMULTICAST_005fTTL)

これはマルチキャストトラフィックのデフォルトのTTLを設定します。デフォルト値は1ですが、ローカルネットワークを超えてトラフィックを通過させるには、この値を増やす必要があります。

変数: **IP\_ADD\_MEMBERSHIP** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-IP_005fADD_005fMEMBERSHIP)

変数: **IP\_DROP\_MEMBERSHIP** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-IP_005fDROP_005fMEMBERSHIP)

これらは `setsockopt` でのみ使用でき、`getsockopt` では使用できません。 値は、整数 IPv4 アドレスのペア `(MULTIADDR . INTERFACEADDR)` です ([ネットワーク アドレス変換](https://doc.guix.gnu.org/guile/latest/en/guile.html#Network-Address-Conversion) を参照)。 MULTIADDR は、インターフェース INTERFACEADDR に追加または削除するマルチキャスト アドレスです。 INTERFACEADDR は、システムにインターフェースを選択させる `INADDR_ANY` にすることができます。 INTERFACEADDR は、それをサポートするシステムでは、インターフェース インデックス番号にすることもできます。

最後に、IPv6レベル（`IPPROTO_IPV6`）については、以下のオプション名が定義されています。詳細は`man 7 ipv6`を参照してください。

変数: **IPV6\_V6ONLY** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-IPV6_005fV6ONLY)

`AF_INET6`ソケットがIPv6パケットの送信のみに制限されているか、IPv4マップドIPv6アドレスのパケットも送信できるかを決定します。

`IPPROTO_TCP` レベルでは、以下のオプション名が定義されています（システムによって提供される場合）。それらの意味については、`man 7 tcp` を参照してください。

変数: **TCP\_NODELAY** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-TCP_005fNODELAY)

変数: **TCP\_CORK** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-TCP_005fCORK)

取得または返される値は整数です。

Scheme Procedure: **shutdown** sock how [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-shutdown)

C 関数: **scm\_shutdown** (sock、方法) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fshutdown)

ソケットは`close-port`コマンドを使用するだけで簡単に閉じることができます。`shutdown`コマンドでは、パラメータhowに応じて、接続上の受信または送信を個別にシャットダウンできます。

0

このソケットへのデータ受信を停止します。今後データが到着した場合は、拒否します。

1

このソケットからのデータ送信を停止します。送信待ちのデータはすべて破棄します。既に送信されたデータの確認応答を待つのを停止します。確認応答が失われた場合でも再送信しません。

2

受信と送信の両方を停止してください。

戻り値は指定されていません。

Scheme Procedure: **connect** sock sockaddr [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-connect)

スキーム手順: **connect** sock AF\_INET ipv4addr port [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-connect-1)

スキーム手順: **connect** sock AF\_INET6 ipv6addr port \[flowinfo \[scopeid\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-connect-2)

スキーム手順: **connect** sock AF\_UNIX パス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-connect-3)

C 関数: **scm\_connect** (sock, fam, address, args) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fconnect)

ソケットポート sock で指定されたアドレスへの接続を開始します。接続先は、ソケットアドレスオブジェクト、または `make-socket-address` でそのようなオブジェクトを作成する際に使用する引数と同じです ([ネットワークソケットアドレス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Network-Socket-Address) を参照)。ソケットが非ブロッキングとして構成されていて、すぐに接続を確立できなかった場合を除き、true を返します。

(ソケットAF_INET INADDR_LOOPBACK 23に接続)
(ソケットを接続 (make-socket-address AF\_INET INADDR\_LOOPBACK 23))

Scheme手順: **bind** sock sockaddr [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bind)

スキーム手順: **bind** sock AF\_INET ipv4addr port [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bind-1)

スキーム手順: **bind** sock AF\_INET6 ipv6addr port \[flowinfo \[scopeid\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bind-2)

Scheme Procedure: **bind** sock AF\_UNIX path [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bind-3)

C 関数: **scm\_bind** (sock, fam, address, args) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbind)

ソケットポート sock を指定されたアドレスにバインドします。アドレスは、ソケットアドレスオブジェクト、または `make-socket-address` がそのようなオブジェクトを作成する際に使用する引数と同じです ([ネットワークソケットアドレス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Network-Socket-Address) を参照)。戻り値は未定義です。

一般的に、ソケットはサーバーを作成する際、つまり特定のポートでリッスンする場合にのみ、特定のアドレスに明示的にバインドされます。送信接続の場合、システムが既にバインドしていない限り、ローカルアドレスを自動的に割り当てます。

(bind sock AF\_INET INADDR\_ANY 12345)
(ソケットをバインドします (make-socket-address AF\_INET INADDR\_ANY 12345))

Scheme Procedure: **listen** sock backlog [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-listen)

C 関数: **scm\_listen** (sock、backlog) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005flisten)

ソケットが接続要求を受け入れるように設定します。backlogは、保留中の接続のキューの最大長を指定する整数です。キューがいっぱいになると、サーバーが`accept`を呼び出してキューから接続を受け入れるまで、新しいクライアントは接続に失敗します。

戻り値は指定されていません。

Scheme Procedure: **accept** sock \[flags\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-accept)

C 関数: **scm\_accept** (sock) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005faccept)

上記で `listen` によってリスニングが有効になっているソケットポート sock からの接続を受け入れます。

キューに着信接続がない場合、ソケットが非ブロッキング動作に設定されているかどうかに応じて、2つの動作が考えられます。

* 接続待ちがなく、ソケットが `O_NONBLOCK` ポートオプションで非ブロッキングモードに設定されている場合 ([`fcntl`](https://doc.guix.gnu.org/guile/latest/en/guile.html#Ports-and-File-Descriptors) を参照)、直接 `#f` を返します。
* それ以外の場合は、接続が可能になるまでお待ちください。

戻り値はペアです。`car` は接続され通信準備が整った新しいソケットポートです。`cdr` はソケットアドレスオブジェクトです ([ネットワークソケットアドレス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Network-Socket-Address) を参照)。これはリモート接続の元となるアドレスです (以下の `getpeername` と同様)。

フラグが指定されている場合、`SOCK_CLOEXEC` または `SOCK_NONBLOCK` が含まれることがあり、これらは `O_CLOEXEC` および `O_NONBLOCK` と同様に、新しく受け入れられたソケットに適用されます。

すべての通信は、返された新しいソケットを使用して行われます。指定されたソケットはバインドされたまま待機状態となり、必要に応じて再度`accept`を呼び出すことで、別の着信接続を取得できます。

Scheme Procedure: **getsockname** sock [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-getsockname)

C 関数: **scm\_getsockname** (sock) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fgetsockname)

ソケットがローカルにバインドされている場所を示すソケットアドレスオブジェクトを返します。ソケットは、上記の `bind` からローカルアドレスを取得している場合もあれば、通常はバインドされていないソケットに対して `connect` が実行された場合、システムによってアドレスが割り当てられている場合もあります。

多くのシステムでは、`AF_UNIX`名前空間内のソケットのアドレスを読み取ることができないことに注意してください。

Scheme Procedure: **getpeername** sock [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-getpeername)

C 関数: **scm\_getpeername** (sock) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fgetpeername)

ソケットが接続されている場所、つまりリモートエンドポイントを示すソケットアドレスオブジェクトを返します。

多くのシステムでは、`AF_UNIX`名前空間内のソケットのアドレスを読み取ることができないことに注意してください。

Scheme Procedure: **recv!** sock buf \[flags\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-recv_0021)

C 関数: **scm\_recv** (sock、buf、flags) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005frecv)

ソケットポートからデータを受信します。sockは、データを受信するアドレスに既にバインドされている必要があります。bufは、データが書き込まれるバイトベクトルです。bufのサイズは、受信できるデータ量を制限します。パケットプロトコルの場合、この制限を超えるパケットが検出されると、一部のデータが不可逆的に失われます。

オプションの flags 引数は、`MSG_OOB`、`MSG_PEEK`、`MSG_DONTROUTE` などの値、またはそれらのビットごとの OR です。

返される値は、ソケットから読み取られたバイト数です。

データはソケットファイルディスクリプタから直接読み込まれることに注意してください。読み取られなかったバッファリングされたポートデータは無視されます。

Scheme Procedure: **send** sock message \[flags\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-send)

C 関数: **scm\_send** (sock、message、flags) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsend)

ソケットポート sock でバイトベクターメッセージを送信します。sock は既に宛先アドレスにバインドされている必要があります。返される値は送信されたバイト数です。ソケットが非ブロッキングに設定されている場合、この値はメッセージの長さよりも小さくなる可能性があります。オプションの flags 引数は、`MSG_OOB`、`MSG_PEEK`、 `MSG_DONTROUTE` などの値、またはビットごとの OR です。

データはソケットファイルディスクリプタに直接書き込まれるため、フラッシュされていないバッファリングされたポートデータは無視されることに注意してください。

Scheme Procedure: **recvfrom!** sock buf \[flags \[start \[end\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-recvfrom_0021)

C 関数: **scm\_recvfrom** (sock, buf, flags, start, end) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005frecvfrom)

ソケットポート sock からデータを受信し、送信元アドレスとデータを返します。この関数は通常データグラムソケット用ですが、ストリーム指向ソケットでも使用できます。

受信したデータはバイトベクターbufに格納されます。格納には、バイトベクター全体、またはオプションの開始位置と終了位置の間の領域が使用されます。bufのサイズによって受信できるデータ量が制限されます。データグラムプロトコルでは、これより大きいパケットを受信した場合、超過分のバイトは不可逆的に失われます。

戻り値はペアです。`car` は読み取られたバイト数です。`cdr` はデータの送信元を示すソケットアドレスオブジェクトです ([ネットワークソケットアドレス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Network-Socket-Address) を参照)。送信元が不明な場合は `#f` となります。

オプションの flags 引数は、`MSG_OOB`、`MSG_PEEK`、`MSG_DONTROUTE` などのビットごとの OR (`logior`) です。

データはソケットファイルディスクリプタから直接読み取られ、バッファリングされたポートデータは無視されます。

GNU/Linux システムでは、`recvfrom!` はマルチスレッドに対応していないため、`recvfrom!` の呼び出し中はすべてのスレッドが停止します。これを回避するには、アプリケーションで `select`、`O_NONBLOCK`、または `MSG_DONTWAIT` を使用する必要がある場合があります。

Scheme Procedure: **sendto** sock message sockaddr \[flags\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sendto)

スキーム手順: **sendto** sock message AF\_INET ipv4addr port \[flags\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sendto-1)

スキーム手順: **sendto** sock message AF\_INET6 ipv6addr port \[flowinfo \[scopeid \[flags\]\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sendto-2)

Scheme Procedure: **sendto** sock message AF\_UNIX path \[flags\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sendto-3)

C 関数: **scm\_sendto** (sock、message、fam、address、args\_and\_flags) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsendto)

バイトベクターメッセージをデータグラムソケットポートsockとして送信します。宛先は、ソケットアドレスオブジェクトとして指定するか、`make-socket-address`がそのようなオブジェクトを作成する際に受け取る引数と同じ引数として指定します（[ネットワークソケットアドレス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Network-Socket-Address)を参照）。

宛先アドレスの後には、オプションの flags 引数が続く場合があります。これは、`MSG_OOB`、`MSG_PEEK`、`MSG_DONTROUTE` などの `logior` ([ビット演算](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bitwise-Operations) を参照) です。

返される値は送信されたバイト数です。ソケットがノンブロッキングに設定されている場合、この値はメッセージの長さよりも少なくなる可能性があります。データはソケットファイルディスクリプタに直接書き込まれるため、フラッシュされていないバッファリングされたポートデータは無視されることに注意してください。

* * *

前へ: [ネットワークソケットと通信](https://doc.guix.gnu.org/guile/latest/en/guile.html#Network-Sockets-and-Communication)、上へ: [ネットワーク](https://doc.guix.gnu.org/guile/latest/en/guile.html#Networking) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.2.11.5 ネットワークソケットの例 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Network-Socket-Examples)

以下に、ネットワークソケットの使用方法の例を示します。

#### インターネットソケットクライアントの例 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Internet-Socket-Client-Example)

以下の例は、インターネットソケットクライアントの動作を示しています。ローカルマシン上で実行されているHTTPデーモンに接続し、ルートインデックスURLの内容を返します。

(let ((s (socket PF\_INET SOCK\_STREAM 0)))
(接続 s AF\_INET (inet-pton AF\_INET "127.0.0.1") 80)
(「GET / HTTP/1.0\\r\\n\\r\\n」を表示します)

(do ((line (read-line s) (read-line s)))
((eof-object? line))
（表示行）
(改行)))

#### インターネットソケットサーバーの例 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Internet-Socket-Server-Example)

以下の例は、ポート2904で着信接続を待ち受け、クライアントに挨拶メッセージを返すシンプルなインターネットサーバーを示しています。

(let ((s (socket PF\_INET SOCK\_STREAM 0)))
(setsockopt s SOL\_SOCKET SO\_REUSEADDR 1)
;; 具体的な住所？
;; (バインド s AF\_INET (inet-pton AF\_INET "127.0.0.1") 2904)
(bind s AF\_INET INADDR\_ANY 2904)
（5番目の音声を聞いてください）

(simple-format #t "pid: ~S のクライアントをリッスンしています" (getpid))
（改行）

(#t の間)
(let\* ((client-connection (accept s))
(クライアント詳細 (cdrクライアント接続))
(クライアント (自動車クライアント接続)))
(simple-format #t "新しいクライアント接続を取得しました: ~S"
顧客情報）
（改行）
(simple-format #t "クライアントアドレス: ~S"
(gethostbyaddr
(sockaddr:addr client-details)))
（改行）
;; クライアントポートに挨拶を返す
(「こんにちはクライアント\r\n」クライアントを表示)
（親しい顧客））））

* * *

次へ: [ロケール](https://doc.guix.gnu.org/guile/latest/en/guile.html#Locales)、前: [ネットワーク](https://doc.guix.gnu.org/guile/latest/en/guile.html#Networking)、上: [POSIX システムコールとネットワーク](https://doc.guix.gnu.org/guile/latest/en/guile.html#POSIX) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
