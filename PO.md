# docstring の翻訳 (PO ファイル)

rs-editor の Lisp の関数と変数の docstring は英語で書き、翻訳は gettext の PO 形式のファイル (`po/*.po`) に置きます。
`C-h f` (`describe-function`) は、変数 `help-language` の言語の翻訳があれば翻訳を、無ければ英語の docstring を `*Help*` に出します。

## ファイル

| ファイル | 内容 |
| --- | --- |
| `lisp/*.scm` | docstring の元 (英語) |
| `po/rseditor.pot` | 全ての docstring の一覧 (テンプレート)。`tools/po/update.py` が作る生成物なので、手で直さない |
| `po/ja.po` | 日本語の翻訳。`msgstr` を手で書く |
| `tools/po/update.py` | `lisp/*.scm` から docstring を集めて、POT を作り、各言語の PO に混ぜる (gettext の `xgettext` と `msgmerge` に当たる) |
| `core/src/scheme/po.rs` | エディタが PO を読む所。PO はビルドのときに埋め込む (`include_str!`) |

PO の 1 つの項目は、次の形です。

```po
#: lisp/org.scm
msgctxt "function:org-cycle"
msgid "Move to the next table field, cycle the heading, or indent."
msgstr "表なら次の欄へ, 見出しなら開け閉め, ほかは字下げ (TAB)。"
```

- `msgctxt` は `function:名前` (関数とコマンド) か `variable:名前` (変数)。
- `msgid` は英語の docstring、`msgstr` はその翻訳です。

## 手順

### 1. docstring を英語で書く

- 関数とコマンド (`define`、`define-command`) は、引数の後に文字列を書きます。

  ```scheme
  (define (org-take items n)
    "Return the first N elements of ITEMS."
    ...)
  ```

  本体が文字列だけの関数では、その文字列は docstring になりません (値です)。docstring を付けるときは、その前に書きます。
- 変数は、`define` の 3 つ目に書きます。

  ```scheme
  (define org-directory "~/org" "Directory where Org files are kept.")
  ```

- Emacs の書き方に合わせます。
  - 1 行目は、それだけで分かる 1 つの文にします (命令形: `Return ...`、`Move ...`)。
  - 引数は大文字で書きます (`ITEMS`、`N`)。
  - ほかの関数や変数は、`` `org-directory' `` のように書きます。
  - 文字列の中の `"` と `\` は、`\"` と `\\` にします。

### 2. PO を作り直す

```sh
npm run po:update
```

`tools/po/update.py` が、次のことをします。

- `lisp/*.scm` の外側の `define` と `define-command` から docstring を集め、`po/rseditor.pot` を作り直す。
- 各言語の PO (`po/*.po`) に混ぜる。
  - 新しい docstring は、`msgstr ""` (訳が空) の項目として足す。
  - docstring の文が変わったものは、今までの訳に `#, fuzzy` の印を付け、元の英文を `#| msgid` に残す。
  - 無くなった docstring の項目は消す。
- 言語ごとに、訳した数、`fuzzy` の数、訳していない数を出す。

```text
po/ja.po: 534 translated, 0 fuzzy, 0 untranslated
```

### 3. 訳す

`po/ja.po` を開いて、次のように直します (エディタで直すか、Poedit などの翻訳の道具を使う)。

- 訳していない項目: `msgstr ""` に訳を書く。
- `fuzzy` の項目: `#| msgid` (元の英文) と今の `msgid` を比べて訳を直し、`#, fuzzy` の行と `#|` の行を消す。
  `fuzzy` のままの訳は使われず、英語の docstring が出る。

訳の書き方:

- 文の終わりは「。」、文の中の区切りは「,」にします (`po/ja.po` の、ほかの訳に合わせる)。
- 引数の名前 (`ITEMS` など) は、英語のまま残します。
- 改行の位置は、英語と同じでなくてもかまいません。
- コマンドの訳には、キーを添えてもかまいません (`"今の木を浅くする (M-S-左)。"`)。

### 4. 確かめる

```sh
npm run po:update   # 訳していない数と fuzzy の数が 0 か
npm run check       # po:check (作り直し忘れ) を含む
```

- `npm run po:check` (`update.py --check`) は、ファイルを書かずに、作り直しが要れば失敗します。docstring を変えて `npm run po:update` を忘れると、`npm run check` (CI) で見つかります。
- PO はビルドのときに埋め込むので、アプリで見るには、ビルドし直します。
- アプリで `C-h f` を押し、関数の名前を入れて、訳が出るかを見ます。英語と比べるときは、`M-:` で `(set! help-language "en")` にします (戻すときは `"ja"`)。

## 言語を足す

例えばフランス語 (`fr`) を足すとき。

1. `po/rseditor.pot` を `po/fr.po` に写す。始めの `#` の行 (説明) をフランス語の訳のものに書き換え、ヘッダの `msgstr` に `"Language: fr\n"` の行を足す (`po/ja.po` のヘッダと同じ形)。
2. `npm run po:update` を実行する (`po/*.po` は、全て作り直す)。
3. `core/src/scheme/po.rs` の `LANGUAGES` に、`("fr", include_str!("../../../po/fr.po"))` を足す。
4. `msgstr` を訳し、上の「4. 確かめる」をする。

## どの言語を出すか

- 変数 `help-language` の言語の訳を出します。既定は、環境変数 `LANG` から決めます (`ja_JP.UTF-8` なら `ja`。`C` と `POSIX` と空なら `en`)。
- 初期設定ファイル (`~/.rseditor`) に `(set! help-language "en")` と書けば、いつも英語です。
- 訳の無い言語や、訳の無い項目は、英語の docstring を出します。
