;;; help-fns.scm --- describe functions and variables -*- scheme -*-

;; Emacs の help-fns.el に当たるもの. 起動のときに読む (core/src/scheme/loadup.rs).
;; docstring は, help-language の言語の翻訳 (po/*.po) があれば翻訳を出す.

(define (help-language-from-locale locale)
  "Return the language of LOCALE, such as \"ja\" for \"ja_JP.UTF-8\".
Return \"en\" if LOCALE is #f, empty, \"C\" or \"POSIX\"."
  (let ((language (if (string? locale)
                      (car (split-string (car (split-string locale ".")) "_"))
                      "")))
    (if (member language '("" "C" "POSIX")) "en" language)))

(define help-language (help-language-from-locale (getenv "LANG"))
  "Language of the docstrings shown in *Help*, such as \"ja\".
If there is no translation in this language, or it is \"en\", show the original docstring.
The default is taken from the environment variable LANG.")

(define (help-file-name-nondirectory file)
  "Return FILE without its directory."
  (let loop ((parts (split-string file "/")))
    (if (null? (cdr parts)) (car parts) (loop (cdr parts)))))

(define (help-function-names)
  "Return the names of all functions and commands, sorted.
They are the functions defined in Lisp or Rust, and the commands written in TypeScript."
  (let ((names (editor-command-names)))
    (mapatoms (lambda (symbol)
                (when (fboundp symbol)
                  (set! names (cons (symbol->string symbol) names)))))
    (sort (help-unique names) string<?)))

(define (help-unique items)
  "Return ITEMS without duplicates, keeping the first ones."
  (let loop ((items items) (seen '()))
    (cond ((null? items) (reverse seen))
          ((member (car items) seen) (loop (cdr items) seen))
          (else (loop (cdr items) (cons (car items) seen))))))

(define (help-lisp-function-p function)
  "Return true if FUNCTION is defined in Lisp."
  (and (fboundp function) (list? (help-function-arglist function))))

(define (help-arglist-string function)
  "Return the argument list of the Lisp function FUNCTION as a string.
For example, \"(find-file &rest ARGS)\"."
  (format "(%s)"
          (string-join
           (cons (symbol->string function)
                 (map (lambda (arg)
                        (let ((name (symbol->string arg)))
                          (if (equal? name "&rest") name (string-upcase name))))
                      (help-function-arglist function)))
           " ")))

(define (help-function-kind function)
  "Return the first line of the help of FUNCTION, such as
\"find-file is an interactive Lisp function in `files.scm'.\""
  (let ((lisp (help-lisp-function-p function))
        (command (or (commandp function)
                     (member (symbol->string function) (editor-command-names))))
        (file (symbol-file function)))
    (format "%s is %s%s."
            function
            (cond ((and lisp command) "an interactive Lisp function")
                  (lisp "a Lisp function")
                  (command "an interactive built-in command")
                  ((fboundp function) "a built-in function")
                  (else "not defined as a function"))
            (if file (format " in `%s'" (help-file-name-nondirectory file)) ""))))

(define (help-function-text function)
  "Return the text of the help of FUNCTION (a symbol)."
  (let ((keys (where-is-internal function))
        (doc (and (fboundp function) (documentation function))))
    (string-append
     (help-function-kind function)
     "\n\n"
     (if (null? keys) "" (format "It is bound to %s.\n\n" (string-join keys ", ")))
     (if (help-lisp-function-p function)
         (string-append (help-arglist-string function) "\n\n")
         "")
     (or doc "Not documented.")
     "\n")))

(define help--history '()
  "The texts shown in *Help* before the current one, the latest first.")

(define help--forward '()
  "The texts that `help-go-back' went back from, the nearest first.")

(define help--current #f
  "The text now shown in *Help*, or #f.")

(define help--link-regexp "`([^' \t\n]+)'"
  "Regexp (Rust syntax) that matches a name quoted as `NAME' in a help text; group 1 is the name.")

(define help--link-search-regexp "`\\([^' \t\n]+\\)'"
  "Like `help--link-regexp', in the syntax of `re-search-forward'.")

(define (help--link-p name)
  "Return true if the text NAME names a function or a variable that has help."
  (let ((symbol (string->symbol name)))
    (or (fboundp symbol) (boundp symbol))))

(define (help--put text)
  "Put TEXT in the *Help* buffer, and bind the keys of the buffer."
  (let ((buffer (buffer-name)))
    (switch-to-buffer "*Help*")
    (set-buffer-read-only #f)
    (delete-region (point-min) (point-max))
    (insert text)
    (goto-char (point-min))
    (set-buffer-modified-p #f)
    (set-buffer-read-only #t)
    (for-each (lambda (binding) (local-set-key (car binding) (cadr binding)))
              '(("q" quit-window) ("l" help-go-back) ("r" help-go-forward)
                ("RET" help-follow) ("TAB" help-next-link) ("S-TAB" help-previous-link)
                ("n" help-next-link) ("p" help-previous-link)
                ("SPC" scroll-up-command) ("DEL" scroll-down-command)
                ("<" beginning-of-buffer) (">" end-of-buffer)))
    (switch-to-buffer buffer)
    (display-buffer "*Help*" 1)))

(define (help-show text)
  "Show TEXT in the *Help* buffer, in another window.
The text before it is remembered, so that `help-go-back' can show it again."
  (when (and help--current (not (equal? help--current text)))
    (set! help--history (cons help--current help--history)))
  (set! help--forward '())
  (set! help--current text)
  (help--put text))

(define-command (help-go-back . args)
  "Go back to the previous help text in the *Help* buffer."
  (when (null? help--history)
    (error "No previous help"))
  (set! help--forward (cons help--current help--forward))
  (set! help--current (car help--history))
  (set! help--history (cdr help--history))
  (help--put help--current)
  #f)

(define-command (help-go-forward . args)
  "Go forward to the help text that `help-go-back' went back from."
  (when (null? help--forward)
    (error "No next help"))
  (set! help--history (cons help--current help--history))
  (set! help--current (car help--forward))
  (set! help--forward (cdr help--forward))
  (help--put help--current)
  #f)

(define (help--name-at-point)
  "Return the name at point: the `NAME' quoted on the line, or else the symbol under point, or #f."
  (let* ((pos (point))
         (start (line-beginning-position))
         (end (line-end-position))
         (line (buffer-substring start end))
         (column (- pos start)))
    (let loop ((from 0))
      (let ((m (regexp-match-positions help--link-regexp line from)))
        (cond ((not m) (help--symbol-at line column))
              ((and (<= (car (car m)) column) (< column (cadr (car m))))
               (let ((g (cadr m))) (substring line (car g) (cadr g))))
              (else (loop (cadr (car m)))))))))

(define (help--symbol-at line column)
  "Return the run of symbol characters of LINE that covers COLUMN, or #f."
  (let* ((symbol-char? (lambda (c) (not (member c '(" " "\t" "(" ")" "\"" "'" "`" "," ";" ".")))))
         (n (string-length line)))
    (if (or (>= column n) (not (symbol-char? (substring line column (+ column 1)))))
        #f
        (let* ((b (let back ((i column))
                    (if (and (> i 0) (symbol-char? (substring line (- i 1) i))) (back (- i 1)) i)))
               (e (let fwd ((i column))
                    (if (and (< i n) (symbol-char? (substring line i (+ i 1)))) (fwd (+ i 1)) i))))
          (substring line b e)))))

(define-command (help-follow . args)
  "Show the help of the function or variable named at point in the *Help* buffer."
  (let ((name (help--name-at-point)))
    (cond ((not name) (error "No symbol here"))
          ((fboundp (string->symbol name)) (describe-function (string->symbol name)))
          ((boundp (string->symbol name)) (describe-variable (string->symbol name)))
          (else (error "No help for %s" name)))
    #f))

(define-command (help-next-link . args)
  "Move point to the next link (a quoted name that has help) in the *Help* buffer."
  (let ((start (point)))
    (let loop ()
      (if (re-search-forward help--link-search-regexp #f #t)
          (if (help--link-p (match-string 1))
              (goto-char (match-beginning 1))
              (loop))
          (begin (goto-char start) (error "No more links"))))
    #f))

(define-command (help-previous-link . args)
  "Move point to the previous link in the *Help* buffer."
  (let ((start (point)))
    (let loop ()
      (if (re-search-backward help--link-search-regexp #f #t)
          (if (help--link-p (match-string 1))
              (goto-char (match-beginning 1))
              (loop))
          (begin (goto-char start) (error "No more links"))))
    #f))

(define-command (describe-function . args)
  "Display the full documentation of FUNCTION (a symbol).
Interactively, read FUNCTION in the minibuffer with completion."
  (let ((function (if (null? args)
                      (string->symbol (completing-read "Describe function: "
                                                       (help-function-names) #f #t))
                      (car args))))
    (help-show (help-function-text function))))

(global-set-key "C-h f" 'describe-function)

(provide 'help-fns)

;;; help-fns.scm ends here
