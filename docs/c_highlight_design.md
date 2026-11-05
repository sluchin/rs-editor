# C / C++ のシンタックスハイライト 設計方針

状態: 段階 1 (C の字句)、段階 2 (C++)、段階 3 (宣言の名前) を実装済み。段階 4 は未実装。

## 1. 目的と範囲

- C と C++ のソースに、色を付けます。Tauri の画面と ncurses の TUI の**両方**で、同じ結果にします。
- そのため、字句の解析は、核 (`core/`、Rust) に置きます。TypeScript (CodeMirror の `StreamLanguage`) では、TUI で使えないためです。
- 手本は Emacs 30 の `c-mode` / `c++-mode` の font-lock です。Emacs の既定は `font-lock-maximum-decoration` が `t` (最も詳しい段階) です。
- 言語ごとの違いは、データ (`LanguageSpec`、3.1) にして、言語を足せる形にします。
- 範囲外: 字下げ (`c-indent`)、構文木。C と C++ 以外の言語は、3.4 (Lisp) と 3.5 (Rust) で足しました。

### 段階

| 段階 | 内容 |
|:---|:---|
| 1 | C の字句 (コメント、文字列、文字、プリプロセッサ、キーワード、型、定数)。Tauri と TUI に色を出す。Emacs と比べる |
| 2 | C++ (キーワードと型の表、raw 文字列 `R"x(...)x"`) |
| 3 | 宣言の名前 (関数の名前、変数の名前、型の名前。3.2 を参照) |
| 4 | 再解析の範囲を絞る効率化。必要なら |

## 2. 現状

- 色は、Scheme と Elisp だけで、核の `core/src/highlight/lisp.rs` が字句に分けます (設計は 3.4)。Tauri と TUI の両方に、同じ色が付きます。(以前は、CodeMirror の `StreamLanguage` で付けていたため、Tauri だけでした。)
- `highlight-regexp` は、正規表現と face をバッファに持ちます。TUI (`tui/src/ncurses_screen.rs`) は、下線 (`A_UNDERLINE`) を引くだけで、色は使っていません (`start_color` と `init_pair` を使っていません)。
- メジャーモードの判定は、核の 1 か所です。
  - `lisp/files.scm` の `auto-mode-alist` と `set-auto-mode` が、ファイルの名前と、1 行目の `-*- mode: NAME -*-` から、バッファの方言 (`Dialect`。Fundamental、Elisp、Scheme) と、`major-mode` の名前を決めます。方言は、構文表 (`core/src/syntax.rs`)、字下げ、段落の区切りに使います。
  - `core/src/mode.rs` の `Buffer::mode_info` が、モードラインの名前と、括弧の読み方 (`scheme` / `elisp` / なし)、字下げのタブとタブの幅 (ファイルの変数 `indent-tabs-mode` `tab-width`) を返します。Tauri の画面は `BufferFrame.mode` で受け取り、TUI のモードラインも同じ名前を使います。
  - 画面の `src/lib/modes.ts` は、消しました。

## 3. 方針の中心: 核が「範囲と種類」を返す

核は、色を知りません。「本文の、どこからどこまでが、どの種類か」だけを返します。色への割り当ては、各フロントが行います。

```rust
/// 字句の種類 (Emacs の font-lock の face に対応する).
pub enum TokenKind {
    Comment,          // font-lock-comment-face
    CommentDelimiter, // font-lock-comment-delimiter-face (隣の空白 1 つを含む)
    DocComment,       // font-lock-doc-face ("/** ... */" の全体)
    String,           // font-lock-string-face (文字列, 文字定数, #include の <...>, #error の本文)
    Keyword,          // font-lock-keyword-face (if, return, static, struct, typedef ...)
    Type,             // font-lock-type-face (int, void ... 組み込みの型)
    Constant,         // font-lock-constant-face (NULL, true, false)
    Preprocessor,     // font-lock-preprocessor-face (行頭の空白から "#include" まで)
    Number,           // 字句としては返すが、色は既定で付けない (3.2)
    FunctionName,     // font-lock-function-name-face (宣言と定義の関数、関数ポインタ、関数のような #define)
    VariableName,     // font-lock-variable-name-face (宣言の変数、引数、メンバー、列挙子、#define)
}

/// 本文の範囲 (文字 (char) の位置. `end` は含まない).
pub struct Span { pub start: usize, pub end: usize, pub kind: TokenKind }

/// 本文の全体を字句に分ける (始まりの順. 重ならない. 色の付かない文字は含まない).
pub fn tokenize(language: Language, text: &str) -> Vec<Span>;
```

### 3.1 解析の方式: 本文を 1 回で読む字句解析器

- 正規表現ではなく、手書きの字句解析器 (`core/src/highlight/mod.rs`) にします。ブロックコメント `/* */`、行の継続 (`\` で終わる行。`//` コメント、文字列、プリプロセッサの行が続く) が、行をまたぐためです。行ごとの正規表現では、扱えません。
- 本文の先頭から終わりまでを、1 回の走査で読みます。状態 (指示の行の中か、行の始まりか) は、走査の変数で持ちます。
- バッファ (`Buffer::token_spans`) は、結果を本文の版 (`version`) と言語で覚えます。本文を書き換えるまでは、同じ結果を使い回します。書き換えると、次の要求で、全体を読み直します (行ごとの状態を覚えて、変わった行から先だけを読み直す効率化は、段階 4)。
- 言語ごとの違い (キーワード、型、定数の表) は、データ (`LanguageSpec`、`core/src/highlight/lang.rs`) にして、字句解析器は 1 つにします。表は、Emacs の `c-keywords-regexp`、`c-primitive-type-key`、`c-constant-key` から取りました。
- `Buffer::line_token_spans(line)` は、行の中の位置に切った範囲を返します (TUI が使います)。

### 3.2 Emacs に合わせる点

Emacs 30.2 の `c-mode` で、`font-lock-ensure` の結果を確かめた点です。

- 数値 (`0x1F`、`0`) と、関数の呼び出し (`foo()`) には、色が付きません。`Number` は字句としては返しますが、既定では色を付けません。
- 大文字の名前 (`MAX_LEN`) は、定数の色になりません。`#define MAX_LEN` の名前は、変数の名前の色 (`font-lock-variable-name-face`) です。使う側の `MAX_LEN` には、色が付きません。
- 宣言の名前には、色が付きます。`int count` の `count`、`struct point { int x; }` の `x` は変数の名前の色、`int main(void)` の `main` は関数の名前の色、`struct point` の `point` と `typedef ... point_t` の `point_t` と `size_t n` の `size_t` は型の色です。これは cc-mode の宣言の推測 (`c-font-lock-declarations`) によるもので、字句だけでは決まりません。段階 3 で扱います (3.3)。
- コメントの区切り: `/*` は、続く `*` (最後の `*` が `*/` の一部なら除く) と空白まで。`*/` は、前の空白から (終わりの `*` は 1 つだけ)。`//` は、続く `/` と空白まで。本文が空 (空白だけ、`/**/`、`/*****/`) なら、全体が区切りです。
- ドキュメントのコメント: `/**` の次が `/` と `*` 以外なら、区切りも含めた全体が `font-lock-doc-face` です。
- プリプロセッサ: 行頭の空白から、指示の名前まで (行頭に空白がある `#` から `include` まで) が `font-lock-preprocessor-face` です。`#include <...>` の `<...>` と、`#error` と `#warning` の本文は、文字列の色です。`#if` と `#elif` の行の `defined` は、指示の色で、`_` で区切った部分にも付きます (`__off64_t_defined` の終わりの `defined`)。
- 閉じない文字列は、始まりの引用符が警告の色で、本文だけが文字列の色です。
- 比べ方: `emacs -Q --batch` で、`c-mode` の `font-lock-ensure` を実行して、各文字の face を範囲にして、rs-editor の `token-spans` と比べます (`tests/c-highlight/`。`npm run test:c-highlight`)。宣言の名前の色 (関数、変数、型、ラベル) も比べます。ただし、ファイルの名前に `lexical` が付くケースは、C++ の、宣言の名前を扱わない構文 (テンプレート、名前空間、クラスなど) を含むため、字句だけで決まる種類 (コメント、文字列、キーワード、プリプロセッサ) に絞ります。範囲の前後の改行は、含めません。
- C++ の生の文字列 `R"x(...)x"` (`R` `LR` `uR` `UR` `u8R` の接頭辞): 色が付くのは `(` から `)` までで、引用符と区切りの `x` は付きません。区切りの書き方が正しくなければ (空白、括弧、`\`、引用符を含む、または 16 文字より長い)、普通の文字列です。閉じなければ、`(` の次から本文の終わりまでが文字列です。
- C++ の数値の区切り (`1'000'000`) は、数値の一部で、文字定数ではありません。
- キーワードと型の表は、Emacs の `c-keywords-regexp`、`c-primitive-type-key`、`c-constant-key` (`c-mode` で読んだ値) から取りました。

### 3.3 宣言の名前 (段階 3)

- 字句解析器は、本文を読むときに、コードの字句 (語、数値と文字列、記号 1 文字。コメントとプリプロセッサの行は除く) の並びも覚えます。`core/src/highlight/decl.rs` が、この並びを文の始まりから読み、宣言の名前に色を付けます。結果の範囲は、字句の範囲と合わせて、始まりの順に並べます。名前は、キーワードでない語だけなので、字句の範囲とは重なりません。
- 文の始まりは、本文の先頭と、`;` `{` `}` `else` `do` の次、`for (` の次、`case ...:` と `default:` の次です。文の始まりから、宣言として読めるかを試し、読めなければ式として読み飛ばします (読み取った名前は捨てます)。
- 宣言は、型と、`,` で区切った宣言子です。
  - 型: 前に付く語 (`static` `const` `typedef` など)、組み込みの型、`struct` `union` `enum` (`class`) の名前と本体、`typeof(...)`、`__attribute__((...))`、型の名前。型の名前は、後ろが名前か `*` `&` のときだけ型の色にします (`foo(x)` や `x = y` は宣言ではありません)。`auto` は、型の代わりです。
  - 宣言子: `*` の後ろの名前、`[...]`、ビットフィールド `: 幅`、`= 初期値` (括弧の外の `,` か `;` まで読み飛ばす)、関数の宣言子 `名前(引数)`、関数ポインタ `(*名前)(引数)`。名前は、`typedef` なら型の色、後ろが `(` なら関数の名前の色 (関数ポインタも)、それ以外は変数の名前の色です。
  - 引数: 型と名前を読めた引数に色を付けます。名前の無い引数は、型だけです。`...` は読み飛ばします。
  - 列挙子は、変数の名前の色です。
- ラベル (`名前:`) と `goto 名前` の名前は、定数の色です。
- `#define` の名前は、変数の名前の色、名前の直後に `(` があれば (関数のような形) 関数の名前の色で、引数の名前は変数の名前の色です。`#undef` など、ほかの指示の名前には、色を付けません。
- Emacs に合わせた癖: `_Complex` と `_Imaginary` が付く宣言には、色を付けません。
- C++ では、さらに次の形を読みます (Emacs の `c++-mode` と比べて確かめた形です)。
  - 修飾した名前 `std::string`: `::` の前の名前は定数の色、最後の名前は、型、変数、関数の名前の色です。テンプレートの引数 `vector<int>` は、`::` の前の名前と、`<` が続く名前だけに色を付けます (`Foo<Bar>` の `Bar` には付けません)。
  - `namespace` の名前は定数の色、`using 名前 = 型;` の名前は型の色です。
  - `enum class E : int { ... }` と、クラスの継承 `class D : public B` の名前は、型の色です。
  - コンストラクタとデストラクタ (クラスの本体の中の、型の無い `D(int x)` と `~D()`)、クラスの外の定義 `D::D()` `D::~D()` `void D::f()`、初期化子の並び `: v(x)`、`= default` `= 0`。
  - `template <typename T, int N>` の仮引数 (`T` は型の色、`N` は変数の名前の色)。
  - 範囲 `for (auto x : v)` の `x`。
  - 演算子の関数: クラスの中の `V operator+(...)` は、演算子の記号 (`+` `==` `()` `[]` `<<`) が関数の名前の色です。クラスの外の `V::operator+` は、`::` の前の名前だけに色が付きます (Emacs と同じ)。
  - `typename T::value_type x` (`T` は定数の色、`value_type` は型の色)。
  - 配列へのポインタと参照 `int (*p)[4]` `int (&r)[4]` の名前は、変数の名前の色です (関数ポインタ `(*f)(int)` は、関数の名前の色)。
  - `if` `while` `switch` の条件の中の宣言 `if (auto it = m.find(k); ...)`。変数に初期値が付く形だけです。
- 扱わない形: ファイルの先頭の、型の無い関数の呼び出し (`MODULE_LICENSE("GPL");`) は、Emacs も色を付けないので同じです。変換演算子 `operator bool()` と、`operator new` の名前は読みません。

### 3.4 Scheme と Emacs Lisp

- `core/src/highlight/lisp.rs` が、行ごとに、字句を 1 つずつ読みます (状態は、行をまたいで覚えます。字句は、行をまたぎません)。`tokenize(Language::Scheme | Language::Elisp, text)` が呼びます。C と C++ の字句の表 (`LanguageSpec`) は使わないので、`Language::spec()` は `None` を返します。
- 色を付けるのは、Emacs の `scheme-font-lock-keywords-2` と `lisp-el-font-lock-keywords-2` と同じ所です: コメント、文字列、`(` の直後の特別な形 (`define`、`defun`、`if` など)、特別な形で定義する名前 (関数、変数、型)、`:key` などの印。`car` などの関数と、数には付けません。Emacs Lisp では、文字列の中の正規表現のグループ (`\\(`)、キーの名前 (`\\[command]`)、要らない `\`、`` `name' ``、閉じた括弧の後ろに続く引数 (見落としやすい。警告の色) にも、付けます。
- 種類は、C と同じ `TokenKind` に、`Builtin` (`:key`)、`Warning`、`RegexpGroup` を足しました。コメントの区切り (`;`) は、コメントと同じ種類です (色が同じ)。
- 言語は、構文の方言 (`Buffer::dialect`) で決まります (`emacs-lisp-mode` は Emacs Lisp、`scheme-mode` は Scheme)。`*scratch*` の `lisp-interaction-mode` は、Scheme を評価するので、Scheme です (モード名から決め、コマンドは方言も Scheme にします)。
- 比べ方: `tests/lisp-highlight/` (`npm run test:lisp-highlight`)。`emacs -Q --batch` で、`scheme-mode` と `emacs-lisp-mode` の `font-lock-ensure` を実行し、`token-spans` と比べます。範囲は行をまたがず、色を付けない文字と改行は含みません。
- 画面の `src/lib/lispMode.ts` と `src/lib/elispKeywords.ts` (CodeMirror の `StreamLanguage`) は、核に移したので、消しました。

### 3.5 Rust

- `core/src/highlight/rust.rs` が、rust-lang/rust-mode (`rust-font-lock-keywords`) と同じ並びの正規表現 (`regex` クレート) を、同じ順に当てます。文字ごとに種類を持ち、先に付いた色は上書きしません (Emacs の `override` が `nil`: 範囲に 1 文字でも色が付いていれば、全体を付けない。`keep`: 色の無い文字にだけ付ける)。`Language::spec()` は `None` です。
- コメント、文字列、文字、生の文字列は、正規表現より先に読みます (rust-mode の構文表と `rust-syntax-propertize` に当たる)。入れ子のブロックコメント、`r#"..."#`、文字とライフタイム (`'a` と `'a'`) を区別します。`///` `//!` `/** */` は `DocComment` です。
- `Foo::` (型) と `foo::` (定数) は、後ろに `<` が続かないときだけ付けます (rust-mode の `rust-path-font-lock-matcher`)。
- 種類の対応 (rust-mode の face → 種類): `rust-unsafe` → `Warning`、`font-lock-preprocessor-face` (属性とマクロ) → `Preprocessor`、書式のマクロと `?` → `Builtin`、文字列の中の `{}` → `String`。
- 比べ方: `tests/rust-highlight/` (`RUST_MODE_DIR=<rust-mode の clone> npm run test:rust-highlight`)。rust-mode は Emacs の組み込みではないので、clone を `-L` で読みます。
- 扱わない形: rust-mode の `rust-string-interpolation` の斜体 (種類は文字列と同じ)、`rust-ampersand-face` (既定の色)。

### 3.6 TypeScript

- `core/src/highlight/typescript.rs` が、emacs-typescript/typescript-mode (`typescript--font-lock-keywords-4`) の規則を、同じ順に当てます。Rust (3.5) と同じ `paint.rs` (文字ごとの種類。`nil` / `keep` / `t` の上書きの決まり) を使います。`Language::spec()` は `None` です。
- 構文を先に読みます: `//` と `/* */` (入れ子にしない)、`'` `"` `` ` `` の文字列 (行をまたぐ)、正規表現のリテラル `/.../` (`typescript-syntax-propertize` と同じ見分け方: `=([{,:;|&!` と `return` の後ろ、行の始め)、先頭の `#!`。
- 括弧を読む規則も移しました: `const` `let` `var` の名前 (`{a: b}` と `[a, b]` の分割代入を含む。探すのは行の終わりまで)、`function` とメソッドとアロー関数の引数、ジェネリクス `Foo<T>` の型、`{ class: 1 }` のキーのキーワードの色を消す規則。Emacs が、行の終わりまでしか探さない所は、同じにしています。
- 種類は、`DocTag` `DocType` `DocValue` を足しました (JSDoc の `@param` `{string}` と名前。typescript-mode の `typescript-jsdoc-tag` `-type` `-value`。色は固定: SlateGray, SteelBlue, gold4)。`/**` のコメントの中だけです。
- 比べ方: `tests/ts-highlight/` (`TS_MODE_DIR=<typescript-mode の clone> npm run test:ts-highlight`)。Emacs の結果は `tests/ts-highlight/expected/` に入れてあり (`run.sh -u` で書き直す)、`core/tests/test_highlight_typescript.rs` が、Emacs が無くても比べる。
- 扱わない形: `class` の framework の宣言 (Dojo, ExtJS, Prototype。typescript-mode の `typescript-enabled-frameworks`)。Emacs が `scan-error` で止まる入力 (閉じ括弧だけがあるアロー関数など) は、引数を探すのをやめるだけです。
- `.js` と `.jsx` は含めません。Emacs は、これらを組み込みの `js-mode` で開き、font-lock の規則が違うためです (`.ts` `.tsx` `.mts` `.cts` だけが `typescript-mode`)。

## 4. 言語とモードの結び付け

- 方言 (`Dialect`) は、Lisp の構文表、字下げ、段落、コメントに使われていて、`Fundamental` でなければ Lisp として扱う箇所が多くあります。そのため、C は方言を増やさず、`text-mode` と同じく、方言は `Fundamental` のままで、`major-mode` の名前 (`Buffer::mode_name`) を `c-mode` にします。`Buffer::highlight_language()` が、この名前から言語を決めます。
- 核の `auto-mode-alist` (`lisp/files.scm`) に、`.c` と `.h` (`c-mode`)、`.cc` `.hh` `.cpp` `.hpp` `.cxx` `.hxx` `.c++` `.h++` `.ii` と、大文字の `.C` `.H` など (`c++-mode`) を足しました。コマンド `c-mode` と `c++-mode` は `lisp/simple.scm` にあります。Emacs は `.h` を内容で C か C++ に決めますが、ここでは C にします。
- Scheme からは、`(token-spans)` が、バッファの字句を `(始まり 終わり 種類)` の並びで返します (位置は 1 から。終わりは含まない)。比較の試験が使います。
- モードラインの名前 (`C`、`C++`) は、核の `mode_info` が決めます (画面と TUI で同じ)。`language` は `null` のままで、色は核の字句で付けます。
- 既存の `highlight-regexp` は、そのまま残します。ハイライトは、「字句の種類」の層と、「`highlight-regexp`」の層の 2 つです。後者が上に重なります (Emacs の hi-lock と同じ)。

## 5. フロントへの渡し方

### Tauri (画面)

- 核は、描画を作るとき、言語があって、ウィンドウに出しているバッファの字句の全体を、`BufferFrame.tokens` (`{ from, to, kind }` の並び。位置は UTF-16) に入れます。画面は、スクロールの範囲を核に知らせないため、表示している行ではなく、全体を送ります。
- 画面 (`src/lib/faces.ts` の `tokenHighlighting`) は、種類を CSS のクラス `cm-tok-<種類>` にして、装飾にします。色は `src/styles/CodeEditor.css` で、既存の `--font-lock-*` の変数 (明るい / 暗い) から決めます。本文より先へ出る範囲は、飛ばします。
- 字句が前と同じなら (`src/App.tsx` の `stableTokens`)、同じ配列を渡して、色付けを引き直しません。
- Scheme と Elisp も、同じ仕組みです (3.4)。

### TUI (ncurses)

- 画面のマス (`tui/src/screen.rs` の `Cell`) に、字句の種類 (`token`) を持たせます。`tui/src/render.rs` が、`Buffer::line_token_spans` から、折り返しと幅の広い文字を考えて、マスごとの種類を決めます。
- 種類から色への表は、`tui/src/screen.rs` の `token_style` と `token_pair` の 1 か所です。ncurses の描画 (`tui/src/ncurses_screen.rs`) が、起動のときに `start_color` と `use_default_colors` と `init_pair` を呼び、マスごとに色の対を付けます。端末が色に対応していなければ (`has_colors` が偽)、色を付けません。`highlight-regexp` の下線は、そのままです。
- 色は、8 色の端末で、暗い背景の Emacs の font-lock の色に合わせます (コメントは黄、文字列とドキュメントと型は緑、キーワードはシアンの太字、定数はマゼンタ、プリプロセッサは青の太字)。256 色以上の端末では、Emacs の顔の色 (chocolate1、LightSalmon、Cyan1 など) に近い xterm の色を使います (太字は警告と正規表現のグループだけ)。

## 6. テストと 100% カバレッジ

- 核の字句解析器は、単体テストで、種類ごと、行をまたぐ場合ごとに確かめます (`core/tests/test_highlight_c.rs`)。宣言の名前は `core/tests/test_highlight_decl.rs` です。
- Emacs との比較は、`tests/c-highlight/cases/` の C のファイルを、Emacs と rs-editor の両方で処理して、範囲と種類を比べます。ケースは、コメントの区切りの全ての形、文字列と文字定数のエスケープ、行の継続、プリプロセッサ、キーワード、型、定数、日本語、改行のない終わり、空のファイルを含みます。
- Tauri の側は、核の `FrameBuilder` (`core/tests/test_frame.rs`) と、画面 (`tests/frontend/`) を単体テストします。TUI の側は、マスの種類と、種類から色への表を単体テストします (`tui/tests/test_render.rs`)。ncurses の描画そのものは、端末が要るので、自動のテストの対象外です。
- 100% の行カバレッジ (`npm run coverage:be`、`npm run coverage:fe`) を守ります。
- ja.po: 新しい公開コマンドと変数の docstring を、`npm run po:update` で足して、日本語に訳します。

## 7. 決定事項

1. **範囲**: 段階 1 は C だけ、段階 2 で C++ を足す。
2. **数値の色**: Emacs に合わせて、既定は色なし。
3. **モード判定**: 段階 1 は、核の `auto-mode-alist` と方言に C と C++ を足した。画面の `modes.ts` との二重の判定は、核の `mode_info` に一本化した。
4. **TUI の色**: 8 色を既定にして、256 色以上の端末では Emacs に近い色を使う。

## 8. 実装の順序

1. 済: `core/src/highlight/` に `TokenKind`、`Span`、`LanguageSpec` と、C の字句解析器。単体テスト。
2. 済: `c-mode`、`auto-mode-alist`、`(token-spans)`。
3. 済: `tests/c-highlight/` の Emacs との比較の枠組みと、C のケース。
4. 済: `frame.rs` と `faces.ts` で、Tauri に出す。
5. 済: `tui/` で、ncurses の色を出す。
6. 済: C++ (`c++-mode`、キーワードと型の表、生の文字列、数値の区切り) と、比較のケース。
7. 済: 宣言の名前 (段階 3。`core/src/highlight/decl.rs`、`tests/c-highlight/cases/14-decls.c` と `15-cpp-decls.cpp`)。
8. 未: 再解析の効率化 (段階 4)。測っただけで、実装していない。1.2 MB、3.5 万行の C のファイルの全体の解析が、デバッグビルドで 0.4 秒弱 (起動を含む)。通常のファイルでは気にならないため、遅くなってから行う。
