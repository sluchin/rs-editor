# 未実装の Emacs デフォルト機能

rs-editor が Emacs の初期状態では実装していない機能を以下にまとめました。

## ✅ 実装済み（Emacs デフォルト機能）

**カーソル移動・移動コマンド**
- C-f, C-b - 前後に移動
- C-n, C-p - 上下に移動 (続けて押すあいだ, 桁を保つ)
- C-a, C-e - 行の開始・末尾
- M-f, M-b - 単語単位の移動
- M-<, M-> - バッファの先頭・末尾
- C-v, M-v - ページ単位のスクロール
- C-l - 画面を中央にスクロール (続けて押すと, 上, 下)
- M-g g - 指定行へジャンプ

**削除・編集**
- C-d - 1 文字削除
- C-k - 行削除（kill-line。続けたキルは 1 つにつなげる）
- M-d, M-DEL - 単語削除（kill-word, backward-kill-word）
- C-w, M-w - 選択範囲の削除とコピー（kill-region, kill-ring-save）
- C-y - 貼り付け（yank）
- M-y - 別の削除内容を貼り付け（yank-pop）
- C-t - 文字入れ替え
- M-u, M-l, M-c - 単語の大文字・小文字
- C-o - 改行を入れる（open-line）
- C-/, C-x C-/ - Undo / Redo（バッファごと）

**リージョン選択**
- C-SPC - マーク設定（移動で範囲が広がる。transient-mark-mode）
- C-x C-x - カーソルとマークの入れ替え
- C-x h - バッファ全体選択

**検索**
- C-s, C-r - 前方・後方検索（isearch）

**ファイル操作**
- C-x C-f - ファイルを開く（find-file）
- C-x C-s - 保存（save-buffer）

**バッファ管理**
- C-x b - バッファ切り替え
- C-x k - バッファを閉じる
- C-x C-b - バッファ一覧
- C-x C-c - 終了（保存していないバッファがあれば確認）

**その他**
- M-x - コマンド実行
- C-g - キャンセル（keyboard-quit）

---

## ❌ 未実装の Emacs デフォルト機能

### 高優先度（よく使う機能）

| キー | コマンド | 説明 |
|------|---------|------|
| `C-x C-n` | set-goal-column | ゴールカラム設定 |

### 中優先度（検索・置換）

| キー | コマンド | 説明 |
|------|---------|------|
| `M-%` | query-replace | 検索・置換 |
| `C-M-s` | isearch-forward-regexp | 正規表現検索（前方） |
| `C-M-r` | isearch-backward-regexp | 正規表現検索（後方） |

### 中優先度（ウィンドウ分割）

| キー | コマンド | 説明 |
|------|---------|------|
| `C-x 2` | split-window-below | 縦分割 |
| `C-x 3` | split-window-right | 横分割 |
| `C-x 1` | delete-other-windows | 他ウィンドウ削除 |
| `C-x 0` | delete-window | 現在ウィンドウ削除 |
| `C-x o` | other-window | 別ウィンドウへ移動 |

### 低優先度（補助機能）

| キー | コマンド | 説明 |
|------|---------|------|
| `C-h` | help-command | ヘルプ（C-h k, C-h f など） |
| `C-h k` | describe-key | キーの説明表示 |
| `C-h f` | describe-function | 関数の説明表示 |
| `C-h v` | describe-variable | 変数の説明表示 |
| `C-x C-q` | read-only-mode | 読み込み専用モード |

### 高度な機能

| キー | コマンド | 説明 |
|------|---------|------|
| `C-x (` / `C-x )` | start/end-kbd-macro | マクロ記録開始・終了 |
| `C-x e` | call-last-kbd-macro | マクロ実行 |
| `C-u` | universal-argument | プレフィックス引数 |

---

## rs-editor 独自の実装機能

以下は Emacs にはなく、rs-editor が Scheme エディタとして実装した独自機能です。

| キー | コマンド | 説明 |
|------|---------|------|
| `C-M-f` / `C-M-b` | forward-sexp / backward-sexp | S 式単位の移動 |
| `C-M-k` | kill-sexp | S 式削除 |
| `C-x C-e` | eval-last-sexp | S 式を評価 |
| `M-:` | eval-expression | 式を入力して評価 |

---

## 実装の優先度別ガイドライン

### Phase 9（提案）: ウィンドウ管理

複数ウィンドウのサポートは複雑ですが、アプリケーションの利便性を大幅に向上させます。

### Phase 10（提案）: ヘルプシステム

`C-h` 系コマンドでキーバインドやコマンドの説明を表示する機能。

---

---

## 削除された機能

Emacs はシンタックスハイライトと括弧マッチングを Lisp で実装しており、これらはコア機能ではなくユーザー拡張です。同様に、括弧の自動補完も IDE 固有の機能で Emacs には無い。

**削除したもの:**
- ❌ **シンタックスハイライト** (`schemeTokenizer.ts`) - Lisp ユーザー拡張
- ❌ **括弧マッチハイライト** (`findMatchingBracket`, `findUnmatchedClose` from `bracketMatch.ts`) - Lisp ユーザー拡張
- ❌ **括弧自動補完** (`bracketAutoClose.ts`) - IDE 固有機能

**保持したもの:**
- ✅ **S-expression ナビゲーション** (`forwardSexp`, `backwardSexp`, `killSexp`) - Emacs Lisp に対応する機能

### 未接続の機能

| 機能 | ファイル | 状態 | 説明 |
|------|---------|------|------|
| **REPL コンポーネント** | `src/components/REPL.tsx` | ⚠️ **未接続** | 対話的 Read-Eval-Print Loop、履歴機能（`App.tsx` にマウントされていない） |

---

## まとめ

rs-editor は Emacs の**基本的な編集機能の大部分**を実装しており、特に：
- ✅ 全ての基本的なカーソル移動
- ✅ kill-ring ベースの削除・貼り付け
- ✅ Undo/Redo
- ✅ 検索機能
- ✅ ファイル・バッファ管理

**Emacs にはない独自実装済み機能:**
- ✅ S-expression ナビゲーション（C-M-f/b/k）
- ✅ Scheme 式評価（C-x C-e, M-:）
- ✅ REPL（**未接続**）

**Emacs デフォルト機能で未実装:**
- ❌ ウィンドウ分割
- ❌ ヘルプシステム
- ❌ マクロ機能
- ❌ 複雑な検索・置換

**実装の状態:**

Scheme エディタとしての**コア機能は十分実装されている**と言えます。**次のステップ**として：
1. 置換（M-%）と正規表現の検索を実装する
2. 前置引数（C-u）とキーボードマクロを実装する
3. Scheme の処理系を育てて, Lisp でカスタマイズできるようにする
