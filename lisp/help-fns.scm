;;; help-fns.scm --- describe functions and variables -*- scheme -*-

;; Emacs の help-fns.el に当たるもの. 起動のときに読む (src-tauri/src/scheme/loadup.rs).
;; docstring は, help-language の言語の翻訳 (po/*.po) があれば, 翻訳を出す.

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

(define (help-show text)
  "Show TEXT in the *Help* buffer, in another window."
  (let ((buffer (buffer-name)))
    (switch-to-buffer "*Help*")
    (delete-region (point-min) (point-max))
    (insert text)
    (goto-char (point-min))
    (set-buffer-modified-p #f)
    (switch-to-buffer buffer)
    (display-buffer "*Help*" 1)))

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
