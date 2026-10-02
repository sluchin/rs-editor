# 標準機能識別子

実装は `cond-expand` および `features`による使用のために， 下に列挙されている任意のあるいはすべての機能識別子を提供してもよいが， もし対応する機能を提供していなければ，その機能識別子は提供してはならない。

**r7rs**

すべての R<sup>7</sup>RS Scheme 実装はこの機能をもつ。

**exact-closed**

`/` を除くすべての代数演算は，与えられた正確入力に対して正確値をもたらす。

**exact-complex**

正確複素数が提供されている。

**ieee-float**

不正確数が IEEE 754 バイナリ浮動小数点数である。

**full-unicode**

Unicode version 6.0 におけるすべての Unicode 文字表現が， Scheme 文字としてサポートされている。

**ratios**

除数が非ゼロのとき，`/` に正確な引数を指定して，正確な結果をもたらす。

**posix**

この実装が，POSIX システム上で実行されている。

**windows**

この実装が，Windows上で実行されている。

**unix, darwin, gnu-linux, bsd, freebsd, solaris, ...**

オペレーティングシステムフラグ(おそらく一つ以上)。

**i386, x86-64, ppc, sparc, jvm, clr, llvm, ...**

CPU アーキテクチャフラグ。

**ilp32, lp64, ilp64, ...**

C メモリモデルフラグ。

**big-endian, little-endian**

バイトオーダフラグ。

***⟨名前⟩***

この実装の名前。

***⟨名前-バージョン⟩***

この実装の名前およびバージョン。
