;;; dired.scm --- directory listings -*- scheme -*-

;; Emacs の dired.el に当たるもの (`dired`, `dired-other-window`). 一覧は, Emacs 30 と同じく `ls -al` で作る.
;; 一覧を読んで, ファイルを開くなどの dired のコマンド (`dired-find-file` など) は無い.

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
            (with-current-buffer name (dired--fill dir))
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

(define (dired--revert)
  "List the directory of the current Dired buffer again, and return #t.  Return #f if it is not a Dired buffer."
  (let ((entry (assoc (buffer-name) dired--directories)))
    (and entry
         (let ((point (point)))
           (dired--fill (cadr entry))
           (goto-char (min point (point-max)))
           #t))))

(define (files--revert-dired) (dired--revert))
