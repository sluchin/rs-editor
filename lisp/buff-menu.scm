;;; buff-menu.scm --- the buffer list -*- scheme -*-

;; Emacs の buff-menu.el に当たるもの (`list-buffers`).
;; 見出しの行 (header-line) が無いので, 見出しを本文の 1 行目に書く (Emacs で `Buffer-menu-use-header-line` を nil にしたときと同じ).

(define Buffer-menu-use-header-line #f
  "If non-nil, use the header line to display Buffer Menu column titles.
This editor has no header line, so the titles are the first line of the buffer.")

(define Buffer-menu-name-width 19
  "Width of buffer name column in the Buffer Menu.")

(define Buffer-menu-size-width 7
  "Width of buffer size column in the Buffer Menu.")

(define Buffer-menu-mode-width 16
  "Width of mode name column in the Buffer Menu.")

(define Buffer-menu-files-only #f
  "Non-nil if the current Buffer Menu lists only file buffers.")

(define Buffer-menu--names '()
  "The names of the buffers listed in *Buffer List*, in the order of its lines.")

(define (buff-menu--spaces n)
  (if (<= n 0) "" (string-append " " (buff-menu--spaces (- n 1)))))

(define (buff-menu--column text width right)
  "Pad TEXT to WIDTH columns (on the left if RIGHT), then add the one space between columns."
  (let ((fill (buff-menu--spaces (- width (string-width text)))))
    (if right
        (string-append fill text " ")
        (string-append text fill " "))))

(define (buff-menu--mode-name dialect)
  (case dialect
    ((scheme) "Scheme")
    ((elisp) "ELisp")
    (else "Fundamental")))

(define (buff-menu--file-name file)
  "Replace the home directory in FILE with ~."
  (let ((home (expand-file-name "~")))
    (if (string-prefix? (string-append home "/") file)
        (string-append "~" (substring file (string-length home)))
        file)))

(define (buff-menu--row name old)
  "Return the line of the Buffer Menu for the buffer NAME (OLD is the buffer the command was run in)."
  (let ((file (buffer-file-name name))
        (size (with-current-buffer name (buffer-size)))
        (dialect (with-current-buffer name (syntax-dialect))))
    (string-append (if (equal? name old) "." " ")
                   " "
                   (if (buffer-modified-p name) "*" " ")
                   " "
                   (buff-menu--column name Buffer-menu-name-width #f)
                   (buff-menu--column (number->string size) Buffer-menu-size-width #t)
                   (buff-menu--column (buff-menu--mode-name dialect) Buffer-menu-mode-width #f)
                   (if file (buff-menu--file-name file) "")
                   "\n")))

(define (buff-menu--listed-p name files-only)
  (let ((file (buffer-file-name name)))
    (and (not (equal? name "*Buffer List*"))
         (or file (not (equal? (substring name 0 1) " ")))
         (or file (not files-only)))))

(define (list-buffers-noselect . args)
  "Create and return a buffer with a list of names of existing buffers.
The buffer is named *Buffer List*.
If FILES-ONLY is non-nil, show only the buffers that visit files."
  (let* ((files-only (and (pair? args) (car args) (>= (prefix-numeric-value (car args)) 0)))
         (old (buffer-name))
         (names (filter (lambda (name) (buff-menu--listed-p name files-only)) (buffer-list)))
         (rows (map (lambda (name) (buff-menu--row name old)) names)))
    (switch-to-buffer "*Buffer List*")
    (set-buffer-read-only #f)
    (delete-region (point-min) (point-max))
    (set! Buffer-menu-files-only files-only)
    (set! Buffer-menu--names names)
    (insert (string-append " C R M Buffer    Size Mode File \n" (apply string-append rows)))
    (goto-char (point-min))
    (set-buffer-modified-p #f)
    (set-buffer-read-only #t)
    (for-each (lambda (binding) (local-set-key (car binding) (cadr binding)))
              '(("RET" Buffer-menu-this-window) ("f" Buffer-menu-this-window)
                ("o" Buffer-menu-other-window) ("q" quit-window)
                ("n" next-line) ("p" previous-line) ("SPC" next-line)))
    (switch-to-buffer old)
    "*Buffer List*"))

(define-command (list-buffers . args)
  "Display a list of names of existing buffers.
The list is displayed in a buffer named *Buffer List*.
The first column shows \".\" for the buffer you came from, then \"%\" for a read-only
buffer and \"*\" for a modified buffer; then follow the name, size, mode and file.
With a non-negative prefix argument FILES-ONLY, show only the buffers that visit files."
  (display-buffer (list-buffers-noselect (if (pair? args) (car args) (current-prefix-arg))))
  #f)

(define (Buffer-menu-buffer)
  "Return the name of the buffer described by the current Buffer Menu line."
  (let ((n (- (simple--count-lines (point-min) (line-beginning-position)) 1)))
    (if (and (>= n 0) (< n (length Buffer-menu--names)))
        (list-ref Buffer-menu--names n)
        (user-error "No buffer on this line"))))

(define-command (Buffer-menu-this-window)
  "Select this line's buffer in this window."
  (switch-to-buffer (Buffer-menu-buffer))
  #f)

(define-command (Buffer-menu-other-window)
  "Select this line's buffer in another window."
  (switch-to-buffer-other-window (Buffer-menu-buffer))
  #f)
