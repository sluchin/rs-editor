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

(define-command (copy-rectangle-to-register . args)
  "Copy rectangular region of text between START and END into REGISTER.
If DELETE-FLAG is true (interactively, with a prefix argument), delete the region after copying.
Called from Lisp, takes four arguments: REGISTER, START, END and DELETE-FLAG."
  (let* ((keyboard (null? args))
         (register (if keyboard (register--read "Copy rectangle to register: ") (car args)))
         (start (if keyboard (region-beginning) (cadr args)))
         (end (if keyboard (region-end) (caddr args)))
         (delete (if keyboard (current-prefix-arg) (and (> (length args) 3) (list-ref args 3))))
         (rectangle (if delete (delete-extract-rectangle start end) (extract-rectangle start end))))
    (set-register register rectangle)
    (when (and (not delete) keyboard) (set-deactivate-mark #t))
    #f))

(define-command (number-to-register . args)
  "Store NUMBER in REGISTER.  If NUMBER is false, read a decimal number from the buffer
at point, and move point to the end of that number.
Interactively, NUMBER is the prefix argument (none means false)."
  (let* ((keyboard (null? args))
         (number (if keyboard (current-prefix-arg) (car args)))
         (register (if keyboard (register--read "Number to register: ") (cadr args))))
    (set-register register
                  (cond (number (prefix-numeric-value number))
                        ((looking-at "\\s-*-?[0-9]+")
                         (goto-char (match-end 0))
                         (string->number (string-trim (match-string 0))))
                        (else 0)))
    #f))

(define-command (append-to-register . args)
  "Append the region between START and END to REGISTER.
If DELETE-FLAG is true (interactively, with a prefix argument), delete the region after appending."
  (let* ((keyboard (null? args))
         (register (if keyboard (register--read "Append to register: ") (car args)))
         (start (if keyboard (region-beginning) (cadr args)))
         (end (if keyboard (region-end) (caddr args)))
         (delete (if keyboard (current-prefix-arg) (and (> (length args) 3) (list-ref args 3))))
         (value (get-register register))
         (text (buffer-substring start end)))
    (set-register register
                  (cond ((not value) text)
                        ((string? value) (string-append value text))
                        (else (error "Register does not contain text"))))
    (set-deactivate-mark #t)
    (when delete (delete-region start end))
    #f))

(define-command (increment-register . args)
  "Augment contents of REGISTER using PREFIX.
If REGISTER contains a number, add the numeric value of PREFIX to it.
If REGISTER is empty or contains text, append the region to it, deleting the region
if PREFIX is true.  Interactively, PREFIX is the raw prefix argument."
  (let* ((keyboard (null? args))
         (prefix (if keyboard (current-prefix-arg) (car args)))
         (register (if keyboard (register--read "Increment register: ") (cadr args)))
         (value (get-register register)))
    (cond ((number? value) (set-register register (+ (prefix-numeric-value prefix) value)))
          ((or (not value) (string? value))
           (append-to-register register (region-beginning) (region-end) prefix))
          (else (error "Register does not contain a number or text")))
    #f))

(define-command (point-to-register . args)
  "Store the current location of point in REGISTER.
Use jump-to-register to go to that location."
  (let ((register (if (null? args) (register--read "Point to register: ") (car args))))
    (set-register register (list 'marker (buffer-name) (point-marker)))
    #f))

(define-command (window-configuration-to-register . args)
  "Store the window configuration of the selected frame in REGISTER.
Use jump-to-register to restore the configuration."
  (let ((register (if (null? args) (register--read "Window configuration to register: ") (car args))))
    (set-register register (list 'window-configuration (current-window-configuration) (buffer-name) (point-marker)))
    #f))

(define-command (kmacro-to-register . args)
  "Store the last keyboard macro in the register REGISTER.
Use `jump-to-register' to run the macro."
  (let ((register (if (null? args) (register--read "Kmacro to register: ") (car args))))
    (unless last-kbd-macro
      (error "No keyboard macro defined"))
    (set-register register (list 'kbd-macro last-kbd-macro))
    #f))

(define-command (jump-to-register . args)
  "Go to the location stored in REGISTER, restore the configuration stored there,
or run the keyboard macro stored there.
Push the mark if going to the location moves point, unless called in succession."
  (let* ((register (if (null? args) (register--read "Jump to register: ") (car args)))
         (value (get-register register)))
    (cond ((and (pair? value) (eq? (car value) 'marker))
           (unless (get-buffer (cadr value)) (error "That register's buffer no longer exists"))
           (switch-to-buffer (cadr value))
           (let ((position (marker-position (caddr value))))
             (unless (or (= (point) position) (eq? last-command 'jump-to-register))
               (push-mark))
             (goto-char position)))
          ((and (pair? value) (eq? (car value) 'window-configuration))
           (set-window-configuration (cadr value))
           (when (get-buffer (caddr value))
             (with-current-buffer (caddr value)
               (goto-char (marker-position (list-ref value 3))))))
          ((and (pair? value) (eq? (car value) 'kbd-macro))
           (execute-kbd-macro (cadr value) 1))
          (else (error "Register doesn’t contain a buffer position or configuration")))
    #f))

(define-command (prepend-to-register . args)
  "Prepend the region between START and END to REGISTER.
If DELETE-FLAG is true (interactively, with a prefix argument), delete the region after prepending."
  (let* ((keyboard (null? args))
         (register (if keyboard (register--read "Prepend to register: ") (car args)))
         (start (if keyboard (region-beginning) (cadr args)))
         (end (if keyboard (region-end) (caddr args)))
         (delete (if keyboard (current-prefix-arg) (and (> (length args) 3) (list-ref args 3))))
         (value (get-register register))
         (text (buffer-substring start end)))
    (set-register register
                  (cond ((not value) text)
                        ((string? value) (string-append text value))
                        (else (error "Register does not contain text"))))
    (set-deactivate-mark #t)
    (when delete (delete-region start end))
    #f))
