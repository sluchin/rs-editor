# 開発環境のセットアップ

Docker を使わずに、手元の Linux (Debian / Ubuntu) で開発するときに必要なものです。

## 必要なもの

| 道具 | 版 | 使うところ |
| --- | --- | --- |
| Node.js | 22 (CI と同じ) | フロントエンドのビルドとテスト (`npm`) |
| Rust | stable (`rustup`) | `core/`、`src-tauri/`、`tui/` |
| Python 3 | 3.10 以上 | PO の更新 (`tools/po/update.py`)、`docs/functions.md` の生成 |
| Emacs | 30 (`emacs -Q --batch` が動くこと) | Emacs と動きを比べるテストの期待値の再生成、`docs/functions.md` の生成 |
| `cargo-tarpaulin` | 0.37 以上 | Rust のカバレッジ (100% を保つ) |
| `markdownlint-cli2` | `npm install` で入る | Markdown のチェック |

## システムのパッケージ

```bash
sudo apt-get update
sudo apt-get install -y \
  build-essential pkg-config curl wget file libssl-dev \
  libwebkit2gtk-4.1-dev libappindicator3-dev libayatana-appindicator3-dev \
  librsvg2-dev libsoup-3.0-dev libgtk-3-dev patchelf \
  libncurses-dev \
  python3 emacs
```

- `libncurses-dev` は、ターミナルのエディタ (`tui/`) に要ります。
- `emacs` は、版が 30 でないと、期待値 (`core/tests/*.json`) を作り直したときに差が出ます。`emacs --version` で確かめます。

## Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup component add clippy rustfmt llvm-tools-preview
cargo install cargo-tarpaulin
```

## Node.js の依存

```bash
npm install
```

`markdownlint-cli2`、Tauri CLI、vitest、ESLint などは、`devDependencies` に入っています。`npm install` を使わずに Markdown だけ確かめるなら、`sudo npm install -g markdownlint-cli2` で入れ、`markdownlint-cli2 docs/functions.md` のようにファイルを指定して動かします (`npx markdownlint` は別のパッケージ名なので動きません)。

## 動作の確認

```bash
npm run check          # 整形, Lint, テスト (CI と同じ)
npm run coverage       # カバレッジ (100% を保つ)
npm run po:check       # docstring の翻訳の作り直し忘れ
sh tools/emacs-commands/run.sh   # docs/functions.md の再生成
```

Emacs と比べるテストの期待値は、次で作り直します。

```bash
OUT=core/tests/edit.json emacs -Q --batch -l tools/syntax/edit.el
```

`cargo tarpaulin` は既定の時間切れ (60 秒) だと足りません。`--timeout 1200` を付けます。

```bash
cargo tarpaulin --workspace --skip-clean --engine llvm --timeout 1200
```
