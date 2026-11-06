;;; hippie-exp.scm --- expand text trying various ways to find its expansion -*- scheme -*-

;; Emacs の hippie-exp.el に当たるもの (`hippie-expand`).
;; 位置の印 (marker) は, 今のバッファの中だけで使う. 他のバッファを探すときは, 位置を数で覚える.

(define hippie-expand-try-functions-list
  '(try-complete-file-name-partially
    try-complete-file-name
    try-expand-all-abbrevs
    try-expand-list
    try-expand-line
    try-expand-dabbrev
    try-expand-dabbrev-all-buffers
    try-expand-dabbrev-from-kill
    try-complete-lisp-symbol-partially
    try-complete-lisp-symbol)
  "The list of expansion functions tried in order by `hippie-expand'.
To change the behavior of `hippie-expand', remove, change the order of,
or insert functions in this list.")

(define hippie-expand-verbose #t
  "Non-nil makes `hippie-expand' output which function it is trying.")

(define hippie-expand-dabbrev-skip-space #f
  "Non-nil means tolerate trailing spaces in the abbreviation to expand.")

(define hippie-expand-dabbrev-as-symbol #t
  "Non-nil means expand as symbols, i.e. syntax `_' is considered a letter.")

(define hippie-expand-no-restriction #t
  "Non-nil means that narrowed buffers are widened during search.")

(define hippie-expand-max-buffers #f
  "The maximum number of buffers (apart from the current) searched.
If nil, all buffers are searched.")

(define hippie-expand-ignore-buffers '("^ \\*.*\\*$" dired-mode)
  "A list specifying which buffers not to search (if not current).
Can contain both regexps matching buffer names (as strings) and major modes (as atoms).")

(define hippie-expand-only-buffers '()
  "A list specifying the only buffers to search (in addition to current).
Can contain both regexps matching buffer names (as strings) and major modes (as atoms).
If non-nil, this variable overrides the variable `hippie-expand-ignore-buffers'.")

(define he-num -1)
(define he--buffer #f)
(define he-string-beg #f)
(define he-string-end #f)
(define he-search-loc #f)
(define he-search-string "")
(define he-expand-list '())
(define he-tried-table '())
(define he-search-loc2 #f)
(define he-search-bw #f)
(define he-search-bufs '())
(define he-searched-n-bufs 0)
(define he--search-pos 1)

(define (he--beg) (marker-position he-string-beg))
(define (he--end) (marker-position he-string-end))

(define (he--markers)
  "Make the markers `he-string-beg', `he-string-end' and `he-search-loc' in the current buffer."
  (if (not (equal? he--buffer (buffer-name)))
      (begin
        (if (and he--buffer (get-buffer he--buffer))
            (with-current-buffer he--buffer
              (marker-release he-string-beg)
              (marker-release he-string-end)
              (marker-release he-search-loc)))
        (set! he--buffer (buffer-name))
        (set! he-string-beg (copy-marker 1))
        (set! he-string-end (copy-marker 1))
        (set! he-search-loc (copy-marker 1)))))

(define-command (hippie-expand . args)
  "Try to expand text before point, using multiple methods.
The expansion functions in `hippie-expand-try-functions-list' are
tried in order, until a possible expansion is found.  Repeated
application of `hippie-expand' inserts successively possible
expansions.
With a positive numeric argument, jumps directly to the ARG next
function in this list.  With a negative argument or just \\[universal-argument],
undoes the expansion."
  (let ((arg (if (pair? args) (car args) (current-prefix-arg))))
    (if (or (not arg) (and (number? arg) (> arg 0)))
        (let ((first (or (= he-num -1) (not (eq? this-command last-command))))
              (count (length hippie-expand-try-functions-list)))
          (when first
            (set! he-num -1)
            (set! he-tried-table '()))
          (if arg
              (if (not first) (he-reset-string))
              (set! arg 0))
          (let loop ((i (max (+ he-num arg) 0)))
            (if (or (>= i count)
                    (let ((f (list-ref hippie-expand-try-functions-list i)))
                      ((if (symbol? f) (eval f) f) (= he-num i))))
                (set! he-num i)
                (loop (+ i 1))))
          (if (>= he-num count)
              (begin
                (set! he-num -1)
                (message (if first "No expansion found" "No further expansions found"))
                (ding))
              (if hippie-expand-verbose
                  (message "Using %s" (list-ref hippie-expand-try-functions-list he-num)))))
        (when (and (>= he-num 0) (equal? he--buffer (buffer-name)))
          (set! he-num -1)
          (he-reset-string)
          (if hippie-expand-verbose
              (message "Undoing expansions"))))
    #f))

(define (he-init-string beg end)
  "Initialize the string to expand: the text from BEG to END."
  (he--markers)
  (marker-set he-string-beg beg)
  (marker-set he-string-end end)
  (set! he-search-string (buffer-substring beg end)))

(define (he-reset-string)
  "Put back the string being expanded."
  (let ((newpos (point-marker)))
    (goto-char (he--beg))
    (insert he-search-string)
    (delete-region (point) (he--end))
    (goto-char (marker-position newpos))
    (marker-release newpos)))

(define (he-substitute-string str . rest)
  "Replace the string being expanded with STR, transferring its case if TRANS-CASE."
  (let ((trans-case (and (pair? rest) (car rest) case-replace case-fold-search))
        (newpos (point-marker)))
    (goto-char (he--beg))
    (let ((subst (if trans-case (he-transfer-case he-search-string str) str)))
      (set! he-tried-table (cons subst he-tried-table))
      (insert subst)
      (delete-region (point) (he--end))
      (goto-char (marker-position newpos))
      (marker-release newpos))))

(define (he-capitalize-first str)
  "Return STR in lower case, with its first word character in upper case."
  (let ((i (string-match "\\sw" str)))
    (if i
        (string-append (string-downcase (substring str 0 i))
                       (string-upcase (substring str i (+ i 1)))
                       (string-downcase (substring str (+ i 1))))
        str)))

(define (he-ordinary-case-p str)
  (or (equal? str (string-downcase str))
      (equal? str (string-upcase str))
      (equal? str (dabbrev--capitalize str))
      (equal? str (he-capitalize-first str))))

(define (he-transfer-case from-str to-str)
  "Return TO-STR in the case of FROM-STR."
  (cond ((equal? from-str (substring to-str 0 (min (string-length from-str) (string-length to-str))))
         to-str)
        ((not (he-ordinary-case-p to-str)) to-str)
        ((equal? from-str (string-downcase from-str)) (string-downcase to-str))
        ((equal? from-str (string-upcase from-str)) (string-upcase to-str))
        ((equal? from-str (he-capitalize-first from-str)) (he-capitalize-first to-str))
        ((equal? from-str (dabbrev--capitalize from-str)) (dabbrev--capitalize to-str))
        (else to-str)))

(define (he-string-member str lst . rest)
  "Return non-nil if STR (in the case of the string to expand, if TRANS-CASE) is in LST."
  (and str
       (member (if (and (pair? rest) (car rest) case-replace case-fold-search)
                   (he-transfer-case he-search-string str)
                   str)
               lst)))

(define (he-buffer-member lst)
  "Return non-nil if the current buffer's major mode or name is matched by LST."
  (or (member major-mode lst)
      (let loop ((lst lst))
        (cond ((null? lst) #f)
              ((and (string? (car lst)) (string-match (car lst) (buffer-name))) lst)
              (else (loop (cdr lst)))))))

(define (he--result expansion old . rest)
  "Substitute EXPANSION (transferring the case if TRANS-CASE) and return #t; or reset the string and return #f."
  (if expansion
      (begin (he-substitute-string expansion (and (pair? rest) (car rest))) #t)
      (begin (if old (he-reset-string)) #f)))

;;; File names

(define he-file-name-chars "-a-zA-Z0-9_/.,~^#$+="
  "Characters that may be part of a file name.")

(define (he-file-name-beg)
  (let ((op (point)))
    (save-excursion
      (skip-chars-backward he-file-name-chars)
      (if (> (skip-syntax-backward "w") 0)
          op
          (point)))))

(define (he--file-name-all-completions name dir)
  "Return the names of the files in DIR starting with NAME (directories end in a slash)."
  (map (lambda (file) (if (file-directory? (expand-file-name file dir)) (string-append file "/") file))
       (filter (lambda (file) (string-prefix? name file)) (directory-files dir))))

(define (he--file-name-completion name dir)
  "Return the longest common prefix of the files in DIR starting with NAME, #t for a unique exact match, or #f."
  (let ((all (he--file-name-all-completions name dir)))
    (cond ((null? all) #f)
          ((and (null? (cdr all)) (equal? (car all) name)) #t)
          (else (completion--common-prefix all)))))

(define (he--file-name-parts)
  "Return the name part and the directory part of `he-search-string'."
  (list (file-name-nondirectory he-search-string)
        (expand-file-name (or (file-name-directory he-search-string) ""))))

(define (try-complete-file-name old)
  "Try to complete text as a file name.
The argument OLD has to be nil the first call of this function, and t
for subsequent calls (for further possible completions of the same
string).  It returns t if a new completion is found, nil otherwise."
  (when (not old)
    (he-init-string (he-file-name-beg) (point))
    (let* ((parts (he--file-name-parts))
           (name-part (car parts))
           (dir-part (cadr parts)))
      (if (not (he-string-member name-part he-tried-table))
          (set! he-tried-table (cons name-part he-tried-table)))
      (set! he-expand-list
            (if (and (not (equal? he-search-string "")) (file-directory? dir-part))
                (sort (he--file-name-all-completions name-part dir-part) string<?)
                '()))))
  (let loop ()
    (when (and (pair? he-expand-list) (he-string-member (car he-expand-list) he-tried-table))
      (set! he-expand-list (cdr he-expand-list))
      (loop)))
  (if (null? he-expand-list)
      (he--result #f old)
      (let ((filename (string-append (or (file-name-directory he-search-string) "") (car he-expand-list))))
        (he-substitute-string filename)
        (set! he-tried-table (cons (car he-expand-list) (cdr he-tried-table)))
        (set! he-expand-list (cdr he-expand-list))
        #t)))

(define (try-complete-file-name-partially old)
  "Try to complete text as a file name, as many characters as unique.
The argument OLD has to be nil the first call of this function.  It
returns t if a unique, possibly partial, completion is found, nil
otherwise."
  (let ((expansion #f))
    (when (not old)
      (he-init-string (he-file-name-beg) (point))
      (let* ((parts (he--file-name-parts))
             (name-part (car parts))
             (dir-part (cadr parts)))
        (if (and (not (equal? he-search-string "")) (file-directory? dir-part))
            (set! expansion (he--file-name-completion name-part dir-part)))
        (if (or (eq? expansion #t)
                (equal? expansion name-part)
                (he-string-member expansion he-tried-table))
            (set! expansion #f))))
    (if (not expansion)
        (he--result #f old)
        (begin
          (he-substitute-string (string-append (or (file-name-directory he-search-string) "") expansion))
          (set! he-tried-table (cons expansion (cdr he-tried-table)))
          #t))))

;;; Lisp symbols

(define (he-lisp-symbol-beg)
  (save-excursion
    (skip-syntax-backward "w_")
    (point)))

(define (he--lisp-symbols prefix)
  "Return the names of the defined symbols starting with PREFIX, sorted."
  (let ((names '()))
    (mapatoms (lambda (symbol)
                (let ((name (symbol->string symbol)))
                  (if (string-prefix? prefix name)
                      (set! names (cons name names))))))
    (sort names string<?)))

(define (try-complete-lisp-symbol old)
  "Try to complete word as an Emacs Lisp symbol.
The argument OLD has to be nil the first call of this function, and t
for subsequent calls (for further possible completions of the same
string).  It returns t if a new completion is found, nil otherwise."
  (when (not old)
    (he-init-string (he-lisp-symbol-beg) (point))
    (if (not (he-string-member he-search-string he-tried-table))
        (set! he-tried-table (cons he-search-string he-tried-table)))
    (set! he-expand-list (if (equal? he-search-string "") '() (he--lisp-symbols he-search-string))))
  (let loop ()
    (when (and (pair? he-expand-list) (he-string-member (car he-expand-list) he-tried-table))
      (set! he-expand-list (cdr he-expand-list))
      (loop)))
  (if (null? he-expand-list)
      (he--result #f old)
      (let ((expansion (car he-expand-list)))
        (set! he-expand-list (cdr he-expand-list))
        (he--result expansion old))))

(define (try-complete-lisp-symbol-partially old)
  "Try to complete as an Emacs Lisp symbol, as many characters as unique.
The argument OLD has to be nil the first call of this function.  It
returns t if a unique, possibly partial, completion is found, nil
otherwise."
  (let ((expansion #f))
    (when (not old)
      (he-init-string (he-lisp-symbol-beg) (point))
      (if (not (equal? he-search-string ""))
          (let ((all (he--lisp-symbols he-search-string)))
            (set! expansion (cond ((null? all) #f)
                                  ((and (null? (cdr all)) (equal? (car all) he-search-string)) #t)
                                  (else (completion--common-prefix all))))))
      (if (or (eq? expansion #t)
              (equal? expansion he-search-string)
              (he-string-member expansion he-tried-table))
          (set! expansion #f)))
    (he--result expansion old)))

;;; Searching from the string to expand

(define (he--search-around old beg search)
  "Search backward from the string to expand with SEARCH, then forward.
BEG is the start of the string on the first call (OLD is false)."
  (let ((expansion #f))
    (when (not old)
      (he-init-string beg (point))
      (marker-set he-search-loc (he--beg))
      (set! he-search-bw #t))
    (if (not (equal? he-search-string ""))
        (save-excursion
          (save-restriction
            (if hippie-expand-no-restriction (widen))
            (when he-search-bw
              (goto-char (marker-position he-search-loc))
              (set! expansion (search he-search-string #t))
              (marker-set he-search-loc (point))
              (when (not expansion)
                (marker-set he-search-loc (he--end))
                (set! he-search-bw #f)))
            (when (not expansion)
              (goto-char (marker-position he-search-loc))
              (set! expansion (search he-search-string #f))
              (marker-set he-search-loc (point))))))
    (he--result expansion old #t)))

(define (he--all-buffers old beg-function search-function)
  "Search the other buffers with SEARCH-FUNCTION for an expansion of the text from BEG-FUNCTION."
  (let ((expansion #f)
        (buf (buffer-name))
        (orig-case-fold-search case-fold-search))
    (when (not old)
      (he-init-string (beg-function) (point))
      (set! he-search-bufs (buffer-list))
      (set! he-searched-n-bufs 0)
      (set! he--search-pos 1))
    (if (not (equal? he-search-string ""))
        (let loop ()
          (when (and (pair? he-search-bufs)
                     (not expansion)
                     (or (not hippie-expand-max-buffers)
                         (< he-searched-n-bufs hippie-expand-max-buffers)))
            (with-current-buffer (car he-search-bufs)
              (if (and (not (equal? (buffer-name) buf))
                       (if (pair? hippie-expand-only-buffers)
                           (he-buffer-member hippie-expand-only-buffers)
                           (not (he-buffer-member hippie-expand-ignore-buffers))))
                  (save-excursion
                    (save-restriction
                      (if hippie-expand-no-restriction (widen))
                      (goto-char he--search-pos)
                      (let ((saved case-fold-search))
                        (set! case-fold-search orig-case-fold-search)
                        (set! expansion (search-function he-search-string))
                        (set! case-fold-search saved))
                      (set! he--search-pos (point))
                      (when (not expansion)
                        (set! he-search-bufs (cdr he-search-bufs))
                        (set! he-searched-n-bufs (+ he-searched-n-bufs 1))
                        (set! he--search-pos 1))))
                  (begin
                    (set! he-search-bufs (cdr he-search-bufs))
                    (set! he--search-pos 1))))
            (loop))))
    (he--result expansion old #t)))

;;; Lines

(define (he-line-search-regexp pat)
  (string-append "^\\(\\s-*\\)\\(" (regexp-quote pat) "[^\n]*[^ \t\n]\\)"))

(define (he-line-beg)
  (save-excursion
    (if (re-search-backward (he-line-search-regexp "") (line-beginning-position) #t)
        (match-beginning 2)
        (point))))

(define (he-line-search str reverse)
  (let loop ()
    (if (if reverse
            (re-search-backward (he-line-search-regexp str) #f #t)
            (re-search-forward (he-line-search-regexp str) #f #t))
        (let ((result (buffer-substring (match-end 1) (match-end 0))))
          (if (he-string-member result he-tried-table #t)
              (loop)
              result))
        #f)))

(define (try-expand-line old)
  "Try to complete the current line to an entire line in the buffer.
The argument OLD has to be nil the first call of this function, and t
for subsequent calls (for further possible completions of the same
string).  It returns t if a new completion is found, nil otherwise."
  (he--search-around old (if old #f (he-line-beg)) he-line-search))

(define (try-expand-line-all-buffers old)
  "Try to complete the current line, searching all other buffers.
The argument OLD has to be nil the first call of this function, and t
for subsequent calls (for further possible completions of the same
string).  It returns t if a new completion is found, nil otherwise."
  (he--all-buffers old he-line-beg (lambda (string) (he-line-search string #f))))

;;; Lists

(define (he-list-search str . rest)
  (let ((reverse (and (pair? rest) (car rest))))
    (let loop ()
      (if (if reverse (search-backward str #f #t) (search-forward str #f #t))
          (let* ((pos (point))
                 (beg (match-beginning 0))
                 (end (condition-case e (or (scan-lists beg 1 0) (point-max)) (error #f)))
                 (result (and end
                              (not (and reverse (> end (he--beg))))
                              (buffer-substring beg end))))
            (goto-char pos)
            (if (and result (not (he-string-member result he-tried-table #t)))
                result
                (loop)))
          #f))))

(define (he-list-beg)
  (save-excursion
    (condition-case e
        (goto-char (up-list-position (point) -1))
      (error #f))
    (point)))

(define (try-expand-list old)
  "Try to complete the current beginning of a list.
The argument OLD has to be nil the first call of this function, and t
for subsequent calls (for further possible completions of the same
string).  It returns t if a new completion is found, nil otherwise."
  (he--search-around old (if old #f (he-list-beg)) he-list-search))

(define (try-expand-list-all-buffers old)
  "Try to complete the current list, searching all other buffers.
The argument OLD has to be nil the first call of this function, and t
for subsequent calls (for further possible completions of the same
string).  It returns t if a new completion is found, nil otherwise."
  (he--all-buffers old he-list-beg he-list-search))

;;; Abbrevs

(define (try-expand-all-abbrevs old)
  "Try to expand word before point according to all abbrev tables.
The argument OLD has to be nil the first call of this function, and t
for subsequent calls (for further possible expansions of the same
string).  It returns t if a new expansion is found, nil otherwise."
  (when (not old)
    (he-init-string (he-dabbrev-beg) (point))
    (set! he-expand-list
          (if (equal? he-search-string "")
              '()
              (map (lambda (table) (abbrev-expansion (string-downcase he-search-string) table))
                   (list local-abbrev-table global-abbrev-table)))))
  (let loop ()
    (when (and (pair? he-expand-list)
               (or (not (car he-expand-list))
                   (he-string-member (car he-expand-list) he-tried-table #t)))
      (set! he-expand-list (cdr he-expand-list))
      (loop)))
  (if (null? he-expand-list)
      (he--result #f old)
      (let ((expansion (car he-expand-list)))
        (set! he-expand-list (cdr he-expand-list))
        (he--result expansion old #t))))

;;; Dynamic abbreviations

(define (he--dabbrev-regexp pattern)
  (cond ((not hippie-expand-dabbrev-as-symbol)
         (string-append "\\<" (regexp-quote pattern) "\\sw+"))
        ((equal? (char-syntax (substring pattern 0 1)) "_")
         (string-append (regexp-quote pattern) "\\(\\sw\\|\\s_\\)+"))
        (else (string-append "\\<" (regexp-quote pattern) "\\(\\sw\\|\\s_\\)+"))))

(define (he--symbol-char? c)
  (member (char-syntax c) '("_" "w")))

(define (he-dabbrev-search pattern . rest)
  (let ((reverse (and (pair? rest) (car rest)))
        (regpat (he--dabbrev-regexp pattern)))
    (let loop ()
      (if (if reverse (re-search-backward regpat #f #t) (re-search-forward regpat #f #t))
          (let ((result (buffer-substring (match-beginning 0) (match-end 0))))
            (if (or (and hippie-expand-dabbrev-as-symbol
                         (> (match-beginning 0) (point-min))
                         (he--symbol-char? (char-after (- (match-beginning 0) 1))))
                    (he-string-member result he-tried-table #t))
                (loop)
                result))
          #f))))

(define (he-dabbrev-beg)
  (let ((op (point)))
    (save-excursion
      (if hippie-expand-dabbrev-skip-space
          (skip-syntax-backward ". "))
      (if (= (skip-syntax-backward (if hippie-expand-dabbrev-as-symbol "w_" "w")) 0)
          op
          (point)))))

(define (try-expand-dabbrev old)
  "Try to expand word \"dynamically\", searching the current buffer.
The argument OLD has to be nil the first call of this function, and t
for subsequent calls (for further possible expansions of the same
string).  It returns t if a new expansion is found, nil otherwise."
  (he--search-around old (if old #f (he-dabbrev-beg)) he-dabbrev-search))

(define (try-expand-dabbrev-all-buffers old)
  "Try to expand word \"dynamically\", searching all other buffers.
The argument OLD has to be nil the first call of this function, and t
for subsequent calls (for further possible expansions of the same
string).  It returns t if a new expansion is found, nil otherwise."
  (he--all-buffers old he-dabbrev-beg he-dabbrev-search))

(define (he--kill-ring)
  "Return the strings in the kill ring, the most recent first."
  (let loop ((i (- (kill-ring-length) 1)) (kills '()))
    (if (< i 0)
        kills
        (loop (- i 1) (cons (simple--current-kill i #t) kills)))))

(define (he-dabbrev-kill-search pattern)
  (let ((regpat (he--dabbrev-regexp pattern)))
    (let loop ()
      (cond ((null? he-expand-list) #f)
            ((string-match regpat (car he-expand-list) he-search-loc2)
             (let* ((killstr (car he-expand-list))
                    (start (match-beginning 0))
                    (result (substring killstr start (match-end 0))))
               (set! he-search-loc2 (+ start 1))
               (if (or (and hippie-expand-dabbrev-as-symbol
                            (> start 0)
                            (he--symbol-char? (substring killstr (- start 1) start)))
                       (he-string-member result he-tried-table #t))
                   (loop)
                   result)))
            (else
             (set! he-expand-list (cdr he-expand-list))
             (set! he-search-loc2 0)
             (loop))))))

(define (try-expand-dabbrev-from-kill old)
  "Try to expand word \"dynamically\", searching the kill ring.
The argument OLD has to be nil the first call of this function, and t
for subsequent calls (for further possible completions of the same
string).  It returns t if a new completion is found, nil otherwise."
  (when (not old)
    (he-init-string (he-dabbrev-beg) (point))
    (set! he-expand-list (if (equal? he-search-string "") '() (he--kill-ring)))
    (set! he-search-loc2 0))
  (he--result (and (not (equal? he-search-string "")) (he-dabbrev-kill-search he-search-string))
              old #t))
