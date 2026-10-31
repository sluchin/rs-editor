;;; isearch.scm --- incremental search -*- scheme -*-

;; Emacs の isearch.el に当たるもの. Emacs はコマンドループで動くが, ここでは, `read-key` で 1 つずつキーを読む手続きにする.
;; 状態 (`isearch-cmds`) の積み方, 探す位置, 失敗したときの戻り方は, Emacs 30 の `isearch-search`, `isearch-search-and-update`,
;; `isearch-repeat`, `isearch-pop-state` と同じ. `replace.scm` を先に読む.

(define isearch-ring '()
  "List of the previous search strings (the newest first).")

(define isearch-regexp-ring '()
  "List of the previous regexp search strings (the newest first).")

(define isearch--string "")
(define isearch--forward #t)
(define isearch--regexp #f)
(define isearch--function #f)
(define isearch--success #t)
(define isearch--other-end #f)
(define isearch--barrier 1)
(define isearch--opoint 1)
(define isearch--wrapped #f)
(define isearch--just-started #t)
(define isearch--adjusted #f)
(define isearch--yank-flag #f)
(define isearch--error #f)
(define isearch--fold #t)
(define isearch--cmds '())

;;; 状態

(define (isearch--get-state)
  (list isearch--string (point) isearch--success isearch--forward isearch--other-end
        isearch--wrapped isearch--barrier isearch--error isearch--fold isearch--function isearch--regexp))

(define (isearch--set-state st)
  (set! isearch--string (list-ref st 0))
  (goto-char (list-ref st 1))
  (set! isearch--success (list-ref st 2))
  (set! isearch--forward (list-ref st 3))
  (set! isearch--other-end (list-ref st 4))
  (set! isearch--wrapped (list-ref st 5))
  (set! isearch--barrier (list-ref st 6))
  (set! isearch--error (list-ref st 7))
  (set! isearch--fold (list-ref st 8))
  (set! isearch--function (list-ref st 9))
  (set! isearch--regexp (list-ref st 10)))

(define (isearch--push-state)
  (set! isearch--cmds (cons (isearch--get-state) isearch--cmds)))

(define (isearch--pop-state)
  (set! isearch--cmds (cdr isearch--cmds))
  (isearch--set-state (car isearch--cmds)))

;;; 探す

(define (isearch--words string)
  (filter (lambda (w) (not (equal? w ""))) (split-string string "\\W+")))

(define (isearch--pattern)
  "Return the regexp to search for, or #f to search for the string itself."
  (cond ((eq? isearch--function 'symbol)
         (string-append "\\_<" (if isearch--regexp isearch--string (regexp-quote isearch--string)) "\\_>"))
        ((eq? isearch--function 'word)
         (string-append "\\b" (string-join (map regexp-quote (isearch--words isearch--string)) "\\W+") "\\b"))
        (isearch--regexp isearch--string)
        (else #f)))

(define (isearch--search-string)
  "Search from point for the search string.  Return #t (point moved to the match) or #f."
  (let ((pattern (isearch--pattern)))
    (condition-case e
        (cond ((and pattern isearch--forward) (re-search-forward pattern #f #t))
              (pattern (re-search-backward pattern #f #t))
              (isearch--forward (search-forward isearch--string #f #t))
              (else (search-backward isearch--string #f #t)))
      (error (set! isearch--error "incomplete input") 'error))))

(define (isearch--search)
  "Do the search with the current search string."
  (let ((saved case-fold-search)
        (found #f))
    (set! isearch--fold (and case-fold-search (replace--no-upper-p isearch--string isearch--regexp)))
    (set! case-fold-search isearch--fold)
    (set! isearch--error #f)
    (set! found (isearch--search-string))
    (set! case-fold-search saved)
    ;; 正規表現が誤っていれば, 成功のままにして, 何も変えない (Emacs は, 本体を途中でやめる)
    (when (not (eq? found 'error))
      (set! isearch--success (if found #t #f))
      (set! isearch--just-started #f)
      (if isearch--success
          (set! isearch--other-end (if isearch--forward (match-beginning 0) (match-end 0)))))
    (if (not isearch--success)
        ;; 失敗したら, 前の成功の所へ戻る
        (goto-char (list-ref (car isearch--cmds) 1)))))

(define (isearch--search-and-update)
  (when (or isearch--success isearch--regexp)
    (if (and (not isearch--forward) (not isearch--adjusted)
             (isearch--looking-at)
             (or isearch--yank-flag (<= (match-end 0) (min isearch--opoint isearch--barrier))))
        (begin
          (set! isearch--success #t)
          (set! isearch--error #f)
          (set! isearch--other-end (match-end 0)))
        (begin
          (if (and isearch--other-end (not isearch--adjusted))
              (goto-char (if isearch--forward
                             isearch--other-end
                             (min isearch--opoint isearch--barrier (+ isearch--other-end 1)))))
          (isearch--search))))
  (isearch--push-state))

(define (isearch--looking-at)
  "Return #t if the search string matches at point."
  (let ((saved case-fold-search)
        (pattern (or (isearch--pattern) (regexp-quote isearch--string))))
    (set! case-fold-search (and case-fold-search (replace--no-upper-p isearch--string isearch--regexp)))
    (let ((result (condition-case e (looking-at pattern) (error #f))))
      (set! case-fold-search saved)
      result)))

(define (isearch--repeat forward)
  (if (eq? isearch--forward forward)
      (if (equal? isearch--string "")
          (let ((ring (if isearch--regexp isearch-regexp-ring isearch-ring)))
            (if (null? ring)
                (set! isearch--error "No previous search string")
                (set! isearch--string (car ring))))
          (if (not isearch--success)
              (begin
                (set! isearch--wrapped #t)
                (goto-char (if isearch--forward (point-min) (point-max))))))
      (begin
        (set! isearch--forward (not isearch--forward))
        (set! isearch--success #t)))
  (set! isearch--barrier (point))
  (if (equal? isearch--string "")
      (set! isearch--success #t)
      (if (and isearch--success (equal? (point) isearch--other-end) (not isearch--just-started))
          (if (if isearch--forward (eobp) (bobp))
              (set! isearch--success #f)
              (begin
                (if isearch--forward (forward-char 1) (backward-char 1))
                (isearch--search)))
          (isearch--search)))
  (isearch--push-state))

;;; 入力

(define (isearch--odd-backslashes string)
  "Return #t if STRING ends in an odd number of backslashes."
  (let loop ((i (string-length string)) (n 0))
    (if (and (> i 0) (equal? (substring string (- i 1) i) "\\"))
        (loop (- i 1) (+ n 1))
        (= (modulo n 2) 1))))

(define (isearch--fallback want-backslash allow-invalid to-barrier)
  "Return point to the previous successful match so that a regexp can be made liberal (Emacs isearch-fallback)."
  (when (and isearch--regexp
             (eq? (not (isearch--odd-backslashes isearch--string)) (not want-backslash))
             (or (not isearch--error) (not (list-ref (cadr isearch--cmds) 7)) allow-invalid))
    (if to-barrier
        (begin (goto-char isearch--barrier) (set! isearch--adjusted #t))
        (let loop ((stack isearch--cmds))
          (let ((previous (cdr stack))
                (frame (car stack)))
            (if (and (pair? previous)
                     (or (list-ref frame 7)
                         (let* ((string (list-ref frame 0))
                                (last (substring string (- (string-length string) 1) (string-length string))))
                           (if (isearch--odd-backslashes (list-ref (car previous) 0))
                               (equal? last "}")
                               (member last '("*" "?" "+"))))))
                (loop previous)
                (let ((last-other-end (or (and (pair? previous) (list-ref (car previous) 4)) isearch--barrier)))
                  (goto-char (if isearch--forward (max last-other-end isearch--barrier) (min last-other-end isearch--barrier)))
                  (set! isearch--adjusted #t))))))))

(define (isearch--add string)
  (cond ((member string '("*" "?")) (isearch--fallback #f #f #f))
        ((equal? string "}") (isearch--fallback #t #t #f))
        ((equal? string "|") (isearch--fallback #t #f #t)))
  (set! isearch--string (string-append isearch--string string))
  (isearch--search-and-update))

(define (isearch--delete-char)
  (if (not (null? (cdr isearch--cmds)))
      (isearch--pop-state)))

(define (isearch--yank-word)
  "Add the word after point (or after the current match) to the search string."
  (let ((text (save-excursion
                (if (and (not isearch--forward) isearch--other-end) (goto-char isearch--other-end))
                (let ((start (point)))
                  (forward-word 1)
                  (buffer-substring start (point))))))
    (set! isearch--yank-flag #t)
    (isearch--add text)
    (set! isearch--yank-flag #f)))

(define (isearch--update-ring)
  (when (not (equal? isearch--string ""))
    (let ((ring (if isearch--regexp isearch-regexp-ring isearch-ring)))
      (let ((new (cons isearch--string (filter (lambda (s) (not (equal? s isearch--string))) ring))))
        (if isearch--regexp (set! isearch-regexp-ring new) (set! isearch-ring new))))))

(define (isearch--done nopush)
  (if (not nopush) (isearch--update-ring))
  (if (and (not (= (point) isearch--opoint)) (not (use-region-p)))
      (begin
        (push-mark isearch--opoint)
        (message "Mark saved where search started"))))

(define (isearch--exit)
  (if (equal? isearch--string "")
      ;; 空のまま終えると, 文字列を聞いて探す (nonincremental search)
      (let ((string (read-string "Search: ")))
        (when (not (equal? string ""))
          (goto-char isearch--opoint)
          (set! isearch--string string)
          (set! isearch--success #t)
          (isearch--search-and-update))
        (isearch--done #f))
      (isearch--done #f)))

(define (isearch--abort)
  (if (and isearch--success (not isearch--error))
      (begin
        (goto-char isearch--opoint)
        (error "Quit"))
      (begin
        (let loop ()
          (when (and (not (null? (cdr isearch--cmds))) (or (not isearch--success) isearch--error))
            (isearch--pop-state)
            (loop)))
        'continue)))

(define (isearch--prompt)
  (let* ((prefix (string-append (if (not isearch--success) "failing " "")
                                (if (eq? isearch--adjusted #t) "pending " "")
                                (if isearch--wrapped "wrapped " "")
                                (if isearch--regexp "regexp " "")
                                (cond ((eq? isearch--function 'symbol) "symbol ") ((eq? isearch--function 'word) "word ") (else ""))
                                "I-search" (if isearch--forward "" " backward") ": "))
         (head (string-upcase (substring prefix 0 1))))
    (string-append head (substring prefix 1 (string-length prefix)) isearch--string
                   (if isearch--error (string-append " [" isearch--error "]") ""))))

(define (isearch--printing-key key)
  "Return the string to add to the search string for KEY, or #f if KEY is not a printing key."
  (cond ((equal? key "Tab") "\t")
        ((= (string-length key) 1) key)
        (else #f)))

(define (isearch-mode forward regexp function . initial)
  "Search incrementally.  FORWARD is the direction, REGEXP #t for a regexp search, FUNCTION #f, `word' or `symbol'.
The keys: C-s and C-r repeat the search forward or backward, DEL deletes the last character,
C-w adds the word at point, C-q quotes the next key, M-r toggles the regexp mode,
RET exits, C-g backs up to the last match or, if it was found, cancels the search.
Another key exits the search, and is ignored."
  (set! isearch--forward forward)
  (set! isearch--regexp regexp)
  (set! isearch--function function)
  (set! isearch--string "")
  (set! isearch--cmds '())
  (set! isearch--success #t)
  (set! isearch--wrapped #f)
  (set! isearch--barrier (point))
  (set! isearch--adjusted #f)
  (set! isearch--yank-flag #f)
  (set! isearch--error #f)
  (set! isearch--other-end #f)
  (set! isearch--just-started #t)
  (set! isearch--opoint (point))
  (isearch--push-state)
  (if (pair? initial) (isearch--add (car initial)))
  (let loop ()
    (let ((key (read-key (isearch--prompt) #t)))
      (cond ((equal? key "C-s") (isearch--repeat #t) (loop))
            ((equal? key "C-r") (isearch--repeat #f) (loop))
            ((member key '("Backspace" "Delete")) (isearch--delete-char) (loop))
            ((equal? key "Enter") (isearch--exit))
            ((equal? key "C-g") (if (eq? (isearch--abort) 'continue) (loop)))
            ((equal? key "C-w") (isearch--yank-word) (loop))
            ((equal? key "C-q") (isearch--add (read-key "Quote: ")) (loop))
            ((equal? key "M-r")
             (set! isearch--regexp (not isearch--regexp))
             (isearch--search-and-update)
             (loop))
            ((isearch--printing-key key) (isearch--add (isearch--printing-key key)) (loop))
            (else (isearch--done #f))))))

(define-command (isearch-forward . args)
  "Do incremental search forward.  With a prefix argument, do a regexp search."
  (isearch-mode #t (if (null? args) (and (current-prefix-arg) #t) (car args)) #f))

(define-command (isearch-backward . args)
  "Do incremental search backward.  With a prefix argument, do a regexp search."
  (isearch-mode #f (if (null? args) (and (current-prefix-arg) #t) (car args)) #f))

(define-command (isearch-forward-regexp . args)
  "Do incremental search forward for a regexp."
  (isearch-mode #t #t #f))

(define-command (isearch-backward-regexp . args)
  "Do incremental search backward for a regexp."
  (isearch-mode #f #t #f))

(define-command (isearch-forward-word . args)
  "Do incremental search forward for a sequence of words, ignoring punctuation."
  (isearch-mode #t #f 'word))

(define-command (isearch-forward-symbol . args)
  "Do incremental search forward for a symbol."
  (isearch-mode #t #f 'symbol))

(define-command (isearch-forward-symbol-at-point . args)
  "Do incremental search forward for the symbol found near point."
  (let ((symbol (hi-lock--symbol-at-point)))
    (if (not symbol)
        (error "No symbol at point")
        (isearch-mode #t #f 'symbol symbol))))

(provide 'isearch)

;;; isearch.scm ends here
