# C / C++ のシンタックスハイライト 設計方針

状態: 提案 (実装前)。承認されたら、この方針で実装します。

## 1. 目的と範囲

- C と C++ のソースに、色を付けます。Tauri の画面と ncurses の TUI の**両方**で、同じ結果にします。
- そのため、字句の解析は、核 (`core/`、Rust) に置きます。TypeScript (CodeMirror の `StreamLanguage`) では、TUI で使えないためです。
- 目標は Emacs の `c-mode` / `c++-mode` の font-lock (標準の段階。`font-lock-maximum-decoration` の既定) と、同じ種類の色が付くことです。
- 範囲外 (後回し): 字下げ (`c-indent`)、構文木、意味の解析 (関数の定義と呼び出しの区別など)、C 以外の言語 (Rust、Python など)。ただし、言語を足せる形にします (4 章)。

### 段階

| 段階 | 内容 |
|:---|:---|
| 1 | 核の字句解析器 (C)。Tauri と TUI に色を出す。Emacs と比べる |
| 2 | C++ (キーワード、`::`、テンプレート、raw 文字列、`//` コメントの継続行) |
| 3 | 範囲を更新する効率化 (再解析の範囲を絞る)。必要なら |

## 2. 今の状況 (調べた結果)

- 色は、Scheme と Elisp だけです。`src/lib/lispMode.ts` (CodeMirror) で付けるため、Tauri だけです。
- `highlight-regexp` は、正規表現と face をバッファに持ちます。TUI は下線を引くだけで、色は使っていません (ncurses の `init_pair` を使っていません)。
- メジャーモードは `src/lib/modes.ts` (TypeScript) にあります。`language` は `'scheme' | 'elisp' | null` です。C のモードも、`.c` `.h` の自動判定も、ありません。

## 3. 方針の中心: 核が「範囲と種類」を返す

核は、色を知りません。「バッファの、どこからどこまでが、どの種類か」だけを返します。色への割り当ては、各フロントが行います。

```rust
/// 字句の種類 (Emacs の font-lock の face に対応する).
pub enum TokenKind {
    Comment,          // font-lock-comment-face
    CommentDelimiter, // font-lock-comment-delimiter-face (// や /*)
    String,           // font-lock-string-face
    Keyword,          // font-lock-keyword-face (if, for, struct ...)
    Type,             // font-lock-type-face (int, char, size_t ...)
    Constant,         // font-lock-constant-face (NULL, true, 大文字の定数 ...)
    Builtin,          // font-lock-builtin-face
    Preprocessor,     // font-lock-preprocessor-face (#include ...)
    Number,           // (Emacs の c-mode には既定で無い。後述)
    FunctionName,     // font-lock-function-name-face (定義の名前)
    Variable, Warning // 必要になったら
}

pub struct Span { pub start: usize, pub end: usize, pub kind: TokenKind } // 文字 (char) の位置

pub trait Highlighter {
    /// 行 `line` の字句を返す。`state` は、前の行の終わりの状態 (ブロックコメントの中、など)。
    fn highlight_line(&self, text: &str, state: State) -> (Vec<Span>, State);
}
```

### 3.1 解析の方式: 行ごとの状態機械

- 正規表現ではなく、手書きの字句解析器にします。理由は、ブロックコメント `/* */`、文字列、`'\''`、行の継続 (`\` で終わる行)、C++ の raw 文字列 `R"x(...)x"` が、行をまたぐためです。行ごとの正規表現では、扱えません。
- 状態は小さな値です (`Normal`、`BlockComment`、`String` の継続、`RawString(区切り)`、`PreprocessorContinued`)。行の終わりの状態を、バッファが行ごとに覚えます。
- 編集したら、その行から先を、状態が前と同じになるまで再解析します (段階 3 で効率化。段階 1 は、表示している範囲の前の行から計算します)。
- 解析器は、Rust の `core/src/highlight/` に置きます。言語ごとの違い (キーワードの表、コメントの形式) は、データ (`LanguageSpec`) にして、字句解析器は 1 つにします。C と C++ は、表の違いと、少しの分岐だけです。

### 3.2 Emacs に合わせる点

- `c-mode` の font-lock は、コメント、文字列、プリプロセッサ、キーワード、型、定数、ラベルです。**数値には色を付けません**。`Number` を出すかは、Emacs と同じにするか (色なし) を、実装の最初に決めます (提案: 既定は Emacs と同じ。設定で有効にできるようにする)。
- 色の対応は、Emacs 30 の `emacs -Q --batch` で、`c-mode` と `c++-mode` の `font-lock-ensure` を実行して、各文字の face を出して比べます。`tests/org-lint/` と同じ、Emacs との比較の枠組みを、`tests/c-highlight/` に作ります。
- キーワードと型の表は、Emacs の `cc-fonts.el` と `cc-langs.el` の `c-primitive-type-kwds` などを、元にします。

## 4. 言語とモードの結び付け

- 今は、`modes.ts` (TypeScript) が、モードと言語を決めています。TUI が同じモードの判定を使うには、核に、同じ判定が要ります。
- 提案: 核に `major_mode` の情報 (名前、`auto-mode-alist` の表、言語) を持たせます。Scheme から `(c-mode)` と `(c++-mode)` のコマンドで切り替えます。`modes.ts` は、核の判定を呼ぶだけにします (二重管理をやめる)。
- ただし、段階 1 では、`modes.ts` の変更を、小さくします。`language` に、`'c' | 'cpp'` を足し、核の判定は、`.c` `.h` `.cc` `.cpp` `.cxx` `.hpp` `.hh` と `-*- mode: c -*-` だけを、核に置きます。
- 既存の `highlight-regexp` は、そのまま残します。ハイライトは、「字句の種類」の層と、「`highlight-regexp`」の層の 2 つです。後者が上に重なります (Emacs の hi-lock と同じ)。

## 5. フロントへの渡し方

### Tauri (画面)

- `frame.rs` が作る `Frame` に、表示している行だけの `Vec<Span>` を足します。種類は、文字列 (`"keyword"` など) で渡します。
- JS 側 (`src/lib/faces.ts`) は、種類を CSS のクラス (`.tok-keyword` など) に変えて、装飾にします。色はテーマ (明るい / 暗い) で決めます。
- Scheme と Elisp は、今の CodeMirror の仕組みのままにします。将来、核に移せますが、今回は触りません。

### TUI (ncurses)

- `tui/src/screen.rs` に、色の対を足します (`start_color`、`init_pair`)。種類から色の対への表を、1 か所に置きます。端末が色に対応していなければ、色を付けません (下線の `highlight-regexp` は、そのままです)。
- 8 色 / 256 色の端末の両方で、見えるようにします。既定は 8 色 (青、緑、シアン、マゼンタ、黄) で、256 色の端末が使えるときに、Emacs に近い色にします。
- 画面の描画は、行ごとに、`Span` を順に ncurses の属性 (`attron`) に変えます。マルチバイトと全角の幅は、今の描画の処理に従います。

## 6. テストと 100% カバレッジ

- 核の字句解析器は、単体テストで、種類ごと、行をまたぐ状態ごとに確かめます (`core/tests/test_highlight_c.rs`)。
- Emacs との比較 (`npm run test:c-highlight`) は、`tests/c-highlight/cases/*.c, *.cpp` を、Emacs と rs-editor の両方で処理して、文字ごとの種類を比べます。ケースは、標準の C の書き方の全体 (コメントと文字列の入れ子、`#define` の継続行、三文字表記、raw 文字列) をカバーするようにします。
- Tauri と TUI の側は、表への変換を、単体テストします。ncurses の描画は、既存の TUI の試験の方法に従います。
- 100% の行カバレッジ (`npm run coverage:be`) を、守ります。字句解析器は、状態の分岐が多いので、「状態の表」を使い、到達できない分岐を作らない書き方にします。
- ja.po: 新しい公開コマンド (`c-mode` など) の docstring を、`npm run po:update` で足して、日本語に訳します。

## 7. 決めたいこと (承認のときに、教えてください)

1. **範囲**: 段階 1 は C だけ、段階 2 で C++ でよいですか (提案)。
2. **数値の色**: Emacs に合わせて、既定は色なしでよいですか (提案)。それとも、数値にも色を付けますか。
3. **モード判定**: 段階 1 は、`modes.ts` に `'c' | 'cpp'` を足す小さな変更でよいですか。核に判定を移すのは、別の仕事にします (提案)。
4. **TUI の色**: 8 色を既定にして、256 色は、後で足す形でよいですか (提案)。

## 8. 実装の順序 (承認後)

1. `core/src/highlight/` に `TokenKind`、`Span`、`State`、`LanguageSpec` と、C の字句解析器を作る。単体テスト。
2. `tests/c-highlight/` の Emacs との比較の枠組みを作り、C のケースを通す。
3. `frame.rs` と `faces.ts` で、Tauri に出す。
4. `tui/` で、ncurses の色を出す。
5. C++ の表と分岐を足し、比較のケースを足す。
6. TODO.md、ja.po、`.claude/CLAUDE.md` を更新して、1 つずつコミットする (push は、頼まれてから)。
