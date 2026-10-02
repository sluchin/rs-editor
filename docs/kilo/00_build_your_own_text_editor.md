# 独自のテキストエディタを作成する

ようこそ！これは、C言語でテキストエディタを作成する方法を説明する説明書です。

テキストエディタは [antirez の kilo](http://antirez.com/news/108) をベースに、いくつかの変更を加えたものです。依存関係のない単一ファイルに約 1000 行の C 言語が記述されており、最小限のエディタに期待されるすべての基本機能に加え、構文ハイライトと検索機能も実装されています。

この小冊子では、**184ステップ**でエディタの構築手順を解説します。各ステップでは、数行のコードを追加、変更、または削除します。ほとんどのステップでは、プログラムをコンパイルして実行することで、**変更内容を確認できます**。

手順ごとに、時には詳細に説明します。文章はざっと目を通したり、読み飛ばしたりしても構いません。なぜなら、このチュートリアルの要点は、**テキストエディタをゼロから構築する**ことだからです。途中で学ぶことはすべておまけで、コードの変更を入力して結果を観察するだけでも、学ぶことはたくさんあります。

チュートリアル自体に関する詳細情報（行き詰まった場合の対処法やヘルプの入手先など）については、[付録](https://viewsourcecode.org/snaptoken/kilo/08.appendices.html)を参照してください。

準備ができたら、[第 1 章](https://viewsourcecode.org/snaptoken/kilo/01.setup.html) に進んでください。

## 目次

1. [セットアップ](https://viewsourcecode.org/snaptoken/kilo/01.setup.html)
2. [生モードへの移行](https://viewsourcecode.org/snaptoken/kilo/02.enteringRawMode.html)
3. [生の入出力](https://viewsourcecode.org/snaptoken/kilo/03.rawInputAndOutput.html)
4. [テキストビューア](https://viewsourcecode.org/snaptoken/kilo/04.aTextViewer.html)
5. [テキストエディタ](https://viewsourcecode.org/snaptoken/kilo/05.aTextEditor.html)
6. [検索](https://viewsourcecode.org/snaptoken/kilo/06.search.html)
7. [構文ハイライト](https://viewsourcecode.org/snaptoken/kilo/07.syntaxHighlighting.html)
8. [付録](https://viewsourcecode.org/snaptoken/kilo/08.appendices.html)

[snaptokenチュートリアルに戻る](https://viewsourcecode.org/snaptoken)
