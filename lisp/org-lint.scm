;;; org-lint.scm --- Org の書き方の誤りを調べる (org-lint.el の Scheme 版) -*- scheme -*-

;; Emacs の org-lint (org 9.7.11) を, rs-editor の Scheme で書き直したもの.
;;
;; 使い方: (require 'org-lint) と書く (org.scm を読んでいれば, 読まれている). M-x org-lint で, Org のバッファを調べ,
;; *Org Lint* のバッファに, 行, 信頼度, 警告, チェッカーの名前を並べる (RET で, その行に移る).
;; プログラムからは, (org-lint) が, Emacs と同じ形 ((ID (LINE TRUST DESCRIPTION CHECKER)) の並び) を返す.
;; 引数に, チェッカーの名前の並びを渡すと, それだけを動かす.
;;
;; Emacs と違う所:
;; - Emacs は, 構文木 (org-element) を作って調べる. ここは, 木が無いので, 行を種類 (見出し, ドロワー, ブロック,
;;   キーワード, リスト, 表, 段落など) に分けて, その上で調べる. 入れ子のリストの細かい構造は, 見ない.
;; - 調べられないチェッカーは, 登録しない (org-lint-unsupported-checkers に並べた). 木か, ox (書き出し),
;;   org-id, org-cite, babel の仕組みが要るもの.
;; - 信頼度は, Emacs と同じ (low, high, 付いていないものは nil).

(require 'org)

(define (org-lint--affiliated-records ast)
  ;; 親和的なキーワードの行の (記録 孤立か) の並び. 孤立とは, 続く要素が無いこと
  (let loop ((rest (org-lint--records ast)) (out '()))
    (cond ((null? rest) (reverse out))
          ((eq? (org-lint--kind (car rest)) 'affiliated)
           (loop (cdr rest) (cons (list (car rest) (not (org-lint--named-element? (cdr rest)))) out)))
          (else (loop (cdr rest) out)))))

(define (org-lint-orphaned-affiliated-keywords ast)
  (org-lint--filter-map
   (lambda (x)
     (let* ((r (car x)) (key (string-upcase (org-lint--keyword-key (org-lint--text r)))))
       (and (cadr x)
            (not (member key '("RESULT" "RESULTS")))
            (list (org-lint--start r) (format "Orphaned affiliated keyword: \"%s\"" key)))))
   (org-lint--affiliated-records ast)))

(define (org-lint-obsolete-affiliated-keywords ast)
  (org-lint--filter-map
   (lambda (x)
     (let* ((r (car x))
            (m (regexp-match "(?i)^[ \\t]*#\\+(DATA|LABEL|RESNAME|SOURCE|SRCNAME|TBLNAME|RESULT|HEADERS):" (org-lint--text r))))
       (and m (not (cadr x))
            (let ((key (string-upcase (cadr m))))
              (list (org-lint--start r)
                    (format "Obsolete affiliated keyword: \"%s\".  Use \"%s\" instead" key
                            (cond ((equal? key "HEADERS") "HEADER")
                                  ((equal? key "RESULT") "RESULTS")
                                  (else "NAME"))))))))
   (org-lint--affiliated-records ast)))

(define (org-lint-invalid-keyword-syntax ast)
  (org-lint--filter-map
   (lambda (r)
     (let ((m (regexp-match "^[ \\t]*#\\+([^ \\t:]*)(?: |$)" (org-lint--text r))))
       (and m
            (not (regexp-match "(?i)^(BEGIN|END)" (cadr m)))
            (not (regexp-match "(?i)^[ \\t]*#\\+(CAPTION|RESULTS)(\\[.*\\])?:(?: |$)" (org-lint--text r)))
            (list (org-lint--start r) (format "Possible missing colon in keyword \"%s\"" (cadr m))))))
   (org-lint--records ast)))

(define (org-lint-invalid-block ast)
  (org-lint--filter-map
   (lambda (r)
     (let* ((text (org-lint--text r))
            (m (regexp-match-positions "(?i)^[ \\t]*#\\+(BEGIN|END)(?::|_[^ \\t]*)?[ \\t]*" text)))
       (and m
            (let ((name (string-trim text)))
              (cond ((and (regexp-match "(?i)^END" (substring text (car (cadr m)) (cadr (cadr m))))
                          (< (cadr (car m)) (string-length text)))
                     (list (org-lint--start r) (format "Invalid block closing line \"%s\"" name)))
                    ((not (member (org-lint--kind r)
                                  '(center-block comment-block dynamic-block example-block export-block
                                                 quote-block special-block src-block verse-block)))
                     (list (org-lint--start r) (format "Possible incomplete block \"%s\"" name)))
                    (else #f))))))
   (org-lint--records ast)))

(define org-lint--raw-kinds '(comment-block example-block export-block src-block verse-block))

(define (org-lint-incomplete-drawer ast)
  (let loop ((rest (org-lint--records ast)) (inside #f) (out '()))
    (if (null? rest)
        (reverse out)
        (let* ((r (car rest)) (kind (org-lint--kind r)) (text (org-lint--text r))
               (m (regexp-match org-lint--drawer-re text)))
          (cond
           ((member kind '(property-drawer node-property)) (loop (cdr rest) #f out))
           ((eq? kind 'drawer)
            (loop (cdr rest) (not (regexp-match org-lint--drawer-end-re text)) out))
           ((member kind org-lint--raw-kinds) (loop (cdr rest) inside out))
           ((and m inside)
            (loop (cdr rest) inside
                  (cons (list (org-lint--start r) (string-append "Possible misleading drawer entry " (org-lint--quote (string-trim text)))) out)))
           ((and m (not inside))
            (loop (cdr rest) inside
                  (cons (list (org-lint--start r) (string-append "Possible incomplete drawer " (org-lint--quote (string-trim text)))) out)))
           (else (loop (cdr rest) inside out)))))))

(define (org-lint-indented-diary-sexp ast)
  (org-lint--filter-map
   (lambda (r)
     (and (regexp-match "^[ \\t]+%%\\(" (org-lint--text r))
          (not (member (org-lint--kind r) (cons 'diary-sexp org-lint--raw-kinds)))
          (list (org-lint--start r) "Possible indented diary-sexp")))
   (org-lint--records ast)))

(define (org-lint-misplaced-planning-info ast)
  (org-lint--filter-map
   (lambda (r)
     (and (regexp-match "(?i)^[ \\t]*(CLOSED|DEADLINE|SCHEDULED):" (org-lint--text r))
          (not (member (org-lint--kind r) (cons 'planning org-lint--raw-kinds)))
          (list (org-lint--start r) "Misplaced planning info line")))
   (org-lint--records ast)))

(define (org-lint-obsolete-properties-drawer ast)
  (org-lint--filter-map
   (lambda (r)
     (and (eq? (org-lint--kind r) 'drawer)
          (regexp-match "^[ \\t]*:PROPERTIES:[ \\t]*$" (org-lint--text r))
          (list (org-lint--start r) "Incorrect contents for PROPERTIES drawer")))
   (org-lint--records ast)))


;;; リンク

;; 角括弧のリンク "[[PATH][DESCRIPTION]]" の記録: (位置 PATH DESCRIPTION か #f 直後が "]" か)
(define (org-lint--links-of record)
  (let loop ((text (org-lint--text record)) (offset 0) (out '()))
    (let ((m (regexp-match-positions "\\[\\[([^\\]\\[]+)\\](?:\\[(.+?)\\])?\\]" text)))
      (if m
          (let* ((whole (car m)) (path (cadr m)) (desc (caddr m))
                 (end (cadr whole)))
            (loop (substring text end) (+ offset end)
                  (cons (list (+ (org-lint--start record) offset (car whole))
                              (substring text (car path) (cadr path))
                              (and desc (substring text (car desc) (cadr desc)))
                              (and (< end (string-length text)) (equal? (substring text end (+ end 1)) "]")))
                        out)))
          (reverse out)))))

(define (org-lint--links ast)
  (apply append (map org-lint--links-of (org-lint--text-records ast))))

(define org-lint--link-types '("file+sys" "file+emacs" "shell" "news" "mailto" "https" "http" "ftp" "help" "elisp" "id")
  )

;; リンクの種類と, 種類を除いた PATH: (種類 PATH 応用). 種類は custom-id, fuzzy, file, ほか
(define (org-lint--link-type path)
  (let ((m (regexp-match "^([A-Za-z][-A-Za-z0-9+._]*):(.*)$" path)))
    (cond
     ((regexp-match "^#" path) (list "custom-id" (substring path 1) #f))
     ((and m (equal? (cadr m) "file")) (list "file" (caddr m) #f))
     ((and m (member (cadr m) org-lint--link-types))
      (if (regexp-match "^file\\+" (cadr m))
          (list "file" (caddr m) (substring (cadr m) 5))
          (list (cadr m) (caddr m) #f)))
     ((regexp-match "^(/|\\./|\\.\\./|~)" path) (list "file" path #f))
     (else (list "fuzzy" path #f)))))

(define (org-lint--normalize str)
  (string-downcase
   (string-join (filter (lambda (w) (not (equal? w ""))) (split-string (org-replace-all str "\t" " ") " ")) " ")))

;; 最初に合った RE を TO に置き換える (TO は文字そのもの)
(define (org-lint--replace-first re str to)
  (let ((m (regexp-match-positions re str)))
    (if m
        (string-append (substring str 0 (car (car m))) to (substring str (cadr (car m))))
        str)))

;; STR の中の CH (1 文字の文字列) の数
(define (org-lint--count-char ch str)
  (let loop ((i 0) (n 0))
    (if (>= i (string-length str))
        n
        (loop (+ i 1) (if (equal? (substring str i (+ i 1)) ch) (+ n 1) n)))))

(define (org-lint--headline-title text)
  ;; 見出しの題 (星, TODO のキーワード, 優先度, タグを除く)
  (let* ((t1 (org-lint--strip-stars text))
         (t2 (org-lint--replace-first "^(TODO|DONE)( +|$)" t1 ""))
         (t3 (org-lint--replace-first "^\\[#[A-Z0-9]+\\]( +|$)" t2 "")))
    (org-lint--replace-first "[ \\t]+:[[:alnum:]_@#%:]+:[ \\t]*$" t3 "")))

(define (org-lint--fuzzy-destinations ast)
  ;; (headline の題の並び 対象と NAME の並び)
  (list
   (map (lambda (r) (org-lint--normalize (org-lint--headline-title (org-lint--text r))))
        (org-lint--records-of ast 'headline))
   (append
    (apply append
           (map (lambda (r) (map (lambda (t) (org-lint--normalize (string-join (car t) " ")))
                                 (org-lint--targets (org-lint--text r))))
                (org-lint--text-records ast)))
    (org-lint--filter-map
     (lambda (r)
       (let ((m (regexp-match "(?i)^[ \\t]*#\\+NAME:[ \\t]*(.*?)[ \\t]*$" (org-lint--text r))))
         (and m (org-lint--normalize (cadr m)))))
     (org-lint--records-of ast 'affiliated)))))

(define (org-lint-invalid-custom-id-link ast)
  (let ((ids (org-lint--filter-map
              (lambda (r) (let ((p (org-lint--node-property r)))
                            (and p (equal? (string-upcase (car p)) "CUSTOM_ID") (cadr p))))
              (org-lint--records-of ast 'node-property))))
    (org-lint--filter-map
     (lambda (l)
       (let ((info (org-lint--link-type (cadr l))))
         (and (equal? (car info) "custom-id") (not (member (cadr info) ids))
              (list (car l) (format "Unknown custom ID \"%s\"" (cadr info))))))
     (org-lint--links ast))))

(define (org-lint-invalid-fuzzy-link ast)
  (let* ((dest (org-lint--fuzzy-destinations ast)))
    (org-lint--filter-map
     (lambda (l)
       (let ((info (org-lint--link-type (cadr l))))
         (and (equal? (car info) "fuzzy")
              (let* ((path (cadr info))
                     (heading-only (regexp-match "^\\*" path))
                     (name (if heading-only (substring path 1) path))
                     (key (org-lint--normalize name)))
                (and (not (member key (car dest)))
                     (or heading-only (not (member key (cadr dest))))
                     (list (car l) (format "Unknown fuzzy location \"%s\"" name)))))))
     (org-lint--links ast))))

(define (org-lint--buffer-directory)
  (let ((file (buffer-file-name)))
    (if file
        (org-lint--replace-first "[^/]*$" file "")
        "")))

(define (org-lint-link-to-local-file ast)
  (org-lint--filter-map
   (lambda (l)
     (let ((info (org-lint--link-type (cadr l))))
       (and (equal? (car info) "file")
            (let* ((raw (cadr info))
                   (path (org-lint--replace-first "::.*$" raw ""))
                   (full (if (equal? (org-lint--buffer-directory) "") path (expand-file-name path (org-lint--buffer-directory)))))
              (and (not (file-exists? full))
                   (list (car l) (string-append "Link to non-existent local file " (org-lint--quote path))))))))
   (org-lint--links ast)))

(define (org-lint-file-application ast)
  (org-lint--filter-map
   (lambda (l)
     (let ((info (org-lint--link-type (cadr l))))
       (and (caddr info)
            (list (car l) (format "Deprecated \"file+%s\" link type" (caddr info))))))
   (org-lint--links ast)))

(define (org-lint--obsolete-percent? uri)
  ;; "%XX" が, 25 5B 5D 20 だけで, 1 つはあるとき #t
  (let loop ((rest uri) (found #f))
    (let ((m (regexp-match-positions "%(..)?" rest)))
      (cond ((not m) found)
            ((and (cadr m) (member (substring rest (car (cadr m)) (cadr (cadr m))) '("25" "5B" "5D" "20")))
             (loop (substring rest (cadr (car m))) #t))
            (else #f)))))

(define (org-lint-percent-encoding-link-escape ast)
  (org-lint--filter-map
   (lambda (l)
     (let ((info (org-lint--link-type (cadr l))))
       (and (org-lint--obsolete-percent? (if (equal? (car info) "fuzzy") (cadr l) (cadr info)))
            (list (car l) "Link escaped with obsolete percent-encoding syntax"))))
   (org-lint--links ast)))

(define (org-lint-confusing-brackets ast)
  (org-lint--filter-map
   (lambda (l) (and (list-ref l 3) (list (car l) "Trailing ']' after link end")))
   (org-lint--links ast)))

(define (org-lint-brackets-inside-description ast)
  (org-lint--filter-map
   (lambda (l)
     (let ((desc (caddr l)))
       (and desc
            (> (org-lint--count-char "[" desc) (org-lint--count-char "]" desc))
            (list (car l)
                  (format "No closing ']' matches '[' in link description: %s" desc)))))
   (org-lint--links ast)))


;;; キーワード

;; キーワード KEY の行の (記録 値) の並び
(define (org-lint--keywords-named ast key)
  (org-lint--filter-map
   (lambda (r)
     (let ((m (regexp-match "^[ \\t]*#\\+([^ \\t:]+)(?:\\[.*\\])?:(?: +(.*?))?[ \\t]*$" (org-lint--text r))))
       (and m (equal? (string-upcase (cadr m)) key)
            (list r (if (caddr m) (caddr m) "")))))
   (org-lint--records-of ast 'keyword)))

(define org-lint--babel-header-names
  '("cache" "cmdline" "colnames" "comments" "eval" "exports" "epilogue" "file" "file-desc" "file-ext"
    "file-mode" "hlines" "mkdirp" "no-expand" "noeval" "noweb" "noweb-ref" "noweb-sep" "noweb-prefix"
    "output-dir" "padline" "post" "prologue" "results" "rownames" "sep" "session" "shebang" "tangle"
    "tangle-mode" "var" "wrap"))

(define (org-lint-deprecated-header-syntax ast)
  (append
   (org-lint--filter-map
    (lambda (k)
      (let ((m (regexp-match (string-append "(?i)^(" (string-join org-lint--babel-header-names "|") ")[ \\t]") (cadr k))))
        (and m (list (org-lint--start (car k))
                     (format "Deprecated syntax for \"%s\".  Use header-args instead" (cadr m))))))
    (org-lint--keywords-named ast "PROPERTY"))
   (org-lint--filter-map
    (lambda (r)
      (let ((p (org-lint--node-property r)))
        (and p (member (string-downcase (car p)) org-lint--babel-header-names)
             (list (org-lint--start r)
                   (format "Deprecated syntax for \"%s\".  Use :header-args: instead" (car p))))))
    (org-lint--records-of ast 'node-property))))

(define (org-lint--url? str) (regexp-match "^[A-Za-z][A-Za-z0-9+.-]*://" str))

(define (org-lint--strip-quotes str)
  (if (regexp-match "^\".*\"$" str) (substring str 1 (- (string-length str) 1)) str))

(define (org-lint--file-missing? file)
  (let ((dir (org-lint--buffer-directory)))
    (not (file-exists? (if (equal? dir "") file (expand-file-name file dir))))))

(define (org-lint-non-existent-setupfile-parameter ast)
  (org-lint--filter-map
   (lambda (k)
     (let ((file (org-lint--strip-quotes (cadr k))))
       (and (not (org-lint--url? file)) (org-lint--file-missing? file)
            (list (org-lint--start (car k)) (string-append "Non-existent setup file " (org-lint--quote file))))))
   (org-lint--keywords-named ast "SETUPFILE")))

(define (org-lint-wrong-include-link-parameter ast)
  (org-lint--filter-map
   (lambda (k)
     (let ((m (regexp-match "^(\".+?\"|[^ \\t]+)[ \\t]*" (cadr k))))
       (if (not m)
           (list (org-lint--start (car k)) "Missing location argument in INCLUDE keyword")
           (let* ((path (org-lint--strip-quotes (cadr m)))
                  (file (org-lint--replace-first "::.*$" path "")))
             (and (not (equal? file "")) (not (org-lint--url? file)) (org-lint--file-missing? file)
                  (list (org-lint--start (car k)) "Non-existent file argument in INCLUDE keyword"))))))
   (org-lint--keywords-named ast "INCLUDE")))

(define (org-lint-obsolete-include-markup ast)
  (org-lint--filter-map
   (lambda (k)
     (let ((m (regexp-match "(?i)^(?:\".+\"|[^ \\t]+)[ \\t]+(ASCII|BEAMER|HTML|LATEX|MAN|MARKDOWN|MD|ODT|ORG|TEXINFO)" (cadr k))))
       (and m (list (org-lint--start (car k))
                    (format "Obsolete markup \"%s\" in INCLUDE keyword.  Use \"export %s\" instead" (cadr m) (cadr m))))))
   (org-lint--keywords-named ast "INCLUDE")))

(define (org-lint-inactive-planning ast)
  (org-lint--filter-map
   (lambda (r)
     (let ((scheduled (regexp-match "(?i)SCHEDULED:[ \\t]+\\[" (org-lint--text r)))
           (deadline (regexp-match "(?i)DEADLINE:[ \\t]+\\[" (org-lint--text r))))
       (cond (scheduled (list (org-lint--start r) "Inactive timestamp in SCHEDULED will not appear in agenda."))
             (deadline (list (org-lint--start r) "Inactive timestamp in DEADLINE will not appear in agenda."))
             (else #f))))
   (org-lint--records-of ast 'planning)))

;;; チェッカーの登録

;; 登録されたチェッカーの並び (新しいものが先). 各々は (NAME SUMMARY FUNCTION TRUST CATEGORIES).
(define org-lint--checkers '())

(define (org-lint-add-checker name summary fun . props)
  "Add a new checker for the linter.
NAME is a unique symbol, SUMMARY is a short description, and FUN is called with the scanned buffer
and returns a list of (POSITION MESSAGE).  The optional PROPS are the trust level (low or high, or #f)
and the list of categories."
  (let ((trust (if (pair? props) (car props) #f))
        (categories (if (and (pair? props) (pair? (cdr props))) (cadr props) '())))
    (set! org-lint--checkers
          (cons (list name summary fun trust categories)
                (filter (lambda (c) (not (eq? (car c) name))) org-lint--checkers)))
    name))

(define (org-lint--checker-name c) (car c))
(define (org-lint--checker-fun c) (caddr c))
(define (org-lint--checker-trust c) (list-ref c 3))

;;; 行の種類

;; 行の記録: (N START TEXT KIND BLOCK) の並び.
;; N は行の番号 (1 から), START は行の始めの位置, TEXT は行の文字.
;; KIND は, その行を含む要素 (Emacs の org-element-at-point が返すもの) の種類:
;;   headline planning property-drawer node-property drawer drawer-end
;;   src-block example-block export-block comment-block verse-block (中身と, 始めと終わりの行)
;;   center-block quote-block special-block dynamic-block (始めと終わりの行)
;;   keyword affiliated comment fixed-width table item footnote-definition diary-sexp horizontal-rule
;;   latex-environment paragraph blank
;; BLOCK は, ブロックの種類の名前 (大文字) か #f.

(define (org-lint--n r) (car r))
(define (org-lint--start r) (cadr r))
(define (org-lint--text r) (caddr r))
(define (org-lint--kind r) (list-ref r 3))
(define (org-lint--block r) (list-ref r 4))

;; 要素に付く (親和的な) キーワード (ATTR_* を除く).
(define org-lint--affiliated-keywords
  '("CAPTION" "DATA" "HEADER" "HEADERS" "LABEL" "NAME" "PLOT" "RESNAME" "RESULT" "RESULTS" "SOURCE"
    "SRCNAME" "TBLNAME"))

(define (org-lint--affiliated-key? key)
  (let ((up (string-upcase key)))
    (or (member up org-lint--affiliated-keywords)
        (regexp-match "^ATTR_[-_A-Za-z0-9]+$" up))))

(define (org-lint--keyword-key text)
  ;; "#+KEY: value" の KEY (キーワードの行でなければ #f)
  (let ((m (regexp-match "^[ \\t]*#\\+([^ \\t:]+)(\\[.*\\])?:( |$)" text)))
    (and m (cadr m))))

(define (org-lint--raw-block-type name)
  (let ((up (string-upcase name)))
    (cond ((equal? up "SRC") 'src-block)
          ((equal? up "EXAMPLE") 'example-block)
          ((equal? up "EXPORT") 'export-block)
          ((equal? up "COMMENT") 'comment-block)
          ((equal? up "VERSE") 'verse-block)
          ((equal? up "CENTER") 'center-block)
          ((equal? up "QUOTE") 'quote-block)
          (else 'special-block))))

(define (org-lint--raw-block? type)
  (and (member type '(src-block example-block export-block comment-block verse-block)) #t))

(define (org-lint--headline? text) (regexp-match "^\\*+ " text))

;; text-list の先頭から, 次の見出しの前までに, PRED に合う行があるか
(define (org-lint--has-before-headline texts pred)
  (let loop ((rest texts))
    (cond ((null? rest) #f)
          ((org-lint--headline? (car rest)) #f)
          ((pred (car rest)) #t)
          (else (loop (cdr rest))))))

(define (org-lint--pop-container name container)
  ;; CONTAINER から, 最初の NAME までを取り除く
  (cond ((null? container) '())
        ((equal? (car container) name) (cdr container))
        (else (org-lint--pop-container name (cdr container)))))

(define org-lint--drawer-re "^[ \\t]*:([-_\\w]+):[ \\t]*$")
(define org-lint--node-property-re "^[ \\t]*:[^ \\t]+:(?:[ \\t]+.*?)?[ \\t]*$")
(define org-lint--drawer-end-re "(?i)^[ \\t]*:END:[ \\t]*$")

(define (org-lint--properties-valid? tail)
  ;; :END: までの行が, 全て節の性質の行か
  (let loop ((rest tail))
    (cond ((null? rest) #f)
          ((regexp-match org-lint--drawer-end-re (car rest)) #t)
          ((regexp-match org-lint--node-property-re (car rest)) (loop (cdr rest)))
          (else #f))))

;; TEXTS (バッファの行) を, 行の記録の並びに分ける.
(define (org-lint--scan texts)
  (let loop ((rest texts) (n 1) (start 1) (out '())
             ;; 開いている生のブロック: (種類 名前) か #f. ドロワーを開いているか. 前の行の種類. 開いているブロックの名前
             (block #f) (drawer #f) (prev 'start) (container '()))
    (if (null? rest)
        (reverse out)
        (let* ((text (car rest))
               (tail (cdr rest))
               (next-start (+ start (string-length text) 1))
               (emit (lambda (kind blk) (list n start text kind blk))))
          (cond
           ;; 生のブロックの中
           ((and block (org-lint--raw-block? (car block)))
            (if (regexp-match (string-append "(?i)^[ \\t]*#\\+END_" (regexp-quote (cadr block)) "[ \\t]*$") text)
                (loop tail (+ n 1) next-start (cons (emit (car block) (string-upcase (cadr block))) out) #f drawer 'block-end container)
                (loop tail (+ n 1) next-start (cons (emit (car block) (string-upcase (cadr block))) out) block drawer 'raw container)))
           ;; 見出し (ドロワーとブロックの外)
           ((org-lint--headline? text)
            (loop tail (+ n 1) next-start (cons (emit 'headline #f) out) #f #f 'headline '()))
           (else
            (let ((begin (regexp-match "(?i)^[ \\t]*#\\+BEGIN_([^ \\t]+)(.*)$" text))
                  (dynamic (regexp-match "(?i)^[ \\t]*#\\+BEGIN:[ \\t]+[^ \\t]" text))
                  (end (regexp-match "(?i)^[ \\t]*#\\+END_([^ \\t]+)[ \\t]*$" text))
                  (dynamic-end (regexp-match "(?i)^[ \\t]*#\\+END:?[ \\t]*$" text)))
              (cond
               ;; ブロックの始め (終わりが, 次の見出しまでにある)
               ((and begin
                     (org-lint--has-before-headline
                      tail
                      (lambda (l) (regexp-match (string-append "(?i)^[ \\t]*#\\+END_" (regexp-quote (cadr begin)) "[ \\t]*$") l))))
                (let ((type (org-lint--raw-block-type (cadr begin))))
                  (loop tail (+ n 1) next-start (cons (emit type (string-upcase (cadr begin))) out)
                        (if (org-lint--raw-block? type) (list type (cadr begin)) #f)
                        drawer 'block-begin
                        (if (org-lint--raw-block? type) container (cons (string-upcase (cadr begin)) container)))))
               ;; 動的ブロックの始め
               ((and dynamic (org-lint--has-before-headline tail (lambda (l) (regexp-match "(?i)^[ \\t]*#\\+END:?[ \\t]*$" l))))
                (loop tail (+ n 1) next-start (cons (emit 'dynamic-block ":") out) #f drawer 'block-begin (cons ":" container)))
               ;; ブロックの終わり (中身が container のブロック)
               ((and end (member (string-upcase (cadr end)) container))
                (loop tail (+ n 1) next-start
                      (cons (emit (org-lint--raw-block-type (cadr end)) (string-upcase (cadr end))) out)
                      #f drawer 'block-end (org-lint--pop-container (string-upcase (cadr end)) container)))
               ((and dynamic-end (member ":" container))
                (loop tail (+ n 1) next-start (cons (emit 'dynamic-block ":") out) #f drawer 'block-end
                      (org-lint--pop-container ":" container)))
               ;; ドロワー
               ((and drawer (regexp-match org-lint--drawer-end-re text))
                (loop tail (+ n 1) next-start (cons (emit (if (eq? drawer 'properties) 'property-drawer 'drawer) #f) out)
                      #f #f 'drawer-end container))
               (drawer
                (loop tail (+ n 1) next-start
                      (cons (if (eq? drawer 'properties)
                                (emit 'node-property #f)
                                (emit (org-lint--plain-kind text prev) #f))
                            out)
                      #f drawer 'drawer-body container))
               ((and (regexp-match "(?i)^[ \\t]*:PROPERTIES:[ \\t]*$" text)
                     (member prev '(headline planning start))
                     (org-lint--properties-valid? tail))
                (loop tail (+ n 1) next-start (cons (emit 'property-drawer #f) out) #f 'properties 'drawer-begin container))
               ((and (regexp-match org-lint--drawer-re text)
                     (not (regexp-match org-lint--drawer-end-re text))
                     (org-lint--has-before-headline tail (lambda (l) (regexp-match org-lint--drawer-end-re l))))
                (loop tail (+ n 1) next-start (cons (emit 'drawer #f) out) #f 'drawer 'drawer-begin container))
               (else
                (let ((kind (org-lint--plain-kind text prev)))
                  (loop tail (+ n 1) next-start (cons (emit kind #f) out) #f #f
                        (if (and (eq? prev 'start) (eq? kind 'comment)) 'start kind)
                        container)))))))))))

;; ブロックとドロワーの外の行 TEXT の種類. PREV は前の行の種類.
(define (org-lint--plain-kind text prev)
  (cond
   ((regexp-match "^[ \\t]*$" text) 'blank)
   ((and (eq? prev 'headline)
         (regexp-match "(?i)^[ \\t]*(SCHEDULED|DEADLINE|CLOSED):" text))
    'planning)
   ((and (eq? prev 'planning)
         (regexp-match "(?i)^[ \\t]*(SCHEDULED|DEADLINE|CLOSED):" text))
    'planning)
   ((org-lint--keyword-key text)
    (if (org-lint--affiliated-key? (org-lint--keyword-key text)) 'affiliated 'keyword))
   ((regexp-match "^[ \\t]*#( |$)" text) 'comment)
   ((regexp-match "^[ \\t]*:( |$)" text) 'fixed-width)
   ((regexp-match "^[ \\t]*\\|" text) 'table)
   ((regexp-match "^[ \\t]*-{5,}[ \\t]*$" text) 'horizontal-rule)
   ((regexp-match "^%%\\(" text) 'diary-sexp)
   ((regexp-match "^\\[fn:[-_A-Za-z0-9]+\\]" text) 'footnote-definition)
   ((regexp-match "^[ \\t]*([-+]|[0-9]+[.)]|[A-Za-z][.)]|\\*)[ \\t]" text) 'item)
   (else 'paragraph)))


;;; 部品

(define (org-lint--records ast) (cadr ast))

(define (org-lint--filter-map f items)
  (let loop ((rest items) (out '()))
    (if (null? rest)
        (reverse out)
        (let ((v (f (car rest))))
          (loop (cdr rest) (if v (cons v out) out))))))

;; STR を, `%S' のように, 文字列の書き方にする.
(define (org-lint--quote str)
  (string-append "\"" (org-replace-all (org-replace-all str "\\" "\\\\") "\"" "\\\"") "\""))

(define (org-lint--strip-stars text)
  (let ((m (regexp-match-positions "^\\*+[ \\t]+" text)))
    (if m (substring text (cadr (car m))) text)))

(define (org-lint--string-nw? str) (regexp-match "[^ \\t\\n]" str))

(define (org-lint--node-property r)
  ;; 節の性質の行 ":KEY: VALUE" の (KEY VALUE) か #f
  (let ((m (regexp-match "^[ \\t]*:([^ \\t]+):(?:[ \\t]+(.*?))?[ \\t]*$" (org-lint--text r))))
    (and m (list (cadr m) (if (caddr m) (caddr m) "")))))

(define (org-lint--records-of ast kind)
  (filter (lambda (r) (eq? (org-lint--kind r) kind)) (org-lint--records ast)))

;; ITEMS は (KEY POSITION) の並び (バッファの順). 重なった KEY の報告を返す. 後の重なりは全て, 最初のものは 1 回報告する.
(define (org-lint--collect-duplicates items message)
  (let loop ((rest items) (seen '()) (originals '()) (out '()))
    (if (null? rest)
        (append out (map (lambda (k) (list (cadr k) (message (car k)))) originals))
        (let* ((item (car rest))
               (prior (assoc (car item) seen)))
          (cond ((not (car item)) (loop (cdr rest) seen originals out))
                (prior
                 (loop (cdr rest) seen
                       (if (member prior originals) originals (append originals (list prior)))
                       (append out (list (list (cadr item) (message (car item)))))))
                (else (loop (cdr rest) (append seen (list item)) originals out)))))))

(define (org-lint--duration? str)
  (or (regexp-match "^([ \\t]*[0-9]+(\\.[0-9]*)?[ \\t]*(min|h|d|w|m|y))+[ \\t]*$" str)
      (regexp-match "^[ \\t]*[0-9]+:[0-9]{2}(:[0-9]{2})?[ \\t]*$" str)
      (regexp-match "^([ \\t]*[0-9]+[ \\t]*(h|d|w|m|y)[ \\t]+)+[0-9]+:[0-9]{2}(:[0-9]{2})?[ \\t]*$" str)))

(define org-lint--special-properties
  '("ALLTAGS" "BLOCKED" "CLOCKSUM" "CLOCKSUM_T" "CLOSED" "DEADLINE" "FILE" "ITEM" "PRIORITY"
    "SCHEDULED" "TAGS" "TIMESTAMP" "TIMESTAMP_IA" "TODO"))

(define org-lint--text-kinds '(headline paragraph item table footnote-definition))

(define (org-lint--text-records ast)
  (filter (lambda (r) (member (org-lint--kind r) org-lint--text-kinds)) (org-lint--records ast)))

;;; チェッカー

(define (org-lint--paragraph-end rest r)
  ;; REST は R の次からの行の記録. 段落 (R を含む, 続く paragraph の行) の次の行の始めの位置.
  ;; 段落が, バッファの終わりまでなら, R の始め.
  (let loop ((rest rest))
    (cond ((null? rest) (org-lint--start r))
          ((eq? (org-lint--kind (car rest)) 'paragraph) (loop (cdr rest)))
          (else (org-lint--start (car rest))))))

(define (org-lint-misplaced-heading ast)
  ;; Emacs は, 見出し以外の位置に, 段落の次の行の始め (org-element-at-point が, 照合の結果を書き換えるため) を報告する.
  (let loop ((rest (org-lint--records ast)) (out '()))
    (if (null? rest)
        (reverse out)
        (let* ((r (car rest))
               (kind (org-lint--kind r))
               (matches
                (if (member kind '(paragraph headline item))
                    (let scan ((text (org-lint--text r)) (n 0))
                      (let ((m (regexp-match-positions "[^*\\n\\r ,]\\*+\\* " text)))
                        (if m (scan (substring text (cadr (car m))) (+ n 1)) n)))
                    0))
               (pos (if (eq? kind 'headline) (org-lint--start r) (org-lint--paragraph-end (cdr rest) r))))
          (loop (cdr rest)
                (let add ((k matches) (out out))
                  (if (= k 0) out (add (- k 1) (cons (list pos "Possibly misplaced heading line") out)))))))))

(define (org-lint-duplicate-custom-id ast)
  (org-lint--collect-duplicates
   (org-lint--filter-map
    (lambda (r)
      (let ((p (org-lint--node-property r)))
        (and p (equal? (string-upcase (car p)) "CUSTOM_ID") (list (cadr p) (org-lint--start r)))))
    (org-lint--records-of ast 'node-property))
   (lambda (key) (format "Duplicate CUSTOM_ID property \"%s\"" key))))

(define (org-lint-duplicate-name ast)
  (let loop ((rest (org-lint--records ast)) (items '()))
    (if (null? rest)
        (org-lint--collect-duplicates
         (reverse items)
         (lambda (key) (format "Duplicate NAME \"%s\"" key)))
        (let* ((r (car rest))
               (m (and (eq? (org-lint--kind r) 'affiliated)
                       (regexp-match "(?i)^[ \\t]*#\\+NAME:[ \\t]*(.*?)[ \\t]*$" (org-lint--text r))))
               (named (and m (org-lint--named-element? (cdr rest)))))
          (loop (cdr rest) (if named (cons (list (cadr m) (org-lint--start r)) items) items))))))

(define (org-lint--named-element? rest)
  ;; 親和的なキーワードの次に, 要素が続くか
  (cond ((null? rest) #f)
        ((eq? (org-lint--kind (car rest)) 'affiliated) (org-lint--named-element? (cdr rest)))
        ((member (org-lint--kind (car rest)) '(blank headline)) #f)
        (else #t)))

(define (org-lint--targets text)
  ;; TEXT の中の対象 "<<...>>" の (内容 位置) の並び
  (let loop ((rest text) (offset 0) (out '()))
    (let ((m (regexp-match-positions "(^|[^<])<<([^<>\\n \\t]([^<>\\n]*[^<>\\n \\t])?)>>([^>]|$)" rest)))
      (if m
          (let* ((whole (car m)) (inner (caddr m))
                 (value (substring rest (car inner) (cadr inner))))
            (loop (substring rest (- (cadr whole) 1)) (+ offset (- (cadr whole) 1))
                  (cons (list (split-string value " ") (+ offset (car inner) -2)) out)))
          (reverse out)))))

(define (org-lint-duplicate-target ast)
  (org-lint--collect-duplicates
   (apply append
          (map (lambda (r)
                 (map (lambda (t) (list (car t) (+ (org-lint--start r) (cadr t))))
                      (org-lint--targets (org-lint--text r))))
               (org-lint--text-records ast)))
   (lambda (key) (format "Duplicate target <<%s>>" (string-join key " ")))))

(define (org-lint--footnote-label r)
  (let ((m (regexp-match "^\\[fn:([-_A-Za-z0-9]+)\\]" (org-lint--text r))))
    (and m (cadr m))))

(define (org-lint-duplicate-footnote-definition ast)
  (org-lint--collect-duplicates
   (org-lint--filter-map (lambda (r) (let ((l (org-lint--footnote-label r))) (and l (list l (org-lint--start r)))))
               (org-lint--records-of ast 'footnote-definition))
   (lambda (key) (format "Duplicate footnote definition \"%s\"" key))))

(define (org-lint--footnote-references ast)
  ;; 脚注の参照の (種類 ラベル 位置) の並び. 種類は standard か inline
  (apply append
         (map (lambda (r)
                (let* ((text (org-lint--text r))
                       (skip (if (eq? (org-lint--kind r) 'footnote-definition)
                                 (string-length (car (regexp-match "^\\[fn:[-_A-Za-z0-9]+\\]" text)))
                                 0)))
                  (let loop ((rest (substring text skip)) (offset skip) (out '()))
                    (let ((m (regexp-match-positions "\\[fn:([-_A-Za-z0-9]*)(:[^\\]]*)?\\]" rest)))
                      (if m
                          (let* ((whole (car m)) (label (cadr m)) (def (caddr m))
                                 (l (substring rest (car label) (cadr label))))
                            (loop (substring rest (cadr whole)) (+ offset (cadr whole))
                                  (if (or (equal? l "") (not (or def (> (string-length l) 0))))
                                      out
                                      (cons (list (if def 'inline 'standard) l (+ (org-lint--start r) offset (car whole))) out))))
                          (reverse out))))))
              (filter (lambda (r) (not (eq? (org-lint--kind r) 'headline))) (org-lint--text-records ast)))))

(define (org-lint-undefined-footnote-reference ast)
  (let* ((refs (org-lint--footnote-references ast))
         (defs (append (org-lint--filter-map org-lint--footnote-label (org-lint--records-of ast 'footnote-definition))
                       (org-lint--filter-map (lambda (x) (and (eq? (car x) 'inline) (cadr x))) refs))))
    (org-lint--filter-map (lambda (x)
                  (and (eq? (car x) 'standard) (not (member (cadr x) defs))
                       (list (caddr x) (format "Missing definition for footnote [%s]" (cadr x)))))
                refs)))

(define (org-lint-unreferenced-footnote-definition ast)
  (let ((labels (map cadr (org-lint--footnote-references ast))))
    (org-lint--filter-map (lambda (r)
                  (let ((l (org-lint--footnote-label r)))
                    (and l (not (member l labels))
                         (list (org-lint--start r) (format "No reference for footnote definition [%s]" l)))))
                (org-lint--records-of ast 'footnote-definition))))

(define (org-lint-deprecated-export-blocks ast)
  (org-lint--filter-map
   (lambda (r)
     (let ((m (and (eq? (org-lint--kind r) 'special-block)
                   (regexp-match "(?i)^[ \\t]*#\\+BEGIN_([^ \\t]+)" (org-lint--text r)))))
       (and m (member (string-upcase (cadr m)) '("ASCII" "BEAMER" "HTML" "LATEX" "MAN" "MARKDOWN" "MD" "ODT" "ORG" "TEXINFO"))
            (list (org-lint--start r)
                  (format "Deprecated syntax for export block.  Use \"BEGIN_EXPORT %s\" instead" (cadr m))))))
   (org-lint--records ast)))

(define (org-lint-missing-language-in-src-block ast)
  (org-lint--filter-map
   (lambda (r)
     (and (eq? (org-lint--kind r) 'src-block)
          (regexp-match "(?i)^[ \\t]*#\\+BEGIN_SRC[ \\t]*$" (org-lint--text r))
          (list (org-lint--start r) "Missing language in source block")))
   (org-lint--records ast)))

(define (org-lint-missing-backend-in-export-block ast)
  (org-lint--filter-map
   (lambda (r)
     (and (eq? (org-lint--kind r) 'export-block)
          (regexp-match "(?i)^[ \\t]*#\\+BEGIN_EXPORT[ \\t]*$" (org-lint--text r))
          (list (org-lint--start r) "Missing backend in export block")))
   (org-lint--records ast)))

(define (org-lint-deprecated-category-setup ast)
  (let loop ((rest (org-lint--records-of ast 'keyword)) (seen #f) (out '()))
    (cond ((null? rest) (reverse out))
          ((not (equal? (string-upcase (or (org-lint--keyword-key (org-lint--text (car rest))) "")) "CATEGORY"))
           (loop (cdr rest) seen out))
          (seen (loop (cdr rest) seen
                      (cons (list (org-lint--start (car rest))
                                  "Spurious CATEGORY keyword.  Set :CATEGORY: property instead")
                            out)))
          (else (loop (cdr rest) #t out)))))

(define (org-lint-special-property-in-properties-drawer ast)
  (org-lint--filter-map
   (lambda (r)
     (let ((p (org-lint--node-property r)))
       (and p (member (string-upcase (car p)) org-lint--special-properties)
            (list (org-lint--start r)
                  (format "Special property \"%s\" found in a properties drawer" (car p))))))
   (org-lint--records-of ast 'node-property)))

(define (org-lint-invalid-effort-property ast)
  (org-lint--filter-map
   (lambda (r)
     (let ((p (org-lint--node-property r)))
       (and p (equal? (car p) "EFFORT") (org-lint--string-nw? (cadr p))
            (not (org-lint--duration? (cadr p)))
            (list (org-lint--start r)
                  (string-append "Invalid effort duration format: " (org-lint--quote (cadr p)))))))
   (org-lint--records-of ast 'node-property)))

(define (org-lint-invalid-id-property ast)
  (org-lint--filter-map
   (lambda (r)
     (let ((p (org-lint--node-property r)))
       (and p (equal? (car p) "ID") (org-lint--string-nw? (cadr p)) (regexp-match "::" (cadr p))
            (list (org-lint--start r)
                  (string-append "IDs should not include \"::\": " (org-lint--quote (cadr p)))))))
   (org-lint--records-of ast 'node-property)))

(define (org-lint-quote-section ast)
  (org-lint--filter-map
   (lambda (r)
     (let ((title (org-lint--strip-stars (org-lint--text r))))
       (and (or (regexp-match "^QUOTE " title) (regexp-match "^COMMENT QUOTE " title))
            (list (org-lint--start r) "Deprecated QUOTE section"))))
   (org-lint--records-of ast 'headline)))

(define (org-lint-spurious-colons ast)
  (org-lint--filter-map
   (lambda (r)
     (let ((m (regexp-match "[ \\t]+(:[[:alnum:]_@#%:]+:)[ \\t]*$" (org-lint--text r))))
       (and m (regexp-match "::" (cadr m))
            (list (org-lint--start r) "Tags contain a spurious colon"))))
   (org-lint--records-of ast 'headline)))

;;; チェッカーの登録

(org-lint-add-checker 'misplaced-heading "Report accidentally misplaced heading lines."
                      org-lint-misplaced-heading 'low '())
(org-lint-add-checker 'duplicate-custom-id "Report duplicates CUSTOM_ID properties"
                      org-lint-duplicate-custom-id #f '(link))
(org-lint-add-checker 'duplicate-name "Report duplicate NAME values"
                      org-lint-duplicate-name #f '(babel link))
(org-lint-add-checker 'duplicate-target "Report duplicate targets"
                      org-lint-duplicate-target #f '(link))
(org-lint-add-checker 'duplicate-footnote-definition "Report duplicate footnote definitions"
                      org-lint-duplicate-footnote-definition #f '(footnote))
(org-lint-add-checker 'deprecated-export-blocks "Report deprecated export block syntax"
                      org-lint-deprecated-export-blocks 'low '(obsolete export))
(org-lint-add-checker 'missing-language-in-src-block "Report missing language in source blocks"
                      org-lint-missing-language-in-src-block #f '(babel))
(org-lint-add-checker 'missing-backend-in-export-block "Report missing backend in export blocks"
                      org-lint-missing-backend-in-export-block #f '(export))
(org-lint-add-checker 'deprecated-category-setup "Report misuse of CATEGORY keyword"
                      org-lint-deprecated-category-setup #f '(obsolete))
(org-lint-add-checker 'special-property-in-properties-drawer "Report special properties in properties drawers"
                      org-lint-special-property-in-properties-drawer #f '(properties))
(org-lint-add-checker 'invalid-effort-property "Report invalid duration in EFFORT property"
                      org-lint-invalid-effort-property #f '(properties))
(org-lint-add-checker 'invalid-id-property "Report invalid ID property"
                      org-lint-invalid-id-property #f '(properties))
(org-lint-add-checker 'undefined-footnote-reference "Report footnote references without definition"
                      org-lint-undefined-footnote-reference #f '(footnote))
(org-lint-add-checker 'unreferenced-footnote-definition "Report footnotes definitions without reference"
                      org-lint-unreferenced-footnote-definition #f '(footnote))
(org-lint-add-checker 'quote-section "Report obsolete QUOTE section"
                      org-lint-quote-section 'low '(obsolete))
(org-lint-add-checker 'spurious-colons "Report spurious colons in tags"
                      org-lint-spurious-colons #f '(tags))
(org-lint-add-checker 'orphaned-affiliated-keywords "Report orphaned affiliated keywords"
                      org-lint-orphaned-affiliated-keywords 'low '())
(org-lint-add-checker 'obsolete-affiliated-keywords "Report obsolete affiliated keywords"
                      org-lint-obsolete-affiliated-keywords #f '(obsolete))
(org-lint-add-checker 'invalid-keyword-syntax "Report mistakes in keyword syntax"
                      org-lint-invalid-keyword-syntax 'low '())
(org-lint-add-checker 'invalid-block "Report invalid blocks"
                      org-lint-invalid-block 'low '())
(org-lint-add-checker 'misplaced-planning-info "Report misplaced planning info line"
                      org-lint-misplaced-planning-info 'low '())
(org-lint-add-checker 'incomplete-drawer "Report forbidden or incomplete drawers"
                      org-lint-incomplete-drawer 'low '())
(org-lint-add-checker 'indented-diary-sexp "Report indented diary sexps"
                      org-lint-indented-diary-sexp 'low '())
(org-lint-add-checker 'obsolete-properties-drawer "Report obsolete syntax for properties drawers"
                      org-lint-obsolete-properties-drawer #f '(obsolete properties))
(org-lint-add-checker 'invalid-custom-id-link "Report \"custom-id\" links with unknown destination"
                      org-lint-invalid-custom-id-link #f '(link))
(org-lint-add-checker 'invalid-fuzzy-link "Report \"fuzzy\" links with unknown destination"
                      org-lint-invalid-fuzzy-link #f '(link))
(org-lint-add-checker 'trailing-bracket-after-link "Report potentially confused trailing ']' after link."
                      org-lint-confusing-brackets 'low '(link))
(org-lint-add-checker 'unclosed-brackets-in-link-description "Report potentially confused trailing ']' after link."
                      org-lint-brackets-inside-description 'low '(link))
(org-lint-add-checker 'link-to-local-file "Report links to non-existent local files"
                      org-lint-link-to-local-file 'low '(link))
(org-lint-add-checker 'file-application "Report obsolete \"file+application\" link"
                      org-lint-file-application #f '(link obsolete))
(org-lint-add-checker 'percent-encoding-link-escape "Report obsolete escape syntax in links"
                      org-lint-percent-encoding-link-escape 'low '(link obsolete))
(org-lint-add-checker 'deprecated-header-syntax "Report deprecated Babel header syntax"
                      org-lint-deprecated-header-syntax 'low '(obsolete babel))
(org-lint-add-checker 'non-existent-setupfile-parameter "Report SETUPFILE keywords with non-existent file parameter"
                      org-lint-non-existent-setupfile-parameter 'low '())
(org-lint-add-checker 'wrong-include-link-parameter "Report INCLUDE keywords with misleading link parameter"
                      org-lint-wrong-include-link-parameter 'low '())
(org-lint-add-checker 'obsolete-include-markup "Report obsolete markup in INCLUDE keyword"
                      org-lint-obsolete-include-markup 'low '(obsolete export))
(org-lint-add-checker 'planning-inactive "Report inactive timestamps in SCHEDULED/DEADLINE."
                      org-lint-inactive-planning 'high '(timestamp))

;;; レポートの組み立て

;; バッファの行 TEXTS に対する CHECKERS の報告. (ID (LINE TRUST DESCRIPTION CHECKER)) の並びで, 位置の順.
(define (org-lint--generate-reports texts checkers)
  (let* ((records (org-lint--scan texts))
         (ast (list texts records))
         (raw (apply append
                     (map (lambda (c)
                            (let ((trust (org-lint--checker-trust c)))
                              (map (lambda (report)
                                     (list (car report) (if trust (symbol->string trust) "nil")
                                           (cadr report) (org-lint--checker-name c)))
                                   ((org-lint--checker-fun c) ast))))
                          checkers)))
         (sorted (sort raw (lambda (a b) (< (car a) (car b))))))
    (let loop ((rest sorted) (id 1) (out '()))
      (if (null? rest)
          (reverse out)
          (let ((report (car rest)))
            (loop (cdr rest) (+ id 1)
                  (cons (list id (list (number->string (car (org-pos-line texts (car report))))
                                       (cadr report) (caddr report) (list-ref report 3)))
                        out)))))))

(define (org-lint--select arg)
  (cond ((not arg) org-lint--checkers)
        ((pair? arg) (filter (lambda (c) (member (org-lint--checker-name c) arg)) org-lint--checkers))
        (else (user-error (format "Invalid argument `%S' for `org-lint'" arg)))))

(define org-lint-buffer-name "*Org Lint*" "Name of the buffer that shows the reports of `org-lint'.")

(define org-lint--source #f)
(define org-lint--rows '())

(define (org-lint-reports . args)
  "Check the current Org buffer for syntax mistakes, and return the reports.
With a list of checker names (symbols) as the argument, run only those checkers.
Each report is (ID (LINE TRUST DESCRIPTION CHECKER)), sorted by line."
  (let ((arg (if (pair? args) (car args) #f)))
    (org-lint--generate-reports (org-lines) (org-lint--select arg))))

(define (org-lint--pad str width)
  (if (< (string-length str) width)
      (string-append str (org-repeat " " (- width (string-length str))))
      str))

;; REPORTS を *Org Lint* のバッファに出す.
(define (org-lint--display reports)
  (let ((source (buffer-name)))
    (switch-to-buffer org-lint-buffer-name)
    (delete-region (point-min) (point-max))
    (insert "Line  Trust  Warning  Checker\n")
    (for-each
     (lambda (report)
       (let ((v (cadr report)))
         (insert (org-lint--pad (list-ref v 0) 6) (org-lint--pad (list-ref v 1) 7)
                 (list-ref v 2) "  " (symbol->string (list-ref v 3)) "\n")))
     reports)
    (set-buffer-modified-p #f)
    (set! org-lint--source source)
    (set! org-lint--rows (map (lambda (report) (string->number (car (cadr report)))) reports))
    (local-set-key "RET" 'org-lint-goto)
    (goto-char (org-line-start (org-lines) 2))))

(define-command (org-lint . args)
  "Check the current Org buffer for syntax mistakes.
Show the reports in the *Org Lint* buffer; RET there moves to the line of the report.
With a list of checker names (symbols) as the argument, run only those checkers and return the reports."
  (cond ((pair? args) (org-lint-reports (car args)))
        (else
         (message "Org linting process starting...")
         (org-lint--display (org-lint-reports))
         (message "Org linting process completed"))))

(define-command (org-lint-goto)
  "Move to the line of the report at point."
  (let* ((row (- (org-current-line (org-lines)) 1))
         (line (and (>= row 1) (<= row (length org-lint--rows)) (list-ref org-lint--rows (- row 1)))))
    (unless line (user-error "No report on this line"))
    (switch-to-buffer org-lint--source)
    (goto-char (org-line-start (org-lines) line))))

(define org-lint-unsupported-checkers
  '(suspicious-language-in-src-block combining-keywords-with-affiliated
    invalid-babel-call-block wrong-header-argument wrong-header-value named-result empty-header-argument
    invalid-coderef-link invalid-id-link unknown-options-item misspelled-export-option
    invalid-macro-argument-and-template extraneous-element-in-footnote-section invalid-image-alignment
    mismatched-planning-repeaters non-existent-bibliography
    missing-print-bibliography invalid-cite-export-declaration incomplete-citation item-number
    LaTeX-$ beamer-frame timestamp-syntax)
  "Checkers of Emacs's org-lint that are not ported.
They need the syntax tree (links, timestamps, items), the exporter, Babel, org-id or org-cite.")

(provide 'org-lint)
