;;; dired.scm --- directory listings -*- scheme -*-

;; Emacs の dired.el と dired-aux.el に当たるもの (`dired`, `dired-other-window`). 一覧は, Emacs 30 と同じく `ls -al` で作る.
;; 一覧のバッファは読み込み専用で, Emacs の `dired-mode-map` のキーの多くを割り当てる (`dired--keys`). 割り当てない文字は
;; `undefined`, 数字は前置引数にする (Emacs の `suppress-keymap`).
;; 一覧は, 変えた後に作り直し (印は引き継ぐ), Emacs のように 1 行ずつ足さない.
;; `set-buffer-directory` は, 画面 (Tauri) に, dired のバッファのディレクトリ (`default-directory`) を教える.

(define dired--directories '()
  "List of (BUFFER-NAME DIRECTORY) for the Dired buffers.")

(define (dired--directory-file-name dir)
  "Return DIR without the trailing slashes (\"/\" stays)."
  (let loop ((d dir))
    (if (and (> (string-length d) 1) (equal? (substring d (- (string-length d) 1)) "/"))
        (loop (substring d 0 (- (string-length d) 1)))
        d)))

(define (dired--base-name dir)
  "Return the name of the Dired buffer for DIR (its last component; \"/\" stays)."
  (let ((m (regexp-match "([^/]+)$" dir)))
    (if m (cadr m) dir)))

(define (dired--text dir)
  "Return the text of the Dired buffer of DIR: the directory, then the `ls -al' lines indented by two spaces."
  (let ((lines (filter (lambda (line) (and (not (equal? line ""))
                                            (not (string-prefix? "total " line))))
                       (split-string (directory-listing dir) "\n"))))
    (apply string-append
           (map (lambda (line) (string-append "  " line "\n"))
                (cons (string-append dir ":") lines)))))

(define (dired--trivial-p name)
  "Return true if NAME is \".\", \"..\" or starts with \"#\" or \".#\" (Emacs's `dired-trivial-filenames')."
  (or (equal? name ".") (equal? name "..") (string-prefix? "#" name) (string-prefix? ".#" name)))

(define (dired--goto-first-file)
  "Move point to the name of the first nontrivial file (`dired--trivial-p'), or to the end if there is none."
  (goto-char (point-min))
  (let loop ()
    (let ((name (dired--name-here)))
      (cond ((eobp) #f)
            ((and name (not (dired--trivial-p name))) (dired--move-to-filename))
            (else (forward-line 1) (loop))))))

(define (dired--writable thunk)
  "Call THUNK with the current buffer writable, then make it read-only and unmodified again."
  (set-buffer-read-only #f)
  (unwind-protect (thunk)
    (set-buffer-read-only #t)
    (set-buffer-modified-p #f)))

(define (dired--fill dir)
  "Replace the text of the current buffer with the listing of DIR."
  (dired--writable (lambda ()
                     (delete-region (point-min) (point-max))
                     (insert (dired--text dir))))
  (set-buffer-directory dir)
  (dired--goto-first-file))

(define (dired--live)
  "Forget the Dired buffers that no longer exist."
  (set! dired--directories
        (filter (lambda (entry) (member (car entry) (buffer-list))) dired--directories)))

(define (dired-noselect dirname)
  "Like `dired' but return the Dired buffer (its name) without selecting it.
An existing Dired buffer of the same directory is reused."
  (let ((dir (dired--directory-file-name (expand-file-name dirname))))
    (dired--live)
    (let ((open (filter (lambda (entry) (equal? (cadr entry) dir)) dired--directories)))
      (if (pair? open)
          (car (car open))
          (let ((name (generate-new-buffer (dired--base-name dir))))
            (with-current-buffer name (dired--fill dir) (dired--set-keys))
            (set! dired--directories (cons (list name dir) dired--directories))
            name)))))

(define (dired--default-directory)
  "Return the directory to start from: the directory of the Dired buffer or of the visited file, or the home."
  (let ((entry (assoc (buffer-name) dired--directories))
        (file (buffer-file-name)))
    (cond (entry (string-append (cadr entry) "/"))
          (file (or (file-name-directory file) "~/"))
          (else "~/"))))

(define-command (dired . args)
  "\"Edit\" directory DIRNAME--delete, rename, print, etc. some files in it.
Interactively, read DIRNAME in the minibuffer with file name completion.
The listing is made by `ls -al'.  An existing Dired buffer of the directory is shown again."
  (let ((dirname (if (null? args)
                     (read-file-name "Dired (directory): " (dired--default-directory))
                     (car args))))
    (switch-to-buffer (dired-noselect dirname))
    #f))

(define-command (dired-other-window . args)
  "\"Edit\" directory DIRNAME.  Like `dired' but selects in another window."
  (let ((dirname (if (null? args)
                     (read-file-name "Dired in other window (directory): " (dired--default-directory))
                     (car args))))
    (display-buffer (dired-noselect dirname))
    (other-window 1)
    #f))

(define (dired--goto-file name)
  "Move point to the name NAME in the current Dired buffer.  Return #f, without moving, if the listing has no such file."
  (let ((found (save-excursion
                 (goto-char (point-min))
                 (and (re-search-forward (string-append dired--filename-regexp (regexp-quote name) "\\( -> .*\\)?$")
                                         #f #t)
                      (begin (goto-char (match-beginning 0))
                             (re-search-forward dired--filename-regexp (line-end-position) #t)
                             (point))))))
    (and found (begin (goto-char found) #t))))

(define (dired--jump other file)
  "Show the Dired buffer of the directory of FILE with point on FILE.  List the directory again if FILE is missing."
  (let* ((path (dired--directory-file-name (expand-file-name file)))
         (dir (dired--directory-file-name (or (file-name-directory path) "/")))
         (dir (if (equal? dir "") "/" dir))
         (name (dired--base-name path))
         (buffer (dired-noselect dir)))
    (if other
        (begin (display-buffer buffer) (other-window 1))
        (switch-to-buffer buffer))
    (unless (or (equal? path "/") (dired--goto-file name))
      (dired--revert)
      (dired--goto-file name))
    #f))

(define (dired--jump-file)
  "Return the file `dired-jump' starts from: the visited file, the directory of a Dired buffer, or the home."
  (let ((entry (assoc (buffer-name) dired--directories))
        (file (buffer-file-name)))
    (cond (entry (cadr entry))
          (file file)
          (else (expand-file-name "~/")))))

(define-command (dired-jump . args)
  "Jump to Dired buffer corresponding to current buffer.
If in a buffer visiting a file, Dired that file's directory and
move to that file's line in the directory listing.
If the current buffer isn't visiting a file, Dired the home directory.
If in Dired already, pop up a level and go to the old directory's line.
When OTHER-WINDOW is true, jump to Dired buffer in other window.
When FILE-NAME is given, jump to its line in Dired."
  (dired--jump (and (pair? args) (car args))
               (if (and (pair? args) (pair? (cdr args))) (cadr args) (dired--jump-file))))

(define-command (dired-jump-other-window . args)
  "Like `dired-jump' but in other window."
  (dired--jump #t (if (pair? args) (car args) (dired--jump-file))))

(define (dired--revert)
  "List the directory of the current Dired buffer again, and return #t.  Return #f if it is not a Dired buffer.
The marks stay, and point stays on the same file (Emacs's `dired-revert')."
  (let ((entry (assoc (buffer-name) dired--directories)))
    (and entry
         (let ((point (point))
               (name (dired--name-here))
               (marks (dired--marks)))
           (dired--fill (cadr entry))
           (dired--put-marks marks)
           (unless (and name (dired--goto-file name))
             (goto-char (min point (point-max))))
           #t))))

(define (files--revert-dired) (dired--revert))

;;; 行とファイルの名前

(define dired--filename-regexp "[A-Z][a-z][a-z] +[0-9]+ +[0-9:]+ "
  "Regexp that ends the date of an `ls -al' line, so that the file name follows it.")

(define (dired--filename-start)
  "Return the position of the file name on the current line, or #f if the line has no file."
  (save-excursion
    (goto-char (line-beginning-position))
    (and (re-search-forward dired--filename-regexp (line-end-position) #t)
         (point))))

(define (dired--type)
  "Return the file type character of the current line (\"-\", \"d\", \"l\" ...) or #f if it lists no file."
  (and (dired--filename-start)
       (buffer-substring (+ (line-beginning-position) 2) (+ (line-beginning-position) 3))))

(define (dired--name-at start)
  "Return the file name that starts at START, without the target of a symbolic link."
  (let ((name (buffer-substring start (line-end-position))))
    (if (equal? (dired--type) "l")
        (let ((arrow (string-match " -> " name)))
          (if arrow (substring name 0 arrow) name))
        name)))

(define (dired--name-here)
  "Return the file name on the current line (without the directory), or #f."
  (let ((start (dired--filename-start)))
    (and start (dired--name-at start))))

(define (dired--dot-p name)
  (or (equal? name ".") (equal? name "..")))

(define (dired-get-filename . args)
  "Return the name of the file on the current line, or #f.  With LOCALP non-nil, without the directory."
  (let ((name (dired--name-here))
        (entry (assoc (buffer-name) dired--directories)))
    (and name entry
         (if (and (pair? args) (car args))
             name
             (string-append (cadr entry) "/" name)))))

(define (dired--directory)
  "Return the directory of the current Dired buffer."
  (let ((entry (assoc (buffer-name) dired--directories)))
    (if entry (cadr entry) (error "Not a Dired buffer"))))

(define (dired--move-to-filename)
  (let ((start (dired--filename-start)))
    (if start (goto-char start))))

(define (dired--line-mark)
  "Return the mark (the first character) of the current line."
  (buffer-substring (line-beginning-position) (min (point-max) (+ (line-beginning-position) 1))))

;;; 移る

(define (dired--next n)
  (forward-line n)
  (dired--move-to-filename))

(define-command (dired-next-line . args)
  "Move down lines then position at the file name."
  (dired--next (simple--arg args))
  #f)

(define-command (dired-previous-line . args)
  "Move up lines then position at the file name."
  (dired--next (- (simple--arg args)))
  #f)

(define (dired--search-line n match? message)
  "Move to the Nth next line (previous if N is negative) for which MATCH? is true, at the file name.
Signal an error with MESSAGE, without moving, if there are not so many."
  (let ((start (point)))
    (let loop ((n n))
      (cond ((= n 0) (dired--move-to-filename))
            ((if (> n 0)
                 (begin (forward-line 1) (eobp))
                 (begin (goto-char (line-beginning-position)) (bobp)))
             (goto-char start)
             (error message))
            ((> n 0) (if (match?) (loop (- n 1)) (loop n)))
            (else (forward-line -1) (if (match?) (loop (+ n 1)) (loop n)))))))

(define-command (dired-next-dirline . args)
  "Goto ARGth next directory file line."
  (dired--search-line (simple--arg args) (lambda () (equal? (dired--type) "d")) "No more subdirectories")
  #f)

(define-command (dired-prev-dirline . args)
  "Goto ARGth previous directory file line."
  (dired--search-line (- (simple--arg args)) (lambda () (equal? (dired--type) "d")) "No more subdirectories")
  #f)

(define (dired--marked-line-p)
  (let ((mark (dired--line-mark)))
    (and (not (equal? mark " ")) (not (equal? mark "")) (dired--filename-start) #t)))

(define (dired--next-marked-file n)
  "Move to the Nth next marked file; at the end, wrap around (Emacs's `dired-next-marked-file' with WRAP)."
  (let ((start (point)))
    (condition-case nil
        (dired--search-line n dired--marked-line-p "No next marked file")
      (error
       (message "(Wraparound for next marked file)")
       (goto-char (if (> n 0) (point-min) (point-max)))
       ;; 始めの行 (終わりの行) も, 見る
       (if (> n 0) (forward-line -1))
       (condition-case nil
           (dired--search-line n dired--marked-line-p "No next marked file")
         (error (goto-char start) (error "No next marked file")))))))

(define-command (dired-next-marked-file . args)
  "Move to the ARGth next marked file, wrapping around to the beginning."
  (dired--next-marked-file (simple--arg args))
  #f)

(define-command (dired-prev-marked-file . args)
  "Move to the ARGth previous marked file, wrapping around to the end."
  (dired--next-marked-file (- (simple--arg args)))
  #f)

(define-command (dired-goto-file . args)
  "Go to line describing file FILE in this Dired buffer."
  (let* ((dir (dired--directory))
         (file (dired--directory-file-name
                (expand-file-name (if (null? args)
                                      (read-file-name "Goto file: " (string-append dir "/"))
                                      (car args))
                                  dir))))
    (push-mark)
    (let ((start (point)))
      (if (and (equal? (dired--directory-file-name (or (file-name-directory file) "/")) dir)
               (dired--goto-file (dired--base-name file)))
          (point)
          (begin (goto-char start) #f)))))

;;; 印

(define (dired--mark-line char)
  "Put the mark CHAR in the first column of the current line, if it lists a file."
  (when (dired--filename-start)
    (dired--writable (lambda ()
                       (save-excursion
                         (goto-char (line-beginning-position))
                         (delete-char 1)
                         (insert char))))))

(define (dired--between-files)
  (not (dired--filename-start)))

(define (dired--repeat-over-lines n function)
  "Call FUNCTION on the next N file lines (the previous -N lines if N is negative), then move to the file name.
Like Emacs's `dired-repeat-over-lines'."
  (goto-char (line-beginning-position))
  (let loop ((n n))
    (when (and (> n 0) (not (eobp)))
      (while (and (not (eobp)) (dired--between-files)) (forward-line 1))
      (let ((next (save-excursion (forward-line 1) (point))))
        (save-excursion (function))
        (goto-char next))
      (loop (- n 1))))
  (let loop ((n n))
    (when (and (< n 0) (not (bobp)))
      (forward-line -1)
      (while (and (not (bobp)) (dired--between-files)) (forward-line -1))
      (save-excursion (function))
      (loop (+ n 1))))
  (dired--move-to-filename))

(define (dired--mark-lines char n)
  "Put CHAR on the next N file lines.  \".\" and \"..\" are not flagged for deletion."
  (dired--repeat-over-lines n (lambda ()
                                (let ((name (dired--name-here)))
                                  (when (and name (not (and (equal? char "D") (dired--dot-p name))))
                                    (dired--mark-line char)))))
  #f)

(define-command (dired-mark . args)
  "Mark the file at point in the Dired buffer, then move to the next line.
With a prefix arg, mark files on the next ARG lines (the previous lines if ARG is negative)."
  (dired--mark-lines "*" (simple--arg args)))

(define-command (dired-flag-file-deletion . args)
  "In Dired, flag the current line's file for deletion, then move to the next line.
With a prefix arg, flag files on the next ARG lines."
  (dired--mark-lines "D" (simple--arg args)))

(define-command (dired-unmark . args)
  "Unmark the file at point in the Dired buffer, then move to the next line.
With a prefix arg, unmark files on the next ARG lines."
  (dired--mark-lines " " (simple--arg args)))

(define-command (dired-unmark-backward . args)
  "In Dired, move up lines and remove marks or deletion flags there.
Optional prefix ARG says how many lines to unmark/unflag; default is one line."
  (dired--mark-lines " " (- (simple--arg args))))

(define-command (dired-toggle-marks)
  "Toggle marks: marked files become unmarked, and vice versa.
Flagged files (indicated with flags such as `C' and `D', not with `*') are not affected,
and `.' and `..' are never toggled."
  (save-excursion
    (goto-char (point-min))
    (while (not (eobp))
      (let ((name (dired--name-here)))
        (when (and name (not (dired--dot-p name)))
          (cond ((equal? (dired--line-mark) " ") (dired--mark-line "*"))
                ((equal? (dired--line-mark) "*") (dired--mark-line " ")))))
      (forward-line 1)))
  #f)

(define (dired--mark-if char predicate msg)
  "Put CHAR on the lines whose file satisfies PREDICATE (called with the name, point on the line).
MSG names the kind of the files in the messages.  Return the count, or #f if it is 0 (Emacs's `dired-mark-if')."
  (let ((count 0))
    (message "%s %ss%s..."
             (cond ((equal? char " ") "Unmarking") ((equal? char "D") "Flagging") (else "Marking"))
             msg
             (if (equal? char "D") " for deletion" ""))
    (save-excursion
      (goto-char (point-min))
      (while (not (eobp))
        (let ((name (dired--name-here)))
          (when (and name (predicate name) (not (equal? (dired--line-mark) char)))
            (dired--mark-line char)
            (set! count (+ count 1))))
        (forward-line 1)))
    (message "%s %s%s %s%s" count msg (if (= count 1) "" "s")
             (if (equal? char " ") "un" "")
             (if (equal? char "D") "flagged" "marked"))
    (and (> count 0) count)))

(define (dired--unflag-char args char)
  "Return \" \" if there is a prefix argument (in ARGS or the current one), else CHAR."
  (if (if (pair? args) (car args) (current-prefix-arg)) " " char))

(define-command (dired-mark-directories . args)
  "Mark all directory file lines except `.' and `..'.
With prefix argument, unmark or unflag all those files."
  (dired--mark-if (dired--unflag-char args "*")
                  (lambda (name) (and (equal? (dired--type) "d") (not (dired--dot-p name))))
                  "directory file"))

(define-command (dired-mark-executables . args)
  "Mark all executable files.
With prefix argument, unmark or unflag all those files."
  (dired--mark-if (dired--unflag-char args "*")
                  (lambda (name)
                    (let ((modes (buffer-substring (+ (line-beginning-position) 2) (+ (line-beginning-position) 12))))
                      (and (equal? (substring modes 0 1) "-")
                           (string-match-p "^-..[xs]\\|^-.....[xs]\\|^-........[xs]" modes)
                           #t)))
                  "executable file"))

(define-command (dired-mark-symlinks . args)
  "Mark all symbolic links.
With prefix argument, unmark or unflag all those files."
  (dired--mark-if (dired--unflag-char args "*")
                  (lambda (name) (equal? (dired--type) "l"))
                  "symbolic link"))

(define (dired--mark-regexp regexp char)
  (dired--mark-if char
                  (lambda (name) (and (not (dired--dot-p name)) (string-match-p regexp name) #t))
                  "matching file"))

(define-command (dired-mark-files-regexp . args)
  "Mark all files matching REGEXP for use in later commands.
A prefix argument means to unmark them instead.  `.' and `..' are never marked."
  (let* ((unmark (current-prefix-arg))
         (regexp (if (pair? args)
                     (car args)
                     (read-string (if unmark "Unmark files (regexp): " "Mark files (regexp): ")))))
    (dired--mark-regexp regexp (cond ((and (pair? args) (pair? (cdr args))) (cadr args))
                                     (unmark " ")
                                     (else "*")))))

(define-command (dired-flag-files-regexp . args)
  "In Dired, flag all files containing the specified REGEXP for deletion.
`.' and `..' are never flagged."
  (dired--mark-regexp (if (pair? args) (car args) (read-string "Flag for deletion (regexp): ")) "D"))

(define dired-garbage-files-regexp "\\.\\(aux\\|bak\\|dvi\\|log\\|orig\\|rej\\|toc\\)\\'"
  "Regular expression to match \"garbage\" files for `dired-flag-garbage-files'.")

(define-command (dired-flag-garbage-files)
  "Flag for deletion all files that match `dired-garbage-files-regexp'."
  (dired--mark-regexp dired-garbage-files-regexp "D"))

(define-command (dired-flag-backup-files . args)
  "Flag all backup files (names ending with `~') for deletion.
With prefix argument, unmark or unflag these files."
  (dired--mark-if (dired--unflag-char args "D")
                  (lambda (name) (and (not (equal? (dired--type) "d")) (string-suffix? "~" name)))
                  "backup file"))

(define-command (dired-flag-auto-save-files . args)
  "Flag for deletion files whose names suggest they are auto save files.
A prefix argument says to unmark or unflag those files instead."
  (dired--mark-if (dired--unflag-char args "D")
                  (lambda (name)
                    (and (not (equal? (dired--type) "d"))
                         (> (string-length name) 1)
                         (string-prefix? "#" name) (string-suffix? "#" name)))
                  "auto save file"))

(define (dired--unmark-all mark)
  "Remove MARK (or any mark if it is #f) from every file, and report the count."
  (let ((count 0))
    (save-excursion
      (goto-char (point-min))
      (while (not (eobp))
        (let ((old (dired--line-mark)))
          (when (and (dired--filename-start)
                     (not (equal? old " "))
                     (or (not mark) (equal? old mark)))
            (dired--mark-line " ")
            (set! count (+ count 1))))
        (forward-line 1)))
    (message (if (= count 1) "1 mark removed" (format "%d marks removed" count))))
  #f)

(define-command (dired-unmark-all-marks)
  "Remove all marks from all files in the Dired buffer."
  (dired--unmark-all #f))

(define-command (dired-unmark-all-files . args)
  "Remove a specific mark (or any mark) from every file.
After this command, type the mark character to remove, or type RET to remove all marks."
  (let ((key (if (pair? args)
                 (car args)
                 ;; Emacs は, 問いをエコー領域に出し, 読んだ後に消す
                 (let ((key (begin (message "Remove marks (RET means all): ")
                                   (read-key "Remove marks (RET means all): "))))
                   (message "")
                   key))))
    (dired--unmark-all (if (member key '("Enter" "RET" "C-m")) #f key))))

(define (dired--marked char)
  "Return the file names (without the directory) of the lines marked with CHAR."
  (save-excursion
    (goto-char (point-min))
    (let loop ((names '()))
      (if (eobp)
          (reverse names)
          (let ((name (and (equal? (dired--line-mark) char) (dired--name-here))))
            (forward-line 1)
            (loop (if name (cons name names) names)))))))

(define (dired--marks)
  "Return a list of (NAME MARK) of the marked lines."
  (save-excursion
    (goto-char (point-min))
    (let loop ((marks '()))
      (if (eobp)
          marks
          (let* ((name (dired--name-here))
                 (mark (dired--line-mark))
                 (next (if (and name (not (equal? mark " ")))
                           (cons (list name mark) marks)
                           marks)))
            (forward-line 1)
            (loop next))))))

(define (dired--put-marks marks)
  "Put back MARKS (a list of (NAME MARK)) on the lines of the names."
  (save-excursion
    (for-each (lambda (entry)
                (when (dired--goto-file (car entry))
                  (dired--mark-line (cadr entry))))
              marks)))

;;; 一覧を作り直す

(define (dired--anchor skip)
  "Return the name of the first file at or after the current line that is not in SKIP, or #f.
Emacs inserts a new entry before this line."
  (save-excursion
    (goto-char (line-beginning-position))
    (let loop ()
      (if (eobp)
          #f
          (let ((name (dired--name-here)))
            (if (and name (not (member name skip)))
                name
                (begin (forward-line 1) (loop))))))))

(define (dired--names)
  "Return the names of the files in the current Dired buffer, in the order of the lines."
  (save-excursion
    (goto-char (point-min))
    (let loop ((names '()))
      (if (eobp)
          (reverse names)
          (let* ((name (dired--name-here))
                 (next (if name (cons name names) names)))
            (forward-line 1)
            (loop next))))))

(define (dired--order order anchor new)
  "Return ORDER (a list of names) with NEW put before ANCHOR (or at the end if ANCHOR is not in it)."
  (let ((rest (filter (lambda (name) (not (member name new))) order)))
    (if (and anchor (member anchor rest))
        (apply append (map (lambda (name) (if (equal? name anchor) (append new (list name)) (list name))) rest))
        (append rest new))))

(define (dired--reorder order)
  "Move the lines of the files in ORDER, one after another, to the end of the buffer.
Names the listing does not have are skipped.  Emacs keeps the order of the lines
and inserts a new entry at point, instead of sorting the names again."
  (dired--writable
   (lambda ()
     (for-each (lambda (name)
                 (when (dired--goto-file name)
                   (let* ((start (line-beginning-position))
                          (end (min (point-max) (+ (line-end-position) 1)))
                          (text (buffer-substring start end)))
                     (delete-region start end)
                     (goto-char (point-max))
                     (insert text))))
               order))))

(define (dired--relist marks . place)
  "List the directory again, keep the order of the lines, then put MARKS (a list of (NAME MARK)) back.
PLACE is (ANCHOR NAMES): the lines of NAMES go before the line of ANCHOR.  Point stays on the same file;
if it is gone, point goes to ANCHOR (the next file that remains)."
  (let ((name (dired--name-here))
        (order (dired--names)))
    (dired--fill (dired--directory))
    (dired--reorder (if (pair? place)
                        (dired--order order (car place) (cadr place))
                        order))
    (dired--put-marks marks)
    (cond ((and name (dired--goto-file name)) #t)
          ((and (pair? place) (car place) (dired--goto-file (car place))) #t)
          (else (dired--goto-first-file)))))

(define (dired--others marks names)
  (filter (lambda (entry) (not (member (car entry) names))) marks))

(define-command (dired-do-redisplay . args)
  "Redisplay the Dired buffer, keeping the marks and the order of the lines."
  (dired--relist (dired--marks))
  #f)

(define-command (dired-do-kill-lines . args)
  "Remove all marked lines, or the next ARG lines.
The files or directories on those lines are _not_ deleted.  Only the Dired listing is affected.
To restore the removals, use \\[revert-buffer]."
  (let ((arg (if (pair? args) (car args) (current-prefix-arg))))
    (if arg
        (let loop ((n (prefix-numeric-value arg)))
          (cond ((= n 0) (dired--move-to-filename))
                ((not (dired--filename-start)) (error "Can only kill file lines"))
                (else
                 (dired--writable (lambda ()
                                    (delete-region (line-beginning-position)
                                                   (save-excursion (forward-line 1) (point)))))
                 (if (< n 0) (forward-line -1))
                 (loop (if (> n 0) (- n 1) (+ n 1))))))
        (let ((count 0))
          (save-excursion
            (goto-char (point-min))
            (while (not (eobp))
              (if (and (equal? (dired--line-mark) "*") (dired--filename-start))
                  (begin
                    (dired--writable (lambda ()
                                       (delete-region (line-beginning-position)
                                                      (save-excursion (forward-line 1) (point)))))
                    (set! count (+ count 1)))
                  (forward-line 1))))
          (message "Killed %d line%s." count (if (= count 1) "" "s")))))
  #f)

;;; ファイルを消す・写す・名前を変える

(define (dired--next-names n)
  "Return the names of the next N files (the previous -N files if N is negative), starting at this line."
  (save-excursion
    (let ((names '()))
      (dired--repeat-over-lines n (lambda ()
                                    (let ((name (dired--name-here)))
                                      (if name (set! names (cons name names))))))
      (if (> n 0) (reverse names) names))))

(define (dired--marked-or-current)
  "Return the files to operate on: with a numeric prefix argument, the next ARG files;
with another prefix argument, the current file; else the files marked with `*', or the current file."
  (let ((arg (current-prefix-arg))
        (marked (dired--marked "*")))
    (cond ((and arg (not (pair? arg)))
           (let ((names (dired--next-names (prefix-numeric-value arg))))
             (if (null? names) (error "No file on this line") names)))
          ((and (not arg) (pair? marked)) marked)
          ((dired--name-here) (list (dired--name-here)))
          (else (error "No file on this line")))))

(define (dired--mark-prompt names)
  "Return the name of the files NAMES in a prompt: the name, \"[next N files]\" or \"* [N files]\" (Emacs's `dired-mark-prompt')."
  (let ((arg (current-prefix-arg)))
    (cond ((= (length names) 1) (car names))
          ((integer? arg) (format "[next %d files]" arg))
          (else (format "* [%d files]" (length names))))))

(define (dired--delete names)
  "Ask, then delete the files NAMES (names in the directory of the current Dired buffer)."
  (let ((dir (dired--directory)))
    (when (yes-or-no-p (format "Delete %s " (dired--mark-prompt names)))
      (message "Deleting... ")
      (for-each (lambda (name)
                  (let ((path (string-append dir "/" name)))
                    (if (file-directory? path) (delete-directory path #t) (delete-file path))))
                names)
      (message "Deleting...done")
      (dired--relist (dired--others (dired--marks) names) (dired--anchor names) '()))))

(define-command (dired-do-flagged-delete . args)
  "In Dired, delete the files flagged for deletion."
  (let ((names (dired--marked "D")))
    (if (null? names)
        (message "(No deletions requested)")
        (dired--delete names)))
  #f)

(define-command (dired-do-delete . args)
  "Delete all marked (or next ARG) files."
  (dired--delete (dired--marked-or-current))
  #f)

(define (dired--target dir name prompt)
  "Read the target with PROMPT.  For one file NAME, return its absolute name (inside the directory if it is one).
For several files (NAME is #f), return the directory."
  (let* ((typed (read-file-name prompt (string-append dir "/")))
         (target (dired--directory-file-name (expand-file-name typed dir))))
    (cond ((not name)
           (if (file-directory? target) target (error (format "Target must be a directory: %s" target))))
          ((file-directory? target) (string-append target "/" name))
          (else target))))

(define (dired--transfer verb prompt operate keep-mark rename-p)
  "Copy or move the marked (or current) files.  OPERATE takes the source and the target."
  (let* ((dir (dired--directory))
         (names (dired--marked-or-current))
         (marks (dired--marks))
         (anchor (dired--anchor (if rename-p names (list))))
         (one (and (= (length names) 1) (car names)))
         (target (dired--target dir one (format prompt (dired--mark-prompt names)))))
    (let loop ((rest names) (done 0))
      (when (pair? rest)
        (message "%s: %d of %d" verb (+ done 1) (length names))
        (operate (string-append dir "/" (car rest))
                 (if (= (length names) 1) target (string-append target "/" (car rest))))
        (loop (cdr rest) (+ done 1))))
    (message "%s: %d file%s done" verb (length names) (if (= (length names) 1) "" "s"))
    (let ((new (dired--base-name target)))
      (dired--relist (keep-mark marks names new)
                     anchor
                     (if (and (= (length names) 1)
                              (equal? (dired--directory-file-name (or (file-name-directory target) ""))
                                      (dired--directory-file-name dir)))
                         (list new)
                         (list)))
      ;; Emacs moves point to the file renamed in the same directory
      (if (and rename-p (= (length names) 1)) (dired--goto-file new)))
    #f))

(define-command (dired-do-copy . args)
  "Copy all marked (or next ARG) files, or copy the current file."
  (dired--transfer "Copy" "Copy %s to: " (lambda (from to) (copy-file from to))
                   (lambda (marks names new) (cons (list new "C") (dired--others marks names)))
                   #f))

(define-command (dired-do-rename . args)
  "Rename all marked (or next ARG) files, or rename the current file."
  (dired--transfer "Move" "Rename %s to: " (lambda (from to) (rename-file from to))
                   (lambda (marks names new)
                     (map (lambda (entry)
                            (if (member (car entry) names) (list new (cadr entry)) entry))
                          marks))
                   #t))

(define-command (dired-create-directory . args)
  "Create a directory called DIRECTORY."
  (let* ((dir (dired--directory))
         (target (expand-file-name (if (null? args)
                                       (read-file-name "Create directory: " (string-append dir "/"))
                                       (car args))
                                   dir)))
    (let ((anchor (dired--anchor (list))))
      (make-directory target)
      (dired--relist (dired--marks) anchor
                     (if (equal? (dired--directory-file-name (or (file-name-directory target) ""))
                                 (dired--directory-file-name dir))
                         (list (dired--base-name target))
                         (list)))))
  #f)

(define-command (dired-copy-filename-as-kill . args)
  "Copy names of marked (or next ARG) files into the kill ring.
If there are several names, they will be separated by a space, and file names that have
spaces or quote characters in them will be quoted (with double quotes).
With a zero prefix arg, use the absolute file name of each marked file."
  (let* ((arg (if (pair? args) (car args) (current-prefix-arg)))
         (dir (dired--directory))
         (marked (let ((marked (dired--marked "*")))
                   (if (pair? marked) marked (list (or (dired--name-here) (error "No file on this line"))))))
         (names (cond ((or (not arg) (pair? arg)) marked)
                      ((= (prefix-numeric-value arg) 0)
                       (map (lambda (name) (string-append dir "/" name)) marked))
                      (else (dired--next-names (prefix-numeric-value arg)))))
         (text (if (= (length names) 1)
                   (car names)
                   (string-join (map (lambda (name)
                                       (if (string-match-p "[ \"']" name) (format "%S" name) name))
                                     names)
                                " "))))
    (unless (equal? text "")
      (kill-new text)
      (message "%s" text)))
  #f)

;;; ファイルを開く

(define (dired--file-for-visit)
  (or (dired-get-filename) (error "No file on this line")))

(define-command (dired-find-file)
  "In Dired, visit the file or directory named on this line."
  (let ((file (dired--file-for-visit)))
    (switch-to-buffer (if (file-directory? file) (dired-noselect file) (find-file-noselect file))))
  #f)

(define-command (dired-find-file-other-window)
  "In Dired, visit this file or directory in another window."
  (let ((file (dired--file-for-visit)))
    (if (file-directory? file) (dired-other-window file) (find-file-other-window file)))
  #f)

(define-command (dired-display-file)
  "In Dired, display this file or directory in another window."
  (display-buffer (find-file-noselect (dired--file-for-visit)))
  #f)

(define-command (dired-view-file)
  "In Dired, examine a file read-only.  A directory is shown with Dired."
  (let ((file (dired--file-for-visit)))
    (if (file-directory? file) (dired file) (find-file-read-only file)))
  #f)

(define-command (dired-up-directory)
  "Run Dired on the parent directory of the current directory, with point on the directory."
  (let* ((dir (dired--directory))
         (up (dired--directory-file-name (or (file-name-directory dir) "/"))))
    (unless (equal? dir "/")
      (switch-to-buffer (dired-noselect (if (equal? up "") "/" up)))
      (dired--goto-file (dired--base-name dir))))
  #f)

;;; キー

(define dired--keys
  '(("n" dired-next-line) ("SPC" dired-next-line) ("C-n" dired-next-line) ("p" dired-previous-line)
    ("S-SPC" dired-previous-line) ("C-p" dired-previous-line)
    (">" dired-next-dirline) ("<" dired-prev-dirline) ("j" dired-goto-file)
    ("M-}" dired-next-marked-file) ("* C-n" dired-next-marked-file)
    ("M-{" dired-prev-marked-file) ("* C-p" dired-prev-marked-file)
    ("RET" dired-find-file) ("e" dired-find-file) ("f" dired-find-file) ("o" dired-find-file-other-window)
    ("C-o" dired-display-file) ("v" dired-view-file) ("^" dired-up-directory) ("q" quit-window)
    ("m" dired-mark) ("* m" dired-mark) ("u" dired-unmark) ("* u" dired-unmark)
    ("DEL" dired-unmark-backward) ("* DEL" dired-unmark-backward)
    ("U" dired-unmark-all-marks) ("* !" dired-unmark-all-marks)
    ("M-DEL" dired-unmark-all-files) ("* ?" dired-unmark-all-files)
    ("t" dired-toggle-marks) ("* t" dired-toggle-marks)
    ("* *" dired-mark-executables) ("* /" dired-mark-directories) ("* @" dired-mark-symlinks)
    ("* %" dired-mark-files-regexp) ("% m" dired-mark-files-regexp) ("% d" dired-flag-files-regexp)
    ("% &" dired-flag-garbage-files)
    ("d" dired-flag-file-deletion) ("x" dired-do-flagged-delete) ("#" dired-flag-auto-save-files)
    ("~" dired-flag-backup-files) ("D" dired-do-delete) ("C" dired-do-copy) ("R" dired-do-rename)
    ("+" dired-create-directory) ("g" revert-buffer) ("l" dired-do-redisplay) ("k" dired-do-kill-lines)
    ("w" dired-copy-filename-as-kill))
  "The keys of the Dired buffers (part of Emacs's `dired-mode-map').")

(define (dired--set-keys)
  "Give the current Dired buffer its keys.  The other printable characters are `undefined', and the digits
and `-' make a prefix argument (like Emacs's `suppress-keymap')."
  (let loop ((code 33))
    (when (< code 127)
      (let ((key (char-to-string code)))
        (cond ((and (>= code 48) (<= code 57)) (local-set-key key 'digit-argument))
              ((equal? key "-") (local-set-key key 'negative-argument))
              ((not (or (assoc key dired--keys) (member key '("*" "%"))))
               (local-set-key key 'undefined))))
      (loop (+ code 1))))
  (for-each (lambda (binding) (local-set-key (car binding) (cadr binding))) dired--keys))
