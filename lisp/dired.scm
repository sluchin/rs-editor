;;; dired.scm --- directory listings -*- scheme -*-

;; Emacs の dired.el に当たるもの (`dired`, `dired-other-window`). 一覧は, Emacs 30 と同じく `ls -al` で作る.
;; 一覧のバッファには, 移動 (n p), 印 (m d u U), 消す (x D), 名前を変える・写す (R C), ディレクトリを作る (+), 一覧を作り直す (g),
;; 開く (RET f), 上へ (^) のキーを割り当てる. 一覧は, 変えた後に作り直し (印は引き継ぐ), Emacs のように 1 行ずつ足さない.
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

(define (dired--goto-first-file)
  "Move point to the name of the first file (after \".\" and \"..\"), or to the end if there is none."
  (goto-char (point-min))
  (forward-line 3)
  (if (not (eobp))
      (re-search-forward "[A-Z][a-z][a-z] +[0-9]+ +[0-9:]+ " (line-end-position) #t)))

(define (dired--fill dir)
  "Replace the text of the current buffer with the listing of DIR."
  (delete-region (point-min) (point-max))
  (insert (dired--text dir))
  (set-buffer-directory dir)
  (dired--goto-first-file)
  (set-buffer-modified-p #f))

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
  "Move point to the name NAME in the current Dired buffer.  Return #f if the listing has no such file."
  (goto-char (point-min))
  (and (re-search-forward (string-append "[A-Z][a-z][a-z] +[0-9]+ +[0-9:]+ " (regexp-quote name) "$")
                          #f #t)
       (begin (goto-char (match-beginning 0))
              (re-search-forward "[A-Z][a-z][a-z] +[0-9]+ +[0-9:]+ " (line-end-position) #t)
              #t)))

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
  "List the directory of the current Dired buffer again, and return #t.  Return #f if it is not a Dired buffer."
  (let ((entry (assoc (buffer-name) dired--directories)))
    (and entry
         (let ((point (point)))
           (dired--fill (cadr entry))
           (goto-char (min point (point-max)))
           #t))))

(define (files--revert-dired) (dired--revert))
;;; dired のコマンド

(define dired--filename-regexp "[A-Z][a-z][a-z] +[0-9]+ +[0-9:]+ "
  "Regexp that ends the date of an `ls -al' line, so that the file name follows it.")

(define (dired--filename-start)
  "Return the position of the file name on the current line, or #f if the line has no file."
  (save-excursion
    (goto-char (line-beginning-position))
    (and (re-search-forward dired--filename-regexp (line-end-position) #t)
         (point))))

(define (dired-get-filename . args)
  "Return the name of the file on the current line, or #f.  With LOCALP non-nil, without the directory."
  (let ((start (dired--filename-start))
        (entry (assoc (buffer-name) dired--directories)))
    (and start entry
         (let ((name (buffer-substring start (line-end-position))))
           (if (and (pair? args) (car args))
               name
               (string-append (cadr entry) "/" name))))))

(define (dired--directory)
  "Return the directory of the current Dired buffer."
  (let ((entry (assoc (buffer-name) dired--directories)))
    (if entry (cadr entry) (error "Not a Dired buffer"))))

(define (dired--move-to-filename)
  (let ((start (dired--filename-start)))
    (if start (goto-char start))))

(define (dired--next n)
  (forward-line n)
  (dired--move-to-filename))

(define-command (dired-next-line . args)
  "Move down lines then position at the file name."
  (dired--next (if (null? args) 1 (car args)))
  #f)

(define-command (dired-previous-line . args)
  "Move up lines then position at the file name."
  (dired--next (- (if (null? args) 1 (car args))))
  #f)

(define (dired--mark-line char)
  "Put the mark CHAR in the first column of the current line, if it lists a file."
  (when (dired--filename-start)
    (save-excursion
      (goto-char (line-beginning-position))
      (delete-char 1)
      (insert char))
    (set-buffer-modified-p #f)))

(define (dired--mark-and-next char args)
  (dired--mark-line char)
  (dired--next (if (null? args) 1 (car args)))
  #f)

(define-command (dired-mark . args)
  "Mark the file at point in the Dired buffer, then move to the next line."
  (dired--mark-and-next "*" args))

(define-command (dired-flag-file-deletion . args)
  "In Dired, flag the current line's file for deletion, then move to the next line."
  (dired--mark-and-next "D" args))

(define-command (dired-unmark . args)
  "Unmark the file at point in the Dired buffer, then move to the next line."
  (dired--mark-and-next " " args))

(define (dired--marked char)
  "Return the file names (without the directory) of the lines marked with CHAR."
  (save-excursion
    (goto-char (point-min))
    (let loop ((names '()))
      (if (eobp)
          (reverse names)
          (let ((name (and (equal? (buffer-substring (line-beginning-position) (+ (line-beginning-position) 1)) char)
                           (dired--filename-start)
                           (buffer-substring (dired--filename-start) (line-end-position)))))
            (forward-line 1)
            (loop (if name (cons name names) names)))))))

(define-command (dired-unmark-all-marks)
  "Remove all marks from all files in the Dired buffer."
  (let ((count 0))
    (save-excursion
      (goto-char (point-min))
      (while (not (eobp))
        (when (and (dired--filename-start)
                   (not (equal? (buffer-substring (line-beginning-position) (+ (line-beginning-position) 1)) " ")))
          (dired--mark-line " ")
          (set! count (+ count 1)))
        (forward-line 1)))
    (message (if (= count 0) "No marks removed" (format "%d mark%s removed" count (if (= count 1) "" "s")))))
  #f)

(define (dired--marks)
  "Return a list of (NAME MARK) of the marked lines."
  (save-excursion
    (goto-char (point-min))
    (let loop ((marks '()))
      (if (eobp)
          marks
          (let ((start (dired--filename-start))
                (mark (buffer-substring (line-beginning-position) (+ (line-beginning-position) 1))))
            (let ((next (if (and start (not (equal? mark " ")))
                            (cons (list (buffer-substring start (line-end-position)) mark) marks)
                            marks)))
              (forward-line 1)
              (loop next)))))))

(define (dired--relist marks)
  "List the directory again, then put MARKS (a list of (NAME MARK)) back."
  (let ((name (dired-get-filename #t)))
    (dired--revert)
    (for-each (lambda (entry)
                (when (dired--goto-file (car entry))
                  (dired--mark-line (cadr entry))))
              marks)
    (if (not (and name (dired--goto-file name)))
        (dired--goto-first-file))))

(define (dired--others marks names)
  (filter (lambda (entry) (not (member (car entry) names))) marks))

(define (dired--delete names)
  "Ask, then delete the files NAMES (names in the directory of the current Dired buffer)."
  (let ((dir (dired--directory)))
    (when (yes-or-no-p (if (= (length names) 1)
                           (format "Delete %s " (car names))
                           (format "Delete %d files " (length names))))
      (message "Deleting... ")
      (for-each (lambda (name)
                  (let ((path (string-append dir "/" name)))
                    (if (file-directory? path) (delete-directory path #t) (delete-file path))))
                names)
      (message "Deleting...done")
      (dired--relist (dired--others (dired--marks) names)))))

(define-command (dired-do-flagged-delete . args)
  "In Dired, delete the files flagged for deletion."
  (let ((names (dired--marked "D")))
    (if (null? names)
        (message "(No deletions requested)")
        (dired--delete names)))
  #f)

(define (dired--marked-or-current)
  "Return the files marked with `*', or the current file."
  (let ((marked (dired--marked "*")))
    (cond ((pair? marked) marked)
          ((dired-get-filename #t) (list (dired-get-filename #t)))
          (else (error "No file on this line")))))

(define-command (dired-do-delete . args)
  "Delete all marked (or next ARG) files."
  (dired--delete (dired--marked-or-current))
  #f)

(define (dired--target dir name prompt)
  "Read the target for NAME with PROMPT.  Return its absolute name (inside the directory if it is one)."
  (let* ((typed (read-file-name prompt (string-append dir "/")))
         (target (expand-file-name typed dir)))
    (if (file-directory? target)
        (string-append target "/" name)
        target)))

(define (dired--transfer verb prompt operate keep-mark)
  "Copy or move the marked (or current) files.  OPERATE takes the source and the target."
  (let* ((dir (dired--directory))
         (names (dired--marked-or-current))
         (marks (dired--marks))
         (name (car names))
         (target (dired--target dir name (format prompt (if (= (length names) 1) name (format "%d files" (length names)))))))
    (let loop ((rest names) (done 0))
      (when (pair? rest)
        (message "%s: %d of %d" verb (+ done 1) (length names))
        (operate (string-append dir "/" (car rest))
                 (if (= (length names) 1) target (string-append target "/" (car rest))))
        (loop (cdr rest) (+ done 1))))
    (message "%s: %d file%s done" verb (length names) (if (= (length names) 1) "" "s"))
    (dired--relist (keep-mark marks names (dired--base-name target)))
    #f))

(define-command (dired-do-copy . args)
  "Copy all marked (or next ARG) files, or copy the current file."
  (dired--transfer "Copy" "Copy %s to: " (lambda (from to) (copy-file from to))
                   (lambda (marks names new) (cons (list new "C") (dired--others marks names)))))

(define-command (dired-do-rename . args)
  "Rename all marked (or next ARG) files, or rename the current file."
  (dired--transfer "Move" "Rename %s to: " (lambda (from to) (rename-file from to))
                   (lambda (marks names new)
                     (map (lambda (entry)
                            (if (member (car entry) names) (list new (cadr entry)) entry))
                          marks))))

(define-command (dired-create-directory . args)
  "Create a directory called DIRECTORY."
  (let* ((dir (dired--directory))
         (target (expand-file-name (if (null? args)
                                       (read-file-name "Create directory: " (string-append dir "/"))
                                       (car args))
                                   dir)))
    (make-directory target)
    (dired--relist (dired--marks)))
  #f)

(define-command (dired-do-redisplay . args)
  "Redisplay the Dired buffer, keeping the marks."
  (dired--relist (dired--marks))
  #f)

(define-command (dired-find-file)
  "In Dired, visit the file or directory named on this line."
  (let ((file (dired-get-filename)))
    (if file
        (switch-to-buffer (if (file-directory? file) (dired-noselect file) (find-file-noselect file)))
        (error "No file on this line")))
  #f)

(define-command (dired-up-directory)
  "Run Dired on the parent directory of the current directory."
  (let* ((dir (dired--directory))
         (up (dired--directory-file-name (or (file-name-directory dir) "/"))))
    (switch-to-buffer (dired-noselect (if (equal? up "") "/" up))))
  #f)

(define (dired--set-keys)
  "Give the current Dired buffer its keys."
  (for-each (lambda (binding) (local-set-key (car binding) (cadr binding)))
            '(("n" dired-next-line) ("SPC" dired-next-line) ("p" dired-previous-line)
              ("m" dired-mark) ("d" dired-flag-file-deletion) ("u" dired-unmark)
              ("U" dired-unmark-all-marks) ("x" dired-do-flagged-delete) ("D" dired-do-delete)
              ("C" dired-do-copy) ("R" dired-do-rename) ("+" dired-create-directory)
              ("g" dired-do-redisplay) ("RET" dired-find-file) ("f" dired-find-file)
              ("^" dired-up-directory))))
