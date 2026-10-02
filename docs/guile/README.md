# Guile リファレンスマニュアル（日本語訳）

GNU Guile 3.0.11 のリファレンスマニュアルの日本語訳です。
原文は <https://doc.guix.gnu.org/guile/latest/en/guile.html> で公開されています。
全体を章ごとのファイルに分割しており、ファイル名は `章_節_小節_タイトル.md` の形式です。

## 目次

全体の目次は [00_contents.md](00_contents.md) にあります。主な章は次のとおりです。

- [序文](00_preface.md)
- [1 はじめに](01_introduction.md)
- [2 こんにちは、ガイル！](02_hello_guile.md)
- [3 Hello Scheme!](03_hello_scheme.md)
- [4 Scheme でのプログラミング](04_programming_in_scheme.md)
- [5 C言語によるプログラミング](05_programming_in_c.md)
- [6 API リファレンス](06_00_api_reference.md)（`06_*`）
- [7 Guile モジュール](07_00_guile_modules.md)（`07_*`）
- [8 GOOPS](08_00_goops.md)（`08_*`）
- [9 Guile の実装](09_00_guile_implementation.md)（`09_*`）
- [脚注](99_footnotes.md)
- [GNU フリー文書利用許諾契約書](a_gnu_free_documentation_license.md)

## 索引

- [概念索引](index_concept.md)
- [手続き索引](index_procedure.md)
- [変数索引](index_variable.md)
- [型索引](index_type.md)
- [R5RS 索引](index_r5rs.md)

## 原文（英語）

[org/](org/00_contents.md) に、原文の英語版を日本語訳と同じ区切り・同じファイル名で分割して置いています（例: `org/06_06_01_booleans.md` は `06_06_01_booleans.md` の原文）。
本文中のリンクはオンライン版ではなく、`org/` 内の Markdown ファイルを指します。

- 全体の目次: [org/00_contents.md](org/00_contents.md)
- 索引: [概念](org/index_concept.md)、[手続き](org/index_procedure.md)、[変数](org/index_variable.md)、[型](org/index_type.md)、[R5RS](org/index_r5rs.md)
- 訳文と見比べるときは、同じファイル名のものを開いてください。

リンクの扱いについての注意:

- 章・節への参照は、見出しに対応するファイルと見出しの位置へのリンクに置き換えています。
- 索引の項目など、見出しではない位置への参照は、その項目がある節のファイルまでしか指しません。
- GnuTLS など別のマニュアルへの参照（`gnutls-guile.html`、`r6rs.html` など）は、元のオンラインのリンクのままです。

## その他のファイル

- `guile_reference_manual_ja.md`、`guile_reference_manual_en.md`、`guile_reference_manual_en.html`、`guile_reference_manual.pdf` — 分割前の全文（日本語・英語）と PDF 版です。`guile_reference_manual_en.md` は `org/` の分割元で、リンクはオンライン版を指したままです。
- `old/` — 分割・整形前の旧版です。

## 注意

- 機械翻訳を元にしているため、用語の揺れや誤訳が含まれる場合があります。正確な内容は原文で確認してください。
- 本マニュアルは GNU フリー文書利用許諾契約書 1.3 以降の下で配布されています。詳細は [00_contents.md](00_contents.md) と [a_gnu_free_documentation_license.md](a_gnu_free_documentation_license.md) を参照してください。
