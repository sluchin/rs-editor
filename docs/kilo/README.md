# Build Your Own Text Editor（日本語訳）

Paul Smith 氏（snaptoken）によるチュートリアル「Build Your Own Text Editor」の日本語訳です。
C 言語で、antirez の [kilo](http://antirez.com/news/108) をベースにしたテキストエディタを 184 ステップで作ります。
原文は <https://viewsourcecode.org/snaptoken/kilo/> で公開されています。

## 目次

はじめに: [00_build_your_own_text_editor.md](00_build_your_own_text_editor.md)

1. [セットアップ](01_setup.md)
2. [生モードへの移行](02_entering_raw_mode.md)
3. [生の入出力](03_raw_input_and_output.md)
4. [テキストビューア](04_a_text_viewer.md)
5. [テキストエディタ](05_a_text_editor.md)
6. [検索](06_search.md)
7. [構文ハイライト](07_syntax_highlighting.md)
8. [付録](08_appendices.md)

## その他のファイル

- [build_your_own_text_editor_ai/](build_your_own_text_editor_ai/README.md) — 各章の内容をまとめた日本語の解説ノートです。翻訳ではなく、原文の構成に沿った要約です。
- `old/` — 旧版のメモ（`ARCHITECTURE.md`、`IMPLEMENTATION_GUIDE.md`）です。

## ライセンス

- チュートリアルの本文は CC BY 4.0 で公開されています。
- チュートリアル中の kilo のコードは BSD 2-clause（原作者 Salvatore Sanfilippo / antirez）で公開されています。
