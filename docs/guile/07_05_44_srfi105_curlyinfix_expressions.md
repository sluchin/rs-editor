#### 7.5.44 SRFI-105 中括弧式。

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

リーダーオプションの詳細については、[Reading Scheme Code](06_16_reading_and_evaluating_scheme_code.md#6162-リーディングスキームコード) を参照してください。

* * *

次へ: [SRFI-119 Wisp: よりシンプルなインデント対応スキーム。](07_05_46_srfi119_wisp_simpler_indentationsensitive_scheme.md#7546-srfi-119-wisp-よりシンプルなインデント対応スキーム)、前: [SRFI-105 カーリー中置式。](#7544-srfi-105-中括弧式)、上: [SRFI サポート モジュール](07_05_00_srfi_support_modules.md#75-srfi-サポート-モジュール) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
