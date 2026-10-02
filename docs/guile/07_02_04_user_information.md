#### 7.2.4 ユーザー情報 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#User-Information-1)

このセクションの機能は、ユーザーおよびグループデータベースへのインターフェースを提供します。これらの機能は再入可能ではないため、慎重に使用してください。

以下の関数は、ユーザー情報を表すオブジェクトを受け取り、選択されたコンポーネントを返します。

スキーム手順: **passwd:name** pw [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-passwd_003aname)

ユーザーIDの名前。

スキーム手順: **passwd:passwd** pw [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-passwd_003apasswd)

暗号化されたパスワード。

スキーム手順: **passwd:uid** pw [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-passwd_003auid)

ユーザーID番号。

スキーム手順: **passwd:gid** pw [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-passwd_003agid)

グループID番号。

スキーム手順: **passwd:gecos** pw [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-passwd_003agecos)

フルネーム。

スキーム手順: **passwd:dir** pw [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-passwd_003adir)

ホームディレクトリ。

スキーム手順: **passwd:shell** pw [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-passwd_003ashell)

ログインシェル。

  

Scheme Procedure: **getpwuid** uid [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-getpwuid)

ユーザーデータベースで整数型のユーザーIDを検索します。

Scheme Procedure: **getpwnam** name [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-getpwnam)

ユーザーデータベースでユーザー名文字列を検索します。

スキーム手順: **setpwent** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-setpwent)

`getpwent` がユーザーデータベースから読み込むために使用するストリームを初期化します。次回 `getpwent` を使用すると、最初のエントリが返されます。戻り値は未指定です。

スキーム手順: **getpwent** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-getpwent)

ユーザーデータベースストリームの次のエントリを読み込みます。戻り値は上記のようなpasswdユーザーオブジェクト、またはエントリがなくなった場合は`#f`です。

スキーム手順: **endpwent** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-endpwent)

`getpwent`で使用されるストリームを閉じます。戻り値は未指定です。

スキームプロシージャ: **setpw** \[arg\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-setpw)

C 関数: **scm\_setpwent** (arg) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsetpwent)

引数が真の場合、パスワードデータストリームを初期化またはリセットします。それ以外の場合は、ストリームを閉じます。`setpwent` および `endpwent` プロシージャは、この上に実装されています。

スキーム手順: **getpw** \[user\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-getpw)

C 関数: **scm\_getpwuid** (user) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fgetpwuid)

ユーザーデータベースのエントリを検索します。user は整数、文字列、または省略可能で、それぞれ getpwuid、getpwnam、getpwent の動作になります。

以下の関数は、グループ情報を表すオブジェクトを受け取り、選択されたコンポーネントを返します。

スキーム手順: **group:name** gr [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-group_003aname)

グループ名。

スキーム手順: **group:passwd** gr [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-group_003apasswd)

暗号化されたグループパスワード。

スキーム手順: **group:gid** gr [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-group_003agid)

グループID番号。

スキーム手順: **group:mem** gr [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-group_003amem)

このグループを補助グループとして持つユーザーIDのリスト。

  

スキームプロシージャ: **getgrgid** gid [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-getgrgid)

グループデータベースで整数型のグループIDを検索します。

スキームプロシージャ: **getgrnam** 名前 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-getgrnam)

グループデータベースでグループ名を検索してください。

スキーム手順: **setgrent** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-setgrent)

`getgrent` がグループデータベースから読み込むために使用するストリームを初期化します。次回 `getgrent` を使用すると、最初のエントリが返されます。戻り値は未指定です。

スキーム手順: **getgrent** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-getgrent)

`setgrent`で設定されたストリームを使用して、グループデータベース内の次のエントリを返します。

スキーム手順: **endgrent** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-endgrent)

`getgrent`で使用されるストリームを閉じます。戻り値は未指定です。

Scheme手順: **setgr** \[arg\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-setgr)

C 関数: **scm\_setgrent** (arg) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsetgrent)

引数が真の場合、グループデータストリームを初期化またはリセットします。それ以外の場合は、ストリームを閉じます。`setgrent` および `endgrent` プロシージャは、この上に実装されています。

スキーム手順: **getgr** \[group\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-getgr)

C 関数: **scm\_getgrgid** (グループ) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fgetgrgid)

グループデータベースのエントリを検索します。group には整数、文字列、または省略することができ、それぞれ getgrgid、getgrnam、getgrent の動作になります。

ユーザーデータベースへのアクセス手順に加えて、以下のショートカット手順も利用可能です。

スキーム手順: **getlogin** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-getlogin)

C 関数: **scm\_getlogin** () [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fgetlogin)

プロセスの制御端末にログインしているユーザー名を含む文字列を返します。この情報が取得できない場合は `#f` を返します。

* * *

次へ: [ランタイム環境](https://doc.guix.gnu.org/guile/latest/en/guile.html#Runtime-Environment)、前: [ユーザー情報](https://doc.guix.gnu.org/guile/latest/en/guile.html#User-Information)、上: [POSIX システムコールとネットワーク](https://doc.guix.gnu.org/guile/latest/en/guile.html#POSIX) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
