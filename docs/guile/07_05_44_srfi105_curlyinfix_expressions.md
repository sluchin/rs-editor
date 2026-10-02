#### 7.5.44 SRFI-105 中括弧式。 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d105-Curly_002dinfix-expressions_002e)

Guile の組み込みリーダーは、SRFI-105 の中括弧式をサポートしています。[SRFI-105 の仕様](http://srfi.schemers.org/srfi-105/srfi-105.html) を参照してください。いくつかの例を以下に示します。

{n <= 5} ⇒ (<= n 5)
{a + b + c} ⇒ (+ abc)
{a \* {b + c}} ⇒ (\* a (+ bc))
{(- a) / b} ⇒ (/ (- a) b)
{-(a) / b} ⇒ (/ (- a) b) も同様
{(fab) + (gh)} ⇒ (+ (fab) (gh))
{f(ab) + g(h)} ⇒ (+ (fab) (gh)) も同様
{f\[ab\] + g(h)} ⇒ (+ ($bracket-apply$ fab) (gh))
'{a + f(b) + x} ⇒ '(+ a (fb) x)
{length(x) >= 6} ⇒ (>= (length x) 6)
{n-1 + n-2} ⇒ (+ n-1 n-2)
{n \* factorial{n - 1}} ⇒ (\* n (factorial (- n 1)))
{{a > 0} かつ {b >= 1}} ⇒ (かつ (> a 0) (>= b 1))
{f{n - 1}(x)} ⇒ ((f (- n 1)) x)
{a . z} ⇒ ($nfx$ a . z)
{a + b - c} ⇒ ($nfx$ a + b - c)

ファイル内で中括弧式を有効にするには、中括弧式が最初に使用される前に、リーダーディレクティブ `#!curly-infix` を配置します。Guile のリーダーで中括弧式をグローバルに有効にするには、読み取りオプション `curly-infix` を設定します。

Guile は、SRFI-105 に対する以下の非標準拡張機能も実装しています。`curly-infix` が有効で、角括弧に他の意味が割り当てられていない場合 (つまり、`square-brackets` 読み取りオプションが無効になっている場合)、角括弧内のリストは通常のリストとして読み込まれますが、先頭に特殊記号 `$bracket-list$` が追加されます。ファイル内でこの読み取りオプションの組み合わせを有効にするには、リーダー ディレクティブ `#!curly-infix-and-bracket-lists` を使用します。例:

\[ab\] ⇒ ($bracket-list$ ab)
\[a . b\] ⇒ ($bracket-list$ a . b)

リーダーオプションの詳細については、[Reading Scheme Code](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scheme-Read) を参照してください。

* * *

次へ: [SRFI-119 Wisp: よりシンプルなインデント対応スキーム。](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d119)、前: [SRFI-105 カーリー中置式。](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d105)、上: [SRFI サポート モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI-Support) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
