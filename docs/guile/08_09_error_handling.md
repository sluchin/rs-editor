### 8.9 エラー処理 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Error-Handling-2)

プロシージャ`goops-error`は、以下の汎用関数のデフォルトメソッドによって適切なエラーを発生させるために呼び出されます。

* `slot-missing` ([slot-missing](https://doc.guix.gnu.org/guile/latest/en/guile.html#Accessing-Slots)を参照)
* `slot-unbound` ([slot-unbound](https://doc.guix.gnu.org/guile/latest/en/guile.html#Accessing-Slots)を参照)
* `no-method` ([no-method](https://doc.guix.gnu.org/guile/latest/en/guile.html#Handling-Invocation-Errors)を参照)
* `no-applicable-method` ([no-applicable-method](https://doc.guix.gnu.org/guile/latest/en/guile.html#Handling-Invocation-Errors)を参照)
* `no-next-method` ([no-next-method](https://doc.guix.gnu.org/guile/latest/en/guile.html#Handling-Invocation-Errors)を参照)

これらの関数を特定のクラスやメタクラスに合わせてカスタマイズする場合でも、検出したエラー状態を通知するために `goops-error` を使用することをお勧めします。

手順: **goops-error** format-string arg … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-goops_002derror)

キー `goops-error` と format-string および arg ... から構築されたエラーメッセージを使用してエラーを発生させます。エラーメッセージのフォーマットは `scm-error` と同様です。

* * *

次へ: [メタオブジェクトプロトコル](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Metaobject-Protocol)、前: [エラー処理](https://doc.guix.gnu.org/guile/latest/en/guile.html#GOOPS-Error-Handling)、上: [GOOPS](https://doc.guix.gnu.org/guile/latest/en/guile.html#GOOPS) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
