;;; paredit.scm --- edit S-expressions and keep the parentheses balanced -*- scheme -*-

;; paredit (Taylor Campbell) の, 基本の部分. 起動のときに読む (core/src/scheme/loadup.rs).
;; `M-x paredit-mode` で, バッファのキーを割り当てる. 切ると, キーは, 元のコマンドと同じ動きをする
;; (キーの割り当ては, バッファから消さずに, コマンドがモードを見る).
;; 移していないもの: 引用符の中のエスケープの細かい扱い, `paredit-forward-up` などの移動, `C-u` の数引数.

(define paredit--openers '("(" "[" "{"))
(define paredit--closers '(")" "]" "}"))

(define (paredit--on)
  "Return true if Paredit mode is on in the current buffer."
  (buffer-minor-mode-p 'paredit))

(define (paredit--next)
  "Return the character after point as a string, or \"\" at the end of the buffer."
  (let ((c (char-after))) (if (string? c) c "")))

(define (paredit--prev)
  "Return the character before point as a string, or \"\" at the start of the buffer."
  (let ((c (char-before))) (if (string? c) c "")))

(define (paredit--in-string) (list-ref (syntax-ppss) 3))
(define (paredit--in-comment) (list-ref (syntax-ppss) 4))
(define (paredit--literal) (or (paredit--in-string) (paredit--in-comment)))

(define (paredit--pair? open close)
  "Return true if the strings OPEN and CLOSE are a pair of delimiters."
  (member (list open close) '(("(" ")") ("[" "]") ("{" "}") ("\"" "\""))))

(define (paredit--needs-space-before)
  "Return true if a space is needed between the text before point and an opening delimiter."
  (let ((c (paredit--prev)))
    (not (or (equal? c "")
             (member c '(" " "\t" "\n" "(" "[" "{" "'" "`" "," "@" "#"))))))

(define-command (paredit-open-round . args)
  "Insert a balanced pair of parentheses and put point between them.
Insert just a `(' in a string or a comment."
  (if (or (not (paredit--on)) (paredit--literal))
      (insert "(")
      (begin
        (if (paredit--needs-space-before) (insert " "))
        (insert "()")
        (backward-char 1)))
  #f)

(define-command (paredit-close-round . args)
  "Move past the closing parenthesis of the list at point, deleting the whitespace before it.
Insert just a `)' in a string or a comment."
  (if (or (not (paredit--on)) (paredit--literal))
      (insert ")")
      (condition-case e
          (begin
            (up-list)
            ;; whitespace before the closing delimiter goes away
            (let* ((close (- (point) 1))
                   (start (save-excursion
                            (goto-char close)
                            (skip-chars-backward " \t\n")
                            (point))))
              (if (< start close) (delete-region start close))))
        (error (message "No list to close"))))
  #f)

(define-command (paredit-doublequote . args)
  "Insert a pair of double quotes, or in a string, a quoted double quote, or move past the closing one."
  (cond ((or (not (paredit--on)) (paredit--in-comment)) (insert "\""))
        ((paredit--in-string)
         (if (equal? (paredit--next) "\"") (forward-char 1) (insert "\\\"")))
        (else (insert "\"\"") (backward-char 1)))
  #f)

(define-command (paredit-forward-delete . args)
  "Delete the character after point; delete an empty pair of delimiters as a whole,
and do not delete a delimiter of a pair that holds something."
  (let ((c (paredit--next)))
    (cond ((or (not (paredit--on)) (equal? c "")) (delete-char 1))
          ((and (paredit--in-string) (equal? c "\""))
           (if (equal? (paredit--prev) "\"") (begin (backward-char 1) (delete-char 2))))
          ((paredit--literal) (delete-char 1))
          ((member c paredit--openers)
           (if (paredit--pair? c (substring (buffer-substring (point) (min (point-max) (+ (point) 2))) 1 2))
               (delete-char 2)
               (forward-char 1)))
          ((equal? c "\"")
           (if (equal? (substring (buffer-substring (point) (min (point-max) (+ (point) 2))) 1) "\"")
               (delete-char 2)
               (forward-char 1)))
          ((member c paredit--closers)
           (if (paredit--pair? (paredit--prev) c) (begin (backward-char 1) (delete-char 2))))
          (else (delete-char 1))))
  #f)

(define-command (paredit-backward-delete . args)
  "Delete the character before point; delete an empty pair of delimiters as a whole,
and move back over a delimiter of a pair that holds something."
  (let ((c (paredit--prev)))
    (cond ((or (not (paredit--on)) (equal? c "")) (delete-char -1))
          ((and (equal? c "\"") (paredit--in-string))
           (if (equal? (paredit--next) "\"") (begin (backward-char 1) (delete-char 2))))
          ((and (equal? c "\"") (not (paredit--in-string))) (backward-char 1))
          ((paredit--literal) (delete-char -1))
          ((member c paredit--openers)
           (if (paredit--pair? c (paredit--next)) (begin (backward-char 1) (delete-char 2))))
          ((member c paredit--closers) (backward-char 1))
          (else (delete-char -1))))
  #f)

(define (paredit--kill-end)
  "Return the position that `paredit-kill' kills up to: the end of the last whole S-expression
that starts on the line (a sexp that spans lines is killed whole), or the end of the line."
  (save-excursion
    (let loop ()
      (skip-chars-forward " \t")
      (let ((c (paredit--next)))
        (cond ((or (equal? c "") (equal? c "\n")) (point))
              ((member c paredit--closers) (point))
              ((equal? c ";") (line-end-position))
              (else
               (let ((end (condition-case e (scan-sexps (point) 1) (error #f))))
                 (if (not end)
                     (line-end-position)
                     (begin (goto-char end) (loop))))))))))

(define-command (paredit-kill . args)
  "Kill a line as if with `kill-line', but respect delimiters:
kill the S-expressions up to the end of the line, and in a string, up to its end."
  (cond ((or (not (paredit--on)) (paredit--in-comment) (= (point) (line-end-position)))
         (kill-line))
        ((paredit--in-string)
         (let ((string-end (- (up-list-position (point) 1) 1)))
           (simple--kill (point) (min (line-end-position) string-end))))
        (else (simple--kill (point) (paredit--kill-end))))
  #f)

(define (paredit--enclosing)
  "Return the start of the list around point (the position of its opening delimiter), or signal an error."
  (let ((open (list-ref (syntax-ppss) 1)))
    (if open open (user-error "Not inside a list"))))

(define (paredit--close-of open)
  "Return the position of the closing delimiter of the list that starts at OPEN."
  (- (scan-lists open 1 0) 1))

(define-command (paredit-forward-slurp-sexp . args)
  "Add the S-expression after the list around point to the list: move its closing delimiter forward."
  (let* ((open (paredit--enclosing))
         (close (paredit--close-of open))
         (closer (buffer-substring close (+ close 1)))
         (target (condition-case e (scan-sexps (+ close 1) 1) (error #f)))
         (here (point)))
    (if (not target) (user-error "Nothing to slurp"))
    (goto-char target)
    (insert closer)
    (goto-char close)
    (delete-char 1)
    (goto-char here))
  #f)

(define-command (paredit-forward-barf-sexp . args)
  "Remove the last S-expression of the list around point from the list: move its closing delimiter back."
  (let* ((open (paredit--enclosing))
         (close (paredit--close-of open))
         (closer (buffer-substring close (+ close 1)))
         (last (condition-case e (scan-sexps close -1) (error #f)))
         (here (point)))
    (if (or (not last) (<= last open)) (user-error "Nothing to barf"))
    (let ((insert-at (save-excursion
                       (goto-char last)
                       (skip-chars-backward " \t\n")
                       (point))))
      (goto-char close)
      (delete-char 1)
      (goto-char insert-at)
      (insert closer)
      (goto-char (min here insert-at))))
  #f)

(define-command (paredit-backward-slurp-sexp . args)
  "Add the S-expression before the list around point to the list: move its opening delimiter back."
  (let* ((open (paredit--enclosing))
         (opener (buffer-substring open (+ open 1)))
         (target (condition-case e (scan-sexps open -1) (error #f)))
         (here (point)))
    (if (not target) (user-error "Nothing to slurp"))
    (goto-char open)
    (delete-char 1)
    (goto-char target)
    (insert opener)
    (goto-char here))
  #f)

(define-command (paredit-backward-barf-sexp . args)
  "Remove the first S-expression of the list around point from the list: move its opening delimiter forward."
  (let* ((open (paredit--enclosing))
         (opener (buffer-substring open (+ open 1)))
         (close (paredit--close-of open))
         (first-end (condition-case e (scan-sexps (+ open 1) 1) (error #f)))
         (here (point)))
    (if (or (not first-end) (> first-end close)) (user-error "Nothing to barf"))
    (let ((insert-at (save-excursion
                       (goto-char first-end)
                       (skip-chars-forward " \t\n")
                       (point))))
      (goto-char insert-at)
      (insert opener)
      (goto-char open)
      (delete-char 1)
      (goto-char (max here (- insert-at 1)))))
  #f)

(define-command (paredit-splice-sexp . args)
  "Remove the delimiters of the list around point, keeping what is in it."
  (let* ((open (paredit--enclosing))
         (close (paredit--close-of open))
         (here (point)))
    (goto-char close)
    (delete-char 1)
    (goto-char open)
    (delete-char 1)
    (goto-char (if (> here open) (- here 1) here)))
  #f)

(define-command (paredit-raise-sexp . args)
  "Replace the list around point with the S-expression at point."
  (let* ((open (paredit--enclosing))
         (close (paredit--close-of open))
         (end (scan-sexps (point) 1))
         (start (scan-sexps end -1))
         (text (buffer-substring start end)))
    (delete-region open (+ close 1))
    (goto-char open)
    (insert text)
    (goto-char open))
  #f)

(define-command (paredit-wrap-round . args)
  "Wrap the S-expression after point in a pair of parentheses; leave point after the open one."
  (let ((end (condition-case e (scan-sexps (point) 1) (error #f))))
    (if (not end) (user-error "Nothing to wrap"))
    (goto-char end)
    (insert ")")
    (goto-char (scan-sexps end -1))
    (insert "("))
  #f)

(define paredit--keys
  '(("(" paredit-open-round) (")" paredit-close-round) ("\"" paredit-doublequote)
    ("DEL" paredit-backward-delete) ("C-d" paredit-forward-delete) ("C-k" paredit-kill)
    ("C-)" paredit-forward-slurp-sexp) ("C-}" paredit-forward-barf-sexp)
    ("C-(" paredit-backward-slurp-sexp) ("C-{" paredit-backward-barf-sexp)
    ("M-s" paredit-splice-sexp) ("M-r" paredit-raise-sexp) ("M-(" paredit-wrap-round))
  "List of (KEY COMMAND) that `paredit-mode' binds in the buffer.")

(define-command (paredit-mode . args)
  "Toggle Paredit mode: keys that edit S-expressions and keep the delimiters balanced.
With a positive argument, turn it on; with zero or a negative one, turn it off.
The keys are bound in the buffer when the mode is first turned on, and do their usual work while it is off."
  (let ((on (modes--command args 'paredit #f "Paredit mode")))
    (if on
        (for-each (lambda (binding) (local-set-key (car binding) (cadr binding))) paredit--keys))
    on))
