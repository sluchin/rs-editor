;;; subr.scm --- basic Lisp subroutines -*- scheme -*-

;; Emacs の subr.el に当たるもの. 起動のときに, 始めに読む (core/src/scheme/loadup.rs).

(define noninteractive #f
  "Non-nil means this editor is running in batch mode (--batch), without a screen or a keyboard.")

(define (y-or-n-p prompt)
  "Ask the user a y or n question, and return #t if the answer is y.
PROMPT is followed by \"(y or n) \". y or SPC means yes, and n or DEL means no.
Other keys ask again."
  (let loop ((ask (string-append prompt "(y or n) ")))
    (let ((key (read-key ask)))
      (cond ((member key '("y" " ")) #t)
            ((member key '("n" "Backspace" "Delete")) #f)
            (else (loop (string-append "Please answer y or n.  " prompt "(y or n) ")))))))

(define minibuffer-default-prompt-format " (default %s)"
  "Format string used to show the default value in a prompt.")

(define (read-number prompt . args)
  "Read a number in the minibuffer with PROMPT, and return it.
DEFAULT (the first of ARGS) is returned for an empty input, and shown in PROMPT.  It can be a list:
its first element is that value, and M-n shows its non-#f elements.  HIST (the second) is the history.
An input that is not a number asks again."
  (let* ((default (if (pair? args) (car args) #f))
         (default1 (if (pair? default) (car default) default))
         (hist (if (and (pair? args) (pair? (cdr args)) (cadr args)) (cadr args) 'read-number-history))
         (shown (format minibuffer-default-prompt-format (if default1 default1 "")))
         (prompt (cond ((not default1) prompt)
                       ((string-match "\\(\\):[ \t]*\\'" prompt)
                        (string-append (substring prompt 0 (match-beginning 1)) shown
                                       (substring prompt (match-beginning 1) (string-length prompt))))
                       (else (string-append (substring prompt 0 (string-match "[ \t]*\\'" prompt)) shown))))
         (defaults (cond ((pair? default) (map number->string (filter (lambda (n) n) default)))
                         (default (list (number->string default)))
                         (else '()))))
    (let loop ()
      (let* ((input (read-string prompt #f hist defaults))
             (n (if (equal? input "") default1 (string->number input))))
        (if (number? n)
            n
            (begin
              (message "Please enter a number.")
              (loop)))))))

(define (yes-or-no-p prompt)
  "Ask the user a yes or no question, and return #t if the answer is yes.
PROMPT is followed by \"(yes or no) \". The answer must be typed in full, and ended with RET.
Other answers ask again."
  (let loop ((ask (string-append prompt "(yes or no) ")))
    (let ((answer (string-trim (read-string ask))))
      (cond ((equal? answer "yes") #t)
            ((equal? answer "no") #f)
            (noninteractive
             ;; バッチモードは, ベルと知らせで促し, 同じプロンプトで聞き直す (Emacs と同じ)
             (ding)
             (message "Please answer yes or no.")
             (loop ask))
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

(define (concat . strings)
  "Concatenate all the argument strings and make the result a new string."
  (apply string-append strings))

(define (mapconcat function sequence . separator)
  "Apply FUNCTION to each element of SEQUENCE, and concatenate the results as strings.
In between each pair of results, stick in SEPARATOR (default: the empty string)."
  (let ((sep (if (null? separator) "" (car separator))))
    (string-join (map function sequence) sep)))

(define (string< a b)
  "Return #t if the first string is less than the second (compared by character code)."
  (string<? a b))

(define (funcall function . arguments)
  "Call FUNCTION with the remaining arguments."
  (apply function arguments))
