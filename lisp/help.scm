;;; help.scm --- help commands -*- scheme -*-

;; Emacs の help.el と help-fns.el の残りに当たるもの (`describe-key`, `describe-variable`, `where-is` など).
;; `describe-function` は help-fns.scm. キーの割り当て (`key-binding`, `all-key-bindings`) と最近のキー (`recent-keys`) は,
;; ターミナルのアプリが受け持つ.

(define (help--variable-names)
  "Return the names of all variables, sorted."
  (let ((names '()))
    (mapatoms (lambda (symbol)
                (if (and (boundp symbol) (not (fboundp symbol)))
                    (set! names (cons (symbol->string symbol) names)))))
    (sort names string<?)))

(define (help--command-names)
  "Return the names of all commands, sorted."
  (filter (lambda (name) (commandp (string->symbol name))) (help-function-names)))

(define (help--read-symbol prompt names)
  "Read a symbol with PROMPT, completing in NAMES."
  (string->symbol (completing-read prompt names #f #t)))

(define (help-variable-text variable)
  "Return the text of the help of VARIABLE (a symbol)."
  (let ((file (symbol-file variable))
        (doc (documentation-property variable 'variable-documentation)))
    (string-append
     (format "%s is a variable%s.\n\n" variable
             (if file (format " defined in `%s'" (help-file-name-nondirectory file)) ""))
     (format "Its value is %S.\n\n" (symbol-value variable))
     (or doc "Not documented.")
     "\n")))

(define-command (describe-variable . args)
  "Display the full documentation of VARIABLE (a symbol).
Interactively, read VARIABLE in the minibuffer with completion."
  (let ((variable (if (null? args)
                      (help--read-symbol "Describe variable: " (help--variable-names))
                      (car args))))
    (if (not (boundp variable))
        (error (format "%s is void as a variable" variable)))
    (help-show (help-variable-text variable))
    #f))

(define-command (describe-command . args)
  "Display the full documentation of COMMAND (a symbol).
Interactively, read COMMAND in the minibuffer with completion."
  (let ((command (if (null? args)
                     (help--read-symbol "Describe command: " (help--command-names))
                     (car args))))
    (help-show (help-function-text command))
    #f))

(define-command (describe-symbol . args)
  "Display the full documentation of SYMBOL.
Show the function and the variable of that name, if there are any.
Interactively, read SYMBOL in the minibuffer with completion."
  (let* ((symbol (if (null? args)
                     (help--read-symbol "Describe symbol: "
                                        (sort (help-unique (append (help-function-names)
                                                                    (help--variable-names)))
                                              string<?))
                     (car args)))
         (function (fboundp symbol))
         (variable (and (boundp symbol) (not function))))
    (cond ((and (not function) (not variable))
           (error (format "I don't know anything about %s" symbol)))
          (else
           (help-show (string-append (if function (help-function-text symbol) "")
                                     (if (and function variable) "\n" "")
                                     (if variable (help-variable-text symbol) "")))))
    #f))

(define (help--key-name key)
  "Return the name of KEY, as `read-key' returned it, in the notation of key bindings."
  (cond ((equal? key " ") "SPC")
        ((equal? key "Enter") "RET")
        ((equal? key "Backspace") "DEL")
        ((equal? key "Delete") "<delete>")
        ((equal? key "Escape") "ESC")
        (else key)))

(define (help--read-key-sequence prompt)
  "Read keys with PROMPT until they make a complete key sequence; return it as a string such as \"C-x C-f\"."
  (let loop ((keys '()))
    (let* ((keys (append keys (list (help--key-name (read-key
                                                      (if (null? keys)
                                                          prompt
                                                          (format "%s %s" prompt (string-join keys ""))))))))
           (joined (string-join keys " "))
           (binding (key-binding joined)))
      (if (eq? binding #t) (loop keys) joined))))

(define (help--key-command keys)
  "Return the name of the command KEYS (a string such as \"C-x C-f\") runs, or #f."
  (let ((binding (key-binding keys)))
    (and (string? binding) binding)))

(define-command (describe-key . args)
  "Display documentation of the function invoked by KEY.
Interactively, read the key sequence after the prompt."
  (let* ((keys (if (null? args) (help--read-key-sequence "Describe key: ") (car args)))
         (command (help--key-command keys)))
    (if command
        (help-show (string-append (format "%s runs the command %s (found in global-map)\n\n" keys command)
                                  (help-function-text (string->symbol command))))
        (help-show (format "%s is undefined\n" keys)))
    #f))

(define-command (describe-key-briefly . args)
  "Print the name of the function KEY invokes.  KEY is a string such as \"C-x C-f\".
Interactively, read the key sequence after the prompt."
  (let* ((keys (if (null? args) (help--read-key-sequence "Describe key briefly: ") (car args)))
         (command (help--key-command keys)))
    (if command
        (message "%s runs the command %s (found in global-map)" keys command)
        (message "%s is undefined" keys))
    #f))

(define-command (where-is . args)
  "Print message listing the key sequences that invoke the command DEFINITION.
Interactively, read the command name in the minibuffer with completion."
  (let* ((command (if (null? args)
                      (help--read-symbol "Where is command: " (help--command-names))
                      (car args)))
         (keys (where-is-internal command)))
    (if (null? keys)
        (message "%s is not on any key" command)
        (message "%s is on %s" command (string-join keys ", ")))
    #f))

(define-command (describe-bindings . args)
  "Display a buffer showing a list of all defined keys, and their definitions."
  (help-show
   (string-append
    "Global Bindings:\n\n"
    (format "%-24s%s\n" "Key" "Binding")
    (format "%-24s%s\n" "---" "-------")
    (apply string-append
           (map (lambda (binding) (format "%-24s%s\n" (car binding) (cadr binding)))
                (sort (all-key-bindings)
                      (lambda (a b) (string<? (car a) (car b))))))))
  #f)

(define-command (view-lossage . args)
  "Display the last 300 input keystrokes."
  (help-show (string-append "Recent input:\n\n"
                            (help--wrap (string-join (recent-keys) " ") 70)
                            "\n"))
  #f)

(define (help--wrap text width)
  "Return TEXT with its lines broken between words so that none is longer than WIDTH."
  (let loop ((words (split-string text " ")) (line "") (out ""))
    (cond ((null? words) (string-append out line))
          ((and (> (string-length line) 0)
                (> (+ (string-length line) 1 (string-length (car words))) width))
           (loop words "" (string-append out line "\n")))
          (else (loop (cdr words)
                      (if (equal? line "") (car words) (string-append line " " (car words)))
                      out)))))

(define-command (view-echo-area-messages . args)
  "View the log of recent echo-area messages: the `*Messages*' buffer.
The buffer is shown in another window, with point at its end."
  (let ((log (message-log)))
    (with-current-buffer (get-buffer-create "*Messages*")
      (set-buffer-read-only #f)
      (erase-buffer)
      (insert (apply string-append (map (lambda (line) (string-append line "\n")) log)))
      (goto-char (point-max))
      (set-buffer-modified-p #f)
      (set-buffer-read-only #t))
    (display-buffer "*Messages*")
    #f))

(define help--for-help-text
  "You have typed C-h, the help character.  Type a Help option:
(Type q to exit the Help command.)

b           Display all key bindings.
c KEYS      Display the command name run by the given key sequence.
e           Show the *Messages* buffer.
f           Display documentation of the given function.
i           Read the manuals with Info.
k KEYS      Display the full documentation for the key sequence.
l           Show last 300 input keystrokes (lossage).
o           Display the documentation of the given symbol.
r           Display the Emacs manual in Info mode.
s           Display contents of current syntax table, plus explanations.
v           Display the given variable's documentation and value.
w COMMAND   Display which keystrokes invoke the given command.
x COMMAND   Display the given command's documentation.
q           Quit this help.
C-q         Toggle the short help of the common help commands.
")

(define-command (help-for-help . args)
  "You have typed \\[help-for-help], the help character.  Type a Help option.
The options are listed in the *Help* buffer."
  (help-show help--for-help-text)
  #f)

(define-command (help-quit . args)
  "Just exit from the help command's prompt."
  #f)

(define-command (help-quick-toggle . args)
  "Toggle the display of the short help of the common help commands."
  (if (get-buffer "*Quick Help*")
      (begin (delete-windows-on "*Quick Help*") (kill-buffer "*Quick Help*"))
      (begin
        (with-current-buffer (get-buffer-create "*Quick Help*")
          (insert help--for-help-text)
          (goto-char (point-min))
          (set-buffer-modified-p #f))
        (display-buffer "*Quick Help*")))
  #f)

(define-command (search-forward-help-for-help . args)
  "Search forward in the help for help text."
  (help-for-help)
  (let loop ((windows (window-list)))
    (cond ((null? windows) #f)
          ((equal? (window-buffer (car windows)) "*Help*") (select-window (car windows)))
          (else (loop (cdr windows)))))
  (isearch-forward)
  #f)

(define (help--syntax-description class)
  "Return the description of the syntax class character CLASS."
  (cond ((equal? class "w") "word")
        ((equal? class "_") "symbol")
        ((equal? class " ") "whitespace")
        ((equal? class ".") "punctuation")
        ((equal? class "(") "open")
        ((equal? class ")") "close")
        ((equal? class "\"") "string")
        ((equal? class "'") "expression prefix")
        ((equal? class "\\") "escape")
        ((equal? class "<") "comment start")
        ((equal? class ">") "comment end")
        (else class)))

(define (help--char-string code)
  "Return CODE as a string, control characters as ^X."
  (cond ((< code 32) (format "^%s" (char-to-string (+ code 64))))
        ((= code 127) "^?")
        (else (char-to-string code))))

(define (help--syntax-run start end class)
  "Return the line that says the characters START to END have the syntax CLASS."
  (format "%-12s%s\n"
          (if (= start end)
              (help--char-string start)
              (format "%s .. %s" (help--char-string start) (help--char-string end)))
          (help--syntax-description class)))

(define (help--syntax-text)
  "Return the text of the syntax table of the current buffer, for the ASCII characters."
  (let loop ((code 0) (start 0) (class #f) (out ""))
    (let ((now (if (< code 128) (char-syntax (char-to-string code)) #f)))
      (cond ((and (< code 128) (equal? now class))
             (loop (+ code 1) start class out))
            (else
             (let ((out (if class
                            (string-append out (help--syntax-run start (- code 1) class))
                            out)))
               (if (< code 128)
                   (loop (+ code 1) code now out)
                   out)))))))

(define-command (describe-syntax . args)
  "Describe the syntax specifications in the syntax table of the current buffer.
The ASCII characters are listed in runs that have the same syntax."
  (help-show (string-append "The syntax table of the current buffer:\n\n" (help--syntax-text)))
  #f)

(provide 'help)

;;; help.scm ends here
