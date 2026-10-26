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

(define org-directory "~/org" "Directory where Org files are kept.")
(define org-default-notes-file "~/.notes" "Default file for capture.")
(define org-agenda-files '()
  "Files shown in the agenda.
A directory means the .org files in it.")
(define org-todo-keywords '((sequence "TODO" "DONE"))
  "TODO states, written like ((sequence \"TODO\" \"|\" \"DONE\")).
The words after | are done states; without |, the last word is the done state.
A key to select the state can be written in parentheses, like \"TODO(t)\".")
(define org-log-done #f
  "Whether to record the time when an entry is marked done.
If it is 'time or 'note, write CLOSED: [DATE]; if #f, write nothing.")
(define org-log-repeat 'time
  "Whether to record the time when a repeating entry is marked done.
If #f, write nothing.")
(define org-tags-column -77
  "Column to align tags to.
If negative, align the end of the tags to its absolute value.")
(define org-priority-highest "A" "Highest priority.")
(define org-priority-lowest "C" "Lowest priority.")
(define org-priority-default "B" "Priority of an entry without one.")
(define org-capture-templates '()
  "Capture templates, a list of (KEY DESCRIPTION TYPE (TARGET) TEMPLATE).
If empty, use the default one.
TYPE is entry (heading), item (list item), checkitem (checkbox) or plain (text).
TARGET is (file \"FILE\") or (file+headline \"FILE\" \"HEADING\"); \"\" means `org-default-notes-file'.")
(define org-hide-leading-stars #f "If true, show only the last star of a heading.")
(define org-return-follows-link #f "If true, RET opens the link at point.")
(define org-deadline-warning-days 14
  "Number of days before a deadline to show it in today's agenda.")
(define org-agenda-span 7 "Number of days in the agenda.")
(define org-agenda-start-on-weekday 1
  "Weekday the agenda starts on (1 is Monday).
If #f, start from today.")
(define org-mode-hook '() "Procedures called when Org mode is turned on.")
(define org-capture-after-finalize-hook '() "Procedures called when a capture is finished.")

;;; 文字列とリスト

(define (org-take items n)
  "Return the first N elements of ITEMS."
  (if (or (<= n 0) (null? items)) '() (cons (car items) (org-take (cdr items) (- n 1)))))

(define (org-drop items n)
  "Return ITEMS without the first N elements."
  (if (or (<= n 0) (null? items)) items (org-drop (cdr items) (- n 1))))

(define (org-last items)
  "Return the last element of ITEMS."
  (list-ref items (- (length items) 1)))

(define (org-index-of x items)
  "Return the index of X in ITEMS, counted from 0, or #f if it is not found."
  (let loop ((rest items) (i 0))
    (cond ((null? rest) #f)
          ((equal? (car rest) x) i)
          (else (loop (cdr rest) (+ i 1))))))

(define (org-unique items)
  "Return ITEMS without duplicates, keeping the first ones."
  (let loop ((rest items) (seen '()))
    (cond ((null? rest) (reverse seen))
          ((member (car rest) seen) (loop (cdr rest) seen))
          (else (loop (cdr rest) (cons (car rest) seen))))))

(define (org-range n)
  "Return the list of numbers from 0 to N - 1."
  (let loop ((i (- n 1)) (result '()))
    (if (< i 0) result (loop (- i 1) (cons i result)))))

(define (org-repeat s n)
  "Return the string S repeated N times."
  (if (<= n 0) "" (string-append s (org-repeat s (- n 1)))))

(define (org-spaces n)
  "Return a string of N spaces."
  (org-repeat " " n))

(define (org-pad-right s width)
  "Pad S with spaces on the right to WIDTH.
If S is longer, return it as it is."
  (string-append s (org-spaces (- width (string-width s)))))

(define (org-pad-left s width)
  "Pad S with spaces on the left to WIDTH."
  (string-append (org-spaces (- width (string-width s))) s))

(define (org-2d n)
  "Return N as two digits, padded with 0."
  (if (< n 10) (format "0%d" n) (format "%d" n)))

(define (org-blank? s)
  "Return #t if S consists only of whitespace."
  (if (regexp-match "^[ \\t]*$" s) #t #f))

(define (org-indentation line)
  "Return the width of the indentation of LINE."
  (string-length (car (regexp-match "^[ \\t]*" line))))

(define (org-all-matches regexp s)
  "Return all matches of REGEXP in S.
Each match is a list with (START END) or #f for each group."
  (let loop ((start 0) (found '()))
    (let ((m (and (<= start (string-length s)) (regexp-match-positions regexp s start))))
      (if (not m)
          (reverse found)
          (let ((from (car (car m))) (to (cadr (car m))))
            (loop (if (= to from) (+ to 1) to) (cons m found)))))))

(define (org-match-string s m n)
  "Return the string of group N of the match M in S, or #f if the group did not match."
  (let ((p (list-ref m n)))
    (and p (substring s (car p) (cadr p)))))

(define (org-replace-all s from to)
  "Replace all occurrences of FROM in S with TO."
  (string-join (split-string s from) to))

(define (org-char s n)
  "Return the Nth character of S as a string, counted from 0."
  (substring s n (+ n 1)))

;;; バッファの行

(define (org-lines)
  "Return the lines of the buffer.
If the buffer ends with a newline, the last line is empty."
  (split-string (buffer-string) "\n"))

(define (org-line-count lines)
  "Return the number of LINES, not counting the empty line after the last newline."
  (let ((n (length lines)))
    (if (and (> n 1) (equal? (org-last lines) "")) (- n 1) n)))

(define (org-line lines n)
  "Return line N of LINES, counted from 1."
  (list-ref lines (- n 1)))

(define (org-line-start lines n)
  "Return the position of the start of line N."
  (let loop ((i 1) (rest lines) (pos 1))
    (if (or (>= i n) (null? rest))
        pos
        (loop (+ i 1) (cdr rest) (+ pos (string-length (car rest)) 1)))))

(define (org-line-end lines n)
  "Return the position of the end of line N, before the newline."
  (+ (org-line-start lines n) (string-length (org-line lines n))))

(define (org-pos-line lines pos)
  "Return the line and column of POS as (LINE COLUMN).
The column is counted from 0."
  (let loop ((n 1) (rest lines) (start 1))
    (let ((end (+ start (string-length (car rest)))))
      (if (or (<= pos end) (null? (cdr rest)))
          (list n (- pos start))
          (loop (+ n 1) (cdr rest) (+ end 1))))))

(define (org-current-line lines)
  "Return the line of point."
  (car (org-pos-line lines (point))))

(define (org-current-column lines)
  "Return the column of point, counted from 0."
  (cadr (org-pos-line lines (point))))

(define (org-goto-line-column lines n col)
  "Move to column COL of line N."
  (goto-char (+ (org-line-start lines n) (min col (string-length (org-line lines n))))))

(define (org-replace-region start end new)
  "Replace the text between START and END with NEW.
If point is after the replaced text, it moves with the text."
  (let ((p (point)))
    (unless (equal? (buffer-substring start end) new)
      (delete-region start end)
      (goto-char start)
      (insert new)
      (goto-char (cond ((<= p start) p)
                       ((>= p end) (+ p (- (string-length new) (- end start))))
                       (else (min p (+ start (string-length new)))))))))

(define (org-replace-lines lines from to new)
  "Replace lines FROM to TO with the list of lines NEW."
  (org-replace-region (org-line-start lines from) (org-line-end lines to) (string-join new "\n")))

(define (org-set-line lines n new)
  "Replace line N with NEW."
  (org-replace-lines lines n n (list new)))

(define (org-set-line-changed lines n new)
  "Replace line N with NEW, changing only the part that differs.
Point moves with the unchanged text."
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

(define (org-insert-lines-after lines n new)
  "Insert the lines NEW after line N."
  (let ((end (org-line-end lines n)))
    (org-replace-region end end (string-append "\n" (string-join new "\n")))))

(define (org-delete-lines lines from to)
  "Delete lines FROM to TO, including the newlines."
  (let ((start (org-line-start lines from))
        (count (length lines)))
    (cond ((< to count) (org-replace-region start (org-line-start lines (+ to 1)) ""))
          ((> from 1) (org-replace-region (- start 1) (org-line-end lines to) ""))
          (else (org-replace-region start (org-line-end lines to) "")))))

;;; 見出し

(define (org-level line)
  "Return the level of the heading LINE, or #f if it is not a heading."
  (let ((m (regexp-match "^(\\*+)(?: |$)" line)))
    (and m (string-length (cadr m)))))

(define (org-headings lines)
  "Return the headings of LINES as a list of (LINE LEVEL)."
  (let loop ((rest lines) (n 1) (found '()))
    (if (null? rest)
        (reverse found)
        (let ((level (org-level (car rest))))
          (loop (cdr rest) (+ n 1) (if level (cons (list n level) found) found))))))

(define (org-heading-line lines n)
  "Return the nearest heading line at or before line N, or #f if there is none."
  (let loop ((headings (org-headings lines)) (found #f))
    (cond ((or (null? headings) (> (car (car headings)) n)) found)
          (else (loop (cdr headings) (car (car headings)))))))

(define (org-subtree-end lines n)
  "Return the last line of the subtree of the heading at line N."
  (let ((level (org-level (org-line lines n))))
    (let loop ((headings (org-headings lines)))
      (cond ((null? headings) (org-line-count lines))
            ((and (> (car (car headings)) n) (<= (cadr (car headings)) level))
             (- (car (car headings)) 1))
            (else (loop (cdr headings)))))))

(define (org-entry-end lines n)
  "Return the last line of the body of the heading at line N, before the next heading."
  (let loop ((headings (org-headings lines)))
    (cond ((null? headings) (org-line-count lines))
          ((> (car (car headings)) n) (- (car (car headings)) 1))
          (else (loop (cdr headings))))))

(define (org-back-to-heading lines)
  "Return the heading line of point.
Signal an error if there is none."
  (or (org-heading-line lines (org-current-line lines))
      (error "Before first headline at position %d in buffer %s" (point) (buffer-name))))

(define (org-at-heading-p)
  "Return #t if point is on a heading."
  (let ((lines (org-lines)))
    (if (org-level (org-line lines (org-current-line lines))) #t #f)))

(define (org-todo-regexp)
  "Return the regexp of the TODO keywords, in a group."
  (let ((words (org-todo-keywords-1)))
    (if (null? words) "[^\\s\\S]" (string-join (map regexp-quote words) "|"))))

(define (org-parse-heading line)
  "Split the heading LINE into (STARS TODO PRIORITY TITLE TAGS).
Missing parts are #f. Return #f if LINE is not a heading."
  (let ((m (regexp-match
            (string-append "^(\\*+)(?: +(" (org-todo-regexp) "))?(?: +(\\[#.\\]))?"
                           "(?: +(.*?))??(?:[ \\t]+(:[\\w@#%:]+:))?[ \\t]*$")
            line)))
    (and m (org-level line) (cdr m))))

(define (org-make-heading stars todo priority title tags)
  "Return a heading line made of STARS, TODO, PRIORITY, TITLE and TAGS, with the tags aligned."
  (let ((head (string-join (filter (lambda (x) (and x (not (equal? x ""))))
                                   (list stars todo priority title))
                           " ")))
    (if tags (org-tags-line head tags) head)))

(define (org-tags-line head tags)
  "Return HEAD followed by TAGS aligned to `org-tags-column'."
  (let* ((width (string-width head))
         (column (if (< org-tags-column 0)
                     (- (- org-tags-column) (string-width tags))
                     org-tags-column)))
    (string-append head (org-spaces (max 1 (- column width))) tags)))

(define (org-heading-tags parsed)
  "Return the tags of the parsed heading PARSED as a list."
  (let ((tags (list-ref parsed 4)))
    (if tags (filter (lambda (t) (not (equal? t ""))) (split-string tags ":")) '())))

(define (org-tags-string tags)
  "Return TAGS as a string like :a:b:, or #f if TAGS is empty."
  (if (null? tags) #f (string-append ":" (string-join tags ":") ":")))

;;; TODO のキーワード

(define (org-keyword-name word)
  "Return the name of the keyword WORD: \"TODO\" for \"TODO(t)\"."
  (car (split-string word "(")))

(define (org-keyword-key word)
  "Return the key to select the keyword WORD: \"t\" for \"TODO(t)\".
Return #f if there is none."
  (let ((m (regexp-match "\\(([^@!/)])" word)))
    (and m (cadr m))))

(define (org-todo-sets)
  "Return the sets of TODO keywords as a list of ((NOT-DONE...) (DONE...))."
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

(define (org-todo-keywords-1)
  "Return all the TODO keywords."
  (org-unique (apply append (map (lambda (set) (append (car set) (cadr set))) (org-todo-sets)))))

(define (org-done-keywords)
  "Return the done keywords."
  (apply append (map cadr (org-todo-sets))))

(define (org-done-keyword? word)
  "Return #t if WORD is a done keyword."
  (if (and word (member word (org-done-keywords))) #t #f))

(define (org-todo-set word)
  "Return the set of keywords containing WORD, or the first set if there is none."
  (let loop ((sets (org-todo-sets)))
    (cond ((null? sets) (car (org-todo-sets)))
          ((member word (append (car (car sets)) (cadr (car sets)))) (car sets))
          (else (loop (cdr sets))))))

(define (org-todo-keys)
  "Return the keys and keywords as a list of (KEY KEYWORD)."
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

(define (org-font-lock-keywords)
  "Return the regexps and faces for highlighting, as a list of (REGEXP FACE GROUP).
The regexps are JavaScript regexps of the editor, matched line by line.
Later ones are drawn over earlier ones."
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

(define (org-fontify)
  "Highlight the buffer."
  (unhighlight-regexp #t)
  (for-each (lambda (k) (highlight-regexp (car k) (cadr k) (caddr k))) (org-font-lock-keywords)))

;;; Org mode

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
    ("C-c C-o" org-open-at-point)) "Key bindings of Org mode.")

(define-command (org-mode)
  "Turn on Org mode: set the keys and highlighting, and read #+STARTUP."
  (for-each (lambda (key) (local-set-key (car key) (cadr key))) org-mode-keys)
  (org-fontify)
  (org-set-startup-visibility)
  (run-hooks 'org-mode-hook))

(define (org-find-file-hook)
  "Turn on Org mode if the visited file is an .org file.
This is called from `find-file-hook'."
  (let ((file (buffer-file-name)))
    (when (and file (regexp-match "\\.org$" file))
      (org-mode))))

(define (org-set-startup-visibility)
  "Set the visibility by #+STARTUP: (overview, content or showall)."
  (let ((m (regexp-match "(?mi)^#\\+STARTUP:.*\\b(overview|fold|content|showall|showeverything)\\b"
                         (buffer-string))))
    (when m
      (let ((how (string-downcase (cadr m))))
        (cond ((member how '("overview" "fold")) (org-cycle-overview (org-lines)))
              ((equal? how "content") (org-cycle-content (org-lines))))))))

;;; 開け閉め

(define org-cycle-last #f
  "Last cycling state, as (BUFFER POINT STATUS).
Whether the key was pressed again is decided by whether point has moved.")

(define (org-cycle-repeated? status)
  "Return #t if the key is pressed again and the previous state was STATUS."
  (and org-cycle-last
       (equal? (org-take org-cycle-last 2) (list (buffer-name) (point)))
       (eq? (caddr org-cycle-last) status)))

(define (org-cycle-remember status)
  "Remember STATUS as the last cycling state."
  (set! org-cycle-last (list (buffer-name) (point) status)))

(define (org-fold-end lines to)
  "Return the end of the region to hide: the end of line TO.
On the last line, it is before the last newline."
  (org-line-end lines to))

(define (org-fold-showing lines shown last)
  "Show the headings SHOWN (a list of lines), hiding the body of each one
up to the next shown heading (up to line LAST)."
  (let loop ((rest shown))
    (unless (null? rest)
      (let* ((n (car rest))
             (to (if (null? (cdr rest)) last (- (cadr rest) 1)))
             (from (org-line-end lines n))
             (end (org-fold-end lines to)))
        (when (> end from) (outline-flag-region from end #t))
        (loop (cdr rest))))))

(define (org-cycle-local lines n)
  "Cycle the subtree of the heading at line N.
The order is EMPTY ENTRY, CHILDREN, SUBTREE and FOLDED."
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

(define (org-cycle-overview lines)
  "Show only the top-level headings."
  (let* ((headings (org-headings lines))
         (top (if (null? headings) 1 (apply min (map cadr headings)))))
    (outline-flag-region 1 (point-max) #f)
    (org-fold-showing lines (map car (filter (lambda (h) (<= (cadr h) top)) headings))
                      (org-line-count lines))))

(define (org-cycle-content lines)
  "Show all the headings, hiding the bodies."
  (outline-flag-region 1 (point-max) #f)
  (org-fold-showing lines (map car (org-headings lines)) (org-line-count lines)))

(define-command (org-global-cycle)
  "Cycle the whole buffer: OVERVIEW, CONTENTS and SHOW ALL."
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

(define-command (org-cycle)
  "Move to the next table field, cycle the heading, or indent.
In a table, move to the next field; on a heading, cycle its subtree; otherwise indent."
  (let* ((lines (org-lines))
         (n (org-current-line lines)))
    (cond ((org-table-line? (org-line lines n)) (org-table-next-field))
          ((org-level (org-line lines n)) (org-cycle-local lines n))
          (else (call-interactively 'indent-for-tab-command)))))

(define-command (org-shifttab)
  "Move to the previous table field, or cycle the whole buffer."
  (let ((lines (org-lines)))
    (if (org-table-line? (org-line lines (org-current-line lines)))
        (org-table-previous-field)
        (org-global-cycle))))

(define (org-reveal pos)
  "Show POS if it is hidden."
  (when (outline-invisible-p pos)
    (outline-flag-region (max 1 (- pos 1)) (+ pos 1) #f)))

;;; 見出しを作る

(define (org-heading-title-range line)
  "Return the range of the title in the heading LINE as (START END), or #f if there is no title."
  (let ((m (regexp-match-positions
            (string-append "^(\\*+)(?: +(" (org-todo-regexp) "))?(?: +(\\[#.\\]))?"
                           "(?: +(.*?))??(?:[ \\t]+(:[\\w@#%:]+:))?[ \\t]*$")
            line)))
    (and m (list-ref m 4))))

(define (org-new-heading-todo lines n todo)
  "Return the TODO state of a new heading after line N.
If TODO is true, it is the not-done state of the previous heading, or the first state."
  (and todo
       (let* ((h (org-heading-line lines n))
              (current (and h (cadr (org-parse-heading (org-line lines h))))))
         (if (and current (not (org-done-keyword? current)))
             current
             (car (org-todo-keywords-1))))))

(define (org-insert-heading-1 todo)
  "Insert a heading (M-RET).
If TODO is true, insert a TODO heading."
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

(define (org-insert-heading-after-subtree todo)
  "Insert a heading after the current subtree (C-RET).
If TODO is true, insert a TODO heading."
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

(define-command (org-insert-heading)
  "Insert a heading, even in a list item."
  (org-insert-heading-1 #f))
(define-command (org-insert-heading-respect-content)
  "Insert a heading after the current subtree."
  (org-insert-heading-after-subtree #f))
(define-command (org-insert-todo-heading-respect-content)
  "Insert a TODO heading after the current subtree."
  (org-insert-heading-after-subtree #t))

(define-command (org-meta-return)
  "Insert a list item or a heading.
In a table, do nothing; in a list, insert an item; otherwise insert a heading."
  (cond ((org-in-item) (org-insert-item #f))
        (else (org-insert-heading-1 #f))))

(define-command (org-insert-todo-heading)
  "Insert a TODO heading, or a checkbox item in a list."
  (if (org-in-item) (org-insert-item #t) (org-insert-heading-1 #t)))

;;; 見出しを動かす

(define (org-shift-headings lines from to delta)
  "Change the level of the headings from line FROM to line TO by DELTA."
  (when (and (< delta 0) (= (org-level (org-line lines from)) 1))
    (error "Cannot promote to level 0.  UNDO to recover if necessary"))
  (for-each (lambda (n) (when (org-level (org-line lines n)) (org-shift-heading n delta)))
            (map (lambda (i) (+ from i)) (org-range (+ (- to from) 1)))))

(define (org-shift-heading n delta)
  "Change the level of the heading at line N by DELTA.
Add or remove stars and realign the tags; point moves with the text."
  (let* ((lines (org-lines))
         (end (+ (org-line-start lines n) (org-level (org-line lines n)))))
    (if (> delta 0)
        (org-replace-region end end "*")
        (org-replace-region (- end 1) end ""))
    (let* ((lines (org-lines))
           (parsed (org-parse-heading (org-line lines n))))
      (when (list-ref parsed 4)
        (org-set-line-changed lines n (apply org-make-heading parsed))))))

(define (org-promote-1 delta subtree)
  "Promote or demote the heading by DELTA.
If SUBTREE is true, change the whole subtree."
  (let* ((lines (org-lines))
         (n (org-back-to-heading lines)))
    (org-shift-headings lines n (if subtree (org-subtree-end lines n) n) delta)))

(define-command (org-do-promote)
  "Promote the current heading."
  (org-promote-1 -1 #f))
(define-command (org-do-demote)
  "Demote the current heading."
  (org-promote-1 1 #f))
(define-command (org-promote-subtree)
  "Promote the current subtree."
  (org-promote-1 -1 #t))
(define-command (org-demote-subtree)
  "Demote the current subtree."
  (org-promote-1 1 #t))

(define (org-sibling lines n forward)
  "Return the line of the previous or next heading at the same level as line N.
FORWARD selects the next one. Return #f if it would cross the parent."
  (let ((level (org-level (org-line lines n)))
        (headings (if forward
                      (filter (lambda (h) (> (car h) n)) (org-headings lines))
                      (reverse (filter (lambda (h) (< (car h) n)) (org-headings lines))))))
    (let loop ((rest headings))
      (cond ((null? rest) #f)
            ((< (cadr (car rest)) level) #f)
            ((= (cadr (car rest)) level) (car (car rest)))
            (else (loop (cdr rest)))))))

(define (org-move-subtree forward)
  "Swap the current subtree with the previous or next one at the same level.
FORWARD selects the next one."
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

(define-command (org-move-subtree-up)
  "Move the current subtree up past the previous one."
  (org-move-subtree #f))
(define-command (org-move-subtree-down)
  "Move the current subtree down past the next one."
  (org-move-subtree #t))

(define (org-drag-line forward)
  "Swap the current line with the previous or next line.
FORWARD selects the next one."
  (let* ((lines (org-lines))
         (n (org-current-line lines))
         (m (if forward (+ n 1) (- n 1)))
         (col (org-current-column lines)))
    (when (or (< m 1) (> m (org-line-count lines)))
      (error (if forward "Cannot move further down" "Cannot move further up")))
    (let ((from (min n m)) (to (max n m)))
      (org-replace-lines lines from to (list (org-line lines to) (org-line lines from)))
      (org-goto-line-column (org-lines) m col))))

(define-command (org-ctrl-c-star)
  "Turn a heading into text, or an item or text into a heading.
Point moves with the text."
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

(define (org-goto-heading lines n)
  "Move to the heading at line N, showing it."
  (goto-char (org-line-start lines n))
  (org-reveal (point)))

(define (org-visible-heading forward)
  "Move to the next or previous visible heading.
FORWARD selects the next one. Without one, move to the end or start of the buffer."
  (let* ((lines (org-lines))
         (n (org-current-line lines))
         (candidates (filter (lambda (h)
                               (and (if forward (> (car h) n) (< (car h) n))
                                    (not (outline-invisible-p (org-line-start lines (car h))))))
                             (org-headings lines))))
    (cond ((null? candidates) (goto-char (if forward (point-max) (point-min))))
          (forward (goto-char (org-line-start lines (car (car candidates)))))
          (else (goto-char (org-line-start lines (car (org-last candidates))))))))

(define-command (org-next-visible-heading)
  "Move to the next visible heading."
  (org-visible-heading #t))
(define-command (org-previous-visible-heading)
  "Move to the previous visible heading."
  (org-visible-heading #f))

(define (org-heading-same-level forward)
  "Move to the next or previous heading at the same level, within the parent.
FORWARD selects the next one. Without one, do not move."
  (let* ((lines (org-lines))
         (n (org-current-line lines))
         (h (org-back-to-heading lines)))
    (if (and (not forward) (not (= h n)))
        (goto-char (org-line-start lines h))
        (let ((sibling (org-sibling lines h forward)))
          (when sibling (goto-char (org-line-start lines sibling)))))))

(define-command (org-forward-heading-same-level)
  "Move to the next heading at the same level."
  (org-heading-same-level #t))
(define-command (org-backward-heading-same-level)
  "Move to the previous heading at the same level."
  (org-heading-same-level #f))

(define (org-parent lines n)
  "Return the line of the parent heading of line N, or #f if there is none."
  (let ((level (org-level (org-line lines n))))
    (let loop ((rest (reverse (filter (lambda (h) (< (car h) n)) (org-headings lines)))))
      (cond ((null? rest) #f)
            ((< (cadr (car rest)) level) (car (car rest)))
            (else (loop (cdr rest)))))))

(define-command (outline-up-heading)
  "Move to the parent heading."
  (let* ((lines (org-lines))
         (parent (org-parent lines (org-back-to-heading lines))))
    (if parent
        (goto-char (org-line-start lines parent))
        (error "Already at top level of the outline"))))

;;; リスト

(define (org-parse-item line)
  "Split the list item LINE into (INDENT BULLET CHECKBOX TEXT).
CHECKBOX is #f if there is none. Return #f if LINE is not an item."
  (let ((m (regexp-match "^([ \\t]*)([-+*]|[0-9]+[.)])(?:[ \\t]+|$)(?:(\\[[ X-]\\])(?:[ \\t]+|$))?(.*)$" line)))
    (and m
         (not (and (equal? (cadr m) "") (equal? (caddr m) "*")))
         (cdr m))))

(define (org-make-item indent bullet box text)
  "Return an item line made of INDENT, BULLET, BOX and TEXT."
  (string-append indent bullet " " (if box (string-append box " ") "") text))

(define (org-ordered-bullet? bullet)
  "Return #t if BULLET is a number."
  (if (regexp-match "^[0-9]" bullet) #t #f))

(define (org-item-end lines n)
  "Return the last line of the item at line N.
It includes the child items and continuation lines, but not trailing empty lines."
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

(define (org-item-at lines n)
  "Return the line of the item containing line N, or #f if there is none."
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

(define (org-in-item)
  "Return the line of the item at point, or #f if there is none."
  (let ((lines (org-lines)))
    (org-item-at lines (org-current-line lines))))

(define (org-item-sibling lines n forward)
  "Return the previous or next item at the same level as line N, or #f if there is none.
FORWARD selects the next one."
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

(define (org-item-siblings lines n)
  "Return the lines of the items at the same level as line N in the list, from the first."
  (let ((first (let loop ((k n))
                 (let ((previous (org-item-sibling lines k #f)))
                   (if previous (loop previous) k)))))
    (let loop ((k first) (found '()))
      (if k (loop (org-item-sibling lines k #t) (cons k found)) (reverse found)))))

(define (org-item-parent lines n)
  "Return the parent item of line N, or #f if there is none."
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

(define (org-item-children lines n)
  "Return the child items of line N, one level deeper."
  (let* ((end (org-item-end lines n))
         (items (filter (lambda (k) (org-parse-item (org-line lines k)))
                        (map (lambda (i) (+ n 1 i)) (org-range (- end n)))))
         (indent (if (null? items) 0 (apply min (map (lambda (k) (org-indentation (org-line lines k))) items)))))
    (filter (lambda (k) (= (org-indentation (org-line lines k)) indent)) items)))

(define (org-fix-bullets lines n)
  "Make the bullets of the list at line N the same, and renumber them.
They follow the bullet of the first item."
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

(define (org-insert-item checkbox)
  "Insert a list item.
If CHECKBOX is true, add a checkbox."
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

(define (org-move-item forward)
  "Swap the current item with the previous or next one.
FORWARD selects the next one."
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

(define (org-indent-item delta subtree)
  "Change the indentation of the current item by DELTA (1 deeper, -1 shallower).
If SUBTREE is true, change the children too."
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

(define org-list-bullets '("-" "+" "*" "1." "1)")
  "Order of the bullets used by S-left, S-right and C-c -.")

(define (org-cycle-list-bullet forward)
  "Change the bullet of the list.
If FORWARD is #f, use the previous bullet."
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

(define (org-item-move-point forward)
  "Move to the next or previous item (S-up, S-down).
FORWARD selects the next one."
  (let* ((lines (org-lines))
         (i (or (org-in-item) (error "Not in an item")))
         (sibling (org-item-sibling lines i forward)))
    (if sibling
        (goto-char (org-line-start lines sibling))
        (error (if forward "On last item" "On first item")))))

(define-command (org-ctrl-c-minus)
  "Insert a table rule, change the list bullet, or turn a heading or text into an item."
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

(define-command (org-toggle-checkbox)
  "Toggle the checkbox of the current item."
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

(define (org-fix-parent-checkboxes lines i)
  "Update the checkboxes of the parents of item I from their children.
A parent is [X] if all children are done, and [-] if some are."
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

(define (org-statistics lines n)
  "Return the done and total counts for the statistics cookie at line N, as (DONE TOTAL)."
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

(define (org-update-cookies-in-line lines n)
  "Rewrite the statistics cookies in line N."
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

(define-command (org-update-statistics-cookies)
  "Rewrite all the statistics cookies in the buffer."
  (org-update-statistics))

(define (org-update-statistics)
  "Rewrite all the statistics cookies in the headings and items of the buffer."
  (let ((lines (org-lines)))
    (for-each (lambda (n)
                (let ((line (org-line lines n)))
                  (when (and (regexp-match "\\[[0-9]*(?:%|/[0-9]*)\\]" line)
                             (or (org-level line) (org-parse-item line)))
                    (org-update-cookies-in-line (org-lines) n))))
              (map (lambda (i) (+ i 1)) (org-range (org-line-count lines))))))

;;; 日付

(define (org-today)
  "Return today's date, like 2026-10-26."
  (format-time-string "%Y-%m-%d"))

(define (org-date-time date)
  "Return the time of noon on DATE, in seconds.
Noon keeps the date right when adding days across daylight saving time."
  (date-to-time (string-append date " 12:00")))

(define (org-date-add date days)
  "Add DAYS days to DATE."
  (format-time-string "%Y-%m-%d" (+ (org-date-time date) (* days 86400))))

(define (org-make-date year month day)
  "Return the date of YEAR, MONTH and DAY.
A month or day out of range carries over."
  (let* ((m (- month 1))
         (y (+ year (/ (- m (modulo m 12)) 12)))
         (first (format "%d-%s-01" y (org-2d (+ (modulo m 12) 1)))))
    (org-date-add first (- day 1))))

(define (org-date-parts date)
  "Return DATE as (YEAR MONTH DAY)."
  (map string->number (split-string date "-")))

(define (org-date-add-months date months)
  "Add MONTHS months to DATE."
  (let ((p (org-date-parts date)))
    (org-make-date (car p) (+ (cadr p) months) (caddr p))))

(define (org-days-between a b)
  "Return the number of days from A to B."
  (let ((d (- (org-date-time b) (org-date-time a))))
    (if (>= d 0)
        (quotient (+ d 43200) 86400)
        (- (quotient (+ (- d) 43200) 86400)))))

(define (org-weekday date)
  "Return the weekday of DATE, like Mon."
  (format-time-string "%a" (org-date-time date)))

(define (org-date-shift date n unit)
  "Add N units to DATE. UNIT is d, w, m or y."
  (cond ((equal? unit "d") (org-date-add date n))
        ((equal? unit "w") (org-date-add date (* 7 n)))
        ((equal? unit "m") (org-date-add-months date n))
        ((equal? unit "y") (org-date-add-months date (* 12 n)))
        (else date)))

(define (org-make-timestamp date time active . rest)
  "Return a timestamp of DATE and TIME, like <2026-10-26 Mon 10:00 +1w>.
If ACTIVE is #f, use [ ] instead of < >."
  (string-append (if active "<" "[") date " " (org-weekday date)
                 (if time (string-append " " time) "")
                 (if (and (pair? rest) (car rest) (not (equal? (car rest) ""))) (car rest) "")
                 (if active ">" "]")))

(define (org-now-inactive)
  "Return the current date and time, like [2026-10-26 Mon 10:00]."
  (org-make-timestamp (org-today) (format-time-string "%H:%M") #f))

(define org-ts-regexp
  (string-append "([<\\[])([0-9]{4}-[0-9]{2}-[0-9]{2})(?: +[^ \\]>0-9+-]+)?"
                 "(?: +([0-9]{1,2}:[0-9]{2})(?:-([0-9]{1,2}:[0-9]{2}))?)?"
                 "((?: +[.+]?\\+[0-9]+[hdwmy])?(?: +-[0-9]+[hdwmy])?)[ ]*([>\\]])")
  "Regexp of a timestamp.
Groups: 1 bracket, 2 date, 3 time, 4 end time, 5 repeater and warning.")

(define (org-parse-timestamp s)
  "Split the timestamp S into (BRACKET DATE TIME END-TIME REPEATER).
Return #f if S is not a timestamp."
  (let ((m (regexp-match (string-append "^" org-ts-regexp "$") s)))
    (and m (equal? (if (equal? (cadr m) "<") ">" "]") (list-ref m 6))
         (list (cadr m) (caddr m) (list-ref m 3) (list-ref m 4) (list-ref m 5)))))

(define (org-timestamp-string parsed)
  "Return the timestamp of the parsed timestamp PARSED."
  (let ((time (and (caddr parsed)
                   (if (list-ref parsed 3)
                       (string-append (caddr parsed) "-" (list-ref parsed 3))
                       (caddr parsed)))))
    (org-make-timestamp (cadr parsed) time (equal? (car parsed) "<") (list-ref parsed 4))))

(define (org-timestamp-repeater parsed)
  "Return the repeater of PARSED as (MARK NUMBER UNIT), or #f if there is none."
  (let ((m (regexp-match " +([.+]?\\+)([0-9]+)([hdwmy])" (or (list-ref parsed 4) ""))))
    (and m (list (cadr m) (string->number (caddr m)) (list-ref m 3)))))

(define (org-timestamp-warning parsed)
  "Return the warning period of PARSED, like -3d, or #f if there is none."
  (let ((m (regexp-match " +-([0-9]+)([hdwmy])" (or (list-ref parsed 4) ""))))
    (and m (let ((n (string->number (cadr m))))
             (cond ((equal? (caddr m) "w") (* 7 n))
                   ((equal? (caddr m) "m") (* 30 n))
                   ((equal? (caddr m) "y") (* 365 n))
                   (else n))))))

(define org-weekdays '("sun" "mon" "tue" "wed" "thu" "fri" "sat") "Names of the weekdays.")

(define (org-read-date-analyze input default)
  "Read the date INPUT relative to DEFAULT, and return (DATE TIME).
TIME is #f if there is none. This is part of the syntax of `org-read-date':
empty (DEFAULT), +3 (3 days from today), +2w, -1d, ++1w (from DEFAULT),
2026-10-30, 10-30, 10/30, 30, fri (next Friday), 10:00, 10:00-11:00 and 3pm."
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

(define (org-normalize-time time)
  "Make the hour of TIME two digits (9:00 to 09:00)."
  (string-join (map (lambda (t)
                      (let ((p (split-string t ":")))
                        (string-append (org-2d (string->number (car p))) ":" (cadr p))))
                    (split-string time "-"))
               "-"))

(define (org-read-date prompt default)
  "Read a date in the minibuffer with PROMPT, and return (DATE TIME).
DEFAULT is the default date."
  (org-read-date-analyze (read-string (format "%sDate+time [%s]: " prompt default)) default))

(define (org-timestamp-at lines n col)
  "Return the timestamp at column COL of line N as (START END PARSED), or #f if there is none.
START and END are columns."
  (let ((line (org-line lines n)))
    (let loop ((ms (org-all-matches org-ts-regexp line)))
      (cond ((null? ms) #f)
            ((and (<= (car (car (car ms))) col) (< col (cadr (car (car ms)))))
             (let* ((range (car (car ms)))
                    (parsed (org-parse-timestamp (substring line (car range) (cadr range)))))
               (if parsed (list (car range) (cadr range) parsed) (loop (cdr ms)))))
            (else (loop (cdr ms)))))))

(define (org-insert-timestamp-command active)
  "Insert a timestamp, or change the one at point (C-c . and C-c !).
If ACTIVE is #f, insert an inactive timestamp."
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

(define-command (org-time-stamp)
  "Insert a timestamp, or change the one at point."
  (org-insert-timestamp-command #t))
(define-command (org-time-stamp-inactive)
  "Insert an inactive timestamp, or change the one at point."
  (org-insert-timestamp-command #f))

(define (org-timestamp-change parsed what n)
  "Change the part WHAT of PARSED by N (S-arrows).
WHAT is year, month, day, hour or minute."
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

(define (org-timestamp-part at col)
  "Return the part of the timestamp AT at column COL: year, month, day, hour or minute."
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

(define (org-shift-timestamp n by-day)
  "Change the timestamp at point by N.
If BY-DAY is true, change the day; otherwise the part at point.
Return #f if there is no timestamp at point."
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

(define (org-planning-line? line)
  "Return #t if LINE is a planning line (SCHEDULED: and so on)."
  (if (regexp-match "^[ \\t]*(?:SCHEDULED|DEADLINE|CLOSED): *[<\\[]" line) #t #f))

(define (org-planning-items line)
  "Return the planning items of LINE as ((KEYWORD TIMESTAMP) ...)."
  (map (lambda (m) (list (org-match-string line m 1) (org-match-string line m 2)))
       (org-all-matches "(SCHEDULED|DEADLINE|CLOSED): *([<\\[][^>\\]\\n]*[>\\]])" line)))

(define (org-get-planning lines n keyword)
  "Return the timestamp of KEYWORD in the planning of the heading at line N, or #f."
  (and (< n (org-line-count lines))
       (org-planning-line? (org-line lines (+ n 1)))
       (let ((found (assoc keyword (org-planning-items (org-line lines (+ n 1))))))
         (and found (cadr found)))))

(define (org-set-planning lines n keyword stamp)
  "Set the timestamp of KEYWORD in the planning of the heading at line N to STAMP.
If STAMP is #f, remove it. A new one is written at the start of the line."
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

(define (org-planning-command keyword)
  "Read a date and write it as KEYWORD (C-c C-s and C-c C-d)."
  (let* ((lines (org-lines))
         (h (org-back-to-heading lines))
         (old (org-get-planning lines h keyword))
         (parsed (and old (org-parse-timestamp old)))
         (answer (org-read-date (string-append keyword " ") (if parsed (cadr parsed) (org-today))))
         (stamp (org-make-timestamp (car answer) (cadr answer) #t (and parsed (list-ref parsed 4)))))
    (org-set-planning lines h keyword stamp)
    (message (if (equal? keyword "SCHEDULED") "Scheduled to %s" "Deadline on %s") stamp)))

(define-command (org-schedule)
  "Write the SCHEDULED date of the current heading."
  (org-planning-command "SCHEDULED"))
(define-command (org-deadline)
  "Write the DEADLINE date of the current heading."
  (org-planning-command "DEADLINE"))

;;; TODO

(define (org-next-todo old forward all)
  "Return the state after or before OLD.
FORWARD selects the next one. If ALL is true, cycle through all the states;
otherwise cycle within the set of OLD (C-c C-t)."
  (let ((words (if all
                   (org-todo-keywords-1)
                   (let ((set (org-todo-set old))) (append (car set) (cadr set))))))
    (if (not old)
        (if forward (car words) (org-last words))
        (let ((i (org-index-of old words)))
          (cond ((not i) (car words))
                (forward (and (< (+ i 1) (length words)) (list-ref words (+ i 1))))
                (else (and (> i 0) (list-ref words (- i 1)))))))))

(define (org-fast-todo-selection)
  "Select a state by its key (SPC removes the state)."
  (let* ((keys (org-todo-keys))
         (answer (read-string
                  (string-append (string-join (map (lambda (k) (format "[%s] %s" (car k) (cadr k))) keys) "  ")
                                 "  [SPC] clear: ")))
         (found (assoc answer keys)))
    (cond ((org-blank? answer) #f)
          (found (cadr found))
          ((member (string-upcase answer) (org-todo-keywords-1)) (string-upcase answer))
          (else (error "Quit")))))

(define (org-set-todo h new)
  "Set the state of the heading at line H to NEW, or remove it if NEW is #f.
Marking it done writes CLOSED, and moves a repeating date forward."
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

(define (org-repeating-stamps lines h)
  "Return the repeating timestamps of the heading at line H as ((KEYWORD TIMESTAMP) ...)."
  (if (and (< h (org-line-count lines)) (org-planning-line? (org-line lines (+ h 1))))
      (filter (lambda (i) (and (member (car i) '("SCHEDULED" "DEADLINE"))
                               (let ((p (org-parse-timestamp (cadr i))))
                                 (and p (org-timestamp-repeater p)))))
              (org-planning-items (org-line lines (+ h 1))))
      '()))

(define (org-auto-repeat h done-word repeats)
  "Move the repeating timestamps REPEATS forward, and reset the state of line H.
DONE-WORD is the done state. LAST_REPEAT and a log of the state change are written."
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

(define (org-log-beginning lines h)
  "Return the line to write a log of the heading at line H.
It is after the heading, the planning line and PROPERTIES."
  (let* ((n (if (and (< h (org-line-count lines)) (org-planning-line? (org-line lines (+ h 1)))) (+ h 1) h)))
    (if (and (< n (org-line-count lines))
             (regexp-match "^[ \\t]*:PROPERTIES:[ \\t]*$" (org-line lines (+ n 1))))
        (let loop ((k (+ n 2)))
          (cond ((> k (org-line-count lines)) n)
                ((regexp-match "^[ \\t]*:END:[ \\t]*$" (org-line lines k)) k)
                (else (loop (+ k 1)))))
        n)))

(define (org-entry-put lines h name value)
  "Set the property NAME of the heading at line H to VALUE, creating PROPERTIES if needed."
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

(define (org-entry-get lines h name)
  "Return the property NAME of the heading at line H, or #f if there is none."
  (let ((end (org-entry-end lines h)))
    (let loop ((k (+ h 1)))
      (if (> k end)
          #f
          (let ((m (regexp-match (format "^[ \\t]*:%s:[ \\t]*(.*?)[ \\t]*$" (regexp-quote name)) (org-line lines k))))
            (if m (cadr m) (loop (+ k 1))))))))

(define-command (org-todo)
  "Change the TODO state of the current heading.
If some states have keys, select one by its key."
  (let* ((lines (org-lines))
         (h (org-back-to-heading lines))
         (old (cadr (org-parse-heading (org-line lines h)))))
    (org-set-todo h (if (null? (org-todo-keys))
                        (org-next-todo old #t #f)
                        (org-fast-todo-selection)))))

;;; 優先度

(define (org-priority-chars)
  "Return the priority characters, highest first."
  (let* ((letters "ABCDEFGHIJKLMNOPQRSTUVWXYZ")
         (from (string-search org-priority-highest letters))
         (to (string-search org-priority-lowest letters)))
    (map (lambda (i) (org-char letters (+ from i))) (org-range (+ (- to from) 1)))))

(define (org-set-priority new)
  "Set the priority of the current heading to NEW, or remove it if NEW is #f."
  (let* ((lines (org-lines))
         (h (org-back-to-heading lines))
         (parsed (org-parse-heading (org-line lines h))))
    (org-set-line lines h (org-make-heading (car parsed) (cadr parsed) (and new (format "[#%s]" new))
                                            (list-ref parsed 3) (list-ref parsed 4)))
    (if new
        (message "Priority of current item set to %s" new)
        (message "Priority removed"))))

(define (org-priority-shift up)
  "Raise the priority if UP is true, or lower it (S-up, S-down).
Going out of range removes it."
  (let* ((lines (org-lines))
         (h (org-back-to-heading lines))
         (current (caddr (org-parse-heading (org-line lines h))))
         (chars (org-priority-chars)))
    (if (not current)
        (org-set-priority org-priority-default)
        (let ((i (+ (or (org-index-of (substring current 2 3) chars) 0) (if up -1 1))))
          (org-set-priority (and (>= i 0) (< i (length chars)) (list-ref chars i)))))))

(define-command (org-priority-up)
  "Raise the priority of the current heading."
  (org-priority-shift #t))
(define-command (org-priority-down)
  "Lower the priority of the current heading."
  (org-priority-shift #f))

(define-command (org-priority)
  "Select the priority of the current heading (SPC removes it)."
  (let ((answer (string-upcase
                 (read-string (format "Priority %s-%s, SPC to remove: " org-priority-highest org-priority-lowest)))))
    (cond ((org-blank? answer) (org-set-priority #f))
          ((member answer (org-priority-chars)) (org-set-priority answer))
          (else (error "Priority must be between `%s' and `%s'" org-priority-highest org-priority-lowest)))))

;;; タグ

(define (org-buffer-tags lines)
  "Return all the tags in LINES."
  (org-unique (apply append (map (lambda (line)
                                   (let ((parsed (org-parse-heading line)))
                                     (if parsed (org-heading-tags parsed) '())))
                                 lines))))

(define (org-set-tags h tags)
  "Set the tags of the heading at line H to TAGS."
  (let* ((lines (org-lines))
         (parsed (org-parse-heading (org-line lines h))))
    (org-set-line lines h (org-make-heading (car parsed) (cadr parsed) (caddr parsed) (list-ref parsed 3)
                                            (org-tags-string tags)))))

(define-command (org-set-tags-command)
  "Read the tags of the current heading, separated by : or spaces."
  (let* ((lines (org-lines))
         (h (org-back-to-heading lines))
         (parsed (org-parse-heading (org-line lines h)))
         (all (org-buffer-tags lines))
         (answer (read-string (if (null? all) "Tags: " (format "Tags (%s): " (string-join all " ")))
                              (or (list-ref parsed 4) ""))))
    (org-set-tags h (org-unique (filter (lambda (t) (not (equal? t "")))
                                        (split-string (org-replace-all answer " " ":") ":"))))))

;;; 表

(define (org-table-line? line)
  "Return #t if LINE is a table line."
  (if (regexp-match "^[ \\t]*\\|" line) #t #f))

(define (org-table-hline? line)
  "Return #t if LINE is a table rule."
  (if (regexp-match "^[ \\t]*\\|-" line) #t #f))

(define (org-table-bounds lines n)
  "Return the first and last lines of the table containing line N, as (START END)."
  (let ((first (let loop ((k n))
                 (if (and (> k 1) (org-table-line? (org-line lines (- k 1)))) (loop (- k 1)) k)))
        (last (let loop ((k n))
                (if (and (< k (org-line-count lines)) (org-table-line? (org-line lines (+ k 1))))
                    (loop (+ k 1))
                    k))))
    (list first last)))

(define (org-table-cells line)
  "Return the fields of the table LINE, or 'hline for a rule."
  (if (org-table-hline? line)
      'hline
      (let* ((body (cadr (regexp-match "^[ \\t]*\\|(.*?)\\|?[ \\t]*$" line))))
        (map string-trim (split-string body "|")))))

(define (org-table-number? s)
  "Return #t if the field S is a number."
  (if (regexp-match "^[<>]?[-+^.0-9]*[0-9][-+^.0-9eEdDx()%:]*$" s) #t #f))

(define (org-table-render indent rows)
  "Return the aligned lines of the table ROWS, indented by INDENT.
ROWS is a list of field lists or 'hline."
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

(define (org-table-context)
  "Return the table at point as (START END INDENT ROW COLUMN OFFSET).
OFFSET is the column within the field."
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

(define (org-table-align-and-go rows-change row column offset)
  "Align the table and move to row ROW, field COLUMN, OFFSET from the start of its text.
ROWS-CHANGE changes the rows before aligning."
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

(define (org-table-cell-column line column offset)
  "Return the column OFFSET from the start of the text of field COLUMN in the aligned LINE.
For an empty field, it is after the space after |."
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

(define (org-table-columns rows)
  "Return the number of fields of ROWS."
  (let ((data (filter pair? rows)))
    (if (null? data) 1 (apply max (map length data)))))

(define-command (org-table-align)
  "Align the table at point."
  (let ((ctx (org-table-context)))
    (org-table-align-and-go (lambda (rows) rows) (list-ref ctx 3) (list-ref ctx 4) (list-ref ctx 5))))

(define (org-table-empty-row rows)
  "Return an empty row for ROWS."
  (map (lambda (i) "") (org-range (org-table-columns rows))))

(define-command (org-table-next-field)
  "Move to the next field; in the last field, add a row."
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

(define-command (org-table-previous-field)
  "Move to the previous field."
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

(define-command (org-table-next-row)
  "Move to the same field in the next row.
If there is no next row or it is a rule, add a row."
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

(define (org-table-insert-hline)
  "Insert a rule below the current row."
  (let ((ctx (org-table-context)))
    (org-table-align-and-go (lambda (rows) (append (org-take rows (+ (list-ref ctx 3) 1)) (list 'hline)
                                                   (org-drop rows (+ (list-ref ctx 3) 1))))
                            (list-ref ctx 3) (list-ref ctx 4) (list-ref ctx 5))))

(define (org-swap items i j)
  "Swap elements I and J of ITEMS."
  (map (lambda (k) (list-ref items (cond ((= k i) j) ((= k j) i) (else k)))) (org-range (length items))))

(define (org-table-map-cells rows f)
  "Apply F to the fields of each row of ROWS, keeping rules."
  (map (lambda (row) (if (eq? row 'hline) row (f row))) rows))

(define (org-table-pad rows)
  "Pad each row of ROWS to the number of fields."
  (let ((columns (org-table-columns rows)))
    (org-table-map-cells rows (lambda (row) (append row (map (lambda (i) "") (org-range (- columns (length row)))))))))

(define (org-table-move-column forward)
  "Move the current column left or right (M-left, M-right).
FORWARD moves it right."
  (let* ((ctx (org-table-context))
         (c (list-ref ctx 4))
         (d (if forward (+ c 1) (- c 1)))
         (lines (org-lines))
         (rows (map org-table-cells (org-take (org-drop lines (- (car ctx) 1)) (+ (- (cadr ctx) (car ctx)) 1)))))
    (when (or (< d 0) (>= d (org-table-columns rows)))
      (error "Cannot move column further"))
    (org-table-align-and-go (lambda (rows) (org-table-map-cells (org-table-pad rows) (lambda (row) (org-swap row c d))))
                            (list-ref ctx 3) d (list-ref ctx 5))))

(define (org-table-move-row forward)
  "Move the current row up or down (M-up, M-down).
FORWARD moves it down."
  (let* ((ctx (org-table-context))
         (r (list-ref ctx 3))
         (d (if forward (+ r 1) (- r 1))))
    (when (or (< d 0) (> d (- (cadr ctx) (car ctx))))
      (error "Cannot move row further"))
    (org-table-align-and-go (lambda (rows) (org-swap rows r d)) d (list-ref ctx 4) (list-ref ctx 5))))

(define (org-table-delete-column)
  "Delete the current column (M-S-left)."
  (let* ((ctx (org-table-context))
         (c (list-ref ctx 4)))
    (org-table-align-and-go
     (lambda (rows) (org-table-map-cells (org-table-pad rows)
                                         (lambda (row) (append (org-take row c) (org-drop row (+ c 1))))))
     (list-ref ctx 3) (max 0 (- c 1)) 0)))

(define (org-table-insert-column)
  "Insert a column on the left (M-S-right)."
  (let* ((ctx (org-table-context))
         (c (list-ref ctx 4)))
    (org-table-align-and-go
     (lambda (rows) (org-table-map-cells (org-table-pad rows)
                                         (lambda (row) (append (org-take row c) (list "") (org-drop row c)))))
     (list-ref ctx 3) c 0)))

(define (org-table-kill-row)
  "Delete the current row (M-S-up)."
  (let* ((ctx (org-table-context))
         (r (list-ref ctx 3)))
    (if (= (car ctx) (cadr ctx))
        (let ((lines (org-lines)))
          (org-delete-lines lines (car ctx) (car ctx)))
        (org-table-align-and-go (lambda (rows) (append (org-take rows r) (org-drop rows (+ r 1))))
                                (min r (- (cadr ctx) (car ctx) 1)) (list-ref ctx 4) 0))))

(define (org-table-insert-row)
  "Insert a row above (M-S-down)."
  (let* ((ctx (org-table-context))
         (r (list-ref ctx 3)))
    (org-table-align-and-go (lambda (rows) (append (org-take rows r) (list (org-table-empty-row rows)) (org-drop rows r)))
                            r (list-ref ctx 4) 0)))

(define-command (org-table-create-or-convert-from-region)
  "Create a table."
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

(define-command (org-sort)
  "Sort the children of the current heading.
Sort by a (title), n (number), o (TODO order), p (priority) or t (date);
uppercase letters sort in reverse."
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

(define (org-modifier-cursor-error)
  "Signal an error for a key pressed outside tables, headings and items."
  (error "This command is active in special context like tables, headlines or items"))
(define (org-shiftselect-error)
  "Signal an error for a shift key pressed where Org mode does not use it."
  (error "To use shift-selection with Org mode, customize `org-support-shift-select'"))

(define (org-context)
  "Return the kind of the line of point: table, heading, item or text."
  (let* ((lines (org-lines))
         (line (org-line lines (org-current-line lines))))
    (cond ((org-table-line? line) 'table)
          ((org-level line) 'heading)
          ((org-parse-item line) 'item)
          (else 'text))))

(define-command (org-metaleft)
  "Move the column left, promote the heading, outdent the item, or move back a word."
  (case (org-context)
    ((table) (org-table-move-column #f))
    ((heading) (org-do-promote))
    ((item) (org-indent-item -1 #f))
    (else (call-interactively 'backward-word))))

(define-command (org-metaright)
  "Move the column right, demote the heading, indent the item, or move forward a word."
  (case (org-context)
    ((table) (org-table-move-column #t))
    ((heading) (org-do-demote))
    ((item) (org-indent-item 1 #f))
    (else (call-interactively 'forward-word))))

(define-command (org-metaup)
  "Move the table row, subtree, item or line up."
  (case (org-context)
    ((table) (org-table-move-row #f))
    ((heading) (org-move-subtree #f))
    ((item) (org-move-item #f))
    (else (org-drag-line #f))))

(define-command (org-metadown)
  "Move the table row, subtree, item or line down."
  (case (org-context)
    ((table) (org-table-move-row #t))
    ((heading) (org-move-subtree #t))
    ((item) (org-move-item #t))
    (else (org-drag-line #t))))

(define-command (org-shiftmetaleft)
  "Delete the table column, promote the subtree, or outdent the item with its children."
  (case (org-context)
    ((table) (org-table-delete-column))
    ((heading) (org-promote-subtree))
    ((item) (org-indent-item -1 #t))
    (else (org-modifier-cursor-error))))

(define-command (org-shiftmetaright)
  "Insert a table column, demote the subtree, or indent the item with its children."
  (case (org-context)
    ((table) (org-table-insert-column))
    ((heading) (org-demote-subtree))
    ((item) (org-indent-item 1 #t))
    (else (org-modifier-cursor-error))))

(define-command (org-shiftmetaup)
  "Delete the table row, or move the subtree, item or line up."
  (case (org-context)
    ((table) (org-table-kill-row))
    ((heading) (org-move-subtree #f))
    ((item) (org-move-item #f))
    (else (org-drag-line #f))))

(define-command (org-shiftmetadown)
  "Insert a table row, or move the subtree, item or line down."
  (case (org-context)
    ((table) (org-table-insert-row))
    ((heading) (org-move-subtree #t))
    ((item) (org-move-item #t))
    (else (org-drag-line #t))))

(define (org-at-item-bullet-p)
  "Return #t if point is on the bullet of a list item."
  (let* ((lines (org-lines))
         (line (org-line lines (org-current-line lines)))
         (item (org-parse-item line)))
    (and item (< (org-current-column lines) (+ (string-length (car item)) (string-length (cadr item)) 1)))))

(define-command (org-shiftup)
  "Change the timestamp, raise the priority, or move to the previous item."
  (cond ((org-shift-timestamp 1 #f) #t)
        ((eq? (org-context) 'heading) (org-priority-up))
        ((org-in-item) (org-item-move-point #f))
        (else (org-shiftselect-error))))

(define-command (org-shiftdown)
  "Change the timestamp, lower the priority, or move to the next item."
  (cond ((org-shift-timestamp -1 #f) #t)
        ((eq? (org-context) 'heading) (org-priority-down))
        ((org-in-item) (org-item-move-point #t))
        (else (org-shiftselect-error))))

(define-command (org-shiftright)
  "Change the timestamp by a day, the TODO state, or the list bullet."
  (cond ((org-shift-timestamp 1 #t) #t)
        ((eq? (org-context) 'heading)
         (let* ((lines (org-lines)) (h (org-back-to-heading lines)))
           (org-set-todo h (org-next-todo (cadr (org-parse-heading (org-line lines h))) #t #t))))
        ((org-at-item-bullet-p) (org-cycle-list-bullet #t))
        (else (org-shiftselect-error))))

(define-command (org-shiftleft)
  "Change the timestamp back by a day, the TODO state, or the list bullet."
  (cond ((org-shift-timestamp -1 #t) #t)
        ((eq? (org-context) 'heading)
         (let* ((lines (org-lines)) (h (org-back-to-heading lines)))
           (org-set-todo h (org-next-todo (cadr (org-parse-heading (org-line lines h))) #f #t))))
        ((org-at-item-bullet-p) (org-cycle-list-bullet #f))
        (else (org-shiftselect-error))))

(define-command (org-ctrl-c-ctrl-c)
  "Align the table, toggle the checkbox, set the heading tags, or reread the #+ lines."
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

(define-command (org-return)
  "Move to the next table row, open the link at point, or insert a newline.
Links are opened if `org-return-follows-link' is true."
  (let* ((lines (org-lines))
         (line (org-line lines (org-current-line lines))))
    (cond ((org-table-line? line) (org-table-next-row))
          ((and org-return-follows-link (org-link-at-point)) (org-open-at-point))
          (else (call-interactively 'newline)))))

;;; リンク

(define org-stored-links '() "Stored links, a list of (LINK DESCRIPTION), newest first.")

(define (org-abbreviate-file-name file)
  "Replace the home directory in FILE with ~."
  (let ((home (expand-file-name "~")))
    (if (string-prefix? (string-append home "/") file)
        (string-append "~" (substring file (string-length home)))
        file)))

(define (org-file-directory file)
  "Return the directory of FILE, up to the last /."
  (let ((m (regexp-match "^(.*/)[^/]*$" file)))
    (if m (cadr m) "./")))

(define (org-file-base file)
  "Return the name of FILE without the directory and extension."
  (let ((m (regexp-match "([^/]*?)(?:\\.[^./]*)?$" file)))
    (cadr m)))

(define (org-link-at-point)
  "Return the link at point as (LINK DESCRIPTION START END), or #f if there is none.
START and END are columns."
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

(define (org-make-link-string link description)
  "Return the link LINK with DESCRIPTION, like [[LINK][DESCRIPTION]]."
  (if (and description (not (equal? description "")) (not (equal? description link)))
      (format "[[%s][%s]]" link description)
      (format "[[%s]]" link)))

(define-command (org-store-link)
  "Store a link to the current location."
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

(define (org-strip-cookies title)
  "Remove the statistics cookies ([2/3] [66%]) from TITLE."
  (let loop ((ms (reverse (org-all-matches " *\\[[0-9]*(?:%|/[0-9]*)\\] *" title))) (s title))
    (if (null? ms)
        (string-trim s)
        (loop (cdr ms) (string-append (substring s 0 (car (car (car ms))))
                                      (if (or (= (car (car (car ms))) 0)
                                              (= (cadr (car (car ms))) (string-length s)))
                                          "" " ")
                                      (substring s (cadr (car (car ms)))))))))

(define-command (org-insert-link)
  "Insert a link, or change the link at point."
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

(define-command (org-open-at-point)
  "Open the link at point."
  (let ((at (org-link-at-point)))
    (if at (org-link-open (car at)) (message "No link found"))))

(define (org-link-open link)
  "Open LINK.
http and so on use `browse-url', file: opens in the editor,
and others are searched in the buffer."
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

(define (org-link-search target)
  "Search the buffer for TARGET.
TARGET is *HEADING, #ID, a line number, /REGEXP/, <<TARGET>>, a title or words."
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

(define org-agenda-buffer-name "*Org Agenda*" "Name of the agenda buffer.")
(define org-agenda-markers '() "Entries of the agenda lines, a list of (LINE FILE HEADING-LINE).")
(define org-agenda-redo-command #f "Procedure to rebuild the agenda (called by r and g).")
(define org-agenda-start-day #f "First day of the weekly agenda.")
(define org-agenda-previous-buffer #f "Buffer shown before the agenda.")

(define (org-agenda-file-list)
  "Return the agenda files.
A directory in `org-agenda-files' means the .org files in it."
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

(define (org-agenda-file-text file)
  "Return the contents of FILE, or of its buffer if the buffer is modified."
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

(define (org-agenda-entries file)
  "Return the headings of FILE as a list of
(FILE LINE TODO PRIORITY TITLE TAGS CATEGORY SCHEDULED DEADLINE (DATES...) BODY)."
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

(define (org-agenda-all-entries)
  "Return the headings of all the agenda files."
  (apply append (map org-agenda-entries (org-agenda-file-list))))

(define (org-entry-file e)
  "Return the file of the agenda entry E."
  (list-ref e 0))
(define (org-entry-line e)
  "Return the line of the agenda entry E."
  (list-ref e 1))
(define (org-entry-todo e)
  "Return the TODO state of the agenda entry E."
  (list-ref e 2))
(define (org-entry-priority e)
  "Return the priority of the agenda entry E."
  (list-ref e 3))
(define (org-entry-title e)
  "Return the title of the agenda entry E."
  (list-ref e 4))
(define (org-entry-tags e)
  "Return the tags of the agenda entry E."
  (list-ref e 5))
(define (org-entry-category e)
  "Return the category of the agenda entry E."
  (list-ref e 6))

(define (org-timestamp-on? parsed day)
  "Return #t if the timestamp PARSED falls on DAY, including repeats."
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

(define (org-agenda-line e time extra)
  "Return an agenda line of the entry E with TIME and EXTRA.
It shows the category, time, planning mark, state, title and tags."
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

(define (org-agenda-time parsed)
  "Return the time of PARSED as shown in the agenda, like 10:00...... or 10:00-11:00.
A one-digit hour is padded with a space."
  (and parsed (caddr parsed)
       (let ((pad (lambda (t) (let ((h (string->number (car (split-string t ":")))))
                                (string-append (if (< h 10) " " "") (format "%d" h) ":" (cadr (split-string t ":")))))))
         (if (list-ref parsed 3)
             (string-append (pad (caddr parsed)) "-" (pad (list-ref parsed 3)))
             (string-append (pad (caddr parsed)) "......")))))

(define (org-agenda-day-items entries day today)
  "Return the agenda lines of DAY from ENTRIES, as a list of (TIME LINE ENTRY PRIORITY).
TODAY is today's date."
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

(define (org-agenda-priority e)
  "Return the priority of the entry E as a number, like `org-get-priority' in Emacs.
The lowest priority is 0, and each step up adds 1000."
  (let* ((chars (org-priority-chars))
         (m (regexp-match "[A-Z0-9]" (or (org-entry-priority e) "")))
         (i (or (and m (org-index-of (car m) chars)) (org-index-of org-priority-default chars))))
    (* 1000 (- (length chars) 1 i))))

(define (org-agenda-sort items)
  "Sort ITEMS: lines with a time first, in time order;
lines with the same time or no time by priority, highest first.
This is the default of `org-agenda-sorting-strategy' in Emacs
(time-up priority-down category-keep)."
  (let ((by-priority (lambda (a b) (> (list-ref a 3) (list-ref b 3)))))
    (append (sort (filter car items)
                  (lambda (a b) (or (string<? (car a) (car b))
                                    (and (equal? (car a) (car b)) (by-priority a b)))))
            (sort (filter (lambda (i) (not (car i))) items) by-priority))))

(define (org-agenda-show rows redo)
  "Show ROWS in the agenda buffer, and remember REDO to rebuild it.
ROWS is a list of (LINE ENTRY); ENTRY is #f for a heading line."
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

(define (org-agenda-mode)
  "Set the keys and highlighting of the agenda buffer."
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

(define (org-agenda-date-line day)
  "Return the line of DAY, like \"Monday     26 October 2026 W44\"."
  (let* ((time (org-date-time day))
         (weekday (format-time-string "%A" time))
         (week (if (equal? (format-time-string "%u" time) "1")
                   (string-append " W" (format-time-string "%V" time))
                   "")))
    (format "%s %s %s %s%s" (org-pad-right weekday 10)
            (org-pad-left (format "%d" (string->number (format-time-string "%d" time))) 2)
            (format-time-string "%B" time) (format-time-string "%Y" time) week)))

(define (org-agenda-list-from start)
  "Show the weekly agenda for `org-agenda-span' days from START."
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

(define (org-agenda-week-start day)
  "Return the first day of the week of DAY, on `org-agenda-start-on-weekday'."
  (if org-agenda-start-on-weekday
      (let ((w (string->number (format-time-string "%u" (org-date-time day)))))
        (org-date-add day (- (modulo (- w org-agenda-start-on-weekday) 7))))
      day))

(define-command (org-agenda-list)
  "Show the weekly agenda."
  (org-agenda-list-from (org-agenda-week-start (org-today))))

(define-command (org-todo-list)
  "Show the list of TODO entries."
  (let ((entries (filter (lambda (e) (and (org-entry-todo e) (not (org-done-keyword? (org-entry-todo e)))))
                         (org-agenda-all-entries))))
    (org-agenda-show
     (append (list (list "Global list of TODO items of type: ALL" #f))
             (map (lambda (e) (list (org-agenda-line e #f "") e)) entries))
     org-todo-list)))

(define-command (org-tags-view)
  "Show the entries matching tags.
a+b means both, a|b either, and -a without a."
  (let* ((match (read-string "Match: "))
         (entries (filter (lambda (e) (org-tags-match? match (org-entry-tags e))) (org-agenda-all-entries))))
    (org-agenda-show
     (cons (list (format "Headlines with TAGS match: %s" match) #f)
           (map (lambda (e) (list (org-agenda-line e #f "") e)) entries))
     (lambda () #t))))

(define (org-tags-match? match tags)
  "Return #t if TAGS match MATCH."
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

(define-command (org-search-view)
  "Show the headings containing all the words, ignoring case."
  (let* ((words (map string-downcase (split-string (read-string "Search for: "))))
         (entries (filter (lambda (e)
                            (let ((text (string-downcase (list-ref e 10))))
                              (null? (filter (lambda (w) (not (string-search w text))) words))))
                          (org-agenda-all-entries))))
    (org-agenda-show
     (cons (list (format "Search words: %s" (string-join words " ")) #f)
           (map (lambda (e) (list (org-agenda-line e #f "") e)) entries))
     (lambda () #t))))

(define-command (org-agenda)
  "Select an agenda view."
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

(define (org-agenda-marker)
  "Return the entry of the agenda line at point as (FILE LINE).
Signal an error if there is none."
  (let* ((n (org-current-line (org-lines)))
         (found (assoc n org-agenda-markers)))
    (if found (cdr found) (error "Command not allowed in this line"))))

(define-command (org-agenda-switch-to)
  "Open the file of the entry at point."
  (let ((m (org-agenda-marker)))
    (find-file (car m))
    (org-mode)
    (org-goto-heading (org-lines) (cadr m))))

(define-command (org-agenda-show-and-scroll-up)
  "Show the entry at point in another window."
  (let ((m (org-agenda-marker)))
    (display-buffer (find-file-noselect (car m)) (cadr m))))
(define-command (org-agenda-goto)
  "Show the entry at point in another window."
  (org-agenda-show-and-scroll-up))

(define-command (org-agenda-next-line)
  "Move to the next line of the agenda."
  (call-interactively 'next-line))
(define-command (org-agenda-previous-line)
  "Move to the previous line of the agenda."
  (call-interactively 'previous-line))

(define (org-agenda-move days)
  "Move the weekly agenda by DAYS days (f and b)."
  (if org-agenda-start-day
      (org-agenda-list-from (org-date-add org-agenda-start-day days))
      (error "Not allowed in this agenda")))
(define-command (org-agenda-later)
  "Show the next week."
  (org-agenda-move org-agenda-span))
(define-command (org-agenda-earlier)
  "Show the previous week."
  (org-agenda-move (- org-agenda-span)))
(define-command (org-agenda-goto-today)
  "Show the week of today."
  (if org-agenda-start-day (org-agenda-list) (error "Not allowed in this agenda")))

(define-command (org-agenda-redo)
  "Rebuild the agenda."
  (let ((n (org-current-line (org-lines))))
    (org-agenda-redo-command)
    (org-goto-line-column (org-lines) (min n (org-line-count (org-lines))) 0)))

(define-command (org-agenda-todo)
  "Change the TODO state of the entry at point."
  (let ((m (org-agenda-marker)))
    (find-file (car m))
    (org-mode)
    (org-goto-heading (org-lines) (cadr m))
    (org-todo)
    (switch-to-buffer org-agenda-buffer-name)
    (org-agenda-redo)))

(define-command (org-agenda-quit)
  "Kill the agenda buffer."
  (let ((previous org-agenda-previous-buffer))
    (kill-buffer org-agenda-buffer-name)
    (when (and previous (member previous (buffer-list)))
      (switch-to-buffer previous))))

;;; キャプチャ

(define org-capture-buffer-name "*Capture*" "Name of the capture buffer.")
(define org-capture-current #f
  "Current capture, as (TEMPLATE BUFFER POSITION).
BUFFER and POSITION are where the capture started.")

(define (org-capture-templates-list)
  "Return the capture templates, or the default ones if there are none."
  (if (null? org-capture-templates)
      '(("t" "Task" entry (file+headline "" "Tasks") "* TODO %?\n  %u\n  %a"))
      org-capture-templates))

(define (org-capture-expand-file file)
  "Return the target file of FILE.
\"\" means `org-default-notes-file'; a relative name is in `org-directory'."
  (cond ((or (not file) (equal? file "")) (expand-file-name org-default-notes-file))
        ((regexp-match "^(?:/|~)" file) (expand-file-name file))
        (else (expand-file-name file org-directory))))

(define (org-capture-fill-template template annotation file)
  "Fill TEMPLATE, and return (TEXT POSITION).
ANNOTATION is the link to the original location, and FILE is its file."
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

(define-command (org-capture)
  "Select a template and write in *Capture*."
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

(define (org-capture-place type target text)
  "Insert the captured TEXT of TYPE into TARGET.
This is called in the target buffer."
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

(define (org-capture-adjust-level text level)
  "Change the levels of the headings in TEXT to LEVEL."
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

(define-command (org-capture-finalize)
  "Finish the capture, add it to the target and save it."
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

(define-command (org-capture-kill)
  "Abort the capture."
  (unless org-capture-current (error "This does not seem to be a capture buffer for Org mode"))
  (let ((previous (cadr org-capture-current)))
    (kill-buffer org-capture-buffer-name)
    (set! org-capture-current #f)
    (when (and previous (member previous (buffer-list))) (switch-to-buffer previous))
    (message "Capture aborted")))

;;; バッファ

(define (org-buffer-list)
  "Return the Org buffers (.org files)."
  (filter (lambda (name)
            (let ((file (buffer-file-name name)))
              (and file (regexp-match "\\.org$" file))))
          (buffer-list)))

(define-command (org-switchb)
  "Switch to an Org buffer."
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
