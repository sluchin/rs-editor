# 序文

> **原文**: [Guile Reference Manual - Preface](https://www.gnu.org/software/guile/manual/html_node/Preface.html)
>
> このドキュメントは GNU Free Documentation License の下で公開されている原文の翻訳です。

このマニュアルでは、GNU の拡張用ユビキタス知的言語（GNU's Ubiquitous Intelligent Language for Extensions）である Guile の使い方を説明します。特に Guile バージョン 3.0.11 に関するものです。

- このマニュアルの貢献者
- Guile のライセンス

## このマニュアルの貢献者

Guile 自体と同様に、Guile リファレンスマニュアルも生き物のような存在であり、長い期間にわたって多くの人々によって手入れされてきました。そのため、「そう、この一人の人物がこのマニュアルを書いた」と言えるような個人を特定するのは困難です。

それでも、多くの貢献の中で、際立った世話役が何人かいます。まず第一に挙げるべきは Neil Jerram で、彼は10年以上にわたってこの文書に取り組んできました。細部と全体像の両方に対する Neil の注意力は、一世代の Guile ハッカーたちの理解に真の違いをもたらしました。

次に、この文書に対する Marius Vollmer の影響に触れておくべきでしょう。Marius は Guile の API が明確化された――いわば火にかけられて鍛えられた――時期に Guile のメンテナを務めており、マニュアルにも同じ変化をもたらすという良識を持っていました。

Martin Grabmueller は Guile 1.6 リリースの準備としてマニュアル全体にわたり多大な貢献をしました。その中には、Scheme のデータ型、制御機構、手続きに関するドキュメントの多くを充実させたことが含まれます。さらに、彼は Guile の SRFI モジュールと、Guile REPL に関連するモジュールのドキュメントも執筆しました。

2010年から Guile を共同でメンテナンスしている Ludovic Courtès と Andy Wingo、そして Mark Weaver もまた、Guile 2.0 とともに登場した新しいモジュールやサブシステムのドキュメントを書くことで、マニュアルに足跡を残しました。Ludovic、Andy、Mark は、Guile が進化する中で既存の文章が妥当性を保つようにする責任も負っています。このマニュアルの問題を報告する方法の詳細については、「バグの報告」を参照してください。

このマニュアルの最初のバージョンの内容は、Guile の基盤となった SCM システムの作者である Aubrey Jaffer の文書と、Guile の最初のメンテナである Tom Lord の文書を取り入れ、またそれらから着想を得ていました。この文章の大部分は書き直されましたが、そのすべてが重要であり、構成の一部は今も残っています。

Guile の最初のバージョンのマニュアルは、主に Mark Galassi と Jim Blandy によって執筆、編集、編纂されました。特に Jim は、Guile のデータ表現と、Guile オブジェクトにアクセスするための C API に関する元のチュートリアルを書きました。

Thien-Thi Nguyen、Kevin Ryde、Mikael Djurfeldt、Christian Lynbech、Julian Graham、Gary Houston、Tim Pierce、その他数十名の人々からも、相当な部分が寄稿されました。読者であるあなたも、この尊敬すべき人々の仲間に加わることを大歓迎します。参加方法については、Guile のウェブサイト http://www.gnu.org/software/guile/ を訪れてください。

## Guile のライセンス

Guile はフリーソフトウェアです。Guile は著作権で保護されており、パブリックドメインではありません。その配布や再配布には制限がありますが、これらの制限は、協力的な人が行いたいと思うことはすべて許可するように設計されています。

- Guile ライブラリ（libguile）とそのサポートファイルは、GNU Lesser General Public License バージョン3以降の条件の下で公開されています。ファイル `COPYING.LESSER` と `COPYING` を参照してください。
- Guile の readline モジュールは、GNU General Public License バージョン3以降の条件の下で公開されています。ファイル `COPYING` を参照してください。
- あなたが今読んでいるマニュアルは、GNU Free Documentation License の条件の下で公開されています（「GNU Free Documentation License」を参照）。

Guile ライブラリにリンクする C コードは、そのライブラリの条件に従います。基本的に、そのようなコードは、ユーザーが新しいバージョンまたは変更されたバージョンの Guile に対して再リンクできる限り、どのような条件でも公開できます。

Guile readline モジュールにリンクする C コードは、そのモジュールの条件に従います。基本的に、そのようなコードはフリーな条件で公開しなければなりません。

Guile で実行されるために書かれた（ただし Guile 自体から派生したものではない）Scheme レベルのコードは、いかなる制限も受けず、どのような条件でも公開できます。私たちは、作者がフリーな条件で公開することを推奨します。

Guile には一切の保証がないことを認識しておく必要があります。このことはライセンスの中で詳しく説明されています。

---

> **ライセンス**: この翻訳は GNU Free Documentation License v1.3 以降に基づいて作成されています。
> 原文の著作権: Copyright (C) 1996-2023 Free Software Foundation, Inc.
