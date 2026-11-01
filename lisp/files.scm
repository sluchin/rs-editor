;;; files.scm --- file input and output commands -*- scheme -*-

;; Emacs の files.el に当たるもの. 起動のときに読む (core/src/scheme/loadup.rs).
;; コマンドは, 引数を省くと (キーと M-x から呼ぶと), ミニバッファで聞く (Emacs の interactive に当たる).

(define-command (find-file . args)
  "Edit file FILENAME.
Switch to a buffer visiting file FILENAME, creating one if none already exists.
Interactively, read FILENAME in the minibuffer with file name completion."
  (let ((filename (if (null? args) (read-file-name "Find file: ") (car args))))
    (switch-to-buffer (find-file-noselect filename))))

(define-command (find-file-other-window . args)
  "Edit file FILENAME, in another window.
Like `find-file' but creates a new window or reuses an existing one.
Interactively, read FILENAME in the minibuffer with file name completion."
  (let ((filename (if (null? args) (read-file-name "Find file in other window: ") (car args))))
    (display-buffer (find-file-noselect filename))
    (other-window 1)
    #f))

(define-command (find-alternate-file . args)
  "Find file FILENAME, select its buffer, and kill the previous buffer.
If the current buffer visits a file and is modified, ask before killing it.
Interactively, read FILENAME in the minibuffer; the initial input is the current file."
  (let ((filename (if (null? args)
                      (read-file-name "Find alternate file: " (or (buffer-file-name) ""))
                      (car args)))
        (old (buffer-name)))
    (if (and (buffer-file-name)
             (buffer-modified-p)
             (not (yes-or-no-p (format "Kill and replace buffer ‘%s’ without saving it? " old))))
        (message "Aborted")
        (begin
          (set-buffer-modified-p #f)
          (kill-buffer old)
          (find-file filename)))))

(define-command (insert-file . args)
  "Insert the contents of file FILENAME at point.
Point stays before the inserted text.
Interactively, read FILENAME in the minibuffer with file name completion."
  (let ((filename (if (null? args) (read-file-name "Insert file: ") (car args)))
        (pos (point)))
    (insert (read-file-string filename))
    (goto-char pos)))

(define (files--write-file filename)
  "Write the current buffer into FILENAME, and make the buffer visit it.
Ask before overwriting an existing file other than the visited one."
  (if (and (not (equal? (expand-file-name filename) (buffer-file-name)))
           (file-exists? filename)
           (not (y-or-n-p (format "File ‘%s’ exists; overwrite? " filename))))
      (message "Canceled")
      (files--write-buffer filename)))

(define-command (write-file . args)
  "Write the current buffer into file FILENAME, and make the buffer visit it.
The buffer is renamed after the file. Ask before overwriting another existing file.
Interactively, read FILENAME in the minibuffer with file name completion."
  (files--write-file (if (null? args) (read-file-name "Write file: ") (car args))))

(define-command (save-buffer)
  "Save the current buffer in its file, if it is modified.
If the buffer visits no file, read a file name in the minibuffer and write it there.
The first save after visiting a file keeps the old file as a backup (FILE~)."
  (cond ((not (buffer-file-name)) (files--write-file (read-file-name "Save file: ")))
        ((buffer-modified-p) (files--write-buffer (buffer-file-name)))
        (else (message "(No changes need to be saved)"))))

(define-command (save-some-buffers)
  "Save some modified file-visiting buffers, asking about each one.
Answer y or SPC to save it, n or DEL to skip it, ! to save it and all the rest,
. to save it and stop, and q or RET to stop."
  (let ((names (filter (lambda (name) (and (buffer-file-name name) (buffer-modified-p name)))
                       (buffer-list))))
    (if (null? names)
        (message "(No files need saving)")
        (let loop ((names names) (saved 0))
          (if (null? names)
              (when (= saved 0) (message ""))
              (let* ((name (car names))
                     (file (buffer-file-name name))
                     (key (read-key (format "Save file %s? (y, n, !, ., q) " file))))
                (cond ((member key '("y" " "))
                       (files--write-buffer file name)
                       (loop (cdr names) (+ saved 1)))
                      ((member key '("n" "Backspace" "Delete")) (loop (cdr names) saved))
                      ((equal? key "!")
                       (for-each (lambda (name) (files--write-buffer (buffer-file-name name) name))
                                 names))
                      ((equal? key ".") (files--write-buffer file name))
                      ((member key '("q" "Enter")) (loop '() saved))
                      (else (loop names saved)))))))))

(define (make-auto-save-file-name)
  "Return the auto save file name of the file the current buffer visits (#NAME# in the same directory).
Return #f if the buffer visits no file."
  (let ((file (buffer-file-name)))
    (and file
         (string-append (or (file-name-directory file) "") "#" (file-name-nondirectory file) "#"))))

(define-command (revert-buffer . args)
  "Replace the buffer text with the text of the visited file on disk.
Ask before reverting, unless the optional second argument NOCONFIRM is true.
A Dired buffer is listed again without asking.
Point stays at the same character, and the change can be undone."
  (cond ((files--revert-dired) #t)
        ((not (buffer-file-name))
         (message "Buffer does not seem to be associated with any file"))
        (else
         (let ((file (buffer-file-name))
               (noconfirm (and (pair? args) (pair? (cdr args)) (cadr args))))
           (when (or noconfirm
                     (yes-or-no-p (if (buffer-modified-p)
                                      (format "Discard edits and reread from %s? " file)
                                      (format "Revert buffer from file %s? " file))))
             (files--set-buffer-text (read-file-string file) #f))
           (message "")))))

(define-command (revert-buffer-quick . args)
  "Like `revert-buffer', but asks for less confirmation.
If the current buffer is visiting a file, and the buffer is not modified, no confirmation is
required."
  (cond ((and (buffer-file-name) (not (buffer-modified-p)))
         (revert-buffer #t #t)
         (message "Reverted buffer"))
        (else (revert-buffer #t))))

(define-command (not-modified . args)
  "Mark current buffer as unmodified, not needing to be saved.
With prefix ARG, mark buffer as modified, so \\[save-buffer] will save.

It is not a good idea to use this function in Lisp programs, because it prints a message in the
minibuffer.  Instead, use `set-buffer-modified-p'."
  (let ((arg (if (null? args) (current-prefix-arg) (car args))))
    (message (if arg "Modification-flag set" "Modification-flag cleared"))
    (set-buffer-modified-p (and arg #t))
    #f))

(define-command (rename-uniquely)
  "Rename current buffer to a similar name not already taken.
This function is useful for creating multiple shell process buffers or multiple mail buffers,
etc."
  (let* ((base (buffer-name))
         (match (regexp-match-positions "<[0-9]+>$" base)))
    (if (and (pair? match)
             (not (and (buffer-file-name)
                       (equal? base (file-name-nondirectory (buffer-file-name))))))
        (set! base (substring base 0 (car (car match)))))
    (rename-buffer (generate-new-buffer-name base))
    #f))

(define (files--find-file-read-only finder filename)
  "Visit FILENAME with FINDER and make its buffer read-only."
  (if (not (file-exists? filename))
      (error (format "%s does not exist" filename)))
  (with-current-buffer (find-file-noselect filename)
    (set-buffer-read-only #t))
  (finder filename))

(define-command (find-file-read-only . args)
  "Edit file FILENAME but don't allow changes.
Like \\[find-file], but marks buffer as read-only.
Use \\[read-only-mode] to permit editing."
  (files--find-file-read-only find-file
                              (if (null? args) (read-file-name "Find file read-only: ") (car args))))

(define-command (find-file-read-only-other-window . args)
  "Edit file FILENAME in another window but don't allow changes.
Like \\[find-file-other-window], but marks buffer as read-only.
Use \\[read-only-mode] to permit editing."
  (files--find-file-read-only find-file-other-window
                              (if (null? args)
                                  (read-file-name "Find file read-only other window: ")
                                  (car args))))

(define list-directory-brief-switches "-CF"
  "Switches for `list-directory' to pass to `ls' for a brief listing.")

(define list-directory-verbose-switches "-l"
  "Switches for `list-directory' to pass to `ls' for a verbose listing.")

(define-command (list-directory . args)
  "Display a list of files in or matching DIRNAME, a la `ls'.
DIRNAME is globbed by the shell if necessary.
Prefix arg (second arg if noninteractive) means supply -l switch to `ls'.
Actions controlled by variables `list-directory-brief-switches' and
`list-directory-verbose-switches'."
  (let* ((pfx (current-prefix-arg))
         (verbose (if (null? args) pfx (and (pair? (cdr args)) (cadr args))))
         (dirname (expand-file-name
                   (if (null? args)
                       (read-file-name (if pfx "List directory (verbose): " "List directory (brief): "))
                       (car args))))
         (switches (if verbose list-directory-verbose-switches list-directory-brief-switches))
         (wildcard (not (file-directory? dirname)))
         (result (call-shell-command
                  (string-append "ls " switches " -- " (if wildcard dirname (shell-quote-argument dirname)))
                  #f #f)))
    (get-buffer-create "*Directory*")
    (with-current-buffer "*Directory*"
      (set-buffer-read-only #f)
      (erase-buffer)
      (insert "Directory " dirname "\n" (cadr result))
      (goto-char (point-min)))
    (display-buffer "*Directory*")
    #f))


(define-command (recover-this-file)
  "Recover the visited file from its auto save file.
Ask before replacing the buffer text. The buffer is marked as modified."
  (let ((file (buffer-file-name))
        (auto (make-auto-save-file-name)))
    (cond ((not file) (message "This buffer is not visiting a file"))
          ((and (file-exists? file) (not (file-exists? auto)))
           (message (format "Auto save file %s does not exist" auto)))
          ((not (file-newer-than-file-p auto file))
           (message (format "Auto-save file %s not current" auto)))
          (else
           (when (yes-or-no-p (format "Recover auto save file %s? " auto))
             (files--set-buffer-text (read-file-string auto) #t))
           (message "")))))

(provide 'files)

;;; files.scm ends here
