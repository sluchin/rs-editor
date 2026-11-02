;;; compile.scm --- grep and next-error -*- scheme -*-

;; Emacs の grep.el と compile.el に当たるもの (`grep`, `next-error`, `previous-error`, `compile-goto-error`).
;; 結果のバッファ (`*grep*`, `*xref*`) は `ファイル:行:行の文字` の行を並べ, `next-error` はその行を順にたどる.

(define next-error-last-buffer #f
  "The name of the buffer `next-error' reads the locations from, or #f.")

(define next-error-directory ""
  "The directory the file names of `next-error-last-buffer' are relative to.")

(define next-error--start 1
  "The position where the locations of `next-error-last-buffer' start.")

(define next-error--end 1
  "The position where the locations of `next-error-last-buffer' end.")

(define (compile--set-source name directory start end)
  "Make the buffer NAME the source of `next-error': its locations lie between START and END."
  (set! next-error-last-buffer name)
  (set! next-error-directory directory)
  (set! next-error--start start)
  (set! next-error--end end))

(define (compile--hit text)
  "Return (FILE LINE) if the line TEXT is `FILE:LINE:TEXT', or #f."
  (let ((found (regexp-match "^([^:]+):([0-9]+):" text)))
    (and found (list (cadr found) (string->number (caddr found))))))

(define (compile--line-hit)
  "Return the location on the line at point, or #f if the line is outside the locations."
  (and (>= (line-beginning-position) next-error--start)
       (<= (line-end-position) next-error--end)
       (compile--hit (buffer-substring (line-beginning-position) (line-end-position)))))

(define (compile--scan forward)
  "Move point to the next (or previous) line with a location and return it.
If there is none, leave point where it was and return #f."
  (let ((origin (point)))
    (let loop ()
      (let ((pos (if forward (+ (line-end-position) 1) (- (line-beginning-position) 1))))
        (cond ((or (> pos (point-max)) (< pos (point-min)))
               (goto-char origin)
               #f)
              (else
               (goto-char pos)
               (or (compile--line-hit) (loop))))))))

(define (compile--default-directory)
  "Return the directory `grep' starts from (that of the Dired buffer if dired.scm is loaded, else of the visited file)."
  (if (fboundp 'dired--default-directory)
      (dired--default-directory)
      (let ((file (buffer-file-name)))
        (or (and file (file-name-directory file)) "~/"))))

(define (compile--visit hit other-window)
  "Visit the file and line of HIT, in another window if OTHER-WINDOW."
  (let* ((file (car hit))
         (path (if (string-prefix? "/" file) file (string-append next-error-directory file)))
         (buffer (find-file-noselect path)))
    (if other-window
        (window--pop-to-buffer-other-window buffer)
        (pop-to-buffer-same-window buffer))
    (goto-char (point-min))
    (forward-line (- (cadr hit) 1))
    (message "")))

(define (compile--next-error forward)
  "Visit the next (or previous) location of `next-error-last-buffer'."
  (let ((name next-error-last-buffer))
    (if (not (and name (get-buffer name)))
        (user-error "No buffers contain error message locations"))
    (let ((hit (with-current-buffer name (compile--scan forward))))
      (if (not hit)
          (user-error (if forward "Moved past last grep hit" "Moved back before first grep hit")))
      (compile--visit hit (equal? (buffer-name) name)))
    #f))

(define-command (next-error . args)
  "Visit the next location of the grep (or xref) results."
  (compile--next-error #t))

(define-command (previous-error . args)
  "Visit the previous location of the grep (or xref) results."
  (compile--next-error #f))

(define-command (first-error . args)
  "Visit the first location of the grep (or xref) results."
  (let ((name next-error-last-buffer))
    (if (not (and name (get-buffer name)))
        (user-error "No buffers contain error message locations"))
    (with-current-buffer name (goto-char (point-min)))
    (compile--next-error #t)))

(define-command (compile-goto-error . args)
  "Visit the location on the current line of the results, in another window."
  (let ((hit (compile--line-hit)))
    (if (not hit)
        (user-error "No error here"))
    (set! next-error-last-buffer (buffer-name))
    (compile--visit hit #t)
    #f))

(define (grep--home-name path)
  "Return PATH with the home directory written as ~."
  (let ((home (getenv "HOME")))
    (if (and home (string-prefix? (string-append home "/") path))
        (string-append "~" (substring path (string-length home) (string-length path)))
        path)))

(define (grep--files directory)
  "Return the files below DIRECTORY, not those in .git, target, or node_modules."
  (grep-files directory))

(define (grep--time)
  (format-time-string "%a %b %e %H:%M:%S %Y"))

(define (grep--show directory pattern hits)
  "Show HITS ((FILE LINE TEXT) ...) in the *grep* buffer, in another window; return the buffer name."
  (let ((start 1)
        (end 1)
        (count (length hits)))
    (with-current-buffer (get-buffer-create "*grep*")
      (set-buffer-read-only #f)
      (erase-buffer)
      (insert (format "-*- mode: grep; default-directory: \"%s\" -*-\n" (grep--home-name directory)))
      (insert (format "Grep started at %s\n\n" (grep--time)))
      (insert (format "grep (built-in) -r -e %s .\n" pattern))
      (set! start (point))
      (for-each (lambda (hit)
                  (insert (format "%s:%d:%s\n" (car hit) (cadr hit) (caddr hit))))
                hits)
      (set! end (point))
      (insert (format "\nGrep finished with %s found at %s\n"
                      (cond ((= count 0) "no matches") ((= count 1) "1 match") (else (format "%d matches" count)))
                      (grep--time)))
      (goto-char (point-min))
      (set-buffer-modified-p #f)
      (set-buffer-read-only #t)
      (local-set-key "RET" 'compile-goto-error))
    (compile--set-source "*grep*" directory start end)
    (display-buffer "*grep*")
    "*grep*"))

(define-command (grep . args)
  "Search the files below a directory for lines that match a regexp.
Interactively, read the regexp and the directory in the minibuffer.
The matches are listed in the *grep* buffer; \\[next-error] visits them."
  (let* ((pattern (if (pair? args) (car args) (read-string "Run grep (regexp): ")))
         (dir (if (and (pair? args) (pair? (cdr args)))
                  (cadr args)
                  (read-file-name "In directory: " (compile--default-directory))))
         (directory (let ((expanded (expand-file-name dir)))
                      (if (string-suffix? "/" expanded) expanded (string-append expanded "/"))))
         (hits (map (lambda (hit)
                      (list (substring (car hit) (string-length directory) (string-length (car hit)))
                            (cadr hit)
                            (caddr hit)))
                    (rust-grep pattern (grep--files directory)))))
    (grep--show directory pattern hits)
    (message (format "Grep finished with %s found"
                     (cond ((null? hits) "no matches") ((null? (cdr hits)) "1 match") (else (format "%d matches" (length hits))))))
    #f))

(provide 'compile)

;;; compile.scm ends here
