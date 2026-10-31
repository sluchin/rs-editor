;;; register.scm --- registers -*- scheme -*-

;; Emacs の register.el に当たるもの. レジスタの名前は 1 文字の文字列で, 中身は, 文字列か, 数か, 矩形 (文字列の並び).

(define register-alist '()
  "List of elements (NAME CONTENTS), one for each Emacs register.
NAME is a one-character string, and CONTENTS is a string, a number or a rectangle.")

(define (get-register register)
  "Return the contents of the register REGISTER, or #f if it is empty."
  (let ((entry (assoc register register-alist)))
    (and entry (cadr entry))))

(define (set-register register value)
  "Set the contents of the register REGISTER to VALUE, and return VALUE."
  (set! register-alist
        (cons (list register value)
              (let loop ((rest register-alist))
                (cond ((null? rest) '())
                      ((equal? (car (car rest)) register) (loop (cdr rest)))
                      (else (cons (car rest) (loop (cdr rest))))))))
  value)

(define (register--read prompt)
  "Read the name of a register with PROMPT."
  (let ((key (read-key prompt #t)))
    (if (equal? key "Enter") "\n" key)))

(define-command (copy-to-register . args)
  "Copy the region into the register REGISTER.
Called from a program with REGISTER, START and END (and a true DELETE-FLAG to delete the text),
copy the text between START and END.  Interactively, with a prefix argument, delete the region."
  (let* ((register (if (null? args) (register--read "Copy to register: ") (car args)))
         (start (if (< (length args) 3) (region-beginning) (cadr args)))
         (end (if (< (length args) 3) (region-end) (caddr args)))
         (delete (if (< (length args) 4) (and (null? args) (current-prefix-arg)) (list-ref args 3))))
    (set-register register (buffer-substring start end))
    (when delete (delete-region start end))
    (set-deactivate-mark #t)
    #f))

(define-command (insert-register . args)
  "Insert contents of the register REGISTER.  REGISTER is a one-character string.
Normally puts point before and mark after the inserted text.
If the optional second argument ARG is true, puts mark before and point after.
Interactively, ARG is true unless there is a prefix argument."
  (let* ((register (if (null? args) (register--read "Insert register: ") (car args)))
         (arg (if (null? args) (not (current-prefix-arg)) (and (pair? (cdr args)) (cadr args))))
         (value (get-register register)))
    (push-mark)
    (cond ((pair? value) (insert-rectangle value))
          ((string? value) (insert-for-yank value))
          ((number? value) (insert (number->string value)))
          (else (error "Register does not contain text")))
    (unless arg (exchange-point-and-mark))
    #f))
