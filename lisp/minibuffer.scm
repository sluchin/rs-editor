;;; minibuffer.scm --- completion in the buffer -*- scheme -*-

;; Emacs の minibuffer.el に当たるもの (`completion-at-point`, `complete-symbol`).
;; 補完の候補は, 先頭が合うものだけ (basic の方式. partial-completion などは無い).

(define completion-at-point-functions '()
  "List of functions to be used by `completion-at-point'.
Each function is called with no argument and returns (START END CANDIDATES) for the text at
point, or #f if it does not apply.")

(define (completion--common-prefix strings)
  "Return the longest string that is a prefix of every string in STRINGS."
  (let loop ((prefix (car strings)) (rest (cdr strings)))
    (if (null? rest)
        prefix
        (let ((s (car rest)))
          (let shrink ((n (min (string-length prefix) (string-length s))))
            (if (equal? (substring prefix 0 n) (substring s 0 n))
                (loop (substring prefix 0 n) (cdr rest))
                (shrink (- n 1))))))))

(define (completion--dedupe strings)
  "Return STRINGS without duplicates, keeping the first of each."
  (let loop ((rest strings) (seen '()))
    (cond ((null? rest) (reverse seen))
          ((member (car rest) seen) (loop (cdr rest) seen))
          (else (loop (cdr rest) (cons (car rest) seen))))))

(define (completion--show-list candidates)
  "Show CANDIDATES in the *Completions* buffer, in another window."
  (let ((buffer (buffer-name)))
    (switch-to-buffer "*Completions*")
    (delete-region (point-min) (point-max))
    (insert "Possible completions are:\n")
    (for-each (lambda (c) (insert c "\n")) (sort candidates string<?))
    (goto-char (point-min))
    (set-buffer-modified-p #f)
    (switch-to-buffer buffer)
    (display-buffer "*Completions*" 1)))

(define (completion-in-region start end candidates)
  "Complete the text between START and END using CANDIDATES (a list of strings).
Insert the longest common prefix of the candidates that start with the text.  If that adds
nothing, list the candidates in the *Completions* buffer.  Return #t if there was a match."
  (set! this-command 'completion-at-point)
  (let* ((text (buffer-substring start end))
         (matches (completion--dedupe (filter (lambda (c) (string-prefix? text c)) candidates))))
    (cond ((null? matches) (message "No match") #f)
          ((and (null? (cdr matches)) (equal? (car matches) text)) #t)
          (else
           (let ((prefix (completion--common-prefix matches)))
             (cond ((> (string-length prefix) (string-length text))
                    (goto-char end)
                    (insert prefix)
                    (delete-region start end)
                    #t)
                   (else
                    (completion--show-list matches)
                    (message "")
                    #t)))))))

(define-command (completion-at-point . args)
  "Perform completion on the text around point.
The completion method is determined by `completion-at-point-functions'."
  (let loop ((functions completion-at-point-functions))
    (if (null? functions)
        (message "No match")
        (let ((result ((car functions))))
          (if result
              (completion-in-region (car result) (cadr result) (caddr result))
              (loop (cdr functions))))))
  #f)

(define (complete-symbol--candidates prefix)
  "Return the names defined in the evaluator that start with PREFIX."
  (let ((names '()))
    (mapatoms (lambda (symbol)
                (let ((name (symbol->string symbol)))
                  (if (string-prefix? prefix name) (set! names (cons name names))))))
    (reverse names)))

(define (complete-symbol--capf)
  "Return (START END CANDIDATES) for the symbol before point."
  (let* ((end (point))
         (start (save-excursion
                  (skip-syntax-backward "w_")
                  (point))))
    (list start end (complete-symbol--candidates (buffer-substring start end)))))

(define-command (complete-symbol . args)
  "Perform completion on the symbol preceding point.
Complete with the names defined in the Scheme evaluator."
  (set! completion-at-point-functions (list complete-symbol--capf))
  (completion-at-point)
  #f)
