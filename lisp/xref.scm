;;; xref.scm --- find definitions and references -*- scheme -*-

;; Emacs の xref.el に当たるもの (`xref-find-definitions`, `xref-go-back` など).
;; 定義は, Lisp の `define` と `define-command` を, 名前を定義したファイル (`symbol-file`) から探す.
;; 参照は, `xref-project-directory` (始めは今のディレクトリ) の下のファイルを, 名前の単語で探す (`rust-grep`).

(define xref--back-stack '()
  "List of (BUFFER-NAME MARKER) to go back to, the latest first.")

(define xref--forward-stack '()
  "List of (BUFFER-NAME MARKER) to go forward to, the latest first.")

(define xref-prompt-for-identifier #f
  "If non-nil, `xref-find-definitions' always reads the identifier in the minibuffer.")

(define xref-project-directory "."
  "The directory `xref-find-references' searches.")

(define (xref--location)
  "Return (BUFFER-NAME MARKER) for point."
  (list (buffer-name) (point-marker)))

(define (xref--push-marker-stack)
  "Remember the current location, for `xref-go-back'."
  (set! xref--back-stack (cons (xref--location) xref--back-stack))
  (set! xref--forward-stack '()))

(define (xref--jump location)
  "Show the buffer of LOCATION in the selected window and put point at its marker."
  (let ((name (car location)))
    (if (not (get-buffer name))
        (error "The buffer of the location was killed"))
    (pop-to-buffer-same-window name)
    (goto-char (marker-position (cadr location)))))

(define-command (xref-go-back . args)
  "Go back to the previous position in xref history."
  (if (null? xref--back-stack)
      (user-error "At start of xref history"))
  (let ((location (car xref--back-stack)))
    (set! xref--back-stack (cdr xref--back-stack))
    (set! xref--forward-stack (cons (xref--location) xref--forward-stack))
    (xref--jump location)
    #f))

(define-command (xref-go-forward . args)
  "Go to the point where we previously were, before `xref-go-back'."
  (if (null? xref--forward-stack)
      (user-error "At end of xref history"))
  (let ((location (car xref--forward-stack)))
    (set! xref--forward-stack (cdr xref--forward-stack))
    (set! xref--back-stack (cons (xref--location) xref--back-stack))
    (xref--jump location)
    #f))

(define (xref--identifier-at-point)
  "Return the symbol at or next to point, or #f."
  (save-excursion
    (skip-syntax-backward "w_")
    (let ((start (point)))
      (skip-syntax-forward "w_")
      (if (= start (point)) #f (buffer-substring start (point))))))

(define (xref--read-identifier prompt)
  "Read an identifier with PROMPT; the default is the symbol at point."
  (let* ((default (xref--identifier-at-point))
         (answer (read-string (if default (format "%s (default %s): " prompt default) (format "%s: " prompt)))))
    (if (equal? answer "")
        (or default (user-error "There is no default identifier"))
        answer)))

(define (xref--identifier args prompt)
  "Return the identifier ARGS name, the symbol at point, or one read with PROMPT."
  (cond ((and (pair? args) (string? (car args))) (car args))
        ((or xref-prompt-for-identifier (current-prefix-arg) (not (xref--identifier-at-point)))
         (xref--read-identifier prompt))
        (else (xref--identifier-at-point))))

(define (xref--definition-regexp name)
  "Return a regexp (for `grep') that matches the line that defines NAME."
  (string-append "^[ \t]*\\(define(-command)?[ \t]+\\(?" (regexp-quote name) "([ \t)]|$)"))

(define (xref--definition name)
  "Return (FILE LINE) of the definition of NAME, or #f."
  (let ((file (symbol-file (string->symbol name))))
    (and file
         (file-exists? file)
         (let ((found (rust-grep (xref--definition-regexp name) (list file))))
           (and (pair? found)
                (list file (cadr (car found))))))))

(define (xref--goto-definition name other-window)
  "Go to the definition of NAME, in another window if OTHER-WINDOW."
  (let ((definition (xref--definition name)))
    (if (not definition)
        (user-error (format "No definition found for: %s" name)))
    (xref--push-marker-stack)
    (let ((buffer (find-file-noselect (car definition))))
      (if other-window
          (window--pop-to-buffer-other-window buffer)
          (pop-to-buffer-same-window buffer))
      (goto-char (point-min))
      (forward-line (- (cadr definition) 1))
      (back-to-indentation))
    #f))

(define-command (xref-find-definitions . args)
  "Find the definition of the identifier at point.
With a prefix argument, or if there is no identifier at point, read it in the minibuffer."
  (xref--goto-definition (xref--identifier args "Find definitions of") #f))

(define-command (xref-find-definitions-other-window . args)
  "Like `xref-find-definitions' but switch to the other window."
  (xref--goto-definition (xref--identifier args "Find definitions of") #t))

(define (xref--project-files)
  "Return the files below `xref-project-directory', not those in .git, target, or node_modules."
  (filter (lambda (file)
            (not (regexp-match "/(\\.git|target|node_modules)/" file)))
          (directory-files-recursively xref-project-directory "")))

(define (xref--show title lines . locations)
  "Show LINES (strings) under TITLE in the *xref* buffer, in another window; return the buffer name.
If LOCATIONS is not empty, the lines are `FILE:LINE:TEXT' and `next-error' visits them."
  (with-current-buffer (get-buffer-create "*xref*")
    (set-buffer-read-only #f)
    (erase-buffer)
    (insert title "\n\n")
    (let ((start (point)))
      (insert (apply string-append (map (lambda (line) (string-append line "\n")) lines)))
      (if (pair? locations)
          (compile--set-source "*xref*" "" start (point))))
    (goto-char (point-min))
    (set-buffer-modified-p #f)
    (set-buffer-read-only #t))
  (display-buffer "*xref*")
  "*xref*")

(define-command (xref-find-references . args)
  "Find references to the identifier at point.
With a prefix argument, or if there is no identifier at point, read it in the minibuffer.
The references are listed in the *xref* buffer as FILE:LINE:TEXT."
  (let* ((name (xref--identifier args "Find references of"))
         (found (rust-grep (string-append "\\b" (regexp-quote name) "\\b") (xref--project-files))))
    (if (null? found)
        (user-error (format "No references found for: %s" name)))
    (xref--push-marker-stack)
    (xref--show (format "References to %s:" name)
                (map (lambda (hit) (format "%s:%d:%s" (car hit) (cadr hit) (caddr hit))) found)
                #t)
    #f))

(define-command (xref-find-apropos . args)
  "Find all meaningful symbols that match PATTERN, a regexp.
They are listed in the *xref* buffer as NAME (FILE)."
  (let* ((pattern (if (pair? args) (car args) (read-string "Search for pattern (word list or regexp): ")))
         (names (filter (lambda (name) (regexp-match pattern name)) (help-function-names))))
    (if (null? names)
        (user-error (format "No apropos matches for: %s" pattern)))
    (xref--push-marker-stack)
    (xref--show (format "Definitions matching %s:" pattern)
                (map (lambda (name)
                       (let ((file (symbol-file (string->symbol name))))
                         (if file (format "%s  (%s)" name file) name)))
                     names))
    #f))

(define-command (help-find-source . args)
  "Switch to the source of the function whose description is shown in *Help*, in another window.
Interactively, read the function name in the minibuffer."
  (let ((function (if (null? args)
                      (help--read-symbol "Find source of function: " (help-function-names))
                      (car args))))
    (xref--goto-definition (symbol->string function) #t)))

(provide 'xref)

;;; xref.scm ends here
