# 未実装の Emacs デフォルト機能

rs-editor が Emacs の初期状態では実装していない機能を以下にまとめました。

## ✅ 実装済み（Emacs デフォルト機能）

**カーソル移動・移動コマンド**
- C-f, C-b - 前後に移動
- C-n, C-p - 上下に移動
- C-a, C-e - 行の開始・末尾
- M-f, M-b - 単語単位の移動
- M-<, M-> - バッファの先頭・末尾

**削除・編集**
- C-d - 1 文字削除
- C-k - 行削除（kill-line）
- C-w - 選択範囲削除（kill-region）
- C-y - 貼り付け（yank）
- M-y - 別の削除内容を貼り付け（yank-pop）
- C-t - 文字入れ替え
- C-/, C-x C-/ - Undo / Redo

**リージョン選択**
- C-SPC - マーク設定
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

**その他**
- M-x - コマンド実行
- C-g - キャンセル（keyboard-quit）

---

## ❌ 未実装の Emacs デフォルト機能

### 高優先度（よく使う機能）

| キー | コマンド | 説明 |
|------|---------|------|
| `C-x C-c` | save-buffers-kill-emacs | アプリを終了 |
| `C-l` | recenter-top-bottom | 画面を中央にスクロール |
| `C-v` / `M-v` | scroll-down / scroll-up | ページ単位のスクロール |
| `M-d` | kill-word | 単語を削除 |
| `M-DEL` | backward-kill-word | 単語を後退削除 |
| `C-x C-n` | set-goal-column | ゴールカラム設定 |
| `M-g g` | goto-line | 指定行へジャンプ |

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
| `DEL` | delete-backward-char | 1 文字後退削除 |
| `M-u` | upcase-word | 単語を大文字化 |
| `M-l` | downcase-word | 単語を小文字化 |
| `M-c` | capitalize-word | 単語を先頭大文字化 |

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

### Phase 8（提案）: 必須機能の追加

高優先度機能を実装することで、使いやすさが大幅に向上します：

1. **C-l (recenter)** - スクロール位置を調整（簡単）
2. **C-v / M-v (page-down / page-up)** - ページ送り（中程度）
3. **C-x C-c (exit)** - アプリ終了確認（簡単）
4. **M-d / M-DEL (kill-word)** - 単語削除（中程度）
5. **M-g g (goto-line)** - 行ジャンプ（中程度）

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
- ❌ ページスクロール（C-v/M-v）
- ❌ ウィンドウ分割
- ❌ ヘルプシステム
- ❌ マクロ機能
- ❌ 複雑な検索・置換

**実装の状態:**

Scheme エディタとしての**コア機能は十分実装されている**と言えます。**次のステップ**として：
1. 未接続の括弧補完を有効化する（簡単）
2. 未接続の REPL をマウントする（簡単）
3. 高優先度の Emacs 機能を実装する（C-l, C-v/M-v など）
