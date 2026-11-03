;;; sort.scm --- commands to sort text in a buffer -*- scheme -*-

;; Emacs の sort.el に当たるもの. 行を並べ替える.

(define (sort--split-lines text)
  "Return the lines of TEXT (without their newlines).  A final newline does not start a new line."
  (let ((lines (split-string text "\n")))
    (if (and (pair? lines) (equal? (car (reverse lines)) ""))
        (reverse (cdr (reverse lines)))
        lines)))

(define (sort--join-lines lines final-newline)
  "Return LINES joined with newlines, with a final newline if FINAL-NEWLINE."
  (let ((text (string-join lines "\n")))
    (if (and final-newline (pair? lines)) (string-append text "\n") text)))

(define (sort--replace-region beg end text)
  "Replace the text between BEG and END with TEXT, keeping point where it was."
  (save-excursion
    (goto-char beg)
    (delete-region beg end)
    (insert text)))

(define-command (sort-lines . args)
  "Sort lines in region alphabetically; REVERSE non-nil means descending order.
Called from a program, there are three arguments: REVERSE, BEG and END (the region to sort)."
  (let* ((descending (if (null? args) (current-prefix-arg) (car args)))
         (beg (if (and (pair? args) (pair? (cdr args))) (cadr args) (region-beginning)))
         (end (if (and (pair? args) (pair? (cdr args))) (caddr args) (region-end)))
         (from (min beg end))
         (to (max beg end))
         (text (buffer-substring from to))
         (lines (sort--split-lines text))
         (sorted (sort lines string<?)))
    (sort--replace-region from to
                          (sort--join-lines (if descending (reverse sorted) sorted)
                                            (and (> (string-length text) 0)
                                                 (equal? (substring text (- (string-length text) 1)) "\n"))))
    #f))

(define-command (reverse-region . args)
  "Reverse the order of lines in a region.
From a program takes two point or marker arguments, BEG and END."
  (let* ((beg (if (null? args) (region-beginning) (car args)))
         (end (if (null? args) (region-end) (cadr args)))
         (from (min beg end))
         (to (max beg end)))
    (save-excursion
      ;; the largest region that starts at the beginning of a line and ends at the end of one
      (goto-char from)
      (unless (bolp) (forward-line 1))
      (set! from (point))
      (goto-char to)
      (unless (and (eolp) (not (bolp)))
        (forward-line -1)
        (end-of-line))
      (set! to (point)))
    (when (< from to)
      (sort--replace-region from to
                            (string-join (reverse (split-string (buffer-substring from to) "\n")) "\n")))
    #f))

(define-command (delete-duplicate-lines . args)
  "Delete all but one copy of any identical lines in the region.
Optional REVERSE keeps the last copy; ADJACENT deletes only adjacent duplicates;
KEEP-BLANKS keeps blank lines.  Return the number of deleted lines."
  (let* ((beg (if (null? args) (region-beginning) (car args)))
         (end (if (null? args) (region-end) (cadr args)))
         (rest (if (or (null? args) (null? (cdr args))) '() (cddr args)))
         (from-end (and (pair? rest) (car rest)))
         (adjacent (and (pair? rest) (pair? (cdr rest)) (cadr rest)))
         (keep-blanks (and (pair? rest) (pair? (cdr rest)) (pair? (cddr rest)) (caddr rest)))
         (from (min beg end))
         (to (max beg end))
         (text (buffer-substring from to))
         (lines (sort--split-lines text))
         (ordered (if from-end (reverse lines) lines)))
    (let loop ((todo ordered) (seen '()) (kept '()) (deleted 0))
      (if (null? todo)
          (begin
            (when (> deleted 0)
              (sort--replace-region from to
                                    (sort--join-lines (if from-end kept (reverse kept))
                                                      (and (> (string-length text) 0)
                                                           (equal? (substring text (- (string-length text) 1)) "\n")))))
            deleted)
          (let ((line (car todo)))
            (if (and (not (and keep-blanks (equal? line "")))
                     (if adjacent
                         (and (pair? kept) (equal? (car kept) line))
                         (member line seen)))
                (loop (cdr todo) seen kept (+ deleted 1))
                (loop (cdr todo) (cons line seen) (cons line kept) deleted)))))))

(provide 'sort)

;;; sort.scm ends here
