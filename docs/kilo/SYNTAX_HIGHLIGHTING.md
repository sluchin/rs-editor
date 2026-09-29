# Kilo - シンタックスハイライトガイド

このドキュメントは, Kilo のシンタックスハイライトシステムについて説明します.

## シンタックスハイライトの概要

### 目的
シンタックスハイライトは, プログラムコードの異なる部分 (キーワード, コメント, 文字列など) に異なる色を適用し, コードの可読性を向上させます.

### Kilo での実装方法
1. **ファイル拡張子に基づく言語検出**.
2. **行のスキャンと トークン識別**.
3. **各文字のハイライトタイプ割り当て**.
4. **ANSI エスケープシーケンスで色を適用**.

## ハイライトタイプ

Kilo では, 次のハイライトタイプが定義されています.

```c
#define HL_NORMAL 0       // デフォルト色 (白).
#define HL_COMMENT 1      // コメント (シアン).
#define HL_MLCOMMENT 2    // マルチラインコメント (シアン).
#define HL_KEYWORD1 3     // 言語キーワード (黄).
#define HL_KEYWORD2 4     // 型キーワード (緑).
#define HL_STRING 5       // 文字列リテラル (赤).
#define HL_NUMBER 6       // 数値リテラル (マゼンタ).
#define HL_MATCH 7        // 検索マッチハイライト (青).
```

### 色の対応
| ハイライトタイプ | 色 | 用途 |
|-----------------|-----|------|
| `HL_NORMAL` | 白 (37) | 通常のテキスト |
| `HL_COMMENT` | シアン (36) | 単一行コメント |
| `HL_MLCOMMENT` | シアン (36) | マルチラインコメント |
| `HL_KEYWORD1` | 黄 (33) | 制御フロー (if, for など) |
| `HL_KEYWORD2` | 緑 (32) | 型キーワード (int, char など) |
| `HL_STRING` | 赤 (31) | 文字列 ("..." や '...') |
| `HL_NUMBER` | マゼンタ (35) | 数値リテラル (123, 3.14 など) |
| `HL_MATCH` | 青 (34) | 検索マッチ |

## 言語定義

### editorSyntax 構造体
```c
struct editorSyntax {
    char *filetype;                    // 言語名 ("C", "Makefile" など).
    char **filematch;                  // ファイルマッチパターン.
    char **keywords;                   // キーワード配列.
    char *singleline_comment_start;    // 単一行コメント開始 ("//" など).
    char *multiline_comment_start;     // マルチラインコメント開始 ("/*" など).
    char *multiline_comment_end;       // マルチラインコメント終了 ("*/" など).
    int flags;                         // オプションフラグ.
};
```

### C 言語の定義例
```c
struct editorSyntax HLDB[] = {
    {
        "c",
        {"*.c", "*.h", "*.cpp", NULL},
        keywords,
        "//",  // 単一行コメント.
        "/*", "*/",  // マルチラインコメント.
        HL_HIGHLIGHT_STRINGS | HL_HIGHLIGHT_NUMBERS  // フラグ.
    },
    // ... 他の言語定義 ...
    {NULL, NULL, NULL, NULL, NULL, NULL, 0}  // 終了マーカー.
};
```

### キーワード配列
```c
char *C_HL_keywords[] = {
    // キーワード1 (制御フロー).
    "if", "else", "while", "for", "do", "switch",
    "case", "default", "break", "continue", "return",
    "goto", "asm", "auto",
    
    // キーワード2 (型) - | で区切られる.
    "int|", "long|", "double|", "float|", "char|",
    "unsigned|", "signed|", "void|", "const|", "static|",
    "struct|", "union|", "enum|", "typedef|",
    
    NULL  // 終了マーカー.
};
```

**キーワード2 の識別:**
パイプ文字 (|) で区切られたキーワードは, `HL_KEYWORD2` として扱われます. これにより, 型キーワードを異なる色で表示できます.

## ハイライト処理

### editorUpdateSyntax() 関数
```c
void editorUpdateSyntax(erow *row) {
    row->hl = realloc(row->hl, row->rsize);
    memset(row->hl, HL_NORMAL, row->rsize);
    
    if (E.syntax == NULL) return;
    
    char **keywords = E.syntax->keywords;
    char *scs = E.syntax->singleline_comment_start;
    char *mcs = E.syntax->multiline_comment_start;
    char *mce = E.syntax->multiline_comment_end;
    
    int scs_len = scs ? strlen(scs) : 0;
    int mcs_len = mcs ? strlen(mcs) : 0;
    int mce_len = mce ? strlen(mce) : 0;
    
    int prev_sep = 1;  // 前の文字が区切り文字か.
    int in_string = 0;
    int in_comment = 0;
    
    int i = 0;
    while (i < row->rsize) {
        char c = row->render[i];
        unsigned char prev_hl = (i > 0) ? row->hl[i - 1] : HL_NORMAL;
        
        // マルチラインコメントの処理.
        if (in_comment) {
            row->hl[i] = HL_MLCOMMENT;
            if (strncmp(&row->render[i], mce, mce_len) == 0) {
                memset(&row->hl[i], HL_MLCOMMENT, mce_len);
                i += mce_len;
                in_comment = 0;
                prev_sep = 1;
                continue;
            } else {
                i++;
                continue;
            }
        }
        
        // マルチラインコメント開始.
        if (!in_string && strncmp(&row->render[i], mcs, mcs_len) == 0) {
            memset(&row->hl[i], HL_MLCOMMENT, mcs_len);
            i += mcs_len;
            in_comment = 1;
            continue;
        }
        
        // 単一行コメント.
        if (!in_string && strncmp(&row->render[i], scs, scs_len) == 0) {
            memset(&row->hl[i], HL_COMMENT, row->rsize - i);
            break;
        }
        
        // 文字列処理.
        if (E.syntax->flags & HL_HIGHLIGHT_STRINGS) {
            if (in_string) {
                row->hl[i] = HL_STRING;
                if (c == '\\' && i + 1 < row->rsize) {
                    row->hl[i + 1] = HL_STRING;
                    i += 2;
                    continue;
                }
                if (c == in_string) in_string = 0;
                i++;
                prev_sep = 1;
                continue;
            } else {
                if (c == '"' || c == '\'') {
                    in_string = c;
                    row->hl[i] = HL_STRING;
                    i++;
                    continue;
                }
            }
        }
        
        // 数値処理.
        if (E.syntax->flags & HL_HIGHLIGHT_NUMBERS) {
            if ((isdigit(c) && (prev_sep || prev_hl == HL_NUMBER)) ||
                (c == '.' && prev_hl == HL_NUMBER)) {
                row->hl[i] = HL_NUMBER;
                i++;
                prev_sep = 0;
                continue;
            }
        }
        
        // キーワード処理.
        if (prev_sep) {
            int j;
            for (j = 0; keywords[j]; j++) {
                int klen = strlen(keywords[j]);
                int kw2 = keywords[j][klen - 1] == '|';
                if (kw2) klen--;
                
                if (!strncmp(&row->render[i], keywords[j], klen) &&
                    is_separator(row->render[i + klen])) {
                    int hl = kw2 ? HL_KEYWORD2 : HL_KEYWORD1;
                    memset(&row->hl[i], hl, klen);
                    i += klen;
                    break;
                }
            }
            if (keywords[j] != NULL) {
                prev_sep = 0;
                continue;
            }
        }
        
        prev_sep = is_separator(c);
        i++;
    }
    
    int changed = (row->hlopen != in_comment);
    row->hlopen = in_comment;
}
```

### 処理の流れ
1. **初期化**: ハイライト配列を `HL_NORMAL` で初期化.
2. **行のスキャン**: 各文字を処理.
3. **コメント検出**: マルチラインコメントの開始・終了を検出.
4. **文字列検出**: クォーテーションを検出し, 文字列をハイライト.
5. **数値検出**: 数字のシーケンスを検出.
6. **キーワード検出**: 単語がキーワード配列に存在するか確認.
7. **状態保存**: 行の状態 (コメント内など) を保存.

## 言語検出

### editorSelectSyntaxHighlight() 関数
```c
void editorSelectSyntaxHighlight(void) {
    E.syntax = NULL;
    if (E.filename == NULL) return;
    
    char *ext = strrchr(E.filename, '.');
    
    for (unsigned int j = 0; HLDB[j].filetype; j++) {
        struct editorSyntax *s = &HLDB[j];
        unsigned int i = 0;
        
        while (s->filematch[i]) {
            int is_ext = (s->filematch[i][0] == '*');
            if ((is_ext && ext && !strcmp(ext, s->filematch[i] + 1)) ||
                (!is_ext && strstr(E.filename, s->filematch[i]))) {
                E.syntax = s;
                
                // 全行のハイライトを更新.
                int filerow;
                for (filerow = 0; filerow < E.numrows; filerow++) {
                    editorUpdateSyntax(&E.row[filerow]);
                }
                
                return;
            }
            i++;
        }
    }
}
```

**言語検出の優先順位:**
1. ファイル拡張子マッチング (`*.c`, `*.h` など).
2. ファイル名マッチング (Makefile など).
3. デフォルト (ハイライトなし).

## 新しい言語の追加

### ステップ 1: キーワード配列を定義
```c
char *PYTHON_HL_keywords[] = {
    "and", "as", "break", "class", "continue", "def", "del",
    "elif", "else", "except", "finally", "for", "from", "global",
    "if", "import", "in", "is", "lambda", "not", "or", "pass",
    "raise", "return", "try", "while", "with", "yield",
    
    // 型キーワード.
    "int|", "str|", "list|", "dict|", "tuple|", "bool|", "float|",
    
    NULL
};
```

### ステップ 2: 言語定義を追加
```c
struct editorSyntax HLDB[] = {
    {
        "python",
        {"*.py", NULL},
        PYTHON_HL_keywords,
        "#",                    // 単一行コメント.
        "\"\"\"", "\"\"\"",     // マルチラインコメント.
        HL_HIGHLIGHT_STRINGS | HL_HIGHLIGHT_NUMBERS
    },
    // ... 他の言語 ...
};
```

### ステップ 3: コメント開始文字の処理を更新 (必要に応じて)
複数文字のコメント開始 (例: `"""`) に対応する場合, `editorUpdateSyntax()` を調整してください.

## パフォーマンス最適化

### 遅延更新
- 表示される行のみハイライト更新.
- オフスクリーン行は次回スクロール時に更新.

### インクリメンタル更新
```c
// 1 行が変更されたとき.
void editorUpdateRow(erow *row) {
    editorUpdateSyntax(row);  // その行だけ更新.
}

// マルチラインコメント内にある次の行も更新.
if (i < E.numrows && (E.row[i].hlopen || prev_hlopen)) {
    editorUpdateSyntax(&E.row[i]);
}
```

### キャッシング
行が変更されない限り, ハイライト情報を再計算しません.

## カスタマイズ例

### Scheme 言語のハイライト追加
```c
char *SCHEME_HL_keywords[] = {
    "define", "lambda", "if", "cond", "case", "let", "let*",
    "letrec", "begin", "do", "quasiquote", "unquote", "quote",
    
    // 型関連.
    "number?|", "string?|", "boolean?|", "list?|", "symbol?|",
    "procedure?|",
    
    NULL
};

// HLDB に追加.
{
    "scheme",
    {"*.scm", "*.ss", NULL},
    SCHEME_HL_keywords,
    ";",     // 単一行コメント.
    NULL, NULL,  // マルチラインコメントなし.
    HL_HIGHLIGHT_STRINGS | HL_HIGHLIGHT_NUMBERS
}
```

### カスタム色の設定
```c
int editorSyntaxToColor(int hl) {
    switch(hl) {
        case HL_COMMENT:
        case HL_MLCOMMENT:
            return 36;  // シアン.
        case HL_KEYWORD1:
            return 33;  // 黄.
        case HL_KEYWORD2:
            return 32;  // 緑.
        case HL_STRING:
            return 31;  // 赤.
        case HL_NUMBER:
            return 35;  // マゼンタ.
        case HL_MATCH:
            return 34;  // 青 (カスタマイズ可).
        default:
            return 37;  // 白.
    }
}
```

## トラブルシューティング

### ハイライトが適用されない
- ファイル拡張子が正しいか確認.
- HLDB に言語が登録されているか確認.
- `editorSelectSyntaxHighlight()` が呼ばれているか確認.

### 誤ったハイライト
- キーワードの順序を確認 (より長いキーワードを先に定義).
- コメント/文字列の開始・終了文字が正しいか確認.
- マルチラインコメントの状態が保存されているか確認.

### パフォーマンスの問題
- 大きなファイルの場合, 全行の再ハイライトを避ける.
- 変更行の周辺行のみ再ハイライト.

## 参考資料

- HLDB (Highlight Database) の詳細: Kilo ソースコード参照.
- C の正規表現: POSIX regex 関数 (regex.h).
