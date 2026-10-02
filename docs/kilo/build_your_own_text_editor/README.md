# Build Your Own Text Editor 章別ガイド

このディレクトリは、Paul Smith 氏（snaptoken）によるチュートリアル「Build Your Own Text Editor」を読み進めるための、日本語の解説ノートです。

## 位置づけ

- **翻訳ではありません。** 原文の構成（章とステップの順序）に沿って、各段階で何を作り、何を学ぶのかを筆者の言葉でまとめたものです。
- 原文の本文は CC BY 4.0、チュートリアル中の kilo のコードは BSD 2-clause（原作者 Salvatore Sanfilippo / antirez）で公開されています。
- 詳細な説明や実際の手順は、必ず原文で確認してください。

## 出典

- チュートリアル: https://viewsourcecode.org/snaptoken/kilo/
- リポジトリ: https://github.com/snaptoken/kilo-tutorial （本文は CC BY 4.0）
- 元のエディタ kilo: https://github.com/antirez/kilo （BSD 2-clause）

## 目次

| 章 | ファイル | 内容 |
|---|---|---|
| 1 | `01_setup.md` | 開発環境と最小のプログラム |
| 2 | `02_entering_raw_mode.md` | 端末の raw モード |
| 3 | `03_raw_input_and_output.md` | キー入力と画面描画の基礎 |
| 4 | `04_a_text_viewer.md` | ファイルを表示するビューア |
| 5 | `05_a_text_editor.md` | 編集と保存 |
| 6 | `06_search.md` | 検索機能 |
| 7 | `07_syntax_highlighting.md` | 構文ハイライト |

既存の `docs/kilo/` 直下のドキュメントは、`kilo.c` そのものについての解説で、このガイドとは別のものです。
