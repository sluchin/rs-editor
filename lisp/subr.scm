;;; subr.scm --- basic Lisp subroutines -*- scheme -*-

;; Emacs の subr.el に当たるもの. 起動のときに, 始めに読む (core/src/scheme/loadup.rs).

(define (y-or-n-p prompt)
  "Ask the user a y or n question, and return #t if the answer is y.
PROMPT is followed by \"(y or n) \". y or SPC means yes, and n or DEL means no.
Other keys ask again."
  (let loop ((ask (string-append prompt "(y or n) ")))
    (let ((key (read-key ask)))
      (cond ((member key '("y" " ")) #t)
            ((member key '("n" "Backspace" "Delete")) #f)
            (else (loop (string-append "Please answer y or n.  " prompt "(y or n) ")))))))

(define (yes-or-no-p prompt)
  "Ask the user a yes or no question, and return #t if the answer is yes.
PROMPT is followed by \"(yes or no) \". The answer must be typed in full, and ended with RET.
Other answers ask again."
  (let loop ((ask (string-append prompt "(yes or no) ")))
    (let ((answer (string-trim (read-string ask))))
      (cond ((equal? answer "yes") #t)
            ((equal? answer "no") #f)
            (else (loop (string-append "Please answer yes or no.  " prompt "(yes or no) ")))))))

(define (file-name-directory filename)
  "Return the directory part of FILENAME, up to and including the last /.
Return #f if FILENAME has no /."
  (let loop ((i (- (string-length filename) 1)))
    (cond ((< i 0) #f)
          ((equal? (substring filename i (+ i 1)) "/") (substring filename 0 (+ i 1)))
          (else (loop (- i 1))))))

(define (file-name-nondirectory filename)
  "Return the file name of FILENAME without its directory part."
  (let ((dir (file-name-directory filename)))
    (if dir (substring filename (string-length dir)) filename)))

(provide 'subr)

;;; subr.scm ends here
