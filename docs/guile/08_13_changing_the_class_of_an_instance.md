### 8.13 インスタンスのクラスの変更 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Changing-the-Class-of-an-Instance-1)

再定義可能なクラスが再定義されると、再定義されたクラスの既存のインスタンスは、そのインスタンスのスロットが次に参照または設定される前に、新しいクラス定義に合わせて変更されます。GOOPS は、汎用関数 `change-class` を呼び出すことで、各インスタンスを変更します。

より一般的には、既存のインスタンスのクラスは、汎用関数 `change-class` をインスタンスと新しいクラスの 2 つの引数で呼び出すことで、いつでも変更できます。

`change-class` のデフォルトメソッドは、インスタンスの既存クラスと新規クラスのスロット定義を参照して、クラス変更の実装方法を決定します。新規クラスに既存クラスのスロットと同じ名前のスロットがある場合、それらのスロットの値は保持されます。既存クラスにのみ存在するスロットは破棄されます。新規クラスにのみ存在するスロットは、対応するスロット定義の init 関数を使用して初期化されます ([slot-init- function](https://doc.guix.gnu.org/guile/latest/en/guile.html#Classes) を参照)。

汎用: **change-class** インスタンス new-class [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-change_002dclass)

メソッド: **change-class** (obj <object>) (new <redefinable-class>) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-change_002dclass-1)

インスタンス obj を変更して、クラス new のインスタンスにします。obj 自体は、既に再定義可能なクラスのインスタンスである必要があります。

objの各スロットの値は、newに同じ名前のスロットが存在する場合にのみ保持され、それ以外のスロット値は破棄されます。

新規オブジェクト内のスロットのうち、オブジェクト obj の既存のスロットに対応しないものは、新規オブジェクトのスロット定義の初期化関数に従って初期化されます。

デフォルトの `change-class` メソッドは、戻り値を返す前に、最後に別の汎用関数 `update-instance-for-different-class` を呼び出します。適用された `update-instance-for-different-class` メソッドは、クラスの変更を完了または変更するために必要な、new-instance に対する追加の調整を行うことができます。適用されたメソッドの戻り値は無視されます。

汎用: **update-instance-for-different-class** old-instance new-instance [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-update_002dinstance_002dfor_002ddifferent_002dclass)

クラスが変更されたばかりのインスタンスに最終的な調整を加えるためにカスタマイズ可能な汎用関数。デフォルトの `update-instance-for-different-class` メソッドは何も実行しません。

クラスの動作をカスタマイズして変更するには、変更対象のインスタンスのクラス、または新しいクラスのメタクラスによって特殊化された `change-class` メソッドを定義することで実現できます。

* * *

次へ: [GNU フリー文書ライセンス](https://doc.guix.gnu.org/guile/latest/en/guile.html#GNU-Free-Documentation-License)、前: [GOOPS](https://doc.guix.gnu.org/guile/latest/en/guile.html#GOOPS)、上: [Guile リファレンス マニュアル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Top) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
