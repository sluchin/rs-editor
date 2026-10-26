;;; org.scm --- Org mode の Scheme 版 -*- scheme -*-

;; Emacs の Org mode (org 9.7.11) を, rs-editor の Scheme で書き直したもの.
;;
;; 使い方: 初期設定ファイル (~/.rseditor.d/init.scm) に (require 'org) と書く.
;; .org のファイルを開くと, Org mode になる (find-file-hook). M-x org-mode でも, Org mode にする.
;; 設定は, (set! org-directory "~/org") のように変える (変数の名前と既定の値は, Emacs と同じ).
;;
;; できること (キーは Emacs と同じ):
;; - 見出しの開け閉め: TAB (見出しごと), S-TAB (全て). #+STARTUP: overview なども読む.
;; - 見出しを作る, 動かす: M-RET, C-RET, M-S-RET, C-S-RET, M-左右 (深さ), M-S-左右 (木ごと),
;;   M-上下 (木を動かす), C-c * (見出しにする, 戻す).
;; - 動く: C-c C-n, C-c C-p (次と前の見出し), C-c C-f, C-c C-b (同じ深さ), C-c C-u (上の見出し).
;; - TODO: C-c C-t, S-左右 (状態), S-上下 (優先度), C-c , (優先度). 済みにすると CLOSED を書く
;;   (org-log-done). 繰り返し (+1w など) の予定は, 日付を進め, 始めの状態に戻す.
;; - 日付: C-c . (日付), C-c ! (日付. 予定にしない), C-c C-s (SCHEDULED), C-c C-d (DEADLINE),
;;   S-上下左右 (カーソルの所の, 年, 月, 日, 時, 分を変える).
;; - タグ: C-c C-q (見出しの右に揃える). C-c C-c も, 見出しではタグ.
;; - リスト: M-RET (項目を足す), M-S-RET (チェックボックスの項目を足す), M-左右上下,
;;   C-c C-c (チェックボックス), C-c - (リストにする, 戻す. 印を変える), [2/3] [66%] (数える).
;; - 表: TAB, S-TAB, RET (欄を動き, 揃える), C-c C-c (揃える), C-c - (横線), C-c | (表を作る),
;;   M-左右上下 (列と行を動かす), M-S-左右上下 (列と行を消す, 足す).
;; - リンク: C-c C-l (入れる), C-c C-o (開く), C-c l (覚える. org-store-link).
;;   http は browse-url で, file: はエディタで, *見出し と #id と <<目当て>> は, そのファイルの中で開く.
;; - アジェンダ (C-c a): 週の予定 (a), TODO の一覧 (t), タグ (m), 言葉 (s).
;;   *Org Agenda* で, RET (開く), SPC (ほかの窓に出す), n, p, f, b, ., r, g, t, q.
;; - キャプチャ (C-c c): テンプレートで書き, C-c C-c で, ファイルに足して保存する (C-c C-k で止める).
;;   %? %t %T %u %U %a %i %f %F %^{問い} %^{問い|既定} %[ファイル] %(式) が使える.
;; - バッファ: M-x org-switchb (Org のバッファに移る).
;;
;; Emacs と違う所:
;; - 直前のコマンド (last-command) が無いので, TAB と S-TAB を続けて押したかは, カーソルが
;;   動いていないかで決める.
;; - 文字を隠さない (リンクの [[ ]], 強調の * /), 見出しの色は 8 段まで, 表の計算 (#+TBLFM) は無い.
;; - アジェンダは, 保存したファイル (書き換えたバッファは, そのバッファ) から読む. 時刻の表 (time grid) は無い.
;; - 書き出し (export), 時間の記録 (clock), 添付, リファイル, babel は無い.

;;; 設定 (初期設定ファイルで, (set! org-directory "~/notes") のように変える)

;; Org のファイルを置くディレクトリ
(define org-directory "~/org")
;; キャプチャの既定のファイル
(define org-default-notes-file "~/.notes")
;; アジェンダに出すファイル (ディレクトリなら, その中の .org のファイル)
(define org-agenda-files '())
;; TODO の状態 ((sequence "TODO" "|" "DONE") のように書く. | の後が済み. | が無ければ, 最後が済み.
;; "TODO(t)" のように, 括弧の中に, 選ぶキーを書ける)
(define org-todo-keywords '((sequence "TODO" "DONE")))
;; 済みにした時刻を書く (#f なら書かない. 'time か 'note なら, CLOSED: [日時] を書く)
(define org-log-done #f)
;; 繰り返しの予定を済みにしたとき, 時刻を書く (#f なら書かない)
(define org-log-repeat 'time)
;; タグを揃える桁 (負なら, タグの終わりを, その桁 (の絶対値) に揃える)
(define org-tags-column -77)
;; 優先度 (一番高いもの, 一番低いもの, 無いときの既定)
(define org-priority-highest "A")
(define org-priority-lowest "C")
(define org-priority-default "B")
;; キャプチャのテンプレート ((キー 説明 種類 (行き先) テンプレート) のリスト. 空なら, 既定のもの)
;; 種類: entry (見出し), item (リストの項目), checkitem (チェックボックス), plain (文)
;; 行き先: (file "ファイル"), (file+headline "ファイル" "見出し"). "" は org-default-notes-file
(define org-capture-templates '())
;; 前の見出しを重ねて見せない (#t なら, 見出しの * を, 最後の 1 つだけ見せる)
(define org-hide-leading-stars #f)
;; RET でリンクを開く
(define org-return-follows-link #f)
;; 締め切りの何日前から, 今日のアジェンダに出すか
(define org-deadline-warning-days 14)
;; アジェンダの日数
(define org-agenda-span 7)
;; アジェンダの始めの曜日 (1 が月曜. #f なら今日から)
(define org-agenda-start-on-weekday 1)
;; Org mode にしたとき, 呼ぶ手続き
(define org-mode-hook '())
;; キャプチャを終えたとき, 呼ぶ手続き
(define org-capture-after-finalize-hook '())

;;; 文字列とリスト

;; リストの始めの n 個
(define (org-take items n)
  (if (or (<= n 0) (null? items)) '() (cons (car items) (org-take (cdr items) (- n 1)))))

;; リストの始めの n 個を除いたもの
(define (org-drop items n)
  (if (or (<= n 0) (null? items)) items (org-drop (cdr items) (- n 1))))

;; リストの最後
(define (org-last items) (list-ref items (- (length items) 1)))

;; リストの中の位置 (0 から. 無ければ #f)
(define (org-index-of x items)
  (let loop ((rest items) (i 0))
    (cond ((null? rest) #f)
          ((equal? (car rest) x) i)
          (else (loop (cdr rest) (+ i 1))))))

;; 重なりを除く (始めのものを残す)
(define (org-unique items)
  (let loop ((rest items) (seen '()))
    (cond ((null? rest) (reverse seen))
          ((member (car rest) seen) (loop (cdr rest) seen))
          (else (loop (cdr rest) (cons (car rest) seen))))))

;; 0 から n - 1 までの数
(define (org-range n)
  (let loop ((i (- n 1)) (result '()))
    (if (< i 0) result (loop (- i 1) (cons i result)))))

;; 文字列を n 回つなげる
(define (org-repeat s n)
  (if (<= n 0) "" (string-append s (org-repeat s (- n 1)))))

;; 空白 n 個
(define (org-spaces n) (org-repeat " " n))

;; 右を空白で埋めて, 幅を width にする (長ければ, そのまま)
(define (org-pad-right s width)
  (string-append s (org-spaces (- width (string-width s)))))

;; 左を空白で埋めて, 幅を width にする
(define (org-pad-left s width)
  (string-append (org-spaces (- width (string-width s))) s))

;; 2 桁の数 (前を 0 で埋める)
(define (org-2d n)
  (if (< n 10) (format "0%d" n) (format "%d" n)))

;; 空白だけの文字列なら #t
(define (org-blank? s) (if (regexp-match "^[ \\t]*$" s) #t #f))

;; 行の始めの空白の幅
(define (org-indentation line)
  (string-length (car (regexp-match "^[ \\t]*" line))))

;; 文字列の中の, 正規表現に合う所を全て返す (それぞれ, 括弧ごとの (始め 終わり) か #f のリスト)
(define (org-all-matches regexp s)
  (let loop ((start 0) (found '()))
    (let ((m (and (<= start (string-length s)) (regexp-match-positions regexp s start))))
      (if (not m)
          (reverse found)
          (let ((from (car (car m))) (to (cadr (car m))))
            (loop (if (= to from) (+ to 1) to) (cons m found)))))))

;; 合った所の, n 番目の括弧の文字列 (無ければ #f)
(define (org-match-string s m n)
  (let ((p (list-ref m n)))
    (and p (substring s (car p) (cadr p)))))

;; 文字列の中の from を, 全て to にする
(define (org-replace-all s from to)
  (string-join (split-string s from) to))

;; 文字列の n 文字目 (0 から)
(define (org-char s n) (substring s n (+ n 1)))

;;; バッファの行

;; バッファの行 (改行で分けたもの. 最後が改行なら, 最後は空の文字列)
(define (org-lines) (split-string (buffer-string) "\n"))

;; 行の数 (最後の改行の後の, 空の行は数えない)
(define (org-line-count lines)
  (let ((n (length lines)))
    (if (and (> n 1) (equal? (org-last lines) "")) (- n 1) n)))

;; n 行目 (1 から)
(define (org-line lines n) (list-ref lines (- n 1)))

;; n 行目の始めの位置
(define (org-line-start lines n)
  (let loop ((i 1) (rest lines) (pos 1))
    (if (or (>= i n) (null? rest))
        pos
        (loop (+ i 1) (cdr rest) (+ pos (string-length (car rest)) 1)))))

;; n 行目の終わりの位置 (改行の前)
(define (org-line-end lines n)
  (+ (org-line-start lines n) (string-length (org-line lines n))))

;; 位置の行と桁 ((行 桁). 桁は 0 から)
(define (org-pos-line lines pos)
  (let loop ((n 1) (rest lines) (start 1))
    (let ((end (+ start (string-length (car rest)))))
      (if (or (<= pos end) (null? (cdr rest)))
          (list n (- pos start))
          (loop (+ n 1) (cdr rest) (+ end 1))))))

;; カーソルの行
(define (org-current-line lines) (car (org-pos-line lines (point))))

;; カーソルの桁 (0 から)
(define (org-current-column lines) (cadr (org-pos-line lines (point))))

;; n 行目の桁 col へ動く
(define (org-goto-line-column lines n col)
  (goto-char (+ (org-line-start lines n) (min col (string-length (org-line lines n))))))

;; 始めと終わりの間を, 文字列に置き換える. カーソルは, 置き換えた所の後なら, 一緒に動く
(define (org-replace-region start end new)
  (let ((p (point)))
    (unless (equal? (buffer-substring start end) new)
      (delete-region start end)
      (goto-char start)
      (insert new)
      (goto-char (cond ((<= p start) p)
                       ((>= p end) (+ p (- (string-length new) (- end start))))
                       (else (min p (+ start (string-length new)))))))))

;; from 行から to 行までを, 新しい行 (のリスト) に置き換える
(define (org-replace-lines lines from to new)
  (org-replace-region (org-line-start lines from) (org-line-end lines to) (string-join new "\n")))

;; n 行目を置き換える
(define (org-set-line lines n new) (org-replace-lines lines n n (list new)))

;; n 行目を置き換える. 変わった所だけを置き換える (カーソルは, 変わらない所と一緒に動く)
(define (org-set-line-changed lines n new)
  (let* ((line (org-line lines n))
         (len (min (string-length line) (string-length new)))
         (same (lambda (i j) (equal? (substring line i (+ i 1)) (substring new j (+ j 1)))))
         (prefix (let loop ((i 0)) (if (and (< i len) (same i i)) (loop (+ i 1)) i)))
         (suffix (let loop ((i 0))
                   (if (and (< i (- len prefix))
                            (same (- (string-length line) i 1) (- (string-length new) i 1)))
                       (loop (+ i 1))
                       i)))
         (start (org-line-start lines n)))
    (org-replace-region (+ start prefix) (+ start (- (string-length line) suffix))
                        (substring new prefix (- (string-length new) suffix)))))

;; n 行目の後に, 行を入れる
(define (org-insert-lines-after lines n new)
  (let ((end (org-line-end lines n)))
    (org-replace-region end end (string-append "\n" (string-join new "\n")))))

;; from 行から to 行までを消す (改行も)
(define (org-delete-lines lines from to)
  (let ((start (org-line-start lines from))
        (count (length lines)))
    (cond ((< to count) (org-replace-region start (org-line-start lines (+ to 1)) ""))
          ((> from 1) (org-replace-region (- start 1) (org-line-end lines to) ""))
          (else (org-replace-region start (org-line-end lines to) "")))))

;;; 見出し

;; 見出しの深さ (見出しでなければ #f)
(define (org-level line)
  (let ((m (regexp-match "^(\\*+)(?: |$)" line)))
    (and m (string-length (cadr m)))))

;; 見出しの行と深さ ((行 深さ) のリスト)
(define (org-headings lines)
  (let loop ((rest lines) (n 1) (found '()))
    (if (null? rest)
        (reverse found)
        (let ((level (org-level (car rest))))
          (loop (cdr rest) (+ n 1) (if level (cons (list n level) found) found))))))

;; n 行目か, その前の, 一番近い見出しの行 (無ければ #f)
(define (org-heading-line lines n)
  (let loop ((headings (org-headings lines)) (found #f))
    (cond ((or (null? headings) (> (car (car headings)) n)) found)
          (else (loop (cdr headings) (car (car headings)))))))

;; n 行目の見出しの木の, 最後の行
(define (org-subtree-end lines n)
  (let ((level (org-level (org-line lines n))))
    (let loop ((headings (org-headings lines)))
      (cond ((null? headings) (org-line-count lines))
            ((and (> (car (car headings)) n) (<= (cadr (car headings)) level))
             (- (car (car headings)) 1))
            (else (loop (cdr headings)))))))

;; n 行目の見出しの, 本文の最後の行 (次の見出しの前)
(define (org-entry-end lines n)
  (let loop ((headings (org-headings lines)))
    (cond ((null? headings) (org-line-count lines))
          ((> (car (car headings)) n) (- (car (car headings)) 1))
          (else (loop (cdr headings))))))

;; カーソルの見出しの行 (無ければ, 失敗にする)
(define (org-back-to-heading lines)
  (or (org-heading-line lines (org-current-line lines))
      (error "Before first headline at position %d in buffer %s" (point) (buffer-name))))

;; カーソルが見出しにあれば #t
(define (org-at-heading-p)
  (let ((lines (org-lines)))
    (if (org-level (org-line lines (org-current-line lines))) #t #f)))

;; TODO のキーワードの正規表現 (括弧の中)
(define (org-todo-regexp)
  (let ((words (org-todo-keywords-1)))
    (if (null? words) "[^\\s\\S]" (string-join (map regexp-quote words) "|"))))

;; 見出しを分ける ((星 TODO 優先度 題 タグ). 無いものは #f. 見出しでなければ #f)
(define (org-parse-heading line)
  (let ((m (regexp-match
            (string-append "^(\\*+)(?: +(" (org-todo-regexp) "))?(?: +(\\[#.\\]))?"
                           "(?: +(.*?))??(?:[ \\t]+(:[\\w@#%:]+:))?[ \\t]*$")
            line)))
    (and m (org-level line) (cdr m))))

;; 見出しの行を作る (タグは揃える)
(define (org-make-heading stars todo priority title tags)
  (let ((head (string-join (filter (lambda (x) (and x (not (equal? x ""))))
                                   (list stars todo priority title))
                           " ")))
    (if tags (org-tags-line head tags) head)))

;; タグを揃えた行 (org-tags-column)
(define (org-tags-line head tags)
  (let* ((width (string-width head))
         (column (if (< org-tags-column 0)
                     (- (- org-tags-column) (string-width tags))
                     org-tags-column)))
    (string-append head (org-spaces (max 1 (- column width))) tags)))

;; 見出しのタグ (リスト)
(define (org-heading-tags parsed)
  (let ((tags (list-ref parsed 4)))
    (if tags (filter (lambda (t) (not (equal? t ""))) (split-string tags ":")) '())))

;; タグのリストを, :a:b: にする (空なら #f)
(define (org-tags-string tags)
  (if (null? tags) #f (string-append ":" (string-join tags ":") ":")))

;;; TODO のキーワード

;; キーワードの名前 ("TODO(t)" なら "TODO")
(define (org-keyword-name word) (car (split-string word "(")))

;; キーワードを選ぶキー ("TODO(t)" なら "t". 無ければ #f)
(define (org-keyword-key word)
  (let ((m (regexp-match "\\(([^@!/)])" word)))
    (and m (cadr m))))

;; TODO の組 (((未完の言葉) (済みの言葉)) のリスト)
(define (org-todo-sets)
  (map (lambda (set)
         (let* ((words (if (string? (car set)) set (cdr set)))
                (bar (org-index-of "|" words)))
           (if bar
               (list (map org-keyword-name (org-take words bar))
                     (map org-keyword-name (org-drop words (+ bar 1))))
               (list (map org-keyword-name (org-take words (- (length words) 1)))
                     (list (org-keyword-name (org-last words)))))))
       (if (and (pair? org-todo-keywords) (string? (car org-todo-keywords)))
           (list org-todo-keywords)
           org-todo-keywords)))

;; 全ての TODO のキーワード
(define (org-todo-keywords-1)
  (org-unique (apply append (map (lambda (set) (append (car set) (cadr set))) (org-todo-sets)))))

;; 済みのキーワード
(define (org-done-keywords)
  (apply append (map cadr (org-todo-sets))))

;; キーワードが済みなら #t
(define (org-done-keyword? word) (if (and word (member word (org-done-keywords))) #t #f))

;; キーワードの組 (無ければ, 始めの組)
(define (org-todo-set word)
  (let loop ((sets (org-todo-sets)))
    (cond ((null? sets) (car (org-todo-sets)))
          ((member word (append (car (car sets)) (cadr (car sets)))) (car sets))
          (else (loop (cdr sets))))))

;; キーとキーワード ((キー キーワード) のリスト)
(define (org-todo-keys)
  (apply append
         (map (lambda (set)
                (apply append
                       (map (lambda (word)
                              (let ((key (org-keyword-key word)))
                                (if key (list (list key (org-keyword-name word))) '())))
                            (filter string? (if (string? (car set)) set (cdr set))))))
              (if (and (pair? org-todo-keywords) (string? (car org-todo-keywords)))
                  (list org-todo-keywords)
                  org-todo-keywords))))

;;; 色 (face)

;; 色付けの正規表現と face (画面のエディタの, JavaScript の正規表現. 行ごとに合わせる.
;; (正規表現 face 括弧の番号) のリスト. 後のものを上に重ねる)
(define (org-font-lock-keywords)
  (let ((todo (string-join (map regexp-quote (filter (lambda (w) (not (org-done-keyword? w)))
                                                     (org-todo-keywords-1)))
                           "|"))
        (done (string-join (map regexp-quote (org-done-keywords)) "|")))
    (append
     ;; 見出し (8 段. 9 段目からは, 1 段目の色に戻る)
     (map (lambda (n)
            (list (format "^(?:\\*{8})*\\*{%d}(?: .*)?$" (+ n 1)) (format "org-level-%d" (+ n 1)) 0))
          (org-range 8))
     (if org-hide-leading-stars '(("^(\\*+)\\*(?: |$)" "org-hide" 1)) '())
     (if (equal? done "") '() (list (list (format "^\\*+ +(?:%s) +(.*)$" done) "org-headline-done" 1)))
     (if (equal? todo "") '() (list (list (format "^\\*+ +(%s)(?= |$)" todo) "org-todo" 1)))
     (if (equal? done "") '() (list (list (format "^\\*+ +(%s)(?= |$)" done) "org-done" 1)))
     '(("^\\*+ +(?:[A-Z]+ +)?(\\[#[A-Z0-9]\\])" "org-priority" 1)
       ("^\\*+ .*?[ \\t](:[\\w@#%:]+:)[ \\t]*$" "org-tag" 1)
       ("^[ \\t]*#\\+.*" "org-meta-line" 0)
       ("^[ \\t]*#\\+(?:TITLE|title): +(.*)$" "org-document-title" 1)
       ("^[ \\t]*# .*" "org-comment" 0)
       ("^[ \\t]*(:[\\w-]+:)" "org-special-keyword" 1)
       ("^[ \\t]*(:(?:PROPERTIES|END|LOGBOOK):)[ \\t]*$" "org-drawer" 1)
       ("^[ \\t]*\\|.*" "org-table" 0)
       ("^[ \\t]*(?:[-+]|[0-9]+[.)]|(?<=[ \\t])\\*) +(\\[[ X-]\\])" "org-checkbox" 1)
       ("\\[[0-9]*%\\]|\\[[0-9]*/[0-9]*\\]" "org-checkbox-statistics" 0)
       ("(?:^|[ \\t])(SCHEDULED:|DEADLINE:|CLOSED:)" "org-special-keyword" 1)
       ("<[0-9]{4}-[0-9]{2}-[0-9]{2}[^>\\n]*>" "org-date" 0)
       ("\\[[0-9]{4}-[0-9]{2}-[0-9]{2}[^\\]\\n]*\\]" "org-date" 0)
       ("<<[^<>\\n]+>>" "org-target" 0)
       ("(?:^|[\\s('\"{])(\\*[^\\s*](?:[^*\\n]*[^\\s*])?\\*)(?=[\\s\\-.,:!?;'\")}\\[]|$)" "bold" 1)
       ("(?:^|[\\s('\"{])(/[^\\s/](?:[^/\\n]*[^\\s/])?/)(?=[\\s\\-.,:!?;'\")}\\[]|$)" "italic" 1)
       ("(?:^|[\\s('\"{])(_[^\\s_](?:[^_\\n]*[^\\s_])?_)(?=[\\s\\-.,:!?;'\")}\\[]|$)" "underline" 1)
       ("(?:^|[\\s('\"{])(\\+[^\\s+](?:[^+\\n]*[^\\s+])?\\+)(?=[\\s\\-.,:!?;'\")}\\[]|$)" "org-strike" 1)
       ("(?:^|[\\s('\"{])(=[^\\s=](?:[^=\\n]*[^\\s=])?=)(?=[\\s\\-.,:!?;'\")}\\[]|$)" "org-verbatim" 1)
       ("(?:^|[\\s('\"{])(~[^\\s~](?:[^~\\n]*[^\\s~])?~)(?=[\\s\\-.,:!?;'\")}\\[]|$)" "org-code" 1)
       ("\\[\\[[^\\]\\n]+\\](?:\\[[^\\]\\n]+\\])?\\]" "org-link" 0)
       ("(?:https?|ftp|mailto|file):[^\\s\\]>)\"']+" "org-link" 0)))))

;; 色を付ける
(define (org-fontify)
  (unhighlight-regexp #t)
  (for-each (lambda (k) (highlight-regexp (car k) (cadr k) (caddr k))) (org-font-lock-keywords)))

;;; Org mode

;; Org mode のキー
(define org-mode-keys
  '(("TAB" org-cycle)
    ("S-TAB" org-shifttab)
    ("RET" org-return)
    ("M-RET" org-meta-return)
    ("M-S-RET" org-insert-todo-heading)
    ("C-RET" org-insert-heading-respect-content)
    ("C-S-RET" org-insert-todo-heading-respect-content)
    ("M-<left>" org-metaleft)
    ("M-<right>" org-metaright)
    ("M-<up>" org-metaup)
    ("M-<down>" org-metadown)
    ("M-S-<left>" org-shiftmetaleft)
    ("M-S-<right>" org-shiftmetaright)
    ("M-S-<up>" org-shiftmetaup)
    ("M-S-<down>" org-shiftmetadown)
    ("S-<left>" org-shiftleft)
    ("S-<right>" org-shiftright)
    ("S-<up>" org-shiftup)
    ("S-<down>" org-shiftdown)
    ("C-c C-n" org-next-visible-heading)
    ("C-c C-p" org-previous-visible-heading)
    ("C-c C-f" org-forward-heading-same-level)
    ("C-c C-b" org-backward-heading-same-level)
    ("C-c C-u" outline-up-heading)
    ("C-c C-t" org-todo)
    ("C-c ," org-priority)
    ("C-c C-q" org-set-tags-command)
    ("C-c ." org-time-stamp)
    ("C-c !" org-time-stamp-inactive)
    ("C-c C-s" org-schedule)
    ("C-c C-d" org-deadline)
    ("C-c C-c" org-ctrl-c-ctrl-c)
    ("C-c -" org-ctrl-c-minus)
    ("C-c *" org-ctrl-c-star)
    ("C-c |" org-table-create-or-convert-from-region)
    ("C-c ^" org-sort)
    ("C-c #" org-update-statistics-cookies)
    ("C-c C-x C-b" org-toggle-checkbox)
    ("C-c C-l" org-insert-link)
    ("C-c C-o" org-open-at-point)))

;; Org mode にする (キー, 色, #+STARTUP)
(define-command (org-mode)
  (for-each (lambda (key) (local-set-key (car key) (cadr key))) org-mode-keys)
  (org-fontify)
  (org-set-startup-visibility)
  (run-hooks 'org-mode-hook))

;; ファイルを開いたとき (find-file-hook): .org なら Org mode にする
(define (org-find-file-hook)
  (let ((file (buffer-file-name)))
    (when (and file (regexp-match "\\.org$" file))
      (org-mode))))

;; #+STARTUP: の見せ方 (overview, content, showall)
(define (org-set-startup-visibility)
  (let ((m (regexp-match "(?mi)^#\\+STARTUP:.*\\b(overview|fold|content|showall|showeverything)\\b"
                         (buffer-string))))
    (when m
      (let ((how (string-downcase (cadr m))))
        (cond ((member how '("overview" "fold")) (org-cycle-overview (org-lines)))
              ((equal? how "content") (org-cycle-content (org-lines))))))))

;;; 開け閉め

;; 直前の開け閉め ((バッファ カーソル 状態). 続けて押したかを, カーソルが動いていないかで決める)
(define org-cycle-last #f)

;; 続けて押していて, 前の状態が status なら #t
(define (org-cycle-repeated? status)
  (and org-cycle-last
       (equal? (org-take org-cycle-last 2) (list (buffer-name) (point)))
       (eq? (caddr org-cycle-last) status)))

;; 状態を覚える
(define (org-cycle-remember status)
  (set! org-cycle-last (list (buffer-name) (point) status)))

;; 隠す範囲の終わり (to 行の終わり. 最後の行なら, 最後の改行の前)
(define (org-fold-end lines to) (org-line-end lines to))

;; 見せる見出し (行のリスト) の, それぞれの本文を, 次の見せる見出しの前まで隠す (last 行まで)
(define (org-fold-showing lines shown last)
  (let loop ((rest shown))
    (unless (null? rest)
      (let* ((n (car rest))
             (to (if (null? (cdr rest)) last (- (cadr rest) 1)))
             (from (org-line-end lines n))
             (end (org-fold-end lines to)))
        (when (> end from) (outline-flag-region from end #t))
        (loop (cdr rest))))))

;; 見出しの木の開け閉め (EMPTY ENTRY, CHILDREN, SUBTREE, FOLDED の順)
(define (org-cycle-local lines n)
  (let* ((level (org-level (org-line lines n)))
         (last (org-subtree-end lines n))
         (eoh (org-line-end lines n))
         (eos (org-fold-end lines last))
         (children (filter (lambda (h) (and (> (car h) n) (<= (car h) last))) (org-headings lines)))
         (folded (or (outline-invisible-p eoh)
                     (org-blank? (buffer-substring eoh eos)))))
    (cond
     ((<= eos eoh)
      (message "EMPTY ENTRY")
      (org-cycle-remember #f))
     ((and folded (not (null? children)))
      ;; 本文と, 子の見出しを見せる (子の本文は隠す)
      (let* ((depth (cadr (car children)))
             (shown (map car (filter (lambda (h) (<= (cadr h) depth)) children))))
        (outline-flag-region eoh eos #f)
        (org-fold-showing lines shown last))
      (message "CHILDREN")
      (org-cycle-remember 'children))
     ((or folded (org-cycle-repeated? 'children))
      (outline-flag-region eoh eos #f)
      (message (if folded "SUBTREE (NO CHILDREN)" "SUBTREE"))
      (org-cycle-remember 'subtree))
     (else
      (outline-flag-region eoh eos #t)
      (message "FOLDED")
      (org-cycle-remember 'folded)))))

;; 一番上の見出しだけを見せる
(define (org-cycle-overview lines)
  (let* ((headings (org-headings lines))
         (top (if (null? headings) 1 (apply min (map cadr headings)))))
    (outline-flag-region 1 (point-max) #f)
    (org-fold-showing lines (map car (filter (lambda (h) (<= (cadr h) top)) headings))
                      (org-line-count lines))))

;; 全ての見出しを見せる (本文は隠す)
(define (org-cycle-content lines)
  (outline-flag-region 1 (point-max) #f)
  (org-fold-showing lines (map car (org-headings lines)) (org-line-count lines)))

;; 全体の開け閉め (OVERVIEW, CONTENTS, SHOW ALL の順)
(define-command (org-global-cycle)
  (let ((lines (org-lines)))
    (cond
     ((org-cycle-repeated? 'overview)
      (org-cycle-content lines)
      (message "CONTENTS...done")
      (org-cycle-remember 'contents))
     ((org-cycle-repeated? 'contents)
      (outline-flag-region 1 (point-max) #f)
      (message "SHOW ALL")
      (org-cycle-remember 'all))
     (else
      (org-cycle-overview lines)
      (message "OVERVIEW")
      (org-cycle-remember 'overview)))))

;; TAB: 表なら次の欄, 見出しなら開け閉め, ほかは字下げ
(define-command (org-cycle)
  (let* ((lines (org-lines))
         (n (org-current-line lines)))
    (cond ((org-table-line? (org-line lines n)) (org-table-next-field))
          ((org-level (org-line lines n)) (org-cycle-local lines n))
          (else (call-interactively 'indent-for-tab-command)))))

;; S-TAB: 表なら前の欄, ほかは全体の開け閉め
(define-command (org-shifttab)
  (let ((lines (org-lines)))
    (if (org-table-line? (org-line lines (org-current-line lines)))
        (org-table-previous-field)
        (org-global-cycle))))

;; 位置が隠れていれば, 見せる
(define (org-reveal pos)
  (when (outline-invisible-p pos)
    (outline-flag-region (max 1 (- pos 1)) (+ pos 1) #f)))

;;; 見出しを作る

;; 見出しの題の, 行の中の範囲 ((始め 終わり). 題が無ければ #f)
(define (org-heading-title-range line)
  (let ((m (regexp-match-positions
            (string-append "^(\\*+)(?: +(" (org-todo-regexp) "))?(?: +(\\[#.\\]))?"
                           "(?: +(.*?))??(?:[ \\t]+(:[\\w@#%:]+:))?[ \\t]*$")
            line)))
    (and m (list-ref m 4))))

;; 新しい見出しの TODO (todo が #t なら, 前の見出しの, 済みでない状態. 無ければ, 始めの状態)
(define (org-new-heading-todo lines n todo)
  (and todo
       (let* ((h (org-heading-line lines n))
              (current (and h (cadr (org-parse-heading (org-line lines h))))))
         (if (and current (not (org-done-keyword? current)))
             current
             (car (org-todo-keywords-1))))))

;; 見出しを入れる (M-RET). todo が #t なら, TODO の見出し
(define (org-insert-heading-1 todo)
  (let* ((lines (org-lines))
         (n (org-current-line lines))
         (col (org-current-column lines))
         (line (org-line lines n))
         (start (org-line-start lines n))
         (h (org-heading-line lines n))
         (keyword (org-new-heading-todo lines n todo))
         (prefix (string-append (org-repeat "*" (if h (org-level (org-line lines h)) 1)) " "
                                (if keyword (string-append keyword " ") ""))))
    (cond
     ((org-level line)
      (let ((title (org-heading-title-range line)))
        (cond
         ;; 行の始め: 前に入れる
         ((= col 0)
          (org-replace-lines lines n n (list prefix line))
          (goto-char (+ start (string-length prefix))))
         ;; 題の中: 分ける (カーソルの後の題を, 新しい見出しにする)
         ((and title (>= col (car title)) (<= col (cadr title)))
          (let* ((split (string-trim (substring line col (cadr title))))
                 (rest (substring line (cadr title)))
                 (head (string-append (substring line 0 col)
                                      (if (org-blank? rest) "" rest)))
                 (parsed (org-parse-heading head))
                 (kept (if (and parsed (list-ref parsed 4))
                           (apply org-make-heading parsed)
                           head)))
            (org-replace-lines lines n n (list kept (string-append prefix split)))
            (goto-char (+ start (string-length kept) 1 (string-length prefix) (string-length split)))))
         (else
          (org-replace-lines lines n n (list line prefix))
          (goto-char (+ start (string-length line) 1 (string-length prefix)))))))
     ;; 文の行の始め: 見出しにする
     ((= col 0)
      (org-set-line lines n (string-append prefix line))
      (goto-char (+ start (string-length prefix))))
     ;; 文の途中: 分ける
     (else
      (org-replace-lines lines n n (list (substring line 0 col) (string-append prefix (substring line col))))
      (goto-char (+ start col 1 (string-length prefix)))))))

;; 木の後に見出しを入れる (C-RET)
(define (org-insert-heading-after-subtree todo)
  (let* ((lines (org-lines))
         (n (org-current-line lines))
         (h (org-heading-line lines n))
         (keyword (org-new-heading-todo lines n todo))
         (prefix (string-append (org-repeat "*" (if h (org-level (org-line lines h)) 1)) " "
                                (if keyword (string-append keyword " ") ""))))
    (if h
        (let ((last (org-subtree-end lines h)))
          (org-insert-lines-after lines last (list prefix))
          (goto-char (+ (org-line-end lines last) 1 (string-length prefix))))
        ;; 始めの見出しの前: 始めの見出しの前に入れる
        (let ((headings (org-headings lines)))
          (if (null? headings)
              (begin
                (goto-char (point-max))
                (unless (or (= (point) 1) (equal? (org-last lines) "")) (insert "\n"))
                (insert prefix))
              (let ((first (car (car headings))))
                (goto-char (org-line-start lines first))
                (insert prefix "\n")
                (goto-char (- (point) 1))))))))

;; 見出しを入れる (リストの項目でなくても)
(define-command (org-insert-heading) (org-insert-heading-1 #f))
(define-command (org-insert-heading-respect-content) (org-insert-heading-after-subtree #f))
(define-command (org-insert-todo-heading-respect-content) (org-insert-heading-after-subtree #t))

;; M-RET: 表なら何もしない, リストなら項目, ほかは見出し
(define-command (org-meta-return)
  (cond ((org-in-item) (org-insert-item #f))
        (else (org-insert-heading-1 #f))))

;; M-S-RET: TODO の見出し (リストなら, チェックボックスの項目)
(define-command (org-insert-todo-heading)
  (if (org-in-item) (org-insert-item #t) (org-insert-heading-1 #t)))

;;; 見出しを動かす

;; from 行から to 行までの見出しの深さを変える
(define (org-shift-headings lines from to delta)
  (when (and (< delta 0) (= (org-level (org-line lines from)) 1))
    (error "Cannot promote to level 0.  UNDO to recover if necessary"))
  (for-each (lambda (n) (when (org-level (org-line lines n)) (org-shift-heading n delta)))
            (map (lambda (i) (+ from i)) (org-range (+ (- to from) 1)))))

;; n 行目の見出しの深さを変える. 星を足すか消して, タグを揃え直す (カーソルは, 文と一緒に動く)
(define (org-shift-heading n delta)
  (let* ((lines (org-lines))
         (end (+ (org-line-start lines n) (org-level (org-line lines n)))))
    (if (> delta 0)
        (org-replace-region end end "*")
        (org-replace-region (- end 1) end ""))
    (let* ((lines (org-lines))
           (parsed (org-parse-heading (org-line lines n))))
      (when (list-ref parsed 4)
        (org-set-line-changed lines n (apply org-make-heading parsed))))))

;; 見出しを浅く, 深くする (subtree が #t なら, 木ごと)
(define (org-promote-1 delta subtree)
  (let* ((lines (org-lines))
         (n (org-back-to-heading lines)))
    (org-shift-headings lines n (if subtree (org-subtree-end lines n) n) delta)))

(define-command (org-do-promote) (org-promote-1 -1 #f))
(define-command (org-do-demote) (org-promote-1 1 #f))
(define-command (org-promote-subtree) (org-promote-1 -1 #t))
(define-command (org-demote-subtree) (org-promote-1 1 #t))

;; 同じ深さの, 前か後の見出しの行 (親を越えるなら #f)
(define (org-sibling lines n forward)
  (let ((level (org-level (org-line lines n)))
        (headings (if forward
                      (filter (lambda (h) (> (car h) n)) (org-headings lines))
                      (reverse (filter (lambda (h) (< (car h) n)) (org-headings lines))))))
    (let loop ((rest headings))
      (cond ((null? rest) #f)
            ((< (cadr (car rest)) level) #f)
            ((= (cadr (car rest)) level) (car (car rest)))
            (else (loop (cdr rest)))))))

;; 木を, 前か後の, 同じ深さの木と入れ換える
(define (org-move-subtree forward)
  (let* ((lines (org-lines))
         (n (org-back-to-heading lines))
         (sibling (or (org-sibling lines n forward)
                      (error "Cannot move past superior level or buffer limit")))
         (first (if forward n sibling))
         (second (if forward sibling n))
         (last (org-subtree-end lines second))
         (offset (- (org-current-line lines) n))
         (col (org-current-column lines))
         (block (lambda (from to) (org-take (org-drop lines (- from 1)) (+ (- to from) 1))))
         (a (block first (- second 1)))
         (b (block second last))
         (folded (outline-invisible-p (org-line-end lines n))))
    (org-replace-lines lines first last (append b a))
    (let* ((new-lines (org-lines))
           (moved (if forward (+ first (length b)) first)))
      (when folded
        (outline-flag-region (org-line-end new-lines moved)
                             (org-line-end new-lines (org-subtree-end new-lines moved)) #t))
      (org-goto-line-column new-lines (+ moved offset) col))))

(define-command (org-move-subtree-up) (org-move-subtree #f))
(define-command (org-move-subtree-down) (org-move-subtree #t))

;; 2 つの行を入れ換える (n 行目と, その前か後の行)
(define (org-drag-line forward)
  (let* ((lines (org-lines))
         (n (org-current-line lines))
         (m (if forward (+ n 1) (- n 1)))
         (col (org-current-column lines)))
    (when (or (< m 1) (> m (org-line-count lines)))
      (error (if forward "Cannot move further down" "Cannot move further up")))
    (let ((from (min n m)) (to (max n m)))
      (org-replace-lines lines from to (list (org-line lines to) (org-line lines from)))
      (org-goto-line-column (org-lines) m col))))

;; C-c *: 見出しを文に, 項目と文を見出しにする (カーソルは, 文と一緒に動く)
(define-command (org-ctrl-c-star)
  (let* ((lines (org-lines))
         (n (org-current-line lines))
         (line (org-line lines n))
         (item (org-parse-item line)))
    (cond
     ((org-table-line? line) (message "No formulas"))
     ((org-level line)
      (org-set-line-changed lines n (string-trim (substring line (org-level line)))))
     (else
      (let* ((h (org-heading-line lines n))
             (stars (org-repeat "*" (+ 1 (if h (org-level (org-line lines h)) 0)))))
        (org-set-line-changed lines n
                      (if item
                          (string-append stars " "
                                         (cond ((equal? (list-ref item 2) "[X]")
                                                (string-append (car (org-done-keywords)) " "))
                                               ((list-ref item 2)
                                                (string-append (car (org-todo-keywords-1)) " "))
                                               (else ""))
                                         (list-ref item 3))
                          (string-append stars " " (string-trim line)))))))))

;;; 動く

;; 見出しへ動く (見せる)
(define (org-goto-heading lines n)
  (goto-char (org-line-start lines n))
  (org-reveal (point)))

;; 次と前の, 見えている見出し (無ければ, バッファの終わりと始め)
(define (org-visible-heading forward)
  (let* ((lines (org-lines))
         (n (org-current-line lines))
         (candidates (filter (lambda (h)
                               (and (if forward (> (car h) n) (< (car h) n))
                                    (not (outline-invisible-p (org-line-start lines (car h))))))
                             (org-headings lines))))
    (cond ((null? candidates) (goto-char (if forward (point-max) (point-min))))
          (forward (goto-char (org-line-start lines (car (car candidates)))))
          (else (goto-char (org-line-start lines (car (org-last candidates))))))))

(define-command (org-next-visible-heading) (org-visible-heading #t))
(define-command (org-previous-visible-heading) (org-visible-heading #f))

;; 同じ深さの, 次と前の見出し (親の中だけ. 無ければ動かない)
(define (org-heading-same-level forward)
  (let* ((lines (org-lines))
         (n (org-current-line lines))
         (h (org-back-to-heading lines)))
    (if (and (not forward) (not (= h n)))
        (goto-char (org-line-start lines h))
        (let ((sibling (org-sibling lines h forward)))
          (when sibling (goto-char (org-line-start lines sibling)))))))

(define-command (org-forward-heading-same-level) (org-heading-same-level #t))
(define-command (org-backward-heading-same-level) (org-heading-same-level #f))

;; 上の見出しの行 (無ければ #f)
(define (org-parent lines n)
  (let ((level (org-level (org-line lines n))))
    (let loop ((rest (reverse (filter (lambda (h) (< (car h) n)) (org-headings lines)))))
      (cond ((null? rest) #f)
            ((< (cadr (car rest)) level) (car (car rest)))
            (else (loop (cdr rest)))))))

;; C-c C-u: 上の見出しへ
(define-command (outline-up-heading)
  (let* ((lines (org-lines))
         (parent (org-parent lines (org-back-to-heading lines))))
    (if parent
        (goto-char (org-line-start lines parent))
        (error "Already at top level of the outline"))))

;;; リスト

;; リストの項目を分ける ((字下げ 印 チェックボックス 文). チェックボックスが無ければ #f. 項目でなければ #f)
(define (org-parse-item line)
  (let ((m (regexp-match "^([ \\t]*)([-+*]|[0-9]+[.)])(?:[ \\t]+|$)(?:(\\[[ X-]\\])(?:[ \\t]+|$))?(.*)$" line)))
    (and m
         (not (and (equal? (cadr m) "") (equal? (caddr m) "*")))
         (cdr m))))

;; 項目の行を作る
(define (org-make-item indent bullet box text)
  (string-append indent bullet " " (if box (string-append box " ") "") text))

;; 印が数なら #t
(define (org-ordered-bullet? bullet) (if (regexp-match "^[0-9]" bullet) #t #f))

;; n 行目の項目の, 最後の行 (子の項目と, 続きの行を含む. 後ろの空の行は含まない)
(define (org-item-end lines n)
  (let ((indent (org-indentation (org-line lines n)))
        (count (org-line-count lines)))
    (let loop ((k (+ n 1)) (last n) (blanks 0))
      (if (> k count)
          last
          (let ((line (org-line lines k)))
            (cond ((org-blank? line)
                   (if (>= blanks 1) last (loop (+ k 1) last (+ blanks 1))))
                  ((or (org-level line) (<= (org-indentation line) indent)) last)
                  (else (loop (+ k 1) k 0))))))))

;; n 行目を含む項目の行 (無ければ #f)
(define (org-item-at lines n)
  (let loop ((k n))
    (if (< k 1)
        #f
        (let ((line (org-line lines k)))
          (cond ((org-level line) #f)
                ((and (org-parse-item line) (>= (org-item-end lines k) n)) k)
                ((and (< k n) (not (org-blank? line)) (= (org-indentation line) 0)
                      (not (org-parse-item line)))
                 #f)
                (else (loop (- k 1))))))))

;; カーソルの項目の行 (無ければ #f)
(define (org-in-item)
  (let ((lines (org-lines)))
    (org-item-at lines (org-current-line lines))))

;; 同じ深さの, 前か後の項目 (無ければ #f)
(define (org-item-sibling lines n forward)
  (let ((indent (org-indentation (org-line lines n))))
    (if forward
        (let loop ((k (+ (org-item-end lines n) 1)))
          (cond ((> k (org-line-count lines)) #f)
                ((org-blank? (org-line lines k)) (loop (+ k 1)))
                ((and (org-parse-item (org-line lines k))
                      (not (org-level (org-line lines k)))
                      (= (org-indentation (org-line lines k)) indent))
                 k)
                (else #f)))
        (let loop ((k (- n 1)))
          (if (< k 1)
              #f
              (let ((line (org-line lines k)))
                (cond ((org-level line) #f)
                      ((org-blank? line) (loop (- k 1)))
                      ((and (org-parse-item line) (= (org-indentation line) indent))
                       (and (>= (org-item-end lines k) (- n 1)) k))
                      ((> (org-indentation line) indent) (loop (- k 1)))
                      (else #f))))))))

;; リストの, 同じ深さの項目の行 (始めから)
(define (org-item-siblings lines n)
  (let ((first (let loop ((k n))
                 (let ((previous (org-item-sibling lines k #f)))
                   (if previous (loop previous) k)))))
    (let loop ((k first) (found '()))
      (if k (loop (org-item-sibling lines k #t) (cons k found)) (reverse found)))))

;; 項目の上の項目 (無ければ #f)
(define (org-item-parent lines n)
  (let ((indent (org-indentation (org-line lines n))))
    (let loop ((k (- n 1)))
      (if (< k 1)
          #f
          (let ((line (org-line lines k)))
            (cond ((org-level line) #f)
                  ((and (org-parse-item line) (< (org-indentation line) indent)
                        (>= (org-item-end lines k) n))
                   k)
                  (else (loop (- k 1)))))))))

;; 項目の子の項目 (すぐ下の深さのもの)
(define (org-item-children lines n)
  (let* ((end (org-item-end lines n))
         (items (filter (lambda (k) (org-parse-item (org-line lines k)))
                        (map (lambda (i) (+ n 1 i)) (org-range (- end n)))))
         (indent (if (null? items) 0 (apply min (map (lambda (k) (org-indentation (org-line lines k))) items)))))
    (filter (lambda (k) (= (org-indentation (org-line lines k)) indent)) items)))

;; リストの印を揃え, 数を振り直す (始めの項目の印に合わせる)
(define (org-fix-bullets lines n)
  (let* ((siblings (org-item-siblings lines n))
         (first (org-parse-item (org-line lines (car siblings))))
         (bullet (cadr first))
         (delimiter (substring bullet (- (string-length bullet) 1))))
    (for-each
     (lambda (k i)
       (let* ((lines (org-lines))
              (item (org-parse-item (org-line lines k)))
              (new (if (org-ordered-bullet? bullet) (format "%d%s" (+ i 1) delimiter) bullet)))
         (unless (equal? new (cadr item))
           (org-set-line lines k (org-make-item (car item) new (caddr item) (list-ref item 3))))))
     siblings (org-range (length siblings)))))

;; 項目を入れる (checkbox が #t なら, チェックボックスを付ける)
(define (org-insert-item checkbox)
  (let* ((lines (org-lines))
         (n (org-current-line lines))
         (col (org-current-column lines))
         (i (org-item-at lines n))
         (item (org-parse-item (org-line lines i)))
         (indent (car item))
         (text-start (- (string-length (org-line lines i)) (string-length (list-ref item 3))))
         (new (org-make-item indent (cadr item) (and checkbox "[ ]") "")))
    ;; 新しい項目の行
    (define k
      (if (and (= n i) (<= col text-start))
          ;; 文の前: 前に入れる
          (begin (org-replace-lines lines i i (list new (org-line lines i))) i)
          ;; 分ける (カーソルの後の文を, 新しい項目にする)
          (let* ((p (point))
                 (before (buffer-substring 1 p))
                 (after (buffer-substring p (point-max)))
                 (from (+ 1 (string-length (cadr (regexp-match "^([\\s\\S]*?)[ \\t\\n]*$" before)))))
                 (to (+ p (string-length (car (regexp-match "^[ \\t]*" after))))))
            (org-replace-region from to (string-append "\n" new))
            (+ (car (org-pos-line lines from)) 1))))
    ;; 新しい項目の, 文の始めへ
    (let ((lines (org-lines)))
      (org-fix-bullets lines k)
      (let* ((lines (org-lines))
             (line (org-line lines k))
             (item (org-parse-item line)))
        (goto-char (+ (org-line-start lines k) (- (string-length line) (string-length (list-ref item 3)))))
        (when checkbox (org-update-statistics))))))

;; 項目を, 前か後の項目と入れ換える
(define (org-move-item forward)
  (let* ((lines (org-lines))
         (n (org-current-line lines))
         (col (org-current-column lines))
         (i (or (and (org-parse-item (org-line lines n)) n) (error "Not at an item")))
         (sibling (or (org-item-sibling lines i forward)
                      (error (if forward "Cannot move this item further down"
                                 "Cannot move this item further up"))))
         (first (if forward i sibling))
         (second (if forward sibling i))
         (last (org-item-end lines second))
         (block (lambda (from to) (org-take (org-drop lines (- from 1)) (+ (- to from) 1))))
         (a (block first (- second 1)))
         (b (block second last)))
    (org-replace-lines lines first last (append b a))
    (let ((moved (if forward (+ first (length b)) first)))
      (org-fix-bullets (org-lines) moved)
      (org-goto-line-column (org-lines) moved col))))

;; 項目の字下げを変える (subtree が #t なら, 子も. delta は 1 (深く) か -1 (浅く))
(define (org-indent-item delta subtree)
  (let* ((lines (org-lines))
         (n (org-current-line lines))
         (col (org-current-column lines))
         (i (or (and (org-parse-item (org-line lines n)) n) (error "Not at an item")))
         (item (org-parse-item (org-line lines i)))
         (indent (org-indentation (org-line lines i)))
         (end (org-item-end lines i))
         (children (org-item-children lines i))
         (last (if subtree end (if (null? children) end (- (car children) 1))))
         (previous (org-item-sibling lines i #f))
         (parent (org-item-parent lines i))
         (new-indent
          (if (> delta 0)
              (if previous
                  (let ((p (org-parse-item (org-line lines previous))))
                    (+ (org-indentation (org-line lines previous)) (string-length (cadr p)) 1))
                  (if parent
                      (error "Cannot indent the first item of a list")
                      (if subtree (+ indent 1)
                          (error "At first item: use S-M-<left/right> to move the whole list"))))
              (cond (parent (org-indentation (org-line lines parent)))
                    ((and (not previous) subtree)
                     (if (= indent 0) (error "Cannot outdent beyond margin") (- indent 1)))
                    ((not previous) (error "At first item: use S-M-<left/right> to move the whole list"))
                    (else (error "Cannot outdent top-level items"))))))
    (when (and (< delta 0) (not subtree) (not (null? children)))
      (error "Cannot outdent an item without its children"))
    (let* ((shift (- new-indent indent))
           (whole (and (> delta 0) (not previous) (not parent)))
           (to (if whole (org-item-end lines (org-last (org-item-siblings lines i))) last)))
      (org-replace-lines
       lines i to
       (map (lambda (line)
              (cond ((org-blank? line) line)
                    ((> shift 0) (string-append (org-spaces shift) line))
                    (else (substring line (min (org-indentation line) (- shift))))))
            (org-take (org-drop lines (- i 1)) (+ (- to i) 1))))
      (let ((lines (org-lines)))
        (org-fix-bullets lines i)
        (org-goto-line-column (org-lines) n (max 0 (+ col shift)))
        (org-update-statistics)))))

;; 印の順 (S-左右, C-c -)
(define org-list-bullets '("-" "+" "*" "1." "1)"))

;; リストの印を変える (forward が #f なら, 前の印)
(define (org-cycle-list-bullet forward)
  (let* ((lines (org-lines))
         (i (or (org-in-item) (error "Not at an item")))
         (siblings (org-item-siblings lines i))
         (item (org-parse-item (org-line lines (car siblings))))
         (indent (car item))
         (current (if (org-ordered-bullet? (cadr item))
                      (string-append "1" (substring (cadr item) (- (string-length (cadr item)) 1)))
                      (cadr item)))
         (bullets (if (equal? indent "") (filter (lambda (b) (not (equal? b "*"))) org-list-bullets)
                      org-list-bullets))
         (index (or (org-index-of current bullets) 0))
         (new (list-ref bullets (modulo (+ index (if forward 1 -1)) (length bullets))))
         (col (org-current-column lines))
         (n (org-current-line lines)))
    (for-each (lambda (k)
                (let* ((lines (org-lines))
                       (item (org-parse-item (org-line lines k))))
                  (org-set-line lines k (org-make-item (car item) new (caddr item) (list-ref item 3)))))
              siblings)
    (org-fix-bullets (org-lines) (car siblings))
    (org-goto-line-column (org-lines) n col)))

;; 次と前の項目へ (S-上下)
(define (org-item-move-point forward)
  (let* ((lines (org-lines))
         (i (or (org-in-item) (error "Not in an item")))
         (sibling (org-item-sibling lines i forward)))
    (if sibling
        (goto-char (org-line-start lines sibling))
        (error (if forward "On last item" "On first item")))))

;; C-c -: 表なら横線, 項目なら印を変える, 見出しと文は項目にする
(define-command (org-ctrl-c-minus)
  (let* ((lines (org-lines))
         (n (org-current-line lines))
         (line (org-line lines n)))
    (cond ((org-table-line? line) (org-table-insert-hline))
          ((org-in-item) (org-cycle-list-bullet #t))
          ((org-level line)
           (let ((parsed (org-parse-heading line)))
             (org-set-line lines n (string-append "- " (or (list-ref parsed 3) "")))))
          (else
           (org-set-line lines n (string-append (car (regexp-match "^[ \\t]*" line)) "- "
                                                (string-trim line)))))))

;;; チェックボックスと, 数える印 ([2/3] [66%])

;; チェックボックスを切り換える
(define-command (org-toggle-checkbox)
  (let* ((lines (org-lines))
         (i (or (org-in-item) (error "Cannot toggle checkbox outside of a list")))
         (item (org-parse-item (org-line lines i))))
    (if (caddr item)
        (begin
          (org-set-line lines i (org-make-item (car item) (cadr item)
                                               (if (equal? (caddr item) "[X]") "[ ]" "[X]")
                                               (list-ref item 3)))
          (org-fix-parent-checkboxes (org-lines) i)
          (org-update-statistics))
        (message "Item has no checkbox"))))

;; 上の項目のチェックボックスを, 子に合わせる (全て済みなら [X], 一部なら [-])
(define (org-fix-parent-checkboxes lines i)
  (let ((parent (org-item-parent lines i)))
    (when parent
      (let* ((item (org-parse-item (org-line lines parent)))
             (boxes (filter (lambda (b) b)
                            (map (lambda (k) (caddr (org-parse-item (org-line lines k))))
                                 (org-item-children lines parent))))
             (done (length (filter (lambda (b) (equal? b "[X]")) boxes))))
        (when (and (caddr item) (not (null? boxes)))
          (org-set-line lines parent
                        (org-make-item (car item) (cadr item)
                                       (cond ((= done (length boxes)) "[X]")
                                             ((and (= done 0)
                                                   (null? (filter (lambda (b) (equal? b "[-]")) boxes)))
                                              "[ ]")
                                             (else "[-]"))
                                       (list-ref item 3)))
          (org-fix-parent-checkboxes (org-lines) parent))))))

;; 数える印の, 済みと全ての数 ((済み 全て))
(define (org-statistics lines n)
  (let ((line (org-line lines n)))
    (if (org-level line)
        (let* ((end (org-entry-end lines n))
               (items (filter (lambda (k) (org-parse-item (org-line lines k)))
                              (map (lambda (i) (+ n 1 i)) (org-range (- end n)))))
               (indent (if (null? items) 0
                           (apply min (map (lambda (k) (org-indentation (org-line lines k))) items))))
               (boxes (filter (lambda (b) b)
                              (map (lambda (k) (caddr (org-parse-item (org-line lines k))))
                                   (filter (lambda (k) (= (org-indentation (org-line lines k)) indent))
                                           items)))))
          (if (null? boxes)
              ;; チェックボックスが無ければ, 子の見出しの TODO
              (let* ((last (org-subtree-end lines n))
                     (children (filter (lambda (h) (and (> (car h) n) (<= (car h) last)))
                                       (org-headings lines)))
                     (depth (if (null? children) 0 (apply min (map cadr children))))
                     (todos (filter (lambda (todo) todo)
                                    (map (lambda (h) (cadr (org-parse-heading (org-line lines (car h)))))
                                         (filter (lambda (h) (= (cadr h) depth)) children)))))
                (list (length (filter org-done-keyword? todos)) (length todos)))
              (list (length (filter (lambda (b) (equal? b "[X]")) boxes)) (length boxes))))
        (let ((boxes (filter (lambda (b) b)
                             (map (lambda (k) (caddr (org-parse-item (org-line lines k))))
                                  (org-item-children lines n)))))
          (list (length (filter (lambda (b) (equal? b "[X]")) boxes)) (length boxes))))))

;; 行の, 数える印を書き直す
(define (org-update-cookies-in-line lines n)
  (let* ((line (org-line lines n))
         (stats (org-statistics lines n))
         (cookies (org-all-matches "\\[([0-9]*)(%|/([0-9]*))\\]" line)))
    (unless (null? cookies)
      (org-set-line
       lines n
       (let loop ((rest cookies) (start 0) (result ""))
         (if (null? rest)
             (string-append result (substring line start))
             (let* ((m (car rest))
                    (percent (equal? (org-match-string line m 2) "%"))
                    (new (if percent
                             (format "[%d%%]" (if (= (cadr stats) 0) 0
                                                  (quotient (* 100 (car stats)) (cadr stats))))
                             (format "[%d/%d]" (car stats) (cadr stats)))))
               (loop (cdr rest) (cadr (car m))
                     (string-append result (substring line start (car (car m))) new)))))))))

;; バッファの, 全ての数える印を書き直す (C-c #)
(define-command (org-update-statistics-cookies) (org-update-statistics))

(define (org-update-statistics)
  (let ((lines (org-lines)))
    (for-each (lambda (n)
                (let ((line (org-line lines n)))
                  (when (and (regexp-match "\\[[0-9]*(?:%|/[0-9]*)\\]" line)
                             (or (org-level line) (org-parse-item line)))
                    (org-update-cookies-in-line (org-lines) n))))
              (map (lambda (i) (+ i 1)) (org-range (org-line-count lines))))))

;;; 日付

;; 今日 (2026-10-26 の形)
(define (org-today) (format-time-string "%Y-%m-%d"))

;; 日付の昼の時刻 (秒. 夏時間でも, 日を足して日付がずれないように, 昼にする)
(define (org-date-time date) (date-to-time (string-append date " 12:00")))

;; 日付に日を足す
(define (org-date-add date days)
  (format-time-string "%Y-%m-%d" (+ (org-date-time date) (* days 86400))))

;; 年, 月, 日から日付を作る (月と日が範囲を越えれば, 繰り上げる)
(define (org-make-date year month day)
  (let* ((m (- month 1))
         (y (+ year (/ (- m (modulo m 12)) 12)))
         (first (format "%d-%s-01" y (org-2d (+ (modulo m 12) 1)))))
    (org-date-add first (- day 1))))

;; 日付の (年 月 日)
(define (org-date-parts date) (map string->number (split-string date "-")))

;; 日付に月を足す
(define (org-date-add-months date months)
  (let ((p (org-date-parts date)))
    (org-make-date (car p) (+ (cadr p) months) (caddr p))))

;; a から b までの日数
(define (org-days-between a b)
  (let ((d (- (org-date-time b) (org-date-time a))))
    (if (>= d 0)
        (quotient (+ d 43200) 86400)
        (- (quotient (+ (- d) 43200) 86400)))))

;; 曜日 (Mon など)
(define (org-weekday date) (format-time-string "%a" (org-date-time date)))

;; 日付に, 数と単位 (d w m y) で足す
(define (org-date-shift date n unit)
  (cond ((equal? unit "d") (org-date-add date n))
        ((equal? unit "w") (org-date-add date (* 7 n)))
        ((equal? unit "m") (org-date-add-months date n))
        ((equal? unit "y") (org-date-add-months date (* 12 n)))
        (else date)))

;; 日付の文字 (<2026-10-26 Mon 10:00 +1w> の形. active が #f なら [ ])
(define (org-make-timestamp date time active . rest)
  (string-append (if active "<" "[") date " " (org-weekday date)
                 (if time (string-append " " time) "")
                 (if (and (pair? rest) (car rest) (not (equal? (car rest) ""))) (car rest) "")
                 (if active ">" "]")))

;; 今の日時 ([2026-10-26 Mon 10:00])
(define (org-now-inactive)
  (org-make-timestamp (org-today) (format-time-string "%H:%M") #f))

;; 日付の正規表現 (括弧: 1 括弧 2 日付 3 時刻 4 終わりの時刻 5 繰り返しと警告)
(define org-ts-regexp
  (string-append "([<\\[])([0-9]{4}-[0-9]{2}-[0-9]{2})(?: +[^ \\]>0-9+-]+)?"
                 "(?: +([0-9]{1,2}:[0-9]{2})(?:-([0-9]{1,2}:[0-9]{2}))?)?"
                 "((?: +[.+]?\\+[0-9]+[hdwmy])?(?: +-[0-9]+[hdwmy])?)[ ]*([>\\]])"))

;; 日付を分ける ((括弧 日付 時刻 終わりの時刻 繰り返しと警告). 日付でなければ #f)
(define (org-parse-timestamp s)
  (let ((m (regexp-match (string-append "^" org-ts-regexp "$") s)))
    (and m (equal? (if (equal? (cadr m) "<") ">" "]") (list-ref m 6))
         (list (cadr m) (caddr m) (list-ref m 3) (list-ref m 4) (list-ref m 5)))))

;; 分けた日付から, 文字を作る
(define (org-timestamp-string parsed)
  (let ((time (and (caddr parsed)
                   (if (list-ref parsed 3)
                       (string-append (caddr parsed) "-" (list-ref parsed 3))
                       (caddr parsed)))))
    (org-make-timestamp (cadr parsed) time (equal? (car parsed) "<") (list-ref parsed 4))))

;; 繰り返し ((印 数 単位). 無ければ #f)
(define (org-timestamp-repeater parsed)
  (let ((m (regexp-match " +([.+]?\\+)([0-9]+)([hdwmy])" (or (list-ref parsed 4) ""))))
    (and m (list (cadr m) (string->number (caddr m)) (list-ref m 3)))))

;; 警告の日数 (-3d など. 無ければ #f)
(define (org-timestamp-warning parsed)
  (let ((m (regexp-match " +-([0-9]+)([hdwmy])" (or (list-ref parsed 4) ""))))
    (and m (let ((n (string->number (cadr m))))
             (cond ((equal? (caddr m) "w") (* 7 n))
                   ((equal? (caddr m) "m") (* 30 n))
                   ((equal? (caddr m) "y") (* 365 n))
                   (else n))))))

;; 曜日の名前
(define org-weekdays '("sun" "mon" "tue" "wed" "thu" "fri" "sat"))

;; 入れた日付を読む ((日付 時刻). 時刻が無ければ #f. org-read-date と同じ書き方の一部)
;; 空 (既定), +3 (今日から 3 日後), +2w, -1d, ++1w (既定から), 2026-10-30, 10-30, 10/30, 30,
;; fri (次の金曜), 10:00, 10:00-11:00, 3pm
(define (org-read-date-analyze input default)
  (let* ((today (org-today))
         (s (string-downcase (string-trim input)))
         (tm (regexp-match "([0-9]{1,2}):([0-9]{2})(?:-([0-9]{1,2}):([0-9]{2}))?" s))
         (ampm (and (not tm) (regexp-match "\\b([0-9]{1,2})(am|pm)\\b" s)))
         (time (cond (tm (string-append (format "%d" (string->number (cadr tm))) ":" (caddr tm)
                                        (if (list-ref tm 3)
                                            (string-append "-" (format "%d" (string->number (list-ref tm 3)))
                                                           ":" (list-ref tm 4))
                                            "")))
                     (ampm (let ((h (modulo (string->number (cadr ampm)) 12)))
                             (format "%d:00" (if (equal? (caddr ampm) "pm") (+ h 12) h))))
                     (else #f)))
         (rest (string-trim (cond (tm (org-replace-all s (car tm) ""))
                                  (ampm (org-replace-all s (car ampm) ""))
                                  (else s))))
         (rel (regexp-match "^(\\+\\+|\\+|-|--)([0-9]*)([dwmy]?)$" rest))
         (iso (regexp-match "^([0-9]{4})-([0-9]{1,2})-([0-9]{1,2})$" rest))
         (md (regexp-match "^([0-9]{1,2})-([0-9]{1,2})$" rest))
         (us (regexp-match "^([0-9]{1,2})/([0-9]{1,2})(?:/([0-9]{2,4}))?$" rest))
         (day (regexp-match "^([0-9]{1,2})$" rest))
         (wday (and (>= (string-length rest) 3)
                    (org-index-of (substring rest 0 3) org-weekdays)))
         (this (org-date-parts today))
         (future (lambda (date later) (if (string<? date today) (later date) date)))
         (date
          (cond
           ((equal? rest "") default)
           (rel (let* ((n (if (equal? (caddr rel) "") 1 (string->number (caddr rel))))
                       (unit (if (equal? (list-ref rel 3) "") "d" (list-ref rel 3)))
                       (base (if (= (string-length (cadr rel)) 2) default today)))
                  (org-date-shift base (if (string-prefix? "-" (cadr rel)) (- n) n) unit)))
           (iso (org-make-date (string->number (cadr iso)) (string->number (caddr iso))
                               (string->number (list-ref iso 3))))
           (md (future (org-make-date (car this) (string->number (cadr md)) (string->number (caddr md)))
                       (lambda (d) (org-date-add-months d 12))))
           (us (if (list-ref us 3)
                   (let ((y (string->number (list-ref us 3))))
                     (org-make-date (if (< y 100) (+ 2000 y) y) (string->number (cadr us))
                                    (string->number (caddr us))))
                   (future (org-make-date (car this) (string->number (cadr us)) (string->number (caddr us)))
                           (lambda (d) (org-date-add-months d 12)))))
           (day (future (org-make-date (car this) (cadr this) (string->number (cadr day)))
                        (lambda (d) (org-date-add-months d 1))))
           (wday (let ((now (string->number (format-time-string "%w" (org-date-time today)))))
                   (org-date-add today (modulo (- wday now) 7))))
           (else (error "Cannot read date: %s" input)))))
    (list date (and time (org-normalize-time time)))))

;; 時刻の時を 2 桁にする (9:00 → 09:00)
(define (org-normalize-time time)
  (string-join (map (lambda (t)
                      (let ((p (split-string t ":")))
                        (string-append (org-2d (string->number (car p))) ":" (cadr p))))
                    (split-string time "-"))
               "-"))

;; 日付を聞く ((日付 時刻))
(define (org-read-date prompt default)
  (org-read-date-analyze (read-string (format "%sDate+time [%s]: " prompt default)) default))

;; カーソルの行の, カーソルのある日付 ((始め 終わり 分けたもの). 桁で. 無ければ #f)
(define (org-timestamp-at lines n col)
  (let ((line (org-line lines n)))
    (let loop ((ms (org-all-matches org-ts-regexp line)))
      (cond ((null? ms) #f)
            ((and (<= (car (car (car ms))) col) (< col (cadr (car (car ms)))))
             (let* ((range (car (car ms)))
                    (parsed (org-parse-timestamp (substring line (car range) (cadr range)))))
               (if parsed (list (car range) (cadr range) parsed) (loop (cdr ms)))))
            (else (loop (cdr ms)))))))

;; C-c . と C-c !: 日付を入れる (日付の上なら, 変える)
(define (org-insert-timestamp-command active)
  (let* ((lines (org-lines))
         (n (org-current-line lines))
         (at (org-timestamp-at lines n (org-current-column lines)))
         (default (if at (cadr (caddr at)) (org-today)))
         (answer (org-read-date "" default))
         (stamp (org-make-timestamp (car answer) (cadr answer) active)))
    (if at
        (let ((start (org-line-start lines n)))
          (org-replace-region (+ start (car at)) (+ start (cadr at)) stamp)
          (goto-char (+ start (car at) (string-length stamp))))
        (insert stamp))))

(define-command (org-time-stamp) (org-insert-timestamp-command #t))
(define-command (org-time-stamp-inactive) (org-insert-timestamp-command #f))

;; 日付を変える (S-上下左右). what: year month day hour minute
(define (org-timestamp-change parsed what n)
  (let* ((date (cadr parsed))
         (time (caddr parsed)))
    (if (member what '("hour" "minute"))
        ;; 範囲 (09:00-10:00) なら, 終わりも同じだけ動かす
        (let* ((end (list-ref parsed 3))
               (to-minutes (lambda (t) (let ((hm (map string->number (split-string t ":"))))
                                         (+ (* 60 (car hm)) (cadr hm)))))
               (hhmm (lambda (m) (string-append (org-2d (quotient m 60)) ":" (org-2d (modulo m 60)))))
               (minutes (to-minutes time))
               (step (if (equal? what "hour") 60 5))
               (rounded (if (and (equal? what "minute") (not (= 0 (modulo minutes 5))))
                            (if (> n 0) (- minutes (modulo minutes 5)) (+ minutes (- 5 (modulo minutes 5))))
                            minutes))
               (new (+ rounded (* n step)))
               (days (/ (- new (modulo new 1440)) 1440)))
          (list (car parsed) (org-date-add date days) (hhmm (modulo new 1440))
                (and end (hhmm (modulo (+ (to-minutes end) (- new minutes)) 1440)))
                (list-ref parsed 4)))
        (list (car parsed)
              (cond ((equal? what "year") (org-date-add-months date (* 12 n)))
                    ((equal? what "month") (org-date-add-months date n))
                    (else (org-date-add date n)))
              time (list-ref parsed 3) (list-ref parsed 4)))))

;; カーソルの所の, 日付の部分 (year month day hour minute)
(define (org-timestamp-part at col)
  (let* ((k (- col (car at)))
         (parsed (caddr at))
         (time (caddr parsed))
         ;; 時刻の始め (<YYYY-MM-DD 曜日 の後)
         (time-start (and time (+ 13 (string-length (org-weekday (cadr parsed))))))
         (hour-end (and time (+ time-start (string-length (car (split-string time ":")))))))
    ;; 終わりの直後も, その部分 (Emacs の org-at-timestamp-p と同じ)
    (cond ((and (>= k 1) (<= k 5)) "year")
          ((and (>= k 6) (<= k 8)) "month")
          ((and time (>= k time-start) (<= k hour-end)) "hour")
          ((and time (> k hour-end) (<= k (+ time-start (string-length time)))) "minute")
          (else "day"))))

;; カーソルの日付を変える (無ければ #f)
(define (org-shift-timestamp n by-day)
  (let* ((lines (org-lines))
         (line-n (org-current-line lines))
         (col (org-current-column lines))
         (at (org-timestamp-at lines line-n col)))
    (and at
         (let* ((what (if by-day "day" (org-timestamp-part at col)))
                (new (org-timestamp-string (org-timestamp-change (caddr at) what n)))
                (start (org-line-start lines line-n)))
           (org-replace-region (+ start (car at)) (+ start (cadr at)) new)
           (goto-char (+ start col))
           #t))))

;;; SCHEDULED, DEADLINE, CLOSED

;; 行が予定の行 (SCHEDULED: など) なら #t
(define (org-planning-line? line)
  (if (regexp-match "^[ \\t]*(?:SCHEDULED|DEADLINE|CLOSED): *[<\\[]" line) #t #f))

;; 予定の行の, ((言葉 日付) ...)
(define (org-planning-items line)
  (map (lambda (m) (list (org-match-string line m 1) (org-match-string line m 2)))
       (org-all-matches "(SCHEDULED|DEADLINE|CLOSED): *([<\\[][^>\\]\\n]*[>\\]])" line)))

;; n 行目の見出しの, 予定 (言葉 → 日付. 無ければ #f)
(define (org-get-planning lines n keyword)
  (and (< n (org-line-count lines))
       (org-planning-line? (org-line lines (+ n 1)))
       (let ((found (assoc keyword (org-planning-items (org-line lines (+ n 1))))))
         (and found (cadr found)))))

;; n 行目の見出しの予定を変える (stamp が #f なら消す. 新しいものは, 行の始めに書く)
(define (org-set-planning lines n keyword stamp)
  (let* ((has (and (< n (org-line-count lines)) (org-planning-line? (org-line lines (+ n 1)))))
         (line (and has (org-line lines (+ n 1))))
         (indent (if has (car (regexp-match "^[ \\t]*" line)) ""))
         (items (filter (lambda (i) (not (equal? (car i) keyword)))
                        (if has (org-planning-items line) '())))
         (new (if stamp (cons (list keyword stamp) items) items))
         (text (string-append indent (string-join (map (lambda (i) (string-append (car i) ": " (cadr i))) new) " "))))
    (cond ((and has (null? new)) (org-delete-lines lines (+ n 1) (+ n 1)))
          (has (org-set-line lines (+ n 1) text))
          ((not (null? new)) (org-insert-lines-after lines n (list text))))))

;; C-c C-s と C-c C-d: 予定と締め切りを書く
(define (org-planning-command keyword)
  (let* ((lines (org-lines))
         (h (org-back-to-heading lines))
         (old (org-get-planning lines h keyword))
         (parsed (and old (org-parse-timestamp old)))
         (answer (org-read-date (string-append keyword " ") (if parsed (cadr parsed) (org-today))))
         (stamp (org-make-timestamp (car answer) (cadr answer) #t (and parsed (list-ref parsed 4)))))
    (org-set-planning lines h keyword stamp)
    (message (if (equal? keyword "SCHEDULED") "Scheduled to %s" "Deadline on %s") stamp)))

(define-command (org-schedule) (org-planning-command "SCHEDULED"))
(define-command (org-deadline) (org-planning-command "DEADLINE"))

;;; TODO

;; 次か前の状態 (C-c C-t は, 同じ組の中で回す. S-左右は, 全ての状態を回す)
(define (org-next-todo old forward all)
  (let ((words (if all
                   (org-todo-keywords-1)
                   (let ((set (org-todo-set old))) (append (car set) (cadr set))))))
    (if (not old)
        (if forward (car words) (org-last words))
        (let ((i (org-index-of old words)))
          (cond ((not i) (car words))
                (forward (and (< (+ i 1) (length words)) (list-ref words (+ i 1))))
                (else (and (> i 0) (list-ref words (- i 1)))))))))

;; 状態をキーで選ぶ (SPC で消す)
(define (org-fast-todo-selection)
  (let* ((keys (org-todo-keys))
         (answer (read-string
                  (string-append (string-join (map (lambda (k) (format "[%s] %s" (car k) (cadr k))) keys) "  ")
                                 "  [SPC] clear: ")))
         (found (assoc answer keys)))
    (cond ((org-blank? answer) #f)
          (found (cadr found))
          ((member (string-upcase answer) (org-todo-keywords-1)) (string-upcase answer))
          (else (error "Quit")))))

;; 見出しの状態を変える (new が #f なら消す). 済みにすれば CLOSED を書き, 繰り返しなら日付を進める
(define (org-set-todo h new)
  (let* ((lines (org-lines))
         (parsed (org-parse-heading (org-line lines h)))
         (old (cadr parsed))
         (done (and (org-done-keyword? new) (not (org-done-keyword? old))))
         (repeats (and done (org-repeating-stamps lines h))))
    (org-set-line lines h (org-make-heading (car parsed) new (caddr parsed) (list-ref parsed 3)
                                            (list-ref parsed 4)))
    (cond
     ((and done (not (null? repeats)))
      (org-auto-repeat h new repeats))
     (done
      (when org-log-done (org-set-planning (org-lines) h "CLOSED" (org-now-inactive))))
     ((and (org-done-keyword? old) (not (org-done-keyword? new)))
      (when (org-get-planning (org-lines) h "CLOSED")
        (org-set-planning (org-lines) h "CLOSED" #f))))
    (org-update-statistics)))

;; 繰り返す予定 ((言葉 日付) ...)
(define (org-repeating-stamps lines h)
  (if (and (< h (org-line-count lines)) (org-planning-line? (org-line lines (+ h 1))))
      (filter (lambda (i) (and (member (car i) '("SCHEDULED" "DEADLINE"))
                               (let ((p (org-parse-timestamp (cadr i))))
                                 (and p (org-timestamp-repeater p)))))
              (org-planning-items (org-line lines (+ h 1))))
      '()))

;; 繰り返しの予定を進め, 始めの状態に戻す (LAST_REPEAT と, 状態の変わった記録を書く)
(define (org-auto-repeat h done-word repeats)
  (let* ((head (car (car (org-todo-set done-word))))
         (old-word (let ((set (org-todo-set done-word))) (and (not (null? (car set))) (car (car set)))))
         (lines (org-lines))
         (parsed (org-parse-heading (org-line lines h))))
    (org-set-line lines h (org-make-heading (car parsed) head (caddr parsed) (list-ref parsed 3)
                                            (list-ref parsed 4)))
    (for-each
     (lambda (item)
       (let* ((p (org-parse-timestamp (cadr item)))
              (r (org-timestamp-repeater p))
              (today (org-today))
              (shifted
               (cond
                ;; ++: 今日より後まで進める
                ((equal? (car r) "++")
                 (let loop ((d (org-date-shift (cadr p) (cadr r) (caddr r))))
                   (if (string<? today d) d (loop (org-date-shift d (cadr r) (caddr r))))))
                ;; .+: 今日から
                ((equal? (car r) ".+") (org-date-shift today (cadr r) (caddr r)))
                (else (org-date-shift (cadr p) (cadr r) (caddr r))))))
         (org-set-planning (org-lines) h (car item)
                           (org-timestamp-string (list (car p) shifted (caddr p) (list-ref p 3) (list-ref p 4))))))
     repeats)
    (when org-log-repeat
      (let* ((lines (org-lines))
             (stamp (org-now-inactive)))
        (org-entry-put lines h "LAST_REPEAT" stamp)
        (let* ((lines (org-lines))
               (after (org-log-beginning lines h)))
          (org-insert-lines-after lines after
                                  (list (format "- State %s from %s %s"
                                                (org-pad-right (format "\"%s\"" done-word) 12)
                                                (org-pad-right (if old-word (format "\"%s\"" old-word) "") 12)
                                                stamp))))))
    (let ((lines (org-lines)))
      (message "Entry repeats: %s"
               (if (org-planning-line? (org-line lines (+ h 1))) (string-trim (org-line lines (+ h 1))) "")))))

;; 記録を書く行 (見出しと, 予定の行と, PROPERTIES の後)
(define (org-log-beginning lines h)
  (let* ((n (if (and (< h (org-line-count lines)) (org-planning-line? (org-line lines (+ h 1)))) (+ h 1) h)))
    (if (and (< n (org-line-count lines))
             (regexp-match "^[ \\t]*:PROPERTIES:[ \\t]*$" (org-line lines (+ n 1))))
        (let loop ((k (+ n 2)))
          (cond ((> k (org-line-count lines)) n)
                ((regexp-match "^[ \\t]*:END:[ \\t]*$" (org-line lines k)) k)
                (else (loop (+ k 1)))))
        n)))

;; 見出しの PROPERTIES に, 値を書く (無ければ, 作る)
(define (org-entry-put lines h name value)
  (let* ((n (if (and (< h (org-line-count lines)) (org-planning-line? (org-line lines (+ h 1)))) (+ h 1) h))
         (line (format ":%s: %s" name value)))
    (if (and (< n (org-line-count lines))
             (regexp-match "^[ \\t]*:PROPERTIES:[ \\t]*$" (org-line lines (+ n 1))))
        (let loop ((k (+ n 2)))
          (cond ((or (> k (org-line-count lines)) (regexp-match "^[ \\t]*:END:[ \\t]*$" (org-line lines k)))
                 (org-replace-lines lines (- k 1) (- k 1) (list (org-line lines (- k 1)) line)))
                ((regexp-match (format "^[ \\t]*:%s:" (regexp-quote name)) (org-line lines k))
                 (org-set-line lines k line))
                (else (loop (+ k 1)))))
        (org-insert-lines-after lines n (list ":PROPERTIES:" line ":END:")))))

;; 見出しの PROPERTIES の値 (無ければ #f)
(define (org-entry-get lines h name)
  (let ((end (org-entry-end lines h)))
    (let loop ((k (+ h 1)))
      (if (> k end)
          #f
          (let ((m (regexp-match (format "^[ \\t]*:%s:[ \\t]*(.*?)[ \\t]*$" (regexp-quote name)) (org-line lines k))))
            (if m (cadr m) (loop (+ k 1))))))))

;; C-c C-t: 状態を変える (キーの付いた状態があれば, キーで選ぶ)
(define-command (org-todo)
  (let* ((lines (org-lines))
         (h (org-back-to-heading lines))
         (old (cadr (org-parse-heading (org-line lines h)))))
    (org-set-todo h (if (null? (org-todo-keys))
                        (org-next-todo old #t #f)
                        (org-fast-todo-selection)))))

;;; 優先度

;; 優先度の文字 (高い順)
(define (org-priority-chars)
  (let* ((letters "ABCDEFGHIJKLMNOPQRSTUVWXYZ")
         (from (string-search org-priority-highest letters))
         (to (string-search org-priority-lowest letters)))
    (map (lambda (i) (org-char letters (+ from i))) (org-range (+ (- to from) 1)))))

;; 見出しの優先度を変える (new が #f なら消す)
(define (org-set-priority new)
  (let* ((lines (org-lines))
         (h (org-back-to-heading lines))
         (parsed (org-parse-heading (org-line lines h))))
    (org-set-line lines h (org-make-heading (car parsed) (cadr parsed) (and new (format "[#%s]" new))
                                            (list-ref parsed 3) (list-ref parsed 4)))
    (if new
        (message "Priority of current item set to %s" new)
        (message "Priority removed"))))

;; 優先度を上げる, 下げる (S-上下. 範囲を越えれば消す)
(define (org-priority-shift up)
  (let* ((lines (org-lines))
         (h (org-back-to-heading lines))
         (current (caddr (org-parse-heading (org-line lines h))))
         (chars (org-priority-chars)))
    (if (not current)
        (org-set-priority org-priority-default)
        (let ((i (+ (or (org-index-of (substring current 2 3) chars) 0) (if up -1 1))))
          (org-set-priority (and (>= i 0) (< i (length chars)) (list-ref chars i)))))))

(define-command (org-priority-up) (org-priority-shift #t))
(define-command (org-priority-down) (org-priority-shift #f))

;; C-c ,: 優先度を選ぶ (SPC で消す)
(define-command (org-priority)
  (let ((answer (string-upcase
                 (read-string (format "Priority %s-%s, SPC to remove: " org-priority-highest org-priority-lowest)))))
    (cond ((org-blank? answer) (org-set-priority #f))
          ((member answer (org-priority-chars)) (org-set-priority answer))
          (else (error "Priority must be between `%s' and `%s'" org-priority-highest org-priority-lowest)))))

;;; タグ

;; バッファの全てのタグ
(define (org-buffer-tags lines)
  (org-unique (apply append (map (lambda (line)
                                   (let ((parsed (org-parse-heading line)))
                                     (if parsed (org-heading-tags parsed) '())))
                                 lines))))

;; 見出しのタグを変える
(define (org-set-tags h tags)
  (let* ((lines (org-lines))
         (parsed (org-parse-heading (org-line lines h))))
    (org-set-line lines h (org-make-heading (car parsed) (cadr parsed) (caddr parsed) (list-ref parsed 3)
                                            (org-tags-string tags)))))

;; C-c C-q: タグを入れる (: か空白で分ける)
(define-command (org-set-tags-command)
  (let* ((lines (org-lines))
         (h (org-back-to-heading lines))
         (parsed (org-parse-heading (org-line lines h)))
         (all (org-buffer-tags lines))
         (answer (read-string (if (null? all) "Tags: " (format "Tags (%s): " (string-join all " ")))
                              (or (list-ref parsed 4) ""))))
    (org-set-tags h (org-unique (filter (lambda (t) (not (equal? t "")))
                                        (split-string (org-replace-all answer " " ":") ":"))))))

;;; 表

;; 表の行なら #t
(define (org-table-line? line) (if (regexp-match "^[ \\t]*\\|" line) #t #f))

;; 横線の行なら #t
(define (org-table-hline? line) (if (regexp-match "^[ \\t]*\\|-" line) #t #f))

;; n 行目を含む表の, 始めと終わりの行 ((始め 終わり))
(define (org-table-bounds lines n)
  (let ((first (let loop ((k n))
                 (if (and (> k 1) (org-table-line? (org-line lines (- k 1)))) (loop (- k 1)) k)))
        (last (let loop ((k n))
                (if (and (< k (org-line-count lines)) (org-table-line? (org-line lines (+ k 1))))
                    (loop (+ k 1))
                    k))))
    (list first last)))

;; 表の行の欄 (横線なら 'hline)
(define (org-table-cells line)
  (if (org-table-hline? line)
      'hline
      (let* ((body (cadr (regexp-match "^[ \\t]*\\|(.*?)\\|?[ \\t]*$" line))))
        (map string-trim (split-string body "|")))))

;; 欄が数なら #t
(define (org-table-number? s)
  (if (regexp-match "^[<>]?[-+^.0-9]*[0-9][-+^.0-9eEdDx()%:]*$" s) #t #f))

;; 表の行を揃える (rows は, 欄のリストか 'hline のリスト. 揃えた行のリストを返す)
(define (org-table-render indent rows)
  (let* ((data (filter pair? rows))
         (columns (if (null? data) 1 (apply max (map length data))))
         (cell (lambda (row i) (if (< i (length row)) (list-ref row i) "")))
         (widths (map (lambda (i)
                        (apply max (cons 1 (map (lambda (row) (string-width (cell row i))) data))))
                      (org-range columns)))
         (numeric (map (lambda (i)
                         (let* ((cells (filter (lambda (c) (not (equal? c "")))
                                               (map (lambda (row) (cell row i)) data)))
                                (numbers (filter org-table-number? cells)))
                           (and (not (null? cells)) (> (* 2 (length numbers)) (length cells)))))
                       (org-range columns))))
    (map (lambda (row)
           (if (eq? row 'hline)
               (string-append indent "|"
                              (string-join (map (lambda (w) (org-repeat "-" (+ w 2))) widths) "+") "|")
               (string-append indent "| "
                              (string-join (map (lambda (i)
                                                  (if (list-ref numeric i)
                                                      (org-pad-left (cell row i) (list-ref widths i))
                                                      (org-pad-right (cell row i) (list-ref widths i))))
                                                (org-range columns))
                                           " | ")
                              " |")))
         rows)))

;; カーソルの表 ((始め 終わり 字下げ 行 欄の番号 欄の中の桁))
(define (org-table-context)
  (let* ((lines (org-lines))
         (n (org-current-line lines))
         (col (org-current-column lines))
         (bounds (org-table-bounds lines n))
         (first (car bounds))
         (line (org-line lines n))
         (indent (car (regexp-match "^[ \\t]*" line)))
         (before (substring line 0 col))
         (column (max 0 (- (length (split-string before "|")) 2)))
         (cell-start (let loop ((i 0) (pos (string-length indent)))
                       (let ((next (string-search "|" line (+ pos 1))))
                         (if (or (= i column) (not next)) (+ pos 1) (loop (+ i 1) next)))))
         (offset (max 0 (- col cell-start
                           (string-length (car (regexp-match "^[ ]*" (substring line cell-start))))))))
    (list first (cadr bounds) indent (- n first) column offset)))

;; 表を揃え, 行 row, 欄 column (の, 文字の始めから offset) へ動く. rows-change で, 行を変えてから揃える
(define (org-table-align-and-go rows-change row column offset)
  (let* ((ctx (org-table-context))
         (lines (org-lines))
         (rows (rows-change (map org-table-cells (org-take (org-drop lines (- (car ctx) 1))
                                                           (+ (- (cadr ctx) (car ctx)) 1)))))
         (rendered (org-table-render (caddr ctx) rows)))
    (org-replace-lines lines (car ctx) (cadr ctx) rendered)
    (let* ((r (max 0 (min row (- (length rendered) 1))))
           (line (list-ref rendered r)))
      (org-goto-line-column (org-lines) (+ (car ctx) r)
                            (if (org-table-hline? line)
                                (string-length (caddr ctx))
                                (org-table-cell-column line column offset))))))

;; 揃えた行の, 欄 column の文字の始めから offset の桁 (空の欄なら, | の後の空白の後)
(define (org-table-cell-column line column offset)
  (let* ((start (let loop ((i 0) (pos (string-search "|" line)))
                  (let ((next (string-search "|" line (+ pos 1))))
                    (if (or (= i column) (not next) (= next (- (string-length line) 1)))
                        (+ pos 1)
                        (loop (+ i 1) next)))))
         (end (or (string-search "|" line start) (string-length line)))
         (cell (substring line start end))
         (spaces (string-length (car (regexp-match "^ *" cell))))
         (text (string-trim cell)))
    (if (equal? text "")
        (+ start 1)
        (+ start spaces (min offset (string-length text))))))

;; 欄の数
(define (org-table-columns rows)
  (let ((data (filter pair? rows)))
    (if (null? data) 1 (apply max (map length data)))))

;; 揃える (C-c C-c)
(define-command (org-table-align)
  (let ((ctx (org-table-context)))
    (org-table-align-and-go (lambda (rows) rows) (list-ref ctx 3) (list-ref ctx 4) (list-ref ctx 5))))

;; 空の行
(define (org-table-empty-row rows) (map (lambda (i) "") (org-range (org-table-columns rows))))

;; 次の欄へ (TAB. 最後の欄なら, 行を足す)
(define-command (org-table-next-field)
  (let* ((ctx (org-table-context))
         (row (list-ref ctx 3))
         (column (list-ref ctx 4))
         (lines (org-lines))
         (rows (map org-table-cells (org-take (org-drop lines (- (car ctx) 1)) (+ (- (cadr ctx) (car ctx)) 1))))
         (columns (org-table-columns rows))
         (next (let loop ((r row) (c (+ column 1)))
                 (cond ((>= r (length rows)) #f)
                       ((eq? (list-ref rows r) 'hline) (loop (+ r 1) 0))
                       ((< c columns) (list r c))
                       (else (loop (+ r 1) 0))))))
    (if next
        (org-table-align-and-go (lambda (rows) rows) (car next) (cadr next) 0)
        (org-table-align-and-go (lambda (rows) (append rows (list (org-table-empty-row rows))))
                                (length rows) 0 0))))

;; 前の欄へ (S-TAB)
(define-command (org-table-previous-field)
  (let* ((ctx (org-table-context))
         (lines (org-lines))
         (rows (map org-table-cells (org-take (org-drop lines (- (car ctx) 1)) (+ (- (cadr ctx) (car ctx)) 1))))
         (columns (org-table-columns rows))
         (previous (let loop ((r (list-ref ctx 3)) (c (- (list-ref ctx 4) 1)))
                     (cond ((< r 0) #f)
                           ((eq? (list-ref rows r) 'hline) (loop (- r 1) (- columns 1)))
                           ((>= c 0) (list r c))
                           (else (loop (- r 1) (- columns 1)))))))
    (if previous
        (org-table-align-and-go (lambda (rows) rows) (car previous) (cadr previous) 0)
        (org-table-align-and-go (lambda (rows) rows) (list-ref ctx 3) (list-ref ctx 4) 0))))

;; 次の行の, 同じ欄へ (RET. 次が無いか横線なら, 行を足す)
(define-command (org-table-next-row)
  (let* ((ctx (org-table-context))
         (row (list-ref ctx 3))
         (lines (org-lines))
         (rows (map org-table-cells (org-take (org-drop lines (- (car ctx) 1)) (+ (- (cadr ctx) (car ctx)) 1)))))
    (if (and (< (+ row 1) (length rows)) (not (eq? (list-ref rows (+ row 1)) 'hline)))
        (org-table-align-and-go (lambda (rows) rows) (+ row 1) (list-ref ctx 4) 0)
        (org-table-align-and-go (lambda (rows) (append (org-take rows (+ row 1))
                                                       (list (org-table-empty-row rows))
                                                       (org-drop rows (+ row 1))))
                                (+ row 1) (list-ref ctx 4) 0))))

;; 横線を下に入れる (C-c -)
(define (org-table-insert-hline)
  (let ((ctx (org-table-context)))
    (org-table-align-and-go (lambda (rows) (append (org-take rows (+ (list-ref ctx 3) 1)) (list 'hline)
                                                   (org-drop rows (+ (list-ref ctx 3) 1))))
                            (list-ref ctx 3) (list-ref ctx 4) (list-ref ctx 5))))

;; リストの i 番目と j 番目を入れ換える
(define (org-swap items i j)
  (map (lambda (k) (list-ref items (cond ((= k i) j) ((= k j) i) (else k)))) (org-range (length items))))

;; 行の欄を変える (横線はそのまま)
(define (org-table-map-cells rows f)
  (map (lambda (row) (if (eq? row 'hline) row (f row))) rows))

;; 行を, 欄の数に揃える
(define (org-table-pad rows)
  (let ((columns (org-table-columns rows)))
    (org-table-map-cells rows (lambda (row) (append row (map (lambda (i) "") (org-range (- columns (length row)))))))))

;; 列を動かす (M-左右)
(define (org-table-move-column forward)
  (let* ((ctx (org-table-context))
         (c (list-ref ctx 4))
         (d (if forward (+ c 1) (- c 1)))
         (lines (org-lines))
         (rows (map org-table-cells (org-take (org-drop lines (- (car ctx) 1)) (+ (- (cadr ctx) (car ctx)) 1)))))
    (when (or (< d 0) (>= d (org-table-columns rows)))
      (error "Cannot move column further"))
    (org-table-align-and-go (lambda (rows) (org-table-map-cells (org-table-pad rows) (lambda (row) (org-swap row c d))))
                            (list-ref ctx 3) d (list-ref ctx 5))))

;; 行を動かす (M-上下)
(define (org-table-move-row forward)
  (let* ((ctx (org-table-context))
         (r (list-ref ctx 3))
         (d (if forward (+ r 1) (- r 1))))
    (when (or (< d 0) (> d (- (cadr ctx) (car ctx))))
      (error "Cannot move row further"))
    (org-table-align-and-go (lambda (rows) (org-swap rows r d)) d (list-ref ctx 4) (list-ref ctx 5))))

;; 列を消す (M-S-左)
(define (org-table-delete-column)
  (let* ((ctx (org-table-context))
         (c (list-ref ctx 4)))
    (org-table-align-and-go
     (lambda (rows) (org-table-map-cells (org-table-pad rows)
                                         (lambda (row) (append (org-take row c) (org-drop row (+ c 1))))))
     (list-ref ctx 3) (max 0 (- c 1)) 0)))

;; 列を左に足す (M-S-右)
(define (org-table-insert-column)
  (let* ((ctx (org-table-context))
         (c (list-ref ctx 4)))
    (org-table-align-and-go
     (lambda (rows) (org-table-map-cells (org-table-pad rows)
                                         (lambda (row) (append (org-take row c) (list "") (org-drop row c)))))
     (list-ref ctx 3) c 0)))

;; 行を消す (M-S-上)
(define (org-table-kill-row)
  (let* ((ctx (org-table-context))
         (r (list-ref ctx 3)))
    (if (= (car ctx) (cadr ctx))
        (let ((lines (org-lines)))
          (org-delete-lines lines (car ctx) (car ctx)))
        (org-table-align-and-go (lambda (rows) (append (org-take rows r) (org-drop rows (+ r 1))))
                                (min r (- (cadr ctx) (car ctx) 1)) (list-ref ctx 4) 0))))

;; 行を上に足す (M-S-下)
(define (org-table-insert-row)
  (let* ((ctx (org-table-context))
         (r (list-ref ctx 3)))
    (org-table-align-and-go (lambda (rows) (append (org-take rows r) (list (org-table-empty-row rows)) (org-drop rows r)))
                            r (list-ref ctx 4) 0)))

;; C-c |: 表を作る
(define-command (org-table-create-or-convert-from-region)
  (let* ((lines (org-lines))
         (n (org-current-line lines)))
    (if (org-table-line? (org-line lines n))
        (error "Table already exists")
        (let* ((answer (read-string "Table size Columns x Rows [e.g. 5x2]: " "5x2"))
               (m (or (regexp-match "^ *([0-9]+) *[xX*] *([0-9]+) *$" answer)
                      (error "Invalid table size: %s" answer)))
               (columns (string->number (cadr m)))
               (rows (string->number (caddr m)))
               (indent (car (regexp-match "^[ \\t]*" (org-line lines n))))
               (empty (map (lambda (i) "") (org-range columns)))
               (table (org-table-render indent (append (list empty) (if (> rows 1) (list 'hline) '())
                                                       (map (lambda (i) empty) (org-range (- rows 1)))))))
          (cond ((and (org-blank? (org-line lines n)) (= n (length lines)))
                 ;; 最後の改行の後: 表の後にも改行を入れる
                 (org-replace-lines lines n n (append table '(""))))
                ((org-blank? (org-line lines n)) (org-replace-lines lines n n table))
                (else (org-insert-lines-after lines n table)))
          (let ((lines (org-lines)))
            (org-goto-line-column lines (if (org-table-line? (org-line lines n)) n (+ n 1))
                                  (+ (string-length indent) 2)))))))

;;; 並べ替え

;; C-c ^: 見出しの子を並べ替える (a: 題, n: 数, o: TODO の順, p: 優先度, t: 日付. 大文字は逆)
(define-command (org-sort)
  (let* ((lines (org-lines))
         (h (org-back-to-heading lines))
         (last (org-subtree-end lines h))
         (children (filter (lambda (x) (and (> (car x) h) (<= (car x) last))) (org-headings lines))))
    (when (null? children) (error "Nothing to sort"))
    (let* ((answer (read-string "Sort: [a]lpha  [n]umeric  [p]riority  [t]ime  t[o]do order  A/N/P/T/O means reversed: "))
           (how (string-downcase answer))
           (reverse? (not (equal? how answer)))
           (depth (apply min (map cadr children)))
           (starts (map car (filter (lambda (x) (= (cadr x) depth)) children)))
           (blocks (map (lambda (s) (org-take (org-drop lines (- s 1)) (+ (- (org-subtree-end lines s) s) 1))) starts))
           (key (lambda (block)
                  (let* ((parsed (org-parse-heading (car block)))
                         (title (or (list-ref parsed 3) "")))
                    (cond ((equal? how "a") (string-downcase title))
                          ((equal? how "n") (let ((m (regexp-match "^[-+]?[0-9.]+" title)))
                                              (if m (string->number (car m)) 0)))
                          ((equal? how "o") (or (org-index-of (cadr parsed) (org-todo-keywords-1)) 1000))
                          ((equal? how "p") (or (org-index-of (if (caddr parsed) (substring (caddr parsed) 2 3)
                                                                  org-priority-default)
                                                              (org-priority-chars))
                                                1000))
                          ((equal? how "t") (let ((m (regexp-match "[<\\[]([0-9]{4}-[0-9]{2}-[0-9]{2})"
                                                                   (string-join block "\n"))))
                                              (if m (cadr m) "9999")))
                          (else (error "Invalid sorting type `%s'" answer))))))
           (less (lambda (a b) (let ((x (key a)) (y (key b)))
                                 (if (string? x) (string<? x y) (< x y)))))
           (sorted (sort blocks (if reverse? (lambda (a b) (less b a)) less))))
      (org-replace-lines lines (car starts) last (apply append sorted))
      (goto-char (org-line-start (org-lines) h))
      (message "Sorting entries...done"))))

;;; キーの振り分け (表, 見出し, リスト, 日付で, 動きを変える)

;; ほかの所で押したとき
(define (org-modifier-cursor-error)
  (error "This command is active in special context like tables, headlines or items"))
(define (org-shiftselect-error)
  (error "To use shift-selection with Org mode, customize `org-support-shift-select'"))

;; カーソルの行の種類 (table heading item text)
(define (org-context)
  (let* ((lines (org-lines))
         (line (org-line lines (org-current-line lines))))
    (cond ((org-table-line? line) 'table)
          ((org-level line) 'heading)
          ((org-parse-item line) 'item)
          (else 'text))))

(define-command (org-metaleft)
  (case (org-context)
    ((table) (org-table-move-column #f))
    ((heading) (org-do-promote))
    ((item) (org-indent-item -1 #f))
    (else (call-interactively 'backward-word))))

(define-command (org-metaright)
  (case (org-context)
    ((table) (org-table-move-column #t))
    ((heading) (org-do-demote))
    ((item) (org-indent-item 1 #f))
    (else (call-interactively 'forward-word))))

(define-command (org-metaup)
  (case (org-context)
    ((table) (org-table-move-row #f))
    ((heading) (org-move-subtree #f))
    ((item) (org-move-item #f))
    (else (org-drag-line #f))))

(define-command (org-metadown)
  (case (org-context)
    ((table) (org-table-move-row #t))
    ((heading) (org-move-subtree #t))
    ((item) (org-move-item #t))
    (else (org-drag-line #t))))

(define-command (org-shiftmetaleft)
  (case (org-context)
    ((table) (org-table-delete-column))
    ((heading) (org-promote-subtree))
    ((item) (org-indent-item -1 #t))
    (else (org-modifier-cursor-error))))

(define-command (org-shiftmetaright)
  (case (org-context)
    ((table) (org-table-insert-column))
    ((heading) (org-demote-subtree))
    ((item) (org-indent-item 1 #t))
    (else (org-modifier-cursor-error))))

(define-command (org-shiftmetaup)
  (case (org-context)
    ((table) (org-table-kill-row))
    ((heading) (org-move-subtree #f))
    ((item) (org-move-item #f))
    (else (org-drag-line #f))))

(define-command (org-shiftmetadown)
  (case (org-context)
    ((table) (org-table-insert-row))
    ((heading) (org-move-subtree #t))
    ((item) (org-move-item #t))
    (else (org-drag-line #t))))

;; カーソルがリストの印の上なら #t
(define (org-at-item-bullet-p)
  (let* ((lines (org-lines))
         (line (org-line lines (org-current-line lines)))
         (item (org-parse-item line)))
    (and item (< (org-current-column lines) (+ (string-length (car item)) (string-length (cadr item)) 1)))))

(define-command (org-shiftup)
  (cond ((org-shift-timestamp 1 #f) #t)
        ((eq? (org-context) 'heading) (org-priority-up))
        ((org-in-item) (org-item-move-point #f))
        (else (org-shiftselect-error))))

(define-command (org-shiftdown)
  (cond ((org-shift-timestamp -1 #f) #t)
        ((eq? (org-context) 'heading) (org-priority-down))
        ((org-in-item) (org-item-move-point #t))
        (else (org-shiftselect-error))))

(define-command (org-shiftright)
  (cond ((org-shift-timestamp 1 #t) #t)
        ((eq? (org-context) 'heading)
         (let* ((lines (org-lines)) (h (org-back-to-heading lines)))
           (org-set-todo h (org-next-todo (cadr (org-parse-heading (org-line lines h))) #t #t))))
        ((org-at-item-bullet-p) (org-cycle-list-bullet #t))
        (else (org-shiftselect-error))))

(define-command (org-shiftleft)
  (cond ((org-shift-timestamp -1 #t) #t)
        ((eq? (org-context) 'heading)
         (let* ((lines (org-lines)) (h (org-back-to-heading lines)))
           (org-set-todo h (org-next-todo (cadr (org-parse-heading (org-line lines h))) #f #t))))
        ((org-at-item-bullet-p) (org-cycle-list-bullet #f))
        (else (org-shiftselect-error))))

;; C-c C-c: 表を揃える, チェックボックス, 見出しのタグ, #+ の行を読み直す
(define-command (org-ctrl-c-ctrl-c)
  (let* ((lines (org-lines))
         (line (org-line lines (org-current-line lines)))
         (item (org-in-item)))
    (cond ((org-table-line? line) (org-table-align))
          ((org-level line) (org-set-tags-command))
          ((regexp-match "^[ \\t]*#\\+" line)
           (org-mode)
           (message "Local setup has been refreshed"))
          (item
           (if (caddr (org-parse-item (org-line lines item)))
               (org-toggle-checkbox)
               (begin (org-fix-bullets lines item) (org-update-statistics))))
          ((regexp-match "\\[[0-9]*(?:%|/[0-9]*)\\]" line) (org-update-statistics))
          (else (error "C-c C-c can do nothing useful here")))))

;; RET: 表なら次の行, リンクの上なら開く (org-return-follows-link), ほかは改行
(define-command (org-return)
  (let* ((lines (org-lines))
         (line (org-line lines (org-current-line lines))))
    (cond ((org-table-line? line) (org-table-next-row))
          ((and org-return-follows-link (org-link-at-point)) (org-open-at-point))
          (else (call-interactively 'newline)))))

;;; リンク

;; 覚えたリンク ((リンク 説明) のリスト. 新しい順)
(define org-stored-links '())

;; ホームを ~ にする
(define (org-abbreviate-file-name file)
  (let ((home (expand-file-name "~")))
    (if (string-prefix? (string-append home "/") file)
        (string-append "~" (substring file (string-length home)))
        file)))

;; ディレクトリ (最後の / まで)
(define (org-file-directory file)
  (let ((m (regexp-match "^(.*/)[^/]*$" file)))
    (if m (cadr m) "./")))

;; ファイルの名前 (ディレクトリと拡張子を除く)
(define (org-file-base file)
  (let ((m (regexp-match "([^/]*?)(?:\\.[^./]*)?$" file)))
    (cadr m)))

;; カーソルの所のリンク ((リンク 説明 始め 終わり). 桁で. 無ければ #f)
(define (org-link-at-point)
  (let* ((lines (org-lines))
         (line (org-line lines (org-current-line lines)))
         (col (org-current-column lines)))
    (let loop ((ms (append (org-all-matches "\\[\\[([^\\]\\n]+)\\](?:\\[([^\\]\\n]+)\\])?\\]" line)
                           (org-all-matches "(?:https?|ftp|mailto|file):[^\\s\\]>)\"']+" line))))
      (cond ((null? ms) #f)
            ((and (<= (car (car (car ms))) col) (< col (cadr (car (car ms)))))
             (let ((m (car ms)))
               (if (> (length m) 1)
                   (list (org-match-string line m 1) (org-match-string line m 2) (car (car m)) (cadr (car m)))
                   (list (org-match-string line m 0) #f (car (car m)) (cadr (car m))))))
            (else (loop (cdr ms)))))))

;; リンクの文字 ([[リンク][説明]])
(define (org-make-link-string link description)
  (if (and description (not (equal? description "")) (not (equal? description link)))
      (format "[[%s][%s]]" link description)
      (format "[[%s]]" link)))

;; C-c l: 今の所へのリンクを覚える
(define-command (org-store-link)
  (let ((file (buffer-file-name)))
    (if (not file)
        (message "Cannot link to a buffer which is not visiting a file")
        (let* ((lines (org-lines))
               (n (org-current-line lines))
               (path (string-append "file:" (org-abbreviate-file-name file)))
               (h (and (regexp-match "\\.org$" file) (org-heading-line lines n)))
               (link
                (if h
                    (let* ((parsed (org-parse-heading (org-line lines h)))
                           (id (org-entry-get lines h "CUSTOM_ID"))
                           (title (org-strip-cookies (or (list-ref parsed 3) ""))))
                      (list (if id (string-append path "::#" id) (string-append path "::*" title)) title))
                    (let ((text (string-trim (org-line lines n))))
                      (list (if (equal? text "") path (string-append path "::" text)) #f)))))
          (set! org-stored-links (cons link (filter (lambda (l) (not (equal? (car l) (car link))))
                                                    org-stored-links)))
          (message "Stored: %s" (or (cadr link) (car link)))
          link))))

;; 題から, 数える印 ([2/3] [66%]) を除く
(define (org-strip-cookies title)
  (let loop ((ms (reverse (org-all-matches " *\\[[0-9]*(?:%|/[0-9]*)\\] *" title))) (s title))
    (if (null? ms)
        (string-trim s)
        (loop (cdr ms) (string-append (substring s 0 (car (car (car ms))))
                                      (if (or (= (car (car (car ms))) 0)
                                              (= (cadr (car (car ms))) (string-length s)))
                                          "" " ")
                                      (substring s (cadr (car (car ms)))))))))

;; C-c C-l: リンクを入れる (リンクの上なら, 変える)
(define-command (org-insert-link)
  (let* ((at (org-link-at-point))
         (link (if at
                   (read-string "Link: " (car at))
                   (completing-read "Link: " (map car org-stored-links))))
         (stored (assoc link org-stored-links))
         (description (read-string "Description: "
                                   (cond (at (or (cadr at) ""))
                                         (stored (or (cadr stored) ""))
                                         (else "")))))
    (when (org-blank? link) (error "No link selected"))
    (set! org-stored-links (filter (lambda (l) (not (equal? (car l) link))) org-stored-links))
    (if at
        (let* ((lines (org-lines))
               (start (org-line-start lines (org-current-line lines)))
               (new (org-make-link-string link description)))
          (org-replace-region (+ start (caddr at)) (+ start (list-ref at 3)) new)
          (goto-char (+ start (caddr at) (string-length new))))
        (insert (org-make-link-string link description)))))

;; C-c C-o: カーソルの所のリンクを開く
(define-command (org-open-at-point)
  (let ((at (org-link-at-point)))
    (if at (org-link-open (car at)) (message "No link found"))))

;; リンクを開く (http などは browse-url, file: はエディタ, ほかはバッファの中を探す)
(define (org-link-open link)
  (cond
   ((regexp-match "^(?:https?|ftp|mailto|news):" link) (browse-url link))
   ((or (string-prefix? "file:" link) (regexp-match "^(?:/|\\./|\\.\\./|~/)" link))
    (let* ((body (if (string-prefix? "file:" link) (substring link 5) link))
           (parts (split-string body "::"))
           (file (car parts))
           (here (buffer-file-name))
           (path (expand-file-name file (if here (org-file-directory here) "."))))
      (if (regexp-match "\\.(?:x?html?|pdf)$" path)
          (browse-url path)
          (begin
            (find-file path)
            (when (and (regexp-match "\\.org$" path) (not (member 'org-find-file-hook find-file-hook)))
              (org-mode))
            (when (> (length parts) 1)
              (org-link-search (string-join (cdr parts) "::")))))))
   ((regexp-match "^[a-z]+:" link) (error "Unknown link type: %s" link))
   (else (org-link-search link))))

;; バッファの中を探す (*見出し, #id, 行の数, /正規表現/, <<目当て>>, 題, 言葉)
(define (org-link-search target)
  (let* ((lines (org-lines))
         (count (org-line-count lines))
         (find (lambda (pred)
                 (let loop ((k 1))
                   (cond ((> k count) #f)
                         ((pred (org-line lines k)) k)
                         (else (loop (+ k 1)))))))
         (title-is (lambda (title)
                     (lambda (line)
                       (let ((parsed (org-parse-heading line)))
                         (and parsed (equal? (org-strip-cookies (or (list-ref parsed 3) "")) title))))))
         (found
          (cond
           ((string-prefix? "*" target) (find (title-is (substring target 1))))
           ((string-prefix? "#" target)
            (let ((h (find (lambda (line) (regexp-match (format "^[ \\t]*:CUSTOM_ID:[ \\t]*%s[ \\t]*$"
                                                                (regexp-quote (substring target 1)))
                                                        line)))))
              (and h (org-heading-line lines h))))
           ((regexp-match "^[0-9]+$" target) (min (string->number target) count))
           ((regexp-match "^/.*/$" target)
            (let ((re (substring target 1 (- (string-length target) 1))))
              (find (lambda (line) (regexp-match re line)))))
           (else
            (or (find (lambda (line) (string-search (string-append "<<" target ">>") line)))
                (find (title-is target))
                (find (lambda (line) (string-search target line))))))))
    (if found
        (begin (goto-char (org-line-start lines found)) (org-reveal (point)))
        (error "No match for fuzzy expression: %s" target))))

;;; アジェンダ

;; アジェンダのバッファの名前
(define org-agenda-buffer-name "*Org Agenda*")
;; アジェンダの行の項目 ((行 ファイル 見出しの行) のリスト)
(define org-agenda-markers '())
;; アジェンダを作り直す手続き (r と g で呼ぶ)
(define org-agenda-redo-command #f)
;; 週のアジェンダの始めの日
(define org-agenda-start-day #f)
;; アジェンダを出す前のバッファ
(define org-agenda-previous-buffer #f)

;; アジェンダのファイル (ディレクトリは, その中の .org のファイル)
(define (org-agenda-file-list)
  (apply append
         (map (lambda (f)
                (let ((path (expand-file-name f)))
                  (if (file-directory? path)
                      (sort (filter (lambda (file)
                                      (not (string-search "/" (substring file (+ (string-length path) 1)))))
                                    (directory-files-recursively path "^[^.#].*\\.org$"))
                            string<?)
                      (list path))))
              (if (string? org-agenda-files) (list org-agenda-files) org-agenda-files))))

;; ファイルの中身 (書き換えたバッファがあれば, そのバッファ)
(define (org-agenda-file-text file)
  (let ((buffer (let loop ((names (buffer-list)))
                  (cond ((null? names) #f)
                        ((equal? (buffer-file-name (car names)) file) (car names))
                        (else (loop (cdr names)))))))
    (if (and buffer (buffer-modified-p buffer))
        (let ((current (buffer-name)))
          (switch-to-buffer buffer)
          (let ((text (buffer-string)))
            (switch-to-buffer current)
            text))
        (if (file-exists? file) (read-file-string file) ""))))

;; ファイルの見出し ((ファイル 行 状態 優先度 題 タグ 分類 予定 締め切り (日付...) 本文) のリスト)
(define (org-agenda-entries file)
  (let* ((lines (split-string (org-agenda-file-text file) "\n"))
         (category-m (regexp-match "(?m)^#\\+CATEGORY:[ \\t]*(\\S+)" (string-join lines "\n")))
         (category (if category-m (cadr category-m) (org-file-base file)))
         (headings (org-headings lines)))
    (map (lambda (h)
           (let* ((n (car h))
                  (parsed (org-parse-heading (org-line lines n)))
                  (end (org-entry-end lines n))
                  (body (org-take (org-drop lines n) (- end n)))
                  (planning (and (not (null? body)) (org-planning-line? (car body)) (org-planning-items (car body))))
                  (get (lambda (k) (let ((found (and planning (assoc k planning)))) (and found (cadr found)))))
                  (text (string-join (cons (org-line lines n)
                                           (if planning (cdr body) body))
                                     "\n"))
                  (stamps (filter (lambda (s) (equal? (car s) "<"))
                                  (filter (lambda (s) s)
                                          (map (lambda (m) (org-parse-timestamp (org-match-string text m 0)))
                                               (org-all-matches org-ts-regexp text)))))
                  (own-category (org-entry-get lines n "CATEGORY")))
             (list file n (cadr parsed) (caddr parsed) (or (list-ref parsed 3) "")
                   (org-heading-tags parsed) (or own-category category)
                   (get "SCHEDULED") (get "DEADLINE") stamps (string-join (cons (org-line lines n) body) "\n"))))
         headings)))

;; 全てのアジェンダのファイルの見出し
(define (org-agenda-all-entries)
  (apply append (map org-agenda-entries (org-agenda-file-list))))

;; 項目の部分
(define (org-entry-file e) (list-ref e 0))
(define (org-entry-line e) (list-ref e 1))
(define (org-entry-todo e) (list-ref e 2))
(define (org-entry-priority e) (list-ref e 3))
(define (org-entry-title e) (list-ref e 4))
(define (org-entry-tags e) (list-ref e 5))
(define (org-entry-category e) (list-ref e 6))

;; 日付が, 繰り返しも含めて, その日に当たれば #t
(define (org-timestamp-on? parsed day)
  (let ((base (cadr parsed))
        (r (org-timestamp-repeater parsed)))
    (cond ((equal? base day) #t)
          ((or (not r) (string<? day base)) #f)
          (else
           (let ((n (cadr r)) (unit (caddr r)) (diff (org-days-between base day))
                 (b (org-date-parts base)) (d (org-date-parts day)))
             (cond ((equal? unit "d") (= 0 (modulo diff n)))
                   ((equal? unit "w") (= 0 (modulo diff (* 7 n))))
                   ((equal? unit "m") (and (= (caddr b) (caddr d))
                                           (= 0 (modulo (+ (* 12 (- (car d) (car b))) (- (cadr d) (cadr b))) n))))
                   ((equal? unit "y") (and (= (caddr b) (caddr d)) (= (cadr b) (cadr d))
                                           (= 0 (modulo (- (car d) (car b)) n))))
                   (else #f)))))))

;; アジェンダの 1 行 (分類, 時刻, 予定の印, 状態, 題, タグ)
(define (org-agenda-line e time extra)
  (let* ((head (string-append "  " (org-pad-right (string-append (org-entry-category e) ":") 12)
                              (if time (org-pad-right time 12) "")
                              (if (equal? extra "") "" (string-append extra " "))
                              (if (org-entry-todo e) (string-append (org-entry-todo e) " ") "")
                              (if (org-entry-priority e) (string-append (org-entry-priority e) " ") "")
                              (org-entry-title e)))
         (tags (org-tags-string (org-entry-tags e))))
    (if tags
        (string-append head (org-spaces (max 1 (- 80 (string-width head) (string-width tags)))) tags)
        head)))

;; 時刻の見せ方 (10:00...... か 10:00-11:00. 時が 1 桁なら, 前に空白)
(define (org-agenda-time parsed)
  (and parsed (caddr parsed)
       (let ((pad (lambda (t) (let ((h (string->number (car (split-string t ":")))))
                                (string-append (if (< h 10) " " "") (format "%d" h) ":" (cadr (split-string t ":")))))))
         (if (list-ref parsed 3)
             (string-append (pad (caddr parsed)) "-" (pad (list-ref parsed 3)))
             (string-append (pad (caddr parsed)) "......")))))

;; その日の行 ((時刻 行 項目 優先度) のリスト)
(define (org-agenda-day-items entries day today)
  (apply append
         (map (lambda (e)
                (let* ((done (org-done-keyword? (org-entry-todo e)))
                       (sched (and (list-ref e 7) (org-parse-timestamp (list-ref e 7))))
                       (dead (and (list-ref e 8) (org-parse-timestamp (list-ref e 8))))
                       ;; (時刻 行 見出し 優先度). 優先度は, 締め切りが近いほど, 予定が過ぎたほど高い
                       (item (lambda (parsed extra bonus)
                               (list (org-agenda-time parsed)
                                     (org-agenda-line e (org-agenda-time parsed) extra) e
                                     (+ (org-agenda-priority e) bonus)))))
                  (append
                   ;; 日付
                   (map (lambda (p) (item p "" 0))
                        (filter (lambda (p) (org-timestamp-on? p day)) (list-ref e 9)))
                   ;; 締め切り
                   (if dead
                       (let ((diff (org-days-between day (cadr dead)))
                             (warning (or (org-timestamp-warning dead) org-deadline-warning-days)))
                         (cond ((org-timestamp-on? dead day) (list (item dead "Deadline: " (- diff))))
                               ((and (equal? day today) (not done) (< diff 0))
                                (list (item #f (format "%s d. ago: " (org-pad-left (format "%d" (- diff)) 2)) (- diff))))
                               ((and (equal? day today) (not done) (> diff 0) (<= diff warning))
                                (list (item #f (format "In %s d.: " (org-pad-left (format "%d" diff) 3)) (- diff))))
                               (else '())))
                       '())
                   ;; 予定
                   (if sched
                       (let ((diff (org-days-between (cadr sched) day)))
                         (cond ((org-timestamp-on? sched day) (list (item sched "Scheduled: " 99)))
                               ((and (equal? day today) (not done) (> diff 0))
                                (list (item #f (format "Sched.%sx: " (org-pad-left (format "%d" diff) 2)) (+ 99 diff))))
                               (else '())))
                       '()))))
              entries)))

;; 見出しの優先度の数 (Emacs の org-get-priority. 一番低いものが 0 で, 1 つ上がるごとに 1000)
(define (org-agenda-priority e)
  (let* ((chars (org-priority-chars))
         (m (regexp-match "[A-Z0-9]" (or (org-entry-priority e) "")))
         (i (or (and m (org-index-of (car m) chars)) (org-index-of org-priority-default chars))))
    (* 1000 (- (length chars) 1 i))))

;; 時刻のある行を先に, 時刻の順に並べる. 同じ時刻と, 時刻の無い行は, 優先度の高い順
;; (Emacs の org-agenda-sorting-strategy の既定 time-up priority-down category-keep)
(define (org-agenda-sort items)
  (let ((by-priority (lambda (a b) (> (list-ref a 3) (list-ref b 3)))))
    (append (sort (filter car items)
                  (lambda (a b) (or (string<? (car a) (car b))
                                    (and (equal? (car a) (car b)) (by-priority a b)))))
            (sort (filter (lambda (i) (not (car i))) items) by-priority))))

;; アジェンダのバッファに出す (rows は (行 項目) のリスト. 項目が #f なら, 見出しの行)
(define (org-agenda-show rows redo)
  (unless (equal? (buffer-name) org-agenda-buffer-name)
    (set! org-agenda-previous-buffer (buffer-name)))
  (switch-to-buffer org-agenda-buffer-name)
  (set! org-agenda-start-day #f)
  (delete-region (point-min) (point-max))
  (insert (string-join (map car rows) "\n") "\n")
  (set-buffer-modified-p #f)
  (set! org-agenda-markers
        (filter (lambda (m) (cadr m))
                (map (lambda (row i) (list (+ i 1) (and (cadr row) (org-entry-file (cadr row)))
                                           (and (cadr row) (org-entry-line (cadr row)))))
                     rows (org-range (length rows)))))
  (set! org-agenda-redo-command redo)
  (org-agenda-mode)
  (goto-char (point-min)))

;; アジェンダのキーと色
(define (org-agenda-mode)
  (for-each (lambda (key) (local-set-key (car key) (cadr key)))
            '(("RET" org-agenda-switch-to) ("SPC" org-agenda-show-and-scroll-up)
              ("TAB" org-agenda-goto) ("n" org-agenda-next-line) ("p" org-agenda-previous-line)
              ("f" org-agenda-later) ("b" org-agenda-earlier) ("." org-agenda-goto-today)
              ("r" org-agenda-redo) ("g" org-agenda-redo) ("t" org-agenda-todo) ("q" org-agenda-quit)))
  (unhighlight-regexp #t)
  (for-each (lambda (k) (highlight-regexp (car k) (cadr k) (caddr k)))
            (append
             (list '("^(?:Week|Day|\\d+-days?)-agenda.*|^Global list of TODO.*|^Headlines with TAGS match.*|^Search words.*" "org-agenda-structure" 0)
                   '("^[A-Z][a-z]+ +\\d+ [A-Z][a-z]+ \\d{4}(?: W\\d{2})?$" "org-agenda-date" 0)
                   (list (format "^%s$" (org-agenda-date-line (org-today))) "org-agenda-date-today" 0)
                   '("^  .{12}(?:[ \\d:.-]{12})?(Scheduled: .*)" "org-scheduled" 1)
                   '("^  .{12}(Sched\\. ?\\d+x: .*)" "org-scheduled-previously" 1)
                   '("^  .{12}(?:[ \\d:.-]{12})?(Deadline: .*|\\d+ d\\. ago: .*)" "org-warning" 1)
                   '("^  .{12}(In +\\d+ d\\.: .*)" "org-upcoming-deadline" 1))
             (let ((todo (string-join (map regexp-quote (filter (lambda (w) (not (org-done-keyword? w)))
                                                                (org-todo-keywords-1)))
                                      "|"))
                   (done (string-join (map regexp-quote (org-done-keywords)) "|")))
               (append (if (equal? todo "") '() (list (list (format "^  .*?\\b(%s)\\b" todo) "org-todo" 1)))
                       (if (equal? done "") '() (list (list (format "^  .*?\\b(%s)\\b" done) "org-done" 1))))))))

;; 日の行 (Monday     26 October 2026 W44)
(define (org-agenda-date-line day)
  (let* ((time (org-date-time day))
         (weekday (format-time-string "%A" time))
         (week (if (equal? (format-time-string "%u" time) "1")
                   (string-append " W" (format-time-string "%V" time))
                   "")))
    (format "%s %s %s %s%s" (org-pad-right weekday 10)
            (org-pad-left (format "%d" (string->number (format-time-string "%d" time))) 2)
            (format-time-string "%B" time) (format-time-string "%Y" time) week)))

;; 週のアジェンダ (始めの日から, org-agenda-span 日)
(define (org-agenda-list-from start)
  (let* ((entries (org-agenda-all-entries))
         (today (org-today))
         (days (map (lambda (i) (org-date-add start i)) (org-range org-agenda-span)))
         (title (format "%s-agenda (W%s):"
                        (if (= org-agenda-span 7) "Week" (if (= org-agenda-span 1) "Day" (format "%d-days" org-agenda-span)))
                        (format-time-string "%V" (org-date-time start)))))
    (org-agenda-show
     (cons (list title #f)
           (apply append
                  (map (lambda (day)
                         (cons (list (org-agenda-date-line day) #f)
                               (map (lambda (i) (list (cadr i) (caddr i)))
                                    (org-agenda-sort (org-agenda-day-items entries day today)))))
                       days)))
     (lambda () (org-agenda-list-from start)))
    ;; 週のアジェンダでだけ, f と b と . が使える
    (set! org-agenda-start-day start)))

;; 週の始め (org-agenda-start-on-weekday の曜日)
(define (org-agenda-week-start day)
  (if org-agenda-start-on-weekday
      (let ((w (string->number (format-time-string "%u" (org-date-time day)))))
        (org-date-add day (- (modulo (- w org-agenda-start-on-weekday) 7))))
      day))

;; 週のアジェンダ (a)
(define-command (org-agenda-list) (org-agenda-list-from (org-agenda-week-start (org-today))))

;; TODO の一覧 (t)
(define-command (org-todo-list)
  (let ((entries (filter (lambda (e) (and (org-entry-todo e) (not (org-done-keyword? (org-entry-todo e)))))
                         (org-agenda-all-entries))))
    (org-agenda-show
     (append (list (list "Global list of TODO items of type: ALL" #f))
             (map (lambda (e) (list (org-agenda-line e #f "") e)) entries))
     org-todo-list)))

;; タグの一覧 (m. a+b は両方, a|b はどちらか, -a は無いもの)
(define-command (org-tags-view)
  (let* ((match (read-string "Match: "))
         (entries (filter (lambda (e) (org-tags-match? match (org-entry-tags e))) (org-agenda-all-entries))))
    (org-agenda-show
     (cons (list (format "Headlines with TAGS match: %s" match) #f)
           (map (lambda (e) (list (org-agenda-line e #f "") e)) entries))
     (lambda () #t))))

;; タグが合えば #t
(define (org-tags-match? match tags)
  (let ((any (split-string match "|")))
    (if (> (length any) 1)
        (if (null? (filter (lambda (m) (org-tags-match? m tags)) any)) #f #t)
        (let loop ((terms (org-all-matches "([+-]?)([^+-]+)" match)))
          (cond ((null? terms) #t)
                (else
                 (let* ((term (car terms))
                        (sign (org-match-string match term 1))
                        (tag (string-trim (org-match-string match term 2)))
                        (has (if (member tag tags) #t #f)))
                   (if (equal? has (not (equal? sign "-"))) (loop (cdr terms)) #f))))))))

;; 言葉の一覧 (s. 全ての言葉を含む見出し. 大文字と小文字は区別しない)
(define-command (org-search-view)
  (let* ((words (map string-downcase (split-string (read-string "Search for: "))))
         (entries (filter (lambda (e)
                            (let ((text (string-downcase (list-ref e 10))))
                              (null? (filter (lambda (w) (not (string-search w text))) words))))
                          (org-agenda-all-entries))))
    (org-agenda-show
     (cons (list (format "Search words: %s" (string-join words " ")) #f)
           (map (lambda (e) (list (org-agenda-line e #f "") e)) entries))
     (lambda () #t))))

;; C-c a: アジェンダを選ぶ
(define-command (org-agenda)
  (let ((answer (completing-read "Press key for an agenda command: "
                                 '("a Agenda for current week" "t List of all TODO entries"
                                   "m Match a TAGS query" "s Search for keywords")
                                 #f #f)))
    (case (string->symbol (car (split-string (string-append answer " "))))
      ((a) (org-agenda-list))
      ((t) (org-todo-list))
      ((m) (org-tags-view))
      ((s) (org-search-view))
      (else (error "Invalid agenda command: %s" answer)))))

;; カーソルの行の項目 ((ファイル 行). 無ければ失敗)
(define (org-agenda-marker)
  (let* ((n (org-current-line (org-lines)))
         (found (assoc n org-agenda-markers)))
    (if found (cdr found) (error "Command not allowed in this line"))))

;; RET: 項目のファイルを開く
(define-command (org-agenda-switch-to)
  (let ((m (org-agenda-marker)))
    (find-file (car m))
    (org-mode)
    (org-goto-heading (org-lines) (cadr m))))

;; SPC と TAB: 項目を, ほかの窓に出す
(define-command (org-agenda-show-and-scroll-up)
  (let ((m (org-agenda-marker)))
    (display-buffer (find-file-noselect (car m)) (cadr m))))
(define-command (org-agenda-goto) (org-agenda-show-and-scroll-up))

(define-command (org-agenda-next-line) (call-interactively 'next-line))
(define-command (org-agenda-previous-line) (call-interactively 'previous-line))

;; f と b: 次と前の週
(define (org-agenda-move days)
  (if org-agenda-start-day
      (org-agenda-list-from (org-date-add org-agenda-start-day days))
      (error "Not allowed in this agenda")))
(define-command (org-agenda-later) (org-agenda-move org-agenda-span))
(define-command (org-agenda-earlier) (org-agenda-move (- org-agenda-span)))
(define-command (org-agenda-goto-today)
  (if org-agenda-start-day (org-agenda-list) (error "Not allowed in this agenda")))

;; r と g: 作り直す
(define-command (org-agenda-redo)
  (let ((n (org-current-line (org-lines))))
    (org-agenda-redo-command)
    (org-goto-line-column (org-lines) (min n (org-line-count (org-lines))) 0)))

;; t: 項目の状態を変える
(define-command (org-agenda-todo)
  (let ((m (org-agenda-marker)))
    (find-file (car m))
    (org-mode)
    (org-goto-heading (org-lines) (cadr m))
    (org-todo)
    (switch-to-buffer org-agenda-buffer-name)
    (org-agenda-redo)))

;; q: アジェンダを消す
(define-command (org-agenda-quit)
  (let ((previous org-agenda-previous-buffer))
    (kill-buffer org-agenda-buffer-name)
    (when (and previous (member previous (buffer-list)))
      (switch-to-buffer previous))))

;;; キャプチャ

;; キャプチャのバッファの名前
(define org-capture-buffer-name "*Capture*")
;; 書いているキャプチャ ((テンプレート 元のバッファ 元の位置))
(define org-capture-current #f)

;; テンプレート (無ければ, 既定のもの)
(define (org-capture-templates-list)
  (if (null? org-capture-templates)
      '(("t" "Task" entry (file+headline "" "Tasks") "* TODO %?\n  %u\n  %a"))
      org-capture-templates))

;; 行き先のファイル ("" は org-default-notes-file. 相対なら org-directory から)
(define (org-capture-expand-file file)
  (cond ((or (not file) (equal? file "")) (expand-file-name org-default-notes-file))
        ((regexp-match "^(?:/|~)" file) (expand-file-name file))
        (else (expand-file-name file org-directory))))

;; テンプレートを埋める ((文 カーソルの位置))
(define (org-capture-fill-template template annotation file)
  (let loop ((rest template) (result "") (cursor #f))
    (let ((m (regexp-match-positions "%(\\?|[tTuUaAilfFk%]|\\^\\{[^}]*\\}|\\[[^\\]]*\\]|\\([^)]*\\))" rest)))
      (if (not m)
          (list (string-append result rest) (or cursor (+ (string-length result) (string-length rest))))
          (let* ((start (car (car m)))
                 (end (cadr (car m)))
                 (code (substring rest (+ start 1) end))
                 (before (string-append result (substring rest 0 start)))
                 (value
                  (cond
                   ((equal? code "?") "")
                   ((equal? code "t") (org-make-timestamp (org-today) #f #t))
                   ((equal? code "T") (org-make-timestamp (org-today) (format-time-string "%H:%M") #t))
                   ((equal? code "u") (org-make-timestamp (org-today) #f #f))
                   ((equal? code "U") (org-now-inactive))
                   ((member code '("a" "A" "l")) annotation)
                   ((equal? code "i") "")
                   ((equal? code "k") "")
                   ((equal? code "f") (if file (car (regexp-match "[^/]*$" file)) ""))
                   ((equal? code "F") (or file ""))
                   ((equal? code "%") "%")
                   ((string-prefix? "^{" code)
                    (let* ((spec (split-string (substring code 2 (- (string-length code) 1)) "|"))
                           (prompt (car spec)))
                      (if (null? (cdr spec))
                          (read-string (string-append prompt ": "))
                          (completing-read (format "%s [%s]: " prompt (cadr spec)) (cdr spec) #f #f #f #f (cadr spec)))))
                   ((string-prefix? "[" code)
                    (let ((path (expand-file-name (substring code 1 (- (string-length code) 1)))))
                      (if (file-exists? path) (read-file-string path) "")))
                   (else (format "%s" (eval (read code)))))))
            (loop (substring rest end) (string-append before value)
                  (or cursor (and (equal? code "?") (string-length before)))))))))

;; C-c c: テンプレートを選び, *Capture* で書く
(define-command (org-capture)
  (let* ((templates (org-capture-templates-list))
         (answer (completing-read "Select a capture template: "
                                  (map (lambda (t) (string-append (car t) " " (cadr t))) templates)
                                  #f #f))
         (key (car (split-string (string-append answer " "))))
         (template (or (assoc key templates) (error "No capture template referred to by \"%s\" keys" key)))
         (link (and (buffer-file-name) (org-store-link)))
         (annotation (if link (org-make-link-string (car link) (cadr link)) ""))
         (filled (org-capture-fill-template (list-ref template 4) annotation (buffer-file-name))))
    (set! org-capture-current (list template (buffer-name) (point)))
    (switch-to-buffer org-capture-buffer-name)
    (delete-region (point-min) (point-max))
    (insert (car filled))
    (goto-char (+ 1 (cadr filled)))
    (set-buffer-modified-p #f)
    (local-set-key "C-c C-c" 'org-capture-finalize)
    (local-set-key "C-c C-k" 'org-capture-kill)
    (org-fontify)
    (message "Capture buffer.  Finish `C-c C-c', abort `C-c C-k'.")))

;; キャプチャの文を, 行き先に入れる (行き先のバッファで呼ぶ)
(define (org-capture-place type target text)
  (let* ((lines (org-lines))
         (headline (and (equal? (car target) 'file+headline) (caddr target)))
         (h (and headline
                 (let loop ((hs (org-headings lines)))
                   (cond ((null? hs) #f)
                         ((equal? (org-strip-cookies (or (list-ref (org-parse-heading (org-line lines (car (car hs)))) 3) ""))
                                  headline)
                          (car (car hs)))
                         (else (loop (cdr hs))))))))
    ;; 見出しが無ければ, 終わりに作る
    (when (and headline (not h))
      (goto-char (point-max))
      (unless (or (= (point) 1) (equal? (org-last lines) "")) (insert "\n"))
      (insert "* " headline "\n")
      (set! lines (org-lines))
      (set! h (car (org-last (org-headings lines)))))
    (let* ((level (if h (+ (org-level (org-line lines h)) 1) 1))
           (body (string-trim (if (equal? type 'entry) (org-capture-adjust-level text level) text)))
           (body (cond ((equal? type 'item) (if (org-parse-item body) body (string-append "- " body)))
                       ((equal? type 'checkitem) (if (org-parse-item body) body (string-append "- [ ] " body)))
                       (else body)))
           (last (if h (org-subtree-end lines h) (org-line-count lines))))
      (if (or (= last 0) (and (= last 1) (equal? (org-line lines 1) "")))
          (begin (delete-region (point-min) (point-max)) (insert body "\n"))
          (org-insert-lines-after lines last (list body)))
      (when (> (point-max) 1)
        (goto-char (point-max))
        (unless (equal? (buffer-substring (- (point) 1) (point)) "\n") (insert "\n"))))))

;; 見出しの深さを, level に合わせる
(define (org-capture-adjust-level text level)
  (let* ((lines (split-string text "\n"))
         (levels (filter (lambda (l) l) (map org-level lines))))
    (if (null? levels)
        text
        (let ((delta (- level (apply min levels))))
          (string-join (map (lambda (line)
                              (let ((l (org-level line)))
                                (if l (string-append (org-repeat "*" (+ l delta)) (substring line l)) line)))
                            lines)
                       "\n")))))

;; C-c C-c: キャプチャを終え, 行き先に足して保存する
(define-command (org-capture-finalize)
  (unless org-capture-current (error "This does not seem to be a capture buffer for Org mode"))
  (let* ((template (car org-capture-current))
         (previous (cadr org-capture-current))
         (text (buffer-string))
         (target (list-ref template 3))
         (file (org-capture-expand-file (cadr target))))
    (kill-buffer org-capture-buffer-name)
    (set! org-capture-current #f)
    (find-file file)
    (org-capture-place (list-ref template 2) target text)
    (call-interactively 'save-buffer)
    (when (member previous (buffer-list)) (switch-to-buffer previous))
    (run-hooks 'org-capture-after-finalize-hook)
    (message "Captured to %s" (org-abbreviate-file-name file))))

;; C-c C-k: キャプチャを止める
(define-command (org-capture-kill)
  (unless org-capture-current (error "This does not seem to be a capture buffer for Org mode"))
  (let ((previous (cadr org-capture-current)))
    (kill-buffer org-capture-buffer-name)
    (set! org-capture-current #f)
    (when (and previous (member previous (buffer-list))) (switch-to-buffer previous))
    (message "Capture aborted")))

;;; バッファ

;; Org のバッファ (.org のファイル)
(define (org-buffer-list)
  (filter (lambda (name)
            (let ((file (buffer-file-name name)))
              (and file (regexp-match "\\.org$" file))))
          (buffer-list)))

;; Org のバッファに移る
(define-command (org-switchb)
  (let ((buffers (org-buffer-list)))
    (if (null? buffers)
        (error "No Org buffers")
        (switch-to-buffer (completing-read "Org buffer: " buffers #f #t)))))

;;; 始め

(add-hook 'find-file-hook 'org-find-file-hook)
;; Org の手引きの勧めるキー (Emacs は割り当てない)
(global-set-key "C-c l" 'org-store-link)
(global-set-key "C-c a" 'org-agenda)
(global-set-key "C-c c" 'org-capture)

(provide 'org)
