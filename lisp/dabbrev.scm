;;; dabbrev.scm --- dynamic abbreviation expansion -*- scheme -*-

;; Emacs の dabbrev.el に当たるもの (`dabbrev-expand`).
;; 探すのは今のバッファだけ (他のバッファと, 展開の後に空白を入れて続ける機能は無い).

(define dabbrev-backward-only #f
  "If non-nil, `dabbrev-expand' only looks backwards.")

(define dabbrev-case-replace 'case-replace
  "Controls whether dabbrev preserves case when expanding the abbreviation.")

(define dabbrev--abbrev-char-regexp "\\sw\\|\\s_")
(define dabbrev--last-table '())
(define dabbrev--last-abbreviation #f)
(define dabbrev--last-abbrev-location #f)
(define dabbrev--last-direction #f)
(define dabbrev--last-expansion #f)
(define dabbrev--last-expansion-location #f)

(define (dabbrev--location)
  "Return the position of `dabbrev--last-expansion-location' (a number, or a marker kept across the edit)."
  (if (pair? dabbrev--last-expansion-location)
      (marker-position (cadr dabbrev--last-expansion-location))
      dabbrev--last-expansion-location))

(define (dabbrev--reset-global-variables)
  (if (pair? dabbrev--last-expansion-location)
      (marker-release (cadr dabbrev--last-expansion-location)))
  (set! dabbrev--last-table '())
  (set! dabbrev--last-abbreviation #f)
  (set! dabbrev--last-abbrev-location #f)
  (set! dabbrev--last-direction #f)
  (set! dabbrev--last-expansion #f)
  (set! dabbrev--last-expansion-location #f))

(define (dabbrev--ignore-case-p abbrev)
  "Return non-nil if case should be ignored when searching for ABBREV."
  case-fold-search)

(define (dabbrev--goto-start-of-abbrev)
  "Back over all abbrev type characters."
  (if (> (point) (point-min))
      (begin
        (backward-char 1)
        (let loop ()
          (if (and (looking-at dabbrev--abbrev-char-regexp) (> (point) (point-min)))
              (begin (backward-char 1) (loop))))
        (if (not (looking-at dabbrev--abbrev-char-regexp))
            (forward-char 1)))))

(define (dabbrev--abbrev-at-point)
  "Extract the symbol before point to serve as abbreviation."
  (if (bobp)
      (user-error "No possible abbreviation preceding point"))
  (let ((end (point)))
    (save-excursion
      (backward-char 1)
      (if (not (looking-at dabbrev--abbrev-char-regexp))
          (if (re-search-backward dabbrev--abbrev-char-regexp #f #t)
              (forward-char 1)
              (user-error "No possible abbreviation preceding point")))
      (dabbrev--goto-start-of-abbrev)
      (buffer-substring (point) end))))

(define (dabbrev--search abbrev reverse ignore-case)
  "Search for something that could be used to expand ABBREV, leaving point at its end."
  (let ((pattern1 (string-append (regexp-quote abbrev) "\\(" dabbrev--abbrev-char-regexp "\\)"))
        (pattern2 (string-append (regexp-quote abbrev) "\\(\\(" dabbrev--abbrev-char-regexp "\\)+\\)"))
        (found #f)
        (result #f))
    (let loop ()
      (if (and (not found)
               (if reverse
                   (re-search-backward pattern1 #f #t)
                   (re-search-forward pattern1 #f #t)))
          (let ((begin-of-match (match-beginning 0))
                (end-of-match (match-end 0)))
            (goto-char begin-of-match)
            (dabbrev--goto-start-of-abbrev)
            (if (looking-at pattern1)
                (begin
                  (re-search-forward pattern2)
                  (set! result (match-string 0))
                  (set! found (if ignore-case (string-downcase result) result))
                  (if (member found dabbrev--last-table)
                      (set! found #f))))
            (goto-char (if reverse begin-of-match end-of-match))
            (loop))))
    (and found
         (begin
           (set! dabbrev--last-table (cons found dabbrev--last-table))
           result))))

(define (dabbrev--try-find abbrev reverse n ignore-case)
  "Search for ABBREV, backwards if REVERSE, N times.  Return the expansion found."
  (save-excursion
    (let ((expansion #f)
          (saved case-fold-search))
      (if dabbrev--last-expansion-location
          (goto-char (dabbrev--location)))
      (set! case-fold-search ignore-case)
      (let loop ((count n))
        (if (> count 0)
            (begin
              (set! expansion (dabbrev--search abbrev reverse ignore-case))
              (if expansion (loop (- count 1))))))
      (set! case-fold-search saved)
      (if expansion
          (set! dabbrev--last-expansion-location (point)))
      expansion)))

(define (dabbrev--find-expansion abbrev direction ignore-case)
  "Find one occurrence of ABBREV, and return the expansion.
DIRECTION > 0 means look that many times backwards, < 0 forwards, = 0 both."
  (save-excursion
    (or (and (>= direction 0)
             (begin (set! dabbrev--last-direction (min 1 direction)) #t)
             (dabbrev--try-find abbrev #t (max 1 direction) ignore-case))
        (and (<= direction 0)
             (begin (set! dabbrev--last-direction -1) #t)
             (dabbrev--try-find abbrev #f (max 1 (- direction)) ignore-case)))))

(define (dabbrev--first-letter-position string)
  (let loop ((i 0))
    (cond ((>= i (string-length string)) #f)
          ((not (equal? (string-downcase (substring string i (+ i 1)))
                        (string-upcase (substring string i (+ i 1)))))
           i)
          (else (loop (+ i 1))))))

(define (dabbrev--substitute-expansion old abbrev expansion)
  "Replace OLD (or ABBREV, if OLD is false) before point with EXPANSION."
  (let* ((use-case-replace (and (dabbrev--ignore-case-p abbrev)
                                (if (eq? dabbrev-case-replace 'case-replace)
                                    case-replace
                                    dabbrev-case-replace)))
         (rest (substring expansion 1))
         (first-letter (dabbrev--first-letter-position abbrev)))
    (if (or (not first-letter)
            (and (not (and (or (equal? rest (string-downcase rest))
                               (equal? rest (string-upcase rest)))
                           (or (equal? abbrev (string-downcase abbrev))
                               (and (equal? abbrev (string-upcase abbrev))
                                    (> (- (string-length abbrev) first-letter) 1)))))
                 (equal? abbrev (substring expansion 0 (string-length abbrev)))))
        (set! use-case-replace #f))
    (if (and (equal? expansion (string-downcase expansion))
             (equal? abbrev (string-downcase abbrev)))
        (set! use-case-replace #f))
    (if use-case-replace
        (set! expansion (string-downcase expansion)))
    (if old
        (save-excursion (search-backward old))
        (begin (search-backward abbrev) (search-forward abbrev)))
    (replace-match expansion (not use-case-replace) #t)
    expansion))

(define-command (dabbrev-expand . args)
  "Expand previous word \"dynamically\".

Expands to the most recent, preceding word for which this is a prefix.
If no suitable preceding word is found, words following point are considered.

A positive prefix argument, N, says to take the Nth backward *distinct* possibility.
A negative argument says search forward.

If the cursor has not moved from the end of the previous expansion and no argument is given,
replace the previously-made expansion with the next possible expansion not yet tried."
  (let* ((arg (if (pair? args) (car args) (current-prefix-arg)))
         (abbrev #f)
         (old #f)
         (direction 0)
         (expansion #f))
    (save-excursion
      (if (and (not arg)
               dabbrev--last-abbrev-location
               (eq? last-command this-command))
          (begin
            (set! abbrev dabbrev--last-abbreviation)
            (set! old dabbrev--last-expansion)
            (set! direction dabbrev--last-direction))
          (begin
            (dabbrev--reset-global-variables)
            (set! direction (if arg (prefix-numeric-value arg) (if dabbrev-backward-only 1 0)))
            (set! abbrev (dabbrev--abbrev-at-point))
            (set! old #f)))
      (set! expansion (dabbrev--find-expansion abbrev direction (dabbrev--ignore-case-p abbrev))))
    (cond ((not expansion)
           (dabbrev--reset-global-variables)
           (if old
               (save-excursion
                 (search-backward old)
                 (insert abbrev)
                 (delete-region (point) (+ (point) (string-length old)))))
           (user-error (format "No%s dynamic expansion for ‘%s’ found" (if old " further" "") abbrev)))
          (else
           ;; The expansion edits the text before the place to continue from: keep it as a marker.
           (if (and (number? dabbrev--last-expansion-location) (> dabbrev--last-expansion-location (point)))
               (set! dabbrev--last-expansion-location
                     (list 'marker (save-excursion (goto-char dabbrev--last-expansion-location) (point-marker)))))
           (set! expansion (dabbrev--substitute-expansion old abbrev expansion))
           (set! dabbrev--last-expansion expansion)
           (set! dabbrev--last-abbreviation abbrev)
           (set! dabbrev--last-abbrev-location (point))))
    #f))

(define dabbrev--check-other-buffers #f)
(define dabbrev--check-all-buffers #f)

(define (dabbrev--capitalize string)
  "Return STRING with the first letter of each word upper case and the rest lower case."
  (let loop ((i 0) (in-word #f) (out ""))
    (if (>= i (string-length string))
        out
        (let* ((c (substring string i (+ i 1)))
               (word (pair? (regexp-match-positions "\\w" c))))
          (loop (+ i 1) word
                (string-append out (if word (if in-word (string-downcase c) (string-upcase c)) c)))))))

(define (dabbrev--find-all-expansions abbrev ignore-case)
  "Return a list of all possible expansions of ABBREV, the last one found first.
If IGNORE-CASE is non-nil, accept matches which differ in case."
  (save-excursion
    (goto-char (point-min))
    (let loop ((all '()))
      (let ((expansion (dabbrev--find-expansion abbrev -1 ignore-case)))
        (if expansion
            (loop (cons expansion all))
            (begin (message "Scanning for dabbrevs...done") all))))))

(define (dabbrev--capf)
  "Return (START END CANDIDATES) for the dabbrev completion of the abbreviation at point."
  (let* ((abbrev (dabbrev--abbrev-at-point))
         (start (begin (search-backward abbrev) (point)))
         (end (begin (search-forward abbrev) (point)))
         (ignore-case (dabbrev--ignore-case-p abbrev))
         (completions (save-excursion
                        (set! dabbrev--last-abbreviation abbrev)
                        (dabbrev--find-all-expansions abbrev ignore-case))))
    (if (null? completions)
        (user-error (format "No dynamic expansion for \"%s\" found%s" abbrev
                            (if dabbrev--check-other-buffers "" " in this-buffer"))))
    (list start end
          (cond ((not (and ignore-case
                           (if (eq? dabbrev-case-replace 'case-replace) case-replace dabbrev-case-replace)))
                 completions)
                ((equal? abbrev (string-upcase abbrev)) (map string-upcase completions))
                ((equal? (substring abbrev 0 1) (string-upcase (substring abbrev 0 1)))
                 (map dabbrev--capitalize completions))
                (else (map string-downcase completions))))))

(define-command (dabbrev-completion . args)
  "Completion on current word.
Like `dabbrev-expand' but finds all expansions in the current buffer
and presents suggestions for completion."
  (dabbrev--reset-global-variables)
  (let ((arg (if (null? args) (current-prefix-arg) (car args))))
    (set! dabbrev--check-other-buffers (and arg #t))
    (set! dabbrev--check-all-buffers (and arg (= (prefix-numeric-value arg) 16))))
  (set! completion-at-point-functions (list dabbrev--capf))
  (completion-at-point)
  #f)
