# C / C++ のシンタックスハイライト 設計方針

状態: 実装前。

## 1. 目的と範囲

- C と C++ のソースに、色を付けます。Tauri の画面と ncurses の TUI の**両方**で、同じ結果にします。
- そのため、字句の解析は、核 (`core/`、Rust) に置きます。TypeScript (CodeMirror の `StreamLanguage`) では、TUI で使えないためです。
- 手本は Emacs 30 の `c-mode` / `c++-mode` の font-lock です。Emacs の既定は `font-lock-maximum-decoration` が `t` (最も詳しい段階) です。
- 言語ごとの違いは、データ (`LanguageSpec`、3.1) にして、言語を足せる形にします。
- 範囲外: 字下げ (`c-indent`)、構文木、C と C++ 以外の言語 (Rust、Python など)。

### 段階

| 段階 | 内容 |
|:---|:---|
| 1 | C の字句 (コメント、文字列、文字、プリプロセッサ、キーワード、型、定数)。Tauri と TUI に色を出す。Emacs と比べる |
| 2 | C++ (キーワードと型の表、raw 文字列 `R"x(...)x"`) |
| 3 | 宣言の名前 (関数の名前、変数の名前、型の名前。3.2 を参照) |
| 4 | 再解析の範囲を絞る効率化。必要なら |

## 2. 現状

- 色は、Scheme と Elisp だけです。`src/lib/lispMode.ts` (CodeMirror) で付けるため、Tauri だけです。
- `highlight-regexp` は、正規表現と face をバッファに持ちます。TUI (`tui/src/ncurses_screen.rs`) は、下線 (`A_UNDERLINE`) を引くだけで、色は使っていません (`start_color` と `init_pair` を使っていません)。
- メジャーモードの判定は、2 か所にあります。
  - 核: `lisp/files.scm` の `auto-mode-alist` と `set-auto-mode` が、ファイル名から、バッファの方言 (`Dialect`。Fundamental、Elisp、Scheme) を決めます。方言は、構文表 (`core/src/syntax.rs`)、字下げ、段落の区切りに使います。TUI も、これを使います。
  - 画面: `src/lib/modes.ts` の `AUTO_MODES` と `majorMode` が、モードラインの名前と、CodeMirror の言語 (`'scheme' | 'elisp' | null`) を決めます。
- どちらにも、C と C++ はありません。

## 3. 方針の中心: 核が「範囲と種類」を返す

核は、色を知りません。「行の、どこからどこまでが、どの種類か」だけを返します。色への割り当ては、各フロントが行います。

```rust
/// 字句の種類 (Emacs の font-lock の face に対応する).
pub enum TokenKind {
    Comment,          // font-lock-comment-face
    CommentDelimiter, // font-lock-comment-delimiter-face ("/* " "// " " */". 後ろと前の空白を含む)
    String,           // font-lock-string-face (文字列, 文字定数, #include の <...>)
    Keyword,          // font-lock-keyword-face (if, return, static, struct, typedef ...)
    Type,             // font-lock-type-face (int, void, bool ...)
    Constant,         // font-lock-constant-face (NULL, true, false)
    Preprocessor,     // font-lock-preprocessor-face (#include, #define ...)
    Number,           // 色は既定で付けない (3.2)
    FunctionName,     // font-lock-function-name-face (段階 3)
    VariableName,     // font-lock-variable-name-face (段階 3)
}

/// 行の中の範囲 (文字 (char) の位置. `Buffer::highlighted_spans` と同じ単位).
pub struct Span { pub start: usize, pub end: usize, pub kind: TokenKind }

pub trait Highlighter {
    /// 1 行 `text` の字句を返す. `state` は, 前の行の終わりの状態 (ブロックコメントの中, など).
    fn highlight_line(&self, text: &str, state: State) -> (Vec<Span>, State);
}
```

### 3.1 解析の方式: 行ごとの状態機械

- 正規表現ではなく、手書きの字句解析器にします。ブロックコメント `/* */`、行の継続 (`\` で終わる行。`//` コメント、文字列、プリプロセッサの行が続く)、C++ の raw 文字列が、行をまたぐためです。行ごとの正規表現では、扱えません。
- 状態は小さな値です (`Normal`、`BlockComment`、`LineCommentContinued`、`StringContinued`、`PreprocessorContinued`、`RawString(区切り)`)。行の終わりの状態を、バッファが行ごとに覚えます。
- 編集したら、その行から先を、状態が前と同じになるまで再解析します。段階 1 から 3 は、バッファの先頭から、表示している範囲の終わりまで計算します (段階 4 で効率化)。
- 解析器は、`core/src/highlight/` に置きます。言語ごとの違い (キーワード、型、定数の表、コメントの形式、raw 文字列の有無) は、データ (`LanguageSpec`) にして、字句解析器は 1 つにします。

### 3.2 Emacs に合わせる点

Emacs 30.2 の `c-mode` で、`font-lock-ensure` の結果を確かめた点です。

- 数値 (`0x1F`、`0`) と、関数の呼び出し (`foo()`) には、色が付きません。`Number` は字句としては返しますが、既定では色を付けません。
- 大文字の名前 (`MAX_LEN`) は、定数の色になりません。`#define MAX_LEN` の名前は、変数の名前の色 (`font-lock-variable-name-face`) です。使う側の `MAX_LEN` には、色が付きません。
- 宣言の名前には、色が付きます。`int count` の `count`、`struct point { int x; }` の `x` は変数の名前の色、`int main(void)` の `main` は関数の名前の色、`struct point` の `point` と `typedef ... point_t` の `point_t` と `size_t n` の `size_t` は型の色です。これは cc-mode の宣言の推測 (`c-font-lock-declarations`) によるもので、字句だけでは決まりません。段階 3 で扱い、段階 1 と 2 の Emacs との比較では、これらの face を比べる対象から外します。
- コメントの区切りの色は、隣の空白 1 つを含みます (`/*` の後、`*/` の前、`//` の後の空白)。
- 比べ方: `emacs -Q --batch` で、`c-mode` と `c++-mode` の `font-lock-ensure` を実行して、各文字の face を出して比べます。`tests/org-lint/` と同じ、Emacs との比較の枠組みを、`tests/c-highlight/` に作ります。
- キーワードと型の表は、Emacs の `cc-langs.el` (`c-primitive-type-kwds`、`c-constant-kwds` など) を元にします。

## 4. 言語とモードの結び付け

- 段階 1 で、核の `auto-mode-alist` (`lisp/files.scm`) に、`.c` `.h` (C) と、`.cc` `.cpp` `.cxx` `.hh` `.hpp` (C++) を足します。方言 (`Dialect`) に `C` と `Cpp` を足し、`set-syntax-dialect`、`syntax-dialect`、`major-mode-name` (`c-mode`、`c++-mode`) も対応させます。構文表は、Emacs の `c-mode-syntax-table` に合わせます。
- コマンド `c-mode` と `c++-mode` を足します (方言を変える)。
- 画面の `modes.ts` にも、同じファイル名の判定と、モードラインの名前 (`C`、`C++`) を足します。`language` は、C と C++ では `null` のままにして、色は核の字句 (5 章) で付けます。
- `modes.ts` の判定を、核の判定に一本化する作業は、別の作業にします。
- 既存の `highlight-regexp` は、そのまま残します。ハイライトは、「字句の種類」の層と、「`highlight-regexp`」の層の 2 つです。後者が上に重なります (Emacs の hi-lock と同じ)。

## 5. フロントへの渡し方

### Tauri (画面)

- `frame.rs` が作る `Frame` に、表示している行の `Span` を足します。種類は、文字列 (`"keyword"` など) で渡します。
- JS 側 (`src/lib/faces.ts`) は、種類を CSS のクラス (`.tok-keyword` など) に変えて、装飾にします。色はテーマ (明るい / 暗い) で決めます。
- Scheme と Elisp は、今の CodeMirror の仕組みのままにします。

### TUI (ncurses)

- `tui/src/ncurses_screen.rs` で、色を始めて (`start_color`、`use_default_colors`)、種類ごとの色の対 (`init_pair`) を作ります。種類から色の対への表は、1 か所に置きます。画面のマス (`tui/src/screen.rs` の `Cell`) に、今の `reverse` と `underline` と並べて、字句の種類を足します。`tui/src/render.rs` が、`Span` からマスの種類を決めます。
- 端末が色に対応していなければ (`has_colors` が偽)、色を付けません。`highlight-regexp` の下線は、そのままです。
- 既定は 8 色 (青、緑、シアン、マゼンタ、黄) です。256 色の端末で Emacs に近い色にするのは、後で足します。
- マルチバイトと全角の幅は、今の描画の処理に従います。

## 6. テストと 100% カバレッジ

- 核の字句解析器は、単体テストで、種類ごと、行をまたぐ状態ごとに確かめます (`core/tests/test_highlight_c.rs`)。
- Emacs との比較 (`npm run test:c-highlight`) は、`tests/c-highlight/cases/` の C と C++ のファイルを、Emacs と rs-editor の両方で処理して、文字ごとの種類を比べます。ケースは、コメント、文字列と文字定数のエスケープ、行の継続、プリプロセッサ、キーワード、型、定数を含めます。
- TUI の側は、`render.rs` が作るマスの種類と、種類から色の対への表を、単体テストします。Tauri の側は、種類から CSS のクラスへの変換を、単体テストします。
- 100% の行カバレッジ (`npm run coverage:be`) を、守ります。状態の分岐が多いので、状態の表を使い、到達できない分岐を作らない書き方にします。
- ja.po: 新しい公開コマンド (`c-mode`、`c++-mode`) と変数の docstring を、`npm run po:update` で足して、日本語に訳します。

## 7. 決定事項

1. **範囲**: 段階 1 は C だけ、段階 2 で C++ を足す。
2. **数値の色**: Emacs に合わせて、既定は色なし。
3. **モード判定**: 段階 1 は、核の `auto-mode-alist` と方言に C と C++ を足し、`modes.ts` にもモードラインの名前を足す。両者の一本化は別の作業。
4. **TUI の色**: 8 色を既定にして、256 色は後で足す。

## 8. 実装の順序

1. `core/src/highlight/` に `TokenKind`、`Span`、`State`、`LanguageSpec` と、C の字句解析器を作る。単体テスト。
2. 方言 `C`、`auto-mode-alist`、`c-mode` を足す。
3. `tests/c-highlight/` の Emacs との比較の枠組みを作り、C のケースを通す。
4. `frame.rs` と `faces.ts` で、Tauri に出す。
5. `tui/` で、ncurses の色を出す。
6. C++ の表と分岐を足し、比較のケースを足す。
7. TODO.md、ja.po、`.claude/CLAUDE.md` を更新する。
