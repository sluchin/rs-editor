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
                   (if (with-current-buffer name (buffer-read-only)) "%" " ")
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
              '(("RET" Buffer-menu-this-window) ("f" Buffer-menu-this-window) ("e" Buffer-menu-this-window)
                ("o" Buffer-menu-other-window) ("q" quit-window)
                ("n" next-line) ("p" previous-line) ("SPC" next-line)
                ("d" Buffer-menu-delete) ("k" Buffer-menu-delete) ("C-k" Buffer-menu-delete)
                ("C-d" Buffer-menu-delete-backwards) ("s" Buffer-menu-save) ("x" Buffer-menu-execute)
                ("u" Buffer-menu-unmark) ("U" Buffer-menu-unmark-all) ("DEL" Buffer-menu-backup-unmark)
                ("~" Buffer-menu-not-modified) ("%" Buffer-menu-toggle-read-only)))
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

(define (Buffer-menu--line-index)
  "Return the index in `Buffer-menu--names' of the current line, or #f on the title line or after the list."
  (let ((n (- (simple--count-lines (point-min) (line-beginning-position)) 1)))
    (and (>= n 0) (< n (length Buffer-menu--names)) n)))

(define (Buffer-menu-buffer . args)
  "Return the name of the buffer described by the current Buffer Menu line.
If there is no buffer here, return nil if ERROR-IF-NON-EXISTENT-P
is nil or omitted, and signal an error otherwise."
  (let* ((error-if-non-existent-p (or (null? args) (car args)))
         (n (Buffer-menu--line-index))
         (name (and n (list-ref Buffer-menu--names n))))
    (cond ((not name) (if error-if-non-existent-p (user-error "No buffer on this line") #f))
          ((not (member name (buffer-list)))
           (if error-if-non-existent-p (error "This buffer has been killed") #f))
          (else name))))

(define (Buffer-menu--set-col col desc)
  "Replace the column COL (0 for the mark, 1 for read-only, 2 for modified) of the current line with DESC."
  (save-excursion
    (set-buffer-read-only #f)
    (goto-char (+ (line-beginning-position) col))
    (delete-char 1)
    (insert desc)
    (set-buffer-modified-p #f)
    (set-buffer-read-only #t)))

(define (Buffer-menu--col col)
  "Return the column COL of the current line."
  (let ((pos (+ (line-beginning-position) col)))
    (if (< pos (line-end-position)) (buffer-substring pos (+ pos 1)) "")))

(define (Buffer-menu--delete-entry)
  "Delete the current line and its buffer name from the list."
  (let ((n (Buffer-menu--line-index)))
    (set! Buffer-menu--names
          (let loop ((names Buffer-menu--names) (i 0))
            (cond ((null? names) '())
                  ((= i n) (cdr names))
                  (else (cons (car names) (loop (cdr names) (+ i 1)))))))
    (set-buffer-read-only #f)
    (delete-region (line-beginning-position) (min (point-max) (+ (line-end-position) 1)))
    (set-buffer-modified-p #f)
    (set-buffer-read-only #t)))

(define (Buffer-menu--unmark)
  (Buffer-menu--set-col 0 " ")
  (let ((buf (Buffer-menu-buffer #f)))
    (when buf
      (Buffer-menu--set-col 2 (if (buffer-modified-p buf) "*" " ")))))

(define-command (Buffer-menu-delete . args)
  "Mark the buffer on this Buffer Menu buffer line for deletion.
A subsequent \\<Buffer-menu-mode-map>\\[Buffer-menu-execute] command will delete it.

If prefix argument ARG is non-nil, it specifies the number of
buffers to delete; a negative ARG means to delete backwards."
  (let loop ((arg (let ((n (simple--arg args))) (if (= n 0) 1 n))))
    (cond ((> arg 0)
           (if (Buffer-menu-buffer #f) (Buffer-menu--set-col 0 "D"))
           (forward-line 1)
           (loop (- arg 1)))
          ((< arg 0)
           (if (Buffer-menu-buffer #f) (Buffer-menu--set-col 0 "D"))
           (forward-line -1)
           (loop (+ arg 1)))))
  #f)

(define-command (Buffer-menu-delete-backwards . args)
  "Mark the buffer on this Buffer Menu line for deletion, and move up.
A subsequent \\<Buffer-menu-mode-map>\\[Buffer-menu-execute] command
will delete the marked buffer.  Prefix ARG means move that many lines."
  (Buffer-menu-delete (- (simple--arg args))))

(define-command (Buffer-menu-save)
  "Mark the buffer on this Buffer Menu line for saving.
A subsequent \\<Buffer-menu-mode-map>\\[Buffer-menu-execute] command will save it."
  (if (Buffer-menu-buffer #f) (Buffer-menu--set-col 2 "S"))
  (forward-line 1)
  #f)

(define-command (Buffer-menu-unmark . args)
  "Cancel all requested operations on buffer on this line and move down.
Optional prefix arg means move up."
  (if (Buffer-menu--line-index) (Buffer-menu--unmark))
  (forward-line (if (simple--raw-arg args) -1 1))
  #f)

(define-command (Buffer-menu-unmark-all)
  "Cancel all requested operations on buffers."
  (save-excursion
    (goto-char (point-min))
    (let loop ()
      (unless (eobp)
        (if (Buffer-menu--line-index) (Buffer-menu--unmark))
        (forward-line 1)
        (loop))))
  #f)

(define-command (Buffer-menu-backup-unmark)
  "Move up and cancel all requested operations on buffer on line above."
  (forward-line -1)
  (let loop ()
    (when (and (not (Buffer-menu--line-index)) (not (bobp)))
      (forward-line -1)
      (loop)))
  (unless (bobp) (Buffer-menu--unmark))
  #f)

(define-command (Buffer-menu-not-modified . args)
  "Mark the buffer on this line as unmodified (no changes to save).
If ARG is non-nil (interactively, with a prefix argument), mark
it as modified."
  (let ((arg (simple--raw-arg args)))
    (with-current-buffer (Buffer-menu-buffer #t)
      (set-buffer-modified-p (and arg #t)))
    (Buffer-menu--set-col 2 (if arg "*" " ")))
  #f)

(define-command (Buffer-menu-toggle-read-only)
  "Toggle read-only status of buffer on this line.
This behaves like invoking \\[read-only-mode] in that buffer."
  (let ((read-only (with-current-buffer (Buffer-menu-buffer #t)
                     (set-buffer-read-only (not (buffer-read-only)))
                     (buffer-read-only))))
    (Buffer-menu--set-col 1 (if read-only "%" " ")))
  #f)

(define-command (Buffer-menu-execute)
  "Save and/or delete marked buffers in the Buffer Menu.
Buffers marked with \\<Buffer-menu-mode-map>\\[Buffer-menu-save] are saved.
Buffers marked with \\<Buffer-menu-mode-map>\\[Buffer-menu-delete] are deleted."
  (save-excursion
    (goto-char (point-min))
    (let loop ()
      (unless (eobp)
        (let ((n (Buffer-menu--line-index)))
          (cond ((not n) (forward-line 1))
                ((not (Buffer-menu-buffer #f)) (Buffer-menu--delete-entry))
                (else
                 (let ((buffer (Buffer-menu-buffer #f))
                       (delete (equal? (Buffer-menu--col 0) "D")))
                   (when (equal? (Buffer-menu--col 2) "S")
                     (condition-case e
                         (begin
                           (with-current-buffer buffer (save-buffer))
                           (Buffer-menu--set-col 2 " "))
                       (error (message (format "Error saving %s" buffer)))))
                   (if (and delete
                            (not (equal? buffer (buffer-name)))
                            (begin
                              (when (and (buffer-file-name buffer) (buffer-modified-p buffer))
                                (if (yes-or-no-p (format "Buffer %s modified; kill anyway? " buffer))
                                    (with-current-buffer buffer (set-buffer-modified-p #f))))
                              (and (not (and (buffer-file-name buffer) (buffer-modified-p buffer)))
                                   (begin (kill-buffer buffer) #t))))
                       (Buffer-menu--delete-entry)
                       (forward-line 1)))))
          (loop)))))
  #f)

(define-command (Buffer-menu-this-window)
  "Select this line's buffer in this window."
  (switch-to-buffer (Buffer-menu-buffer))
  #f)

(define-command (Buffer-menu-other-window)
  "Select this line's buffer in another window."
  (switch-to-buffer-other-window (Buffer-menu-buffer))
  #f)
