;;; eldoc.scm --- show the arguments of the function at point -*- scheme -*-

;; Emacs の eldoc.el に当たるもの. 起動のときに読む (core/src/scheme/loadup.rs).
;; Lisp のモードで, カーソルが関数の呼び出しの中にあるとき, その関数の引数の一覧をエコー領域に出す.
;; Emacs は, 少し待ってから (eldoc-idle-delay) 出す. ここは, コマンドの後 (post-command-hook) に出す.

(define eldoc--on #f
  "True while `eldoc-mode' is on.")

(define eldoc--last #f
  "The text that eldoc showed last, so that it is not shown again and again.")

(define (eldoc--lisp-buffer-p)
  "Return true if the current buffer is in a Lisp mode."
  (member (symbol->string (major-mode-name))
          '("scheme-mode" "emacs-lisp-mode" "lisp-interaction-mode" "lisp-data-mode")))

(define (eldoc--function-at-point)
  "Return the symbol of the function that the call around point calls, or #f."
  (condition-case e
      (save-excursion
        (backward-up-list)
        (forward-char 1)
        (let* ((start (point))
               (name (begin (skip-chars-forward "^ \t\n()") (buffer-substring start (point)))))
          (and (not (equal? name "")) (string->symbol name))))
    (error #f)))

(define (eldoc--usage function)
  "Return the usage line (the last line of the documentation, such as \"(car LIST)\") of FUNCTION, or #f."
  (let* ((doc (documentation function))
         (lines (and (string? doc) (split-string doc "\n")))
         (last-line (and lines (pair? lines) (car (reverse lines)))))
    (and last-line (string-match (format "^(%s[ )]" (regexp-quote (symbol->string function))) last-line)
         last-line)))

(define (eldoc--string)
  "Return the text for eldoc at point (the arguments of the function being called), or #f.
A Lisp function shows its arguments; a built-in one shows the usage line of its documentation."
  (let ((function (and (eldoc--lisp-buffer-p) (eldoc--function-at-point))))
    (and function
         (fboundp function)
         (if (help-lisp-function-p function)
             (help-arglist-string function)
             (eldoc--usage function)))))

(define (eldoc--post-command)
  "Show the arguments of the function at point in the echo area, for `eldoc-mode'."
  (when eldoc--on
    (let ((text (eldoc--string)))
      (when (and text (not (equal? text eldoc--last)))
        (message "%s" text))
      (set! eldoc--last text))))

(define-command (eldoc-mode . args)
  "Toggle Eldoc mode: in a Lisp buffer, show the arguments of the function at point in the echo area.
With a positive argument, turn it on; with zero or a negative one, turn it off.
Unlike Emacs, it is off at first, and the text shows right after a command."
  (let* ((interactive (null? args))
         (mode (simple--mode-arg args))
         (on (if (eq? mode 'toggle) (not eldoc--on) (= mode 1))))
    (set! eldoc--on on)
    (set! eldoc--last #f)
    (if on
        (add-hook 'post-command-hook 'eldoc--post-command)
        (remove-hook 'post-command-hook 'eldoc--post-command))
    (if interactive (message (if on "Eldoc mode enabled" "Eldoc mode disabled")))
    on))
