;;; files.scm --- file input and output commands -*- scheme -*-

;; Emacs の files.el に当たるもの. 起動のときに読む (core/src/scheme/loadup.rs).
;; コマンドは, 引数を省くと (キーと M-x から呼ぶと), ミニバッファで聞く (Emacs の interactive に当たる).

(define auto-mode-alist
  '(("\\.el$" elisp)
    ("\\.(scm|ss|sld|sls|rkt)$" scheme)
    ("\\.(txt|text)$" text-mode))
  "List of (REGEXP DIALECT) to choose the syntax dialect (major mode) of a visited file.
The first REGEXP (Rust regex syntax) that matches the file name decides; other files are Fundamental.")

(define (set-auto-mode)
  "Select the syntax dialect (major mode) of the current buffer from its file name.
See `auto-mode-alist'."
  (let ((file (buffer-file-name)))
    (let loop ((modes auto-mode-alist))
      (cond ((or (not file) (null? modes)) (set-syntax-dialect 'fundamental))
            ((regexp-match (car (car modes)) file)
             (set-syntax-dialect (if (equal? (cadr (car modes)) 'text-mode) 'fundamental (cadr (car modes))))
             (if (equal? (cadr (car modes)) 'text-mode) (set-major-mode-name 'text-mode)))
            (else (loop (cdr modes)))))))

(define (find-buffer-visiting filename)
  "Return the buffer visiting file FILENAME, or #f if there is none."
  (let ((file (expand-file-name filename)))
    (let ((names (filter (lambda (name) (equal? (buffer-file-name name) file)) (buffer-list))))
      (and (pair? names) (car names)))))

(define (find-file-noselect filename . args)
  "Read file FILENAME into a buffer and return the buffer.
If a buffer exists visiting FILENAME, return that one.  A new buffer is named after the file,
its syntax dialect is chosen by `set-auto-mode', and `find-file-hook' runs in it.
If FILENAME does not exist, the new buffer is empty and visits it.
If FILENAME is a directory, return a Dired buffer (`dired-noselect')."
  (let ((file (expand-file-name filename)))
    (or (and (file-directory? file) (dired-noselect file))
        (find-buffer-visiting file)
        (let ((name (generate-new-buffer (file-name-nondirectory file))))
          (with-current-buffer name
            (if (file-exists? file)
                ;; 読めなければ, 作ったバッファを消して失敗する
                (condition-case e
                    (insert-file-contents file #t)
                  (error (kill-buffer name) (error (cadr e))))
                (files--set-buffer-file-name file))
            (set-auto-mode)
            (run-hooks 'find-file-hook)
            ;; フックが名前を変えることもある
            (buffer-name))))))

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
  (insert-file-contents (if (null? args) (read-file-name "Insert file: ") (car args)))
  #f)

;;; Commands of the builtins (Emacs の C のコマンド). 組み込みは, files--NAME で呼ぶ
;; 2 度読んでも, 組み込みを指したままにする (2 度目の write-region などは, ここのコマンド)
(define files--write-region (if (commandp 'write-region) files--write-region write-region))
(define files--copy-file (if (commandp 'copy-file) files--copy-file copy-file))
(define files--do-auto-save (if (commandp 'do-auto-save) files--do-auto-save do-auto-save))

(define (files--confirm-overwrite file action)
  "Signal an error unless FILE does not exist or the user agrees to ACTION it anyway."
  (when (and (file-exists? file)
             (not (yes-or-no-p (format "File %s already exists; %s anyway? " file action))))
    (signal 'file-already-exists (list "File already exists" file))))

(define-command (write-region . args)
  "Write current region into specified file.
When called from a program, requires three arguments: START, END and FILENAME.
START and END are normally buffer positions specifying the part of the buffer to write.
If START is #f, that means to write the whole buffer.  If START is a string, then output
that string to the file instead of any buffer contents; END is ignored.
Optional fourth argument APPEND if non-#f means append to existing file contents (if any).
Optional fifth argument VISIT, if #t or a string, means set the last-save-file-modtime of
buffer to this file's modtime and mark buffer not modified; if a string, the buffer visits
that file name.
Interactively, write the region to a file read in the minibuffer, asking before overwriting
an existing file."
  (if (null? args)
      (let ((start (region-beginning))
            (end (region-end))
            (file (read-file-name "Write region to file: ")))
        (files--confirm-overwrite file "overwrite")
        (files--write-region start end file))
      (apply files--write-region args)))

(define-command (copy-file . args)
  "Copy FILE to NEWNAME.  Both args must be strings.
Signal a `file-already-exists' error if file NEWNAME already exists, unless a third argument
OK-IF-ALREADY-EXISTS is supplied and non-#f.
Interactively, read FILE and NEWNAME in the minibuffer; if NEWNAME is a directory, copy FILE
into it, and ask before overwriting an existing file."
  (if (null? args)
      (let* ((file (read-file-name "Copy file: "))
             (newname (read-file-name (format "Copy %s to file: " file)))
             (target (if (file-directory? newname)
                         (expand-file-name (file-name-nondirectory file) newname)
                         newname)))
        (files--confirm-overwrite target "copy to it")
        (files--copy-file file target #t))
      (apply files--copy-file args)))

(define-command (do-auto-save . args)
  "Auto-save all buffers that need it.
This writes the modified file-visiting buffers to their auto save files (#NAME#),
and prints \"Auto-saving...done\" if anything was saved."
  (when (> (files--do-auto-save) 0)
    (message "Auto-saving...")
    (message "Auto-saving...done"))
  #f)

(define-command (set-visited-file-name . args)
  "Change the name of the file visited in the current buffer to FILENAME.
The buffer is renamed after the file, and marked as modified (the new file is not saved yet).
If FILENAME is #f, the buffer visits no file.
Interactively, read FILENAME in the minibuffer; the initial input is the current file."
  (files--set-visited-file-name
   (if (null? args) (read-file-name "Set visited file name: " (or (buffer-file-name) "")) (car args))))

(define (files--set-visited-file-name filename)
  "Make the current buffer visit FILENAME (see `set-visited-file-name')."
  (let ((file (and filename (expand-file-name filename))))
    (files--set-buffer-file-name file)
    (when file
      (let ((base (file-name-nondirectory file)))
        (unless (equal? base (buffer-name))
          (rename-buffer base #t)))
      (set-buffer-modified-p #t))))

(define mode-require-final-newline #t
  "Non-#f means a file-visiting buffer gets a final newline when it is saved (if it is not empty).")

(define (files--require-final-newline)
  "Add a newline at the end of the current buffer unless it is empty or already ends with one."
  (when mode-require-final-newline
    (save-excursion
      (save-restriction
        (widen)
        (let ((end (point-max)))
          (when (and (> end 1) (not (equal? (buffer-substring (- end 1) end) "\n")))
            (goto-char end)
            (insert "\n")))))))

(define (make-backup-file-name file)
  "Return the backup file name of FILE (FILE~)."
  (string-append file "~"))

(define (backup-buffer)
  "Make a backup of the file visited by the current buffer, if appropriate.
This is done before the first save after visiting the file: the old file is copied to its
backup file name (`make-backup-file-name').  Nothing is done if the file does not exist yet."
  (let ((file (buffer-file-name)))
    (when (and file
               (not (files--buffer-backed-up-p))
               (file-exists? file)
               (not (file-directory? file)))
      (copy-file file (make-backup-file-name file) #t)
      (files--set-buffer-backed-up #t))))

(define (delete-auto-save-file-if-necessary)
  "Delete the auto save file of the current buffer, if it exists."
  (let ((auto (make-auto-save-file-name)))
    (when (and auto (file-exists? auto))
      (delete-file auto))))

(define-command (basic-save-buffer . args)
  "Save the current buffer in its visited file, if it has been modified.
Make a backup first (`backup-buffer'), delete the auto save file after writing,
and run `after-save-hook'.  If the buffer visits no file, read a file name."
  (cond ((not (buffer-file-name)) (files--write-file (read-file-name "Save file: ")))
        ((buffer-modified-p)
         ;; 訪れてから (最後に保存してから) ファイルが外で書き換わっていれば、確かめる (Emacs と同じ)
         (unless (or (verify-visited-file-modtime)
                     (not (file-exists? (buffer-file-name)))
                     (yes-or-no-p (format "%s has changed since visited or saved.  Save anyway? "
                                          (file-name-nondirectory (buffer-file-name)))))
           (user-error "Save not confirmed"))
         (backup-buffer)
         (files--require-final-newline)
         (write-region #f #f (buffer-file-name) #f #t)
         (delete-auto-save-file-if-necessary)
         (run-hooks 'after-save-hook))
        (else (message "(No changes need to be saved)"))))

(define (files--write-file filename)
  "Write the current buffer into FILENAME, and make the buffer visit it.
Ask before overwriting an existing file other than the visited one."
  (if (and (not (equal? (expand-file-name filename) (buffer-file-name)))
           (file-exists? filename)
           (not (y-or-n-p (format "File ‘%s’ exists; overwrite? " filename))))
      (message "Canceled")
      (begin
        (set-visited-file-name filename)
        (basic-save-buffer))))

(define-command (write-file . args)
  "Write the current buffer into file FILENAME, and make the buffer visit it.
The buffer is renamed after the file. Ask before overwriting another existing file.
Interactively, read FILENAME in the minibuffer with file name completion."
  (files--write-file (if (null? args) (read-file-name "Write file: ") (car args))))

(define-command (save-buffer)
  "Save the current buffer in its file, if it is modified.
If the buffer visits no file, read a file name in the minibuffer and write it there.
The first save after visiting a file keeps the old file as a backup (FILE~)."
  (basic-save-buffer))

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
                       (with-current-buffer name (basic-save-buffer))
                       (loop (cdr names) (+ saved 1)))
                      ((member key '("n" "Backspace" "Delete")) (loop (cdr names) saved))
                      ((equal? key "!")
                       (for-each (lambda (name) (with-current-buffer name (basic-save-buffer)))
                                 names))
                      ((equal? key ".") (with-current-buffer name (basic-save-buffer)))
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
             (files--set-buffer-text (read-file-string file) #f)
             (set-visited-file-modtime))
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
