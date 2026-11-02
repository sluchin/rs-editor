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

(define (shell-quote-argument argument)
  "Quote ARGUMENT for passing as argument to an inferior shell."
  (if (equal? argument "")
      "''"
      (string-append "'"
                     (string-join (split-string argument "'") "'\\''")
                     "'")))

(define (display-warning type message . rest)
  "Display a warning message in the `*Warnings*' buffer, creating it if needed.
TYPE is a symbol or a list of symbols that says what the warning is about.
MESSAGE is the text.  The optional LEVEL (a symbol: debug, warning or error) is warning by default;
a debug warning is only added to the buffer, the other levels also show it in another window."
  (let ((level (if (pair? rest) (car rest) 'warning))
        (kind (if (pair? type) (car type) type)))
    (with-current-buffer (get-buffer-create "*Warnings*")
      (goto-char (point-max))
      (insert (format "%s (%s): %s\n"
                      (if (eq? level 'error) "Error" (if (eq? level 'debug) "Debug" "Warning"))
                      kind message))
      (set-buffer-modified-p #f))
    (unless (eq? level 'debug)
      (display-buffer "*Warnings*"))
    #f))
